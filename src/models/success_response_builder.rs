use chrono::Utc;
use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct SuccessResponseBuilder {
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

    #[serde(rename = "first")]
    pub first_link: Option<String>,

    #[serde(rename = "prev")]
    pub prev_link: Option<String>,

    #[serde(rename = "next")]
    pub next_link: Option<String>,

    #[serde(rename = "last")]
    pub last_link: Option<String>,
}

impl SuccessResponseBuilder {
    pub fn new() -> Self {
        Self {
            meta: Meta {
                total_pages: None,
                timestamp: Utc::now().with_timezone(&Tokyo).to_rfc3339(),
            },
            data: vec![],
            included: vec![],
            links: Links {
                self_link: None,
                first_link: None,
                prev_link: None,
                next_link: None,
                last_link: None,
            },
        }
    }

    pub fn total_pages(mut self, total_pages: i32) -> Self {
        self.meta.total_pages = Some(total_pages);
        self
    }

    pub fn push_data(mut self, r#type: String, attributes: Value, relationships: Value) -> Self {
        self.data.push(Data {
            r#type,
            id: Uuid::new_v4().to_string(),
            attributes,
            relationships,
        });
        self
    }

    pub fn push_included(
        mut self,
        r#type: String,
        attributes: Value,
        relationships: Value,
    ) -> Self {
        self.included.push(Data {
            r#type,
            id: Uuid::new_v4().to_string(),
            attributes,
            relationships,
        });
        self
    }

    pub fn self_link(mut self, self_link: String) -> Self {
        self.links.self_link = Some(self_link);
        self
    }

    pub fn first_link(mut self, first_link: String) -> Self {
        self.links.first_link = Some(first_link);
        self
    }

    pub fn prev_link(mut self, prev_link: String) -> Self {
        self.links.prev_link = Some(prev_link);
        self
    }

    pub fn next_link(mut self, next_link: String) -> Self {
        self.links.next_link = Some(next_link);
        self
    }

    pub fn last_link(mut self, last_link: String) -> Self {
        self.links.last_link = Some(last_link);
        self
    }
}
