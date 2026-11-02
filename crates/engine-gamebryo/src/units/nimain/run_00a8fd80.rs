//! `NiMain/run_00a8fd80` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_13e0;
/// `00a8fce0` (`thiscall`, engine map `NiSingleShaderMaterialLibrary::NiSingleShaderMaterialLibrary`):
/// constructs the object in the block and answers it.
const MATERIAL_LIBRARY_CONSTRUCTOR: u32 = 0x00a8_fce0;
/// `011f5e40`: where `00a8fd80` keeps the object.
const MATERIAL_LIBRARY_SLOT: u32 = 0x011f_5e40;

// Translated from 00a8fd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; run by the start-up
/// `00a89cc0`, no arguments): allocates 0x14 bytes (`00aa13e0`), constructs the object
/// with `00a8fce0` and stores what it answers in `011f5e40`; a failed allocation stores
/// null.
pub fn fn_00a8fd80(e: &mut Engine) {
    let block = e.call(ALLOCATE_BLOCK, &args![0x14u32]).u32();
    if block == 0 {
        e.mem.set_u32(MATERIAL_LIBRARY_SLOT, 0);
        return;
    }
    let object = e.call(MATERIAL_LIBRARY_CONSTRUCTOR, &args![block]).u32();
    e.mem.set_u32(MATERIAL_LIBRARY_SLOT, object);
}

/// `011f5e44`: the global array of material libraries (data pointer; count at `011f5e4c`).
const LIBRARY_LIST: u32 = 0x011f_5e44;
/// `011f4400`: the counted word the library constructor `00a8fe70` increments.
const LIBRARY_COUNT: u32 = 0x011f_4400;
/// Import slot of `InterlockedDecrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The handle of the empty `NiFixedString`, which is never counted.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;

// Translated from 00a8fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): the counterpart of the library
/// constructor `00a8fe70`. Sets the vtable `0109f7f4`, removes `this` from the global array
/// at `011f5e44` (the first entry equal to it, at a non-negative index, takes the last entry
/// and the count at `011f5e4c` goes down), releases the name handle at +8 (unless it is the
/// empty string's), sets the base vtable `0101dce4` and decrements `011f4400`
/// (`InterlockedDecrement`).
pub fn fn_00a8fdb0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0109_f7f4);
    let count = e.mem.u32(LIBRARY_LIST + 8);
    let mut index = 0u32;
    if count > 0 {
        let data = e.mem.u32(LIBRARY_LIST);
        loop {
            if e.mem.u32(data.wrapping_add(4 * index)) == base {
                if index as i32 >= 0 {
                    let last = count.wrapping_sub(1);
                    e.mem.set_u32(LIBRARY_LIST + 8, last);
                    let moved = e.mem.u32(data.wrapping_add(4 * last));
                    e.mem.set_u32(data.wrapping_add(4 * index), moved);
                }
                break;
            }
            index = index.wrapping_add(1);
            if index >= count {
                break;
            }
        }
    }
    let name = e.mem.u32(base + 8);
    if name != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    e.mem.set_u32(base, 0x0101_dce4);
    e.call(INTERLOCKED_DECREMENT, &args![LIBRARY_COUNT]);
}

/// `011f5e34`: the map of materials by name (bucket count at +4, buckets at +8; each node
/// the next node, the name handle and the material).
const MATERIAL_MAP: u32 = 0x011f_5e34;

