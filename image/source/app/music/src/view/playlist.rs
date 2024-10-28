use frand_home_state::{Node, State};
use yew::{function_component, html, Html, Properties};

use crate::state::server::playlist_state::PlaylistItemsState;

#[derive(Properties, PartialEq)]
pub struct PlaylistProperty {
    pub visible: Node<bool>,
    pub list_items: <PlaylistItemsState as State>::Property,
    pub musiclist_playlist_id: Node<String>,
}

#[function_component]
pub fn Playlist(prop: &PlaylistProperty) -> Html {
    let visible = prop.visible.clone();
    let visible_value = *visible.value();
    let onclick_visible = move |_| {
        visible.emit(!visible_value)
    };

    let callback_item = prop.list_items.items.callback_item();
    let items: Vec<_> = prop.list_items.items.items().clone().into_iter()
    .enumerate()
    .map(|(index, item)| {
        let musiclist_playlist_id = prop.musiclist_playlist_id.clone();
        let title = item.title.clone();
        let playlist_id = item.playlist_id.clone();
        let onclick_playlist = move |_| {
            musiclist_playlist_id.emit(playlist_id.clone())
        };

        let item_refresh = item.refresh;
        let callback_item = callback_item.clone();
        let onclick_item_refresh = move |_| {
            let mut item = item.clone();
            item.refresh = true;
            callback_item.emit((index, item));
        };

        html! {
            <div>   
                <button disabled={item_refresh} onclick={onclick_item_refresh}> 
                {"🔄"}
                </button>
                <button disabled={item_refresh} onclick={onclick_playlist}>
                {title}
                </button>
            </div>
        }
    }).collect(); 

    match visible_value {
        true => {
            html! {
                <div style="display:flex; flex-direction: column;">                    
                    <button onclick={onclick_visible}>{" < Playlist"}</button>
                    {items}
                </div>
            }
        },
        false => { 
            html! {                 
                <button onclick={onclick_visible}>{" > "}</button>
            } 
        },
    }
}
