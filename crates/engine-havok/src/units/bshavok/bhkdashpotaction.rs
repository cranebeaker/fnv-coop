//! `BSHavok/bhkdashpotaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkDashpotAction` is a `bhkBinaryAction` (0x10 bytes): its Havok action is at `+8` and
//! its construction info at `+0xC`. The construction info this class makes (`CreateCinfo`) is
//! a 16-byte aligned 0x38-byte block inside a pool block of 0x50 bytes (the byte before it
//! holds the alignment adjustment, 1 to 16, to undo when the block is released): two
//! vectors at `+0x10` and `+0x20` and two floats at `+0x30` and `+0x34`. The Havok action
//! keeps the same vectors at `+0x20` and `+0x30` and the floats at `+0x40` and `+0x44`.
//!
//! The functions that allocate or release Havok objects raise the per-thread allocation
//! word at `TLS + 0x2b4` to 0x10 for their duration. The exception-unwinding frames
//! (`FS:[0]`) and `/GS` cookie checks of the functions marked `Translation incomplete` are
//! not modeled. The 16-byte aligned vector moves (`MOVAPS`) are done as plain byte copies;
//! their alignment faults are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhkconstraint::{
    add_viewer_string, allocate_havok, enter_scope, leave_scope,
};
use crate::units::bshavok::bhkconvextransformshape::copy_bytes;

/// `operator new` (`00aa13e0`, cdecl, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The pooled allocator object at `011f6238` and its allocate `(size)` (`00aa3e40`) and
/// release `(block)` (`00aa4060`) methods.
const POOL: u32 = 0x011f_6238;
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
const POOL_RELEASE: u32 = 0x00aa_4060;
/// The base constructor the object constructor starts with (`00c87c30`).
const BASE_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable` methods: `Kill`, the clone lock and unlock.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkBinaryAction` methods: destructor body (`00c8a720`), `LoadBinary` (`00c8a7a0`),
/// `SaveBinary` (`00c8aac0`), `GetViewerStrings` (`00c8ad40`) and an unnamed `CopyCinfo`
/// (`00c8a800`, `(this, cinfo)`).
const BINARY_ACTION_DESTRUCT: u32 = 0x00c8_a720;
const BINARY_ACTION_LOAD_BINARY: u32 = 0x00c8_a7a0;
const BINARY_ACTION_SAVE_BINARY: u32 = 0x00c8_aac0;
const BINARY_ACTION_GET_VIEWER_STRINGS: u32 = 0x00c8_ad40;
const BINARY_ACTION_COPY_CINFO: u32 = 0x00c8_a800;
/// An unnamed method (`004b4db0`, `RET 4`) that takes a pointer to a 16-byte vector; the
/// code calls it on the cinfo's `+0x10` once per vector.
const VECTOR_ASSIGN: u32 = 0x004b_4db0;
/// The Havok dashpot action constructor `(first, second, 0)` (`00d4d980`, `RET 0xC`) and
/// `hkReferencedObject::removeReference` (`00c905b0`).
const HK_DASHPOT_CONSTRUCT: u32 = 0x00d4_d980;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `NiGetViewerString` for a text (`00a7d330`, cdecl).
const VIEWER_STRING: u32 = 0x00a7_d330;
/// The vtables of the classes the constructor passes through (RTTI-confirmed `bhkAction`,
/// `bhkBinaryAction`, `bhkDashpotAction`) and their live-object counts.
const VTABLE_ACTION: u32 = 0x0102_016c;
const VTABLE_BINARY_ACTION: u32 = 0x010c_5ef4;
const VTABLE_DASHPOT_ACTION: u32 = 0x010c_970c;
const COUNT_ACTION: u32 = 0x0126_8154;
const COUNT_BINARY_ACTION: u32 = 0x0126_8194;
const COUNT_DASHPOT_ACTION: u32 = 0x0126_85c4;
/// The global holding the text `GetViewerStrings` starts with (read as a word).
const CLASS_TEXT: u32 = 0x0126_85c8;
/// The float every lane of the default vectors takes, and the defaults of the two floats.
const DEFAULT_VECTOR_FLOAT: u32 = 0x0101_1d78;
const DEFAULT_FIRST_FLOAT: u32 = 0x0101_e2bc;
const DEFAULT_SECOND_FLOAT: u32 = 0x0101_3ea4;

