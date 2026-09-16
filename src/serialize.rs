
/// A trait implemented by all **`Sized` types** that can be converted into a **serial**.
/// 
/// A **serial** is a flat stream of text or bytes.
/// 
/// ## The following types are Serilizable by default:
///  + u8, u16, u32, u64, u128, usize.
///  + i8, i16, i32, i64, i128, isize.
///  + f32, f64.
///  + bool.
///  + char.
///  + String.
///  + `Option<T>` where T: `Serialize`.
///  + `Result<O, E>` where O: `Serialize`, E: `Serialize`.
///  + Arrays -> [T; const C: usize] where T: `Serialize`.
///  + `Vec<T>` where T: `Serialize`.
///  + `Box<T>` where T: `Serialize`.
///  + `Box<[T]>` where T: `Serialize`.
pub trait Serialize {
    /// Creates a serial out of a `Sized` type and a [`Serializer`].
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Err>;
}

/// An trait that controls how a Serializable object is turned into a serial.
/// 
/// The Serializer is responsible for the outputted format, and for checking the correctness of the output.
/// 
/// A Serializer can be converted into a [`SubSerializer`], In order to handle Structs, Tuples...
pub trait Serializer {
    type Ok;
    type Err;

    // SubSerializers for compound types
    type TupleSerializer         : TupleSerializer          <Ok = Self::Ok, Err = Self::Err>;
    type StructSerializer        : StructSerializer         <Ok = Self::Ok, Err = Self::Err>;
    type StructUnnamedSerializer : StructUnnamedSerializer  <Ok = Self::Ok, Err = Self::Err>;
    type EnumSerializer          : EnumSerializer           <Ok = Self::Ok, MainSerializer = Self, Err = Self::Err>;


    // Scalars ---------------
    /// Serializes an u8.
    fn serialize_u8   (self, v: u8   ) -> Result<Self::Ok, Self::Err>;

    /// Serializes an u16.
    fn serialize_u16  (self, v: u16  ) -> Result<Self::Ok, Self::Err>;

    /// Serializes an u32.
    fn serialize_u32  (self, v: u32  ) -> Result<Self::Ok, Self::Err>;

    /// Serializes an u64.
    fn serialize_u64  (self, v: u64  ) -> Result<Self::Ok, Self::Err>;

    /// Serializes an u128.
    fn serialize_u128 (self, v: u128 ) -> Result<Self::Ok, Self::Err>;

    /// Serializes an usize.
    fn serialize_usize(self, v: usize) -> Result<Self::Ok, Self::Err>;


    /// Serializes an i8.
    fn serialize_i8   (self, v: i8   ) -> Result<Self::Ok, Self::Err>;

    /// Serializes an i16.
    fn serialize_i16  (self, v: i16  ) -> Result<Self::Ok, Self::Err>;

    /// Serializes an i32.
    fn serialize_i32  (self, v: i32  ) -> Result<Self::Ok, Self::Err>;

    /// Serializes an i64.
    fn serialize_i64  (self, v: i64  ) -> Result<Self::Ok, Self::Err>;

    /// Serializes an i128.
    fn serialize_i128 (self, v: i128 ) -> Result<Self::Ok, Self::Err>;

    /// Serializes an isize.
    fn serialize_isize(self, v: isize) -> Result<Self::Ok, Self::Err>;


    /// Serializes a f32.
    fn serialize_f32(self, v: f32)   -> Result<Self::Ok, Self::Err>;

    /// Serializes a f64.
    fn serialize_f64(self, v: f64)   -> Result<Self::Ok, Self::Err>;


    /// Serializes a bool.
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Err>;

    /// Serializes a char.
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Err>;

    /// Serializes a str.
    fn serialize_str (self, v: &str) -> Result<Self::Ok, Self::Err>;


    // Unit types ---------------
    /// Serializes the unit type `()`.
    fn serialize_unit(self) -> Result<Self::Ok, Self::Err>;

    /// Serializes a unit struct, such as ```struct Example```.
    fn serialize_unit_struct(self, name: &str) -> Result<Self::Ok, Self::Err>;

    /// Serializes a newtype struct's contents, such as ```struct Example(usize)```.
    fn serialize_newtype_struct_contents<T: Serialize>(self, name: &str, contents: &T) -> Result<Self::Ok, Self::Err>;


    // Compound types ---------------
    /// Converts the [`Serializer`] into a [`TupleSerializer`].
    fn into_tuple_serializer(self)                      -> Result<Self::TupleSerializer, Self::Err>;

