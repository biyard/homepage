#![allow(non_snake_case)]
use dagit_roadmap::DagitRoadmap;
use dioxus::prelude::*;
use feeds::Feeds;
use following::Following;
use members::Members;
use service_summary::ServiceSummary;
use slogan_section::SloganSection;
use usecases::UseCases;

pub mod controller;
pub mod dagit_roadmap;
pub mod feeds;
pub mod following;
pub mod members;
pub mod service_summary;
pub mod slogan_section;
pub mod usecases;

#[component]
pub fn Home() -> Element {
    let ctrl = controller::Controller::init()?;

    rsx! {
        if !ctrl.is_loaded() {
            div {
                class: "flex w-full flex-col items-center justify-center",
                "Loading..."
            }
        } else {
            div {
                class: "flex w-full flex-col items-center justify-center",
                SloganSection {}
                ServiceSummary {}
                DagitRoadmap {}
                UseCases {}
                Feeds {}
                Members {}
                Following {}
            }
        }
    }
}
