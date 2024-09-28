#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub(super) fn UseCases() -> Element {
    let ctrl = super::controller::Controller::use_controller();
    rsx! {
        div { "UseCases page"}
    }
}
