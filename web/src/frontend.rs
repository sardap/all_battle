use crate::AppState;
use actix_files::NamedFile;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, web, Result};
use actix_web::{Error, HttpRequest};

async fn get_index(data: web::Data<AppState>) -> NamedFile {
    let path = data.build_dir.join("index.html");
    NamedFile::open(path).unwrap()
}

#[get("/{filename:.*}")]
async fn get_static_file(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<actix_files::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    match actix_files::NamedFile::open(data.build_dir.join(path)) {
        Ok(file) => Ok(file
            .use_last_modified(true)
            .set_content_disposition(ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![],
            })),
        Err(_) => Err(actix_web::error::ErrorNotFound("Not found")),
    }
}

pub(crate) fn scope() -> actix_web::Scope {
    web::scope("")
        .route("/", web::get().to(get_index))
        .route("/groups", web::get().to(get_index))
        .route("/mon-rank", web::get().to(get_index))
        .route("/trainer-rank", web::get().to(get_index))
        .route("/battles", web::get().to(get_index))
        .service(get_static_file)
}
