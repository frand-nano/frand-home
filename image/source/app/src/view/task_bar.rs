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
            <div id="task_bar_user" class="vertical">
                <img src={user.picture.value().clone()} />
                //<p>{user.name.value()}</p>
                //<p>{user.email.value()}</p>
            </div>
        }
    } else {
        let onclick = format!("location.href='{}'", user.login.value());
        html! {    
            <div id="task_bar_user">        
                <button class="round" onClick={onclick}> {"Login"} </button>
            </div>
        }
    };

    html! {
        <div id="task_bar" class="horizontal bottom_line">
            <nav>

            </nav>
            {user}
        </div>
    }
}