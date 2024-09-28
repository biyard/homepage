use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub category: String,
    pub name: String,
    pub short_description: String,
    pub image: String,
}
