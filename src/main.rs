mod locale;
mod logging;
mod ui;

use crate::ui::assets::Assets;
use anyhow::Context;
use gpui::{App, Application};
use gpui_platform::current_platform;
use locale::L10nState;
use tracing::debug;
use ui::about_window::open_about_window;
use ui::app_menus::{OpenAbout, Quit, app_menus};
use ui::component::text_input::bind_keys;
use ui::main_window::open_main_window;
use ui::toast::ToastState;

fn main() -> anyhow::Result<()> {
    let _guards = logging::init_tracing().with_context(|| "failed to initialize logging")?;

    let platform = current_platform(false);
    let app = Application::new_inaccessible(platform).with_assets(Assets);
    app.on_reopen(|cx| {
        debug!("on reopen");
        open_main_window(cx);
    });
    app.run(move |cx: &mut App| {
        Assets.load_fonts(cx);
        L10nState::init(cx);
        ToastState::init(cx);

        cx.on_action(|_: &Quit, cx| {
            cx.quit();
        });
        cx.on_action(|_: &OpenAbout, cx| {
            open_about_window(cx);
        });

        let menus = app_menus(cx);
        cx.set_menus(menus);
        bind_keys(cx);

        cx.activate(true);
        open_main_window(cx);
    });
    Ok(())
}
