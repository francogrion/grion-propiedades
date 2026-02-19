use serde::{ Deserialize, Serialize };
use surrealdb::types::SurrealValue;

#[derive(Clone, Debug, Deserialize, Serialize, SurrealValue)]
pub(crate) struct Image {
    pub(crate) url: String,
}
