//! `Havok SDK/run_00df7190` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00df7190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x4` (`SUB ECX,0x4`) and jumps to `00df71d0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00df7190(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_71d0, &args![this.addr().wrapping_sub(0x4), flags])
        .u32()
}

// Translated from 00df71a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00df71d0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00df71a0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_71d0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00df71d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry point inside `00df71b0` (the three thunks `00df7190`, `00df71a0` and `00df71b0` jump
/// here after moving `this` back; entered with `this` in `ECX` already moved, one stack word
/// `flags`, `RET 4`): the scalar deleting destructor's body. The four vtable words at `+0xc`,
/// `+8`, `+4` and `+0` become `010ccfac`, `010ccfa0`, `010ccf8c` and `010c3bb8`, and when bit 0
/// of `flags` is set the block (`0x90` bytes) is returned to the memory router's allocator (slot
/// `8`). Returns `this` (`EAX`). The class is unconfirmed.
pub fn deleting_destructor_00df71d0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(0xc), 0x010c_cfac);
    e.mem.set_u32(base.wrapping_add(0x8), 0x010c_cfa0);
    e.mem.set_u32(base.wrapping_add(0x4), 0x010c_cf8c);
    e.mem.set_u32(base, 0x010c_3bb8);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![base, 0x90u32]);
    }
    this
}

// Translated from 00df71b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0xc` (`SUB ECX,0xc`) and continues in the body at `00df71d0`
/// (the same stack word `flags`); the caller gets its `EAX`, the moved `this`. The class is
/// unconfirmed.
pub fn deleting_destructor_00df71b0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    deleting_destructor_00df71d0(e, Ptr::new(this.addr().wrapping_sub(0xc)), flags)
}

// Translated from 00df71c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: returns (`FLD`) the float at `+4` of the object `this + 0x10`
/// points to. The class is unconfirmed.
pub fn fn_00df71c0(e: &mut Engine, this: Ptr) -> f32 {
    let inner = e.mem.u32(this.addr() + 0x10);
    x87_quiet(e.mem.f32(inner + 4))
}

// Translated from 00df7210 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: stores to the byte at `out` whether the signed word at `+0x14` of
/// the object `this + 0x10` points to is greater than zero (`SETG`), and returns `out` (`EAX`).
/// The class is unconfirmed.
pub fn fn_00df7210(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let inner = e.mem.u32(this.addr() + 0x10);
    let count = e.mem.i32(inner + 0x14);
    e.mem.set_u8(out.addr(), u8::from(count > 0));
    out
}

/// `FLD float ptr [[this + 0x14] + 4]`.
fn float_at_target(e: &mut Engine, this_addr: u32) -> f32 {
    let target = e.mem.u32(this_addr + 0x14);
    x87_quiet(e.mem.f32(target + 4))
}

// Translated from 00df7230 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: looks at the node `node` (the code treats it as a pointer to a
/// record whose first word is a pointer) and returns the float at `+4` of the object at
/// `this + 0x14` (`FLD`). If the record's first word is not null and `node` differs from the
/// pointer at `this + 0x18`, it asks the object at `[this + 0x10] + 8` (virtual slot `4`, a
/// function that pops three words: the address of the stack word that holds `node`, the pointer
/// at `this + 0x18` and `node`) for a byte flag; when that flag is not zero, it calls the
/// function pointer in the table at `[this + 0x20] + 0x9ac` (entries `0x14` bytes apart), chosen
/// by the byte at `[this + 0x20] + ((this[0x1c] + 0xd) << 5) + [record + 0xc]`, with
/// `(this[0x18], node, this + 0x20, this[0x14], 0)` (cdecl). The class is unconfirmed.
pub fn fn_00df7230(e: &mut Engine, this: Ptr, node: Ptr) -> f32 {
    let this_addr = this.addr();
    let record = e.mem.u32(node.addr());
    if record != 0 {
        let previous = e.mem.u32(this_addr + 0x18);
        if previous != node.addr() {
            let holder = e.mem.u32(this_addr + 0x10) + 8;
            // The flag is the frame's `node` argument slot (`ESP+0x18` at the call): slot 4
            // writes it there and answers its address, and the byte is tested through that
            // address, so the test is made while the slot is there. (Scoped to the slot call
            // alone, the byte was read from a freed block.)
            let set = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), node.addr());
                let flag = e.vcall(holder, 4, &args![slot, previous, node]).u32();
                e.mem.u8(flag) != 0
            });
            if !set {
                return float_at_target(e, this_addr);
            }
            let table = e.mem.u32(this_addr + 0x20);
            let index = e.mem.u32(this_addr + 0x1c).wrapping_add(0xd);
            let selector = index
                .wrapping_shl(5)
                .wrapping_add(e.mem.u32(record + 0xc))
                .wrapping_add(table);
            let which = u32::from(e.mem.u8(selector));
            let entry = table
                .wrapping_add(which.wrapping_mul(0x14))
                .wrapping_add(0x9ac);
            let function = e.mem.u32(entry);
            let target = e.mem.u32(this_addr + 0x14);
            // 00df727c: `this + 0x18` read again after the slot-4 call, not the word passed to it.
            let current = e.mem.u32(this_addr + 0x18);
            e.call(
                function,
                &args![current, node, this_addr + 0x20, target, 0u32],
            );
        }
    }
    float_at_target(e, this_addr)
}

