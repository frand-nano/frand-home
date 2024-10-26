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