#![no_std]

extern crate alloc;
mod os;
mod logger;

pub use logger::{Log, log_line, Logger};
pub use azathoth_utils::format_str;

/// Cyan ansi color
pub const CYAN: &str = "\x1B[36m";

/// Yellow ansi color
pub const YELLOW: &str = "\x1B[33m";

/// Red ansi color
pub const RED: &str = "\x1B[31m";
/// Red on cyan ansi color
pub const RED_ON_CYAN: &str = "\x1B[31;46m";

/// Bold red on cyan ansi color
pub const BOLD_RED_ON_CYAN: &str = "\x1B[1;31;46m";

/// Underline red on cyan ansi color
pub const UNDERLINE_RED_ON_CYAN: &str = "\x1B[4;31;46m";

/// Bold red ansi color
pub const RED_BOLD: &str = "\x1B[1;31m";

/// Green ansi color
pub const GREEN: &str = "\x1B[32m";

/// Purple ansi color
pub const PURPLE: &str = "\x1B[35m";

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
        $crate::log_line("INFO", &azathoth_utils::format_str!($($arg)*), file!(), line!(), $crate::CYAN)
    };
}

/// Macro for logging errors
#[macro_export]
macro_rules! error {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        let s = $crate::format_str!($fmt $(, $arg)*);
        $crate::__log_export::log_line("ERROR", &s, file!(), line!(), $crate::__log_export::RED_BOLD)
    }};
}

/// Macro for logging warnings
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log_line("WARN", &azathoth_utils::format_str!($($arg)*), file!(), line!(), $crate::YELLOW)
    };
}

/// Macro for logging successful operations
#[macro_export]
macro_rules! success {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        let s = $crate::format_str!($fmt $(, $arg)*);
        $crate::__log_export::log_line("SUCCESS", &s, file!(), line!(), $crate::__log_export::GREEN)
    }};
}
/// Macro for logging critical information, such as errors that should never happen or a "rare" event
#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {
        $crate::log_line("CRITICAL", &azathoth_utils::format_str!($($arg)*), file!(), line!(), $crate::BOLD_RED_ON_CYAN)
    };
}

/// Macro for logging debug statements
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::log_line("DEBUG", &azathoth_utils::format_str!($($arg)*), file!(), line!(), $crate::PURPLE_BOLD)
        }
    };
}

#[doc(hidden)]
pub mod __log_export {
    pub use super::{log_line, BOLD_RED_ON_CYAN, CYAN, GREEN, PURPLE_BOLD, RED_BOLD, YELLOW};
}