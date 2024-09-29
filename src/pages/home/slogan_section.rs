#![allow(non_snake_case)]
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
                class: "text-[40px] text-center lg:text-[52px] lg:text-left 2xl:text-[110px] font-bold text-white",
                "{headlines[0]}"
            }
            div {
                class: "lg:pl-[120px] 2xl:pl-[240px] lg:text-left text-center 2xl:text-left flex flex-col gap-[20px]",
                div {
                    class: "text-[40px] lg:text-[52px] 2xl:text-[110px] font-bold text-white",
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