    /// Converts the [`Serializer`] into a [`StructSerializer`].
    /// 
    /// For structs with named fields.
    fn into_struct_serializer(self, name: &str)         -> Result<Self::StructSerializer, Self::Err>;

    /// Converts the [`Serializer`] into a [`StructUnnamedSerializer`].
    /// 
    /// For structs with unnamed fields.
    fn into_struct_unnamed_serializer(self, name: &str) -> Result<Self::StructUnnamedSerializer, Self::Err>;

    /// Converts the [`Serializer`] into a [`EnumSerializer`].
    fn into_enum_serializer(self, name: &str)           -> Result<Self::EnumSerializer, Self::Err>;


    // List-like types ---------------
    /// Serializes a constant-size array of [`Serializable`](Serialize) objects.
    fn serialize_array<T: Serialize, const C: usize>(self, array: &[T; C]) -> Result<Self::Ok, Self::Err>;
    
    /// Serializes a Vec of [`Serializable`](Serialize) objects.
    fn serialize_vec<T: Serialize>(self, list: &Vec<T>) -> Result<Self::Ok, Self::Err>;

    /// Serializes a boxed slice of of [`Serializable`](Serialize) objects.
    fn serialize_boxed_slice<T: Serialize>(self, list: &Box<[T]>) -> Result<Self::Ok, Self::Err>;


    // Map ---------------
    /// Serializes a Map of K-V, key-value pairs, where K and V are both [`Serializable`](Serialize) types.
    fn serialize_map<K: Serialize, V: Serialize>(self, map: &Vec<(K, V)>) -> Result<Self::Ok, Self::Err>;


    // Box ---------------
    /// Serialize a Boxed T object, where T: [`Serialize`].
    fn serialize_boxed<T: Serialize>(self, value: &Box<T>) -> Result<Self::Ok, Self::Err>;


    // Option ---------------
    /// Serializes an optional [`Serializable`](Serialize) object.
    fn serialize_option<T: Serialize>(self, option: &Option<T>) -> Result<Self::Ok, Self::Err>;

    
    // Result ---------------
    /// Serializes a Result<O, E> whwre O, E are both [`Serializable`](Serialize) types.
    fn serialize_result<O: Serialize, E: Serialize>(self, result: &Result<O, E>) -> Result<Self::Ok, Self::Err>;
}

/// A `SubSerializer` that handles the serialization for a concrete structure, like a struct, enum...
pub trait SubSerializer {
    type Ok;
    type Err;

    /// Ends the subserializer, potentially returning an error.
    fn end(self) -> Result<Self::Ok, Self::Err>;
}

/// A [`SubSerializer`] that handles Tuple serialization.
pub trait TupleSerializer: SubSerializer {
    /// Serializes an element of a tuple.
    fn serialize_element<T: Serialize>(&mut self, element: &T) -> Result<(), Self::Err>;
}

/// A [`SubSerializer`] that handles struct serialization.
/// 
/// For structs with named fields.
pub trait StructSerializer: SubSerializer {
    /// Serializes a named field of a struct.
    fn serialize_field<T: Serialize>(&mut self, field_name: &str, value: &T) -> Result<(), Self::Err>;
}

/// A [`SubSerializer`] that handles struct serialization.
/// 
/// For structs with unnamed fields.
pub trait StructUnnamedSerializer: SubSerializer {
    /// Serializes an unnamed field of a struct.
    fn serialize_unnamed_field<T: Serialize>(&mut self, value: &T) -> Result<(), Self::Err>;
}

/// A [`SubSerializer`] that handles enum serialization.
pub trait EnumSerializer: SubSerializer {
    type MainSerializer: Serializer;

    /// Serializes an unit variant.
    fn serialize_unit_variant(self, name: &str) -> Result<Self::Ok, Self::Err>;

    /// Serializes a newtype variant, such as ```ExampleEnum::NewTypeVariantExample(usize),```.
    fn serialize_newtype_variant<T: Serialize>(self, name: &str, value: &T) -> Result<Self::Ok, Self::Err>;

    /// Serializes a tuple variant, such as ```ExampleEnum::TupleVariantExample(usize, String),```.
    fn serialize_tuple_variant(self, name: &str) -> Result<<Self::MainSerializer as Serializer>::TupleSerializer, Self::Err>;

    /// Serializes a struct variant, such as ```ExampleEnum::StructVariantExample{x: f32, y: f32},```.
    fn serialize_struct_variant(self, name: &str) -> Result<<Self::MainSerializer as Serializer>::StructSerializer, Self::Err>;
}