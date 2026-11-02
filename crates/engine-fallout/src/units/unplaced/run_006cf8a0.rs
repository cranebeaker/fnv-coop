//! `(unplaced)/run_006cf8a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006bc870`: the base part's assignment (`thiscall`, one word).
const BASE_ASSIGN: u32 = 0x006b_c870;

// Translated from 006cf900 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: assignment from `other`: the base part (`006bc870`), then
/// the three words at `+0xc`. Returns `this`.
pub fn fn_006cf900(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(BASE_ASSIGN, &args![this, other]);
    for i in 0..3 {
        let word = e.mem.u32(other.addr() + 0xc + 4 * i);
        e.mem.set_u32(this.addr() + 0xc + 4 * i, word);
    }
    this
}

/// The destination array's resize (`006d01d0` for 0x18-byte elements, `006cff60` for
/// 0x28-byte ones; thiscall: count, 1) and element address (`006cee10` / `00691750`;
/// thiscall: index), and the 0x28-byte element's assignment (`006dcce0`; thiscall: the
/// source).
const RESIZE_SIZE_18: u32 = 0x006d_01d0;
const ELEMENT_AT_SIZE_18: u32 = 0x006c_ee10;
const RESIZE_SIZE_28: u32 = 0x006c_ff60;
const ELEMENT_AT_SIZE_28: u32 = 0x0069_1750;
const ASSIGN_SIZE_28: u32 = 0x006d_cce0;

/// The copy of one array (buffer `+4`, count `+8`) into `destination`: resizes it to the
/// count (`resize(count, 1)`), then assigns each element `i` to the destination's element
/// `i` (`element_at`; `assign` on that address with the source element's).
fn copy_into(
    e: &mut Engine,
    this: Ptr,
    destination: u32,
    element: u32,
    resize: u32,
    element_at: u32,
    assign: &dyn Fn(&mut Engine, u32, u32),
) {
    let base = this.addr();
    let count = e.mem.u32(base + 8);
    e.call(resize, &args![destination, count, 1u32]);
    let mut index = 0u32;
    while index < e.mem.u32(base + 8) {
        let source = index
            .wrapping_mul(element)
            .wrapping_add(e.mem.u32(base + 4));
        let target = e.call(element_at, &args![destination, index]).u32();
        assign(e, target, source);
        index += 1;
    }
}

// Translated from 006cf8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 4`; called from `BuildPathToValidNavMeshTriangle`
/// and `006ea780`): copies this array of 0x18-byte elements into `destination` (resize
/// `006d01d0`, element address `006cee10`, assignment [`fn_006cf900`]); see
/// [`copy_into`].
pub fn fn_006cf8a0(e: &mut Engine, this: Ptr, destination: u32) {
    copy_into(
        e,
        this,
        destination,
        0x18,
        RESIZE_SIZE_18,
        ELEMENT_AT_SIZE_18,
        &|e, target, source| {
            fn_006cf900(e, Ptr::new(target), Ptr::new(source));
        },
    );
}

// Translated from 006cf940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 4`; called from `BuildPathToValidNavMeshTriangle`,
/// `BuildStraightPathUsingRayCasts` and others): copies this array of 0x28-byte elements
/// into `destination` (resize `006cff60`, element address `00691750`, assignment
/// `006dcce0`); see [`copy_into`].
pub fn fn_006cf940(e: &mut Engine, this: Ptr, destination: u32) {
    copy_into(
        e,
        this,
        destination,
        0x28,
        RESIZE_SIZE_28,
        ELEMENT_AT_SIZE_28,
        &|e, target, source| {
            e.call(ASSIGN_SIZE_28, &args![target, source]);
        },
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c_f900, fn_006cf900(Ptr, Ptr) -> Ptr),
        entry!(0x006c_f8a0, fn_006cf8a0(Ptr, u32)),
        entry!(0x006c_f940, fn_006cf940(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assigns_base_then_point() {
        let mut e = Engine::new();
        e.register(BASE_ASSIGN, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        let other = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_u32(other + 4 * i, 0x100 + i);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c_f900, &args![this, other]).u32(), this);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x103);
        assert_eq!(e.mem.u32(this + 0x14), 0x105);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_ASSIGN, vec![this, other])));
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn source(e: &mut Engine, count: u32) -> u32 {
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x4000);
        e.mem.set_u32(array + 8, count);
        array
    }

    #[test]
    fn copies_0x18_byte_elements() {
        let mut e = Engine::new();
        e.register(RESIZE_SIZE_18, |_, _| Ret::default());
        e.register(ELEMENT_AT_SIZE_18, |_, a| {
            (0x9000 + 0x100 * a[1]).into_ret()
        });
        e.register(BASE_ASSIGN, |_, _| Ret::default());
        e.map(0x4000, 0x100);
        e.map(0x9000, 0x300);
        e.mem.set_u32(0x4018 + 0xc, 0x55);
        let array = source(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x006c_f8a0, &args![array, 0x7000u32]);
        assert_eq!(calls_to(&e, RESIZE_SIZE_18), vec![vec![0x7000, 2, 1]]);
        assert_eq!(
            calls_to(&e, BASE_ASSIGN),
            vec![vec![0x9000, 0x4000], vec![0x9100, 0x4018]]
        );
        assert_eq!(e.mem.u32(0x9100 + 0xc), 0x55);
    }

    #[test]
    fn copies_0x28_byte_elements() {
        let mut e = Engine::new();
        e.register(RESIZE_SIZE_28, |_, _| Ret::default());
        e.register(ELEMENT_AT_SIZE_28, |_, a| (0x9000 + a[1]).into_ret());
        e.register(ASSIGN_SIZE_28, |_, _| Ret::default());
        let array = source(&mut e, 3);
        e.call_log = Some(vec![]);
        e.call(0x006c_f940, &args![array, 0x7000u32]);
        assert_eq!(
            calls_to(&e, ASSIGN_SIZE_28),
            vec![
                vec![0x9000, 0x4000],
                vec![0x9001, 0x4028],
                vec![0x9002, 0x4050]
            ]
        );
        let empty = source(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x006c_f940, &args![empty, 0x7000u32]);
        assert_eq!(calls_to(&e, RESIZE_SIZE_28), vec![vec![0x7000, 0, 1]]);
        assert!(calls_to(&e, ASSIGN_SIZE_28).is_empty());
    }
}
