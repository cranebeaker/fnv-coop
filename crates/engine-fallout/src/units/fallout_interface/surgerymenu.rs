//! `fallout/interface/menus/surgerymenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// `SurgeryMenu` (Xbox PDB; the PC offsets are the code's): the animation state (+0x28),
// the tiles (`pTiles`, +0x2c), the tool location (+0x74), the burning flag (+0x90), the
// two sound handles (+0x94, +0xa0), the pain (+0xac), the current damaged part (+0xc0)
// and level (+0xc4). `pSurgeryMenu` is `011dada4`; `xLimbWounds` (`011dae48`) is a list
// of `LimbWound`s (a list of sequences +0, the actor value +4, the damage level +8 and a
// reset flag +0xc).

/// `SurgeryMenu::pSurgeryMenu` (`011dada4`, by its use), the limb wounds list head
/// (`011dae48`), the skin and camera holders (`011dae5c`, `011dade8`; `NiPointer`s read
/// through `00559450`) and the class number.
const SURGERY_MENU_ME: u32 = 0x011d_ada4;
const LIMB_WOUNDS: u32 = 0x011d_ae48;
const SKIN_HOLDER_011DAE5C: u32 = 0x011d_ae5c;
const SKIN_ROOT_HOLDER_011DADE8: u32 = 0x011d_ade8;
const SURGERY_MENU_CLASS: u32 = 0x41e;

/// List nodes: the empty test (`008256d0`), the item slot (`006815c0`) and the next
/// node (`00726070`); the first word of a holder (`00559450`) and a list clear
/// (`00470470`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const NODE_SELF: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const FIRST_WORD: u32 = 0x0055_9450;
const LIST_CLEAR: u32 = 0x0047_0470;
/// The menu by class (`00a09030`, cdecl), `Interface::IsTopMenuID` (`00702450`, cdecl),
/// the integer trait setter (`00700320`, `RET 8`), the float trait setter (`00a012d0`,
/// `RET 0xc`), the string trait setter (`00a01350`, `RET 0xc`), `Tile::TextToTrait`
/// (`00a01860`, cdecl) of `"_Value"` (`0106ed18`), `Menu::StartFadeOut` (`00a1d910`),
/// the interface manager (`004b7210`) and its menu tile (`0045cd60`).
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const VALUE_TRAIT_NAME: u32 = 0x0106_ed18;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const MANAGER_MENU_TILE: u32 = 0x0045_cd60;
/// Whether a gamepad is in use (`004b71d0`, no words).
const GAMEPAD_IN_USE_004B71D0: u32 = 0x004b_71d0;
/// Sounds: the handle's constructor (`0041a250`), assignment (`00418900`, `RET 4`) and
/// destructor (`00483710`), `BSAudio::QInstance` (`00453a70`),
/// `GetSoundHandleByName` (`00ad7550`, `RET 0xc`), `BSSoundHandle::Play` (`00ad8830`,
/// `RET 4`), `::Stop` (`00ad88f0`) and `::Release` (`00ad8d10`); `"UISurgeryExit"`
/// (`0107767c`).
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_STOP: u32 = 0x00ad_88f0;
const SOUND_RELEASE: u32 = 0x00ad_8d10;
const EXIT_SOUND: u32 = 0x0107_767c;
/// The player (`011dea3c`) and its actor value owner (+0xa4, slot +8: a value).
const PLAYER: u32 = 0x011d_ea3c;
/// The screen size (`004dc1f0`, `004dc200`) and the cursor setter `00703080` (cdecl,
/// two `float`s).
const SCREEN_004DC1F0: u32 = 0x004d_c1f0;
const SCREEN_004DC200: u32 = 0x004d_c200;
const SET_CURSOR_00703080: u32 = 0x0070_3080;
/// The fixed-string constructor (`00438170`, `RET 4`) and destructor (`004381b0`) of
/// `"BSFaceGenNiNodeSkinned"` (`010203f0`), the interpolation `004b3ab0` (cdecl, five
/// `float`s, `ST0`) with 100.0 (`01016410`), the clock (`011f6394`, milliseconds through
/// `00825c00`), 1000.0 (`01017b70`, `double`), the update data (`0043d410`, `RET 0xc`) and
/// the object update (`00a59c60`, `RET 4`).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const SKIN_NODE_NAME: u32 = 0x0102_03f0;
const INTERPOLATE_004B3AB0: u32 = 0x004b_3ab0;
const HUNDRED_01016410: u32 = 0x0101_6410;
const CLOCK_011F6394: u32 = 0x011f_6394;
const MILLISECONDS: u32 = 0x0082_5c00;
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
const UPDATE_DATA_INIT: u32 = 0x0043_d410;
const UPDATE_OBJECT: u32 = 0x00a5_9c60;
/// `ActorValue::GetActorValueName` (`0066e950`, cdecl), a sequence's `00493770` (`ST0`),
/// its length (`00508100`, `ST0`) and `00759450` (`ST0`), and
/// `NiControllerSequence::SetPhase` (`00a328b0`, `RET 8`).
const ACTOR_VALUE_NAME: u32 = 0x0066_e950;
const SEQUENCE_00493770: u32 = 0x0049_3770;
const SEQUENCE_LENGTH: u32 = 0x0050_8100;
const SEQUENCE_00759450: u32 = 0x0075_9450;
const SEQUENCE_SET_PHASE: u32 = 0x00a3_28b0;
/// 0.0 (`01012060`, `double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// The items of the list from `node` while it is not empty (`008256d0`), as the code
/// walks them: `visit` gets each node; it answers false to stop.
fn walk_list(e: &mut Engine, mut node: u32, mut visit: impl FnMut(&mut Engine, u32) -> bool) {
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        if !visit(e, node) {
            return;
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

/// The item of a list node (the word its slot holds).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(slot)
}

// Translated from 007e1970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no words): the surgery menu's class number `0x41e`.
pub fn fn_007e1970(_e: &mut Engine) -> u32 {
    SURGERY_MENU_CLASS
}

// Translated from 007e1980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e1980 - the C++ exception-unwinding frame (FS:[0] around the sound handles) is not translated.
/// `SurgeryMenu::Close` (Xbox PDB; no words): nothing without the menu's tile. With the
/// menu: plays `"UISurgeryExit"`, sets the tile's trait 0x1772 to 1, fades the menu out,
/// resets each limb (`ResetLimb` of each wound's actor value) and sets the interface
/// manager's menu tile's 0xfa9 to 255. Then `Shutdown3DElements`, and both sound
/// handles (+0x94, +0xa0) stopped and released.
pub fn surgery_menu_close(e: &mut Engine) {
    let class = fn_007e1970(e);
    let tile = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    if tile == 0 {
        return;
    }
    if e.global::<u32>(SURGERY_MENU_ME) != 0 {
        // The handle locals at `EBP - 0x1c` and `EBP - 0x2c`.
        e.with_stack(0x18, |e, frame| {
            let (sound, found) = (frame.addr() + 0xc, frame.addr());
            e.call(SOUND_HANDLE_CONSTRUCT, &args![sound]);
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let handle = e
                .call(
                    GET_SOUND_HANDLE_BY_NAME,
                    &args![audio, found, EXIT_SOUND, 0x121u32],
                )
                .u32();
            e.call(SOUND_HANDLE_ASSIGN, &args![sound, handle]);
            e.call(SOUND_HANDLE_DESTRUCT, &args![found]);
            e.call(SOUND_PLAY, &args![sound, 0u32]);
            e.call(TILE_SET_INT, &args![tile, 0x1772u32, 1u32]);
            let me = e.global::<u32>(SURGERY_MENU_ME);
            e.call(MENU_START_FADE_OUT, &args![me]);
            walk_list(e, LIMB_WOUNDS, |e, node| {
                let wound = node_item(e, node);
                let limb = e.mem.u32(wound + 4);
                surgery_menu_reset_limb(e, limb);
                true
            });
            let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
            let menu_tile = e.call(MANAGER_MENU_TILE, &args![manager]).u32();
            e.call(TILE_SET_INT, &args![menu_tile, 0xfa9u32, 0xffu32]);
            e.call(SOUND_HANDLE_DESTRUCT, &args![sound]);
        });
    }
    surgery_menu_shutdown_3d_elements(e);
    for (call, offset) in [
        (SOUND_STOP, 0x94u32),
        (SOUND_STOP, 0xa0),
        (SOUND_RELEASE, 0x94),
        (SOUND_RELEASE, 0xa0),
    ] {
        let me = e.global::<u32>(SURGERY_MENU_ME);
        e.call(call, &args![me + offset]);
    }
}

// Translated from 007e3750 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e3750 - the C++ exception-unwinding frame (FS:[0] around the fixed string) is not translated.
/// Unnamed (`thiscall`; the skin's pain shading): with the skin (`011dae5c`): its
/// `"BSFaceGenNiNodeSkinned"` child's geometry (slot +0x9c, then +0x100) gets value 0xd
/// (slot +0x114) set to the pain (+0xac) mapped from 0..100 onto 0..1 (`004b3ab0`), and
/// the skin is updated at the clock's time in seconds.
pub fn fn_007e3750(e: &mut Engine, this: Ptr) {
    if e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32() == 0 {
        return;
    }
    let skin = e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32();
    // The fixed string at `EBP - 0x20`, the update data at `EBP - 0x1c`.
    e.with_stack(0x10, |e, frame| {
        let (name, data) = (frame.addr(), frame.addr() + 4);
        let fixed = e
            .call(FIXED_STRING_CONSTRUCT, &args![name, SKIN_NODE_NAME])
            .u32();
        let child = e.vcall(skin, 0x9c, &args![fixed]).u32();
        let geometry = e.vcall(child, 0x100, &args![]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
        let pain = x87_quiet_bits(e.mem.u32(this.addr() + 0xac));
        let hundred = x87_quiet_bits(e.global::<u32>(HUNDRED_01016410));
        let (zero, one) = (x87_store32(0.0), x87_store32(1.0));
        let shade = e
            .call(INTERPOLATE_004B3AB0, &args![zero, one, zero, hundred, pain])
            .f64();
        e.vcall(geometry, 0x114, &args![0xdu32, x87_store32(shade)]);
        let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
        let seconds = x87_store32(x87_div(f64::from(now), e.global::<f64>(THOUSAND_DOUBLE)));
        e.call(
            UPDATE_DATA_INIT,
            &args![data, x87_quiet_bits(seconds), 1u32, 0u32],
        );
        let skin = e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32();
        e.call(UPDATE_OBJECT, &args![skin, data]);
    });
}

// Translated from 007e41f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, two integers; a cursor move): with the menu open and not burning
/// (+0x90), `00703080(first / 004dc1f0(), second / 004dc200())` as `float`s (the second
/// computed first).
pub fn fn_007e41f0(e: &mut Engine, first: i32, second: i32) {
    let me = e.global::<u32>(SURGERY_MENU_ME);
    if me == 0 || e.mem.u8(me + 0x90) != 0 {
        return;
    }
    let size = e.call(SCREEN_004DC200, &args![]).i32();
    let y = x87_store32(x87_div(f64::from(second), f64::from(size)));
    let size = e.call(SCREEN_004DC1F0, &args![]).i32();
    let x = x87_store32(x87_div(f64::from(first), f64::from(size)));
    e.call(
        SET_CURSOR_00703080,
        &args![x87_quiet_bits(x), x87_quiet_bits(y)],
    );
}

// Translated from 007e4260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::DoKey` (Xbox PDB, `RET 4`): `W` clicks id 6 and `S` id 5 (slot +0xc
/// with 0), either case; answers whether the key was used.
pub fn surgery_menu_do_key(e: &mut Engine, this: Ptr, key: u32) -> bool {
    if key == 0x57 || key == 0x77 {
        e.vcall(this.addr(), 0xc, &args![6u32, 0u32]);
        return true;
    }
    if key == 0x53 || key == 0x73 {
        e.vcall(this.addr(), 0xc, &args![5u32, 0u32]);
        return true;
    }
    false
}

// Translated from 007e4fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): the address of the 0x14-byte element `index` of the
/// array at +0x18.
pub fn fn_007e4fc0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0x14)
        .wrapping_add(e.mem.u32(this.addr() + 0x18))
}

// Translated from 007e5420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl: an array of `count` points of two `double`s and a point; a polygon
/// test): the crossing test of the point against the polygon's edges, toggled for each
/// edge the point's horizontal crosses on the side the edge runs; answers 1 when inside.
pub fn fn_007e5420(e: &mut Engine, points: u32, count: u32, point: u32) -> u32 {
    let double = |e: &Engine, at: u32| f64::from_bits(e.mem.u64(at));
    let x = x87_load64(e.mem.u64(point));
    let y = x87_load64(e.mem.u64(point + 8));
    let mut previous = count.wrapping_sub(1).wrapping_shl(4).wrapping_add(points);
    // FCOMP; TEST AH,0x41: whether the point's y is at most the vertex's.
    let mut previous_below = y <= double(e, previous + 8);
    let mut current = points;
    let mut inside = 0u32;
    let mut left = count.wrapping_add(1);
    loop {
        left = left.wrapping_sub(1);
        if left == 0 {
            break;
        }
        let current_below = y <= double(e, current + 8);
        if previous_below != current_below {
            let across = x87_mul(
                x87_sub(x87_load64(e.mem.u64(current + 8)), y),
                x87_sub(x87_load64(e.mem.u64(previous)), double(e, current)),
            );
            let along = x87_mul(
                x87_sub(x87_load64(e.mem.u64(current)), x),
                x87_sub(x87_load64(e.mem.u64(previous + 8)), double(e, current + 8)),
            );
            if (along <= across) == current_below {
                inside = u32::from(inside == 0);
            }
        }
        previous_below = current_below;
        previous = current;
        current = current.wrapping_add(0x10);
    }
    inside
}

// Translated from 007e5540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`; `this` not read): a limb's wound damage level from its
/// condition: 0 from 100, 1 from 75, 2 from 50, 3 from 25, else 4.
pub fn fn_007e5540(_e: &mut Engine, _this: Ptr, condition: i32) -> u32 {
    if condition >= 0x64 {
        0
    } else if condition >= 0x4b {
        1
    } else if condition >= 0x32 {
        2
    } else if condition >= 0x19 {
        3
    } else {
        4
    }
}

// Translated from 007e5720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::GetCurrentLimbWound` (Xbox PDB; `this` not read): the limb wound whose
/// actor value is the open menu's damaged part (+0xc0), or 0.
pub fn surgery_menu_get_current_limb_wound(e: &mut Engine, _this: Ptr) -> u32 {
    let mut found = 0;
    walk_list(e, LIMB_WOUNDS, |e, node| {
        let wound = node_item(e, node);
        let me = e.global::<u32>(SURGERY_MENU_ME);
        if e.mem.u32(wound + 4) == e.mem.u32(me + 0xc0) {
            found = node_item(e, node);
            return false;
        }
        true
    });
    found
}

// Translated from 007e5780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::LimbWound::ResetLimb` (Xbox PDB; `this` a limb wound): marks it reset
/// (+0xc), clears each of its sequences' lists (+4 of each item) and its own list, and
/// sets its damage level from the player's actor value (`007e5540` of the open menu).
pub fn surgery_menu_limb_wound_reset_limb(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 0xc, 1);
    let list = e.mem.u32(base);
    walk_list(e, list, |e, node| {
        let item = node_item(e, node);
        let inner = e.mem.u32(item + 4);
        e.call(LIST_CLEAR, &args![inner]);
        true
    });
    let list = e.mem.u32(base);
    e.call(LIST_CLEAR, &args![list]);
    let limb = e.mem.u32(base + 4);
    let player = e.global::<u32>(PLAYER);
    let condition = e.vcall(player + 0xa4, 8, &args![limb]).i32();
    let me = e.global::<u32>(SURGERY_MENU_ME);
    let level = fn_007e5540(e, Ptr::new(me), condition);
    e.mem.set_u32(base + 8, level);
}

// Translated from 007e5810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::ResetLimb` (Xbox PDB; cdecl, the limb's actor value): with the menu
/// open but not on top, resets the first limb wound of that actor value.
pub fn surgery_menu_reset_limb(e: &mut Engine, limb: u32) {
    if e.global::<u32>(SURGERY_MENU_ME) == 0
        || e.call(IS_TOP_MENU_ID, &args![SURGERY_MENU_CLASS]).bool()
    {
        return;
    }
    walk_list(e, LIMB_WOUNDS, |e, node| {
        let wound = node_item(e, node);
        if e.mem.u32(wound + 4) == limb {
            let wound = node_item(e, node);
            surgery_menu_limb_wound_reset_limb(e, Ptr::new(wound));
            return false;
        }
        true
    });
}

// Translated from 007e5890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::ResetAllLimbs` (Xbox PDB; no words): with the menu open, resets every
/// limb wound.
pub fn surgery_menu_reset_all_limbs(e: &mut Engine) {
    if e.global::<u32>(SURGERY_MENU_ME) == 0 {
        return;
    }
    walk_list(e, LIMB_WOUNDS, |e, node| {
        let wound = node_item(e, node);
        surgery_menu_limb_wound_reset_limb(e, Ptr::new(wound));
        true
    });
}

// Translated from 007e58e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::GetLimbPercentFromSequences` (Xbox PDB; `ST0`): without a current limb
/// wound, sets the value tile (+0x70, `"_Value"`) to 1.0 and the name tile (+0x4c, 0xfc4)
/// to the damaged part's actor value name, and answers 0. Otherwise one minus the sum of
/// the sequences' `00493770` (at least 0) over the sum of their lengths less
/// `00759450`.
pub fn surgery_menu_get_limb_percent_from_sequences(e: &mut Engine, this: Ptr) -> f32 {
    let base = this.addr();
    let wound = surgery_menu_get_current_limb_wound(e, this);
    if wound == 0 {
        let which = e.call(TEXT_TO_TRAIT, &args![VALUE_TRAIT_NAME]).u32();
        let tile = e.mem.u32(base + 0x70);
        e.call(TILE_SET_FLOAT, &args![tile, which, x87_store32(1.0), 1u32]);
        let limb = e.mem.u32(base + 0xc0);
        let name = e.call(ACTOR_VALUE_NAME, &args![limb]).u32();
        let tile = e.mem.u32(base + 0x4c);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, name, 1u32]);
        return 0.0;
    }
    let mut done = x87_store32(0.0);
    let mut total = x87_store32(0.0);
    let list = e.mem.u32(wound);
    walk_list(e, list, |e, node| {
        let item = node_item(e, node);
        let sequence = e.mem.u32(item);
        let value = e.call(SEQUENCE_00493770, &args![sequence]).f64();
        done = x87_store32(x87_add(value, x87_m32(done)));
        let length = e.call(SEQUENCE_LENGTH, &args![sequence]).f64();
        let start = e.call(SEQUENCE_00759450, &args![sequence]).f64();
        total = x87_store32(x87_add(x87_sub(length, start), x87_m32(total)));
        true
    });
    // FCOMP with 0.0; TEST AH,5: below 0 becomes 0.
    if x87_load32(done) < e.global::<f64>(ZERO_DOUBLE) {
        done = x87_store32(0.0);
    }
    done = x87_store32(x87_div(x87_load32(done), x87_m32(total)));
    let left = x87_store32(x87_sub(1.0, x87_load32(done)));
    f32::from_bits(x87_quiet_bits(left))
}

// Translated from 007e59f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::SetLimbSequences` (Xbox PDB, `RET 4`: a fraction): each sequence of the
/// open menu's current limb wound gets its length times the fraction as its phase
/// (`NiControllerSequence::SetPhase(.., 1)`); then the skin root (`011dade8`) is updated
/// at time 0.
pub fn surgery_menu_set_limb_sequences(e: &mut Engine, _this: Ptr, fraction: f32) {
    let me = e.global::<u32>(SURGERY_MENU_ME);
    let wound = surgery_menu_get_current_limb_wound(e, Ptr::new(me));
    let list = e.mem.u32(wound);
    walk_list(e, list, |e, node| {
        let item = node_item(e, node);
        let sequence = e.mem.u32(item);
        let length = e.call(SEQUENCE_LENGTH, &args![sequence]).f64();
        let phase = x87_store32(x87_mul(length, x87_m32(fraction.to_bits())));
        let item = node_item(e, node);
        let sequence = e.mem.u32(item);
        e.call(
            SEQUENCE_SET_PHASE,
            &args![sequence, x87_quiet_bits(phase), 1u32],
        );
        true
    });
    // The update data at `EBP - 0x10`.
    e.with_stack(0xc, |e, data| {
        e.call(UPDATE_DATA_INIT, &args![data, x87_store32(0.0), 1u32, 0u32]);
        let root = e.call(FIRST_WORD, &args![SKIN_ROOT_HOLDER_011DADE8]).u32();
        e.call(UPDATE_OBJECT, &args![root, data]);
    });
}

