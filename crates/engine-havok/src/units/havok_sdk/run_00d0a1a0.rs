//! `Havok SDK/run_00d0a1a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d0a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_8e80`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d0a240(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_8e80, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d0a1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x4 (`SUB ECX,0x4`) and jumps to `00d0a1a0`. `00d0a1a0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0a1d0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_a1a0, &args![this.addr().wrapping_sub(0x4), flags])
        .u32()
}

// Translated from 00d0a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x14 (`SUB ECX,0x14`) and jumps to `00d0a240`. `00d0a240` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0a200(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_a240, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

// Translated from 00d0a210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x18 (`SUB ECX,0x18`) and jumps to `00d0a240`. `00d0a240` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0a210(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_a240, &args![this.addr().wrapping_sub(0x18), flags])
        .u32()
}

// Translated from 00d0a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x1c (`SUB ECX,0x1c`) and jumps to `00d0a240`. `00d0a240` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0a220(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_a240, &args![this.addr().wrapping_sub(0x1c), flags])
        .u32()
}

// Translated from 00d0a230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d0a240`. `00d0a240` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0a230(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_a240, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d0a1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): writes the two vtable words (`+4` = `0x010c_cfac`,
/// then `+0` = `0x010c_cfa0`) and, when bit 0 of `flags` is set, calls `00401030` on `this`.
/// Returns `this`. Reached through the thunk `00d0a1d0` (`this - 4`).
pub fn fn_00d0a1a0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr() + 4, 0x010c_cfac);
    e.mem.set_u32(this.addr(), 0x010c_cfa0);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00d0a1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter: the byte at `+0x68` (`__fastcall`, result in `AL`).
pub fn fn_00d0a1e0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x68)
}

// Translated from 00d0a1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Setter: stores the low byte of the stack word at `+0x68`.
pub fn fn_00d0a1f0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x68, value);
}

// Translated from 00d0a290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0xc` to 1. The stack word (`RET 4`) is never read.
pub fn fn_00d0a290(e: &mut Engine, this: Ptr, _unused_1: u32) {
    e.mem.set_u8(this.addr() + 0xc, 1);
}

