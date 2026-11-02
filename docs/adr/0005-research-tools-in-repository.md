# ADR-0005: Research tools live in `research/`

- **Status:** Accepted
- **Date:** 2026-10-05

## Context

The Ghidra scripts and helpers used so far lived only in the maintainer's
private research folder, so other sessions and contributors could not repeat
the method.

## Decision

The project's own research tools are committed under `research/`:
`research/ghidra` (Ghidra scripts: identity, complete export, function
cards, command-table labels, name-map import) and `research/nv-oracle` (its
own Cargo workspace: `nv-call`, `nv-probe`, `nv-inject`). They contain no
game content. Their outputs (exports, databases, logs, vectors, snapshots)
stay in the private research tree.

## Consequences

- The tools are tested here only on synthetic binaries; their READMEs say
  what still needs the real exe.
- Third-party tool snapshots and anything they produce from the game stay
  private.
