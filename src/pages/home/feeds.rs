#![allow(non_snake_case)]
use crate::prelude::*;
#[allow(unused_imports)]
use dioxus::prelude::*;

#[component]
pub(super) fn Feeds() -> Element {
    let ctrl = super::controller::Controller::use_controller();

    rsx! {
        div { "Feeds section"}
    }
}
