use traits::*;

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum BFTokenType {
    IncrementPtr(usize),      // >
    DecrementPtr(usize),      // <
    IncrementData(usize),     // +
    DecrementData(usize),     // -
    Output,                   // .
    Input,                    // ,
    
    // These store their corresponding tokens
    //  aka LoopStart stores the position of
    //  the matching LoopEnd and vice versa.
    LoopStart(usize),         // [
    LoopEnd(usize),           // ]

    // Optimised instructions
    SetCurrent(i8),           // Sets the current cell to value
    AddCurrentUp(usize),      // Adds current cell to cell in [current + value]
    AddCurrentDown(usize)     // Adds current cell to cell in [current - value]
}

#[derive(Clone, Debug, Copy)]
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

        // Store the previous 5 tokens, these are used
        //  for optimisations.
        let mut last_tokens = [BFToken { pos : 0, token_type : BFTokenType::Input }; 5];

        // Loop through each character
        for character in input_string.chars() {
            // Match our character to a TokenType.
            let token_type = match character {
                '>' => {
                    // Batch up all the '>' tokens
                    let mut new_x = 1;
                    if last_tokens[0].pos > 0 {
                        match last_tokens[0].token_type {
                            BFTokenType::IncrementPtr(x) => {
                                tokens.pop();
                                pos-=1;
                                new_x += x;
                            },
                            _ => {}
                        }
                    }
                    BFTokenType::IncrementPtr(new_x)
                },
                '<' => {
                    let mut new_x = 1;
                    if last_tokens[0].pos > 0 {
                        match last_tokens[0].token_type {
                            BFTokenType::DecrementPtr(x) => {
                                tokens.pop();
                                pos-=1;                             
                                new_x += x;                           
                            },
                            _ => {}
                        }
                    }
                    BFTokenType::DecrementPtr(new_x)
                },
                '+' =>  {
                    let mut new_x = 1;
                    if last_tokens[0].pos > 0 {
                        match last_tokens[0].token_type {
                            BFTokenType::IncrementData(x) => {
                                tokens.pop();
                                pos-=1;
                                new_x += x;
                            },
                            _ => {}
                        }
                    }
                    BFTokenType::IncrementData(new_x)
                },
                '-' => {
                    let mut new_x = 1;
                    if last_tokens[0].pos > 0 {
                        match last_tokens[0].token_type {
                            BFTokenType::DecrementData(x) => {
                                tokens.pop();
                                pos-=1;
                                new_x += x;
                            },
                            _ => {}
                        }
                    }
                    BFTokenType::DecrementData(new_x)
                },
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
                    let mut ret_token : BFTokenType = BFTokenType::LoopEnd(index);      

                    // Update it's data position to this ] token
                    tokens[index].token_type = BFTokenType::LoopStart(pos);

                    match last_tokens[0].token_type {
                        // This currently checks for [-] or [+] and replaces 
                        //  those with a set current cell to 0 instruction.
                        BFTokenType::IncrementData(_) 
                        | BFTokenType::DecrementData(_) => {
                            match last_tokens[1].token_type {
                                BFTokenType::LoopStart(_) => {
                                    ret_token = BFTokenType::SetCurrent(0);
                                    tokens.pop();
                                    tokens.pop();
                                    pos-=2;
                                },
                                _ => { }
                            }
                        },

                        //  Optimisation for [-<+>] or [->+<] pattern.
                        BFTokenType::DecrementPtr(x)
                        | BFTokenType::IncrementPtr(x) => {
                            match last_tokens[1].token_type {
                                BFTokenType::IncrementData(a) => {
                                    match last_tokens[2].token_type {
                                        BFTokenType::IncrementPtr(y)
                                        | BFTokenType::DecrementPtr(y) => {
                                             if x == y {
                                                match last_tokens[3].token_type {
                                                    BFTokenType::DecrementData(b) => {
                                                        if a == b {
                                                            match last_tokens[4].token_type {
                                                                BFTokenType::LoopStart(_) => {
                                                                    if last_tokens[0].token_type == BFTokenType::DecrementPtr(x) {
                                                                        ret_token = BFTokenType::AddCurrentUp(x);
                                                                    } else {
                                                                        ret_token = BFTokenType::AddCurrentDown(x);
                                                                    }
                                                                    tokens.pop();
                                                                    tokens.pop();
                                                                    tokens.pop();
                                                                    tokens.pop();
                                                                    tokens.pop();
                                                                    pos-=5;
                                                                },
                                                                _ => {}
                                                            }
                                                        }
                                                    },
                                                    _ => { }
                                                }
                                            }
                                        },
                                        _ => { }
                                    }
                                }, 
                                _ => { }
                            };
                        },
                        
                        // Empty Loop
                        BFTokenType::LoopStart(_) => {
                            tokens.pop();
                            pos-=1;
                            continue;
                        }
                        _ => { }
                    }
                    
                    ret_token
                },

                // BF is a very simple language - if there is an 
                //  unrecognised character, it is ignored.
                _ => continue
            };

            let mut i = 1;
            // Recopy last tokens instead of shifting as they 
            //  may have chagned.
            while i < 5 && pos > i-1 {
                last_tokens[i] = tokens[pos-i].clone();
                i+=1;
            }

            pos+=1;

            // Add it to the list of tokens
            let token = BFToken { token_type : token_type, pos : pos};

            last_tokens[0] = token.clone();

            tokens.push(token);
        }

        // No way for this to currently fail.
        LexResult::Success(tokens)
    }
}
