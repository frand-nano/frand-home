use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::{Field, Fields, Ident};
use quote::quote;

use crate::is_atomic;

pub fn property_state(
    state_name: &Ident, 
    state_fields: &Fields,
) -> TokenStream {
    let state_message_name = {
        let state_message_name = format!("{state_name}Message");
        Ident::new(&state_message_name, state_name.span())
    };
    let state_property_name = {
        let state_message_name = format!("{state_name}Property");
        Ident::new(&state_message_name, state_name.span())
    };

    let state_fields: Vec<(bool, &Field)> = match state_fields {
        syn::Fields::Named(fields_named) => fields_named.named.iter()
        .map(|field| (is_atomic(&field.ty), field))
        .collect(),
        syn::Fields::Unnamed(_) => todo!(),
        syn::Fields::Unit => todo!(),
    };

    let message_variants: Vec<TokenStream> = state_fields.iter()
    .map(|(is_atomic, field)| {        
        let field_name = field.ident.as_ref();
        let pascal_name = field_name
        .map(|field_name| {
            let pascal_name = field_name.to_string().to_case(Case::Pascal);
            Ident::new(&pascal_name, field_name.span())
        });

        let field_ty = &field.ty;

        if *is_atomic {        
            quote! { 
                #pascal_name(#field_ty) 
            }
        } else {        
            quote! { 
                #pascal_name(<#field_ty as frand_home_base::State>::Message) 
            }
        }        
    })
    .collect();

    let property_fields: Vec<TokenStream> = state_fields.iter()
    .map(|(is_atomic, field)| {
        let field_name = &field.ident;
        let field_ty = &field.ty;

        if *is_atomic {        
            quote! { 
                pub #field_name: frand_home_base::Node<#field_ty> 
            }
        } else {        
            quote! { 
                pub #field_name: <#field_ty as frand_home_base::State>::Property
            }
        }        
    })
    .collect();

    let impl_state_property_applys: Vec<TokenStream> = state_fields.iter()
    .map(|(is_atomic, field)| {
        let field_name = &field.ident;
        let field_ty = &field.ty;

        if *is_atomic {   
            quote! { 
                self.#field_name.apply(value.#field_name.clone())
            }   
        } else {
            quote! { 
                self.#field_name.apply_message(
                    <#field_ty as frand_home_base::State>::Message::State(
                        value.#field_name.clone(),
                    ), 
                )
            }        
        }
    })
    .collect();

    let impl_state_property_apply_cases: Vec<TokenStream> = state_fields.iter()
    .map(|(is_atomic, field)| {        
        let field_name = field.ident.as_ref();
        let pascal_name = field_name
        .map(|field_name| {
            let pascal_name = field_name.to_string().to_case(Case::Pascal);
            Ident::new(&pascal_name, field_name.span())
        });

        if *is_atomic {   
            quote! { 
                Self::Message::#pascal_name(value) => self.#field_name.apply(value)
            }   
        } else {
            quote! { 
                Self::Message::#pascal_name(message) => self.#field_name.apply_message(message)
            }        
        }
    })
    .collect();

    let impl_state_property_export_cases: Vec<TokenStream> = state_fields.iter()
    .map(|(is_atomic, field)| {        
        let field_name = field.ident.as_ref();
        let pascal_name = field_name
        .map(|field_name| {
            let pascal_name = field_name.to_string().to_case(Case::Pascal);
            Ident::new(&pascal_name, field_name.span())
        });

        if *is_atomic {   
            quote! { 
                Self::Message::#pascal_name(value) => *value = self.#field_name.value().clone()
            }   
        } else {
            quote! { 
                Self::Message::#pascal_name(message) => self.#field_name.export_message(message)
            }        
        }
    })
    .collect();

    let impl_message_cases: Vec<TokenStream> = state_fields.iter()
    .enumerate()
    .map(|(index, (is_atomic, field))| {
        let id = index + 2;        
        let field_name = field.ident.as_ref();
        let pascal_name = field_name
        .map(|field_name| {
            let pascal_name = field_name.to_string().to_case(Case::Pascal);
            Ident::new(&pascal_name, field_name.span())
        });
        let field_ty = &field.ty;
        
        if *is_atomic {   
            quote! { 
                #id => Ok(Self::#pascal_name(*value.downcast()?))
            }   
        } else {
            quote! {                 
                #id => Ok(Self::#pascal_name(
                    <#field_ty as frand_home_base::State>::Message::new(ids, index+1, value)
                ))
            }        
        }
    })
    .collect();

    let impl_property_fields: Vec<TokenStream> = state_fields.iter()
    .enumerate()
    .map(|(index, (is_atomic, field))| {
        let id = index + 2;        
        let field_name = &field.ident;
        let field_ty = &field.ty;

        if *is_atomic {        
            quote! { 
                #field_name: frand_home_base::Node::new(
                    frand_home_base::vec_pushed(&ids, #id), 
                    context,
                )
            }
        } else {        
            quote! { 
                #field_name: <#field_ty as frand_home_base::State>::Property::new(
                    frand_home_base::vec_pushed(&ids, #id), 
                    context,
                )
            }
        }        
    })
    .collect();

    quote! {
        #[derive(Default, Clone, PartialEq, frand_home_base::yew::Properties)]
        pub struct #state_property_name {
            pub state: frand_home_base::Node<#state_name>,
            #(#property_fields,)*
        }

        #[derive(Serialize, Deserialize, Clone)]
        pub enum #state_message_name {
            Error(String),
            State(#state_name),
            #(#message_variants,)*
        }

        impl frand_home_base::NodeValue for #state_name {}

        impl frand_home_base::State for #state_name {
            type Property = #state_property_name;
            type Message = #state_message_name;
        }

        impl frand_home_base::StateProperty for #state_property_name {
            type Message = #state_message_name;
        
            fn apply_message(&mut self, message: Self::Message) {
                match message {
                    Self::Message::Error(err) => log::error!("{err}"),
                    Self::Message::State(value) => {
                        #(#impl_state_property_applys;)*
                        self.state.apply(value);
                    },
                    #(#impl_state_property_apply_cases,)*
                }
            }

            fn export_message(&self, message: &mut Self::Message) {
                match message {
                    Self::Message::Error(err) => *err = format!("Export err from Node is no meaning. err: {err}"),
                    Self::Message::State(value) => *value = self.state.value().clone(),
                    #(#impl_state_property_export_cases,)*
                }
            }
            
            fn new<Comp, Msg>(
                #[allow(unused_variables)] ids: Vec<usize>,
                context: Option<&frand_home_base::yew::Context<Comp>>,
            ) -> Self 
            where
                Comp: frand_home_base::yew::BaseComponent, 
                Msg: frand_home_base::StateMessage,
                <Comp as frand_home_base::yew::BaseComponent>::Message: From<Msg>,
            {
                Self { 
                    state: frand_home_base::Node::new(
                        frand_home_base::vec_pushed(&ids, 1), 
                        context,
                    ),
                    #(#impl_property_fields,)*
                }
            }
        }

        impl frand_home_base::StateMessage for #state_message_name {
            fn error(err: String) -> Self { Self::Error(err) }

            fn try_new(
                ids: &[usize], 
                index: usize, 
                #[allow(unused_variables)] value: Box<dyn std::any::Any>,
            ) -> Result<Self, Box<dyn std::any::Any>> {
                match ids[index] {
                    1 => Ok(Self::State(*value.downcast()?)),
                    #(#impl_message_cases,)*
                    _ => Err(value),
                }        
            }
        }
    }
}