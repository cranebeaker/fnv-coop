//! `(unplaced)/run_00c7f060` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the constructed class: slot 0 is `00c7de30`, which the engine map names
/// `bhkRagdollController::_scalar_deleting_destructor_`.
const RAGDOLL_CONTROLLER_VTABLE: u32 = 0x010c_4ddc;
/// The vtable stored at `+0xa0` (a member object).
const MEMBER_VTABLE_01071AEC: u32 = 0x0107_1aec;
/// The vtable of the `BSSimpleArray<bhkRagdollController::BoneTransf, 1024>` the constructor
/// keeps as a local (slot 0 `00c7a690` is its scalar deleting destructor, slot 1 `_Allocate`,
/// slot 2 `00cb9cb0` `_Deallocate` in the engine map).
const BONE_TRANSFORM_ARRAY_VTABLE: u32 = 0x010c_4db8;
/// The data cell that holds slot 2 of that vtable (`00cb9cb0`), which the constructor calls
/// through the cell (`CALL dword ptr [010c4dc0]`).
const BONE_TRANSFORM_ARRAY_DEALLOCATE_CELL: u32 = 0x010c_4dc0;
/// The pooled-string handle of the empty `NiFixedString` (a dword global).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// The `float` stored in both pose slots of the two 0x50-byte records.
const RECORD_FLOAT: u32 = 0x010c_4df8;
/// The `float` stored at `+0x280`.
const FLOAT_AT_280: u32 = 0x0102_2958;
/// The `float` stored at `+0x27c`.
const FLOAT_AT_27C: u32 = 0x0101_7868;
/// The `float` copied four times to `+0x30`.
const FLOAT_AT_30: u32 = 0x0101_2068;
/// The 16 bytes `(0.0, 0.0, 0.0, 1.0)` (an identity quaternion).
const IDENTITY_QUATERNION: u32 = 0x010c_71b0;
/// The imported `InterlockedDecrement` (`stdcall`).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `bhkRagdollController::CopyPoseTo3D` (Xbox PDB, `00c7d3b0`, `cdecl`, five stack words).
const COPY_POSE_TO_3D: u32 = 0x00c7_d3b0;
/// `bhkRagdollController::Init` as the decompiler names `00c7e9a0` (`thiscall`, `RET 8`).
const INIT: u32 = 0x00c7_e9a0;
/// `bhkRagdollController::ResetPose` (Xbox PDB, `00c75c00`, `cdecl`, four stack words).
const RESET_POSE: u32 = 0x00c7_5c00;
/// The virtual slot (byte offset) of the two source objects that answers their ragdoll data.
const SLOT_RAGDOLL_DATA: u32 = 0x0c;
/// The virtual slot (byte offset) that deletes the object.
const SLOT_DELETE: u32 = 0x04;

fn set_dword(e: &mut Engine, base: u32, offset: u32, value: u32) {
    e.mem.set_u32(base.wrapping_add(offset), value);
}

fn set_byte(e: &mut Engine, base: u32, offset: u32, value: u8) {
    e.mem.set_u8(base.wrapping_add(offset), value);
}

fn set_word(e: &mut Engine, base: u32, offset: u32, value: u16) {
    e.mem.set_u16(base.wrapping_add(offset), value);
}

/// `MOVAPS [base + offset], xmm` with the sixteen bytes given.
fn set_sixteen(e: &mut Engine, base: u32, offset: u32, bytes: &[u8]) {
    e.mem.write(base.wrapping_add(offset), bytes);
}

/// `FLD dword [global]` then `FST`/`FSTP dword`: the global's float, a signalling NaN quieted.
fn quiet_float_bits(e: &Engine, global: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(global))
}

/// Releases a reference-counted object held in a member the way the exe's inlined smart pointer
/// assignment does: `InterlockedDecrement` of the count at `+4`, the delete slot when it
/// reaches zero.
fn release_held(e: &mut Engine, held: u32) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
        .u32();
    if count == 0 {
        e.vcall(held, SLOT_DELETE, &args![]);
    }
}

