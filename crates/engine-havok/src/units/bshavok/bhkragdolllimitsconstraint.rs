//! `BSHavok/bhkragdolllimitsconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

//!
//! `bhkRagdollLimitsConstraint` (Xbox PDB) is a `bhkConstraint` (0x10 bytes: `+8` the Havok
//! constraint, `+0xC` the cinfo block). Its cinfo wrapper (`hkRagdollLimitsConstraintCinfo`
//! in the Xbox PDB) keeps the `hkpRagdollLimitsData` (0xC0 bytes) at `+4`.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame
//! that is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

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
/// `bhkRefObject` constructor (`00c87c30`) and `bhkSerializable::Kill` (Xbox PDB).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
const KILL: u32 = 0x00c8_76a0;
/// `bhkConstraint` functions (Xbox PDB): destructor, `CopyMembers` (`(this, clone, process)`)
/// and `CopyCinfo` (`(this, cinfo)`).
const CONSTRAINT_DESTRUCTOR: u32 = 0x00c8_9640;
const CONSTRAINT_COPY_MEMBERS: u32 = 0x00c8_9730;
const CONSTRAINT_COPY_CINFO: u32 = 0x00c8_9c80;
/// `hkConstraintCinfo` functions (Xbox PDB): `CloneData` (`(this, data, cinfo)`),
/// `LoadBinary`, `SaveBinary` and `GetViewerStrings` (`(this, stream or array)`).
const CONSTRAINT_CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
const CONSTRAINT_CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
const CONSTRAINT_CINFO_SAVE_BINARY: u32 = 0x00c8_a280;
const CONSTRAINT_CINFO_GET_VIEWER_STRINGS: u32 = 0x00c8_a560;
/// The byte swap of a 0x30-byte block of three vectors (`00c8cdb0`, cdecl `(block)`).
const SWAP_BLOCK: u32 = 0x00c8_cdb0;
/// `hkpRagdollLimitsData::hkpRagdollLimitsData` (Xbox PDB; `this` = the block).
const RAGDOLL_LIMITS_DATA_CONSTRUCT: u32 = 0x00d4_f2a0;
/// `bhkSerializable::CloneCritLock` and `CloneCritUnlock` (Xbox PDB).
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `NiGetViewerString` for a float (`00a7d550`, cdecl `(name, bits)`) and the viewer-string
/// array's grow (`0096ad30`) and set-at (`0096ae90`) methods.
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// Class vtable and the base-class vtable the inlined constructor stores before it, with the
/// object counters.
const VTABLE_CLASS: u32 = 0x010c_9b0c;
const VTABLE_BASE: u32 = 0x0108_febc;
const COUNT_CLASS: u32 = 0x0126_8600;
const COUNT_BASE: u32 = 0x0126_8188;
/// The vtable of the 0x14-byte block `CreateCinfo` builds.
const VTABLE_CINFO_BLOCK: u32 = 0x010c_9be8;
/// Names of the viewer strings (`"ConeMinAngle"`, `"PlaneMinAngle"`, `"PlaneMaxAngle"`,
/// `"TwistMinAngle"`, `"TwistMaxAngle"`).
const NAME_CONE_MIN_ANGLE: u32 = 0x010c_9c40;
const NAME_PLANE_MIN_ANGLE: u32 = 0x010c_9c30;
const NAME_PLANE_MAX_ANGLE: u32 = 0x010c_9c20;
const NAME_TWIST_MIN_ANGLE: u32 = 0x010c_9c10;
const NAME_TWIST_MAX_ANGLE: u32 = 0x010c_9c00;

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

/// The inlined `new bhkRagdollLimitsConstraint`: a 0x10-byte block from the game heap, the
/// base constructors and the two counters; 0 when the allocation fails.
fn construct_constraint(e: &mut Engine) -> u32 {
    let object = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_BASE);
    e.mem.set_u32(object + 0xc, 0);
    count_up(e, COUNT_BASE);
    e.mem.set_u32(object, VTABLE_CLASS);
    count_up(e, COUNT_CLASS);
    object
}

