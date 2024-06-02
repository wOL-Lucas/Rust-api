mod schema;
mod model;
mod services;
use env_logger::Env;
use actix_web::{
    web,
    App,
    HttpServer,
};

use dotenv::dotenv;
use sqlx::{
    Postgres,
    Pool,
    postgres::PgPoolOptions,
};


pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at port 8080");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new().max_connections(10).connect(&database_url).await {
        Ok(pool) => {
            println!("db connection resolved");
            pool
        }
        Err(e) => {
            println!("db connection failed: {:?}", e);
            std::process::exit(1);
        }
    };
    
    let _ = HttpServer::new( move || {
        App::new()
        .app_data(web::Data::new(AppState {
            db: pool.clone()
        }))
        .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run().await;

    return Ok(());
}
