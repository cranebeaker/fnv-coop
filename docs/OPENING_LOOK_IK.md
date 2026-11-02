# Opening Doc look IK trace

## Reconciled implementation, 2026-10-06

Two independent ports existed: (A) this branch's (`claude/m1-look-ik`,
7fce95d), a translation of `bhkRagdollController` with nv-call oracle
tests, and (B) a contributor's, on `playcon/claude/lookik-native-solver`
(42195d5), `lookik-head-tracking` (da055f5), `lookik-actor-anchor`
(5dfb5d9), `lookik-aim-point` (f6af27d, bc36030), `head-track-target`
(9defa47, c4fa1c8) and `facegen-eyes` (ae5b593, ddc0d59, 8fe07f2). B was
not merged (it is based on an old main); its evidence-backed parts were
re-checked in Ghidra and ported onto A on `claude/m1-lookik-reconcile`.
Credit to B's author for the target choice, eye darting and anchor traces.

`crates/world/src/look_ik.rs` ports the controller, `head_track.rs` whom
actors look at, `face.rs` the FaceGen eye update; `viewer/src/look.rs`
runs the controller on placed people after animation sampling
(`ActorRig::pose_now`) and before skinning, `ai.rs` keeps the target
slots and `faces.rs` moves the eyes. Status: implemented and unit-tested;
pure helpers checked on the real CPU (nv-call); seen working in the viewer
(below); **not** compared with the original game.

### Differences and how they were settled

| Question | A | B | Evidence | Kept |
| --- | --- | --- | --- | --- |
| Does the eye chain (mode 0, `bUseHeadPose` +0x190) run? | yes, for non-creatures | never (only the constructor writes +0x190) | 0087e541..0087e551 call the setter 004e4660 (`this+400` = 0x190) when `IsCreature` (vtable +0x21c: `Character`'s and `PlayerCharacter`'s 0047c850 return 0, `Creature`'s 008d0360 returns 1) is false; B's scan found direct stores only | A |
| What the eyes aim at (`fDesiredEyeHeading`/`Pitch`, +0x150/+0x154) | LookIK's eye heading/pitch | always 0 | 00888970 (00888989..008889ec) stores +0x19c/+0x1a0 through 00888a20 while +0xb3, else 0 | A, with B's darting around it |
| Tracking distance | own head to the anchor | between the two positions (+0x1f4) | 008a3100: its own and the target's +0x1f4 positions go to 00457990 | B |
| Units of the reach (5), behind side (5), ease-ahead (3) | Havok | game units | 008a3b70 stores the target through 004a3e00, which scales by 011c582c (1/6.9991) | A |
| "No previous rotation yet" test (00c78610) | x, y, z within 0.001 of 0 | all four zero | 00c78cc4 `AND EDX,0x7` keeps only x, y, z | A (identity counts as none) |
| Initial previous rotation, step limit | identity, armed | none (inferred), armed | constructor 00c7f1bf..00c7f1cc writes 010c71b0 (identity) at +0x120 and 1 at +0x130 per bone | A |
| Point for a target behind | eye point + side | head + side | follows from mode 0 running | A |
| Axis override `(+0x2a4)+0x1c` | not modelled | never created | 00c7de60 creates it for head-pose actors, storing the first actor's +0x150/+0x160 for others sharing the skeleton; the same set-up pose gives the same axes here | no effect |
| Other actors' look anchor | head node | head node, palette fallback, 0.9 x height | 008a2fa0; with a head pose its z is the actor's own eye point (00c757b0: head pose +0xf0 bone 1 under SG bone +0x144 and +0x10) | B + z from 00c757b0 |
| Eye limits | constructor's 60 degrees, dead zones 0 | traced (`fTrackEyeXY` 28, dead zones 20/10) but taken as unused | 00607420 → 00607810(0), 00607830, 00c748d0(0); settings 00f80fc0..00f81050 | B's values in A's mode 0 |
| Head-pose head bone local (00a86bf0) | identity, unresolved | — | 00a86bf0 is NiTransform's identity constructor | A, resolved |
| Quaternion helpers | native operation order, oracle-tested | generic f32, `acos(w)` without abs | nv-call vectors | A |
| Target out of range | nothing changes | nothing changes | 008a3a72/008a3a83 | same |
| Pose the update starts from | animated | animated (00c79680 in, 00c79a50 out) | B's trace | same |
| INI overrides | defaults only | `[LookIK]`/`[RagdollAnim]` read | registered INI settings | B |

### Ported from B

- Whom actors look at: the six target slots, the chooser 008a3ed0, filter
  008a4810 and score 008a46c0 (constants re-read: 100 at 01017a40, 500 at
  01013d84, 100 at 01016410, 120 degrees at 01084d88, 1000 at 01017b70,
  0.5 at 01011588), 008a3100's timers and frame budget, and the
  `SayTo`/dialogue/conversation slots with demote (HEAD_TRACK_TARGET.md).
- Other actors' look anchor (008a2fa0) and its 0.9 x height fallback.
- The player's anchor: `Camera1st` in first person; in third person
  (+0x64a set) as an actor, their body's `Bip01 Head`.
- FaceGen eye darting (0064be40, 0064bf90, 0064bda0; timer ranges and
  chances re-read at 01016248..010162c0, emotion offsets at
  00f81140..00f814d0) and the look morphs 0064c410 (LookDown 8, LookLeft
  9, LookRight 10, LookUp 11), now aimed at LookIK's eye angles.
- The eye dead zones going to LookIK mode 0 (ddc0d59).

### Verified

- Unit-tested: all of the above. Oracle (nv-call, same image and CPU):
  A's vectors, plus 0064c410 (five heading/pitch cases: modifier weights
  and kept angles bit for bit) and 00649f00/00649f70/00649fe0/0064a070
  (ranges and dead zones), with the settings snapshotted at their
  defaults; script `%USERPROFILE%\nv-re\work\lookikreconcile-2026-10-06\eyes_vectors.ps1`.
