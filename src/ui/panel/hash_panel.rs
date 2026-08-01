use std::collections::HashSet;

use gpui::{ClickEvent, Context, Entity, ReadGlobal, Render, Window, div, prelude::*, px};
use md5::{Digest, Md5};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};

use crate::locale::L10nState;
use crate::ui::component::action_button::ActionButton;
use crate::ui::component::app_input::AppInput;
use crate::ui::component::checkbox::Checkbox;
use crate::ui::component::result_block::ResultBlock;
use crate::ui::component::text_input::{InputEvent, TextInput};
use crate::ui::theme::MAIN_THEME;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum HashAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Sha3_256,
    Sha3_512,
    Blake3,
}

impl HashAlgorithm {
    fn all() -> [Self; 7] {
        [
            Self::Md5,
            Self::Sha1,
            Self::Sha256,
            Self::Sha512,
            Self::Sha3_256,
            Self::Sha3_512,
            Self::Blake3,
        ]
    }

    fn locale_key<'a>(&self, l10n: &'a crate::locale::L10n) -> &'a str {
        match self {
            Self::Md5 => l10n.hash_md5,
            Self::Sha1 => l10n.hash_sha1,
            Self::Sha256 => l10n.hash_sha256,
            Self::Sha512 => l10n.hash_sha512,
            Self::Sha3_256 => l10n.hash_sha3_256,
            Self::Sha3_512 => l10n.hash_sha3_512,
            Self::Blake3 => l10n.hash_blake3,
        }
    }

    fn compute(&self, input: &str) -> String {
        match self {
            Self::Md5 => hex::encode(Md5::digest(input.as_bytes())),
            Self::Sha1 => hex::encode(Sha1::digest(input.as_bytes())),
            Self::Sha256 => hex::encode(Sha256::digest(input.as_bytes())),
            Self::Sha512 => hex::encode(Sha512::digest(input.as_bytes())),
            Self::Sha3_256 => {
                use sha3::Digest;
                let mut hasher = Sha3_256::new();
                hasher.update(input.as_bytes());
                hex::encode(hasher.finalize())
            }
            Self::Sha3_512 => {
                use sha3::Digest;
                let mut hasher = Sha3_512::new();
                hasher.update(input.as_bytes());
                hex::encode(hasher.finalize())
            }
            Self::Blake3 => blake3::hash(input.as_bytes()).to_hex().to_string(),
        }
    }
}

pub struct HashPanel {
    input: Entity<TextInput>,
    selected: HashSet<HashAlgorithm>,
    input_text: String,
    results: Vec<(HashAlgorithm, String)>,
    has_computed: bool,
}

impl HashPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let l10n = L10nState::global(cx).l10n;
        let input = cx.new(|cx| TextInput::new(l10n.hash_input_placeholder, cx));
        cx.subscribe(&input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.compute_hash(cx),
            InputEvent::Changed => {}
        })
        .detach();

        Self {
            input,
            selected: HashAlgorithm::all().into_iter().collect(),
            input_text: String::new(),
            results: Vec::new(),
            has_computed: false,
        }
    }

    fn toggle_algorithm(&mut self, algo: HashAlgorithm, cx: &mut Context<Self>) {
        if self.selected.contains(&algo) {
            self.selected.remove(&algo);
        } else {
            self.selected.insert(algo);
        }
        cx.notify();
    }

    fn compute_hash(&mut self, cx: &mut Context<Self>) {
        let input = self.input.read(cx).value().to_string();
        if input.is_empty() {
            return;
        }
        self.input_text = input.clone();
        self.results = self
            .selected
            .iter()
            .map(|algo| (*algo, algo.compute(&input)))
            .collect();

        self.results.sort_by_key(|(algo, _)| *algo as u8);
        self.has_computed = true;
        cx.notify();
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.set_value("", cx));
        self.input_text.clear();
        self.results.clear();
        self.has_computed = false;
        cx.notify();
    }
}

impl Render for HashPanel {
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
                    .w_full()
                    .flex()
                    .flex_row()
                    .gap(px(8.))
                    .child(AppInput::field(self.input.clone()).with_clear(
                        cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| this.clear(cx)),
                    ))
                    .child(
                        ActionButton::new("Go", "hash-go")
                            .on_click(cx.listener(|this, _, _, cx| this.compute_hash(cx))),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .gap(px(16.))
                    .flex_wrap()
                    .children(HashAlgorithm::all().iter().map(|&algo| {
                        let is_selected = self.selected.contains(&algo);
                        let label = algo.locale_key(l10n);
                        Checkbox::new(label, label).selected(is_selected).on_click(
                            cx.listener(move |this, _, _, cx| this.toggle_algorithm(algo, cx)),
                        )
                    })),
            )
            .child(
                div()
                    .w_full()
                    .mt(px(20.))
                    .text_size(px(16.))
                    .text_color(theme.label_result_text)
                    .child(l10n.label_result),
            )
            .when(self.has_computed && !self.input_text.is_empty(), |this| {
                this.child(
                    div()
                        .w_full()
                        .flex_1()
                        .min_h_0()
                        .id("hash-result")
                        .overflow_y_scroll()
                        .flex()
                        .flex_col()
                        .mt(px(12.))
                        .gap(px(8.))
                        .children(self.results.iter().map(|(algo, hash)| {
                            let algo_label = algo.locale_key(l10n);
                            ResultBlock::new(format!("hash-row-{}", algo_label), hash.clone())
                                .label(algo_label)
                        })),
                )
            })
    }
}
