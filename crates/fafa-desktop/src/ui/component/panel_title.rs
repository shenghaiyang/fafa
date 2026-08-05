use gpui::{
    App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};
#[derive(IntoElement)]
pub struct PanelTitle {
    title: SharedString,
    size: f32,
    weight: FontWeight,
}

impl PanelTitle {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            size: 24.0,
            weight: FontWeight::SEMIBOLD,
        }
    }

    #[allow(dead_code)]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    #[allow(dead_code)]
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }
}

impl RenderOnce for PanelTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .px(px(16.))
            .text_size(px(self.size))
            .font_weight(self.weight)
            .child(self.title)
    }
}
