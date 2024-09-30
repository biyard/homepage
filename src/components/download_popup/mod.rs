#![allow(non_snake_case)]
use crate::{
    apis::users::{keep_updates, KeepUpdatesRequest},
    components::filled_button::FilledButton,
};
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[component]
pub fn DownloadPopup() -> Element {
    let mut email = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "grid grid-rows-5 w-[370px] h-[500px] drop-shadow-[0px_0px_20px_rgba(0,0,0,0.4)] rounded-[20px] overflow-hidden",
            div {
                style: format!("background-image: url('{}')",asset!("./assets/images/popup.png")),
                class: "row-span-2 bg-[#21344C] flex justify-center items-center bg-center bg-no-repeat",
            }
            div {
                class: "row-span-3 bg-white px-[20px] py-[30px] flex flex-col justify-center items-center gap-[30px]",
                div {
                    class: "flex flex-col gap-[10px]",
                    div {
                        class: "text-[32px] text-center leading-[35px] font-bold text-[#21344C]",
                        "Select below to download!"
                    }
                    div {
                        class: "text-[18px] text-center font-regular text-[#21344C]",
                        "Leave your email to keep updated"
                    }
                }
                div {
                    class: "flex flex-col w-full gap-[10px]",
                    input {
                        class: "w-full h-[52px] bg-transparent rounded-[10px] px-[24px] py-[14px] text-[16px] leading-[24px] font-regular border-[1px] border-[#21344C] focus:outline-none focus:border-[#03AB79] transition-all duration-300 ease-in-out text-[#21344C]",
                        placeholder: "Email (optional)",
                        onchange: move |e| email.set(e.value()),

                    }
                    div {
                        class: "flex flex-row justify-between items-center",
                        FilledButton {
                            background_color: "bg-[#21344C]",
                            text_color: "text-white",
                            onclick: move |_| {
                                spawn(async move {
                                    let email = email();
                                    match keep_updates(KeepUpdatesRequest { email }).await {
                                        Ok(_) => {},
                                        Err(_) => {
                                            tracing::error!("Failed to subscribe!");
                                        }
                                    };
                                });
                            },
                            "BROCHURE"
                        }
                        FilledButton {
                            background_color: "bg-[#21344C]",
                            text_color: "text-white",
                            onclick: move |_| {
                                spawn(async move {
                                    let email = email();
                                    match keep_updates(KeepUpdatesRequest { email }).await {
                                        Ok(_) => {},
                                        Err(_) => {
                                            tracing::error!("Failed to subscribe!");
                                        }
                                    };
                                });
                            },
                            "COMPANY DECK"
                        }
                    }
                }
            }
        }
    }
}
