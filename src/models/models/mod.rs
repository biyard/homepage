use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub role: String,
    pub image: String,
    pub description: String,
    pub email: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub web: Option<String>,
}
