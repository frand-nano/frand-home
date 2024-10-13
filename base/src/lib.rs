pub extern crate yew;

mod state;
mod node;
mod option;
mod vec;

pub use self::{
    state::*,
    node::*,
    option::*,
    vec::*,
};

pub use frand_home_macro::*;

pub fn vec_pushed<V: Clone>(vec: &Vec<V>, value: V) -> Vec<V> {
    let mut result = vec.clone();
    result.push(value);
    result
}