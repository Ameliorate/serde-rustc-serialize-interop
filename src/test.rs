use super::*;

#[derive(Serialize, Deserialize, RustcDecodable, RustcEncodable, Clone, Copy, Debug, Eq, PartialEq)]
struct TestStruct(u32);

#[test]
fn serde_to_serde() {
    let test = TestStruct(238524);
    let interop = Interop::serde(&test).unwrap();
    let test_deser: TestStruct = interop.serde_deser().unwrap();
    assert_eq!(test, test_deser);
}

#[test]
#[should_panic(expected = "Called serde_deser on value constructed using rustc-serialize")]
fn serde_to_rustc() {
    let test = TestStruct(2305349807);
    let interop = Interop::rustc(&test).unwrap();
    let test_deser: TestStruct = interop.serde_deser().unwrap();
    assert_eq!(test, test_deser);
}

#[test]
fn rustc_to_rustc() {
    let test = TestStruct(2568913);
    let interop = Interop::rustc(&test).unwrap();
    let test_deser: TestStruct = interop.rustc_deser().unwrap();
    assert_eq!(test, test_deser);
}

#[test]
#[should_panic(expected = "Called rustc_deser on value constructed using serde")]
fn rustc_to_serde() {
    let test = TestStruct(23537566);
    let interop = Interop::serde(&test).unwrap();
    let test_deser: TestStruct = interop.rustc_deser().unwrap();
    assert_eq!(test, test_deser);
}
