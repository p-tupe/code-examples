use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let file = fs::File::open("./test.log").expect("could not open file");

    let logs = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter_map(parse_line)
        .take(10);

    for l in logs {
        dbg!(l);
    }
}

#[derive(Debug)]
struct Log {
    timestamp: String,
    level: String,
    ip: String,
    path: String,
    method: String,
    status: String,
    duration: String,
}

fn parse_line(line: String) -> Option<Log> {
    let parts: Vec<String> = line.split_whitespace().map(str::to_string).collect();

    let [timestamp, level, ip, path, method, status, duration]: [String; 7] =
        parts.try_into().ok()?;

    Some(Log {
        timestamp,
        level,
        ip,
        path,
        method,
        status,
        duration,
    })
}
