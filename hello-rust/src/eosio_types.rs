use std::io::Result;

use crate::{
    derive_eosio_deserialize,
    eosio_bin::{read_varuint32, EosioDeserialize},
};

#[derive(Debug)]
pub struct BinaryExtension<T> {
    pub value: Option<T>,
}

impl<'a, T: EosioDeserialize<'a>> EosioDeserialize<'a> for BinaryExtension<T> {
    fn eosio_deserialize(src: &mut &'a [u8]) -> Result<Self> {
        if src.len() > 0 {
            Ok(Self {
                value: Some(T::eosio_deserialize(src)?),
            })
        } else {
            Ok(Self { value: None })
        }
    }
}

// A string which might contain invalid UTF-8
derive_eosio_deserialize! {
    pub struct Stringish<'a> {
        pub value: &'a [u8],
    }
}

derive_eosio_deserialize! {
    pub struct Bytes<'a> {
        pub value: &'a [u8],
    }
}

derive_eosio_deserialize! {
    pub struct Checksum256 {
        pub value: [u8; 32],
    }
}

derive_eosio_deserialize! {
    pub struct Float128 {
        pub value: u128,
    }
}

derive_eosio_deserialize! {
    pub struct Name {
        pub value: u64,
    }
}

derive_eosio_deserialize! {
    pub struct BlockTimestamp {
        pub slot: u32,
    }
}

derive_eosio_deserialize! {
    pub struct TimePoint {
        pub microseconds: i64,
    }
}

derive_eosio_deserialize! {
    pub struct TimePointSec {
        pub seconds: u32,
    }
}

#[derive(Debug)]
pub struct Varuint32 {
    pub value: u32,
}

impl<'a> EosioDeserialize<'a> for Varuint32 {
    fn eosio_deserialize(src: &mut &'a [u8]) -> Result<Self> {
        Ok(Varuint32 {
            value: read_varuint32(src)?,
        })
    }
}

pub type EccPublicKey = [u8; 33];
pub type EccSignature = [u8; 65];
pub type EccPrivateKey = [u8; 32];

derive_eosio_deserialize! {
    pub struct WebauthnPublicKey<'a> {
        pub key: EccPublicKey,
        pub user_presence: u8,
        pub rpid: Stringish<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct WebauthnSignature<'a> {
        pub compact_signature: EccSignature,
        pub auth_data: &'a [u8],
        pub client_json: Stringish<'a>,
    }
}

derive_eosio_deserialize! {
    pub enum PublicKey<'a> {
        K1(EccPublicKey),
        R1(EccPublicKey),
        Webauthn(WebauthnPublicKey<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum PrivateKey {
        K1(EccPrivateKey),
        R1(EccPrivateKey),
    }
}

derive_eosio_deserialize! {
    pub enum Signature<'a> {
        K1(EccSignature),
        R1(EccSignature),
        Webauthn(WebauthnSignature<'a>),
    }
}
