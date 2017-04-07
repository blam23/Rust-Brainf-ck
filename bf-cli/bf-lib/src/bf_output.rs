use bf_lexer::*;
use std::fs::File;
use std::io::Write;

pub fn dump_tokens(tokens : Vec<BFToken>, file_name : String) {
    use bf_lexer::BFTokenType::*;

    let mut token_string : String = String::new();

    let mut indent : String = String::new();
    for token in tokens {
        match token.token_type {
            IncrementPtr(x) => token_string = format!("{}\n{}{}", token_string, indent, (format!("\tmov {}", x))),
            DecrementPtr(x) => token_string = format!("{}\n{}{}", token_string, indent, (format!("\tmov -{}", x))),
            IncrementData(x) => token_string = format!("{}\n{}{}", token_string, indent, (format!("\tadd {}", x))),
            DecrementData(x) => token_string = format!("{}\n{}{}", token_string, indent, (format!("\tsub {}", x))),
            Output => token_string.push_str("\n\tprnt"),
            Input => token_string.push_str("\n\tgetc"),
            LoopStart(x) => {
                token_string = format!("{}\n{}{}", token_string, indent, (format!("\tjmpeq 0 .{}\n{}\t.{}", x, indent, token.pos-1)));
                indent.push('\t');
            },
            LoopEnd(x) => {
                indent.pop();
                token_string = format!("{}\n{}{}", token_string, indent, (format!("\tjmpneq 0 .{}\n{}.{}", x, indent, token.pos-1)));
            },
            SetBlock(x, y) => token_string = format!("{}\n{}{}", token_string, indent, (format!("\tset {} {}", x, y))),
            AddCurrentUp(x) => token_string = format!("{}\n{}{}", token_string, indent, (format!("\taddc {}", x))),
            AddCurrentDown(x) => token_string = format!("{}\n{}{}", token_string, indent, (format!("\taddc -{}", x)))
        }
    }

    println!("Dumping to '{}':\n{}", file_name, token_string);

    let mut file = File::create(file_name).expect("Unable to create dump file.");
    file.write_all(token_string.as_bytes()).expect("Unable to write to dump file.");
}