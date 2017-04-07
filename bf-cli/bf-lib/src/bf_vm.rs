use traits::*;
use bf_lexer::*;
extern crate std;
use std::io;
use std::io::Write;
use std::io::Read;

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

}

impl VMSettings { 
    pub fn new() -> VMSettings {
        VMSettings {
            prompt_for_input : false,
        }
    }
}

impl VM<BFToken> for BFVM {

    // This will loop through the tokens until
    //  the instruction pointer reaches the end
    //  of the token vector.
    fn run(&mut self, data : Vec<BFToken>) -> VMResult {

        //println!("Tokens: {:?}", data);

        let mut reader = io::stdin();
        let mut writer = io::stdout();

        while self.inst_ptr < data.len() {
            let result = self.step(&data, &mut reader, &mut writer);
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
    pub fn step(&mut self, data : &Vec<BFToken>, reader : &mut io::Stdin, writer : &mut io::Stdout) -> VMResult {
        // Import enum -> Allows for using enum values without
        //  BFTokenType:: prefix
        use bf_lexer::BFTokenType::*;
        
        // Get the current token
        let token = &data[self.inst_ptr];

        match token.token_type {

            // >    Increments data pointer
            IncrementPtr(x) => self.data_ptr+=x,

            // <    Decrements data pointer
            DecrementPtr(x) => self.data_ptr-=x,

            // +    Wrapping adds 1 to cell that data pointer is pointing to
            IncrementData(x) => self.mem[self.data_ptr] = self.mem[self.data_ptr].wrapping_add((x % 255) as i8),

            // -    Wrapping subtracts 1 from cell
            DecrementData(x) => self.mem[self.data_ptr] = self.mem[self.data_ptr].wrapping_sub((x % 255) as i8),

            // .    Prints the current cell as a character to stdout (65 - A)
            Output => {
                // Write current cell to stdout as a byte
                let data = &[self.mem[self.data_ptr] as u8];
                writer.write(data).expect("Unable to write to STDOUT");
            },

            // ,    Reads input from stdin and puts it into current cell
            Input => {
                if self.settings.prompt_for_input {
                    print!("\n> ");
                    io::stdout().flush().ok().expect("Could not flush stdout");
                }
                // Read one byte from stdin as a signed byte and store it
                let mut buffer = [0u8; 1];
                reader.read(&mut buffer[..]).expect("Unable to read from STDIN");
                self.mem[self.data_ptr] = buffer[0] as i8;
            },

            // [     If current data cell is 0 skip to matching ]
            LoopStart(x) => {
                if self.mem[self.data_ptr] == 0 {
                    self.inst_ptr = x;
                }
            },

            // ]     If current data cell isn't 0 skip to matching [
            LoopEnd(x) => {
                if self.mem[self.data_ptr] != 0 {
                    self.inst_ptr = x;
                }
            },

            // Optimisation - Sets current cell to 0
            SetCurrent(x) => {
                self.mem[self.data_ptr] = x;
            },

            // Optimisation - Adds current cell contents to cell offset by +x
            AddCurrentUp(x) => {
                self.mem[self.data_ptr + x] += self.mem[self.data_ptr];
                self.mem[self.data_ptr] = 0;
            },

            // Optimisation - Adds current cell contents to cell offset by -x
            AddCurrentDown(x) => {
                self.mem[self.data_ptr - x] += self.mem[self.data_ptr];
                self.mem[self.data_ptr] = 0;
            }
        }
        
        self.inst_ptr+=1;

        VMResult::Success
    }
}