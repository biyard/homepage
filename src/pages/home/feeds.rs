#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub(super) fn Feeds() -> Element {
    let ctrl = super::controller::Controller::use_controller();

    rsx! {
        div {
            class: "w-full flex flex-col items-center justify-center gap-[50px] py-[100px] px-[20px]",
            div {
                class: "max-w-[1440px] 2xl:w-[1440px] flex flex-col gap-[30px]",
                div {
                    class: "text-[42px] leading-[54px] font-black",
                    "Voices of Biyard"
                }
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 grid-row-dense gap-[30px]",
                    for (i, feed) in ctrl.feeds().iter().enumerate() {
                        FeedCard {
                            title: feed.title.clone(),
                            date: feed.published.clone(),
                            external_link: feed.external_link.clone(),
                            class: if i != ctrl.feeds().len() - 1 {
                                Some("border-b-[1px] sm:border-b-[0px] sm:border-r-[1px] border-white col-span-1 pb-[30px]".to_string())
                            } else {
                                None
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FeedCard(
    title: String,
    date: String,
    external_link: String,
    class: Option<String>,
) -> Element {
    rsx! {
        a {
            class: format!("flex flex-col gap-[10px] pl-[20px] items-start overflow-hidden opacity-75 hover:opacity-100 {}", class.unwrap_or_default()),
            href: external_link,
            target: "_blank",
            div {
                class: "text-[32px] leading-[38px] fond-bold",
                "{title}"
            }
            div {
                class: "text-[18px] leading-[24px] fond-light text-opacity-30",
                "{date}"
            }
        }
    }
}
