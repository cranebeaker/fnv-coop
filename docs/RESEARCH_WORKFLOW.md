# Shared native research workflow

The overall method, its sources and the order of work are in
[METHODOLOGY.md](METHODOLOGY.md). The project's own research tools are in
`research/` (Ghidra scripts in `research/ghidra`, native oracles in
`research/nv-oracle`); they contain no game content.

Start with the current M1 topic handoff and its executable addresses. Read
the matching Ghidra export, inspect associated game data, then compare in the
original game. Keep exports, third-party tool snapshots, databases,
recordings, oracle logs and manifests in the private research tree outside
this repository.

## gamedb evaluation, 2026-10-04

Evaluated [gamedb](https://github.com/smileybaal/gamedb) at commit
`7054201291d704d64cdf37a9d3573c5d16a81fcf`, after reading its README, CLI,
parser, indexer and tests. Its release suite passed 25 integration tests,
including the built-in self-test.

The four-export player look-lock probe indexed three functions and one call
edge, with zero reported skipped/failed files. It silently missed `005cc4f0`:
the Ghidra declaration separates its return type from the function-name line.
Consequently the graph recovered removal's call to `005cc7a0`, but omitted
the assignment caller. Name-only edges also cannot establish the player-only
branch guard. A broad read of the mouse handler returned 754 lines.

**Decision:** retain direct, targeted Ghidra queries for now. Full-corpus
indexing was not justified. The private export inventory has 5,210 candidate
files, 943 exact duplicate copies and 45 same-basename groups with differing
contents; executable/export provenance is insufficient to merge those variants.
File counts and hashes do not establish extraction or behavioral correctness.

Private evidence: `%USERPROFILE%/nv-re/work/gamedb-eval-2026-10-04/` contains
`FINDINGS.md`, `player-look-manifest.json`, `export-inventory.json`, the tool
snapshot, four copied exports and the fresh `player-look-gate.sqlite` database.
No existing database, Ghidra project or game file was replaced.

## Queries across sessions

1. Follow the topic's address and executable hash; use `rg` to locate that
   address or function in the matching private export directory. Read only
   the relevant function or range. Inspect assembly when calling conventions
   or omitted register adjustments make the C output ambiguous.
2. Record source path, address, executable identity, export hash and guarded
   branch conditions beside the finding. Link the Rust implementation and
   generated regression, and distinguish original-game observation from tests.
3. If revisiting gamedb, select a version-consistent, hashed source snapshot
   and a new named database. Start with `search`, then a narrow `graph`/`read`.
   Compare extracted functions against an expected address list and verify
   edges in Ghidra. Zero skipped files is not an extraction-completeness check.
4. Do not combine the conflicting exports or repair/index the entire corpus
   until a bounded probe demonstrates useful retrieval and correct extraction.
   The private findings contain the exact commands needed to repeat this probe.

## Ghidra MCP trial, 2026-10-07

Question: can one shared Ghidra server replace the per-agent `ghidra.ps1` runs, which each start their own
JVM on their own project copy?

**Version.** bethington/ghidra-mcp, tag v6.0.0, commit `8cd2078e10b9ba28b188cb84ce5b9051a904b995` (Apache 2.0).
The release targets Ghidra 12.1.2; it compiled unchanged against our 12.1.4 and ran. The latest tag is a
7.0.0 release candidate, so the last stable tag was used.

**Setup.** Built the Java plugin/headless server with the project's own Gradle wrapper (about 2.5 minutes, the only
downloads were the Gradle distribution and Maven Central). Ran its headless server (`GhidraMCPHeadlessServer`) on a
fresh copy of the analyzed project (`ghidra_mcp`, copied from `ghidra_1`), bound to 127.0.0.1:8089, script endpoints
off, no auth token. Agents reach it through the project's Python bridge (stdio MCP, installed in a venv with
`pip install mcp`). We did not run its setup script or its other scripts (one self-elevates; none needed).

