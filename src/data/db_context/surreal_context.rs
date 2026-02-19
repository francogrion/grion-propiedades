use log::debug;
use once_cell::sync::Lazy;
use surrealdb::{ Result, Surreal, engine::any::Any, opt::auth::Root };

pub static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

pub async fn connect_db() -> Result<()> {
    // URL de tu instancia en Surreal Cloud
    let url =
        "wss://grion-propiedad-06e737tn75r1j97flkva6ns3dg.aws-euw1.surreal.cloud";

    // Conectar usando el engine Any (acepta ws, wss, http, https)
    DB.connect(url).await?;

    // Autenticación
    DB.signin(Root {
        username: "gp-admin".to_string(),
        password: "GrionPropiedadesAdmin11".to_string(),
    }).await?;

    // Seleccionar namespace y database
    DB.use_ns("gp-dev").use_db("main").await?;

    debug!("Connected to SurrealDB 🗄️");

    Ok(())
}
