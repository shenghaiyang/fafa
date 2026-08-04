use argon2::password_hash::phc::Salt;
use argon2::{Argon2, PasswordHasher};
use bcrypt::{DEFAULT_COST, hash};
use gpui::{Context, Entity, ReadGlobal, Render, SharedString, Window, div, prelude::*, px};

use crate::locale::L10nState;
use crate::ui::component::action_button::ActionButton;
use crate::ui::component::app_input::AppInput;
use crate::ui::component::error_block::ErrorBlock;
use crate::ui::component::option_label::option_label;
use crate::ui::component::radio::RadioButton;
use crate::ui::component::result_block::ResultBlock;
use crate::ui::component::text_input::{InputEvent, TextInput};

#[derive(Clone, Copy, PartialEq, Eq)]
enum HashAlgorithm {
    Argon2,
    Bcrypt,
}

pub struct PasswordHashPanel {
    pwd_input: Entity<TextInput>,
    algorithm: HashAlgorithm,
    pwd_hash_result: Option<String>,
    pwd_error: Option<String>,
}

impl PasswordHashPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let l10n = L10nState::global(cx).l10n;
        let pwd_input = cx.new(|cx| TextInput::new(l10n.hash_password_input, cx));
        cx.subscribe(&pwd_input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.generate_hash(cx),
            InputEvent::Changed => {}
        })
        .detach();

        Self {
            pwd_input,
            algorithm: HashAlgorithm::Argon2,
            pwd_hash_result: None,
            pwd_error: None,
        }
    }

    fn generate_hash(&mut self, cx: &mut Context<Self>) {
        let password = self.pwd_input.read(cx).value().to_string();
        if password.is_empty() {
            return;
        }

        match self.algorithm {
            HashAlgorithm::Argon2 => {
                let salt = Salt::generate();
                match Argon2::default().hash_password_with_salt(password.as_bytes(), &salt) {
                    Ok(hash) => {
                        self.pwd_hash_result = Some(hash.to_string());
                        self.pwd_error = None;
                    }
                    Err(e) => {
                        let l10n = L10nState::global(cx).l10n;
                        self.pwd_error = Some(format!("{}: {}", l10n.hash_password_error, e));
                        self.pwd_hash_result = None;
                    }
                }
            }
            HashAlgorithm::Bcrypt => match hash(password, DEFAULT_COST) {
                Ok(h) => {
                    self.pwd_hash_result = Some(h);
                    self.pwd_error = None;
                }
                Err(e) => {
                    let l10n = L10nState::global(cx).l10n;
                    self.pwd_error = Some(format!("{}: {}", l10n.hash_password_error, e));
                    self.pwd_hash_result = None;
                }
            },
        }
        cx.notify();
    }

    fn select_algorithm(&mut self, algorithm: HashAlgorithm, cx: &mut Context<Self>) {
        self.algorithm = algorithm;
        self.pwd_hash_result = None;
        self.pwd_error = None;
        cx.notify();
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.pwd_input
            .update(cx, |input, cx| input.set_value("", cx));
        self.pwd_hash_result = None;
        self.pwd_error = None;
        cx.notify();
    }
}

impl Render for PasswordHashPanel {
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
                    .child(option_label(l10n.hash_password_algorithm))
                    .child(
                        RadioButton::new("pwd-algo-argon2", l10n.hash_password_argon2)
                            .selected(self.algorithm == HashAlgorithm::Argon2)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.select_algorithm(HashAlgorithm::Argon2, cx)
                            })),
                    )
                    .child(
                        RadioButton::new("pwd-algo-bcrypt", l10n.hash_password_bcrypt)
                            .selected(self.algorithm == HashAlgorithm::Bcrypt)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.select_algorithm(HashAlgorithm::Bcrypt, cx)
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.))
                    .child(AppInput::field(self.pwd_input.clone()))
                    .child(
                        ActionButton::new(l10n.hash_password_hash, "pwd-hash")
                            .compact()
                            .on_click(cx.listener(|this, _, _, cx| this.generate_hash(cx))),
                    )
                    .child(
                        ActionButton::new(l10n.action_clear, "pwd-clear")
                            .secondary()
                            .on_click(cx.listener(|this, _, _, cx| this.clear(cx))),
                    ),
            )
            .when_some(self.pwd_hash_result.as_ref(), |this, hash| {
                this.child(
                    ResultBlock::new("pwd-hash-result", SharedString::from(hash.clone()))
                        .label(l10n.hash_password_result),
                )
            })
            .when_some(self.pwd_error.as_ref(), |this, err| {
                this.child(ErrorBlock::new(SharedString::from(err.clone())))
            })
    }
}
