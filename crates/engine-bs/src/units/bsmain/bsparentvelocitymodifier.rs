//! `BSMain/bsparentvelocitymodifier.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31650` (named `NiPSysModifier::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C31650: u32 = 0x00c3_1650;

// Translated from 00c63930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSParentVelocityModifier` vtable word 010c384c, right after
/// `BSParentVelocityModifier::LoadBinary` (010c3848). Forwards `this` and the stream to
/// `00c31650` (`NiPSysModifier::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c63930(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C31650, &args![this, stream]);
}

/// `00c31670` (named `NiPSysModifier::RegisterStreamables` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00C31670: u32 = 0x00c3_1670;

// Translated from 00c63950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSParentVelocityModifier` vtable word 010c3850, after the
/// `LinkObject` word 010c384c and before `BSParentVelocityModifier::SaveBinary` (010c3854).
/// Forwards `this` and the stream to `00c31670` (`NiPSysModifier::RegisterStreamables`).
/// Returns the `bool` in `AL`: Xbox PDB `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z`
/// returns `_N`; `stream` is the `NiStream` reference as an address. Parameter count from the
/// `RET 4` of both bodies.
pub fn fn_00c63950(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00C31670, &args![this, stream])
        .bool()
}

// Translated from 00c63aa0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043c8 without reading the receiver. The map names this body `BSParentVelocityModifier::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c63aa0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43c8
}

/// The `NiPSysModifier` constructor (`00c31720`), clone-member copy
/// (`00c31780`: `this`, the clone and the cloning process), `LoadBinary`
/// (`00c318b0`) and destructor (`00c31750`).
const NI_PSYS_MODIFIER_CONSTRUCT: u32 = 0x00c3_1720;
const NI_PSYS_MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const NI_PSYS_MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const NI_PSYS_MODIFIER_DESTRUCT: u32 = 0x00c3_1750;
/// The Gamebryo allocator (`00aa13e0`, cdecl, the size) and the sized
/// delete (`00aa1460`, cdecl: the block and the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// Reads a float from the stream (`00a26df0`, cdecl: the stream and the
/// address).
const STREAM_READ_FLOAT: u32 = 0x00a2_6df0;
/// The `BSParentVelocityModifier` vtable (its RTTI locator names
/// `.?AVBSParentVelocityModifier@@`) and the object size.
const BS_PARENT_VELOCITY_MODIFIER_VTABLE: u32 = 0x010c_37fc;
const BS_PARENT_VELOCITY_MODIFIER_SIZE: u32 = 0x3c;
/// The point the two vectors start as (three words at `011f426c`).
const START_POINT: u32 = 0x011f_426c;
/// The float the class reads and copies (+0x20).
const DAMPING: u32 = 0x20;

// Translated from 00c63330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSParentVelocityModifier` constructor: the `NiPSysModifier` part,
/// the class's vtable, the word at +0xC 3000, the two vectors at +0x24 and
/// +0x30 the start point, the floats at +0x20 and +0x18 0.0. Returns
/// `this`.
pub fn fn_00c63330(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_PSYS_MODIFIER_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, BS_PARENT_VELOCITY_MODIFIER_VTABLE);
    e.mem.set_u32(base + 0xc, 3000);
    for vector in [0x24u32, 0x30] {
        for word in 0..3 {
            let value = e.mem.u32(START_POINT + 4 * word);
            e.mem.set_u32(base + vector + 4 * word, value);
        }
    }
    e.mem.set_f32(base + DAMPING, 0.0);
    e.mem.set_f32(base + 0x18, 0.0);
    this
}

// Translated from 00c63880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c63880 - C++ exception unwinding (FS:[0] frame) not translated
/// `BSParentVelocityModifier::CreateObject` (the class's stream factory): a
/// new 0x3C-byte object from the Gamebryo allocator, constructed; null when
/// the allocation fails.
pub fn fn_00c63880(e: &mut Engine) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![BS_PARENT_VELOCITY_MODIFIER_SIZE])
        .u32();
    if object == 0 {
        return Ptr::NULL;
    }
    fn_00c63330(e, Ptr::new(object))
}

