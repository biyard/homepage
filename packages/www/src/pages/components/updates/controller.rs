use bdk::prelude::*;
use common::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub email: Signal<String>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            email: use_signal(|| String::new()),
        };

        Ok(ctrl)
    }

    pub async fn submit(&self) {
        match Update::get_client(crate::config::get().api_endpoint)
            .submit(self.email())
            .await
        {
            Ok(res) => {
                tracing::debug!("Update submit response: {:?}", res);
                btracing::i!(self.lang, Info::KeepUpdateSubmit);
            }
            Err(err) => {
                btracing::e!(self.lang, err);
            }
        };
    }
}
