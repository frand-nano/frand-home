use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PropertyState)]
pub struct UserState {
    pub name: String,
}