#[macro_use]
extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
mod container;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

static API_VERSION: &'static str = "v1.37";
#[derive(Debug, Fail)]
pub enum WhaleError {
    #[fail(display = "failed to connect to '{}': {:?}", _0, _1)]
    ConnectError { addr: String, e: String },
    #[fail(display = "failed to write to socket: {}", _0)]
    WriteError { e: String },
    #[fail(display = "failed to read to socket: {}", _0)]
    ReadError { e: String },
    #[fail(display = "failed to format api request")]
    RequestError {},
}
pub struct DockerClient {
    socket: UnixStream,
}

impl DockerClient {
    pub fn new(addr: &str) -> Result<DockerClient, WhaleError> {
        match UnixStream::connect(addr) {
            Ok(s) => Ok(DockerClient { socket: s }),
            Err(e) => Err(WhaleError::ConnectError {
                addr: String::from(addr),
                e: e.to_string(),
            }),
        }
    }

    pub fn get_all_container(&mut self) -> Result<Vec<container::Container>, WhaleError> {
        let req = match get_formatted_api_request("/containers/json?all=1", "GET", "") {
            Some(r) => r,
            None => return Err(WhaleError::RequestError {}),
        };

        println!("{}", req);

        let resp = self.request_to_string(&req)?;

        let container: Result<Vec<container::Container>, serde_json::Error> =
            serde_json::from_str(&resp);

        match container {
            Ok(c) => Ok(c),
            Err(e) => Err(WhaleError::ReadError { e: e.to_string() }),
        }
    }

    pub fn request(&mut self, req: &str) -> Result<Vec<u8>, WhaleError> {
        let mut sock = &self.socket;
        let data = req.as_bytes();

        match sock.write_all(data) {
            Err(e) => return Err(WhaleError::WriteError { e: e.to_string() }),
            _ => {}
        };

        let mut buf = Vec::new();
        match sock.read_to_end(&mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(WhaleError::ReadError { e: e.to_string() }),
        }
    }

    pub fn request_to_string(&mut self, req: &str) -> Result<String, WhaleError> {
        let buf = self.request(req)?;

        match String::from_utf8(buf) {
            Ok(s) => Ok(s),
            Err(e) => Err(WhaleError::ReadError {
                e: format!("could not parse vec into string: {}", e),
            }),
        }
    }
}
fn get_formatted_api_request(api_endpoint: &str, method: &str, body: &str) -> Option<String> {
    if method == "GET" || method == "get" {
        return Some(format!(
            "GET {endpoint}{body} HTTP/1.1\r\nHost: {version}\r\n\r\n",
            endpoint = api_endpoint,
            body = body,
            version = API_VERSION
        ));
    }

    if method == "POST" || method == "post" {
        return Some(format!(
            "POST {endpoint} HTTP/1.1\r\nHost: {version}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{external_body}\r\n\r\n",
            endpoint = api_endpoint,
            version = API_VERSION,
            length = body.len(),
            external_body = body
        ));
    }

    None
}
