#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::env;

use actix_files as fs;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

use crate::database::DbPool;

mod database;
mod schema;
mod routes;

pub struct State {
    pub db: DbPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    dotenv().ok();

    info!("Starting yiffOS Packages server");

    let db_pool = database::connect();

    HttpServer::new(move || {
        App::new()
            // Database state
            .app_data(State { db: db_pool.clone() })

            // Static files
            .service(fs::Files::new("/css", "./templates/css"))
            .service(fs::Files::new("/js", "./templates/js"))

            // Index
            .service(routes::index::index)

            // Package view
            .service(routes::package_view::package_view)
    })
        .workers(env::var("WORKERS").unwrap_or_else(|_| "1".to_string()).parse().unwrap())
        .bind(("0.0.0.0", 6969))?
        .run()
        .await
}