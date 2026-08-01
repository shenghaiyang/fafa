use crate::ui::component::copy_button::CopyButton;
use crate::ui::theme::MAIN_THEME;
use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    prelude::FluentBuilder, px,
};

#[derive(IntoElement)]
pub struct ResultBlock {
    id: SharedString,
    label: Option<SharedString>,
    value: SharedString,
}

impl ResultBlock {
    pub fn new(id: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: None,
            value: value.into(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for ResultBlock {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = &MAIN_THEME;
        let copy_id = format!("{}-copy", self.id);

        div()
            .w_full()
            .flex()
            .flex_col()
            .when_some(self.label, |this, label| {
                this.child(
                    div()
                        .text_size(px(13.))
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(theme.text_dim)
                        .child(label),
                )
            })
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .px(px(10.))
                    .py(px(8.))
                    .rounded(px(4.))
                    .border(px(1.))
                    .border_color(theme.border)
                    .items_center()
                    .bg(theme.result_bg)
                    .child(div().child(CopyButton::new(copy_id, &self.value)))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .w(px(300.))
                            .ml(px(8.))
                            .text_color(theme.result_text)
                            .text_size(px(16.))
                            .child(self.value),
                    ),
            )
    }
}
