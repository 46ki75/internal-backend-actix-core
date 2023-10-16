use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}

#[derive(Deserialize, Serialize)]
pub struct Error {
    pub id: String,
    pub code: i32,
    pub status: String,
    pub source: Source,
    pub title: String,
    pub detail: String,
    pub meta: Meta,
}

#[derive(Deserialize, Serialize)]
pub struct Source {
    pub pointer: String,
    pub parameter: String,
}

#[derive(Deserialize, Serialize)]
pub struct Meta {
    pub timestamp: String,
}
