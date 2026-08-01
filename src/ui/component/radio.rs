use crate::ui::theme::MAIN_THEME;
use gpui::prelude::*;
use gpui::{
    App, ClickEvent, ElementId, IntoElement, RenderOnce, SharedString, StatefulInteractiveElement,
    Window, div, px,
};

#[derive(IntoElement)]
pub struct RadioButton {
    id: ElementId,
    label: SharedString,
    selected: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl RadioButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            selected: false,
            on_click: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for RadioButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = &MAIN_THEME;
        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.))
            .cursor_pointer()
            .when_some(self.on_click, |this, handler| {
                this.on_click(move |event, window, cx| handler(event, window, cx))
            })
            .child(
                div()
                    .size(px(14.))
                    .rounded(px(7.))
                    .border_1()
                    .border_color(if self.selected {
                        theme.accent
                    } else {
                        theme.text_dim
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(div().size(px(8.)).rounded(px(4.)).bg(if self.selected {
                        theme.accent
                    } else {
                        gpui::hsla(0., 0., 0., 0.)
                    })),
            )
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(if self.selected {
                        theme.text
                    } else {
                        theme.text_dim
                    })
                    .child(self.label),
            )
    }
}
