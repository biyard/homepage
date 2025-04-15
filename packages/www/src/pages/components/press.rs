use bdk::prelude::*;
use common::{News, NewsQuery};

use crate::config;

#[component]
pub fn PressAndNews(lang: Language) -> Element {
    let news = use_resource(|| async {
        match News::get_client(config::get().api_endpoint)
            .query(NewsQuery::new(4))
            .await
        {
            Ok(res) => res.items,
            Err(e) => {
                tracing::error!("Failed to fetch news: {}", e);
                vec![]
            }
        }
    });

    rsx! {
        section { id: "press-and-news", class: "w-full py-120",
            div { class: "w-full max-w-wrapper flex flex-col items-start justify-center gap-48",
                h1 { class: "text-[45px]/64 text-center font-medium text-white",
                    "Press & "
                    span { class: "text-primary", "News" }
                }

                if let Some(news) = news() {
                    div {
                        id: "news-container",
                        class: "w-full grid grid-cols-2 gap-22 max-tablet:grid-cols-1",

                        div {
                            id: "main-news",
                            class: "w-full flex flex-col gap-24",

                            img {
                                src: news[0].image.clone(),
                                class: "w-full h-410 object-cover rounded-[16px]",
                            }

                            div { class: "flex flex-col gap-4",
                                label { class: "text-sm/16 text-primary font-medium tracking-[0.5px]",
                                    {news[0].category.clone()}
                                }
                                h2 { class: "text-[28px]/36 text-white font-semibold",
                                    {news[0].title.clone()}
                                }
                            }

                            p { class: "text-[15px]/23 text-white font-extralight",
                                {news[0].contents.clone()}
                            }
                        }

                        div {
                            id: "sub-news",
                            class: "h-full w-full grid grid-rows-3 gap-24 max-tablet:hidden",
                            for i in 1..news.len() {
                                div { class: "w-full flex flex-row gap-24 items-center",
                                    img {
                                        src: news[i].image.clone(),
                                        class: "w-300 h-full object-cover rounded-[16px]",
                                    }
                                    div {
                                        class: "flex flex-col gap-4",
                                        id: "sub-news-content",
                                        label { class: "text-sm/16 text-primary font-medium tracking-[0.5px]",
                                            {news[i].category.clone()}
                                        }
                                        h2 { class: "text-[28px]/36 text-white font-semibold",
                                            {news[i].title.clone()}
                                        }

                                    }
                                }
                            }
                        }
                    }
                }


            }
        }
    }
}
