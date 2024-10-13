use frand_home_common::state::client::user_state::UserStateProperty;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TaskBarProperty {
    pub user: UserStateProperty,
}

#[function_component]
pub fn TaskBar(prop: &TaskBarProperty) -> Html {
    let user = prop.user.clone();
    
    html! {
        <div>
            <p>{user.name.value()}</p>
        </div>
    }
}