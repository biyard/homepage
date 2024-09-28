#![allow(non_snake_case)]
#[allow(unused_imports)]
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub(super) fn Members() -> Element {
    let ctrl = super::controller::Controller::use_controller();

    rsx! {
        div { "Members section"}
    }
}
