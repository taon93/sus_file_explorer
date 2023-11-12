use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use serde::Serialize;

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("{\"message\": \"Hello world!\"}")
}

#[derive(Debug, Deserialize)]
struct PathData {
  path: String,
}

#[derive(Serialize)]
struct PathResponse {
  message: String,
}

#[post("/path")]
async fn path(data: web::Json<PathData>) -> Result<impl Responder> {
  let response_obj = PathResponse {
    message: format!("uooo panie: {}", data.path),
  };
  println!("{:?}", data);
  Ok(web::Json(response_obj))
}

async fn manual_hello() -> impl Responder {
  HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    let cors = Cors::default()
      .allowed_origin("http://127.0.0.1:3000")
      .allowed_origin("http://localhost:3000")
      .allowed_methods(vec!["GET", "POST"])
      .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
      .allowed_header(http::header::CONTENT_TYPE)
      .max_age(3600);

    App::new()
      .wrap(cors)
      .service(hello)
      .service(path)
      .route("/hey", web::get().to(manual_hello))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
