use proc_macro2::{Span, TokenStream};
use syn::Ident;
use quote::quote;

pub fn node_state_root(
    state_name: &Ident,
) -> TokenStream {
    let node_name = Ident::new("Node", Span::mixed_site());
    let message_name = Ident::new("Message", Span::mixed_site());
    
    quote! {
        impl Default for #node_name {
            fn default() -> Self {
                <Self as frand_home_node::Node<#state_name>>::new_default(vec![], None)
            }
        }

        impl frand_home_node::RootMessage for #message_name {
            fn error(err: String) -> Self { Self::Error(err) }
        }

        impl TryFrom<#message_name> for String {
            type Error = anyhow::Error;

            fn try_from(value: #message_name) -> std::result::Result<Self, Self::Error> {
                Ok(serde_json::to_string_pretty(&value)?)
            }
        }

        impl TryFrom<&bytestring::ByteString> for #message_name {
            type Error = serde_json::Error;

            fn try_from(value: &bytestring::ByteString) -> std::result::Result<Self, serde_json::Error> {
                serde_json::from_str(value)
            }
        }

        impl From<anyhow::Result<String>> for #message_name {
            fn from(value: anyhow::Result<String>) -> Self {
                match value {
                    Ok(value) => {
                        match serde_json::from_str(&value) {
                            Ok(result) => result,
                            Err(err) => <Self as frand_home_node::RootMessage>::error(
                                format!(
                                    "❗ {}::from(anyhow::Result<String>) err: {}, value: {}", 
                                    stringify!(#message_name),
                                    err,
                                    value,
                                ),
                            ),
                        }
                    },
                    Err(_) => <Self as frand_home_node::RootMessage>::error(
                        format!(
                            "❗ {}::from(anyhow::Result<String>) value: Err", 
                            stringify!(#message_name),
                        ),
                    ),
                }
            }
        }

        impl From<anyhow::Result<Vec<u8>>> for #message_name {
            fn from(value: anyhow::Result<Vec<u8>>) -> Self {
                match value {
                    Ok(value) => {
                        match serde_json::from_slice(&value) {
                            Ok(result) => result,
                            Err(err) => <Self as frand_home_node::RootMessage>::error(
                                format!(
                                    "❗ {}::from(anyhow::Result<Vec<u8>>) err: {}", 
                                    stringify!(#message_name),
                                    err,
                                ),
                            ),
                        }
                    },
                    Err(_) => <Self as frand_home_node::RootMessage>::error(
                        format!(
                            "❗ {}::from(anyhow::Result<Vec<u8>>) value: Err", 
                            stringify!(#message_name),
                        ),
                    ),
                }
            }
        }
    }
}