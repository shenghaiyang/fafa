use crate::locale::L10nState;
use crate::ui::component::action_button::ActionButton;
use crate::ui::component::app_input::AppInput;
use crate::ui::component::error_block::ErrorBlock;
use crate::ui::component::option_label::option_label;
use crate::ui::component::radio::RadioButton;
use crate::ui::component::result_block::ResultBlock;
use crate::ui::component::text_input::{InputEvent, TextInput};
use crate::ui::theme::MAIN_THEME;
use gpui::{
    Context, Entity, FontWeight, ReadGlobal, Render, SharedString, Window, div, prelude::*, px,
};
use jiff::tz::{Offset, TimeZone};

const OFFSET_LIST: &[i32] = &[
    -12, -11, -10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14,
];

fn format_offset(hours: i32) -> String {
    if hours == 0 {
        "UTC".to_string()
    } else {
        format!("UTC{:+}", hours)
    }
}

const TZ_MENU_TOP: f32 = 52.0;

struct TimestampResult {
    tz_name: String,
    tz_time: String,
    utc_time: String,
}

struct DateTimeResult {
    seconds: i64,
    millis: i64,
}

enum ParseError {
    Format,
    Tz,
}

pub struct TimestampPanel {
    ts_input: Entity<TextInput>,
    dt_input: Entity<TextInput>,
    ts_result: Option<TimestampResult>,
    ts_error: Option<String>,
    dt_result: Option<DateTimeResult>,
    dt_error: Option<String>,
    ts_unit: bool,
    tz_offset: i32,
    tz_menu_open: bool,
}

impl TimestampPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let l10n = L10nState::global(cx).l10n;
        let ts_input = cx.new(|cx| TextInput::new(l10n.timestamp_placeholder, cx));
        let dt_input = cx.new(|cx| TextInput::new(l10n.timestamp_datetime_placeholder, cx));
        cx.subscribe(&ts_input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.ts_to_datetime(cx),
            InputEvent::Changed => {}
        })
        .detach();
        cx.subscribe(&dt_input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.dt_to_timestamp(cx),
            InputEvent::Changed => {}
        })
        .detach();

        Self {
            ts_input,
            dt_input,
            ts_result: None,
            ts_error: None,
            dt_result: None,
            dt_error: None,
            ts_unit: false,
            tz_offset: 8,
            tz_menu_open: false,
        }
    }

    fn tz(&self) -> TimeZone {
        TimeZone::fixed(
            Offset::from_seconds(self.tz_offset * 3600).expect("invalid timezone offset"),
        )
    }

    fn format_zoned(zdt: &jiff::Zoned) -> String {
        let base = zdt.strftime("%Y-%m-%d %H:%M:%S").to_string();
        let nano = zdt.timestamp().subsec_nanosecond();
        if nano == 0 {
            base
        } else {
            format!("{}.{:03}", base, nano / 1_000_000)
        }
    }

    fn ts_to_datetime(&mut self, cx: &mut Context<Self>) {
        let input = self.ts_input.read(cx).value().trim().to_string();
        self.ts_error = None;
        if input.is_empty() {
            self.ts_result = None;
            cx.notify();
            return;
        }

        let l10n = L10nState::global(cx).l10n;
        match input.parse::<i64>() {
            Ok(ts) => {
                let (secs, subsec) = if self.ts_unit {
                    (
                        ts.div_euclid(1000),
                        (ts.rem_euclid(1000) as i32) * 1_000_000,
                    )
                } else {
                    (ts, 0)
                };
                match jiff::Timestamp::new(secs, subsec) {
                    Ok(timestamp) => {
                        let tz = self.tz();
                        let tz_zdt = jiff::Zoned::new(timestamp, tz);
                        let utc_zdt = jiff::Zoned::new(timestamp, TimeZone::UTC);
                        self.ts_result = Some(TimestampResult {
                            tz_name: format_offset(self.tz_offset),
                            tz_time: Self::format_zoned(&tz_zdt),
                            utc_time: Self::format_zoned(&utc_zdt),
                        });
                    }
                    Err(_) => {
                        self.ts_result = None;
                        self.ts_error = Some(l10n.timestamp_error_timestamp.to_string());
                    }
                }
            }
            Err(_) => {
                self.ts_result = None;
                self.ts_error = Some(l10n.timestamp_error_timestamp.to_string());
            }
        }
        cx.notify();
    }

    fn dt_to_timestamp(&mut self, cx: &mut Context<Self>) {
        let input = self.dt_input.read(cx).value().trim().to_string();
        self.dt_error = None;
        if input.is_empty() {
            self.dt_result = None;
            cx.notify();
            return;
        }

        let l10n = L10nState::global(cx).l10n;
        let tz = self.tz();
        match parse_datetime(&input, &tz) {
            Ok((seconds, millis)) => {
                self.dt_result = Some(DateTimeResult { seconds, millis });
            }
            Err(ParseError::Format) => {
                self.dt_result = None;
                self.dt_error = Some(l10n.timestamp_error_datetime.to_string());
            }
            Err(ParseError::Tz) => {
                self.dt_result = None;
                self.dt_error = Some(l10n.timestamp_error_datetime_tz.to_string());
            }
        }
        cx.notify();
    }

    fn now(&mut self, cx: &mut Context<Self>) {
        let now = jiff::Timestamp::now();
        let value = if self.ts_unit {
            now.as_millisecond().to_string()
        } else {
            now.as_second().to_string()
        };
        self.ts_input
            .update(cx, |input, cx| input.set_value(value.as_str(), cx));
        self.ts_to_datetime(cx);
    }

    fn toggle_tz_menu(&mut self, cx: &mut Context<Self>) {
        self.tz_menu_open = !self.tz_menu_open;
        cx.notify();
    }

    fn close_tz_menu(&mut self, cx: &mut Context<Self>) {
        if self.tz_menu_open {
            self.tz_menu_open = false;
            cx.notify();
        }
    }

    fn select_offset(&mut self, offset: i32, cx: &mut Context<Self>) {
        self.tz_offset = offset;
        self.tz_menu_open = false;
        self.ts_error = None;
        self.dt_error = None;

        let has_ts = !self.ts_input.read(cx).value().trim().is_empty();
        let has_dt = !self.dt_input.read(cx).value().trim().is_empty();
        if has_ts {
            self.ts_to_datetime(cx);
        }
        if has_dt {
            self.dt_to_timestamp(cx);
        }
        if !has_ts && !has_dt {
            cx.notify();
        }
    }

    fn select_ts_unit(&mut self, ms: bool, cx: &mut Context<Self>) {
        self.ts_unit = ms;
        self.ts_error = None;
        let has_ts = !self.ts_input.read(cx).value().trim().is_empty();
        if has_ts {
            self.ts_to_datetime(cx);
        } else {
            self.ts_result = None;
            cx.notify();
        }
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.ts_input
            .update(cx, |input, cx| input.set_value("", cx));
        self.dt_input
            .update(cx, |input, cx| input.set_value("", cx));
        self.ts_result = None;
        self.ts_error = None;
        self.dt_result = None;
        self.dt_error = None;
        self.tz_menu_open = false;
        cx.notify();
    }
}