// Translated from 00df72c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: replaces the stack word `node` by the word at `node + 0x20`, moves `this` back by `8`
/// (`ADD ECX,-8`) and jumps to `00df7230`; its result is that function's.
pub fn fn_00df72c0(e: &mut Engine, this: Ptr, node: Ptr) -> f32 {
    let inner = e.mem.u32(node.addr() + 0x20);
    fn_00df7230(e, Ptr::new(this.addr().wrapping_sub(8)), Ptr::new(inner))
}

// Translated from 00df72e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: replaces the stack word `node` by the word at `node + 0xc`, moves `this` back by `0xc`
/// (`ADD ECX,-0xc`) and jumps to `00df7230`; its result is that function's.
pub fn fn_00df72e0(e: &mut Engine, this: Ptr, node: Ptr) -> f32 {
    let inner = e.mem.u32(node.addr() + 0xc);
    fn_00df7230(e, Ptr::new(this.addr().wrapping_sub(0xc)), Ptr::new(inner))
}

// Translated from 00df7300 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: runs `00df7230` with `this` moved back by `4` and `node`,
/// throws its float away (`FSTP ST0`), stores `1` to the byte at `out` and returns `out`
/// (`EAX`).
pub fn fn_00df7300(e: &mut Engine, this: Ptr, out: Ptr, node: Ptr) -> Ptr {
    fn_00df7230(e, Ptr::new(this.addr().wrapping_sub(4)), node);
    e.mem.set_u8(out.addr(), 1);
    out
}

