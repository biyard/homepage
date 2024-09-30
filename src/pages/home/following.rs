#![allow(non_snake_case)]

#[allow(unused_imports)]
use crate::prelude::*;
use crate::{
    apis::users::{keep_updates, KeepUpdatesRequest},
    components::filled_button::FilledButton,
};
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[component]
pub(super) fn Following() -> Element {
    let mut email = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "w-full flex flex-col items-center justify-center gap-[50px] px-[20px]",
            div {
                class: "max-w-[1440px] 2xl:w-[1440px] m-auto grid grid-cols-3 px-[50px] md:px-[100px] py-[35px] items-center bg-[linear-gradient(285.82deg,#392D69_-64.65%,#7754AC_88.96%)] rounded-[20px]",
                div {
                    class: "flex flex-col justify-start items-start gap-[10px] col-span-3 md:col-span-2",
                    div {
                        class: "text-[18px] md:text-[32px] font-bold",
                        "For technical collaborations, investment proposals, or service updates, please enter your email below."
                    }
                    input {
                        class: "w-full h-[52px] bg-transparent rounded-[10px] px-[24px] py-[14px] text-[16px] leading-[24px] font-regular border-[1px] border-white focus:outline-none focus:border-[#03AB79] transition-all duration-300 ease-in-out",
                        placeholder: "Enter your email here",
                            onchange: move |e| {
                                tracing::debug!("email: {}", e.value());
                                email.set(e.value());
                                e.stop_propagation();
                            },
                    }
                    FilledButton {
                        onclick: move |_| {
                            let email = email().clone();
                            if !email.is_empty() && email.contains('@') {
                                spawn(async move {
                                    match keep_updates(KeepUpdatesRequest { email }).await {
                                        Ok(_) => {},
                                        Err(_) => {
                                            tracing::error!("Failed to subscribe!");
                                        }
                                    };
                                });
                            }
                        },
                        "GET UPDATES"
                    }
                }
                div {
                    class: "col-span-0 md:col-span-1",
                }
            }
        }
    }
}
