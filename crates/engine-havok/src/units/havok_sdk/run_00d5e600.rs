//! `Havok SDK/run_00d5e600` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d5e850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d5_e7c0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d5e850(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d5_e7c0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The `TlsGetValue` import slot (`kernel32`).
const TLS_GET_VALUE: u32 = 0x00fd_f044;
/// The TLS index of the profiling record stream (a word in `.data`).
const PROFILE_SLOT_0126835C: u32 = 0x0126_835c;
/// The record labels: `"TtrcShpCollect"` and the end label shared with other
/// Havok profiling scopes.
const SHAPE_COLLECT_LABEL: u32 = 0x010c_e1f8;
const END_LABEL: u32 = 0x010c_41f8;

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low_00d5e600() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record
/// stream (`TlsGetValue` of the index at `0126835c`) while its cursor
/// (`+4`) is below its limit (`+0xc`); a record is 12 bytes.
fn profile_record_00d5e600(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT_0126835C);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low_00d5e600());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

// Translated from 00d5e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall, three words (`RET 0xc`). Writes a profiling
/// record, walks the child shapes with the iterator embedded at
/// `this + 0x14` (virtual slots `+8` first key, `+0xc` next key, `+0x14`
/// child for a key and a 0x200-byte scratch buffer; `-1` ends) and for each
/// child builds a 16-byte body record on the stack, `{child, key,
/// [parent + 8], parent}`, then calls the child's slot `+0x24` with
/// `(first, &record, third)`. Ends with the end-label record. `RDTSC` is
/// the host clock.
pub fn fn_00d5e600(e: &mut Engine, this: Ptr, first: u32, parent: Ptr, third: u32) {
    profile_record_00d5e600(e, SHAPE_COLLECT_LABEL);
    let iterator = this.addr() + 0x14;
    e.with_stack(0x214, |e, frame| {
        let record = frame.addr() + 0x10;
        let buffer = frame.addr() + 0x20;
        let mut key = e.vcall(iterator, 8, &args![]).u32();
        while key != u32::MAX {
            let child = e.vcall(iterator, 0x14, &args![key, buffer]).u32();
            let motion = e.mem.u32(parent.addr() + 8);
            e.mem.set_u32(record + 8, motion);
            e.mem.set_u32(record + 0xc, parent.addr());
            e.mem.set_u32(record, child);
            e.mem.set_u32(record + 4, key);
            e.vcall(child, 0x24, &args![first, record, third]);
            key = e.vcall(iterator, 0xc, &args![key]).u32();
        }
    });
    profile_record_00d5e600(e, END_LABEL);
}

// Translated from 00d5e840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x14` (`SUB ECX,0x14`) and jumps to `00d5e850`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d5e840(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d5_e850, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;

// Translated from 00d5e6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 8`: two stack words `source` and `count`; descriptions, not
/// confirmed): fills the pointer array at `this + 0x44` (data, size `+0x48`, capacity word `+0x4c`)
/// from the `count` pointers at `source`: reserves `max(count, 2 * capacity)` pointers through
/// `00c908c0` when the capacity is below `count` (signed), sets the size to `count`, copies the
/// float at `*source[0] + 0x10` (through `FLD`/`FSTP`; read even when `count` is 0) to
/// `this + 0x10`, and copies each pointer, calling `00c90510` (thiscall on it, no stack words) as
/// it goes.
pub fn fn_00d5e6e0(e: &mut Engine, this: Ptr, source: u32, count: i32) {
    let me = this.addr();
    let array = me + 0x44;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 4u32]);
    }
    e.mem.set_i32(array + 4, count);
    let first = e.mem.u32(source);
    let radius = x87_quiet_bits(e.mem.u32(first + 0x10));
    e.mem.set_u32(me + 0x10, radius);
    if count > 0 {
        for index in 0..count as u32 {
            let data = e.mem.u32(array);
            let pointer = e.mem.u32(source.wrapping_add(index * 4));
            e.mem.set_u32(data.wrapping_add(index * 4), pointer);
            let pointer = e.mem.u32(source.wrapping_add(index * 4));
            e.call(0x00c9_0510, &args![pointer]);
        }
    }
}

