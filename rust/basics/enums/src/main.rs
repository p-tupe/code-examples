#[derive(Debug)]
struct IPv4;

#[derive(Debug)]
struct IPv6;

#[derive(Debug)]
enum IP {
    V4(IPv4),
    V6(IPv6),
}

fn main() {
    let four = IP::V4(IPv4);
    let six = IP::V6(IPv6);

    println!("{:?} {:?}", four, six);

    route_ip(four);
    route_ip(six);
}

fn route_ip(ip: IP) {}
