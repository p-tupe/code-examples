use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize)]
pub struct IPs {
    pub ip: String,
    pub count: u32,
}

#[derive(Debug, Default, Serialize)]
pub struct Stats {
    pub total_requests: u32,
    pub skipped_lines: u32,
    pub errors_per_endpoint: HashMap<String, u32>,
    pub p95_latency_ms: HashMap<String, u32>,
    pub top_ips: Vec<IPs>,
}

#[allow(dead_code)]
pub struct Log<'a> {
    pub timestamp: &'a str,
    pub level: &'a str,
    pub ip: &'a str,
    pub path: &'a str,
    pub method: &'a str,
    pub status: u32,
    pub duration: u32,
}

impl<'a> Log<'a> {
    pub fn build(line: &'a str) -> Option<Log<'a>> {
        let mut parts = line.split_whitespace();

        Some(Log {
            timestamp: parts.next()?,
            level: parts.next()?,
            ip: parts.next()?,
            path: parts.next()?,
            method: parts.next()?,
            status: parts.next()?.parse().ok()?,
            duration: parts.next()?.strip_suffix("ms")?.parse().ok()?,
        })
    }
}
