use bdk::prelude::*;

#[component]
pub fn Footer(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    rsx! {
        footer { ..attributes,{children} }
    }
}
