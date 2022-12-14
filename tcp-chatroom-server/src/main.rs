#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use common::Message;
use tokio::io::AsyncBufReadExt;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    // Use this queue to push message for broadcasting
    let mq = Arc::new(Mutex::new(VecDeque::<Arc<String>>::new()));
    // Broadcast messages to clients
    let (sx, rx) = tokio::sync::broadcast::channel::<Arc<String>>(1024);

    tokio::spawn({
        let mq = mq.clone();
        async move {
            loop {
                let mut mq = mq.lock().await;
                let Some(message) =  mq.pop_front() else {
                    continue
                };
                let _ = sx.send(message);
            }
        }
    });

    //  Waiting for connecting
    loop {
        let (mut reader, mut sender) = {
            let Ok((socket, _)) = listener.accept().await else {
                continue
            };
            socket.into_split()
        };

        // Parse headers
        let headers = async {
            let mut hashmap = HashMap::<String, String>::new();

            let mut buffer: Vec<u8> = Vec::new();
            loop {
                let Ok(one_byte) = reader.read_u8().await else {
                    break None
                };

                match one_byte {
                    b'\n' => {
                        let mut kv: Vec<String> = buffer
                            .split(|&byte| byte == b':')
                            .filter_map(|bytes| String::from_utf8(bytes.to_vec()).ok())
                            .collect();

                        // safety: pop two elements when vector just has two elements
                        if kv.len() == 2 {
                            unsafe {
                                let v = kv.pop().unwrap_unchecked();
                                let k = kv.pop().unwrap_unchecked();
                                hashmap.insert(k, v);
                            }
                        }
                        // goto next line
                        buffer.clear();
                        continue;
                    }
                    // zero ends the headers
                    b'\0' => break Some(hashmap),
                    _____ => buffer.push(one_byte),
                }
            }
        }
        .await;
        let Some(headers) = headers else {
            continue
        };

        // Use the GoToGroup token to pass.
        match headers.get("Hello") {
            Some(data) if data.trim() == "GoToGroup" => {}
            _ => continue,
        }

        // Reader coroutine
        tokio::spawn({
            let mq = mq.clone();
            async move {
                let mut lines = BufReader::new(reader).lines();

                while let Ok(Some(line)) = lines.next_line().await {
                    if serde_json::from_str::<Message>(line.as_str()).is_ok() {
                        println!("{}", line);
                        mq.lock().await.push_back(Arc::new(line));
                    }
                }
            }
        });
        // Sender coroutine
        tokio::spawn({
            let mut receiver = rx.resubscribe();
            async move {
                loop {
                    let Ok(msg) = receiver.recv().await else {
                        continue
                    };
                    let _ = sender.write(msg.as_bytes()).await;
                    let _ = sender.write("\n".as_bytes()).await;
                }
            }
        });
    }
}
