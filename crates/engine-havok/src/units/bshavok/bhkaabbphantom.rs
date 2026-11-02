//! `BSHavok/bhkaabbphantom.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkAabbPhantom` (Xbox PDB) is a 0x18-byte `bhkPhantom`: `+0x8` is the Havok phantom
//! (`hkpAabbPhantom`, 0xE0 bytes), `+0xC` the cinfo that `CreateCinfo` builds on demand (a
//! 0x50-byte block aligned to 16 bytes, its `+0x20`/`+0x30` the two AABB corners) and `+0x14` a
//! byte the constructor clears.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame or
//! a `/GS` cookie check (`00ec408c`), neither of which is modeled. The `bhkPhantom` base
//! constructors that the compiler inlined write their class vtables one after the other; only
//! the last one survives, and the intermediate stores are written here too.

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
/// `bhkRefObject` constructor (`00c87c30`: vtable `010c5b54`, clears `+8`).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable::Kill` (Xbox PDB).
const KILL: u32 = 0x00c8_76a0;
/// `bhkPhantom::~bhkPhantom` (Xbox PDB).
const PHANTOM_DESTRUCTOR: u32 = 0x00c9_f3b0;
/// `bhkSerializable::CloneCritLock` and `CloneCritUnlock` (Xbox PDB).
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkWorldObject::CopyMembers` (`this, clone, process`), `LoadBinary`, `SaveBinary`
/// (`this, stream`) and `CopyCinfo` (`this, cinfo`), all Xbox PDB names.
const WORLD_OBJECT_COPY_MEMBERS: u32 = 0x00c8_5900;
const WORLD_OBJECT_LOAD_BINARY: u32 = 0x00c8_5f20;
const WORLD_OBJECT_SAVE_BINARY: u32 = 0x00c8_6060;
const WORLD_OBJECT_COPY_CINFO: u32 = 0x00c8_5880;
/// `bhkPhantom::GetViewerStrings` (Xbox PDB).
const PHANTOM_GET_VIEWER_STRINGS: u32 = 0x00c9_f480;
/// Writes the two AABB corners of the phantom (`this`) to the 32 bytes at its argument
/// (`006288c0`, unnamed in the engine map).
const WRITE_AABB: u32 = 0x0062_88c0;
/// `hkpAabbPhantom::hkpAabbPhantom` (Xbox PDB): `(this, aabb, collisionFilterInfo)`.
const HKP_AABB_PHANTOM_CONSTRUCT: u32 = 0x00cf_2cb0;
/// `hkReferencedObject` release (`00c905b0`, `this` only).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `NiGetViewerString` and the two array helpers of the viewer-string array.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The word the class passes to `NiGetViewerString`.
const CLASS_NAME_POINTER: u32 = 0x0126_8248;

/// Class vtable and the two base-class vtables the inlined constructors store before it.
const VTABLE_CLASS: u32 = 0x0103_0864;
const VTABLE_PHANTOM: u32 = 0x0103_078c;
const VTABLE_SERIALIZABLE: u32 = 0x0103_0394;
/// Object counters of the three constructors (`bhkSerializable`, `bhkPhantom`,
/// `bhkAabbPhantom`).
const COUNT_SERIALIZABLE: u32 = 0x0126_810c;
const COUNT_PHANTOM: u32 = 0x0126_8238;
const COUNT_CLASS: u32 = 0x0126_8244;

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

/// The byte swap of the eight cinfo words (`+0x30..+0x3C`, then `+0x20..+0x2C`) that
/// `LoadBinary` and `SaveBinary` apply when the stream's flag byte is not 1.
fn swap_cinfo_words(e: &mut Engine, cinfo: u32) {
    for offset in [0x30u32, 0x34, 0x38, 0x3c, 0x20, 0x24, 0x28, 0x2c] {
        let word = e.mem.u32(cinfo.wrapping_add(offset));
        e.mem.set_u32(cinfo.wrapping_add(offset), word.swap_bytes());
    }
}

/// The inlined `new bhkAabbPhantom`: a 0x18-byte block from the game heap, the base
/// constructors and the three counters; 0 when the allocation fails.
fn construct_aabb_phantom(e: &mut Engine) -> u32 {
    let object = e.call(HEAP_ALLOCATE, &args![0x18u32]).u32();
    if object == 0 {
        return 0;
    }
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_SERIALIZABLE);
    e.mem.set_u32(object + 0xc, 0);
    e.mem.set_u32(object + 0x10, 0);
    count_up(e, COUNT_SERIALIZABLE);
    e.mem.set_u32(object, VTABLE_PHANTOM);
    count_up(e, COUNT_PHANTOM);
    e.mem.set_u32(object, VTABLE_CLASS);
    count_up(e, COUNT_CLASS);
    e.mem.set_u8(object + 0x14, 0);
    object
}

// Translated from 00c9f530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f530 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAabbPhantom::~bhkAabbPhantom` (Xbox PDB): stores the class vtable, runs `Kill`
/// (`00c876a0`), decrements the class counter and runs the `bhkPhantom` destructor
/// (`00c9f3b0`).
pub fn bhk_aabb_phantom_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    e.call(KILL, &args![this]);
    let count: u32 = e.global(COUNT_CLASS);
    e.set_global(COUNT_CLASS, count.wrapping_sub(1));
    e.call(PHANTOM_DESTRUCTOR, &args![this]);
}

