use actix_session::SessionExt;
use actix_web::{http::header::ContentType, HttpRequest, HttpResponse};
use uuid::Uuid;
use crate::{authorize::user::User, session::SessionUtil};

pub async fn get_login(request: HttpRequest) -> HttpResponse {
    let session = request.get_session();
    let peer_ip = match request.peer_addr() {
        Some(peer_addr) => peer_addr.ip().to_string(),
        None => {
            log::error!(" get_login request.peer_addr() is None");          
            String::from("None")
        },
    };
    let state = Uuid::new_v4().to_string();

    if let Err(err) = session.insert("peer_ip", &peer_ip) {
        log::error!(" get_login session.insert 
            peer_ip: {peer_ip}, 
            err: {err},
        ");          
    }

    if let Err(err) = session.insert("state", &state) {
        log::error!(" get_login session.insert 
            state: {state}, 
            err: {err},
        ");         
    }
    
    let user = session.user();
    log::info!("{user} {} get_login", user.additional_info_text());

    login_button_html(&user)
    .unwrap_or(HttpResponse::InternalServerError().finish())
}

fn login_button_html(user: &User) -> anyhow::Result<HttpResponse> {
    let login_url = user.login_url()?;
    let onclick = format!("location.href=\'{login_url}\'");
    
    Ok(HttpResponse::Ok()
    .content_type(ContentType::html())
    .body(format!("
        <button 
            onClick={onclick}
            style='
                height:150px;
                width:150px;
                font-size:20px;
                display:block;
                margin:auto;
        '>Login</button>
    ")))
}