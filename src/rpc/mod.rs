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

trait Data {
    fn data(&self) -> &str;
    fn set_data<T>(&mut self, data: T) where T: Serialize;
    fn get_data<'a, T: Deserialize<'a>>(&'a self) -> T {
        let bytes = self.data().as_bytes();
        serde_json::from_slice(bytes).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    type_name: String,
    data: String,
}

impl Data for Request {
    fn data(&self) -> &str {
        &self.data
    }

    fn set_data<T>(&mut self, data: T) where T: Serialize {
        self.data = serde_json::to_string(&data).unwrap();
    }
}

impl Request {
    fn new<T>(type_name: String, data: T) -> Request
        where
            T: Serialize
    {
        Request { type_name, data: serde_json::to_string(&data).unwrap() }
    }

    fn set_name(&mut self, name: String) {
        self.type_name = name;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    data: String
}

impl Data for Response {
    fn data(&self) -> &str {
        &self.data
    }

    fn set_data<T>(&mut self, data: T) where T: Serialize {
        self.data = serde_json::to_string(&data).unwrap();
    }
}

impl Response {
    fn new<T>(data: T) -> Response
        where
            T: Serialize
    {
        Response { data: serde_json::to_string(&data).unwrap() }
    }
}

pub trait Handler: Any {}

impl<T: Any> Handler for T {}

trait HelloService {
    fn say_hello(&self, content: String) -> String;
    fn send_hello(&self, author: String, content: String) -> String;
}

