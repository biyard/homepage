#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn FeatureButton(completed: Option<bool>, no_features: usize, text: String) -> Element {
    rsx! {
        div {
            class: "border border-[#027351]/30 text-[13px] font-regular text-white hover:bg-[#014D36] cursor-pointer transition-all duration-300 ease-in-out flex justify-center items-center active:bg-[#03AB79] active:text-white active:border-[#03AB79] rounded-[10px] px-[20px] py-[30px] w-[170px] h-[240px] flex-col",
            div {
                class: "flex flex-col justify-end items-end h-[160px]",
                if completed.unwrap_or(false) {
                    img {
                        src: asset!("./assets/images/done.png"),
                        class: "w-[25px] h-[25px]",
                    }
                }
                div {
                    class: "text-[120px] font-regular leading-[130px]",
                    "{no_features}"
                }
            }
            div {
                class: "text-[13px] font-regular",
                "{text}"
            }
        }
    }
}

#[component]
pub fn ProgressFeature(
    code_name: String,
    goal: String,
    text: String,
    no_features: usize,
    completed_date: Option<String>,
) -> Element {
    // let _ = "";
    rsx! {
        div {
            class: format!("border border-[#027351]/30 text-[13px] font-regular text-white hover:bg-[#014D36] cursor-pointer transition-all duration-300 ease-in-out flex justify-between items-center active:bg-[#03AB79] active:text-white active:border-[#03AB79] rounded-[10px] px-[20px] py-[30px] w-[520px] h-[240px] flex-row {}", if completed_date.clone().is_none() { "bg-[#027351]" } else { "" }),
            div {
                class: "flex flex-col justify-between items-start h-[160px]",
                div {
                    class: "flex flex-col",
                    div {
                        class: "text-[32px] font-bold leading-[32px]",
                        "{code_name}"
                    }
                    if let Some(date) = completed_date.clone() {
                        div {
                            class: "text-[16px] font-light",
                            span {
                                class: "text-white/30",
                                "Completed "
                            }
                            span {
                                // FIXME: Re
                                "{date}"
                            }
                        }
                    }
                }
                div {
                    class: "text-[16px] font-regular",
                    "{goal}"
                }
            }
            div {
                class: "flex flex-col",
                div {
                    class: "flex flex-col justify-end items-end h-[160px]",
                    if completed_date.is_some() {
                        img {
                            src: asset!("./assets/images/done.png"),
                             class: "w-[25px] h-[25px]",
                        }
                    }
                    div {
                        class: "text-[120px] font-regular leading-[130px]",
                        "{no_features}"
                    }
                }
                div {
                    class: "text-[13px] font-regular",
                    "{text}"
                }
            }
        }
    }
}