**What matched.** Decompiled 00c78610 (look-IK), 00c755e0 and 0095f930 (grab spring) three ways. The server output
is character for character identical to a Ghidra script using the GUI default decompiler options, and identical
through the MCP bridge. It is NOT identical to our `Decompile.java` run through `ghidra.ps1`: the server applies the
program's decompiler options ("respect read-only flags"), so constants read from read-only data are folded in
(`0.001`, `0.017453292`, `0.5`, `3.0`) where `ghidra.ps1` leaves the data references (01017d00, 01023128 and so on) unresolved. The code
is otherwise the same, same variables, same control flow. Both are valid; the server form is easier to read, the
script form shows the data addresses. Cite which one a finding used. (Making `Decompile.java` use
`DecompileOptions.grabFromProgram` would make the two agree.)

**Numbers** (this machine, 31 GB RAM).

| | ghidra.ps1 (one script run) | shared server |
|---|---|---|
| Startup | about 8 s every query (JVM + project open) | 15.7 s first start, 5.6 s later starts, once |
| Time per decompile | 8.6 to 9.8 s per invocation, any of the three functions | 0.3 s small, 0.6 to 1.0 s large (28 KB output) |
| Memory | peak working set 411 MB, 614 MB private, per run (heap cap 8 G) | 290 to 365 MB working set total, flat |
| Per added agent | another JVM each | one 1 s Python bridge process, no extra JVM |

The 8 GB figure is the heap cap, not what a decompile actually touches; the real cost of `ghidra.ps1` is one JVM per
query per agent and the multiples of that. Heavy scripts (whole-program scans) will use more in either route.

**Concurrency.** Three bridge clients (3 separate MCP sessions) made 30 decompile calls each at once, plus a fourth
client hitting the HTTP side directly with 30 more. All 120 calls succeeded in 24 s total. Mean latency per call was
about 0.7 s for each client against 0.59 s for a lone client, so requests are served in parallel with little
slowdown. Working set peaked at 354 MB. The server stayed healthy afterward.

**Risks.**
- No read-only mode. The server exposes 245 endpoints, including rename, retype, comment, delete, save and import,
  and the bridge lists them all as tools (244; 181 with the lazy config). Safety rests on agents using read tools only.
  Mitigations: loopback bind, scripts off, the project is a disposable copy, edits live in memory unless a save
  endpoint is called. One agent's stray write would be visible to every other agent until the server restarts.
- The server holds one shared in-memory program, so any change is shared, unlike the per-agent copies.
- Runtime needs Python and the `mcp` package from PyPI for the bridge, plus the built jar. Pin and keep them in the
  tools folder, not in the repo.
- Stop is a process kill; it leaves a stale project lock that the next start overcomes.
- The server's own project folder is rewritten on open (owner file), another reason to use a copy.

**Decision.** Use it for agent queries: one server, started once, registered as an MCP server (`ghidra-nvrs`), with
agents told to use read tools only. It removes the per-query JVM start (about 10x to 30x faster per decompile), the
per-agent project copies and the memory stacking that was running the machine out. Keep `ghidra.ps1`, its scripts
and the committed `research/` tools as the documented reproducible method: they run on a pinned project with no
extra services, so findings and reports must still cite results that `ghidra.ps1` reproduces (note the read-only
constant folding difference above). Treat the MCP server as a convenience layer, never as the source of record, and
do not allow it to write.

**How agents use it.** The maintainer starts the server once per session (local start script, kept with the tool
outside the repository). Agents query it read-only, over MCP or plain HTTP on the loopback port, e.g.
`decompile_function?address=0x00c78610`, `disassemble_function`, `get_xrefs_to`, `get_function_callers`,
`list_strings?filter=...`. Never call rename, set-type, comment, delete, save, import or script endpoints. Use a
private project copy with `ghidra.ps1` only for whole-program scripts or to reproduce a result for provenance.
