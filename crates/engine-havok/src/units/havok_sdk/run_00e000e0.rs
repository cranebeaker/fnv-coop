//! `Havok SDK/run_00e000e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e000e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x60-byte stack buffer by calling `00d50c00` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e000e0(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, buffer| {
        e.call(0x00d5_0c00, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e003b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x170-byte stack buffer by calling `00e003d0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e003b0(e: &mut Engine) -> u32 {
    e.with_stack(0x170, |e, buffer| {
        e.call(0x00e0_03d0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e004d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0xe0-byte stack buffer by calling `00e004f0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e004d0(e: &mut Engine) -> u32 {
    e.with_stack(0xe0, |e, buffer| {
        e.call(0x00e0_04f0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e005f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00e00610` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e005f0(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00e0_0610, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e006b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00e006d0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e006b0(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00e0_06d0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e00150 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6cdc.
pub fn return_constant_00e00150(_e: &mut Engine) -> u32 {
    0x010c_6cdc
}

// Translated from 00e00190 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_81a4.
pub fn return_constant_00e00190(_e: &mut Engine) -> u32 {
    0x010e_81a4
}

// Translated from 00e001f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x20-byte frame stores the vtable word 0x010c_c2fc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the constant 0x010d_79c4 (the stored word is not read back). Nothing is destroyed.
pub fn fn_00e001f0(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        0x010d_79c4
    })
}

// Translated from 00e00290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x50-byte frame zeroes the `u16` at +0x24, stores
/// the vtable word 0x010c_c040 at the start, 0 at +0x20 and 0x8000 in the `u16` at +0x26, runs the
/// constructor at `00cb0a70` on the member at frame+0x28 (argument 0) and returns the constant 0x010d_79e8 (the stored word is not read back).
pub fn fn_00e00290(e: &mut Engine) -> u32 {
    e.with_stack(0x50, |e, frame| {
        let base = frame.addr();
        e.mem.set_u16(base + 0x24, 0);
        e.mem.set_u32(base, 0x010c_c040);
        e.mem.set_u32(base + 0x20, 0);
        e.mem.set_u16(base + 0x26, 0x8000);
        e.call(0x00cb_0a70, &args![base + 0x28, 0u32]);
        0x010d_79e8
    })
}

// Translated from 00e00320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6ea4.
pub fn return_constant_00e00320(_e: &mut Engine) -> u32 {
    0x010c_6ea4
}

// Translated from 00e00360 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_83b4.
pub fn return_constant_00e00360(_e: &mut Engine) -> u32 {
    0x010e_83b4
}

// Translated from 00e00480 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_6dc4.
pub fn return_constant_00e00480(_e: &mut Engine) -> u32 {
    0x010c_6dc4
}

// Translated from 00e005a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_749c.
pub fn return_constant_00e005a0(_e: &mut Engine) -> u32 {
    0x010c_749c
}

// Translated from 00e00780 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_c41c.
pub fn return_constant_00e00780(_e: &mut Engine) -> u32 {
    0x010c_c41c
}

// Translated from 00e007e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_b5ac.
pub fn return_constant_00e007e0(_e: &mut Engine) -> u32 {
    0x010c_b5ac
}

// Translated from 00e00820 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_8824.
pub fn return_constant_00e00820(_e: &mut Engine) -> u32 {
    0x010e_8824
}

// Translated from 00e001c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cc2fc`, runs
/// `00cb0a70` (an `hkStringPtr` function, `this` = `object + 0x14`, one stack word `extra`) and
/// writes the vtable word `010d79c4`. The function takes two stack words and pops none (`RET`),
/// and returns nothing. The class is unconfirmed.
pub fn fn_00e001c0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.mem.set_u32(object.addr(), 0x010d_79c4);
    }
}

// Translated from 00e00240 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cc040`, the
/// 16-bit words `0x8000` at `+0x26` and 0 at `+0x24`, the word 0 at `+0x20`, runs `00cb0a70`
/// (an `hkStringPtr` function, `this` = `object + 0x28`, one stack word `extra`), writes
/// `0xfffffff0` at `+0x34` and the vtable word `010d79e8`. The function takes two stack words and
/// pops none (`RET`), and returns nothing. The class is unconfirmed.
pub fn fn_00e00240(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        let base = object.addr();
        e.mem.set_u32(base, 0x010c_c040);
        e.mem.set_u16(base + 0x26, 0x8000);
        e.mem.set_u32(base + 0x20, 0);
        e.mem.set_u16(base + 0x24, 0);
        e.call(0x00cb_0a70, &args![base + 0x28, extra]);
        e.mem.set_u32(base + 0x34, 0xffff_fff0);
        e.mem.set_u32(base, 0x010d_79e8);
    }
}

// Translated from 00e003d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor (`this` in `ECX`, one stack word `extra` that the
/// callee pops). It runs `00cd2c50` on `this` with `extra`, then empties three `hkArray`-shaped
/// triples (data, size, capacity-and-flags = `0x80000000`) at `+0x8c`, `+0x98` and `+0x160`, with
/// the vtable word at `+0` set to `010d087c` after the second one. Returns `this` (`EAX`). The
/// class is unconfirmed (the same layout as `00dfc850`).
pub fn fn_00e003d0(e: &mut Engine, this: Ptr, extra: u32) -> Ptr {
    let base = this.addr();
    e.call(0x00cd_2c50, &args![this, extra]);
    e.mem.set_u32(base + 0x8c, 0);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u32(base + 0x94, 0x8000_0000);
    e.mem.set_u32(base + 0x98, 0);
    e.mem.set_u32(base + 0x9c, 0);
    e.mem.set_u32(base + 0xa0, 0x8000_0000);
    e.mem.set_u32(base, 0x010d_087c);
    e.mem.set_u32(base + 0x160, 0);
    e.mem.set_u32(base + 0x164, 0);
    e.mem.set_u32(base + 0x168, 0x8000_0000);
    this
}

// Translated from 00e004f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor (`this` in `ECX`, one stack word `extra` that the
/// callee pops). It runs `00cd2c50` on `this` with `extra`, then empties three `hkArray`-shaped
/// triples (data, size, capacity-and-flags = `0x80000000`) at `+0x8c`, `+0x98` and `+0xd0`, with
/// the vtable word at `+0` set to `010cc004` after the second one. Returns `this` (`EAX`). The
/// class is unconfirmed.
pub fn fn_00e004f0(e: &mut Engine, this: Ptr, extra: u32) -> Ptr {
    let base = this.addr();
    e.call(0x00cd_2c50, &args![this, extra]);
    e.mem.set_u32(base + 0x8c, 0);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u32(base + 0x94, 0x8000_0000);
    e.mem.set_u32(base + 0x98, 0);
    e.mem.set_u32(base + 0x9c, 0);
    e.mem.set_u32(base + 0xa0, 0x8000_0000);
    e.mem.set_u32(base, 0x010c_c004);
    e.mem.set_u32(base + 0xd0, 0);
    e.mem.set_u32(base + 0xd4, 0);
    e.mem.set_u32(base + 0xd8, 0x8000_0000);
    this
}

// Translated from 00e00610 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`). The vtable word at `+0` takes three values in turn (`010305a8`,
/// `010cc364`, `010ce390`) and the word at `+0x10` three (`010c6fb0`, `010cc344`, `010ce370`);
/// when `mode` is not zero the stages also write `+0xc` (0, 7, 7) and the byte at `+0x15` (3,
/// 2). Returns `this` (`EAX`). The class is unconfirmed.
pub fn fn_00e00610(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
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
    e.mem.set_u32(base, 0x010c_e390);
    e.mem.set_u32(base + 0x10, 0x010c_e370);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 7);
        e.mem.set_u8(base + 0x15, 2);
    }
    this
}

