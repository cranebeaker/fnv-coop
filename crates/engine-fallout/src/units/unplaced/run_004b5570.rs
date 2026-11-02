//! `(unplaced)/run_004b5570` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two float `atan2` wrappers, a 3x3 float matrix element read, and two
//! searches of a node's children for a property (id 3, `00438220`) whose
//! word at +0x1C is 8 to 12.

#[allow(unused_imports)]
use crate::prelude::*;

/// The CRT `atan2` (`00ec9760`; it hands `011c26a0`, the descriptor named
/// "atan2", to the CRT's two-argument dispatcher). `cdecl`, two doubles,
/// result in `ST0`.
const ATAN2: u32 = 0x00ec_9760;
/// The child count of a node (`0043b480`, `thiscall`).
const CHILD_COUNT: u32 = 0x0043_b480;
/// The child at an index (`0043b4a0`, `thiscall`, `RET 4`).
const CHILD_AT: u32 = 0x0043_b4a0;
/// Returns 3 (`00438220`, no arguments): the property id asked for.
const PROPERTY_ID: u32 = 0x0043_8220;
/// `NiAVObject::GetProperty` (`00a59d30`, Xbox PDB; `thiscall`, `RET 4`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// The word at +0x1C of its `this` (`00441110`; named
/// `PathingLocation::GetWorldspace` by the map, a folded body).
const WORD_AT_1C: u32 = 0x0044_1110;
/// `0045bc00` (`thiscall`, `RET 4`): [`fn_004b5710`] calls it with 0 on
/// what slot 0x0C of a child returns.
const CHILD_LOOKUP: u32 = 0x0045_bc00;

/// The property id `00438220` gives on `object` (`00a59d30`), when it is
/// non-null and the signed word `00441110` returns for it is 8 to 12.
fn matching_property(e: &mut Engine, object: u32) -> Option<u32> {
    let id = e.call(PROPERTY_ID, &[]).u32();
    let property = e.call(GET_PROPERTY, &args![object, id]).u32();
    if property == 0 {
        return None;
    }
    let low = e.call(WORD_AT_1C, &args![property]).i32();
    let in_range = low >= 8 && e.call(WORD_AT_1C, &args![property]).i32() <= 12;
    in_range.then_some(property)
}

// Translated from 004b5570 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`cdecl`.) [`fn_004b5590`] on copies of both floats (`FLD`/`FSTP`, so
/// signalling NaNs are passed quiet); its `ST0` result stays this
/// function's result.
pub fn fn_004b5570(e: &mut Engine, y: f32, x: f32) -> f32 {
    fn_004b5590(e, x87_quiet(y), x87_quiet(x))
}

// Translated from 004b5590 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`cdecl`.) `atan2(y, x)` in double precision (`00ec9760`), rounded to a
/// float.
pub fn fn_004b5590(e: &mut Engine, y: f32, x: f32) -> f32 {
    let y = f64::from(x87_quiet(y));
    let x = f64::from(x87_quiet(x));
    e.call(ATAN2, &args![y, x]).f32()
}

// Translated from 004b55c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`thiscall`, `RET 8`.) The float at `this + row * 12 + column * 4` (a
/// 3x3 matrix's element; `FLD`, so a signalling NaN comes back quiet).
pub fn fn_004b55c0(e: &mut Engine, this: Ptr, row: u32, column: u32) -> f32 {
    let at = this
        .addr()
        .wrapping_add(row.wrapping_mul(12))
        .wrapping_add(column.wrapping_mul(4));
    x87_quiet(e.mem.f32(at))
}

// Translated from 004b55e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`cdecl`.) Searches the children of `node` in order: a child whose slot
/// 0x0C answers non-zero is searched the same way (the search stops at the
/// next index once that found something); otherwise a child whose slot 0x18
/// answers non-zero gives its property when that matches
/// ([`matching_property`]). Returns the property found or 0.
pub fn fn_004b55e0(e: &mut Engine, node: u32) -> u32 {
    let mut found = 0;
    if e.call(CHILD_COUNT, &args![node]).u32() == 0 {
        return 0;
    }
    let count = e.call(CHILD_COUNT, &args![node]).u32();
    let mut index = 0u32;
    while index < count {
        if found != 0 {
            return found;
        }
        let child = e.call(CHILD_AT, &args![node, index]).u32();
        if child != 0 {
            if e.vcall(child, 0x0c, &[]).u32() != 0 {
                found = fn_004b55e0(e, child);
            } else if e.vcall(child, 0x18, &[]).u32() != 0 {
                if let Some(property) = matching_property(e, child) {
                    return property;
                }
            }
        }
        index = index.wrapping_add(1);
    }
    found
}

