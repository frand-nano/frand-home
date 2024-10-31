use task_bar::TaskBarView;
use yew::{html, Html};

use crate::state::{client::client::Client, server::server::Server};

pub mod task_bar;

pub fn view(
    server_prop: &Server::Node,
    client_prop: &Client::Node,
) -> Html {
    html! {
        <div>
            <TaskBarView
                user = { client_prop.user.clone() }
                task_bar = { client_prop.task_bar.clone() }
            />     
            {frand_home_music::view(&server_prop.music, &client_prop.music)}
        </div>
    }
}