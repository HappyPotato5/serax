use std::{collections::VecDeque, mem::MaybeUninit};

use crate::deserialize::{Deserialize, self};

/// An error that showed during Deserialization.
/// 
/// Usually because the buffer wasn't long enough.
#[derive(Debug, Clone)]
pub struct DeserializeError(pub String);

struct Deserializer {
    stream: VecDeque<u8>
}

/// Deserializes a `Sized` object from its serial.
pub fn from_bytes<T: Deserialize>(bytes: Vec<u8>) -> Result<T, DeserializeError> {
    let mut deser = Deserializer::new(bytes);
    T::deserialize(&mut deser)
}

impl Deserializer {
    fn new(data: Vec<u8>) -> Deserializer {
        Deserializer { stream: VecDeque::from(data) }
    }

    fn extract<T: Deserialize, const C: usize>(&mut self) -> Result<[T; C], DeserializeError> {
        if C > self.stream.len() { return Err(DeserializeError("Not enough bytes".to_string())); }

        let mut result = [const {MaybeUninit::uninit()}; C];

        for i in 0..C {
            let t = T::deserialize(&mut *self)?;
            result[i].write(t);
        }

        Ok(result.map(|x| unsafe {x.assume_init()}))
    }

    fn extract_bytes<const C: usize>(&mut self) -> Result<[u8; C], DeserializeError> {
        if C > self.stream.len() { return Err(DeserializeError("Not enough bytes".to_string())); }

        let mut result = [const {MaybeUninit::uninit()}; C];
        let mut d = self.stream.drain(0..C);

        for i in 0..C {
            result[i].write(d.next().ok_or_else(|| DeserializeError("Not enough bytes left".to_string()))?);
        }

        Ok(result.map(|x| unsafe {x.assume_init()}))
    }

    fn get_bytes(&mut self, len: usize) -> Result<Vec<u8>, DeserializeError> {
        if len > self.stream.len() { return Err(DeserializeError("Not enough bytes".to_string())); }

        Ok(self.stream.drain(0..len).collect())
    }

    fn take_u8(&mut self) -> Result<u8, DeserializeError> {
        Ok(u8::from_be_bytes(self.extract_bytes::<1>()?))
    }

    fn take_u16(&mut self) -> Result<u16, DeserializeError> {
        Ok(u16::from_be_bytes(self.extract_bytes::<2>()?))
    }

    fn take_u32(&mut self) -> Result<u32, DeserializeError> {
        Ok(u32::from_be_bytes(self.extract_bytes::<4>()?))
    }

    fn take_u64(&mut self) -> Result<u64, DeserializeError> {
        Ok(u64::from_be_bytes(self.extract_bytes::<8>()?))
    }

    fn take_u128(&mut self) -> Result<u128, DeserializeError> {
        Ok(u128::from_be_bytes(self.extract_bytes::<16>()?))
    }

    fn take_usize(&mut self) -> Result<usize, DeserializeError> {
        Ok(self.take_u64()? as usize)
    }


    fn take_i8(&mut self) -> Result<i8, DeserializeError> {
        Ok(i8::from_be_bytes(self.extract_bytes::<1>()?))
    }

    fn take_i16(&mut self) -> Result<i16, DeserializeError> {
        Ok(i16::from_be_bytes(self.extract_bytes::<2>()?))
    }

    fn take_i32(&mut self) -> Result<i32, DeserializeError> {
        Ok(i32::from_be_bytes(self.extract_bytes::<4>()?))
    }

    fn take_i64(&mut self) -> Result<i64, DeserializeError> {
        Ok(i64::from_be_bytes(self.extract_bytes::<8>()?))
    }

    fn take_i128(&mut self) -> Result<i128, DeserializeError> {
        Ok(i128::from_be_bytes(self.extract_bytes::<16>()?))
    }

    fn take_isize(&mut self) -> Result<isize, DeserializeError> {
        Ok(self.take_i64()? as isize)
    }


    fn take_f32(&mut self) -> Result<f32, DeserializeError> {
        Ok(f32::from_be_bytes(self.extract_bytes::<4>()?))
    }

    fn take_f64(&mut self) -> Result<f64, DeserializeError> {
        Ok(f64::from_be_bytes(self.extract_bytes::<8>()?))
    }


    fn take_bool(&mut self) -> Result<bool, DeserializeError> {
        Ok(self.take_u8()? != 0)
    }

    fn take_char(&mut self) -> Result<char, DeserializeError> {
        char::from_u32(self.take_u32()?).ok_or_else(|| DeserializeError("Invalid char found".to_string()))
    }

    fn take_string(&mut self) -> Result<String, DeserializeError> {
        let len = self.take_usize()?;
        let bytes = self.get_bytes(len)?;

        String::from_utf8(bytes).map_err(|_| DeserializeError("Invalid utf8 string".to_string()))
    }
}

