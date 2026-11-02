//! `NiParticle/nipsyssphereemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c20020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x5c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_sphere_emitter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x5cu32]);
    }
    this
}

// Translated from 00c1fe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2510.
pub fn ni_p_sys_sphere_emitter_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2510
}

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00c31af0` (unnamed, `thiscall`, no words): the `NiPSysVolumeEmitter` part
/// of the constructor.
const VOLUME_EMITTER_CONSTRUCT: u32 = 0x00c3_1af0;
/// `00c31b10` (`thiscall`, two words: the clone, the cloning process): the
/// emitter part of the member copy.
const VOLUME_EMITTER_COPY_MEMBERS: u32 = 0x00c3_1b10;
/// `NiPSysVolumeEmitter::LoadBinary` (`00c31b20`, `thiscall`, one word).
const VOLUME_EMITTER_LOAD_BINARY: u32 = 0x00c3_1b20;
/// The `NiPSysSphereEmitter` vtable.
const SPHERE_EMITTER_VTABLE: u32 = 0x010b_d974;
/// `m_fEmitterRadius` (Xbox PDB), the float at `+0x58`, as bits.
const RADIUS: u32 = 0x58;

/// The inlined constructor: `00c31af0`, the radius 0 (`FLDZ`), the vtable.
fn construct_sphere_emitter(e: &mut Engine, block: u32) -> Ptr {
    e.call(VOLUME_EMITTER_CONSTRUCT, &args![block]);
    e.mem.set_u32(block + RADIUS, 0);
    e.mem.set_u32(block, SPHERE_EMITTER_VTABLE);
    Ptr::new(block)
}

// Translated from 00c1ffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::CreateClone` (Xbox PDB): 0x5C bytes constructed as a
/// sphere emitter (null when the allocation fails, then used anyway as the
/// exe does), the emitter members copied (`00c31b10`), the radius copied
/// (`FLD`/`FSTP`, quieting a signalling NaN). Answers the clone.
pub fn ni_p_sys_sphere_emitter_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x5cu32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        construct_sphere_emitter(e, block)
    };
    e.call(VOLUME_EMITTER_COPY_MEMBERS, &args![this, clone, process]);
    let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
    e.mem.set_u32(clone.addr() + RADIUS, radius);
    clone
}

// Translated from 00c1fff0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's creation function (Ghidra made no function of it; listed for
/// `NiPSysSphereEmitter` in `00c1aa00`'s loader table; `cdecl`, no words):
/// 0x5C bytes constructed as a sphere emitter, or null.
pub fn fn_00c1fff0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x5cu32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct_sphere_emitter(e, block)
}

// Translated from 00c20040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::LoadBinary` (Xbox PDB): the volume-emitter part,
/// then the radius, four bytes read from the stream's input file (`+0x24c`,
/// its read function at `+8`, `cdecl (file, dest, size, &count, 1)`, the
/// result added to its position `+4`); the count word is the argument slot.
pub fn ni_p_sys_sphere_emitter_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(VOLUME_EMITTER_LOAD_BINARY, &args![this, stream]);
    let file = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(file + 8);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        let got = e
            .call(read, &args![file, this.addr() + RADIUS, 4u32, count, 1u32])
            .u32();
        let position = e.mem.u32(file + 4);
        e.mem.set_u32(file + 4, position.wrapping_add(got));
    });
}

