#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    apis::home::{get_home, GetHomeResponse},
    models::{
        feed::Feed, highlight_service::HighlightService, models::Member, service::Service,
        slogan::Slogan,
    },
};

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Controller {
    // pub slogan: Signal<Option<Slogan>>,
    pub loaded: Signal<bool>,
    // pub services: Signal<Vec<Service>>,
    // pub highlight_service: Signal<Option<HighlightService>>,
    // pub feeds: Signal<Vec<Feed>>,
    // pub members: Signal<Vec<Member>>,
    pub result: Signal<Option<GetHomeResponse>>,
}

impl Controller {
    pub fn init() -> Result<Self, RenderError> {
        let mut ctrl = Self::default();
        use_context_provider(|| ctrl);

        let result = get_home();
        ctrl.result.set(Some(result.clone()));
        ctrl.loaded.set(true);

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
        (self.result)().unwrap_or_default().slogan
    }

    pub fn services(&self) -> Vec<Service> {
        (self.result)().unwrap_or_default().services
    }

    pub fn highlight_service(&self) -> HighlightService {
        (self.result)().unwrap_or_default().highlight_service
    }

    pub fn feeds(&self) -> Vec<Feed> {
        (self.result)().unwrap_or_default().feeds
    }

    pub fn members(&self) -> Vec<Member> {
        (self.result)().unwrap_or_default().members
    }
}
