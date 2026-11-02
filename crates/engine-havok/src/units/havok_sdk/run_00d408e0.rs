//! `Havok SDK/run_00d408e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d408e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `record`): fills four function words (`+0` = `0x00d401d0`, `+4` =
/// `0x00d406c0`, `+8` = `0x00dac450`, `+0xc` = `0x00daf8e0`) and the bytes at `+0x10` and `+0x11`
/// with 0.
pub fn fn_00d408e0(e: &mut Engine, record: Ptr) {
    let r = record.addr();
    e.mem.set_u32(r, 0x00d4_01d0);
    e.mem.set_u32(r + 4, 0x00d4_06c0);
    e.mem.set_u32(r + 8, 0x00da_c450);
    e.mem.set_u32(r + 0xc, 0x00da_f8e0);
    e.mem.set_u8(r + 0x10, 0);
    e.mem.set_u8(r + 0x11, 0);
}

// Translated from 00d40910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): builds a 0x14-byte record on
/// the stack (the bytes at `+0x10` and `+0x11` zero, then `00d408e0`) and registers it with
/// `00cfbbe0` on `queue` with `(record, 4, 4)`. The padding bytes at `+0x12` are uninitialised in
/// the exe (zero here).
pub fn fn_00d40910(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x14, |e, record| {
        let r = record.addr();
        e.mem.set_u8(r + 0x10, 0);
        e.mem.set_u8(r + 0x11, 0);
        fn_00d408e0(e, record);
        e.call(0x00cf_bbe0, &args![queue, record, 4u32, 4u32]);
    });
}

/// `this`'s vectors at `+0x20`, `+0x30` and `+0x40` as the four-lane arrays of the exe's
/// `UNPCKLPS`/`UNPCKHPS`/`MOVLHPS`/`MOVHLPS`/`SHUFPS` sequence.
fn lanes_00d40940(e: &Engine, at: u32) -> [f32; 4] {
    [0u32, 1, 2, 3].map(|lane| e.mem.f32(at + lane * 4))
}

// Translated from 00d40940 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, no stack words; descriptions, not confirmed): copies the vector at
/// `+0x50` to `+0xc0`, and stores at `+0xd0` the vector `v * M` for `v` that copy and `x0`, `x1`,
/// `x2` the vectors at `+0x20`, `+0x30`, `+0x40`: `(vy * A + vx * B) + vz * C` with
/// `A = (x0.y, x1.y, x2.y, x2.w)`, `B = (x0.x, x1.x, x2.x, x2.y)` and `C = (x0.z, x1.z, x2.z,
/// x2.w)` (the packed matrix the shuffles build; the odd fourth lanes are the code's).
pub fn fn_00d40940(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    for lane in 0..4 {
        let bits = e.mem.u32(me + 0x50 + lane * 4);
        e.mem.set_u32(me + 0xc0 + lane * 4, bits);
    }
    let x0 = lanes_00d40940(e, me + 0x20);
    let x1 = lanes_00d40940(e, me + 0x30);
    let x2 = lanes_00d40940(e, me + 0x40);
    let v = lanes_00d40940(e, me + 0xc0);
    let a = [x0[1], x1[1], x2[1], x2[3]];
    let b = [x0[0], x1[0], x2[0], x2[1]];
    let c = [x0[2], x1[2], x2[2], x2[3]];
    // `MULPS` with the splatted lane as the destination, `ADDPS`: the SSE NaN rules.
    use crate::sse::{add, mul};
    for lane in 0..4usize {
        let by_y = mul(v[1], a[lane]);
        let by_x = mul(v[0], b[lane]);
        let by_z = mul(v[2], c[lane]);
        e.mem
            .set_f32(me + 0xd0 + lane as u32 * 4, add(add(by_y, by_x), by_z));
    }
}

