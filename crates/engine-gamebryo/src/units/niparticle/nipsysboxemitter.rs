//! `NiParticle/nipsysboxemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c2f4f0`: the jump target of `00c1fdd0`.
const JUMP_TARGET_00C1FDD0: u32 = 0x00c2_f4f0;

/// `00c31c90`: the jump target of `00c20150`.
const JUMP_TARGET_00C20150: u32 = 0x00c3_1c90;

/// `00c31b40`: the jump target of `00c20160`.
const JUMP_TARGET_00C20160: u32 = 0x00c3_1b40;

// Translated from 00c1fdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c2f4f0` (engine map: `NiPSysAirFieldModifier::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysBoxEmitter@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_box_emitter_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00C1FDD0, &args![this, stream]).bool()
}

// Translated from 00c20150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::ProcessClone` (Xbox PDB): a 5-byte jump to `00c31c90` (engine map: `NiPSysVolumeEmitter::ProcessClone`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?ProcessClone@NiPSysBoxEmitter@@UAAXAAVNiCloningProcess@@@Z`.
pub fn ni_p_sys_box_emitter_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(JUMP_TARGET_00C20150, &args![this, process]);
}

// Translated from 00c20160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::LinkObject` (Xbox PDB): a 5-byte jump to `00c31b40` (engine map: `NiPSysVolumeEmitter::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysBoxEmitter@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_box_emitter_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C20160, &args![this, stream]);
}

// Translated from 00c201c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2518.
pub fn ni_p_sys_box_emitter_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2518
}

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `_rand` (`00ecadb8`, `cdecl`, no words).
const RAND: u32 = 0x00ec_adb8;
/// `32767.0` (`double`), the divisor that makes `rand()` a fraction.
const RAND_DIVISOR: u32 = 0x0102_9790;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// `00524c40` (unnamed, `thiscall` on a transform, two words: the result
/// point, the point; returns the result): the transform applied to a point.
const TRANSFORM_POINT: u32 = 0x0052_4c40;
/// `00c31af0` (unnamed, `thiscall`, no words): the `NiPSysVolumeEmitter` part
/// of the constructor the clone and create functions run.
const VOLUME_EMITTER_CONSTRUCT: u32 = 0x00c3_1af0;
/// `00c31b10` (`thiscall`, two words: the clone, the cloning process): a jump
/// to `00c21f60`, the emitter part of the member copy.
const VOLUME_EMITTER_COPY_MEMBERS: u32 = 0x00c3_1b10;
/// `NiPSysModifier::~NiPSysModifier` (`00c31750`, `thiscall`).
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
/// `NiPSysVolumeEmitter::LoadBinary` (`00c31b20`, `thiscall`, one word: the stream).
const VOLUME_EMITTER_LOAD_BINARY: u32 = 0x00c3_1b20;
/// The `NiPSysBoxEmitter` vtable the constructors store.
const BOX_EMITTER_VTABLE: u32 = 0x010b_da2c;
/// The size of a `NiPSysBoxEmitter` (the allocations and the sized delete).
const BOX_EMITTER_SIZE: u32 = 0x64;

layout! {
    /// `NiPSysBoxEmitter` (Xbox PDB), 0x64 bytes on PC: the emitter's box
    /// (floats, kept as their bit patterns).
    pub struct NiPSysBoxEmitter: 0x64 {
        /// `m_fEmitterWidth` (Xbox PDB).
        0x58 m_fEmitterWidth: u32,
        /// `m_fEmitterHeight` (Xbox PDB).
        0x5C m_fEmitterHeight: u32,
        /// `m_fEmitterDepth` (Xbox PDB).
        0x60 m_fEmitterDepth: u32,
    }
}

