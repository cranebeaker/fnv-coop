# Engine-owned state: Phase 2

Phase 2's gate remains a real `.fos` round trip, compared change form by
change form. The first bounded step is `engine::extra_data::ExtraList`:
a handle to a list in an explicitly supplied `Engine`. It is not yet used
by `world` or the viewer. Current save-reader evidence and limitations are
in [FOS_SAVES.md](FOS_SAVES.md); overall priority stays in
[MILESTONES.md](MILESTONES.md) and [HANDOFF.md](HANDOFF.md).

## Saved-local value follow-on

The next bounded batch corrects `SCRIPT_LOCAL` copying (`005ac150`) and
`ScriptLocals::SetVariable` (`005a9290`) (Xbox PDB). PC reads and writes the
entire integer flag byte rather than normalizing it to a Boolean. Its x87
load/store also quiets signaling NaNs while preserving their sign and
payload. SetVariable fetches a live list item twice before advancing and
changes only the first matching ID. The implementation preserves those details.

Two save-reader helpers, `008648e0` and `00864940`, complete the generated
numeric/reference-local load path. The first decodes a three-byte saved
index, stores the resolved form ID, then reports a nonzero index resolving
to zero. It checks the index after the decoder has run; the second tests
the three bytes with the PC short-circuit order. Reference locals store
the form ID as opaque low 32 bits in the eight-byte slot, not a numeric
floating-point conversion. An older test double incorrectly modeled the
latter; its fixture now uses the real helper. The matching writer helper,
`00865db0`, initializes and encodes the compact ID, then appends its three
bytes. Its unused second argument remains unused, as in the PC executable.
`ScriptLocals::SaveGame` (`005a9db0`) also reloads the reference-variable ID,
effect flag and effect pointer after the respective helper calls, matching
the PC reads instead of retaining values from before those calls.

One hundred and two private native cases pass: forty copy cases (including
aliases), eighteen setters, nine full generated LoadGame buffers and eleven
reader cases, six writer cases and eighteen full generated SaveGame buffers.
The SaveGame cases include signed zero, infinities, a subnormal, quiet and
signaling NaNs, created-reference IDs and both action-flag states. Guards,
raw flags, first-match behavior, return widths and
stack cleanup are checked where defined. Native calls use FalloutNV.exe
1.4.0.525 with FCW `027f` and MXCSR `1f80`. Independent PC/source review
found two additional SaveGame reload gaps; both are corrected with callback
regressions. All focused checks pass: 1,343 script tests, eight reader tests
and ten writer tests, including thirteen new regressions. Three copied
overlap-fixture strings had an extra trailing byte; replacing them with the
exact independently captured 80-byte buffers resolved the initial test-only
failure. Production behavior was unchanged by that correction.
All 16,273 workspace tests, including doctests, strict lint, formatting and
release/debug builds pass. Checks used one Cargo job to limit memory peaks.
Final normal-registry release probes pass all nine LoadGame and eighteen
SaveGame buffers against the native full-buffer expectations.

Eighteen native SaveGame/LoadGame/SaveGame chains also pass, with 36
additional calls carrying the actual saved bytes and post-load state into
the next leg. They verify zero omission, raw reference bits, action flags
and signaling-NaN quieting on load. The final Rust release probe passes all
eighteen three-leg chains with the actual saved output carried into the next
leg and distinct destination layouts. Signaling NaNs quiet on load; the next
save preserves the resulting quiet bits.

These generated buffers contain nonzero numeric and created-reference
locals. They do not establish restoration of the unavailable mod scripts
in the historical saves. The existing world importer uses numeric form IDs
plus a reference-kind tag; its future engine bridge needs an explicit
conversion to and from the engine's opaque low-bit representation. No world
or viewer integration is included here.

## Inventory source ownership probe

A private read-only prototype against the preceding release registry loads
eighteen original multi-stack Count/Health items in fresh isolated engines.
Its immutable snapshots retain each stack, raw health bits, the separate item
count delta, exact captured save and containing-record bytes, saved/resolved
identities, record metadata, inventory ordinal and validated source spans.
Untouched writes reproduce all eighteen original item payloads byte-for-byte.
A generated three-stack fixture preserves absent versus explicit zero/one
Count, absent versus explicit minus-one Health, and Worn nodes. Snapshots
survive engine disposal and clone independently; two simultaneous owners
with coincident arena addresses retain distinct data.

