use axum::{ Json, response::IntoResponse };

use crate::{
    data::repositories::property_repository::PropertyRepository,
    domain::models::property::Property,
};

pub async fn get_all_properties_query() -> impl IntoResponse {
    let repository = PropertyRepository::new();

    let mut properties: Vec<Property> = Vec::new();
    if let Ok(result) = repository.get_all().await {
        properties = result;
    }

    let json_response =
        serde_json::json!({
        "status": "success",
        "results": properties.len(),
        "properties": properties,
    });

    Json(json_response)
}
