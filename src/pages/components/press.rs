use bdk::prelude::*;

use crate::news::get_news;

#[component]
pub fn PressAndNews(lang: Language) -> Element {
    let news = use_server_future(|| async { get_news().await.unwrap_or_default() })?;
    let news = news().unwrap_or_default();
    let (main_news, sub_news) = news.split_at(1);

    rsx! {
        section { id: "press-and-news", class: "w-full py-120",
            div { class: "w-full max-w-wrapper flex flex-col items-start justify-center gap-48",
                h1 { class: "text-[45px]/64 text-center font-medium text-white",
                    "Press & "
                    span { class: "text-primary", "News" }
                }

                div {
                    id: "news-container",
                    class: "w-full grid grid-cols-2 gap-22",

                    div { id: "main-news", class: "w-full flex flex-col gap-24",

                        img {
                            src: main_news[0].image.clone(),
                            class: "w-full h-410 object-cover rounded-[16px]",
                        }

                        div { class: "flex flex-col gap-4",
                            label { class: "text-sm/16 text-primary font-medium tracking-[0.5px]",
                                {main_news[0].category.clone()}
                            }
                            h2 { class: "text-[28px]/36 text-white font-semibold",
                                {main_news[0].title.clone()}
                            }
                        }

                        p { class: "text-[15px]/23 text-white font-extralight",
                            {main_news[0].contents.clone()}
                        }
                    }

                    div {
                        id: "sub-news",
                        class: "h-full w-full grid grid-rows-3 gap-24",
                        for news in sub_news {
                            div { class: "w-full flex flex-row gap-24",
                                img {
                                    src: news.image.clone(),
                                    class: "w-300 h-full object-cover rounded-[16px]",
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}
