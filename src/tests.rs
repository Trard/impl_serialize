use crate as impl_serialize;
use serde::{ser, Serializer};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
enum SerializationError {
    #[error("Expected value higher {than}. Found {current_value}")]
    ExpectedValueHigher {
        than: i64,
        current_value: i64
    },
    #[error("Cannot serialize")]
    CannotSerialize,
    #[error("Cannot serialize value from char {0}")]
    CannotSerializeFromChar(char),
    #[error("Cannot serialize value from {0}")]
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

mod general {
    use super::*;

    #[test]
    fn all_value_type() {
        #[derive(Clone, Copy)]
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

            impl_serialize!(Err(SerializationError::CannotSerialize), [
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

        let serializer = MySerializer;

        assert_eq!(
            serializer.serialize_bool(true).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_i8(4).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_i16(1).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_i32(1).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_i64(4).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_u8(4).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_u16(4).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_u32(4).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_u64(5).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_f32(4.3).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_f64(5.1).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_char('f').err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_str("str").err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_bytes(&[1, 2, 3, 4]).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_none().err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_some(&1).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_unit().err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_unit_struct("struct").err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer
                .serialize_unit_variant("struct", 5, "variant")
                .err()
                .unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer
                .serialize_newtype_struct("struct", "value")
                .err()
                .unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer
                .serialize_newtype_variant("struct", 5, "varint", "value")
                .err()
                .unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_seq(None).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_tuple(5).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_tuple_struct("name", 5).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer
                .serialize_tuple_variant("struct", 1, "varint", 5)
                .err()
                .unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_map(None).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer.serialize_struct("name", 1).err().unwrap(),
            SerializationError::CannotSerialize
        );

        assert_eq!(
            serializer
                .serialize_struct_variant("struct", 1, "varint", 5)
                .err()
                .unwrap(),
            SerializationError::CannotSerialize
        );
    }
}

mod metavariables {
    use super::*;

    #[test]
    fn all_value_type() {
        #[derive(Clone, Copy)]
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

        let serializer = MySerializer;

        assert_eq!(
            serializer.serialize_bool(true).err().unwrap(),
            SerializationError::CannotSerializeFrom("bool".to_string())
        );

        assert_eq!(
            serializer.serialize_i8(4).err().unwrap(),
            SerializationError::CannotSerializeFrom("i8".to_string())
        );

        assert_eq!(
            serializer.serialize_i16(1).err().unwrap(),
            SerializationError::CannotSerializeFrom("i16".to_string())
        );

        assert_eq!(
            serializer.serialize_i32(1).err().unwrap(),
            SerializationError::CannotSerializeFrom("i32".to_string())
        );

        assert_eq!(
            serializer.serialize_i64(4).err().unwrap(),
            SerializationError::CannotSerializeFrom("i64".to_string())
        );

        assert_eq!(
            serializer.serialize_u8(4).err().unwrap(),
            SerializationError::CannotSerializeFrom("u8".to_string())
        );

        assert_eq!(
            serializer.serialize_u16(4).err().unwrap(),
            SerializationError::CannotSerializeFrom("u16".to_string())
        );

        assert_eq!(
            serializer.serialize_u32(4).err().unwrap(),
            SerializationError::CannotSerializeFrom("u32".to_string())
        );

        assert_eq!(
            serializer.serialize_u64(5).err().unwrap(),
            SerializationError::CannotSerializeFrom("u64".to_string())
        );

        assert_eq!(
            serializer.serialize_f32(4.3).err().unwrap(),
            SerializationError::CannotSerializeFrom("f32".to_string())
        );

        assert_eq!(
            serializer.serialize_f64(5.1).err().unwrap(),
            SerializationError::CannotSerializeFrom("f64".to_string())
        );

        assert_eq!(
            serializer.serialize_char('f').err().unwrap(),
            SerializationError::CannotSerializeFrom("char".to_string())
        );

        assert_eq!(
            serializer.serialize_str("str").err().unwrap(),
            SerializationError::CannotSerializeFrom("str".to_string())
        );

        assert_eq!(
            serializer.serialize_bytes(&[1, 2, 3, 4]).err().unwrap(),
            SerializationError::CannotSerializeFrom("bytes".to_string())
        );

        assert_eq!(
            serializer.serialize_none().err().unwrap(),
            SerializationError::CannotSerializeFrom("none".to_string())
        );

        assert_eq!(
            serializer.serialize_some(&1).err().unwrap(),
            SerializationError::CannotSerializeFrom("some".to_string())
        );

        assert_eq!(
            serializer.serialize_unit().err().unwrap(),
            SerializationError::CannotSerializeFrom("unit".to_string())
        );

        assert_eq!(
            serializer.serialize_unit_struct("struct").err().unwrap(),
            SerializationError::CannotSerializeFrom("unit_struct".to_string())
        );

        assert_eq!(
            serializer
                .serialize_unit_variant("struct", 5, "variant")
                .err()
                .unwrap(),
            SerializationError::CannotSerializeFrom("unit_variant".to_string())
        );

        assert_eq!(
            serializer
                .serialize_newtype_struct("struct", "value")
                .err()
                .unwrap(),
            SerializationError::CannotSerializeFrom("newtype_struct".to_string())
        );

        assert_eq!(
            serializer
                .serialize_newtype_variant("struct", 5, "varint", "value")
                .err()
                .unwrap(),
            SerializationError::CannotSerializeFrom("newtype_variant".to_string())
        );

        assert_eq!(
            serializer.serialize_seq(None).err().unwrap(),
            SerializationError::CannotSerializeFrom("seq".to_string())
        );

        assert_eq!(
            serializer.serialize_tuple(5).err().unwrap(),
            SerializationError::CannotSerializeFrom("tuple".to_string())
        );

        assert_eq!(
            serializer.serialize_tuple_struct("name", 5).err().unwrap(),
            SerializationError::CannotSerializeFrom("tuple_struct".to_string())
        );

        assert_eq!(
            serializer
                .serialize_tuple_variant("struct", 1, "varint", 5)
                .err()
                .unwrap(),
            SerializationError::CannotSerializeFrom("tuple_variant".to_string())
        );

        assert_eq!(
            serializer.serialize_map(None).err().unwrap(),
            SerializationError::CannotSerializeFrom("map".to_string())
        );

        assert_eq!(
            serializer.serialize_struct("name", 1).err().unwrap(),
            SerializationError::CannotSerializeFrom("struct".to_string())
        );

        assert_eq!(
            serializer
                .serialize_struct_variant("struct", 1, "varint", 5)
                .err()
                .unwrap(),
            SerializationError::CannotSerializeFrom("struct_variant".to_string())
        );
    }

    #[test]
    fn char() {
        #[derive(Clone, Copy)]
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

            impl_serialize!(Err(SerializationError::CannotSerializeFromChar(v)), char);

            //other
            impl_serialize!(Err(SerializationError::CannotSerialize), [
                bool,
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
            ]);
        }

        let serializer = MySerializer;

        assert_eq!(
            serializer.serialize_char('h').err().unwrap(),
            SerializationError::CannotSerializeFromChar('h')
        );
    }

    #[test]
    fn v_more_than_one() {
        #[derive(Clone, Copy)]
        struct MySerializer;

        impl ser::Serializer for MySerializer {
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
                            current_value: v
                        })
                    }
                },
                [
                    i8, i16, i32, i64,
                    u8, u16, u32, u64,
                    f32, f64
                ]
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
    }
}