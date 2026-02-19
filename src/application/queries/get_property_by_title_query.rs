use axum::{ extract::Path, Json, http::StatusCode, response::IntoResponse };
use log::debug;
use serde_json::json;

use crate::data::repositories::property_repository::PropertyRepository;

use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PropertyByTitleQuery {
    pub title: String,
}

pub async fn get_property_by_title_query(Query(
    params,
): Query<PropertyByTitleQuery>) -> Result<
    impl IntoResponse,
    (StatusCode, Json<serde_json::Value>)
> {
    let repository = PropertyRepository::new();
    debug!("Fetching Property with title = {}", params.title);

    match repository.get_by_title(params.title.clone()).await {
        Ok(property) => Ok((StatusCode::OK, Json(property))),
        Err(_) =>
            Err((
                StatusCode::NOT_FOUND,
                Json(
                    json!({
                "error": format!("Property with title {} not found", params.title)
            })
                ),
            )),
    }
}
