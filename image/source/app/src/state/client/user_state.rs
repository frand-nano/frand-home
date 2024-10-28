use frand_home_state::PropertyState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PropertyState)]
pub struct UserState {
    pub name: String,
    pub email: String,
    pub picture: String,
    pub authenticated: bool,
    pub server_whitelist: bool,
    pub client_whitelist: bool,
}