Twenty-seven rejection cases cover mismatched metadata, detached backing
and invalid spans. Admission excludes three complete-record decode failures
and three unresolved holder IDs, without changing the eighteen-item target
set. The private harness uses generated form headers and partial allocator
startup; it is not production runtime initialization. Its strict source
validation reparses the captured save and would need admission-time caching
in a production API. Unknown payload semantics, whole-save writing and
world/viewer integration remain open. The reusable immutable source envelope
is now `fos::inventory_source`. Its 38 package tests pass; on 2026-10-12
all 16,291 workspace tests (one ignored), strict workspace lint, formatting
and the release build passed with one Cargo job. Its scope and
source-preservation rules are described in [FOS_SAVES.md](FOS_SAVES.md).

A second private probe passes the actual new `ItemEnvelope` objects into
the saved-local batch's final release engine registry. All eighteen original
targets consume the complete input, retain ordered Count/Health values and
the separate signed count delta, and reproduce the untouched item bytes.
Owned snapshots retain their source envelopes after both the source container
and engine are dropped. The same three decode failures and three unresolved
holders remain outside admission. This uses generated form headers and
partial allocator setup; it establishes bounded composition, not production
startup or world integration. Both private probes were rerun on 2026-10-12
against this branch's final release `fos` and `engine` libraries rather than
the frozen standalone build. The read-only comparison checked 36,623 decoded
records and 168,638 preserved items with zero invalid captures; all eighteen
original targets pass both the envelope comparison and the engine bridge,
with the same three decode failures and three unresolved holders.

## Inventory replay through the engine (Phase 2 step A), 2026-10-13

Each saved inventory was loaded by the translated buffer loader `004d4160`
and written back by `004d4090`, and the written section was compared with the
original bytes. This is the path the 1.4 PC change-form load takes: the
two change-form loaders `00847df0` (call at `00848b68`) and `008495d0`
(call at `008496f9`) dispatch form vtable slot `0x5c`. For TESObjectREFR
(vtable `0102f55c`) that slot is `005623d0`, which calls `004d4160` under
change flag `0x08000020`. The actor loader `00932a00` reaches `005623d0`
too (`00932a2f`). The save side is `00562230`, which calls `004d4090`
(`00562300`). The stream path `InventoryChanges::LoadGame` (`004d3cc0`, Xbox
PDB) is reached only from `TESObjectREFR::LoadGame` (`0055ec40`, slot
`0x60`, called directly from `009320e0`). Its load fix `004cbbc0` is reached
only from `004d3cc0`. Neither PC change-form loader calls slot `0x60`.

Two behaviors of the original code determine the expected output.

- **Reversed order.** `004d4160` adds each loaded entry with `005ae3d0`
  (call at `004d42a9`, ECX = the list of changes). That function pushes to
  the front of the list: `00470440` copies the old head into a new node, and
  the new entry becomes the head. `004d4090` writes the list in order. So
  loading and then saving writes an inventory's entries in reverse order,
  with each entry's bytes unchanged. The translation does the same.
- **Dropped entries.** An entry whose saved ID resolves to no form is
  deleted, and a null entry is added in its place (`004d4160`). `005ae3d0`
  ignores the null and `004d4090` neither writes nor counts it.

One translation gap was found and closed: `004d4090` switches the save
buffer's mode word through the shared setter `0050f9c0`, which was
untranslated. It is now in `tesobject.rs`, with a regression test.

The private probe ran against all eight original saves. It replays all
36,629 flagged records (168,638 items) through the release registry, with
generated form/reference receivers and the partial allocator setup. Script
receivers are built from installed SCPT records only when every plugin of
the save is installed. Results:

| Outcome | Records | Items |
| --- | ---: | ---: |
| Exact, single entry | 6,252 | |
| Exact, entries in the reverse order explained above | 21,881 | |
| Exact after the game's drop of entries resolving to no form | 110 | 121 dropped |
| Empty inventories (count zero; replayed from a synthesized zero because fos does not expose the empty section's position) | 36 | 0 |
| **All of the above (every item reproduced)** | **28,279** | **104,447** |
| Excepted: complete-record decode failure (known) | 3 | |
| Excepted: unresolved holder (known) | 3 | |
| Not testable here: script not built (8,341 in saves whose plugins are not installed, 2 open in Save 138) | 8,343 | 64,070 with the row below |
| Open: untranslated function during load | 1 | |

No record produced different bytes. The script cases are an environment
limit, not a translation result. Seven of the saves list mod plugins that
are not installed on this machine, so their item scripts cannot be built.
For such an item, the script extra loads with no script and no locals, and
saving it passes the null locals to `ScriptLocals::SaveGame` (`005a9db0`,
Xbox PDB) at `00426ce8`. That function reads `[this+0xc]` at `005a9dce`
without a null test, so the original game would fault the same way.

Open items for a later session:

- `00401280`: untranslated. It is reached from the script event handler
  `005ca260` through `005accb0` while loading one Save 138 record (holder
  `090009e9`, a DLC reference with a script extra).
- Two Save 138 records (holders `0200a2f1`, `0200a2fd`) carry a script that
  was not built from the installed plugins, although all ten plugins are
  installed. The cause is unexamined.
- The planned committed engine regression is not done: generated data
  through `004d4160` and `004d4090` with multiple stacks, Worn, and absent
  versus explicit count/health. A generated-memory prototype stopped at the
  map constructor's allocator (`006e20e0` calls `00aa1070`, which reads
  `011f6080`). It needs the partial allocator startup (`00aa2230`,
  `00aa21d0`, `00aa2020`) and the vtable/RTTI words recorded in the private
  dump.
