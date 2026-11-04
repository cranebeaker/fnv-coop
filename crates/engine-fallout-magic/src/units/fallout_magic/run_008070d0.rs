//! `fallout/magic/run_008070d0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00807770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: an index): the address of element
/// `index` of an array of 0x70-byte elements whose base is the word at `this`
/// (`base + index * 0x70`, 32-bit wrap-around).
pub fn fn_00807770(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0x70)
        .wrapping_add(e.mem.u32(this.addr()))
}

/// `bhkUtilFunctions::GetNiAVObject` (engine map, `00c7fa90`, `cdecl`, one
/// word) and `TESObjectREFR::FindReferenceFor3D` (engine map, `0056f930`,
/// `cdecl`, one word).
const GET_NI_AV_OBJECT: u32 = 0x00c7_fa90;
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) and the two type
/// descriptors of the cast in `008070d0`: from `011841cc`
/// (`.?AVTESObjectREFR@@`) to `011a0e88` (`.?AVMagicProjectile@@`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_MAGIC_PROJECTILE: u32 = 0x011a_0e88;
/// `004b5950` and `004b59f0` (`cdecl`, one word: the contact's word +0x48),
/// and `004b5a80` (`cdecl`, one word).
const CONTACT_TEST_004B5950: u32 = 0x004b_5950;
const CONTACT_OBJECT_004B59F0: u32 = 0x004b_59f0;
const CONTACT_OBJECT_004B5A80: u32 = 0x004b_5a80;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map, `008d8520`,
/// `thiscall`): the word at +0x68.
const SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
/// The player (`011dea3c`) and `0089f4e0` on it (`thiscall`: its word
/// +0x638).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_WORD_638_0089F4E0: u32 = 0x0089_f4e0;
/// `008ace90` (`thiscall`, returns a byte).
const ACTOR_TEST_008ACE90: u32 = 0x008a_ce90;
/// `00408d60` (`thiscall`: the address of a setting's value) on the
/// setting `bDrawSpellContact:General` (`011dbaa4`).
const SETTING_VALUE: u32 = 0x0040_8d60;
const DRAW_SPELL_CONTACT: u32 = 0x011d_baa4;
/// `006815c0` (`thiscall`): answers `this`.
const SELF_006815C0: u32 = 0x0068_15c0;
/// `00632d40` (`thiscall`, one word): copies the contact into a local.
const COPY_CONTACT_00632D40: u32 = 0x0063_2d40;
/// `00559450` (`thiscall`): the word at `this`.
const WORD_AT_0_00559450: u32 = 0x0055_9450;
/// `004a3a40` (`cdecl`, one word).
const NODE_OF_004A3A40: u32 = 0x004a_3a40;
/// The allocator `00aa13e0` (`cdecl`, the size) and the node constructor
/// `00a5ecb0` (`thiscall`, one word).
const ALLOCATE: u32 = 0x00aa_13e0;
const NODE_CONSTRUCT_00A5ECB0: u32 = 0x00a5_ecb0;
/// `00416870` (`thiscall`, three floats) and `00414430` (`thiscall`, four
/// floats): fill a local with the floats.
const SET_THREE_00416870: u32 = 0x0041_6870;
const SET_FOUR_00414430: u32 = 0x0041_4430;
/// `0044ddc0` (`thiscall`: the word at +8), `00436aa0` (`thiscall`:
/// `this + 0x30`), `00624b10` (`cdecl`, two words: copies three rows),
/// `00458620` (`cdecl`, two words: copies a point), `0043fa80` and
/// `00440460` (`thiscall`, one word: store a rotation and a translation).
const WORD_AT_8_0044DDC0: u32 = 0x0044_ddc0;
const PLUS_30_00436AA0: u32 = 0x0043_6aa0;
const COPY_ROWS_00624B10: u32 = 0x0062_4b10;
const COPY_POINT_00458620: u32 = 0x0045_8620;
const SET_ROTATE_0043FA80: u32 = 0x0043_fa80;
const SET_TRANSLATE_00440460: u32 = 0x0044_0460;
/// `MakeTriPoint` (engine map, `004b29b0`, `cdecl`, three words) and
/// `TES::AddTempDebugObject` (engine map, `00458e20`, `thiscall`, two
/// words) on `TES` (`011dea10`).
const MAKE_TRI_POINT: u32 = 0x004b_29b0;
const ADD_TEMP_DEBUG_OBJECT: u32 = 0x0045_8e20;
const TES: u32 = 0x011d_ea10;
/// The floats 5.0 (`0101712c`) and 50.0 (`0101b268`) the debug marker is
/// made with.
const MARKER_SIZE: u32 = 0x0101_712c;
const MARKER_TIME: u32 = 0x0101_b268;
/// `009185e0` (`thiscall`: the word at +0x94) and
/// `MagicCaster::GetCasterAsActor` (engine map, `00815410`).
const WORD_AT_94_009185E0: u32 = 0x0091_85e0;
const GET_CASTER_AS_ACTOR: u32 = 0x0081_5410;
/// `1.0f` and `0.0f` as `FLD1` / `FLDZ` push them.
const ONE_BITS: u32 = 0x3f80_0000;
const ZERO_BITS: u32 = 0;
/// The `bhkTelekinesisListener` vtables (`01079744`, `01079728`,
/// `01079718`, RTTI `.?AVbhkTelekinesisListener@@`).
const TELEKINESIS_LISTENER_VTABLE: u32 = 0x0107_9744;
const TELEKINESIS_LISTENER_VTABLE_4: u32 = 0x0107_9728;
const TELEKINESIS_LISTENER_VTABLE_8: u32 = 0x0107_9718;
/// The base and member constructors `006234b0`, `0061f580`, `006c0100`
/// and `008c71b0` (`thiscall`, one word), and the destructors `006234d0`,
/// `00554650`, `006c0150` (`thiscall`).
const BASE_CONSTRUCT_006234B0: u32 = 0x0062_34b0;
const BASE_4_CONSTRUCT_0061F580: u32 = 0x0061_f580;
const BASE_8_CONSTRUCT_006C0100: u32 = 0x006c_0100;
const MEMBER_14_CONSTRUCT_008C71B0: u32 = 0x008c_71b0;
const BASE_DESTRUCT_006234D0: u32 = 0x0062_34d0;
const BASE_4_DESTRUCT_00554650: u32 = 0x0055_4650;
const BASE_8_DESTRUCT_006C0150: u32 = 0x006c_0150;
/// `006c1da0` and `006c1dd0` (`thiscall`, one word) on the world at +0x10.
const ADD_LISTENER_006C1DA0: u32 = 0x006c_1da0;
const ADD_LISTENER_006C1DD0: u32 = 0x006c_1dd0;
/// `bhkCharacterProxy::operatorP` (engine map, `004ae750`, `thiscall`) and
/// `00c9c540` (`thiscall`, one word).
const CHARACTER_PROXY_OPERATOR_P: u32 = 0x004a_e750;
const PROXY_ADD_00C9C540: u32 = 0x00c9_c540;
/// The free `00401030` (`cdecl`, one word).
const FREE: u32 = 0x0040_1030;
/// `00807cb0` (`cdecl`, two words).
const LISTENER_CALLBACK_00807CB0: u32 = 0x0080_7cb0;
/// `bhkTelekinesisListener::Finish` (engine map, `00807a10`, `thiscall`,
/// one word).
const TELEKINESIS_LISTENER_FINISH: u32 = 0x0080_7a10;

