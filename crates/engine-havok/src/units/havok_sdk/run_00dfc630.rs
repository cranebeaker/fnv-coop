//! `Havok SDK/run_00dfc630` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::release_array;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dfd080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00df_cfd0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dfd080(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00df_cfd0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dfc630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x0102_e368.
pub fn return_constant_00dfc630(_e: &mut Engine) -> u32 {
    0x0102_e368
}

// Translated from 00dfc6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0a64.
pub fn return_constant_00dfc6d0(_e: &mut Engine) -> u32 {
    0x010d_0a64
}

// Translated from 00dfc740 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0aac.
pub fn return_constant_00dfc740(_e: &mut Engine) -> u32 {
    0x010d_0aac
}

// Translated from 00dfc910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_11ec.
pub fn return_constant_00dfc910(_e: &mut Engine) -> u32 {
    0x010e_11ec
}

// Translated from 00dfc9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0af4.
pub fn return_constant_00dfc9a0(_e: &mut Engine) -> u32 {
    0x010d_0af4
}

// Translated from 00dfca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_b71c.
pub fn return_constant_00dfca30(_e: &mut Engine) -> u32 {
    0x010c_b71c
}

// Translated from 00dfc7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00e3b060` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfc7a0(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00e3_b060, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfc830 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x170-byte stack buffer by calling `00dfc850` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfc830(e: &mut Engine) -> u32 {
    e.with_stack(0x170, |e, buffer| {
        e.call(0x00df_c850, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfcb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00e3b960` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfcb70(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00e3_b960, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00d53df0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfce10(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00d5_3df0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfcf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x50-byte stack buffer by calling `00dfcf60` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfcf40(e: &mut Engine) -> u32 {
    e.with_stack(0x50, |e, buffer| {
        e.call(0x00df_cf60, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfcaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_a04c.
pub fn return_constant_00dfcaa0(_e: &mut Engine) -> u32 {
    0x010d_a04c
}

// Translated from 00dfcb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0b3c.
pub fn return_constant_00dfcb10(_e: &mut Engine) -> u32 {
    0x010d_0b3c
}

// Translated from 00dfcc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_37e4.
pub fn return_constant_00dfcc00(_e: &mut Engine) -> u32 {
    0x010d_37e4
}

// Translated from 00dfcc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_89e4.
pub fn return_constant_00dfcc70(_e: &mut Engine) -> u32 {
    0x010c_89e4
}

// Translated from 00dfcce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0be4.
pub fn return_constant_00dfcce0(_e: &mut Engine) -> u32 {
    0x010d_0be4
}

// Translated from 00dfcd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_91ec.
pub fn return_constant_00dfcd50(_e: &mut Engine) -> u32 {
    0x010c_91ec
}

// Translated from 00dfcea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0cd4.
pub fn return_constant_00dfcea0(_e: &mut Engine) -> u32 {
    0x010d_0cd4
}

// Translated from 00dfd120 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_222c.
pub fn return_constant_00dfd120(_e: &mut Engine) -> u32 {
    0x010e_222c
}

// Translated from 00dfc850 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor (`this` in `ECX`, one stack word `extra` that the
/// callee pops). It runs `00cd2c50` on `this` with `extra`, then empties three `hkArray`-shaped
/// triples (data, size, capacity-and-flags = `0x80000000`, "not owned") at `+0x8c`, `+0x98` and
/// `+0x160`, with the vtable word at `+0` set to `010ce15c` after the second one. Returns `this`
/// (`EAX`). The class is unconfirmed; `00dfc830` builds one on the stack.
pub fn fn_00dfc850(e: &mut Engine, this: Ptr, extra: u32) -> Ptr {
    let base = this.addr();
    e.call(0x00cd_2c50, &args![this, extra]);
    e.mem.set_u32(base + 0x8c, 0);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u32(base + 0x94, 0x8000_0000);
    e.mem.set_u32(base + 0x98, 0);
    e.mem.set_u32(base + 0x9c, 0);
    e.mem.set_u32(base + 0xa0, 0x8000_0000);
    e.mem.set_u32(base, 0x010c_e15c);
    e.mem.set_u32(base + 0x160, 0);
    e.mem.set_u32(base + 0x164, 0);
    e.mem.set_u32(base + 0x168, 0x8000_0000);
    this
}

// Translated from 00dfcf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`). The vtable word at `+0` takes three values in turn (`010305a8`,
/// `010cc364`, `010e2144`) and the word at `+0x10` three (`010c6fb0`, `010cc344`, `010e2124`);
/// when `mode` is not zero the stages also write `+0xc` (0, 7, `0x17`) and the byte at `+0x15`
/// (3, 4). Returns `this` (`EAX`). The class is unconfirmed.
pub fn fn_00dfcf60(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_05a8);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base + 0x10, 0x010c_6fb0);
    e.mem.set_u32(base, 0x010c_c364);
    e.mem.set_u32(base + 0x10, 0x010c_c344);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 7);
        e.mem.set_u8(base + 0x15, 3);
    }
    e.mem.set_u32(base, 0x010e_2144);
    e.mem.set_u32(base + 0x10, 0x010e_2124);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 0x17);
        e.mem.set_u8(base + 0x15, 4);
    }
    this
}

// Translated from 00dfcfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to the deleting destructor
/// `00dfd080` (`RET 4`); the caller gets its `EAX`, `this - 0x10`. The class is unconfirmed.
pub fn fn_00dfcfc0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    deleting_destructor_00dfd080(e, Ptr::new(this.addr().wrapping_sub(0x10)), flags)
}

