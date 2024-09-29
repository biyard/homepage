#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::prelude::OutlinedButton;

#[component]
pub(super) fn UseCases() -> Element {
    rsx! {
        div {
            class: "w-full flex flex-col items-center justify-center gap-[50px] py-[100px]",
            div {
                class: "max-w-[1440px] 2xl:w-[1440px] m-auto grid grid-cols-3 flex flex-row py-[20px] px-[100px] justify-between items-center bg-[radial-gradient(96.21%_121.53%_at_97.68%_89.39%,#1C3E35_0%,#5B9883_100%)] rounded-[20px]",
                div {
                    class: "col-span-2 flex flex-row justify-start items-center gap-[10px]",
                }

                div {
                    class: "col-span-1 flex flex-col gap-[20px]",
                    div {
                        class: "text-[32px] leading-[40px] font-black text-white",
                        "Use Cases & Exhibitions"
                    }
                    div {
                        class: "text-[16px] leading-[24px] max-w-[445px] text-white",
                        "Experience full stack decentralization: from DAOs and crypto cloud services to games, NFTs, and social media, the Internet Computer has something for everyone."
                    }
                    OutlinedButton {
                        onclick: move |_| {},
                        "GET INTO IT"
                    }
                }
            }
        }
    }
}
