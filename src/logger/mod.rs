use glossa::GetText;
use owo_colors::OwoColorize;
use std::env;

#[cfg(feature = "env-logger")]
pub mod env_logger;

use crate::{
    assets::get_l10n_text,
    level::{self, color::get_l10n_level},
    time::get_offset_time,
};

#[macro_export]
macro_rules! get_pkg_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Before initialising the logger, print the localised message "Initialising/Initializing" to stderr.
///
/// # Example
///
/// ```
/// use std::env::set_var;
/// use log_l10n::{logger::before_init, get_pkg_name};
///
/// set_var("RUST_LOG", "debug");
/// before_init(get_pkg_name!(), "RUST_LOG");
/// ```
pub fn before_init(pkg: &str, env: &str) {
    // Check if the environment variable specified by "env" exists
    match env::var(env).as_deref() {
        // If the environment variable exists and starts with "debug" or "trace", print out a debug log message
        Ok(v) if v.starts_with("debug") || v.starts_with("trace") => {
            eprintln!(
                // Print the current time offset using the "get_offset_time()" function
                "{} [{}] {}: {}",
                get_offset_time(),
                // Get the localised level text for "debug" using the "get_l10n_level()" function
                get_l10n_level(level::Lv::Debug as usize),
                // Print the package name passed in as "pkg"
                pkg,
                // Get the localised text for "log-core:init-logger" using the "get_l10n_text()" function and print it out.
                get_l10n_text()
                    .get("log-core", "init-logger")
                    .unwrap_or("")
                    .cyan()
            )
        }
        // If the environment variable doesn't exist or doesn't start with "debug" or "trace", do nothing
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use std::env::set_var;

    use super::*;

    #[test]
    fn show_init_msg() {
        set_var("RUST_LOG", "debug");
        before_init(get_pkg_name!(), "RUST_LOG");
    }
}
