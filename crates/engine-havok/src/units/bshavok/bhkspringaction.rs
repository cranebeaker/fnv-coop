//! `BSHavok/bhkspringaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use super::bhklimitedhingeconstraint::{
    allocate_sized, enter_scope, increment, push_viewer_string, ALLOCATE, POOL_ALLOCATE,
    POOL_OBJECT, REMOVE_REFERENCE,
};
#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkSpringAction` (0x10 bytes, vtable `010c955c`, RTTI-confirmed): a `bhkBinaryAction`
/// (vtable `010c5ef4`, RTTI-confirmed) over a `bhkAction` (vtable `0102016c`); the Havok
/// action is at `+8` and `pInfo` at `+0xC`. `pInfo` is a 0x50-byte cinfo, 16-byte aligned
/// inside a pool block: `+4` and `+8` the two Havok objects, two vectors at `+0x10` and
/// `+0x20`, three `float`s at `+0x30`, `+0x34` and `+0x38` and two flag bytes at `+0x3C`
/// and `+0x3D`. The Havok action keeps the vectors at `+0x30` and `+0x40`, the `float`s
/// at `+0x50`, `+0x54` and `+0x58` and the flag bytes at `+0x5C` and `+0x5D`.
///
/// The functions that allocate Havok objects raise the per-thread allocation word at
/// `TLS + 0x2b4` to 0x10 for their duration and put the old value back. The
/// exception-unwinding frames (`FS:[0]` registrations) and the `/GS` cookie check of the
/// functions marked `Translation incomplete` are not modeled.
const VTABLE_SPRING_ACTION: u32 = 0x010c_955c;
const VTABLE_BINARY_ACTION: u32 = 0x010c_5ef4;
const VTABLE_ACTION: u32 = 0x0102_016c;
const COUNT_ACTION: u32 = 0x0126_8154;
const COUNT_BINARY_ACTION: u32 = 0x0126_8194;
const COUNT_SPRING_ACTION: u32 = 0x0126_85ac;
/// The class's `NiRTTI` global (`012685b0`, whose viewer string heads the list).
const SPRING_ACTION_RTTI: u32 = 0x0126_85b0;
/// `bhkBinaryAction` methods: the destructor body (`00c8a720`), `CopyCinfo` (`00c8a800`),
/// `LoadBinary` (`00c8a7a0`), `SaveBinary` (`00c8aac0`), `CopyMembers` (`00c8a850`, `(clone,
/// process_info)`) and `GetViewerStrings` (`00c8ad40`); `bhkSerializable::Kill`
/// (`00c876a0`), `CloneCritLock` (`00c876f0`), `CloneCritUnlock` (`00c87700`) and the
/// `bhkRefObject` constructor (`00c87c30`).
const BINARY_DESTRUCT: u32 = 0x00c8_a720;
const BINARY_COPY_CINFO: u32 = 0x00c8_a800;
const BINARY_LOAD_BINARY: u32 = 0x00c8_a7a0;
const BINARY_SAVE_BINARY: u32 = 0x00c8_aac0;
const BINARY_COPY_MEMBERS: u32 = 0x00c8_a850;
const BINARY_GET_VIEWER_STRINGS: u32 = 0x00c8_ad40;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_CRIT_LOCK: u32 = 0x00c8_76f0;
const CLONE_CRIT_UNLOCK: u32 = 0x00c8_7700;
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// Copy of a four-`float` vector (`004b4db0`, `ECX` = destination, one word: the source).
const COPY_VECTOR: u32 = 0x004b_4db0;
/// `hkpSpringAction::hkpSpringAction` (`00d4d340`, `ECX` = the block, three words: the two
/// Havok objects and 0) and its `setPositionsInBodySpace` (`00d4d410`, `ECX` = the action,
/// two words: pointers to the two vectors).
const HK_SPRING_CONSTRUCT: u32 = 0x00d4_d340;
const HK_SET_POSITIONS: u32 = 0x00d4_d410;
/// `NiGetViewerString` for an `NiRTTI` (`00a7d330`, cdecl, one word).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// Pool block layout of the default cinfo (`00cc2f40`): the `float` globals `01013974` and
/// `0101e2bc` that it leaves at `+0x34` and `+0x38`.
const DEFAULT_FLOATS: [u32; 2] = [0x0101_3974, 0x0101_e2bc];

