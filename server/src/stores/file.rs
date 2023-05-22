use std::env::current_dir;
use std::fs::{ create_dir, File, write, read_to_string };
use std::path::{PathBuf, Path};
use chrono::{ Utc, DateTime };
use log::error;

use crate::error::Error;
use crate::config::Config;
use crate::stores::Store;


pub struct Filesystem {
    path: PathBuf
}

impl Filesystem {
    pub fn new(config: &Config) -> Result<Filesystem, Error> {
        // If no explicit path was set use current working directory.
        let mut path = match current_dir() {
            Ok(path) => path,
            Err(err) => return Err(Error::Init("could not retrieve current working directory")),
        };
        path.push("diary");

        // Override current working directory with custom config if its not the default value.
        if config.file_path != "./diary" {
            path = Path::new(&config.file_path).to_path_buf();
        }

        if path.exists() {
            return Ok(Filesystem{ path });
        }

        match create_dir(path.clone()) {
            Ok(_) => (),
            Err(_) => return Err(Error::Init("could not create directory")),
        };

        Ok(Filesystem { path })
    }


    /// Returns the full path to the file that stores the diary contents.
    fn file_path(&self) -> PathBuf {
        let date = Utc::now().format("%Y-%m-%d");

        let mut file_name = date.to_string();
        file_name.push_str(".txt");

        let mut file_path = self.path.clone();
        file_path.push(file_name);

        file_path
    }
}

impl Store for Filesystem {
    fn retrieve_latest(&mut self) -> String {
        let file_path = self.file_path();

        // If no file was created and written to yet return nothing.
        if !Path::new(&file_path).exists() {
            return "".to_string();
        }

        read_to_string(file_path).expect("Unable to read file")
    }

    fn store(&mut self, content: &str) -> Result<(), Error> {
        // File name should be the date of its creation.
        let file_path = self.file_path();

        // Create file if it does not exist yet.
        if !Path::new(&self.path).exists() {
            match File::create(file_path.clone()) {
                Ok(_) => (),
                Err(_) => return Err(Error::Storing("could not create file"))
            }
        }

        // Write contents.
        match write(file_path, content) {
            Ok(_) => (),
            Err(_) => return Err(Error::Storing("could not write to file"))
        }

        Ok(())
    }


}
