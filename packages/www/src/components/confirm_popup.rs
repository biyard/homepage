use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn ConfirmPopup(title: String, description: String, btn_label: String) -> Element {
    let mut popup: PopupService = use_context();

    rsx! {
        div { class: "max-w-358 w-full flex flex-col items-center gap-35",
            div { class: "col gap-24 items-center",
                h1 { class: "subhead-bold-20 whitespace-pre-line", {title} }
                p { class: "descript-extralight-15 text-gray-300 whitespace-pre-line",
                    {description}
                }
            }

            button {
                class: "btn-primary !w-full !py-13",
                onclick: move |_| {
                    popup.close();
                },
                {btn_label}
            }
        }
    }
}
