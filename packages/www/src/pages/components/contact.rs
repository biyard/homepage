use bdk::prelude::*;

#[component]
pub fn Contact(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    let tr: ContactTranslate = translate(&lang);

    rsx! {
        section { id: "contact", class: "w-full relative",
            div { class: "absolute top-[1/2] left-[2/3] h-1328 w-1328 bg-purple-blur/40 blur-[500px]" }
            div { class: "w-full max-w-wrapper grid grid-cols-2 gap-24 py-120",
                h1 { class: "col-span-1 text-[45px]/64 text-center font-medium text-white",
                    "Contact "
                    span { class: "text-primary", "Us" }
                }

                div {
                    id: "contact-form",
                    class: "col-span-1 w-full flex flex-col gap-48",
                    TextInput {
                        label: tr.first_name,
                        placeholder: "Name",
                        oninput: move |name| {
                            tracing::debug!("name: {}", name);
                        },
                    }
                }
            }

        }
    }
}

#[component]
pub fn TextInput(
    label: String,
    placeholder: Option<String>,
    oninput: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "w-full flex flex-col items-start gap-8",
            label { class: "text-[15px]/22 tracking-[0.5px] font-medium text-neutral-400",
                {label}
            }
            input {
                class: "w-full h-44 rounded-[4px] border-b-1 border-gray-600 px-20 flex flex-row items-center",
                placeholder,
                oninput: move |event| {
                    oninput(event.value());
                },
            }
        }
    }
}

translate! {
    ContactTranslate;

    first_name: {
        ko: "이름",
        en: "First name",
    },
}
