#![allow(non_snake_case)]
use crate::{components::feature_button::FeatureButton, prelude::*};
use dioxus::prelude::*;

#[component]
pub(super) fn DagitRoadmap() -> Element {
    let ctrl = super::controller::Controller::use_controller();
    let service = ctrl.highlight_service();

    rsx! {
        div {
            class: "w-full flex flex-col items-center justify-center",
            div {
                class: "max-w-[1440px] m-auto flex flex-col gap-[50px]",
                div {
                    class: "flex flex-row justify-start items-center gap-[10px]",
                    div {
                        class: "text-[42px] font-black",
                        "{service.name}"
                    }
                    OutlinedButton {
                        onclick: move |_| {},
                        shape: ButtonShape::Circle,
                        icons::external_link {}
                    }
                }
                div {
                    class: "text-[16px] max-w-[582px]",
                    "{service.description}"
                }
            }

            div {
                class: "flex flex-row gap-[30px]",
                FeatureButton {
                    completed: true,
                    no_features: service.past_features.len(),
                    text: "Past features",
                }
                FeatureButton {
                    completed: false,
                    no_features: service.future_features.len(),
                    text: "Future features",
                }

            }

        }
    }
}
