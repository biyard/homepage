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
            a {
                href: "mailto:hi@biyard.co",
                icons::email {}
            }
            a {
                href: "https://www.linkedin.com/company/75498162",
                target: "_blank",
                icons::linkedin {}
            }
        }
    }
}
