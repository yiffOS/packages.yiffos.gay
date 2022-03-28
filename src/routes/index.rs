use actix_web::{get, HttpResponse, Responder, web};
use askama_actix::Template;
use crate::database::packages::return_all_packages;
use crate::State;

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
pub async fn index(state: web::Data<State>) -> impl Responder {
    debug!("Calling: index");

    let packages = return_all_packages(state.db.clone().get().unwrap());
    let render_packages = packages.iter().map(|p| {
        IndexPackageInformation {
            repo: p.repo.clone(),
            name: p.name.clone(),
            version: p.version.clone(),
            description: p.description.clone(),
        }
    }).collect();

    debug!("{}" , packages.len());

    let index = IndexTemplate {
        packages: render_packages
    }.render().unwrap();

    HttpResponse::Ok().body(index)
}