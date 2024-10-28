use std::collections::HashSet;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct, Type};

mod property_state;
mod property_state_root;

#[proc_macro]
pub fn property_state(item: TokenStream) -> TokenStream {
    let state = parse_macro_input!(item as ItemStruct);

    property_state::property_state(&state.ident, &state.fields).into()
}

#[proc_macro_derive(PropertyState)]
pub fn property_state_derive(item: TokenStream) -> TokenStream {
    let state = parse_macro_input!(item as ItemStruct);

    property_state::property_state(&state.ident, &state.fields).into()
}

#[proc_macro_derive(PropertyStateRoot)]
pub fn property_state_root(item: TokenStream) -> TokenStream {
    let state = parse_macro_input!(item as ItemStruct);
    let property_state = property_state::property_state(&state.ident, &state.fields);
    let property_state_root = property_state_root::property_state_root(&state.ident);
    
    quote! { 
        #property_state
        #property_state_root
    }.into()
}

lazy_static! {    
    static ref ATOMICS: HashSet<&'static str> = HashSet::from([
        stringify!(i8),stringify!(i16),stringify!(i32),stringify!(i64),stringify!(i128),stringify!(isize),
        stringify!(u8),stringify!(u16),stringify!(i32),stringify!(i64),stringify!(i128),stringify!(usize),
        stringify!(f32),stringify!(f64),
        stringify!(char),
        stringify!(bool),
        stringify!(()),
        stringify!(String),
    ]);
}

fn is_atomic(ty: &Type) -> bool {
    let ty = ty.to_token_stream().to_string();
    ATOMICS.contains(ty.as_str())
}