/// `FLD dword` of the float at `addr`.
fn load_float(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// The float at `addr` as an x87 arithmetic memory operand.
fn float_operand(e: &Engine, addr: u32) -> f64 {
    x87_m32(e.mem.u32(addr))
}

/// `rand() / 32767.0` stored as a float (`FILD`, `FDIV double`, `FSTP dword`).
fn random_fraction(e: &mut Engine) -> u32 {
    let value = e.call(RAND, &[]).i32();
    let divisor: f64 = e.global(RAND_DIVISOR);
    x87_store32(x87_div(f64::from(value), divisor))
}

/// One `NiStream` read through the input file the stream keeps at `+0x24c`:
/// the file's read function (`+8`, `cdecl (file, dest, size, &count, 1)`)
/// with the count word set to `size`, then the result added to the file's
/// position (`+4`).
fn stream_read(e: &mut Engine, stream: Ptr, dest: u32, size: u32, count: Ptr) {
    let file = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(file + 8);
    e.mem.set_u32(count.addr(), size);
    let got = e.call(read, &args![file, dest, size, count, 1u32]).u32();
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(got));
}

// Translated from 00c201d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::ComputeVolumeInitialPositionAndVelocity` (Xbox PDB):
/// three `rand()` fractions `r1`, `r2`, `r3` (in that call order) make the
/// position `((r3 - 0.5) * width, (r2 - 0.5) * height, (r1 - 0.5) * depth)`,
/// which is stored in `position`, put through `transform` (`00524c40` on a
/// local copy) and stored again; then `velocity` is multiplied by the
/// transform's rotation (rows at `+0`, `+0xC`, `+0x18`), all three inputs read
/// before any is written. x87 code, one helper call per instruction.
pub fn ni_p_sys_box_emitter_compute_volume_initial_position_and_velocity(
    e: &mut Engine,
    this: Ptr<NiPSysBoxEmitter>,
    transform: Ptr,
    position: Ptr,
    velocity: Ptr,
) {
    let first = random_fraction(e);
    let second = random_fraction(e);
    let third = random_fraction(e);
    let half = x87_load64(e.global::<u64>(HALF));
    let base = this.addr();
    let x = x87_store32(x87_mul(
        x87_sub(x87_load32(third), half),
        load_float(e, base + 0x58),
    ));
    let y = x87_store32(x87_mul(
        x87_sub(x87_load32(second), half),
        float_operand(e, base + 0x5c),
    ));
    let z = x87_store32(x87_mul(
        x87_sub(x87_m32(first), half),
        float_operand(e, base + 0x60),
    ));
    let out = position.addr();
    e.mem.set_u32(out, x);
    e.mem.set_u32(out + 4, y);
    e.mem.set_u32(out + 8, z);
    e.with_stack(0xc, |e, local| {
        e.mem.set_u32(local.addr(), x);
        e.mem.set_u32(local.addr() + 4, y);
        e.mem.set_u32(local.addr() + 8, z);
        let result = e
            .call(TRANSFORM_POINT, &args![transform, local, position])
            .u32();
        for i in 0..3 {
            let word = e.mem.u32(result + 4 * i);
            e.mem.set_u32(out + 4 * i, word);
        }
    });
    let m = transform.addr();
    let v = velocity.addr();
    let row = |e: &Engine, first: u32| -> u32 {
        let middle = x87_mul(load_float(e, m + first + 4), float_operand(e, v + 4));
        let left = x87_mul(load_float(e, m + first), float_operand(e, v));
        let right = x87_mul(load_float(e, m + first + 8), float_operand(e, v + 8));
        x87_store32(x87_add(x87_add(middle, left), right))
    };
    let vx = row(e, 0);
    let vy = row(e, 0xc);
    let vz = row(e, 0x18);
    e.mem.set_u32(v, vx);
    e.mem.set_u32(v + 4, vy);
    e.mem.set_u32(v + 8, vz);
}

/// The inlined constructor of `00c202f0` and `00c20350`: `00c31af0`, the box
/// zeroed (`FLDZ`), the vtable.
fn construct_box_emitter(e: &mut Engine, block: u32) -> Ptr<NiPSysBoxEmitter> {
    let clone: Ptr<NiPSysBoxEmitter> = Ptr::new(block);
    e.call(VOLUME_EMITTER_CONSTRUCT, &args![clone]);
    e.set(clone, NiPSysBoxEmitter::m_fEmitterWidth, 0);
    e.mem.set_u32(block, BOX_EMITTER_VTABLE);
    e.set(clone, NiPSysBoxEmitter::m_fEmitterHeight, 0);
    e.set(clone, NiPSysBoxEmitter::m_fEmitterDepth, 0);
    clone
}

