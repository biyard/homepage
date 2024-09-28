#![allow(non_snake_case)]
#[allow(unused_imports)]
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub(super) fn Footer() -> Element {
    rsx! {
        div {
            class: "w-full h-[120px] flex flex-row justify-center items-center text-[16px] font-regular gap-[8px]",

            "© Biyard. All rights reserved."
            icons::email {}
            icons::web {}
            icons::linkedin {}
        }
    }
}
