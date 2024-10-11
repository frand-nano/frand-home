use std::rc::Rc;

use yew::{Callback, Properties};

#[derive(Default, Clone, PartialEq, Properties)]
pub struct Node<V: PartialEq> {
    ids: Vec<usize>,
    callback: Callback<V>,
    value: Rc<V>,
}