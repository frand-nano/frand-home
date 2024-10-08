use actix_files::NamedFile;
use actix_session::SessionExt;
use actix_web::{get, web::Path, HttpRequest, HttpResponse};
use crate::session::SessionUtil;

#[get("/res/{path}")]
pub async fn get_res_path(
    path: Path<(String,)>,
    request: HttpRequest,
) -> HttpResponse {
    if !request.get_session().client_whitelist() {
        return super::get_login(request).await;
    }

    let (path,) = path.into_inner();  

    match NamedFile::open_async(format!("./res/{path}")).await {
        Ok(response) => response.into_response(&request),
        Err(err) => {
            log::error!("❗ get_res_path path: {path}, err: {err}");  
            HttpResponse::NotFound().finish()   
        },
    }
}
