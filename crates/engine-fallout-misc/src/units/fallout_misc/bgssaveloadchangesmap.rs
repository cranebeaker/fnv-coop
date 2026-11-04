//! `fallout/misc/saveload/bgssaveloadchangesmap.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;
use crate::types::NiTPointerMap;

layout! {
    /// `BGSFormChanges` (Xbox PDB); both PC words agree with the symbols.
    pub struct BGSFormChanges: 8 {
        0x00 iChangeFlags: u32,
        0x04 UnloadedFormBuffer: u32,
    }
}

// Translated from 00845580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadChangesMap::_scalar_deleting_destructor_` (Xbox PDB).
pub fn changes_map_delete(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_008455b0(e, this);
    if flags & 1 != 0 {
        e.call(0x00401030, &args![this]);
    }
    this
}

// Translation incomplete: 008455b0 - C++ exception-unwind frame omitted; normal destruction translated.
// Translated from 008455b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Changes-map destructor; the decompiler's MFC name is a false library match.
pub fn fn_008455b0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), 0x0107ed68);
    fn_00845610(e, this);
    fn_00845c00(e, this);
}

// Translated from 00845610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Delete each nonnull change record before clearing the map's nodes.
pub fn fn_00845610(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.with_stack(12, |e, temporary| {
        let cursor = e.call(0x004b9ba0, &args![this]).u32();
        e.mem.set_u32(temporary.addr(), cursor);
        while e.mem.u32(temporary.addr()) != 0 {
            e.mem.set_u32(temporary.addr() + 4, 0);
            e.mem.set_u32(temporary.addr() + 8, 0);
            e.call(
                0x006b7f20,
                &args![this, temporary, temporary.addr() + 4, temporary.addr() + 8],
            );
            let record = e.mem.u32(temporary.addr() + 8);
            if record != 0 {
                fn_00845690(e, Ptr::new(record), 1);
            }
        }
    });
    e.call(0x00438af0, &args![this]);
}

// Translated from 00845690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Delete the eight-byte change record, optionally releasing its storage.
pub fn fn_00845690(e: &mut Engine, this: Ptr<BGSFormChanges>, flags: u32) -> Ptr<BGSFormChanges> {
    fn_008456c0(e, this);
    if flags & 1 != 0 {
        e.call(0x00401030, &args![this]);
    }
    this
}

// Translated from 008456c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroy `BGSFormChanges::UnloadedFormBuffer` (Xbox PDB).
pub fn fn_008456c0(e: &mut Engine, this: Ptr<BGSFormChanges>) {
    e.call(0x00702220, &args![this.addr().wrapping_add(4)]);
}

// Translated from 008456e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy the record's change flags into the caller-provided result word.
pub fn fn_008456e0(e: &mut Engine, this: Ptr<NiTPointerMap>, result: Ptr, key: u32) -> Ptr {
    let record = e.call(0x009a4250, &args![this, key]).u32();
    if record == 0 {
        e.call(0x008c71b0, &args![result, 0u32]);
    } else {
        let value = e.mem.u32(record);
        e.mem.set_u32(result.addr(), value);
    }
    result
}

// Translated from 00845720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Test the supplied flags against a found change record; returns AL.
pub fn fn_00845720(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, flags: u32) -> u8 {
    let record = e.call(0x009a4250, &args![this, key]).u32();
    if record == 0 {
        return 0;
    }
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), flags);
        let value = e.call(0x00559450, &args![argument]).u32();
        e.call(0x004280f0, &args![record, value]).u32() as u8
    })
}

// Translated from 00845760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy the record's unloaded-buffer word into the explicit result.
pub fn fn_00845760(e: &mut Engine, this: Ptr<NiTPointerMap>, result: Ptr, key: u32) -> Ptr {
    let record = e.call(0x009a4250, &args![this, key]).u32();
    if record == 0 {
        e.call(0x008c71b0, &args![result, 0u32]);
    } else {
        let value = e.mem.u32(record.wrapping_add(4));
        e.mem.set_u32(result.addr(), value);
    }
    result
}

fn new_change_record(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32) -> u32 {
    let allocation = e.call(0x00401000, &args![8u32]).u32();
    let record = if allocation == 0 {
        0
    } else {
        fn_00845880(e, Ptr::new(allocation)).addr()
    };
    e.call(0x00844700, &args![this, key, record]);
    record
}

