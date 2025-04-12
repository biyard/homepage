use bdk::prelude::*;

#[component]
pub fn Approach(title: String, description: String, children: Element) -> Element {
    rsx! {
        div { class: "col-span-1 flex flex-col gap-8 items-start justify-start",
            {children}
            div { class: "flex flex-col gap-12 items-start justify-start",
                h2 { class: "text-[28px]/36 text-white font-semibold", {title} }
                p { class: "text-[15px]/23 text-gray-300 font-extralight", {description} }
            }
        }
    }
}
