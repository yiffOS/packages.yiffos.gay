use actix_files::NamedFile;
use actix_web::{get, Error};
use actix_web::http::header::{ContentDisposition, DispositionType};

pub mod index;
pub mod package_view;

#[get("/favicon.ico")]
pub async fn favicon() -> Result<NamedFile, Error> {
    debug!("Calling: favicon");

    Ok(NamedFile::open("templates/favicon.ico")?
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}