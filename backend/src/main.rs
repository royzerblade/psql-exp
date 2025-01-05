mod connect;
mod handlers;
mod models;
mod schema;
use crate::handlers::{create_spell, delete_spell, read_spells, update_spell};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().send_wildcard();
        App::new()
            .wrap(cors)
            .route("/create_spell", web::post().to(create_spell))
            .route("/read_spells", web::get().to(read_spells))
            .route("/update_spell/{id}", web::put().to(update_spell))
            .route("/delete_spell/{id}", web::delete().to(delete_spell))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