/// Byte-swaps the eleven words at `+0x10..+0x38` of `info`.
fn swap_cinfo_words(e: &mut Engine, info: u32) {
    for index in 0..11 {
        let word = e.mem.u32(info + 0x10 + 4 * index);
        e.mem.set_u32(info + 0x10 + 4 * index, word.swap_bytes());
    }
}

/// Constructs a `bhkSpringAction` in `object`: the `bhkRefObject` constructor
/// (`00c87c30`), then the `bhkAction`, `bhkBinaryAction` and `bhkSpringAction` vtables in
/// turn with their counters up by one, and `+0xC` cleared.
fn construct(e: &mut Engine, object: u32) {
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_ACTION);
    e.mem.set_u32(object + 0xc, 0);
    increment(e, COUNT_ACTION);
    e.mem.set_u32(object, VTABLE_BINARY_ACTION);
    increment(e, COUNT_BINARY_ACTION);
    e.mem.set_u32(object, VTABLE_SPRING_ACTION);
    increment(e, COUNT_SPRING_ACTION);
}

// Translated from 00cc2ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2ac0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSpringAction::~bhkSpringAction` (Xbox PDB): sets the class vtable, runs
/// `bhkSerializable::Kill` (`00c876a0`), takes one off the counter `012685ac` and runs the
/// `bhkBinaryAction` destructor body (`00c8a720`).
pub fn bhk_spring_action_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_SPRING_ACTION);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNT_SPRING_ACTION);
    e.mem.set_u32(COUNT_SPRING_ACTION, count.wrapping_sub(1));
    e.call(BINARY_DESTRUCT, &args![this]);
}

// Translated from 00cc2b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the action's `CopyCinfo`; called by `CreateCinfo`, `00cc2f90`): runs the
/// `bhkBinaryAction` `CopyCinfo` (`00c8a800`, `(this, info)`). With a Havok action at
/// `+8`, `info + 0x30`, `+0x34` and `+0x38` take its `float`s at `+0x50`, `+0x54` and
/// `+0x58`, the vector at `+0x30` goes to `info + 0x10` and the one at `+0x40` to
/// `info + 0x20` (`004b4db0`), and the flag bytes at `+0x5C` and `+0x5D` go to `info +
/// 0x3C` and `+0x3D`.
pub fn fn_00cc2b60(e: &mut Engine, this: Ptr, info: u32) {
    e.call(BINARY_COPY_CINFO, &args![this, info]);
    if this.addr() == 0 {
        return;
    }
    let havok = e.mem.u32(this.addr() + 8);
    if havok == 0 {
        return;
    }
    for (from, to) in [(0x50, 0x30), (0x54, 0x34), (0x58, 0x38)] {
        let bits = x87_quiet_bits(e.mem.u32(havok + from));
        e.mem.set_u32(info + to, bits);
    }
    e.call(COPY_VECTOR, &args![info + 0x10, havok + 0x30]);
    e.call(COPY_VECTOR, &args![info + 0x20, havok + 0x40]);
    let first = e.mem.u8(havok + 0x5c);
    e.mem.set_u8(info + 0x3c, first);
    let second = e.mem.u8(havok + 0x5d);
    e.mem.set_u8(info + 0x3d, second);
}

