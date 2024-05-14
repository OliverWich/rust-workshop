use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub year: u16,
}

#[derive(Serialize, Deserialize)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub year: u16,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}