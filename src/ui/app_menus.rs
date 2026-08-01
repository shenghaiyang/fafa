use crate::locale::L10nState;
use gpui::{App, Menu, MenuItem, ReadGlobal, actions};

actions!(app, [Quit, OpenAbout,]);

pub fn app_menus(cx: &mut App) -> Vec<Menu> {
    let l10n = L10nState::global(cx).l10n;
    vec![Menu {
        name: l10n.app_name.into(),
        disabled: false,
        items: vec![
            MenuItem::action(l10n.menu_about, OpenAbout),
            MenuItem::separator(),
            MenuItem::action(l10n.menu_quit, Quit),
        ],
    }]
}
