//! `NiMain/run_00a819e0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiSkinPartition::Partition` vtable (its RTTI locator names
/// `.?AVPartition@NiSkinPartition@@`).
const PARTITION_VTABLE: u32 = 0x0109_e470;
/// `NiFree` (`00aa10f0`, cdecl, the block).
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00a81ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSkinPartition::Partition` constructor (Ghidra made no function of
/// it; `NiSkinPartition::LoadBinary` passes it to the vector constructor):
/// the vtable, the six array pointers (+4 to +0x18), the strip count
/// (+0x22) and the renderer data (+0x28) cleared; the other counts are left
/// as they are. Returns `this`.
pub fn fn_00a81ba0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, PARTITION_VTABLE);
    for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x18] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u16(base + 0x22, 0);
    e.mem.set_u32(base + 0x28, 0);
    this
}

// Translated from 00a81bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSkinPartition::Partition` destructor body: the vtable, then the
/// six arrays (+4 to +0x18) freed with `NiFree`.
pub fn fn_00a81bd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let first = e.mem.u32(base + 4);
    e.mem.set_u32(base, PARTITION_VTABLE);
    e.call(NI_FREE, &args![first]);
    for offset in [8u32, 0xc, 0x10, 0x14, 0x18] {
        let block = e.mem.u32(base + offset);
        e.call(NI_FREE, &args![block]);
    }
}

/// `011f5b04`: the pointer to the object these functions look after (its member at +0x190
/// is what `00714900` and `00714960`, unit `fallout/interface/interfacemanager.cpp`, work
/// on; its class is not confirmed here).
const HELD_OBJECT: u32 = 0x011f_5b04;
/// `00714900` (no arguments) and `00714960` (`cdecl`, one word), engine map unit
/// `fallout/interface/interfacemanager.cpp`.
const HELD_OBJECT_FINISH: u32 = 0x0071_4900;
const HELD_OBJECT_START: u32 = 0x0071_4960;
/// The byte of the held object that `00a81a20` sets and `00a81a80` clears.
const BUSY: u32 = 0x1b0;

// Translated from 00a819e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): when the object at `011f5b04` exists, runs its
/// virtual slot `+0x9c`, then, when the pointer is still set, its slot `+0` with 1 (the
/// deleting destructor's slot) and clears the pointer.
pub fn fn_00a819e0(e: &mut Engine) {
    let object = e.mem.u32(HELD_OBJECT);
    if object == 0 {
        return;
    }
    e.vcall(object, 0x9c, &args![]);
    let object = e.mem.u32(HELD_OBJECT);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.mem.set_u32(HELD_OBJECT, 0);
}

// Translated from 00a81a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): when the object at `011f5b04` exists, sets its
/// byte at +0x1b0 and calls `00714960` with -1.
pub fn fn_00a81a20(e: &mut Engine) {
    let object = e.mem.u32(HELD_OBJECT);
    if object == 0 {
        return;
    }
    e.mem.set_u8(object + BUSY, 1);
    e.call(HELD_OBJECT_START, &args![0xffff_ffffu32]);
}

// Translated from 00a81a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, an index; `RET 4`): the number of entries in the
/// ring of record `index` (records of 16 bytes from +8: the size at +4, the head at +8, the
/// tail at +0xc): 0 when the head is -1, `tail - head` when the head is below the tail,
/// otherwise `size - head + tail` (wrapping).
pub fn fn_00a81a40(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let record = (index << 4).wrapping_add(this.addr()).wrapping_add(8);
    let head = e.mem.u32(record + 8);
    if head == 0xffff_ffff {
        return 0;
    }
    let tail = e.mem.u32(record + 0xc);
    if head < tail {
        return tail.wrapping_sub(head);
    }
    e.mem.u32(record + 4).wrapping_sub(head).wrapping_add(tail)
}

// Translated from 00a81a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): when the object at `011f5b04` exists and any of
/// its four rings holds entries (rings 0 and 1 counted inline, 2 and 3 by
/// [`fn_00a81a40`]), runs its virtual slot `+0xa0` with `(1, object + 0x19c, 0, 0)`;
/// otherwise calls `00714900`. Either way the byte at +0x1b0 of the object the pointer then
/// holds is cleared.
pub fn fn_00a81a80(e: &mut Engine) {
    let object = e.mem.u32(HELD_OBJECT);
    if object == 0 {
        return;
    }
    let mut pending = false;
    for index in [0u32, 1] {
        let record = object + 8 + 0x10 * index;
        let head = e.mem.u32(record + 8);
        if head == 0xffff_ffff {
            continue;
        }
        let tail = e.mem.u32(record + 0xc);
        let count = if head < tail {
            tail.wrapping_sub(head)
        } else {
            e.mem.u32(record + 4).wrapping_sub(head).wrapping_add(tail)
        };
        if count != 0 {
            pending = true;
            break;
        }
    }
    if !pending {
        pending =
            fn_00a81a40(e, Ptr::new(object), 2) != 0 || fn_00a81a40(e, Ptr::new(object), 3) != 0;
    }
    if pending {
        e.vcall(object, 0xa0, &args![1u32, object + 0x19c, 0u32, 0u32]);
    } else {
        e.call(HELD_OBJECT_FINISH, &args![]);
    }
    let object = e.mem.u32(HELD_OBJECT);
    e.mem.set_u8(object.wrapping_add(BUSY), 0);
}

