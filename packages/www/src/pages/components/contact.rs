mod controller;
mod i18n;

use bdk::prelude::*;
use common::Need;

use crate::{Dropdown, Input, SecondaryButton};

#[component]
pub fn Contact(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    let mut ctrl = controller::Controller::new(lang)?;
    let tr: i18n::ContactTranslate = translate(&lang);

    rsx! {
        section {
            id: "contact",
            class: "w-full relative flex flex-col justify-center",
            div { class: "absolute top-[1/2] left-[2/3] h-1328 w-1328 bg-purple-blur/40 blur-[500px] max-tablet:w-614 max-tablet:h-614 max-tablet:bg-purple-blur/80" }
            div { class: "w-full max-w-wrapper grid grid-cols-2 gap-24 py-120 z-1 max-tablet:grid-cols-1 max-tablet:gap-48",
                h1 { class: "col-span-1 text-[45px]/64 text-center font-medium text-white text-left max-tablet:text-center max-tablet:text-[32px]/42",
                    "Contact "
                    span { class: "text-primary", "Us" }
                }

                div {
                    id: "contact-form",
                    class: "col-span-1 w-full flex flex-col gap-48 max-tablet:gap-32",
                    div { class: "grid grid-cols-2 gap-24 max-tablet:grid-cols-1",
                        TextInput {
                            label: tr.first_name,
                            name: "first_name",
                            placeholder: "Name",
                            oninput: move |name| ctrl.first_name.set(name),
                        }
                        TextInput {
                            label: tr.last_name,
                            name: "last_name",
                            placeholder: "Name",
                            oninput: move |name| ctrl.last_name.set(name),
                        }
                    }

                    TextInput {
                        label: tr.email,
                        name: "email",
                        placeholder: "Email",
                        oninput: move |email| ctrl.email.set(email),
                    }


                    TextInput {
                        label: tr.company,
                        name: "company_name",
                        placeholder: "Name",
                        oninput: move |company_name| ctrl.company_name.set(company_name),
                    }

                    div { class: "w-full flex flex-col items-start gap-8",
                        label { class: "text-[15px]/22 tracking-[0.5px] font-medium text-neutral-400",
                            {tr.needs}
                        }

                        Dropdown {
                            items: Need::variants(&lang),
                            selected: ctrl.selected_need(),
                            onselect: move |need| ctrl.set_need(need),
                        }
                    }

                    div { class: "w-full flex flex-col items-start gap-8",
                        label { class: "text-[15px]/22 tracking-[0.5px] font-medium text-neutral-400",
                            {tr.help}
                        }

                        textarea {
                            class: "w-full rounded-[4px] border-1 border-gray-600 px-16 py-11 flex flex-row items-center focus:outline-none focus:border-primary",
                            placeholder: tr.help_placeholder,
                            name: "help",
                            rows: 4,
                            oninput: move |evt| ctrl.help.set(evt.value()),
                        }

                    }

                    SecondaryButton {
                        onclick: move |_| async move {
                            ctrl.submit().await;
                        },
                        {tr.btn_submit}
                    }
                } // end of contact-form
            }

        }
    }
}

#[component]
pub fn TextInput(
    label: String,
    name: Option<String>,
    placeholder: Option<String>,
    oninput: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "w-full flex flex-col items-start gap-8",
            label { class: "text-[15px]/22 tracking-[0.5px] font-medium text-neutral-400",
                {label}
            }
            Input {
                placeholder,
                name,
                oninput: move |value| oninput(value),
            }
        }
    }
}
