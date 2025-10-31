#[cfg(any(feature = "v2016_4", feature = "dox"))]
use crate::RepoListRefsExtFlags;
#[cfg(any(feature = "v2017_10", feature = "dox"))]
use crate::RepoMode;
use crate::{Checksum, ObjectDetails, ObjectName, ObjectType, Repo, RepoTransactionStats};
use ffi::OstreeRepoListObjectsFlags;
use glib::ffi as glib_sys;
use glib::prelude::*;
use glib::{self, translate::*, Error};
#[cfg(any(feature = "v2017_10", feature = "dox"))]
use std::os::fd::BorrowedFd;
use std::{
    collections::{HashMap, HashSet},
    future::Future,
    mem::MaybeUninit,
    path::Path,
    pin::Pin,
    ptr,
};

unsafe extern "C" fn read_variant_table(
    _key: glib_sys::gpointer,
    value: glib_sys::gpointer,
    hash_set: glib_sys::gpointer,
) {
    let value: glib::Variant = from_glib_none(value as *const glib_sys::GVariant);
    let set: &mut HashSet<ObjectName> = &mut *(hash_set as *mut HashSet<ObjectName>);
    set.insert(ObjectName::new_from_variant(value));
}

unsafe extern "C" fn read_variant_object_map(
    key: glib_sys::gpointer,
    value: glib_sys::gpointer,
    hash_set: glib_sys::gpointer,
) {
    let key: glib::Variant = from_glib_none(key as *const glib_sys::GVariant);
    let value: glib::Variant = from_glib_none(value as *const glib_sys::GVariant);
    let set: &mut HashMap<ObjectName, ObjectDetails> =
        &mut *(hash_set as *mut HashMap<ObjectName, ObjectDetails>);
    if let Some(details) = ObjectDetails::new_from_variant(value) {
        set.insert(ObjectName::new_from_variant(key), details);
    }
}

unsafe fn from_glib_container_variant_set(ptr: *mut glib_sys::GHashTable) -> HashSet<ObjectName> {
    let mut set = HashSet::new();
    glib_sys::g_hash_table_foreach(
        ptr,
        Some(read_variant_table),
        &mut set as *mut HashSet<ObjectName> as *mut _,
    );
    glib_sys::g_hash_table_unref(ptr);
    set
}

unsafe fn from_glib_container_variant_map(
    ptr: *mut glib_sys::GHashTable,
) -> HashMap<ObjectName, ObjectDetails> {
    let mut set = HashMap::new();
    glib_sys::g_hash_table_foreach(
        ptr,
        Some(read_variant_object_map),
        &mut set as *mut HashMap<ObjectName, ObjectDetails> as *mut _,
    );
    glib_sys::g_hash_table_unref(ptr);
    set
}

/// An open transaction in the repository.
///
/// This will automatically invoke [`ostree::Repo::abort_transaction`] when the value is dropped.
pub struct TransactionGuard<'a> {
    /// Reference to the repository for this transaction.
    repo: Option<&'a Repo>,
}

impl<'a> TransactionGuard<'a> {
    /// Returns a reference to the repository.
    pub fn repo(&self) -> &Repo {
        // SAFETY: This is only set to None in `commit`, which consumes self
        self.repo.unwrap()
    }

    /// Commit this transaction.
    pub fn commit<P: IsA<gio::Cancellable>>(
        mut self,
        cancellable: Option<&P>,
    ) -> Result<RepoTransactionStats, glib::Error> {
        // Safety: This is the only function which mutates this option
        let repo = self.repo.take().unwrap();
        repo.commit_transaction(cancellable)
    }
}

impl<'a> Drop for TransactionGuard<'a> {
    fn drop(&mut self) {
        if let Some(repo) = self.repo {
            // TODO: better logging in ostree?
            // See also https://github.com/ostreedev/ostree/issues/2413
            let _ = repo.abort_transaction(gio::Cancellable::NONE);
        }
    }
}

impl Repo {
    /// Create a new `Repo` object for working with an OSTree repo at the given path.
    pub fn new_for_path<P: AsRef<Path>>(path: P) -> Repo {
        Repo::new(&gio::File::for_path(path.as_ref()))
    }

