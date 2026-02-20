use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use surrealdb::types::{ RecordId, SurrealValue };

#[derive(Clone, Debug, Deserialize, Serialize, SurrealValue)]
pub(crate) struct Property {
    pub(crate) id: Option<RecordId>,
    pub(crate) bathrooms: u32,
    pub(crate) bedrooms: u32,
    pub(crate) created_at: Option<DateTime<Utc>>,
    pub(crate) currency: String,
    pub(crate) description: String,
    pub(crate) estado: RecordId,
    pub(crate) featured: bool,
    pub(crate) images: Option<Vec<RecordId>>,
    pub(crate) location: String,
    pub(crate) operation: String,
    pub(crate) price: f64,
    pub(crate) surface_m2: f64,
    pub(crate) title: String,
    pub(crate) property_type: String,
}
