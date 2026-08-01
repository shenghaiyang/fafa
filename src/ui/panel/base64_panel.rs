use base64::Engine;
use base64::alphabet::{self, Alphabet};
use base64::engine::DecodePaddingMode;
use base64::engine::general_purpose::{GeneralPurpose, GeneralPurposeConfig};
use gpui::{Context, Entity, ReadGlobal, Render, SharedString, Window, div, prelude::*, px};

use crate::locale::L10nState;
use crate::ui::component::action_button::ActionButton;
use crate::ui::component::app_input::AppInput;
use crate::ui::component::error_block::ErrorBlock;
use crate::ui::component::option_label::option_label;
use crate::ui::component::radio::RadioButton;
use crate::ui::component::result_block::ResultBlock;
use crate::ui::component::text_input::{InputEvent, TextInput};

struct Base64Result {
    value: String,
    is_binary: bool,
}

pub struct Base64Panel {
    input: Entity<TextInput>,
    url_safe: bool,
    padded: bool,
    result: Option<Base64Result>,
    error: Option<String>,
}

impl Base64Panel {
    fn alphabet(&self) -> &'static Alphabet {
        if self.url_safe {
            &alphabet::URL_SAFE
        } else {
            &alphabet::STANDARD
        }
    }

    fn encode_engine(&self) -> GeneralPurpose {
        GeneralPurpose::new(
            self.alphabet(),
            GeneralPurposeConfig::new().with_encode_padding(self.padded),
        )
    }

    fn decode_engine(&self) -> GeneralPurpose {
        GeneralPurpose::new(
            self.alphabet(),
            GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent),
        )
    }

    fn encode(&mut self, cx: &mut Context<Self>) {
        let input = self.input.read(cx).value().to_string();
        self.result = Some(Base64Result {
            value: self.encode_engine().encode(input),
            is_binary: false,
        });
        self.error = None;
        cx.notify();
    }

    fn decode(&mut self, cx: &mut Context<Self>) {
        let input = self.input.read(cx).value().to_string();
        match self.decode_engine().decode(input) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(text) => {
                    self.result = Some(Base64Result {
                        value: text,
                        is_binary: false,
                    });
                }
                Err(not_utf8) => {
                    let hex = not_utf8
                        .into_bytes()
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<String>();
                    self.result = Some(Base64Result {
                        value: hex,
                        is_binary: true,
                    });
                }
            },
            Err(_) => {
                let l10n = L10nState::global(cx).l10n;
                self.error = Some(l10n.base64_error.to_string());
                self.result = None;
            }
        }
        cx.notify();
    }

    fn select_url_safe(&mut self, url_safe: bool, cx: &mut Context<Self>) {
        self.url_safe = url_safe;
        self.result = None;
        self.error = None;
        cx.notify();
    }

    fn select_padded(&mut self, padded: bool, cx: &mut Context<Self>) {
        self.padded = padded;
        self.result = None;
        self.error = None;
        cx.notify();
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.set_value("", cx));
        self.result = None;
        self.error = None;
        cx.notify();
    }

    pub fn new(cx: &mut Context<Self>) -> Self {
        let l10n = L10nState::global(cx).l10n;
        let input = cx.new(|cx| TextInput::new(l10n.base64_input_placeholder, cx));
        cx.subscribe(&input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.encode(cx),
            InputEvent::Changed => {}
        })
        .detach();

        Self {
            input,
            url_safe: false,
            padded: true,
            result: None,
            error: None,
        }
    }
}

impl Render for Base64Panel {
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
                    .items_center()
                    .gap(px(12.))
                    .child(option_label(l10n.base64_alphabet))
                    .child(
                        RadioButton::new("b64-radio-std", l10n.base64_standard)
                            .selected(!self.url_safe)
                            .on_click(
                                cx.listener(move |this, _, _, cx| this.select_url_safe(false, cx)),
                            ),
                    )
                    .child(
                        RadioButton::new("b64-radio-url", l10n.base64_url_safe)
                            .selected(self.url_safe)
                            .on_click(
                                cx.listener(move |this, _, _, cx| this.select_url_safe(true, cx)),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.))
                    .child(option_label(l10n.base64_padding))
                    .child(
                        RadioButton::new("b64-radio-pad", l10n.base64_padded)
                            .selected(self.padded)
                            .on_click(
                                cx.listener(move |this, _, _, cx| this.select_padded(true, cx)),
                            ),
                    )
                    .child(
                        RadioButton::new("b64-radio-nopad", l10n.base64_unpadded)
                            .selected(!self.padded)
                            .on_click(
                                cx.listener(move |this, _, _, cx| this.select_padded(false, cx)),
                            ),
                    ),
            )
            .child(AppInput::field(self.input.clone()))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.))
                    .child(
                        ActionButton::new(l10n.base64_encode, "b64-encode")
                            .on_click(cx.listener(|this, _, _, cx| this.encode(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.base64_decode, "b64-decode")
                            .on_click(cx.listener(|this, _, _, cx| this.decode(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.action_clear, "b64-clear")
                            .secondary()
                            .on_click(cx.listener(|this, _, _, cx| this.clear(cx))),
                    ),
            )
            .when_some(self.error.as_ref(), |this, err| {
                this.child(ErrorBlock::new(SharedString::from(err.clone())))
            })
            .when_some(self.result.as_ref(), |this, r| {
                let block = ResultBlock::new("b64-result", SharedString::from(r.value.clone()));
                this.child(if r.is_binary {
                    block.label("HEX")
                } else {
                    block
                })
            })
    }
}
