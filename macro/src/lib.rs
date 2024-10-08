use std::collections::HashSet;
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Field, Ident, ItemStruct, Type};

#[proc_macro_derive(PropertyState)]
pub fn property_state(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let state = parse_macro_input!(item as ItemStruct);

    let state_name = &state.ident;
    let state_message_name = {
        let state_message_name = format!("{state_name}Message");
        Ident::new(&state_message_name, state_name.span())
    };
    let state_property_name = {
        let state_message_name = format!("{state_name}Property");
        Ident::new(&state_message_name, state_name.span())
    };

    let state_fields: Vec<(bool, &Field)> = match &state.fields {
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

    let impl_state_cases: Vec<TokenStream> = state_fields.iter()
    .enumerate()
    .map(|(_, (is_atomic, field))| {
        let field_name = field.ident.as_ref();
        let pascal_name = field_name
        .map(|field_name| {
            let pascal_name = field_name.to_string().to_case(Case::Pascal);
            Ident::new(&pascal_name, field_name.span())
        });

        if *is_atomic {   
            quote! { 
                Self::Message::#pascal_name(value) => self.#field_name = value
            }   
        } else {
            quote! { 
                Self::Message::#pascal_name(value) => self.#field_name.apply(value)
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
                #id => Ok(Self::#pascal_name(<#field_ty as frand_home_base::State>::Message::new(ids, index+1, value)))
            }        
        }
    })
    .collect();

    let impl_property_fields: Vec<TokenStream> = state_fields.iter()
    .enumerate()
    .map(|(index, (is_atomic, field))| {
        let id = index + 2;
        let field_name = field.ident.as_ref();
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

    let apply_property_fields: Vec<TokenStream> = state_fields.iter()
    .map(|(is_atomic, field)| {
        let field_name = field.ident.as_ref();
        
        if *is_atomic {   
            quote! { 
                #field_name: self.#field_name.applied(state.#field_name.clone())
            }  
        } else {
            quote! { 
                #field_name: self.#field_name.applied(&state.#field_name)
            }      
        }
    })
    .collect();

    quote! {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub enum #state_message_name {
            Error(String),
            State(#state_name),
            #(#message_variants,)*
        }

        #[derive(frand_home_base::Properties, PartialEq, Clone, Default)]
        pub struct #state_property_name {
            pub callback: frand_home_base::Callback<#state_name>,
            #(#property_fields,)*
        }

        impl frand_home_base::State for #state_name {
            type Message = #state_message_name;
            type Property = #state_property_name;
        
            fn apply(&mut self, message: Self::Message) {
                match message {
                    Self::Message::Error(err) => log::error!("{err}"),
                    Self::Message::State(value) => *self = value,
                    #(#impl_state_cases,)*
                }
            }
        }

        impl frand_home_base::StateMessage for #state_message_name {
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

            fn error(err: String) -> Self {
                Self::Error(err)
            }
        }

        impl frand_home_base::StateProperty for #state_property_name {
            fn new<Comp, Msg>(
                #[allow(unused_variables)] ids: Vec<usize>,
                context: Option<&frand_home_base::Context<Comp>>,
            ) -> Self 
            where
                Comp: frand_home_base::BaseComponent, 
                Msg: frand_home_base::StateMessage,
                <Comp as frand_home_base::BaseComponent>::Message: From<Msg>,
            {
                let callback_ids = ids.clone();
                let callback = match context {
                    Some(context) => context.link().callback(move |state: #state_name| 
                        Msg::new(callback_ids.as_slice(), 0, Box::new(state))
                    ),
                    None => Default::default(),
                };

                Self { 
                    callback,
                    #(#impl_property_fields,)*
                }
            }
        }

        impl #state_property_name {        
            pub fn applied(&self, state: &#state_name) -> Self {
                Self {
                    callback: self.callback.clone(),        
                    #(#apply_property_fields,)*
                }
            }

            pub fn callback(&self) -> &frand_home_base::Callback<#state_name> {
                &self.callback
            }

            pub fn emit(&self, state: #state_name) {
                self.callback.emit(state)
            }
        }
    }.into()
}

#[proc_macro_derive(JsonConvert)]
pub fn json_convert(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let state = parse_macro_input!(item as ItemStruct);

    let state_name = &state.ident;
    let state_message_name = {
        let state_message_name = format!("{state_name}Message");
        Ident::new(&state_message_name, state_name.span())
    };

    quote! {
        impl TryFrom<#state_message_name> for String {
            type Error = anyhow::Error;

            fn try_from(value: #state_message_name) -> std::result::Result<Self, Self::Error> {
                Ok(serde_json::to_string_pretty(&value)?)
            }
        }

        impl TryFrom<&bytestring::ByteString> for #state_message_name {
            type Error = serde_json::Error;

            fn try_from(value: &bytestring::ByteString) -> std::result::Result<Self, serde_json::Error> {
                serde_json::from_str(value)
            }
        }

        impl From<anyhow::Result<String>> for #state_message_name {
            fn from(value: anyhow::Result<String>) -> Self {
                match value {
                    Ok(value) => {
                        match serde_json::from_str(&value) {
                            Ok(result) => result,
                            Err(err) => <Self as frand_home_base::StateMessage>::error(
                                format!(
                                    "❗ {}::from(anyhow::Result<String>) err: {}, value: {}", 
                                    stringify!(#state_message_name),
                                    err,
                                    value,
                                ),
                            ),
                        }
                    },
                    Err(_) => <Self as frand_home_base::StateMessage>::error(
                        format!(
                            "❗ {}::from(anyhow::Result<String>) value: Err", 
                            stringify!(#state_message_name),
                        ),
                    ),
                }
            }
        }

        impl From<anyhow::Result<Vec<u8>>> for #state_message_name {
            fn from(value: anyhow::Result<Vec<u8>>) -> Self {
                match value {
                    Ok(value) => {
                        match serde_json::from_slice(&value) {
                            Ok(result) => result,
                            Err(err) => <Self as frand_home_base::StateMessage>::error(
                                format!(
                                    "❗ {}::from(anyhow::Result<Vec<u8>>) err: {}", 
                                    stringify!(#state_message_name),
                                    err,
                                ),
                            ),
                        }
                    },
                    Err(_) => <Self as frand_home_base::StateMessage>::error(
                        format!(
                            "❗ {}::from(anyhow::Result<Vec<u8>>) value: Err", 
                            stringify!(#state_message_name),
                        ),
                    ),
                }
            }
        }
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