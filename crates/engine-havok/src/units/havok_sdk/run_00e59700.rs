//! `Havok SDK/run_00e59700` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{
    addps, addss, fcom_status, load_vec, mulps, shufps, store_vec, subps, unpckhps,
};
use crate::units::havok_sdk::run_00e4f910::{scalar_vec, Cpu};

/// The CRT square root (`00ec6040`): the argument in `ST0` as a leading `double`, the root back in
/// `ST0`.
const X87_SQRT: u32 = 0x00ec_6040;
/// `00df22f0` (Ghidra's name for it is `hkpProcessFlyingColors`): cdecl, one word, the address of
/// the byte `00e59a40` tests.
const PROCESS_FLYING_COLORS: u32 = 0x00df_22f0;
/// `00e59340`: cdecl, `(ok, matrix, inverse, epsilon)`, the 2 by 2 inverse.
const INVERT_2X2: u32 = 0x00e5_9340;
/// `00e59700`: register inputs `edx`, `esi`, `edi`, no stack words.
const SOLVE_STEP: u32 = 0x00e5_9700;
/// The byte `00e59a40` tests before it does anything.
const FLYING_COLORS_FLAG: u32 = 0x0126_efb9;

// Translated from 00e59700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs first: `edx` points at a table of `float`s (read at entries 0 to 15, and at
/// `+0x40` and `+0x44`), `esi` at a state record (`float`s at `+0x80` to `+0x98`) and `edi` at an
/// output record (`float`s at `+8`, `+0x1c` and `+0x20`); the exe's call has no stack words and
/// the x87 stack is empty on entry and exit. The products `state[+0x94] * state[+0x84]` and
/// `state[+0x90] * state[+0x80]` are stored at `edi + 0x1c` and `edi + 0x20`; when the sum of
/// their squares is below the `double` at `01012070` the float at `state[+0x98]` is set to 0 and
/// that is all. Otherwise the table is walked (at most 0x10 steps) until the squared length
/// passes it, two square roots (`00ec6040`) and an interpolation between the last two steps
/// follow, the results are clamped to the range of `-1.1` to `1.1` and stored into
/// `state[+0x80]`, `[+0x84]` and `[+0x98]`, and a third root goes to `edi + 8`. The function is
/// the exe's code instruction by instruction on the [`Cpu`] state.
pub fn fn_00e59700(e: &mut Engine, edx: u32, esi: u32, edi: u32) {
    run_00e59700(e, [edx, esi, edi], 0);
}

/// `00e59700` on a frame whose words start as `fill` (the exe's frame has whatever was on the
/// stack: the function must not depend on it).
fn run_00e59700(e: &mut Engine, registers: [u32; 3], fill: u32) {
    e.with_stack(0x100, |e, frame| {
        let top = frame.addr() + 0xc0;
        for word in 0..0x40 {
            e.mem.set_u32(frame.addr() + 4 * word, fill);
        }
        let mut cpu = Cpu {
            esp: top,
            edx: registers[0],
            esi: registers[1],
            edi: registers[2],
            ..Cpu::default()
        };
        e.mem.set_u32(top, 0);
        let mut pc: u32 = 0x00e5_9700;
        'run: loop {
            match pc {
                0x00e59700 => {
                    // 00e59700: SUB ESP,0x18
                    cpu.esp = cpu.sub32(cpu.esp, 0x18);
                    // 00e59703: FLD float ptr [ESI + 0x94]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x94))));
                    // 00e59709: FMUL float ptr [ESI + 0x84]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esi.wrapping_add(0x84))));
                        cpu.set_st(0, v);
                    }
                    // 00e5970f: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59713: FLD float ptr [ESI + 0x90]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x90))));
                    // 00e59719: FMUL float ptr [ESI + 0x80]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esi.wrapping_add(0x80))));
                        cpu.set_st(0, v);
                    }
                    // 00e5971f: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59723: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00e59727: FST float ptr [EDI + 0x1c]
                    e.mem
                        .set_u32(cpu.edi.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    // 00e5972a: FLD float ptr [ESP + 0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x10))));
                    // 00e5972e: FST float ptr [EDI + 0x20]
                    e.mem
                        .set_u32(cpu.edi.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    // 00e59731: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e59733: FLD ST2
                    {
                        let v = cpu.st(2);
                        cpu.st_push(v);
                    }
                    // 00e59735: FMUL ST0
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e59737: FLD ST1
                    {
                        let v = cpu.st(1);
                        cpu.st_push(v);
                    }
                    // 00e59739: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5973b: FADDP
                    {
                        let v = x87_add(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5973d: FCOMP double ptr [0x01012070]
                    cpu.sw = u16::from(fcom_status(cpu.st(0), e.mem.f64(0x01012070))) << 8;
                    cpu.st_pop();
                    // 00e59743: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e59745: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e59748: JP 0x00e5975a
                    if cpu.pf {
                        pc = 0x00e5975a;
                        continue 'run;
                    }
                    // 00e5974a: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5974c: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5974e: FLDZ
                    cpu.st_push(0.0);
                    // 00e59750: FSTP float ptr [ESI + 0x98]
                    e.mem
                        .set_u32(cpu.esi.wrapping_add(0x98), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59756: ADD ESP,0x18
                    cpu.esp = cpu.add32(cpu.esp, 0x18);
                    // 00e59759: RET
                    break 'run;
                }
                0x00e5975a => {
                    // 00e5975a: FLD float ptr [EDX + 0x44]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edx.wrapping_add(0x44))));
                    // 00e5975d: XOR ECX,ECX
                    cpu.ecx = cpu.xor32(cpu.ecx, cpu.ecx);
                    // 00e5975f: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59761: FXCH
                    cpu.st_exchange(1);
                    // 00e59763: FSTP float ptr [ESP + 0x8]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x8), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59767: FLDZ
                    cpu.st_push(0.0);
                    // 00e59769: FST float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    // 00e5976d: FST float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    // 00e59771: FSTP float ptr [ESP + 0x4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59775: FLD float ptr [EDX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edx)));
                    // 00e59777: FMUL ST1
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(1));
                        cpu.set_st(0, v);
                    }
                    // 00e59779: FSTP float ptr [ESP + 0xc]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xc), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5977d: FLD float ptr [ESP + 0xc]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xc))));
                    // 00e59781: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e59783: FLD float ptr [ESP + 0x8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x8))));
                    // 00e59787: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e59789: FMUL ST0
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e5978b: FLD ST2
                    {
                        let v = cpu.st(2);
                        cpu.st_push(v);
                    }
                    // 00e5978d: FMULP ST3
                    {
                        let v = x87_mul(cpu.st(3), cpu.st(0));
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    // 00e5978f: FADDP ST2,ST0
                    {
                        let v = x87_add(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59791: FXCH
                    cpu.st_exchange(1);
                    // 00e59793: FSTP float ptr [ESP]
                    e.mem.set_u32(cpu.esp, x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59796: FLD1
                    cpu.st_push(1.0);
                    // 00e59798: FLD float ptr [ESP]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp)));
                    // 00e5979b: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e5979d: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5979f: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e597a2: JNZ 0x00e597b2
                    if !cpu.zf {
                        pc = 0x00e597b2;
                        continue 'run;
                    }
                    pc = 0x00e597a4;
                    continue 'run;
                }
                0x00e597a4 => {
                    // 00e597a4: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e597a6: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e597a8: JMP 0x00e59807
                    pc = 0x00e59807;
                    continue 'run;
                }
                0x00e597aa => {
                    // 00e597aa: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e597ac: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e597ae: FXCH
                    cpu.st_exchange(1);
                    // 00e597b0: FXCH ST2
                    cpu.st_exchange(2);
                    pc = 0x00e597b2;
                    continue 'run;
                }
                0x00e597b2 => {
                    // 00e597b2: INC ECX
                    cpu.ecx = cpu.inc32(cpu.ecx);
                    // 00e597b3: CMP ECX,0x10
                    cpu.cmp32(cpu.ecx, 0x10);
                    // 00e597b6: JGE 0x00e597a4
                    if cpu.sf == cpu.of {
                        pc = 0x00e597a4;
                        continue 'run;
                    }
                    // 00e597b8: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e597ba: FST float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    // 00e597be: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e597c0: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e597c4: FXCH
                    cpu.st_exchange(1);
                    // 00e597c6: FSTP float ptr [ESP + 0x4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e597ca: FLD float ptr [EDX + 0x40]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edx.wrapping_add(0x40))));
                    // 00e597cd: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e597cf: FXCH
                    cpu.st_exchange(1);
                    // 00e597d1: FSTP float ptr [ESP + 0x8]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x8), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e597d5: FLD float ptr [EDX + ECX*0x4]
                    cpu.st_push(x87_load32(
                        e.mem.u32(cpu.edx.wrapping_add(cpu.ecx.wrapping_mul(4))),
                    ));
                    // 00e597d8: FMUL ST2
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(2));
                        cpu.set_st(0, v);
                    }
                    // 00e597da: FSTP float ptr [ESP + 0xc]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xc), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e597de: FLD float ptr [ESP + 0xc]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xc))));
                    // 00e597e2: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e597e4: FLD float ptr [ESP + 0x8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x8))));
                    // 00e597e8: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e597ea: FMUL ST0
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e597ec: FLD ST2
                    {
                        let v = cpu.st(2);
                        cpu.st_push(v);
                    }
                    // 00e597ee: FMULP ST3
                    {
                        let v = x87_mul(cpu.st(3), cpu.st(0));
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    // 00e597f0: FADDP ST2,ST0
                    {
                        let v = x87_add(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e597f2: FXCH
                    cpu.st_exchange(1);
                    // 00e597f4: FSTP float ptr [ESP]
                    e.mem.set_u32(cpu.esp, x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e597f7: FLD float ptr [ESP]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp)));
                    // 00e597fa: FCOM ST3
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(3))) << 8;
                    // 00e597fc: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e597fe: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e59801: JNZ 0x00e597aa
                    if !cpu.zf {
                        pc = 0x00e597aa;
                        continue 'run;
                    }
                    // 00e59803: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e59805: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e59807;
                    continue 'run;
                }
                0x00e59807 => {
                    // 00e59807: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59809: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5980b: CALL 0x00ec6040
                    {
                        let v = cpu.st(0);
                        cpu.st_pop();
                        let r = e.call(X87_SQRT, &args![v]);
                        cpu.st_push(r.st0);
                    }
                    // 00e59810: FSTP float ptr [ESP]
                    e.mem.set_u32(cpu.esp, x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59813: FLD float ptr [ESP]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp)));
                    // 00e59816: FSUB double ptr [0x01012070]
                    {
                        let v = x87_sub(cpu.st(0), e.mem.f64(0x01012070));
                        cpu.set_st(0, v);
                    }
                    // 00e5981c: FSTP float ptr [ESP]
                    e.mem.set_u32(cpu.esp, x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5981f: FLD float ptr [ESP + 0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x4))));
                    // 00e59823: CALL 0x00ec6040
                    {
                        let v = cpu.st(0);
                        cpu.st_pop();
                        let r = e.call(X87_SQRT, &args![v]);
                        cpu.st_push(r.st0);
                    }
                    // 00e59828: FSTP float ptr [ESP + 0x4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5982c: FLD float ptr [ESP + 0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x4))));
                    // 00e59830: FLD1
                    cpu.st_push(1.0);
                    // 00e59832: FSUB ST1,ST0
                    {
                        let v = x87_sub(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    // 00e59834: FXCH
                    cpu.st_exchange(1);
                    // 00e59836: FSTP float ptr [ESP + 0x4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5983a: FLD float ptr [ESP]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp)));
                    // 00e5983d: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5983f: FLD float ptr [ESP + 0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x4))));
                    // 00e59843: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e59845: FSUBP ST2,ST0
                    {
                        let v = x87_sub(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59847: FXCH
                    cpu.st_exchange(1);
                    // 00e59849: FSTP float ptr [ESP + 0x4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5984d: FLD float ptr [ESP + 0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x4))));
                    // 00e59851: FSUB double ptr [0x01012060]
                    {
                        let v = x87_sub(cpu.st(0), e.mem.f64(0x01012060));
                        cpu.set_st(0, v);
                    }
                    // 00e59857: FSTP float ptr [ESP]
                    e.mem.set_u32(cpu.esp, x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5985a: FLD float ptr [ESP]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp)));
                    // 00e5985d: FABS
                    {
                        let v = cpu.st(0).abs();
                        cpu.set_st(0, v);
                    }
                    // 00e5985f: FSTP float ptr [ESP]
                    e.mem.set_u32(cpu.esp, x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59862: FLD float ptr [ESP]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp)));
                    // 00e59865: FLD float ptr [0x0104da4c]
                    cpu.st_push(x87_load32(e.mem.u32(0x0104da4c)));
                    // 00e5986b: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e5986d: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5986f: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e59871: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e59874: JNZ 0x00e5987a
                    if !cpu.zf {
                        pc = 0x00e5987a;
                        continue 'run;
                    }
                    // 00e59876: FST float ptr [ESP + 0x4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    pc = 0x00e5987a;
                    continue 'run;
                }
                0x00e5987a => {
                    // 00e5987a: FLD float ptr [ESP + 0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x4))));
                    // 00e5987e: FDIVR ST0,ST4
                    {
                        let v = x87_div(cpu.st(4), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e59880: FSTP float ptr [ESP + 0x4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59884: FXCH
                    cpu.st_exchange(1);
                    // 00e59886: FCHS
                    {
                        let v = -cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    // 00e59888: FLD float ptr [ESP + 0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x4))));
                    // 00e5988c: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5988e: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59890: FXCH
                    cpu.st_exchange(1);
                    // 00e59892: FSTP float ptr [ESP]
                    e.mem.set_u32(cpu.esp, x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59895: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59897: FXCH
                    cpu.st_exchange(1);
                    // 00e59899: FSTP float ptr [ESP + 0x4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5989d: FLD float ptr [ESP + 0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x4))));
                    // 00e598a1: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e598a3: FMUL float ptr [ESP + 0x10]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x10))));
                        cpu.set_st(0, v);
                    }
                    // 00e598a7: FLD float ptr [ESP]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp)));
                    // 00e598aa: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e598ac: FMUL float ptr [ESP + 0x8]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x8))));
                        cpu.set_st(0, v);
                    }
                    // 00e598b0: FADDP ST2,ST0
                    {
                        let v = x87_add(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e598b2: FXCH
                    cpu.st_exchange(1);
                    // 00e598b4: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e598b8: FLD float ptr [0x010ec9e8]
                    cpu.st_push(x87_load32(e.mem.u32(0x010ec9e8)));
                    // 00e598be: FLD float ptr [ESP + 0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x10))));
                    // 00e598c2: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e598c4: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e598c6: FLD float ptr [0x01056528]
                    cpu.st_push(x87_load32(e.mem.u32(0x01056528)));
                    // 00e598cc: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e598cf: JP 0x00e598db
                    if cpu.pf {
                        pc = 0x00e598db;
                        continue 'run;
                    }
                    // 00e598d1: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e598d3: FXCH
                    cpu.st_exchange(1);
                    // 00e598d5: FST float ptr [ESP + 0x8]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x8), x87_store32(cpu.st(0)));
                    // 00e598d9: JMP 0x00e598f4
                    pc = 0x00e598f4;
                    continue 'run;
                }
                0x00e598db => {
                    // 00e598db: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e598dd: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e598df: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e598e2: JP 0x00e598ec
                    if cpu.pf {
                        pc = 0x00e598ec;
                        continue 'run;
                    }
                    // 00e598e4: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e598e6: FST float ptr [ESP + 0x8]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x8), x87_store32(cpu.st(0)));
                    // 00e598ea: JMP 0x00e598f2
                    pc = 0x00e598f2;
                    continue 'run;
                }
                0x00e598ec => {
                    // 00e598ec: FXCH
                    cpu.st_exchange(1);
                    // 00e598ee: FSTP float ptr [ESP + 0x8]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x8), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e598f2;
                    continue 'run;
                }
                0x00e598f2 => {
                    // 00e598f2: FXCH
                    cpu.st_exchange(1);
                    pc = 0x00e598f4;
                    continue 'run;
                }
                0x00e598f4 => {
                    // 00e598f4: FLD float ptr [ESP + 0xc]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xc))));
                    // 00e598f8: FMULP ST3
                    {
                        let v = x87_mul(cpu.st(3), cpu.st(0));
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    // 00e598fa: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00e598fe: FMULP ST4
                    {
                        let v = x87_mul(cpu.st(4), cpu.st(0));
                        cpu.set_st(4, v);
                    }
                    cpu.st_pop();
                    // 00e59900: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e59902: FADDP ST3,ST0
                    {
                        let v = x87_add(cpu.st(3), cpu.st(0));
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    // 00e59904: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e59906: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5990a: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00e5990e: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e59910: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e59912: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e59915: JP 0x00e599b2
                    if cpu.pf {
                        pc = 0x00e599b2;
                        continue 'run;
                    }
                    // 00e5991b: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5991d;
                    continue 'run;
                }
                0x00e5991d => {
                    // 00e5991d: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5991f;
                    continue 'run;
                }
                0x00e5991f => {
                    // 00e5991f: FSTP float ptr [ESP + 0xc]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xc), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59923: FLD float ptr [ESI + 0x8c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x8c))));
                    // 00e59929: FMUL float ptr [ESP + 0x8]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x8))));
                        cpu.set_st(0, v);
                    }
                    // 00e5992d: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59931: FLD float ptr [ESI + 0x88]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x88))));
                    // 00e59937: FMUL float ptr [ESP + 0xc]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0xc))));
                        cpu.set_st(0, v);
                    }
                    // 00e5993b: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5993f: FLD float ptr [ESI + 0x84]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x84))));
                    // 00e59945: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00e59949: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5994b: FSUBP ST2,ST0
                    {
                        let v = x87_sub(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5994d: FXCH
                    cpu.st_exchange(1);
                    // 00e5994f: FSTP float ptr [ESP + 0x8]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x8), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59953: FSTP float ptr [ESI + 0x84]
                    e.mem
                        .set_u32(cpu.esi.wrapping_add(0x84), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59959: FLD float ptr [ESI + 0x80]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x80))));
                    // 00e5995f: FLD float ptr [ESP + 0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x10))));
                    // 00e59963: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e59965: FSUBP ST2,ST0
                    {
                        let v = x87_sub(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59967: FXCH
                    cpu.st_exchange(1);
                    // 00e59969: FSTP float ptr [ESI + 0x98]
                    e.mem
                        .set_u32(cpu.esi.wrapping_add(0x98), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5996f: FSTP float ptr [ESI + 0x80]
                    e.mem
                        .set_u32(cpu.esi.wrapping_add(0x80), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59975: FLD float ptr [ESI + 0x68]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x68))));
                    // 00e59978: FLD float ptr [ESI + 0x60]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x60))));
                    // 00e5997b: FMUL ST0
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e5997d: FLD ST1
                    {
                        let v = cpu.st(1);
                        cpu.st_push(v);
                    }
                    // 00e5997f: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59981: FADDP
                    {
                        let v = x87_add(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e59983: FSTP float ptr [ESP + 0xc]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xc), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59987: FLD float ptr [ESP + 0xc]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xc))));
                    // 00e5998b: FSUB double ptr [0x01012060]
                    {
                        let v = x87_sub(cpu.st(0), e.mem.f64(0x01012060));
                        cpu.set_st(0, v);
                    }
                    // 00e59991: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59995: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00e59999: FABS
                    {
                        let v = cpu.st(0).abs();
                        cpu.set_st(0, v);
                    }
                    // 00e5999b: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5999f: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00e599a3: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e599a5: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e599a7: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e599aa: JP 0x00e599c8
                    if cpu.pf {
                        pc = 0x00e599c8;
                        continue 'run;
                    }
                    // 00e599ac: FSTP float ptr [ESP + 0xc]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xc), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e599b0: JMP 0x00e599ca
                    pc = 0x00e599ca;
                    continue 'run;
                }
                0x00e599b2 => {
                    // 00e599b2: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e599b4: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e599b6: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e599b8: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e599bb: JNZ 0x00e5991d
                    if !cpu.zf {
                        pc = 0x00e5991d;
                        continue 'run;
                    }
                    // 00e599c1: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e599c3: JMP 0x00e5991f
                    pc = 0x00e5991f;
                    continue 'run;
                }
                0x00e599c8 => {
                    // 00e599c8: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e599ca;
                    continue 'run;
                }
                0x00e599ca => {
                    // 00e599ca: FLD float ptr [ESI + 0x7c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x7c))));
                    // 00e599cd: FMUL float ptr [ESI + 0x98]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esi.wrapping_add(0x98))));
                        cpu.set_st(0, v);
                    }
                    // 00e599d3: FMUL float ptr [ESI + 0x88]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esi.wrapping_add(0x88))));
                        cpu.set_st(0, v);
                    }
                    // 00e599d9: FSTP float ptr [ESP + 0x10]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e599dd: FLD float ptr [ESI + 0x74]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x74))));
                    // 00e599e0: FMUL float ptr [ESP + 0x8]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x8))));
                        cpu.set_st(0, v);
                    }
                    // 00e599e4: FMUL float ptr [ESI + 0x8c]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esi.wrapping_add(0x8c))));
                        cpu.set_st(0, v);
                    }
                    // 00e599ea: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e599ee: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00e599f2: FLD float ptr [ESP + 0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x10))));
                    // 00e599f6: FMUL ST0
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e599f8: FLD ST1
                    {
                        let v = cpu.st(1);
                        cpu.st_push(v);
                    }
                    // 00e599fa: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e599fc: FADDP
                    {
                        let v = x87_add(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e599fe: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59a02: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00e59a06: FLD float ptr [ESP + 0xc]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xc))));
                    // 00e59a0a: FDIVP ST2,ST0
                    {
                        let v = x87_div(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59a0c: FXCH
                    cpu.st_exchange(1);
                    // 00e59a0e: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59a12: FMUL float ptr [ESP + 0x14]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                        cpu.set_st(0, v);
                    }
                    // 00e59a16: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59a1a: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00e59a1e: CALL 0x00ec6040
                    {
                        let v = cpu.st(0);
                        cpu.st_pop();
                        let r = e.call(X87_SQRT, &args![v]);
                        cpu.st_push(r.st0);
                    }
                    // 00e59a23: FSTP float ptr [ESP + 0x14]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59a27: FLD float ptr [ESP + 0x14]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x14))));
                    // 00e59a2b: FSTP float ptr [EDI + 0x8]
                    e.mem
                        .set_u32(cpu.edi.wrapping_add(0x8), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59a2e: ADD ESP,0x18
                    cpu.esp = cpu.add32(cpu.esp, 0x18);
                    // 00e59a31: RET
                    break 'run;
                }
                other => panic!("jump to {other:#x} outside the recovered function"),
            }
        }
    });
}

