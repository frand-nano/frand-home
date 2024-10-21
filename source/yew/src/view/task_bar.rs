use frand_home_common::state::client::{user_state::UserStateProperty, view::task_bar::TaskBarStateProperty};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TaskBarProperty {
    pub user: UserStateProperty,
    pub task_bar: TaskBarStateProperty,
}

#[function_component]
pub fn TaskBar(prop: &TaskBarProperty) -> Html {
    let user = prop.user.clone();

    let playlist_visible = prop.task_bar.playlist_visible.clone();
    let playlist_visible_value = *playlist_visible.value();
    let onclick_playlist_visible = move |_| {
        playlist_visible.emit(!playlist_visible_value)
    };

    html! {
        <div>
            <p>{user.name.value()}</p>
            <button onclick={onclick_playlist_visible}>{"Playlist"}</button>
        </div>
    }
}