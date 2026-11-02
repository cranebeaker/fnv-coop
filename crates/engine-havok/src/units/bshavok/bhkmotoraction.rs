//! `BSHavok/bhkmotoraction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use super::bhklimitedhingeconstraint::{
    allocate_sized, enter_scope, increment, push_viewer_string, ALLOCATE, POOL_ALLOCATE,
    POOL_OBJECT, REMOVE_REFERENCE,
};
#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cc3300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMotorAction::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc3270`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_motor_action_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_3270;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc32e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMotorAction::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_85bc.
pub fn bhk_motor_action_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_85bc
}

/// `bhkMotorAction` (0x10 bytes, vtable `010c9634`; Xbox PDB name, RTTI-confirmed): a
/// `bhkUnaryAction` (the Havok action at `+8`, `pInfo` at `+0xC`). `pInfo` is a 0x28-byte
/// cinfo, 16-byte aligned inside a 0x40-byte pool block: `+4` the Havok object the action
/// works on, `+0x10..+0x1C` a vector, `+0x20` the spin rate, `+0x24` the gain and `+0x28` a
/// byte set to 1. The Havok action keeps the vector at `+0x20`, the spin rate at `+0x30` and
/// the gain at `+0x34` (the viewer strings name the last two "SpinRate" and "Gain").
///
/// The functions that allocate or tear down Havok objects raise the per-thread allocation
/// word at `TLS + 0x2b4` to 0x10 for their duration and put the old value back. The
/// exception-unwinding frames (`FS:[0]` registrations) and the `/GS` cookie check of the
/// functions marked `Translation incomplete` are not modeled.
const VTABLE_MOTOR_ACTION: u32 = 0x010c_9634;
/// Vtables and instance counters of the base classes the constructor passes through:
/// `bhkAction` (`0102016c`, `01268154`) and `bhkUnaryAction` (`0102009c`, `01268118`),
/// both RTTI-confirmed; the counter of `bhkMotorAction` itself is `012685b8`.
const VTABLE_ACTION: u32 = 0x0102_016c;
const VTABLE_UNARY_ACTION: u32 = 0x0102_009c;
const COUNT_ACTION: u32 = 0x0126_8154;
const COUNT_UNARY_ACTION: u32 = 0x0126_8118;
const COUNT_MOTOR_ACTION: u32 = 0x0126_85b8;
/// The class's `NiRTTI` global (whose viewer string heads the list), the names of the two
/// `float` viewer strings (`010c96fc` "SpinRate", `01097d24` "Gain") and the `float` the
/// gain starts from.
const MOTOR_ACTION_RTTI: u32 = 0x0126_85bc;
const NAME_SPIN_RATE: u32 = 0x010c_96fc;
const NAME_GAIN: u32 = 0x0109_7d24;
const DEFAULT_GAIN: u32 = 0x0101_62c0;
/// The default vector the cinfo's `+0x10` takes when the action has no Havok object.
const DEFAULT_VECTOR: u32 = 0x0126_7e30;
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
/// Copy of a four-`float` vector (`004b4db0`, `ECX` = destination, one word: the source).
const COPY_VECTOR: u32 = 0x004b_4db0;
/// `hkpMotorAction::hkpMotorAction` (`00d4d720`, `ECX` = the block, four words: the Havok
/// object, a pointer to the vector, the spin rate and the gain bits).
const HK_MOTOR_ACTION_CONSTRUCT: u32 = 0x00d4_d720;
/// `NiGetViewerString` for an `NiRTTI` (`00a7d330`, cdecl, one word) and for a `float`
/// (`00a7d550`, cdecl `(name, bits)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;

// Translated from 00cc3270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3270 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMotorAction::~bhkMotorAction` (Xbox PDB): sets the class vtable, runs
/// `bhkSerializable::Kill` (`00c876a0`), takes one off the counter `012685b8` and runs the
/// `bhkUnaryAction` destructor (`00c86720`).
pub fn bhk_motor_action_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_MOTOR_ACTION);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNT_MOTOR_ACTION);
    e.mem.set_u32(COUNT_MOTOR_ACTION, count.wrapping_sub(1));
    e.call(UNARY_DESTRUCT, &args![this]);
}

