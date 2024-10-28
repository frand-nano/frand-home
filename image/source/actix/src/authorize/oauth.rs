use std::str::FromStr;
use actix_http::{header::LOCATION, Uri};
use actix_web::{dev::PeerAddr, get, web::{Data, Query}, HttpResponse};
use actix_session::Session;
use awc::Client;
use serde::{Deserialize, Serialize};
use crate::{session::SessionUtil, CONFIG};

#[derive(Debug, Serialize, Deserialize)]
struct QueryCode {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: String,
}

#[get("/oauth")]
pub async fn get_oauth(
    peer_addr: PeerAddr,
    session: Session,
    query: Query<QueryCode>,
    client: Data<Client>,
) -> actix_web::Result<HttpResponse> {     
    log::info!("🔑 {:#?}", query);

    let authorization_code = &query.code;   
    let query_state = query.state.to_owned();
    let session_state = session.state();

    if authorization_code.is_empty() { 
        log::warn!("⛔ Authorization code not provided");  
        return Ok(HttpResponse::Unauthorized().finish());
    }  else if query_state != session_state {
        log::warn!("⛔ query state: {query_state}, session state: {session_state}");  
        return Ok(HttpResponse::Unauthorized().finish());
    } 

    let token_response = request_token(authorization_code.as_str(), &client).await?;
    let user = get_user(token_response, &client,).await?;
    let peer_ip = peer_addr.0.ip().to_string();

    session.insert("peer_ip", peer_ip)?;   
    session.insert("user_id", user.id)?;   
    session.insert("user_name", user.name)?;   
    session.insert("user_email", user.email)?;   
    session.insert("picture", user.picture)?;   

    let user = session.user();
    log::info!("{user} {}", user.additional_info_text());

    Ok(HttpResponse::Found()
    .append_header((LOCATION, "/"))
    .finish())
}

async fn request_token(
    authorization_code: &str,
    client: &Client,
) -> Result<OAuthResponse, Box<dyn std::error::Error>> {
    let oauth_redirect = CONFIG.oauth_redirect_with_port()?;
    let oauth_token = CONFIG.uris.oauth_token.as_str();

    let client_id = CONFIG.keys.client_id.as_str();
    let client_secret = CONFIG.keys.client_secret.as_str();

    let params = [
        ("grant_type", "authorization_code"),
        ("code", authorization_code),
        ("redirect_uri", oauth_redirect.as_str()),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    let mut response = client.post(oauth_token).query(&params)?.send().await?;

    if response.status().is_success() {
        Ok(response.json::<OAuthResponse>().await?)
    } else {
        Err(From::from(format!("OAuth fail with status:{}", response.status())))
    }
}

async fn get_user(
    oauth_response: OAuthResponse,
    client: &Client,
) -> Result<UserInfo, Box<dyn std::error::Error>> {
    let oauth_userinfo = CONFIG.uris.oauth_userinfo.as_str();
    let access_token = oauth_response.access_token;
    let id_token = oauth_response.id_token;

    let uri = Uri::from_str(&format!("{oauth_userinfo}?access_token={access_token}"))?;
    let mut response = client.get(uri).bearer_auth(id_token).send().await?;

    if response.status().is_success() {
        Ok(response.json::<UserInfo>().await?)
    } else {
        Err(From::from(format!("Get User Data fail with status:{}", response.status())))
    }
}