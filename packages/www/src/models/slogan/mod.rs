use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Slogan {
    pub title: String,
    pub description: String,
}

impl Slogan {
    pub fn headlines(&self) -> Vec<String> {
        self.title
            .clone()
            .split('\n')
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }
}