/// A 0xC0-byte `hkpRagdollLimitsData` from the Havok allocator (memory-size word at `+4` =
/// 0xC0), constructed by `00d4f2a0`.
fn new_limits_data(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0xc0u32]).u32();
    e.mem.set_u16(block + 4, 0xc0);
    e.call(RAGDOLL_LIMITS_DATA_CONSTRUCT, &args![block]).u32()
}

/// Reads or writes `size` bytes at `destination` through the stream's file: the function
/// at `function` of the file object found at `stream + file_slot`, called as
/// `(file, buffer, size, &size, 1)`; the count it answers moves the file position (`+4`).
fn transfer(e: &mut Engine, stream: u32, file_slot: u32, function: u32, buffer: u32, size: u32) {
    let file = e.mem.u32(stream + file_slot);
    let routine = e.mem.u32(file + function);
    let count = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), size);
        e.call(routine, &args![file, buffer, size, slot, 1u32])
            .u32()
    });
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(count));
}

/// Appends `text` to the viewer-string array: grows it by its grow-by (`0096ad30`) when the
/// size (`+0xa`) has reached the capacity (`+8`), then stores through `0096ae90` (array,
/// index = the old size, address of the pointer).
fn push_viewer_string(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRINGS_GROW, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![array, size, slot]);
    });
}

// Translated from 00cc5e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc5e60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollLimitsConstraint::~bhkRagdollLimitsConstraint` (Xbox PDB): stores the class
/// vtable, runs `Kill` (`00c876a0`), decrements the class counter and runs the
/// `bhkConstraint` destructor (`00c89640`).
pub fn bhk_ragdoll_limits_constraint_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    e.call(KILL, &args![this]);
    let count: u32 = e.global(COUNT_CLASS);
    e.set_global(COUNT_CLASS, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCTOR, &args![this]);
}

// Translated from 00cc5f30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc5f30 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkRagdollLimitsConstraintCinfo::CloneData` (Xbox PDB), `(data, info)`: `data` is the
/// destination (a new 0xC0-byte `hkpRagdollLimitsData`, `00d4f2a0`, when it is 0). Copies the
/// six vectors at `+0x20` .. `+0x7f` of the data block held at `this + 4` (16-byte moves)
/// and the floats at `+0xa0`, `+0xb0`, `+0xb4`, `+0x88` and `+0x8c` (`FLD`/`FSTP` copies),
/// then answers what `hkConstraintCinfo::CloneData` (`00c8a1b0`) answers on
/// `(this, data, info)`.
pub fn hk_ragdoll_limits_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    info: Ptr,
) -> u32 {
    let source = e.mem.u32(this.addr() + 4);
    let destination = if data.addr() == 0 {
        new_limits_data(e)
    } else {
        data.addr()
    };
    for offset in (0x20..0x80u32).step_by(4) {
        let value = e.mem.u32(source + offset);
        e.mem.set_u32(destination + offset, value);
    }
    for offset in [0xa0u32, 0xb0, 0xb4, 0x88, 0x8c] {
        let value = x87_quiet_bits(e.mem.u32(source + offset));
        e.mem.set_u32(destination + offset, value);
    }
    e.call(CONSTRAINT_CINFO_CLONE_DATA, &args![this, destination, info])
        .u32()
}

// Translated from 00cc6030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc6030 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkRagdollLimitsConstraintCinfo::CreateData` (Xbox PDB): when `this + 4` is 0, builds a
/// 0xC0-byte `hkpRagdollLimitsData` (`00d4f2a0`) there. Answers 0xC0.
pub fn hk_ragdoll_limits_constraint_cinfo_create_data(e: &mut Engine, this: Ptr) -> u32 {
    if e.mem.u32(this.addr() + 4) == 0 {
        let data = new_limits_data(e);
        e.mem.set_u32(this.addr() + 4, data);
    }
    0xc0
}

