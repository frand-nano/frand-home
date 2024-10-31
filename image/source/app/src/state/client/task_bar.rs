use frand_home_node::node_state;
use serde::{Deserialize, Serialize};

#[node_state]
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct TaskBar {
    pub playlist_visible: bool,
}
