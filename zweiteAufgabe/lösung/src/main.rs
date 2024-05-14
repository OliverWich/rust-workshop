use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod models;
mod routes;

use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let books_db = web::Data::new(Arc::new(Mutex::new(HashMap::<u32, models::Book>::new())));

    HttpServer::new(move || {
        App::new()
            .app_data(books_db.clone())
            .service(get_books)
            .service(create_book)
            .service(get_book_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}