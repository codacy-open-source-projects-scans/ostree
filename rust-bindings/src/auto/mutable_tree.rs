// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2018_7", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_7")))]
use crate::Repo;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "OstreeMutableTree")]
    pub struct MutableTree(Object<ffi::OstreeMutableTree, ffi::OstreeMutableTreeClass>);

    match fn {
        type_ => || ffi::ostree_mutable_tree_get_type(),
    }
}

impl MutableTree {
    #[doc(alias = "ostree_mutable_tree_new")]
    pub fn new() -> MutableTree {
        unsafe {
            from_glib_full(ffi::ostree_mutable_tree_new())
        }
    }

    #[cfg(any(feature = "v2018_7", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_7")))]
    #[doc(alias = "ostree_mutable_tree_new_from_checksum")]
    #[doc(alias = "new_from_checksum")]
    pub fn from_checksum(repo: &Repo, contents_checksum: &str, metadata_checksum: &str) -> MutableTree {
        unsafe {
            from_glib_full(ffi::ostree_mutable_tree_new_from_checksum(repo.to_glib_none().0, contents_checksum.to_glib_none().0, metadata_checksum.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2021_5", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2021_5")))]
    #[doc(alias = "ostree_mutable_tree_new_from_commit")]
    #[doc(alias = "new_from_commit")]
    pub fn from_commit(repo: &Repo, rev: &str) -> Result<MutableTree, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ostree_mutable_tree_new_from_commit(repo.to_glib_none().0, rev.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2018_7", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_7")))]
    #[doc(alias = "ostree_mutable_tree_check_error")]
    pub fn check_error(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_mutable_tree_check_error(self.to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_mutable_tree_ensure_dir")]
    pub fn ensure_dir(&self, name: &str) -> Result<MutableTree, glib::Error> {
        unsafe {
            let mut out_subdir = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_mutable_tree_ensure_dir(self.to_glib_none().0, name.to_glib_none().0, &mut out_subdir, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(from_glib_full(out_subdir)) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_mutable_tree_ensure_parent_dirs")]
    pub fn ensure_parent_dirs(&self, split_path: &[&str], metadata_checksum: &str) -> Result<MutableTree, glib::Error> {
        unsafe {
            let mut out_parent = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_mutable_tree_ensure_parent_dirs(self.to_glib_none().0, split_path.to_glib_none().0, metadata_checksum.to_glib_none().0, &mut out_parent, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(from_glib_full(out_parent)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2018_7", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_7")))]
    #[doc(alias = "ostree_mutable_tree_fill_empty_from_dirtree")]
    pub fn fill_empty_from_dirtree(&self, repo: &Repo, contents_checksum: &str, metadata_checksum: &str) -> bool {
        unsafe {
            from_glib(ffi::ostree_mutable_tree_fill_empty_from_dirtree(self.to_glib_none().0, repo.to_glib_none().0, contents_checksum.to_glib_none().0, metadata_checksum.to_glib_none().0))
        }
    }

    #[doc(alias = "ostree_mutable_tree_get_contents_checksum")]
    #[doc(alias = "get_contents_checksum")]
    pub fn contents_checksum(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::ostree_mutable_tree_get_contents_checksum(self.to_glib_none().0))
        }
    }

    //#[doc(alias = "ostree_mutable_tree_get_files")]
    //#[doc(alias = "get_files")]
    //pub fn files(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe { TODO: call ffi:ostree_mutable_tree_get_files() }
    //}

    #[doc(alias = "ostree_mutable_tree_get_metadata_checksum")]
    #[doc(alias = "get_metadata_checksum")]
    pub fn metadata_checksum(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::ostree_mutable_tree_get_metadata_checksum(self.to_glib_none().0))
        }
    }

    //#[doc(alias = "ostree_mutable_tree_get_subdirs")]
    //#[doc(alias = "get_subdirs")]
    //pub fn subdirs(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 1, id: 23 } {
    //    unsafe { TODO: call ffi:ostree_mutable_tree_get_subdirs() }
    //}

    #[doc(alias = "ostree_mutable_tree_lookup")]
    pub fn lookup(&self, name: &str) -> Result<(Option<glib::GString>, Option<MutableTree>), glib::Error> {
        unsafe {
            let mut out_file_checksum = ptr::null_mut();
            let mut out_subdir = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_mutable_tree_lookup(self.to_glib_none().0, name.to_glib_none().0, &mut out_file_checksum, &mut out_subdir, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok((from_glib_full(out_file_checksum), from_glib_full(out_subdir))) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2018_9", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_9")))]
    #[doc(alias = "ostree_mutable_tree_remove")]
    pub fn remove(&self, name: &str, allow_noent: bool) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_mutable_tree_remove(self.to_glib_none().0, name.to_glib_none().0, allow_noent.into_glib(), &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_mutable_tree_replace_file")]
    pub fn replace_file(&self, name: &str, checksum: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_mutable_tree_replace_file(self.to_glib_none().0, name.to_glib_none().0, checksum.to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[doc(alias = "ostree_mutable_tree_set_contents_checksum")]
    pub fn set_contents_checksum(&self, checksum: &str) {
        unsafe {
            ffi::ostree_mutable_tree_set_contents_checksum(self.to_glib_none().0, checksum.to_glib_none().0);
        }
    }

    #[doc(alias = "ostree_mutable_tree_set_metadata_checksum")]
    pub fn set_metadata_checksum(&self, checksum: &str) {
        unsafe {
            ffi::ostree_mutable_tree_set_metadata_checksum(self.to_glib_none().0, checksum.to_glib_none().0);
        }
    }

    #[doc(alias = "ostree_mutable_tree_walk")]
    pub fn walk(&self, split_path: &[&str], start: u32) -> Result<MutableTree, glib::Error> {
        unsafe {
            let mut out_subdir = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::ostree_mutable_tree_walk(self.to_glib_none().0, split_path.to_glib_none().0, start, &mut out_subdir, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(from_glib_full(out_subdir)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl Default for MutableTree {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

impl fmt::Display for MutableTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MutableTree")
    }
}
