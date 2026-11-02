//! `BSHavok/bhkmousespringaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use super::bhklimitedhingeconstraint::{
    allocate_sized, enter_scope, increment, push_viewer_string, ALLOCATE, POOL_ALLOCATE,
    POOL_OBJECT, REMOVE_REFERENCE,
};
#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkMouseSpringAction` (0x10 bytes, vtable `0101ffcc`; Xbox PDB name, RTTI-confirmed): a
/// `bhkUnaryAction` (the Havok action at `+8`, `pInfo` at `+0xC`). `pInfo` is a 0x50-byte
/// cinfo, 16-byte aligned inside a pool block: `+4` the Havok object the action works on,
/// two vectors at `+0x10` and `+0x20`, and four `float`s at `+0x30`, `+0x34`, `+0x38` and
/// `+0x3C` (the viewer strings name them "Damping", "Elasticity", "Max Force" and "Object
/// Damping"). The Havok action keeps the vectors at `+0x20` and `+0x30` and the `float`s
/// at `+0x40`, `+0x44`, `+0x48` and `+0x4C`.
///
/// The functions that allocate or tear down Havok objects raise the per-thread allocation
/// word at `TLS + 0x2b4` to 0x10 for their duration and put the old value back. The
/// exception-unwinding frames (`FS:[0]` registrations) and the `/GS` cookie check of the
/// functions marked `Translation incomplete` are not modeled.
const VTABLE_MOUSE_SPRING_ACTION: u32 = 0x0101_ffcc;
/// Vtables and instance counters of the base classes the constructor passes through:
/// `bhkAction` (`0102016c`, `01268154`) and `bhkUnaryAction` (`0102009c`, `01268118`),
/// both RTTI-confirmed; the counter of `bhkMouseSpringAction` itself is `01268124`.
const VTABLE_ACTION: u32 = 0x0102_016c;
const VTABLE_UNARY_ACTION: u32 = 0x0102_009c;
const COUNT_ACTION: u32 = 0x0126_8154;
const COUNT_UNARY_ACTION: u32 = 0x0126_8118;
const COUNT_MOUSE_SPRING_ACTION: u32 = 0x0126_8124;
/// The class's `NiRTTI` global (whose viewer string heads the list), the names of the four
/// `float` viewer strings (`010c38a0` "Damping", `010c5b28` "Elasticity", `010c5b18`
/// "Object Damping", `010c5b0c` "Max Force") and the four `float`s (0.5, 0.3, 250.0 and
/// 0.95) that the cinfo constructor (`004b4f40`) leaves at `+0x30..+0x3C` and that the local
/// cinfo of `GetViewerStrings` starts from.
const MOUSE_SPRING_RTTI: u32 = 0x0126_812c;
const NAME_DAMPING: u32 = 0x010c_38a0;
const NAME_ELASTICITY: u32 = 0x010c_5b28;
const NAME_OBJECT_DAMPING: u32 = 0x010c_5b18;
const NAME_MAX_FORCE: u32 = 0x010c_5b0c;
const DEFAULT_FLOATS: [u32; 4] = [0x0101_6248, 0x0101_7870, 0x0101_b2d0, 0x0101_ffc0];
/// `bhkUnaryAction` base methods: `~` (`00c86720`), `CopyCinfo` (`00c867d0`), `LoadBinary`
/// (`00c86790`), `SaveBinary` (`00c868b0`), `CopyMembers` (`00c86800`, `(clone,
/// process_info)`) and `GetViewerStrings` (`00c86b80`); `bhkSerializable::Kill` (`00c876a0`),
/// `CloneCritLock` (`00c876f0`), `CloneCritUnlock` (`00c87700`) and the `bhkRefObject`
/// constructor (`00c87c30`).
const UNARY_DESTRUCT: u32 = 0x00c8_6720;
const UNARY_COPY_CINFO: u32 = 0x00c8_67d0;
const UNARY_LOAD_BINARY: u32 = 0x00c8_6790;
const UNARY_SAVE_BINARY: u32 = 0x00c8_68b0;
const UNARY_COPY_MEMBERS: u32 = 0x00c8_6800;
const UNARY_GET_VIEWER_STRINGS: u32 = 0x00c8_6b80;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_CRIT_LOCK: u32 = 0x00c8_76f0;
const CLONE_CRIT_UNLOCK: u32 = 0x00c8_7700;
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// Copy of a four-`float` vector (`004b4db0`, `ECX` = destination, one word: the source)
/// and the cinfo constructor (`004b4f40`, `ECX` = the cinfo, answers it).
const COPY_VECTOR: u32 = 0x004b_4db0;
const CINFO_CONSTRUCT: u32 = 0x004b_4f40;
/// Pool free (`00aa4060`, thiscall on the pool object, one word: the block).
const POOL_FREE: u32 = 0x00aa_4060;
/// `hkpMouseSpringAction::hkpMouseSpringAction` (`00cbb560`, `ECX` = the block, seven
/// words: pointers to the vector at `+0x10` and to the vector at `+0x20`, the `float` bits
/// of `+0x30`, `+0x34` and `+0x3C`, the Havok object and 0) and its `setMaxRelativeForce`
/// (`00cbb1b0`, one word of `float` bits).
const HK_MOUSE_SPRING_CONSTRUCT: u32 = 0x00cb_b560;
const HK_SET_MAX_RELATIVE_FORCE: u32 = 0x00cb_b1b0;
/// `NiGetViewerString` for an `NiRTTI` (`00a7d330`, cdecl, one word) and for a `float`
/// (`00a7d550`, cdecl `(name, bits)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;

