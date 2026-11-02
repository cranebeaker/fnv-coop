# PDB tools

## `msf_repack.py`

```sh
python research/pdb/msf_repack.py <in.pdb> <out.pdb>
```

Rewrites a PDB's container (MSF 7.00) with 4096-byte pages, copying every
stream unchanged, so that LLVM's `llvm-pdbutil` can read it.

The Xbox 360 prototype PDBs (policy: [ADR-0002](../../docs/adr/0002-xbox-prototype-symbols.md))
use 1024-byte pages, and their stream directory is too large for a single
block-map page: the superblock then lists several block-map pages one after
another. `llvm-pdbutil` stops with "Too many directory blocks" because it only
accepts one. With 4096-byte pages the directory's page list fits in one.

Checked on 2026-10-06 with `Fallout_Release_MemDebug.pdb` (3,289 streams) and
LLVM's `llvm-pdbutil` from the maintainer's LLVM install: `dump -summary`
reports the same GUID and age the loader checks
(`{036BA387-1386-4F96-A709-DA70225CDF94}`, age 1), and `dump -publics`,
`-types` and `-symbols` all run (120,338 public symbols; 51,293 procedure
records with parameter and local names). Only the container is tested; the
copy is never written back over the original.

Useful dumps (outputs stay in the private research tree):

```sh
llvm-pdbutil dump -publics  MemDebug.repack.pdb > publics.txt   # names, Xbox addresses
llvm-pdbutil dump -types    MemDebug.repack.pdb > types.txt     # classes, fields, offsets
llvm-pdbutil dump -symbols  MemDebug.repack.pdb > symbols.txt   # functions, parameters, locals
```

Standard library only; no game content.
