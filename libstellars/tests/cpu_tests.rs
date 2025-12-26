use serde_json::from_reader;
use std::fs::File;
use libstellars::Stellar;

#[test]
#[cfg(feature = "test-utils")]
fn opcode_0x00() {
    let file = File::open("resources/cpu/00.json").expect("Expected 00.json");
    let json: serde_json::Value = from_reader(file).expect("File should be a json");

    for test in json.as_array().unwrap() {
        let initial_state = test.get("initial").unwrap().as_object().unwrap();
        let final_state = test.get("final").unwrap().as_object().unwrap();

        let stellar = Stellar::new();
        stellar.read().unwrap().set_initial_state(initial_state);

        stellar.read().unwrap().run_opcode();

        let (equal, differences) = stellar.read().unwrap().check_final_state(final_state);

        if !equal {
            eprintln!("Test \"{}\" with differences:", test.get("name").unwrap().as_str().unwrap());
            for diff in differences {
                eprintln!("  - {}", diff);
            }
        }

        assert!(equal);
    }
}

#[test]
#[cfg(feature = "test-utils")]
fn opcode_0x01() {
    let file = File::open("resources/cpu/01.json").expect("Expected 01.json");
    let json: serde_json::Value = from_reader(file).expect("File should be a json");

    for test in json.as_array().unwrap() {
        let initial_state = test.get("initial").unwrap().as_object().unwrap();
        let final_state = test.get("final").unwrap().as_object().unwrap();

        let stellar = Stellar::new();
        stellar.read().unwrap().set_initial_state(initial_state);

        stellar.read().unwrap().run_opcode();

        let (equal, differences) = stellar.read().unwrap().check_final_state(final_state);

        if !equal {
            eprintln!("Test \"{}\" with differences:", test.get("name").unwrap().as_str().unwrap());
            for diff in differences {
                eprintln!("  - {}", diff);
            }
        }

        assert!(equal);
    }
}