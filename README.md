# impl_serialize!

This library provides a simple procedural macro for fast implementing serialize methods in [`serde::Serializer`] trait.

```toml
[dependencies]
impl_serialize = "2.0"
```

# Example
```rust
use impl_serialize::impl_serialize;
use serde::ser;
use thiserror::Error;

#[derive(Debug, Error)]
enum SerializationError {
    #[error("Other error")]
    OtherError,
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

struct MySerializer;

impl ser::Serializer for MySerializer {
    type Ok = ();
    type Error = SerializationError;

    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;

    //with value type
    impl_serialize!(
        |value_type: &str| {
            Err(SerializationError::CannotSerializeFrom(value_type.to_string()))
        },
        bool
    );
    
    //without value type
    impl_serialize!(
        |_| Err(SerializationError::OtherError),
        char
    );

    //for many types
    impl_serialize!(
        |value_type: &str| {
            Err(SerializationError::CannotSerializeFrom(value_type.to_string()))
        },
        [
            bytes,
            i8, i16, i32, i64,
            u8, u16, u32, u64,
            f32, f64,
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
