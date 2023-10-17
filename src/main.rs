use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
mod models;
use models::error_response_builder::ErrorResponseBuilder;
use models::success_response_builder::SuccessResponseBuilder;

#[get("/")]
async fn hello() -> impl Responder {
    let res: SuccessResponseBuilder = SuccessResponseBuilder::new()
        .total_pages(0)
        .self_link("http://localhost:8080".to_string())
        .push_data(
            "message".to_string(),
            json!({"message":"Hellow from root!"}),
            json!({}),
        );

    HttpResponse::Ok().json(res)
}

#[get("/error")]
async fn error() -> impl Responder {
    let res: ErrorResponseBuilder = ErrorResponseBuilder::new().push(
        400,
        "400_BAD_REQUEST",
        "/data/url",
        "sort",
        "Bad Request",
        "Request format is incorrect. Please review it again.",
    );

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