// Translated from 00d5e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (thiscall, `RET 8`: two stack words `source` and `count`; no map name, class
/// unconfirmed): `+0x10` = 0.0, the `u16` at `+6` = 1, `+8` = 0, `+0xc` = 0x16, the vtable words
/// `+0x14` = `0x010c6fb0` and then `0x010d_10a4` and `+0` = `0x010d_10c4`, an empty pointer array
/// at `+0x44` (data 0, size 0, capacity word `0x80000000`), `+0x18` = 1.0; then `00d5e6e0` with
/// `(source, count)` and `00d5e210` with 1. Returns `this`.
pub fn fn_00d5e750(e: &mut Engine, this: Ptr, source: u32, count: i32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, 0x16);
    e.mem.set_u32(me + 0x14, 0x010c_6fb0);
    e.mem.set_u32(me, 0x010d_10c4);
    e.mem.set_u32(me + 0x14, 0x010d_10a4);
    e.mem.set_u32(me + 0x44, 0);
    e.mem.set_u32(me + 0x48, 0);
    e.mem.set_u32(me + 0x4c, 0x8000_0000);
    e.mem.set_u32(me + 0x18, 1.0f32.to_bits());
    fn_00d5e6e0(e, this, source, count);
    e.call(0x00d5_e210, &args![this, 1u32]);
    this
}

