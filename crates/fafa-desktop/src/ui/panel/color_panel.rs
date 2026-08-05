use gpui::{
    Context, Entity, FontWeight, Hsla, ReadGlobal, Render, SharedString, Window, div, prelude::*,
    px,
};

use crate::locale::{L10n, L10nState};
use crate::ui::component::action_button::ActionButton;
use crate::ui::component::app_input::AppInput;
use crate::ui::component::copy_button::CopyButton;
use crate::ui::component::error_block::ErrorBlock;
use crate::ui::component::text_input::{InputEvent, TextInput};
use crate::ui::theme::{MAIN_THEME, MONO_FONT};

const FORMATS: [ColorFormat; 3] = [ColorFormat::Hex, ColorFormat::Rgb, ColorFormat::Hsl];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ColorFormat {
    Hex,
    Rgb,
    Hsl,
}

impl ColorFormat {
    fn label(self, l10n: &L10n) -> &'static str {
        match self {
            ColorFormat::Hex => l10n.color_hex,
            ColorFormat::Rgb => l10n.color_rgb,
            ColorFormat::Hsl => l10n.color_hsl,
        }
    }

    fn parse(self, s: &str) -> Option<ColorRgba> {
        match self {
            ColorFormat::Hex => parse_hex(s),
            ColorFormat::Rgb => parse_rgb(s),
            ColorFormat::Hsl => parse_hsl(s),
        }
    }
}

#[derive(Clone)]
struct ColorRgba {
    r: u8,
    g: u8,
    b: u8,
    a: f32,
}

impl ColorRgba {
    fn alpha_byte(&self) -> u8 {
        (self.a * 255.0).round() as u8
    }

    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    fn to_rgba(&self) -> String {
        format!("rgba({}, {}, {}, {:.2})", self.r, self.g, self.b, self.a)
    }

    fn to_rgba_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.r,
            self.g,
            self.b,
            self.alpha_byte()
        )
    }

    fn to_argb_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.alpha_byte(),
            self.r,
            self.g,
            self.b
        )
    }

    fn to_hsla(&self) -> (f64, f64, f64, f32) {
        let r = self.r as f64 / 255.0;
        let g = self.g as f64 / 255.0;
        let b = self.b as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h;
        let s;
        let l = (max + min) / 2.0;

        if delta.abs() < f64::EPSILON {
            h = 0.0;
            s = 0.0;
        } else {
            s = delta / (1.0 - (2.0 * l - 1.0).abs());
            if max == r {
                h = 60.0 * (((g - b) / delta) % 6.0);
            } else if max == g {
                h = 60.0 * (((b - r) / delta) + 2.0);
            } else {
                h = 60.0 * (((r - g) / delta) + 4.0);
            }
        }
        let h = if h < 0.0 { h + 360.0 } else { h };

        (h.round(), (s * 100.0).round(), (l * 100.0).round(), self.a)
    }

    fn to_hsla_f32(&self) -> (f32, f32, f32, f32) {
        let (h, s, l, a) = self.to_hsla();
        (
            (h / 360.0) as f32,
            (s / 100.0) as f32,
            (l / 100.0) as f32,
            a,
        )
    }
}

fn parse_hex(s: &str) -> Option<ColorRgba> {
    let s = s.trim_start_matches('#');
    if s.len() != 6 && s.len() != 8 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    let a = if s.len() == 8 {
        u8::from_str_radix(&s[6..8], 16).ok()? as f32 / 255.0
    } else {
        1.0
    };
    Some(ColorRgba { r, g, b, a })
}

fn parse_rgb(s: &str) -> Option<ColorRgba> {
    let s = s.trim();
    let inner = s
        .strip_prefix("rgba(")
        .or_else(|| s.strip_prefix("RGBA("))
        .or_else(|| s.strip_prefix("rgb("))
        .or_else(|| s.strip_prefix("RGB("))?;
    let inner = inner.strip_suffix(')')?;
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 3 && parts.len() != 4 {
        return None;
    }
    let r = parts[0].trim().parse::<u8>().ok()?;
    let g = parts[1].trim().parse::<u8>().ok()?;
    let b = parts[2].trim().parse::<u8>().ok()?;
    let a = if let Some(raw) = parts.get(3) {
        let v = raw.trim().parse::<f32>().ok()?;
        if v <= 1.0 { v } else { v / 255.0 }
    } else {
        1.0
    };
    Some(ColorRgba { r, g, b, a })
}

