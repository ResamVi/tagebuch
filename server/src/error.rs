use std::fmt::{Display, Formatter};

use log::SetLoggerError;

/// Declare all errors that can occur when parsing a gitignore file.
#[derive(Debug)]
pub enum Error {
    /// An error initializing the logger. From reading the lib code this should not occur.
    LoggerInit(SetLoggerError),

    /// An error when setting up the storage (e.g. SQL statement that sets up the schema failed).
    Init(),

    /// An error while accepting the TCP connection.
    TcpConnection(std::io::Error),

    /// An error ...
    ReadMessage(tungstenite::error::Error),

    /// An error ...
    Storing(),
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::LoggerInit(err) => {
                write!(f, "could not initialize logger: {err}")
            },
            Error::Init() => {
                write!(f, "could not initialize storage")
            },
            Error::TcpConnection(err) => {
                write!(f, "could not accept TCP connection: {err}")
            },
            Error::ReadMessage(err) => {
                write!(f, "could not accept TCP connection: {err}")
            },
            Error::Storing() => {
                write!(f, "could not store content")
            },
        }
    }
}