- Scripted items from the modded saves remain untested until those plugins
  are available.

## World inventory stacks (Phase 2 step B), 2026-10-13

`GameState::inventory_stacks` (`world::inventory_stacks`) now holds each
imported holder's saved inventory changes as plain owned data, built from
`fos::inventory_source` envelopes in `world::fos_import`. Per holder: the
change form's version and its items in saved order. Per item
(`ItemChange`, `004bed60`): the saved id, its resolution through the
save's own form-ID array (null, in save, or missing index), the nv-rs
form it maps to, the signed count delta, and its stacks in order. Per
stack: `ExtraCount` (signed 16-bit), `ExtraHealth` as raw bits and
`ExtraWorn`; every other extra keeps its kind and complete original
bytes, uninterpreted. Explicit Count 0 and absent Count stay distinct;
empty stacks are kept. Exe path: `TESObjectREFR::LoadGame_ov2` (`005623d0`,
vtable `0102f5b8`) loads the inventory per item through `004d4160` when
change flag 0x08000020 is set; `SaveGame_ov2` (`00562230`) writes it
through `004d4090` (names Xbox PDB).

The existing views are derived from the stacks on import, with the same
rules as before: `items` is the record's contents plus each delta,
`equipped` gets each worn stack, and `weapon_health` takes the last stack
with health over the weapon's `DATA` health. Gameplay still changes only
those views, so after play the stacks describe the save as imported, not
the live inventory. nv-rs's own saves write the stacks (`invholder`,
`invitem`, `invstack`, `invextra` lines) and read them back exactly.
A holder whose record decodes but has no captured stacks is reported as an
import failure instead of being imported a second way.

Generated-save tests cover a container and the player: three stacks with
explicit Count 0, a signaling-NaN health's bits, opaque Poison and HotKey
bytes, an empty stack, a worn stack, a Cannot-wear opaque extra, a
missing-index item and a null item, through import and nv-rs's own save
and load. A private probe imported the eight original saves used above
with the official load order. For every holder the decoders read
(2,481 to 2,667 per save; 85 to 92 items with several stacks each), the
kept stacks match the decoder's items in order: identity, delta, each
stack's Count, raw Health bits, Worn, and the kinds of the other extras.
Per-item count sums match. nv-rs's own save and load then reproduce the
stacks, counts, equipped items and weapon conditions exactly. No import
reported missing stacks. The three known record decode failures are the
only decode failures, and holders whose plugin is not loaded are dropped
as before. The probe and its output stay private.

Open: one live inventory (the stacks becoming the authority that
gameplay edits, without a second mutable copy); how the item menus show
several stacks of one item. `ContainerMenu::UpdateLists` (`0075c280`)
adds its rows through `00719ef0`; whether that gives one row per stack is
not traced, so the Pip-Boy, trade and container menus still show one row
per item with the old condition rule. Writing a `.fos` from the stacks is
also open: Count, Health and Worn are kept as values, not their original
bytes.

## Script-extra follow-on

The current normal-registry debug and release probes load one original script-bearing
inventory item and write its payload back byte-for-byte. Both real BEGIN
event handlers run once; the reconstructed script, locals and reference
action entry match the assertions, with no parser/evaluator error callbacks.
The fixture uses the original change-form version and complete save-ID
mappings. Its script header and 234 compiled bytes come from SCPT `00169538`
in today's installed `FalloutNV.esm`. The save's historical winning script
version and plugin overrides have not been established. This is an
installed-base-script fixture, not a reconstruction of that historical
plugin environment.

