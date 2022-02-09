use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Create connection pool
    let manager = ConnectionManager::<PgConnection>::new(
        "host=0.0.0.0 user=postgres password=password dbname=measurements port=5432",
    );
    let pool = r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create database pool");

    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(
                web::scope("/api/v1")
                    .app_data(web::JsonConfig::default().limit(1024))
                    .configure(logging_api::api::config),
            )
    })
    .bind("127.0.0.1:3080")?
    .run()
    .await
}
