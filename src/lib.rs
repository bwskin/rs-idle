mod xinterface {
    use std::ffi::c_void;
    use x11::xlib::{XCloseDisplay, XDefaultRootWindow, XFree, XOpenDisplay};
    use x11::xss::{XScreenSaverAllocInfo, XScreenSaverQueryExtension, XScreenSaverQueryInfo};

    pub struct Display {
        display_ptr: *mut x11::xlib::Display,
    }

    impl Display {
        pub fn open() -> Result<Display, ()> {
            let display_ptr;
            unsafe {
                display_ptr = XOpenDisplay(std::ptr::null());
            }

            if display_ptr.is_null() {
                return Err(());
            } else {
                return Ok(Display { display_ptr });
            }
        }
        pub fn has_screen_saver_extesnsion(&self) -> bool {
            let mut event_basep: i32 = 0;
            let mut error_basep: i32 = 0;
            unsafe {
                XScreenSaverQueryExtension(self.display_ptr, &mut event_basep, &mut error_basep)
                    != 0
            }
        }

        pub fn get_screen_saver_info(&self) -> Result<ScreenSaverInfo, ()> {
            let xssi_ptr;
            unsafe {
                xssi_ptr = XScreenSaverAllocInfo();
            }
            if xssi_ptr.is_null() {
                return Err(());
            }
            unsafe {
                let root_window = XDefaultRootWindow(self.display_ptr);
                if XScreenSaverQueryInfo(self.display_ptr, root_window, xssi_ptr) == 0 {
                    return Err(());
                }
            }
            Ok(ScreenSaverInfo { xssi_ptr })
        }
    }

    impl Drop for Display {
        fn drop(&mut self) {
            unsafe {
                XCloseDisplay(self.display_ptr);
            }
        }
    }

    pub struct ScreenSaverInfo {
        xssi_ptr: *mut x11::xss::XScreenSaverInfo,
    }

    impl ScreenSaverInfo {
        pub fn idle_time(&self) -> u64 {
            unsafe { (*self.xssi_ptr).idle }
        }
    }

    impl Drop for ScreenSaverInfo {
        fn drop(&mut self) {
            unsafe {
                XFree(self.xssi_ptr as *mut c_void);
            }
        }
    }
}

use xinterface::Display;

pub enum Error {
    CannotOpenDisplay,
    NoScreenSaverExtension,
    CannotGetScreenSaverinfo,
}

impl Error {
    pub fn message(&self) -> &'static str {
        match self {
            Self::CannotOpenDisplay => "Cannot open display",
            Self::NoScreenSaverExtension => "Screen doesn't have screen saver extension",
            Self::CannotGetScreenSaverinfo => "Cannot get screen saver info",
        }
    }
}

pub fn get_idle_time() -> Result<u64, Error> {
    let display = Display::open().or(Err(Error::CannotOpenDisplay))?;

    if !display.has_screen_saver_extesnsion() {
        return Err(Error::NoScreenSaverExtension);
    }

    let screen_saver_info = display
        .get_screen_saver_info()
        .or(Err(Error::CannotGetScreenSaverinfo))?;

    Ok(screen_saver_info.idle_time())
}
