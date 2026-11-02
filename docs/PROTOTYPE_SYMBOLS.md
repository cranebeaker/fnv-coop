# Using the Xbox 360 prototype symbols (local steps)

Policy: [ADR-0002](adr/0002-xbox-prototype-symbols.md), set by the maintainer
on 2026-10-05.

- Names, types, layouts and source-file names from the Xbox 360 prototype
  builds may be used, including in committed docs and comments (marked
  `(Xbox PDB)`). In Ghidra, tag each transferred name `src:xbox_pdb.<tier>`.
- Decompiled logic follows [ADR-0003](adr/0003-decompiled-code-in-the-project.md).
- Leaked SDK and source trees stay excluded, and so do fnv-source-atlas's
  `sdk_*` tables.
- Keep the XEX and PDB, which reportedly come from late-2025 preservation
  releases, in `%USERPROFILE%\nv-re\x360\` with their SHA-256 recorded. Never
  commit them.

Researched in a cloud session that had neither the XEX nor the PDB. The
tools, counts and SQL below were checked there as noted under "What was
verified". Variables (`$NvRe`, `$Headless`, `$Work`) follow
[research/ghidra/README.md](../research/ghidra/README.md).

## 1. Load the Xbox build

Use `Fallout_Release_MemDebug.xex` (title 425307E0, 2010-11-10). It is the
newest of the builds, and both the public decomp and the atlas use it.
`Fallout_Release_Beta` (2010-08-22) is only a second opinion.

Loader: **Xenon360 v0.7.0** [X1] (tag `03fa0749`; later commits change only
its README).
- It targets exactly Ghidra 12.1.4 and adds VMX128 instructions, which stock
  Ghidra lacks.
- It checks the PDB/XDB GUID and age and applies the PDB with Ghidra's
  Universal PDB code, headless too.
- It built and passed its smoke test in the cloud session. Its PDB path is
  untested, and the project is new.

The alternative is XEXLoaderWV [X2]. Its 12.1.3 release raises Ghidra's
version prompt on 12.1.4, but `master` (`4bcf58da`) builds a 12.1.4 package
with `gradle buildExtension`. It has no VMX128 support and loads PDBs only in
the GUI. Install only one of the two loaders.

```powershell
# GUI: File > Install Extensions > ghidra_12.1.4_XenonVMX128_0.7.0.zip; restart.
$env:GHIDRA_HEADLESS_MAXMEM = '24G'
$X = "$NvRe\x360"
& $Headless "$NvRe\ghidra-x360" FNV_X360 -import "$X\Fallout_Release_MemDebug.xex" `
  -loader XenonXexLoader -loader-symbol-file "$X\Fallout_Release_MemDebug.pdb" `
  -loader-symbol-mode all -log "$X\import.log"
```

Use symbol mode `all`. The default, `public-symbols`, maps to Ghidra's
`PUBLIC_SYMBOLS_ONLY` and loads no types.

Check the import:
- the log accepts the GUID and age;
- there are about 69,012 functions;
- `bhkRagdollController::BoneTrack` is at 0x82B23CA8.

## 2. Transfer names to the PC exe

**Fastest route:** the vtable-slot tier of BethesdaGhidraScripts (BGS) [X3] at
`dabb919186e0dde601908adfa5b174f699525b66` (v1.2.2, still HEAD).

- **Use only its Python data layer,** `pdb_naming.build_fallback_symbols()`.
  It does not need the Ghidra 12.0.4 that BGS pins. Skip BGS's PyGhidra
  importer: it is untested on 12.1.4 and writes no provenance tags.
- **Default output:** 9,406 entries, of which 458 are PDB-derived function
  names (`xbox_pdb_matched` 362, `string_anchor` 85, `thunk_jmp` 11).
- **`BGS_FNV_INCLUDE_EXPERIMENTAL=1`** adds `ctor_ghidra_xref` 3,519,
  `callgraph_align` 2,574 (PDB-derived), `dtor_ghidra_inferred` 193,
  `source_file` 71 and `imm_paired` 14.
- **The Xbox vtable tier is off by default.** The shipped
  `refs/fnv_xbox_vtables.json` (3,996 Xbox tables, 72,143 slots) uses schema
  v1, which the loader ignores. Converting it to v2 enables `xbox_vtable`,
  which names 9,226 PC functions.
- **Drop the folded stubs.** 231 of those 9,226 (2.5%) are stubs shared by
  several methods after identical-code folding; one empty function sits
  behind 200 slots.
- **Leave `BGS_FNV_ARTIFACTS` and `BGS_FNV_XBOX_EXE` unset.** The first only
  adds signatures, `.obj` tags and locals from PDB extracts. The second is
  read only by `extract_xbox_rare_immediates.py`.

```powershell
git clone https://github.com/1001Bits/BethesdaGhidraScripts "$NvRe\tools\bgs"
git -C "$NvRe\tools\bgs" checkout dabb919186e0dde601908adfa5b174f699525b66
Set-Location "$NvRe\tools\bgs\scripts\commonlibnvse"
python merge_xbox_vtables.py refs\v2.json refs\fnv_xbox_vtables.json
Move-Item -Force refs\v2.json refs\fnv_xbox_vtables.json
python "$NvRe\names\bgs_to_csv.py" "$NvRe\names\xbox_names.csv"
```

`bgs_to_csv.py` (below; copy it to `%USERPROFILE%\nv-re\names\`):
- keeps the function entries of `xbox_vtable`, `xbox_pdb_matched`,
  `string_anchor` and `thunk_jmp`, minus folded stubs;
- limits names to `[A-Za-z0-9_:~<>]` by turning `*` into `P`, `&` into `R` and
  anything else into `_`;
- suffixes repeated names with `_ovN`;
- writes virtual addresses (RVA + 0x400000).

The cloud run produced 9,130 rows with `llvm-undname`. Windows demangles with
dbghelp, so the count there may differ slightly.

```python
import collections, csv, re, sys
sys.path.insert(0, '.')
import pdb_naming as P
T = {'xbox_vtable': 'vt', 'xbox_pdb_matched': 'cls', 'string_anchor': 'str', 'thunk_jmp': 'thunk'}
meth = collections.defaultdict(set)
for rva, n in P._load_xbox_vtable_methods():
    meth[rva].add(n.rsplit('::', 1)[-1])
