# families: look-alike families of exe functions

Many FalloutNV.exe functions are near-identical template instantiations:
BSSimpleArray, BSScrapArray, NiTMap, LockFreeMap and NiTPointerList helpers,
hkArray, scalar deleting destructors, constructor and destructor pairs, Havok
class reflection stubs. Once one member of such a family is translated, the
others are mostly the same Rust with different offsets, element sizes and
callees. This tool groups every function by its normalised instruction shape
and lists each family with its translated members (and where their Rust is)
and its open members, so a batch can be a whole family plus a finished
sibling as the template.

## Inputs

- `research/engine-map/engine_map.tsv`: every function (address, Xbox PDB
  name, unit, subsystem).
- The ledger's per-function TSV (status and the Rust `file:line` citing it):

  ```text
  cargo run -q --release --manifest-path scripts/ledger/Cargo.toml -- --tsv <private>/ledger.tsv
  git checkout -- docs/LEDGER.md      # the ledger rewrites it too
  ```

- Each function's disassembly from the read-only Ghidra server
  (`disassemble_function`, default `http://127.0.0.1:8090`), cached by
  `fetch` in a private directory, one `<address>.txt` per function. The
  cache is never committed.

## Run

From this directory (compile through the machine-wide queue):

```text
heavy.sh cargo build --release
./target/release/families fetch --map ../engine-map/engine_map.tsv --cache <private>/dis --threads 6
./target/release/families group --map ../engine-map/engine_map.tsv --ledger <private>/ledger.tsv \
    --cache <private>/dis --out families.tsv --summary summary.txt \
    --focus-unit fallout/ai/middlehighprocess.cpp
./target/release/families show --map ../engine-map/engine_map.tsv --ledger <private>/ledger.tsv \
    --cache <private>/dis [--shape] <address>...
```

`fetch` only fetches what the cache lacks (about 15 minutes for all 66,000
functions at 6 concurrent requests). `group` takes seconds; re-run it after
the ledger moves. `show` prints a function's strict and loose families with
each member's status and Rust citation; `--shape` also prints its
normalised instructions (to the terminal only).

## Masking rules

Each instruction becomes `MNEMONIC op,op,...`, each operand normalised:

| Operand | strict | loose |
|---|---|---|
| register (`EAX`, `CL`, `ST0`, `FS`...) | kept | kept |
| branch (`Jcc`, `JMP`, `LOOP`) to an instruction of the same function | `L<index>` of the target instruction | same |
| branch out of the function (tail jump) | `X` | `X` |
| direct `CALL` target | `F` | `F` |
| immediate or displacement in the image range `0x00400000..0x01300000` (the image including `.bss`, which ends near `0x01280000`: globals, vtables, RTTI objects, float constants in `.rdata`, SEH handlers, cookies) | `A` | `A` |
| immediate or displacement in `0x01300000..0xffff0000` (float bit patterns, hashes, masks, magic numbers) | `K` | `K` |
| other immediate or displacement (struct and stack offsets, sizes, counts, small negatives) | kept | `I` |
| index scale (`ECX*0x4`) | kept | `*S` |
| memory operand | size, segment and base/index registers kept, terms normalised as above | same |

A function's shape is its normalised instruction sequence; its family id is
the FNV-1a 64 hash of the shape, prefixed `s` (strict) or `l` (loose). Two
functions are in the same strict family when they do the same thing to the
same offsets, calling possibly different functions and touching possibly
different globals: typically the same template over different types of the
same layout. The loose level also ignores offsets and sizes, so it joins
the same template over different element sizes and same-shaped accessors of
different fields. Every strict family lies inside one loose family.

Register allocation is kept, so the same source compiled with different
registers is not grouped. Very short shapes (`MOV EAX,..; RET`, a bare
`RET 4`) make large families of unrelated functions; the `insns` column lets
the lead filter them out.

## Outputs

`families.tsv`, one row per family with at least one open member (sorted
by open members, then size):

| Column | Meaning |
|---|---|
| `family` | `s...` strict or `l...` loose id (stable across runs while the shape stays) |
| `level` | `strict`, or `loose` for a loose family that joins two or more strict families (loose families equal to one strict family are not repeated) |
| `insns` | instructions per member |
| `members`, `open`, `translated` | counts |
| `units` | Xbox units of the members (the first 12, then `+N more`) |
| `names` | up to three Xbox PDB names of members |
| `translated_members` | `address file:line` of each translated member (the ledger's first citation) |
| `open_members` | addresses of open members |
| `other_members` | `address:status` for the rest (traced, library, platform, static init table) |

`summary.txt`: how many functions fall into families of two or more, how
many open functions have a translated sibling at each level, per subsystem
and for the `--focus-unit`s.

Both hold only addresses, counts, engine-map names and units, ledger
statuses and Rust paths: nothing from the executable itself.
