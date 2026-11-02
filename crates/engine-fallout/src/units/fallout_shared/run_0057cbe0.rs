//! `fallout shared/run_0057cbe0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word `0057ce60` answers (`011c9f6c`).
const SIZE_WORD: u32 = 0x011c_9f6c;
/// The two floats `0057ce70` stores (`01200500`, `01200504`).
const FLOAT_PAIR: u32 = 0x0120_0500;
/// The user count `0057cd10` raises and `0057ce90` lowers (`011ca444`), and the two blocks
/// the first user makes and the last one frees: one of 12-byte elements (`011ca43c`) and
/// one of words (`011ca440`).
const USER_COUNT: u32 = 0x011c_a444;
const ELEMENT_BLOCK: u32 = 0x011c_a43c;
const WORD_BLOCK: u32 = 0x011c_a440;
/// The objects `0057cd10` asks: `0043d4d0` on `011ca524` (`thiscall`, answers the address of
/// the divisor word) and `00403e20` on `011ca510` and `011ca48c` (`thiscall`, each answers
/// the address of a float).
const DIVISOR_OWNER: u32 = 0x011c_a524;
const FIRST_FLOAT_OWNER: u32 = 0x011c_a510;
const SECOND_FLOAT_OWNER: u32 = 0x011c_a48c;
const DIVISOR_OF: u32 = 0x0043_d4d0;
const FLOAT_OF: u32 = 0x0040_3e20;
/// `00401000` (`cdecl`, one word): allocates; `00401030` (`cdecl`, one word): frees.
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The compiler's `vector constructor iterator` (`00401050`, `stdcall`, four words) and the
/// element constructor `0057cd10` gives it (`006815c0`).
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
const ELEMENT_CONSTRUCTOR: u32 = 0x0068_15c0;
/// `00559a40` (`thiscall`, `RET 4`: copy-constructs from the object at the address given),
/// `0045cec0` (`thiscall`: destroys it) and `004a4930` (`thiscall`, `RET 4`: frees a node).
const COPY_CONSTRUCT: u32 = 0x0055_9a40;
const DESTROY: u32 = 0x0045_cec0;
const FREE_NODE: u32 = 0x004a_4930;

// Translated from 0057cbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`, `this` not read): steps the list
/// position at `position` to the next node (`*node`) and answers the address of the old
/// node's item (`node + 8`).
pub fn fn_0057cbe0(e: &mut Engine, _this: Ptr, position: Ptr) -> u32 {
    let node = e.mem.u32(position.addr());
    let item = node.wrapping_add(8);
    let next = e.mem.u32(node);
    e.mem.set_u32(position.addr(), next);
    item
}

// Translated from 0057cc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): the object's own slot `0x10` with
/// `(block, count * 8)` (an allocator's free of `count` 8-byte elements).
pub fn fn_0057cc10(e: &mut Engine, this: Ptr, block: u32, count: u32) {
    e.vcall(this.addr(), 0x10, &args![block, count << 3]);
}

// Translated from 0057cc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the list's `RemoveTail` for items held by value, `thiscall`,
/// `RET 4`: the address of the answer object): unlinks the tail node (head `+0`, tail `+4`,
/// count `+8`; a node's next `+0`, previous `+4`, item `+8`), copies its item into a local
/// (`00559a40`), frees the node (`004a4930`), counts one fewer, copies the local into the
/// answer object, destroys the local (`0045cec0`) and answers the answer object.
// Translation incomplete: 0057cc40 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) and the /GS cookie word are not translated.
pub fn fn_0057cc40(e: &mut Engine, this: Ptr, answer: Ptr) -> Ptr {
    let list = this.addr();
    let node = e.mem.u32(list + 4);
    let previous = e.mem.u32(e.mem.u32(list + 4) + 4);
    e.mem.set_u32(list + 4, previous);
    if e.mem.u32(list + 4) == 0 {
        e.mem.set_u32(list, 0);
    } else {
        let tail = e.mem.u32(list + 4);
        e.mem.set_u32(tail, 0);
    }
    e.with_stack(4, |e, local| {
        e.call(COPY_CONSTRUCT, &args![local, node.wrapping_add(8)]);
        e.call(FREE_NODE, &args![this, node]);
        let count = e.mem.u32(list + 8).wrapping_sub(1);
        e.mem.set_u32(list + 8, count);
        e.call(COPY_CONSTRUCT, &args![answer, local]);
        e.call(DESTROY, &args![local]);
    });
    answer
}

