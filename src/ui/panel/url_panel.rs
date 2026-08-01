use gpui::{Context, Entity, ReadGlobal, Render, SharedString, Window, div, prelude::*, px};
use percent_encoding::{NON_ALPHANUMERIC, percent_decode, utf8_percent_encode};

use crate::locale::L10nState;
use crate::ui::component::action_button::ActionButton;
use crate::ui::component::app_input::AppInput;
use crate::ui::component::result_block::ResultBlock;
use crate::ui::component::text_input::{InputEvent, TextInput};

pub struct UrlPanel {
    input: Entity<TextInput>,
    result: Option<String>,
}

impl UrlPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let l10n = L10nState::global(cx).l10n;
        let input = cx.new(|cx| TextInput::new(l10n.url_input_placeholder, cx));
        cx.subscribe(&input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.url_encode(cx),
            InputEvent::Changed => {}
        })
        .detach();

        Self {
            input,
            result: None,
        }
    }

    fn url_encode(&mut self, cx: &mut Context<Self>) {
        let input = self.input.read(cx).value().to_string();
        self.result = Some(utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string());
        cx.notify();
    }

    fn url_decode(&mut self, cx: &mut Context<Self>) {
        let input = self.input.read(cx).value().to_string();
        self.result = Some(
            percent_decode(input.as_bytes())
                .decode_utf8()
                .map(|c| c.to_string())
                .unwrap_or(input),
        );
        cx.notify();
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.set_value("", cx));
        self.result = None;
        cx.notify();
    }
}

impl Render for UrlPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let l10n = L10nState::global(cx).l10n;

        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.))
                    .child(AppInput::field(self.input.clone())),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.))
                    .child(
                        ActionButton::new(l10n.url_encode, "url-encode")
                            .on_click(cx.listener(|this, _, _, cx| this.url_encode(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.url_decode, "url-decode")
                            .on_click(cx.listener(|this, _, _, cx| this.url_decode(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.action_clear, "url-clear")
                            .secondary()
                            .on_click(cx.listener(|this, _, _, cx| this.clear(cx))),
                    ),
            )
            .when_some(self.result.as_ref(), |this, result| {
                this.child(ResultBlock::new(
                    "url-result",
                    SharedString::from(result.clone()),
                ))
            })
    }
}
