// cargo +nightly rustdoc --all-features -- --cfg __log_l10n_doc --document-private-items ; open ./target/doc/log_l10n/index.html
#![cfg_attr(__log_l10n_doc, feature(doc_auto_cfg, doc_notable_trait))]

//! # Log-L10n
//!
//! L10n support for log.
//!
//! ## Quick Start
//!
//! ### add deps
//!
//! ```sh
//! cargo add log-l10n --features=env-logger
//! cargo add log
//! ```
//!
//! ```rust
//! use log_l10n::logger::{before_init, env_logger};
//!
//! let env_name = "CUSTOM_LOG_ENV";
//! let pkg = log_l10n::get_pkg_name!();
//!
//! before_init(pkg, env_name);
//! env_logger::init(env_name, Some("info"));
//!
//! log::info!("Hello");
//! log::error!("Err MSG");
//! log::debug!("World");
//! ```
//!
//! ![preview](https://raw.githubusercontent.com/2moe/log-l10n/main/assets/img/preview.jpg)
mod assets;

#[cfg(feature = "logger")]
pub mod logger;

pub mod level;

#[cfg(feature = "time")]
pub mod time;
