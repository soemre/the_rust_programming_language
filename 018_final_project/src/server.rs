use std::{
    io::{Error, ErrorKind, Result},
    net::{Incoming, TcpListener},
};

use crate::{request::Request, router::Router};

pub mod request;
pub mod response;

pub struct HttpServer {
    listener: TcpListener,
    router: Router,
    error_handler: Box<dyn Fn(Option<Request>, Error)>,
}

impl HttpServer {
    pub fn new<T>(port: u32, router: Router, error_handler: T) -> Result<HttpServer>
    where
        T: Fn(Option<Request>, Error) + 'static,
    {
        const IP: &str = "127.0.0.1";
        let addr = format!("{IP}:{port}");

        let listener = TcpListener::bind(addr)?;

        let error_handler = Box::new(error_handler);

        Ok(HttpServer {
            listener,
            router,
            error_handler,
        })
    }

    pub fn listen(&self) {
        let conn = self.connect();

        let run = |req: &mut Result<Request>| -> Result<()> {
            let req = req.as_mut().ok().ok_or(ErrorKind::NotConnected)?;
            let resp = self.router.handle(&*req)?;
            req.respond(resp)?;
            Ok(())
        };

        for mut req in conn {
            if let Err(err) = run(&mut req) {
                (self.error_handler)(req.ok(), err)
            }
        }
    }

    fn connect(&self) -> Connection {
        Connection {
            iter: self.listener.incoming(),
        }
    }
}

struct Connection<'a> {
    iter: Incoming<'a>,
}

impl Iterator for Connection<'_> {
    type Item = Result<Request>;

    fn next(&mut self) -> Option<Self::Item> {
        let stream = match self.iter.next()? {
            Ok(s) => s,
            Err(e) => return Some(Err(e)),
        };

        Some(Request::parse(stream))
    }
}
