use grion_propiedades::data::db_context::surreal_context::connect_db;

#[tokio::main]
async fn main() {
    env_logger::init();
    connect_db().await.unwrap();
    println!("Hello, world!");
}