fn parse_hsl(s: &str) -> Option<ColorRgba> {
    let s = s.trim();
    let inner = s
        .strip_prefix("hsla(")
        .or_else(|| s.strip_prefix("HSLA("))
        .or_else(|| s.strip_prefix("hsl("))
        .or_else(|| s.strip_prefix("HSL("))?;
    let inner = inner.strip_suffix(')')?;
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 3 && parts.len() != 4 {
        return None;
    }
    let h = parts[0].trim().parse::<f64>().ok()?;
    let s_val = parts[1].trim().trim_end_matches('%').parse::<f64>().ok()?;
    let l = parts[2].trim().trim_end_matches('%').parse::<f64>().ok()?;
    let a = if let Some(raw) = parts.get(3) {
        raw.trim().parse::<f32>().ok()?.clamp(0.0, 1.0)
    } else {
        1.0
    };

    let h = h % 360.0;
    if h < 0.0 {
        return None;
    }
    let s = s_val / 100.0;
    let l = l / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r1, g1, b1) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Some(ColorRgba {
        r: ((r1 + m) * 255.0).round() as u8,
        g: ((g1 + m) * 255.0).round() as u8,
        b: ((b1 + m) * 255.0).round() as u8,
        a,
    })
}

struct ColorResults {
    hex: String,
    rgba_fields: [String; 4],
    rgba: String,
    rgba_hex: String,
    argb_hex: String,
    hsla: String,
}

pub struct ColorPanel {
    input: Entity<TextInput>,
    format: ColorFormat,
    result: Option<ColorResults>,
    error: Option<String>,
    preview_color: Option<ColorRgba>,
}

fn result_value_row(
    id: &'static str,
    label: &'static str,
    value: &str,
    chip: Option<Hsla>,
) -> impl IntoElement {
    let theme = &MAIN_THEME;
    div()
        .flex()
        .flex_col()
        .gap(px(4.))
        .child(
            div()
                .text_size(px(11.))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_dim)
                .child(label),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .h(px(32.))
                        .px(px(10.))
                        .rounded(px(4.))
                        .bg(theme.result_bg)
                        .border_1()
                        .border_color(theme.input_border)
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.))
                        .when_some(chip, |this, c| {
                            this.child(
                                div()
                                    .flex_none()
                                    .w(px(16.))
                                    .h(px(16.))
                                    .rounded(px(3.))
                                    .border_1()
                                    .border_color(theme.border)
                                    .bg(c),
                            )
                        })
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .font_family(MONO_FONT)
                                .text_size(px(12.))
                                .text_color(theme.result_text)
                                .child(value.to_string()),
                        ),
                )
                .when(!value.is_empty(), |this| {
                    this.child(CopyButton::new(format!("{}-copy", id), value.to_string()))
                }),
        )
}

fn result_rgba_row(
    id: &'static str,
    label: &'static str,
    values: [String; 4],
    copy_value: &str,
) -> impl IntoElement {
    let theme = &MAIN_THEME;
    let names = ["R", "G", "B", "A"];
    div()
        .flex()
        .flex_col()
        .gap(px(4.))
        .child(
            div()
                .text_size(px(11.))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_dim)
                .child(label),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_end()
                .gap(px(6.))
                .children((0..4).map(|i| {
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(2.))
                        .child(
                            div()
                                .text_size(px(10.))
                                .text_color(theme.text_dim)
                                .child(names[i]),
                        )
                        .child(
                            div()
                                .h(px(32.))
                                .px(px(8.))
                                .rounded(px(4.))
                                .bg(theme.result_bg)
                                .border_1()
                                .border_color(theme.input_border)
                                .flex()
                                .items_center()
                                .font_family(MONO_FONT)
                                .text_size(px(12.))
                                .text_color(theme.result_text)
                                .child(values[i].clone()),
                        )
                }))
                .when(!copy_value.is_empty(), |this| {
                    this.child(CopyButton::new(
                        format!("{}-copy", id),
                        copy_value.to_string(),
                    ))
                }),
        )
}

