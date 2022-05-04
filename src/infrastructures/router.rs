pub mod router {
    use actix_web::{get, web, HttpResponse, Responder};
    use serde_json::json;

    #[get("/")]
    pub async fn index() -> impl Responder {
        // HttpResponse::Ok().body("Hello world!")
        web::Json(json!({ "height": 174.3 }))
    }

    #[get("/{id}/{name}")]
    pub async fn sample_get(path: web::Path<(u32, String)>) -> impl Responder {
        let (id, name) = path.into_inner();
        HttpResponse::Ok().body(format!("Hello {}! id:{}", name, id))
    }
}
