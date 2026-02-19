use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use surrealdb::types::{ RecordId, SurrealValue };

use super::{ estado::Estado, image::Image };

#[derive(Clone, Debug, Deserialize, Serialize, SurrealValue)]
pub(crate) struct Property {
    pub(crate) id: Option<RecordId>,
    pub(crate) bathrooms: u32,
    pub(crate) bedrooms: u32,
    pub(crate) created_at: Option<DateTime<Utc>>,
    pub(crate) currency: String,
    pub(crate) description: String,
    pub(crate) estado: Estado,
    pub(crate) featured: bool,
    pub(crate) image: Option<Image>,
    pub(crate) images: Vec<Image>,
    pub(crate) location: String,
    pub(crate) operation: String,
    pub(crate) price: f64,
    pub(crate) surface_m2: f64,
    pub(crate) title: String,
    pub(crate) type_: String,
}
