use std::{error, fs, result};

pub fn determine_response(request_line: &str) -> (&str, &str) {
    // todo: read routing config from config/routes.conf
    match request_line {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    }
}

pub fn build_response(request_line: &str) -> result::Result<String, Box<dyn error::Error>> {
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
