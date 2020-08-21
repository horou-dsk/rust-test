use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame, Command};
use std::collections::HashMap;
use mini_redis::cmd::Command::{Set, Get};
use std::sync::{Mutex, Arc};
use bytes::Bytes;
use std::hash;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let mut vs = vec![Mutex::new(HashMap::new())];

    for i in 0..32 {
        // vs.push(Mutex::new(HashMap::new()));
    }

    let db = Arc::new(vs);

    loop {
        // The second item contains the ip and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        println!("Accepted");
        process(socket, db).await;
        println!("是否阻塞");
        // tokio::spawn(async move {
        //
        // });
    }
}

async fn process(socket: TcpStream, db: ShardedDb) {
    // A hashmap is used to store data
    // let mut db = HashMap::new();

    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut s = DefaultHasher::new();
                let key = cmd.key().to_string();
                key.hash(&mut s);
                let mut db = db[s.finish() as usize % db.len()].lock().unwrap();
                db.insert(key, cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let mut s = DefaultHasher::new();
                let key = cmd.key().to_string();
                key.hash(&mut s);
                let mut db = db[s.finish() as usize % db.len()].lock().unwrap();
                if let Some(value) = db.get(&key) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}