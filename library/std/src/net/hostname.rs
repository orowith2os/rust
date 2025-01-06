use crate::ffi::OsString;

/// Returns the system hostname.
///
/// The only time this should ever error out is if the system is broken
/// in some way. Such errors depend on those cases.
#[unstable(feature = "gethostname", issue = "135142")]
pub fn hostname() -> crate::io::Result<OsString> {
    crate::sys_common::net::gethostname()
}
