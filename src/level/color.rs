use glossa::assets::OnceCell;
pub use owo_colors::{OwoColorize, Style, Styled};

use crate::level::get_plain_level_arr;

/// Gets the localized log level name with colored styles.
///
/// # Example
///
/// ```
/// // let debug_lv = log::Level::Debug;
/// // let s = get_l10n_level(debug_lv as usize);
/// // dbg!(s);
///
/// use log_l10n::level::{color::get_l10n_level, Lv};
///
/// let w = Lv::Warn;
/// let s = get_l10n_level(w as usize);
/// println!("{s}")
/// ```
pub fn get_l10n_level<'z, 'y, 'x>(level: usize) -> &'z Styled<&'y &'x str> {
    let lv = level.saturating_sub(1);
    unsafe { get_styled_level_arr().get_unchecked(lv) }
}
fn get_styled_level_arr<'y, 'x>() -> &'static [Styled<&'y &'x str>] {
    static L: OnceCell<[Styled<&&str>; 5]> = OnceCell::new();

    L.get_or_init(init_styled_level_arr)
}
fn init_styled_level_arr<'y, 'x>() -> [Styled<&'y &'x str>; 5] {
    let new_sty = Style::new;
    let d = || "".style(new_sty());
    let mut arr = [d(), d(), d(), d(), d()];

    for ((i, s), a) in get_plain_level_arr()
        .iter()
        .take(5)
        .enumerate()
        .zip(arr.as_mut())
    {
        match i {
            0 => *a = s.style(new_sty().bold().bright_red()),
            1 => {
                *a = s.style(
                    new_sty()
                        .bold()
                        .yellow()
                        .underline(),
                )
            }
            2 => *a = s.style(new_sty().green()),
            3 => *a = s.style(new_sty().blue()),
            4 => *a = s.style(new_sty().cyan()),
            _ => break,
        }
    }
    arr
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::str_to_lv_usize;
    #[test]
    fn get_log_lv() {
        let lv = log::Level::Warn as usize - 1;
        let s = &get_styled_level_arr()[lv];
        dbg!(s);
    }

    #[test]
    fn get_styled_lv() {
        let lv = log::Level::Debug;
        let s = get_l10n_level(lv as usize);
        dbg!(s);
    }

    #[test]
    fn str_to_lv() {
        let u = str_to_lv_usize("DebU!!!");
        dbg!(u, get_l10n_level(u));

        let e = str_to_lv_usize("erro");
        dbg!(e, get_l10n_level(e));
    }

    #[test]
    fn log_l10n_lv() {
        let w = crate::level::Lv::Warn;
        let s = get_l10n_level(w as usize);
        println!("{s}")
    }
}
