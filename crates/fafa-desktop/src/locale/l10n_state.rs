use crate::locale::L10n;
use crate::locale::Locale;
use crate::locale::en_us::EN_US;
use crate::locale::zh_cn::ZH_CN;
use gpui::{App, Global};

pub struct L10nState {
    pub l10n: &'static L10n,
}

fn get_l10n(locale: Locale) -> &'static L10n {
    match locale {
        Locale::EnUs => &EN_US,
        Locale::ZhCn => &ZH_CN,
    }
}

fn system_locale() -> Locale {
    match sys_locale::get_locale() {
        Some(tag) if tag.to_ascii_lowercase().starts_with("zh") => Locale::ZhCn,
        _ => Locale::EnUs,
    }
}

impl L10nState {
    pub fn init(cx: &mut App) {
        let state = Self {
            l10n: get_l10n(system_locale()),
        };
        cx.set_global(state);
    }
}

impl Global for L10nState {}