/// The camera holder (`011dadb4`, an `NiPointer`), `NiCamera::WorldPtToScreenPt`
/// (`00a6fc50`, `RET 0x10`) with the tolerance 1e-5 (`010718c0`), an object's world
/// translation (`0045bb80`), `InterfaceManager::GetScreenWidth` (`00715d40`, `ST0`) and
/// the height (`00715da0`, `ST0`), the point difference (`004e8880`, `RET 8`) and length
/// (`00589850`, `ST0`), `_ftol2` (`00ec62c0`), the cursor's position (`00747d00`,
/// `0073b020` on the interface manager), the point constructor (`006815c0`) and
/// `"CenterNode"` (`01077774`), `"FrameNode"` (`01077768`).
const CAMERA_HOLDER_011DADB4: u32 = 0x011d_adb4;
const WORLD_PT_TO_SCREEN_PT: u32 = 0x00a6_fc50;
const SCREEN_TOLERANCE_010718C0: u32 = 0x0107_18c0;
const WORLD_TRANSLATE_0045BB80: u32 = 0x0045_bb80;
const GET_SCREEN_WIDTH: u32 = 0x0071_5d40;
const GET_SCREEN_HEIGHT_00715DA0: u32 = 0x0071_5da0;
const POINT_DIFFERENCE_004E8880: u32 = 0x004e_8880;
const POINT_LENGTH_00589850: u32 = 0x0058_9850;
const FTOL2: u32 = 0x00ec_62c0;
const CURSOR_X_00747D00: u32 = 0x0074_7d00;
const CURSOR_Y_0073B020: u32 = 0x0073_b020;
const CENTER_NODE_NAME: u32 = 0x0107_7774;
const FRAME_NODE_NAME: u32 = 0x0107_7768;
/// The render: the renderer (`0043c4b0`) and its `007148c0` (`RET 8`), `0045c670`, the
/// culler (`008d80e0`), `00c4f270` (`RET 4`), `00709d50` (cdecl: the camera and 0.15 at
/// `01030ff4`), the scene nodes (`00450b80`, cdecl), the interface texture (`007fd640`),
/// the `NiPointer` constructor (`00633c90`, `RET 4`), assignment (`0066b0d0`, `RET 4`)
/// and destructor (`0045cec0`), the culler's accumulator (`004a0fd0`, `RET 4`),
/// `BSShaderUtil::AccumulateScene` (`00b6bee0`, cdecl), `00b6c0d0` (cdecl), `00c4f2d0`,
/// the app-culled setter (`00450f90`, `RET 4`).
const RENDERER: u32 = 0x0043_c4b0;
const RENDERER_007148C0: u32 = 0x0071_48c0;
const GETTER_0045C670: u32 = 0x0045_c670;
const CULLER_008D80E0: u32 = 0x008d_80e0;
const CULLER_00C4F270: u32 = 0x00c4_f270;
const CAMERA_00709D50: u32 = 0x0070_9d50;
const CAMERA_VALUE_01030FF4: u32 = 0x0103_0ff4;
const SCENE_NODE_00450B80: u32 = 0x0045_0b80;
const TEXTURE_007FD640: u32 = 0x007f_d640;
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const CULLER_SET_ACCUMULATOR: u32 = 0x004a_0fd0;
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
const RENDER_00B6C0D0: u32 = 0x00b6_c0d0;
const CULLER_00C4F2D0: u32 = 0x00c4_f2d0;
const SET_APP_CULLED: u32 = 0x0045_0f90;
/// A setting's value address (`00403e20`) and the skin placement settings (`011dae28`,
/// `011dadf0`, `011dae6c`, `011dadfc`), `"Bip01"` (`0101e460`), the translation setter
/// (`004bc1f0`, `RET 0xc`) and the scale setter (`00440490`, `RET 4`).
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_011DAE28: u32 = 0x011d_ae28;
const SETTING_011DADF0: u32 = 0x011d_adf0;
const SETTING_011DAE6C: u32 = 0x011d_ae6c;
const SETTING_011DADFC: u32 = 0x011d_adfc;
const BIP01_NAME: u32 = 0x0101_e460;
const SET_TRANSLATE: u32 = 0x004b_c1f0;
const SET_SCALE_00440490: u32 = 0x0044_0490;
/// The detach slot of a node (+0xe8) and `00b5d180` on the scene nodes,
/// `PlayerCharacter::CloneInventory3D` (`00950c20`, `RET 4`), the property `005d9660`
/// names, `NiAVObject::RemoveProperty` (`00a5b230`, `RET 4`) and `UpdateProperties`
/// (`00a5a040`).
const SCENE_00B5D180: u32 = 0x00b5_d180;
const CLONE_INVENTORY_3D: u32 = 0x0095_0c20;
const PROPERTY_005D9660: u32 = 0x005d_9660;
const REMOVE_PROPERTY: u32 = 0x00a5_b230;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// The largest float (`01016970`).
const LARGEST_01016970: u32 = 0x0101_6970;
/// The condition interpolation ends: 25.0 (`010181e8`), 50.0 (`0101b268`), 75.0
/// (`0102f0f8`), and 100.0 (`01017a40`, `double`).
const CONDITION_25: u32 = 0x0101_81e8;
const CONDITION_50: u32 = 0x0101_b268;
const CONDITION_75: u32 = 0x0102_f0f8;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
/// The controller manager (`0043b230`), the sequence lookup (`0047a520`, `RET 4`) of
/// `"Forward"` (`01048458`), `"Right"` (`0104843c`) and `"Left"` (`01048444`), the frame
/// time (`0084d030` on `011f6394`, `ST0`), `NiControllerSequence::Update` (`00a34ba0`,
/// `RET 8`), the parent walk (`004a8a90`), the type test `00653270` (cdecl) with
/// `011f36bc`, `NiObjectNET::GetController` (`00a5c570`, `RET 4`) with `011f36ac`,
/// `NiControllerManager::DeactivateAll` (`0048fef0`, `RET 4`), the sequence play
/// (`0047aab0`, `RET 0x18`), `0047aa40` (`RET 4`), `004954c0` and
/// `NiTimeController::StartAnimations` (`00a6d2d0`, cdecl).
const CONTROLLER_MANAGER_0043B230: u32 = 0x0043_b230;
const SEQUENCE_LOOKUP: u32 = 0x0047_a520;
const FORWARD_NAME: u32 = 0x0104_8458;
const RIGHT_NAME: u32 = 0x0104_843c;
const LEFT_NAME: u32 = 0x0104_8444;
const FRAME_TIME_0084D030: u32 = 0x0084_d030;
const SEQUENCE_UPDATE: u32 = 0x00a3_4ba0;
const PARENT_004A8A90: u32 = 0x004a_8a90;
const TYPE_TEST_00653270: u32 = 0x0065_3270;
const TYPE_011F36BC: u32 = 0x011f_36bc;
const GET_CONTROLLER: u32 = 0x00a5_c570;
const CONTROLLER_TYPE_011F36AC: u32 = 0x011f_36ac;
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
const PLAY_SEQUENCE: u32 = 0x0047_aab0;
const MANAGER_0047AA40: u32 = 0x0047_aa40;
const SEQUENCE_004954C0: u32 = 0x0049_54c0;
const START_ANIMATIONS: u32 = 0x00a6_d2d0;
/// `"ScarMesh"` (`01077780`).
const SCAR_MESH_NAME: u32 = 0x0107_7780;
/// The decal: the camera ray (`00a71080`, `RET 0x14`), the default hit point
/// (`011f426c`, three words), `NiPick::NiPick` (`00e98f20`, `RET 8`), `00458b30` (`RET
/// 4`), the target (`00705fc0`, `RET 4`), `NiPick::PickObjects` (`00e98e20`, `RET 0xc`),
/// the results (`00500940`), the record (`00968670`, `RET 4`), its point (`00413f40`),
/// 10.0 (`01020758`, `double`), `TESForm::GetFormByEditorID` (`00483a00`, cdecl) of
/// `"DecalSurgeryGood"` (`010777e0`) and `"DecalSurgeryBad"` (`010777d0`) cast
/// (`__RTDynamicCast`, `00ec43fb`, from `01183028` to `01187788`), the decal data's
/// constructor (`004a37b0`), the point from three floats (`00416870`, `RET 0xc`),
/// `004a4240` (`ST0`), the player's `008d6f30`, 0.5 (`01016248`), 20.0 (`01017868`),
/// the decal manager (`0049fef0`) and `BGSDecalManager::AddDecal` (`004a10d0`, `RET
/// 0xc`), `NiPick::~NiPick` (`00e98fa0`).
const CAMERA_RAY: u32 = 0x00a7_1080;
const DEFAULT_HIT_011F426C: u32 = 0x011f_426c;
const NI_PICK_CONSTRUCT: u32 = 0x00e9_8f20;
const PICK_00458B30: u32 = 0x0045_8b30;
const PICK_SET_TARGET: u32 = 0x0070_5fc0;
const PICK_OBJECTS: u32 = 0x00e9_8e20;
const PICK_RESULTS: u32 = 0x0050_0940;
const PICK_RECORD: u32 = 0x0096_8670;
const RECORD_POINT_00413F40: u32 = 0x0041_3f40;
const TEN_DOUBLE: u32 = 0x0102_0758;
const FORM_BY_EDITOR_ID: u32 = 0x0048_3a00;
const GOOD_DECAL_NAME: u32 = 0x0107_77e0;
const BAD_DECAL_NAME: u32 = 0x0107_77d0;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_01183028: u32 = 0x0118_3028;
const CAST_TARGET_01187788: u32 = 0x0118_7788;
const DECAL_DATA_CONSTRUCT: u32 = 0x004a_37b0;
const POINT_FROM_FLOATS: u32 = 0x0041_6870;
const DECAL_004A4240: u32 = 0x004a_4240;
const PLAYER_008D6F30: u32 = 0x008d_6f30;
const HALF_01016248: u32 = 0x0101_6248;
const TWENTY_01017868: u32 = 0x0101_7868;
const DECAL_MANAGER: u32 = 0x0049_fef0;
const ADD_DECAL: u32 = 0x004a_10d0;
const NI_PICK_DESTRUCT: u32 = 0x00e9_8fa0;

/// A named object of the frame geometry (+0xe0, slot +0x9c with a fixed string built in
/// the local at `local`).
fn frame_object(e: &mut Engine, this: Ptr, name: u32, local: u32) -> u32 {
    let fixed = e.call(FIXED_STRING_CONSTRUCT, &args![local, name]).u32();
    let frame = e.mem.u32(this.addr() + 0xe0);
    let object = e.vcall(frame, 0x9c, &args![fixed]).u32();
    e.call(FIXED_STRING_DESTRUCT, &args![local]);
    object
}

/// The screen point of `object` (`WorldPtToScreenPt` of its world translation through
/// the camera, tolerance 1e-5) into the two floats at `x` and `y`.
fn screen_point(e: &mut Engine, object: u32, x: u32, y: u32) {
    let tolerance = x87_quiet_bits(e.global::<u32>(SCREEN_TOLERANCE_010718C0));
    let point = e.call(WORLD_TRANSLATE_0045BB80, &args![object]).u32();
    let camera = e.call(FIRST_WORD, &args![CAMERA_HOLDER_011DADB4]).u32();
    e.call(
        WORLD_PT_TO_SCREEN_PT,
        &args![camera, point, x, y, tolerance],
    );
}

// Translated from 007e4540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e4540 - the C++ exception-unwinding frame (FS:[0] around the fixed strings) is not translated.
/// Unnamed (`thiscall`; whether the cursor is over the wound window): true with a
/// gamepad. Otherwise the frame's `"CenterNode"` and `"FrameNode"` are taken to screen
/// pixels (`x` times the screen width, `1 - y` times the height); their centre and the
/// distance between them as the radius, or (800, 475) and 325 without them; true when
/// the cursor is within the radius of the centre (`!(radius < distance)`).
pub fn fn_007e4540(e: &mut Engine, this: Ptr) -> bool {
    e.call(INTERFACE_MANAGER, &args![]);
    if e.call(GAMEPAD_IN_USE_004B71D0, &args![]).bool() {
        return true;
    }
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x80, |e, frame| {
        let ebp = frame.addr() + 0x80;
        let local = |offset: u32| ebp - offset;
        let center = frame_object(e, this, CENTER_NODE_NAME, local(0x40));
        let edge = frame_object(e, this, FRAME_NODE_NAME, local(0x44));
        let (center_x, edge_x, difference) = (local(0x34), local(0x3c), local(0x4c));
        let (left, top, radius) = if center != 0 && edge != 0 {
            e.call(NODE_SELF, &args![center_x]);
            e.call(NODE_SELF, &args![edge_x]);
            screen_point(e, center, center_x, center_x + 4);
            screen_point(e, edge, edge_x, edge_x + 4);
            for point in [center_x, edge_x] {
                let width = e.call(GET_SCREEN_WIDTH, &args![]).f64();
                let x = x87_store32(x87_mul(width, x87_m32(e.mem.u32(point))));
                e.mem.set_u32(point, x);
                let flipped = x87_sub(1.0, x87_load32(e.mem.u32(point + 4)));
                let height = e.call(GET_SCREEN_HEIGHT_00715DA0, &args![]).f64();
                e.mem
                    .set_u32(point + 4, x87_store32(x87_mul(height, flipped)));
            }
            let x = x87_load32(e.mem.u32(center_x));
            let left = e.call(FTOL2, &args![x]).i32();
            let y = x87_load32(e.mem.u32(center_x + 4));
            let top = e.call(FTOL2, &args![y]).i32();
            let delta = e
                .call(
                    POINT_DIFFERENCE_004E8880,
                    &args![center_x, difference, edge_x],
                )
                .u32();
            let length = e.call(POINT_LENGTH_00589850, &args![delta]).f64();
            let radius = e.call(FTOL2, &args![length]).i32();
            (left, top, radius)
        } else {
            (0x320, 0x1db, 0x145)
        };
        let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
        let cursor = local(0x1c);
        e.call(NODE_SELF, &args![cursor]);
        let x = e
            .call(CURSOR_X_00747D00, &args![manager])
            .i32()
            .wrapping_sub(left);
        e.mem.set_u32(cursor, x87_store32(f64::from(x)));
        let y = e
            .call(CURSOR_Y_0073B020, &args![manager])
            .i32()
            .wrapping_sub(top);
        e.mem.set_u32(cursor + 4, x87_store32(f64::from(y)));
        let distance = e.call(POINT_LENGTH_00589850, &args![cursor]).f64();
        // FCOMPP of the radius against the distance; TEST AH,1: outside when below.
        !(f64::from(radius) < distance)
    })
}

// Translated from 007e47a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e47a0 - the C++ exception-unwinding frame (FS:[0] around the texture pointer) is not translated.
/// Unnamed (no words; the surgery render): with the skin root (`011dade8`) and the skin
/// (`011dae5c`): `007148c0(renderer, 0, 4)`; the culler (`008d80e0` of `0045c670()`,
/// `00c4f270(1)`), the camera (`011dadb4`) given 0.15 (`00709d50`); with a local pointer
/// to the interface texture as the accumulator, the scene node 1 is accumulated and
/// rendered, the skin shown (app-culled 0), then the scene node 3; the accumulator
/// cleared, the pointer released and `00c4f2d0` closes the culler.
pub fn fn_007e47a0(e: &mut Engine) {
    if e.call(FIRST_WORD, &args![SKIN_ROOT_HOLDER_011DADE8]).u32() == 0
        || e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32() == 0
    {
        return;
    }
    let renderer = e.call(RENDERER, &args![]).u32();
    e.call(RENDERER_007148C0, &args![renderer, 0u32, 4u32]);
    let object = e.call(GETTER_0045C670, &args![]).u32();
    let culler = e.call(CULLER_008D80E0, &args![object]).u32();
    e.call(CULLER_00C4F270, &args![culler, 1u32]);
    let camera = e.call(FIRST_WORD, &args![CAMERA_HOLDER_011DADB4]).u32();
    let value = x87_quiet_bits(e.global::<u32>(CAMERA_VALUE_01030FF4));
    e.call(CAMERA_00709D50, &args![camera, value]);
    let scene = e.call(SCENE_NODE_00450B80, &args![1u32]).u32();
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let texture = e.call(TEXTURE_007FD640, &args![manager]).u32();
    // The local `NiPointer` at `EBP - 0x20`.
    e.with_stack(4, |e, pointer| {
        e.call(NI_POINTER_CONSTRUCT, &args![pointer, texture]);
        let held = e.call(FIRST_WORD, &args![pointer]).u32();
        e.call(CULLER_SET_ACCUMULATOR, &args![culler, held]);
        e.call(ACCUMULATE_SCENE, &args![camera, scene, culler]);
        let held = e.call(FIRST_WORD, &args![pointer]).u32();
        e.call(RENDER_00B6C0D0, &args![camera, held, 0u32]);
        let skin = e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32();
        e.call(SET_APP_CULLED, &args![skin, 0u32]);
        let scene = e.call(SCENE_NODE_00450B80, &args![3u32]).u32();
        e.call(ACCUMULATE_SCENE, &args![camera, scene, culler]);
        let held = e.call(FIRST_WORD, &args![pointer]).u32();
        e.call(RENDER_00B6C0D0, &args![camera, held, 0u32]);
        e.call(CULLER_SET_ACCUMULATOR, &args![culler, 0u32]);
        e.call(NI_POINTER_ASSIGN, &args![pointer, 0u32]);
        e.call(CULLER_00C4F2D0, &args![culler]);
        e.call(NI_POINTER_DESTRUCT, &args![pointer]);
    });
}

/// A setting's float (`00403e20`, then `FLD`).
fn setting_float(e: &mut Engine, setting: u32) -> f64 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_load32(e.mem.u32(value))
}

// Translated from 007e4fe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e4fe0 - the C++ exception-unwinding frame (FS:[0] around the fixed strings) is not translated.
/// Unnamed (cdecl, three `float`s; places the skin): with the skin (`011dae5c`), its
/// `"Bip01"` gets the translation (`-011dae6c`, `011dadf0 - second`, `011dae28 - first`)
/// (as the code pushes them) and the scale `011dadfc * third`; the skin is updated.
pub fn fn_007e4fe0(e: &mut Engine, first: f32, second: f32, third: f32) {
    if e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32() == 0 {
        return;
    }
    let skin = e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32();
    // The fixed strings at `EBP - 0x1c` and `EBP - 0x20`, the update data at `EBP - 0x18`.
    e.with_stack(0x14, |e, frame| {
        let (data, name, scale_name) = (frame.addr() + 8, frame.addr() + 4, frame.addr());
        let a = x87_store32(x87_sub(
            setting_float(e, SETTING_011DAE28),
            x87_load32(first.to_bits()),
        ));
        let b = x87_store32(x87_sub(
            setting_float(e, SETTING_011DADF0),
            x87_load32(second.to_bits()),
        ));
        let c = x87_store32(-setting_float(e, SETTING_011DAE6C));
        let fixed = e
            .call(FIXED_STRING_CONSTRUCT, &args![name, BIP01_NAME])
            .u32();
        let bone = e.vcall(skin, 0x9c, &args![fixed]).u32();
        e.call(
            SET_TRANSLATE,
            &args![bone, c, x87_quiet_bits(b), x87_quiet_bits(a)],
        );
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
        let skin = e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32();
        let scale = x87_store32(x87_mul(
            setting_float(e, SETTING_011DADFC),
            x87_load32(third.to_bits()),
        ));
        let fixed = e
            .call(FIXED_STRING_CONSTRUCT, &args![scale_name, BIP01_NAME])
            .u32();
        let bone = e.vcall(skin, 0x9c, &args![fixed]).u32();
        e.call(SET_SCALE_00440490, &args![bone, x87_quiet_bits(scale)]);
        e.call(FIXED_STRING_DESTRUCT, &args![scale_name]);
        e.call(UPDATE_DATA_INIT, &args![data, x87_store32(0.0), 0u32, 0u32]);
        let skin = e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32();
        e.call(UPDATE_OBJECT, &args![skin, data]);
    });
}

// Translated from 007e5170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::Shutdown3DElements` (Xbox PDB; no words): detaches the skin root from
/// scene node 1 and the skin from node 3 (slot +0xe8), `00b5d180` on both nodes, the
/// player's inventory 3D cloned again (0), the skin's `005d9660` property removed and its
/// properties updated, and the camera, skin root and skin holders cleared.
pub fn surgery_menu_shutdown_3d_elements(e: &mut Engine) {
    for (node, holder) in [(1u32, SKIN_ROOT_HOLDER_011DADE8), (3, SKIN_HOLDER_011DAE5C)] {
        let scene = e.call(SCENE_NODE_00450B80, &args![node]).u32();
        let child = e.call(FIRST_WORD, &args![holder]).u32();
        e.vcall(scene, 0xe8, &args![child]);
    }
    for node in [1u32, 3] {
        let scene = e.call(SCENE_NODE_00450B80, &args![node]).u32();
        e.call(SCENE_00B5D180, &args![scene]);
    }
    let player = e.global::<u32>(PLAYER);
    e.call(CLONE_INVENTORY_3D, &args![player, 0u32]);
    let property = e.call(PROPERTY_005D9660, &args![]).u32();
    let skin = e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32();
    e.call(REMOVE_PROPERTY, &args![skin, property]);
    let skin = e.call(FIRST_WORD, &args![SKIN_HOLDER_011DAE5C]).u32();
    e.call(UPDATE_PROPERTIES, &args![skin]);
    for holder in [
        CAMERA_HOLDER_011DADB4,
        SKIN_ROOT_HOLDER_011DADE8,
        SKIN_HOLDER_011DAE5C,
    ] {
        e.call(NI_POINTER_ASSIGN, &args![holder, 0u32]);
    }
}

/// The screen point of the node's object (`006815c0` of the node) into `point`, its
/// `y` flipped (`1 - y`), and its distance from `cursor` (`004e8880` into `scratch`).
fn node_distance(e: &mut Engine, node: u32, point: u32, cursor: u32, scratch: u32) -> f64 {
    let object = node_item(e, node);
    screen_point(e, object, point, point + 4);
    let flipped = x87_store32(x87_sub(1.0, x87_load32(e.mem.u32(point + 4))));
    e.mem.set_u32(point + 4, flipped);
    let delta = e
        .call(POINT_DIFFERENCE_004E8880, &args![point, scratch, cursor])
        .u32();
    e.call(POINT_LENGTH_00589850, &args![delta]).f64()
}

// Translated from 007e5250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::SortSequenceByCursorDistance` (Xbox PDB; cdecl, two sequence records
/// with an object list at +4): 0 without the menu. The cursor (+0x74) over the screen
/// size; the smallest screen distance of the first record's objects to it (starting at
/// the largest float); 1 when any object of the second record is nearer, else -1. Both
/// walks stop without the camera (`011dadb4`).
pub fn surgery_menu_sort_sequence_by_cursor_distance(
    e: &mut Engine,
    first: u32,
    second: u32,
) -> i32 {
    if e.global::<u32>(SURGERY_MENU_ME) == 0 {
        return 0;
    }
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x30, |e, frame| {
        let ebp = frame.addr() + 0x30;
        let local = |offset: u32| ebp - offset;
        let best = local(0x18);
        let max = x87_quiet_bits(e.global::<u32>(LARGEST_01016970));
        e.mem.set_u32(best, max);
        let point = local(8);
        e.call(NODE_SELF, &args![point]);
        let cursor = local(0x10);
        let me = e.global::<u32>(SURGERY_MENU_ME);
        let (x, y) = (e.mem.u32(me + 0x74), e.mem.u32(me + 0x78));
        e.mem.set_u32(cursor, x);
        e.mem.set_u32(cursor + 4, y);
        let width = e.call(GET_SCREEN_WIDTH, &args![]).f64();
        let x = x87_store32(x87_div(x87_m32(e.mem.u32(cursor)), width));
        e.mem.set_u32(cursor, x);
        let height = e.call(GET_SCREEN_HEIGHT_00715DA0, &args![]).f64();
        let y = x87_store32(x87_div(x87_m32(e.mem.u32(cursor + 4)), height));
        e.mem.set_u32(cursor + 4, y);
        let camera_set =
            |e: &mut Engine| e.call(FIRST_WORD, &args![CAMERA_HOLDER_011DADB4]).u32() != 0;
        let mut node = e.mem.u32(first + 4);
        while camera_set(e) && node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let distance = node_distance(e, node, point, cursor, local(0x20));
            // FCOMPP of the best against the distance; TEST AH,0x41: above it only.
            if x87_load32(e.mem.u32(best)) > distance {
                let delta = e
                    .call(
                        POINT_DIFFERENCE_004E8880,
                        &args![point, local(0x28), cursor],
                    )
                    .u32();
                let length = e.call(POINT_LENGTH_00589850, &args![delta]).f64();
                e.mem.set_u32(best, x87_store32(length));
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        let mut node = e.mem.u32(second + 4);
        while camera_set(e) && node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let distance = node_distance(e, node, point, cursor, local(0x30));
            if x87_load32(e.mem.u32(best)) > distance {
                return 1;
            }
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        -1
    })
}

