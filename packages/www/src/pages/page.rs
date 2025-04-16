use super::*;
use bdk::prelude::*;

use common::*;

#[component]
pub fn IndexPage(lang: Language) -> Element {
    let tr: IndexTranslate = translate(&lang);
    let news = use_server_future(|| async {
        let conf = crate::config::get();
        let news = match News::get_client(conf.api_endpoint)
            .query(NewsQuery::new(4))
            .await
        {
            Ok(res) => res.items,
            Err(e) => {
                tracing::error!("Failed to fetch news: {}", e);
                vec![]
            }
        };
        let members = match Member::get_client(conf.api_endpoint)
            .query(MemberQuery::new(4))
            .await
        {
            Ok(members) => members.items,
            Err(e) => {
                tracing::error!("Failed to fetch members: {:?}", e);
                vec![]
            }
        };
        (members, news)
    })?;

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
            if let Some((members, news)) = news() {
                Team { lang, members }
                PressAndNews { lang, news }
            }
            Contact { lang }
            div { class: "w-full items-center flex flex-col gap-393 max-tablet:gap-276",
                Updates { lang }
                Footer { lang }
            }
        } // end of this page
    }
}
