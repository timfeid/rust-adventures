use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
}

#[derive(Debug)]
pub struct Request {
    headers: HashMap<String, String>,
    method: Method,
    path: String,
}

impl Request {
    fn parse_start_line(lines: &[String]) -> (Method, String) {
        let mut method = Method::Get;
        let mut path = "/";

        if let Some(start_line) = lines.first() {
            let mut parts = start_line.split_whitespace();
            method = match parts.next() {
                Some("POST") => Method::Post,
                _ => Method::Get,
            };
            path = parts.next().unwrap_or("/");
        }

        (method, path.to_owned())
    }

    pub fn new(lines: &[String]) -> Self {
        let mut headers: HashMap<String, String> = HashMap::new();
        let (method, path) = Request::parse_start_line(lines);

        for line in &lines[1..] {
            let header_parts: Vec<&str> = line.split(':').collect();
            headers.insert(
                header_parts.first().unwrap().to_string(),
                header_parts[1..].join(":").trim().to_owned(),
            );
        }

        Request {
            headers,
            method,
            path,
        }
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

#[cfg(test)]
mod tests {
    use super::{Method, Request};

    #[test]
    fn parses_post_request() {
        let lines = vec![
            String::from("POST / HTTP/1.1"),
            String::from("Host: localhost:3000"),
        ];

        let request = Request::new(&lines);
        assert_eq!(request.get_method(), &Method::Post);
    }

    #[test]
    fn parses_path() {
        let path = "/first/second";
        let lines = vec![
            format!("POST {} HTTP/1.1", path),
            String::from("Host: localhost:3000"),
        ];

        let request = Request::new(&lines);
        println!("{:#?}", request.get_path());
        assert_eq!(request.get_path(), &path);
    }

    #[test]
    fn parses_headers() {
        let lines = vec![
            String::from("POST / HTTP/1.1"),
            String::from("Host: localhost:3000"),
        ];

        let request = Request::new(&lines);
        assert_eq!(request.get_headers().get("Host").unwrap(), "localhost:3000");
    }
}
