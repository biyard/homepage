use bdk::prelude::*;

#[component]
pub fn Input(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    placeholder: Option<String>,
    name: Option<String>,
    oninput: EventHandler<String>,
) -> Element {
    rsx! {
        input {
            class: "w-full h-44 rounded-[4px] border-b-1 border-gray-600 px-20 flex flex-row items-center focus:outline-none focus:border-b-primary",
            placeholder,
            name,
            oninput: move |event| {
                oninput(event.value());
            },
        }
    }
}
