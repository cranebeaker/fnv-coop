//! `Havok SDK/run_00e01230` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e012f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_9f40`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e012f0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_9f40, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e012b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00e012f0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00e012b0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00e0_12f0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00e012c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0xc` (`SUB ECX,0xc`) and jumps to `00e012f0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00e012c0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00e0_12f0, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00e012d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to `00e012f0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00e012d0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00e0_12f0, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 00e012e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x14` (`SUB ECX,0x14`) and jumps to `00e012f0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00e012e0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00e0_12f0, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

// Translated from 00e01230 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): builds a temporary object in a 0x40-byte frame with
/// the constructor at `00e01250` (`this` = the frame, one argument 0) and returns the word at the
/// start of the object, the vtable pointer the constructor stored there. The temporary is not
/// destroyed.
pub fn fn_00e01230(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, record| {
        e.call(0x00e0_1250, &args![record, 0u32]);
        e.mem.u32(record.addr())
    })
}

// Translated from 00e01370 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_e25c.
pub fn return_constant_00e01370(_e: &mut Engine) -> u32 {
    0x010c_e25c
}

// Translated from 00e013e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6ff8.
pub fn return_constant_00e013e0(_e: &mut Engine) -> u32 {
    0x010c_6ff8
}

// Translated from 00e01440 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_70b4.
pub fn return_constant_00e01440(_e: &mut Engine) -> u32 {
    0x010c_70b4
}

// Translated from 00e01490 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x30-byte frame stores the vtable word 0x010e_929c at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x2c (argument 0) and
/// returns the word at the start of the frame. Nothing is destroyed.
pub fn fn_00e01490(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010e_929c);
        e.call(0x00cb_0a70, &args![base + 0x2c, 0u32]);
        e.mem.u32(base)
    })
}

// Translated from 00e01250 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`). It writes the words at `+8`, `+0xc`, `+0x10` and `+0x14` with the
/// base-class values (`010c3b80`, `010c3b98`, `010c3bac`, `010c3b8c`), then the vtable word at
/// `+0` (`010e90e8`) and the four words again with the final values (`010e90dc`, `010e90c8`,
/// `010e90bc`, `010e90b0`); when `mode` is not zero it sets `+0x20` to 3. Returns `this`
/// (`EAX`). The class is unconfirmed.
pub fn fn_00e01250(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x8, 0x010c_3b80);
    e.mem.set_u32(base + 0xc, 0x010c_3b98);
    e.mem.set_u32(base + 0x10, 0x010c_3bac);
    e.mem.set_u32(base + 0x14, 0x010c_3b8c);
    e.mem.set_u32(base, 0x010e_90e8);
    e.mem.set_u32(base + 0x8, 0x010e_90dc);
    e.mem.set_u32(base + 0xc, 0x010e_90c8);
    e.mem.set_u32(base + 0x10, 0x010e_90bc);
    e.mem.set_u32(base + 0x14, 0x010e_90b0);
    if mode != 0 {
        e.mem.set_u32(base + 0x20, 3);
    }
    this
}

