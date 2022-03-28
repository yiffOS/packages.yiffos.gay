use actix_web::{get, HttpResponse, Responder, web};

#[get("/{get_package}")]
pub async fn package_view(package_name: web::Path<String>) -> impl Responder {
    let package_name = package_name.into_inner();
    debug!("Calling: package_view with package_name: {}", &package_name);

    HttpResponse::Ok().body(package_name)
}