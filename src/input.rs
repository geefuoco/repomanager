use std::io::{stdin};
use std::error::Error;

pub fn read_input() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    Ok(buffer)
}
