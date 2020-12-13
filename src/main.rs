#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use std::io;

use actix_files::{Files, NamedFile};
use actix_web::{
    App, http, HttpResponse, HttpServer, web,
};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use sqlx::SqlitePool;

use config::CONFIG;
use actix_cors::Cors;

mod common;
mod auth;
mod db_utils;
mod config;
mod errors;
mod middleware;
mod runs;
mod shoes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool = SqlitePool::builder().max_size(1).build(&format!("sqlite:{}", &CONFIG.database_file))
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .app_data(
                web::JsonConfig::default()
                    // register error_handler for JSON extractors.
                    .error_handler(errors::json_error_handler),
            )
            .service(
                web::scope("/api")
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allow_any_header()
                            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                            .supports_credentials()
                            .max_age(3600),
                    )
                    .configure(auth::init)
                    .configure(shoes::init)
                    .configure(runs::init)
            )
            .service(
                web::scope("").default_service(
                    Files::new("", &CONFIG.static_webapp_folder)
                        .index_file("index.html")
                        .use_last_modified(true)
                        .default_handler(|req: ServiceRequest| {
                            // for SPA like Angular
                            let (http_req, _payload) = req.into_parts();
                            async {
                                if http_req.method() == http::Method::GET {
                                    let response = NamedFile::open("webapp_build/index.html")?
                                        .into_response(&http_req)?;
                                    Ok(ServiceResponse::new(http_req, response))
                                } else {
                                    Ok(ServiceResponse::new(
                                        http_req,
                                        HttpResponse::NotFound().body("Not found :("),
                                    ))
                                }
                            }
                        }),
                ),
            )
    })
        .workers(4)
        .bind(&CONFIG.server)?
        .run()
        .await
}
