use frand_home_node::NodeState;
use serde::{Deserialize, Serialize};

use super::{lyrics_state::LyricsState, musiclist_state::MusiclistState, youtube_player_state::YoutubePlayerState};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, NodeState)]
pub struct ClientState {
    pub playlist_visible: bool,
    pub musiclist: MusiclistState,
    pub youtube_player: YoutubePlayerState,
    pub lyrics: LyricsState,
}
