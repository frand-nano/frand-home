use frand_home_common::{state::server::music::playlist_state::PlaylistItemsState, Node, State};
use yew::{function_component, html, Html, Properties};

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

    let items: Vec<_> = prop.list_items.items.items().clone().into_iter()
    .map(|item| {
        let musiclist_playlist_id = prop.musiclist_playlist_id.clone();
        let title = item.title.clone();
        let playlist_id = item.playlist_id.clone();
        let onclick_playlist = move |_| {
            musiclist_playlist_id.emit(playlist_id.clone())
        };
        html! {
            <button onclick={onclick_playlist}>
            {title}
            </button>
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
