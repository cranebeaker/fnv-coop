//! `BSMain/bsreference.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a5d2f0` (named `NiObject::RegisterStreamables` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00A5D2F0: u32 = 0x00a5_d2f0;

// Translated from 00c64b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNodeReferences::RegisterStreamables` (engine map vtable name, vtable word 010c1448):
/// forwards `this` and the stream to `00a5d2f0` (`NiObject::RegisterStreamables`, which
/// tail-jumps through slot 0x24 of the argument's vtable). Returns the `bool` in `AL`: Xbox
/// PDB `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z` returns `_N`; `stream` is the
/// `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_node_references_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00A5D2F0, &args![this, stream])
        .bool()
}

/// The global memory manager (`011f6238`), its `Allocate` (`00aa3e40`, `thiscall`,
/// `(size)`) and `Deallocate` (`00aa4060`, `thiscall`, `(block)`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `strlen`, `strcpy_s` and `_stricmp` (CRT, `cdecl`).
const STRLEN: u32 = 0x00ec_6130;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRICMP: u32 = 0x00ec_68e4;
/// `00aa13e0` (`cdecl`, one word): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiObject`'s constructor (`00a5d3a0`, `thiscall`), `NiObject::LoadBinary` (`00a5d610`),
/// `NiObject::IsEqual` (`00a5d470`) and `00a89af0` (`thiscall`, `(stream)`, the base call
/// of the `SaveBinary` and `LinkObject` bodies here).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_BASE_STREAM_CALL: u32 = 0x00a8_9af0;
/// `NiStream::LoadCString` (`00a64ce0`, `thiscall` on the stream, `(&string)`), the C-string
/// save `00a64160` (`thiscall` on the stream, `(string)`), `NiStream::ReadMultipleLinkIDs`
/// (`00a64670`), `NiStream::GetNumberOfLinkIDs` (`00a63460`) and
/// `NiStream::GetObjectFromLinkID` (`00a63430`).
const NI_STREAM_LOAD_C_STRING: u32 = 0x00a6_4ce0;
const NI_STREAM_SAVE_C_STRING: u32 = 0x00a6_4160;
const NI_STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const NI_STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const NI_STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiStreamSaveBinary<unsigned int>` (`0044adf0`, `cdecl`, `(stream, &word)`).
const STREAM_SAVE_WORD: u32 = 0x0044_adf0;
/// `BSStream`'s reference map (`+0x5C4`) and its lookup (`00853130`, `thiscall`, `(name,
/// &found)`).
const REFERENCE_MAP_OFFSET: u32 = 0x5c4;
const REFERENCE_MAP_LOOKUP: u32 = 0x0085_3130;
/// `BSReference` (0xC bytes, vtable `010c1364`; its name, a memory-manager string, at +8).
const BS_REFERENCE_VTABLE: u32 = 0x010c_1364;
const BS_REFERENCE_SIZE: u32 = 0xc;
const NAME: u32 = 0x8;
/// `BSNodeReferences` (0x18 bytes, vtable `010c13f4`): an `NiTArray` at +8 (its vtable
/// `01069d84`; made by `0068ac40`, `thiscall`, `(max, grow_by)`; resized by `00c4c920`,
/// `thiscall`, `(count)`).
const BS_NODE_REFERENCES_VTABLE: u32 = 0x010c_13f4;
const BS_NODE_REFERENCES_SIZE: u32 = 0x18;
const NODE_ARRAY: u32 = 0x8;
const NODE_ARRAY_VTABLE: u32 = 0x0106_9d84;
const NI_T_ARRAY_CONSTRUCT: u32 = 0x0068_ac40;
const NODE_ARRAY_SET_SIZE: u32 = 0x00c4_c920;

// Translated from 00c64780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSReference::SetName` (Xbox PDB, engine map; `thiscall`, `(name)`): frees the old name
/// (`MemoryManager::Deallocate`), clears it, and with a `name` stores a copy in a new
/// memory-manager block of its length plus one (copied with `strcpy_s` when the block was
/// given).
pub fn bs_reference_set_name(e: &mut Engine, this: Ptr, name: u32) {
    let old = e.mem.u32(this.addr().wrapping_add(NAME));
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, old]);
    e.mem.set_u32(this.addr().wrapping_add(NAME), 0);
    if name == 0 {
        return;
    }
    let size = e.call(STRLEN, &args![name]).u32().wrapping_add(1);
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    e.mem.set_u32(this.addr().wrapping_add(NAME), block);
    if e.mem.u32(this.addr().wrapping_add(NAME)) != 0 {
        let copy = e.mem.u32(this.addr().wrapping_add(NAME));
        e.call(STRCPY_S, &args![copy, size, name]);
    }
}