// Translated from 00c7f060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c7f060 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f31ab0, state variable) is not translated
/// Unnamed in the engine map: the constructor of `bhkRagdollController` (it stores the vtable
/// `010c4ddc`, whose destructor entry is the map's `bhkRagdollController::
/// _scalar_deleting_destructor_`, and is called from `CreateRagdollInstance`). `thiscall`,
/// `RET 0xc` (two source objects and a word). Fills in the members at the offsets the exe
/// stores (two 0x50-byte records at `+0xf6` and `+0x146`, the identity quaternions at
/// `+0xc0`, `+0x20`, the 16-byte constants, many flag bytes), then, with the ragdoll data the
/// two source objects answer (virtual slot +0xc, when they are not null) and a local array
/// of bone transforms: copies the pose to 3D (`00c7d3b0`) when both exist, runs `00c7e9a0` on
/// `this` with the first object and the word, resets the pose (`00c75c00`) from the first
/// object's data and frees the local array's buffer. Returns `this`.
///
/// The two inlined reference releases at `+0x84` and `+0x2a4` read members the exe has just
/// cleared, so they never run; they are kept. The third word of `00c75c00` is the dword at the
/// local byte slot the copy wrote (its upper three bytes are the upper three bytes of the second
/// source pointer).
pub fn fn_00c7f060(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, word: u32) -> Ptr {
    let base = this.addr();
    set_dword(e, base, 0, RAGDOLL_CONTROLLER_VTABLE);
    set_dword(e, base, 0x84, 0);
    set_dword(e, base, 0x94, 0);
    set_dword(e, base, 0x98, 0);
    set_dword(e, base, 0x9c, 0x8000_0000);
    set_dword(e, base, 0xa0, MEMBER_VTABLE_01071AEC);
    set_dword(e, base, 0xa4, 0);
    set_dword(e, base, 0xa8, 0);
    set_dword(e, base, 0xac, 0);
    set_dword(e, base, 0x22c, 0);
    set_dword(e, base, 0x230, 0);
    set_dword(e, base, 0x234, 0x8000_0000);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    set_dword(e, base, 0x24c, empty);
    set_dword(e, base, 0x250, 0);
    set_dword(e, base, 0x260, 0);
    set_dword(e, base, 0x264, 0);
    set_dword(e, base, 0x268, 0);
    set_dword(e, base, 0x26c, 0);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    set_dword(e, base, 0x2a0, empty);
    set_dword(e, base, 0x2a4, 0);
    set_dword(e, base, 0x2a8, 0);
    set_dword(e, base, 0x2ac, 0);
    set_dword(e, base, 0x2b0, 0);
    set_dword(e, base, 0x2b4, 0);
    // FLDZ; FST float
    set_dword(e, base, 0x23c, 0);
    let record_float = quiet_float_bits(e, RECORD_FLOAT);
    set_dword(e, base, 0x48, 0);
    set_dword(e, base, 0x4c, 0);
    set_byte(e, base, 0x50, 0);
    set_dword(e, base, 0x54, 0);
    set_dword(e, base, 0x58, 0);
    set_dword(e, base, 0x5c, 0);
    set_byte(e, base, 0x240, 0);
    set_byte(e, base, 0x241, 1);
    set_byte(e, base, 0x242, 1);
    let identity = e.mem.bytes(IDENTITY_QUATERNION, 16);
    for record in 0..2u32 {
        let at = 0xf6 + 0x50 * record;
        set_word(e, base, at - 2, 0xffff);
        set_dword(e, base, at + 2, record_float);
        set_word(e, base, at, 0xffff);
        set_sixteen(e, base, at + 0xa, &[0; 16]);
        set_sixteen(e, base, at + 0x1a, &[0; 16]);
        set_byte(e, base, at + 0x3b, 0);
        set_byte(e, base, at + 0x3a, 1);
        set_sixteen(e, base, at + 0x2a, &identity);
        set_dword(e, base, at + 0x3e, 0);
        set_dword(e, base, at + 0x42, 0);
    }
    set_sixteen(e, base, 0xd0, &[0; 16]);
    set_sixteen(e, base, 0xe0, &[0; 16]);
    set_byte(e, base, 0xb4, 0);
    set_dword(e, base, 0x1a0, 0);
    set_dword(e, base, 0x19c, 0);
    set_sixteen(e, base, 0xc0, &identity);
    set_dword(e, base, 0x1a8, 0);
    for offset in [
        0x1a4, 0x1a5, 0x42, 0xb0, 0x21f, 0x21c, 0x41, 0xb1, 0x220, 0x21d, 0x255, 0x43, 0xb2, 0x221,
        0xb3, 0x222, 0x21e, 0x256, 0x1bd, 0x1ed,
    ] {
        set_byte(e, base, offset, 0);
    }
    set_dword(e, base, 0x228, 0);
    set_byte(e, base, 0x254, 0);
    set_byte(e, base, 0x190, 0);
    set_dword(e, base, 0x194, 0);
    // The inlined smart pointer assignment of `+0x84` (cleared above, so never taken).
    let held = e.mem.u32(base.wrapping_add(0x84));
    if held != 0 {
        release_held(e, held);
        set_dword(e, base, 0x84, 0);
    }
    set_dword(e, base, 0x248, 0);
    set_dword(e, base, 0x258, 0xffff_ffff);
    // The same for `+0x2a4`.
    let held = e.mem.u32(base.wrapping_add(0x2a4));
    if held != 0 {
        release_held(e, held);
        set_dword(e, base, 0x2a4, 0);
    }
    set_dword(e, base, 0x88, 0);
    set_dword(e, base, 0x290, 0x3f80_0000);
    set_byte(e, base, 0x8c, 0);
    set_byte(e, base, 0x28c, 0);
    set_byte(e, base, 0x28d, 0);
    set_dword(e, base, 0x244, 0);
    set_byte(e, base, 0x28e, 0);
    let float_at_280 = quiet_float_bits(e, FLOAT_AT_280);
    set_byte(e, base, 0x28f, 0);
    set_dword(e, base, 0x294, 0);
    set_byte(e, base, 0x243, 0);
    set_byte(e, base, 0x223, 0);
    set_dword(e, base, 0x90, 0);
    set_dword(e, base, 0xc, 2);
    set_sixteen(e, base, 0x10, &[0; 16]);
    set_sixteen(e, base, 0x20, &identity);
    let broadcast = e.mem.u32(FLOAT_AT_30);
    for lane in 0..4 {
        set_dword(e, base, 0x30 + 4 * lane, broadcast);
    }
    set_dword(e, base, 0x280, float_at_280);
    let float_at_27c = quiet_float_bits(e, FLOAT_AT_27C);
    set_word(e, base, 0x23a, 0);
    set_dword(e, base, 0x27c, float_at_27c);
    set_dword(e, base, 0x8, 0);
    set_byte(e, base, 0x40, 0);
    set_byte(e, base, 0x284, 1);
    set_dword(e, base, 0x288, 0xffff_ffff);
    set_byte(e, base, 0x44, 0);
    set_byte(e, base, 0x4, 0);
    set_byte(e, base, 0x198, 0);
    set_byte(e, base, 0x224, 1);
    set_word(e, base, 0x238, 0);

    let skeleton = if first.is_null() {
        0
    } else {
        e.vcall(first.addr(), SLOT_RAGDOLL_DATA, &args![]).u32()
    };
    let pose = if second.is_null() {
        0
    } else {
        e.vcall(second.addr(), SLOT_RAGDOLL_DATA, &args![]).u32()
    };
    // The locals: a byte slot (inside the dword that held `second`), a dword and the array.
    let array = e.mem.alloc(0x10);
    let byte_slot = e.mem.alloc(4);
    let dword_slot = e.mem.alloc(4);
    e.mem.set_u32(byte_slot, second.addr() & 0xffff_ff00);
    e.mem.set_u32(dword_slot, 0);
    e.mem.set_u32(array, BONE_TRANSFORM_ARRAY_VTABLE);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0);
    e.mem.set_u32(array + 0xc, 0);
    if skeleton != 0 && pose != 0 {
        e.call(
            COPY_POSE_TO_3D,
            &args![skeleton, pose, array, byte_slot, dword_slot],
        );
    }
    e.call(INIT, &args![this, first, word]);
    let slot_word = e.mem.u32(byte_slot);
    let slot_dword = e.mem.u32(dword_slot);
    e.call(RESET_POSE, &args![skeleton, array, slot_word, slot_dword]);
    set_dword(e, base, 0x2b8, 0);
    e.mem.set_u32(array, BONE_TRANSFORM_ARRAY_VTABLE);
    let buffer = e.mem.u32(array + 4);
    if buffer != 0 {
        let deallocate = e.mem.u32(BONE_TRANSFORM_ARRAY_DEALLOCATE_CELL);
        e.call(deallocate, &args![array, buffer]);
    }
    e.mem.free(dword_slot);
    e.mem.free(byte_slot);
    e.mem.free(array);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c7_f060, fn_00c7f060(Ptr, Ptr, Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    const CONSTANT_PAGES: [(u32, u32); 8] = [
        (EMPTY_FIXED_STRING, 4),
        (RECORD_FLOAT, 4),
        (FLOAT_AT_280, 4),
        (FLOAT_AT_27C, 4),
        (FLOAT_AT_30, 4),
        (IDENTITY_QUATERNION, 16),
        (BONE_TRANSFORM_ARRAY_DEALLOCATE_CELL, 4),
        (0x0107_1aec, 4),
    ];

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for (address, length) in CONSTANT_PAGES {
            e.map(address, length);
        }
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0abc_def0);
        e.mem.set_u32(RECORD_FLOAT, 0x3f86_0a92);
        e.mem.set_u32(FLOAT_AT_280, 0x461c_4000);
        e.mem.set_u32(FLOAT_AT_27C, 0x41a0_0000);
        e.mem.set_u32(FLOAT_AT_30, 0x3f80_0001);
        e.mem.write(
            IDENTITY_QUATERNION,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x3f],
        );
        e.mem
            .set_u32(BONE_TRANSFORM_ARRAY_DEALLOCATE_CELL, 0x0f00_f200);
        e.register(INIT, |_, _| Ret::default());
        e.register(RESET_POSE, |_, _| Ret::default());
        e.register(COPY_POSE_TO_3D, |_, _| Ret::default());
        e.register(0x0f00_f200, |_, _| Ret::default());
        e
    }

    fn source_object(e: &mut Engine, data: u32) -> u32 {
        let mut slots = [0u32; 4];
        slots[SLOT_RAGDOLL_DATA as usize / 4] = 0x0f00_f000;
        e.put_vtable(0x0f00_f100, &slots);
        e.register(0x0f00_f000, |e, a| e.mem.u32(a[0] + 8).into_ret());
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0f00_f100);
        e.mem.set_u32(object + 8, data);
        object
    }

    fn filled(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x2c0);
        for offset in (0..0x2c0).step_by(4) {
            e.mem.set_u32(this + offset, 0xa5a5_a5a5);
        }
        this
    }

    #[test]
    fn at_00c7f060_fills_in_the_members() {
        let mut e = rig();
        let first = source_object(&mut e, 0x1000);
        let second = source_object(&mut e, 0x2000);
        let this = filled(&mut e);
        assert_eq!(
            e.call(0x00c7_f060, &args![this, first, second, 7u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), RAGDOLL_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xa0), 0x0107_1aec);
        assert_eq!(e.mem.u32(this + 0x9c), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x234), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x24c), 0x0abc_def0);
        assert_eq!(e.mem.u32(this + 0x2a0), 0x0abc_def0);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(this + 0x288), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x258), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x290), 1.0f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x280), 0x461c_4000);
        assert_eq!(e.mem.u32(this + 0x27c), 0x41a0_0000);
        assert_eq!(e.mem.u32(this + 0x2b8), 0);
        // The quaternion constants and the broadcast lanes.
        for base in [0x20u32, 0xc0] {
            assert_eq!(e.mem.u32(this + base), 0);
            assert_eq!(e.mem.u32(this + base + 0xc), 1.0f32.to_bits());
        }
        for lane in 0..4 {
            assert_eq!(e.mem.u32(this + 0x30 + 4 * lane), 0x3f80_0001);
        }
        // The two records.
        for at in [0xf6u32, 0x146] {
            assert_eq!(e.mem.u16(this + at - 2), 0xffff);
            assert_eq!(e.mem.u16(this + at), 0xffff);
            assert_eq!(e.mem.u32(this + at + 2), 0x3f86_0a92);
            assert_eq!(e.mem.u8(this + at + 0x3a), 1);
            assert_eq!(e.mem.u8(this + at + 0x3b), 0);
            assert_eq!(e.mem.u32(this + at + 0x2a + 12), 1.0f32.to_bits());
            assert_eq!(e.mem.u32(this + at + 0xa), 0);
            assert_eq!(e.mem.u32(this + at + 0x3e), 0);
            assert_eq!(e.mem.u32(this + at + 0x42), 0);
        }
        // A few flag bytes and words.
        assert_eq!(e.mem.u8(this + 0x241), 1);
        assert_eq!(e.mem.u8(this + 0x242), 1);
        assert_eq!(e.mem.u8(this + 0x284), 1);
        assert_eq!(e.mem.u8(this + 0x224), 1);
        assert_eq!(e.mem.u16(this + 0x23a), 0);
        assert_eq!(e.mem.u16(this + 0x238), 0);
        // Untouched gaps keep their contents.
        assert_eq!(e.mem.u32(this + 0x60), 0xa5a5_a5a5);
        assert_eq!(e.mem.u32(this + 0x7c), 0xa5a5_a5a5);
        assert_eq!(e.mem.u32(this + 0x1c0), 0xa5a5_a5a5);
        assert_eq!(e.mem.u32(this + 0x2bc), 0xa5a5_a5a5);
    }

    #[test]
    fn at_00c7f060_copies_the_pose_then_inits_resets_and_frees_the_array() {
        let mut e = rig();
        let first = source_object(&mut e, 0x1000);
        let second = source_object(&mut e, 0x2000);
        e.register_double(COPY_POSE_TO_3D, move |e, a| {
            // The array gets a buffer; the byte slot a byte; the dword slot a value.
            e.mem.set_u32(a[2] + 4, 0x6000);
            e.mem.set_u8(a[3], 0x42);
            e.mem.set_u32(a[4], 0x1234);
            Ret::default()
        });
        let this = filled(&mut e);
        e.call(0x00c7_f060, &args![this, first, second, 7u32]);
        let copy = &calls(&e, COPY_POSE_TO_3D)[0];
        assert_eq!(&copy[..2], [0x1000, 0x2000]);
        assert_eq!(calls(&e, INIT), vec![vec![this, first, 7]]);
        // `00c75c00(skeleton, array, byte slot word, dword slot)`: the word holds the upper
        // three bytes of `second` and the byte the copy wrote.
        let reset = &calls(&e, RESET_POSE)[0];
        assert_eq!(reset[0], 0x1000);
        assert_eq!(reset[1], copy[2]);
        assert_eq!(reset[2], (second & 0xffff_ff00) | 0x42);
        assert_eq!(reset[3], 0x1234);
        // The array's buffer is freed through the vtable cell.
        assert_eq!(calls(&e, 0x0f00_f200), vec![vec![copy[2], 0x6000]]);
        // Order: copy, init, reset, free.
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| [COPY_POSE_TO_3D, INIT, RESET_POSE, 0x0f00_f200].contains(a))
            .collect();
        assert_eq!(order, vec![COPY_POSE_TO_3D, INIT, RESET_POSE, 0x0f00_f200]);
    }

    #[test]
    fn at_00c7f060_without_source_objects_skips_the_copy_but_still_resets() {
        let mut e = rig();
        let this = filled(&mut e);
        e.call(0x00c7_f060, &args![this, 0u32, 0u32, 5u32]);
        assert!(calls(&e, COPY_POSE_TO_3D).is_empty());
        assert_eq!(calls(&e, INIT), vec![vec![this, 0, 5]]);
        let reset = &calls(&e, RESET_POSE)[0];
        assert_eq!(reset[0], 0);
        assert_eq!(reset[2], 0);
        assert_eq!(reset[3], 0);
        assert!(calls(&e, 0x0f00_f200).is_empty());
    }

    #[test]
    fn at_00c7f060_needs_both_sources_for_the_copy() {
        let mut e = rig();
        let first = source_object(&mut e, 0x1000);
        let this = filled(&mut e);
        e.call(0x00c7_f060, &args![this, first, 0u32, 5u32]);
        assert!(calls(&e, COPY_POSE_TO_3D).is_empty());
        assert_eq!(calls(&e, RESET_POSE)[0][0], 0x1000);
    }

    #[test]
    fn at_00c7f060_quiets_the_float_constants() {
        let mut e = rig();
        e.mem.set_u32(RECORD_FLOAT, 0x7f80_0001);
        e.mem.set_u32(FLOAT_AT_280, 0xff80_0002);
        e.mem.set_u32(FLOAT_AT_27C, 0x7f80_0003);
        // The broadcast lanes are a plain move: no quieting.
        e.mem.set_u32(FLOAT_AT_30, 0x7f80_0004);
        let this = filled(&mut e);
        e.call(0x00c7_f060, &args![this, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0xf6 + 2), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x146 + 2), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x280), 0xffc0_0002);
        assert_eq!(e.mem.u32(this + 0x27c), 0x7fc0_0003);
        assert_eq!(e.mem.u32(this + 0x30), 0x7f80_0004);
    }
}
