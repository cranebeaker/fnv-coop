# ADR-0016: Script extenders built in

**Status:** Accepted (lead, 2026-11-07, from the design's own recommendations; the maintainer said
"start it now", 2026-11-07)
**Date:** 2026-11-07

## Context

Mods for Fallout: New Vegas are compiled against the xNVSE script extender and its plugins, most of
all JIP LN NVSE. Their compiled scripts (`SCDA`) call commands at opcodes the exe does not have:
xNVSE's 654 from `0x1400`, JIP LN's 1,118 at `0x2200` (the former lutana plugin's block, 299) and
`0x2600` (JIP's own, 819; 32 empty slots). FalloutNV.exe 1.4.0.525 without the DLLs answers "unknown
command" and stops the script; nv-rs does the same today. The maintainer wants both built in, with
nothing to install (the toggleable-fixes goal, 2026-11-01), and every extender author gave
permission to translate their code into nv-rs, credited, with their notices kept, JIP's GPL code
and the former lutana plugin's part included (maintainer, 2026-11-03).

What the port has: the exe's compiled-script interpreter, translated. `ScriptRunner::Run`
(`005e0d20`) keeps the line cursor, length and skip count in guest words and calls
`EvaluateLine` (`005e1550`) by address; `EvaluateLine` gets a command's record from
`ScriptCompiler::GetFunctionDef` (`005b1120`) and calls the handler word at `+0x18` by address with
the eight words `parameters, data, reference, container, script, locals, result, cursor`, which are
exactly xNVSE's `COMMAND_ARGS`. Commands inside `if`/`set` go through `CExpression::Eval`
(`00593db0`) and the operand reader (`005ac7a0`); arguments are read by `Script::ParseParameters`
(`005accb0`).

What the extenders do to the exe (xNVSE 6.4.9, `0ccd23a`; JIP LN NVSE 57.30, `5a30ac4`): xNVSE copies
the script command table (`0x01190910`, 0x280 records of 0x28 bytes) into its own memory, pads it to
`0x1400`, appends its commands and its plugins', and writes the new numbers over 19 immediates in 13
exe functions (`CommandTable.cpp:68-124,376-435`). It detours function entries (the script context
around `Run`, its own argument reader at `005accb0`) and patches instructions inside functions (the
main-loop tick at `0086B386`, the co-save sites, the evaluator call at `005949D4`). JIP is xNVSE's
plugin: it reads arguments, makes strings and arrays and calls scripts through xNVSE's plugin
interface, and adds its own always-on game patches.

The design behind this record (planning notes beside the lead's tools, 2026-11-03, after an
independent review) has an inventory of every command (what it reads, calls and needs), the exe
addresses each infrastructure step touches, checked in Ghidra, and the build order below.

## Decision

### 1. Crates

- `crates/nvse`: xNVSE built in: the command table, the script context, argument reading, the
  expression evaluator, string and array variables, loops, user functions and lambdas, events,
  delayed calls, the co-save, the version answers, the run-time compiler's extensions, the Rust
  plugin API, and the 654 commands (`src/commands/<area>.rs`, one per xNVSE `Commands_*.cpp`).
  Its name table (the 654 names in opcode order) lives there: the world crate that held it is gone.
- `crates/jip`: JIP LN NVSE built in, after xNVSE's core (its I-13 step); it depends on `nvse`.
- Both depend on `engine-core` only. They call game functions by address and declare the layouts
  they read with `layout!`, each field checked against the exe and the Xbox PDB.
- `host::Host::start_with` installs them after `play_patch::funcs()` and the fix rows, so a wrapper
  wraps whatever is registered at its address (the translation, or a row that is on).
  `Engine::new` never installs them: the engine crates' own tests stay the exe.

### 2. The command table in guest memory, and the immediates (option A)

- One script table in guest memory, as xNVSE builds it: `0x1000..0x127f` copied from the exe's
  table, xNVSE's padding command up to `0x1400` (empty names, "command used for padding", returns
  true), xNVSE's commands from `0x1400`, JIP's at `0x2200` and `0x2600` when JIP is built in. An
  opcode past the end stays unknown, so a mod that needs a plugin nv-rs does not have fails at that
  call, as without the DLL. A command not built yet has its record (its name, so the log names it)
  with no handler: the line fails and the script stops for that run, as it does without the DLL.
  The console table stays where it is.
- **Option A:** each of the immediates xNVSE patches is a read of one engine-owned descriptor,
  `engine_core::extend::ScriptTables { start, end, max_opcode }`, whose values are the exe's until an
  extender moves the table. The code is unchanged and only the numbers move; a test pins every read
  to the exe's immediate with nothing installed. `GetFunction` (`005b1910`) and `HelpFunction`
  (`005bc950`) keep their two immediates (`005B19A3`, `005BCBE3`) until their seams exist (I-14):
  xNVSE jumps out of both before their table loops, and until then the console takes vanilla
  commands only.
- Records live in the built-ins' data range (section 4). Return types are kept outside the
  record, by opcode, as xNVSE keeps them.

### 3. Wraps and seams (accepted, with the wrap chain)

- `engine-core` has a wrap chain: `Engine::wrap(slot, f)` puts a plain function in front of what is
  registered at an exe address, `Engine::call_next` runs the next link as part of the same call
  (one entry on the call stack, the call log, the counts and the trace). Every wrappable address and
  its owners, in order (rows, `nvse`, `jip`), are a static table (`WRAPS`); an unlisted place is a
  build error.
- Seams: one line at the hooked instruction inside a translation, `e.site(addr, &[words])`, backed
  by a static table of sites and owners (`SITES`), rows first, then `nvse`, then `jip`; with nothing
  registered it is one test. A seam is a reviewed one-line edit with a test that the vanilla path is
  unchanged.
- Fix rows whose patch is inside a large function register at a seam at the patch (question 14);
  small functions get a copy at the entry, as ADR-0010's rows do.

### 4. Where built-ins' guest data and code addresses live

The heap already never enters the platform's area between its two ranges
(`mem::PLATFORM_AREA`, `0x7E000000..0x80000000`), and no `VirtualAlloc` at a chosen address lands
there. The extenders take its top: `mem::EXTENDER_CODE` (`0x7E800000..0x7F000000`: handlers at
`0x7E800000 + 4 * index`, evals at `0x7EC00000 + 4 * index`, named routines from `0x7EFF0000`) and
`mem::BUILTIN_DATA` (`0x7F000000..0x80000000`: tables, strings, parameter lists). No heap address
moves. `profile::kind_of` names the extender ranges.

### 5. The per-opcode rule (question 13)

Every change xNVSE or JIP makes that reaches vanilla content is split by the opcode of the command
it acts on:

- for an **extender opcode** (`0x1400` up) it is part of the extender, in every preset: argument
  reading (xNVSE's reader, wrapped over `005accb0`), the evaluator call at `005949D4`, the result
  zeroed before a handler runs (`005ACB9D`, `005E16C3`, `005E2329`);
- for a **vanilla opcode** it is a fix row, on in `fixes`, off under `vanilla`: `xnvse.extract_args`,
  `xnvse.expression_command_call`, xNVSE's `GetFactionRank`/`ModFactionRank` handlers and the
  ShowRouletteMenu short name, JIP's replacements of vanilla commands and of `0x127D`, JIP's result
  zeroing for vanilla opcodes, JIP's always-on game patches and its `[GamePatches]` switches.

Under `vanilla`, a mod's vanilla commands that use xNVSE's argument encodings fail as in the exe
without xNVSE; its extender commands work.

### 6. Everything else, in short

- **Script context:** a context stack per game thread, pushed and popped by wraps at `005e0d20` and
  `005e1160` exactly where xNVSE does (the push in `Run` unconditional, the pop only with a script);
  the current line recorded by a wrap at `005e1550`, with the pointer to `Run`'s `length` word, so
  `Break`, `Continue`, `Loop` and `Goto` move the next line as xNVSE moves the exe's frame word.
- **Strings and arrays:** extender state, host side, ids allocated as xNVSE allocates them so
  co-saves load both ways.
- **Expressions, loops, functions, lambdas, events, delayed calls, the tick, the co-save, input,
  inventory references, the run-time compiler:** as xNVSE does them, at its sites, each site a
  wrap or a seam.
- **Threads:** commands and handlers run on the main game thread; an event raised on another thread
  is queued for the tick, as xNVSE does.
- **Numbers:** commands compute as the plugins were compiled (SSE2 doubles); out-of-range
  conversions through a helper, never `as`.

### 7. Marking, credits and notices

- Names, layouts and formats from xNVSE carry `(xNVSE)` at their first use in a file; from JIP
  `(JIP LN NVSE)`. A translated function carries `Translated from xNVSE 6.4.9 (0ccd23a), <file>:
  <function>.` and, where it replaces an exe function, the exe address.
- Each crate's `NOTICE` credits the authors as their READMEs do: the original NVSE (Ian Patterson,
  Stephen Abel, Paul Connelly, Hugues LE PORS), xNVSE's developers (korri123, cnf13, jazzisparis,
  Demorome) and contributors; JIP LN NVSE's author (jazzisparis) and the former lutana plugin's
  author; JIP's code keeps its GPL-3.0 notice; `common/` keeps its zlib-style notice (Ian
  Patterson). Third-party code inside
  xNVSE keeps its own licence file beside its translation (the MT19937 generator, BSD 3-clause;
  Algohol, MIT); nothing is taken from `xxhash64.h`.
