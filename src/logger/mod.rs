use glossa::GetText;
use std::env;

#[cfg(feature = "env-logger")]
pub mod env_logger;

use crate::{
    assets::get_l10n_text,
    level::{self, color::get_l10n_level},
    time::get_offset_time,
};

pub fn before_init(pkg: &str, env: &str) {
    match env::var(env).as_deref() {
        Ok(v) if v.starts_with("debug") || v.starts_with("trace") => {
            eprintln!(
                "{} [{}] {}: {}",
                get_offset_time(),
                get_l10n_level(level::Lv::Debug as usize),
                pkg,
                get_l10n_text()
                    .get("log-core", "init-logger")
                    .unwrap_or("")
            )
        }
        _ => {}
    }
}