A second normal-registry probe passes in debug and release on an original 34-byte item from a
save whose ten official plugins are all installed. It resolves the script
using the saved plugin order: SCPT `00166652`, with 93 compiled bytes and
three declared variables. Loading creates the three locals with their exact
record IDs, default value bits and flags, calls both real `OnAdd` and
`GameMode` BEGIN handlers once, and reconstructs the expected action entry.
Writing the item reproduces its original payload byte-for-byte. Its saved
variable list is empty, so this verifies initialization from actual SLSD
records, not restoration of nonzero saved values. Current installed files
supply the records; historical file contents have not been verified.

Generated form/reference receivers, partial allocator startup and unresolved
references used only inside skipped bodies remain explicit. There are no
game-function doubles or game-file writes. These successes cover two selected
original items, not the 168,641 entries scanned from eight saves. Three
containing records that fail complete decoding are excluded, as are entries
with unresolved item IDs. Ordinary script-body execution, whole-FOS round
trips and world/viewer integration remain open.

The current dependency batch implements three PC helpers:

| Address | Behavior | Native evidence |
| --- | --- | --- |
| `008d0360` | Return true in AL, without reading the receiver | Six guarded cases, including null/unmapped receivers |
| `0047c850` | Return false in AL, without reading the receiver | Six guarded cases, including null/unmapped receivers |
| `008a81c0` | Read the version byte at receiver +0x1c into AL | Six guarded cases, including high-bit byte values |

All eighteen native cases pass with zero stack cleanup and unchanged
surrounding bytes. The reference vtable and load-buffer call chain confirm
the first two folded predicates; neither mapped unit supplies a reliable
Xbox name for all their shared callers. `BGSLoadFormBuffer::GetVersion`
(Xbox PDB) names the third helper. Independent PC/vtable/caller review agrees.
Four Rust regressions cover these bodies and the real load-buffer virtual
call chain. Final focused tests and all 16,238 workspace tests, including
doctests, pass, along with strict workspace lint, formatting and release/debug
builds. Both original-item probes pass against the final release registry;
the earlier debug runs passed as well. The release probes preserve each
selected original payload byte-for-byte and run both expected BEGIN handlers.

Earlier merged batches established the dependencies below. Each passed
Windows core workspace tests, strict lint, formatting and release/debug
builds before publication; both core and viewer CI gated merging.

