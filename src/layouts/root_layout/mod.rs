#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use footer::Footer;
use top_menu::TopMenu;

pub mod footer;
pub mod top_menu;

use crate::{
    components::download_popup::DownloadPopup, routes::Route, services::popup_service::PopupService,
};

#[component]
pub fn RootLayout() -> Element {
    let mut popup = PopupService::use_popup_service();

    rsx! {
        div {
            class: "w-full bg-[#21344D] text-white",
            div {
                class: format!("{}", match (popup.data)() {
                    Some(_) => "fixed top-0 left-0 w-screen h-screen bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-[30px] bg-[#21344C]/30 z-[101]",
                    _ => "fixed top-0 left-0 w-[0px] h-[0px] hidden overflow-hidden",
                }),

                DownloadPopup { }
                div {
                    class: "fixed top-0 left-0 w-full h-full z-[102]",
                    onclick: move |_| {
                        tracing::debug!("close popup");
                        popup.close();
                    },
                }
            }
            TopMenu { }
            div {
                class: "flex flex-col w-full",
                div {
                    Outlet::<Route> {}
                }
                Footer {}
            }
        }
    }
}
