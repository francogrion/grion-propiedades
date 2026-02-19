use log::debug;
use once_cell::sync::Lazy;
use surrealdb::engine::any;
use surrealdb::{ Result, Surreal, engine::remote::ws::{ Client }, opt::auth::Root };

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn connect_db() -> Result<()> {
    // URL de tu instancia en Surreal Cloud
    let url = "wss://grion-propiedad-06e737tn75r1j97flkva6ns3dg.aws-euw1.surreal.cloud";
    let db = any::connect(url).await?;

    let _ = db.signin(Root {
        username: "gp-admin".to_string(),
        password: "GrionPropiedadesAdmin11".to_string(),
    }).await;
    let _ = db.use_ns("gp-dev").use_db("main").await?;
    debug!("Connected to SurrealDB 🗄️");
    Ok(())
}
