use std::fmt::Display;
use frand_home_app::state::client::user_state::UserState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, Hash)]
pub struct User {
    server_whitelist: bool,
    client_whitelist: bool,
    name: String,
    email: String,
    id: i128,
    ip: String,
    picture: String,
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

impl From<User> for UserState {
    fn from(value: User) -> Self {
        UserState { 
            authenticated: value.authenticated(),
            name: value.name,
            email: value.email,
            picture: value.picture,
            server_whitelist: value.server_whitelist,
            client_whitelist: value.client_whitelist,
        }
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
    ) -> Self {
        Self { 
            server_whitelist,
            client_whitelist, 
            name, 
            email, 
            id, 
            ip,
            picture,
        }
    }

    pub fn additional_info_text(&self) -> String {
        let email = &self.email;
        let id = &self.id;
        let ip = &self.ip;
        
        format!("📄 {email} : {id} : {ip}")
    }
}