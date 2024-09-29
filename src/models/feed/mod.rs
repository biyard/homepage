use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    pub title: String,
    pub published: String,
    pub external_link: String,
}
