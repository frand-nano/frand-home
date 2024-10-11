pub extern crate yew;

mod state;
mod node;

pub use self::{
    state::*,
    node::*,
};

pub fn vec_pushed<V: Clone>(vec: &Vec<V>, value: V) -> Vec<V> {
    let mut result = vec.clone();
    result.push(value);
    result
}