// Translated from 00c9f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f5a0 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (cdecl, no arguments): `new bhkAabbPhantom`, 0 when the heap returns 0. The
/// cdecl entry takes no argument words.
pub fn fn_00c9f5a0(e: &mut Engine) -> u32 {
    construct_aabb_phantom(e)
}

// Translated from 00c9f640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f640 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAabbPhantom::CreateClone` (Xbox PDB): with the clone lock held (`00c876f0`,
/// `00c87700`) builds a new `bhkAabbPhantom` and runs `bhkWorldObject::CopyMembers`
/// (`00c85900`) on `(this, clone, process)`. Answers the clone (0 when the allocation
/// failed; `CopyMembers` still runs with it).
pub fn bhk_aabb_phantom_create_clone(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    e.call(CLONE_LOCK, &args![]);
    let clone = construct_aabb_phantom(e);
    e.call(WORLD_OBJECT_COPY_MEMBERS, &args![this, clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    clone
}

// Translated from 00c9f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAabbPhantom::LoadBinary` (Xbox PDB): `bhkWorldObject::LoadBinary` (`00c85f20`), then
/// the cinfo (virtual slot 0xB4, `CreateCinfo`, given a scratch out-pointer): when the byte
/// at `stream + 0x1E5` is not 1 the eight cinfo words are byte swapped.
pub fn bhk_aabb_phantom_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(WORLD_OBJECT_LOAD_BINARY, &args![this, stream]);
    let cinfo = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
    if e.mem.u8(stream.addr() + 0x1e5) != 1 {
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00c9f770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAabbPhantom::SaveBinary` (Xbox PDB): gets the cinfo (virtual slot 0xB4), byte swaps
/// its eight words when the byte at `stream + 0x1E4` is not 1, runs
/// `bhkWorldObject::SaveBinary` (`00c86060`) and swaps them back under the same test.
pub fn bhk_aabb_phantom_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let cinfo = e.with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]).u32());
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
    e.call(WORLD_OBJECT_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream.addr() + 0x1e4) != 1 {
        swap_cinfo_words(e, cinfo);
    }
}

// Translated from 00c9f830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f830 - the /GS cookie check (00ec408c) is not modeled.
/// `bhkAabbPhantom::CopyCinfo` (Xbox PDB): `bhkWorldObject::CopyCinfo` (`00c85880`) on
/// `(this, cinfo)`, then the phantom's AABB (`006288c0`, 32 bytes) is copied to
/// `cinfo + 0x20` and `cinfo + 0x30`.
pub fn bhk_aabb_phantom_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: Ptr) {
    e.call(WORLD_OBJECT_COPY_CINFO, &args![this, cinfo]);
    e.with_stack(0x20, |e, aabb| {
        e.call(WRITE_AABB, &args![this, aabb]);
        for lane in 0..8u32 {
            let word = e.mem.u32(aabb.addr() + 4 * lane);
            e.mem.set_u32(cinfo.addr() + 0x20 + 4 * lane, word);
        }
    });
}

// Translated from 00c9f890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f890 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAabbPhantom::Init` (Xbox PDB), with the allocation scope word set to 0x10: builds
/// the 0xE0-byte `hkpAabbPhantom` from the cinfo (`00cf2cb0`, `(cinfo + 0x20, cinfo[0])`;
/// its memory-size word at `+4` is set to 0xE0), hands it to virtual slot 0x8C
/// (`SethkObject`), releases the creation reference (`00c905b0`) and runs virtual slot
/// 0xBC (`PostInit`) with the cinfo.
pub fn bhk_aabb_phantom_init(e: &mut Engine, this: Ptr, info: Ptr) {
    let (word, saved) = enter_scope(e);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let phantom = e.vcall(allocator, 4, &args![0xe0u32]).u32();
    e.mem.set_u16(phantom + 4, 0xe0);
    let collision_info = e.mem.u32(info.addr());
    let phantom = e
        .call(
            HKP_AABB_PHANTOM_CONSTRUCT,
            &args![phantom, info.addr() + 0x20, collision_info],
        )
        .u32();
    e.vcall(this.addr(), 0x8c, &args![phantom]);
    e.call(REMOVE_REFERENCE, &args![phantom]);
    e.vcall(this.addr(), 0xbc, &args![info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00c9f970 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f970 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAabbPhantom::GetViewerStrings` (Xbox PDB), with the allocation scope word set to
/// 0x10: runs `bhkPhantom::GetViewerStrings` (`00c9f480`) on `(this, strings)`, then
/// appends the string `NiGetViewerString` makes of the word at `01268248` to the array
/// (grown by the step at `+0xE` plus the size when the 16-bit size at `+0xA` has reached
/// the capacity at `+0x8`).
pub fn bhk_aabb_phantom_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let (word, saved) = enter_scope(e);
    e.call(PHANTOM_GET_VIEWER_STRINGS, &args![this, strings]);
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

// Translated from 00c9fa20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9fa20 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAabbPhantom::CreateCinfo` (Xbox PDB), with the allocation scope word set to 0x10:
/// stores in the byte at `created` whether the cinfo at `+0xC` was missing; then builds it:
/// a 0x50-byte block from the memory manager (`00aa3e40`) moved up to the next 16-byte
/// boundary (the distance, 1 to 16, is stored in the byte before the cinfo), with `+0xC`,
/// `+0x10` and `+4`, `+0` cleared, `+0x14` = 0x80000000, the byte at `+8` = 2 and the
/// two vectors at `+0x20` and `+0x30` zeroed. When the phantom's Havok object (`+8`)
/// exists, `CopyCinfo` (`00c9f830`) fills it. Answers the cinfo.
pub fn bhk_aabb_phantom_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(missing));
    if missing {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x50u32])
            .u32();
        let pad = 0x10 - (block & 0xf);
        let cinfo = block.wrapping_add(pad);
        e.mem.set_u8(cinfo.wrapping_sub(1), pad as u8);
        e.mem.set_u32(cinfo + 0xc, 0);
        e.mem.set_u32(cinfo + 0x10, 0);
        e.mem.set_u32(cinfo + 0x14, 0x8000_0000);
        e.mem.set_u32(cinfo, 0);
        e.mem.set_u32(cinfo + 4, 0);
        e.mem.set_u8(cinfo + 8, 2);
        for lane in 0..8u32 {
            e.mem.set_u32(cinfo + 0x20 + 4 * lane, 0);
        }
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if e.mem.u32(this.addr() + 8) != 0 {
            bhk_aabb_phantom_copy_cinfo(e, this, Ptr::new(cinfo));
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(word, saved);
    cinfo
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9f530, bhk_aabb_phantom_destructor(Ptr)),
        entry!(0x00c9f5a0, fn_00c9f5a0() -> u32),
        entry!(0x00c9f640, bhk_aabb_phantom_create_clone(Ptr, u32) -> u32),
        entry!(0x00c9f700, bhk_aabb_phantom_load_binary(Ptr, Ptr)),
        entry!(0x00c9f770, bhk_aabb_phantom_save_binary(Ptr, Ptr)),
        entry!(0x00c9f830, bhk_aabb_phantom_copy_cinfo(Ptr, Ptr)),
        entry!(0x00c9f890, bhk_aabb_phantom_init(Ptr, Ptr)),
        entry!(0x00c9f970, bhk_aabb_phantom_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00c9fa20, bhk_aabb_phantom_create_cinfo(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

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
        for counter in [COUNT_SERIALIZABLE, COUNT_PHANTOM, COUNT_CLASS] {
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

    #[test]
    fn destructor_stores_the_vtable_kills_and_destroys_the_base() {
        let mut e = Engine::new();
        map_counters(&mut e);
        e.register(KILL, nothing);
        e.register(PHANTOM_DESTRUCTOR, nothing);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00c9f530, &args![this]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00c9f530, KILL, PHANTOM_DESTRUCTOR]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 9);
        assert_eq!(calls(&e, PHANTOM_DESTRUCTOR), [[this]]);
    }

    #[test]
    fn new_builds_the_object_and_counts_it_or_answers_zero() {
        let mut e = Engine::new();
        map_counters(&mut e);
        let block = e.mem.alloc(0x18);
        e.register_double(HEAP_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(REF_OBJECT_CONSTRUCT, nothing);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c9f5a0, &args![]).u32(), block);
        assert_eq!(calls(&e, HEAP_ALLOCATE), [[0x18]]);
        assert_eq!(calls(&e, REF_OBJECT_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u8(block + 0x14), 0);
        for counter in [COUNT_SERIALIZABLE, COUNT_PHANTOM, COUNT_CLASS] {
            assert_eq!(e.global::<u32>(counter), 11);
        }
        e.register(HEAP_ALLOCATE, nothing);
        assert_eq!(e.call(0x00c9f5a0, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
    }

    #[test]
    fn create_clone_copies_members_between_the_clone_lock_calls() {
        let mut e = Engine::new();
        map_counters(&mut e);
        let block = e.mem.alloc(0x18);
        e.register_double(HEAP_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        for address in [
            REF_OBJECT_CONSTRUCT,
            CLONE_LOCK,
            CLONE_UNLOCK,
            WORLD_OBJECT_COPY_MEMBERS,
        ] {
            e.register(address, nothing);
        }
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c9f640, &args![this, 0x7700u32]).u32(), block);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00c9f640,
                CLONE_LOCK,
                HEAP_ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                WORLD_OBJECT_COPY_MEMBERS,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(
            calls(&e, WORLD_OBJECT_COPY_MEMBERS),
            [[this, block, 0x7700]]
        );
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
    }

    /// An engine whose phantom object answers slot 0xB4 with `cinfo` (eight words
    /// 0x01020304 + lane) and a stream with the flag bytes at `+0x1E4` and `+0x1E5`.
    fn swap_setup(flag: u8) -> (Engine, Ptr, Ptr, u32) {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x50);
        e.register_double(0x0900_00b4, move |_, _| Ret {
            eax: cinfo,
            ..Ret::default()
        });
        let this = Ptr::new(object(&mut e, &[(0xb4, 0x0900_00b4)]));
        for lane in 0..8u32 {
            e.mem.set_u32(cinfo + 0x20 + 4 * lane, 0x0102_0304 + lane);
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
            e.register(WORLD_OBJECT_LOAD_BINARY, nothing);
            e.call_log = Some(vec![]);
            e.call(0x00c9f700, &args![this, stream]);
            assert_eq!(
                calls(&e, WORLD_OBJECT_LOAD_BINARY),
                [[this.addr(), stream.addr()]]
            );
            let word = e.mem.u32(cinfo + 0x20);
            if swapped {
                assert_eq!(word, 0x0403_0201);
                assert_eq!(e.mem.u32(cinfo + 0x3c), (0x0102_0304u32 + 7).swap_bytes());
            } else {
                assert_eq!(word, 0x0102_0304);
            }
        }
    }

    #[test]
    fn save_binary_swaps_around_the_base_save_and_restores_the_words() {
        for (flag, swapped) in [(0u8, true), (1, false)] {
            let (mut e, this, stream, cinfo) = swap_setup(flag);
            e.register_double(WORLD_OBJECT_SAVE_BINARY, move |e, _| {
                let seen = e.mem.u32(cinfo + 0x20);
                e.mem.set_u32(cinfo + 0x48, seen);
                Ret::default()
            });
            e.call(0x00c9f770, &args![this, stream]);
            let during = e.mem.u32(cinfo + 0x48);
            if swapped {
                assert_eq!(during, 0x0403_0201);
            } else {
                assert_eq!(during, 0x0102_0304);
            }
            assert_eq!(e.mem.u32(cinfo + 0x20), 0x0102_0304);
            assert_eq!(e.mem.u32(cinfo + 0x2c), 0x0102_0304 + 3);
        }
    }

    #[test]
    fn copy_cinfo_copies_the_aabb_to_both_corners() {
        let mut e = Engine::new();
        e.register(WORLD_OBJECT_COPY_CINFO, nothing);
        e.register(WRITE_AABB, |e, a| {
            for lane in 0..8u32 {
                e.mem.set_u32(a[1] + 4 * lane, 0x100 + lane);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x18);
        let cinfo = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        e.call(0x00c9f830, &args![this, cinfo]);
        assert_eq!(calls(&e, WORLD_OBJECT_COPY_CINFO), [[this, cinfo]]);
        for lane in 0..8u32 {
            assert_eq!(e.mem.u32(cinfo + 0x20 + 4 * lane), 0x100 + lane);
        }
        assert_eq!(calls(&e, WRITE_AABB)[0][0], this);
    }

    #[test]
    fn init_builds_the_phantom_inside_the_scope_word() {
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
        let phantom = e.mem.alloc(0xe0);
        e.register_double(0x0900_0004, move |e, a| {
            assert_eq!(a[1], 0xe0);
            let scope = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(scope), 0x10);
            Ret {
                eax: phantom,
                ..Ret::default()
            }
        });
        e.register(HKP_AABB_PHANTOM_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x0900_008c, nothing);
        e.register(0x0900_00bc, nothing);
        e.register(REMOVE_REFERENCE, nothing);
        let this = object(&mut e, &[(0x8c, 0x0900_008c), (0xbc, 0x0900_00bc)]);
        let info = e.mem.alloc(0x60);
        e.mem.set_u32(info, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x00c9f890, &args![this, info]);
        assert_eq!(e.mem.u16(phantom + 4), 0xe0);
        assert_eq!(
            calls(&e, HKP_AABB_PHANTOM_CONSTRUCT),
            [[phantom, info + 0x20, 0x1234]]
        );
        assert_eq!(calls(&e, 0x0900_008c), [[this, phantom]]);
        assert_eq!(calls(&e, REMOVE_REFERENCE), [[phantom]]);
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
            e.register(PHANTOM_GET_VIEWER_STRINGS, nothing);
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(STRINGS_GROW, nothing);
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(STRINGS_SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            let this = e.mem.alloc(0x18);
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.call_log = Some(vec![]);
            e.call(0x00c9f970, &args![this, strings]);
            assert_eq!(calls(&e, PHANTOM_GET_VIEWER_STRINGS), [[this, strings]]);
            assert_eq!(calls(&e, NI_GET_VIEWER_STRING), [[0x5151_0001]]);
            let grown = calls(&e, STRINGS_GROW);
            if size >= capacity {
                assert_eq!(grown, [[strings, 5 + u32::from(size)]]);
            } else {
                assert!(grown.is_empty());
            }
            assert_eq!(
                &calls(&e, STRINGS_SET_AT)[0][..2],
                [strings, u32::from(size)]
            );
            assert_eq!(stored.get(), 0x7777_0001);
            assert_eq!(e.mem.u32(word), 0x66);
        }
    }

    #[test]
    fn create_cinfo_builds_an_aligned_block_once() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x66);
        let block = e.mem.alloc(0x80) + 5;
        e.register_double(MEMORY_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(WORLD_OBJECT_COPY_CINFO, nothing);
        e.register(WRITE_AABB, nothing);
        let this = e.mem.alloc(0x18);
        let created = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00c9fa20, &args![this, created]).u32();
        let pad = 0x10 - (block & 0xf);
        assert_eq!(cinfo, block + pad);
        assert_eq!(e.mem.u8(cinfo - 1), pad as u8);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert_eq!(e.mem.u32(cinfo + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u8(cinfo + 8), 2);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), [[MEMORY_MANAGER, 0x50]]);
        // No Havok object yet: CopyCinfo is not run.
        assert!(calls(&e, WORLD_OBJECT_COPY_CINFO).is_empty());
        // The second time the cinfo exists: nothing is allocated.
        assert_eq!(e.call(0x00c9fa20, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
        // With a Havok object, a fresh cinfo is filled by CopyCinfo.
        let other = e.mem.alloc(0x18);
        e.mem.set_u32(other + 8, 0x4444);
        e.call(0x00c9fa20, &args![other, created]);
        assert_eq!(calls(&e, WORLD_OBJECT_COPY_CINFO).len(), 1);
        assert_eq!(e.mem.u32(word), 0x66);
    }
}
