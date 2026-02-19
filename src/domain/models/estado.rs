use serde::{ Deserialize, Serialize };
use surrealdb::types::SurrealValue;

#[derive(Clone, Debug, Deserialize, Serialize, SurrealValue)]
pub(crate) struct Estado {
    pub nombre: String,
    pub descripcion: String,
}
