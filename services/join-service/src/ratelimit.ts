// Fixed-window counter, pure. State is persisted by the RateLimiter Durable Object (one per IP).
export interface WindowState {
  start: number;
  count: number;
}

export interface Decision {
  allowed: boolean;
  state: WindowState;
  retryAfterMs: number;
}

export function checkWindow(
  prev: WindowState | undefined,
  now: number,
  limit: number,
  windowMs: number,
): Decision {
  let state: WindowState =
    prev && now >= prev.start && now - prev.start < windowMs ? { ...prev } : { start: now, count: 0 };
  if (state.count >= limit) {
    return { allowed: false, state, retryAfterMs: Math.max(0, state.start + windowMs - now) };
  }
  state = { start: state.start, count: state.count + 1 };
  return { allowed: true, state, retryAfterMs: 0 };
}

export interface Bucket {
  limit: number;
  windowMs: number;
}

/** Per IP limits. Joins are tighter on failures via the same bucket: guessing is what we defend against. */
export const BUCKETS: Record<"host" | "join" | "delete", Bucket> = {
  host: { limit: 10, windowMs: 60_000 },
  join: { limit: 30, windowMs: 60_000 },
  delete: { limit: 20, windowMs: 60_000 },
};
