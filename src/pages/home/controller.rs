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
    pub slogan: Signal<Slogan>,
    pub loaded: Signal<bool>,
    pub services: Signal<Vec<Service>>,
    pub highlight_service: Signal<HighlightService>,
    pub feeds: Signal<Vec<Feed>>,
    pub members: Signal<Vec<Member>>,
}

impl Controller {
    pub fn init() -> Result<Self, RenderError> {
        let mut ctrl = Self::default();
        use_context_provider(|| ctrl);

        let result = get_home();
        ctrl.slogan.set(result.slogan);
        ctrl.services.set(result.services);
        ctrl.highlight_service.set(result.highlight_service);
        ctrl.feeds.set(result.feeds);
        ctrl.members.set(result.members);

        tracing::debug!("Home data loaded");

        Ok(ctrl)
    }

    pub fn use_controller() -> Self {
        use_context()
    }

    pub fn is_loaded(&self) -> bool {
        (self.loaded)()
    }

    pub fn slogan(&self) -> Slogan {
        (self.slogan)()
    }

    pub fn services(&self) -> Vec<Service> {
        (self.services)()
    }

    pub fn highlight_service(&self) -> HighlightService {
        (self.highlight_service)()
    }

    pub fn feeds(&self) -> Vec<Feed> {
        (self.feeds)()
    }

    pub fn members(&self) -> Vec<Member> {
        (self.members)()
    }
}
