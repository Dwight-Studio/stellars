use std::io;
use std::io::Write;
use std::sync::{Arc, RwLock};
use libstellars::Stellar;

pub struct DebuggerState {
    stellars: Arc<RwLock<Stellar>>,

    paused: bool,
    stepping: bool,
    redraw_requested: bool,
    breakpoints: Vec<u16>,
}

impl DebuggerState {
    pub fn new(stellars: Arc<RwLock<Stellar>>) -> Self {
        Self {
            stellars,

            paused: true,
            stepping: false,
            redraw_requested: false,
            breakpoints: Vec::new(),
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn redraw_requested(&self) -> bool { self.redraw_requested }

    pub fn update(&mut self) {
        let debug_data = self.stellars.read().unwrap().get_debug_info();

        if self.breakpoints.contains(&debug_data.cpu.registers.pc) {
            self.paused = true;
        }

        if self.stepping {
            self.paused = true;
            self.stepping = false;
            println!("Executed opcode {:02X} at {:04X}", debug_data.cpu.executed_opcode.0, debug_data.cpu.executed_opcode.1);
        }

        if self.redraw_requested {
            self.redraw_requested = false;
        }
    }

    pub fn process_debugger_input(&mut self) {
        let mut input = String::new();

        loop {
            while input.trim().is_empty() {
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
            }

            let parsed_input = input.trim().split(" ").collect::<Vec<&str>>();
            match parsed_input[0] {
                "help" => {
                    println!(concat!(
                    "help:              Show this help screen.\n",
                    "break <address>:   Add a breakpoint at the address provided. E.g: break 0xF000\n",
                    "del <id>:          Remove a breakpoint by it's index\n",
                    "list:              List all the breakpoints\n",
                    "run:               Run the program until a breakpoint is encountered.\n",
                    "print:             Print debug informations.\n",
                    "step:              Step through one instruction.\n",
                    "read <address>:    Read the value at <address>.\n",
                    "display:           Display the picture buffer even if it is still filling up"
                    ))
                }
                "break" => {
                    if parsed_input.len() >= 2 {
                        if parsed_input[1].starts_with("0x") {
                            let hex_string = parsed_input[1].replace("0x", "");
                            let address = u16::from_str_radix(&hex_string, 16).unwrap();

                            self.breakpoints.push(address);
                            println!("Breakpoint {} set at 0x{:04X}.", self.breakpoints.len() - 1, address);
                        }
                    } else {
                        println!("Missing argument <address>.");
                    }
                }
                "del" => {
                    if parsed_input.len() >= 2 {
                        let id = parsed_input[1].parse::<usize>().unwrap();

                        self.breakpoints.remove(id);
                        println!("Removed breakpoint {id}.");
                    } else {
                        println!("Missing argument <index>.");
                    }
                }
                "list" => {
                    for i in 0..self.breakpoints.len() {
                        println!("Breakpoint {i}: 0x{:04X}", self.breakpoints[i]);
                    }
                }
                "run" => {
                    self.paused = false;
                    break;
                }
                "print" => {
                    self.stellars.read().unwrap().get_debug_info().print_debug();
                }
                "step" => {
                    self.stepping = true;
                    self.paused = false;
                    break;
                }
                "read" => {
                    if parsed_input.len() >= 2 {
                        if parsed_input[1].starts_with("0x") {
                            let hex_string = parsed_input[1].replace("0x", "");
                            let address = u16::from_str_radix(&hex_string, 16).unwrap();
                            let value = self.stellars.read().unwrap().read(address);

                            println!("0x{:04X}: 0x{:02X}", address, value);
                        }
                    } else {
                        println!("Missing argument <address>.");
                    }
                }
                "display" => {
                    self.redraw_requested = true;
                    break;
                }
                &_ => { println!("Unknown command. For help, type \"help\"."); }
            }
            input.clear();
        }
    }
}