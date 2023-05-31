use glossa::{assets::OnceCell, GetText};

use crate::assets::get_l10n_text;

#[cfg(feature = "color")]
pub mod color;

/// The log levels are arranged in a certain order, and the positions of `error` and `warn` cannot be swapped.
pub const LV: [&str; 5] = ["error", "warn", "info", "debug", "trace"];

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
pub enum Lv {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace,
}

pub(crate) fn get_log_core_l10n(key: &str) -> &str {
    get_l10n_text()
        .get("log-core", key)
        .unwrap_or(key)
}

/// Converts a string log level to its corresponding usize representation.
///
/// # Example
///
/// ```
/// use log_l10n::level::str_to_lv_usize;
///
/// let s = "DeBUG";
/// let lv = str_to_lv_usize(s);
/// assert_eq!(lv, 4);
/// ```
///
/// For better efficiency, please use `["error", "warn", "info", "debug", "trace"]` instead of a mix of uppercase and lowercase letters.
pub fn str_to_lv_usize(s: &str) -> usize {
    // If the log level is found in the LV slice, return its index plus 1.
    if let Ok(s) = LV.binary_search(&s) {
        return s + 1;
    }

    // Convert the input string to lowercase.
    let l = s.to_ascii_lowercase();

    // Check the first four characters of the lowercase string.
    // If the string only has three or fewer characters, return 0.
    let Some(lcase) = l.get(..4) else {
        return 0
    };

    // Match the lowercase string with one of the log levels and return its corresponding usize value.
    match lcase {
        "erro" => 1,
        "warn" => 2,
        "info" => 3,
        "debu" => 4,
        "trac" => 5,
        _ => 0,
    }
}

/// It is similar to `get_l10n_level()`, but without the colored styling. See also: [color::get_l10n_level](crate::level::color::get_l10n_level)
pub fn get_l10n_level_no_color<'a>(level: usize) -> &'a str {
    let lv = level.saturating_sub(1);
    unsafe { get_plain_level_arr().get_unchecked(lv) }
}

fn get_plain_level_arr<'a>() -> &'static [&'a str; 5] {
    static L: OnceCell<[&str; 5]> = OnceCell::new();
    L.get_or_init(init_plain_level_arr)
}

fn init_plain_level_arr<'a>() -> [&'a str; 5] {
    let mut lv_l10n = LV;
    for (i, value) in LV
        .into_iter()
        .map(get_log_core_l10n)
        .enumerate()
    {
        unsafe { *lv_l10n.get_unchecked_mut(i) = value }
    }
    lv_l10n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_plain_lv() {
        let lv = log::Level::Info;
        let s = get_l10n_level_no_color(lv as usize);
        dbg!(s);
    }
}
