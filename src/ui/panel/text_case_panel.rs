use gpui::{Context, Entity, ReadGlobal, Render, Window, div, prelude::*, px};
use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTrainCase,
};

use crate::locale::{L10n, L10nState};
use crate::ui::component::action_button::ActionButton;
use crate::ui::component::app_input::AppInput;
use crate::ui::component::result_block::ResultBlock;
use crate::ui::component::text_input::{InputEvent, TextInput};

struct CaseResult {
    label: &'static str,
    value: String,
}

fn empty_results(l10n: &'static L10n) -> Vec<CaseResult> {
    vec![
        CaseResult {
            label: l10n.text_case_camel,
            value: String::new(),
        },
        CaseResult {
            label: l10n.text_case_pascal,
            value: String::new(),
        },
        CaseResult {
            label: l10n.text_case_snake,
            value: String::new(),
        },
        CaseResult {
            label: l10n.text_case_shouty_snake,
            value: String::new(),
        },
        CaseResult {
            label: l10n.text_case_kebab,
            value: String::new(),
        },
        CaseResult {
            label: l10n.text_case_shouty_kebab,
            value: String::new(),
        },
        CaseResult {
            label: l10n.text_case_train,
            value: String::new(),
        },
        CaseResult {
            label: l10n.text_case_title_case,
            value: String::new(),
        },
    ]
}

pub struct TextCasePanel {
    input: Entity<TextInput>,
    results: Vec<CaseResult>,
}

impl TextCasePanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let l10n = L10nState::global(cx).l10n;
        let input = cx.new(|cx| TextInput::new(l10n.text_case_input_placeholder, cx));
        cx.subscribe(&input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.convert_case(cx),
            InputEvent::Changed => {}
        })
        .detach();

        Self {
            input,
            results: empty_results(l10n),
        }
    }

    fn convert_case(&mut self, cx: &mut Context<Self>) {
        let input = self.input.read(cx).value().trim().to_string();
        if input.is_empty() {
            return;
        }

        let l10n = L10nState::global(cx).l10n;
        self.results = vec![
            CaseResult {
                label: l10n.text_case_camel,
                value: input.to_lower_camel_case(),
            },
            CaseResult {
                label: l10n.text_case_pascal,
                value: input.to_pascal_case(),
            },
            CaseResult {
                label: l10n.text_case_snake,
                value: input.to_snake_case(),
            },
            CaseResult {
                label: l10n.text_case_shouty_snake,
                value: input.to_shouty_snake_case(),
            },
            CaseResult {
                label: l10n.text_case_kebab,
                value: input.to_kebab_case(),
            },
            CaseResult {
                label: l10n.text_case_shouty_kebab,
                value: input.to_shouty_kebab_case(),
            },
            CaseResult {
                label: l10n.text_case_train,
                value: input.to_train_case(),
            },
            CaseResult {
                label: l10n.text_case_title_case,
                value: {
                    let pascal = input.to_pascal_case();
                    let mut result = String::new();
                    for (i, ch) in pascal.chars().enumerate() {
                        if i > 0 && ch.is_uppercase() {
                            result.push(' ');
                        }
                        result.push(ch);
                    }
                    result
                },
            },
        ];
        cx.notify();
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.set_value("", cx));
        for result in &mut self.results {
            result.value.clear();
        }
        cx.notify();
    }
}

impl Render for TextCasePanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let l10n = L10nState::global(cx).l10n;

        div()
            .size_full()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.))
                    .child(AppInput::field(self.input.clone()))
                    .child(
                        ActionButton::new(l10n.text_case_convert, "text-case-convert")
                            .on_click(cx.listener(|this, _, _, cx| this.convert_case(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.action_clear, "text-case-clear")
                            .secondary()
                            .on_click(cx.listener(|this, _, _, cx| this.clear(cx))),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .flex_1()
                    .id("text-case-result")
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .gap(px(6.))
                    .children(self.results.iter().map(|r| {
                        ResultBlock::new(format!("text-case-{}", r.label), r.value.clone())
                            .label(r.label)
                    })),
            )
    }
}
