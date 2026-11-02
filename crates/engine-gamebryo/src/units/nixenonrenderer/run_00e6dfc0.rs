//! `NiXenonRenderer/run_00e6dfc0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and deallocate
/// (`00aa4060`) (`thiscall`, one word each), the Direct3D object (`0126f0d8`), the format of a
/// pixel format (`00e7bd20`, unnamed; `cdecl`) and `IDirect3D9::CheckDeviceFormat` (slot 0x28
/// by the interface's order).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const DIRECT3D_0126F0D8: u32 = 0x0126_f0d8;
const FORMAT_OF_00E7BD20: u32 = 0x00e7_bd20;
const CHECK_DEVICE_FORMAT: u32 = 0x28;

/// The pixel format each sampler entry starts from (0x11 words copied) and the tag stored at
/// its +0x10, by entry (the switch's jump table at `00e6e5d4`, read in Ghidra): entries 6 and 7
/// have none; entries 2 and 22 are tagged with their own index.
fn sampler_source(entry: u32) -> Option<(u32, u32)> {
    let source = match entry {
        0 => (0x011a_a888, 0),
        1 => (0x011a_a8d0, 1),
        2 | 22 => (0x011a_ab58, entry),
        3 => (0x011a_a960, 3),
        4 => (0x011a_a9f0, 4),
        5 => (0x011a_aac8, 5),
        8 => (0x011a_a498, 8),
        9 => (0x011a_a4e0, 9),
        10 => (0x011a_a528, 0xa),
        11 => (0x011a_a408, 0xb),
        12 => (0x011a_ad08, 0xc),
        13 => (0x011a_a450, 0xd),
        14 => (0x011a_a720, 0xe),
        15 => (0x011a_a768, 0xf),
        16 => (0x011a_a6d8, 0x10),
        17 => (0x011a_a648, 0x11),
        18 => (0x011a_a5b8, 0x12),
        19 => (0x011a_a600, 0x13),
        20 => (0x011a_a690, 0x14),
        21 => (0x011a_a570, 0x15),
        _ => return None,
    };
    Some(source)
}

// Translated from 00e6dfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e6dfc0`): fills a sampler block, `thiscall` on the
/// renderer with three words: the block (0x16 pixel-format words), a word it does not read
/// and the resource type. For each entry 0 to 0x15 with a source: a 0x44-byte copy of the
/// source pixel format (from the memory manager; null when that fails, and the stores still go
/// to it), its format (`00e7bd20`) at +0xc and its tag at +0x10. A made copy replaces the
/// entry's old one (freed) when Direct3D accepts its format for the resource type
/// (`CheckDeviceFormat` with the renderer's adapter, device type and adapter format, usage 0);
/// otherwise it is freed and the entry cleared.
pub fn fn_00e6dfc0(e: &mut Engine, this: Ptr, block: Ptr, _unused_2: u32, resource: u32) {
    let renderer = this.addr();
    let slots = block.addr();
    for entry in 0..0x16u32 {
        let Some((source, tag)) = sampler_source(entry) else {
            continue;
        };
        let copy = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x44u32])
            .u32();
        if copy != 0 {
            for word in 0..0x11 {
                let value = e.mem.u32(source + 4 * word);
                e.mem.set_u32(copy + 4 * word, value);
            }
        }
        let format = e.call(FORMAT_OF_00E7BD20, &args![copy]).u32();
        if entry == 2 || entry == 22 {
            e.mem.set_u32(copy.wrapping_add(0xc), format);
            e.mem.set_u32(copy.wrapping_add(0x10), tag);
        } else {
            e.mem.set_u32(copy.wrapping_add(0x10), tag);
            e.mem.set_u32(copy.wrapping_add(0xc), format);
        }
        if copy == 0 {
            continue;
        }
        let slot = slots.wrapping_add(4 * entry);
        let old = e.mem.u32(slot);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, old]);
        let direct3d = e.mem.u32(DIRECT3D_0126F0D8);
        let adapter = e.mem.u32(renderer + 0x5c4);
        let device_type = e.mem.u32(renderer + 0x5c8);
        let adapter_format = e.mem.u32(renderer + 0x5d0);
        let check = e.mem.u32(copy + 0xc);
        let result = e
            .vcall(
                direct3d,
                CHECK_DEVICE_FORMAT,
                &args![adapter, device_type, adapter_format, 0u32, resource, check],
            )
            .u32();
        let kept = if result != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, copy]);
            0
        } else {
            copy
        };
        e.mem.set_u32(slot, kept);
    }
}