// Translated from 00e006d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`). The vtable word at `+0` takes three values in turn (`010305a8`,
/// `010ce3c8`, `010ce404`) and the word at `+0x14` is set to `010c6fd0` at the end; when `mode`
/// is not zero the stages also write `+0xc` (0, 8, 8) and the byte at `+0x10` (2, 1). Returns
/// `this` (`EAX`). The class is unconfirmed.
pub fn fn_00e006d0(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_05a8);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base, 0x010c_e3c8);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 8);
        e.mem.set_u8(base + 0x10, 2);
    }
    e.mem.set_u32(base, 0x010c_e404);
    e.mem.set_u32(base + 0x14, 0x010c_6fd0);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 8);
        e.mem.set_u8(base + 0x10, 1);
    }
    this
}

// Translated from 00e000a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc53d9.
pub fn fn_00e000a0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e000c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00d50c00` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc53cf.
pub fn fn_00e000c0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.call(0x00d5_0c00, &args![object, extra]);
    }
}

// Translated from 00e00100 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5449.
pub fn fn_00e00100(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00120 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped). When
/// `object` is not null it writes the vtable word `010c59f4` at its start; when `mode` is not zero it
/// then stores the high 16 bits of the float 1.0 (`FLD1`, `FSTP` into the stack slot of the first
/// argument, `SAR 16`), `0x3f80`, in the 16-bit word at `+0x11e`; and it ends with the vtable word
/// `010c6cdc`. Returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc543f.
pub fn fn_00e00120(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x010c_59f4);
        if mode != 0 {
            let one = 1.0f32.to_bits() as i32;
            e.mem.set_u16(base + 0x11e, (one >> 16) as u16);
        }
        e.mem.set_u32(base, 0x010c_6cdc);
    }
}

// Translated from 00e00160 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc54b9.
pub fn fn_00e00160(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00180 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e81a4` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc54af.
pub fn fn_00e00180(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_81a4);
    }
}

