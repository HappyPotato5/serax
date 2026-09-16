
use serax::{deserialize::{EnumDeserializer, StructDeserializer, StructUnnamedDeserializer, TupleDeserializer}, serialize::{EnumSerializer, StructSerializer, StructUnnamedSerializer, SubSerializer, TupleSerializer}, *};



#[derive(Debug)]
struct Test {
    num: u32,
    b: bool,
    s: String,
    t: isize,
}

impl Default for Test {
    fn default() -> Self {
        Self { num: 10, b: false, s: "Mi nombre es encarna".to_string(), t: -32 }
    }
}

impl Serialize for Test {
    fn serialize<T: serialize::Serializer>(&self, serializer: T) -> Result<T::Ok, T::Err> {
        let mut ser = serializer.into_struct_serializer("Test")?;
        ser.serialize_field("num", &self.num)?;
        ser.serialize_field("b", &self.b)?;
        ser.serialize_field("s", &self.s)?;
        ser.serialize_field("t", &self.t)?;
        ser.end() // deprecated
    }
}

impl Deserialize for Test {
    fn deserialize<T: deserialize::Deserializer>(deserializer: T) -> Result<Self, T::Err> {
        let mut deser = deserializer.into_struct_deserializer("Test")?;
        let num: u32 = deser.deserialize_field("num")?;
        let b: bool = deser.deserialize_field("b")?;
        let s: String = deser.deserialize_field("s")?;
        let t: isize = deser.deserialize_field("t")?;
        // deser.end()?; deprecated

        Ok(Test {
            num, b, s, t
        })
    }
}

#[test]
fn manual_struct_ser_deser() {
    let test = Test::default();
    let test_out = format!("{:?}", test);

    assert_eq!(test_out, "Test { num: 10, b: false, s: \"Mi nombre es encarna\", t: -32 }");

    let bytes = binary::to_bytes(&test).expect("Error serializing test");
    let bytes_out = format!("{:?}", bytes);

    assert_eq!(bytes_out, "[0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 20, 77, 105, 32, 110, 111, 109, 98, 114, 101, 32, 101, 115, 32, 101, 110, 99, 97, 114, 110, 97, 255, 255, 255, 255, 255, 255, 255, 224]");

    let reconstructed_test: Test = binary::from_bytes(bytes).expect("error Deserializing test");
    let reconstructed_out = format!("{:?}", reconstructed_test);

    assert_eq!(reconstructed_out, test_out);
}



#[derive(Debug, Clone, Copy)]
struct TestUnit;

impl Serialize for TestUnit {
    fn serialize<T: serialize::Serializer>(&self, serializer: T) -> Result<T::Ok, T::Err> {
        serializer.serialize_unit_struct("TestUnit")
    }
}

impl Deserialize for TestUnit {
    fn deserialize<T: deserialize::Deserializer>(deserializer: T) -> Result<Self, T::Err> {
        deserializer.deserialize_unit_struct("TestUnit")?;
        Ok(TestUnit)
    }
}

#[test]
fn manual_unit_struct_ser_deser() {
    let test = TestUnit;
    let unit_out = format!("{:?}", test);

    assert_eq!(unit_out, "TestUnit");

    let bytes = binary::to_bytes(&test).expect("Error serializing test");
    let bytes_out = format!("{:?}", bytes);

    assert_eq!(bytes_out, "[]");

    let reconstructed_test: TestUnit = binary::from_bytes(bytes).expect("error Deserializing test");
    let reconstructed_out = format!("{:?}", reconstructed_test);

    assert_eq!(reconstructed_out, unit_out);
}



#[derive(Debug, Clone)]
struct TestUnnamed(usize, String, i32);

impl Serialize for TestUnnamed {
    fn serialize<T: serialize::Serializer>(&self, serializer: T) -> Result<T::Ok, T::Err> {
        let mut ser = serializer.into_struct_unnamed_serializer("TestUnnamed")?;

        ser.serialize_unnamed_field(&self.0)?;
        ser.serialize_unnamed_field(&self.1)?;
        ser.serialize_unnamed_field(&self.2)?;

        ser.end()
    }
}

impl Deserialize for TestUnnamed {
    fn deserialize<T: deserialize::Deserializer>(deserializer: T) -> Result<Self, T::Err> {
        let mut deser = deserializer.into_struct_unnamed_deserializer("TestUnnamed")?;
        
        let a0 = deser.deserialize_unnamed_field()?;
        let a1 = deser.deserialize_unnamed_field()?;
        let a2 = deser.deserialize_unnamed_field()?;

        Ok(TestUnnamed(a0,a1,a2))
    }
}

#[test]
fn manual_unnammed_ser_deser() {
    let test = TestUnnamed(10, "tuputamadre".to_string(), -55);
    let unit_out = format!("{:?}", test);

    assert_eq!(unit_out, "TestUnnamed(10, \"tuputamadre\", -55)");

    let bytes = binary::to_bytes(&test).expect("Error serializing test");
    let bytes_out = format!("{:?}", bytes);

    assert_eq!(bytes_out, "[0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 11, 116, 117, 112, 117, 116, 97, 109, 97, 100, 114, 101, 255, 255, 255, 201]");

    let reconstructed_test: TestUnnamed = binary::from_bytes(bytes).expect("error Deserializing test");
    let reconstructed_out = format!("{:?}", reconstructed_test);

    assert_eq!(reconstructed_out, unit_out);
}



