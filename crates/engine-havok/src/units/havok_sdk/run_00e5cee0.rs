//! `Havok SDK/run_00e5cee0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds wrappers that build a MOPP query machine on their frame and call its query
//! method, four interpreters of MOPP code (`00e5d7d0`, `00e5e430`, `00e5f070`, `00e5fb50`: the
//! same loop for different query shapes) with the set-ups that feed them (`00e5d070`, `00e5d360`,
//! `00e5d650`, `00e5d710`), and the methods of a shape-collection object (`00e60650` onwards).
//! The interpreters are transcribed instruction by instruction (`Fpu`, `Flags` and the SSE
//! models of `run_00d6bf60`); the compiler's switch of their dispatch (a byte map of the code byte and a jump table of
//! 23 cases) is written as two `match`es.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00d6bf60::{
    addps, addss, andps, cmpltps, load_vec, maxps, minps, movmskps, mulps, shufps, store_vec,
    subps, unpckhps, unpcklps, xorps,
};
use crate::units::havok_sdk::run_00e53090::{time_stamp, Flags, Fpu};

// Translated from 00e5cee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An empty function (a single `RET`, no arguments).
pub fn fn_00e5cee0(_e: &mut Engine) {}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wrapper (cdecl, four stack words) around `hkpMoppEarlyExitObbVirtualMachine::queryObb`
/// (`00e647e0`, Xbox PDB name from the engine map): the virtual machine object is a block of the
/// frame passed as `this`, the result byte is another frame slot, the four words are passed on
/// after it, and the answer is whether the byte the call returns the address of is non-zero.
pub fn fn_00e5cf00(e: &mut Engine, first: u32, second: u32, third: u32, fourth: u32) -> bool {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), first);
        e.mem.set_u32(esp.wrapping_add(8), second);
        e.mem.set_u32(esp.wrapping_add(12), third);
        e.mem.set_u32(esp.wrapping_add(16), fourth);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5cf00;
        loop {
            match pc {
                0x00e5cf00 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5cf00
                    ebp = esp; // 00e5cf01
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5cf03
                    eax = e.mem.u32(ebp.wrapping_add(0x14)); // 00e5cf06
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5cf09
                    edx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5cf0c
                    {
                        let r = fl.sub(esp, 0x00000060u32, 32);
                        esp = r;
                    } // 00e5cf0f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5cf12
                    eax = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5cf13
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5cf16
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5cf17
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5cf18
                    ecx = esp.wrapping_add(0x1f); // 00e5cf19
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5cf1d
                    ecx = esp.wrapping_add(0x24); // 00e5cf1e
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                            e.mem.u32(esp.wrapping_add(16)),
                        ];
                        let r = e.call(0x00e647e0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(20); // 00e5cf22
                    {
                        let r = fl.xor(edx, edx, 32);
                        edx = r;
                    } // 00e5cf27
                    fl.sub(u32::from(e.mem.u8(eax)), (edx & 0xff), 8); // 00e5cf29
                    edx = (edx & 0xffff_ff00) | (((!fl.zf) as u32) & 0xff); // 00e5cf2b
                    eax = edx; // 00e5cf2e
                    esp = ebp; // 00e5cf30
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5cf32
                    return (eax as u8) != 0; // 00e5cf33
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5cf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wrapper (cdecl, three stack words): calls `00e64f00` with a block of the frame as `this` and
/// the three words as its stack arguments.
pub fn fn_00e5cf40(e: &mut Engine, first: u32, second: u32, third: u32) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), first);
        e.mem.set_u32(esp.wrapping_add(8), second);
        e.mem.set_u32(esp.wrapping_add(12), third);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5cf40;
        loop {
            match pc {
                0x00e5cf40 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5cf40
                    ebp = esp; // 00e5cf41
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5cf43
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5cf46
                    ecx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5cf49
                    edx = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5cf4c
                    {
                        let r = fl.sub(esp, 0x00000020u32, 32);
                        esp = r;
                    } // 00e5cf4f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5cf52
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5cf53
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5cf54
                    ecx = esp.wrapping_add(0xc); // 00e5cf55
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00e64f00, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(12); // 00e5cf59
                    esp = ebp; // 00e5cf5e
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5cf60
                    return; // 00e5cf61
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5cf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wrapper (cdecl, three stack words): a 0xd0-byte object of the frame is set up with `00e65e70`,
/// `00e65d90` is called on it with the three words, and `00e65ea0` takes it down.
pub fn fn_00e5cf70(e: &mut Engine, first: u32, second: u32, third: u32) {
    e.with_stack(0x400, |e, frame| {
        let mut esp = frame.addr() + 0x300;
        e.mem.set_u32(esp.wrapping_add(4), first);
        e.mem.set_u32(esp.wrapping_add(8), second);
        e.mem.set_u32(esp.wrapping_add(12), third);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5cf70;
        loop {
            match pc {
                0x00e5cf70 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5cf70
                    ebp = esp; // 00e5cf71
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5cf73
                    {
                        let r = fl.sub(esp, 0x000000d0u32, 32);
                        esp = r;
                    } // 00e5cf76
                    ecx = esp; // 00e5cf7c
                    {
                        let w = [ecx];
                        let r = e.call(0x00e65e70, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5cf7f
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5cf84
                    ecx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5cf87
                    edx = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5cf8a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5cf8d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5cf8e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5cf8f
                    ecx = esp.wrapping_add(0xc); // 00e5cf90
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00e65d90, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(12); // 00e5cf94
                    ecx = esp; // 00e5cf99
                    {
                        let w = [ecx];
                        let r = e.call(0x00e65ea0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5cf9c
                    esp = ebp; // 00e5cfa1
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5cfa3
                    return; // 00e5cfa4
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wrapper (cdecl, two stack words): builds a three-word record on the frame (zero, zero, `word`) and
/// calls `00e65ee0` on it with the word at `object + 0x20` as the stack argument.
pub fn fn_00e5cfb0(e: &mut Engine, object: Ptr, word: u32) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), object.addr());
        e.mem.set_u32(esp.wrapping_add(8), word);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5cfb0;
        loop {
            match pc {
                0x00e5cfb0 => {
                    {
                        let r = fl.sub(esp, 0x00000018u32, 32);
                        esp = r;
                    } // 00e5cfb0
                    eax = e.mem.u32(esp.wrapping_add(0x20)); // 00e5cfb3
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5cfb7
                    edx = e.mem.u32(ecx.wrapping_add(0x20)); // 00e5cfbb
                    e.mem.set_u32(esp.wrapping_add(0x8), eax); // 00e5cfbe
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    } // 00e5cfc2
                    e.mem.set_u32(esp, eax); // 00e5cfc4
                    e.mem.set_u32(esp.wrapping_add(0x4), eax); // 00e5cfc7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5cfcb
                    eax = esp.wrapping_add(0x4); // 00e5cfcc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5cfd0
                    ecx = esp.wrapping_add(0x10); // 00e5cfd1
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e65ee0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5cfd5
                    {
                        let r = fl.add(esp, 0x00000018u32, 32);
                        esp = r;
                    } // 00e5cfda
                    return; // 00e5cfdd
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wrapper (cdecl, five stack words, the fourth a float) for `hkpMoppObbVirtualMachine::queryObb`
/// (`00dd1bf0`, Xbox PDB name): a block of the frame is `this`; the float goes through `FLD`/`FSTP`
/// into the call's argument slot.
pub fn fn_00e5cfe0(e: &mut Engine, first: u32, second: u32, third: u32, distance: f32, fifth: u32) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), first);
        e.mem.set_u32(esp.wrapping_add(8), second);
        e.mem.set_u32(esp.wrapping_add(12), third);
        e.mem.set_u32(esp.wrapping_add(16), distance.to_bits());
        e.mem.set_u32(esp.wrapping_add(20), fifth);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5cfe0;
        loop {
            match pc {
                0x00e5cfe0 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5cfe0
                    ebp = esp; // 00e5cfe1
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5cfe3
                    eax = e.mem.u32(ebp.wrapping_add(0x18)); // 00e5cfe6
                    fpu.fld_m32(e.mem.u32(ebp.wrapping_add(0x14))); // 00e5cfe9
                    edx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5cfec
                    {
                        let r = fl.sub(esp, 0x00000040u32, 32);
                        esp = r;
                    } // 00e5cfef
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5cff2
                    eax = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5cff3
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5cff6
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5cff7
                    e.mem.set_u32(esp, fpu.fstp_m32()); // 00e5cffa
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5cffd
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5cffe
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5cfff
                    ecx = esp.wrapping_add(0x14); // 00e5d000
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                            e.mem.u32(esp.wrapping_add(16)),
                        ];
                        let r = e.call(0x00dd1bf0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(20); // 00e5d004
                    esp = ebp; // 00e5d009
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d00b
                    return; // 00e5d00c
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5d010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wrapper (cdecl, three stack words) for `hkpMoppObbVirtualMachine::queryAabb` (`00dd1da0`, Xbox PDB
/// name): a block of the frame is `this`.
pub fn fn_00e5d010(e: &mut Engine, first: u32, second: u32, third: u32) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), first);
        e.mem.set_u32(esp.wrapping_add(8), second);
        e.mem.set_u32(esp.wrapping_add(12), third);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5d010;
        loop {
            match pc {
                0x00e5d010 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5d010
                    ebp = esp; // 00e5d011
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5d013
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d016
                    ecx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5d019
                    edx = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5d01c
                    {
                        let r = fl.sub(esp, 0x00000040u32, 32);
                        esp = r;
                    } // 00e5d01f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5d022
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5d023
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5d024
                    ecx = esp.wrapping_add(0xc); // 00e5d025
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00dd1da0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(12); // 00e5d029
                    esp = ebp; // 00e5d02e
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d030
                    return; // 00e5d031
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5d040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wrapper (cdecl, three stack words) for `hkpMoppModifyVirtualMachine::queryAabb` (`00e67230`, Xbox
/// PDB name): a block of the frame is `this`.
pub fn fn_00e5d040(e: &mut Engine, first: u32, second: u32, third: u32) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), first);
        e.mem.set_u32(esp.wrapping_add(8), second);
        e.mem.set_u32(esp.wrapping_add(12), third);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5d040;
        loop {
            match pc {
                0x00e5d040 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5d040
                    ebp = esp; // 00e5d041
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5d043
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d046
                    ecx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5d049
                    edx = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5d04c
                    {
                        let r = fl.sub(esp, 0x00000050u32, 32);
                        esp = r;
                    } // 00e5d04f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5d052
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5d053
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5d054
                    ecx = esp.wrapping_add(0xc); // 00e5d055
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00e67230, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(12); // 00e5d059
                    esp = ebp; // 00e5d05e
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d060
                    return; // 00e5d061
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5d070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Query set-up (cdecl: `object`, `list` (a record of a count and the address of the boxes), `third`,
/// `fourth`): builds the argument block of the interpreter `00e5d7d0` on a large frame: the
/// reciprocal of the float at `object + 0x1c` scaled by 16777216.0 (the `double` at `01091c00`), the
/// boxes transposed four at a time into structure-of-arrays form (`UNPCKLPS`), their absolute
/// values and a doubled centre offset by the vector at `object + 0x10` (`MOVSS`/`SHUFPS`/`MULPS`
/// /`ADDPS`), then calls `00e5d7d0` with the word at `object + 0x20`. Transcribed instruction by
/// instruction.
pub fn fn_00e5d070(e: &mut Engine, object: Ptr, list: Ptr, third: u32, fourth: u32) {
    e.with_stack(0x900, |e, frame| {
        let mut esp = frame.addr() + 0x800;
        e.mem.set_u32(esp.wrapping_add(4), object.addr());
        e.mem.set_u32(esp.wrapping_add(8), list.addr());
        e.mem.set_u32(esp.wrapping_add(12), third);
        e.mem.set_u32(esp.wrapping_add(16), fourth);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5d070;
        loop {
            match pc {
                0x00e5d070 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5d070
                    ebp = esp; // 00e5d071
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5d073
                    {
                        let r = fl.sub(esp, 0x00000484u32, 32);
                        esp = r;
                    } // 00e5d076
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e5d07c
                    ebx = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5d07d
                    fpu.fld_m32(e.mem.u32(ebx.wrapping_add(0x1c))); // 00e5d080
                    edx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5d083
                    fpu.fld1(); // 00e5d086
                    eax = e.mem.u32(ebx.wrapping_add(0x20)); // 00e5d088
                    fpu.fdivrp(1); // 00e5d08b
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d08d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5d090
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5d091
                    edi = e.mem.u32(edx); // 00e5d092
                    edx = e.mem.u32(edx.wrapping_add(0x4)); // 00e5d094
                    e.mem.set_u32(esp.wrapping_add(0x14), eax); // 00e5d097
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    } // 00e5d09b
                    esi = edi; // 00e5d09d
                    {
                        let r = fl.and(esi, 0xfffffffcu32, 32);
                        esi = r;
                    } // 00e5d09f
                    e.mem.set_u32(esp.wrapping_add(0x50), ecx); // 00e5d0a2
                    x[1] = xorps(x[1], x[1]); // 00e5d0a6
                    e.mem.set_u32(esp.wrapping_add(0x20), eax); // 00e5d0a9
                    e.mem.set_u32(esp.wrapping_add(0x24), eax); // 00e5d0ad
                    e.mem.set_u32(esp.wrapping_add(0x28), eax); // 00e5d0b1
                    e.mem.set_u32(esp.wrapping_add(0x2c), 0x00000010u32); // 00e5d0b5
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e5d0bd
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax); // 00e5d0c1
                    store_vec(e, esp.wrapping_add(0x30), x[1]); // 00e5d0c5
                    ecx = esp.wrapping_add(0x90); // 00e5d0ca
                    e.mem.set_u32(esp.wrapping_add(0x80), eax); // 00e5d0d1
                    e.mem.set_u32(esp.wrapping_add(0x60), fpu.fstp_m32()); // 00e5d0d8
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x60))); // 00e5d0dc
                    fpu.fmul_m64(e.mem.u64(0x01091c00u32)); // 00e5d0e0
                    e.mem.set_u32(esp.wrapping_add(0x10), fpu.fstp_m32()); // 00e5d0e6
                    x[0] = [
                        f32::from_bits(e.mem.u32(esp.wrapping_add(0x10))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d0ea
                    x[0] = shufps(x[0], x[0], 0x0); // 00e5d0f0
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5d0f4
                    e.mem.set_u32(esp.wrapping_add(0x10), eax); // 00e5d0f9
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e5d1bf;
                        continue;
                    } // 00e5d0fd
                    {
                        let r = fl.dec(esi, 32);
                        esi = r;
                    } // 00e5d103
                    {
                        let r = fl.shr(esi, 2, 32);
                        esi = r;
                    } // 00e5d104
                    {
                        let r = fl.inc(esi, 32);
                        esi = r;
                    } // 00e5d107
                    ebx = esi.wrapping_mul(4); // 00e5d108
                    eax = edx.wrapping_add(0x24); // 00e5d10f
                    e.mem.set_u32(esp.wrapping_add(0x10), ebx); // 00e5d112
                    pc = 0x00e5d120; // 00e5d116
                    continue;
                }
                0x00e5d120 => {
                    x[3] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x4))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d120
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_add(0xc))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d125
                    x[0] = [f32::from_bits(e.mem.u32(edx)), 0.0, 0.0, 0.0]; // 00e5d12a
                    x[2] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x14))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d12e
                    {
                        let r = fl.add(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5d133
                    {
                        let r = fl.add(edx, 0x00000040u32, 32);
                        edx = r;
                    } // 00e5d136
                    {
                        let r = fl.add(eax, 0x00000040u32, 32);
                        eax = r;
                    } // 00e5d139
                    x[0] = unpcklps(x[0], x[3]); // 00e5d13c
                    x[2] = unpcklps(x[2], x[4]); // 00e5d13f
                    x[0] = unpcklps(x[0], x[2]); // 00e5d142
                    store_vec(e, ecx.wrapping_sub(0x40), x[0]); // 00e5d145
                    x[3] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x40))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d149
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x30))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d14e
                    x[0] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x60))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d153
                    x[2] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x50))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d158
                    x[0] = unpcklps(x[0], x[3]); // 00e5d15d
                    x[2] = unpcklps(x[2], x[4]); // 00e5d160
                    x[0] = unpcklps(x[0], x[2]); // 00e5d163
                    store_vec(e, ecx.wrapping_sub(0x30), x[0]); // 00e5d166
                    x[3] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x3c))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d16a
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x2c))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d16f
                    x[0] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x5c))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d174
                    x[2] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x4c))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d179
                    x[0] = unpcklps(x[0], x[3]); // 00e5d17e
                    x[2] = unpcklps(x[2], x[4]); // 00e5d181
                    x[0] = unpcklps(x[0], x[2]); // 00e5d184
                    store_vec(e, ecx.wrapping_sub(0x20), x[0]); // 00e5d187
                    x[0] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x58))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d18b
                    x[2] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x48))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d190
                    x[3] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x38))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d195
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x28))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d19a
                    x[0] = unpcklps(x[0], x[3]); // 00e5d19f
                    x[2] = unpcklps(x[2], x[4]); // 00e5d1a2
                    x[0] = unpcklps(x[0], x[2]); // 00e5d1a5
                    store_vec(e, ecx.wrapping_sub(0x10), x[0]); // 00e5d1a8
                    {
                        let r = fl.inc(e.mem.u32(esp.wrapping_add(0x80)), 32);
                        e.mem.set_u32(esp.wrapping_add(0x80), r);
                    } // 00e5d1ac
                    {
                        let r = fl.sub(esi, 0x00000001u32, 32);
                        esi = r;
                    } // 00e5d1b3
                    if !fl.zf {
                        pc = 0x00e5d120;
                        continue;
                    } // 00e5d1b6
                    ebx = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5d1bc
                    pc = 0x00e5d1bf;
                }
                0x00e5d1bf => {
                    {
                        let r = fl.sub(edi, e.mem.u32(esp.wrapping_add(0x10)), 32);
                        edi = r;
                    } // 00e5d1bf
                    esi = edx.wrapping_add(0x10); // 00e5d1c3
                    {
                        let r = fl.sub(edi, 0x00000001u32, 32);
                        edi = r;
                    } // 00e5d1c6
                    eax = edx.wrapping_add(0x20); // 00e5d1c9
                    if fl.zf {
                        pc = 0x00e5d1e4;
                        continue;
                    } // 00e5d1cc
                    {
                        let r = fl.sub(edi, 0x00000001u32, 32);
                        edi = r;
                    } // 00e5d1ce
                    if fl.zf {
                        pc = 0x00e5d1e6;
                        continue;
                    } // 00e5d1d1
                    {
                        let r = fl.sub(edi, 0x00000001u32, 32);
                        edi = r;
                    } // 00e5d1d3
                    if fl.zf {
                        pc = 0x00e5d1e8;
                        continue;
                    } // 00e5d1d6
                    eax = e.mem.u32(esp.wrapping_add(0x80)); // 00e5d1d8
                    pc = 0x00e5d277; // 00e5d1df
                    continue;
                }
                0x00e5d1e4 => {
                    esi = edx; // 00e5d1e4
                    pc = 0x00e5d1e6;
                }
                0x00e5d1e6 => {
                    eax = edx; // 00e5d1e6
                    pc = 0x00e5d1e8;
                }
                0x00e5d1e8 => {
                    x[0] = [f32::from_bits(e.mem.u32(edx)), 0.0, 0.0, 0.0]; // 00e5d1e8
                    x[4] = [f32::from_bits(e.mem.u32(eax)), 0.0, 0.0, 0.0]; // 00e5d1ec
                    x[3] = [f32::from_bits(e.mem.u32(esi)), 0.0, 0.0, 0.0]; // 00e5d1f0
                    x[2][0] = x[0][0]; // 00e5d1f4
                    {
                        let r = fl.add(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5d1f8
                    x[3] = unpcklps(x[3], x[0]); // 00e5d1fb
                    x[2] = unpcklps(x[2], x[4]); // 00e5d1fe
                    x[2] = unpcklps(x[2], x[3]); // 00e5d201
                    store_vec(e, ecx.wrapping_sub(0x40), x[2]); // 00e5d204
                    x[0] = [
                        f32::from_bits(e.mem.u32(edx.wrapping_add(0x4))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d208
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_add(0x4))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d20d
                    x[3] = [
                        f32::from_bits(e.mem.u32(esi.wrapping_add(0x4))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d212
                    x[2][0] = x[0][0]; // 00e5d217
                    x[3] = unpcklps(x[3], x[0]); // 00e5d21b
                    x[2] = unpcklps(x[2], x[4]); // 00e5d21e
                    x[2] = unpcklps(x[2], x[3]); // 00e5d221
                    store_vec(e, ecx.wrapping_sub(0x30), x[2]); // 00e5d224
                    x[0] = [
                        f32::from_bits(e.mem.u32(edx.wrapping_add(0x8))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d228
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_add(0x8))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d22d
                    x[3] = [
                        f32::from_bits(e.mem.u32(esi.wrapping_add(0x8))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d232
                    x[2][0] = x[0][0]; // 00e5d237
                    x[3] = unpcklps(x[3], x[0]); // 00e5d23b
                    x[2] = unpcklps(x[2], x[4]); // 00e5d23e
                    x[2] = unpcklps(x[2], x[3]); // 00e5d241
                    store_vec(e, ecx.wrapping_sub(0x20), x[2]); // 00e5d244
                    x[0] = [
                        f32::from_bits(e.mem.u32(edx.wrapping_add(0xc))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d248
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_add(0xc))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d24d
                    x[3] = [
                        f32::from_bits(e.mem.u32(esi.wrapping_add(0xc))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d252
                    x[2][0] = x[0][0]; // 00e5d257
                    x[3] = unpcklps(x[3], x[0]); // 00e5d25b
                    x[2] = unpcklps(x[2], x[4]); // 00e5d25e
                    x[2] = unpcklps(x[2], x[3]); // 00e5d261
                    store_vec(e, ecx.wrapping_sub(0x10), x[2]); // 00e5d264
                    eax = e.mem.u32(esp.wrapping_add(0x80)); // 00e5d268
                    {
                        let r = fl.inc(eax, 32);
                        eax = r;
                    } // 00e5d26f
                    e.mem.set_u32(esp.wrapping_add(0x80), eax); // 00e5d270
                    pc = 0x00e5d277;
                }
                0x00e5d277 => {
                    {
                        let r = fl.shl(eax, 6, 32);
                        eax = r;
                    } // 00e5d277
                    {
                        let r = fl.sub(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5d27a
                    edx = esp.wrapping_add(0x90); // 00e5d27d
                    eax = esp.wrapping_add(eax).wrapping_add(0x250); // 00e5d284
                    fl.sub(ecx, edx, 32); // 00e5d28b
                    if fl.cf {
                        pc = 0x00e5d32d;
                        continue;
                    } // 00e5d28d
                    pc = 0x00e5d293;
                }
                0x00e5d293 => {
                    x[0] = load_vec(e, ecx); // 00e5d293
                    x[2] = load_vec(e, 0x01017d10u32); // 00e5d296
                    x[0] = andps(x[0], x[2]); // 00e5d29d
                    store_vec(e, eax, x[0]); // 00e5d2a0
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5d2a3
                    x[2] = load_vec(e, 0x01017d10u32); // 00e5d2a7
                    x[0] = andps(x[0], x[2]); // 00e5d2ae
                    store_vec(e, eax.wrapping_add(0x10), x[0]); // 00e5d2b1
                    x[2] = load_vec(e, ecx.wrapping_add(0x20)); // 00e5d2b5
                    x[0] = load_vec(e, 0x01017d10u32); // 00e5d2b9
                    x[0] = andps(x[0], x[2]); // 00e5d2c0
                    store_vec(e, eax.wrapping_add(0x20), x[0]); // 00e5d2c3
                    store_vec(e, eax.wrapping_add(0x30), x[1]); // 00e5d2c7
                    x[0] = load_vec(e, ebx.wrapping_add(0x10)); // 00e5d2cb
                    x[3] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5d2cf
                    x[4] = load_vec(e, ecx); // 00e5d2d3
                    x[2] = x[0]; // 00e5d2d6
                    x[2] = shufps(x[2], x[0], 0x55); // 00e5d2d9
                    x[2] = mulps(x[2], x[3]); // 00e5d2dd
                    x[3] = x[0]; // 00e5d2e0
                    x[3] = shufps(x[3], x[0], 0x0); // 00e5d2e3
                    x[3] = mulps(x[3], x[4]); // 00e5d2e7
                    x[2] = addps(x[2], x[3]); // 00e5d2ea
                    x[3] = x[0]; // 00e5d2ed
                    x[3] = shufps(x[3], x[0], 0xaa); // 00e5d2f0
                    x[0] = load_vec(e, ecx.wrapping_add(0x20)); // 00e5d2f4
                    x[3] = mulps(x[3], x[0]); // 00e5d2f8
                    x[0] = load_vec(e, ecx.wrapping_add(0x30)); // 00e5d2fb
                    x[2] = addps(x[2], x[3]); // 00e5d2ff
                    x[2] = addps(x[2], x[0]); // 00e5d302
                    store_vec(e, ecx.wrapping_add(0x30), x[2]); // 00e5d305
                    x[0] = load_vec(e, ecx.wrapping_add(0x30)); // 00e5d309
                    x[2] = load_vec(e, ecx.wrapping_add(0x30)); // 00e5d30d
                    x[0] = addps(x[0], x[2]); // 00e5d311
                    store_vec(e, ecx.wrapping_add(0x30), x[0]); // 00e5d314
                    {
                        let r = fl.sub(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5d318
                    edx = esp.wrapping_add(0x90); // 00e5d31b
                    {
                        let r = fl.sub(eax, 0x00000040u32, 32);
                        eax = r;
                    } // 00e5d322
                    fl.sub(ecx, edx, 32); // 00e5d325
                    if !fl.cf {
                        pc = 0x00e5d293;
                        continue;
                    } // 00e5d327
                    pc = 0x00e5d32d;
                }
                0x00e5d32d => {
                    eax = e.mem.u32(ebp.wrapping_add(0x14)); // 00e5d32d
                    edx = e.mem.u32(esp.wrapping_add(0x14)); // 00e5d330
                    ecx = esp.wrapping_add(0x30); // 00e5d334
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5d338
                    e.mem.set_u32(esp.wrapping_add(0x74), eax); // 00e5d339
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5d33d
                    eax = esp.wrapping_add(0x20); // 00e5d33e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5d342
                    ecx = esp.wrapping_add(0x5c); // 00e5d343
                    {
                        fn_00e5d7d0(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5d347
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d34c
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d34d
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d34e
                    esp = ebp; // 00e5d34f
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d351
                    return; // 00e5d352
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5d360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same query set-up as `00e5d070` for the interpreter `00e5e430`. Transcribed instruction by
/// instruction.
pub fn fn_00e5d360(e: &mut Engine, object: Ptr, list: Ptr, third: u32, fourth: u32) {
    e.with_stack(0x900, |e, frame| {
        let mut esp = frame.addr() + 0x800;
        e.mem.set_u32(esp.wrapping_add(4), object.addr());
        e.mem.set_u32(esp.wrapping_add(8), list.addr());
        e.mem.set_u32(esp.wrapping_add(12), third);
        e.mem.set_u32(esp.wrapping_add(16), fourth);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5d360;
        loop {
            match pc {
                0x00e5d360 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5d360
                    ebp = esp; // 00e5d361
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5d363
                    {
                        let r = fl.sub(esp, 0x00000484u32, 32);
                        esp = r;
                    } // 00e5d366
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e5d36c
                    ebx = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5d36d
                    fpu.fld_m32(e.mem.u32(ebx.wrapping_add(0x1c))); // 00e5d370
                    edx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5d373
                    fpu.fld1(); // 00e5d376
                    eax = e.mem.u32(ebx.wrapping_add(0x20)); // 00e5d378
                    fpu.fdivrp(1); // 00e5d37b
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d37d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5d380
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5d381
                    edi = e.mem.u32(edx); // 00e5d382
                    edx = e.mem.u32(edx.wrapping_add(0x4)); // 00e5d384
                    e.mem.set_u32(esp.wrapping_add(0x14), eax); // 00e5d387
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    } // 00e5d38b
                    esi = edi; // 00e5d38d
                    {
                        let r = fl.and(esi, 0xfffffffcu32, 32);
                        esi = r;
                    } // 00e5d38f
                    e.mem.set_u32(esp.wrapping_add(0x50), ecx); // 00e5d392
                    x[1] = xorps(x[1], x[1]); // 00e5d396
                    e.mem.set_u32(esp.wrapping_add(0x20), eax); // 00e5d399
                    e.mem.set_u32(esp.wrapping_add(0x24), eax); // 00e5d39d
                    e.mem.set_u32(esp.wrapping_add(0x28), eax); // 00e5d3a1
                    e.mem.set_u32(esp.wrapping_add(0x2c), 0x00000010u32); // 00e5d3a5
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e5d3ad
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax); // 00e5d3b1
                    store_vec(e, esp.wrapping_add(0x30), x[1]); // 00e5d3b5
                    ecx = esp.wrapping_add(0x90); // 00e5d3ba
                    e.mem.set_u32(esp.wrapping_add(0x80), eax); // 00e5d3c1
                    e.mem.set_u32(esp.wrapping_add(0x60), fpu.fstp_m32()); // 00e5d3c8
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x60))); // 00e5d3cc
                    fpu.fmul_m64(e.mem.u64(0x01091c00u32)); // 00e5d3d0
                    e.mem.set_u32(esp.wrapping_add(0x10), fpu.fstp_m32()); // 00e5d3d6
                    x[0] = [
                        f32::from_bits(e.mem.u32(esp.wrapping_add(0x10))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d3da
                    x[0] = shufps(x[0], x[0], 0x0); // 00e5d3e0
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5d3e4
                    e.mem.set_u32(esp.wrapping_add(0x10), eax); // 00e5d3e9
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e5d4af;
                        continue;
                    } // 00e5d3ed
                    {
                        let r = fl.dec(esi, 32);
                        esi = r;
                    } // 00e5d3f3
                    {
                        let r = fl.shr(esi, 2, 32);
                        esi = r;
                    } // 00e5d3f4
                    {
                        let r = fl.inc(esi, 32);
                        esi = r;
                    } // 00e5d3f7
                    ebx = esi.wrapping_mul(4); // 00e5d3f8
                    eax = edx.wrapping_add(0x24); // 00e5d3ff
                    e.mem.set_u32(esp.wrapping_add(0x10), ebx); // 00e5d402
                    pc = 0x00e5d410; // 00e5d406
                    continue;
                }
                0x00e5d410 => {
                    x[3] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x4))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d410
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_add(0xc))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d415
                    x[0] = [f32::from_bits(e.mem.u32(edx)), 0.0, 0.0, 0.0]; // 00e5d41a
                    x[2] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x14))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d41e
                    {
                        let r = fl.add(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5d423
                    {
                        let r = fl.add(edx, 0x00000040u32, 32);
                        edx = r;
                    } // 00e5d426
                    {
                        let r = fl.add(eax, 0x00000040u32, 32);
                        eax = r;
                    } // 00e5d429
                    x[0] = unpcklps(x[0], x[3]); // 00e5d42c
                    x[2] = unpcklps(x[2], x[4]); // 00e5d42f
                    x[0] = unpcklps(x[0], x[2]); // 00e5d432
                    store_vec(e, ecx.wrapping_sub(0x40), x[0]); // 00e5d435
                    x[3] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x40))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d439
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x30))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d43e
                    x[0] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x60))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d443
                    x[2] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x50))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d448
                    x[0] = unpcklps(x[0], x[3]); // 00e5d44d
                    x[2] = unpcklps(x[2], x[4]); // 00e5d450
                    x[0] = unpcklps(x[0], x[2]); // 00e5d453
                    store_vec(e, ecx.wrapping_sub(0x30), x[0]); // 00e5d456
                    x[3] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x3c))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d45a
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x2c))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d45f
                    x[0] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x5c))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d464
                    x[2] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x4c))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d469
                    x[0] = unpcklps(x[0], x[3]); // 00e5d46e
                    x[2] = unpcklps(x[2], x[4]); // 00e5d471
                    x[0] = unpcklps(x[0], x[2]); // 00e5d474
                    store_vec(e, ecx.wrapping_sub(0x20), x[0]); // 00e5d477
                    x[0] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x58))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d47b
                    x[2] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x48))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d480
                    x[3] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x38))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d485
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_sub(0x28))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d48a
                    x[0] = unpcklps(x[0], x[3]); // 00e5d48f
                    x[2] = unpcklps(x[2], x[4]); // 00e5d492
                    x[0] = unpcklps(x[0], x[2]); // 00e5d495
                    store_vec(e, ecx.wrapping_sub(0x10), x[0]); // 00e5d498
                    {
                        let r = fl.inc(e.mem.u32(esp.wrapping_add(0x80)), 32);
                        e.mem.set_u32(esp.wrapping_add(0x80), r);
                    } // 00e5d49c
                    {
                        let r = fl.sub(esi, 0x00000001u32, 32);
                        esi = r;
                    } // 00e5d4a3
                    if !fl.zf {
                        pc = 0x00e5d410;
                        continue;
                    } // 00e5d4a6
                    ebx = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5d4ac
                    pc = 0x00e5d4af;
                }
                0x00e5d4af => {
                    {
                        let r = fl.sub(edi, e.mem.u32(esp.wrapping_add(0x10)), 32);
                        edi = r;
                    } // 00e5d4af
                    esi = edx.wrapping_add(0x10); // 00e5d4b3
                    {
                        let r = fl.sub(edi, 0x00000001u32, 32);
                        edi = r;
                    } // 00e5d4b6
                    eax = edx.wrapping_add(0x20); // 00e5d4b9
                    if fl.zf {
                        pc = 0x00e5d4d4;
                        continue;
                    } // 00e5d4bc
                    {
                        let r = fl.sub(edi, 0x00000001u32, 32);
                        edi = r;
                    } // 00e5d4be
                    if fl.zf {
                        pc = 0x00e5d4d6;
                        continue;
                    } // 00e5d4c1
                    {
                        let r = fl.sub(edi, 0x00000001u32, 32);
                        edi = r;
                    } // 00e5d4c3
                    if fl.zf {
                        pc = 0x00e5d4d8;
                        continue;
                    } // 00e5d4c6
                    eax = e.mem.u32(esp.wrapping_add(0x80)); // 00e5d4c8
                    pc = 0x00e5d567; // 00e5d4cf
                    continue;
                }
                0x00e5d4d4 => {
                    esi = edx; // 00e5d4d4
                    pc = 0x00e5d4d6;
                }
                0x00e5d4d6 => {
                    eax = edx; // 00e5d4d6
                    pc = 0x00e5d4d8;
                }
                0x00e5d4d8 => {
                    x[0] = [f32::from_bits(e.mem.u32(edx)), 0.0, 0.0, 0.0]; // 00e5d4d8
                    x[4] = [f32::from_bits(e.mem.u32(eax)), 0.0, 0.0, 0.0]; // 00e5d4dc
                    x[3] = [f32::from_bits(e.mem.u32(esi)), 0.0, 0.0, 0.0]; // 00e5d4e0
                    x[2][0] = x[0][0]; // 00e5d4e4
                    {
                        let r = fl.add(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5d4e8
                    x[3] = unpcklps(x[3], x[0]); // 00e5d4eb
                    x[2] = unpcklps(x[2], x[4]); // 00e5d4ee
                    x[2] = unpcklps(x[2], x[3]); // 00e5d4f1
                    store_vec(e, ecx.wrapping_sub(0x40), x[2]); // 00e5d4f4
                    x[0] = [
                        f32::from_bits(e.mem.u32(edx.wrapping_add(0x4))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d4f8
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_add(0x4))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d4fd
                    x[3] = [
                        f32::from_bits(e.mem.u32(esi.wrapping_add(0x4))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d502
                    x[2][0] = x[0][0]; // 00e5d507
                    x[3] = unpcklps(x[3], x[0]); // 00e5d50b
                    x[2] = unpcklps(x[2], x[4]); // 00e5d50e
                    x[2] = unpcklps(x[2], x[3]); // 00e5d511
                    store_vec(e, ecx.wrapping_sub(0x30), x[2]); // 00e5d514
                    x[0] = [
                        f32::from_bits(e.mem.u32(edx.wrapping_add(0x8))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d518
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_add(0x8))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d51d
                    x[3] = [
                        f32::from_bits(e.mem.u32(esi.wrapping_add(0x8))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d522
                    x[2][0] = x[0][0]; // 00e5d527
                    x[3] = unpcklps(x[3], x[0]); // 00e5d52b
                    x[2] = unpcklps(x[2], x[4]); // 00e5d52e
                    x[2] = unpcklps(x[2], x[3]); // 00e5d531
                    store_vec(e, ecx.wrapping_sub(0x20), x[2]); // 00e5d534
                    x[0] = [
                        f32::from_bits(e.mem.u32(edx.wrapping_add(0xc))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d538
                    x[4] = [
                        f32::from_bits(e.mem.u32(eax.wrapping_add(0xc))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d53d
                    x[3] = [
                        f32::from_bits(e.mem.u32(esi.wrapping_add(0xc))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d542
                    x[2][0] = x[0][0]; // 00e5d547
                    x[3] = unpcklps(x[3], x[0]); // 00e5d54b
                    x[2] = unpcklps(x[2], x[4]); // 00e5d54e
                    x[2] = unpcklps(x[2], x[3]); // 00e5d551
                    store_vec(e, ecx.wrapping_sub(0x10), x[2]); // 00e5d554
                    eax = e.mem.u32(esp.wrapping_add(0x80)); // 00e5d558
                    {
                        let r = fl.inc(eax, 32);
                        eax = r;
                    } // 00e5d55f
                    e.mem.set_u32(esp.wrapping_add(0x80), eax); // 00e5d560
                    pc = 0x00e5d567;
                }
                0x00e5d567 => {
                    {
                        let r = fl.shl(eax, 6, 32);
                        eax = r;
                    } // 00e5d567
                    {
                        let r = fl.sub(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5d56a
                    edx = esp.wrapping_add(0x90); // 00e5d56d
                    eax = esp.wrapping_add(eax).wrapping_add(0x250); // 00e5d574
                    fl.sub(ecx, edx, 32); // 00e5d57b
                    if fl.cf {
                        pc = 0x00e5d61d;
                        continue;
                    } // 00e5d57d
                    pc = 0x00e5d583;
                }
                0x00e5d583 => {
                    x[0] = load_vec(e, ecx); // 00e5d583
                    x[2] = load_vec(e, 0x01017d10u32); // 00e5d586
                    x[0] = andps(x[0], x[2]); // 00e5d58d
                    store_vec(e, eax, x[0]); // 00e5d590
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5d593
                    x[2] = load_vec(e, 0x01017d10u32); // 00e5d597
                    x[0] = andps(x[0], x[2]); // 00e5d59e
                    store_vec(e, eax.wrapping_add(0x10), x[0]); // 00e5d5a1
                    x[2] = load_vec(e, ecx.wrapping_add(0x20)); // 00e5d5a5
                    x[0] = load_vec(e, 0x01017d10u32); // 00e5d5a9
                    x[0] = andps(x[0], x[2]); // 00e5d5b0
                    store_vec(e, eax.wrapping_add(0x20), x[0]); // 00e5d5b3
                    store_vec(e, eax.wrapping_add(0x30), x[1]); // 00e5d5b7
                    x[0] = load_vec(e, ebx.wrapping_add(0x10)); // 00e5d5bb
                    x[3] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5d5bf
                    x[4] = load_vec(e, ecx); // 00e5d5c3
                    x[2] = x[0]; // 00e5d5c6
                    x[2] = shufps(x[2], x[0], 0x55); // 00e5d5c9
                    x[2] = mulps(x[2], x[3]); // 00e5d5cd
                    x[3] = x[0]; // 00e5d5d0
                    x[3] = shufps(x[3], x[0], 0x0); // 00e5d5d3
                    x[3] = mulps(x[3], x[4]); // 00e5d5d7
                    x[2] = addps(x[2], x[3]); // 00e5d5da
                    x[3] = x[0]; // 00e5d5dd
                    x[3] = shufps(x[3], x[0], 0xaa); // 00e5d5e0
                    x[0] = load_vec(e, ecx.wrapping_add(0x20)); // 00e5d5e4
                    x[3] = mulps(x[3], x[0]); // 00e5d5e8
                    x[0] = load_vec(e, ecx.wrapping_add(0x30)); // 00e5d5eb
                    x[2] = addps(x[2], x[3]); // 00e5d5ef
                    x[2] = addps(x[2], x[0]); // 00e5d5f2
                    store_vec(e, ecx.wrapping_add(0x30), x[2]); // 00e5d5f5
                    x[0] = load_vec(e, ecx.wrapping_add(0x30)); // 00e5d5f9
                    x[2] = load_vec(e, ecx.wrapping_add(0x30)); // 00e5d5fd
                    x[0] = addps(x[0], x[2]); // 00e5d601
                    store_vec(e, ecx.wrapping_add(0x30), x[0]); // 00e5d604
                    {
                        let r = fl.sub(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5d608
                    edx = esp.wrapping_add(0x90); // 00e5d60b
                    {
                        let r = fl.sub(eax, 0x00000040u32, 32);
                        eax = r;
                    } // 00e5d612
                    fl.sub(ecx, edx, 32); // 00e5d615
                    if !fl.cf {
                        pc = 0x00e5d583;
                        continue;
                    } // 00e5d617
                    pc = 0x00e5d61d;
                }
                0x00e5d61d => {
                    eax = e.mem.u32(ebp.wrapping_add(0x14)); // 00e5d61d
                    edx = e.mem.u32(esp.wrapping_add(0x14)); // 00e5d620
                    ecx = esp.wrapping_add(0x30); // 00e5d624
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5d628
                    e.mem.set_u32(esp.wrapping_add(0x74), eax); // 00e5d629
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5d62d
                    eax = esp.wrapping_add(0x20); // 00e5d62e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5d632
                    ecx = esp.wrapping_add(0x5c); // 00e5d633
                    {
                        fn_00e5e430(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5d637
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d63c
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d63d
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d63e
                    esp = ebp; // 00e5d63f
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d641
                    return; // 00e5d642
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5d650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Query set-up for a single point (cdecl: `object`, `point` (four floats), `third`, `fourth`):
/// the scaled reciprocal of the float at `object + 0x1c` is built as in `00e5d070`, the point minus
/// the vector at `object + 0x10` is doubled, and `00e5f070` is called with the word at
/// `object + 0x20`. Transcribed instruction by instruction.
pub fn fn_00e5d650(e: &mut Engine, object: Ptr, point: Ptr, third: u32, fourth: u32) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), object.addr());
        e.mem.set_u32(esp.wrapping_add(8), point.addr());
        e.mem.set_u32(esp.wrapping_add(12), third);
        e.mem.set_u32(esp.wrapping_add(16), fourth);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5d650;
        loop {
            match pc {
                0x00e5d650 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5d650
                    ebp = esp; // 00e5d651
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5d653
                    {
                        let r = fl.sub(esp, 0x00000080u32, 32);
                        esp = r;
                    } // 00e5d656
                    eax = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5d65c
                    fpu.fld_m32(e.mem.u32(eax.wrapping_add(0x1c))); // 00e5d65f
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d662
                    fpu.fld1(); // 00e5d665
                    edx = e.mem.u32(eax.wrapping_add(0x20)); // 00e5d667
                    fpu.fdivrp(1); // 00e5d66a
                    x[1] = load_vec(e, eax.wrapping_add(0x10)); // 00e5d66c
                    eax = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5d670
                    x[2] = load_vec(e, esp.wrapping_add(0x70)); // 00e5d673
                    x[0] = xorps(x[0], x[0]); // 00e5d678
                    store_vec(e, esp.wrapping_add(0x20), x[0]); // 00e5d67b
                    e.mem.set_u32(esp.wrapping_add(0x40), ecx); // 00e5d680
                    {
                        let r = fl.xor(ecx, ecx, 32);
                        ecx = r;
                    } // 00e5d684
                    e.mem.set_u32(esp.wrapping_add(0x10), ecx); // 00e5d686
                    e.mem.set_u32(esp.wrapping_add(0x14), ecx); // 00e5d68a
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e5d68e
                    e.mem.set_u32(esp.wrapping_add(0x8), ecx); // 00e5d692
                    e.mem.set_u32(esp.wrapping_add(0xc), ecx); // 00e5d696
                    ecx = esp.wrapping_add(0x20); // 00e5d69a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5d69e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5d69f
                    edx = esp.wrapping_add(0x10); // 00e5d6a0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5d6a4
                    ecx = esp.wrapping_add(0x4c); // 00e5d6a5
                    e.mem.set_u32(esp.wrapping_add(0x28), 0x00000010u32); // 00e5d6a9
                    e.mem.set_u32(esp.wrapping_add(0x5c), fpu.fstp_m32()); // 00e5d6b1
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x5c))); // 00e5d6b5
                    fpu.fmul_m64(e.mem.u64(0x01091c00u32)); // 00e5d6b9
                    e.mem.set_u32(esp.wrapping_add(0x10), fpu.fstp_m32()); // 00e5d6bf
                    x[0] = [
                        f32::from_bits(e.mem.u32(esp.wrapping_add(0x10))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d6c3
                    fpu.fld_m32(e.mem.u32(eax.wrapping_add(0xc))); // 00e5d6c9
                    x[0] = shufps(x[0], x[0], 0x0); // 00e5d6cc
                    fpu.fadd(0, 0); // 00e5d6d0
                    store_vec(e, esp.wrapping_add(0x3c), x[0]); // 00e5d6d2
                    x[0] = load_vec(e, eax); // 00e5d6d7
                    eax = e.mem.u32(ebp.wrapping_add(0x14)); // 00e5d6da
                    x[0] = subps(x[0], x[1]); // 00e5d6dd
                    x[0] = addps(x[0], x[0]); // 00e5d6e0
                    x[1] = x[0]; // 00e5d6e3
                    x[0] = unpckhps(x[0], x[2]); // 00e5d6e6
                    x[1] = shufps(x[1], x[0], 0xc4); // 00e5d6e9
                    store_vec(e, esp.wrapping_add(0x7c), x[1]); // 00e5d6ed
                    e.mem.set_u32(esp.wrapping_add(0x88), fpu.fstp_m32()); // 00e5d6f2
                    e.mem.set_u32(esp.wrapping_add(0x6c), eax); // 00e5d6f9
                    {
                        fn_00e5f070(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5d6fd
                    esp = ebp; // 00e5d702
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d704
                    return; // 00e5d705
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5d710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same single-point query set-up as `00e5d650` for the interpreter `00e5fb50`. Transcribed
/// instruction by instruction.
pub fn fn_00e5d710(e: &mut Engine, object: Ptr, point: Ptr, third: u32, fourth: u32) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), object.addr());
        e.mem.set_u32(esp.wrapping_add(8), point.addr());
        e.mem.set_u32(esp.wrapping_add(12), third);
        e.mem.set_u32(esp.wrapping_add(16), fourth);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5d710;
        loop {
            match pc {
                0x00e5d710 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5d710
                    ebp = esp; // 00e5d711
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5d713
                    {
                        let r = fl.sub(esp, 0x00000080u32, 32);
                        esp = r;
                    } // 00e5d716
                    eax = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5d71c
                    fpu.fld_m32(e.mem.u32(eax.wrapping_add(0x1c))); // 00e5d71f
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d722
                    fpu.fld1(); // 00e5d725
                    edx = e.mem.u32(eax.wrapping_add(0x20)); // 00e5d727
                    fpu.fdivrp(1); // 00e5d72a
                    x[1] = load_vec(e, eax.wrapping_add(0x10)); // 00e5d72c
                    eax = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5d730
                    x[2] = load_vec(e, esp.wrapping_add(0x70)); // 00e5d733
                    x[0] = xorps(x[0], x[0]); // 00e5d738
                    store_vec(e, esp.wrapping_add(0x20), x[0]); // 00e5d73b
                    e.mem.set_u32(esp.wrapping_add(0x40), ecx); // 00e5d740
                    {
                        let r = fl.xor(ecx, ecx, 32);
                        ecx = r;
                    } // 00e5d744
                    e.mem.set_u32(esp.wrapping_add(0x10), ecx); // 00e5d746
                    e.mem.set_u32(esp.wrapping_add(0x14), ecx); // 00e5d74a
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e5d74e
                    e.mem.set_u32(esp.wrapping_add(0x8), ecx); // 00e5d752
                    e.mem.set_u32(esp.wrapping_add(0xc), ecx); // 00e5d756
                    ecx = esp.wrapping_add(0x20); // 00e5d75a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5d75e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5d75f
                    edx = esp.wrapping_add(0x10); // 00e5d760
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5d764
                    ecx = esp.wrapping_add(0x4c); // 00e5d765
                    e.mem.set_u32(esp.wrapping_add(0x28), 0x00000010u32); // 00e5d769
                    e.mem.set_u32(esp.wrapping_add(0x5c), fpu.fstp_m32()); // 00e5d771
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x5c))); // 00e5d775
                    fpu.fmul_m64(e.mem.u64(0x01091c00u32)); // 00e5d779
                    e.mem.set_u32(esp.wrapping_add(0x10), fpu.fstp_m32()); // 00e5d77f
                    x[0] = [
                        f32::from_bits(e.mem.u32(esp.wrapping_add(0x10))),
                        0.0,
                        0.0,
                        0.0,
                    ]; // 00e5d783
                    fpu.fld_m32(e.mem.u32(eax.wrapping_add(0xc))); // 00e5d789
                    x[0] = shufps(x[0], x[0], 0x0); // 00e5d78c
                    fpu.fadd(0, 0); // 00e5d790
                    store_vec(e, esp.wrapping_add(0x3c), x[0]); // 00e5d792
                    x[0] = load_vec(e, eax); // 00e5d797
                    eax = e.mem.u32(ebp.wrapping_add(0x14)); // 00e5d79a
                    x[0] = subps(x[0], x[1]); // 00e5d79d
                    x[0] = addps(x[0], x[0]); // 00e5d7a0
                    x[1] = x[0]; // 00e5d7a3
                    x[0] = unpckhps(x[0], x[2]); // 00e5d7a6
                    x[1] = shufps(x[1], x[0], 0xc4); // 00e5d7a9
                    store_vec(e, esp.wrapping_add(0x7c), x[1]); // 00e5d7ad
                    e.mem.set_u32(esp.wrapping_add(0x88), fpu.fstp_m32()); // 00e5d7b2
                    e.mem.set_u32(esp.wrapping_add(0x6c), eax); // 00e5d7b9
                    {
                        fn_00e5fb50(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5d7bd
                    esp = ebp; // 00e5d7c2
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5d7c4
                    return; // 00e5d7c5
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5d7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// MOPP code interpreter (thiscall, `RET 0xc`: `this` the machine, `sink`, `code` (the address of the
/// current code byte) and `bounds` (the 32 bytes of query data)): each step dispatches on the code
/// byte (values up to 0x6b through a byte map and a 23-entry jump table written as `match`es
/// on the code byte and the case), tests the query against the node's split planes with SSE (`MULPS`, `ADDPS`,
/// `SHUFPS`, `CMPLTPS`, `MOVMSKPS`), and either descends by calling itself with the offset code
/// pointer, walks on, or reports hits through the machine's virtual methods (`00c85750`, `00c90940`
/// array growth, `00cb1600`, `00cc00e0`, `00cc01c0` and the `00e65ee0`-family callees). An opcode
/// the table does not know ends in an `INT3`, which stops here with a panic. Transcribed
/// instruction by instruction. The four variants of this interpreter differ in the data layout of
/// the query.
pub fn fn_00e5d7d0(e: &mut Engine, this: Ptr, sink: Ptr, code: Ptr, bounds: Ptr) {
    e.with_stack(0x500, |e, frame| {
        let mut esp = frame.addr() + 0x400;
        e.mem.set_u32(esp.wrapping_add(4), sink.addr());
        e.mem.set_u32(esp.wrapping_add(8), code.addr());
        e.mem.set_u32(esp.wrapping_add(12), bounds.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5d7d0;
        loop {
            match pc {
                0x00e5d7d0 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5d7d0
                    ebp = esp; // 00e5d7d1
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5d7d3
                    {
                        let r = fl.sub(esp, 0x000002c4u32, 32);
                        esp = r;
                    } // 00e5d7d6
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e5d7dc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5d7dd
                    esi = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5d7de
                    ebx = ecx; // 00e5d7e1
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5d7e3
                    edi = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5d7e4
                    e.mem.set_u32(esp.wrapping_add(0x18), ebx); // 00e5d7e7
                    pc = 0x00e5d7f0; // 00e5d7eb
                    continue;
                }
                0x00e5d7f0 => {
                    eax = u32::from(e.mem.u8(esi)); // 00e5d7f0
                    fl.sub(eax, 0x0000006bu32, 32); // 00e5d7f3
                    if !fl.cf && !fl.zf {
                        pc = 0x00e5e261;
                        continue;
                    } // 00e5d7f6
                    ecx = match eax {
                        0 => 0,
                        1..=4 => 1,
                        5 => 2,
                        6 => 3,
                        7 => 4,
                        8 | 12..=15 | 29..=31 | 44..=47 | 84..=95 => 22,
                        9 => 5,
                        10 => 6,
                        11 => 7,
                        16..=18 => 8,
                        19..=28 => 9,
                        32..=34 => 10,
                        35..=37 => 11,
                        38..=40 => 12,
                        41..=43 => 13,
                        48..=79 => 14,
                        80 => 15,
                        81 => 16,
                        82 => 17,
                        83 => 18,
                        96..=99 => 19,
                        100..=103 => 20,
                        104..=107 => 21,
                        _ => unreachable!(),
                    }; // 00e5d7fc
                    pc = match ecx {
                        0 => 0x00e5_e352,
                        1 => 0x00e5_e088,
                        2 => 0x00e5_e046,
                        3 => 0x00e5_e053,
                        4 => 0x00e5_e069,
                        5 => 0x00e5_e0da,
                        6 => 0x00e5_e11f,
                        7 => 0x00e5_e16d,
                        8 => 0x00e5_d840,
                        9 => 0x00e5_d80a,
                        10 => 0x00e5_d8ac,
                        11 => 0x00e5_d90d,
                        12 => 0x00e5_debb,
                        13 => 0x00e5_de60,
                        14 => 0x00e5_e315,
                        15 => 0x00e5_e2c7,
                        16 => 0x00e5_e2cd,
                        17 => 0x00e5_e2dc,
                        18 => 0x00e5_e2f4,
                        19 => 0x00e5_e1cd,
                        20 => 0x00e5_e1dd,
                        21 => 0x00e5_e1f6,
                        22 => 0x00e5_e261,
                        _ => unreachable!(),
                    }; // 00e5d803
                    continue;
                }
                0x00e5d80a => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5d80a
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d80e
                    x[0] = load_vec(e, eax); // 00e5d811
                    store_vec(e, esp.wrapping_add(0x70), x[0]); // 00e5d814
                    x[0] = load_vec(e, eax.wrapping_add(0x10)); // 00e5d819
                    eax = esp.wrapping_add(0x70); // 00e5d81d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5d821
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5d822
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5d825
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5d826
                    ecx = ebx; // 00e5d827
                    e.mem.set_u32(esp.wrapping_add(0x20), edx); // 00e5d829
                    store_vec(e, esp.wrapping_add(0x8c), x[0]); // 00e5d82d
                    {
                        fn_00e5d7d0(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5d835
                    {
                        let r = fl.add(esi, e.mem.u32(esp.wrapping_add(0x14)), 32);
                        esi = r;
                    } // 00e5d83a
                    pc = 0x00e5d7f0; // 00e5d83e
                    continue;
                }
                0x00e5d840 => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d840
                    x[0] = load_vec(e, ecx); // 00e5d843
                    store_vec(e, esp.wrapping_add(0x70), x[0]); // 00e5d846
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5d84b
                    store_vec(e, esp.wrapping_add(0x80), x[0]); // 00e5d84f
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5d857
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5d85b
                    e.mem.set_u32(esp.wrapping_add(0x14), ecx); // 00e5d85f
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5d863
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5d866
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d868
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x38)),
                            32,
                        );
                        edx = r;
                    } // 00e5d86b
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5d86f
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5d873
                    edx = e.mem.u32(esp.wrapping_add(0x14)); // 00e5d877
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5d87b
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x30),
                        fpu.fstp_m32(),
                    ); // 00e5d87e
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5d882
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5d885
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x38)),
                            32,
                        );
                        edx = r;
                    } // 00e5d887
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5d88b
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5d88f
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5d893
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e5d896
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5d898
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_add(0x30),
                        fpu.fstp_m32(),
                    ); // 00e5d89b
                    edx = u32::from(e.mem.u8(esi.wrapping_sub(0x1))); // 00e5d89f
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5d8a3
                    pc = 0x00e5d990; // 00e5d8a7
                    continue;
                }
                0x00e5d8ac => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d8ac
                    x[0] = load_vec(e, ecx); // 00e5d8af
                    store_vec(e, esp.wrapping_add(0x70), x[0]); // 00e5d8b2
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5d8b7
                    store_vec(e, esp.wrapping_add(0x80), x[0]); // 00e5d8bb
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5d8c3
                    ecx = edx.wrapping_add(0x1); // 00e5d8c7
                    e.mem.set_u32(esp.wrapping_add(0x14), ecx); // 00e5d8ca
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5d8ce
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5d8d1
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x78)),
                            32,
                        );
                        edx = r;
                    } // 00e5d8d3
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5d8d7
                    edx = e.mem.u32(esp.wrapping_add(0x14)); // 00e5d8db
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5d8df
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5d8e3
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5d8e5
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x78)),
                            32,
                        );
                        edx = r;
                    } // 00e5d8e8
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x10),
                        fpu.fstp_m32(),
                    ); // 00e5d8ec
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5d8f0
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5d8f4
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d8f8
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5d8fb
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e5d8fe
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5d900
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x70),
                        fpu.fstp_m32(),
                    ); // 00e5d903
                    eax = u32::from(e.mem.u8(esi.wrapping_sub(0x1))); // 00e5d907
                    pc = 0x00e5d98a; // 00e5d90b
                    continue;
                }
                0x00e5d90d => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d90d
                    x[0] = load_vec(e, ecx); // 00e5d910
                    store_vec(e, esp.wrapping_add(0x70), x[0]); // 00e5d913
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5d918
                    store_vec(e, esp.wrapping_add(0x80), x[0]); // 00e5d91c
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5d924
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5d928
                    e.mem.set_u32(esp.wrapping_add(0x14), ecx); // 00e5d92c
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5d930
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5d933
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d935
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x84)),
                            32,
                        );
                        edx = r;
                    } // 00e5d938
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5d93f
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5d943
                    edx = e.mem.u32(esp.wrapping_add(0x14)); // 00e5d947
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5d94b
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x7c),
                        fpu.fstp_m32(),
                    ); // 00e5d94e
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5d952
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5d955
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x84)),
                            32,
                        );
                        edx = r;
                    } // 00e5d957
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5d95e
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5d962
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5d966
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x1c),
                        fpu.fstp_m32(),
                    ); // 00e5d969
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5d96d
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x5))); // 00e5d971
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5d975
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x6))); // 00e5d979
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5d97d
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5d980
                    {
                        let r = fl.add(ebx, edx, 32);
                        ebx = r;
                    } // 00e5d983
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5d985
                    {
                        let r = fl.add(esi, 0x00000007u32, 32);
                        esi = r;
                    } // 00e5d987
                    pc = 0x00e5d98a;
                }
                0x00e5d98a => {
                    e.mem.set_u32(esp.wrapping_add(0x14), eax); // 00e5d98a
                    edx = eax; // 00e5d98e
                    pc = 0x00e5d990;
                }
                0x00e5d990 => {
                    eax = u32::from(e.mem.u8(ebx.wrapping_add(esi))); // 00e5d990
                    fl.sub(eax, 0x00000026u32, 32); // 00e5d994
                    if fl.sf != fl.of {
                        pc = 0x00e5da7f;
                        continue;
                    } // 00e5d997
                      // no change (a move of a register onto itself); // 00e5d99d
                    pc = 0x00e5d9a0;
                }
                0x00e5d9a0 => {
                    fl.sub(eax, 0x0000002bu32, 32); // 00e5d9a0
                    if !fl.zf && fl.sf == fl.of {
                        pc = 0x00e5da7f;
                        continue;
                    } // 00e5d9a3
                    fl.sub(eax, 0x00000029u32, 32); // 00e5d9a9
                    if fl.sf != fl.of {
                        pc = 0x00e5da17;
                        continue;
                    } // 00e5d9ac
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x1))); // 00e5d9ae
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x2))); // 00e5d9b3
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5d9b8
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5d9bb
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x3))); // 00e5d9bd
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5d9c2
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5d9c5
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d9c7
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5d9ca
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5d9ce
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5d9d2
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5d9d6
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0xa4),
                        fpu.fstp_m32(),
                    ); // 00e5d9d9
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x4))); // 00e5d9e0
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x5))); // 00e5d9e5
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5d9ea
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5d9ed
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x6))); // 00e5d9ef
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5d9f4
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5d9f7
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5d9f9
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5d9fc
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5da00
                    {
                        let r = fl.add(ebx, 0x00000007u32, 32);
                        ebx = r;
                    } // 00e5da04
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5da07
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5da0b
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x94),
                        fpu.fstp_m32(),
                    ); // 00e5da0e
                    pc = 0x00e5da6e; // 00e5da15
                    continue;
                }
                0x00e5da17 => {
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x1))); // 00e5da17
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5da1c
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5da1f
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5da21
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5da25
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5da2c
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5da30
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5da34
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5da37
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x98),
                        fpu.fstp_m32(),
                    ); // 00e5da3a
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x2))); // 00e5da41
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5da46
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5da49
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5da4b
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5da4f
                    {
                        let r = fl.add(ebx, 0x00000003u32, 32);
                        ebx = r;
                    } // 00e5da56
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5da59
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5da5d
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5da61
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5da64
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x88),
                        fpu.fstp_m32(),
                    ); // 00e5da67
                    pc = 0x00e5da6e;
                }
                0x00e5da6e => {
                    eax = u32::from(e.mem.u8(ebx.wrapping_add(esi))); // 00e5da6e
                    fl.sub(eax, 0x00000026u32, 32); // 00e5da72
                    edx = e.mem.u32(esp.wrapping_add(0x14)); // 00e5da75
                    if fl.sf == fl.of {
                        pc = 0x00e5d9a0;
                        continue;
                    } // 00e5da79
                    pc = 0x00e5da7f;
                }
                0x00e5da7f => {
                    eax = u32::from(e.mem.u8(edx.wrapping_add(esi))); // 00e5da7f
                    fl.sub(eax, 0x00000026u32, 32); // 00e5da83
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax); // 00e5da86
                    if fl.sf != fl.of {
                        pc = 0x00e5db66;
                        continue;
                    } // 00e5da8a
                    pc = 0x00e5da90;
                }
                0x00e5da90 => {
                    fl.sub(eax, 0x0000002bu32, 32); // 00e5da90
                    if !fl.zf && fl.sf == fl.of {
                        pc = 0x00e5db62;
                        continue;
                    } // 00e5da93
                    fl.sub(eax, 0x00000029u32, 32); // 00e5da99
                    eax = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x1))); // 00e5da9c
                    if fl.sf != fl.of {
                        pc = 0x00e5db03;
                        continue;
                    } // 00e5daa1
                    ecx = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x2))); // 00e5daa3
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5daa8
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5daab
                    ecx = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x3))); // 00e5daad
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5dab2
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5dab5
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5dab7
                    e.mem.set_u32(esp.wrapping_add(0x10), eax); // 00e5dabb
                    eax = e.mem.u32(esp.wrapping_add(0x18)); // 00e5dabf
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5dac3
                    fpu.fmul_m32(e.mem.u32(eax.wrapping_add(0x10))); // 00e5dac7
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x34),
                        fpu.fstp_m32(),
                    ); // 00e5daca
                    ecx = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x4))); // 00e5dace
                    eax = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x5))); // 00e5dad3
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5dad8
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5dadb
                    ecx = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x6))); // 00e5dadd
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5dae2
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5dae5
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5dae7
                    e.mem.set_u32(esp.wrapping_add(0x10), eax); // 00e5daeb
                    eax = e.mem.u32(esp.wrapping_add(0x18)); // 00e5daef
                    {
                        let r = fl.add(edx, 0x00000007u32, 32);
                        edx = r;
                    } // 00e5daf3
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5daf6
                    fpu.fmul_m32(e.mem.u32(eax.wrapping_add(0x10))); // 00e5dafa
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x24),
                        fpu.fstp_m32(),
                    ); // 00e5dafd
                    pc = 0x00e5db51; // 00e5db01
                    continue;
                }
                0x00e5db03 => {
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5db03
                    {
                        let r = fl.shl(eax, ((ecx & 0xff) & 0x1f), 32);
                        eax = r;
                    } // 00e5db06
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5db08
                    {
                        let r = fl.add(
                            eax,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        eax = r;
                    } // 00e5db0c
                    e.mem.set_u32(esp.wrapping_add(0x10), eax); // 00e5db13
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5db17
                    eax = e.mem.u32(esp.wrapping_add(0x18)); // 00e5db1b
                    fpu.fmul_m32(e.mem.u32(eax.wrapping_add(0x10))); // 00e5db1f
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x28),
                        fpu.fstp_m32(),
                    ); // 00e5db22
                    eax = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x2))); // 00e5db26
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5db2b
                    {
                        let r = fl.shl(eax, ((ecx & 0xff) & 0x1f), 32);
                        eax = r;
                    } // 00e5db2e
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5db30
                    {
                        let r = fl.add(
                            eax,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        eax = r;
                    } // 00e5db34
                    {
                        let r = fl.add(edx, 0x00000003u32, 32);
                        edx = r;
                    } // 00e5db3b
                    e.mem.set_u32(esp.wrapping_add(0x10), eax); // 00e5db3e
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5db42
                    eax = e.mem.u32(esp.wrapping_add(0x18)); // 00e5db46
                    fpu.fmul_m32(e.mem.u32(eax.wrapping_add(0x10))); // 00e5db4a
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x18),
                        fpu.fstp_m32(),
                    ); // 00e5db4d
                    pc = 0x00e5db51;
                }
                0x00e5db51 => {
                    eax = u32::from(e.mem.u8(edx.wrapping_add(esi))); // 00e5db51
                    fl.sub(eax, 0x00000026u32, 32); // 00e5db55
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax); // 00e5db58
                    if fl.sf == fl.of {
                        pc = 0x00e5da90;
                        continue;
                    } // 00e5db5c
                    pc = 0x00e5db62;
                }
                0x00e5db62 => {
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5db62
                    pc = 0x00e5db66;
                }
                0x00e5db66 => {
                    eax = e.mem.u32(esp.wrapping_add(0x18)); // 00e5db66
                    x[1] = load_vec(e, esp.wrapping_add(0x80)); // 00e5db6a
                    x[2] = load_vec(e, esp.wrapping_add(0x70)); // 00e5db72
                    {
                        let r = fl.add(eax, 0x00000020u32, 32);
                        eax = r;
                    } // 00e5db77
                    ecx = eax.wrapping_add(0x20); // 00e5db7a
                    eax = e.mem.u32(eax.wrapping_add(0x10)); // 00e5db7d
                    x[0] = x[1]; // 00e5db80
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e5db83
                    e.mem.set_u32(esp.wrapping_add(0x3c), 0x0000000fu32); // 00e5db86
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5db8e
                    x[0] = addps(x[0], x[2]); // 00e5db92
                    x[1] = subps(x[1], x[2]); // 00e5db95
                    e.mem.set_u32(esp.wrapping_add(0x40), eax); // 00e5db98
                    if fl.sf {
                        pc = 0x00e5dc8c;
                        continue;
                    } // 00e5db9c
                    x[4] = x[0]; // 00e5dba2
                    x[5] = x[0]; // 00e5dba5
                    x[4] = shufps(x[4], x[0], 0x55); // 00e5dba8
                    x[5] = shufps(x[5], x[0], 0x0); // 00e5dbac
                    x[2] = x[1]; // 00e5dbb0
                    x[3] = x[1]; // 00e5dbb3
                    x[0] = shufps(x[0], x[0], 0xaa); // 00e5dbb6
                    x[2] = shufps(x[2], x[1], 0x55); // 00e5dbba
                    x[3] = shufps(x[3], x[1], 0x0); // 00e5dbbe
                    store_vec(e, esp.wrapping_add(0xc0), x[0]); // 00e5dbc2
                    eax = ecx.wrapping_add(0x220); // 00e5dbca
                    x[1] = shufps(x[1], x[1], 0xaa); // 00e5dbd0
                    x[0] = xorps(x[0], x[0]); // 00e5dbd4
                    store_vec(e, esp.wrapping_add(0xb0), x[1]); // 00e5dbd7
                    store_vec(e, esp.wrapping_add(0xa0), x[0]); // 00e5dbdf
                    e.mem.set_u32(esp.wrapping_add(0x20), eax); // 00e5dbe7
                    pc = 0x00e5dbf4; // 00e5dbeb
                    continue;
                }
                0x00e5dbf0 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5dbf0
                    pc = 0x00e5dbf4;
                }
                0x00e5dbf4 => {
                    x[1] = load_vec(e, eax.wrapping_sub(0x20)); // 00e5dbf4
                    x[0] = load_vec(e, eax.wrapping_sub(0x10)); // 00e5dbf8
                    x[6] = load_vec(e, ecx); // 00e5dbfc
                    x[7] = load_vec(e, eax.wrapping_sub(0x1f0)); // 00e5dbff
                    x[0] = mulps(x[0], x[2]); // 00e5dc06
                    x[1] = mulps(x[1], x[3]); // 00e5dc09
                    x[1] = addps(x[1], x[0]); // 00e5dc0c
                    x[0] = load_vec(e, eax); // 00e5dc0f
                    x[0] = mulps(x[0], load_vec(e, esp.wrapping_add(0xb0))); // 00e5dc12
                    x[1] = addps(x[1], x[0]); // 00e5dc1a
                    x[0] = load_vec(e, eax.wrapping_sub(0x210)); // 00e5dc1d
                    x[6] = mulps(x[6], x[5]); // 00e5dc24
                    x[0] = mulps(x[0], x[4]); // 00e5dc27
                    x[0] = addps(x[0], x[6]); // 00e5dc2a
                    x[6] = load_vec(e, eax.wrapping_sub(0x200)); // 00e5dc2d
                    x[6] = mulps(x[6], load_vec(e, esp.wrapping_add(0xc0))); // 00e5dc34
                    x[6] = addps(x[6], x[7]); // 00e5dc3c
                    x[0] = addps(x[0], x[6]); // 00e5dc3f
                    x[6] = load_vec(e, esp.wrapping_add(0xa0)); // 00e5dc42
                    x[7] = x[0]; // 00e5dc4a
                    x[7] = subps(x[7], x[1]); // 00e5dc4d
                    x[7] = cmpltps(x[7], x[6]); // 00e5dc50
                    eax = movmskps(x[7]); // 00e5dc54
                    fl.sub(eax, 0x0000000fu32, 32); // 00e5dc57
                    if !fl.zf {
                        pc = 0x00e5dcaa;
                        continue;
                    } // 00e5dc5a
                    eax = e.mem.u32(esp.wrapping_add(0x20)); // 00e5dc5c
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x1c)), 0x00000040u32, 32);
                        e.mem.set_u32(esp.wrapping_add(0x1c), r);
                    } // 00e5dc60
                    x[0] = addps(x[0], x[1]); // 00e5dc65
                    x[0] = cmpltps(x[0], x[6]); // 00e5dc68
                    ecx = movmskps(x[0]); // 00e5dc6c
                    {
                        let r = fl.and(e.mem.u32(esp.wrapping_add(0x3c)), ecx, 32);
                        e.mem.set_u32(esp.wrapping_add(0x3c), r);
                    } // 00e5dc6f
                    {
                        let r = fl.add(eax, 0x00000040u32, 32);
                        eax = r;
                    } // 00e5dc73
                    {
                        let r = fl.sub(e.mem.u32(esp.wrapping_add(0x40)), 0x00000001u32, 32);
                        e.mem.set_u32(esp.wrapping_add(0x40), r);
                    } // 00e5dc76
                    e.mem.set_u32(esp.wrapping_add(0x20), eax); // 00e5dc7b
                    if !fl.sf {
                        pc = 0x00e5dbf0;
                        continue;
                    } // 00e5dc7f
                    fl.sub(e.mem.u32(esp.wrapping_add(0x3c)), 0x0000000fu32, 32); // 00e5dc85
                    if !fl.zf {
                        pc = 0x00e5dcb4;
                        continue;
                    } // 00e5dc8a
                    pc = 0x00e5dc8c;
                }
                0x00e5dc8c => {
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5dc8c
                    {
                        let r = fl.add(edx, esi, 32);
                        edx = r;
                    } // 00e5dc90
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5dc92
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5dc93
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5dc94
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e65ee0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5dc97
                    edx = e.mem.u32(esp.wrapping_add(0x14)); // 00e5dc9c
                    e.mem.set_u32(esp.wrapping_add(0x20), 0x00000001u32); // 00e5dca0
                    pc = 0x00e5dcbc; // 00e5dca8
                    continue;
                }
                0x00e5dcaa => {
                    e.mem.set_u32(esp.wrapping_add(0x20), 0x00000000u32); // 00e5dcaa
                    pc = 0x00e5dcbc; // 00e5dcb2
                    continue;
                }
                0x00e5dcb4 => {
                    e.mem.set_u32(esp.wrapping_add(0x20), 0x00000002u32); // 00e5dcb4
                    pc = 0x00e5dcbc;
                }
                0x00e5dcbc => {
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5dcbc
                    x[1] = load_vec(e, eax.wrapping_add(0x10)); // 00e5dcbf
                    x[2] = load_vec(e, eax); // 00e5dcc3
                    eax = e.mem.u32(esp.wrapping_add(0x18)); // 00e5dcc6
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5dcca
                    eax = e.mem.u32(eax.wrapping_add(0x30)); // 00e5dcce
                    {
                        let r = fl.add(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5dcd1
                    x[0] = x[1]; // 00e5dcd4
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e5dcd7
                    e.mem.set_u32(esp.wrapping_add(0x1c), 0x0000000fu32); // 00e5dcda
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e5dce2
                    x[0] = addps(x[0], x[2]); // 00e5dce6
                    x[1] = subps(x[1], x[2]); // 00e5dce9
                    e.mem.set_u32(esp.wrapping_add(0x10), eax); // 00e5dcec
                    if fl.sf {
                        pc = 0x00e5dddc;
                        continue;
                    } // 00e5dcf0
                    x[4] = x[0]; // 00e5dcf6
                    x[5] = x[0]; // 00e5dcf9
                    x[4] = shufps(x[4], x[0], 0x55); // 00e5dcfc
                    x[5] = shufps(x[5], x[0], 0x0); // 00e5dd00
                    x[2] = x[1]; // 00e5dd04
                    x[3] = x[1]; // 00e5dd07
                    x[0] = shufps(x[0], x[0], 0xaa); // 00e5dd0a
                    x[2] = shufps(x[2], x[1], 0x55); // 00e5dd0e
                    x[3] = shufps(x[3], x[1], 0x0); // 00e5dd12
                    store_vec(e, esp.wrapping_add(0xb0), x[0]); // 00e5dd16
                    eax = ecx.wrapping_add(0x220); // 00e5dd1e
                    x[1] = shufps(x[1], x[1], 0xaa); // 00e5dd24
                    x[0] = xorps(x[0], x[0]); // 00e5dd28
                    store_vec(e, esp.wrapping_add(0xc0), x[1]); // 00e5dd2b
                    store_vec(e, esp.wrapping_add(0xa0), x[0]); // 00e5dd33
                    e.mem.set_u32(esp.wrapping_add(0x40), eax); // 00e5dd3b
                    pc = 0x00e5dd45; // 00e5dd3f
                    continue;
                }
                0x00e5dd41 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x3c)); // 00e5dd41
                    pc = 0x00e5dd45;
                }
                0x00e5dd45 => {
                    x[1] = load_vec(e, eax.wrapping_sub(0x20)); // 00e5dd45
                    x[0] = load_vec(e, eax.wrapping_sub(0x10)); // 00e5dd49
                    x[6] = load_vec(e, ecx); // 00e5dd4d
                    x[7] = load_vec(e, eax.wrapping_sub(0x1f0)); // 00e5dd50
                    x[0] = mulps(x[0], x[2]); // 00e5dd57
                    x[1] = mulps(x[1], x[3]); // 00e5dd5a
                    x[1] = addps(x[1], x[0]); // 00e5dd5d
                    x[0] = load_vec(e, eax); // 00e5dd60
                    x[0] = mulps(x[0], load_vec(e, esp.wrapping_add(0xc0))); // 00e5dd63
                    x[1] = addps(x[1], x[0]); // 00e5dd6b
                    x[0] = load_vec(e, eax.wrapping_sub(0x210)); // 00e5dd6e
                    x[6] = mulps(x[6], x[5]); // 00e5dd75
                    x[0] = mulps(x[0], x[4]); // 00e5dd78
                    x[0] = addps(x[0], x[6]); // 00e5dd7b
                    x[6] = load_vec(e, eax.wrapping_sub(0x200)); // 00e5dd7e
                    x[6] = mulps(x[6], load_vec(e, esp.wrapping_add(0xb0))); // 00e5dd85
                    x[6] = addps(x[6], x[7]); // 00e5dd8d
                    x[0] = addps(x[0], x[6]); // 00e5dd90
                    x[6] = load_vec(e, esp.wrapping_add(0xa0)); // 00e5dd93
                    x[7] = x[0]; // 00e5dd9b
                    x[7] = subps(x[7], x[1]); // 00e5dd9e
                    x[7] = cmpltps(x[7], x[6]); // 00e5dda1
                    ecx = movmskps(x[7]); // 00e5dda5
                    fl.sub(ecx, 0x0000000fu32, 32); // 00e5dda8
                    if !fl.zf {
                        pc = 0x00e5ddf8;
                        continue;
                    } // 00e5ddab
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x3c)), 0x00000040u32, 32);
                        e.mem.set_u32(esp.wrapping_add(0x3c), r);
                    } // 00e5ddad
                    x[0] = addps(x[0], x[1]); // 00e5ddb2
                    x[0] = cmpltps(x[0], x[6]); // 00e5ddb5
                    eax = movmskps(x[0]); // 00e5ddb9
                    {
                        let r = fl.and(e.mem.u32(esp.wrapping_add(0x1c)), eax, 32);
                        e.mem.set_u32(esp.wrapping_add(0x1c), r);
                    } // 00e5ddbc
                    eax = e.mem.u32(esp.wrapping_add(0x40)); // 00e5ddc0
                    {
                        let r = fl.add(eax, 0x00000040u32, 32);
                        eax = r;
                    } // 00e5ddc4
                    {
                        let r = fl.sub(e.mem.u32(esp.wrapping_add(0x10)), 0x00000001u32, 32);
                        e.mem.set_u32(esp.wrapping_add(0x10), r);
                    } // 00e5ddc7
                    e.mem.set_u32(esp.wrapping_add(0x40), eax); // 00e5ddcc
                    if !fl.sf {
                        pc = 0x00e5dd41;
                        continue;
                    } // 00e5ddd0
                    fl.sub(e.mem.u32(esp.wrapping_add(0x1c)), ecx, 32); // 00e5ddd6
                    if !fl.zf {
                        pc = 0x00e5ddfc;
                        continue;
                    } // 00e5ddda
                    pc = 0x00e5dddc;
                }
                0x00e5dddc => {
                    ecx = ebx.wrapping_add(esi); // 00e5dddc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5dddf
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5dde0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5dde4
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5dde5
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e65ee0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5dde8
                    edx = e.mem.u32(esp.wrapping_add(0x14)); // 00e5dded
                    eax = 0x00000001u32; // 00e5ddf1
                    pc = 0x00e5de01; // 00e5ddf6
                    continue;
                }
                0x00e5ddf8 => {
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    } // 00e5ddf8
                    pc = 0x00e5de01; // 00e5ddfa
                    continue;
                }
                0x00e5ddfc => {
                    eax = 0x00000002u32; // 00e5ddfc
                    pc = 0x00e5de01;
                }
                0x00e5de01 => {
                    fl.sub(e.mem.u32(esp.wrapping_add(0x20)), 0x00000002u32, 32); // 00e5de01
                    if fl.zf {
                        pc = 0x00e5de1c;
                        continue;
                    } // 00e5de06
                    fl.sub(eax, 0x00000002u32, 32); // 00e5de08
                    if !fl.zf {
                        pc = 0x00e5e352;
                        continue;
                    } // 00e5de0b
                    {
                        let r = fl.add(esi, ebx, 32);
                        esi = r;
                    } // 00e5de11
                    ebx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5de13
                    pc = 0x00e5d7f0; // 00e5de17
                    continue;
                }
                0x00e5de1c => {
                    fl.sub(eax, 0x00000002u32, 32); // 00e5de1c
                    if fl.zf {
                        pc = 0x00e5de43;
                        continue;
                    } // 00e5de1f
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5de21
                    x[0] = load_vec(e, esp.wrapping_add(0x70)); // 00e5de24
                    ebx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5de29
                    store_vec(e, eax, x[0]); // 00e5de2d
                    x[0] = load_vec(e, esp.wrapping_add(0x80)); // 00e5de30
                    store_vec(e, eax.wrapping_add(0x10), x[0]); // 00e5de38
                    {
                        let r = fl.add(esi, edx, 32);
                        esi = r;
                    } // 00e5de3c
                    pc = 0x00e5d7f0; // 00e5de3e
                    continue;
                }
                0x00e5de43 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5de43
                    eax = esp.wrapping_add(0x70); // 00e5de47
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5de4b
                    {
                        let r = fl.add(edx, esi, 32);
                        edx = r;
                    } // 00e5de4c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5de4e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5de4f
                    {
                        fn_00e5d7d0(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5de50
                    {
                        let r = fl.add(esi, ebx, 32);
                        esi = r;
                    } // 00e5de55
                    ebx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5de57
                    pc = 0x00e5d7f0; // 00e5de5b
                    continue;
                }
                0x00e5de60 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5de60
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5de64
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5de68
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5de6b
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5de6d
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5de71
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5de74
                    e.mem.set_u32(esp.wrapping_add(0x10), ecx); // 00e5de76
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5de7a
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5de7d
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5de81
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0xa4),
                        fpu.fstp_m32(),
                    ); // 00e5de84
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5de8b
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x5))); // 00e5de8f
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5de93
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5de96
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x6))); // 00e5de98
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5de9c
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5de9f
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5dea1
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5dea5
                    {
                        let r = fl.add(esi, 0x00000007u32, 32);
                        esi = r;
                    } // 00e5dea8
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5deab
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5deaf
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x94),
                        fpu.fstp_m32(),
                    ); // 00e5deb2
                    pc = 0x00e5df08; // 00e5deb9
                    continue;
                }
                0x00e5debb => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5debb
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5debf
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5dec2
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5dec4
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5dec7
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5dece
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5ded2
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5ded6
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x98),
                        fpu.fstp_m32(),
                    ); // 00e5ded9
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5dee0
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5dee4
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5dee7
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5dee9
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5deec
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5def3
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5def6
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x10))); // 00e5defa
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5defe
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x88),
                        fpu.fstp_m32(),
                    ); // 00e5df01
                    pc = 0x00e5df08;
                }
                0x00e5df08 => {
                    eax = (eax & 0xffff_ff00) | ((u32::from(e.mem.u8(esi))) & 0xff); // 00e5df08
                    fl.sub((eax & 0xff), 0x00000026u32, 8); // 00e5df0a
                    if fl.cf {
                        pc = 0x00e5df16;
                        continue;
                    } // 00e5df0c
                    fl.sub((eax & 0xff), 0x0000002bu32, 8); // 00e5df0e
                    if fl.cf || fl.zf {
                        pc = 0x00e5d7f0;
                        continue;
                    } // 00e5df10
                    pc = 0x00e5df16;
                }
                0x00e5df16 => {
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5df16
                    x[1] = load_vec(e, eax.wrapping_add(0x10)); // 00e5df19
                    x[2] = load_vec(e, eax); // 00e5df1d
                    eax = e.mem.u32(ebx.wrapping_add(0x30)); // 00e5df20
                    ecx = ebx.wrapping_add(0x20); // 00e5df23
                    edx = ecx.wrapping_add(0x20); // 00e5df26
                    x[0] = x[1]; // 00e5df29
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e5df2c
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e5df2f
                    e.mem.set_u32(esp.wrapping_add(0x20), 0x0000000fu32); // 00e5df33
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5df3b
                    x[0] = addps(x[0], x[2]); // 00e5df3f
                    x[1] = subps(x[1], x[2]); // 00e5df42
                    e.mem.set_u32(esp.wrapping_add(0x40), eax); // 00e5df45
                    if fl.sf {
                        pc = 0x00e5e036;
                        continue;
                    } // 00e5df49
                    x[4] = x[0]; // 00e5df4f
                    x[5] = x[0]; // 00e5df52
                    x[4] = shufps(x[4], x[0], 0x55); // 00e5df55
                    x[5] = shufps(x[5], x[0], 0x0); // 00e5df59
                    x[2] = x[1]; // 00e5df5d
                    x[3] = x[1]; // 00e5df60
                    x[0] = shufps(x[0], x[0], 0xaa); // 00e5df63
                    x[2] = shufps(x[2], x[1], 0x55); // 00e5df67
                    x[3] = shufps(x[3], x[1], 0x0); // 00e5df6b
                    store_vec(e, esp.wrapping_add(0xb0), x[0]); // 00e5df6f
                    x[1] = shufps(x[1], x[1], 0xaa); // 00e5df77
                    x[0] = xorps(x[0], x[0]); // 00e5df7b
                    store_vec(e, esp.wrapping_add(0xc0), x[1]); // 00e5df7e
                    store_vec(e, esp.wrapping_add(0xa0), x[0]); // 00e5df86
                    eax = edx.wrapping_add(0x220); // 00e5df8e
                    pc = 0x00e5df94;
                }
                0x00e5df94 => {
                    x[1] = load_vec(e, eax.wrapping_sub(0x20)); // 00e5df94
                    x[0] = load_vec(e, eax.wrapping_sub(0x10)); // 00e5df98
                    x[6] = load_vec(e, edx); // 00e5df9c
                    x[7] = load_vec(e, eax.wrapping_sub(0x1f0)); // 00e5df9f
                    x[0] = mulps(x[0], x[2]); // 00e5dfa6
                    x[1] = mulps(x[1], x[3]); // 00e5dfa9
                    x[1] = addps(x[1], x[0]); // 00e5dfac
                    x[0] = load_vec(e, eax); // 00e5dfaf
                    x[0] = mulps(x[0], load_vec(e, esp.wrapping_add(0xc0))); // 00e5dfb2
                    x[1] = addps(x[1], x[0]); // 00e5dfba
                    x[0] = load_vec(e, eax.wrapping_sub(0x210)); // 00e5dfbd
                    x[6] = mulps(x[6], x[5]); // 00e5dfc4
                    x[0] = mulps(x[0], x[4]); // 00e5dfc7
                    x[0] = addps(x[0], x[6]); // 00e5dfca
                    x[6] = load_vec(e, eax.wrapping_sub(0x200)); // 00e5dfcd
                    x[6] = mulps(x[6], load_vec(e, esp.wrapping_add(0xb0))); // 00e5dfd4
                    x[6] = addps(x[6], x[7]); // 00e5dfdc
                    x[0] = addps(x[0], x[6]); // 00e5dfdf
                    x[6] = load_vec(e, esp.wrapping_add(0xa0)); // 00e5dfe2
                    x[7] = x[0]; // 00e5dfea
                    x[7] = subps(x[7], x[1]); // 00e5dfed
                    x[7] = cmpltps(x[7], x[6]); // 00e5dff0
                    edx = movmskps(x[7]); // 00e5dff4
                    fl.sub(edx, 0x0000000fu32, 32); // 00e5dff7
                    if !fl.zf {
                        pc = 0x00e5e352;
                        continue;
                    } // 00e5dffa
                    edx = e.mem.u32(esp.wrapping_add(0x10)); // 00e5e000
                    x[0] = addps(x[0], x[1]); // 00e5e004
                    x[0] = cmpltps(x[0], x[6]); // 00e5e007
                    ecx = movmskps(x[0]); // 00e5e00b
                    {
                        let r = fl.and(e.mem.u32(esp.wrapping_add(0x20)), ecx, 32);
                        e.mem.set_u32(esp.wrapping_add(0x20), r);
                    } // 00e5e00e
                    {
                        let r = fl.add(edx, 0x00000040u32, 32);
                        edx = r;
                    } // 00e5e012
                    {
                        let r = fl.add(eax, 0x00000040u32, 32);
                        eax = r;
                    } // 00e5e015
                    {
                        let r = fl.sub(e.mem.u32(esp.wrapping_add(0x40)), 0x00000001u32, 32);
                        e.mem.set_u32(esp.wrapping_add(0x40), r);
                    } // 00e5e018
                    e.mem.set_u32(esp.wrapping_add(0x10), edx); // 00e5e01d
                    if !fl.sf {
                        pc = 0x00e5df94;
                        continue;
                    } // 00e5e021
                    fl.sub(e.mem.u32(esp.wrapping_add(0x20)), 0x0000000fu32, 32); // 00e5e027
                    if !fl.zf {
                        pc = 0x00e5d7f0;
                        continue;
                    } // 00e5e02c
                    ecx = e.mem.u32(esp.wrapping_add(0x3c)); // 00e5e032
                    pc = 0x00e5e036;
                }
                0x00e5e036 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5e036
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5e037
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e65ee0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5e038
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5e03d
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5e03e
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5e03f
                    esp = ebp; // 00e5e040
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5e042
                    return; // 00e5e043
                }
                0x00e5e046 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e046
                    esi = esi.wrapping_add(edx).wrapping_add(0x2); // 00e5e04a
                    pc = 0x00e5d7f0; // 00e5e04e
                    continue;
                }
                0x00e5e053 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e053
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e057
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e05b
                    {
                        let r = fl.add(ecx, esi, 32);
                        ecx = r;
                    } // 00e5e05e
                    esi = ecx.wrapping_add(eax).wrapping_add(0x3); // 00e5e060
                    pc = 0x00e5d7f0; // 00e5e064
                    continue;
                }
                0x00e5e069 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e069
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e06d
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5e071
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5e075
                    {
                        let r = fl.add(edx, eax, 32);
                        edx = r;
                    } // 00e5e078
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5e07a
                    {
                        let r = fl.add(ecx, esi, 32);
                        ecx = r;
                    } // 00e5e07d
                    esi = ecx.wrapping_add(edx).wrapping_add(0x4); // 00e5e07f
                    pc = 0x00e5d7f0; // 00e5e083
                    continue;
                }
                0x00e5e088 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e088
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e08c
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5e08f
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0x8)), 32);
                        edx = r;
                    } // 00e5e091
                    e.mem.set_u32(esp.wrapping_add(0x60), edx); // 00e5e094
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e098
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e09c
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5e09f
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0xc)), 32);
                        edx = r;
                    } // 00e5e0a1
                    e.mem.set_u32(esp.wrapping_add(0x64), edx); // 00e5e0a4
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5e0a8
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e0ac
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5e0af
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0x10)), 32);
                        edx = r;
                    } // 00e5e0b1
                    e.mem.set_u32(esp.wrapping_add(0x68), edx); // 00e5e0b4
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e0b8
                    {
                        let r = fl.sub(ecx, eax, 32);
                        ecx = r;
                    } // 00e5e0bb
                    e.mem.set_u32(esp.wrapping_add(0x6c), ecx); // 00e5e0bd
                    edx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5e0c1
                    e.mem.set_u32(esp.wrapping_add(0x5c), edx); // 00e5e0c4
                    eax = e.mem.u32(edi); // 00e5e0c8
                    e.mem.set_u32(esp.wrapping_add(0x58), eax); // 00e5e0ca
                    edi = esp.wrapping_add(0x58); // 00e5e0ce
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5e0d2
                    pc = 0x00e5d7f0; // 00e5e0d5
                    continue;
                }
                0x00e5e0da => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e0da
                    ecx = esp.wrapping_add(0x58); // 00e5e0de
                    fl.sub(edi, ecx, 32); // 00e5e0e2
                    if fl.zf {
                        pc = 0x00e5e113;
                        continue;
                    } // 00e5e0e4
                    edx = e.mem.u32(edi); // 00e5e0e6
                    e.mem.set_u32(esp.wrapping_add(0x58), edx); // 00e5e0e8
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5e0ec
                    e.mem.set_u32(esp.wrapping_add(0x5c), ecx); // 00e5e0ef
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5e0f3
                    e.mem.set_u32(esp.wrapping_add(0x60), edx); // 00e5e0f6
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5e0fa
                    e.mem.set_u32(esp.wrapping_add(0x64), ecx); // 00e5e0fd
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5e101
                    e.mem.set_u32(esp.wrapping_add(0x68), edx); // 00e5e104
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e108
                    e.mem.set_u32(esp.wrapping_add(0x6c), ecx); // 00e5e10b
                    edi = esp.wrapping_add(0x58); // 00e5e10f
                    pc = 0x00e5e113;
                }
                0x00e5e113 => {
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x58)), eax, 32);
                        e.mem.set_u32(esp.wrapping_add(0x58), r);
                    } // 00e5e113
                    {
                        let r = fl.add(esi, 0x00000002u32, 32);
                        esi = r;
                    } // 00e5e117
                    pc = 0x00e5d7f0; // 00e5e11a
                    continue;
                }
                0x00e5e11f => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e11f
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e123
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e127
                    ecx = esp.wrapping_add(0x58); // 00e5e12a
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5e12e
                    fl.sub(edi, ecx, 32); // 00e5e130
                    if fl.zf {
                        pc = 0x00e5e161;
                        continue;
                    } // 00e5e132
                    edx = e.mem.u32(edi); // 00e5e134
                    e.mem.set_u32(esp.wrapping_add(0x58), edx); // 00e5e136
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5e13a
                    e.mem.set_u32(esp.wrapping_add(0x5c), ecx); // 00e5e13d
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5e141
                    e.mem.set_u32(esp.wrapping_add(0x60), edx); // 00e5e144
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5e148
                    e.mem.set_u32(esp.wrapping_add(0x64), ecx); // 00e5e14b
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5e14f
                    e.mem.set_u32(esp.wrapping_add(0x68), edx); // 00e5e152
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e156
                    e.mem.set_u32(esp.wrapping_add(0x6c), ecx); // 00e5e159
                    edi = esp.wrapping_add(0x58); // 00e5e15d
                    pc = 0x00e5e161;
                }
                0x00e5e161 => {
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x58)), eax, 32);
                        e.mem.set_u32(esp.wrapping_add(0x58), r);
                    } // 00e5e161
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5e165
                    pc = 0x00e5d7f0; // 00e5e168
                    continue;
                }
                0x00e5e16d => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e16d
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e171
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5e175
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e179
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5e17c
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5e17e
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e182
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5e185
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e187
                    ecx = esp.wrapping_add(0x58); // 00e5e18a
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5e18e
                    fl.sub(edi, ecx, 32); // 00e5e190
                    if fl.zf {
                        pc = 0x00e5e1c1;
                        continue;
                    } // 00e5e192
                    edx = e.mem.u32(edi); // 00e5e194
                    e.mem.set_u32(esp.wrapping_add(0x58), edx); // 00e5e196
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5e19a
                    e.mem.set_u32(esp.wrapping_add(0x5c), ecx); // 00e5e19d
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5e1a1
                    e.mem.set_u32(esp.wrapping_add(0x60), edx); // 00e5e1a4
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5e1a8
                    e.mem.set_u32(esp.wrapping_add(0x64), ecx); // 00e5e1ab
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5e1af
                    e.mem.set_u32(esp.wrapping_add(0x68), edx); // 00e5e1b2
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e1b6
                    e.mem.set_u32(esp.wrapping_add(0x6c), ecx); // 00e5e1b9
                    edi = esp.wrapping_add(0x58); // 00e5e1bd
                    pc = 0x00e5e1c1;
                }
                0x00e5e1c1 => {
                    e.mem.set_u32(esp.wrapping_add(0x58), eax); // 00e5e1c1
                    {
                        let r = fl.add(esi, 0x00000005u32, 32);
                        esi = r;
                    } // 00e5e1c5
                    pc = 0x00e5d7f0; // 00e5e1c8
                    continue;
                }
                0x00e5e1cd => {
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e1cd
                    {
                        let r = fl.add(esi, 0x00000002u32, 32);
                        esi = r;
                    } // 00e5e1d1
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x124),
                        ecx,
                    ); // 00e5e1d4
                    pc = 0x00e5e21f; // 00e5e1db
                    continue;
                }
                0x00e5e1dd => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e1dd
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e1e1
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5e1e5
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5e1e8
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x134),
                        edx,
                    ); // 00e5e1ea
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5e1f1
                    pc = 0x00e5e21f; // 00e5e1f4
                    continue;
                }
                0x00e5e1f6 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e1f6
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e1fa
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5e1fe
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5e201
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5e203
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5e207
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5e20a
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5e20c
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5e210
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5e213
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x144),
                        edx,
                    ); // 00e5e215
                    {
                        let r = fl.add(esi, 0x00000005u32, 32);
                        esi = r;
                    } // 00e5e21c
                    pc = 0x00e5e21f;
                }
                0x00e5e21f => {
                    eax = e.mem.u32(esp.wrapping_add(0x5c)); // 00e5e21f
                    edx = esp.wrapping_add(0x58); // 00e5e223
                    fl.sub(edi, edx, 32); // 00e5e227
                    if fl.zf {
                        pc = 0x00e5e258;
                        continue;
                    } // 00e5e229
                    ecx = e.mem.u32(edi); // 00e5e22b
                    e.mem.set_u32(esp.wrapping_add(0x58), ecx); // 00e5e22d
                    edx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5e231
                    e.mem.set_u32(esp.wrapping_add(0x5c), edx); // 00e5e234
                    ecx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5e238
                    e.mem.set_u32(esp.wrapping_add(0x60), ecx); // 00e5e23b
                    edx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5e23f
                    e.mem.set_u32(esp.wrapping_add(0x64), edx); // 00e5e242
                    ecx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5e246
                    e.mem.set_u32(esp.wrapping_add(0x68), ecx); // 00e5e249
                    edx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e24d
                    e.mem.set_u32(esp.wrapping_add(0x6c), edx); // 00e5e250
                    edi = esp.wrapping_add(0x58); // 00e5e254
                    pc = 0x00e5e258;
                }
                0x00e5e258 => {
                    e.mem.set_u32(esp.wrapping_add(0x5c), eax); // 00e5e258
                    pc = 0x00e5d7f0; // 00e5e25c
                    continue;
                }
                0x00e5e261 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000200u32); // 00e5e261
                    eax = esp.wrapping_add(0xd4); // 00e5e266
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5e26d
                    ecx = esp.wrapping_add(0x9c); // 00e5e26e
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00cb1600, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5e275
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x010d5100u32); // 00e5e27a
                    ecx = esp.wrapping_add(0x98); // 00e5e27f
                    {
                        let w = [ecx, e.mem.u32(esp)];
                        let r = e.call(0x00cc01c0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(4); // 00e5e286
                    ecx = e.mem.u32(0x01268360u32); // 00e5e28b
                    edx = e.mem.u32(ecx); // 00e5e291
                    edx = e.mem.u32(edx.wrapping_add(0xc)); // 00e5e293
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x0000012du32); // 00e5e296
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x010ecd88u32); // 00e5e29b
                    eax = esp.wrapping_add(0xd8); // 00e5e2a0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5e2a7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x1298feddu32); // 00e5e2a8
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000003u32); // 00e5e2ad
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                            e.mem.u32(esp.wrapping_add(16)),
                        ];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(20); // 00e5e2af
                    fl.and(eax, eax, 32); // 00e5e2b1
                    if fl.zf {
                        pc = 0x00e5e2b6;
                        continue;
                    } // 00e5e2b3
                    panic!("INT3 at 00e5e2b5"); // 00e5e2b5
                }
                0x00e5e2b6 => {
                    ecx = esp.wrapping_add(0x94); // 00e5e2b6
                    {
                        let w = [ecx];
                        let r = e.call(0x00cc00e0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5e2bd
                    pc = 0x00e5d7f0; // 00e5e2c2
                    continue;
                }
                0x00e5e2c7 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e2c7
                    pc = 0x00e5e318; // 00e5e2cb
                    continue;
                }
                0x00e5e2cd => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e2cd
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e2d1
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e2d5
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5e2d8
                    pc = 0x00e5e318; // 00e5e2da
                    continue;
                }
                0x00e5e2dc => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e2dc
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e2e0
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5e2e4
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e2e8
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5e2eb
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e2ed
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5e2f0
                    pc = 0x00e5e318; // 00e5e2f2
                    continue;
                }
                0x00e5e2f4 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e2f4
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e2f8
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5e2fc
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e300
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5e303
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5e305
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e309
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5e30c
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e30e
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5e311
                    pc = 0x00e5e318; // 00e5e313
                    continue;
                }
                0x00e5e315 => {
                    {
                        let r = fl.add(eax, 0xffffffd0u32, 32);
                        eax = r;
                    } // 00e5e315
                    pc = 0x00e5e318;
                }
                0x00e5e318 => {
                    ebx = e.mem.u32(ebx); // 00e5e318
                    esi = eax; // 00e5e31a
                    {
                        let r = fl.add(esi, e.mem.u32(edi), 32);
                        esi = r;
                    } // 00e5e31c
                    e.mem.set_u32(esp.wrapping_add(0x14), eax); // 00e5e31e
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5e322
                    ecx = e.mem.u32(ebx.wrapping_add(0x8)); // 00e5e327
                    {
                        let r = fl.and(ecx, 0x3fffffffu32, 32);
                        ecx = r;
                    } // 00e5e32a
                    fl.sub(e.mem.u32(ebx.wrapping_add(0x4)), ecx, 32); // 00e5e330
                    if !fl.zf {
                        pc = 0x00e5e344;
                        continue;
                    } // 00e5e333
                    edx = e.mem.u32(eax.wrapping_add(0x10)); // 00e5e335
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000004u32); // 00e5e338
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e5e33a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5e33b
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00c90940, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5e33c
                    {
                        let r = fl.add(esp, 0x0000000cu32, 32);
                        esp = r;
                    } // 00e5e341
                    pc = 0x00e5e344;
                }
                0x00e5e344 => {
                    eax = e.mem.u32(ebx.wrapping_add(0x4)); // 00e5e344
                    ecx = e.mem.u32(ebx); // 00e5e347
                    ecx = ecx.wrapping_add(eax.wrapping_mul(4)); // 00e5e349
                    {
                        let r = fl.inc(eax, 32);
                        eax = r;
                    } // 00e5e34c
                    e.mem.set_u32(ebx.wrapping_add(0x4), eax); // 00e5e34d
                    e.mem.set_u32(ecx, esi); // 00e5e350
                    pc = 0x00e5e352;
                }
                0x00e5e352 => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5e352
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5e353
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5e354
                    esp = ebp; // 00e5e355
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5e357
                    return; // 00e5e358
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5e430 (decompiled, FalloutNV.exe 1.4.0.525)
/// MOPP code interpreter (thiscall, `RET 0xc`: `this` the machine, `sink`, `code` (the address of the
/// current code byte) and `bounds` (the 32 bytes of query data)): each step dispatches on the code
/// byte (values up to 0x6b through a byte map and a 23-entry jump table written as `match`es
/// on the code byte and the case), tests the query against the node's split planes with SSE (`MULPS`, `ADDPS`,
/// `SHUFPS`, `CMPLTPS`, `MOVMSKPS`), and either descends by calling itself with the offset code
/// pointer, walks on, or reports hits through the machine's virtual methods (`00c85750`, `00c90940`
/// array growth, `00cb1600`, `00cc00e0`, `00cc01c0` and the `00e65ee0`-family callees). An opcode
/// the table does not know ends in an `INT3`, which stops here with a panic. Transcribed
/// instruction by instruction. The four variants of this interpreter differ in the data layout of
/// the query.
pub fn fn_00e5e430(e: &mut Engine, this: Ptr, sink: Ptr, code: Ptr, bounds: Ptr) {
    e.with_stack(0x500, |e, frame| {
        let mut esp = frame.addr() + 0x400;
        e.mem.set_u32(esp.wrapping_add(4), sink.addr());
        e.mem.set_u32(esp.wrapping_add(8), code.addr());
        e.mem.set_u32(esp.wrapping_add(12), bounds.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5e430;
        loop {
            match pc {
                0x00e5e430 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5e430
                    ebp = esp; // 00e5e431
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5e433
                    {
                        let r = fl.sub(esp, 0x00000294u32, 32);
                        esp = r;
                    } // 00e5e436
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e5e43c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5e43d
                    esi = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5e43e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5e441
                    edi = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5e442
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e5e445
                                                                // no change (a move of a register onto itself); // 00e5e449
                    pc = 0x00e5e450;
                }
                0x00e5e450 => {
                    eax = u32::from(e.mem.u8(esi)); // 00e5e450
                    fl.sub(eax, 0x0000006bu32, 32); // 00e5e453
                    if !fl.cf && !fl.zf {
                        pc = 0x00e5eeac;
                        continue;
                    } // 00e5e456
                    ecx = match eax {
                        0 => 0,
                        1..=4 => 1,
                        5 => 2,
                        6 => 3,
                        7 => 4,
                        8 | 12..=15 | 29..=31 | 44..=47 | 84..=95 => 22,
                        9 => 5,
                        10 => 6,
                        11 => 7,
                        16..=18 => 8,
                        19..=28 => 9,
                        32..=34 => 10,
                        35..=37 => 11,
                        38..=40 => 12,
                        41..=43 => 13,
                        48..=79 => 14,
                        80 => 15,
                        81 => 16,
                        82 => 17,
                        83 => 18,
                        96..=99 => 19,
                        100..=103 => 20,
                        104..=107 => 21,
                        _ => unreachable!(),
                    }; // 00e5e45c
                    pc = match ecx {
                        0 => 0x00e5_ef92,
                        1 => 0x00e5_ecd3,
                        2 => 0x00e5_ec91,
                        3 => 0x00e5_ec9e,
                        4 => 0x00e5_ecb4,
                        5 => 0x00e5_ed25,
                        6 => 0x00e5_ed6a,
                        7 => 0x00e5_edb8,
                        8 => 0x00e5_e499,
                        9 => 0x00e5_e46a,
                        10 => 0x00e5_e4f9,
                        11 => 0x00e5_e553,
                        12 => 0x00e5_eaf6,
                        13 => 0x00e5_ea96,
                        14 => 0x00e5_ef57,
                        15 => 0x00e5_ef09,
                        16 => 0x00e5_ef0f,
                        17 => 0x00e5_ef1e,
                        18 => 0x00e5_ef36,
                        19 => 0x00e5_ee18,
                        20 => 0x00e5_ee28,
                        21 => 0x00e5_ee41,
                        22 => 0x00e5_eeac,
                        _ => unreachable!(),
                    }; // 00e5e463
                    continue;
                }
                0x00e5e46a => {
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5e46a
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e46e
                    x[0] = load_vec(e, eax); // 00e5e471
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e474
                    edx = esp.wrapping_add(0x40); // 00e5e478
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5e47c
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5e47d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5e480
                    store_vec(e, esp.wrapping_add(0x48), x[0]); // 00e5e481
                    x[0] = load_vec(e, eax.wrapping_add(0x10)); // 00e5e486
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5e48a
                    store_vec(e, esp.wrapping_add(0x5c), x[0]); // 00e5e48b
                    {
                        fn_00e5e430(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5e490
                    {
                        let r = fl.add(esi, ebx, 32);
                        esi = r;
                    } // 00e5e495
                    pc = 0x00e5e450; // 00e5e497
                    continue;
                }
                0x00e5e499 => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e499
                    x[0] = load_vec(e, ecx); // 00e5e49c
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5e49f
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5e4a4
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e4a8
                    store_vec(e, esp.wrapping_add(0x50), x[0]); // 00e5e4ab
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e4b0
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e4b4
                    {
                        let r = fl.shl(ebx, ((ecx & 0xff) & 0x1f), 32);
                        ebx = r;
                    } // 00e5e4b8
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e4ba
                    {
                        let r = fl.add(
                            ebx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x38)),
                            32,
                        );
                        ebx = r;
                    } // 00e5e4bd
                    e.mem.set_u32(esp.wrapping_add(0x14), ebx); // 00e5e4c1
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e4c5
                    ebx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e4c9
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5e4cd
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x30),
                        fpu.fstp_m32(),
                    ); // 00e5e4d0
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e4d4
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5e4d7
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x38)),
                            32,
                        );
                        edx = r;
                    } // 00e5e4d9
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5e4dd
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e4e1
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5e4e5
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e5e4e8
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5e4ea
                    e.mem
                        .set_u32(esp.wrapping_add(eax.wrapping_mul(4)), fpu.fstp_m32()); // 00e5e4ed
                    eax = u32::from(e.mem.u8(esi.wrapping_sub(0x1))); // 00e5e4f0
                    pc = 0x00e5e5c9; // 00e5e4f4
                    continue;
                }
                0x00e5e4f9 => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e4f9
                    x[0] = load_vec(e, ecx); // 00e5e4fc
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5e4ff
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5e504
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e508
                    store_vec(e, esp.wrapping_add(0x50), x[0]); // 00e5e50b
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e510
                    ebx = edx.wrapping_add(0x1); // 00e5e514
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5e517
                    {
                        let r = fl.shl(ebx, ((ecx & 0xff) & 0x1f), 32);
                        ebx = r;
                    } // 00e5e519
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x78)),
                            32,
                        );
                        edx = r;
                    } // 00e5e51b
                    {
                        let r = fl.add(
                            ebx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x78)),
                            32,
                        );
                        ebx = r;
                    } // 00e5e51f
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5e523
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e527
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e52b
                    e.mem.set_u32(esp.wrapping_add(0x14), ebx); // 00e5e52f
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e5e533
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5e535
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5e538
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x40),
                        fpu.fstp_m32(),
                    ); // 00e5e53b
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e53f
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5e543
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e546
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x70),
                        fpu.fstp_m32(),
                    ); // 00e5e549
                    eax = u32::from(e.mem.u8(esi.wrapping_sub(0x1))); // 00e5e54d
                    pc = 0x00e5e5c9; // 00e5e551
                    continue;
                }
                0x00e5e553 => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e553
                    x[0] = load_vec(e, ecx); // 00e5e556
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5e559
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5e55e
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e562
                    store_vec(e, esp.wrapping_add(0x50), x[0]); // 00e5e565
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5e56a
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5e56e
                    {
                        let r = fl.shl(ebx, ((ecx & 0xff) & 0x1f), 32);
                        ebx = r;
                    } // 00e5e572
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e574
                    {
                        let r = fl.add(
                            ebx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x84)),
                            32,
                        );
                        ebx = r;
                    } // 00e5e577
                    e.mem.set_u32(esp.wrapping_add(0x14), ebx); // 00e5e57e
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e582
                    ebx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e586
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5e58a
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x7c),
                        fpu.fstp_m32(),
                    ); // 00e5e58d
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e591
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5e594
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x84)),
                            32,
                        );
                        edx = r;
                    } // 00e5e596
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5e59d
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e5a1
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5e5a5
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x4c),
                        fpu.fstp_m32(),
                    ); // 00e5e5a8
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5e5ac
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x5))); // 00e5e5b0
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5e5b4
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x6))); // 00e5e5b8
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5e5bc
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5e5bf
                    {
                        let r = fl.add(ebx, edx, 32);
                        ebx = r;
                    } // 00e5e5c2
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5e5c4
                    {
                        let r = fl.add(esi, 0x00000007u32, 32);
                        esi = r;
                    } // 00e5e5c6
                    pc = 0x00e5e5c9;
                }
                0x00e5e5c9 => {
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi))); // 00e5e5c9
                    fl.sub(ecx, 0x00000026u32, 32); // 00e5e5cd
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5e5d0
                    if fl.sf != fl.of {
                        pc = 0x00e5e6c9;
                        continue;
                    } // 00e5e5d4
                    pc = 0x00e5e5da;
                }
                0x00e5e5da => {
                    fl.sub(ecx, 0x0000002bu32, 32); // 00e5e5da
                    if !fl.zf && fl.sf == fl.of {
                        pc = 0x00e5e6c9;
                        continue;
                    } // 00e5e5dd
                    fl.sub(ecx, 0x00000029u32, 32); // 00e5e5e3
                    if fl.sf != fl.of {
                        pc = 0x00e5e659;
                        continue;
                    } // 00e5e5e6
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x1))); // 00e5e5e8
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x2))); // 00e5e5ed
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5e5f2
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5e5f5
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x3))); // 00e5e5f7
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5e5fc
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5e5ff
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e601
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5e604
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e608
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e60c
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5e610
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e613
                    e.mem.set_u32(
                        ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_sub(0xa4),
                        fpu.fstp_m32(),
                    ); // 00e5e617
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x4))); // 00e5e61e
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x5))); // 00e5e623
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5e628
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5e62b
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x6))); // 00e5e62d
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5e632
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5e635
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e637
                    e.mem.set_u32(esp.wrapping_add(0x14), ecx); // 00e5e63a
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e63e
                    {
                        let r = fl.add(ebx, 0x00000007u32, 32);
                        ebx = r;
                    } // 00e5e642
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e645
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5e649
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e64c
                    e.mem.set_u32(
                        edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x94),
                        fpu.fstp_m32(),
                    ); // 00e5e650
                    pc = 0x00e5e6b8; // 00e5e657
                    continue;
                }
                0x00e5e659 => {
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x1))); // 00e5e659
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e65e
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5e661
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e663
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5e667
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5e66e
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e672
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e676
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5e67a
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e67d
                    e.mem.set_u32(
                        edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x98),
                        fpu.fstp_m32(),
                    ); // 00e5e680
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x2))); // 00e5e687
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e68c
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5e68f
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e691
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5e695
                    {
                        let r = fl.add(ebx, 0x00000003u32, 32);
                        ebx = r;
                    } // 00e5e69c
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5e69f
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e6a3
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e6a7
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5e6ab
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e6ae
                    e.mem.set_u32(
                        edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x88),
                        fpu.fstp_m32(),
                    ); // 00e5e6b1
                    pc = 0x00e5e6b8;
                }
                0x00e5e6b8 => {
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi))); // 00e5e6b8
                    fl.sub(ecx, 0x00000026u32, 32); // 00e5e6bc
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5e6bf
                    if fl.sf == fl.of {
                        pc = 0x00e5e5da;
                        continue;
                    } // 00e5e6c3
                    pc = 0x00e5e6c9;
                }
                0x00e5e6c9 => {
                    ecx = u32::from(e.mem.u8(eax.wrapping_add(esi))); // 00e5e6c9
                    fl.sub(ecx, 0x00000026u32, 32); // 00e5e6cd
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5e6d0
                    if fl.sf != fl.of {
                        pc = 0x00e5e7b1;
                        continue;
                    } // 00e5e6d4
                    pc = 0x00e5e6da;
                }
                0x00e5e6da => {
                    fl.sub(ecx, 0x0000002bu32, 32); // 00e5e6da
                    if !fl.zf && fl.sf == fl.of {
                        pc = 0x00e5e7b1;
                        continue;
                    } // 00e5e6dd
                    fl.sub(ecx, 0x00000029u32, 32); // 00e5e6e3
                    if fl.sf != fl.of {
                        pc = 0x00e5e74d;
                        continue;
                    } // 00e5e6e6
                    ecx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x1))); // 00e5e6e8
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x2))); // 00e5e6ed
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5e6f2
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5e6f5
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x3))); // 00e5e6f7
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5e6fc
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5e6ff
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e701
                    e.mem.set_u32(esp.wrapping_add(0x14), ecx); // 00e5e705
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e709
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e70d
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5e711
                    e.mem.set_u32(
                        esp.wrapping_add(edx.wrapping_mul(4)).wrapping_sub(0x64),
                        fpu.fstp_m32(),
                    ); // 00e5e714
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x4))); // 00e5e718
                    ecx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x5))); // 00e5e71d
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5e722
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5e725
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x6))); // 00e5e727
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5e72c
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5e72f
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e731
                    e.mem.set_u32(esp.wrapping_add(0x14), ecx); // 00e5e735
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e739
                    {
                        let r = fl.add(eax, 0x00000007u32, 32);
                        eax = r;
                    } // 00e5e73d
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e740
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5e744
                    e.mem.set_u32(
                        esp.wrapping_add(edx.wrapping_mul(4)).wrapping_sub(0x54),
                        fpu.fstp_m32(),
                    ); // 00e5e747
                    pc = 0x00e5e7a0; // 00e5e74b
                    continue;
                }
                0x00e5e74d => {
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x1))); // 00e5e74d
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e752
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5e755
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e757
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5e75b
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5e762
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e766
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e76a
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5e76e
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x58),
                        fpu.fstp_m32(),
                    ); // 00e5e771
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x2))); // 00e5e775
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5e77a
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5e77d
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e77f
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5e783
                    {
                        let r = fl.add(eax, 0x00000003u32, 32);
                        eax = r;
                    } // 00e5e78a
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5e78d
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5e791
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e795
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5e799
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x48),
                        fpu.fstp_m32(),
                    ); // 00e5e79c
                    pc = 0x00e5e7a0;
                }
                0x00e5e7a0 => {
                    ecx = u32::from(e.mem.u8(eax.wrapping_add(esi))); // 00e5e7a0
                    fl.sub(ecx, 0x00000026u32, 32); // 00e5e7a4
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5e7a7
                    if fl.sf == fl.of {
                        pc = 0x00e5e6da;
                        continue;
                    } // 00e5e7ab
                    pc = 0x00e5e7b1;
                }
                0x00e5e7b1 => {
                    x[1] = load_vec(e, esp.wrapping_add(0x50)); // 00e5e7b1
                    x[2] = load_vec(e, esp.wrapping_add(0x40)); // 00e5e7b6
                    {
                        let r = fl.add(eax, esi, 32);
                        eax = r;
                    } // 00e5e7bb
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax); // 00e5e7bd
                    eax = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e7c1
                    {
                        let r = fl.add(eax, 0x00000020u32, 32);
                        eax = r;
                    } // 00e5e7c5
                    edx = eax.wrapping_add(0x20); // 00e5e7c8
                    eax = e.mem.u32(eax.wrapping_add(0x10)); // 00e5e7cb
                    x[0] = x[1]; // 00e5e7ce
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e5e7d1
                    e.mem.set_u32(esp.wrapping_add(0x24), 0x0000000fu32); // 00e5e7d4
                    e.mem.set_u32(esp.wrapping_add(0x20), edx); // 00e5e7dc
                    x[0] = addps(x[0], x[2]); // 00e5e7e0
                    x[1] = subps(x[1], x[2]); // 00e5e7e3
                    e.mem.set_u32(esp.wrapping_add(0x14), eax); // 00e5e7e6
                    if fl.sf {
                        pc = 0x00e5e8cc;
                        continue;
                    } // 00e5e7ea
                    x[4] = x[0]; // 00e5e7f0
                    x[5] = x[0]; // 00e5e7f3
                    x[4] = shufps(x[4], x[0], 0x55); // 00e5e7f6
                    x[5] = shufps(x[5], x[0], 0x0); // 00e5e7fa
                    x[2] = x[1]; // 00e5e7fe
                    x[3] = x[1]; // 00e5e801
                    x[0] = shufps(x[0], x[0], 0xaa); // 00e5e804
                    x[2] = shufps(x[2], x[1], 0x55); // 00e5e808
                    x[3] = shufps(x[3], x[1], 0x0); // 00e5e80c
                    store_vec(e, esp.wrapping_add(0x90), x[0]); // 00e5e810
                    x[1] = shufps(x[1], x[1], 0xaa); // 00e5e818
                    x[0] = xorps(x[0], x[0]); // 00e5e81c
                    store_vec(e, esp.wrapping_add(0x80), x[1]); // 00e5e81f
                    store_vec(e, esp.wrapping_add(0x70), x[0]); // 00e5e827
                    ecx = edx.wrapping_add(0x220); // 00e5e82c
                    pc = 0x00e5e832;
                }
                0x00e5e832 => {
                    x[1] = load_vec(e, ecx.wrapping_sub(0x20)); // 00e5e832
                    x[0] = load_vec(e, ecx.wrapping_sub(0x10)); // 00e5e836
                    x[6] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5e83a
                    x[7] = load_vec(e, ecx.wrapping_sub(0x1f0)); // 00e5e83e
                    x[0] = mulps(x[0], x[2]); // 00e5e845
                    x[1] = mulps(x[1], x[3]); // 00e5e848
                    x[1] = addps(x[1], x[0]); // 00e5e84b
                    x[0] = load_vec(e, ecx); // 00e5e84e
                    x[0] = mulps(x[0], load_vec(e, esp.wrapping_add(0x80))); // 00e5e851
                    x[0] = addps(x[0], x[6]); // 00e5e859
                    x[6] = load_vec(e, edx); // 00e5e85c
                    x[1] = addps(x[1], x[0]); // 00e5e85f
                    x[0] = load_vec(e, ecx.wrapping_sub(0x210)); // 00e5e862
                    x[6] = mulps(x[6], x[5]); // 00e5e869
                    x[0] = mulps(x[0], x[4]); // 00e5e86c
                    x[0] = addps(x[0], x[6]); // 00e5e86f
                    x[6] = load_vec(e, ecx.wrapping_sub(0x200)); // 00e5e872
                    x[6] = mulps(x[6], load_vec(e, esp.wrapping_add(0x90))); // 00e5e879
                    x[6] = addps(x[6], x[7]); // 00e5e881
                    x[0] = addps(x[0], x[6]); // 00e5e884
                    x[6] = load_vec(e, esp.wrapping_add(0x70)); // 00e5e887
                    x[7] = x[0]; // 00e5e88c
                    x[7] = subps(x[7], x[1]); // 00e5e88f
                    x[7] = cmpltps(x[7], x[6]); // 00e5e892
                    edx = movmskps(x[7]); // 00e5e896
                    fl.sub(edx, 0x0000000fu32, 32); // 00e5e899
                    if !fl.zf {
                        pc = 0x00e5e8fa;
                        continue;
                    } // 00e5e89c
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5e89e
                    x[0] = addps(x[0], x[1]); // 00e5e8a2
                    x[0] = cmpltps(x[0], x[6]); // 00e5e8a5
                    eax = movmskps(x[0]); // 00e5e8a9
                    {
                        let r = fl.and(e.mem.u32(esp.wrapping_add(0x24)), eax, 32);
                        e.mem.set_u32(esp.wrapping_add(0x24), r);
                    } // 00e5e8ac
                    {
                        let r = fl.add(edx, 0x00000040u32, 32);
                        edx = r;
                    } // 00e5e8b0
                    {
                        let r = fl.add(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5e8b3
                    {
                        let r = fl.sub(e.mem.u32(esp.wrapping_add(0x14)), 0x00000001u32, 32);
                        e.mem.set_u32(esp.wrapping_add(0x14), r);
                    } // 00e5e8b6
                    e.mem.set_u32(esp.wrapping_add(0x20), edx); // 00e5e8bb
                    if !fl.sf {
                        pc = 0x00e5e832;
                        continue;
                    } // 00e5e8bf
                    fl.sub(e.mem.u32(esp.wrapping_add(0x24)), 0x0000000fu32, 32); // 00e5e8c5
                    if !fl.zf {
                        pc = 0x00e5e904;
                        continue;
                    } // 00e5e8ca
                    pc = 0x00e5e8cc;
                }
                0x00e5e8cc => {
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e8cc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5e8d0
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e8d1
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5e8d5
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5e8d6
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66600, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5e8d9
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5e8de
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e8e2
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5e8e6
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5e8e7
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5e8e8
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66290, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5e8eb
                    e.mem.set_u32(esp.wrapping_add(0x20), 0x00000001u32); // 00e5e8f0
                    pc = 0x00e5e90c; // 00e5e8f8
                    continue;
                }
                0x00e5e8fa => {
                    e.mem.set_u32(esp.wrapping_add(0x20), 0x00000000u32); // 00e5e8fa
                    pc = 0x00e5e90c; // 00e5e902
                    continue;
                }
                0x00e5e904 => {
                    e.mem.set_u32(esp.wrapping_add(0x20), 0x00000002u32); // 00e5e904
                    pc = 0x00e5e90c;
                }
                0x00e5e90c => {
                    eax = e.mem.u32(esp.wrapping_add(0x18)); // 00e5e90c
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5e910
                    x[1] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5e913
                    x[2] = load_vec(e, ecx); // 00e5e917
                    {
                        let r = fl.add(eax, 0x00000020u32, 32);
                        eax = r;
                    } // 00e5e91a
                    edx = eax.wrapping_add(0x20); // 00e5e91d
                    eax = e.mem.u32(eax.wrapping_add(0x10)); // 00e5e920
                    {
                        let r = fl.add(ebx, esi, 32);
                        ebx = r;
                    } // 00e5e923
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e5e925
                    x[0] = x[1]; // 00e5e928
                    e.mem.set_u32(esp.wrapping_add(0x24), ebx); // 00e5e92b
                    esi = 0x0000000fu32; // 00e5e92f
                    x[0] = addps(x[0], x[2]); // 00e5e934
                    x[1] = subps(x[1], x[2]); // 00e5e937
                    if fl.sf {
                        pc = 0x00e5ea12;
                        continue;
                    } // 00e5e93a
                    x[4] = x[0]; // 00e5e940
                    x[5] = x[0]; // 00e5e943
                    x[4] = shufps(x[4], x[0], 0x55); // 00e5e946
                    x[5] = shufps(x[5], x[0], 0x0); // 00e5e94a
                    x[2] = x[1]; // 00e5e94e
                    x[3] = x[1]; // 00e5e951
                    x[0] = shufps(x[0], x[0], 0xaa); // 00e5e954
                    x[2] = shufps(x[2], x[1], 0x55); // 00e5e958
                    x[3] = shufps(x[3], x[1], 0x0); // 00e5e95c
                    store_vec(e, esp.wrapping_add(0x80), x[0]); // 00e5e960
                    x[1] = shufps(x[1], x[1], 0xaa); // 00e5e968
                    x[0] = xorps(x[0], x[0]); // 00e5e96c
                    store_vec(e, esp.wrapping_add(0x90), x[1]); // 00e5e96f
                    store_vec(e, esp.wrapping_add(0x70), x[0]); // 00e5e977
                    ecx = edx.wrapping_add(0x220); // 00e5e97c
                    pc = 0x00e5e982;
                }
                0x00e5e982 => {
                    x[1] = load_vec(e, ecx.wrapping_sub(0x20)); // 00e5e982
                    x[0] = load_vec(e, ecx.wrapping_sub(0x10)); // 00e5e986
                    x[6] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5e98a
                    x[7] = load_vec(e, ecx.wrapping_sub(0x1f0)); // 00e5e98e
                    x[0] = mulps(x[0], x[2]); // 00e5e995
                    x[1] = mulps(x[1], x[3]); // 00e5e998
                    x[1] = addps(x[1], x[0]); // 00e5e99b
                    x[0] = load_vec(e, ecx); // 00e5e99e
                    x[0] = mulps(x[0], load_vec(e, esp.wrapping_add(0x90))); // 00e5e9a1
                    x[0] = addps(x[0], x[6]); // 00e5e9a9
                    x[6] = load_vec(e, edx); // 00e5e9ac
                    x[1] = addps(x[1], x[0]); // 00e5e9af
                    x[0] = load_vec(e, ecx.wrapping_sub(0x210)); // 00e5e9b2
                    x[6] = mulps(x[6], x[5]); // 00e5e9b9
                    x[0] = mulps(x[0], x[4]); // 00e5e9bc
                    x[0] = addps(x[0], x[6]); // 00e5e9bf
                    x[6] = load_vec(e, ecx.wrapping_sub(0x200)); // 00e5e9c2
                    x[6] = mulps(x[6], load_vec(e, esp.wrapping_add(0x80))); // 00e5e9c9
                    x[6] = addps(x[6], x[7]); // 00e5e9d1
                    x[0] = addps(x[0], x[6]); // 00e5e9d4
                    x[6] = load_vec(e, esp.wrapping_add(0x70)); // 00e5e9d7
                    x[7] = x[0]; // 00e5e9dc
                    x[7] = subps(x[7], x[1]); // 00e5e9df
                    x[7] = cmpltps(x[7], x[6]); // 00e5e9e2
                    ebx = movmskps(x[7]); // 00e5e9e6
                    fl.sub(ebx, 0x0000000fu32, 32); // 00e5e9e9
                    if !fl.zf {
                        pc = 0x00e5ea35;
                        continue;
                    } // 00e5e9ec
                    x[0] = addps(x[0], x[1]); // 00e5e9ee
                    x[0] = cmpltps(x[0], x[6]); // 00e5e9f1
                    ebx = movmskps(x[0]); // 00e5e9f5
                    {
                        let r = fl.and(esi, ebx, 32);
                        esi = r;
                    } // 00e5e9f8
                    {
                        let r = fl.add(edx, 0x00000040u32, 32);
                        edx = r;
                    } // 00e5e9fa
                    {
                        let r = fl.add(ecx, 0x00000040u32, 32);
                        ecx = r;
                    } // 00e5e9fd
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e5ea00
                    if !fl.sf {
                        pc = 0x00e5e982;
                        continue;
                    } // 00e5ea03
                    fl.sub(esi, 0x0000000fu32, 32); // 00e5ea09
                    if !fl.zf {
                        pc = 0x00e5ea39;
                        continue;
                    } // 00e5ea0c
                    ebx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5ea0e
                    pc = 0x00e5ea12;
                }
                0x00e5ea12 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5ea12
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e5ea16
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5ea17
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5ea18
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66600, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5ea1b
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5ea20
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e5ea24
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5ea25
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5ea26
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66290, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5ea29
                    eax = 0x00000001u32; // 00e5ea2e
                    pc = 0x00e5ea42; // 00e5ea33
                    continue;
                }
                0x00e5ea35 => {
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    } // 00e5ea35
                    pc = 0x00e5ea3e; // 00e5ea37
                    continue;
                }
                0x00e5ea39 => {
                    eax = 0x00000002u32; // 00e5ea39
                    pc = 0x00e5ea3e;
                }
                0x00e5ea3e => {
                    ebx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5ea3e
                    pc = 0x00e5ea42;
                }
                0x00e5ea42 => {
                    fl.sub(e.mem.u32(esp.wrapping_add(0x20)), 0x00000002u32, 32); // 00e5ea42
                    if fl.zf {
                        pc = 0x00e5ea59;
                        continue;
                    } // 00e5ea47
                    fl.sub(eax, 0x00000002u32, 32); // 00e5ea49
                    if !fl.zf {
                        pc = 0x00e5ef92;
                        continue;
                    } // 00e5ea4c
                    esi = ebx; // 00e5ea52
                    pc = 0x00e5e450; // 00e5ea54
                    continue;
                }
                0x00e5ea59 => {
                    fl.sub(eax, 0x00000002u32, 32); // 00e5ea59
                    if fl.zf {
                        pc = 0x00e5ea7b;
                        continue;
                    } // 00e5ea5c
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5ea5e
                    x[0] = load_vec(e, esp.wrapping_add(0x40)); // 00e5ea61
                    esi = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5ea66
                    store_vec(e, eax, x[0]); // 00e5ea6a
                    x[0] = load_vec(e, esp.wrapping_add(0x50)); // 00e5ea6d
                    store_vec(e, eax.wrapping_add(0x10), x[0]); // 00e5ea72
                    pc = 0x00e5e450; // 00e5ea76
                    continue;
                }
                0x00e5ea7b => {
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5ea7b
                    eax = esp.wrapping_add(0x40); // 00e5ea7f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5ea83
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e5ea84
                    ecx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5ea85
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5ea89
                    {
                        fn_00e5e430(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5ea8a
                    esi = ebx; // 00e5ea8f
                    pc = 0x00e5e450; // 00e5ea91
                    continue;
                }
                0x00e5ea96 => {
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ea96
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5ea9a
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5ea9e
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5eaa1
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5eaa3
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5eaa7
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5eaaa
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5eaac
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5eaaf
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5eab3
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5eab7
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5eabb
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0xa4),
                        fpu.fstp_m32(),
                    ); // 00e5eabe
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5eac5
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x5))); // 00e5eac9
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5eacd
                    {
                        let r = fl.add(edx, ebx, 32);
                        edx = r;
                    } // 00e5ead0
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x6))); // 00e5ead2
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5ead6
                    {
                        let r = fl.add(edx, ebx, 32);
                        edx = r;
                    } // 00e5ead9
                    ebx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5eadb
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5eadf
                    {
                        let r = fl.add(esi, 0x00000007u32, 32);
                        esi = r;
                    } // 00e5eae3
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5eae6
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5eaea
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x94),
                        fpu.fstp_m32(),
                    ); // 00e5eaed
                    pc = 0x00e5eb4a; // 00e5eaf4
                    continue;
                }
                0x00e5eaf6 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5eaf6
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5eafa
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5eafd
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5eaff
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5eb03
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5eb0a
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5eb0e
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5eb12
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5eb15
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x98),
                        fpu.fstp_m32(),
                    ); // 00e5eb18
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5eb1f
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5eb23
                    {
                        let r = fl.shl(ebx, ((ecx & 0xff) & 0x1f), 32);
                        ebx = r;
                    } // 00e5eb26
                    ecx = edx; // 00e5eb28
                    {
                        let r = fl.add(
                            ebx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        ebx = r;
                    } // 00e5eb2a
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5eb31
                    e.mem.set_u32(esp.wrapping_add(0x14), ebx); // 00e5eb34
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x14))); // 00e5eb38
                    ebx = e.mem.u32(esp.wrapping_add(0x18)); // 00e5eb3c
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5eb40
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x88),
                        fpu.fstp_m32(),
                    ); // 00e5eb43
                    pc = 0x00e5eb4a;
                }
                0x00e5eb4a => {
                    eax = (eax & 0xffff_ff00) | ((u32::from(e.mem.u8(esi))) & 0xff); // 00e5eb4a
                    fl.sub((eax & 0xff), 0x00000026u32, 8); // 00e5eb4c
                    if fl.cf {
                        pc = 0x00e5eb5b;
                        continue;
                    } // 00e5eb4e
                    fl.sub((eax & 0xff), 0x0000002bu32, 8); // 00e5eb50
                    if fl.cf || fl.zf {
                        pc = 0x00e5e450;
                        continue;
                    } // 00e5eb52
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5eb58
                    pc = 0x00e5eb5b;
                }
                0x00e5eb5b => {
                    x[1] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5eb5b
                    x[2] = load_vec(e, ecx); // 00e5eb5f
                    ecx = e.mem.u32(ebx.wrapping_add(0x30)); // 00e5eb62
                    {
                        let r = fl.add(ebx, 0x00000020u32, 32);
                        ebx = r;
                    } // 00e5eb65
                    {
                        let r = fl.sub(ecx, 0x00000001u32, 32);
                        ecx = r;
                    } // 00e5eb68
                    edx = ebx.wrapping_add(0x20); // 00e5eb6b
                    x[0] = x[1]; // 00e5eb6e
                    e.mem.set_u32(esp.wrapping_add(0x24), ebx); // 00e5eb71
                    e.mem.set_u32(esp.wrapping_add(0x20), 0x0000000fu32); // 00e5eb75
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5eb7d
                    x[0] = addps(x[0], x[2]); // 00e5eb81
                    x[1] = subps(x[1], x[2]); // 00e5eb84
                    if fl.sf {
                        pc = 0x00e5ec76;
                        continue;
                    } // 00e5eb87
                    x[4] = x[0]; // 00e5eb8d
                    x[5] = x[0]; // 00e5eb90
                    x[4] = shufps(x[4], x[0], 0x55); // 00e5eb93
                    x[5] = shufps(x[5], x[0], 0x0); // 00e5eb97
                    x[2] = x[1]; // 00e5eb9b
                    x[3] = x[1]; // 00e5eb9e
                    x[0] = shufps(x[0], x[0], 0xaa); // 00e5eba1
                    x[2] = shufps(x[2], x[1], 0x55); // 00e5eba5
                    x[3] = shufps(x[3], x[1], 0x0); // 00e5eba9
                    store_vec(e, esp.wrapping_add(0x80), x[0]); // 00e5ebad
                    x[1] = shufps(x[1], x[1], 0xaa); // 00e5ebb5
                    x[0] = xorps(x[0], x[0]); // 00e5ebb9
                    store_vec(e, esp.wrapping_add(0x90), x[1]); // 00e5ebbc
                    store_vec(e, esp.wrapping_add(0x70), x[0]); // 00e5ebc4
                    eax = edx.wrapping_add(0x220); // 00e5ebc9
                    pc = 0x00e5ebcf;
                }
                0x00e5ebcf => {
                    x[1] = load_vec(e, eax.wrapping_sub(0x20)); // 00e5ebcf
                    x[0] = load_vec(e, eax.wrapping_sub(0x10)); // 00e5ebd3
                    x[6] = load_vec(e, eax.wrapping_add(0x10)); // 00e5ebd7
                    x[7] = load_vec(e, edx); // 00e5ebdb
                    x[0] = mulps(x[0], x[2]); // 00e5ebde
                    x[1] = mulps(x[1], x[3]); // 00e5ebe1
                    x[1] = addps(x[1], x[0]); // 00e5ebe4
                    x[0] = load_vec(e, eax); // 00e5ebe7
                    x[0] = mulps(x[0], load_vec(e, esp.wrapping_add(0x90))); // 00e5ebea
                    x[0] = addps(x[0], x[6]); // 00e5ebf2
                    x[1] = addps(x[1], x[0]); // 00e5ebf5
                    x[0] = load_vec(e, eax.wrapping_sub(0x210)); // 00e5ebf8
                    x[0] = mulps(x[0], x[4]); // 00e5ebff
                    x[6] = x[5]; // 00e5ec02
                    x[6] = mulps(x[6], x[7]); // 00e5ec05
                    x[7] = load_vec(e, eax.wrapping_sub(0x1f0)); // 00e5ec08
                    x[0] = addps(x[0], x[6]); // 00e5ec0f
                    x[6] = load_vec(e, eax.wrapping_sub(0x200)); // 00e5ec12
                    x[6] = mulps(x[6], load_vec(e, esp.wrapping_add(0x80))); // 00e5ec19
                    x[6] = addps(x[6], x[7]); // 00e5ec21
                    x[0] = addps(x[0], x[6]); // 00e5ec24
                    x[6] = load_vec(e, esp.wrapping_add(0x70)); // 00e5ec27
                    x[7] = x[0]; // 00e5ec2c
                    x[7] = subps(x[7], x[1]); // 00e5ec2f
                    x[7] = cmpltps(x[7], x[6]); // 00e5ec32
                    edx = movmskps(x[7]); // 00e5ec36
                    fl.sub(edx, 0x0000000fu32, 32); // 00e5ec39
                    if !fl.zf {
                        pc = 0x00e5ef92;
                        continue;
                    } // 00e5ec3c
                    x[0] = addps(x[0], x[1]); // 00e5ec42
                    x[0] = cmpltps(x[0], x[6]); // 00e5ec45
                    edx = movmskps(x[0]); // 00e5ec49
                    {
                        let r = fl.and(e.mem.u32(esp.wrapping_add(0x20)), edx, 32);
                        e.mem.set_u32(esp.wrapping_add(0x20), r);
                    } // 00e5ec4c
                    edx = e.mem.u32(esp.wrapping_add(0x14)); // 00e5ec50
                    {
                        let r = fl.add(edx, 0x00000040u32, 32);
                        edx = r;
                    } // 00e5ec54
                    {
                        let r = fl.add(eax, 0x00000040u32, 32);
                        eax = r;
                    } // 00e5ec57
                    {
                        let r = fl.sub(ecx, 0x00000001u32, 32);
                        ecx = r;
                    } // 00e5ec5a
                    e.mem.set_u32(esp.wrapping_add(0x14), edx); // 00e5ec5d
                    if !fl.sf {
                        pc = 0x00e5ebcf;
                        continue;
                    } // 00e5ec61
                    fl.sub(e.mem.u32(esp.wrapping_add(0x20)), 0x0000000fu32, 32); // 00e5ec67
                    if !fl.zf {
                        pc = 0x00e5e450;
                        continue;
                    } // 00e5ec6c
                    ebx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5ec72
                    pc = 0x00e5ec76;
                }
                0x00e5ec76 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5ec76
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5ec77
                    ecx = ebx; // 00e5ec78
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66600, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5ec7a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5ec7f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5ec80
                    ecx = ebx; // 00e5ec81
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66290, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5ec83
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5ec88
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5ec89
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5ec8a
                    esp = ebp; // 00e5ec8b
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5ec8d
                    return; // 00e5ec8e
                }
                0x00e5ec91 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ec91
                    esi = esi.wrapping_add(eax).wrapping_add(0x2); // 00e5ec95
                    pc = 0x00e5e450; // 00e5ec99
                    continue;
                }
                0x00e5ec9e => {
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ec9e
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5eca2
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5eca6
                    {
                        let r = fl.add(edx, esi, 32);
                        edx = r;
                    } // 00e5eca9
                    esi = edx.wrapping_add(ecx).wrapping_add(0x3); // 00e5ecab
                    pc = 0x00e5e450; // 00e5ecaf
                    continue;
                }
                0x00e5ecb4 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ecb4
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5ecb8
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5ecbc
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5ecc0
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5ecc3
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5ecc5
                    {
                        let r = fl.add(edx, esi, 32);
                        edx = r;
                    } // 00e5ecc8
                    esi = edx.wrapping_add(eax).wrapping_add(0x4); // 00e5ecca
                    pc = 0x00e5e450; // 00e5ecce
                    continue;
                }
                0x00e5ecd3 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ecd3
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5ecd7
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5ecda
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0x8)), 32);
                        edx = r;
                    } // 00e5ecdc
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e5ecdf
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5ece3
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5ece7
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5ecea
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0xc)), 32);
                        edx = r;
                    } // 00e5ecec
                    e.mem.set_u32(esp.wrapping_add(0x34), edx); // 00e5ecef
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5ecf3
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5ecf7
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5ecfa
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0x10)), 32);
                        edx = r;
                    } // 00e5ecfc
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e5ecff
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5ed03
                    {
                        let r = fl.sub(ecx, eax, 32);
                        ecx = r;
                    } // 00e5ed06
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e5ed08
                    edx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5ed0c
                    e.mem.set_u32(esp.wrapping_add(0x2c), edx); // 00e5ed0f
                    eax = e.mem.u32(edi); // 00e5ed13
                    e.mem.set_u32(esp.wrapping_add(0x28), eax); // 00e5ed15
                    edi = esp.wrapping_add(0x28); // 00e5ed19
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5ed1d
                    pc = 0x00e5e450; // 00e5ed20
                    continue;
                }
                0x00e5ed25 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ed25
                    ecx = esp.wrapping_add(0x28); // 00e5ed29
                    fl.sub(edi, ecx, 32); // 00e5ed2d
                    if fl.zf {
                        pc = 0x00e5ed5e;
                        continue;
                    } // 00e5ed2f
                    edx = e.mem.u32(edi); // 00e5ed31
                    e.mem.set_u32(esp.wrapping_add(0x28), edx); // 00e5ed33
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5ed37
                    e.mem.set_u32(esp.wrapping_add(0x2c), ecx); // 00e5ed3a
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5ed3e
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e5ed41
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5ed45
                    e.mem.set_u32(esp.wrapping_add(0x34), ecx); // 00e5ed48
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5ed4c
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e5ed4f
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5ed53
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e5ed56
                    edi = esp.wrapping_add(0x28); // 00e5ed5a
                    pc = 0x00e5ed5e;
                }
                0x00e5ed5e => {
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x28)), eax, 32);
                        e.mem.set_u32(esp.wrapping_add(0x28), r);
                    } // 00e5ed5e
                    {
                        let r = fl.add(esi, 0x00000002u32, 32);
                        esi = r;
                    } // 00e5ed62
                    pc = 0x00e5e450; // 00e5ed65
                    continue;
                }
                0x00e5ed6a => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ed6a
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5ed6e
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5ed72
                    ecx = esp.wrapping_add(0x28); // 00e5ed75
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5ed79
                    fl.sub(edi, ecx, 32); // 00e5ed7b
                    if fl.zf {
                        pc = 0x00e5edac;
                        continue;
                    } // 00e5ed7d
                    edx = e.mem.u32(edi); // 00e5ed7f
                    e.mem.set_u32(esp.wrapping_add(0x28), edx); // 00e5ed81
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5ed85
                    e.mem.set_u32(esp.wrapping_add(0x2c), ecx); // 00e5ed88
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5ed8c
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e5ed8f
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5ed93
                    e.mem.set_u32(esp.wrapping_add(0x34), ecx); // 00e5ed96
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5ed9a
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e5ed9d
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5eda1
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e5eda4
                    edi = esp.wrapping_add(0x28); // 00e5eda8
                    pc = 0x00e5edac;
                }
                0x00e5edac => {
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x28)), eax, 32);
                        e.mem.set_u32(esp.wrapping_add(0x28), r);
                    } // 00e5edac
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5edb0
                    pc = 0x00e5e450; // 00e5edb3
                    continue;
                }
                0x00e5edb8 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5edb8
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5edbc
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5edc0
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5edc4
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5edc7
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5edc9
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5edcd
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5edd0
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5edd2
                    ecx = esp.wrapping_add(0x28); // 00e5edd5
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5edd9
                    fl.sub(edi, ecx, 32); // 00e5eddb
                    if fl.zf {
                        pc = 0x00e5ee0c;
                        continue;
                    } // 00e5eddd
                    edx = e.mem.u32(edi); // 00e5eddf
                    e.mem.set_u32(esp.wrapping_add(0x28), edx); // 00e5ede1
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5ede5
                    e.mem.set_u32(esp.wrapping_add(0x2c), ecx); // 00e5ede8
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5edec
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e5edef
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5edf3
                    e.mem.set_u32(esp.wrapping_add(0x34), ecx); // 00e5edf6
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5edfa
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e5edfd
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5ee01
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e5ee04
                    edi = esp.wrapping_add(0x28); // 00e5ee08
                    pc = 0x00e5ee0c;
                }
                0x00e5ee0c => {
                    e.mem.set_u32(esp.wrapping_add(0x28), eax); // 00e5ee0c
                    {
                        let r = fl.add(esi, 0x00000005u32, 32);
                        esi = r;
                    } // 00e5ee10
                    pc = 0x00e5e450; // 00e5ee13
                    continue;
                }
                0x00e5ee18 => {
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ee18
                    {
                        let r = fl.add(esi, 0x00000002u32, 32);
                        esi = r;
                    } // 00e5ee1c
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x154),
                        ecx,
                    ); // 00e5ee1f
                    pc = 0x00e5ee6a; // 00e5ee26
                    continue;
                }
                0x00e5ee28 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ee28
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5ee2c
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5ee30
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5ee33
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x164),
                        edx,
                    ); // 00e5ee35
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5ee3c
                    pc = 0x00e5ee6a; // 00e5ee3f
                    continue;
                }
                0x00e5ee41 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ee41
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5ee45
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5ee49
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5ee4c
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5ee4e
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5ee52
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5ee55
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5ee57
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5ee5b
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5ee5e
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x174),
                        edx,
                    ); // 00e5ee60
                    {
                        let r = fl.add(esi, 0x00000005u32, 32);
                        esi = r;
                    } // 00e5ee67
                    pc = 0x00e5ee6a;
                }
                0x00e5ee6a => {
                    eax = e.mem.u32(esp.wrapping_add(0x2c)); // 00e5ee6a
                    edx = esp.wrapping_add(0x28); // 00e5ee6e
                    fl.sub(edi, edx, 32); // 00e5ee72
                    if fl.zf {
                        pc = 0x00e5eea3;
                        continue;
                    } // 00e5ee74
                    ecx = e.mem.u32(edi); // 00e5ee76
                    e.mem.set_u32(esp.wrapping_add(0x28), ecx); // 00e5ee78
                    edx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5ee7c
                    e.mem.set_u32(esp.wrapping_add(0x2c), edx); // 00e5ee7f
                    ecx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5ee83
                    e.mem.set_u32(esp.wrapping_add(0x30), ecx); // 00e5ee86
                    edx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5ee8a
                    e.mem.set_u32(esp.wrapping_add(0x34), edx); // 00e5ee8d
                    ecx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5ee91
                    e.mem.set_u32(esp.wrapping_add(0x38), ecx); // 00e5ee94
                    edx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5ee98
                    e.mem.set_u32(esp.wrapping_add(0x3c), edx); // 00e5ee9b
                    edi = esp.wrapping_add(0x28); // 00e5ee9f
                    pc = 0x00e5eea3;
                }
                0x00e5eea3 => {
                    e.mem.set_u32(esp.wrapping_add(0x2c), eax); // 00e5eea3
                    pc = 0x00e5e450; // 00e5eea7
                    continue;
                }
                0x00e5eeac => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000200u32); // 00e5eeac
                    eax = esp.wrapping_add(0xa4); // 00e5eeb1
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5eeb8
                    ecx = esp.wrapping_add(0x6c); // 00e5eeb9
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00cb1600, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5eebd
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x010d5100u32); // 00e5eec2
                    ecx = esp.wrapping_add(0x68); // 00e5eec7
                    {
                        let w = [ecx, e.mem.u32(esp)];
                        let r = e.call(0x00cc01c0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(4); // 00e5eecb
                    ecx = e.mem.u32(0x01268360u32); // 00e5eed0
                    edx = e.mem.u32(ecx); // 00e5eed6
                    edx = e.mem.u32(edx.wrapping_add(0xc)); // 00e5eed8
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x0000012du32); // 00e5eedb
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x010ecd88u32); // 00e5eee0
                    eax = esp.wrapping_add(0xa8); // 00e5eee5
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5eeec
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x1298feddu32); // 00e5eeed
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000003u32); // 00e5eef2
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                            e.mem.u32(esp.wrapping_add(16)),
                        ];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(20); // 00e5eef4
                    fl.and(eax, eax, 32); // 00e5eef6
                    if fl.zf {
                        pc = 0x00e5eefb;
                        continue;
                    } // 00e5eef8
                    panic!("INT3 at 00e5eefa"); // 00e5eefa
                }
                0x00e5eefb => {
                    ecx = esp.wrapping_add(0x64); // 00e5eefb
                    {
                        let w = [ecx];
                        let r = e.call(0x00cc00e0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5eeff
                    pc = 0x00e5e450; // 00e5ef04
                    continue;
                }
                0x00e5ef09 => {
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ef09
                    pc = 0x00e5ef5a; // 00e5ef0d
                    continue;
                }
                0x00e5ef0f => {
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ef0f
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5ef13
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5ef17
                    {
                        let r = fl.add(ebx, eax, 32);
                        ebx = r;
                    } // 00e5ef1a
                    pc = 0x00e5ef5a; // 00e5ef1c
                    continue;
                }
                0x00e5ef1e => {
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ef1e
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5ef22
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5ef26
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5ef2a
                    {
                        let r = fl.add(ebx, ecx, 32);
                        ebx = r;
                    } // 00e5ef2d
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5ef2f
                    {
                        let r = fl.add(ebx, edx, 32);
                        ebx = r;
                    } // 00e5ef32
                    pc = 0x00e5ef5a; // 00e5ef34
                    continue;
                }
                0x00e5ef36 => {
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5ef36
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5ef3a
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5ef3e
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5ef42
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5ef46
                    {
                        let r = fl.add(ebx, eax, 32);
                        ebx = r;
                    } // 00e5ef49
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5ef4b
                    {
                        let r = fl.add(ebx, ecx, 32);
                        ebx = r;
                    } // 00e5ef4e
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5ef50
                    {
                        let r = fl.add(ebx, edx, 32);
                        ebx = r;
                    } // 00e5ef53
                    pc = 0x00e5ef5a; // 00e5ef55
                    continue;
                }
                0x00e5ef57 => {
                    ebx = eax.wrapping_sub(0x30); // 00e5ef57
                    pc = 0x00e5ef5a;
                }
                0x00e5ef5a => {
                    eax = e.mem.u32(esp.wrapping_add(0x18)); // 00e5ef5a
                    {
                        let r = fl.add(ebx, e.mem.u32(edi), 32);
                        ebx = r;
                    } // 00e5ef5e
                    esi = e.mem.u32(eax); // 00e5ef60
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5ef62
                    ecx = e.mem.u32(esi.wrapping_add(0x8)); // 00e5ef67
                    {
                        let r = fl.and(ecx, 0x3fffffffu32, 32);
                        ecx = r;
                    } // 00e5ef6a
                    fl.sub(e.mem.u32(esi.wrapping_add(0x4)), ecx, 32); // 00e5ef70
                    if !fl.zf {
                        pc = 0x00e5ef84;
                        continue;
                    } // 00e5ef73
                    edx = e.mem.u32(eax.wrapping_add(0x10)); // 00e5ef75
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000004u32); // 00e5ef78
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5ef7a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5ef7b
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00c90940, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5ef7c
                    {
                        let r = fl.add(esp, 0x0000000cu32, 32);
                        esp = r;
                    } // 00e5ef81
                    pc = 0x00e5ef84;
                }
                0x00e5ef84 => {
                    eax = e.mem.u32(esi.wrapping_add(0x4)); // 00e5ef84
                    ecx = e.mem.u32(esi); // 00e5ef87
                    ecx = ecx.wrapping_add(eax.wrapping_mul(4)); // 00e5ef89
                    {
                        let r = fl.inc(eax, 32);
                        eax = r;
                    } // 00e5ef8c
                    e.mem.set_u32(esi.wrapping_add(0x4), eax); // 00e5ef8d
                    e.mem.set_u32(ecx, ebx); // 00e5ef90
                    pc = 0x00e5ef92;
                }
                0x00e5ef92 => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5ef92
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5ef93
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5ef94
                    esp = ebp; // 00e5ef95
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5ef97
                    return; // 00e5ef98
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5f070 (decompiled, FalloutNV.exe 1.4.0.525)
/// MOPP code interpreter (thiscall, `RET 0xc`: `this` the machine, `sink`, `code` (the address of the
/// current code byte) and `bounds` (the 32 bytes of query data)): each step dispatches on the code
/// byte (values up to 0x6b through a byte map and a 23-entry jump table written as `match`es
/// on the code byte and the case), tests the query against the node's split planes with SSE (`MULPS`, `ADDPS`,
/// `SHUFPS`, `CMPLTPS`, `MOVMSKPS`), and either descends by calling itself with the offset code
/// pointer, walks on, or reports hits through the machine's virtual methods (`00c85750`, `00c90940`
/// array growth, `00cb1600`, `00cc00e0`, `00cc01c0` and the `00e65ee0`-family callees). An opcode
/// the table does not know ends in an `INT3`, which stops here with a panic. Transcribed
/// instruction by instruction. The four variants of this interpreter differ in the data layout of
/// the query.
pub fn fn_00e5f070(e: &mut Engine, this: Ptr, sink: Ptr, code: Ptr, bounds: Ptr) {
    e.with_stack(0x500, |e, frame| {
        let mut esp = frame.addr() + 0x400;
        e.mem.set_u32(esp.wrapping_add(4), sink.addr());
        e.mem.set_u32(esp.wrapping_add(8), code.addr());
        e.mem.set_u32(esp.wrapping_add(12), bounds.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5f070;
        loop {
            match pc {
                0x00e5f070 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5f070
                    ebp = esp; // 00e5f071
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5f073
                    {
                        let r = fl.sub(esp, 0x00000284u32, 32);
                        esp = r;
                    } // 00e5f076
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e5f07c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5f07d
                    esi = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5f07e
                    ebx = ecx; // 00e5f081
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5f083
                    edi = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5f084
                    e.mem.set_u32(esp.wrapping_add(0x24), ebx); // 00e5f087
                    pc = 0x00e5f090; // 00e5f08b
                    continue;
                }
                0x00e5f090 => {
                    eax = u32::from(e.mem.u8(esi)); // 00e5f090
                    fl.sub(eax, 0x0000006bu32, 32); // 00e5f093
                    if !fl.cf && !fl.zf {
                        pc = 0x00e5f97d;
                        continue;
                    } // 00e5f096
                    ecx = match eax {
                        0 => 0,
                        1..=4 => 1,
                        5 => 2,
                        6 => 3,
                        7 => 4,
                        8 | 12..=15 | 29..=31 | 44..=47 | 84..=95 => 22,
                        9 => 5,
                        10 => 6,
                        11 => 7,
                        16..=18 => 8,
                        19..=28 => 9,
                        32..=34 => 10,
                        35..=37 => 11,
                        38..=40 => 12,
                        41..=43 => 13,
                        48..=79 => 14,
                        80 => 15,
                        81 => 16,
                        82 => 17,
                        83 => 18,
                        96..=99 => 19,
                        100..=103 => 20,
                        104..=107 => 21,
                        _ => unreachable!(),
                    }; // 00e5f09c
                    pc = match ecx {
                        0 => 0x00e5_fa79,
                        1 => 0x00e5_f7a4,
                        2 => 0x00e5_f762,
                        3 => 0x00e5_f76f,
                        4 => 0x00e5_f785,
                        5 => 0x00e5_f7f6,
                        6 => 0x00e5_f83b,
                        7 => 0x00e5_f889,
                        8 => 0x00e5_f0dd,
                        9 => 0x00e5_f0aa,
                        10 => 0x00e5_f145,
                        11 => 0x00e5_f1a3,
                        12 => 0x00e5_f652,
                        13 => 0x00e5_f5f7,
                        14 => 0x00e5_fa31,
                        15 => 0x00e5_f9e3,
                        16 => 0x00e5_f9e9,
                        17 => 0x00e5_f9f8,
                        18 => 0x00e5_fa10,
                        19 => 0x00e5_f8e9,
                        20 => 0x00e5_f8f9,
                        21 => 0x00e5_f912,
                        22 => 0x00e5_f97d,
                        _ => unreachable!(),
                    }; // 00e5f0a3
                    continue;
                }
                0x00e5f0aa => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5f0aa
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f0ae
                    x[0] = load_vec(e, eax); // 00e5f0b1
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5f0b4
                    x[0] = load_vec(e, eax.wrapping_add(0x10)); // 00e5f0b9
                    eax = esp.wrapping_add(0x40); // 00e5f0bd
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5f0c1
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5f0c2
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5f0c5
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5f0c6
                    ecx = ebx; // 00e5f0c7
                    e.mem.set_u32(esp.wrapping_add(0x2c), edx); // 00e5f0c9
                    store_vec(e, esp.wrapping_add(0x5c), x[0]); // 00e5f0cd
                    {
                        fn_00e5f070(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5f0d2
                    {
                        let r = fl.add(esi, e.mem.u32(esp.wrapping_add(0x20)), 32);
                        esi = r;
                    } // 00e5f0d7
                    pc = 0x00e5f090; // 00e5f0db
                    continue;
                }
                0x00e5f0dd => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f0dd
                    x[0] = load_vec(e, ecx); // 00e5f0e0
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5f0e3
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5f0e8
                    store_vec(e, esp.wrapping_add(0x50), x[0]); // 00e5f0ec
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f0f1
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f0f5
                    e.mem.set_u32(esp.wrapping_add(0x20), ecx); // 00e5f0f9
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f0fd
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f100
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f102
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x38)),
                            32,
                        );
                        edx = r;
                    } // 00e5f105
                    e.mem.set_u32(esp.wrapping_add(0x1c), edx); // 00e5f109
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x1c))); // 00e5f10d
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5f111
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5f115
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x30),
                        fpu.fstp_m32(),
                    ); // 00e5f118
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f11c
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f11f
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x38)),
                            32,
                        );
                        edx = r;
                    } // 00e5f121
                    e.mem.set_u32(esp.wrapping_add(0x1c), edx); // 00e5f125
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x1c))); // 00e5f129
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5f12d
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e5f130
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5f132
                    e.mem
                        .set_u32(esp.wrapping_add(eax.wrapping_mul(4)), fpu.fstp_m32()); // 00e5f135
                    edx = u32::from(e.mem.u8(esi.wrapping_sub(0x1))); // 00e5f138
                    e.mem.set_u32(esp.wrapping_add(0x20), edx); // 00e5f13c
                    pc = 0x00e5f223; // 00e5f140
                    continue;
                }
                0x00e5f145 => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f145
                    x[0] = load_vec(e, ecx); // 00e5f148
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5f14b
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5f150
                    store_vec(e, esp.wrapping_add(0x50), x[0]); // 00e5f154
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f159
                    ecx = edx.wrapping_add(0x1); // 00e5f15d
                    e.mem.set_u32(esp.wrapping_add(0x20), ecx); // 00e5f160
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f164
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f167
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x78)),
                            32,
                        );
                        edx = r;
                    } // 00e5f169
                    e.mem.set_u32(esp.wrapping_add(0x1c), edx); // 00e5f16d
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5f171
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x1c))); // 00e5f175
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f179
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5f17b
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x78)),
                            32,
                        );
                        edx = r;
                    } // 00e5f17e
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x40),
                        fpu.fstp_m32(),
                    ); // 00e5f182
                    e.mem.set_u32(esp.wrapping_add(0x1c), edx); // 00e5f186
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x1c))); // 00e5f18a
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f18e
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5f191
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e5f194
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5f196
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x70),
                        fpu.fstp_m32(),
                    ); // 00e5f199
                    eax = u32::from(e.mem.u8(esi.wrapping_sub(0x1))); // 00e5f19d
                    pc = 0x00e5f21d; // 00e5f1a1
                    continue;
                }
                0x00e5f1a3 => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f1a3
                    x[0] = load_vec(e, ecx); // 00e5f1a6
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5f1a9
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5f1ae
                    store_vec(e, esp.wrapping_add(0x50), x[0]); // 00e5f1b2
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f1b7
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f1bb
                    e.mem.set_u32(esp.wrapping_add(0x20), ecx); // 00e5f1bf
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f1c3
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f1c6
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f1c8
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x84)),
                            32,
                        );
                        edx = r;
                    } // 00e5f1cb
                    e.mem.set_u32(esp.wrapping_add(0x1c), edx); // 00e5f1d2
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x1c))); // 00e5f1d6
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5f1da
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5f1de
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x7c),
                        fpu.fstp_m32(),
                    ); // 00e5f1e1
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f1e5
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f1e8
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x84)),
                            32,
                        );
                        edx = r;
                    } // 00e5f1ea
                    e.mem.set_u32(esp.wrapping_add(0x1c), edx); // 00e5f1f1
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x1c))); // 00e5f1f5
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5f1f9
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x4c),
                        fpu.fstp_m32(),
                    ); // 00e5f1fc
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5f200
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x5))); // 00e5f204
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5f208
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x6))); // 00e5f20c
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5f210
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f213
                    {
                        let r = fl.add(ebx, edx, 32);
                        ebx = r;
                    } // 00e5f216
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5f218
                    {
                        let r = fl.add(esi, 0x00000007u32, 32);
                        esi = r;
                    } // 00e5f21a
                    pc = 0x00e5f21d;
                }
                0x00e5f21d => {
                    e.mem.set_u32(esp.wrapping_add(0x20), eax); // 00e5f21d
                    edx = eax; // 00e5f221
                    pc = 0x00e5f223;
                }
                0x00e5f223 => {
                    eax = u32::from(e.mem.u8(ebx.wrapping_add(esi))); // 00e5f223
                    fl.sub(eax, 0x00000026u32, 32); // 00e5f227
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax); // 00e5f22a
                    if fl.sf != fl.of {
                        pc = 0x00e5f327;
                        continue;
                    } // 00e5f22e
                    pc = 0x00e5f234;
                }
                0x00e5f234 => {
                    fl.sub(eax, 0x0000002bu32, 32); // 00e5f234
                    if !fl.zf && fl.sf == fl.of {
                        pc = 0x00e5f327;
                        continue;
                    } // 00e5f237
                    fl.sub(eax, 0x00000029u32, 32); // 00e5f23d
                    if fl.sf != fl.of {
                        pc = 0x00e5f2b3;
                        continue;
                    } // 00e5f240
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x1))); // 00e5f242
                    eax = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x2))); // 00e5f247
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5f24c
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5f24f
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x3))); // 00e5f251
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f256
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5f259
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f25b
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e5f25e
                    eax = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f262
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f266
                    fpu.fmul_m32(e.mem.u32(eax.wrapping_add(0x10))); // 00e5f26a
                    eax = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5f26d
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0xa4),
                        fpu.fstp_m32(),
                    ); // 00e5f271
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x4))); // 00e5f278
                    eax = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x5))); // 00e5f27d
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5f282
                    {
                        let r = fl.add(ecx, eax, 32);
                        ecx = r;
                    } // 00e5f285
                    eax = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x6))); // 00e5f287
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5f28c
                    {
                        let r = fl.add(ecx, eax, 32);
                        ecx = r;
                    } // 00e5f28f
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f291
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e5f294
                    ecx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f298
                    {
                        let r = fl.add(ebx, 0x00000007u32, 32);
                        ebx = r;
                    } // 00e5f29c
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f29f
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5f2a3
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5f2a6
                    e.mem.set_u32(
                        eax.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x94),
                        fpu.fstp_m32(),
                    ); // 00e5f2aa
                    pc = 0x00e5f316; // 00e5f2b1
                    continue;
                }
                0x00e5f2b3 => {
                    eax = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x1))); // 00e5f2b3
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f2b8
                    {
                        let r = fl.shl(eax, ((ecx & 0xff) & 0x1f), 32);
                        eax = r;
                    } // 00e5f2bb
                    ecx = eax; // 00e5f2bd
                    eax = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5f2bf
                    {
                        let r = fl.add(
                            ecx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        ecx = r;
                    } // 00e5f2c3
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e5f2ca
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f2ce
                    ecx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f2d2
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5f2d6
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f2d9
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x98),
                        fpu.fstp_m32(),
                    ); // 00e5f2dc
                    eax = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x2))); // 00e5f2e3
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f2e8
                    {
                        let r = fl.shl(eax, ((ecx & 0xff) & 0x1f), 32);
                        eax = r;
                    } // 00e5f2eb
                    ecx = eax; // 00e5f2ed
                    eax = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5f2ef
                    {
                        let r = fl.add(
                            ecx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        ecx = r;
                    } // 00e5f2f3
                    {
                        let r = fl.add(ebx, 0x00000003u32, 32);
                        ebx = r;
                    } // 00e5f2fa
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e5f2fd
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f301
                    ecx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f305
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5f309
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f30c
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x88),
                        fpu.fstp_m32(),
                    ); // 00e5f30f
                    pc = 0x00e5f316;
                }
                0x00e5f316 => {
                    eax = u32::from(e.mem.u8(ebx.wrapping_add(esi))); // 00e5f316
                    fl.sub(eax, 0x00000026u32, 32); // 00e5f31a
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax); // 00e5f31d
                    if fl.sf == fl.of {
                        pc = 0x00e5f234;
                        continue;
                    } // 00e5f321
                    pc = 0x00e5f327;
                }
                0x00e5f327 => {
                    eax = u32::from(e.mem.u8(edx.wrapping_add(esi))); // 00e5f327
                    fl.sub(eax, 0x00000026u32, 32); // 00e5f32b
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax); // 00e5f32e
                    if fl.sf != fl.of {
                        pc = 0x00e5f40e;
                        continue;
                    } // 00e5f332
                    pc = 0x00e5f338;
                }
                0x00e5f338 => {
                    fl.sub(eax, 0x0000002bu32, 32); // 00e5f338
                    if !fl.zf && fl.sf == fl.of {
                        pc = 0x00e5f40a;
                        continue;
                    } // 00e5f33b
                    fl.sub(eax, 0x00000029u32, 32); // 00e5f341
                    eax = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x1))); // 00e5f344
                    if fl.sf != fl.of {
                        pc = 0x00e5f3ab;
                        continue;
                    } // 00e5f349
                    ecx = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x2))); // 00e5f34b
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f350
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5f353
                    ecx = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x3))); // 00e5f355
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f35a
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5f35d
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5f35f
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e5f363
                    eax = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f367
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f36b
                    fpu.fmul_m32(e.mem.u32(eax.wrapping_add(0x10))); // 00e5f36f
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x64),
                        fpu.fstp_m32(),
                    ); // 00e5f372
                    ecx = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x4))); // 00e5f376
                    eax = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x5))); // 00e5f37b
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5f380
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5f383
                    ecx = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x6))); // 00e5f385
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f38a
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5f38d
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5f38f
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e5f393
                    eax = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f397
                    {
                        let r = fl.add(edx, 0x00000007u32, 32);
                        edx = r;
                    } // 00e5f39b
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f39e
                    fpu.fmul_m32(e.mem.u32(eax.wrapping_add(0x10))); // 00e5f3a2
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x54),
                        fpu.fstp_m32(),
                    ); // 00e5f3a5
                    pc = 0x00e5f3f9; // 00e5f3a9
                    continue;
                }
                0x00e5f3ab => {
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f3ab
                    {
                        let r = fl.shl(eax, ((ecx & 0xff) & 0x1f), 32);
                        eax = r;
                    } // 00e5f3ae
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5f3b0
                    {
                        let r = fl.add(
                            eax,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        eax = r;
                    } // 00e5f3b4
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e5f3bb
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f3bf
                    eax = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f3c3
                    fpu.fmul_m32(e.mem.u32(eax.wrapping_add(0x10))); // 00e5f3c7
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x58),
                        fpu.fstp_m32(),
                    ); // 00e5f3ca
                    eax = u32::from(e.mem.u8(edx.wrapping_add(esi).wrapping_add(0x2))); // 00e5f3ce
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f3d3
                    {
                        let r = fl.shl(eax, ((ecx & 0xff) & 0x1f), 32);
                        eax = r;
                    } // 00e5f3d6
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5f3d8
                    {
                        let r = fl.add(
                            eax,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        eax = r;
                    } // 00e5f3dc
                    {
                        let r = fl.add(edx, 0x00000003u32, 32);
                        edx = r;
                    } // 00e5f3e3
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e5f3e6
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f3ea
                    eax = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f3ee
                    fpu.fmul_m32(e.mem.u32(eax.wrapping_add(0x10))); // 00e5f3f2
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x48),
                        fpu.fstp_m32(),
                    ); // 00e5f3f5
                    pc = 0x00e5f3f9;
                }
                0x00e5f3f9 => {
                    eax = u32::from(e.mem.u8(edx.wrapping_add(esi))); // 00e5f3f9
                    fl.sub(eax, 0x00000026u32, 32); // 00e5f3fd
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax); // 00e5f400
                    if fl.sf == fl.of {
                        pc = 0x00e5f338;
                        continue;
                    } // 00e5f404
                    pc = 0x00e5f40a;
                }
                0x00e5f40a => {
                    e.mem.set_u32(esp.wrapping_add(0x20), edx); // 00e5f40a
                    pc = 0x00e5f40e;
                }
                0x00e5f40e => {
                    ecx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f40e
                    x[1] = load_vec(e, esp.wrapping_add(0x50)); // 00e5f412
                    fpu.fld_m32(e.mem.u32(ecx.wrapping_add(0x3c))); // 00e5f417
                    x[0] = load_vec(e, esp.wrapping_add(0x40)); // 00e5f41a
                    fpu.fmul(0, 0); // 00e5f41f
                    x[2] = x[1]; // 00e5f421
                    x[1] = addps(x[1], x[0]); // 00e5f424
                    x[2] = subps(x[2], x[0]); // 00e5f427
                    e.mem.set_u32(esp.wrapping_add(0x18), fpu.fstp_m32()); // 00e5f42a
                    x[0] = load_vec(e, ecx.wrapping_add(0x30)); // 00e5f42e
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5f432
                    x[1] = subps(x[1], x[0]); // 00e5f435
                    x[1] = andps(x[1], load_vec(e, 0x01017d10u32)); // 00e5f438
                    x[0] = x[1]; // 00e5f43f
                    x[0] = minps(x[0], x[2]); // 00e5f442
                    x[0] = subps(x[0], x[1]); // 00e5f445
                    x[0] = mulps(x[0], x[0]); // 00e5f448
                    x[3] = x[0]; // 00e5f44b
                    x[3] = shufps(x[3], x[0], 0x55); // 00e5f44e
                    x[4] = x[0]; // 00e5f452
                    x[3] = addss(x[3], x[0]); // 00e5f455
                    x[4] = shufps(x[4], x[0], 0xaa); // 00e5f459
                    x[4] = addss(x[4], x[3]); // 00e5f45d
                    e.mem.set_u32(esp.wrapping_add(0x74), x[4][0].to_bits()); // 00e5f461
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x74))); // 00e5f467
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x18))); // 00e5f46b
                    fsw = fpu.fcom_st(1); // 00e5f46f
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e5f471
                    fpu.fstp_st(1); // 00e5f473
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e5f475
                    if fl.pf {
                        pc = 0x00e5f486;
                        continue;
                    } // 00e5f478
                    fpu.pop(); // 00e5f47a
                    e.mem.set_u32(esp.wrapping_add(0x1c), 0x00000000u32); // 00e5f47c
                    pc = 0x00e5f4dd; // 00e5f484
                    continue;
                }
                0x00e5f486 => {
                    x[2] = addps(x[2], x[1]); // 00e5f486
                    x[0] = x[2]; // 00e5f489
                    x[0] = mulps(x[0], x[0]); // 00e5f48c
                    x[1] = x[0]; // 00e5f48f
                    x[1] = shufps(x[1], x[0], 0x55); // 00e5f492
                    x[2] = x[0]; // 00e5f496
                    x[1] = addss(x[1], x[0]); // 00e5f499
                    x[2] = shufps(x[2], x[0], 0xaa); // 00e5f49d
                    x[2] = addss(x[2], x[1]); // 00e5f4a1
                    e.mem.set_u32(esp.wrapping_add(0x80), x[2][0].to_bits()); // 00e5f4a5
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x80))); // 00e5f4ae
                    fsw = fpu.fcompp(); // 00e5f4b5
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e5f4b7
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e5f4b9
                    if fl.pf {
                        pc = 0x00e5f4d5;
                        continue;
                    } // 00e5f4bc
                    {
                        let r = fl.add(edx, esi, 32);
                        edx = r;
                    } // 00e5f4be
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5f4c0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5f4c1
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e65ee0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5f4c2
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5f4c7
                    e.mem.set_u32(esp.wrapping_add(0x1c), 0x00000001u32); // 00e5f4cb
                    pc = 0x00e5f4dd; // 00e5f4d3
                    continue;
                }
                0x00e5f4d5 => {
                    e.mem.set_u32(esp.wrapping_add(0x1c), 0x00000002u32); // 00e5f4d5
                    pc = 0x00e5f4dd;
                }
                0x00e5f4dd => {
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f4dd
                    x[0] = load_vec(e, eax.wrapping_add(0x10)); // 00e5f4e0
                    x[2] = load_vec(e, eax); // 00e5f4e4
                    ecx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f4e7
                    fpu.fld_m32(e.mem.u32(ecx.wrapping_add(0x3c))); // 00e5f4eb
                    x[1] = x[0]; // 00e5f4ee
                    x[0] = addps(x[0], x[2]); // 00e5f4f1
                    fpu.fmul(0, 0); // 00e5f4f4
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5f4f6
                    x[1] = subps(x[1], x[2]); // 00e5f4f9
                    x[2] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5f4fc
                    e.mem.set_u32(esp.wrapping_add(0x18), fpu.fstp_m32()); // 00e5f500
                    x[0] = subps(x[0], x[2]); // 00e5f504
                    x[0] = andps(x[0], load_vec(e, 0x01017d10u32)); // 00e5f507
                    x[2] = x[0]; // 00e5f50e
                    x[0] = minps(x[0], x[1]); // 00e5f511
                    x[0] = subps(x[0], x[2]); // 00e5f514
                    x[0] = mulps(x[0], x[0]); // 00e5f517
                    x[3] = x[0]; // 00e5f51a
                    x[3] = shufps(x[3], x[0], 0x55); // 00e5f51d
                    x[4] = x[0]; // 00e5f521
                    x[3] = addss(x[3], x[0]); // 00e5f524
                    x[4] = shufps(x[4], x[0], 0xaa); // 00e5f528
                    x[4] = addss(x[4], x[3]); // 00e5f52c
                    e.mem.set_u32(esp.wrapping_add(0x6c), x[4][0].to_bits()); // 00e5f530
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x6c))); // 00e5f536
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x18))); // 00e5f53a
                    fsw = fpu.fcom_st(1); // 00e5f53e
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e5f540
                    fpu.fstp_st(1); // 00e5f542
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e5f544
                    if fl.pf {
                        pc = 0x00e5f54f;
                        continue;
                    } // 00e5f547
                    fpu.pop(); // 00e5f549
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    } // 00e5f54b
                    pc = 0x00e5f59b; // 00e5f54d
                    continue;
                }
                0x00e5f54f => {
                    x[1] = addps(x[1], x[2]); // 00e5f54f
                    x[0] = x[1]; // 00e5f552
                    x[0] = mulps(x[0], x[0]); // 00e5f555
                    x[1] = x[0]; // 00e5f558
                    x[1] = shufps(x[1], x[0], 0x55); // 00e5f55b
                    x[2] = x[0]; // 00e5f55f
                    x[1] = addss(x[1], x[0]); // 00e5f562
                    x[2] = shufps(x[2], x[0], 0xaa); // 00e5f566
                    x[2] = addss(x[2], x[1]); // 00e5f56a
                    e.mem.set_u32(esp.wrapping_add(0x78), x[2][0].to_bits()); // 00e5f56e
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x78))); // 00e5f574
                    fsw = fpu.fcompp(); // 00e5f578
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e5f57a
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e5f57c
                    if fl.pf {
                        pc = 0x00e5f596;
                        continue;
                    } // 00e5f57f
                    edx = ebx.wrapping_add(esi); // 00e5f581
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5f584
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5f585
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e65ee0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5f586
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5f58b
                    eax = 0x00000001u32; // 00e5f58f
                    pc = 0x00e5f59b; // 00e5f594
                    continue;
                }
                0x00e5f596 => {
                    eax = 0x00000002u32; // 00e5f596
                    pc = 0x00e5f59b;
                }
                0x00e5f59b => {
                    fl.sub(e.mem.u32(esp.wrapping_add(0x1c)), 0x00000002u32, 32); // 00e5f59b
                    if fl.zf {
                        pc = 0x00e5f5b6;
                        continue;
                    } // 00e5f5a0
                    fl.sub(eax, 0x00000002u32, 32); // 00e5f5a2
                    if !fl.zf {
                        pc = 0x00e5fa79;
                        continue;
                    } // 00e5f5a5
                    {
                        let r = fl.add(esi, ebx, 32);
                        esi = r;
                    } // 00e5f5ab
                    ebx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f5ad
                    pc = 0x00e5f090; // 00e5f5b1
                    continue;
                }
                0x00e5f5b6 => {
                    fl.sub(eax, 0x00000002u32, 32); // 00e5f5b6
                    if fl.zf {
                        pc = 0x00e5f5da;
                        continue;
                    } // 00e5f5b9
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f5bb
                    x[0] = load_vec(e, esp.wrapping_add(0x40)); // 00e5f5be
                    ebx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f5c3
                    store_vec(e, eax, x[0]); // 00e5f5c7
                    x[0] = load_vec(e, esp.wrapping_add(0x50)); // 00e5f5ca
                    store_vec(e, eax.wrapping_add(0x10), x[0]); // 00e5f5cf
                    {
                        let r = fl.add(esi, edx, 32);
                        esi = r;
                    } // 00e5f5d3
                    pc = 0x00e5f090; // 00e5f5d5
                    continue;
                }
                0x00e5f5da => {
                    ecx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f5da
                    eax = esp.wrapping_add(0x40); // 00e5f5de
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5f5e2
                    {
                        let r = fl.add(edx, esi, 32);
                        edx = r;
                    } // 00e5f5e3
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5f5e5
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5f5e6
                    {
                        fn_00e5f070(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5f5e7
                    {
                        let r = fl.add(esi, ebx, 32);
                        esi = r;
                    } // 00e5f5ec
                    ebx = e.mem.u32(esp.wrapping_add(0x24)); // 00e5f5ee
                    pc = 0x00e5f090; // 00e5f5f2
                    continue;
                }
                0x00e5f5f7 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f5f7
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f5fb
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5f5ff
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5f602
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5f604
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5f608
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5f60b
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e5f60d
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f611
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f614
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5f618
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0xa4),
                        fpu.fstp_m32(),
                    ); // 00e5f61b
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5f622
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x5))); // 00e5f626
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5f62a
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5f62d
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x6))); // 00e5f62f
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5f633
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5f636
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5f638
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f63c
                    {
                        let r = fl.add(esi, 0x00000007u32, 32);
                        esi = r;
                    } // 00e5f63f
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f642
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5f646
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x94),
                        fpu.fstp_m32(),
                    ); // 00e5f649
                    pc = 0x00e5f69f; // 00e5f650
                    continue;
                }
                0x00e5f652 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f652
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f656
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f659
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f65b
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5f65e
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5f665
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f669
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5f66d
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x98),
                        fpu.fstp_m32(),
                    ); // 00e5f670
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f677
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f67b
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f67e
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f680
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5f683
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5f68a
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5f68d
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5f691
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5f695
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x88),
                        fpu.fstp_m32(),
                    ); // 00e5f698
                    pc = 0x00e5f69f;
                }
                0x00e5f69f => {
                    eax = (eax & 0xffff_ff00) | ((u32::from(e.mem.u8(esi))) & 0xff); // 00e5f69f
                    fl.sub((eax & 0xff), 0x00000026u32, 8); // 00e5f6a1
                    if fl.cf {
                        pc = 0x00e5f6ad;
                        continue;
                    } // 00e5f6a3
                    fl.sub((eax & 0xff), 0x0000002bu32, 8); // 00e5f6a5
                    if fl.cf || fl.zf {
                        pc = 0x00e5f090;
                        continue;
                    } // 00e5f6a7
                    pc = 0x00e5f6ad;
                }
                0x00e5f6ad => {
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5f6ad
                    fpu.fld_m32(e.mem.u32(ebx.wrapping_add(0x3c))); // 00e5f6b0
                    x[0] = load_vec(e, eax.wrapping_add(0x10)); // 00e5f6b3
                    fpu.fmul(0, 0); // 00e5f6b7
                    x[2] = load_vec(e, eax); // 00e5f6b9
                    x[1] = x[0]; // 00e5f6bc
                    x[0] = addps(x[0], x[2]); // 00e5f6bf
                    e.mem.set_u32(esp.wrapping_add(0x18), fpu.fstp_m32()); // 00e5f6c2
                    x[1] = subps(x[1], x[2]); // 00e5f6c6
                    x[2] = load_vec(e, ebx.wrapping_add(0x30)); // 00e5f6c9
                    x[0] = subps(x[0], x[2]); // 00e5f6cd
                    x[2] = load_vec(e, 0x01017d10u32); // 00e5f6d0
                    ecx = ebx.wrapping_add(0x20); // 00e5f6d7
                    x[0] = andps(x[0], x[2]); // 00e5f6da
                    x[2] = x[0]; // 00e5f6dd
                    x[0] = minps(x[0], x[1]); // 00e5f6e0
                    x[0] = subps(x[0], x[2]); // 00e5f6e3
                    x[0] = mulps(x[0], x[0]); // 00e5f6e6
                    x[3] = x[0]; // 00e5f6e9
                    x[3] = shufps(x[3], x[0], 0x55); // 00e5f6ec
                    x[4] = x[0]; // 00e5f6f0
                    x[3] = addss(x[3], x[0]); // 00e5f6f3
                    x[4] = shufps(x[4], x[0], 0xaa); // 00e5f6f7
                    x[4] = addss(x[4], x[3]); // 00e5f6fb
                    e.mem.set_u32(esp.wrapping_add(0x7c), x[4][0].to_bits()); // 00e5f6ff
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x7c))); // 00e5f705
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x18))); // 00e5f709
                    fsw = fpu.fcom_st(1); // 00e5f70d
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e5f70f
                    fpu.fstp_st(1); // 00e5f711
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e5f713
                    if !fl.pf {
                        pc = 0x00e5fa77;
                        continue;
                    } // 00e5f716
                    x[1] = addps(x[1], x[2]); // 00e5f71c
                    x[0] = x[1]; // 00e5f71f
                    x[0] = mulps(x[0], x[0]); // 00e5f722
                    x[1] = x[0]; // 00e5f725
                    x[1] = shufps(x[1], x[0], 0x55); // 00e5f728
                    x[2] = x[0]; // 00e5f72c
                    x[1] = addss(x[1], x[0]); // 00e5f72f
                    x[2] = shufps(x[2], x[0], 0xaa); // 00e5f733
                    x[2] = addss(x[2], x[1]); // 00e5f737
                    e.mem.set_u32(esp.wrapping_add(0x70), x[2][0].to_bits()); // 00e5f73b
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x70))); // 00e5f741
                    fsw = fpu.fcompp(); // 00e5f745
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e5f747
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e5f749
                    if fl.pf {
                        pc = 0x00e5f090;
                        continue;
                    } // 00e5f74c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5f752
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5f753
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e65ee0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5f754
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5f759
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5f75a
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5f75b
                    esp = ebp; // 00e5f75c
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5f75e
                    return; // 00e5f75f
                }
                0x00e5f762 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f762
                    esi = esi.wrapping_add(edx).wrapping_add(0x2); // 00e5f766
                    pc = 0x00e5f090; // 00e5f76a
                    continue;
                }
                0x00e5f76f => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f76f
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f773
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f777
                    {
                        let r = fl.add(ecx, esi, 32);
                        ecx = r;
                    } // 00e5f77a
                    esi = ecx.wrapping_add(eax).wrapping_add(0x3); // 00e5f77c
                    pc = 0x00e5f090; // 00e5f780
                    continue;
                }
                0x00e5f785 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f785
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f789
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5f78d
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5f791
                    {
                        let r = fl.add(edx, eax, 32);
                        edx = r;
                    } // 00e5f794
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5f796
                    {
                        let r = fl.add(ecx, esi, 32);
                        ecx = r;
                    } // 00e5f799
                    esi = ecx.wrapping_add(edx).wrapping_add(0x4); // 00e5f79b
                    pc = 0x00e5f090; // 00e5f79f
                    continue;
                }
                0x00e5f7a4 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f7a4
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f7a8
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f7ab
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0x8)), 32);
                        edx = r;
                    } // 00e5f7ad
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e5f7b0
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f7b4
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f7b8
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f7bb
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0xc)), 32);
                        edx = r;
                    } // 00e5f7bd
                    e.mem.set_u32(esp.wrapping_add(0x34), edx); // 00e5f7c0
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5f7c4
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f7c8
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5f7cb
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0x10)), 32);
                        edx = r;
                    } // 00e5f7cd
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e5f7d0
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f7d4
                    {
                        let r = fl.sub(ecx, eax, 32);
                        ecx = r;
                    } // 00e5f7d7
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e5f7d9
                    edx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5f7dd
                    e.mem.set_u32(esp.wrapping_add(0x2c), edx); // 00e5f7e0
                    eax = e.mem.u32(edi); // 00e5f7e4
                    e.mem.set_u32(esp.wrapping_add(0x28), eax); // 00e5f7e6
                    edi = esp.wrapping_add(0x28); // 00e5f7ea
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5f7ee
                    pc = 0x00e5f090; // 00e5f7f1
                    continue;
                }
                0x00e5f7f6 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f7f6
                    ecx = esp.wrapping_add(0x28); // 00e5f7fa
                    fl.sub(edi, ecx, 32); // 00e5f7fe
                    if fl.zf {
                        pc = 0x00e5f82f;
                        continue;
                    } // 00e5f800
                    edx = e.mem.u32(edi); // 00e5f802
                    e.mem.set_u32(esp.wrapping_add(0x28), edx); // 00e5f804
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5f808
                    e.mem.set_u32(esp.wrapping_add(0x2c), ecx); // 00e5f80b
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5f80f
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e5f812
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5f816
                    e.mem.set_u32(esp.wrapping_add(0x34), ecx); // 00e5f819
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5f81d
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e5f820
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f824
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e5f827
                    edi = esp.wrapping_add(0x28); // 00e5f82b
                    pc = 0x00e5f82f;
                }
                0x00e5f82f => {
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x28)), eax, 32);
                        e.mem.set_u32(esp.wrapping_add(0x28), r);
                    } // 00e5f82f
                    {
                        let r = fl.add(esi, 0x00000002u32, 32);
                        esi = r;
                    } // 00e5f833
                    pc = 0x00e5f090; // 00e5f836
                    continue;
                }
                0x00e5f83b => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f83b
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f83f
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f843
                    ecx = esp.wrapping_add(0x28); // 00e5f846
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5f84a
                    fl.sub(edi, ecx, 32); // 00e5f84c
                    if fl.zf {
                        pc = 0x00e5f87d;
                        continue;
                    } // 00e5f84e
                    edx = e.mem.u32(edi); // 00e5f850
                    e.mem.set_u32(esp.wrapping_add(0x28), edx); // 00e5f852
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5f856
                    e.mem.set_u32(esp.wrapping_add(0x2c), ecx); // 00e5f859
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5f85d
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e5f860
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5f864
                    e.mem.set_u32(esp.wrapping_add(0x34), ecx); // 00e5f867
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5f86b
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e5f86e
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f872
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e5f875
                    edi = esp.wrapping_add(0x28); // 00e5f879
                    pc = 0x00e5f87d;
                }
                0x00e5f87d => {
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x28)), eax, 32);
                        e.mem.set_u32(esp.wrapping_add(0x28), r);
                    } // 00e5f87d
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5f881
                    pc = 0x00e5f090; // 00e5f884
                    continue;
                }
                0x00e5f889 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f889
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f88d
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5f891
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f895
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5f898
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5f89a
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f89e
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5f8a1
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f8a3
                    ecx = esp.wrapping_add(0x28); // 00e5f8a6
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5f8aa
                    fl.sub(edi, ecx, 32); // 00e5f8ac
                    if fl.zf {
                        pc = 0x00e5f8dd;
                        continue;
                    } // 00e5f8ae
                    edx = e.mem.u32(edi); // 00e5f8b0
                    e.mem.set_u32(esp.wrapping_add(0x28), edx); // 00e5f8b2
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5f8b6
                    e.mem.set_u32(esp.wrapping_add(0x2c), ecx); // 00e5f8b9
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5f8bd
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e5f8c0
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5f8c4
                    e.mem.set_u32(esp.wrapping_add(0x34), ecx); // 00e5f8c7
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5f8cb
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e5f8ce
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f8d2
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e5f8d5
                    edi = esp.wrapping_add(0x28); // 00e5f8d9
                    pc = 0x00e5f8dd;
                }
                0x00e5f8dd => {
                    e.mem.set_u32(esp.wrapping_add(0x28), eax); // 00e5f8dd
                    {
                        let r = fl.add(esi, 0x00000005u32, 32);
                        esi = r;
                    } // 00e5f8e1
                    pc = 0x00e5f090; // 00e5f8e4
                    continue;
                }
                0x00e5f8e9 => {
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f8e9
                    {
                        let r = fl.add(esi, 0x00000002u32, 32);
                        esi = r;
                    } // 00e5f8ed
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x154),
                        ecx,
                    ); // 00e5f8f0
                    pc = 0x00e5f93b; // 00e5f8f7
                    continue;
                }
                0x00e5f8f9 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f8f9
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f8fd
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5f901
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5f904
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x164),
                        edx,
                    ); // 00e5f906
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5f90d
                    pc = 0x00e5f93b; // 00e5f910
                    continue;
                }
                0x00e5f912 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f912
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f916
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5f91a
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5f91d
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5f91f
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5f923
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5f926
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5f928
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5f92c
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5f92f
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x174),
                        edx,
                    ); // 00e5f931
                    {
                        let r = fl.add(esi, 0x00000005u32, 32);
                        esi = r;
                    } // 00e5f938
                    pc = 0x00e5f93b;
                }
                0x00e5f93b => {
                    eax = e.mem.u32(esp.wrapping_add(0x2c)); // 00e5f93b
                    edx = esp.wrapping_add(0x28); // 00e5f93f
                    fl.sub(edi, edx, 32); // 00e5f943
                    if fl.zf {
                        pc = 0x00e5f974;
                        continue;
                    } // 00e5f945
                    ecx = e.mem.u32(edi); // 00e5f947
                    e.mem.set_u32(esp.wrapping_add(0x28), ecx); // 00e5f949
                    edx = e.mem.u32(edi.wrapping_add(0x4)); // 00e5f94d
                    e.mem.set_u32(esp.wrapping_add(0x2c), edx); // 00e5f950
                    ecx = e.mem.u32(edi.wrapping_add(0x8)); // 00e5f954
                    e.mem.set_u32(esp.wrapping_add(0x30), ecx); // 00e5f957
                    edx = e.mem.u32(edi.wrapping_add(0xc)); // 00e5f95b
                    e.mem.set_u32(esp.wrapping_add(0x34), edx); // 00e5f95e
                    ecx = e.mem.u32(edi.wrapping_add(0x10)); // 00e5f962
                    e.mem.set_u32(esp.wrapping_add(0x38), ecx); // 00e5f965
                    edx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5f969
                    e.mem.set_u32(esp.wrapping_add(0x3c), edx); // 00e5f96c
                    edi = esp.wrapping_add(0x28); // 00e5f970
                    pc = 0x00e5f974;
                }
                0x00e5f974 => {
                    e.mem.set_u32(esp.wrapping_add(0x2c), eax); // 00e5f974
                    pc = 0x00e5f090; // 00e5f978
                    continue;
                }
                0x00e5f97d => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000200u32); // 00e5f97d
                    eax = esp.wrapping_add(0x94); // 00e5f982
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5f989
                    ecx = esp.wrapping_add(0x8c); // 00e5f98a
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00cb1600, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5f991
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x010d5100u32); // 00e5f996
                    ecx = esp.wrapping_add(0x88); // 00e5f99b
                    {
                        let w = [ecx, e.mem.u32(esp)];
                        let r = e.call(0x00cc01c0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(4); // 00e5f9a2
                    ecx = e.mem.u32(0x01268360u32); // 00e5f9a7
                    edx = e.mem.u32(ecx); // 00e5f9ad
                    edx = e.mem.u32(edx.wrapping_add(0xc)); // 00e5f9af
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x0000012du32); // 00e5f9b2
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x010ecd88u32); // 00e5f9b7
                    eax = esp.wrapping_add(0x98); // 00e5f9bc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5f9c3
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x1298feddu32); // 00e5f9c4
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000003u32); // 00e5f9c9
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                            e.mem.u32(esp.wrapping_add(16)),
                        ];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(20); // 00e5f9cb
                    fl.and(eax, eax, 32); // 00e5f9cd
                    if fl.zf {
                        pc = 0x00e5f9d2;
                        continue;
                    } // 00e5f9cf
                    panic!("INT3 at 00e5f9d1"); // 00e5f9d1
                }
                0x00e5f9d2 => {
                    ecx = esp.wrapping_add(0x84); // 00e5f9d2
                    {
                        let w = [ecx];
                        let r = e.call(0x00cc00e0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5f9d9
                    pc = 0x00e5f090; // 00e5f9de
                    continue;
                }
                0x00e5f9e3 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f9e3
                    pc = 0x00e5fa34; // 00e5f9e7
                    continue;
                }
                0x00e5f9e9 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f9e9
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f9ed
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5f9f1
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5f9f4
                    pc = 0x00e5fa34; // 00e5f9f6
                    continue;
                }
                0x00e5f9f8 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5f9f8
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5f9fc
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5fa00
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5fa04
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5fa07
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5fa09
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5fa0c
                    pc = 0x00e5fa34; // 00e5fa0e
                    continue;
                }
                0x00e5fa10 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5fa10
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5fa14
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5fa18
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5fa1c
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5fa1f
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5fa21
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5fa25
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5fa28
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5fa2a
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e5fa2d
                    pc = 0x00e5fa34; // 00e5fa2f
                    continue;
                }
                0x00e5fa31 => {
                    {
                        let r = fl.add(eax, 0xffffffd0u32, 32);
                        eax = r;
                    } // 00e5fa31
                    pc = 0x00e5fa34;
                }
                0x00e5fa34 => {
                    ebx = e.mem.u32(ebx); // 00e5fa34
                    esi = eax; // 00e5fa36
                    {
                        let r = fl.add(esi, e.mem.u32(edi), 32);
                        esi = r;
                    } // 00e5fa38
                    e.mem.set_u32(esp.wrapping_add(0x20), eax); // 00e5fa3a
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5fa3e
                    ecx = e.mem.u32(ebx.wrapping_add(0x8)); // 00e5fa43
                    {
                        let r = fl.and(ecx, 0x3fffffffu32, 32);
                        ecx = r;
                    } // 00e5fa46
                    fl.sub(e.mem.u32(ebx.wrapping_add(0x4)), ecx, 32); // 00e5fa4c
                    if !fl.zf {
                        pc = 0x00e5fa60;
                        continue;
                    } // 00e5fa4f
                    edx = e.mem.u32(eax.wrapping_add(0x10)); // 00e5fa51
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000004u32); // 00e5fa54
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e5fa56
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5fa57
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00c90940, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e5fa58
                    {
                        let r = fl.add(esp, 0x0000000cu32, 32);
                        esp = r;
                    } // 00e5fa5d
                    pc = 0x00e5fa60;
                }
                0x00e5fa60 => {
                    eax = e.mem.u32(ebx.wrapping_add(0x4)); // 00e5fa60
                    ecx = e.mem.u32(ebx); // 00e5fa63
                    ecx = ecx.wrapping_add(eax.wrapping_mul(4)); // 00e5fa65
                    {
                        let r = fl.inc(eax, 32);
                        eax = r;
                    } // 00e5fa68
                    e.mem.set_u32(ebx.wrapping_add(0x4), eax); // 00e5fa69
                    e.mem.set_u32(ecx, esi); // 00e5fa6c
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5fa6e
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5fa6f
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5fa70
                    esp = ebp; // 00e5fa71
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5fa73
                    return; // 00e5fa74
                }
                0x00e5fa77 => {
                    fpu.pop(); // 00e5fa77
                    pc = 0x00e5fa79;
                }
                0x00e5fa79 => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5fa79
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5fa7a
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5fa7b
                    esp = ebp; // 00e5fa7c
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e5fa7e
                    return; // 00e5fa7f
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e5fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// MOPP code interpreter (thiscall, `RET 0xc`: `this` the machine, `sink`, `code` (the address of the
/// current code byte) and `bounds` (the 32 bytes of query data)): each step dispatches on the code
/// byte (values up to 0x6b through a byte map and a 23-entry jump table written as `match`es
/// on the code byte and the case), tests the query against the node's split planes with SSE (`MULPS`, `ADDPS`,
/// `SHUFPS`, `CMPLTPS`, `MOVMSKPS`), and either descends by calling itself with the offset code
/// pointer, walks on, or reports hits through the machine's virtual methods (`00c85750`, `00c90940`
/// array growth, `00cb1600`, `00cc00e0`, `00cc01c0` and the `00e65ee0`-family callees). An opcode
/// the table does not know ends in an `INT3`, which stops here with a panic. Transcribed
/// instruction by instruction. The four variants of this interpreter differ in the data layout of
/// the query.
pub fn fn_00e5fb50(e: &mut Engine, this: Ptr, sink: Ptr, code: Ptr, bounds: Ptr) {
    e.with_stack(0x500, |e, frame| {
        let mut esp = frame.addr() + 0x400;
        e.mem.set_u32(esp.wrapping_add(4), sink.addr());
        e.mem.set_u32(esp.wrapping_add(8), code.addr());
        e.mem.set_u32(esp.wrapping_add(12), bounds.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e5fb50;
        loop {
            match pc {
                0x00e5fb50 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e5fb50
                    ebp = esp; // 00e5fb51
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e5fb53
                    {
                        let r = fl.sub(esp, 0x00000284u32, 32);
                        esp = r;
                    } // 00e5fb56
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e5fb5c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5fb5d
                    esi = e.mem.u32(ebp.wrapping_add(0xc)); // 00e5fb5e
                    ebx = ecx; // 00e5fb61
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5fb63
                    edi = e.mem.u32(ebp.wrapping_add(0x8)); // 00e5fb64
                    e.mem.set_u32(esp.wrapping_add(0x20), ebx); // 00e5fb67
                    pc = 0x00e5fb6b;
                }
                0x00e5fb6b => {
                    eax = u32::from(e.mem.u8(esi)); // 00e5fb6b
                    fl.sub(eax, 0x0000006bu32, 32); // 00e5fb6e
                    if !fl.cf && !fl.zf {
                        pc = 0x00e60475;
                        continue;
                    } // 00e5fb71
                    ecx = match eax {
                        0 => 0,
                        1..=4 => 1,
                        5 => 2,
                        6 => 3,
                        7 => 4,
                        8 | 12..=15 | 29..=31 | 44..=47 | 84..=95 => 22,
                        9 => 5,
                        10 => 6,
                        11 => 7,
                        16..=18 => 8,
                        19..=28 => 9,
                        32..=34 => 10,
                        35..=37 => 11,
                        38..=40 => 12,
                        41..=43 => 13,
                        48..=79 => 14,
                        80 => 15,
                        81 => 16,
                        82 => 17,
                        83 => 18,
                        96..=99 => 19,
                        100..=103 => 20,
                        104..=107 => 21,
                        _ => unreachable!(),
                    }; // 00e5fb77
                    pc = match ecx {
                        0 => 0x00e6_0571,
                        1 => 0x00e6_029c,
                        2 => 0x00e6_025a,
                        3 => 0x00e6_0267,
                        4 => 0x00e6_027d,
                        5 => 0x00e6_02ee,
                        6 => 0x00e6_0333,
                        7 => 0x00e6_0381,
                        8 => 0x00e5_fbb8,
                        9 => 0x00e5_fb85,
                        10 => 0x00e5_fc1c,
                        11 => 0x00e5_fc7a,
                        12 => 0x00e6_013b,
                        13 => 0x00e6_00e0,
                        14 => 0x00e6_0529,
                        15 => 0x00e6_04db,
                        16 => 0x00e6_04e1,
                        17 => 0x00e6_04f0,
                        18 => 0x00e6_0508,
                        19 => 0x00e6_03e1,
                        20 => 0x00e6_03f1,
                        21 => 0x00e6_040a,
                        22 => 0x00e6_0475,
                        _ => unreachable!(),
                    }; // 00e5fb7e
                    continue;
                }
                0x00e5fb85 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5fb85
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fb89
                    x[0] = load_vec(e, eax); // 00e5fb8c
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5fb8f
                    x[0] = load_vec(e, eax.wrapping_add(0x10)); // 00e5fb94
                    eax = esp.wrapping_add(0x40); // 00e5fb98
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e5fb9c
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5fb9d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e5fba0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5fba1
                    ecx = ebx; // 00e5fba2
                    e.mem.set_u32(esp.wrapping_add(0x28), edx); // 00e5fba4
                    store_vec(e, esp.wrapping_add(0x5c), x[0]); // 00e5fba8
                    {
                        fn_00e5fb50(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e5fbad
                    {
                        let r = fl.add(esi, e.mem.u32(esp.wrapping_add(0x1c)), 32);
                        esi = r;
                    } // 00e5fbb2
                    pc = 0x00e5fb6b; // 00e5fbb6
                    continue;
                }
                0x00e5fbb8 => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fbb8
                    x[0] = load_vec(e, ecx); // 00e5fbbb
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5fbbe
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5fbc3
                    store_vec(e, esp.wrapping_add(0x50), x[0]); // 00e5fbc7
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5fbcc
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5fbd0
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5fbd4
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5fbd8
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5fbdb
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fbdd
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x38)),
                            32,
                        );
                        edx = r;
                    } // 00e5fbe0
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5fbe4
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fbe8
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5fbec
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5fbf0
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x30),
                        fpu.fstp_m32(),
                    ); // 00e5fbf3
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5fbf7
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5fbfa
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x38)),
                            32,
                        );
                        edx = r;
                    } // 00e5fbfc
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5fc00
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fc04
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5fc08
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e5fc0b
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e5fc0d
                    e.mem
                        .set_u32(esp.wrapping_add(eax.wrapping_mul(4)), fpu.fstp_m32()); // 00e5fc10
                    eax = u32::from(e.mem.u8(esi.wrapping_sub(0x1))); // 00e5fc13
                    pc = 0x00e5fcf4; // 00e5fc17
                    continue;
                }
                0x00e5fc1c => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fc1c
                    x[0] = load_vec(e, ecx); // 00e5fc1f
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5fc22
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5fc27
                    store_vec(e, esp.wrapping_add(0x50), x[0]); // 00e5fc2b
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5fc30
                    ecx = edx.wrapping_add(0x1); // 00e5fc34
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5fc37
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5fc3b
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5fc3e
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x78)),
                            32,
                        );
                        edx = r;
                    } // 00e5fc40
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5fc44
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5fc48
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fc4c
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5fc50
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5fc52
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x78)),
                            32,
                        );
                        edx = r;
                    } // 00e5fc55
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x40),
                        fpu.fstp_m32(),
                    ); // 00e5fc59
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5fc5d
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fc61
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fc65
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5fc68
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e5fc6b
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e5fc6d
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x70),
                        fpu.fstp_m32(),
                    ); // 00e5fc70
                    eax = u32::from(e.mem.u8(esi.wrapping_sub(0x1))); // 00e5fc74
                    pc = 0x00e5fcf4; // 00e5fc78
                    continue;
                }
                0x00e5fc7a => {
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fc7a
                    x[0] = load_vec(e, ecx); // 00e5fc7d
                    store_vec(e, esp.wrapping_add(0x40), x[0]); // 00e5fc80
                    x[0] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5fc85
                    store_vec(e, esp.wrapping_add(0x50), x[0]); // 00e5fc89
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e5fc8e
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e5fc92
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5fc96
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5fc9a
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5fc9d
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fc9f
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x84)),
                            32,
                        );
                        edx = r;
                    } // 00e5fca2
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5fca9
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fcad
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5fcb1
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5fcb5
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x7c),
                        fpu.fstp_m32(),
                    ); // 00e5fcb8
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5fcbc
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5fcbf
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x84)),
                            32,
                        );
                        edx = r;
                    } // 00e5fcc1
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5fcc8
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fccc
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e5fcd0
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x4c),
                        fpu.fstp_m32(),
                    ); // 00e5fcd3
                    ebx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e5fcd7
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x5))); // 00e5fcdb
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e5fcdf
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x6))); // 00e5fce3
                    {
                        let r = fl.shl(ebx, 8, 32);
                        ebx = r;
                    } // 00e5fce7
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e5fcea
                    {
                        let r = fl.add(ebx, edx, 32);
                        ebx = r;
                    } // 00e5fced
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e5fcef
                    {
                        let r = fl.add(esi, 0x00000007u32, 32);
                        esi = r;
                    } // 00e5fcf1
                    pc = 0x00e5fcf4;
                }
                0x00e5fcf4 => {
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi))); // 00e5fcf4
                    fl.sub(ecx, 0x00000026u32, 32); // 00e5fcf8
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5fcfb
                    if fl.sf != fl.of {
                        pc = 0x00e5fdf4;
                        continue;
                    } // 00e5fcff
                    pc = 0x00e5fd05;
                }
                0x00e5fd05 => {
                    fl.sub(ecx, 0x0000002bu32, 32); // 00e5fd05
                    if !fl.zf && fl.sf == fl.of {
                        pc = 0x00e5fdf4;
                        continue;
                    } // 00e5fd08
                    fl.sub(ecx, 0x00000029u32, 32); // 00e5fd0e
                    if fl.sf != fl.of {
                        pc = 0x00e5fd84;
                        continue;
                    } // 00e5fd11
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x1))); // 00e5fd13
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x2))); // 00e5fd18
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5fd1d
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5fd20
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x3))); // 00e5fd22
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5fd27
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e5fd2a
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fd2c
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5fd2f
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5fd33
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fd37
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5fd3b
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5fd3e
                    e.mem.set_u32(
                        ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_sub(0xa4),
                        fpu.fstp_m32(),
                    ); // 00e5fd42
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x4))); // 00e5fd49
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x5))); // 00e5fd4e
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5fd53
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5fd56
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x6))); // 00e5fd58
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5fd5d
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5fd60
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5fd62
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e5fd66
                    ecx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5fd6a
                    {
                        let r = fl.add(ebx, 0x00000007u32, 32);
                        ebx = r;
                    } // 00e5fd6e
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fd71
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5fd75
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fd78
                    e.mem.set_u32(
                        ecx.wrapping_add(edx.wrapping_mul(4)).wrapping_sub(0x94),
                        fpu.fstp_m32(),
                    ); // 00e5fd7b
                    pc = 0x00e5fde3; // 00e5fd82
                    continue;
                }
                0x00e5fd84 => {
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x1))); // 00e5fd84
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5fd89
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5fd8c
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5fd8e
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5fd92
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5fd99
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fd9d
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5fda1
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5fda5
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fda8
                    e.mem.set_u32(
                        edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x98),
                        fpu.fstp_m32(),
                    ); // 00e5fdab
                    edx = u32::from(e.mem.u8(ebx.wrapping_add(esi).wrapping_add(0x2))); // 00e5fdb2
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5fdb7
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5fdba
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5fdbc
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5fdc0
                    {
                        let r = fl.add(ebx, 0x00000003u32, 32);
                        ebx = r;
                    } // 00e5fdc7
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5fdca
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fdce
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5fdd2
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5fdd6
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5fdd9
                    e.mem.set_u32(
                        edx.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x88),
                        fpu.fstp_m32(),
                    ); // 00e5fddc
                    pc = 0x00e5fde3;
                }
                0x00e5fde3 => {
                    ecx = u32::from(e.mem.u8(ebx.wrapping_add(esi))); // 00e5fde3
                    fl.sub(ecx, 0x00000026u32, 32); // 00e5fde7
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5fdea
                    if fl.sf == fl.of {
                        pc = 0x00e5fd05;
                        continue;
                    } // 00e5fdee
                    pc = 0x00e5fdf4;
                }
                0x00e5fdf4 => {
                    ecx = u32::from(e.mem.u8(eax.wrapping_add(esi))); // 00e5fdf4
                    fl.sub(ecx, 0x00000026u32, 32); // 00e5fdf8
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5fdfb
                    if fl.sf != fl.of {
                        pc = 0x00e5fedc;
                        continue;
                    } // 00e5fdff
                    pc = 0x00e5fe05;
                }
                0x00e5fe05 => {
                    fl.sub(ecx, 0x0000002bu32, 32); // 00e5fe05
                    if !fl.zf && fl.sf == fl.of {
                        pc = 0x00e5fedc;
                        continue;
                    } // 00e5fe08
                    fl.sub(ecx, 0x00000029u32, 32); // 00e5fe0e
                    if fl.sf != fl.of {
                        pc = 0x00e5fe78;
                        continue;
                    } // 00e5fe11
                    ecx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x1))); // 00e5fe13
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x2))); // 00e5fe18
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5fe1d
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5fe20
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x3))); // 00e5fe22
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5fe27
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5fe2a
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5fe2c
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e5fe30
                    ecx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5fe34
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fe38
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5fe3c
                    e.mem.set_u32(
                        esp.wrapping_add(edx.wrapping_mul(4)).wrapping_sub(0x64),
                        fpu.fstp_m32(),
                    ); // 00e5fe3f
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x4))); // 00e5fe43
                    ecx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x5))); // 00e5fe48
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e5fe4d
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5fe50
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x6))); // 00e5fe52
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e5fe57
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e5fe5a
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5fe5c
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e5fe60
                    ecx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5fe64
                    {
                        let r = fl.add(eax, 0x00000007u32, 32);
                        eax = r;
                    } // 00e5fe68
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fe6b
                    fpu.fmul_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e5fe6f
                    e.mem.set_u32(
                        esp.wrapping_add(edx.wrapping_mul(4)).wrapping_sub(0x54),
                        fpu.fstp_m32(),
                    ); // 00e5fe72
                    pc = 0x00e5fecb; // 00e5fe76
                    continue;
                }
                0x00e5fe78 => {
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x1))); // 00e5fe78
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5fe7d
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5fe80
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5fe82
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5fe86
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5fe8d
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5fe91
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5fe95
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5fe99
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x58),
                        fpu.fstp_m32(),
                    ); // 00e5fe9c
                    edx = u32::from(e.mem.u8(eax.wrapping_add(esi).wrapping_add(0x2))); // 00e5fea0
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e5fea5
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e5fea8
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5feaa
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e5feae
                    {
                        let r = fl.add(eax, 0x00000003u32, 32);
                        eax = r;
                    } // 00e5feb5
                    e.mem.set_u32(esp.wrapping_add(0x18), edx); // 00e5feb8
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x18))); // 00e5febc
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5fec0
                    fpu.fmul_m32(e.mem.u32(edx.wrapping_add(0x10))); // 00e5fec4
                    e.mem.set_u32(
                        esp.wrapping_add(ecx.wrapping_mul(4)).wrapping_sub(0x48),
                        fpu.fstp_m32(),
                    ); // 00e5fec7
                    pc = 0x00e5fecb;
                }
                0x00e5fecb => {
                    ecx = u32::from(e.mem.u8(eax.wrapping_add(esi))); // 00e5fecb
                    fl.sub(ecx, 0x00000026u32, 32); // 00e5fecf
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e5fed2
                    if fl.sf == fl.of {
                        pc = 0x00e5fe05;
                        continue;
                    } // 00e5fed6
                    pc = 0x00e5fedc;
                }
                0x00e5fedc => {
                    ecx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5fedc
                    x[1] = load_vec(e, esp.wrapping_add(0x50)); // 00e5fee0
                    fpu.fld_m32(e.mem.u32(ecx.wrapping_add(0x3c))); // 00e5fee5
                    x[0] = load_vec(e, esp.wrapping_add(0x40)); // 00e5fee8
                    fpu.fmul(0, 0); // 00e5feed
                    x[2] = x[1]; // 00e5feef
                    x[1] = addps(x[1], x[0]); // 00e5fef2
                    x[2] = subps(x[2], x[0]); // 00e5fef5
                    e.mem.set_u32(esp.wrapping_add(0x18), fpu.fstp_m32()); // 00e5fef8
                    x[0] = load_vec(e, ecx.wrapping_add(0x30)); // 00e5fefc
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5ff00
                    x[1] = subps(x[1], x[0]); // 00e5ff03
                    x[1] = andps(x[1], load_vec(e, 0x01017d10u32)); // 00e5ff06
                    x[0] = x[1]; // 00e5ff0d
                    x[0] = minps(x[0], x[2]); // 00e5ff10
                    x[0] = subps(x[0], x[1]); // 00e5ff13
                    x[0] = mulps(x[0], x[0]); // 00e5ff16
                    x[3] = x[0]; // 00e5ff19
                    x[3] = shufps(x[3], x[0], 0x55); // 00e5ff1c
                    x[4] = x[0]; // 00e5ff20
                    x[3] = addss(x[3], x[0]); // 00e5ff23
                    x[4] = shufps(x[4], x[0], 0xaa); // 00e5ff27
                    x[4] = addss(x[4], x[3]); // 00e5ff2b
                    e.mem.set_u32(esp.wrapping_add(0x74), x[4][0].to_bits()); // 00e5ff2f
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x74))); // 00e5ff35
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x18))); // 00e5ff39
                    edx = eax.wrapping_add(esi); // 00e5ff3d
                    fsw = fpu.fcom_st(1); // 00e5ff40
                    e.mem.set_u32(esp.wrapping_add(0x1c), edx); // 00e5ff42
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e5ff46
                    fpu.fstp_st(1); // 00e5ff48
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e5ff4a
                    if fl.pf {
                        pc = 0x00e5ff5b;
                        continue;
                    } // 00e5ff4d
                    fpu.pop(); // 00e5ff4f
                    e.mem.set_u32(esp.wrapping_add(0x18), 0x00000000u32); // 00e5ff51
                    pc = 0x00e5ffbc; // 00e5ff59
                    continue;
                }
                0x00e5ff5b => {
                    x[2] = addps(x[2], x[1]); // 00e5ff5b
                    x[0] = x[2]; // 00e5ff5e
                    x[0] = mulps(x[0], x[0]); // 00e5ff61
                    x[1] = x[0]; // 00e5ff64
                    x[1] = shufps(x[1], x[0], 0x55); // 00e5ff67
                    x[2] = x[0]; // 00e5ff6b
                    x[1] = addss(x[1], x[0]); // 00e5ff6e
                    x[2] = shufps(x[2], x[0], 0xaa); // 00e5ff72
                    x[2] = addss(x[2], x[1]); // 00e5ff76
                    e.mem.set_u32(esp.wrapping_add(0x7c), x[2][0].to_bits()); // 00e5ff7a
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x7c))); // 00e5ff80
                    fsw = fpu.fcompp(); // 00e5ff84
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e5ff86
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e5ff88
                    if fl.pf {
                        pc = 0x00e5ffb4;
                        continue;
                    } // 00e5ff8b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5ff8d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5ff8e
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66600, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5ff8f
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5ff94
                    ecx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5ff98
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e5ff9c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e5ff9d
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5ff9e
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66290, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e5ffa1
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e5ffa6
                    e.mem.set_u32(esp.wrapping_add(0x18), 0x00000001u32); // 00e5ffaa
                    pc = 0x00e5ffbc; // 00e5ffb2
                    continue;
                }
                0x00e5ffb4 => {
                    e.mem.set_u32(esp.wrapping_add(0x18), 0x00000002u32); // 00e5ffb4
                    pc = 0x00e5ffbc;
                }
                0x00e5ffbc => {
                    ecx = e.mem.u32(esp.wrapping_add(0x20)); // 00e5ffbc
                    fpu.fld_m32(e.mem.u32(ecx.wrapping_add(0x3c))); // 00e5ffc0
                    {
                        let r = fl.add(esi, ebx, 32);
                        esi = r;
                    } // 00e5ffc3
                    ebx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e5ffc5
                    fpu.fmul(0, 0); // 00e5ffc8
                    x[0] = load_vec(e, ebx.wrapping_add(0x10)); // 00e5ffca
                    x[2] = load_vec(e, ebx); // 00e5ffce
                    x[1] = x[0]; // 00e5ffd1
                    e.mem.set_u32(esp.wrapping_add(0x24), fpu.fstp_m32()); // 00e5ffd4
                    x[0] = addps(x[0], x[2]); // 00e5ffd8
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e5ffdb
                    x[1] = subps(x[1], x[2]); // 00e5ffde
                    x[2] = load_vec(e, ecx.wrapping_add(0x10)); // 00e5ffe1
                    x[0] = subps(x[0], x[2]); // 00e5ffe5
                    x[0] = andps(x[0], load_vec(e, 0x01017d10u32)); // 00e5ffe8
                    x[2] = x[0]; // 00e5ffef
                    x[0] = minps(x[0], x[1]); // 00e5fff2
                    x[0] = subps(x[0], x[2]); // 00e5fff5
                    x[0] = mulps(x[0], x[0]); // 00e5fff8
                    x[3] = x[0]; // 00e5fffb
                    x[3] = shufps(x[3], x[0], 0x55); // 00e5fffe
                    x[4] = x[0]; // 00e60002
                    x[3] = addss(x[3], x[0]); // 00e60005
                    x[4] = shufps(x[4], x[0], 0xaa); // 00e60009
                    x[4] = addss(x[4], x[3]); // 00e6000d
                    e.mem.set_u32(esp.wrapping_add(0x80), x[4][0].to_bits()); // 00e60011
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x80))); // 00e6001a
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x24))); // 00e60021
                    fsw = fpu.fcom_st(1); // 00e60025
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e60027
                    fpu.fstp_st(1); // 00e60029
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e6002b
                    if fl.pf {
                        pc = 0x00e60036;
                        continue;
                    } // 00e6002e
                    fpu.pop(); // 00e60030
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    } // 00e60032
                    pc = 0x00e6008d; // 00e60034
                    continue;
                }
                0x00e60036 => {
                    x[1] = addps(x[1], x[2]); // 00e60036
                    x[0] = x[1]; // 00e60039
                    x[0] = mulps(x[0], x[0]); // 00e6003c
                    x[1] = x[0]; // 00e6003f
                    x[1] = shufps(x[1], x[0], 0x55); // 00e60042
                    x[2] = x[0]; // 00e60046
                    x[1] = addss(x[1], x[0]); // 00e60049
                    x[2] = shufps(x[2], x[0], 0xaa); // 00e6004d
                    x[2] = addss(x[2], x[1]); // 00e60051
                    e.mem.set_u32(esp.wrapping_add(0x78), x[2][0].to_bits()); // 00e60055
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x78))); // 00e6005b
                    fsw = fpu.fcompp(); // 00e6005f
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e60061
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e60063
                    if fl.pf {
                        pc = 0x00e60088;
                        continue;
                    } // 00e60066
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60068
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e60069
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66600, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e6006a
                    ecx = e.mem.u32(esp.wrapping_add(0x20)); // 00e6006f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60073
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e60074
                    {
                        let r = fl.add(ecx, 0x00000020u32, 32);
                        ecx = r;
                    } // 00e60075
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66290, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e60078
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e6007d
                    eax = 0x00000001u32; // 00e60081
                    pc = 0x00e6008d; // 00e60086
                    continue;
                }
                0x00e60088 => {
                    eax = 0x00000002u32; // 00e60088
                    pc = 0x00e6008d;
                }
                0x00e6008d => {
                    fl.sub(e.mem.u32(esp.wrapping_add(0x18)), 0x00000002u32, 32); // 00e6008d
                    if fl.zf {
                        pc = 0x00e600a6;
                        continue;
                    } // 00e60092
                    fl.sub(eax, 0x00000002u32, 32); // 00e60094
                    if !fl.zf {
                        pc = 0x00e60571;
                        continue;
                    } // 00e60097
                    ebx = e.mem.u32(esp.wrapping_add(0x20)); // 00e6009d
                    pc = 0x00e5fb6b; // 00e600a1
                    continue;
                }
                0x00e600a6 => {
                    fl.sub(eax, 0x00000002u32, 32); // 00e600a6
                    if fl.zf {
                        pc = 0x00e600c7;
                        continue;
                    } // 00e600a9
                    x[0] = load_vec(e, esp.wrapping_add(0x40)); // 00e600ab
                    store_vec(e, ebx, x[0]); // 00e600b0
                    x[0] = load_vec(e, esp.wrapping_add(0x50)); // 00e600b3
                    store_vec(e, ebx.wrapping_add(0x10), x[0]); // 00e600b8
                    ebx = e.mem.u32(esp.wrapping_add(0x20)); // 00e600bc
                    esi = edx; // 00e600c0
                    pc = 0x00e5fb6b; // 00e600c2
                    continue;
                }
                0x00e600c7 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x20)); // 00e600c7
                    eax = esp.wrapping_add(0x40); // 00e600cb
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e600cf
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e600d0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e600d1
                    {
                        fn_00e5fb50(
                            e,
                            Ptr::new(ecx),
                            Ptr::new(e.mem.u32(esp)),
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            Ptr::new(e.mem.u32(esp.wrapping_add(8))),
                        );
                    }
                    esp = esp.wrapping_add(12); // 00e600d2
                    ebx = e.mem.u32(esp.wrapping_add(0x20)); // 00e600d7
                    pc = 0x00e5fb6b; // 00e600db
                    continue;
                }
                0x00e600e0 => {
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e600e0
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e600e4
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e600e8
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e600eb
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e600ed
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e600f1
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e600f4
                    e.mem.set_u32(esp.wrapping_add(0x24), ecx); // 00e600f6
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e600fa
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x24))); // 00e600fd
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e60101
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0xa4),
                        fpu.fstp_m32(),
                    ); // 00e60104
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e6010b
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x5))); // 00e6010f
                    {
                        let r = fl.shl(ecx, 8, 32);
                        ecx = r;
                    } // 00e60113
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e60116
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x6))); // 00e60118
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e6011c
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e6011f
                    e.mem.set_u32(esp.wrapping_add(0x24), edx); // 00e60121
                    edx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e60125
                    {
                        let r = fl.add(esi, 0x00000007u32, 32);
                        esi = r;
                    } // 00e60128
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x24))); // 00e6012b
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e6012f
                    e.mem.set_u32(
                        edx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x94),
                        fpu.fstp_m32(),
                    ); // 00e60132
                    pc = 0x00e60188; // 00e60139
                    continue;
                }
                0x00e6013b => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e6013b
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e6013f
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e60142
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e60144
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e60147
                    e.mem.set_u32(esp.wrapping_add(0x24), edx); // 00e6014e
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x24))); // 00e60152
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e60156
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x98),
                        fpu.fstp_m32(),
                    ); // 00e60159
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e60160
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e60164
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e60167
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e60169
                    {
                        let r = fl.add(
                            edx,
                            e.mem
                                .u32(edi.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x90)),
                            32,
                        );
                        edx = r;
                    } // 00e6016c
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e60173
                    e.mem.set_u32(esp.wrapping_add(0x24), edx); // 00e60176
                    fpu.fild_m32(e.mem.i32(esp.wrapping_add(0x24))); // 00e6017a
                    fpu.fmul_m32(e.mem.u32(ebx.wrapping_add(0x10))); // 00e6017e
                    e.mem.set_u32(
                        ecx.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x88),
                        fpu.fstp_m32(),
                    ); // 00e60181
                    pc = 0x00e60188;
                }
                0x00e60188 => {
                    eax = (eax & 0xffff_ff00) | ((u32::from(e.mem.u8(esi))) & 0xff); // 00e60188
                    fl.sub((eax & 0xff), 0x00000026u32, 8); // 00e6018a
                    if fl.cf {
                        pc = 0x00e60196;
                        continue;
                    } // 00e6018c
                    fl.sub((eax & 0xff), 0x0000002bu32, 8); // 00e6018e
                    if fl.cf || fl.zf {
                        pc = 0x00e5fb6b;
                        continue;
                    } // 00e60190
                    pc = 0x00e60196;
                }
                0x00e60196 => {
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e60196
                    fpu.fld_m32(e.mem.u32(ebx.wrapping_add(0x3c))); // 00e60199
                    x[0] = load_vec(e, eax.wrapping_add(0x10)); // 00e6019c
                    fpu.fmul(0, 0); // 00e601a0
                    x[2] = load_vec(e, eax); // 00e601a2
                    x[1] = x[0]; // 00e601a5
                    x[0] = addps(x[0], x[2]); // 00e601a8
                    e.mem.set_u32(esp.wrapping_add(0x24), fpu.fstp_m32()); // 00e601ab
                    ecx = ebx.wrapping_add(0x20); // 00e601af
                    x[1] = subps(x[1], x[2]); // 00e601b2
                    x[2] = load_vec(e, ecx.wrapping_add(0x10)); // 00e601b5
                    x[0] = subps(x[0], x[2]); // 00e601b9
                    x[2] = load_vec(e, 0x01017d10u32); // 00e601bc
                    x[0] = andps(x[0], x[2]); // 00e601c3
                    x[2] = x[0]; // 00e601c6
                    x[0] = minps(x[0], x[1]); // 00e601c9
                    x[0] = subps(x[0], x[2]); // 00e601cc
                    x[0] = mulps(x[0], x[0]); // 00e601cf
                    x[3] = x[0]; // 00e601d2
                    x[3] = shufps(x[3], x[0], 0x55); // 00e601d5
                    x[4] = x[0]; // 00e601d9
                    x[3] = addss(x[3], x[0]); // 00e601dc
                    x[4] = shufps(x[4], x[0], 0xaa); // 00e601e0
                    x[4] = addss(x[4], x[3]); // 00e601e4
                    e.mem.set_u32(esp.wrapping_add(0x6c), x[4][0].to_bits()); // 00e601e8
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x6c))); // 00e601ee
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x24))); // 00e601f2
                    e.mem.set_u32(esp.wrapping_add(0x18), ecx); // 00e601f6
                    fsw = fpu.fcom_st(1); // 00e601fa
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e601fc
                    fpu.fstp_st(1); // 00e601fe
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e60200
                    if !fl.pf {
                        pc = 0x00e6056f;
                        continue;
                    } // 00e60203
                    x[1] = addps(x[1], x[2]); // 00e60209
                    x[0] = x[1]; // 00e6020c
                    x[0] = mulps(x[0], x[0]); // 00e6020f
                    x[1] = x[0]; // 00e60212
                    x[1] = shufps(x[1], x[0], 0x55); // 00e60215
                    x[2] = x[0]; // 00e60219
                    x[1] = addss(x[1], x[0]); // 00e6021c
                    x[2] = shufps(x[2], x[0], 0xaa); // 00e60220
                    x[2] = addss(x[2], x[1]); // 00e60224
                    e.mem.set_u32(esp.wrapping_add(0x70), x[2][0].to_bits()); // 00e60228
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x70))); // 00e6022e
                    fsw = fpu.fcompp(); // 00e60232
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e60234
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e60236
                    if fl.pf {
                        pc = 0x00e5fb6b;
                        continue;
                    } // 00e60239
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e6023f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e60240
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66600, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e60241
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e60246
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e6024a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e6024b
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00e66290, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e6024c
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60251
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60252
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60253
                    esp = ebp; // 00e60254
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60256
                    return; // 00e60257
                }
                0x00e6025a => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e6025a
                    esi = esi.wrapping_add(edx).wrapping_add(0x2); // 00e6025e
                    pc = 0x00e5fb6b; // 00e60262
                    continue;
                }
                0x00e60267 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e60267
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e6026b
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e6026f
                    {
                        let r = fl.add(ecx, esi, 32);
                        ecx = r;
                    } // 00e60272
                    esi = ecx.wrapping_add(eax).wrapping_add(0x3); // 00e60274
                    pc = 0x00e5fb6b; // 00e60278
                    continue;
                }
                0x00e6027d => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e6027d
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e60281
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e60285
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e60289
                    {
                        let r = fl.add(edx, eax, 32);
                        edx = r;
                    } // 00e6028c
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e6028e
                    {
                        let r = fl.add(ecx, esi, 32);
                        ecx = r;
                    } // 00e60291
                    esi = ecx.wrapping_add(edx).wrapping_add(0x4); // 00e60293
                    pc = 0x00e5fb6b; // 00e60297
                    continue;
                }
                0x00e6029c => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e6029c
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e602a0
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e602a3
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0x8)), 32);
                        edx = r;
                    } // 00e602a5
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e602a8
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e602ac
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e602b0
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e602b3
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0xc)), 32);
                        edx = r;
                    } // 00e602b5
                    e.mem.set_u32(esp.wrapping_add(0x34), edx); // 00e602b8
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e602bc
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e602c0
                    {
                        let r = fl.shl(edx, ((ecx & 0xff) & 0x1f), 32);
                        edx = r;
                    } // 00e602c3
                    {
                        let r = fl.add(edx, e.mem.u32(edi.wrapping_add(0x10)), 32);
                        edx = r;
                    } // 00e602c5
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e602c8
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e602cc
                    {
                        let r = fl.sub(ecx, eax, 32);
                        ecx = r;
                    } // 00e602cf
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e602d1
                    edx = e.mem.u32(edi.wrapping_add(0x4)); // 00e602d5
                    e.mem.set_u32(esp.wrapping_add(0x2c), edx); // 00e602d8
                    eax = e.mem.u32(edi); // 00e602dc
                    e.mem.set_u32(esp.wrapping_add(0x28), eax); // 00e602de
                    edi = esp.wrapping_add(0x28); // 00e602e2
                    {
                        let r = fl.add(esi, 0x00000004u32, 32);
                        esi = r;
                    } // 00e602e6
                    pc = 0x00e5fb6b; // 00e602e9
                    continue;
                }
                0x00e602ee => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e602ee
                    ecx = esp.wrapping_add(0x28); // 00e602f2
                    fl.sub(edi, ecx, 32); // 00e602f6
                    if fl.zf {
                        pc = 0x00e60327;
                        continue;
                    } // 00e602f8
                    edx = e.mem.u32(edi); // 00e602fa
                    e.mem.set_u32(esp.wrapping_add(0x28), edx); // 00e602fc
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e60300
                    e.mem.set_u32(esp.wrapping_add(0x2c), ecx); // 00e60303
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e60307
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e6030a
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e6030e
                    e.mem.set_u32(esp.wrapping_add(0x34), ecx); // 00e60311
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e60315
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e60318
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e6031c
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e6031f
                    edi = esp.wrapping_add(0x28); // 00e60323
                    pc = 0x00e60327;
                }
                0x00e60327 => {
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x28)), eax, 32);
                        e.mem.set_u32(esp.wrapping_add(0x28), r);
                    } // 00e60327
                    {
                        let r = fl.add(esi, 0x00000002u32, 32);
                        esi = r;
                    } // 00e6032b
                    pc = 0x00e5fb6b; // 00e6032e
                    continue;
                }
                0x00e60333 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e60333
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e60337
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e6033b
                    ecx = esp.wrapping_add(0x28); // 00e6033e
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e60342
                    fl.sub(edi, ecx, 32); // 00e60344
                    if fl.zf {
                        pc = 0x00e60375;
                        continue;
                    } // 00e60346
                    edx = e.mem.u32(edi); // 00e60348
                    e.mem.set_u32(esp.wrapping_add(0x28), edx); // 00e6034a
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e6034e
                    e.mem.set_u32(esp.wrapping_add(0x2c), ecx); // 00e60351
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e60355
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e60358
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e6035c
                    e.mem.set_u32(esp.wrapping_add(0x34), ecx); // 00e6035f
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e60363
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e60366
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e6036a
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e6036d
                    edi = esp.wrapping_add(0x28); // 00e60371
                    pc = 0x00e60375;
                }
                0x00e60375 => {
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x28)), eax, 32);
                        e.mem.set_u32(esp.wrapping_add(0x28), r);
                    } // 00e60375
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e60379
                    pc = 0x00e5fb6b; // 00e6037c
                    continue;
                }
                0x00e60381 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e60381
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e60385
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e60389
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e6038d
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e60390
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e60392
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e60396
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e60399
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e6039b
                    ecx = esp.wrapping_add(0x28); // 00e6039e
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e603a2
                    fl.sub(edi, ecx, 32); // 00e603a4
                    if fl.zf {
                        pc = 0x00e603d5;
                        continue;
                    } // 00e603a6
                    edx = e.mem.u32(edi); // 00e603a8
                    e.mem.set_u32(esp.wrapping_add(0x28), edx); // 00e603aa
                    ecx = e.mem.u32(edi.wrapping_add(0x4)); // 00e603ae
                    e.mem.set_u32(esp.wrapping_add(0x2c), ecx); // 00e603b1
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e603b5
                    e.mem.set_u32(esp.wrapping_add(0x30), edx); // 00e603b8
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e603bc
                    e.mem.set_u32(esp.wrapping_add(0x34), ecx); // 00e603bf
                    edx = e.mem.u32(edi.wrapping_add(0x10)); // 00e603c3
                    e.mem.set_u32(esp.wrapping_add(0x38), edx); // 00e603c6
                    ecx = e.mem.u32(edi.wrapping_add(0x14)); // 00e603ca
                    e.mem.set_u32(esp.wrapping_add(0x3c), ecx); // 00e603cd
                    edi = esp.wrapping_add(0x28); // 00e603d1
                    pc = 0x00e603d5;
                }
                0x00e603d5 => {
                    e.mem.set_u32(esp.wrapping_add(0x28), eax); // 00e603d5
                    {
                        let r = fl.add(esi, 0x00000005u32, 32);
                        esi = r;
                    } // 00e603d9
                    pc = 0x00e5fb6b; // 00e603dc
                    continue;
                }
                0x00e603e1 => {
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e603e1
                    {
                        let r = fl.add(esi, 0x00000002u32, 32);
                        esi = r;
                    } // 00e603e5
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x154),
                        ecx,
                    ); // 00e603e8
                    pc = 0x00e60433; // 00e603ef
                    continue;
                }
                0x00e603f1 => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e603f1
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e603f5
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e603f9
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e603fc
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x164),
                        edx,
                    ); // 00e603fe
                    {
                        let r = fl.add(esi, 0x00000003u32, 32);
                        esi = r;
                    } // 00e60405
                    pc = 0x00e60433; // 00e60408
                    continue;
                }
                0x00e6040a => {
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e6040a
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e6040e
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e60412
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e60415
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e60417
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e6041b
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e6041e
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e60420
                    {
                        let r = fl.shl(edx, 8, 32);
                        edx = r;
                    } // 00e60424
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e60427
                    e.mem.set_u32(
                        esp.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(0x174),
                        edx,
                    ); // 00e60429
                    {
                        let r = fl.add(esi, 0x00000005u32, 32);
                        esi = r;
                    } // 00e60430
                    pc = 0x00e60433;
                }
                0x00e60433 => {
                    eax = e.mem.u32(esp.wrapping_add(0x2c)); // 00e60433
                    edx = esp.wrapping_add(0x28); // 00e60437
                    fl.sub(edi, edx, 32); // 00e6043b
                    if fl.zf {
                        pc = 0x00e6046c;
                        continue;
                    } // 00e6043d
                    ecx = e.mem.u32(edi); // 00e6043f
                    e.mem.set_u32(esp.wrapping_add(0x28), ecx); // 00e60441
                    edx = e.mem.u32(edi.wrapping_add(0x4)); // 00e60445
                    e.mem.set_u32(esp.wrapping_add(0x2c), edx); // 00e60448
                    ecx = e.mem.u32(edi.wrapping_add(0x8)); // 00e6044c
                    e.mem.set_u32(esp.wrapping_add(0x30), ecx); // 00e6044f
                    edx = e.mem.u32(edi.wrapping_add(0xc)); // 00e60453
                    e.mem.set_u32(esp.wrapping_add(0x34), edx); // 00e60456
                    ecx = e.mem.u32(edi.wrapping_add(0x10)); // 00e6045a
                    e.mem.set_u32(esp.wrapping_add(0x38), ecx); // 00e6045d
                    edx = e.mem.u32(edi.wrapping_add(0x14)); // 00e60461
                    e.mem.set_u32(esp.wrapping_add(0x3c), edx); // 00e60464
                    edi = esp.wrapping_add(0x28); // 00e60468
                    pc = 0x00e6046c;
                }
                0x00e6046c => {
                    e.mem.set_u32(esp.wrapping_add(0x2c), eax); // 00e6046c
                    pc = 0x00e5fb6b; // 00e60470
                    continue;
                }
                0x00e60475 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000200u32); // 00e60475
                    eax = esp.wrapping_add(0x94); // 00e6047a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60481
                    ecx = esp.wrapping_add(0x8c); // 00e60482
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(0x00cb1600, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e60489
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x010d5100u32); // 00e6048e
                    ecx = esp.wrapping_add(0x88); // 00e60493
                    {
                        let w = [ecx, e.mem.u32(esp)];
                        let r = e.call(0x00cc01c0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(4); // 00e6049a
                    ecx = e.mem.u32(0x01268360u32); // 00e6049f
                    edx = e.mem.u32(ecx); // 00e604a5
                    edx = e.mem.u32(edx.wrapping_add(0xc)); // 00e604a7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x0000012du32); // 00e604aa
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x010ecd88u32); // 00e604af
                    eax = esp.wrapping_add(0x98); // 00e604b4
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e604bb
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x1298feddu32); // 00e604bc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000003u32); // 00e604c1
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                            e.mem.u32(esp.wrapping_add(16)),
                        ];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(20); // 00e604c3
                    fl.and(eax, eax, 32); // 00e604c5
                    if fl.zf {
                        pc = 0x00e604ca;
                        continue;
                    } // 00e604c7
                    panic!("INT3 at 00e604c9"); // 00e604c9
                }
                0x00e604ca => {
                    ecx = esp.wrapping_add(0x84); // 00e604ca
                    {
                        let w = [ecx];
                        let r = e.call(0x00cc00e0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e604d1
                    pc = 0x00e5fb6b; // 00e604d6
                    continue;
                }
                0x00e604db => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e604db
                    pc = 0x00e6052c; // 00e604df
                    continue;
                }
                0x00e604e1 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e604e1
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e604e5
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e604e9
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e604ec
                    pc = 0x00e6052c; // 00e604ee
                    continue;
                }
                0x00e604f0 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e604f0
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e604f4
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e604f8
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e604fc
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e604ff
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e60501
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e60504
                    pc = 0x00e6052c; // 00e60506
                    continue;
                }
                0x00e60508 => {
                    eax = u32::from(e.mem.u8(esi.wrapping_add(0x1))); // 00e60508
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x2))); // 00e6050c
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x3))); // 00e60510
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e60514
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e60517
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x4))); // 00e60519
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e6051d
                    {
                        let r = fl.add(eax, ecx, 32);
                        eax = r;
                    } // 00e60520
                    {
                        let r = fl.shl(eax, 8, 32);
                        eax = r;
                    } // 00e60522
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e60525
                    pc = 0x00e6052c; // 00e60527
                    continue;
                }
                0x00e60529 => {
                    {
                        let r = fl.add(eax, 0xffffffd0u32, 32);
                        eax = r;
                    } // 00e60529
                    pc = 0x00e6052c;
                }
                0x00e6052c => {
                    ebx = e.mem.u32(ebx); // 00e6052c
                    esi = eax; // 00e6052e
                    {
                        let r = fl.add(esi, e.mem.u32(edi), 32);
                        esi = r;
                    } // 00e60530
                    e.mem.set_u32(esp.wrapping_add(0x1c), eax); // 00e60532
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e60536
                    ecx = e.mem.u32(ebx.wrapping_add(0x8)); // 00e6053b
                    {
                        let r = fl.and(ecx, 0x3fffffffu32, 32);
                        ecx = r;
                    } // 00e6053e
                    fl.sub(e.mem.u32(ebx.wrapping_add(0x4)), ecx, 32); // 00e60544
                    if !fl.zf {
                        pc = 0x00e60558;
                        continue;
                    } // 00e60547
                    edx = e.mem.u32(eax.wrapping_add(0x10)); // 00e60549
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000004u32); // 00e6054c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e6054e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e6054f
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00c90940, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e60550
                    {
                        let r = fl.add(esp, 0x0000000cu32, 32);
                        esp = r;
                    } // 00e60555
                    pc = 0x00e60558;
                }
                0x00e60558 => {
                    eax = e.mem.u32(ebx.wrapping_add(0x4)); // 00e60558
                    ecx = e.mem.u32(ebx); // 00e6055b
                    ecx = ecx.wrapping_add(eax.wrapping_mul(4)); // 00e6055d
                    {
                        let r = fl.inc(eax, 32);
                        eax = r;
                    } // 00e60560
                    e.mem.set_u32(ebx.wrapping_add(0x4), eax); // 00e60561
                    e.mem.set_u32(ecx, esi); // 00e60564
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60566
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60567
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60568
                    esp = ebp; // 00e60569
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6056b
                    return; // 00e6056c
                }
                0x00e6056f => {
                    fpu.pop(); // 00e6056f
                    pc = 0x00e60571;
                }
                0x00e60571 => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60571
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60572
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60573
                    esp = ebp; // 00e60574
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60576
                    return; // 00e60577
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

// Translated from 00e60650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x1c` (thiscall, no stack arguments).
pub fn fn_00e60650(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x1c))
}

// Translated from 00e60660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `this + 0x14` (thiscall, no stack arguments), or null when `this`
/// is null.
pub fn fn_00e60660(_e: &mut Engine, this: Ptr) -> Ptr {
    if this.is_null() {
        Ptr::NULL
    } else {
        Ptr::new(this.addr().wrapping_add(0x14))
    }
}

// Translated from 00e60670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 0 (no arguments).
pub fn fn_00e60670(_e: &mut Engine) -> u32 {
    0
}

// Translated from 00e60680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The next index (thiscall, `RET 4`): `index + 1` while it is below the signed word at
/// `this + 0x14`, otherwise -1.
pub fn fn_00e60680(e: &mut Engine, this: Ptr, index: i32) -> i32 {
    let next = index.wrapping_add(1);
    if next < e.mem.i32(this.addr().wrapping_add(0x14)) {
        next
    } else {
        -1
    }
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e60690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual call through the owner (thiscall, `RET 8`): the owner is the word at `this + 0xc`, its
/// array of words is at `this + 0x10`; calls the owner's object's method at `+0x14` of its table (the
/// word at `owner + 0x10`, then `+0x14`) with `owner + 0x10` as `this`, the array entry `index` and
/// `second` as stack arguments, and answers that call's `EAX` (the exe ends on it). The
/// function is slot `+0x14` of an `hkpShapeContainer` vtable (in `hkpConvexPieceShape`), whose
/// other implementations the engine map names `getChildShape` (a `const hkpShape*` in the Xbox
/// PDB; they answer it).
pub fn fn_00e60690(e: &mut Engine, this: Ptr, index: u32, second: u32) -> u32 {
    e.with_stack(0x200, |e, frame| {
        let mut esp = frame.addr() + 0x100;
        e.mem.set_u32(esp.wrapping_add(4), index);
        e.mem.set_u32(esp.wrapping_add(8), second);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e60690;
        loop {
            match pc {
                0x00e60690 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60690
                    esi = e.mem.u32(esp.wrapping_add(0xc)); // 00e60691
                    eax = ecx; // 00e60695
                    ecx = e.mem.u32(eax.wrapping_add(0xc)); // 00e60697
                    eax = e.mem.u32(eax.wrapping_add(0x10)); // 00e6069a
                    edx = e.mem.u32(ecx.wrapping_add(0x10)); // 00e6069d
                    edx = e.mem.u32(edx.wrapping_add(0x14)); // 00e606a0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e606a3
                    esi = e.mem.u32(esp.wrapping_add(0xc)); // 00e606a4
                    eax = e.mem.u32(eax.wrapping_add(esi.wrapping_mul(4))); // 00e606a8
                    {
                        let r = fl.add(ecx, 0x00000010u32, 32);
                        ecx = r;
                    } // 00e606ab
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e606ae
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e606af
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e606b1
                    return eax; // 00e606b2
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

// Translated from 00e606c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 16 bytes at the address held in `this + 0x18` to `out` (thiscall, `RET 4`; a
/// `MOVAPS` load and store, bit for bit).
pub fn fn_00e606c0(e: &mut Engine, this: Ptr, out: Ptr) {
    let source = e.mem.u32(this.addr().wrapping_add(0x18));
    for lane in 0..4u32 {
        let word = e.mem.u32(source.wrapping_add(4 * lane));
        e.mem.set_u32(out.addr().wrapping_add(4 * lane), word);
    }
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e606e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the vertices (thiscall, `RET 4`): for each of the `word[+0x1c]` 16-byte vectors at the
/// address in `this + 0x18` copies the vector to `out` and then stores the float at `this + 0x10` in
/// its fourth lane (through the x87 stack).
/// Answers `EAX` at the RET 00e60722: `out`, loaded at 00e606e7. The RTTI-less vtable `010ece2c`
/// holds this at slot +0x30 and hkpShape::castRayBundleImpl (00c9e400) at +0x28; +0x30 is
/// getCollisionSpheresImpl (`const hkSphere*` in the Xbox PDB) in the hkpBoxShape, hkpSphereShape
/// and hkpCapsuleShape vtables, so this is taken as that method (the class is unconfirmed).
/// hkpConvexTransformShape::getCollisionSpheresImpl reads it through its child shape's slot
/// +0x30 (`MOV EDI,EAX` at 00d5df2f).
pub fn fn_00e606e0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    e.with_stack(0x200, |e, frame| {
        let mut esp = frame.addr() + 0x100;
        e.mem.set_u32(esp.wrapping_add(4), out.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e606e0;
        loop {
            match pc {
                0x00e606e0 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e606e0
                    ebp = esp; // 00e606e1
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e606e3
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e606e6
                    eax = e.mem.u32(ebp.wrapping_add(0x8)); // 00e606e7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e606ea
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e606eb
                    {
                        let r = fl.xor(esi, esi, 32);
                        esi = r;
                    } // 00e606ec
                    fl.sub(e.mem.u32(ecx.wrapping_add(0x1c)), esi, 32); // 00e606ee
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e606f1
                    edx = eax; // 00e606f2
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e6071c;
                        continue;
                    } // 00e606f4
                    {
                        let r = fl.xor(edi, edi, 32);
                        edi = r;
                    } // 00e606f6
                    pc = 0x00e60700; // 00e606f8
                    continue;
                }
                0x00e60700 => {
                    ebx = e.mem.u32(ecx.wrapping_add(0x18)); // 00e60700
                    x[0] = load_vec(e, ebx.wrapping_add(edi)); // 00e60703
                    store_vec(e, edx, x[0]); // 00e60707
                    fpu.fld_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e6070a
                    e.mem.set_u32(edx.wrapping_add(0xc), fpu.fstp_m32()); // 00e6070d
                    {
                        let r = fl.inc(esi, 32);
                        esi = r;
                    } // 00e60710
                    {
                        let r = fl.add(edx, 0x00000010u32, 32);
                        edx = r;
                    } // 00e60711
                    {
                        let r = fl.add(edi, 0x00000010u32, 32);
                        edi = r;
                    } // 00e60714
                    fl.sub(esi, e.mem.u32(ecx.wrapping_add(0x1c)), 32); // 00e60717
                    if fl.sf != fl.of {
                        pc = 0x00e60700;
                        continue;
                    } // 00e6071a
                    pc = 0x00e6071c;
                }
                0x00e6071c => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6071c
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6071d
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6071e
                    esp = ebp; // 00e6071f
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60721
                    return eax; // 00e60722
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e60730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes vertices by index (thiscall, `RET 0xc`: `indexes` (16-bit values, advanced by 2), `count`,
/// `out`): for each index `i` the owner's method at slot `+0x14` (`this + 0x20`'s `+0x10` table) is
/// called with the block number `i / 3` (a multiply by `0x55555556`), a frame buffer of 0x200 bytes
/// receives the block, and the vector `(i % 3) + 2` of it is copied to `out` with `i | 0x3f000000` in
/// its fourth lane. `count` is processed down to zero (a negative count does nothing).
pub fn fn_00e60730(e: &mut Engine, this: Ptr, indexes: Ptr, count: i32, out: Ptr) {
    e.with_stack(0x500, |e, frame| {
        let mut esp = frame.addr() + 0x400;
        e.mem.set_u32(esp.wrapping_add(4), indexes.addr());
        e.mem.set_u32(esp.wrapping_add(8), count as u32);
        e.mem.set_u32(esp.wrapping_add(12), out.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e60730;
        loop {
            match pc {
                0x00e60730 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e60730
                    ebp = esp; // 00e60731
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e60733
                    {
                        let r = fl.sub(esp, 0x00000214u32, 32);
                        esp = r;
                    } // 00e60736
                    eax = e.mem.u32(ebp.wrapping_add(0xc)); // 00e6073c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e6073f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60740
                    {
                        let r = fl.add(eax, 0xffffffffu32, 32);
                        eax = r;
                    } // 00e60741
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e60744
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e60745
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e60749
                    if fl.sf {
                        pc = 0x00e607b3;
                        continue;
                    } // 00e6074d
                    ebx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e6074f
                    pc = 0x00e60752;
                }
                0x00e60752 => {
                    eax = e.mem.u32(ebp.wrapping_add(0x8)); // 00e60752
                    edi = u32::from(e.mem.u16(eax)); // 00e60755
                    ecx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e60758
                    ecx = e.mem.u32(ecx.wrapping_add(0x20)); // 00e6075c
                    eax = 0x55555556u32; // 00e6075f
                    {
                        let wide = i64::from(eax as i32) * i64::from(edi as i32);
                        eax = wide as u32;
                        edx = (wide >> 32) as u32;
                        fl.cf = wide != i64::from(wide as i32);
                        fl.of = fl.cf;
                    } // 00e60764
                    esi = edx; // 00e60766
                    eax = esp.wrapping_add(0x20); // 00e60768
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e6076c
                    eax = e.mem.u32(esp.wrapping_add(0x20)); // 00e6076d
                    eax = e.mem.u32(eax.wrapping_add(0x24)); // 00e60771
                    {
                        let r = fl.shr(esi, 31, 32);
                        esi = r;
                    } // 00e60774
                    {
                        let r = fl.add(esi, edx, 32);
                        esi = r;
                    } // 00e60777
                    edx = e.mem.u32(ecx.wrapping_add(0x10)); // 00e60779
                    eax = e.mem.u32(eax.wrapping_add(esi.wrapping_mul(4))); // 00e6077c
                    edx = e.mem.u32(edx.wrapping_add(0x14)); // 00e6077f
                    {
                        let r = fl.add(ecx, 0x00000010u32, 32);
                        ecx = r;
                    } // 00e60782
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60785
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e60786
                    {
                        let r = fl.add(e.mem.u32(ebp.wrapping_add(0x8)), 0x00000002u32, 32);
                        e.mem.set_u32(ebp.wrapping_add(0x8), r);
                    } // 00e60788
                    edx = edi; // 00e6078c
                    ecx = esi.wrapping_add(esi.wrapping_mul(2)); // 00e6078e
                    {
                        let r = fl.sub(edx, ecx, 32);
                        edx = r;
                    } // 00e60791
                    {
                        let r = fl.add(edx, 0x00000002u32, 32);
                        edx = r;
                    } // 00e60793
                    {
                        let r = fl.shl(edx, 4, 32);
                        edx = r;
                    } // 00e60796
                    x[0] = load_vec(e, edx.wrapping_add(eax)); // 00e60799
                    {
                        let r = fl.or(edi, 0x3f000000u32, 32);
                        edi = r;
                    } // 00e6079d
                    store_vec(e, ebx, x[0]); // 00e607a3
                    e.mem.set_u32(ebx.wrapping_add(0xc), edi); // 00e607a6
                    {
                        let r = fl.add(ebx, 0x00000010u32, 32);
                        ebx = r;
                    } // 00e607a9
                    {
                        let r = fl.sub(e.mem.u32(esp.wrapping_add(0x18)), 0x00000001u32, 32);
                        e.mem.set_u32(esp.wrapping_add(0x18), r);
                    } // 00e607ac
                    if !fl.sf {
                        pc = 0x00e60752;
                        continue;
                    } // 00e607b1
                    pc = 0x00e607b3;
                }
                0x00e607b3 => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e607b3
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e607b4
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e607b5
                    esp = ebp; // 00e607b6
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e607b8
                    return; // 00e607b9
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

// Translated from 00e607c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises a member (thiscall, `RET 4`): the float `value` goes through the x87 stack into
/// `this + 0x10`, the 16-bit word at `+6` becomes 1, the words at `+8` and `+0xc` become 0 and
/// 0x14, the word at `+0x14` is set to `010c6fb0` and then to `010ece08`, and the vtable word at
/// `+0` to `010ece2c`. Returns `this`.
pub fn fn_00e607c0(e: &mut Engine, this: Ptr, value: f32) -> Ptr {
    let base = this.addr();
    e.mem
        .set_u32(base.wrapping_add(0x10), x87_quiet_bits(value.to_bits()));
    e.mem.set_u16(base.wrapping_add(6), 1);
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.mem.set_u32(base.wrapping_add(0xc), 0x14);
    e.mem.set_u32(base.wrapping_add(0x14), 0x010c_6fb0);
    e.mem.set_u32(base, 0x010e_ce2c);
    e.mem.set_u32(base.wrapping_add(0x14), 0x010e_ce08);
    this
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e60800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bounding box of the transformed vertices (thiscall, `RET 0xc`: `rotation` (four 16-byte rows:
/// three columns and a translation), `margin`, `bounds` (two 16-byte vectors)): `bounds[0]` starts at
/// the float at `0104da48` and `bounds[1]` at the one at `0104e148` (splatted; the large float and its negative), every vertex is
/// transformed (`x * c0 + y * c1 + (z * c2 + t)`) and folded in with `MINPS`/`MAXPS`; then
/// `this + 0x10`'s float plus `margin` is subtracted from the minimum and added to the maximum.
pub fn fn_00e60800(e: &mut Engine, this: Ptr, rotation: Ptr, margin: f32, bounds: Ptr) {
    e.with_stack(0x200, |e, frame| {
        let mut esp = frame.addr() + 0x100;
        e.mem.set_u32(esp.wrapping_add(4), rotation.addr());
        e.mem.set_u32(esp.wrapping_add(8), margin.to_bits());
        e.mem.set_u32(esp.wrapping_add(12), bounds.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e60800;
        loop {
            match pc {
                0x00e60800 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e60800
                    ebp = esp; // 00e60801
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e60803
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e60806
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e60807
                    x[0] = [f32::from_bits(e.mem.u32(0x0104da48u32)), 0.0, 0.0, 0.0]; // 00e6080a
                    x[0] = shufps(x[0], x[0], 0x0); // 00e60812
                    store_vec(e, eax, x[0]); // 00e60816
                    x[0] = [f32::from_bits(e.mem.u32(0x0104e148u32)), 0.0, 0.0, 0.0]; // 00e60819
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e60821
                    x[0] = shufps(x[0], x[0], 0x0); // 00e60822
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e60826
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60828
                    store_vec(e, eax.wrapping_add(0x10), x[0]); // 00e60829
                    fl.sub(e.mem.u32(ecx.wrapping_add(0x1c)), ebx, 32); // 00e6082d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e60830
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e6089c;
                        continue;
                    } // 00e60831
                    esi = e.mem.u32(ebp.wrapping_add(0x8)); // 00e60833
                    {
                        let r = fl.xor(edi, edi, 32);
                        edi = r;
                    } // 00e60836
                    pc = 0x00e60840; // 00e60838
                    continue;
                }
                0x00e60840 => {
                    edx = e.mem.u32(ecx.wrapping_add(0x18)); // 00e60840
                    x[1] = load_vec(e, edx.wrapping_add(edi)); // 00e60843
                    x[2] = load_vec(e, esi); // 00e60847
                    x[3] = load_vec(e, esi.wrapping_add(0x10)); // 00e6084a
                    x[0] = x[1]; // 00e6084e
                    x[0] = shufps(x[0], x[1], 0x0); // 00e60851
                    x[0] = mulps(x[0], x[2]); // 00e60855
                    {
                        let r = fl.add(edx, edi, 32);
                        edx = r;
                    } // 00e60858
                    x[2] = x[1]; // 00e6085a
                    x[2] = shufps(x[2], x[1], 0x55); // 00e6085d
                    x[2] = mulps(x[2], x[3]); // 00e60861
                    x[0] = addps(x[0], x[2]); // 00e60864
                    x[2] = x[1]; // 00e60867
                    x[2] = shufps(x[2], x[1], 0xaa); // 00e6086a
                    x[1] = load_vec(e, esi.wrapping_add(0x20)); // 00e6086e
                    x[2] = mulps(x[2], x[1]); // 00e60872
                    x[1] = load_vec(e, esi.wrapping_add(0x30)); // 00e60875
                    x[2] = addps(x[2], x[1]); // 00e60879
                    x[1] = load_vec(e, eax); // 00e6087c
                    x[0] = addps(x[0], x[2]); // 00e6087f
                    x[1] = minps(x[1], x[0]); // 00e60882
                    store_vec(e, eax, x[1]); // 00e60885
                    x[1] = load_vec(e, eax.wrapping_add(0x10)); // 00e60888
                    x[1] = maxps(x[1], x[0]); // 00e6088c
                    {
                        let r = fl.inc(ebx, 32);
                        ebx = r;
                    } // 00e6088f
                    store_vec(e, eax.wrapping_add(0x10), x[1]); // 00e60890
                    {
                        let r = fl.add(edi, 0x00000010u32, 32);
                        edi = r;
                    } // 00e60894
                    fl.sub(ebx, e.mem.u32(ecx.wrapping_add(0x1c)), 32); // 00e60897
                    if fl.sf != fl.of {
                        pc = 0x00e60840;
                        continue;
                    } // 00e6089a
                    pc = 0x00e6089c;
                }
                0x00e6089c => {
                    fpu.fld_m32(e.mem.u32(ecx.wrapping_add(0x10))); // 00e6089c
                    x[1] = load_vec(e, eax); // 00e6089f
                    fpu.fadd_m32(e.mem.u32(ebp.wrapping_add(0xc))); // 00e608a2
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e608a5
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e608a6
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e608a7
                    e.mem.set_u32(esp, fpu.fstp_m32()); // 00e608a8
                    x[0] = [f32::from_bits(e.mem.u32(esp)), 0.0, 0.0, 0.0]; // 00e608ab
                    x[0] = shufps(x[0], x[0], 0x0); // 00e608b0
                    x[1] = subps(x[1], x[0]); // 00e608b4
                    store_vec(e, eax, x[1]); // 00e608b7
                    x[1] = load_vec(e, eax.wrapping_add(0x10)); // 00e608ba
                    x[1] = addps(x[1], x[0]); // 00e608be
                    store_vec(e, eax.wrapping_add(0x10), x[1]); // 00e608c1
                    esp = ebp; // 00e608c5
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e608c7
                    return; // 00e608c8
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e608d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Support vertex (thiscall, `RET 8`: `direction`, `out`): for each of the `word[+0x28]` pieces the
/// owner's method at slot `+0x14` is called (block from the array at `this + 0x24`, frame buffer of
/// 0x200 bytes) and then the returned object's virtual method at `+0x38` with `direction` and a
/// frame vector; the dot product with `direction` (three lanes) is compared (`FCOMP`) with the best so
/// far (the float at `0104e148` to start); a better one is kept and its vector written to `out`. The
/// fourth lane of `out` is the index of the best piece mixed with `0x3f000000`.
pub fn fn_00e608d0(e: &mut Engine, this: Ptr, direction: Ptr, out: Ptr) {
    e.with_stack(0x500, |e, frame| {
        let mut esp = frame.addr() + 0x400;
        e.mem.set_u32(esp.wrapping_add(4), direction.addr());
        e.mem.set_u32(esp.wrapping_add(8), out.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e608d0;
        loop {
            match pc {
                0x00e608d0 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e608d0
                    ebp = esp; // 00e608d1
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e608d3
                    {
                        let r = fl.sub(esp, 0x00000224u32, 32);
                        esp = r;
                    } // 00e608d6
                    fpu.fld_m32(e.mem.u32(0x0104e148u32)); // 00e608dc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e608e2
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e608e3
                    e.mem.set_u32(esp.wrapping_add(0x14), fpu.fstp_m32()); // 00e608e4
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e608e8
                    edi = ecx; // 00e608e9
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e608eb
                    {
                        let r = fl.xor(esi, esi, 32);
                        esi = r;
                    } // 00e608ed
                    fl.sub(e.mem.u32(edi.wrapping_add(0x28)), ebx, 32); // 00e608ef
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e6099c;
                        continue;
                    } // 00e608f2
                    e.mem.set_u32(esp.wrapping_add(0x14), ebx); // 00e608f8
                                                                // no change (a move of a register onto itself); // 00e608fc
                    pc = 0x00e60900;
                }
                0x00e60900 => {
                    ecx = e.mem.u32(edi.wrapping_add(0x20)); // 00e60900
                    eax = e.mem.u32(ecx.wrapping_add(0x10)); // 00e60903
                    eax = e.mem.u32(eax.wrapping_add(0x14)); // 00e60906
                    edx = esp.wrapping_add(0x30); // 00e60909
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e6090d
                    edx = e.mem.u32(edi.wrapping_add(0x24)); // 00e6090e
                    edx = e.mem.u32(edx.wrapping_add(ebx.wrapping_mul(4))); // 00e60911
                    {
                        let r = fl.add(ecx, 0x00000010u32, 32);
                        ecx = r;
                    } // 00e60914
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e60917
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(eax, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e60918
                    edx = e.mem.u32(eax); // 00e6091a
                    edx = e.mem.u32(edx.wrapping_add(0x38)); // 00e6091c
                    ecx = esp.wrapping_add(0x20); // 00e6091f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e60923
                    ecx = e.mem.u32(ebp.wrapping_add(0x8)); // 00e60924
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e60927
                    ecx = eax; // 00e60928
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e6092a
                    eax = e.mem.u32(ebp.wrapping_add(0x8)); // 00e6092c
                    x[0] = load_vec(e, eax); // 00e6092f
                    x[1] = load_vec(e, esp.wrapping_add(0x20)); // 00e60932
                    x[0] = mulps(x[0], x[1]); // 00e60937
                    x[2] = x[0]; // 00e6093a
                    x[2] = shufps(x[2], x[0], 0x55); // 00e6093d
                    x[3] = x[0]; // 00e60941
                    x[2] = addss(x[2], x[0]); // 00e60944
                    x[3] = shufps(x[3], x[0], 0xaa); // 00e60948
                    x[3] = addss(x[3], x[2]); // 00e6094c
                    e.mem.set_u32(esp.wrapping_add(0x1c), x[3][0].to_bits()); // 00e60950
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x1c))); // 00e60956
                    fpu.fld_m32(e.mem.u32(esp.wrapping_add(0x18))); // 00e6095a
                    fsw = fpu.fcom_st(1); // 00e6095e
                    fpu.pop();
                    eax = (eax & 0xffff_0000) | (u32::from(fsw) << 8); // 00e60960
                    fl.and(((eax >> 8) & 0xff), 0x00000005u32, 8); // 00e60962
                    if fl.pf {
                        pc = 0x00e6098b;
                        continue;
                    } // 00e60965
                    eax = e.mem.u32(esp.wrapping_add(0x2c)); // 00e60967
                    e.mem.set_u32(esp.wrapping_add(0x18), fpu.fstp_m32()); // 00e6096b
                    ecx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e6096f
                    {
                        let r = fl.and(eax, 0xc0ffffffu32, 32);
                        eax = r;
                    } // 00e60972
                    edx = ((eax as i32) >> 31) as u32; // 00e60977
                    {
                        let r = fl.and(edx, 0x0000000fu32, 32);
                        edx = r;
                    } // 00e60978
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    } // 00e6097b
                    esi = eax; // 00e6097d
                    {
                        let r = fl.sar(esi, 4, 32);
                        esi = r;
                    } // 00e6097f
                    {
                        let r = fl.add(esi, e.mem.u32(esp.wrapping_add(0x14)), 32);
                        esi = r;
                    } // 00e60982
                    store_vec(e, ecx, x[1]); // 00e60986
                    pc = 0x00e6098d; // 00e60989
                    continue;
                }
                0x00e6098b => {
                    fpu.pop(); // 00e6098b
                    pc = 0x00e6098d;
                }
                0x00e6098d => {
                    {
                        let r = fl.add(e.mem.u32(esp.wrapping_add(0x14)), 0x00000003u32, 32);
                        e.mem.set_u32(esp.wrapping_add(0x14), r);
                    } // 00e6098d
                    {
                        let r = fl.inc(ebx, 32);
                        ebx = r;
                    } // 00e60992
                    fl.sub(ebx, e.mem.u32(edi.wrapping_add(0x28)), 32); // 00e60993
                    if fl.sf != fl.of {
                        pc = 0x00e60900;
                        continue;
                    } // 00e60996
                    pc = 0x00e6099c;
                }
                0x00e6099c => {
                    edx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e6099c
                    {
                        let r = fl.or(esi, 0x3f000000u32, 32);
                        esi = r;
                    } // 00e6099f
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e609a5
                    e.mem.set_u32(edx.wrapping_add(0xc), esi); // 00e609a6
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e609a9
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e609aa
                    esp = ebp; // 00e609ab
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e609ad
                    return; // 00e609ae
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e609c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collision query over the pieces (thiscall, `RET 0xc`): writes a profiling record "TtrcConvxPiece"
/// (a 12-byte record of a label and `RDTSC`'s low word appended to the thread's monitor stream
/// while its cursor is below its limit; the host clock stands in for `RDTSC`), increments the counter at
/// `second + 0x40`, and for every piece (`word[+0x28]` of them) asks the filter at `first + 0x24`
/// (its method `+0`) when it is set, then the owner's method at `+0x14` and the returned object's
/// method at `+0x20` (`first`, `second`); the index of the last piece that answers is stored at
/// `second + 0x20 + counter * 4`; the counter is decremented, a closing record is written and
/// `*result` is whether a piece answered.
/// Answers `result`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00e609c0(e: &mut Engine, this: Ptr, result: Ptr, first: Ptr, second: Ptr) -> Ptr {
    e.with_stack(0x500, |e, frame| {
        let mut esp = frame.addr() + 0x400;
        e.mem.set_u32(esp.wrapping_add(4), result.addr());
        e.mem.set_u32(esp.wrapping_add(8), first.addr());
        e.mem.set_u32(esp.wrapping_add(12), second.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e609c0;
        loop {
            match pc {
                0x00e609c0 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e609c0
                    ebp = esp; // 00e609c1
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e609c3
                    eax = e.mem.u32(0x0126835cu32); // 00e609c6
                    {
                        let r = fl.sub(esp, 0x00000214u32, 32);
                        esp = r;
                    } // 00e609cb
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e609d1
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e609d2
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e609d3
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e609d4
                    esi = ecx; // 00e609d5
                    {
                        let w = [e.mem.u32(esp)];
                        let r = e.call(0x00fdf044, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(4); // 00e609d7
                    ecx = eax; // 00e609dd
                    edi = e.mem.u32(ecx.wrapping_add(0x4)); // 00e609df
                    fl.sub(edi, e.mem.u32(ecx.wrapping_add(0xc)), 32); // 00e609e2
                    if !fl.cf {
                        pc = 0x00e60a00;
                        continue;
                    } // 00e609e5
                    e.mem.set_u32(edi, 0x010ece74u32); // 00e609e7
                    {
                        let stamp = time_stamp();
                        eax = stamp as u32;
                        edx = (stamp >> 32) as u32;
                    } // 00e609ed
                    e.mem.set_u32(esp.wrapping_add(0x14), eax); // 00e609ef
                    edx = e.mem.u32(esp.wrapping_add(0x14)); // 00e609f3
                    e.mem.set_u32(edi.wrapping_add(0x4), edx); // 00e609f7
                    {
                        let r = fl.add(edi, 0x0000000cu32, 32);
                        edi = r;
                    } // 00e609fa
                    e.mem.set_u32(ecx.wrapping_add(0x4), edi); // 00e609fd
                    pc = 0x00e60a00;
                }
                0x00e60a00 => {
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e60a00
                    {
                        let r = fl.inc(e.mem.u32(eax.wrapping_add(0x40)), 32);
                        e.mem.set_u32(eax.wrapping_add(0x40), r);
                    } // 00e60a03
                    {
                        let r = fl.or(ecx, 0xffffffffu32, 32);
                        ecx = r;
                    } // 00e60a06
                    {
                        let r = fl.xor(edi, edi, 32);
                        edi = r;
                    } // 00e60a09
                    fl.sub(e.mem.u32(esi.wrapping_add(0x28)), edi, 32); // 00e60a0b
                    e.mem.set_u32(esp.wrapping_add(0x14), ecx); // 00e60a0e
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e60aa1;
                        continue;
                    } // 00e60a12
                    ebx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e60a18
                    pc = 0x00e60a1b;
                }
                0x00e60a1b => {
                    fl.sub(e.mem.u32(ebx.wrapping_add(0x24)), 0x00000000u32, 32); // 00e60a1b
                    if fl.zf {
                        pc = 0x00e60a5d;
                        continue;
                    } // 00e60a1f
                    eax = e.mem.u32(esi.wrapping_add(0x20)); // 00e60a21
                    fl.and(eax, eax, 32); // 00e60a24
                    if fl.zf {
                        pc = 0x00e60a31;
                        continue;
                    } // 00e60a26
                    {
                        let r = fl.add(eax, 0x00000010u32, 32);
                        eax = r;
                    } // 00e60a28
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e60a2b
                    pc = 0x00e60a39; // 00e60a2f
                    continue;
                }
                0x00e60a31 => {
                    e.mem.set_u32(esp.wrapping_add(0x18), 0x00000000u32); // 00e60a31
                    pc = 0x00e60a39;
                }
                0x00e60a39 => {
                    eax = e.mem.u32(esi.wrapping_add(0x24)); // 00e60a39
                    eax = e.mem.u32(eax.wrapping_add(edi.wrapping_mul(4))); // 00e60a3c
                    ecx = e.mem.u32(ebx.wrapping_add(0x24)); // 00e60a3f
                    edx = e.mem.u32(ecx); // 00e60a42
                    edx = e.mem.u32(edx); // 00e60a44
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60a46
                    eax = e.mem.u32(esp.wrapping_add(0x1c)); // 00e60a47
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60a4b
                    eax = e.mem.u32(esi.wrapping_add(0x20)); // 00e60a4c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60a4f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e60a50
                    eax = esp.wrapping_add(0x2f); // 00e60a51
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60a55
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                            e.mem.u32(esp.wrapping_add(16)),
                        ];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(20); // 00e60a56
                    fl.sub(u32::from(e.mem.u8(eax)), 0x00000000u32, 8); // 00e60a58
                    if fl.zf {
                        pc = 0x00e60a94;
                        continue;
                    } // 00e60a5b
                    pc = 0x00e60a5d;
                }
                0x00e60a5d => {
                    eax = e.mem.u32(esi.wrapping_add(0x20)); // 00e60a5d
                    edx = esp.wrapping_add(0x20); // 00e60a60
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e60a64
                    edx = e.mem.u32(esi.wrapping_add(0x24)); // 00e60a65
                    edx = e.mem.u32(edx.wrapping_add(edi.wrapping_mul(4))); // 00e60a68
                    ecx = eax.wrapping_add(0x10); // 00e60a6b
                    eax = e.mem.u32(ecx); // 00e60a6e
                    eax = e.mem.u32(eax.wrapping_add(0x14)); // 00e60a70
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e60a73
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(eax, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e60a74
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e60a76
                    edx = e.mem.u32(eax); // 00e60a79
                    edx = e.mem.u32(edx.wrapping_add(0x20)); // 00e60a7b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e60a7e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e60a7f
                    ecx = esp.wrapping_add(0x1b); // 00e60a80
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e60a84
                    ecx = eax; // 00e60a85
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(12); // 00e60a87
                    fl.sub(
                        u32::from(e.mem.u8(esp.wrapping_add(0x13))),
                        0x00000000u32,
                        8,
                    ); // 00e60a89
                    if fl.zf {
                        pc = 0x00e60a94;
                        continue;
                    } // 00e60a8e
                    e.mem.set_u32(esp.wrapping_add(0x14), edi); // 00e60a90
                    pc = 0x00e60a94;
                }
                0x00e60a94 => {
                    {
                        let r = fl.inc(edi, 32);
                        edi = r;
                    } // 00e60a94
                    fl.sub(edi, e.mem.u32(esi.wrapping_add(0x28)), 32); // 00e60a95
                    if fl.sf != fl.of {
                        pc = 0x00e60a1b;
                        continue;
                    } // 00e60a98
                    ecx = e.mem.u32(esp.wrapping_add(0x14)); // 00e60a9a
                    eax = e.mem.u32(ebp.wrapping_add(0x10)); // 00e60a9e
                    pc = 0x00e60aa1;
                }
                0x00e60aa1 => {
                    {
                        let r = fl.dec(e.mem.u32(eax.wrapping_add(0x40)), 32);
                        e.mem.set_u32(eax.wrapping_add(0x40), r);
                    } // 00e60aa1
                    edx = e.mem.u32(eax.wrapping_add(0x40)); // 00e60aa4
                    fl.sub(ecx, 0xffffffffu32, 32); // 00e60aa7
                    if fl.zf {
                        pc = 0x00e60ab0;
                        continue;
                    } // 00e60aaa
                    e.mem.set_u32(
                        eax.wrapping_add(edx.wrapping_mul(4)).wrapping_add(0x20),
                        ecx,
                    ); // 00e60aac
                    pc = 0x00e60ab0;
                }
                0x00e60ab0 => {
                    eax = e.mem.u32(0x0126835cu32); // 00e60ab0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60ab5
                    {
                        let w = [e.mem.u32(esp)];
                        let r = e.call(0x00fdf044, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(4); // 00e60ab6
                    esi = eax; // 00e60abc
                    ecx = e.mem.u32(esi.wrapping_add(0x4)); // 00e60abe
                    fl.sub(ecx, e.mem.u32(esi.wrapping_add(0xc)), 32); // 00e60ac1
                    if !fl.cf {
                        pc = 0x00e60adf;
                        continue;
                    } // 00e60ac4
                    e.mem.set_u32(ecx, 0x010c41f8u32); // 00e60ac6
                    {
                        let stamp = time_stamp();
                        eax = stamp as u32;
                        edx = (stamp >> 32) as u32;
                    } // 00e60acc
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e60ace
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e60ad2
                    e.mem.set_u32(ecx.wrapping_add(0x4), edx); // 00e60ad6
                    {
                        let r = fl.add(ecx, 0x0000000cu32, 32);
                        ecx = r;
                    } // 00e60ad9
                    e.mem.set_u32(esi.wrapping_add(0x4), ecx); // 00e60adc
                    pc = 0x00e60adf;
                }
                0x00e60adf => {
                    fl.sub(e.mem.u32(esp.wrapping_add(0x14)), 0xffffffffu32, 32); // 00e60adf
                    eax = e.mem.u32(ebp.wrapping_add(0x8)); // 00e60ae4
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60ae7
                    ecx = (ecx & 0xffff_ff00) | (((!fl.zf) as u32) & 0xff); // 00e60ae8
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60aeb
                    e.mem.set_u8(eax, (ecx & 0xff) as u8); // 00e60aec
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60aee
                    esp = ebp; // 00e60aef
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60af1
                    return; // 00e60af2
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    });
    result
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e60b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collision query over the pieces with a callback (thiscall, `RET 0xc`): the same profiling records and
/// filter test as `00e609c0` (the host clock stands in for `RDTSC`), but each accepted piece calls the
/// returned object's method at `+0x24` with `first`, a four-word record on the frame (that object, the piece
/// position, the word at `second + 8` and `second`) and `third`.
pub fn fn_00e60b00(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, third: u32) {
    e.with_stack(0x500, |e, frame| {
        let mut esp = frame.addr() + 0x400;
        e.mem.set_u32(esp.wrapping_add(4), first.addr());
        e.mem.set_u32(esp.wrapping_add(8), second.addr());
        e.mem.set_u32(esp.wrapping_add(12), third);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e60b00;
        loop {
            match pc {
                0x00e60b00 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e60b00
                    ebp = esp; // 00e60b01
                    {
                        let r = fl.and(esp, 0xfffffff0u32, 32);
                        esp = r;
                    } // 00e60b03
                    eax = e.mem.u32(0x0126835cu32); // 00e60b06
                    {
                        let r = fl.sub(esp, 0x00000224u32, 32);
                        esp = r;
                    } // 00e60b0b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e60b11
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60b12
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e60b13
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60b14
                    esi = ecx; // 00e60b15
                    {
                        let w = [e.mem.u32(esp)];
                        let r = e.call(0x00fdf044, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(4); // 00e60b17
                    ecx = eax; // 00e60b1d
                    edi = e.mem.u32(ecx.wrapping_add(0x4)); // 00e60b1f
                    fl.sub(edi, e.mem.u32(ecx.wrapping_add(0xc)), 32); // 00e60b22
                    if !fl.cf {
                        pc = 0x00e60b40;
                        continue;
                    } // 00e60b25
                    e.mem.set_u32(edi, 0x010ece74u32); // 00e60b27
                    {
                        let stamp = time_stamp();
                        eax = stamp as u32;
                        edx = (stamp >> 32) as u32;
                    } // 00e60b2d
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e60b2f
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e60b33
                    e.mem.set_u32(edi.wrapping_add(0x4), edx); // 00e60b37
                    {
                        let r = fl.add(edi, 0x0000000cu32, 32);
                        edi = r;
                    } // 00e60b3a
                    e.mem.set_u32(ecx.wrapping_add(0x4), edi); // 00e60b3d
                    pc = 0x00e60b40;
                }
                0x00e60b40 => {
                    {
                        let r = fl.xor(edi, edi, 32);
                        edi = r;
                    } // 00e60b40
                    fl.sub(e.mem.u32(esi.wrapping_add(0x28)), edi, 32); // 00e60b42
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e60bdd;
                        continue;
                    } // 00e60b45
                    ebx = e.mem.u32(ebp.wrapping_add(0x8)); // 00e60b4b
                    pc = 0x00e60b4e;
                }
                0x00e60b4e => {
                    fl.sub(e.mem.u32(ebx.wrapping_add(0x24)), 0x00000000u32, 32); // 00e60b4e
                    if fl.zf {
                        pc = 0x00e60b90;
                        continue;
                    } // 00e60b52
                    eax = e.mem.u32(esi.wrapping_add(0x20)); // 00e60b54
                    fl.and(eax, eax, 32); // 00e60b57
                    if fl.zf {
                        pc = 0x00e60b64;
                        continue;
                    } // 00e60b59
                    {
                        let r = fl.add(eax, 0x00000010u32, 32);
                        eax = r;
                    } // 00e60b5b
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e60b5e
                    pc = 0x00e60b6c; // 00e60b62
                    continue;
                }
                0x00e60b64 => {
                    e.mem.set_u32(esp.wrapping_add(0x18), 0x00000000u32); // 00e60b64
                    pc = 0x00e60b6c;
                }
                0x00e60b6c => {
                    eax = e.mem.u32(esi.wrapping_add(0x24)); // 00e60b6c
                    eax = e.mem.u32(eax.wrapping_add(edi.wrapping_mul(4))); // 00e60b6f
                    ecx = e.mem.u32(ebx.wrapping_add(0x24)); // 00e60b72
                    edx = e.mem.u32(ecx); // 00e60b75
                    edx = e.mem.u32(edx); // 00e60b77
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60b79
                    eax = e.mem.u32(esp.wrapping_add(0x1c)); // 00e60b7a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60b7e
                    eax = e.mem.u32(esi.wrapping_add(0x20)); // 00e60b7f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60b82
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e60b83
                    eax = esp.wrapping_add(0x2f); // 00e60b84
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60b88
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                            e.mem.u32(esp.wrapping_add(16)),
                        ];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(20); // 00e60b89
                    fl.sub(u32::from(e.mem.u8(eax)), 0x00000000u32, 8); // 00e60b8b
                    if fl.zf {
                        pc = 0x00e60bd3;
                        continue;
                    } // 00e60b8e
                    pc = 0x00e60b90;
                }
                0x00e60b90 => {
                    ecx = e.mem.u32(esi.wrapping_add(0x20)); // 00e60b90
                    eax = e.mem.u32(ecx.wrapping_add(0x10)); // 00e60b93
                    eax = e.mem.u32(eax.wrapping_add(0x14)); // 00e60b96
                    edx = esp.wrapping_add(0x30); // 00e60b99
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e60b9d
                    edx = e.mem.u32(esi.wrapping_add(0x24)); // 00e60b9e
                    edx = e.mem.u32(edx.wrapping_add(edi.wrapping_mul(4))); // 00e60ba1
                    {
                        let r = fl.add(ecx, 0x00000010u32, 32);
                        ecx = r;
                    } // 00e60ba4
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e60ba7
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(eax, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e60ba8
                    ecx = e.mem.u32(ebp.wrapping_add(0xc)); // 00e60baa
                    e.mem.set_u32(esp.wrapping_add(0x2c), ecx); // 00e60bad
                    ecx = e.mem.u32(ecx.wrapping_add(0x8)); // 00e60bb1
                    e.mem.set_u32(esp.wrapping_add(0x28), ecx); // 00e60bb4
                    ecx = e.mem.u32(ebp.wrapping_add(0x10)); // 00e60bb8
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e60bbb
                    ecx = esp.wrapping_add(0x24); // 00e60bbc
                    e.mem.set_u32(esp.wrapping_add(0x24), eax); // 00e60bc0
                    e.mem.set_u32(esp.wrapping_add(0x28), edi); // 00e60bc4
                    edx = e.mem.u32(eax); // 00e60bc8
                    edx = e.mem.u32(edx.wrapping_add(0x24)); // 00e60bca
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e60bcd
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e60bce
                    ecx = eax; // 00e60bcf
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(12); // 00e60bd1
                    pc = 0x00e60bd3;
                }
                0x00e60bd3 => {
                    {
                        let r = fl.inc(edi, 32);
                        edi = r;
                    } // 00e60bd3
                    fl.sub(edi, e.mem.u32(esi.wrapping_add(0x28)), 32); // 00e60bd4
                    if fl.sf != fl.of {
                        pc = 0x00e60b4e;
                        continue;
                    } // 00e60bd7
                    pc = 0x00e60bdd;
                }
                0x00e60bdd => {
                    eax = e.mem.u32(0x0126835cu32); // 00e60bdd
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60be2
                    {
                        let w = [e.mem.u32(esp)];
                        let r = e.call(0x00fdf044, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(4); // 00e60be3
                    esi = eax; // 00e60be9
                    ecx = e.mem.u32(esi.wrapping_add(0x4)); // 00e60beb
                    fl.sub(ecx, e.mem.u32(esi.wrapping_add(0xc)), 32); // 00e60bee
                    if !fl.cf {
                        pc = 0x00e60c0c;
                        continue;
                    } // 00e60bf1
                    e.mem.set_u32(ecx, 0x010c41f8u32); // 00e60bf3
                    {
                        let stamp = time_stamp();
                        eax = stamp as u32;
                        edx = (stamp >> 32) as u32;
                    } // 00e60bf9
                    e.mem.set_u32(esp.wrapping_add(0x18), eax); // 00e60bfb
                    edx = e.mem.u32(esp.wrapping_add(0x18)); // 00e60bff
                    e.mem.set_u32(ecx.wrapping_add(0x4), edx); // 00e60c03
                    {
                        let r = fl.add(ecx, 0x0000000cu32, 32);
                        ecx = r;
                    } // 00e60c06
                    e.mem.set_u32(esi.wrapping_add(0x4), ecx); // 00e60c09
                    pc = 0x00e60c0c;
                }
                0x00e60c0c => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60c0c
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60c0d
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60c0e
                    esp = ebp; // 00e60c0f
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60c11
                    return; // 00e60c12
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e60c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor through a base pointer (thiscall, `RET 4`): `this` is moved down by 0x14 to the
/// object, its word at `+0x14` is reset to `010c6fb0` and its vtable to `0102e388`; when bit 0 of `flags`
/// is set the object is freed through the allocator (`00c85750` router, vtable slot `+8` with the object and
/// its 16-bit word at `+4`). Returns the object.
pub fn fn_00e60c20(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.with_stack(0x200, |e, frame| {
        let mut esp = frame.addr() + 0x100;
        e.mem.set_u32(esp.wrapping_add(4), u32::from(flags));
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e60c20;
        loop {
            match pc {
                0x00e60c20 => {
                    {
                        let r = fl.sub(ecx, 0x00000014u32, 32);
                        ecx = r;
                    } // 00e60c20
                    pc = 0x00e60c30; // 00e60c23
                    continue;
                }
                0x00e60c30 => {
                    fl.and(u32::from(e.mem.u8(esp.wrapping_add(0x4))), 0x00000001u32, 8); // 00e60c30
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60c35
                    esi = ecx; // 00e60c36
                    e.mem.set_u32(esi.wrapping_add(0x14), 0x010c6fb0u32); // 00e60c38
                    e.mem.set_u32(esi, 0x0102e388u32); // 00e60c3f
                    if fl.zf {
                        pc = 0x00e60c5c;
                        continue;
                    } // 00e60c45
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e60c47
                    edx = u32::from(e.mem.u16(esi.wrapping_add(0x4))); // 00e60c4c
                    ecx = e.mem.u32(eax.wrapping_add(0x10)); // 00e60c50
                    eax = e.mem.u32(ecx); // 00e60c53
                    eax = e.mem.u32(eax.wrapping_add(0x8)); // 00e60c55
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e60c58
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60c59
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(eax, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e60c5a
                    pc = 0x00e60c5c;
                }
                0x00e60c5c => {
                    eax = esi; // 00e60c5c
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60c5e
                    return Ptr::new(eax); // 00e60c5f
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

/// The memory router `00e60c30` asks for the allocator (`00c85750`, no arguments).
const MEMORY_ROUTER_00C85750: u32 = 0x00c8_5750;

// Translated from 00e60c30 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00e60c30 to the `RET 4` at 00e60c5f, inside the extent the
/// map gives `00e60c20`): the deleting destructor body [`fn_00e60c20`] reaches after its
/// `SUB ECX, 0x14`, entered directly from its own vtable slot (`010ece2c`). `thiscall` with one
/// stack word: writes `010c6fb0` at `+0x14` and `0102e388` at `+0`, and with bit 0 of the word set
/// frees the object through the memory router's allocator (slot `8`) with the 16-bit size at `+4`.
/// Answers `this`.
pub fn fn_00e60c30(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let object = this.addr();
    e.mem.set_u32(object.wrapping_add(0x14), 0x010c_6fb0);
    e.mem.set_u32(object, 0x0102_e388);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER_00C85750, &args![]).u32();
        let size = u32::from(e.mem.u16(object.wrapping_add(4)));
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        e.vcall(allocator, 8, &args![object, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e5cee0, fn_00e5cee0()),
        entry!(0x00e5cf00, fn_00e5cf00(u32, u32, u32, u32) -> bool),
        entry!(0x00e5cf40, fn_00e5cf40(u32, u32, u32)),
        entry!(0x00e5cf70, fn_00e5cf70(u32, u32, u32)),
        entry!(0x00e5cfb0, fn_00e5cfb0(Ptr, u32)),
        entry!(0x00e5cfe0, fn_00e5cfe0(u32, u32, u32, f32, u32)),
        entry!(0x00e5d010, fn_00e5d010(u32, u32, u32)),
        entry!(0x00e5d040, fn_00e5d040(u32, u32, u32)),
        entry!(0x00e5d070, fn_00e5d070(Ptr, Ptr, u32, u32)),
        entry!(0x00e5d360, fn_00e5d360(Ptr, Ptr, u32, u32)),
        entry!(0x00e5d650, fn_00e5d650(Ptr, Ptr, u32, u32)),
        entry!(0x00e5d710, fn_00e5d710(Ptr, Ptr, u32, u32)),
        entry!(0x00e5d7d0, fn_00e5d7d0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e5e430, fn_00e5e430(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e5f070, fn_00e5f070(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e5fb50, fn_00e5fb50(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e60650, fn_00e60650(Ptr) -> u32),
        entry!(0x00e60660, fn_00e60660(Ptr) -> Ptr),
        entry!(0x00e60670, fn_00e60670() -> u32),
        entry!(0x00e60680, fn_00e60680(Ptr, i32) -> i32),
        entry!(0x00e60690, fn_00e60690(Ptr, u32, u32) -> u32),
        entry!(0x00e606c0, fn_00e606c0(Ptr, Ptr)),
        entry!(0x00e606e0, fn_00e606e0(Ptr, Ptr) -> u32),
        entry!(0x00e60730, fn_00e60730(Ptr, Ptr, i32, Ptr)),
        entry!(0x00e607c0, fn_00e607c0(Ptr, f32) -> Ptr),
        entry!(0x00e60800, fn_00e60800(Ptr, Ptr, f32, Ptr)),
        entry!(0x00e608d0, fn_00e608d0(Ptr, Ptr, Ptr)),
        entry!(0x00e609c0, fn_00e609c0(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00e60b00, fn_00e60b00(Ptr, Ptr, Ptr, u32)),
        entry!(0x00e60c20, fn_00e60c20(Ptr, u8) -> Ptr),
        entry!(0x00e6_0c30, fn_00e60c30(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00e53090::test_support::*;
    /// A fake callee: records its argument words in a shared log and answers `eax`.
    const LOG: u32 = 0x7300_0000;

    fn fake(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |e, a| {
            let at = e.mem.u32(LOG);
            e.mem.set_u32(LOG + 4 + 4 * at, address);
            e.mem.set_u32(LOG + 8 + 4 * at, a.len() as u32);
            for (k, w) in a.iter().enumerate() {
                e.mem.set_u32(LOG + 12 + 4 * at + 4 * k as u32, *w);
            }
            e.mem.set_u32(LOG, at + 2 + a.len() as u32);
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    /// The calls the fakes saw, as (address, words).
    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        let total = e.mem.u32(LOG);
        let mut out = Vec::new();
        let mut at = 0;
        while at < total {
            let address = e.mem.u32(LOG + 4 + 4 * at);
            let count = e.mem.u32(LOG + 8 + 4 * at);
            let words = (0..count)
                .map(|k| e.mem.u32(LOG + 12 + 4 * at + 4 * k))
                .collect();
            out.push((address, words));
            at += 2 + count;
        }
        out
    }

    fn logging(e: &mut Engine) {
        e.map(LOG, 0x1000);
    }

    #[test]
    fn the_empty_function_returns() {
        let mut e = Engine::new();
        fn_00e5cee0(&mut e);
        assert_eq!(e.call(0x00e5_cee0, &args![]).u32(), 0);
    }

    #[test]
    fn collision_spheres_00e606e0_answer_the_buffer() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let vectors = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, vectors);
        e.mem.set_u32(this + 0x1c, 2);
        e.mem.set_f32(this + 0x10, 0.5);
        let out = e.mem.alloc(0x20);
        // EAX is `out` (00e606e7) at the RET 00e60722; 00d5df2f reads it.
        assert_eq!(e.call(0x00e6_06e0, &args![this, out]).u32(), out);
        assert_eq!(e.mem.f32(out + 0x1c), 0.5);
        // No vectors: still `out`.
        e.mem.set_u32(this + 0x1c, 0);
        assert_eq!(e.call(0x00e6_06e0, &args![this, out]).u32(), out);
    }

    #[test]
    fn early_exit_obb_query_answers_whether_the_flag_byte_is_set() {
        let mut e = Engine::new();
        logging(&mut e);
        e.map(0x7400_0000, 16);
        fake(&mut e, 0x00e6_47e0, 0x7400_0000);
        assert!(!fn_00e5cf00(&mut e, 1, 2, 3, 4));
        e.mem.set_u8(0x7400_0000, 5);
        assert!(e.call(0x00e5_cf00, &args![1u32, 2u32, 3u32, 4u32]).bool());
        let seen = calls(&e);
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].1.len(), 6);
        assert_eq!(&seen[0].1[2..], &[1, 2, 3, 4]);
    }

    #[test]
    fn query_wrapper_passes_three_words_to_the_machine() {
        let mut e = Engine::new();
        logging(&mut e);
        fake(&mut e, 0x00e6_4f00, 0);
        fn_00e5cf40(&mut e, 7, 8, 9);
        let seen = calls(&e);
        assert_eq!(seen.len(), 1);
        assert_eq!(&seen[0].1[1..], &[7, 8, 9]);
        assert_ne!(seen[0].1[0], 0);
    }

    #[test]
    fn query_wrapper_builds_runs_and_takes_down_an_object() {
        let mut e = Engine::new();
        logging(&mut e);
        fake(&mut e, 0x00e6_5e70, 0);
        fake(&mut e, 0x00e6_5d90, 0);
        fake(&mut e, 0x00e6_5ea0, 0);
        fn_00e5cf70(&mut e, 1, 2, 3);
        let seen = calls(&e);
        let order: Vec<u32> = seen.iter().map(|c| c.0).collect();
        assert_eq!(order, vec![0x00e6_5e70, 0x00e6_5d90, 0x00e6_5ea0]);
        assert_eq!(seen[0].1, seen[2].1);
        assert_eq!(seen[0].1[0], seen[1].1[0]);
        assert_eq!(&seen[1].1[1..], &[1, 2, 3]);
    }

    #[test]
    fn query_wrapper_builds_a_three_word_record() {
        let mut e = Engine::new();
        logging(&mut e);
        e.register(0x00e6_5ee0, |e, a| {
            // the record's words are copied into the log by the fake of the next call
            let record = a[1];
            e.mem.set_u32(LOG + 0x800, e.mem.u32(record));
            e.mem.set_u32(LOG + 0x804, e.mem.u32(record + 4));
            e.mem.set_u32(LOG + 0x808, e.mem.u32(record + 8));
            e.mem.set_u32(LOG + 0x80c, a[2]);
            Ret::default()
        });
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x20, 0x1234);
        fn_00e5cfb0(&mut e, Ptr::new(object), 77);
        assert_eq!(e.mem.u32(LOG + 0x800), 0);
        assert_eq!(e.mem.u32(LOG + 0x804), 0);
        assert_eq!(e.mem.u32(LOG + 0x808), 77);
        assert_eq!(e.mem.u32(LOG + 0x80c), 0x1234);
    }

    #[test]
    fn obb_query_wrapper_passes_the_float_through_the_x87_stack() {
        let mut e = Engine::new();
        logging(&mut e);
        fake(&mut e, 0x00dd_1bf0, 0);
        fn_00e5cfe0(&mut e, 1, 2, 3, f32::from_bits(0x7f80_0001), 5);
        let seen = calls(&e);
        assert_eq!(&seen[0].1[1..], &[1, 2, 3, 0x7fc0_0001, 5]);
    }

    #[test]
    fn obb_aabb_query_wrapper() {
        let mut e = Engine::new();
        logging(&mut e);
        fake(&mut e, 0x00dd_1da0, 0);
        fn_00e5d010(&mut e, 4, 5, 6);
        assert_eq!(&calls(&e)[0].1[1..], &[4, 5, 6]);
    }

    #[test]
    fn modify_aabb_query_wrapper() {
        let mut e = Engine::new();
        logging(&mut e);
        fake(&mut e, 0x00e6_7230, 0);
        fn_00e5d040(&mut e, 4, 5, 6);
        assert_eq!(&calls(&e)[0].1[1..], &[4, 5, 6]);
    }

    /// A query object: the vector at `+0x10`, the float at `+0x1c`, the code address at `+0x20`.
    fn query_object(e: &mut Engine, code: &[u8]) -> Ptr {
        let object = e.mem.alloc(0x40);
        for (k, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(object + 0x10 + 4 * k as u32, *v);
        }
        e.mem.set_f32(object + 0x1c, 2.0);
        let bytes = e.mem.alloc(code.len() as u32 + 8);
        e.mem.write(bytes, code);
        e.mem.set_u32(object + 0x20, bytes);
        Ptr::new(object)
    }

    /// A collector record (the first word of the machine the set-ups pass on).
    fn collector(e: &mut Engine) -> Ptr {
        let record = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(record.addr() + 8, 0x8000_0000);
        record
    }

    /// A list of `count` boxes: the count and the address of 4 floats each.
    fn box_list(e: &mut Engine, count: u32) -> Ptr {
        let data = e.mem.alloc(16 * count.max(1) + 64);
        for k in 0..count * 4 {
            e.mem.set_f32(data + 4 * k, 0.5 + k as f32);
        }
        let list = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(list.addr(), count);
        e.mem.set_u32(list.addr() + 4, data);
        list
    }

    #[test]
    fn box_query_set_up_runs_the_interpreter_on_the_objects_code() {
        for count in [0u32, 1, 4, 5] {
            let mut e = Engine::new();
            heap(&mut e);
            constants(&mut e);
            let object = query_object(&mut e, &[0x32]);
            let found = collector(&mut e);
            let list = box_list(&mut e, count);
            fn_00e5d070(&mut e, object, list, found.addr(), 77);
            assert_eq!(words_of(&e, found), vec![2], "{count} boxes");
        }
    }

    #[test]
    fn second_box_query_set_up() {
        for count in [0u32, 3, 4] {
            let mut e = Engine::new();
            heap(&mut e);
            constants(&mut e);
            let object = query_object(&mut e, &[0x50, 7]);
            let found = collector(&mut e);
            let list = box_list(&mut e, count);
            fn_00e5d360(&mut e, object, list, found.addr(), 77);
            assert_eq!(words_of(&e, found), vec![7], "{count} boxes");
        }
    }

    #[test]
    fn point_query_set_up() {
        let mut e = Engine::new();
        heap(&mut e);
        constants(&mut e);
        let object = query_object(&mut e, &[0x31]);
        let found = collector(&mut e);
        let point = vector(&mut e, [4.0, 5.0, 6.0, 7.0]);
        fn_00e5d650(&mut e, object, point, found.addr(), 77);
        assert_eq!(words_of(&e, found), vec![1]);
    }

    #[test]
    fn second_point_query_set_up() {
        let mut e = Engine::new();
        heap(&mut e);
        constants(&mut e);
        let object = query_object(&mut e, &[0x51, 0, 9]);
        let found = collector(&mut e);
        let point = vector(&mut e, [4.0, 5.0, 6.0, 7.0]);
        fn_00e5d710(&mut e, object, point, found.addr(), 77);
        assert_eq!(words_of(&e, found), vec![9]);
    }

    /// Runs the interpreter variant `run` on `code` with a collector, `base` as the word the
    /// terminal ids are added to, and returns what it collected.
    fn collected(
        run: fn(&mut Engine, Ptr, Ptr, Ptr, Ptr),
        code: &[u8],
        base: u32,
    ) -> (Vec<u32>, u32) {
        let mut e = Engine::new();
        heap(&mut e);
        let record = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(record.addr() + 8, 0x8000_0000);
        let machine = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(machine.addr(), record.addr());
        let sink = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(sink.addr(), base);
        let bytes = e.mem.alloc(code.len() as u32 + 8);
        e.mem.write(bytes, code);
        let bounds = Ptr::new(e.mem.alloc(0x40));
        run(&mut e, machine, sink, Ptr::new(bytes), bounds);
        (words_of(&e, record), e.mem.u32(sink.addr()))
    }

    #[test]
    fn interpreter_collects_terminals() {
        let run = fn_00e5d7d0;
        // 0x00 returns at once; 0x30 + n is a small terminal; 0x50..0x53 carry 8 to 32 bits
        assert_eq!(collected(run, &[0x00], 100).0, Vec::<u32>::new());
        assert_eq!(collected(run, &[0x32], 100).0, vec![102]);
        assert_eq!(collected(run, &[0x50, 9], 100).0, vec![109]);
        assert_eq!(collected(run, &[0x51, 1, 2], 100).0, vec![100 + 0x0102]);
        assert_eq!(collected(run, &[0x52, 1, 2, 3], 0).0, vec![0x01_0203]);
        assert_eq!(collected(run, &[0x53, 1, 2, 3, 4], 0).0, vec![0x0102_0304]);
    }

    #[test]
    fn second_interpreter_collects_terminals() {
        let run = fn_00e5e430;
        assert_eq!(collected(run, &[0x00], 100).0, Vec::<u32>::new());
        assert_eq!(collected(run, &[0x32], 100).0, vec![102]);
        assert_eq!(collected(run, &[0x50, 9], 100).0, vec![109]);
        assert_eq!(collected(run, &[0x53, 1, 2, 3, 4], 0).0, vec![0x0102_0304]);
    }

    #[test]
    fn third_interpreter_collects_terminals() {
        let run = fn_00e5f070;
        assert_eq!(collected(run, &[0x00], 100).0, Vec::<u32>::new());
        assert_eq!(collected(run, &[0x32], 100).0, vec![102]);
        assert_eq!(collected(run, &[0x50, 9], 100).0, vec![109]);
        assert_eq!(collected(run, &[0x53, 1, 2, 3, 4], 0).0, vec![0x0102_0304]);
    }

    #[test]
    fn fourth_interpreter_collects_terminals() {
        let run = fn_00e5fb50;
        assert_eq!(collected(run, &[0x00], 100).0, Vec::<u32>::new());
        assert_eq!(collected(run, &[0x32], 100).0, vec![102]);
        assert_eq!(collected(run, &[0x50, 9], 100).0, vec![109]);
        assert_eq!(collected(run, &[0x53, 1, 2, 3, 4], 0).0, vec![0x0102_0304]);
    }

    #[test]
    fn member_at_plus_0x1c() {
        let mut e = Engine::new();
        let m = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(m.addr() + 0x1c, 0xdead);
        assert_eq!(fn_00e60650(&mut e, m), 0xdead);
    }

    #[test]
    fn base_member_address_or_null() {
        let mut e = Engine::new();
        let m = Ptr::new(e.mem.alloc(0x40));
        assert_eq!(fn_00e60660(&mut e, m).addr(), m.addr() + 0x14);
        assert!(fn_00e60660(&mut e, Ptr::NULL).is_null());
    }

    #[test]
    fn the_constant_zero() {
        let mut e = Engine::new();
        assert_eq!(fn_00e60670(&mut e), 0);
    }

    #[test]
    fn next_index_or_minus_one() {
        let mut e = Engine::new();
        let m = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(m.addr() + 0x14, 5);
        assert_eq!(fn_00e60680(&mut e, m, 2), 3);
        assert_eq!(fn_00e60680(&mut e, m, 4), -1);
        assert_eq!(fn_00e60680(&mut e, m, 9), -1);
    }

    #[test]
    fn call_through_the_owners_table() {
        let mut e = Engine::new();
        logging(&mut e);
        fake(&mut e, 0x7500_1000, 0);
        let (owner, table) = (e.mem.alloc(0x40), e.mem.alloc(0x40));
        e.mem.set_u32(owner + 0x10, table);
        e.mem.set_u32(table + 0x14, 0x7500_1000);
        let array = e.mem.alloc(16);
        for k in 0..4 {
            e.mem.set_u32(array + 4 * k, 100 + k);
        }
        let this = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr() + 0xc, owner);
        e.mem.set_u32(this.addr() + 0x10, array);
        fn_00e60690(&mut e, this, 2, 55);
        let seen = calls(&e);
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].1, vec![owner + 0x10, 102, 55]);
    }

    #[test]
    fn call_through_the_owners_table_answers_its_eax() {
        let mut e = Engine::new();
        e.register(0x7500_1000, |_, _| 0x0abc_0000u32.into_ret());
        let (owner, table) = (e.mem.alloc(0x40), e.mem.alloc(0x40));
        e.mem.set_u32(owner + 0x10, table);
        e.mem.set_u32(table + 0x14, 0x7500_1000);
        let array = e.mem.alloc(16);
        let this = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr() + 0xc, owner);
        e.mem.set_u32(this.addr() + 0x10, array);
        assert_eq!(fn_00e60690(&mut e, this, 2, 55), 0x0abc_0000);
    }

    #[test]
    fn copy_the_vector_the_member_points_at() {
        let mut e = Engine::new();
        let source = vector(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let this = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr() + 0x18, source.addr());
        let out = vector(&mut e, [0.0; 4]);
        fn_00e606c0(&mut e, this, out);
        assert_eq!(lanes(&e, out), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn copy_all_vertices_with_the_radius_in_the_fourth_lane() {
        let mut e = Engine::new();
        let points = vector_record(&mut e, &[[1.0, 2.0, 3.0, 9.0], [4.0, 5.0, 6.0, 9.0]]);
        let this = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr() + 0x18, e.mem.u32(points.addr()));
        e.mem.set_u32(this.addr() + 0x1c, 2);
        e.mem.set_f32(this.addr() + 0x10, 0.25);
        let out = e.mem.alloc(48);
        fn_00e606e0(&mut e, this, Ptr::new(out));
        assert_eq!(lanes(&e, Ptr::new(out)), [1.0, 2.0, 3.0, 0.25]);
        assert_eq!(lanes(&e, Ptr::new(out + 16)), [4.0, 5.0, 6.0, 0.25]);
        assert_eq!(e.mem.u32(out + 32), 0);
        // no vertices, nothing written
        e.mem.set_u32(this.addr() + 0x1c, 0);
        fn_00e606e0(&mut e, this, Ptr::new(out + 32));
        assert_eq!(e.mem.u32(out + 32), 0);
    }

    /// A shape-collection object: the owner at `+0x20` (whose sub-object at `+0x10` has a table
    /// with the block fetcher at `+0x14`), the block ids at `+0x24` and their count at `+0x28`.
    fn shapes(e: &mut Engine, fetcher: u32, blocks: &[u32]) -> Ptr {
        let (owner, table) = (e.mem.alloc(0x40), e.mem.alloc(0x40));
        e.mem.set_u32(owner + 0x10, table);
        e.mem.set_u32(table + 0x14, fetcher);
        let ids = e.mem.alloc(4 * blocks.len() as u32 + 8);
        for (k, b) in blocks.iter().enumerate() {
            e.mem.set_u32(ids + 4 * k as u32, *b);
        }
        let this = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr() + 0x20, owner);
        e.mem.set_u32(this.addr() + 0x24, ids);
        e.mem.set_u32(this.addr() + 0x28, blocks.len() as u32);
        this
    }

    #[test]
    fn vertices_by_index() {
        let mut e = Engine::new();
        logging(&mut e);
        // the fetcher answers a buffer of six vectors
        let buffer = e.mem.alloc(16 * 6);
        for v in 0..6u32 {
            for l in 0..4u32 {
                e.mem.set_f32(buffer + 16 * v + 4 * l, (10 * v + l) as f32);
            }
        }
        fake(&mut e, 0x7500_1000, buffer);
        let this = shapes(&mut e, 0x7500_1000, &[500, 501, 502]);
        let indexes = e.mem.alloc(8);
        e.mem.set_u16(indexes, 4);
        e.mem.set_u16(indexes + 2, 8);
        let out = e.mem.alloc(32);
        fn_00e60730(&mut e, this, Ptr::new(indexes), 2, Ptr::new(out));
        // index 4: block 1 (id 501), vertex 4 % 3 + 2 = 3; index 8: block 2, vertex 4
        assert_eq!(lanes(&e, Ptr::new(out))[..3], [30.0, 31.0, 32.0]);
        assert_eq!(e.mem.u32(out + 12), 4 | 0x3f00_0000);
        assert_eq!(lanes(&e, Ptr::new(out + 16))[..3], [40.0, 41.0, 42.0]);
        assert_eq!(e.mem.u32(out + 28), 8 | 0x3f00_0000);
        let seen = calls(&e);
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].1[1], 501);
        assert_eq!(seen[1].1[1], 502);
        // a count of zero does nothing
        fn_00e60730(&mut e, this, Ptr::new(indexes), 0, Ptr::new(out));
        assert_eq!(calls(&e).len(), 2);
    }

    #[test]
    fn member_initialisation() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(0x40));
        assert_eq!(fn_00e607c0(&mut e, this, 1.5), this);
        let base = this.addr();
        assert_eq!(e.mem.f32(base + 0x10), 1.5);
        assert_eq!(e.mem.u16(base + 6), 1);
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(e.mem.u32(base + 0xc), 0x14);
        assert_eq!(e.mem.u32(base + 0x14), 0x010e_ce08);
        assert_eq!(e.mem.u32(base), 0x010e_ce2c);
        e.call(0x00e6_07c0, &args![this, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(base + 0x10), 0x7fc0_0001);
    }

    #[test]
    fn bounding_box_of_the_transformed_vertices() {
        let mut e = Engine::new();
        constants(&mut e);
        let points = vector_record(&mut e, &[[1.0, 2.0, 3.0, 0.0], [-1.0, 0.0, 5.0, 0.0]]);
        let this = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr() + 0x18, e.mem.u32(points.addr()));
        e.mem.set_u32(this.addr() + 0x1c, 2);
        e.mem.set_f32(this.addr() + 0x10, 0.25);
        let rotation = e.mem.alloc(64);
        let rows = [
            [1.0f32, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0; 4],
        ];
        for (row, v) in rows.iter().enumerate() {
            for (l, x) in v.iter().enumerate() {
                e.mem.set_f32(rotation + 16 * row as u32 + 4 * l as u32, *x);
            }
        }
        let bounds = e.mem.alloc(32);
        fn_00e60800(&mut e, this, Ptr::new(rotation), 0.5, Ptr::new(bounds));
        assert_eq!(lanes(&e, Ptr::new(bounds)), [-1.75, -0.75, 2.25, -0.75]);
        assert_eq!(lanes(&e, Ptr::new(bounds + 16)), [1.75, 2.75, 5.75, 0.75]);
    }

    #[test]
    fn support_piece_in_a_direction() {
        let mut e = Engine::new();
        logging(&mut e);
        constants(&mut e);
        // the owner's fetcher answers an object whose method at +0x38 writes a vertex into its
        // buffer
        let (object, object_table) = (e.mem.alloc(0x40), e.mem.alloc(0x80));
        e.mem.set_u32(object, object_table);
        e.mem.set_u32(object_table + 0x38, 0x7500_1010);
        fake(&mut e, 0x7500_1000, object);
        e.register(0x7500_1010, |e, a| {
            // first call (0, 1, 0, 0) with id word 0x20, second (0, 3, 0, 0) with id word 0x10
            let calls_so_far = e.mem.u32(LOG + 0x800);
            e.mem.set_u32(LOG + 0x800, calls_so_far + 1);
            let (y, id) = if calls_so_far == 0 {
                (1.0f32, 0x20u32)
            } else {
                (3.0, 0x10)
            };
            e.mem.set_f32(a[2], 0.0);
            e.mem.set_f32(a[2] + 4, y);
            e.mem.set_f32(a[2] + 8, 0.0);
            e.mem.set_u32(a[2] + 12, id);
            Ret::default()
        });
        let this = shapes(&mut e, 0x7500_1000, &[1, 2]);
        let direction = vector(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let out = vector(&mut e, [9.0, 9.0, 9.0, 9.0]);
        fn_00e608d0(&mut e, this, direction, out);
        // the second piece has the larger dot product: its vertex, and its id word 0x10
        // (divided by 16) plus its position 3
        assert_eq!(lanes(&e, out)[..3], [0.0, 3.0, 0.0]);
        assert_eq!(e.mem.u32(out.addr() + 12), (1 + 3) | 0x3f00_0000);
    }

    const STREAM: u32 = 0x7600_0000;

    /// The monitor stream the profiling records go to: `TlsGetValue` (import slot `00fdf044`)
    /// answers it for the index at `0126835c`; its cursor (`+4`) is below its limit (`+0xc`).
    fn monitor(e: &mut Engine) {
        e.map(0x0126_8000, 0x1000);
        e.mem.set_u32(0x0126_835c, 7);
        e.map(STREAM, 0x1000);
        e.mem.set_u32(STREAM + 4, STREAM + 0x40);
        e.mem.set_u32(STREAM + 0xc, STREAM + 0x100);
        e.register(0x00fd_f044, |_, a| {
            assert_eq!(a[0], 7);
            Ret {
                eax: STREAM,
                ..Ret::default()
            }
        });
    }

    /// Pieces whose owner's fetcher (`0x7500_1000`) answers an object with the piece tests at
    /// `+0x20` (`0x7500_1010`) and `+0x24` (`0x7500_1020`).
    fn piece_world(e: &mut Engine, pieces: &[u32]) -> (Ptr, Ptr, Ptr) {
        let (object, object_table) = (e.mem.alloc(0x40), e.mem.alloc(0x80));
        e.mem.set_u32(object, object_table);
        e.mem.set_u32(object_table + 0x20, 0x7500_1010);
        e.mem.set_u32(object_table + 0x24, 0x7500_1020);
        fake(e, 0x7500_1000, object);
        let this = shapes(e, 0x7500_1000, pieces);
        let first = Ptr::new(e.mem.alloc(0x40));
        let second = Ptr::new(e.mem.alloc(0x80));
        (this, first, second)
    }

    #[test]
    fn query_over_the_pieces_records_the_last_that_answers() {
        let mut e = Engine::new();
        logging(&mut e);
        monitor(&mut e);
        let (this, first, second) = piece_world(&mut e, &[10, 20, 30]);
        // the piece test writes its answer to the byte at a[1]: true for the second piece only
        e.register(0x7500_1010, |e, a| {
            let n = e.mem.u32(LOG + 0x800);
            e.mem.set_u32(LOG + 0x800, n + 1);
            e.mem.set_u8(a[1], u8::from(n == 1));
            Ret::default()
        });
        let result = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(second.addr() + 0x40, 0);
        fn_00e609c0(&mut e, this, result, first, second);
        assert_eq!(e.mem.u8(result.addr()), 1);
        // the counter went up and down again; the index was stored where it stood
        assert_eq!(e.mem.u32(second.addr() + 0x40), 0);
        assert_eq!(e.mem.u32(second.addr() + 0x20), 1);
        // two profiling records of 12 bytes
        assert_eq!(e.mem.u32(STREAM + 4), STREAM + 0x40 + 24);
        assert_eq!(e.mem.u32(STREAM + 0x40), 0x010e_ce74);
        assert_eq!(e.mem.u32(STREAM + 0x4c), 0x010c_41f8);
    }

    #[test]
    fn query_over_the_pieces_answers_its_result_pointer() {
        let mut e = Engine::new();
        logging(&mut e);
        monitor(&mut e);
        let (this, first, second) = piece_world(&mut e, &[10, 20, 30]);
        e.register(0x7500_1010, |e, a| {
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        let result = Ptr::new(e.mem.alloc(8));
        // EAX is `result` at the RET (the hkBool result pointer the caller reads through).
        assert_eq!(fn_00e609c0(&mut e, this, result, first, second), result);
    }

    #[test]
    fn query_over_the_pieces_without_an_answer() {
        let mut e = Engine::new();
        logging(&mut e);
        monitor(&mut e);
        let (this, first, second) = piece_world(&mut e, &[10, 20]);
        e.register(0x7500_1010, |e, a| {
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        e.mem.set_u32(second.addr() + 0x20, 0x77);
        let result = Ptr::new(e.mem.alloc(8));
        fn_00e609c0(&mut e, this, result, first, second);
        assert_eq!(e.mem.u8(result.addr()), 0);
        assert_eq!(e.mem.u32(second.addr() + 0x20), 0x77);
    }

    #[test]
    fn a_filter_that_refuses_keeps_the_piece_from_being_asked() {
        let mut e = Engine::new();
        logging(&mut e);
        monitor(&mut e);
        let (this, first, second) = piece_world(&mut e, &[10, 20]);
        // the filter object at first + 0x24: its method at slot 0 answers the address of a flag
        let (filter, filter_table) = (e.mem.alloc(0x20), e.mem.alloc(0x20));
        e.mem.set_u32(filter, filter_table);
        e.mem.set_u32(filter_table, 0x7500_1030);
        e.mem.set_u32(first.addr() + 0x24, filter);
        e.register(0x7500_1030, |_, _| Ret {
            eax: 0x7600_0800,
            ..Ret::default()
        });
        e.map(0x7600_0800, 16);
        e.register(0x7500_1010, |e, a| {
            e.mem.set_u8(a[1], 1);
            e.mem.set_u32(LOG + 0x900, 1);
            Ret::default()
        });
        let result = Ptr::new(e.mem.alloc(8));
        fn_00e609c0(&mut e, this, result, first, second);
        assert_eq!(e.mem.u8(result.addr()), 0);
        assert_eq!(e.mem.u32(LOG + 0x900), 0);
        // once the filter accepts, the piece is asked
        e.mem.set_u8(0x7600_0800, 1);
        fn_00e609c0(&mut e, this, result, first, second);
        assert_eq!(e.mem.u8(result.addr()), 1);
        assert_eq!(e.mem.u32(LOG + 0x900), 1);
    }

    #[test]
    fn query_over_the_pieces_with_a_callback() {
        let mut e = Engine::new();
        logging(&mut e);
        monitor(&mut e);
        let (this, first, second) = piece_world(&mut e, &[10, 20]);
        e.mem.set_u32(second.addr() + 8, 0x1234);
        e.register(0x7500_1020, |e, a| {
            // a = (object, first, record, third): note the record's words
            let n = e.mem.u32(LOG + 0xa00);
            e.mem.set_u32(LOG + 0xa04 + 20 * n, a[2]);
            for k in 0..4 {
                e.mem
                    .set_u32(LOG + 0xa08 + 20 * n + 4 * k, e.mem.u32(a[2] + 4 * k));
            }
            e.mem.set_u32(LOG + 0xa00, n + 1);
            e.mem.set_u32(LOG + 0xa80 + 4 * n, a[3]);
            Ret::default()
        });
        let third = e.mem.alloc(8);
        fn_00e60b00(&mut e, this, first, second, third);
        assert_eq!(e.mem.u32(LOG + 0xa00), 2);
        for n in 0..2u32 {
            // record: the object the fetcher answered, the piece position, second[8], second
            let record = LOG + 0xa08 + 20 * n;
            assert_ne!(e.mem.u32(record), 0);
            assert_eq!(e.mem.u32(record + 4), n);
            assert_eq!(e.mem.u32(record + 8), 0x1234);
            assert_eq!(e.mem.u32(record + 12), second.addr());
            assert_eq!(e.mem.u32(LOG + 0xa80 + 4 * n), third);
        }
        assert_eq!(e.mem.u32(STREAM + 4), STREAM + 0x40 + 24);
    }

    #[test]
    fn deleting_destructor_through_the_base_pointer() {
        let mut e = Engine::new();
        heap(&mut e);
        logging(&mut e);
        fake(&mut e, BLOCK_FREE, 0);
        let object = e.mem.alloc(0x40);
        e.mem.set_u16(object + 4, 0x30);
        let this = Ptr::new(object + 0x14);
        assert_eq!(fn_00e60c20(&mut e, this, 0).addr(), object);
        assert_eq!(e.mem.u32(object + 0x14), 0x010c_6fb0);
        assert_eq!(e.mem.u32(object), 0x0102_e388);
        assert!(calls(&e).is_empty());
        assert_eq!(fn_00e60c20(&mut e, this, 1).addr(), object);
        let seen = calls(&e);
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].1, vec![ALLOCATOR, object, 0x30]);
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_00e60c30_stores_the_vtables_and_frees_with_the_size() {
        let mut e = Engine::new();
        let mut slots = [0u32; 4];
        slots[2] = 0x0f00_0008;
        e.put_vtable(0x0f00_1000, &slots);
        e.register(0x0f00_0008, |_, _| Ret::default());
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, 0x0f00_1000);
        let router = e.mem.alloc(0x14);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER_00C85750, move |_, _| router.into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 4, 0x30);
        assert_eq!(e.call(0x00e6_0c30, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 0x14), 0x010c_6fb0);
        assert_eq!(e.call(0x00e6_0c30, &args![this, 1u32]).u32(), this);
        let freed: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0f00_0008)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(freed, vec![vec![allocator, this, 0x30]]);
    }
}
