//! `BSHavok/bhkangulardashpotaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cc4200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAngularDashpotAction::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc4180`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_angular_dashpot_action_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_4180;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc41f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAngularDashpotAction::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_85d4.
pub fn bhk_angular_dashpot_action_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_85d4
}

/// Per-thread word (`TLS + 0x2b4`) that the allocating functions set to 0x10 around their work.
const SCOPE_WORD: u32 = 0x2b4;
/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator whose slot `+4`
/// allocates `(bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The game heap's allocator (`00aa13e0`, cdecl `(size)`).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// `MemoryManager` allocation (`00aa3e40`, `this` = the manager at `011f6238`, `(size)`).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `bhkRefObject` constructor (`00c87c30`).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable::Kill` (Xbox PDB).
const KILL: u32 = 0x00c8_76a0;
/// `bhkBinaryAction::~bhkBinaryAction` (Xbox PDB).
const BINARY_ACTION_DESTRUCTOR: u32 = 0x00c8_a720;
/// `bhkBinaryAction`: the unnamed cinfo copy (`00c8a800`, `(this, cinfo)`), `LoadBinary`,
/// `SaveBinary` and `GetViewerStrings` (Xbox PDB names, `(this, stream or strings)`).
const BINARY_ACTION_COPY_CINFO: u32 = 0x00c8_a800;
const BINARY_ACTION_LOAD_BINARY: u32 = 0x00c8_a7a0;
const BINARY_ACTION_SAVE_BINARY: u32 = 0x00c8_aac0;
const BINARY_ACTION_GET_VIEWER_STRINGS: u32 = 0x00c8_ad40;
/// `bhkSerializable::CloneCritLock` and `CloneCritUnlock` (Xbox PDB).
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `hkpAngularDashpotAction::hkpAngularDashpotAction` (Xbox PDB):
/// `(this, bodyA, bodyB, 0)`; answers the constructed object.
const HKP_ANGULAR_DASHPOT_CONSTRUCT: u32 = 0x00d4_dbe0;
/// `hkReferencedObject` release (`00c905b0`, `this` only).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `NiGetViewerString` and the two array helpers of the viewer-string array.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The word the class passes to `NiGetViewerString`.
const CLASS_NAME_POINTER: u32 = 0x0126_85d4;

/// Class vtable and the two base-class vtables the inlined constructors store before it, with
/// the object counters of the three constructors (`bhkAction`, `bhkBinaryAction`,
/// `bhkAngularDashpotAction`).
const VTABLE_CLASS: u32 = 0x010c_97e4;
const VTABLE_BINARY_ACTION: u32 = 0x010c_5ef4;
const VTABLE_ACTION: u32 = 0x0102_016c;
const COUNT_ACTION: u32 = 0x0126_8154;
const COUNT_BINARY_ACTION: u32 = 0x0126_8194;
const COUNT_CLASS: u32 = 0x0126_85d0;
/// The default vector (four floats) the cinfo copy uses when there is no Havok action, and the
/// two float constants of the cinfo constructor (`+0x20`, `+0x24`).
const DEFAULT_VECTOR: u32 = 0x0126_83b0;
const DEFAULT_STRENGTH: u32 = 0x0101_e2bc;
const DEFAULT_DAMPING: u32 = 0x0101_3ea4;

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

fn count_up(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// The byte swap of the six cinfo words `+0x10..+0x24` that `LoadBinary` and `SaveBinary` apply
/// when the stream's flag byte is not 1.
fn swap_cinfo_words(e: &mut Engine, cinfo: u32) {
    for offset in [0x10u32, 0x14, 0x18, 0x1c, 0x20, 0x24] {
        let word = e.mem.u32(cinfo.wrapping_add(offset));
        e.mem.set_u32(cinfo.wrapping_add(offset), word.swap_bytes());
    }
}

/// The inlined `new bhkAngularDashpotAction`: a 0x10-byte block from the game heap, the base
/// constructors and the three counters; 0 when the allocation fails.
fn construct_dashpot_action(e: &mut Engine) -> u32 {
    let object = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_ACTION);
    e.mem.set_u32(object + 0xc, 0);
    count_up(e, COUNT_ACTION);
    e.mem.set_u32(object, VTABLE_BINARY_ACTION);
    count_up(e, COUNT_BINARY_ACTION);
    e.mem.set_u32(object, VTABLE_CLASS);
    count_up(e, COUNT_CLASS);
    object
}

