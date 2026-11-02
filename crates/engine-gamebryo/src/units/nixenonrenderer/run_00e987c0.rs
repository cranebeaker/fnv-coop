//! `NiXenonRenderer/run_00e987c0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The device's vtable slot that creates a vertex buffer (+0x68), COM
/// `stdcall (device, length, usage, format, pool, &buffer, handle)`.
const DEVICE_SLOT_CREATE_VERTEX_BUFFER: u32 = 0x68;
/// The buffer's vtable slot that releases it (+8, COM `Release`).
const BUFFER_SLOT_RELEASE: u32 = 0x8;
/// The lock flags written to the out word: append (`0x1000`) and discard (`0x2000`).
const LOCK_APPEND: u32 = 0x1000;
const LOCK_DISCARD: u32 = 0x2000;

// Translated from 00e987c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A dynamic vertex buffer's allocation (unnamed in the engine map; the
/// class is not confirmed), thiscall `(size, &offset, &lock flags,
/// discard, write only)` (the last two are bytes). The object holds the
/// vertex format (+4), the capacity (+8), the device (+0xC), the buffer
/// (+0x10) and the used size (+0x14). When the capacity is below `size`,
/// releases the buffer and grows the capacity to `size` (rounded down to
/// 16 plus 0x20 unless already a multiple of 16). Without a buffer, creates
/// one (usage 0x218 for write only, else 0x208; pool 0) and, on success,
/// clears the used size. Then, unless `discard` is set and when `size`
/// still fits, hands out the used size as the offset with the append
/// flag; otherwise offset 0 with the discard flag and the used size
/// `size`. An offset of 0 always gets the discard flag. Answers the buffer.
pub fn fn_00e987c0(
    e: &mut Engine,
    this: Ptr,
    size: u32,
    offset_out: u32,
    flags_out: u32,
    discard: u8,
    write_only: u8,
) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + 8) < size {
        let buffer = e.mem.u32(base + 0x10);
        if buffer != 0 {
            e.vcall(buffer, BUFFER_SLOT_RELEASE, &args![]);
        }
        e.mem.set_u32(base + 0x10, 0);
        let capacity = if size & 0xf != 0 {
            (size & 0xffff_fff0).wrapping_add(0x20)
        } else {
            size
        };
        e.mem.set_u32(base + 8, capacity);
    }
    if e.mem.u32(base + 0x10) == 0 {
        let usage = if write_only != 0 { 0x218u32 } else { 0x208 };
        let device = e.mem.u32(base + 0xc);
        let format = e.mem.u32(base + 4);
        let capacity = e.mem.u32(base + 8);
        let result = e
            .vcall(
                device,
                DEVICE_SLOT_CREATE_VERTEX_BUFFER,
                &args![capacity, usage, format, 0u32, base + 0x10, 0u32],
            )
            .i32();
        if result >= 0 {
            e.mem.set_u32(base + 0x14, 0);
        }
    }
    let used = e.mem.u32(base + 0x14);
    if discard == 0 && e.mem.u32(base + 8).wrapping_sub(used) >= size {
        e.mem.set_u32(offset_out, used);
        e.mem.set_u32(base + 0x14, used.wrapping_add(size));
        e.mem.set_u32(flags_out, LOCK_APPEND);
    } else {
        e.mem.set_u32(offset_out, 0);
        e.mem.set_u32(base + 0x14, size);
        e.mem.set_u32(flags_out, LOCK_DISCARD);
    }
    if e.mem.u32(offset_out) == 0 {
        e.mem.set_u32(flags_out, LOCK_DISCARD);
    }
    e.mem.u32(base + 0x10)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e987c0,
        fn_00e987c0(Ptr, u32, u32, u32, u8, u8) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const CREATE_DOUBLE: u32 = 0xdead_5000;
    const RELEASE_DOUBLE: u32 = 0xdead_5010;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn grows_creates_and_hands_out_ranges() {
        let mut e = Engine::new();
        let device_vtable = e.mem.alloc(0x70);
        e.mem.set_u32(
            device_vtable + DEVICE_SLOT_CREATE_VERTEX_BUFFER,
            CREATE_DOUBLE,
        );
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, device_vtable);
        let buffer_vtable = e.mem.alloc(0x10);
        e.mem
            .set_u32(buffer_vtable + BUFFER_SLOT_RELEASE, RELEASE_DOUBLE);
        let old_buffer = e.mem.alloc(4);
        e.mem.set_u32(old_buffer, buffer_vtable);
        e.register(CREATE_DOUBLE, |e, a| {
            e.mem.set_u32(a[5], 0x7700);
            Ret::default()
        });
        e.register(RELEASE_DOUBLE, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 4, 0x52);
        e.mem.set_u32(this + 8, 0x10);
        e.mem.set_u32(this + 0xc, device);
        e.mem.set_u32(this + 0x10, old_buffer);
        e.mem.set_u32(this + 0x14, 0x10);
        let out = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let answer = e.call(0x00e9_87c0, &args![this, 0x31u32, out, out + 4, 0u32, 1u32]);
        assert_eq!(answer.u32(), 0x7700);
        assert_eq!(calls(&e, RELEASE_DOUBLE), vec![vec![old_buffer]]);
        assert_eq!(e.mem.u32(this + 8), 0x50);
        assert_eq!(
            calls(&e, CREATE_DOUBLE),
            vec![vec![device, 0x50, 0x218, 0x52, 0, this + 0x10, 0]]
        );
        // A fresh buffer starts at offset 0: discard.
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u32(out + 4), LOCK_DISCARD);
        assert_eq!(e.mem.u32(this + 0x14), 0x31);
        // The next 0x10 bytes fit after the used ones: append.
        e.call_log = Some(vec![]);
        e.call(0x00e9_87c0, &args![this, 0x10u32, out, out + 4, 0u32, 0u32]);
        assert!(calls(&e, CREATE_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(out), 0x31);
        assert_eq!(e.mem.u32(out + 4), LOCK_APPEND);
        assert_eq!(e.mem.u32(this + 0x14), 0x41);
        // A discard request starts over.
        e.call(0x00e9_87c0, &args![this, 0x8u32, out, out + 4, 1u32, 0u32]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u32(out + 4), LOCK_DISCARD);
        assert_eq!(e.mem.u32(this + 0x14), 8);
        // A failed creation keeps the used size.
        e.mem.set_u32(this + 0x10, 0);
        e.register(CREATE_DOUBLE, |_, _| Ret {
            eax: 0x8000_0000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00e9_87c0, &args![this, 0x10u32, out, out + 4, 0u32, 0u32]);
        assert_eq!(answer.u32(), 0);
        assert_eq!(calls(&e, CREATE_DOUBLE)[0][2], 0x208);
        assert_eq!(e.mem.u32(out), 8);
        assert_eq!(e.mem.u32(this + 0x14), 0x18);
    }
}
