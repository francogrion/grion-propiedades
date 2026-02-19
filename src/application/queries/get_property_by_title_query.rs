use axum::{ extract::Path, Json, http::StatusCode, response::IntoResponse };
use log::debug;
use serde_json::json;

use crate::data::repositories::property_repository::PropertyRepository;

pub async fn get_property_by_title_query(Path(title): Path<String>) -> Result<
    impl IntoResponse,
    (StatusCode, Json<serde_json::Value>)
> {
    let repository = PropertyRepository::new();
    debug!("Fetching Property with title = {}", title);

    match repository.get_by_title(title.clone()).await {
        Ok(property) => Ok((StatusCode::OK, Json(property))),
        Err(_) =>
            Err((
                StatusCode::NOT_FOUND,
                Json(
                    json!({
                "error": format!("Property with title {} not found", title)
            })
                ),
            )),
    }
}
