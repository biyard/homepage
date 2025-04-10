use bdk::prelude::*;

#[component]
pub fn Team(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    rsx! {
        section { id: "team", ..attributes, {children} }
    }
}
