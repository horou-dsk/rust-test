mod connection;

extern crate rand;

use mini_redis::{client, Result};
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use std::time::{SystemTime, Instant};
use rand::{Rng};
use std::thread;
use tokio::time::Duration;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Vec<u8>,
        resp: Responder<()>,
    }
}

/// Provided by the requester and used by the manager task to send the command
/// response back to the requester.
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // Create a new channel with a capacity of at most 32.
    let (mut tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        println!("哈哈哈，我的天");

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp } => {
                    resp.send(client.get(&key).await);
                }
                Set { key, val, resp } => {
                    resp.send(client.set(&key, val.into()).await);
                }
            }
        }
    });

    for i in 0..10 {

        let mut tx2 = tx.clone();
        // Spawn two tasks, one gets a key, the other sets a key

        let t2 = tokio::spawn(async move {

            loop {
                let (resp_tx, resp_rx) = oneshot::channel();

                let cmd = Command::Set {
                    key: vec!["foo", "sdfder", "xcfgdg", "sdf1212", "dj128384"][rand::thread_rng().gen_range(0, 5)].to_string(),
                    val: b"bar".to_vec(),
                    resp: resp_tx,
                };

                // Send the SET request
                let now = Instant::now();
                tx2.send(cmd).await.unwrap();

                // Await the response
                let res = resp_rx.await;
                println!("{}", now.elapsed().as_millis());
                println!("Set GOT = {:?}", res);

                thread::sleep(Duration::from_millis(100));
            }

        });

    }

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        tx.send(cmd).await.unwrap();

        // Await the response
        let res = resp_rx.await;
        println!("Get GOT = {:?}", res)
    });
    t1.await.unwrap();

    manager.await.unwrap();





    /*while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }*/
}