# join-service

Cloudflare Worker + SQLite-backed Durable Objects that map a short join code to a host's opaque
connection blob (lobby id, or `ip:port` plus a token). The service never interprets the blob and
never logs it.

- Code: 6 chars from `ABCDEFGHJKMNPQRTVWXYZ2346789` (no 0 O 1 I L U S 5), about 482 million codes.
- One `JoinRoom` Durable Object per code (single SQLite row, alarm wipes it at expiry).
- One `RateLimiter` Durable Object per client IP (fixed window: host 10/min, join 30/min, delete 20/min).
- CORS is off: no `Access-Control-*` headers are sent, `OPTIONS` returns 404. The game client is not a browser.
- `observability` is disabled in `wrangler.jsonc`, and the code never calls `console.*`.

## API

All bodies are JSON. Responses carry `cache-control: no-store`.

| Call | Notes |
| --- | --- |
| `POST /host` body `{"blob": "...", "ttl_s": 1800}` | blob 1..512 printable chars; `ttl_s` optional, clamped 60..21600, default 1800. Returns 201 `{code, secret, expires_at, ttl_s}`. Keep `secret`, it is shown once (only its SHA-256 is stored). |
| `GET /join/:code` | 200 `{blob}`, 404 if unknown or expired, 400 if the code is malformed. Codes are case-insensitive; spaces and dashes are ignored. |
| `DELETE /host/:code` header `X-Host-Secret: <secret>` | 200 `{ok:true}`, 401 no header, 403 wrong secret, 404 unknown. |

Rate limited calls return 429 with `Retry-After` seconds.

## Local tests (no deploy, no install)

Needs Node 22.6 or newer.

    cd services/join-service
    node --experimental-strip-types --test "test/*.test.ts"

(`npm test` runs the same thing.) The logic lives in pure modules (`code`, `room`, `ratelimit`,
`validate`, `router`); only `src/index.ts` touches Cloudflare APIs.

## Deploy steps (run by the human owner, not by an agent)

Use a dedicated wrangler profile tied to the co-op identity, never your personal account. Check the
final `workers.dev` URL and the account name before and after, and make sure neither exposes a
personal name.

1. `cd services/join-service && npm install` (installs wrangler, typescript, workers-types).
2. Pick a profile directory for this project, for example `.wrangler-profile` outside the repo, and
   point wrangler at it for every command in the session:
   - PowerShell: `$env:WRANGLER_HOME = "C:\path\to\coop-profile"` (older wrangler: `$env:XDG_CONFIG_HOME`)
   - bash: `export WRANGLER_HOME=/path/to/coop-profile`
3. `npx wrangler login` and sign in with the co-op Cloudflare account. Then `npx wrangler whoami` and
   confirm the account name is the co-op one.
4. Optional: `npm run typecheck`.
5. Review `wrangler.jsonc` (`name` decides the `<name>.<account-subdomain>.workers.dev` URL; the
   subdomain is chosen per account, so confirm it is neutral).
6. `npx wrangler deploy`. The first deploy applies migration `v1` (`new_sqlite_classes`, which the
   free plan requires).
7. Smoke test with the printed URL:

       curl -X POST https://<url>/host -d '{"blob":"test"}'
       curl https://<url>/join/<code>
       curl -X DELETE https://<url>/host/<code> -H "X-Host-Secret: <secret>"

8. Grep the repo and the deployed URL/account for personal identifiers before announcing the URL.

## Limits and notes

- Free plan: each request costs a couple of Durable Object requests (rate limiter + room). Fine for
  small scale; at larger scale replace the limiter with a Workers Rate Limiting binding.
- Brute forcing codes is bounded by the per IP join limit against 482M codes and a short TTL.
- No heartbeat or extend call: hosts that want longer sessions pass a larger `ttl_s`.
- Anyone with the code gets the blob, so the blob should hold a short-lived token, not a long-term secret.
