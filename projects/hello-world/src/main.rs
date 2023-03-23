pub mod request;
pub mod response;

use request::Request;
use response::Response;
use std::{
    io::{BufRead, BufReader, Result},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
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

    match response.send(&stream) {
        Ok(_) => {
            println!("success!\n{}", response.to_string())
        }
        Err(e) => {
            println!("oh no, {}", e)
        }
    }

    match stream.shutdown(std::net::Shutdown::Both) {
        Ok(_) => println!("connection closed"),
        Err(_) => println!("something went wrong trying to close the socket"),
    }
}