impl ColorPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let l10n = L10nState::global(cx).l10n;
        let input = cx.new(|cx| TextInput::new(l10n.color_input_placeholder, cx));
        cx.subscribe(&input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.convert_color(cx),
            InputEvent::Changed => this.update_preview(cx),
        })
        .detach();

        Self {
            input,
            format: ColorFormat::Hex,
            result: None,
            error: None,
            preview_color: None,
        }
    }

    fn update_preview(&mut self, cx: &mut Context<Self>) {
        let input = self.input.read(cx).value().trim().to_string();
        self.preview_color = if input.is_empty() {
            None
        } else {
            self.format.parse(&input)
        };
        self.error = None;
        cx.notify();
    }

    fn convert_color(&mut self, cx: &mut Context<Self>) {
        let input = self.input.read(cx).value().trim().to_string();
        if input.is_empty() {
            return;
        }

        match self.format.parse(&input) {
            Some(c) => {
                let (h, s, l, a) = c.to_hsla();
                self.result = Some(ColorResults {
                    hex: c.to_hex(),
                    rgba_fields: [
                        c.r.to_string(),
                        c.g.to_string(),
                        c.b.to_string(),
                        format!("{:.2}", c.a),
                    ],
                    rgba: c.to_rgba(),
                    rgba_hex: c.to_rgba_hex(),
                    argb_hex: c.to_argb_hex(),
                    hsla: format!("hsla({:.0}, {:.0}%, {:.0}%, {:.2})", h, s, l, a),
                });
                self.error = None;
                self.preview_color = Some(c);
            }
            None => {
                let l10n = L10nState::global(cx).l10n;
                self.error = Some(l10n.color_error.to_string());
                self.result = None;
                self.preview_color = None;
            }
        }
        cx.notify();
    }

    fn select_format(&mut self, format: ColorFormat, cx: &mut Context<Self>) {
        self.format = format;
        self.result = None;
        self.error = None;
        self.update_preview(cx);
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.set_value("", cx));
        self.result = None;
        self.error = None;
        self.preview_color = None;
        cx.notify();
    }
}

impl Render for ColorPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let l10n = L10nState::global(cx).l10n;
        let theme = &MAIN_THEME;
        let res = self.result.as_ref();
        let hex = res.map(|r| r.hex.as_str()).unwrap_or("");
        let rgba = res.map(|r| r.rgba.as_str()).unwrap_or("");
        let rgba_hex = res.map(|r| r.rgba_hex.as_str()).unwrap_or("");
        let argb_hex = res.map(|r| r.argb_hex.as_str()).unwrap_or("");
        let hsla = res.map(|r| r.hsla.as_str()).unwrap_or("");
        let rgba_fields = res
            .map(|r| r.rgba_fields.clone())
            .unwrap_or_else(|| [String::new(), String::new(), String::new(), String::new()]);
        let chip = self.preview_color.as_ref().map(|c| {
            let (h, s, l, a) = c.to_hsla_f32();
            gpui::hsla(h, s, l, a)
        });

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
                    .gap(px(8.))
                    .children(FORMATS.iter().map(|&format| {
                        let is_active = self.format == format;
                        div()
                            .id(SharedString::from(format!("color-fmt-{:?}", format)))
                            .flex_none()
                            .h(px(28.))
                            .px(px(10.))
                            .rounded(px(6.))
                            .bg(if is_active {
                                theme.button_bg
                            } else {
                                theme.sidebar_bg
                            })
                            .text_color(if is_active {
                                theme.button_text
                            } else {
                                theme.sidebar_text
                            })
                            .text_size(px(11.))
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .child(format.label(l10n))
                            .on_click(
                                cx.listener(move |this, _, _, cx| this.select_format(format, cx)),
                            )
                    }))
                    .child(AppInput::field(self.input.clone()))
                    .child(
                        div()
                            .flex_none()
                            .w(px(32.))
                            .h(px(32.))
                            .rounded(px(6.))
                            .border_1()
                            .border_color(theme.input_border)
                            .bg(if let Some(c) = self.preview_color.as_ref() {
                                let (h, s, l, a) = c.to_hsla_f32();
                                gpui::hsla(h, s, l, a)
                            } else {
                                theme.result_bg
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.))
                    .child(
                        ActionButton::new(l10n.color_convert, "color-convert")
                            .on_click(cx.listener(|this, _, _, cx| this.convert_color(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.action_clear, "color-clear")
                            .secondary()
                            .on_click(cx.listener(|this, _, _, cx| this.clear(cx))),
                    ),
            )
            .when_some(self.error.as_ref(), |this, err| {
                this.child(ErrorBlock::new(SharedString::from(err.clone())))
            })
            .child(
                div()
                    .w_full()
                    .flex_1()
                    .id("color-result")
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .gap(px(10.))
                    .child(result_value_row("color-hex", l10n.color_hex, hex, chip))
                    .child(result_rgba_row(
                        "color-rgba",
                        l10n.color_rgba,
                        rgba_fields,
                        rgba,
                    ))
                    .child(result_value_row(
                        "color-rgba-hex",
                        l10n.color_rgba_hex,
                        rgba_hex,
                        chip,
                    ))
                    .child(result_value_row(
                        "color-argb-hex",
                        l10n.color_argb_hex,
                        argb_hex,
                        chip,
                    ))
                    .child(result_value_row("color-hsla", l10n.color_hsla, hsla, None)),
            )
    }
}
