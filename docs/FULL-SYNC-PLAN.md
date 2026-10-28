# Full campaign sync plan (v1, 2026-10-20)

Goal: 2 to 4 players play the entire New Vegas campaign plus DLC in **one shared world**. Everything a player can
see or affect is the same on every machine: story, NPCs, combat, loot, world objects, time, the map.
Status today: the story layer is largely built (see automatedqa.md). This plan covers **everything**, and
especially the live layer we do not own yet: NPCs, combat, avatars and world objects.

## 0. What we learned from other projects

| Project (engine) | What they do | Lesson for us |
|---|---|---|
| Skyrim Together Reborn (Creation, open source TiltedEvolution) | Party leader advances quests; NPC **owner per actor**; ownership transfer with an **epoch counter checked server-side**; leveled NPC **pick is shared state**; stale owner state caused a mass-duplication bug (PR #906) | Epochs on every ownership change; reject updates from non-owners or old epochs; never apply echoed or stale assignment state; share leveled picks; make handoff idempotent |
| TES3MP (Morrowind) | **Cell authority** player runs a cell's AI; others are dedicated (remote) actors; server accepts actor cell changes only from the authority (followers excepted); journal, topics and factions stored on the server; scripts run locally | Authority per cell or area; follower exception; server-held story log (we have it) |
| VaultMP (FO3, open source) | Player puppets: restrained actor plus SetPos plus PlayGroup; no NPC sync | Puppet technique and its engine traps |
| NV:MP (FNV, closed) | Players plus area NPC sync to one host; quest actors deliberately "localized"; no quests | Interim avatar and NPC layer only |
| Gaffer on Games / general netcode | Snapshot interpolation needs about 100 to 150 ms buffer at 10 Hz; authority per entity | 20 to 30 Hz snapshots, a 100 ms interpolation buffer, events on a reliable channel |

## 1. The model: one world, three kinds of authority

1. **Story authority = the host, always.** Quests, stages, objectives, dialogue results, globals, karma and
   reputation, endings, scripted world changes. Done by **effect capture**: the host runs the scripts, and clients
   apply the recorded effects without re-running scripts. This is built.
2. **Actor authority = an area owner per cell or spawn site.** Live NPC and creature AI, movement and combat
   runs on exactly **one** machine per actor. Default owner: the host, if a host player is in or next to the cell.
   Otherwise the **nearest player**. Ownership carries an **epoch**; only the current owner at the current epoch
   may publish an actor's movement, AV, death or inventory. Everyone else shows a remote-driven copy.
3. **Player authority = each player owns their own avatar.** Position, animation, equipment, their own inventory
   and their own hits *as attacker*.

Hard rules that hold everywhere:
- **Nothing is ever applied twice.** Every reliable message carries a (session epoch, seq) and an idempotency id.
- **Nothing stale wins.** Ownership epoch plus sequence checks, and a client never adopts an echo of its own request.
- **One identity per thing.** Every ref has a network id: plugin plus local id for placed refs, server-issued
  ids for created 0xFF refs. Leveled picks and random rolls are made once, by the owner, and shared.
- **Detect and heal.** Periodic state digests per area; on mismatch the owner's (or host's) state is pushed and
  logged with exactly which values differed.

## 2. Transport
- ENet (already in use). Channels:
  - reliable ordered: story txns, events (spawn, death, hit, drop, container), ownership;
  - unreliable sequenced: actor snapshots;
  - voice (later, optional).
- Actor snapshots at **20 Hz near players, 5 Hz far**, with interest management by cell and distance.
  Delta-compressed against the last acked snapshot. The receiver uses a **100 ms interpolation buffer** and
  extrapolates at most 250 ms.
  **Player avatars (2026-10-27, docs/tools/avatar.md):** 30 Hz near (far 5 Hz and out 1 Hz as before) and a low-latency playback instead of the fixed 100 ms buffer: the delay follows the measured
  arrival jitter (20 ms floor), the pose is dead-reckoned past the newest snapshot (damped) and prediction misses are faded out. NPCs keep the 20 Hz / 100 ms rule.
- The host is also the relay for 3 to 4 players (star topology), so no NAT mesh is needed. A dedicated
  headless host is a later option.

## 3. System by system (everything)