// Translated from 00c86cb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c86cb0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMouseSpringAction::~bhkMouseSpringAction` (Xbox PDB): sets the class vtable, runs
/// `bhkSerializable::Kill` (`00c876a0`), takes one off the counter `01268124` and runs the
/// `bhkUnaryAction` destructor (`00c86720`).
pub fn bhk_mouse_spring_action_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_MOUSE_SPRING_ACTION);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNT_MOUSE_SPRING_ACTION);
    e.mem
        .set_u32(COUNT_MOUSE_SPRING_ACTION, count.wrapping_sub(1));
    e.call(UNARY_DESTRUCT, &args![this]);
}

// Translated from 00c86d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMouseSpringAction::CopyCinfo` (Xbox PDB; called by `CreateCinfo`, `00c87100`, and
/// `GetViewerStrings`): runs `bhkUnaryAction`'s `CopyCinfo` (`00c867d0`, `(this, info)`).
/// With a Havok action, `info + 0x30`, `+0x34` and `+0x3C` take its `float`s at `+0x40`,
/// `+0x44` and `+0x4C`, the vector at `+0x20` goes to `info + 0x10` and the one at `+0x30`
/// to `info + 0x20` (`004b4db0`), and `info + 0x38` takes its `float` at `+0x48`. Without
/// one (or without the action) the `float`s at `+0x30`, `+0x34`, `+0x3C` are 0.0 and the
/// eight words at `+0x10..+0x2C` and the word at `+0x38` are cleared.
pub fn bhk_mouse_spring_action_copy_cinfo(e: &mut Engine, this: Ptr, info: u32) {
    e.call(UNARY_COPY_CINFO, &args![this, info]);
    let havok = havok_action(e, this);
    for (from, to) in [(0x40, 0x30), (0x44, 0x34), (0x4c, 0x3c)] {
        let bits = if havok != 0 {
            x87_quiet_bits(e.mem.u32(havok + from))
        } else {
            0
        };
        e.mem.set_u32(info + to, bits);
    }
    if havok != 0 {
        e.call(COPY_VECTOR, &args![info + 0x10, havok + 0x20]);
        e.call(COPY_VECTOR, &args![info + 0x20, havok + 0x30]);
        let bits = x87_quiet_bits(e.mem.u32(havok + 0x48));
        e.mem.set_u32(info + 0x38, bits);
    } else {
        for offset in [0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x38] {
            e.mem.set_u32(info + offset, 0);
        }
    }
}

/// The Havok action at `+8` of the action, 0 for a null action or none.
fn havok_action(e: &Engine, this: Ptr) -> u32 {
    if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    }
}

