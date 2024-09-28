#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub(super) fn SloganSection() -> Element {
    let ctrl = super::controller::Controller::use_controller();
    let slogan = ctrl.slogan();
    let headlines = slogan.headlines();
    let description = slogan.description;

    rsx! {
        div {
            class: "my-[190px] mx-auto flex flex-col justify-center gap-[10px] p-[20px]",
            div {
                class: "text-[110px] font-bold text-white",
                "{headlines[0]}"
            }
            div {
                class: "pl-[240px] flex flex-col gap-[20px]",
                div {
                    class: "text-[110px] font-bold text-white",
                    "{headlines[1]}"
                }
                div {
                    class: "text-[18px] font-light text-white max-w-[625px]",
                    "{description}"
                }
            }
        }
    }
}
