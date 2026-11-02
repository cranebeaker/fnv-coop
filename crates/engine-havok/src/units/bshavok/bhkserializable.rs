//! `BSHavok/bhkserializable.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkSerializable` (vtable `0102023c`) is the 0x10-byte part of the Havok wrapper objects
//! that can be saved and loaded: the Havok object sits at `+8` and the Cinfo at `+0xc`.
//!
//! The exception-unwinding frames (`FS:[0]`) of the functions marked `Translation incomplete`
//! are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObject::CopyMembers` (`00a5d450`, engine map unnamed; thiscall: clone, process) and
/// `NiObject::LoadBinary` (`00a5d610`, thiscall: stream).
const NI_COPY_MEMBERS: u32 = 0x00a5_d450;
const NI_LOAD_BINARY: u32 = 0x00a5_d610;
/// The function the save and link paths call first (`00a89af0`, thiscall: stream; the engine
/// map does not name it).
const NI_STREAM_OBJECT: u32 = 0x00a8_9af0;
/// `bhkRefObject::SethkObject` (`00c87c50`, thiscall: the Havok object), `~bhkRefObject`
/// (`00c87d20`) and `bhkRefObject::GetViewerStrings` (`00c87de0`).
const REF_OBJECT_SET_HK_OBJECT: u32 = 0x00c8_7c50;
const REF_OBJECT_DESTRUCTOR: u32 = 0x00c8_7d20;
const REF_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00c8_7de0;
/// The memory manager (`011f6238`) and its free (`00aa4060`, thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `NiGetViewerString` (`00a7d330`, cdecl: name) and the string array's resize (`0096ad30`)
/// and store (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The class name word (`01268134`) `GetViewerStrings` passes to `NiGetViewerString`.
const CLASS_RTTI: u32 = 0x0126_8134;
/// The clone lock (`0126813c`, a critical section) and the `kernel32` imports that take it.
const CLONE_LOCK_SECTION: u32 = 0x0126_813c;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The vtable `~bhkSerializable` stores.
const SERIALIZABLE_VTABLE: u32 = 0x0102_023c;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

// Virtual slots (byte offsets) of the bhk object classes.
/// `SethkObject` (engine map names the `bhkSerializable` one `00c87760`).
const VSLOT_SET_HK_OBJECT: u32 = 0x8c;
/// Answers the world of the object (unconfirmed name).
const VSLOT_GET_WORLD: u32 = 0x94;
/// Removes the object from its world (unconfirmed name).
const VSLOT_REMOVE: u32 = 0xa0;
/// Unconfirmed name (`bhkSerializable::PostInit`, `00c876d0`, jumps to this slot).
const VSLOT_POST_INIT: u32 = 0xa4;
/// Answers the byte size of the Cinfo (unconfirmed name).
const VSLOT_CINFO_SIZE: u32 = 0xa8;
/// Reads the object's members from the Cinfo (unconfirmed name; `LoadBinary` calls it).
const VSLOT_LOAD_CINFO: u32 = 0xac;
/// Takes the Cinfo (unconfirmed name; `LinkObject` and `CopyMembers` call it).
const VSLOT_SET_CINFO: u32 = 0xb0;
/// `CreateCinfo` (out parameter: the address of a flag byte).
const VSLOT_CREATE_CINFO: u32 = 0xb4;

layout! {
    /// `bhkSerializable` (Xbox PDB, 0x10 bytes): a `bhkRefObject`.
    pub struct BhkSerializable: 0x10 {
        /// `phkObject` (Xbox PDB): the Havok object.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB): the Cinfo.
        0x0C pInfo: Ptr,
    }
}

/// Calls `CreateCinfo` (virtual `+0xb4`, with the address of a scratch flag word) and answers
/// the Cinfo and the flag word (the exe reads a whole dword of which only the flag byte is
/// written; the other bytes are zero here).
fn create_cinfo(e: &mut Engine, this: u32) -> (u32, u32) {
    e.with_stack(4, |e, created| {
        let cinfo = e.vcall(this, VSLOT_CREATE_CINFO, &args![created]).u32();
        (cinfo, e.mem.u32(created.addr()))
    })
}

