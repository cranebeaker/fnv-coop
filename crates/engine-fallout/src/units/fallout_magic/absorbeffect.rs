//! `fallout/magic/absorbeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00804f80` (named `ActiveEffect::GetSaveSize` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const GET_SAVE_SIZE_TARGET_00804F80: u32 = 0x0080_4f80;

// Translated from 00802d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::GetSaveSize` (engine map vtable name; vtable words 010792bc and
/// others): forwards `this` and the `Actor` pointer to the base body `00804f80`
/// (`ActiveEffect::GetSaveSize`; Xbox PDB `?GetSaveSize@ActiveEffect@@UAAGPAVActor@@@Z`
/// returns `G`, an unsigned short). The callee ends with `MOV AX,[EBP-4]`; the wrapper returns
/// that 16-bit size.
pub fn value_modifier_effect_get_save_size(e: &mut Engine, this: Ptr, actor: Ptr) -> u16 {
    e.call(GET_SAVE_SIZE_TARGET_00804F80, &args![this, actor])
        .u16()
}

/// `008050d0` (named `ActiveEffect::SaveGame` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const SAVE_GAME_TARGET_008050D0: u32 = 0x0080_50d0;

// Translated from 00802db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::SaveGame` (engine map vtable name; vtable words 010792c4 and others):
/// forwards `this` and one pointer to the base body `008050d0` (`ActiveEffect::SaveGame`; Xbox
/// PDB declares overloads taking `Actor *` and `BGSSaveFormBuffer *`, both void). Returns
/// nothing.
pub fn value_modifier_effect_save_game(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(SAVE_GAME_TARGET_008050D0, &args![this, argument]);
}

/// `00805330` (named `ActiveEffect::LoadGame` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_GAME_TARGET_00805330: u32 = 0x0080_5330;

// Translated from 00802dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect::LoadGame` (engine map vtable name; vtable words 010792cc and others):
/// forwards `this` and one pointer to the base body `00805330` (`ActiveEffect::LoadGame`; Xbox
/// PDB declares overloads taking `Actor *` and `BGSLoadFormBuffer *`, both void). Returns
/// nothing.
pub fn value_modifier_effect_load_game(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(LOAD_GAME_TARGET_00805330, &args![this, argument]);
}

// Translated from 00803a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`): stores `value` in the `float` at `this + 0x13c` (`FLD`/`FSTP`: a
/// signalling NaN arrives quiet). The class is unconfirmed.
pub fn fn_00803a90(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x13c, x87_quiet_bits(value.to_bits()));
}

/// The `AbsorbEffect` vtable (`010792ac`, its first slot the scalar deleting destructor).
const ABSORB_EFFECT_VTABLE: u32 = 0x0107_92ac;
/// `0082ba60` (`thiscall`, `RET 0xC`): the `ValueModifierEffect` base constructor;
/// `0080d930` (`thiscall`): the base destructor.
const BASE_CONSTRUCT: u32 = 0x0082_ba60;
const BASE_DESTRUCT: u32 = 0x0080_d930;
/// The `NiPointer` members (+0x4C the node, +0x50 and +0x54 the two spine nodes, +0x58 the
/// bolt shader): `00633c90` (`RET 4`) constructs one, `0066b0d0` (`RET 4`) assigns it,
/// `0045cec0` destructs it and `00559450` reads it.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
const POINTER_GET: u32 = 0x0055_9450;
/// `BoltShader::Stop` (`00bc1470`) and `BoltShader::Start` (`00bc18d0`), `cdecl` (node,
/// shader / node, 1), engine map names.
const BOLT_STOP: u32 = 0x00bc_1470;
const BOLT_START: u32 = 0x00bc_18d0;
/// `009611e0` (`thiscall` on a node): the object whose slot 0xE8 takes the node away.
const NODE_OWNER: u32 = 0x0096_11e0;
/// `operator new` (`00401000`) and `operator delete` (`00401030`), `cdecl`; the Ni allocator
/// `00aa13e0` (`cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const NI_NEW: u32 = 0x00aa_13e0;
/// The `ValueModifierEffect` methods the overrides call first: `Start` (`0082c000`),
/// `Update` (`0082c660`, `RET 4`), `Finish` (`0082c810`), `FinishInitLoadGame` (`0082cd10`,
/// `RET 4`), `ShowAttributeReductionMessage` (`0082be10`) and
/// `CheckAdjustNoNegativeMagnitude` (`0082ce70`, `RET 4`), engine map names.
const BASE_START: u32 = 0x0082_c000;
const BASE_UPDATE: u32 = 0x0082_c660;
const BASE_FINISH: u32 = 0x0082_c810;
const BASE_FINISH_INIT_LOAD_GAME: u32 = 0x0082_cd10;
const SHOW_ATTRIBUTE_REDUCTION_MESSAGE: u32 = 0x0082_be10;
const CHECK_ADJUST_NO_NEGATIVE_MAGNITUDE: u32 = 0x0082_ce70;
/// `00825c00` (the effect setting of the effect item at +0xC) and `00403e60` (`RET 4`: a
/// flag), the "detrimental" test the overrides make with 2.
const EFFECT_SETTING_OF: u32 = 0x0082_5c00;
const SETTING_HAS_FLAG: u32 = 0x0040_3e60;
/// The `double` 0.0 (`01012060`) the magnitude (+0x20) is compared with.
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// The caster (`0045cd60`, then `MagicCaster::GetCasterAsActor` `00815410`) and the target
/// (`0059bb30`, then `MagicTarget::GetTargetAsActor` `00822b40`).
const CASTER_OF: u32 = 0x0045_cd60;
const CASTER_AS_ACTOR: u32 = 0x0081_5410;
const TARGET_OF: u32 = 0x0059_bb30;
const TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `00495460` (`thiscall`): the float at +0x1C in `ST0` (the effect's magnitude).
const MAGNITUDE: u32 = 0x0049_5460;
/// `ActiveEffect::Dispel` (`00804210`, `RET 4`) and `00705cf0` (`thiscall`: whether the
/// effect is ending), engine map names.
const DISPEL: u32 = 0x0080_4210;
const IS_ENDING: u32 = 0x0070_5cf0;
/// `TESObjectREFR::GetDistanceFromReference` (`005723b0`, `RET 0xC`), the setting value
/// address `00403e20` and `CombatFormulas::CalcWeaponReach` (`006463e0`, `cdecl`).
const DISTANCE_FROM_REFERENCE: u32 = 0x0057_23b0;
const SETTING_FLOAT: u32 = 0x0040_3e20;
const SETTING_WORD: u32 = 0x0043_d4d0;
const WEAPON_REACH: u32 = 0x0064_63e0;
/// `MiddleHighProcess::GetSavedAcquireObject` (`008d8520`).
const SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// The settings `Update` and `Update3D` read (`011d1aa8` the reach, `011d1c10` the bolt
/// length) and `Start3D` gives the bolt.
const SETTING_REACH: u32 = 0x011d_1aa8;
const SETTING_BOLT_LENGTH: u32 = 0x011d_1c10;
/// `00bc2740` (`thiscall` on the bolt, `RET 0x10`), `00bc2360` (`thiscall` on the bolt).
const BOLT_UPDATE: u32 = 0x00bc_2740;
const BOLT_READY: u32 = 0x00bc_2360;
/// `Update3D`'s callees: the menu-mode byte `011db9f4`, the default point `011f426c`,
/// `0045bb80` (a node's world position), `00440460` (`RET 4`), `00439ef0` (`RET 8`),
/// `006815c0` (answers `this`), `00457990` (a point's length, `ST0`), `atan2` (`004b5550`,
/// `cdecl`, two floats), `NiMatrix3::FromEulerAnglesZYX` (`00a59780`, `RET 0xC`) and
/// `0043fa80` (`RET 4`).
const MENU_MODE_BYTE: u32 = 0x011d_b9f4;
const DEFAULT_POINT: u32 = 0x011f_426c;
const WORLD_POSITION: u32 = 0x0045_bb80;
const SET_POSITION: u32 = 0x0044_0460;
const POINT_MINUS: u32 = 0x0043_9ef0;
const SELF: u32 = 0x0068_15c0;
const POINT_LENGTH: u32 = 0x0045_7990;
const ATAN2: u32 = 0x004b_5550;
const FROM_EULER_ANGLES_ZYX: u32 = 0x00a5_9780;
const SET_ROTATE: u32 = 0x0043_fa80;
/// `Start3D`'s callees: `00a5ecb0` (`RET 4`, constructs a node), `00448a80` (the
/// `BSXFlags` name, the word at `01202ddc`), `NiObjectNET::GetExtraData` (`00a5bdd0`, `RET
/// 4`), `BSXFlags::BSXFlags` (`00c42f70`), `NiObjectNET::AddExtraData` (`00a5bc40`, `RET 8`),
/// `00519230` (`RET 8`), `008d6f30` and `005497a0` (the target's cell and its node),
/// `00496940`, `00537bd0`, `00438170` (`RET 4`, a fixed string) and its destructor
/// `004381b0`, `00822510` (`RET 4`), and the bolt setters `00821620`, `00507780`, `005077a0`,
/// `005f9b10` (`RET 4`), `00414430` (`RET 0x10`, a colour).
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const BSX_FLAGS_NAME: u32 = 0x0044_8a80;
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const BSX_FLAGS_CONSTRUCT: u32 = 0x00c4_2f70;
const ADD_EXTRA_DATA: u32 = 0x00a5_bc40;
const FLAGS_SET_519230: u32 = 0x0051_9230;
const PARENT_CELL: u32 = 0x008d_6f30;
const CELL_NODE: u32 = 0x0054_97a0;
const ACTOR_3D_ROOT: u32 = 0x0049_6940;
const ROOT_NODE: u32 = 0x0053_7bd0;
const FIXED_STRING: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const NODE_TEST: u32 = 0x0082_2510;
const BOLT_SET_821620: u32 = 0x0082_1620;
const BOLT_SET_507780: u32 = 0x0050_7780;
const BOLT_SET_5077A0: u32 = 0x0050_77a0;
const BOLT_SET_5F9B10: u32 = 0x005f_9b10;
const MAKE_COLOR: u32 = 0x0041_4430;
/// "Bip01 Spine1" (`01020584`).
const TEXT_SPINE: u32 = 0x0102_0584;

/// The node member (`this + offset`) read through `00559450`.
fn member(e: &mut Engine, this: Ptr, offset: u32) -> u32 {
    e.call(POINTER_GET, &args![this.addr().wrapping_add(offset)])
        .u32()
}

/// Whether the effect's setting is detrimental (`00403e60` with 2 on `00825c00` of the item at
/// +0xC).
fn is_detrimental(e: &mut Engine, this: Ptr) -> bool {
    let item = e.mem.u32(this.addr().wrapping_add(0xc));
    let setting = e.call(EFFECT_SETTING_OF, &args![item]).u32();
    e.call(SETTING_HAS_FLAG, &args![setting, 2u32]).bool()
}

/// The caster as an actor (0 without a caster).
fn caster_actor(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(CASTER_OF, &args![this]).u32() == 0 {
        return 0;
    }
    let caster = e.call(CASTER_OF, &args![this]).u32();
    e.call(CASTER_AS_ACTOR, &args![caster]).u32()
}

/// Stops the bolt (`BoltShader::Stop` with the node and the shader) and, with a node, has its
/// owner (`009611e0`, slot 0xE8) take it and clears it: the common tail of the destructor and
/// `Finish`.
fn stop_bolt(e: &mut Engine, this: Ptr) {
    let shader = member(e, this, 0x58);
    let node = member(e, this, 0x4c);
    e.call(BOLT_STOP, &args![node, shader]);
}

fn release_node(e: &mut Engine, this: Ptr) {
    if member(e, this, 0x4c) == 0 {
        return;
    }
    let node = member(e, this, 0x4c);
    let owner = e.call(NODE_OWNER, &args![node]).u32();
    if owner != 0 {
        let node = member(e, this, 0x4c);
        e.vcall(owner, 0xe8, &args![node]);
    }
    e.call(POINTER_ASSIGN, &args![this.addr().wrapping_add(0x4c), 0u32]);
}

// Translated from 00802cb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00802cb0 - the FS:[0] exception frame (unwinding the members) is not translated.
/// The `AbsorbEffect` constructor (unnamed in the map; `RET 0xC`: three words for the base):
/// the `ValueModifierEffect` base (`0082ba60`), the vtable `010792ac`, the four pointer
/// members constructed null (+0x4C, +0x50, +0x54, +0x58) and assigned null (+0x58, +0x4C,
/// +0x54, +0x50). Returns `this`. The exception frame is not translated.
pub fn fn_00802cb0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![this, first, second, third]);
    e.mem.set_u32(base, ABSORB_EFFECT_VTABLE);
    for offset in [0x4cu32, 0x50, 0x54, 0x58] {
        e.call(POINTER_CONSTRUCT, &args![base.wrapping_add(offset), 0u32]);
    }
    for offset in [0x58u32, 0x4c, 0x54, 0x50] {
        e.call(POINTER_ASSIGN, &args![base.wrapping_add(offset), 0u32]);
    }
    this
}