// Translated from 00cc3320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the cinfo `info` from the action (called by `CreateCinfo`, `00cc3630`; map name
/// unknown). Runs `bhkUnaryAction`'s `CopyCinfo` (`00c867d0`, `(this, info)`), sets the byte
/// at `info + 0x28` and copies the Havok action's vector at `+0x20` (the default vector
/// `01267e30` when there is no Havok action) into `info + 0x10` (`004b4db0`). The cinfo's
/// `+0x24` and `+0x20` take the Havok action's `+0x34` and `+0x30` `float`s (0.0 without
/// one).
pub fn fn_00cc3320(e: &mut Engine, this: Ptr, info: u32) {
    e.call(UNARY_COPY_CINFO, &args![this, info]);
    e.mem.set_u8(info + 0x28, 1);
    let havok = havok_action(e, this);
    let vector = if havok != 0 {
        havok + 0x20
    } else {
        DEFAULT_VECTOR
    };
    e.call(COPY_VECTOR, &args![info + 0x10, vector]);
    let gain = if havok != 0 {
        x87_quiet_bits(e.mem.u32(havok + 0x34))
    } else {
        0
    };
    e.mem.set_u32(info + 0x24, gain);
    let spin = if havok != 0 {
        x87_quiet_bits(e.mem.u32(havok + 0x30))
    } else {
        0
    };
    e.mem.set_u32(info + 0x20, spin);
}

/// The Havok action at `+8` of the action, 0 for a null action or none.
fn havok_action(e: &Engine, this: Ptr) -> u32 {
    if this.addr() != 0 {
        e.mem.u32(this.addr() + 8)
    } else {
        0
    }
}

/// Constructs a `bhkMotorAction` in `object`: the `bhkRefObject` constructor (`00c87c30`),
/// then the `bhkAction`, `bhkUnaryAction` and `bhkMotorAction` vtables in turn with their
/// counters up by one, and `+0xC` cleared.
fn construct(e: &mut Engine, object: u32) {
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_ACTION);
    e.mem.set_u32(object + 0xc, 0);
    increment(e, COUNT_ACTION);
    e.mem.set_u32(object, VTABLE_UNARY_ACTION);
    increment(e, COUNT_UNARY_ACTION);
    e.mem.set_u32(object, VTABLE_MOTOR_ACTION);
    increment(e, COUNT_MOTOR_ACTION);
}

// Translated from 00cc33a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc33a0 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates 0x10 bytes (`00aa13e0`) and, when that succeeded, constructs a
/// `bhkMotorAction` there; answers the object, or 0. Takes no arguments (map name
/// unknown).
pub fn fn_00cc33a0(e: &mut Engine) -> u32 {
    let object = e.call(ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    construct(e, object);
    object
}

// Translated from 00cc3440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMotorAction::LoadBinary` (Xbox PDB): the `bhkUnaryAction` load (`00c86790`), then,
/// for a stream not in PC byte order (`stream + 0x1E5` not 1), swaps the six words at
/// `+0x10..+0x24` of the cinfo that `CreateCinfo` (slot `0xB4`) answers.
pub fn bhk_motor_action_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(UNARY_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let info = e
            .with_stack(4, |e, created| e.vcall(this.addr(), 0xb4, &args![created]))
            .u32();
        swap_cinfo_words(e, info);
    }
}

/// Byte-swaps the six words at `+0x10..+0x24` of `info`.
fn swap_cinfo_words(e: &mut Engine, info: u32) {
    for offset in [0x10, 0x14, 0x18, 0x1c, 0x20, 0x24] {
        let word = e.mem.u32(info + offset);
        e.mem.set_u32(info + offset, word.swap_bytes());
    }
}

// Translated from 00cc34a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMotorAction::SaveBinary` (Xbox PDB): takes the cinfo from `CreateCinfo` (slot
/// `0xB4`), swaps its six words at `+0x10..+0x24` when the stream is not in PC byte order
/// (`stream + 0x1E4` not 1), runs the `bhkUnaryAction` save (`00c868b0`) and swaps them
/// back.
pub fn bhk_motor_action_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
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