// Translation incomplete: 008457b0 - C++ allocation exception-unwind frame omitted.
// Translated from 008457b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadChangesMap::AddChangeFlags` (Xbox PDB).
pub fn add_change_flags(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, flags: u32) {
    let mut record = e.call(0x009a4250, &args![this, key]).u32();
    if record == 0 {
        record = new_change_record(e, this, key);
    } else if e.call(0x00559450, &args![record.wrapping_add(4)]).u32() != 0 {
        return;
    }
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), flags);
        let value = e.call(0x00559450, &args![argument]).u32();
        e.call(0x00841100, &args![record, value]);
    });
}

// Translated from 00845880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Construct the two embedded words of `BGSFormChanges` (Xbox PDB).
pub fn fn_00845880(e: &mut Engine, this: Ptr<BGSFormChanges>) -> Ptr<BGSFormChanges> {
    e.call(0x0044dee0, &args![this]);
    e.call(0x0044dee0, &args![this.addr().wrapping_add(4)]);
    this
}

// Translation incomplete: 008458b0 - C++ allocation exception-unwind frame omitted.
// Translated from 008458b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replace a record's flags, creating a record when absent.
pub fn fn_008458b0(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, flags: u32) {
    let mut record = e.call(0x009a4250, &args![this, key]).u32();
    if record == 0 {
        record = new_change_record(e, this, key);
    }
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), flags);
        let value = e.call(0x00559450, &args![argument]).u32();
        e.call(0x00537e90, &args![record, value]);
    });
}

// Translation incomplete: 00845960 - C++ allocation exception-unwind frame omitted.
// Translated from 00845960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replace the flags and unloaded-buffer word, creating a record when absent.
pub fn fn_00845960(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, flags: u32, buffer: u32) {
    let mut record = e.call(0x009a4250, &args![this, key]).u32();
    if record == 0 {
        record = new_change_record(e, this, key);
    }
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), flags);
        let value = e.call(0x00559450, &args![argument]).u32();
        e.call(0x00537e90, &args![record, value]);
    });
    e.mem.set_u32(record.wrapping_add(4), buffer);
}

// Translated from 00845a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadChangesMap::RemoveChanges` (Xbox PDB).
pub fn remove_changes(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32) -> bool {
    let record = e.call(0x009a4250, &args![this, key]).u32();
    if record == 0 {
        return false;
    }
    fn_00845690(e, Ptr::new(record), 1);
    e.call(0x00405430, &args![this, key]);
    true
}

// Translated from 00845a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Remove selected flags, retaining a record while other flags or the manager require it.
pub fn fn_00845a80(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, flags: u32) -> bool {
    let record = e.call(0x009a4250, &args![this, key]).u32();
    if record != 0 && e.call(0x00559450, &args![record.wrapping_add(4)]).u32() == 0 {
        e.with_stack(4, |e, argument| {
            e.mem.set_u32(argument.addr(), flags);
            let value = e.call(0x00559450, &args![argument]).u32();
            e.call(0x00562210, &args![record, value]);
        });
        if e.call(0x00559450, &args![record]).u32() != 0 {
            return false;
        }
        let manager = e.mem.u32(0x011c3f2c);
        if e.call(0x00469860, &args![manager, key]).bool() {
            return false;
        }
        fn_00845690(e, Ptr::new(record), 1);
        e.call(0x00405430, &args![this, key]);
    }
    true
}

// Translated from 00845b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Construct the pointer-map specialization and install its vtable.
pub fn fn_00845b30(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr<NiTPointerMap> {
    fn_00845b90(e, this, buckets);
    e.mem.set_u32(this.addr(), 0x0107ed88);
    this
}

// Translated from 00845b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, BGSFormChanges*>::_scalar_deleting_destructor_` (Xbox PDB).
pub fn pointer_map_delete(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00845c00(e, this);
    if flags & 1 != 0 {
        e.call(0x00401030, &args![this]);
    }
    this
}

// Translated from 00845b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialize the map base and zero the freshly allocated bucket array.
pub fn fn_00845b90(e: &mut Engine, this: Ptr<NiTPointerMap>, buckets: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), 0x0107eda8);
    e.set(this, NiTPointerMap::m_uiHashSize, buckets);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let size = e.get(this, NiTPointerMap::m_uiHashSize).wrapping_shl(2);
    let table = e.call(0x00aa1070, &args![size]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    let size = e.get(this, NiTPointerMap::m_uiHashSize).wrapping_shl(2);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(0x00403d30, &args![table, 0u32, size]);
    this
}

