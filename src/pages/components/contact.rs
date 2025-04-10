use bdk::prelude::*;

#[component]
pub fn Contact(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    rsx! {
        section { id: "contact", ..attributes, {children} }
    }
}
