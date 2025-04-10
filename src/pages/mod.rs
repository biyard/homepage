mod components;
mod controller;
mod i18n;
mod layout;
mod page;

pub(self) use components::*;
pub(self) use controller::*;
pub(self) use i18n::*;

pub use layout::*;
pub use page::*;

mod _routes;

pub use _routes::*;