// Translated from 00c63840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::CopyMembers` (by its use in `CreateClone`):
/// the `NiPSysModifier` part, then the float at +0x20 (two `FLD`/`FSTP`
/// pairs).
pub fn fn_00c63840(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(NI_PSYS_MODIFIER_COPY_MEMBERS, &args![this, clone, cloning]);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + DAMPING));
    e.mem.set_u32(clone.addr() + DAMPING, value);
}

// Translated from 00c637b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c637b0 - C++ exception unwinding (FS:[0] frame) not translated
/// `BSParentVelocityModifier::CreateClone` (Xbox PDB): a new object as
/// `CreateObject` makes it (null when the allocation fails, used as is),
/// then the members copied into it (`00c63840`).
pub fn bs_parent_velocity_modifier_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = fn_00c63880(e);
    fn_00c63840(e, this, clone, cloning);
    clone
}

// Translated from 00c63900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::LoadBinary` (Xbox PDB): the `NiPSysModifier`
/// part, then the float at +0x20 (`00a26df0`).
pub fn bs_parent_velocity_modifier_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_PSYS_MODIFIER_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_READ_FLOAT, &args![stream, this.addr() + DAMPING]);
}

// Translated from 00c63a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::_scalar_deleting_destructor_` (Xbox PDB): the
/// `NiPSysModifier` destructor, then the sized delete of 0x3C bytes when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bs_parent_velocity_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(NI_PSYS_MODIFIER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, BS_PARENT_VELOCITY_MODIFIER_SIZE]);
    }
    this
}

// ---- Batch n067 ---------------------------------------------------------------------------
//
// The class's fields (Xbox PDB names, PC offsets): `fLastTimeDelta` +0x18, `fTimeLast` +0x1c,
// `fDamping` +0x20 (`DAMPING` above), `pntLast` +0x24 and `velLast` +0x30 (points); the
// `NiPSysModifier` base holds `m_pkTarget` (the particle system) at +0x10.

/// `fLastTimeDelta`, `fTimeLast`, `pntLast` and `velLast` (Xbox PDB, PC offsets) and
/// `NiPSysModifier::m_pkTarget` (+0x10).
const LAST_TIME_DELTA: u32 = 0x18;
const TIME_LAST: u32 = 0x1c;
const POINT_LAST: u32 = 0x24;
const VELOCITY_LAST: u32 = 0x30;
const TARGET: u32 = 0x10;
/// `NiPSysModifier::SaveBinary`, `IsEqual` and `GetViewerStrings` (engine map).
const NI_PSYS_MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const NI_PSYS_MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const NI_PSYS_MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// Writes a float through the stream (`NiStreamSaveBinary<float>`, cdecl: the stream, the
/// float's address).
const STREAM_SAVE_FLOAT: u32 = 0x00a3_da50;
/// `NiGetViewerString` (cdecl, the name) with the word at `012043c8` (the first word of the
/// class's `NiRTTI` record), `NiGetViewerString_ov9` (cdecl: a label and a float), the label
/// `"Damping"` (`010c38a0`), and the add to the viewer strings (`0096a610`: `this` the
/// strings, the address of a word holding the string).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const TYPE_NAME_WORD: u32 = 0x0120_43c8;
const FLOAT_VIEWER_STRING: u32 = 0x00a7_d550;
const DAMPING_LABEL: u32 = 0x010c_38a0;
const ADD_VIEWER_STRING: u32 = 0x0096_a610;
/// Point helpers (all `this` a point): adds the other point in place (`0063c8a0`) and
/// subtracts it in place (`004578c0`); `this - other` into an out point (`00439ef0`: this,
/// out, other; returns out); `this * scale` into an out point (`0045bb20`: this, out,
/// scale); `scale * point` into an out point (`004a3760`, cdecl: out, scale, the point;
/// returns out); `this / divisor` into an out point (`0053d280`: this, out, divisor;
/// returns out); and the inequality test `00439090` (this, other; true when they differ).
const POINT_ADD_IN_PLACE: u32 = 0x0063_c8a0;
const POINT_SUBTRACT_IN_PLACE: u32 = 0x0045_78c0;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_SCALE: u32 = 0x0045_bb20;
const SCALE_TIMES_POINT: u32 = 0x004a_3760;
const POINT_DIVIDE: u32 = 0x0053_d280;
const POINT_DIFFERS: u32 = 0x0043_9090;
/// The `double` 0.0 (`01012060`).
const ZERO: u32 = 0x0101_2060;

