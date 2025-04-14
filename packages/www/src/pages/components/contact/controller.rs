use bdk::prelude::*;
use common::*;

use crate::config;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub first_name: Signal<String>,
    pub last_name: Signal<String>,
    pub email: Signal<String>,
    pub company_name: Signal<String>,
    pub needs: Signal<Need>,
    pub help: Signal<String>,
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
        };

        Ok(ctrl)
    }

    pub fn set_need(&mut self, need: String) {
        self.needs
            .set(need.parse::<Need>().unwrap_or(Need::GeneralInquiry));
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
                btracing::i!(self.lang, Info::ContactSubmit);
            }
            Err(err) => {
                btracing::e!(self.lang, err);
            }
        }
    }
}