/// Constructs a `bhkMouseSpringAction` in `object`: the `bhkRefObject` constructor
/// (`00c87c30`), then the `bhkAction`, `bhkUnaryAction` and `bhkMouseSpringAction` vtables
/// in turn with their counters up by one, and `+0xC` cleared.
fn construct(e: &mut Engine, object: u32) {
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_ACTION);
    e.mem.set_u32(object + 0xc, 0);
    increment(e, COUNT_ACTION);
    e.mem.set_u32(object, VTABLE_UNARY_ACTION);
    increment(e, COUNT_UNARY_ACTION);
    e.mem.set_u32(object, VTABLE_MOUSE_SPRING_ACTION);
    increment(e, COUNT_MOUSE_SPRING_ACTION);
}

// Translated from 00c86de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c86de0 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates 0x10 bytes (`00aa13e0`) and, when that succeeded, constructs a
/// `bhkMouseSpringAction` there; answers the object, or 0. Takes no arguments (map name
/// unknown).
pub fn fn_00c86de0(e: &mut Engine) -> u32 {
    let object = e.call(ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    construct(e, object);
    object
}

/// Byte-swaps the twelve words at `+0x10..+0x3C` of `info`.
fn swap_cinfo_words(e: &mut Engine, info: u32) {
    for index in 0..12 {
        let word = e.mem.u32(info + 0x10 + 4 * index);
        e.mem.set_u32(info + 0x10 + 4 * index, word.swap_bytes());
    }
}

// Translated from 00c86e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMouseSpringAction::LoadBinary` (Xbox PDB): the `bhkUnaryAction` load (`00c86790`),
/// then, for a stream not in PC byte order (`stream + 0x1E5` not 1), swaps the twelve
/// words at `+0x10..+0x3C` of the cinfo that `CreateCinfo` (slot `0xB4`) answers.
pub fn bhk_mouse_spring_action_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(UNARY_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let info = e
            .with_stack(4, |e, created| e.vcall(this.addr(), 0xb4, &args![created]))
            .u32();
        swap_cinfo_words(e, info);
    }
}

// Translated from 00c86f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMouseSpringAction::SaveBinary` (Xbox PDB): takes the cinfo from `CreateCinfo` (slot
/// `0xB4`), swaps its twelve words at `+0x10..+0x3C` when the stream is not in PC byte
/// order (`stream + 0x1E4` not 1), runs the `bhkUnaryAction` save (`00c868b0`) and swaps
/// them back.
pub fn bhk_mouse_spring_action_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let info = e
        .with_stack(4, |e, created| e.vcall(this.addr(), 0xb4, &args![created]))
        .u32();
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_cinfo_words(e, info);
    }
    e.call(UNARY_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_cinfo_words(e, info);
    }
}

