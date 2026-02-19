use axum::http::{
    HeaderValue,
    Method,
    header::{ ACCEPT, AUTHORIZATION, CONTENT_TYPE },
};

use grion_propiedades::{
    api::router::create_router,
    data::db_context::surreal_context::connect_db,
};

use log::info;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    env_logger::init();
    connect_db().await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

    info!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
