# Function's args

Every generated function have input arguments. And you can use this args inside your expression.

For details of specific input args see [`serde::Serializer`].

# Example
```rust
use thiserror::Error;
use serde::ser;
use ser::Serializer;
use impl_serialize::impl_serialize;

#[derive(Debug, Error, PartialEq)]
enum SerializationError {
    #[error("Expected value higher {than}. Found {current_value}")]
    ExpectedValueHigher {
        than: i64,
        current_value: i64
    },
    #[error("Cannot serialize")]
    CannotSerialize,
    #[error("Custom({0})")]
    Custom(String),
}

impl serde::ser::Error for SerializationError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        SerializationError::Custom(msg.to_string())
    }
}
#[derive(Clone, Copy)]
struct MoreThanOneSerializer;

impl ser::Serializer for MoreThanOneSerializer {
    type Error = SerializationError;
    type Ok = i64;

    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;

    impl_serialize!(
        {
            let v = v as i64;
            if v > 1 {
                Ok(v)
            } else {
                Err(SerializationError::ExpectedValueHigher {
                    than: 1,
                    current_value: v,
                })
            }
        },
        [i8, i16, i32, i64, u8, u16, u32, u64, f32, f64]
    );

    impl_serialize!(Err(SerializationError::CannotSerialize), [
        bool, bytes, char, str,
        none, some, unit,
        unit_struct, unit_variant,
        newtype_struct, newtype_variant,
        seq, map,
        tuple, tuple_struct, tuple_variant,
        struct, struct_variant
    ]);
}

let more_than_one_serializer = MoreThanOneSerializer;

assert_eq!(
    more_than_one_serializer.serialize_i32(4).ok().unwrap(),
    4
);

assert_eq!(
    more_than_one_serializer.serialize_i32(71).ok().unwrap(),
    71
);

assert_eq!(
    more_than_one_serializer.serialize_i32(0).err().unwrap(),
    SerializationError::ExpectedValueHigher {
        than: 1,
        current_value: 0,
    }
);
```

# Special value_type metavariable

Every generated function have variable `value_type: &str` inside it. You can use it as well. `value_type` equals to type after `fn serialize_`.

| function name             | value_type      |
|---------------------------|-----------------|
| serialize_bool            | bool            |
| serialize_i8              | i8              |
| serialize_i16             | i16             |
| serialize_i32             | i32             |
| serialize_i64             | i64             |
| serialize_u8              | u8              |
| serialize_u16             | u16             |
| serialize_u32             | u32             |
| serialize_u64             | u64             |
| serialize_f32             | f32             |
| serialize_f64             | f64             |
| serialize_char            | char            |
| serialize_str             | str             |
| serialize_bytes           | bytes           |
| serialize_none            | none            |
| serialize_some            | some            |
| serialize_unit            | unit            |
| serialize_unit_struct     | unit_struct     |
| serialize_unit_variant    | unit_variant    |
| serialize_newtype_struct  | newtype_struct  |
| serialize_newtype_variant | newtype_variant |
| serialize_seq             | seq             |
| serialize_tuple           | tuple           |
| serialize_tuple_struct    | tuple_struct    |
| serialize_tuple_variant   | tuple_variant   |
| serialize_map             | map             |
| serialize_struct          | struct          |
| serialize_struct_variant  | struct_variant  |

# Example
```rust
use thiserror::Error;
use serde::ser;
use ser::Serializer;
use impl_serialize::impl_serialize;

#[derive(Debug, Error, PartialEq)]
enum SerializationError {
    #[error("Cannot serialize from({0})")]
    CannotSerializeFrom(String),
    #[error("Custom({0})")]
    Custom(String),
}

impl serde::ser::Error for SerializationError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        SerializationError::Custom(msg.to_string())
    }
}

#[derive(Clone, Copy)]
struct CharSerializer;

impl ser::Serializer for CharSerializer {
    type Error = SerializationError;
    type Ok = char;

    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;

    impl_serialize!(Ok(v), char);

    impl_serialize!(Err(SerializationError::CannotSerializeFrom(value_type.to_string())), [
        i8, i16, i32, i64,
        u8, u16, u32, u64,
        f32, f64,
        bool,
        bytes,
        str,
        none, some, unit,
        unit_struct, unit_variant,
        newtype_struct, newtype_variant,
        seq, map,
        tuple, tuple_struct, tuple_variant,
        struct, struct_variant
    ]);
}

let char_serializer = CharSerializer;

assert_eq!(
    CharSerializer.serialize_char('k').ok().unwrap(),
    'k'
);

assert_eq!(
    char_serializer.serialize_char('h').ok().unwrap(),
    'h'
);

assert_eq!(
    char_serializer.serialize_i8(0).err().unwrap(),
    SerializationError::CannotSerializeFrom("i8".to_string())
);

assert_eq!(
    char_serializer.serialize_none().err().unwrap(),
    SerializationError::CannotSerializeFrom("none".to_string())
);
```

[More examples...](../src/tests/metavariables.rs)

[`serde::serializer`]: https://docs.rs/serde/latest/serde/trait.Serializer.html
