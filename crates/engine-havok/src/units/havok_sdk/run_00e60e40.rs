//! `Havok SDK/run_00e60e40` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00e53090::{Flags, Fpu};

#[allow(
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_parens,
    unreachable_code,
    clippy::never_loop
)]
// Translated from 00e60e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Index list for a strip or fan of `count` vertices (cdecl: `mode`, `count`, `first`, `out` (an
/// hkArray-like record: data pointer, size, capacity word)): mode 3 appends `count` indexes
/// `(first + i) & 0xffff`; mode 4 appends `count * 3 - 6` indexes, three per triangle `(first + k,
/// first + k + 1, first + k + 2)` of a strip with the last two swapped on odd triangles; any other
/// mode does nothing. The array grows through `hkArrayUtil::_reserve` (the allocator router
/// `00c85750`) when its capacity is short. Transcribed instruction by instruction.
pub fn fn_00e60e40(e: &mut Engine, mode: i32, count: i32, first: i32, out: Ptr) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), mode as u32);
        e.mem.set_u32(esp.wrapping_add(8), count as u32);
        e.mem.set_u32(esp.wrapping_add(12), first as u32);
        e.mem.set_u32(esp.wrapping_add(16), out.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e60e40;
        loop {
            match pc {
                0x00e60e40 => {
                    eax = e.mem.u32(esp.wrapping_add(0x4)); // 00e60e40
                    {
                        let r = fl.sub(eax, 0x00000003u32, 32);
                        eax = r;
                    } // 00e60e44
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e60e47
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e60e48
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60e49
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e60e4a
                    if fl.zf {
                        pc = 0x00e60ee8;
                        continue;
                    } // 00e60e4b
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e60e51
                    if !fl.zf {
                        pc = 0x00e60f3f;
                        continue;
                    } // 00e60e54
                    ebp = e.mem.u32(esp.wrapping_add(0x18)); // 00e60e5a
                    edi = ebp.wrapping_add(ebp.wrapping_mul(2)).wrapping_sub(0x6); // 00e60e5e
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e60e62
                    esi = e.mem.u32(esp.wrapping_add(0x20)); // 00e60e67
                    edx = e.mem.u32(eax.wrapping_add(0x10)); // 00e60e6b
                    ebx = e.mem.u32(esi.wrapping_add(0x4)); // 00e60e6e
                    eax = e.mem.u32(esi.wrapping_add(0x8)); // 00e60e71
                    ecx = ebx.wrapping_add(edi); // 00e60e74
                    {
                        let r = fl.and(eax, 0x3fffffffu32, 32);
                        eax = r;
                    } // 00e60e77
                    fl.sub(eax, ecx, 32); // 00e60e7c
                    if fl.sf == fl.of {
                        pc = 0x00e60e95;
                        continue;
                    } // 00e60e7e
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    } // 00e60e80
                    fl.sub(ecx, eax, 32); // 00e60e82
                    if fl.sf == fl.of {
                        pc = 0x00e60e88;
                        continue;
                    } // 00e60e84
                    ecx = eax; // 00e60e86
                    pc = 0x00e60e88;
                }
                0x00e60e88 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000004u32); // 00e60e88
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e60e8a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60e8b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e60e8c
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                        ];
                        let r = e.call(0x00c908c0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e60e8d
                    {
                        let r = fl.add(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e60e92
                    pc = 0x00e60e95;
                }
                0x00e60e95 => {
                    {
                        let r = fl.add(e.mem.u32(esi.wrapping_add(0x4)), edi, 32);
                        e.mem.set_u32(esi.wrapping_add(0x4), r);
                    } // 00e60e95
                    fl.sub(ebp, 0x00000002u32, 32); // 00e60e98
                    eax = e.mem.u32(esi); // 00e60e9b
                    eax = eax.wrapping_add(ebx.wrapping_mul(4)); // 00e60e9d
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e60f3f;
                        continue;
                    } // 00e60ea0
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e60ea6
                    esi = 0x00000001u32; // 00e60eaa
                    ecx = edx.wrapping_add(0x1); // 00e60eaf
                    {
                        let r = fl.sub(esi, edx, 32);
                        esi = r;
                    } // 00e60eb2
                    pc = 0x00e60eb4;
                }
                0x00e60eb4 => {
                    edx = ecx.wrapping_sub(0x1); // 00e60eb4
                    e.mem.set_u32(eax, edx); // 00e60eb7
                    edx = esi.wrapping_add(ecx); // 00e60eb9
                    {
                        let r = fl.add(eax, 0x00000004u32, 32);
                        eax = r;
                    } // 00e60ebc
                    fl.and((edx & 0xff), 0x00000001u32, 8); // 00e60ebf
                    edx = ecx.wrapping_add(0x1); // 00e60ec2
                    if !fl.zf {
                        pc = 0x00e60ed0;
                        continue;
                    } // 00e60ec5
                    e.mem.set_u32(eax, ecx); // 00e60ec7
                    {
                        let r = fl.add(eax, 0x00000004u32, 32);
                        eax = r;
                    } // 00e60ec9
                    e.mem.set_u32(eax, edx); // 00e60ecc
                    pc = 0x00e60ed7; // 00e60ece
                    continue;
                }
                0x00e60ed0 => {
                    e.mem.set_u32(eax, edx); // 00e60ed0
                    {
                        let r = fl.add(eax, 0x00000004u32, 32);
                        eax = r;
                    } // 00e60ed2
                    e.mem.set_u32(eax, ecx); // 00e60ed5
                    pc = 0x00e60ed7;
                }
                0x00e60ed7 => {
                    ecx = edx; // 00e60ed7
                    edx = esi.wrapping_add(ecx); // 00e60ed9
                    {
                        let r = fl.add(eax, 0x00000004u32, 32);
                        eax = r;
                    } // 00e60edc
                    fl.sub(edx, ebp, 32); // 00e60edf
                    if fl.sf != fl.of {
                        pc = 0x00e60eb4;
                        continue;
                    } // 00e60ee1
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60ee3
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60ee4
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60ee5
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60ee6
                    return; // 00e60ee7
                }
                0x00e60ee8 => {
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e60ee8
                    esi = e.mem.u32(esp.wrapping_add(0x20)); // 00e60eed
                    edx = e.mem.u32(eax.wrapping_add(0x10)); // 00e60ef1
                    ebx = e.mem.u32(esi.wrapping_add(0x4)); // 00e60ef4
                    edi = e.mem.u32(esp.wrapping_add(0x18)); // 00e60ef7
                    eax = e.mem.u32(esi.wrapping_add(0x8)); // 00e60efb
                    ecx = ebx.wrapping_add(edi); // 00e60efe
                    {
                        let r = fl.and(eax, 0x3fffffffu32, 32);
                        eax = r;
                    } // 00e60f01
                    fl.sub(eax, ecx, 32); // 00e60f06
                    if fl.sf == fl.of {
                        pc = 0x00e60f1f;
                        continue;
                    } // 00e60f08
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    } // 00e60f0a
                    fl.sub(ecx, eax, 32); // 00e60f0c
                    if fl.sf != fl.of {
                        pc = 0x00e60f12;
                        continue;
                    } // 00e60f0e
                    eax = ecx; // 00e60f10
                    pc = 0x00e60f12;
                }
                0x00e60f12 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000004u32); // 00e60f12
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e60f14
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60f15
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e60f16
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                        ];
                        let r = e.call(0x00c908c0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e60f17
                    {
                        let r = fl.add(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e60f1c
                    pc = 0x00e60f1f;
                }
                0x00e60f1f => {
                    eax = e.mem.u32(esi); // 00e60f1f
                    {
                        let r = fl.add(e.mem.u32(esi.wrapping_add(0x4)), edi, 32);
                        e.mem.set_u32(esi.wrapping_add(0x4), r);
                    } // 00e60f21
                    ecx = eax.wrapping_add(ebx.wrapping_mul(4)); // 00e60f24
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    } // 00e60f27
                    fl.and(edi, edi, 32); // 00e60f29
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e60f3f;
                        continue;
                    } // 00e60f2b
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e60f2d
                    pc = 0x00e60f31;
                }
                0x00e60f31 => {
                    esi = edx.wrapping_add(eax); // 00e60f31
                    esi = (esi & 0xffff); // 00e60f34
                    e.mem.set_u32(ecx.wrapping_add(eax.wrapping_mul(4)), esi); // 00e60f37
                    {
                        let r = fl.inc(eax, 32);
                        eax = r;
                    } // 00e60f3a
                    fl.sub(eax, edi, 32); // 00e60f3b
                    if fl.sf != fl.of {
                        pc = 0x00e60f31;
                        continue;
                    } // 00e60f3d
                    pc = 0x00e60f3f;
                }
                0x00e60f3f => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60f3f
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60f40
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60f41
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60f42
                    return; // 00e60f43
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
// Translated from 00e60f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a list of 32-bit vertex indexes (cdecl: `mode`, `indexes` (the array itself), `count`,
/// `offset`, `out` (an hkArray-like record of words)): mode 3 appends `indexes[0..count]`, each plus
/// `offset` (a plain `hkString::memCpy` (`00cb1260`) when `offset` is 0); mode 4 treats the indexes as a
/// strip and appends `count * 3 - 6` words, the triangle `(i, i + 1, i + 2)` plus `offset` for each
/// start `i`, the last two swapped on every second triangle; any other mode does nothing. The
/// array grows through `hkArrayUtil::_reserve` (the allocator router `00c85750`) when its capacity
/// is short. Transcribed instruction by instruction.
pub fn fn_00e60f50(e: &mut Engine, mode: i32, indexes: Ptr, count: i32, offset: i32, out: Ptr) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), mode as u32);
        e.mem.set_u32(esp.wrapping_add(8), indexes.addr());
        e.mem.set_u32(esp.wrapping_add(12), count as u32);
        e.mem.set_u32(esp.wrapping_add(16), offset as u32);
        e.mem.set_u32(esp.wrapping_add(20), out.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e60f50;
        loop {
            match pc {
                0x00e60f50 => {
                    eax = e.mem.u32(esp.wrapping_add(0x4)); // 00e60f50
                    {
                        let r = fl.sub(eax, 0x00000003u32, 32);
                        eax = r;
                    } // 00e60f54
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e60f57
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e60f58
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60f59
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e60f5a
                    if fl.zf {
                        pc = 0x00e60fff;
                        continue;
                    } // 00e60f5b
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e60f61
                    if !fl.zf {
                        pc = 0x00e6107f;
                        continue;
                    } // 00e60f64
                    ebp = e.mem.u32(esp.wrapping_add(0x1c)); // 00e60f6a
                    edi = ebp.wrapping_add(ebp.wrapping_mul(2)).wrapping_sub(0x6); // 00e60f6e
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e60f72
                    esi = e.mem.u32(esp.wrapping_add(0x24)); // 00e60f77
                    edx = e.mem.u32(eax.wrapping_add(0x10)); // 00e60f7b
                    ebx = e.mem.u32(esi.wrapping_add(0x4)); // 00e60f7e
                    eax = e.mem.u32(esi.wrapping_add(0x8)); // 00e60f81
                    ecx = ebx.wrapping_add(edi); // 00e60f84
                    {
                        let r = fl.and(eax, 0x3fffffffu32, 32);
                        eax = r;
                    } // 00e60f87
                    fl.sub(eax, ecx, 32); // 00e60f8c
                    if fl.sf == fl.of {
                        pc = 0x00e60fa5;
                        continue;
                    } // 00e60f8e
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    } // 00e60f90
                    fl.sub(ecx, eax, 32); // 00e60f92
                    if fl.sf == fl.of {
                        pc = 0x00e60f98;
                        continue;
                    } // 00e60f94
                    ecx = eax; // 00e60f96
                    pc = 0x00e60f98;
                }
                0x00e60f98 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000004u32); // 00e60f98
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e60f9a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e60f9b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e60f9c
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                        ];
                        let r = e.call(0x00c908c0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e60f9d
                    {
                        let r = fl.add(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e60fa2
                    pc = 0x00e60fa5;
                }
                0x00e60fa5 => {
                    {
                        let r = fl.add(e.mem.u32(esi.wrapping_add(0x4)), edi, 32);
                        e.mem.set_u32(esi.wrapping_add(0x4), r);
                    } // 00e60fa5
                    eax = e.mem.u32(esi); // 00e60fa8
                    edx = 0x00000002u32; // 00e60faa
                    fl.sub(ebp, edx, 32); // 00e60faf
                    eax = eax.wrapping_add(ebx.wrapping_mul(4)); // 00e60fb1
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e6107f;
                        continue;
                    } // 00e60fb4
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e60fba
                    esi = e.mem.u32(esp.wrapping_add(0x20)); // 00e60fbe
                    {
                        let r = fl.add(ecx, 0x00000004u32, 32);
                        ecx = r;
                    } // 00e60fc2
                    pc = 0x00e60fc5;
                }
                0x00e60fc5 => {
                    edi = e.mem.u32(ecx.wrapping_sub(0x4)); // 00e60fc5
                    {
                        let r = fl.add(edi, esi, 32);
                        edi = r;
                    } // 00e60fc8
                    e.mem.set_u32(eax, edi); // 00e60fca
                    {
                        let r = fl.add(eax, 0x00000004u32, 32);
                        eax = r;
                    } // 00e60fcc
                    fl.and((edx & 0xff), 0x00000001u32, 8); // 00e60fcf
                    if !fl.zf {
                        pc = 0x00e60fdf;
                        continue;
                    } // 00e60fd2
                    edi = e.mem.u32(ecx); // 00e60fd4
                    {
                        let r = fl.add(edi, esi, 32);
                        edi = r;
                    } // 00e60fd6
                    e.mem.set_u32(eax, edi); // 00e60fd8
                    edi = e.mem.u32(ecx.wrapping_add(0x4)); // 00e60fda
                    pc = 0x00e60fe8; // 00e60fdd
                    continue;
                }
                0x00e60fdf => {
                    edi = e.mem.u32(ecx.wrapping_add(0x4)); // 00e60fdf
                    {
                        let r = fl.add(edi, esi, 32);
                        edi = r;
                    } // 00e60fe2
                    e.mem.set_u32(eax, edi); // 00e60fe4
                    edi = e.mem.u32(ecx); // 00e60fe6
                    pc = 0x00e60fe8;
                }
                0x00e60fe8 => {
                    {
                        let r = fl.add(eax, 0x00000004u32, 32);
                        eax = r;
                    } // 00e60fe8
                    {
                        let r = fl.add(edi, esi, 32);
                        edi = r;
                    } // 00e60feb
                    e.mem.set_u32(eax, edi); // 00e60fed
                    {
                        let r = fl.inc(edx, 32);
                        edx = r;
                    } // 00e60fef
                    {
                        let r = fl.add(eax, 0x00000004u32, 32);
                        eax = r;
                    } // 00e60ff0
                    {
                        let r = fl.add(ecx, 0x00000004u32, 32);
                        ecx = r;
                    } // 00e60ff3
                    fl.sub(edx, ebp, 32); // 00e60ff6
                    if fl.sf != fl.of {
                        pc = 0x00e60fc5;
                        continue;
                    } // 00e60ff8
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60ffa
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60ffb
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60ffc
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e60ffd
                    return; // 00e60ffe
                }
                0x00e60fff => {
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e60fff
                    esi = e.mem.u32(esp.wrapping_add(0x24)); // 00e61004
                    edx = e.mem.u32(eax.wrapping_add(0x10)); // 00e61008
                    ebx = e.mem.u32(esi.wrapping_add(0x4)); // 00e6100b
                    edi = e.mem.u32(esp.wrapping_add(0x1c)); // 00e6100e
                    eax = e.mem.u32(esi.wrapping_add(0x8)); // 00e61012
                    ecx = ebx.wrapping_add(edi); // 00e61015
                    {
                        let r = fl.and(eax, 0x3fffffffu32, 32);
                        eax = r;
                    } // 00e61018
                    fl.sub(eax, ecx, 32); // 00e6101d
                    if fl.sf == fl.of {
                        pc = 0x00e61036;
                        continue;
                    } // 00e6101f
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    } // 00e61021
                    fl.sub(ecx, eax, 32); // 00e61023
                    if fl.sf == fl.of {
                        pc = 0x00e61029;
                        continue;
                    } // 00e61025
                    ecx = eax; // 00e61027
                    pc = 0x00e61029;
                }
                0x00e61029 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000004u32); // 00e61029
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e6102b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e6102c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e6102d
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                        ];
                        let r = e.call(0x00c908c0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e6102e
                    {
                        let r = fl.add(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e61033
                    pc = 0x00e61036;
                }
                0x00e61036 => {
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e61036
                    {
                        let r = fl.add(e.mem.u32(esi.wrapping_add(0x4)), edi, 32);
                        e.mem.set_u32(esi.wrapping_add(0x4), r);
                    } // 00e6103a
                    ecx = e.mem.u32(esi); // 00e6103d
                    eax = ecx.wrapping_add(ebx.wrapping_mul(4)); // 00e6103f
                    fl.and(edx, edx, 32); // 00e61042
                    if !fl.zf {
                        pc = 0x00e61061;
                        continue;
                    } // 00e61044
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e61046
                    edx = edi.wrapping_mul(4); // 00e6104a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e61051
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e61052
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e61053
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00cb1260, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e61054
                    {
                        let r = fl.add(esp, 0x0000000cu32, 32);
                        esp = r;
                    } // 00e61059
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6105c
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6105d
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6105e
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6105f
                    return; // 00e61060
                }
                0x00e61061 => {
                    fl.and(edi, edi, 32); // 00e61061
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e6107f;
                        continue;
                    } // 00e61063
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e61065
                    {
                        let r = fl.sub(ecx, eax, 32);
                        ecx = r;
                    } // 00e61069
                    pc = 0x00e61070; // 00e6106b
                    continue;
                }
                0x00e61070 => {
                    esi = e.mem.u32(ecx.wrapping_add(eax)); // 00e61070
                    {
                        let r = fl.add(esi, edx, 32);
                        esi = r;
                    } // 00e61073
                    e.mem.set_u32(eax, esi); // 00e61075
                    {
                        let r = fl.add(eax, 0x00000004u32, 32);
                        eax = r;
                    } // 00e61077
                    {
                        let r = fl.sub(edi, 0x00000001u32, 32);
                        edi = r;
                    } // 00e6107a
                    if !fl.zf {
                        pc = 0x00e61070;
                        continue;
                    } // 00e6107d
                    pc = 0x00e6107f;
                }
                0x00e6107f => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6107f
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61080
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61081
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61082
                    return; // 00e61083
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
// Translated from 00e61090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same as `00e60f50` for 16-bit indexes (the array `out` holds 16-bit entries too).
/// Transcribed instruction by instruction.
pub fn fn_00e61090(e: &mut Engine, mode: i32, indexes: Ptr, count: i32, offset: i32, out: Ptr) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), mode as u32);
        e.mem.set_u32(esp.wrapping_add(8), indexes.addr());
        e.mem.set_u32(esp.wrapping_add(12), count as u32);
        e.mem.set_u32(esp.wrapping_add(16), offset as u32);
        e.mem.set_u32(esp.wrapping_add(20), out.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e61090;
        loop {
            match pc {
                0x00e61090 => {
                    eax = e.mem.u32(esp.wrapping_add(0x4)); // 00e61090
                    {
                        let r = fl.sub(eax, 0x00000003u32, 32);
                        eax = r;
                    } // 00e61094
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e61097
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e61098
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e61099
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e6109a
                    if fl.zf {
                        pc = 0x00e6114b;
                        continue;
                    } // 00e6109b
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e610a1
                    if !fl.zf {
                        pc = 0x00e611c5;
                        continue;
                    } // 00e610a4
                    ebp = e.mem.u32(esp.wrapping_add(0x1c)); // 00e610aa
                    edi = ebp.wrapping_add(ebp.wrapping_mul(2)).wrapping_sub(0x6); // 00e610ae
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e610b2
                    esi = e.mem.u32(esp.wrapping_add(0x24)); // 00e610b7
                    edx = e.mem.u32(eax.wrapping_add(0x10)); // 00e610bb
                    ebx = e.mem.u32(esi.wrapping_add(0x4)); // 00e610be
                    eax = e.mem.u32(esi.wrapping_add(0x8)); // 00e610c1
                    ecx = ebx.wrapping_add(edi); // 00e610c4
                    {
                        let r = fl.and(eax, 0x3fffffffu32, 32);
                        eax = r;
                    } // 00e610c7
                    fl.sub(eax, ecx, 32); // 00e610cc
                    if fl.sf == fl.of {
                        pc = 0x00e610e5;
                        continue;
                    } // 00e610ce
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    } // 00e610d0
                    fl.sub(ecx, eax, 32); // 00e610d2
                    if fl.sf == fl.of {
                        pc = 0x00e610d8;
                        continue;
                    } // 00e610d4
                    ecx = eax; // 00e610d6
                    pc = 0x00e610d8;
                }
                0x00e610d8 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000002u32); // 00e610d8
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e610da
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e610db
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e610dc
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                        ];
                        let r = e.call(0x00c908c0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e610dd
                    {
                        let r = fl.add(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e610e2
                    pc = 0x00e610e5;
                }
                0x00e610e5 => {
                    {
                        let r = fl.add(e.mem.u32(esi.wrapping_add(0x4)), edi, 32);
                        e.mem.set_u32(esi.wrapping_add(0x4), r);
                    } // 00e610e5
                    eax = e.mem.u32(esi); // 00e610e8
                    edx = 0x00000002u32; // 00e610ea
                    fl.sub(ebp, edx, 32); // 00e610ef
                    eax = eax.wrapping_add(ebx.wrapping_mul(2)); // 00e610f1
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e611c5;
                        continue;
                    } // 00e610f4
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e610fa
                    esi = e.mem.u32(esp.wrapping_add(0x20)); // 00e610fe
                    {
                        let r = fl.add(ecx, edx, 32);
                        ecx = r;
                    } // 00e61102
                    pc = 0x00e61104;
                }
                0x00e61104 => {
                    edi = (edi & 0xffff_0000)
                        | ((u32::from(e.mem.u16(ecx.wrapping_sub(0x2)))) & 0xffff); // 00e61104
                    {
                        let r = fl.add((edi & 0xffff), (esi & 0xffff), 16);
                        edi = (edi & 0xffff_0000) | ((r) & 0xffff);
                    } // 00e61108
                    e.mem.set_u16(eax, (edi & 0xffff) as u16); // 00e6110b
                    {
                        let r = fl.add(eax, 0x00000002u32, 32);
                        eax = r;
                    } // 00e6110e
                    fl.and((edx & 0xff), 0x00000001u32, 8); // 00e61111
                    if !fl.zf {
                        pc = 0x00e61125;
                        continue;
                    } // 00e61114
                    edi = u32::from(e.mem.u16(ecx)); // 00e61116
                    {
                        let r = fl.add((edi & 0xffff), (esi & 0xffff), 16);
                        edi = (edi & 0xffff_0000) | ((r) & 0xffff);
                    } // 00e61119
                    e.mem.set_u16(eax, (edi & 0xffff) as u16); // 00e6111c
                    edi = u32::from(e.mem.u16(ecx.wrapping_add(0x2))); // 00e6111f
                    pc = 0x00e61132; // 00e61123
                    continue;
                }
                0x00e61125 => {
                    edi = u32::from(e.mem.u16(ecx.wrapping_add(0x2))); // 00e61125
                    {
                        let r = fl.add((edi & 0xffff), (esi & 0xffff), 16);
                        edi = (edi & 0xffff_0000) | ((r) & 0xffff);
                    } // 00e61129
                    e.mem.set_u16(eax, (edi & 0xffff) as u16); // 00e6112c
                    edi = u32::from(e.mem.u16(ecx)); // 00e6112f
                    pc = 0x00e61132;
                }
                0x00e61132 => {
                    {
                        let r = fl.add(eax, 0x00000002u32, 32);
                        eax = r;
                    } // 00e61132
                    {
                        let r = fl.add((edi & 0xffff), (esi & 0xffff), 16);
                        edi = (edi & 0xffff_0000) | ((r) & 0xffff);
                    } // 00e61135
                    e.mem.set_u16(eax, (edi & 0xffff) as u16); // 00e61138
                    {
                        let r = fl.inc(edx, 32);
                        edx = r;
                    } // 00e6113b
                    {
                        let r = fl.add(eax, 0x00000002u32, 32);
                        eax = r;
                    } // 00e6113c
                    {
                        let r = fl.add(ecx, 0x00000002u32, 32);
                        ecx = r;
                    } // 00e6113f
                    fl.sub(edx, ebp, 32); // 00e61142
                    if fl.sf != fl.of {
                        pc = 0x00e61104;
                        continue;
                    } // 00e61144
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61146
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61147
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61148
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61149
                    return; // 00e6114a
                }
                0x00e6114b => {
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e6114b
                    esi = e.mem.u32(esp.wrapping_add(0x24)); // 00e61150
                    edx = e.mem.u32(eax.wrapping_add(0x10)); // 00e61154
                    ebx = e.mem.u32(esi.wrapping_add(0x4)); // 00e61157
                    edi = e.mem.u32(esp.wrapping_add(0x1c)); // 00e6115a
                    eax = e.mem.u32(esi.wrapping_add(0x8)); // 00e6115e
                    ecx = ebx.wrapping_add(edi); // 00e61161
                    {
                        let r = fl.and(eax, 0x3fffffffu32, 32);
                        eax = r;
                    } // 00e61164
                    fl.sub(eax, ecx, 32); // 00e61169
                    if fl.sf == fl.of {
                        pc = 0x00e61182;
                        continue;
                    } // 00e6116b
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    } // 00e6116d
                    fl.sub(ecx, eax, 32); // 00e6116f
                    if fl.sf == fl.of {
                        pc = 0x00e61175;
                        continue;
                    } // 00e61171
                    ecx = eax; // 00e61173
                    pc = 0x00e61175;
                }
                0x00e61175 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000002u32); // 00e61175
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e61177
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e61178
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e61179
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                        ];
                        let r = e.call(0x00c908c0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e6117a
                    {
                        let r = fl.add(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e6117f
                    pc = 0x00e61182;
                }
                0x00e61182 => {
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e61182
                    {
                        let r = fl.add(e.mem.u32(esi.wrapping_add(0x4)), edi, 32);
                        e.mem.set_u32(esi.wrapping_add(0x4), r);
                    } // 00e61186
                    ecx = e.mem.u32(esi); // 00e61189
                    eax = ecx.wrapping_add(ebx.wrapping_mul(2)); // 00e6118b
                    fl.and(edx, edx, 32); // 00e6118e
                    if !fl.zf {
                        pc = 0x00e611a9;
                        continue;
                    } // 00e61190
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e61192
                    edx = edi.wrapping_add(edi); // 00e61196
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e61199
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e6119a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e6119b
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00cb1260, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e6119c
                    {
                        let r = fl.add(esp, 0x0000000cu32, 32);
                        esp = r;
                    } // 00e611a1
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e611a4
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e611a5
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e611a6
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e611a7
                    return; // 00e611a8
                }
                0x00e611a9 => {
                    fl.and(edi, edi, 32); // 00e611a9
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e611c5;
                        continue;
                    } // 00e611ab
                    ecx = e.mem.u32(esp.wrapping_add(0x18)); // 00e611ad
                    {
                        let r = fl.sub(ecx, eax, 32);
                        ecx = r;
                    } // 00e611b1
                    pc = 0x00e611b3;
                }
                0x00e611b3 => {
                    esi = (esi & 0xffff_0000)
                        | ((u32::from(e.mem.u16(ecx.wrapping_add(eax)))) & 0xffff); // 00e611b3
                    {
                        let r = fl.add((esi & 0xffff), (edx & 0xffff), 16);
                        esi = (esi & 0xffff_0000) | ((r) & 0xffff);
                    } // 00e611b7
                    e.mem.set_u16(eax, (esi & 0xffff) as u16); // 00e611ba
                    {
                        let r = fl.add(eax, 0x00000002u32, 32);
                        eax = r;
                    } // 00e611bd
                    {
                        let r = fl.sub(edi, 0x00000001u32, 32);
                        edi = r;
                    } // 00e611c0
                    if !fl.zf {
                        pc = 0x00e611b3;
                        continue;
                    } // 00e611c3
                    pc = 0x00e611c5;
                }
                0x00e611c5 => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e611c5
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e611c6
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e611c7
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e611c8
                    return; // 00e611c9
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
// Translated from 00e61280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the indexes a primitive record describes to `out` (cdecl: `record`, `out` (an hkArray-like
/// record of words)): the byte at `record + 0x14` selects the source: 0 calls `00e60e40(word[+0],
/// word[+8], word[+0xc], out)`; 1 (for modes 3 and 4 only) reads 16-bit indexes: a scratch block
/// is taken from the memory router (`00c85750`: its first word is a bump pointer advanced by
/// `(2 * count + 0x7f) & ~0x7f`), `00e61090` fills it from the address in `record + 0x18`, each entry
/// is pushed on `out` (growing through `00c90940`), the bump pointer is restored and the block
/// freed through the allocator (vtable slot `+0x10`) when it was taken from the heap; 2 (modes 3 and 4)
/// calls `00e60f50` on the address in `record + 0x18`. Transcribed instruction by instruction.
pub fn fn_00e61280(e: &mut Engine, record: Ptr, out: Ptr) {
    e.with_stack(0x300, |e, frame| {
        let mut esp = frame.addr() + 0x200;
        e.mem.set_u32(esp.wrapping_add(4), record.addr());
        e.mem.set_u32(esp.wrapping_add(8), out.addr());
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e61280;
        loop {
            match pc {
                0x00e61280 => {
                    {
                        let r = fl.sub(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e61280
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e61283
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e61284
                    edi = e.mem.u32(esp.wrapping_add(0x1c)); // 00e61285
                    eax = u32::from(e.mem.u8(edi.wrapping_add(0x14))); // 00e61289
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e6128d
                    {
                        let r = fl.sub(eax, ebx, 32);
                        eax = r;
                    } // 00e6128f
                    if fl.zf {
                        pc = 0x00e613c4;
                        continue;
                    } // 00e61291
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e61297
                    if fl.zf {
                        pc = 0x00e612d1;
                        continue;
                    } // 00e6129a
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    } // 00e6129c
                    if !fl.zf {
                        pc = 0x00e613dd;
                        continue;
                    } // 00e6129f
                    eax = u32::from(e.mem.u8(edi)); // 00e612a5
                    ecx = eax.wrapping_sub(0x3); // 00e612a8
                    fl.sub(ecx, 0x00000001u32, 32); // 00e612ab
                    if !fl.cf && !fl.zf {
                        pc = 0x00e613dd;
                        continue;
                    } // 00e612ae
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e612b4
                    ecx = e.mem.u32(edi.wrapping_add(0x8)); // 00e612b8
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e612bb
                    edx = e.mem.u32(edi.wrapping_add(0x18)); // 00e612bc
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e612bf
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e612c0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e612c1
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e612c2
                    {
                        fn_00e60f50(
                            e,
                            e.mem.u32(esp) as i32,
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            e.mem.u32(esp.wrapping_add(8)) as i32,
                            e.mem.u32(esp.wrapping_add(12)) as i32,
                            Ptr::new(e.mem.u32(esp.wrapping_add(16))),
                        );
                    } // 00e612c3
                    {
                        let r = fl.add(esp, 0x00000014u32, 32);
                        esp = r;
                    } // 00e612c8
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e612cb
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e612cc
                    {
                        let r = fl.add(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e612cd
                    return; // 00e612d0
                }
                0x00e612d1 => {
                    eax = u32::from(e.mem.u8(edi)); // 00e612d1
                    {
                        let r = fl.add(eax, 0xfffffffdu32, 32);
                        eax = r;
                    } // 00e612d4
                    fl.sub(eax, 0x00000001u32, 32); // 00e612d7
                    if !fl.cf && !fl.zf {
                        pc = 0x00e613dd;
                        continue;
                    } // 00e612da
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e612e0
                    esi = e.mem.u32(edi.wrapping_add(0x8)); // 00e612e1
                    e.mem.set_u32(esp.wrapping_add(0xc), ebx); // 00e612e4
                    e.mem.set_u32(esp.wrapping_add(0x10), ebx); // 00e612e8
                    e.mem.set_u32(esp.wrapping_add(0x14), 0x80000000u32); // 00e612ec
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e612f4
                    ecx = e.mem.u32(eax); // 00e612f9
                    edx = esi.wrapping_add(esi).wrapping_add(0x7f); // 00e612fb
                    {
                        let r = fl.and(edx, 0xffffff80u32, 32);
                        edx = r;
                    } // 00e612ff
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    } // 00e61302
                    e.mem.set_u32(eax, edx); // 00e61304
                    edx = e.mem.u32(edi.wrapping_add(0x18)); // 00e61306
                    eax = esp.wrapping_add(0xc); // 00e61309
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e6130d
                    eax = u32::from(e.mem.u8(edi)); // 00e6130e
                    e.mem.set_u32(esp.wrapping_add(0x10), ecx); // 00e61311
                    e.mem.set_u32(esp.wrapping_add(0x1c), ecx); // 00e61315
                    ecx = e.mem.u32(edi.wrapping_add(0x8)); // 00e61319
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e6131c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e6131d
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e6131e
                    {
                        let r = fl.or(esi, 0x80000000u32, 32);
                        esi = r;
                    } // 00e6131f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e61325
                    e.mem.set_u32(esp.wrapping_add(0x28), esi); // 00e61326
                    {
                        fn_00e61090(
                            e,
                            e.mem.u32(esp) as i32,
                            Ptr::new(e.mem.u32(esp.wrapping_add(4))),
                            e.mem.u32(esp.wrapping_add(8)) as i32,
                            e.mem.u32(esp.wrapping_add(12)) as i32,
                            Ptr::new(e.mem.u32(esp.wrapping_add(16))),
                        );
                    } // 00e6132a
                    {
                        let r = fl.add(esp, 0x00000014u32, 32);
                        esp = r;
                    } // 00e6132f
                    fl.sub(e.mem.u32(edi.wrapping_add(0x8)), ebx, 32); // 00e61332
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e61380;
                        continue;
                    } // 00e61335
                    esi = e.mem.u32(esp.wrapping_add(0x24)); // 00e61337
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e6133b
                                             // no change (a move of a register onto itself); // 00e6133c
                    pc = 0x00e61340;
                }
                0x00e61340 => {
                    ecx = e.mem.u32(esp.wrapping_add(0x10)); // 00e61340
                    ebp = u32::from(e.mem.u16(ecx.wrapping_add(ebx.wrapping_mul(2)))); // 00e61344
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e61348
                    edx = e.mem.u32(esi.wrapping_add(0x8)); // 00e6134d
                    {
                        let r = fl.and(edx, 0x3fffffffu32, 32);
                        edx = r;
                    } // 00e61350
                    fl.sub(e.mem.u32(esi.wrapping_add(0x4)), edx, 32); // 00e61356
                    if !fl.zf {
                        pc = 0x00e6136a;
                        continue;
                    } // 00e61359
                    eax = e.mem.u32(eax.wrapping_add(0x10)); // 00e6135b
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, 0x00000004u32); // 00e6135e
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e61360
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e61361
                    {
                        let w = [
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                        ];
                        let r = e.call(0x00c90940, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e61362
                    {
                        let r = fl.add(esp, 0x0000000cu32, 32);
                        esp = r;
                    } // 00e61367
                    pc = 0x00e6136a;
                }
                0x00e6136a => {
                    ecx = e.mem.u32(esi.wrapping_add(0x4)); // 00e6136a
                    edx = e.mem.u32(esi); // 00e6136d
                    eax = edx.wrapping_add(ecx.wrapping_mul(4)); // 00e6136f
                    e.mem.set_u32(eax, ebp); // 00e61372
                    {
                        let r = fl.inc(e.mem.u32(esi.wrapping_add(0x4)), 32);
                        e.mem.set_u32(esi.wrapping_add(0x4), r);
                    } // 00e61374
                    {
                        let r = fl.inc(ebx, 32);
                        ebx = r;
                    } // 00e61377
                    fl.sub(ebx, e.mem.u32(edi.wrapping_add(0x8)), 32); // 00e61378
                    if fl.sf != fl.of {
                        pc = 0x00e61340;
                        continue;
                    } // 00e6137b
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e6137d
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6137f
                    pc = 0x00e61380;
                }
                0x00e61380 => {
                    esi = e.mem.u32(esp.wrapping_add(0x18)); // 00e61380
                    fl.sub(esi, e.mem.u32(esp.wrapping_add(0xc)), 32); // 00e61384
                    if !fl.zf {
                        pc = 0x00e6138e;
                        continue;
                    } // 00e61388
                    e.mem.set_u32(esp.wrapping_add(0x10), ebx); // 00e6138a
                    pc = 0x00e6138e;
                }
                0x00e6138e => {
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e6138e
                    e.mem.set_u32(eax, esi); // 00e61393
                    {
                        let r = e.call(0x00c85750, &[]);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e61395
                    ecx = e.mem.u32(eax.wrapping_add(0x10)); // 00e6139a
                    eax = e.mem.u32(esp.wrapping_add(0x14)); // 00e6139d
                    e.mem.set_u32(esp.wrapping_add(0x10), ebx); // 00e613a1
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e613a5
                    fl.and(eax, eax, 32); // 00e613a6
                    if fl.sf {
                        pc = 0x00e613dd;
                        continue;
                    } // 00e613a8
                    edx = e.mem.u32(ecx); // 00e613aa
                    edx = e.mem.u32(edx.wrapping_add(0x10)); // 00e613ac
                    {
                        let r = fl.and(eax, 0x3fffffffu32, 32);
                        eax = r;
                    } // 00e613af
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    } // 00e613b4
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e613b6
                    eax = e.mem.u32(esp.wrapping_add(0xc)); // 00e613b7
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e613bb
                    {
                        let w = [ecx, e.mem.u32(esp), e.mem.u32(esp.wrapping_add(4))];
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(8); // 00e613bc
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e613be
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e613bf
                    {
                        let r = fl.add(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e613c0
                    return; // 00e613c3
                }
                0x00e613c4 => {
                    eax = e.mem.u32(esp.wrapping_add(0x20)); // 00e613c4
                    ecx = e.mem.u32(edi.wrapping_add(0xc)); // 00e613c8
                    edx = e.mem.u32(edi.wrapping_add(0x8)); // 00e613cb
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e613ce
                    eax = u32::from(e.mem.u8(edi)); // 00e613cf
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e613d2
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e613d3
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e613d4
                    {
                        fn_00e60e40(
                            e,
                            e.mem.u32(esp) as i32,
                            e.mem.u32(esp.wrapping_add(4)) as i32,
                            e.mem.u32(esp.wrapping_add(8)) as i32,
                            Ptr::new(e.mem.u32(esp.wrapping_add(12))),
                        );
                    } // 00e613d5
                    {
                        let r = fl.add(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e613da
                    pc = 0x00e613dd;
                }
                0x00e613dd => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e613dd
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e613de
                    {
                        let r = fl.add(esp, 0x00000010u32, 32);
                        esp = r;
                    } // 00e613df
                    return; // 00e613e2
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
// Translated from 00e613f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sum over the `word[+0x10]` pieces (thiscall, no stack arguments): the sum of `00e673f0` (a callee
/// that answers a count for one piece) for each index.
pub fn fn_00e613f0(e: &mut Engine, this: Ptr) -> i32 {
    e.with_stack(0x200, |e, frame| {
        let mut esp = frame.addr() + 0x100;
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e613f0;
        loop {
            match pc {
                0x00e613f0 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e613f0
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e613f1
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e613f2
                    edi = ecx; // 00e613f3
                    {
                        let r = fl.xor(ebx, ebx, 32);
                        ebx = r;
                    } // 00e613f5
                    {
                        let r = fl.xor(esi, esi, 32);
                        esi = r;
                    } // 00e613f7
                    fl.sub(e.mem.u32(edi.wrapping_add(0x10)), ebx, 32); // 00e613f9
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e61413;
                        continue;
                    } // 00e613fc
                      // no change (a move of a register onto itself); // 00e613fe
                    pc = 0x00e61400;
                }
                0x00e61400 => {
                    eax = e.mem.u32(edi.wrapping_add(0xc)); // 00e61400
                    ecx = e.mem.u32(eax.wrapping_add(esi.wrapping_mul(4))); // 00e61403
                    {
                        let w = [ecx];
                        let r = e.call(0x00e673f0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e61406
                    {
                        let r = fl.inc(esi, 32);
                        esi = r;
                    } // 00e6140b
                    {
                        let r = fl.add(ebx, eax, 32);
                        ebx = r;
                    } // 00e6140c
                    fl.sub(esi, e.mem.u32(edi.wrapping_add(0x10)), 32); // 00e6140e
                    if fl.sf != fl.of {
                        pc = 0x00e61400;
                        continue;
                    } // 00e61411
                    pc = 0x00e61413;
                }
                0x00e61413 => {
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61413
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61414
                    eax = ebx; // 00e61415
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61417
                    return eax as i32; // 00e61418
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
// Translated from 00e61420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Locates piece `index` (thiscall, `RET 0x10`): walks the `word[+0x10]` pieces adding up the counts
/// of `00e673f0` until `index` falls inside one, then calls `00e67420(index - sum, second, third,
/// fourth)` for it.
pub fn fn_00e61420(e: &mut Engine, this: Ptr, index: u32, second: u32, third: u32, fourth: u32) {
    e.with_stack(0x200, |e, frame| {
        let mut esp = frame.addr() + 0x100;
        e.mem.set_u32(esp.wrapping_add(4), index);
        e.mem.set_u32(esp.wrapping_add(8), second);
        e.mem.set_u32(esp.wrapping_add(12), third);
        e.mem.set_u32(esp.wrapping_add(16), fourth);
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi, mut ebp) =
            (0u32, this.addr(), 0u32, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut fpu = Fpu::new();
        let mut pc = 0x00e61420;
        loop {
            match pc {
                0x00e61420 => {
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebx); // 00e61420
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ebp); // 00e61421
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, esi); // 00e61422
                    ebx = ecx; // 00e61423
                    {
                        let r = fl.xor(esi, esi, 32);
                        esi = r;
                    } // 00e61425
                    {
                        let r = fl.xor(ebp, ebp, 32);
                        ebp = r;
                    } // 00e61427
                    fl.sub(e.mem.u32(ebx.wrapping_add(0x10)), esi, 32); // 00e61429
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e61470;
                        continue;
                    } // 00e6142c
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edi); // 00e6142e
                                             // nop // 00e6142f
                    pc = 0x00e61430;
                }
                0x00e61430 => {
                    eax = e.mem.u32(ebx.wrapping_add(0xc)); // 00e61430
                    edi = e.mem.u32(eax.wrapping_add(esi.wrapping_mul(4))); // 00e61433
                    ecx = edi; // 00e61436
                    {
                        let w = [ecx];
                        let r = e.call(0x00e673f0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    } // 00e61438
                    ecx = e.mem.u32(esp.wrapping_add(0x14)); // 00e6143d
                    {
                        let r = fl.add(eax, ebp, 32);
                        eax = r;
                    } // 00e61441
                    fl.sub(ecx, eax, 32); // 00e61443
                    if fl.cf {
                        pc = 0x00e61456;
                        continue;
                    } // 00e61445
                    {
                        let r = fl.inc(esi, 32);
                        esi = r;
                    } // 00e61447
                    fl.sub(esi, e.mem.u32(ebx.wrapping_add(0x10)), 32); // 00e61448
                    ebp = eax; // 00e6144b
                    if fl.sf != fl.of {
                        pc = 0x00e61430;
                        continue;
                    } // 00e6144d
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6144f
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61450
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61451
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61452
                    return; // 00e61453
                }
                0x00e61456 => {
                    edx = e.mem.u32(esp.wrapping_add(0x20)); // 00e61456
                    eax = e.mem.u32(esp.wrapping_add(0x1c)); // 00e6145a
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e6145e
                    edx = e.mem.u32(esp.wrapping_add(0x1c)); // 00e6145f
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, eax); // 00e61463
                    {
                        let r = fl.sub(ecx, ebp, 32);
                        ecx = r;
                    } // 00e61464
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, edx); // 00e61466
                    esp = esp.wrapping_sub(4);
                    e.mem.set_u32(esp, ecx); // 00e61467
                    ecx = edi; // 00e61468
                    {
                        let w = [
                            ecx,
                            e.mem.u32(esp),
                            e.mem.u32(esp.wrapping_add(4)),
                            e.mem.u32(esp.wrapping_add(8)),
                            e.mem.u32(esp.wrapping_add(12)),
                        ];
                        let r = e.call(0x00e67420, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    esp = esp.wrapping_add(16); // 00e6146a
                    edi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e6146f
                    pc = 0x00e61470;
                }
                0x00e61470 => {
                    esi = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61470
                    ebp = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61471
                    ebx = e.mem.u32(esp);
                    esp = esp.wrapping_add(4); // 00e61472
                    return; // 00e61473
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e60e40, fn_00e60e40(i32, i32, i32, Ptr)),
        entry!(0x00e60f50, fn_00e60f50(i32, Ptr, i32, i32, Ptr)),
        entry!(0x00e61090, fn_00e61090(i32, Ptr, i32, i32, Ptr)),
        entry!(0x00e61280, fn_00e61280(Ptr, Ptr)),
        entry!(0x00e613f0, fn_00e613f0(Ptr) -> i32),
        entry!(0x00e61420, fn_00e61420(Ptr, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00e53090::test_support::*;
    fn words(e: &mut Engine, rec: Ptr) -> Vec<u32> {
        let data = e.mem.u32(rec.addr());
        (0..e.mem.u32(rec.addr() + 4))
            .map(|k| e.mem.u32(data + 4 * k))
            .collect()
    }

    fn halves(e: &mut Engine, rec: Ptr) -> Vec<u16> {
        let data = e.mem.u32(rec.addr());
        (0..e.mem.u32(rec.addr() + 4))
            .map(|k| e.mem.u16(data + 2 * k))
            .collect()
    }

    /// An empty array record that has to grow (capacity word with the not-owned flag).
    fn empty(e: &mut Engine) -> Ptr {
        let rec = e.mem.alloc(0x40);
        e.mem.set_u32(rec + 8, 0x8000_0000);
        Ptr::new(rec)
    }

    #[test]
    fn index_lists_for_modes_3_and_4() {
        let mut e = Engine::new();
        heap(&mut e);
        let out = empty(&mut e);
        fn_00e60e40(&mut e, 3, 3, 0xfffe, out);
        assert_eq!(words(&mut e, out), vec![0xfffe, 0xffff, 0]);
        let strip = empty(&mut e);
        fn_00e60e40(&mut e, 4, 5, 10, strip);
        assert_eq!(
            words(&mut e, strip),
            vec![10, 11, 12, 11, 13, 12, 12, 13, 14]
        );
        // fewer than three vertices make no triangle; other modes do nothing
        let none = empty(&mut e);
        fn_00e60e40(&mut e, 4, 2, 0, none);
        fn_00e60e40(&mut e, 5, 9, 0, none);
        assert_eq!(words(&mut e, none), Vec::<u32>::new());
        // the arrays are appended to
        fn_00e60e40(&mut e, 3, 2, 100, out);
        assert_eq!(words(&mut e, out).len(), 5);
    }

    #[test]
    fn index_lists_from_32_bit_indexes() {
        let mut e = Engine::new();
        heap(&mut e);
        e.register(0x00cb_1260, |e, a| {
            // `hkString::memCpy`: (destination, source, bytes)
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let indexes = e.mem.alloc(32);
        for (k, v) in [5u32, 6, 9, 4].iter().enumerate() {
            e.mem.set_u32(indexes + 4 * k as u32, *v);
        }
        let plain = empty(&mut e);
        fn_00e60f50(&mut e, 3, Ptr::new(indexes), 4, 0, plain);
        assert_eq!(words(&mut e, plain), vec![5, 6, 9, 4]);
        let shifted = empty(&mut e);
        fn_00e60f50(&mut e, 3, Ptr::new(indexes), 4, 100, shifted);
        assert_eq!(words(&mut e, shifted), vec![105, 106, 109, 104]);
        let strip = empty(&mut e);
        fn_00e60f50(&mut e, 4, Ptr::new(indexes), 4, 1, strip);
        assert_eq!(words(&mut e, strip), vec![6, 7, 10, 7, 5, 10]);
        let other = empty(&mut e);
        fn_00e60f50(&mut e, 2, Ptr::new(indexes), 4, 1, other);
        assert_eq!(words(&mut e, other), Vec::<u32>::new());
    }

    #[test]
    fn index_lists_from_16_bit_indexes() {
        let mut e = Engine::new();
        heap(&mut e);
        e.register(0x00cb_1260, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let indexes = e.mem.alloc(16);
        for (k, v) in [5u16, 6, 9, 4].iter().enumerate() {
            e.mem.set_u16(indexes + 2 * k as u32, *v);
        }
        let plain = empty(&mut e);
        fn_00e61090(&mut e, 3, Ptr::new(indexes), 4, 0, plain);
        assert_eq!(halves(&mut e, plain), vec![5, 6, 9, 4]);
        let shifted = empty(&mut e);
        fn_00e61090(&mut e, 3, Ptr::new(indexes), 4, 100, shifted);
        assert_eq!(halves(&mut e, shifted), vec![105, 106, 109, 104]);
        let strip = empty(&mut e);
        fn_00e61090(&mut e, 4, Ptr::new(indexes), 4, 1, strip);
        assert_eq!(halves(&mut e, strip), vec![6, 7, 10, 7, 5, 10]);
    }

    /// A primitive record: the mode at `+0`, the count at `+8`, the first index at `+0xc`, the
    /// source selector byte at `+0x14` and the index array address at `+0x18`.
    fn primitive(e: &mut Engine, mode: u8, count: u32, source: u8, indexes: u32) -> Ptr {
        let record = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u8(record.addr(), mode);
        e.mem.set_u32(record.addr() + 8, count);
        e.mem.set_u32(record.addr() + 0xc, 20);
        e.mem.set_u8(record.addr() + 0x14, source);
        e.mem.set_u32(record.addr() + 0x18, indexes);
        record
    }

    #[test]
    fn primitive_records_by_source() {
        let mut e = Engine::new();
        heap(&mut e);
        e.register(0x00cb_1260, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        // the scratch block the 16-bit source reads into comes from the router's bump pointer
        e.map(0x7700_0000, 0x1000);
        e.mem.set_u32(ROUTER, 0x7700_0000);
        let halves_at = e.mem.alloc(16);
        for (k, v) in [7u16, 8, 9].iter().enumerate() {
            e.mem.set_u16(halves_at + 2 * k as u32, *v);
        }
        let words_at = e.mem.alloc(16);
        for (k, v) in [70u32, 80, 90].iter().enumerate() {
            e.mem.set_u32(words_at + 4 * k as u32, *v);
        }
        // source 0: generated from the first index
        let a = empty(&mut e);
        let record = primitive(&mut e, 3, 3, 0, 0);
        fn_00e61280(&mut e, record, a);
        assert_eq!(words(&mut e, a), vec![20, 21, 22]);
        // source 1: 16-bit indexes widened into words, the bump pointer restored
        let b = empty(&mut e);
        let record = primitive(&mut e, 3, 3, 1, halves_at);
        fn_00e61280(&mut e, record, b);
        assert_eq!(words(&mut e, b), vec![7, 8, 9]);
        assert_eq!(e.mem.u32(ROUTER), 0x7700_0000);
        let b4 = empty(&mut e);
        let record = primitive(&mut e, 4, 3, 1, halves_at);
        fn_00e61280(&mut e, record, b4);
        assert_eq!(words(&mut e, b4), vec![7, 8, 9]);
        // source 2: 32-bit indexes
        let c = empty(&mut e);
        let record = primitive(&mut e, 3, 3, 2, words_at);
        fn_00e61280(&mut e, record, c);
        assert_eq!(words(&mut e, c), vec![70, 80, 90]);
        // other modes with sources 1 and 2 do nothing
        let d = empty(&mut e);
        let record = primitive(&mut e, 5, 3, 2, words_at);
        fn_00e61280(&mut e, record, d);
        let record = primitive(&mut e, 5, 3, 1, halves_at);
        fn_00e61280(&mut e, record, d);
        assert_eq!(words(&mut e, d), Vec::<u32>::new());
    }

    /// A collection of pieces that count themselves with `00e673f0` (a double answering the first
    /// word of the piece it is given): the piece pointers are an array at `+0xc`, their number is at
    /// `+0x10`.
    fn counting(e: &mut Engine, counts: &[u32]) -> Ptr {
        e.register(0x00e6_73f0, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let array = e.mem.alloc(4 * counts.len() as u32 + 8);
        for (k, c) in counts.iter().enumerate() {
            let piece = e.mem.alloc(0x40);
            e.mem.set_u32(piece, *c);
            e.mem.set_u32(array + 4 * k as u32, piece);
        }
        let this = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(this.addr() + 0xc, array);
        e.mem.set_u32(this.addr() + 0x10, counts.len() as u32);
        this
    }

    #[test]
    fn total_of_the_piece_counts() {
        let mut e = Engine::new();
        let this = counting(&mut e, &[3, 4, 5]);
        assert_eq!(fn_00e613f0(&mut e, this), 12);
        e.mem.set_u32(this.addr() + 0x10, 0);
        assert_eq!(fn_00e613f0(&mut e, this), 0);
    }

    #[test]
    fn piece_lookup_by_running_count() {
        let mut e = Engine::new();
        let this = counting(&mut e, &[4, 4, 4]);
        e.register(0x00e6_7420, |e, a| {
            e.mem.set_u32(0x7300_0800, a.len() as u32);
            for (k, w) in a.iter().enumerate() {
                e.mem.set_u32(0x7300_0804 + 4 * k as u32, *w);
            }
            Ret::default()
        });
        e.map(0x7300_0000, 0x1000);
        // counts of 4 each: index 9 falls into the third piece (position 1 inside it)
        fn_00e61420(&mut e, this, 9, 1, 2, 3);
        assert_eq!(e.mem.u32(0x7300_0800), 5);
        assert_eq!(e.mem.u32(0x7300_0808), 1);
        assert_eq!(e.mem.u32(0x7300_080c), 1);
        assert_eq!(e.mem.u32(0x7300_0810), 2);
        assert_eq!(e.mem.u32(0x7300_0814), 3);
        // an index past the last piece calls nothing
        e.mem.set_u32(0x7300_0800, 0);
        fn_00e61420(&mut e, this, 100, 1, 2, 3);
        assert_eq!(e.mem.u32(0x7300_0800), 0);
    }
}
