use traits::*;

#[derive(PartialEq)]
pub enum BFTokenType {
    IncrementPtr,
    DecrementPtr,
    IncrementData,
    DecrementData,
    Output,
    Input,
    LoopStart,
    LoopEnd
}

pub struct BFToken {
    pub token_type : BFTokenType,
    pub pos : usize
}

pub struct BFLexer {
}

impl Lexer<Vec<BFToken>> for BFLexer {
    fn parse(input_string:String) -> LexResult<Vec<BFToken>> {
        let mut tokens = vec![];

        let mut pos : usize = 0;
        for character in input_string.chars() {
            pos+=1;
            let token_type = match character {
                '>' => BFTokenType::IncrementPtr,
                '<' => BFTokenType::DecrementPtr,
                '+' => BFTokenType::IncrementData,
                '-' => BFTokenType::DecrementData,
                '.' => BFTokenType::Output,
                ',' => BFTokenType::Input,
                '[' => BFTokenType::LoopStart,
                ']' => BFTokenType::LoopEnd,
                _ => continue
            };
            tokens.push(BFToken { token_type : token_type, pos : pos-1});
        }

        LexResult::Success(tokens)
    }
}
