use traits::*;
use bf_lexer::*;
extern crate std;
use std::io;
use std::io::Write;

pub struct BFVM {
    // 60000 bytes is double standard BF size
    mem : [i8; 60000],
    data_ptr : usize,
    inst_ptr : usize,
}

impl VM<BFToken> for BFVM {
    fn run(&mut self, data : Vec<BFToken>) -> VMResult {
        while self.inst_ptr < data.len() {
            let result = self.step(&data);
            match result {
                VMResult::Success => continue,
                _ => return result,
            }
        }

        VMResult::Success
    }
}

impl BFVM {
    pub fn new() -> BFVM {
        BFVM {

            // Initialise memory to 0
            mem : [0; 60000],

            // Start data pointer in middle of memory, so it can
            //  go backwards.
            data_ptr : 30000,

            inst_ptr : 0,
        }
    }

    pub fn step(&mut self, data : &Vec<BFToken>) -> VMResult {
        let token = &data[self.inst_ptr];

        match token.token_type {
            BFTokenType::IncrementPtr => self.data_ptr+=1,
            BFTokenType::DecrementPtr => self.data_ptr-=1,
            BFTokenType::IncrementData => self.mem[self.data_ptr] = self.mem[self.data_ptr].wrapping_add(1),
            BFTokenType::DecrementData => self.mem[self.data_ptr] = self.mem[self.data_ptr].wrapping_sub(1),
            BFTokenType::Output => print!("{}", self.mem[self.data_ptr] as u8 as char),
            BFTokenType::Input => {
                print!("\n> ");
                io::stdout().flush().ok().expect("Could not flush stdout");
                let mut input_buffer = String::new();
                io::stdin().read_line(&mut input_buffer).expect("Failed to read data from stdin");
                let trimmed = input_buffer.trim();

                match trimmed.parse::<i8>() {
                    Ok(x) => self.mem[self.data_ptr] = x,
                    Err(..) => return VMResult::Error { message : String::from("Invalid console input!") }
                }
            },
            BFTokenType::LoopStart => {
                if self.mem[self.data_ptr] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        self.inst_ptr+=1;
                        if data[self.inst_ptr].token_type == BFTokenType::LoopStart {
                            depth+=1;
                        } else if data[self.inst_ptr].token_type == BFTokenType::LoopEnd {
                            depth-=1;
                        }
                        if self.inst_ptr >= data.len() {
                            return VMResult::Error { message : String::from("Bracket mismatch - Could not find matching ]") }
                        }
                    }
                }
            },
            BFTokenType::LoopEnd => {
                if self.mem[self.data_ptr] != 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        self.inst_ptr-=1;
                        if data[self.inst_ptr].token_type == BFTokenType::LoopEnd {
                            depth+=1;
                        } else if data[self.inst_ptr].token_type == BFTokenType::LoopStart {
                            depth-=1;
                        } else if self.inst_ptr == 0 {
                            return VMResult::Error { message : String::from("Bracket mismatch - Could not find matching [") }
                        }
                    }
                    return VMResult::Success;
                }
            }
        }
        
        self.inst_ptr+=1;

        VMResult::Success
    }
}