// Translated from 00c64810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c64810 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f3054f, state variable) is not translated
/// Unnamed in the engine map (the `BSReference` loader `BSStream::InitLoaders` registers, no
/// arguments): allocates 0xC bytes and, when that succeeds, constructs an `NiObject` in it
/// with `BSReference`'s vtable and no name. Returns it, or 0.
pub fn fn_00c64810(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![BS_REFERENCE_SIZE]).u32();
    if block == 0 {
        return 0;
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, BS_REFERENCE_VTABLE);
    e.mem.set_u32(block.wrapping_add(NAME), 0);
    block
}

// Translated from 00c648a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSReference::LoadBinary` (Xbox PDB, engine map; vtable): `NiObject::LoadBinary`, the
/// name read into +8 (`NiStream::LoadCString`), and, when the stream (a `BSStream`) has a
/// reference map (+0x5C4) holding the name (`00853130`), the object found is given to the
/// stream's vtable slot 0x28.
pub fn bs_reference_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(
        NI_STREAM_LOAD_C_STRING,
        &args![stream, this.addr().wrapping_add(NAME)],
    );
    let name = e.mem.u32(this.addr().wrapping_add(NAME));
    let found = e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        let map = e.mem.u32(stream.wrapping_add(REFERENCE_MAP_OFFSET));
        if map != 0 && name != 0 {
            e.call(REFERENCE_MAP_LOOKUP, &args![map, name, found]);
        }
        e.mem.u32(found.addr())
    });
    if found != 0 {
        e.vcall(stream, 0x28, &args![found]);
    }
}

// Translated from 00c64930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSReference`'s vtable word `010c13b4` (slot 0x50, its
/// `LinkObject` place): takes `this` and one stack word (`RET 4`) and does nothing.
pub fn fn_00c64930(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00c64940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSReference`'s vtable word `010c13b8` (slot 0x54, its
/// `RegisterStreamables` place): takes `this` and one stack word (`RET 4`) and answers true
/// in `AL`.
pub fn fn_00c64940(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    true
}

// Translated from 00c64950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSReference::SaveBinary` (Xbox PDB, engine map; vtable): `00a89af0` with the stream,
/// then the name saved as a C string (`00a64160` on the stream).
pub fn bs_reference_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(OBJECT_BASE_STREAM_CALL, &args![this, stream]);
    let name = e.mem.u32(this.addr().wrapping_add(NAME));
    e.call(NI_STREAM_SAVE_C_STRING, &args![stream, name]);
}

// Translated from 00c64980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSReference::IsEqual` (Xbox PDB, engine map; vtable): true when `NiObject::IsEqual`
/// holds, both names are set and they compare equal ignoring case (`_stricmp`); false
/// otherwise.
pub fn bs_reference_is_equal(e: &mut Engine, this: Ptr, other: u32) -> bool {
    if e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).u8() == 0 {
        return false;
    }
    let mine = e.mem.u32(this.addr().wrapping_add(NAME));
    if mine == 0 {
        return false;
    }
    let theirs = e.mem.u32(other.wrapping_add(NAME));
    if theirs == 0 {
        return false;
    }
    e.call(STRICMP, &args![mine, theirs]).u32() == 0
}

// Translated from 00c649f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c649f0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30581, state variable) is not translated
/// Unnamed in the engine map (the `BSNodeReferences` loader `BSStream::InitLoaders`
/// registers, no arguments): allocates 0x18 bytes and, when that succeeds, constructs an
/// `NiObject` in it with `BSNodeReferences`' vtable and its array (+8, `0068ac40` with 0
/// and 1, then the array's vtable). Returns it, or 0.
pub fn fn_00c649f0(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![BS_NODE_REFERENCES_SIZE]).u32();
    if block == 0 {
        return 0;
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, BS_NODE_REFERENCES_VTABLE);
    let array = block.wrapping_add(NODE_ARRAY);
    e.call(NI_T_ARRAY_CONSTRUCT, &args![array, 0u32, 1u32]);
    e.mem.set_u32(array, NODE_ARRAY_VTABLE);
    block
}