// Translated from 00a81b30 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, two pointers): a
/// sort comparison on the float at +4 of each, highest first: -1 when the first is greater,
/// 1 when it is smaller, 0 when they are equal or either is a NaN. The floats are copied
/// into the argument words first (`FLD`/`FSTP`, quieting a signalling NaN) and compared with
/// `FCOM`.
pub fn fn_00a81b30(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let a = x87_load32(x87_quiet_bits(e.mem.u32(first.addr() + 4)));
    let b = x87_load32(x87_quiet_bits(e.mem.u32(second.addr() + 4)));
    if b < a {
        -1
    } else if b > a {
        1
    } else {
        0
    }
}

// Translated from 00a81b80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; `cdecl`, two pointers): a
/// sort comparison on the 16-bit word at +0 of each, lowest first: -1 when the second is
/// greater, 1 when it is smaller, 0 when equal.
pub fn fn_00a81b80(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let a = e.mem.u16(first.addr());
    let b = e.mem.u16(second.addr());
    match b.cmp(&a) {
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a81ba0, fn_00a81ba0(Ptr) -> Ptr),
        entry!(0x00a81bd0, fn_00a81bd0(Ptr)),
        entry!(0x00a8_19e0, fn_00a819e0()),
        entry!(0x00a8_1a20, fn_00a81a20()),
        entry!(0x00a8_1a40, fn_00a81a40(Ptr, u32) -> u32),
        entry!(0x00a8_1a80, fn_00a81a80()),
        entry!(0x00a8_1b30, fn_00a81b30(Ptr, Ptr) -> i32),
        entry!(0x00a8_1b80, fn_00a81b80(Ptr, Ptr) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_constructor_and_destructor() {
        let mut e = Engine::new();
        e.register(NI_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xaa; 0x2c]);
        assert_eq!(e.call(0x00a8_1ba0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PARTITION_VTABLE);
        assert_eq!(e.mem.bytes(this + 4, 0x18), vec![0; 0x18]);
        assert_eq!(e.mem.u16(this + 0x1c), 0xaaaa);
        assert_eq!(e.mem.u16(this + 0x22), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        for (i, offset) in [4u32, 8, 0xc, 0x10, 0x14, 0x18].iter().enumerate() {
            e.mem.set_u32(this + offset, 0x100 + i as u32);
        }
        e.mem.set_u32(this, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_1bd0, &args![this]);
        assert_eq!(e.mem.u32(this), PARTITION_VTABLE);
        let freed: Vec<_> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == NI_FREE)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(freed, vec![0x100, 0x101, 0x102, 0x103, 0x104, 0x105]);
    }
}

#[cfg(test)]
mod held_object_tests {
    use super::*;

    const VTABLE: u32 = 0x0300_0000;

    /// An engine with the held object at `011f5b04` (vtable slots `+0`, `+0x9c` and `+0xa0`
    /// recording doubles) and doubles for the two interface functions.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.map(VTABLE, 0x100);
        for slot in [0u32, 0x9c, 0xa0] {
            e.mem.set_u32(VTABLE + slot, 0x0300_1000 + slot);
            e.register(0x0300_1000 + slot, |_, _| Ret::default());
        }
        e.register(HELD_OBJECT_FINISH, |_, _| Ret::default());
        e.register(HELD_OBJECT_START, |_, _| Ret::default());
        let object = e.mem.alloc(0x1c0);
        e.mem.set_u32(object, VTABLE);
        for index in 0..4u32 {
            e.mem.set_u32(object + 0x10 + 0x10 * index, 0xffff_ffff);
        }
        e.mem.set_u32(HELD_OBJECT, object);
        e.call_log = Some(vec![]);
        (e, object)
    }

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .replace(vec![])
            .unwrap()
            .into_iter()
            .skip(1)
            .collect()
    }

    #[test]
    fn release_runs_both_slots_and_clears_the_pointer() {
        let (mut e, object) = engine();
        e.call(0x00a8_19e0, &args![]);
        assert_eq!(
            calls(&mut e),
            vec![(0x0300_109c, vec![object]), (0x0300_1000, vec![object, 1])]
        );
        assert_eq!(e.mem.u32(HELD_OBJECT), 0);
        e.call(0x00a8_19e0, &args![]);
        assert!(calls(&mut e).is_empty());
        // A slot that clears the pointer skips the destructor slot.
        let (mut e, object) = engine();
        e.register(0x0300_109c, |e, _| {
            e.mem.set_u32(HELD_OBJECT, 0);
            Ret::default()
        });
        e.call(0x00a8_19e0, &args![]);
        assert_eq!(calls(&mut e), vec![(0x0300_109c, vec![object])]);
    }

    #[test]
    fn start_sets_the_busy_byte() {
        let (mut e, object) = engine();
        e.call(0x00a8_1a20, &args![]);
        assert_eq!(e.mem.u8(object + BUSY), 1);
        assert_eq!(calls(&mut e), vec![(HELD_OBJECT_START, vec![0xffff_ffff])]);
        e.mem.set_u32(HELD_OBJECT, 0);
        e.call(0x00a8_1a20, &args![]);
        assert!(calls(&mut e).is_empty());
    }

    #[test]
    fn ring_counts() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        // Record 2: size at +0x2c, head +0x30, tail +0x34.
        e.mem.set_u32(this + 0x2c, 10);
        e.mem.set_u32(this + 0x30, 0xffff_ffff);
        assert_eq!(e.call(0x00a8_1a40, &args![this, 2u32]).u32(), 0);
        e.mem.set_u32(this + 0x30, 3);
        e.mem.set_u32(this + 0x34, 7);
        assert_eq!(e.call(0x00a8_1a40, &args![this, 2u32]).u32(), 4);
        e.mem.set_u32(this + 0x30, 7);
        e.mem.set_u32(this + 0x34, 3);
        assert_eq!(e.call(0x00a8_1a40, &args![this, 2u32]).u32(), 6);
        e.mem.set_u32(this + 0x34, 7);
        assert_eq!(e.call(0x00a8_1a40, &args![this, 2u32]).u32(), 10);
    }

    #[test]
    fn finish_flushes_pending_rings_or_finishes() {
        // All rings empty or unused: the interface function.
        let (mut e, object) = engine();
        e.mem.set_u8(object + BUSY, 1);
        e.mem.set_u32(object + 0x20, 4);
        e.mem.set_u32(object + 0x24, 4);
        e.mem.set_u32(object + 0x1c, 0);
        e.call(0x00a8_1a80, &args![]);
        assert_eq!(calls(&mut e), vec![(HELD_OBJECT_FINISH, vec![])]);
        assert_eq!(e.mem.u8(object + BUSY), 0);
        // Any ring with entries: the object's slot +0xa0.
        for head_at in [0x10u32, 0x20, 0x30, 0x40] {
            let (mut e, object) = engine();
            e.mem.set_u8(object + BUSY, 1);
            e.mem.set_u32(object + head_at, 1);
            e.mem.set_u32(object + head_at + 4, 2);
            e.call(0x00a8_1a80, &args![]);
            assert_eq!(
                calls(&mut e),
                vec![(0x0300_10a0, vec![object, 1, object + 0x19c, 0, 0])]
            );
            assert_eq!(e.mem.u8(object + BUSY), 0);
        }
        // No object: nothing.
        let (mut e, _) = engine();
        e.mem.set_u32(HELD_OBJECT, 0);
        e.call(0x00a8_1a80, &args![]);
        assert!(calls(&mut e).is_empty());
    }

    #[test]
    fn comparisons_sort_highest_first() {
        let mut e = Engine::new();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        for (x, y, answer) in [
            (2.0f32, 1.0f32, -1i32),
            (1.0, 2.0, 1),
            (1.0, 1.0, 0),
            (f32::NAN, 1.0, 0),
            (1.0, f32::NAN, 0),
        ] {
            e.mem.set_f32(a + 4, x);
            e.mem.set_f32(b + 4, y);
            assert_eq!(e.call(0x00a8_1b30, &args![a, b]).i32(), answer);
        }
        for (x, y, answer) in [
            (2u16, 1u16, 1i32),
            (1, 2, -1),
            (0x8000, 0x8000, 0),
            (0xffff, 0, 1),
        ] {
            e.mem.set_u16(a, x);
            e.mem.set_u16(b, y);
            assert_eq!(e.call(0x00a8_1b80, &args![a, b]).i32(), answer);
        }
    }
}