### 3.1 Player avatars (replaces NV:MP for players)
- **Puppet** = a restrained actor spawned from a template NPC with the player's race, face, hair and equipment.
  Driven by SetPos/rotation plus animation groups (locomotion state, weapon draw, sneak, jump, reload, aim),
  with an equipment correction loop.
- Puppets are never saved and are purged on load. Each puppet maps to a player slot (the ghost-ref map used by
  the acting-player design).
- Traps to handle: restrained actors refusing commands (VaultMP's list), animation re-assert after menus,
  speed matching, cell borders vs transitions.

### 3.2 NPCs and creatures (the big one)
- **Area ownership service** (on the host):
  - assign, transfer and release with epochs;
  - candidates are players in or next to the cell, preferring the host;
  - hysteresis, so ownership doesn't flap;
  - follower exception: companions are owned by their recruiter wherever they go.
- **Owner side:** native AI runs. Publish position, rotation, animation and AI state (combat target, package,
  alert or sneak, weapon drawn), plus health and other AVs on change.
- **Non-owner side:** the actor's AI is **disabled** (restrained or AI-off). The actor is driven from snapshots.
  If not loaded locally, a **stand-in clone** is used, never saved and purged on load.
- **Ownership transfer:** the new owner receives the authoritative state snapshot first, applies it, then
  restores native AI. Idempotent, with no Respawn or Reset3D during a handoff (the STR #906 lesson).
- **Named or story NPCs:** owned like others for movement, but **death or kill only through the story
  authority** (a KillActor effect). Essential flags are respected; scripted resurrects remain host effects.
- **Leveled NPCs and creatures:** the owner's pick is shared state; other machines swap the base to match.

### 3.3 Spawning and encounters
- **Spawn-site arbitration:** one owner per spawn site, leveled marker or encounter zone. Non-owners turn their
  own roll off and show the owner's. Random encounters (VER* quests and PlaceAtMe in scripts) go through the
  SpawnRef effect with net ids.
- **Respawn timers and cell reset:** host-authoritative. Cleared interiors stay cleared for everyone.

### 3.4 Combat
- **Hits:** use xNVSE `onhit` (attacker, victim, damage, limb, impact point, weapon, ammo captured at fire
  time). The **victim's owner** applies the damage and sanity-checks it: distance, line of fire, weapon range,
  impact point near the victim, rate. Then it publishes the resulting AV change and limb state.
- **Player as victim:** the player's own machine applies the damage reported by the NPC's owner.
- **Projectiles, explosions, grenades, mines:** spawn events from the shooter's owner. Explosions apply damage
  through the victims' owners. Visual-only copies elsewhere.
- **VATS (NON-NEGOTIABLE, owner requirement): full VATS for every player, fully synced.**
  - **Real-time VATS (the Fallout 76 model), the default.** The world is shared, so time cannot slow for one
    player. Entering VATS highlights targets with hit chances, lets the player pick a target and cycle limbs, and
    fires in real time, with the hit chance deciding hits. No time slowdown for anyone.
  - Reference: Bethesda's own design for Fallout 76, and a Fallout 4 mod that ports real-time VATS and reworks
    VATS perks (Grim Reaper's Sprint, Four Leaf Clover, Better Criticals) for it. Do the same for New Vegas's
    VATS perks.
  - **Optional host setting "party slow-mo":** when any player enters VATS, ALL players slow together for a short,
    capped window (host-authoritative time scale, synced).
  - VATS is never simply disabled (that is what NV:MP did).
  - Hooks needed: suppress VATS's world time multiplier and its camera pause in real-time mode, keep the
    targeting UI, and queue shots that fire immediately.
  - Target and limb selection, hit chance (from that player's own skills and perks), AP use and the kill-cam all
    work normally.
  - Every VATS shot resolves through the victim owner's normal hit path. Damage, limb crippling and death are
    identical on all machines, and other players see the attack play out in real time.
  - Perks that change VATS (Grim Reaper's Sprint, Ninja, Mysterious Stranger, and others) work per player.
  - Companions in VATS must behave correctly as well.
  - Required tooling: the bots must drive VATS end to end, including **choosing a specific limb**. That needs
    static RE of the VATS limb-highlight state. Bot-side VATS limb choice is currently unsupported (see
    docs/tools/T6a-actions.md), so solving it is a blocking task for the combat milestone.
  - Required tests: a two-player VATS playbook per weapon class (gun, melee, explosive, energy) with limb
    targeting, kill-cam on and off, the crippled-limb state matching, and the other player's world not pausing.
- **Death:** only the owner (or the story host for named NPCs) declares death. Bodies are placed where they
  fell; ragdolls are visual only.
- **Crime, alarms and faction hostility:** host-authoritative, as SetEnemy/SetAlly effects. Detection by any
  player counts.

### 3.5 Loot, items and containers
- Quest rewards go to everyone (built). Leveled loot is resolved once by the owner, sending the real items
  (built for scripts; extend to corpse and container generation at cell load).
- **Containers and corpses:** shared contents (owner = host). Open, take and put are intents to the host with
  optimistic local apply and rollback. A container has one "in use" lock at a time (the barter-style menu).
- **Dropped items and world physics objects:** item drops via xNVSE `ondrop`, giving a networked world ref with
  a net id. Pickups are claims; first claim wins, and losers get "already taken". Physics clutter: position
  snapshots only while moving, then rest.
- **Player inventories:** each player's own. Equipment changes are published for puppets.
- **Anti-duplication:** every transfer is an idempotent txn with an id; the container digest is checked on close.

### 3.6 World objects
- Doors (open/closed, lock level), terminals, activators, switches, traps, enable/disable, destroyed states,
  ownership flags: ObjectState effects, host-authoritative, applied everywhere. Mostly built, the rest is in L12.
- Ref script variables (about 10k writes): RefScriptVar effects.

### 3.7 Story (built; finish the gaps)
- Effect capture for stage, topic, ambient and object scripts; guest script suppression; GuestLocalRun for
  per-machine scripts (with local-only effects); ExclusiveStages; ReadyCheck at one-way points;
  PerPlayerVars/Scenes; rewards to all with ids; payment by initiator.
- **Guest interactions** (activate, trigger enter, pickup, hit) are forwarded as intents and run on the host with
  the acting-player context. That context needs the puppet or ghost ref of the guest: see 3.1.
- Dialogue: initiator picks, everyone sees it; forced dialogue; menus on the origin player; consent prompts.

### 3.8 Player progression
- XP: everyone gets quest XP (built); kill XP goes to whoever dealt the killing blow, or a party share
  (setting). Levels, perks, skills and SPECIAL are per player.
- Karma, reputation, factions, endings: shared (owner decisions).
- Hardcore needs, radiation, addictions: per player.

### 3.9 Time, weather, travel, saves
- Game time and weather: host-authoritative. Wait and sleep need party consent (built in rules).
- Fast travel: party consent or tether rules per quest; out of combat only. Cells: free roam; story moments
  tether.
- Saves: only the host's save is the world. Clients keep a sidecar (their character plus delivered reward ids).
  Rejoin sends the baseline. Autosaves are suppressed on clients.

### 3.10 Companions
- Owned by their recruiter: actor ownership follows the recruiter. Follower caps per player. Commands are
  owner-only. The companion wheel opens on the owner.

## 4. Desync: prevent, detect, heal
- **Prevent:** single authority per thing; epochs; idempotent ids; no script re-runs; shared random picks;
  interest-managed snapshots.
- **Detect:** every 2 s, each machine sends per-area digests (actors: alive, health bucket, position bucket, cell;
  containers; object states; story digest). Hash mismatch triggers a targeted diff request.
- **Heal:** the owner pushes the authoritative values for the mismatching keys. Repeated mismatch on the same
  key is logged as a bug with both values. Rubber-banding rules: actor more than 2 m off, smooth; more than 10 m
  or a different cell, snap.
- **Observe:** a desync dashboard in the test tooling counts heals per category per minute; target 0 in tests.
- **Status (2026-10-21):** written and unit-tested game-free, not yet run in a game and not yet wired into the frame loop: docs/tools/area-desync.md
  (`synccore/areadigest`, `plugin/areadigest`, `tools/areadigest/dashboard.py`).

## 5. Build order (milestones)
1. **Live-layer spike (about 1 week):** prove on our exe that a restrained puppet moves smoothly from a 100 ms
   interpolation buffer; that AI-off plus snapshot driving works on a native NPC; and that xNVSE `onhit`/`ondrop`
   give full data. Game-free code first, then one small in-game proof.
2. **Avatars (about 3 weeks):** our own player puppets over ENet; NV:MP becomes optional.
3. **Actor ownership plus NPC snapshots (about 4 to 6 weeks):** the area ownership service with epochs and
   transfer; owner and non-owner paths; stand-ins; named-NPC protection; leveled picks.
4. **Combat (about 3 to 4 weeks):** hits via the victim owner, projectiles and explosions, death, VATS policy.
5. **Spawns, encounters, loot and containers (about 3 weeks):** spawn-site arbitration, net ids, drops and
   pickups, container locking and anti-dupe.
6. **Desync digests per area plus heal (about 2 weeks),** reusing the story digest machinery.
7. **Finish the story gaps (in parallel):** guest intents with puppet refs, L11/L12 wiring, per-machine effects.
8. **Questline by questline in-game sweeps plus two-game playthrough bots:** Goodsprings, then the prologue, and
   onward through every questline and DLC.

Rough total: 5 to 7 months of agent work for the live layer, in parallel with finishing the story layer.
That gets everything synced at the level described above.

## 6. Testing (automated, every layer)
- Game-free: an actor-ownership simulator (2 to 4 clients, lossy link, ownership churn, kill/transfer races,
  stale-echo injection like STR #906) with mutation twins.
- In-game two-copy tests:
  - a **fight test** (both players vs the same raiders: positions, health and deaths match; no duplicates);
  - a **loot race** (both grab the same item: one wins);
  - a **spawn test** (an encounter appears once);
  - an **ownership handoff** (host walks away mid-fight: the fight continues, no flapping or duplicates);
  - a **door/terminal/container** matrix;
  - the existing story sweeps.
- Desync dashboard: heals per minute must be 0 on all scripted tests.

## 6a. Split-screen mode (owner requirement: easy, for alpha testers and real players)

Split-screen on one PC = **two or more game copies on the same machine, each a borderless window filling part of
the screen, each answering to its own controller**, connected through our normal co-op session (host plus
clients on localhost). This is how Nucleus Co-op does it for games without native split-screen. We already have
most of the pieces: multi-instance (mutex and FindWindow neutralized in-process), hardlinked instances with their
own INIs and saves, always-active windows, and the game pool.

What to build:
- **A one-click launcher:** "Split-screen: 2 players (left/right or top/bottom), 3 or 4 (quad)". It provisions
  instances, sets each INI to the right resolution and borderless mode, starts host plus clients, and positions
  the windows with SetWindowPos on the right monitor.
- **Input routing, the main work:**
  - **Gamepads:** each instance answers only to its assigned XInput controller (hook XInputGetState in our
    plugin, mapping the virtual pad 0 to physical pad N; the Nucleus approach).
  - **Keyboard and mouse:** player 1 can keep KB/M while the others use pads (easiest). Multiple KB/M sets need
    raw-input filtering per device (the ProtoInput approach) and are optional and later.
  - The game's own in-process input layer (DirectInput) must ignore input meant for the other window. Our bot
    input driver already hooks the control-state query, so reuse it.
- **Focus:** every window keeps running and taking pad input while another is focused (bAlwaysActive plus a
  focus fake, so the game doesn't pause or drop input).
- **Audio:** by default only player 1's instance plays sound; optional per-player audio devices later.
- **Performance:** each copy is a full game, so RAM and GPU scale with players. Use the low-cost profile options
  per instance, document the minimum specs, and keep the per-instance fps cap configurable.
- **UI:** the HUD scales correctly at half and quarter resolutions (check HUD and menu layout at the small sizes).
- **Tests:** the bots and animation checker already drive multiple local instances; add a split-screen smoke
  playbook (windows placed, pad N reaches instance N only, both keep simulating while unfocused).

Difficulty: **moderate**. The co-op itself is the same as online (we already run several copies on one PC). The
new work is window layout, per-instance pad routing and focus faking, about 2 to 4 weeks of agent work
including tests.

## 6b. The toolkit (built 2026-10-20): use it for everything

**For a new chat implementing this plan:** read this section, then `docs/TOOLS-BRIEF.md` and each tool's status doc
in `docs/tools/*.md`. Develop **game-free first** with T1/T2/T5, and use the real game (T3/T4/T6/T7) only to
confirm. Keep CODE ONLY game launches to one copy at a time on the owner's PC (RAM is tight). Never touch the
owner's Documents\My Games INIs or saves.

| Tool | Where | Use it for |
|---|---|---|
| **T1 Flight recorder + analyzer** | `synccore/flight/`, `plugin/flight/`, `tools/flight/` (`flight.py`), spec `docs/tools/flight-format.md` | Every machine records messages, txns, ownership and incremental bucketed digests (about 0.01 ms per change). `flight.py diverge` aligns traces and narrows a desync to bucket, key and per-machine value. `shrink` cuts a failure to a minimal repro and `gentest` turns it into a regression test. Bridge methods `flight.dump` and `flight.status`. |
| **T2 Fake engine + fuzzer** | `synccore/enginemodel/` (world model plus the **actor-ownership service with epochs**: real product code), `synccore/fuzz/`, `tools/simfuzz/run.py` | Develop actor, combat, loot and spawn sync game-free. The fuzzer runs about 90 seeded 2 to 4 player sessions per second with invariants (converge, no double-apply, one owner per epoch, no dupes, no stale), shrinks failures and writes .nvfr traces. It found 3 real ownership bugs (double AI, stale position after a host load, duplicate spawn identity); fixing them is in progress. |
| **T3 Hot-reload** | `plugin/loader/`, `plugin/reload/`, `tools/runner/reload_proof.py` | Swap the plugin core inside a running game in seconds (opt-in CMake option). State and hooks carry over, and in-flight detours are drained. |
| **T4 Warm pool + scenarios** | `tools/pool/` (`pool.py up|lease|...`, low-cost INI profile, RAM guard), `tools/scenarios/` plus `scenarios/*.json` (14) | Keep 1 to 2 minimized game copies warm and jump to a known state in seconds without save reloads. |
| **T5 Chaos box** | `tools/chaos/` (relay, profiles lan to hell, RPC client), `synccore/chaos/impair.h` | Bad-network testing, identical in the simulator and between real game copies. |
| **T6 Bots + playbooks** | `tools/bots/` (Bot API; real in-process input for move/aim/fire; menus, dialogue picks, consent votes, terminals, inventory, drops, VATS, genuine fast travel), `plugin/bots/` (dev bridge methods, `[dev] bots=1`), `tools/bots/run_coop.py` plus 15 playbooks | Automated two-player tests of fights, loot races, containers, doors and terminals, companions, dialogue, PNR votes, quest sync, fast travel, ownership handoff, save/load rejoin, VATS, and the Goodsprings opening. Each step snapdiffs all machines, with strict quiet-window repair rules under chaos. |
| **T7 Animation/pose checker** | `plugin/animprobe/` (`[dev] animprobe=1`), `tools/animcheck/` | Avatar fidelity (the real player vs their puppet elsewhere) and NPC pose sync; glitch detectors (slide, T-pose, float, sink, jitter, rubber-band, stuck, death pose). Playbooks assert on its counters. |
| **RE notes** | `docs/re/dialogmenu-menus-vats-inventory.md`, `docs/engine-addresses.md` | DialogMenu, menus, VATS, inventory and terminal internals (static, verified against the exe). |

**The loop for every new sync feature:**
1. Model it in the fake engine.
2. Fuzz it until it's clean.
3. Implement it in the plugin.
4. Hot-reload it into a warm game.
5. Run the playbook with chaos.
6. Check the animations.
7. Commit.
Every desync found becomes a shrunk regression test automatically.

**First steps for the implementing chat:**
- Finish and commit the in-flight tooling fixes (the 3 fuzzer-found ownership bugs; the RefAngle load revert;
  the coop UI-bridge wiring; hot-reload drain).
- Then do the **build speed-up**: one shared build tree, sccache plus precompiled headers, runtime-switchable
  mutation twins, and affected-only test runs. Today about 13 separate build dirs recompile the same code and
  full runs take about 10 minutes.
- Then follow section 5 (milestones), starting with avatars and actor ownership in the real plugin.

## 7. Risks
- Engine resistance to driving actors (AI fights the puppet): the spike de-risks it first.
- Cell loading differs per machine: stand-ins plus ownership by loaded state.
- NV:MP coexistence during the transition: keep it optional and off by default once avatars land.
- Performance on weak PCs: interest management and rate limits; per-frame budget tests.
