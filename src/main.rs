use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
};

fn determine_response(request_line: &str) -> (&str, &str) {
    if request_line.contains("GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    }
}

fn build_response(request_line: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (status_line, filename) = determine_response(request_line);
    let contents: String = fs::read_to_string(format!("static/{filename}"))?;
    let content_length: usize = contents.len();
    let response: String = format!(
        "{status_line}\r\n\
        Content-Length: {content_length}\r\n\r\n\
        {contents}"
    );
    Ok(response)
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // from the docs:
    // "A BufReader<R> performs large, infrequent reads on the underlying Read
    // and maintains an in-memory buffer of the results."
    let buf_reader = BufReader::new(&mut stream);
    // todo: don't use unwrap() here, instead handle Option with match
    let request_line = buf_reader.lines().next().unwrap()?;
    let response: String = build_response(&request_line)?;
    stream.write_all(response.as_bytes())?;
    Ok(())
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
        handle_connection(stream?)?;
    }
    Ok(())
}