/// Copies three words from `from` to `to` (`MOV` copies: no quieting).
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// Adds `string` to `strings` through a local word holding it, as the exe does.
fn add_viewer_string(e: &mut Engine, strings: Ptr, string: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), string);
        e.call(ADD_VIEWER_STRING, &args![strings, local]);
    });
}

// Translated from 00c634b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::Initialize` (Xbox PDB; `RET 8`: the particle system and the
/// particle's index, of which only the low 16 bits are read): adds `velLast` to the
/// particle's entry (0x1C bytes each in the array at the system's +0x70; `0063c8a0`), then
/// subtracts `velLast` times `fLastTimeDelta` (`0045bb20` into a local, the float copied
/// with `FLD`/`FSTP`) from its position (0xC bytes each in the array at the system's
/// +0x20; `004578c0`).
pub fn bs_parent_velocity_modifier_initialize(e: &mut Engine, this: Ptr, system: Ptr, index: u16) {
    let base = this.addr();
    let index = u32::from(index);
    let entries = e.mem.u32(system.addr() + 0x70);
    let entry = entries.wrapping_add(index * 0x1c);
    e.call(POINT_ADD_IN_PLACE, &args![entry, base + VELOCITY_LAST]);
    let positions = e.mem.u32(system.addr() + 0x20);
    let delta = x87_quiet_bits(e.mem.u32(base + LAST_TIME_DELTA));
    e.with_stack(12, |e, scaled| {
        let scaled = e
            .call(POINT_SCALE, &args![base + VELOCITY_LAST, scaled, delta])
            .u32();
        let position = positions.wrapping_add(index * 0xc);
        e.call(POINT_SUBTRACT_IN_PLACE, &args![position, scaled]);
    });
}

// Translated from 00c63520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::Update` (Xbox PDB; `RET 8`: the time and the particle
/// system). Nothing without particles (the 16-bit count at the system's +0x50). Without a
/// parent node (the target's (+0x10) word at +0x18), the modifier's own slot 0x94. Else:
/// when `pntLast` still equals the start point (`011f426c`, `FUCOMPP` per component), it
/// starts there: `velLast` the start point, `fTimeLast` the time, `pntLast` the parent's
/// point at +0x8C. Then with `delta = time - fTimeLast` (stored as a float): unless it
/// compares equal to 0.0 (a NaN does not), the new velocity is `(fDamping * (point -
/// pntLast)) / delta` (`00439ef0`, `004a3760`, `0053d280`), and when it differs from
/// `velLast` by a non-zero step (`00439ef0`, `00439090` against the start point) every
/// particle's entry gets that step added (`0063c8a0`, 0x1C bytes apart from the system's
/// +0x70); `velLast` becomes the new velocity. Finally `fLastTimeDelta` = `delta`,
/// `fTimeLast` = the time and `pntLast` = the parent's point (read before the update).
pub fn bs_parent_velocity_modifier_update(e: &mut Engine, this: Ptr, time: f32, system: Ptr) {
    let base = this.addr();
    if e.mem.u16(system.addr() + 0x50) == 0 {
        return;
    }
    let target = e.mem.u32(base + TARGET);
    let parent = e.mem.u32(target + 0x18);
    if parent == 0 {
        e.vcall(base, 0x94, &[]);
        return;
    }
    let unset = (0..3).all(|axis| {
        x87_load32(e.mem.u32(START_POINT + 4 * axis))
            == x87_load32(e.mem.u32(base + POINT_LAST + 4 * axis))
    });
    let time = time.to_bits();
    if unset {
        copy_point(e, START_POINT, base + VELOCITY_LAST);
        e.mem.set_u32(base + TIME_LAST, x87_quiet_bits(time));
        copy_point(e, parent + 0x8c, base + POINT_LAST);
    }
    let mut count = u32::from(e.mem.u16(system.addr() + 0x50));
    // The locals: the parent's point (12), the new velocity (12), the step (12), and the
    // temporaries of the subtraction and the scaling (12 each).
    e.with_stack(60, |e, locals| {
        let point = locals.addr();
        let velocity = point + 12;
        let step = point + 24;
        let difference = point + 36;
        let scaled = point + 48;
        copy_point(e, parent + 0x8c, point);
        let delta = x87_store32(x87_sub(
            x87_load32(time),
            x87_m32(e.mem.u32(base + TIME_LAST)),
        ));
        let zero: f64 = e.global(ZERO);
        if x87_load32(delta) != zero {
            let difference = e
                .call(POINT_SUBTRACT, &args![point, difference, base + POINT_LAST])
                .u32();
            let damping = x87_quiet_bits(e.mem.u32(base + DAMPING));
            let scaled = e
                .call(SCALE_TIMES_POINT, &args![scaled, damping, difference])
                .u32();
            e.call(
                POINT_DIVIDE,
                &args![scaled, velocity, x87_quiet_bits(delta)],
            );
            e.call(POINT_SUBTRACT, &args![velocity, step, base + VELOCITY_LAST]);
            if e.call(POINT_DIFFERS, &args![step, START_POINT]).bool() {
                let mut entry = e.mem.u32(system.addr() + 0x70);
                loop {
                    let left = count;
                    count = count.wrapping_sub(1);
                    if left == 0 {
                        break;
                    }
                    e.call(POINT_ADD_IN_PLACE, &args![entry, step]);
                    entry = entry.wrapping_add(0x1c);
                }
            }
            copy_point(e, velocity, base + VELOCITY_LAST);
        }
        e.mem.set_u32(base + LAST_TIME_DELTA, x87_quiet_bits(delta));
        e.mem.set_u32(base + TIME_LAST, x87_quiet_bits(time));
        copy_point(e, point, base + POINT_LAST);
    });
}

