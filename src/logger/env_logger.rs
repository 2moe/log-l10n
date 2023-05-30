use crate::{level::color::get_l10n_level, time::get_offset_time};
use env_logger::Env;
use std::io::Write;

// #[cfg(feature = "color")]
use owo_colors::OwoColorize;

/// Initialises a logger using the `env_logger` crate. It sets up the log levels and formatting for log messages.
///
/// The closure uses the `get_offset_time` function to get the current time with an offset, and formats it as a timestamp with milliseconds and an optional offset indicator ("Z" for UTC).
/// It then calls the `match_log_level` function to get the localised log level string and corresponding `Style`.
/// Finally, it formats the log message using the provided `fmt` parameter and the log record's `module_path`, `line`, and `args`.
/// The resulting logger configuration is initialised using the `init` method on the `env_logger::Builder`.
pub fn init(env_name: &str, lv: Option<&str>) {
    env_logger::Builder::from_env(
        Env::new().filter_or(env_name, lv.unwrap_or("info")),
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
