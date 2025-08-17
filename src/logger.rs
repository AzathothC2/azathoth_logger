use alloc::vec::Vec;
use core::cell::UnsafeCell;
use crate::ANSI_RESET;
#[cfg(target_os = "windows")]
use crate::os::windows::WinInnerLog as InnerLog;
#[cfg(target_os="linux")]
use crate::os::linux::LinuxInnerLog as InnerLog;
/// Small trait to track and extend functionality
pub trait Logger {
    /// Logs a string value
    fn log(&self, msg: &str);

    /// Logs a byte array
    fn log_bytes(&self, bytes: &[u8]);
}

/// Simple logger for no-std situations
pub struct Log {
    inner: UnsafeCell<InnerLog>,
}

unsafe impl Send for Log {}
unsafe impl Sync for Log {}

impl Logger for Log {
    fn log(&self, msg: &str) {
        self.get_ref().log(msg.as_bytes())
    }
    fn log_bytes(&self, bytes: &[u8]) {
        self.get_ref().log(bytes)
    }
}


impl Log {
    pub(crate) const fn new() -> Log {
        Self {
            inner: UnsafeCell::new(InnerLog::new()),
        }
    }

    /// Initializes the log object
    ///
    /// **This function must also be called at least once before using the logging functionality**
    /// # Safety
    /// This function expects the [`Log::new()`] function to be called at least once to initialize the inner log
    ///
    /// # Example
    ///
    /// ```no-run
    /// fn main() {
    ///     if unsafe { !azathoth_logger::LOG.init() } {
    ///        return; // Failed
    ///     }
    ///     azathoth_logger::success!("Hello world!");
    /// }
    pub unsafe fn init(&self) -> bool {
        unsafe { (*self.inner.get()).init() }
    }
    #[unsafe(link_section = ".text")]
    fn get_ref(&self) -> &InnerLog {
        unsafe { &*self.inner.get() }
    }
}

#[inline(always)]
#[unsafe(link_section = ".text")]
fn u32_to_str_buf(mut n: u32, buf: &mut [u8; 10]) -> &[u8] {
    let mut i = 10;
    if n == 0 {
        i -= 1;
        buf[i] = b'0';
    } else {
        while n > 0 {
            i -= 1;
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
        }
    }
    &buf[i..]
}

#[unsafe(link_section = ".text")]
pub fn log_line(level: &str, msg: &str, file: &str, line: u32, color_code: &str) {
    let mut buf = Vec::new();
    buf.extend_from_slice(color_code.as_bytes());
    buf.extend_from_slice(b"[");
    buf.extend_from_slice(level.as_bytes());
    buf.extend_from_slice(b"][");
    buf.extend_from_slice(file.as_bytes());
    buf.extend_from_slice(b":");
    let mut line_buf = [0u8; 10];
    let line_str = u32_to_str_buf(line, &mut line_buf);
    buf.extend_from_slice(line_str);
    buf.extend_from_slice(b"]: ");
    buf.extend_from_slice(msg.as_bytes());
    buf.extend_from_slice(ANSI_RESET.as_bytes());
    buf.extend_from_slice(b"\n");
    crate::LOG.log_bytes(&buf)
}