// Translated from 00cc2bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2bc0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name: allocates 0x10 bytes (`00aa13e0`) and, when that succeeded, constructs a
/// `bhkSpringAction` there; answers the object, or 0. Takes no arguments.
pub fn fn_00cc2bc0(e: &mut Engine) -> u32 {
    let object = e.call(ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    construct(e, object);
    object
}

// Translated from 00cc2c60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2c60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSpringAction::CreateClone` (Xbox PDB), vtable slot `0x48`: inside the clone lock
/// (`00c876f0` / `00c87700`), allocates 0x10 bytes (`00aa13e0`) and constructs a
/// `bhkSpringAction` there when that succeeded, then copies the members through slot
/// `0xD0` (`(clone, process_info)`). Answers the clone (0 when the allocation failed).
pub fn bhk_spring_action_create_clone(e: &mut Engine, this: Ptr, process_info: u32) -> u32 {
    e.call(CLONE_CRIT_LOCK, &args![]);
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let clone = if block != 0 {
        construct(e, block);
        block
    } else {
        0
    };
    e.vcall(this.addr(), 0xd0, &args![clone, process_info]);
    e.call(CLONE_CRIT_UNLOCK, &args![]);
    clone
}

/// Multiplies the `float` at `address` by `scale` as the x87 does (in double precision,
/// stored back as a `float`).
fn scale_float(e: &mut Engine, address: u32, scale: f64) {
    let value = f64::from(x87_quiet(e.mem.f32(address)));
    e.mem.set_f32(address, (value * scale) as f32);
}

// Translated from 00cc2d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSpringAction::CopyMembers` (vtable slot `0xD0`; the map names this body
/// `bhkDashpotAction::CopyMembers`, the two being folded): takes the action's cinfo from
/// `CreateCinfo` (slot `0xB4`), and when the `float` at `+0x10` of `process_info` is not
/// 1.0 (a NaN counts as not 1.0) multiplies the cinfo's eight `float`s at `+0x10..+0x2C`
/// by it; then runs the `bhkBinaryAction` `CopyMembers` (`00c8a850`, `(this, clone,
/// process_info)`).
pub fn bhk_spring_action_copy_members(e: &mut Engine, this: Ptr, clone: u32, process_info: u32) {
    let info = e
        .with_stack(4, |e, created| e.vcall(this.addr(), 0xb4, &args![created]))
        .u32();
    let scale = f64::from(x87_quiet(e.mem.f32(process_info + 0x10)));
    if scale != 1.0 {
        for offset in 0..8 {
            scale_float(e, info + 0x10 + 4 * offset, scale);
        }
    }
    e.call(BINARY_COPY_MEMBERS, &args![this, clone, process_info]);
}

// Translated from 00cc2dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSpringAction::LoadBinary` (Xbox PDB): the `bhkBinaryAction` load (`00c8a7a0`),
/// then, for a stream not in PC byte order (`stream + 0x1E5` not 1), swaps the eleven
/// words at `+0x10..+0x38` of the cinfo that `CreateCinfo` (slot `0xB4`) answers.
pub fn bhk_spring_action_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(BINARY_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let info = e
            .with_stack(4, |e, created| e.vcall(this.addr(), 0xb4, &args![created]))
            .u32();
        swap_cinfo_words(e, info);
    }
}

// Translated from 00cc2e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSpringAction::SaveBinary` (Xbox PDB): takes the cinfo from `CreateCinfo` (slot
/// `0xB4`), swaps its eleven words at `+0x10..+0x38` when the stream is not in PC byte
/// order (`stream + 0x1E4` not 1), runs the `bhkBinaryAction` save (`00c8aac0`) and swaps
/// them back.
pub fn bhk_spring_action_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let info = e
        .with_stack(4, |e, created| e.vcall(this.addr(), 0xb4, &args![created]))
        .u32();
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_cinfo_words(e, info);
    }
    e.call(BINARY_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_cinfo_words(e, info);
    }
}

// Translated from 00cc2f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the default cinfo, built by `CreateCinfo`): `ECX` = the cinfo. Clears the
/// words at `+0`, `+4` and `+8`, the eight `float`s at `+0x10..+0x2C` become 0.0, `+0x30`
/// 1.0, `+0x34` and `+0x38` the `float` globals `01013974` and `0101e2bc`, and the bytes
/// at `+0x3C` and `+0x3D` 1. Answers the cinfo.
pub fn fn_00cc2f40(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    for offset in [0, 4, 8, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
        e.mem.set_u32(me + offset, 0);
    }
    e.mem.set_f32(me + 0x30, 1.0);
    for (offset, global) in [(0x34, DEFAULT_FLOATS[0]), (0x38, DEFAULT_FLOATS[1])] {
        let bits = x87_quiet_bits(e.mem.u32(global));
        e.mem.set_u32(me + offset, bits);
    }
    e.mem.set_u8(me + 0x3c, 1);
    e.mem.set_u8(me + 0x3d, 1);
    this
}

