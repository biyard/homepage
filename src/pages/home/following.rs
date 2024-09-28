#![allow(non_snake_case)]
#[allow(unused_imports)]
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub(super) fn Following() -> Element {
    let ctrl = super::controller::Controller::use_controller();

    rsx! {
        div { "Following section"}
    }
}