// Translated from 00e011f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc6199.
pub fn fn_00e011f0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e01210 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00e01250` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc618f.
pub fn fn_00e01210(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        fn_00e01250(e, object, extra);
    }
}

// Translated from 00e01320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc6219.
pub fn fn_00e01320(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e01340 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped); a
/// constructor chain the compiler collapsed. When `object` is not null it writes the vtable word
/// `010305a8` at its start, the word 0 at `+0xc` when `mode` is not zero, then the vtable word
/// `010ce25c`, and `0x5` at `+0xc` whatever `mode` is; returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc620f.
pub fn fn_00e01340(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x0103_05a8);
        if mode != 0 {
            e.mem.set_u32(base + 0xc, 0);
        }
        e.mem.set_u32(base, 0x010c_e25c);
        e.mem.set_u32(base + 0xc, 0x5);
    }
}

// Translated from 00e01380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc6289.
pub fn fn_00e01380(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e013a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped); a
/// constructor chain the compiler collapsed. When `object` is not null it writes the vtable word
/// `010305a8` at its start, the word 0 at `+0xc` when `mode` is not zero, then the vtable word
/// `010c6ff8`, the word `010c6fd0` at `+0x14`, and `0x1b` at `+0xc` whatever `mode` is; returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc627f.
pub fn fn_00e013a0(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x0103_05a8);
        if mode != 0 {
            e.mem.set_u32(base + 0xc, 0);
        }
        e.mem.set_u32(base, 0x010c_6ff8);
        e.mem.set_u32(base + 0x14, 0x010c_6fd0);
        e.mem.set_u32(base + 0xc, 0x1b);
    }
}

// Translated from 00e013f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc62f9.
pub fn fn_00e013f0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e01410 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped); a
/// constructor chain the compiler collapsed. When `object` is not null it writes the vtable word
/// `010305a8` at its start, the word 0 at `+0xc` when `mode` is not zero, then the vtable word
/// `010c70b4`, and `0x4` at `+0xc` whatever `mode` is; returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc62ef.
pub fn fn_00e01410(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x0103_05a8);
        if mode != 0 {
            e.mem.set_u32(base + 0xc, 0);
        }
        e.mem.set_u32(base, 0x010c_70b4);
        e.mem.set_u32(base + 0xc, 0x4);
    }
}

// Translated from 00e01450 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc6369.
pub fn fn_00e01450(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e01470 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null it writes the vtable word `010e929c` at its start and runs `00cb0a70` (an
/// `hkStringPtr` function) on the member at `object + 0x2c` with `extra`; returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc635f.
pub fn fn_00e01470(e: &mut Engine, object: Ptr, extra: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x010e_929c);
        e.call(0x00cb_0a70, &args![base + 0x2c, extra]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_12f0, deleting_destructor_00e012f0(Ptr, u8) -> Ptr),
        entry!(0x00e0_12b0, fn_00e012b0(Ptr, u32) -> u32),
        entry!(0x00e0_12c0, fn_00e012c0(Ptr, u32) -> u32),
        entry!(0x00e0_12d0, fn_00e012d0(Ptr, u32) -> u32),
        entry!(0x00e0_12e0, fn_00e012e0(Ptr, u32) -> u32),
        entry!(0x00e01230, fn_00e01230() -> u32),
        entry!(0x00e01370, return_constant_00e01370() -> u32),
        entry!(0x00e013e0, return_constant_00e013e0() -> u32),
        entry!(0x00e01440, return_constant_00e01440() -> u32),
        entry!(0x00e01490, fn_00e01490() -> u32),
        entry!(0x00e0_1250, fn_00e01250(Ptr, u32) -> Ptr),
        entry!(0x00e0_11f0, fn_00e011f0(Ptr)),
        entry!(0x00e0_1210, fn_00e01210(Ptr, u32)),
        entry!(0x00e0_1320, fn_00e01320(Ptr)),
        entry!(0x00e0_1340, fn_00e01340(Ptr, u32)),
        entry!(0x00e0_1380, fn_00e01380(Ptr)),
        entry!(0x00e0_13a0, fn_00e013a0(Ptr, u32)),
        entry!(0x00e0_13f0, fn_00e013f0(Ptr)),
        entry!(0x00e0_1410, fn_00e01410(Ptr, u32)),
        entry!(0x00e0_1450, fn_00e01450(Ptr)),
        entry!(0x00e0_1470, fn_00e01470(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00e012f0_frees_only_when_bit_zero_is_set() {
        check(0x00e0_12f0, 0x00e4_9f40);
    }
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
    fn thunk_00e012b0() {
        assert_eq!(check(0x00e0_12b0, 0x00e0_12f0, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00e012c0() {
        assert_eq!(check(0x00e0_12c0, 0x00e0_12f0, 0xc), 0xdead_beef);
    }

    #[test]
    fn thunk_00e012d0() {
        assert_eq!(check(0x00e0_12d0, 0x00e0_12f0, 0x10), 0xdead_beef);
    }

    #[test]
    fn thunk_00e012e0() {
        assert_eq!(check(0x00e0_12e0, 0x00e0_12f0, 0x14), 0xdead_beef);
    }
}

#[cfg(test)]
mod record_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type ConstructorLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Double for a constructor: logs its argument words and writes `vtable` at the start of `this`.
    fn constructor_double(e: &mut Engine, address: u32, vtable: u32) -> ConstructorLog {
        let log: ConstructorLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], vtable);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    /// What the probe saw on one call: `this`, the argument, the word `behind` bytes before `this`
    /// (the frame's vtable slot) and, when asked for, the `u32` at +0x20, `u16` at +0x24 and `u16`
    /// at +0x26 of the frame.
    type ProbeCall = (u32, u32, u32, [u32; 3]);
    type ProbeLog = Rc<RefCell<Vec<ProbeCall>>>;

    /// Double for the member constructor: logs, per call, what `ProbeCall` describes, with the
    /// vtable slot `behind[call]` bytes before `this`.
    fn frame_probe(e: &mut Engine, address: u32, behind: &[u32], with_tail: bool) -> ProbeLog {
        let log: ProbeLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        let behind = behind.to_vec();
        e.register_double(address, move |e, a| {
            let call = seen.borrow().len();
            let slot = a[0] - behind[call];
            let mut tail = [0; 3];
            if with_tail {
                tail = [
                    e.mem.u32(slot + 0x20),
                    u32::from(e.mem.u16(slot + 0x24)),
                    u32::from(e.mem.u16(slot + 0x26)),
                ];
            }
            seen.borrow_mut().push((a[0], a[1], e.mem.u32(slot), tail));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn fn_00e01230_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_1250, 0x0a0b_0c0d);
        let result = e.call(0x00e0_1230, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e01370_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1370, &args![]).u32(), 0x010c_e25c);
    }

    #[test]
    fn return_constant_00e013e0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_13e0, &args![]).u32(), 0x010c_6ff8);
    }

    #[test]
    fn return_constant_00e01440_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1440, &args![]).u32(), 0x010c_70b4);
    }

    #[test]
    fn fn_00e01490_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x2c], false);
        let result = e.call(0x00e0_1490, &args![]).u32();
        assert_eq!(result, 0x010e_929c);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010e_929c);
        }
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// With mode 0 the final vtable words are in place and +0x20 is untouched; with a mode
    /// +0x20 becomes 3.
    #[test]
    fn fn_00e01250_writes_the_final_vtable_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.mem.write(this, &[0x55; 0x30]);
        assert_eq!(e.call(0x00e0_1250, &args![this, 0u32]).u32(), this);
        let words = [
            0x010e_90e8,
            0x010e_90dc,
            0x010e_90c8,
            0x010e_90bc,
            0x010e_90b0,
        ];
        for (offset, word) in [0u32, 8, 0xc, 0x10, 0x14].into_iter().zip(words) {
            assert_eq!(e.mem.u32(this + offset), word);
        }
        assert_eq!(e.mem.u32(this + 4), 0x5555_5555);
        assert_eq!(e.mem.u32(this + 0x20), 0x5555_5555);
        e.call(0x00e0_1250, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this + 0x20), 3);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Address the tests give to a virtual function (slot 0 of the object's vtable).
    const SLOT_DOUBLE: u32 = 0x0900_0011;

    /// Double over `address` that logs the words of each call.
    fn logging_double(e: &mut Engine, address: u32) -> CallLog {
        let log: CallLog = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log
    }

    /// Slot 0 of the object's vtable is a double: it runs once with `(object, 0)`.
    fn check_cleanup(address: u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, SLOT_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let log = logging_double(&mut e, SLOT_DOUBLE);
        e.call(address, &args![object]);
        assert_eq!(*log.borrow(), vec![vec![object, 0]]);
    }

    /// With either `mode` the object ends with the final vtable word, the extra words and `value`
    /// at `+0xc`; the words around them stay as they were.
    fn check_staged(address: u32, vtable: u32, extras: &[(u32, u32)], value: u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(address, &args![0u32, 1u32]);
        for mode in [0u32, 1] {
            e.mem.write(object, &[0x66; 0x40]);
            e.call(address, &args![object, mode]);
            assert_eq!(e.mem.u32(object), vtable);
            assert_eq!(e.mem.u32(object + 0xc), value);
            for offset in [4, 8, 0x10, 0x14, 0x18] {
                let expected = extras
                    .iter()
                    .find(|(at, _)| *at == offset)
                    .map_or(0x6666_6666, |(_, word)| *word);
                assert_eq!(e.mem.u32(object + offset), expected);
            }
        }
    }

    /// The vtable word is in place when the member constructor runs on `object + 0x2c` with `extra`.
    fn check_member(address: u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.register_double(0x00cb_0a70, move |e, a| {
            assert_eq!(a, [object + 0x2c, 7]);
            assert_eq!(e.mem.u32(object), 0x010e_929c);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(address, &args![0u32, 7u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(address, &args![object, 7u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn fn_00e011f0_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_11f0);
    }

    #[test]
    fn fn_00e01320_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_1320);
    }

    #[test]
    fn fn_00e01340_writes_the_final_vtable_and_the_value() {
        check_staged(0x00e0_1340, 0x010c_e25c, &[], 0x5);
    }

    #[test]
    fn fn_00e01380_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_1380);
    }

    #[test]
    fn fn_00e013a0_writes_the_final_vtable_and_the_value() {
        check_staged(0x00e0_13a0, 0x010c_6ff8, &[(0x14, 0x010c_6fd0)], 0x1b);
    }

    #[test]
    fn fn_00e013f0_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_13f0);
    }

    #[test]
    fn fn_00e01410_writes_the_final_vtable_and_the_value() {
        check_staged(0x00e0_1410, 0x010c_70b4, &[], 0x4);
    }

    #[test]
    fn fn_00e01450_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_1450);
    }

    #[test]
    fn fn_00e01470_stores_the_vtable_then_constructs_the_member() {
        check_member(0x00e0_1470);
    }

    #[test]
    fn fn_00e01210_runs_the_collapsed_constructor_for_a_non_null_object() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x30);
        e.call(0x00e0_1210, &args![0u32, 5u32]);
        e.mem.write(object, &[0x55; 0x30]);
        e.call(0x00e0_1210, &args![object, 0u32]);
        let words = [
            0x010e_90e8,
            0x010e_90dc,
            0x010e_90c8,
            0x010e_90bc,
            0x010e_90b0,
        ];
        for (offset, word) in [0u32, 8, 0xc, 0x10, 0x14].into_iter().zip(words) {
            assert_eq!(e.mem.u32(object + offset), word);
        }
        assert_eq!(e.mem.u32(object + 0x20), 0x5555_5555);
        e.call(0x00e0_1210, &args![object, 5u32]);
        assert_eq!(e.mem.u32(object + 0x20), 3);
    }
}
