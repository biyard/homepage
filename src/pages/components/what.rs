use approach::Approach;
use bdk::prelude::*;
use service_card::ServiceCard;

use crate::{DAGIT, RATEL, SUS};
mod approach;
mod icons;
mod service_card;

#[component]
pub fn WhatWeDo(children: Element, lang: Language) -> Element {
    let tr: WhatWeDoTranslate = translate(&lang);

    rsx! {
        section { id: "what-we-do", class: "w-full relative py-100",
            div { class: "absolute left-[1/2] h-879 w-879 bg-purple-blur/50 blur-[300px]" }
            div { class: "relative w-full max-w-wrapper min-h-screen flex flex-col items-center justify-center gap-48",
                div { class: "w-full flex flex-col items-center gap-24 z-1",
                    h1 { class: "text-[45px]/64 text-center font-medium text-white",
                        {tr.what}
                        {" "}
                        span { class: "text-primary", {tr.we_do} }
                    }
                    p { class: "text-[15px]/23 text-center font-extralight text-gray-300 whitespace-pre-line",
                        {tr.description}
                    }
                }

                div { id: "services", class: "grid grid-cols-3 gap-25",
                    ServiceCard {
                        title: tr.dagit_title,
                        description: tr.dagit_description,
                        to: "https://dagit.club",
                        img { src: DAGIT }
                    }
                    ServiceCard {
                        title: tr.ratel_title,
                        description: tr.ratel_description,
                        to: "https://ratel.foundation",
                        img { src: RATEL }
                    }
                    ServiceCard { title: tr.sus_title, description: tr.sus_description,
                        img { src: SUS }
                    }
                } // services

                div {
                    id: "our-approach",
                    class: "w-full flex flex-row gap-58 justify-start",

                    h1 { class: "text-[45px]/64 text-left font-medium text-white",
                        {tr.our}
                        {" "}
                        span { class: "text-primary", {tr.approach} }
                    }

                    div { class: "w-full grid grid-cols-3 gap-64 items-start justify-start",
                        Approach {
                            title: tr.practical,
                            description: tr.practical_description,
                            icons::Pratical {}
                        }

                        Approach {
                            title: tr.people,
                            description: tr.people_description,
                            icons::PeopleCentric {}
                        }

                        Approach {
                            title: tr.scalable,
                            description: tr.scalable_description,
                            icons::Scalable {}
                        }
                    }
                }
            }
        }
    }
}

translate! {
    WhatWeDoTranslate;

    what: {
        ko: "What",
        en: "What",
    },
    we_do: {
        ko: "we do",
        en: "we do",
    },
    description: {
        ko: "Biyard는 블록체인, 인공지능, 보안 등 첨단 기술을 활용하여\n실제 문제를 해결합니다. 우리의 초점은 다음과 같은 주요 분야에 걸쳐 있습니다",
        en: "At Biyard, we leverage advanced technologies like Blockchain, Artificial Intelligence, and\nSecurity to solve real-world problems. Our focus spans critical sectors including",
    },

    dagit_title: {
        ko: "Art & Culture",
        en: "Art & Culture",
    },

    dagit_description: {
        ko: "",
        en: "Revolutionizing art management and marketplaces through decentralized platforms.",
    },

    ratel_title: {
        ko: "Democracy & Governance",
        en: "Democracy & Governance",
    },

    ratel_description: {
        ko: "",
        en: "Strengthening democratic processes with transparent, secure digital solutions.",
    },

    sus_title: {
        ko: "Sustainability",
        en: "Sustainability",
    },

    sus_description: {
        ko: "",
        en: "Ensuring our technology contributes positively to society and the planet.",
    },

    our: {
        ko: "+ Our",
        en: "+ Our",
    },

    approach: {
        ko: "Approach",
        en: "Approach",
    },

    practical: {
        ko: "Practical",
        en: "Practical",
    },

    practical_description: {
        ko: "",
        en: "Real solutions for real problems.",
    },

    people: {
        ko: "People-Centric",
        en: "People-Centric",
    },

    people_description: {
        ko: "",
        en: "Technology built with and for communities.",
    },

    scalable: {
        ko: "Scalable",
        en: "Scalable",
    },

    scalable_description: {
        ko: "",
        en: "Global vision, local impact.",
    },
}
