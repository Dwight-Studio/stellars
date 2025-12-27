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
0x20, 0x21, 0x24, 0x25, 0x26, 0x28, 0x29, 0x2a, 0x2c, 0x2d, 0x2e,
0x30, 0x31, 0x35, 0x36, 0x38, 0x39, 0x3d, 0x3e,
0x40, 0x41, 0x45, 0x46, 0x48, 0x49, 0x4a, 0x4c, 0x4d, 0x4e,
0x50, 0x51, 0x55, 0x56, 0x58, 0x59, 0x5d, 0x5e,
0x60, 0x61, 0x65, 0x66, 0x68, 0x69, 0x6a, 0x6c, 0x6d, 0x6e); */
//generate_opcode_tests!();