import { createReadStream, writeFileSync } from "node:fs";
import { createInterface } from "node:readline/promises";
import { argv, exit } from "node:process";

import { Log } from "./log.mts";

const statsByPath: Record<
  string,
  {
    errors: number;
    durations: number[];
    p95: number;
  }
> = {};
const statsByIP: Record<string, number> = {};
let total_requests = 0;
let skipped_lines = 0;

async function main() {
  if (!argv[2]) {
    console.error("Usage: logcrunch.mts test.log");
    exit(1);
  }

  const rl = createInterface({ input: createReadStream(argv[2]) });
  rl.on("line", parseLine);
  rl.on("error", console.error);
  rl.on("close", parseResults);
}

function parseLine(line: string) {
  total_requests++;
  try {
    const log = new Log(line);

    if (statsByPath[log.path] === undefined) {
      statsByPath[log.path] = {
        errors: 0,
        durations: [log.duration],
        p95: 0,
      };
    } else {
      statsByPath[log.path]?.durations.push(log.duration);
    }

    if (log.status >= 400) statsByPath[log.path]!.errors++;

    if (statsByIP[log.ip] === undefined) {
      statsByIP[log.ip] = 1;
    } else {
      statsByIP[log.ip]! += 1;
    }
  } catch (err) {
    skipped_lines++;
    // if (err instanceof Error) console.error(err.message);
    // else console.error(err);
  }
}

function parseResults() {
  console.log("\nTotal/Skipped: ", total_requests + "/" + skipped_lines + "\n");

  for (const [path, { errors, durations }] of Object.entries(statsByPath)) {
    durations.sort((a, b) => a - b);
    const pIdx = Math.floor(0.95 * durations.length);
    statsByPath[path]!.p95 = durations[pIdx]!;
    console.log("Stats for path: ", path);
    console.log("  Errors: ", errors);
    console.log("  P95: ", durations[pIdx] + "ms");
  }

  const sortedIPs = Object.keys(statsByIP).sort(
    (a, b) => statsByIP[b]! - statsByIP[a]!,
  );
  sortedIPs.length = 10;
  const top_ips = sortedIPs.map((ip) => ({ ip, count: statsByIP[ip] }));
  console.log("\nTop 10 IPs by count:");
  console.log(top_ips);

  writeFileSync(
    "results.json",
    JSON.stringify({
      total_requests,
      skipped_lines,
      errors_per_endpoint: Object.entries(statsByPath).map(
        ([path, { errors }]) => ({ path, errors }),
      ),
      p95_latency_ms: Object.entries(statsByPath).map(([path, { p95 }]) => ({
        path,
        p95,
      })),
      top_ips,
    }),
  );
}

await main().catch(console.error);