// Translated from 007e5590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::ShowLimbCondition` (Xbox PDB): the healed fraction (1 less
/// `GetLimbPercentFromSequences`) maps the open menu's damage level (+0xc4; jump table
/// `007e570c`) onto a condition: from 100 down to 0, 25, 50 or 75 for levels 4, 3, 2, 1
/// (`004b3ab0`), 100 for level 0. The value tile (+0x70, `"_Value"`) gets the condition
/// (truncated) over 100 and the name tile (+0x4c, 0xfc4) the part's actor value name;
/// answers the truncated condition.
pub fn surgery_menu_show_limb_condition(e: &mut Engine, this: Ptr) -> i32 {
    let base = this.addr();
    let percent = f64::from(surgery_menu_get_limb_percent_from_sequences(e, this));
    let healed = x87_store32(x87_sub(1.0, percent));
    let me = e.global::<u32>(SURGERY_MENU_ME);
    let hundred = x87_quiet_bits(e.global::<u32>(HUNDRED_01016410));
    let (zero, one) = (x87_store32(0.0), x87_store32(1.0));
    let interpolate = |e: &mut Engine, low: u32| {
        let value = e
            .call(
                INTERPOLATE_004B3AB0,
                &args![hundred, low, zero, one, healed],
            )
            .f64();
        x87_store32(value)
    };
    let condition = match e.mem.u32(me + 0xc4) {
        0 => hundred,
        1 => {
            let low = x87_quiet_bits(e.global::<u32>(CONDITION_75));
            interpolate(e, low)
        }
        2 => {
            let low = x87_quiet_bits(e.global::<u32>(CONDITION_50));
            interpolate(e, low)
        }
        3 => {
            let low = x87_quiet_bits(e.global::<u32>(CONDITION_25));
            interpolate(e, low)
        }
        4 => interpolate(e, zero),
        _ => zero,
    };
    let whole = e.call(FTOL2, &args![x87_load32(condition)]).i32();
    let shown = x87_store32(x87_div(f64::from(whole), e.global::<f64>(HUNDRED_DOUBLE)));
    let which = e.call(TEXT_TO_TRAIT, &args![VALUE_TRAIT_NAME]).u32();
    let tile = e.mem.u32(base + 0x70);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, which, x87_quiet_bits(shown), 1u32],
    );
    let limb = e.mem.u32(base + 0xc0);
    let name = e.call(ACTOR_VALUE_NAME, &args![limb]).u32();
    let tile = e.mem.u32(base + 0x4c);
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, name, 1u32]);
    whole
}

/// The animation's sequence of the controller manager by the state (+0x28): 1
/// `"Forward"`, 2 `"Right"`, 3 `"Left"` (through a fixed string in the local at
/// `local`), else none.
fn animation_sequence(e: &mut Engine, this: Ptr, manager: u32, local: u32) -> u32 {
    let name = match e.mem.u32(this.addr() + 0x28) {
        1 => FORWARD_NAME,
        2 => RIGHT_NAME,
        3 => LEFT_NAME,
        _ => return 0,
    };
    let fixed = e.call(FIXED_STRING_CONSTRUCT, &args![local, name]).u32();
    let sequence = e.call(SEQUENCE_LOOKUP, &args![manager, fixed]).u32();
    e.call(FIXED_STRING_DESTRUCT, &args![local]);
    sequence
}

// Translated from 007e5a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e5a90 - the C++ exception-unwinding frame (FS:[0] around the fixed strings) is not translated.
/// `SurgeryMenu::UpdateAnimations` (Xbox PDB): with the frame's controller manager
/// (`0043b230` of +0xe0): the animation's sequence; the animation time (+0xec) advances
/// by the frame time; with a sequence it is updated to that time
/// (`NiControllerSequence::Update(time, 1)`) and the first object up the manager's
/// parents that passes the type test (`00653270` with `011f36bc`) gets an update at
/// that time (slot +0x94). The frame geometry is then updated at time 0.
pub fn surgery_menu_update_animations(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let frame = e.mem.u32(base + 0xe0);
    let manager = e.call(CONTROLLER_MANAGER_0043B230, &args![frame]).u32();
    // The frame below `EBP`: the fixed strings, then the update data.
    e.with_stack(0x30, |e, locals| {
        let (name, data, final_data) = (locals.addr(), locals.addr() + 0xc, locals.addr() + 0x1c);
        if manager != 0 {
            let sequence = animation_sequence(e, this, manager, name);
            let elapsed = e.call(FRAME_TIME_0084D030, &args![CLOCK_011F6394]).f64();
            let time = x87_store32(x87_add(elapsed, x87_m32(e.mem.u32(base + 0xec))));
            e.mem.set_u32(base + 0xec, time);
            if sequence != 0 {
                let time = x87_quiet_bits(e.mem.u32(base + 0xec));
                e.call(SEQUENCE_UPDATE, &args![sequence, time, 1u32]);
                let mut node = manager;
                while node != 0 {
                    if e.call(TYPE_TEST_00653270, &args![TYPE_011F36BC, node])
                        .u32()
                        != 0
                    {
                        let time = x87_quiet_bits(e.mem.u32(base + 0xec));
                        e.call(UPDATE_DATA_INIT, &args![data, time, 0u32, 0u32]);
                        e.vcall(node, 0x94, &args![data]);
                        break;
                    }
                    node = e.call(PARENT_004A8A90, &args![node]).u32();
                }
            }
        }
        e.call(
            UPDATE_DATA_INIT,
            &args![final_data, x87_store32(0.0), 0u32, 0u32],
        );
        let frame = e.mem.u32(base + 0xe0);
        e.call(UPDATE_OBJECT, &args![frame, final_data]);
    });
}

// Translated from 007e5c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e5c80 - the C++ exception-unwinding frame (FS:[0] around the fixed strings) is not translated.
/// `SurgeryMenu::SetAnimation` (Xbox PDB, `RET 4`): keeps the animation (+0x28) and
/// restarts its time (+0xec); with the open menu's frame controller (`GetController`)
/// and the animation's sequence: deactivates all, plays it (`0047aab0(seq, 0, 1, 1.0,
/// 0.0, 0)`), `0047aa40(1)` and `004954c0` on the sequence. The frame's animations are
/// started.
pub fn surgery_menu_set_animation(e: &mut Engine, this: Ptr, animation: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 0x28, animation);
    e.mem.set_u32(base + 0xec, x87_store32(0.0));
    let me = e.global::<u32>(SURGERY_MENU_ME);
    let frame = e.mem.u32(me + 0xe0);
    let controller = e
        .call(GET_CONTROLLER, &args![frame, CONTROLLER_TYPE_011F36AC])
        .u32();
    if controller != 0 {
        let sequence = e.with_stack(4, |e, name| {
            animation_sequence(e, this, controller, name.addr())
        });
        if sequence != 0 {
            e.call(DEACTIVATE_ALL, &args![controller, x87_store32(0.0)]);
            e.call(
                PLAY_SEQUENCE,
                &args![
                    controller,
                    sequence,
                    0u32,
                    1u32,
                    x87_store32(1.0),
                    x87_store32(0.0),
                    0u32
                ],
            );
            e.call(MANAGER_0047AA40, &args![controller, 1u32]);
            e.call(SEQUENCE_004954C0, &args![sequence]);
        }
    }
    let me = e.global::<u32>(SURGERY_MENU_ME);
    let frame = e.mem.u32(me + 0xe0);
    e.call(START_ANIMATIONS, &args![frame]);
}

// Translated from 007e5e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`; the skin geometry index of a limb's actor value; tables
/// `007e5e98`/`007e5eb8`): -1 gives 6; 0x19 to 0x1e give 5, 0, 1, 3, 2, 4; others 0.
pub fn fn_007e5e30(_e: &mut Engine, _this: Ptr, limb: i32) -> u32 {
    match limb {
        -1 => 6,
        0x19 => 5,
        0x1a => 0,
        0x1b => 1,
        0x1c => 3,
        0x1d => 2,
        0x1e => 4,
        _ => 0,
    }
}

// Translated from 007e5ee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e5ee0 - the C++ exception-unwinding frame (FS:[0] around the fixed string) is not translated.
/// `SurgeryMenu::SwitchSkinGeometry` (Xbox PDB, `RET 8`: the old and new limbs): the skin
/// root (`011dade8`) detaches the old limb's skin geometry (`pSkinGeometry` +0xc8 of the
/// open menu, slot +0xe8) and attaches the new one's (slot +0xdc with 1); the open menu's
/// scar (+0xe8) is the root's `"ScarMesh"`.
pub fn surgery_menu_switch_skin_geometry(e: &mut Engine, this: Ptr, old: i32, new: i32) {
    let skin_of = |e: &mut Engine, limb: i32| {
        let index = fn_007e5e30(e, this, limb);
        let me = e.global::<u32>(SURGERY_MENU_ME);
        e.call(
            FIRST_WORD,
            &args![me.wrapping_add(0xc8).wrapping_add(index.wrapping_mul(4))],
        )
        .u32()
    };
    let root = e.call(FIRST_WORD, &args![SKIN_ROOT_HOLDER_011DADE8]).u32();
    let old_skin = skin_of(e, old);
    e.vcall(root, 0xe8, &args![old_skin]);
    let root = e.call(FIRST_WORD, &args![SKIN_ROOT_HOLDER_011DADE8]).u32();
    let new_skin = skin_of(e, new);
    e.vcall(root, 0xdc, &args![new_skin, 1u32]);
    let root = e.call(FIRST_WORD, &args![SKIN_ROOT_HOLDER_011DADE8]).u32();
    // The fixed string at `EBP - 0x10`.
    e.with_stack(4, |e, name| {
        let fixed = e
            .call(FIXED_STRING_CONSTRUCT, &args![name, SCAR_MESH_NAME])
            .u32();
        let scar = e.vcall(root, 0x9c, &args![fixed]).u32();
        let me = e.global::<u32>(SURGERY_MENU_ME);
        e.mem.set_u32(me + 0xe8, scar);
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
    });
}

// Translated from 007e5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e5ff0 - the C++ exception-unwinding frame (FS:[0] around the pick) is not translated.
/// `SurgeryMenu::PlaceDecal` (Xbox PDB, `RET 0xc`: the screen position as two fractions
/// and a `bool` for a good decal): the camera's ray through the pixel (the fractions times
/// the screen size, truncated); the default point (`011f426c`) or, when the ray picks the
/// open menu's scar, the first hit's point raised by 10; the decal data (the point, up
/// (0, 0, 1), the scar, `004a4240`, the `"DecalSurgeryGood"` or `"DecalSurgeryBad"`
/// texture set, the player's `008d6f30`, sizes 0.5, 0.5 and 20, a zero angle, the side
/// (1, 0, 0), 75, flags 1, 1, 0) is added to the decal manager (`AddDecal(.., 1, 1)`).
pub fn surgery_menu_place_decal(e: &mut Engine, _this: Ptr, x: f32, y: f32, good: u8) {
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x120, |e, frame| {
        let ebp = frame.addr() + 0x120;
        let local = |offset: u32| ebp - offset;
        let (origin, direction, pick) = (local(0xe8), local(0xa8), local(0xdc));
        let (hit, decal) = (local(0x9c), local(0x8c));
        e.call(NODE_SELF, &args![origin]);
        e.call(NODE_SELF, &args![direction]);
        let width = e.call(SCREEN_004DC1F0, &args![]).i32();
        let pixel_x = x87_store32(x87_mul(f64::from(width), x87_m32(x.to_bits())));
        let height = e.call(SCREEN_004DC200, &args![]).i32();
        let pixel_y = x87_store32(x87_mul(f64::from(height), x87_m32(y.to_bits())));
        let row = e.call(FTOL2, &args![x87_load32(pixel_y)]).u32();
        let column = e.call(FTOL2, &args![x87_load32(pixel_x)]).u32();
        let camera = e.call(FIRST_WORD, &args![CAMERA_HOLDER_011DADB4]).u32();
        e.call(
            CAMERA_RAY,
            &args![camera, column, row, origin, direction, 0u32],
        );
        for i in 0..3 {
            let word = e.global::<u32>(DEFAULT_HIT_011F426C + 4 * i);
            e.mem.set_u32(hit + 4 * i, word);
        }
        e.call(NI_PICK_CONSTRUCT, &args![pick, 0u32, 8u32]);
        e.call(PICK_00458B30, &args![pick, 0u32]);
        let me = e.global::<u32>(SURGERY_MENU_ME);
        let scar = e.mem.u32(me + 0xe8);
        e.call(PICK_SET_TARGET, &args![pick, scar]);
        if e.call(PICK_OBJECTS, &args![pick, origin, direction, 0u32])
            .bool()
        {
            let results = e.call(PICK_RESULTS, &args![pick]).u32();
            let record = e.call(PICK_RECORD, &args![results, 0u32]).u32();
            if record != 0 {
                let point = e.call(RECORD_POINT_00413F40, &args![record]).u32();
                for i in 0..3 {
                    let word = e.mem.u32(point + 4 * i);
                    e.mem.set_u32(hit + 4 * i, word);
                }
                let raised = x87_store32(x87_add(
                    x87_load32(e.mem.u32(hit + 8)),
                    e.global::<f64>(TEN_DOUBLE),
                ));
                e.mem.set_u32(hit + 8, raised);
            }
        }
        let name = if good != 0 {
            GOOD_DECAL_NAME
        } else {
            BAD_DECAL_NAME
        };
        let form = e.call(FORM_BY_EDITOR_ID, &args![name]).u32();
        let texture_set = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, CAST_SOURCE_01183028, CAST_TARGET_01187788, 0u32],
            )
            .u32();
        e.call(DECAL_DATA_CONSTRUCT, &args![decal]);
        for i in 0..3 {
            let word = e.mem.u32(hit + 4 * i);
            e.mem.set_u32(decal + 4 * i, word);
        }
        let (zero, one) = (x87_store32(0.0), x87_store32(1.0));
        let up = e
            .call(POINT_FROM_FLOATS, &args![local(0x104), zero, zero, one])
            .u32();
        for i in 0..3 {
            let word = e.mem.u32(up + 4 * i);
            e.mem.set_u32(local(0x80) + 4 * i, word);
        }
        let me = e.global::<u32>(SURGERY_MENU_ME);
        let scar = e.mem.u32(me + 0xe8);
        e.mem.set_u32(local(0x64), scar);
        let value = e.call(DECAL_004A4240, &args![]).f64();
        e.mem.set_u32(local(0x48), x87_store32(value));
        e.mem.set_u32(local(0x5c), texture_set);
        e.mem.set_u8(local(0x1a), 0);
        let player = e.global::<u32>(PLAYER);
        let word = e.call(PLAYER_008D6F30, &args![player]).u32();
        e.mem.set_u32(local(0x44), word);
        let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
        e.mem.set_u32(local(0x54), half);
        e.mem.set_u32(local(0x50), half);
        let twenty = x87_quiet_bits(e.global::<u32>(TWENTY_01017868));
        e.mem.set_u32(local(0x4c), twenty);
        e.mem.set_u32(local(0x30), zero);
        let side = e
            .call(POINT_FROM_FLOATS, &args![local(0x110), one, zero, zero])
            .u32();
        for i in 0..3 {
            let word = e.mem.u32(side + 4 * i);
            e.mem.set_u32(local(0x2c) + 4 * i, word);
        }
        let seventy_five = x87_quiet_bits(e.global::<u32>(CONDITION_75));
        e.mem.set_u32(local(0x34), seventy_five);
        e.mem.set_u8(local(0x18), 1);
        e.mem.set_u8(local(0x17), 1);
        e.mem.set_u8(local(0x15), 0);
        let manager = e.call(DECAL_MANAGER, &args![]).u32();
        e.call(ADD_DECAL, &args![manager, decal, 1u32, 1u32]);
        e.call(NI_PICK_DESTRUCT, &args![pick]);
    });
}

/// The vector normalisation (`004e7560`, `ST0`, the result dropped) and scale
/// (`006ad180`, `RET 4`, a `float`).
const POINT_NORMALIZE_004E7560: u32 = 0x004e_7560;
const POINT_SCALE_006AD180: u32 = 0x006a_d180;

// Translated from 007e42c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e42c0 - the C++ exception-unwinding frame (FS:[0] around the fixed strings) is not translated.
/// `SurgeryMenu::PlaceToolInWindow` (Xbox PDB, `RET 8`: the cursor's pixel position):
/// the wound window's centre and radius as in `007e4540` (from `"CenterNode"` and
/// `"FrameNode"`, or (800, 475) and 325); the offset of the position from the centre,
/// cut to the radius when beyond it (normalised and scaled), places the tool tile
/// (+0x44, traits 0xfa1 and 0xfa2).
pub fn surgery_menu_place_tool_in_window(e: &mut Engine, this: Ptr, x: i32, y: i32) {
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x80, |e, frame| {
        let ebp = frame.addr() + 0x80;
        let local = |offset: u32| ebp - offset;
        let center = frame_object(e, this, CENTER_NODE_NAME, local(0x3c));
        let edge = frame_object(e, this, FRAME_NODE_NAME, local(0x40));
        let (center_x, edge_x, difference) = (local(0x30), local(0x38), local(0x48));
        let (left, top, radius) = if center != 0 && edge != 0 {
            e.call(NODE_SELF, &args![center_x]);
            e.call(NODE_SELF, &args![edge_x]);
            screen_point(e, center, center_x, center_x + 4);
            screen_point(e, edge, edge_x, edge_x + 4);
            for point in [center_x, edge_x] {
                let width = e.call(GET_SCREEN_WIDTH, &args![]).f64();
                let x = x87_store32(x87_mul(width, x87_m32(e.mem.u32(point))));
                e.mem.set_u32(point, x);
                let flipped = x87_sub(1.0, x87_load32(e.mem.u32(point + 4)));
                let height = e.call(GET_SCREEN_HEIGHT_00715DA0, &args![]).f64();
                e.mem
                    .set_u32(point + 4, x87_store32(x87_mul(height, flipped)));
            }
            let x = x87_load32(e.mem.u32(center_x));
            let left = e.call(FTOL2, &args![x]).i32();
            let y = x87_load32(e.mem.u32(center_x + 4));
            let top = e.call(FTOL2, &args![y]).i32();
            let delta = e
                .call(
                    POINT_DIFFERENCE_004E8880,
                    &args![center_x, difference, edge_x],
                )
                .u32();
            let length = e.call(POINT_LENGTH_00589850, &args![delta]).f64();
            let radius = e.call(FTOL2, &args![length]).i32();
            (left, top, radius)
        } else {
            (0x320, 0x1db, 0x145)
        };
        let offset = local(0x18);
        e.call(NODE_SELF, &args![offset]);
        e.mem
            .set_u32(offset, x87_store32(f64::from(x.wrapping_sub(left))));
        e.mem
            .set_u32(offset + 4, x87_store32(f64::from(y.wrapping_sub(top))));
        let distance = e.call(POINT_LENGTH_00589850, &args![offset]).f64();
        // FCOMPP of the radius against the distance; TEST AH,5: cut when below it.
        if f64::from(radius) < distance {
            e.call(POINT_NORMALIZE_004E7560, &args![offset]);
            let scale = x87_store32(f64::from(radius));
            e.call(POINT_SCALE_006AD180, &args![offset, scale]);
        }
        let tool_x = x87_store32(x87_add(f64::from(left), x87_m32(e.mem.u32(offset))));
        let tool = e.mem.u32(this.addr() + 0x44);
        e.call(
            TILE_SET_FLOAT,
            &args![tool, 0xfa1u32, x87_quiet_bits(tool_x), 1u32],
        );
        let tool_y = x87_store32(x87_add(f64::from(top), x87_m32(e.mem.u32(offset + 4))));
        let tool = e.mem.u32(this.addr() + 0x44);
        e.call(
            TILE_SET_FLOAT,
            &args![tool, 0xfa2u32, x87_quiet_bits(tool_y), 1u32],
        );
    });
}

/// The controller manager's sequence count (`00495d00`) and
/// `NiControllerManager::GetSequenceAt` (`00495d20`, `RET 4`), an object's name
/// (`0043b1b0` of `00413f40`), `sprintf` (`00ec623a`, cdecl) with `"Bone%s"`
/// (`010777c8`), `strncmp` (`00ec7ec0`, cdecl), a sequence's block count (`0084e3a0`:
/// the word at +0xc) and test (`004937a0`, `RET 4`), its block removal (`00a32b80`, `RET
/// 4`), its owner (`005f5f80`) and its manager (`008d6f30`), the allocator (`00401000`),
/// the list constructor (`0096a2d0`), the list add (`005ae3d0`, `RET 4`).
const SEQUENCE_COUNT_00495D00: u32 = 0x0049_5d00;
const GET_SEQUENCE_AT: u32 = 0x0049_5d20;
const OBJECT_NAME_SOURCE_00413F40: u32 = 0x0041_3f40;
const OBJECT_NAME_0043B1B0: u32 = 0x0043_b1b0;
const SPRINTF: u32 = 0x00ec_623a;
const BONE_FORMAT: u32 = 0x0107_77c8;
const STRNCMP: u32 = 0x00ec_7ec0;
const BLOCK_COUNT_0084E3A0: u32 = 0x0084_e3a0;
const BLOCK_TEST_004937A0: u32 = 0x0049_37a0;
const REMOVE_BLOCK_00A32B80: u32 = 0x00a3_2b80;
const SEQUENCE_OWNER_005F5F80: u32 = 0x005f_5f80;
const SEQUENCE_MANAGER_008D6F30: u32 = 0x008d_6f30;
const ALLOCATE: u32 = 0x0040_1000;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_ADD: u32 = 0x005a_e3d0;
/// The condition range ends 25.0 (`0104f2f0`), 50.0 (`0101e2c0`) and 75.0 (`010731a8`),
/// `double`s.
const TWENTY_FIVE_DOUBLE: u32 = 0x0104_f2f0;
const FIFTY_DOUBLE: u32 = 0x0101_e2c0;
const SEVENTY_FIVE_DOUBLE: u32 = 0x0107_31a8;

