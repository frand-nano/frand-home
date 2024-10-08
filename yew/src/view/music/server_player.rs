use frand_home_common::{state::server::view::music::server_player_state::ServerPlayerState, State};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ServerPlayerProperty {
    pub state: <ServerPlayerState as State>::Property,
}

#[function_component]
pub fn ServerPlayer(_prop: &ServerPlayerProperty) -> Html {
    html! {
        <div>

        </div>
    }
}