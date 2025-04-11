use bdk::prelude::*;

use crate::{SecondaryButton, TOP_BG};

#[component]
pub fn Top(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    lang: Language,
) -> Element {
    let tr: TopTranslate = translate(&lang);

    rsx! {
        section { id: "top", ..attributes,
            div { class: "w-849 h-849 absolute -top-[356.74px] -left-[90px] bg-[rgba(33,0,151,0.8)] opacity-50 border border-black blur-[250px] box-border z-1" }
            img { class: "absolute top-64 right-0 z-0", src: TOP_BG }
            div { class: "w-full max-w-1440 flex flex-col z-2 my-290",
                div { class: "w-full flex flex-col gap-96",
                    div { class: "w-full max-w-648 flex flex-col gap-32",
                        h1 { class: "font-black text-[64px]/89 tracking-[-0.69px] uppercase",
                            {tr.deep}
                            {" "}
                            span { class: "text-primary", {tr.tech} }
                            br {}
                            {tr.deep}
                            {" "}
                            span { class: "text-primary", {tr.impact} }
                        }

                        p { class: "font-extralight text-[15px]/23 tracking-[0px]",
                            {tr.description}
                        }
                    }

                    SecondaryButton { onclick: |_| {}, {tr.btn_see_all_services} }
                }
            }

        }
    }
}

translate! {
    TopTranslate;

    deep: {
        ko: "DEEP",
        en: "DEEP",
    },

    tech: {
        ko: "TECH.",
        en: "TECH.",
    },

    impact: {
        ko: "IMPACT.",
        en: "IMPACT.",
    },

    description: {
        ko: "",
        en: "Biyard is a leading blockchain technology company focused on building decentralized solutions that drive innovation and transparency. From secure digital content rights protection to enhancing the transparency, trust, and efficiency of public polls and surveys, we empower governments, enterprises, and developers to unlock the full potential of Web3. Our flagship platform, d.AGIT, pioneers new ways to safeguard and manage digital assets with trust and security. At Biyard, we’re shaping the future of a decentralized digital economy.",
    },

    btn_see_all_services: {
        ko: "모든 서비스 보기",
        en: "See All Services",
    },
}
