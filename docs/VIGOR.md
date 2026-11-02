# Vit-o-matic menu: implementation evidence

M1 work, 2026-10-03. Original rendered menu published; acceptance pending.
Use with OPENING.md; this is focused research, not another backlog.
Executable/build hash is recorded in OPENING.md. Decompiled code stays in
`%USERPROFILE%\nv-re\decomp\sleepmenus` and `decomp\codex-m1`.

Implemented: cellview::vigor (camera, placement, page sequences, two-model
scene, live number/button/bulb appearances, original triangle picking),
ui::menus::vigor (3D callbacks, XML input and page state), and
viewer/src/game_menus/vigor.rs. Viewer compiles and a first release opens
the original scene from ShowLoveTesterMenuParams40 without startup errors.
Built-in screenshot mode produced an inspected clean Strength page:
`%USERPROFILE%\nv-re\work\codex-m1\vigor-page1-clean.png`.
It shows the real cabinet, animated page, remaining digits and five lit bulbs.
The viewer integration regression changes live values, rejects excess points
and premature Done, then closes at the budget. Fixtures generate all inputs;
viewer now has an internal testdata dev-dependency. The old SPECIAL text
substitute is removed. Full verification/publication status is in OPENING.md.
Live PC input check passed on resuming: mouse + changed Strength 5->6,
Up changed it to7, A with3 remaining kept the menu open, mouse Right played
the Perception page transition, three Up presses changed Perception5->8
and remaining points3->0, and A closed the menu/restored the room and HUD.
This isolated run opened through ShowLoveTesterMenuParams40, not the quest.
Log: codex-m1/vigor-resume.log. The same process then loaded the existing
stage55 quicksave for opening-route validation. Full-route and original-game
visual comparison remain unverified. Index-page mouse controls still need a
live check (real-asset triangle tests cover their geometry).

## What the menu contains

Original-game observation after the user's manual setup (2026-10-03):
Strength page, value7, remaining00. Cabinet/page framing and control geometry
correspond to our model scene; the original keeps a blurred room behind the
cabinet and has much brighter lit bulbs than our first capture. This is a
qualitative comparison, not a pixel match (different room pose/values).

That exposed a viewer composition bug. Bevy0.16.1 upscaling chooses an
implicit alpha blend when another camera has already registered the target,
based on query iteration rather than camera draw order. The scene's output
clear also inherited opaque black. Now its output explicitly uses REPLACE
and transparent clear; the HUD clears its own intermediate to transparent
and blends over the scene. This removes black behind the cabinet and avoids
applying alpha to the already blended bulb glows again. A camera-output
regression and rendered before/after check cover it; fixed capture:
`nv-re/work/codex-m1/vigor-background-fixed.png`.
Exact final colour/bloom parity remains unverified.

The blurred room (B31, traced: [MENU_FADES.md](MENU_FADES.md), "The world
behind the menus"): as the tester opens (menu mode, LoveTesterMenu 1074 on
top) the game captures the room once with `PopupBackgroundFX`
(`00032B38`: a blur of 3 and nothing else, `00718ab0`) and holds it while
the menu is up (1074 keeps it even out of menu mode, `0086f450`); the
cabinet is drawn over it. The blur is the game's radius-3 pass,
`ISBLUR7` down then across with the exe's row-3 weights. Checked live in
the release viewer (window capture at VCG01 stage 60, `nv-re/work/b31/
vigor.png`): the room behind the cabinet is blurred, the cabinet and its
screen sharp; the log says `Menu background captured with
Some(FormId(00032B38)) (top menu Some(1074))`. Not compared side by side
with the original.

LoveTesterMenu 1074 loads `menus\chargen\love_tester_menu.xml`. Its XML
draws nothing on its own: invisible hotrects and unsized images catch input.
The two models are `architecture\Goodsprings\NV_VitoMaticVigorTester_Activate.NIF`
and `NV_VitoMaticVigorTester_Cabinet.NIF` (007928e0). Both are available in
the installed mesh archive. `codex-m1/vigor-activate.txt`, `vigor-cabinet.txt`
and `vigor-menu.txt` are real-data inspector outputs.

The activate model has the machine's animation sequences, including Forward
(1.30 s, 229 transform tracks) and Backward (1.27 s). The cabinet has 45
visible shapes, 5,838 triangles. This is a separate menu scene, like the
existing lockpicking renderer; a text panel does not represent it.

## Camera and transforms: corrections to older notes

007928e0 applies rotations using 00524ac0 (X) and 004a0c90 (Z), with
0101ff38 = single-precision pi/2, 0102b3c8 = single-precision pi and
01074b90 = 0.12566371 rad. Disassembly0079299d..00792a1b gives
X(pi/2) * Z(pi) * X(0.12566371), using those helpers' matrix signs
(the same helpers as the lockpick scene).
004bc1f0 writes the model's translation at +0x58; -85/-75 are NOT FOV.
Model translation is (0,-85,-75) below the display-dimension threshold,
otherwise (0,-85,-85).

Read constants using their instruction operand widths. At 00792a22 the
comparison uses a DOUBLE at 01074b88: bytes 00000000 40840000 mean **640**,
not the 0 returned by reading only its first float. 00706e50 converts the
integer from 004dc1f0 to floating point. The aspect ratio in 007945f0 uses
004dc200 / 004dc1f0 (the existing lockpick trace identifies height/width).

