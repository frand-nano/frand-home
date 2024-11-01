use std::collections::HashMap;
use actix_session::SessionExt;
use actix_web::{http::header::ContentType, HttpRequest, HttpResponse};
use url::Url;
use uuid::Uuid;
use crate::{session::SessionUtil, CONFIG};

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

    login_button_html(state)
    .unwrap_or(HttpResponse::InternalServerError().finish())
}

fn login_button_html(state: String) -> anyhow::Result<HttpResponse> {
    let login_url = login_url(state)?;
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

fn login_url(state: String) -> anyhow::Result<Url> {
    let oauth_redirect = CONFIG.oauth_redirect_with_port()?;
    let oauth_root = CONFIG.uris.oauth_root.as_str();
    let oauth_scope_profile = CONFIG.uris.oauth_scope_profile.as_str();
    let oauth_scope_email = CONFIG.uris.oauth_scope_email.as_str();
    let client_id = CONFIG.keys.client_id.as_str();
    
    let scope = format!("{oauth_scope_profile} {oauth_scope_email}");

    let mut options = HashMap::new();
    options.insert("redirect_uri", oauth_redirect.as_str());
    options.insert("client_id", client_id);
    options.insert("access_type", "offline");
    options.insert("response_type", "code");
    options.insert("scope", &scope);
    options.insert("state", &state);
    
    Ok(Url::parse_with_params(oauth_root, &options)?)
}