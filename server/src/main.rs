use std::fmt::{Debug, Formatter, Display};
use std::{thread, time};
use std::net::{TcpListener, TcpStream};

use simple_logger::SimpleLogger;
use log::{info, warn, error, SetLoggerError};

use postgres::{Client, NoTls, error::DbError};
use tungstenite::{accept, WebSocket, Message};

const QUERY_SCHEMA: &str = "
    CREATE TABLE IF NOT EXISTS entries (
        date        TIMESTAMP PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
        content     TEXT NOT NULL
    )
";

const QUERY_SELECT: &str = "
    SELECT content FROM entries WHERE date = date_trunc('day', CURRENT_TIMESTAMP);
";

/// Declare all errors that can occur when parsing a gitignore file.
#[derive(Debug)]
enum Error {
    /// An error initializing the logger. From reading the lib code this should not occur.
    LoggerInit(SetLoggerError),

    /// An error in the SQL statement that can't be fixed during runtime.
    SqlSyntax(postgres::Error),

    /// An error while accepting the TCP connection.
    TcpConnection(std::io::Error),

    /// An error ...
    ReadMessage(tungstenite::error::Error),
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::LoggerInit(err) => {
                write!(f, "could not initialize logger: {err}")
            },
            Error::SqlSyntax(err) => {
                write!(f, "could not define schema: {err}")
            },
            Error::TcpConnection(err) => {
                write!(f, "could not accept TCP connection: {err}")
            },
            Error::ReadMessage(err) => {
                write!(f, "could not accept TCP connection: {err}")
            },
        }
    }
}

fn main() -> Result<(), Error> {
    // Initialize logger.
    match SimpleLogger::new().env().init() {
        Ok(_) => (),
        Err(err) => return Err(Error::LoggerInit(err)),
    }
    info!("logger initalized");

    // Endlessly try to connect.
    let mut client = loop {
        let mut connect_attempt = Client::connect("host=localhost user=postgres password=example", NoTls);
        match connect_attempt {
            Ok(client) => break client,
            Err(e) => warn!("{}", e),
        }

        thread::sleep(time::Duration::from_secs(3));
    };
    info!("connected to database");

    // Init database if it's the first time.
    match client.batch_execute(QUERY_SCHEMA) {
        Ok(_) => (),
        Err(err) => return Err(Error::SqlSyntax(err)),
    };

    // Listen for websocket connections.
    let server = match TcpListener::bind("127.0.0.1:4123") {
        Ok(server) => server,
        Err(err) => todo!(),
    };

    for incoming in server.incoming() {
        // Accept TCP connection.
        let stream = match incoming {
            Ok(stream) => stream,
            Err(err) => return Err(Error::TcpConnection(err)),
        };

        // Upgrade to Websocket connection.
        let mut websocket_conn = match accept(stream) {
            Ok(websocket_conn) => websocket_conn,
            Err(err) => {
                warn!("{}", err); 
                break;
            }
        };

        // Send client what has been written for this day.
        let row = match client.query_one(QUERY_SELECT, &[]) {
            Ok(row) => row,
            Err(err) => panic!("{}", err),
        };
        let content: String = row.get("content");
        websocket_conn.write_message(Message::Text(content));

        // Listen to what client has to write and store it.
        loop {
            match store(&mut client, &mut websocket_conn) {
                Ok(_) => (),
                Err(err) => {
                    error!("{}", err);
                    break;
                }
            }
        }
    }

    Ok(())
}

fn store(client: &mut Client, connection: &mut WebSocket<TcpStream>) -> Result<(), Error> {
    // Read message (but check if connection closed).
    let content = match connection.read_message() {
        Ok(content) => content,
        Err(err) => return Err(Error::ReadMessage(err))
    };

    client.execute("
       INSERT INTO entries (content, date) 
       VALUES ($1, date_trunc('day', CURRENT_TIMESTAMP))
       ON CONFLICT (date) 
       DO UPDATE SET content = $1
    ", &[&content.to_string()]);

    Ok(())
}