- No C++ is committed.

### Build order

| Step | What |
| --- | --- |
| I-1 | Extension slots, the wrap chain, the seam table, the extender ranges |
| I-2 | The script table in guest memory, the descriptor reads, `GetFunctionDef`, names |
| I-3 | The script context (wraps at `005e0d20`, `005e1160`, `005e13a0`, `005e1550`) |
| I-4 | xNVSE's argument reader for extender opcodes, results, the zeroing seams |
| I-5 | String and array variables, the tick seam, format strings |
| I-6 | The expression evaluator, `Let`, `eval`, inline expressions |
| I-7 | Loops, labels, user functions, lambdas, `CallFunction` |
| I-8 | The rest of the tick, delayed calls, present notifications, exits |
| I-9 | Events |
| I-10 | The co-save |
| I-11 | Input |
| I-12 | Inventory references and form extra data |
| I-13 | JIP's core |
| I-14 | The run-time compiler and the console |
| I-15 | The fix rows (parallel, ADR-0010's track) |

Command batches follow the steps they need, most-used first: after I-4, xNVSE's inventory, weapon,
math, Algohol, package, faction and quest getters and setters.

## The design's questions, answered

1. **Permission:** granted by every author (maintainer, 2026-11-03), JIP's GPL code, the former
   lutana plugin's part and NVSE's contributors included; credits and notices kept (section 7).
   I-14's script data cache uses an XXH64 implementation under its own licence.
2. **The 19 immediates:** option A, descriptor reads (section 2).
3. **Seams in translations:** accepted, with the wrap chain (section 3).
4. **Built-ins' guest data:** the reserved block, the top of the platform's area that the heap never
   enters (section 4). On today's `main` that block already exists, so `HEAP_END` does not move.
5. **Verification runs install the extenders:** yes; `Host::start` installs them like
   `Host::start_with`. Vanilla content never calls an extender opcode, and the per-opcode rule keeps
   every change to vanilla commands a row, off under `vanilla`.
6. **The xNVSE version answered:** 6.4.9 (packed `0x06040090`), the source translated.
7. **JIP "installed" under `vanilla`:** yes: its commands are there in every preset, so
   `IsPluginInstalled` answers installed; JIP's fixes stay rows.
8. **xNVSE's engine fixes that commands rely on:** go with the extender in every preset (split by
   opcode where they can be); each such row records why.
