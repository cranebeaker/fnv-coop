//! `Havok SDK/run_00e46820` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c9c670`: thiscall on `item`, one stack word, `RET 4`.
const RELEASE_ITEM: u32 = 0x00c9_c670;
/// The base constructor (`00cbfba0`): thiscall, `this` only.
const BASE_CONSTRUCTOR: u32 = 0x00cb_fba0;

// Translated from 00e46820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: removes `item` from the pointer array at `*(this + 0x34)` (`[+0x38]`
/// entries) and writes whether it was there to `out` (a byte). The first match is replaced by the
/// last entry (the count drops by one; nothing is copied when it was the last). A null `item`
/// gives 0 without a search.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00d1b726).
pub fn fn_00e46820(e: &mut Engine, this: Ptr, out: u32, item: u32) -> u32 {
    let this = this.addr();
    if item == 0 {
        e.mem.set_u8(out, 0);
        return out;
    }
    let count = e.mem.i32(this.wrapping_add(0x38));
    let array = e.mem.u32(this.wrapping_add(0x34));
    for index in 0..count.max(0) as u32 {
        let entry = e.mem.u32(array.wrapping_add(index.wrapping_mul(4)));
        if entry != item {
            continue;
        }
        let last = e.mem.u32(this.wrapping_add(0x38)).wrapping_sub(1);
        e.mem.set_u32(this.wrapping_add(0x38), last);
        if last != index {
            let data = e.mem.u32(this.wrapping_add(0x34));
            let moved = e.mem.u32(data.wrapping_add(last.wrapping_mul(4)));
            e.mem
                .set_u32(data.wrapping_add(index.wrapping_mul(4)), moved);
        }
        e.mem.set_u8(out, 1);
        return out;
    }
    e.mem.set_u8(out, 0);
    out
}

// Translated from 00e46880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4` (virtual, in the interface at `+0x30`): for a non-null `item`, removes it
/// from the array of `this - 0x30` (`00e46820`; its result byte goes to a stack word that
/// nothing reads) and then calls `00c9c670` on `item` with `this`, or 0 when `this` is `0x30`
/// (the compiler's null check of the adjusted pointer).
pub fn fn_00e46880(e: &mut Engine, this: Ptr, item: u32) {
    if item == 0 {
        return;
    }
    let this = this.addr();
    e.with_stack(4, |e, out| {
        fn_00e46820(e, Ptr::new(this.wrapping_sub(0x30)), out.addr(), item);
    });
    let adjusted = if this == 0x30 { 0 } else { this };
    e.call(RELEASE_ITEM, &args![item, adjusted]);
}

