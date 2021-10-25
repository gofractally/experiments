use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields};

fn deserialize_struct(input: &DeriveInput, data: &DataStruct) -> TokenStream2 {
    let init = match &data.fields {
        Fields::Named(named) => {
            let fields = named
                .named
                .iter()
                .map(|field| {
                    let name = &field.ident.as_ref().unwrap();
                    let ty = &field.ty;
                    quote! {
                        #name: <#ty>::eosio_deserialize(_src)?,
                    }
                })
                .fold(quote! {}, |acc, new| quote! {#acc #new});
            quote! {{#fields}}
        }
        Fields::Unnamed(unnamed) => {
            let fields = unnamed
                .unnamed
                .iter()
                .map(|field| {
                    let ty = &field.ty;
                    quote! {
                        <#ty>::eosio_deserialize(_src)?,
                    }
                })
                .fold(quote! {}, |acc, new| quote! {#acc #new});
            quote! {(#fields)}
        }
        Fields::Unit => {
            quote! {()}
        }
    };

    let name = &input.ident;
    let generics = &input.generics;
    quote! {
        impl<'a> crate::bin::EosioDeserialize<'a> for #name #generics {
            fn eosio_deserialize(_src: &mut &'a [u8]) -> std::io::Result<Self> {
                Ok(Self #init)
            }
        }
    }
} // handle_struct

fn deserialize_enum(input: &DeriveInput, data: &DataEnum) -> TokenStream2 {
    let name = &input.ident;
    let generics = &input.generics;

    let variants = data
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let vname = &v.ident;
            let init = v
                .fields
                .iter()
                .map(|field| {
                    let ty = &field.ty;
                    quote! {
                        <#ty>::eosio_deserialize(_src)?,
                    }
                })
                .fold(quote! {}, |acc, new| quote! {#acc #new});
            quote! {
                #i => #name::#vname(#init),
            }
        })
        .fold(quote! {}, |acc, new| quote! {#acc #new});

    quote! {
        impl<'a> crate::bin::EosioDeserialize<'a> for #name #generics {
            fn eosio_deserialize(_src: &mut &'a [u8]) -> std::io::Result<Self> {
                Ok(match crate::types::Varuint32::eosio_deserialize(_src)?.value as usize {
                    #variants
                    _ => Err(::std::io::Error::new(::std::io::ErrorKind::UnexpectedEof, "Invalid variant index"))?,
                })
            }
        }
    }
}

pub fn deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = match &input.data {
        Data::Struct(data) => deserialize_struct(&input, data),
        Data::Enum(data) => deserialize_enum(&input, data),
        Data::Union(_) => {
            unimplemented!("EosioDeserialize does not support union")
        }
    };
    TokenStream::from(expanded)
}