// Translated from 00d409d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the inputs are in registers: `EAX` = `first`, `ECX` = `second`, no stack words;
/// descriptions, not confirmed): for `k` in 0 and 1, when the word at `second + 4 * k` is not 0:
/// with `p = *(first + 4 * k)`, calls `00c905b0` on `**p` (thiscall, no stack words) and stores
/// that word at `*p`.
pub fn fn_00d409d0(e: &mut Engine, first: u32, second: u32) {
    for k in 0..2u32 {
        let slot = first.wrapping_add(k * 4);
        let value = e.mem.u32(second.wrapping_add(k * 4));
        if value != 0 {
            let p = e.mem.u32(slot);
            let held = e.mem.u32(e.mem.u32(p));
            e.call(0x00c9_05b0, &args![held]);
            let p = e.mem.u32(slot);
            let value = e.mem.u32(second.wrapping_add(k * 4));
            e.mem.set_u32(p, value);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_08e0, fn_00d408e0(Ptr)),
        entry!(0x00d4_0910, fn_00d40910(Ptr)),
        entry!(0x00d4_0940, fn_00d40940(Ptr)),
        entry!(0x00d4_09d0, fn_00d409d0(u32, u32)),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d408e0_fills_the_record() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x20);
        for word in 0..8 {
            e.mem.set_u32(record + word * 4, 0xffff_ffff);
        }
        e.call(0x00d4_08e0, &args![record]);
        let words: Vec<u32> = (0..5).map(|word| e.mem.u32(record + word * 4)).collect();
        assert_eq!(
            words,
            vec![
                0x00d4_01d0,
                0x00d4_06c0,
                0x00da_c450,
                0x00da_f8e0,
                0xffff_0000
            ]
        );
    }

    #[test]
    fn fn_00d40910_registers_the_record() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<u32>::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(0x00cf_bbe0, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..4 {
                words.push(e.mem.u32(a[1] + word * 4));
            }
            words.push(e.mem.u32(a[1] + 0x10));
            *log.borrow_mut() = words;
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d4_0910, &args![queue]);
        let seen = seen.borrow();
        assert_eq!(seen[0], queue);
        assert_eq!(seen[2..4], [4, 4]);
        assert_eq!(
            seen[4..],
            [0x00d4_01d0, 0x00d4_06c0, 0x00da_c450, 0x00da_f8e0, 0]
        );
    }

    #[test]
    fn fn_00d40940_builds_the_packed_product() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        let vectors = [
            (0x20u32, [1.0f32, 2.0, 3.0, 4.0]),
            (0x30, [5.0, 6.0, 7.0, 8.0]),
            (0x40, [9.0, 10.0, 11.0, 12.0]),
            (0x50, [1.0, 10.0, 100.0, 0.0]),
        ];
        for (offset, lanes) in vectors {
            for (lane, value) in lanes.into_iter().enumerate() {
                e.mem.set_f32(this + offset + lane as u32 * 4, value);
            }
        }
        e.call(0x00d4_0940, &args![this]);
        let result: Vec<f32> = (0..4)
            .map(|lane| e.mem.f32(this + 0xd0 + lane * 4))
            .collect();
        // vy * (2, 6, 10, 12) + vx * (1, 5, 9, 10) + vz * (3, 7, 11, 12) for (1, 10, 100).
        assert_eq!(result, vec![321.0, 765.0, 1209.0, 1330.0]);
        let copy: Vec<f32> = (0..4)
            .map(|lane| e.mem.f32(this + 0xc0 + lane * 4))
            .collect();
        assert_eq!(copy, vec![1.0, 10.0, 100.0, 0.0]);
    }

    #[test]
    fn fn_00d409d0_releases_and_replaces_the_non_zero_entries() {
        let mut e = Engine::new();
        e.register(0x00c9_05b0, |_, _| Ret::default());
        // first: two pointers p0, p1; each points at a word that holds a pointer to an object
        // whose first word is the held object: **p = held.
        let build = |e: &mut Engine, held: u32| {
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, held);
            let cell = e.mem.alloc(8);
            e.mem.set_u32(cell, object);
            cell
        };
        let (p0, p1) = (build(&mut e, 0x1111), build(&mut e, 0x2222));
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, p0);
        e.mem.set_u32(first + 4, p1);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x7001);
        e.mem.set_u32(second + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00d4_09d0, &args![first, second]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x00c9_05b0, vec![0x1111])]);
        // `*p0` was replaced; the second entry was zero and left alone.
        assert_eq!(e.mem.u32(p0), 0x7001);
        assert_ne!(e.mem.u32(p1), 0);
    }
}
