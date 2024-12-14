use hello::{
    request::Request, response::Response, router::Router, server::HttpServer, status_code,
};

fn main() {
    let mut router = Router::new();
    router.get("/", get_hello);

    let server = HttpServer::new(7878, router, |req, err| {
        let response = Response::return_file(status_code::NOT_FOUND, "404.html").unwrap();
        if let Some(mut req) = req {
            req.respond(response).unwrap();
            eprintln!("Error: {err}")
        }
    })
    .unwrap();

    server.listen();
}

fn get_hello(_req: &Request) -> Response {
    Response::return_file(status_code::OK, "hello.html").unwrap()
}