// Translated from 00c63780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::HandleReset` (Xbox PDB): `pntLast` back to the start point
/// (`011f426c`, word by word).
pub fn bs_parent_velocity_modifier_handle_reset(e: &mut Engine, this: Ptr) {
    copy_point(e, START_POINT, this.addr() + POINT_LAST);
}

// Translated from 00c63970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::SaveBinary` (Xbox PDB): the `NiPSysModifier` part, then
/// `fDamping` (`NiStreamSaveBinary<float>`).
pub fn bs_parent_velocity_modifier_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_PSYS_MODIFIER_SAVE_BINARY, &args![this, stream]);
    e.call(STREAM_SAVE_FLOAT, &args![stream, this.addr() + DAMPING]);
}

// Translated from 00c639a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::IsEqual` (Xbox PDB): the `NiPSysModifier` test, and when it
/// holds the two `fDamping` equal (`FUCOMPP`: a NaN is never equal).
pub fn bs_parent_velocity_modifier_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e
        .call(NI_PSYS_MODIFIER_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    x87_load32(e.mem.u32(this.addr() + DAMPING)) == x87_load32(e.mem.u32(other.addr() + DAMPING))
}

// Translated from 00c63a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParentVelocityModifier::GetViewerStrings` (Xbox PDB): the `NiPSysModifier` strings,
/// the string `NiGetViewerString` makes of the word at `012043c8`, and `"Damping"` with
/// `fDamping` (`00a7d550`, the float copied with `FLD`/`FSTP`), each added through
/// `0096a610`.
pub fn bs_parent_velocity_modifier_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_PSYS_MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let type_name = e.mem.u32(TYPE_NAME_WORD);
    let string = e.call(GET_VIEWER_STRING, &args![type_name]).u32();
    add_viewer_string(e, strings, string);
    let damping = x87_quiet_bits(e.mem.u32(this.addr() + DAMPING));
    let string = e
        .call(FLOAT_VIEWER_STRING, &args![DAMPING_LABEL, damping])
        .u32();
    add_viewer_string(e, strings, string);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c6_34b0,
            bs_parent_velocity_modifier_initialize(Ptr, Ptr, u16)
        ),
        entry!(
            0x00c6_3520,
            bs_parent_velocity_modifier_update(Ptr, f32, Ptr)
        ),
        entry!(0x00c6_3780, bs_parent_velocity_modifier_handle_reset(Ptr)),
        entry!(
            0x00c6_3970,
            bs_parent_velocity_modifier_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c6_39a0,
            bs_parent_velocity_modifier_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00c6_3a00,
            bs_parent_velocity_modifier_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c6_3aa0, fn_00c63aa0(Ptr) -> u32),
        entry!(0x00c6_3930, fn_00c63930(Ptr, Ptr)),
        entry!(0x00c6_3950, fn_00c63950(Ptr, Ptr) -> bool),
        entry!(0x00c63330, fn_00c63330(Ptr) -> Ptr),
        entry!(0x00c63880, fn_00c63880() -> Ptr),
        entry!(0x00c63840, fn_00c63840(Ptr, Ptr, Ptr)),
        entry!(
            0x00c637b0,
            bs_parent_velocity_modifier_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00c63900,
            bs_parent_velocity_modifier_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c63a70,
            bs_parent_velocity_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c63930_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C31650, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_3930, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C31650)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c63950_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00C31670, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c6_3950, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00C31670)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c63aa0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_3aa0, &args![0x1000u32]).u32(), 0x0120_43c8);
        assert_eq!(e.call(0x00c6_3aa0, &args![0u32]).u32(), 0x0120_43c8);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn constructor_factory_and_clone() {
        let mut e = Engine::new();
        e.register(NI_PSYS_MODIFIER_CONSTRUCT, nothing);
        e.register(NI_PSYS_MODIFIER_COPY_MEMBERS, nothing);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.map(START_POINT, 12);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(START_POINT + 4 * i as u32, *v);
        }
        let this = e.mem.alloc(0x3c);
        e.mem.write(this, &[0xaa; 0x3c]);
        assert_eq!(e.call(0x00c6_3330, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_PARENT_VELOCITY_MODIFIER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 3000);
        assert_eq!(e.mem.f32(this + 0x28), 2.0);
        assert_eq!(e.mem.f32(this + 0x38), 3.0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0xaaaa_aaaa);
        let object = e.call(0x00c6_3880, &args![]).u32();
        assert_eq!(e.mem.u32(object), BS_PARENT_VELOCITY_MODIFIER_VTABLE);
        e.mem.set_f32(this + 0x20, 0.75);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c6_37b0, &args![this, 0x99u32]).u32();
        assert_eq!(
            calls_to(&e, NI_PSYS_MODIFIER_COPY_MEMBERS),
            vec![vec![this, clone, 0x99]]
        );
        assert_eq!(e.mem.f32(clone + 0x20), 0.75);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c6_3880, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_and_deleting_destructor() {
        let mut e = Engine::new();
        e.register(NI_PSYS_MODIFIER_LOAD_BINARY, nothing);
        e.register(STREAM_READ_FLOAT, nothing);
        e.register(NI_PSYS_MODIFIER_DESTRUCT, nothing);
        e.register(SIZED_DELETE, nothing);
        e.call_log = Some(vec![]);
        e.call(0x00c6_3900, &args![0x100u32, 0x200u32]);
        assert_eq!(
            calls_to(&e, NI_PSYS_MODIFIER_LOAD_BINARY),
            vec![vec![0x100, 0x200]]
        );
        assert_eq!(calls_to(&e, STREAM_READ_FLOAT), vec![vec![0x200, 0x120]]);
        assert_eq!(e.call(0x00c6_3a70, &args![0x100u32, 0u32]).u32(), 0x100);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00c6_3a70, &args![0x100u32, 1u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![0x100, 0x3c]]);
        assert_eq!(calls_to(&e, NI_PSYS_MODIFIER_DESTRUCT).len(), 2);
    }
}

