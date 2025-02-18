use std::fmt::{Debug, Formatter, Display};
use std::thread::spawn;
use std::{thread, time, env};
use std::net::{TcpListener, TcpStream};

use simple_logger::SimpleLogger;
use log::{info, warn, error, SetLoggerError, debug};
use postgres::{Client, NoTls, error::DbError};
use tungstenite::{accept, WebSocket, Message};

use crate::stores::Store;
use crate::stores::db::Database;
use crate::stores::file::Filesystem;
use crate::error::Error;

mod error;
mod stores;
mod config;

fn main() -> Result<(), Error> {
    // Initialize logger.
    match SimpleLogger::new().env().init() {
        Ok(_) => (),
        Err(err) => return Err(Error::LoggerInit(err)),
    }
    info!("logger initalized");

    // Load configurations from environment variables.
    let config = config::Config::from_env();
    info!("configs loaded: {}", config.file_path());

    // Initialize sink where diary entries are stored.
    let mut storage = match Filesystem::new(&config) {
        Ok(store) => store,
        Err(err) => return Err(err),
    };
    info!("diary storage initialized");

    // Listen for websocket connections.
    let server = match TcpListener::bind(config.bind_address()) {
        Ok(server) => server,
        Err(err) => return Err(Error::TcpConnection(err)),
    };
    info!("listening to connections on port {}", config.port);

    for incoming in server.incoming() {
        handle_client(&mut storage, &config.secret, incoming);
    }
    info!("stopped listening to TCP connections. bye");

    Ok(())
}

fn handle_client(storage: &mut impl Store, secret: &str, incoming: Result<TcpStream, std::io::Error>) -> Result<(), Error> {
    // Accept TCP connection.
    let stream = match incoming {
        Ok(stream) => stream,
        Err(err) => return Err(Error::TcpConnection(err)),
    };
    info!("accepted TCP connection");

    // Upgrade to Websocket connection.
    let mut websocket_conn = match accept(stream) {
        Ok(websocket_conn) => websocket_conn,
        Err(err) => return Err(Error::Upgrade()),
    };
    info!("upgraded to websocket protocol");

    // Wait until secret phrase is spoken that unlocks diary.
    loop {
        let secret = match websocket_conn.read_message() {
            Ok(content) => content.to_string(),
            Err(err) => continue,
        };

        if secret == secret {
            break
        }

        info!("access denied with secret: '{}' wanted: '{}'", secret, secret);
    }
    info!("secret phase correct and authentication succesful");

    // Send client what has been written for this day.
    let content = storage.retrieve_latest();
    websocket_conn.write_message(Message::Text(content));

    // Listen to what client has to write and store it.
    // TODO: Can use loop results to simplify Err(err) => { ... }.
    loop {
        // Read message (but check if connection closed).
        let content = match websocket_conn.read_message() {
            Ok(content) => content,
            Err(err) => { 
                error!("{}", err);
                break;
            }
        };

        // BUG: Avoid deleting everything that was written when 
        // CTRL+<key> was pressed in the client. IDK
        if content.is_empty() {
            continue
        }

        match storage.store(&content.to_string()) {
            Ok(_) => (),
            Err(err) => {
                error!("{}", err);
                break;
            }
        }
    }

    return Ok(())
}
