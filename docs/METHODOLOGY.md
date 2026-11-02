# Decompilation and rewrite methodology

Written 2026-10-05. This is the recommended method for reading `FalloutNV.exe` and reimplementing it faithfully in Rust and Bevy. The tools for its first steps are in `research/`; see "Tools in this repository" at the end. This is not legal advice.

## Evidence limits

This document was researched in a cloud container that had no game files, no FalloutNV.exe and no Ghidra project. Nothing here was measured on the binary. Every number about the exe comes from public community sources or the repo's own docs, or is marked as an estimate.

The container's egress proxy blocked several primary sources: Hidden Palace, Microsoft Learn, arXiv full texts, Nexus, Steam community, decomp.dev, wiki.multimedia.cx, facegen.com, bevy.org and developers.openai.com. Some legal primary sources (cornell.edu, justia, wikipedia) were blocked during the final checking pass. Claims that rest on blocked sources are marked uncertain.

The local measurements this method needs are listed as tasks for the maintainer's machine:
- function count
- toolchain
- FPU flags
- cold `analyzeHeadless` cost
- the FNVIntro.bik header tag

## Bottom line

- **Do not decompile the whole exe and then port it.** The fastest faithful route is decompilation-informed behavioural reimplementation:
  - Ghidra supplies the specification.
  - Original Rust implements it.
  - Mechanical oracles built from the original game show that the behaviour matches.
  - Here "1:1" means the same outputs, constants, branch conditions and operation order. It does not mean a function-by-function port.
