use diesel::PgConnection;

use crate::database::connect;

#[derive(Queryable)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub epoch: i32,
    pub description: String,
    pub groups: Option<Vec<String>>,
    pub url: String,
    pub license: Option<Vec<String>>,
    pub depends: Option<Vec<String>>,
    pub optional_depends: Option<Vec<String>>,
    pub make_depends: Option<Vec<String>>,
    pub provides: Vec<String>,
    pub conflicts: Option<Vec<String>>,
    pub replaces: Option<Vec<String>>,
    pub maintainers: Vec<String>,
    pub repo: String
}

pub fn return_all_packages(connection: PgConnection) {
    unimplemented!()
}