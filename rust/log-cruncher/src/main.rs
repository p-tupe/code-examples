use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Result},
};

use crate::schema::{IPs, Log, Stats};

mod schema;

fn main() -> Result<()> {
    let mut stats = Stats::default();
    let mut durations: HashMap<String, Vec<u32>> = HashMap::new();
    let mut ip_counts = HashMap::new();

    let file = File::open("./test.log")?;
    let mut line = String::new();
    let mut reader = BufReader::new(file);
    while reader.read_line(&mut line)? > 0 {
        stats.total_requests += 1;

        let Some(log) = Log::build(&line) else {
            line.clear();
            stats.skipped_lines += 1;
            continue;
        };

        if log.status > 399 {
            *stats
                .errors_per_endpoint
                .entry(log.path.to_string())
                .or_default() += 1;
        }

        if let Some(v) = durations.get_mut(log.path) {
            v.push(log.duration);
        } else {
            durations.insert(log.path.to_string(), vec![log.duration]);
        };

        if let Some(v) = ip_counts.get_mut(log.ip) {
            *v += 1;
        } else {
            ip_counts.insert(log.ip.to_string(), 1);
        };

        *ip_counts.entry(log.ip.to_string()).or_default() += 1;
        line.clear();
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

    stats.top_ips.sort_by_key(|a| std::cmp::Reverse(a.count));
    stats.top_ips.truncate(10);

    let result_writer = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./results.json")?;

    serde_json::to_writer(result_writer, &stats)?;

    Ok(())
}
