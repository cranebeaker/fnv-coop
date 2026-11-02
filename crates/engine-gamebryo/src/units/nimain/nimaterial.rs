//! `NiMain/nimaterial.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiMaterial::ms_RTTI` (the class's `NiRTTI` record).
const NI_MATERIAL_RTTI: u32 = 0x011f_5e38;

// Translated from 00a8f860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterial::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_material_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_MATERIAL_RTTI)
}

/// `011f5e34`: the global `NiTFixedStringMap` of materials `00a8f870` walks (hash size at
/// +4, bucket table at +8).
const MATERIAL_MAP: u32 = 0x011f_5e34;
/// `00a8f770` (`thiscall` on the map, `RET 0xc`; engine map
/// `NiTFixedStringMapBase<...>::GetNext`): advances the iterator at the first address and
/// stores the key and the value at the other two.
const MAP_GET_NEXT: u32 = 0x00a8_f770;
/// `00aa01f0` (`thiscall`, no stack word; engine map `NiTFixedStringMapBase<...>::RemoveAll`).
const MAP_REMOVE_ALL: u32 = 0x00aa_01f0;
/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// The memory manager singleton and its deallocate (`00aa4060`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
/// Import slot of `InterlockedDecrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The handle of the empty `NiFixedString`, which is never counted.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;

