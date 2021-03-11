use crate::rpc::{HelloService, Request, Response, encode_and_send, decode};
use std::net::{IpAddr, Ipv4Addr};
use std::fmt::Debug;
use serde::Serialize;
use tokio::runtime::Runtime;
use tokio::net::TcpSocket;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

struct Transport {
    host: Ipv4Addr,
    port: u32,
    runtime: Runtime,
}

impl Transport {
    fn new(host: Ipv4Addr, port: u32) -> Transport {
        let runtime = Runtime::new().unwrap();
        Transport { host, port, runtime }
    }
}

impl Transport {
    fn send<T: Serialize>(&self, data: (&'static str, Vec<T>)) -> String {
        let (method_type, send_data) = data;

        let request = Request::new(method_type.into(), serde_json::to_string(&send_data).unwrap());

        println!("发送数据");

        let addr = format!("{}:{}", self.host, self.port).parse().unwrap();

        let output = self.runtime.block_on(async {
            let (mut buf, socket) = ([0; 1024], TcpSocket::new_v4().unwrap());

            let mut stream = socket.connect(addr).await.unwrap();

            encode_and_send(&mut stream, request).await;

            let len = stream.read(&mut buf).await.unwrap();

            let res = decode::<Response>(&buf[0..len]);

            println!("{:?}", res.data);

            res.data
        });
        output
    }
}

struct HelloServiceProxy {
    transport: Transport
}

impl HelloServiceProxy {
    pub fn new(host: Ipv4Addr, port: u32) -> Self {
        HelloServiceProxy { transport: Transport::new(host, port) }
    }
}

///
/// 优化方向，代理实现由过程宏完成
///
impl HelloService for HelloServiceProxy {
    fn say_hello(&self, content: String) -> String {
        self.transport.send(("say_hello", vec![content]))
    }

    fn send_hello(&self, author: String, content: String) -> String {
        self.transport.send(("send_hello", vec![author, content]))
    }
}

pub fn client_send() {
    let service = HelloServiceProxy::new("127.0.0.1".parse().unwrap(), 7878);
    service.say_hello("rpc simple demo".into());
    service.send_hello("Tom".into(), "rpc simple demo".into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        client_send()
    }
}




