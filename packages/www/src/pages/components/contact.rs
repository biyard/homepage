mod controller;

use bdk::prelude::*;

use crate::SecondaryButton;

#[component]
pub fn Contact(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    lang: Language,
) -> Element {
    let mut ctrl = controller::Controller::new(lang)?;
    let tr: ContactTranslate = translate(&lang);

    rsx! {
        section { id: "contact", class: "w-full relative",
            div { class: "absolute top-[1/2] left-[2/3] h-1328 w-1328 bg-purple-blur/40 blur-[500px]" }
            div { class: "w-full max-w-wrapper grid grid-cols-2 gap-24 py-120 z-1",
                h1 { class: "col-span-1 text-[45px]/64 text-center font-medium text-white",
                    "Contact "
                    span { class: "text-primary", "Us" }
                }

                div {
                    id: "contact-form",
                    class: "col-span-1 w-full flex flex-col gap-48",
                    div { class: "grid grid-cols-2 gap-24",
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

                    TextInput {
                        label: tr.needs,
                        name: "needs",
                        oninput: move |need| ctrl.set_need(need),
                    }

                    TextInput {
                        label: tr.help,
                        name: "help",
                        oninput: move |msg| ctrl.help.set(msg),
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
            input {
                class: "w-full h-44 rounded-[4px] border-b-1 border-gray-600 px-20 flex flex-row items-center focus:border-b-1 active:border-primary",
                placeholder,
                name,
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

    last_name: {
        ko: "성",
        en: "Last name",
    },

    email: {
        ko: "이메일",
        en: "Email",
    },

    company: {
        ko: "회사명",
        en: "Company name",
    },

    needs: {
        ko: "어떤 도움이 필요하신가요?",
        en: "Which topic best fit your needs?",
    },

    help: {
        ko: "어떻게 도와드릴까요?",
        en: "How can we help?",
    },

    btn_submit: {
        ko: "제출하기",
        en: "Submit",
    },
}
