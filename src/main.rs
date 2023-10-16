use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use chrono_tz::Asia::Tokyo;
use uuid::Uuid;
mod models;
use models::error_response;
use models::response;

use serde_json::json;

#[get("/")]
async fn hello() -> impl Responder {
    let res: response::Response = response::Response {
        meta: response::Meta {
            total_pages: Some(20),
            timestamp: Utc::now().with_timezone(&Tokyo).to_rfc3339(),
        },
        data: vec![response::Data {
            r#type: "nk".to_string(),
            id: Uuid::new_v4().to_string(),
            attributes: json!({}),
            relationships: json!({}),
        }],
        included: vec![],
        links: response::Links {
            self_link: Some("http://localhost:3000".to_string()),
            first: None,
            prev: None,
            next: None,
            last: None,
        },
    };

    HttpResponse::Ok().json(res)
}

#[get("/error")]
async fn error() -> impl Responder {
    let res: error_response::ErrorResponse = error_response::ErrorResponse {
        errors: vec![error_response::Error {
            id: Uuid::new_v4().to_string(),
            code: 400,
            status: "400 Bad Request".to_string(),
            source: error_response::Source {
                pointer: "/data/json".to_string(),
                parameter: "sort".to_string(),
            },
            title: "Bad Request".to_string(),
            detail: "Invalid Format".to_string(),
            meta: error_response::Meta {
                timestamp: Utc::now().with_timezone(&Tokyo).to_rfc3339(),
            },
        }],
    };

    HttpResponse::Ok().json(res)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(error).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
