// Request handling as a pure function over injected dependencies, so it is unit-testable without Workers.
import { generateCode, normalizeCode } from "./code.ts";
import type { RemoveResult } from "./room.ts";
import { parseHostBody } from "./validate.ts";

export interface Req {
  method: string;
  path: string;
  ip: string;
  headers: (name: string) => string | null;
  body: () => Promise<string>;
}

export interface Res {
  status: number;
  json: unknown;
  headers?: Record<string, string>;
}

export interface RoomApi {
  claim(blob: string, secretHash: string, ttlS: number): Promise<number | null>;
  lookup(): Promise<string | null>;
  remove(secretHash: string): Promise<RemoveResult>;
}

export interface Deps {
  /** Returns retry-after seconds when limited, else 0. */
  rateLimit(ip: string, bucket: "host" | "join" | "delete"): Promise<number>;
  room(code: string): RoomApi;
  randomBytes(n: number): Uint8Array;
  sha256Hex(s: string): Promise<string>;
}

const err = (status: number, message: string, headers?: Record<string, string>): Res => ({
  status,
  json: { error: message },
  headers,
});

function hex(b: Uint8Array): string {
  let s = "";
  for (const x of b) s += x.toString(16).padStart(2, "0");
  return s;
}

export async function handle(req: Req, d: Deps): Promise<Res> {
  const parts = req.path.split("/").filter((p) => p.length > 0);
  const m = req.method.toUpperCase();

  let bucket: "host" | "join" | "delete" | null = null;
  if (m === "POST" && parts.length === 1 && parts[0] === "host") bucket = "host";
  else if (m === "GET" && parts.length === 2 && parts[0] === "join") bucket = "join";
  else if (m === "DELETE" && parts.length === 2 && parts[0] === "host") bucket = "delete";
  if (!bucket) return err(404, "not found");

  const retry = await d.rateLimit(req.ip, bucket);
  if (retry > 0) return err(429, "rate limited", { "Retry-After": String(retry) });

  if (bucket === "host") {
    const parsed = parseHostBody(await req.body());
    if (!parsed.ok) return err(400, parsed.error);
    const secret = hex(d.randomBytes(24));
    const secretHash = await d.sha256Hex(secret);
    for (let attempt = 0; attempt < 6; attempt++) {
      const code = generateCode(d.randomBytes);
      const exp = await d.room(code).claim(parsed.value.blob, secretHash, parsed.value.ttlS);
      if (exp !== null) {
        return { status: 201, json: { code, secret, expires_at: exp, ttl_s: parsed.value.ttlS } };
      }
    }
    return err(503, "no free code, retry");
  }

  const code = normalizeCode(parts[1]);
  if (!code) return err(400, "bad code");

  if (bucket === "join") {
    const blob = await d.room(code).lookup();
    if (blob === null) return err(404, "unknown or expired code");
    return { status: 200, json: { blob } };
  }

  const secret = req.headers("x-host-secret");
  if (!secret) return err(401, "missing X-Host-Secret");
  const r = await d.room(code).remove(await d.sha256Hex(secret));
  if (r === "missing") return err(404, "unknown or expired code");
  if (r === "forbidden") return err(403, "wrong secret");
  return { status: 200, json: { ok: true } };
}