/// The imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00e6e630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e6dfc0`): an `NiTPointerMap` step with an `NiPointer`
/// value (the map's `GetNext`), `thiscall` with three words: the address of the iterator (an
/// item: next +0, key +4, value +8), of the key and of the value. The key is copied out; the
/// value word gets an `NiPointer` assignment of the item's value (unless it holds it already:
/// the old one's count down through `InterlockedDecrement` and deleted at zero, the new one
/// stored and counted up when not null). The iterator moves to the item's next one; at the end
/// of a bucket to the first item of the next non-empty bucket after the item's (the map's hash
/// of the key, virtual slot 4, plus one; the bucket count and array read once), or null.
pub fn fn_00e6e630(e: &mut Engine, this: Ptr, iterator: Ptr, key: Ptr, value: Ptr) {
    let map = this.addr();
    let item = e.mem.u32(iterator.addr());
    let item_key = e.mem.u32(item.wrapping_add(4));
    e.mem.set_u32(key.addr(), item_key);
    let held = e.mem.u32(value.addr());
    if held != e.mem.u32(item.wrapping_add(8)) {
        if held != 0
            && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(held, 4, &args![]);
        }
        let item_value = e.mem.u32(item.wrapping_add(8));
        e.mem.set_u32(value.addr(), item_value);
        if item_value != 0 {
            e.call(
                IMPORT_INTERLOCKED_INCREMENT,
                &args![item_value.wrapping_add(4)],
            );
        }
    }
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(iterator.addr(), next);
        return;
    }
    let item_key = e.mem.u32(item.wrapping_add(4));
    let mut bucket = e.vcall(map, 4, &args![item_key]).u32().wrapping_add(1);
    let size = e.mem.u32(map.wrapping_add(4));
    if bucket < size {
        let buckets = e.mem.u32(map.wrapping_add(8));
        while bucket < size {
            let first = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
            if first != 0 {
                e.mem.set_u32(iterator.addr(), first);
                return;
            }
            bucket += 1;
        }
    }
    e.mem.set_u32(iterator.addr(), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e6_dfc0, fn_00e6dfc0(Ptr, Ptr, u32, u32)),
        entry!(0x00e6_e630, fn_00e6e630(Ptr, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e6dfc0_keeps_the_formats_direct3d_accepts() {
        let mut e = Engine::new();
        e.map(0x011a_a000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        for entry in 0..0x16 {
            if let Some((source, _)) = sampler_source(entry) {
                e.mem.set_u32(source, 0x100 + entry);
            }
        }
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        // The format is the copy's first word plus 0x1000.
        e.register(FORMAT_OF_00E7BD20, |e, a| {
            (e.mem.u32(a[0]) + 0x1000).into_ret()
        });
        let slots: Vec<u32> = (0..0x10).map(|i| 0x6100 + 4 * i).collect();
        e.put_vtable(0x6000, &slots);
        // Accepts every format but entry 3's.
        e.register(0x6128, |_, a| u32::from(a[6] == 0x1103).into_ret());
        let direct3d = e.mem.alloc(4);
        e.mem.set_u32(direct3d, 0x6000);
        e.mem.set_u32(DIRECT3D_0126F0D8, direct3d);
        let renderer = e.mem.alloc(0xb80);
        e.mem.set_u32(renderer + 0x5c4, 0);
        e.mem.set_u32(renderer + 0x5c8, 1);
        e.mem.set_u32(renderer + 0x5d0, 0x16);
        let block = e.mem.alloc(0x58);
        e.mem.set_u32(block, 0x7777);
        e.call_log = Some(vec![]);
        e.call(0x00e6_dfc0, &args![renderer, block, 1u32, 3u32]);
        let entry = |e: &Engine, i: u32| e.mem.u32(block + 4 * i);
        let first = entry(&e, 0);
        assert_eq!(e.mem.u32(first), 0x100);
        assert_eq!(e.mem.u32(first + 0xc), 0x1100);
        assert_eq!(e.mem.u32(first + 0x10), 0);
        assert_eq!(e.mem.u32(entry(&e, 2) + 0x10), 2);
        assert_eq!(e.mem.u32(entry(&e, 21) + 0x10), 0x15);
        assert_eq!(entry(&e, 3), 0);
        assert_eq!(entry(&e, 6), 0);
        let log = e.call_log.clone().unwrap();
        let checks: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == 0x6128)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(checks.len(), 20);
        assert_eq!(checks[0], &vec![direct3d, 0, 1, 0x16, 0, 3, 0x1100]);
        // The old entry 0 was freed.
        assert!(log.contains(&(MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, 0x7777])));
    }
}