// Translated from 00e001a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5529.
pub fn fn_00e001a0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00220 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5599.
pub fn fn_00e00220(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e002d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5609.
pub fn fn_00e002d0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e002f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped). When
/// `object` is not null it writes the vtable word `010c59f4` at its start; when `mode` is not zero it
/// then stores the high 16 bits of the float 1.0 (`FLD1`, `FSTP` into the stack slot of the first
/// argument, `SAR 16`), `0x3f80`, in the 16-bit word at `+0x11e`; and it ends with the vtable word
/// `010c6ea4`. Returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc55ff.
pub fn fn_00e002f0(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x010c_59f4);
        if mode != 0 {
            let one = 1.0f32.to_bits() as i32;
            e.mem.set_u16(base + 0x11e, (one >> 16) as u16);
        }
        e.mem.set_u32(base, 0x010c_6ea4);
    }
}

// Translated from 00e00330 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5699.
pub fn fn_00e00330(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00350 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e83b4` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc568f.
pub fn fn_00e00350(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_83b4);
    }
}

// Translated from 00e00370 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5719.
pub fn fn_00e00370(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00390 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00e003d0` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc570f.
pub fn fn_00e00390(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        fn_00e003d0(e, object, extra);
    }
}

// Translated from 00e00430 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5789.
pub fn fn_00e00430(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00450 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped). When
/// `object` is not null it writes the vtable word `010c59f4` at its start; when `mode` is not zero it
/// then stores the high 16 bits of the float 1.0 (`FLD1`, `FSTP` into the stack slot of the first
/// argument, `SAR 16`), `0x3f80`, in the 16-bit word at `+0x11e`; and it ends with the vtable word
/// `010c6dc4`. Returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc577f.
pub fn fn_00e00450(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x010c_59f4);
        if mode != 0 {
            let one = 1.0f32.to_bits() as i32;
            e.mem.set_u16(base + 0x11e, (one >> 16) as u16);
        }
        e.mem.set_u32(base, 0x010c_6dc4);
    }
}

