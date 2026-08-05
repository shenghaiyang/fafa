use crate::locale::L10nState;
use crate::ui::theme::MONO_FONT;
use gpui::{
    App, AppContext, Context, FocusHandle, FontWeight, ReadGlobal, SharedString, TitlebarOptions,
    Window, WindowBounds, WindowKind, WindowOptions, div, img, prelude::*, px, size,
};
use gpui_util::ResultExt;

const REPO_URL: &str = env!("CARGO_PKG_REPOSITORY");

pub fn open_about_window(cx: &mut App) {
    struct AboutWindow {
        focus_handle: FocusHandle,
        version_line: SharedString,
        description: &'static str,
        app_name: &'static str,
    }

    impl AboutWindow {
        fn new(cx: &mut Context<Self>) -> Self {
            let l10n = L10nState::global(cx).l10n;
            let version = format!("{}{}", l10n.about_version_prefix, env!("CARGO_PKG_VERSION"),);
            Self {
                focus_handle: cx.focus_handle(),
                version_line: version.into(),
                description: l10n.about_description,
                app_name: l10n.app_name,
            }
        }
    }

    impl Render for AboutWindow {
        fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let theme = crate::ui::theme::MAIN_THEME;
            div()
                .size_full()
                .bg(theme.bg)
                .text_color(theme.text)
                .flex()
                .flex_col()
                .child(div().h(px(32.)))
                .p(px(16.))
                .child(
                    div()
                        .w_full()
                        .h_full()
                        .rounded(px(8.))
                        .border_1()
                        .border_color(theme.border)
                        .bg(theme.panel_bg)
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(6.))
                        .child(
                            img("images/logo.webp")
                                .rounded(px(12.))
                                .size(px(80.))
                                .mb(px(4.)),
                        )
                        .child(
                            div()
                                .text_size(px(23.))
                                .font_weight(FontWeight::BOLD)
                                .child(self.app_name),
                        )
                        .child(
                            div()
                                .text_size(px(13.))
                                .text_color(theme.text_dim)
                                .child(self.version_line.clone()),
                        )
                        .child(div().h(px(8.)))
                        .child(
                            div()
                                .text_size(px(14.))
                                .px(px(16.))
                                .text_center()
                                .child(SharedString::from(self.description)),
                        )
                        .child(div().h(px(12.)))
                        .child(
                            div()
                                .id("about-repo")
                                .text_size(px(12.))
                                .font_family(MONO_FONT)
                                .text_color(theme.accent)
                                .cursor_pointer()
                                .hover(|this| this.underline())
                                .child(REPO_URL)
                                .on_click(|_, _, cx| cx.open_url(REPO_URL)),
                        ),
                )
        }
    }

    if let Some(existing) = cx
        .windows()
        .into_iter()
        .find_map(|w| w.downcast::<AboutWindow>())
    {
        cx.activate(true);
        existing
            .update(cx, |about_window, window, cx| {
                window.activate_window();
                about_window.focus_handle.focus(window, cx);
            })
            .log_err();
        return;
    }

    let window_size = size(px(360.), px(420.));
    let l10n = L10nState::global(cx).l10n;
    cx.activate(true);
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(l10n.about_window_title.into()),
                appears_transparent: true,
                traffic_light_position: None,
            }),
            window_bounds: Some(WindowBounds::centered(window_size, cx)),
            is_resizable: false,
            is_minimizable: false,
            kind: WindowKind::Floating,
            ..Default::default()
        },
        |window, cx| {
            let about_window = cx.new(AboutWindow::new);
            let focus_handle = about_window.read(cx).focus_handle.clone();
            window.activate_window();
            focus_handle.focus(window, cx);
            about_window
        },
    )
    .log_err();
}
