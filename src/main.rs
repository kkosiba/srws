/* standard library imports */
use std::{
    error,
    io::{prelude::*, BufReader, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    path, result,
    str::FromStr,
};

/* local modules */
mod logging;
mod responses;

fn get_core_config(
    config_path: &path::Path,
) -> result::Result<(IpAddr, String), Box<dyn error::Error>> {
    let config = ini::Ini::load_from_file(config_path)?;
    let core_section = config.section(Some("core")).unwrap();
    let server_address =
        IpAddr::from_str(core_section.get("server_address").unwrap_or("127.0.0.1"))?;
    let port = core_section.get("port").unwrap_or("5006");
    Ok((server_address, port.to_string()))
}

fn get_listener(
    server_address: IpAddr,
    port: &str,
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
    let response: String = responses::build_response(&request_line)?;
    let response_first_line = response.lines().next().unwrap();
    log::info!("{request_line} -- {response_first_line}");
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let config_path = path::Path::new("config/server.conf");

    // todo: read log format from the config
    let mut log_builder = logging::get_log_builder(None);
    log_builder.init();

    let (server_address, port) = get_core_config(config_path)?;
    let listener = get_listener(server_address, &port)?;
    log::info!("Listening on {}:{}...", &server_address, &port,);

    // accept connections and process them one by one
    for stream in listener.incoming() {
        // stream represents an open connection between client and server
        handle_connection(stream?)?;
    }
    Ok(())
}
