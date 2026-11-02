//! `NiParticle/nipsyscylinderemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c1fc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCylinderEmitter::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x60 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_cylinder_emitter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x60u32]);
    }
    this
}

// Translated from 00c1fa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCylinderEmitter::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2508.
pub fn ni_p_sys_cylinder_emitter_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2508
}

use crate::units::niparticle::nipsysmodifier::stream_read;
use crate::units::niparticle::nipsysvolumeemitter::NiPSysVolumeEmitter;

layout! {
    /// `NiPSysCylinderEmitter` (Xbox PDB), 0x60 bytes; the PC code uses the
    /// same offsets. Floats are declared as their bit patterns.
    pub struct NiPSysCylinderEmitter: 0x60 {
        /// `m_fEmitterRadius` (Xbox PDB).
        0x58 m_fEmitterRadius: u32,
        /// `m_fEmitterHeight` (Xbox PDB).
        0x5C m_fEmitterHeight: u32,
    }
}

/// The vtable of `NiPSysCylinderEmitter` (RTTI
/// `.?AVNiPSysCylinderEmitter@@`).
const VTABLE: u32 = 0x010b_d8b4;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// The `NiPSysVolumeEmitter` constructor (`00c31af0`, `thiscall`) and the
/// jump to the `NiPSysEmitter` member copy (`00c31b10`, `(dest, process)`).
const VOLUME_EMITTER_CONSTRUCTOR: u32 = 0x00c3_1af0;
const EMITTER_COPY_MEMBERS_THUNK: u32 = 0x00c3_1b10;
/// `NiPSysVolumeEmitter::LoadBinary` (`00c31b20`, one word).
const VOLUME_EMITTER_LOAD_BINARY: u32 = 0x00c3_1b20;
/// `_rand` (`00ecadb8`) and `_ftol2` (`00ec62c0`, the `ST0` value as a
/// leading `f64`).
const RAND: u32 = 0x00ec_adb8;
const FTOL: u32 = 0x00ec_62c0;
/// `double` constants 32767.0 (`RAND_MAX`), 0.5 and 512.0.
const RAND_MAX_DOUBLE: u32 = 0x0102_9790;
const HALF: u32 = 0x0101_1588;
const TABLE_SIZE: u32 = 0x0102_8338;
/// The `float` 2π at `011aea24`.
const TWO_PI: u32 = 0x011a_ea24;
/// The two 512-entry `float` tables read by angle index (`011f52e0` gives
/// the y factor, `011f4ae0` the x factor here).
const TABLE_A: u32 = 0x011f_52e0;
const TABLE_B: u32 = 0x011f_4ae0;
/// `00524c40` (unnamed, `fallout shared/tesobjectweap.cpp`; `thiscall` on
/// the transform, `(result, point)`, answers the result): the transform of
/// a point.
const TRANSFORM_POINT: u32 = 0x0052_4c40;

/// `rand() / 32767.0` rounded to a `float`.
fn random_unit(e: &mut Engine) -> u32 {
    let r = e.call(RAND, &args![]).i32();
    let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
    x87_store32(x87_div(f64::from(r), rand_max))
}

// Translated from 00c1fa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVolumeEmitter::SetEmitterObj` (Xbox PDB): stores the emitter
/// object (no reference count).
pub fn ni_p_sys_volume_emitter_set_emitter_obj(
    e: &mut Engine,
    this: Ptr<NiPSysVolumeEmitter>,
    object: u32,
) {
    e.set(this, NiPSysVolumeEmitter::m_pkEmitterObj, object);
}

