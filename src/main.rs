use std::{
    io::{prelude::*, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    // from the docs:
    // "A BufReader<R> performs large, infrequent reads on the underlying Read
    // and maintains an in-memory buffer of the results."
    let buf_reader = BufReader::new(&mut stream);

    let request: Vec<String> = buf_reader
        // returns an iterator over the lines of buf_reader
        .lines()
        .map(|result| result.unwrap()) // todo: handle this more gracefully
        // returns an iterator that yields lines from the reader when they're
        // non-empty and ignores the rest
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", request);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // todo: read from a config file later?
    let socket_address = SocketAddr::from(([127, 0, 0, 1], 5006));

    // create a TcpListener and bind it to the socket IP address on the specified port
    let listener = TcpListener::bind(socket_address)?;
    println!("Listening on port {}...", socket_address.port());

    // accept connections and process them one by one
    for stream in listener.incoming() {
        // stream represents an open connection between client and server
        handle_connection(stream?);
    }
    Ok(())
}
