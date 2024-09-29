#![allow(non_snake_case)]
use crate::components::filled_button::FilledButton;
#[allow(unused_imports)]
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub(super) fn Following() -> Element {
    rsx! {
        div {
            class: "w-full flex flex-col items-center justify-center gap-[50px]",
            div {
                class: "max-w-[1440px] 2xl:w-[1440px] m-auto grid grid-cols-3 px-[100px] py-[35px] items-center bg-[linear-gradient(285.82deg,#392D69_-64.65%,#7754AC_88.96%)] rounded-[20px]",
                div {
                    class: "flex flex-col justify-start items-start gap-[10px] col-span-2",
                    div {
                        class: "text-[32px] leading-[40px] font-bold",
                        "For technical collaborations, investment proposals, or service inquiries, please contact us at the email below."
                    }
                    input {
                        class: "w-full h-[52px] bg-transparent rounded-[10px] px-[24px] py-[14px] text-[16px] leading-[24px] font-regular border-[1px] border-white focus:outline-none focus:border-[#03AB79] transition-all duration-300 ease-in-out",
                        placeholder: "Enter your email here"
                    }
                    FilledButton {
                        onclick: move |_| {},
                        "GET UPDATES"
                    }
                }
                div {
                    class: "col-span-1",
                }
            }
        }
    }
}