- Live in the viewer (release build, `--screenshot` frames and a
  temporary per-second log of each controller, kept privately in
  `%USERPROFILE%\nv-re\work\lookikreconcile-2026-10-06\live\`): in the
  opening (`--new-game`; `GSDocMitchellHouse --stage VCG01 0`) Doc's
  target is the player through each `SayTo` line, his head turns to the
  camera and, with the player beside him, stops at the cone while the
  eyes go to their 28-degree limit on that side (LookLeft); between lines
  the ACTION slot is demoted and the look eases out (1 degree an update)
  until the next line. In the Prospector Saloon Sunny Smiles turns her
  head and eyes to the player, in and out of the dialogue menu (whose
  camera still frames her). In Goodsprings (`WastelandNV --at
  -61250,13800,10280,330,5`) two Powder Gangers (00101C9E, 00101C9F)
  look at each other's eye points, one turning his head back over his
  shoulder as he walks away, and switch to the player when he comes
  within 400.
- Not compared with the original game; `nv-probe` recordings of Doc are
  the next step.

Names come from the Xbox 360 prototype's PDB (`Fallout_Release_MemDebug`,
Xbox PDB). The PC layout matches `bhkRagdollController` (Xbox PDB) with
the PC 0xC bytes shorter before `bInitLookIK` and 0x10 shorter from
`CurrentHeadRotOffset`, checked against the PC constructor 00c7f060 and
the functions below:

| PC | Field (Xbox PDB) |
| --- | --- |
| +0x10 | `WorldFromModel` (hkQsTransform) |
| +0x54 | `pHeadPose` (two-bone head/eye pose) |
| +0x88 | `pSGPose` (scene-graph pose) |
| +0xb0..b3 | `bInitLookIK`, `bEnableLookIK`, `bEaseOutLookIK`, `bActiveLookIK` |
| +0xc0 | `CurrentHeadRotOffset` |
| +0xd0 | `CurrentTargetWS` |
| +0xf0 | `BoneParamA[2]` (0x50 each: `pPose`, `ssBoneIdx`, `ssParentIdx`, `fLookAtMaxAngle`, `FwdLS`, `FwdParentMS`, `PrevRotLS`, `bLimitBoneRot`, `bActive`, `fDeadHeading`, `fDeadPitch`) |
| +0x190 | `bUseHeadPose` |
| +0x194 | `eCurrentBone` (0 `EYE_BONE`, 1 `HEAD_BONE`) |
| +0x19c / +0x1a0 | `fEyeHeading` / `fEyePitch` |
| +0x1a4 | `bTargetBehind` (not "below") |

Function pairs confirmed by these field uses: 008a3b70
`SetLookAtIKTarget`, 00c75580 `SetLookIKActive`, 00c755e0 `LimitBoneRot`
(returns the angle applied), 00c78160 `AdjustCurrentTarget`, 00c78610
`BoneTrack` (returns true when the target is outside the cone), 00c79340
`InitBoneParams`, 00c7aa60 `DoLookAtIK`, 00c7d630 `DoRagdollAnim`,
00c7de60 `InitLookIK`, 00c7f060 the constructor (all Xbox PDB names).

Behaviour traced (PC addresses):
- Set-up (0087e130): the first body part with IK data (BPND flag 0x02)
  and head tracking (0x20) gives the head bone (BPNI) and its maximum
  angle (BPND f32 at 0x14, degrees). Actors whose `IsCreature` (PC vtable
  +0x21c; Xbox +0x218) is false get `bUseHeadPose`, a two-bone pose with
  the eye 9, 6, 0 game units from the head, tracked inside the head frame.
- `InitBoneParams`: `FwdLS` is model +Y in the bone's frame (inverse
  rotation, normalized over xyz), `FwdParentMS` the same in the parent's
  frame (not normalized). Note the native vector rotations come in both
  directions; each is classified by its cross-product term.
- `AdjustCurrentTarget`: targets behind (local Y < 0) move 5 Havok units
  to the side of the eye point; while easing out the target is 3 units
  ahead of the head; the stored target is kept within 5 Havok units.
- `BoneTrack`: the cone around the parent-carried forward
  (`fLookAtMaxAngle`, 60 degrees by default, the body part's value for
  the head), the turn solve 00ce0290 including the eye-offset correction
  asin(offset / distance), then `LimitBoneRot` caps the change to
  `fAngleMax` 3.5 (easing out `fAngleMaxEase` 1.0) degrees per update;
  easing ends below `fEaseAngleShutOff` 0.5 degrees.
- `DoLookAtIK`: head first (when the head is active or there is no head
  pose), `CurrentHeadRotOffset` applied, then the eye bone; the eye's
  heading/pitch are pi/2 - acos of its forward against -Z and +X.
- Units: Havok, game units / 6.9991255 (011b02ec; 01267c20 and
  011c582c are set from it). Settings: the INI files set no LookIK
  values (their `fLookAtGain`/`fLookAtTargetGain` aren't registered), so
  the constructor defaults apply; `bLookIK`/`bRagdollAnim` default 1.
- Target point: the player in first person is looked at at the camera
  (00952ff0); other actors at their head node (008a2fa0).

Oracle evidence (nv-call, image SHA-256 19406942...739F, AMD Ryzen AI 9
HX 370): 005611c0 (normalize), 00c66320, 00c74ac0, 00c74c20, 00cb2450 and
six 00ce0290 cases (with and without the eye offset, cone clamp) are
regression tests in `look_ik.rs`; private vectors and scripts in
`%USERPROFILE%\nv-re\work\lookik-2026-10-06\`. The x87 listing of
00ce0a49 (`FMUL ST1`) stores into ST1; reading it the other way gives the
wrong correction.

Unresolved (labelled in the code):
- The model frame is taken as the skeleton root's (+Y forward); the
  native SG pose's root (via `pRagdollRoot`) is not traced.
- `bEnableRagdollAnim` (+0x41) gating isn't modelled; +0x1a8
  (`fTrackEyeZ`) is stored but no reader was found.
- Not modelled in 008a3100: the package branch (+0x27c, flag 0x100000),
  the FaceGen-distance return, the detection-level and process
  conditions (`noticed` stands for detection level >= 1); 00663510's
  three untraced gate conditions. Expressions aren't driven, so the eyes
  dart as for no expression.
- Anchors come from the last pose drawn (a frame late for actors posed
  later); the player's third-person anchor keeps the head's own height
  (whether the player's controller has a head pose isn't traced).
- Precision: SSE estimates use exact reciprocals (tier C) and x87
  extended precision is approximated with f64.

## Follow-up output trace, 2026-10-04

No runtime gaze change is implemented. Ghidra assembly confirms mode1's
active call at `00c7aa8f..00c7aab0`, selected node+0x144, second transform
selector+0x146 and quaternion cache+0x170..0x17c. `00c78610` rebuilds the
selected/subtree transforms and passes its selected quaternion through
`00c755e0`, which caps the relative rotation and writes both node and cache.

The final `005611c0`/`005611e0` correction is now resolved as quaternion
normalization: sum four squared components, reciprocal square root with one
Newton refinement, then scale all four components. The native implementation
uses SSE approximations, so bit-identical results need separate validation.

The cap selects `01267d24` or `01267d30` using controller+0xb2 and converts
degrees to radians. No delta-time factor appears in this helper: the cap is
per invocation. Although both globals are zero in the executable image,
constructors `00fbd100`/`00fbd130` register `fAngleMax:LookIK` default 3.5
and `fAngleMaxEase:LookIK` default 1.0 respectively. Post-INI runtime values
remain unverified; see the constructor evidence below.
The second mode0 call uses a temporary transform and changes+0x181 and
feedback metrics; it is not merely an inert cache update.

Remaining requirements: verify post-INI cap values, identify
the per-mode+0x110/+0xf6 setup and skeleton transform spaces, establish the
quaternion product convention, then compare a port in the original game.
Private evidence: `nv-re/work/m1-overnight-2026-10-04/gaze/FINDINGS.md` and
`hashes.sha256.json` (read-only Ghidra disassembly/helper exports). Analysis
image: FNV1.4.0.525, unpacked SHA256
`19406942E48724D797300C4EA6BE9AC69A32F670B8C35DB09279A2258422739F`.

## Earlier trace

Research status, 2026-10-03. FNV executable addresses below refer to the
existing `nv-re/decomp/codex-m1` Ghidra project. No runtime behavior has been
implemented from this trace.

## Confirmed native path

Doc's actor update `008a3100` chooses a tracked actor, obtains its anchor from
the target actor's virtual `+0x194` method, and calls `008a3b70` on the actor's
controller stored at actor `+0xac`. `008a3b70` writes the supplied xyz into
controller `+0xd0` (fourth component zero), then invokes `00c75580` to enable
the look system. The controller is created by `0087e130` through constructor
`00c7f060`; its initialization path includes a specific LookIK initialization
failure diagnostic.

The visual consumer is verified, including register/stack roles:
`00c7d630` has the controller in ECX and passes `this+0x10` on the stack to
`00c7aa60` (`dis_00c7d630.txt`, `00c7d744-00c7d747`). The callee keeps ECX as
the controller and calls `00c78160` with that same `this`. `00c78160` reads
controller `+0xd0`, alongside its matrix at `+0x10`, and calls `00c7f840`.
That helper subtracts matrix translation from target xyz, applies the inverse
controller rotation to get a local direction, and normalizes it. Thus the
target-to-local-direction transform is established and this is the actor's
LookIK path, not a generic look-at inference.

`00c78160` records whether the local target is below in controller `+0x1a4`.
The below-target branch is conditional on state byte `+0xb2`; it uses signed
clamp values `+5` and `-5` before mapping through selected skeleton
transforms. It also limits movement of the smoothed point to 5 native units
per update (`[011b05a8]`, `00c78598-00c785f0`). The precise coordinate
meaning and units of those clamps are not yet established.

`00c7aa60` calls `00c78610` with its second argument 1 for the active result,
then calls it again with that argument 0, using a temporary matrix. `00c78610`
reads the same target at controller `+0xd0`, selected skeleton index `+0x144`,
and per-mode index at `+0xf4`/`+0xf6`. Its body builds and composes quaternion
transforms and writes node transforms, but the Ghidra output is too large and
ambiguous to claim the final rotation formula or limits without tracing the
exact output writes and helpers.

## Unresolved, exact next trace

The controller constructor initializes short `+0x144` to `-1`. The setter is
now found in `00c79340`: it writes the result of `00cdd390` to
`controller + mode * 0x50 + 0xf4`; mode1 therefore writes `+0x144`.
`00c7de60` calls it with mode1. Its sole caller is `0087e575`, passing the
string returned through `0063d040 -> 00559450` from the body-part iterator
at local `-0x290`. **Assembly confirms the name is BPNI**: `0063d04a`
adds0x14 before the string getter; Ghidra's C output omitted that adjustment.
The loader `005e427c` checks BPNI and `005e42b0` writes its string at+0x14.
The iterator (`005e5320`) scans 15 BPTD part slots+0x34, selecting flag0x02;
`0087e940` selects flag0x20 for LookIK. DefaultBodyPartData0000001D's Head
entry has flags0x3b and BPNI `Bip01 Head`. Thus mode1's selected node is
data-driven, and is Bip01 Head for the standard human body data. BPNN is
different (Bip01 Neck1, stored at+4); do not confuse the two. The temporary
Ghidra project lock cleared. Evidence: look-node-getter.txt,
look-body-strings.txt and look-body-data.txt in nv-re/work/codex-m1.

For output rotation, continue `00c78610` from its call with second argument 1
in `00c7aa60`: label each transform source (`+0x110..+0x11c`, per-node data,
target local direction), follow the final writes to the selected node's
rotation, and decompile only helpers on that dataflow. Separately trace
`00c755e0` if it contributes the angle or easing value. Confirm whether the
second call (second argument 0) is just a temporary/cache update. These are the
minimum gaps before a runtime implementation can reproduce native behavior.

The opening's `SayTo Player` is a scripted talk event, not a dialogue package;
it is an explicit reason for Doc to have the player as a target, but the
viewer currently has no native actor look-target/effect wiring. Random
`SitChairRelax` idle variation is separate from the absent gaze behavior.
`viewer/src/faces.rs` documents that head-turn and eye tracking are absent;
the seated body-turn guard in `viewer/src/ai.rs` must remain intact.

## Evidence locations

Follow-up, 2026-10-04: `00c755e0` caps a relative quaternion per invocation
(no delta time in this helper). Constructors `00fbd100` and `00fbd130`
register `fAngleMax:LookIK` default 3.5 at `01267d24` and
`fAngleMaxEase:LookIK` default 1.0 at `01267d30`. Controller byte `+0xb2`
selects the value; multiplication by 0.0174533 converts degrees to radians.
These are constructor defaults, not verified post-INI runtime values.
`005611c0`/`005611e0` normalize the quaternion using four-component norm
and reciprocal square root refinement, with a zero guard. The mode0 call
updates `+0x181` and feedback metrics; it is not merely an inert cache.
Mode1 selects the `+0x144` node and caches quaternion `+0x170..+0x17c`.
The precise `+0x110`/`+0xf6` setup, transform conventions and original-game
comparison still block a faithful runtime port. Private evidence and 38-file
hash manifest: nv-re/work/m1-overnight-2026-10-04/gaze/. Unpacked executable
1.4.0.525 SHA256
`19406942E48724D797300C4EA6BE9AC69A32F670B8C35DB09279A2258422739F`.

Decompilation and disassembly: `%USERPROFILE%\\nv-re\\decomp\\codex-m1`
(`008a3100`, `008a3b70`, `00c7f060`, `00c7d630`, `00c7aa60`, `00c78160`,
`00c7f840`, `00c78610`). Opening route and package evidence:
`docs/OPENING.md` and `%USERPROFILE%\\nv-re\\work\\codex-m1\\animation-repro.log`.
