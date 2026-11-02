//! `fallout shared/run_00600f70` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constructors of the template classes behind `TESIdleManager`'s
//! root map (`NiTStringMap<IDLE_ANIM_ROOT*>` down to `NiTMapBase`) and of
//! `NiTArray<TESIdleForm*,NiTMallocInterface<..>>`, each named by the RTTI
//! of the vtable it stores.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTStringMap<IDLE_ANIM_ROOT*>`'s vtable (RTTI).
const STRING_MAP_VTABLE: u32 = 0x0104_a0d0;
/// `NiTStringTemplateMap<NiTMap<const char*,IDLE_ANIM_ROOT*>,..>`'s vtable
/// (RTTI).
const STRING_TEMPLATE_MAP_VTABLE: u32 = 0x0104_a070;
/// `NiTMap<const char*,IDLE_ANIM_ROOT*>`'s vtable (RTTI).
const MAP_VTABLE: u32 = 0x0104_a090;
/// `NiTMapBase<DFALL<NiTMapItem<..>>,..>`'s vtable (RTTI).
const MAP_BASE_VTABLE: u32 = 0x0104_a0b0;
/// `NiTArray<TESIdleForm*,NiTMallocInterface<..>>`'s vtable (RTTI).
const IDLE_FORM_ARRAY_VTABLE: u32 = 0x0104_a060;
/// The bucket table allocation (`00aa1070`, cdecl(size)).
const ALLOCATE_TABLE: u32 = 0x00aa_1070;
/// `memset` (`00403d30`, cdecl(destination, value, size)).
const MEMSET: u32 = 0x0040_3d30;
/// `NiTMallocInterface` allocation of `count` elements (`0096afc0`, cdecl).
const MALLOC_ALLOCATE: u32 = 0x0096_afc0;

/// `NiTMapBase` fields (offsets from the PC code): the bucket count, the
/// bucket table and the item count.
const MAP_HASH_SIZE: u32 = 0x04;
const MAP_TABLE: u32 = 0x08;
const MAP_COUNT: u32 = 0x0c;
/// `NiTStringTemplateMap`'s key-copy flag.
const MAP_COPY_KEYS: u32 = 0x10;

// Translated from 00600fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTStringMap<IDLE_ANIM_ROOT*>` constructor (thiscall(size, copy
/// flag)): the `NiTStringTemplateMap` one (`006010a0`, the flag's low
/// byte), then its own vtable. Returns `this`.
pub fn fn_00600fd0(e: &mut Engine, this: Ptr, size: u32, copy: u8) -> Ptr {
    fn_006010a0(e, this, size, copy);
    e.mem.set_u32(this.addr(), STRING_MAP_VTABLE);
    this
}

// Translated from 00601000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<TESIdleForm*,..>` constructor (thiscall(max size, grow
/// by), the low half of each word): `m_usMaxSize` and `m_usGrowBy` set,
/// size and used size cleared, `m_pBase` allocated for `max_size` elements
/// (null for 0). Returns `this`.
pub fn fn_00601000(e: &mut Engine, this: Ptr, max_size: u16, grow_by: u16) -> Ptr {
    let array: Ptr<crate::types::NiTArray> = this.cast();
    e.mem.set_u32(this.addr(), IDLE_FORM_ARRAY_VTABLE);
    e.set(array, crate::types::NiTArray::m_usMaxSize, max_size);
    e.set(array, crate::types::NiTArray::m_usGrowBy, grow_by);
    e.set(array, crate::types::NiTArray::m_usSize, 0);
    e.set(array, crate::types::NiTArray::m_usESize, 0);
    let max = e.get(array, crate::types::NiTArray::m_usMaxSize);
    if max as i32 > 0 {
        let base = e.call(MALLOC_ALLOCATE, &args![u32::from(max)]).u32();
        e.set(array, crate::types::NiTArray::m_pBase, base);
    } else {
        e.set(array, crate::types::NiTArray::m_pBase, 0);
    }
    this
}

// Translated from 006010a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTStringTemplateMap<..>` constructor (thiscall(size, copy flag)):
/// the `NiTMap` one (`006010d0`), its vtable, and the key-copy byte at
/// `+0x10`. Returns `this`.
pub fn fn_006010a0(e: &mut Engine, this: Ptr, size: u32, copy: u8) -> Ptr {
    fn_006010d0(e, this, size);
    e.mem.set_u32(this.addr(), STRING_TEMPLATE_MAP_VTABLE);
    e.mem.set_u8(this.addr() + MAP_COPY_KEYS, copy);
    this
}

// Translated from 006010d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<const char*,IDLE_ANIM_ROOT*>` constructor (thiscall(size)):
/// the `NiTMapBase` one (`00601100`), then its vtable. Returns `this`.
pub fn fn_006010d0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    fn_00601100(e, this, size);
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    this
}

// Translated from 00601100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<..>` constructor (thiscall(size)): its vtable, the
/// bucket count, the item count cleared, and a bucket table of `size << 2`
/// bytes (`00aa1070`) cleared with `memset`. Returns `this`.
pub fn fn_00601100(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, MAP_BASE_VTABLE);
    e.mem.set_u32(base + MAP_HASH_SIZE, size);
    e.mem.set_u32(base + MAP_COUNT, 0);
    let bytes = e.mem.u32(base + MAP_HASH_SIZE) << 2;
    let table = e.call(ALLOCATE_TABLE, &args![bytes]).u32();
    e.mem.set_u32(base + MAP_TABLE, table);
    let bytes = e.mem.u32(base + MAP_HASH_SIZE) << 2;
    let table = e.mem.u32(base + MAP_TABLE);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