/// `NiPSysVolumeEmitter::IsEqual` (`00c31b80`, `thiscall`, one word),
/// `SaveBinary` (`00c31b60`, `thiscall`, one word) and `GetViewerStrings`
/// (`00c31db0`, `thiscall`, one word).
const VOLUME_EMITTER_IS_EQUAL: u32 = 0x00c3_1b80;
const VOLUME_EMITTER_SAVE_BINARY: u32 = 0x00c3_1b60;
const VOLUME_EMITTER_GET_VIEWER_STRINGS: u32 = 0x00c3_1db0;
/// `_rand` (`00ecadb8`, `cdecl`, no words) and `_ftol2` (`00ec62c0`: the
/// `ST0` value as a leading `f64`, popped).
const RAND: u32 = 0x00ec_adb8;
const FTOL: u32 = 0x00ec_62c0;
/// `NiMatrix3::operator*(NiPoint3)` (`00524c40`, `thiscall` on the matrix,
/// `(result, point)`, answers the result).
const MATRIX_TIMES_POINT: u32 = 0x0052_4c40;
/// The `double` 32767.0 (`RAND_MAX`, `01029790`) and 512.0 (`01028338`), and
/// the `float` 2π (`011aea4c`).
const RAND_MAX_DOUBLE: u32 = 0x0102_9790;
const TABLE_SIZE_DOUBLE: u32 = 0x0102_8338;
const TWO_PI_FLOAT: u32 = 0x011a_ea4c;
/// The two 512-entry `float` tables read by angle index (`011f52e0`,
/// `011f4ae0`).
const TABLE_A: u32 = 0x011f_52e0;
const TABLE_B: u32 = 0x011f_4ae0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, one word: the class name) and
/// its `float` overload (`00a7d550`, `cdecl`, `(name, value)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// The viewer-string array's grow (`0096ad30`, `thiscall`, one word: the
/// new size) and store (`0096ae90`, `thiscall`, `(index, &string)`).
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The `NiRTTI` record of `NiPSysSphereEmitter` (its first word is the class
/// name) and the string `"Radius"` (`01015a9c`).
const SPHERE_EMITTER_RTTI: u32 = 0x0120_2510;
const RADIUS_NAME: u32 = 0x0101_5a9c;
/// `NiStream`'s output stream (+0x250) and that stream's write function
/// (+0xC, `cdecl (stream, source, size, &count, 1)`) and position (+4).
const STREAM_OUTPUT: u32 = 0x250;

/// Appends `text` to the viewer-string array (the inlined `Add`): grows it
/// to size plus step when the 16-bit size (+0xA) has reached the capacity
/// (+0x8), then stores the string at the old size through the argument
/// slot's address.
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00c1fde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::IsEqual` (Xbox PDB): the volume-emitter comparison
/// (`00c31b80`), then the radii (`FUCOMPP`: a NaN is unequal).
pub fn ni_p_sys_sphere_emitter_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(VOLUME_EMITTER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = e.mem.f32(this.addr() + RADIUS);
    let theirs = e.mem.f32(other.addr() + RADIUS);
    theirs == mine
}

// Translated from 00c1fe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::ComputeVolumeInitialPositionAndVelocity` (Xbox
/// PDB): a random point in the sphere: the radius times `rand()/32767`, two
/// angles `2π * rand()/32767` turned into table indices (`ftol(angle * 512 /
/// 2π) & 0x1ff`) and looked up in the two 512-entry tables; the position is
/// `(B[i1] * r * A[i2], r * A[i2] * A[i1], r * B[i2])`, turned by
/// `orientation` (`00524c40`), and the velocity is turned by
/// `orientation` inline (every product read before any store). x87 in the
/// exe's order.
pub fn ni_p_sys_sphere_emitter_compute_volume_initial_position_and_velocity(
    e: &mut Engine,
    this: Ptr,
    orientation: Ptr,
    position: Ptr,
    velocity: Ptr,
) {
    let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
    let r = e.call(RAND, &args![]).i32();
    let unit = x87_store32(x87_div(f64::from(r), rand_max));
    let radius = x87_store32(x87_mul(
        x87_load32(e.mem.u32(this.addr() + RADIUS)),
        x87_m32(unit),
    ));
    let two_pi = x87_load32(e.global::<u32>(TWO_PI_FLOAT));
    let r = e.call(RAND, &args![]).i32();
    let unit = x87_store32(x87_div(f64::from(r), rand_max));
    let first_angle = x87_store32(x87_mul(two_pi, x87_m32(unit)));
    let r = e.call(RAND, &args![]).i32();
    let second_unit = x87_store32(x87_div(f64::from(r), rand_max));
    // FLD 2π; FLD ST0; FDIVR 512.0: the scale 512 / 2π stays on the stack.
    let table_size: f64 = e.global(TABLE_SIZE_DOUBLE);
    let scale = x87_div(table_size, two_pi);
    let first = e
        .call(FTOL, &args![x87_mul(x87_load32(first_angle), scale)])
        .u32()
        & 0x1ff;
    let a_first = x87_quiet_bits(e.global::<u32>(TABLE_A + 4 * first));
    // FMULP ST2: 2π * the third random value, stored as the second angle.
    let second_angle = x87_store32(x87_mul(two_pi, x87_load32(second_unit)));
    let second = e
        .call(FTOL, &args![x87_mul(scale, x87_m32(second_angle))])
        .u32()
        & 0x1ff;
    let a_second = x87_quiet_bits(e.global::<u32>(TABLE_A + 4 * second));
    let b_second = x87_quiet_bits(e.global::<u32>(TABLE_B + 4 * second));
    let r = x87_load32(radius);
    let ring = x87_mul(x87_load32(a_second), r);
    let b_first = x87_load32(e.global::<u32>(TABLE_B + 4 * first));
    let x = x87_store32(x87_mul(b_first, ring));
    e.mem.set_u32(position.addr(), x);
    let y = x87_store32(x87_mul(ring, x87_load32(a_first)));
    e.mem.set_u32(position.addr() + 4, y);
    let z = x87_store32(x87_mul(r, x87_m32(b_second)));
    e.mem.set_u32(position.addr() + 8, z);
    e.with_stack(12, |e, turned| {
        let result = e
            .call(MATRIX_TIMES_POINT, &args![orientation, turned, position])
            .u32();
        for word in 0..3 {
            let value = e.mem.u32(result + 4 * word);
            e.mem.set_u32(position.addr() + 4 * word, value);
        }
    });
    let m = |e: &Engine, i: u32| x87_load32(e.mem.u32(orientation.addr() + 4 * i));
    let v = |e: &Engine, i: u32| x87_m32(e.mem.u32(velocity.addr() + 4 * i));
    let mut rows = [0u32; 3];
    for (row, out) in rows.iter_mut().enumerate() {
        let base = 3 * row as u32;
        let sum = x87_add(
            x87_mul(m(e, base + 1), v(e, 1)),
            x87_mul(m(e, base), v(e, 0)),
        );
        *out = x87_store32(x87_add(sum, x87_mul(m(e, base + 2), v(e, 2))));
    }
    for (word, value) in rows.iter().enumerate() {
        e.mem.set_u32(velocity.addr() + 4 * word as u32, *value);
    }
}

// Translated from 00c20080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::SaveBinary` (Xbox PDB): the volume-emitter part
/// (`00c31b60`), then the radius, four bytes through the stream's output
/// (its write function at +0xC, the result added to its position +4); the
/// count word is the argument slot.
pub fn ni_p_sys_sphere_emitter_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(VOLUME_EMITTER_SAVE_BINARY, &args![this, stream]);
    let output = e.mem.u32(stream.addr() + STREAM_OUTPUT);
    let write = e.mem.u32(output + 0xc);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        let put = e
            .call(
                write,
                &args![output, this.addr() + RADIUS, 4u32, count, 1u32],
            )
            .u32();
        let position = e.mem.u32(output + 4);
        e.mem.set_u32(output + 4, position.wrapping_add(put));
    });
}