// Translated from 00cc3540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3540 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMotorAction::CreateClone` (Xbox PDB), vtable slot `0x48`: inside the clone lock
/// (`00c876f0` / `00c87700`) and with the allocation-scope word at 0x10, allocates 0x10
/// bytes (`00aa13e0`) and constructs a `bhkMotorAction` there when that succeeded, then
/// copies the members (`00c86800`, `(clone, process_info)`). Answers the clone (0 when
/// the allocation failed).
pub fn bhk_motor_action_create_clone(e: &mut Engine, this: Ptr, process_info: u32) -> u32 {
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

// Translated from 00cc3630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3630 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMotorAction::CreateCinfo` (Xbox PDB), vtable slot `0xB4`: `created` (a byte) is set
/// to whether `pInfo` was empty. If so, a 0x40-byte block is taken from the pool
/// (`00aa3e40`; the result is not checked) and the cinfo is the 16-byte aligned address
/// inside it (the pad count is kept in the byte before it). Its `+0`, `+4` are cleared,
/// `+0x10..+0x20` are 0.0, `+0x24` takes the `float` at `010162c0` and the byte at `+0x28`
/// is 1; `pInfo` takes it and, when the Havok action at `+8` exists, the cinfo is filled
/// from it (`00cc3320`). Answers `pInfo`. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_motor_action_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let empty = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, 0x40u32]).u32();
        let pad = 0x10 - (block & 0xf) as u8;
        let info = block.wrapping_add(u32::from(pad));
        e.mem.set_u8(info.wrapping_sub(1), pad);
        e.mem.set_u32(info, 0);
        e.mem.set_u32(info + 4, 0);
        for offset in [0x10, 0x14, 0x18, 0x1c, 0x20] {
            e.mem.set_u32(info + offset, 0);
        }
        let gain = x87_quiet_bits(e.mem.u32(DEFAULT_GAIN));
        e.mem.set_u32(info + 0x24, gain);
        e.mem.set_u8(info + 0x28, 1);
        let has_action = e.mem.u32(me + 8) != 0;
        e.mem.set_u32(me + 0xc, info);
        if has_action {
            fn_00cc3320(e, this, info);
        }
    }
    let info = e.mem.u32(me + 0xc);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00cc3710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3710 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkMotorAction::Init` (Xbox PDB), vtable slot `0xB0`: with the allocation-scope word at
/// 0x10, allocates 0x40 bytes through the router (size word at `+4`; the result is not
/// checked) and constructs the `hkpMotorAction` there (`00d4d720`) from the cinfo's `+4`
/// Havok object, a copy of its vector at `+0x10..+0x1C`, and its `+0x20` and `+0x24`
/// `float`s. The new Havok action goes to slot `0x8C`, is released
/// (`00c905b0`), and slot `0xBC` is called with the cinfo.
pub fn bhk_motor_action_init(e: &mut Engine, this: Ptr, info: u32) {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let block = allocate_sized(e, 0x40);
    let gain = x87_quiet_bits(e.mem.u32(info + 0x24));
    let spin = x87_quiet_bits(e.mem.u32(info + 0x20));
    let hk_object = e.mem.u32(info + 4);
    let action = e.with_stack(16, |e, vector| {
        for offset in [0, 4, 8, 12] {
            let lane = e.mem.u32(info + 0x10 + offset);
            e.mem.set_u32(vector.addr() + offset, lane);
        }
        e.call(
            HK_MOTOR_ACTION_CONSTRUCT,
            &args![block, hk_object, vector, spin, gain],
        )
        .u32()
    });
    e.vcall(me, 0x8c, &args![action]);
    e.call(REMOVE_REFERENCE, &args![action]);
    e.vcall(me, 0xbc, &args![info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00cc3840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3840 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMotorAction::GetViewerStrings` (Xbox PDB), vtable slot `0x60`: with the
