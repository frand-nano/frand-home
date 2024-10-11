use frand_home_common::{state::client::user_state::UserStateProperty, Node};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TaskBarProperty {
    pub user: UserStateProperty,
    pub number: Node<i32>,
}

#[function_component]
pub fn TaskBar(prop: &TaskBarProperty) -> Html {
    let user = prop.user.clone();
    
    let number = prop.number.clone();
    let number_value = *number.value();
    let onclick_number = move |_| {
        number.emit(number_value + 1)
    };

    html! {
        <div>
            <p>{user.name.value()}</p>
            <button onclick={onclick_number}>
            {"Number: "}{number_value}
            </button>
        </div>
    }
}