// Translated from 00c200c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::GetViewerStrings` (Xbox PDB): the volume-emitter
/// strings (`00c31db0`), then the class name and `"Radius"` with the radius
/// (`FLD`/`FSTP` pushes it, quieting a signalling NaN).
pub fn ni_p_sys_sphere_emitter_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(VOLUME_EMITTER_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(SPHERE_EMITTER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
    let text = e
        .call(NI_GET_VIEWER_STRING_FLOAT, &args![RADIUS_NAME, radius])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1_fde0, ni_p_sys_sphere_emitter_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00c1_fe20,
            ni_p_sys_sphere_emitter_compute_volume_initial_position_and_velocity(
                Ptr, Ptr, Ptr, Ptr
            )
        ),
        entry!(0x00c2_0080, ni_p_sys_sphere_emitter_save_binary(Ptr, Ptr)),
        entry!(
            0x00c2_00c0,
            ni_p_sys_sphere_emitter_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c1_ffa0, ni_p_sys_sphere_emitter_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c1_fff0, fn_00c1fff0() -> Ptr),
        entry!(0x00c2_0040, ni_p_sys_sphere_emitter_load_binary(Ptr, Ptr)),
        entry!(0x00c1fe10, ni_p_sys_sphere_emitter_get_r_t_t_i() -> u32),
        entry!(0x00c20020, ni_p_sys_sphere_emitter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_sphere_emitter_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c3_1750, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c20020, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c20020, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x5c]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_sphere_emitter_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c1_fe10, &args![]).u32(), 0x0120_2510);
    }
}

