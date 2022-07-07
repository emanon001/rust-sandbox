use anyhow::Result;
use log::{debug, error};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

pub fn serve(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address)?;
    loop {
        let (stream, _) = listener.accept()?;
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|e| error!("{:?}", e));
        });
    }
}

fn handler(mut stream: TcpStream) -> Result<()> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes])?;
    }
}
