import { DEFAULT_TTL_S, MAX_BLOB_LEN, MAX_TTL_S, MIN_TTL_S } from "./room.ts";

export interface HostRequest {
  blob: string;
  ttlS: number;
}

export type Parsed = { ok: true; value: HostRequest } | { ok: false; error: string };

/** Body: {"blob": string, "ttl_s"?: number}. The blob is opaque but must be printable text. */
export function parseHostBody(raw: string): Parsed {
  if (raw.length > 2048) return { ok: false, error: "body too large" };
  let j: unknown;
  try {
    j = JSON.parse(raw);
  } catch {
    return { ok: false, error: "invalid json" };
  }
  if (typeof j !== "object" || j === null) return { ok: false, error: "invalid body" };
  const o = j as Record<string, unknown>;
  const blob = o.blob;
  if (typeof blob !== "string" || blob.length === 0 || blob.length > MAX_BLOB_LEN) {
    return { ok: false, error: "blob must be a string of 1.." + MAX_BLOB_LEN + " chars" };
  }
  // Reject control characters.
  for (let i = 0; i < blob.length; i++) {
    const c = blob.charCodeAt(i);
    if (c < 0x20 || c === 0x7f) return { ok: false, error: "blob has control characters" };
  }
  let ttlS = DEFAULT_TTL_S;
  if (o.ttl_s !== undefined) {
    if (typeof o.ttl_s !== "number" || !Number.isFinite(o.ttl_s)) return { ok: false, error: "bad ttl_s" };
    ttlS = Math.min(MAX_TTL_S, Math.max(MIN_TTL_S, Math.floor(o.ttl_s)));
  }
  return { ok: true, value: { blob, ttlS } };
}
