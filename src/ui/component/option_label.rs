use crate::ui::theme::MAIN_THEME;
use gpui::{FontWeight, IntoElement, ParentElement, Styled, div, px};

pub fn option_label(text: &'static str) -> impl IntoElement {
    div()
        .flex_none()
        .w(px(64.))
        .text_size(px(12.))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(MAIN_THEME.text_dim)
        .child(text)
}