/// allocation-scope word at 0x10, the `bhkUnaryAction` strings (`00c86b80`), then the
/// viewer string of the class `NiRTTI` (the global at `012685bc`), then "SpinRate" and
/// "Gain" with the Havok action's `+0x30` and `+0x34` `float`s (0.0 without one).
pub fn bhk_motor_action_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(UNARY_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(MOTOR_ACTION_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let havok = havok_action(e, this);
    let spin = if havok != 0 {
        x87_quiet_bits(e.mem.u32(havok + 0x30))
    } else {
        0
    };
    let text = e
        .call(VIEWER_STRING_FLOAT, &args![NAME_SPIN_RATE, spin])
        .u32();
    push_viewer_string(e, strings, text);
    let havok = havok_action(e, this);
    let gain = if havok != 0 {
        x87_quiet_bits(e.mem.u32(havok + 0x34))
    } else {
        0
    };
    let text = e.call(VIEWER_STRING_FLOAT, &args![NAME_GAIN, gain]).u32();
    push_viewer_string(e, strings, text);
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc_3270, bhk_motor_action_destruct(Ptr)),
        entry!(0x00cc_3320, fn_00cc3320(Ptr, u32)),
        entry!(0x00cc_33a0, fn_00cc33a0() -> u32),
        entry!(0x00cc_3440, bhk_motor_action_load_binary(Ptr, u32)),
        entry!(0x00cc_34a0, bhk_motor_action_save_binary(Ptr, u32)),
        entry!(0x00cc_3540, bhk_motor_action_create_clone(Ptr, u32) -> u32),
        entry!(0x00cc_3630, bhk_motor_action_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00cc_3710, bhk_motor_action_init(Ptr, u32)),
        entry!(0x00cc_3840, bhk_motor_action_get_viewer_strings(Ptr, u32)),
        entry!(0x00cc32e0, bhk_motor_action_get_r_t_t_i() -> u32),
        entry!(0x00cc3300, bhk_motor_action_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_motor_action_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_3270, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc3300, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc3300, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_3270), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_motor_action_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_32e0, &args![]).u32(), 0x0126_85bc);
    }
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
        e.set_global(COUNT_MOTOR_ACTION, 2u32);
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_KILL, 0, &log);
        record(&mut e, UNARY_DESTRUCT, 0, &log);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc_3270, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_MOTOR_ACTION);
        assert_eq!(e.global::<u32>(COUNT_MOTOR_ACTION), 1);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [SERIALIZABLE_KILL, UNARY_DESTRUCT]);
        assert_eq!(calls_to(&log, SERIALIZABLE_KILL), [[this]]);
        assert_eq!(calls_to(&log, UNARY_DESTRUCT), [[this]]);
    }

    #[test]
    fn fn_00cc3320_copies_the_vector_and_the_two_floats() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, UNARY_COPY_CINFO, 0, &log);
        record(&mut e, COPY_VECTOR, 0, &log);
        let havok = e.mem.alloc(0x40);
        // A signalling NaN comes out quiet through FLD / FSTP.
        e.mem.set_u32(havok + 0x30, 0x7fa0_0000);
        e.mem.set_f32(havok + 0x34, 2.5);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, havok);
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0x20, 0xdead);
        e.call(0x00cc_3320, &args![this, info]);
        assert_eq!(calls_to(&log, UNARY_COPY_CINFO), [[this, info]]);
        assert_eq!(calls_to(&log, COPY_VECTOR), [[info + 0x10, havok + 0x20]]);
        assert_eq!(e.mem.u8(info + 0x28), 1);
        assert_eq!(e.mem.f32(info + 0x24), 2.5);
        assert_eq!(e.mem.u32(info + 0x20), 0x7fe0_0000);
        // Without a Havok action the default vector is copied and the floats are 0.0.
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(info + 0x24, 0xbeef);
        e.call(0x00cc_3320, &args![this, info]);
        assert_eq!(
            calls_to(&log, COPY_VECTOR)[1],
            [info + 0x10, DEFAULT_VECTOR]
        );
        assert_eq!((e.mem.u32(info + 0x24), e.mem.u32(info + 0x20)), (0, 0));
    }

    #[test]
    fn fn_00cc33a0_builds_the_object_and_counts_it() {
        let mut e = engine();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 0xc, 7);
        let log: Calls = Rc::default();
        record(&mut e, ALLOCATE, object, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        assert_eq!(e.call(0x00cc_33a0, &args![]).u32(), object);
        assert_eq!(calls_to(&log, ALLOCATE), [[0x10]]);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT), [[object]]);
        assert_eq!(e.mem.u32(object), VTABLE_MOTOR_ACTION);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_ACTION), 1);
        assert_eq!(e.global::<u32>(COUNT_UNARY_ACTION), 1);
        assert_eq!(e.global::<u32>(COUNT_MOTOR_ACTION), 1);
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cc_33a0, &args![]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn load_and_save_binary_swap_the_cinfo_words_only_for_a_foreign_stream() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, UNARY_LOAD_BINARY, 0, &log);
        record(&mut e, UNARY_SAVE_BINARY, 0, &log);
        let info = e.mem.alloc(0x40);
        e.register_double(0x0900_0080, move |_, _| ret(info));
        let this = vobject(&mut e, 0x10, &[(0xb4, 0x0900_0080)]);
        let stream = e.mem.alloc(0x300);
        for offset in [0x10, 0x14, 0x18, 0x1c, 0x20, 0x24] {
            e.mem.set_u32(info + offset, 0x0102_0304);
        }
        e.mem.set_u32(info + 0xc, 0x0102_0304);
        // PC byte order: untouched.
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00cc_3440, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0x10), 0x0102_0304);
        assert_eq!(calls_to(&log, UNARY_LOAD_BINARY), [[this, stream]]);
        // Foreign order: the six words are swapped after the base load.
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00cc_3440, &args![this, stream]);
        for offset in [0x10, 0x14, 0x18, 0x1c, 0x20, 0x24] {
            assert_eq!(e.mem.u32(info + offset), 0x0403_0201);
        }
        assert_eq!(e.mem.u32(info + 0xc), 0x0102_0304);
        // Save: swapped for the base save and swapped back.
        e.mem.set_u8(stream + 0x1e4, 0);
        e.register_double(UNARY_SAVE_BINARY, move |e, _| {
            assert_eq!(e.mem.u32(info + 0x24), 0x0102_0304);
            Ret::default()
        });
        e.call(0x00cc_34a0, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0x24), 0x0403_0201);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.register_double(UNARY_SAVE_BINARY, move |e, _| {
            assert_eq!(e.mem.u32(info + 0x24), 0x0403_0201);
            Ret::default()
        });
        e.call(0x00cc_34a0, &args![this, stream]);
        assert_eq!(e.mem.u32(info + 0x24), 0x0403_0201);
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
        assert_eq!(e.call(0x00cc_3540, &args![0x5000u32, 0x99u32]).u32(), clone);
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
        assert_eq!(e.mem.u32(clone), VTABLE_MOTOR_ACTION);
        assert_eq!(calls_to(&log, UNARY_COPY_MEMBERS), [[0x5000, clone, 0x99]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // A failed allocation copies into nothing and answers 0.
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cc_3540, &args![0x5000u32, 0x98u32]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&log, UNARY_COPY_MEMBERS)[1], [0x5000, 0, 0x98]);
    }

    #[test]
    fn create_cinfo_aligns_the_block_fills_it_and_copies_from_the_action() {
        let mut e = engine();
        e.set_global(DEFAULT_GAIN, 0.75f32);
        let base = e.mem.alloc(0x60);
        for offset in 0..0x18 {
            e.mem.set_u32(base + 4 * offset, 0xaaaa_aaaa);
        }
        let block = base + 4;
        let log: Calls = Rc::default();
        record(&mut e, POOL_ALLOCATE, block, &log);
        record(&mut e, UNARY_COPY_CINFO, 0, &log);
        record(&mut e, COPY_VECTOR, 0, &log);
        let this = e.mem.alloc(0x10);
        let created = e.mem.alloc(4);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
        // No Havok action: the defaults stay.
        let info = e.call(0x00cc_3630, &args![this, created]).u32();
        let pad = 0x10 - (block & 0xf);
        assert_eq!(info, block + pad);
        assert_eq!(e.mem.u8(info - 1), pad as u8);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_OBJECT, 0x40]]);
        assert_eq!(e.mem.u32(this + 0xc), info);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0, 0));
        for offset in [0x10, 0x14, 0x18, 0x1c, 0x20] {
            assert_eq!(e.mem.u32(info + offset), 0);
        }
        assert_eq!(e.mem.f32(info + 0x24), 0.75);
        assert_eq!(e.mem.u8(info + 0x28), 1);
        assert!(calls_to(&log, UNARY_COPY_CINFO).is_empty());
        assert_eq!(e.mem.u32(word), 0x77);
        // The second time it is kept.
        assert_eq!(e.call(0x00cc_3630, &args![this, created]).u32(), info);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&log, POOL_ALLOCATE).len(), 1);
        // With a Havok action the cinfo is filled from it.
        e.mem.set_u32(this + 0xc, 0);
        let havok = e.mem.alloc(0x50);
        e.mem.set_u32(this + 8, havok);
        e.call(0x00cc_3630, &args![this, created]);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, UNARY_COPY_CINFO), [[this, info]]);
        assert_eq!(calls_to(&log, COPY_VECTOR).len(), 1);
    }

    #[test]
    fn init_builds_the_havok_action_from_the_cinfo_and_releases_it() {
        let (mut e, word, block) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, 0x0900_0070, 0, &log);
        record(&mut e, 0x0900_0074, 0, &log);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let capture = Rc::clone(&seen);
        e.register_double(HK_MOTOR_ACTION_CONSTRUCT, move |e, a| {
            let mut words = vec![a[0], a[1]];
            words.extend((0..4).map(|lane| e.mem.u32(a[2] + 4 * lane)));
            words.extend([a[3], a[4]]);
            capture.borrow_mut().push(words);
            ret(0x8800)
        });
        let this = vobject(&mut e, 0x10, &[(0x8c, 0x0900_0070), (0xbc, 0x0900_0074)]);
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info + 4, 0x6000);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0].into_iter().enumerate() {
            e.mem.set_f32(info + 0x10 + 4 * lane as u32, value);
        }
        e.mem.set_u32(info + 0x24, 0x7fa0_0000);
        e.call(0x00cc_3710, &args![this, info]);
        assert_eq!(
            *seen.borrow(),
            [[
                block,
                0x6000,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                4.0f32.to_bits(),
                5.0f32.to_bits(),
                0x7fe0_0000
            ]]
        );
        assert_eq!(e.mem.u16(block + 4), 0x40);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x0900_0070, REMOVE_REFERENCE, 0x0900_0074]);
        assert_eq!(calls_to(&log, 0x0900_0070), [[this, 0x8800]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0x8800]]);
        assert_eq!(calls_to(&log, 0x0900_0074), [[this, info]]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_the_spin_rate_and_the_gain() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(MOTOR_ACTION_RTTI, 0x1234u32);
        let log: Calls = Rc::default();
        record(&mut e, UNARY_GET_VIEWER_STRINGS, 0, &log);
        record(&mut e, NI_GET_VIEWER_STRING, 0x3000, &log);
        record(&mut e, STRINGS_GROW, 0, &log);
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
        let havok = e.mem.alloc(0x40);
        e.mem.set_f32(havok + 0x30, 1.5);
        e.mem.set_f32(havok + 0x34, -2.0);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, havok);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 4);
        e.call(0x00cc_3840, &args![this, strings]);
        assert_eq!(calls_to(&log, UNARY_GET_VIEWER_STRINGS), [[this, strings]]);
        assert_eq!(calls_to(&log, NI_GET_VIEWER_STRING), [[0x1234]]);
        assert_eq!(
            *floats.borrow(),
            [
                (NAME_SPIN_RATE, 1.5f32.to_bits()),
                (NAME_GAIN, (-2.0f32).to_bits())
            ]
        );
        assert_eq!(*pushed.borrow(), [(0, 0x3000), (1, 0x3001), (2, 0x3002)]);
        // Full at the second and third element: grown by the step plus the size.
        assert_eq!(calls_to(&log, STRINGS_GROW), [[strings, 5], [strings, 6]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // Without a Havok action both floats are 0.0.
        e.mem.set_u32(this + 8, 0);
        floats.borrow_mut().clear();
        e.call(0x00cc_3840, &args![this, strings]);
        assert_eq!(*floats.borrow(), [(NAME_SPIN_RATE, 0), (NAME_GAIN, 0)]);
    }
}
