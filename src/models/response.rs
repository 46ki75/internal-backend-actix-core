use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct Response {
    pub meta: Meta,
    pub data: Vec<Data>,
    pub included: Vec<Data>,
    pub links: Links,
}

#[derive(Deserialize, Serialize)]
pub struct Meta {
    #[serde(rename = "totalPages")]
    pub total_pages: Option<i32>,
    pub timestamp: String,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub r#type: String,
    pub id: String,
    pub attributes: Value,
    pub relationships: Value,
}

#[derive(Deserialize, Serialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: Option<String>,
    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
}