/// The limb condition (`limb_condition_value`) truncated (`_ftol2`).
fn limb_condition(e: &mut Engine, limb: u32) -> i32 {
    let value = limb_condition_value(e, limb);
    e.call(FTOL2, &args![value]).i32()
}

/// A new list (8 bytes, `0096a2d0`), or 0 when the allocation fails.
fn new_list(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![8u32]).u32();
    if block != 0 {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    } else {
        0
    }
}

// Translated from 007e4920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e4920 - the C++ exception-unwinding frame (FS:[0] around the lists) and the /GS cookie check (00ec408c) are not translated.
/// `SurgeryMenu::PrepareSequences` (Xbox PDB; cdecl, the skin root): nothing without a
/// root. With its controller manager (kept in the open menu's +0xe4): the current limb
/// wound, or a new one (16 bytes: the menu's limb, its damage level, a new list) added
/// to `xLimbWounds`; for each of the manager's sequences, when the wound is new or reset
/// a record (12 bytes: the sequence, a new list) collects the owners' objects (slot
/// +0x9c) of the blocks whose name matches `"Bone<sequence name>"` in its first six
/// characters; the other blocks are removed (`00a32b80`). The damage level (+0xc4) is
/// set from the limb's condition and gives the healed fraction (25-point steps); each
/// sequence of the current wound is played when it belongs to the manager, updated at
/// time 0, and placed at its length times the fraction (kept in the record's +8) when
/// the wound is new or reset, else at the kept phase; the wound's reset flag is cleared.
/// The root is updated.
pub fn surgery_menu_prepare_sequences(e: &mut Engine, root: u32) {
    if root == 0 {
        return;
    }
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0xb0, |e, frame| {
        let ebp = frame.addr() + 0xb0;
        let local = |offset: u32| ebp - offset;
        let me = |e: &Engine| e.global::<u32>(SURGERY_MENU_ME);
        let controller = e
            .call(GET_CONTROLLER, &args![root, CONTROLLER_TYPE_011F36AC])
            .u32();
        let mut existed = true;
        if controller != 0 {
            e.mem.set_u32(me(e) + 0xe4, controller);
            let wound = surgery_menu_get_current_limb_wound(e, Ptr::new(me(e)));
            let holder = local(0x30);
            if wound == 0 {
                existed = false;
                let created = e.call(ALLOCATE, &args![0x10u32]).u32();
                e.mem.set_u32(holder, created);
                let limb = e.mem.u32(me(e) + 0xc0);
                e.mem.set_u32(created + 4, limb);
                let limb = e.mem.u32(created + 4);
                let condition = limb_condition(e, limb);
                let level = fn_007e5540(e, Ptr::new(me(e)), condition);
                e.mem.set_u32(created + 8, level);
                let list = new_list(e);
                e.mem.set_u32(created, list);
                e.mem.set_u8(created + 0xc, 0);
                e.call(LIST_ADD, &args![LIMB_WOUNDS, holder]);
            } else {
                e.mem.set_u32(holder, wound);
            }
            let fresh = |e: &Engine| wound == 0 || e.mem.u8(wound + 0xc) == 1;
            let mut index = 0u32;
            loop {
                let manager = e.mem.u32(me(e) + 0xe4);
                if index >= e.call(SEQUENCE_COUNT_00495D00, &args![manager]).u32() {
                    break;
                }
                let manager = e.mem.u32(me(e) + 0xe4);
                let sequence = e.call(GET_SEQUENCE_AT, &args![manager, index]).u32();
                let source = e.call(OBJECT_NAME_SOURCE_00413F40, &args![sequence]).u32();
                let name = e.call(OBJECT_NAME_0043B1B0, &args![source]).u32();
                let bone = local(0x68);
                e.call(SPRINTF, &args![bone, BONE_FORMAT, name]);
                let record_holder = local(0x44);
                e.mem.set_u32(record_holder, 0);
                if fresh(e) {
                    let record = e.call(ALLOCATE, &args![0xcu32]).u32();
                    e.mem.set_u32(record_holder, record);
                    let list = new_list(e);
                    let record = e.mem.u32(record_holder);
                    e.mem.set_u32(record + 4, list);
                    e.mem.set_u32(record, sequence);
                }
                let mut block = 0u32;
                while block < e.call(BLOCK_COUNT_0084E3A0, &args![sequence]).u32() {
                    let entry = fn_007e4fc0(e, Ptr::new(sequence), block);
                    if e.call(BLOCK_TEST_004937A0, &args![sequence, block]).u32() != 0 {
                        let text = e.call(NODE_SELF, &args![entry]).u32();
                        let block_name = e.call(OBJECT_NAME_0043B1B0, &args![text]).u32();
                        if e.call(STRNCMP, &args![bone, block_name, 6u32]).u32() != 0 {
                            e.call(REMOVE_BLOCK_00A32B80, &args![sequence, block]);
                        } else if fresh(e) {
                            let owner = e.call(SEQUENCE_OWNER_005F5F80, &args![sequence]).u32();
                            let key = e.call(NODE_SELF, &args![entry]).u32();
                            let object = e.vcall(owner, 0x9c, &args![key]).u32();
                            let object_holder = local(0x98);
                            e.mem.set_u32(object_holder, object);
                            let record = e.mem.u32(record_holder);
                            let list = e.mem.u32(record + 4);
                            e.call(LIST_ADD, &args![list, object_holder]);
                        }
                    }
                    block += 1;
                }
                e.call(NODE_SELF, &args![local(0x3c)]);
                e.call(NODE_SELF, &args![local(0x70)]);
                if e.mem.u32(record_holder) != 0 {
                    let wound = e.mem.u32(holder);
                    let list = e.mem.u32(wound);
                    e.call(LIST_ADD, &args![list, record_holder]);
                }
                index += 1;
            }
        }
        let mut fraction = x87_store32(0.0);
        let limb = e.mem.u32(me(e) + 0xc0);
        let condition = limb_condition(e, limb);
        let level = fn_007e5540(e, Ptr::new(me(e)), condition);
        e.mem.set_u32(me(e) + 0xc4, level);
        let quarter = e.global::<f64>(TWENTY_FIVE_DOUBLE);
        let half = e.global::<f64>(FIFTY_DOUBLE);
        let three_quarters = e.global::<f64>(SEVENTY_FIVE_DOUBLE);
        // Jump table `007e4fac`.
        match e.mem.u32(me(e) + 0xc4) {
            1 => {
                let limb = e.mem.u32(me(e) + 0xc0);
                let value = limb_condition_value(e, limb);
                let left = x87_sub(quarter, x87_sub(value, three_quarters));
                fraction = x87_store32(x87_div(left, quarter));
            }
            2 => {
                let limb = e.mem.u32(me(e) + 0xc0);
                let value = limb_condition_value(e, limb);
                let left = x87_add(x87_sub(quarter, x87_sub(value, half)), quarter);
                fraction = x87_store32(x87_div(left, half));
            }
            3 => {
                let limb = e.mem.u32(me(e) + 0xc0);
                let value = limb_condition_value(e, limb);
                let left = x87_add(x87_sub(quarter, x87_sub(value, quarter)), half);
                fraction = x87_store32(x87_div(left, three_quarters));
            }
            4 => {
                let limb = e.mem.u32(me(e) + 0xc0);
                let value = limb_condition_value(e, limb);
                let left = x87_add(x87_sub(quarter, value), three_quarters);
                fraction = x87_store32(x87_div(left, e.global::<f64>(HUNDRED_DOUBLE)));
            }
            _ => {}
        }
        let wound = surgery_menu_get_current_limb_wound(e, Ptr::new(me(e)));
        if wound != 0 && e.mem.u32(me(e) + 0xc4) != 0 {
            let list = e.mem.u32(wound);
            walk_list(e, list, |e, node| {
                let record = node_item(e, node);
                let sequence = e.mem.u32(record);
                let manager = e.call(SEQUENCE_MANAGER_008D6F30, &args![sequence]).u32();
                if manager == e.mem.u32(me(e) + 0xe4) {
                    let record = node_item(e, node);
                    let sequence = e.mem.u32(record);
                    let manager = e.mem.u32(me(e) + 0xe4);
                    e.call(
                        PLAY_SEQUENCE,
                        &args![
                            manager,
                            sequence,
                            0u32,
                            1u32,
                            x87_store32(1.0),
                            x87_store32(0.0),
                            0u32
                        ],
                    );
                }
                let record = node_item(e, node);
                let sequence = e.mem.u32(record);
                e.call(SEQUENCE_UPDATE, &args![sequence, x87_store32(0.0), 1u32]);
                if !existed || e.mem.u8(wound + 0xc) != 0 {
                    let record = node_item(e, node);
                    let sequence = e.mem.u32(record);
                    let length = e.call(SEQUENCE_LENGTH, &args![sequence]).f64();
                    let phase = x87_store32(x87_mul(length, x87_m32(fraction)));
                    let record = node_item(e, node);
                    let sequence = e.mem.u32(record);
                    e.call(
                        SEQUENCE_SET_PHASE,
                        &args![sequence, x87_quiet_bits(phase), 1u32],
                    );
                    let record = node_item(e, node);
                    let sequence = e.mem.u32(record);
                    let length = e.call(SEQUENCE_LENGTH, &args![sequence]).f64();
                    let kept = x87_mul(length, x87_m32(fraction));
                    let record = node_item(e, node);
                    e.mem.set_u32(record + 8, x87_store32(kept));
                } else {
                    let record = node_item(e, node);
                    let phase = x87_quiet_bits(e.mem.u32(record + 8));
                    let record = node_item(e, node);
                    let sequence = e.mem.u32(record);
                    e.call(SEQUENCE_SET_PHASE, &args![sequence, phase, 1u32]);
                }
                true
            });
            e.mem.set_u8(wound + 0xc, 0);
        }
        let data = local(0x20);
        e.call(UPDATE_DATA_INIT, &args![data, x87_store32(0.0), 1u32, 0u32]);
        e.call(UPDATE_OBJECT, &args![root, data]);
    });
}

/// The limb's condition (the player's actor value owner, slot +0xc, `ST0`) without
/// truncating it.
fn limb_condition_value(e: &mut Engine, limb: u32) -> f64 {
    let player = e.global::<u32>(PLAYER);
    e.vcall(player + 0xa4, 0xc, &args![limb]).f64()
}

/// The list sort with a comparison (`0083fd60`, `RET 4`), the list count (`005ae380`),
/// a sequence's phase (`00508120`, `ST0`), `fabs`-like `00408840` (cdecl, a `float`,
/// `ST0`), the wobble `00c5d320` (cdecl, a `float`, `ST0`), the wound formula
/// `00648d70` (cdecl, six words), `BSSoundHandle::IsPlaying` (`00ad8930`), the root's
/// translation (`0043c490`), and the constants 0.015 (`010776d8`), 0.006 (`010776d0`),
/// 33.0 (`010356c0`), 0.00025 (`010776c8`), 2.0 (`01011590`), 0.1 (`0101ffa0`), as
/// `double`s; 500.0 (`01013d84`), a `float`; the trait static `011dad9c`.
const LIST_SORT_0083FD60: u32 = 0x0083_fd60;
const LIST_COUNT_005AE380: u32 = 0x005a_e380;
const SEQUENCE_PHASE_00508120: u32 = 0x0050_8120;
const MAGNITUDE_00408840: u32 = 0x0040_8840;
const WOBBLE_00C5D320: u32 = 0x00c5_d320;
const WOUND_FORMULA_00648D70: u32 = 0x0064_8d70;
const SOUND_IS_PLAYING: u32 = 0x00ad_8930;
const ROOT_TRANSLATE_0043C490: u32 = 0x0043_c490;
const CLOSE_RATE_010776D8: u32 = 0x0107_76d8;
const CLOSE_RATE_010776D0: u32 = 0x0107_76d0;
const TICKS_010356C0: u32 = 0x0103_56c0;
const WOBBLE_RATE_010776C8: u32 = 0x0107_76c8;
const TWO_DOUBLE: u32 = 0x0101_1590;
const TENTH_DOUBLE: u32 = 0x0101_ffa0;
const FIVE_HUNDRED_01013D84: u32 = 0x0101_3d84;
const TRAIT_011DAD9C: u32 = 0x011d_ad9c;
/// The tile float trait getter (`00a011b0`, `RET 4`, `ST0`).
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;

/// The clock's milliseconds as the code loads them (`FILD qword` of the zero-extended
/// word).
fn milliseconds_since(e: &mut Engine, start: u32) -> f64 {
    let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
    f64::from(now.wrapping_sub(start))
}

/// The skin geometry of the limb (`pSkinGeometry` +0xc8 of `owner` at `007e5e30`'s
/// index, through `00559450`).
fn limb_skin(e: &mut Engine, this: Ptr, owner: u32, limb: u32) -> u32 {
    let index = fn_007e5e30(e, this, limb as i32);
    e.call(
        FIRST_WORD,
        &args![owner.wrapping_add(0xc8).wrapping_add(index.wrapping_mul(4))],
    )
    .u32()
}

// Translated from 007e2de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SurgeryMenu::UpdateFlesh` (Xbox PDB; `ST0`): 0 without the camera or a current limb
/// wound. The wound's sequence records are sorted by cursor distance; with more than one,
/// the cursor (over the screen size) is tested against each record's quad of screen
/// points (the record's first two objects and the next record's, `007e5420`); inside one
/// the wound is burning. Burning starts its timer (+0x88), plays the wound sound (+0xa0)
/// and, once past `500 - 500 * luck / 100` milliseconds (`004b3ab0` of actor value 0x25,
/// truncated), closes the first two sequences by their remaining length's `00408840`
/// times 0.015 and 0.006 per 33 ms since the last update (+0x8c), sets their phases
/// (kept in the records' +8), updates the limb's skin and, with a scar and the mean
/// phase below 0.1, places a good decal. Not burning places a bad decal, plays the flesh
/// sound (+0x94), stops the wound sound and clears the timer. Then the last update is
/// now, `00648d70` runs on the pain and the menu tile's trait `011dad9c`, the limb's skin
/// wobbles (`00c5d320` of the clock) and is updated. Answers 0 when burning, else 100.
pub fn surgery_menu_update_flesh(e: &mut Engine, this: Ptr) -> f32 {
    let base = this.addr();
    let wound = surgery_menu_get_current_limb_wound(e, this);
    if e.call(FIRST_WORD, &args![CAMERA_HOLDER_011DADB4]).u32() == 0 || wound == 0 {
        return 0.0;
    }
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x2a0, |e, frame| {
        let ebp = frame.addr() + 0x2a0;
        let local = |offset: u32| ebp - offset;
        let list = e.mem.u32(wound);
        e.call(LIST_SORT_0083FD60, &args![list, 0x007e_5250u32]);
        let mut burning = false;
        let cursor = local(0x18);
        let list = e.mem.u32(wound);
        if e.call(LIST_COUNT_005AE380, &args![list]).u32() > 1 {
            let list = e.mem.u32(wound);
            let _corners = e
                .call(LIST_COUNT_005AE380, &args![list])
                .u32()
                .wrapping_shl(1);
            e.call(NODE_SELF, &args![cursor]);
            let (x, y) = (e.mem.u32(base + 0x74), e.mem.u32(base + 0x78));
            e.mem.set_u32(cursor, x);
            e.mem.set_u32(cursor + 4, y);
            let width = e.call(GET_SCREEN_WIDTH, &args![]).f64();
            let x = x87_store32(x87_div(x87_m32(e.mem.u32(cursor)), width));
            e.mem.set_u32(cursor, x);
            let height = e.call(GET_SCREEN_HEIGHT_00715DA0, &args![]).f64();
            let y = x87_store32(x87_div(x87_m32(e.mem.u32(cursor + 4)), height));
            e.mem.set_u32(cursor + 4, y);
            // The quad of four `double` points at `EBP - 0x1b0` and the cursor as
            // `double`s at `EBP - 0x28`.
            let quad = local(0x1b0);
            let point = local(0x28);
            let mut node = e.mem.u32(wound);
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let (a, b) = (local(0x30), local(0x1b8));
                e.call(NODE_SELF, &args![a]);
                e.call(NODE_SELF, &args![b]);
                let record = node_item(e, node);
                let objects = e.mem.u32(record + 4);
                let object = node_item(e, objects);
                screen_point(e, object, a, a + 4);
                let record = node_item(e, node);
                let objects = e.mem.u32(record + 4);
                let second = e.call(NODE_NEXT, &args![objects]).u32();
                let object = node_item(e, second);
                screen_point(e, object, b, b + 4);
                for corner in [a, b] {
                    let flipped = x87_store32(x87_sub(1.0, x87_load32(e.mem.u32(corner + 4))));
                    e.mem.set_u32(corner + 4, flipped);
                }
                let store_double = |e: &mut Engine, at: u32, bits: u32| {
                    e.mem.set_u64(at, x87_load32(bits).to_bits());
                };
                let words = [
                    e.mem.u32(a),
                    e.mem.u32(a + 4),
                    e.mem.u32(b),
                    e.mem.u32(b + 4),
                ];
                for (i, &word) in words.iter().enumerate() {
                    store_double(e, quad + 8 * i as u32, word);
                }
                let (cx, cy) = (e.mem.u32(cursor), e.mem.u32(cursor + 4));
                store_double(e, point, cx);
                store_double(e, point + 8, cy);
                if e.call(NODE_NEXT, &args![node]).u32() != 0 {
                    let (c, d) = (local(0x1c0), local(0x1c8));
                    e.call(NODE_SELF, &args![c]);
                    e.call(NODE_SELF, &args![d]);
                    let next = e.call(NODE_NEXT, &args![node]).u32();
                    let record = node_item(e, next);
                    let objects = e.mem.u32(record + 4);
                    let object = node_item(e, objects);
                    screen_point(e, object, c, c + 4);
                    let next = e.call(NODE_NEXT, &args![node]).u32();
                    let record = node_item(e, next);
                    let objects = e.mem.u32(record + 4);
                    let second = e.call(NODE_NEXT, &args![objects]).u32();
                    let object = node_item(e, second);
                    screen_point(e, object, d, d + 4);
                    for corner in [c, d] {
                        let flipped = x87_store32(x87_sub(1.0, x87_load32(e.mem.u32(corner + 4))));
                        e.mem.set_u32(corner + 4, flipped);
                    }
                    let words = [
                        e.mem.u32(c),
                        e.mem.u32(c + 4),
                        e.mem.u32(d),
                        e.mem.u32(d + 4),
                    ];
                    for (i, &word) in words.iter().enumerate() {
                        store_double(e, quad + 0x20 + 8 * i as u32, word);
                    }
                    if fn_007e5420(e, quad, 4, point) != 0 {
                        burning = true;
                    }
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
            if burning && e.mem.u32(base + 0x88) == 0 {
                let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
                e.mem.set_u32(base + 0x88, now);
            }
            if burning {
                if !e.call(SOUND_IS_PLAYING, &args![base + 0xa0]).bool() {
                    e.call(SOUND_PLAY, &args![base + 0xa0, 0u32]);
                }
                if e.mem.u32(base + 0x88) != 0 {
                    let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
                    let held = now.wrapping_sub(e.mem.u32(base + 0x88));
                    let player = e.global::<u32>(PLAYER);
                    let luck = e.vcall(player + 0xa4, 8, &args![0x25u32]).i32();
                    let hundred = x87_quiet_bits(e.global::<u32>(HUNDRED_01016410));
                    let five_hundred = x87_quiet_bits(e.global::<u32>(FIVE_HUNDRED_01013D84));
                    let zero = x87_store32(0.0);
                    let delay = e
                        .call(
                            INTERPOLATE_004B3AB0,
                            &args![
                                five_hundred,
                                zero,
                                zero,
                                hundred,
                                x87_store32(f64::from(luck))
                            ],
                        )
                        .f64();
                    let delay = x87_fistp64_chop(X87Ext::from(delay)) as u32;
                    if held > delay {
                        close_wound(e, this, wound, cursor);
                    }
                }
            } else {
                let (x, y) = (e.mem.u32(cursor), e.mem.u32(cursor + 4));
                surgery_menu_place_decal(e, this, f32::from_bits(x), f32::from_bits(y), 0);
                if !e.call(SOUND_IS_PLAYING, &args![base + 0x94]).bool() {
                    e.call(SOUND_PLAY, &args![base + 0x94, 0u32]);
                }
                if e.call(SOUND_IS_PLAYING, &args![base + 0xa0]).bool() {
                    e.call(SOUND_STOP, &args![base + 0xa0]);
                }
                e.mem.set_u32(base + 0x88, 0);
            }
        }
        let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
        e.mem.set_u32(base + 0x8c, now);
        // `MOV EAX,1; TEST EAX,EAX`: always taken.
        let player = e.global::<u32>(PLAYER);
        let luck = e.vcall(player + 0xa4, 8, &args![0x25u32]).u32();
        let pain = x87_store32(x87_div(
            x87_load32(e.mem.u32(base + 0xac)),
            e.global::<f64>(HUNDRED_DOUBLE),
        ));
        let which = e.global::<u32>(TRAIT_011DAD9C);
        let menu_tile = e.call(NODE_NEXT, &args![base]).u32();
        let value = e.call(TILE_GET_FLOAT, &args![menu_tile, which]).f64();
        let value = x87_store32(x87_div(value, e.global::<f64>(HUNDRED_DOUBLE)));
        e.call(
            WOUND_FORMULA_00648D70,
            &args![
                local(0x20c),
                local(0x218),
                local(0x21c),
                x87_quiet_bits(value),
                x87_quiet_bits(pain),
                luck
            ],
        );
        let wobble = local(0x214);
        e.call(NODE_SELF, &args![wobble]);
        let rate = e.global::<f64>(WOBBLE_RATE_010776C8);
        let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
        let angle = x87_store32(x87_mul(f64::from(now), rate));
        let x = e.call(WOBBLE_00C5D320, &args![x87_quiet_bits(angle)]).f64();
        e.mem.set_u32(wobble, x87_store32(x));
        let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
        let angle = x87_store32(x87_mul(f64::from(now.wrapping_add(0x3039)), rate));
        let y = e.call(WOBBLE_00C5D320, &args![x87_quiet_bits(angle)]).f64();
        e.mem.set_u32(wobble + 4, x87_store32(y));
        let limb = e.mem.u32(wound + 4);
        let skin = limb_skin(e, this, base, limb);
        let position = e.call(ROOT_TRANSLATE_0043C490, &args![skin]).u32();
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.mem.set_u32(local(0x208) + 4 * i, word);
        }
        let x = x87_quiet_bits(e.mem.u32(wobble));
        let y = x87_quiet_bits(e.mem.u32(wobble + 4));
        let limb = e.mem.u32(wound + 4);
        let skin = limb_skin(e, this, base, limb);
        e.call(SET_TRANSLATE, &args![skin, x, y, x87_store32(1.0)]);
        let data = local(0x1fc);
        e.call(UPDATE_DATA_INIT, &args![data, x87_store32(0.0), 0u32, 0u32]);
        let limb = e.mem.u32(wound + 4);
        let skin = limb_skin(e, this, base, limb);
        e.call(UPDATE_OBJECT, &args![skin, data]);
        if burning {
            0.0
        } else {
            f32::from_bits(x87_quiet_bits(e.global::<u32>(HUNDRED_01016410)))
        }
    })
}