/// The scalar deleting destructor shape: `body` on `this`, then `00401030`
/// frees `this` when bit 0 of `flags` is set. Returns `this`.
fn delete_with(e: &mut Engine, this: Ptr, flags: u32, body: u32) -> Ptr {
    e.call(body, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00600f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (`RET 4`; the engine map gives it no
/// name; the class is unconfirmed): the destructor body `00600b90` on
/// `this`, then `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00600f70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_with(e, this, flags, 0x0060_0b90)
}

// Translated from 00600fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (`RET 4`; the engine map gives it no
/// name; the class is unconfirmed): the destructor body `005ffbd0` on
/// `this`, then `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00600fa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_with(e, this, flags, 0x005f_fbd0)
}

// Translated from 00601070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (`RET 4`; the engine map gives it no
/// name; the class is unconfirmed): the destructor body `00600d70` on
/// `this`, then `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00601070(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_with(e, this, flags, 0x0060_0d70)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060_0f70, fn_00600f70(Ptr, u32) -> Ptr),
        entry!(0x0060_0fa0, fn_00600fa0(Ptr, u32) -> Ptr),
        entry!(0x0060_1070, fn_00601070(Ptr, u32) -> Ptr),
        entry!(0x0060_0fd0, fn_00600fd0(Ptr, u32, u8) -> Ptr),
        entry!(0x0060_1000, fn_00601000(Ptr, u16, u16) -> Ptr),
        entry!(0x0060_10a0, fn_006010a0(Ptr, u32, u8) -> Ptr),
        entry!(0x0060_10d0, fn_006010d0(Ptr, u32) -> Ptr),
        entry!(0x0060_1100, fn_00601100(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::NiTArray;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE_TABLE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(MALLOC_ALLOCATE, |_, a| (0x0010_0000 + a[0]).into_ret());
        e
    }

    #[test]
    fn map_base_allocates_and_clears_its_table() {
        let mut e = engine();
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + MAP_COUNT, 5);
        e.call_log = Some(vec![]);
        let back = e.call(0x0060_1100, &args![map, 0x25u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, map);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + MAP_HASH_SIZE), 0x25);
        assert_eq!(e.mem.u32(map + MAP_COUNT), 0);
        let table = e.mem.u32(map + MAP_TABLE);
        assert_eq!(calls(&log, ALLOCATE_TABLE), vec![vec![0x94]]);
        assert_eq!(calls(&log, MEMSET), vec![vec![table, 0, 0x94]]);
    }

    #[test]
    fn map_layers_store_their_vtables_and_flag() {
        let mut e = engine();
        let map = e.mem.alloc(0x14);
        assert_eq!(e.call(0x0060_10d0, &args![map, 3u32]).u32(), map);
        assert_eq!(e.mem.u32(map), MAP_VTABLE);
        assert_eq!(e.call(0x0060_10a0, &args![map, 3u32, 0x0201u32]).u32(), map);
        assert_eq!(e.mem.u32(map), STRING_TEMPLATE_MAP_VTABLE);
        assert_eq!(e.mem.u8(map + MAP_COPY_KEYS), 1);
        assert_eq!(e.call(0x0060_0fd0, &args![map, 3u32, 0u32]).u32(), map);
        assert_eq!(e.mem.u32(map), STRING_MAP_VTABLE);
        assert_eq!(e.mem.u8(map + MAP_COPY_KEYS), 0);
        assert_eq!(e.mem.u32(map + MAP_HASH_SIZE), 3);
    }

    #[test]
    fn idle_form_array_constructor() {
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        e.set(array, NiTArray::m_usSize, 3);
        let back = e
            .call(0x0060_1000, &args![array, 0x0001_0004u32, 2u32])
            .u32();
        assert_eq!(back, array.addr());
        assert_eq!(e.mem.u32(array.addr()), IDLE_FORM_ARRAY_VTABLE);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 4);
        assert_eq!(e.get(array, NiTArray::m_usGrowBy), 2);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0x0010_0004);
        e.call(0x0060_1000, &args![array, 0u32, 2u32]);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    #[test]
    fn deleting_destructors_free_only_with_bit_zero() {
        for (addr, body) in [
            (0x0060_0f70u32, 0x0060_0b90u32),
            (0x0060_0fa0, 0x005f_fbd0),
            (0x0060_1070, 0x0060_0d70),
        ] {
            let mut e = Engine::new();
            e.register(body, |_, _| Ret::default());
            e.register(0x0040_1030, |_, _| Ret::default());
            for (flags, frees) in [(0u32, false), (2, false), (1, true), (3, true)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(addr, &args![0x1000u32, flags]).u32(), 0x1000);
                let log = e.call_log.take().unwrap();
                let mut expected = vec![(addr, vec![0x1000, flags]), (body, vec![0x1000])];
                if frees {
                    expected.push((0x0040_1030, vec![0x1000]));
                }
                assert_eq!(log, expected);
            }
        }
    }
}
