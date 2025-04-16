use bdk::prelude::*;
use common::*;

mod member_card;

#[component]
pub fn Team(lang: Language, members: Vec<MemberSummary>) -> Element {
    rsx! {
        section { id: "our-team", class: "w-full relative",
            div { class: "absolute top-[1/2] -left-313 h-1261 w-1261 bg-purple-blur/40 blur-[500px]" }

            div { class: "w-full max-w-wrapper min-h-screen flex flex-col gap-48 py-110 max-tablet:justify-center",
                h1 { class: "text-[45px]/64 text-center font-medium text-white",
                    "Our "
                    span { class: "text-primary", "Team" }
                }

                div {
                    class: "w-full grid grid-cols-4 gap-24 max-tablet:flex max-tablet:flex-row max-tablet:overflow-x-scroll",
                    style: "scrollbar-width: none; -ms-overflow-style: none; &::-webkit-scrollbar {{ display: none; }}",
                    for member in members {
                        member_card::MemberCard { member }
                    }
                }
            }
        }
    }
}