#[cfg(test)]
mod b051_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e6e630_steps_through_the_buckets() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x7004, |_, _| Ret::default());
        // The hash is the key's low two bits.
        e.register(0x7104, |_, a| (a[1] & 3).into_ret());
        e.put_vtable(0x6000, &[0, 0x7004]);
        e.put_vtable(0x6100, &[0, 0x7104]);
        e.call_log = Some(vec![]);
        let value = |e: &mut Engine| {
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, 0x6000);
            e.mem.set_u32(object + 4, 1);
            object
        };
        let map = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(0x10);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc), e.mem.alloc(0xc)];
        let values = [value(&mut e), value(&mut e)];
        // Bucket 0: item 0 (key 4) then item 1 (key 8); bucket 3: item 2 (key 3).
        e.mem.set_u32(items[0], items[1]);
        e.mem.set_u32(items[0] + 4, 4);
        e.mem.set_u32(items[0] + 8, values[0]);
        e.mem.set_u32(items[1] + 4, 8);
        e.mem.set_u32(items[1] + 8, values[1]);
        e.mem.set_u32(items[2] + 4, 3);
        e.mem.set_u32(buckets, items[0]);
        e.mem.set_u32(buckets + 0xc, items[2]);
        e.mem.set_u32(map, 0x6100);
        e.mem.set_u32(map + 4, 4);
        e.mem.set_u32(map + 8, buckets);
        let iterator = e.mem.alloc(4);
        let key = e.mem.alloc(4);
        let held = e.mem.alloc(4);
        e.mem.set_u32(iterator, items[0]);
        e.call(0x00e6_e630, &args![map, iterator, key, held]);
        assert_eq!(e.mem.u32(key), 4);
        assert_eq!(e.mem.u32(held), values[0]);
        assert_eq!(e.mem.u32(values[0] + 4), 2);
        assert_eq!(e.mem.u32(iterator), items[1]);
        assert!(calls(&e, 0x7104).is_empty());
        e.call(0x00e6_e630, &args![map, iterator, key, held]);
        assert_eq!(e.mem.u32(held), values[1]);
        assert_eq!(e.mem.u32(values[0] + 4), 1);
        assert_eq!(e.mem.u32(iterator), items[2]);
        assert_eq!(calls(&e, 0x7104), vec![vec![map, 8]]);
        // The last item: its value (null) replaces the held one, which is deleted.
        e.mem.set_u32(values[1] + 4, 1);
        e.call(0x00e6_e630, &args![map, iterator, key, held]);
        assert_eq!(e.mem.u32(held), 0);
        assert_eq!(calls(&e, 0x7004), vec![vec![values[1]]]);
        assert_eq!(e.mem.u32(iterator), 0);
    }
}
