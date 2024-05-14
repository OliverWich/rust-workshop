use crate::models::{Book, ErrorResponse, NewBook};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type BooksDb = Arc<Mutex<HashMap<u32, Book>>>;

#[get("/books")]
pub async fn get_books(books_db: web::Data<BooksDb>) -> impl Responder {
    let books = books_db.lock().unwrap();
    let books_vec : Vec<Book> = books.values().cloned().collect();
    HttpResponse::Ok().json(books_vec)
}

#[post("/books")]
pub async fn create_book(
    books_db: web::Data<BooksDb>,
    new_book: web::Json<NewBook>,
) -> impl Responder {
    let mut books = books_db.lock().unwrap();
    let new_id = books.len() as u32 + 1;
    let book = Book {
        id: new_id,
        title: new_book.title.clone(),
        author: new_book.author.clone(),
        year: new_book.year,
    };
    books.insert(new_id, book.clone());
    HttpResponse::Created().json(book)
}

#[get("/books/{id}")]
pub async fn get_book_by_id(
    books_db: web::Data<BooksDb>,
    path: web::Path<u32>,
) -> impl Responder {
    let id = path.into_inner();
    let books = books_db.lock().unwrap();
    if let Some(book) = books.get(&id) {
        HttpResponse::Ok().json(book)
    } else {
        let err_response = ErrorResponse {
            message: format!("Book with id {} not found", id),
        };
        HttpResponse::NotFound().json(err_response)
    }
}