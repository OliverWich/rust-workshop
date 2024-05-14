use actix_web::{get, post, web, HttpResponse, Responder};
use crate::db::BooksDb;


#[get("/hello")]
pub async fn hello(books_db: web::Data<BooksDb>) -> impl Responder {
    let books = books_db.lock_and_unwrap();
    let num_books = books.len();
    HttpResponse::Ok().body(format!("Number of books: {}", num_books))
}

#[get("/books")]
pub async fn get_books(books_db: web::Data<BooksDb>) -> impl Responder {
    let books = books_db.lock_and_unwrap();
    
    let books_vec: Vec<crate::models::Book> = books.values().cloned().collect();

    HttpResponse::Ok().json(books_vec)}

#[get("/books/{id}")]
pub async fn get_book_by_id(books_db: web::Data<BooksDb>, id: web::Path<u32>) -> impl Responder {
    let books = books_db.lock_and_unwrap();
    let book = books.get(&id).unwrap();
    let book = serde_json::to_string(&book).unwrap();
    HttpResponse::Ok().body(book)
}
