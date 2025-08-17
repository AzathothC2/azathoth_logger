use azathoth_core::os::linux::consts::{STDOUT, SYS_WRITE};
use azathoth_core::os::linux::syscalls::syscall3;

pub struct LinuxInnerLog;

impl LinuxInnerLog {
    /// Returns empty struct on linux based systems
    #[unsafe(link_section = ".text")]
    pub const fn new() -> Self {
        Self
    }
    /// Empty on linux based systems
    pub fn init(&mut self) -> bool { true }
    
    #[unsafe(link_section = ".text")]
    pub(crate) fn log(&self, msg: &[u8]) {
        syscall3(SYS_WRITE, STDOUT, msg.as_ptr() as usize, msg.len());
    }
}