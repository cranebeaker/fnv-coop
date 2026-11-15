# engine-coop: the co-op layer on the upstream engine

This workspace holds the co-op code that runs on the upstream engine
([slaterain/nv-rs](https://github.com/slaterain/nv-rs)). That engine runs the game's own program
code, translated to Rust, so the whole story plays on it.

## Rules

- **Upstream is a pinned dependency.** `Cargo.toml` names one upstream revision. This repository
  holds no upstream code. Move the revision only when the conformance rows and a smoke route pass
  on the new one.
- **One binary, one engine.** The root workspace links the older engine snapshot. The two engines
  export the same symbol, so no binary links both. Crates that both sides share
  (`coop-core`, `coop-net`, `coop-policy`, `coop-voice`) depend on no engine.
- **Program addresses come from our own work.** Each address and layout in this workspace has a
  line that names its source: our analysis of `FalloutNV.exe` 1.4.0.525, or the project's own
  plugin (branch `legacy-xnvse`, MIT). Do not copy from upstream's files that carry third-party
  marks, and do not copy from fnv_decomp.
- **State is read and written between frames.** A function override by address is used only where
  a read cannot do the work.

## Build

```
cd engine-coop
cargo build --profile play -p coop-probe
```

The first build compiles the whole engine. Keep the target folder.

## Run a probe

The engine answers Windows' questions from a table measured on the machine. Make the table and
a scratch instance first (the table describes one machine: keep it out of the repository):

```
python -I tools/mk_table.py <TOOLS TARGET FOLDER> <GAME>/FalloutNV.exe <SCRATCH>/machine.tsv
python -I tools/mk_instance.py <SCRATCH>/solo --table <SCRATCH>/machine.tsv --plugins FalloutNV.esm,DeadMoney.esm
```

`mk_table.py` runs upstream's two capture tools (`scripts/d3d9-capture`, and the 32-bit
`scripts/nvapi-capture`). `mk_instance.py` points the table's folders at a scratch folder, and
copies the player's settings as a window: the game never reads or shows the player's own
Documents in a test run, and the player's files are only read.

Set `NV_D3D9_ADAPTER` to the instance's `machine.tsv`, then:

```
coop-probe <GAME>/Data --frames 3000 --script probe/scripts/m0b_console2.txt
```

Set `NV_REPEATABLE=1` for a test run: the engine then reads a test clock, two runs are equal,
and a loaded machine does not change the order of the game's threads. With the real clock a
run on a loaded machine can stop in its first frames (`would block forever`).

A probe script has one step a line: `WHEN VERB ARGUMENTS`. `probe/src/main.rs` lists the
verbs: console lines, quest and global reads, `call`, `peek` and `poke` by address, `pos` of a
reference, and overrides that count or block a function.

With no `NV_D3D9` the game runs with no window. `NV_D3D9=real` with `NV_D3D9_BACKGROUND=1`
gives a real window that does not take the focus. Some places need the real window (an
occlusion query has no answer without it).

## Members

- `coop-engine`: the adapter. It sees the game program by address only (read, write, call),
  and names no engine type. Its tests run on a fake program: no engine, no game files.
  `addr.rs` holds each program fact with its source. `gate.rs` decides where story code runs
  on a guest, and `audit.rs` compares a guest's shared state with the host's. `avatar.rs` is
  the remote player's body. `changes.rs` reads the game's map of changed forms, and
  `record.rs` carries one changed form as the game's own save writes it. `clock.rs` is the
  game's clock on a guest. `game.rs`
  implements the session's interface (the traits of `crates/coop-game`) on the program.
- `glue`: the one crate that names upstream types. It gives the adapter its view of the
  program and starts a game for a test. `glue/src/gate.rs` puts the guest gate on the game:
  six overrides by address, each refused when the address has a wrap. `glue/src/clock.rs`
  stops a guest's clock advance (one override, with its own start check).
- `coop-app`: the co-op program without the engine. `App` runs the frame loop on a `Machine`
  (the game's frames, its screen, the guest gate): the session before and after each frame,
  the flags of the harness contract (`tools/coop-run`), the control port's answers and the
  state dump. Its tests run two whole programs in one session on fake game programs.
- `nv-coop`: the program that a player starts, as the host or as a guest. It gives the loop of
  `coop-app` the real game. See "Run two games in one session" below.
- `peer` (`coop-engine-peer`): one game behind a line protocol, for a test harness in another
  process. `coop-engine/src/peer.rs` lists the requests. With `--menu` the server waits at
  the main menu, so that a guest gets its gate before its first frame of play.
- `probe` (`coop-probe`): starts the engine, runs the game's frames, and runs a probe script
  between frames. `probe/scripts` holds the scripts.
- `tools`: the machine table and the scratch instance for a test run.

## The conformance tests on the real game

The backend conformance tests of `crates/coop-headless` run on this workspace's peer server.
Each peer of a test is one game process.

```
cargo build --profile play -p coop-engine-peer
```

Then, in the root workspace, with `COOP_ENGINE_PEER` set to the built program,
`NV_D3D9_ADAPTER` set to a scratch instance's `machine.tsv`, and `FNV_ESM_DIR` set to the
game's Data folder:

```
cargo test -p coop-headless --test engine_backend
```

Set `COOP_ENGINE_PEER` for this test only. Without it the engine backend is
`PENDING (engine backend)`, and other tests of that crate check exactly that. Each test starts
real games: use `-- --test-threads=1` on a machine that cannot run four games at one time.

## Run two games in one session

```
cargo build --profile play -p nv-coop
```

Each player runs one `nv-coop`. Each program needs its own scratch instance
(`NV_D3D9_ADAPTER`), as a probe does.

```
nv-coop <DATA FOLDER> GSProspectorSaloonInterior --coop-host 47611 --coop-test-token
nv-coop <DATA FOLDER> --coop-join 127.0.0.1:47611 --coop-test-token --coop-no-chargen
```

- The host starts in the cell that its command line names. A guest gets its gate at the main
  menu, and then it enters the host's cell.
- `--coop-control PORT` opens the control port on 127.0.0.1 (one JSON object a line):
  `{"cmd":"state"}`, `{"cmd":"run","line":"SetStage VCG03 10"}`, `{"cmd":"quit"}`.
- `--coop-log FILE` and `--coop-state-dump FILE` write the event log and the state dump of the
  harness contract. `nv-coop --help` lists each flag.
- A script line names a quest by its editor id or its form id, and a global by its editor id.
  The game's console sets a stage only with a quest's form id (measured), so the adapter puts
  the form id in place of a quest's editor id.

Not built in this program yet: the character making (a guest needs `--coop-no-chargen`), a new
game, a cell with no editor id (an exterior), the control requests `keys`, `screenshot` and
`bot`, and the other player's avatar in the loop.

## The guest gate

The host's game runs the story once. On a guest:

- A script of a shared quest does not run (its timed script, a stage result).
- A script of a quest that the policy gives to each player runs, and each script that a
  `guest_local` list of the policy names. An object's script and an effect's script run too:
  their menus, sounds and messages are the guest's own.
- A change of a shared quest's state from any code (stage, running, completed, failed,
  objective) does nothing and answers success. It waits as one intent for the host.
- The adapter's own call into the program (a console line, an apply) is not gated.
- A write that the gate cannot stop (a script's `set` line) is found by the audit: it names
  the slot, and the host's value comes back.
- The script variables of a quest whose own script is a local script are each machine's own.
  They are not exported, not applied and not compared.

The gate fails loudly. It refuses an address with a wrap or with no translation. At its start
it checks that the adapter's console line reaches it and passes, and that a stage change of a
shared quest is held with no byte changed. It counts each script run: no run in five seconds
of play is a "gate blind" alarm.

## What the probes showed (2026-11-10, upstream 815b187)

- The engine builds and runs out of tree. A console line (`coc <cell>`) reaches play from the
  main menu in two frames.
- Reads and calls of game functions between frames work with stepped threads: a quest's
  stage, a global, the game's own `SetStage` with its result scripts.
- An override by address sees calls from the host and from scripts. An address that the
  built-in script extender wraps must not be overridden: the probe refuses it.
- The console compiles a line with form ids. It does not resolve a quest's editor id.
- Two instances run side by side. In repeatable mode two runs are equal line for line.
- Places of the whole story and of each add-on load with no stop.
- Open: with real threads the run hangs after its first frame on the test PC.

## What works on two real games (upstream 815b187)

- Quest state: stages, done stages, flags, objectives and globals cross from a host's game to
  a guest's game as plain state, by snapshot and by delta. A second identical snapshot changes
  nothing. A guest refuses to set a stage. The conformance tests `snapshot_carries_stage`,
  `delta_carries_change`, `snapshot_is_idempotent`, `guest_runs_no_quest_script` and
  `global_replicates` pass, and their mutation twins fail.
- The guest gate: the conformance tests `guest_runs_no_shared_quest_script`,
  `guest_runs_local_quest`, `local_script_cannot_move_shared_quest`,
  `inline_global_is_repaired`, `guest_stage_ask_moves_host_once`,
  `audit_names_leak_when_gate_is_blind` and `adapter_line_passes_gate` pass on real games, and
  their mutation twins fail.
- The remote player's avatar: a restrained actor that the game's own mover walks from the
  remote player's movement flags, with a commanded heading and place. The conformance tests
  `avatar_follows_walk`, `avatar_holds_place`, `avatar_removed_and_made_again` and
  `avatar_look` pass on real games, and their mutation twins fail. A local hit on an avatar
  is recorded for the host and not applied, and local code does not kill an avatar
  (`avatar_survives_local_damage`, `avatar_hit_recorded_not_applied`). No save holds an
  avatar, and no load meets one (`avatar_not_in_save`).
- The world's state: the adapter reads the game's own map of changed forms, saves one
  changed form with the game's own save functions, and a guest's game loads the bytes with
  its own load: at once, or with the form's cell. The conformance tests `change_is_found`,
  `record_disables_on_guest`, `record_moves_on_guest` and `record_waits_for_its_cell` pass
  on real games, and their mutation twins fail.
- The session's interface: the engine adapter passes six conformance rows of `coop-game` on
  real games, and their twins fail (`the_interface_rows_pass_on_the_engine_and_their_twins_fail`).
  The two rows of a player's character are open until the per-player rows exist here.
- The session between two games (`nv-coop`, two programs on one PC over UDP): the guest joins,
  gets its gate at the main menu and plays in the host's cell. A stage that the host sets
  reaches the guest's game as plain state. A script line from the guest runs on the host's
  game, and each game has the result. The two state dumps have the same hash of the shared
  state without the game's clock (309 quests with state and 255 globals), at each check.
  The guest's gate stopped each script run of a shared quest (about one run in six).
- The game's clock: a guest's clock does not run by itself. The program's clock advance
  (`00867A40`) does nothing on a guest, and the host's clock comes as plain state, with the
  count of midnights. With the host's clock running, and across a midnight, the guest's hash
  of the shared state equals the host's root at the guest's revision (the contract's strict
  hash). A local `set GameHour` on a guest is one `clock_leak`, and the host's value is back
  before the next frame.
- Known limits of the session on real games:
  - One frame of the game loads a whole cell (9 seconds for the first cell with two games on
    one PC). The program cannot poll the network inside a frame, so the host keeps a silent
    guest for 20 seconds.
  - A host in a menu does not advance its clock, and the clock of each guest stands with it.
    The pause rule is a later decision.
  - The game starts its own opening dialogue in Doc Mitchell's house on a host with no
    character yet (a menu about 100 frames after the entry).
- Not yet: per-player rows, a dialogue result's owner, an exterior cell for an avatar, the
  avatars in the session's loop, the host's hit ledger, records of each kind, party saves.
  The plan is in the project's notes.
