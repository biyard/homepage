use super::*;
use crate::{BiyardHorizontalSymbolSignature, route::Route};
use bdk::prelude::{by_components::meta::MetaSeoTemplate, *};

#[component]
pub fn IndexLayout(lang: Language) -> Element {
    let tr: LayoutTranslate = translate(&lang);

    #[cfg(feature = "web")]
    use_effect(|| {
        use wasm_bindgen::JsCast;
        use web_sys::{Element, window};

        if let Some(window) = window() {
            let location = window.location();
            if let Ok(hash) = location.hash() {
                tracing::debug!("hash :{hash}");

                if !hash.is_empty() {
                    let document = window.document().unwrap();
                    // web_sys::console::log_1(&hash);
                    if let Some(target) = document.query_selector(&hash).ok().flatten() {
                        let scroll = web_sys::ScrollIntoViewOptions::new();
                        scroll.set_behavior(web_sys::ScrollBehavior::Smooth);

                        let _ = target
                            .dyn_ref::<Element>()
                            .unwrap()
                            .scroll_into_view_with_scroll_into_view_options(&scroll);
                    }
                }
            }
        }
    });

    rsx! {
        MetaSeoTemplate {
            lang,
            title: "Biyard",
            keywords: "blockchain, ai, security, cryptography, web3, metaverse, digital twin, digital asset, NFT, tokenization, digital identity, digital wallet, digital currency, Physical AI, DID, Cryptocurrency, DAO, Deeptech",
            author: "Biyard Corp.",
            url: "https://biyard.co",
        }

        div { class: "w-full flex flex-col justify-start items-center overflow-hidden",
            header { class: "fixed top-0 left-1/2 -translate-x-1/2  flex flex-row justify-between items-center w-full max-w-1440 my-24 backdrop-blue-[50px] rounded-2xl py-20 px-30 gap-10 bg-menu-shade z-20 max-desktop:max-w-[calc(100vw-40px)]",
                a { href: "#top", BiyardHorizontalSymbolSignature {} }
                nav { class: "flex flex-row justify-center items-center gap-48 font-outfit font-semibold text-base/16 tracking-[0.5px] text-center",
                    MenuItem { href: "#intro", {tr.intro} }

                    MenuItem { href: "#what-we-do", {tr.what} }

                    MenuItem { href: "#our-team", {tr.team} }

                    MenuItem { href: "#press-and-news", {tr.press} }

                    a {
                        class: "text-bg py-10 px-20 rounded-[50px] bg-primary",
                        href: "#contact",
                        {tr.contact}
                    }
                }
            }

            Outlet::<Route> {}
        }
    }
}

#[component]
pub fn MenuItem(href: String, children: Element) -> Element {
    let mut hover = use_signal(|| false);

    rsx! {
        a {
            class: "flex flex-col min-w-110 gap-7 mt-7 items-center",
            href,
            onmouseenter: move |_| hover.set(true),
            onmouseleave: move |_| hover.set(false),
            {children}
            div {
                class: "transition-all duration-500 w-0 h-1 aria-hover:bg-primary aria-hover:w-full",
                "aria-hover": hover(),
            }
        }
    }
}