- **Stay on Ghidra 12.1.4 [1].** The time is lost in the workflow around Ghidra, not in the decompiler. Replace cold `analyzeHeadless` runs for each query, and the per-agent project copies, with two things: one shared, annotated database and a persistent read-only query service. Measure the cold-start cost first so the gain is known.
- **Name the binary from the exe's own self-description before reading more code:**
  - its MSVC RTTI (1,749 classes in xNVSE's list [2]) and NiRTTI
  - its command, settings and form tables
- **Use community PC addresses (xNVSE, JIP LN, JohnnyGuitar, BethesdaGhidraScripts) only as pinned, tagged hypotheses. The community pool is not clean:**
  - BethesdaGhidraScripts (BGS) v1.2.2 emits Xbox-PDB-derived names at default settings, and no flag turns them off [86].
  - JohnnyGuitar has added many PDB-matching names since the November 2025 leak [88].
  - Import from pinned pre-leak commits, with PDB-derived files removed.
  - Commit a name only when it is corroborated by PC evidence.
- **Replace exports parsed from C text** with an export that is complete by construction. Generate it from Ghidra's function list, with a manifest that checks the counts. gamedb's silent miss of 005cc4f0 shows why [3].
- **Build oracles before scaling agents, in this order:**
  1. an in-process Rust probe DLL for the running game
  2. function-level input/output tables: Unicorn for integer and plain float code, and a native 32-bit harness for SSE approximations and transcendentals
  3. WinDbg TTD for short deep dives
  4. apitrace frame and per-draw images for shaders, plus scof
- **Separate two lanes, and enforce the separation with the operating system, not tool permission rules.**
  - In the research lane, agents read Ghidra and write maths-only specs privately.
  - Implementation-lane agents run in cloud sessions or under a separate non-admin Windows user, so the private tree is unreachable by every route [92][93][95].
  - Accept work only when the oracle tests pass.
- **Keep Bevy as a thin adapter.**
  - Simulation rules live in headless core crates.
  - Bevy drives them from a single-threaded schedule with ambiguity detection on, fed a GetTickCount-style quantised delta through `TimeUpdateStrategy::ManualDuration` [100].
  - To unblock the M1 save gate for mid-playback audio, either add bevy_kira_audio 0.23, which works with Bevy 0.16, or migrate to Bevy 0.19.1 [103][101].
- **Scope work by route.** Trace what M1 actually executes and work through that list. Do not aim for uniform coverage of about 42k first-party functions.
- **Keep the Xbox 360 prototype PDBs, and anything derived from them, out of the project** unless the user changes the policy after weighing the legal risk. Consider analysing a DRM-free GOG 1.4.0.525 exe instead of the Steamless-unpacked Steam exe.

## Scale and what "fastest" means

**Size.** No public source gives the PC function count.
- One headless run reports 16,040 functions for the unpacked Steam exe [4]. That is probably too low.
- BethesdaGhidraScripts ships a list of 18,663 known PC function-start addresses [5]. The file holds addresses only, with no names, and it is stale. Re-running its generator on the shipped refs gives 19,805 addresses: 12,125 from vtables and 7,680 from NVSE. PDB data adds none of them [86].

The best sizing proxy is a symbolized Xbox 360 build used by a public matching-decomp project [6]:
- 69,012 functions in about 16.3 MB of PowerPC code
- about 42.5k first-party functions (Bethesda, Obsidian, Gamebryo, Creation)
- about 46% of the code is middleware or platform libraries: Havok 13%, Gamebryo core 13%, XDK/CRT/D3DX 17%
- about 54% is game code

The PC .text section is about 12.4 MB. A PC count of 45k-65k functions is an unverified inference and should be measured.

**Known timelines.**
- **Matching decompilation** runs at roughly 1k-3k functions per year, even with large teams. Twilight Princess took about 5.3 years and 81 authors for 16,405 functions [7]. At that rate a New Vegas matching decomp would take 13-22 team-years, and the result would be C++, not Rust.
- **Clean-room engines built only from data** took 9-18+ years or stalled. OpenMW has no 1.0 after 18 years [8]. OpenXcom shows that a smaller, well-documented game can be done in about 4.4 years [9].
- **Projects with a naming accelerator and a mechanical check** shipped in months to about 2 years.
  - re3 reached a standalone exe in about 2 years by replacing functions inside the running original [10].
  - OpenRCT2 stopped needing the original exe after about 2.5 years [11].

**Agent throughput.**
- Agent-driven N64 matching reached about 25 functions a day using 4 worktrees and a byte-exact judge. It still needed human experts for a tail of about 5% [12].
- Snowboard Kids 2 decompile commits went from tens per month to 1,311 in November 2025 once agents were used [13].

Without an execution oracle, LLM output is wrong too often:
- The best LLM4Decompile model re-executes correctly 64.9% of the time on small GCC functions [14].
- The best single-shot CRUST-bench result was 15/100 [15].
- A September 2026 study found that LLM refinement raised Ghidra's build rate from 75% to 90%, while behavioural match fell from 74% to 62% [16]. This comes from a secondary summary, because arXiv was blocked.

**What "fastest" means here.** The fastest route has the shortest loop from "unknown behaviour" to "verified Rust". It is not the route that decompiles the most code. Three things shorten that loop:
- names, to find the right function in minutes
- oracles, to know the answer is right without a play session
- scoping, to do only what the milestone route runs

**Not achievable quickly:**
- A literal port of every function. There are too many, and the result would likely be a derivative work.
- Bit-exact Havok solver behaviour (ragdolls, debris, constraint solving). Reproduce the gameplay contract instead.
- Bit-exact results for SSE approximations across CPU vendors.
- Bit-exact pixels against D3D9. Rasterisation conventions, MSAA resolve, LOD selection and FP16 precision differ, so shader acceptance uses tolerances (step 8).
- Loading original .fos saves. This is separate scope; xEdit publishes a partial schema [17].
- A full campaign with M3-level evidence in less than several years.

## Approaches compared

| Approach | Speed | Fidelity | Legal risk | Fit with Rust/Bevy | Verdict |
| --- | --- | --- | --- | --- | --- |
| Data-driven clean-room reimplementation (no binary reading) | Slowest: 9-18+ years in precedents [8] | Approximate where rules are not visible in data | Lowest | Good | Use only as the verifier and for formats; too imprecise for IK maths and timing |
| Non-matching decomp, then port function by function | Medium | High if every function is checked | High: a Rust translation of decompiled code is likely a derivative work [18][19] | Poor: mirrors a C++/Gamebryo object model | Rejected as stated; keep the reading, drop the transliteration |
| Matching decompilation | 13-22 team-years [7] | Byte-exact, but in C++ | High; survives only by rights-holder tolerance | None; still needs a rewrite | Rejected |
| Static recompilation | Fast for consoles (Unleashed Recompiled: about 5 months [20]) | Exact, but unreadable | High | None; output is machine-translated C++ | Rejected; would only cover the Xbox 360 build |
| LLM transliteration of Ghidra C | Fast to produce | Low without an oracle [14][15][16] | Highest | Superficial | Rejected; LLMs may propose names and types only |
| **Decompilation-informed behavioural reimplementation with oracles** | Months per milestone (estimate) | Verified per function and per route | Low to moderate if the lanes stay separate and the separation is OS-enforced | Headless core crates with a thin Bevy adapter | **Recommended** |

## The recommended pipeline

### 1. Reproducible binary identity and private workspace

**Inputs.** The retail exe. Today that is the Steamless-unpacked Steam 1.4.0.525 exe (SHA-256 19406942...739F) [21].

**Steps.**
- Record the exe identity once:
  - SHA-256 of the original and the unpacked exe
  - RSDS GUID/age/path [22]
  - Rich header, LAA bit and section table
- Consider a DRM-free GOG 1.4.0.525 exe.
  - xNVSE treats GOG and Steam 1.4.0.525 as the same version, with identical hook addresses [22][23].
  - Analysing the GOG exe avoids the Steamless step, which is plausibly circumvention under 17 USC 1201 [24][25].
  - Confirm privately that the section hashes match and that 50+ known addresses line up.
- Measure the open toolchain facts in one session (about 0.5-1 day):
  - the Ghidra function count
  - the x87 vs SSE instruction mix (the existing FindInstr script)
  - the `IDirect3D9::CreateDevice` BehaviorFlags (FPU_PRESERVE is 0x2), readable from an existing apitrace. Without FPU_PRESERVE, Direct3D sets the creating thread's x87 unit to single precision [26].
  - whether the CRT is linked statically or dynamically
  - the main-loop time source
  - the cold cost of one `analyzeHeadless` query: `Measure-Command { analyzeHeadless <proj> <name> -process FalloutNV.exe -noanalysis -readOnly -postScript Decompile.java 0x00c755e0 }`
  - the FNVIntro.bik revision: `Format-Hex FNVIntro.bik -Count 4`
- Build a private "oracle install": a copied or hard-linked game tree that holds NVSE, injectors and the LAA flag.
  - The current apitrace workflow copies d3d9.dll into the game folder. That conflicts with "never modify the user's installation" [27][28].
  - Launching the exe directly currently gives a Steam error [28], so test `SteamAppId=22380` first.

**Output.** An `exe-identity` record in the private tree, and a one-line build constant in the repo.

**Gate.** Every later artifact carries the exe SHA-256.

### 2. Naming and type bootstrap

All of this happens in a copy of the private Ghidra project. Every imported label gets two tags:
- a source tag: `src:rtti`, `src:xnvse`, `src:jip`, `src:jg`, `src:bgs`, `src:llm` or `own`
- a pin tag holding the source repo's commit hash

1. **Self-description from the exe** (1.5-3 days). This tier is the only one that cannot carry PDB taint, so build vtable labels here rather than importing BGS's.
   - Run the RTTI analyzer and RecoverClassesFromRTTIScript. Run them on a copy, because the script has thrown exceptions on real binaries [29].
   - Label every vtable slot `Class::vfNN`.
   - Name the 495 NiRTTI classes (registry at 00a7b610 [28]).
   - Label the command tables. They use 40-byte entries: the script table at 0x01190910 holds 640 slots (opcodes 0x1000-0x127F), and the console table at 0x0118E8E0 holds 206 entries [30]. The repo's "48-byte entries" note conflicts with this. The console table's 206 entries end exactly where the script table starts: 0x01190910 - 0x0118E8E0 = 0x2030 bytes = 206 x 40. Recheck it in Ghidra [28][30].
   - Label the settings globals and the form-type, extra-data, actor-value and anim-group tables.
   - Tag each function with the .cpp names in its assert strings.
2. **Community PC facts** (2-4 days). Treat these as hypotheses with known taint risk.

   Sources and pins (the last commit on the default branch before 2025-11-26) [88][89][90][91]:

   | Source | What it provides | Licence | Pin |
   | --- | --- | --- | --- |
   | xNVSE | 1,749 RTTI addresses and about 2.6k address constants | no licence file [2][23] | `fb0f4e95ac46c7cf49b2c68b2d222b362ea250ef` (2025-10-13) |
   | JIP LN NVSE | 1,842 vtable enums, 3,785 GMST and 1,157 INI setting addresses | GPL-3.0 [31] | not checked; pin likewise |
   | JohnnyGuitar | 2,859 `GAME - 0x` annotations, 2,335 typed call wrappers, 720 size asserts | LGPL-2.1 [32] | `e9c9d22d09e86a95ba18518b17c8b56974a241f6` (2025-11-24) |
   | kNVSE | addresses and names | not recorded | `dbf8713d908eb10c35089dafbdfbbc4d575fed20` (2025-05-27) |
   | yUI | addresses and names | not recorded | `0e17b44cd08e52cb812d81736765746f6c3bb692` (2025-08-19); `dff50dc` was merged after the cutoff |

   **Why pin.** In the 486 JohnnyGuitar commits after the pin, 1,324 of 5,312 added qualified names (24.9%) match the Xbox `symbols.txt` [87]. The pinned tree's own baseline is 4.1%. For the other repos the post-pin additions are small: kNVSE 16, yUI 7, xNVSE 1.

   **What pinning does not do.** Every pinned tree still holds 0.7-4.1% of names that match. Matching does not prove PDB origin, because PC RTTI and strings can yield the same names. So pinning reduces post-leak additions but does not prove an import is PDB-free [86].

   How to import BethesdaGhidraScripts (BGS) v1.2.2 (commit `dabb9191`) [86]:
   - **Remove its PDB-derived inputs first, because no flag excludes them.** At default settings `build_fallback_symbols()` returns 9,406 entries. 458 of them are PDB-derived or PDB-assisted: 362 `xbox_pdb_matched` functions, 85 `string_anchor` and 11 `thunk_jmp`.
     - Delete `fnv_pdb_matched_classes.txt`, `fnv_thunk_names.csv` and `fnv_string_anchored.csv` from the private copy; each loader returns nothing when its file is missing.
     - Leave `BGS_FNV_ARTIFACTS` unset and `artifacts/fnv` empty, because `Fallout_Debug_*.json` there attaches PDB signatures, locals and structs.
     - Never set `BGS_FNV_INCLUDE_EXPERIMENTAL`.
     - Pin the commit. Three loaders (`xbox_vtable`, `string_xref`, `global_label`) are empty today only because of missing schema or `# EVIDENCE=` headers. A refreshed refs file would switch them on silently.
     - `parse_commonlib_types.py` calls the fallback symbol builder unconditionally, despite the README.
   - **Do not import BGS's 1,479 `pc_vtable` labels.** Their class names agree with PC RTTI, but the script that generated them is not in the repo. Tier 1 rebuilds them from the exe.
   - **Use BGS's libclang path for xNVSE types,** because Ghidra's own C parser fails on C++ templates [33]. BGS only renames `FUN_*` names [34]. BGS was developed on Ghidra 12.0.4, so check it against 12.1.4.
   - **Normalise addresses before import.** BGS uses RVAs, while xNVSE, JIP and JG use absolute VAs [5].
   - **Add a small extractor** that pairs wrapper method names with their `ThisCall(0x...)` targets, which existing harvesters miss.

   **Commit rule.** A community name may enter the repo (docs, comments, registry) only when PC evidence corroborates it:
   - an RTTI or NiRTTI class name
   - an assert or other string
   - a command or setting table entry
   - the analyst's own reading

   Names without corroboration stay private hypotheses. Do not check names against `symbols.txt` inside the project; that list is itself PDB-derived.
3. **Library noise.**
   - Check how many CRT functions Ghidra's FunctionID databases matched (vsOlder covers VS2008) [36].
   - Do not build Havok signatures, because no clean SDK source is available. Use Havok's hkClass reflection data inside the exe instead, if FNV keeps it.
4. **Spot-check.** Compare 100 random imported names against the decompilation before anyone relies on them.

**Output.** A named database, plus a hashed name map keyed by exe SHA-256 and entry address, with source and pin tags.

**Gate.**
- The spot-check error rate is recorded.
- Coverage per name tier is added to `nvinspect coverage`.
- No label carries a PDB-derived BGS source tag.

**Expected speedup.** Finding a system's code becomes a name search instead of string and xref hunting. Estimate: 2-5x per investigation in AI, process, animation, UI and save code, with little gain for rendering.

### 3. Agent-queryable decompiler access and complete-by-construction exports

- **One canonical database.**
  - Run a local Ghidra 12.1.x Server bound to localhost, or one long-running PyGhidra process.
  - `analyzeHeadless` accepts `ghidra://localhost/<repo>` with `-connect`, `-readOnly` and `-commit` [37]. Check-in merges with the latest version [38].
  - Use one writer with an exclusive checkout, and read-only clients.
  - Fold the renames from `ghidra_N` and `decomp\codex-m1` into it, then retire the per-agent copies.
- **Persistent query service.** The default is pyghidra-mcp [39].
  - It is a Python package that drives an installed Ghidra through pyghidra, not a Ghidra extension. It may run on 12.1.4 without a rebuild; this is unverified.
  - Ghidra extensions do need a rebuild or a check against 12.1.4:
    - bethington/ghidra-mcp 6.0.0 is stable, has scripts off by default and integrates with Ghidra Server, but targets 12.1.3 [40].
    - ReVa 7.3.1 works only in GUI assistant mode, because its headless projects are ephemeral, and has no 12.1.4 build [41].
  - Set it up once, on the research lane only:

    ```
    $env:GHIDRA_INSTALL_DIR = "<ghidra_12.1.4 dir>"
    uvx pyghidra-mcp --transport streamable-http --host 127.0.0.1 --port 8000 `
      --project-path <nv-re>\ghidra-query\<project>.gpr
    ```

    Point it at a read-only copy of the canonical project, refreshed after each writer check-in. Its own read-only mode is unverified.
  - The user approves it once with a server-level Claude Code rule, `"permissions": { "allow": ["mcp__pyghidra-mcp"] }`, in the research lane's settings only. The auto-mode classifier has blocked Ghidra runs twice [28]. Confirm the tool name prefix from `/mcp` after the first connection.
  - Adopt it only if the step 1 measurement shows cold `analyzeHeadless` queries take seconds or more.
- **Complete-by-construction export.**
  - Write one Java GhidraScript or PyGhidra script that walks `FunctionManager.getFunctions(true)` and decompiles with a ParallelDecompiler pool of 8-12 workers. Throughput stops scaling at about 10-12 threads, and Python-driven decompilation has been reported to be far slower [42][43].
  - For each function, write a JSON record with:
    - entry address, body ranges, signature and calling convention
    - callers, callees and data references from ReferenceManager
    - strings, vtable slot, RTTI class and label provenance (source and pin tags)
    - decompile status (ok, timeout or error)
    - the C text and the disassembly
  - The manifest:
    - checks that the number of functions written equals `getFunctionCount()`
    - reports `.text` bytes outside any function, and vtable slots that point at undefined code
    - stamps the exe SHA-256, Ghidra version and analysis options
- **Function cards.** Write one per target, listing:
  - address, size and calling convention
  - callees and globals
  - machine-decoded float constants. Agents should never convert 0x40600000 to 3.5 by hand.
  - an instruction-class tier

**Gate.** All 2,000+ addresses already cited in repo code and docs resolve to exported functions or data, and 005cc4f0 is present.

**Expected speedup.** Lookups go from a cold JVM start plus a permission prompt to a call on a warm server, or to a read of local files. The cold-start cost has not been measured on the user's machine, so the speedup is unmeasured.

### 4. Dynamic coverage to prioritise

- **Instrument.** Add first-hit `int3` coverage to the probe DLL (step 6), first over every exported function entry and later over basic blocks. Write drcov output. Cross-check with TinyInst litecov, which claims about 15-20% overhead [44].
- **Diff runs.** Use Cartographer's set operations, for example the opening minus main-menu idle. Cartographer needs a rebuild for Ghidra 12.1 [45]. Frida Stalker (about 100x slower [46]) and Intel PT are poor fits.
- **Build the queue.** Join the result with `nvinspect coverage` and the script stall list from `nvinspect play`. The output is a queue scoped to the route: the M1 functions that actually ran, grouped by RTTI class and module tag.

**Gate.** The queue can be reproduced from a recorded route.

**Expected speedup.** "Which functions implement X" takes minutes instead of hours. In one agentic project only about 1.9k of 13.4k boot-path functions actually ran [47].

### 5. Spec extraction with clean-room separation

- **Analyst (research lane).** Has Ghidra or MCP access and reads the function card, decompilation and disassembly.
  - Reading the disassembly is required for any function with x87/SSE code, pointer adjustments or odd decompiler output.
  - Ghidra has open x87 reordering bugs [48].
  - nv-rs has recorded at least seven decompiler or inference misreads, such as the omitted +0x14 adjustment on the look-IK path [49].
- **Spec.** Lives in the private `nv-re\specs` folder and covers:
  - purpose, and inputs and outputs with units
  - constants with their addresses
  - formulas in maths notation
  - branch conditions and float operation order, written as rules
  - data dependencies and FPU state
  - test vectors
  - provenance (exe SHA-256, addresses, export hash)
  - one line on why observation alone was not enough

  A spec contains no code, pseudo-code or Ghidra identifiers.
- **Shader specs.** Shaders follow the same rule. The analyst reads the `.sdp` disassembly from `crates/shaders` (1,007 shaders in `shaderpackage013.sdp`), or recorded `CreatePixelShader` blobs [28][121], and writes:
  - the formula
  - each constant's meaning and source (record field, INI or GMST)
  - the per-vertex/per-pixel split
  - blend and depth state
  - pass order

  ENGINE_REFERENCE "From the game's shaders" is already written this way, for example `atten = 1 - saturate(d²/r²)` and the 13 bloom taps with σ = radius/2 [28]. WGSL is written from the spec. It is never written by mapping disassembly instructions one to one (`mad r0, ...` to a WGSL line). A formula is a fact or method of operation; an instruction-by-instruction port is translation risk [19][75].
- **Spec reviewer.** Works in a fresh context.
  - Checks 3 or more random constants and every branch against the disassembly.
  - Checks that the spec explains behaviour rather than transliterating code.

**Gate.** The spec passes review.

**Cost.** About 15-30 extra minutes per topic. Parallel implementation, and test vectors that become regressions, make up for it.

### 6. Oracles

Use the cheapest oracle that answers the question.

| Oracle | Use for | Notes |
| --- | --- | --- |
| scof console session | Post-INI values, quest stages, actor values | Zero tooling. Use absolute paths outside the install. Output flushes only on the next `scof` [28]. The syntax of `GetINISetting fAngleMax:LookIK` is unverified. Drive it with the documented scan-code `keybd_event` driver, not Computer Use [28]. |
| Unicorn function harness (private) | Tier A/B functions: integer, SSE arithmetic, plain x87 | Map the unpacked image or a minidump (dumpulator supports 32-bit WoW64 [50]) and call by address. **Explicitly write FPCW (0x027F, or 0x007F for the D3D thread) and MXCSR 0x1F80**, because Unicorn's reset leaves them inconsistent [51]. Unicorn computes RSQRTSS/RCPSS exactly [52], so mark those functions as emulator-inexact. Unicorn is GPL, so keep it out of nv-rs crates. |
| Native 32-bit harness (private) | Tier C/D: rsqrt/rcp, x87 transcendentals, exhaustive sweeps | Reserve 0x00400000, map the sections, set CW/MXCSR and call directly on the real CPU. Record the CPU vendor, because RSQRTSS bits reportedly differ between Intel and AMD (uncertain; the primary source was blocked). Implemented as `nv-call` in `research/nv-oracle`; tested only on synthetic code under Wine so far. |
| In-process probe DLL, "nv-oracle" | Per-frame state, call arguments and returns, input and clock replay, golden vectors on live state, audio/animation playback positions for save tests | Rust target `i686-pc-windows-msvc` (Tier 1 [53]). `extern "thiscall"` has been stable since 1.73 [54] and naked functions since 1.88 [55]; implemented as `nv-probe` in `research/nv-oracle`, with its own hooks and no external crates (retour [56] is an alternative). Inject it into the running game, or load it as an NVSE plugin from the oracle install; the ABI is two C exports and a 3-field struct [57][58]. Take each address's calling convention from the disassembly, because LTCG can create custom register conventions [59]. Use `#[repr(C, packed(4))]`-style layouts checked against Ghidra offsets. |
| WinDbg TTD | Who wrote a field, call order, real arguments in a short window | Records x86 processes and needs admin rights. About 10-20x slowdown and several GB every few minutes [60]. Use `TTD.exe -attach <pid> -ring -maxFile 2048`, or `-monitor FalloutNV.exe -recordmode Manual` started by the probe through the Live Recorder API [61]. Query without symbols: `dx @$cursession.TTD.Memory(0xc755e0, 0xc755e1, "e")` [62]. Ghidra 11.1 and later opens TTD traces with a synced timeline [63]. |
| apitrace | Render state, shaders, pass order, reference images per frame and per draw | Still the only practical native D3D9 capture for 32-bit programs [64]. Try `apitrace trace --api d3d9 <exe>` injection instead of copying d3d9.dll; injection on Windows is undocumented and must be tested. `dump-images` and `replay -s/-S` write PNGs per call; `replay -D CALL` dumps state including `RENDER_TARGET_n`. D3D9 MSAA targets are resolved, and `A16B16G16R16F` targets are written as float PFM, so the HDR scene can be read before tone mapping [106][107][108][109]. The probe can emit `D3DPERF_SetMarker` labels for draws. Latest release: 14.0 (2025-03-10) [111]. |

**nv-oracle phases** (estimates):
1. Injector, Present hook and per-frame JSONL, including the x87 control word and MXCSR: 2 days.
2. Probe manifest and trampolines: 2-3 days.
3. Golden vectors by calling engine functions by address: 1-2 days.
4. DirectInput 8 and timer virtualisation for repeatable routes: 3-5 days. Mouse/look is the input that cannot be driven today [28].

Check repeatability by diffing the hashes of two runs.

**CPU fidelity tiers.** Record a tier in every regression test.
- **A, SSE/integer:** 0 ULP. Rust primitive float operations are exact IEEE 754 with no flush-to-zero [65].
- **B, x87 arithmetic:** 0 ULP, after choosing f32 (PC=24) or f64-then-round (PC=53) from the confirmed per-thread control word.
- **C, rsqrt/rcp approximations:** at most N ULP against the native oracle, with the vendor recorded, or the exact host bits via `core::arch` intrinsics.
- **D, transcendentals:** 1-2 ULP against the native oracle. Rust's `sin` and `cos` are not covered by RFC 3514's guarantees [65].

Integer and state logic must match exactly. Runs under Proton/DXVK are not float ground truth [66].

**Shader fidelity tiers.** The tolerances below are proposals, to be calibrated on the first topic.
- **S0, formula unit tests.** Evaluate the Rust mirror of a WGSL formula on recorded constants and texels from `replay -D`. Target: at most 1/255, or a float tolerance on HDR values.
- **S1, per draw.** Compare the FP16 render target before and after one draw, inside that mesh's mask. Target: mean at most 2/255, 99th percentile at most 4/255. This needs a viewer readback of its FP16 target, which does not exist yet; Bevy's `Screenshot::primary_window()` only captures the final 8-bit window (`viewer/src/main.rs`).
- **S2, final frame.** Target: mean below 1.5/255, which is the current HUD bar [28], plus a ꟻLIP mean below an agreed threshold [112]. Mask randomly placed grass and particles.
- **Differences that cannot be removed,** so tolerances must allow for them: the D3D9 half-pixel convention, MSAA sample patterns and resolve, mip and anisotropic LOD selection, and FP16 precision. The half-pixel source was not checked this session.

### 7. Rust implementation with regression and oracle tests

- **Implementer (clean lane).**
  - **Gets:** the spec, the curated vectors, game data and nv-rs.
  - **Never has:** Ghidra or MCP tools, or any route to `nv-re`.
  - **Why OS isolation:** permission rules alone do not give that guarantee.
    - Claude Code's Read deny rules are "best-effort". They do not cover `grep -r` run from a directory, or scripts that open files themselves.
    - Claude Code's OS sandbox does not run on native Windows.
    - Parameterised `mcp__` deny rules are skipped [92][93].
    - Codex can deny path reads, but on Windows only with `[windows] sandbox = "elevated"`, and MCP stdio servers run unsandboxed [96].
  - **Use one of these two isolations:**
    1. **Cloud session (preferred, near-zero setup).** Claude Code on the web runs each session in an isolated Anthropic-managed VM with no route to local disk [95]. Conditions:
       - `nv-re` must never be pushed to any GitHub account or org that the Claude GitHub App or the user's `gh` token can reach.
       - Connectors holding findings must be off for these sessions.
       - Start cloud sessions only from the clean nv-rs repo. On Windows, `claude --cloud` on a repo without a remote uploads uncommitted changes unfiltered [95].
    2. **Separate non-admin Windows user.** The OS enforces this for every route: Read, Bash, PowerShell, MCP and subagents.
       - `net user nvimpl * /add`, and make sure `nvimpl` is not in Administrators (administrators can bypass ACLs).
       - `icacls "%USERPROFILE%\nv-re" /deny nvimpl:(OI)(CI)F /T`
       - `runas /user:nvimpl powershell`, with nvimpl's own clone of nv-rs and its own Claude or Codex login.
       - Audit shared locations: `C:\` root folders, and Ghidra and apitrace output directories.
       - These commands come from general knowledge and were not checked against Microsoft docs.

    WSL2 is not isolated by default; it auto-mounts `/mnt/c` and allows interop [97]. Use it only with `[automount] enabled=false`, `[interop] enabled=false`, `appendWindowsPath=false` and no sudo for the agent's user.

    Keep `Read(//c/Users/<you>/nv-re/**)` deny rules and a bare `mcp__*` deny as defense in depth only [92].

    Record the setup (ACL output and session environment) with each topic as independent-creation evidence.
- **Code placement.**
  - Rules go in world, physics, preview or cellview, never in the viewer, so oracle tests run headless in Windows CI [27]. Core crates stay free of external dependencies.
  - Property tests use an in-repo PRNG. Curated vectors go in `crates/<crate>/tests/oracle_<topic>.rs`.
  - Full tables run as `#[ignore]` tests gated by an environment variable such as `NV_ORACLE_DIR`. This is a new convention and should be documented in CONTRIBUTING.md.
- **Bevy adapter (viewer workspace, Bevy 0.16 today).** No corpus source covered Gamebryo-to-Bevy mapping, so the transform and animation parts below are design recommendations, not researched facts.
  - **Simulation clock.** Each frame, before the time update runs, write the vanilla-style quantised delta (whole milliseconds, as from GetTickCount [71]) into `TimeUpdateStrategy::ManualDuration`. Use `Time<Fixed>` (`set_timestep`, `discard_overstep`) only where the exe itself steps at a fixed rate. `Time<Virtual>::set_max_delta` must reproduce the exe's own clamp, not Bevy's default [99][100].
  - **Deterministic ordering.** Run game logic in one dedicated schedule, with ambiguity detection set to `LogLevel::Error` so that unordered conflicting systems fail the build.
    - Bevy 0.16-0.18 use `set_executor_kind(ExecutorKind::SingleThreaded)`; 0.19 uses `set_executor(SingleThreadedExecutor::new())`.
    - Order side effects in the exe's order with `.chain()` or explicit `before`/`after`.
    - A single-threaded executor alone does not fix the order [101].
  - **Transforms and animation.** Keep NiNode hierarchy evaluation, keyframe interpolation and controller blending in core crates, in the order and precision given by the specs. Write only the final local transforms into Bevy `Transform` and let Bevy propagate them. Do not use `bevy_animation` blending for fidelity-critical paths; the viewer does not use `AnimationPlayer` today. Golden tests compare core-crate pose output with probe-captured bone transforms.
  - **Shaders.**
    - Hand-written WGSL implements shader specs (step 5).
    - The 12 existing `viewer/src/*.wgsl` files (about 3,205 lines) say they were "ported from" game shaders. Whether any were transliterated instruction by instruction is unverified.
    - Audit them: each WGSL function gets a cited spec section and an S0-S2 oracle ID. Anything that cannot be traced to a spec is rewritten from one.
  - **Save restoration of in-progress audio (M1 blocker).** Bevy 0.16.1 with Rodio 0.20.1 exposes no sink seek or position [105]. Choose one of three routes:
    - (a) **Fastest:** add bevy_kira_audio 0.23 (Bevy ^0.16, kira ^0.9.6) in the viewer workspace only. It provides `AudioInstance::seek_to`, playback position in `PlaybackState`, and a start position [103][104].
    - (b) Migrate to Bevy 0.19.1. It has `AudioSinkPlayback::position()` and `try_seek()` from 0.17 on, and `PlaybackSettings::with_start_position`. It needs the `symphonia-vorbis` feature, because the default lewton OGG decoder always returns `NotSupported` on seek. Symphonia OGG has reported buffering and looping issues [102].
    - (c) Keep Rodio and restart from a decoded sample cursor. A private probe already showed source-sample cursor continuation [105].

    All three report decoder or mixer position, not the audible head. Device buffering remains, so the acceptance tolerance must be set from a probe measurement. Animation restore is not blocked by Bevy: poses come from core-crate state, and `ActiveAnimation::seek_to` exists in 0.16.1 if needed.
  - **Upgrade policy.**
    - Stay pinned to `bevy = "0.16"` through M1 and take route (a).
    - After M1, migrate to 0.19.1 (latest stable, 2026-08-13 [98]) one hop at a time, with a compile and visual S2 check after each hop. The guides list 117, 64 and 103 entries for 0.16→0.17, 0.17→0.18 and 0.18→0.19 [101]. The costliest change is the 0.19 render-graph removal, which affects the `ViewNode` code in `viewer/src/water.rs` and `viewer/src/grade.rs`. Next is Parley text. Estimate: 1.5-3 engineer-weeks, unverified, with no trial build run.
    - Skip 0.20 until it is stable. Its `.chain_weak()` change loosens render-side ordering [101].
    - When migrating, move bevy_kira_audio to the release that matches the Bevy version (0.26.0 for 0.19).
- **Unsupported input.** Prefer an explicit unsupported or stop path to a plausible wrong value. c2-rs scores wrong output below refusal [67], and nv-rs already stops scripts on unhandled functions [68].
- **Adversarial reviewer.**
  - Hunts counterexamples near branch boundaries.
  - Runs `cargo mutants` on the changed function [69], with a threshold such as 90% or more of mutants killed.
  - Runs a faithfulness lint that rejects quiet fakes such as constant returns, empty bodies and unlabelled guesses [47].

### 8. Acceptance against the original game

- **Function level:** the oracle tests pass at the declared CPU tier.
- **Route level:** an `nvinspect` diff compares the probe's per-frame JSONL from the original with the matching trace from nv-rs. Comparisons happen at sync points (cell load, quest stage, fixed frames) with tolerances.
- **Render level:** apitrace reference images, compared at the declared S tier. Example on the user's PC, using the existing Goodsprings recording and the viewer flags documented in ENGINE_REFERENCE [28]:

  ```
  apitrace dump --grep=Present FalloutNVGoodsprings.trace > presents.txt
  apitrace dump-images --calls=152857171 -o ref\gs_ FalloutNVGoodsprings.trace
  apitrace replay -m -S <first>-<last draw call> -s perdraw\gs_ FalloutNVGoodsprings.trace
  apitrace replay -D <call before the HDR blend draw> FalloutNVGoodsprings.trace > hdr.json
  nv-viewer <Data> WastelandNV --at -72151.5,639.2589,8281.634,0,0 --run "set TimeScale to 0" --run "set GameHour to 13.09783" --cloud-time 58.022 --wait 2 --screenshot test\gs_0152857171.png
  apitrace diff-images --fuzz 0.02 --output report.html ref\gs_ test\gs_
  flip -r ref\gs_0152857171.png -t test\gs_0152857171.png
  ```

  Notes on these commands:
  - `diff-images` pairs files by name and needs Pillow [110].
  - ꟻLIP is v1.7 and BSD-3, and handles both LDR and HDR images [112].
  - Replay must run on the same machine and GPU.
  - Whether the D3D9 retracer honours `--headless` is unverified.
  - The reference images are recordings and stay private.
- **Ledger:** each topic row records "implemented", "tested" and "compared against the original" as separate fields, with evidence links and the isolation record. "Reviewed" never defaults to true. In the Burnout project's ledger, 21,245 of 21,254 functions were marked reviewed only because that was the default [47].

## Order of work

The exe map below is built from three inputs:
- per-directory byte counts in the Xbox proxy (aggregates only [6])
- community hook addresses on PC, 87% of which fall below 0xA00000 [31][32]
- the repo's coverage table [70]

| Milestone | Ghidra and oracle focus | Not decompiled line by line |
| --- | --- | --- |
| **M1** | 1. **Look-IK pilot first**, to test the pipeline: the leaf maths at 00c755e0, 005611c0/005611e0 and 00c7f840, then 00c78610 using a minidump taken while Doc looks [49].<br>2. **Bevy determinism harness:** ManualDuration clock, single-threaded logic schedule, ambiguity set to Error.<br>3. **Camera and assist timing** from timestamped probe traces [21].<br>4. **RaceSex menu** field meanings from memory diffs.<br>5. **The M1 menus:** Start, Loading, SPECIAL, tags, Message, Dialogue, HUD.<br>6. **The main-loop dt rule** [71].<br>7. **Save restoration of in-progress audio/animation:** bevy_kira_audio 0.23 route, with probe-measured positions as the oracle [103][105]. | **Opening movie.** First read the tag with `Format-Hex`; Bink 1 is `BIK` + b/f/g/h/i/k [113]. The default is to link LGPL FFmpeg dynamically in the viewer workspace only: `ffmpeg-next` 9.0.0 is WTFPL [116], and BtbN ships win64 `lgpl-shared` builds [117]. CI needs `FFMPEG_DIR` plus clang for bindgen, untested on windows-latest. A distributed bundle needs the LGPL notice, a source offer and a replaceable DLL [115]. Stopgap: the user converts the movie locally with the `ffmpeg` CLI, and the output is never committed. An original Bink decoder stays gated until a decoder-grade prose spec is confirmed; the MultimediaWiki pages cited by FFmpeg could not be read [113][114]. It must not be written from FFmpeg source.<br>**FaceGen readers.** CTL (`FRCTL001`) and EGT (`FREGT003`): learn layouts from the vendor's file-format page (cited, unverified) [118], Smithbox's CTL template (MIT) [119] and bethesda-multitool's EGT reader (0BSD) [120]. Confirm against the user's files and the exe. Avoid the GPL OpenMW-fork `fglib`. |
| **M2** | Remaining used script and condition handlers: 640 slots, of which 325 of 622 named functions are handled, covering 98.7% of uses [68]. Verify the 311 that are handled and used with console oracles. Then AI process levels, packages and procedures (about 1.6 MB in the proxy [6]); combat, VATS and perk entry points (74 slots, one reserved); character-controller glue. | Havok solver internals. Reproduce the character proxy, contacts and collision layers from exe glue plus recordings [72]. |
| **M3** | FNV-only systems: Caravan, Blackjack, Roulette, hacking, crafting, reputation, challenges, hardcore. Magic archetypes (37 slots). Remaining record types (22 not started [70]). Bevy 0.19.1 migration (one hop at a time, S2 check per hop). | Large menu layouts, which are XML data. |
| **M4** | Frame timing and threading order. AI runs as threaded tasks in the exe, so the Bevy logic schedule must pin the order of side effects explicitly, and route diffs must cover the sync points. | |
| **M5** | Plugin, load-order and script-extension behaviour. Import of original saves only if it is scoped, using xEdit's .fos schema [17]. | Native NVSE DLL compatibility is out of scope. |
| **M6** | No new reverse engineering; keep the VR boundaries in docs/VR.md. | |

Use the exe's closed enumerations as the acceptance checklist. Add them to `nvinspect coverage` with handler addresses and an acceptance-level status:
- 640 command slots and about 250 condition functions
- 74 perk entry points and 37 archetypes
- 77 actor values and 17 package types
- 37 menus and 245 animation groups
- 121 form types and 133 extra-data types
- the RTTI classes
- every WGSL function, with its spec section and S tier

## Agent pipeline

| Role | Model and effort | Access | Artifact | Gate |
| --- | --- | --- | --- | --- |
| Scout | Cheap model | Coverage, drcov | `queue.json` (scoped to the route, gated by size) | Reproducible |
| Indexer | Deterministic script | Ghidra API | Function cards | Card count = API count |
| Analyst | Strongest available model, high effort | pyghidra-mcp read-only, disassembly (research lane, local user) | Private spec and harness descriptor | Spec review |
| Spec reviewer | Strong model, fresh context | Spec and disassembly | Pass/fail | 3+ constants and all branches checked |
| Oracle runner | Deterministic | Harness, probe, TTD, apitrace | Private vectors, reference images and manifest | A calibration vector reproduces an observed in-game value |
| Implementer | Cheaper capable model [27] | Spec, curated vectors, data, repo; runs in a cloud session or as `nvimpl`; **no** route to RE files | Rust, WGSL and tests | Tests pass at the declared tier |
| Adversary | Strong model, fresh context | Diff and spec only | Counterexamples, mutation report | Mutation threshold, faithfulness lint |
| Integrator | Maintainer or agent | Repo, oracle install | PR, ledger row with isolation record | CI (core and viewer), route diff, S2 render diff |

- **Parallelism.** 3-4 git worktrees, one owner per file, and one fresh reviewer at a time [47][12].
- **Attempt caps.** About 30 attempts per function, or a deadline. In one project the 85th percentile of successful attempts was 28 [73].
- **Size gate.** Split functions over about 1,000 instructions, because agents "give up immediately" on them [73].
- **Logging.** Log to files and offer a fast test mode that runs a sample [74].
- **Cost.** About $10 per session, based on Anthropic's compiler project: 16 agents, about 2,000 sessions, about $20k [74]. Measure model choice: on one hard project, GPT-family models beat Claude [12].
- **Expected throughput.** Plausibly 10-30 verified oracle-eligible functions a day across 3-4 worktrees. This is an estimate by analogy and has not been measured on x86 MSVC C++. Stateful systems will be slower. Measure throughput on the look-IK pilot before scaling.

## Legal and repository rules

**Do:**
- Use only the user's purchased 1.4.0.525 exe, game data and the user's own recordings. US fair use covers private intermediate copies made to extract functional facts [75][76].
- Commit facts:
  - addresses with the exe build
  - constants with units
  - formulas, including shader formulas
  - format facts
  - small curated test vectors

  Facts and methods of operation are not protected [19]. Whether oracle vectors count as "recordings" is a decision for the user.
- Write original Rust and WGSL, with simulation in core crates and a thin Bevy adapter. Structural divergence from a C++/Gamebryo design is the strongest evidence of independent creation.
- Enforce the clean-room lanes at the OS level (a cloud session or the `nvimpl` user) and keep the isolation records [92][93][95].
- Add a CI taint lint:
  - It fails on Ghidra's automatic names (`FUN_`, `DAT_` or `LAB_` plus an address, `param_N`, register inputs, `CONCAT`), fenced C blocks in docs, absolute user-profile paths, control characters, and game or recording file extensions.
  - Done in this batch: `crates/nvinspect/tests/repo_hygiene.rs`, after fixing the existing Ghidra identifiers, the NUL byte in `crates/ui/src/tile.rs` and the committed user-profile paths.
- Keep call-graph walkthroughs and per-function narratives private. EU and UK law limits passing on information derived from decompilation [19], while the right to observe and test a running program cannot be waived [77]. Record the maintainer's jurisdiction.
- Stay non-commercial, and use no Fallout logos or art.

**Don't:**
- Translate decompiled functions into Rust, map shader disassembly instruction by instruction into WGSL, or keep decompiled text or disassembly in context while writing code. Write from the maths spec instead (step 5).
  - Microsoft's June 2026 takedown notice described 4JLibs as a project that "seeks to recreate and distribute the source code" [78]. That notice also alleged leaked material and an authentication bypass.
  - re3 is still disabled [10].
- Commit decompilation, exports, exe bytes, shader blobs, recordings or apitrace reference images.
- Use leaked or prototype material, or names derived from it. This includes BGS's `xbox_pdb_matched`, `string_anchor` and `thunk_jmp` tiers, and community names added after 2025-11-26 that have no PC corroboration [86][88].
  - Fair use needs an authorised copy.
  - DMCA notices have cited leak-derived naming as evidence of taint [79].
- Document, link or automate DRM removal [24].
- Port FFmpeg or other GPL/LGPL code into the MIT/Apache core crates. Dynamic linking of LGPL FFmpeg is allowed only in the viewer workspace, with LGPL obligations met for any distributed binary [115].
- Vendor xNVSE, JIP or JG headers. xNVSE has no licence file [23], JIP is GPL-3.0 [31] and JG is LGPL-2.1 [32]. JG's "GAME -" bodies are derived from decompilation.
- Rely on Read or Grep deny rules as the clean-room boundary [92].
- Modify the user's installation.

The EULA and Steam terms forbid reverse engineering [80]. For a US user, the remaining exposure is contractual. This is not legal advice; a short IP consultation before a wide release is reasonable.

**Xbox 360 PDB decision (for the user, not an agent).**
- Xbox 360 dev-kit builds with matching PDBs are publicly reported to circulate. They reportedly come from Hidden Palace releases in Nov-Dec 2025; this is unverified here because the site was blocked.
- A CC0 matching decomp publishes a symbols.txt that names all 69,012 functions of a 2010-11-10 MemDebug build (Xbox build 425307E0) [6][87].
- fnv-source-atlas (MIT for its code) ships data derived from the PDB and says its redistribution rights are unresolved [81].
- **PDB-derived names already reach the community pool:**
  - BGS's default FNV path loads PDB-matched and PDB-assisted names without any opt-in. Its code comment calls the fallback "NVSE-known + Xbox dev-kit PDB", while its README says "xNVSE headers" [86].
  - After the leak, JohnnyGuitar's additions match symbols.txt at about six times its pre-leak baseline. No commit message mentions PDB or Xbox, and matching does not prove origin [88].
  - Step 2's pin, file removal and corroboration rule are the mitigation.
- Names transferred from Xbox would cover a lot: about 75% of unique PC virtual bodies by slot position [5]. They would not be reliable enough to trust unchecked:
  - An independent check found 86.8% exact slot-name agreement against PC headers [32].
  - BSim across instruction sets reached only a 52-63% true-positive rate [82].
- The provenance falls outside the current rules (exe, data, recordings) [27].

The default is **excluded**. Any change needs a written policy amendment that names the source and requires private-only storage and no commits. Anything built from leaked SDK or source trees stays excluded without exception.

## Risks and open questions

- **Determinism.** Havok, AI and IO threads, frame time quantised by GetTickCount [71] and an unknown RNG seed may limit route diffs to sync points with tolerances.
- **FPU state.** Several facts are unknown:
  - whether CreateDevice passes FPU_PRESERVE
  - the per-thread control words
  - whether the CRT is linked statically and dispatches to SSE2 code paths
  - the user's CPU vendor, which decides which RSQRTSS bits count as canonical
- **Calling conventions.** It is unknown whether the exe was built with /GL (LTCG), so each address's convention must come from the disassembly [59].
- **Name provenance.** Community names can be wrong, stale or PDB-derived, and pinned pre-leak trees still contain 0.7-4.1% names that match symbols.txt [86]. The corroboration rule is the control; its false-rejection rate is unmeasured.
- **Isolation gaps.** Several points are unverified:
  - whether Claude Code Read deny rules catch PowerShell `Get-Content`
  - whether subagents inherit the sandbox [94]
  - the icacls and Windows Sandbox commands
  - Codex's official docs, which were cited from source [96]

  The recommended isolations do not depend on any of these.
- **Tooling churn.** Several tools may not work on the current Ghidra:
  - The Ghidra MCP extensions and BGS are not built for 12.1.4 [41][40].
  - pyghidra-mcp's 12.1.4 compatibility and read-only behaviour are unverified [39].
  - Ghidra 12.2 requires JDK 25 [63].
  - Measure Python versus Java export throughput [42].
- **Bevy churn.** The 0.16 → 0.19.1 migration cost is estimated, not measured. Bevy 0.20 changes render-side ordering. Audio positions are mixer-side, not audible [101][102].
- **Opening movie.** The Bink revision is unknown, and a decoder-grade public Bink 1 video spec is unconfirmed [113]. The FFmpeg route adds a CI dependency that has not been tested on windows-latest.
- **Shader tolerances.** The S0-S2 thresholds are proposals. The viewer has no FP16 readback for S1 yet.
- **Address-space pressure.** TTD and the probe run inside a 2 GB process that is not large-address-aware. Apply the LAA flag only to the private copy.
- **Policy decisions pending:**
  - committing curated oracle vectors
  - a committed address and name registry
  - an NVSE plugin versus a standalone injector
  - the Xbox PDB gate
  - jurisdiction
  - FFmpeg linking versus local conversion for the movie
  - bevy_kira_audio now versus migrating to 0.19.1 first
- **Unmeasured:**
  - the cost of a cold `analyzeHeadless` run
  - the total function count
  - how many M1 functions are pure leaf functions rather than stateful, which sets how many are eligible for oracles

## Rejected approaches

- **Matching decompilation, PC or Xbox.** 13-22 team-years at known rates [7]. reccmp mainly supports old MSVC [83]. The output is C++, and the Xbox decomp bans AI contributions [6].
- **Static recompilation (XenonRecomp).** The output is unreadable, needs an XMA/MMIO runtime, and covers only the 360 build [20].
- **Decompile, then c2rust or LLM transliteration.** Behavioural correctness is low [14][15], and the result is likely a derivative work [18].
- **Instruction-by-instruction shader porting from `.sdp` disassembly to WGSL.** It carries translation risk. It is replaced by maths specs plus S0-S2 oracles.
- **A pure data-only clean room.** Too slow and too imprecise for IK maths and timing [8].
- **Switching to IDA or Binary Ninja as the primary tool.** Hex-Rays scores higher on DecompileBench [84], but migration and licence costs outweigh the gain. IDA Home is at most an optional second opinion on x87-heavy functions. Sending code to its cloud decompiler is the user's call.
- **Full-corpus indexing by parsing C text (gamedb as built).** It silently drops functions [3].
- **Emulators as bit-exact ground truth for rsqrt, rcp or x87 transcendentals.** Unicorn and Ghidra p-code do not model them faithfully [52][85].
- **PIX, Nsight, Intel GPA, or RenderDoc via DXVK as D3D9 ground truth.** They lack x86 or D3D9 support, or capture translated state.
- **Tool permission rules as the clean-room boundary.** They are best-effort, there is no OS sandbox on native Windows, and MCP is not covered [92][93][96].
- **Bevy's default parallel executor or `bevy_animation` blending for fidelity-critical logic.** The order is unspecified unless pinned, and the blending semantics are not the exe's.
- **Cloud CI with game data, or loaders and DLLs in the real install.** Forbidden by the repo rules; use the private oracle copy.
- **Leaked PDBs, Gamebryo SDK copies, fnv-source-atlas data, or BGS's PDB tiers as a "free" name list.** Excluded on provenance unless the user decides otherwise.

## Sources

1. https://github.com/NationalSecurityAgency/ghidra/releases
2. https://github.com/xNVSE/NVSE/blob/master/nvse/nvse/GameRTTI_1_4_0_525.inc
3. docs/RESEARCH_WORKFLOW.md (nv-rs)
4. https://github.com/sirus20x6/New-New-Vegas
5. https://github.com/1001Bits/BethesdaGhidraScripts/tree/main/scripts/commonlibnvse/refs
6. https://github.com/ieee802dot11ac/fnv
7. https://github.com/zeldaret/tp
8. https://gitlab.com/OpenMW/openmw/-/releases
9. https://github.com/OpenXcom/OpenXcom
10. https://github.com/github/dmca/blob/master/2021/02/2021-02-19-take-two.md
11. https://github.com/OpenRCT2/OpenRCT2
12. https://raw.githubusercontent.com/cdlewis/blog/main/content/blog/decompiling-more-snowboard-kids.md
13. https://github.com/cdlewis/snowboardkids2-decomp
14. https://github.com/albertan017/LLM4Decompile
15. https://github.com/anirudhkhatry/CRUST-bench
16. https://arxiv.org/abs/2609.05370
17. https://github.com/TES5Edit/TES5Edit/blob/dev-4.1.6/Core/wbDefinitionsFNVSaves.pas
18. https://en.wikipedia.org/wiki/Computer_Associates_International,_Inc._v._Altai,_Inc.
19. https://eur-lex.europa.eu/eli/dir/2009/24/oj
20. https://github.com/hedge-dev/XenonRecomp
21. docs/OPENING.md (nv-rs)
22. https://github.com/xNVSE/NVSE/blob/master/nvse/loader_common/IdentifyEXE.cpp
23. https://github.com/xNVSE/NVSE/blob/master/README.md
24. https://www.law.cornell.edu/uscode/text/17/1201
25. https://github.com/atom0s/Steamless
26. https://raw.githubusercontent.com/MicrosoftDocs/win32/docs/desktop-src/direct3d9/d3dcreate.md
27. AGENTS.md (nv-rs)
28. docs/ENGINE_REFERENCE.md (nv-rs)
29. https://github.com/NationalSecurityAgency/ghidra/issues/8896
30. https://github.com/xNVSE/NVSE/blob/master/nvse/nvse/CommandTable.cpp
31. https://github.com/jazzisparis/JIP-LN-NVSE
32. https://github.com/carxt/JohnnyGuitarNVSE
33. https://github.com/NationalSecurityAgency/ghidra/issues/3959
34. https://github.com/1001Bits/BethesdaGhidraScripts
35. https://github.com/1001Bits/BethesdaGhidraScripts/blob/main/scripts/commonlibnvse/pdb_naming.py
36. https://github.com/NationalSecurityAgency/ghidra-data/tree/master/FunctionID
37. https://raw.githubusercontent.com/NationalSecurityAgency/ghidra/Ghidra_12.1.4_build/Ghidra/RuntimeScripts/Common/support/analyzeHeadlessREADME.md
38. https://raw.githubusercontent.com/NationalSecurityAgency/ghidra/Ghidra_12.1.4_build/Ghidra/Features/Base/src/main/help/help/topics/VersionControl/project_repository.htm
39. https://github.com/clearbluejar/pyghidra-mcp (README: `uvx pyghidra-mcp --transport streamable-http --project-path ...`, `GHIDRA_INSTALL_DIR`)
40. https://github.com/bethington/ghidra-mcp
41. https://github.com/cyberkaida/reverse-engineering-assistant/releases
42. https://github.com/NationalSecurityAgency/ghidra/issues/8429
43. https://github.com/clearbluejar/ghidrecomp
44. https://github.com/googleprojectzero/TinyInst
45. https://github.com/nccgroup/Cartographer
46. https://raw.githubusercontent.com/frida/frida-website/main/_i18n/en/_docs/javascript-api.md
47. https://github.com/BurnoutDecomp/BP-Decomp_Workflow/blob/HEAD/STRATEGY.md
48. https://github.com/NationalSecurityAgency/ghidra/issues/3935
49. docs/OPENING_LOOK_IK.md (nv-rs)
50. https://github.com/mrexodia/dumpulator
51. https://raw.githubusercontent.com/unicorn-engine/unicorn/master/qemu/target/i386/unicorn.c
52. https://raw.githubusercontent.com/unicorn-engine/unicorn/master/qemu/target/i386/ops_sse.h
53. https://raw.githubusercontent.com/rust-lang/rust/master/src/doc/rustc/src/platform-support.md
54. https://github.com/rust-lang/rust/pull/114562
55. https://blog.rust-lang.org/2025/07/03/stabilizing-naked-functions
56. https://github.com/Hpmason/retour-rs
57. https://raw.githubusercontent.com/xNVSE/NVSE/master/nvse/nvse/PluginAPI.h
58. https://raw.githubusercontent.com/xNVSE/NVSE/master/nvse/nvse/PluginManager.cpp
59. https://raw.githubusercontent.com/MicrosoftDocs/cpp-docs/main/docs/build/reference/ltcg-link-time-code-generation.md
60. https://raw.githubusercontent.com/MicrosoftDocs/windows-driver-docs/staging/windows-driver-docs-pr/debuggercmds/time-travel-debugging-overview.md
61. https://raw.githubusercontent.com/MicrosoftDocs/windows-driver-docs/staging/windows-driver-docs-pr/debuggercmds/time-travel-debugging-ttd-exe-command-line-util.md
62. https://raw.githubusercontent.com/MicrosoftDocs/windows-driver-docs/staging/windows-driver-docs-pr/debuggercmds/time-travel-debugging-memory-objects.md
63. https://raw.githubusercontent.com/NationalSecurityAgency/ghidra/master/Ghidra/Configurations/Public_Release/src/global/docs/ChangeHistory.md
64. https://raw.githubusercontent.com/apitrace/apitrace/master/docs/USAGE.markdown
65. https://raw.githubusercontent.com/rust-lang/rfcs/master/text/3514-float-semantics.md
66. https://github.com/doitsujin/dxvk/pull/5943
67. https://raw.githubusercontent.com/freeqaz/c2-rs/HEAD/CLAUDE.md
68. docs/TECHNICAL_REFERENCE.md (nv-rs)
69. https://github.com/sourcefrog/cargo-mutants
70. crates/nvinspect/src/coverage_table.rs (nv-rs)
71. https://github.com/carxt/New-Vegas-Tick-Fix
72. https://github.com/niftools/nifxml/blob/develop/nif.xml
73. https://raw.githubusercontent.com/cdlewis/blog/main/content/blog/scaling-claude.md
74. https://www.anthropic.com/engineering/building-c-compiler
75. https://law.justia.com/cases/federal/appellate-courts/F2/977/1510/305345/
76. https://en.wikipedia.org/wiki/Sony_Computer_Entertainment,_Inc._v._Connectix_Corp.
77. https://www.legislation.gov.uk/ukpga/1988/48/section/50BA
78. https://github.com/github/dmca/blob/master/2026/06/2026-06-03-microsoft-2.md
79. https://github.com/github/dmca/blob/master/2024/09/2024-09-24-take-two.md
80. https://github.com/SteamTracking/SteamTracking/blob/master/ClientExtracted/steam_subscriber_agreement.txt
81. https://raw.githubusercontent.com/amarkham23/fnv-source-atlas/main/README.md
82. https://github.com/MISP/bsimvis/issues/30
83. https://github.com/isledecomp/reccmp
84. https://arxiv.org/abs/2505.11340
85. https://raw.githubusercontent.com/NationalSecurityAgency/ghidra/master/Ghidra/Processors/x86/data/languages/ia.sinc
86. https://github.com/1001Bits/BethesdaGhidraScripts/blob/dabb919186e0dde601908adfa5b174f699525b66/scripts/commonlibnvse/pdb_naming.py (plus `paths.py`, `dump_fnv_anchors.py`, `parse_commonlib_types.py`, `refs/` and `scripts/apply_fnv_to_user_project.py` at the same commit)
87. https://github.com/ieee802dot11ac/fnv/blob/074e045ef8220666fd8481e0745af6ecf197f24b/config/425307E0/symbols.txt
88. https://github.com/carxt/JohnnyGuitarNVSE/commits/master
89. https://github.com/korri123/kNVSE/commits/master
90. https://github.com/yvileapsis/yUI-NVSE/commits/main
91. https://github.com/xNVSE/NVSE/commits/master
92. https://code.claude.com/docs/en/permissions
93. https://code.claude.com/docs/en/sandboxing
94. https://code.claude.com/docs/en/sub-agents
95. https://code.claude.com/docs/en/claude-code-on-the-web
96. https://github.com/openai/codex at commit 7c2ce907: `codex-rs/core/src/config/permissions_tests.rs`, `codex-rs/windows-sandbox-rs/src/lib.rs`, `deny_read_acl.rs`, `codex-rs/rmcp-client/src/stdio_server_launcher.rs`
97. https://raw.githubusercontent.com/MicrosoftDocs/WSL/main/WSL/wsl-config.md
98. https://crates.io/api/v1/crates/bevy
99. https://docs.rs/bevy_time/0.19.1/bevy_time/struct.Fixed.html
100. https://docs.rs/bevy_time/0.19.1/bevy_time/enum.TimeUpdateStrategy.html
101. https://github.com/bevyengine/bevy-website/tree/main/content/learn/migration-guides (0.16-to-0.17, 0.17-to-0.18, 0.18-to-0.19, 0.19-to-0.20)
102. https://github.com/RustAudio/rodio/issues/775
103. https://crates.io/crates/bevy_kira_audio
104. https://docs.rs/kira/0.12.5/kira/sound/static_sound/struct.StaticSoundHandle.html
105. docs/PERSISTENCE.md (nv-rs), Bevy 0.16.1 / Rodio 0.20.1 probe note
106. https://raw.githubusercontent.com/apitrace/apitrace/master/retrace/retrace_main.cpp
107. https://raw.githubusercontent.com/apitrace/apitrace/master/cli/cli_dump_images.cpp
108. https://raw.githubusercontent.com/apitrace/apitrace/master/retrace/d3d9state_formats.cpp
109. https://raw.githubusercontent.com/apitrace/apitrace/master/retrace/state_writer.cpp
110. https://raw.githubusercontent.com/apitrace/apitrace/master/scripts/snapdiff.py
111. https://github.com/apitrace/apitrace/releases/tag/14.0
112. https://github.com/NVlabs/flip
113. https://raw.githubusercontent.com/FFmpeg/FFmpeg/master/libavformat/bink.c
114. https://raw.githubusercontent.com/FFmpeg/FFmpeg/master/libavcodec/bink.c
115. https://raw.githubusercontent.com/FFmpeg/FFmpeg/master/LICENSE.md
116. https://raw.githubusercontent.com/zmwangx/rust-ffmpeg/master/Cargo.toml
117. https://raw.githubusercontent.com/BtbN/FFmpeg-Builds/master/README.md
118. https://raw.githubusercontent.com/Papyrus7914/TrId2-file-type-identifier/main/database/(egt)%20(application_octet-stream)%20FaceGen%20statistical%20Texture%20information.yaml (cites https://facegen.com/dl/sdk/doc/manual/fileformats.html, unverified)
119. https://raw.githubusercontent.com/vawser/Smithbox/main/Documentation/Binary%20Templates/CTL.bt
120. https://raw.githubusercontent.com/slfx77/bethesda-multitool/main/src/BethesdaMultitool/Core/Formats/FaceGen/EgtFormat.cs
121. crates/shaders/src/lib.rs and viewer/src/game_lit.wgsl (nv-rs)
## Tools in this repository

Added on 2026-10-05 and tested only on synthetic binaries. None of them has
been run against `FalloutNV.exe` yet.

| Tool | Methodology step | What it does | Tested here |
| --- | --- | --- | --- |
| [`research/ghidra`](../research/ghidra/README.md) | 1-3 | `NvExeIdentity` (exe identity, sections, CodeView), `NvExportProgram` (export of every function with a count gate, uncovered code and vtable checks), `NvFunctionCard` (cards with decoded float constants and CPU-fidelity tier A-D), `NvLabelCommandTables` (names command handlers; stride is a parameter), `NvImportNameMap` (name maps with `src:`/`pin:` provenance) | Headless on Ghidra 12.1.4 against a stripped MinGW fixture |
| [`research/nv-oracle`](../research/nv-oracle/README.md) | 6 | `nv-call` (calls one exe function on the real CPU with chosen FPU/SSE state), `nv-probe` (in-game hooks logging arguments, memory and returns; injectable or NVSE plugin), `nv-inject` | 32-bit Wine 9 against a synthetic target; MSVC target compiled and clippy-checked but not linked |
| `crates/nvinspect/tests/repo_hygiene.rs` | Repository rules | Rejects game/binary file types, user-profile paths, control characters and Ghidra automatic names in code and docs | Runs in `cargo test` |

## Decisions after this document

On 2026-10-05 the maintainer changed two of the rules above. The decision
records take precedence over this document where they differ:

- [ADR-0002](adr/0002-xbox-prototype-symbols.md): the Xbox 360 prototype
  symbols are used in full, with `src:xbox_pdb.<tier>` tags in Ghidra and
  `(Xbox PDB)` markers in committed text. This replaces "Xbox 360 PDB
  decision" (default excluded) and the step 2 rules that keep PDB-derived
  names out. Local steps: [PROTOTYPE_SYMBOLS.md](PROTOTYPE_SYMBOLS.md).
- [ADR-0003](adr/0003-decompiled-code-in-the-project.md): decompiled logic may
  be translated into Rust when marked with its address. This replaces the
  "Don't translate decompiled functions" rule and makes the OS-isolated
  clean-room lanes (step 5, step 7 "Implementer") optional. Oracle tests,
  provenance and regression tests still apply.
- [ADR-0004](adr/0004-mudcrab-style-layout.md): the repository layout and
  docs follow Mudcrab's shape; the paths in this document move then.

Leaked SDK and source trees remain outside this methodology.