// Translated from 00a8f870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimaterial.cpp`, no arguments): walks the material map
/// at `011f5e34` and calls vtable slot +0x50 of every value. The iterator starts at the first
/// non-empty bucket (none: nothing is walked); the key starts as the empty string's handle
/// and the value as null. Each step is `GetNext` (`00a8f770`, the map read again from the
/// global each time) with the addresses of the iterator, the key and the value; the walk
/// ends when the iterator is null, and the last key is released (unless empty). The
/// decompiler shows the key starting at 0; the disassembly loads the global `0109b220`.
pub fn fn_00a8f870(e: &mut Engine) {
    let map = e.mem.u32(MATERIAL_MAP);
    let size = e.mem.u32(map + 4);
    let mut position = 0u32;
    if size > 0 {
        let buckets = e.mem.u32(map + 8);
        let mut index = 0u32;
        loop {
            if e.mem.u32(buckets.wrapping_add(4 * index)) != 0 {
                position = e.mem.u32(buckets.wrapping_add(4 * index));
                break;
            }
            index = index.wrapping_add(1);
            if index >= size {
                break;
            }
        }
    }
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    // The three stack words: the value (+0), the iterator (+4), the key (+8).
    e.with_stack(0xc, |e, locals| {
        let value = locals.addr();
        let iterator = value + 4;
        let key = value + 8;
        e.mem.set_u32(iterator, position);
        e.mem.set_u32(value, 0);
        e.mem.set_u32(key, empty);
        if position == 0 {
            return;
        }
        let mut map = map;
        loop {
            e.call(MAP_GET_NEXT, &args![map, iterator, key, value]);
            let object = e.mem.u32(value);
            if object != 0 {
                e.vcall(object, 0x50, &args![]);
            }
            if e.mem.u32(iterator) == 0 {
                break;
            }
            map = e.mem.u32(MATERIAL_MAP);
        }
        let handle = e.mem.u32(key);
        if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
}

// Translated from 00a8fa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTFixedStringMap<NiMaterial *>::scalar deleting destructor` (Xbox PDB name in the engine
/// map; `thiscall`, `RET 4`): sets the vtable `0109f798` and empties the map (`00aa01f0`),
/// sets the base vtable `0109f72c` and empties it again, frees the bucket table at +8
/// (`00aa10f0`), and for bit 0 of `flags` gives `this` back to the memory manager. Returns
/// `this`.
pub fn ni_t_fixed_string_map_ni_material_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0109_f798);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(base, 0x0109_f72c);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(base + 8);
    e.call(FREE_BLOCK, &args![buckets]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// Import slot of `InterlockedIncrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `NiRefObject::ms_uiObjects` (`011f4400`) and the vtables of `NiRefObject` (`0101dce4`),
/// `NiMaterial` (`0109f73c`) and the material map's base class (`0109f72c`, RTTI
/// `NiTFixedStringMapBase<DFALL<NiTMapItem<NiFixedString,NiMaterial *>>,NiMaterial *>`).
const REF_OBJECT_COUNT: u32 = 0x011f_4400;
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const NI_MATERIAL_VTABLE: u32 = 0x0109_f73c;
const MATERIAL_MAP_BASE_VTABLE: u32 = 0x0109_f72c;
/// The material map's `SetAt` (`00a6e980`, engine map `NiTFixedStringMapBase<..>::SetAt`;
/// `thiscall`, the key's address and the value) and `00aa0250` (`thiscall` on the map, the
/// key's address), which the destructor runs with the material's name.
const MAP_SET_AT: u32 = 0x00a6_e980;
const MAP_REMOVE_KEY: u32 = 0x00aa_0250;
/// The sized delete (`00aa1460`, `cdecl`: pointer, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiMaterial` (Xbox PDB) `m_kName` (+8, an `NiFixedString` handle whose count is 8 bytes
/// before it).
const NAME: u32 = 0x8;

// Translated from 00a8f910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the scalar deleting destructor of the material map's base
/// class by the vtable it sets, `0109f72c`): sets that vtable, empties the map
/// (`00aa01f0`), frees the bucket table at +8 (`00aa10f0`), and for bit 0 of `flags` gives
/// `this` back to the memory manager. Answers `this`.
pub fn fn_00a8f910(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), MATERIAL_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE_BLOCK, &args![buckets]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a8f950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterial::NiMaterial` (Xbox PDB; the address of the name): the inlined `NiRefObject`
/// constructor (its vtable, count 0, `ms_uiObjects` raised), `NiMaterial`'s vtable, the
/// name's handle taken (its count raised unless it is the empty string's) and stored at +8,
/// and the material entered in the material map under its name (`SetAt`). Answers `this`.
pub fn ni_material_ni_material(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    let material = this.addr();
    e.mem.set_u32(material, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(material + 4, 0);
    e.call(INTERLOCKED_INCREMENT, &args![REF_OBJECT_COUNT]);
    e.mem.set_u32(material, NI_MATERIAL_VTABLE);
    let handle = e.mem.u32(name.addr());
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
    }
    let handle = e.mem.u32(name.addr());
    e.mem.set_u32(material + NAME, handle);
    let map = e.mem.u32(MATERIAL_MAP);
    e.call(MAP_SET_AT, &args![map, material + NAME, material]);
    this
}

// Translated from 00a8f9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMaterial::_scalar_deleting_destructor_` (Xbox PDB): `NiMaterial`'s vtable, the name
/// given to `00aa0250` on the material map, the name's count dropped (unless it is the
/// empty string's), the inlined `NiRefObject` destructor (its vtable, `ms_uiObjects`
/// dropped), then the sized delete of 0xc bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn ni_material_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let material = this.addr();
    e.mem.set_u32(material, NI_MATERIAL_VTABLE);
    let map = e.mem.u32(MATERIAL_MAP);
    e.call(MAP_REMOVE_KEY, &args![map, material + NAME]);
    let handle = e.mem.u32(material + NAME);
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
    e.mem.set_u32(material, NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![REF_OBJECT_COUNT]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xcu32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_f910, fn_00a8f910(Ptr, u32) -> Ptr),
        entry!(0x00a8_f950, ni_material_ni_material(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a8_f9b0,
            ni_material_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a8f860, ni_material_get_rtti() -> Ptr),
        entry!(0x00a8_f870, fn_00a8f870()),
        entry!(
            0x00a8_fa10,
            ni_t_fixed_string_map_ni_material_p_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_material_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8f860, &args![]).u32(), NI_MATERIAL_RTTI);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a8f870_calls_slot_0x50_of_every_value() {
        let mut e = Engine::new();
        e.map(MATERIAL_MAP & !0xfff, 0x1000);
        e.map(EMPTY_FIXED_STRING & !0xfff, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x4444);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0; 0x14]);
        e.mem.set_u32(0x0150_0050, 0x0150_1000);
        e.register(0x0150_1000, |_, _| Ret::default());
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, 0x0150_0000);
        e.mem.set_u32(second, 0x0150_0000);
        // GetNext: the first step gives `first` with key 0x1008 and moves on, the second
        // gives null with key 0x2008, the third gives `second` and ends the walk.
        let step = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let counter = step.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let (iterator, key, value) = (a[1], a[2], a[3]);
            let n = counter.get();
            counter.set(n + 1);
            let (next, k, v) = match n {
                0 => (0x77, 0x1008, first),
                1 => (0x78, 0x2008, 0),
                _ => (0, 0x3008, second),
            };
            e.mem.set_u32(iterator, next);
            e.mem.set_u32(key, k);
            e.mem.set_u32(value, v);
            Ret::default()
        });
        let map = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(0xc);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u32(buckets + 4, 0x55);
        e.mem.set_u32(MATERIAL_MAP, map);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f870, &args![]);
        let log = e.call_log.take().unwrap();
        let steps: Vec<_> = log.iter().filter(|c| c.0 == MAP_GET_NEXT).collect();
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].1[0], map);
        let visited: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == 0x0150_1000)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(visited, [first, second]);
        let releases: Vec<_> = log
            .iter()
            .filter(|c| c.0 == INTERLOCKED_DECREMENT)
            .collect();
        assert_eq!(releases, [&(INTERLOCKED_DECREMENT, vec![0x3000])]);

        // No non-empty bucket: nothing is walked.
        e.mem.set_u32(buckets + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f870, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn material_map_destructor_empties_twice_and_frees_on_bit_0() {
        let mut e = Engine::new();
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x6000);
        for flags in [1u32, 0] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a8_fa10, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![
                (MAP_REMOVE_ALL, vec![this]),
                (MAP_REMOVE_ALL, vec![this]),
                (FREE_BLOCK, vec![0x6000]),
            ];
            if flags == 1 {
                expected.push((DEALLOCATE, vec![MEMORY_MANAGER, this]));
            }
            assert_eq!(log[1..], expected[..]);
            assert_eq!(e.mem.u32(this), 0x0109_f72c);
        }
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x2000);
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0300_0008);
        e.mem.set_u32(MATERIAL_MAP, 0x0310_0000);
        for addr in [
            INTERLOCKED_INCREMENT,
            INTERLOCKED_DECREMENT,
            MAP_SET_AT,
            MAP_REMOVE_KEY,
            MAP_REMOVE_ALL,
            FREE_BLOCK,
            DEALLOCATE,
            SIZED_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn map_base_destructor_empties_frees_and_deletes_on_bit_0() {
        let mut e = engine();
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x0320_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_f910, &args![map, 0u32]).u32(), map);
        assert_eq!(e.mem.u32(map), MATERIAL_MAP_BASE_VTABLE);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![map]]);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![0x0320_0000]]);
        assert!(calls(&e, DEALLOCATE).is_empty());
        e.call(0x00a8_f910, &args![map, 1u32]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, map]]);
    }

    #[test]
    fn constructor_takes_the_name_and_enters_the_map() {
        let mut e = engine();
        let material = e.mem.alloc(0xc);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x0330_0008);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_f950, &args![material, name]).u32(), material);
        assert_eq!(e.mem.u32(material), NI_MATERIAL_VTABLE);
        assert_eq!(e.mem.u32(material + 4), 0);
        assert_eq!(e.mem.u32(material + NAME), 0x0330_0008);
        assert_eq!(
            calls(&e, INTERLOCKED_INCREMENT),
            vec![vec![REF_OBJECT_COUNT], vec![0x0330_0000]]
        );
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![0x0310_0000, material + NAME, material]]
        );
        // The empty string is not counted.
        e.mem.set_u32(name, 0x0300_0008);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f950, &args![material, name]);
        assert_eq!(
            calls(&e, INTERLOCKED_INCREMENT),
            vec![vec![REF_OBJECT_COUNT]]
        );
    }

    #[test]
    fn destructor_leaves_the_map_drops_the_name_and_deletes_on_bit_0() {
        let mut e = engine();
        let material = e.mem.alloc(0xc);
        e.mem.set_u32(material + NAME, 0x0330_0008);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_f9b0, &args![material, 0u32]).u32(), material);
        assert_eq!(e.mem.u32(material), NI_REF_OBJECT_VTABLE);
        assert_eq!(
            calls(&e, MAP_REMOVE_KEY),
            vec![vec![0x0310_0000, material + NAME]]
        );
        assert_eq!(
            calls(&e, INTERLOCKED_DECREMENT),
            vec![vec![0x0330_0000], vec![REF_OBJECT_COUNT]]
        );
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.mem.set_u32(material + NAME, 0x0300_0008);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f9b0, &args![material, 1u32]);
        assert_eq!(
            calls(&e, INTERLOCKED_DECREMENT),
            vec![vec![REF_OBJECT_COUNT]]
        );
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![material, 0xc]]);
    }
}
