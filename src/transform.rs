use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform(pub Vec<String>);

impl Transform {
    pub fn new(transform: String) -> Self {
        Self(vec![transform])
    }
}
