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
/// Macro for logging general information (INFO)
#[macro_export]
macro_rules! info {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        let s = $crate::format_str!($fmt $(, $arg)*);
        $crate::__log_export::log_line("INFO", &s, file!(), line!(), $crate::__log_export::CYAN);
    }};
    ($s:expr $(,)?) => {{
        $crate::__log_export::log_line("INFO", &$s, file!(), line!(), $crate::__log_export::CYAN);
    }};
}

/// Macro for logging errors (ERROR)
#[macro_export]
macro_rules! error {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        let s = $crate::format_str!($fmt $(, $arg)*);
        $crate::__log_export::log_line("ERROR", &s, file!(), line!(), $crate::__log_export::RED_BOLD);
    }};
    ($s:expr $(,)?) => {{
        $crate::__log_export::log_line("ERROR", &$s, file!(), line!(), $crate::__log_export::RED_BOLD);
    }};
}

/// Macro for logging warnings (WARN)
#[macro_export]
macro_rules! warn {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        let s = $crate::format_str!($fmt $(, $arg)*);
        $crate::__log_export::log_line("WARN", &s, file!(), line!(), $crate::__log_export::YELLOW);
    }};
    ($s:expr $(,)?) => {{
        $crate::__log_export::log_line("WARN", &$s, file!(), line!(), $crate::__log_export::YELLOW);
    }};
}

/// Macro for logging success messages (SUCCESS)
#[macro_export]
macro_rules! success {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        let s = $crate::format_str!($fmt $(, $arg)*);
        $crate::__log_export::log_line("SUCCESS", &s, file!(), line!(), $crate::__log_export::GREEN);
    }};
    ($s:expr $(,)?) => {{
        $crate::__log_export::log_line("SUCCESS", &$s, file!(), line!(), $crate::__log_export::GREEN);
    }};
}

/// Macro for logging critical notifications (CRITICAL)
#[macro_export]
macro_rules! critical {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        let s = $crate::format_str!($fmt $(, $arg)*);
        $crate::__log_export::log_line("CRITICAL", &s, file!(), line!(), $crate::__log_export::BOLD_RED_ON_CYAN);
    }};
    ($s:expr $(,)?) => {{
        $crate::__log_export::log_line("CRITICAL", &$s, file!(), line!(), $crate::__log_export::BOLD_RED_ON_CYAN);
    }};
}

/// DEBUG (no-op in release)
#[macro_export]
macro_rules! debug {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        #[cfg(debug_assertions)]
        {
            let s = $crate::format_str!($fmt $(, $arg)*);
            $crate::__log_export::log_line("DEBUG", &s, file!(), line!(), $crate::__log_export::PURPLE_BOLD);
        }
        #[cfg(not(debug_assertions))] { () }
    }};
    ($s:expr $(,)?) => {{
        #[cfg(debug_assertions)]
        {
            $crate::__log_export::log_line("DEBUG", &$s, file!(), line!(), $crate::__log_export::PURPLE_BOLD);
        }
        #[cfg(not(debug_assertions))] { () }
    }};
}


#[doc(hidden)]
pub mod __log_export {
    pub use super::{BOLD_RED_ON_CYAN, CYAN, GREEN, PURPLE_BOLD, RED_BOLD, YELLOW, log_line};
}
