#![no_std]

extern crate alloc;
mod logger;
mod os;

pub use azathoth_utils::format_str;
pub use logger::{Log, Logger, log_line};

/// Cyan ansi color
pub const CYAN: &str = "\x1B[36m";

/// Yellow ansi color
pub const YELLOW: &str = "\x1B[33m";

/// Bold red on cyan ansi color
pub const BOLD_RED_ON_CYAN: &str = "\x1B[1;31;46m";

/// Bold red ansi color
pub const RED_BOLD: &str = "\x1B[1;31m";

/// Green ansi color
pub const GREEN: &str = "\x1B[32m";

/// Bold purple ansi color
pub const PURPLE_BOLD: &str = "\x1B[1;35m";

/// Ansi reset parameter
pub const ANSI_RESET: &str = "\x1B[0m";

#[unsafe(link_section = ".text")]
pub static LOG: Log = Log::new();

/// Macro for logging general information
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::__log_export::log_line("INFO", &$crate::format_str!($($arg)*), file!(), line!(), $crate::__log_export::CYAN)
    };
}

/// Macro for logging errors
#[macro_export]
macro_rules! error {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        let s = $crate::format_str!($fmt $(, $arg)*);
        $crate::__log_export::log_line("ERROR", &$crate::format_str!($fmt $(, $arg)*), file!(), line!(), $crate::__log_export::RED_BOLD)
    }};
}

/// Macro for logging warnings
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::__log_export::log_line("WARN", &$crate::format_str!($fmt $(, $arg)*), file!(), line!(), $crate::__log_export::YELLOW)
    };
}

/// Macro for logging successful operations
#[macro_export]
macro_rules! success {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        $crate::__log_export::log_line("SUCCESS", &$crate::format_str!($fmt $(, $arg)*), file!(), line!(), $crate::__log_export::GREEN)
    }};
}
/// Macro for logging critical information, such as errors that should never happen or a "rare" event
#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {
        $crate::__log_export::log_line("CRITICAL", &$crate::format_str!($($arg)*), file!(), line!(), $crate::__log_export::BOLD_RED_ON_CYAN)
    };
}

/// Macro for logging debug statements
#[macro_export]
macro_rules! debug {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        #[cfg(debug_assertions)]
        {
            $crate::__log_export::log_line("DEBUG", &$crate::format_str!($fmt $(, $arg)*), file!(), line!(), $crate::__log_export::PURPLE_BOLD,);
        }
        #[cfg(not(debug_assertions))]
        { () }
    }};
}

#[doc(hidden)]
pub mod __log_export {
    pub use super::{BOLD_RED_ON_CYAN, CYAN, GREEN, PURPLE_BOLD, RED_BOLD, YELLOW, log_line};
}