// Translated from 00a8fe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the address of a name handle): the material of that
/// name in the map at `011f5e34`, or 0: an empty handle (the empty string, or a word of 0 at
/// `handle - 4`) answers 0; otherwise the bucket `handle % bucket count` is walked for the
/// node whose key is the handle.
pub fn fn_00a8fe20(e: &mut Engine, name: Ptr) -> u32 {
    let handle = e.mem.u32(name.addr());
    if handle == e.global::<u32>(EMPTY_FIXED_STRING) || e.mem.u32(handle.wrapping_sub(4)) == 0 {
        return 0;
    }
    let map = e.global::<u32>(MATERIAL_MAP);
    let bucket = handle % e.mem.u32(map + 4);
    let mut node = e.mem.u32(e.mem.u32(map + 8).wrapping_add(4 * bucket));
    while node != 0 {
        if handle == e.mem.u32(node + 4) {
            return e.mem.u32(node + 8);
        }
        node = e.mem.u32(node);
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_fd80, fn_00a8fd80()),
        entry!(0x00a8_fdb0, fn_00a8fdb0(Ptr)),
        entry!(0x00a8_fe20, fn_00a8fe20(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn stores_the_constructed_object() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        let block = e.mem.alloc(0x14);
        e.register_double(ALLOCATE_BLOCK, move |_, a| {
            assert_eq!(a, &[0x14]);
            block.into_ret()
        });
        e.register_double(MATERIAL_LIBRARY_CONSTRUCTOR, move |_, a| {
            assert_eq!(a, &[block]);
            (block + 0x10).into_ret()
        });
        e.call(0x00a8_fd80, &args![]);
        assert_eq!(e.mem.u32(MATERIAL_LIBRARY_SLOT), block + 0x10);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(MATERIAL_LIBRARY_SLOT, 0x1234);
        e.register(ALLOCATE_BLOCK, |_, _| 0u32.into_ret());
        e.call(0x00a8_fd80, &args![]);
        assert_eq!(e.mem.u32(MATERIAL_LIBRARY_SLOT), 0);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a8fdb0_unlists_the_library_and_releases_its_name() {
        let mut e = Engine::new();
        e.map(LIBRARY_LIST & !0xfff, 0x1000);
        e.map(EMPTY_FIXED_STRING & !0xfff, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x4444);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        let data = e.mem.alloc(0x10);
        for (index, entry) in [0x10u32, this, 0x30, 0x40].into_iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, entry);
        }
        e.mem.set_u32(LIBRARY_LIST, data);
        e.mem.set_u32(LIBRARY_LIST + 8, 4);
        e.mem.set_u32(this + 8, 0x1008);
        e.call_log = Some(vec![]);
        e.call(0x00a8_fdb0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (INTERLOCKED_DECREMENT, vec![0x1000]),
                (INTERLOCKED_DECREMENT, vec![LIBRARY_COUNT]),
            ]
        );
        assert_eq!(e.mem.u32(LIBRARY_LIST + 8), 3);
        assert_eq!(e.mem.u32(data + 4), 0x40);
        assert_eq!(e.mem.u32(this), 0x0101_dce4);

        // Not listed, empty name: the list stays, only the class count goes down.
        e.mem.set_u32(this + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00a8_fdb0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(INTERLOCKED_DECREMENT, vec![LIBRARY_COUNT])]);
        assert_eq!(e.mem.u32(LIBRARY_LIST + 8), 3);
    }
}

#[cfg(test)]
mod lookup_tests {
    use super::*;

    #[test]
    fn finds_the_material_of_a_name() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0109_b228);
        let map = e.mem.alloc(0xc);
        let buckets = e.mem.alloc(0xc);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u32(MATERIAL_MAP, map);
        let text = e.mem.alloc(0x20);
        let handle = text + 0x10;
        e.mem.set_u32(handle - 4, 5);
        let other = e.mem.alloc(0xc);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(other, node);
        e.mem.set_u32(other + 4, handle + 3);
        e.mem.set_u32(node + 4, handle);
        e.mem.set_u32(node + 8, 0x7777);
        e.mem.set_u32(buckets + 4 * (handle % 3), other);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, handle);
        assert_eq!(e.call(0x00a8_fe20, &args![name]).u32(), 0x7777);
        e.mem.set_u32(node + 4, handle + 6);
        assert_eq!(e.call(0x00a8_fe20, &args![name]).u32(), 0);
        e.mem.set_u32(buckets + 4 * (handle % 3), 0);
        assert_eq!(e.call(0x00a8_fe20, &args![name]).u32(), 0);
        e.mem.set_u32(handle - 4, 0);
        assert_eq!(e.call(0x00a8_fe20, &args![name]).u32(), 0);
        e.mem.set_u32(name, 0x0109_b228);
        assert_eq!(e.call(0x00a8_fe20, &args![name]).u32(), 0);
    }
}
