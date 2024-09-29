#![allow(non_snake_case)]
use crate::{
    components::feature_button::{FeatureButton, ProgressFeature},
    models::highlight_service::FeatureStatus,
    prelude::*,
};
use dioxus::prelude::*;

#[component]
pub(super) fn DagitRoadmap() -> Element {
    let ctrl = super::controller::Controller::use_controller();
    let service = ctrl.highlight_service();

    rsx! {
        div {
            class: "w-full flex flex-col items-center justify-center gap-[50px] py-[100px]",
            div {
                class: "max-w-[1440px] 2xl:w-[1440px] m-auto flex flex-col gap-[10px]",
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
                style: "scrollbar-width: none;",
                class: "flex flex-row gap-[30px] w-full justify-center overflow-x-auto px-[5px]",
                FeatureButton {
                    completed: true,
                    no_features: service.past_features.len(),
                    text: "Past features",
                }
                for working_feature in service.working_features.iter() {
                    ProgressFeature {
                        code_name: working_feature.code_name.clone(),
                        goal: working_feature.goal.clone(),
                        text: "Features",
                        no_features: working_feature.features.len(),
                        completed_date: match &working_feature.status {
                            FeatureStatus::Completed(date) => Some(date.clone()),
                            _ => None,
                        },
                    }
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
