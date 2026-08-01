use crate::ui::component::text_input::TextInput;
use crate::ui::theme::MAIN_THEME;
use gpui::prelude::*;
use gpui::{
    App, ClickEvent, Entity, InteractiveElement, IntoElement, RenderOnce, Window, div, px, svg,
};

#[derive(IntoElement)]
pub struct AppInput {
    input: Entity<TextInput>,
    clear_handler: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl AppInput {
    pub fn field(input: Entity<TextInput>) -> Self {
        Self {
            input,
            clear_handler: None,
        }
    }

    pub fn with_clear<F>(mut self, handler: F) -> Self
    where
        F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    {
        self.clear_handler = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for AppInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = &MAIN_THEME;
        div()
            .flex_1()
            .flex()
            .items_center()
            .h(px(32.))
            .rounded(px(6.))
            .bg(theme.input_bg)
            .border_1()
            .border_color(theme.input_border)
            .overflow_hidden()
            .child(div().flex_1().min_w_0().child(self.input))
            .when_some(self.clear_handler, |this, handler| {
                let clear = handler;
                this.child(
                    div()
                        .id("clear-")
                        .mr(px(4.))
                        .size(px(22.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(px(4.))
                        .cursor_pointer()
                        .hover(|this| this.bg(gpui::hsla(0., 0., 0., 0.08)))
                        .child(
                            svg()
                                .text_color(theme.accent)
                                .path("icons/x.svg")
                                .size(px(14.)),
                        )
                        .on_click(move |event, window, cx| clear(event, window, cx)),
                )
            })
    }
}
