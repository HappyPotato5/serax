
use crate::{Serialize, serialize::{self}};

/// An error that showed up during serialization
#[derive(Debug, Clone)]
pub struct SerializeError(pub String);

struct Serializer {
    stream: Vec<u8>
}

/// Serializes a `Sized` object into its serial.
pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, SerializeError> {
    let mut ser = Serializer::new();
    value.serialize(&mut ser)?;
    Ok(ser.stream)
}

impl Serializer {
    fn new() -> Serializer {
        Serializer { stream: Vec::new() }
    }

    fn push_u8(&mut self, v: u8) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_u16(&mut self, v: u16) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_u32(&mut self, v: u32) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_u64(&mut self, v: u64) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_u128(&mut self, v: u128) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_usize(&mut self, v: usize) {
        self.push_u64(v as u64);
    }


    fn push_i8(&mut self, v: i8) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_i16(&mut self, v: i16) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_i32(&mut self, v: i32) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_i64(&mut self, v: i64) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_i128(&mut self, v: i128) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_isize(&mut self, v: isize) {
        self.push_i64(v as i64);
    }


    fn push_f32(&mut self, v: f32) {
        self.stream.extend(v.to_be_bytes());
    }

    fn push_f64(&mut self, v: f64) {
        self.stream.extend(v.to_be_bytes());
    }

    
    fn push_bool(&mut self, v: bool) {
        self.stream.push(v as u8);
    }
    
    fn push_char(&mut self, v: char) {
        self.stream.extend((v as u32).to_be_bytes())
    }

    fn push_str(&mut self, v: &str) {
        self.push_usize(v.len());
        self.stream.extend(v.bytes());
    }
}

impl<'a> serialize::Serializer for &'a mut Serializer {
    type Ok = ();
    type Err = SerializeError;

    // Compound type serializers
    type TupleSerializer         = TupleSerializer<'a>;
    type StructSerializer        = StructSerializer<'a>;
    type StructUnnamedSerializer = StructUnnamedSerializer<'a>;
    type EnumSerializer          = EnumSerializer<'a>;

    // Primitives
    fn serialize_u8   (self, v: u8   ) -> Result<Self::Ok, Self::Err> {
        self.push_u8(v);
        Ok(())
    }

    fn serialize_u16  (self, v: u16  ) -> Result<Self::Ok, Self::Err> {
        self.push_u16(v);
        Ok(())
    }

    fn serialize_u32  (self, v: u32  ) -> Result<Self::Ok, Self::Err> {
        self.push_u32(v);
        Ok(())
    }

    fn serialize_u64  (self, v: u64  ) -> Result<Self::Ok, Self::Err> {
        self.push_u64(v);
        Ok(())
    }

    fn serialize_u128 (self, v: u128 ) -> Result<Self::Ok, Self::Err> {
        self.push_u128(v);
        Ok(())
    }

    fn serialize_usize(self, v: usize) -> Result<Self::Ok, Self::Err> {
        self.push_usize(v);
        Ok(())
    }


    fn serialize_i8   (self, v: i8   ) -> Result<Self::Ok, Self::Err> {
        self.push_i8(v);
        Ok(())
    }

    fn serialize_i16  (self, v: i16  ) -> Result<Self::Ok, Self::Err> {
        self.push_i16(v);
        Ok(())
    }

    fn serialize_i32  (self, v: i32  ) -> Result<Self::Ok, Self::Err> {
        self.push_i32(v);
        Ok(())
    }

    fn serialize_i64  (self, v: i64  ) -> Result<Self::Ok, Self::Err> {
        self.push_i64(v);
        Ok(())
    }

    fn serialize_i128 (self, v: i128 ) -> Result<Self::Ok, Self::Err> {
        self.push_i128(v);
        Ok(())
    }

    fn serialize_isize(self, v: isize) -> Result<Self::Ok, Self::Err> {
        self.push_isize(v);
        Ok(())
    }


    fn serialize_f32(self, v: f32)   -> Result<Self::Ok, Self::Err> {
        self.push_f32(v);
        Ok(())
    }