// Translated from 00e59a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(factor, settings, state, out)`: `factor` points at two `float`s (the first is
/// multiplied into the impulses, the second into the `out` rows' fourth and thirteenth words),
/// `settings` and `state` are records read at the offsets the code uses (`settings + 4`, `+0x54`,
/// `+0x64`, `+0x68`, `+0xc8` and `+0xcc`; `state`'s two 0x60-byte entries from `+0x20`, its
/// vectors up to `+0x178`, and the two bodies its entries point at, which are updated at `+0x10`
/// to `+0x2c`) and `out` receives `float`s from the start in two nine-word rows. Nothing happens
/// unless the byte at `0126efb9` is nonzero: when it is 0, `00df22f0` (Ghidra's name for it is
/// `hkpProcessFlyingColors`) is given the byte's address and the byte is tested again. The rows
/// are built in two loops of two iterations (square roots by `00ec6040`), `00e59340` inverts a 2
/// by 2 matrix and `00e59700` is run once per iteration of the last loop of the solve. The
/// function is the exe's code instruction by instruction on the [`Cpu`] state (the exe keeps its
/// values on the x87 stack through the loops and mixes SSE with it).
pub fn fn_00e59a40(e: &mut Engine, factor: u32, settings: u32, state: u32, out: u32) {
    run_00e59a40(e, [factor, settings, state, out], 0);
}

