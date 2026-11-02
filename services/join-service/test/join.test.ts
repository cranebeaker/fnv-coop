import { test } from "node:test";
import assert from "node:assert/strict";
import { createHash, randomBytes } from "node:crypto";
import { ALPHABET, generateCode, normalizeCode } from "../src/code.ts";
import { checkWindow } from "../src/ratelimit.ts";
import { claim, lookup, remove, nextExpiry, type RoomRow, type RoomStore } from "../src/room.ts";
import { parseHostBody } from "../src/validate.ts";
import { handle, type Deps, type Req, type RoomApi } from "../src/router.ts";

function memStore(): RoomStore & { row: RoomRow | null } {
  const s = {
    row: null as RoomRow | null,
    load() { return s.row; },
    save(r: RoomRow) { s.row = r; },
    clear() { s.row = null; },
  };
  return s;
}

test("code generation uses the alphabet, is unambiguous, and is uniform-ish", () => {
  for (const bad of "0O1ILUS5") assert.ok(!ALPHABET.includes(bad), bad);
  assert.equal(new Set(ALPHABET).size, ALPHABET.length);
  const counts = new Map<string, number>();
  for (let i = 0; i < 20000; i++) {
    const c = generateCode((n) => randomBytes(n));
    assert.equal(c.length, 6);
    for (const ch of c) counts.set(ch, (counts.get(ch) ?? 0) + 1);
  }
  const expected = (20000 * 6) / ALPHABET.length;
  for (const ch of ALPHABET) assert.ok(Math.abs((counts.get(ch) ?? 0) - expected) < expected * 0.15, ch);
});

test("rejection sampling skips biased bytes", () => {
  // 252 = 256 - 256 % 28 -> bytes >= 252 must be skipped.
  let calls = 0;
  const code = generateCode((n) => {
    calls++;
    return calls === 1 ? new Uint8Array(n).fill(255) : new Uint8Array(n).fill(0);
  });
  assert.equal(code, ALPHABET[0].repeat(6));
});

test("normalizeCode", () => {
  assert.equal(normalizeCode(" ab-cd ef "), "ABCDEF");
  assert.equal(normalizeCode("ABCDE"), null);
  assert.equal(normalizeCode("ABCDE0"), null);
});

test("rate limit window", () => {
  let st = undefined;
  for (let i = 0; i < 3; i++) {
    const d = checkWindow(st, 1000 + i, 3, 60000);
    assert.ok(d.allowed);
    st = d.state;
  }
  const blocked = checkWindow(st, 5000, 3, 60000);
  assert.ok(!blocked.allowed);
  assert.equal(blocked.retryAfterMs, 56000);
  assert.ok(checkWindow(blocked.state, 61001, 3, 60000).allowed);
});

test("room ttl, claim, remove", () => {
  const s = memStore();
  assert.equal(claim(s, 0, "b", "h", 60), 60000);
  assert.equal(claim(s, 1000, "x", "h2", 60), null); // live room keeps its code
  assert.equal(lookup(s, 59999), "b");
  assert.equal(nextExpiry(s), 60000);
  assert.equal(lookup(s, 60000), null); // expired and cleared
  assert.equal(s.row, null);
  assert.equal(claim(s, 70000, "c", "h3", 60), 130000); // reusable after expiry
  assert.equal(remove(s, 70001, "nope"), "forbidden");
  assert.equal(remove(s, 70001, "h3"), "removed");
  assert.equal(remove(s, 70001, "h3"), "missing");
});

test("body validation", () => {
  assert.deepEqual(parseHostBody('{"blob":"steam:123:tok"}'), { ok: true, value: { blob: "steam:123:tok", ttlS: 1800 } });
  assert.equal(parseHostBody('{"blob":"a","ttl_s":5}').ok && (parseHostBody('{"blob":"a","ttl_s":5}') as any).value.ttlS, 60);
  assert.equal((parseHostBody('{"blob":"a","ttl_s":999999}') as any).value.ttlS, 21600);
  for (const bad of ["", "nope", "[]", '{"blob":""}', '{"blob":5}', '{"blob":"a\\u0000b"}', '{"blob":"a","ttl_s":"x"}', JSON.stringify({ blob: "x".repeat(513) })]) {
    assert.equal(parseHostBody(bad).ok, false, bad);
  }
});

