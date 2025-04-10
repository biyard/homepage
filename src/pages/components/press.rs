use bdk::prelude::*;

#[component]
pub fn PressAndNews(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    rsx! {
        section { id: "press-and-news", ..attributes, {children} }
    }
}
