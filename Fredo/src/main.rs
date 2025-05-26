mod server;
use server::HttpRequests;

fn main() {
    println!("Hello, world!");

    const url: &str = "http://localhost:9090";

    let server = server::Server{url:url};

    server.get_request();
}
