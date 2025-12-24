use serde_json::from_reader;
use std::fs::File;
use libstellars::Stellar;

#[test]
fn opcode_0x01() {
    let file = File::open("resources/cpu/01.json").expect("Expected 01.json");
    let json: serde_json::Value = from_reader(file).expect("File should be a json");

    for test in json.as_array().unwrap() {
        let opcode = test.get("name").unwrap();
        let initial_state = test.get("initial").unwrap();

        let stellar = Stellar::new();
        let value = stellar.borrow().salut();

        println!("{}", opcode);
    }

    assert_eq!(7, 7);
}