// Translated from 00df7320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: runs `00df7230` on the node at `object` plus the signed byte at
/// `object + 5` (`MOVSX`) with the incoming `this` (`ECX`) passed through unchanged; its float
/// stays in `ST0` as the result. The second stack word is never read (`RET 8`).
pub fn fn_00df7320(e: &mut Engine, this: Ptr, object: Ptr, _unused_1: u32) -> f32 {
    let offset = i32::from(e.mem.i8(object.addr() + 5));
    let node = object.addr().wrapping_add(offset as u32);
    fn_00df7230(e, this, Ptr::new(node))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_7190, fn_00df7190(Ptr, u32) -> u32),
        entry!(0x00df_71a0, fn_00df71a0(Ptr, u32) -> u32),
        entry!(0x00df_71b0, deleting_destructor_00df71b0(Ptr, u8) -> Ptr),
        entry!(0x00df_71d0, deleting_destructor_00df71d0(Ptr, u8) -> Ptr),
        entry!(0x00df_71c0, fn_00df71c0(Ptr) -> f32),
        entry!(0x00df_7210, fn_00df7210(Ptr, Ptr) -> Ptr),
        entry!(0x00df_7230, fn_00df7230(Ptr, Ptr) -> f32),
        entry!(0x00df_72c0, fn_00df72c0(Ptr, Ptr) -> f32),
        entry!(0x00df_72e0, fn_00df72e0(Ptr, Ptr) -> f32),
        entry!(0x00df_7300, fn_00df7300(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00df_7320, fn_00df7320(Ptr, Ptr, u32) -> f32),
    ]
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
    fn thunk_00df7190() {
        assert_eq!(check(0x00df_7190, 0x00df_71d0, 0x4), 0xdead_beef);
    }

    #[test]
    fn thunk_00df71a0() {
        assert_eq!(check(0x00df_71a0, 0x00df_71d0, 0x8), 0xdead_beef);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;
    const FLAG_SLOT_DOUBLE: u32 = 0x0900_0021;
    const TABLE_DOUBLE: u32 = 0x0900_0031;
    const ROUTER: u32 = 0x00c8_5750;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// The deleting destructor stores its four vtables, and frees the block (0x90 bytes) only
    /// when bit 0 of the flags is set; `this` is returned moved back by 0xc.
    #[test]
    fn deleting_destructor_00df71b0_frees_only_when_bit_zero_is_set() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let block = e.mem.alloc(0x90);
        let got = e
            .call(0x00df_71b0, &args![block + 0xc, 0xffff_fffeu32])
            .u32();
        assert_eq!(got, block);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(block), 0x010c_3bb8);
        assert_eq!(e.mem.u32(block + 4), 0x010c_cf8c);
        assert_eq!(e.mem.u32(block + 8), 0x010c_cfa0);
        assert_eq!(e.mem.u32(block + 0xc), 0x010c_cfac);
        let got = e.call(0x00df_71b0, &args![block + 0xc, 1u32]).u32();
        assert_eq!(got, block);
        assert_eq!(*log.borrow(), vec![vec![allocator, block, 0x90]]);
    }

    /// The float at +4 of the object at +0x10 comes back; a signalling NaN comes back quiet.
    #[test]
    fn fn_00df71c0_returns_the_float() {
        let mut e = Engine::new();
        let inner = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, inner);
        e.mem.set_f32(inner + 4, 2.5);
        assert_eq!(e.call(0x00df_71c0, &args![this]).f32(), 2.5);
        e.mem.set_u32(inner + 4, 0x7f80_0001);
        let got = e.call(0x00df_71c0, &args![this]).f32();
        assert_eq!(got.to_bits() & 0x7fff_ffff, 0x7fc0_0001);
    }

    /// The flag is set exactly when the signed count is above zero.
    #[test]
    fn fn_00df7210_is_greater_than_zero() {
        let mut e = Engine::new();
        let inner = e.mem.alloc(0x20);
        let this = e.mem.alloc(0x20);
        let out = e.mem.alloc(4);
        e.mem.set_u32(this + 0x10, inner);
        for (count, expected) in [(1i32, 1u8), (0, 0), (-5, 0), (i32::MAX, 1)] {
            e.mem.set_i32(inner + 0x14, count);
            e.mem.set_u8(out, 0xaa);
            assert_eq!(e.call(0x00df_7210, &args![this, out]).u32(), out);
            assert_eq!(e.mem.u8(out), expected);
        }
    }

    struct Fixture {
        e: Engine,
        this: u32,
        node: u32,
        previous: u32,
        holder: u32,
        target: u32,
        flag: u32,
        log: Log,
    }

    /// An object at `this` whose float is 1.5 at `target + 4`. The object at `this + 0x10`,
    /// `+ 8` has a vtable whose slot 4 is a double that records its words (tagged `0xf1a9`,
    /// then the arguments and the word the first argument points to) and returns the address
    /// of the byte `flag`. The table at `[this + 0x20] + 0x9ac` has a double at entry 2, which
    /// the selector byte picks.
    fn fixture() -> Fixture {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let this = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x10);
        e.mem.set_f32(target + 4, 1.5);
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record + 0xc, 3);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, record);
        let previous = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, FLAG_SLOT_DOUBLE);
        let holder_base = e.mem.alloc(0x20);
        e.mem.set_u32(holder_base + 8, vtable);
        let flag = e.mem.alloc(4);
        let flag_log = Rc::clone(&log);
        e.register_double(FLAG_SLOT_DOUBLE, move |e, a| {
            let mut words = vec![0xf1a9];
            words.extend_from_slice(a);
            words.push(e.mem.u32(a[1]));
            flag_log.borrow_mut().push(words);
            Ret {
                eax: flag,
                ..Ret::default()
            }
        });
        let table = e.mem.alloc(0x1000);
        e.mem.set_u32(this + 0x10, holder_base);
        e.mem.set_u32(this + 0x14, target);
        e.mem.set_u32(this + 0x18, previous);
        e.mem.set_u32(this + 0x1c, 1);
        e.mem.set_u32(this + 0x20, table);
        e.mem.set_u8(table + ((1 + 0xd) << 5) + 3, 2);
        let table_log = Rc::clone(&log);
        e.register_double(TABLE_DOUBLE, move |_, a| {
            let mut words = vec![0x7ab1];
            words.extend_from_slice(a);
            table_log.borrow_mut().push(words);
            Ret::default()
        });
        e.mem.set_u32(table + 0x9ac + 2 * 0x14, TABLE_DOUBLE);
        Fixture {
            e,
            this,
            node,
            previous,
            holder: holder_base + 8,
            target,
            flag,
            log,
        }
    }

    /// A null record: only the float comes back, nothing is asked.
    #[test]
    fn fn_00df7230_null_record_returns_the_float_only() {
        let mut f = fixture();
        f.e.mem.set_u32(f.node, 0);
        let got = f.e.call(0x00df_7230, &args![f.this, f.node]).f32();
        assert_eq!(got, 1.5);
        assert!(f.log.borrow().is_empty());
    }

    /// The node already is the pointer at +0x18: nothing is asked either.
    #[test]
    fn fn_00df7230_same_node_returns_the_float_only() {
        let mut f = fixture();
        f.e.mem.set_u32(f.this + 0x18, f.node);
        let got = f.e.call(0x00df_7230, &args![f.this, f.node]).f32();
        assert_eq!(got, 1.5);
        assert!(f.log.borrow().is_empty());
    }

    /// A zero flag byte: the question is asked, the table is not called.
    #[test]
    fn fn_00df7230_zero_flag_skips_the_table_call() {
        let mut f = fixture();
        f.e.mem.set_u8(f.flag, 0);
        let got = f.e.call(0x00df_7230, &args![f.this, f.node]).f32();
        assert_eq!(got, 1.5);
        let log = f.log.borrow();
        assert_eq!(log.len(), 1);
        // Tag, this (the holder), the address of the stack word, the previous node, the
        // node, and the node the stack word held.
        assert_eq!(log[0][0], 0xf1a9);
        assert_eq!(log[0][1], f.holder);
        assert_eq!(log[0][3..], [f.previous, f.node, f.node]);
    }

    /// A non-zero flag byte: the table entry the selector byte picks is called with the five
    /// words, and the float is read after it.
    #[test]
    fn fn_00df7230_nonzero_flag_calls_the_table_entry() {
        let mut f = fixture();
        f.e.mem.set_u8(f.flag, 1);
        let got = f.e.call(0x00df_7230, &args![f.this, f.node]).f32();
        assert_eq!(got, 1.5);
        let log = f.log.borrow();
        let calls: Vec<_> = log.iter().filter(|w| w[0] == 0x7ab1).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0][1..],
            [f.previous, f.node, f.this + 0x20, f.target, 0]
        );
    }

    /// The table entry gets the word at `this + 0x18` as it is after the slot-4 call (read again
    /// at 00df727c), not the one the call was given.
    #[test]
    fn fn_00df7230_passes_the_word_read_after_the_slot_call() {
        let mut f = fixture();
        f.e.mem.set_u8(f.flag, 1);
        let (this, flag) = (f.this, f.flag);
        f.e.register_double(FLAG_SLOT_DOUBLE, move |e, _| {
            e.mem.set_u32(this + 0x18, 0x0123_4560);
            Ret {
                eax: flag,
                ..Ret::default()
            }
        });
        f.e.call(0x00df_7230, &args![f.this, f.node]);
        let log = f.log.borrow();
        let calls: Vec<_> = log.iter().filter(|w| w[0] == 0x7ab1).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][1..3], [0x0123_4560, f.node]);
    }

    /// The flag is written into the slot that holds `node` and tested through the address the
    /// slot answers while the slot is still there. With the freed-block check a slot freed
    /// before the test would read as the fill word's low byte, calling the table by chance.
    #[test]
    fn fn_00df7230_tests_the_flag_in_a_live_slot() {
        for (byte, table_calls) in [(0u8, 0usize), (1, 1)] {
            let mut f = fixture();
            f.e.mem.enable_freed_check(true);
            f.e.register_double(FLAG_SLOT_DOUBLE, move |e, a| {
                e.mem.set_u8(a[1], byte);
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
            let got = f.e.call(0x00df_7230, &args![f.this, f.node]).f32();
            assert_eq!(got, 1.5);
            let log = f.log.borrow();
            let calls = log.iter().filter(|w| w[0] == 0x7ab1).count();
            assert_eq!(calls, table_calls, "flag {byte}");
        }
    }

    /// The thunks pass the adjusted `this` and node on to `00df7230`.
    #[test]
    fn thunks_adjust_this_and_node() {
        let mut f = fixture();
        f.e.mem.set_u8(f.flag, 0);
        // 00df72c0: the node is the word at node + 0x20, this is moved back by 8.
        let outer = f.e.mem.alloc(0x40);
        f.e.mem.set_u32(outer + 0x20, f.node);
        let got = f.e.call(0x00df_72c0, &args![f.this + 8, outer]).f32();
        assert_eq!(got, 1.5);
        assert_eq!(f.log.borrow().len(), 1);
        assert_eq!(f.log.borrow()[0][1], f.holder);
        f.log.borrow_mut().clear();
        // 00df72e0: the word at node + 0xc, this moved back by 0xc.
        let outer = f.e.mem.alloc(0x40);
        f.e.mem.set_u32(outer + 0xc, f.node);
        let got = f.e.call(0x00df_72e0, &args![f.this + 0xc, outer]).f32();
        assert_eq!(got, 1.5);
        assert_eq!(f.log.borrow().len(), 1);
        f.log.borrow_mut().clear();
        // 00df7300: node passed, `out` byte set to 1, `out` returned.
        let out = f.e.mem.alloc(4);
        f.e.mem.set_u8(out, 0);
        let got = f.e.call(0x00df_7300, &args![f.this + 4, out, f.node]).u32();
        assert_eq!(got, out);
        assert_eq!(f.e.mem.u8(out), 1);
        assert_eq!(f.log.borrow().len(), 1);
        f.log.borrow_mut().clear();
        // 00df7320: the node is the object plus the signed byte at +5; `this` is unchanged.
        let block = f.e.mem.alloc(0x80);
        let object = block + 0x40;
        f.e.mem.set_i8(object + 5, -16);
        let record = f.e.mem.u32(f.node);
        f.e.mem.set_u32(object - 16, record);
        let got = f.e.call(0x00df_7320, &args![f.this, object, 0u32]).f32();
        assert_eq!(got, 1.5);
        assert_eq!(f.log.borrow().len(), 1);
        assert_eq!(f.log.borrow()[0][1], f.holder);
        assert_eq!(f.log.borrow()[0][5], object - 16);
    }

    /// The body at `00df71d0` is an entry of its own (`this` as given, not moved back), and the
    /// unit's two older thunks reach it through the engine.
    #[test]
    fn deleting_destructor_00df71d0_is_an_entry_of_its_own() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let block = e.mem.alloc(0x90);
        let got = e.call(0x00df_71d0, &args![block, 0u32]).u32();
        assert_eq!(got, block);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(block), 0x010c_3bb8);
        assert_eq!(e.mem.u32(block + 0xc), 0x010c_cfac);
        let got = e.call(0x00df_71d0, &args![block, 1u32]).u32();
        assert_eq!(got, block);
        assert_eq!(*log.borrow(), vec![vec![allocator, block, 0x90]]);
        log.borrow_mut().clear();
        // `00df7190` moves `this` back by 4 and `00df71a0` by 8, then run the same body.
        assert_eq!(e.call(0x00df_7190, &args![block + 4, 1u32]).u32(), block);
        assert_eq!(e.call(0x00df_71a0, &args![block + 8, 1u32]).u32(), block);
        assert_eq!(
            *log.borrow(),
            vec![vec![allocator, block, 0x90], vec![allocator, block, 0x90]]
        );
    }
}
