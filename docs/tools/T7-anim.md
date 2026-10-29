# T7 animation and pose checker

Owner: T7-anim. Code only; nothing here has run in the game yet. Commands:

- Python tool: `python -I tools/animcheck/run_tests.py` (ends in `ALL OK`; 46 tests, 14 mutation twins, static engine check + its twin)
- C++ probe core: `plugin\animprobe\run_tests.cmd` (ends in `ALL OK`; 6 twins). Both are in `tools/test_all.py`.

## Probe (plugin/animprobe, needs nvcoop.ini `[dev] animprobe=1`)

Bridge methods, read only, main thread, every raw read SEH-guarded (a wrong offset gives a missing field, not a crash):

- `anim.probe {refs?: ["0xFF000E","player"], near?: {radius}|number, bones?: bool, max?: 1..64, bone_names?: [...], scan_animdata?: bool}`
  returns `{v, frame, t_ms, cost_us, cell, actors:[...], skipped:[...]}`. `near` walks the player's cell reference list
  (same cell only; other cells need explicit `refs`). No params = player + everything within 2000 units.
- `anim.stats` returns probe cost (p50/p95/max/mean us, us per actor). `anim.bones` lists the default 16 bones.
- Per actor: `ref, base, player, valid3d, first_person, dead, life, pos, rot, cell`; `anim {action, sequence_raw, seqs[{slot, group, state, t, begin, end, freq, weight}]}`
  (AnimData slots 0..7 plus animSeq100 as slot 8); `move_flags` (ActorMover flags, measured in game: 0x100 walk, 0x200 run mode, 0x400 sneak; 0x800 swim not measured);
  `weapon {out, aiming, form}`; `stance {knocked, sit_sleep, swimming, ragdoll_ctrl}`; `ai {in_combat, combat_target, package}`;
  `bones {name:[x,y,z] world}` for Pelvis, Spine1/2, Neck1, Head, L/R UpperArm/Forearm/Hand, L/R Calf/Foot, Weapon (found by name in the NiNode tree, up to 700 nodes).
- Group ids follow the xNVSE enum (`tools/animcheck/animgroups_table.py`, generated). That the game uses the same numbering is NOT yet measured.

### Which node for the player (avatar fidelity)
The probe reads the reference's own 3D: `refr+0x64 -> RenderState -> +0x14` (what `TESObjectREFR::GetNiNode` 0x43FCD0 returns). The virtual
`PlayerCharacter::GetNiNode` (0x950B60) returns the first-person arms node (`+0x694`) whenever `+0x64A` is 0, so it is not used. Animation
sequences come from the process's AnimData (`BaseProcess` slot 0x6E), i.e. the third-person set, not `Actor::GetAnimData`, which switches to
the first-person set for the player. Both choices are verified statically by `verify_engine.py`; that the third-person body keeps animating
in first person is a documented engine behaviour we have not seen in a run.

### Cost
Measured now (game-free): encoding 16 actors x 16 bones to JSON takes about 0.95 ms (59 us per actor) per call on this PC (`animprobe_tests --bench`).
Engine reads are unknown until measured: `anim.stats` reports them; `tools/animcheck/cli.py collect` prints them after a run. `bones:false` skips the skeleton walk.

