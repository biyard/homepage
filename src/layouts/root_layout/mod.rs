#![allow(non_snake_case)]
use dioxus::prelude::*;
use footer::Footer;
use top_menu::TopMenu;

pub mod footer;
pub mod top_menu;

use crate::{routes::Route, services::popup_service::PopupService};

#[component]
pub fn RootLayout() -> Element {
    let mut popup = PopupService::use_popup_service();

    rsx! {
        div {
            class: "w-full bg-[#21344D] text-white",
            // PopupZone
            div {
                class: format!("{}", match popup.is_opened() {
                    true => "fixed top-0 left-0 w-screen h-screen bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-[30px] bg-[#21344C]/30 z-[101]",
                    false => "hidden"
                }),
                onclick: move |_| {
                    popup.close();
                },
                if popup.is_opened() {
                    {popup.render()}
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
