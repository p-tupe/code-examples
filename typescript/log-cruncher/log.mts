const LEVEL = ["INFO", "WARN", "ERROR", "DEBUG"] as const;

const METHOD = ["GET", "POST", "PUT", "DELETE", "PATCH"] as const;

const IPRegex = /^\d+\.\d+\.\d+\.\d+$/;

export class Log {
  timestamp: Date; // ISO 8601 format (parseable datetime)
  level: (typeof LEVEL)[number];
  ip: string; // dotted decimal IPv4 (4 octets, 0-255 each)
  path: string; // starts with `/`
  method: (typeof METHOD)[number];
  status: number; // integer 100-599
  duration: number; // ends with `ms`

  constructor(line: string) {
    const [timestamp, level, ip, path, method, status, duration] =
      line.split(" ");

    const statusNum = Number(status);

    if (!timestamp) throw new Error("Invalid timestamp");
    if (!level || !LEVEL.includes(level as any))
      throw new Error("Invalid Level");
    if (!method || !METHOD.includes(method as any))
      throw new Error("Invalid Method");
    if (!ip || !IPRegex.test(ip)) throw new Error("Invalid IP");
    if (!path || !path.startsWith("/")) throw new Error("Invalid Path");
    if (!status || statusNum < 100 || statusNum > 599)
      throw new Error("Invalid Status");
    if (!duration || !duration.endsWith("ms"))
      throw new Error("Invalid Duration");

    this.timestamp = new Date(timestamp);
    this.level = level as (typeof LEVEL)[number];
    this.ip = ip;
    this.path = path;
    this.method = method as (typeof METHOD)[number];
    this.status = statusNum;
    this.duration = parseFloat(duration);

    return Object.freeze(this);
  }
}
