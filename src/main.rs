use chrono;
use env_logger;
use ini;
use std::{
    error, fs,
    io::{prelude::*, BufReader, Write},
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
    let config = ini::Ini::load_from_file(config_path)?;
    let core_section = config.section(Some("core")).unwrap();
    let server_address =
        IpAddr::from_str(core_section.get("server_address").unwrap_or("127.0.0.1"))?.into();
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
    let response: String = build_response(&request_line)?;
    let response_first_line = response.lines().next().unwrap();
    log::info!("{request_line} -- {response_first_line}");
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn get_log_builder(format: Option<&'static str>) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();
    let log_format = format.unwrap_or("%Y-%m-%dT%H:%M:%S");
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format(log_format),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info);
    builder
}

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let config_path = path::Path::new("config/server.conf");

    // todo: read log format from the config
    let mut log_builder = get_log_builder(None);
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