    fn serialize_f64(self, v: f64)   -> Result<Self::Ok, Self::Err> {
        self.push_f64(v);
        Ok(())
    }


    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Err> {
        self.push_bool(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Err> {
        self.push_char(v);
        Ok(())
    }

    fn serialize_str (self, v: &str) -> Result<Self::Ok, Self::Err> {
        self.push_str(v);
        Ok(())
    }

    // Unit types
    fn serialize_unit(self) -> Result<Self::Ok, Self::Err> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &str) -> Result<Self::Ok, Self::Err> {
        Ok(())
    }

    fn serialize_newtype_struct_contents<T: Serialize>(self, _: &str, contents: &T) -> Result<Self::Ok, Self::Err>{
        contents.serialize(self)
    }

    // Compound types
    fn into_tuple_serializer(self)          -> Result<Self::TupleSerializer, Self::Err> {
        Ok(TupleSerializer {
            serializer: self
        })
    }

    fn into_struct_serializer(self, _: &str)         -> Result<Self::StructSerializer, Self::Err> {
        Ok(StructSerializer {
            serializer: self
        })
    }

    fn into_struct_unnamed_serializer(self, _: &str) -> Result<Self::StructUnnamedSerializer, Self::Err> {
        Ok(StructUnnamedSerializer {
            serializer: self
        })
    }

    fn into_enum_serializer(self, _: &str)           -> Result<Self::EnumSerializer, Self::Err> {
        Ok(EnumSerializer {
            serializer: self
        })
    }

    // List-like objects
    fn serialize_array<T: crate::Serialize, const C: usize>(self, array: &[T; C]) -> Result<Self::Ok, Self::Err> {
        self.push_usize(C);
        for el in array {
            el.serialize(&mut *self)?;
        }
        Ok(())
    }

    fn serialize_vec<T: crate::Serialize>(self, list: &Vec<T>) -> Result<Self::Ok, Self::Err> {
        self.push_usize(list.len());
        for el in list {
            el.serialize(&mut *self)?;
        }
        Ok(())
    }

    fn serialize_boxed_slice<T: crate::Serialize>(self, list: &Box<[T]>) -> Result<Self::Ok, Self::Err> {
        self.push_usize(list.len());
        for el in list {
            el.serialize(&mut *self)?;
        }
        Ok(())
    }

    fn serialize_map<K: crate::Serialize, V: crate::Serialize>(self, map: &Vec<(K, V)>) -> Result<Self::Ok, Self::Err> {
        self.push_usize(map.len());
        for (k, v) in map {
            k.serialize(&mut *self)?;
            v.serialize(&mut *self)?;
        }
        Ok(())
    }

    fn serialize_boxed<T: Serialize>(self, value: &Box<T>) -> Result<Self::Ok, Self::Err> {
        value.serialize(self)
    }

    fn serialize_option<T: crate::Serialize>(self, option: &Option<T>) -> Result<Self::Ok, Self::Err> {
        self.serialize_bool(option.is_some())?;
        match option {
            Some(t) => t.serialize(&mut *self)?,
            None => (),
        }
        Ok(())
    }

    fn serialize_result<O: crate::Serialize, E: crate::Serialize>(self, result: &Result<O, E>) -> Result<Self::Ok, Self::Err> {
        self.serialize_bool(result.is_ok())?;
        match result {
            Ok(o) => o.serialize(&mut *self)?,
            Err(e) => e.serialize(&mut *self)?,
        }
        Ok(())
    }
}


struct TupleSerializer<'a> {
    serializer: &'a mut Serializer
}

impl serialize::TupleSerializer for TupleSerializer<'_> {
    fn serialize_element<T: crate::Serialize>(&mut self, element: &T) -> Result<(), Self::Err> {
        element.serialize(&mut *self.serializer)
    }
}

impl serialize::SubSerializer for TupleSerializer<'_> {
    type Ok = ();
    type Err = SerializeError;

    fn end(self) -> Result<Self::Ok, Self::Err> {
        Ok(())
    }
}


struct StructSerializer<'a> {
    serializer: &'a mut Serializer
}

impl serialize::StructSerializer for StructSerializer<'_> {
    fn serialize_field<T: crate::Serialize>(&mut self, _: &str, value: &T) -> Result<(), Self::Err> {
        value.serialize(&mut *self.serializer)
    }
}

