use std::fmt::{Display, Formatter};

use log::SetLoggerError;

/// Declare all errors that can occur when parsing a gitignore file.
#[derive(Debug)]
pub enum Error {
    /// An error initializing the logger. From reading the lib code this should not occur.
    LoggerInit(SetLoggerError),

    /// An error when setting up the storage (e.g. SQL statement that sets up the schema failed).
    Init(&'static str),

    /// An error while accepting the TCP connection.
    TcpConnection(std::io::Error),

    /// ...
    Upgrade(),

    /// An error ...
    ReadMessage(tungstenite::error::Error),

    /// An error occured while trying to store the contents into the diary sink.
    Storing(&'static str),
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::LoggerInit(err) => {
                write!(f, "could not initialize logger: {err}")
            },
            Error::Init(msg) => {
                write!(f, "could not initialize storage: {msg}")
            },
            Error::TcpConnection(err) => {
                write!(f, "could not accept TCP connection: {err}")
            },
            Error::Upgrade() => {
                write!(f, "could not upgrade TCP connection to websocket")
            },
            Error::ReadMessage(err) => {
                write!(f, "could not accept TCP connection: {err}")
            },
            Error::Storing(err) => {
                write!(f, "could not store content: {err}")
            },
        }
    }
}

