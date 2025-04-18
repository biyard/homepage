use bdk::prelude::{dioxus_popup::PopupService, *};
use common::*;

use crate::ConfirmPopup;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub email: Signal<String>,
    pub popup: PopupService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            email: use_signal(|| String::new()),
            popup: use_context(),
        };

        Ok(ctrl)
    }

    pub async fn submit(&mut self) {
        match Update::get_client(crate::config::get().api_endpoint)
            .submit(self.email())
            .await
        {
            Ok(res) => {
                tracing::debug!("Update submit response: {:?}", res);
                self.popup
                    .open(rsx! {
                        ConfirmPopup {
                            title: "Thank you for subscribing!",
                            description: "You’ll now receive updates, news, and helpful \ninsights regularly. Stay tuned!",
                            btn_label: "Confirm",
                        }
                    });
            }
            Err(err) => {
                btracing::e!(self.lang, err);
            }
        };
    }
}
