use actix_web::{get, HttpResponse, Responder};
use askama_actix::Template;

struct IndexPackageInformation {
    repo: String,
    name: String,
    version: String,
    description: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    packages: Vec<IndexPackageInformation>,
}

#[get("/")]
pub async fn index() -> impl Responder {
    let index = IndexTemplate {
        packages: vec![IndexPackageInformation {
            repo: "f".to_string(),
            name: "d".to_string(),
            version: "b".to_string(),
            description: "w".to_string(),
        },
                       IndexPackageInformation {
                           repo: "f".to_string(),
                           name: "d".to_string(),
                           version: "b".to_string(),
                           description: "w".to_string(),
                       }]
    }.render().unwrap();

    HttpResponse::Ok().body(index)
}