// Translated from 00802df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AbsorbEffect::scalar deleting destructor` (engine map, `RET 4`: flags): the destructor,
/// then `operator delete` with flag 1. Returns `this`.
pub fn absorb_effect_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    absorb_effect_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00802e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00802e20 - the FS:[0] exception frame (unwinding the members) is not translated.
/// `AbsorbEffect::~AbsorbEffect` (engine map): the vtable, the bolt stopped and its shader
/// cleared, the node given back to its owner and cleared, the four pointers destructed
/// (+0x58, +0x54, +0x50, +0x4C) and the base destructor (`0080d930`). The exception frame is
/// not translated.
pub fn absorb_effect_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, ABSORB_EFFECT_VTABLE);
    stop_bolt(e, this);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(0x58), 0u32]);
    release_node(e, this);
    for offset in [0x58u32, 0x54, 0x50, 0x4c] {
        e.call(POINTER_DESTRUCT, &args![base.wrapping_add(offset)]);
    }
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00802f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00802f40 - the FS:[0] exception frame (deleting the copy on unwinding) is not translated.
/// `AbsorbEffect::Copy` (engine map; `ECX`): a new effect (0x5C bytes, `operator new`) built
/// with the words at +0x28, +8 and +0xC (null when the allocation fails), handed to vtable slot
/// 0x44 of this one; answers it. The exception frame is not translated.
pub fn absorb_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let block = e.call(OPERATOR_NEW, &args![0x5cu32]).u32();
    let copy = if block != 0 {
        let first = e.mem.u32(base.wrapping_add(0x28));
        let second = e.mem.u32(base.wrapping_add(8));
        let third = e.mem.u32(base.wrapping_add(0xc));
        fn_00802cb0(e, Ptr::new(block), first, second, third).addr()
    } else {
        0
    };
    e.vcall(base, 0x44, &args![copy]);
    copy
}

// Translated from 00802ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00802ff0 - the FS:[0] exception frame (deleting the effect on unwinding) is not translated.
/// Unnamed in the map (`cdecl`: three words): a new `AbsorbEffect` (0x5C bytes) built with
/// them, null when the allocation fails. The exception frame is not translated.
pub fn fn_00802ff0(e: &mut Engine, first: u32, second: u32, third: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x5cu32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00802cb0(e, Ptr::new(block), first, second, third).addr()
}

/// Whether the magnitude (+0x20) equals the `double` 0.0 (`FCOMP`; unordered is unequal).
fn magnitude_is_zero(e: &mut Engine, this: Ptr) -> bool {
    let magnitude = x87_load32(e.mem.u32(this.addr().wrapping_add(0x20)));
    magnitude == e.global::<f64>(DOUBLE_ZERO)
}

