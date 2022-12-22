pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
  
pub struct Forth;
use std::{convert::TryInto, str::FromStr};
pub type ForthResult = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        unimplemented!()
    }

    pub fn stack(&self) -> &[Value] {
        unimplemented!()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        unimplemented!("result of evaluating '{}'", input)
    }
}
