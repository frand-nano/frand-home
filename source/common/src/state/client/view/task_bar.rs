use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, PropertyState)]
pub struct TaskBarState {
    pub playlist_visible: bool,
}
