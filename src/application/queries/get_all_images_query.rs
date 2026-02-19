use axum::{ Json, response::IntoResponse };

use crate::{
    data::repositories::image_repository::ImageRepository,
    domain::models::image::Image,
};

pub async fn get_all_images_query() -> impl IntoResponse {
    let repository = ImageRepository::new();

    let mut images: Vec<Image> = Vec::new();
    if let Ok(result) = repository.get_all().await {
        images = result;
    }

    let json_response =
        serde_json::json!({
        "status": "success",
        "results": images.len(),
        "images": images,
    });

    Json(json_response)
}