// Translated from 00803070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AbsorbEffect::Start` (engine map; `ECX`): `ValueModifierEffect::Start`; for a
/// detrimental effect or a zero magnitude, gives the caster (as an actor, or 0) the negated
/// magnitude (`00495460`, slot 0x64 with the caster, the value and -1); then the attribute
/// reduction message and `Start3D`.
pub fn absorb_effect_start(e: &mut Engine, this: Ptr) {
    e.call(BASE_START, &args![this]);
    if is_detrimental(e, this) || magnitude_is_zero(e, this) {
        let caster = caster_actor(e, this);
        let magnitude = e.call(MAGNITUDE, &args![this]).f64();
        let value = x87_store32(-magnitude);
        e.vcall(this.addr(), 0x64, &args![caster, value, 0xffff_ffffu32]);
    }
    e.call(SHOW_ATTRIBUTE_REDUCTION_MESSAGE, &args![this]);
    absorb_effect_start3_d(e, this);
}

// Translated from 00803120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AbsorbEffect::Update` (engine map, `RET 4`: the elapsed time): `ValueModifierEffect::Update`;
/// unless detrimental, with a positive magnitude, reads slot 0x60 (and dispels when the
/// target's value for it, slot 8 of its +0xA4 component, is not above 0) and, unless ending
/// (`00705cf0`), gives the caster the negated magnitude times the time (slot 0x64). Then,
/// unless ending, when target and caster are actors (slot 0x1D0) not dead (slot 0x22C with 0)
/// and within weapon reach (`GetDistanceFromReference`, `CalcWeaponReach` of `011d1aa8`), the
/// caster's acquired object (slot 0x504 of `GetSavedAcquireObject`) with its byte at +0x1E
/// clear dispels; out of reach or missing anything dispels too. Unless ending, `Update3D`.
pub fn absorb_effect_update(e: &mut Engine, this: Ptr, elapsed: f32) {
    let base = this.addr();
    let elapsed_bits = elapsed.to_bits();
    e.call(BASE_UPDATE, &args![this, x87_quiet_bits(elapsed_bits)]);
    let caster = caster_actor(e, this);
    let target = if e.call(TARGET_OF, &args![this]).u32() != 0 {
        let target = e.call(TARGET_OF, &args![this]).u32();
        e.call(TARGET_AS_ACTOR, &args![target]).u32()
    } else {
        0
    };
    let magnitude = x87_load32(e.mem.u32(base.wrapping_add(0x20)));
    if !is_detrimental(e, this) && magnitude > e.global::<f64>(DOUBLE_ZERO) {
        e.vcall(base, 0x60, &[]);
        if target != 0 {
            let value_kind = e.vcall(base, 0x60, &[]).u32();
            let value = e
                .vcall(target.wrapping_add(0xa4), 8, &args![value_kind])
                .i32();
            if f64::from(value) <= e.global::<f64>(DOUBLE_ZERO) {
                e.call(DISPEL, &args![this, 0u32]);
            }
        }
        if !e.call(IS_ENDING, &args![this]).bool() {
            let magnitude = e.call(MAGNITUDE, &args![this]).f64();
            let drained = x87_mul(-magnitude, x87_m32(elapsed_bits));
            let drained = x87_quiet_bits(x87_store32(drained));
            e.vcall(base, 0x64, &args![caster, drained, 0xffff_ffffu32]);
        }
    }
    let alive = |e: &mut Engine, actor: u32| {
        actor != 0
            && e.vcall(actor, 0x1d0, &[]).u32() != 0
            && !e.vcall(actor, 0x22c, &args![0u32]).bool()
    };
    if !e.call(IS_ENDING, &args![this]).bool() && alive(e, target) && alive(e, caster) {
        let distance = e
            .call(DISTANCE_FROM_REFERENCE, &args![caster, target, 0u32, 0u32])
            .f64();
        let distance = x87_store32(distance);
        let reach_at = e.call(SETTING_FLOAT, &args![SETTING_REACH]).u32();
        let reach_setting = x87_quiet_bits(e.mem.u32(reach_at));
        let reach = e.call(WEAPON_REACH, &args![reach_setting]).f64();
        if x87_load32(distance) > reach {
            e.call(DISPEL, &args![this, 0u32]);
        } else if e.call(SAVED_ACQUIRE_OBJECT, &args![caster]).u32() != 0 {
            let acquire = e.call(SAVED_ACQUIRE_OBJECT, &args![caster]).u32();
            let found = e.vcall(acquire, 0x504, &args![target, 0u32]).u32();
            if found != 0 && e.mem.u8(found.wrapping_add(0x1e)) == 0 {
                e.call(DISPEL, &args![this, 0u32]);
            }
        }
    } else {
        e.call(DISPEL, &args![this, 0u32]);
    }
    if !e.call(IS_ENDING, &args![this]).bool() {
        absorb_effect_update3_d(e, this, f32::from_bits(x87_quiet_bits(elapsed_bits)));
    }
}

// Translated from 008033b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AbsorbEffect::Finish` (engine map; `ECX`): `ValueModifierEffect::Finish`; for a
/// detrimental effect, takes back from the caster the magnitude as
/// `CheckAdjustNoNegativeMagnitude` adjusts it (slot 0x64 with the caster, the value and -1);
/// then the bolt stopped and the node given back to its owner and cleared.
pub fn absorb_effect_finish(e: &mut Engine, this: Ptr) {
    e.call(BASE_FINISH, &args![this]);
    if is_detrimental(e, this) {
        let caster = caster_actor(e, this);
        let magnitude = e.call(MAGNITUDE, &args![this]).f64();
        let adjusted = e
            .call(
                CHECK_ADJUST_NO_NEGATIVE_MAGNITUDE,
                &args![this, x87_store32(magnitude)],
            )
            .f64();
        let adjusted = x87_quiet_bits(x87_store32(adjusted));
        e.vcall(this.addr(), 0x64, &args![caster, adjusted, 0xffff_ffffu32]);
    }
    stop_bolt(e, this);
    release_node(e, this);
}

/// `Start3D`'s spine step for one actor: its 3D root's (`00496940`, `00537bd0`) node named
/// "Bip01 Spine1" (slot 0x8C with a fixed string, destroyed after) into the member at
/// `offset`; when `00822510` refuses it, the other actor's slot 0x1D0 instead.
fn take_spine(e: &mut Engine, this: Ptr, actor: u32, other: u32, offset: u32, string: u32) {
    let root = e.vcall(actor, 0x1e4, &[]).u32();
    if root == 0 {
        return;
    }
    let data = e.call(ACTOR_3D_ROOT, &args![root]).u32();
    let node = e.call(ROOT_NODE, &args![data]).u32();
    let name = e.call(FIXED_STRING, &args![string, TEXT_SPINE]).u32();
    let spine = e.vcall(node, 0x8c, &args![name]).u32();
    let target = this.addr().wrapping_add(offset);
    e.call(POINTER_ASSIGN, &args![target, spine]);
    e.call(FIXED_STRING_DESTRUCT, &args![string]);
    if e.call(NODE_TEST, &args![target, 0u32]).bool() {
        let fallback = e.vcall(other, 0x1d0, &[]).u32();
        e.call(POINTER_ASSIGN, &args![target, fallback]);
    }
}

/// The float value of a setting (`00403e20`; `FLD`/`FSTP` keep its bits, quieted).
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let at = e.call(SETTING_FLOAT, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(at))
}

/// A colour made from three float settings and 1.0 (`00414430` into `local`), its four words.
fn color_from(e: &mut Engine, local: u32, settings: [u32; 3]) -> [u32; 4] {
    let alpha = x87_store32(1.0);
    let third = setting_bits(e, settings[0]);
    let second = setting_bits(e, settings[1]);
    let first = setting_bits(e, settings[2]);
    let color = e
        .call(MAKE_COLOR, &args![local, first, second, third, alpha])
        .u32();
    [0, 4, 8, 12].map(|i| e.mem.u32(color.wrapping_add(i)))
}

