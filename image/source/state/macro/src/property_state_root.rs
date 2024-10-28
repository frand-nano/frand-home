use proc_macro2::TokenStream;
use syn::Ident;
use quote::quote;

pub fn property_state_root(
    state_name: &Ident, 
) -> TokenStream {
    let state_property_name = {
        let state_message_name = format!("{state_name}Property");
        Ident::new(&state_message_name, state_name.span())
    };
    let state_message_name = {
        let state_message_name = format!("{state_name}Message");
        Ident::new(&state_message_name, state_name.span())
    };

    quote! {
        impl Default for #state_property_name {
            fn default() -> Self {
                <Self as frand_home_state::StateProperty>::new_default(vec![])
            }
        }

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
                            Err(err) => <Self as frand_home_state::StateMessage>::error(
                                format!(
                                    "❗ {}::from(anyhow::Result<String>) err: {}, value: {}", 
                                    stringify!(#state_message_name),
                                    err,
                                    value,
                                ),
                            ),
                        }
                    },
                    Err(_) => <Self as frand_home_state::StateMessage>::error(
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
                            Err(err) => <Self as frand_home_state::StateMessage>::error(
                                format!(
                                    "❗ {}::from(anyhow::Result<Vec<u8>>) err: {}", 
                                    stringify!(#state_message_name),
                                    err,
                                ),
                            ),
                        }
                    },
                    Err(_) => <Self as frand_home_state::StateMessage>::error(
                        format!(
                            "❗ {}::from(anyhow::Result<Vec<u8>>) value: Err", 
                            stringify!(#state_message_name),
                        ),
                    ),
                }
            }
        }
    }
}