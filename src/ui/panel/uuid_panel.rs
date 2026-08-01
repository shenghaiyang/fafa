use gpui::{
    Context, Entity, FontWeight, ReadGlobal, Render, TextAlign, Window, div, prelude::*, px,
};

use crate::locale::L10nState;
use crate::ui::component::action_button::ActionButton;
use crate::ui::component::copy_button::CopyButton;
use crate::ui::component::option_label::option_label;
use crate::ui::component::radio::RadioButton;
use crate::ui::component::text_input::{InputEvent, TextInput};
use crate::ui::theme::{MAIN_THEME, MONO_FONT};

const MIN_COUNT: usize = 1;
const MAX_COUNT: usize = 100;

#[derive(Clone, Copy, PartialEq, Eq)]
enum UuidVersion {
    V4,
    V7,
}

pub struct UuidPanel {
    count_input: Entity<TextInput>,
    version: UuidVersion,
    dashed: bool,
    result: Option<Vec<String>>,
}

impl UuidPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let l10n = L10nState::global(cx).l10n;
        let count_input = cx.new(|cx| {
            let mut input = TextInput::new(l10n.uuid_count_placeholder, cx);
            input.set_value("1", cx);
            input
        });
        cx.subscribe(
            &count_input,
            |this, _, event: &InputEvent, cx| match event {
                InputEvent::Submit => this.generate(cx),
                InputEvent::Changed => {}
            },
        )
        .detach();

        Self {
            count_input,
            version: UuidVersion::V4,
            dashed: true,
            result: None,
        }
    }

    fn generate(&mut self, cx: &mut Context<Self>) {
        use uuid::Uuid;

        let raw = self.count_input.read(cx).value().trim().to_string();
        let count = raw
            .parse::<usize>()
            .unwrap_or(MIN_COUNT)
            .clamp(MIN_COUNT, MAX_COUNT);

        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            let uuid = match self.version {
                UuidVersion::V4 => Uuid::new_v4(),
                UuidVersion::V7 => Uuid::now_v7(),
            };
            out.push(if self.dashed {
                uuid.to_string()
            } else {
                uuid.simple().to_string()
            });
        }
        self.result = Some(out);
        cx.notify();
    }

    fn select_version(&mut self, version: UuidVersion, cx: &mut Context<Self>) {
        self.version = version;
        self.result = None;
        cx.notify();
    }

    fn select_style(&mut self, dashed: bool, cx: &mut Context<Self>) {
        self.dashed = dashed;
        self.result = None;
        cx.notify();
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.count_input
            .update(cx, |input, cx| input.set_value("1", cx));
        self.result = None;
        cx.notify();
    }
}

impl Render for UuidPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let l10n = L10nState::global(cx).l10n;
        let theme = &MAIN_THEME;

        div()
            .size_full()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.))
                    .child(option_label(l10n.uuid_version))
                    .child(
                        RadioButton::new("uuid-radio-v4", l10n.uuid_v4)
                            .selected(self.version == UuidVersion::V4)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.select_version(UuidVersion::V4, cx)
                            })),
                    )
                    .child(
                        RadioButton::new("uuid-radio-v7", l10n.uuid_v7)
                            .selected(self.version == UuidVersion::V7)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.select_version(UuidVersion::V7, cx)
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.))
                    .child(option_label(l10n.uuid_style))
                    .child(
                        RadioButton::new("uuid-radio-dashed", l10n.uuid_dashed)
                            .selected(self.dashed)
                            .on_click(
                                cx.listener(move |this, _, _, cx| this.select_style(true, cx)),
                            ),
                    )
                    .child(
                        RadioButton::new("uuid-radio-simple", l10n.uuid_simple)
                            .selected(!self.dashed)
                            .on_click(
                                cx.listener(move |this, _, _, cx| this.select_style(false, cx)),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(option_label(l10n.uuid_count))
                    .child(
                        div()
                            .w(px(64.))
                            .h(px(32.))
                            .rounded(px(6.))
                            .bg(theme.input_bg)
                            .border_1()
                            .border_color(theme.input_border)
                            .overflow_hidden()
                            .child(self.count_input.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(theme.text_dim)
                            .child(l10n.uuid_count_hint),
                    )
                    .child(
                        ActionButton::new(l10n.uuid_new, "uuid-new")
                            .on_click(cx.listener(|this, _, _, cx| this.generate(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.action_clear, "uuid-clear")
                            .secondary()
                            .on_click(cx.listener(|this, _, _, cx| this.clear(cx))),
                    ),
            )
            .when_some(self.result.as_ref(), |this, result| {
                let joined = result.join("\n");
                let line_numbers = (1..=result.len())
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                this.child(
                    div()
                        .w_full()
                        .flex_1()
                        .min_h_0()
                        .flex()
                        .flex_col()
                        .gap(px(6.))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(10.))
                                .child(
                                    div()
                                        .text_size(px(11.))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_dim)
                                        .child(format!("{} {}", result.len(), l10n.uuid_unit)),
                                )
                                .child(CopyButton::new("uuid-copy-all", joined.clone())),
                        )
                        .child(
                            div()
                                .id("uuid-result-box")
                                .w_full()
                                .flex_1()
                                .min_h_0()
                                .flex()
                                .flex_row()
                                .items_start()
                                .overflow_y_scroll()
                                .rounded(px(4.))
                                .border_1()
                                .border_color(theme.input_border)
                                .bg(theme.panel_bg)
                                .child(
                                    div()
                                        .w(px(38.))
                                        .min_h_auto()
                                        .flex_none()
                                        .py(px(6.))
                                        .pr(px(8.))
                                        .bg(theme.result_bg)
                                        .text_align(TextAlign::Right)
                                        .text_size(px(14.))
                                        .text_color(theme.text_dim)
                                        .font_family(MONO_FONT)
                                        .child(line_numbers.clone()),
                                )
                                .child(div().w(px(1.)).bg(theme.border))
                                .child(
                                    div()
                                        .flex_1()
                                        .min_h_auto()
                                        .font_family(MONO_FONT)
                                        .min_w_0()
                                        .py(px(6.))
                                        .px(px(10.))
                                        .text_size(px(14.))
                                        .text_color(theme.result_text)
                                        .child(joined.clone()),
                                ),
                        ),
                )
            })
    }
}