// Translated from 008034c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008034c0 - the FS:[0] exception frame (unwinding the new objects and strings) is not translated.
/// `AbsorbEffect::Start3D` (engine map; `ECX`): a new node (0xAC bytes, `00aa13e0`,
/// `00a5ecb0` with 0) for the bolt (+0x4C) with `BSXFlags` extra data (made when missing,
/// `00519230` with 1, 1). With a target and a caster (their 3D objects, slots 4 and 0x2C):
/// the node goes under the target cell's node (slot 0xDC with 1), the bolt shader is started
/// on it (`BoltShader::Start`, +0x58), the spines of caster (+0x50) and target (+0x54) are
/// found ("Bip01 Spine1"), and the bolt gets its settings: seven floats (`011d1710`,
/// `011d1c88`, `011d1af0`, `011d1d3c`, `011d1ac0`, `011d1ab4`, `011d1680`), a word
/// (`011d1c64`), two colours (`00414430` of three float settings and 1.0), then 0, 1 and
/// `00bc2360`. The exception frame is not translated.
pub fn absorb_effect_start3_d(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let block = e.call(NI_NEW, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(0x4c), node]);
    let name = e.call(BSX_FLAGS_NAME, &[]).u32();
    let node = member(e, this, 0x4c);
    let mut flags = e.call(GET_EXTRA_DATA, &args![node, name]).u32();
    if flags == 0 {
        let block = e.call(NI_NEW, &args![0x10u32]).u32();
        flags = if block != 0 {
            e.call(BSX_FLAGS_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        let name = e.call(BSX_FLAGS_NAME, &[]).u32();
        let node = member(e, this, 0x4c);
        e.call(ADD_EXTRA_DATA, &args![node, name, flags]);
    }
    e.call(FLAGS_SET_519230, &args![flags, 1u32, 1u32]);
    let target = if e.call(TARGET_OF, &args![this]).u32() != 0 {
        let target = e.call(TARGET_OF, &args![this]).u32();
        e.vcall(target, 4, &[]).u32()
    } else {
        0
    };
    let caster = if e.call(CASTER_OF, &args![this]).u32() != 0 {
        let caster = e.call(CASTER_OF, &args![this]).u32();
        e.vcall(caster, 0x2c, &[]).u32()
    } else {
        0
    };
    if target == 0 || caster == 0 {
        return;
    }
    let cell = e.call(PARENT_CELL, &args![target]).u32();
    let cell_node = e.call(CELL_NODE, &args![cell]).u32();
    let node = member(e, this, 0x4c);
    e.vcall(cell_node, 0xdc, &args![node, 1u32]);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(0x58), 0u32]);
    let node = member(e, this, 0x4c);
    let shader = e.call(BOLT_START, &args![node, 1u32]).u32();
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(0x58), shader]);
    e.with_stack(8, |e, strings| {
        take_spine(e, this, caster, target, 0x50, strings.addr());
        take_spine(e, this, target, caster, 0x54, strings.addr() + 4);
    });
    if member(e, this, 0x58) == 0 {
        return;
    }
    // Each setting's float is read before the bolt; the setters at 00803b30, 00803b50 and
    // 00803a90 are this unit's own.
    let steps = [
        (0x011d_1710u32, BOLT_SET_821620),
        (0x011d_1c88, BOLT_SET_507780),
        (0x011d_1af0, 0x0080_3b30),
        (0x011d_1d3c, BOLT_SET_5077A0),
        (0x011d_1ac0, 0x0080_3b50),
        (0x011d_1ab4, BOLT_SET_5F9B10),
        (0x011d_1680, 0x0080_3a90),
    ];
    for (setting, setter) in steps {
        let value = setting_bits(e, setting);
        let bolt = member(e, this, 0x58);
        match setter {
            0x0080_3b30 => fn_00803b30(e, Ptr::new(bolt), f32::from_bits(value)),
            0x0080_3b50 => fn_00803b50(e, Ptr::new(bolt), f32::from_bits(value)),
            0x0080_3a90 => fn_00803a90(e, Ptr::new(bolt), f32::from_bits(value)),
            _ => {
                e.call(setter, &args![bolt, value]);
            }
        }
    }
    let at = e.call(SETTING_WORD, &args![0x011d_1c64u32]).u32();
    let word = e.mem.u32(at);
    let bolt = member(e, this, 0x58);
    fn_00803a70(e, Ptr::new(bolt), word);
    e.with_stack(0x20, |e, colors| {
        let [a, b, c, d] = color_from(
            e,
            colors.addr() + 0x10,
            [0x011d_18b4, 0x011d_1570, 0x011d_1c94],
        );
        let bolt = member(e, this, 0x58);
        fn_00803ab0(e, Ptr::new(bolt), a, b, c, d);
        let [a, b, c, d] = color_from(e, colors.addr(), [0x011d_1818, 0x011d_1830, 0x011d_16a4]);
        let bolt = member(e, this, 0x58);
        fn_00803ae0(e, Ptr::new(bolt), a, b, c, d);
    });
    let bolt = member(e, this, 0x58);
    fn_00803b10(e, Ptr::new(bolt), 0);
    let bolt = member(e, this, 0x58);
    fn_00803a50(e, Ptr::new(bolt), 1);
    let bolt = member(e, this, 0x58);
    e.call(BOLT_READY, &args![bolt]);
}

// Translated from 00803a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a byte): stores it at +0x177.
pub fn fn_00803a50(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x177), value);
}

// Translated from 00803a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a word): stores it at +0x128.
pub fn fn_00803a70(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x128), value);
}

// Translated from 00803ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 0x10`: four words, a colour by value): stores them at +0x154.
pub fn fn_00803ab0(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32, d: u32) {
    let at = this.addr().wrapping_add(0x154);
    for (i, word) in [a, b, c, d].iter().enumerate() {
        e.mem.set_u32(at.wrapping_add(4 * i as u32), *word);
    }
}

// Translated from 00803ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 0x10`: four words, a colour by value): stores them at +0x164.
pub fn fn_00803ae0(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32, d: u32) {
    let at = this.addr().wrapping_add(0x164);
    for (i, word) in [a, b, c, d].iter().enumerate() {
        e.mem.set_u32(at.wrapping_add(4 * i as u32), *word);
    }
}

// Translated from 00803b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a byte): stores it at +0x175.
pub fn fn_00803b10(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x175), value);
}

// Translated from 00803b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a float): stores it at +0x144 (`FLD`/`FSTP`: a signalling
/// NaN arrives quiet).
pub fn fn_00803b30(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x144, x87_quiet_bits(value.to_bits()));
}

// Translated from 00803b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a float): stores it at +0x150 (`FLD`/`FSTP`).
pub fn fn_00803b50(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x150, x87_quiet_bits(value.to_bits()));
}

