
/// A trait implemented by all `Sized` types that can be reconstructed from a **serial** of themselves.
/// 
/// A **serial** is a flat stream of text or bytes.
/// 
/// ## The following types are Deserializable by default:
/// + u8, u16, u32, u64, u128, usize.
/// + i8, i16, i32, i64, i128, isize.
/// + f32, f64.
/// + bool.
/// + char.
/// + String.
/// + `Option<T>` where T: `Deserialize`.
/// + `Result<O, E>` where O: `Deserialize`, E: `Deserialize`.
/// + Arrays -> [T; const C: usize] where T: `Deserialize`.
/// + `Vec<T>` where T: `Deserialize`.
/// + `Box<[T]>` where T: `Deserialize`.
pub trait Deserialize 
    where Self: Sized {
    /// Reconstructs a `Sized` `Self` from its **serial** form.
    fn deserialize<T: Deserializer>(deserializer: T) -> Result<Self, T::Err>;
}

pub trait Deserializer 
    where Self: Sized {
    type Err;


    // SubDeserializers for compound types
    type TupleDeserializer         : TupleDeserializer          <Err = Self::Err>;
    type StructDeserializer        : StructDeserializer         <Err = Self::Err>;
    type StructUnnamedDeserializer : StructUnnamedDeserializer  <Err = Self::Err>;
    type EnumDeserializer          : EnumDeserializer           <MainDeserializer = Self, Err = Self::Err>;


    // Scalars ---------------
    /// Deserializes an u8.
    fn deserialize_u8   (self) -> Result<u8   , Self::Err>;

    /// Deserializes an u16.
    fn deserialize_u16  (self) -> Result<u16  , Self::Err>;

    /// Deserializes an u32.
    fn deserialize_u32  (self) -> Result<u32  , Self::Err>;

    /// Deserializes an u64.
    fn deserialize_u64  (self) -> Result<u64  , Self::Err>;

    /// Deserializes an u128.
    fn deserialize_u128 (self) -> Result<u128 , Self::Err>;

    /// Deserializes an usize.
    fn deserialize_usize(self) -> Result<usize, Self::Err>;


    /// Deserializes an i8.
    fn deserialize_i8   (self) -> Result<i8   , Self::Err>;

    /// Deserializes an i16.
    fn deserialize_i16  (self) -> Result<i16  , Self::Err>;

    /// Deserializes an i32.
    fn deserialize_i32  (self) -> Result<i32  , Self::Err>;

    /// Deserializes an i64.
    fn deserialize_i64  (self) -> Result<i64  , Self::Err>;

    /// Deserializes an i128.
    fn deserialize_i128 (self) -> Result<i128 , Self::Err>;

    /// Deserializes an isize.
    fn deserialize_isize(self) -> Result<isize, Self::Err>;


    /// Deserializes an f32.
    fn deserialize_f32 (self)   -> Result<f32, Self::Err>;

    /// Deserializes an f64.
    fn deserialize_f64 (self)   -> Result<f64, Self::Err>;


    /// Deserializes a bool.
    fn deserialize_bool(self) -> Result<bool, Self::Err>;

    /// Deserializes a char.
    fn deserialize_char(self) -> Result<char, Self::Err>;

    /// Deserializes a String.
    fn deserialize_string(self) -> Result<String, Self::Err>;


    // Unit types ---------------
    /// Deserializes and returns the `unit type` from its serial form.
    fn deserialize_unit(self) -> Result<(), Self::Err>;

    /// Returns `Ok(())` if the unit struct with the given `name` was found in the **serial**.
    /// 
    /// The [`deserialize()`](Deserialize::deserialize()) method is responsible for building and returning the **instance**.
    fn deserialize_unit_struct(self, name: &str) -> Result<(), Self::Err>;

    /// Returns Ok(newtype_contents) if the newtype struct serial, and it's contents were found.
    /// 
    /// The [`deserialize()`](Deserialize::deserialize()) method is responsible for building and returning the **instance**.
    fn deserialize_newtype_struct_contents<T: Deserialize>(self, name: &str) -> Result<T, Self::Err>;


    // Compound types ---------------
    /// Converts the [`Deserializer`] into a [`TupleDeserializer`].
    fn into_tuple_deserializer(self)                      -> Result<Self::TupleDeserializer, Self::Err>;

    /// Converts the [`Deserializer`] into a [`StructDeserializer`], for structs with named fields.
    fn into_struct_deserializer(self, name: &str)         -> Result<Self::StructDeserializer, Self::Err>;

    /// Converts the [`Deserializer`] into a [`StructUnnamedDeserializer`], for structs with unnamed fields.
    fn into_struct_unnamed_deserializer(self, name: &str) -> Result<Self::StructUnnamedDeserializer, Self::Err>;

    /// Converts the [`Deserializer`] into an [`EnumDeserializer`].
    fn into_enum_deserializer(self, name: &str)           -> Result<Self::EnumDeserializer, Self::Err>;


    // List-like types ---------------
    /// Deserializes a const-sized array of [`Deserializable`](Deserialize) elements.
    fn deserialize_array<T: Deserialize, const C: usize>(self) -> Result<[T; C], Self::Err>;

    /// Deserializes a vec of [`Deserializable`](Deserialize) elements.
    fn deserialize_vec<T: Deserialize>(self) -> Result<Vec<T>, Self::Err>;

    /// Deserializes a boxed slice of [`Deserializable`](Deserialize) elements.
    fn deserialize_boxed_slice<T: Deserialize>(self) -> Result<Box<[T]>, Self::Err>;


    // Map ---------------
    /// Deserializes a map of K-V, key-value pairs where K and V are both [`Deserializable`](Deserialize) types.
    fn deserialize_map<K: Deserialize, V: Deserialize>(self) -> Result<Vec<(K, V)>, Self::Err>;


    // Option ---------------
    /// Deserializes an optional [`Deserializable`](Deserialize) type.
    fn deserialize_option<T: Deserialize>(self) -> Result<Option<T>, Self::Err>;


    // Result ---------------
    /// Deserializes a Result<O, E> where O and E are both [`Deserializable`](Deserialize) types.
    fn deserialize_result<O: Deserialize, E: Deserialize>(self) -> Result<Result<O, E>, Self::Err>;
}