    /// Open using the target directory file descriptor.
    #[cfg(any(feature = "v2017_10", feature = "dox"))]
    pub fn open_at_dir(dir: BorrowedFd<'_>, path: &str) -> Result<Repo, glib::Error> {
        use std::os::unix::io::AsRawFd;
        crate::Repo::open_at(dir.as_raw_fd(), path, gio::Cancellable::NONE)
    }

    /// A version of [`create_at`] which resolves the path relative to the provided directory file descriptor, and also returns the opened repo.
    #[cfg(any(feature = "v2017_10", feature = "dox"))]
    pub fn create_at_dir(
        dir: BorrowedFd<'_>,
        path: &str,
        mode: RepoMode,
        options: Option<&glib::Variant>,
    ) -> Result<Repo, glib::Error> {
        use std::os::unix::io::AsRawFd;
        crate::Repo::create_at(dir.as_raw_fd(), path, mode, options, gio::Cancellable::NONE)?;
        Repo::open_at_dir(dir, path)
    }

    /// A wrapper for [`prepare_transaction`] which ensures the transaction will be aborted when the guard goes out of scope.
    pub fn auto_transaction<P: IsA<gio::Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<TransactionGuard<'_>, glib::Error> {
        let _ = self.prepare_transaction(cancellable)?;
        Ok(TransactionGuard { repo: Some(self) })
    }

    /// Return a copy of the directory file descriptor for this repository.
    #[cfg(any(feature = "v2016_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2016_4")))]
    pub fn dfd_as_file(&self) -> std::io::Result<std::fs::File> {
        use std::os::unix::prelude::FromRawFd;
        use std::os::unix::prelude::IntoRawFd;
        unsafe {
            // A temporary owned file instance
            let dfd = std::fs::File::from_raw_fd(self.dfd());
            // So we can call dup() on it
            let copy = dfd.try_clone();
            // Now release our temporary ownership of the original
            let _ = dfd.into_raw_fd();
            copy
        }
    }

    /// Borrow the directory file descriptor for this repository.
    #[cfg(feature = "v2017_10")]
    pub fn dfd_borrow(&self) -> BorrowedFd {
        unsafe { BorrowedFd::borrow_raw(self.dfd()) }
    }

    /// Find all objects reachable from a commit.
    pub fn traverse_commit<P: IsA<gio::Cancellable>>(
        &self,
        commit_checksum: &str,
        maxdepth: i32,
        cancellable: Option<&P>,
    ) -> Result<HashSet<ObjectName>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let mut hashtable = ptr::null_mut();
            let _ = ffi::ostree_repo_traverse_commit(
                self.to_glib_none().0,
                commit_checksum.to_glib_none().0,
                maxdepth,
                &mut hashtable,
                cancellable.map(AsRef::as_ref).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_container_variant_set(hashtable))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// List all branch names (refs).
    pub fn list_refs<P: IsA<gio::Cancellable>>(
        &self,
        refspec_prefix: Option<&str>,
        cancellable: Option<&P>,
    ) -> Result<HashMap<String, String>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let mut hashtable = ptr::null_mut();
            let _ = ffi::ostree_repo_list_refs(
                self.to_glib_none().0,
                refspec_prefix.to_glib_none().0,
                &mut hashtable,
                cancellable.map(AsRef::as_ref).to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_container(hashtable))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// List all repo objects
    pub fn list_objects<P: IsA<gio::Cancellable>>(
        &self,
        flags: OstreeRepoListObjectsFlags,
        cancellable: Option<&P>,
    ) -> Result<HashMap<ObjectName, ObjectDetails>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let mut hashtable = ptr::null_mut();

            ffi::ostree_repo_list_objects(
                self.to_glib_none().0,
                flags,
                &mut hashtable,
                cancellable.map(AsRef::as_ref).to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_container_variant_map(hashtable))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// List refs with extended options.
    #[cfg(any(feature = "v2016_4", feature = "dox"))]
    pub fn list_refs_ext<P: IsA<gio::Cancellable>>(
        &self,
        refspec_prefix: Option<&str>,
        flags: RepoListRefsExtFlags,
        cancellable: Option<&P>,
    ) -> Result<HashMap<String, String>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let mut hashtable = ptr::null_mut();
            let _ = ffi::ostree_repo_list_refs_ext(
                self.to_glib_none().0,
                refspec_prefix.to_glib_none().0,
                &mut hashtable,
                flags.into_glib(),
                cancellable.map(AsRef::as_ref).to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_container(hashtable))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Resolve a refspec to a commit SHA256.
    /// Returns an error if the refspec does not exist.
    pub fn require_rev(&self, refspec: &str) -> Result<glib::GString, Error> {
        // SAFETY: Since we said `false` for "allow_noent", this function must return a value
        Ok(self.resolve_rev(refspec, false)?.unwrap())
    }

    /// Load the contents (for regular files) and metadata for a content object.
    #[doc(alias = "ostree_repo_load_file")]
    pub fn load_file<P: IsA<gio::Cancellable>>(
        &self,
        checksum: &str,
        cancellable: Option<&P>,
    ) -> Result<(Option<gio::InputStream>, gio::FileInfo, glib::Variant), glib::Error> {
        unsafe {
            let mut out_input = ptr::null_mut();
            let mut out_file_info = ptr::null_mut();
            let mut out_xattrs = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::ostree_repo_load_file(
                self.to_glib_none().0,
                checksum.to_glib_none().0,
                &mut out_input,
                &mut out_file_info,
                &mut out_xattrs,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((
                    from_glib_full(out_input),
                    from_glib_full(out_file_info),
                    from_glib_full(out_xattrs),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Query metadata for a content object.
    ///
    /// This is similar to [`load_file`], but is more efficient if reading the file content is not needed.
    pub fn query_file<P: IsA<gio::Cancellable>>(
        &self,
        checksum: &str,
        cancellable: Option<&P>,
    ) -> Result<(gio::FileInfo, glib::Variant), glib::Error> {
        unsafe {
            let mut out_file_info = ptr::null_mut();
            let mut out_xattrs = ptr::null_mut();
            let mut error = ptr::null_mut();
            let r = ffi::ostree_repo_load_file(
                self.to_glib_none().0,
                checksum.to_glib_none().0,
                ptr::null_mut(),
                &mut out_file_info,
                &mut out_xattrs,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                debug_assert!(r != 0);
                Ok((from_glib_full(out_file_info), from_glib_full(out_xattrs)))
            } else {
                debug_assert_eq!(r, 0);
                Err(from_glib_full(error))
            }
        }
    }

    /// Write a content object from provided input.
    pub fn write_content<P: IsA<gio::InputStream>, Q: IsA<gio::Cancellable>>(
        &self,
        expected_checksum: Option<&str>,
        object_input: &P,
        length: u64,
        cancellable: Option<&Q>,
    ) -> Result<Checksum, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let mut out_csum = ptr::null_mut();
            let _ = ffi::ostree_repo_write_content(
                self.to_glib_none().0,
                expected_checksum.to_glib_none().0,
                object_input.as_ref().to_glib_none().0,
                length,
                &mut out_csum,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(out_csum))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Write a metadata object.
    pub fn write_metadata<P: IsA<gio::Cancellable>>(
        &self,
        objtype: ObjectType,
        expected_checksum: Option<&str>,
        object: &glib::Variant,
        cancellable: Option<&P>,
    ) -> Result<Checksum, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let mut out_csum = ptr::null_mut();
            let _ = ffi::ostree_repo_write_metadata(
                self.to_glib_none().0,
                objtype.into_glib(),
                expected_checksum.to_glib_none().0,
                object.to_glib_none().0,
                &mut out_csum,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(out_csum))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Asynchronously write a content object.
    pub fn write_content_async<
        P: IsA<gio::InputStream>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<Checksum, Error>) + Send + 'static,
    >(
        &self,
        expected_checksum: Option<&str>,
        object: &P,
        length: u64,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn write_content_async_trampoline<
            R: FnOnce(Result<Checksum, Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut out_csum = MaybeUninit::uninit();
            let _ = ffi::ostree_repo_write_content_finish(
                _source_object as *mut _,
                res,
                out_csum.as_mut_ptr(),
                &mut error,
            );
            let out_csum = out_csum.assume_init();
            let result = if error.is_null() {
                Ok(Checksum::from_glib_full(out_csum))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = write_content_async_trampoline::<R>;
        unsafe {
            ffi::ostree_repo_write_content_async(
                self.to_glib_none().0,
                expected_checksum.to_glib_none().0,
                object.as_ref().to_glib_none().0,
                length,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    /// Asynchronously write a content object.
    pub fn write_content_async_future<P: IsA<gio::InputStream> + Clone + 'static>(
        &self,
        expected_checksum: Option<&str>,
        object: &P,
        length: u64,
    ) -> Pin<Box<dyn Future<Output = Result<Checksum, Error>> + 'static>> {
        let expected_checksum = expected_checksum.map(ToOwned::to_owned);
        let object = object.clone();
        Box::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.write_content_async(
                expected_checksum
                    .as_ref()
                    .map(::std::borrow::Borrow::borrow),
                &object,
                length,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    /// Asynchronously write a metadata object.
    pub fn write_metadata_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<Checksum, Error>) + Send + 'static,
    >(
        &self,
        objtype: ObjectType,
        expected_checksum: Option<&str>,
        object: &glib::Variant,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn write_metadata_async_trampoline<
            Q: FnOnce(Result<Checksum, Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut out_csum = MaybeUninit::uninit();
            let _ = ffi::ostree_repo_write_metadata_finish(
                _source_object as *mut _,
                res,
                out_csum.as_mut_ptr(),
                &mut error,
            );
            let out_csum = out_csum.assume_init();
            let result = if error.is_null() {
                Ok(Checksum::from_glib_full(out_csum))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = write_metadata_async_trampoline::<Q>;
        unsafe {
            ffi::ostree_repo_write_metadata_async(
                self.to_glib_none().0,
                objtype.into_glib(),
                expected_checksum.to_glib_none().0,
                object.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    /// Asynchronously write a metadata object.
    pub fn write_metadata_async_future(
        &self,
        objtype: ObjectType,
        expected_checksum: Option<&str>,
        object: &glib::Variant,
    ) -> Pin<Box<dyn Future<Output = Result<Checksum, Error>> + 'static>> {
        let expected_checksum = expected_checksum.map(ToOwned::to_owned);
        let object = object.clone();
        Box::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.write_metadata_async(
                objtype,
                expected_checksum
                    .as_ref()
                    .map(::std::borrow::Borrow::borrow),
                &object,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    /// Load and parse directory metadata.
    /// In particular, uid/gid/mode are stored in big-endian format; this function
    /// converts them to host native endianness.
    pub fn read_dirmeta(&self, checksum: &str) -> Result<crate::DirMetaParsed, glib::Error> {
        let v = self.load_variant(crate::ObjectType::DirMeta, checksum)?;
        // Safety: We know the variant type will match since we just passed it above
        Ok(crate::DirMetaParsed::from_variant(&v).unwrap())
    }

    /// List all commit objects; an optional prefix filter may be applied.
    #[doc(alias = "ostree_repo_list_commit_objects_starting_with")]
    pub fn list_commit_objects_starting_with<P: IsA<gio::Cancellable>>(
        &self,
        prefix: Option<&str>,
        cancellable: Option<&P>,
    ) -> Result<HashSet<glib::GString>, glib::Error> {
        use glib::ffi::gpointer;
        let prefix = prefix.unwrap_or("");
        unsafe {
            let repo = self.to_glib_none().0;
            let mut commits = ptr::null_mut();
            let cancellable = cancellable.map(|p| p.as_ref()).to_glib_none().0;
            let mut error = ptr::null_mut();
            let prefix = prefix.to_glib_none();
            let r = ffi::ostree_repo_list_commit_objects_starting_with(
                repo,
                prefix.0,
                &mut commits,
                cancellable,
                &mut error,
            );
            if !error.is_null() {
                assert_eq!(r, 0);
                return Err(from_glib_full(error));
            }
            assert_ne!(r, 0);
            let mut ret = HashSet::with_capacity(glib::ffi::g_hash_table_size(commits) as usize);
            unsafe extern "C" fn visit_hash_table(
                key: *mut libc::c_void,
                _value: gpointer,
                r: *mut libc::c_void,
            ) -> glib::ffi::gboolean {
                let key: glib::Variant = from_glib_none(key as *const glib::ffi::GVariant);
                let checksum = crate::object_name_deserialize(&key).0;
                let r = &mut *(r as *mut HashSet<glib::GString>);
                r.insert(checksum);
                true.into()
            }
            glib::ffi::g_hash_table_foreach_remove(
                commits,
                Some(visit_hash_table),
                &mut ret as *mut HashSet<glib::GString> as *mut _,
            );
            Ok(ret)
        }
    }
}