// Translated from 00cc60b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc60b0 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (cdecl, no arguments): `new bhkRagdollLimitsConstraint`, 0 when the heap returns 0.
pub fn fn_00cc60b0(e: &mut Engine) -> u32 {
    construct_constraint(e)
}

// Translated from 00cc6140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc6140 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollLimitsConstraint::CreateClone` (Xbox PDB) with the clone lock held (`00c876f0`,
/// `00c87700`): builds a new `bhkRagdollLimitsConstraint` and runs
/// `bhkConstraint::CopyMembers` (`00c89730`) on `(this, clone, process)` (with 0 when the
/// allocation failed). Answers the clone.
pub fn bhk_ragdoll_limits_constraint_create_clone(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    e.call(CLONE_LOCK, &args![]);
    let clone = construct_constraint(e);
    e.call(CONSTRAINT_COPY_MEMBERS, &args![this, clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    clone
}

// Translated from 00cc61f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRagdollLimitsConstraintCinfo::LoadBinary` (Xbox PDB): `hkConstraintCinfo::LoadBinary`
/// (`00c8a230`) on `(this, stream)`, then reads through the read function of the stream's
/// file (`[stream + 0x24C]`, function at `+8`) the two 0x30-byte blocks at `+0x20` and
/// `+0x50` of the data block at `this + 4` (called as `(file, destination, 0x30, 0, 0)`)
/// and, unless the byte at `stream + 0x1e5` is 1, swaps both (`00c8cdb0`); then the floats
/// at `+0xa0`, `+0xb0`, `+0xb4`, `+0x88` and `+0x8c`, each as `(file, destination, 4, &4,
/// 1)`. Each count read is added to the file position at `+4`.
pub fn hk_ragdoll_limits_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(CONSTRAINT_CINFO_LOAD_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    for offset in [0x20u32, 0x50] {
        let file = e.mem.u32(stream.addr() + 0x24c);
        let read = e.mem.u32(file + 8);
        let count = e
            .call(read, &args![file, data + offset, 0x30u32, 0u32, 0u32])
            .u32();
        let position = e.mem.u32(file + 4);
        e.mem.set_u32(file + 4, position.wrapping_add(count));
    }
    if e.mem.u8(stream.addr() + 0x1e5) != 1 {
        e.call(SWAP_BLOCK, &args![data + 0x20]);
        e.call(SWAP_BLOCK, &args![data + 0x50]);
    }
    for offset in [0xa0u32, 0xb0, 0xb4, 0x88, 0x8c] {
        transfer(e, stream.addr(), 0x24c, 8, data + offset, 4);
    }
}

// Translated from 00cc6310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc6310 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollLimitsConstraint::CreateCinfo` (Xbox PDB), with the allocation scope word set
/// to 0x10: stores in the byte at `created` whether the cinfo at `+0xC` was missing; then
/// builds it: a 0x14-byte block from the memory manager (`00aa3e40`) whose `+8` is 1, `+0xC`
/// and `+0x10` are 0, vtable is `010c9be8` and whose `+4` is 0, the cinfo being the address
/// `block + 4` (0 when the manager answers 0), stored at `+0xC` and, when `+8` is not 0,
/// filled by `bhkConstraint::CopyCinfo` (`00c89c80`) given the block (0 for no cinfo).
/// Answers the cinfo.
pub fn bhk_ragdoll_limits_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(missing));
    if missing {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
            .u32();
        let cinfo = if block == 0 {
            0
        } else {
            e.mem.set_u32(block + 8, 1);
            e.mem.set_u32(block + 0xc, 0);
            e.mem.set_u32(block + 0x10, 0);
            e.mem.set_u32(block, VTABLE_CINFO_BLOCK);
            e.mem.set_u32(block + 4, 0);
            block + 4
        };
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if e.mem.u32(this.addr() + 8) != 0 {
            let argument = if cinfo == 0 { 0 } else { cinfo - 4 };
            e.call(CONSTRAINT_COPY_CINFO, &args![this, argument]);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(word, saved);
    cinfo
}

// Translated from 00cc63e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRagdollLimitsConstraintCinfo::SaveBinary` (Xbox PDB): `hkConstraintCinfo::SaveBinary`
/// (`00c8a280`) on `(this, stream)`; unless the byte at `stream + 0x1e4` is 1, swaps the two
/// 0x30-byte blocks at `+0x20` and `+0x50` of the data block at `this + 4` (`00c8cdb0`);
/// writes both blocks, then the floats at `+0xa0`, `+0xb0`, `+0xb4`, `+0x88` and `+0x8c`,
/// through the write function of the stream's file (`[stream + 0x250]`, function at `+0xC`,
/// `(file, source, size, &size, 1)`), adding each count to the file position at `+4`; and
/// swaps the two blocks back when the byte is not 1 (read again after the writes).
pub fn hk_ragdoll_limits_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(CONSTRAINT_CINFO_SAVE_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        e.call(SWAP_BLOCK, &args![data + 0x20]);
        e.call(SWAP_BLOCK, &args![data + 0x50]);
    }
    transfer(e, stream.addr(), 0x250, 0xc, data + 0x20, 0x30);
    transfer(e, stream.addr(), 0x250, 0xc, data + 0x50, 0x30);
    for offset in [0xa0u32, 0xb0, 0xb4, 0x88, 0x8c] {
        transfer(e, stream.addr(), 0x250, 0xc, data + offset, 4);
    }
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        e.call(SWAP_BLOCK, &args![data + 0x20]);
        e.call(SWAP_BLOCK, &args![data + 0x50]);
    }
}

