use crate::get_log_core_l10n;
use glossa::assets::OnceCell;

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

pub fn str_to_lv_usize(s: &str) -> usize {
    if let Ok(s) = LV.binary_search(&s) {
        return s + 1;
    }

    let l = s.to_ascii_lowercase();

    let Some(lcase) = l.get(..4) else {
        return 0
    };

    match lcase {
        "erro" => 1,
        "warn" => 2,
        "info" => 3,
        "debu" => 4,
        "trac" => 5,
        _ => 0,
    }
}

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
