use std::io::{Error, ErrorKind, Read, Result};

pub fn read_u8_arr<R: Read, const SIZE: usize>(src: &mut R) -> Result<[u8; SIZE]> {
    let mut bytes: [u8; SIZE] = [0; SIZE];
    if src.read(&mut bytes)? != SIZE {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"));
    }
    Ok(bytes)
}

pub fn read_varuint32<R: Read>(src: &mut R) -> Result<u32> {
    let mut result: u32 = 0;
    let mut shift = 0;
    loop {
        if shift >= 35 {
            return Err(Error::new(ErrorKind::Other, "Bar varuint"));
        }
        let b = read_u8_arr::<R, 1>(src)?[0];
        result |= ((b & 0x7f) as u32) << shift;
        shift += 7;
        if b & 0x80 == 0 {
            break;
        }
    }
    Ok(result)
}

pub trait EosioDeserialize<'a>: Sized {
    fn eosio_deserialize(src: &mut &'a [u8]) -> Result<Self>;
}

impl<'a> EosioDeserialize<'a> for &'a [u8] {
    fn eosio_deserialize(src: &mut &'a [u8]) -> Result<Self> {
        let size = read_varuint32(src)? as usize;
        if src.len() < size {
            return Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"));
        }
        let res = &src[..size];
        *src = &src[size..];
        Ok(res)
    }
}

impl<'a, const SIZE: usize> EosioDeserialize<'a> for [u8; SIZE] {
    fn eosio_deserialize(src: &mut &'a [u8]) -> Result<Self> {
        let mut result: [u8; SIZE] = [0; SIZE];
        if src.read(&mut result)? != SIZE {
            Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))
        } else {
            Ok(result)
        }
    }
}

impl<'a, T: EosioDeserialize<'a>> EosioDeserialize<'a> for Vec<T> {
    fn eosio_deserialize(src: &mut &'a [u8]) -> Result<Self> {
        let size = read_varuint32(src)? as usize;
        // println!("[ vec: {} {}", size, std::any::type_name::<T>());
        let mut vec = Vec::<T>::with_capacity(size);
        for _ in 0..size {
            vec.push(T::eosio_deserialize(src)?);
        }
        // println!("] vec: {}", size);
        Ok(vec)
    }
}

impl<'a, T: EosioDeserialize<'a>> EosioDeserialize<'a> for Option<T> {
    fn eosio_deserialize(src: &mut &'a [u8]) -> Result<Self> {
        if <u8>::from_le_bytes(read_u8_arr(src)?) != 0 {
            Ok(Some(T::eosio_deserialize(src)?))
        } else {
            Ok(None)
        }
    }
}

impl<'a, T: EosioDeserialize<'a>> EosioDeserialize<'a> for Box<T> {
    fn eosio_deserialize(src: &mut &'a [u8]) -> Result<Self> {
        Ok(Box::new(T::eosio_deserialize(src)?))
    }
}

macro_rules! impl_eosio_deserialize_fixed {
    ($t:ty) => {
        impl<'a> EosioDeserialize<'a> for $t {
            fn eosio_deserialize(src: &mut &'a [u8]) -> Result<Self> {
                Ok(<$t>::from_le_bytes(read_u8_arr(src)?))
            }
        }
    };
}

impl<'a> EosioDeserialize<'a> for bool {
    fn eosio_deserialize(src: &mut &'a [u8]) -> Result<Self> {
        Ok(<u8>::from_le_bytes(read_u8_arr(src)?) != 0)
    }
}

impl_eosio_deserialize_fixed! {i8}
impl_eosio_deserialize_fixed! {i16}
impl_eosio_deserialize_fixed! {i32}
impl_eosio_deserialize_fixed! {i64}
impl_eosio_deserialize_fixed! {i128}
impl_eosio_deserialize_fixed! {u8}
impl_eosio_deserialize_fixed! {u16}
impl_eosio_deserialize_fixed! {u32}
impl_eosio_deserialize_fixed! {u64}
impl_eosio_deserialize_fixed! {u128}
impl_eosio_deserialize_fixed! {f32}
impl_eosio_deserialize_fixed! {f64}

#[macro_export]
macro_rules! derive_eosio_deserialize {
    (pub struct $name:ident {
        $(pub $field_name:ident: $field_type:ty,)*
    }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $field_name: $field_type,)*
        }
        impl<'a> crate::bin::EosioDeserialize<'a> for $name {
            fn eosio_deserialize(_src: &mut &'a [u8]) -> std::io::Result<Self> {
                Ok($name{
                    $($field_name: <$field_type>::eosio_deserialize(_src)?,)*
                })
            }
        }
    };

    (pub struct $name:ident<'a> {
        $(pub $field_name:ident: $field_type:ty,)*
    }) => {
        #[derive(Debug)]
        pub struct $name<'a> {
            $(pub $field_name: $field_type,)*
        }
        impl<'a> crate::bin::EosioDeserialize<'a> for $name<'a> {
            fn eosio_deserialize(src: &mut &'a [u8]) -> std::io::Result<$name<'a>> {
                let res: $name<'a> = $name{
                    $($field_name: <$field_type>::eosio_deserialize(src)?,)*
                };
                Ok(res)
            }
        }
    };

    (pub enum $name:ident {
        $($field_name:ident($field_type:ty),)*
    }) => {
        #[derive(Debug)]
        pub enum $name {
            $($field_name($field_type),)*
        }
        impl<'a> crate::bin::EosioDeserialize<'a> for $name {
            fn eosio_deserialize(src: &mut &'a [u8]) -> std::io::Result<Self> {
                // TODO: need better approach
                let fns = [
                    $(&|src: &mut &'a [u8]| -> std::io::Result<$name>{
                        Ok($name::$field_name(<$field_type>::eosio_deserialize(src)?))
                    } as &dyn Fn(&mut &'a [u8]) -> std::io::Result<$name>,)*
                ];
                fns[crate::bin::read_varuint32(src)? as usize](src)
            }
        }
    };

    (pub enum $name:ident<'a> {
        $($field_name:ident($field_type:ty),)*
    }) => {
        #[derive(Debug)]
        pub enum $name<'a> {
            $($field_name($field_type),)*
        }
        impl<'a> crate::bin::EosioDeserialize<'a> for $name<'a> {
            fn eosio_deserialize(src: &mut &'a [u8]) -> std::io::Result<Self> {
                // TODO: need better approach
                let fns = [
                    $(&|src: &mut &'a [u8]| -> std::io::Result<$name<'a>>{
                        Ok($name::$field_name(<$field_type>::eosio_deserialize(src)?))
                    } as &dyn Fn(&mut &'a [u8]) -> std::io::Result<$name<'a>>,)*
                ];
                fns[crate::bin::read_varuint32(src)? as usize](src)
            }
        }
    };
} // derive_eosio_deserialize
