use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum FeatureStatus {
    InProgress,
    Completed(String),
    NotStarted,
}

impl Default for FeatureStatus {
    fn default() -> Self {
        Self::NotStarted
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Feature {
    pub title: String,
    pub description: String,
    pub status: FeatureStatus,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkingFeature {
    pub code_name: String,
    pub goal: String,
    pub features: Vec<Feature>,
    pub status: FeatureStatus,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct HighlightService {
    pub name: String,
    pub description: String,
    pub external_link: String,
    pub past_features: Vec<Feature>,
    pub working_features: Vec<WorkingFeature>,
    pub future_features: Vec<Feature>,
}
