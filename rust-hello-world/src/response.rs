use std::{collections::HashMap, net::TcpStream, io::Write};

use chrono::Utc;

enum Status {
    Ok,
    NotFound,
}

pub struct Response {
    status_code: Status,
    body: String,
    headers: HashMap<String, String>,
}

impl Response {
    fn default_headers() -> HashMap<String, String> {
        let mut map = HashMap::new();
        let now = Utc::now();
        let date = now.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        map.insert("Content-Type".to_owned(), "text/html".to_owned());
        map.insert("Date".to_owned(), date);
        map.insert("Connection".to_owned(), "Closed".to_owned());

        map
    }

    fn new(status_code: Status) -> Self {
        Response {
            status_code,
            body: String::from(""),
            headers: Response::default_headers(),
        }
    }

    pub fn set_body(&mut self, body: Option<String>) -> &mut Self {
        self.body = match body {
            Some(body) => body,
            None => String::from(""),
        };

        self
    }

    pub fn ok(body: Option<String>) -> Self {
        let mut response = Response::new(Status::Ok);
        response.set_body(body);

        response
    }

    pub fn not_found(body: Option<String>) -> Self {
        let mut response = Response::new(Status::NotFound);
        response.set_body(body);

        response
    }

    pub fn send(&mut self, mut stream: &TcpStream) -> std::result::Result<(), std::io::Error> {
        self.headers.insert("Content-Length".to_owned(), self.body.len().to_string());
        stream.write_all(&self.to_string().as_bytes())
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let response = format!("HTTP/1.1 {}", &get_response_code(&self.status_code));
        let headers: Vec<String> = self.headers
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect();

        response + "\r\n" + &headers.join("\r\n") + "\r\n\r\n" + &self.body
    }
}

fn get_response_code(status_code: &Status) -> String {
    match status_code {
        Status::Ok => "200 OK".to_string(),
        Status::NotFound => "404 Not Found".to_string(),
    }
}
