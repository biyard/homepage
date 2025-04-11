use animation::IntroAnimation;
use bdk::prelude::*;

use crate::*;
mod animation;

#[component]
pub fn Intro(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    let tr: IntroTranslate = translate(&lang);

    rsx! {
        section { id: "intro",
            div { class: "w-full max-w-1440 flex flex-col gap-96",
                div { class: "border w-full border-gray-800 backdrop-blur-sm rounded-[16px] py-40 px-118 flex flex-row justify-between items-start gap-120 max-desktop:flex-col max-desktop:gap-40 max-desktop:px-20",
                    LabeledTextWithLogo {
                        icon: rsx! {
                            BiyardSymbolOnly { width: 20 }
                        },
                        title: tr.vision,
                        description: tr.vision_description,
                    }

                    LabeledTextWithLogo {
                        icon: rsx! {
                            BiyardSymbolOnly { width: 20 }
                        },
                        title: tr.mission,
                        description: tr.mission_description,
                    }
                } // vision and mission

                div { class: "w-full flex flex-row items-center justify-between gap-187",
                    IntroAnimation {}
                    div { class: "flex flex-col gap-48 items-start",
                        h2 { class: "font-medium text-[45px]/64",
                            "Core "
                            span { class: "text-primary", "Values" }
                        }

                        div { class: "w-full grid grid-cols-2 gap-x-51 gap-y-80",
                            LabeledTextWithLogo {
                                icon: rsx! {
                                    Glasses {}
                                },
                                title: tr.innovation,
                                description: tr.innovation_description,
                                small_gap: true,
                            }
                            LabeledTextWithLogo {
                                icon: rsx! {
                                    Glasses {}
                                },
                                title: tr.inclusivity,
                                description: tr.inclusivity_description,
                                small_gap: true,
                            }
                            LabeledTextWithLogo {
                                icon: rsx! {
                                    Glasses {}
                                },
                                title: tr.integrity,
                                description: tr.integrity_description,
                                small_gap: true,
                            }
                            LabeledTextWithLogo {
                                icon: rsx! {
                                    Glasses {}
                                },
                                title: tr.sustainability,
                                description: tr.sustainability_description,
                                small_gap: true,
                            }
                        }
                    }

                }
            }
        }
    }
}

#[component]
pub fn LabeledTextWithLogo(
    icon: Element,
    title: String,
    description: String,
    #[props(default = false)] small_gap: bool,
) -> Element {
    rsx! {
        div { class: "flex-1 flex flex-col gap-24",
            div { class: "flex flex-row",
                div {
                    class: "w-54 aria-sm:w-44 flex flex-row justify-start items-center",
                    "aria-sm": small_gap,
                    {icon}
                }
                label { class: "font-semibold text-[28px]/36 tracking-[0px]", {title} }
            }
            p {
                class: "text-gray-300 font-extralight text-[15px]/23 tracking-[0px] ml-54 aria-sm:ml-44 whitespace-pre-line",
                "aria-sm": small_gap,

                {description}
            }
        }
    }
}

translate! {
    IntroTranslate;

    vision: {
        ko: "Vision",
        en: "Vision",
    },

    vision_description: {
        en: "To create a future where technology bridges gaps and transforms global challenges into sustainable opportunities.",
        ko: "기술이 격차를 해소하고 글로벌 과제를 지속 가능한 기회로 전환하는 미래를 창조합니다.",
    },

    mission: {
        ko: "Mission",
        en: "Mission",
    },

    mission_description: {
        en: "Harness cutting-edge deep-tech innovations, including Blockchain, AI, and Security, to deliver practical solutions that empower communities, enhance transparency, and foster inclusive growth.",
        ko: "첨단 블록체인, AI 및 보안 기술 혁신을 활용하여 지역 사회를 지원하고 투명성을 높이며 포용적 성장을 촉진하는 실용적인 솔루션을 제공합니다.",
    },

    innovation: {
        ko: "혁신",
        en: "Innovation",
    },

    innovation_description: {
        en: "Continuously exploring and\npioneering new technologies",
        ko: "지속적으로 새로운 기술을 탐구하고 개척합니다",
    },

    inclusivity: {
         ko: "포용성",
         en: "Inclusivity",
    },

    inclusivity_description: {
        en: "Designing solutions accessible to\neveryone, everywhere.",
        ko: "모든 사람이 어디서나 접근할 수 있는 솔루션을 설계합니다.",
    },

    integrity: {
         ko: "진실성",
         en: "Integrity",
    },

    integrity_description: {
        en: "Upholding transparency and trust in\nevery interaction.",
        ko: "모든 상호 작용에서 투명성과 신뢰를 지킵니다.",
    },

    sustainability: {
         ko: "지속가능성",
         en: "Sustainability",
    },

    sustainability_description: {
        en: "Ensuring our technology contributes\npositively to society and the planet.",
        ko: "우리의 기술이 사회와 지구에 긍정적으로 기여하도록 보장합니다.",
    },
}
