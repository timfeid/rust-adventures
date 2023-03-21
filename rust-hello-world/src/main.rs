use std::{net::{TcpListener, TcpStream}, io::{Result, Read, BufRead}, str};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let buffer_size: usize = stream.read(&mut buf).unwrap();

    let request = str::from_utf8(&buf[..buffer_size]).unwrap();

    println!("{}", request);
}