// Translated from 00dfcfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a destructor (`this` in `ECX`) that releases three `hkArray`-shaped
/// triples in turn, the one at `+0x30` (byte elements), then those at `+0x24` and `+0x18`
/// (16-byte elements), and sets the vtable word at `+0x10` to `010c6fb0` and the one at `+0` to
/// `0102e388`. The class is unconfirmed; `00dfd080` is its deleting form.
pub fn fn_00dfcfd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    release_array(e, base + 0x30, 1);
    release_array(e, base + 0x24, 16);
    release_array(e, base + 0x18, 16);
    e.mem.set_u32(base + 0x10, 0x010c_6fb0);
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00dfc570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011babc8.
pub fn fn_00dfc570(_e: &mut Engine) {}

// Translated from 00dfc580 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011babcc.
pub fn fn_00dfc580(_e: &mut Engine) {}

// Translated from 00dfc5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bac08.
pub fn fn_00dfc5a0(_e: &mut Engine) {}

// Translated from 00dfc5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bac4c.
pub fn fn_00dfc5c0(_e: &mut Engine) {}

// Translated from 00dfc5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bac64.
pub fn fn_00dfc5e0(_e: &mut Engine) {}

// Translated from 00dfc600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc21e9.
pub fn fn_00dfc600(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfc620 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `0102e368` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc21df.
pub fn fn_00dfc620(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x0102_e368);
    }
}

// Translated from 00dfc640 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bad14.
pub fn fn_00dfc640(_e: &mut Engine) {}

// Translated from 00dfc650 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bad18.
pub fn fn_00dfc650(_e: &mut Engine) {}

// Translated from 00dfc670 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bad68.
pub fn fn_00dfc670(_e: &mut Engine) {}

// Translated from 00dfc690 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc22d9.
pub fn fn_00dfc690(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfc6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bad64.
pub fn fn_00dfc6b0(_e: &mut Engine) {}

// Translated from 00dfc6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010d0a64` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc22cf.
pub fn fn_00dfc6c0(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010d_0a64);
    }
}

// Translated from 00dfc6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011badac.
pub fn fn_00dfc6e0(_e: &mut Engine) {}

// Translated from 00dfc700 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2379.
pub fn fn_00dfc700(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfc720 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bada8.
pub fn fn_00dfc720(_e: &mut Engine) {}

// Translated from 00dfc730 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010d0aac` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc236f.
pub fn fn_00dfc730(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010d_0aac);
    }
}

// Translated from 00dfc750 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011badf8.
pub fn fn_00dfc750(_e: &mut Engine) {}

// Translated from 00dfc770 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00e3b060` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc240f.
pub fn fn_00dfc770(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.call(0x00e3_b060, &args![object, extra]);
    }
}

// Translated from 00dfc790 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2419.
pub fn fn_00dfc790(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfc7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bae70.
pub fn fn_00dfc7d0(_e: &mut Engine) {}

// Translated from 00dfc7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc24c9.
pub fn fn_00dfc7f0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfc810 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00dfc850` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc24bf.
pub fn fn_00dfc810(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        fn_00dfc850(e, object, extra);
    }
}

// Translated from 00dfc8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011baee8.
pub fn fn_00dfc8b0(_e: &mut Engine) {}

// Translated from 00dfc8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2579.
pub fn fn_00dfc8d0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfc8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011baee4.
pub fn fn_00dfc8f0(_e: &mut Engine) {}

// Translated from 00dfc900 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e11ec` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc256f.
pub fn fn_00dfc900(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_11ec);
    }
}

// Translated from 00dfc920 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011baf34.
pub fn fn_00dfc920(_e: &mut Engine) {}

// Translated from 00dfc940 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2639.
pub fn fn_00dfc940(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfc960 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011baf30.
pub fn fn_00dfc960(_e: &mut Engine) {}

// Translated from 00dfc970 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped). When
/// `object` is not null it writes the vtable word `010d0af4` at its start and, when `mode` is not
/// zero and the byte at `+0x9a` is not zero, calls `00d4f270` on it (`this` in `ECX`) with the
/// word the code builds by pushing `object` and overwriting the low byte with 1; returns nothing.
/// Ghidra made no function here; the address is an immediate operand at 00fc262f.
pub fn fn_00dfc970(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x010d_0af4);
        if mode != 0 && e.mem.u8(base + 0x9a) != 0 {
            e.call(0x00d4_f270, &args![object, (base & 0xffff_ff00) | 1]);
        }
    }
}

// Translated from 00dfc9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011baf80.
pub fn fn_00dfc9b0(_e: &mut Engine) {}

// Translated from 00dfc9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc26f9.
pub fn fn_00dfc9d0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfc9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011baf7c.
pub fn fn_00dfc9f0(_e: &mut Engine) {}

// Translated from 00dfca00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped). When
/// `object` is not null it writes the vtable word `010cb71c` at its start and, when `mode` is not
/// zero and the byte at `+0x116` is not zero, calls `00cdea00` on it (`this` in `ECX`) with the
/// word the code builds by pushing `object` and overwriting the low byte with 1; returns nothing.
/// Ghidra made no function here; the address is an immediate operand at 00fc26ef.
pub fn fn_00dfca00(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x010c_b71c);
        if mode != 0 && e.mem.u8(base + 0x116) != 0 {
            e.call(0x00cd_ea00, &args![object, (base & 0xffff_ff00) | 1]);
        }
    }
}

// Translated from 00dfca40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bafc8.
pub fn fn_00dfca40(_e: &mut Engine) {}

// Translated from 00dfca60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc27a9.
pub fn fn_00dfca60(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfca80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bafc4.
pub fn fn_00dfca80(_e: &mut Engine) {}

// Translated from 00dfca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010da04c` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc279f.
pub fn fn_00dfca90(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010d_a04c);
    }
}

// Translated from 00dfcab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb010.
pub fn fn_00dfcab0(_e: &mut Engine) {}

