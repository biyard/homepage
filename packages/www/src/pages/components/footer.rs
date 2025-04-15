use bdk::prelude::*;

use crate::{Github, LinkedIn};

#[component]
pub fn Footer(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    let tr: FooterTranslate = translate(&lang);

    rsx! {
        footer { class: "w-full max-w-wrapper flex flex-row justify-between items-center mx-auto py-24",
            p { class: "text-[15px]/23 font-extralight", {tr.copyright} }
            div { id: "socials", class: "flex flex-row gap-50",
                a { href: "https://github.com/biyard", target: "_blank",
                    Github { class: "hover:[&>path]:fill-white/80" }
                }
                a {
                    href: "https://www.linkedin.com/company/75498162",
                    target: "_blank",
                    LinkedIn { class: "hover:[&>path]:fill-white/80" }
                }
            }
        }
    }
}

translate! {
    FooterTranslate;

    copyright: {
        ko: "© Biyard. All rights reserved.",
        en: "© Biyard. All rights reserved.",
    },
}