#[derive(Debug, Clone)]
struct TestNewtype(usize);

impl Serialize for TestNewtype {
    fn serialize<T: serialize::Serializer>(&self, serializer: T) -> Result<T::Ok, T::Err> {
        serializer.serialize_newtype_struct_contents("TestNewtype", &self.0)
    }
}

impl Deserialize for TestNewtype {
    fn deserialize<T: deserialize::Deserializer>(deserializer: T) -> Result<Self, T::Err> {
        let inner = deserializer.deserialize_newtype_struct_contents::<usize>("TestNewtype")?;

        Ok(TestNewtype(inner))
    }
}

#[test]
fn manual_newtype_ser_deser() {
    let test = TestNewtype(10);
    let unit_out = format!("{:?}", test);

    assert_eq!(unit_out, "TestNewtype(10)");

    let bytes = binary::to_bytes(&test).expect("Error serializing test");
    let bytes_out = format!("{:?}", bytes);

    assert_eq!(bytes_out, "[0, 0, 0, 0, 0, 0, 0, 10]");

    let reconstructed_test: TestNewtype = binary::from_bytes(bytes).expect("error Deserializing test");
    let reconstructed_out = format!("{:?}", reconstructed_test);

    assert_eq!(reconstructed_out, unit_out);
}



#[derive(Debug)]
enum TestEnum {
    A,
    B(usize, String),
    C{num: usize, s: String},
    D(String),
    F
}

impl Serialize for TestEnum {
    fn serialize<T: serialize::Serializer>(&self, serializer: T) -> Result<T::Ok, T::Err> {
        let ser = serializer.into_enum_serializer("TestEnum")?;
        
        match self {
            TestEnum::A => ser.serialize_unit_variant("A"),
            TestEnum::B(a, b) => {
                let mut ser = ser.serialize_tuple_variant("B")?;
                ser.serialize_element(a)?;
                ser.serialize_element(b)?;
                ser.end()
            },
            TestEnum::C { num, s } => {
                let mut ser = ser.serialize_struct_variant("C")?;
                ser.serialize_field("num", num)?;
                ser.serialize_field("s", s)?;
                ser.end()
            },
            TestEnum::D(s) => ser.serialize_newtype_variant("D", s),
            TestEnum::F => ser.serialize_unit_variant("F"),
        }
    }
}

impl Deserialize for TestEnum {
    fn deserialize<T: deserialize::Deserializer>(deserializer: T) -> Result<Self, T::Err> {
        let mut deser = deserializer.into_enum_deserializer("TestEnum")?;

        Ok(match deser.deserialize_variant_ident()?.as_str() {
            "A" => {
                deser.deserialize_unit_variant()?;
                Self::A
            },
            "B" => {
                let mut deser = deser.deserialize_tuple_variant()?;

                let a0 = deser.deserialize_element()?;
                let a1 = deser.deserialize_element()?;

                Self::B(a0, a1)
            },
            "C" => {
                let mut deser = deser.deserialize_struct_variant()?;

                let num = deser.deserialize_field("num")?;
                let s = deser.deserialize_field("s")?;

                Self::C{num, s}
            },
            "D" => {
                Self::D(deser.deserialize_newtype_variant_contents()?)
            },
            "F" => {
                deser.deserialize_unit_variant()?;
                Self::F
            },
            _ => panic!("Unexpected enum ident")
        })
    }
}

fn make_all_variant_array() -> [TestEnum; 5] {
    [
        TestEnum::A,
        TestEnum::B(5, "22".to_string()),
        TestEnum::C {num: 10, s: "me cago en tus muertos".to_string()},
        TestEnum::D("senpai".to_string()),
        TestEnum::F
    ]
}

fn make_all_variant_results_array() -> [(&'static str, &'static str); 5] {
    [
        ("A", "[0, 0, 0, 0, 0, 0, 0, 1, 65]"),
        ("B(5, \"22\")", "[0, 0, 0, 0, 0, 0, 0, 1, 66, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 2, 50, 50]"),
        ("C { num: 10, s: \"me cago en tus muertos\" }", "[0, 0, 0, 0, 0, 0, 0, 1, 67, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 22, 109, 101, 32, 99, 97, 103, 111, 32, 101, 110, 32, 116, 117, 115, 32, 109, 117, 101, 114, 116, 111, 115]"),
        ("D(\"senpai\")", "[0, 0, 0, 0, 0, 0, 0, 1, 68, 0, 0, 0, 0, 0, 0, 0, 6, 115, 101, 110, 112, 97, 105]"),
        ("F", "[0, 0, 0, 0, 0, 0, 0, 1, 70]"),
    ]
}

#[test]
fn manual_enum_ser_deser() {
    for (test, (a, b)) in make_all_variant_array().into_iter().zip(make_all_variant_results_array()) {
        let test_out = format!("{:?}", test);

        assert_eq!(test_out, a);

        let bytes = binary::to_bytes(&test).expect("Error serializing test");
        let bytes_out = format!("{:?}", bytes);

        assert_eq!(bytes_out, b);

        let reconstructed_test: TestEnum = binary::from_bytes(bytes).expect("error Deserializing test");
        let reconstructed_out = format!("{:?}", reconstructed_test);

        assert_eq!(reconstructed_out, test_out);
    }
}