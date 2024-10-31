use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};


mod node_state;
mod node_state_root;

#[proc_macro_derive(NodeState)]
pub fn node_state_derive(item: TokenStream) -> TokenStream {
    let state = parse_macro_input!(item as ItemStruct);

    node_state::node_state(&state.ident, &state.fields).into()
}

#[proc_macro_derive(NodeStateRoot)]
pub fn node_state_root(item: TokenStream) -> TokenStream {
    let state = parse_macro_input!(item as ItemStruct);
    let node_state = node_state::node_state(&state.ident, &state.fields);
    let node_state_root = node_state_root::node_state_root(&state.ident);    
    
    quote::quote! { 
        #node_state
        #node_state_root
    }.into()
}