
use mysql::PooledConn;

use crate::{backend::{database::insert_update_music, youtube::PlaylistItemsItem}, state::{client::musiclist::MusiclistItem, server::playlist::PlaylistId}};

pub async fn insert_update_musics(
    conn: &mut PooledConn,
    playlist_id: &PlaylistId,
    playlist_items: Vec<PlaylistItemsItem>,
) -> anyhow::Result<Vec<MusiclistItem::State>> {
    let musics: Vec<MusiclistItem::State> = playlist_items.into_iter()
    .map(|item| MusiclistItem::State {        
        music_id: item.snippet.resource_id.video_id,
        youtube_title: item.snippet.title,
        ..Default::default()
    })
    .collect();

    log::info!("📜 insert_update_musics playlist_id: {playlist_id}");

    for music in &musics {
        insert_update_music(conn, playlist_id, music)?;
    }

    Ok(musics)
}