/// The closing step of `UpdateFlesh` (see there) for the wound's first two sequence
/// records; `cursor` holds the cursor's screen fractions.
fn close_wound(e: &mut Engine, this: Ptr, wound: u32, cursor: u32) {
    let base = this.addr();
    let list = e.mem.u32(wound);
    let first_record = node_item(e, list);
    let list = e.mem.u32(wound);
    let second_node = e.call(NODE_NEXT, &args![list]).u32();
    let second_record = node_item(e, second_node);
    let list = e.mem.u32(wound);
    let first_record_again = node_item(e, list);
    let list = e.mem.u32(wound);
    let second_node = e.call(NODE_NEXT, &args![list]).u32();
    let second_record_again = node_item(e, second_node);
    let sequences = [e.mem.u32(first_record), e.mem.u32(second_record)];
    let records = [first_record_again, second_record_again];
    let mut phases = [0u32; 2];
    for (phase, &sequence) in phases.iter_mut().zip(sequences.iter()) {
        let value = e.call(SEQUENCE_PHASE_00508120, &args![sequence]).f64();
        *phase = x87_store32(value);
    }
    if e.mem.u32(base + 0x8c) == 0 {
        let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
        e.mem.set_u32(base + 0x8c, now);
    } else {
        for (k, rate) in [CLOSE_RATE_010776D8, CLOSE_RATE_010776D0]
            .iter()
            .enumerate()
        {
            let sequence = sequences[k];
            let length = e.call(SEQUENCE_LENGTH, &args![sequence]).f64();
            let start = e.call(SEQUENCE_00759450, &args![sequence]).f64();
            let left = x87_store32(x87_sub(length, start));
            let size = e
                .call(MAGNITUDE_00408840, &args![x87_quiet_bits(left)])
                .f64();
            let speed = x87_mul(size, e.global::<f64>(*rate));
            let since = milliseconds_since(e, e.mem.u32(base + 0x8c));
            let step = x87_mul(x87_div(since, e.global::<f64>(TICKS_010356C0)), speed);
            phases[k] = x87_store32(x87_sub(x87_m32(phases[k]), step));
        }
    }
    for k in 0..2 {
        e.call(
            SEQUENCE_SET_PHASE,
            &args![sequences[k], x87_quiet_bits(phases[k]), 1u32],
        );
        e.mem.set_u32(records[k] + 8, x87_quiet_bits(phases[k]));
    }
    // The update data at `EBP - 0x1e4`.
    e.with_stack(0xc, |e, data| {
        e.call(UPDATE_DATA_INIT, &args![data, x87_store32(0.0), 1u32, 0u32]);
        let limb = e.mem.u32(wound + 4);
        let me = e.global::<u32>(SURGERY_MENU_ME);
        let skin = limb_skin(e, this, me, limb);
        e.call(UPDATE_OBJECT, &args![skin, data]);
    });
    let me = e.global::<u32>(SURGERY_MENU_ME);
    if e.mem.u32(me + 0xe8) != 0 {
        let mean = x87_div(
            x87_add(x87_load32(phases[0]), x87_m32(phases[1])),
            e.global::<f64>(TWO_DOUBLE),
        );
        // FCOMP with 0.1; TEST AH,5: below it only.
        if mean < e.global::<f64>(TENTH_DOUBLE) {
            let (x, y) = (e.mem.u32(cursor), e.mem.u32(cursor + 4));
            surgery_menu_place_decal(e, this, f32::from_bits(x), f32::from_bits(y), 1);
        }
    }
}

/// `SurgeryMenu::UpdateToolLocation`'s trait guard bits (`011daebc`) and its eight tool
/// traits (`"_MinResistanceTime"`, `"_MaxResistanceTime"`, `"_MinMagnitude"`,
/// `"_MaxMagnitude"`, `"_MinXBoxMag"`, `"_MaxXBoxMag"`, `"_MouseMinPercent"`,
/// `"_MouseMaxPercent"`): each name with the static that keeps its trait, bit by bit.
const TOOL_TRAIT_GUARD: u32 = 0x011d_aebc;
const TOOL_TRAITS: [(u32, u32); 8] = [
    (0x0107_7754, 0x011d_aeb8),
    (0x0107_7740, 0x011d_aeb4),
    (0x0107_7730, 0x011d_aeb0),
    (0x0107_7720, 0x011d_aeac),
    (0x0107_7714, 0x011d_aea8),
    (0x0107_7708, 0x011d_aea4),
    (0x0107_76f4, 0x011d_aea0),
    (0x0107_76e0, 0x011d_ae9c),
];
/// `DoClick`'s trait guard bits (`011dae98`) and its two count traits
/// (`"_StimpacCount"` kept in `011dae94`, `"_MorphineCount"` in `011dae90`).
const CLICK_TRAIT_GUARD: u32 = 0x011d_ae98;
const STIMPAK_COUNT_NAME: u32 = 0x0107_7664;
const STIMPAK_COUNT_TRAIT: u32 = 0x011d_ae94;
const MORPHINE_COUNT_NAME: u32 = 0x0107_7654;
const MORPHINE_COUNT_TRAIT: u32 = 0x011d_ae90;
const IS_TOP_MENU_NAME: u32 = 0x0107_768c;
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
const MENU_TILE_OF: u32 = 0x0072_6070;
const ITEM_COUNT: u32 = 0x0072_6070;
const INPUT_HOLDER_011DEA0C: u32 = 0x011d_ea0c;
const INPUT_DEVICE_00877720: u32 = 0x0087_7720;
const THUMBSTICK_00A23390: u32 = 0x00a2_3390;
const MOUSE_AXIS_00A239E0: u32 = 0x00a2_39e0;
const CONTROL_STATE: u32 = 0x00a2_4660;
const CHECK_MENU_BUTTON: u32 = 0x0071_7a40;
const STICK_SPEED_SETTING_011D8C5C: u32 = 0x011d_8c5c;
const POINT2_ADD_004E88D0: u32 = 0x004e_88d0;
const POINT2_SCALE_004A4F40: u32 = 0x004a_4f40;
const POINT2_SCALED_00532FD0: u32 = 0x0053_2fd0;
const CURSOR_SCALE_0070ECB0: u32 = 0x0070_ecb0;
const TILE_00A01440: u32 = 0x00a0_1440;
const TILE_00A013D0: u32 = 0x00a0_13d0;
const SET_CURSOR_POSITION_007151B0: u32 = 0x0071_51b0;
const PAIN_CHANGE_PER_SECOND: u32 = 0x0064_8e10;
const GET_PAIN_CAUSED_BY_WOUND: u32 = 0x0064_8e30;
const GET_SURGERY_AUTO_ATTEMPT_PERCENTAGES: u32 = 0x0064_8fa0;
const SPRINTF_S: u32 = 0x0040_6d00;
const HEALTH_FORMAT: u32 = 0x0107_14ec;
const PERCENT_FORMAT: u32 = 0x0107_7674;
const RANDOM_INT: u32 = 0x0094_4460;
const ACTOR_RESTORE_ACTOR_VALUE: u32 = 0x0088_b740;
const PAIN_SOUND_ONE: u32 = 0x0107_76b8;
const PAIN_SOUND_TWO: u32 = 0x0107_76a8;
const PAIN_SOUND_THREE: u32 = 0x0107_7698;
const TILE_ID_00720010: u32 = 0x0072_0010;
const GET_DEFAULT_OBJECT: u32 = 0x0058_db10;
const REFERENCE_GET_INVENTORY_ITEM: u32 = 0x0057_6260;
const ACTOR_QUEUE_EQUIP_OBJECT: u32 = 0x0088_c650;
const ITEM_GET_FORM: u32 = 0x0044_ddc0;
const ITEM_CHANGE_DELETE: u32 = 0x0044_59e0;
const MAGIC_TARGET_IS_SPELL_TARGET: u32 = 0x0082_2b90;
const SMALLER_OF: u32 = 0x0040_ebd0;
/// -0.5 (`0102295c`), 0.25 (`0101622c`), 10.0 (`01017b78`), all `float`s; 5.0
/// (`01020998`), 1.0 (`01012070`), 30.0 (`0101db88`), 60.0 (`01012638`) and 85.0
/// (`01072ff0`), `double`s; 1000.0 (`01013974`, `float`).
const MINUS_HALF_0102295C: u32 = 0x0102_295c;
const QUARTER_0101622C: u32 = 0x0101_622c;
const MOVEMENT_CAP_01017B78: u32 = 0x0101_7b78;
const FIVE_01020998: u32 = 0x0102_0998;
const ONE_01012070: u32 = 0x0101_2070;
const THIRTY_0101DB88: u32 = 0x0101_db88;
const SIXTY_01012638: u32 = 0x0101_2638;
const EIGHTY_FIVE_01072FF0: u32 = 0x0107_2ff0;
const THOUSAND_01013974: u32 = 0x0101_3974;

/// Looks up a trait by name into its static once (`00a00940` with -1), guarded by a bit.
fn lazy_trait(e: &mut Engine, guard: u32, bit: u32, name: u32, holder: u32) {
    let flags = e.global::<u32>(guard);
    if flags & bit == 0 {
        e.set_global(guard, flags | bit);
        let id = e.call(ADD_USER_TRAIT, &args![name, 0xffff_ffffu32]).u32();
        e.set_global(holder, id);
    }
}

/// `004b3ab0` of five `float`s (as the code pushes them, last pushed first).
fn interpolate_between(e: &mut Engine, from: u32, to: u32, low: u32, high: u32, x: u32) -> f64 {
    e.call(INTERPOLATE_004B3AB0, &args![from, to, low, high, x])
        .f64()
}

/// The menu tile's `float` of the trait kept in `holder`.
fn menu_trait_float(e: &mut Engine, this: Ptr, holder: u32) -> f64 {
    let which = e.global::<u32>(holder);
    let tile = e.call(MENU_TILE_OF, &args![this.addr()]).u32();
    e.call(TILE_GET_FLOAT, &args![tile, which]).f64()
}

/// `FLD float; FCOMP 0.0; TEST AH,1`: the value's sign (or NaN) picks `-value`.
fn signed_by(sign: u32, value: u32) -> u32 {
    if x87_load32(sign) >= 0.0 {
        x87_quiet_bits(value)
    } else {
        x87_store32(-x87_load32(value))
    }
}

/// The thumbstick's move scaled by the setting `011d8c5c`, truncated (`_ftol2`).
fn stick_step(e: &mut Engine, amount: i32) -> i32 {
    let speed = setting_float(e, STICK_SPEED_SETTING_011D8C5C);
    e.call(FTOL2, &args![x87_mul(speed, f64::from(amount))])
        .i32()
}

// Translated from 007e3890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e3890 - the C++ exception-unwinding frame (FS:[0] around the trait statics) is not translated.
/// `SurgeryMenu::UpdateToolLocation` (Xbox PDB; `thiscall`, one `bool`): looks up the
/// eight tool traits once and reads them from the menu tile (the last two over 100).
/// Burning: sets the burning flag (+0x90) and tile +0x5c's 0xfa3 to 1; `00648d70` of the
/// pain and luck gives the resistance, magnitude and mouse fractions, each mapped onto
/// its trait pair (`004b3ab0`); the resistance (+0x7c) wobbles with the clock
/// (`00c5d320`), signed by the wobble, and is added to the tool location (+0x74). The
/// gamepad's stick (or the mouse's axes) moves it further, scaled by the mouse fraction;
/// the tool movement (+0x84) is the move's length times magnitude and fraction, at most
/// 10, over 5. The tool is placed (`PlaceToolInWindow`) and the cursor set from it. Not
/// burning: the flag, the movement and 0xfa3 cleared, and the tool placed at the
/// gamepad's tile point plus the stick, or at the cursor.
pub fn surgery_menu_update_tool_location(e: &mut Engine, this: Ptr, burning: bool) {
    let base = this.addr();
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    for (i, &(name, holder)) in TOOL_TRAITS.iter().enumerate() {
        lazy_trait(e, TOOL_TRAIT_GUARD, 1 << i, name, holder);
    }
    let mut range = [0u32; 8];
    for (i, &(_, holder)) in TOOL_TRAITS.iter().enumerate() {
        let value = menu_trait_float(e, this, holder);
        range[i] = if i >= 6 {
            x87_store32(x87_div(value, e.global::<f64>(HUNDRED_DOUBLE)))
        } else {
            x87_store32(value)
        };
    }
    let q = x87_quiet_bits;
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x80, |e, frame| {
        let ebp = frame.addr() + 0x80;
        let local = |offset: u32| ebp - offset;
        let (resistance, magnitude, pull) = (local(0x3c), local(0x50), local(0x54));
        let point = local(0x4c);
        if burning {
            e.mem.set_u8(base + 0x90, 1);
            let tile = e.mem.u32(base + 0x5c);
            e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 1u32]);
            let player = e.global::<u32>(PLAYER);
            let luck = e.vcall(player + 0xa4, 8, &args![0x25u32]).u32();
            let pain = x87_store32(x87_div(
                x87_load32(e.mem.u32(base + 0xac)),
                e.global::<f64>(HUNDRED_DOUBLE),
            ));
            let value = menu_trait_float(e, this, TRAIT_011DAD9C);
            let value = x87_store32(x87_div(value, e.global::<f64>(HUNDRED_DOUBLE)));
            e.call(
                WOUND_FORMULA_00648D70,
                &args![resistance, magnitude, pull, q(value), q(pain), luck],
            );
            let (zero, one) = (x87_store32(0.0), x87_store32(1.0));
            let x = q(e.mem.u32(resistance));
            let v = interpolate_between(e, q(range[0]), q(range[1]), zero, one, x);
            e.mem.set_u32(resistance, x87_store32(v));
            let rate = e.global::<f64>(WOBBLE_RATE_010776C8);
            let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
            let angle = x87_store32(x87_mul(f64::from(now), rate));
            let wobble_x = x87_store32(e.call(WOBBLE_00C5D320, &args![q(angle)]).f64());
            let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
            let angle = x87_store32(x87_mul(f64::from(now.wrapping_add(0x3039)), rate));
            let wobble_y = x87_store32(e.call(WOBBLE_00C5D320, &args![q(angle)]).f64());
            let x = q(e.mem.u32(magnitude));
            let v = interpolate_between(e, q(range[2]), q(range[3]), zero, one, x);
            e.mem.set_u32(magnitude, x87_store32(v));
            let minus_half = q(e.global::<u32>(MINUS_HALF_0102295C));
            let half = q(e.global::<u32>(HALF_01016248));
            let high = signed_by(wobble_x, range[3]);
            let low = signed_by(wobble_x, range[2]);
            let v = interpolate_between(e, low, high, minus_half, half, q(wobble_x));
            let resist_x = x87_store32(x87_mul(v, x87_m32(e.mem.u32(magnitude))));
            let high = signed_by(wobble_y, range[3]);
            let low = signed_by(wobble_y, range[2]);
            let v = interpolate_between(e, low, high, minus_half, half, q(wobble_y));
            let resist_y = x87_store32(x87_mul(v, x87_m32(e.mem.u32(magnitude))));
            let x = q(e.mem.u32(pull));
            let v = interpolate_between(e, q(range[6]), q(range[7]), zero, one, x);
            e.mem.set_u32(pull, x87_store32(v));
            // `MOV EAX,1; TEST EAX,EAX`: always taken.
            e.mem.set_u32(base + 0x7c, q(resist_x));
            e.mem.set_u32(base + 0x80, q(resist_y));
            e.call(POINT2_ADD_004E88D0, &args![base + 0x74, base + 0x7c]);
            let holder = e.global::<u32>(INPUT_HOLDER_011DEA0C);
            let device = e.call(INPUT_DEVICE_00877720, &args![holder]).u32();
            e.call(NODE_SELF, &args![point]);
            e.call(INTERFACE_MANAGER, &args![]);
            if e.call(GAMEPAD_IN_USE_004B71D0, &args![]).bool() {
                let x = e
                    .call(THUMBSTICK_00A23390, &args![device, 0u32, 7u32])
                    .i32();
                let y = e
                    .call(THUMBSTICK_00A23390, &args![device, 0u32, 8u32])
                    .i32();
                let step = stick_step(e, x);
                e.mem.set_u32(point, x87_store32(f64::from(step)));
                let step = stick_step(e, y.wrapping_neg());
                e.mem.set_u32(point + 4, x87_store32(f64::from(step)));
                let quarter = q(e.global::<u32>(QUARTER_0101622C));
                let x = q(e.mem.u32(pull));
                let scale = interpolate_between(e, quarter, half, zero, one, x);
                let scaled = e
                    .call(
                        POINT2_SCALE_004A4F40,
                        &args![point, local(0x70), x87_store32(scale)],
                    )
                    .u32();
                e.call(POINT2_ADD_004E88D0, &args![base + 0x74, scaled]);
            } else {
                let x = e.call(MOUSE_AXIS_00A239E0, &args![device, 1u32]).i32();
                e.mem.set_u32(point, x87_store32(f64::from(x)));
                let y = e.call(MOUSE_AXIS_00A239E0, &args![device, 2u32]).i32();
                e.mem.set_u32(point + 4, x87_store32(f64::from(y)));
                let scaled = e
                    .call(
                        POINT2_SCALED_00532FD0,
                        &args![local(0x78), q(e.mem.u32(pull)), point],
                    )
                    .u32();
                e.call(POINT2_ADD_004E88D0, &args![base + 0x74, scaled]);
            }
            let product = x87_mul(x87_load32(e.mem.u32(magnitude)), x87_m32(e.mem.u32(pull)));
            let length = e.call(POINT_LENGTH_00589850, &args![point]).f64();
            e.mem
                .set_u32(base + 0x84, x87_store32(x87_mul(length, product)));
            if x87_load32(e.mem.u32(base + 0x84)) > e.global::<f64>(TEN_DOUBLE) {
                let cap = q(e.global::<u32>(MOVEMENT_CAP_01017B78));
                e.mem.set_u32(base + 0x84, cap);
            }
            let movement = x87_div(
                x87_load32(e.mem.u32(base + 0x84)),
                e.global::<f64>(FIVE_01020998),
            );
            e.mem.set_u32(base + 0x84, x87_store32(movement));
            let y = x87_load32(e.mem.u32(base + 0x78));
            let y = e.call(FTOL2, &args![y]).i32();
            let x = x87_load32(e.mem.u32(base + 0x74));
            let x = e.call(FTOL2, &args![x]).i32();
            surgery_menu_place_tool_in_window(e, this, x, y);
            let scale = x87_store32(e.call(CURSOR_SCALE_0070ECB0, &args![]).f64());
            let tile = e.mem.u32(base + 0x44);
            let width = e.call(TILE_00A01440, &args![tile]).f64();
            let screen = e.call(SCREEN_004DC200, &args![]).i32();
            let across = x87_store32(x87_mul(x87_div(width, f64::from(screen)), x87_m32(scale)));
            let tile = e.mem.u32(base + 0x44);
            let height = e.call(TILE_00A013D0, &args![tile]).f64();
            let screen = e.call(SCREEN_004DC1F0, &args![]).i32();
            let down = x87_store32(x87_mul(x87_div(height, f64::from(screen)), x87_m32(scale)));
            e.call(
                SET_CURSOR_POSITION_007151B0,
                &args![manager, q(down), q(across)],
            );
        } else {
            e.mem.set_u8(base + 0x90, 0);
            e.mem.set_u32(base + 0x84, x87_store32(0.0));
            let tile = e.mem.u32(base + 0x5c);
            e.call(TILE_SET_INT, &args![tile, 0xfa3u32, 0u32]);
            e.call(INTERFACE_MANAGER, &args![]);
            if e.call(GAMEPAD_IN_USE_004B71D0, &args![]).bool() {
                let holder = e.global::<u32>(INPUT_HOLDER_011DEA0C);
                let device = e.call(INPUT_DEVICE_00877720, &args![holder]).u32();
                let x = e
                    .call(THUMBSTICK_00A23390, &args![device, 0u32, 7u32])
                    .i32();
                let y = e
                    .call(THUMBSTICK_00A23390, &args![device, 0u32, 8u32])
                    .i32();
                let step_x = stick_step(e, x);
                let step_y = stick_step(e, y.wrapping_neg());
                let tile = e.mem.u32(base + 0x44);
                let top = e.call(TILE_GET_FLOAT, &args![tile, 0xfa2u32]).f64();
                let top = e.call(FTOL2, &args![top]).i32().wrapping_add(step_y);
                let tile = e.mem.u32(base + 0x44);
                let left = e.call(TILE_GET_FLOAT, &args![tile, 0xfa1u32]).f64();
                let left = e.call(FTOL2, &args![left]).i32().wrapping_add(step_x);
                surgery_menu_place_tool_in_window(e, this, left, top);
            } else {
                let y = e.call(CURSOR_Y_0073B020, &args![manager]).i32();
                let x = e.call(CURSOR_X_00747D00, &args![manager]).i32();
                surgery_menu_place_tool_in_window(e, this, x, y);
            }
        }
    });
}

/// The pain floor of the wound: `GetPainCausedByWound` of the menu tile's trait
/// `011dad9c` over 100, times 100.
fn pain_floor(e: &mut Engine, this: Ptr) -> f64 {
    let value = menu_trait_float(e, this, TRAIT_011DAD9C);
    let value = x87_store32(x87_div(value, e.global::<f64>(HUNDRED_DOUBLE)));
    let pain = e
        .call(GET_PAIN_CAUSED_BY_WOUND, &args![x87_quiet_bits(value)])
        .f64();
    x87_mul(pain, e.global::<f64>(HUNDRED_DOUBLE))
}

