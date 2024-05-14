use std::collections::HashMap;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use crate::routes::{get_book_by_id, get_books, hello};

mod models;
mod db;
mod routes;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let books_db = web::Data::new(Arc::new(Mutex::new(HashMap::<u32, models::Book>::new())));

    HttpServer::new(move || {
        App::new()
            .app_data(books_db.clone())
            .service(hello)
            .service(get_books)
            .service(get_book_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}