// Translated from 0057ce60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no argument): the word at `011c9f6c`.
pub fn fn_0057ce60(e: &mut Engine) -> u32 {
    e.mem.u32(SIZE_WORD)
}

// Translated from 0057ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two floats as their bits): stores them at `01200500`
/// and `01200504` through the x87 stack.
pub fn fn_0057ce70(e: &mut Engine, first: u32, second: u32) {
    e.mem.set_u32(FLOAT_PAIR, x87_quiet_bits(first));
    e.mem.set_u32(FLOAT_PAIR + 4, x87_quiet_bits(second));
}

// Translated from 0057ce90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` saved, not read): one user fewer (`011ca444`); the last
/// one frees the two blocks (`00401030`) and clears their pointers.
pub fn fn_0057ce90(e: &mut Engine, _this: Ptr) {
    let users = e.mem.u32(USER_COUNT).wrapping_sub(1);
    e.mem.set_u32(USER_COUNT, users);
    if users != 0 {
        return;
    }
    for slot in [ELEMENT_BLOCK, WORD_BLOCK] {
        let block = e.mem.u32(slot);
        if block != 0 {
            e.call(FREE, &args![block]);
            e.mem.set_u32(slot, 0);
        }
    }
}

/// `MUL` by `size` with the compiler's overflow guard (`SETO; NEG; OR`): `0xffffffff`
/// when the product does not fit in 32 bits.
fn guarded_bytes(count: u32, size: u32) -> u32 {
    let product = u64::from(count) * u64::from(size);
    if product > u64::from(u32::MAX) {
        0xffff_ffff
    } else {
        product as u32
    }
}

