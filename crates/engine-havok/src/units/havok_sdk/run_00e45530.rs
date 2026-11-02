//! `Havok SDK/run_00e45530` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor (`00cbfba0`): thiscall, `this` only.
const BASE_CONSTRUCTOR: u32 = 0x00cb_fba0;

// Translated from 00e45530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall constructor: runs the base constructor (`00cbfba0`), stores the vtables
/// (`010e6dec` at `+0`, `010e6de0` at `+8`, `010e6dcc`, `010e6dc0` and `010e6db4` at `+0xc`,
/// `+0x10`, `+0x14`), the byte 1 at `+0x30` and clears the 32 words of the table from `+0x34`
/// to `+0xb0`. Returns `this`.
pub fn fn_00e45530(e: &mut Engine, this: Ptr) -> u32 {
    let this = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this, 0x010e_6dec);
    e.mem.set_u32(this.wrapping_add(8), 0x010e_6de0);
    e.mem.set_u32(this.wrapping_add(0xc), 0x010e_6dcc);
    e.mem.set_u32(this.wrapping_add(0x10), 0x010e_6dc0);
    e.mem.set_u32(this.wrapping_add(0x14), 0x010e_6db4);
    e.mem.set_u8(this.wrapping_add(0x30), 1);
    for word in 0..32u32 {
        e.mem.set_u32(this.wrapping_add(0x34 + 4 * word), 0);
    }
    this
}

// Translated from 00e455f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: writes a byte to `out`. With either mask `layers` or `against` zero the
/// byte is the one at `this + 0x30`. Otherwise it is whether `against` shares a bit with the OR
/// of the table words `*(this + 0x34 + 4 * k)` over the set bits `k` (0 to 31) of `layers`
/// (a table word is only read for a set bit).
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00e455f0(e: &mut Engine, this: Ptr, out: u32, layers: u32, against: u32) -> u32 {
    let this = this.addr();
    if layers == 0 || against == 0 {
        let default = e.mem.u8(this.wrapping_add(0x30));
        e.mem.set_u8(out, default);
        return out;
    }
    let mut combined = 0u32;
    for bit in 0..32u32 {
        if layers & (1 << bit) != 0 {
            combined |= e.mem.u32(this.wrapping_add(0x34 + 4 * bit));
        }
    }
    e.mem.set_u8(out, u8::from(against & combined != 0));
    out
}

// Translated from 00e45890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc` (an adjuster thunk for the interface at `+8`): `00e455f0` on `this - 8`
/// with the words at `+0x1c` of `first` and `second`. Returns `out`.
pub fn fn_00e45890(e: &mut Engine, this: Ptr, out: u32, first: u32, second: u32) -> u32 {
    let first_mask = e.mem.u32(first.wrapping_add(0x1c));
    let second_mask = e.mem.u32(second.wrapping_add(0x1c));
    fn_00e455f0(
        e,
        Ptr::new(this.addr().wrapping_sub(8)),
        out,
        first_mask,
        second_mask,
    );
    out
}

// Translated from 00e458c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x18` (an adjuster thunk for the interface at `+0xc`): asks `object` (virtual
/// slot `0x10`, one stack word `argument`) for the second mask, takes the first from `+0x1c` of
/// the root of the chain of `node` (following `+0xc` while it is not null) and runs `00e455f0`
/// on `this - 0xc`. The second and fourth stack words are not read. Returns `out`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e458c0(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    _unused_2: u32,
    node: u32,
    _unused_4: u32,
    object: u32,
    argument: u32,
) -> u32 {
    let against = e.vcall(object, 0x10, &args![argument]).u32();
    let mut root = node;
    loop {
        let parent = e.mem.u32(root.wrapping_add(0xc));
        if parent == 0 {
            break;
        }
        root = parent;
    }
    let layers = e.mem.u32(root.wrapping_add(0x1c));
    fn_00e455f0(
        e,
        Ptr::new(this.addr().wrapping_sub(0xc)),
        out,
        layers,
        against,
    );
    out
}

// Translated from 00e45910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x20` (an adjuster thunk for the interface at `+0xc`): asks `first_object`
/// and then `second_object` (virtual slot `0x10`, with `first_argument` and `second_argument`)
/// for the two masks and runs `00e455f0` on `this - 0xc`. The second to fourth stack words are
/// not read. Returns `out`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e45910(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    first_object: u32,
    second_object: u32,
    first_argument: u32,
    second_argument: u32,
) -> u32 {
    let layers = e.vcall(first_object, 0x10, &args![first_argument]).u32();
    let against = e.vcall(second_object, 0x10, &args![second_argument]).u32();
    fn_00e455f0(
        e,
        Ptr::new(this.addr().wrapping_sub(0xc)),
        out,
        layers,
        against,
    );
    out
}

