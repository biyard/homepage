use super::*;
use crate::{BiyardHorizontalSymbolSignature, Hamburger, MenuBack, route::Route};
use bdk::prelude::{by_components::meta::MetaSeoTemplate, *};

#[component]
pub fn IndexLayout(lang: Language) -> Element {
    let mut ctrl = LayoutController::new(lang)?;
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
            header { class: "fixed top-0 left-0 w-screen flex items-center justify-center z-50",
                div {
                    class: "w-full group flex flex-row justify-between items-center w-full max-w-1440 backdrop-blue-[50px] rounded-2xl my-24 py-20 px-30 gap-10 bg-menu-shade z-20 max-desktop:max-w-[calc(100vw-40px)] max-tablet:flex-col max-tablet:bg-transparent max-tablet:aria-expanded:bg-black max-tablet:my-0 max-tablet:max-w-full max-tablet:aria-expanded:h-screen",
                    "aria-expanded": ctrl.expanded_menu(),

                    div { class: "max-tablet:w-full flex flex-row justify-between items-center",
                        a { href: "#top", BiyardHorizontalSymbolSignature {} }
                        div {
                            class: "hidden overflow-hidden max-tablet:block cursor-pointer",
                            onclick: move |_| ctrl.toggle_menu(),
                            Hamburger { class: "block group-aria-[icon=open]:block" }
                            MenuBack { class: "hidden group-aria-[icon=opened]:block" }
                        }
                    }
                    nav { class: "flex flex-row justify-center items-center gap-48 font-outfit font-semibold text-base/16 tracking-[0.5px] text-center max-tablet:h-full max-tablet:flex-col max-tablet:z-100 max-tablet:bg-black max-tablet:hidden max-tablet:group-aria-expanded:flex",
                        MenuItem {
                            href: "#intro",
                            onclick: move |_| ctrl.expanded_menu.set(false),
                            {tr.intro}
                        }

                        MenuItem {
                            href: "#what-we-do",
                            onclick: move |_| ctrl.expanded_menu.set(false),
                            {tr.what}
                        }

                        MenuItem {
                            href: "#our-team",
                            onclick: move |_| ctrl.expanded_menu.set(false),
                            {tr.team}
                        }

                        MenuItem {
                            href: "#press-and-news",
                            onclick: move |_| ctrl.expanded_menu.set(false),
                            {tr.press}
                        }


                        a {
                            class: "text-bg py-10 px-20 rounded-[50px] bg-primary hover:bg-primary/80",
                            href: "#contact",
                            onclick: move |_| ctrl.expanded_menu.set(false),
                            {tr.contact}
                        }
                    }
                }
            }

            Outlet::<Route> {}
        }
    }
}

#[component]
pub fn MenuItem(href: String, onclick: EventHandler<()>, children: Element) -> Element {
    let mut hover = use_signal(|| false);

    rsx! {
        a {
            class: "flex flex-col min-w-110 gap-7 mt-7 items-center max-tablet:hover:text-primary",
            href,
            onmouseenter: move |_| hover.set(true),
            onmouseleave: move |_| hover.set(false),
            onclick: move |_| {
                onclick.call(());
            },
            {children}
            div {
                class: "transition-all duration-500 w-0 h-1 aria-hover:bg-primary aria-hover:w-full max-tablet:aria-hover:bg-transparent",
                "aria-hover": hover(),
            }
        }
    }
}
