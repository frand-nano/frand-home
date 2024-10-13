use frand_home_macro::PropertyState;
use serde::{Deserialize, Serialize};

use super::user_state::UserState;

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PropertyState)]
pub struct ClientState {
    pub user: UserState,
}
