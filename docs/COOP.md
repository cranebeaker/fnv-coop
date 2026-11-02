# Co-op: hosting and joining

One viewer process hosts; others join. Quest and story state is shared: the host runs the game's
rules and scripts, guests follow. See `crates/coop-session` (the driver), `crates/coop-core`
(protocol), `crates/coop-net` (UDP transport) and `crates/world/src/coop` (the bridge to
`GameState`).

## How to play

Host (any start works: a cell, `--new-game`, `--load-fos`, `--character`):

    nv-viewer "<game dir>\Data" --new-game --coop-host 27961 --coop-name Ann

The host logs a join code (`coop_join_code` event, also printed). Guest, on another machine or
another process (the cell is the host's, so leave it out):

    nv-viewer "<game dir>\Data" --coop-join NVRS-... --coop-name Bo

The default port is 27961 (UDP). Forward it on the host's router for play over the internet, and
put the public address in the code (the launcher does this). Each hosted session has a random
token that the `NVRS-...` code carries (coop-net's `JoinCode`); a plain `HOST:PORT` has no token
and is refused unless both sides pass `--coop-test-token` (test harness only). Steam play is in
[COOP-STEAM.md](COOP-STEAM.md). The six character join-service codes (`services/join-service`) are parsed but this
build does not call the service yet.

A guest waits (up to 90 s) for the host's place, then loads that cell. The status line at the top
left shows peers and ping; a message line announces joins, leaves, lost host and desync repairs.

## Flags

| Flag | Meaning |
|---|---|
| `--coop-host [PORT]` | host on UDP PORT (default 27961) |
| `--coop-join CODE` | join with an `NVRS-` code (UDP or Steam lobby); CELL optional. `HOST[:PORT]` only with `--coop-test-token` |
| `--coop-steam` | Steam (Spacewar) transport; falls back to UDP with a `transport_fallback` event (see COOP-STEAM.md) |
| `--coop-steam-friend` | guest: join a Steam friend who is hosting, no code |
| `--coop-token HEX` | host: use this 32 hex digit session token (the launcher's, so its code matches) |
| `--coop-test-token` | test harness only: the fixed token, so a plain `HOST:PORT` works through a relay |
| `--coop-name NAME` | name shown to the others |
| `--coop-log FILE` | JSONL event log, flushed per line |
| `--coop-state-dump FILE` | shared-state JSON, atomic, about once a second and at exit |
| `--coop-net PROFILE` | test only: delay/lose this peer's packets (`perfect good_wifi bad_wifi mobile congested flaky partition`) |
| `--coop-quit-file FILE` | exit cleanly (final dump, `shutdown` event) when the file appears |
| `--coop-quit-after SECONDS` | the same, this long after `coop_ready` |

`--stage`, `--run` and `--run-at` wait for the session to be ready (host: first guest in; guest:
join snapshot applied) and their clock starts there. On the host they run as usual. On a guest
they are sent to the host (reliably, resent until acknowledged, applied exactly once) and run
there, so scripted tests work from either side. `--new-game` and `--load-fos` are the host's and
are refused with `--coop-join`.

Events in the log: `start`, `coop_join_code`, `transport_fallback`, `steam_lobby`, `steam_joined`, `coop_listening`, `loaded`, `coop_connected`, `coop_ready`, `script`,
`state_dump`, `coop_repair`, `error`, `shutdown`, plus `peer_join`, `peer_leave`, `place`, `lost`,
`reconnected`, `desync`, `cmd_sent`, `cmd_recv`, `sync`. The state dump schema is in
`coop_session::dump` and `tools/coop-run/README.md`.

## Architecture

    viewer/src/coop.rs   Bevy shell: connect(), CoopPlugin (Input: receive/apply, AfterFrame: send)
    crates/coop-session  Session: transport + HostSync/GuestSync + HostBridge/GuestBridge
    crates/coop-net      Endpoint over UDP (handshake, slots, reliable lane, keep-alive)
    crates/coop-core     FormID keyed store, deltas, snapshots, hashes, repair, chaos transport
    crates/world/coop    GameState <-> fields, authority gates (Guest runs no scripts)

Per frame: `Session::pre_frame` (receive; a guest applies the replicated store to its
`GameState`), the game frame, `Session::post_frame` (the host mirrors its changes into the store,
the 20 Hz protocol clock sends deltas, hash checks and pings). The guest sets `Authority::Guest`
on its `GameState`, so quest, item and reference scripts do not run there. A desync is found by
hash and repaired shard by shard; a lost link reconnects (resume, or a new snapshot).

Besides coop-core's messages the session sends a few of its own (`coop_session::ext`, first byte
0xFE): the host's place, guest commands and acks, names and ping, the roster.

## Live layer in the viewer

`SessionConfig::live` (the viewer sets it) turns on, in `crates/coop-session/src/live.rs`: the campaign
policy gate and per-player shadows (host: `enable_coop_policy` with the bundled table; guest:
`enable_shadows`), the world's live adapters (`LiveHostAdapter` / `LiveGuestAdapter`) and a plain-data
`LiveView` per frame. The viewer side is `viewer/src/coop_live/` (glue only):

| File | Does |
|---|---|
| `frame.rs` | writes the local pose (`gs.coop.live.local_*`), runs `Session::live_frame`, keeps the `LiveView` in `LiveNow` |
| `puppets.rs` | one puppet actor per remote player in view (`made_placement` of the player NPC at `puppet_of(player)`, `spawn_with`), driven each frame from the interpolated avatar, name tags with distance |
| `actors.rs` | host registers the people on screen; owners publish animation ids; other peers' copies of owned actors are placed from the owner's state |
| `intents.rs` | container actions (`GameState::coop_loot` from the menus), the dialogue menu tapped from outside (open / pick / line done / close), subtitles for other players' lines, the guest following the host to a new cell or worldspace, approved fast travel |
| `chat.rs` | Y opens a line, Enter sends, Escape cancels; last lines at the bottom left |
| `bot.rs` | `coop ...` script lines: the test bot (see tools/coop-run/README.md) |

`CoopPuppet` (a component) marks the entities the live layer drives; `ai::move_actors` skips them
(`Query<Person, NpcOnly>`). `Some(player)` is a remote player's puppet, `None` the local copy of an
actor owned by another peer; a dead copy loses the mark so the ordinary AI path lets it fall.

Frame order: `receive` (Input) ... the frame ... `puppets::drive` and `actors::apply_remote` just before
`animate_actors` ... `bot`, `loot`, `tap_dialogue`, `frame::run`, `puppets::sync`, name tags, `track`,
`publish`, `play_lines`, `follow_place`, `travel`, `chat::show` (all AfterFrame) ... `send`.

Menus and time. In a session the local player's menus (Pip-Boy, container, barter, message boxes) do not
stop the simulation or the animations (`coop_live::menus_freeze_time()` guards `frame_order::begin_frame`'s
`menu_up`, `ai::move_actors`, `scripts` time and `animate_actors`), because the host's time is everyone's time
and another player's menu is never mirrored here. The remaining policy: the dialogue menu still stops
this machine's simulation while it is open (the speaker is held for the whole conversation). On a guest that
only pauses its own copy of the world; on the host it pauses the world for the guests until the line or the
conversation ends, bounded by the dialogue session's heartbeat. V.A.T.S. slow motion stays local.

Dialogue. The host's own lines run natively (`SessionConfig::native_dialogue`): the session still opens and
closes them for the others but does not run their scripts a second time. A guest's lines run on the host
(`RunBegin` / `RunEnd`, once), the guest's own menu plays them. Everyone else sees the line as a subtitle
and reports it done after a reading time.

Containers. The host opens containers natively and the loot bridge follows the world. A guest's move is a
request (`LootOp::Take` / `Put`); the confirmed result reaches its `items`. The open container menu of a
guest does not redraw until its next action.

## Limits

* Only what coop-core's schema replicates is shared: quests, stages, objectives, globals, topics,
  items by holder, script variables, karma, experience, level, perks, reputation, map markers,
  faction changes, disabled and destroyed references. The player reference is one shared holder, so
  inventory is shared, not per player.
* The guest does not show the host's avatar, NPC positions or combat yet (avatar and actor modules
  of coop-core are not wired into the viewer). `positions` in the state dump holds the local
  avatar only.
* A guest loads the host's cell once, at join. If the host later moves to another cell the guest
  stays where it is (the `place` event is logged; following needs a cell change in the viewer).
* Dialogue, barter, lockpicking and VATS run on the machine that started them. A guest's scripted
  lines (`--stage`, `--run`) go to the host; its own menus' outcomes do not yet.
* The fixed session token exists only for the test harness (`--coop-test-token`); players always
  use a join code with the host's random token.
* Players are shown when they are in the same cell, or outdoors within 2.5 squares; a player in an
  interior whose coordinates fall near an outdoor player's may show as a false positive.
* A guest follows the host's place with a fresh scene load (heading 0); it does not walk through the door.
* Barter, lockpicking, terminals and VATS run on the machine that started them; their outcomes on
  shared state go through the host only where the world layer gates them. A guest's scripted lines
  (`--stage`, `--run`) go to the host and run on the guest's own shadow rows; prefix `local ` to run
  one on the guest itself.
* Dead players are not shown lying down; downed and dead flags reach the view only.
* Puppets are not saved: refs in the puppet block (`0xFE000000`+) must not be written by a save; co-op
  sessions block saving through the policy gate.
* A plain `HOST:PORT` uses a fixed session token (friends and test runs). Use a join code for a
  random token.
* Peers must run the same build and the same plugin list (checked at the handshake).
* Reconnect after a long cut takes a full snapshot (a few seconds). Packets are not encrypted.
