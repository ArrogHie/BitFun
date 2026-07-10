//! Per-process single-instance detection via OS primitives.
//!
//! On Windows a named kernel mutex ensures at most one process holds the
//! "primary" role.  On other platforms the function always returns `true`
//! because the system tray icon duplication issue is Windows-specific.

#[cfg(target_os = "windows")]
mod imp {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::sync::OnceLock;

    extern "system" {
        fn CreateMutexW(
            lp_mutex_attributes: *const std::ffi::c_void,
            b_initial_owner: i32,
            lp_name: *const u16,
        ) -> isize;
        fn GetLastError() -> u32;
    }

    const ERROR_ALREADY_EXISTS: u32 = 183;

    /// The handle is kept alive for the lifetime of the primary process so the
    /// kernel mutex is not destroyed prematurely.  Non-primary instances
    /// intentionally leak their handle — it is harmless and avoids a separate
    /// `CloseHandle` extern declaration (which would clash with the existing
    /// `windows`‑crate declaration in the same build graph).
    static PRIMARY_MUTEX: OnceLock<isize> = OnceLock::new();

    pub(crate) fn is_primary_instance_impl() -> bool {
        #[cfg(not(test))]
        let name: Vec<u16> = OsStr::new("Global\\BitFun_Desktop_Instance_Mutex")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        #[cfg(test)]
        let name: Vec<u16> = OsStr::new("Global\\BitFun_Desktop_Instance_Mutex_Test")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let handle = unsafe { CreateMutexW(std::ptr::null(), 0, name.as_ptr()) };
        if handle == 0 {
            // If we cannot create the mutex, conservatively claim we're the
            // primary instance so the tray icon is not silently lost.
            return true;
        }

        let is_primary = unsafe { GetLastError() } != ERROR_ALREADY_EXISTS;

        if is_primary {
            PRIMARY_MUTEX.set(handle).ok();
        }
        // Non-primary: intentionally leak the duplicate handle — the kernel
        // object is still owned by the primary process and the leaked handle
        // is reclaimed by the OS when this process exits.

        is_primary
    }
}

#[cfg(not(target_os = "windows"))]
mod imp {
    pub(crate) fn is_primary_instance_impl() -> bool {
        true
    }
}

pub(crate) fn is_primary_instance() -> bool {
    imp::is_primary_instance_impl()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_call_primary_second_not() {
        // The first call in a process should always report primary, and a
        // second call must not because the mutex is already held.  Verifying
        // both assertions in the same test avoids the ordering dependency
        // that would exist between two separate tests sharing the mutex.
        assert!(is_primary_instance(), "first call must report primary");
        assert!(
            !is_primary_instance(),
            "second call must not report primary"
        );
    }
}
