use crate::scopes::{store::store_scope, user::user_scope};
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use std::io::Result;

mod config;
mod extractors;
mod models;
mod schema;
mod scopes;

struct AppState {
    secret: String,
    pool: DbPool,
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                secret: String::from("#Easy#Commerce#SecretKey#"),
                pool: pool.clone(),
            }))
            .wrap(middleware::Logger::default())
            .service(user_scope())
            .service(store_scope())
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
