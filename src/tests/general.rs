use super::*;

#[test]
fn all() {
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
