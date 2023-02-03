use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::io::Result;

mod extractors;
mod scopes;

struct AppState {
    secret: String,
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                secret: String::from("#Easy#Commerce#SecretKey#"),
            }))
            .service(scopes::user::user_scope())
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
