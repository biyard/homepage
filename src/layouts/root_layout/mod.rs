#![allow(non_snake_case)]
use dioxus::prelude::*;
use footer::Footer;
use top_menu::TopMenu;

pub mod footer;
pub mod top_menu;

use crate::routes::Route;

#[component]
pub fn RootLayout() -> Element {
    rsx! {
        div {
            class: "w-full bg-[#21344D] text-white",
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
