# ADR-0002: Full use of the Xbox 360 prototype symbols

- **Status:** Accepted (maintainer decision)
- **Date:** 2026-10-05

## Context

Xbox 360 development builds of Fallout: New Vegas from 2010 became public in
late 2025 together with their debug symbols. A public matching-decompilation
project lists all 69,012 function names of one of them (about 48,000 with
full C++ signatures), along with its class tables and source-file names.
Community tools already transfer some of these names to PC addresses. The
builds and symbols are unofficial material that was not released by the
rights holder. The research behind [METHODOLOGY.md](../METHODOLOGY.md)
recommended keeping them private. The maintainer weighed that risk and chose
full use.

## Decision

Names, types, struct layouts and source-file names from the prototype
symbols may be used in research and in committed code comments and docs.
Their provenance is always recorded:

- In Ghidra, transferred names carry the tag `src:xbox_pdb.<tier>` plus a pin
  naming the tool and commit that transferred them
  (`research/ghidra/NvImportNameMap.java`).
- In committed comments and docs, a name or layout taken from the prototype
  is followed by `(Xbox PDB)` the first time it appears in a file, so it can
  be found and removed later if needed.
- A prototype name is a strong hint, not a fact about the PC build:
  1.4.0.525 is a later build for another CPU. Behaviour is still confirmed
  from the PC exe, the data or a recording.

## Consequences

- Finding code becomes a name search. Struct field names can answer open
  questions such as the look-IK controller fields in
  [OPENING_LOOK_IK.md](../OPENING_LOOK_IK.md).
- Transfer accuracy is limited: about 87% exact agreement by vtable slot and
  52-63% true positives for cross-architecture similarity search in the
  sources cited by METHODOLOGY.md. Imported names are spot-checked.
- Legal risk rises; see METHODOLOGY.md "Legal and repository rules". This
  ADR does not cover leaked SDKs or source trees.
- This ADR supersedes METHODOLOGY.md's "Xbox 360 PDB decision" (default
  excluded) and the rules there that keep prototype-derived names private.
  Local transfer steps: [PROTOTYPE_SYMBOLS.md](../PROTOTYPE_SYMBOLS.md).
