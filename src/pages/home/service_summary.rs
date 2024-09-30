#![allow(non_snake_case)]
use crate::components::image_card::ImageCard;
use dioxus::prelude::*;

#[component]
pub(super) fn ServiceSummary() -> Element {
    let ctrl = super::controller::Controller::use_controller();
    let services = ctrl.services();

    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-[10px] p-[20px]",
            for service in services.iter() {
                ImageCard {
                    category: "{service.category}",
                    title: "{service.name}",
                    description: "{service.short_description}",
                    image: "{service.image}",
                }
            }
        }
    }
}
