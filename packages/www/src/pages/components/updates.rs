mod controller;

use bdk::prelude::*;

#[component]
pub fn Updates(lang: Language) -> Element {
    let mut ctrl = controller::Controller::new(lang)?;
    let tr: UpdatesTranslate = translate(&lang);

    rsx! {
        div { class: "w-full px-28 flex flex-col items-center",
            div {
                id: "get-updates",
                class: "w-full max-w-wrapper py-40 px-118 bg-black/50 border border-gray-800 backdrop-blur-[5px] rounded-2xl flex flex-col gap-24 max-tablet:px-16 max-tablet:py-24",
                h2 { class: "text-[28px]/36 font-semibold whitespace-pre-line max-tablet:text-xl/34 max-tablet:whitespace-normal",
                    {tr.title}
                }
                div { class: "w-full flex flex-row gap-24 max-tablet:flex-col max-tablet:gap-48 max-tablet:items-center",
                    input {
                        class: "w-full h-44 border-b border-b-gray-600 rounded-sm px-20 flex flex-col justify-center focus:outline-none focus:border-b-primary placeholder:text-gray-600",
                        name: "email",
                        placeholder: tr.placeholder,
                        oninput: move |evt| ctrl.email.set(evt.value()),
                    }
                    button {
                        class: "btn-secondary rounded-full",
                        onclick: move |_| async move {
                            ctrl.submit().await;
                        },
                        {tr.btn_submit}
                    }
                }
            }
        }
    }
}

translate! {
    UpdatesTranslate;

    title: {
        ko: "Stay in the loop with our latest tech \nbreakthroughs and service updates!",
        en: "Stay in the loop with our latest tech \nbreakthroughs and service updates!",
    },

    placeholder: {
        ko: "이메일을 입력하세요",
        en: "Please enter your email address.",
    },

    btn_submit: {
        ko: "구독하기",
        en: "Get Updates",
    },
}
