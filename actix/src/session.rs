use actix_session::Session;
use crate::{authorize::user::User, CONFIG};

pub trait SessionUtil {
    fn get_string(&self, key: &str) -> String;    

    fn user_id(&self) -> String { self.get_string("user_id") }  
    fn user_name(&self) -> String { self.get_string("user_name") }  
    fn state(&self) -> String { self.get_string("state") }  
    fn picture(&self) -> String { self.get_string("picture") }  

    fn server_whitelist(&self) -> bool {
        CONFIG.settings.server_whitelists.contains(&self.user_id())
    }

    fn client_whitelist(&self) -> bool {
        CONFIG.settings.client_whitelists_all 
        || CONFIG.settings.client_whitelists.contains(&self.user_id())
    }

    fn user(&self) -> User {
        User::new(
            self.server_whitelist(),
            self.client_whitelist(),
            self.user_name(),
            self.get_string("user_email"),
            self.user_id().parse().unwrap_or_default(),
            self.get_string("peer_ip"),
            self.picture(),
        )
    }
}

impl SessionUtil for Session {
    fn get_string(&self, key: &str) -> String {        
        if let Ok(Some(value)) = self.get::<String>(key) {
            value
        } else { String::from(key) }
    }
}