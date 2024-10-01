// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::RepoFinder;
#[cfg(any(feature = "v2018_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_6")))]
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "OstreeRepoFinderOverride")]
    pub struct RepoFinderOverride(Object<ffi::OstreeRepoFinderOverride, ffi::OstreeRepoFinderOverrideClass>) @implements RepoFinder;

    match fn {
        type_ => || ffi::ostree_repo_finder_override_get_type(),
    }
}

impl RepoFinderOverride {
    #[cfg(any(feature = "v2018_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_6")))]
    #[doc(alias = "ostree_repo_finder_override_new")]
    pub fn new() -> RepoFinderOverride {
        unsafe {
            from_glib_full(ffi::ostree_repo_finder_override_new())
        }
    }

    #[cfg(any(feature = "v2018_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_6")))]
    #[doc(alias = "ostree_repo_finder_override_add_uri")]
    pub fn add_uri(&self, uri: &str) {
        unsafe {
            ffi::ostree_repo_finder_override_add_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }
}

#[cfg(any(feature = "v2018_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2018_6")))]
impl Default for RepoFinderOverride {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

impl fmt::Display for RepoFinderOverride {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RepoFinderOverride")
    }
}