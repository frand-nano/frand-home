use frand_home_node::node_state;
use serde::{Deserialize, Serialize};

use crate::state::client::{musiclist::Musiclist, youtube_player::YoutubePlayer, lyrics::Lyrics};

#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MusicClient {
    pub playlist_visible: bool,
    pub musiclist: Musiclist::State,
    pub youtube_player: YoutubePlayer::State,
    pub lyrics: Lyrics::State,
}
