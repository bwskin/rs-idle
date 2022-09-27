mod xinterface {
    use x11::xlib::{XCloseDisplay, XDefaultRootWindow, XFree, XOpenDisplay};
    use x11::xss::{XScreenSaverAllocInfo, XScreenSaverQueryExtension, XScreenSaverQueryInfo};
    use std::ffi::c_void;

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
                XScreenSaverQueryExtension(
                    self.display_ptr,
                    &mut event_basep,
                    &mut error_basep
                ) != 0
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

pub fn get_idle_time() -> u64 {
    let display = Display::open().unwrap_or_else(|_| {
        println!("Cannot open display");
        std::process::exit(1);
    });

    if !display.has_screen_saver_extesnsion() {
        println!("Screen doesn't have screen saver extension");
        std::process::exit(1);
    }

    let screen_saver_info = display.get_screen_saver_info().unwrap_or_else(|_| {
        println!("Cannot get screen saver info");
        std::process::exit(1);
    });

    screen_saver_info.idle_time()
}
