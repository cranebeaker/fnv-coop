import { DurableObject } from "cloudflare:workers";
import { checkWindow, BUCKETS, type WindowState } from "./ratelimit.ts";
import { claim, lookup, nextExpiry, remove, type RoomRow, type RoomStore } from "./room.ts";
import { handle, type Deps, type RoomApi } from "./router.ts";

export interface Env {
  JOIN_ROOM: DurableObjectNamespace<JoinRoom>;
  RATE_LIMITER: DurableObjectNamespace<RateLimiter>;
}

/** One instance per join code. SQLite-backed, a single row. */
export class JoinRoom extends DurableObject<Env> {
  private store: RoomStore;

  constructor(ctx: DurableObjectState, env: Env) {
    super(ctx, env);
    const sql = ctx.storage.sql;
    sql.exec(
      "CREATE TABLE IF NOT EXISTS room (id INTEGER PRIMARY KEY CHECK (id = 1), blob TEXT NOT NULL, secret_hash TEXT NOT NULL, expires_at INTEGER NOT NULL)",
    );
    this.store = {
      load(): RoomRow | null {
        const rows = sql.exec("SELECT blob, secret_hash, expires_at FROM room WHERE id = 1").toArray();
        if (rows.length === 0) return null;
        const r = rows[0];
        return { blob: r.blob as string, secretHash: r.secret_hash as string, expiresAt: r.expires_at as number };
      },
      save(row: RoomRow): void {
        sql.exec(
          "INSERT OR REPLACE INTO room (id, blob, secret_hash, expires_at) VALUES (1, ?, ?, ?)",
          row.blob,
          row.secretHash,
          row.expiresAt,
        );
      },
      clear(): void {
        sql.exec("DELETE FROM room");
      },
    };
  }

  async claim(blob: string, secretHash: string, ttlS: number): Promise<number | null> {
    const exp = claim(this.store, Date.now(), blob, secretHash, ttlS);
    if (exp !== null) await this.ctx.storage.setAlarm(exp);
    return exp;
  }

  async lookup(): Promise<string | null> {
    return lookup(this.store, Date.now());
  }

  async remove(secretHash: string) {
    const r = remove(this.store, Date.now(), secretHash);
    if (r === "removed") await this.ctx.storage.deleteAlarm();
    return r;
  }

  /** Expiry: wipe everything so an idle code costs no storage. */
  async alarm(): Promise<void> {
    const exp = nextExpiry(this.store);
    if (exp !== null && exp > Date.now()) {
      await this.ctx.storage.setAlarm(exp);
      return;
    }
    await this.ctx.storage.deleteAll();
  }
}

/** One instance per client IP (fixed window counters). */
export class RateLimiter extends DurableObject<Env> {
  async hit(bucket: "host" | "join" | "delete"): Promise<number> {
    const key = "w:" + bucket;
    const prev = await this.ctx.storage.get<WindowState>(key);
    const b = BUCKETS[bucket];
    const now = Date.now();
    const d = checkWindow(prev, now, b.limit, b.windowMs);
    await this.ctx.storage.put(key, d.state);
    await this.ctx.storage.setAlarm(now + b.windowMs * 2);
    return d.allowed ? 0 : Math.max(1, Math.ceil(d.retryAfterMs / 1000));
  }

  async alarm(): Promise<void> {
    await this.ctx.storage.deleteAll();
  }
}

async function sha256Hex(s: string): Promise<string> {
  const d = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(s));
  return [...new Uint8Array(d)].map((x) => x.toString(16).padStart(2, "0")).join("");
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);
    const deps: Deps = {
      rateLimit: (ip, bucket) => env.RATE_LIMITER.get(env.RATE_LIMITER.idFromName(ip)).hit(bucket),
      room: (code): RoomApi => env.JOIN_ROOM.get(env.JOIN_ROOM.idFromName(code)),
      randomBytes: (n) => crypto.getRandomValues(new Uint8Array(n)),
      sha256Hex,
    };
    const res = await handle(
      {
        method: request.method,
        path: url.pathname,
        ip: request.headers.get("CF-Connecting-IP") ?? "unknown",
        headers: (n) => request.headers.get(n),
        body: () => request.text(),
      },
      deps,
    );
    // No CORS headers on purpose: browsers on other origins cannot read responses.
    return new Response(JSON.stringify(res.json), {
      status: res.status,
      headers: { "content-type": "application/json", "cache-control": "no-store", ...res.headers },
    });
  },
} satisfies ExportedHandler<Env>;
