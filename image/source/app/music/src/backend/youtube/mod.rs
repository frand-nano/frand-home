mod playlist;
mod playlist_items;

use std::collections::HashSet;

use awc::Client;

use crate::state::server::playlist::PlaylistId;

pub use self::{
    playlist::*,
    playlist_items::*,
};

use super::config::Config;

pub async fn get_playlist_items_all(
    client: &Client,
    config: &Config,
    playlist_id: &PlaylistId,
) -> anyhow::Result<Vec<PlaylistItemsItem>> {
    let mut result = Vec::default();
    let mut tokens = HashSet::new();

    let mut playlist_page = PlaylistPage { 
        playlist_id: playlist_id.to_string(), 
        page_token: None, 
    };

    loop {
        let mut playlist_items = PlaylistItems::youtube_get(client, config, &playlist_page).await?;

        result.append(&mut playlist_items.items);
        playlist_page.page_token = playlist_items.next_page_token;
        
        if playlist_page.page_token.is_none() { break; }

        if tokens.contains(&playlist_page.page_token) { 
            break; 
        } else { 
            tokens.insert(playlist_page.page_token.clone()); 
        }
    }
    
    Ok(result)
}
