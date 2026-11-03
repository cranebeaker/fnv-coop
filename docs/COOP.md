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

## The start of a game

Every player has their own character; the story is shared. `VCG00` and `VCG01` (the opening) are
per player in the campaign policy, so each player's rows move on their own. State machine:
`coop_session::start::StartFlow`, phases `joining -> character_creation -> arriving -> playing`
(the dump's `start.phase`, per peer). Code: `crates/coop-session/src/start.rs`; viewer glue:
`coop::start_menus`, `coop::connect`.

**Host.** `--coop-host` with `--new-game` runs the normal opening at once (not when the first guest
arrives): VCG00 start, Doc Mitchell's house, his speech, the name box, the Vit-o-Matic, tags,
traits. The host's own menus set the host player's character. Host phase: `character_creation`
until its `VCG01` reaches stage 105 (or it is finished), then `playing`. `--coop-load FILE` (the
launcher's Continue: the campaign's newest save) or `--load-fos FILE` plays on from a save
(`playing` at once). The host tells guests how far its opening is (`Opening`: `in_progress` or
`over`; over also when no opening ever ran, e.g. a cell start).

**Guest.** Joins any time. The join snapshot carries the shared story; the guest gets its own
character, local menus only (in a session no menu stops anyone's world):

| host's opening | venue | what the guest does |
|---|---|---|
| in progress | `host_place` | loads the host's place (Doc's house) and runs the menus there |
| over | `local_room` | loads `GSDocMitchellHouse` alone, runs the menus, then follows the host (`follow_place` is held until the menus are done) |

Watching first: while the host's opening is in progress the guest does not open its first menu
until nothing is playing on its screen (the host's intro movie reaches it as a scene, a late joiner is
replayed it from the point the host is at) and the host's own opening has reached its name box
(`VCG01` stage 10, `start::NAME_STAGE`, read from the host's shadow row; with no opening quest rows
visible the guest is not held). Between menus a running scene holds the next one too.

While the menus run the host's copy of the guest's own shadow does not replace the guest's live rows
(`ShadowBank::hold_own`, set by the session from `StartFlow::holds_own_rows`); the host acknowledges
the sheet only after it is applied, and the hold ends with that acknowledgement.

The menus, in order: name box, Vit-o-Matic (40 points), tag skills (3), traits (at most 2), each
the single-player menu, opened by `start_menus` when no other box is up. Then the guest sends its
character (`Sheet`: name, sex, SPECIAL, tags, perks) to the host, resent until acknowledged. The
host stores it in the guest's shadow with the guest's own `VCG00` = 100 and `VCG01` = 105 rows (no
stage script runs, so nothing opens on the host); the shadow comes back to the guest. Only then
the guest is `arriving`, and `playing` once it stands in the host's place. A guest never
overrides the host's shared state.

Who plays Doc's dialogue: the initiator picks (the campaign rule). The host's opening dialogue is
the host's own (its scripts run once, on the host); Doc's speech is not replayed for a guest. The guest's own name box, Vit-o-Matic, tags and traits are what Doc's
scripted menus would have given it. Doc's speech lines and the race, sex and face menu
(`ShowRaceMenu`) are not part of a guest's flow; the race menu is not built in nv-rs at all yet
(`docs/OPENING.md`, B15), neither for the host.

**`--new-game`, `--load-fos`, `--coop-load` on a guest** mean "my own character": `--new-game`
makes it with the menus (the default anyway), `--load-fos FILE` or `--coop-load FILE` read only the
name, SPECIAL, tags and perks of that save and skip the menus. Nothing else of the save is used.
`--coop-no-chargen` (harness) skips the character making.

Automation (no human playtests): `--answer-boxes` accepts the name box, picks Done in the tag
skills and traits menus, spends the spare Vit-o-Matic points on Strength and accepts, and answers
the opening's message boxes (`--box-answers 2` = "Finished - Travel Onward"). Scenarios:
`tools/coop-run/scenarios/live_new_game_start.json` (and its twin); headless:
`crates/coop-headless/tests/start.rs`.

Launcher: `nv-launcher --start host` (new game by default; `--continue ID` for a campaign) and
`--start join --join NVRS-...`. `--coop-help` lists the viewer's co-op flags.

## Flags

| Flag | Meaning |
|---|---|
| `--coop-host [PORT]` | host on UDP PORT (default 27961) |
| `--coop-join CODE` | join with an `NVRS-` code (UDP or Steam lobby); CELL optional. `HOST[:PORT]` only with `--coop-test-token` |
| `--coop-steam` | Steam (Spacewar) transport; falls back to UDP with a `transport_fallback` event (see COOP-STEAM.md) |
| `--coop-steam-friend` | guest: join a Steam friend who is hosting, no code |
| `--coop-token HEX` | host: use this 32 hex digit session token (the launcher's, so its code matches) |
| `--coop-test-token` | test harness only: the fixed token, so a plain `HOST:PORT` works through a relay |
| `--coop-load FILE` | host: start from this nv-rs save (the launcher's Continue); guest: take only your character from it |
| `--coop-no-chargen` | test only: a guest skips making its character |
| `--coop-help` | list the co-op flags |
| `--coop-name NAME` | name shown to the others |
| `--coop-difficulty normal\|balanced`, `--coop-friendly-fire on\|off` | host: the campaign rules (launcher: `--difficulty`, `--friendly-fire`); friendly fire is player versus player damage, see "Player versus player" |
| `--coop-log FILE` | JSONL event log, flushed per line |
| `--coop-state-dump FILE` | shared-state JSON, atomic, about once a second and at exit |
| `--coop-net PROFILE` | test only: delay/lose this peer's packets (`perfect good_wifi bad_wifi mobile congested flaky partition`) |
| `--coop-allow-mismatch` | developer override of the join compatibility check (the host's flag decides) |
| `--coop-voice off\|ptt\|vad`, `--coop-ptt-key`, `--coop-ping-key` | voice mode and keys (see Voice chat) |
| `--coop-couch SEAT/PLAYERS`, `--coop-window X,Y,W,H`, `--coop-gamepad N` | several players on one PC (docs/COOP-COUCH.md) |
| `--coop-quit-file FILE` | exit cleanly (final dump, `shutdown` event) when the file appears |
| `--coop-quit-after SECONDS` | the same, this long after `coop_ready` |
| `--coop-control PORT` | live control port on 127.0.0.1 (JSON lines: state, run, bot, keys, screenshot, entities, quit); client `tools/coop-run/live.py`, see `tools/coop-run/README.md` |

`--stage`, `--run` and `--run-at` wait for the session to be ready (host: first guest in; guest:
join snapshot applied) and their clock starts there. On the host they run as usual. On a guest
they are sent to the host (reliably, resent until acknowledged, applied exactly once) and run
there, so scripted tests work from either side. On a guest `--new-game`, `--load-fos` and
`--coop-load` mean "my own character" (see "The start of a game"); the host's world is never replaced.
A host's `--new-game` starts the opening at once; its `--stage` lines still wait for the first guest.

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

## Combat: hits on people and player versus player

Everything about damage is decided on the host.

* A guest's blow on an NPC is a request (`ActorMsg::Hit`, whole points; fractions of a point
  carry over to the next blow so a fist does the same damage every frame). The host's actor
  ledger applies it, sends `Damaged` (health afterwards) and, once, `Died`; every machine
  writes the ledger's health into its own copy of the NPC. A hit on someone the ledger does not
  know, in another cell, or past the rate limit is refused.
* The person under the crosshair is shown below it with a name, a bar and points
  (`coop_live/enemy_health.rs`; the game's own enemy meter is not drawn by this viewer).
* Remote players' attack animations: the swing or shot is an action event (`ActionKind::Swing`
  / `Shoot`, with the attack group); bare fists have no equip animation, so the puppet's
  picker is told they are up (`coop_live/actions.rs`, `puppets::drive`), otherwise it drops the
  attack and the puppet stands with its arms down.

**Player versus player** is off by default: a blow of one player on another is refused and
counted (`friendly_blocked`). The host turns it on with `--coop-friendly-fire on` (launcher:
`--friendly-fire on`; it is kept in the party save), or in a running host with the bot line
`coop ff 1` (full damage; `2` half damage, `0` off). With it on:

* a hit of any player on another, melee or ranged, goes to the host (`PlayerHit`; the host
  player's go straight to the vitals) and is applied by `VitalsHost::player_vs_player` at full
  damage, in the host's ledger of every player's health; only the victim's machine changes the
  victim's health (`PlayerHurt`);
* someone who is down or dead cannot hurt anyone and cannot be hurt (they wait for a revive,
  same rules as against NPCs); a player's own grenade always hurts them;
* the blow that puts a player down is credited to the last player who hit them within five
  seconds (`PvpDown`, logged as `player A put player B down`, counted `pvp_downs`). No
  experience, karma or crime is paid for it;
* revives work as ever (30 percent health, ailments cured).

## Voice chat

Behind the viewer's cargo feature `voice` (default off, so CI and the plain build have no audio
code). `cargo build -p viewer --features voice` pulls `coop-voice` with `capture` (cpal for the
microphone and the speakers) and, if libopus is available, add `coop-voice/opus`; without Opus the
built-in 4 bit ADPCM codec is used (16 kHz, 20 ms frames, about 8 KB/s a talker).

* Capture: the default input device; a frame goes through `VoiceTx` (resample, push-to-talk or voice
  activity gate, encode) and leaves as `Intent::Voice`, which the session sends on the unreliable
  channel as `SocialMsg::Voice`. The host checks the speaker slot, rate-limits (80 packets and
  24 KB a second per speaker) and relays to everyone else.
* Playback: `LiveView::voice` frames go to `VoiceRx`, which keeps a jitter buffer per speaker,
  decodes (or conceals a lost frame), applies the per-player volume and mute and positional gain and
  pan from the speaker's puppet position (full volume inside about 17 m, falling to 15% at about
  100 m; another cell is heard flat at 50%), and mixes to the output device.
* Keys: push-to-talk Backslash (`--coop-ptt-key`), ping middle mouse (`--coop-ping-key`), F8 toggles
  self-mute, F7 toggles deafen. `--coop-voice off|ptt|vad` sets the mode (default `ptt`). Names are
  Bevy `KeyCode` names (`Backslash`, `KeyB`, `F10`) or `MouseMiddle`, `MouseBack`, `MouseForward`.
* Per-player controls: `/mute NAME`, `/unmute NAME`, `/vol NAME 0-200` typed into chat (Y).
* The speaking indicator is a green dot on the speaker's name tag while a frame arrived in the last
  300 ms.

## Compass, map markers, pings and name tags

`coop_core::social::compass` is the maths (pure, tested): teammates and markers become compass
glyphs (position across the strip, edge-clamped when behind you, distance, height arrow, fade), map
glyphs for the Pip-Boy and the local map, name tags with a health bar (tone: healthy, hurt,
critical, downed), and `PingBook` for the player's own pings (600 ms cooldown, 8 s lifetime, three
at a time) and one waypoint. The viewer (`coop_live/hud_social.rs`) draws them on the existing
compass and map code: teammates and pings on the compass strip, teammate arrows and ping marks on
the Pip-Boy world map and the local map (drawn with the objective marker glyph), floating name tags with health bars (points over the best health seen from that player, the stream has no maximum), and a line at the top of the screen for pins behind you ("< Ann 40 m"). The ping key places a
ping where the view ray hits the world (12000 units out if nothing is hit); `Shift` + the ping key
sets the waypoint and `Ctrl` + the ping key clears it. The host expires everybody's pings.

## Weather and time

The host owns both. `coop_core::env`: the host sends its sky (current and fading-out weather, the
script override, fade progress) reliably when it changes and every 5 s, and its clock (day, hour,
time scale) twice a second on the unreliable channel. A guest applies the sky through the game's own
weather calls (`WeatherState::force` and `set`, what `ForceWeather` and `SetWeather` run) and keeps
the host's weather as the override so its own weather step never rolls; a fade that drifts by more
than 12% is corrected, and a lost override (a new climate) is put back the same frame. The guest's
`GameHour` and `GameDaysPassed` are the smoothed host clock (`ClockSmoother`: runs at the host's
scale between samples, never backwards, snaps on a sleep or wait). `world::coop::env` is the game
side; the session glue is `coop_session::live_extra`.

## Join compatibility

The handshake (`proto::Hello`) carries the protocol version, the game executable version, the build
hash, the rules hash, the volatile-key hash, the ordered plugin list and its hash. The host decides
(`coop_core::compat::decide`):

| Difference | Without `--coop-allow-mismatch` | With it |
|---|---|---|
| protocol or rules | refused | refused |
| executable version, build, volatile keys | refused | joins with a warning |
| a plugin missing or extra, or a different load order | refused | joins with a warning |

A refusal names the cause: "You are missing 1 plugin that the host has: DeadMoney.esm. Install it and
match the host's load order." The guest gets it as `Event::Compat` (the HUD message line and the
launcher's log), the host gets "Player 2 could not join: ... They lack: DeadMoney.esm." The
override is for developers; the host's flag decides. The scenario `live_compat_mismatch_refused`
(and its twin) tests the refusal end to end.

## Budget and four peers

Typical play, steady state, release build (measure with `coop-sim budget`):

| Link | Limit |
|---|---|
| host to one guest | under 64 KB/s |
| one guest to the host | under 24 KB/s |
| host to all guests (3) | under 192 KB/s |
| one peer's frame (session, live layer, scripts, no rendering): mean / p99 | under 4 ms / 16 ms |

The join snapshot is a one-off and is reported apart (`join up/down`). `coop-sim budget SCENARIO
[--guests N] [--all-profiles] [--gate-cpu]` runs a scenario with 1 to 3 guests under one or every
chaos profile and prints the table; it fails when a limit is exceeded (CPU only with `--gate-cpu`, as
debug builds are several times slower). Scenarios list their guests with `"guests": [..]` (up to 3)
and check them with `"on": "guest2" | "guest3" | "all"`. `coop-sim suite --soak` runs
`live_four_peers` under every profile; `tools/coop-run/scenarios/live_four_viewers.json` is the same
script on four real viewers (it fits the test PC's RAM) with its twin.

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
* Peers must run the same build, game version and plugin list in the same order; the handshake refuses with a message naming the difference (see Join compatibility).
* Reconnect after a long cut takes a full snapshot (a few seconds). Packets are not encrypted.

## Shared people and creatures (2026-11-02)

* The live layer files a player and an actor under their place: the interior cell, or outdoors the
  worldspace (`world::coop::live::space_of_player`). Before, an exterior actor belonged to its home grid
  square, so it had an owner only while a player stood in that square and hits from outdoors were refused
  (the attacker's cell never matched).
* Every peer reports the people and creatures it has loaded (`actors::track`): the host registers them, a
  guest sends `ActorMsg::Track` and the host registers what it did not know. A creature found where only one
  peer has loaded is therefore shared; `actors::adopt_remote` gives the other peer an entity for it when it
  comes within the loaded squares.
* The host alone decides a spawn: a person or creature a script made (`PlaceAtMe`) is announced once as
  `Spawned` under the host's id (`ActorHost::announce_made`); guests make no people or creatures of their
  own (`placed::place_at_me` returns nothing for a guest) and show the announced ones through
  `bring_in_made`. Hits and death go through the host ledger, once.
* A dormant actor (nobody shares its place) is not applied over the local copy.
* Dump counters for a real run: `live.actors_tracked`, `actors_known`, `actors_owned_local`,
  `actors_remote`, `actors_dormant`, `creatures`, `spawned`, and the viewer's `npc_entities`,
  `npc_entities_visible`, `creature_entities`, `creature_entities_visible`; `actors.<name>` has
  `alive`, `owner` (`me`, `other`, `none`) and `creature`. Scenarios: `live_npcs_present`;
  `crates/coop-headless/tests/npcs.rs` has `live_creature_spawn_shared` with its twin and a soak.

## Host migration

There is none, by policy. The host is slot 0 for the life of a session and every rule that needs an authority (scripts, policy, ownership of actors) runs there. If the host leaves or its link is lost for good, the session ends and the party reloads the last party save under a new host. A guest never becomes a host. The master checklist row core.host_migration guards this statement.
