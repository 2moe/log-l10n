use crate::{level::color::get_l10n_level, time::get_offset_time};
use env_logger::Env;
use std::io::Write;

// #[cfg(feature = "color")]
use owo_colors::OwoColorize;

/// Initialises a logger using the `env_logger` crate. It sets up the log levels and formatting for log messages.
///
/// First, read the log level (e.g. info, debug, trace) from the specified environment variable. If the environment variable is empty, then use the value of `default_lv`. If `default_lv` is `None`, then the default value is `info`.
///
/// The closure uses the `get_offset_time` function to get the current time with an offset, and formats it as a timestamp with milliseconds and an optional offset indicator ("Z" for UTC).
///
/// It then calls the `get_l10n_level()` function to get the localised log level string with colour(color).
///
/// The resulting logger configuration is initialised using the `init` method on the `env_logger::Builder`.
///
/// # Example
///
/// ```
/// //use std::env::set_var;
/// //set_var("YOUR_CUSTOM_LOG_ENV", "debug");
///
/// log_l10n::logger::env_logger::init("YOUR_CUSTOM_LOG_ENV", Some("info"));
///
/// log::info!("Hello");
/// log::debug!("World");
/// log::error!("Err MSG");
/// ```
pub fn init(env_name: &str, default_lv: Option<&str>) {
    env_logger::Builder::from_env(
        Env::new().filter_or(env_name, default_lv.unwrap_or("info")),
    )
    .format(move |fmt, r| {
        let now = get_offset_time();
        let offset_str = if now.offset().is_utc() { "Z" } else { "" };
        writeln!(
            fmt,
            "{:02}:{:02}:{:02}.{:03}{} [{}] {}:{}  {}",
            now.hour(),
            now.minute(),
            now.second(),
            now.millisecond(),
            offset_str,
            get_l10n_level(r.level() as usize),
            r.module_path().unwrap_or(""),
            r.line().unwrap_or(0).blue(),
            r.args()
        )
    })
    .init();
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn init_logger() {
        crate::logger::env_logger::init("MY_CUSTOM_LOG_ENV", Some("info"));

        log::info!("Hello");
        log::error!("Err MSG");
        log::debug!("World");
    }
}
