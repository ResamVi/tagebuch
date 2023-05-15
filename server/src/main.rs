use std::env::VarsOs;
use std::fmt::{Debug, Formatter, Display};
use std::{thread, time, env};
use std::net::{TcpListener, TcpStream};

use simple_logger::SimpleLogger;
use log::{info, warn, error, SetLoggerError, debug};
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

    /// An error in the SQL statement that sets up the schema that can't be fixed during runtime.
    SchemaInit(postgres::Error),

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
            Error::SchemaInit(err) => {
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

struct Config {
    secret: String,
    host: String,
    port: String,
    database_host: String,
    database_user: String,
    database_password: String,
    database_port: String,
}

impl Config {
    fn from_env() -> Self {
        let secret = match env::var_os("SECRET") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "sesam öffne dich".to_string(),
        };
            
        let host = match env::var_os("HOST") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "0.0.0.0".to_string(),
        };
        let port = match env::var_os("PORT") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "4123".to_string(),
        };

        let database_host = match env::var_os("DB_HOST") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "localhost".to_string(),
        };

        let database_user = match env::var_os("DB_USER") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "postgres".to_string(),
        };

        let database_password = match env::var_os("DB_PASS") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "example".to_string(),
        };

        let database_port = match env::var_os("DB_PORT") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "5433".to_string(),
        };

        Config {
            secret,
            host,
            port,
            database_host,
            database_user,
            database_password,
            database_port,
        }
    }

    fn connection_string(&self) -> String {
        format!("host={} user={} password={} port={}", 
                self.database_host, 
                self.database_user, 
                self.database_password, 
                self.database_port
        )
    }

    fn bind_address(&self) -> String {
        format!("{0}:{1}", self.host, self.port)
    }
}

struct Postgres {
    client: Client
} 

impl Postgres {
    fn new(config: &Config) -> Result<Postgres, Error> {
        // Endlessly try to connect.
        let mut client = loop {
            let mut connect_attempt = Client::connect(&config.connection_string(), NoTls);
            match connect_attempt {
                Ok(client) => break client,
                Err(e) => warn!("{}", e),
            }

            thread::sleep(time::Duration::from_secs(3));
        };

        // Init database if it's the first time we started up.
        match client.batch_execute(QUERY_SCHEMA) {
            Ok(_) => (),
            Err(err) => return Err(Error::SchemaInit(err)),
        };
        info!("schema initialized");

        Ok(Postgres {
            client
        })
    }
}

fn main() -> Result<(), Error> {
    // Initialize logger.
    match SimpleLogger::new().env().init() {
        Ok(_) => (),
        Err(err) => return Err(Error::LoggerInit(err)),
    }
    info!("logger initalized");

    // Load configurations from environment variables.
    let config = Config::from_env();
    info!("configs loaded: {}", config.connection_string());

    // Initialize sink where diary entries are stored.
    let store = match Postgres::new(config) {
        Ok(store) => store,
        Err(err) => return Err(err),
    };
    info!("database connection established");


    // Listen for websocket connections.
    let server = match TcpListener::bind(config.bind_address()) {
        Ok(server) => server,
        Err(err) => return Err(Error::TcpConnection(err)),
    };
    info!("listening to connections on port {}", config.port);

    for incoming in server.incoming() {
        // Accept TCP connection.
        let stream = match incoming {
            Ok(stream) => stream,
            Err(err) => return Err(Error::TcpConnection(err)),
        };
        info!("accepted TCP connection");

        // Upgrade to Websocket connection.
        let mut websocket_conn = match accept(stream) {
            Ok(websocket_conn) => websocket_conn,
            Err(err) => {
                warn!("{}", err); 
                break;
            }
        };
        info!("upgraded to websocket protocol");


        // Wait until secret phrase is spoken that unlocks diary.
        loop {
            let secret = match websocket_conn.read_message() {
                Ok(content) => content.to_string(),
                Err(err) => continue,
            };

            if secret == config.secret {
                break
            }

            info!("access denied with secret: '{}' wanted: '{}'", secret, config.secret);
        }

        // Send client what has been written for this day.
        let content = match client.query_one(QUERY_SELECT, &[]) {
            Ok(row) => row.get("content"),
            Err(err) => "".to_string(), 
        };
        info!("sent current text {}", content);

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
    info!("stopped listening to TCP connections. bye");

    Ok(())
}

fn store(client: &mut Client, connection: &mut WebSocket<TcpStream>) -> Result<(), Error> {
    // Read message (but check if connection closed).
    let content = match connection.read_message() {
        Ok(content) => content,
        Err(err) => return Err(Error::ReadMessage(err))
    };

    // BUG: Avoid deleting everything that was written when 
    // CTRL+<key> was pressed in the client. IDK
    if content.is_empty() {
        return Ok(())
    }

    // TODO: Check for error
    client.execute("
       INSERT INTO entries (content, date) 
       VALUES ($1, date_trunc('day', CURRENT_TIMESTAMP))
       ON CONFLICT (date) 
       DO UPDATE SET content = $1
    ", &[&content.to_string()]);

    info!("stored user input");
    Ok(())
}
