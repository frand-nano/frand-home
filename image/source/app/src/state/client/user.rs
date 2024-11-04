use frand_home_node::node_state;
use serde::{Deserialize, Serialize};

#[node_state]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub picture: String,
    pub login: String,
    pub authenticated: bool,
    pub server_whitelist: bool,
    pub client_whitelist: bool,
}