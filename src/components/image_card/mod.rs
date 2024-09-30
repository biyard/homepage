#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn ImageCard(category: String, title: String, description: String, image: String) -> Element {
    rsx! {
        div {
            class: "w-full grid grid-cols-5 bg-[#08203E] rounded-[8px] items-center overflow-hidden",
            div {
                class: "col-span-3 flex flex-col justify-center px-[20px]",
                div {
                    class: "text-[16px] fond-regular",
                    "{category}"
                }
                div {
                    class: "text-[24px] fond-bold",
                    "{title}"
                }
                div {
                    class: "max-[1200px]:hidden text-[16px] fond-regular",
                    "{description}"
                }
            }
            div {
                class: "col-span-2 flex justify-end items-right bg-fit bg-no-repeat bg-right h-full w-full",
                img {
                    src: "{image}",
                }
            }
        }
    }
}
