use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Estado {
    pub nombre: String,
    pub descripcion: String,
}
