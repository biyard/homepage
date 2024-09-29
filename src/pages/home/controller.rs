#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    apis::home::get_home,
    models::{
        feed::Feed, highlight_service::HighlightService, models::Member, service::Service,
        slogan::Slogan,
    },
};

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Controller {
    pub slogan: Signal<Option<Slogan>>,
    pub loaded: Signal<bool>,
    pub services: Signal<Option<Vec<Service>>>,
    pub highlight_service: Signal<Option<HighlightService>>,
    pub feeds: Signal<Option<Vec<Feed>>>,
    pub members: Signal<Option<Vec<Member>>>,
}

impl Controller {
    pub fn init() -> Self {
        let mut ctrl = Self::default();
        use_context_provider(|| ctrl);

        let _ = use_server_future(move || async move {
            match get_home().await {
                Ok(result) => {
                    ctrl.slogan.set(Some(result.slogan));
                    ctrl.services.set(Some(result.services));
                    ctrl.highlight_service.set(Some(result.highlight_service));
                    ctrl.feeds.set(Some(result.feeds));
                    ctrl.members.set(Some(result.members));
                }
                Err(err) => {
                    tracing::error!("Failed to get home data: {:?}", err);
                }
            };
            tracing::debug!("Home data loaded");

            ctrl.loaded.set(true);
        });

        ctrl
    }

    pub fn use_controller() -> Self {
        use_context()
    }

    pub fn is_loaded(&self) -> bool {
        (self.loaded)()
    }

    pub fn slogan(&self) -> Slogan {
        (self.slogan)().unwrap_or_default()
    }

    pub fn services(&self) -> Vec<Service> {
        (self.services)().unwrap_or_default()
    }

    pub fn highlight_service(&self) -> HighlightService {
        (self.highlight_service)().unwrap_or_default()
    }

    pub fn feeds(&self) -> Vec<Feed> {
        (self.feeds)().unwrap_or_default()
    }

    pub fn members(&self) -> Vec<Member> {
        (self.members)().unwrap_or_default()
    }
}