// Translated from 00c202f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::CreateClone` (Xbox PDB): allocates 0x64 bytes; when the
/// allocation succeeds runs the volume-emitter construction (`00c31af0`),
/// zeroes the box and stores the box-emitter vtable. Then (also on a null
/// clone, as the exe does) copies the emitter members (`00c31b10(clone,
/// process)`) and the three box floats (`FLD`/`FSTP`, which quiet a
/// signalling NaN). Answers the clone.
pub fn ni_p_sys_box_emitter_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysBoxEmitter>,
    process: Ptr,
) -> Ptr<NiPSysBoxEmitter> {
    let block = e.call(NI_ALLOCATE, &args![BOX_EMITTER_SIZE]).u32();
    let clone: Ptr<NiPSysBoxEmitter> = if block == 0 {
        Ptr::NULL
    } else {
        construct_box_emitter(e, block)
    };
    e.call(VOLUME_EMITTER_COPY_MEMBERS, &args![this, clone, process]);
    for field in [
        NiPSysBoxEmitter::m_fEmitterWidth,
        NiPSysBoxEmitter::m_fEmitterHeight,
        NiPSysBoxEmitter::m_fEmitterDepth,
    ] {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(clone, field, value);
    }
    clone
}

// Translated from 00c20350 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's creation function (Ghidra made no function of it; the
/// `NiStream` loader table of `00c1aa00` lists it for `NiPSysBoxEmitter`;
/// `cdecl`, no words): allocates 0x64 bytes and, when that succeeds,
/// constructs a box emitter there (`00c31af0`, the box zeroed, the vtable);
/// answers it, or null.
pub fn fn_00c20350(e: &mut Engine) -> Ptr<NiPSysBoxEmitter> {
    let block = e.call(NI_ALLOCATE, &args![BOX_EMITTER_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct_box_emitter(e, block)
}

// Translated from 00c20390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::_scalar_deleting_destructor_` (Xbox PDB): the modifier
/// destructor (`00c31750`), then the sized delete of 0x64 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_box_emitter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, BOX_EMITTER_SIZE]);
    }
    this
}

// Translated from 00c203b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::LoadBinary` (Xbox PDB): the volume-emitter part
/// (`00c31b20`), then width, height and depth, four bytes each, read from the
/// stream's file (the count word is the function's own argument slot).
pub fn ni_p_sys_box_emitter_load_binary(e: &mut Engine, this: Ptr<NiPSysBoxEmitter>, stream: Ptr) {
    e.call(VOLUME_EMITTER_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), stream.addr());
        for offset in [0x58u32, 0x5c, 0x60] {
            stream_read(e, stream, this.addr() + offset, 4, count);
        }
    });
}

use crate::units::niparticle::nipsysdata::{add_viewer_string, stream_write};

/// `NiPSysVolumeEmitter`'s `IsEqual` (`00c31b80`), `SaveBinary`
/// (`00c31b60`) and `GetViewerStrings` (`00c31db0`) (Xbox PDB names;
/// `thiscall`, one word).
const VOLUME_EMITTER_IS_EQUAL: u32 = 0x00c3_1b80;
const VOLUME_EMITTER_SAVE_BINARY: u32 = 0x00c3_1b60;
const VOLUME_EMITTER_GET_VIEWER_STRINGS: u32 = 0x00c3_1db0;
/// `NiGetViewerString` (`00a7d330`, the text) and `NiGetViewerString_ov9`
/// (`00a7d550`: a label and a `float`) (Xbox PDB names; `cdecl`).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// `NiPSysBoxEmitter::ms_RTTI` (its name is the first word).
const RTTI: u32 = 0x0120_2518;
/// The labels "Width" (`0109dc8c`), "Height" (`0109dc84`) and "Depth"
/// (`010a0eb4`).
const LABEL_WIDTH: u32 = 0x0109_dc8c;
const LABEL_HEIGHT: u32 = 0x0109_dc84;
const LABEL_DEPTH: u32 = 0x010a_0eb4;

