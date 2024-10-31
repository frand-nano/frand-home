use frand_home_node::node_state_root;
use serde::{Deserialize, Serialize};

use super::{client::client::Client, server::server::Server};

#[node_state_root]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct App {
    pub server: Server::State,
    pub client: Client::State,
    pub opened: (),
    pub closed: (),
    pub alert: String,
}

