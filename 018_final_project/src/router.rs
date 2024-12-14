use crate::{
    request::{Method, Request},
    response::Response,
};
use std::{
    collections::HashMap,
    io::{ErrorKind, Result},
};

type Handler = fn(&Request) -> Response;

pub struct Router {
    get: HashMap<&'static str, Handler>,
    post: HashMap<&'static str, Handler>,
    put: HashMap<&'static str, Handler>,
    delete: HashMap<&'static str, Handler>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            get: HashMap::new(),
            post: HashMap::new(),
            put: HashMap::new(),
            delete: HashMap::new(),
        }
    }

    pub fn get(&mut self, uri: &'static str, handler: Handler) {
        self.get.entry(uri).insert_entry(handler);
    }

    pub fn post(&mut self, uri: &'static str, handler: Handler) {
        self.post.entry(uri).insert_entry(handler);
    }

    pub fn put(&mut self, uri: &'static str, handler: Handler) {
        self.put.entry(uri).insert_entry(handler);
    }

    pub fn delete(&mut self, uri: &'static str, handler: Handler) {
        self.delete.entry(uri).insert_entry(handler);
    }

    pub fn handle(&self, req: &Request) -> Result<Response> {
        let handler = match req.method() {
            Method::Get => self.get.get(req.uri()),
            Method::Post => self.post.get(req.uri()),
            Method::Put => self.put.get(req.uri()),
            Method::Delete => self.delete.get(req.uri()),
        }
        .ok_or(ErrorKind::NotFound)?;

        Ok(handler(req))
    }
}
