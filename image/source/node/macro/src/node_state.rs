use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use syn::{Field, Fields, Ident};
use quote::quote;

pub fn node_state(
    state_name: &Ident,
    state_fields: &Fields,
) -> TokenStream {
    let node_name = Ident::new("Node", Span::mixed_site());
    let message_name = Ident::new("Message", Span::mixed_site());

    let state_fields: Vec<(usize, &Field)> = match state_fields {
        syn::Fields::Named(fields_named) => fields_named.named.iter().enumerate().collect(),
        syn::Fields::Unnamed(_) => todo!(),
        syn::Fields::Unit => todo!(),
    };  

    let field_indexes: Vec<_> = state_fields.iter().map(|(index, _)| index + 2).collect();
    let field_names: Vec<_> = state_fields.iter().filter_map(|(_, field)| field.ident.as_ref()).collect();
    let field_tys: Vec<_> = state_fields.iter().map(|(_, field)| &field.ty).collect();

    let pascal_names: Vec<_> = field_names.iter()
    .map(|field_name| {
        let pascal_name = field_name.to_string().to_case(Case::Pascal);
        Ident::new(&pascal_name, field_name.span())
    }).collect();
    quote! {
        #[derive(Debug, Clone, yew::Properties)]
        pub struct #node_name {
            ids: Vec<usize>,
            callback: frand_home_node::Callback<#state_name>,
            #(pub #field_names: <#field_tys as frand_home_node::State>::Node,)*
        }

        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub enum #message_name {
            Error(String),
            State(#state_name),
            #(
                #pascal_names(<
                    <#field_tys as frand_home_node::State>::Node as frand_home_node::Node<#field_tys>
                >::Message),
            )*
        }

        impl frand_home_node::State for #state_name {
            type Node = #node_name;
        }

        impl PartialEq for #node_name {
            fn eq(&self, other: &Self) -> bool {
                true 
                #(&& self.#field_names == other.#field_names)*
            }
        }

        impl frand_home_node::Node<#state_name> for #node_name {
            type Message = #message_name;
            
            fn new<Comp: yew::BaseComponent, Msg: frand_home_node::RootMessage>(
                ids: Vec<usize>,
                id: Option<usize>,
                context: Option<&yew::Context<Comp>>,
            ) -> Self where <Comp as yew::BaseComponent>::Message: From<Msg> {
                let ids = frand_home_node::ids_pushed(ids, id);
                Self { 
                    ids: ids.clone(),
                    callback: frand_home_node::Callback::new(ids.clone(), frand_home_node::STATE_ID, context),
                    #(
                        #field_names: <<#field_tys as frand_home_node::State>::Node as frand_home_node::Node<#field_tys>>::new(
                            ids.clone(), Some(#field_indexes), context,
                        ),
                    )*
                }
            }    

            fn new_default(
                ids: Vec<usize>,
                id: Option<usize>,
            ) -> Self {
                let ids = frand_home_node::ids_pushed(ids, id);
                Self { 
                    ids: ids.clone(),
                    callback: frand_home_node::Callback::new_default(ids.clone(), frand_home_node::STATE_ID),
                    #(
                        #field_names: <<#field_tys as frand_home_node::State>::Node as frand_home_node::Node<#field_tys>>::new_default(
                            ids.clone(), Some(#field_indexes),
                        ),
                    )*
                }
            }

            fn ids(&self) -> &Vec<usize> { &self.ids }
            fn set_id(&mut self, index: usize, id: usize) { 
                self.ids[index] = id;
                self.callback.set_id(index, id);
                #(self.#field_names.set_id(index, id);)*
            }
            fn callback(&self) -> &frand_home_node::Callback<#state_name> { &self.callback }
            fn clone_state(&self) -> #state_name { 
                #state_name {
                    #(#field_names: self.#field_names.clone_state(),)*
                }
            }
            fn apply_state(&mut self, state: #state_name) { 
                #(self.#field_names.apply_state(state.#field_names);)*
            }
            fn apply(&mut self, message: Self::Message) {
                match message {
                    #message_name::Error(err) => {
                        log::error!(" {}.apply_message: {err}", stringify!(#node_name));
                    },
                    #message_name::State(state) => self.apply_state(state),
                    #(#message_name::#pascal_names(message) => self.#field_names.apply(message),)*
                }
            }
        }

        impl frand_home_node::Message for #message_name {
            fn try_error(err: String) -> anyhow::Result<Self> { Ok(Self::Error(err)) }
            
            fn try_new(depth: usize, data: frand_home_node::MessageData) -> anyhow::Result<Self> {
                match data.ids[depth] {
                    frand_home_node::STATE_ID => match data.data.downcast() {
                        Ok(data) => Ok(Self::State(*data)),
                        Err(_) => Err(anyhow::anyhow!("ids: {:?}, depth: {}", data.ids, depth)),
                    },
                    #(#field_indexes => Ok(Self::#pascal_names(
                        <<
                            <#field_tys as frand_home_node::State>::Node as frand_home_node::Node<#field_tys>
                        >::Message as frand_home_node::Message>::try_new(depth+1, data)?
                    )),)*
                    _ => Err(anyhow::anyhow!("ids: {:?}, depth: {}", data.ids, depth)),
                }    
            }
        }
    }
}