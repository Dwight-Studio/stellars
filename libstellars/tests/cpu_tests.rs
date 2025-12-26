use serde_json::from_reader;
use std::fs::File;
use libstellars::Stellar;

#[test]
fn opcode_0x01() {
    let file = File::open("resources/test.json").expect("Expected test.json");
    let json: serde_json::Value = from_reader(file).expect("File should be a json");

    for test in json.as_array().unwrap() {
        let initial_state = test.get("initial").unwrap().as_object().unwrap();
        let final_state = test.get("final").unwrap().as_object().unwrap();

        let stellar = Stellar::new();
        stellar.borrow().set_initial_state(initial_state);

        stellar.borrow().run_opcode();

        let equal = stellar.borrow().check_final_state(final_state);

        assert!(equal);
    }
}