// Translated from 00dfcad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2869.
pub fn fn_00dfcad0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfcaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb00c.
pub fn fn_00dfcaf0(_e: &mut Engine) {}

// Translated from 00dfcb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010d0b3c` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc285f.
pub fn fn_00dfcb00(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010d_0b3c);
    }
}

// Translated from 00dfcb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb058.
pub fn fn_00dfcb20(_e: &mut Engine) {}

// Translated from 00dfcb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00e3b960` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc290f.
pub fn fn_00dfcb40(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.call(0x00e3_b960, &args![object, extra]);
    }
}

// Translated from 00dfcb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2919.
pub fn fn_00dfcb60(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfcba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb0c8.
pub fn fn_00dfcba0(_e: &mut Engine) {}

// Translated from 00dfcbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc29c9.
pub fn fn_00dfcbc0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfcbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb0c4.
pub fn fn_00dfcbe0(_e: &mut Engine) {}

// Translated from 00dfcbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010d37e4` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc29bf.
pub fn fn_00dfcbf0(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010d_37e4);
    }
}

// Translated from 00dfcc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb118.
pub fn fn_00dfcc10(_e: &mut Engine) {}

// Translated from 00dfcc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2a89.
pub fn fn_00dfcc30(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfcc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb114.
pub fn fn_00dfcc50(_e: &mut Engine) {}

// Translated from 00dfcc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010c89e4` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc2a7f.
pub fn fn_00dfcc60(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_89e4);
    }
}

// Translated from 00dfcc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb168.
pub fn fn_00dfcc80(_e: &mut Engine) {}

// Translated from 00dfcca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2b49.
pub fn fn_00dfcca0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfccc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb164.
pub fn fn_00dfccc0(_e: &mut Engine) {}

// Translated from 00dfccd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010d0be4` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc2b3f.
pub fn fn_00dfccd0(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010d_0be4);
    }
}

// Translated from 00dfccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb1ac.
pub fn fn_00dfccf0(_e: &mut Engine) {}

// Translated from 00dfcd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2c09.
pub fn fn_00dfcd10(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfcd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb1a8.
pub fn fn_00dfcd30(_e: &mut Engine) {}

// Translated from 00dfcd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010c91ec` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc2bff.
pub fn fn_00dfcd40(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_91ec);
    }
}

// Translated from 00dfcd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb1f0.
pub fn fn_00dfcd60(_e: &mut Engine) {}

// Translated from 00dfcd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00dd0420` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is stored in the data word at 011bb204.
pub fn fn_00dfcd80(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.call(0x00dd_0420, &args![object, extra]);
    }
}

// Translated from 00dfcdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Loads it into `ECX` and
/// jumps to `00d51620` (a plain `RET`, no stack words), which empties the four `hkArray`-shaped
/// triples at `+0x10`, `+0x1c`, `+0x28` and `+0x34`; returns nothing. Ghidra made no function here; the address is stored in the data word at 011bb208.
pub fn fn_00dfcdb0(e: &mut Engine, object: Ptr) {
    e.call(0x00d5_1620, &args![object]);
}

// Translated from 00dfcdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb28c.
pub fn fn_00dfcdc0(_e: &mut Engine) {}

// Translated from 00dfcde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00d53df0` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2cff.
pub fn fn_00dfcde0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.call(0x00d5_3df0, &args![object, extra]);
    }
}

// Translated from 00dfce00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2d09.
pub fn fn_00dfce00(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfce40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb2d8.
pub fn fn_00dfce40(_e: &mut Engine) {}

// Translated from 00dfce60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2da9.
pub fn fn_00dfce60(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb2d4.
pub fn fn_00dfce80(_e: &mut Engine) {}

// Translated from 00dfce90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010d0cd4` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc2d9f.
pub fn fn_00dfce90(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010d_0cd4);
    }
}

// Translated from 00dfceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb324.
pub fn fn_00dfceb0(_e: &mut Engine) {}

// Translated from 00dfced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb320.
pub fn fn_00dfced0(_e: &mut Engine) {}

// Translated from 00dfcee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb388.
pub fn fn_00dfcee0(_e: &mut Engine) {}