// Translated from 00cc6540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRagdollLimitsConstraintCinfo::GetViewerStrings` (Xbox PDB), `(array)`:
/// `hkConstraintCinfo::GetViewerStrings` (`00c8a560`) on `(this, array)`, then, when the data
/// block at `this + 4` exists, appends one `NiGetViewerString` float string (`00a7d550`,
/// `(name, bits)`, the float copied by `FLD`/`FSTP`) for `ConeMinAngle` (`+0xa0`),
/// `PlaneMinAngle` (`+0xb0`), `PlaneMaxAngle` (`+0xb4`), `TwistMinAngle` (`+0x88`) and
/// `TwistMaxAngle` (`+0x8c`).
pub fn hk_ragdoll_limits_constraint_cinfo_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    array: Ptr,
) {
    e.call(CONSTRAINT_CINFO_GET_VIEWER_STRINGS, &args![this, array]);
    let data = e.mem.u32(this.addr() + 4);
    if data == 0 {
        return;
    }
    for (offset, name) in [
        (0xa0u32, NAME_CONE_MIN_ANGLE),
        (0xb0, NAME_PLANE_MIN_ANGLE),
        (0xb4, NAME_PLANE_MAX_ANGLE),
        (0x88, NAME_TWIST_MIN_ANGLE),
        (0x8c, NAME_TWIST_MAX_ANGLE),
    ] {
        let value = x87_quiet_bits(e.mem.u32(data + offset));
        let text = e.call(VIEWER_STRING_FLOAT, &args![name, value]).u32();
        push_viewer_string(e, array.addr(), text);
    }
}

