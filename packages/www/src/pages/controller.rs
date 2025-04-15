use bdk::prelude::*;

#[derive(Clone, Copy, DioxusController)]
pub struct LayoutController {
    #[allow(dead_code)]
    pub lang: Language,
    pub expanded_menu: Signal<bool>,
}

impl LayoutController {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            expanded_menu: use_signal(|| false),
        };

        Ok(ctrl)
    }

    pub fn toggle_menu(&mut self) {
        self.expanded_menu.set(!self.expanded_menu());
    }
}
