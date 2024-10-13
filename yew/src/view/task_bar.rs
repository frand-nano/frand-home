use frand_home_common::{state::client::user_state::UserStateProperty, Node, OptionNode, VecNode};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TaskBarProperty {
    pub user: UserStateProperty,
    pub number: Node<i32>,
    pub numbers: VecNode<i32>,
    pub dec: OptionNode<i32>,
}

#[function_component]
pub fn TaskBar(prop: &TaskBarProperty) -> Html {
    let user = prop.user.clone();
    
    let number = prop.number.clone();
    let number_value = *number.value();
    let onclick_number = move |_| {
        number.emit(number_value + 1)
    };

    let dec = prop.dec.clone();
    let dec_value = *dec.value();
    let onclick_dec = move |_| {
        match dec_value {
            Some(dec_value) => {
                if dec_value == 0 {
                    dec.emit(None);                
                } else {
                    dec.emit(Some(dec_value / 2));                
                }
            },
            None => {
                dec.emit(Some(107));   
            },
        }        
    };

    let pop = prop.numbers.callback_pop().clone();
    let onclick_pop = move |_| {
        pop.emit(())
    };

    let push = prop.numbers.callback_push().clone();
    let onclick_push = move |_| {
        push.emit(Default::default())
    };

    let item = prop.numbers.callback_item().clone();
    let numbers = prop.numbers.items().clone().into_iter().enumerate()
    .map(|(index, number)| {
        let item = item.clone();
        let onclick_item = move |_| {
            item.emit((index, number + 1))
        };
        html! {
            <button onclick={onclick_item}>
            {"Number: "}{number}
            </button>        
        }
    }).collect::<Vec<_>>();

    html! {
        <div>
            <div>
                <p>{user.name.value()}</p>
                <button onclick={onclick_number}>
                {"Number: "}{number_value}
                </button>
                <button onclick={onclick_dec}>
                {"Dec: "}{dec_value}
                </button>

                <button onclick={onclick_pop}>
                {" < "}
                </button>
                <button onclick={onclick_push}>
                {" > "}
                </button>
            </div>
            <div>
                {numbers}
            </div>
        </div>
    }
}