// Translated from 00e00490 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5809.
pub fn fn_00e00490(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e004b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00e004f0` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc57ff.
pub fn fn_00e004b0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        fn_00e004f0(e, object, extra);
    }
}

// Translated from 00e00550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5899.
pub fn fn_00e00550(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped); a
/// constructor chain the compiler collapsed. When `object` is not null it writes the vtable word
/// `010305a8` at its start, the word 0 at `+0xc` when `mode` is not zero, then the vtable word
/// `010c749c`, and `0x3` at `+0xc` whatever `mode` is; returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc588f.
pub fn fn_00e00570(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x0103_05a8);
        if mode != 0 {
            e.mem.set_u32(base + 0xc, 0);
        }
        e.mem.set_u32(base, 0x010c_749c);
        e.mem.set_u32(base + 0xc, 0x3);
    }
}

// Translated from 00e005b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5909.
pub fn fn_00e005b0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e005d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00e00610` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc58ff.
pub fn fn_00e005d0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        fn_00e00610(e, object, extra);
    }
}

// Translated from 00e00670 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5979.
pub fn fn_00e00670(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00690 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `extra` (cdecl, nothing popped). When
/// `object` is not null, runs `00e006d0` on it (`this` in `ECX`, `extra` pushed and popped by the
/// callee); returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc596f.
pub fn fn_00e00690(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        fn_00e006d0(e, object, extra);
    }
}

// Translated from 00e00720 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc59e9.
pub fn fn_00e00720(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00740 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped); a
/// constructor chain the compiler collapsed. When `object` is not null it writes the vtable word
/// `010305a8` at its start, the word 0 at `+0xc` when `mode` is not zero, then the vtable word
/// `010cc41c`, the word `010c6fd0` at `+0x10`, and `0xf` at `+0xc` whatever `mode` is; returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc59df.
pub fn fn_00e00740(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x0103_05a8);
        if mode != 0 {
            e.mem.set_u32(base + 0xc, 0);
        }
        e.mem.set_u32(base, 0x010c_c41c);
        e.mem.set_u32(base + 0x10, 0x010c_6fd0);
        e.mem.set_u32(base + 0xc, 0xf);
    }
}