// Translated from 00c87600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::CopyMembers` (engine map name), `(clone, process)`: the `NiObject` copy
/// (`00a5d450`), then the Cinfo of this object (`CreateCinfo`, virtual `+0xb4`) is given to
/// the clone (virtual `+0xb0` of the clone) and the virtual at `+0xa4` runs on this object
/// with 1.
pub fn bhk_serializable_copy_members(e: &mut Engine, this: Ptr, clone: Ptr, process: Ptr) {
    e.call(NI_COPY_MEMBERS, &args![this, clone, process]);
    let (cinfo, _) = create_cinfo(e, this.addr());
    e.vcall(clone.addr(), VSLOT_SET_CINFO, &args![cinfo]);
    e.vcall(this.addr(), VSLOT_POST_INIT, &args![1u32]);
}

// Translated from 00c87650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::LoadBinary` (engine map name), `(stream)`: the `NiObject` load
/// (`00a5d610`), then the virtual at `+0xac` with the stream.
pub fn bhk_serializable_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_LOAD_BINARY, &args![this, stream]);
    e.vcall(this.addr(), VSLOT_LOAD_CINFO, &args![stream]);
}

// Translated from 00c87670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::LinkObject` (engine map name), `(stream)`: runs `00a89af0` with the
/// stream, then, if the Cinfo (`+0xc`) is set, the virtual at `+0xb0` with it.
pub fn bhk_serializable_link_object(e: &mut Engine, this: Ptr<BhkSerializable>, stream: Ptr) {
    e.call(NI_STREAM_OBJECT, &args![this, stream]);
    let cinfo = e.get(this, BhkSerializable::pInfo).addr();
    if cinfo != 0 {
        e.vcall(this.addr(), VSLOT_SET_CINFO, &args![cinfo]);
    }
}

// Translated from 00c876a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::Kill` (engine map name): if the Havok object (`+8`) is set, sets it to 0
/// (virtual `+0x8c`); then runs the virtual at `+0xa4` with 1.
pub fn bhk_serializable_kill(e: &mut Engine, this: Ptr<BhkSerializable>) {
    if e.get(this, BhkSerializable::phkObject).addr() != 0 {
        e.vcall(this.addr(), VSLOT_SET_HK_OBJECT, &args![0u32]);
    }
    e.vcall(this.addr(), VSLOT_POST_INIT, &args![1u32]);
}

// Translated from 00c876d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::PostInit` (engine map name): jumps to the virtual at `+0xa4` with the
/// stack word it was given replaced by 1.
pub fn bhk_serializable_post_init(e: &mut Engine, this: Ptr, _unused_1: u32) {
    e.vcall(this.addr(), VSLOT_POST_INIT, &args![1u32]);
}

// Translated from 00c876f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::CloneCritLock` (engine map name): `EnterCriticalSection` on the clone
/// lock (`0126813c`).
pub fn bhk_serializable_clone_crit_lock(e: &mut Engine) {
    e.call(ENTER_CRITICAL_SECTION, &args![CLONE_LOCK_SECTION]);
}

// Translated from 00c87700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::CloneCritUnlock` (engine map name): `LeaveCriticalSection` on the clone
/// lock (`0126813c`).
pub fn bhk_serializable_clone_crit_unlock(e: &mut Engine) {
    e.call(LEAVE_CRITICAL_SECTION, &args![CLONE_LOCK_SECTION]);
}

// Translated from 00c87710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::LoadCInfo` (engine map name), `(stream)`: takes the Cinfo (`CreateCinfo`)
/// and its size (virtual `+0xa8`), has the stream's file object (`stream + 0x24c`) read that
/// many bytes into it (the function at `+8` of the file object, with the file, the Cinfo, the
/// size, 0 and 0), adds the count it answers to the file's total (`+4`) and answers the size.
pub fn bhk_serializable_load_c_info(e: &mut Engine, this: Ptr, stream: Ptr) -> u32 {
    let (cinfo, _) = create_cinfo(e, this.addr());
    let size = e.vcall(this.addr(), VSLOT_CINFO_SIZE, &args![]).u32();
    let file = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(file + 8);
    let count = e.call(read, &args![file, cinfo, size, 0u32, 0u32]).u32();
    let total = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, total.wrapping_add(count));
    size
}

