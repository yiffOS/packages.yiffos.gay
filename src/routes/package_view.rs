use actix_web::{get, HttpResponse, Responder, web};

#[get("/{get_package}")]
pub async fn package_view(package_name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(package_name.into_inner())
}