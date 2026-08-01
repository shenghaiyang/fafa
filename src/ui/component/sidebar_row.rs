use gpui::prelude::FluentBuilder;
use gpui::{
    App, ClickEvent, ElementId, FontWeight, Hsla, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, px, svg,
};

#[derive(IntoElement)]
pub struct SidebarRow {
    id: ElementId,
    icon: SharedString,
    label: SharedString,
    selected: bool,
    active_bg: Hsla,
    hover_bg: Hsla,
    text_color: Hsla,
    active_text: Hsla,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl SidebarRow {
    pub fn new(
        id: impl Into<ElementId>,
        icon: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            label: label.into(),
            selected: false,
            active_bg: Hsla::default(),
            hover_bg: Hsla::default(),
            text_color: Hsla::default(),
            active_text: Hsla::default(),
            on_click: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn active_bg(mut self, color: Hsla) -> Self {
        self.active_bg = color;
        self
    }

    pub fn hover_bg(mut self, color: Hsla) -> Self {
        self.hover_bg = color;
        self
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = color;
        self
    }

    pub fn active_text(mut self, color: Hsla) -> Self {
        self.active_text = color;
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

impl RenderOnce for SidebarRow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let icon_color = if self.selected {
            self.active_text
        } else {
            self.text_color
        };

        div()
            .id(self.id)
            .w_full()
            .h(px(36.))
            .flex_shrink_0()
            .px(px(14.5))
            .py(px(4.))
            .rounded(px(100.))
            .flex()
            .flex_row()
            .items_center()
            .when_else(
                self.selected,
                |this| this.bg(self.active_bg).text_color(self.active_text),
                |this| {
                    this.text_color(self.text_color)
                        .hover(move |this| this.bg(self.hover_bg))
                },
            )
            .child(
                div().size(px(20.)).mr(px(10.)).child(
                    svg()
                        .path(self.icon)
                        .size_full()
                        .text_color(icon_color)
                        .flex_none(),
                ),
            )
            .text_size(px(13.))
            .font_weight(FontWeight::NORMAL)
            .child(self.label)
            .when_some(self.on_click, |this, on_click| this.on_click(on_click))
    }
}
