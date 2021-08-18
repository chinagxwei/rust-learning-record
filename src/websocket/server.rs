use regex::Regex;
use sha1::Digest;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use std::convert::TryInto;

pub struct WebsocketServer {
    host: String,
    port: u16,
    runtime: Option<Runtime>,
}

enum Opcode {
    Extended = 0x0,
    Text = 0x1,
    Binary = 0x2,
    Close = 0x8,
    Ping = 0x9,
    Pong = 0xA,
}

impl WebsocketServer {
    pub fn new(host: String, port: u16) -> WebsocketServer {
        WebsocketServer {
            host,
            port,
            runtime: None,
        }
    }

    pub fn init(&mut self) -> &mut WebsocketServer {
        if self.runtime.is_none() {
            self.runtime = Runtime::new().ok();
        }

        self
    }

    pub fn start(&mut self) {
        if self.runtime.is_none() {
            return;
        }

        self.runtime.as_ref().unwrap().block_on(async {
            let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))
                .await
                .unwrap();
            println!("Server has started on 127.0.0.1:7878.\r\nWaiting for a connection...");
            loop {
                let (mut socket, _) = listener.accept().await.expect("listener accept error");
                tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    loop {
                        let n = match socket.read(&mut buf).await {
                            // socket closed
                            Ok(n) if n == 0 => return,
                            Ok(n) => {
                                let byte_data = buf.get(0..n).unwrap();
                                let str_data =
                                    String::from_utf8_lossy(byte_data).replace("\r\n", "\n");
                                println!("{}", str_data);
                                let get = Regex::new(r"^GET").unwrap();
                                if get.is_match(str_data.as_str()) {
                                    let response = connect(str_data);
                                    println!("{:?}", response);
                                    println!("{:?}", response.as_bytes());
                                    socket.write_all(response.as_bytes()).await;
                                    let welcome_msg = "{\"data\":\"welcome!\"}";
                                    let welcome_msg_head = [129 as u8, welcome_msg.len() as u8];
                                    socket.write_all(&welcome_msg_head).await;
                                    socket.write_all(welcome_msg.as_bytes()).await;
                                } else {
                                    println!("data length: {}", byte_data.len());
                                    let result = decoded(byte_data);
                                    if let Some(msg) = result {
                                        println!("result content: {}", msg);
                                        if msg.len() > 0 {
                                            let mut msg_head = [0_u8; 2];
                                            msg_head[0] = byte_data[0];
                                            msg_head[1] = msg.len() as u8;
                                            socket.write_all(&msg_head).await;
                                            socket.write_all(msg.as_bytes()).await;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                n
                            }
                            Err(e) => {
                                println!("failed to read from socket; err = {:?}", e);
                                return;
                            }
                        };
                    }
                    println!("line end;")
                });
                println!("wait connect;")
            }
        })
    }
}

fn connect(data: String) -> String {
    let sec_key_text = Regex::new(r"Sec-WebSocket-Key: (.*)").unwrap();
    let group = sec_key_text.captures(data.as_str()).unwrap();
    let sec_key = group.get(1)
        .unwrap()
        .as_str()
        .to_string() + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    let mut sha1 = sha1::Sha1::new();
    sha1.update(sec_key);
    let result = sha1.finalize();
    let sec_accept = base64::encode(result);
    return format!(
        "HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Accept: {}\r\n\r\n",
        sec_accept
    );
}

fn decoded(data: &[u8]) -> Option<String> {
    println!("{:?}", data);
    let payload = data[1] & 127;
    let (maks, decoded) = if payload <= 125 {
        (data.get(2..6), data.get(6..))
    } else {
        (None, None)
    };
    if let (Some(maks_bytes), Some(decoded_bytes)) = (maks, decoded) {
        let mut coded = Vec::with_capacity(decoded_bytes.len());
        for (i, v) in decoded_bytes.iter().enumerate() {
            coded.push(v ^ maks_bytes[i % 4])
        }
        return String::from_utf8(coded).ok();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // let mut ws = WebsocketServer::new(String::from("127.0.0.1"), 7878);
        // ws.init().start();
        let sec_key = "TOP1Of6oU4+faElIXGf1xQ==258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
        let mut sha1 = sha1::Sha1::new();
        sha1.update(sec_key);
        let result = sha1.finalize();
        let sec_accept = base64::encode(result);
        println!("{}", sec_accept);
    }
}
