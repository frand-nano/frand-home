use frand_home_state::PropertyState;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct YoutubePlayerState {
    pub video_id: String,
}
