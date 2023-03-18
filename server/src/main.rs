use postgres::{Client, NoTls, error::DbError};
use std::{thread, time};
use std::net::{TcpListener, TcpStream};
use tungstenite::{accept, WebSocket};

fn main() {
    // Endlessly try to connect.
    let mut client = loop {
        let mut connect_attempt = Client::connect("host=localhost user=postgres password=example", NoTls);
        match connect_attempt {
            Ok(client) => break client,
            Err(e) => println!("{:?}", e),
        }

        thread::sleep(time::Duration::from_secs(3));
    };
    println!("connected to database");

    // Init database if first time.
    let schema_defined = client.batch_execute("
        CREATE TABLE IF NOT EXISTS entries (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            date    TIMESTAMP
        )
    ");
    if schema_defined.is_err() {
        panic!("the query was written in a way that's not idempotent")
    }

    // Listen for websocket connections.
    let server = TcpListener::bind("127.0.0.1:4123").unwrap();
    for stream in server.incoming() {
        let mut websocket_conn = accept(stream.unwrap()).unwrap();
        loop {
            if store(&mut websocket_conn) {
                break
            }
        }
    }
}

fn store(connection: &mut WebSocket<TcpStream>) -> bool {
    // Check if connection closed.
    let msg = connection.read_message();
    match msg {
        Ok(content) => println!("{:?}", content),
        Err(e) => return true,
    }

    return false;
}