// Translated from 007e23c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e23c0 - the /GS cookie check (00ec408c) around the text buffer is not translated.
/// `SurgeryMenu::DoIdle` (Xbox PDB; `thiscall`): not the top menu sets the menu tile's
/// `"_IsTopMenu"` to 0 and stops. Otherwise it is set to 1 and the animations updated.
/// With the gamepad: button 9 held (2) takes the tool location from tile +0x44's
/// 0xfa1/0xfa2; button 9 pressed (0) burns (`UpdateToolLocation(true)`, `BurnFlesh`),
/// else idle. With the mouse: control 4 held moves the tool location to the cursor;
/// the menu tile's 0xfa9 is 0 inside the window (`007e4540`), else 255; the tool stays
/// buried (+0x91) while control 4 is down, or it enters inside the window; buried burns,
/// else idle. Idle: the pain (+0xac) falls by `PainChangePerSecond(1)` per second since
/// the pain timer (+0xb0), not below the wound's floor; both timers (+0xb0, +0x8c) are
/// now; `UpdateToolLocation(false)`; both sounds stopped. Then the skin's shading
/// (`007e3750`), tile +0x54's 0x1004 the pain over 100 and tile +0x48's `"_Value"`
/// `"health/max"` (actor values 0x10).
pub fn surgery_menu_do_idle(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if !e.call(IS_TOP_MENU_ID, &args![SURGERY_MENU_CLASS]).bool() {
        let which = e.call(TEXT_TO_TRAIT, &args![IS_TOP_MENU_NAME]).u32();
        let tile = e.call(MENU_TILE_OF, &args![base]).u32();
        e.call(TILE_SET_INT, &args![tile, which, 0u32]);
        return;
    }
    let which = e.call(TEXT_TO_TRAIT, &args![IS_TOP_MENU_NAME]).u32();
    let tile = e.call(MENU_TILE_OF, &args![base]).u32();
    e.call(TILE_SET_INT, &args![tile, which, 1u32]);
    surgery_menu_update_animations(e, this);
    let mut idle = false;
    e.call(INTERFACE_MANAGER, &args![]);
    if e.call(GAMEPAD_IN_USE_004B71D0, &args![]).bool() {
        if e.call(CHECK_MENU_BUTTON, &args![9u32, 1u32]).i32() == 2 {
            let tile = e.mem.u32(base + 0x44);
            let x = e.call(TILE_GET_FLOAT, &args![tile, 0xfa1u32]).f64();
            e.mem.set_u32(base + 0x74, x87_store32(x));
            let tile = e.mem.u32(base + 0x44);
            let y = e.call(TILE_GET_FLOAT, &args![tile, 0xfa2u32]).f64();
            e.mem.set_u32(base + 0x78, x87_store32(y));
        }
        if e.call(CHECK_MENU_BUTTON, &args![9u32, 1u32]).i32() == 0 {
            surgery_menu_update_tool_location(e, this, true);
            surgery_menu_burn_flesh(e, this);
        } else {
            idle = true;
        }
    } else {
        let holder = e.global::<u32>(INPUT_HOLDER_011DEA0C);
        let device = e.call(INPUT_DEVICE_00877720, &args![holder]).u32();
        if e.call(CONTROL_STATE, &args![device, 4u32, 1u32]).u32() != 0 {
            let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
            let x = e.call(CURSOR_X_00747D00, &args![manager]).i32();
            e.mem.set_u32(base + 0x74, x87_store32(f64::from(x)));
            let y = e.call(CURSOR_Y_0073B020, &args![manager]).i32();
            e.mem.set_u32(base + 0x78, x87_store32(f64::from(y)));
        }
        let inside = fn_007e4540(e, this);
        let alpha = if inside { 0u32 } else { 0xff };
        let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
        let menu_tile = e.call(MANAGER_MENU_TILE, &args![manager]).u32();
        e.call(TILE_SET_INT, &args![menu_tile, 0xfa9u32, alpha]);
        let held = e.call(CONTROL_STATE, &args![device, 4u32, 0u32]).u32() != 0
            && e.mem.u8(base + 0x91) != 0;
        let buried =
            held || (e.call(CONTROL_STATE, &args![device, 4u32, 1u32]).u32() != 0 && inside);
        e.mem.set_u8(base + 0x91, u8::from(buried));
        if buried {
            surgery_menu_update_tool_location(e, this, true);
            surgery_menu_burn_flesh(e, this);
        } else {
            idle = true;
        }
    }
    if idle {
        let rate = e
            .call(PAIN_CHANGE_PER_SECOND, &args![x87_store32(1.0)])
            .f64();
        let rate = x87_store32(rate);
        let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
        let elapsed = f64::from(now.wrapping_sub(e.mem.u32(base + 0xb0)));
        let fall = x87_mul(
            x87_div(elapsed, e.global::<f64>(THOUSAND_DOUBLE)),
            x87_m32(rate),
        );
        let pain = x87_sub(x87_m32(e.mem.u32(base + 0xac)), fall);
        e.mem.set_u32(base + 0xac, x87_store32(pain));
        let floor = pain_floor(e, this);
        if x87_load32(e.mem.u32(base + 0xac)) < floor {
            let floor = pain_floor(e, this);
            e.mem.set_u32(base + 0xac, x87_store32(floor));
        }
        let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
        e.mem.set_u32(base + 0xb0, now);
        e.mem.set_u32(base + 0x8c, now);
        surgery_menu_update_tool_location(e, this, false);
        for offset in [0x94u32, 0xa0] {
            if e.call(SOUND_IS_PLAYING, &args![base + offset]).bool() {
                e.call(SOUND_STOP, &args![base + offset]);
            }
        }
    }
    fn_007e3750(e, this);
    let shown = x87_store32(x87_div(
        x87_load32(e.mem.u32(base + 0xac)),
        e.global::<f64>(HUNDRED_DOUBLE),
    ));
    let tile = e.mem.u32(base + 0x54);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, 0x1004u32, x87_quiet_bits(shown), 1u32],
    );
    let player = e.global::<u32>(PLAYER);
    let most = e.vcall(player + 0xa4, 0x1c, &args![0x10u32]).u32();
    let player = e.global::<u32>(PLAYER);
    let health = e.vcall(player + 0xa4, 8, &args![0x10u32]).u32();
    e.with_stack(0x100, |e, buffer| {
        let text = buffer.addr();
        e.call(
            SPRINTF_S,
            &args![text, 0x100u32, HEALTH_FORMAT, health, most],
        );
        let which = e.call(TEXT_TO_TRAIT, &args![VALUE_TRAIT_NAME]).u32();
        let tile = e.mem.u32(base + 0x48);
        e.call(TILE_SET_STRING, &args![tile, which, text, 1u32]);
    });
}

/// Plays the named sound once through a local handle (constructed, assigned from the
/// audio's handle by name with flags 0x121, played, destroyed).
fn play_named_sound(e: &mut Engine, name: u32) {
    e.with_stack(0x18, |e, frame| {
        let (sound, found) = (frame.addr() + 0xc, frame.addr());
        e.call(SOUND_HANDLE_CONSTRUCT, &args![sound]);
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let handle = e
            .call(
                GET_SOUND_HANDLE_BY_NAME,
                &args![audio, found, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![sound, handle]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![found]);
        e.call(SOUND_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![sound]);
    });
}

// Translated from 007e28b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e28b0 - the C++ exception-unwinding frame (FS:[0] around the sound handles) and the /GS cookie check (00ec408c) are not translated.
/// `SurgeryMenu::BurnFlesh` (Xbox PDB; `thiscall`): `UpdateFlesh`, then
/// `PainChangePerSecond` of the tool movement (+0x84; the answer unused). The pain
/// (+0xac) rises by 30 per second since the pain timer (+0xb0), at most 100. One time in
/// five (`00944460(0, 5)` answering 1) a pain sound plays as the pain crosses 60
/// (`"UISurgeryPain01"`), 85 (`02`) or reaches 100 (`03`). The pain timer is now; the
/// limb's actor value is restored by `ShowLimbCondition` less its current value. Once
/// the time since the hit-point timer (+0xb4) passes the threshold (1000 down to
/// `500 - 400 * luck / 100` as the pain goes 0..100), the player is hurt (slot +0x3b0
/// with 0x10, -1, 0) and the timer is now. Tile +0x50's `"_Value"` shows the luck `"%"`.
pub fn surgery_menu_burn_flesh(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let _flesh = surgery_menu_update_flesh(e, this);
    let movement = x87_quiet_bits(e.mem.u32(base + 0x84));
    e.call(PAIN_CHANGE_PER_SECOND, &args![movement]);
    let old = x87_quiet_bits(e.mem.u32(base + 0xac));
    let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
    let elapsed = f64::from(now.wrapping_sub(e.mem.u32(base + 0xb0)));
    let rise = x87_mul(
        x87_div(elapsed, e.global::<f64>(THOUSAND_DOUBLE)),
        e.global::<f64>(THIRTY_0101DB88),
    );
    let pain = x87_add(rise, x87_m32(e.mem.u32(base + 0xac)));
    e.mem.set_u32(base + 0xac, x87_store32(pain));
    if x87_load32(e.mem.u32(base + 0xac)) > e.global::<f64>(HUNDRED_DOUBLE) {
        let most = x87_quiet_bits(e.global::<u32>(HUNDRED_01016410));
        e.mem.set_u32(base + 0xac, most);
    }
    if e.call(RANDOM_INT, &args![0u32, 5u32]).i32() == 1 {
        let before = x87_load32(old);
        let sixty = e.global::<f64>(SIXTY_01012638);
        let eighty_five = e.global::<f64>(EIGHTY_FIVE_01072FF0);
        let hundred = e.global::<f64>(HUNDRED_DOUBLE);
        let pain = x87_load32(e.mem.u32(base + 0xac));
        let sound = if before < sixty && pain > sixty {
            Some(PAIN_SOUND_ONE)
        } else if before < eighty_five && pain > eighty_five {
            Some(PAIN_SOUND_TWO)
        } else if before < hundred && pain == hundred {
            Some(PAIN_SOUND_THREE)
        } else {
            None
        };
        if let Some(name) = sound {
            play_named_sound(e, name);
        }
    }
    let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
    e.mem.set_u32(base + 0xb0, now);
    let shown = surgery_menu_show_limb_condition(e, this);
    let limb = e.mem.u32(base + 0xc0);
    let player = e.global::<u32>(PLAYER);
    let current = e.vcall(player + 0xa4, 8, &args![limb]).i32();
    let change = x87_store32(f64::from(shown.wrapping_sub(current)));
    let limb = e.mem.u32(base + 0xc0);
    let player = e.global::<u32>(PLAYER);
    e.call(ACTOR_RESTORE_ACTOR_VALUE, &args![player, limb, change]);
    let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
    let since = f64::from(now.wrapping_sub(e.mem.u32(base + 0xb4)));
    let q = x87_quiet_bits;
    let pain = q(e.mem.u32(base + 0xac));
    let zero = x87_store32(0.0);
    let hundred = q(e.global::<u32>(HUNDRED_01016410));
    let thousand = q(e.global::<u32>(THOUSAND_01013974));
    let player = e.global::<u32>(PLAYER);
    let luck = e.vcall(player + 0xa4, 8, &args![0x25u32]).i32();
    let five_hundred = q(e.global::<u32>(FIVE_HUNDRED_01013D84));
    let luck = x87_store32(f64::from(luck));
    let quickest = interpolate_between(e, hundred, five_hundred, zero, hundred, luck);
    let threshold = interpolate_between(e, x87_store32(quickest), thousand, hundred, zero, pain);
    if threshold < since {
        let player = e.global::<u32>(PLAYER);
        e.vcall(player, 0x3b0, &args![0x10u32, 0xffff_ffffu32, 0u32]);
        let now = e.call(MILLISECONDS, &args![CLOCK_011F6394]).u32();
        e.mem.set_u32(base + 0xb4, now);
    }
    let player = e.global::<u32>(PLAYER);
    let luck = e.vcall(player + 0xa4, 8, &args![0x25u32]).u32();
    e.with_stack(0x100, |e, buffer| {
        let text = buffer.addr();
        e.call(SPRINTF_S, &args![text, 0x100u32, PERCENT_FORMAT, luck]);
        let which = e.call(TEXT_TO_TRAIT, &args![VALUE_TRAIT_NAME]).u32();
        let tile = e.mem.u32(base + 0x50);
        e.call(TILE_SET_STRING, &args![tile, which, text, 1u32]);
    });
}

/// The limb after `limb` by `DoClick`'s button 14 (`next`) or 15 (else): the six limb
/// actor values 0x19..0x1e in turn (-1 counts as 0x19); others stay.
fn neighbour_limb(limb: i32, next: bool) -> Option<i32> {
    let order = [0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e];
    let at = if limb == -1 {
        0
    } else {
        order.iter().position(|&l| l == limb)?
    };
    let to = if next { (at + 1) % 6 } else { (at + 5) % 6 };
    Some(order[to])
}

/// `DoClick`'s common tail after choosing a limb: the pain from the limb's condition,
/// tile +0x70's `"_Value"` the condition over 100, tile +0x4c's 0xfc4 the limb's name,
/// the skin switched from `old` and the new skin's sequences prepared.
fn show_chosen_limb(e: &mut Engine, this: Ptr, old: i32) {
    let base = this.addr();
    let limb = e.mem.u32(base + 0xc0);
    let player = e.global::<u32>(PLAYER);
    let condition = e.vcall(player + 0xa4, 0xc, &args![limb]).f64();
    let pain = e
        .call(GET_PAIN_CAUSED_BY_WOUND, &args![x87_store32(condition)])
        .f64();
    e.mem.set_u32(base + 0xac, x87_store32(pain));
    let limb = e.mem.u32(base + 0xc0);
    let player = e.global::<u32>(PLAYER);
    let condition = e.vcall(player + 0xa4, 0xc, &args![limb]).f64();
    let shown = x87_store32(x87_div(condition, e.global::<f64>(HUNDRED_DOUBLE)));
    let which = e.call(TEXT_TO_TRAIT, &args![VALUE_TRAIT_NAME]).u32();
    let tile = e.mem.u32(base + 0x70);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, which, x87_quiet_bits(shown), 1u32],
    );
    let limb = e.mem.u32(base + 0xc0);
    let name = e.call(ACTOR_VALUE_NAME, &args![limb]).u32();
    let tile = e.mem.u32(base + 0x4c);
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, name, 1u32]);
    let limb = e.mem.u32(base + 0xc0) as i32;
    surgery_menu_switch_skin_geometry(e, this, old, limb);
    let limb = e.mem.u32(base + 0xc0) as i32;
    let index = fn_007e5e30(e, this, limb);
    let me = e.global::<u32>(SURGERY_MENU_ME);
    let skin = e
        .call(
            FIRST_WORD,
            &args![me.wrapping_add(0xc8).wrapping_add(index.wrapping_mul(4))],
        )
        .u32();
    surgery_menu_prepare_sequences(e, skin);
}

/// The extra data of an inventory entry for equipping: its list's first item, or 0.
fn first_extra(e: &mut Engine, entry: u32) -> u32 {
    if e.call(FIRST_WORD, &args![entry]).u32() != 0 {
        let list = e.call(FIRST_WORD, &args![entry]).u32();
        node_item(e, list)
    } else {
        0
    }
}