fn increment_count(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// The construction info of `this`: virtual slot `0xB4` fills a flag byte and answers it.
fn cinfo_of(e: &mut Engine, this: Ptr) -> u32 {
    e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32()
}

/// The Havok action of `this`, or 0 when `this` or the action is null.
fn action_of(e: &Engine, this: Ptr) -> u32 {
    if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    }
}

/// Byte-swaps the ten words at `cinfo + 0x10` through `cinfo + 0x34` in place.
fn swap_cinfo_words(e: &mut Engine, cinfo: u32) {
    for offset in (0x10..=0x34).step_by(4) {
        let word = e.mem.u32(cinfo + offset);
        e.mem.set_u32(cinfo + offset, word.swap_bytes());
    }
}

/// Makes a `bhkDashpotAction` in the 0x10-byte block (the constructor of `00cc3b30`): the
/// base constructor (`00c87c30`), then the vtable of each class on the way down
/// (`bhkAction`, `bhkBinaryAction`, `bhkDashpotAction`) with its live-object count raised,
/// `+0xC` cleared after the first.
fn construct(e: &mut Engine, block: u32) {
    e.call(BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, VTABLE_ACTION);
    e.mem.set_u32(block + 0xc, 0);
    increment_count(e, COUNT_ACTION);
    e.mem.set_u32(block, VTABLE_BINARY_ACTION);
    increment_count(e, COUNT_BINARY_ACTION);
    e.mem.set_u32(block, VTABLE_DASHPOT_ACTION);
    increment_count(e, COUNT_DASHPOT_ACTION);
}

// Translated from 00cc39b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc39b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkDashpotAction::~bhkDashpotAction` (Ghidra name): sets the class vtable, runs
/// `bhkSerializable::Kill`, decrements the live-object count at `012685c4` and runs the
/// `bhkBinaryAction` destructor body (`00c8a720`).
pub fn bhk_dashpot_action_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_DASHPOT_ACTION);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(COUNT_DASHPOT_ACTION);
    e.set_global(COUNT_DASHPOT_ACTION, count.wrapping_sub(1));
    e.call(BINARY_ACTION_DESTRUCT, &args![this]);
}

// Translated from 00cc3a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3a50 - the /GS cookie check is not modeled.
/// `bhkDashpotAction::CopyCinfo` (Ghidra name): the unnamed base copy (`00c8a800`), then two
/// local vectors that start with every lane the float at `01011d78` (raw bits) and, when
/// `this` and its Havok action exist, take the action's vectors at `+0x20` and `+0x30`,
/// are handed one after the other to the unnamed method `004b4db0` on `cinfo + 0x10`.
/// Then the cinfo's `+0x30` and `+0x34` become the action's floats at `+0x40` and `+0x44`
/// (through the x87 stack), or 0 without an action.
pub fn bhk_dashpot_action_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: u32) {
    e.call(BINARY_ACTION_COPY_CINFO, &args![this, cinfo]);
    let default = e.mem.u32(DEFAULT_VECTOR_FLOAT);
    e.with_stack(0x20, |e, local| {
        let first = local.addr();
        let second = first + 0x10;
        for lane in [0, 4, 8, 12] {
            e.mem.set_u32(first + lane, default);
            e.mem.set_u32(second + lane, default);
        }
        let action = action_of(e, this);
        if action != 0 {
            copy_bytes(e, first, action + 0x20, 16);
            copy_bytes(e, second, action + 0x30, 16);
        }
        e.call(VECTOR_ASSIGN, &args![cinfo + 0x10, first]);
        e.call(VECTOR_ASSIGN, &args![cinfo + 0x10, second]);
    });
    let action = action_of(e, this);
    let first = if action != 0 {
        x87_quiet_bits(e.mem.u32(action + 0x40))
    } else {
        0
    };
    e.mem.set_u32(cinfo + 0x30, first);
    let action = action_of(e, this);
    let second = if action != 0 {
        x87_quiet_bits(e.mem.u32(action + 0x44))
    } else {
        0
    };
    e.mem.set_u32(cinfo + 0x34, second);
}