// Translated from 00c87010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c87010 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMouseSpringAction::CreateClone` (Xbox PDB), vtable slot `0x48`: inside the clone
/// lock (`00c876f0` / `00c87700`) and with the allocation-scope word at 0x10, allocates 0x10
/// bytes (`00aa13e0`) and constructs a `bhkMouseSpringAction` there when that succeeded,
/// then copies the members (`00c86800`, `(clone, process_info)`). Answers the clone (0 when
/// the allocation failed).
pub fn bhk_mouse_spring_action_create_clone(e: &mut Engine, this: Ptr, process_info: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_CRIT_LOCK, &args![]);
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let clone = if block != 0 {
        construct(e, block);
        block
    } else {
        0
    };
    e.call(UNARY_COPY_MEMBERS, &args![this, clone, process_info]);
    e.call(CLONE_CRIT_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00c87100 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c87100 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMouseSpringAction::CreateCinfo` (Xbox PDB), vtable slot `0xB4`: `created` (a byte)
/// is set to whether `pInfo` was empty. If so, a 0x50-byte block is taken from the pool
/// (`00aa3e40`; the result is not checked) and the cinfo is the 16-byte aligned address
/// inside it (the pad count is kept in the byte before it); it is constructed
/// (`004b4f40`), becomes `pInfo` and, when the Havok action at `+8` exists, is filled
/// from it (`CopyCinfo`, `00c86d20`). Answers `pInfo`. The allocation-scope word is 0x10
/// meanwhile.
pub fn bhk_mouse_spring_action_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let empty = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, 0x50u32]).u32();
        let pad = 0x10 - (block & 0xf) as u8;
        let aligned = block.wrapping_add(u32::from(pad));
        e.mem.set_u8(aligned.wrapping_sub(1), pad);
        let info = e.call(CINFO_CONSTRUCT, &args![aligned]).u32();
        let has_action = e.mem.u32(me + 8) != 0;
        e.mem.set_u32(me + 0xc, info);
        if has_action {
            bhk_mouse_spring_action_copy_cinfo(e, this, info);
        }
    }
    let info = e.mem.u32(me + 0xc);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00c871c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c871c0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name (vtable slot `0xA4` of the `bhkMouseSpringAction` and `bhkMotorAction`
/// vtables): when the low byte of `flag` is not 0, gives the pool block of `pInfo`
/// (`+0xC`; the aligned address minus the pad count kept in the byte before it) back to
/// the pool (`00aa4060` on the pool object `011f6238`) if there is one, and clears `+0xC`.
/// The allocation-scope word is 0x10 meanwhile.
pub fn fn_00c871c0(e: &mut Engine, this: Ptr, flag: u32) {
    let (word, saved) = enter_scope(e);
    if flag as u8 != 0 {
        let info = e.mem.u32(this.addr() + 0xc);
        if info != 0 {
            let pad = u32::from(e.mem.u8(info.wrapping_sub(1)));
            e.call(POOL_FREE, &args![POOL_OBJECT, info.wrapping_sub(pad)]);
        }
        e.mem.set_u32(this.addr() + 0xc, 0);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c87260 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c87260 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkMouseSpringAction::Init` (Xbox PDB), vtable slot `0xB0`: with the allocation-scope
/// word at 0x10, allocates 0x60 bytes through the router (size word at `+4`; the result is
/// not checked) and constructs the `hkpMouseSpringAction` there (`00cbb560`) from copies
/// of the cinfo's vectors at `+0x10` and `+0x20`, its `float`s at `+0x30`, `+0x34` and
/// `+0x3C`, its `+4` Havok object and 0, then sets the maximum relative force from the
/// `float` at `+0x38` (`00cbb1b0`). The new Havok action goes to slot `0x8C`, is released
/// (`00c905b0`), and slot `0xBC` is called with the cinfo.
pub fn bhk_mouse_spring_action_init(e: &mut Engine, this: Ptr, info: u32) {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let block = allocate_sized(e, 0x60);
    let hk_object = e.mem.u32(info + 4);
    let first = x87_quiet_bits(e.mem.u32(info + 0x30));
    let second = x87_quiet_bits(e.mem.u32(info + 0x34));
    let third = x87_quiet_bits(e.mem.u32(info + 0x3c));
    let force = x87_quiet_bits(e.mem.u32(info + 0x38));
    let action = e.with_stack(0x20, |e, vectors| {
        let second_vector = vectors.addr() + 0x10;
        for lane in 0..4 {
            let low = e.mem.u32(info + 0x10 + 4 * lane);
            let high = e.mem.u32(info + 0x20 + 4 * lane);
            e.mem.set_u32(vectors.addr() + 4 * lane, low);
            e.mem.set_u32(second_vector + 4 * lane, high);
        }
        e.call(
            HK_MOUSE_SPRING_CONSTRUCT,
            &args![
                block,
                vectors,
                second_vector,
                first,
                second,
                third,
                hk_object,
                0u32
            ],
        )
        .u32()
    });
    e.call(HK_SET_MAX_RELATIVE_FORCE, &args![action, force]);
    e.vcall(me, 0x8c, &args![action]);
    e.call(REMOVE_REFERENCE, &args![action]);
    e.vcall(me, 0xbc, &args![info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00c873d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c873d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMouseSpringAction::GetViewerStrings` (Xbox PDB), vtable slot `0x60`: with the
/// allocation-scope word at 0x10, the `bhkUnaryAction` strings (`00c86b80`), then the
/// viewer string of the class `NiRTTI` (the global at `0126812c`). A local cinfo is built
/// (words `+0` and `+4` cleared, the eight `float`s at `+0x10..+0x2C` 0.0, `+0x30..+0x3C`
/// the four defaults) and filled from the action (`CopyCinfo`, `00c86d20`; its other words
/// are left as the stack had them); its `float`s at `+0x30`, `+0x34`, `+0x3C` and `+0x38`
/// are added under "Damping", "Elasticity", "Object Damping" and "Max Force".
pub fn bhk_mouse_spring_action_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(UNARY_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(MOUSE_SPRING_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let values = e.with_stack(0x40, |e, local| {
        let info = local.addr();
        e.mem.set_u32(info, 0);
        e.mem.set_u32(info + 4, 0);
        for offset in [0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
            e.mem.set_u32(info + offset, 0);
        }
        for (index, default) in DEFAULT_FLOATS.into_iter().enumerate() {
            let bits = x87_quiet_bits(e.mem.u32(default));
            e.mem.set_u32(info + 0x30 + 4 * index as u32, bits);
        }
        bhk_mouse_spring_action_copy_cinfo(e, this, info);
        [0x30, 0x34, 0x3c, 0x38].map(|offset| x87_quiet_bits(e.mem.u32(info + offset)))
    });
    for (name, bits) in [
        NAME_DAMPING,
        NAME_ELASTICITY,
        NAME_OBJECT_DAMPING,
        NAME_MAX_FORCE,
    ]
    .into_iter()
    .zip(values)
    {
        let text = e.call(VIEWER_STRING_FLOAT, &args![name, bits]).u32();
        push_viewer_string(e, strings, text);
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c8_6cb0, bhk_mouse_spring_action_destruct(Ptr)),
        entry!(0x00c8_6d20, bhk_mouse_spring_action_copy_cinfo(Ptr, u32)),
        entry!(0x00c8_6de0, fn_00c86de0() -> u32),
        entry!(0x00c8_6e80, bhk_mouse_spring_action_load_binary(Ptr, u32)),
        entry!(0x00c8_6f10, bhk_mouse_spring_action_save_binary(Ptr, u32)),
        entry!(0x00c8_7010, bhk_mouse_spring_action_create_clone(Ptr, u32) -> u32),
        entry!(0x00c8_7100, bhk_mouse_spring_action_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00c8_71c0, fn_00c871c0(Ptr, u32)),
        entry!(0x00c8_7260, bhk_mouse_spring_action_init(Ptr, u32)),
        entry!(
            0x00c8_73d0,
            bhk_mouse_spring_action_get_viewer_strings(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod action_tests {
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
        e.set_global(COUNT_MOUSE_SPRING_ACTION, 2u32);
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_KILL, 0, &log);
        record(&mut e, UNARY_DESTRUCT, 0, &log);
        let this = e.mem.alloc(0x10);
        e.call(0x00c8_6cb0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_MOUSE_SPRING_ACTION);
        assert_eq!(e.global::<u32>(COUNT_MOUSE_SPRING_ACTION), 1);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [SERIALIZABLE_KILL, UNARY_DESTRUCT]);
        assert_eq!(calls_to(&log, SERIALIZABLE_KILL), [[this]]);
        assert_eq!(calls_to(&log, UNARY_DESTRUCT), [[this]]);
    }

    #[test]
    fn copy_cinfo_takes_the_vectors_and_floats_from_the_havok_action() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, UNARY_COPY_CINFO, 0, &log);
        record(&mut e, COPY_VECTOR, 0, &log);
        let havok = e.mem.alloc(0x50);
        e.mem.set_f32(havok + 0x40, 1.5);
        // A signalling NaN comes out quiet through FLD / FSTP.
        e.mem.set_u32(havok + 0x44, 0x7fa0_0000);
        e.mem.set_f32(havok + 0x48, 3.5);
        e.mem.set_f32(havok + 0x4c, 4.5);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, havok);
        let info = e.mem.alloc(0x50);
        e.call(0x00c8_6d20, &args![this, info]);
        assert_eq!(calls_to(&log, UNARY_COPY_CINFO), [[this, info]]);
        assert_eq!(
            calls_to(&log, COPY_VECTOR),
            [[info + 0x10, havok + 0x20], [info + 0x20, havok + 0x30]]
        );
        assert_eq!(e.mem.f32(info + 0x30), 1.5);
        assert_eq!(e.mem.u32(info + 0x34), 0x7fe0_0000);
        assert_eq!(e.mem.f32(info + 0x38), 3.5);
        assert_eq!(e.mem.f32(info + 0x3c), 4.5);
        // Without a Havok action the floats and the vector words are cleared.
        for offset in [
            0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c,
        ] {
            e.mem.set_u32(info + offset, 0xdead);
        }
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_6d20, &args![this, info]);
        assert_eq!(calls_to(&log, COPY_VECTOR).len(), 2);
        for offset in [
            0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c,
        ] {
            assert_eq!(e.mem.u32(info + offset), 0, "offset {offset:#x}");
        }
    }

    #[test]
    fn fn_00c86de0_builds_the_object_and_counts_it() {
        let mut e = engine();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 0xc, 7);
        let log: Calls = Rc::default();
        record(&mut e, ALLOCATE, object, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        assert_eq!(e.call(0x00c8_6de0, &args![]).u32(), object);
        assert_eq!(calls_to(&log, ALLOCATE), [[0x10]]);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT), [[object]]);
        assert_eq!(e.mem.u32(object), VTABLE_MOUSE_SPRING_ACTION);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_ACTION), 1);
        assert_eq!(e.global::<u32>(COUNT_UNARY_ACTION), 1);
        assert_eq!(e.global::<u32>(COUNT_MOUSE_SPRING_ACTION), 1);
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00c8_6de0, &args![]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn load_and_save_binary_swap_the_twelve_cinfo_words_only_for_a_foreign_stream() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, UNARY_LOAD_BINARY, 0, &log);
        record(&mut e, UNARY_SAVE_BINARY, 0, &log);
        let info = e.mem.alloc(0x50);
        e.register_double(0x0900_0080, move |_, _| ret(info));
        let this = vobject(&mut e, 0x10, &[(0xb4, 0x0900_0080)]);
        let stream = e.mem.alloc(0x300);
        for index in 0..12 {
            e.mem.set_u32(info + 0x10 + 4 * index, 0x0102_0304);
        }
        e.mem.set_u32(info + 0xc, 0x0102_0304);
        e.mem.set_u32(info + 0x40, 0x0102_0304);
        // PC byte order: untouched.
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00c8_6e80, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0x10), 0x0102_0304);
        assert_eq!(calls_to(&log, UNARY_LOAD_BINARY), [[this, stream]]);
        // Foreign order: the twelve words are swapped after the base load.
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00c8_6e80, &args![this, stream]);
        for index in 0..12 {
            assert_eq!(e.mem.u32(info + 0x10 + 4 * index), 0x0403_0201);
        }
        assert_eq!(e.mem.u32(info + 0xc), 0x0102_0304);
        assert_eq!(e.mem.u32(info + 0x40), 0x0102_0304);
        // Save: swapped for the base save and swapped back.
        e.mem.set_u8(stream + 0x1e4, 0);
        e.register_double(UNARY_SAVE_BINARY, move |e, _| {
            assert_eq!(e.mem.u32(info + 0x3c), 0x0102_0304);
            Ret::default()
        });
        e.call(0x00c8_6f10, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0x3c), 0x0403_0201);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.register_double(UNARY_SAVE_BINARY, move |e, _| {
            assert_eq!(e.mem.u32(info + 0x3c), 0x0403_0201);
            Ret::default()
        });
        e.call(0x00c8_6f10, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0x3c), 0x0403_0201);
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
        record(&mut e, UNARY_COPY_MEMBERS, 0, &log);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
        assert_eq!(e.call(0x00c8_7010, &args![0x5000u32, 0x99u32]).u32(), clone);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                CLONE_CRIT_LOCK,
                ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                UNARY_COPY_MEMBERS,
                CLONE_CRIT_UNLOCK
            ]
        );
        assert_eq!(calls_to(&log, ALLOCATE), [[0x10]]);
        assert_eq!(e.mem.u32(clone), VTABLE_MOUSE_SPRING_ACTION);
        assert_eq!(calls_to(&log, UNARY_COPY_MEMBERS), [[0x5000, clone, 0x99]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // A failed allocation copies into nothing and answers 0.
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00c8_7010, &args![0x5000u32, 0x98u32]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&log, UNARY_COPY_MEMBERS)[1], [0x5000, 0, 0x98]);
    }

    #[test]
    fn create_cinfo_aligns_the_block_constructs_it_and_copies_from_the_action() {
        let mut e = engine();
        let base = e.mem.alloc(0x70);
        let block = base + 4;
        let log: Calls = Rc::default();
        record(&mut e, POOL_ALLOCATE, block, &log);
        record(&mut e, UNARY_COPY_CINFO, 0, &log);
        record(&mut e, COPY_VECTOR, 0, &log);
        let constructed = Rc::clone(&log);
        e.register_double(CINFO_CONSTRUCT, move |_, a| {
            constructed.borrow_mut().push((CINFO_CONSTRUCT, a.to_vec()));
            ret(a[0])
        });
        let this = e.mem.alloc(0x10);
        let created = e.mem.alloc(4);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
        // No Havok action: nothing is copied.
        let info = e.call(0x00c8_7100, &args![this, created]).u32();
        let pad = 0x10 - (block & 0xf);
        assert_eq!(info, block + pad);
        assert_eq!(e.mem.u8(info - 1), pad as u8);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_OBJECT, 0x50]]);
        assert_eq!(calls_to(&log, CINFO_CONSTRUCT), [[info]]);
        assert_eq!(e.mem.u32(this + 0xc), info);
        assert!(calls_to(&log, UNARY_COPY_CINFO).is_empty());
        assert_eq!(e.mem.u32(word), 0x77);
        // The second time it is kept.
        assert_eq!(e.call(0x00c8_7100, &args![this, created]).u32(), info);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&log, POOL_ALLOCATE).len(), 1);
        // With a Havok action the cinfo is filled from it.
        e.mem.set_u32(this + 0xc, 0);
        let havok = e.mem.alloc(0x50);
        e.mem.set_u32(this + 8, havok);
        e.call(0x00c8_7100, &args![this, created]);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, UNARY_COPY_CINFO), [[this, info]]);
    }

    #[test]
    fn fn_00c871c0_gives_the_pool_block_back_only_when_asked() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, POOL_FREE, 0, &log);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
        let this = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x20);
        let info = base + 8;
        e.mem.set_u8(info - 1, 8);
        e.mem.set_u32(this + 0xc, info);
        e.call(0x00c8_71c0, &args![this, 0x0100u32]);
        assert_eq!(e.mem.u32(this + 0xc), info);
        assert!(calls_to(&log, POOL_FREE).is_empty());
        e.call(0x00c8_71c0, &args![this, 0x0101u32]);
        assert_eq!(calls_to(&log, POOL_FREE), [[POOL_OBJECT, base]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // Nothing to free: only +0xC is cleared again.
        e.call(0x00c8_71c0, &args![this, 1u32]);
        assert_eq!(calls_to(&log, POOL_FREE).len(), 1);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn init_builds_the_havok_action_from_the_cinfo_and_releases_it() {
        let (mut e, word, block) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, HK_SET_MAX_RELATIVE_FORCE, 0, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, 0x0900_0070, 0, &log);
        record(&mut e, 0x0900_0074, 0, &log);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let capture = Rc::clone(&seen);
        e.register_double(HK_MOUSE_SPRING_CONSTRUCT, move |e, a| {
            let mut words = vec![a[0]];
            words.extend((0..4).map(|lane| e.mem.u32(a[1] + 4 * lane)));
            words.extend((0..4).map(|lane| e.mem.u32(a[2] + 4 * lane)));
            words.extend(a[3..].iter().copied());
            capture.borrow_mut().push(words);
            ret(0x8800)
        });
        let this = vobject(&mut e, 0x10, &[(0x8c, 0x0900_0070), (0xbc, 0x0900_0074)]);
        let info = e.mem.alloc(0x50);
        e.mem.set_u32(info + 4, 0x6000);
        for lane in 0..8 {
            e.mem.set_f32(info + 0x10 + 4 * lane, 1.0 + lane as f32);
        }
        e.mem.set_f32(info + 0x30, 10.0);
        e.mem.set_u32(info + 0x34, 0x7fa0_0000);
        e.mem.set_f32(info + 0x38, 12.0);
        e.mem.set_f32(info + 0x3c, 13.0);
        e.call(0x00c8_7260, &args![this, info]);
        let mut expected = vec![block];
        expected.extend((0..8).map(|lane| (1.0 + lane as f32).to_bits()));
        expected.extend([10.0f32.to_bits(), 0x7fe0_0000, 13.0f32.to_bits(), 0x6000, 0]);
        assert_eq!(*seen.borrow(), [expected]);
        assert_eq!(e.mem.u16(block + 4), 0x60);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                HK_SET_MAX_RELATIVE_FORCE,
                0x0900_0070,
                REMOVE_REFERENCE,
                0x0900_0074
            ]
        );
        assert_eq!(
            calls_to(&log, HK_SET_MAX_RELATIVE_FORCE),
            [[0x8800, 12.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, 0x0900_0070), [[this, 0x8800]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0x8800]]);
        assert_eq!(calls_to(&log, 0x0900_0074), [[this, info]]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_the_four_floats_of_a_stack_cinfo() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(MOUSE_SPRING_RTTI, 0x1234u32);
        e.set_global(DEFAULT_FLOATS[0], 0.5f32);
        e.set_global(DEFAULT_FLOATS[1], 0.25f32);
        e.set_global(DEFAULT_FLOATS[2], 250.0f32);
        e.set_global(DEFAULT_FLOATS[3], 0.75f32);
        let log: Calls = Rc::default();
        record(&mut e, UNARY_GET_VIEWER_STRINGS, 0, &log);
        record(&mut e, NI_GET_VIEWER_STRING, 0x3000, &log);
        record(&mut e, STRINGS_GROW, 0, &log);
        record(&mut e, UNARY_COPY_CINFO, 0, &log);
        record(&mut e, COPY_VECTOR, 0, &log);
        let floats: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let named = Rc::clone(&floats);
        e.register_double(VIEWER_STRING_FLOAT, move |_, a| {
            named.borrow_mut().push((a[0], a[1]));
            ret(0x3000 + named.borrow().len() as u32)
        });
        let pushed: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let stored = Rc::clone(&pushed);
        e.register_double(STRINGS_SET_AT, move |e, a| {
            let text = e.mem.u32(a[2]);
            stored.borrow_mut().push((a[1], text));
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        // Without a Havok action CopyCinfo clears the floats (the defaults the local cinfo
        // starts from are overwritten).
        let this = e.mem.alloc(0x10);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 16);
        e.call(0x00c8_73d0, &args![this, strings]);
        assert_eq!(calls_to(&log, UNARY_GET_VIEWER_STRINGS), [[this, strings]]);
        assert_eq!(calls_to(&log, NI_GET_VIEWER_STRING), [[0x1234]]);
        assert_eq!(calls_to(&log, UNARY_COPY_CINFO).len(), 1);
        assert_eq!(
            *floats.borrow(),
            [
                (NAME_DAMPING, 0),
                (NAME_ELASTICITY, 0),
                (NAME_OBJECT_DAMPING, 0),
                (NAME_MAX_FORCE, 0)
            ]
        );
        assert_eq!(
            *pushed.borrow(),
            [
                (0, 0x3000),
                (1, 0x3001),
                (2, 0x3002),
                (3, 0x3003),
                (4, 0x3004)
            ]
        );
        assert_eq!(e.mem.u32(word), 0x77);
        // With one the floats come from it (+0x40, +0x44, +0x4C and +0x48).
        let havok = e.mem.alloc(0x50);
        for (offset, value) in [(0x40, 1.0f32), (0x44, 2.0), (0x48, 3.0), (0x4c, 4.0)] {
            e.mem.set_f32(havok + offset, value);
        }
        e.mem.set_u32(this + 8, havok);
        floats.borrow_mut().clear();
        e.call(0x00c8_73d0, &args![this, strings]);
        assert_eq!(
            *floats.borrow(),
            [
                (NAME_DAMPING, 1.0f32.to_bits()),
                (NAME_ELASTICITY, 2.0f32.to_bits()),
                (NAME_OBJECT_DAMPING, 4.0f32.to_bits()),
                (NAME_MAX_FORCE, 3.0f32.to_bits())
            ]
        );
    }
}
