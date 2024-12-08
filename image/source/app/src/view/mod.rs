use task_bar::TaskBarView;
use yew::{html, Html};

use crate::state::{client::client::Client, server::server::Server};

pub mod task_bar;

pub fn view(
    server: &Server::Node,
    client: &Client::Node,
) -> Html {
    html! {
        <>
            <TaskBarView
                user = { client.user.clone() }
                task_bar = { client.task_bar.clone() }
            />     
            <div id="content">
                {frand_home_music::view(&server.music, &client.music)}
            </div>
        </>
    }
}