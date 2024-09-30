#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::prelude::OutlinedButton;

#[component]
pub(super) fn UseCases() -> Element {
    rsx! {
        div {
            class: "w-full flex flex-col items-center justify-center gap-[50px] py-[100px] px-[20px]",
            div {
                class: "w-full 2xl:w-[1440px] m-auto grid grid-cols-3 flex flex-row py-[20px] px-[100px] justify-between items-center bg-[radial-gradient(96.21%_121.53%_at_97.68%_89.39%,#2a1454_0%,#8750f7_100%)] rounded-[20px] gap-[20px]",
                div {
                    class: "col-span-0 h-full md:col-span-1 flex flex-row justify-start items-center gap-[10px] bg-cover bg-no-repeat",
                    img {
                        src: "https://blockchain-4all.com/assets/img/hero/me.png",
                        class: "border-[1px] border-[#5b2565] rotate-[4.27deg] rounded-[38px] max-[768px]:hidden",
                    }
                }

                div {
                    class: "col-span-3 md:col-span-2 flex flex-col gap-[20px]",
                    div {
                        class: "text-[32px] leading-[40px] font-black text-white",
                        "BLOCKCHAIN FOR ALL"
                    }
                    div {
                        class: "text-[16px] leading-[24px] max-w-[445px] text-white",
                        "Blockchain Convergence: The Next Big Thing, 3-4 OCTOBER 2024, Casa Llotja de Mar, Barcelona"
                    }
                    OutlinedButton {
                        onclick: move |_| {
                            #[cfg(feature = "web")]
                            let _ = web_sys::window().unwrap().open_with_url_and_target("https://blockchain-4all.com/", "_blank");

                        },
                        "GO TO B4A"
                    }
                }
            }
        }
    }
}
