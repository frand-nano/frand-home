use frand_home_common::{state::client::view::music::playlist_state::PlaylistState, Node, State};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct PlaylistProperty {
    pub state: <PlaylistState as State>::Property,
    pub musiclist_playlist_id: Node<String>,
}

#[function_component]
pub fn Playlist(prop: &PlaylistProperty) -> Html {
    let items: Vec<_> = prop.state.list_items.items.items().clone().into_iter()
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

    match prop.state.visible.value() {
        true => {
            html! {
                <div style="display:flex; flex-direction: column;">
                    <p>{"Playlist"}</p>
                    {items}
                </div>
            }
        },
        false => { html! { <div> </div> } },
    }
}