// Translated from 00c87760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::SethkObject` (engine map name), `(hk)`: does nothing if `hk` is already
/// the Havok object (`+8`; 0 for a null `this`). Otherwise, if there is a current one and its
/// world (virtual `+0x94`) is set and has the word at `+0x350` set, removes the object
/// (virtual `+0xa0`); then runs `bhkRefObject::SethkObject` (`00c87c50`).
pub fn bhk_serializable_seth_k_object(e: &mut Engine, this: Ptr<BhkSerializable>, hk: u32) {
    let current = if this.addr() != 0 {
        e.get(this, BhkSerializable::phkObject).addr()
    } else {
        0
    };
    if hk == current {
        return;
    }
    if current != 0 {
        let world = e.vcall(this.addr(), VSLOT_GET_WORLD, &args![]).u32();
        if world != 0 && e.mem.u32(world + 0x350) != 0 {
            e.vcall(this.addr(), VSLOT_REMOVE, &args![]);
        }
    }
    e.call(REF_OBJECT_SET_HK_OBJECT, &args![this, hk]);
}

// Translated from 00c877b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c877b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSerializable::~bhkSerializable` (engine map name): stores the vtable, and inside the
/// `TLS + 0x2b4 = 0x10` scope frees the Cinfo (`+0xc`, without a null check) through the
/// memory manager (`00aa4060`) and clears it; then runs `~bhkRefObject` (`00c87d20`).
pub fn bhk_serializable_destructor(e: &mut Engine, this: Ptr<BhkSerializable>) {
    e.mem.set_u32(this.addr(), SERIALIZABLE_VTABLE);
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    let cinfo = e.get(this, BhkSerializable::pInfo).addr();
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, cinfo]);
    e.set(this, BhkSerializable::pInfo, Ptr::new(0));
    e.mem.set_u32(thread + SCOPE_WORD, saved);
    e.call(REF_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 00c87850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSerializable::SaveBinary` (engine map name), `(stream)`: takes the Cinfo
/// (`CreateCinfo`), runs `00a89af0` with the stream, takes the Cinfo size (virtual `+0xa8`)
/// and has the stream's file object (`stream + 0x250`) write it (the function at `+0xc` of
/// the file object, with the file, the Cinfo, the size, the address of a word holding 1, and
/// 1), adds the count it answers to the file's total (`+4`); then runs the virtual at `+0xa4`
/// with the flag word `CreateCinfo` left.
pub fn bhk_serializable_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (cinfo, flag) = create_cinfo(e, this.addr());
    e.call(NI_STREAM_OBJECT, &args![this, stream]);
    let size = e.vcall(this.addr(), VSLOT_CINFO_SIZE, &args![]).u32();
    let file = e.mem.u32(stream.addr() + 0x250);
    let write = e.mem.u32(file + 0xc);
    let count = e.with_stack(4, |e, one| {
        e.mem.set_u32(one.addr(), 1);
        e.call(write, &args![file, cinfo, size, one, 1u32]).u32()
    });
    let total = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, total.wrapping_add(count));
    e.vcall(this.addr(), VSLOT_POST_INIT, &args![flag]);
}

// Translated from 00c878c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c878c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSerializable::GetViewerStrings` (engine map name), `(strings)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, the base strings (`00c87de0`) and the text
/// `NiGetViewerString` makes of the word at `01268134`, stored in the string array (grown by
/// `0096ad30` when its size, `+0xa`, has reached its capacity, `+8`; stored by `0096ae90`).
pub fn bhk_serializable_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    e.call(REF_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(CLASS_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    let array = strings.addr();
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
    e.mem.set_u32(thread + SCOPE_WORD, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c87600, bhk_serializable_copy_members(Ptr, Ptr, Ptr)),
        entry!(0x00c87650, bhk_serializable_load_binary(Ptr, Ptr)),
        entry!(
            0x00c87670,
            bhk_serializable_link_object(Ptr<BhkSerializable>, Ptr)
        ),
        entry!(0x00c876a0, bhk_serializable_kill(Ptr<BhkSerializable>)),
        entry!(0x00c876d0, bhk_serializable_post_init(Ptr, u32)),
        entry!(0x00c876f0, bhk_serializable_clone_crit_lock()),
        entry!(0x00c87700, bhk_serializable_clone_crit_unlock()),
        entry!(0x00c87710, bhk_serializable_load_c_info(Ptr, Ptr) -> u32),
        entry!(
            0x00c87760,
            bhk_serializable_seth_k_object(Ptr<BhkSerializable>, u32)
        ),
        entry!(
            0x00c877b0,
            bhk_serializable_destructor(Ptr<BhkSerializable>)
        ),
        entry!(0x00c87850, bhk_serializable_save_binary(Ptr, Ptr)),
        entry!(0x00c878c0, bhk_serializable_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SET_HK_OBJECT_SLOT: u32 = 0x7000_008c;
    const GET_WORLD_SLOT: u32 = 0x7000_0094;
    const REMOVE_SLOT: u32 = 0x7000_00a0;
    const POST_INIT_SLOT: u32 = 0x7000_00a4;
    const CINFO_SIZE_SLOT: u32 = 0x7000_00a8;
    const LOAD_CINFO_SLOT: u32 = 0x7000_00ac;
    const SET_CINFO_SLOT: u32 = 0x7000_00b0;
    const CREATE_CINFO_SLOT: u32 = 0x7000_00b4;
    const OBJECT_VTABLE: u32 = 0x7001_1000;
    const READ_FILE: u32 = 0x7000_1000;
    const WRITE_FILE: u32 = 0x7000_1004;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    /// An object whose vtable has every virtual slot the unit calls.
    fn object(e: &mut Engine) -> u32 {
        e.put_vtable(
            OBJECT_VTABLE,
            &(0..0x30)
                .map(|slot| match slot * 4 {
                    0x8c => SET_HK_OBJECT_SLOT,
                    0x94 => GET_WORLD_SLOT,
                    0xa0 => REMOVE_SLOT,
                    0xa4 => POST_INIT_SLOT,
                    0xa8 => CINFO_SIZE_SLOT,
                    0xac => LOAD_CINFO_SLOT,
                    0xb0 => SET_CINFO_SLOT,
                    0xb4 => CREATE_CINFO_SLOT,
                    _ => 0,
                })
                .collect::<Vec<u32>>(),
        );
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, OBJECT_VTABLE);
        this
    }

    /// Doubles for every virtual slot: `CreateCinfo` answers `cinfo` and sets the flag byte
    /// it is given the address of to 1, the size is `size`.
    fn virtual_doubles(e: &mut Engine, cinfo: u32, size: u32) {
        e.register_double(CREATE_CINFO_SLOT, move |e, a| {
            e.mem.set_u8(a[1], 1);
            cinfo.into_ret()
        });
        e.register_double(CINFO_SIZE_SLOT, move |_, _| size.into_ret());
        for slot in [
            SET_HK_OBJECT_SLOT,
            REMOVE_SLOT,
            POST_INIT_SLOT,
            LOAD_CINFO_SLOT,
            SET_CINFO_SLOT,
        ] {
            e.register(slot, |_, _| Ret::default());
        }
    }

    #[test]
    fn bhk_serializable_copy_members_hands_the_cinfo_to_the_clone() {
        let mut e = logged_engine();
        e.register(NI_COPY_MEMBERS, |_, _| Ret::default());
        virtual_doubles(&mut e, 0x5000, 0);
        let this = object(&mut e);
        let clone = object(&mut e);
        e.call(0x00c8_7600, &args![this, clone, 0x9999u32]);
        assert_eq!(
            calls_to(&e, NI_COPY_MEMBERS),
            vec![vec![this, clone, 0x9999]]
        );
        assert_eq!(calls_to(&e, CREATE_CINFO_SLOT).len(), 1);
        assert_eq!(calls_to(&e, SET_CINFO_SLOT), vec![vec![clone, 0x5000]]);
        assert_eq!(calls_to(&e, POST_INIT_SLOT), vec![vec![this, 1]]);
        assert_eq!(
            order_of(
                &e,
                &[
                    NI_COPY_MEMBERS,
                    CREATE_CINFO_SLOT,
                    SET_CINFO_SLOT,
                    POST_INIT_SLOT
                ]
            ),
            vec![
                NI_COPY_MEMBERS,
                CREATE_CINFO_SLOT,
                SET_CINFO_SLOT,
                POST_INIT_SLOT
            ]
        );
    }

    #[test]
    fn bhk_serializable_load_binary_loads_the_base_then_the_cinfo() {
        let mut e = logged_engine();
        e.register(NI_LOAD_BINARY, |_, _| Ret::default());
        virtual_doubles(&mut e, 0, 0);
        let this = object(&mut e);
        e.call(0x00c8_7650, &args![this, 0x7777u32]);
        assert_eq!(calls_to(&e, NI_LOAD_BINARY), vec![vec![this, 0x7777]]);
        assert_eq!(calls_to(&e, LOAD_CINFO_SLOT), vec![vec![this, 0x7777]]);
        assert_eq!(
            order_of(&e, &[NI_LOAD_BINARY, LOAD_CINFO_SLOT]),
            vec![NI_LOAD_BINARY, LOAD_CINFO_SLOT]
        );
    }

    #[test]
    fn bhk_serializable_link_object_sets_the_cinfo_only_when_there_is_one() {
        let mut e = logged_engine();
        e.register(NI_STREAM_OBJECT, |_, _| Ret::default());
        virtual_doubles(&mut e, 0, 0);
        let this = object(&mut e);
        e.call(0x00c8_7670, &args![this, 0x7777u32]);
        assert_eq!(calls_to(&e, NI_STREAM_OBJECT), vec![vec![this, 0x7777]]);
        assert!(calls_to(&e, SET_CINFO_SLOT).is_empty());
        e.mem.set_u32(this + 0xc, 0x6000);
        e.call(0x00c8_7670, &args![this, 0x7778u32]);
        assert_eq!(calls_to(&e, SET_CINFO_SLOT), vec![vec![this, 0x6000]]);
        assert_eq!(calls_to(&e, NI_STREAM_OBJECT).len(), 2);
    }

    #[test]
    fn bhk_serializable_kill_clears_the_havok_object_only_when_set() {
        let mut e = logged_engine();
        virtual_doubles(&mut e, 0, 0);
        let this = object(&mut e);
        e.call(0x00c8_76a0, &args![this]);
        assert!(calls_to(&e, SET_HK_OBJECT_SLOT).is_empty());
        assert_eq!(calls_to(&e, POST_INIT_SLOT), vec![vec![this, 1]]);
        e.mem.set_u32(this + 8, 0x4000);
        e.call(0x00c8_76a0, &args![this]);
        assert_eq!(calls_to(&e, SET_HK_OBJECT_SLOT), vec![vec![this, 0]]);
        assert_eq!(calls_to(&e, POST_INIT_SLOT).len(), 2);
        assert_eq!(
            order_of(&e, &[SET_HK_OBJECT_SLOT, POST_INIT_SLOT])[1..],
            [SET_HK_OBJECT_SLOT, POST_INIT_SLOT]
        );
    }

    #[test]
    fn bhk_serializable_post_init_runs_the_virtual_with_one() {
        let mut e = logged_engine();
        virtual_doubles(&mut e, 0, 0);
        let this = object(&mut e);
        e.call(0x00c8_76d0, &args![this, 0xdead_beefu32]);
        assert_eq!(calls_to(&e, POST_INIT_SLOT), vec![vec![this, 1]]);
    }

    #[test]
    fn bhk_serializable_clone_lock_enters_and_leaves_the_critical_section() {
        let mut e = logged_engine();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.call(0x00c8_76f0, &args![]);
        e.call(0x00c8_7700, &args![]);
        assert_eq!(
            calls_to(&e, ENTER_CRITICAL_SECTION),
            vec![vec![CLONE_LOCK_SECTION]]
        );
        assert_eq!(
            calls_to(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![CLONE_LOCK_SECTION]]
        );
    }

    #[test]
    fn bhk_serializable_load_c_info_reads_the_cinfo_from_the_file() {
        let mut e = logged_engine();
        virtual_doubles(&mut e, 0x5000, 0x30);
        e.register(READ_FILE, |_, _| 0x28u32.into_ret());
        let this = object(&mut e);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 4, 100);
        e.mem.set_u32(file + 8, READ_FILE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x24c, file);
        let size = e.call(0x00c8_7710, &args![this, stream]).u32();
        assert_eq!(size, 0x30);
        assert_eq!(
            calls_to(&e, READ_FILE),
            vec![vec![file, 0x5000, 0x30, 0, 0]]
        );
        assert_eq!(e.mem.u32(file + 4), 0x28 + 100);
    }

    #[test]
    fn bhk_serializable_seth_k_object_returns_early_for_the_current_object() {
        let mut e = logged_engine();
        e.register(REF_OBJECT_SET_HK_OBJECT, |_, _| Ret::default());
        virtual_doubles(&mut e, 0, 0);
        let this = object(&mut e);
        e.mem.set_u32(this + 8, 0x4000);
        e.call(0x00c8_7760, &args![this, 0x4000u32]);
        assert!(calls_to(&e, REF_OBJECT_SET_HK_OBJECT).is_empty());
        assert!(calls_to(&e, GET_WORLD_SLOT).is_empty());
    }

    #[test]
    fn bhk_serializable_seth_k_object_without_a_current_object_just_sets() {
        let mut e = logged_engine();
        e.register(REF_OBJECT_SET_HK_OBJECT, |_, _| Ret::default());
        virtual_doubles(&mut e, 0, 0);
        let this = object(&mut e);
        e.call(0x00c8_7760, &args![this, 0x4000u32]);
        assert_eq!(
            calls_to(&e, REF_OBJECT_SET_HK_OBJECT),
            vec![vec![this, 0x4000]]
        );
        assert!(calls_to(&e, GET_WORLD_SLOT).is_empty());
    }

    #[test]
    fn bhk_serializable_seth_k_object_removes_from_a_world_that_has_the_flag_set() {
        for (world_flag, removed) in [(0u32, false), (1, true)] {
            let mut e = logged_engine();
            e.register(REF_OBJECT_SET_HK_OBJECT, |_, _| Ret::default());
            virtual_doubles(&mut e, 0, 0);
            let world = e.mem.alloc(0x360);
            e.mem.set_u32(world + 0x350, world_flag);
            e.register_double(GET_WORLD_SLOT, move |_, _| world.into_ret());
            let this = object(&mut e);
            e.mem.set_u32(this + 8, 0x4000);
            e.call(0x00c8_7760, &args![this, 0x5000u32]);
            assert_eq!(calls_to(&e, REMOVE_SLOT).len(), usize::from(removed));
            assert_eq!(
                calls_to(&e, REF_OBJECT_SET_HK_OBJECT),
                vec![vec![this, 0x5000]]
            );
        }
    }

    #[test]
    fn bhk_serializable_seth_k_object_skips_the_removal_without_a_world() {
        let mut e = logged_engine();
        e.register(REF_OBJECT_SET_HK_OBJECT, |_, _| Ret::default());
        virtual_doubles(&mut e, 0, 0);
        e.register(GET_WORLD_SLOT, |_, _| Ret::default());
        let this = object(&mut e);
        e.mem.set_u32(this + 8, 0x4000);
        e.call(0x00c8_7760, &args![this, 0u32]);
        assert!(calls_to(&e, REMOVE_SLOT).is_empty());
        assert_eq!(calls_to(&e, REF_OBJECT_SET_HK_OBJECT), vec![vec![this, 0]]);
    }

    #[test]
    fn bhk_serializable_destructor_frees_the_cinfo_in_the_scope_then_the_base() {
        let mut e = logged_engine();
        e.register(MEMORY_FREE, |_, _| Ret::default());
        e.register(REF_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x5000);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00c8_77b0, &args![this]);
        assert_eq!(e.mem.u32(this), SERIALIZABLE_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(
            calls_to(&e, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, 0x5000]]
        );
        assert_eq!(calls_to(&e, REF_OBJECT_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_serializable_destructor_frees_a_null_cinfo_too() {
        let mut e = logged_engine();
        e.register(MEMORY_FREE, |_, _| Ret::default());
        e.register(REF_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call(0x00c8_77b0, &args![this]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, 0]]);
    }

    #[test]
    fn bhk_serializable_save_binary_writes_the_cinfo_to_the_file() {
        let mut e = logged_engine();
        e.register(NI_STREAM_OBJECT, |_, _| Ret::default());
        virtual_doubles(&mut e, 0x5000, 0x30);
        e.register_double(WRITE_FILE, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 1);
            0x30u32.into_ret()
        });
        let this = object(&mut e);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 4, 8);
        e.mem.set_u32(file + 0xc, WRITE_FILE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + 0x250, file);
        e.call(0x00c8_7850, &args![this, stream]);
        assert_eq!(calls_to(&e, NI_STREAM_OBJECT), vec![vec![this, stream]]);
        let writes = calls_to(&e, WRITE_FILE);
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0][..3], [file, 0x5000, 0x30]);
        assert_eq!(writes[0][4], 1);
        assert_eq!(e.mem.u32(file + 4), 8 + 0x30);
        assert_eq!(calls_to(&e, POST_INIT_SLOT), vec![vec![this, 1]]);
        assert_eq!(
            order_of(
                &e,
                &[
                    CREATE_CINFO_SLOT,
                    NI_STREAM_OBJECT,
                    CINFO_SIZE_SLOT,
                    WRITE_FILE,
                    POST_INIT_SLOT
                ]
            ),
            vec![
                CREATE_CINFO_SLOT,
                NI_STREAM_OBJECT,
                CINFO_SIZE_SLOT,
                WRITE_FILE,
                POST_INIT_SLOT
            ]
        );
    }

    #[test]
    fn bhk_serializable_get_viewer_strings_stores_the_class_name_text() {
        for (capacity, grows) in [(0u16, true), (8, false)] {
            let mut e = logged_engine();
            e.register(REF_OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
            e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
            e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
            e.register(STRING_ARRAY_SET_AT, |e, a| {
                let base = e.mem.u32(a[0] + 4);
                let value = e.mem.u32(a[2]);
                e.mem.set_u32(base + a[1] * 4, value);
                Ret::default()
            });
            e.mem.set_u32(CLASS_RTTI, 0x4242);
            let this = e.mem.alloc(0x10);
            let array = e.mem.alloc(0x10);
            let base = e.mem.alloc(0x40);
            e.mem.set_u32(array + 4, base);
            e.mem.set_u16(array + 8, capacity);
            e.mem.set_u16(array + 0xa, 0);
            e.mem.set_u16(array + 0xe, 4);
            let thread = e.tls();
            e.mem.set_u32(thread + SCOPE_WORD, 0x77);
            e.call(0x00c8_78c0, &args![this, array]);
            assert_eq!(
                calls_to(&e, REF_OBJECT_GET_VIEWER_STRINGS),
                vec![vec![this, array]]
            );
            assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4242]]);
            let resized = calls_to(&e, STRING_ARRAY_SET_SIZE);
            if grows {
                assert_eq!(resized, vec![vec![array, 4]]);
            } else {
                assert!(resized.is_empty());
            }
            let stored = calls_to(&e, STRING_ARRAY_SET_AT);
            assert_eq!(stored.len(), 1);
            assert_eq!(stored[0][..2], [array, 0]);
            assert_eq!(e.mem.u32(base), 0x1111);
            assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
        }
    }
}
