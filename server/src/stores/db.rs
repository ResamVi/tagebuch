use std::env::VarsOs;
use std::fmt::{Debug, Formatter, Display};
use std::{thread, time, env};
use std::net::{TcpListener, TcpStream};

use simple_logger::SimpleLogger;
use log::{info, warn, error, SetLoggerError, debug};
use postgres::{Client, NoTls, error::DbError};
use tungstenite::{accept, WebSocket, Message};

use crate::error::Error;
use crate::config::Config;
use crate::stores::Store;

pub struct Database {
    client: Client
} 

impl Database {
    pub fn new(config: &Config) -> Result<Database, Error> {
        // Endlessly try to connect.
        let mut client = loop {
            let mut connect_attempt = Client::connect(&config.connection_string(), NoTls);
            match connect_attempt {
                Ok(client) => break client,
                Err(e) => warn!("{}", e),
            }

            thread::sleep(time::Duration::from_secs(3));
        };

        const QUERY_SCHEMA: &str = "
            CREATE TABLE IF NOT EXISTS entries (
                date        TIMESTAMP PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
                content     TEXT NOT NULL
            )
        ";


        // Init database if it's the first time we started up.
        match client.batch_execute(QUERY_SCHEMA) {
            Ok(_) => (),
            Err(err) => return Err(Error::Init()),
        };
        info!("schema initialized");

        Ok(Database { client })
    }
}


impl Store for Database {
    fn retrieve_latest(&mut self) -> String {
        const QUERY_SELECT: &str = "SELECT content FROM entries WHERE date = date_trunc('day', CURRENT_TIMESTAMP);";

        let content = match self.client.query_one(QUERY_SELECT, &[]) {
            Ok(row) => row.get("content"),
            Err(err) => "".to_string(), 
        };
        info!("sent current text {}", content);

        content
    }

    fn store(&mut self, content: &str) -> Result<(), Error> {
        const QUERY_INSERT: &str = "INSERT INTO entries (content, date) VALUES ($1, date_trunc('day', CURRENT_TIMESTAMP)) ON CONFLICT (date) DO UPDATE SET content = $1";

        // TODO: Check for error
        match self.client.execute(QUERY_INSERT, &[&content.to_string()]) {
            Ok(_) => (),
            Err(err) => return Err(Error::Storing()),
        }

        info!("stored user input");
        Ok(())
    }
}


