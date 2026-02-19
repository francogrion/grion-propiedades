use axum::{ Router, routing::get };

use crate::application::queries::{
    get_all_properties_query::get_all_properties_query,
    get_property_by_id_query::get_property_by_id_query,
    get_property_by_title_query::get_property_by_title_query,
};

use super::health_checker_handler;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/properties", get(get_all_properties_query))
        .route("/api/properties/{id}", get(get_property_by_id_query))
        .route("/api/properties/find", get(get_property_by_title_query))
}
