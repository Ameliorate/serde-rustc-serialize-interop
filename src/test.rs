use super::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
struct TestStruct;

#[test]
fn serde_to_serde() {
    let test = TestStruct;
    let interop = Interop::serde(&test).unwrap();
    let test_deser: TestStruct = interop.serde_deser().unwrap();
    assert_eq!(test, test_deser);
}