seen, rows = collections.Counter(), []
for s in P.build_fallback_symbols():
    tier = re.split(r'[ /|]', s['src'])[0]
    if s['t'] != 'func' or tier not in T or len(meth[s['a']]) > 1:
        continue
    n = re.sub(r'[^A-Za-z0-9_:~<>]+', '_', s['n'].replace('*', 'P').replace('&', 'R'))
    seen[n] += 1
    n += f'_ov{seen[n]}' if seen[n] > 1 else ''
    rows.append(['%08x' % (0x400000 + s['a']), n, 'xbox_pdb.' + T[tier], 'bgs-dabb919186e0'])
with open(sys.argv[1], 'w', newline='') as f:
    csv.writer(f).writerows([['address', 'name', 'source', 'pin']] + rows)
```

The `kind` column is left out on purpose. With an explicit `kind=function`,
one address where Ghidra cannot create a function aborts the whole file.

### Cross-checks and more names

- **fnv-source-atlas** [X4] (`499c42ca`). Its v0.5.0 release ships a SQLite
  database built from MemDebug (`fnv-source-atlas-data-0.5.0.zip`, 752 MiB,
  SHA-256 `2d4713502310a4ba00872ceb260e5f04c1b41e19291e381540183e27c4af07a9`).
  - It holds candidate matches only; each piece of evidence names an
    independence group (`class_slot_alignment`, `string_reference`,
    `call_graph`).
  - Its own export applies only reviewer-accepted matches. Instead, query it
    for match sets with exactly one alternative, together with their
    supporting groups (SQL below).
  - First check that its `pc_executable` digest is your exe.
- **ieee802dot11ac/fnv** [X5] (`074e045e`, CC0). `config/425307E0/symbols.txt`
  lines look like `<mangled> = .text:0x82B23CA8; // type:function size:0xCC8`.
  `splits.txt` maps 3,181 `.cpp` files to address ranges. Both use Xbox
  addresses only; use them for searching, source-file tags and ordering
  functions within a file. Its README asks contributors not to use AI; this
  plan only reads its CC0 data.
- **Ghidra Version Tracking** [X7], with the Xbox program as source and a PC
  copy as destination; cross-language sessions are supported.
  - Run Exact Data Match on strings and accept the 1:1 matches.
  - Then run the Data Reference, Function Reference and BSim Program
    correlators, the last with "Use Accepted Matches as Seeds".
  - Byte and instruction correlators cannot match PowerPC to x86. BSim across
    instruction sets found only 52-63% of true matches, with 4-6% false
    positives [X6].

