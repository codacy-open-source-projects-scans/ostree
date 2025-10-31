#[cfg(any(feature = "v2017_13", feature = "dox"))]
use crate::ChecksumFlags;
use crate::{Checksum, ObjectType};
use glib::ffi::GFALSE;
use glib::{prelude::*, translate::*};
use std::{future::Future, mem::MaybeUninit, pin::Pin, ptr};

/// Compute the SHA-256 checksum of a file.
pub fn checksum_file<P: IsA<gio::File>, Q: IsA<gio::Cancellable>>(
    f: &P,
    objtype: ObjectType,
    cancellable: Option<&Q>,
) -> Result<Checksum, Box<dyn std::error::Error + Send>> {
    unsafe {
        let mut out_csum = ptr::null_mut();
        let mut error = ptr::null_mut();
        let ret = ffi::ostree_checksum_file(
            f.as_ref().to_glib_none().0,
            objtype.into_glib(),
            &mut out_csum,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            &mut error,
        );
        checksum_file_error(out_csum, error, ret)
    }
}

/// Asynchronously compute the SHA-256 checksum of a file.
pub fn checksum_file_async<
    P: IsA<gio::File>,
    Q: IsA<gio::Cancellable>,
    R: FnOnce(Result<Checksum, Box<dyn std::error::Error + Send>>) + Send + 'static,
>(
    f: &P,
    objtype: ObjectType,
    io_priority: i32,
    cancellable: Option<&Q>,
    callback: R,
) {
    let user_data: Box<R> = Box::new(callback);
    unsafe extern "C" fn checksum_file_async_trampoline<
        R: FnOnce(Result<Checksum, Box<dyn std::error::Error + Send>>) + Send + 'static,
    >(
        _source_object: *mut glib::gobject_ffi::GObject,
        res: *mut gio::ffi::GAsyncResult,
        user_data: glib::ffi::gpointer,
    ) {
        let mut error = ptr::null_mut();
        let mut out_csum = MaybeUninit::uninit();
        let ret = ffi::ostree_checksum_file_async_finish(
            _source_object as *mut _,
            res,
            out_csum.as_mut_ptr(),
            &mut error,
        );
        let out_csum = out_csum.assume_init();
        let result = checksum_file_error(out_csum, error, ret);
        let callback: Box<R> = Box::from_raw(user_data as *mut _);
        callback(result);
    }
    let callback = checksum_file_async_trampoline::<R>;
    unsafe {
        ffi::ostree_checksum_file_async(
            f.as_ref().to_glib_none().0,
            objtype.into_glib(),
            io_priority,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            Some(callback),
            Box::into_raw(user_data) as *mut _,
        );
    }
}

/// Asynchronously compute the SHA-256 checksum of a file.
#[allow(clippy::type_complexity)]
pub fn checksum_file_async_future<P: IsA<gio::File> + Clone + 'static>(
    f: &P,
    objtype: ObjectType,
    io_priority: i32,
) -> Pin<Box<dyn Future<Output = Result<Checksum, Box<dyn std::error::Error + Send>>> + 'static>> {
    let f = f.clone();
    Box::pin(gio::GioFuture::new(&f, move |f, cancellable, send| {
        checksum_file_async(f, objtype, io_priority, Some(cancellable), move |res| {
            send.resolve(res);
        });
    }))
}

/// Compute the OSTree checksum of a content object.
pub fn checksum_file_from_input<P: IsA<gio::InputStream>, Q: IsA<gio::Cancellable>>(
    file_info: &gio::FileInfo,
    xattrs: Option<&glib::Variant>,
    in_: Option<&P>,
    objtype: ObjectType,
    cancellable: Option<&Q>,
) -> Result<Checksum, Box<dyn std::error::Error + Send>> {
    unsafe {
        let mut out_csum = ptr::null_mut();
        let mut error = ptr::null_mut();
        let ret = ffi::ostree_checksum_file_from_input(
            file_info.to_glib_none().0,
            xattrs.to_glib_none().0,
            in_.map(|p| p.as_ref()).to_glib_none().0,
            objtype.into_glib(),
            &mut out_csum,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            &mut error,
        );
        checksum_file_error(out_csum, error, ret)
    }
}

/// Compute the OSTree checksum of a file.
#[cfg(any(feature = "v2017_13", feature = "dox"))]
pub fn checksum_file_at<P: IsA<gio::Cancellable>>(
    dfd: i32,
    path: &std::path::Path,
    stbuf: Option<&libc::stat>,
    objtype: ObjectType,
    flags: ChecksumFlags,
    cancellable: Option<&P>,
) -> Result<glib::GString, glib::Error> {
    unsafe {
        let mut out_checksum = ptr::null_mut();
        let mut error = ptr::null_mut();
        ffi::ostree_checksum_file_at(
            dfd,
            path.to_glib_none().0,
            stbuf
                .map(|p| p as *const libc::stat as *mut libc::stat)
                .unwrap_or(ptr::null_mut()),
            objtype.into_glib(),
            flags.into_glib(),
            &mut out_checksum,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(out_checksum))
        } else {
            Err(from_glib_full(error))
        }
    }
}

unsafe fn checksum_file_error(
    out_csum: *mut [*mut u8; 32],
    error: *mut glib::ffi::GError,
    ret: i32,
) -> Result<Checksum, Box<dyn std::error::Error + Send>> {
    if !error.is_null() {
        Err(Box::<glib::Error>::new(from_glib_full(error)))
    } else if ret == GFALSE {
        Err(Box::new(glib::bool_error!("unknown error")))
    } else {
        Ok(Checksum::from_glib_full(out_csum))
    }
}
