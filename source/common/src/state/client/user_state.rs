use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PropertyState)]
pub struct UserState {
    pub name: String,
    pub server_whitelist: bool,
    pub client_whitelist: bool,
}