### Offsets: verified vs not
`python -I tools/animcheck/verify_engine.py` (read-only, FalloutNV 1.4.0.525 reference) proves: vtable addresses; BaseProcess/HighProcess slots
0x52, 0x6E, 0x9F, 0xF9, 0xFA, 0x101, 0x103, 0x115 and the fields they read; Actor `baseProcess` +0x68, life state +0x108; Actor slot 0x10B = GetCombatTarget;
refID +0xC; NiNode children data/count +0xA0/+0xA6; NiAVObject world translate at **+0x8C** (xNVSE's `NiAVObject` struct puts it at 0x88: wrong, we use raw offsets);
BSAnimGroupSequence `last` +0x34, size 0x78; PlayerMover/ActorMover slot 8.
MEASURED 2026-10-29 (live memory reads of 1.4.0.525): BSAnimGroupSequence animGroup pointer at +0x74 (the dword at +0x68 is 0x0000FFFF), group id byte at +0x10 of the TESAnimGroup (+0x08 is not it).
NOT proven (runtime-guarded: sequences must have a known vtable, mover must have a known vtable): AnimData `animSequence[]` at 0xE0, sequence state/begin/end/freq/weight,
EntryData layout, Actor +0x14D swimming, +0xAC ragdoll, +0x190 mover, PlayerCharacter +0x64A meaning. `scan_animdata:true` reports which AnimData dwords point at sequences.

## animcheck (tools/animcheck)

Streams are JSONL of probe replies (`model.py`). `Collector` polls bridges (any object with `.call`), stamps `rx_ms`, writes `<machine>.probe.jsonl`.
CLI: `cli.py analyze host.jsonl client.jsonl --avatar "A=host:14>client:FF000A" --html r.html --json r.json --nvfr r.nvfr --assert "pose_err_p95<=12"`; `cli.py collect --instance host=0 --instance client=1 --seconds 10 --out DIR`.
Python: `analyze(streams, th, owner, ref_maps, expect_moving, suppress, auto_offset, avatars)` then `counters.check(report, {"pose_err_p95":"<=12"})` returns failure strings.

Comparison: observer sample at t is matched to owner state at t - 100 ms (interpolation buffer), searching +-150 ms; best match in the window counts.
Pose error = mean distance over common bones after root (pelvis) alignment and rotation into the actor's facing frame. Mismatch episodes shorter than 400 ms are transitions, not counted.

Detectors (single machine): sliding (moving with no locomotion anim, or speed outside the anim's band), tpose (no sequence running, or arms straight out), floating/sinking
(z vs tracked ground; falls, jumps, swimming, knock-down, ragdoll excluded), jitter (direction reversals), rubberband (jump over 150 units, snap-back flagged),
stuck (only when the caller gives `expect_moving` windows), deathpose (alive anim on dead actor, death anim on live one, or no death anim). All thresholds are first guesses in `config.py`; none measured.

### Counters (flat keys in `report["counters"]`, for T6b)
NPCs: `pose_err_p50/p95/max, pos_err_p95, facing_err_p95_deg, group_mismatch_pct, group_mismatch_episodes, wrong_weapon, wrong_facing, wrong_aim, stance_mismatch, dead_mismatch, glitches, glitch.<kind>, glitches@<machine>, samples, actors, compared`.
Avatars (separate, only when `avatars=` is given; avatar refs are removed from all NPC numbers): `avatar.pose_err_p50/p95/max, avatar.pos_err_p95, avatar.facing_err_p95_deg, avatar.aim_err_p95_deg, avatar.anim_mismatches (= group_mismatch_episodes), avatar.group_mismatch_pct, avatar.wrong_weapon, avatar.wrong_facing, avatar.wrong_aim, avatar.stance_mismatch, avatar.dead_mismatch, avatar.glitches, avatar.glitch.<kind>, avatar.compared`.
Kinds: sliding, tpose, floating, sinking, jitter, rubberband, stuck, deathpose. Glitch detectors run on both the real player and the puppet.

Flight trace: findings are exported as Marker records (`nvfr_export.py`, uses `tools/flight/nvfr.py`); raw samples stay in JSONL.

## First in-game check (for the lead, one game)
1. `[dev] animprobe=1` (provision with `python -I tools/runner/run.py provision --n 1 --ini dev.animprobe=1`; provisioning rewrites nvcoop.ini, so a hand edit is lost), run `cli.py collect` while walking, drawing, reloading, sneaking; print the group numbers and compare with `animgroups.py` names.
2. Run with `scan_animdata:true` once; the offsets should be 0xE0.. in steps of 4 (plus 0x100).
3. Walk in first person with a puppet on the other machine and run `analyze --avatar`.
4. Tune `config.py` thresholds from the clean run.

## NPC fight assertions (enemy-anim, 2026-10-21)
`tools/animcheck/npc.py` adds `analyze_npc` / `check_fight` / `FIGHT_ASSERTS` on top of `analyze`: the eight glitch detectors asserted zero, one-shot
events (attack, hit reaction, reload) matched between owner and copy, weapon never drawn, and corpse resting places. Tests `tests/test_npc*.py`, runner
`python -I tools/animcheck/run_npc_tests.py` (14 twins). The in-game proof script is `tools/animcheck/npc_proof.py`. Details and thresholds:
`docs/tools/enemy-anim.md`.
