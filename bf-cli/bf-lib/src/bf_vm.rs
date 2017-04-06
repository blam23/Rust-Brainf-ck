use traits::*;
use bf_lexer::*;
extern crate std;
use std::io;
use std::io::Write;

// Struct for our Virtual Machine that interprets
//  the Brainfuck tokens
pub struct BFVM {

    // Memory storage for the BF programs to use
    //  Note: 60000 bytes is double standard BF size
    mem : [i8; 60000],

    // Current location BF program is looking at
    //  in memory.
    data_ptr : usize,

    // Current location the VM is running instructions
    //  from (using it's token list, not 'mem' memory)
    inst_ptr : usize,

    // Settings that can be changed via input args
    settings : VMSettings
}

// Extra settings for the VM
//  Should these just in the BFVM struct?
pub struct VMSettings {

    // Controls if we show the prompt or not.
    // Input from a file should naturally work by
    //  controlling stdin via a pipe could use
    //  isatty to determine this automatically
    //  later on. 
    pub prompt_for_input : bool,

    // Controls if input is read as series of
    //  character numbers or as a char that will 
    //  be converted to a string.
    // Examples     true:  '101' => 101
    //              false: 'A'   => 65
    //              true:  'A'   => Invalid
    //              false: '101' => 1nvalid
    pub input_as_char : bool,
}

impl VMSettings { 
    pub fn new() -> VMSettings {
        VMSettings {
            prompt_for_input : false,
            input_as_char : false
        }
    }
}

impl VM<BFToken> for BFVM {

    // This will loop through the tokens until
    //  the instruction pointer reaches the end
    //  of the token vector.
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
    // Creates a new BFVM.
    pub fn new(settings : VMSettings) -> BFVM {
        BFVM {

            // Initialise memory to 0
            mem : [0; 60000],

            // Start data pointer in middle of memory, so it can
            //  go backwards.
            data_ptr : 30000,

            // Instruction pointer also needs to be 0
            inst_ptr : 0,

            settings : settings
        }
    }

    // Interprets the current token.
    pub fn step(&mut self, data : &Vec<BFToken>) -> VMResult {
        // Get the current token
        let token = &data[self.inst_ptr];

        match token.token_type {

            // >    Increments data pointer
            BFTokenType::IncrementPtr => self.data_ptr+=1,

            // <    Decrements data pointer
            BFTokenType::DecrementPtr => self.data_ptr-=1,

            // +    Wrapping adds 1 to cell that data pointer is pointing to
            BFTokenType::IncrementData => self.mem[self.data_ptr] = self.mem[self.data_ptr].wrapping_add(1),

            // -    Wrapping subtracts 1 from cell
            BFTokenType::DecrementData => self.mem[self.data_ptr] = self.mem[self.data_ptr].wrapping_sub(1),

            // .    Prints the current cell as a character to stdout (65 - A)
            BFTokenType::Output => print!("{}", self.mem[self.data_ptr] as u8 as char),

            // ,    Reads input from stdin and puts it into current cell
            BFTokenType::Input => {
                if self.settings.prompt_for_input {
                    print!("\n> ");
                    io::stdout().flush().ok().expect("Could not flush stdout");
                }
                let mut input_buffer = String::new();
                io::stdin().read_line(&mut input_buffer).expect("Failed to read data from stdin");
                let trimmed = input_buffer.trim();

                if self.settings.input_as_char {
                    let data = trimmed.chars().nth(0).unwrap() as u8 as i8;
                    self.mem[self.data_ptr] = data;
                } else {
                    match trimmed.parse::<i8>() {
                        Ok(x) => self.mem[self.data_ptr] = x,
                        Err(..) => return VMResult::Error { message : String::from("Invalid console input!") }
                    }
                }
            },

            // [     If current data cell is 0 skip to matching ]
            BFTokenType::LoopStart => {
                if self.mem[self.data_ptr] == 0 {

                    // Depth is used to match an open bracket to the right close bracket.
                    //  If we have:
                    //    [.,[++[+]++.,]-]
                    //       ^
                    //  we need to skip over the inner [+] and find the next close bracket
                    //  that does not have an open bracket preceding it, ex:
                    //
                    //    [.,[++[+]++.,]-]
                    //       ^---------^
                    //
                    //  Depth will be:
                    //    [.,[++[+]++.,]-]
                    //       11122111110

                    let mut depth = 1;

                    // TODO: Make this efficient
                    //   Use a stack?

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

            // ]     If current data cell isn't 0 skip to matching [
            BFTokenType::LoopEnd => {
                if self.mem[self.data_ptr] != 0 {

                    // This is done the same way as LoopStart but going backwards

                    let mut depth = 1;

                    // TODO: Make this efficient
                    //   Use a stack?

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
                    // return immediately to skip adding 1 to instruction ptr
                    return VMResult::Success;
                }
            }
        }
        
        self.inst_ptr+=1;

        VMResult::Success
    }
}