#[cfg(test)]
mod sphere_tests {
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

    #[test]
    fn create_clone_constructs_and_copies_the_radius() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(VOLUME_EMITTER_CONSTRUCT, |_, _| Ret::default());
        e.register(VOLUME_EMITTER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + RADIUS, 0x7f80_0002);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c1_ffa0, &args![this, 0x7000u32]).u32();
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x5c]]);
        assert_eq!(
            calls_to(&e, VOLUME_EMITTER_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        assert_eq!(e.mem.u32(clone), SPHERE_EMITTER_VTABLE);
        assert_eq!(e.mem.u32(clone + RADIUS), 0x7fc0_0002);
    }

    #[test]
    fn create_function_constructs_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(VOLUME_EMITTER_CONSTRUCT, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c1_fff0, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(VOLUME_EMITTER_CONSTRUCT, |_, _| Ret::default());
        let made = e.call(0x00c1_fff0, &args![]).u32();
        assert_eq!(e.mem.u32(made), SPHERE_EMITTER_VTABLE);
    }

    #[test]
    fn load_binary_reads_the_radius() {
        const READ: u32 = 0x0100_0010;
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_f32(a[1], 2.5);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 4, 0x20);
        e.mem.set_u32(file + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let this = e.mem.alloc(0x5c);
        e.call_log = Some(vec![]);
        e.call(0x00c2_0040, &args![this, stream]);
        assert_eq!(
            calls_to(&e, VOLUME_EMITTER_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.f32(this + RADIUS), 2.5);
        assert_eq!(e.mem.u32(file + 4), 0x24);
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn is_equal_needs_the_base_and_equal_radii() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x5c);
        let other = e.mem.alloc(0x5c);
        e.mem.set_f32(this + RADIUS, 2.0);
        e.mem.set_f32(other + RADIUS, 2.0);
        e.register(VOLUME_EMITTER_IS_EQUAL, |_, _| word(0x1234_5600));
        assert!(!e.call(0x00c1_fde0, &args![this, other]).bool());
        e.register(VOLUME_EMITTER_IS_EQUAL, |_, _| word(1));
        assert!(e.call(0x00c1_fde0, &args![this, other]).bool());
        e.mem.set_f32(other + RADIUS, 3.0);
        assert!(!e.call(0x00c1_fde0, &args![this, other]).bool());
        e.mem.set_u32(this + RADIUS, 0x7fc0_0000);
        e.mem.set_u32(other + RADIUS, 0x7fc0_0000);
        assert!(!e.call(0x00c1_fde0, &args![this, other]).bool());
    }

    #[test]
    fn compute_position_and_velocity_uses_the_tables_and_the_orientation() {
        let mut e = Engine::new();
        e.map(RAND_MAX_DOUBLE, 8);
        e.set_global(RAND_MAX_DOUBLE, 32767.0f64);
        e.map(TABLE_SIZE_DOUBLE, 8);
        e.set_global(TABLE_SIZE_DOUBLE, 512.0f64);
        e.map(TWO_PI_FLOAT, 4);
        e.set_global(TWO_PI_FLOAT, 4.0f32);
        e.map(TABLE_A, 0x800);
        e.map(TABLE_B, 0x800);
        for i in 0..512u32 {
            e.set_global(TABLE_A + 4 * i, i as f32);
            e.set_global(TABLE_B + 4 * i, 1000.0 + i as f32);
        }
        // rand: 32767 (radius scale 1), 32767 (angle 4.0), 16383.5-ish.
        let rolls = Rc::new(RefCell::new(vec![32767, 32767, 0]));
        let next = rolls.clone();
        e.register_double(RAND, move |_, _| word(next.borrow_mut().remove(0)));
        e.register(FTOL, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word(v as i32 as u32)
        });
        e.register(MATRIX_TIMES_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v + 1.0);
            }
            word(a[1])
        });
        let this = e.mem.alloc(0x5c);
        e.mem.set_f32(this + RADIUS, 2.0);
        let orientation = e.mem.alloc(0x24);
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(orientation + 4 * i as u32, *v);
        }
        let position = e.mem.alloc(12);
        let velocity = e.mem.alloc(12);
        e.mem.set_f32(velocity, 1.0);
        e.mem.set_f32(velocity + 4, 10.0);
        e.mem.set_f32(velocity + 8, 100.0);
        e.call_log = Some(vec![]);
        e.call(0x00c1_fe20, &args![this, orientation, position, velocity]);
        // First index: ftol(4.0 * 512 / 4) & 0x1ff = 512 & 0x1ff = 0; the
        // second: the angle 0, index 0.
        let ring = 0.0f32 * 2.0;
        assert_eq!(e.mem.f32(position), 1000.0 * ring + 1.0);
        assert_eq!(e.mem.f32(position + 4), ring * 0.0 + 1.0);
        assert_eq!(e.mem.f32(position + 8), 2.0 * 1000.0 + 1.0);
        assert_eq!(e.mem.f32(velocity), 321.0);
        assert_eq!(e.mem.f32(velocity + 4), 654.0);
        assert_eq!(e.mem.f32(velocity + 8), 987.0);
        let turned = calls_to(&e, MATRIX_TIMES_POINT);
        assert_eq!(turned.len(), 1);
        assert_eq!(turned[0][0], orientation);
        assert_eq!(turned[0][2], position);
        assert_eq!(calls_to(&e, FTOL).len(), 2);
    }

    #[test]
    fn save_binary_writes_the_radius_after_the_base() {
        const WRITE: u32 = 0x0100_0020;
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            word(a[2] * a[4])
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 4, 0x30);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = e.mem.alloc(0x5c);
        e.call_log = Some(vec![]);
        e.call(0x00c2_0080, &args![this, stream]);
        assert_eq!(
            calls_to(&e, VOLUME_EMITTER_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        let writes = calls_to(&e, WRITE);
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0][..3], [output, this + RADIUS, 4]);
        assert_eq!(writes[0][4], 1);
        assert_eq!(e.mem.u32(output + 4), 0x34);
    }

    #[test]
    fn get_viewer_strings_appends_the_name_and_the_radius() {
        let mut e = Engine::new();
        e.map(SPHERE_EMITTER_RTTI, 4);
        e.set_global(SPHERE_EMITTER_RTTI, 0x0555_0000u32);
        e.register(VOLUME_EMITTER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0001));
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, _| word(0x7000_0002));
        e.register(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let stored = Rc::new(RefCell::new(vec![]));
        let seen = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            seen.borrow_mut().push((a[1], e.mem.u32(a[2])));
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + RADIUS, 0x7f80_0001);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c2_00c0, &args![this, strings]);
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x0555_0000]]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![vec![RADIUS_NAME, 0x7fc0_0001]]
        );
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 6]]);
        assert_eq!(*stored.borrow(), vec![(1, 0x7000_0001), (2, 0x7000_0002)]);
    }
}
