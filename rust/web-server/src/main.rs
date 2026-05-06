use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind(":7878").expect("TCP listener failed to bind!");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Stream connected!")
    }
}
