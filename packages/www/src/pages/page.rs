use super::*;
use bdk::prelude::*;

#[component]
pub fn IndexPage(lang: Language) -> Element {
    let tr: IndexTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage {
            title: tr.title,
            description: tr.description,
            image: crate::MEET_BIYARD.to_string(),
        }

        div {
            id: "index-page",
            class: "flex flex-col w-full justify-start items-center",
            Top { lang }
            Intro { lang }
            WhatWeDo { lang }
            Team { lang }
            PressAndNews { lang }
            Contact { lang }
            div { class: "w-full items-center flex flex-col gap-393",
                Updates { lang }
                Footer { lang }
            }
        } // end of this page
    }
}