// Translated from 00e468b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall constructor: runs the base constructor (`00cbfba0`), stores the vtables (`010e7700`
/// at `+0`, `010e76f4`, `010e76e0`, `010e76d4` and `010e76c8` at `+8`, `+0xc`, `+0x10`,
/// `+0x14`, `010e76ac` at `+0x30`, after `0102f13c` which it overwrites at once) and an empty
/// unowned `hkArray` (`data` 0, `size` 0, `capacity | flags` `0x80000000`) at `+0x34`. Returns
/// `this`.
pub fn fn_00e468b0(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.wrapping_add(0x30), 0x0102_f13c);
    e.mem.set_u32(this, 0x010e_7700);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_76f4);
    e.mem.set_u32(this.wrapping_add(0xc), 0x010e_76e0);
    e.mem.set_u32(this.wrapping_add(0x10), 0x010e_76d4);
    e.mem.set_u32(this.wrapping_add(0x14), 0x010e_76c8);
    e.mem.set_u32(this.wrapping_add(0x30), 0x010e_76ac);
    e.mem.set_u32(this.wrapping_add(0x34), 0);
    e.mem.set_u32(this.wrapping_add(0x38), 0);
    e.mem.set_u32(this.wrapping_add(0x3c), 0x8000_0000);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_6820, fn_00e46820(Ptr, u32, u32) -> u32),
        entry!(0x00e4_6880, fn_00e46880(Ptr, u32)),
        entry!(0x00e4_68b0, fn_00e468b0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An object with the array `items` at `+0x34`.
    fn owner(e: &mut Engine, items: &[u32]) -> u32 {
        let this = e.mem.alloc(0x40);
        let array = e.mem.alloc(4 * items.len() as u32 + 4);
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *item);
        }
        e.mem.set_u32(this + 0x34, array);
        e.mem.set_u32(this + 0x38, items.len() as u32);
        this
    }

    fn items(e: &Engine, this: u32) -> Vec<u32> {
        let array = e.mem.u32(this + 0x34);
        (0..e.mem.u32(this + 0x38))
            .map(|index| e.mem.u32(array + 4 * index))
            .collect()
    }

    fn remove(e: &mut Engine, this: u32, item: u32) -> u8 {
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 0xee);
        e.call(0x00e4_6820, &args![this, out, item]);
        assert_eq!(e.mem.u8(out + 1), 0);
        e.mem.u8(out)
    }

    #[test]
    fn removed_item_is_replaced_by_the_last_one() {
        let mut e = Engine::new();
        let this = owner(&mut e, &[0x11, 0x22, 0x33, 0x44]);
        assert_eq!(remove(&mut e, this, 0x22), 1);
        assert_eq!(items(&e, this), vec![0x11, 0x44, 0x33]);
        assert_eq!(remove(&mut e, this, 0x33), 1);
        assert_eq!(items(&e, this), vec![0x11, 0x44]);
    }

    #[test]
    fn remove_answers_its_out_pointer() {
        let mut e = Engine::new();
        let this = owner(&mut e, &[0x11, 0x22]);
        let out = e.mem.alloc(8);
        // EAX is `out` at every RET (the hkBool result pointer the callers read through).
        assert_eq!(e.call(0x00e4_6820, &args![this, out, 0x22u32]).u32(), out);
    }

    #[test]
    fn missing_or_null_item_gives_zero() {
        let mut e = Engine::new();
        let this = owner(&mut e, &[0x11, 0x22]);
        assert_eq!(remove(&mut e, this, 0x99), 0);
        assert_eq!(remove(&mut e, this, 0), 0);
        assert_eq!(items(&e, this), vec![0x11, 0x22]);
        let empty = owner(&mut e, &[]);
        assert_eq!(remove(&mut e, empty, 0x11), 0);
    }

    fn release_double(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(RELEASE_ITEM, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        seen
    }

    #[test]
    fn interface_removal_releases_the_item_with_this() {
        let mut e = Engine::new();
        let seen = release_double(&mut e);
        let object = owner(&mut e, &[0x11, 0x22]);
        e.call(0x00e4_6880, &args![object + 0x30, 0x11u32]);
        assert_eq!(items(&e, object), vec![0x22]);
        assert_eq!(*seen.borrow(), vec![vec![0x11, object + 0x30]]);
    }

    #[test]
    fn interface_removal_of_a_null_item_does_nothing() {
        let mut e = Engine::new();
        let seen = release_double(&mut e);
        let object = owner(&mut e, &[0x11]);
        e.call(0x00e4_6880, &args![object + 0x30, 0u32]);
        assert!(seen.borrow().is_empty());
        assert_eq!(items(&e, object), vec![0x11]);
    }

    #[test]
    fn interface_at_address_0x30_passes_zero() {
        let mut e = Engine::new();
        let seen = release_double(&mut e);
        // `this - 0x30` is the null pointer, so the (empty) object at 0 is not touched: map it.
        e.map(0, 0x1000);
        e.call(0x00e4_6880, &args![0x30u32, 0x11u32]);
        assert_eq!(*seen.borrow(), vec![vec![0x11, 0]]);
    }

    #[test]
    fn constructor_stores_the_vtables_and_the_empty_array() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(BASE_CONSTRUCTOR, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00e4_68b0, &args![this]).u32(), this);
        assert_eq!(*seen.borrow(), vec![vec![this]]);
        let words: Vec<u32> = [0u32, 8, 0xc, 0x10, 0x14, 0x30, 0x34, 0x38, 0x3c]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            words,
            vec![
                0x010e_7700,
                0x010e_76f4,
                0x010e_76e0,
                0x010e_76d4,
                0x010e_76c8,
                0x010e_76ac,
                0,
                0,
                0x8000_0000
            ]
        );
    }
}