impl<'a> deserialize::Deserializer for &'a mut Deserializer {
    type Err = DeserializeError;

    type TupleDeserializer         = TupleDeserializer<'a>;
    type StructDeserializer        = StructDeserializer<'a>;
    type StructUnnamedDeserializer = StructUnnamedDeserializer<'a>;
    type EnumDeserializer          = EnumDeserializer<'a>;

    // Primitive types
    fn deserialize_u8   (self) -> Result<u8   , Self::Err> {
        self.take_u8()
    }

    fn deserialize_u16  (self) -> Result<u16  , Self::Err> {
        self.take_u16()
    }

    fn deserialize_u32  (self) -> Result<u32  , Self::Err> {
        self.take_u32()
    }

    fn deserialize_u64  (self) -> Result<u64  , Self::Err> {
        self.take_u64()
    }

    fn deserialize_u128 (self) -> Result<u128 , Self::Err> {
        self.take_u128()
    }

    fn deserialize_usize(self) -> Result<usize, Self::Err> {
        self.take_usize()
    }

    fn deserialize_i8   (self) -> Result<i8   , Self::Err> {
        self.take_i8()
    }

    fn deserialize_i16  (self) -> Result<i16  , Self::Err> {
        self.take_i16()
    }

    fn deserialize_i32  (self) -> Result<i32  , Self::Err> {
        self.take_i32()
    }

    fn deserialize_i64  (self) -> Result<i64  , Self::Err> {
        self.take_i64()
    }

    fn deserialize_i128 (self) -> Result<i128 , Self::Err> {
        self.take_i128()
    }

    fn deserialize_isize(self) -> Result<isize, Self::Err> {
        self.take_isize()
    }

    fn deserialize_f32 (self)   -> Result<f32, Self::Err> {
        self.take_f32()
    }

    fn deserialize_f64 (self)   -> Result<f64, Self::Err> {
        self.take_f64()
    }

    fn deserialize_bool(self) -> Result<bool, Self::Err> {
        self.take_bool()
    }

    fn deserialize_char(self) -> Result<char, Self::Err> {
        self.take_char()
    }

    fn deserialize_string(self) -> Result<String, Self::Err> {
        self.take_string()
    }

    // Unit types
    fn deserialize_unit(self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn deserialize_unit_struct(self, _: &str) -> Result<(), Self::Err> {
        Ok(())
    }

    fn deserialize_newtype_struct_contents<T: Deserialize>(self, _: &str) -> Result<T, Self::Err> {
        T::deserialize(self)
    }

    // Compound types
    fn into_tuple_deserializer(self)          -> Result<Self::TupleDeserializer, Self::Err> {
        Ok(TupleDeserializer {
            deserializer: self
        })
    }

    fn into_struct_deserializer(self, _: &str)         -> Result<Self::StructDeserializer, Self::Err> {
        Ok(StructDeserializer {
            deserializer: self
        })
    }

    fn into_struct_unnamed_deserializer(self, _: &str) -> Result<Self::StructUnnamedDeserializer, Self::Err> {
        Ok(StructUnnamedDeserializer {
            deserializer: self
        })
    }

    fn into_enum_deserializer(self, _: &str)           -> Result<Self::EnumDeserializer, Self::Err> {
        Ok(EnumDeserializer {
            deserializer: self
        })
    }

    // List-like objects
    fn deserialize_array<T: Deserialize, const C: usize>(self) -> Result<[T; C], Self::Err> {
        self.extract::<T, C>()
    }

    fn deserialize_vec<T: Deserialize>(self) -> Result<Vec<T>, Self::Err> {
        let len = self.take_usize()?;
        let mut result = Vec::with_capacity(len);

        for _ in 0..len {
            result.push(T::deserialize(&mut *self)?);
        }

        Ok(result)
    }

    fn deserialize_boxed_slice<T: Deserialize>(self) -> Result<Box<[T]>, Self::Err> {
        Ok(self.deserialize_vec()?.into_boxed_slice())
    }

    fn deserialize_map<K: Deserialize, V: Deserialize>(self) -> Result<Vec<(K, V)>, Self::Err> {
        let len = self.take_usize()?;
        let mut result = Vec::with_capacity(len);

        for _ in 0..len {
            let k = K::deserialize(&mut *self)?;
            let v = V::deserialize(&mut *self)?;
            result.push((k, v));
        }

        Ok(result)
    }

    // Other
    fn serialize_boxed<T: Deserialize>(self) -> Result<Box<T>, Self::Err> {
        Ok(Box::new(T::deserialize(self)?))
    }

    fn deserialize_option<T: Deserialize>(self) -> Result<Option<T>, Self::Err> {
        let is_some = self.take_bool()?;

        Ok(if is_some {
            Some(T::deserialize(&mut *self)?)
        } else {
            None
        })
    }

    fn deserialize_result<O: Deserialize, E: Deserialize>(self) -> Result<Result<O, E>, Self::Err> {
        let is_ok = self.take_bool()?;

        Ok(if is_ok {
            Ok(O::deserialize(&mut *self)?)
        } else {
            Err(E::deserialize(&mut *self)?)
        })
    }
}


struct TupleDeserializer<'a> {
    deserializer: &'a mut Deserializer
}

