mod client;
pub mod server;

use serde::{Deserialize, Serialize};
use std::any::Any;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

fn decode<'a, T: Deserialize<'a>>(data: &'a [u8]) -> T {
    serde_json::from_slice(data).unwrap()
}

async fn encode_and_send<T: Serialize>(stream: &mut TcpStream, data: T) {
    let send_data = serde_json::to_string(&data).unwrap();
    stream.write(send_data.as_bytes()).await;
    stream.flush();
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    type_name: String,
    data: String,
}

impl Request {
    fn new(type_name: String, data: String) -> Request {
        Request { type_name, data }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    data: String
}

impl Response {
    fn new(data: String) -> Response {
        Response { data }
    }
}

pub trait Handler: Any {}

impl<T: Any> Handler for T {}

trait HelloService {
    fn say_hello(&self, content: String) -> String;
    fn send_hello(&self, content: String) -> String;
}

