use std::time::Duration;

use gpui::{App, AsyncApp, Global, SharedString};

const TOAST_DURATION: Duration = Duration::from_millis(1600);

#[derive(Default)]
pub struct ToastState {
    pub message: Option<SharedString>,
    token: u64,
}

impl Global for ToastState {}

impl ToastState {
    pub fn init(cx: &mut App) {
        cx.set_global(Self::default());
    }
}

pub fn show_toast(cx: &mut App, message: impl Into<SharedString>) {
    let message = message.into();
    let token = {
        let state = cx.global_mut::<ToastState>();
        state.token = state.token.wrapping_add(1);
        state.message = Some(message);
        state.token
    };

    let executor = cx.background_executor().clone();
    cx.spawn(async move |async_cx: &mut AsyncApp| {
        executor.timer(TOAST_DURATION).await;
        async_cx.update(|cx| {
            let state = cx.global_mut::<ToastState>();
            if state.token == token {
                state.message = None;
            }
        });
    })
    .detach();
}