9. **Test inputs:** planned: a scratch copy of the game with xNVSE 6.4.9 and JIP 57.30 (never the
   player's own), test plugins compiled there, logs compared line by line with nv-rs; the lead asks
   the maintainer about downloading the two for that copy.
10. **A mods overlay:** exists already (`play.sh`'s Play with mods: a scratch install).
11. **The world crate's interpreter:** gone; extender scripts run on the engine path only.
12. **Numbering:** this record, ADR-0016.
13. **The per-opcode rule:** accepted (section 5).
14. **Fix rows inside large functions:** registered at a seam at the patch (section 3).
15. **The co-save under `vanilla`:** written beside every save under every preset, as xNVSE does,
    since the commands are there in every preset and a stale co-save must not be read with a newer
    save.

## Consequences

- Mods' compiled scripts call the built-in commands at their opcodes, with nothing installed; a
  mod that needs a plugin nv-rs does not have fails at that call, as without the DLL.
- Vanilla content runs as before under `vanilla`: the table's vanilla records, the descriptor reads
  and every seam are pinned to the exe by tests, and every change to how vanilla opcodes behave is
  a row.
- Engine crates change only by the descriptor reads, one-line seams and the `engine-core`
  additions (extension slots, the wrap chain, the seam table).
- The Rust plugin API is the base for later built-ins (JohnnyGuitar, ShowOff, kNVSE), which
  register their opcode ranges the same way.