// Translated from 004b5710 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`cdecl`.) Searches the children of `node` from the last to the first:
/// for a child whose slot 0x0C returns an object, `0045bc00(0)` on that
/// object, and a matching property ([`matching_property`]) of what that
/// returns is the result. Without one, [`fn_004b55e0`] on `node`. A node
/// without children gives 0.
pub fn fn_004b5710(e: &mut Engine, node: u32) -> u32 {
    if e.call(CHILD_COUNT, &args![node]).u32() == 0 {
        return 0;
    }
    let mut index = e.call(CHILD_COUNT, &args![node]).u32().wrapping_sub(1);
    loop {
        let child = e.call(CHILD_AT, &args![node, index]).u32();
        if child != 0 {
            let inner = e.vcall(child, 0x0c, &[]).u32();
            if inner != 0 {
                let object = e.call(CHILD_LOOKUP, &args![inner, 0u32]).u32();
                if object != 0 {
                    if let Some(property) = matching_property(e, object) {
                        return property;
                    }
                }
            }
        }
        if index == 0 {
            break;
        }
        index -= 1;
    }
    fn_004b55e0(e, node)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b5570, fn_004b5570(f32, f32) -> f32),
        entry!(0x004b5590, fn_004b5590(f32, f32) -> f32),
        entry!(0x004b55c0, fn_004b55c0(Ptr, u32, u32) -> f32),
        entry!(0x004b55e0, fn_004b55e0(u32) -> u32),
        entry!(0x004b5710, fn_004b5710(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn atan2_double(e: &mut Engine) {
        e.register(ATAN2, |_, a| {
            let y = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let x = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            Ret {
                st0: y.atan2(x),
                ..Ret::default()
            }
        });
    }

    #[test]
    fn the_outer_wrapper_passes_both_floats_on() {
        let mut e = Engine::new();
        atan2_double(&mut e);
        let r = e.call(0x004b_5570, &args![1.0f32, 1.0f32]).f32();
        assert_eq!(r, (1.0f64.atan2(1.0)) as f32);
        e.call_log = Some(vec![]);
        e.call(0x004b_5570, &[0x7f80_0001, 0]);
        let log = e.call_log.take().unwrap();
        let words = &log.iter().find(|c| c.0 == ATAN2).unwrap().1;
        let y = f64::from_bits(u64::from(words[0]) | u64::from(words[1]) << 32);
        assert!(y.is_nan());
        assert_eq!(y.to_bits() >> 51 & 1, 1, "quiet");
    }

    #[test]
    fn atan2_is_widened_and_rounded() {
        let mut e = Engine::new();
        atan2_double(&mut e);
        e.call_log = Some(vec![]);
        let r = e.call(0x004b_5590, &args![-2.0f32, 0.5f32]).f32();
        assert_eq!(r, ((-2.0f64).atan2(0.5)) as f32);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(ATAN2, args![-2.0f64, 0.5f64].to_vec())));
    }

    #[test]
    fn reads_a_matrix_element() {
        let mut e = Engine::new();
        let m = e.mem.alloc(36);
        e.mem.set_f32(m + 12 + 8, 7.25);
        assert_eq!(e.call(0x004b_55c0, &[m, 1, 2]).f32(), 7.25);
        e.mem.set_u32(m + 24, 0xff80_0003);
        assert_eq!(e.call(0x004b_55c0, &[m, 2, 0]).f32().to_bits(), 0xffc0_0003);
    }

    /// A node at `n` with children at `n + 0x100 * (i + 1)`, each with a
    /// vtable at `0x0300_0000 + 0x100 * kind`; the slot doubles return
    /// fixed words read from the child (`+4` for slot 0x0C, `+8` for 0x18).
    fn tree(e: &mut Engine) {
        e.register(CHILD_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CHILD_AT, |e, a| e.mem.u32(a[0] + 4 + 4 * a[1]).into_ret());
        e.register(0x0300_1000, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0300_1004, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0x0300_1000, 0, 0, 0x0300_1004]);
        e.register(PROPERTY_ID, |_, _| 3u32.into_ret());
        e.register(GET_PROPERTY, |e, a| {
            assert_eq!(a[1], 3);
            e.mem.u32(a[0] + 0xc).into_ret()
        });
        e.register(WORD_AT_1C, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        e.register(CHILD_LOOKUP, |e, a| {
            assert_eq!(a[1], 0);
            e.mem.u32(a[0] + 0x10).into_ret()
        });
    }

    fn node(e: &mut Engine, children: &[u32]) -> u32 {
        let n = e.mem.alloc(4 + 4 * children.len() as u32);
        e.mem.set_u32(n, children.len() as u32);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(n + 4 + 4 * i as u32, *c);
        }
        n
    }

    /// An object with the vtable, slot results `inner` (0x0C) and `leaf`
    /// (0x18), property `property` and lookup result `lookup`.
    fn object(e: &mut Engine, inner: u32, leaf: u32, property: u32, lookup: u32) -> u32 {
        let o = e.mem.alloc(0x14);
        e.mem.set_u32(o, 0x0300_0000);
        e.mem.set_u32(o + 4, inner);
        e.mem.set_u32(o + 8, leaf);
        e.mem.set_u32(o + 0xc, property);
        e.mem.set_u32(o + 0x10, lookup);
        o
    }

    fn property(e: &mut Engine, kind: i32) -> u32 {
        let p = e.mem.alloc(0x20);
        e.mem.set_u32(p + 0x1c, kind as u32);
        p
    }

    fn child_indices(log: &Log) -> Vec<u32> {
        log.iter()
            .filter(|c| c.0 == CHILD_AT)
            .map(|c| c.1[1])
            .collect()
    }

    #[test]
    fn the_forward_search_finds_a_leaf_property_in_range() {
        let mut e = Engine::new();
        tree(&mut e);
        let wrong = property(&mut e, 13);
        let low = property(&mut e, 7);
        let right = property(&mut e, 8);
        let a = object(&mut e, 0, 1, wrong, 0);
        let b = object(&mut e, 0, 1, low, 0);
        let c = object(&mut e, 0, 0, right, 0);
        let d = object(&mut e, 0, 1, right, 0);
        let n = node(&mut e, &[a, 0, b, c, d, a]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_55e0, &[n]).u32(), right);
        let log = e.call_log.take().unwrap();
        assert_eq!(child_indices(&log), vec![0, 1, 2, 3, 4]);

        let empty = node(&mut e, &[]);
        assert_eq!(e.call(0x004b_55e0, &[empty]).u32(), 0);
        let none = node(&mut e, &[a, b]);
        assert_eq!(e.call(0x004b_55e0, &[none]).u32(), 0);
    }

    #[test]
    fn the_forward_search_recurses_and_stops_after_a_find() {
        let mut e = Engine::new();
        tree(&mut e);
        let right = property(&mut e, 12);
        let leaf = object(&mut e, 0, 1, right, 0);
        // A branch child is a node itself: its count word is the vtable,
        // so build it as an object whose own words double as a node.
        let branch = e.mem.alloc(0x14);
        e.mem.set_u32(branch, 0x0300_0000);
        e.mem.set_u32(branch + 4, 1);
        e.register_double(CHILD_COUNT, move |e, a| {
            if a[0] == branch {
                1u32.into_ret()
            } else {
                e.mem.u32(a[0]).into_ret()
            }
        });
        e.register_double(CHILD_AT, move |e, a| {
            if a[0] == branch {
                leaf.into_ret()
            } else {
                e.mem.u32(a[0] + 4 + 4 * a[1]).into_ret()
            }
        });
        let other = object(&mut e, 0, 1, right, 0);
        let n = node(&mut e, &[branch, other]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_55e0, &[n]).u32(), right);
        let log = e.call_log.take().unwrap();
        // The outer loop reads child 0 only; the recursion reads the leaf.
        let outer: Vec<_> = log
            .iter()
            .filter(|c| c.0 == CHILD_AT && c.1[0] == n)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(outer, vec![0]);
    }

    #[test]
    fn the_backward_search_looks_up_inner_objects_last_first() {
        let mut e = Engine::new();
        tree(&mut e);
        let right = property(&mut e, 10);
        let wrong = property(&mut e, 2);
        let holder_right = object(&mut e, 0, 0, right, 0);
        let holder_wrong = object(&mut e, 0, 0, wrong, 0);
        let inner_right = object(&mut e, 0, 0, 0, holder_right);
        let inner_wrong = object(&mut e, 0, 0, 0, holder_wrong);
        let inner_none = object(&mut e, 0, 0, 0, 0);
        let a = object(&mut e, inner_right, 0, 0, 0);
        let b = object(&mut e, inner_wrong, 0, 0, 0);
        let c = object(&mut e, inner_none, 0, 0, 0);
        let d = object(&mut e, 0, 0, 0, 0);
        let n = node(&mut e, &[d, a, b, 0, c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_5710, &[n]).u32(), right);
        let log = e.call_log.take().unwrap();
        assert_eq!(child_indices(&log), vec![4, 3, 2, 1]);
        assert!(log.contains(&(CHILD_LOOKUP, vec![inner_right, 0])));
    }

    #[test]
    fn the_backward_search_falls_back_to_the_forward_search() {
        let mut e = Engine::new();
        tree(&mut e);
        let right = property(&mut e, 9);
        let leaf = object(&mut e, 0, 1, right, 0);
        let n = node(&mut e, &[leaf]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_5710, &[n]).u32(), right);
        let log = e.call_log.take().unwrap();
        assert_eq!(child_indices(&log), vec![0, 0]);

        let empty = node(&mut e, &[]);
        assert_eq!(e.call(0x004b_5710, &[empty]).u32(), 0);
    }
}
