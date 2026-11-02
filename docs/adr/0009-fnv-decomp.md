# ADR-0009: The community's matching decompilation (fnv_decomp) may be used

- **Status:** Accepted by the maintainer, 2026-11-02, with the permission of fnv_decomp's
  authors.
- **Date:** 2026-11-02
- **Extends:** [ADR-0003](0003-decompiled-code-in-the-project.md) (decompiled logic may be
  translated, marked with its address).

## Context

On 2026-11-02 the fnvdecomp community ([fnvdecomp.com](https://fnvdecomp.com)) published
[momusdigital/fnv_decomp](https://github.com/momusdigital/fnv_decomp) (this decision was made
against its commit `2aabd5ad331a54374381f8f6153e042d075fb50f`): C++ source for
`FalloutNV.exe` 1.4.0.525. Its authors report that, built with Visual C++ 2008 (2,713 of its 2,988 units with SP1, 274 with the original
release) and 2005 SP1 (1 unit), every in-scope function compiles to the same bytes as the exe:
68,657 functions and 1,954 exception-handling tails.

About half of its functions (36,718 of its 68,657 in `build/units.json`) still have
placeholder names made from their addresses, in every folder, and many types are wrappers
named after an address. The files under `src/staging/` hold functions with no known source
file, grouped by address range: their file names are not real source paths. Its source paths
come from the Xbox 360 build (1,529 units say so), and many of its names may too; it does not
say.
Check a name against the Xbox prototype symbols ([ADR-0002](0002-xbox-prototype-symbols.md))
before marking it `(Xbox PDB)`.

Its repository grants no licence. Its authors wrote to the maintainer on 2026-11-02
(spelling corrected):

> You are fine to use everything however you please, you don't have to credit us, that's why
> we chose to send it out with no license.

The maintainer keeps the original message.

## Decision

- What may come into nv-rs from it: Rust translated from or checked against its C++, and its
  names, types and layouts. No C++ file, header or excerpt is committed, in code or in docs
  (the rule against committing decompiler output stands). Its comments were written by machine
  (its README says so): they are hints, copied only after they are checked against the exe.
- Placeholder names (made from addresses, or wrappers named after one) are not adopted,
  wherever they appear; write real names from what the code does.
- The exe stays the reference. A translation keeps its address marker; a fix found through
  fnv_decomp is checked against the disassembly before it merges, keeps a regression test,
  and keeps the oracle check where the function is eligible. Where x87 code rounds
  intermediate values is read from the disassembly: the C++ does not show it.
- Marked where used, as ADR-0002 and ADR-0003 material is:
  - a translation written or corrected from it gets `, read against fnv_decomp` after its
    marker:

    ```text
    // Translated from 00c755e0 (decompiled, FalloutNV.exe 1.4.0.525), read against fnv_decomp.
    ```

  - a name, type or layout taken from it gets `(fnv_decomp)` where it first appears in a file;
    `(Xbox PDB)` instead when the name is the Xbox prototype's.
- Scope: the permission covers the fnvdecomp contributors' own work. It does not cover
  `third_party/` (zlib, under its own licence), and it gives no rights in the game or in the
  libraries linked into it (Gamebryo, Havok, SpeedTree, FaceGen, the C runtime). Material from
  it is treated like ADR-0003 material: nv-rs's GPL covers only original work, as
  [LEGAL.md](../../LEGAL.md) says.
- Credit is not required by its authors; LEGAL.md thanks them and records the permission.

## Consequences

- Translations can be read against source: local variable sizes (the temporary memory a
  function needs), field layouts, inlined callees and the order of float operations are often
  written out instead of inferred from the disassembly.
- An audit of every translated function against it becomes possible; each difference is still
  settled in the exe.
- Material taken from it can be found with `rg "fnv_decomp"` and removed if the permission were
  ever withdrawn. A published statement from its authors (a licence in their repository) would
  make the permission clear to everyone who receives nv-rs; the maintainer may ask for one.
