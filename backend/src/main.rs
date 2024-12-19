// main.rs
// attaches the files in the package to the `main.rs` file
mod connect;
mod handlers;
mod models;
mod schema;
// import the functionality for starting the server
use actix_web::{web, App, HttpServer};
// import the handler function
use crate::handlers::{create_spell, delete_spell, read_spells, update_spell};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/create_spell", web::post().to(create_spell))
            .route("/read_spells", web::get().to(read_spells))
            .route("/update_spell/{id}", web::put().to(update_spell))
            .route("/delete_spell/{id}", web::delete().to(delete_spell))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