// Translated from 00807730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`; `this` is not read): the reference of the
/// 3D object of the word (`bhkUtilFunctions::GetNiAVObject`, then
/// `TESObjectREFR::FindReferenceFor3D` when there is one), or 0.
pub fn fn_00807730(e: &mut Engine, _this: Ptr, body: u32) -> u32 {
    let object = e.call(GET_NI_AV_OBJECT, &args![body]).u32();
    if object != 0 {
        return e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
    }
    0
}

/// The debug node of one side of the contact (`008070d0`): `holder` is the
/// local word the code reads the object from (`[EBP-0x90]` or
/// `[EBP-0x68]`), `scale`, `rows` and `point` the locals it fills.
fn draw_contact_side(e: &mut Engine, holder: u32, scale: u32, rows: u32, point: u32) {
    let object = e.mem.u32(holder);
    let held = e.call(WORD_AT_0_00559450, &args![object]).u32();
    let parent = e.call(NODE_OF_004A3A40, &args![held]).u32();
    if parent == 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NODE_CONSTRUCT_00A5ECB0, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.call(
        SET_THREE_00416870,
        &args![scale, ONE_BITS, ONE_BITS, ONE_BITS],
    );
    e.vcall(parent, 0xd4, &args![node, scale]);
    e.call(SELF_006815C0, &args![rows]);
    let object = e.mem.u32(holder);
    let transform = e.call(WORD_AT_8_0044DDC0, &args![object]).u32();
    let source = e.call(SELF_006815C0, &args![transform]).u32();
    e.call(COPY_ROWS_00624B10, &args![rows, source]);
    e.call(SET_ROTATE_0043FA80, &args![node, rows]);
    e.call(SELF_006815C0, &args![point]);
    let object = e.mem.u32(holder);
    let transform = e.call(WORD_AT_8_0044DDC0, &args![object]).u32();
    let source = e.call(PLUS_30_00436AA0, &args![transform]).u32();
    e.call(COPY_POINT_00458620, &args![point, source]);
    e.call(SET_TRANSLATE_00440460, &args![node, point]);
}

