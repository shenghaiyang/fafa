use crate::locale::L10nState;
use crate::ui::theme::MAIN_THEME;
use crate::ui::toast::show_toast;
use gpui::{
    App, ClipboardItem, ElementId, InteractiveElement, IntoElement, ParentElement, ReadGlobal,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, px, svg,
};

#[derive(IntoElement)]
pub struct CopyButton {
    id: ElementId,
    text: SharedString,
}

impl CopyButton {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

impl RenderOnce for CopyButton {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = &MAIN_THEME;
        let text = self.text.clone();
        div()
            .id(self.id)
            .flex_none()
            .size(px(24.))
            .p(px(4.))
            .rounded(px(4.))
            .hover(|this| this.bg(theme.sidebar_hover))
            .child(
                svg()
                    .path("icons/copy.svg")
                    .size_full()
                    .text_color(theme.sidebar_text),
            )
            .on_click(move |_, _, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(text.to_string()));
                let message = L10nState::global(cx).l10n.copy_success;
                show_toast(cx, message);
            })
    }
}
