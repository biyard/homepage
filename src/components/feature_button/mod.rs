#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn FeatureButton(completed: Option<bool>, no_features: usize, text: String) -> Element {
    rsx! {
        div {
            class: "border border-[#027351]/30 text-[13px] font-regular text-white hover:bg-[#014D36] hover:text-[#03AB79] hover:border-white cursor-pointer transition-all duration-300 ease-in-out flex justify-center items-center active:bg-[#03AB79] active:text-white active:border-[#03AB79] rounded-[10px] px-[20px] py-[30px] w-[170px] h-[240px] flex-col",
            div {
                class: "flex flex-col justify-end items-end h-[160px]",
                if completed.unwrap_or(false) {
                    img {
                        src: asset!("./assets/images/done.png"),
                        class: "w-[25px] h-[25px]",
                    }
                }
                div {
                    class: "text-[120px] font-regular",
                    "{no_features}"
                }
            }
            div {
                class: "text-[13px] font-regular",
                "{text}"
            }
        }
    }
}