/// A SubDeserializer is responsible for the deserialization of the elements of Compound types such as structs, enums...
pub trait SubDeserializer {
    type Err;

    /// Ends the `SubSerializer`.
    fn end(self) -> Result<(), Self::Err>;
}

/// A `TupleSerializer` is able to deserialize tuple elements **in order**, **one at a time**.
pub trait TupleDeserializer: SubDeserializer {
    /// Deserilizes the next tuple element.
    fn deserialize_element<T: Deserialize>(&mut self) -> Result<T, Self::Err>;
}

/// A `StructDeserializer` is able to deserialize **named fields** from a struct **in order**, **one at a time**.
pub trait StructDeserializer: SubDeserializer {
    /// Deserializes the next named field of a struct.
    fn deserialize_field<T: Deserialize>(&mut self, field_name: &str) -> Result<T, Self::Err>;
}

/// A `StructUnnamedDeserializer` is able to deserialize **unnamed fields** from a struct **in order**, **one at a time**.
pub trait StructUnnamedDeserializer: SubDeserializer {
    /// Deserializes the next unnamed field of the struct.
    fn deserialize_unnamed_field<T: Deserialize>(&mut self) -> Result<T, Self::Err>;
}

/// An `EnumDeserializer` is able to deserialize an enum depending of the **variant** stored in the **serial**.
/// 
/// ## How to use:
/// First use [`Self::deserialize_variant_ident()`] to get the **identifier** of the stored **variant**.
/// 
/// After that, based on the deserialized **identifier**, use the corresponding method to deserialize the **variant**.
///  + [`Self::deserialize_unit_variant()`] for unit variants.
///  + [`Self::deserialize_newtype_variant_contents()`] for newtype variants contents.
///  + [`Self::deserialize_tuple_variant()`] for tuple variants.
///  + [`Self::deserialize_struct_variant()`] for struct variants.
pub trait EnumDeserializer: SubDeserializer {
    type MainDeserializer: Deserializer;

    /// Deserializes the variant's **identifier**.
    fn deserialize_variant_ident(&mut self) -> Result<String, Self::Err>;

    /// Deserializes a unit variant.
    /// 
    /// Returns `Ok(())` if there's a unit variant.
    /// 
    /// Building the Unit variant and returning it is the responsibility of [`Deserialize::deserialize()`].
    fn deserialize_unit_variant(self) -> Result<(), Self::Err>;

    /// Deserializes the contents of a newtype variant.
    /// 
    /// Returns `Ok(contents)` if the newtype variant **contents** are found.
    fn deserialize_newtype_variant_contents<T: Deserialize>(self) -> Result<T, Self::Err>;

    /// Deserializes a tuple variant, transforming into a [`TupleDeserializer`].
    fn deserialize_tuple_variant(self) -> Result<<Self::MainDeserializer as Deserializer>::TupleDeserializer, Self::Err>;

    /// Deserializes a struct variant, transforming into a [`StructDeserializer`].
    fn deserialize_struct_variant(self) -> Result<<Self::MainDeserializer as Deserializer>::StructDeserializer, Self::Err>;
}