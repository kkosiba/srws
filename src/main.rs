extern crate ini;

use ini::Ini;
use std::{
    error, fs,
    io::{prelude::*, BufReader},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    path, result,
    str::FromStr,
};

fn determine_response(request_line: &str) -> (&str, &str) {
    // todo: read routing config from config/routes.conf
    match request_line {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    }
}

fn build_response(request_line: &str) -> result::Result<String, Box<dyn error::Error>> {
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

fn get_core_config(
    config_path: &path::Path,
) -> result::Result<(IpAddr, String), Box<dyn error::Error>> {
    let config = Ini::load_from_file(config_path)?;
    let core_section = config.section(Some("core")).unwrap();
    let server_address = IpAddr::from_str(core_section.get("server_address").unwrap())?.into();
    let port = core_section.get("port").unwrap();
    Ok((server_address, port.to_string()))
}

fn get_listener(
    server_address: IpAddr,
    port: String,
) -> result::Result<TcpListener, Box<dyn error::Error>> {
    let socket_address = SocketAddr::new(server_address, port.parse::<u16>()?);
    // create a TcpListener and bind it to the socket address on the specified port
    let listener = TcpListener::bind(socket_address)?;
    Ok(listener)
}

fn handle_connection(mut stream: TcpStream) -> result::Result<(), Box<dyn error::Error>> {
    // from the docs:
    // "A BufReader<R> performs large, infrequent reads on the underlying Read
    // and maintains an in-memory buffer of the results."
    let buf_reader = BufReader::new(&mut stream);
    // todo: don't use unwrap() here, instead handle Option with match
    let request_line = buf_reader.lines().next().unwrap()?;
    let response: String = build_response(&request_line)?;
    let response_first_line = response.lines().next().unwrap();
    println!("{request_line} -- {response_first_line}");
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let config_path = path::Path::new("config/server.conf");
    let (server_address, port) = get_core_config(config_path)?;
    let listener = get_listener(server_address, port)?;
    println!("Listening on port {}...", &listener.local_addr()?.port());

    // accept connections and process them one by one
    for stream in listener.incoming() {
        // stream represents an open connection between client and server
        handle_connection(stream?)?;
    }
    Ok(())
}
