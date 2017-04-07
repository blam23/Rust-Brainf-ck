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
    SetBlock(usize,i8),        // Sets blocks of sells to value
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
        // Import enum -> Allows for using enum values without
        //  BFTokenType:: prefix
        use self::BFTokenType::*;

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
        let mut last_tokens = [BFToken { pos : 0, token_type : Input }; 5];

        // Loop through each character
        for character in input_string.chars() {
            // Match our character to a TokenType.
            let token_type = match character {
                '>' => {
                    // Batch up all the '>' tokens
                    let mut new_x = 1;
                    if last_tokens[0].pos > 0 {
                        if let IncrementPtr(x) = last_tokens[0].token_type {
                            tokens.pop();
                            pos-=1;
                            new_x += x;
                        }
                    }
                    IncrementPtr(new_x)
                },
                '<' => {
                    let mut new_x = 1;
                    if last_tokens[0].pos > 0 {
                        if let DecrementPtr(x) = last_tokens[0].token_type {
                            tokens.pop();
                            pos-=1;                             
                            new_x += x;                           
                        }
                    }
                    DecrementPtr(new_x)
                },
                '+' =>  {
                    let mut new_x = 1;
                    if last_tokens[0].pos > 0 {
                        if let IncrementData(x) = last_tokens[0].token_type {
                            tokens.pop();
                            pos-=1;
                            new_x += x;
                        }
                    }
                    IncrementData(new_x)
                },
                '-' => {
                    let mut new_x = 1;
                    if last_tokens[0].pos > 0 {
                        if let DecrementData(x) = last_tokens[0].token_type {
                            tokens.pop();
                            pos-=1;
                            new_x += x;
                        }
                    }
                    DecrementData(new_x)
                },
                '.' => Output,
                ',' => Input,
                '[' => {
                    // Push this pos onto stack
                    loop_stack.push(pos);

                    // Store temp value of 0 for now - will be updated
                    //  once we know matching ]
                    LoopStart(0)
                },
                ']' => {

                    // Get matching bracket.
                    let index = loop_stack.pop().expect("Bracket mismatch");
                    let mut ret_token : BFTokenType = LoopEnd(index);      

                    // Update it's data position to this ] token
                    tokens[index].token_type = LoopStart(pos);

                    // Lots of optimisations can be done when you have a loop.
                    //  At this point the lexer knows all the components of the
                    //  loop and can optimise out common loop patterns into single
                    //  instruction calls.
                    match last_tokens[0].token_type {
                        // This currently checks for [-] or [+] and replaces 
                        //  those with a set current cell to 0 instruction.
                        // It also looks for a [-]>[-] type pattern where
                        //  multiple cells are set to 0
                        IncrementData(_) 
                        | DecrementData(_) => {
                            if let LoopStart(_) = last_tokens[1].token_type {
                                let mut size = 1;
                                // Check for blocks
                                if let IncrementPtr(1) = last_tokens[2].token_type {
                                    if let SetBlock(x, 0) = last_tokens[3].token_type {
                                        size = x + 1;
                                        tokens.pop();
                                        tokens.pop();
                                        pos-=2;
                                    };
                                }

                                ret_token = SetBlock(size, 0);
                                tokens.pop();
                                tokens.pop();
                                pos-=2;
                            }
                        },

                        // TODO: Multiplication detection & optimisation?
                        // Optimisation for [-<+>] or [->+<] pattern.
                        // These loops will add the current cell value 
                        //  to the cell value offset by the amount of
                        //  ptr increments '>' and '<'.
                        // As it adds to the offset cell, it takes from
                        //  the current cell. This will loop until the 
                        //  current cell is 0.
                        // So it can be equated to addition and setting
                        //  the current value to 0
                        //
                        // Examples:
                        //
                        //  [->+<]         adds mem[current] to mem[current+1]
                        //                 sets mem[current] to 0
                        //
                        //  [->>>>+<<<<]   adds mem[current] to mem[current+4]
                        //                 sets mem[current] to 0
                        //
                        // Note that due to the combination optimisation
                        //  this only needs to check for [->+<] or [-<+>]
                        DecrementPtr(x)
                        | IncrementPtr(x) => {
                            if let IncrementData(a) = last_tokens[1].token_type {
                                match last_tokens[2].token_type {
                                    IncrementPtr(y)
                                    | DecrementPtr(y) => {
                                        if x == y {
                                            if let DecrementData(b) = last_tokens[3].token_type  {
                                                if a == b {
                                                    if let LoopStart(_) = last_tokens[4].token_type {
                                                        if last_tokens[0].token_type == DecrementPtr(x) {
                                                            ret_token = AddCurrentUp(x);
                                                        } else {
                                                            ret_token = AddCurrentDown(x);
                                                        }
                                                        tokens.pop();
                                                        tokens.pop();
                                                        tokens.pop();
                                                        tokens.pop();
                                                        tokens.pop();
                                                        pos-=5;
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    _ => { }
                                }
                            };
                        },
                        
                        // Empty Loop
                        LoopStart(_) => {
                            tokens.pop();
                            pos-=1;
                            continue;
                        },
                        
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
            //  may have changed due to optimisations.
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