// Translated from 00c1fa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCylinderEmitter::ComputeVolumeInitialPositionAndVelocity` (Xbox
/// PDB): `thiscall`, `RET 0xc` (transform, position, velocity). A point in
/// the cylinder: the distance `m_fEmitterRadius * rand()/32767`, the angle
/// `2π * rand()/32767` turned into a table index (`ftol(angle * 512/2π) &
/// 0x1ff`), x = distance times the entry of `011f4ae0`, y = distance times
/// the entry of `011f52e0`, z = `(rand()/32767 - 0.5) * m_fEmitterHeight`;
/// the point goes through the transform (`00524c40`) into `position`, and
/// the velocity is turned by the transform's rotation (its 3x3 matrix,
/// rows times the velocity). The x87 arithmetic is done instruction by
/// instruction in the exe's order.
pub fn ni_p_sys_cylinder_emitter_compute_volume_initial_position_and_velocity(
    e: &mut Engine,
    this: Ptr<NiPSysCylinderEmitter>,
    transform: Ptr,
    position: Ptr,
    velocity: Ptr,
) {
    let unit = random_unit(e);
    let distance = x87_store32(x87_mul(
        x87_load32(e.get(this, NiPSysCylinderEmitter::m_fEmitterRadius)),
        x87_m32(unit),
    ));
    let unit = random_unit(e);
    let two_pi = x87_load32(e.global::<u32>(TWO_PI));
    let angle = x87_store32(x87_mul(two_pi, x87_m32(unit)));
    let table_size = x87_load64(e.global::<u64>(TABLE_SIZE));
    let scale = x87_div(table_size, two_pi);
    let index = e.call(FTOL, &args![x87_mul(scale, x87_m32(angle))]).u32() & 0x1ff;
    let a = x87_store32(x87_load32(e.global::<u32>(TABLE_A + 4 * index)));
    let b = x87_load32(e.global::<u32>(TABLE_B + 4 * index));
    let distance = x87_load32(distance);
    let p = position.addr();
    e.mem.set_u32(p, x87_store32(x87_mul(b, distance)));
    e.mem
        .set_u32(p + 4, x87_store32(x87_mul(distance, x87_m32(a))));
    let unit = random_unit(e);
    let half: f64 = e.global(HALF);
    let z = x87_store32(x87_mul(
        x87_sub(x87_load32(unit), half),
        x87_m32(e.get(this, NiPSysCylinderEmitter::m_fEmitterHeight)),
    ));
    e.mem.set_u32(p + 8, z);
    e.with_stack(0xc, |e, point| {
        let moved = e
            .call(TRANSFORM_POINT, &args![transform, point, position])
            .u32();
        for word in 0..3 {
            let value = e.mem.u32(moved + 4 * word);
            e.mem.set_u32(p + 4 * word, value);
        }
    });
    // Each row: `FLD m[3r+1]; FMUL v[1]; FLD m[3r]; FMUL v[0]; FADDP;
    // FLD m[3r+2]; FMUL v[2]; FADDP; FSTP`.
    let matrix = |e: &Engine, i: u32| x87_load32(e.mem.u32(transform.addr() + 4 * i));
    let v = |e: &Engine, i: u32| x87_m32(e.mem.u32(velocity.addr() + 4 * i));
    let row = |e: &Engine, r: u32| {
        let first = x87_mul(matrix(e, 3 * r + 1), v(e, 1));
        let second = x87_mul(matrix(e, 3 * r), v(e, 0));
        let sum = x87_add(first, second);
        x87_store32(x87_add(sum, x87_mul(matrix(e, 3 * r + 2), v(e, 2))))
    };
    let x = row(e, 0);
    let y = row(e, 1);
    let z = row(e, 2);
    e.mem.set_u32(velocity.addr(), x);
    e.mem.set_u32(velocity.addr() + 4, y);
    e.mem.set_u32(velocity.addr() + 8, z);
}

// Translated from 00c1fba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCylinderEmitter::CreateClone` (Xbox PDB): a 0x60-byte object from
/// `00aa13e0` (constructed: the `NiPSysVolumeEmitter` constructor, the
/// vtable, radius and height 0.0; 0 when the allocation fails), the
/// emitter member copy into it (`00c31b10`; it still runs into a null
/// clone), then the radius and height copied. Returns the clone.
pub fn ni_p_sys_cylinder_emitter_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysCylinderEmitter>,
    process: Ptr,
) -> Ptr<NiPSysCylinderEmitter> {
    let clone = construct(e);
    e.call(EMITTER_COPY_MEMBERS_THUNK, &args![this, clone, process]);
    for field in [
        NiPSysCylinderEmitter::m_fEmitterRadius,
        NiPSysCylinderEmitter::m_fEmitterHeight,
    ] {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(clone, field, value);
    }
    clone
}

/// The allocation and construction `CreateClone` and the stream factory
/// share (0 when the allocation fails).
fn construct(e: &mut Engine) -> Ptr<NiPSysCylinderEmitter> {
    let object = Ptr::<NiPSysCylinderEmitter>::new(e.call(NI_NEW, &args![0x60u32]).u32());
    if object.is_null() {
        return Ptr::NULL;
    }
    e.call(VOLUME_EMITTER_CONSTRUCTOR, &args![object]);
    let zero = 0.0f32.to_bits();
    e.set(object, NiPSysCylinderEmitter::m_fEmitterRadius, zero);
    e.mem.set_u32(object.addr(), VTABLE);
    e.set(object, NiPSysCylinderEmitter::m_fEmitterHeight, zero);
    object
}

