use serde::{ Deserialize, Serialize };
use surrealdb::types::{ RecordId, SurrealValue };

#[derive(Clone, Debug, Deserialize, Serialize, SurrealValue)]
pub(crate) struct Image {
    pub(crate) id: Option<RecordId>,
    pub(crate) url: String,
}
