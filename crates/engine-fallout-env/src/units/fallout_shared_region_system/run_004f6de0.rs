//! `fallout shared/region system/run_004f6de0` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00eca190`: a CRT math routine taking a `double` on the stack and
/// returning in `ST0` (Ghidra's library match names it `cos`; unconfirmed).
const DOUBLE_MATH_00ECA190: u32 = 0x00ec_a190;

// Translated from 004f6de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 5` (`ECX` only, no stack words).
pub fn fn_004f6de0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 5)
}

// Translated from 004f6e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `float` wrapper of the `double` routine `00eca190` (cdecl, one word):
/// widens `value` (`FLD dword`, `FSTP qword`), calls it, and rounds the
/// result to a `float` (`FSTP dword`, then `FLD dword` into `ST0`). Ghidra's
/// library matcher calls this `logf` and the callee `cos`; neither is
/// confirmed.
pub fn fn_004f6e40(e: &mut Engine, value: f32) -> f32 {
    let widened = x87_load32(value.to_bits());
    let result = e.call(DOUBLE_MATH_00ECA190, &args![widened]).f64();
    x87_quiet(f32::from_bits(x87_store32(result)))
}

/// `NiPoint3::UnitizeGetLength` (`00457910`, `thiscall`): normalizes the
/// point and answers its former length in `ST0`.
const UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
/// `005f65d0` (`thiscall`, `RET 4`: the address of an item word): whether
/// the list holds the item.
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// `006815c0` (`thiscall`): the address of a node's item.
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`): the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `005f36f0` (`thiscall`; the engine map's `ActorMover::GetPreferredMoveMode`
/// is a folded body): the item's nested list object.
const NESTED_LIST_005F36F0: u32 = 0x005f_36f0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl) and the two type descriptors
/// `004f6f70` passes (`01183028`, `01186350`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_6350;

// Translated from 004f6e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: a point by value, three floats):
/// normalizes a copy of the point (`NiPoint3::UnitizeGetLength`), divides
/// its normalized z by the length (both stored as floats) and answers
/// `004f6e40` of that (left in `ST0`).
pub fn fn_004f6e00(e: &mut Engine, x: f32, y: f32, z: f32) -> f32 {
    e.with_stack(0xc, |e, point| {
        let p = point.addr();
        e.mem.set_u32(p, x.to_bits());
        e.mem.set_u32(p + 4, y.to_bits());
        e.mem.set_u32(p + 8, z.to_bits());
        let length = x87_store32(e.call(UNITIZE_GET_LENGTH, &args![point]).f64());
        let unit_z = e.mem.u32(p + 8);
        let ratio = x87_store32(x87_div(x87_load32(unit_z), x87_m32(length)));
        fn_004f6e40(e, f32::from_bits(x87_quiet_bits(ratio)))
    })
}

// Translated from 004f6e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: a form, two bytes):
/// whether `form` is in the list at +4 (`005f65d0` with the address of the
/// argument word; false for a null form), or, with either byte set, in the
/// nested list (`005f36f0`) of an item searched recursively: every item
/// when the first byte is set, otherwise (second byte) the items with a
/// form at vtable `+4` that `004f6f70` accepts.
pub fn fn_004f6e60(e: &mut Engine, this: Ptr, form: u32, all: u8, typed: u8) -> bool {
    if form == 0 {
        return false;
    }
    let held = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), form);
        e.call(LIST_CONTAINS, &args![this.addr().wrapping_add(4), slot])
            .bool()
    });
    if held {
        return true;
    }
    if all == 0 && typed == 0 {
        return false;
    }
    let mut node = if this.is_null() { 0 } else { this.addr() + 4 };
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        let search = if all != 0 {
            true
        } else if typed != 0 {
            let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
            let item = e.mem.u32(item_address);
            if e.vcall(item, 4, &args![]).u32() != 0 {
                let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
                let item = e.mem.u32(item_address);
                fn_004f6f70(e, Ptr::new(item))
            } else {
                false
            }
        } else {
            false
        };
        if search {
            let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
            let item = e.mem.u32(item_address);
            let nested = e.call(NESTED_LIST_005F36F0, &args![item]).u32();
            if fn_004f6e60(e, Ptr::new(nested), form, all, typed) {
                return true;
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 004f6f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): whether the word at +4 is set
/// and casts (`__RTDynamicCast` from `01183028` to `01186350`) to non-null.
pub fn fn_004f6f70(e: &mut Engine, this: Ptr) -> bool {
    let object = e.mem.u32(this.addr() + 4);
    if object == 0 {
        return false;
    }
    let object = e.mem.u32(this.addr() + 4);
    e.call(
        DYNAMIC_CAST,
        &args![object, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
    )
    .u32()
        != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_6de0, fn_004f6de0(Ptr) -> u8),
        entry!(0x004f_6e40, fn_004f6e40(f32) -> f32),
        entry!(0x004f_6e00, fn_004f6e00(f32, f32, f32) -> f32),
        entry!(0x004f_6e60, fn_004f6e60(Ptr, u32, u8, u8) -> bool),
        entry!(0x004f_6f70, fn_004f6f70(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_004f6de0_reads_the_byte_at_5() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.mem.set_u8(object + 5, 0x9c);
        assert_eq!(e.call(0x004f_6de0, &args![object]).u32() & 0xff, 0x9c);
    }

    #[test]
    fn fn_004f6e40_passes_the_widened_value_and_rounds_the_result() {
        let mut e = Engine::new();
        e.register(DOUBLE_MATH_00ECA190, |_, a| {
            let value = f64::take(a, &mut 0);
            (value / 3.0).into_ret()
        });
        e.call_log = Some(vec![]);
        let back = e.call(0x004f_6e40, &args![1.0f32]).f32();
        assert_eq!(back, (1.0f64 / 3.0) as f32);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == DOUBLE_MATH_00ECA190).collect();
        assert_eq!(inner[0].1, args![1.0f64]);
        // A signalling NaN argument reaches the routine quiet.
        e.call_log = Some(vec![]);
        e.call(0x004f_6e40, &[0x7f80_0001]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == DOUBLE_MATH_00ECA190).collect();
        let bits = u64::from(inner[0].1[0]) | u64::from(inner[0].1[1]) << 32;
        assert_eq!(bits, 0x7ff8_0000_2000_0000);
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn fn_004f6e00_divides_the_unit_z_by_the_length() {
        let mut e = Engine::new();
        e.register(UNITIZE_GET_LENGTH, |e, a| {
            let p = a[0];
            let (x, y, z) = (e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8));
            let length = ((x * x + y * y + z * z) as f64).sqrt();
            e.mem.set_f32(p, (x as f64 / length) as f32);
            e.mem.set_f32(p + 4, (y as f64 / length) as f32);
            e.mem.set_f32(p + 8, (z as f64 / length) as f32);
            length.into_ret()
        });
        e.register(DOUBLE_MATH_00ECA190, |_, a| f64::take(a, &mut 0).into_ret());
        e.call_log = Some(vec![]);
        let back = e.call(0x004f_6e00, &args![0.0f32, 3.0f32, 4.0f32]).f32();
        // z/|p| = 0.8, divided by 5.
        assert_eq!(back, (0.8f32 as f64 / 5.0) as f32);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == UNITIZE_GET_LENGTH).collect();
        assert_eq!(inner.len(), 1);
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// List objects: the list at +4 is the first node (item, next); the
    /// held forms are the words of the nodes. Items: vtable slot `+4` the
    /// word at +4 (also read by `004f6f70`), nested list at +8.
    fn setup(e: &mut Engine) {
        e.register(NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_CONTAINS, |e, a| {
            let form = e.mem.u32(a[1]);
            let mut node = a[0];
            let mut found = false;
            while node != 0 {
                found |= e.mem.u32(node) == form;
                node = e.mem.u32(node + 4);
            }
            u32::from(found).into_ret()
        });
        e.put_vtable(0x0300_0000, &[0, 0x0300_1004]);
        e.register(0x0300_1004, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NESTED_LIST_005F36F0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(DYNAMIC_CAST, |e, a| {
            if e.mem.u32(a[0]) == 0x0777 {
                a[0].into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.call_log = Some(vec![]);
    }

    fn list_object(e: &mut Engine, words: &[u32]) -> u32 {
        let object = e.mem.alloc(0xc);
        let mut node = object + 4;
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(node, *w);
            if i + 1 < words.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
        object
    }

    fn item(e: &mut Engine, form: u32, nested: u32) -> u32 {
        let i = e.mem.alloc(0xc);
        e.mem.set_u32(i, 0x0300_0000);
        e.mem.set_u32(i + 4, form);
        e.mem.set_u32(i + 8, nested);
        i
    }

    #[test]
    fn fn_004f6e60_searches_the_list_and_nested_lists() {
        let mut e = Engine::new();
        setup(&mut e);
        let inner = list_object(&mut e, &[0x55]);
        // A form whose first word passes the cast double.
        let typed_form = e.mem.alloc(4);
        e.mem.set_u32(typed_form, 0x0777);
        let plain_form = e.mem.alloc(4);
        let plain = item(&mut e, plain_form, inner);
        let this = list_object(&mut e, &[plain]);
        let search = |e: &mut Engine, this: u32, form: u32, all: u32, typed: u32| {
            e.call(0x004f_6e60, &args![this, form, all, typed]).bool()
        };
        assert!(!search(&mut e, this, 0, 1, 1));
        assert!(search(&mut e, this, plain, 0, 0));
        assert!(!search(&mut e, this, 0x55, 0, 0));
        assert!(search(&mut e, this, 0x55, 1, 0));
        // Second byte only: the plain item's form does not cast.
        assert!(!search(&mut e, this, 0x55, 0, 1));
        let typed = item(&mut e, typed_form, inner);
        let other = list_object(&mut e, &[typed]);
        assert!(search(&mut e, other, 0x55, 0, 1));
        assert_eq!(calls(&e, DYNAMIC_CAST).len(), 2);
    }

    #[test]
    fn fn_004f6f70_casts_the_word_at_4() {
        let mut e = Engine::new();
        setup(&mut e);
        let typed_form = e.mem.alloc(4);
        e.mem.set_u32(typed_form, 0x0777);
        let this = e.mem.alloc(8);
        assert!(!e.call(0x004f_6f70, &args![this]).bool());
        assert!(calls(&e, DYNAMIC_CAST).is_empty());
        e.mem.set_u32(this + 4, typed_form);
        assert!(e.call(0x004f_6f70, &args![this]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST),
            vec![vec![typed_form, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]]
        );
    }
}