// Translated from 00c20170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::IsEqual` (Xbox PDB): `NiPSysVolumeEmitter`'s, then
/// equal width, height and depth (`FUCOMPP`: a NaN is unequal).
pub fn ni_p_sys_box_emitter_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysBoxEmitter>,
    other: Ptr<NiPSysBoxEmitter>,
) -> bool {
    if !e.call(VOLUME_EMITTER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    for field in [
        NiPSysBoxEmitter::m_fEmitterWidth,
        NiPSysBoxEmitter::m_fEmitterHeight,
        NiPSysBoxEmitter::m_fEmitterDepth,
    ] {
        let ours = x87_m32(e.get(this, field));
        let theirs = x87_m32(e.get(other, field));
        if theirs != ours {
            return false;
        }
    }
    true
}

// Translated from 00c20440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::SaveBinary` (Xbox PDB): `NiPSysVolumeEmitter`'s, then
/// the width, height and depth (4 bytes each).
pub fn ni_p_sys_box_emitter_save_binary(e: &mut Engine, this: Ptr<NiPSysBoxEmitter>, stream: Ptr) {
    e.call(VOLUME_EMITTER_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, slot| {
        for field in [
            NiPSysBoxEmitter::m_fEmitterWidth,
            NiPSysBoxEmitter::m_fEmitterHeight,
            NiPSysBoxEmitter::m_fEmitterDepth,
        ] {
            let value = this.addr() + field.off;
            stream_write(e, stream.addr(), value, 4, 4, slot.addr());
        }
    });
}

// Translated from 00c204d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBoxEmitter::GetViewerStrings` (Xbox PDB): `NiPSysVolumeEmitter`'s
/// strings, the class name, then the width, height and depth.
pub fn ni_p_sys_box_emitter_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysBoxEmitter>,
    strings: Ptr,
) {
    let strings = strings.addr();
    e.call(VOLUME_EMITTER_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, string);
    for (label, field) in [
        (LABEL_WIDTH, NiPSysBoxEmitter::m_fEmitterWidth),
        (LABEL_HEIGHT, NiPSysBoxEmitter::m_fEmitterHeight),
        (LABEL_DEPTH, NiPSysBoxEmitter::m_fEmitterDepth),
    ] {
        let value = x87_quiet_bits(e.get(this, field));
        let string = e.call(VIEWER_STRING_FLOAT, &args![label, value]).u32();
        add_viewer_string(e, strings, string);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_01d0,
            ni_p_sys_box_emitter_compute_volume_initial_position_and_velocity(
                Ptr<NiPSysBoxEmitter>,
                Ptr,
                Ptr,
                Ptr,
            )
        ),
        entry!(
            0x00c2_02f0,
            ni_p_sys_box_emitter_create_clone(Ptr<NiPSysBoxEmitter>, Ptr) -> Ptr<NiPSysBoxEmitter>
        ),
        entry!(0x00c2_0350, fn_00c20350() -> Ptr<NiPSysBoxEmitter>),
        entry!(0x00c2_0390, ni_p_sys_box_emitter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c2_03b0,
            ni_p_sys_box_emitter_load_binary(Ptr<NiPSysBoxEmitter>, Ptr)
        ),
        entry!(0x00c201c0, ni_p_sys_box_emitter_get_r_t_t_i() -> u32),
        entry!(0x00c1_fdd0, ni_p_sys_box_emitter_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c2_0150, ni_p_sys_box_emitter_process_clone(Ptr, Ptr)),
        entry!(0x00c2_0160, ni_p_sys_box_emitter_link_object(Ptr, Ptr)),
        entry!(
            0x00c2_0170,
            ni_p_sys_box_emitter_is_equal(Ptr<NiPSysBoxEmitter>, Ptr<NiPSysBoxEmitter>) -> bool
        ),
        entry!(
            0x00c2_0440,
            ni_p_sys_box_emitter_save_binary(Ptr<NiPSysBoxEmitter>, Ptr)
        ),
        entry!(
            0x00c2_04d0,
            ni_p_sys_box_emitter_get_viewer_strings(Ptr<NiPSysBoxEmitter>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c1fdd0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c1_fdd0;
        const TARGET: u32 = 0x00c2_f4f0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00c20150_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_0150;
        const TARGET: u32 = 0x00c3_1c90;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00c20160_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_0160;
        const TARGET: u32 = 0x00c3_1b40;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn ni_p_sys_box_emitter_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_01c0, &args![]).u32(), 0x0120_2518);
    }
}