// Translated from 00c1fbf0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project (registered through `push
/// 00c1aada`): the `NiPSysCylinderEmitter` stream factory (`cdecl`, no
/// arguments): a 0x60-byte object from `00aa13e0`, constructed (the
/// `NiPSysVolumeEmitter` constructor, the vtable `010bd8b4`, radius and
/// height 0.0); 0 when the allocation fails.
pub fn fn_00c1fbf0(e: &mut Engine) -> Ptr<NiPSysCylinderEmitter> {
    construct(e)
}

// Translated from 00c1fc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCylinderEmitter::LoadBinary` (Xbox PDB):
/// `NiPSysVolumeEmitter::LoadBinary`, then the radius and the height (4
/// bytes each; the component-size word is the stream argument's stack
/// word).
pub fn ni_p_sys_cylinder_emitter_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysCylinderEmitter>,
    stream: Ptr,
) {
    e.call(VOLUME_EMITTER_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, slot| {
        for field in [
            NiPSysCylinderEmitter::m_fEmitterRadius,
            NiPSysCylinderEmitter::m_fEmitterHeight,
        ] {
            stream_read(e, stream.addr(), this.addr() + field.off, 4, 4, slot.addr());
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
/// `NiPSysCylinderEmitter::ms_RTTI` (its name is the first word).
const RTTI: u32 = 0x0120_2508;
/// The labels "Radius" (`01015a9c`) and "Height" (`0109dc84`).
const LABEL_RADIUS: u32 = 0x0101_5a9c;
const LABEL_HEIGHT: u32 = 0x0109_dc84;

// Translated from 00c1f9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCylinderEmitter::IsEqual` (Xbox PDB): `NiPSysVolumeEmitter`'s,
/// then equal radii and heights (`FUCOMPP`: a NaN is unequal).
pub fn ni_p_sys_cylinder_emitter_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysCylinderEmitter>,
    other: Ptr<NiPSysCylinderEmitter>,
) -> bool {
    if !e.call(VOLUME_EMITTER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let ours = x87_m32(e.get(this, NiPSysCylinderEmitter::m_fEmitterRadius));
    let theirs = x87_m32(e.get(other, NiPSysCylinderEmitter::m_fEmitterRadius));
    if theirs != ours {
        return false;
    }
    let ours = x87_m32(e.get(this, NiPSysCylinderEmitter::m_fEmitterHeight));
    let theirs = x87_m32(e.get(other, NiPSysCylinderEmitter::m_fEmitterHeight));
    theirs == ours
}

// Translated from 00c1fca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCylinderEmitter::SaveBinary` (Xbox PDB): `NiPSysVolumeEmitter`'s,
/// then the radius and the height (4 bytes each).
pub fn ni_p_sys_cylinder_emitter_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysCylinderEmitter>,
    stream: Ptr,
) {
    e.call(VOLUME_EMITTER_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, slot| {
        let radius = this.addr() + NiPSysCylinderEmitter::m_fEmitterRadius.off;
        stream_write(e, stream.addr(), radius, 4, 4, slot.addr());
        let height = this.addr() + NiPSysCylinderEmitter::m_fEmitterHeight.off;
        stream_write(e, stream.addr(), height, 4, 4, slot.addr());
    });
}

// Translated from 00c1fd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysCylinderEmitter::GetViewerStrings` (Xbox PDB):
/// `NiPSysVolumeEmitter`'s strings, the class name, the radius and the
/// height.
pub fn ni_p_sys_cylinder_emitter_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysCylinderEmitter>,
    strings: Ptr,
) {
    let strings = strings.addr();
    e.call(VOLUME_EMITTER_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, string);
    let radius = x87_quiet_bits(e.get(this, NiPSysCylinderEmitter::m_fEmitterRadius));
    let string = e
        .call(VIEWER_STRING_FLOAT, &args![LABEL_RADIUS, radius])
        .u32();
    add_viewer_string(e, strings, string);
    let height = x87_quiet_bits(e.get(this, NiPSysCylinderEmitter::m_fEmitterHeight));
    let string = e
        .call(VIEWER_STRING_FLOAT, &args![LABEL_HEIGHT, height])
        .u32();
    add_viewer_string(e, strings, string);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c1_fa30,
            ni_p_sys_volume_emitter_set_emitter_obj(Ptr<NiPSysVolumeEmitter>, u32)
        ),
        entry!(
            0x00c1_fa50,
            ni_p_sys_cylinder_emitter_compute_volume_initial_position_and_velocity(
                Ptr<NiPSysCylinderEmitter>,
                Ptr,
                Ptr,
                Ptr,
            )
        ),
        entry!(
            0x00c1_fba0,
            ni_p_sys_cylinder_emitter_create_clone(
                Ptr<NiPSysCylinderEmitter>,
                Ptr,
            ) -> Ptr<NiPSysCylinderEmitter>
        ),
        entry!(0x00c1_fbf0, fn_00c1fbf0() -> Ptr<NiPSysCylinderEmitter>),
        entry!(
            0x00c1_fc40,
            ni_p_sys_cylinder_emitter_load_binary(Ptr<NiPSysCylinderEmitter>, Ptr)
        ),
        entry!(0x00c1fa40, ni_p_sys_cylinder_emitter_get_r_t_t_i() -> u32),
        entry!(0x00c1fc20, ni_p_sys_cylinder_emitter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c1_f9e0,
            ni_p_sys_cylinder_emitter_is_equal(
                Ptr<NiPSysCylinderEmitter>,
                Ptr<NiPSysCylinderEmitter>,
            ) -> bool
        ),
        entry!(
            0x00c1_fca0,
            ni_p_sys_cylinder_emitter_save_binary(Ptr<NiPSysCylinderEmitter>, Ptr)
        ),
        entry!(
            0x00c1_fd00,
            ni_p_sys_cylinder_emitter_get_viewer_strings(Ptr<NiPSysCylinderEmitter>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_cylinder_emitter_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00c1fc20, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c1fc20, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x60]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_cylinder_emitter_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c1_fa40, &args![]).u32(), 0x0120_2508);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    fn log_of(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn set_emitter_obj_stores_the_object() {
        let mut e = Engine::new();
        let this = e.new_object::<NiPSysVolumeEmitter>();
        e.call(0x00c1_fa30, &args![this, 0x6600u32]);
        assert_eq!(e.get(this, NiPSysVolumeEmitter::m_pkEmitterObj), 0x6600);
    }

    #[test]
    fn volume_position_and_velocity_in_the_cylinder() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x2_0000);
        e.set_global(RAND_MAX_DOUBLE, 32767.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(TABLE_SIZE, 512.0f64);
        e.map(TWO_PI & !0xfff, 0x1000);
        e.set_global(TWO_PI, std::f32::consts::TAU);
        e.map(TABLE_B, 0x1000);
        e.set_global(TABLE_A, 0.5f32);
        e.set_global(TABLE_B, 0.25f32);
        let mut values = [32767u32, 0, 32767].into_iter();
        e.register_double(RAND, move |_, _| Ret {
            eax: values.next().unwrap(),
            ..Ret::default()
        });
        e.register(TRANSFORM_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[2] + 4 * i) + 10.0;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let this = e.new_object::<NiPSysCylinderEmitter>();
        e.set(
            this,
            NiPSysCylinderEmitter::m_fEmitterRadius,
            2.0f32.to_bits(),
        );
        e.set(
            this,
            NiPSysCylinderEmitter::m_fEmitterHeight,
            4.0f32.to_bits(),
        );
        let transform = e.mem.alloc(0x34);
        for i in 0..9 {
            e.mem.set_f32(transform + 4 * i, (i + 1) as f32);
        }
        let position = e.mem.alloc(12);
        let velocity = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_f32(velocity + 4 * i, 1.0);
        }
        e.call_log = Some(vec![]);
        e.call(0x00c1_fa50, &args![this, transform, position, velocity]);
        let point = log_of(&e, TRANSFORM_POINT);
        assert_eq!(point.len(), 1);
        assert_eq!((point[0][0], point[0][2]), (transform, position));
        let f = |e: &Engine, a: u32, i: u32| e.mem.f32(a + 4 * i);
        assert_eq!(
            [f(&e, position, 0), f(&e, position, 1), f(&e, position, 2)],
            [10.5, 11.0, 12.0]
        );
        assert_eq!(
            [f(&e, velocity, 0), f(&e, velocity, 1), f(&e, velocity, 2)],
            [6.0, 15.0, 24.0]
        );
    }

    fn construction_doubles(e: &mut Engine) {
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(VOLUME_EMITTER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(EMITTER_COPY_MEMBERS_THUNK, |_, _| Ret::default());
    }

    #[test]
    fn create_clone_copies_radius_and_height() {
        let mut e = Engine::new();
        construction_doubles(&mut e);
        let this = e.new_object::<NiPSysCylinderEmitter>();
        e.set(this, NiPSysCylinderEmitter::m_fEmitterRadius, 0x7f80_0001);
        e.set(
            this,
            NiPSysCylinderEmitter::m_fEmitterHeight,
            5.0f32.to_bits(),
        );
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c1_fba0, &args![this, 0x4000u32])
            .ptr::<NiPSysCylinderEmitter>();
        assert_eq!(e.mem.u32(clone.addr()), VTABLE);
        assert_eq!(
            log_of(&e, EMITTER_COPY_MEMBERS_THUNK),
            vec![vec![this.addr(), clone.addr(), 0x4000]]
        );
        assert_eq!(
            e.get(clone, NiPSysCylinderEmitter::m_fEmitterRadius),
            0x7fc0_0001
        );
        assert_eq!(
            e.get(clone, NiPSysCylinderEmitter::m_fEmitterHeight),
            5.0f32.to_bits()
        );
    }

    #[test]
    fn factory_constructs_or_answers_null() {
        let mut e = Engine::new();
        construction_doubles(&mut e);
        let made = e.call(0x00c1_fbf0, &args![]).ptr::<NiPSysCylinderEmitter>();
        assert_eq!(e.mem.u32(made.addr()), VTABLE);
        assert_eq!(e.get(made, NiPSysCylinderEmitter::m_fEmitterRadius), 0);
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_fbf0, &args![]).u32(), 0);
        assert!(log_of(&e, VOLUME_EMITTER_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn load_binary_reads_radius_and_height() {
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_LOAD_BINARY, |_, _| Ret::default());
        let bytes: Vec<u8> = [3.0f32, 4.0].iter().flat_map(|v| v.to_le_bytes()).collect();
        let stream = test_stream::new(&mut e, 0x1402_0007, &bytes);
        let this = e.new_object::<NiPSysCylinderEmitter>();
        e.call(0x00c1_fc40, &args![this, stream]);
        assert_eq!(
            e.get(this, NiPSysCylinderEmitter::m_fEmitterRadius),
            3.0f32.to_bits()
        );
        assert_eq!(
            e.get(this, NiPSysCylinderEmitter::m_fEmitterHeight),
            4.0f32.to_bits()
        );
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to};

    fn emitter(e: &mut Engine, radius: f32, height: f32) -> Ptr<NiPSysCylinderEmitter> {
        let this = e.new_object::<NiPSysCylinderEmitter>();
        e.set(
            this,
            NiPSysCylinderEmitter::m_fEmitterRadius,
            radius.to_bits(),
        );
        e.set(
            this,
            NiPSysCylinderEmitter::m_fEmitterHeight,
            height.to_bits(),
        );
        this
    }

    #[test]
    fn is_equal_compares_radius_and_height() {
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_IS_EQUAL, |_, _| answer(1));
        let this = emitter(&mut e, 1.0, 2.0);
        let check = |e: &mut Engine, other| e.call(0x00c1_f9e0, &args![this, other]).bool();
        let same = emitter(&mut e, 1.0, 2.0);
        assert!(check(&mut e, same));
        let radius = emitter(&mut e, 1.5, 2.0);
        assert!(!check(&mut e, radius));
        let height = emitter(&mut e, 1.0, 2.5);
        assert!(!check(&mut e, height));
        let nan = emitter(&mut e, 1.0, f32::NAN);
        e.set(
            this,
            NiPSysCylinderEmitter::m_fEmitterHeight,
            f32::NAN.to_bits(),
        );
        assert!(!check(&mut e, nan));
        e.register(VOLUME_EMITTER_IS_EQUAL, |_, _| answer(0));
        assert!(!check(&mut e, same));
    }

    #[test]
    fn save_binary_writes_radius_then_height() {
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_SAVE_BINARY, |_, _| Ret::default());
        let stream = viewer_test::out_stream(&mut e);
        let this = emitter(&mut e, 1.0, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x00c1_fca0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, VOLUME_EMITTER_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        let mut bytes = 1.0f32.to_le_bytes().to_vec();
        bytes.extend(2.0f32.to_le_bytes());
        assert_eq!(viewer_test::written(&e, stream), (bytes, 8));
    }

    #[test]
    fn viewer_strings_add_the_name_radius_and_height() {
        let mut e = Engine::new();
        e.register(VOLUME_EMITTER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        viewer_test::counting(&mut e, &[VIEWER_STRING, VIEWER_STRING_FLOAT]);
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_0050);
        let strings = viewer_test::strings(&mut e, 8);
        let this = emitter(&mut e, 1.0, 2.0);
        e.set(this, NiPSysCylinderEmitter::m_fEmitterHeight, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00c1_fd00, &args![this, strings]);
        assert_eq!(viewer_test::collected(&e, strings), vec![1, 2, 3]);
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x0100_0050]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FLOAT),
            vec![
                vec![LABEL_RADIUS, 1.0f32.to_bits()],
                vec![LABEL_HEIGHT, 0x7fc0_0001]
            ]
        );
    }
}