/// `00e59a40` on a frame whose words start as `fill` (the exe's frame has whatever was on the
/// stack: the function must not depend on it).
fn run_00e59a40(e: &mut Engine, args: [u32; 4], fill: u32) {
    e.with_stack(0x400, |e, frame| {
        let top = frame.addr() + 0x3c0;
        for word in 0..0x100 {
            e.mem.set_u32(frame.addr() + 4 * word, fill);
        }
        let mut cpu = Cpu {
            esp: top,
            ..Cpu::default()
        };
        e.mem.set_u32(top, 0);
        for (i, arg) in args.iter().enumerate() {
            e.mem.set_u32(top + 4 + 4 * i as u32, *arg);
        }
        let mut pc: u32 = 0x00e5_9a40;
        'run: loop {
            match pc {
                0x00e59a40 => {
                    // 00e59a40: PUSH EBP
                    cpu.push(e, cpu.ebp);
                    // 00e59a41: MOV EBP,ESP
                    cpu.ebp = cpu.esp;
                    // 00e59a43: AND ESP,0xfffffff0
                    cpu.esp = cpu.and32(cpu.esp, 0xfffffff0);
                    // 00e59a46: SUB ESP,0x1a4
                    cpu.esp = cpu.sub32(cpu.esp, 0x1a4);
                    // 00e59a4c: CMP byte ptr [0x0126efb9],0x0
                    cpu.cmp8(u32::from(e.mem.u8(FLYING_COLORS_FLAG)), 0x0);
                    // 00e59a53: PUSH EBX
                    cpu.push(e, cpu.ebx);
                    // 00e59a54: PUSH ESI
                    cpu.push(e, cpu.esi);
                    // 00e59a55: PUSH EDI
                    cpu.push(e, cpu.edi);
                    // 00e59a56: JNZ 0x00e59a72
                    if !cpu.zf {
                        pc = 0x00e59a72;
                        continue 'run;
                    }
                    // 00e59a58: PUSH 0x126efb9
                    cpu.push(e, FLYING_COLORS_FLAG);
                    // 00e59a5d: CALL 0x00df22f0
                    e.call(
                        PROCESS_FLYING_COLORS,
                        &args![e.mem.u32(cpu.esp.wrapping_add(0))],
                    );
                    // 00e59a62: ADD ESP,0x4
                    cpu.esp = cpu.add32(cpu.esp, 0x4);
                    // 00e59a65: CMP byte ptr [0x0126efb9],0x0
                    cpu.cmp8(u32::from(e.mem.u8(FLYING_COLORS_FLAG)), 0x0);
                    // 00e59a6c: JZ 0x00e5a7f0
                    if cpu.zf {
                        pc = 0x00e5a7f0;
                        continue 'run;
                    }
                    pc = 0x00e59a72;
                    continue 'run;
                }
                0x00e59a72 => {
                    // 00e59a72: MOV EDI,dword ptr [EBP + 0x14]
                    cpu.edi = e.mem.u32(cpu.ebp.wrapping_add(0x14));
                    // 00e59a75: FLD double ptr [0x010c9300]
                    cpu.st_push(e.mem.f64(0x010c9300));
                    // 00e59a7b: MOV EBX,dword ptr [EBP + 0x10]
                    cpu.ebx = e.mem.u32(cpu.ebp.wrapping_add(0x10));
                    // 00e59a7e: FLD1
                    cpu.st_push(1.0);
                    // 00e59a80: MOV ESI,dword ptr [EBP + 0xc]
                    cpu.esi = e.mem.u32(cpu.ebp.wrapping_add(0xc));
                    // 00e59a83: ADD EDI,0x18
                    cpu.edi = cpu.add32(cpu.edi, 0x18);
                    // 00e59a86: ADD ESI,0x54
                    cpu.esi = cpu.add32(cpu.esi, 0x54);
                    // 00e59a89: LEA EDX,[EBX + 0x20]
                    cpu.edx = cpu.ebx.wrapping_add(0x20);
                    // 00e59a8c: LEA EAX,[ESP + 0xc0]
                    cpu.eax = cpu.esp.wrapping_add(0xc0);
                    // 00e59a93: MOV dword ptr [ESP + 0x20],0x2
                    e.mem.set_u32(cpu.esp.wrapping_add(0x20), 0x2);
                    pc = 0x00e59a9b;
                    continue 'run;
                }
                0x00e59a9b => {
                    // 00e59a9b: MOV ECX,dword ptr [EDX + 0x10]
                    cpu.ecx = e.mem.u32(cpu.edx.wrapping_add(0x10));
                    // 00e59a9e: MOVAPS XMM1,xmmword ptr [EDX]
                    cpu.x[1] = load_vec(e, cpu.edx);
                    // 00e59aa1: MOVAPS XMM3,xmmword ptr [EDX + -0x20]
                    cpu.x[3] = load_vec(e, cpu.edx.wrapping_sub(0x20));
                    // 00e59aa5: MOVAPS XMM2,xmmword ptr [EBX + 0x110]
                    cpu.x[2] = load_vec(e, cpu.ebx.wrapping_add(0x110));
                    // 00e59aac: MOVAPS XMM6,xmmword ptr [EBX + 0x120]
                    cpu.x[6] = load_vec(e, cpu.ebx.wrapping_add(0x120));
                    // 00e59ab3: MOVAPS XMM4,XMM1
                    cpu.x[4] = cpu.x[1];
                    // 00e59ab6: SHUFPS XMM4,XMM1,0xc9
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[1], 201);
                    // 00e59aba: MOVAPS xmmword ptr [EAX + -0x50],XMM1
                    store_vec(e, cpu.eax.wrapping_sub(0x50), cpu.x[1]);
                    // 00e59abe: SHUFPS XMM1,XMM1,0xd2
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 210);
                    // 00e59ac2: MOVAPS XMM0,XMM3
                    cpu.x[0] = cpu.x[3];
                    // 00e59ac5: SUBPS XMM0,XMM2
                    cpu.x[0] = subps(cpu.x[0], cpu.x[2]);
                    // 00e59ac8: MOVAPS XMM2,xmmword ptr [ECX + 0x40]
                    cpu.x[2] = load_vec(e, cpu.ecx.wrapping_add(0x40));
                    // 00e59acc: SUBPS XMM3,XMM2
                    cpu.x[3] = subps(cpu.x[3], cpu.x[2]);
                    // 00e59acf: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00e59ad2: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e59ad5: SHUFPS XMM1,XMM0,0xc9
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 201);
                    // 00e59ad9: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00e59adc: MOVAPS XMM5,XMM0
                    cpu.x[5] = cpu.x[0];
                    // 00e59adf: SHUFPS XMM5,XMM0,0xd2
                    cpu.x[5] = shufps(cpu.x[5], cpu.x[0], 210);
                    // 00e59ae3: MULPS XMM5,XMM4
                    cpu.x[5] = mulps(cpu.x[5], cpu.x[4]);
                    // 00e59ae6: SUBPS XMM1,XMM5
                    cpu.x[1] = subps(cpu.x[1], cpu.x[5]);
                    // 00e59ae9: MOVAPS XMM7,xmmword ptr [EBX + 0x100]
                    cpu.x[7] = load_vec(e, cpu.ebx.wrapping_add(0x100));
                    // 00e59af0: MOVAPS XMM5,XMM3
                    cpu.x[5] = cpu.x[3];
                    // 00e59af3: SHUFPS XMM5,XMM3,0xc9
                    cpu.x[5] = shufps(cpu.x[5], cpu.x[3], 201);
                    // 00e59af7: MULPS XMM5,XMM2
                    cpu.x[5] = mulps(cpu.x[5], cpu.x[2]);
                    // 00e59afa: MOVAPS XMM2,XMM3
                    cpu.x[2] = cpu.x[3];
                    // 00e59afd: SHUFPS XMM2,XMM3,0xd2
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[3], 210);
                    // 00e59b01: MULPS XMM2,XMM4
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[4]);
                    // 00e59b04: SUBPS XMM2,XMM5
                    cpu.x[2] = subps(cpu.x[2], cpu.x[5]);
                    // 00e59b07: MOVAPS XMM5,xmmword ptr [EBX + 0x130]
                    cpu.x[5] = load_vec(e, cpu.ebx.wrapping_add(0x130));
                    // 00e59b0e: MOVAPS XMM4,XMM1
                    cpu.x[4] = cpu.x[1];
                    // 00e59b11: SHUFPS XMM4,XMM1,0x55
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[1], 85);
                    // 00e59b15: MULPS XMM4,XMM5
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[5]);
                    // 00e59b18: MOVAPS XMM5,XMM1
                    cpu.x[5] = cpu.x[1];
                    // 00e59b1b: SHUFPS XMM5,XMM1,0x0
                    cpu.x[5] = shufps(cpu.x[5], cpu.x[1], 0);
                    // 00e59b1f: MULPS XMM5,XMM6
                    cpu.x[5] = mulps(cpu.x[5], cpu.x[6]);
                    // 00e59b22: MOVAPS XMM6,xmmword ptr [ECX + 0x50]
                    cpu.x[6] = load_vec(e, cpu.ecx.wrapping_add(0x50));
                    // 00e59b26: ADDPS XMM4,XMM5
                    cpu.x[4] = addps(cpu.x[4], cpu.x[5]);
                    // 00e59b29: MOVAPS XMM5,XMM1
                    cpu.x[5] = cpu.x[1];
                    // 00e59b2c: SHUFPS XMM5,XMM1,0xaa
                    cpu.x[5] = shufps(cpu.x[5], cpu.x[1], 170);
                    // 00e59b30: MOVAPS XMM1,xmmword ptr [EBX + 0x140]
                    cpu.x[1] = load_vec(e, cpu.ebx.wrapping_add(0x140));
                    // 00e59b37: MULPS XMM5,XMM1
                    cpu.x[5] = mulps(cpu.x[5], cpu.x[1]);
                    // 00e59b3a: ADDPS XMM4,XMM5
                    cpu.x[4] = addps(cpu.x[4], cpu.x[5]);
                    // 00e59b3d: MOVAPS XMM5,xmmword ptr [ECX + 0x60]
                    cpu.x[5] = load_vec(e, cpu.ecx.wrapping_add(0x60));
                    // 00e59b41: MOVAPS XMM1,XMM2
                    cpu.x[1] = cpu.x[2];
                    // 00e59b44: SHUFPS XMM1,XMM2,0x55
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[2], 85);
                    // 00e59b48: MULPS XMM1,XMM5
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[5]);
                    // 00e59b4b: MOVAPS XMM5,XMM2
                    cpu.x[5] = cpu.x[2];
                    // 00e59b4e: SHUFPS XMM5,XMM2,0x0
                    cpu.x[5] = shufps(cpu.x[5], cpu.x[2], 0);
                    // 00e59b52: MULPS XMM5,XMM6
                    cpu.x[5] = mulps(cpu.x[5], cpu.x[6]);
                    // 00e59b55: MOVAPS XMM6,xmmword ptr [ECX + 0x30]
                    cpu.x[6] = load_vec(e, cpu.ecx.wrapping_add(0x30));
                    // 00e59b59: ADDPS XMM1,XMM5
                    cpu.x[1] = addps(cpu.x[1], cpu.x[5]);
                    // 00e59b5c: MOVAPS XMM5,XMM2
                    cpu.x[5] = cpu.x[2];
                    // 00e59b5f: SHUFPS XMM5,XMM2,0xaa
                    cpu.x[5] = shufps(cpu.x[5], cpu.x[2], 170);
                    // 00e59b63: MOVAPS XMM2,xmmword ptr [ECX + 0x70]
                    cpu.x[2] = load_vec(e, cpu.ecx.wrapping_add(0x70));
                    // 00e59b67: MULPS XMM5,XMM2
                    cpu.x[5] = mulps(cpu.x[5], cpu.x[2]);
                    // 00e59b6a: ADDPS XMM1,XMM5
                    cpu.x[1] = addps(cpu.x[1], cpu.x[5]);
                    // 00e59b6d: MOVAPS XMM2,XMM4
                    cpu.x[2] = cpu.x[4];
                    // 00e59b70: MULPS XMM2,XMM4
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[4]);
                    // 00e59b73: MOVAPS xmmword ptr [EAX + -0x40],XMM4
                    store_vec(e, cpu.eax.wrapping_sub(0x40), cpu.x[4]);
                    // 00e59b77: MOVAPS XMM4,XMM1
                    cpu.x[4] = cpu.x[1];
                    // 00e59b7a: MULPS XMM4,XMM1
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[1]);
                    // 00e59b7d: MULPS XMM2,XMM7
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[7]);
                    // 00e59b80: MULPS XMM4,XMM6
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[6]);
                    // 00e59b83: ADDPS XMM2,XMM4
                    cpu.x[2] = addps(cpu.x[2], cpu.x[4]);
                    // 00e59b86: MOVAPS XMM4,XMM2
                    cpu.x[4] = cpu.x[2];
                    // 00e59b89: MOVAPS XMM5,XMM7
                    cpu.x[5] = cpu.x[7];
                    // 00e59b8c: MOVAPS xmmword ptr [EAX + -0x30],XMM1
                    store_vec(e, cpu.eax.wrapping_sub(0x30), cpu.x[1]);
                    // 00e59b90: ADDPS XMM5,XMM6
                    cpu.x[5] = addps(cpu.x[5], cpu.x[6]);
                    // 00e59b93: UNPCKHPS XMM2,XMM5
                    cpu.x[2] = unpckhps(cpu.x[2], cpu.x[5]);
                    // 00e59b96: SHUFPS XMM4,XMM2,0xc4
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[2], 196);
                    // 00e59b9a: MOVAPS XMM1,XMM4
                    cpu.x[1] = cpu.x[4];
                    // 00e59b9d: SHUFPS XMM1,XMM4,0x4e
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[4], 78);
                    // 00e59ba1: ADDPS XMM1,XMM4
                    cpu.x[1] = addps(cpu.x[1], cpu.x[4]);
                    // 00e59ba4: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00e59ba7: SHUFPS XMM2,XMM1,0xb1
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[1], 177);
                    // 00e59bab: ADDPS XMM2,XMM1
                    cpu.x[2] = addps(cpu.x[2], cpu.x[1]);
                    // 00e59bae: MOVSS dword ptr [ESP + 0x28],XMM2
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), cpu.x[2][0].to_bits());
                    // 00e59bb4: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00e59bb8: FADD ST0,ST2
                    {
                        let v = x87_add(cpu.st(0), cpu.st(2));
                        cpu.set_st(0, v);
                    }
                    // 00e59bba: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59bbe: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e59bc2: MOVAPS XMM1,xmmword ptr [EDX + -0x10]
                    cpu.x[1] = load_vec(e, cpu.edx.wrapping_sub(0x10));
                    // 00e59bc6: FST float ptr [EAX + -0x24]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x24), x87_store32(cpu.st(0)));
                    // 00e59bc9: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00e59bcc: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e59bce: SHUFPS XMM2,XMM1,0xc9
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[1], 201);
                    // 00e59bd2: FDIVR ST0,ST2
                    {
                        let v = x87_div(cpu.st(2), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e59bd4: FSTP float ptr [EAX + -0x34]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x34), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59bd7: FADD float ptr [ESI]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(cpu.esi)));
                        cpu.set_st(0, v);
                    }
                    // 00e59bd9: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59bdd: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e59be1: FST float ptr [EAX + 0x2c]
                    e.mem
                        .set_u32(cpu.eax.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    // 00e59be4: FDIVR ST0,ST1
                    {
                        let v = x87_div(cpu.st(1), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e59be6: FSTP float ptr [EAX + 0x28]
                    e.mem
                        .set_u32(cpu.eax.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59be9: MOVAPS xmmword ptr [EAX + -0x20],XMM1
                    store_vec(e, cpu.eax.wrapping_sub(0x20), cpu.x[1]);
                    // 00e59bed: SHUFPS XMM1,XMM1,0xd2
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[1], 210);
                    // 00e59bf1: MOVAPS XMM4,XMM1
                    cpu.x[4] = cpu.x[1];
                    // 00e59bf4: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e59bf7: SHUFPS XMM1,XMM0,0xc9
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 201);
                    // 00e59bfb: MULPS XMM1,XMM4
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[4]);
                    // 00e59bfe: MOVAPS xmmword ptr [ESP + 0x50],XMM4
                    store_vec(e, cpu.esp.wrapping_add(0x50), cpu.x[4]);
                    // 00e59c03: MOVAPS XMM4,XMM0
                    cpu.x[4] = cpu.x[0];
                    // 00e59c06: SHUFPS XMM4,XMM0,0xd2
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[0], 210);
                    // 00e59c0a: MOVAPS XMM0,XMM3
                    cpu.x[0] = cpu.x[3];
                    // 00e59c0d: SHUFPS XMM0,XMM3,0xd2
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[3], 210);
                    // 00e59c11: MULPS XMM0,XMM2
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[2]);
                    // 00e59c14: MULPS XMM4,XMM2
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[2]);
                    // 00e59c17: SUBPS XMM1,XMM4
                    cpu.x[1] = subps(cpu.x[1], cpu.x[4]);
                    // 00e59c1a: MOVAPS XMM4,XMM3
                    cpu.x[4] = cpu.x[3];
                    // 00e59c1d: SHUFPS XMM4,XMM3,0xc9
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[3], 201);
                    // 00e59c21: MULPS XMM4,xmmword ptr [ESP + 0x50]
                    cpu.x[4] = mulps(cpu.x[4], load_vec(e, cpu.esp.wrapping_add(0x50)));
                    // 00e59c26: MOVAPS XMM3,xmmword ptr [EBX + 0x130]
                    cpu.x[3] = load_vec(e, cpu.ebx.wrapping_add(0x130));
                    // 00e59c2d: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00e59c30: SHUFPS XMM2,XMM1,0x55
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[1], 85);
                    // 00e59c34: SUBPS XMM0,XMM4
                    cpu.x[0] = subps(cpu.x[0], cpu.x[4]);
                    // 00e59c37: MOVAPS XMM4,xmmword ptr [EBX + 0x120]
                    cpu.x[4] = load_vec(e, cpu.ebx.wrapping_add(0x120));
                    // 00e59c3e: MULPS XMM2,XMM3
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[3]);
                    // 00e59c41: MOVAPS XMM3,XMM1
                    cpu.x[3] = cpu.x[1];
                    // 00e59c44: SHUFPS XMM3,XMM1,0x0
                    cpu.x[3] = shufps(cpu.x[3], cpu.x[1], 0);
                    // 00e59c48: MULPS XMM3,XMM4
                    cpu.x[3] = mulps(cpu.x[3], cpu.x[4]);
                    // 00e59c4b: MOVAPS XMM4,xmmword ptr [ECX + 0x50]
                    cpu.x[4] = load_vec(e, cpu.ecx.wrapping_add(0x50));
                    // 00e59c4f: ADDPS XMM2,XMM3
                    cpu.x[2] = addps(cpu.x[2], cpu.x[3]);
                    // 00e59c52: MOVAPS XMM3,XMM1
                    cpu.x[3] = cpu.x[1];
                    // 00e59c55: SHUFPS XMM3,XMM1,0xaa
                    cpu.x[3] = shufps(cpu.x[3], cpu.x[1], 170);
                    // 00e59c59: MOVAPS XMM1,xmmword ptr [EBX + 0x140]
                    cpu.x[1] = load_vec(e, cpu.ebx.wrapping_add(0x140));
                    // 00e59c60: MULPS XMM3,XMM1
                    cpu.x[3] = mulps(cpu.x[3], cpu.x[1]);
                    // 00e59c63: ADDPS XMM2,XMM3
                    cpu.x[2] = addps(cpu.x[2], cpu.x[3]);
                    // 00e59c66: MOVAPS XMM3,xmmword ptr [ECX + 0x60]
                    cpu.x[3] = load_vec(e, cpu.ecx.wrapping_add(0x60));
                    // 00e59c6a: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e59c6d: SHUFPS XMM1,XMM0,0x55
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 85);
                    // 00e59c71: MULPS XMM1,XMM3
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[3]);
                    // 00e59c74: MOVAPS XMM3,XMM0
                    cpu.x[3] = cpu.x[0];
                    // 00e59c77: SHUFPS XMM3,XMM0,0x0
                    cpu.x[3] = shufps(cpu.x[3], cpu.x[0], 0);
                    // 00e59c7b: MULPS XMM3,XMM4
                    cpu.x[3] = mulps(cpu.x[3], cpu.x[4]);
                    // 00e59c7e: ADDPS XMM1,XMM3
                    cpu.x[1] = addps(cpu.x[1], cpu.x[3]);
                    // 00e59c81: MOVAPS XMM3,XMM0
                    cpu.x[3] = cpu.x[0];
                    // 00e59c84: SHUFPS XMM3,XMM0,0xaa
                    cpu.x[3] = shufps(cpu.x[3], cpu.x[0], 170);
                    // 00e59c88: MOVAPS XMM0,xmmword ptr [ECX + 0x70]
                    cpu.x[0] = load_vec(e, cpu.ecx.wrapping_add(0x70));
                    // 00e59c8c: MULPS XMM3,XMM0
                    cpu.x[3] = mulps(cpu.x[3], cpu.x[0]);
                    // 00e59c8f: ADDPS XMM1,XMM3
                    cpu.x[1] = addps(cpu.x[1], cpu.x[3]);
                    // 00e59c92: MOVAPS XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00e59c95: MULPS XMM0,XMM1
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[1]);
                    // 00e59c98: MOVAPS xmmword ptr [EAX],XMM1
                    store_vec(e, cpu.eax, cpu.x[1]);
                    // 00e59c9b: MULPS XMM0,XMM6
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[6]);
                    // 00e59c9e: MOVAPS XMM1,XMM2
                    cpu.x[1] = cpu.x[2];
                    // 00e59ca1: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00e59ca4: MULPS XMM1,XMM7
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[7]);
                    // 00e59ca7: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00e59caa: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e59cad: UNPCKHPS XMM0,XMM5
                    cpu.x[0] = unpckhps(cpu.x[0], cpu.x[5]);
                    // 00e59cb0: SHUFPS XMM1,XMM0,0xc4
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 196);
                    // 00e59cb4: MOVAPS XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00e59cb7: SHUFPS XMM0,XMM1,0x4e
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[1], 78);
                    // 00e59cbb: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00e59cbe: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e59cc1: SHUFPS XMM1,XMM0,0xb1
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 177);
                    // 00e59cc5: ADDPS XMM1,XMM0
                    cpu.x[1] = addps(cpu.x[1], cpu.x[0]);
                    // 00e59cc8: MOVSS dword ptr [ESP + 0x40],XMM1
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x40), cpu.x[1][0].to_bits());
                    // 00e59cce: FLD float ptr [ESP + 0x40]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x40))));
                    // 00e59cd2: MOVAPS xmmword ptr [EAX + -0x10],XMM2
                    store_vec(e, cpu.eax.wrapping_sub(0x10), cpu.x[2]);
                    // 00e59cd6: FADD ST0,ST2
                    {
                        let v = x87_add(cpu.st(0), cpu.st(2));
                        cpu.set_st(0, v);
                    }
                    // 00e59cd8: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59cdc: MOV ECX,dword ptr [EDX + 0x14]
                    cpu.ecx = e.mem.u32(cpu.edx.wrapping_add(0x14));
                    // 00e59cdf: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e59ce3: MOVAPS XMM0,xmmword ptr [ECX + 0x10]
                    cpu.x[0] = load_vec(e, cpu.ecx.wrapping_add(0x10));
                    // 00e59ce7: FST float ptr [EAX + 0xc]
                    e.mem
                        .set_u32(cpu.eax.wrapping_add(0xc), x87_store32(cpu.st(0)));
                    // 00e59cea: MOVAPS XMM1,xmmword ptr [ECX + 0x20]
                    cpu.x[1] = load_vec(e, cpu.ecx.wrapping_add(0x20));
                    // 00e59cee: FDIVR ST0,ST1
                    {
                        let v = x87_div(cpu.st(1), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e59cf0: MOVAPS XMM2,xmmword ptr [EBX + 0x160]
                    cpu.x[2] = load_vec(e, cpu.ebx.wrapping_add(0x160));
                    // 00e59cf7: MOVAPS XMM3,xmmword ptr [EBX + 0x170]
                    cpu.x[3] = load_vec(e, cpu.ebx.wrapping_add(0x170));
                    // 00e59cfe: SUBPS XMM2,XMM0
                    cpu.x[2] = subps(cpu.x[2], cpu.x[0]);
                    // 00e59d01: MOVAPS XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00e59d04: ADD EDX,0x60
                    cpu.edx = cpu.add32(cpu.edx, 0x60);
                    // 00e59d07: ADD EAX,0xa0
                    cpu.eax = cpu.add32(cpu.eax, 0xa0);
                    // 00e59d0c: ADD ESI,0x64
                    cpu.esi = cpu.add32(cpu.esi, 0x64);
                    // 00e59d0f: ADD EDI,0x24
                    cpu.edi = cpu.add32(cpu.edi, 0x24);
                    // 00e59d12: SUB dword ptr [ESP + 0x20],0x1
                    {
                        let at = cpu.esp.wrapping_add(0x20);
                        let v = cpu.sub32(e.mem.u32(at), 0x1);
                        e.mem.set_u32(at, v);
                    }
                    // 00e59d17: FSTP float ptr [EAX + 0xffffff5c]
                    e.mem
                        .set_u32(cpu.eax.wrapping_add(0xffffff5c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59d1d: FLD float ptr [EAX + 0xffffff5c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax.wrapping_add(0xffffff5c))));
                    // 00e59d23: MOVAPS XMM4,xmmword ptr [EAX + 0xffffff30]
                    cpu.x[4] = load_vec(e, cpu.eax.wrapping_add(0xffffff30));
                    // 00e59d2a: FSTP float ptr [EAX + -0x80]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x80), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59d2d: MOVAPS XMM5,xmmword ptr [EAX + 0xffffff20]
                    cpu.x[5] = load_vec(e, cpu.eax.wrapping_add(0xffffff20));
                    // 00e59d34: FLD float ptr [EAX + 0xffffff6c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax.wrapping_add(0xffffff6c))));
                    // 00e59d3a: MULPS XMM0,XMM4
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[4]);
                    // 00e59d3d: FSTP float ptr [EAX + -0x7c]
                    e.mem
                        .set_u32(cpu.eax.wrapping_sub(0x7c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59d40: FLD float ptr [EDI + -0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edi.wrapping_sub(0x28))));
                    // 00e59d43: MOVAPS XMM4,XMM3
                    cpu.x[4] = cpu.x[3];
                    // 00e59d46: MULPS XMM4,XMM5
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[5]);
                    // 00e59d49: MOVAPS XMM5,xmmword ptr [EAX + 0xffffff10]
                    cpu.x[5] = load_vec(e, cpu.eax.wrapping_add(0xffffff10));
                    // 00e59d50: ADDPS XMM0,XMM4
                    cpu.x[0] = addps(cpu.x[0], cpu.x[4]);
                    // 00e59d53: MOVAPS XMM4,XMM2
                    cpu.x[4] = cpu.x[2];
                    // 00e59d56: MULPS XMM4,XMM5
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[5]);
                    // 00e59d59: ADDPS XMM0,XMM4
                    cpu.x[0] = addps(cpu.x[0], cpu.x[4]);
                    // 00e59d5c: MOVAPS XMM4,XMM0
                    cpu.x[4] = cpu.x[0];
                    // 00e59d5f: SHUFPS XMM4,XMM0,0x55
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[0], 85);
                    // 00e59d63: MOVAPS XMM5,XMM0
                    cpu.x[5] = cpu.x[0];
                    // 00e59d66: ADDSS XMM4,XMM0
                    cpu.x[4] = addss(cpu.x[4], cpu.x[0]);
                    // 00e59d6a: SHUFPS XMM5,XMM0,0xaa
                    cpu.x[5] = shufps(cpu.x[5], cpu.x[0], 170);
                    // 00e59d6e: MOVAPS XMM0,xmmword ptr [EAX + 0xffffff60]
                    cpu.x[0] = load_vec(e, cpu.eax.wrapping_add(0xffffff60));
                    // 00e59d75: MULPS XMM1,XMM0
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[0]);
                    // 00e59d78: MOVAPS XMM0,xmmword ptr [EAX + 0xffffff50]
                    cpu.x[0] = load_vec(e, cpu.eax.wrapping_add(0xffffff50));
                    // 00e59d7f: MULPS XMM3,XMM0
                    cpu.x[3] = mulps(cpu.x[3], cpu.x[0]);
                    // 00e59d82: MOVAPS XMM0,xmmword ptr [EAX + 0xffffff40]
                    cpu.x[0] = load_vec(e, cpu.eax.wrapping_add(0xffffff40));
                    // 00e59d89: MULPS XMM2,XMM0
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[0]);
                    // 00e59d8c: ADDPS XMM1,XMM3
                    cpu.x[1] = addps(cpu.x[1], cpu.x[3]);
                    // 00e59d8f: ADDPS XMM1,XMM2
                    cpu.x[1] = addps(cpu.x[1], cpu.x[2]);
                    // 00e59d92: MOVAPS XMM0,XMM1
                    cpu.x[0] = cpu.x[1];
                    // 00e59d95: SHUFPS XMM0,XMM1,0x55
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[1], 85);
                    // 00e59d99: MOVAPS XMM2,XMM1
                    cpu.x[2] = cpu.x[1];
                    // 00e59d9c: ADDSS XMM0,XMM1
                    cpu.x[0] = addss(cpu.x[0], cpu.x[1]);
                    // 00e59da0: SHUFPS XMM2,XMM1,0xaa
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[1], 170);
                    // 00e59da4: ADDSS XMM2,XMM0
                    cpu.x[2] = addss(cpu.x[2], cpu.x[0]);
                    // 00e59da8: MOVSS dword ptr [ESP + 0x48],XMM2
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x48), cpu.x[2][0].to_bits());
                    // 00e59dae: FADD float ptr [ESP + 0x48]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x48))));
                        cpu.set_st(0, v);
                    }
                    // 00e59db2: ADDSS XMM5,XMM4
                    cpu.x[5] = addss(cpu.x[5], cpu.x[4]);
                    // 00e59db6: MOVSS dword ptr [ESP + 0x2c],XMM5
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), cpu.x[5][0].to_bits());
                    // 00e59dbc: FSTP float ptr [EDI + -0x28]
                    e.mem
                        .set_u32(cpu.edi.wrapping_sub(0x28), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59dbf: FLD float ptr [EDI + -0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edi.wrapping_sub(0x24))));
                    // 00e59dc2: FADD float ptr [ESP + 0x2c]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x2c))));
                        cpu.set_st(0, v);
                    }
                    // 00e59dc6: FSTP float ptr [EDI + -0x24]
                    e.mem
                        .set_u32(cpu.edi.wrapping_sub(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59dc9: JNZ 0x00e59a9b
                    if !cpu.zf {
                        pc = 0x00e59a9b;
                        continue 'run;
                    }
                    // 00e59dcf: MOV EAX,dword ptr [EBX + 0x30]
                    cpu.eax = e.mem.u32(cpu.ebx.wrapping_add(0x30));
                    // 00e59dd2: MOVAPS XMM1,xmmword ptr [ESP + 0x140]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0x140));
                    // 00e59dda: LEA EDX,[EBX + 0x30]
                    cpu.edx = cpu.ebx.wrapping_add(0x30);
                    // 00e59ddd: CMP EAX,dword ptr [EBX + 0x90]
                    cpu.cmp32(cpu.eax, e.mem.u32(cpu.ebx.wrapping_add(0x90)));
                    // 00e59de3: JNZ 0x00e59e4b
                    if !cpu.zf {
                        pc = 0x00e59e4b;
                        continue 'run;
                    }
                    // 00e59de5: MOVAPS XMM0,xmmword ptr [ESP + 0xa0]
                    cpu.x[0] = load_vec(e, cpu.esp.wrapping_add(0xa0));
                    // 00e59ded: MOVAPS XMM2,xmmword ptr [EAX + 0x30]
                    cpu.x[2] = load_vec(e, cpu.eax.wrapping_add(0x30));
                    // 00e59df1: MULPS XMM1,XMM0
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[0]);
                    // 00e59df4: MOVAPS XMM0,xmmword ptr [EBX + 0x100]
                    cpu.x[0] = load_vec(e, cpu.ebx.wrapping_add(0x100));
                    // 00e59dfb: MOVAPS XMM3,XMM2
                    cpu.x[3] = cpu.x[2];
                    // 00e59dfe: SHUFPS XMM3,XMM2,0xff
                    cpu.x[3] = shufps(cpu.x[3], cpu.x[2], 255);
                    // 00e59e02: MOVAPS XMM2,xmmword ptr [ESP + 0xc0]
                    cpu.x[2] = load_vec(e, cpu.esp.wrapping_add(0xc0));
                    // 00e59e0a: MULPS XMM3,XMM1
                    cpu.x[3] = mulps(cpu.x[3], cpu.x[1]);
                    // 00e59e0d: SHUFPS XMM0,XMM0,0xff
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 255);
                    // 00e59e11: MULPS XMM0,XMM1
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[1]);
                    // 00e59e14: MOVAPS XMM1,xmmword ptr [ESP + 0x160]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0x160));
                    // 00e59e1c: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00e59e1f: MOVAPS XMM2,xmmword ptr [EAX + 0x30]
                    cpu.x[2] = load_vec(e, cpu.eax.wrapping_add(0x30));
                    // 00e59e23: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00e59e26: MOVAPS XMM2,xmmword ptr [ESP + 0x150]
                    cpu.x[2] = load_vec(e, cpu.esp.wrapping_add(0x150));
                    // 00e59e2e: ADDPS XMM0,XMM3
                    cpu.x[0] = addps(cpu.x[0], cpu.x[3]);
                    // 00e59e31: MOVAPS XMM3,xmmword ptr [ESP + 0xb0]
                    cpu.x[3] = load_vec(e, cpu.esp.wrapping_add(0xb0));
                    // 00e59e39: MULPS XMM2,XMM3
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[3]);
                    // 00e59e3c: MOVAPS XMM3,xmmword ptr [EBX + 0x100]
                    cpu.x[3] = load_vec(e, cpu.ebx.wrapping_add(0x100));
                    // 00e59e43: MULPS XMM2,XMM3
                    cpu.x[2] = mulps(cpu.x[2], cpu.x[3]);
                    // 00e59e46: ADDPS XMM1,XMM2
                    cpu.x[1] = addps(cpu.x[1], cpu.x[2]);
                    // 00e59e49: JMP 0x00e59e81
                    pc = 0x00e59e81;
                    continue 'run;
                }
                0x00e59e4b => {
                    // 00e59e4b: MOVAPS XMM2,xmmword ptr [ESP + 0xa0]
                    cpu.x[2] = load_vec(e, cpu.esp.wrapping_add(0xa0));
                    // 00e59e53: MOVAPS XMM0,xmmword ptr [EBX + 0x100]
                    cpu.x[0] = load_vec(e, cpu.ebx.wrapping_add(0x100));
                    // 00e59e5a: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00e59e5d: MOVAPS XMM2,xmmword ptr [ESP + 0xb0]
                    cpu.x[2] = load_vec(e, cpu.esp.wrapping_add(0xb0));
                    // 00e59e65: SHUFPS XMM0,XMM0,0xff
                    cpu.x[0] = shufps(cpu.x[0], cpu.x[0], 255);
                    // 00e59e69: MULPS XMM0,XMM1
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[1]);
                    // 00e59e6c: MOVAPS XMM1,xmmword ptr [ESP + 0x150]
                    cpu.x[1] = load_vec(e, cpu.esp.wrapping_add(0x150));
                    // 00e59e74: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00e59e77: MOVAPS XMM2,xmmword ptr [EBX + 0x100]
                    cpu.x[2] = load_vec(e, cpu.ebx.wrapping_add(0x100));
                    // 00e59e7e: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    pc = 0x00e59e81;
                    continue 'run;
                }
                0x00e59e81 => {
                    // 00e59e81: MOV ESI,dword ptr [EBP + 0x14]
                    cpu.esi = e.mem.u32(cpu.ebp.wrapping_add(0x14));
                    // 00e59e84: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00e59e87: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e59e8a: SHUFPS XMM1,XMM0,0x55
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 85);
                    // 00e59e8e: MOVAPS XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00e59e91: ADDSS XMM1,XMM0
                    cpu.x[1] = addss(cpu.x[1], cpu.x[0]);
                    // 00e59e95: SHUFPS XMM2,XMM0,0xaa
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[0], 170);
                    // 00e59e99: ADDSS XMM2,XMM1
                    cpu.x[2] = addss(cpu.x[2], cpu.x[1]);
                    // 00e59e9d: MOVSS dword ptr [ESP + 0x28],XMM2
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), cpu.x[2][0].to_bits());
                    // 00e59ea3: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00e59ea7: FSTP float ptr [ESP + 0x44]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x44), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59eab: LEA ECX,[ESP + 0x70]
                    cpu.ecx = cpu.esp.wrapping_add(0x70);
                    // 00e59eaf: FLD float ptr [ESP + 0xe4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xe4))));
                    // 00e59eb6: LEA EDI,[ESI + 0x10]
                    cpu.edi = cpu.esi.wrapping_add(0x10);
                    // 00e59eb9: FSTP float ptr [ESP + 0x50]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x50), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59ebd: MOV dword ptr [ESP + 0x1c],0x2
                    e.mem.set_u32(cpu.esp.wrapping_add(0x1c), 0x2);
                    // 00e59ec5: FLD float ptr [ESP + 0x184]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x184))));
                    // 00e59ecc: FSTP float ptr [ESP + 0x5c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x5c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59ed0: FLD float ptr [ESP + 0x44]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x44))));
                    // 00e59ed4: FST float ptr [ESP + 0x54]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x54), x87_store32(cpu.st(0)));
                    // 00e59ed8: FSTP float ptr [ESP + 0x58]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x58), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59edc: FLDZ
                    cpu.st_push(0.0);
                    pc = 0x00e59ede;
                    continue 'run;
                }
                0x00e59ede => {
                    // 00e59ede: FLD float ptr [EDI]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edi)));
                    // 00e59ee0: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59ee4: FCOM float ptr [ESP + 0x20]
                    cpu.sw = u16::from(fcom_status(
                        cpu.st(0),
                        x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x20))),
                    )) << 8;
                    // 00e59ee8: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e59eea: TEST AH,0x44
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x44);
                    // 00e59eed: JNP 0x00e59f84
                    if !cpu.pf {
                        pc = 0x00e59f84;
                        continue 'run;
                    }
                    // 00e59ef3: MOVSS XMM3,dword ptr [ESP + 0x20]
                    cpu.x[3] = scalar_vec(e, cpu.esp.wrapping_add(0x20));
                    // 00e59ef9: FST float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    // 00e59efd: MOV EAX,dword ptr [EDX]
                    cpu.eax = e.mem.u32(cpu.edx);
                    // 00e59eff: MOVAPS XMM1,xmmword ptr [EBX + 0x100]
                    cpu.x[1] = load_vec(e, cpu.ebx.wrapping_add(0x100));
                    // 00e59f06: MOVAPS XMM0,xmmword ptr [EAX + 0x30]
                    cpu.x[0] = load_vec(e, cpu.eax.wrapping_add(0x30));
                    // 00e59f0a: MOVAPS XMM5,xmmword ptr [EBX + 0xe0]
                    cpu.x[5] = load_vec(e, cpu.ebx.wrapping_add(0xe0));
                    // 00e59f11: MOVAPS XMM2,XMM3
                    cpu.x[2] = cpu.x[3];
                    // 00e59f14: SHUFPS XMM2,XMM3,0x0
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[3], 0);
                    // 00e59f18: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00e59f1b: MULPS XMM0,XMM2
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[2]);
                    // 00e59f1e: MOVAPS XMM2,xmmword ptr [ECX]
                    cpu.x[2] = load_vec(e, cpu.ecx);
                    // 00e59f21: MOVAPS XMM4,XMM1
                    cpu.x[4] = cpu.x[1];
                    // 00e59f24: SHUFPS XMM4,XMM1,0xff
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[1], 255);
                    // 00e59f28: MULPS XMM4,XMM2
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[2]);
                    // 00e59f2b: ADDPS XMM4,XMM5
                    cpu.x[4] = addps(cpu.x[4], cpu.x[5]);
                    // 00e59f2e: MOVAPS xmmword ptr [EBX + 0xe0],XMM4
                    store_vec(e, cpu.ebx.wrapping_add(0xe0), cpu.x[4]);
                    // 00e59f35: MOVAPS XMM4,XMM0
                    cpu.x[4] = cpu.x[0];
                    // 00e59f38: SHUFPS XMM4,XMM0,0xff
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[0], 255);
                    // 00e59f3c: MULPS XMM4,XMM2
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[2]);
                    // 00e59f3f: MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    cpu.x[2] = load_vec(e, cpu.eax.wrapping_add(0x10));
                    // 00e59f43: SUBPS XMM2,XMM4
                    cpu.x[2] = subps(cpu.x[2], cpu.x[4]);
                    // 00e59f46: MOVAPS XMM4,xmmword ptr [ECX + 0x10]
                    cpu.x[4] = load_vec(e, cpu.ecx.wrapping_add(0x10));
                    // 00e59f4a: MOVAPS xmmword ptr [EAX + 0x10],XMM2
                    store_vec(e, cpu.eax.wrapping_add(0x10), cpu.x[2]);
                    // 00e59f4e: MOVSS XMM2,dword ptr [ESP + 0x2c]
                    cpu.x[2] = scalar_vec(e, cpu.esp.wrapping_add(0x2c));
                    // 00e59f54: MULPS XMM4,XMM1
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[1]);
                    // 00e59f57: MOVAPS XMM1,xmmword ptr [EBX + 0xf0]
                    cpu.x[1] = load_vec(e, cpu.ebx.wrapping_add(0xf0));
                    // 00e59f5e: ADDPS XMM4,XMM1
                    cpu.x[4] = addps(cpu.x[4], cpu.x[1]);
                    // 00e59f61: MOVAPS XMM1,xmmword ptr [ECX + 0x20]
                    cpu.x[1] = load_vec(e, cpu.ecx.wrapping_add(0x20));
                    // 00e59f65: MOVAPS xmmword ptr [EBX + 0xf0],XMM4
                    store_vec(e, cpu.ebx.wrapping_add(0xf0), cpu.x[4]);
                    // 00e59f6c: MULPS XMM0,XMM1
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[1]);
                    // 00e59f6f: MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    cpu.x[1] = load_vec(e, cpu.eax.wrapping_add(0x20));
                    // 00e59f73: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00e59f76: ADDSS XMM2,XMM3
                    cpu.x[2] = addss(cpu.x[2], cpu.x[3]);
                    // 00e59f7a: MOVAPS xmmword ptr [EAX + 0x20],XMM0
                    store_vec(e, cpu.eax.wrapping_add(0x20), cpu.x[0]);
                    // 00e59f7e: MOVSS dword ptr [ESP + 0x2c],XMM2
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), cpu.x[2][0].to_bits());
                    pc = 0x00e59f84;
                    continue 'run;
                }
                0x00e59f84 => {
                    // 00e59f84: ADD EDI,0x24
                    cpu.edi = cpu.add32(cpu.edi, 0x24);
                    // 00e59f87: ADD EDX,0x60
                    cpu.edx = cpu.add32(cpu.edx, 0x60);
                    // 00e59f8a: ADD ECX,0xa0
                    cpu.ecx = cpu.add32(cpu.ecx, 0xa0);
                    // 00e59f90: SUB dword ptr [ESP + 0x1c],0x1
                    {
                        let at = cpu.esp.wrapping_add(0x1c);
                        let v = cpu.sub32(e.mem.u32(at), 0x1);
                        e.mem.set_u32(at, v);
                    }
                    // 00e59f95: JNZ 0x00e59ede
                    if !cpu.zf {
                        pc = 0x00e59ede;
                        continue 'run;
                    }
                    // 00e59f9b: FLD float ptr [EBX + 0x44]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ebx.wrapping_add(0x44))));
                    // 00e59f9e: MOV EAX,dword ptr [EBP + 0xc]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0xc));
                    // 00e59fa1: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59fa5: LEA ECX,[ESI + 0x18]
                    cpu.ecx = cpu.esi.wrapping_add(0x18);
                    // 00e59fa8: FLD float ptr [EBX + 0xa4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ebx.wrapping_add(0xa4))));
                    // 00e59fae: MOV dword ptr [ESP + 0x2c],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x2c), cpu.ecx);
                    // 00e59fb2: FSTP float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59fb6: LEA EDI,[ESP + 0xb0]
                    cpu.edi = cpu.esp.wrapping_add(0xb0);
                    // 00e59fbd: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00e59fc1: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e59fc3: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e59fc7: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e59fc9: FADDP ST2,ST0
                    {
                        let v = x87_add(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59fcb: FXCH
                    cpu.st_exchange(1);
                    // 00e59fcd: FSTP float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59fd1: FLD float ptr [EAX + 0x68]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax.wrapping_add(0x68))));
                    // 00e59fd4: FLD float ptr [ESP + 0x28]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x28))));
                    // 00e59fd8: MOV dword ptr [ESP + 0x28],0x2
                    e.mem.set_u32(cpu.esp.wrapping_add(0x28), 0x2);
                    // 00e59fe0: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e59fe2: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59fe4: FLD float ptr [EAX + 0x64]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax.wrapping_add(0x64))));
                    // 00e59fe7: FMULP ST3
                    {
                        let v = x87_mul(cpu.st(3), cpu.st(0));
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    // 00e59fe9: FXCH
                    cpu.st_exchange(1);
                    // 00e59feb: FADDP ST2,ST0
                    {
                        let v = x87_add(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e59fed: FXCH
                    cpu.st_exchange(1);
                    // 00e59fef: FSTP float ptr [ESP + 0xdc]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xdc), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e59ff6: FMUL float ptr [EAX + 0xcc]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.eax.wrapping_add(0xcc))));
                        cpu.set_st(0, v);
                    }
                    // 00e59ffc: FLD float ptr [EAX + 0xc8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax.wrapping_add(0xc8))));
                    // 00e5a002: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a004: FADDP
                    {
                        let v = x87_add(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a006: FSTP float ptr [ESP + 0x17c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x17c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a00d: FST float ptr [ESI + 0x8]
                    e.mem
                        .set_u32(cpu.esi.wrapping_add(0x8), x87_store32(cpu.st(0)));
                    // 00e5a010: FST float ptr [ESI + 0x2c]
                    e.mem
                        .set_u32(cpu.esi.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    // 00e5a013: LEA ESI,[EBX + 0x48]
                    cpu.esi = cpu.ebx.wrapping_add(0x48);
                    pc = 0x00e5a016;
                    continue 'run;
                }
                0x00e5a016 => {
                    // 00e5a016: CMP byte ptr [ESI + 0x8],0x0
                    cpu.cmp8(u32::from(e.mem.u8(cpu.esi.wrapping_add(0x8))), 0x0);
                    // 00e5a01a: FLD double ptr [0x01011588]
                    cpu.st_push(e.mem.f64(0x01011588));
                    // 00e5a020: MOV EAX,dword ptr [ESI + -0x18]
                    cpu.eax = e.mem.u32(cpu.esi.wrapping_sub(0x18));
                    // 00e5a023: MOVAPS XMM0,xmmword ptr [EAX + 0x10]
                    cpu.x[0] = load_vec(e, cpu.eax.wrapping_add(0x10));
                    // 00e5a027: MOVAPS XMM3,xmmword ptr [EAX + 0x20]
                    cpu.x[3] = load_vec(e, cpu.eax.wrapping_add(0x20));
                    // 00e5a02b: MOVAPS XMM2,xmmword ptr [EBX + 0xe0]
                    cpu.x[2] = load_vec(e, cpu.ebx.wrapping_add(0xe0));
                    // 00e5a032: MOVAPS XMM1,xmmword ptr [EBX + 0xf0]
                    cpu.x[1] = load_vec(e, cpu.ebx.wrapping_add(0xf0));
                    // 00e5a039: MOVAPS XMM4,xmmword ptr [EDI + -0x20]
                    cpu.x[4] = load_vec(e, cpu.edi.wrapping_sub(0x20));
                    // 00e5a03d: SUBPS XMM2,XMM0
                    cpu.x[2] = subps(cpu.x[2], cpu.x[0]);
                    // 00e5a040: MOVAPS XMM0,xmmword ptr [EDI + -0x30]
                    cpu.x[0] = load_vec(e, cpu.edi.wrapping_sub(0x30));
                    // 00e5a044: MULPS XMM0,XMM1
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[1]);
                    // 00e5a047: MULPS XMM4,XMM3
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[3]);
                    // 00e5a04a: ADDPS XMM0,XMM4
                    cpu.x[0] = addps(cpu.x[0], cpu.x[4]);
                    // 00e5a04d: MOVAPS XMM4,xmmword ptr [EDI + -0x40]
                    cpu.x[4] = load_vec(e, cpu.edi.wrapping_sub(0x40));
                    // 00e5a051: MULPS XMM4,XMM2
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[2]);
                    // 00e5a054: ADDPS XMM0,XMM4
                    cpu.x[0] = addps(cpu.x[0], cpu.x[4]);
                    // 00e5a057: MOVAPS XMM4,XMM0
                    cpu.x[4] = cpu.x[0];
                    // 00e5a05a: SHUFPS XMM4,XMM0,0x55
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[0], 85);
                    // 00e5a05e: MOVAPS XMM5,XMM0
                    cpu.x[5] = cpu.x[0];
                    // 00e5a061: ADDSS XMM4,XMM0
                    cpu.x[4] = addss(cpu.x[4], cpu.x[0]);
                    // 00e5a065: SHUFPS XMM5,XMM0,0xaa
                    cpu.x[5] = shufps(cpu.x[5], cpu.x[0], 170);
                    // 00e5a069: ADDSS XMM5,XMM4
                    cpu.x[5] = addss(cpu.x[5], cpu.x[4]);
                    // 00e5a06d: MOVSS dword ptr [ESP + 0x6c],XMM5
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x6c), cpu.x[5][0].to_bits());
                    // 00e5a073: FLD float ptr [ESP + 0x6c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x6c))));
                    // 00e5a077: FST float ptr [EDI + 0x20]
                    e.mem
                        .set_u32(cpu.edi.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    // 00e5a07a: MOVAPS XMM0,xmmword ptr [EDI + 0x10]
                    cpu.x[0] = load_vec(e, cpu.edi.wrapping_add(0x10));
                    // 00e5a07e: FLD float ptr [ECX + -0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ecx.wrapping_sub(0x4))));
                    // 00e5a081: MOV EAX,dword ptr [EBP + 0x8]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0x8));
                    // 00e5a084: FMUL double ptr [0x010290b0]
                    {
                        let v = x87_mul(cpu.st(0), e.mem.f64(0x010290b0));
                        cpu.set_st(0, v);
                    }
                    // 00e5a08a: MULPS XMM0,XMM3
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[3]);
                    // 00e5a08d: MOVAPS XMM3,xmmword ptr [EDI]
                    cpu.x[3] = load_vec(e, cpu.edi);
                    // 00e5a090: MULPS XMM1,XMM3
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[3]);
                    // 00e5a093: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00e5a096: MOVAPS XMM1,xmmword ptr [EDI + -0x10]
                    cpu.x[1] = load_vec(e, cpu.edi.wrapping_sub(0x10));
                    // 00e5a09a: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00e5a09d: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00e5a0a0: MOVAPS XMM1,XMM0
                    cpu.x[1] = cpu.x[0];
                    // 00e5a0a3: SHUFPS XMM1,XMM0,0x55
                    cpu.x[1] = shufps(cpu.x[1], cpu.x[0], 85);
                    // 00e5a0a7: MOVAPS XMM2,XMM0
                    cpu.x[2] = cpu.x[0];
                    // 00e5a0aa: ADDSS XMM1,XMM0
                    cpu.x[1] = addss(cpu.x[1], cpu.x[0]);
                    // 00e5a0ae: SHUFPS XMM2,XMM0,0xaa
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[0], 170);
                    // 00e5a0b2: ADDSS XMM2,XMM1
                    cpu.x[2] = addss(cpu.x[2], cpu.x[1]);
                    // 00e5a0b6: MOVSS dword ptr [ESP + 0x30],XMM2
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x30), cpu.x[2][0].to_bits());
                    // 00e5a0bc: FADD float ptr [ESP + 0x30]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x30))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a0c0: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a0c4: FLD float ptr [ESP + 0x20]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x20))));
                    // 00e5a0c8: FSTP float ptr [EDI + 0x28]
                    e.mem
                        .set_u32(cpu.edi.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a0cb: FLD float ptr [EDI + 0x2c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edi.wrapping_add(0x2c))));
                    // 00e5a0ce: FABS
                    {
                        let v = cpu.st(0).abs();
                        cpu.set_st(0, v);
                    }
                    // 00e5a0d0: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a0d4: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e5a0d8: FMUL float ptr [EAX]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.eax)));
                        cpu.set_st(0, v);
                    }
                    // 00e5a0da: FSTP float ptr [ESP + 0x40]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x40), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a0de: FLD float ptr [ESI]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi)));
                    // 00e5a0e0: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a0e4: JZ 0x00e5a18a
                    if cpu.zf {
                        pc = 0x00e5a18a;
                        continue 'run;
                    }
                    // 00e5a0ea: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a0ec: FCHS
                    {
                        let v = -cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    // 00e5a0ee: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a0f2: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e5a0f6: FST float ptr [EDI + 0x24]
                    e.mem
                        .set_u32(cpu.edi.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    // 00e5a0f9: FST float ptr [ESP + 0x48]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x48), x87_store32(cpu.st(0)));
                    // 00e5a0fd: FLD float ptr [EDI + 0x30]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edi.wrapping_add(0x30))));
                    // 00e5a100: FMUL float ptr [EDI + 0x3c]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.edi.wrapping_add(0x3c))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a103: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a107: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e5a10b: FLD float ptr [ECX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ecx)));
                    // 00e5a10d: FMUL ST4
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(4));
                        cpu.set_st(0, v);
                    }
                    // 00e5a10f: FADDP ST3,ST0
                    {
                        let v = x87_add(cpu.st(3), cpu.st(0));
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    // 00e5a111: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a113: FXCH
                    cpu.st_exchange(1);
                    // 00e5a115: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a117: FXCH
                    cpu.st_exchange(1);
                    // 00e5a119: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a11d: FLD float ptr [ESI + -0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_sub(0x10))));
                    // 00e5a120: FMUL float ptr [ESP + 0x40]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x40))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a124: FMUL float ptr [EDI + 0x3c]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.edi.wrapping_add(0x3c))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a127: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a12b: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e5a12f: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a131: FABS
                    {
                        let v = cpu.st(0).abs();
                        cpu.set_st(0, v);
                    }
                    // 00e5a133: FSTP float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a137: FLD float ptr [ESP + 0x24]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                    // 00e5a13b: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a13f: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e5a141: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a143: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a145: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e5a148: JP 0x00e5a179
                    if cpu.pf {
                        pc = 0x00e5a179;
                        continue 'run;
                    }
                    // 00e5a14a: FXCH
                    cpu.st_exchange(1);
                    // 00e5a14c: FCOMP ST3
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(3))) << 8;
                    cpu.st_pop();
                    // 00e5a14e: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a150: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a153: JNZ 0x00e5a166
                    if !cpu.zf {
                        pc = 0x00e5a166;
                        continue 'run;
                    }
                    // 00e5a155: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a159: FSUB float ptr [ESP + 0x1c]
                    {
                        let v = x87_sub(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a15d: FSTP float ptr [EDI + 0x24]
                    e.mem
                        .set_u32(cpu.edi.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a160: FST float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    // 00e5a164: JMP 0x00e5a1a2
                    pc = 0x00e5a1a2;
                    continue 'run;
                }
                0x00e5a166 => {
                    // 00e5a166: FCHS
                    {
                        let v = -cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    // 00e5a168: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a16c: FSUB float ptr [ESP + 0x1c]
                    {
                        let v = x87_sub(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a170: FSTP float ptr [EDI + 0x24]
                    e.mem
                        .set_u32(cpu.edi.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a173: FST float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    // 00e5a177: JMP 0x00e5a1a2
                    pc = 0x00e5a1a2;
                    continue 'run;
                }
                0x00e5a179 => {
                    // 00e5a179: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a17b: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a17d: FSUB float ptr [ESP + 0x1c]
                    {
                        let v = x87_sub(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a181: FSTP float ptr [EDI + 0x24]
                    e.mem
                        .set_u32(cpu.edi.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a184: FST float ptr [ESP + 0x24]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    // 00e5a188: JMP 0x00e5a1a2
                    pc = 0x00e5a1a2;
                    continue 'run;
                }
                0x00e5a18a => {
                    // 00e5a18a: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a18c: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a18e: FST float ptr [ECX]
                    e.mem.set_u32(cpu.ecx, x87_store32(cpu.st(0)));
                    // 00e5a190: FLD float ptr [ECX + -0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ecx.wrapping_sub(0x18))));
                    // 00e5a193: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a197: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a19b: FST float ptr [EDI + 0x24]
                    e.mem
                        .set_u32(cpu.edi.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    // 00e5a19e: FSTP float ptr [ESP + 0x48]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x48), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e5a1a2;
                    continue 'run;
                }
                0x00e5a1a2 => {
                    // 00e5a1a2: FLD float ptr [ESI + -0x10]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_sub(0x10))));
                    // 00e5a1a5: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a1a9: FCOM float ptr [ESI + -0xc]
                    cpu.sw = u16::from(fcom_status(
                        cpu.st(0),
                        x87_m32(e.mem.u32(cpu.esi.wrapping_sub(0xc))),
                    )) << 8;
                    // 00e5a1ac: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a1ae: TEST AH,0x44
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x44);
                    // 00e5a1b1: JNP 0x00e5a230
                    if !cpu.pf {
                        pc = 0x00e5a230;
                        continue 'run;
                    }
                    // 00e5a1b3: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a1b5: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a1b7: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a1b9: FLD float ptr [ESP + 0x20]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x20))));
                    // 00e5a1bd: FLD float ptr [ESP + 0x48]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x48))));
                    // 00e5a1c1: FLD ST1
                    {
                        let v = cpu.st(1);
                        cpu.st_push(v);
                    }
                    // 00e5a1c3: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a1c5: FMUL ST0
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e5a1c7: FADDP
                    {
                        let v = x87_add(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a1c9: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a1cd: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a1d1: CALL 0x00ec6040
                    {
                        let v = cpu.st(0);
                        cpu.st_pop();
                        let r = e.call(X87_SQRT, &args![v]);
                        cpu.st_push(r.st0);
                    }
                    // 00e5a1d6: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a1da: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a1de: FMUL float ptr [ESI + -0xc]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esi.wrapping_sub(0xc))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a1e1: FADD float ptr [ESP + 0x1c]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a1e5: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a1e9: FLD float ptr [ESI + -0x8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_sub(0x8))));
                    // 00e5a1ec: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a1f0: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e5a1f4: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a1f8: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e5a1fa: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a1fc: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a1fe: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a201: JZ 0x00e5a209
                    if cpu.zf {
                        pc = 0x00e5a209;
                        continue 'run;
                    }
                    // 00e5a203: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a207: JMP 0x00e5a20b
                    pc = 0x00e5a20b;
                    continue 'run;
                }
                0x00e5a209 => {
                    // 00e5a209: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5a20b;
                    continue 'run;
                }
                0x00e5a20b => {
                    // 00e5a20b: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e5a20f: MOV ECX,dword ptr [ESP + 0x2c]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x2c));
                    // 00e5a213: FCOMP double ptr [0x01012060]
                    cpu.sw = u16::from(fcom_status(cpu.st(0), e.mem.f64(0x01012060))) << 8;
                    cpu.st_pop();
                    // 00e5a219: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a21b: FLDZ
                    cpu.st_push(0.0);
                    // 00e5a21d: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e5a220: JP 0x00e5a226
                    if cpu.pf {
                        pc = 0x00e5a226;
                        continue 'run;
                    }
                    // 00e5a222: FST float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    pc = 0x00e5a226;
                    continue 'run;
                }
                0x00e5a226 => {
                    // 00e5a226: FLD1
                    cpu.st_push(1.0);
                    // 00e5a228: FLD double ptr [0x010c9300]
                    cpu.st_push(e.mem.f64(0x010c9300));
                    // 00e5a22e: FXCH ST2
                    cpu.st_exchange(2);
                    pc = 0x00e5a230;
                    continue 'run;
                }
                0x00e5a230 => {
                    // 00e5a230: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e5a234: MOV EDX,dword ptr [EBP + 0x8]
                    cpu.edx = e.mem.u32(cpu.ebp.wrapping_add(0x8));
                    // 00e5a237: FMUL float ptr [ESP + 0x40]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x40))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a23b: ADD ECX,0x24
                    cpu.ecx = cpu.add32(cpu.ecx, 0x24);
                    // 00e5a23e: ADD EDI,0xa0
                    cpu.edi = cpu.add32(cpu.edi, 0xa0);
                    // 00e5a244: ADD ESI,0x60
                    cpu.esi = cpu.add32(cpu.esi, 0x60);
                    // 00e5a247: SUB dword ptr [ESP + 0x28],0x1
                    {
                        let at = cpu.esp.wrapping_add(0x28);
                        let v = cpu.sub32(e.mem.u32(at), 0x1);
                        e.mem.set_u32(at, v);
                    }
                    // 00e5a24c: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a250: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a254: MOV dword ptr [ESP + 0x2c],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x2c), cpu.ecx);
                    // 00e5a258: FST float ptr [EDI + -0x58]
                    e.mem
                        .set_u32(cpu.edi.wrapping_sub(0x58), x87_store32(cpu.st(0)));
                    // 00e5a25b: FST float ptr [EDI + -0x54]
                    e.mem
                        .set_u32(cpu.edi.wrapping_sub(0x54), x87_store32(cpu.st(0)));
                    // 00e5a25e: FADD ST0,ST3
                    {
                        let v = x87_add(cpu.st(0), cpu.st(3));
                        cpu.set_st(0, v);
                    }
                    // 00e5a260: FDIVR ST0,ST2
                    {
                        let v = x87_div(cpu.st(2), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e5a262: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a266: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a26a: FST float ptr [EDI + -0x50]
                    e.mem
                        .set_u32(cpu.edi.wrapping_sub(0x50), x87_store32(cpu.st(0)));
                    // 00e5a26d: FSTP float ptr [EDI + -0x4c]
                    e.mem
                        .set_u32(cpu.edi.wrapping_sub(0x4c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a270: FLD float ptr [EDI + -0x68]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edi.wrapping_sub(0x68))));
                    // 00e5a273: FMUL float ptr [EDI + -0x7c]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.edi.wrapping_sub(0x7c))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a276: FLD float ptr [EDX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.edx)));
                    // 00e5a278: FMUL float ptr [ESP + 0x24]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x24))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a27c: FADDP
                    {
                        let v = x87_add(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a27e: FSTP float ptr [EDI + -0x60]
                    e.mem
                        .set_u32(cpu.edi.wrapping_sub(0x60), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a281: JNZ 0x00e5a016
                    if !cpu.zf {
                        pc = 0x00e5a016;
                        continue 'run;
                    }
                    // 00e5a287: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a289: FLD float ptr [EBX + 0xac]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ebx.wrapping_add(0xac))));
                    // 00e5a28f: FADD float ptr [EBX + 0x4c]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(cpu.ebx.wrapping_add(0x4c))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a292: FCOMP double ptr [0x01012060]
                    cpu.sw = u16::from(fcom_status(cpu.st(0), e.mem.f64(0x01012060))) << 8;
                    cpu.st_pop();
                    // 00e5a298: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a29a: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a29d: JNZ 0x00e5a45c
                    if !cpu.zf {
                        pc = 0x00e5a45c;
                        continue 'run;
                    }
                    // 00e5a2a3: FLD float ptr [EBX + 0x44]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ebx.wrapping_add(0x44))));
                    // 00e5a2a6: MOV EAX,dword ptr [EBP + 0xc]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0xc));
                    // 00e5a2a9: FABS
                    {
                        let v = cpu.st(0).abs();
                        cpu.set_st(0, v);
                    }
                    // 00e5a2ab: MOV ECX,EDX
                    cpu.ecx = cpu.edx;
                    // 00e5a2ad: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a2b1: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a2b5: FLD float ptr [EBX + 0xa4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ebx.wrapping_add(0xa4))));
                    // 00e5a2bb: FABS
                    {
                        let v = cpu.st(0).abs();
                        cpu.set_st(0, v);
                    }
                    // 00e5a2bd: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a2c1: FADD float ptr [ESP + 0x18]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a2c5: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a2c9: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a2cd: FMUL float ptr [EAX + 0x4]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.eax.wrapping_add(0x4))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a2d0: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a2d4: FLD float ptr [ECX]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ecx)));
                    // 00e5a2d6: FMUL float ptr [ESP + 0x18]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a2da: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a2de: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a2e2: FST float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    // 00e5a2e6: FLD float ptr [EBX + 0x4c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ebx.wrapping_add(0x4c))));
                    // 00e5a2e9: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a2ed: FLD float ptr [ESP + 0x2c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x2c))));
                    // 00e5a2f1: FCOM ST3
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(3))) << 8;
                    // 00e5a2f3: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a2f5: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a2f8: JNZ 0x00e5a300
                    if !cpu.zf {
                        pc = 0x00e5a300;
                        continue 'run;
                    }
                    // 00e5a2fa: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a2fe: JMP 0x00e5a30a
                    pc = 0x00e5a30a;
                    continue 'run;
                }
                0x00e5a300 => {
                    // 00e5a300: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a302: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e5a304: FST float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    // 00e5a308: FXCH ST2
                    cpu.st_exchange(2);
                    pc = 0x00e5a30a;
                    continue 'run;
                }
                0x00e5a30a => {
                    // 00e5a30a: FLD float ptr [ESP + 0x20]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x20))));
                    // 00e5a30e: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e5a312: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e5a314: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a316: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a319: JP 0x00e5a323
                    if cpu.pf {
                        pc = 0x00e5a323;
                        continue 'run;
                    }
                    // 00e5a31b: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a31d: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a31f: FLD ST1
                    {
                        let v = cpu.st(1);
                        cpu.st_push(v);
                    }
                    // 00e5a321: JMP 0x00e5a325
                    pc = 0x00e5a325;
                    continue 'run;
                }
                0x00e5a323 => {
                    // 00e5a323: FDIVP
                    {
                        let v = x87_div(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5a325;
                    continue 'run;
                }
                0x00e5a325 => {
                    // 00e5a325: FSTP float ptr [ESP + 0x30]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x30), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a329: FLD float ptr [ESP + 0x30]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x30))));
                    // 00e5a32d: FLD double ptr [0x01032980]
                    cpu.st_push(e.mem.f64(0x01032980));
                    // 00e5a333: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e5a335: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a337: FLD float ptr [0x010141cc]
                    cpu.st_push(x87_load32(e.mem.u32(0x010141cc)));
                    // 00e5a33d: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a340: JNZ 0x00e5a34c
                    if !cpu.zf {
                        pc = 0x00e5a34c;
                        continue 'run;
                    }
                    // 00e5a342: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a344: FXCH
                    cpu.st_exchange(1);
                    // 00e5a346: FST float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    // 00e5a34a: JMP 0x00e5a354
                    pc = 0x00e5a354;
                    continue 'run;
                }
                0x00e5a34c => {
                    // 00e5a34c: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e5a34e: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a352: FXCH
                    cpu.st_exchange(1);
                    pc = 0x00e5a354;
                    continue 'run;
                }
                0x00e5a354 => {
                    // 00e5a354: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e5a358: FSTP float ptr [ESP + 0x30]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x30), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a35c: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e5a35e: FSTP float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a362: FLD float ptr [EBX + 0xac]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ebx.wrapping_add(0xac))));
                    // 00e5a368: FSTP float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a36c: FLD float ptr [ESP + 0x2c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x2c))));
                    // 00e5a370: FCOM ST4
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(4))) << 8;
                    // 00e5a372: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a374: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a377: JNZ 0x00e5a37d
                    if !cpu.zf {
                        pc = 0x00e5a37d;
                        continue 'run;
                    }
                    // 00e5a379: FSTP ST4
                    {
                        let v = cpu.st(0);
                        cpu.set_st(4, v);
                    }
                    cpu.st_pop();
                    // 00e5a37b: JMP 0x00e5a37f
                    pc = 0x00e5a37f;
                    continue 'run;
                }
                0x00e5a37d => {
                    // 00e5a37d: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5a37f;
                    continue 'run;
                }
                0x00e5a37f => {
                    // 00e5a37f: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e5a381: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a385: FLD float ptr [ESP + 0x20]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x20))));
                    // 00e5a389: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e5a38d: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e5a38f: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a391: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a394: JP 0x00e5a39c
                    if cpu.pf {
                        pc = 0x00e5a39c;
                        continue 'run;
                    }
                    // 00e5a396: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a398: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a39a: JMP 0x00e5a3a0
                    pc = 0x00e5a3a0;
                    continue 'run;
                }
                0x00e5a39c => {
                    // 00e5a39c: FSTP ST3
                    {
                        let v = cpu.st(0);
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    // 00e5a39e: FDIVRP ST2,ST0
                    {
                        let v = x87_div(cpu.st(0), cpu.st(2));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5a3a0;
                    continue 'run;
                }
                0x00e5a3a0 => {
                    // 00e5a3a0: FXCH
                    cpu.st_exchange(1);
                    // 00e5a3a2: FSTP float ptr [ESP + 0x34]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x34), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a3a6: FLD float ptr [ESP + 0x34]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x34))));
                    // 00e5a3aa: FCOM ST2
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(2))) << 8;
                    // 00e5a3ac: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a3ae: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a3b0: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e5a3b3: JP 0x00e5a3b9
                    if cpu.pf {
                        pc = 0x00e5a3b9;
                        continue 'run;
                    }
                    // 00e5a3b5: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a3b7: JMP 0x00e5a3bb
                    pc = 0x00e5a3bb;
                    continue 'run;
                }
                0x00e5a3b9 => {
                    // 00e5a3b9: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5a3bb;
                    continue 'run;
                }
                0x00e5a3bb => {
                    // 00e5a3bb: FSTP float ptr [ESP + 0x1c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a3bf: FLD float ptr [ESP + 0x1c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x1c))));
                    // 00e5a3c3: FSTP float ptr [ESP + 0x34]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x34), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a3c7: FLD float ptr [ESP + 0x30]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x30))));
                    // 00e5a3cb: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a3cd: FMUL float ptr [ESP + 0x50]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x50))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a3d1: FSTP float ptr [ESP + 0x50]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x50), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a3d5: FLD float ptr [ESP + 0x34]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x34))));
                    // 00e5a3d9: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a3db: FMUL float ptr [ESP + 0x44]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x44))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a3df: FSTP float ptr [ESP + 0x58]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x58), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a3e3: FMULP
                    {
                        let v = x87_mul(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a3e5: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a3e9: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a3ed: CALL 0x00ec6040
                    {
                        let v = cpu.st(0);
                        cpu.st_pop();
                        let r = e.call(X87_SQRT, &args![v]);
                        cpu.st_push(r.st0);
                    }
                    // 00e5a3f2: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a3f6: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a3fa: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a3fe: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a402: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a404: FMUL float ptr [ESP + 0x44]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x44))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a408: FSTP float ptr [ESP + 0x54]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x54), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a40c: FMUL float ptr [ESP + 0x5c]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x5c))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a410: FSTP float ptr [ESP + 0x5c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x5c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a414: FLD float ptr [ESP + 0x30]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x30))));
                    // 00e5a418: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a41a: FMUL float ptr [ESP + 0xe0]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0xe0))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a421: FSTP float ptr [ESP + 0xe0]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xe0), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a428: FDIVR float ptr [ESP + 0xe4]
                    {
                        let v = x87_div(x87_m32(e.mem.u32(cpu.esp.wrapping_add(0xe4))), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e5a42f: FSTP float ptr [ESP + 0xe4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xe4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a436: FLD float ptr [ESP + 0x34]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x34))));
                    // 00e5a43a: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a43c: FMUL float ptr [ESP + 0x180]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x180))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a443: FSTP float ptr [ESP + 0x180]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x180), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a44a: FDIVR float ptr [ESP + 0x184]
                    {
                        let v = x87_div(x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x184))), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e5a451: FSTP float ptr [ESP + 0x184]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x184), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a458: FLDZ
                    cpu.st_push(0.0);
                    // 00e5a45a: JMP 0x00e5a45e
                    pc = 0x00e5a45e;
                    continue 'run;
                }
                0x00e5a45c => {
                    // 00e5a45c: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5a45e;
                    continue 'run;
                }
                0x00e5a45e => {
                    // 00e5a45e: FLD1
                    cpu.st_push(1.0);
                    // 00e5a460: PUSH ECX
                    cpu.push(e, cpu.ecx);
                    // 00e5a461: FST float ptr [ESP + 0x34]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x34), x87_store32(cpu.st(0)));
                    // 00e5a465: LEA EDX,[ESP + 0x34]
                    cpu.edx = cpu.esp.wrapping_add(0x34);
                    // 00e5a469: FSTP float ptr [ESP + 0x40]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x40), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a46d: LEA EAX,[ESP + 0x54]
                    cpu.eax = cpu.esp.wrapping_add(0x54);
                    // 00e5a471: LEA ECX,[ESP + 0x53]
                    cpu.ecx = cpu.esp.wrapping_add(0x53);
                    // 00e5a475: FST float ptr [ESP + 0x38]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x38), x87_store32(cpu.st(0)));
                    // 00e5a479: FST float ptr [ESP + 0x3c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x3c), x87_store32(cpu.st(0)));
                    // 00e5a47d: FSTP float ptr [ESP]
                    e.mem.set_u32(cpu.esp, x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a480: PUSH EDX
                    cpu.push(e, cpu.edx);
                    // 00e5a481: PUSH EAX
                    cpu.push(e, cpu.eax);
                    // 00e5a482: PUSH ECX
                    cpu.push(e, cpu.ecx);
                    // 00e5a483: CALL 0x00e59340
                    e.call(
                        INVERT_2X2,
                        &args![
                            e.mem.u32(cpu.esp.wrapping_add(0)),
                            e.mem.u32(cpu.esp.wrapping_add(4)),
                            e.mem.u32(cpu.esp.wrapping_add(8)),
                            e.mem.u32(cpu.esp.wrapping_add(12))
                        ],
                    );
                    // 00e5a488: FLD float ptr [ESP + 0x188]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x188))));
                    // 00e5a48f: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a491: MOV ECX,dword ptr [EBP + 0x14]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0x14));
                    // 00e5a494: FMUL float ptr [ESP + 0x44]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x44))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a498: ADD ESP,0x10
                    cpu.esp = cpu.add32(cpu.esp, 0x10);
                    // 00e5a49b: FLD float ptr [ESP + 0xd8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xd8))));
                    // 00e5a4a2: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a4a4: FMUL float ptr [ESP + 0x30]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x30))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a4a8: FADDP ST2,ST0
                    {
                        let v = x87_add(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a4aa: FXCH
                    cpu.st_exchange(1);
                    // 00e5a4ac: FCHS
                    {
                        let v = -cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    // 00e5a4ae: FSTP float ptr [ESP + 0xf4]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0xf4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a4b5: FLD float ptr [ESP + 0x3c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x3c))));
                    // 00e5a4b9: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a4bb: FMUL float ptr [ESP + 0x38]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x38))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a4bf: FADDP
                    {
                        let v = x87_add(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a4c1: FCHS
                    {
                        let v = -cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    // 00e5a4c3: FSTP float ptr [ESP + 0x194]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x194), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a4ca: FLD float ptr [ESP + 0xf0]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xf0))));
                    // 00e5a4d1: FMUL float ptr [ESP + 0x100]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x100))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a4d8: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a4dc: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a4e0: FST float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    // 00e5a4e4: FLD float ptr [ESP + 0x104]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x104))));
                    // 00e5a4eb: FMUL float ptr [ESP + 0xf4]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0xf4))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a4f2: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a4f6: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a4fa: FST float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    // 00e5a4fe: FLD float ptr [ESP + 0x20]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x20))));
                    // 00e5a502: FLD float ptr [ESP + 0x2c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x2c))));
                    // 00e5a506: FMUL ST0
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e5a508: FLD ST1
                    {
                        let v = cpu.st(1);
                        cpu.st_push(v);
                    }
                    // 00e5a50a: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a50c: FADDP
                    {
                        let v = x87_add(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a50e: FSTP float ptr [ESP + 0x30]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x30), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a512: FLD float ptr [ESP + 0x190]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x190))));
                    // 00e5a519: FMUL float ptr [ESP + 0x1a0]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x1a0))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a520: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a524: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a528: FST float ptr [ESP + 0x2c]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x2c), x87_store32(cpu.st(0)));
                    // 00e5a52c: FLD float ptr [ESP + 0x194]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x194))));
                    // 00e5a533: FMUL float ptr [ESP + 0x1a4]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x1a4))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a53a: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a53e: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a542: FST float ptr [ESP + 0x20]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    // 00e5a546: FLD float ptr [ESP + 0x20]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x20))));
                    // 00e5a54a: FLD float ptr [ESP + 0x2c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x2c))));
                    // 00e5a54e: FMUL ST0
                    {
                        let v = x87_mul(cpu.st(0), cpu.st(0));
                        cpu.set_st(0, v);
                    }
                    // 00e5a550: FLD ST1
                    {
                        let v = cpu.st(1);
                        cpu.st_push(v);
                    }
                    // 00e5a552: FMULP ST2
                    {
                        let v = x87_mul(cpu.st(2), cpu.st(0));
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a554: FADDP
                    {
                        let v = x87_add(cpu.st(1), cpu.st(0));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a556: FSTP float ptr [ESP + 0x34]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x34), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a55a: FLD float ptr [ESP + 0xd0]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xd0))));
                    // 00e5a561: FABS
                    {
                        let v = cpu.st(0).abs();
                        cpu.set_st(0, v);
                    }
                    // 00e5a563: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a567: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a56b: FLD float ptr [EBX + 0xc0]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.ebx.wrapping_add(0xc0))));
                    // 00e5a571: FCOMPP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    cpu.st_pop();
                    // 00e5a573: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a575: FLDZ
                    cpu.st_push(0.0);
                    // 00e5a577: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a57a: JP 0x00e5a588
                    if cpu.pf {
                        pc = 0x00e5a588;
                        continue 'run;
                    }
                    // 00e5a57c: FST float ptr [ECX + 0x14]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    // 00e5a57f: FST float ptr [ECX + 0x18]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    // 00e5a582: FST float ptr [ECX + 0x38]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x38), x87_store32(cpu.st(0)));
                    // 00e5a585: FST float ptr [ECX + 0x3c]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x3c), x87_store32(cpu.st(0)));
                    pc = 0x00e5a588;
                    continue 'run;
                }
                0x00e5a588 => {
                    // 00e5a588: FLD1
                    cpu.st_push(1.0);
                    // 00e5a58a: FLD float ptr [ESP + 0x30]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x30))));
                    // 00e5a58e: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e5a590: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a592: FLD float ptr [ESP + 0x34]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x34))));
                    // 00e5a596: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a599: JZ 0x00e5a64a
                    if cpu.zf {
                        pc = 0x00e5a64a;
                        continue 'run;
                    }
                    // 00e5a59f: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a5a1: FCOM
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    // 00e5a5a3: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a5a5: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a5a8: JZ 0x00e5a5d2
                    if cpu.zf {
                        pc = 0x00e5a5d2;
                        continue 'run;
                    }
                    // 00e5a5aa: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a5ac: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a5ae: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e5a5b0: FSTP float ptr [ECX + 0x1c]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x1c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a5b3: FXCH ST3
                    cpu.st_exchange(3);
                    // 00e5a5b5: FSTP float ptr [ECX + 0x20]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x20), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a5b8: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e5a5ba: FSTP float ptr [ECX + 0x40]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x40), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a5bd: FXCH
                    cpu.st_exchange(1);
                    // 00e5a5bf: FSTP float ptr [ECX + 0x44]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x44), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a5c2: FST float ptr [ECX]
                    e.mem.set_u32(cpu.ecx, x87_store32(cpu.st(0)));
                    // 00e5a5c4: FST float ptr [ECX + 0x24]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    // 00e5a5c7: FST float ptr [ECX + 0x4]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    // 00e5a5ca: FST float ptr [ECX + 0x28]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    // 00e5a5cd: JMP 0x00e5a709
                    pc = 0x00e5a709;
                    continue 'run;
                }
                0x00e5a5d2 => {
                    // 00e5a5d2: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a5d4: FSTP ST5
                    {
                        let v = cpu.st(0);
                        cpu.set_st(5, v);
                    }
                    cpu.st_pop();
                    // 00e5a5d6: FSTP ST3
                    {
                        let v = cpu.st(0);
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    // 00e5a5d8: FSTP ST1
                    {
                        let v = cpu.st(0);
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a5da: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a5dc: FLD float ptr [ESP + 0x30]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x30))));
                    // 00e5a5e0: FCOM ST2
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(2))) << 8;
                    // 00e5a5e2: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a5e4: TEST AH,0x41
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x41);
                    // 00e5a5e7: JNZ 0x00e5a60d
                    if !cpu.zf {
                        pc = 0x00e5a60d;
                        continue 'run;
                    }
                    // 00e5a5e9: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e5a5eb: FXCH
                    cpu.st_exchange(1);
                    pc = 0x00e5a5ed;
                    continue 'run;
                }
                0x00e5a5ed => {
                    // 00e5a5ed: FLD ST2
                    {
                        let v = cpu.st(2);
                        cpu.st_push(v);
                    }
                    // 00e5a5ef: FLD1
                    cpu.st_push(1.0);
                    // 00e5a5f1: FDIVRP
                    {
                        let v = x87_div(cpu.st(0), cpu.st(1));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a5f3: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a5f7: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a5fb: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a5fd: FMUL float ptr [ECX + 0x14]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.ecx.wrapping_add(0x14))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a600: FSTP float ptr [ECX + 0x14]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x14), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a603: FMUL float ptr [ECX + 0x18]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.ecx.wrapping_add(0x18))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a606: FSTP float ptr [ECX + 0x18]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a609: FXCH
                    cpu.st_exchange(1);
                    // 00e5a60b: FXCH ST2
                    cpu.st_exchange(2);
                    pc = 0x00e5a60d;
                    continue 'run;
                }
                0x00e5a60d => {
                    // 00e5a60d: FXCH ST2
                    cpu.st_exchange(2);
                    // 00e5a60f: FCOMP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    // 00e5a611: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a613: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e5a616: JP 0x00e5a634
                    if cpu.pf {
                        pc = 0x00e5a634;
                        continue 'run;
                    }
                    // 00e5a618: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a61a: FLD1
                    cpu.st_push(1.0);
                    // 00e5a61c: FDIVRP
                    {
                        let v = x87_div(cpu.st(0), cpu.st(1));
                        cpu.set_st(1, v);
                    }
                    cpu.st_pop();
                    // 00e5a61e: FSTP float ptr [ESP + 0x18]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x18), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a622: FLD float ptr [ESP + 0x18]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18))));
                    // 00e5a626: FLD ST0
                    {
                        let v = cpu.st(0);
                        cpu.st_push(v);
                    }
                    // 00e5a628: FMUL float ptr [ECX + 0x38]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.ecx.wrapping_add(0x38))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a62b: FSTP float ptr [ECX + 0x38]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x38), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a62e: FMUL float ptr [ECX + 0x3c]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.ecx.wrapping_add(0x3c))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a631: FSTP float ptr [ECX + 0x3c]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x3c), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    pc = 0x00e5a634;
                    continue 'run;
                }
                0x00e5a634 => {
                    // 00e5a634: FCOMPP
                    cpu.sw = u16::from(fcom_status(cpu.st(0), cpu.st(1))) << 8;
                    cpu.st_pop();
                    cpu.st_pop();
                    // 00e5a636: FNSTSW AX
                    cpu.eax = (cpu.eax & 0xffff_0000) | u32::from(cpu.sw);
                    // 00e5a638: TEST AH,0x5
                    cpu.test8((cpu.eax >> 8) & 0xff, 0x5);
                    // 00e5a63b: JP 0x00e5a656
                    if cpu.pf {
                        pc = 0x00e5a656;
                        continue 'run;
                    }
                    // 00e5a63d: MOV ECX,0x1
                    cpu.ecx = 0x1;
                    // 00e5a642: XOR EAX,EAX
                    cpu.eax = cpu.xor32(cpu.eax, cpu.eax);
                    // 00e5a644: MOV dword ptr [ESP + 0x20],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x20), cpu.ecx);
                    // 00e5a648: JMP 0x00e5a667
                    pc = 0x00e5a667;
                    continue 'run;
                }
                0x00e5a64a => {
                    // 00e5a64a: FSTP ST3
                    {
                        let v = cpu.st(0);
                        cpu.set_st(3, v);
                    }
                    cpu.st_pop();
                    // 00e5a64c: FSTP ST6
                    {
                        let v = cpu.st(0);
                        cpu.set_st(6, v);
                    }
                    cpu.st_pop();
                    // 00e5a64e: FSTP ST4
                    {
                        let v = cpu.st(0);
                        cpu.set_st(4, v);
                    }
                    cpu.st_pop();
                    // 00e5a650: FSTP ST2
                    {
                        let v = cpu.st(0);
                        cpu.set_st(2, v);
                    }
                    cpu.st_pop();
                    // 00e5a652: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    // 00e5a654: JMP 0x00e5a5ed
                    pc = 0x00e5a5ed;
                    continue 'run;
                }
                0x00e5a656 => {
                    // 00e5a656: MOV dword ptr [ESP + 0x20],0x0
                    e.mem.set_u32(cpu.esp.wrapping_add(0x20), 0x0);
                    // 00e5a65e: MOV ECX,dword ptr [ESP + 0x20]
                    cpu.ecx = e.mem.u32(cpu.esp.wrapping_add(0x20));
                    // 00e5a662: MOV EAX,0x1
                    cpu.eax = 0x1;
                    pc = 0x00e5a667;
                    continue 'run;
                }
                0x00e5a667 => {
                    // 00e5a667: LEA EDX,[ECX + ECX*0x4]
                    cpu.edx = cpu.ecx.wrapping_add(cpu.ecx.wrapping_mul(4));
                    // 00e5a66a: SHL EDX,0x5
                    cpu.edx = cpu.shl32(cpu.edx, 0x5);
                    // 00e5a66d: LEA ECX,[ESP + EDX*0x1 + 0xd8]
                    cpu.ecx = cpu.esp.wrapping_add(cpu.edx).wrapping_add(0xd8);
                    // 00e5a674: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(cpu.esp.wrapping_add(0x18), cpu.ecx);
                    // 00e5a678: MOV dword ptr [ESP + 0x28],0x2
                    e.mem.set_u32(cpu.esp.wrapping_add(0x28), 0x2);
                    pc = 0x00e5a680;
                    continue 'run;
                }
                0x00e5a680 => {
                    // 00e5a680: MOV ECX,dword ptr [EBP + 0x14]
                    cpu.ecx = e.mem.u32(cpu.ebp.wrapping_add(0x14));
                    // 00e5a683: LEA ESI,[EAX + EAX*0x4]
                    cpu.esi = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(4));
                    // 00e5a686: LEA EDX,[EAX + EAX*0x8]
                    cpu.edx = cpu.eax.wrapping_add(cpu.eax.wrapping_mul(8));
                    // 00e5a689: IMUL EAX,EAX,0x64
                    cpu.eax = cpu.imul32(cpu.eax, 0x64);
                    // 00e5a68c: SHL ESI,0x5
                    cpu.esi = cpu.shl32(cpu.esi, 0x5);
                    // 00e5a68f: LEA ESI,[ESP + ESI*0x1 + 0x70]
                    cpu.esi = cpu.esp.wrapping_add(cpu.esi).wrapping_add(0x70);
                    // 00e5a693: LEA EDI,[ECX + EDX*0x4]
                    cpu.edi = cpu.ecx.wrapping_add(cpu.edx.wrapping_mul(4));
                    // 00e5a696: MOV EDX,dword ptr [EBP + 0xc]
                    cpu.edx = e.mem.u32(cpu.ebp.wrapping_add(0xc));
                    // 00e5a699: FLD float ptr [ESI + 0x68]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x68))));
                    // 00e5a69c: LEA EDX,[EAX + EDX*0x1 + 0x8]
                    cpu.edx = cpu.eax.wrapping_add(cpu.edx).wrapping_add(0x8);
                    // 00e5a6a0: FCHS
                    {
                        let v = -cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    // 00e5a6a2: FMUL float ptr [ESI + 0x70]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esi.wrapping_add(0x70))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a6a5: FSTP float ptr [ESI + 0x84]
                    e.mem
                        .set_u32(cpu.esi.wrapping_add(0x84), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a6ab: CALL 0x00e59700
                    e.call(SOLVE_STEP, &args![cpu.edx, cpu.esi, cpu.edi]);
                    // 00e5a6b0: SUB dword ptr [ESP + 0x28],0x1
                    {
                        let at = cpu.esp.wrapping_add(0x28);
                        let v = cpu.sub32(e.mem.u32(at), 0x1);
                        e.mem.set_u32(at, v);
                    }
                    // 00e5a6b5: FLD float ptr [ESI + 0x84]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esi.wrapping_add(0x84))));
                    // 00e5a6bb: FMUL float ptr [ESP + 0x54]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x54))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a6bf: MOV EAX,dword ptr [ESP + 0x18]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x18));
                    // 00e5a6c3: FADD float ptr [EAX]
                    {
                        let v = x87_add(cpu.st(0), x87_m32(e.mem.u32(cpu.eax)));
                        cpu.set_st(0, v);
                    }
                    // 00e5a6c5: FSTP float ptr [EAX]
                    e.mem.set_u32(cpu.eax, x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a6c7: MOV EAX,dword ptr [ESP + 0x20]
                    cpu.eax = e.mem.u32(cpu.esp.wrapping_add(0x20));
                    // 00e5a6cb: JNZ 0x00e5a680
                    if !cpu.zf {
                        pc = 0x00e5a680;
                        continue 'run;
                    }
                    // 00e5a6cd: FLD float ptr [ESP + 0xec]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xec))));
                    // 00e5a6d4: MOV EAX,dword ptr [EBP + 0x14]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0x14));
                    // 00e5a6d7: FMUL float ptr [ESP + 0x108]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x108))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a6de: MOV ECX,EAX
                    cpu.ecx = cpu.eax;
                    // 00e5a6e0: FSTP float ptr [EAX]
                    e.mem.set_u32(cpu.eax, x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a6e2: FLD float ptr [ESP + 0x18c]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x18c))));
                    // 00e5a6e9: FMUL float ptr [ESP + 0x1a8]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x1a8))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a6f0: FSTP float ptr [EAX + 0x24]
                    e.mem
                        .set_u32(cpu.eax.wrapping_add(0x24), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a6f3: FLD float ptr [ESP + 0xd8]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xd8))));
                    // 00e5a6fa: FSTP float ptr [EAX + 0x4]
                    e.mem
                        .set_u32(cpu.eax.wrapping_add(0x4), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a6fd: FLD float ptr [ESP + 0x178]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x178))));
                    // 00e5a704: FSTP float ptr [EAX + 0x28]
                    e.mem
                        .set_u32(cpu.eax.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a707: FLDZ
                    cpu.st_push(0.0);
                    pc = 0x00e5a709;
                    continue 'run;
                }
                0x00e5a709 => {
                    // 00e5a709: FLD float ptr [ESP + 0xf0]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0xf0))));
                    // 00e5a710: MOV EAX,dword ptr [EBP + 0x8]
                    cpu.eax = e.mem.u32(cpu.ebp.wrapping_add(0x8));
                    // 00e5a713: FSTP float ptr [ECX + 0x10]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x10), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a716: LEA EDX,[EBX + 0x30]
                    cpu.edx = cpu.ebx.wrapping_add(0x30);
                    // 00e5a719: FLD float ptr [ESP + 0x190]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.esp.wrapping_add(0x190))));
                    // 00e5a720: MOV ESI,0x2
                    cpu.esi = 0x2;
                    // 00e5a725: FSTP float ptr [ECX + 0x34]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x34), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a728: FLD float ptr [EAX + 0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax.wrapping_add(0x4))));
                    // 00e5a72b: FMUL float ptr [ESP + 0xf4]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0xf4))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a732: FSTP float ptr [ECX + 0xc]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0xc), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a735: FLD float ptr [EAX + 0x4]
                    cpu.st_push(x87_load32(e.mem.u32(cpu.eax.wrapping_add(0x4))));
                    // 00e5a738: FMUL float ptr [ESP + 0x194]
                    {
                        let v = x87_mul(cpu.st(0), x87_m32(e.mem.u32(cpu.esp.wrapping_add(0x194))));
                        cpu.set_st(0, v);
                    }
                    // 00e5a73f: FSTP float ptr [ECX + 0x30]
                    e.mem
                        .set_u32(cpu.ecx.wrapping_add(0x30), x87_store32(cpu.st(0)));
                    cpu.st_pop();
                    // 00e5a742: LEA ECX,[ESP + 0xa0]
                    cpu.ecx = cpu.esp.wrapping_add(0xa0);
                    pc = 0x00e5a749;
                    continue 'run;
                }
                0x00e5a749 => {
                    // 00e5a749: MOVSS XMM3,dword ptr [ECX + 0x54]
                    cpu.x[3] = scalar_vec(e, cpu.ecx.wrapping_add(0x54));
                    // 00e5a74e: FST float ptr [ESP + 0x28]
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), x87_store32(cpu.st(0)));
                    // 00e5a752: MOV EAX,dword ptr [EDX]
                    cpu.eax = e.mem.u32(cpu.edx);
                    // 00e5a754: MOVAPS XMM1,xmmword ptr [EBX + 0x100]
                    cpu.x[1] = load_vec(e, cpu.ebx.wrapping_add(0x100));
                    // 00e5a75b: MOVAPS XMM5,xmmword ptr [EBX + 0xe0]
                    cpu.x[5] = load_vec(e, cpu.ebx.wrapping_add(0xe0));
                    // 00e5a762: MOVAPS XMM2,XMM3
                    cpu.x[2] = cpu.x[3];
                    // 00e5a765: SHUFPS XMM2,XMM3,0x0
                    cpu.x[2] = shufps(cpu.x[2], cpu.x[3], 0);
                    // 00e5a769: MOVAPS XMM0,XMM2
                    cpu.x[0] = cpu.x[2];
                    // 00e5a76c: MULPS XMM0,XMM1
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[1]);
                    // 00e5a76f: MOVAPS XMM1,xmmword ptr [EAX + 0x30]
                    cpu.x[1] = load_vec(e, cpu.eax.wrapping_add(0x30));
                    // 00e5a773: MULPS XMM1,XMM2
                    cpu.x[1] = mulps(cpu.x[1], cpu.x[2]);
                    // 00e5a776: MOVAPS XMM2,xmmword ptr [ECX]
                    cpu.x[2] = load_vec(e, cpu.ecx);
                    // 00e5a779: MOVAPS XMM4,XMM0
                    cpu.x[4] = cpu.x[0];
                    // 00e5a77c: SHUFPS XMM4,XMM0,0xff
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[0], 255);
                    // 00e5a780: MULPS XMM4,XMM2
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[2]);
                    // 00e5a783: ADDPS XMM4,XMM5
                    cpu.x[4] = addps(cpu.x[4], cpu.x[5]);
                    // 00e5a786: MOVAPS xmmword ptr [EBX + 0xe0],XMM4
                    store_vec(e, cpu.ebx.wrapping_add(0xe0), cpu.x[4]);
                    // 00e5a78d: MOVAPS XMM4,XMM1
                    cpu.x[4] = cpu.x[1];
                    // 00e5a790: SHUFPS XMM4,XMM1,0xff
                    cpu.x[4] = shufps(cpu.x[4], cpu.x[1], 255);
                    // 00e5a794: MULPS XMM4,XMM2
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[2]);
                    // 00e5a797: MOVAPS XMM2,xmmword ptr [EAX + 0x10]
                    cpu.x[2] = load_vec(e, cpu.eax.wrapping_add(0x10));
                    // 00e5a79b: SUBPS XMM2,XMM4
                    cpu.x[2] = subps(cpu.x[2], cpu.x[4]);
                    // 00e5a79e: MOVAPS XMM4,xmmword ptr [ECX + 0x10]
                    cpu.x[4] = load_vec(e, cpu.ecx.wrapping_add(0x10));
                    // 00e5a7a2: MOVAPS xmmword ptr [EAX + 0x10],XMM2
                    store_vec(e, cpu.eax.wrapping_add(0x10), cpu.x[2]);
                    // 00e5a7a6: MOVSS XMM2,dword ptr [ESP + 0x28]
                    cpu.x[2] = scalar_vec(e, cpu.esp.wrapping_add(0x28));
                    // 00e5a7ac: MULPS XMM4,XMM0
                    cpu.x[4] = mulps(cpu.x[4], cpu.x[0]);
                    // 00e5a7af: MOVAPS XMM0,xmmword ptr [EBX + 0xf0]
                    cpu.x[0] = load_vec(e, cpu.ebx.wrapping_add(0xf0));
                    // 00e5a7b6: ADDPS XMM4,XMM0
                    cpu.x[4] = addps(cpu.x[4], cpu.x[0]);
                    // 00e5a7b9: MOVAPS XMM0,xmmword ptr [ECX + 0x20]
                    cpu.x[0] = load_vec(e, cpu.ecx.wrapping_add(0x20));
                    // 00e5a7bd: MOVAPS xmmword ptr [EBX + 0xf0],XMM4
                    store_vec(e, cpu.ebx.wrapping_add(0xf0), cpu.x[4]);
                    // 00e5a7c4: MULPS XMM0,XMM1
                    cpu.x[0] = mulps(cpu.x[0], cpu.x[1]);
                    // 00e5a7c7: MOVAPS XMM1,xmmword ptr [EAX + 0x20]
                    cpu.x[1] = load_vec(e, cpu.eax.wrapping_add(0x20));
                    // 00e5a7cb: ADD ECX,0xa0
                    cpu.ecx = cpu.add32(cpu.ecx, 0xa0);
                    // 00e5a7d1: ADD EDX,0x60
                    cpu.edx = cpu.add32(cpu.edx, 0x60);
                    // 00e5a7d4: SUB ESI,0x1
                    cpu.esi = cpu.sub32(cpu.esi, 0x1);
                    // 00e5a7d7: ADDPS XMM0,XMM1
                    cpu.x[0] = addps(cpu.x[0], cpu.x[1]);
                    // 00e5a7da: ADDSS XMM2,XMM3
                    cpu.x[2] = addss(cpu.x[2], cpu.x[3]);
                    // 00e5a7de: MOVAPS xmmword ptr [EAX + 0x20],XMM0
                    store_vec(e, cpu.eax.wrapping_add(0x20), cpu.x[0]);
                    // 00e5a7e2: MOVSS dword ptr [ESP + 0x28],XMM2
                    e.mem
                        .set_u32(cpu.esp.wrapping_add(0x28), cpu.x[2][0].to_bits());
                    // 00e5a7e8: JNZ 0x00e5a749
                    if !cpu.zf {
                        pc = 0x00e5a749;
                        continue 'run;
                    }
                    // 00e5a7ee: FSTP ST0
                    {
                        let v = cpu.st(0);
                        cpu.set_st(0, v);
                    }
                    cpu.st_pop();
                    pc = 0x00e5a7f0;
                    continue 'run;
                }
                0x00e5a7f0 => {
                    // 00e5a7f0: POP EDI
                    cpu.edi = cpu.pop(e);
                    // 00e5a7f1: POP ESI
                    cpu.esi = cpu.pop(e);
                    // 00e5a7f2: POP EBX
                    cpu.ebx = cpu.pop(e);
                    // 00e5a7f3: MOV ESP,EBP
                    cpu.esp = cpu.ebp;
                    // 00e5a7f5: POP EBP
                    cpu.ebp = cpu.pop(e);
                    // 00e5a7f6: RET
                    break 'run;
                }
                other => panic!("jump to {other:#x} outside the recovered function"),
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e5_9700, fn_00e59700(u32, u32, u32)),
        entry!(0x00e5_9a40, fn_00e59a40(u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The words of the executable's data the two functions read, at their addresses.
    const EXE_WORDS: [(u32, &[u32]); 10] = [
        // The double 0.5.
        (0x0101_1588, &[0, 0x3fe0_0000]),
        // The doubles 0.0 and 1.0.
        (0x0101_2060, &[0, 0]),
        (0x0101_2070, &[0, 0x3ff0_0000]),
        // The float `1e-4` and the double `1e-4`.
        (0x0101_41cc, &[0x38d1_b717]),
        (0x0103_2980, &[0xe000_0000, 0x3f1a_36e2]),
        // The double 0.25.
        (0x0102_90b0, &[0, 0x3fd0_0000]),
        // The double `2^-23`.
        (0x010c_9300, &[0, 0x3e80_0000]),
        // `1.1920929e-07`, and the floats 1.1 and -1.1.
        (0x0104_da4c, &[0x3400_0000]),
        (0x0105_6528, &[0x3f8c_cccd]),
        (0x010e_c9e8, &[0xbf8c_cccd]),
    ];

    /// Doubles the engine needs: the CRT square root (leading `double`, root in `ST0`).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for (address, words) in EXE_WORDS {
            for (index, word) in words.iter().enumerate() {
                e.mem.set_u32(address + 4 * index as u32, *word);
            }
        }
        e.register_double(X87_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        e
    }

    fn float(value: f64) -> f64 {
        f64::from(value as f32)
    }

    /// The state record of `00e59700` as the floats at `+0x60 ..= +0x98`, the table of 18
    /// floats and the output record.
    struct Inputs {
        table: [f32; 18],
        state: [f32; 40],
        out: [f32; 9],
    }

    /// The function as the exe's code reads, on `f64` with a rounding to `f32` at each store.
    /// Returns the state and output it leaves.
    fn reference(inputs: &Inputs) -> ([f32; 40], [f32; 9]) {
        let t = |index: usize| f64::from(inputs.table[index]);
        let mut s = inputs.state;
        let mut out = inputs.out;
        let state = |s: &[f32; 40], offset: usize| f64::from(s[offset / 4]);
        let f9 = float(state(&s, 0x94) * state(&s, 0x84));
        let f10 = float(state(&s, 0x90) * state(&s, 0x80));
        out[7] = f9 as f32;
        out[8] = f10 as f32;
        if f9 * f9 + f10 * f10 < 1.0 {
            s[0x98 / 4] = 0.0;
            return (s, out);
        }
        let mut a = float(t(17) * f9);
        let mut b = float(t(0) * f10);
        let mut l = float(a * a + b * b);
        let (mut a_prev, mut b_prev, mut l_prev) = (0.0, 0.0, 0.0);
        let mut index = 0;
        while l <= 1.0 {
            index += 1;
            if index >= 16 {
                break;
            }
            (a_prev, b_prev, l_prev) = (a, b, l);
            a = float(t(16) * a_prev);
            b = float(t(index) * f10);
            l = float(a * a + b * b);
        }
        let x = float(float(l.sqrt()) - 1.0);
        let y = float(float(l_prev.sqrt()) - 1.0);
        let d = float(x - y);
        let magnitude = float(d.abs());
        let epsilon = f64::from(f32::from_bits(0x3400_0000));
        let d = if epsilon > magnitude { epsilon } else { d };
        let r = float(1.0 / d);
        let p1 = float(-y * r);
        let p2 = float(x * r);
        let clamp = |v: f64| {
            let low = f64::from(f32::from_bits(0xbf8c_cccd));
            let high = f64::from(f32::from_bits(0x3f8c_cccd));
            if v < low {
                low
            } else if high < v {
                high
            } else {
                v
            }
        };
        let z = clamp(float(p2 * a_prev + p1 * a));
        let w = clamp(float(p1 * b + p2 * b_prev));
        let zc = float(state(&s, 0x8c) * z);
        let wc = float(state(&s, 0x88) * w);
        let leftover = float(state(&s, 0x84) - zc);
        let old_80 = state(&s, 0x80);
        s[0x84 / 4] = zc as f32;
        s[0x98 / 4] = float(old_80 - wc) as f32;
        s[0x80 / 4] = wc as f32;
        let sum = float(state(&s, 0x68) * state(&s, 0x68) + state(&s, 0x60) * state(&s, 0x60));
        let sum = if sum.abs() < epsilon { epsilon } else { sum };
        let v1 = float(state(&s, 0x7c) * state(&s, 0x98) * state(&s, 0x88));
        let v2 = float(state(&s, 0x74) * leftover * state(&s, 0x8c));
        let q = float(v2 * v2 + v1 * v1);
        let inverse = float(1.0 / sum);
        out[2] = float(float(q * inverse).sqrt()) as f32;
        (s, out)
    }

    fn run(inputs: &Inputs, fill: u32) -> (Vec<f32>, Vec<f32>) {
        let mut e = engine();
        let table = e.mem.alloc(0x50);
        let state = e.mem.alloc(0xb0);
        let out = e.mem.alloc(0x30);
        for (index, value) in inputs.table.iter().enumerate() {
            e.mem.set_f32(table + 4 * index as u32, *value);
        }
        for (index, value) in inputs.state.iter().enumerate() {
            e.mem.set_f32(state + 4 * index as u32, *value);
        }
        for (index, value) in inputs.out.iter().enumerate() {
            e.mem.set_f32(out + 4 * index as u32, *value);
        }
        run_00e59700(&mut e, [table, state, out], fill);
        (
            (0..40).map(|index| e.mem.f32(state + 4 * index)).collect(),
            (0..9).map(|index| e.mem.f32(out + 4 * index)).collect(),
        )
    }

    fn close(actual: &[f32], expected: &[f32]) {
        assert_eq!(actual.len(), expected.len());
        for (index, (a, b)) in actual.iter().zip(expected).enumerate() {
            let allowed = 1e-4 * b.abs().max(a.abs()) + 1e-6;
            assert!((a - b).abs() <= allowed, "word {index}: {a} against {b}");
        }
    }

    /// Inputs where the step is large enough: `f9 = a * b`, `f10 = c * d`.
    fn inputs(f9: f32, f10: f32, table_scale: f32) -> Inputs {
        let mut table = [0.0f32; 18];
        for (index, value) in table.iter_mut().enumerate() {
            *value = 0.3 + 0.07 * index as f32 * table_scale;
        }
        table[16] = 1.5 * table_scale;
        table[17] = 0.75 * table_scale;
        let mut state = [0.0f32; 40];
        state[0x94 / 4] = f9;
        state[0x84 / 4] = 1.0;
        state[0x90 / 4] = f10;
        state[0x80 / 4] = 1.0;
        state[0x8c / 4] = 2.0;
        state[0x88 / 4] = 0.5;
        state[0x7c / 4] = 1.0;
        state[0x74 / 4] = 1.0;
        state[0x60 / 4] = 0.25;
        state[0x68 / 4] = 0.5;
        Inputs {
            table,
            state,
            out: [-1.0; 9],
        }
    }

    #[test]
    fn a_small_step_clears_the_result_and_stores_the_products() {
        let mut inputs = inputs(0.5, 0.25, 1.0);
        inputs.state[0x98 / 4] = 7.0;
        let (state, out) = run(&inputs, 0);
        assert_eq!(state[0x98 / 4], 0.0);
        // The products are stored whatever follows; the rest of the state is not changed.
        assert_eq!(out[7], 0.5);
        assert_eq!(out[8], 0.25);
        assert_eq!(out[2], -1.0);
        assert_eq!(state[0x84 / 4], 1.0);
        assert_eq!(state[0x80 / 4], 1.0);
    }

    #[test]
    fn a_step_of_exactly_one_is_not_small() {
        // `f9 = 1`, `f10 = 0`: the sum of squares 1 is not below the limit.
        let inputs = inputs(1.0, 0.0, 1.0);
        let (state, out) = run(&inputs, 0);
        let (expected_state, expected_out) = reference(&inputs);
        assert_ne!(out[2], -1.0);
        close(&state, &expected_state);
        close(&out, &expected_out);
    }

    #[test]
    fn a_long_step_is_found_at_once() {
        // `f9 = 2`, `f10 = 1`, the first table entries 1: the squared length 5 is above 1 at
        // once, so `Y = sqrt(0) - 1 = -1` and `X = sqrt(5) - 1`.
        let mut inputs = inputs(2.0, 1.0, 1.0);
        inputs.table[17] = 1.0;
        inputs.table[0] = 1.0;
        let (state, out) = run(&inputs, 0);
        let (expected_state, expected_out) = reference(&inputs);
        close(&state, &expected_state);
        close(&out, &expected_out);
        // By hand: `D = sqrt(5)`, `P1 = 1 / D`, `Z = P1 * 2`, the new `state[+0x84]` is `2 * Z`.
        let d = 5.0f64.sqrt();
        let z = 2.0 / d;
        assert!((f64::from(state[0x84 / 4]) - 2.0 * z).abs() < 1e-5);
        // And `state[+0x80] = 0.5 * W`, `W = 1 / D`.
        assert!((f64::from(state[0x80 / 4]) - 0.5 / d).abs() < 1e-5);
    }

    #[test]
    fn the_table_is_walked_until_the_length_passes_one() {
        for table_scale in [0.5, 1.0, 1.5, 3.0] {
            for (f9, f10) in [(1.0, 0.5), (1.2, -0.7), (0.9, 0.9), (-1.5, 0.2), (0.0, 1.0)] {
                let inputs = inputs(f9, f10, table_scale);
                let (state, out) = run(&inputs, 0);
                let (expected_state, expected_out) = reference(&inputs);
                close(&state, &expected_state);
                close(&out, &expected_out);
            }
        }
    }

    #[test]
    fn a_walk_that_never_passes_one_ends_at_the_sixteenth_entry() {
        // The table shrinks the step each time: the walk stops by its count.
        let mut inputs = inputs(1.0, 0.0, 1.0);
        inputs.table[17] = 0.5;
        inputs.table[16] = 0.5;
        let (state, out) = run(&inputs, 0);
        let (expected_state, expected_out) = reference(&inputs);
        close(&state, &expected_state);
        close(&out, &expected_out);
    }

    #[test]
    fn many_inputs_agree_with_the_hand_written_steps() {
        let mut seed = 0x2468_ace1u32;
        let mut next = move || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            f32::from((seed >> 16) as u16 % 4000) / 1000.0 - 1.5
        };
        for _ in 0..300 {
            let mut inputs = inputs(0.0, 0.0, 1.0);
            for value in inputs.table.iter_mut() {
                *value = next();
            }
            for offset in [
                0x60usize, 0x68, 0x74, 0x7c, 0x80, 0x84, 0x88, 0x8c, 0x90, 0x94,
            ] {
                inputs.state[offset / 4] = next();
            }
            let (state, out) = run(&inputs, 0);
            let (expected_state, expected_out) = reference(&inputs);
            close(&state, &expected_state);
            close(&out, &expected_out);
        }
    }

    #[test]
    fn the_result_does_not_depend_on_the_frame_fill() {
        let inputs = inputs(1.2, -0.7, 1.0);
        let first = run(&inputs, 0);
        assert_eq!(run(&inputs, 0xdead_beef), first);
        assert_eq!(run(&inputs, 0x7fc0_0000), first);
    }

    #[test]
    fn registered_entry_runs_with_register_inputs() {
        let mut e = engine();
        let table = e.mem.alloc(0x50);
        let state = e.mem.alloc(0xb0);
        let out = e.mem.alloc(0x30);
        e.mem.set_f32(state + 0x94, 0.5);
        e.mem.set_f32(state + 0x84, 0.5);
        e.mem.set_f32(state + 0x98, 3.0);
        e.call(0x00e5_9700, &args![table, state, out]);
        assert_eq!(e.mem.f32(state + 0x98), 0.0);
        assert_eq!(e.mem.f32(out + 0x1c), 0.25);
        assert_eq!(e.mem.f32(out + 0x20), 0.0);
    }

    /// What the callee doubles of `00e59a40` saw.
    #[derive(Default)]
    struct Seen {
        flag_calls: Vec<u32>,
        inversions: Vec<Vec<u32>>,
        solves: Vec<[u32; 3]>,
    }

    /// An engine for `00e59a40`: `set_flag` says whether `00df22f0` sets the byte.
    fn flying_engine(set_flag: bool, doubles: bool) -> (Engine, Rc<RefCell<Seen>>) {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Seen::default()));
        let log = Rc::clone(&seen);
        e.register_double(PROCESS_FLYING_COLORS, move |e, a| {
            log.borrow_mut().flag_calls.push(a[0]);
            if set_flag {
                e.mem.set_u8(a[0], 1);
            }
            Ret::default()
        });
        if !doubles {
            return (e, seen);
        }
        // The 2 by 2 inverse: reported fine, the identity.
        let log = Rc::clone(&seen);
        e.register_double(INVERT_2X2, move |e, a| {
            log.borrow_mut().inversions.push(a.to_vec());
            e.mem.set_u8(a[0], 1);
            for (index, value) in [1.0f32, 0.0, 0.0, 1.0].into_iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * index as u32, value);
            }
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(SOLVE_STEP, move |_, a| {
            log.borrow_mut().solves.push([a[0], a[1], a[2]]);
            Ret::default()
        });
        (e, seen)
    }

    /// The records `00e59a40` works on, with values from a small generator: the factor (two
    /// floats), the settings, the state with its two body pointers and the output.
    struct Records {
        factor: u32,
        settings: u32,
        state: u32,
        out: u32,
        bodies: [u32; 2],
    }

    fn records(e: &mut Engine, seed: u32) -> Records {
        let mut seed = seed;
        let mut next = move || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            f32::from((seed >> 16) as u16 % 2000) / 1000.0 + 0.1
        };
        let factor = e.mem.alloc(0x10);
        let settings = e.mem.alloc(0xe0);
        let state = e.mem.alloc(0x200);
        let out = e.mem.alloc(0x80);
        let bodies = [e.mem.alloc(0x80), e.mem.alloc(0x80)];
        for address in [factor, settings, state, bodies[0], bodies[1]] {
            let size = e.mem.block_size(address).unwrap_or(0x10);
            for word in 0..size / 4 {
                e.mem.set_f32(address + 4 * word, next());
            }
        }
        // Each of the two entries (0x60 bytes apart) has two body pointers, at `+0x30` and `+0x34`.
        for entry in [state, state + 0x60] {
            e.mem.set_u32(entry + 0x30, bodies[0]);
            e.mem.set_u32(entry + 0x34, bodies[1]);
        }
        Records {
            factor,
            settings,
            state,
            out,
            bodies,
        }
    }

    #[test]
    fn without_the_flag_nothing_else_runs() {
        let (mut e, seen) = flying_engine(false, true);
        let r = records(&mut e, 1);
        e.mem.set_f32(r.out, 5.0);
        e.call(0x00e5_9a40, &args![r.factor, r.settings, r.state, r.out]);
        let seen = seen.borrow();
        // The byte is tested, `00df22f0` is given its address once, the byte is tested again.
        assert_eq!(seen.flag_calls, vec![FLYING_COLORS_FLAG]);
        assert!(seen.inversions.is_empty() && seen.solves.is_empty());
        assert_eq!(e.mem.f32(r.out), 5.0);
    }

    #[test]
    fn a_byte_already_set_is_not_asked_for_again() {
        let (mut e, seen) = flying_engine(false, true);
        e.mem.set_u8(FLYING_COLORS_FLAG, 1);
        let r = records(&mut e, 2);
        e.call(0x00e5_9a40, &args![r.factor, r.settings, r.state, r.out]);
        let seen = seen.borrow();
        assert!(seen.flag_calls.is_empty());
        // The 2 by 2 inverse was made once, `00e59700` ran per iteration of the last loop
        // (twice) on `(edx, esi, edi)`.
        assert_eq!(seen.inversions.len(), 1);
        assert_eq!(seen.inversions[0].len(), 4);
        assert_eq!(seen.solves.len(), 2);
    }

    #[test]
    fn the_flag_set_by_the_first_call_lets_the_function_run() {
        let (mut e, seen) = flying_engine(true, true);
        let r = records(&mut e, 3);
        e.call(0x00e5_9a40, &args![r.factor, r.settings, r.state, r.out]);
        let seen = seen.borrow();
        assert_eq!(seen.flag_calls, vec![FLYING_COLORS_FLAG]);
        assert_eq!(seen.inversions.len(), 1);
        assert_eq!(e.mem.u8(FLYING_COLORS_FLAG), 1);
    }

    /// Everything the function wrote: the output, the state and the two bodies.
    fn written(e: &Engine, r: &Records) -> Vec<u32> {
        let mut words = Vec::new();
        for (address, count) in [
            (r.out, 0x20),
            (r.state, 0x80),
            (r.bodies[0], 0x20),
            (r.bodies[1], 0x20),
        ] {
            words.extend((0..count).map(|index| e.mem.u32(address + 4 * index)));
        }
        words
    }

    fn solve_run(seed: u32, fill: u32) -> (Vec<u32>, Vec<[u32; 3]>, Vec<Vec<u32>>) {
        let (mut e, seen) = flying_engine(false, false);
        e.mem.set_u8(FLYING_COLORS_FLAG, 1);
        let r = records(&mut e, seed);
        run_00e59a40(&mut e, [r.factor, r.settings, r.state, r.out], fill);
        let seen = seen.borrow();
        // The addresses differ between runs of different engines only by the allocation order,
        // which is the same; the register words are therefore comparable.
        (
            written(&e, &r),
            seen.solves.clone(),
            seen.inversions.clone(),
        )
    }

    #[test]
    fn the_solve_does_not_depend_on_the_frame_fill() {
        for seed in 4u32..28 {
            let first = solve_run(seed, 0);
            assert_eq!(solve_run(seed, 0xdead_beef), first);
            assert_eq!(solve_run(seed, 0x7fc0_0000), first);
        }
    }

    #[test]
    fn the_solve_changes_the_bodies_and_the_output() {
        let (mut e, _) = flying_engine(false, true);
        e.mem.set_u8(FLYING_COLORS_FLAG, 1);
        let r = records(&mut e, 8);
        let before = written(&e, &r);
        e.call(0x00e5_9a40, &args![r.factor, r.settings, r.state, r.out]);
        let after = written(&e, &r);
        assert_ne!(before, after);
        // The bodies are updated at `+0x10 ..= +0x2c`; nothing from `+0x30` on is written.
        for base in [0xa0usize, 0xc0] {
            assert_eq!(
                before[base + 12..base + 0x20],
                after[base + 12..base + 0x20]
            );
        }
    }

    #[test]
    fn registered_entries_exist() {
        let list = funcs();
        let addresses: Vec<u32> = list.iter().map(|entry| entry.0).collect();
        assert_eq!(addresses, vec![0x00e5_9700, 0x00e5_9a40]);
    }
}
