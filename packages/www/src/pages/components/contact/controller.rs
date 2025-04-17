use bdk::prelude::{dioxus_popup::PopupService, *};
use common::*;

use crate::{ConfirmPopup, config};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub first_name: Signal<String>,
    pub last_name: Signal<String>,
    pub email: Signal<String>,
    pub company_name: Signal<String>,
    pub needs: Signal<Need>,
    pub help: Signal<String>,
    pub selected_need: Signal<usize>,
    pub popup: PopupService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            first_name: use_signal(|| String::new()),
            last_name: use_signal(|| String::new()),
            email: use_signal(|| String::new()),
            company_name: use_signal(|| String::new()),
            needs: use_signal(|| Need::GeneralInquiry),
            help: use_signal(|| String::new()),
            selected_need: use_signal(|| 0),
            popup: use_context(),
        };

        Ok(ctrl)
    }

    pub fn set_need(&mut self, need: usize) {
        self.needs.set(Need::VARIANTS[need]);
    }

    pub async fn submit(&mut self) {
        match Contact::get_client(config::get().api_endpoint)
            .submit(
                self.last_name(),
                self.first_name(),
                self.email(),
                self.company_name(),
                self.needs(),
                self.help(),
            )
            .await
        {
            Ok(res) => {
                tracing::debug!("Contact submit response: {:?}", res);
                self.popup
                    .open(rsx! {
                        ConfirmPopup {
                            title: "Thank you! \nYour message has been received",
                            description: "We appreciate your message and will get \nback to you shortly.",
                            btn_label: "Confirm",
                        }
                    });
            }
            Err(err) => {
                btracing::e!(self.lang, err);
            }
        }
    }
}
