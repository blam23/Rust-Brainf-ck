use traits::*;

#[derive(PartialEq)]
pub enum BFTokenType {
    IncrementPtr,      // >
    DecrementPtr,      // <
    IncrementData,     // +
    DecrementData,     // -
    Output,            // .
    Input,             // ,
    LoopStart,         // [
    LoopEnd            // ]
}

pub struct BFToken {
    pub token_type : BFTokenType,

    // Currently pos is unused, but it just tracks where
    //  in the input the token was.
    // TODO: Debug printing that uses this.
    pub pos : usize
}

pub struct BFLexer {
}

impl Lexer<Vec<BFToken>> for BFLexer {

    fn parse(input_string:String) -> LexResult<Vec<BFToken>> {
        // Create empty vector
        let mut tokens = vec![];

        // Store where in the file / string the token was found.
        // TODO: Currently 1 dimensional, 
        //   should be updated to line : position?
        let mut pos : usize = 0;

        // Loop through each character
        for character in input_string.chars() {

            // Currently this keeps track of where the token is
            //  including ignored characters.
            pos+=1;

            // Match our character to a TokenType.
            let token_type = match character {
                '>' => BFTokenType::IncrementPtr,
                '<' => BFTokenType::DecrementPtr,
                '+' => BFTokenType::IncrementData,
                '-' => BFTokenType::DecrementData,
                '.' => BFTokenType::Output,
                ',' => BFTokenType::Input,
                '[' => BFTokenType::LoopStart,
                ']' => BFTokenType::LoopEnd,

                // BF is a very simple language - if there is an 
                // unrecognised character, it is ignored.
                _ => continue
            };

            // Add it to the list of tokens
            tokens.push(BFToken { token_type : token_type, pos : pos-1});
        }

        // No way for this to currently fail.
        LexResult::Success(tokens)
    }
}
