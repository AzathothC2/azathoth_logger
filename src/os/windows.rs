use core::ffi::c_void;
use core::mem::transmute;
use core::ptr::null_mut;
use azathoth_core::os::windows::fn_defs::{GetStdHandle_t, WriteConsoleA_t};
use azathoth_libload::{get_proc_address, load_library};
use azathoth_utils::crc32;

/// Inner windows logger struct
pub struct WinInnerLog {
    get_std_handle: Option<GetStdHandle_t>,
    write_console: Option<WriteConsoleA_t>,
}

impl WinInnerLog {
    /// Creates a new windows Inner log object
    #[unsafe(link_section = ".text")]
    pub const fn new() -> Self {
        Self {
            get_std_handle: None,
            write_console: None,
        }
    }

    #[unsafe(link_section = ".text")]
    pub fn init(&mut self) -> bool {
        unsafe {
            let hasher = |name: &str| -> u32 { crc32(name)};
            let k32 = match load_library("KERNEL32", &hasher) {
                Some(ptr) => ptr,
                None => return false,
            };
            let get_std_handle = match get_proc_address(k32, &hasher, "GetStdHandle")
                .map(|f| transmute::<_, GetStdHandle_t>(f)) {
                Some(handle) => handle,
                None => return false,
            };
            let write_console = match get_proc_address(k32, &hasher, "WriteConsoleA")
                .map(|f| transmute::<_, WriteConsoleA_t>(f)) {
                Some(handle) => handle,
                None => return false,
            };
            self.get_std_handle = Some(get_std_handle);
            self.write_console = Some(write_console);
            true
        }
    }
    pub(crate) fn log(&self, msg: &[u8]) {
        unsafe {
            let gsh = match self.get_std_handle {
                Some(handle) => handle,
                None => return,
            };
            let write_console = match self.write_console {
                Some(write_console) => write_console,
                None => return,
            };
            let stdout_handle: u32 = 4294967285u32;
            let handle = gsh(stdout_handle);
            if handle.is_null() {
                return;
            }

            let mut written = 0;
            let _ = write_console(
                handle,
                msg.as_ptr() as *const c_void,
                msg.len() as u32,
                &mut written,
                null_mut(),
            );
        }
    }
}