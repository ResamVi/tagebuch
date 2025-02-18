pub mod db;
pub mod file;

use crate::error::Error;

pub trait Store {
    fn retrieve_latest(&mut self) -> String;
    fn store(&mut self, content: &str) -> Result<(), Error>;
}

