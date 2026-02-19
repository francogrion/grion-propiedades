use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Image {
    pub(crate) url: String,
}