// Translated from 00d0a2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing (`RET 4`): a virtual method that ignores `this` and its stack word.
pub fn fn_00d0a2a0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d0a2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+8` to 1 when the word at `other + 0x10` is not zero.
pub fn fn_00d0a2b0(e: &mut Engine, this: Ptr, other: Ptr) {
    if e.mem.u32(other.addr() + 0x10) != 0 {
        e.mem.set_u8(this.addr() + 8, 1);
    }
}

// Translated from 00d0a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: `+6` (`u16`) = 1, `+0xc` = `+0x10` = 0, `+0x1c` = `owner`, three vtable words
/// (`+0` = `0x010c_d278`, `+0x14` = `0x010c_d25c`, `+0x18` = `0x010c_d244`; the exe first writes
/// `0x0102_f13c` and `0x0104_e3d0` to the last two), `+0x20` = 1, `+0x21` = `flag`, `+8` = 0;
/// then calls `00c93820` and `00c93860` on `owner` with the addresses `this + 0x14` and
/// `this + 0x18`. Returns `this`.
pub fn fn_00d0a2d0(e: &mut Engine, this: Ptr, owner: u32, flag: u8) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0x0102_f13c);
    e.mem.set_u32(base + 0x18, 0x0104_e3d0);
    e.mem.set_u32(base + 0x1c, owner);
    e.mem.set_u32(base, 0x010c_d278);
    e.mem.set_u32(base + 0x14, 0x010c_d25c);
    e.mem.set_u32(base + 0x18, 0x010c_d244);
    e.mem.set_u8(base + 0x20, 1);
    e.mem.set_u8(base + 0x21, flag);
    e.mem.set_u32(base + 8, 0);
    let owner = e.mem.u32(base + 0x1c);
    e.call(0x00c9_3820, &args![owner, base + 0x14]);
    let owner = e.mem.u32(base + 0x1c);
    e.call(0x00c9_3860, &args![owner, base + 0x18]);
    this
}

// Translated from 00d0a340 (decompiled, FalloutNV.exe 1.4.0.525)
/// If the byte at `+0xd` is zero, sets the byte at `+0xc` to 1; otherwise jumps to `00d0a270`
/// with `this - 0x14` and the stack word unchanged (a thunk that adds 0x10 to the word, loads
/// `this + 0x10` and jumps to `00d97710`).
pub fn fn_00d0a340(e: &mut Engine, this: Ptr, value: u32) {
    if e.mem.u8(this.addr() + 0xd) == 0 {
        e.mem.set_u8(this.addr() + 0xc, 1);
        return;
    }
    e.call(0x00d0_a270, &args![this.addr().wrapping_sub(0x14), value]);
}

// Translated from 00d0a360 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the word at `other + 0x10` is not zero: if the byte at `+9` is set, jumps to `00d0a270`
/// with `this - 0x18` and `other`; otherwise sets the byte at `+8` to 1.
pub fn fn_00d0a360(e: &mut Engine, this: Ptr, other: Ptr) {
    if e.mem.u32(other.addr() + 0x10) == 0 {
        return;
    }
    if e.mem.u8(this.addr() + 9) != 0 {
        e.call(0x00d0_a270, &args![this.addr().wrapping_sub(0x18), other]);
        return;
    }
    e.mem.set_u8(this.addr() + 8, 1);
}

// Translated from 00d0a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds a 0x18-byte local (vtable `0x010c_cfb8`, `+4` = `first`, `+8`/`+0xc` = `second + 0x14` /
/// `second + 0x10` or 0 for a null `second`, `+0x10` = `third`, `+0x14` = 0) and calls `00d98ca0`
/// (cdecl) with the address of the stack word `third`, the word at `this + 0x10`, `first`,
/// `first + 0x10` and the local.
pub fn fn_00d0a390(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) {
    e.with_stack(0x18, |e, local| {
        let l = local.addr();
        e.mem.set_u32(l + 0x10, third);
        e.mem.set_u32(l + 0x14, 0);
        e.mem.set_u32(l, 0x010c_cfb8);
        e.mem.set_u32(l + 4, first);
        if second != 0 {
            e.mem.set_u32(l + 8, second.wrapping_add(0x14));
            e.mem.set_u32(l + 0xc, second.wrapping_add(0x10));
        } else {
            e.mem.set_u32(l + 8, 0);
            e.mem.set_u32(l + 0xc, 0);
        }
        let field = e.mem.u32(this.addr() + 0x10);
        // The first argument is the address of the stack word that held `third`.
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), third);
            e.call(
                0x00d9_8ca0,
                &args![slot, field, first, first.wrapping_add(0x10), local],
            );
        });
    });
}

// Translated from 00d0a400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same call as `00d0a390` with a larger local frame: the 0x18-byte local (vtable
/// `0x010c_cfb8`, `+4` = `first`, `+8`/`+0xc` from `second` as there, `+0x10` = the address of a
/// buffer in the frame, `+0x14` = `third`) and, in that buffer, `1.0` at `+0x10`, `-1` at `+0x14`
/// and `+0x20`, 0 at `+0x40` and `+0x50`; the rest of the frame is left uninitialised by the exe
/// (zero here). `00d98ca0` gets the address of an uninitialised byte of the frame, the word at
/// `this + 0x10`, `first`, `first + 0x10` and the local.
pub fn fn_00d0a400(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) {
    e.with_stack(0x90, |e, frame| {
        let s = frame.addr();
        e.mem.set_u32(s + 0x24, third);
        e.mem.set_u32(s + 0x10, 0x010c_cfb8);
        e.mem.set_u32(s + 0x14, first);
        if second != 0 {
            e.mem.set_u32(s + 0x18, second.wrapping_add(0x14));
            e.mem.set_u32(s + 0x1c, second.wrapping_add(0x10));
        } else {
            e.mem.set_u32(s + 0x18, 0);
            e.mem.set_u32(s + 0x1c, 0);
        }
        // FLD1 / FSTP.
        e.mem.set_u32(s + 0x3c, 0xffff_ffff);
        e.mem.set_u32(s + 0x38, 1.0f32.to_bits());
        e.mem.set_u32(s + 0x48, 0xffff_ffff);
        e.mem.set_u32(s + 0x20, s + 0x28);
        let field = e.mem.u32(this.addr() + 0x10);
        e.mem.set_u32(s + 0x68, 0);
        e.mem.set_u32(s + 0x78, 0);
        e.call(
            0x00d9_8ca0,
            &args![s + 0xf, field, first, first.wrapping_add(0x10), s + 0x10],
        );
    });
}

// Translated from 00d0a490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds a local object in a 0x90-byte frame (vtable word `0x010c_cfcc` at `+0`, 0 at `+0x24`,
/// the float at `0104da4c` copied to `+0x80`; the rest uninitialised in the exe, zero here) and
/// calls `00d984b0` on it with the word at `this + 0x10` and the seven stack words.
pub fn fn_00d0a490(
    e: &mut Engine,
    this: Ptr,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) {
    e.with_stack(0x90, |e, frame| {
        let s = frame.addr();
        let bits = x87_quiet_bits(e.mem.u32(0x0104_da4c));
        e.mem.set_u32(s + 0x80, bits);
        let field = e.mem.u32(this.addr() + 0x10);
        e.mem.set_u32(s, 0x010c_cfcc);
        e.mem.set_u32(s + 0x24, 0);
        e.call(
            0x00d9_84b0,
            &args![frame, field, a1, a2, a3, a4, a5, a6, a7],
        );
    });
}

// Translated from 00d0a4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor (`__fastcall`): writes the three vtable words (`+0`, `+0x14`, `+0x18`), calls
/// `00c917a0` and `00c917f0` on the word at `+0x1c` with `this + 0x14` and `this + 0x18`; when
/// the word at `+0x10` is not null destroys it (`00d97850`, an `hkKdTree` destructor per the
/// decompiler) and frees it (0x50 bytes) through slot 8 of the memory router's allocator; then
/// writes the base vtable words (`0x0102_e388`, `0x0104_e3d0`, `0x0102_f13c`).
pub fn fn_00d0a4f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010c_d278);
    e.mem.set_u32(base + 0x14, 0x010c_d25c);
    e.mem.set_u32(base + 0x18, 0x010c_d244);
    let owner = e.mem.u32(base + 0x1c);
    e.call(0x00c9_17a0, &args![owner, base + 0x14]);
    let owner = e.mem.u32(base + 0x1c);
    e.call(0x00c9_17f0, &args![owner, base + 0x18]);
    let tree = e.mem.u32(base + 0x10);
    if tree != 0 {
        e.call(0x00d9_7850, &args![tree]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![tree, 0x50u32]);
    }
    e.mem.set_u32(base, 0x0102_e388);
    e.mem.set_u32(base + 0x18, 0x0104_e3d0);
    e.mem.set_u32(base + 0x14, 0x0102_f13c);
}

// Translated from 00d0a270 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00d0a270 to its `JMP` at 00d0a27e, inside the body Ghidra
/// gives `00d0a340`): `thiscall`, one stack word. Moves the word up by 0x10 in its stack slot
/// and jumps to `00d97710` on the object at `this + 0x10` with it (that function pops the word).
pub fn fn_00d0a270(e: &mut Engine, this: Ptr, value: u32) {
    let inner = e.mem.u32(this.addr().wrapping_add(0x10));
    e.call(0x00d9_7710, &args![inner, value.wrapping_add(0x10)]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_a240, deleting_destructor_00d0a240(Ptr, u8) -> Ptr),
        entry!(0x00d0_a270, fn_00d0a270(Ptr, u32)),
        entry!(0x00d0_a1d0, fn_00d0a1d0(Ptr, u32) -> u32),
        entry!(0x00d0_a200, fn_00d0a200(Ptr, u32) -> u32),
        entry!(0x00d0_a210, fn_00d0a210(Ptr, u32) -> u32),
        entry!(0x00d0_a220, fn_00d0a220(Ptr, u32) -> u32),
        entry!(0x00d0_a230, fn_00d0a230(Ptr, u32) -> u32),
        entry!(0x00d0_a1a0, fn_00d0a1a0(Ptr, u8) -> Ptr),
        entry!(0x00d0_a1e0, fn_00d0a1e0(Ptr) -> u8),
        entry!(0x00d0_a1f0, fn_00d0a1f0(Ptr, u8)),
        entry!(0x00d0_a290, fn_00d0a290(Ptr, u32)),
        entry!(0x00d0_a2a0, fn_00d0a2a0(Ptr, u32)),
        entry!(0x00d0_a2b0, fn_00d0a2b0(Ptr, Ptr)),
        entry!(0x00d0_a2d0, fn_00d0a2d0(Ptr, u32, u8) -> Ptr),
        entry!(0x00d0_a340, fn_00d0a340(Ptr, u32)),
        entry!(0x00d0_a360, fn_00d0a360(Ptr, Ptr)),
        entry!(0x00d0_a390, fn_00d0a390(Ptr, u32, u32, u32)),
        entry!(0x00d0_a400, fn_00d0a400(Ptr, u32, u32, u32)),
        entry!(
            0x00d0_a490,
            fn_00d0a490(Ptr, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00d0_a4f0, fn_00d0a4f0(Ptr)),
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
    fn deleting_destructor_00d0a240_frees_only_when_bit_zero_is_set() {
        check(0x00d0_a240, 0x00d0_8e80);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00d0a1d0() {
        assert_eq!(
            check(0x00d0_a1d0, 0x00d0_a1a0, 0x4, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0a200() {
        assert_eq!(
            check(0x00d0_a200, 0x00d0_a240, 0x14, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0a210() {
        assert_eq!(
            check(0x00d0_a210, 0x00d0_a240, 0x18, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0a220() {
        assert_eq!(
            check(0x00d0_a220, 0x00d0_a240, 0x1c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0a230() {
        assert_eq!(
            check(0x00d0_a230, 0x00d0_a240, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The logged calls to `addr` (argument words).
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00d0a1a0_writes_vtables_and_calls_00401030_on_bit_zero() {
        let mut e = Engine::new();
        e.register(0x0040_1030, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let r = e.call(0x00d0_a1a0, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(r, this);
        assert_eq!(e.mem.u32(this + 4), 0x010c_cfac);
        assert_eq!(e.mem.u32(this), 0x010c_cfa0);
        assert!(calls_to(&e, 0x0040_1030).is_empty());
        let r = e.call(0x00d0_a1a0, &args![this, 1u32]).u32();
        assert_eq!(r, this);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![this]]);
    }

    #[test]
    fn fn_00d0a1e0_reads_the_byte_at_0x68() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x68, 0xaabb_ccdd);
        assert_eq!(e.call(0x00d0_a1e0, &args![this]).u8(), 0xdd);
    }

    #[test]
    fn fn_00d0a1f0_stores_the_low_byte_at_0x68() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x68, 0xffff_ffff);
        e.call(0x00d0_a1f0, &args![this, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(this + 0x68), 0xffff_ff78);
    }

    #[test]
    fn fn_00d0a290_sets_the_byte_at_0xc() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.call(0x00d0_a290, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn fn_00d0a2a0_does_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00d0_a2a0, &args![this, 0x55u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.bytes(this, 0x20), vec![0u8; 0x20]);
    }

    #[test]
    fn fn_00d0a2b0_sets_the_byte_at_8_when_the_other_word_is_set() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        e.call(0x00d0_a2b0, &args![this, other]);
        assert_eq!(e.mem.u8(this + 8), 0);
        e.mem.set_u32(other + 0x10, 7);
        e.call(0x00d0_a2b0, &args![this, other]);
        assert_eq!(e.mem.u8(this + 8), 1);
    }

    #[test]
    fn fn_00d0a2d0_constructs_and_registers_with_the_owner() {
        let mut e = Engine::new();
        e.register(0x00c9_3820, |_, _| Ret::default());
        e.register(0x00c9_3860, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        for offset in (0..0x30).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        let r = e
            .call(0x00d0_a2d0, &args![this, 0x4321u32, 0x1234_5601u32])
            .u32();
        assert_eq!(r, this);
        assert_eq!(e.mem.u32(this), 0x010c_d278);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x010c_d25c);
        assert_eq!(e.mem.u32(this + 0x18), 0x010c_d244);
        assert_eq!(e.mem.u32(this + 0x1c), 0x4321);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(e.mem.u8(this + 0x21), 1);
        assert_eq!(e.mem.u32(this + 0x24), 0xffff_ffff);
        assert_eq!(calls_to(&e, 0x00c9_3820), vec![vec![0x4321, this + 0x14]]);
        assert_eq!(calls_to(&e, 0x00c9_3860), vec![vec![0x4321, this + 0x18]]);
    }

    #[test]
    fn fn_00d0a340_sets_the_flag_or_jumps_to_00d0a270() {
        let mut e = Engine::new();
        e.register(0x00d0_a270, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00d0_a340, &args![this, 0x77u32]);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert!(calls_to(&e, 0x00d0_a270).is_empty());
        e.mem.set_u8(this + 0xc, 0);
        e.mem.set_u8(this + 0xd, 1);
        e.call(0x00d0_a340, &args![this, 0x77u32]);
        assert_eq!(e.mem.u8(this + 0xc), 0);
        assert_eq!(calls_to(&e, 0x00d0_a270), vec![vec![this - 0x14, 0x77]]);
    }

    #[test]
    fn fn_00d0a360_sets_the_flag_or_jumps_to_00d0a270() {
        let mut e = Engine::new();
        e.register(0x00d0_a270, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        // Other's word is zero: nothing happens.
        e.mem.set_u8(this + 9, 1);
        e.call(0x00d0_a360, &args![this, other]);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert!(calls_to(&e, 0x00d0_a270).is_empty());
        // Set, byte at +9 set: the jump.
        e.mem.set_u32(other + 0x10, 3);
        e.call(0x00d0_a360, &args![this, other]);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert_eq!(calls_to(&e, 0x00d0_a270), vec![vec![this - 0x18, other]]);
        // Set, byte at +9 clear: the flag.
        e.mem.set_u8(this + 9, 0);
        e.call(0x00d0_a360, &args![this, other]);
        assert_eq!(e.mem.u8(this + 8), 1);
        assert_eq!(calls_to(&e, 0x00d0_a270).len(), 1);
    }

    #[test]
    fn fn_00d0a390_builds_the_record_and_calls_00d98ca0() {
        for (second, at_8, at_c) in [(0x5000u32, 0x5014u32, 0x5010u32), (0, 0, 0)] {
            let mut e = Engine::new();
            let seen = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
            let log = Rc::clone(&seen);
            e.register_double(0x00d9_8ca0, move |e, a| {
                let mut words = a.to_vec();
                words.push(e.mem.u32(a[0]));
                for offset in (0..0x18).step_by(4) {
                    words.push(e.mem.u32(a[4] + offset));
                }
                log.borrow_mut().push(words);
                Ret::default()
            });
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + 0x10, 0x7777);
            e.call(0x00d0_a390, &args![this, 0x4000u32, second, 0x99u32]);
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            let w = &seen[0];
            assert_eq!(w[1], 0x7777);
            assert_eq!(w[2], 0x4000);
            assert_eq!(w[3], 0x4010);
            assert_eq!(w[5], 0x99);
            assert_eq!(&w[6..], &[0x010c_cfb8, 0x4000, at_8, at_c, 0x99, 0]);
        }
    }

    #[test]
    fn fn_00d0a400_builds_the_frame_and_calls_00d98ca0() {
        for (second, at_8, at_c) in [(0x5000u32, 0x5014u32, 0x5010u32), (0, 0, 0)] {
            let mut e = Engine::new();
            let seen = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
            let log = Rc::clone(&seen);
            e.register_double(0x00d9_8ca0, move |e, a| {
                let mut words = a.to_vec();
                for offset in (0..0x18).step_by(4) {
                    words.push(e.mem.u32(a[4] + offset));
                }
                let buffer = e.mem.u32(a[4] + 0x10);
                for offset in [0x10, 0x14, 0x20, 0x40, 0x50] {
                    words.push(e.mem.u32(buffer + offset));
                }
                log.borrow_mut().push(words);
                Ret::default()
            });
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + 0x10, 0x7777);
            e.call(0x00d0_a400, &args![this, 0x4000u32, second, 0x99u32]);
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            let w = &seen[0];
            assert_eq!(w[0] + 1, w[4]);
            assert_eq!(&w[1..4], &[0x7777, 0x4000, 0x4010]);
            assert_eq!(w[5], 0x010c_cfb8);
            assert_eq!(w[6], 0x4000);
            assert_eq!(w[7], at_8);
            assert_eq!(w[8], at_c);
            assert_eq!(w[10], 0x99);
            let floats = [0x3f80_0000, 0xffff_ffff, 0xffff_ffff, 0, 0];
            assert_eq!(&w[11..], &floats);
        }
    }

    #[test]
    fn fn_00d0a490_builds_the_object_and_calls_00d984b0() {
        for (stored, copied) in [(0x3400_0000u32, 0x3400_0000u32), (0x7f80_0001, 0x7fc0_0001)] {
            let mut e = Engine::new();
            e.map(0x0104_d000, 0x1000);
            e.set_global(0x0104_da4c, stored);
            let seen = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
            let log = Rc::clone(&seen);
            e.register_double(0x00d9_84b0, move |e, a| {
                let mut words = a.to_vec();
                words.push(e.mem.u32(a[0]));
                words.push(e.mem.u32(a[0] + 0x24));
                words.push(e.mem.u32(a[0] + 0x80));
                log.borrow_mut().push(words);
                Ret::default()
            });
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + 0x10, 0x7777);
            e.call(
                0x00d0_a490,
                &args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32],
            );
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            assert_eq!(
                &seen[0][1..],
                &[0x7777, 1, 2, 3, 4, 5, 6, 7, 0x010c_cfcc, 0, copied]
            );
        }
    }

    #[test]
    fn fn_00d0a4f0_destroys_the_tree_and_frees_it() {
        for tree in [0x5678u32, 0] {
            let mut e = Engine::new();
            let vtable = e.mem.alloc(0x20);
            e.mem.set_u32(vtable + 8, 0x0900_0011);
            let allocator = e.mem.alloc(0x10);
            e.mem.set_u32(allocator, vtable);
            let router = e.mem.alloc(0x20);
            e.mem.set_u32(router + 0x10, allocator);
            e.register_double(0x00c8_5750, move |_, _| Ret {
                eax: router,
                ..Ret::default()
            });
            for addr in [0x00c9_17a0, 0x00c9_17f0, 0x00d9_7850, 0x0900_0011] {
                e.register(addr, |_, _| Ret::default());
            }
            let this = e.mem.alloc(0x30);
            e.mem.set_u32(this + 0x10, tree);
            e.mem.set_u32(this + 0x1c, 0x4321);
            e.call_log = Some(vec![]);
            e.call(0x00d0_a4f0, &args![this]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![
                (0x00c9_17a0, vec![0x4321, this + 0x14]),
                (0x00c9_17f0, vec![0x4321, this + 0x18]),
            ];
            if tree != 0 {
                expected.push((0x00d9_7850, vec![tree]));
                expected.push((0x00c8_5750, vec![]));
                expected.push((0x0900_0011, vec![allocator, tree, 0x50]));
            }
            assert_eq!(log[1..], expected[..]);
            assert_eq!(e.mem.u32(this), 0x0102_e388);
            assert_eq!(e.mem.u32(this + 0x14), 0x0102_f13c);
            assert_eq!(e.mem.u32(this + 0x18), 0x0104_e3d0);
        }
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_00d0a270_hands_the_raised_word_to_00d97710() {
        let mut e = Engine::new();
        e.register(0x00d9_7710, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x4444);
        e.call(0x00d0_a270, &args![this, 0x100u32]);
        let seen: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x00d9_7710)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(seen, vec![vec![0x4444, 0x110]]);
    }
}
