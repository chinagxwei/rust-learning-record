use std::collections::HashMap;
use std::any::Any;
use tokio::runtime::Runtime;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::{Mutex, mpsc};
use std::sync::Arc;
use std::error::Error;
use std::borrow::{BorrowMut, Borrow};
use crate::rcp::{HelloService, Request, Response};

struct RpcServer {
    handles: Arc<HashMap<&'static str, Box<dyn Fn(Request) -> String + Send + Sync + 'static>>>
}

impl RpcServer {
    fn add_service<T: Fn(Request) -> String + Send + Sync + 'static>(&mut self, type_name: &'static str, service: T) -> &mut RpcServer {
        Arc::get_mut(self.handles.borrow_mut())
            .unwrap()
            .insert(type_name, Box::new(service));
        self
    }

    fn start(&self, port: u32) -> Result<(), Box<dyn std::error::Error>> {
        println!("server started @ 127.0.0.1:{}", port);
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
            loop {
                let b = self.handles.clone();
                let (mut socket, _) = listener.accept().await.unwrap();
                tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };
                    let request: Request = serde_json::from_slice(&buf[0..n]).unwrap();
                    let f = b.get(request.type_name.as_str()).map(|x| (*x)(request)).unwrap();
                    let res = Response::new(f);
                    let send_data = serde_json::to_string(&res).unwrap();
                    socket.write_all(send_data.as_bytes()).await;
                    socket.flush();
                });
            }
        });

        Ok(())
    }
}

struct HelloServiceImpl;

impl HelloService for HelloServiceImpl {
    fn say_hello(&self, content: String) -> String {
        println!("request is coming: {}", content);
        format!("say hello {}", content)
    }

    fn send_hello(&self, content: String) -> String {
        println!("request is coming: {}", content);
        format!("send hello {}", content)
    }
}

pub fn start(port: u32) -> Result<(), Box<dyn Error>> {
    let mut rpc_server = RpcServer { handles: Default::default() };
    let hello = HelloServiceImpl {};
    let arc_hello = Arc::new(hello);
    let (a,b) = (Arc::clone(&arc_hello),Arc::clone(&arc_hello));
    rpc_server.add_service("say_hello",  move|r| {
        a.say_hello(r.data)
    }).add_service("send_hello",  move|r| {
        b.send_hello(r.data)
    })
        .start(port)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        start(8080);
    }
}