// Translated from 0057cd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, answered): one user more (`011ca444`). The first
/// user makes the two blocks when missing: with `n = (word(011c9f6c) << 8) / divisor`
/// (signed; the divisor word from `0043d4d0` on `011ca524`), `n * n` 12-byte elements
/// (`00401000`, constructed by `006815c0` through the vector constructor; null when the
/// allocation fails) and `n * n` words. Then `0057ce70` gets the floats behind `00403e20`
/// on `011ca48c` and on `011ca510` (asked in the reverse order, each copied through the x87
/// stack).
// Translation incomplete: 0057cd10 - the C++ exception frame (FS:[0] chain, unwind state 0) and the /GS cookie word are not translated.
pub fn fn_0057cd10(e: &mut Engine, this: Ptr) -> Ptr {
    if e.mem.u32(USER_COUNT) == 0 {
        let size = fn_0057ce60(e);
        let scaled = (size << 8) as i32;
        let divisor_at = e.call(DIVISOR_OF, &args![DIVISOR_OWNER]).u32();
        let side = scaled / e.mem.i32(divisor_at);
        let cells = side.wrapping_mul(side) as u32;
        if e.mem.u32(ELEMENT_BLOCK) == 0 {
            let block = e.call(ALLOCATE, &args![guarded_bytes(cells, 0xc)]).u32();
            let elements = if block == 0 {
                0
            } else {
                e.call(
                    VECTOR_CONSTRUCTOR,
                    &args![block, 0xcu32, cells, ELEMENT_CONSTRUCTOR],
                );
                block
            };
            e.mem.set_u32(ELEMENT_BLOCK, elements);
        }
        if e.mem.u32(WORD_BLOCK) == 0 {
            let block = e.call(ALLOCATE, &args![guarded_bytes(cells, 4)]).u32();
            e.mem.set_u32(WORD_BLOCK, block);
        }
    }
    let users = e.mem.u32(USER_COUNT).wrapping_add(1);
    e.mem.set_u32(USER_COUNT, users);
    let first = e.call(FLOAT_OF, &args![FIRST_FLOAT_OWNER]).u32();
    let second_word = x87_quiet_bits(e.mem.u32(first));
    let second = e.call(FLOAT_OF, &args![SECOND_FLOAT_OWNER]).u32();
    let first_word = x87_quiet_bits(e.mem.u32(second));
    fn_0057ce70(e, first_word, second_word);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0057_cbe0, fn_0057cbe0(Ptr, Ptr) -> u32),
        entry!(0x0057_cc10, fn_0057cc10(Ptr, u32, u32)),
        entry!(0x0057_cc40, fn_0057cc40(Ptr, Ptr) -> Ptr),
        entry!(0x0057_ce60, fn_0057ce60() -> u32),
        entry!(0x0057_ce70, fn_0057ce70(u32, u32)),
        entry!(0x0057_ce90, fn_0057ce90(Ptr)),
        entry!(0x0057_cd10, fn_0057cd10(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_9000, 0x2000);
        e.map(0x0120_0000, 0x1000);
        for address in [FREE, VECTOR_CONSTRUCTOR, COPY_CONSTRUCT, DESTROY, FREE_NODE] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn stepping_a_position() {
        let mut e = engine();
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node, 0x7000);
        let position = e.mem.alloc(4);
        e.mem.set_u32(position, node);
        assert_eq!(e.call(0x0057_cbe0, &args![0u32, position]).u32(), node + 8);
        assert_eq!(e.mem.u32(position), 0x7000);
    }

    #[test]
    fn the_free_goes_through_slot_0x10() {
        let mut e = engine();
        e.register(0x0300_1000, |_, _| Ret::default());
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0, 0x0300_1000]);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0300_0000);
        e.call(0x0057_cc10, &args![this, 0x6000u32, 3u32]);
        assert_eq!(calls(&e, 0x0300_1000), vec![vec![this, 0x6000, 24]]);
    }

    #[test]
    fn removing_the_tail_copies_the_item_out() {
        let mut e = engine();
        let (first, second) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        e.mem.set_u32(first, second);
        e.mem.set_u32(second + 4, first);
        let list = e.mem.alloc(0xc);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        let answer = e.mem.alloc(4);
        assert_eq!(e.call(0x0057_cc40, &args![list, answer]).u32(), answer);
        assert_eq!((e.mem.u32(list + 4), e.mem.u32(first)), (first, 0));
        assert_eq!(e.mem.u32(list + 8), 1);
        let copies = calls(&e, COPY_CONSTRUCT);
        assert_eq!(copies[0][1], second + 8);
        assert_eq!(copies[1], vec![answer, copies[0][0]]);
        assert_eq!(calls(&e, FREE_NODE), vec![vec![list, second]]);
        assert_eq!(calls(&e, DESTROY), vec![vec![copies[0][0]]]);
        // The last node: the head is cleared.
        e.call(0x0057_cc40, &args![list, answer]);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (0, 0));
    }

    #[test]
    fn the_small_globals() {
        let mut e = engine();
        e.mem.set_u32(SIZE_WORD, 0x40);
        assert_eq!(e.call(0x0057_ce60, &args![]).u32(), 0x40);
        e.call(0x0057_ce70, &args![1.5f32.to_bits(), 0x7f80_0001u32]);
        assert_eq!(e.mem.f32(FLOAT_PAIR), 1.5);
        assert_eq!(e.mem.u32(FLOAT_PAIR + 4), 0x7fc0_0001);
    }

    #[test]
    fn users_make_and_free_the_blocks() {
        let mut e = engine();
        e.mem.set_u32(SIZE_WORD, 4);
        let divisor = e.mem.alloc(4);
        e.mem.set_i32(divisor, 0x80);
        e.register_double(DIVISOR_OF, move |_, _| divisor.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let (a, b) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_f32(a, 2.0);
        e.mem.set_f32(b, 3.0);
        e.register_double(FLOAT_OF, move |_, x| {
            if x[0] == FIRST_FLOAT_OWNER { a } else { b }.into_ret()
        });
        assert_eq!(e.call(0x0057_cd10, &args![0x5000u32]).u32(), 0x5000);
        // (4 << 8) / 0x80 = 8: 64 cells.
        assert_eq!(calls(&e, ALLOCATE), vec![vec![64 * 12], vec![64 * 4]]);
        let elements = e.mem.u32(ELEMENT_BLOCK);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCTOR),
            vec![vec![elements, 12, 64, ELEMENT_CONSTRUCTOR]]
        );
        assert_eq!(e.mem.u32(USER_COUNT), 1);
        assert_eq!(
            (e.mem.f32(FLOAT_PAIR), e.mem.f32(FLOAT_PAIR + 4)),
            (3.0, 2.0)
        );
        // A second user makes nothing.
        e.call(0x0057_cd10, &args![0x5000u32]);
        assert_eq!(calls(&e, ALLOCATE).len(), 2);
        // Leaving: the last one frees both.
        e.call(0x0057_ce90, &args![0u32]);
        assert!(calls(&e, FREE).is_empty());
        let words = e.mem.u32(WORD_BLOCK);
        e.call(0x0057_ce90, &args![0u32]);
        assert_eq!(calls(&e, FREE), vec![vec![elements], vec![words]]);
        assert_eq!((e.mem.u32(ELEMENT_BLOCK), e.mem.u32(WORD_BLOCK)), (0, 0));
    }
}
