use gpui::{Context, Entity, ReadGlobal, Render, SharedString, Window, div, prelude::*, px};

use crate::locale::{L10n, L10nState};
use crate::ui::component::action_button::ActionButton;
use crate::ui::component::app_input::AppInput;
use crate::ui::component::error_block::ErrorBlock;
use crate::ui::component::option_label::option_label;
use crate::ui::component::radio::RadioButton;
use crate::ui::component::result_block::ResultBlock;
use crate::ui::component::text_input::{InputEvent, TextInput};

const INPUT_BASES: [NumberBase; 4] = [
    NumberBase::Binary,
    NumberBase::Octal,
    NumberBase::Decimal,
    NumberBase::Hex,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hex,
}

impl NumberBase {
    fn radix(self) -> u32 {
        match self {
            NumberBase::Binary => 2,
            NumberBase::Octal => 8,
            NumberBase::Decimal => 10,
            NumberBase::Hex => 16,
        }
    }

    fn label(self, l10n: &L10n) -> &'static str {
        match self {
            NumberBase::Binary => l10n.number_base_binary,
            NumberBase::Octal => l10n.number_base_octal,
            NumberBase::Decimal => l10n.number_base_decimal,
            NumberBase::Hex => l10n.number_base_hex,
        }
    }
}

pub struct NumberBasePanel {
    input: Entity<TextInput>,
    input_base: NumberBase,
    results: Vec<(String, String)>,
    error: Option<String>,
}

impl NumberBasePanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let l10n = L10nState::global(cx).l10n;
        let input = cx.new(|cx| TextInput::new(l10n.number_base_input_placeholder, cx));
        cx.subscribe(&input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.convert_base(cx),
            InputEvent::Changed => {}
        })
        .detach();

        Self {
            input,
            input_base: NumberBase::Decimal,
            results: Vec::new(),
            error: None,
        }
    }

    fn parse_input(&self, input: &str) -> Option<i64> {
        let cleaned = match self.input_base {
            NumberBase::Hex => input
                .strip_prefix("0x")
                .or_else(|| input.strip_prefix("0X"))
                .unwrap_or(input),
            NumberBase::Binary => input
                .strip_prefix("0b")
                .or_else(|| input.strip_prefix("0B"))
                .unwrap_or(input),
            _ => input,
        };
        i64::from_str_radix(cleaned, self.input_base.radix()).ok()
    }

    fn convert_base(&mut self, cx: &mut Context<Self>) {
        let input = self.input.read(cx).value().trim().to_string();
        if input.is_empty() {
            return;
        }
        let l10n = L10nState::global(cx).l10n;

        match self.parse_input(&input) {
            Some(n) => {
                self.results = vec![
                    (l10n.number_base_binary.to_string(), format!("{:b}", n)),
                    (l10n.number_base_octal.to_string(), format!("{:o}", n)),
                    (l10n.number_base_decimal.to_string(), n.to_string()),
                    (l10n.number_base_hex.to_string(), format!("{:x}", n)),
                ];
                self.error = None;
            }
            None => {
                self.error = Some(l10n.error_hint.to_string());
                self.results = Vec::new();
            }
        }
        cx.notify();
    }

    fn select_input_base(&mut self, input_base: NumberBase, cx: &mut Context<Self>) {
        self.input_base = input_base;
        let has_input = !self.input.read(cx).value().trim().is_empty();
        if has_input {
            self.convert_base(cx);
        } else {
            self.results = Vec::new();
            self.error = None;
            cx.notify();
        }
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.set_value("", cx));
        self.results.clear();
        self.error = None;
        cx.notify();
    }
}

impl Render for NumberBasePanel {
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
                    .child(option_label(l10n.number_base_input_base))
                    .children(INPUT_BASES.iter().map(|&input_base| {
                        RadioButton::new(
                            format!("base-input-{:?}", input_base),
                            input_base.label(l10n),
                        )
                        .selected(self.input_base == input_base)
                        .on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.select_input_base(input_base, cx)
                            }),
                        )
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.))
                    .child(AppInput::field(self.input.clone()))
                    .child(
                        ActionButton::new(l10n.number_base_convert, "base-convert")
                            .on_click(cx.listener(|this, _, _, cx| this.convert_base(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.action_clear, "base-clear")
                            .secondary()
                            .on_click(cx.listener(|this, _, _, cx| this.clear(cx))),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.))
                    .children(self.results.iter().map(|(name, value)| {
                        ResultBlock::new(format!("base-{}", name), value.clone())
                            .label(name.clone())
                    })),
            )
            .when_some(self.error.as_ref(), |this, err| {
                this.child(ErrorBlock::new(SharedString::from(err.clone())))
            })
    }
}
