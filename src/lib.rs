//! Crate deprecated. Use newer and more universal [impl_serialize!](https://crates.io/crates/impl_serialize) instead.
//! This library provides a simple procedural macro for fast implementing error methods in
//! [serde::Serializer](https://docs.rs/serde/latest/serde/trait.Serializer.html) trait.

/// Implements error method for [serde::Serializer](https://docs.rs/serde/latest/serde/trait.Serializer.html)
/// 
/// # Examples
///
/// ```
/// use impl_serde_serialize_error::impl_serde_serialize_error;
/// use serde::ser;
/// # use thiserror::Error;
/// # 
/// # #[derive(Debug, Error)]
/// # enum SerializationError {
/// #     #[error("Cannot serialize value")]
/// #     CannotSerialize,
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
///     //for one type
///     impl_serde_serialize_error!(SerializationError::CannotSerialize, bool);
/// 
///     //for many types
///     impl_serde_serialize_error!(SerializationError::CannotSerialize, [bytes, i8, i16]);
/// #
/// # impl_serde_serialize_error!(
/// #     SerializationError::CannotSerialize, [
/// #         i32, i64,
/// #         u8, u16, u32, u64,
/// #         f32, f64,
/// #         char,
/// #         str,
/// #         none, some, unit,
/// #         unit_struct, unit_variant,
/// #         newtype_struct, newtype_variant,
/// #         seq, map,
/// #         tuple, tuple_struct, tuple_variant,
/// #         struct, struct_variant
/// #     ]
/// # );
/// }
/// 
/// ```
#[macro_export]
macro_rules! impl_serde_serialize_error {
    ($error:expr, [$($type:ident),+]) => {
        $(
            impl_serde_serialize_error!($error, $type);
        )+
    };

    ($error:expr, bool) => {
        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, i8) => {
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, i16) => {
        fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, i32) => {
        fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, i64) => {
        fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, u8) => {
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, u16) => {
        fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, u32) => {
        fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, u64) => {
        fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, f32) => {
        fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, f64) => {
        fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, char) => {
        fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, str) => {
        fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, bytes) => {
        fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, none) => {
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, unit) => {
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, some) => {
        fn serialize_some<T: ?Sized + serde::ser::Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, unit_struct) => {
        fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, unit_variant) => {
        fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, newtype_struct) => {
        fn serialize_newtype_struct<T: ?Sized + serde::ser::Serialize>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, newtype_variant) => {
        fn serialize_newtype_variant<T: ?Sized + serde::ser::Serialize>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, seq) => {
        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, tuple) => {
        fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, tuple_struct) => {
        fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, tuple_variant) => {
        fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, map) => {
        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, struct) => {
        fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
            Err($error)
        }
    };

    ($error:expr, struct_variant) => {
        fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
            Err($error)
        }
    };
}

#[cfg(test)]
mod tests {
    use serde::ser;
    use thiserror::Error;

    #[derive(Debug, Error)]
    enum SerializationError {
        #[error("Cannot serialize value")]
        CannotSerialize,
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
    fn all_errors() {
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
            
            impl_serde_serialize_error!(
                SerializationError::CannotSerialize, [
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
                ]
            );
        }
    }
}