// Translated from 00e00790 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5a59.
pub fn fn_00e00790(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e007b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words `object` and `mode` (cdecl, nothing popped); a
/// constructor chain the compiler collapsed. When `object` is not null it writes the vtable word
/// `010305a8` at its start, the word 0 at `+0xc` when `mode` is not zero, then the vtable word
/// `010cb5ac`, and `0x1` at `+0xc` whatever `mode` is; returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5a4f.
pub fn fn_00e007b0(e: &mut Engine, object: Ptr, mode: u32) {
    let base = object.addr();
    if base != 0 {
        e.mem.set_u32(base, 0x0103_05a8);
        if mode != 0 {
            e.mem.set_u32(base + 0xc, 0);
        }
        e.mem.set_u32(base, 0x010c_b5ac);
        e.mem.set_u32(base + 0xc, 0x1);
    }
}

// Translated from 00e007f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc5ac9.
pub fn fn_00e007f0(e: &mut Engine, object: Ptr) {
    e.vcall(object.addr(), 0, &args![0u32]);
}

// Translated from 00e00810 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e8824` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc5abf.
pub fn fn_00e00810(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_8824);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_00e0, fn_00e000e0() -> u32),
        entry!(0x00e0_03b0, fn_00e003b0() -> u32),
        entry!(0x00e0_04d0, fn_00e004d0() -> u32),
        entry!(0x00e0_05f0, fn_00e005f0() -> u32),
        entry!(0x00e0_06b0, fn_00e006b0() -> u32),
        entry!(0x00e00150, return_constant_00e00150() -> u32),
        entry!(0x00e00190, return_constant_00e00190() -> u32),
        entry!(0x00e001f0, fn_00e001f0() -> u32),
        entry!(0x00e00290, fn_00e00290() -> u32),
        entry!(0x00e00320, return_constant_00e00320() -> u32),
        entry!(0x00e00360, return_constant_00e00360() -> u32),
        entry!(0x00e00480, return_constant_00e00480() -> u32),
        entry!(0x00e005a0, return_constant_00e005a0() -> u32),
        entry!(0x00e00780, return_constant_00e00780() -> u32),
        entry!(0x00e007e0, return_constant_00e007e0() -> u32),
        entry!(0x00e00820, return_constant_00e00820() -> u32),
        entry!(0x00e0_01c0, fn_00e001c0(Ptr, u32)),
        entry!(0x00e0_0240, fn_00e00240(Ptr, u32)),
        entry!(0x00e0_03d0, fn_00e003d0(Ptr, u32) -> Ptr),
        entry!(0x00e0_04f0, fn_00e004f0(Ptr, u32) -> Ptr),
        entry!(0x00e0_0610, fn_00e00610(Ptr, u32) -> Ptr),
        entry!(0x00e0_06d0, fn_00e006d0(Ptr, u32) -> Ptr),
        entry!(0x00e0_00a0, fn_00e000a0(Ptr)),
        entry!(0x00e0_00c0, fn_00e000c0(Ptr, u32)),
        entry!(0x00e0_0100, fn_00e00100(Ptr)),
        entry!(0x00e0_0120, fn_00e00120(Ptr, u32)),
        entry!(0x00e0_0160, fn_00e00160(Ptr)),
        entry!(0x00e0_0180, fn_00e00180(Ptr, u32)),
        entry!(0x00e0_01a0, fn_00e001a0(Ptr)),
        entry!(0x00e0_0220, fn_00e00220(Ptr)),
        entry!(0x00e0_02d0, fn_00e002d0(Ptr)),
        entry!(0x00e0_02f0, fn_00e002f0(Ptr, u32)),
        entry!(0x00e0_0330, fn_00e00330(Ptr)),
        entry!(0x00e0_0350, fn_00e00350(Ptr, u32)),
        entry!(0x00e0_0370, fn_00e00370(Ptr)),
        entry!(0x00e0_0390, fn_00e00390(Ptr, u32)),
        entry!(0x00e0_0430, fn_00e00430(Ptr)),
        entry!(0x00e0_0450, fn_00e00450(Ptr, u32)),
        entry!(0x00e0_0490, fn_00e00490(Ptr)),
        entry!(0x00e0_04b0, fn_00e004b0(Ptr, u32)),
        entry!(0x00e0_0550, fn_00e00550(Ptr)),
        entry!(0x00e0_0570, fn_00e00570(Ptr, u32)),
        entry!(0x00e0_05b0, fn_00e005b0(Ptr)),
        entry!(0x00e0_05d0, fn_00e005d0(Ptr, u32)),
        entry!(0x00e0_0670, fn_00e00670(Ptr)),
        entry!(0x00e0_0690, fn_00e00690(Ptr, u32)),
        entry!(0x00e0_0720, fn_00e00720(Ptr)),
        entry!(0x00e0_0740, fn_00e00740(Ptr, u32)),
        entry!(0x00e0_0790, fn_00e00790(Ptr)),
        entry!(0x00e0_07b0, fn_00e007b0(Ptr, u32)),
        entry!(0x00e0_07f0, fn_00e007f0(Ptr)),
        entry!(0x00e0_0810, fn_00e00810(Ptr, u32)),
    ]
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
        fn_00e000e0_returns_first_word_of_built_object,
        0x00e0_00e0,
        0x00d5_0c00
    );
    first_word_test!(
        fn_00e003b0_returns_first_word_of_built_object,
        0x00e0_03b0,
        0x00e0_03d0
    );
    first_word_test!(
        fn_00e004d0_returns_first_word_of_built_object,
        0x00e0_04d0,
        0x00e0_04f0
    );
    first_word_test!(
        fn_00e005f0_returns_first_word_of_built_object,
        0x00e0_05f0,
        0x00e0_0610
    );
    first_word_test!(
        fn_00e006b0_returns_first_word_of_built_object,
        0x00e0_06b0,
        0x00e0_06d0
    );
}

