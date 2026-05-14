use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Result},
};

use serde::Serialize;

fn main() -> Result<()> {
    let file = File::open("./test.log")?;

    let mut stats = Stats::default();
    let mut durations: HashMap<String, Vec<u32>> = HashMap::new();
    let mut ip_counts = HashMap::new();

    for l in BufReader::new(file).lines() {
        stats.total_requests += 1;

        let Some(log) = Log::build(&l?) else {
            stats.skipped_lines += 1;
            continue;
        };

        if log.status > 399 {
            *stats
                .errors_per_endpoint
                .entry(log.path.clone())
                .or_default() += 1;
        }

        durations.entry(log.path).or_default().push(log.duration);

        *ip_counts.entry(log.ip).or_default() += 1;
    }

    durations.into_iter().for_each(|(path, mut values)| {
        values.sort();
        let p95 = values[values.len() * 95 / 100];
        stats.p95_latency_ms.insert(path, p95);
    });

    ip_counts.iter().for_each(|(ip, count)| {
        stats.top_ips.push(IPs {
            ip: ip.clone(),
            count: *count,
        });
    });

    stats.top_ips.sort_by(|a, b| b.count.cmp(&a.count));
    stats.top_ips.truncate(10);

    let result_writer = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./results.json")?;

    serde_json::to_writer(result_writer, &stats)?;

    Ok(())
}

#[derive(Debug, Default, Serialize)]
struct IPs {
    ip: String,
    count: u32,
}

#[derive(Debug, Default, Serialize)]
struct Stats {
    total_requests: u32,
    skipped_lines: u32,
    errors_per_endpoint: HashMap<String, u32>,
    p95_latency_ms: HashMap<String, u32>,
    top_ips: Vec<IPs>,
}

#[allow(dead_code)]
struct Log {
    timestamp: String,
    level: String,
    ip: String,
    path: String,
    method: String,
    status: u32,
    duration: u32,
}

impl Log {
    fn build(line: &str) -> Option<Log> {
        let mut parts = line.split_whitespace();

        Some(Log {
            timestamp: parts.next()?.to_string(),
            level: parts.next()?.to_string(),
            ip: parts.next()?.to_string(),
            path: parts.next()?.to_string(),
            method: parts.next()?.to_string(),
            status: parts.next()?.parse().ok()?,
            duration: parts.next()?.strip_suffix("ms")?.parse().ok()?,
        })
    }
}
