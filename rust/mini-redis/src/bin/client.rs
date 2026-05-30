use bytes::Bytes;
use mini_redis::Result;
use tokio::sync::{mpsc, oneshot};

enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        use mini_redis::client;

        let mut client = client::connect("localhost:6799").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp } => {
                    let val = client.get(&key).await;
                    let _ = resp.send(val);
                }

                Set { key, val, resp } => {
                    let result = client.set(&key, val).await;
                    let _ = resp.send(result);
                }
            };
        }
    });

    let t1 = tokio::spawn(async move {
        println!("Get foo");

        let (resp_tx, resp_rx) = oneshot::channel();

        tx.send(Command::Get {
            key: "foo".into(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let resp = resp_rx.await;
        println!("Get Resp: {:?}", resp);
    });

    let t2 = tokio::spawn(async move {
        println!("Set foo");

        let (resp_tx, resp_rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: "foo".into(),
            val: "bar".into(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let resp = resp_rx.await;
        println!("Set Resp: {:?}", resp);
    });

    t1.await?;
    t2.await?;
    manager.await?;

    Ok(())
}