Camera 007945f0 uses `[Display] fDefaultFOV` (setting object01203150,
initializer00fbc020), multiplied by DOUBLE01023128 (0.0174532923847437,
float pi/180 widened) and DOUBLE01074b98 (0.649999976158142, float0.65
widened), then tan, then FLOAT01016264 (0.75). Vertical extent multiplies
height/width. 01203144 is fNearDistance (initializer00fbbf80). The camera
is positioned (0,0,1), looks at (0,0,0) with up (0,1,0), and is named
Surgery3DCamera. Near/far setup and exact render-space conversion are still
being checked. Do not copy the older agent's float-only readings of the
two double constants (0 and -2); they were disproved by disassembly.

## Pages and sequences

Constructor00720550 sets page +0x48=0, index selection +0x4c=1, animation
time +0x50=0, ready +0x54=true. Setup ends by advancing once (00795c60).
007949f0 increments/decrements page, plays OBJBookSpecialPageTurn, selects
the sequence, resets its time and starts it. Forward from page8 attempts
Done; backward from page0 does nothing. Navigation requires ready.
00791ab0 advances the chosen sequence and marks ready on reaching its end
(or when no sequence exists, or page<1). Do not substitute guessed timers.

Forward table: global011a039c is a direction flag, not entry0. Entries at
011a03a0 through011a03bc are Forward, Backward, Left, Right, FastForward,
FastBackward, FastLeft, FastRight; indexed by the NEW page minus one.
Backward table011a03c0: AimUp, Aim, Holster, JumpLand, JumpLoop, JumpStart,
TurnRight, TurnLeft; indexed by the NEW page. (Aim is the literal string
at010483b8, not AimDown.)

00794d30 maps page1..7 to actor values5..11. Page8 uses selection1..7 only
under 004b71d0's controller guard: 00709d40 calls it, and 00796100 chooses
the XBOX texture directory when true, PC when false (strings0101fd1c/24).
00791e40 increments below10 only while total<budget, decrements above1,
and writes the player's values immediately through0093a7c0. Changed values
play OBJBookSpecialNumber / OBJBookSpecialNumberDown. Done only closes
when total equals budget; otherwise UIActivateNothing. Index previous/next
clamps selection to1..7 and plays OBJBookSpecialFocus when it changes.

## Remaining implementation work

Lighting correction: 00792d61..00792d7c reads the activate model's bound
(0043d450, node+0x20; radius0084d030, bound+0x0c), multiplies by
DOUBLE0102fc70 =20.0 for the light's radius.
This is NOT frame-seconds times a constant. The fallback is at the ORIGIN:
00440460 writes (0,0,0) to +58/5c/60. 0050dd50 writes
(20*bound_radius,0,0) to +e0/e4/e8, the shader radius fields, not position.
FLOAT01019de0 is0.85 for diffuse (004bc2e0 ->+d4/d8/dc); dimmer1
(0050dd20 ->+c4). The earlier interpretation as light position was wrong.
The installed model has no embedded lights. Embedded-light replacements
currently fail explicitly rather than receiving guessed lighting.

007945f0 gets far clip from world scene graph011deb7c, virtual+100 ->
008786b0; it is not the renderer object as earlier notes said. The viewer
borrows its world camera's culling far plane. Exact original far rules
(cell/INI branches in008786b0) remain a shared camera limitation; Bevy uses
infinite reversed depth and far only affects culling. Near/FOV are traced.

Model callbacks: 007953f0 shows both remaining-point digits, tens clamped
0..9, units=max(remaining%10,0). 00795390 indexes BBNumber0..10 by AV.
00796100 supplies the actual texture paths. 00795700 chooses PC/BBRTOn
on page8 while sum<budget, PC/BBRTOff otherwise; 00795780 always chooses
PC/BBLTOff. Index +/- visibility follows limits/budget and controller
selection (00795580/00795630). Cabinet bulb shapes P1_PointVal_01..10_GLOW
are hidden above the current attribute (00795e20). Current attribute is
absent on the PC index page, so those glows are all hidden there.

Picking: 00791e40 tests page-specific array +84+page*32, then common +1a4;
first hit in each, not nearest across all buttons. Page0 has LookInside;
page8 has SPECIAL decrease/increase pairs and AllDone; common order is
P1_Decrease, P1_Increase, P1_RT, P1_LT. NiPick skip-hidden is set by00458b30.
00e9b4f0 transforms origin and direction to geometry space including inverse
scale, without renormalizing. 00eadb90 front-only branch (default pick+10=1)
uses determinant>=1e-5 (010718c0), inclusive edges, nonnegative distance.
Original local vertices and winding are kept independently of renderer
conversion. Keyboard007927a0 always maps Left/Right to previous/next page,
including page8; controller special inputs00791cf0 are separate.

Next: verify live allocation/closing and opening progression, then compare
the original game's menu before claiming M1 acceptance. PC controls are
implemented; controller inputs and replacement models with embedded lights
remain unsupported.
Assets and decompilations must not enter the repository.