// Translated from 00dfcf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2e99.
pub fn fn_00dfcf00(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfcf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00dfcf60` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2e8f.
pub fn fn_00dfcf20(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        fn_00dfcf60(e, object, extra);
    }
}

// Translated from 00dfd0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a lone `RET`, so it reads nothing and does nothing. Ghidra made no function here; the address is stored in the data word at 011bb464.
pub fn fn_00dfd0b0(_e: &mut Engine) {}

// Translated from 00dfd0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2f39.
pub fn fn_00dfd0d0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00dfd0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped); a
/// constructor chain the compiler collapsed. When `object` is not null it writes the vtable word
/// `010305a8` at its start, the word 0 at `+0xc` when `mode` is not zero, then the vtable word
/// `010e222c`, and `0x18` at `+0xc` whatever `mode` is; returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc2f2f.
pub fn fn_00dfd0f0(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x0103_05a8);
        if mode != 0 {
            e.mem.set_u32(base + 0xc, 0);
        }
        e.mem.set_u32(base, 0x010e_222c);
        e.mem.set_u32(base + 0xc, 0x18);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_c7a0, fn_00dfc7a0() -> u32),
        entry!(0x00df_c830, fn_00dfc830() -> u32),
        entry!(0x00df_cb70, fn_00dfcb70() -> u32),
        entry!(0x00df_ce10, fn_00dfce10() -> u32),
        entry!(0x00df_cf40, fn_00dfcf40() -> u32),
        entry!(0x00dfc630, return_constant_00dfc630() -> u32),
        entry!(0x00dfc6d0, return_constant_00dfc6d0() -> u32),
        entry!(0x00dfc740, return_constant_00dfc740() -> u32),
        entry!(0x00dfc910, return_constant_00dfc910() -> u32),
        entry!(0x00dfc9a0, return_constant_00dfc9a0() -> u32),
        entry!(0x00dfca30, return_constant_00dfca30() -> u32),
        entry!(0x00df_d080, deleting_destructor_00dfd080(Ptr, u8) -> Ptr),
        entry!(0x00dfcaa0, return_constant_00dfcaa0() -> u32),
        entry!(0x00dfcb10, return_constant_00dfcb10() -> u32),
        entry!(0x00dfcc00, return_constant_00dfcc00() -> u32),
        entry!(0x00dfcc70, return_constant_00dfcc70() -> u32),
        entry!(0x00dfcce0, return_constant_00dfcce0() -> u32),
        entry!(0x00dfcd50, return_constant_00dfcd50() -> u32),
        entry!(0x00dfcea0, return_constant_00dfcea0() -> u32),
        entry!(0x00dfd120, return_constant_00dfd120() -> u32),
        entry!(0x00df_c850, fn_00dfc850(Ptr, u32) -> Ptr),
        entry!(0x00df_cf60, fn_00dfcf60(Ptr, u32) -> Ptr),
        entry!(0x00df_cfc0, fn_00dfcfc0(Ptr, u8) -> Ptr),
        entry!(0x00df_cfd0, fn_00dfcfd0(Ptr)),
        entry!(0x00df_c570, fn_00dfc570()),
        entry!(0x00df_c580, fn_00dfc580()),
        entry!(0x00df_c5a0, fn_00dfc5a0()),
        entry!(0x00df_c5c0, fn_00dfc5c0()),
        entry!(0x00df_c5e0, fn_00dfc5e0()),
        entry!(0x00df_c600, fn_00dfc600(Ptr)),
        entry!(0x00df_c620, fn_00dfc620(Ptr, u32)),
        entry!(0x00df_c640, fn_00dfc640()),
        entry!(0x00df_c650, fn_00dfc650()),
        entry!(0x00df_c670, fn_00dfc670()),
        entry!(0x00df_c690, fn_00dfc690(Ptr)),
        entry!(0x00df_c6b0, fn_00dfc6b0()),
        entry!(0x00df_c6c0, fn_00dfc6c0(Ptr, u32)),
        entry!(0x00df_c6e0, fn_00dfc6e0()),
        entry!(0x00df_c700, fn_00dfc700(Ptr)),
        entry!(0x00df_c720, fn_00dfc720()),
        entry!(0x00df_c730, fn_00dfc730(Ptr, u32)),
        entry!(0x00df_c750, fn_00dfc750()),
        entry!(0x00df_c770, fn_00dfc770(Ptr, u32)),
        entry!(0x00df_c790, fn_00dfc790(Ptr)),
        entry!(0x00df_c7d0, fn_00dfc7d0()),
        entry!(0x00df_c7f0, fn_00dfc7f0(Ptr)),
        entry!(0x00df_c810, fn_00dfc810(Ptr, u32)),
        entry!(0x00df_c8b0, fn_00dfc8b0()),
        entry!(0x00df_c8d0, fn_00dfc8d0(Ptr)),
        entry!(0x00df_c8f0, fn_00dfc8f0()),
        entry!(0x00df_c900, fn_00dfc900(Ptr, u32)),
        entry!(0x00df_c920, fn_00dfc920()),
        entry!(0x00df_c940, fn_00dfc940(Ptr)),
        entry!(0x00df_c960, fn_00dfc960()),
        entry!(0x00df_c970, fn_00dfc970(Ptr, u32)),
        entry!(0x00df_c9b0, fn_00dfc9b0()),
        entry!(0x00df_c9d0, fn_00dfc9d0(Ptr)),
        entry!(0x00df_c9f0, fn_00dfc9f0()),
        entry!(0x00df_ca00, fn_00dfca00(Ptr, u32)),
        entry!(0x00df_ca40, fn_00dfca40()),
        entry!(0x00df_ca60, fn_00dfca60(Ptr)),
        entry!(0x00df_ca80, fn_00dfca80()),
        entry!(0x00df_ca90, fn_00dfca90(Ptr, u32)),
        entry!(0x00df_cab0, fn_00dfcab0()),
        entry!(0x00df_cad0, fn_00dfcad0(Ptr)),
        entry!(0x00df_caf0, fn_00dfcaf0()),
        entry!(0x00df_cb00, fn_00dfcb00(Ptr, u32)),
        entry!(0x00df_cb20, fn_00dfcb20()),
        entry!(0x00df_cb40, fn_00dfcb40(Ptr, u32)),
        entry!(0x00df_cb60, fn_00dfcb60(Ptr)),
        entry!(0x00df_cba0, fn_00dfcba0()),
        entry!(0x00df_cbc0, fn_00dfcbc0(Ptr)),
        entry!(0x00df_cbe0, fn_00dfcbe0()),
        entry!(0x00df_cbf0, fn_00dfcbf0(Ptr, u32)),
        entry!(0x00df_cc10, fn_00dfcc10()),
        entry!(0x00df_cc30, fn_00dfcc30(Ptr)),
        entry!(0x00df_cc50, fn_00dfcc50()),
        entry!(0x00df_cc60, fn_00dfcc60(Ptr, u32)),
        entry!(0x00df_cc80, fn_00dfcc80()),
        entry!(0x00df_cca0, fn_00dfcca0(Ptr)),
        entry!(0x00df_ccc0, fn_00dfccc0()),
        entry!(0x00df_ccd0, fn_00dfccd0(Ptr, u32)),
        entry!(0x00df_ccf0, fn_00dfccf0()),
        entry!(0x00df_cd10, fn_00dfcd10(Ptr)),
        entry!(0x00df_cd30, fn_00dfcd30()),
        entry!(0x00df_cd40, fn_00dfcd40(Ptr, u32)),
        entry!(0x00df_cd60, fn_00dfcd60()),
        entry!(0x00df_cd80, fn_00dfcd80(Ptr, u32)),
        entry!(0x00df_cdb0, fn_00dfcdb0(Ptr)),
        entry!(0x00df_cdc0, fn_00dfcdc0()),
        entry!(0x00df_cde0, fn_00dfcde0(Ptr, u32)),
        entry!(0x00df_ce00, fn_00dfce00(Ptr)),
        entry!(0x00df_ce40, fn_00dfce40()),
        entry!(0x00df_ce60, fn_00dfce60(Ptr)),
        entry!(0x00df_ce80, fn_00dfce80()),
        entry!(0x00df_ce90, fn_00dfce90(Ptr, u32)),
        entry!(0x00df_ceb0, fn_00dfceb0()),
        entry!(0x00df_ced0, fn_00dfced0()),
        entry!(0x00df_cee0, fn_00dfcee0()),
        entry!(0x00df_cf00, fn_00dfcf00(Ptr)),
        entry!(0x00df_cf20, fn_00dfcf20(Ptr, u32)),
        entry!(0x00df_d0b0, fn_00dfd0b0()),
        entry!(0x00df_d0d0, fn_00dfd0d0(Ptr)),
        entry!(0x00df_d0f0, fn_00dfd0f0(Ptr, u32)),
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
    fn deleting_destructor_00dfd080_frees_only_when_bit_zero_is_set() {
        check(0x00df_d080, 0x00df_cfd0);
    }

    #[test]
    fn return_constant_00dfc630_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c630, &args![]).u32(), 0x0102_e368);
    }

    #[test]
    fn return_constant_00dfc6d0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c6d0, &args![]).u32(), 0x010d_0a64);
    }

    #[test]
    fn return_constant_00dfc740_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c740, &args![]).u32(), 0x010d_0aac);
    }

    #[test]
    fn return_constant_00dfc910_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c910, &args![]).u32(), 0x010e_11ec);
    }

    #[test]
    fn return_constant_00dfc9a0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c9a0, &args![]).u32(), 0x010d_0af4);
    }

    #[test]
    fn return_constant_00dfca30_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ca30, &args![]).u32(), 0x010c_b71c);
    }
}

#[cfg(test)]
mod a07_first_word_tests {
    use crate::prelude::*;

    const BUILT_WORD: u32 = 0x0abc_def0;

    /// A double over `callee` writes `BUILT_WORD` at the buffer it is given; `function`
    /// must call it once with that buffer and a zero flag word, and return the word.
    fn check(function: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |e, a| {
            e.mem.set_u32(a[0], BUILT_WORD);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(function, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0].1[0], 0);
        assert_eq!(calls[0].1[1], 0);
        assert_eq!(result, BUILT_WORD);
    }

    macro_rules! first_word_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check($function, $callee);
            }
        };
    }

    first_word_test!(
        fn_00dfc7a0_returns_first_word_of_built_object,
        0x00df_c7a0,
        0x00e3_b060
    );
    first_word_test!(
        fn_00dfc830_returns_first_word_of_built_object,
        0x00df_c830,
        0x00df_c850
    );
    first_word_test!(
        fn_00dfcb70_returns_first_word_of_built_object,
        0x00df_cb70,
        0x00e3_b960
    );
    first_word_test!(
        fn_00dfce10_returns_first_word_of_built_object,
        0x00df_ce10,
        0x00d5_3df0
    );
    first_word_test!(
        fn_00dfcf40_returns_first_word_of_built_object,
        0x00df_cf40,
        0x00df_cf60
    );
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

    #[test]
    fn fn_00dfc7a0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e3_b060, 0x0a0b_0c0d);
        let result = e.call(0x00df_c7a0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dfc830_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00df_c850, 0x0a0b_0c0d);
        let result = e.call(0x00df_c830, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dfcaa0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_caa0, &args![]).u32(), 0x010d_a04c);
    }

    #[test]
    fn return_constant_00dfcb10_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cb10, &args![]).u32(), 0x010d_0b3c);
    }

    #[test]
    fn fn_00dfcb70_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e3_b960, 0x0a0b_0c0d);
        let result = e.call(0x00df_cb70, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dfcc00_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cc00, &args![]).u32(), 0x010d_37e4);
    }

    #[test]
    fn return_constant_00dfcc70_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cc70, &args![]).u32(), 0x010c_89e4);
    }

    #[test]
    fn return_constant_00dfcce0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cce0, &args![]).u32(), 0x010d_0be4);
    }

    #[test]
    fn return_constant_00dfcd50_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cd50, &args![]).u32(), 0x010c_91ec);
    }

    #[test]
    fn fn_00dfce10_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d5_3df0, 0x0a0b_0c0d);
        let result = e.call(0x00df_ce10, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dfcea0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_cea0, &args![]).u32(), 0x010d_0cd4);
    }

    #[test]
    fn fn_00dfcf40_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00df_cf60, 0x0a0b_0c0d);
        let result = e.call(0x00df_cf40, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dfd120_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_d120, &args![]).u32(), 0x010e_222c);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    /// `00dfc850` calls `00cd2c50(this, extra)` first, then writes its fields.
    #[test]
    fn fn_00dfc850_constructs_over_the_base() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x170);
        e.mem.write(this, &[0x55; 0x170]);
        e.register_double(0x00cd_2c50, move |e, a| {
            assert_eq!(a, [this, 9]);
            assert_eq!(e.mem.u32(this + 0x8c), 0x5555_5555);
            Ret::default()
        });
        assert_eq!(e.call(0x00df_c850, &args![this, 9u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_e15c);
        for offset in [0x8c, 0x90, 0x98, 0x9c, 0x160, 0x164] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        for offset in [0x94, 0xa0, 0x168] {
            assert_eq!(e.mem.u32(this + offset), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0x88), 0x5555_5555);
        assert_eq!(e.mem.u32(this + 0xa4), 0x5555_5555);
    }

    /// Mode 0 leaves +0xc and the byte at +0x15 alone; another mode ends with 0x17 and 4.
    #[test]
    fn fn_00dfcf60_stages_depend_on_the_mode() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0x66; 0x20]);
        assert_eq!(e.call(0x00df_cf60, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010e_2144);
        assert_eq!(e.mem.u32(this + 0x10), 0x010e_2124);
        assert_eq!(e.mem.u32(this + 0xc), 0x6666_6666);
        assert_eq!(e.mem.u8(this + 0x15), 0x66);
        e.call(0x00df_cf60, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x17);
        assert_eq!(e.mem.u8(this + 0x15), 4);
        assert_eq!(e.mem.u32(this), 0x010e_2144);
    }

    /// The thunk runs the deleting destructor on `this - 0x10`, whose base destructor
    /// `00dfcfd0` is a double here.
    #[test]
    fn fn_00dfcfc0_runs_the_destructor_on_this_minus_0x10() {
        let mut e = Engine::new();
        e.register(0x00df_cfd0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20) + 0x10;
        let got = e.call(0x00df_cfc0, &args![this, 0u32]).u32();
        assert_eq!(got, this - 0x10);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00df_cfd0).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this - 0x10]);
    }

    /// Three arrays: the first owned (capacity 5, bytes), the second not owned (negative
    /// capacity word: nothing freed), the third owned (capacity 3 of 16-byte elements).
    #[test]
    fn fn_00dfcfd0_releases_the_owned_arrays() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        // +0x30: data 0x1111, size 4, capacity 5 | flag bits 0x40000000.
        e.mem.set_u32(this + 0x30, 0x1111);
        e.mem.set_u32(this + 0x34, 4);
        e.mem.set_u32(this + 0x38, 0x4000_0005);
        // +0x24: not owned.
        e.mem.set_u32(this + 0x24, 0x2222);
        e.mem.set_u32(this + 0x28, 2);
        e.mem.set_u32(this + 0x2c, 0x8000_0002);
        // +0x18: data 0x3333, size 1, capacity 3.
        e.mem.set_u32(this + 0x18, 0x3333);
        e.mem.set_u32(this + 0x1c, 1);
        e.mem.set_u32(this + 0x20, 3);
        e.call(0x00df_cfd0, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![vec![allocator, 0x1111, 5], vec![allocator, 0x3333, 0x30]]
        );
        for (data, size, capacity) in [(0x30, 0x34, 0x38), (0x24, 0x28, 0x2c), (0x18, 0x1c, 0x20)] {
            assert_eq!(e.mem.u32(this + data), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + capacity), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_6fb0);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
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

    /// A lone `RET`: whatever words it is given, nothing in memory changes and nothing is returned.
    fn check_noop(address: u32) {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x20);
        e.mem.write(block, &[0x5a; 0x20]);
        e.call_log = Some(vec![]);
        let result = e.call(address, &args![block, 1u32]);
        assert_eq!(result, Ret::default());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.bytes(block, 0x20), vec![0x5a; 0x20]);
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

    /// A null object writes nothing (a write at address 0 would stop the test); otherwise only the
    /// first word of the object changes.
    fn check_vtable_store(address: u32, vtable: u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.mem.write(object, &[0x77; 0x20]);
        e.call(address, &args![0u32, 5u32]);
        e.call(address, &args![object, 5u32]);
        assert_eq!(e.mem.u32(object), vtable);
        assert_eq!(e.mem.bytes(object + 4, 0x1c), vec![0x77; 0x1c]);
    }

    /// The callee runs only for a non-null object, with `(object, extra)`.
    fn check_forward(address: u32, callee: u32) {
        let mut e = Engine::new();
        let log = logging_double(&mut e, callee);
        e.call(address, &args![0u32, 7u32]);
        assert!(log.borrow().is_empty());
        e.call(address, &args![0x2000u32, 7u32]);
        assert_eq!(*log.borrow(), vec![vec![0x2000, 7]]);
    }

    /// The vtable word is written for any non-null object; the setter runs only when `mode` and the
    /// flag byte at `offset` are both not zero, with `(object, object with its low byte set to 1)`.
    fn check_setter_flag(address: u32, vtable: u32, offset: u32, setter: u32) {
        let mut e = Engine::new();
        let log = logging_double(&mut e, setter);
        let object = e.mem.alloc(0x200);
        e.mem.write(object, &[0; 0x200]);
        e.call(address, &args![0u32, 1u32]);
        e.call(address, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), vtable);
        e.mem.set_u8(object + offset, 0x10);
        e.call(address, &args![object, 0u32]);
        assert!(log.borrow().is_empty());
        e.mem.set_u8(object + offset, 0);
        e.call(address, &args![object, 1u32]);
        assert!(log.borrow().is_empty());
        e.mem.set_u8(object + offset, 0x10);
        e.call(address, &args![object, 1u32]);
        assert_eq!(
            *log.borrow(),
            vec![vec![object, (object & 0xffff_ff00) | 1]]
        );
    }

    /// The callee runs once with the object as `this`.
    fn check_tail(address: u32, callee: u32) {
        let mut e = Engine::new();
        let log = logging_double(&mut e, callee);
        e.call(address, &args![0x3000u32]);
        assert_eq!(*log.borrow(), vec![vec![0x3000]]);
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

    /// A constructor over the base constructor `00cd2c50` (a double): it runs only for a non-null
    /// object, with `(object, extra)`, and the object then ends with `vtable` and the three empty
    /// arrays (`0x80000000` in their last word) at `+0x8c`, `+0x98` and `tail`.
    fn check_over_base(address: u32, vtable: u32, tail: u32) {
        let mut e = Engine::new();
        let log = logging_double(&mut e, 0x00cd_2c50);
        let object = e.mem.alloc(0x170);
        e.mem.write(object, &[0x55; 0x170]);
        e.call(address, &args![0u32, 9u32]);
        assert!(log.borrow().is_empty());
        e.call(address, &args![object, 9u32]);
        assert_eq!(*log.borrow(), vec![vec![object, 9]]);
        assert_eq!(e.mem.u32(object), vtable);
        for offset in [0x8c, 0x90, 0x98, 0x9c, tail, tail + 4] {
            assert_eq!(e.mem.u32(object + offset), 0);
        }
        for offset in [0x94, 0xa0, tail + 8] {
            assert_eq!(e.mem.u32(object + offset), 0x8000_0000);
        }
    }

    #[test]
    fn fn_00dfc570_does_nothing() {
        check_noop(0x00df_c570);
    }

    #[test]
    fn fn_00dfc580_does_nothing() {
        check_noop(0x00df_c580);
    }

    #[test]
    fn fn_00dfc5a0_does_nothing() {
        check_noop(0x00df_c5a0);
    }

    #[test]
    fn fn_00dfc5c0_does_nothing() {
        check_noop(0x00df_c5c0);
    }

    #[test]
    fn fn_00dfc5e0_does_nothing() {
        check_noop(0x00df_c5e0);
    }

    #[test]
    fn fn_00dfc600_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_c600);
    }

    #[test]
    fn fn_00dfc620_stores_its_vtable_word() {
        check_vtable_store(0x00df_c620, 0x0102_e368);
    }

    #[test]
    fn fn_00dfc640_does_nothing() {
        check_noop(0x00df_c640);
    }

    #[test]
    fn fn_00dfc650_does_nothing() {
        check_noop(0x00df_c650);
    }

    #[test]
    fn fn_00dfc670_does_nothing() {
        check_noop(0x00df_c670);
    }

    #[test]
    fn fn_00dfc690_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_c690);
    }

    #[test]
    fn fn_00dfc6b0_does_nothing() {
        check_noop(0x00df_c6b0);
    }

    #[test]
    fn fn_00dfc6c0_stores_its_vtable_word() {
        check_vtable_store(0x00df_c6c0, 0x010d_0a64);
    }

    #[test]
    fn fn_00dfc6e0_does_nothing() {
        check_noop(0x00df_c6e0);
    }

    #[test]
    fn fn_00dfc700_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_c700);
    }

    #[test]
    fn fn_00dfc720_does_nothing() {
        check_noop(0x00df_c720);
    }

    #[test]
    fn fn_00dfc730_stores_its_vtable_word() {
        check_vtable_store(0x00df_c730, 0x010d_0aac);
    }

    #[test]
    fn fn_00dfc750_does_nothing() {
        check_noop(0x00df_c750);
    }

    #[test]
    fn fn_00dfc770_runs_the_constructor_on_a_non_null_object() {
        check_forward(0x00df_c770, 0x00e3_b060);
    }

    #[test]
    fn fn_00dfc790_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_c790);
    }

    #[test]
    fn fn_00dfc7d0_does_nothing() {
        check_noop(0x00df_c7d0);
    }

    #[test]
    fn fn_00dfc7f0_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_c7f0);
    }

    #[test]
    fn fn_00dfc8b0_does_nothing() {
        check_noop(0x00df_c8b0);
    }

    #[test]
    fn fn_00dfc8d0_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_c8d0);
    }

    #[test]
    fn fn_00dfc8f0_does_nothing() {
        check_noop(0x00df_c8f0);
    }

    #[test]
    fn fn_00dfc900_stores_its_vtable_word() {
        check_vtable_store(0x00df_c900, 0x010e_11ec);
    }

    #[test]
    fn fn_00dfc920_does_nothing() {
        check_noop(0x00df_c920);
    }

    #[test]
    fn fn_00dfc940_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_c940);
    }

    #[test]
    fn fn_00dfc960_does_nothing() {
        check_noop(0x00df_c960);
    }

    #[test]
    fn fn_00dfc970_sets_a_flag_through_its_setter_only_when_both_tests_pass() {
        check_setter_flag(0x00df_c970, 0x010d_0af4, 0x9a, 0x00d4_f270);
    }

    #[test]
    fn fn_00dfc9b0_does_nothing() {
        check_noop(0x00df_c9b0);
    }

    #[test]
    fn fn_00dfc9d0_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_c9d0);
    }

    #[test]
    fn fn_00dfc9f0_does_nothing() {
        check_noop(0x00df_c9f0);
    }

    #[test]
    fn fn_00dfca00_sets_a_flag_through_its_setter_only_when_both_tests_pass() {
        check_setter_flag(0x00df_ca00, 0x010c_b71c, 0x116, 0x00cd_ea00);
    }

    #[test]
    fn fn_00dfca40_does_nothing() {
        check_noop(0x00df_ca40);
    }

    #[test]
    fn fn_00dfca60_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_ca60);
    }

    #[test]
    fn fn_00dfca80_does_nothing() {
        check_noop(0x00df_ca80);
    }

    #[test]
    fn fn_00dfca90_stores_its_vtable_word() {
        check_vtable_store(0x00df_ca90, 0x010d_a04c);
    }

    #[test]
    fn fn_00dfcab0_does_nothing() {
        check_noop(0x00df_cab0);
    }

    #[test]
    fn fn_00dfcad0_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_cad0);
    }

    #[test]
    fn fn_00dfcaf0_does_nothing() {
        check_noop(0x00df_caf0);
    }

    #[test]
    fn fn_00dfcb00_stores_its_vtable_word() {
        check_vtable_store(0x00df_cb00, 0x010d_0b3c);
    }

    #[test]
    fn fn_00dfcb20_does_nothing() {
        check_noop(0x00df_cb20);
    }

    #[test]
    fn fn_00dfcb40_runs_the_constructor_on_a_non_null_object() {
        check_forward(0x00df_cb40, 0x00e3_b960);
    }

    #[test]
    fn fn_00dfcb60_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_cb60);
    }

    #[test]
    fn fn_00dfcba0_does_nothing() {
        check_noop(0x00df_cba0);
    }

    #[test]
    fn fn_00dfcbc0_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_cbc0);
    }

    #[test]
    fn fn_00dfcbe0_does_nothing() {
        check_noop(0x00df_cbe0);
    }

    #[test]
    fn fn_00dfcbf0_stores_its_vtable_word() {
        check_vtable_store(0x00df_cbf0, 0x010d_37e4);
    }

    #[test]
    fn fn_00dfcc10_does_nothing() {
        check_noop(0x00df_cc10);
    }

    #[test]
    fn fn_00dfcc30_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_cc30);
    }

    #[test]
    fn fn_00dfcc50_does_nothing() {
        check_noop(0x00df_cc50);
    }

    #[test]
    fn fn_00dfcc60_stores_its_vtable_word() {
        check_vtable_store(0x00df_cc60, 0x010c_89e4);
    }

    #[test]
    fn fn_00dfcc80_does_nothing() {
        check_noop(0x00df_cc80);
    }

    #[test]
    fn fn_00dfcca0_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_cca0);
    }

    #[test]
    fn fn_00dfccc0_does_nothing() {
        check_noop(0x00df_ccc0);
    }

    #[test]
    fn fn_00dfccd0_stores_its_vtable_word() {
        check_vtable_store(0x00df_ccd0, 0x010d_0be4);
    }

    #[test]
    fn fn_00dfccf0_does_nothing() {
        check_noop(0x00df_ccf0);
    }

    #[test]
    fn fn_00dfcd10_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_cd10);
    }

    #[test]
    fn fn_00dfcd30_does_nothing() {
        check_noop(0x00df_cd30);
    }

    #[test]
    fn fn_00dfcd40_stores_its_vtable_word() {
        check_vtable_store(0x00df_cd40, 0x010c_91ec);
    }

    #[test]
    fn fn_00dfcd60_does_nothing() {
        check_noop(0x00df_cd60);
    }

    #[test]
    fn fn_00dfcd80_runs_the_constructor_on_a_non_null_object() {
        check_forward(0x00df_cd80, 0x00dd_0420);
    }

    #[test]
    fn fn_00dfcdb0_runs_00d51620_on_the_object() {
        check_tail(0x00df_cdb0, 0x00d5_1620);
    }

    #[test]
    fn fn_00dfcdc0_does_nothing() {
        check_noop(0x00df_cdc0);
    }

    #[test]
    fn fn_00dfcde0_runs_the_constructor_on_a_non_null_object() {
        check_forward(0x00df_cde0, 0x00d5_3df0);
    }

    #[test]
    fn fn_00dfce00_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_ce00);
    }

    #[test]
    fn fn_00dfce40_does_nothing() {
        check_noop(0x00df_ce40);
    }

    #[test]
    fn fn_00dfce60_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_ce60);
    }

    #[test]
    fn fn_00dfce80_does_nothing() {
        check_noop(0x00df_ce80);
    }

    #[test]
    fn fn_00dfce90_stores_its_vtable_word() {
        check_vtable_store(0x00df_ce90, 0x010d_0cd4);
    }

    #[test]
    fn fn_00dfceb0_does_nothing() {
        check_noop(0x00df_ceb0);
    }

    #[test]
    fn fn_00dfced0_does_nothing() {
        check_noop(0x00df_ced0);
    }

    #[test]
    fn fn_00dfcee0_does_nothing() {
        check_noop(0x00df_cee0);
    }

    #[test]
    fn fn_00dfcf00_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_cf00);
    }

    #[test]
    fn fn_00dfd0b0_does_nothing() {
        check_noop(0x00df_d0b0);
    }

    #[test]
    fn fn_00dfd0d0_calls_slot_zero_with_zero() {
        check_cleanup(0x00df_d0d0);
    }

    #[test]
    fn fn_00dfd0f0_writes_the_final_vtable_and_the_value() {
        check_staged(0x00df_d0f0, 0x010e_222c, &[], 0x18);
    }

    #[test]
    fn fn_00dfc810_constructs_over_the_base_for_a_non_null_object() {
        check_over_base(0x00df_c810, 0x010c_e15c, 0x160);
    }

    #[test]
    fn fn_00dfcf20_runs_the_staged_constructor_for_a_non_null_object() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.call(0x00df_cf20, &args![0u32, 5u32]);
        e.mem.write(object, &[0x66; 0x20]);
        e.call(0x00df_cf20, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010e_2144);
        assert_eq!(e.mem.u32(object + 0x10), 0x010e_2124);
        assert_eq!(e.mem.u32(object + 0xc), 0x6666_6666);
        assert_eq!(e.mem.u8(object + 0x15), 0x66);
        e.call(0x00df_cf20, &args![object, 5u32]);
        assert_eq!(e.mem.u32(object + 0xc), 0x17);
        assert_eq!(e.mem.u8(object + 0x15), 4);
    }
}
