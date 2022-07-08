use anyhow::{anyhow, Result};
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};
use std::{env, str};

const WEBROOT: &str = "/public";

pub fn make_response(buffer: &[u8]) -> Result<Vec<u8>> {
    let http_pattern = Regex::new(r"(.*) (.*) HTTP/1.([0-1])\r\n.*")?;
    let captures = match http_pattern.captures(str::from_utf8(buffer)?) {
        Some(cap) => cap,
        None => return create_msg_from_code(400, None),
    };

    let method = &captures[1];
    let path = format!(
        "{}{}{}",
        env::current_dir()?.display(),
        WEBROOT,
        &captures[2]
    );

    match method {
        "GET" => {
            let file = match File::open(path) {
                Ok(file) => file,
                Err(_) => {
                    return create_msg_from_code(404, None);
                }
            };
            let mut reader = BufReader::new(file);
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            create_msg_from_code(200, Some(buf))
        }
        _ => create_msg_from_code(501, None),
    }
}

fn create_msg_from_code(status_code: u16, msg: Option<Vec<u8>>) -> Result<Vec<u8>> {
    match status_code {
        200 => {
            let mut header = "HTTP/1.0 200 OK\r\nServer: mio webserver\r\n\r\n"
                .to_string()
                .into_bytes();
            if let Some(mut msg) = msg {
                header.append(&mut msg);
            }
            Ok(header)
        }
        400 => Ok("HTTP/1.0 400 Bad Request\r\nServer: mio webserver\r\n\r\n"
            .to_string()
            .into_bytes()),
        404 => Ok("HTTP/1.0 404 Not Found\r\nServer: mio webserver\r\n\r\n"
            .to_string()
            .into_bytes()),
        501 => Ok(
            "HTTP/1.0 501 Not Implemented\r\nServer: mio webserver\r\n\r\n"
                .to_string()
                .into_bytes(),
        ),
        _ => Err(anyhow!("Undefined status code.")),
    }
}