// Translated from 00cc3b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3b30 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates a 0x10-byte block (`operator new`) and constructs a `bhkDashpotAction` in it
/// (unnamed in the engine map). Answers the object, or 0 when the allocation fails.
pub fn fn_00cc3b30(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct(e, block);
    Ptr::new(block)
}

// Translated from 00cc3bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkDashpotAction::LoadBinary` (Ghidra name): `bhkBinaryAction::LoadBinary`, then, when
/// the stream's byte at `+0x1E5` is not 1, the ten words of the cinfo (virtual slot `0xB4`)
/// from `+0x10` to `+0x34` are byte-swapped.
pub fn bhk_dashpot_action_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(BINARY_ACTION_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let cinfo = cinfo_of(e, this);
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00cc3c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkDashpotAction::SaveBinary` (Ghidra name): takes the cinfo, byte-swaps its ten words
/// (see `LoadBinary`) when the stream's byte at `+0x1E4` is not 1, runs
/// `bhkBinaryAction::SaveBinary`, and swaps the words back under the same condition.
pub fn bhk_dashpot_action_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let cinfo = cinfo_of(e, this);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
    e.call(BINARY_ACTION_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00cc3d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkDashpotAction::KillCinfo` (Ghidra name): when the low byte of `kill` is set and
/// there is a cinfo, releases its pool block (`00aa4060`; the cinfo pointer minus the
/// alignment byte stored just before it), then clears `+0xC`.
pub fn bhk_dashpot_action_kill_cinfo(e: &mut Engine, this: Ptr, kill: u32) {
    if kill as u8 == 0 {
        return;
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    if cinfo != 0 {
        let adjust = e.mem.u8(cinfo.wrapping_sub(1)) as u32;
        e.call(POOL_RELEASE, &args![POOL, cinfo.wrapping_sub(adjust)]);
    }
    e.mem.set_u32(this.addr() + 0xc, 0);
}

// Translated from 00cc3d60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3d60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkDashpotAction::CreateClone` (Ghidra name): in the Havok allocation scope and between
/// the clone lock and unlock (`00c876f0`, `00c87700`), makes a new 0x10-byte action
/// (`operator new` and the constructor of `00cc3b30`; null stays null) and calls this
/// action's virtual slot `0xD0` with `(clone, process)`. Answers the clone.
pub fn bhk_dashpot_action_create_clone(e: &mut Engine, this: Ptr, process: u32) -> Ptr {
    let old = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = fn_00cc3b30(e);
    e.vcall(this.addr(), 0xd0, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, old);
    clone
}

// Translated from 00cc3e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3e50 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkDashpotAction::CreateCinfo` (Ghidra name), `(this, created)`: in the Havok
/// allocation scope stores into the byte at `created` whether the action had no cinfo
/// (`+0xC` null). In that case it takes 0x50 bytes from the pool at `011f6238` (no null
/// check), moves forward to the next 16-byte boundary (1 to 16 bytes, recorded in the byte
/// before the block), writes the cinfo (`+0`, `+4`, `+8` and the eight floats from `+0x10`
/// to `+0x2C` zero, `+0x30` and `+0x34` the floats at `0101e2bc` and `01013ea4` through
/// the x87 stack; `+0xC` is left as the pool gave it), stores it at `+0xC` and, when the
/// action has a Havok action, fills it with `CopyCinfo` (`00cc3a50`). Answers the cinfo at
/// `+0xC`.
pub fn bhk_dashpot_action_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let old = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), missing as u8);
    if missing {
        let raw = e.call(POOL_ALLOCATE, &args![POOL, 0x50u32]).u32();
        let adjust = 0x10 - (raw as u8 & 0xf);
        let cinfo = raw.wrapping_add(adjust as u32);
        e.mem.set_u8(cinfo.wrapping_sub(1), adjust);
        for offset in [0, 4, 8] {
            e.mem.set_u32(cinfo + offset, 0);
        }
        for offset in (0x10..0x30).step_by(4) {
            e.mem.set_u32(cinfo + offset, 0);
        }
        let first = x87_quiet_bits(e.mem.u32(DEFAULT_FIRST_FLOAT));
        e.mem.set_u32(cinfo + 0x30, first);
        let second = x87_quiet_bits(e.mem.u32(DEFAULT_SECOND_FLOAT));
        e.mem.set_u32(cinfo + 0x34, second);
        let has_action = e.mem.u32(this.addr() + 8) != 0;
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if has_action {
            bhk_dashpot_action_copy_cinfo(e, this, cinfo);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    leave_scope(e, old);
    cinfo
}

// Translated from 00cc3f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc3f40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkDashpotAction::Init` (Ghidra name): in the Havok allocation scope builds a 0x60-byte
/// Havok dashpot action from the cinfo's words at `+4` and `+8` (`00d4d980`, with a third
/// argument 0), copies the cinfo's vectors at `+0x10` and `+0x20` to the new action's `+0x20`
/// and `+0x30` and its floats at `+0x30` and `+0x34` to `+0x40` and `+0x44` (the floats
/// through the x87 stack), hands the action to virtual slot `0x8C`, releases it, then calls
/// virtual slot `0xBC` with the cinfo.
pub fn bhk_dashpot_action_init(e: &mut Engine, this: Ptr, cinfo: u32) {
    let old = enter_scope(e);
    let block = allocate_havok(e, 0x60);
    let first = e.mem.u32(cinfo + 4);
    let second = e.mem.u32(cinfo + 8);
    let action = e
        .call(HK_DASHPOT_CONSTRUCT, &args![block, first, second, 0u32])
        .u32();
    copy_bytes(e, action + 0x30, cinfo + 0x20, 16);
    copy_bytes(e, action + 0x20, cinfo + 0x10, 16);
    let distance = x87_quiet_bits(e.mem.u32(cinfo + 0x30));
    e.mem.set_u32(action + 0x40, distance);
    let damping = x87_quiet_bits(e.mem.u32(cinfo + 0x34));
    e.mem.set_u32(action + 0x44, damping);
    e.vcall(this.addr(), 0x8c, &args![action]);
    e.call(REMOVE_REFERENCE, &args![action]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    leave_scope(e, old);
}

// Translated from 00cc4070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4070 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkDashpotAction::GetViewerStrings` (Ghidra name): in the Havok allocation scope runs
/// `bhkBinaryAction::GetViewerStrings`, appends the viewer string of the text in the global
/// at `012685c8`, then builds a local construction info (words at `+4`, `+8`, `+0xC` and
/// the nine floats from `+0x10` to `+0x30` zero, `+0x34` and `+0x38` the floats at
/// `0101e2bc` and `01013ea4` through the x87 stack; its word at `+0` is a stack leftover in
/// the exe and is 0 here) and runs `CopyCinfo` (`00cc3a50`) on it.
pub fn bhk_dashpot_action_get_viewer_strings(e: &mut Engine, this: Ptr, list: u32) {
    let old = enter_scope(e);
    e.call(BINARY_ACTION_GET_VIEWER_STRINGS, &args![this, list]);
    let class_text = e.global::<u32>(CLASS_TEXT);
    let text = e.call(VIEWER_STRING, &args![class_text]).u32();
    add_viewer_string(e, list, text);
    e.with_stack(0x3c, |e, local| {
        let cinfo = local.addr();
        let first = x87_quiet_bits(e.mem.u32(DEFAULT_FIRST_FLOAT));
        let second = x87_quiet_bits(e.mem.u32(DEFAULT_SECOND_FLOAT));
        for offset in [4, 8, 0xc] {
            e.mem.set_u32(cinfo + offset, 0);
        }
        for offset in (0x10..=0x30).step_by(4) {
            e.mem.set_u32(cinfo + offset, 0);
        }
        e.mem.set_u32(cinfo + 0x34, first);
        e.mem.set_u32(cinfo + 0x38, second);
        bhk_dashpot_action_copy_cinfo(e, this, cinfo);
    });
    leave_scope(e, old);
}

// Translated from 00cc3a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkDashpotAction::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc39b0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_dashpot_action_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_39b0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc3a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkDashpotAction::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_85c8.
pub fn bhk_dashpot_action_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_85c8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc39b0, bhk_dashpot_action_destructor(Ptr)),
        entry!(0x00cc3a20, bhk_dashpot_action_get_r_t_t_i() -> u32),
        entry!(0x00cc3a30, bhk_dashpot_action_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc3a50, bhk_dashpot_action_copy_cinfo(Ptr, u32)),
        entry!(0x00cc3b30, fn_00cc3b30() -> Ptr),
        entry!(0x00cc3bd0, bhk_dashpot_action_load_binary(Ptr, u32)),
        entry!(0x00cc3c50, bhk_dashpot_action_save_binary(Ptr, u32)),
        entry!(0x00cc3d30, bhk_dashpot_action_kill_cinfo(Ptr, u32)),
        entry!(0x00cc3d60, bhk_dashpot_action_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00cc3e50, bhk_dashpot_action_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00cc3f40, bhk_dashpot_action_init(Ptr, u32)),
        entry!(0x00cc4070, bhk_dashpot_action_get_viewer_strings(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_dashpot_action_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_39b0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc3a30, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc3a30, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_39b0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_dashpot_action_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_3a20, &args![]).u32(), 0x0126_85c8);
    }

    type Calls = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// An object whose vtable (a fresh block) has the given `(byte offset, target)` slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    fn scope(e: &mut Engine) -> u32 {
        let word = e.tls() + 0x2b4;
        e.mem.u32(word)
    }

    /// Starts the allocation scope word at a recognisable value, so its restore is visible.
    fn mark_scope(e: &mut Engine) {
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
    }

    /// An action (slot `0xB4` answers its `+0xC`, `0xD0` records, `0x8C` and `0xBC` record).
    fn action(e: &mut Engine) -> u32 {
        let this = object(
            e,
            &[
                (0xb4, 0x0900_0100),
                (0xd0, 0x0900_0101),
                (0x8c, 0x0900_0103),
                (0xbc, 0x0900_0104),
            ],
        );
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        quiet(e, &[0x0900_0101, 0x0900_0103, 0x0900_0104]);
        this
    }

    /// A pool block of the aligned kind `CreateCinfo` makes: the cinfo address.
    fn aligned_cinfo(e: &mut Engine) -> u32 {
        let raw = e.mem.alloc(0x50);
        let adjust = 0x10 - (raw as u8 & 0xf);
        let cinfo = raw + adjust as u32;
        e.mem.set_u8(cinfo - 1, adjust);
        cinfo
    }

    #[test]
    fn destructor_sets_the_vtable_kills_counts_down_and_destructs() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_DASHPOT_ACTION, 9u32);
        quiet(&mut e, &[SERIALIZABLE_KILL, BINARY_ACTION_DESTRUCT]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00cc39b0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_DASHPOT_ACTION);
        assert_eq!(e.global::<u32>(COUNT_DASHPOT_ACTION), 8);
        assert_eq!(
            order(&e),
            vec![0x00cc39b0, SERIALIZABLE_KILL, BINARY_ACTION_DESTRUCT]
        );
    }

    #[test]
    fn copy_cinfo_copies_the_vectors_and_floats_of_the_havok_action() {
        let mut e = Engine::new();
        e.map(DEFAULT_VECTOR_FLOAT, 4);
        e.mem.set_u32(DEFAULT_VECTOR_FLOAT, 0x3f00_0000);
        quiet(&mut e, &[BINARY_ACTION_COPY_CINFO]);
        // The vector method records the vector it is given: first word, last word.
        e.register(VECTOR_ASSIGN, |e, a| {
            let at = e.mem.u32(a[0] + 0x40);
            e.mem.set_u32(a[0] + 0x44 + 8 * at, e.mem.u32(a[1]));
            e.mem.set_u32(a[0] + 0x48 + 8 * at, e.mem.u32(a[1] + 12));
            e.mem.set_u32(a[0] + 0x40, at + 1);
            Ret::default()
        });
        let this = action(&mut e);
        let havok = e.mem.alloc(0x60);
        for i in 0..8u32 {
            e.mem.set_u32(havok + 0x20 + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(havok + 0x40, 0x7f80_0001);
        e.mem.set_u32(havok + 0x44, 0x3f80_0000);
        e.mem.set_u32(this + 8, havok);
        let cinfo = e.mem.alloc(0x100);
        // The vector method gets cinfo + 0x10 as its object; its record lives at +0x40 there.
        e.call_log = Some(vec![]);
        e.call(0x00cc3a50, &args![this, cinfo]);
        assert_eq!(
            calls_to(&e, BINARY_ACTION_COPY_CINFO),
            vec![vec![this, cinfo]]
        );
        let records = calls_to(&e, VECTOR_ASSIGN);
        assert_eq!(records.len(), 2);
        assert!(records.iter().all(|call| call[0] == cinfo + 0x10));
        let at = cinfo + 0x10;
        assert_eq!(e.mem.u32(at + 0x40), 2);
        assert_eq!(
            [
                e.mem.u32(at + 0x44),
                e.mem.u32(at + 0x48),
                e.mem.u32(at + 0x4c),
                e.mem.u32(at + 0x50)
            ],
            [0x100, 0x103, 0x104, 0x107]
        );
        assert_eq!(e.mem.u32(cinfo + 0x30), 0x7fc0_0001);
        assert_eq!(e.mem.u32(cinfo + 0x34), 0x3f80_0000);
        // No Havok action: the vectors are every lane the default float, the floats zero.
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(at + 0x40, 0);
        e.call(0x00cc3a50, &args![this, cinfo]);
        assert_eq!(
            [
                e.mem.u32(at + 0x44),
                e.mem.u32(at + 0x48),
                e.mem.u32(at + 0x4c),
                e.mem.u32(at + 0x50)
            ],
            [0x3f00_0000; 4]
        );
        assert_eq!((e.mem.u32(cinfo + 0x30), e.mem.u32(cinfo + 0x34)), (0, 0));
        // No action object at all: the same.
        e.mem.set_u32(cinfo + 0x30, 5);
        e.call(0x00cc3a50, &args![0u32, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 0x30), 0);
    }

    #[test]
    fn allocate_and_construct_makes_a_0x10_byte_action() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        quiet(&mut e, &[BASE_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00cc3b30, &args![]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![made]]);
        assert_eq!(e.mem.u32(made), VTABLE_DASHPOT_ACTION);
        assert_eq!(e.mem.u32(made + 0xc), 0);
        for counter in [COUNT_ACTION, COUNT_BINARY_ACTION, COUNT_DASHPOT_ACTION] {
            assert_eq!(e.global::<u32>(counter), 1);
        }
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc3b30, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_swaps_ten_words_unless_the_stream_byte_is_one() {
        let mut e = Engine::new();
        quiet(&mut e, &[BINARY_ACTION_LOAD_BINARY]);
        let this = action(&mut e);
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0xc, cinfo);
        for offset in (0..0x40).step_by(4) {
            e.mem.set_u32(cinfo + offset, 0x0102_0300 + offset);
        }
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cc3bd0, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x0102_0310);
        assert_eq!(
            calls_to(&e, BINARY_ACTION_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00cc3bd0, &args![this, stream]);
        for offset in (0x10..=0x34).step_by(4) {
            assert_eq!(
                e.mem.u32(cinfo + offset),
                (0x0102_0300 + offset).swap_bytes()
            );
        }
        // The words around the ten are untouched.
        assert_eq!(e.mem.u32(cinfo + 0xc), 0x0102_030c);
        assert_eq!(e.mem.u32(cinfo + 0x38), 0x0102_0338);
    }

    #[test]
    fn save_binary_swaps_around_the_base_save() {
        let mut e = Engine::new();
        // The base save records the first word it sees.
        e.register(BINARY_ACTION_SAVE_BINARY, |e, a| {
            let cinfo = e.mem.u32(a[0] + 0xc);
            let seen = e.mem.u32(cinfo + 0x10);
            e.mem.set_u32(cinfo + 0x3c, seen);
            Ret::default()
        });
        let this = action(&mut e);
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0xc, cinfo);
        for offset in (0x10..=0x34).step_by(4) {
            e.mem.set_u32(cinfo + offset, 0x0102_0300 + offset);
        }
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call(0x00cc3c50, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 0x3c), 0x0102_0310);
        assert_eq!(e.mem.u32(cinfo + 0x10), 0x0102_0310);
        e.mem.set_u8(stream + 0x1e4, 0);
        e.call(0x00cc3c50, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 0x3c), 0x1003_0201);
        // Swapped twice: back to the original words.
        for offset in (0x10..=0x34).step_by(4) {
            assert_eq!(e.mem.u32(cinfo + offset), 0x0102_0300 + offset);
        }
    }

    #[test]
    fn kill_cinfo_releases_the_pool_block_before_the_aligned_cinfo() {
        let mut e = Engine::new();
        quiet(&mut e, &[POOL_RELEASE]);
        let this = e.mem.alloc(0x20);
        let cinfo = aligned_cinfo(&mut e);
        let adjust = e.mem.u8(cinfo - 1) as u32;
        e.mem.set_u32(this + 0xc, cinfo);
        e.call_log = Some(vec![]);
        e.call(0x00cc3d30, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        e.call(0x00cc3d30, &args![this, 0x0100u32]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert!(calls_to(&e, POOL_RELEASE).is_empty());
        e.call(0x00cc3d30, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls_to(&e, POOL_RELEASE), vec![vec![POOL, cinfo - adjust]]);
        // No cinfo: still clears +0xC.
        e.call(0x00cc3d30, &args![this, 1u32]);
        assert_eq!(calls_to(&e, POOL_RELEASE).len(), 1);
    }

    #[test]
    fn create_clone_brackets_the_new_action_with_the_clone_lock() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        let this = action(&mut e);
        quiet(&mut e, &[CLONE_LOCK, CLONE_UNLOCK, BASE_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let clone = e.call(0x00cc3d60, &args![this, 0x4242u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE_DASHPOT_ACTION);
        assert_eq!(
            order(&e),
            vec![
                0x00cc3d60,
                CLONE_LOCK,
                OPERATOR_NEW,
                BASE_CONSTRUCT,
                0x0900_0101,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![this, clone, 0x4242]]);
        assert_eq!(scope(&mut e), 0x77);
        // A failed allocation clones into null.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc3d60, &args![this, 1u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0900_0101), vec![vec![this, 0, 1]]);
    }

    #[test]
    fn create_cinfo_builds_an_aligned_cinfo_once() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        quiet(&mut e, &[BINARY_ACTION_COPY_CINFO]);
        for (addr, bits) in [
            (DEFAULT_FIRST_FLOAT, 0x7f80_0001u32),
            (DEFAULT_SECOND_FLOAT, 0x4000_0000),
        ] {
            e.map(addr, 4);
            e.mem.set_u32(addr, bits);
        }
        // The pool block comes filled with 0xff bytes.
        e.register(POOL_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1]);
            for i in 0..a[1] {
                e.mem.set_u8(block + i, 0xff);
            }
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let this = action(&mut e);
        let created = e.mem.alloc(8);
        e.mem.set_u8(created, 9);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00cc3e50, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&e, POOL_ALLOCATE), vec![vec![POOL, 0x50]]);
        assert_eq!(cinfo % 16, 0);
        let adjust = e.mem.u8(cinfo - 1) as u32;
        assert!((1..=16).contains(&adjust));
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        for offset in [0, 4, 8] {
            assert_eq!(e.mem.u32(cinfo + offset), 0);
        }
        // The word at +0xC is left as the pool gave it.
        assert_eq!(e.mem.u32(cinfo + 0xc), 0xffff_ffff);
        for offset in (0x10..0x30).step_by(4) {
            assert_eq!(e.mem.u32(cinfo + offset), 0, "offset {offset:x}");
        }
        assert_eq!(e.mem.u32(cinfo + 0x30), 0x7fc0_0001);
        assert_eq!(e.mem.u32(cinfo + 0x34), 0x4000_0000);
        // No Havok action: no CopyCinfo.
        assert!(calls_to(&e, BINARY_ACTION_COPY_CINFO).is_empty());
        assert_eq!(scope(&mut e), 0x77);
        // The second call finds it.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc3e50, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert!(calls_to(&e, POOL_ALLOCATE).is_empty());
    }

    #[test]
    fn create_cinfo_fills_the_new_cinfo_when_there_is_a_havok_action() {
        let mut e = Engine::new();
        quiet(&mut e, &[BINARY_ACTION_COPY_CINFO]);
        e.register(POOL_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let this = action(&mut e);
        let havok = e.mem.alloc(0x60);
        e.mem.set_u32(this + 8, havok);
        e.map(DEFAULT_FIRST_FLOAT, 4);
        e.map(DEFAULT_SECOND_FLOAT, 4);
        e.register(VECTOR_ASSIGN, |_, _| Ret::default());
        e.map(DEFAULT_VECTOR_FLOAT, 4);
        let created = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00cc3e50, &args![this, created]).u32();
        assert_eq!(
            calls_to(&e, BINARY_ACTION_COPY_CINFO),
            vec![vec![this, cinfo]]
        );
        assert_eq!(calls_to(&e, VECTOR_ASSIGN).len(), 2);
    }

    #[test]
    fn init_builds_the_havok_action_from_the_cinfo() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        // The memory router and its allocator (slot 4 allocates).
        let allocator = object(&mut e, &[(4, 0x0900_0200)]);
        e.register(0x0900_0200, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        e.map(0x0200_3000, 0x10);
        e.mem.set_u32(0x0200_3000, router);
        e.register(0x00c8_5750, |e, _| Ret {
            eax: e.mem.u32(0x0200_3000),
            ..Ret::default()
        });
        e.register(HK_DASHPOT_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        quiet(&mut e, &[REMOVE_REFERENCE]);
        let this = action(&mut e);
        let cinfo = e.mem.alloc(0x60);
        e.mem.set_u32(cinfo + 4, 0xa1);
        e.mem.set_u32(cinfo + 8, 0xb2);
        for i in 0..8u32 {
            e.mem.set_u32(cinfo + 0x10 + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(cinfo + 0x30, 0x7f80_0001);
        e.mem.set_u32(cinfo + 0x34, 0x3f80_0000);
        e.call_log = Some(vec![]);
        e.call(0x00cc3f40, &args![this, cinfo]);
        let built = calls_to(&e, HK_DASHPOT_CONSTRUCT);
        assert_eq!(built[0][1..], [0xa1, 0xb2, 0]);
        let block = built[0][0];
        assert_eq!(e.mem.u16(block + 4), 0x60);
        // First vector (cinfo + 0x10) to +0x20, second (cinfo + 0x20) to +0x30.
        for i in 0..4u32 {
            assert_eq!(e.mem.u32(block + 0x20 + 4 * i), 0x100 + i);
            assert_eq!(e.mem.u32(block + 0x30 + 4 * i), 0x104 + i);
        }
        assert_eq!(e.mem.u32(block + 0x40), 0x7fc0_0001);
        assert_eq!(e.mem.u32(block + 0x44), 0x3f80_0000);
        assert_eq!(calls_to(&e, 0x0900_0103), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(calls_to(&e, 0x0900_0104), vec![vec![this, cinfo]]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_text_and_copies_into_a_stack_cinfo() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        e.set_global(CLASS_TEXT, 0x1111u32);
        for (addr, bits) in [
            (DEFAULT_FIRST_FLOAT, 0x3f80_0000u32),
            (DEFAULT_SECOND_FLOAT, 0x4000_0000),
            (DEFAULT_VECTOR_FLOAT, 0),
        ] {
            e.map(addr, 4);
            e.mem.set_u32(addr, bits);
        }
        quiet(&mut e, &[BINARY_ACTION_GET_VIEWER_STRINGS, 0x0096_ad30]);
        // The base copy checks the local construction info before CopyCinfo changes it.
        e.register(BINARY_ACTION_COPY_CINFO, |e, a| {
            let local = a[1];
            for offset in [4, 8, 0xc] {
                assert_eq!(e.mem.u32(local + offset), 0);
            }
            for offset in (0x10..=0x30).step_by(4) {
                assert_eq!(e.mem.u32(local + offset), 0, "offset {offset:x}");
            }
            assert_eq!(e.mem.u32(local + 0x34), 0x3f80_0000);
            assert_eq!(e.mem.u32(local + 0x38), 0x4000_0000);
            Ret::default()
        });
        quiet(&mut e, &[VECTOR_ASSIGN]);
        e.register(VIEWER_STRING, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(0x0096_ae90, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x100 + 4 * a[1], text);
            let count = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, count + 1);
            Ret::default()
        });
        let list = e.mem.alloc(0x200);
        e.mem.set_u16(list + 8, 8);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00cc4070, &args![this, list]);
        assert_eq!(e.mem.u32(list + 0x100), 0x1112);
        assert_eq!(
            calls_to(&e, BINARY_ACTION_GET_VIEWER_STRINGS),
            vec![vec![this, list]]
        );
        assert_eq!(calls_to(&e, BINARY_ACTION_COPY_CINFO).len(), 1);
        assert_eq!(scope(&mut e), 0x77);
    }
}
