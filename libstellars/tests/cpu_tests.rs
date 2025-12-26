use serde_json::from_reader;
use std::fs::File;
use libstellars::Stellar;

macro_rules! generate_opcode_tests {
    ($($opcode:expr),*) => {
        $(
            paste::paste! {
                #[test]
                #[cfg(feature = "test-utils")]
                fn [<opcode_ $opcode>]() {
                    opcode_test($opcode);
                }
            }
        )*
    };
}

#[cfg(feature = "test-utils")]
fn opcode_test(opcode: u8) {
    let filename = format!("resources/cpu/{:02x}.json", opcode);
    let file = File::open(&filename).unwrap_or_else(|_| panic!("Expected {}", filename));
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

/* generate_opcode_tests!(0x00, 0x01, 0x05, 0x06, 0x08, 0x09, 0x0a, 0x0d, 0x0e,
0x10, 0x11, 0x15, 0x16, 0x18, 0x19, 0x1d, 0x1e,
0x20); */
//generate_opcode_tests!();