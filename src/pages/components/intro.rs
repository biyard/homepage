use bdk::prelude::*;

#[component]
pub fn Intro(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    rsx! {
        section { id: "intro", ..attributes, {children} }
    }
}
