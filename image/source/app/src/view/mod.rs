use frand_home_node::Item;
use task_bar::TaskBar;
use yew::{html, Html};

use crate::state::{client::client_state::ClientState, server::server_state::ServerState};

pub mod task_bar;

pub fn view(
    server_prop: &<ServerState as Item>::Node,
    client_prop: &<ClientState as Item>::Node,
) -> Html {
    html! {
        <div>
            <TaskBar
                user = { client_prop.user.clone() }
                task_bar = { client_prop.task_bar.clone() }
            />     
            {frand_home_music::view(&server_prop.music, &client_prop.music)}
        </div>
    }
}