// Translated from 00803b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AbsorbEffect::Update3D` (engine map, `RET 4`: the elapsed time): with all four members set
/// and outside menu mode (`011db9f4`), places the node at the caster spine's world position
/// (`0045bb80`, `00440460`), aims it at the target spine (the difference `00439ef0`; the
/// rotation from `atan2` of the height over the horizontal length and of y over x,
/// `NiMatrix3::FromEulerAnglesZYX`, `0043fa80`) and updates the bolt (`00bc2740` with the
/// time, the length and the setting `011d1c10`).
pub fn absorb_effect_update3_d(e: &mut Engine, this: Ptr, elapsed: f32) {
    for offset in [0x50u32, 0x54, 0x4c, 0x58] {
        if member(e, this, offset) == 0 {
            return;
        }
    }
    if e.global::<u8>(MENU_MODE_BYTE) != 0 {
        return;
    }
    // Locals: +0 the point (3 words), +0xC the difference (3 words), +0x18 the matrix (0x24).
    e.with_stack(0x3c, |e, frame| {
        let point = frame.addr();
        let difference = point + 0xc;
        let matrix = point + 0x18;
        for i in 0..3 {
            let word = e.global::<u32>(DEFAULT_POINT + 4 * i);
            e.mem.set_u32(point + 4 * i, word);
        }
        let caster_spine = member(e, this, 0x50);
        let position = e.call(WORLD_POSITION, &args![caster_spine]).u32();
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.mem.set_u32(point + 4 * i, word);
        }
        let node = member(e, this, 0x4c);
        e.call(SET_POSITION, &args![node, point]);
        let target_spine = member(e, this, 0x54);
        let target_position = e.call(WORLD_POSITION, &args![target_spine]).u32();
        e.call(POINT_MINUS, &args![target_position, difference, point]);
        e.call(SELF, &args![matrix]);
        let height = x87_quiet_bits(e.mem.u32(difference + 8));
        e.mem.set_u32(difference + 8, x87_store32(0.0));
        let flat = e.call(POINT_LENGTH, &args![difference]).f64();
        let flat = x87_quiet_bits(x87_store32(flat));
        let pitch = e.call(ATAN2, &args![height, flat]).f64();
        let pitch = x87_store32(-pitch);
        let x = x87_quiet_bits(e.mem.u32(difference));
        let y = x87_quiet_bits(e.mem.u32(difference + 4));
        let heading = e.call(ATAN2, &args![x, y]).f64();
        let heading = x87_store32(heading);
        e.call(
            FROM_EULER_ANGLES_ZYX,
            &args![matrix, heading, x87_store32(0.0), pitch],
        );
        let node = member(e, this, 0x4c);
        e.call(SET_ROTATE, &args![node, matrix]);
        let setting = setting_bits(e, SETTING_BOLT_LENGTH);
        let length = e.call(POINT_LENGTH, &args![difference]).f64();
        let length = x87_quiet_bits(x87_store32(length));
        let time = x87_quiet_bits(elapsed.to_bits());
        let bolt = member(e, this, 0x58);
        e.call(BOLT_UPDATE, &args![bolt, time, length, setting, 0u32]);
    });
}

// Translated from 00803d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AbsorbEffect::FinishInitLoadGame` (engine map, `RET 4`):
/// `ValueModifierEffect::FinishInitLoadGame` with the word, then `Start3D`.
pub fn absorb_effect_finish_init_load_game(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(BASE_FINISH_INIT_LOAD_GAME, &args![this, argument]);
    absorb_effect_start3_d(e, this);
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_2d90, value_modifier_effect_get_save_size(Ptr, Ptr) -> u16),
        entry!(0x0080_2db0, value_modifier_effect_save_game(Ptr, Ptr)),
        entry!(0x0080_2dd0, value_modifier_effect_load_game(Ptr, Ptr)),
        entry!(0x0080_3a90, fn_00803a90(Ptr, f32)),
        entry!(0x0080_2cb0, fn_00802cb0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0080_2df0, absorb_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0080_2e20, absorb_effect_destructor(Ptr)),
        entry!(0x0080_2f40, absorb_effect_copy(Ptr) -> u32),
        entry!(0x0080_2ff0, fn_00802ff0(u32, u32, u32) -> u32),
        entry!(0x0080_3070, absorb_effect_start(Ptr)),
        entry!(0x0080_3120, absorb_effect_update(Ptr, f32)),
        entry!(0x0080_33b0, absorb_effect_finish(Ptr)),
        entry!(0x0080_34c0, absorb_effect_start3_d(Ptr)),
        entry!(0x0080_3a50, fn_00803a50(Ptr, u8)),
        entry!(0x0080_3a70, fn_00803a70(Ptr, u32)),
        entry!(0x0080_3ab0, fn_00803ab0(Ptr, u32, u32, u32, u32)),
        entry!(0x0080_3ae0, fn_00803ae0(Ptr, u32, u32, u32, u32)),
        entry!(0x0080_3b10, fn_00803b10(Ptr, u8)),
        entry!(0x0080_3b30, fn_00803b30(Ptr, f32)),
        entry!(0x0080_3b50, fn_00803b50(Ptr, f32)),
        entry!(0x0080_3b70, absorb_effect_update3_d(Ptr, f32)),
        entry!(0x0080_3d00, absorb_effect_finish_init_load_game(Ptr, u32)),
    ]
}

#[cfg(test)]
mod b066_tests {
    use super::*;