// Translation incomplete: 00845c00 - C++ exception-unwind frame omitted; normal destruction translated.
// Translated from 00845c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pointer-map destructor, including both normal-path clear calls.
pub fn fn_00845c00(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), 0x0107ed88);
    e.call(0x00438af0, &args![this]);
    fn_00845c60(e, this);
}

// Translated from 00845c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Map-base destruction: clear nodes, then release the current bucket array.
pub fn fn_00845c60(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), 0x0107eda8);
    e.call(0x00438af0, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(0x00aa10f0, &args![table]);
}

// Translated from 00845c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, BGSFormChanges*>::_scalar_deleting_destructor_` (Xbox PDB).
pub fn map_base_delete(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_00845c60(e, this);
    if flags & 1 != 0 {
        e.call(0x00401030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00845580,
            changes_map_delete(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x008455b0, fn_008455b0(Ptr<NiTPointerMap>) -> ()),
        entry!(0x00845610, fn_00845610(Ptr<NiTPointerMap>) -> ()),
        entry!(
            0x00845690,
            fn_00845690(Ptr<BGSFormChanges>, u32) -> Ptr<BGSFormChanges>
        ),
        entry!(0x008456c0, fn_008456c0(Ptr<BGSFormChanges>) -> ()),
        entry!(0x008456e0, fn_008456e0(Ptr<NiTPointerMap>, Ptr, u32) -> Ptr),
        entry!(0x00845720, fn_00845720(Ptr<NiTPointerMap>, u32, u32) -> u8),
        entry!(0x00845760, fn_00845760(Ptr<NiTPointerMap>, Ptr, u32) -> Ptr),
        entry!(
            0x008457b0,
            add_change_flags(Ptr<NiTPointerMap>, u32, u32) -> ()
        ),
        entry!(
            0x00845880,
            fn_00845880(Ptr<BGSFormChanges>) -> Ptr<BGSFormChanges>
        ),
        entry!(0x008458b0, fn_008458b0(Ptr<NiTPointerMap>, u32, u32) -> ()),
        entry!(
            0x00845960,
            fn_00845960(Ptr<NiTPointerMap>, u32, u32, u32) -> ()
        ),
        entry!(0x00845a20, remove_changes(Ptr<NiTPointerMap>, u32) -> bool),
        entry!(
            0x00845a80,
            fn_00845a80(Ptr<NiTPointerMap>, u32, u32) -> bool
        ),
        entry!(
            0x00845b30,
            fn_00845b30(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00845b60,
            pointer_map_delete(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00845b90,
            fn_00845b90(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00845c00, fn_00845c00(Ptr<NiTPointerMap>) -> ()),
        entry!(0x00845c60, fn_00845c60(Ptr<NiTPointerMap>) -> ()),
        entry!(
            0x00845c90,
            map_base_delete(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Engine, Ptr<NiTPointerMap>, Ptr<BGSFormChanges>) {
        let mut e = Engine::new();
        e.map(0x1000, 0x1000);
        e.map(0x011c3000, 0x1000);
        let map = e.new_object::<NiTPointerMap>();
        let record = e.new_object::<BGSFormChanges>();
        e.set_global(0x1000, record.addr());
        e.set_global(0x1010, 0u32);
        e.set_global(0x011c3f2c, 0x6000u32);
        e.call_log = Some(vec![]);
        e.register(0x009a4250, |e, a| {
            assert_eq!(a[1], 42);
            e.mem.u32(0x1000).into_ret()
        });
        e.register(0x004b9ba0, |e, _| e.mem.u32(0x1010).into_ret());
        for address in [0x00438af0, 0x00aa10f0, 0x00401030, 0x00702220, 0x00405430] {
            e.register(address, |_, _| ().into_ret());
        }
        e.register(0x00401000, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00aa1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00403d30, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(0x0044dee0, |e, a| {
            e.mem.set_u32(a[0], 0);
            a[0].into_ret()
        });
        e.register(0x008c71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(0x00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x00844700, |e, a| {
            assert_eq!(a[1], 42);
            e.mem.set_u32(0x1000, a[2]);
            ().into_ret()
        });
        e.register(0x00841100, |e, a| {
            let old = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], old | a[1]);
            ().into_ret()
        });
        e.register(0x00537e90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ().into_ret()
        });
        e.register(0x00562210, |e, a| {
            let old = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], old & !a[1]);
            ().into_ret()
        });
        e.register(0x00469860, |_, a| {
            assert_eq!(a, &[0x6000, 42]);
            0x100u32.into_ret()
        });
        (e, map, record)
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, a)| a.clone())
            .collect()
    }

    #[test]
    fn deleting_changes_map_obeys_low_flag_bit() {
        for flags in [0, 1, 2, 3] {
            let (mut e, map, _) = setup();
            assert_eq!(changes_map_delete(&mut e, map, flags), map);
            assert_eq!(calls(&e, 0x00401030).len(), (flags & 1) as usize);
            assert_eq!(calls(&e, 0x00438af0).len(), 3);
        }
    }

    #[test]
    fn changes_map_destructor_installs_vtables_in_order() {
        let (mut e, map, _) = setup();
        e.register(0x00438af0, |e, a| {
            let n = e.mem.u32(0x1010);
            assert_eq!(
                e.mem.u32(a[0]),
                [0x0107ed68, 0x0107ed88, 0x0107eda8][n as usize]
            );
            e.mem.set_u32(0x1010, n + 1);
            ().into_ret()
        });
        fn_008455b0(&mut e, map);
        assert_eq!(e.mem.u32(0x1010), 3);
    }

    #[test]
    fn clear_map_visits_null_and_nonnull_records() {
        for present in [false, true] {
            let (mut e, map, record) = setup();
            e.set_global(0x1010, 1u32);
            e.register_double(0x006b7f20, move |e, a| {
                assert_eq!(a[0], map.addr());
                assert_eq!(e.mem.u32(a[2]), 0);
                assert_eq!(e.mem.u32(a[3]), 0);
                e.mem.set_u32(a[1], 0);
                e.mem.set_u32(a[3], if present { record.addr() } else { 0 });
                ().into_ret()
            });
            fn_00845610(&mut e, map);
            assert_eq!(calls(&e, 0x00401030).len(), usize::from(present));
            assert_eq!(calls(&e, 0x00438af0), vec![vec![map.addr()]]);
        }
    }

    #[test]
    fn record_deleting_destructor_calls_member_before_free() {
        for flags in [0, 1, 2, 3] {
            let (mut e, _, record) = setup();
            assert_eq!(fn_00845690(&mut e, record, flags), record);
            let log = e.call_log.as_ref().unwrap();
            assert_eq!(log[0], (0x00702220, vec![record.addr() + 4]));
            assert_eq!(calls(&e, 0x00401030).len(), (flags & 1) as usize);
        }
    }

    #[test]
    fn record_destructor_only_destroys_second_word() {
        let (mut e, _, record) = setup();
        e.set(record, BGSFormChanges::iChangeFlags, 0xa5a5);
        fn_008456c0(&mut e, record);
        assert_eq!(calls(&e, 0x00702220), vec![vec![record.addr() + 4]]);
        assert_eq!(e.get(record, BGSFormChanges::iChangeFlags), 0xa5a5);
    }

    #[test]
    fn get_flags_copies_or_constructs_zero() {
        for present in [false, true] {
            let (mut e, map, record) = setup();
            let out = Ptr::new(e.mem.alloc(4));
            e.set(record, BGSFormChanges::iChangeFlags, 0xf0000123);
            if !present {
                e.set_global(0x1000, 0u32);
            }
            assert_eq!(fn_008456e0(&mut e, map, out, 42), out);
            assert_eq!(e.mem.u32(out.addr()), if present { 0xf0000123 } else { 0 });
            assert_eq!(calls(&e, 0x008c71b0).len(), usize::from(!present));
        }
    }

    #[test]
    fn test_flags_forwards_stack_word_and_preserves_al() {
        for present in [false, true] {
            let (mut e, map, record) = setup();
            if !present {
                e.set_global(0x1000, 0u32);
            }
            e.register_double(0x004280f0, move |_, a| {
                assert_eq!(a, &[record.addr(), 0x98765432]);
                0x1234feu32.into_ret()
            });
            assert_eq!(
                fn_00845720(&mut e, map, 42, 0x98765432),
                if present { 0xfe } else { 0 }
            );
            assert_eq!(calls(&e, 0x00559450).len(), usize::from(present));
        }
    }

    #[test]
    fn get_buffer_copies_or_constructs_zero() {
        for present in [false, true] {
            let (mut e, map, record) = setup();
            let out = Ptr::new(e.mem.alloc(4));
            e.set(record, BGSFormChanges::UnloadedFormBuffer, 0x12345678);
            if !present {
                e.set_global(0x1000, 0u32);
            }
            assert_eq!(fn_00845760(&mut e, map, out, 42), out);
            assert_eq!(e.mem.u32(out.addr()), if present { 0x12345678 } else { 0 });
        }
    }

    #[test]
    fn add_flags_handles_new_existing_and_loaded_records() {
        for (present, buffer) in [(false, 0), (true, 0), (true, 99)] {
            let (mut e, map, record) = setup();
            e.set(record, BGSFormChanges::iChangeFlags, 1);
            e.set(record, BGSFormChanges::UnloadedFormBuffer, buffer);
            if !present {
                e.set_global(0x1000, 0u32);
            }
            add_change_flags(&mut e, map, 42, 6);
            let found = e.mem.u32(0x1000);
            assert_eq!(
                e.mem.u32(found),
                if buffer != 0 {
                    1
                } else if present {
                    7
                } else {
                    6
                }
            );
            assert_eq!(calls(&e, 0x00401000).len(), usize::from(!present));
        }
    }

    #[test]
    fn change_record_constructor_matches_native_guard_vectors() {
        for fill in [0xa5, 0xcc, 0xff] {
            for offset in [4, 8] {
                let (mut e, _, _) = setup();
                let allocation = e.mem.alloc(24);
                e.mem.write(allocation, &[fill; 24]);
                let this = Ptr::new(allocation + offset);
                assert_eq!(fn_00845880(&mut e, this), this);
                let mut actual = [0; 24];
                e.mem.read(allocation, &mut actual);
                let mut expected = [fill; 24];
                expected[offset as usize..offset as usize + 8].fill(0);
                assert_eq!(actual, expected);
                assert_eq!(
                    calls(&e, 0x0044dee0),
                    vec![vec![this.addr()], vec![this.addr() + 4]]
                );
            }
        }
    }

    #[test]
    fn replace_flags_handles_existing_and_missing_records() {
        for present in [false, true] {
            let (mut e, map, record) = setup();
            e.set(record, BGSFormChanges::iChangeFlags, u32::MAX);
            e.set(record, BGSFormChanges::UnloadedFormBuffer, 99);
            if !present {
                e.set_global(0x1000, 0u32);
            }
            fn_008458b0(&mut e, map, 42, 8);
            let found = e.mem.u32(0x1000);
            assert_eq!(e.mem.u32(found), 8);
            assert_eq!(e.mem.u32(found + 4), if present { 99 } else { 0 });
        }
    }

    #[test]
    fn replace_flags_and_buffer_writes_buffer_after_callback() {
        for present in [false, true] {
            let (mut e, map, _) = setup();
            if !present {
                e.set_global(0x1000, 0u32);
            }
            e.register(0x00537e90, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                e.mem.set_u32(a[0] + 4, 999);
                ().into_ret()
            });
            fn_00845960(&mut e, map, 42, 8, 123);
            let found = e.mem.u32(0x1000);
            assert_eq!(e.mem.u32(found), 8);
            assert_eq!(e.mem.u32(found + 4), 123);
        }
    }

    #[test]
    fn remove_changes_returns_found_and_deletes_before_removing_key() {
        for present in [false, true] {
            let (mut e, map, _) = setup();
            if !present {
                e.set_global(0x1000, 0u32);
            }
            assert_eq!(remove_changes(&mut e, map, 42), present);
            assert_eq!(calls(&e, 0x00405430).len(), usize::from(present));
            if present {
                let addresses: Vec<_> = e
                    .call_log
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(a, _)| *a)
                    .collect();
                assert_eq!(addresses, [0x009a4250, 0x00702220, 0x00401030, 0x00405430]);
            }
        }
    }

    #[test]
    fn remove_selected_flags_covers_every_retention_gate() {
        for (present, buffer, flags, manager_keeps, result, removes) in [
            (false, 0, 1, false, true, false),
            (true, 2, 1, false, true, false),
            (true, 0, 3, false, false, false),
            (true, 0, 1, true, false, false),
            (true, 0, 1, false, true, true),
        ] {
            let (mut e, map, record) = setup();
            e.set(record, BGSFormChanges::iChangeFlags, flags);
            e.set(record, BGSFormChanges::UnloadedFormBuffer, buffer);
            if !present {
                e.set_global(0x1000, 0u32);
            }
            e.register_double(0x00469860, move |_, _| {
                (if manager_keeps { 0x180u32 } else { 0x100u32 }).into_ret()
            });
            assert_eq!(fn_00845a80(&mut e, map, 42, 1), result);
            assert_eq!(calls(&e, 0x00405430).len(), usize::from(removes));
        }
    }

    #[test]
    fn pointer_map_constructor_installs_derived_table() {
        let (mut e, map, _) = setup();
        assert_eq!(fn_00845b30(&mut e, map, 3), map);
        assert_eq!(e.mem.u32(map.addr()), 0x0107ed88);
        assert_eq!(e.get(map, NiTPointerMap::m_uiHashSize), 3);
    }

    #[test]
    fn pointer_map_deleting_destructor_tests_low_bit() {
        for flags in [0, 1, 2, 3] {
            let (mut e, map, _) = setup();
            assert_eq!(pointer_map_delete(&mut e, map, flags), map);
            assert_eq!(calls(&e, 0x00438af0).len(), 2);
            assert_eq!(calls(&e, 0x00401030).len(), (flags & 1) as usize);
        }
    }

    #[test]
    fn map_base_constructor_reloads_size_after_allocation() {
        let (mut e, map, _) = setup();
        e.register_double(0x00aa1070, move |e, a| {
            assert_eq!(a, &[12]);
            assert_eq!(e.mem.u32(map.addr()), 0x0107eda8);
            assert_eq!(e.get(map, NiTPointerMap::m_uiCount), 0);
            e.set(map, NiTPointerMap::m_uiHashSize, 2);
            let result = e.mem.alloc(12);
            e.mem.write(result, &[0xa5; 12]);
            result.into_ret()
        });
        assert_eq!(fn_00845b90(&mut e, map, 3), map);
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        assert_eq!(calls(&e, 0x00403d30), vec![vec![table, 0, 8]]);
        assert_eq!(e.mem.u32(table + 8), 0xa5a5a5a5);
    }

    #[test]
    fn pointer_map_destructor_performs_both_clear_calls() {
        let (mut e, map, _) = setup();
        fn_00845c00(&mut e, map);
        assert_eq!(
            calls(&e, 0x00438af0),
            vec![vec![map.addr()], vec![map.addr()]]
        );
        assert_eq!(e.mem.u32(map.addr()), 0x0107eda8);
    }

    #[test]
    fn map_base_destructor_reloads_bucket_pointer_after_clear() {
        let (mut e, map, _) = setup();
        e.register(0x00438af0, |e, a| {
            e.mem.set_u32(a[0] + 8, 0x12345678);
            ().into_ret()
        });
        fn_00845c60(&mut e, map);
        assert_eq!(calls(&e, 0x00aa10f0), vec![vec![0x12345678]]);
    }

    #[test]
    fn map_base_deleting_destructor_tests_low_bit() {
        for flags in [0, 1, 2, 3] {
            let (mut e, map, _) = setup();
            assert_eq!(map_base_delete(&mut e, map, flags), map);
            assert_eq!(calls(&e, 0x00438af0).len(), 1);
            assert_eq!(calls(&e, 0x00401030).len(), (flags & 1) as usize);
        }
    }

    #[test]
    fn null_record_allocation_still_reaches_insert_and_update() {
        for address in [0x008457b0, 0x008458b0] {
            let (mut e, map, _) = setup();
            e.set_global(0x1000, 0u32);
            e.register(0x00401000, |_, _| 0u32.into_ret());
            e.register(0x00841100, |_, a| {
                assert_eq!(a, &[0, 123]);
                ().into_ret()
            });
            e.register(0x00537e90, |_, a| {
                assert_eq!(a, &[0, 123]);
                ().into_ret()
            });
            e.call(address, &args![map, 42u32, 123u32]);
            assert_eq!(calls(&e, 0x00844700), vec![vec![map.addr(), 42, 0]]);
        }
    }
}
