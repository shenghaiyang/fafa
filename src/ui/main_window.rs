use std::collections::HashMap;

use crate::locale::L10nState;
use crate::ui::component::panel_title::PanelTitle;
use crate::ui::component::sidebar_row::SidebarRow;
use crate::ui::nav::NavDestination;
use crate::ui::panel::base64_panel::Base64Panel;
use crate::ui::panel::color_panel::ColorPanel;
use crate::ui::panel::hash_panel::HashPanel;
use crate::ui::panel::number_base_panel::NumberBasePanel;
use crate::ui::panel::password_hash_panel::PasswordHashPanel;
use crate::ui::panel::qrcode_panel::QrCodePanel;
use crate::ui::panel::text_case_panel::TextCasePanel;
use crate::ui::panel::timestamp_panel::TimestampPanel;
use crate::ui::panel::url_panel::UrlPanel;
use crate::ui::panel::uuid_panel::UuidPanel;
use crate::ui::theme::MAIN_THEME;
use crate::ui::toast::ToastState;
use gpui::AnyView;
use gpui::ClickEvent;
use gpui::Context;
use gpui::FontWeight;
use gpui::ReadGlobal;
use gpui::TitlebarOptions;
use gpui::Window;
use gpui::WindowBackgroundAppearance;
use gpui::WindowBounds;
use gpui::WindowOptions;
use gpui::deferred;
use gpui::div;
use gpui::img;
use gpui::point;
use gpui::prelude::*;
use gpui::px;
use gpui::size;
use gpui::{App, linear_color_stop, linear_gradient};
use gpui_util::ResultExt;

pub struct MainWindow {
    current_nav: NavDestination,
    panels: HashMap<NavDestination, AnyView>,
}

impl MainWindow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let mut panels = HashMap::new();
        panels.insert(NavDestination::Hash, cx.new(HashPanel::new).into());
        panels.insert(
            NavDestination::PasswordHash,
            cx.new(PasswordHashPanel::new).into(),
        );
        panels.insert(NavDestination::QrCode, cx.new(QrCodePanel::new).into());
        panels.insert(
            NavDestination::Timestamp,
            cx.new(TimestampPanel::new).into(),
        );
        panels.insert(NavDestination::Url, cx.new(UrlPanel::new).into());
        panels.insert(
            NavDestination::NumberBase,
            cx.new(NumberBasePanel::new).into(),
        );
        panels.insert(NavDestination::Uuid, cx.new(UuidPanel::new).into());
        panels.insert(NavDestination::Base64, cx.new(Base64Panel::new).into());
        panels.insert(NavDestination::Color, cx.new(ColorPanel::new).into());
        panels.insert(NavDestination::TextCase, cx.new(TextCasePanel::new).into());

        cx.observe_global::<ToastState>(|_, cx| cx.notify())
            .detach();

        Self {
            current_nav: NavDestination::Hash,
            panels,
        }
    }

    fn on_select_nav(&mut self, dest: NavDestination, cx: &mut Context<Self>) {
        self.current_nav = dest;
        cx.notify();
    }

    fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let l10n = L10nState::global(cx).l10n;
        let theme = &MAIN_THEME;
        let nav_destinations = NavDestination::all();
        div()
            .id("sidebar")
            .w(px(240.))
            .h_full()
            .pt(px(40.))
            .bg(theme.bg)
            .flex()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .px(px(20.))
                    .py(px(14.))
                    .mb(px(4.))
                    .flex()
                    .flex_row()
                    .items_center()
                    .id("sidebar-title")
                    .child(
                        img("images/logo.webp")
                            .size(px(42.))
                            .border(px(2.))
                            .border_color(theme.border)
                            .overflow_hidden()
                            .bg(linear_gradient(
                                45.,
                                linear_color_stop(theme.app_icon_bg_from, 1.),
                                linear_color_stop(theme.app_icon_bg_to, 1.),
                            ))
                            .rounded_full(),
                    )
                    .child(
                        div()
                            .ml(px(12.))
                            .text_size(px(20.))
                            .text_color(theme.text)
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(l10n.app_name),
                    )
                    .on_click(cx.listener(|_, event: &ClickEvent, window, _| {
                        if event.click_count() == 2 {
                            window.zoom_window();
                        }
                    })),
            )
            .child(
                div()
                    .id("sidebar-nav")
                    .overflow_y_scroll()
                    .w_full()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .flex_col()
                    .gap(px(3.))
                    .px(px(12.))
                    .children(nav_destinations.into_iter().map(|dest| {
                        let is_active = dest == self.current_nav;
                        SidebarRow::new(
                            format!("sidebar-row-{:?}", dest),
                            dest.icon_path(),
                            dest.locale_key(l10n),
                        )
                        .selected(is_active)
                        .active_bg(theme.sidebar_active)
                        .hover_bg(theme.sidebar_hover)
                        .text_color(theme.sidebar_text)
                        .active_text(theme.sidebar_active_text)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.on_select_nav(dest, cx);
                        }))
                    })),
            )
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let l10n = L10nState::global(cx).l10n;
        let theme = &MAIN_THEME;

        let toast = ToastState::global(cx).message.clone();
        div()
            .size_full()
            .flex()
            .flex_row()
            // Sidebar
            .child(self.render_sidebar(cx))
            // Content area
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .pt(px(14.))
                    .pr(px(14.))
                    .pb(px(14.))
                    .bg(theme.panel_bg)
                    // Panel Title
                    .child(PanelTitle::new(self.current_nav.locale_key(l10n)))
                    // Panel
                    .child(
                        div()
                            .w_full()
                            .flex_1()
                            .min_h_0()
                            .mt(px(12.))
                            .p(px(16.))
                            .id("content")
                            .when_some(
                                self.panels.get(&self.current_nav).cloned(),
                                |div, panel| div.child(panel),
                            ),
                    ),
            )
            // Toast overlay
            .when_some(toast, |this, message| {
                this.child(deferred(
                    div()
                        .absolute()
                        .bottom(px(28.))
                        .left_0()
                        .right_0()
                        .flex()
                        .flex_row()
                        .justify_center()
                        .child(
                            div()
                                .px(px(16.))
                                .py(px(8.))
                                .rounded(px(8.))
                                .bg(theme.text)
                                .text_color(theme.panel_bg)
                                .text_size(px(12.))
                                .child(message),
                        ),
                ))
            })
    }
}

pub fn open_main_window(cx: &mut App) {
    if let Some(existing) = cx
        .windows()
        .into_iter()
        .find_map(|w| w.downcast::<MainWindow>())
    {
        cx.activate(true);
        existing
            .update(cx, |_main_window, window, _cx| {
                window.activate_window();
            })
            .log_err();
        return;
    }
    let l10n = L10nState::global(cx).l10n;
    let window_size = size(px(1100.), px(740.));
    cx.activate(true);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::centered(window_size, cx)),
            titlebar: Some(TitlebarOptions {
                title: Some(l10n.app_name.into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(19.), px(19.))),
            }),
            window_background: WindowBackgroundAppearance::Blurred,
            window_min_size: Some(size(px(640.), px(480.))),
            ..Default::default()
        },
        |window, cx| {
            let main_window = cx.new(MainWindow::new);
            window.activate_window();
            main_window
        },
    )
    .log_err();
}