    #[test]
    fn fn_00803a90_stores_the_float_at_0x13c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x140);
        e.call(0x0080_3a90, &args![this, 1.25f32]);
        assert_eq!(e.mem.f32(this + 0x13c), 1.25);
        e.call(0x0080_3a90, &args![this, 0xff80_0001u32]);
        assert_eq!(e.mem.u32(this + 0x13c), 0xffc0_0001);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_value_modifier_effect_get_save_size_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(GET_SAVE_SIZE_TARGET_00804F80, |_, _| {
            0xdead_beefu32.into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0080_2d90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == GET_SAVE_SIZE_TARGET_00804F80)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u16(), 0xbeef);
    }

    #[test]
    fn forwarder_value_modifier_effect_save_game_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(SAVE_GAME_TARGET_008050D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0080_2db0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == SAVE_GAME_TARGET_008050D0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_value_modifier_effect_load_game_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_GAME_TARGET_00805330, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0080_2dd0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_GAME_TARGET_00805330)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn position(log: &Log, addr: u32) -> usize {
        log.iter().position(|(a, _)| *a == addr).unwrap()
    }

    /// An object whose vtable maps the given slots to the given addresses.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let o = e.mem.alloc(0x200);
        let vt = e.mem.alloc(0x600);
        e.mem.set_u32(o, vt);
        for (slot, target) in slots {
            e.mem.set_u32(vt + slot, *target);
        }
        o
    }

    /// Pointer members as plain words: get reads, assign writes.
    fn pointers(e: &mut Engine) {
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_CONSTRUCT, ok);
        e.register(POINTER_DESTRUCT, ok);
    }

    /// The globals the effect reads: the `double` 0.0, the settings page, the menu byte and
    /// the default point.
    fn globals(e: &mut Engine) {
        e.map(0x0101_2000, 0x1000);
        e.map(0x011d_1000, 0x1000);
        e.map(0x011d_b000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.register(SETTING_FLOAT, |_, a| ret(a[0]));
        e.register(SETTING_WORD, |_, a| ret(a[0]));
    }

    /// `Start3D` leaving early: no node, flags found, no target or caster.
    fn quiet_start3d(e: &mut Engine) {
        e.register(NI_NEW, |_, _| ret(0));
        e.register(BSX_FLAGS_NAME, |_, _| ret(0x77));
        e.register(GET_EXTRA_DATA, |_, _| ret(0x55));
        e.register(FLAGS_SET_519230, ok);
        e.register(TARGET_OF, |_, _| ret(0));
        e.register(CASTER_OF, |_, _| ret(0));
    }

    /// An effect with the item at +0xC, the magnitude at +0x20 and slots 0x44, 0x60, 0x64.
    fn effect(e: &mut Engine, magnitude: f32) -> u32 {
        let this = object(
            e,
            &[
                (0x44, 0x0900_0044),
                (0x60, 0x0900_0060),
                (0x64, 0x0900_0064),
            ],
        );
        e.mem.set_u32(this + 0xc, 0x1234);
        e.mem.set_f32(this + 0x20, magnitude);
        e.register(0x0900_0044, ok);
        e.register(0x0900_0060, |_, _| ret(9));
        e.register(0x0900_0064, ok);
        this
    }

    fn detrimental(e: &mut Engine, yes: bool) {
        e.register(EFFECT_SETTING_OF, |_, a| ret(a[0] + 1));
        if yes {
            e.register(SETTING_HAS_FLAG, |_, a| ret(u32::from(a[1] == 2)));
        } else {
            e.register(SETTING_HAS_FLAG, |_, _| ret(0));
        }
    }

    #[test]
    fn constructor_sets_the_vtable_and_four_null_pointers() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, ok);
        pointers(&mut e);
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + 0x58, 7);
        e.call_log = Some(vec![]);
        let result = e.call(0x0080_2cb0, &[this, 1, 2, 3]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), ABSORB_EFFECT_VTABLE);
        assert_eq!(only(&log, BASE_CONSTRUCT), vec![vec![this, 1, 2, 3]]);
        let offsets = |addr| -> Vec<u32> { only(&log, addr).iter().map(|w| w[0] - this).collect() };
        assert_eq!(offsets(POINTER_CONSTRUCT), vec![0x4c, 0x50, 0x54, 0x58]);
        assert_eq!(offsets(POINTER_ASSIGN), vec![0x58, 0x4c, 0x54, 0x50]);
        assert_eq!(e.mem.u32(this + 0x58), 0);
    }

    fn destructor_doubles(e: &mut Engine) {
        pointers(e);
        e.register(BOLT_STOP, ok);
        e.register(NODE_OWNER, |_, a| ret(a[0] & 0xff00_0000));
        e.register(BASE_DESTRUCT, ok);
        e.register(OPERATOR_DELETE, ok);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_with_flag_one() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let this = e.mem.alloc(0x5c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_2df0, &[this, 0]).u32(), this);
        assert_eq!(e.call(0x0080_2df0, &[this, 3]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, BASE_DESTRUCT).len(), 2);
        assert_eq!(only(&log, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn destructor_stops_the_bolt_and_gives_the_node_back() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let owner = object(&mut e, &[(0xe8, 0x0900_00e8)]);
        e.register(0x0900_00e8, ok);
        e.register_double(NODE_OWNER, move |_, _| ret(owner));
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + 0x4c, 0x4c4c);
        e.mem.set_u32(this + 0x58, 0x5858);
        e.call_log = Some(vec![]);
        e.call(0x0080_2e20, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), ABSORB_EFFECT_VTABLE);
        assert_eq!(only(&log, BOLT_STOP), vec![vec![0x4c4c, 0x5858]]);
        assert_eq!(only(&log, 0x0900_00e8), vec![vec![owner, 0x4c4c]]);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        let destructed: Vec<u32> = only(&log, POINTER_DESTRUCT)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(destructed, vec![0x58, 0x54, 0x50, 0x4c]);
        assert_eq!(only(&log, BASE_DESTRUCT), vec![vec![this]]);
        // Without a node nobody is asked for it.
        let other = e.mem.alloc(0x5c);
        e.call_log = Some(vec![]);
        e.call(0x0080_2e20, &[other]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, NODE_OWNER).is_empty());
    }

    #[test]
    fn copy_builds_a_new_effect_from_three_words() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, ok);
        pointers(&mut e);
        let block = e.mem.alloc(0x5c);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let this = effect(&mut e, 1.0);
        e.mem.set_u32(this + 0x28, 0x28);
        e.mem.set_u32(this + 8, 8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_2f40, &[this]).u32(), block);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, OPERATOR_NEW), vec![vec![0x5c]]);
        assert_eq!(
            only(&log, BASE_CONSTRUCT),
            vec![vec![block, 0x28, 8, 0x1234]]
        );
        assert_eq!(only(&log, 0x0900_0044), vec![vec![this, block]]);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_2f40, &[this]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0900_0044), vec![vec![this, 0]]);
    }

    #[test]
    fn create_allocates_and_constructs() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, ok);
        pointers(&mut e);
        let block = e.mem.alloc(0x5c);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_2ff0, &[4, 5, 6]).u32(), block);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, BASE_CONSTRUCT), vec![vec![block, 4, 5, 6]]);
        assert_eq!(e.mem.u32(block), ABSORB_EFFECT_VTABLE);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x0080_2ff0, &[4, 5, 6]).u32(), 0);
    }

    #[test]
    fn start_gives_the_caster_the_negated_magnitude_when_detrimental_or_zero() {
        let mut e = Engine::new();
        globals(&mut e);
        pointers(&mut e);
        quiet_start3d(&mut e);
        e.register(BASE_START, ok);
        e.register(SHOW_ATTRIBUTE_REDUCTION_MESSAGE, ok);
        e.register(MAGNITUDE, |_, _| st0(3.0));
        e.register(CASTER_AS_ACTOR, |_, _| ret(0xac));
        detrimental(&mut e, true);
        let this = effect(&mut e, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x0080_3070, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, 0x0900_0064),
            vec![vec![this, 0, (-3.0f32).to_bits(), 0xffff_ffff]]
        );
        assert!(position(&log, BASE_START) < position(&log, 0x0900_0064));
        assert!(position(&log, SHOW_ATTRIBUTE_REDUCTION_MESSAGE) < position(&log, NI_NEW));
        // Not detrimental: only with a zero magnitude, and the caster as an actor.
        detrimental(&mut e, false);
        let caster = object(&mut e, &[(0x2c, 0x0900_002c)]);
        e.register(0x0900_002c, ok);
        e.register_double(CASTER_OF, move |_, _| ret(caster));
        e.call_log = Some(vec![]);
        e.call(0x0080_3070, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, 0x0900_0064).is_empty());
        e.mem.set_f32(this + 0x20, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x0080_3070, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, CASTER_AS_ACTOR), vec![vec![caster]]);
        assert_eq!(only(&log, 0x0900_0064)[0][1], 0xac);
        e.mem.set_u32(this + 0x20, 0x7fc0_0000);
        e.call_log = Some(vec![]);
        e.call(0x0080_3070, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, 0x0900_0064).is_empty());
    }

    /// An actor for `Update`: slot 0x1D0 answers `has_3d`, slot 0x22C `dead`, its +0xA4
    /// component's slot 8 answers `value`.
    fn actor(e: &mut Engine, base: u32, has_3d: u32, dead: u32, value: u32) -> u32 {
        let a = object(e, &[(0x1d0, base), (0x22c, base + 4)]);
        let component = e.mem.alloc(0x10);
        e.mem.set_u32(a + 0xa4, component);
        e.mem.set_u32(component + 8, base + 8);
        e.register_double(base, move |_, _| ret(has_3d));
        e.register_double(base + 4, move |_, _| ret(dead));
        e.register_double(base + 8, move |_, _| ret(value));
        a
    }

    fn update_doubles(e: &mut Engine, caster: u32, target: u32) {
        globals(e);
        pointers(e);
        e.register(BASE_UPDATE, ok);
        e.register(DISPEL, ok);
        e.register(IS_ENDING, |_, _| ret(0));
        e.register(MAGNITUDE, |_, _| st0(2.0));
        e.register_double(CASTER_OF, move |_, _| ret(u32::from(caster != 0)));
        e.register_double(CASTER_AS_ACTOR, move |_, _| ret(caster));
        e.register_double(TARGET_OF, move |_, _| ret(u32::from(target != 0)));
        e.register_double(TARGET_AS_ACTOR, move |_, _| ret(target));
        e.register(DISTANCE_FROM_REFERENCE, |_, _| st0(100.0));
        e.register(WEAPON_REACH, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        e.register(SAVED_ACQUIRE_OBJECT, |_, _| ret(0));
    }

    #[test]
    fn update_drains_and_dispels_out_of_reach() {
        let mut e = Engine::new();
        let target = actor(&mut e, 0x0910_0000, 1, 0, 0);
        let caster = actor(&mut e, 0x0920_0000, 1, 0, 5);
        update_doubles(&mut e, caster, target);
        detrimental(&mut e, false);
        let this = effect(&mut e, 2.0);
        e.mem.set_f32(0x011d_1aa8, 50.0);
        e.call_log = Some(vec![]);
        e.call(0x0080_3120, &[this, 0.5f32.to_bits()]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, BASE_UPDATE), vec![vec![this, 0.5f32.to_bits()]]);
        // The target's value (0) is not above zero: dispelled; then out of reach: again.
        assert_eq!(only(&log, 0x0910_0008), vec![vec![target + 0xa4, 9]]);
        assert_eq!(only(&log, DISPEL), vec![vec![this, 0], vec![this, 0]]);
        assert_eq!(
            only(&log, 0x0900_0064),
            vec![vec![this, caster, (-1.0f32).to_bits(), 0xffff_ffff]]
        );
        assert_eq!(
            only(&log, DISTANCE_FROM_REFERENCE),
            vec![vec![caster, target, 0, 0]]
        );
        assert_eq!(only(&log, WEAPON_REACH), vec![vec![50.0f32.to_bits()]]);
        // Update3D ran and left at once (no members).
        assert_eq!(only(&log, POINTER_GET), vec![vec![this + 0x50]]);
        // In reach: the acquired object with its byte clear dispels.
        e.mem.set_f32(0x011d_1aa8, 500.0);
        let found = e.mem.alloc(0x20);
        let acquire = object(&mut e, &[(0x504, 0x0930_0504)]);
        e.register_double(0x0930_0504, move |_, _| ret(found));
        e.register_double(SAVED_ACQUIRE_OBJECT, move |_, _| ret(acquire));
        e.call_log = Some(vec![]);
        e.call(0x0080_3120, &[this, 0.5f32.to_bits()]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, 0x0930_0504), vec![vec![acquire, target, 0]]);
        assert_eq!(only(&log, DISPEL).len(), 2);
        e.mem.set_u8(found + 0x1e, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_3120, &[this, 0.5f32.to_bits()]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, DISPEL).len(), 1);
    }

    #[test]
    fn update_dispels_without_live_actors_and_skips_the_drain_when_detrimental() {
        let mut e = Engine::new();
        let target = actor(&mut e, 0x0910_0000, 1, 1, 5);
        let caster = actor(&mut e, 0x0920_0000, 1, 0, 5);
        update_doubles(&mut e, caster, target);
        detrimental(&mut e, true);
        let this = effect(&mut e, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x0080_3120, &[this, 0.5f32.to_bits()]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, 0x0900_0064).is_empty());
        assert_eq!(only(&log, DISPEL), vec![vec![this, 0]]);
        assert!(only(&log, DISTANCE_FROM_REFERENCE).is_empty());
        // Ending: no drain, a dispel and no Update3D.
        detrimental(&mut e, false);
        e.register(IS_ENDING, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0080_3120, &[this, 0.5f32.to_bits()]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, 0x0900_0064).is_empty());
        assert_eq!(only(&log, DISPEL), vec![vec![this, 0]]);
        assert!(only(&log, POINTER_GET).is_empty());
    }

    #[test]
    fn finish_takes_back_the_adjusted_magnitude_and_stops_the_bolt() {
        let mut e = Engine::new();
        pointers(&mut e);
        e.register(BASE_FINISH, ok);
        e.register(MAGNITUDE, |_, _| st0(3.0));
        e.register(CHECK_ADJUST_NO_NEGATIVE_MAGNITUDE, |_, a| {
            st0(f64::from(f32::from_bits(a[1])) + 1.0)
        });
        e.register(CASTER_OF, |_, _| ret(0));
        e.register(BOLT_STOP, ok);
        e.register(NODE_OWNER, |_, _| ret(0));
        detrimental(&mut e, true);
        let this = effect(&mut e, 2.0);
        e.mem.set_u32(this + 0x4c, 0x4c4c);
        e.mem.set_u32(this + 0x58, 0x5858);
        e.call_log = Some(vec![]);
        e.call(0x0080_33b0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, CHECK_ADJUST_NO_NEGATIVE_MAGNITUDE),
            vec![vec![this, 3.0f32.to_bits()]]
        );
        assert_eq!(
            only(&log, 0x0900_0064),
            vec![vec![this, 0, 4.0f32.to_bits(), 0xffff_ffff]]
        );
        assert_eq!(only(&log, BOLT_STOP), vec![vec![0x4c4c, 0x5858]]);
        assert_eq!(only(&log, NODE_OWNER), vec![vec![0x4c4c]]);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        // The shader stays.
        assert_eq!(e.mem.u32(this + 0x58), 0x5858);
        detrimental(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0080_33b0, &[this]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, 0x0900_0064).is_empty());
    }

    #[test]
    fn start3d_builds_the_bolt_between_the_spines() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x5c);
        globals(&mut e);
        pointers(&mut e);
        e.register(NI_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(NODE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(BSX_FLAGS_NAME, |_, _| ret(0x77));
        e.register(GET_EXTRA_DATA, |_, _| ret(0));
        e.register(BSX_FLAGS_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ADD_EXTRA_DATA, ok);
        e.register(FLAGS_SET_519230, ok);
        let target = object(&mut e, &[(0x1e4, 0x0940_01e4), (0x1d0, 0x0940_01d0)]);
        let caster = object(&mut e, &[(0x1e4, 0x0950_01e4), (0x1d0, 0x0950_01d0)]);
        let target_ref = object(&mut e, &[(4, 0x0940_0004)]);
        let caster_ref = object(&mut e, &[(0x2c, 0x0950_002c)]);
        e.register_double(0x0940_0004, move |_, _| ret(target));
        e.register_double(0x0950_002c, move |_, _| ret(caster));
        e.register_double(TARGET_OF, move |_, _| ret(target_ref));
        e.register_double(CASTER_OF, move |_, _| ret(caster_ref));
        e.register(0x0940_01e4, |_, _| ret(0x41));
        e.register(0x0950_01e4, |_, _| ret(0x51));
        e.register(0x0940_01d0, |_, _| ret(0x4d0));
        e.register(0x0950_01d0, |_, _| ret(0x5d0));
        let cell_node = object(&mut e, &[(0xdc, 0x0960_00dc)]);
        e.register(0x0960_00dc, ok);
        e.register(PARENT_CELL, |_, _| ret(0xce));
        e.register_double(CELL_NODE, move |_, _| ret(cell_node));
        let bolt = e.mem.alloc(0x180);
        e.register_double(BOLT_START, move |_, _| ret(bolt));
        e.register(ACTOR_3D_ROOT, |_, a| ret(a[0] + 0x100));
        let root = object(&mut e, &[(0x8c, 0x0970_008c)]);
        e.register(0x0970_008c, |_, _| ret(0x5e));
        e.register_double(ROOT_NODE, move |_, _| ret(root));
        e.register(FIXED_STRING, |_, a| ret(a[0]));
        e.register(FIXED_STRING_DESTRUCT, ok);
        // The caster's spine is refused, the target's kept.
        e.register_double(NODE_TEST, move |_, a| ret(u32::from(a[0] == this + 0x50)));
        for setter in [
            BOLT_SET_821620,
            BOLT_SET_507780,
            BOLT_SET_5077A0,
            BOLT_SET_5F9B10,
            BOLT_READY,
        ] {
            e.register(setter, ok);
        }
        e.register(MAKE_COLOR, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            ret(a[0])
        });
        let settings = [
            0x011d_1710u32,
            0x011d_1c88,
            0x011d_1af0,
            0x011d_1d3c,
            0x011d_1ac0,
            0x011d_1ab4,
            0x011d_1680,
            0x011d_18b4,
            0x011d_1570,
            0x011d_1c94,
            0x011d_1818,
            0x011d_1830,
            0x011d_16a4,
        ];
        for (i, s) in settings.iter().enumerate() {
            e.mem.set_f32(*s, i as f32 + 1.0);
        }
        e.mem.set_u32(0x011d_1c64, 0x1c64);
        e.call_log = Some(vec![]);
        e.call(0x0080_34c0, &[this]);
        let log = e.call_log.take().unwrap();
        let node = only(&log, NODE_CONSTRUCT)[0][0];
        assert_eq!(only(&log, NI_NEW), vec![vec![0xac], vec![0x10]]);
        assert_eq!(e.mem.u32(this + 0x4c), node);
        let flags = only(&log, BSX_FLAGS_CONSTRUCT)[0][0];
        assert_eq!(only(&log, ADD_EXTRA_DATA), vec![vec![node, 0x77, flags]]);
        assert_eq!(only(&log, FLAGS_SET_519230), vec![vec![flags, 1, 1]]);
        assert_eq!(only(&log, 0x0960_00dc), vec![vec![cell_node, node, 1]]);
        assert_eq!(only(&log, BOLT_START), vec![vec![node, 1]]);
        assert_eq!(e.mem.u32(this + 0x58), bolt);
        assert_eq!(
            only(&log, FIXED_STRING)
                .iter()
                .map(|w| w[1])
                .collect::<Vec<_>>(),
            vec![TEXT_SPINE; 2]
        );
        assert_eq!(only(&log, ACTOR_3D_ROOT), vec![vec![0x51], vec![0x41]]);
        assert_eq!(e.mem.u32(this + 0x50), 0x4d0);
        assert_eq!(e.mem.u32(this + 0x54), 0x5e);
        let bits = |i: usize| (i as f32 + 1.0).to_bits();
        assert_eq!(only(&log, BOLT_SET_821620), vec![vec![bolt, bits(0)]]);
        assert_eq!(only(&log, BOLT_SET_507780), vec![vec![bolt, bits(1)]]);
        assert_eq!(e.mem.u32(bolt + 0x144), bits(2));
        assert_eq!(only(&log, BOLT_SET_5077A0), vec![vec![bolt, bits(3)]]);
        assert_eq!(e.mem.u32(bolt + 0x150), bits(4));
        assert_eq!(only(&log, BOLT_SET_5F9B10), vec![vec![bolt, bits(5)]]);
        assert_eq!(e.mem.u32(bolt + 0x13c), bits(6));
        assert_eq!(e.mem.u32(bolt + 0x128), 0x1c64);
        let one = 1.0f32.to_bits();
        let colors = only(&log, MAKE_COLOR);
        assert_eq!(colors[0][1..], [bits(9), bits(8), bits(7), one]);
        assert_eq!(colors[1][1..], [bits(12), bits(11), bits(10), one]);
        assert_eq!(e.mem.u32(bolt + 0x154), bits(9));
        assert_eq!(e.mem.u32(bolt + 0x160), one);
        assert_eq!(e.mem.u32(bolt + 0x164), bits(12));
        assert_eq!(e.mem.u8(bolt + 0x175), 0);
        assert_eq!(e.mem.u8(bolt + 0x177), 1);
        assert_eq!(only(&log, BOLT_READY), vec![vec![bolt]]);
    }

    #[test]
    fn start3d_without_a_caster_stops_after_the_flags() {
        let mut e = Engine::new();
        pointers(&mut e);
        quiet_start3d(&mut e);
        let this = e.mem.alloc(0x5c);
        e.call_log = Some(vec![]);
        e.call(0x0080_34c0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, FLAGS_SET_519230), vec![vec![0x55, 1, 1]]);
        assert!(only(&log, PARENT_CELL).is_empty());
    }

    #[test]
    fn byte_setters_store_their_bytes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x180);
        e.call(0x0080_3a50, &[this, 0x1ab]);
        e.call(0x0080_3b10, &[this, 0x2cd]);
        assert_eq!(e.mem.u8(this + 0x177), 0xab);
        assert_eq!(e.mem.u8(this + 0x175), 0xcd);
    }

    #[test]
    fn word_setter_stores_at_0x128() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x180);
        e.call(0x0080_3a70, &[this, 0xdead_beef]);
        assert_eq!(e.mem.u32(this + 0x128), 0xdead_beef);
    }

    #[test]
    fn colour_setters_store_four_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x180);
        e.call(0x0080_3ab0, &[this, 1, 2, 3, 4]);
        e.call(0x0080_3ae0, &[this, 5, 6, 7, 8]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this + 0x154 + 4 * i), i + 1);
            assert_eq!(e.mem.u32(this + 0x164 + 4 * i), i + 5);
        }
    }

    #[test]
    fn float_setters_quiet_signalling_nans() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x180);
        e.call(0x0080_3b30, &[this, 0x7f80_0001]);
        e.call(0x0080_3b50, &[this, 2.5f32.to_bits()]);
        assert_eq!(e.mem.u32(this + 0x144), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x150), 2.5);
    }

    #[test]
    fn update3d_aims_the_node_and_updates_the_bolt() {
        let mut e = Engine::new();
        globals(&mut e);
        pointers(&mut e);
        let this = e.mem.alloc(0x5c);
        let from = e.mem.alloc(12);
        let to = e.mem.alloc(12);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(from + 4 * i as u32, *v);
            e.mem.set_f32(to + 4 * i as u32, v * 3.0 + 1.0);
        }
        for (offset, value) in [(0x4cu32, 0x4c4c), (0x50, from), (0x54, to), (0x58, 0x5858)] {
            e.mem.set_u32(this + offset, value);
        }
        e.register(WORLD_POSITION, |_, a| ret(a[0]));
        e.register(SET_POSITION, ok);
        e.register(POINT_MINUS, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            Ret::default()
        });
        e.register(SELF, |_, a| ret(a[0]));
        e.register(POINT_LENGTH, |e, a| {
            let p: Vec<f64> = (0..3).map(|i| f64::from(e.mem.f32(a[0] + 4 * i))).collect();
            st0((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt())
        });
        e.register(ATAN2, |_, a| {
            st0(f64::from(f32::from_bits(a[0])).atan2(f64::from(f32::from_bits(a[1]))))
        });
        e.register(FROM_EULER_ANGLES_ZYX, ok);
        e.register(SET_ROTATE, ok);
        e.register(BOLT_UPDATE, ok);
        e.mem.set_f32(SETTING_BOLT_LENGTH, 7.0);
        e.call_log = Some(vec![]);
        e.call(0x0080_3b70, &[this, 0.25f32.to_bits()]);
        let log = e.call_log.take().unwrap();
        // Difference (3, 5, 7): height 7 over the flat length, x over y.
        let point = only(&log, SET_POSITION)[0][1];
        assert_eq!(only(&log, SET_POSITION), vec![vec![0x4c4c, point]]);
        let atans = only(&log, ATAN2);
        let flat = (34.0f64).sqrt() as f32;
        assert_eq!(atans[0], vec![7.0f32.to_bits(), flat.to_bits()]);
        assert_eq!(atans[1], vec![3.0f32.to_bits(), 5.0f32.to_bits()]);
        let euler = only(&log, FROM_EULER_ANGLES_ZYX);
        assert_eq!(euler[0][1], ((3.0f64).atan2(5.0) as f32).to_bits());
        assert_eq!(euler[0][2], 0);
        assert_eq!(
            euler[0][3],
            (-((7.0f64).atan2(f64::from(flat))) as f32).to_bits()
        );
        assert_eq!(only(&log, SET_ROTATE), vec![vec![0x4c4c, euler[0][0]]]);
        assert_eq!(
            only(&log, BOLT_UPDATE),
            vec![vec![
                0x5858,
                0.25f32.to_bits(),
                flat.to_bits(),
                7.0f32.to_bits(),
                0
            ]]
        );
        // In menu mode nothing moves.
        e.mem.set_u8(MENU_MODE_BYTE, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_3b70, &[this, 0.25f32.to_bits()]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, WORLD_POSITION).is_empty());
    }

    #[test]
    fn finish_init_load_game_passes_the_word_then_starts_the_3d() {
        let mut e = Engine::new();
        pointers(&mut e);
        quiet_start3d(&mut e);
        e.register(BASE_FINISH_INIT_LOAD_GAME, ok);
        let this = e.mem.alloc(0x5c);
        e.call_log = Some(vec![]);
        e.call(0x0080_3d00, &[this, 0x77]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, BASE_FINISH_INIT_LOAD_GAME),
            vec![vec![this, 0x77]]
        );
        assert!(position(&log, BASE_FINISH_INIT_LOAD_GAME) < position(&log, NI_NEW));
    }
}
