extern crate bf_lib;

use bf_lib::traits::*;
use bf_lib::bf_lexer::BFLexer;
use bf_lib::bf_vm::BFVM;
use std::process;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum ArgumentMode {
    Skip,
    Start,
    Str,
    File,
}

fn read_file(in_str : String, mut out_str : &mut String ) {
    let mut file = File::open(in_str).expect("Unable to open file.");
    file.read_to_string(&mut out_str).expect("Unable to read file.");
}

fn print_help() {
    println!("
Rustfuck

Usage:
    bf-cli <file>
    bf-cli ( -f | --file ) <file>
    bf-cli ( -s | --str ) <bfstring>

Options:
    -h --help   Shows this screen.
");
}

fn main() {
    /* ---------------------------------------------------.
    |     Load Arguments                                  |
    '---------------------------------------------------- */
    let mut mode = ArgumentMode::Skip;
    let mut input = String::new();
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
                    }
                    _ => read_file(argument, &mut input)
                };
            },
            ArgumentMode::Str => input = argument,
            ArgumentMode::File => read_file(argument, &mut input)
        }
    }


    /* ---------------------------------------------------.
    |    Interpret and Run Input                          |
    '---------------------------------------------------- */
    let tokens = BFLexer::parse(String::from(input));
    let mut bfvm = BFVM::new();

    let result = match tokens {
        LexResult::Success(t) => bfvm.run(t),
        _ =>  {
            println!("Error !");
            VMResult::Error { message : String::from("Uh oh") }
        }
    };
    println!();

    /* ---------------------------------------------------.
    |    Output Result                                    |
    '---------------------------------------------------- */
    let return_code = if result == VMResult::Success { 0 } else { 1 };
    process::exit(return_code);
}
