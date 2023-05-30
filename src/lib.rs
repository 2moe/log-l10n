use crate::assets::get_l10n_text;
use glossa::GetText;
mod assets;

#[cfg(feature = "logger")]
pub mod logger;

pub mod level;

#[cfg(feature = "time")]
pub mod time;

pub(crate) fn get_log_core_l10n(key: &str) -> &str {
    get_l10n_text()
        .get("log-core", key)
        .unwrap_or(key)
}
