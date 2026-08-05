use crate::ui::theme::MAIN_THEME;
use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

#[derive(IntoElement)]
pub struct ErrorBlock {
    message: SharedString,
}

impl ErrorBlock {
    pub fn new(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl RenderOnce for ErrorBlock {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = &MAIN_THEME;
        div()
            .w_full()
            .px(px(10.))
            .py(px(8.))
            .rounded(px(4.))
            .bg(theme.text_dim.opacity(0.1))
            .text_color(theme.text_dim)
            .text_size(px(12.))
            .child(self.message)
    }
}
