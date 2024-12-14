use std::{
    io::{prelude::*, BufReader, Error, ErrorKind, Result},
    net::TcpStream,
};

use crate::response::Response;

#[derive(Debug)]
pub struct Request {
    method: Method,
    uri: String,
    headers: Vec<(String, String)>,
    content: String,
    context: TcpStream,
}

impl Request {
    pub fn parse(stream: TcpStream) -> Result<Request> {
        let mut raw_buf = BufReader::new(&stream);
        let mut buf = raw_buf.by_ref().lines();

        let raw_req = buf.next().ok_or(ErrorKind::InvalidData)??;
        let mut req_parts = raw_req.split(" ");

        let method = Method::try_from(req_parts.next().ok_or(ErrorKind::InvalidData)?)?;
        let uri = String::from(req_parts.next().ok_or(ErrorKind::InvalidData)?);

        let headers = {
            let mut data = vec![];
            for line in buf.by_ref() {
                let mut line = line?;
                if line.is_empty() {
                    break;
                }

                let mut header_data = line.split_off(line.find(':').ok_or(ErrorKind::InvalidData)?);
                header_data.remove(0);
                let header_data = header_data.trim().to_string();

                data.push((line, header_data));
            }
            data
        };

        let mut content = String::new();
        if let Some((_, length)) = headers.iter().find(|d| d.0 == "Content-Length") {
            let content_length: usize = length.trim().parse().ok().ok_or(ErrorKind::InvalidData)?;

            let mut buf = vec![0; content_length];
            raw_buf.read_exact(&mut buf)?;
            content = String::from_utf8(buf).ok().ok_or(ErrorKind::InvalidData)?;
        }

        Ok(Request {
            method,
            uri,
            headers,
            content,
            context: stream,
        })
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn respond(&mut self, resp: Response) -> Result<()> {
        let resp: String = resp.into();
        self.context.write_all(resp.as_bytes())
    }
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl TryFrom<&str> for Method {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            _ => Err(ErrorKind::InvalidInput.into()),
        }
    }
}