#[cfg(test)]
mod emitter_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn set_floats(e: &mut Engine, addr: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(addr + 4 * i as u32, *v);
        }
    }

    fn floats(e: &Engine, addr: u32) -> [f32; 3] {
        [e.mem.f32(addr), e.mem.f32(addr + 4), e.mem.f32(addr + 8)]
    }

    #[test]
    fn compute_volume_position_scales_the_fractions_and_rotates_the_velocity() {
        let mut e = Engine::new();
        e.map(RAND_DIVISOR, 8);
        e.set_global(RAND_DIVISOR, 32767.0f64);
        e.map(HALF, 8);
        e.set_global(HALF, 0.5f64);
        let mut sequence = vec![32767, 0, 16383].into_iter();
        e.register_double(RAND, move |_, _| Ret {
            eax: sequence.next().unwrap(),
            ..Ret::default()
        });
        let moved = e.mem.alloc(0xc);
        set_floats(&mut e, moved, &[7.0, 8.0, 9.0]);
        e.register_double(TRANSFORM_POINT, move |_, _| Ret {
            eax: moved,
            ..Ret::default()
        });
        let this: Ptr<NiPSysBoxEmitter> = e.new_object();
        set_floats(&mut e, this.addr() + 0x58, &[2.0, 4.0, 8.0]);
        let transform = e.mem.alloc(0x34);
        set_floats(
            &mut e,
            transform,
            &[0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0],
        );
        let position = e.mem.alloc(0xc);
        let velocity = e.mem.alloc(0xc);
        set_floats(&mut e, velocity, &[1.0, 2.0, 3.0]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_01d0, &args![this, transform, position, velocity]);
        let call = &calls_to(&e, TRANSFORM_POINT)[0];
        assert_eq!(call[0], transform);
        assert_eq!(call[2], position);
        // r1 = 1, r2 = 0, r3 = 16383 / 32767: x = (r3 - 0.5) * 2, y = -0.5 * 4, z = 0.5 * 8.
        let r3 = (16383.0f64 / 32767.0) as f32;
        assert_eq!(
            floats(&e, call[1]),
            [((r3 as f64 - 0.5) * 2.0) as f32, -2.0, 4.0]
        );
        assert_eq!(floats(&e, position), [7.0, 8.0, 9.0]);
        assert_eq!(floats(&e, velocity), [2.0, 1.0, 6.0]);
    }

    #[test]
    fn create_clone_constructs_and_copies_the_box_quieting_nans() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(VOLUME_EMITTER_CONSTRUCT, |_, _| Ret::default());
        e.register(VOLUME_EMITTER_COPY_MEMBERS, |_, _| Ret::default());
        let this: Ptr<NiPSysBoxEmitter> = e.new_object();
        e.set(this, NiPSysBoxEmitter::m_fEmitterWidth, 0x7f80_0001);
        e.set(this, NiPSysBoxEmitter::m_fEmitterHeight, 2.5f32.to_bits());
        e.set(this, NiPSysBoxEmitter::m_fEmitterDepth, 3.5f32.to_bits());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_02f0, &args![this, 0x7000u32]).u32();
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x64]]);
        assert_eq!(calls_to(&e, VOLUME_EMITTER_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, VOLUME_EMITTER_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x7000]]
        );
        assert_eq!(e.mem.u32(clone), BOX_EMITTER_VTABLE);
        assert_eq!(e.mem.u32(clone + 0x58), 0x7fc0_0001);
        assert_eq!(e.mem.f32(clone + 0x5c), 2.5);
        assert_eq!(e.mem.f32(clone + 0x60), 3.5);
    }

    #[test]
    fn create_function_answers_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(VOLUME_EMITTER_CONSTRUCT, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c2_0350, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(VOLUME_EMITTER_CONSTRUCT, |_, _| Ret::default());
        let made = e.call(0x00c2_0350, &args![]).u32();
        assert_eq!(e.mem.u32(made), BOX_EMITTER_VTABLE);
        assert_eq!(e.mem.u32(made + 0x60), 0);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.register(MODIFIER_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_0390, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert!(calls_to(&e, NI_SIZED_DELETE).is_empty());
        e.call(0x00c2_0390, &args![0x5000u32, 1u32]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE), vec![vec![0x5000, 0x64]]);
        assert_eq!(calls_to(&e, MODIFIER_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn load_binary_reads_the_three_box_floats() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            e.mem.set_f32(a[1], 1.25);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this: Ptr<NiPSysBoxEmitter> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c2_03b0, &args![this, stream]);
        let reads = calls_to(&e, READ);
        let dests: Vec<u32> = reads.iter().map(|w| w[1]).collect();
        assert_eq!(
            dests,
            vec![this.addr() + 0x58, this.addr() + 0x5c, this.addr() + 0x60]
        );
        assert_eq!(e.mem.u32(file + 4), 12);
        assert_eq!(floats(&e, this.addr() + 0x58), [1.25; 3]);
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to};

    fn emitter(e: &mut Engine, size: [f32; 3]) -> Ptr<NiPSysBoxEmitter> {
        let this = e.new_object::<NiPSysBoxEmitter>();
        e.set(this, NiPSysBoxEmitter::m_fEmitterWidth, size[0].to_bits());
        e.set(this, NiPSysBoxEmitter::m_fEmitterHeight, size[1].to_bits());
        e.set(this, NiPSysBoxEmitter::m_fEmitterDepth, size[2].to_bits());
        this
    }

    #[test]
    fn is_equal_compares_the_box() {
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_IS_EQUAL, |_, _| answer(1));
        let this = emitter(&mut e, [1.0, 2.0, 3.0]);
        let check = |e: &mut Engine, other| e.call(0x00c2_0170, &args![this, other]).bool();
        let same = emitter(&mut e, [1.0, 2.0, 3.0]);
        assert!(check(&mut e, same));
        for size in [[0.0, 2.0, 3.0], [1.0, 0.0, 3.0], [1.0, 2.0, 0.0]] {
            let other = emitter(&mut e, size);
            assert!(!check(&mut e, other));
        }
        e.set(this, NiPSysBoxEmitter::m_fEmitterDepth, f32::NAN.to_bits());
        let nan = emitter(&mut e, [1.0, 2.0, f32::NAN]);
        assert!(!check(&mut e, nan));
        e.register(VOLUME_EMITTER_IS_EQUAL, |_, _| answer(0));
        assert!(!check(&mut e, same));
    }

    #[test]
    fn save_binary_writes_the_box() {
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_SAVE_BINARY, |_, _| Ret::default());
        let stream = viewer_test::out_stream(&mut e);
        let this = emitter(&mut e, [1.0, 2.0, 3.0]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_0440, &args![this, stream]);
        assert_eq!(
            calls_to(&e, VOLUME_EMITTER_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        let bytes: Vec<u8> = [1.0f32, 2.0, 3.0]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect();
        assert_eq!(viewer_test::written(&e, stream), (bytes, 12));
    }

    #[test]
    fn viewer_strings_add_the_name_and_the_box() {
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        viewer_test::counting(&mut e, &[VIEWER_STRING, VIEWER_STRING_FLOAT]);
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_0060);
        let strings = viewer_test::strings(&mut e, 2);
        let this = emitter(&mut e, [1.0, 2.0, 3.0]);
        e.set(this, NiPSysBoxEmitter::m_fEmitterWidth, 0xff80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00c2_04d0, &args![this, strings]);
        assert_eq!(viewer_test::collected(&e, strings), vec![1, 2, 3, 4]);
        assert_eq!(
            calls_to(&e, VOLUME_EMITTER_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x0100_0060]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FLOAT),
            vec![
                vec![LABEL_WIDTH, 0xffc0_0001],
                vec![LABEL_HEIGHT, 2.0f32.to_bits()],
                vec![LABEL_DEPTH, 3.0f32.to_bits()]
            ]
        );
    }
}
