use std::{fs, io::Result};

use crate::status_code;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Response {
    status: status_code::StatusCode,
    headers: Vec<String>,
    content: String,
}

impl Response {
    pub fn return_file(status: status_code::StatusCode, path: &str) -> Result<Response> {
        let mut headers = vec![];

        let content = fs::read_to_string(path)?;
        headers.push(format!("Content-Length: {}", content.len()));

        Ok(Response {
            status,
            headers,
            content,
        })
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        let code = self.status.code;
        let reason = self.status.reason;
        let headers = self.headers.join("\r\n");
        let content = self.content;
        format!("HTTP/1.1 {code} {reason}\r\n{headers}\r\n\r\n{content}")
    }
}
