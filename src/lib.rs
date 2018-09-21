#[macro_use]
extern crate failure;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

#[derive(Debug, Fail)]
pub enum WhaleError {
    #[fail(display = "failed to connect to '{}': {:?}", _0, _1)]
    ConnectError { addr: String, e: String },
    #[fail(display = "failed to write to socket: {}", _0)]
    WriteError { e: String },
    #[fail(display = "failed to read to socket: {}", _0)]
    ReadError { e: String },
}
pub struct DockerClient {
    socket: UnixStream,
}

impl DockerClient {
    pub fn new(addr: &str) -> Result<DockerClient, WhaleError> {
        match UnixStream::connect(addr) {
            Ok(s) => Ok(DockerClient { socket: s }),
            Err(e) => {
                Err(WhaleError::ConnectError {
                    addr: String::from(addr),
                    e: e.to_string(),
                })
            }
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
