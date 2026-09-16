
#![allow(unused)]

use serax::{deserialize::{StructDeserializer, StructUnnamedDeserializer}, *};


#[derive(Debug, Serialize, Deserialize)]
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


#[test]
fn auto_struct_ser_deser() {
    // serax::test_ser!(struct Test {
    //     num: u32,
    //     b: bool,
    //     s: String,
    //     t: isize,
    // });
    // serax::test_deser!(struct Test {
    //     num: u32,
    //     b: bool,
    //     s: String,
    //     t: isize,
    // });
    // println!("\n\n\n\n");

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



#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct TestUnit;


#[test]
fn auto_unit_struct_ser_deser() {
    // serax::test_ser!(struct TestUnit;);
    // serax::test_deser!(struct TestUnit;);
    // println!("\n\n\n\n");

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



#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestUnnamed(usize, String, i32);


#[test]
fn auto_unnammed_ser_deser() {
    // serax::test_ser!(struct TestUnnamed(usize, String, i32););
    // serax::test_deser!(struct TestUnnamed(usize, String, i32););
    // println!("\n\n\n\n");

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



#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestNewtype(usize);


#[test]
fn auto_newtype_ser_deser() {
    // serax::test_ser!(struct TestNewtype(usize););
    // serax::test_deser!(struct TestNewtype(usize););
    // println!("\n\n\n\n");

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



#[derive(Debug, Serialize, Deserialize)]
enum TestEnum {
    A,
    B(usize, String),
    C{num: usize, s: String},
    D(String),
    F
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
fn auto_enum_ser_deser() {
    // serax::test_deser!(
    //     enum TestEnum {
    //         A,
    //         B(usize, String),
    //         C{num: usize, s: String},
    //         D(String),
    //         F
    //     }
    // );
    // println!("\n\n\n\n");

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