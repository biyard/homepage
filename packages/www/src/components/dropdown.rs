use bdk::prelude::*;
use by_components::icons::{arrows::ShapeArrowDown, validations::Check};

#[component]
pub fn Dropdown(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    items: Vec<String>,
    #[props(default = 0)] selected: usize,
    onselect: EventHandler<usize>,
) -> Element {
    let mut selected_item = use_signal(|| 0);
    let mut opened = use_signal(|| false);

    rsx! {
        div {
            class: "relative group w-full h-full inline-block text-left",
            "aria-expanded": opened(),
            div { class: "w-full h-full min-w-150 max-mobile:!min-w-100",
                button {
                    class: "inline-flex w-full h-full flex-row justify-between gap-x-1.5 px-20 py-12 text-sm font-semibold text-white items-center cursor-pointer border-b-1 border-b-gray-600 rounded-sm group-aria-expanded:border-b-primary",
                    id: "menu-button",
                    r#type: "button",
                    onclick: move |_| opened.set(!opened()),
                    span { class: "text-left", {items[selected_item()].clone()} }
                    ShapeArrowDown {
                        class: "[&>path]:stroke-gray-700 [&>path]:fill-gray-700 transition-all group-aria-expanded:rotate-180",
                        size: 20,
                    }
                }
            }
            div {
                aria_labelledby: "menu-button",
                aria_orientation: "vertical",
                "aria-hidden": !opened(),
                class: "absolute right-0 z-10 w-full mt-10 origin-top-right rounded-[10px] ring-1 ring-primary focus:outline-hidden bg-black overflow-hidden aria-hidden:hidden",
                role: "menu",
                tabindex: "-1",
                div { class: "py-1", role: "none",
                    for (i , item) in items.into_iter().enumerate() {
                        div {
                            id: "menu-item-{item}",
                            class: "group text-sm text-gray-700 text-c-wg-50 font-semibold py-15 px-20 flex flex-row w-full justify-between hover:text-white items-center cursor-pointer aria-selected:text-white",
                            "aria-selected": i == selected_item(),
                            onclick: move |_| {
                                opened.set(false);
                                selected_item.set(i);
                                onselect(i);
                            },
                            role: "menuitem",
                            tabindex: "-1",
                            {item.clone()}
                            Check { class: "[&>path]:stroke-white hidden group-aria-selected:block" }
                        }
                    }
                }
            }
        }
    }
}
