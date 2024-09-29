#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Debug, Clone, Copy, Default)]
pub struct PopupService {
    pub data: Signal<Option<Element>>,
}

impl PopupService {
    pub fn init() {
        let srv = PopupService::default();
        use_context_provider(|| srv);
    }

    pub fn render(&self) -> Element {
        (self.data)().clone().unwrap_or(default())
    }

    pub fn is_opened(&self) -> bool {
        (self.data)().is_some()
    }

    pub fn open(&mut self, popup: Element) {
        tracing::debug!("open popup");
        (self.data).set(Some(popup));
    }

    pub fn close(&mut self) {
        (self.data).set(None);
    }

    pub fn use_popup_service() -> PopupService {
        use_context()
    }
}

#[component]
pub fn default() -> Element {
    rsx! {}
}
