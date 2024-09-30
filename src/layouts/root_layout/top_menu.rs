#![allow(non_snake_case)]
// use std::{thread::sleep, time::Duration};

use dioxus::prelude::*;

use crate::{
    components::download_popup::DownloadPopup, prelude::*, services::popup_service::PopupService,
};

#[component]
pub fn TopMenu() -> Element {
    let mut popup = PopupService::use_popup_service();

    rsx! {
        div {
            class:"w-full backdrop-blur-[20px] bg-[#21344D]/50 p-[10px] fixed top-0 left-0 z-[100]",
            div {
                class: "max-w-[1440px] flex flex-row justify-between items-center m-auto",
                div {
                    class: "w-[95px]",
                    icons::logo {}
                }
                div {
                    class: "flex flex-row items-center gap-[20px]",
                    IconButton {
                        onclick: move |_| {
                            popup.open(DownloadPopup());
                        },
                        icons::download {}
                    }
                    // OutlinedButton {
                    //     onclick: move |_| sleep(Duration::from_secs(2)),
                    //     "SIGN IN"
                    // }
                }
            }
        }
    }
}
