
// Lexer
// ==================================================================

pub trait Lexer<T> {
    fn parse(input_string:String) -> LexResult<T>;
}

#[derive(Debug)]
pub enum LexResult<T> {
    Success(T),
    Error { message: String, pos: usize }
}

// VM
// ==================================================================

pub trait VM<T> {
    fn run(&mut self, data : Vec<T>) -> VMResult;
}

#[derive(Debug, PartialEq)]
pub enum VMResult {
    Success,
    Error { message: String }
}