// Translated from 00c64aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNodeReferences::LoadBinary` (Xbox PDB, engine map; vtable): `NiObject::LoadBinary`,
/// then `NiStream::ReadMultipleLinkIDs`.
pub fn bs_node_references_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(NI_STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
}

// Translated from 00c64ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNodeReferences::LinkObject` (Xbox PDB, engine map; vtable): `00a89af0` with the
/// stream; with link ids, the array is sized to their number (`00c4c920`) and each of the
/// number / 2 pairs of linked objects read (`NiStream::GetObjectFromLinkID` twice) has, when
/// both are set, the first one's vtable slot 0xDC called with the second and 0.
pub fn bs_node_references_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(OBJECT_BASE_STREAM_CALL, &args![this, stream]);
    let count = e
        .call(NI_STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream])
        .u32();
    if count == 0 {
        return;
    }
    e.call(
        NODE_ARRAY_SET_SIZE,
        &args![this.addr().wrapping_add(NODE_ARRAY), count],
    );
    let pairs = count >> 1;
    let mut index = 0u32;
    while index < pairs {
        let parent = e
            .call(NI_STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream])
            .u32();
        let child = e
            .call(NI_STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream])
            .u32();
        if parent != 0 && child != 0 {
            e.vcall(parent, 0xdc, &args![child, 0u32]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00c64b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNodeReferences::SaveBinary` (Xbox PDB, engine map; vtable): `00a89af0` with the
/// stream, the array's element count (+0x14) times two saved as a word
/// (`NiStreamSaveBinary<unsigned int>`), then for each of the first that many slots (+0xC)
/// that is set, the link of the entry's word at +0x18 and of the entry itself (the
/// stream's vtable slot 0x30 each).
pub fn bs_node_references_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(OBJECT_BASE_STREAM_CALL, &args![this, stream]);
    let count = u32::from(e.mem.u16(this.addr().wrapping_add(0x14)));
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), count << 1);
        e.call(STREAM_SAVE_WORD, &args![stream, word]);
    });
    let mut index = 0u32;
    while index < count {
        let base = e.mem.u32(this.addr().wrapping_add(0xc));
        let entry = e.mem.u32(base.wrapping_add(index.wrapping_mul(4)));
        if entry != 0 {
            let linked = e.mem.u32(entry.wrapping_add(0x18));
            e.vcall(stream, 0x30, &args![linked]);
            e.vcall(stream, 0x30, &args![entry]);
        }
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_4b70, bs_node_references_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c6_4780, bs_reference_set_name(Ptr, u32)),
        entry!(0x00c6_4810, fn_00c64810() -> u32),
        entry!(0x00c6_48a0, bs_reference_load_binary(Ptr, u32)),
        entry!(0x00c6_4930, fn_00c64930(Ptr, u32)),
        entry!(0x00c6_4940, fn_00c64940(Ptr, u32) -> bool),
        entry!(0x00c6_4950, bs_reference_save_binary(Ptr, u32)),
        entry!(0x00c6_4980, bs_reference_is_equal(Ptr, u32) -> bool),
        entry!(0x00c6_49f0, fn_00c649f0() -> u32),
        entry!(0x00c6_4aa0, bs_node_references_load_binary(Ptr, u32)),
        entry!(0x00c6_4ad0, bs_node_references_link_object(Ptr, u32)),
        entry!(0x00c6_4b90, bs_node_references_save_binary(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_node_references_register_streamables_forwards_this_and_the_argument_and_returns_the_byte(
    ) {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00A5D2F0, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c6_4b70, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00A5D2F0)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod n068_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn set_name_frees_the_old_name_and_copies_the_new_one() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, MEMORY_MANAGER_DEALLOCATE, 0);
        let block = e.mem.alloc(0x10);
        logged(&mut e, &log, MEMORY_MANAGER_ALLOCATE, block);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + NAME, 0x7777);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"Bip01");
        e.call(0x00c6_4780, &args![this, name]);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x7777]]
        );
        assert_eq!(
            calls(&log, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 6]]
        );
        assert_eq!(e.mem.u32(this + NAME), block);
        assert_eq!(e.mem.cstr(block), b"Bip01");
        // No name: only cleared. A failed allocation leaves it null.
        e.call(0x00c6_4780, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + NAME), 0);
        assert_eq!(calls(&log, MEMORY_MANAGER_ALLOCATE).len(), 1);
        logged(&mut e, &log, MEMORY_MANAGER_ALLOCATE, 0);
        e.call(0x00c6_4780, &args![this, name]);
        assert_eq!(e.mem.u32(this + NAME), 0);
    }

    #[test]
    fn loaders_construct_a_reference_and_node_references() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_OBJECT_CONSTRUCT, 0);
        logged(&mut e, &log, NI_T_ARRAY_CONSTRUCT, 0);
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00c6_4810, &args![]).u32(), 0);
        assert_eq!(e.call(0x00c6_49f0, &args![]).u32(), 0);
        assert!(calls(&log, NI_OBJECT_CONSTRUCT).is_empty());
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xcc; 0x18]);
        logged(&mut e, &log, NI_ALLOCATE, block);
        assert_eq!(e.call(0x00c6_4810, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), BS_REFERENCE_VTABLE);
        assert_eq!(e.mem.u32(block + NAME), 0);
        assert_eq!(e.call(0x00c6_49f0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), BS_NODE_REFERENCES_VTABLE);
        assert_eq!(e.mem.u32(block + 8), NODE_ARRAY_VTABLE);
        assert_eq!(
            calls(&log, NI_ALLOCATE),
            vec![vec![0xc], vec![0x18], vec![0xc], vec![0x18]]
        );
        assert_eq!(
            calls(&log, NI_OBJECT_CONSTRUCT),
            vec![vec![block], vec![block]]
        );
        assert_eq!(
            calls(&log, NI_T_ARRAY_CONSTRUCT),
            vec![vec![block + 8, 0, 1]]
        );
    }

    #[test]
    fn load_binary_resolves_the_name_through_the_reference_map() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_OBJECT_LOAD_BINARY, 0);
        e.register(NI_STREAM_LOAD_C_STRING, |e, a| {
            e.mem.set_u32(a[1], 0x4242);
            Ret::default()
        });
        e.register(REFERENCE_MAP_LOOKUP, |e, a| {
            assert_eq!(a[1], 0x4242);
            e.mem.set_u32(a[2], 0x5151);
            1u32.into_ret()
        });
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(vtable + 0x28, 0x0040_6200);
        logged(&mut e, &log, 0x0040_6200, 0);
        let stream = e.mem.alloc(0x600);
        e.mem.set_u32(stream, vtable);
        let this = e.mem.alloc(0xc);
        e.call(0x00c6_48a0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + NAME), 0x4242);
        assert!(calls(&log, 0x0040_6200).is_empty());
        e.mem.set_u32(stream + REFERENCE_MAP_OFFSET, 0x9000);
        e.call(0x00c6_48a0, &args![this, stream]);
        assert_eq!(calls(&log, 0x0040_6200), vec![vec![stream, 0x5151]]);
        assert_eq!(
            calls(&log, NI_OBJECT_LOAD_BINARY),
            vec![vec![this, stream], vec![this, stream]]
        );
    }

    #[test]
    fn link_and_streamables_slots_are_constant() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00c6_4930, &args![0x1000u32, 0x2000u32]);
        assert!(e.call(0x00c6_4940, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn save_binary_saves_the_name() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, OBJECT_BASE_STREAM_CALL, 0);
        logged(&mut e, &log, NI_STREAM_SAVE_C_STRING, 0);
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + NAME, 0x3333);
        e.call(0x00c6_4950, &args![this, 0x6000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (OBJECT_BASE_STREAM_CALL, vec![this, 0x6000]),
                (NI_STREAM_SAVE_C_STRING, vec![0x6000, 0x3333]),
            ]
        );
    }

    #[test]
    fn is_equal_compares_names_ignoring_case() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_OBJECT_IS_EQUAL, 1);
        let this = e.mem.alloc(0xc);
        let other = e.mem.alloc(0xc);
        let a = e.mem.alloc(8);
        e.mem.set_cstr(a, b"Name");
        let b = e.mem.alloc(8);
        e.mem.set_cstr(b, b"nAME");
        assert!(!e.call(0x00c6_4980, &args![this, other]).bool());
        e.mem.set_u32(this + NAME, a);
        assert!(!e.call(0x00c6_4980, &args![this, other]).bool());
        e.mem.set_u32(other + NAME, b);
        assert!(e.call(0x00c6_4980, &args![this, other]).bool());
        e.mem.set_cstr(b, b"Other");
        assert!(!e.call(0x00c6_4980, &args![this, other]).bool());
        e.mem.set_cstr(b, b"name");
        logged(&mut e, &log, NI_OBJECT_IS_EQUAL, 0x100);
        assert!(!e.call(0x00c6_4980, &args![this, other]).bool());
        assert_eq!(calls(&log, NI_OBJECT_IS_EQUAL)[0], vec![this, other]);
    }

    #[test]
    fn node_references_load_reads_the_link_ids() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_OBJECT_LOAD_BINARY, 0);
        logged(&mut e, &log, NI_STREAM_READ_MULTIPLE_LINK_IDS, 0);
        e.call(0x00c6_4aa0, &args![0x1000u32, 0x6000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (NI_OBJECT_LOAD_BINARY, vec![0x1000, 0x6000]),
                (NI_STREAM_READ_MULTIPLE_LINK_IDS, vec![0x6000]),
            ]
        );
    }

    #[test]
    fn node_references_link_attaches_each_complete_pair() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, OBJECT_BASE_STREAM_CALL, 0);
        logged(&mut e, &log, NODE_ARRAY_SET_SIZE, 0);
        logged(&mut e, &log, NI_STREAM_GET_NUMBER_OF_LINK_IDS, 0);
        e.call(0x00c6_4ad0, &args![0x1000u32, 0x6000u32]);
        assert!(calls(&log, NODE_ARRAY_SET_SIZE).is_empty());
        let vtable = e.mem.alloc(0xe0);
        e.mem.set_u32(vtable + 0xdc, 0x0040_6300);
        logged(&mut e, &log, 0x0040_6300, 0);
        let parent = e.mem.alloc(4);
        e.mem.set_u32(parent, vtable);
        let mut objects = vec![parent, 0x2222, 0, 0x3333, parent, 0x4444].into_iter();
        e.register_double(NI_STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            objects.next().unwrap().into_ret()
        });
        logged(&mut e, &log, NI_STREAM_GET_NUMBER_OF_LINK_IDS, 7);
        e.call(0x00c6_4ad0, &args![0x1000u32, 0x6000u32]);
        assert_eq!(calls(&log, NODE_ARRAY_SET_SIZE), vec![vec![0x1008, 7]]);
        assert_eq!(
            calls(&log, 0x0040_6300),
            vec![vec![parent, 0x2222, 0], vec![parent, 0x4444, 0]]
        );
        assert_eq!(calls(&log, OBJECT_BASE_STREAM_CALL).len(), 2);
    }

    #[test]
    fn node_references_save_writes_the_count_and_each_pair() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, OBJECT_BASE_STREAM_CALL, 0);
        e.register(STREAM_SAVE_WORD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 6);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x30, 0x0040_6400);
        logged(&mut e, &log, 0x0040_6400, 0);
        let stream = e.mem.alloc(4);
        e.mem.set_u32(stream, vtable);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x18, 0x1111);
        let third = e.mem.alloc(0x20);
        e.mem.set_u32(third + 0x18, 0x3333);
        let base = e.mem.alloc(0x10);
        e.mem.set_u32(base, first);
        e.mem.set_u32(base + 8, third);
        e.mem.set_u32(base + 0xc, 0x9999);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, base);
        e.mem.set_u16(this + 0x14, 3);
        e.call(0x00c6_4b90, &args![this, stream]);
        assert_eq!(
            calls(&log, 0x0040_6400),
            vec![
                vec![stream, 0x1111],
                vec![stream, first],
                vec![stream, 0x3333],
                vec![stream, third],
            ]
        );
    }
}
