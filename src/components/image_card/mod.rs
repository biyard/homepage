#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn ImageCard(category: String, title: String, description: String, image: String) -> Element {
    rsx! {
        div {
            class: "w-full grid grid-cols-3 flex flex-row h-165 bg-[#08203E] rounded-[8px] items-center overflow-hidden",
            div {
                class: "col-span-2 flex flex-col justify-center p-[20px]",
                div {
                    class: "text-[16px] fond-regular",
                    "{category}"
                }
                div {
                    class: "text-[24px] fond-bold",
                    "{title}"
                }
                div {
                    class: "text-[16px] fond-regular",
                    "{description}"
                }
            }
            div {
                class: "col-span-1 h-full",
                img {
                    src: image,
                    class: "w-full h-full object-cover"
                }
            }
        }
    }
}
