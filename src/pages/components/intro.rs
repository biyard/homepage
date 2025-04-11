use bdk::prelude::*;

use crate::*;

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
                class: "text-gray-300 font-extralight text-[15px]/23 tracking-[0px]",
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
        en: "Harness cutting-e dge deep-tech innovations, including Blockchain, AI, and Security, to deliver practical solutions that empower communities, enhance transparency, and foster inclusive growth.",
        ko: "첨단 블록체인, AI 및 보안 기술 혁신을 활용하여 지역 사회를 지원하고 투명성을 높이며 포용적 성장을 촉진하는 실용적인 솔루션을 제공합니다.",
    },
}
