use log::info;
use postgres::{Client, NoTls, error::DbError};
use std::{thread, time};
use std::net::{TcpListener, TcpStream};
use tungstenite::{accept, WebSocket};
use simple_logger;

#[macro_use] 
use log;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    // Endlessly try to connect.
    let mut client = loop {
        let mut connect_attempt = Client::connect("host=localhost user=postgres password=example", NoTls);
        match connect_attempt {
            Ok(client) => break client,
            Err(e) => info!("{}", e),
        }

        thread::sleep(time::Duration::from_secs(3));
    };
    println!("connected to database");

    // Init database if first time.
    let schema_defined = client.batch_execute("
        CREATE TABLE IF NOT EXISTS entries (
            date        TIMESTAMP PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
            content     TEXT NOT NULL
        )
    ");
    if schema_defined.is_err() {
        panic!("the query was written with an error")
    }

    // Listen for websocket connections.
    let server = TcpListener::bind("127.0.0.1:4123").unwrap();
    for stream in server.incoming() {
        let mut websocket_conn = accept(stream.unwrap()).unwrap();

        // 
        // websocket_conn.write_message()

        loop {
            if store(&mut client, &mut websocket_conn) {
                break
            }
        }
    }
}

fn store(client: &mut Client, connection: &mut WebSocket<TcpStream>) -> bool {
    // Check if connection closed.
    let content = match connection.read_message() {
        Ok(content) => content,
        Err(_) => panic!("idk man how to do rust errors"),
    };

    client.execute("
       INSERT INTO entries (content, date) 
       VALUES ($1, date_trunc('day', CURRENT_TIMESTAMP))
       ON CONFLICT (date) 
       DO UPDATE SET content = $1
    ", &[&content.to_string()]);

    return false;
}