// Translated from 00cc2f90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2f90 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSpringAction::CreateCinfo` (Xbox PDB), vtable slot `0xB4`: `created` (a byte) is
/// set to whether `pInfo` was empty. If so, a 0x50-byte block is taken from the pool
/// (`00aa3e40`; the result is not checked) and the cinfo is the 16-byte aligned address
/// inside it (the pad count is kept in the byte before it); it is built by `00cc2f40`,
/// becomes `pInfo` and, when the Havok action at `+8` exists, is filled from it (`00cc2b60`).
/// Answers `pInfo`. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_spring_action_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let empty = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, 0x50u32]).u32();
        let pad = 0x10 - (block & 0xf) as u8;
        let aligned = block.wrapping_add(u32::from(pad));
        e.mem.set_u8(aligned.wrapping_sub(1), pad);
        let info = fn_00cc2f40(e, Ptr::new(aligned)).addr();
        let has_action = e.mem.u32(me + 8) != 0;
        e.mem.set_u32(me + 0xc, info);
        if has_action {
            fn_00cc2b60(e, this, info);
        }
    }
    let info = e.mem.u32(me + 0xc);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00cc3050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3050 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkSpringAction::Init` (Xbox PDB), vtable slot `0xB0`: with the allocation-scope word
/// at 0x10, allocates 0x60 bytes through the router (size word at `+4`; the result is not
/// checked) and constructs the `hkpSpringAction` there (`00d4d340`) from the cinfo's `+4`
/// and `+8` Havok objects and 0. Its `float`s at `+0x50`, `+0x54` and `+0x58` take the
/// cinfo's `+0x30`, `+0x34` and `+0x38`, its flag bytes at `+0x5C` and `+0x5D` the cinfo's
/// `+0x3C` and `+0x3D`, and `setPositionsInBodySpace` (`00d4d410`) gets copies of the
/// cinfo's vectors at `+0x10` and `+0x20`. The new Havok action goes to slot `0x8C`, is
/// released (`00c905b0`), and slot `0xBC` is called with the cinfo.
pub fn bhk_spring_action_init(e: &mut Engine, this: Ptr, info: u32) {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let block = allocate_sized(e, 0x60);
    let first = e.mem.u32(info + 4);
    let second = e.mem.u32(info + 8);
    let action = e
        .call(HK_SPRING_CONSTRUCT, &args![block, first, second, 0u32])
        .u32();
    for (from, to) in [(0x34, 0x54), (0x38, 0x58), (0x30, 0x50)] {
        let bits = x87_quiet_bits(e.mem.u32(info + from));
        e.mem.set_u32(action + to, bits);
    }
    let flag = e.mem.u8(info + 0x3c);
    e.mem.set_u8(action + 0x5c, flag);
    let flag = e.mem.u8(info + 0x3d);
    e.mem.set_u8(action + 0x5d, flag);
    e.with_stack(0x20, |e, vectors| {
        let second_vector = vectors.addr() + 0x10;
        for lane in 0..4 {
            let low = e.mem.u32(info + 0x10 + 4 * lane);
            let high = e.mem.u32(info + 0x20 + 4 * lane);
            e.mem.set_u32(vectors.addr() + 4 * lane, low);
            e.mem.set_u32(second_vector + 4 * lane, high);
        }
        e.call(HK_SET_POSITIONS, &args![action, vectors, second_vector]);
    });
    e.vcall(me, 0x8c, &args![action]);
    e.call(REMOVE_REFERENCE, &args![action]);
    e.vcall(me, 0xbc, &args![info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00cc31c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc31c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSpringAction::GetViewerStrings` (Xbox PDB), vtable slot `0x60`: with the
/// allocation-scope word at 0x10, the `bhkBinaryAction` strings (`00c8ad40`, `(this,
/// strings)`), then the viewer string of the class `NiRTTI` (the global at `012685b0`).
pub fn bhk_spring_action_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(BINARY_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(SPRING_ACTION_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    e.mem.set_u32(word, saved);
}
// Translated from 00c8a7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBinaryAction::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c8a720`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_binary_action_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c8_a720;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc2b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSpringAction::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc2ac0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_spring_action_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_2ac0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00c8a790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBinaryAction::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8198.
pub fn bhk_binary_action_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8198
}

