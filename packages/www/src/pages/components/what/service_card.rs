use bdk::prelude::{by_components::icons::arrows::ArrowRight, *};

#[component]
pub fn ServiceCard(
    title: String,
    description: String,
    to: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "col-span-1 w-full flex flex-col items-start bg-black/50 rounded-[16px] border border-gray-800 p-32 gap-54",
            div { class: "w-full flex flex-col items-start gap-12",
                h2 { class: "text-[28px]/36 text-white font-semibold", {title} }
                p { class: "text-[15px]/23 text-gray-300 font-extralight", {description} }
            }
            div { class: "w-full flex flex-col items-start gap-24",
                a {
                    class: "text-base/16 font-semibold tracking-[0.5px] text-white flex flex-row gap-4 h-24 aria-hidden:hidden",
                    "aria-hidden": to.is_none(),
                    href: to,
                    "Explore the service"
                    ArrowRight {
                        class: "[&>path]:stroke-white",
                        height: "18",
                        width: "18",
                    }
                }
                div { class: "w-full flex items-center justify-center", {children} }
            }
        }
    }
}