```sql
-- Atlas schema 8: one-alternative candidates with their supporting groups.
SELECT printf('%08x', pg.address) pc, n.name, p.producer,
  (SELECT group_concat(DISTINCT g) FROM (
     SELECT independence_group g FROM match_hypothesis_evidence
      WHERE hypothesis_set_id = s.hypothesis_set_id AND effect = 'supports'
     UNION SELECT independence_group FROM match_hypothesis_alternative_evidence
      WHERE alternative_id = a.alternative_id AND effect = 'supports')) grp
FROM match_hypothesis_sets s
JOIN match_hypothesis_alternatives a ON a.hypothesis_set_id = s.hypothesis_set_id
JOIN match_claims c ON c.claim_id = a.claim_id
JOIN functions pf ON pf.function_id = s.pc_function_id
JOIN address_groups pg ON pg.address_group_id = pf.address_group_id
JOIN function_names n ON n.function_id = c.xbox_function_id AND n.is_primary = 1
JOIN provenance p ON p.provenance_id = s.provenance_id
WHERE s.status = 'candidate' AND (SELECT COUNT(*) FROM match_hypothesis_alternatives x
  WHERE x.hypothesis_set_id = s.hypothesis_set_id) = 1;
-- PC exe check:
SELECT a.digest FROM manifest_entries e JOIN input_artifacts a ON a.content_id = e.content_id WHERE e.role = 'pc_executable';
```

Before importing atlas rows, keep only PC addresses where every row agrees on
one name, require at least 2 distinct groups, demangle names that start with
`?`, and sanitize them like the BGS rows.

### Tiers

The `pin` column holds the tool and commit, for example `bgs-dabb9191`; the
Ghidra tag keeps its first 12 characters.

| source | Evidence | Import |
| --- | --- | --- |
| `xbox_pdb.vt` | PC RTTI class and slot; tables of equal size matched one to one; one method name | after the spot-check |
| `xbox_pdb.str`, `.cls`, `.thunk` | a unique string, or one naming its own function; BGS class and thunk tiers | after the spot-check |
| `xbox_pdb.atlas` | one alternative and at least 2 independence groups | after the spot-check |
| `xbox_pdb.cg` | call-graph propagation; only 180 of BGS's 2,574 rows have 3 or more votes | only with a second, independent group |
| `xbox_pdb.bsim` | BSim | never; leads only |

Slot names transferred by position alone agreed with PC headers 86.8% of the
time ([METHODOLOGY.md](METHODOLOGY.md)). The error rate of the stricter tiers
is unmeasured.

## 3. Import, spot-check, types

1. **Import.** Run the name-map dry run, then the real run, from
   `research/ghidra/README.md` section 2 on `$Work`.
   - One rejected row aborts the whole import.
   - A row whose address already has a non-default name is reported as
     `conflict`. If that name is a `Class::vfNN` label, re-run just those rows
     with `force=1`.
2. **Spot-check 100 random rows per tier** (`Get-Random -Count 100 -SetSeed 1`).
   A row is right when its address sits in a vtable slot whose RTTI class is
   the named class or a base of it, and its strings, the `.cpp` names in its
   asserts and its callees match the Xbox function and that function's
   `splits.txt` file. Also compare both decompilations for ten of the rows.
   With 100 rows, zero errors bounds the error rate below 3% at 95%
   confidence. Keep a tier only if 2 or fewer rows are wrong, and record the
   counts.
