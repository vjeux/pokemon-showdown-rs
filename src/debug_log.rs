//! Debug logging macros
//!
//! These macros provide conditional debug logging that can be disabled at compile time
//! using the `debug-logging` feature flag.
//!
//! When `debug-logging` is disabled (default), these macros compile to nothing,
//! eliminating all formatting and I/O overhead.

/// Debug print to stdout - only when debug-logging feature is enabled
#[cfg(feature = "debug-logging")]
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}

#[cfg(not(feature = "debug-logging"))]
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        ()
    };
}

/// Debug print to stderr - only when debug-logging feature is enabled
#[cfg(feature = "debug-logging")]
#[macro_export]
macro_rules! debug_elog {
    ($($arg:tt)*) => {
        eprintln!($($arg)*)
    };
}

#[cfg(not(feature = "debug-logging"))]
#[macro_export]
macro_rules! debug_elog {
    ($($arg:tt)*) => {
        ()
    };
}
