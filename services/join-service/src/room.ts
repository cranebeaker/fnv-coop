// Pure room logic over an abstract store. The JoinRoom Durable Object supplies a SQLite-backed store.
export interface RoomRow {
  blob: string;
  secretHash: string;
  expiresAt: number;
}

export interface RoomStore {
  load(): RoomRow | null;
  save(row: RoomRow): void;
  clear(): void;
}

export const MIN_TTL_S = 60;
export const MAX_TTL_S = 6 * 3600;
export const DEFAULT_TTL_S = 1800;
export const MAX_BLOB_LEN = 512;

/** Returns expiresAt or null when the code is taken by a live room. */
export function claim(
  store: RoomStore,
  now: number,
  blob: string,
  secretHash: string,
  ttlS: number,
): number | null {
  const cur = store.load();
  if (cur && cur.expiresAt > now) return null;
  const expiresAt = now + ttlS * 1000;
  store.save({ blob, secretHash, expiresAt });
  return expiresAt;
}

export function lookup(store: RoomStore, now: number): string | null {
  const cur = store.load();
  if (!cur) return null;
  if (cur.expiresAt <= now) {
    store.clear();
    return null;
  }
  return cur.blob;
}

export type RemoveResult = "removed" | "missing" | "forbidden";

export function remove(store: RoomStore, now: number, secretHash: string): RemoveResult {
  const cur = store.load();
  if (!cur || cur.expiresAt <= now) {
    if (cur) store.clear();
    return "missing";
  }
  if (!timingSafeEqual(cur.secretHash, secretHash)) return "forbidden";
  store.clear();
  return "removed";
}

/** Next alarm time, or null if nothing is stored. */
export function nextExpiry(store: RoomStore): number | null {
  return store.load()?.expiresAt ?? null;
}

export function timingSafeEqual(a: string, b: string): boolean {
  if (a.length !== b.length) return false;
  let d = 0;
  for (let i = 0; i < a.length; i++) d |= a.charCodeAt(i) ^ b.charCodeAt(i);
  return d === 0;
}