impl deserialize::TupleDeserializer for TupleDeserializer<'_> {
    fn deserialize_element<T: Deserialize>(&mut self) -> Result<T, Self::Err> {
        T::deserialize(&mut *self.deserializer)
    }
}

impl deserialize::SubDeserializer for TupleDeserializer<'_> {
    type Err = DeserializeError;

    fn end(self) -> Result<(), Self::Err> {
        Ok(())
    }
}


struct StructDeserializer<'a> {
    deserializer: &'a mut Deserializer
}

impl deserialize::StructDeserializer for StructDeserializer<'_> {
    fn deserialize_field<T: Deserialize>(&mut self, _: &str) -> Result<T, Self::Err> {
        T::deserialize(&mut *self.deserializer)
    }
}

impl deserialize::SubDeserializer for StructDeserializer<'_> {
    type Err = DeserializeError;

    fn end(self) -> Result<(), Self::Err> {
        Ok(())
    }
}


struct StructUnnamedDeserializer<'a> {
    deserializer: &'a mut Deserializer
}

impl deserialize::StructUnnamedDeserializer for StructUnnamedDeserializer<'_> {
    fn deserialize_unnamed_field<T: Deserialize>(&mut self) -> Result<T, Self::Err> {
        T::deserialize(&mut *self.deserializer)
    }
}

impl deserialize::SubDeserializer for StructUnnamedDeserializer<'_> {
    type Err = DeserializeError;

    fn end(self) -> Result<(), Self::Err> {
        Ok(())
    }
}


struct EnumDeserializer<'a> {
    deserializer: &'a mut Deserializer
}

impl<'a> deserialize::EnumDeserializer for EnumDeserializer<'a> {
    type MainDeserializer = &'a mut Deserializer;

    fn deserialize_variant_ident(&mut self) -> Result<String, Self::Err> {
        <&mut Deserializer as deserialize::Deserializer>::deserialize_string(&mut *self.deserializer)
    }

    fn deserialize_unit_variant(self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn deserialize_newtype_variant_contents<T: Deserialize>(self) -> Result<T, Self::Err> {
        T::deserialize(&mut *self.deserializer)
    }

    fn deserialize_tuple_variant(self) -> Result<<Self::MainDeserializer as deserialize::Deserializer>::TupleDeserializer, Self::Err> {
        <Self::MainDeserializer as deserialize::Deserializer>::into_tuple_deserializer(self.deserializer)
    }

    fn deserialize_struct_variant(self) -> Result<<Self::MainDeserializer as deserialize::Deserializer>::StructDeserializer, Self::Err> {
        <Self::MainDeserializer as deserialize::Deserializer>::into_struct_deserializer(self.deserializer, "")
    }
}

impl deserialize::SubDeserializer for EnumDeserializer<'_> {
    type Err = DeserializeError;

    fn end(self) -> Result<(), Self::Err> {
        Ok(())
    }
}



impl Deserialize for u8 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_u8()
    }
}

impl Deserialize for u16 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_u16()
    }
}

impl Deserialize for u32 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_u32()
    }
}

impl Deserialize for u64 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_u64()
    }
}

impl Deserialize for u128 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_u128()
    }
}

impl Deserialize for usize {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_usize()
    }
}


impl Deserialize for i8 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_i8()
    }
}

impl Deserialize for i16 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_i16()
    }
}

impl Deserialize for i32 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_i32()
    }
}

impl Deserialize for i64 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_i64()
    }
}

impl Deserialize for i128 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_i128()
    }
}

impl Deserialize for isize {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_isize()
    }
}


impl Deserialize for f32 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_f32()
    }
}

impl Deserialize for f64 {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_f64()
    }
}


impl Deserialize for bool {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_bool()
    }
}

impl Deserialize for char {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_char()
    }
}

impl Deserialize for String {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_string()
    }
}


impl<T: Deserialize, const C: usize> Deserialize for [T; C] {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_array()
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_vec()
    }
}

impl<T: Deserialize> Deserialize for Box<[T]> {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_boxed_slice()
    }
}


impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize<S: deserialize::Deserializer>(deserializer: S) -> Result<Self, S::Err> {
        deserializer.deserialize_option()
    }
}

impl<O: Deserialize, E: Deserialize> Deserialize for Result<O, E> {
    fn deserialize<T: deserialize::Deserializer>(deserializer: T) -> Result<Self, T::Err> {
        deserializer.deserialize_result()
    }
}