fn parse_datetime(input: &str, tz: &TimeZone) -> Result<(i64, i64), ParseError> {
    let (base, frac_ms) = match input.rsplit_once('.') {
        Some((b, f)) if !f.is_empty() && f.chars().all(|c| c.is_ascii_digit()) => {
            let mut padded = String::new();
            for ch in f.chars().take(3) {
                padded.push(ch);
            }
            while padded.len() < 3 {
                padded.push('0');
            }
            (b, padded.parse::<i64>().unwrap_or(0))
        }
        _ => (input, 0),
    };

    let dt = jiff::civil::DateTime::strptime("%Y-%m-%d %H:%M:%S", base)
        .map_err(|_| ParseError::Format)?;
    let zdt = dt.to_zoned(tz.clone()).map_err(|_| ParseError::Tz)?;
    let seconds = zdt.timestamp().as_second();
    Ok((seconds, seconds * 1000 + frac_ms))
}

fn section_label(text: &'static str) -> impl IntoElement {
    div()
        .text_size(px(12.))
        .text_color(MAIN_THEME.text_dim)
        .font_weight(FontWeight::SEMIBOLD)
        .child(text)
}

impl Render for TimestampPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let l10n = L10nState::global(cx).l10n;
        let theme = &MAIN_THEME;

        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .relative()
            .on_mouse_down_out(cx.listener(|this, _, _, cx| this.close_tz_menu(cx)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.))
                    .child(section_label(l10n.timestamp_timezone))
                    .child(
                        div()
                            .id("tz-trigger")
                            .h(px(28.))
                            .px(px(10.))
                            .rounded(px(6.))
                            .bg(if self.tz_menu_open {
                                theme.input_bg
                            } else {
                                theme.sidebar_bg
                            })
                            .border_1()
                            .border_color(if self.tz_menu_open {
                                theme.accent
                            } else {
                                theme.input_border
                            })
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.))
                            .cursor_pointer()
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .text_size(px(12.))
                                    .text_color(theme.text)
                                    .child(format_offset(self.tz_offset)),
                            )
                            .child(
                                div()
                                    .text_size(px(10.))
                                    .text_color(theme.text_dim)
                                    .child("▾"),
                            )
                            .on_click(cx.listener(|this, _, _, cx| this.toggle_tz_menu(cx))),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.))
                    .child(option_label(l10n.timestamp_unit_label))
                    .child(
                        RadioButton::new("ts-unit-sec", l10n.timestamp_unit_seconds)
                            .selected(!self.ts_unit)
                            .on_click(
                                cx.listener(move |this, _, _, cx| this.select_ts_unit(false, cx)),
                            ),
                    )
                    .child(
                        RadioButton::new("ts-unit-ms", l10n.timestamp_unit_milliseconds)
                            .selected(self.ts_unit)
                            .on_click(
                                cx.listener(move |this, _, _, cx| this.select_ts_unit(true, cx)),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(section_label(l10n.timestamp_label))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(8.))
                            .child(AppInput::field(self.ts_input.clone()))
                            .child(
                                ActionButton::new(l10n.timestamp_now, "ts-now")
                                    .compact()
                                    .secondary()
                                    .on_click(cx.listener(|this, _, _, cx| this.now(cx))),
                            )
                            .child(
                                ActionButton::new(l10n.timestamp_convert_to_datetime, "ts-to-dt")
                                    .compact()
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.ts_to_datetime(cx)),
                                    ),
                            )
                            .child(
                                ActionButton::new(l10n.action_clear, "ts-clear")
                                    .secondary()
                                    .on_click(cx.listener(|this, _, _, cx| this.clear(cx))),
                            ),
                    )
                    .when_some(self.ts_result.as_ref(), |this, r| {
                        this.child(
                            ResultBlock::new("ts-tz", r.tz_time.clone()).label(r.tz_name.clone()),
                        )
                        .child(ResultBlock::new("ts-utc", r.utc_time.clone()).label("UTC"))
                    })
                    .when_some(self.ts_error.as_ref(), |this, err| {
                        this.child(ErrorBlock::new(err.clone()))
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(section_label(l10n.timestamp_datetime_label))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(8.))
                            .child(AppInput::field(self.dt_input.clone()))
                            .child(
                                ActionButton::new(l10n.timestamp_convert_to_timestamp, "dt-to-ts")
                                    .compact()
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.dt_to_timestamp(cx)),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(theme.text_dim)
                            .child(l10n.timestamp_format),
                    )
                    .when_some(self.dt_result.as_ref(), |this, r| {
                        this.child(
                            ResultBlock::new("dt-sec", r.seconds.to_string())
                                .label(l10n.timestamp_unit_seconds),
                        )
                        .child(
                            ResultBlock::new("dt-ms", r.millis.to_string())
                                .label(l10n.timestamp_unit_milliseconds),
                        )
                    })
                    .when_some(self.dt_error.as_ref(), |this, err| {
                        this.child(ErrorBlock::new(err.clone()))
                    }),
            )
            .when(self.tz_menu_open, |this| {
                this.child(
                    div()
                        .id("tz-menu")
                        .absolute()
                        .top(px(TZ_MENU_TOP))
                        .left_0()
                        .w(px(260.))
                        .max_h(px(340.))
                        .overflow_y_scroll()
                        .py(px(4.))
                        .rounded(px(6.))
                        .border_1()
                        .border_color(theme.border)
                        .bg(theme.panel_bg)
                        .children(OFFSET_LIST.iter().map(|&offset| {
                            let is_active = self.tz_offset == offset;
                            div()
                                .id(SharedString::from(format!("tz-item-{}", offset)))
                                .h(px(26.))
                                .px(px(10.))
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(6.))
                                .text_size(px(12.))
                                .text_color(if is_active { theme.accent } else { theme.text })
                                .hover(|this| this.bg(theme.sidebar_hover))
                                .cursor_pointer()
                                .child(
                                    div()
                                        .w(px(14.))
                                        .text_size(px(11.))
                                        .text_color(theme.accent)
                                        .child(if is_active { "✓" } else { "" }),
                                )
                                .child(format_offset(offset))
                                .on_click(
                                    cx.listener(move |this, _, _, cx| {
                                        this.select_offset(offset, cx)
                                    }),
                                )
                        })),
                )
            })
    }
}