// Translated from 00cc5f10(decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollLimitsConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc5e60`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ragdoll_limits_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_5e60;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc5ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollLimitsConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8604.
pub fn bhk_ragdoll_limits_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8604
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc5e60, bhk_ragdoll_limits_constraint_destructor(Ptr)),
        entry!(0x00cc5ed0, bhk_ragdoll_limits_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cc5f10, bhk_ragdoll_limits_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc5f30, hk_ragdoll_limits_constraint_cinfo_clone_data(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00cc6030, hk_ragdoll_limits_constraint_cinfo_create_data(Ptr) -> u32),
        entry!(0x00cc60b0, fn_00cc60b0() -> u32),
        entry!(0x00cc6140, bhk_ragdoll_limits_constraint_create_clone(Ptr, u32) -> u32),
        entry!(
            0x00cc61f0,
            hk_ragdoll_limits_constraint_cinfo_load_binary(Ptr, Ptr)
        ),
        entry!(0x00cc6310, bhk_ragdoll_limits_constraint_create_cinfo(Ptr, Ptr) -> u32),
        entry!(
            0x00cc63e0,
            hk_ragdoll_limits_constraint_cinfo_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00cc6540,
            hk_ragdoll_limits_constraint_cinfo_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_ragdoll_limits_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_5e60, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc5f10, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc5f10, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_5e60), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ragdoll_limits_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_5ed0, &args![]).u32(), 0x0126_8604);
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

    fn map_counters(e: &mut Engine) {
        for counter in [COUNT_BASE, COUNT_CLASS] {
            e.map(counter, 4);
            e.set_global(counter, 10u32);
        }
    }

    /// A vtable of `slots` (byte offset, target) in front of a 0x40-byte object.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    /// Makes the memory router answer an allocator whose slot 4 returns `block`, recording
    /// the requested size in the word at `block + 0xc8`.
    fn route_allocation(e: &mut Engine, block: u32) {
        let allocator = object(e, &[(4, 0x0900_0004)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0004, move |e, a| {
            e.mem.set_u32(block + 0xc8, a[1]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
    }

    /// A wrapper whose data block (at `+4`) holds the words 0x100.. at `+0x20`..`0x80` and
    /// the five floats 1.0 .. 5.0 at `+0xa0`, `+0xb0`, `+0xb4`, `+0x88`, `+0x8c`.
    fn wrapper_with_data(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0xd0);
        e.mem.set_u32(this + 4, data);
        for word in 0..24u32 {
            e.mem.set_u32(data + 0x20 + 4 * word, 0x100 + word);
        }
        for (offset, value) in [
            (0xa0u32, 1.0f32),
            (0xb0, 2.0),
            (0xb4, 3.0),
            (0x88, 4.0),
            (0x8c, 5.0),
        ] {
            e.mem.set_f32(data + offset, value);
        }
        (this, data)
    }

    #[test]
    fn destructor_stores_the_vtable_kills_and_destroys_the_base() {
        let mut e = Engine::new();
        map_counters(&mut e);
        e.register(KILL, nothing);
        e.register(CONSTRAINT_DESTRUCTOR, nothing);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00cc5e60, &args![this]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00cc5e60, KILL, CONSTRAINT_DESTRUCTOR]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 9);
    }

    #[test]
    fn clone_data_copies_the_vectors_and_floats_into_the_given_data() {
        let mut e = Engine::new();
        let (this, _) = wrapper_with_data(&mut e);
        let destination = e.mem.alloc(0xd0);
        let info = e.mem.alloc(0x20);
        e.register(CONSTRAINT_CINFO_CLONE_DATA, |_, _| Ret {
            eax: 0xc10e,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00cc5f30, &args![this, destination, info]).u32();
        assert_eq!(answer, 0xc10e);
        for word in 0..24u32 {
            assert_eq!(e.mem.u32(destination + 0x20 + 4 * word), 0x100 + word);
        }
        assert_eq!(e.mem.f32(destination + 0xa0), 1.0);
        assert_eq!(e.mem.f32(destination + 0xb0), 2.0);
        assert_eq!(e.mem.f32(destination + 0xb4), 3.0);
        assert_eq!(e.mem.f32(destination + 0x88), 4.0);
        assert_eq!(e.mem.f32(destination + 0x8c), 5.0);
        assert_eq!(e.mem.u32(destination + 0x80), 0);
        assert_eq!(
            calls(&e, CONSTRAINT_CINFO_CLONE_DATA),
            [[this, destination, info]]
        );
    }

    #[test]
    fn clone_data_builds_a_new_data_block_when_none_is_given_and_quiets_nan_floats() {
        let mut e = Engine::new();
        let (this, data) = wrapper_with_data(&mut e);
        e.mem.set_u32(data + 0xb4, 0x7f80_0001);
        e.mem.set_u32(data + 0x40, 0x7f80_0001);
        let block = e.mem.alloc(0xd0);
        route_allocation(&mut e, block);
        e.register(RAGDOLL_LIMITS_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CONSTRAINT_CINFO_CLONE_DATA, nothing);
        let info = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00cc5f30, &args![this, 0u32, info]);
        assert_eq!(e.mem.u32(block + 0xc8), 0xc0);
        assert_eq!(e.mem.u16(block + 4), 0xc0);
        assert_eq!(calls(&e, RAGDOLL_LIMITS_DATA_CONSTRUCT), [[block]]);
        // The vector move keeps the signalling NaN; the FLD/FSTP copy quiets it.
        assert_eq!(e.mem.u32(block + 0x40), 0x7f80_0001);
        assert_eq!(e.mem.u32(block + 0xb4), 0x7fc0_0001);
        assert_eq!(e.mem.f32(block + 0x8c), 5.0);
        assert_eq!(
            calls(&e, CONSTRAINT_CINFO_CLONE_DATA),
            [[this, block, info]]
        );
    }

    #[test]
    fn create_data_builds_the_data_block_once() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0xd0);
        route_allocation(&mut e, block);
        e.register(RAGDOLL_LIMITS_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc6030, &args![this]).u32(), 0xc0);
        assert_eq!(e.mem.u32(this + 4), block);
        assert_eq!(e.mem.u16(block + 4), 0xc0);
        assert_eq!(e.mem.u32(block + 0xc8), 0xc0);
        assert_eq!(e.call(0x00cc6030, &args![this]).u32(), 0xc0);
        assert_eq!(calls(&e, RAGDOLL_LIMITS_DATA_CONSTRUCT).len(), 1);
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
        assert_eq!(e.call(0x00cc60b0, &args![]).u32(), block);
        assert_eq!(calls(&e, HEAP_ALLOCATE), [[0x10]]);
        assert_eq!(calls(&e, REF_OBJECT_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_BASE), 11);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
        e.register(HEAP_ALLOCATE, nothing);
        assert_eq!(e.call(0x00cc60b0, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
    }

    #[test]
    fn create_clone_copies_members_between_the_clone_lock_calls() {
        let mut e = Engine::new();
        map_counters(&mut e);
        let block = e.mem.alloc(0x10);
        e.register_double(HEAP_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        for address in [
            REF_OBJECT_CONSTRUCT,
            CLONE_LOCK,
            CLONE_UNLOCK,
            CONSTRAINT_COPY_MEMBERS,
        ] {
            e.register(address, nothing);
        }
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc6140, &args![this, 0x7700u32]).u32(), block);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00cc6140,
                CLONE_LOCK,
                HEAP_ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                CONSTRAINT_COPY_MEMBERS,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(calls(&e, CONSTRAINT_COPY_MEMBERS), [[this, block, 0x7700]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        // A failed allocation still runs the copy, with 0.
        e.register(HEAP_ALLOCATE, nothing);
        assert_eq!(e.call(0x00cc6140, &args![this, 0x7700u32]).u32(), 0);
        assert_eq!(calls(&e, CONSTRAINT_COPY_MEMBERS)[1], [this, 0, 0x7700]);
    }

    /// A wrapper with a data block at `+4` and a stream whose file (read at `+0x24C`, write
    /// at `+0x250`) has the functions 0x0900_0040 (read) and 0x0900_0044 (write).
    fn stream_setup(e: &mut Engine) -> (u32, u32, u32, u32) {
        let (this, data) = wrapper_with_data(e);
        let file = object(e, &[]);
        e.mem.set_u32(file + 8, 0x0900_0040);
        e.mem.set_u32(file + 0xc, 0x0900_0044);
        e.mem.set_u32(file + 4, 100);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 0x250, file);
        (this, data, stream, file)
    }

    #[test]
    fn load_binary_reads_two_blocks_swaps_them_and_reads_five_floats() {
        for flag in [0u8, 1] {
            let mut e = Engine::new();
            let (this, data, stream, file) = stream_setup(&mut e);
            e.mem.set_u8(stream + 0x1e5, flag);
            e.register(CONSTRAINT_CINFO_LOAD_BINARY, nothing);
            e.register(SWAP_BLOCK, nothing);
            e.register(0x0900_0040, |_, a| Ret {
                eax: a[2],
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x00cc61f0, &args![this, stream]);
            assert_eq!(calls(&e, CONSTRAINT_CINFO_LOAD_BINARY), [[this, stream]]);
            let reads = calls(&e, 0x0900_0040);
            assert_eq!(
                reads[..2],
                [
                    vec![file, data + 0x20, 0x30, 0, 0],
                    vec![file, data + 0x50, 0x30, 0, 0]
                ]
            );
            let floats: Vec<u32> = reads[2..].iter().map(|c| c[1] - data).collect();
            assert_eq!(floats, [0xa0, 0xb0, 0xb4, 0x88, 0x8c]);
            for call in &reads[2..] {
                assert_eq!((call[2], call[4]), (4, 1));
            }
            assert_eq!(e.mem.u32(file + 4), 100 + 0x60 + 5 * 4);
            let swaps = calls(&e, SWAP_BLOCK);
            if flag == 1 {
                assert!(swaps.is_empty());
            } else {
                assert_eq!(swaps, [[data + 0x20], [data + 0x50]]);
            }
        }
    }

    #[test]
    fn create_cinfo_builds_the_block_once_and_fills_it_when_the_havok_object_exists() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        let block = e.mem.alloc(0x14);
        e.register_double(MEMORY_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(CONSTRAINT_COPY_CINFO, nothing);
        let this = e.mem.alloc(0x20);
        let created = e.mem.alloc(4);
        e.mem.set_u32(this + 8, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc6310, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), [[MEMORY_MANAGER, 0x14]]);
        assert_eq!(e.mem.u32(block), VTABLE_CINFO_BLOCK);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0xc), block + 4);
        assert_eq!(calls(&e, CONSTRAINT_COPY_CINFO), [[this, block]]);
        // Second time: nothing is built.
        assert_eq!(e.call(0x00cc6310, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
        assert_eq!(e.mem.u32(word), 0x55);
        // No Havok object: no copy; a failed allocation leaves 0.
        let bare = e.mem.alloc(0x20);
        e.register(MEMORY_ALLOCATE, nothing);
        assert_eq!(e.call(0x00cc6310, &args![bare, created]).u32(), 0);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls(&e, CONSTRAINT_COPY_CINFO).len(), 1);
        e.mem.set_u32(bare + 8, 1);
        e.mem.set_u32(bare + 0xc, 0);
        e.call(0x00cc6310, &args![bare, created]);
        assert_eq!(calls(&e, CONSTRAINT_COPY_CINFO)[1], [bare, 0]);
    }

    #[test]
    fn save_binary_swaps_around_the_writes_unless_the_flag_is_one() {
        for flag in [0u8, 1] {
            let mut e = Engine::new();
            let (this, data, stream, file) = stream_setup(&mut e);
            e.mem.set_u8(stream + 0x1e4, flag);
            e.register(CONSTRAINT_CINFO_SAVE_BINARY, nothing);
            e.register(SWAP_BLOCK, nothing);
            e.register(0x0900_0044, |_, a| Ret {
                eax: a[2],
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x00cc63e0, &args![this, stream]);
            assert_eq!(calls(&e, CONSTRAINT_CINFO_SAVE_BINARY), [[this, stream]]);
            let writes = calls(&e, 0x0900_0044);
            let sources: Vec<u32> = writes.iter().map(|c| c[1] - data).collect();
            assert_eq!(sources, [0x20, 0x50, 0xa0, 0xb0, 0xb4, 0x88, 0x8c]);
            let sizes: Vec<u32> = writes.iter().map(|c| c[2]).collect();
            assert_eq!(sizes, [0x30, 0x30, 4, 4, 4, 4, 4]);
            assert!(writes.iter().all(|c| c[0] == file && c[4] == 1));
            assert_eq!(e.mem.u32(file + 4), 100 + 0x60 + 5 * 4);
            if flag == 1 {
                assert!(calls(&e, SWAP_BLOCK).is_empty());
            } else {
                assert_eq!(
                    calls(&e, SWAP_BLOCK),
                    [[data + 0x20], [data + 0x50], [data + 0x20], [data + 0x50]]
                );
                // Two swaps, seven writes, two swaps.
                let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
                let swap_positions: Vec<usize> = order
                    .iter()
                    .enumerate()
                    .filter(|(_, a)| **a == SWAP_BLOCK)
                    .map(|(i, _)| i)
                    .collect();
                assert_eq!(swap_positions, [2, 3, 11, 12]);
            }
        }
    }

    #[test]
    fn get_viewer_strings_appends_five_angle_strings_when_the_data_exists() {
        let mut e = Engine::new();
        let (this, data) = wrapper_with_data(&mut e);
        e.mem.set_u32(data + 0xb0, 0x7f80_0001);
        let array = e.mem.alloc(0x10);
        e.mem.set_u16(array + 8, 64);
        e.register(CONSTRAINT_CINFO_GET_VIEWER_STRINGS, nothing);
        e.register(VIEWER_STRING_FLOAT, |_, a| Ret {
            eax: 0x3000 + a[0] % 0x100,
            ..Ret::default()
        });
        e.register_double(STRINGS_SET_AT, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(0x0900_1000 + 4 * a[1], text);
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.map(0x0900_1000, 0x100);
        e.call_log = Some(vec![]);
        e.call(0x00cc6540, &args![this, array]);
        assert_eq!(
            calls(&e, CONSTRAINT_CINFO_GET_VIEWER_STRINGS),
            [[this, array]]
        );
        let floats = calls(&e, VIEWER_STRING_FLOAT);
        let names: Vec<u32> = floats.iter().map(|c| c[0]).collect();
        assert_eq!(
            names,
            [
                NAME_CONE_MIN_ANGLE,
                NAME_PLANE_MIN_ANGLE,
                NAME_PLANE_MAX_ANGLE,
                NAME_TWIST_MIN_ANGLE,
                NAME_TWIST_MAX_ANGLE
            ]
        );
        let values: Vec<u32> = floats.iter().map(|c| c[1]).collect();
        assert_eq!(
            values,
            [
                1.0f32.to_bits(),
                0x7fc0_0001,
                3.0f32.to_bits(),
                4.0f32.to_bits(),
                5.0f32.to_bits()
            ]
        );
        assert_eq!(e.mem.u16(array + 0xa), 5);
        assert_eq!(e.mem.u32(0x0900_1000), 0x3040);
        // No data block: only the base runs.
        let bare = e.mem.alloc(0x20);
        e.call(0x00cc6540, &args![bare, array]);
        assert_eq!(calls(&e, VIEWER_STRING_FLOAT).len(), 5);
    }

    #[test]
    fn get_viewer_strings_grows_a_full_array() {
        let mut e = Engine::new();
        let (this, _) = wrapper_with_data(&mut e);
        let array = e.mem.alloc(0x10);
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 8);
        e.register(CONSTRAINT_CINFO_GET_VIEWER_STRINGS, nothing);
        e.register(VIEWER_STRING_FLOAT, nothing);
        e.register(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, 64);
            Ret::default()
        });
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00cc6540, &args![this, array]);
        assert_eq!(calls(&e, STRINGS_GROW), [[array, 8]]);
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 5);
    }
}