// Translated from 00d5e7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor (`__fastcall`, no stack words; no map name, class unconfirmed): writes the vtable
/// words `+0` = `0x010d_10c4` and `+0x14` = `0x010d_10a4`, calls `00c905b0` on each pointer of the
/// array at `+0x44` (count at `+0x48`, each element read again after the previous call), clears the
/// count, frees the data through slot `0x10` of the router's allocator (`4 * (capacity &
/// 0x3fffffff)` bytes) when the sign bit of the capacity word at `+0x4c` is clear, clears the data
/// pointer, makes the capacity word `0x80000000` and writes the base vtable words (`+0x14` =
/// `0x010c6fb0`, `+0` = `0x0102_e388`).
pub fn fn_00d5e7c0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let initial = e.mem.i32(me + 0x48);
    e.mem.set_u32(me, 0x010d_10c4);
    e.mem.set_u32(me + 0x14, 0x010d_10a4);
    if initial > 0 {
        let mut index = 0i32;
        loop {
            let data = e.mem.u32(me + 0x44);
            let item = e.mem.u32(data.wrapping_add(index as u32 * 4));
            e.call(0x00c9_05b0, &args![item]);
            index += 1;
            if index >= e.mem.i32(me + 0x48) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x4c);
    e.mem.set_u32(me + 0x48, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0x44);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0x44, 0);
    e.mem.set_u32(me + 0x4c, 0x8000_0000);
    e.mem.set_u32(me + 0x14, 0x010c_6fb0);
    e.mem.set_u32(me, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d5e600, fn_00d5e600(Ptr, u32, Ptr, u32)),
        entry!(0x00d5_e850, deleting_destructor_00d5e850(Ptr, u8) -> Ptr),
        entry!(0x00d5_e840, fn_00d5e840(Ptr, u32) -> u32),
        entry!(0x00d5_e6e0, fn_00d5e6e0(Ptr, u32, i32)),
        entry!(0x00d5_e750, fn_00d5e750(Ptr, u32, i32) -> Ptr),
        entry!(0x00d5_e7c0, fn_00d5e7c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d5e850_frees_only_when_bit_zero_is_set() {
        check(0x00d5_e850, 0x00d5_e7c0);
    }

    #[test]
    fn fn_00d5e600_hands_each_child_a_body_record() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let stream = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x30);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x30);
        e.set_global(PROFILE_SLOT_0126835C, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        let mut slots = vec![0u32; 8];
        slots[2] = 0x0090_5000;
        slots[3] = 0x0090_5100;
        slots[5] = 0x0090_5200;
        e.put_vtable(0x0090_2000, &slots);
        let mut child_slots = vec![0u32; 10];
        child_slots[9] = 0x0090_5300;
        e.put_vtable(0x0090_3000, &child_slots);
        let child = e.mem.alloc(8);
        e.mem.set_u32(child, 0x0090_3000);
        // Keys 7 then 9, both with the same child.
        e.register(0x0090_5000, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.register(0x0090_5100, |_, a| Ret {
            eax: if a[1] == 7 { 9 } else { u32::MAX },
            ..Ret::default()
        });
        e.register_double(0x0090_5200, move |_, _| Ret {
            eax: child,
            ..Ret::default()
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen_log = std::rc::Rc::clone(&seen);
        e.register_double(0x0090_5300, move |e, a| {
            let record: Vec<u32> = (0..4).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            seen_log.borrow_mut().push((a.to_vec(), record));
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, 0x0090_2000);
        let parent = e.mem.alloc(0x20);
        e.mem.set_u32(parent + 8, 0xcafe);
        e.call(
            0x00d5_e600,
            &args![
                Ptr::<()>::new(this),
                0x11u32,
                Ptr::<()>::new(parent),
                0x33u32
            ],
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].1, vec![child, 7, 0xcafe, parent]);
        assert_eq!(seen[1].1, vec![child, 9, 0xcafe, parent]);
        assert_eq!(seen[0].0[1], 0x11);
        assert_eq!(seen[0].0[3], 0x33);
        // The scope label then the end label were recorded.
        assert_eq!(e.mem.u32(records), SHAPE_COLLECT_LABEL);
        assert_eq!(e.mem.u32(records + 12), END_LABEL);
        assert_eq!(e.mem.u32(stream + 4), records + 24);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00d5e840() {
        assert_eq!(check(0x00d5_e840, 0x00d5_e850, 0x14), 0xdead_beef);
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    fn router(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, 0x0900_0131);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x0900_0131, |_, _| Ret::default());
        allocator
    }

    #[test]
    fn fn_00d5e6e0_copies_the_pointers_and_adds_a_reference_to_each() {
        let mut e = Engine::new();
        let allocator = router(&mut e);
        e.register(0x00c9_0510, |_, _| Ret::default());
        // A reserve that gives the array room for `wanted` pointers.
        e.register(ARRAY_RESERVE, |e, a| {
            let block = e.mem.alloc(a[2] * 4 + 4);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let source = e.mem.alloc(0x10);
        let first = e.mem.alloc(0x20);
        e.mem.set_f32(first + 0x10, 2.5);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(source, first);
        e.mem.set_u32(source + 4, second);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x00d5_e6e0, &args![this, source, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (MEMORY_ROUTER, vec![]),
                (ARRAY_RESERVE, vec![allocator, this + 0x44, 2, 4]),
                (0x00c9_0510, vec![first]),
                (0x00c9_0510, vec![second]),
            ]
        );
        assert_eq!(e.mem.f32(this + 0x10), 2.5);
        assert_eq!(e.mem.u32(this + 0x48), 2);
        let data = e.mem.u32(this + 0x44);
        assert_eq!((e.mem.u32(data), e.mem.u32(data + 4)), (first, second));
    }

    #[test]
    fn fn_00d5e750_constructs_and_collects_the_bounds() {
        let mut e = Engine::new();
        router(&mut e);
        e.register(0x00d5_e210, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.mem.set_f32(object + 0x10, 4.5);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, object);
        let this = e.mem.alloc(0x60);
        for word in 0..0x18 {
            e.mem.set_u32(this + word * 4, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        let result = e.call(0x00d5_e750, &args![this, source, 0u32]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x010d_10c4);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x16);
        // `+0x10` was zeroed and then given the radius of the first child by `00d5e6e0`.
        assert_eq!(e.mem.f32(this + 0x10), 4.5);
        assert_eq!(e.mem.u32(this + 0x14), 0x010d_10a4);
        assert_eq!(e.mem.f32(this + 0x18), 1.0);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x4c), 0x8000_0000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(MEMORY_ROUTER, vec![]), (0x00d5_e210, vec![this, 1])]
        );
    }

    #[test]
    fn fn_00d5e7c0_releases_the_pointers_and_frees_the_array() {
        let mut e = Engine::new();
        let allocator = router(&mut e);
        e.register(0x00c9_05b0, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x1111);
        e.mem.set_u32(data + 4, 0x2222);
        e.mem.set_u32(this + 0x44, data);
        e.mem.set_u32(this + 0x48, 2);
        e.mem.set_u32(this + 0x4c, 0x4000_0003);
        e.call_log = Some(vec![]);
        e.call(0x00d5_e7c0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x00c9_05b0, vec![0x1111]),
                (0x00c9_05b0, vec![0x2222]),
                (MEMORY_ROUTER, vec![]),
                (0x0900_0131, vec![allocator, data, 12]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x4c), 0x8000_0000);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 0x14), 0x010c_6fb0);
    }
}