3. **Types.**
   - Without Ghidra: [research/pdb](../research/pdb/README.md) repacks the
     PDB so that `llvm-pdbutil dump -types|-symbols|-publics` can read it
     (it refuses the original's 1024-byte pages). The symbol dump also has
     parameter and local names, for example the four `PlayBink` integers
     ([MOVIES.md](MOVIES.md)).
   - Drag the needed Xbox classes into a project data type archive, then
     apply it to the PC program.
   - PDB structures carry explicit field offsets, and pointers and `long` are
     4 bytes on both machines, so offsets carry over.
   - PowerPC is big-endian. That does not change offsets, but recheck
     bitfields and VMX `__vector4` members.
   - The 2010 build may have debug-only or Xbox-only fields, and may lack
     fields the PC build added later. Check each layout against the PC
     constructor's field writes and the size it passes to `operator new`.

## 4. Look-IK (M1)

`symbols.txt` names the look-IK controller class `bhkRagdollController`
(`bshavok/bhkRagdollController.cpp`). The PC functions already traced in
[OPENING_LOOK_IK.md](OPENING_LOOK_IK.md) appear in the same order as these
Xbox functions, which suggests the pairs below. **The pairs are unverified.**

| PC address | Xbox function (Xbox PDB) |
| --- | --- |
| 008a3b70 | `SetLookAtIKTarget(NiPoint3 const&, bool)` |
| 00c75580 | `SetLookIKActive(bool)` |
| 00c755e0 | `LimitBoneRot(LookIKBone, hkQuaternion const&)`, returning a float |
| 00c78160 | `AdjustCurrentTarget()` |
| 00c78610 | `BoneTrack(LookIKBone, hkQsTransform&)` |
| 00c79340 | `InitBoneParams(hkaPose*, char const*, LookIKBone)` |
| 00c7aa60 | `DoLookAtIK(hkQsTransform&)` |
| 00c7d630 | possibly `DoRagdollAnim(bool)` |
| 00c7de60 | `InitLookIK(char const*)` |
| 00c7f060 | the constructor |

If the pairs hold:
- the 1/0 "mode" argument of 00c78610 is a `LookIKBone` that picks a bone;
- the 0x50-byte records from +0xf4 hold one bone each: +0xf6 and +0x110
  belong to bone 0, and +0x144 to bone 1;
- the PDB layout of `bhkRagdollController` names those fields and +0xb2 and
  +0x170. `bhkLookIKData` may name them too, if it is 0x50 bytes.

Also search for:
- `LookIK_DATA`, `BSLookIKNote`, `hkaLookAtIkSolver`;
- the settings `fEaseAngleShutOff`, `fMinTrackingDist` and `fMaxTrackingDist`
  (all `:LookIK`);
- for the gaze target and eyes: `HighProcess::UpdateHeadTrackTargets`,
  `BaseProcess::HEAD_TRACK_TYPE` and
  `BSFaceGenAnimationData::SetHeadTrackVector`.

`symbols.txt` has 77 HeadTrack symbols, 50 of them functions. Confirm each pair
in the PC disassembly before recording it in OPENING_LOOK_IK.md.

## What was verified

In the cloud session, without the XEX or PDB:
- Xenon360 at `03fa0749` and XEXLoaderWV at `4bcf58da` both built against
  Ghidra 12.1.4 (Gradle 8.14.3, JDK 21). Xenon360's smoke test passed
  (synthetic XEX, headless, VMX128 decoding, decompiler).
- BGS `pdb_naming.py` at `dabb9191` produced every count above, using
  `llvm-undname` in place of dbghelp, including the v1-to-v2 migration and
  the folded-stub analysis.
- The converter produced 9,130 rows with no names outside the import
  script's character set, no duplicate names and no duplicate addresses.
- The atlas SQL ran on an empty schema-8 database and on a synthetic
  populated one: a one-alternative set was returned with its merged groups,
  and a two-alternative set was excluded.
- The look-IK names, Xbox addresses and their order were read from
  `symbols.txt`.

Not verified: Xenon360 on the real PDB (success, time, whether 24 GB is
enough); the atlas database contents and the PC exe it was built from; Version
Tracking on these two programs; every PC-to-Xbox look-IK pair; and counts
under dbghelp.

## Sources

- [X1] https://github.com/EveryoneEverybody/Xenon360. Tag v0.7.0 =
  `03fa0749b5f6fc326744dbfac73cde43867a4d21`; release asset
  `ghidra_12.1.4_XenonVMX128_0.7.0.zip`; code in
  `src/main/java/xenon360/XenonXexLoader.java` and `XenonPdbSupport.java`.
- [X2] https://github.com/zeroKilo/XEXLoaderWV. `master` =
  `4bcf58da4caa5260644fc19f5eef5ccde3b7ff46`; release 12.1.3 =
  `edbceeba2c1da5065abd3ea02f5d2e2ca445f714`; PDBs load through
  `LoadPdbDialog` (GUI only).
- [X3] https://github.com/1001Bits/BethesdaGhidraScripts/tree/dabb919186e0dde601908adfa5b174f699525b66
  (`scripts/commonlibnvse/pdb_naming.py`, `paths.py`, `merge_xbox_vtables.py`,
  `vtable_schema.py`, `refs/`, `toolchain.lock.json`).
- [X4] https://github.com/amarkham23/fnv-source-atlas. HEAD =
  `499c42ca4f65f3b15996d8d39fc49a2d5b41373c`; tag v0.5.0 =
  `cef3f5ea88d5333e80f2f395c14e8c317d626490`
  (`src/fnv_atlas/schema.py`, `docs/DATA_MODEL.md`, `docs/CONSUMER_EXPORTS.md`).
- [X5] https://github.com/ieee802dot11ac/fnv/tree/074e045ef8220666fd8481e0745af6ecf197f24b/config/425307E0
- [X6] https://github.com/MISP/bsimvis/issues/30
- [X7] Ghidra 12.1.4 help: `VT_Correlators.html`, `VT_Apply_Options.html`,
  `BSim_Correlator.html`; `DefaultPdbApplicator.java`
  (`PUBLIC_SYMBOLS_ONLY`); `ExtensionInstaller.java`;
  `support/analyzeHeadless.bat` (`GHIDRA_HEADLESS_MAXMEM`, default 2G).
