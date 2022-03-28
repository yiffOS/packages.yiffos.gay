use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::database::connect;

#[derive(Queryable, Clone)]
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

#[derive(Debug, Clone)]
pub struct PackageNotFoundError;

pub fn return_all_packages(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Vec<Package> {
    use crate::schema::packages::dsl::*;
    let mut packages_vec: Vec<Package> = vec![];

    let results = packages
        .load::<Package>(&connection)
        .expect("Error getting package information");

    for result in results {
        packages_vec.push(result);
    }

    packages_vec
}

pub fn return_single_package(connection: PooledConnection<ConnectionManager<PgConnection>>, package_name: String) -> Result<Package, PackageNotFoundError> {
    use crate::schema::packages::dsl::*;

    let result = packages
        .filter(name.eq(package_name))
        .load::<Package>(&connection)
        .expect("Error getting package information")
        .into_iter()
        .nth(0);

    if result.is_none() {
        return Err(PackageNotFoundError);
    }

    Ok(result.unwrap())
}

pub fn return_amount_of_packages(connection: PooledConnection<ConnectionManager<PgConnection>>) -> u64 {
    use crate::schema::packages::dsl::*;

    let result = packages
        .load::<Package>(&connection)
        .expect("Error getting package information");

    result.len() as u64
}