// ---- router with in-memory deps ----
function makeDeps(limit = 1000) {
  const rooms = new Map<string, RoomStore & { row: RoomRow | null }>();
  const hits = new Map<string, number>();
  let now = 1_000_000;
  const sha = async (s: string) => createHash("sha256").update(s).digest("hex");
  const deps: Deps = {
    async rateLimit(ip, bucket) {
      const k = ip + bucket;
      const n = (hits.get(k) ?? 0) + 1;
      hits.set(k, n);
      return n > limit ? 30 : 0;
    },
    room(code): RoomApi {
      if (!rooms.has(code)) rooms.set(code, memStore());
      const s = rooms.get(code)!;
      return {
        async claim(blob, h, ttl) { return claim(s, now, blob, h, ttl); },
        async lookup() { return lookup(s, now); },
        async remove(h) { return remove(s, now, h); },
      };
    },
    randomBytes: (n) => randomBytes(n),
    sha256Hex: sha,
  };
  return { deps, advance: (ms: number) => { now += ms; } };
}

function req(method: string, path: string, body = "", headers: Record<string, string> = {}, ip = "1.1.1.1"): Req {
  return { method, path, ip, headers: (n) => headers[n.toLowerCase()] ?? null, body: async () => body };
}

test("host, join, delete flow with secret and expiry", async () => {
  const { deps, advance } = makeDeps();
  const h = await handle(req("POST", "/host", '{"blob":"1.2.3.4:5000|tok","ttl_s":120}'), deps);
  assert.equal(h.status, 201);
  const { code, secret } = h.json as { code: string; secret: string };
  assert.equal(code.length, 6);

  const j = await handle(req("GET", "/join/" + code.toLowerCase()), deps);
  assert.deepEqual([j.status, j.json], [200, { blob: "1.2.3.4:5000|tok" }]);

  assert.equal((await handle(req("DELETE", "/host/" + code), deps)).status, 401);
  assert.equal((await handle(req("DELETE", "/host/" + code, "", { "x-host-secret": "bad" }), deps)).status, 403);
  advance(121_000);
  assert.equal((await handle(req("GET", "/join/" + code), deps)).status, 404);

  const h2 = await handle(req("POST", "/host", '{"blob":"b2"}'), deps);
  const c2 = h2.json as { code: string; secret: string };
  assert.equal((await handle(req("DELETE", "/host/" + c2.code, "", { "x-host-secret": c2.secret }), deps)).status, 200);
  assert.equal((await handle(req("GET", "/join/" + c2.code), deps)).status, 404);
  assert.ok(secret.length >= 32);
});

test("router errors and rate limiting", async () => {
  const { deps } = makeDeps(2);
  assert.equal((await handle(req("GET", "/join/ZZZ"), deps)).status, 400);
  assert.equal((await handle(req("GET", "/join/ABCDEF"), deps)).status, 404);
  const limited = await handle(req("GET", "/join/ABCDEF"), deps);
  assert.equal(limited.status, 429);
  assert.equal(limited.headers?.["Retry-After"], "30");
  // other IPs unaffected
  assert.equal((await handle(req("GET", "/join/ABCDEF", "", {}, "2.2.2.2"), deps)).status, 404);
  assert.equal((await handle(req("POST", "/host", "garbage"), deps)).status, 400);
  assert.equal((await handle(req("OPTIONS", "/host"), deps)).status, 404);
  assert.equal((await handle(req("GET", "/"), deps)).status, 404);
});

test("code collision retries on a live room", async () => {
  const { deps } = makeDeps();
  // Force the first generated code to collide: pre-claim it.
  const fixed = new Uint8Array(16).fill(0);
  let n = 0;
  const first = ALPHABET[0].repeat(6);
  await deps.room(first).claim("old", "h", 600);
  deps.randomBytes = (len) => (n++ === 1 ? fixed : n === 2 ? new Uint8Array(len).fill(1) : randomBytes(len));
  // call 0 is the secret; call 1 gives the colliding code, call 2 a different one
  const res = await handle(req("POST", "/host", '{"blob":"new"}'), deps);
  assert.equal(res.status, 201);
  assert.notEqual((res.json as { code: string }).code, first);
});
