use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

use super::view::music::{lyrics_state::LyricsState, musiclist_state::MusiclistState, playlist_state::PlaylistState, task_bar_state::TaskBarState, youtube_player_state::YoutubePlayerState};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct ClientMusicState {
    pub task_bar: TaskBarState,
    pub playlist: PlaylistState,
    pub youtube_player: YoutubePlayerState,
    pub lyrics: LyricsState,
    pub musiclist: MusiclistState,
}
