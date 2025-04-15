use bdk::prelude::*;

use crate::SecondaryButton;

#[component]
pub fn Top(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    lang: Language,
) -> Element {
    let tr: TopTranslate = translate(&lang);

    rsx! {
        section { id: "top", class: "w-full", ..attributes,
            div { class: "w-849 h-849 absolute -top-356.74 -left-90 bg-purple-blur/80 opacity-50 blur-[500px] z-1" }
            div { class: "w-full max-w-wrapper min-h-screen flex flex-col z-2 py-100 justify-center max-tablet:max-w-full",
                div { class: "w-full flex flex-col gap-64 items-center",
                    div { class: "w-full max-w-648 flex flex-col gap-32 items-center",
                        dotlottie-player {
                            class: "w-208",
                            autoplay: true,
                            src: asset!("/public/logos/logo.json"),
                            speed: 1,
                        }
                        h1 { class: "font-black text-[64px]/89 tracking-[-0.69px] uppercase text-center",
                            {tr.deep}
                            {" "}
                            span { class: "text-primary", {tr.tech} }
                            br {}
                            {tr.deep}
                            {" "}
                            span { class: "text-primary", {tr.impact} }
                        }

                        p { class: "font-extralight text-[15px]/23 tracking-[0px] text-center",
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