| Batch | Implemented paths | Regression/native evidence |
| --- | --- | --- |
| [#135](https://github.com/slaterain/nv-rs/pull/135) | `ScriptLocals` constructor `005a8b80`, `ScriptRunner` constructor `005e0c60`, busy predicate `005e2850`, `ScriptRunManager` instance/action-list dispatch `005e24d0` / `005e2730` (class names from Xbox PDB) | Ten new Rust regressions; ten native cases; 15,876 workspace tests including doctests |
| [#138](https://github.com/slaterain/nv-rs/pull/138) | Action-list entry `005e1160`, parser `005e13a0`, BEGIN/action-list evaluator paths `005e1550`, expression constructors `00593490`, `00594b60`, `00594c60` | Eighteen new regressions; thirty native cases; 16,011 workspace tests including doctests |
| [#140](https://github.com/slaterain/nv-rs/pull/140) | Form-flag predicate `004013e0` and embedded-header accessor `00500940` | Two new regressions; forty-one native cases; 16,062 workspace tests including doctests |

PC instructions confirm constructor sizes 0x14, 0xa4 and eight bytes for
the locals, runner and manager. `CExpression` (Xbox PDB) is 0x710 bytes,
confirmed against its PC constructor/caller. The parser sign-extends opcode
and length words, preserves references for skipped reference prefixes and
consumes 24 stack bytes. Action-list entry preserves busy refusal, fresh
script-size clearing after parser failure, pass termination after evaluator
failure and cleanup order. The evaluator calls the executable's event-table
handler with all eight argument words and preserves its block-skip rules.
Compiler SEH exception cleanup remains partial where marked. Non-BEGIN
ordinary execution/parse modes remain explicitly open and fail loudly.

The metadata scan found 93 unique inventory script IDs, including DLC/mod
indices. A follow-on scan counts every collected instance: 12,479 script
extras, three with nonempty saved variables. All three belong to saves with
missing mod files. Their script `00171475` saves variable IDs 3, 4 and 5,
while the installed base record declares only ID 2; substituting that base
record would not reconstruct the saved script. One official-only save has
all ten listed plugins available, and its records resolve using those exact
saved plugin indices. Historical file contents are still not independently
verified. Next action: finish the current checks and run both optimized
probes before expanding the original-item coverage.
Native executable identity and private evidence location are recorded below;
no raw game data or decompiler exports are committed.

## List boundary

The handle delegates construction, count, bitmap lookup, cached lookup,
insertion, detach and clearing to the existing PC translations
`0040f740`, `0040fe20`, `0040fe80`, `00410220`, `0040ff60`, `00410020` and
`0040fae0`. No new list algorithm is introduced. Insertion retains the
game's special head-insertion types and appends other types. Detach looks
up the predecessor by type, so callers must supply the first matching
node. Duplicate types retain the game's shared-bitmap caveat.

Handles refer to one engine's arena. Copying a handle does not copy its
objects; dropping it does not destroy them. Node construction and lifetime
remain explicit. The adapter exposes detach and forget, not virtual
destruction: subclass destructor dependency closure is not established.

`Engine` currently contains `Rc<RefCell<...>>` test-double state and is not
`Send`, `Sync` or `Clone`. `GameState` has different ownership requirements.
The explicit engine parameter lets this boundary be checked before
changing world/viewer ownership. It does not resolve that integration.

## Dependency evidence, 2026-10-10

Private Ghidra GET queries on port 8090 identified four missing helpers:

| PC address | Behavior | Evidence |
| --- | --- | --- |
| `0040ec80` | Base extra constructor: vtable, type byte, null next | PC disassembly, native generated vectors |
| `004f1540` | Read the type byte, returning AL | PC disassembly, native generated vectors |
| `00403550` | Write the next pointer at +8 | PC disassembly, native generated vectors |
| `0040fc90` | Call the GetCurrentThreadId import at `00fdf1d0` | PC decompilation and disassembly |

`BSExtraData` and its fields are Xbox PDB names. The prototype layout is
0x0c bytes, with the type at +4 and the next pointer at +8; these are checked
against the PC instructions. The engine map does not assign these helpers
a source-unit name, so `extra_data_support` is a dependency module rather
than an asserted original source unit. No executable, symbol database or
raw decompiler output is committed.

Eight generated header cases matched the native executable, including
constructor argument truncation, untouched padding and next-pointer
stores. The executable is FalloutNV.exe 1.4.0.525, unpacked SHA-256
`19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f`.
Private probe inputs and outputs remain in
`%USERPROFILE%/nv-re/work/phase0/codex-audit`.

Integration tests use generated globals and real platform imports, with
no doubles for game functions. They exercise insertion order, the bitmap,
cache changes between lists, removal of head and tail, cleared membership
with a retained old chain, and a real `ExtraNorthRotation` constructor.
The installed-executable probe also passes through the real registry:
construct a list, construct `ExtraNorthRotation`, add, count, find, detach
without deletion, and check empty membership. It maps the user's installed
executable data with `Engine::with_exe`; no game-function doubles or game
file writes are used. This proves that bounded dependency chain, not a
save round trip or original-game gameplay comparison. Full core checks
passed after rebasing onto merged PR #113: 15,154 tests across suites
(13,301 engine tests), clippy with warnings denied, formatting and the
release build. The four ledger-tool tests also pass. A separate read-only
source review found no actionable issues; that review did not independently
verify the private probes. CI still gates merging the PR.

## Open boundary

Payload conversion, reference creation/ownership, arena lifetime, viewer state
ownership and save serialization are not implemented by this step. The
save reader's `Extra::Other` representation discards payloads, so successful
decoding cannot prove preservation of unsupported data. Do not replace the
existing world import or claim a save round trip from these list tests.

## Count payload dependency

`ExtraList::item_count` and `set_item_count` delegate to
`ExtraDataList::GetCount` / `SetCount` (Xbox PDB, PC `00418770` / `00419ad0`).
This is the signed 16-bit `ExtraCount::iCount` at +0x0c, in a 0x10-byte
object; Xbox PDB layout agrees with the PC constructor `00431c20`.
It is distinct from the number of extra nodes and from an inventory
entry's signed 32-bit change against its base record.

The PC setter removes an existing type 0x24 node for counts 0 and 1;
the getter then returns 1. Other values create or update the payload.
The required base destructor `0040ecb0` only resets the vtable to
`01014210`; it does not change the type, link or payload. A generated
16-byte sentinel case matches native output, including untouched bytes.
The native executable identity and private evidence location are as above.

The installed-executable real-registry probe now passes counts 2, 5, -2,
0, 1, 32767 and 1, including removal and reallocation. No game-function
doubles or installation writes are used. The general subclass destruction
boundary remains open; this establishes only the count dependency chain.
The focused ExtraData filter passed 172 tests, including the new destructor
and count lifecycle checks. The full core suite then passed 15,156 tests
on the #113 base, including explicit allocation-release assertions. Count
source is unchanged after rebasing onto #114 plus the list-adapter PR.
The repeat compile exhausted host memory alongside other engine compilers;
it failed before tests. Validation is now serialized. On the merged
list-adapter base, the health follow-on with unchanged count source passed
all 15,315 core tests, clippy, formatting and the release build. Fresh CI
still gates this count PR. The generated ledger is refreshed by the
rolling integration batch after merge, avoiding competing snapshots.

A read-only probe of the user's early Doc Mitchell autosave decoded 3,365
reference/actor change forms and visited 3,319 extra lists. Its 14 count
payloads (values 2 through 40) all adapted and read back correctly through
the real registry, then removed successfully. Lists without a count read
as 1. The save SHA-256 remained unchanged. This checks count payloads only;
other extras are not imported by the probe and no save bytes are written.

Next action: merge after CI, then continue the health payload adapter.
World ownership and full save
round trips remain open.

## Health payload follow-on

`ExtraHealth` (Xbox PDB) is a 0x10-byte object with `fHealth` at +0x0c;
PC constructor `00431d10` confirms the layout. The health adapter delegates
to `004186f0` / `00419970`. An absent node reads the native -1.0 default,
whereas setting -1.0 creates or updates a node; presence remains distinct
from the float value. Zero and negative values are not clamped or removed.

Six generated native constructor vectors preserve signed zero, ordinary
values, padding and the following guard word; signaling NaNs become quiet
through FLD/FSTP. A private Rust registry probe reproduced the existing
constructor mismatch (7f800001 remained signaling instead of 7fc00001).
The constructor now uses the existing x87 bit conversion helper. Regression
tests cover the native vectors and list creation/update, presence, stable
node identity, count preservation and a read that leaves its source bytes
untouched. All 74 health-related tests pass, including these two new
regressions. The installed-executable registry probe now produces the
native quiet-NaN result `7fc00001` for constructor input `7f800001`.

A preliminary real-registry getter/setter probe passes ordinary values,
explicit defaults and signaling NaN handling; the setter already quieted
its argument before reaching the constructor. The early Doc Mitchell save
contains 20 health payloads. A read-only probe adapted all 20 health and
14 count payloads across 3,319 extra lists from 3,365 reference/actor forms.
Values and presence read back correctly, and removing count nodes retained
the health nodes. The source save SHA-256 remained unchanged. No
game-function doubles or save writes were used. On the merged list-adapter
base, all 15,315 core tests (13,462 engine), formatting, workspace clippy
with warnings denied and the release build pass. This also validates the
unchanged count source in the health follow-on. CI still gates merging.
No world/viewer consumer or save writer is added.

## Existing-reference boundary, 2026-10-11

`engine::reference::Reference` adapts an initialized `TESObjectREFR`
(Xbox PDB) in an explicitly supplied engine. Its `extra_data` accessor
dispatches to PC `005d43c0`, which returns the embedded list at +0x44.
Copies alias the same reference. Null is rejected; address validity,
object type and lifetime remain the caller's responsibility. No public
constructor or automatic destructor is added.

The PC reference constructor `0055a2f0` reaches form-header, map, array,
allocator and counter dependencies. The added translations retain their
individual PC addresses. Native generated vectors check five header
cases, two zero-capacity arrays and three allocator-header cases. Generated
tests check allocation requests separately from platform backing size:
requests of 4 and 13 bytes have 8- and 16-byte backing blocks. Callback
regressions verify reloading the form data-handler pointer and map bucket
count after calls that can change them. Compiler exception cleanup remains
explicitly partial in the affected form/reference constructors.

The private installed-executable probe initialized the allocator sequence
from PC `00866690` through `00aa2020`, then ran reference construction,
embedded-list access, count/health updates and count removal through the
real registry. No game-function doubles or game-file writes were used.
This covers that allocator sequence only: the later system-utility startup
has not been reproduced. `Engine::with_exe` maps executable data; it does
not perform that startup by itself.

On the merged #120 base, all 15,587 core tests pass (13,734 engine),
including the new reference alias/isolation regression. Workspace clippy
with warnings denied, formatting, release and debug builds also pass.
The updated installed-executable probe uses the new handle and passes
construction, embedded-list access, count/health changes and count removal
while preserving health. An independent source review found no concrete
issues in the adapter; it did not independently run the private probe.
No world/viewer integration or save round trip is established.

Reference PR #122 passed both core and viewer CI after rebasing onto #121,
and merged as `6bea9a1` on 2026-10-11. No play build was published for this
boundary because the viewer does not consume it yet.

## Buffered extra-data writing, 2026-10-11

The follow-on translates 18 constructor, accessor and writer dependencies
of the existing `00426a30` extra-data writer. `BGSSaveGameBuffer` (Xbox PDB)
has PC buffer/capacity/position/overhead fields at +4/+8/+0xc/+0x10.
`BGSSaveFormBuffer` (Xbox PDB) adds its nine-byte packed header at +0x14 and
form pointer at +0x20; construction clears nine header bytes and preserves
the three alignment bytes before the pointer. Six generated native
constructor vectors confirm the stores, vtables, return pointer and
surrounding guards. Exception cleanup
in `008659c0` remains explicitly partial.

Positioned writing (`00865ce0`) grows by 0x10000, appends the native '|'
separator unless suppressed, and updates the position and debug overhead.
It uses unsigned capacity comparisons; insertion (`00866140`) uses signed
comparisons and retains its original end across callbacks. Tests exercise
field/global changes across callbacks and wrapping arithmetic. Variable
counts (`00865f20`/`00865ff0`) patch a placeholder using one, two or four
tagged bytes. The checked move (`00ec7ce3`) uses the existing CRT memmove
boundary; three native cases pass for zero length/null pointers and overlap
in both directions. Error paths retain native TLS/error-handler calls;
these callees are unresolved and their unit tests use doubles.

The PC change-flag table at `01183dc0` and `01183dc4` uses mask `0x400` for
count and health, not the generic mask used by older synthetic writer
fixtures. A new generated regression prepares both payloads through the
real registry and exercises the buffered writer with matching and filtered
change flags. The installed-executable probe passes with a generated
save-mode receiver and the previously tested allocator startup fragment.
It writes count 5 and health 42.5 through the real registry and native
change-flag table, then decodes the resulting 14 bytes as exactly those
two extras with no unread bytes. No game-function doubles or game-file
writes are used. This checks a payload, not a complete FOS file.

On the merged #123 base, all 15,641 core tests pass (13,788 engine),
including the generated writer regression. Workspace clippy with warnings
denied, formatting, release and debug build checks pass.
No FOS file writing, full save-manager startup, save round trip or viewer
integration is established.

Next action: publish the bounded writer batch,
then trace per-stack state ownership before adding a world consumer.

## Inventory payload dependency chain, 2026-10-11

`ItemChange` (Xbox PDB) is a 0x0c-byte object: the extra-list chain at +0,
signed item-count delta at +4, and item form at +8. Its PC buffered writer
`004bed60` writes the form ID, signed delta, number of non-null stacks and
each stack's extra list separately. These counts must not be collapsed:
the entry delta is against its base record, whereas an `ExtraCount` is the
signed 16-bit count in one stack. The current world importer merges item
counts and overwrites per-item weapon condition as it visits the stacks;
this follow-on does not change that importer.

Seventeen dependencies are implemented for this writer: form-ID encoding,
the forward/reverse save-ID maps, map construction and virtual allocation,
lookup/insertion/release, and the list-node constructor needed to retain
multiple stacks, and the shared word-at-four getter. `BGSSaveLoadFormIDMap`
(Xbox PDB) has maps at +0/+0x10 and
the next index at +0x20, size 0x24, checked against the PC constructors.
Constructor `00846b60` still records omitted compiler exception cleanup.

Ten generated cases match the native executable: three created-form-ID
decodes with unchanged surrounding bytes, four key-equality AL results,
and three word-at-four results with unchanged surrounding bytes.
The native cases use the same executable hash and CPU configuration cited
above. Static peer review of all seventeen bodies found no concrete mismatch;
the list-node constructor's review also checked the aliased-source regression
against its PC load-before-store order and one-word stack argument.
Generated tests cover callback changes, collisions, index reuse and the
real registry's encode/decode chain. The first 16-helper batch passed all
15,690 core tests (13,837 engine), strict workspace lint, formatting and
release/debug builds. The installed-executable probe then reached the
untranslated `00726070` getter while advancing to the second stack. After
adding that dependency, the real-registry probe passes: its 41-byte payload
decodes to entry delta -2, count 2 / health 42.5 in one stack, and count 7 /
health 17.5 in the other. The save-ID map resolves the encoded item back to
its original generated form ID, with no unread payload bytes. Generated form
header, manager/mode receivers and partial allocator startup are explicit;
no game-function doubles or game-file writes are used. Final checks with all
seventeen dependencies pass: 15,691 core tests (13,838 engine), strict
workspace lint, formatting, and release/debug builds.
No FOS file, world consumer or full save round trip is implemented.

Next action: validate native buffer loading before connecting the state
boundary to the world.

## Native inventory buffer reader, 2026-10-11

Eleven reader dependencies are implemented in `bgsloadgamebuffer.rs` and
`bgsloadformbuffer.rs`: buffer construction/attachment, raw reads, variable
counts, form-ID decoding, and form/reference/actor access. The PC raw reader
does not add a bounds check: it skips only when the size field is zero and
reloads its buffer and cursor after callbacks. Tag three leaves the cursor
unchanged; the other tags decode one, two or four bytes and consume a
separator. The derived constructor retains its three header-padding bytes;
its compiler exception cleanup remains explicitly partial.

Peer review against PC disassembly found no mismatch. Ten generated native
cases pass on the same executable/CPU configuration above: two guarded
constructors, four flag-predicate AL results, and four variable-count tags
with cursor checks. Eight focused regressions and all 15,771 workspace
tests (13,918 engine) pass on merged #128, as do strict workspace lint,
formatting, release and debug builds.

The installed-executable probe writes and loads two independent stacks:
count 2 / health 42.5 and count 7 / health 17.5, with signed entry delta -2.
The translated buffered reader consumes all 41 bytes. This uses real
registered translations, generated receivers and partial allocator startup,
without game-function doubles or game-file writes.

The fuller ItemChange load/write probe stops before loading: inserting its
generated form into the real form map reaches untranslated allocator
`0043a010`. Its byte-identical rewrite assertion has not run. Static review
of `004bee00` confirms the existing normal loader path; compiler exception
cleanup remains omitted there. Neither probe establishes a complete FOS
file round trip or a world/viewer consumer.

Next action: translate the form-map allocation dependency and rerun the
full ItemChange load/write probe before connecting world state.

## Form-map allocation and buffer setter follow-on, 2026-10-11

Two dependencies are implemented in `fallout_shared/form_map_support.rs`:
`0043a010` removes and clears a twelve-byte entry under the native lock;
`0043a080` refills the free list and chains its allocated blocks. The map's
renderer template name (Xbox PDB) is not used to infer behavior. PC review
confirms overflow saturation, post-allocation count reloads, the unsigned
do/while bound and store order. Three regressions pass, including actual
pool exhaustion/refill, lock callbacks and saturated allocation size.

The broader inventory probe also reaches shared setter `007037c0`, now
implemented in its mapped `bgsmenupacker.rs` unit. The PC body only stores
one word at receiver +0x24; inventory loading uses it as the form-buffer
setter. The Xbox PDB unit/name does not establish its behavior for all the
shared callers. Independent PC review and three generated native cases
confirm the store, unchanged neighboring bytes and four-byte callee stack
cleanup, using the executable hash and CPU configuration above. One
generated registry regression covers the setter.

A private probe first checked the two draft bodies separately, then passed
again through the final engine build's normal registry. It exercises full
`ItemChange::LoadGame_ov2`, real form-map lookup and installed RTTI: the
form pointer, signed delta and both count/health stacks are reconstructed,
and writing the loaded item reproduces the exact 41-byte payload. This
uses generated form/map/mode receivers and partial allocator startup,
without game-function doubles.
There is still no complete FOS round trip or world/viewer consumer.

A second private probe reads eight existing saves without modifying
them. It extracts inventory entries only after the existing decoder reads
their whole containing reference/actor record successfully. Of the entries
containing only count/health extras, all 10,201 load and rewrite with
byte-identical item payloads, including eighteen entries with multiple
stacks. Three reference/actor records that fail full decoding are explicitly
excluded. Original form-ID indices are retained; generated bound-object
headers supply the form/RTTI boundary. This remains a payload check, using
the separately compiled map draft, not a full FOS or playable-world round
trip. It also passes through the final build's normal registry. Save hashes
and probe output remain private.

Expanding the private probe beyond count/health reaches missing constructor
`005a8b80` while loading script extras. The probe's generated forms and ID
maps must also be extended before claiming coverage of referenced scripts.
The wider set has not passed.

On merged #130, all 15,781 workspace tests pass (13,928 engine), as do strict
workspace lint, formatting and release/debug builds. Both final registry
probes pass. These results do not establish a playable inventory consumer.

Next action: publish this bounded batch, then supply the script-form boundary
and translate the script-local constructor needed by the broader probe.
