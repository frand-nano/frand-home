use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Ident, ItemStruct};


mod node_state;
mod node_state_root;

#[proc_macro_attribute]
pub fn node_state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut state = parse_macro_input!(item as ItemStruct);
    let state_name = state.ident.clone();
    state.ident = Ident::new("State", Span::mixed_site());
    let node_state = node_state::node_state(&state.ident, &state.fields);

    quote::quote! { 
        #[allow(non_snake_case)]
        pub mod #state_name {
            pub use super::*;
            #state
            #node_state
        }
    }.into()
}

#[proc_macro_attribute]
pub fn node_state_root(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut state = parse_macro_input!(item as ItemStruct);
    let state_name = state.ident.clone();
    state.ident = Ident::new("State", Span::mixed_site());
    let node_state = node_state::node_state(&state.ident, &state.fields);
    let node_state_root = node_state_root::node_state_root(&state.ident);    
    
    quote::quote! { 
        #[allow(non_snake_case)]
        pub mod #state_name {
            pub use super::*;
            #state
            #node_state
            #node_state_root
        }
    }.into()
}