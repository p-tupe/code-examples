use std::{
    collections::HashMap,
    process::exit,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

type DB = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:6399").await?;

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();
        tokio::spawn(async move {
            if let Err(e) = process(socket, db).await {
                eprintln!("{e:?}");
                exit(1);
            };
        });
    }
}

async fn process(socket: TcpStream, db: DB) -> Result<()> {
    use mini_redis::Command::{self, Get, Set};

    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await? {
        let resp = match Command::from_frame(frame)? {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().into(), cmd.value().clone());
                Frame::Simple("OK".into())
            }

            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(val) = db.get(cmd.key()) {
                    Frame::Bulk(val.clone().into())
                } else {
                    Frame::Null
                }
            }

            cmd => panic!("unimplemented! {cmd:?}"),
        };

        conn.write_frame(&resp).await?;
    }

    Ok(())
}
