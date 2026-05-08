use std::{
    collections::HashMap,
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

use serde::Serialize;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./test.log")?;

    let mut stats = Stats::default();
    let mut durations = HashMap::new();
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
                .or_insert(1) += 1;
        }

        durations
            .entry(log.path.clone())
            .or_insert(Vec::new())
            .push(log.duration);

        *ip_counts.entry(log.ip).or_insert(0) += 1;
    }

    durations.iter_mut().for_each(|(path, values)| {
        values.sort();
        let p95 = values[values.len() * 95 / 100];
        stats.p95_latency_ms.insert(path.clone(), p95);
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
        // TODO: Use .next()
        let mut parts: Vec<_> = line.split_whitespace().map(String::from).collect();

        if parts.len() < 7 {
            return None;
        }

        let duration = parts.pop()?.strip_suffix("ms")?.parse().ok()?;
        let status = parts.pop()?.parse().ok()?;
        let method = parts.pop()?;
        let path = parts.pop()?;
        // TODO: Validate

        Some(Log {
            timestamp: parts.remove(0),
            level: parts.remove(0),
            ip: parts.remove(0),
            path,
            method,
            status,
            duration,
        })
    }
}
