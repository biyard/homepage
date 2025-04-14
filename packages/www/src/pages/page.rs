use super::*;
use bdk::prelude::*;

#[component]
pub fn IndexPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: IndexTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage {
            title: tr.title,
            description: tr.description,
            image: crate::MEET_BIYARD.to_string(),
        }

        div { id: "index-page", class: "flex flex-col w-full justify-start",
            Top { lang }
            Intro { lang }
            WhatWeDo { lang }
            Team { lang }
            PressAndNews { lang }
            Contact { lang }
            Footer { lang }
        } // end of this page
    }
}
