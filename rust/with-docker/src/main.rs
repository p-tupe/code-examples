//! This code demonstrates a non-trivial rust program (requires tls/tz)
//! that can be run inside docker container.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "your ip is: {}",
        reqwest::blocking::Client::builder()
            .user_agent("curl")
            .build()?
            .get("https://ifconfig.me/")
            .send()?
            .error_for_status()?
            .text()
            .unwrap_or_default()
    );

    Ok(())
}
