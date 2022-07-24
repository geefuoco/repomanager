use std::fmt;
use std::error::Error;
use std::io::{BufReader, prelude::*};
use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct Account {
    user: String,
    password: String
}

impl Account {
    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.user, self.password)
    }
}

pub fn get_github_info() -> Result<Account, Box<dyn Error>> {
    let home = env::var("HOME").unwrap();
    let path = format!("{}/.repomanager/.github_info.txt", home);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut result = String::new();
    reader.read_to_string(&mut result)?;
    let mut result = result.split("\n");
    Ok(
        Account{
            user: result.next().unwrap().to_owned(),
            password: result.next().unwrap().to_owned()
        }
    )
}
