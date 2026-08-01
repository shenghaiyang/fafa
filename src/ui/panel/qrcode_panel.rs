use gpui::{
    Bounds, Context, Entity, ReadGlobal, Render, Window, canvas, div, point, prelude::*, px, quad,
    rgb, size, transparent_black,
};
use qrcode::{EcLevel, QrCode};

use crate::locale::L10nState;
use crate::ui::component::action_button::ActionButton;
use crate::ui::component::app_input::AppInput;
use crate::ui::component::error_block::ErrorBlock;
use crate::ui::component::radio::RadioButton;
use crate::ui::component::text_input::{InputEvent, TextInput};
use crate::ui::theme::MAIN_THEME;

const QR_DARK: u32 = 0x1f2937;
const QR_DISPLAY_SIZE: f32 = 220.0;
const QUIET_ZONE: f32 = 4.0;

#[derive(Clone)]
struct QrGrid {
    width: usize,
    dark: Vec<bool>,
}

impl QrGrid {
    fn from_code(code: &QrCode) -> Self {
        let width = code.width();
        let dark = code
            .to_colors()
            .iter()
            .map(|c| *c == qrcode::types::Color::Dark)
            .collect();
        Self { width, dark }
    }
}

fn paint_qr(grid: &QrGrid, bounds: Bounds<gpui::Pixels>, window: &mut Window) {
    window.paint_quad(quad(
        bounds,
        px(0.),
        rgb(0xffffff),
        px(0.),
        transparent_black(),
        Default::default(),
    ));

    let total = grid.width as f32 + QUIET_ZONE * 2.0;
    let unit = (bounds.size.width.as_f32() / total).min(bounds.size.height.as_f32() / total);
    let origin_x = bounds.origin.x.as_f32() + (bounds.size.width.as_f32() - unit * total) / 2.0;
    let origin_y = bounds.origin.y.as_f32() + (bounds.size.height.as_f32() - unit * total) / 2.0;

    let dark = rgb(QR_DARK);
    for y in 0..grid.width {
        let row = y * grid.width;
        let mut x = 0;
        while x < grid.width {
            if !grid.dark[row + x] {
                x += 1;
                continue;
            }
            let start = x;
            while x < grid.width && grid.dark[row + x] {
                x += 1;
            }
            let left = origin_x + (start as f32 + QUIET_ZONE) * unit;
            let top = origin_y + (y as f32 + QUIET_ZONE) * unit;
            let run = (x - start) as f32 * unit;
            window.paint_quad(quad(
                Bounds {
                    origin: point(px(left), px(top)),
                    size: size(px(run), px(unit)),
                },
                px(0.),
                dark,
                px(0.),
                transparent_black(),
                Default::default(),
            ));
        }
    }
}

pub struct QrCodePanel {
    input: Entity<TextInput>,
    ec_level: EcLevel,
    grid: Option<QrGrid>,
    error: Option<String>,
}

impl QrCodePanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let l10n = L10nState::global(cx).l10n;
        let input = cx.new(|cx| TextInput::new(l10n.qrcode_input_placeholder, cx));
        cx.subscribe(&input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.regenerate(cx),
            InputEvent::Changed => {}
        })
        .detach();

        Self {
            input,
            ec_level: EcLevel::M,
            grid: None,
            error: None,
        }
    }

    fn select_ec(&mut self, level: EcLevel, cx: &mut Context<Self>) {
        self.ec_level = level;
        self.regenerate(cx);
    }

    fn regenerate(&mut self, cx: &mut Context<Self>) {
        let input = self.input.read(cx).value().to_string();
        if input.is_empty() {
            self.error = None;
            self.grid = None;
            cx.notify();
            return;
        }

        let l10n = L10nState::global(cx).l10n;
        match QrCode::with_error_correction_level(input.as_bytes(), self.ec_level) {
            Ok(code) => {
                self.grid = Some(QrGrid::from_code(&code));
                self.error = None;
            }
            Err(_) => {
                self.grid = None;
                self.error = Some(l10n.qrcode_error_too_long.to_string());
            }
        }
        cx.notify();
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.set_value("", cx));
        self.grid = None;
        self.error = None;
        cx.notify();
    }

    fn ec_label<'a>(&self, level: EcLevel, l10n: &'a crate::locale::L10n) -> &'a str {
        match level {
            EcLevel::L => l10n.qrcode_ec_l,
            EcLevel::M => l10n.qrcode_ec_m,
            EcLevel::Q => l10n.qrcode_ec_q,
            EcLevel::H => l10n.qrcode_ec_h,
        }
    }

    fn ec_id(level: EcLevel) -> &'static str {
        match level {
            EcLevel::L => "l",
            EcLevel::M => "m",
            EcLevel::Q => "q",
            EcLevel::H => "h",
        }
    }
}

impl Render for QrCodePanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let l10n = L10nState::global(cx).l10n;
        let theme = &MAIN_THEME;

        let levels = [EcLevel::L, EcLevel::M, EcLevel::Q, EcLevel::H];
        let _input_value = self.input.read(cx).value().to_string();

        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.))
                    .child(
                        AppInput::field(self.input.clone())
                            .with_clear(cx.listener(|this, _, _, cx| this.clear(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.qrcode_generate, "qrcode-generate")
                            .on_click(cx.listener(|this, _, _, cx| this.regenerate(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.action_clear, "qrcode-clear")
                            .secondary()
                            .on_click(cx.listener(|this, _, _, cx| this.clear(cx))),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.))
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(theme.text_dim)
                            .child(l10n.qrcode_ec_caption),
                    )
                    .children(levels.iter().map(|&level| {
                        RadioButton::new(
                            format!("ec-{}", Self::ec_id(level)),
                            self.ec_label(level, l10n),
                        )
                        .selected(self.ec_level == level)
                        .on_click(cx.listener(move |this, _, _, cx| this.select_ec(level, cx)))
                    })),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(10.))
                    .when_some(self.error.clone(), |this, err| {
                        this.child(ErrorBlock::new(err))
                    })
                    .when_some(self.grid.clone(), |this, grid| {
                        let _modules = grid.width;
                        this.child(
                            div().flex().flex_col().items_center().gap(px(10.)).child(
                                div()
                                    .p(px(14.))
                                    .rounded(px(10.))
                                    .border_1()
                                    .border_color(theme.border)
                                    .bg(theme.panel_bg)
                                    .child(
                                        canvas(
                                            |_, _, _| {},
                                            move |bounds, (), window, _| {
                                                paint_qr(&grid, bounds, window)
                                            },
                                        )
                                        .size(px(QR_DISPLAY_SIZE)),
                                    ),
                            ),
                        )
                    })
                    .when(self.grid.is_none() && self.error.is_none(), |this| {
                        this.child(
                            div()
                                .text_size(px(13.))
                                .text_color(theme.text_dim)
                                .child(l10n.qrcode_hint),
                        )
                    }),
            )
    }
}
