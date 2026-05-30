use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("localhost:6399").await?;

    client.set("hello", "world".into()).await?;
    let r = client.get("hello").await?;
    print!("{:?}", r);

    Ok(())
}
