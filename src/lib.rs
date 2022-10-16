//! This library provides a simple procedural macro for fast and easy implementing serialize methods in
//! [serde::Serializer](https://docs.rs/serde/latest/serde/trait.Serializer.html) trait.
//! # Example
#![doc = include_str!("../docs/example.md")]
#![doc = include_str!("../docs/metavariables.md")]

pub use unhygienic2::unhygienic;

/// Macro for fast and easy implementing serialize methods in 
/// [serde::Serializer](https://docs.rs/serde/latest/serde/trait.Serializer.html) trait.
/// # Example
#[doc = include_str!("../docs/example.md")]
#[macro_export]
macro_rules! impl_serialize {
    ($get_result:expr, [$($type:ident),+]) => {
        $(
            impl_serialize!($get_result, $type);
        )+
    };

    ($get_result:expr, bool) => {
        impl_serialize::unhygienic! {
            fn serialize_bool(self, v: bool) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "bool";

                $get_result
            }
        }
    };

    ($get_result:expr, i8) => {
        impl_serialize::unhygienic! {
            fn serialize_i8(self, v: i8) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "i8";
                
                $get_result
            }
        }
    };

    ($get_result:expr, i16) => {
        impl_serialize::unhygienic! {
            fn serialize_i16(self, v: i16) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "i16";

                $get_result
            }
        }
    };

    ($get_result:expr, i32) => {
        impl_serialize::unhygienic! {
            fn serialize_i32(self, v: i32) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "i32";

                $get_result
            }
        }
    };

    ($get_result:expr, i64) => {
        impl_serialize::unhygienic! {
            fn serialize_i64(self, v: i64) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "i64";
                
                $get_result
            }
        }
    };

    ($get_result:expr, u8) => {
        impl_serialize::unhygienic! {
            fn serialize_u8(self, v: u8) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "u8";

                $get_result
            }
        }
    };

    ($get_result:expr, u16) => {
        impl_serialize::unhygienic! {
            fn serialize_u16(self, v: u16) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "u16";

                $get_result
            }
        }
    };

    ($get_result:expr, u32) => {
        impl_serialize::unhygienic! {
            fn serialize_u32(self, v: u32) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "u32";

                $get_result
            }
        }
    };

    ($get_result:expr, u64) => {
        impl_serialize::unhygienic! {
            fn serialize_u64(self, v: u64) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "u64";

                $get_result
            }
        }
    };

    ($get_result:expr, f32) => {
        impl_serialize::unhygienic! {
            fn serialize_f32(self, v: f32) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "f32";

                $get_result
            }
        }
    };

    ($get_result:expr, f64) => {
        impl_serialize::unhygienic! {
            fn serialize_f64(self, v: f64) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "f64";

                $get_result
            }
        }
    };

    ($get_result:expr, char) => {
        impl_serialize::unhygienic! {
            fn serialize_char(self, v: char) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "char";

                $get_result
            }
        }
    };

    ($get_result:expr, str) => {
        impl_serialize::unhygienic! {
            fn serialize_str(self, v: &str) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "str";

                $get_result
            }
        }
    };

    ($get_result:expr, bytes) => {
        impl_serialize::unhygienic! {
            fn serialize_bytes(self, v: &[u8]) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "bytes";
                
                $get_result
            }
        }
    };

    ($get_result:expr, none) => {
        impl_serialize::unhygienic! {
            fn serialize_none(self) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "none";

                $get_result
            }
        }
    };

    ($get_result:expr, unit) => {
        impl_serialize::unhygienic! {
            fn serialize_unit(self) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "unit";

                $get_result
            }
        }
    };

    ($get_result:expr, some) => {
        impl_serialize::unhygienic! {
            fn serialize_some<T: ?Sized + serde::ser::Serialize>(self, value: &T) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "some";

                $get_result
            }
        }
    };

    ($get_result:expr, unit_struct) => {
        impl_serialize::unhygienic! {
            fn serialize_unit_struct(self, name: &'static str) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "unit_struct";

                $get_result
            }
        }
    };

    ($get_result:expr, unit_variant) => {
        impl_serialize::unhygienic! {
            fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "unit_variant";
                
                $get_result
            }
        }
    };

    ($get_result:expr, newtype_struct) => {
        impl_serialize::unhygienic! {
            fn serialize_newtype_struct<T: ?Sized + serde::ser::Serialize>(self, name: &'static str, value: &T) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "newtype_struct";
                
                $get_result
            }
        }
    };

    ($get_result:expr, newtype_variant) => {
        impl_serialize::unhygienic! {
            fn serialize_newtype_variant<T: ?Sized + serde::ser::Serialize>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> core::result::Result<Self::Ok, Self::Error> {
                let value_type = "newtype_variant";
                
                $get_result
            }
        }
    };

    ($get_result:expr, seq) => {
        impl_serialize::unhygienic! {
            fn serialize_seq(self, len: Option<usize>) -> core::result::Result<Self::SerializeSeq, Self::Error> {
                let value_type = "seq";
                
                $get_result
            }
        }
    };

    ($get_result:expr, tuple) => {
        impl_serialize::unhygienic! {
            fn serialize_tuple(self, len: usize) -> core::result::Result<Self::SerializeTuple, Self::Error> {
                let value_type = "tuple";

                $get_result
            }
        }
    };

    ($get_result:expr, tuple_struct) => {
        impl_serialize::unhygienic! {
            fn serialize_tuple_struct(self, name: &'static str, len: usize) -> core::result::Result<Self::SerializeTupleStruct, Self::Error> {
                let value_type = "tuple_struct";

                $get_result
            }
        }
    };

    ($get_result:expr, tuple_variant) => {
        impl_serialize::unhygienic! {
            fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> core::result::Result<Self::SerializeTupleVariant, Self::Error> {
                let value_type = "tuple_variant";

                $get_result
            }
        }
    };

    ($get_result:expr, map) => {
        impl_serialize::unhygienic! {
            fn serialize_map(self, len: Option<usize>) -> core::result::Result<Self::SerializeMap, Self::Error> {
                let value_type = "map";

                $get_result
            }
        }
    };

    ($get_result:expr, struct) => {
        impl_serialize::unhygienic! {
            fn serialize_struct(self, name: &'static str, len: usize) -> core::result::Result<Self::SerializeStruct, Self::Error> {
                let value_type = "struct";

                $get_result
            }
        }
    };

    ($get_result:expr, struct_variant) => {
        impl_serialize::unhygienic! {
            fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> core::result::Result<Self::SerializeStructVariant, Self::Error> {
                let value_type = "struct_variant";

                $get_result
            }
        }
    };
}

#[cfg(test)]
mod tests;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;
