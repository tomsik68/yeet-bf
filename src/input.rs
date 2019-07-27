use std::boxed::Box;
use std::io::{self, Read, Result};

pub trait InputSupplier {
    fn read_char(&mut self) -> Result<char>;
}

struct StdinInputSupplier {}

pub fn stdin() -> Box<InputSupplier> {
    Box::new(StdinInputSupplier::new())
}

impl StdinInputSupplier {
    pub fn new() -> StdinInputSupplier {
        StdinInputSupplier {}
    }
}

impl InputSupplier for StdinInputSupplier {
    fn read_char(&mut self) -> Result<char> {
        let mut result = [0u8];
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_exact(&mut result)?;
        Ok(result[0] as char)
    }
}
