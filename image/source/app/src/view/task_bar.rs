use yew::{function_component, html, Html, Properties};

use crate::state::client::{task_bar::TaskBar, user::User};

#[derive(Properties, PartialEq)]
pub struct TaskBarProperty {
    pub user: User::Node,
    pub task_bar: TaskBar::Node,
}

#[function_component]
pub fn TaskBarView(prop: &TaskBarProperty) -> Html {
    let user = prop.user.clone();

    let user = if *user.authenticated.value() {
        html! {
            <div style="display:flex; flex-direction: row;">
                <img src={user.picture.value().clone()} />
                <div style="display:flex; flex-direction: column;">
                    <p>{user.name.value()}</p>
                    <p>{user.email.value()}</p>
                </div>
            </div>
        }
    } else {
        html! {
            <a href="/login">
                <input type="button" value="Login" />
            </a>
        }
    };

    html! {
        {user}
    }
}