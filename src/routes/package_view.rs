use actix_web::{get, HttpResponse, Responder, web};
use askama_actix::Template;
use crate::database::packages::{Package, return_amount_of_packages, return_single_package};
use crate::State;

struct PackageViewInformation{
    name: String,
    version: String,
    epoch: String,
    description: String,
    url: String,
    licenses: Vec<String>,
    groups: Vec<String>,
    dependencies: Vec<String>,
    required_by: Vec<String>,
    optional_dependencies: Vec<String>,
    build_dependencies: Vec<String>,
    conflicts: Vec<String>,
    replaces: Vec<String>,
    provides: Vec<String>,
    maintainers: Vec<String>
}

#[derive(Template)]
#[template(path = "package.html")]
struct PackageTemplate {
    package: PackageViewInformation,
    packages_amount: u64,
}

#[derive(Template)]
#[template(path = "notfound.html")]
struct PackageNotFoundTemplate {
    package_name: String,
    packages_amount: u64
}

#[get("/{get_package}")]
pub async fn package_view(state: web::Data<State>, package_name: web::Path<String>) -> impl Responder {
    let package_name = package_name.into_inner();
    debug!("Calling: package_view with package_name: {}", &package_name);

    let package = return_single_package(
        state.db.clone().get().unwrap(), package_name.clone());

    if package.is_err() {
        let package_not_found_view = PackageNotFoundTemplate {
            package_name,
            packages_amount: return_amount_of_packages(state.db.clone().get().unwrap())
        }.render().unwrap();

        return HttpResponse::NotFound().body(package_not_found_view);
    }

    let package = package.unwrap();

    let package_view = PackageTemplate {
        package: PackageViewInformation {
            name: package.name,
            version: package.version,
            epoch: package.epoch.to_string(),
            description: package.description,
            url: package.url,
            licenses: package.license.unwrap_or(vec![]),
            groups: package.groups.unwrap_or(vec![]),
            dependencies: package.depends.unwrap_or(vec![]),
            required_by: vec![],
            optional_dependencies: package.optional_depends.unwrap_or(vec![]),
            build_dependencies: package.make_depends.unwrap_or(vec![]),
            conflicts: package.conflicts.unwrap_or(vec![]),
            replaces: package.replaces.unwrap_or(vec![]),
            provides: package.provides,
            maintainers: package.maintainers
        },
        packages_amount: return_amount_of_packages(state.db.clone().get().unwrap()),
    }.render().unwrap();

    HttpResponse::Ok().body(package_view)
}