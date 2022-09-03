use std::net::{SocketAddr, TcpListener, TcpStream};

fn handle_connection(_stream: TcpStream) {
    println!("Connection established!");
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
