mod models;
mod config;
mod errors;
mod db;
mod handlers;

use actix_web::{ App, HttpServer, web, middleware::Logger };
use dotenvy::dotenv;
use handlers::{add_user, get_users, get_user, get_items, add_item, get_item_types};
use ::config::Config;
use deadpool_postgres::Runtime;
use env_logger::Env;

use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use actix_cors::Cors;

use crate::config::ExampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let config_ = Config::builder().add_source(::config::Environment::default().separator("__")).build().unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let connector = MakeTlsConnector::new(TlsConnector::new().unwrap());

    let pool = config.pg.create_pool(Some(Runtime::Tokio1), connector).unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    HttpServer::new(move || {
        let cors = Cors::default().allowed_origin("http://localhost:8080").allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
        ])
        .allowed_header(actix_web::http::header::CONTENT_TYPE)
        .expose_headers(&[actix_web::http::header::CONTENT_DISPOSITION])
        .supports_credentials();
        App::new().wrap(cors).wrap(Logger::default()).wrap(Logger::new("%a %{User-Agent}i")).app_data(web::Data::new(pool.clone()))
        .service(add_user)
        .service(get_user)
        .service(get_users)
        .service(get_items)
        .service(add_item)
        .service(get_item_types)
    })
    .workers(12)
    .bind(config.server_addr.clone())?
    .run()
    .await
}
