#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

use crate::models::{
    highlight_service::{Feature, FeatureStatus, HighlightService, WorkingFeature},
    service::Service,
    slogan::Slogan,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetHomeResponse {
    pub slogan: Slogan,
    pub services: Vec<Service>,
    pub highlight_service: HighlightService,
}

#[server(endpoint = "/home", input = GetUrl, output = Json)]
pub async fn get_home() -> Result<GetHomeResponse, ServerFnError> {
    tracing::debug!("GET /home");

    Ok(GetHomeResponse {
        slogan: Slogan {
            title: "SLOGAN\nDESCRIPTION".to_string(),
            description: "blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah".to_string(),
        },
        services: vec![
            Service {
                category: "Art".to_string(),
                name: "d.AGIT".to_string(),
                short_description: "Service 1 description".to_string(),
                image: "https://cdn.pixabay.com/photo/2015/11/03/08/56/service-1019821_640.jpg".to_string(),
            },
            Service {
                category: "Art".to_string(),
                name: "d.AGIT".to_string(),
                short_description: "Service 1 description".to_string(),
                image: "https://cdn.pixabay.com/photo/2015/11/03/08/56/service-1019821_640.jpg".to_string(),
            },
            Service {
                category: "Art".to_string(),
                name: "d.AGIT".to_string(),
                short_description: "Service 1 description".to_string(),
                image: "https://cdn.pixabay.com/photo/2015/11/03/08/56/service-1019821_640.jpg".to_string(),
            },
        ],
        highlight_service: HighlightService {
            name: "d.AGIT".to_string(),
            description: "Service 1 description".to_string(),
            external_link: "https://dagit.club".to_string(),
            past_features: vec![
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::Completed,
                },
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::Completed,
                },
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::Completed,
                },
            ],
            future_features: vec![
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::NotStarted,
                },
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::NotStarted,
                },
                Feature {
                    title: "Feature 1".to_string(),
                    description: "Feature 1 description".to_string(),
                    status: FeatureStatus::NotStarted,
                },
            ],
            working_features: vec![
                WorkingFeature {
                    code_name: "Alpha 1".to_string(),
                    goal: "Alpha 1 aims to blah blah".to_string(),
                    status: FeatureStatus::Completed,
                    features: vec![
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::Completed,
                        },
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::Completed,
                        },
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::Completed,
                        },
                    ],
                },
                WorkingFeature {
                    code_name: "Alpha 2".to_string(),
                    goal: "Alpha 2 aims to blah blah".to_string(),
                    status: FeatureStatus::InProgress,
                    features: vec![
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::Completed,
                        },
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::InProgress,
                        },
                        Feature {
                            title: "Feature 1".to_string(),
                            description: "Feature 1 description".to_string(),
                            status: FeatureStatus::InProgress,
                        },
                    ],
                }
            ],
        },
    })
}
