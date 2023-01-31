use actix_web::{web, App, HttpServer};
use std::io::Result;

mod extractors;
mod scopes;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(String::from("secret")))
            .service(scopes::user::user_scope())
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
