use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use askama_actix::{Template, TemplateToResponse};

struct IndexPackageInformation {
    name: String,
    version: String,
    description: String
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    packages: Vec<IndexPackageInformation>
}

#[get("/")]
async fn index() -> impl Responder {
    let mut index = IndexTemplate {
        packages: vec![IndexPackageInformation {
            name: "d".to_string(),
            version: "b".to_string(),
            description: "w".to_string()
        },
                       IndexPackageInformation {
                           name: "d".to_string(),
                           version: "b".to_string(),
                           description: "w".to_string()
                       }]
    }.render().unwrap();

    HttpResponse::Ok().body(index)
}

#[get("/{get_package}")]
async fn get_package(package_name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(package_name.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(fs::Files::new("/css", "./templates/css"))
            .service(fs::Files::new("/js", "./templates/js"))
            .service(get_package)

    })
        .bind(("127.0.0.1", 6969))?
        .run()
        .await
}