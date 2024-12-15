use std::{
    io::{Error, ErrorKind, Result},
    net::{Incoming, TcpListener},
    sync::Arc,
};

use crate::{request::Request, router::Router, thread_pool::ThreadPool};

pub mod request;
pub mod response;

pub struct HttpServer {
    listener: TcpListener,
    router: Arc<Router>,
    error_handler: Arc<dyn Fn(Option<Request>, Error) + Send + Sync + 'static>,
}

impl HttpServer {
    pub fn new<T>(port: u32, router: Router, error_handler: T) -> Result<HttpServer>
    where
        T: Fn(Option<Request>, Error) + Send + Sync + 'static,
    {
        const IP: &str = "127.0.0.1";
        let addr = format!("{IP}:{port}");

        let listener = TcpListener::bind(addr)?;

        let router = Arc::new(router);
        let error_handler = Arc::new(error_handler);

        Ok(HttpServer {
            listener,
            router,
            error_handler,
        })
    }

    pub fn listen(&self) {
        let conn = self.connect();
        let pool = ThreadPool::new(4);

        for mut req in conn {
            let error_handler = Arc::clone(&self.error_handler);
            let router = Arc::clone(&self.router);

            let run = move |req: &mut Result<Request>| -> Result<()> {
                let req = req.as_mut().ok().ok_or(ErrorKind::NotConnected)?;
                let resp = router.handle(&*req)?;
                req.respond(resp)?;
                Ok(())
            };

            pool.execute(move || {
                if let Err(err) = run(&mut req) {
                    (error_handler)(req.ok(), err)
                }
            })
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
