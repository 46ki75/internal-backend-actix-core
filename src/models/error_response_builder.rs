use chrono::Utc;
use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct ErrorResponseBuilder {
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

impl ErrorResponseBuilder {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn push<T: ToString, U: ToString, V: ToString, W: ToString, X: ToString>(
        mut self,
        code: i32,
        status: T,
        pointer: U,
        parameter: V,
        title: W,
        detail: X,
    ) -> Self {
        self.errors.push(Error {
            id: Uuid::new_v4().to_string(),
            code,
            status: status.to_string(),
            source: Source {
                pointer: pointer.to_string(),
                parameter: parameter.to_string(),
            },
            title: title.to_string(),
            detail: detail.to_string(),
            meta: Meta {
                timestamp: Utc::now().with_timezone(&Tokyo).to_rfc3339(),
            },
        });
        self
    }
}
