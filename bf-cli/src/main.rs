// Load the main BF library
extern crate bf_lib;

// Import the tools we need from BF library
use bf_lib::traits::*;
use bf_lib::bf_lexer::BFLexer;
use bf_lib::bf_vm::BFVM;
use bf_lib::bf_vm::VMSettings;

// Import various STD library components
use std::process;
use std::fs::File;
use std::io::prelude::*;

// Command line arguments are parsed using a simpel state machine
#[derive(Debug)]
enum ArgumentMode {
    Skip,
    Start,
    Str,
    File,
}

// Reads a file and puts the contents into the out_str String.
fn read_file(in_str : String, mut out_str : &mut String ) {
    // TODO: Return an code here if needed and then print out
    //  a usable error message.
    // Currently panics on failure (expect)!
    let mut file = File::open(in_str).expect("Unable to open file.");
    file.read_to_string(&mut out_str).expect("Unable to read file.");
}

fn print_help() {
    println!("
Rust BrainFuck Interpreter

Usage:
    bf-cli <file>
    bf-cli <file> -u
    bf-cli ( -f | --file ) <file>
    bf-cli ( -s | --str ) <bfstring>

Options:
    -h --help       Shows this screen.
    -u --usermode   Input is prompted for.
");
}

fn main() {
    /* ---------------------------------------------------.
    |     Load Arguments                                  |
    '---------------------------------------------------- */

    // Argument Setup - Skip first argument as it's the binary location.
    let mut mode = ArgumentMode::Skip;
    let mut input = String::new();
    let mut settings = VMSettings::new();

    // Loop through each argument and set various settings as per the state
    // Aka if in Str mode the next argument will be considered the input.
    for argument in std::env::args() {
        match mode {
            ArgumentMode::Skip => mode = ArgumentMode::Start,
            ArgumentMode::Start => { 
                match argument.as_ref() {
                    "-s" | "--str" => mode = ArgumentMode::Str,
                    "-f" | "--file" => mode = ArgumentMode::File,
                    "-h" | "--help" => {
                        print_help();
                        process::exit(1);
                    },
                    "-u" | "--usermode" => settings.prompt_for_input = true,
                    _ => read_file(argument, &mut input)
                };
            },
            ArgumentMode::Str => {
                input = argument;
                mode = ArgumentMode::Start;
            },
            ArgumentMode::File => { 
                read_file(argument, &mut input);
                mode = ArgumentMode::Start;
            },
        }
    }


    /* ---------------------------------------------------.
    |    Interpret and Run Input                          |
    '---------------------------------------------------- */

    // Parse string input into Vector of BFTokens
    // This step also matches brackets up to each other
    let tokens = BFLexer::parse(String::from(input));

    // Create a new VM instance
    let mut bfvm = BFVM::new(settings);

    // If parsing was successful run the script, 
    //  otherwise Error
    // Currently parsing can't fail.
    let result = match tokens {
        LexResult::Success(t) => bfvm.run(t),
        _ =>  {
            println!("Error !");
            VMResult::Error { message : String::from("Uh oh") }
        }
    };

    // In case there was no new line printed
    //  print one now before exit so prompt is on
    //  new line if running from command line.
    println!();

    /* ---------------------------------------------------.
    |    Output Result                                    |
    '---------------------------------------------------- */

    // Setup exit code - 0 = Success
    // Currently all error's except panics are 1
    let return_code = if result == VMResult::Success { 0 } else { 1 };
    process::exit(return_code);
}