#[cfg(test)]
mod n067_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn set_point(e: &mut Engine, at: u32, value: [f32; 3]) {
        for (i, v) in (0u32..).zip(value) {
            e.mem.set_f32(at + 4 * i, v);
        }
    }

    fn apply(e: &mut Engine, a: &[u32], f: fn(f32, f32) -> f32) {
        for i in 0..3 {
            let v = f(e.mem.f32(a[0] + 4 * i), e.mem.f32(a[1] + 4 * i));
            e.mem.set_f32(a[0] + 4 * i, v);
        }
    }

    /// An engine with the start point (0, 0, 0), 0.0 at `01012060` and the point helpers.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(START_POINT, 12);
        e.map(ZERO, 8);
        e.register(POINT_ADD_IN_PLACE, |e, a| {
            apply(e, a, |x, y| x + y);
            a[0].into_ret()
        });
        e.register(POINT_SUBTRACT_IN_PLACE, |e, a| {
            apply(e, a, |x, y| x - y);
            a[0].into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_SCALE, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(SCALE_TIMES_POINT, |e, a| {
            for i in 0..3 {
                let v = f32::from_bits(a[1]) * e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_DIVIDE, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) / f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_DIFFERS, |e, a| {
            u32::from(point(e, a[0]) != point(e, a[1])).into_ret()
        });
        e
    }

    /// A particle system with `count` particles: entries (0x1C bytes) at +0x70 and
    /// positions at +0x20.
    fn system(e: &mut Engine, count: u16) -> u32 {
        let system = e.mem.alloc(0x74);
        e.mem.set_u16(system + 0x50, count);
        let entries = e.mem.alloc(0x1c * 4);
        let positions = e.mem.alloc(0xc * 4);
        e.mem.set_u32(system + 0x70, entries);
        e.mem.set_u32(system + 0x20, positions);
        system
    }

    #[test]
    fn initialize_moves_the_particle_by_the_last_velocity() {
        let mut e = engine();
        let this = e.mem.alloc(0x3c);
        set_point(&mut e, this + VELOCITY_LAST, [1.0, 2.0, 3.0]);
        e.mem.set_f32(this + LAST_TIME_DELTA, 0.5);
        let system = system(&mut e, 4);
        let entries = e.mem.u32(system + 0x70);
        let positions = e.mem.u32(system + 0x20);
        set_point(&mut e, positions + 0x18, [10.0, 10.0, 10.0]);
        // Only the low 16 bits of the index word count.
        e.call(0x00c6_34b0, &args![this, system, 0x0001_0002u32]);
        assert_eq!(point(&e, entries + 0x38), [1.0, 2.0, 3.0]);
        assert_eq!(point(&e, positions + 0x18), [9.5, 9.0, 8.5]);
        assert_eq!(point(&e, entries), [0.0, 0.0, 0.0]);
    }

    #[test]
    fn update_follows_the_parent() {
        let mut e = engine();
        let this = e.mem.alloc(0x3c);
        let target = e.mem.alloc(0x1c);
        let parent = e.mem.alloc(0x98);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_u32(target + 0x18, parent);
        e.mem.set_f32(this + DAMPING, 0.5);
        set_point(&mut e, parent + 0x8c, [2.0, 4.0, 6.0]);
        let system = system(&mut e, 2);
        let entries = e.mem.u32(system + 0x70);
        // First run: starts at the parent's point; no time has passed.
        e.call(0x00c6_3520, &args![this, 1.0f32, system]);
        assert_eq!(point(&e, this + POINT_LAST), [2.0, 4.0, 6.0]);
        assert_eq!(e.mem.f32(this + TIME_LAST), 1.0);
        assert_eq!(e.mem.f32(this + LAST_TIME_DELTA), 0.0);
        assert_eq!(point(&e, this + VELOCITY_LAST), [0.0, 0.0, 0.0]);
        // The parent moves by (4, 0, -2) in 2 seconds: velocity 0.5 * (4, 0, -2) / 2.
        set_point(&mut e, parent + 0x8c, [6.0, 4.0, 4.0]);
        e.call(0x00c6_3520, &args![this, 3.0f32, system]);
        assert_eq!(point(&e, this + VELOCITY_LAST), [1.0, 0.0, -0.5]);
        assert_eq!(point(&e, entries), [1.0, 0.0, -0.5]);
        assert_eq!(point(&e, entries + 0x1c), [1.0, 0.0, -0.5]);
        assert_eq!(e.mem.f32(this + LAST_TIME_DELTA), 2.0);
        assert_eq!(e.mem.f32(this + TIME_LAST), 3.0);
        assert_eq!(point(&e, this + POINT_LAST), [6.0, 4.0, 4.0]);
        // The same velocity again: no step for the particles.
        set_point(&mut e, parent + 0x8c, [10.0, 4.0, 2.0]);
        e.call(0x00c6_3520, &args![this, 5.0f32, system]);
        assert_eq!(point(&e, entries), [1.0, 0.0, -0.5]);
        assert_eq!(point(&e, this + POINT_LAST), [10.0, 4.0, 2.0]);
        // No particles: nothing at all.
        e.mem.set_u16(system + 0x50, 0);
        e.call(0x00c6_3520, &args![this, 9.0f32, system]);
        assert_eq!(e.mem.f32(this + TIME_LAST), 5.0);
        // No parent: the modifier's slot 0x94.
        e.mem.set_u16(system + 0x50, 1);
        e.mem.set_u32(target + 0x18, 0);
        let vtable = e.mem.alloc(0x98);
        e.mem.set_u32(vtable + 0x94, 0x0090_5000);
        e.register(0x0090_5000, nothing);
        e.mem.set_u32(this, vtable);
        e.call_log = Some(vec![]);
        e.call(0x00c6_3520, &args![this, 9.0f32, system]);
        assert_eq!(calls(&e, 0x0090_5000), vec![vec![this]]);
        assert_eq!(e.mem.f32(this + TIME_LAST), 5.0);
    }

    #[test]
    fn reset_save_equal_and_viewer_strings() {
        let mut e = engine();
        let this = e.mem.alloc(0x3c);
        set_point(&mut e, this + POINT_LAST, [1.0, 2.0, 3.0]);
        set_point(&mut e, START_POINT, [0.0, -1.0, 0.0]);
        e.call(0x00c6_3780, &args![this]);
        assert_eq!(point(&e, this + POINT_LAST), [0.0, -1.0, 0.0]);
        e.register(NI_PSYS_MODIFIER_SAVE_BINARY, nothing);
        e.register(STREAM_SAVE_FLOAT, nothing);
        e.call_log = Some(vec![]);
        e.call(0x00c6_3970, &args![this, 0x200u32]);
        assert_eq!(
            calls(&e, NI_PSYS_MODIFIER_SAVE_BINARY),
            vec![vec![this, 0x200]]
        );
        assert_eq!(
            calls(&e, STREAM_SAVE_FLOAT),
            vec![vec![0x200, this + DAMPING]]
        );
        e.register(NI_PSYS_MODIFIER_IS_EQUAL, |_, _| 1u32.into_ret());
        let other = e.mem.alloc(0x3c);
        e.mem.set_f32(this + DAMPING, 0.25);
        e.mem.set_f32(other + DAMPING, 0.25);
        assert!(e.call(0x00c6_39a0, &args![this, other]).bool());
        e.mem.set_f32(other + DAMPING, f32::NAN);
        assert!(!e.call(0x00c6_39a0, &args![this, other]).bool());
        e.register(NI_PSYS_MODIFIER_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00c6_39a0, &args![this, this]).bool());
        e.register(NI_PSYS_MODIFIER_GET_VIEWER_STRINGS, nothing);
        e.register(GET_VIEWER_STRING, |_, a| a[0].into_ret());
        e.register(FLOAT_VIEWER_STRING, |_, a| a[1].into_ret());
        e.register(ADD_VIEWER_STRING, nothing);
        e.map(TYPE_NAME_WORD, 4);
        e.set_global(TYPE_NAME_WORD, 0x3333u32);
        e.call(0x00c6_3a00, &args![this, 0x900u32]);
        assert_eq!(calls(&e, GET_VIEWER_STRING), vec![vec![0x3333]]);
        assert_eq!(
            calls(&e, FLOAT_VIEWER_STRING),
            vec![vec![DAMPING_LABEL, 0.25f32.to_bits()]]
        );
        assert_eq!(calls(&e, ADD_VIEWER_STRING).len(), 2);
    }
}