// Translated from 00cc32f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAngularDashpotAction::QCinfoSize` (Xbox PDB): takes no stack words and returns 0x30.
pub fn bhk_angular_dashpot_action_q_cinfo_size(_e: &mut Engine) -> u32 {
    0x30
}

// Translated from 00cc4180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4180 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAngularDashpotAction::~bhkAngularDashpotAction` (Xbox PDB): stores the class vtable,
/// runs `Kill` (`00c876a0`), decrements the class counter and runs the
/// `bhkBinaryAction` destructor (`00c8a720`).
pub fn bhk_angular_dashpot_action_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    e.call(KILL, &args![this]);
    let count: u32 = e.global(COUNT_CLASS);
    e.set_global(COUNT_CLASS, count.wrapping_sub(1));
    e.call(BINARY_ACTION_DESTRUCTOR, &args![this]);
}

// Translated from 00cc4220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the cinfo copy of the class, called by `CreateCinfo`): runs the
/// `bhkBinaryAction` copy (`00c8a800`) on `(this, cinfo)`, then fills the dashpot fields from
/// the Havok action (`this + 8`): `cinfo + 0x24` = its `+0x34`, `cinfo + 0x10..+0x1C` = the
/// four floats at its `+0x20` (the default vector at `012683b0` when there is none),
/// `cinfo + 0x20` = its `+0x30`; 0 for the single floats when `this` or the action is 0.
/// Every float goes through an `FLD`/`FSTP` pair ([`x87_quiet_bits`]).
pub fn bhk_angular_dashpot_action_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: Ptr) {
    e.call(BINARY_ACTION_COPY_CINFO, &args![this, cinfo]);
    let action = if this.addr() == 0 {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    let single = |e: &Engine, offset: u32| {
        if action == 0 {
            0
        } else {
            x87_quiet_bits(e.mem.u32(action + offset))
        }
    };
    let damping = single(e, 0x34);
    e.mem.set_u32(cinfo.addr() + 0x24, damping);
    let vector = if action == 0 {
        DEFAULT_VECTOR
    } else {
        action + 0x20
    };
    for lane in 0..4u32 {
        let word = x87_quiet_bits(e.mem.u32(vector + 4 * lane));
        e.mem.set_u32(cinfo.addr() + 0x10 + 4 * lane, word);
    }
    let strength = single(e, 0x30);
    e.mem.set_u32(cinfo.addr() + 0x20, strength);
}

// Translated from 00cc42a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc42a0 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (cdecl, no arguments): `new bhkAngularDashpotAction`, 0 when the heap returns 0.
pub fn fn_00cc42a0(e: &mut Engine) -> u32 {
    construct_dashpot_action(e)
}

// Translated from 00cc4340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4340 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAngularDashpotAction::CreateClone` (Xbox PDB): with the clone lock held (`00c876f0`,
/// `00c87700`) builds a new `bhkAngularDashpotAction` and runs virtual slot 0xD0
/// (`CopyMembers`) on `(this, clone, process)`. Answers the clone (0 when the allocation
/// failed; the copy still runs with it).
pub fn bhk_angular_dashpot_action_create_clone(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    e.call(CLONE_LOCK, &args![]);
    let clone = construct_dashpot_action(e);
    e.vcall(this.addr(), 0xd0, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    clone
}

// Translated from 00cc4400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAngularDashpotAction::LoadBinary` (Xbox PDB): `bhkBinaryAction::LoadBinary`
/// (`00c8a7a0`), then, when the byte at `stream + 0x1E5` is not 1, the cinfo (virtual slot
/// 0xB4, given a scratch out-pointer) has its six words at `+0x10..+0x24` byte swapped.
pub fn bhk_angular_dashpot_action_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BINARY_ACTION_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e5) != 1 {
        let cinfo = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00cc4460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAngularDashpotAction::SaveBinary` (Xbox PDB): gets the cinfo (virtual slot 0xB4),
/// byte swaps its six words at `+0x10..+0x24` when the byte at `stream + 0x1E4` is not 1, runs
/// `bhkBinaryAction::SaveBinary` (`00c8aac0`) and swaps them back under the same test.
pub fn bhk_angular_dashpot_action_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let cinfo = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
    e.call(BINARY_ACTION_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00cc4500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the cinfo constructor): clears `+0`, `+4`, `+8` and the four floats at `+0x10`,
/// sets `+0x20` and `+0x24` from the floats at `0101e2bc` and `01013ea4` and the four floats
/// at `+0x10` from `012683b0`; every float goes through an `FLD`/`FSTP` pair
/// ([`x87_quiet_bits`]). Answers `this`.
pub fn fn_00cc4500(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    for offset in [0u32, 4, 8, 0x10, 0x14, 0x18, 0x1c] {
        e.mem.set_u32(me + offset, 0);
    }
    let strength = x87_quiet_bits(e.global::<u32>(DEFAULT_STRENGTH));
    e.mem.set_u32(me + 0x20, strength);
    let damping = x87_quiet_bits(e.global::<u32>(DEFAULT_DAMPING));
    e.mem.set_u32(me + 0x24, damping);
    for lane in 0..4u32 {
        let word = x87_quiet_bits(e.global::<u32>(DEFAULT_VECTOR + 4 * lane));
        e.mem.set_u32(me + 0x10 + 4 * lane, word);
    }
    this
}

// Translated from 00cc4560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4560 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAngularDashpotAction::CreateCinfo` (Xbox PDB), with the allocation scope word set to
/// 0x10: stores in the byte at `created` whether the cinfo at `+0xC` was missing; then builds
/// it: a 0x30-byte block from the memory manager (`00aa3e40`) constructed by `00cc4500`
/// (0 when the manager answers 0), stored at `+0xC` and, when the Havok action (`+8`)
/// exists, filled by the cinfo copy (`00cc4220`). Answers the cinfo.
pub fn bhk_angular_dashpot_action_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(missing));
    if missing {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x30u32])
            .u32();
        let cinfo = if block == 0 {
            0
        } else {
            fn_00cc4500(e, Ptr::new(block)).addr()
        };
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if e.mem.u32(this.addr() + 8) != 0 {
            bhk_angular_dashpot_action_copy_cinfo(e, this, Ptr::new(cinfo));
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(word, saved);
    cinfo
}

