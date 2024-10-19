use actix_files::NamedFile;
use actix_session::SessionExt;
use actix_web::{get, web::Path, HttpRequest, HttpResponse};

use crate::session::SessionUtil;

#[get("/")]
pub async fn get_index(
    request: HttpRequest,
) -> HttpResponse {    
    if !request.get_session().client_whitelist() {
        return super::get_login(request).await;
    }

    match NamedFile::open_async("./target/dist/index.html").await {
        Ok(response) => response.into_response(&request),
        Err(err) => {
            log::error!("❗ get_index err: {err}");  
            HttpResponse::InternalServerError().finish()  
        },
    }
}

#[get("/favicon.ico")]
pub async fn get_index_favicon(
    request: HttpRequest,
) -> HttpResponse {
    match NamedFile::open_async("./res/favicon.ico").await {
        Ok(response) => response.into_response(&request),
        Err(err) => {
            log::error!("❗ get_index_favicon err: {err}");  
            HttpResponse::NotFound().finish()    
        },
    }
}

#[get("/frand-home-yew-{path}")]
pub async fn get_index_path(
    path: Path<(String,)>,
    request: HttpRequest,
) -> HttpResponse {
    if !request.get_session().client_whitelist() {
        return super::get_login(request).await;
    }

    let (path,) = path.into_inner();  

    match NamedFile::open_async(format!("./target/dist/frand-home-yew-{path}")).await {
        Ok(response) => response.into_response(&request),
        Err(err) => {
            log::error!("❗ get_index_path path: {path}, err: {err}");  
            HttpResponse::NotFound().finish()   
        },
    }
}