// Translated from 00e45950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x14` (an adjuster thunk for the interface at `+0x10`): the first mask is the
/// word at `+0x20` of `holder`, the second comes from `object` (virtual slot `0x10`, one stack
/// word `argument`); `00e455f0` runs on `this - 0x10`. The third stack word is not read.
/// Returns `out`.
pub fn fn_00e45950(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    holder: u32,
    _unused_3: u32,
    object: u32,
    argument: u32,
) -> u32 {
    let against = e.vcall(object, 0x10, &args![argument]).u32();
    let layers = e.mem.u32(holder.wrapping_add(0x20));
    fn_00e455f0(
        e,
        Ptr::new(this.addr().wrapping_sub(0x10)),
        out,
        layers,
        against,
    );
    out
}

// Translated from 00e45990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc` (an adjuster thunk for the interface at `+0x14`): `00e455f0` on
/// `this - 0x14` with the word at `+0x24` of `first` and the word at `+0x1c` of `second`.
/// Returns `out`.
pub fn fn_00e45990(e: &mut Engine, this: Ptr, out: u32, first: u32, second: u32) -> u32 {
    let first_mask = e.mem.u32(first.wrapping_add(0x24));
    let second_mask = e.mem.u32(second.wrapping_add(0x1c));
    fn_00e455f0(
        e,
        Ptr::new(this.addr().wrapping_sub(0x14)),
        out,
        first_mask,
        second_mask,
    );
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_5530, fn_00e45530(Ptr) -> u32),
        entry!(0x00e4_55f0, fn_00e455f0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e4_5890, fn_00e45890(Ptr, u32, u32, u32) -> u32),
        entry!(
            0x00e4_58c0,
            fn_00e458c0(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00e4_5910,
            fn_00e45910(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00e4_5950, fn_00e45950(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e4_5990, fn_00e45990(Ptr, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// A filter object (`0xb4` bytes) whose table word `k` is `1 << k` and default byte `default`.
    fn filter(e: &mut Engine, default: u8) -> u32 {
        let this = e.mem.alloc(0xb8);
        e.mem.set_u8(this + 0x30, default);
        for bit in 0..32u32 {
            e.mem.set_u32(this + 0x34 + 4 * bit, 1 << bit);
        }
        this
    }

    /// Runs `00e455f0` and returns the byte it wrote (the byte before it is 0xee).
    fn check(e: &mut Engine, this: u32, layers: u32, against: u32) -> u8 {
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 0xee);
        e.call(0x00e4_55f0, &args![this, out, layers, against]);
        assert_eq!(e.mem.u8(out + 1), 0);
        e.mem.u8(out)
    }

    #[test]
    fn constructor_sets_the_vtables_and_clears_the_table() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(BASE_CONSTRUCTOR, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0xc0);
        for word in 0..48u32 {
            e.mem.set_u32(this + 4 * word, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00e4_5530, &args![this]).u32(), this);
        assert_eq!(*seen.borrow(), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x010e_6dec);
        assert_eq!(e.mem.u32(this + 8), 0x010e_6de0);
        assert_eq!(e.mem.u32(this + 0xc), 0x010e_6dcc);
        assert_eq!(e.mem.u32(this + 0x10), 0x010e_6dc0);
        assert_eq!(e.mem.u32(this + 0x14), 0x010e_6db4);
        assert_eq!(e.mem.u8(this + 0x30), 1);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        // The words around the table are untouched.
        assert_eq!(e.mem.u32(this + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0xb4), 0xffff_ffff);
    }

    #[test]
    fn the_layer_filter_answers_its_out_pointer() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc0);
        let out = e.mem.alloc(4);
        // EAX is `out` at every RET (the hkBool result pointer the callers read through).
        assert_eq!(
            e.call(0x00e4_55f0, &args![this, out, 0u32, 0u32]).u32(),
            out
        );
    }

    #[test]
    fn empty_masks_give_the_default_byte() {
        let mut e = engine();
        let this = filter(&mut e, 1);
        assert_eq!(check(&mut e, this, 0, 5), 1);
        assert_eq!(check(&mut e, this, 5, 0), 1);
        let this = filter(&mut e, 0);
        assert_eq!(check(&mut e, this, 0, 0), 0);
    }

    #[test]
    fn shared_bit_of_the_selected_table_words_gives_one() {
        let mut e = engine();
        let this = filter(&mut e, 0);
        assert_eq!(check(&mut e, this, 0b101, 0b100), 1);
        // Bit 1 of the first mask is not set, so its table word is not read.
        assert_eq!(check(&mut e, this, 0b101, 0b010), 0);
        for bit in [7u32, 8, 15, 16, 23, 24, 30, 31] {
            assert_eq!(check(&mut e, this, 1 << bit, 1 << bit), 1);
            assert_eq!(check(&mut e, this, 1 << bit, !(1 << bit)), 0);
        }
        assert_eq!(check(&mut e, this, 0x8000_0001, 0x8000_0000), 1);
    }

    /// Two objects with the second-mask word at `+0x1c`, `+0x20` and `+0x24`.
    fn holder(e: &mut Engine, at_1c: u32, at_20: u32, at_24: u32) -> u32 {
        let holder = e.mem.alloc(0x30);
        e.mem.set_u32(holder + 0x1c, at_1c);
        e.mem.set_u32(holder + 0x20, at_20);
        e.mem.set_u32(holder + 0x24, at_24);
        holder
    }

    #[test]
    fn thunks_adjust_this_and_pick_the_mask_words() {
        let mut e = engine();
        let object = filter(&mut e, 0);
        let first = holder(&mut e, 0b01, 0b10, 0b100);
        let second = holder(&mut e, 0b100, 0, 0);
        let out = e.mem.alloc(8);
        // 00e45890: this = object + 8; the masks are first[0x1c] and second[0x1c].
        assert_eq!(
            e.call(0x00e4_5890, &args![object + 8, out, first, second])
                .u32(),
            out
        );
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(
            e.call(0x00e4_5890, &args![object + 8, out, second, second])
                .u32(),
            out
        );
        assert_eq!(e.mem.u8(out), 1);
        // 00e45990: this = object + 0x14; the masks are first[0x24] and second[0x1c].
        e.call(0x00e4_5990, &args![object + 0x14, out, first, second]);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00e4_5990, &args![object + 0x14, out, second, first]);
        assert_eq!(e.mem.u8(out), 0);
    }

    /// An object whose virtual slot 4 (`0x10`) returns `value`, recording its argument.
    fn answering(
        e: &mut Engine,
        value: u32,
        log: &Rc<RefCell<Vec<u32>>>,
        slot_address: u32,
    ) -> u32 {
        let seen = Rc::clone(log);
        e.register_double(slot_address, move |_, a| {
            seen.borrow_mut().push(a[1]);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        e.put_vtable(slot_address + 0x1000, &[0, 0, 0, 0, slot_address]);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, slot_address + 0x1000);
        object
    }

    #[test]
    fn chain_thunk_walks_to_the_root_node() {
        let mut e = engine();
        let object = filter(&mut e, 0);
        let log = Rc::new(RefCell::new(Vec::new()));
        let answerer = answering(&mut e, 0b100, &log, 0x0ff0_0010);
        let root = holder(&mut e, 0b100, 0, 0);
        let middle = holder(&mut e, 0b1, 0, 0);
        e.mem.set_u32(middle + 0xc, root);
        let leaf = holder(&mut e, 0b10, 0, 0);
        e.mem.set_u32(leaf + 0xc, middle);
        let out = e.mem.alloc(8);
        let result = e
            .call(
                0x00e4_58c0,
                &args![object + 0xc, out, 0u32, leaf, 0u32, answerer, 0x55u32],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(*log.borrow(), vec![0x55]);
        // A node without a parent is its own root: mask 0b10 against 0b100 shares nothing.
        e.mem.set_u32(leaf + 0xc, 0);
        e.call(
            0x00e4_58c0,
            &args![object + 0xc, out, 0u32, leaf, 0u32, answerer, 0u32],
        );
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn two_object_thunk_asks_both_objects() {
        let mut e = engine();
        let object = filter(&mut e, 0);
        let first_log = Rc::new(RefCell::new(Vec::new()));
        let second_log = Rc::new(RefCell::new(Vec::new()));
        let first = answering(&mut e, 0b11, &first_log, 0x0ff0_0100);
        let second = answering(&mut e, 0b10, &second_log, 0x0ff0_0200);
        let out = e.mem.alloc(8);
        let result = e
            .call(
                0x00e4_5910,
                &args![
                    object + 0xc,
                    out,
                    0u32,
                    0u32,
                    0u32,
                    first,
                    second,
                    7u32,
                    8u32
                ],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(*first_log.borrow(), vec![7]);
        assert_eq!(*second_log.borrow(), vec![8]);
    }

    #[test]
    fn holder_thunk_reads_the_word_at_0x20() {
        let mut e = engine();
        let object = filter(&mut e, 0);
        let log = Rc::new(RefCell::new(Vec::new()));
        let answerer = answering(&mut e, 0b1000, &log, 0x0ff0_0040);
        let holder = holder(&mut e, 0, 0b1000, 0);
        let out = e.mem.alloc(8);
        let result = e
            .call(
                0x00e4_5950,
                &args![object + 0x10, out, holder, 0u32, answerer, 9u32],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(*log.borrow(), vec![9]);
        e.mem.set_u32(holder + 0x20, 0b100);
        e.call(
            0x00e4_5950,
            &args![object + 0x10, out, holder, 0u32, answerer, 9u32],
        );
        assert_eq!(e.mem.u8(out), 0);
    }
}
