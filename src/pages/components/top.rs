use bdk::prelude::*;

#[component]
pub fn Top(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    lang: Language,
) -> Element {
    let tr: TopTranslate = translate(&lang);

    rsx! {
        section { id: "top", ..attributes,
            div { class: "w-849 h-849 absolute -top-[356.74px] -left-[90px] bg-[rgba(33,0,151,0.8)] opacity-50 border border-black blur-[250px] box-border" }
            div { class: "w-full max-w-1440",
                div { class: "w-full max-w-648 flex flex-col",
                    h1 { class: "font-black text-[64px]/89 tracking-[-0.69px] uppercase",
                        {tr.deep}
                        {" "}
                        span { class: "text-primary", {tr.tech} }
                    }

                    h1 { class: "font-black text-[64px]/89 tracking-[-0.69px] uppercase",
                        {tr.deep}
                        {" "}
                        span { class: "text-primary", {tr.impact} }
                    }

                    p { class: "font-extralight text-[15px]/23 tracking-[0px]", {tr.description} }
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
}