// Translated from 008070d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a contact record; `this` is the listener
/// `00806eb0` builds, its owner at +0x650): handles a contact of the
/// character proxy.
///
/// The caster is the reference of the contact's word +0x20 cast to
/// `MagicProjectile` (`__RTDynamicCast`), else the owner at +0x650; when
/// its slot +0x30C answers false nothing happens. The other side is the
/// reference of the word +0x48 (`hit`), an actor when its slot +0x100
/// answers true. When `004b5950` answers non-zero for the word +0x48, a
/// missing actor, an actor whose slot +0x22C (with 0) answers true, a
/// saved acquire object whose slot +0x40C is neither 0 nor 6, or an actor
/// whose slot +0x214 is 4 or 9 end it. So do the player's word +0x638
/// being `hit`, and `008ace90` on the actor answering true.
///
/// With `bDrawSpellContact:General` set, a node is put under each side's
/// 3D object at that object's rotation and position (`draw_contact_side`).
/// The contact point (the record's first three words) is copied; with the
/// setting set a marker is added there (`MakeTriPoint` with 5.0,
/// `TES::AddTempDebugObject` for 50.0). Then the caster's word +0x94 and,
/// on it, `MagicCaster::GetCasterAsActor`, the slot +0x100 of `hit` again
/// (both answers unused), and the caster's slot +0x304 with the point, the
/// object `004b5a80` gives for `004b59f0`'s answer (or 0), `hit` and 0.
///
/// The stack frame is modelled as a block of the frame's size, the locals
/// at their `EBP` offsets.
// Translation incomplete: 008070d0 - the FS:[0] exception frame and the /GS stack cookie check (00ec408c) are not translated
pub fn fn_008070d0(e: &mut Engine, this: Ptr, contact: u32) {
    let first = e.mem.u32(contact.wrapping_add(0x20));
    let object = fn_00807730(e, this, first);
    let mut caster = e
        .call(
            DYNAMIC_CAST,
            &args![
                object,
                0u32,
                TYPE_TES_OBJECT_REFR,
                TYPE_MAGIC_PROJECTILE,
                0u32
            ],
        )
        .u32();
    if caster == 0 {
        caster = e.mem.u32(this.addr().wrapping_add(0x650));
    }
    if !e.vcall(caster, 0x30c, &[]).bool() {
        return;
    }
    let second = e.mem.u32(contact.wrapping_add(0x48));
    let hit = fn_00807730(e, this, second);
    let actor = if hit != 0 && e.vcall(hit, 0x100, &[]).bool() {
        hit
    } else {
        0
    };
    let second = e.mem.u32(contact.wrapping_add(0x48));
    if e.call(CONTACT_TEST_004B5950, &args![second]).u32() != 0 {
        if actor == 0 {
            return;
        }
        if e.vcall(actor, 0x22c, &args![0u32]).bool() {
            return;
        }
        if e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32() != 0 {
            let acquired = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
            if e.vcall(acquired, 0x40c, &[]).u32() != 0 {
                let acquired = e.call(SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
                if e.vcall(acquired, 0x40c, &[]).u32() != 6 {
                    return;
                }
            }
        }
        if e.vcall(actor, 0x214, &[]).u32() == 4 {
            return;
        }
        if e.vcall(actor, 0x214, &[]).u32() == 9 {
            return;
        }
    }
    if hit != 0 {
        let player = e.mem.u32(PLAYER);
        if hit == e.call(PLAYER_WORD_638_0089F4E0, &args![player]).u32() {
            return;
        }
    }
    if actor != 0 && e.call(ACTOR_TEST_008ACE90, &args![actor]).bool() {
        return;
    }
    e.with_stack(0x190, |e, frame| {
        let ebp = frame.addr().wrapping_add(0x190);
        let local = |offset: u32| ebp.wrapping_sub(offset);
        let draw = e.call(SETTING_VALUE, &args![DRAW_SPELL_CONTACT]).u32();
        if e.mem.u8(draw) != 0 {
            e.call(SELF_006815C0, &args![local(0x40)]);
            e.call(COPY_CONTACT_00632D40, &args![local(0xb0), contact]);
            draw_contact_side(e, local(0x90), local(0xd0), local(0xf4), local(0x100));
            draw_contact_side(e, local(0x68), local(0x120), local(0x144), local(0x150));
        }
        let point = local(0x15c);
        e.call(SELF_006815C0, &args![point]);
        let source = e.call(SELF_006815C0, &args![contact]).u32();
        e.call(COPY_POINT_00458620, &args![point, source]);
        let draw = e.call(SETTING_VALUE, &args![DRAW_SPELL_CONTACT]).u32();
        if e.mem.u8(draw) != 0 {
            let colour = e
                .call(
                    SET_FOUR_00414430,
                    &args![local(0x16c), ONE_BITS, ZERO_BITS, ZERO_BITS, ONE_BITS],
                )
                .u32();
            let size = x87_quiet_bits(e.global::<u32>(MARKER_SIZE));
            let marker = e.call(MAKE_TRI_POINT, &args![size, colour, 1u32]).u32();
            e.call(SET_TRANSLATE_00440460, &args![marker, point]);
            let time = x87_quiet_bits(e.global::<u32>(MARKER_TIME));
            let tes = e.mem.u32(TES);
            e.call(ADD_TEMP_DEBUG_OBJECT, &args![tes, marker, time]);
        }
        let target = e.call(WORD_AT_94_009185E0, &args![caster]).u32();
        if hit != 0 {
            e.vcall(hit, 0x100, &[]);
        }
        if target != 0 {
            e.call(GET_CASTER_AS_ACTOR, &args![target]);
        }
        let second = e.mem.u32(contact.wrapping_add(0x48));
        let found = e.call(CONTACT_OBJECT_004B59F0, &args![second]).u32();
        let source = if found != 0 {
            e.call(CONTACT_OBJECT_004B5A80, &args![found]).u32()
        } else {
            0
        };
        let x = e.mem.u32(point);
        let y = e.mem.u32(point.wrapping_add(4));
        let z = e.mem.u32(point.wrapping_add(8));
        e.vcall(caster, 0x304, &args![x, y, z, source, hit, 0u32]);
    });
}

// Translated from 00807790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the class whose vtables are `01079744`, `01079728`
/// and `01079718` (RTTI `bhkTelekinesisListener`; `thiscall`, `RET 0xC`):
/// constructs the bases (`006234b0` on `this`, `0061f580` on +4,
/// `006c0100` on +8), stores the three vtables, constructs +0x14
/// (`008c71b0` with 0), stores the three words at +0xC, +0x10 and +0x14,
/// clears the byte +0x18, and registers the three bases with the word at
/// +0x10 (`008078b0` with `this`, `006c1da0` with `this + 4`, `006c1dd0`
/// with `this + 8`; a null `this` would pass 0). Returns `this`.
// Translation incomplete: 00807790 - the FS:[0] exception frame (unwinding the bases on a throw) is not translated
pub fn fn_00807790(e: &mut Engine, this: Ptr, first: u32, world: u32, third: u32) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_006234B0, &args![base]);
    e.call(BASE_4_CONSTRUCT_0061F580, &args![base.wrapping_add(4)]);
    e.call(BASE_8_CONSTRUCT_006C0100, &args![base.wrapping_add(8)]);
    e.mem.set_u32(base, TELEKINESIS_LISTENER_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(4), TELEKINESIS_LISTENER_VTABLE_4);
    e.mem
        .set_u32(base.wrapping_add(8), TELEKINESIS_LISTENER_VTABLE_8);
    e.call(
        MEMBER_14_CONSTRUCT_008C71B0,
        &args![base.wrapping_add(0x14), 0u32],
    );
    e.mem.set_u32(base.wrapping_add(0xc), first);
    e.mem.set_u32(base.wrapping_add(0x10), world);
    e.mem.set_u32(base.wrapping_add(0x14), third);
    e.mem.set_u8(base.wrapping_add(0x18), 0);
    let world = e.mem.u32(base.wrapping_add(0x10));
    fn_008078b0(e, Ptr::new(world), base);
    let second_base = if base != 0 { base.wrapping_add(4) } else { 0 };
    let world = e.mem.u32(base.wrapping_add(0x10));
    e.call(ADD_LISTENER_006C1DA0, &args![world, second_base]);
    let third_base = if base != 0 { base.wrapping_add(8) } else { 0 };
    let world = e.mem.u32(base.wrapping_add(0x10));
    e.call(ADD_LISTENER_006C1DD0, &args![world, third_base]);
    this
}

