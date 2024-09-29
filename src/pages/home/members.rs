#![allow(non_snake_case)]
use crate::models::models::Member;
#[allow(unused_imports)]
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub(super) fn Members() -> Element {
    let ctrl = super::controller::Controller::use_controller();

    rsx! {
        div {
            class: "w-full flex flex-col items-center justify-center py-[100px] px-[20px]",
            div {
                class: "w-full 2xl:w-[1440px]  grid xl:grid-cols-4 lg:grid-cols-3 md:grid-cols-2 grid-cols-1 grid-flow-row-dense flex flex-wrap justify-center items-start content-start gap-[65px]",
                for member in ctrl.members().iter() {
                    MemberCard {
                        member: member.clone(),
                        class: Some("col-span-1".to_string())
                    }
                }
            }
        }
    }
}

#[component]
pub fn MemberCard(member: Member, class: Option<String>) -> Element {
    rsx! {
        div {
            class: "group w-full h-full",
            div {
                class: format!("rounded-[8px] h-[450px] transition-all duration-1000 [backface-visibility:hidden] [transform-style:preserve-3d] group-hover:[transform:rotateY(180deg)] flex flex-col gap-[10px] items-center justify-center bg-cover bg-[url('{}')] {}", member.image.clone(), class.unwrap_or_default()),
                div {
                    class: "w-full h-full bg-[#000000] bg-opacity-50 flex flex-col items-center justify-center [backface-visibility:hidden]",
                    div {
                        class: "text-[24px] leading-[40px] font-bold transition-all duration-1000 group-hover:[transform:translateX(200px)]",
                        "{member.name}"
                    }
                    div {
                        class: "text-[20px] leading-[45px] font-light text-opacity-30 transition-all duration-1000 group-hover:[transform:translateX(200px)]",
                        "{member.role}"
                    }
                }

                div {
                    class: "absolute inset-0 w-full h-full [backface-visibility:hidden] [transform:rotateY(180deg)] bg-[linear-gradient(180deg,rgba(92,115,185,0.2)_1.67%,rgba(179,48,225,0.2)_100%)] rounded-[8px] flex flex-col items-start justify-start p-[20px]",
                    div {
                        class: "flex flex-col w-full transition-all duration-1000 group-hover:[transform:translateX(0px)] [transform:translateX(-200px)]",
                        div {
                            class: "text-[24px] font-black",
                            "{member.role}"
                        }
                        div {
                            class: "w-[68px] h-[5px] bg-white"
                        }
                        div {
                            class: "flex flex-row justify-between items-center w-full",
                            div {
                                class: "text-[20px] font-black",
                                "{member.name}"
                            }
                            div {
                                class: "flex flex-row gap-[4px] items-center justify-center",
                                if member.email.is_some() {
                                    a {
                                        href: format!("mailto:{}", member.email.clone().unwrap()),
                                        icons::email { }
                                    }
                                }
                                if member.web.is_some() {
                                    a {
                                        href: member.web.clone().unwrap(),
                                        icons::web { }
                                    }
                                }
                                if member.linkedin.is_some() {
                                    a {
                                        href: member.linkedin.clone().unwrap(),
                                        icons::linkedin { }
                                    }
                                }
                                if member.github.is_some() {
                                    a {
                                        href: member.github.clone().unwrap(),
                                        icons::github { }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "text-[14px] font-regular transition-all duration-1000 [transform:translateX(-200px)] group-hover:[transform:translateX(0px)]",
                        "{member.description}"
                    }
                }
            }
        }
    }
}
