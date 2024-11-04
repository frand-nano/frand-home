use std::{collections::HashMap, fmt::Display};
use frand_home_app::state::client::user;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::CONFIG;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, Hash)]
pub struct User {
    server_whitelist: bool,
    client_whitelist: bool,
    name: String,
    email: String,
    id: i128,
    ip: String,
    picture: String,
    state: String,
}

impl User {
    pub fn authenticated(&self) -> bool { self.id != Default::default() }
    pub fn server_whitelist(&self) -> bool { self.server_whitelist }
    pub fn client_whitelist(&self) -> bool { self.client_whitelist }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {    
        let icon = if self.server_whitelist {
            "💎"
        } else if self.client_whitelist {
            if self.authenticated() {
                "👤"
            } else {
                "🌐"
            }            
        } else {
            if self.authenticated() {
                "🌐"
            } else { 
                "❓"  
            }            
        };
        
        if self.authenticated() {
            write!(f, "{icon} {}", self.name)            
        } else { 
            write!(f, "{icon} {}({})", self.ip, self.id)
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl TryFrom<User> for user::User::State {
    type Error = anyhow::Error;

    fn try_from(value: User) -> Result<Self, Self::Error> {
        Ok(user::User::State { 
            login: value.login_url()?,
            authenticated: value.authenticated(),
            name: value.name,
            email: value.email,
            picture: value.picture,
            server_whitelist: value.server_whitelist,
            client_whitelist: value.client_whitelist,
        })
    }
}

impl User {
    pub fn new(
        server_whitelist: bool,
        client_whitelist: bool,
        name: String,
        email: String,
        id: i128,
        ip: String,   
        picture: String, 
        state: String,
    ) -> Self {
        Self { 
            server_whitelist,
            client_whitelist, 
            name, 
            email, 
            id, 
            ip,
            picture,
            state,
        }
    }

    pub fn additional_info_text(&self) -> String {
        let email = &self.email;
        let id = &self.id;
        let ip = &self.ip;
        
        format!("📄 {email} : {id} : {ip}")
    }

    pub fn login_url(&self) -> anyhow::Result<String> {
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
        options.insert("state", &self.state);
        
        Ok(Url::parse_with_params(oauth_root, &options)?.to_string())
    }
}

