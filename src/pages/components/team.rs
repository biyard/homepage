use bdk::prelude::*;

use crate::members::get_members;
mod member_card;

#[component]
pub fn Team(lang: Language) -> Element {
    let members = use_server_future(|| async { get_members().await.unwrap_or_default() })?;

    rsx! {
        section { id: "our-team", class: "w-full relative",
            div { class: "absolute top-[1/2] -left-313 h-1261 w-1261 bg-purple-blur/40 blur-[500px]" }

            div { class: "w-full max-w-wrapper min-h-screen flex flex-col gap-48 py-110",
                h1 { class: "text-[45px]/64 text-center font-medium text-white",
                    "Our "
                    span { class: "text-primary", "Team" }
                }

                div { class: "w-full grid grid-cols-4 gap-24",
                    for member in members().unwrap_or_default() {
                        member_card::MemberCard { member }
                    }
                }
            }
        }
    }
}
