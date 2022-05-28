use crate::response::{grfsc, ResponseType, StatusCode};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpStream};

pub struct CHC {
    pub stream: TcpStream,
    pub buffer: Vec<u8>,
}

impl CHC {
    pub fn new(stream: TcpStream, buffer: Vec<u8>) -> Self {
        CHC { stream, buffer }
    }

    pub fn respond(&mut self, body: String) {
        self.stream.write(body.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        let mut lines = self.buffer.split(|&b| b == b'\n');
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            } else if line.starts_with(b"Cookie:") {
                continue;
            }

            let mut parts = line.splitn(2, |&b| b == b':');
            let key_final: String;
            let value_final: String;

            if let Some(key) = parts.next() {
                let temp_key = std::str::from_utf8(key).unwrap();
                key_final = temp_key.to_string().trim().to_string();
            } else {
                continue;
            }
            if let Some(value) = parts.next() {
                let temp_value = std::str::from_utf8(value).unwrap();
                value_final = temp_value.to_string().trim().to_string();
            } else {
                continue;
            }

            headers.insert(key_final, value_final);
        }

        headers
    }

    pub fn cookies(&self) -> HashMap<String, String> {
        let mut cookies = HashMap::new();
        let mut lines = self.buffer.split(|&b| b == b'\n');
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            } else if !line.starts_with(b"Cookie:") {
                continue;
            }

            let line = &line[7..];
            // split on ';'
            let mut parts = line.split(|&b| b == b';');

            while let Some(part) = parts.next() {
                let mut parts = part.splitn(2, |&b| b == b'=');
                let key_final: String;
                let value_final: String;

                if let Some(key) = parts.next() {
                    let temp_key = std::str::from_utf8(key).unwrap();
                    key_final = temp_key.to_string().trim().to_string();
                } else {
                    continue;
                }
                if let Some(value) = parts.next() {
                    let temp_value = std::str::from_utf8(value).unwrap();
                    value_final = temp_value.to_string().trim().to_string();
                } else {
                    continue;
                }

                cookies.insert(key_final, value_final);
            }
        }

        cookies
    }
}
