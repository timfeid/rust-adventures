pub mod request;
pub mod response;

use request::Request;
use response::Response;
use std::{
    io::{BufRead, BufReader, Result},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct HandleClientError {}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        if let Err(e) = handle_client(stream?) {
            println!("Something went wrong {e}")
        };
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let buffer = BufReader::new(&mut stream);
    let lines: Vec<String> = buffer
        .lines()
        .map(|line| line.unwrap())
        .take_while(|v| !v.is_empty())
        .collect();

    let request = Request::new(&lines);

    println!("{:#?}", request.get_headers());

    let mut response = match request.get_path().as_str() {
        "/" => Response::ok(Some(
            "<html><head></head><body>test</body></html>".to_owned(),
        )),
        _ => Response::not_found(Some(
            "<html><head></head><body>Not found</body></html>".to_owned(),
        )),
    };

    response.send(&stream)?;
    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}