// Translated from 00cc2b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSpringAction::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_85b0.
pub fn bhk_spring_action_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_85b0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c8a790, bhk_binary_action_get_r_t_t_i() -> u32),
        entry!(0x00cc2b30, bhk_spring_action_get_r_t_t_i() -> u32),
        entry!(0x00c8a7e0, bhk_binary_action_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc2b40, bhk_spring_action_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc2ac0, bhk_spring_action_destruct(Ptr)),
        entry!(0x00cc2b60, fn_00cc2b60(Ptr, u32)),
        entry!(0x00cc2bc0, fn_00cc2bc0() -> u32),
        entry!(0x00cc2c60, bhk_spring_action_create_clone(Ptr, u32) -> u32),
        entry!(0x00cc2d20, bhk_spring_action_copy_members(Ptr, u32, u32)),
        entry!(0x00cc2dc0, bhk_spring_action_load_binary(Ptr, u32)),
        entry!(0x00cc2e50, bhk_spring_action_save_binary(Ptr, u32)),
        entry!(0x00cc2f40, fn_00cc2f40(Ptr) -> Ptr),
        entry!(0x00cc2f90, bhk_spring_action_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00cc3050, bhk_spring_action_init(Ptr, u32)),
        entry!(0x00cc31c0, bhk_spring_action_get_viewer_strings(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_binary_action_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c8_a720, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c8a7e0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c8a7e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00c8_a720), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_spring_action_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_2ac0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cc2b40, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc2b40, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_2ac0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_binary_action_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c8_a790, &args![]).u32(), 0x0126_8198);
    }

    #[test]
    fn bhk_spring_action_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_2b30, &args![]).u32(), 0x0126_85b0);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::super::bhklimitedhingeconstraint::support::*;
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    #[test]
    fn destruct_sets_the_vtable_kills_and_counts_down() {
        let mut e = engine();
        e.set_global(COUNT_SPRING_ACTION, 2u32);
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_KILL, 0, &log);
        record(&mut e, BINARY_DESTRUCT, 0, &log);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc_2ac0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_SPRING_ACTION);
        assert_eq!(e.global::<u32>(COUNT_SPRING_ACTION), 1);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [SERIALIZABLE_KILL, BINARY_DESTRUCT]);
        assert_eq!(calls_to(&log, SERIALIZABLE_KILL), [[this]]);
        assert_eq!(calls_to(&log, BINARY_DESTRUCT), [[this]]);
    }

    #[test]
    fn fn_00cc2b60_copies_the_havok_floats_vectors_and_flags_when_there_is_an_action() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, BINARY_COPY_CINFO, 0, &log);
        record(&mut e, COPY_VECTOR, 0, &log);
        let havok = e.mem.alloc(0x60);
        e.mem.set_f32(havok + 0x50, 1.5);
        // A signalling NaN comes out quiet through FLD / FSTP.
        e.mem.set_u32(havok + 0x54, 0x7fa0_0000);
        e.mem.set_f32(havok + 0x58, 3.5);
        e.mem.set_u8(havok + 0x5c, 7);
        e.mem.set_u8(havok + 0x5d, 9);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, havok);
        let info = e.mem.alloc(0x50);
        e.call(0x00cc_2b60, &args![this, info]);
        assert_eq!(calls_to(&log, BINARY_COPY_CINFO), [[this, info]]);
        assert_eq!(
            calls_to(&log, COPY_VECTOR),
            [[info + 0x10, havok + 0x30], [info + 0x20, havok + 0x40]]
        );
        assert_eq!(e.mem.f32(info + 0x30), 1.5);
        assert_eq!(e.mem.u32(info + 0x34), 0x7fe0_0000);
        assert_eq!(e.mem.f32(info + 0x38), 3.5);
        assert_eq!(e.mem.u8(info + 0x3c), 7);
        assert_eq!(e.mem.u8(info + 0x3d), 9);
        // Without a Havok action only the base copy runs.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00cc_2b60, &args![this, info]);
        assert_eq!(calls_to(&log, BINARY_COPY_CINFO).len(), 2);
        assert_eq!(calls_to(&log, COPY_VECTOR).len(), 2);
        // A null action only runs the base copy as well.
        e.call(0x00cc_2b60, &args![0u32, info]);
        assert_eq!(calls_to(&log, BINARY_COPY_CINFO)[2], [0, info]);
        assert_eq!(calls_to(&log, COPY_VECTOR).len(), 2);
    }

    #[test]
    fn fn_00cc2bc0_builds_the_object_and_counts_it() {
        let mut e = engine();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 0xc, 7);
        let log: Calls = Rc::default();
        record(&mut e, ALLOCATE, object, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        assert_eq!(e.call(0x00cc_2bc0, &args![]).u32(), object);
        assert_eq!(calls_to(&log, ALLOCATE), [[0x10]]);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT), [[object]]);
        assert_eq!(e.mem.u32(object), VTABLE_SPRING_ACTION);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_ACTION), 1);
        assert_eq!(e.global::<u32>(COUNT_BINARY_ACTION), 1);
        assert_eq!(e.global::<u32>(COUNT_SPRING_ACTION), 1);
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cc_2bc0, &args![]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_clone_constructs_and_copies_inside_the_clone_lock() {
        let mut e = engine();
        let clone = e.mem.alloc(0x10);
        let log: Calls = Rc::default();
        record(&mut e, CLONE_CRIT_LOCK, 0, &log);
        record(&mut e, CLONE_CRIT_UNLOCK, 0, &log);
        record(&mut e, ALLOCATE, clone, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        record(&mut e, 0x0900_00d0, 0, &log);
        let this = vobject(&mut e, 0x10, &[(0xd0, 0x0900_00d0)]);
        assert_eq!(e.call(0x00cc_2c60, &args![this, 0x99u32]).u32(), clone);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                CLONE_CRIT_LOCK,
                ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                0x0900_00d0,
                CLONE_CRIT_UNLOCK
            ]
        );
        assert_eq!(calls_to(&log, ALLOCATE), [[0x10]]);
        assert_eq!(e.mem.u32(clone), VTABLE_SPRING_ACTION);
        assert_eq!(calls_to(&log, 0x0900_00d0), [[this, clone, 0x99]]);
        // A failed allocation copies into nothing and answers 0.
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cc_2c60, &args![this, 0x98u32]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&log, 0x0900_00d0)[1], [this, 0, 0x98]);
    }

    #[test]
    fn copy_members_scales_the_eight_floats_unless_the_scale_is_one() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, BINARY_COPY_MEMBERS, 0, &log);
        let info = e.mem.alloc(0x50);
        e.register_double(0x0900_00b4, move |_, _| ret(info));
        let this = vobject(&mut e, 0x10, &[(0xb4, 0x0900_00b4)]);
        let process_info = e.mem.alloc(0x20);
        for lane in 0..8 {
            e.mem.set_f32(info + 0x10 + 4 * lane, 1.0 + lane as f32);
        }
        e.mem.set_f32(info + 0x30, 5.0);
        // A scale of exactly 1.0 leaves the cinfo alone.
        e.mem.set_f32(process_info + 0x10, 1.0);
        e.call(0x00cc_2d20, &args![this, 0x77u32, process_info]);
        for lane in 0..8 {
            assert_eq!(e.mem.f32(info + 0x10 + 4 * lane), 1.0 + lane as f32);
        }
        assert_eq!(
            calls_to(&log, BINARY_COPY_MEMBERS),
            [[this, 0x77, process_info]]
        );
        // Any other scale multiplies the eight floats at +0x10..+0x2C and nothing else.
        e.mem.set_f32(process_info + 0x10, 2.0);
        e.call(0x00cc_2d20, &args![this, 0x77u32, process_info]);
        for lane in 0..8 {
            assert_eq!(e.mem.f32(info + 0x10 + 4 * lane), 2.0 * (1.0 + lane as f32));
        }
        assert_eq!(e.mem.f32(info + 0x30), 5.0);
        assert_eq!(calls_to(&log, BINARY_COPY_MEMBERS).len(), 2);
        // A NaN is not 1.0: the floats become NaN.
        e.mem.set_u32(process_info + 0x10, 0x7fc0_0000);
        e.call(0x00cc_2d20, &args![this, 0x77u32, process_info]);
        assert!(e.mem.f32(info + 0x10).is_nan());
        assert!(e.mem.f32(info + 0x2c).is_nan());
    }

    #[test]
    fn load_and_save_binary_swap_the_eleven_cinfo_words_only_for_a_foreign_stream() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, BINARY_LOAD_BINARY, 0, &log);
        record(&mut e, BINARY_SAVE_BINARY, 0, &log);
        let info = e.mem.alloc(0x50);
        e.register_double(0x0900_0080, move |_, _| ret(info));
        let this = vobject(&mut e, 0x10, &[(0xb4, 0x0900_0080)]);
        let stream = e.mem.alloc(0x300);
        for index in 0..11 {
            e.mem.set_u32(info + 0x10 + 4 * index, 0x0102_0304);
        }
        e.mem.set_u32(info + 0xc, 0x0102_0304);
        e.mem.set_u32(info + 0x3c, 0x0102_0304);
        // PC byte order: untouched.
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00cc_2dc0, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0x10), 0x0102_0304);
        assert_eq!(calls_to(&log, BINARY_LOAD_BINARY), [[this, stream]]);
        // Foreign order: the eleven words are swapped after the base load.
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00cc_2dc0, &args![this, stream]);
        for index in 0..11 {
            assert_eq!(e.mem.u32(info + 0x10 + 4 * index), 0x0403_0201);
        }
        assert_eq!(e.mem.u32(info + 0xc), 0x0102_0304);
        assert_eq!(e.mem.u32(info + 0x3c), 0x0102_0304);
        // Save: swapped for the base save and swapped back.
        e.mem.set_u8(stream + 0x1e4, 0);
        e.register_double(BINARY_SAVE_BINARY, move |e, _| {
            assert_eq!(e.mem.u32(info + 0x38), 0x0102_0304);
            Ret::default()
        });
        e.call(0x00cc_2e50, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0x38), 0x0403_0201);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.register_double(BINARY_SAVE_BINARY, move |e, _| {
            assert_eq!(e.mem.u32(info + 0x38), 0x0403_0201);
            Ret::default()
        });
        e.call(0x00cc_2e50, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0x38), 0x0403_0201);
    }

    #[test]
    fn fn_00cc2f40_writes_the_default_cinfo() {
        let mut e = engine();
        e.set_global(DEFAULT_FLOATS[0], 0.5f32);
        e.mem.set_u32(DEFAULT_FLOATS[1], 0x7fa0_0000);
        let info = e.mem.alloc(0x50);
        for offset in (0..0x40).step_by(4) {
            e.mem.set_u32(info + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00cc_2f40, &args![info]).u32(), info);
        for offset in [0, 4, 8, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(info + offset), 0, "offset {offset:#x}");
        }
        // +0xC is not written.
        assert_eq!(e.mem.u32(info + 0xc), 0xdead_beef);
        assert_eq!(e.mem.f32(info + 0x30), 1.0);
        assert_eq!(e.mem.f32(info + 0x34), 0.5);
        assert_eq!(e.mem.u32(info + 0x38), 0x7fe0_0000);
        assert_eq!(e.mem.u8(info + 0x3c), 1);
        assert_eq!(e.mem.u8(info + 0x3d), 1);
    }

    #[test]
    fn create_cinfo_aligns_the_block_builds_the_default_and_copies_from_the_action() {
        let mut e = engine();
        e.set_global(DEFAULT_FLOATS[0], 0.5f32);
        e.set_global(DEFAULT_FLOATS[1], 0.25f32);
        let base = e.mem.alloc(0x70);
        let block = base + 4;
        let log: Calls = Rc::default();
        record(&mut e, POOL_ALLOCATE, block, &log);
        record(&mut e, BINARY_COPY_CINFO, 0, &log);
        record(&mut e, COPY_VECTOR, 0, &log);
        let this = e.mem.alloc(0x10);
        let created = e.mem.alloc(4);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
        // No Havok action: nothing is copied.
        let info = e.call(0x00cc_2f90, &args![this, created]).u32();
        let pad = 0x10 - (block & 0xf);
        assert_eq!(info, block + pad);
        assert_eq!(e.mem.u8(info - 1), pad as u8);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_OBJECT, 0x50]]);
        assert_eq!(e.mem.f32(info + 0x30), 1.0);
        assert_eq!(e.mem.f32(info + 0x34), 0.5);
        assert_eq!(e.mem.u32(this + 0xc), info);
        assert!(calls_to(&log, BINARY_COPY_CINFO).is_empty());
        assert_eq!(e.mem.u32(word), 0x77);
        // The second time it is kept.
        assert_eq!(e.call(0x00cc_2f90, &args![this, created]).u32(), info);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&log, POOL_ALLOCATE).len(), 1);
        // With a Havok action the cinfo is filled from it.
        e.mem.set_u32(this + 0xc, 0);
        let havok = e.mem.alloc(0x60);
        e.mem.set_u32(this + 8, havok);
        e.call(0x00cc_2f90, &args![this, created]);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, BINARY_COPY_CINFO), [[this, info]]);
    }

    #[test]
    fn init_builds_the_havok_action_from_the_cinfo_and_releases_it() {
        let (mut e, word, block) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, 0x0900_0070, 0, &log);
        record(&mut e, 0x0900_0074, 0, &log);
        let action = e.mem.alloc(0x70);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let capture = Rc::clone(&seen);
        e.register_double(HK_SPRING_CONSTRUCT, move |_, a| {
            capture.borrow_mut().push(a.to_vec());
            ret(action)
        });
        let positions = Rc::clone(&seen);
        e.register_double(HK_SET_POSITIONS, move |e, a| {
            let mut words = vec![a[0]];
            words.extend((0..4).map(|lane| e.mem.u32(a[1] + 4 * lane)));
            words.extend((0..4).map(|lane| e.mem.u32(a[2] + 4 * lane)));
            positions.borrow_mut().push(words);
            Ret::default()
        });
        let this = vobject(&mut e, 0x10, &[(0x8c, 0x0900_0070), (0xbc, 0x0900_0074)]);
        let info = e.mem.alloc(0x50);
        e.mem.set_u32(info + 4, 0x6000);
        e.mem.set_u32(info + 8, 0x6100);
        for lane in 0..8 {
            e.mem.set_f32(info + 0x10 + 4 * lane, 1.0 + lane as f32);
        }
        e.mem.set_f32(info + 0x30, 10.0);
        e.mem.set_u32(info + 0x34, 0x7fa0_0000);
        e.mem.set_f32(info + 0x38, 12.0);
        e.mem.set_u8(info + 0x3c, 3);
        e.mem.set_u8(info + 0x3d, 4);
        e.call(0x00cc_3050, &args![this, info]);
        let mut positions = vec![action];
        positions.extend((0..8).map(|lane| (1.0 + lane as f32).to_bits()));
        assert_eq!(*seen.borrow(), [vec![block, 0x6000, 0x6100, 0], positions]);
        assert_eq!(e.mem.u16(block + 4), 0x60);
        assert_eq!(e.mem.f32(action + 0x50), 10.0);
        assert_eq!(e.mem.u32(action + 0x54), 0x7fe0_0000);
        assert_eq!(e.mem.f32(action + 0x58), 12.0);
        assert_eq!(e.mem.u8(action + 0x5c), 3);
        assert_eq!(e.mem.u8(action + 0x5d), 4);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x0900_0070, REMOVE_REFERENCE, 0x0900_0074]);
        assert_eq!(calls_to(&log, 0x0900_0070), [[this, action]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[action]]);
        assert_eq!(calls_to(&log, 0x0900_0074), [[this, info]]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn get_viewer_strings_adds_the_base_strings_then_the_class_name() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(SPRING_ACTION_RTTI, 0x1234u32);
        let log: Calls = Rc::default();
        record(&mut e, BINARY_GET_VIEWER_STRINGS, 0, &log);
        record(&mut e, NI_GET_VIEWER_STRING, 0x3000, &log);
        record(&mut e, STRINGS_GROW, 0, &log);
        let pushed: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let stored = Rc::clone(&pushed);
        e.register_double(STRINGS_SET_AT, move |e, a| {
            let text = e.mem.u32(a[2]);
            stored.borrow_mut().push((a[1], text));
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 16);
        e.call(0x00cc_31c0, &args![this, strings]);
        assert_eq!(calls_to(&log, BINARY_GET_VIEWER_STRINGS), [[this, strings]]);
        assert_eq!(calls_to(&log, NI_GET_VIEWER_STRING), [[0x1234]]);
        assert_eq!(*pushed.borrow(), [(0, 0x3000)]);
        assert!(calls_to(&log, STRINGS_GROW).is_empty());
        assert_eq!(e.mem.u32(word), 0x77);
    }
}