// Translated from 008078b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): when `bhkCharacterProxy::operatorP`
/// (`004ae750`) answers an object for `this`, calls `00c9c540` on it with
/// the word.
pub fn fn_008078b0(e: &mut Engine, this: Ptr, listener: u32) {
    let proxy = e.call(CHARACTER_PROXY_OPERATOR_P, &args![this]).u32();
    if proxy != 0 {
        e.call(PROXY_ADD_00C9C540, &args![proxy, listener]);
    }
}

// Translated from 008078e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the `bhkTelekinesisListener` class
/// (`thiscall`, `RET 4`: the flags): runs the destructor body
/// (`00807910`) and frees `this` (`00401030`) when bit 0 is set. Returns
/// `this`.
pub fn fn_008078e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00807910(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00807910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of the `bhkTelekinesisListener` class (`thiscall`):
/// destroys the bases in reverse order (`006c0150` on +8, `00554650` on
/// +4, `006234d0` on `this`).
// Translation incomplete: 00807910 - the FS:[0] exception frame (unwinding the bases on a throw) is not translated
pub fn fn_00807910(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(BASE_8_DESTRUCT_006C0150, &args![base.wrapping_add(8)]);
    e.call(BASE_4_DESTRUCT_00554650, &args![base.wrapping_add(4)]);
    e.call(BASE_DESTRUCT_006234D0, &args![base]);
}

// Translated from 00807980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the base at +4 of an object, `RET 4`): calls
/// `00807cb0` (`cdecl`) with the object (`this - 4`) and the word, then,
/// when the object is not null, its slot +0xC with 1 (the answer is not
/// used).
pub fn fn_00807980(e: &mut Engine, this: Ptr, argument: u32) {
    let object = this.addr().wrapping_sub(4);
    e.call(LISTENER_CALLBACK_00807CB0, &args![object, argument]);
    if object != 0 {
        e.vcall(object, 0xc, &args![1u32]);
    }
}

// Translated from 008079e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the base at +8 of a `bhkTelekinesisListener`,
/// `RET 4`): when the byte +0x10 (the object's +0x18) is set, calls
/// `bhkTelekinesisListener::Finish` (`00807a10`) on the object
/// (`this - 8`) with the word.
pub fn fn_008079e0(e: &mut Engine, this: Ptr, argument: u32) {
    if e.mem.u8(this.addr().wrapping_add(0x10)) != 0 {
        e.call(
            TELEKINESIS_LISTENER_FINISH,
            &args![this.addr().wrapping_sub(8), argument],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_7770, fn_00807770(Ptr, u32) -> u32),
        entry!(0x0080_70d0, fn_008070d0(Ptr, u32)),
        entry!(0x0080_7730, fn_00807730(Ptr, u32) -> u32),
        entry!(0x0080_7790, fn_00807790(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0080_78b0, fn_008078b0(Ptr, u32)),
        entry!(0x0080_78e0, fn_008078e0(Ptr, u32) -> Ptr),
        entry!(0x0080_7910, fn_00807910(Ptr)),
        entry!(0x0080_7980, fn_00807980(Ptr, u32)),
        entry!(0x0080_79e0, fn_008079e0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_0x70_byte_elements() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0010_0000);
        assert_eq!(e.call(0x0080_7770, &[this, 0]).u32(), 0x0010_0000);
        assert_eq!(e.call(0x0080_7770, &[this, 3]).u32(), 0x0010_0150);
        assert_eq!(e.call(0x0080_7770, &[this, 0xffff_ffff]).u32(), 0x000f_ff90);
    }
}

#[cfg(test)]
mod tests_n048 {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn zero(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    #[test]
    fn reference_of_a_body_needs_its_3d() {
        let mut e = Engine::new();
        e.register(GET_NI_AV_OBJECT, |_, a| {
            ret(if a[0] == 7 { 0x70 } else { 0 })
        });
        e.register(FIND_REFERENCE_FOR_3D, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x0080_7730, &args![0u32, 7u32]).u32(), 0x71);
        assert_eq!(e.call(0x0080_7730, &args![0u32, 8u32]).u32(), 0);
    }

    /// A contact whose +0x20 body gives the caster `0x6000` (through the
    /// cast) and whose +0x48 body gives the actor `0x7000`.
    fn contact_world(e: &mut Engine) -> (u32, u32) {
        // Caster vtable: +0x30C answers 1 (code 0x00b0_0001), +0x304 is recorded.
        e.put_vtable(0x00b1_0000, &[0; 0x310 / 4]);
        e.mem.set_u32(0x00b1_0000 + 0x30c, 0x00b0_0001);
        e.mem.set_u32(0x00b1_0000 + 0x304, 0x00b0_0002);
        e.mem.set_u32(0x00b1_0000 + 0x100, 0x00b0_0001);
        e.mem.set_u32(0x00b1_0000 + 0x22c, 0x00b0_0000);
        e.mem.set_u32(0x00b1_0000 + 0x214, 0x00b0_0003);
        e.register(0x00b0_0001, |_, _| ret(1));
        e.register(0x00b0_0000, |_, _| ret(0));
        e.register(0x00b0_0002, |_, _| Ret::default());
        e.register(0x00b0_0003, |_, _| ret(2));
        let caster = e.mem.alloc(0x100);
        e.mem.set_u32(caster, 0x00b1_0000);
        let actor = e.mem.alloc(0x100);
        e.mem.set_u32(actor, 0x00b1_0000);
        e.register(GET_NI_AV_OBJECT, |_, a| ret(a[0]));
        e.register(FIND_REFERENCE_FOR_3D, |_, a| ret(a[0]));
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(CONTACT_TEST_004B5950, |_, _| ret(0));
        e.register(CONTACT_OBJECT_004B59F0, |_, _| ret(0x55));
        e.register(CONTACT_OBJECT_004B5A80, |_, a| ret(a[0] + 1));
        e.register(PLAYER_WORD_638_0089F4E0, |_, _| ret(0x1234));
        e.register(ACTOR_TEST_008ACE90, |_, _| ret(0));
        e.register(SELF_006815C0, |_, a| ret(a[0]));
        e.register(COPY_POINT_00458620, |e, a| {
            for i in 0..3 {
                let w = e.mem.u32(a[1] + i * 4);
                e.mem.set_u32(a[0] + i * 4, w);
            }
            Ret::default()
        });
        e.register(WORD_AT_94_009185E0, |_, _| ret(0x99));
        zero(e, &[GET_CASTER_AS_ACTOR]);
        let setting = e.mem.alloc(4);
        e.register_double(SETTING_VALUE, move |_, _| ret(setting));
        e.map(PLAYER, 4);
        e.mem.set_u32(PLAYER, 0x4000);
        let contact = e.mem.alloc(0x70);
        e.mem.set_u32(contact, 1.0f32.to_bits());
        e.mem.set_u32(contact + 4, 2.0f32.to_bits());
        e.mem.set_u32(contact + 8, 3.0f32.to_bits());
        e.mem.set_u32(contact + 0x20, caster);
        e.mem.set_u32(contact + 0x48, actor);
        (contact, caster)
    }

    #[test]
    fn contact_reports_the_point_to_the_caster() {
        let mut e = Engine::new();
        let (contact, caster) = contact_world(&mut e);
        let actor = e.mem.u32(contact + 0x48);
        let listener = e.mem.alloc(0x654);
        let (_, log) = run(&mut e, 0x0080_70d0, &args![listener, contact]);
        let reports = calls(&log, 0x00b0_0002);
        assert_eq!(
            reports,
            vec![vec![
                caster,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0x56,
                actor,
                0
            ]]
        );
        assert_eq!(calls(&log, GET_CASTER_AS_ACTOR), vec![vec![0x99]]);
        assert!(calls(&log, ADD_TEMP_DEBUG_OBJECT).is_empty());
        // The caster saying no ends it.
        e.mem.set_u32(0x00b1_0000 + 0x30c, 0x00b0_0000);
        let (_, log) = run(&mut e, 0x0080_70d0, &args![listener, contact]);
        assert!(calls(&log, 0x00b0_0002).is_empty());
    }

    #[test]
    fn contact_falls_back_on_the_owner_and_skips_the_player_target() {
        let mut e = Engine::new();
        let (contact, caster) = contact_world(&mut e);
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        let listener = e.mem.alloc(0x654);
        e.mem.set_u32(listener + 0x650, caster);
        let actor = e.mem.u32(contact + 0x48);
        e.register_double(PLAYER_WORD_638_0089F4E0, move |_, _| ret(actor));
        let (_, log) = run(&mut e, 0x0080_70d0, &args![listener, contact]);
        assert!(calls(&log, 0x00b0_0002).is_empty());
        // Not the player's: the owner reports.
        e.register(PLAYER_WORD_638_0089F4E0, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0080_70d0, &args![listener, contact]);
        assert_eq!(calls(&log, 0x00b0_0002).len(), 1);
        assert_eq!(calls(&log, 0x00b0_0002)[0][0], caster);
    }

    #[test]
    fn contact_with_004b5950_checks_the_actor() {
        let mut e = Engine::new();
        let (contact, _) = contact_world(&mut e);
        e.register(CONTACT_TEST_004B5950, |_, _| ret(1));
        e.register(SAVED_ACQUIRE_OBJECT, |_, _| ret(0));
        let listener = e.mem.alloc(0x654);
        let (_, log) = run(&mut e, 0x0080_70d0, &args![listener, contact]);
        assert_eq!(calls(&log, 0x00b0_0002).len(), 1);
        // Slot +0x214 answering 9 ends it.
        e.register(0x00b0_0003, |_, _| ret(9));
        let (_, log) = run(&mut e, 0x0080_70d0, &args![listener, contact]);
        assert!(calls(&log, 0x00b0_0002).is_empty());
        // A saved acquire object whose slot +0x40C answers 5 ends it.
        e.register(0x00b0_0003, |_, _| ret(2));
        let acquired = e.mem.alloc(4);
        e.put_vtable(0x00b2_0000, &[0; 0x410 / 4]);
        e.mem.set_u32(0x00b2_0000 + 0x40c, 0x00b0_0004);
        e.mem.set_u32(acquired, 0x00b2_0000);
        e.register_double(SAVED_ACQUIRE_OBJECT, move |_, _| ret(acquired));
        e.register(0x00b0_0004, |_, _| ret(5));
        let (_, log) = run(&mut e, 0x0080_70d0, &args![listener, contact]);
        assert!(calls(&log, 0x00b0_0002).is_empty());
        e.register(0x00b0_0004, |_, _| ret(6));
        let (_, log) = run(&mut e, 0x0080_70d0, &args![listener, contact]);
        assert_eq!(calls(&log, 0x00b0_0002).len(), 1);
    }

    #[test]
    fn contact_draws_with_the_setting() {
        let mut e = Engine::new();
        let (contact, _) = contact_world(&mut e);
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, 1);
        e.register_double(SETTING_VALUE, move |_, _| ret(setting));
        e.map(MARKER_SIZE, 4);
        e.set_global::<u32>(MARKER_SIZE, 5.0f32.to_bits());
        e.map(MARKER_TIME, 4);
        e.set_global::<u32>(MARKER_TIME, 50.0f32.to_bits());
        e.map(TES, 4);
        e.mem.set_u32(TES, 0x7e5);
        // No parent node on either side: nothing attached.
        e.register(WORD_AT_0_00559450, |_, _| ret(0));
        e.register(NODE_OF_004A3A40, |_, _| ret(0));
        zero(
            &mut e,
            &[
                COPY_CONTACT_00632D40,
                SET_TRANSLATE_00440460,
                ADD_TEMP_DEBUG_OBJECT,
            ],
        );
        e.register(SET_FOUR_00414430, |_, a| ret(a[0]));
        e.register(MAKE_TRI_POINT, |_, _| ret(0x3a3a));
        let listener = e.mem.alloc(0x654);
        let (_, log) = run(&mut e, 0x0080_70d0, &args![listener, contact]);
        assert_eq!(calls(&log, COPY_CONTACT_00632D40).len(), 1);
        assert!(calls(&log, ALLOCATE).is_empty());
        let four = calls(&log, SET_FOUR_00414430);
        assert_eq!(four[0][1..], [ONE_BITS, 0, 0, ONE_BITS]);
        let tri = calls(&log, MAKE_TRI_POINT);
        assert_eq!(tri, vec![vec![5.0f32.to_bits(), four[0][0], 1]]);
        assert_eq!(
            calls(&log, ADD_TEMP_DEBUG_OBJECT),
            vec![vec![0x7e5, 0x3a3a, 50.0f32.to_bits()]]
        );
        // A parent node on the first side: a new node under it.
        e.register(WORD_AT_0_00559450, |_, a| ret(a[0]));
        e.register(NODE_OF_004A3A40, |_, a| ret(a[0]));
        e.register(COPY_CONTACT_00632D40, |e, a| {
            e.mem.set_u32(a[0] + 0x20, 0x00c1_0000);
            Ret::default()
        });
        e.map(0x00c1_0000, 4);
        e.mem.set_u32(0x00c1_0000, 0x00b3_0000);
        e.put_vtable(0x00b3_0000, &[0; 0xd8 / 4]);
        e.mem.set_u32(0x00b3_0000 + 0xd4, 0x00b0_0005);
        e.register(0x00b0_0005, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| ret(0x00c2_0000));
        e.register(NODE_CONSTRUCT_00A5ECB0, |_, a| ret(a[0] + 0x10));
        zero(
            &mut e,
            &[
                SET_THREE_00416870,
                WORD_AT_8_0044DDC0,
                COPY_ROWS_00624B10,
                SET_ROTATE_0043FA80,
            ],
        );
        let translate = e.mem.alloc(12);
        e.register_double(PLUS_30_00436AA0, move |_, _| ret(translate));
        let (_, log) = run(&mut e, 0x0080_70d0, &args![listener, contact]);
        assert_eq!(calls(&log, ALLOCATE), vec![vec![0xac]]);
        let attach = calls(&log, 0x00b0_0005);
        assert_eq!(attach.len(), 1);
        assert_eq!(attach[0][0], 0x00c1_0000);
        assert_eq!(attach[0][1], 0x00c2_0010);
        assert_eq!(calls(&log, SET_ROTATE_0043FA80)[0][0], 0x00c2_0010);
    }

    #[test]
    fn telekinesis_listener_constructor_registers_its_bases() {
        let mut e = Engine::new();
        zero(
            &mut e,
            &[
                BASE_CONSTRUCT_006234B0,
                BASE_4_CONSTRUCT_0061F580,
                BASE_8_CONSTRUCT_006C0100,
                MEMBER_14_CONSTRUCT_008C71B0,
                ADD_LISTENER_006C1DA0,
                ADD_LISTENER_006C1DD0,
                PROXY_ADD_00C9C540,
            ],
        );
        e.register(CHARACTER_PROXY_OPERATOR_P, |_, a| ret(a[0] + 1));
        let this = e.mem.alloc(0x1c);
        e.mem.set_u8(this + 0x18, 9);
        let (r, log) = run(&mut e, 0x0080_7790, &args![this, 0xau32, 0xb0u32, 0xcu32]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), TELEKINESIS_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 4), TELEKINESIS_LISTENER_VTABLE_4);
        assert_eq!(e.mem.u32(this + 8), TELEKINESIS_LISTENER_VTABLE_8);
        assert_eq!(e.mem.u32(this + 0xc), 0xa);
        assert_eq!(e.mem.u32(this + 0x10), 0xb0);
        assert_eq!(e.mem.u32(this + 0x14), 0xc);
        assert_eq!(e.mem.u8(this + 0x18), 0);
        assert_eq!(
            calls(&log, MEMBER_14_CONSTRUCT_008C71B0),
            vec![vec![this + 0x14, 0]]
        );
        assert_eq!(calls(&log, PROXY_ADD_00C9C540), vec![vec![0xb1, this]]);
        assert_eq!(
            calls(&log, ADD_LISTENER_006C1DA0),
            vec![vec![0xb0, this + 4]]
        );
        assert_eq!(
            calls(&log, ADD_LISTENER_006C1DD0),
            vec![vec![0xb0, this + 8]]
        );
    }

    #[test]
    fn proxy_add_needs_a_proxy() {
        let mut e = Engine::new();
        zero(&mut e, &[PROXY_ADD_00C9C540, CHARACTER_PROXY_OPERATOR_P]);
        let (_, log) = run(&mut e, 0x0080_78b0, &args![0x10u32, 0x20u32]);
        assert!(calls(&log, PROXY_ADD_00C9C540).is_empty());
        e.register(CHARACTER_PROXY_OPERATOR_P, |_, _| ret(0x30));
        let (_, log) = run(&mut e, 0x0080_78b0, &args![0x10u32, 0x20u32]);
        assert_eq!(calls(&log, PROXY_ADD_00C9C540), vec![vec![0x30, 0x20]]);
    }

    #[test]
    fn telekinesis_listener_destructors() {
        let mut e = Engine::new();
        zero(
            &mut e,
            &[
                BASE_8_DESTRUCT_006C0150,
                BASE_4_DESTRUCT_00554650,
                BASE_DESTRUCT_006234D0,
                FREE,
            ],
        );
        let (_, log) = run(&mut e, 0x0080_7910, &args![0x100u32]);
        let order: Vec<u32> = log.iter().skip(1).map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                BASE_8_DESTRUCT_006C0150,
                BASE_4_DESTRUCT_00554650,
                BASE_DESTRUCT_006234D0
            ]
        );
        assert_eq!(calls(&log, BASE_8_DESTRUCT_006C0150), vec![vec![0x108]]);
        let (r, log) = run(&mut e, 0x0080_78e0, &args![0x100u32, 3u32]);
        assert_eq!(r.u32(), 0x100);
        assert_eq!(calls(&log, FREE), vec![vec![0x100]]);
        let (_, log) = run(&mut e, 0x0080_78e0, &args![0x100u32, 2u32]);
        assert!(calls(&log, FREE).is_empty());
        assert_eq!(calls(&log, BASE_DESTRUCT_006234D0).len(), 1);
    }

    #[test]
    fn base_4_callback_then_slot_0xc() {
        let mut e = Engine::new();
        zero(&mut e, &[LISTENER_CALLBACK_00807CB0, 0x00b0_0006]);
        let object = e.mem.alloc(8);
        e.put_vtable(0x00b4_0000, &[0, 0, 0, 0x00b0_0006]);
        e.mem.set_u32(object, 0x00b4_0000);
        let (_, log) = run(&mut e, 0x0080_7980, &args![object + 4, 0x77u32]);
        assert_eq!(
            calls(&log, LISTENER_CALLBACK_00807CB0),
            vec![vec![object, 0x77]]
        );
        assert_eq!(calls(&log, 0x00b0_0006), vec![vec![object, 1]]);
        // `this` 4: the object is null, no virtual call.
        let (_, log) = run(&mut e, 0x0080_7980, &args![4u32, 0x77u32]);
        assert_eq!(calls(&log, LISTENER_CALLBACK_00807CB0), vec![vec![0, 0x77]]);
        assert!(calls(&log, 0x00b0_0006).is_empty());
    }

    #[test]
    fn base_8_finishes_when_flagged() {
        let mut e = Engine::new();
        zero(&mut e, &[TELEKINESIS_LISTENER_FINISH]);
        let object = e.mem.alloc(0x20);
        let (_, log) = run(&mut e, 0x0080_79e0, &args![object + 8, 5u32]);
        assert!(calls(&log, TELEKINESIS_LISTENER_FINISH).is_empty());
        e.mem.set_u8(object + 0x18, 1);
        let (_, log) = run(&mut e, 0x0080_79e0, &args![object + 8, 5u32]);
        assert_eq!(
            calls(&log, TELEKINESIS_LISTENER_FINISH),
            vec![vec![object, 5]]
        );
    }
}