#[cfg(test)]
mod tests {
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
    fn fn_00e000e0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d5_0c00, 0x0a0b_0c0d);
        let result = e.call(0x00e0_00e0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e00150_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0150, &args![]).u32(), 0x010c_6cdc);
    }

    #[test]
    fn return_constant_00e00190_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0190, &args![]).u32(), 0x010e_81a4);
    }

    #[test]
    fn fn_00e001f0_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00e0_01f0, &args![]).u32();
        assert_eq!(result, 0x010d_79c4);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c2fc);
        }
    }

    #[test]
    fn fn_00e00290_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let frame = e.mem.alloc(0x50);
        for offset in (0..0x50).step_by(4) {
            e.mem.set_u32(frame + offset, 0xffff_ffff);
        }
        e.mem.free(frame);
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x28], true);
        let result = e.call(0x00e0_0290, &args![]).u32();
        assert_eq!(result, 0x010d_79e8);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c040);
        }
        assert_eq!(calls[0].3, [0, 0, 0x8000]);
    }

    #[test]
    fn return_constant_00e00320_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0320, &args![]).u32(), 0x010c_6ea4);
    }

    #[test]
    fn return_constant_00e00360_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0360, &args![]).u32(), 0x010e_83b4);
    }

    #[test]
    fn fn_00e003b0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_03d0, 0x0a0b_0c0d);
        let result = e.call(0x00e0_03b0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e00480_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0480, &args![]).u32(), 0x010c_6dc4);
    }

    #[test]
    fn fn_00e004d0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_04f0, 0x0a0b_0c0d);
        let result = e.call(0x00e0_04d0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e005a0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_05a0, &args![]).u32(), 0x010c_749c);
    }

    #[test]
    fn fn_00e005f0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_0610, 0x0a0b_0c0d);
        let result = e.call(0x00e0_05f0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00e006b0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_06d0, 0x0a0b_0c0d);
        let result = e.call(0x00e0_06b0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e00780_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0780, &args![]).u32(), 0x010c_c41c);
    }

    #[test]
    fn return_constant_00e007e0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_07e0, &args![]).u32(), 0x010c_b5ac);
    }

    #[test]
    fn return_constant_00e00820_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0820, &args![]).u32(), 0x010e_8824);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// A null object does nothing; otherwise the vtable word ends at the second value and the
    /// string function runs in between on `object + 0x14`, seeing the first value.
    #[test]
    fn fn_00e001c0_runs_the_string_function_between_the_vtable_writes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.register_double(0x00cb_0a70, move |e, a| {
            assert_eq!(a, [object + 0x14, 7]);
            assert_eq!(e.mem.u32(object), 0x010c_c2fc);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e0_01c0, &args![0u32, 7u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call(0x00e0_01c0, &args![object, 7u32]);
        assert_eq!(e.mem.u32(object), 0x010d_79c4);
    }

    /// The fields are written around the string function (which runs on `object + 0x28`).
    #[test]
    fn fn_00e00240_sets_its_fields_around_the_string_function() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.mem.write(object, &[0x77; 0x40]);
        e.register_double(0x00cb_0a70, move |e, a| {
            assert_eq!(a, [object + 0x28, 3]);
            assert_eq!(e.mem.u32(object), 0x010c_c040);
            assert_eq!(e.mem.u16(object + 0x26), 0x8000);
            assert_eq!(e.mem.u16(object + 0x24), 0);
            assert_eq!(e.mem.u32(object + 0x20), 0);
            assert_eq!(e.mem.u32(object + 0x34), 0x7777_7777);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e0_0240, &args![0u32, 3u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call(0x00e0_0240, &args![object, 3u32]);
        assert_eq!(e.mem.u32(object), 0x010d_79e8);
        assert_eq!(e.mem.u32(object + 0x34), 0xffff_fff0);
        assert_eq!(e.mem.u16(object + 0x1e), 0x7777);
    }

    /// Runs a constructor of the `00cd2c50` family on a block filled with 0x55 bytes, with the
    /// base constructor as a double that checks its words and that nothing is written yet.
    fn construct(address: u32, vtable: u32, tail: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x170);
        e.mem.write(this, &[0x55; 0x170]);
        e.register_double(0x00cd_2c50, move |e, a| {
            assert_eq!(a, [this, 9]);
            assert_eq!(e.mem.u32(this + 0x8c), 0x5555_5555);
            Ret::default()
        });
        assert_eq!(e.call(address, &args![this, 9u32]).u32(), this);
        assert_eq!(e.mem.u32(this), vtable);
        for offset in [0x8c, 0x90, 0x98, 0x9c, tail, tail + 4] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        for offset in [0x94, 0xa0, tail + 8] {
            assert_eq!(e.mem.u32(this + offset), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0x88), 0x5555_5555);
        assert_eq!(e.mem.u32(this + 0xa4), 0x5555_5555);
        assert_eq!(e.mem.u32(this + tail + 0xc), 0x5555_5555);
    }

    #[test]
    fn fn_00e003d0_constructs_over_the_base() {
        construct(0x00e0_03d0, 0x010d_087c, 0x160);
    }

    #[test]
    fn fn_00e004f0_constructs_over_the_base() {
        construct(0x00e0_04f0, 0x010c_c004, 0xd0);
    }

    /// Mode 0 leaves +0xc and the byte at +0x15 alone; another mode ends with 7 and 2.
    #[test]
    fn fn_00e00610_stages_depend_on_the_mode() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0x66; 0x20]);
        assert_eq!(e.call(0x00e0_0610, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_e390);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_e370);
        assert_eq!(e.mem.u32(this + 0xc), 0x6666_6666);
        assert_eq!(e.mem.u8(this + 0x15), 0x66);
        e.call(0x00e0_0610, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this + 0xc), 7);
        assert_eq!(e.mem.u8(this + 0x15), 2);
    }

    /// Mode 0 leaves +0xc and the byte at +0x10 alone; another mode ends with 8 and 1.
    #[test]
    fn fn_00e006d0_stages_depend_on_the_mode() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0x66; 0x20]);
        assert_eq!(e.call(0x00e0_06d0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_e404);
        assert_eq!(e.mem.u32(this + 0x14), 0x010c_6fd0);
        assert_eq!(e.mem.u32(this + 0xc), 0x6666_6666);
        assert_eq!(e.mem.u8(this + 0x10), 0x66);
        e.call(0x00e0_06d0, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(e.mem.u8(this + 0x10), 1);
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

    /// The 16-bit word at `+0x11e` takes the high half of 1.0f only when `mode` is not zero; the
    /// object ends with `vtable` either way.
    fn check_half(address: u32, vtable: u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x130);
        e.call(address, &args![0u32, 1u32]);
        e.mem.write(object, &[0x66; 0x130]);
        e.call(address, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), vtable);
        assert_eq!(e.mem.u16(object + 0x11e), 0x6666);
        e.call(address, &args![object, 9u32]);
        assert_eq!(e.mem.u32(object), vtable);
        assert_eq!(e.mem.u16(object + 0x11e), 0x3f80);
        assert_eq!(e.mem.u16(object + 0x11c), 0x6666);
        assert_eq!(e.mem.u16(object + 0x120), 0x6666);
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
    fn fn_00e000a0_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_00a0);
    }

    #[test]
    fn fn_00e000c0_runs_the_constructor_on_a_non_null_object() {
        check_forward(0x00e0_00c0, 0x00d5_0c00);
    }

    #[test]
    fn fn_00e00100_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0100);
    }

    #[test]
    fn fn_00e00120_stores_the_half_float_one_only_when_mode_is_set() {
        check_half(0x00e0_0120, 0x010c_6cdc);
    }

    #[test]
    fn fn_00e00160_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0160);
    }

    #[test]
    fn fn_00e00180_stores_its_vtable_word() {
        check_vtable_store(0x00e0_0180, 0x010e_81a4);
    }

    #[test]
    fn fn_00e001a0_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_01a0);
    }

    #[test]
    fn fn_00e00220_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0220);
    }

    #[test]
    fn fn_00e002d0_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_02d0);
    }

    #[test]
    fn fn_00e002f0_stores_the_half_float_one_only_when_mode_is_set() {
        check_half(0x00e0_02f0, 0x010c_6ea4);
    }

    #[test]
    fn fn_00e00330_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0330);
    }

    #[test]
    fn fn_00e00350_stores_its_vtable_word() {
        check_vtable_store(0x00e0_0350, 0x010e_83b4);
    }

    #[test]
    fn fn_00e00370_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0370);
    }

    #[test]
    fn fn_00e00430_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0430);
    }

    #[test]
    fn fn_00e00450_stores_the_half_float_one_only_when_mode_is_set() {
        check_half(0x00e0_0450, 0x010c_6dc4);
    }

    #[test]
    fn fn_00e00490_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0490);
    }

    #[test]
    fn fn_00e00550_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0550);
    }

    #[test]
    fn fn_00e00570_writes_the_final_vtable_and_the_value() {
        check_staged(0x00e0_0570, 0x010c_749c, &[], 0x3);
    }

    #[test]
    fn fn_00e005b0_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_05b0);
    }

    #[test]
    fn fn_00e00670_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0670);
    }

    #[test]
    fn fn_00e00720_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0720);
    }

    #[test]
    fn fn_00e00740_writes_the_final_vtable_and_the_value() {
        check_staged(0x00e0_0740, 0x010c_c41c, &[(0x10, 0x010c_6fd0)], 0xf);
    }

    #[test]
    fn fn_00e00790_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_0790);
    }

    #[test]
    fn fn_00e007b0_writes_the_final_vtable_and_the_value() {
        check_staged(0x00e0_07b0, 0x010c_b5ac, &[], 0x1);
    }

    #[test]
    fn fn_00e007f0_calls_slot_zero_with_zero() {
        check_cleanup(0x00e0_07f0);
    }

    #[test]
    fn fn_00e00810_stores_its_vtable_word() {
        check_vtable_store(0x00e0_0810, 0x010e_8824);
    }

    #[test]
    fn fn_00e00390_constructs_over_the_base_for_a_non_null_object() {
        check_over_base(0x00e0_0390, 0x010d_087c, 0x160);
    }

    #[test]
    fn fn_00e004b0_constructs_over_the_base_for_a_non_null_object() {
        check_over_base(0x00e0_04b0, 0x010c_c004, 0xd0);
    }

    #[test]
    fn fn_00e005d0_runs_the_staged_constructor_for_a_non_null_object() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.call(0x00e0_05d0, &args![0u32, 5u32]);
        e.mem.write(object, &[0x66; 0x20]);
        e.call(0x00e0_05d0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010c_e390);
        assert_eq!(e.mem.u32(object + 0x10), 0x010c_e370);
        assert_eq!(e.mem.u32(object + 0xc), 0x6666_6666);
        assert_eq!(e.mem.u8(object + 0x15), 0x66);
        e.call(0x00e0_05d0, &args![object, 5u32]);
        assert_eq!(e.mem.u32(object + 0xc), 7);
        assert_eq!(e.mem.u8(object + 0x15), 2);
    }

    #[test]
    fn fn_00e00690_runs_the_staged_constructor_for_a_non_null_object() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.call(0x00e0_0690, &args![0u32, 5u32]);
        e.mem.write(object, &[0x66; 0x20]);
        e.call(0x00e0_0690, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010c_e404);
        assert_eq!(e.mem.u32(object + 0x14), 0x010c_6fd0);
        assert_eq!(e.mem.u32(object + 0xc), 0x6666_6666);
        assert_eq!(e.mem.u8(object + 0x10), 0x66);
        e.call(0x00e0_0690, &args![object, 5u32]);
        assert_eq!(e.mem.u32(object + 0xc), 8);
        assert_eq!(e.mem.u8(object + 0x10), 1);
    }
}
