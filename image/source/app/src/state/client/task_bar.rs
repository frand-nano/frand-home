use frand_home_node::NodeState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, NodeState)]
pub struct TaskBarState {
    pub playlist_visible: bool,
}