// Translated from 00cc4610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4610 - the FS:[0] exception-unwinding frame and the /GS cookie check (00ec408c) are not modeled.
/// `bhkAngularDashpotAction::Init` (Xbox PDB), with the allocation scope word set to 0x10:
/// builds the 0x40-byte `hkpAngularDashpotAction` (memory-size word at `+4` = 0x40) from the
/// bodies at `info + 4` and `info + 8` (`00d4dbe0`, last argument 0), sets its `+0x30` and
/// `+0x34` from the floats at `info + 0x20` and `info + 0x24` (`FLD`/`FSTP` copies) and its
/// vector at `+0x20` from the four floats at `info + 0x10`, hands it to virtual slot 0x8C
/// (`SethkObject`), releases the creation reference (`00c905b0`) and runs virtual slot 0xBC
/// (`PostInit`) with the cinfo.
pub fn bhk_angular_dashpot_action_init(e: &mut Engine, this: Ptr, info: Ptr) {
    let (word, saved) = enter_scope(e);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x40u32]).u32();
    e.mem.set_u16(block + 4, 0x40);
    let body_a = e.mem.u32(info.addr() + 4);
    let body_b = e.mem.u32(info.addr() + 8);
    let action = e
        .call(
            HKP_ANGULAR_DASHPOT_CONSTRUCT,
            &args![block, body_a, body_b, 0u32],
        )
        .u32();
    let strength = x87_quiet_bits(e.mem.u32(info.addr() + 0x20));
    e.mem.set_u32(action + 0x30, strength);
    let damping = x87_quiet_bits(e.mem.u32(info.addr() + 0x24));
    e.mem.set_u32(action + 0x34, damping);
    for lane in 0..4u32 {
        let word = e.mem.u32(info.addr() + 0x10 + 4 * lane);
        e.mem.set_u32(action + 0x20 + 4 * lane, word);
    }
    e.vcall(this.addr(), 0x8c, &args![action]);
    e.call(REMOVE_REFERENCE, &args![action]);
    e.vcall(this.addr(), 0xbc, &args![info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00cc4740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc4740 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAngularDashpotAction::GetViewerStrings` (Xbox PDB), with the allocation scope word set
/// to 0x10: runs `bhkBinaryAction::GetViewerStrings` (`00c8ad40`) on `(this, strings)`, then
/// appends the string `NiGetViewerString` makes of the word at `012685d4` to the array
/// (grown by the step at `+0xE` plus the size when the 16-bit size at `+0xA` has reached the
/// capacity at `+0x8`).
pub fn bhk_angular_dashpot_action_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let (word, saved) = enter_scope(e);
    e.call(BINARY_ACTION_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    let capacity = u32::from(e.mem.u16(strings.addr() + 8));
    if size >= capacity {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc41f0, bhk_angular_dashpot_action_get_r_t_t_i() -> u32),
        entry!(0x00cc4200, bhk_angular_dashpot_action_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc32f0, bhk_angular_dashpot_action_q_cinfo_size() -> u32),
        entry!(0x00cc4180, bhk_angular_dashpot_action_destructor(Ptr)),
        entry!(0x00cc4220, bhk_angular_dashpot_action_copy_cinfo(Ptr, Ptr)),
        entry!(0x00cc42a0, fn_00cc42a0() -> u32),
        entry!(0x00cc4340, bhk_angular_dashpot_action_create_clone(Ptr, u32) -> u32),
        entry!(0x00cc4400, bhk_angular_dashpot_action_load_binary(Ptr, Ptr)),
        entry!(0x00cc4460, bhk_angular_dashpot_action_save_binary(Ptr, Ptr)),
        entry!(0x00cc4500, fn_00cc4500(Ptr) -> Ptr),
        entry!(0x00cc4560, bhk_angular_dashpot_action_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00cc4610, bhk_angular_dashpot_action_init(Ptr, Ptr)),
        entry!(
            0x00cc4740,
            bhk_angular_dashpot_action_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_angular_dashpot_action_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_4180, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc4200, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc4200, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_4180), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_angular_dashpot_action_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_41f0, &args![]).u32(), 0x0126_85d4);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An object with a fresh vtable holding the given `(byte offset, target)` slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    fn map_counters(e: &mut Engine) {
        for counter in [COUNT_ACTION, COUNT_BINARY_ACTION, COUNT_CLASS] {
            e.map(counter, 4);
            e.set_global(counter, 10u32);
        }
    }

    #[test]
    fn q_cinfo_size_is_0x30() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc32f0, &args![]).u32(), 0x30);
    }

    #[test]
    fn destructor_stores_the_vtable_kills_and_destroys_the_base() {
        let mut e = Engine::new();
        map_counters(&mut e);
        e.register(KILL, nothing);
        e.register(BINARY_ACTION_DESTRUCTOR, nothing);
        let this = object(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00cc4180, &args![this]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00cc4180, KILL, BINARY_ACTION_DESTRUCTOR]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 9);
    }

    #[test]
    fn copy_cinfo_copies_the_action_fields_or_the_defaults() {
        let mut e = Engine::new();
        e.register(BINARY_ACTION_COPY_CINFO, nothing);
        e.map(DEFAULT_VECTOR, 16);
        for lane in 0..4u32 {
            e.mem.set_f32(DEFAULT_VECTOR + 4 * lane, 10.0 + lane as f32);
        }
        let this = object(&mut e, &[]);
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, action);
        for lane in 0..4u32 {
            e.mem.set_f32(action + 0x20 + 4 * lane, 1.0 + lane as f32);
        }
        e.mem.set_f32(action + 0x30, 5.5);
        // A signalling NaN is quieted by the FLD/FSTP pair.
        e.mem.set_u32(action + 0x34, 0x7f80_0001);
        let cinfo = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x00cc4220, &args![this, cinfo]);
        assert_eq!(calls(&e, BINARY_ACTION_COPY_CINFO), [[this, cinfo]]);
        assert_eq!(e.mem.f32(cinfo + 0x10), 1.0);
        assert_eq!(e.mem.f32(cinfo + 0x1c), 4.0);
        assert_eq!(e.mem.f32(cinfo + 0x20), 5.5);
        assert_eq!(e.mem.u32(cinfo + 0x24), 0x7fc0_0001);
        // No Havok action: zeros and the default vector.
        let bare = object(&mut e, &[]);
        e.call(0x00cc4220, &args![bare, cinfo]);
        assert_eq!(e.mem.f32(cinfo + 0x10), 10.0);
        assert_eq!(e.mem.f32(cinfo + 0x1c), 13.0);
        assert_eq!(e.mem.u32(cinfo + 0x20), 0);
        assert_eq!(e.mem.u32(cinfo + 0x24), 0);
    }

    #[test]
    fn new_builds_the_object_and_counts_it_or_answers_zero() {
        let mut e = Engine::new();
        map_counters(&mut e);
        let block = e.mem.alloc(0x10);
        e.register_double(HEAP_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(REF_OBJECT_CONSTRUCT, nothing);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc42a0, &args![]).u32(), block);
        assert_eq!(calls(&e, HEAP_ALLOCATE), [[0x10]]);
        assert_eq!(calls(&e, REF_OBJECT_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        for counter in [COUNT_ACTION, COUNT_BINARY_ACTION, COUNT_CLASS] {
            assert_eq!(e.global::<u32>(counter), 11);
        }
        e.register(HEAP_ALLOCATE, nothing);
        assert_eq!(e.call(0x00cc42a0, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_runs_copy_members_between_the_clone_lock_calls() {
        let mut e = Engine::new();
        map_counters(&mut e);
        let block = e.mem.alloc(0x10);
        e.register_double(HEAP_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        for address in [REF_OBJECT_CONSTRUCT, CLONE_LOCK, CLONE_UNLOCK, 0x0900_00d0] {
            e.register(address, nothing);
        }
        let this = object(&mut e, &[(0xd0, 0x0900_00d0)]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc4340, &args![this, 0x7700u32]).u32(), block);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00cc4340,
                CLONE_LOCK,
                HEAP_ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                0x0900_00d0,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(calls(&e, 0x0900_00d0), [[this, block, 0x7700]]);
    }

    /// A dashpot action whose slot 0xB4 answers a cinfo with six words `0x01020304 + n` at
    /// `+0x10`, and a stream with the flag bytes at `+0x1E4` and `+0x1E5`.
    fn swap_setup(flag: u8) -> (Engine, Ptr, Ptr, u32) {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x30);
        e.register_double(0x0900_00b4, move |_, _| Ret {
            eax: cinfo,
            ..Ret::default()
        });
        let this = Ptr::new(object(&mut e, &[(0xb4, 0x0900_00b4)]));
        for lane in 0..6u32 {
            e.mem.set_u32(cinfo + 0x10 + 4 * lane, 0x0102_0304 + lane);
        }
        let stream = Ptr::new(e.mem.alloc(0x200));
        e.mem.set_u8(stream.addr() + 0x1e4, flag);
        e.mem.set_u8(stream.addr() + 0x1e5, flag);
        (e, this, stream, cinfo)
    }

    #[test]
    fn load_binary_swaps_the_cinfo_words_unless_the_flag_is_one() {
        for (flag, swapped) in [(0u8, true), (1, false)] {
            let (mut e, this, stream, cinfo) = swap_setup(flag);
            e.register(BINARY_ACTION_LOAD_BINARY, nothing);
            e.call_log = Some(vec![]);
            e.call(0x00cc4400, &args![this, stream]);
            assert_eq!(
                calls(&e, BINARY_ACTION_LOAD_BINARY),
                [[this.addr(), stream.addr()]]
            );
            let first = e.mem.u32(cinfo + 0x10);
            let last = e.mem.u32(cinfo + 0x24);
            if swapped {
                assert_eq!(first, 0x0403_0201);
                assert_eq!(last, (0x0102_0304u32 + 5).swap_bytes());
            } else {
                assert_eq!(first, 0x0102_0304);
                assert_eq!(last, 0x0102_0304 + 5);
            }
        }
    }

    #[test]
    fn save_binary_swaps_around_the_base_save_and_restores_the_words() {
        for (flag, swapped) in [(0u8, true), (1, false)] {
            let (mut e, this, stream, cinfo) = swap_setup(flag);
            e.register_double(BINARY_ACTION_SAVE_BINARY, move |e, _| {
                let seen = e.mem.u32(cinfo + 0x10);
                e.mem.set_u32(cinfo + 0x2c, seen);
                Ret::default()
            });
            e.call(0x00cc4460, &args![this, stream]);
            let during = e.mem.u32(cinfo + 0x2c);
            if swapped {
                assert_eq!(during, 0x0403_0201);
            } else {
                assert_eq!(during, 0x0102_0304);
            }
            assert_eq!(e.mem.u32(cinfo + 0x10), 0x0102_0304);
            assert_eq!(e.mem.u32(cinfo + 0x24), 0x0102_0304 + 5);
        }
    }

    #[test]
    fn cinfo_constructor_sets_the_defaults() {
        let mut e = Engine::new();
        e.map(DEFAULT_VECTOR, 16);
        e.map(DEFAULT_STRENGTH, 4);
        e.map(DEFAULT_DAMPING, 4);
        e.set_global(DEFAULT_STRENGTH, 2.5f32);
        e.set_global(DEFAULT_DAMPING, 0.25f32);
        for lane in 0..4u32 {
            e.mem.set_f32(DEFAULT_VECTOR + 4 * lane, 1.0 + lane as f32);
        }
        let cinfo = e.mem.alloc(0x30);
        for offset in 0..12u32 {
            e.mem.set_u32(cinfo + 4 * offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00cc4500, &args![cinfo]).u32(), cinfo);
        for offset in [0u32, 4, 8] {
            assert_eq!(e.mem.u32(cinfo + offset), 0);
        }
        assert_eq!(e.mem.f32(cinfo + 0x10), 1.0);
        assert_eq!(e.mem.f32(cinfo + 0x1c), 4.0);
        assert_eq!(e.mem.f32(cinfo + 0x20), 2.5);
        assert_eq!(e.mem.f32(cinfo + 0x24), 0.25);
    }

    #[test]
    fn create_cinfo_builds_the_cinfo_once() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x66);
        e.map(DEFAULT_VECTOR, 16);
        e.map(DEFAULT_STRENGTH, 4);
        e.map(DEFAULT_DAMPING, 4);
        let block = e.mem.alloc(0x30);
        e.register_double(MEMORY_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(BINARY_ACTION_COPY_CINFO, nothing);
        let this = object(&mut e, &[]);
        let created = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc4560, &args![this, created]).u32(), block);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u32(this + 0xc), block);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), [[MEMORY_MANAGER, 0x30]]);
        assert!(calls(&e, BINARY_ACTION_COPY_CINFO).is_empty());
        // The cinfo now exists: nothing is allocated.
        assert_eq!(e.call(0x00cc4560, &args![this, created]).u32(), block);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
        // With a Havok action the copy runs on the new cinfo.
        let other = object(&mut e, &[]);
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(other + 8, action);
        e.call(0x00cc4560, &args![other, created]);
        assert_eq!(calls(&e, BINARY_ACTION_COPY_CINFO), [[other, block]]);
        assert_eq!(e.mem.u32(word), 0x66);
    }

    #[test]
    fn init_builds_the_dashpot_action_inside_the_scope_word() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        let allocator = object(&mut e, &[(4, 0x0900_0004)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let action = e.mem.alloc(0x40);
        e.register_double(0x0900_0004, move |e, a| {
            assert_eq!(a[1], 0x40);
            let scope = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(scope), 0x10);
            Ret {
                eax: action,
                ..Ret::default()
            }
        });
        e.register(HKP_ANGULAR_DASHPOT_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x0900_008c, nothing);
        e.register(0x0900_00bc, nothing);
        e.register(REMOVE_REFERENCE, nothing);
        let this = object(&mut e, &[(0x8c, 0x0900_008c), (0xbc, 0x0900_00bc)]);
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info + 4, 0xa1);
        e.mem.set_u32(info + 8, 0xb2);
        e.mem.set_f32(info + 0x20, 3.0);
        e.mem.set_u32(info + 0x24, 0x7f80_0001);
        for lane in 0..4u32 {
            e.mem.set_u32(info + 0x10 + 4 * lane, 0x100 + lane);
        }
        e.call_log = Some(vec![]);
        e.call(0x00cc4610, &args![this, info]);
        assert_eq!(e.mem.u16(action + 4), 0x40);
        assert_eq!(
            calls(&e, HKP_ANGULAR_DASHPOT_CONSTRUCT),
            [[action, 0xa1, 0xb2, 0]]
        );
        assert_eq!(e.mem.f32(action + 0x30), 3.0);
        assert_eq!(e.mem.u32(action + 0x34), 0x7fc0_0001);
        for lane in 0..4u32 {
            assert_eq!(e.mem.u32(action + 0x20 + 4 * lane), 0x100 + lane);
        }
        assert_eq!(calls(&e, 0x0900_008c), [[this, action]]);
        assert_eq!(calls(&e, REMOVE_REFERENCE), [[action]]);
        assert_eq!(calls(&e, 0x0900_00bc), [[this, info]]);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    #[test]
    fn get_viewer_strings_grows_a_full_array_before_storing() {
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(CLASS_NAME_POINTER, 4);
            e.set_global(CLASS_NAME_POINTER, 0x5151_0001u32);
            let word = e.tls() + SCOPE_WORD;
            e.mem.set_u32(word, 0x66);
            e.register(BINARY_ACTION_GET_VIEWER_STRINGS, nothing);
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(STRINGS_GROW, nothing);
            e.register(STRINGS_SET_AT, nothing);
            let this = object(&mut e, &[]);
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.call_log = Some(vec![]);
            e.call(0x00cc4740, &args![this, strings]);
            assert_eq!(
                calls(&e, BINARY_ACTION_GET_VIEWER_STRINGS),
                [[this, strings]]
            );
            assert_eq!(calls(&e, NI_GET_VIEWER_STRING), [[0x5151_0001]]);
            let grown = calls(&e, STRINGS_GROW);
            if size >= capacity {
                assert_eq!(grown, [[strings, 5 + u32::from(size)]]);
            } else {
                assert!(grown.is_empty());
            }
            assert_eq!(calls(&e, STRINGS_SET_AT).len(), 1);
            assert_eq!(e.mem.u32(word), 0x66);
        }
    }
}