impl serialize::SubSerializer for StructSerializer<'_> {
    type Ok = ();
    type Err = SerializeError;

    fn end(self) -> Result<Self::Ok, Self::Err> {
        Ok(())
    }
}


struct StructUnnamedSerializer<'a> {
    serializer: &'a mut Serializer
}

impl serialize::StructUnnamedSerializer for StructUnnamedSerializer<'_> {
    fn serialize_unnamed_field<T: crate::Serialize>(&mut self, value: &T) -> Result<(), Self::Err> {
        value.serialize(&mut *self.serializer)
    }
}

impl serialize::SubSerializer for StructUnnamedSerializer<'_> {
    type Ok = ();
    type Err = SerializeError;

    fn end(self) -> Result<Self::Ok, Self::Err> {
        Ok(())
    }
}


struct EnumSerializer<'a> {
    serializer: &'a mut Serializer
}

impl<'a> serialize::EnumSerializer for EnumSerializer<'a> {
    type MainSerializer = &'a mut Serializer;

    fn serialize_unit_variant(self, name: &str) -> Result<Self::Ok, Self::Err> {
        <&mut Serializer as serialize::Serializer>::serialize_str(&mut *self.serializer, name)
    }

    fn serialize_newtype_variant<T: crate::Serialize>(self, name: &str, value: &T) -> Result<Self::Ok, Self::Err> {
        <&mut Serializer as serialize::Serializer>::serialize_str(&mut *self.serializer, name)?;
        value.serialize(&mut *self.serializer)
    }

    fn serialize_tuple_variant(self, name: &str) -> Result<<Self::MainSerializer as serialize::Serializer>::TupleSerializer, Self::Err> {
        <&mut Serializer as serialize::Serializer>::serialize_str(&mut *self.serializer, name)?;
        <Self::MainSerializer as serialize::Serializer>::into_tuple_serializer(self.serializer)
    }

    fn serialize_struct_variant(self, name: &str) -> Result<<Self::MainSerializer as serialize::Serializer>::StructSerializer, Self::Err> {
        <&mut Serializer as serialize::Serializer>::serialize_str(&mut *self.serializer, name)?;
        <Self::MainSerializer as serialize::Serializer>::into_struct_serializer(self.serializer, "")
    }
}

impl serialize::SubSerializer for EnumSerializer<'_> {
    type Ok = ();
    type Err = SerializeError;

    fn end(self) -> Result<Self::Ok, Self::Err> {
        todo!()
    }
}





// Impls for primitive types
impl Serialize for u8 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_u8(*self)
    }
}

impl Serialize for u16 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_u16(*self)
    }
}

impl Serialize for u32 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_u32(*self)
    }
}

impl Serialize for u64 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_u64(*self)
    }
}

impl Serialize for u128 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_u128(*self)
    }
}

impl Serialize for usize {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_usize(*self)
    }
}


impl Serialize for i8 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_i8(*self)
    }
}

impl Serialize for i16 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_i16(*self)
    }
}

impl Serialize for i32 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_i32(*self)
    }
}

impl Serialize for i64 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_i64(*self)
    }
}

impl Serialize for i128 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_i128(*self)
    }
}

impl Serialize for isize {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_isize(*self)
    }
}


impl Serialize for f32 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_f32(*self)
    }
}

impl Serialize for f64 {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_f64(*self)
    }
}


impl Serialize for bool {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_bool(*self)
    }
}

impl Serialize for char {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_char(*self)
    }
}

impl Serialize for String {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_str(self)
    }
}


impl<T: Serialize> Serialize for Option<T> {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_option(self)
    }
}

impl<O: Serialize, E: Serialize> Serialize for Result<O, E> {
    fn serialize<T: serialize::Serializer>(&self, serializer: T) -> Result<T::Ok, T::Err> {
        serializer.serialize_result(self)
    }
}

impl<T: Serialize, const C: usize> Serialize for [T; C] {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_array(self)
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_vec(self)
    }
}

impl<T: Serialize> Serialize for Box<[T]> {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_boxed_slice(self)
    }
}

impl<T: Serialize> Serialize for Box<T> {
    fn serialize<S: serialize::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err> {
        serializer.serialize_boxed(self)
    }
}