use crate::ui::theme::MAIN_THEME;
use gpui::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

#[derive(IntoElement)]
pub struct ActionButton {
    id: ElementId,
    label: SharedString,
    compact: bool,
    secondary: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl ActionButton {
    pub fn new(label: impl Into<SharedString>, id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            compact: false,
            secondary: false,
            on_click: None,
        }
    }

    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.secondary = true;
        self
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for ActionButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = &MAIN_THEME;
        let (padding, text_size) = if self.compact { (12., 12.) } else { (16., 13.) };
        div()
            .id(self.id)
            .flex_none()
            .h(px(32.))
            .px(px(padding))
            .rounded(px(6.))
            .text_size(px(text_size))
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .map(|this| {
                if self.secondary {
                    this.bg(theme.panel_bg)
                        .border_1()
                        .border_color(theme.border)
                        .text_color(theme.text_dim)
                        .hover(|this| this.bg(theme.sidebar_hover))
                } else {
                    this.bg(theme.button_bg)
                        .text_color(theme.button_text)
                        .hover(|this| this.bg(theme.button_hover))
                }
            })
            .child(self.label)
            .when_some(self.on_click, |this, on_click| this.on_click(on_click))
    }
}
