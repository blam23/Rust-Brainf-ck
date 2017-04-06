use traits::*;

#[derive(PartialEq)]
pub enum BFTokenType {
    IncrementPtr,             // >
    DecrementPtr,             // <
    IncrementData,            // +
    DecrementData,            // -
    Output,                   // .
    Input,                    // ,
    
    // These store their corresponding tokens
    //  aka LoopStart stores the position of
    //  the matching LoopEnd and vice versa.
    LoopStart(usize),         // [
    LoopEnd(usize)            // ]
}

pub struct BFToken {
    pub token_type : BFTokenType,

    // Used for matching brackets
    pub pos : usize
}

pub struct BFLexer {
}

impl Lexer<Vec<BFToken>> for BFLexer {

    fn parse(input_string:String) -> LexResult<Vec<BFToken>> {
        // Create empty vector
        let mut tokens : Vec<BFToken> = vec![];

        // Store the index of the token in the vector
        let mut pos : usize = 0;

        // This stores the positions of open brackets
        // This is used to match up bracket pairs 
        //  so that they can jump to each other in O(1)
        let mut loop_stack : Vec<usize> = vec![];

        // Loop through each character
        for character in input_string.chars() {
            // Match our character to a TokenType.
            let token_type = match character {
                '>' => BFTokenType::IncrementPtr,
                '<' => BFTokenType::DecrementPtr,
                '+' => BFTokenType::IncrementData,
                '-' => BFTokenType::DecrementData,
                '.' => BFTokenType::Output,
                ',' => BFTokenType::Input,
                '[' => {
                    // Push this pos onto stack
                    loop_stack.push(pos);

                    // Store temp value of 0 for now - will be updated
                    //  once we know matching ]
                    BFTokenType::LoopStart(0)
                },
                ']' => {
                    // Get matching bracket.
                    let index = loop_stack.pop().expect("Bracket mismatch");
                    // Update it's data position to this ] token
                    tokens[index].token_type = BFTokenType::LoopStart(pos);
                    // Store the [ token's position
                    BFTokenType::LoopEnd(index)
                }

                // BF is a very simple language - if there is an 
                // unrecognised character, it is ignored.
                _ => continue
            };

            pos+=1;

            // Add it to the list of tokens
            tokens.push(BFToken { token_type : token_type, pos : pos});
        }

        // No way for this to currently fail.
        LexResult::Success(tokens)
    }
}
