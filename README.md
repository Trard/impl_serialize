# impl_serde_serialize_error!

This library provides a simple procedural macro for fast implementing error methods in [`serde::Serializer`] trait.

# Example
```rust
use impl_serde_serialize_error::impl_serde_serialize_error;
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
```

[`serde::Serializer`]: https://docs.rs/serde/latest/serde/trait.Serializer.html