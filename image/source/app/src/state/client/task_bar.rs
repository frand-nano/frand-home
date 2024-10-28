use frand_home_state::PropertyState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PropertyState)]
pub struct TaskBarState {
    pub playlist_visible: bool,
}
