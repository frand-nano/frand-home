use frand_home_state::State;
use task_bar::TaskBar;
use yew::{html, Html};

use crate::state::{client::client_state::ClientState, server::server_state::ServerState};

pub mod task_bar;

pub fn view(
    server_prop: &<ServerState as State>::Property,
    client_prop: &<ClientState as State>::Property,
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