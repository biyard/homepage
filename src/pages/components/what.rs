use bdk::prelude::*;

#[component]
pub fn WhatWeDo(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    rsx! {
        section { id: "what-we-do", ..attributes, {children} }
    }
}
