use frand_home_node::NodeState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, NodeState)]
pub struct UserState {
    pub name: String,
    pub email: String,
    pub picture: String,
    pub authenticated: bool,
    pub server_whitelist: bool,
    pub client_whitelist: bool,
}