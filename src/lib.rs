//! This library provides a simple procedural macro for fast implementing serialize methods in
//! [serde::Serializer](https://docs.rs/serde/latest/serde/trait.Serializer.html) trait.

pub use unhygienic2::unhygienic;

/// Implements serialize methods for [serde::Serializer](https://docs.rs/serde/latest/serde/trait.Serializer.html)
/// 
/// # Examples
///
/// ```
/// use impl_serialize::impl_serialize;
/// use serde::ser;
/// # use thiserror::Error;
/// # 
/// # #[derive(Debug, Error)]
/// # enum SerializationError {
/// #     #[error("Other error")]
/// #     OtherError,
/// #     #[error("Cannot serialize value from {0}")]
/// #     CannotSerializeFrom(String),
/// #     #[error("Custom({0})")]
/// #     Custom(String)
/// # }
/// # 
/// # impl serde::ser::Error for SerializationError {
/// #     fn custom<T>(msg:T) -> Self
/// #     where T: std::fmt::Display
/// #     {
/// #         SerializationError::Custom(msg.to_string())
/// #     }
/// # }
/// # 
/// struct MySerializer;
/// 
/// impl ser::Serializer for MySerializer {
/// #    type Ok = ();
/// #    type Error = SerializationError;
/// #
/// #   type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
/// #   type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
/// #   type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
/// #   type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;
/// #   type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
/// #   type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
/// #   type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
/// #
///     //value_type is metavariable (&str) what represents any serializing value type.
///     //for example, value_type will be "i8" when seializing i8 or "bytes" when &[u8] (bytes);
/// 
///     //with value_type
///     impl_serialize!(
///         Err(SerializationError::CannotSerializeFrom(value_type.to_string())),
///         bool
///     );
///     
///     //without value_type
///     impl_serialize!(
///         Err(SerializationError::OtherError),
///         char
///     );
/// 
///     //for many types
///     impl_serialize!(
///         Err(SerializationError::CannotSerializeFrom(value_type.to_string())),
///         [
///             bytes,
///             i8, i16, i32, i64,
///             // other types
/// #           u8, u16, u32, u64,
/// #           f32, f64,
/// #           str,
/// #           none, some, unit,
/// #           unit_struct, unit_variant,
/// #           newtype_struct, newtype_variant,
/// #           seq, map,
/// #           tuple, tuple_struct, tuple_variant,
/// #           struct, struct_variant
///         ]
///     );
/// }
/// ```
#[macro_export]
macro_rules! impl_serialize {
    ($get_result:expr, [$($type:ident),+]) => {
        $(
            impl_serialize!($get_result, $type);
        )+
    };

    ($get_result:expr, bool) => {
        impl_serialize::unhygienic! {
            fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
                let value_type = "bool";

                $get_result
            }
        }
    };

    ($get_result:expr, i8) => {
        impl_serialize::unhygienic! {
            fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
                let value_type = "i8";
                
                $get_result
            }
        }
    };

    ($get_result:expr, i16) => {
        impl_serialize::unhygienic! {
            fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
                let value_type = "i16";

                $get_result
            }
        }
    };

    ($get_result:expr, i32) => {
        impl_serialize::unhygienic! {
            fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
                let value_type = "i32";

                $get_result
            }
        }
    };

    ($get_result:expr, i64) => {
        impl_serialize::unhygienic! {
            fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
                let value_type = "i64";
                
                $get_result
            }
        }
    };

    ($get_result:expr, u8) => {
        impl_serialize::unhygienic! {
            fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
                let value_type = "u8";

                $get_result
            }
        }
    };

    ($get_result:expr, u16) => {
        impl_serialize::unhygienic! {
            fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
                let value_type = "u16";

                $get_result
            }
        }
    };

    ($get_result:expr, u32) => {
        impl_serialize::unhygienic! {
            fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
                let value_type = "u32";

                $get_result
            }
        }
    };

    ($get_result:expr, u64) => {
        impl_serialize::unhygienic! {
            fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
                let value_type = "u64";

                $get_result
            }
        }
    };

    ($get_result:expr, f32) => {
        impl_serialize::unhygienic! {
            fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
                let value_type = "f32";

                $get_result
            }
        }
    };

    ($get_result:expr, f64) => {
        impl_serialize::unhygienic! {
            fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
                let value_type = "f64";

                $get_result
            }
        }
    };

    ($get_result:expr, char) => {
        impl_serialize::unhygienic! {
            fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
                let value_type = "char";

                $get_result
            }
        }
    };

    ($get_result:expr, str) => {
        impl_serialize::unhygienic! {
            fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
                let value_type = "str";

                $get_result
            }
        }
    };

    ($get_result:expr, bytes) => {
        impl_serialize::unhygienic! {
            fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
                let value_type = "bytes";
                
                $get_result
            }
        }
    };

    ($get_result:expr, none) => {
        impl_serialize::unhygienic! {
            fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
                let value_type = "none";

                $get_result
            }
        }
    };

    ($get_result:expr, unit) => {
        impl_serialize::unhygienic! {
            fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
                let value_type = "unit";

                $get_result
            }
        }
    };

    ($get_result:expr, some) => {
        impl_serialize::unhygienic! {
            fn serialize_some<T: ?Sized + serde::ser::Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
                let value_type = "some";

                $get_result
            }
        }
    };

    ($get_result:expr, unit_struct) => {
        impl_serialize::unhygienic! {
            fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
                let value_type = "unit_struct";

                $get_result
            }
        }
    };

    ($get_result:expr, unit_variant) => {
        impl_serialize::unhygienic! {
            fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
                let value_type = "unit_variant";
                
                $get_result
            }
        }
    };

    ($get_result:expr, newtype_struct) => {
        impl_serialize::unhygienic! {
            fn serialize_newtype_struct<T: ?Sized + serde::ser::Serialize>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
                let value_type = "newtype_struct";
                
                $get_result
            }
        }
    };

    ($get_result:expr, newtype_variant) => {
        impl_serialize::unhygienic! {
            fn serialize_newtype_variant<T: ?Sized + serde::ser::Serialize>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> {
                let value_type = "newtype_variant";
                
                $get_result
            }
        }
    };

    ($get_result:expr, seq) => {
        impl_serialize::unhygienic! {
            fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
                let value_type = "seq";
                
                $get_result
            }
        }
    };

    ($get_result:expr, tuple) => {
        impl_serialize::unhygienic! {
            fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
                let value_type = "tuple";

                $get_result
            }
        }
    };

    ($get_result:expr, tuple_struct) => {
        impl_serialize::unhygienic! {
            fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
                let value_type = "tuple_struct";

                $get_result
            }
        }
    };

    ($get_result:expr, tuple_variant) => {
        impl_serialize::unhygienic! {
            fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
                let value_type = "tuple_variant";

                $get_result
            }
        }
    };

    ($get_result:expr, map) => {
        impl_serialize::unhygienic! {
            fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
                let value_type = "map";

                $get_result
            }
        }
    };

    ($get_result:expr, struct) => {
        impl_serialize::unhygienic! {
            fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
                let value_type = "struct";

                $get_result
            }
        }
    };

    ($get_result:expr, struct_variant) => {
        impl_serialize::unhygienic! {
            fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
                let value_type = "struct_variant";

                $get_result
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use serde::ser;
    use thiserror::Error;
    use crate as impl_serialize;

    #[derive(Debug, Error)]
    enum SerializationError {
        #[error("Cannot serialize value from {0}")]
        CannotSerializeFrom(String),
        #[error("Custom({0})")]
        Custom(String)
    }

    impl serde::ser::Error for SerializationError {
        fn custom<T>(msg:T) -> Self
        where T: std::fmt::Display
        {
            SerializationError::Custom(msg.to_string())
        }
    }

    #[test]
    fn all() {
        struct MySerializer;

        impl ser::Serializer for MySerializer {
            type Error = SerializationError;
            type Ok = ();

            type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
            type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
            type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
            type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;
            type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
            type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
            type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
            
            impl_serialize!(Err(SerializationError::CannotSerializeFrom(value_type.to_string())), [
                bool,
                bytes,
                i8, i16, i32, i64,
                u8, u16, u32, u64,
                f32, f64,
                char,
                str,
                none, some, unit,
                unit_struct, unit_variant,
                newtype_struct, newtype_variant,
                seq, map,
                tuple, tuple_struct, tuple_variant,
                struct, struct_variant
            ]);
        }
    }
}