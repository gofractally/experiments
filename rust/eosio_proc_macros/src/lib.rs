use deserialize::deserialize;
use proc_macro::TokenStream;

mod deserialize;

#[proc_macro_derive(EosioDeserialize)]
pub fn derive_deserialize_eosio(input: TokenStream) -> TokenStream {
    deserialize(input)
}