// Translated from 007e1b10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007e1b10 - the C++ exception-unwinding frame (FS:[0] around the trait statics) is not translated.
/// `SurgeryMenu::DoClick` (Xbox PDB; `thiscall`, the tile and an unused word): looks up
/// `"_StimpacCount"` and `"_MorphineCount"` once; by the tile's id (`00720010`): 1
/// closes; 2 attempts the surgery automatically (`GetSurgeryAutoAttemptPercentages` of
/// luck, the default object 4's effect on the player and the limb's condition: the limb
/// healed by the success share plus its sequences' percent, at most 100, the limb's
/// actor value restored and the player hurt by the damage share, the wound marked
/// reset); 4 uses a stimpak (+0xbc, when it has any: the count trait less one, equipped)
/// and starts animation 3; 5 uses morphine (when the count is above 0: the player's
/// default object 4 equipped and its entry deleted, the count less one, the pain less
/// 25 and the wound trait `011dad9c` less 25 or 0) and starts animation 2; 14 and 15
/// choose the next or previous limb.
pub fn surgery_menu_do_click(e: &mut Engine, this: Ptr, tile: u32, _unused: u32) {
    let base = this.addr();
    lazy_trait(
        e,
        CLICK_TRAIT_GUARD,
        1,
        STIMPAK_COUNT_NAME,
        STIMPAK_COUNT_TRAIT,
    );
    lazy_trait(
        e,
        CLICK_TRAIT_GUARD,
        2,
        MORPHINE_COUNT_NAME,
        MORPHINE_COUNT_TRAIT,
    );
    let id = e.call(TILE_ID_00720010, &args![base, tile]).i32();
    let q = x87_quiet_bits;
    match id {
        1 => surgery_menu_close(e),
        2 => {
            let object = e.call(GET_DEFAULT_OBJECT, &args![4u32]).u32();
            let effects = if object != 0 { object + 0x30 } else { 0 };
            let player = e.global::<u32>(PLAYER);
            let affected = e
                .call(
                    MAGIC_TARGET_IS_SPELL_TARGET,
                    &args![player + 0x94, effects, 1u32],
                )
                .u8();
            let limb = e.mem.u32(base + 0xc0);
            let player = e.global::<u32>(PLAYER);
            let condition = e.vcall(player + 0xa4, 0xc, &args![limb]).f64();
            let condition = x87_store32(condition);
            let player = e.global::<u32>(PLAYER);
            let luck = e.vcall(player + 0xa4, 8, &args![0x25u32]).u32();
            e.with_stack(0x20, |e, frame| {
                let ebp = frame.addr() + 0x20;
                let (success, damage) = (ebp - 0x14, ebp - 0x1c);
                e.call(
                    GET_SURGERY_AUTO_ATTEMPT_PERCENTAGES,
                    &args![success, damage, luck, u32::from(affected), condition],
                );
                let hundred = q(e.global::<u32>(HUNDRED_01016410));
                let percent = surgery_menu_get_limb_percent_from_sequences(e, this);
                let healed = x87_add(
                    x87_mul(f64::from(percent), e.global::<f64>(HUNDRED_DOUBLE)),
                    x87_m32(e.mem.u32(success)),
                );
                let healed = x87_store32(healed);
                let capped = e.call(SMALLER_OF, &args![q(healed), hundred]).f64();
                let capped = x87_store32(capped);
                let left = x87_store32(x87_sub(
                    1.0,
                    x87_div(x87_load32(capped), e.global::<f64>(HUNDRED_DOUBLE)),
                ));
                surgery_menu_set_limb_sequences(e, this, f32::from_bits(q(left)));
                let shown = surgery_menu_show_limb_condition(e, this);
                let limb = e.mem.u32(base + 0xc0);
                let player = e.global::<u32>(PLAYER);
                let current = e.vcall(player + 0xa4, 8, &args![limb]).i32();
                let change = x87_store32(f64::from(shown.wrapping_sub(current)));
                let limb = e.mem.u32(base + 0xc0);
                let player = e.global::<u32>(PLAYER);
                e.call(ACTOR_RESTORE_ACTOR_VALUE, &args![player, limb, change]);
                let hurt = x87_store32(-x87_load32(e.mem.u32(damage)));
                let player = e.global::<u32>(PLAYER);
                e.vcall(player, 0x3ac, &args![0x10u32, hurt, 0u32]);
            });
            let me = e.global::<u32>(SURGERY_MENU_ME);
            let wound = surgery_menu_get_current_limb_wound(e, Ptr::new(me));
            e.mem.set_u8(wound + 0xc, 1);
        }
        4 => {
            let me = e.global::<u32>(SURGERY_MENU_ME);
            let stimpaks = e.mem.u32(me + 0xbc);
            if stimpaks != 0 && e.call(ITEM_COUNT, &args![stimpaks]).i32() > 0 {
                let count = menu_trait_float(e, this, STIMPAK_COUNT_TRAIT);
                let count = x87_store32(x87_sub(count, e.global::<f64>(ONE_01012070)));
                let which = e.global::<u32>(STIMPAK_COUNT_TRAIT);
                let tile = e.call(MENU_TILE_OF, &args![base]).u32();
                e.call(TILE_SET_FLOAT, &args![tile, which, q(count), 1u32]);
                let me = e.global::<u32>(SURGERY_MENU_ME);
                let stimpaks = e.mem.u32(me + 0xbc);
                let extra = first_extra(e, stimpaks);
                let count = e.call(ITEM_COUNT, &args![stimpaks]).u32();
                let item = e.call(ITEM_GET_FORM, &args![stimpaks]).u32();
                let player = e.global::<u32>(PLAYER);
                e.call(
                    ACTOR_QUEUE_EQUIP_OBJECT,
                    &args![player, item, count, extra, 0u32, 0u32, 1u32],
                );
            }
            surgery_menu_set_animation(e, this, 3);
        }
        5 => {
            let count = menu_trait_float(e, this, MORPHINE_COUNT_TRAIT);
            if count > e.global::<f64>(ZERO_DOUBLE) {
                let object = e.call(GET_DEFAULT_OBJECT, &args![4u32]).u32();
                let player = e.global::<u32>(PLAYER);
                let entry = e
                    .call(REFERENCE_GET_INVENTORY_ITEM, &args![player, object, 0u32])
                    .u32();
                let count = menu_trait_float(e, this, MORPHINE_COUNT_TRAIT);
                let count = x87_store32(x87_sub(count, e.global::<f64>(ONE_01012070)));
                let which = e.global::<u32>(MORPHINE_COUNT_TRAIT);
                let tile = e.call(MENU_TILE_OF, &args![base]).u32();
                e.call(TILE_SET_FLOAT, &args![tile, which, q(count), 1u32]);
                let extra = first_extra(e, entry);
                let count = e.call(ITEM_COUNT, &args![entry]).u32();
                let object = e.call(GET_DEFAULT_OBJECT, &args![4u32]).u32();
                let player = e.global::<u32>(PLAYER);
                e.call(
                    ACTOR_QUEUE_EQUIP_OBJECT,
                    &args![player, object, count, extra, 0u32, 0u32, 1u32],
                );
                if entry != 0 {
                    e.call(ITEM_CHANGE_DELETE, &args![entry, 1u32]);
                }
                let pain = x87_sub(
                    x87_load32(e.mem.u32(base + 0xac)),
                    e.global::<f64>(TWENTY_FIVE_DOUBLE),
                );
                e.mem.set_u32(base + 0xac, x87_store32(pain));
                let wound = menu_trait_float(e, this, TRAIT_011DAD9C);
                if wound >= e.global::<f64>(TWENTY_FIVE_DOUBLE) {
                    let wound = menu_trait_float(e, this, TRAIT_011DAD9C);
                    let wound = x87_store32(x87_sub(wound, e.global::<f64>(TWENTY_FIVE_DOUBLE)));
                    let which = e.global::<u32>(TRAIT_011DAD9C);
                    let tile = e.call(MENU_TILE_OF, &args![base]).u32();
                    e.call(TILE_SET_FLOAT, &args![tile, which, q(wound), 1u32]);
                } else {
                    let which = e.global::<u32>(TRAIT_011DAD9C);
                    let tile = e.call(MENU_TILE_OF, &args![base]).u32();
                    e.call(TILE_SET_INT, &args![tile, which, 0u32]);
                }
            }
            surgery_menu_set_animation(e, this, 2);
        }
        14 | 15 => {
            let old = e.mem.u32(base + 0xc0) as i32;
            if let Some(limb) = neighbour_limb(old, id == 14) {
                e.mem.set_u32(base + 0xc0, limb as u32);
            }
            show_chosen_limb(e, this, old);
        }
        _ => {}
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007e_1970, fn_007e1970() -> u32),
        entry!(0x007e_1980, surgery_menu_close()),
        entry!(0x007e_3750, fn_007e3750(Ptr)),
        entry!(0x007e_41f0, fn_007e41f0(i32, i32)),
        entry!(0x007e_4260, surgery_menu_do_key(Ptr, u32) -> bool),
        entry!(0x007e_4fc0, fn_007e4fc0(Ptr, u32) -> u32),
        entry!(0x007e_5420, fn_007e5420(u32, u32, u32) -> u32),
        entry!(0x007e_5540, fn_007e5540(Ptr, i32) -> u32),
        entry!(
            0x007e_5720,
            surgery_menu_get_current_limb_wound(Ptr) -> u32
        ),
        entry!(0x007e_5780, surgery_menu_limb_wound_reset_limb(Ptr)),
        entry!(0x007e_5810, surgery_menu_reset_limb(u32)),
        entry!(0x007e_5890, surgery_menu_reset_all_limbs()),
        entry!(
            0x007e_58e0,
            surgery_menu_get_limb_percent_from_sequences(Ptr) -> f32
        ),
        entry!(0x007e_59f0, surgery_menu_set_limb_sequences(Ptr, f32)),
        entry!(0x007e_4540, fn_007e4540(Ptr) -> bool),
        entry!(0x007e_47a0, fn_007e47a0()),
        entry!(0x007e_4fe0, fn_007e4fe0(f32, f32, f32)),
        entry!(0x007e_5170, surgery_menu_shutdown_3d_elements()),
        entry!(
            0x007e_5250,
            surgery_menu_sort_sequence_by_cursor_distance(u32, u32) -> i32
        ),
        entry!(0x007e_5590, surgery_menu_show_limb_condition(Ptr) -> i32),
        entry!(0x007e_5a90, surgery_menu_update_animations(Ptr)),
        entry!(0x007e_5c80, surgery_menu_set_animation(Ptr, u32)),
        entry!(0x007e_5e30, fn_007e5e30(Ptr, i32) -> u32),
        entry!(
            0x007e_5ee0,
            surgery_menu_switch_skin_geometry(Ptr, i32, i32)
        ),
        entry!(0x007e_5ff0, surgery_menu_place_decal(Ptr, f32, f32, u8)),
        entry!(
            0x007e_42c0,
            surgery_menu_place_tool_in_window(Ptr, i32, i32)
        ),
        entry!(0x007e_4920, surgery_menu_prepare_sequences(u32)),
        entry!(0x007e_2de0, surgery_menu_update_flesh(Ptr) -> f32),
        entry!(0x007e_3890, surgery_menu_update_tool_location(Ptr, bool)),
        entry!(0x007e_23c0, surgery_menu_do_idle(Ptr)),
        entry!(0x007e_28b0, surgery_menu_burn_flesh(Ptr)),
        entry!(0x007e_1b10, surgery_menu_do_click(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }
    const MENU_TILE: u32 = 0x5000;
    const TOOL_TILE: u32 = 0x4400;

    /// Every callee of this unit, stubbed (0) unless a test answers it.
    const STUBBED: &[u32] = &[
        LIST_IS_EMPTY,
        NODE_SELF,
        NODE_NEXT,
        FIRST_WORD,
        LIST_CLEAR,
        GET_MENU_BY_CLASS,
        IS_TOP_MENU_ID,
        TILE_SET_INT,
        TILE_SET_FLOAT,
        TILE_SET_STRING,
        TEXT_TO_TRAIT,
        MENU_START_FADE_OUT,
        INTERFACE_MANAGER,
        MANAGER_MENU_TILE,
        GAMEPAD_IN_USE_004B71D0,
        SOUND_HANDLE_CONSTRUCT,
        SOUND_HANDLE_ASSIGN,
        SOUND_HANDLE_DESTRUCT,
        AUDIO_INSTANCE,
        GET_SOUND_HANDLE_BY_NAME,
        SOUND_PLAY,
        SOUND_STOP,
        SOUND_RELEASE,
        SCREEN_004DC1F0,
        SCREEN_004DC200,
        SET_CURSOR_00703080,
        FIXED_STRING_CONSTRUCT,
        FIXED_STRING_DESTRUCT,
        INTERPOLATE_004B3AB0,
        MILLISECONDS,
        UPDATE_DATA_INIT,
        UPDATE_OBJECT,
        ACTOR_VALUE_NAME,
        SEQUENCE_00493770,
        SEQUENCE_LENGTH,
        SEQUENCE_00759450,
        SEQUENCE_SET_PHASE,
        WORLD_PT_TO_SCREEN_PT,
        WORLD_TRANSLATE_0045BB80,
        GET_SCREEN_WIDTH,
        GET_SCREEN_HEIGHT_00715DA0,
        POINT_DIFFERENCE_004E8880,
        POINT_LENGTH_00589850,
        FTOL2,
        CURSOR_X_00747D00,
        CURSOR_Y_0073B020,
        RENDERER,
        RENDERER_007148C0,
        GETTER_0045C670,
        CULLER_008D80E0,
        CULLER_00C4F270,
        CAMERA_00709D50,
        SCENE_NODE_00450B80,
        TEXTURE_007FD640,
        NI_POINTER_CONSTRUCT,
        NI_POINTER_ASSIGN,
        NI_POINTER_DESTRUCT,
        CULLER_SET_ACCUMULATOR,
        ACCUMULATE_SCENE,
        RENDER_00B6C0D0,
        CULLER_00C4F2D0,
        SET_APP_CULLED,
        SETTING_VALUE,
        SET_TRANSLATE,
        SET_SCALE_00440490,
        SCENE_00B5D180,
        CLONE_INVENTORY_3D,
        PROPERTY_005D9660,
        REMOVE_PROPERTY,
        UPDATE_PROPERTIES,
        CONTROLLER_MANAGER_0043B230,
        SEQUENCE_LOOKUP,
        FRAME_TIME_0084D030,
        SEQUENCE_UPDATE,
        PARENT_004A8A90,
        TYPE_TEST_00653270,
        GET_CONTROLLER,
        DEACTIVATE_ALL,
        PLAY_SEQUENCE,
        MANAGER_0047AA40,
        SEQUENCE_004954C0,
        START_ANIMATIONS,
        CAMERA_RAY,
        NI_PICK_CONSTRUCT,
        PICK_00458B30,
        PICK_SET_TARGET,
        PICK_OBJECTS,
        PICK_RESULTS,
        PICK_RECORD,
        RECORD_POINT_00413F40,
        FORM_BY_EDITOR_ID,
        DYNAMIC_CAST,
        DECAL_DATA_CONSTRUCT,
        POINT_FROM_FLOATS,
        DECAL_004A4240,
        PLAYER_008D6F30,
        DECAL_MANAGER,
        ADD_DECAL,
        NI_PICK_DESTRUCT,
        POINT_NORMALIZE_004E7560,
        POINT_SCALE_006AD180,
        SEQUENCE_COUNT_00495D00,
        GET_SEQUENCE_AT,
        OBJECT_NAME_SOURCE_00413F40,
        OBJECT_NAME_0043B1B0,
        SPRINTF,
        STRNCMP,
        BLOCK_COUNT_0084E3A0,
        BLOCK_TEST_004937A0,
        REMOVE_BLOCK_00A32B80,
        SEQUENCE_OWNER_005F5F80,
        SEQUENCE_MANAGER_008D6F30,
        ALLOCATE,
        LIST_CONSTRUCT,
        LIST_ADD,
        LIST_SORT_0083FD60,
        LIST_COUNT_005AE380,
        SEQUENCE_PHASE_00508120,
        MAGNITUDE_00408840,
        WOBBLE_00C5D320,
        WOUND_FORMULA_00648D70,
        SOUND_IS_PLAYING,
        ROOT_TRANSLATE_0043C490,
        TILE_GET_FLOAT,
        ADD_USER_TRAIT,
        INPUT_DEVICE_00877720,
        THUMBSTICK_00A23390,
        MOUSE_AXIS_00A239E0,
        CONTROL_STATE,
        CHECK_MENU_BUTTON,
        POINT2_ADD_004E88D0,
        POINT2_SCALE_004A4F40,
        POINT2_SCALED_00532FD0,
        CURSOR_SCALE_0070ECB0,
        TILE_00A01440,
        TILE_00A013D0,
        SET_CURSOR_POSITION_007151B0,
        PAIN_CHANGE_PER_SECOND,
        GET_PAIN_CAUSED_BY_WOUND,
        GET_SURGERY_AUTO_ATTEMPT_PERCENTAGES,
        SPRINTF_S,
        RANDOM_INT,
        ACTOR_RESTORE_ACTOR_VALUE,
        TILE_ID_00720010,
        GET_DEFAULT_OBJECT,
        REFERENCE_GET_INVENTORY_ITEM,
        ACTOR_QUEUE_EQUIP_OBJECT,
        ITEM_GET_FORM,
        ITEM_CHANGE_DELETE,
        MAGIC_TARGET_IS_SPELL_TARGET,
        SMALLER_OF,
    ];

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn answer_st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    fn near(word: u32, value: f32) -> bool {
        (f32::from_bits(word) - value).abs() < 1e-4
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_3000,
            0x0101_6000,
            0x0101_7000,
            0x0101_8000,
            0x0101_b000,
            0x0101_d000,
            0x0101_e000,
            0x0101_f000,
            0x0102_0000,
            0x0102_2000,
            0x0102_f000,
            0x0103_0000,
            0x0103_5000,
            0x0104_8000,
            0x0104_f000,
            0x0106_e000,
            0x0107_1000,
            0x0107_2000,
            0x0107_3000,
            0x0107_7000,
            0x0118_3000,
            0x0118_7000,
            0x011d_8000,
            0x011d_a000,
            0x011d_e000,
            0x011f_3000,
            0x011f_4000,
            0x011f_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_01012070, 1.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(SIXTY_01012638, 60.0f64);
        e.set_global(THOUSAND_01013974, 1000.0f32);
        e.set_global(FIVE_HUNDRED_01013D84, 500.0f32);
        e.set_global(HALF_01016248, 0.5f32);
        e.set_global(QUARTER_0101622C, 0.25f32);
        e.set_global(HUNDRED_01016410, 100.0f32);
        e.set_global(LARGEST_01016970, f32::MAX);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e.set_global(MOVEMENT_CAP_01017B78, 10.0f32);
        e.set_global(TWENTY_01017868, 20.0f32);
        e.set_global(CONDITION_25, 25.0f32);
        e.set_global(CONDITION_50, 50.0f32);
        e.set_global(CONDITION_75, 75.0f32);
        e.set_global(THIRTY_0101DB88, 30.0f64);
        e.set_global(FIFTY_DOUBLE, 50.0f64);
        e.set_global(TENTH_DOUBLE, 0.1f64);
        e.set_global(TEN_DOUBLE, 10.0f64);
        e.set_global(FIVE_01020998, 5.0f64);
        e.set_global(MINUS_HALF_0102295C, -0.5f32);
        e.set_global(TWENTY_FIVE_DOUBLE, 25.0f64);
        e.set_global(SEVENTY_FIVE_DOUBLE, 75.0f64);
        e.set_global(EIGHTY_FIVE_01072FF0, 85.0f64);
        e.set_global(WOBBLE_RATE_010776C8, 0.00025f64);
        for &address in STUBBED {
            e.register(address, |_, _| Ret::default());
        }
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xffff).into_ret());
        e.register(ADD_USER_TRAIT, |_, a| (a[0] & 0xffff).into_ret());
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            (value as i32 as u32).into_ret()
        });
        e.register(POINT_LENGTH_00589850, |e, a| {
            let (x, y) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            Ret {
                st0: f64::from((x * x + y * y).sqrt()),
                ..Ret::default()
            }
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SMALLER_OF, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: f64::from(if second <= first { second } else { first }),
                ..Ret::default()
            }
        });
        let cell = e.mem.alloc(0x10);
        e.mem.set_f32(cell, 2.0);
        e.register_double(SETTING_VALUE, move |_, _| cell.into_ret());
        let translate = e.mem.alloc(0x10);
        e.register_double(ROOT_TRANSLATE_0043C490, move |_, _| translate.into_ret());
        let slots: Vec<u32> = (0..0x100).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        let scene = object(&mut e);
        e.register_double(SCENE_NODE_00450B80, move |_, _| scene.into_ret());
        let player = object(&mut e);
        e.mem.set_u32(player + 0xa4, VTABLE);
        e.set_global(PLAYER, player);
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, VTABLE);
        object
    }

    /// The open menu with its tiles (`0x4400 + offset`) and a frame geometry.
    fn menu(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, VTABLE);
        e.mem.set_u32(this + 4, MENU_TILE);
        for offset in [0x48u32, 0x4c, 0x50, 0x54, 0x5c, 0x70] {
            e.mem.set_u32(this + offset, 0x4400 + offset);
        }
        e.mem.set_u32(this + 0x44, TOOL_TILE);
        let frame = object(e);
        e.mem.set_u32(this + 0xe0, frame);
        e.set_global(SURGERY_MENU_ME, this);
        Ptr::new(this)
    }

    /// A limb wound for `limb` with one record of `sequence`, first in `xLimbWounds`.
    fn wound(e: &mut Engine, limb: u32, sequence: u32) -> u32 {
        let record = e.mem.alloc(0xc);
        e.mem.set_u32(record, sequence);
        let objects = e.mem.alloc(8);
        e.mem.set_u32(record + 4, objects);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, record);
        let wound = e.mem.alloc(0x10);
        e.mem.set_u32(wound, list);
        e.mem.set_u32(wound + 4, limb);
        e.set_global(LIMB_WOUNDS, wound);
        wound
    }

    #[test]
    fn class_key_and_small_helpers() {
        let mut e = engine();
        let this = menu(&mut e);
        assert_eq!(e.call(0x007e_1970, &args![]).u32(), 0x41e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007e_4260, &args![this, 0x77u32]).bool());
        assert!(e.call(0x007e_4260, &args![this, 0x53u32]).bool());
        assert!(!e.call(0x007e_4260, &args![this, 0x41u32]).bool());
        let me = this.addr();
        assert_eq!(calls(&e, slot(0xc)), vec![vec![me, 6, 0], vec![me, 5, 0]]);
        e.mem.set_u32(me + 0x18, 0x1000);
        assert_eq!(e.call(0x007e_4fc0, &args![this, 3u32]).u32(), 0x103c);
        for (condition, level) in [(100i32, 0u32), (75, 1), (50, 2), (25, 3), (24, 4)] {
            assert_eq!(e.call(0x007e_5540, &args![this, condition]).u32(), level);
        }
        assert_eq!(e.call(0x007e_5e30, &args![this, -1i32]).u32(), 6);
        assert_eq!(e.call(0x007e_5e30, &args![this, 0x1ci32]).u32(), 3);
        assert_eq!(e.call(0x007e_5e30, &args![this, 7i32]).u32(), 0);
        // A unit square and a point inside, then outside.
        let points = e.mem.alloc(0x50);
        for (i, (x, y)) in [(0.0f64, 0.0f64), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
            .iter()
            .enumerate()
        {
            e.mem.set_u64(points + 0x10 * i as u32, x.to_bits());
            e.mem.set_u64(points + 0x10 * i as u32 + 8, y.to_bits());
        }
        let point = e.mem.alloc(0x10);
        e.mem.set_u64(point, 0.5f64.to_bits());
        e.mem.set_u64(point + 8, 0.5f64.to_bits());
        assert_eq!(e.call(0x007e_5420, &args![points, 4u32, point]).u32(), 1);
        e.mem.set_u64(point, 2.0f64.to_bits());
        assert_eq!(e.call(0x007e_5420, &args![points, 4u32, point]).u32(), 0);
        // The cursor move, then none while burning.
        answer(&mut e, SCREEN_004DC1F0, 800);
        answer(&mut e, SCREEN_004DC200, 600);
        e.call(0x007e_41f0, &args![400i32, 300i32]);
        assert_eq!(
            calls(&e, SET_CURSOR_00703080),
            vec![vec![bits(0.5), bits(0.5)]]
        );
        e.mem.set_u8(me + 0x90, 1);
        e.call(0x007e_41f0, &args![400i32, 300i32]);
        assert_eq!(calls(&e, SET_CURSOR_00703080).len(), 1);
    }

    #[test]
    fn limb_wounds_are_found_and_reset() {
        let mut e = engine();
        let this = menu(&mut e);
        let me = this.addr();
        let first = wound(&mut e, 0x19, 0x9000);
        let second = e.mem.alloc(0x10);
        let list = e.mem.alloc(8);
        e.mem.set_u32(second, list);
        e.mem.set_u32(second + 4, 0x1a);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, second);
        e.set_global(LIMB_WOUNDS + 4, node);
        e.mem.set_u32(me + 0xc0, 0x1a);
        assert_eq!(e.call(0x007e_5720, &args![this]).u32(), second);
        e.mem.set_u32(me + 0xc0, 0x1d);
        assert_eq!(e.call(0x007e_5720, &args![this]).u32(), 0);
        answer(&mut e, slot(8), 60);
        e.call_log = Some(vec![]);
        e.call(0x007e_5810, &args![0x1au32]);
        assert_eq!(e.mem.u8(second + 0xc), 1);
        assert_eq!(e.mem.u32(second + 8), 2);
        assert_eq!(e.mem.u8(first + 0xc), 0);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![e.mem.u32(second)]]);
        e.call(0x007e_5890, &args![]);
        assert_eq!(e.mem.u8(first + 0xc), 1);
        // On top the limb is not reset.
        e.mem.set_u8(second + 0xc, 0);
        answer(&mut e, IS_TOP_MENU_ID, 1);
        e.call(0x007e_5810, &args![0x1au32]);
        assert_eq!(e.mem.u8(second + 0xc), 0);
        // The wound's own reset.
        e.call(0x007e_5780, &args![second]);
        assert_eq!(e.mem.u8(second + 0xc), 1);
    }

    #[test]
    fn limb_percent_sequences_and_condition() {
        let mut e = engine();
        let this = menu(&mut e);
        let me = this.addr();
        e.mem.set_u32(me + 0xc0, 0x1b);
        answer(&mut e, ACTOR_VALUE_NAME, 0x7777);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_58e0, &args![this]).f32(), 0.0);
        let value = VALUE_TRAIT_NAME & 0xffff;
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x4470, value, bits(1.0), 1]]
        );
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![0x444c, 0xfc4, 0x7777, 1]]
        );
        wound(&mut e, 0x1b, 0x9000);
        answer_st0(&mut e, SEQUENCE_00493770, 1.0);
        answer_st0(&mut e, SEQUENCE_LENGTH, 4.0);
        assert_eq!(e.call(0x007e_58e0, &args![this]).f32(), 0.75);
        e.set_global(SKIN_ROOT_HOLDER_011DADE8, 0x8800u32);
        e.call(0x007e_59f0, &args![this, 0.5f32]);
        assert_eq!(
            calls(&e, SEQUENCE_SET_PHASE),
            vec![vec![0x9000, bits(2.0), 1]]
        );
        assert_eq!(calls(&e, UPDATE_OBJECT)[0][0], 0x8800);
        // Level 0 shows 100; level 2 maps the healed quarter from 100 down to 50.
        assert_eq!(e.call(0x007e_5590, &args![this]).i32(), 100);
        e.mem.set_u32(me + 0xc4, 2);
        answer_st0(&mut e, INTERPOLATE_004B3AB0, 62.7);
        assert_eq!(e.call(0x007e_5590, &args![this]).i32(), 62);
        assert_eq!(
            calls(&e, INTERPOLATE_004B3AB0),
            vec![vec![bits(100.0), bits(50.0), 0, bits(1.0), bits(0.25)]]
        );
    }

    #[test]
    fn skin_shading_render_placement_and_switch() {
        let mut e = engine();
        let this = menu(&mut e);
        let me = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x007e_3750, &args![this]);
        e.call(0x007e_47a0, &args![]);
        e.call(0x007e_4fe0, &args![1.0f32, 2.0f32, 3.0f32]);
        assert!(calls(&e, INTERPOLATE_004B3AB0).is_empty());
        assert!(calls(&e, RENDERER).is_empty());
        assert!(calls(&e, SET_TRANSLATE).is_empty());
        let skin = object(&mut e);
        e.set_global(SKIN_HOLDER_011DAE5C, skin);
        let root = object(&mut e);
        e.set_global(SKIN_ROOT_HOLDER_011DADE8, root);
        answer(&mut e, slot(0x9c), skin);
        answer(&mut e, slot(0x100), skin);
        e.mem.set_f32(me + 0xac, 50.0);
        answer_st0(&mut e, INTERPOLATE_004B3AB0, 0.5);
        answer(&mut e, MILLISECONDS, 2000);
        e.call(0x007e_3750, &args![this]);
        assert_eq!(calls(&e, slot(0x114)), vec![vec![skin, 0xd, bits(0.5)]]);
        assert_eq!(calls(&e, UPDATE_DATA_INIT)[0][1..], [bits(2.0), 1, 0]);
        e.call(0x007e_47a0, &args![]);
        assert_eq!(calls(&e, ACCUMULATE_SCENE).len(), 2);
        assert_eq!(calls(&e, SET_APP_CULLED), vec![vec![skin, 0]]);
        e.call(0x007e_4fe0, &args![1.0f32, 2.0f32, 3.0f32]);
        assert_eq!(
            calls(&e, SET_TRANSLATE),
            vec![vec![skin, bits(-2.0), bits(0.0), bits(1.0)]]
        );
        assert_eq!(calls(&e, SET_SCALE_00440490), vec![vec![skin, bits(6.0)]]);
        e.call(0x007e_5170, &args![]);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN)[1..],
            [
                vec![CAMERA_HOLDER_011DADB4, 0],
                vec![SKIN_ROOT_HOLDER_011DADE8, 0],
                vec![SKIN_HOLDER_011DAE5C, 0]
            ]
        );
        let player = e.global::<u32>(PLAYER);
        assert_eq!(calls(&e, CLONE_INVENTORY_3D), vec![vec![player, 0]]);
        // Switching from limb 0x19 (index 5) to 0x1a (index 0).
        e.set_global(SKIN_ROOT_HOLDER_011DADE8, root);
        e.mem.set_u32(me + 0xc8 + 5 * 4, 0x111);
        e.mem.set_u32(me + 0xc8, 0x222);
        answer(&mut e, slot(0x9c), 0x333);
        e.call(0x007e_5ee0, &args![this, 0x19i32, 0x1ai32]);
        assert_eq!(calls(&e, slot(0xe8)).last().unwrap(), &vec![root, 0x111]);
        assert_eq!(calls(&e, slot(0xdc)), vec![vec![root, 0x222, 1]]);
        assert_eq!(e.mem.u32(me + 0xe8), 0x333);
    }

    #[test]
    fn animations_start_and_advance() {
        let mut e = engine();
        let this = menu(&mut e);
        let me = this.addr();
        let frame = e.mem.u32(me + 0xe0);
        answer(&mut e, GET_CONTROLLER, 0x4440);
        answer(&mut e, SEQUENCE_LOOKUP, 0x5550);
        e.call_log = Some(vec![]);
        e.call(0x007e_5c80, &args![this, 2u32]);
        assert_eq!(e.mem.u32(me + 0x28), 2);
        assert_eq!(calls(&e, FIXED_STRING_CONSTRUCT)[0][1], RIGHT_NAME);
        assert_eq!(
            calls(&e, PLAY_SEQUENCE),
            vec![vec![0x4440, 0x5550, 0, 1, bits(1.0), 0, 0]]
        );
        assert_eq!(calls(&e, START_ANIMATIONS), vec![vec![frame]]);
        let manager = object(&mut e);
        answer(&mut e, CONTROLLER_MANAGER_0043B230, manager);
        answer_st0(&mut e, FRAME_TIME_0084D030, 0.5);
        answer(&mut e, TYPE_TEST_00653270, 1);
        e.mem.set_f32(me + 0xec, 1.0);
        e.call(0x007e_5a90, &args![this]);
        assert_eq!(e.mem.f32(me + 0xec), 1.5);
        assert_eq!(calls(&e, SEQUENCE_UPDATE), vec![vec![0x5550, bits(1.5), 1]]);
        assert_eq!(calls(&e, slot(0x94))[0][0], manager);
        assert_eq!(calls(&e, UPDATE_OBJECT).last().unwrap()[0], frame);
    }

    #[test]
    fn window_test_and_tool_placement() {
        let mut e = engine();
        let this = menu(&mut e);
        answer(&mut e, GAMEPAD_IN_USE_004B71D0, 1);
        assert!(e.call(0x007e_4540, &args![this]).bool());
        answer(&mut e, GAMEPAD_IN_USE_004B71D0, 0);
        answer(&mut e, CURSOR_X_00747D00, 810);
        answer(&mut e, CURSOR_Y_0073B020, 475);
        assert!(e.call(0x007e_4540, &args![this]).bool());
        answer(&mut e, CURSOR_X_00747D00, 1200);
        assert!(!e.call(0x007e_4540, &args![this]).bool());
        e.call_log = Some(vec![]);
        e.call(0x007e_42c0, &args![this, 900i32, 475i32]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![
                vec![TOOL_TILE, 0xfa1, bits(900.0), 1],
                vec![TOOL_TILE, 0xfa2, bits(475.0), 1]
            ]
        );
        assert!(calls(&e, POINT_NORMALIZE_004E7560).is_empty());
        e.call(0x007e_42c0, &args![this, 1200i32, 475i32]);
        assert_eq!(calls(&e, POINT_NORMALIZE_004E7560).len(), 1);
        assert_eq!(calls(&e, POINT_SCALE_006AD180)[0][1], bits(325.0));
    }

    #[test]
    fn sorting_by_cursor_and_decals() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set_global(SURGERY_MENU_ME, 0u32);
        assert_eq!(e.call(0x007e_5250, &args![0u32, 0u32]).i32(), 0);
        e.set_global(SURGERY_MENU_ME, this.addr());
        let record = |e: &mut Engine, item: u32| {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            let record = e.mem.alloc(8);
            e.mem.set_u32(record + 4, node);
            record
        };
        let (first, second) = (record(&mut e, 0x71), record(&mut e, 0x72));
        assert_eq!(e.call(0x007e_5250, &args![first, second]).i32(), -1);
        e.set_global(CAMERA_HOLDER_011DADB4, 0x777u32);
        let lengths = std::rc::Rc::new(std::cell::Cell::new(0usize));
        let seen = lengths.clone();
        e.register_double(POINT_LENGTH_00589850, move |_, _| {
            let i = seen.get();
            seen.set(i + 1);
            Ret {
                st0: [5.0, 5.0, 3.0][i % 3],
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x007e_5250, &args![first, second]).i32(), 1);
        e.register_double(POINT_LENGTH_00589850, |_, _| Ret {
            st0: 5.0,
            ..Ret::default()
        });
        assert_eq!(e.call(0x007e_5250, &args![first, second]).i32(), -1);
        answer(&mut e, SCREEN_004DC1F0, 1600);
        answer(&mut e, SCREEN_004DC200, 960);
        answer(&mut e, DECAL_MANAGER, 0xdec0);
        let point = e.mem.alloc(0x10);
        answer(&mut e, POINT_FROM_FLOATS, point);
        e.call_log = Some(vec![]);
        e.call(0x007e_5ff0, &args![this, 0.5f32, 0.25f32, 1u32]);
        assert_eq!(calls(&e, CAMERA_RAY)[0][..3], [0x777, 800, 240]);
        assert_eq!(calls(&e, FORM_BY_EDITOR_ID), vec![vec![GOOD_DECAL_NAME]]);
        let added = calls(&e, ADD_DECAL);
        assert_eq!(added.len(), 1);
        assert_eq!((added[0][0], added[0][2], added[0][3]), (0xdec0, 1, 1));
    }

    #[test]
    fn sequences_are_prepared_for_a_new_wound() {
        let mut e = engine();
        let this = menu(&mut e);
        let me = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x007e_4920, &args![0u32]);
        assert!(calls(&e, GET_CONTROLLER).is_empty());
        let controller = object(&mut e);
        answer(&mut e, GET_CONTROLLER, controller);
        answer_st0(&mut e, slot(0xc), 60.0);
        e.mem.set_u32(me + 0xc0, 0x1c);
        e.call(0x007e_4920, &args![0x9990u32]);
        assert_eq!(e.mem.u32(me + 0xe4), controller);
        assert_eq!(e.mem.u32(me + 0xc4), 2);
        assert_eq!(calls(&e, ALLOCATE)[0], vec![0x10]);
        assert_eq!(calls(&e, LIST_ADD)[0][0], LIMB_WOUNDS);
        assert_eq!(calls(&e, UPDATE_OBJECT).last().unwrap()[0], 0x9990);
    }

    #[test]
    fn flesh_burns_hurts_and_shows_luck() {
        let mut e = engine();
        let this = menu(&mut e);
        let me = this.addr();
        assert_eq!(e.call(0x007e_2de0, &args![this]).f32(), 0.0);
        e.set_global(CAMERA_HOLDER_011DADB4, 0x777u32);
        e.mem.set_u32(me + 0xc0, 0x19);
        wound(&mut e, 0x19, 0x9000);
        answer(&mut e, LIST_COUNT_005AE380, 1);
        answer(&mut e, MILLISECONDS, 5000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_2de0, &args![this]).f32(), 100.0);
        assert_eq!(e.mem.u32(me + 0x8c), 5000);
        assert_eq!(calls(&e, WOUND_FORMULA_00648D70).len(), 1);
        // Burning: the pain rises 30 a second past 60, which may sound.
        e.mem.set_f32(me + 0xac, 50.0);
        e.mem.set_u32(me + 0xb0, 4000);
        e.mem.set_u32(me + 0xc0, 0x19);
        answer(&mut e, RANDOM_INT, 1);
        answer(&mut e, slot(8), 40);
        answer_st0(&mut e, INTERPOLATE_004B3AB0, 500.0);
        e.call_log = Some(vec![]);
        e.call(0x007e_28b0, &args![this]);
        assert_eq!(e.mem.f32(me + 0xac), 80.0);
        assert_eq!(calls(&e, GET_SOUND_HANDLE_BY_NAME)[0][2], PAIN_SOUND_ONE);
        assert_eq!(e.mem.u32(me + 0xb0), 5000);
        let player = e.global::<u32>(PLAYER);
        assert_eq!(
            calls(&e, ACTOR_RESTORE_ACTOR_VALUE),
            vec![vec![player, 0x19, bits(60.0)]]
        );
        assert_eq!(
            calls(&e, slot(0x3b0)),
            vec![vec![player, 0x10, 0xffff_ffff, 0]]
        );
        assert_eq!(e.mem.u32(me + 0xb4), 5000);
        assert_eq!(calls(&e, SPRINTF_S)[0][1..], [0x100, PERCENT_FORMAT, 40]);
        assert_eq!(calls(&e, TILE_SET_STRING).last().unwrap()[0], 0x4450);
        // Reaching 100 sounds the third pain; the hurt waits for its threshold.
        e.mem.set_f32(me + 0xac, 95.0);
        e.mem.set_u32(me + 0xb0, 4000);
        answer_st0(&mut e, INTERPOLATE_004B3AB0, 9999.0);
        e.call_log = Some(vec![]);
        e.call(0x007e_28b0, &args![this]);
        assert_eq!(e.mem.f32(me + 0xac), 100.0);
        assert_eq!(calls(&e, GET_SOUND_HANDLE_BY_NAME)[0][2], PAIN_SOUND_THREE);
        assert!(calls(&e, slot(0x3b0)).is_empty());
    }

    #[test]
    fn tool_location_follows_the_cursor_or_the_burn() {
        let mut e = engine();
        let this = menu(&mut e);
        let me = this.addr();
        answer(&mut e, CURSOR_X_00747D00, 900);
        answer(&mut e, CURSOR_Y_0073B020, 475);
        e.call_log = Some(vec![]);
        e.call(0x007e_3890, &args![this, 0u32]);
        let names: Vec<u32> = calls(&e, ADD_USER_TRAIT).iter().map(|c| c[0]).collect();
        assert_eq!(names, TOOL_TRAITS.iter().map(|t| t.0).collect::<Vec<_>>());
        assert_eq!(e.global::<u32>(TOOL_TRAIT_GUARD), 0xff);
        assert_eq!(e.mem.u8(me + 0x90), 0);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x445c, 0xfa3, 0]]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT)[0],
            vec![TOOL_TILE, 0xfa1, bits(900.0), 1]
        );
        e.call(0x007e_3890, &args![this, 0u32]);
        assert_eq!(calls(&e, ADD_USER_TRAIT).len(), 8);
        // The gamepad's stick from the tool tile's place: 10 + 2 * 3, 10 - 2 * 2.
        answer(&mut e, GAMEPAD_IN_USE_004B71D0, 1);
        e.register(THUMBSTICK_00A23390, |_, a| {
            (if a[2] == 7 { 3u32 } else { 2 }).into_ret()
        });
        answer_st0(&mut e, TILE_GET_FLOAT, 10.0);
        e.call_log = Some(vec![]);
        e.call(0x007e_3890, &args![this, 0u32]);
        let placed = calls(&e, TILE_SET_FLOAT);
        assert_eq!(placed[0], vec![TOOL_TILE, 0xfa1, bits(16.0), 1]);
        assert_eq!(placed[1], vec![TOOL_TILE, 0xfa2, bits(6.0), 1]);
        // Burning with the mouse: every mapping answers 2, the axes move (3, 4).
        answer(&mut e, GAMEPAD_IN_USE_004B71D0, 0);
        answer_st0(&mut e, INTERPOLATE_004B3AB0, 2.0);
        e.register(MOUSE_AXIS_00A239E0, |_, a| {
            (if a[1] == 1 { 3u32 } else { 4 }).into_ret()
        });
        e.mem.set_f32(me + 0x74, 100.0);
        e.mem.set_f32(me + 0x78, 50.0);
        e.call_log = Some(vec![]);
        e.call(0x007e_3890, &args![this, 1u32]);
        assert_eq!(e.mem.u8(me + 0x90), 1);
        assert_eq!(e.mem.f32(me + 0x7c), 4.0);
        assert_eq!(e.mem.f32(me + 0x80), 4.0);
        assert_eq!(e.mem.f32(me + 0x84), 2.0);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x445c, 0xfa3, 1]]);
        assert_eq!(calls(&e, POINT2_SCALED_00532FD0)[0][1], bits(2.0));
        assert_eq!(calls(&e, WOUND_FORMULA_00648D70)[0][5], 0);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT)[0],
            vec![TOOL_TILE, 0xfa1, bits(100.0), 1]
        );
        assert_eq!(calls(&e, SET_CURSOR_POSITION_007151B0).len(), 1);
    }

    #[test]
    fn idle_eases_the_pain_and_shows_health() {
        let mut e = engine();
        let this = menu(&mut e);
        let me = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x007e_23c0, &args![this]);
        let top = IS_TOP_MENU_NAME & 0xffff;
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![MENU_TILE, top, 0]]);
        assert!(calls(&e, TILE_SET_FLOAT).is_empty());
        answer(&mut e, IS_TOP_MENU_ID, 1);
        answer_st0(&mut e, PAIN_CHANGE_PER_SECOND, 2.0);
        answer_st0(&mut e, GET_PAIN_CAUSED_BY_WOUND, 0.3);
        answer(&mut e, MILLISECONDS, 3000);
        answer(&mut e, MANAGER_MENU_TILE, 0x6600);
        answer(&mut e, slot(8), 40);
        answer(&mut e, slot(0x1c), 250);
        e.mem.set_f32(me + 0xac, 50.0);
        e.mem.set_u32(me + 0xb0, 1000);
        e.call_log = Some(vec![]);
        e.call(0x007e_23c0, &args![this]);
        assert_eq!(calls(&e, TILE_SET_INT)[0], vec![MENU_TILE, top, 1]);
        assert_eq!(calls(&e, TILE_SET_INT)[1], vec![0x6600, 0xfa9, 0xff]);
        assert_eq!(e.mem.f32(me + 0xac), 46.0);
        assert_eq!((e.mem.u32(me + 0xb0), e.mem.u32(me + 0x8c)), (3000, 3000));
        let shown = calls(&e, TILE_SET_FLOAT);
        let pain = shown.iter().find(|c| c[0] == 0x4454).unwrap();
        assert!(near(pain[2], 0.46) && pain[1] == 0x1004 && pain[3] == 1);
        assert_eq!(
            calls(&e, SPRINTF_S)[0][1..],
            [0x100, HEALTH_FORMAT, 40, 250]
        );
        assert_eq!(
            calls(&e, TILE_SET_STRING)[0][..2],
            [0x4448, VALUE_TRAIT_NAME & 0xffff]
        );
        // Below the wound's floor the pain stays at it.
        answer_st0(&mut e, GET_PAIN_CAUSED_BY_WOUND, 0.5);
        e.mem.set_u32(me + 0xb0, 1000);
        e.call(0x007e_23c0, &args![this]);
        assert_eq!(e.mem.f32(me + 0xac), 50.0);
        // The gamepad's button held takes the tool tile's place, then idles.
        answer(&mut e, GAMEPAD_IN_USE_004B71D0, 1);
        answer(&mut e, CHECK_MENU_BUTTON, 2);
        answer_st0(&mut e, TILE_GET_FLOAT, 12.0);
        e.call(0x007e_23c0, &args![this]);
        assert_eq!(e.mem.f32(me + 0x74), 12.0);
        assert_eq!(e.mem.f32(me + 0x78), 12.0);
    }

    #[test]
    fn clicks_choose_limbs_use_items_and_close() {
        let mut e = engine();
        let this = menu(&mut e);
        let me = this.addr();
        let player = e.global::<u32>(PLAYER);
        let root = object(&mut e);
        e.set_global(SKIN_ROOT_HOLDER_011DADE8, root);
        answer_st0(&mut e, GET_PAIN_CAUSED_BY_WOUND, 0.4);
        answer(&mut e, ACTOR_VALUE_NAME, 0x7777);
        // 15 from none goes to 0x1e; 14 from 0x1e wraps to 0x19.
        answer(&mut e, TILE_ID_00720010, 15);
        e.mem.set_i32(me + 0xc0, -1);
        e.call_log = Some(vec![]);
        e.call(0x007e_1b10, &args![this, 0x123u32, 0u32]);
        assert_eq!(e.global::<u32>(CLICK_TRAIT_GUARD), 3);
        assert_eq!(
            e.global::<u32>(STIMPAK_COUNT_TRAIT),
            STIMPAK_COUNT_NAME & 0xffff
        );
        assert_eq!(calls(&e, TILE_ID_00720010), vec![vec![me, 0x123]]);
        assert_eq!(e.mem.u32(me + 0xc0), 0x1e);
        assert!(near(e.mem.u32(me + 0xac), 0.4));
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![0x444c, 0xfc4, 0x7777, 1]]
        );
        assert_eq!(calls(&e, slot(0xdc)).len(), 1);
        answer(&mut e, TILE_ID_00720010, 14);
        e.call(0x007e_1b10, &args![this, 0x123u32, 0u32]);
        assert_eq!(e.mem.u32(me + 0xc0), 0x19);
        // 4: a stimpak of three used, the count less one.
        let stimpaks = e.mem.alloc(0x10);
        e.mem.set_u32(stimpaks + 4, 3);
        e.mem.set_u32(me + 0xbc, stimpaks);
        answer(&mut e, ITEM_GET_FORM, 0xf0f0);
        answer_st0(&mut e, TILE_GET_FLOAT, 5.0);
        answer(&mut e, TILE_ID_00720010, 4);
        e.call_log = Some(vec![]);
        e.call(0x007e_1b10, &args![this, 0x123u32, 0u32]);
        let stimpak = STIMPAK_COUNT_NAME & 0xffff;
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![MENU_TILE, stimpak, bits(4.0), 1]]
        );
        assert_eq!(
            calls(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![player, 0xf0f0, 3, 0, 0, 0, 1]]
        );
        assert_eq!(e.mem.u32(me + 0x28), 3);
        // 5: morphine used, its entry deleted, the pain and the wound less 25.
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 4, 1);
        answer(&mut e, REFERENCE_GET_INVENTORY_ITEM, entry);
        answer(&mut e, GET_DEFAULT_OBJECT, 0xd00d);
        answer_st0(&mut e, TILE_GET_FLOAT, 30.0);
        e.set_global(TRAIT_011DAD9C, 0x1234u32);
        e.mem.set_f32(me + 0xac, 50.0);
        answer(&mut e, TILE_ID_00720010, 5);
        e.call_log = Some(vec![]);
        e.call(0x007e_1b10, &args![this, 0x123u32, 0u32]);
        assert_eq!(
            calls(&e, ACTOR_QUEUE_EQUIP_OBJECT),
            vec![vec![player, 0xd00d, 1, 0, 0, 0, 1]]
        );
        assert_eq!(calls(&e, ITEM_CHANGE_DELETE), vec![vec![entry, 1]]);
        assert_eq!(e.mem.f32(me + 0xac), 25.0);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT)[1],
            vec![MENU_TILE, 0x1234, bits(5.0), 1]
        );
        assert_eq!(e.mem.u32(me + 0x28), 2);
        // 2: the automatic attempt (success 10, damage 7) on a wound three quarters left.
        let wound = wound(&mut e, 0x19, 0x9000);
        answer_st0(&mut e, SEQUENCE_00493770, 1.0);
        answer_st0(&mut e, SEQUENCE_LENGTH, 4.0);
        answer(&mut e, slot(8), 40);
        answer(&mut e, MAGIC_TARGET_IS_SPELL_TARGET, 1);
        e.register(GET_SURGERY_AUTO_ATTEMPT_PERCENTAGES, |e, a| {
            e.mem.set_f32(a[0], 10.0);
            e.mem.set_f32(a[1], 7.0);
            Ret::default()
        });
        answer(&mut e, TILE_ID_00720010, 2);
        e.call_log = Some(vec![]);
        e.call(0x007e_1b10, &args![this, 0x123u32, 0u32]);
        assert_eq!(
            calls(&e, GET_SURGERY_AUTO_ATTEMPT_PERCENTAGES)[0][2..4],
            [40, 1]
        );
        let phase = calls(&e, SEQUENCE_SET_PHASE);
        assert!(near(phase[0][1], 0.6));
        assert_eq!(
            calls(&e, slot(0x3ac)),
            vec![vec![player, 0x10, bits(-7.0), 0]]
        );
        assert_eq!(e.mem.u8(wound + 0xc), 1);
        // 1 closes the menu: its sound, the tile, the fade and the stopped handles.
        answer(&mut e, GET_MENU_BY_CLASS, 0x6000);
        answer(&mut e, TILE_ID_00720010, 1);
        e.set_global(LIMB_WOUNDS, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x007e_1b10, &args![this, 0x123u32, 0u32]);
        assert_eq!(calls(&e, GET_MENU_BY_CLASS), vec![vec![0x41e]]);
        assert_eq!(calls(&e, GET_SOUND_HANDLE_BY_NAME)[0][2], EXIT_SOUND);
        assert_eq!(calls(&e, TILE_SET_INT)[0], vec![0x6000, 0x1772, 1]);
        assert_eq!(
            calls(&e, SOUND_STOP),
            vec![vec![me + 0x94], vec![me + 0xa0]]
        );
        assert_eq!(calls(&e, SOUND_RELEASE).len(), 2);
    }
}
