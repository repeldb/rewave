extern crate rewave;

use std::net::SocketAddr;
use async_trait::async_trait;
use tokio;
use rewave::wave::*;
use rewave::data::*;

struct RepelServer {}

#[async_trait]
impl Rewave for RepelServer {
    async fn request(&self, request: Request) -> Result<Response, ResponseError> {
        println!("New request : {:?}", request);
        Ok(Response { body: Some(Vec::new()), header: ResponseHeader { status: Status::Ok, message: Some("hello world".to_string()), error: false } }) 
    }
    async fn on_connect(&self, username: Option<&str>, addr: SocketAddr) {
       println!("New connection: {}:{}", addr.ip(), addr.port()) 
    }
}

#[tokio::main]
async fn main() {
    let repel = RepelServer {};
    let rwh = RewaveHandler::new(repel);
    let server = RewaveServer::new(rwh, true);
    server.serve("127.0.0.1:2000").await;
}
