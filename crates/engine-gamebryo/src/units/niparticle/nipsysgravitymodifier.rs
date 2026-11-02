//! `NiParticle/nipsysgravitymodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c22af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2548.
pub fn ni_p_sys_gravity_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2548
}

use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysmodifier::{
    stream_read, NiPSysModifier, READ_LINK_ID, STREAM_VERSION,
};

layout! {
    /// `NiPSysGravityModifier` (Xbox PDB), 0x40 bytes; the PC code uses the
    /// same offsets. Floats are declared as their bit patterns.
    pub struct NiPSysGravityModifier: 0x40 {
        /// `m_pkGravityObj` (Xbox PDB): `NiAVObject*`.
        0x18 m_pkGravityObj: u32,
        /// `m_kGravityAxis` (Xbox PDB): an `NiPoint3`.
        0x1C m_kGravityAxis: u32,
        /// `m_fDecay` (Xbox PDB).
        0x28 m_fDecay: u32,
        /// `m_fStrength` (Xbox PDB).
        0x2C m_fStrength: u32,
        /// `m_eType` (Xbox PDB): 0 and 1 are the two force types `Update`
        /// handles (planar and spherical by their arithmetic).
        0x30 m_eType: u32,
        /// `m_fTurbulence` (Xbox PDB).
        0x34 m_fTurbulence: u32,
        /// `m_fScale` (Xbox PDB).
        0x38 m_fScale: u32,
        /// `m_bWorldAligned` (Xbox PDB).
        0x3C m_bWorldAligned: u8,
    }
}

/// The vtable of `NiPSysGravityModifier` (RTTI
/// `.?AVNiPSysGravityModifier@@`, `010bdff4`).
const VTABLE: u32 = 0x010b_dff4;
/// The axis the constructor gives `m_kGravityAxis` (three `float`s at
/// `011a946c`, copied by `MOV`).
const DEFAULT_AXIS: u32 = 0x011a_946c;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// The `NiPSysModifier` constructor (`00c31720`), member copy (`00c31780`,
/// `(dest, process)`) and `LoadBinary` (`00c318b0`).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
/// `NiColor::LoadBinary` (`00a59be0`, `thiscall` on the colour, the stream):
/// reads the axis (three floats).
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;
/// From file version 10.0.1.14 the turbulence and its scale are stored.
const VERSION_TURBULENCE: u32 = 0x0a00_010e;
/// `NiStream`'s `m_BSStreamHeader.uiVersion` (+4): from Bethesda stream
/// version 21 the world-aligned flag is stored.
const STREAM_BS_VERSION: u32 = 0x4;
const BS_VERSION_WORLD_ALIGNED: u32 = 0x14;

/// `NiTransform::InvertNonUniform` (`004b4880`, `thiscall`, the result), the
/// transform product `0062c250` (`thiscall` on the left operand, `(result,
/// right)`), the matrix-times-vector `004b4500` (`thiscall` on the matrix,
/// `(result, vector)`, answers the result), `004a0c10` (unnamed; normalises
/// the `NiPoint3` it is called on, no result) and
/// `NiPoint3::UnitizeGetLength` (`00457910`, the length in `ST0`).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
const MATRIX_TIMES_VECTOR: u32 = 0x004b_4500;
const UNITIZE: u32 = 0x004a_0c10;
const UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
/// `006397c0` (unnamed, `fallout shared/sky/precipitation.cpp`): a random
/// `float` in `ST0`.
const RANDOM_FLOAT: u32 = 0x0063_97c0;
/// `00ecae6c`: the CRT's x87 `exp` (the argument as a leading `f64`, the
/// result in `ST0`).
const EXP: u32 = 0x00ec_ae6c;
/// `double` constants 1.6 (`3ff99999a0000000`) and 500.0.
const STRENGTH_SCALE: u32 = 0x0106_efa0;
const TURBULENCE_SCALE: u32 = 0x0103_01a8;
/// The 0x34-byte `NiTransform` at +0x68 of the gravity object and of the
/// particle system, and the 3x3 matrix at +0x34 of the gravity object, as
/// the PC code reads them (the world transform and, by its use for a
/// world-aligned axis, the local rotation).
const OBJECT_TRANSFORM: u32 = 0x68;
const OBJECT_MATRIX: u32 = 0x34;
const TRANSFORM_SIZE: u32 = 0x34;

// Translated from 00c22a80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project: the `NiPSysGravityModifier`
/// constructor (`thiscall`; the stream factory `00c22ad0` jumps to it). The
/// `NiPSysModifier` constructor, no gravity object, the vtable `010bdff4`,
/// the axis from `011a946c`, decay 0.0, strength 1.0, type 0, turbulence
/// 0.0 and scale 1.0 (the world-aligned flag is left as it is). Returns
/// `this`.
pub fn fn_00c22a80(e: &mut Engine, this: Ptr<NiPSysGravityModifier>) -> Ptr<NiPSysGravityModifier> {
    e.call(MODIFIER_CONSTRUCTOR, &args![this]);
    e.set(this, NiPSysGravityModifier::m_pkGravityObj, 0);
    e.mem.set_u32(this.addr(), VTABLE);
    for word in 0..3 {
        let value = e.global::<u32>(DEFAULT_AXIS + 4 * word);
        e.mem.set_u32(
            this.addr() + NiPSysGravityModifier::m_kGravityAxis.off + 4 * word,
            value,
        );
    }
    let zero = 0.0f32.to_bits();
    let one = 1.0f32.to_bits();
    e.set(this, NiPSysGravityModifier::m_fDecay, zero);
    e.set(this, NiPSysGravityModifier::m_eType, 0);
    e.set(this, NiPSysGravityModifier::m_fStrength, one);
    e.set(this, NiPSysGravityModifier::m_fTurbulence, zero);
    e.set(this, NiPSysGravityModifier::m_fScale, one);
    this
}

// Translated from 00c22ad0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project (registered through `push
/// 00c1abd0`): the `NiPSysGravityModifier` stream factory (`cdecl`, no
/// arguments): a 0x40-byte object from `00aa13e0` given to the constructor
/// `00c22a80` (a tail jump); 0 when the allocation fails.
pub fn fn_00c22ad0(e: &mut Engine) -> Ptr<NiPSysGravityModifier> {
    let object = Ptr::<NiPSysGravityModifier>::new(e.call(NI_NEW, &args![0x40u32]).u32());
    if object.is_null() {
        return Ptr::NULL;
    }
    fn_00c22a80(e, object)
}

// Translated from 00c22b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityModifier::CreateClone` (Xbox PDB): a 0x40-byte object from
/// `00aa13e0` through the constructor (0 when the allocation fails), the
/// `NiPSysModifier` member copy into it (it still runs into a null clone),
/// then the axis, decay, strength, type, turbulence, scale and the
/// world-aligned flag; the gravity object is not copied. The decay,
/// strength, turbulence and scale go through the x87 stack. Returns the
/// clone.
pub fn ni_p_sys_gravity_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysGravityModifier>,
    process: Ptr,
) -> Ptr<NiPSysGravityModifier> {
    let block = e.call(NI_NEW, &args![0x40u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        fn_00c22a80(e, Ptr::new(block))
    };
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    for word in 0..3 {
        let off = NiPSysGravityModifier::m_kGravityAxis.off + 4 * word;
        let value = e.mem.u32(this.addr() + off);
        e.mem.set_u32(clone.addr() + off, value);
    }
    let copy_float = |e: &mut Engine, field| {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(clone, field, value);
    };
    copy_float(e, NiPSysGravityModifier::m_fDecay);
    copy_float(e, NiPSysGravityModifier::m_fStrength);
    let kind = e.get(this, NiPSysGravityModifier::m_eType);
    e.set(clone, NiPSysGravityModifier::m_eType, kind);
    copy_float(e, NiPSysGravityModifier::m_fTurbulence);
    copy_float(e, NiPSysGravityModifier::m_fScale);
    let aligned = e.get(this, NiPSysGravityModifier::m_bWorldAligned);
    e.set(clone, NiPSysGravityModifier::m_bWorldAligned, aligned);
    clone
}

/// Three floats as their bit patterns.
type Vec3 = [u32; 3];

fn load(bits: u32) -> f64 {
    x87_load32(bits)
}

fn operand(bits: u32) -> f64 {
    x87_m32(bits)
}

fn store(value: f64) -> u32 {
    x87_store32(value)
}

fn read3(e: &Engine, addr: u32) -> Vec3 {
    [e.mem.u32(addr), e.mem.u32(addr + 4), e.mem.u32(addr + 8)]
}

fn write3(e: &mut Engine, addr: u32, v: Vec3) {
    for (i, word) in v.iter().enumerate() {
        e.mem.set_u32(addr + 4 * i as u32, *word);
    }
}

/// `FLD v0; FLD s; FLD ST0; FMULP ST2; FXCH; FSTP; FLD v1; FMUL ST1; FSTP;
/// FMUL v2; FSTP`: the vector scaled by `s`.
fn scale3(v: Vec3, s: u32) -> Vec3 {
    [
        store(x87_mul(load(v[0]), load(s))),
        store(x87_mul(load(v[1]), load(s))),
        store(x87_mul(load(s), operand(v[2]))),
    ]
}

/// `FLD a_i; FADD b_i; FSTP` for each component.
fn add3(a: Vec3, b: Vec3) -> Vec3 {
    [
        store(x87_add(load(a[0]), operand(b[0]))),
        store(x87_add(load(a[1]), operand(b[1]))),
        store(x87_add(load(a[2]), operand(b[2]))),
    ]
}

/// The velocity (at `velocity`) plus `g`, component by component; `order[i]`
/// is true where the code loads the velocity and adds `g` from memory
/// (`FLD vel; FADD g`), false where it loads `g` and adds the velocity.
fn add_to_velocity(e: &mut Engine, velocity: u32, g: Vec3, order: [bool; 3]) {
    for i in 0..3 {
        let slot = velocity + 4 * i as u32;
        let current = e.mem.u32(slot);
        let sum = if order[i] {
            x87_add(load(current), operand(g[i]))
        } else {
            x87_add(load(g[i]), operand(current))
        };
        e.mem.set_u32(slot, store(sum));
    }
}

/// `FLD t_i; FSUB pos_i; FSTP` for each component: the gravity object's
/// position minus the particle's.
fn to_object(object: Vec3, position: Vec3) -> Vec3 {
    [
        store(x87_sub(load(object[0]), operand(position[0]))),
        store(x87_sub(load(object[1]), operand(position[1]))),
        store(x87_sub(load(object[2]), operand(position[2]))),
    ]
}

/// The turbulence vector: three random floats (`006397c0`) times `tf`.
fn turbulence(e: &mut Engine, tf: u32) -> Vec3 {
    let mut t = [0; 3];
    for slot in t.iter_mut() {
        let r = e.call(RANDOM_FLOAT, &args![]).f64();
        *slot = store(x87_mul(r, operand(tf)));
    }
    t
}

/// `exp` of the `float` `x` (stored, reloaded, the result stored and
/// reloaded as the code does): the bits of the result.
fn exp_float(e: &mut Engine, x: u32) -> u32 {
    let result = e.call(EXP, &args![load(x)]).f64();
    x87_quiet_bits(store(result))
}

/// Whether the float is exactly zero (`FLDZ; FCOM m32; TEST AH,0x44; JNP`):
/// a NaN is not.
fn is_zero(bits: u32) -> bool {
    operand(bits) == 0.0
}

// Translated from 00c22b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityModifier::Update` (Xbox PDB): `thiscall`, `RET 8` (time,
/// particle data). With active particles and a gravity object: the
/// transform into the particle system's space (the inverse of the system's
/// transform at +0x68 times the object's) gives the object's position; the
/// axis is turned by the object's matrix at +0x34 (world aligned) or by that
/// transform's rotation and normalised (`004a0c10`); the strength is
/// `m_fStrength * 1.6`. Type 1 (spherical) and type 0 (planar) each have
/// four loops by whether the decay and the turbulence are zero (a NaN is not
/// zero); other types do nothing. Every loop adds to each particle's
/// velocity `force * (time - last update)`:
/// - spherical: the force points at the object (the normalised difference;
///   with both decay and turbulence the code uses the axis instead and the
///   difference only for its length), scaled by `exp(-decay * distance)`
///   with a decay;
/// - planar: the force is along the axis, scaled by `exp(-decay * |d·axis|)`
///   with a decay (the sign of the dot product picks which product goes to
///   `exp`);
/// - with turbulence, three random floats times `scale * turbulence * 500`
///   are added to the force.
///
/// The x87 operations are done instruction by instruction in the exe's
/// order, operand order included; the stack frame keeps the exe's layout
/// (the transform at +0x70, the object's copy at +0xa4, the inverse at
/// +0xd8, the axis at +0x18 and the turned axis at +0x58, relative to the
/// stack pointer the loops use).
pub fn ni_p_sys_gravity_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysGravityModifier>,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    if e.get(data, NiPSysData::m_usActive) == 0 {
        return;
    }
    let object = e.get(this, NiPSysGravityModifier::m_pkGravityObj);
    if object == 0 {
        return;
    }
    e.with_stack(0x10c, |e, frame| {
        let f = frame.addr();
        let bytes = e.mem.bytes(object + OBJECT_TRANSFORM, TRANSFORM_SIZE);
        e.mem.write(f + 0xa4, &bytes);
        let system = e.mem.u32(this.addr() + NiPSysModifier::m_pkTarget.off);
        let bytes = e
            .mem
            .bytes(system.wrapping_add(OBJECT_TRANSFORM), TRANSFORM_SIZE);
        e.mem.write(f + 0x70, &bytes);
        e.call(TRANSFORM_INVERT, &args![f + 0x70, f + 0xd8]);
        e.call(TRANSFORM_MULTIPLY, &args![f + 0xd8, f + 0x70, f + 0xa4]);
        let axis = this.addr() + NiPSysGravityModifier::m_kGravityAxis.off;
        let matrix = if e.get(this, NiPSysGravityModifier::m_bWorldAligned) != 0 {
            e.get(this, NiPSysGravityModifier::m_pkGravityObj) + OBJECT_MATRIX
        } else {
            f + 0x70
        };
        let turned = e
            .call(MATRIX_TIMES_VECTOR, &args![matrix, f + 0x58, axis])
            .u32();
        let direction = read3(e, turned);
        write3(e, f + 0x18, direction);
        e.call(UNITIZE, &args![f + 0x18]);
        let scale: f64 = e.global(STRENGTH_SCALE);
        let strength = store(x87_mul(
            load(e.get(this, NiPSysGravityModifier::m_fStrength)),
            scale,
        ));
        let kind = e.get(this, NiPSysGravityModifier::m_eType);
        let count = e.get(data, NiPSysData::m_usActive);
        let ctx = Gravity {
            this,
            frame: f,
            time: time.to_bits(),
            strength,
            infos: e.get(data, NiPSysData::m_pkParticleInfo),
            positions: e.get(data, NiPSysData::m_pkVertex),
            count,
        };
        match kind {
            1 => spherical(e, &ctx),
            0 => planar(e, &ctx),
            _ => {}
        }
    });
}

/// What every `Update` loop uses.
struct Gravity {
    this: Ptr<NiPSysGravityModifier>,
    /// The stack frame (offsets as the loops' `ESP` sees them).
    frame: u32,
    /// The time, as its bit pattern.
    time: u32,
    /// `m_fStrength * 1.6` (`[ESP+0x30]`).
    strength: u32,
    /// The particle info array (velocity at +0, last update at +0x14) and
    /// the positions (`m_pkVertex`), both read once before the switch.
    infos: u32,
    positions: u32,
    count: u16,
}

impl Gravity {
    fn decay(&self, e: &Engine) -> u32 {
        e.get(self.this, NiPSysGravityModifier::m_fDecay)
    }

    fn turbulence_factor(&self, e: &Engine) -> u32 {
        // `FLD scale; FMUL turbulence; FMUL qword 500.0; FSTP`.
        let factor: f64 = e.global(TURBULENCE_SCALE);
        store(x87_mul(
            x87_mul(
                load(e.get(self.this, NiPSysGravityModifier::m_fScale)),
                operand(e.get(self.this, NiPSysGravityModifier::m_fTurbulence)),
            ),
            factor,
        ))
    }

    /// The gravity object's position in the system's space (`[ESP+0x94]`).
    fn object_position(&self, e: &Engine) -> Vec3 {
        read3(e, self.frame + 0x94)
    }

    /// The normalised axis (`[ESP+0x18]`).
    fn axis(&self, e: &Engine) -> Vec3 {
        read3(e, self.frame + 0x18)
    }

    /// `time - last update` of particle `i`, rounded to a float.
    fn elapsed(&self, e: &Engine, i: u32) -> u32 {
        store(self.elapsed_unrounded(e, i))
    }

    fn elapsed_unrounded(&self, e: &Engine, i: u32) -> f64 {
        let last = e.mem.u32(self.infos + 0x1c * i + 0x14);
        x87_sub(load(self.time), operand(last))
    }

    fn velocity(&self, i: u32) -> u32 {
        self.infos + 0x1c * i
    }

    fn position(&self, e: &Engine, i: u32) -> Vec3 {
        read3(e, self.positions + 0xc * i)
    }
}

/// `exp(-decay * x)` for the planar loops: `x * decay` goes to `exp` when
/// `x` is below zero, `x * -decay` otherwise (a NaN takes the second).
fn planar_falloff(e: &mut Engine, ctx: &Gravity, dot: u32) -> u32 {
    let d = load(dot);
    let product = if d < 0.0 {
        x87_mul(d, operand(ctx.decay(e)))
    } else {
        x87_mul(d, -load(ctx.decay(e)))
    };
    exp_float(e, store(product))
}

/// Type 1: the force points at the gravity object.
fn spherical(e: &mut Engine, ctx: &Gravity) {
    let decay = ctx.decay(e);
    let turbulence_bits = e.get(ctx.this, NiPSysGravityModifier::m_fTurbulence);
    let f = ctx.frame;
    if is_zero(decay) && is_zero(turbulence_bits) {
        for i in 0..u32::from(ctx.count) {
            let to = to_object(ctx.object_position(e), ctx.position(e, i));
            write3(e, f + 0x18, to);
            e.call(UNITIZE, &args![f + 0x18]);
            let k = store(x87_mul(ctx.elapsed_unrounded(e, i), operand(ctx.strength)));
            let g = scale3(ctx.axis(e), k);
            add_to_velocity(e, ctx.velocity(i), g, [true, false, false]);
        }
    } else if is_zero(decay) && !is_zero(turbulence_bits) {
        let tf = ctx.turbulence_factor(e);
        for i in 0..u32::from(ctx.count) {
            let to = to_object(ctx.object_position(e), ctx.position(e, i));
            write3(e, f + 0x18, to);
            e.call(UNITIZE, &args![f + 0x18]);
            let t = turbulence(e, tf);
            let a = scale3(ctx.axis(e), ctx.strength);
            let force = add3(a, t);
            let g = scale3(force, ctx.elapsed(e, i));
            add_to_velocity(e, ctx.velocity(i), g, [false, false, true]);
        }
    } else if !is_zero(decay) && is_zero(turbulence_bits) {
        for i in 0..u32::from(ctx.count) {
            let to = to_object(ctx.object_position(e), ctx.position(e, i));
            write3(e, f + 0x18, to);
            let length = e.call(UNITIZE_GET_LENGTH, &args![f + 0x18]).f64();
            let exponent = store(x87_mul(length, -load(ctx.decay(e))));
            let falloff = exp_float(e, exponent);
            let k = store(x87_mul(
                x87_mul(load(falloff), operand(ctx.strength)),
                ctx.elapsed_unrounded(e, i),
            ));
            let g = scale3(ctx.axis(e), k);
            add_to_velocity(e, ctx.velocity(i), g, [true, false, false]);
        }
    } else if !is_zero(decay) && !is_zero(turbulence_bits) {
        let tf = ctx.turbulence_factor(e);
        let along = scale3(ctx.axis(e), ctx.strength);
        for i in 0..u32::from(ctx.count) {
            let t = turbulence(e, tf);
            let to = to_object(ctx.object_position(e), ctx.position(e, i));
            write3(e, f + 0x58, to);
            let length = e.call(UNITIZE_GET_LENGTH, &args![f + 0x58]).f64();
            let exponent = store(x87_mul(length, -load(ctx.decay(e))));
            let falloff = exp_float(e, exponent);
            let h = scale3(along, falloff);
            let force = add3(h, t);
            let g = scale3(force, ctx.elapsed(e, i));
            // The code keeps the result where the axis was.
            write3(e, f + 0x18, g);
            add_to_velocity(e, ctx.velocity(i), g, [false, false, false]);
        }
    }
}

/// Type 0: the force is along the axis.
fn planar(e: &mut Engine, ctx: &Gravity) {
    let decay = ctx.decay(e);
    let turbulence_bits = e.get(ctx.this, NiPSysGravityModifier::m_fTurbulence);
    if is_zero(decay) && is_zero(turbulence_bits) {
        // The axis and the strength stay on the x87 stack.
        let axis = ctx.axis(e).map(load);
        let strength = load(ctx.strength);
        for i in 0..u32::from(ctx.count) {
            let k = store(x87_mul(ctx.elapsed_unrounded(e, i), strength));
            let k = load(k);
            let g = [
                store(x87_mul(axis[0], k)),
                store(x87_mul(axis[1], k)),
                store(x87_mul(k, axis[2])),
            ];
            add_to_velocity(e, ctx.velocity(i), g, [false, true, false]);
        }
    } else if is_zero(decay) && !is_zero(turbulence_bits) {
        let tf = ctx.turbulence_factor(e);
        let along = scale3(ctx.axis(e), ctx.strength);
        for i in 0..u32::from(ctx.count) {
            let t = turbulence(e, tf);
            let force = add3(along, t);
            let g = scale3(force, ctx.elapsed(e, i));
            add_to_velocity(e, ctx.velocity(i), g, [true, true, false]);
        }
    } else if !is_zero(decay) && is_zero(turbulence_bits) {
        for i in 0..u32::from(ctx.count) {
            // The axis is reloaded from memory each time round and kept on
            // the stack for the dot product and the scaling.
            let axis = ctx.axis(e);
            let a = axis.map(load);
            let d = to_object(ctx.object_position(e), ctx.position(e, i));
            let first = x87_add(x87_mul(a[1], load(d[1])), x87_mul(a[0], operand(d[0])));
            let dot = store(x87_add(x87_mul(a[2], load(d[2])), first));
            let falloff = planar_falloff(e, ctx, dot);
            let k = store(x87_mul(
                ctx.elapsed_unrounded(e, i),
                x87_mul(load(falloff), operand(ctx.strength)),
            ));
            let k = load(k);
            let g = [
                store(x87_mul(k, a[0])),
                store(x87_mul(a[1], k)),
                store(x87_mul(k, a[2])),
            ];
            add_to_velocity(e, ctx.velocity(i), g, [true, true, false]);
        }
    } else if !is_zero(decay) && !is_zero(turbulence_bits) {
        let tf = ctx.turbulence_factor(e);
        let along = scale3(ctx.axis(e), ctx.strength);
        for i in 0..u32::from(ctx.count) {
            let t = turbulence(e, tf);
            let d = to_object(ctx.object_position(e), ctx.position(e, i));
            let axis = ctx.axis(e);
            let first = x87_add(
                x87_mul(load(d[1]), operand(axis[1])),
                x87_mul(load(d[0]), operand(axis[0])),
            );
            let dot = store(x87_add(first, x87_mul(load(d[2]), operand(axis[2]))));
            let falloff = planar_falloff(e, ctx, dot);
            let h = scale3(along, falloff);
            let force = add3(h, t);
            let g = scale3(force, ctx.elapsed(e, i));
            add_to_velocity(e, ctx.velocity(i), g, [true, false, false]);
        }
    }
}

// Translated from 00c236b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, a link id (`00a645b0`, the gravity object), the axis
/// (`NiColor::LoadBinary`), decay, strength and the type (4 bytes read into
/// a local, then stored); from file version 10.0.1.14 the turbulence and
/// its scale; from Bethesda stream version 21 the world-aligned flag (one
/// byte). The component-size word is the stream argument's stack word.
pub fn ni_p_sys_gravity_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysGravityModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_LOAD_BINARY, &args![this, s]);
    e.call(READ_LINK_ID, &args![s]);
    e.call(
        COLOR_LOAD_BINARY,
        &args![this.addr() + NiPSysGravityModifier::m_kGravityAxis.off, s],
    );
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument_slot = frame.addr() + 4;
        e.mem.set_u32(argument_slot, s);
        let field = |off: u32| this.addr() + off;
        stream_read(
            e,
            s,
            field(NiPSysGravityModifier::m_fDecay.off),
            4,
            4,
            argument_slot,
        );
        stream_read(
            e,
            s,
            field(NiPSysGravityModifier::m_fStrength.off),
            4,
            4,
            argument_slot,
        );
        stream_read(e, s, local, 4, 4, argument_slot);
        let kind = e.mem.u32(local);
        e.set(this, NiPSysGravityModifier::m_eType, kind);
        if e.mem.u32(s + STREAM_VERSION) >= VERSION_TURBULENCE {
            stream_read(
                e,
                s,
                field(NiPSysGravityModifier::m_fTurbulence.off),
                4,
                4,
                argument_slot,
            );
            stream_read(
                e,
                s,
                field(NiPSysGravityModifier::m_fScale.off),
                4,
                4,
                argument_slot,
            );
        }
        if e.mem.u32(s + STREAM_BS_VERSION) > BS_VERSION_WORLD_ALIGNED {
            stream_read(
                e,
                s,
                field(NiPSysGravityModifier::m_bWorldAligned.off),
                1,
                1,
                argument_slot,
            );
        }
    });
}

use crate::units::niparticle::nipsysmodifier::{
    add_class_name, add_labelled, add_viewer_string, stream_write, NI_GET_VIEWER_STRING_BOOL,
    NI_GET_VIEWER_STRING_FIXED, NI_GET_VIEWER_STRING_FLOAT, STREAM_SAVE_LINK_ID,
};

/// `NiPSysModifier::IsEqual` (`00c31830`), `SaveBinary` (`00c31950`) and
/// `GetViewerStrings` (`00c319e0`): `thiscall`, one word.
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// The point helpers the code calls on the axis (`thiscall`, one word):
/// `00a69780` writes its three floats to a stream (the engine map names it
/// after `NiColor`, whose body it shares), `00439090` answers whether it
/// differs from another point, `00a59af0` (`NiPoint3::GetViewerString`)
/// makes its viewer string with a label.
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
const POINT_NOT_EQUAL: u32 = 0x0043_9090;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// The `NiRTTI` record of `NiPSysGravityModifier` (`01202548`).
const RTTI: u32 = 0x0120_2548;
/// Labels: `"Gravity Object"`, `"Gravity Axis"`, `"Decay"`, `"Strength"`,
/// `"Turbulence"`, `"TurbulenceScale"`, `"Force Type"`, `"WorldAligned"`;
/// the force type names `"FORCE_PLANAR"` (0), `"FORCE_SPHERICAL"` (1),
/// `"Unknown"`; and `"None"` (the object's name when there is none).
const LABEL_GRAVITY_OBJECT: u32 = 0x010b_e108;
const LABEL_GRAVITY_AXIS: u32 = 0x010b_e0f8;
const LABEL_DECAY: u32 = 0x010b_e0f0;
const LABEL_STRENGTH: u32 = 0x0106_78ac;
const LABEL_TURBULENCE: u32 = 0x010b_e0e4;
const LABEL_TURBULENCE_SCALE: u32 = 0x010b_e0d4;
const LABEL_FORCE_TYPE: u32 = 0x010b_e0a8;
const LABEL_WORLD_ALIGNED: u32 = 0x010b_e098;
const TEXT_FORCE_PLANAR: u32 = 0x010b_e0c4;
const TEXT_FORCE_SPHERICAL: u32 = 0x010b_e0b4;
const TEXT_UNKNOWN: u32 = 0x0106_f148;
const TEXT_NONE: u32 = 0x0102_909c;

// Translated from 00c235f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityModifier::IsEqual` (Xbox PDB): `NiPSysModifier::IsEqual`;
/// both or neither must have a gravity object, two objects must be equal
/// (the first's `IsEqual`, slot +0x5c); the axes must not differ
/// (`00439090` on the other's axis with this one's); then equal decays and
/// strengths (`FUCOMPP`: a NaN is unequal), the same force type, equal
/// turbulences and scales, and the same world-aligned byte.
pub fn ni_p_sys_gravity_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysGravityModifier>,
    other: Ptr<NiPSysGravityModifier>,
) -> bool {
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = e.get(this, NiPSysGravityModifier::m_pkGravityObj);
    let theirs = e.get(other, NiPSysGravityModifier::m_pkGravityObj);
    if (mine != 0) != (theirs != 0) {
        return false;
    }
    if mine != 0 && !e.vcall(mine, 0x5c, &args![theirs]).bool() {
        return false;
    }
    let axis = NiPSysGravityModifier::m_kGravityAxis.off;
    if e.call(
        POINT_NOT_EQUAL,
        &args![other.addr() + axis, this.addr() + axis],
    )
    .bool()
    {
        return false;
    }
    let same_float =
        |e: &Engine, field| x87_load32(e.get(other, field)) == x87_load32(e.get(this, field));
    same_float(e, NiPSysGravityModifier::m_fDecay)
        && same_float(e, NiPSysGravityModifier::m_fStrength)
        && e.get(other, NiPSysGravityModifier::m_eType)
            == e.get(this, NiPSysGravityModifier::m_eType)
        && same_float(e, NiPSysGravityModifier::m_fTurbulence)
        && same_float(e, NiPSysGravityModifier::m_fScale)
        && e.get(other, NiPSysGravityModifier::m_bWorldAligned)
            == e.get(this, NiPSysGravityModifier::m_bWorldAligned)
}

// Translated from 00c237c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityModifier::SaveBinary` (Xbox PDB): `NiPSysModifier`'s
/// `SaveBinary`, the gravity object's link id (stream slot +0x30), the axis
/// (`00a69780`), then decay, strength, force type, turbulence and scale (4
/// bytes each) and the world-aligned byte; every component size is kept in
/// the stack word that held the stream argument.
pub fn ni_p_sys_gravity_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysGravityModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_SAVE_BINARY, &args![this, s]);
    let object = e.get(this, NiPSysGravityModifier::m_pkGravityObj);
    e.vcall(s, STREAM_SAVE_LINK_ID, &args![object]);
    let axis = this.addr() + NiPSysGravityModifier::m_kGravityAxis.off;
    e.call(POINT_SAVE_BINARY, &args![axis, s]);
    e.with_stack(4, |e, argument_slot| {
        e.mem.set_u32(argument_slot.addr(), s);
        for field in [
            NiPSysGravityModifier::m_fDecay,
            NiPSysGravityModifier::m_fStrength,
            NiPSysGravityModifier::m_eType,
            NiPSysGravityModifier::m_fTurbulence,
            NiPSysGravityModifier::m_fScale,
        ] {
            stream_write(e, s, this.addr() + field.off, 4, 4, argument_slot.addr());
        }
        let aligned = this.addr() + NiPSysGravityModifier::m_bWorldAligned.off;
        stream_write(e, s, aligned, 1, 1, argument_slot.addr());
    });
}

// Translated from 00c238c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityModifier::GetViewerStrings` (Xbox PDB):
/// `NiPSysModifier::GetViewerStrings`, then the class name (`NiRTTI`
/// `01202548`), `"Gravity Object"` (the object's name handle, or `"None"`),
/// the axis's string with `"Gravity Axis"` (`NiPoint3::GetViewerString`),
/// `"Decay"`, `"Strength"`, `"Turbulence"`, `"TurbulenceScale"` (the scale),
/// `"Force Type"` (`"FORCE_PLANAR"` for 0, `"FORCE_SPHERICAL"` for 1, else
/// `"Unknown"`) and `"WorldAligned"` appended (the floats pushed through
/// `FLD`/`FSTP`).
pub fn ni_p_sys_gravity_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysGravityModifier>,
    strings: Ptr,
) {
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    add_class_name(e, strings, RTTI);
    let object = e.get(this, NiPSysGravityModifier::m_pkGravityObj);
    let name = if object == 0 {
        TEXT_NONE
    } else {
        e.mem.u32(object + 8)
    };
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_FIXED,
        LABEL_GRAVITY_OBJECT,
        name,
    );
    let axis = this.addr() + NiPSysGravityModifier::m_kGravityAxis.off;
    let text = e
        .call(POINT_GET_VIEWER_STRING, &args![axis, LABEL_GRAVITY_AXIS])
        .u32();
    add_viewer_string(e, strings, text);
    for (field, label) in [
        (NiPSysGravityModifier::m_fDecay, LABEL_DECAY),
        (NiPSysGravityModifier::m_fStrength, LABEL_STRENGTH),
        (NiPSysGravityModifier::m_fTurbulence, LABEL_TURBULENCE),
        (NiPSysGravityModifier::m_fScale, LABEL_TURBULENCE_SCALE),
    ] {
        let value = x87_quiet_bits(e.get(this, field));
        add_labelled(e, strings, NI_GET_VIEWER_STRING_FLOAT, label, value);
    }
    let force = match e.get(this, NiPSysGravityModifier::m_eType) {
        0 => TEXT_FORCE_PLANAR,
        1 => TEXT_FORCE_SPHERICAL,
        _ => TEXT_UNKNOWN,
    };
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_FIXED,
        LABEL_FORCE_TYPE,
        force,
    );
    let aligned = u32::from(e.get(this, NiPSysGravityModifier::m_bWorldAligned));
    add_labelled(
        e,
        strings,
        NI_GET_VIEWER_STRING_BOOL,
        LABEL_WORLD_ALIGNED,
        aligned,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_35f0,
            ni_p_sys_gravity_modifier_is_equal(
                Ptr<NiPSysGravityModifier>,
                Ptr<NiPSysGravityModifier>,
            ) -> bool
        ),
        entry!(
            0x00c2_37c0,
            ni_p_sys_gravity_modifier_save_binary(Ptr<NiPSysGravityModifier>, Ptr)
        ),
        entry!(
            0x00c2_38c0,
            ni_p_sys_gravity_modifier_get_viewer_strings(Ptr<NiPSysGravityModifier>, Ptr)
        ),
        entry!(
            0x00c2_2a80,
            fn_00c22a80(Ptr<NiPSysGravityModifier>) -> Ptr<NiPSysGravityModifier>
        ),
        entry!(0x00c2_2ad0, fn_00c22ad0() -> Ptr<NiPSysGravityModifier>),
        entry!(
            0x00c2_2b00,
            ni_p_sys_gravity_modifier_create_clone(
                Ptr<NiPSysGravityModifier>,
                Ptr,
            ) -> Ptr<NiPSysGravityModifier>
        ),
        entry!(
            0x00c2_2b70,
            ni_p_sys_gravity_modifier_update(Ptr<NiPSysGravityModifier>, f32, Ptr<NiPSysData>)
        ),
        entry!(
            0x00c2_36b0,
            ni_p_sys_gravity_modifier_load_binary(Ptr<NiPSysGravityModifier>, Ptr)
        ),
        entry!(0x00c22af0, ni_p_sys_gravity_modifier_get_r_t_t_i() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_gravity_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_2af0, &args![]).u32(), 0x0120_2548);
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

    fn construction_doubles(e: &mut Engine) {
        e.map(DEFAULT_AXIS & !0xfff, 0x1000);
        e.set_global(DEFAULT_AXIS + 8, 1.0f32);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
    }

    #[test]
    fn factory_and_constructor_set_the_defaults() {
        let mut e = Engine::new();
        construction_doubles(&mut e);
        let made = e.call(0x00c2_2ad0, &args![]).ptr::<NiPSysGravityModifier>();
        assert_eq!(e.mem.u32(made.addr()), VTABLE);
        let f = |e: &Engine, field| f32::from_bits(e.get(made, field));
        assert_eq!(f(&e, NiPSysGravityModifier::m_fStrength), 1.0);
        assert_eq!(f(&e, NiPSysGravityModifier::m_fScale), 1.0);
        assert_eq!(f(&e, NiPSysGravityModifier::m_fDecay), 0.0);
        assert_eq!(e.mem.f32(made.addr() + 0x24), 1.0);
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_2ad0, &args![]).u32(), 0);
        assert!(log_of(&e, MODIFIER_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn create_clone_copies_all_but_the_gravity_object() {
        let mut e = Engine::new();
        construction_doubles(&mut e);
        let this = e.new_object::<NiPSysGravityModifier>();
        e.set(this, NiPSysGravityModifier::m_pkGravityObj, 0x6000);
        e.set(this, NiPSysGravityModifier::m_fDecay, 0x7f80_0001);
        e.set(this, NiPSysGravityModifier::m_eType, 1);
        e.set(this, NiPSysGravityModifier::m_fScale, 3.0f32.to_bits());
        e.set(this, NiPSysGravityModifier::m_bWorldAligned, 1);
        e.mem.set_f32(this.addr() + 0x20, 7.0);
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c2_2b00, &args![this, 0x4000u32])
            .ptr::<NiPSysGravityModifier>();
        assert_eq!(
            log_of(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x4000]]
        );
        assert_eq!(e.get(clone, NiPSysGravityModifier::m_pkGravityObj), 0);
        assert_eq!(e.get(clone, NiPSysGravityModifier::m_fDecay), 0x7fc0_0001);
        assert_eq!(e.get(clone, NiPSysGravityModifier::m_eType), 1);
        assert_eq!(
            e.get(clone, NiPSysGravityModifier::m_fScale),
            3.0f32.to_bits()
        );
        assert_eq!(e.get(clone, NiPSysGravityModifier::m_bWorldAligned), 1);
        assert_eq!(e.mem.f32(clone.addr() + 0x20), 7.0);
    }

    /// A gravity modifier over one particle: the transform product puts the
    /// object at `object`, the axis comes back turned as it is, the
    /// normalisations are done in `f64`, `exp` and the random floats are the
    /// host's `exp` and 0.5.
    struct Scene {
        e: Engine,
        this: Ptr<NiPSysGravityModifier>,
        data: Ptr<NiPSysData>,
    }

    fn scene(kind: u32, axis: [f32; 3], object: [f32; 3], position: [f32; 3]) -> Scene {
        let mut e = Engine::new();
        e.map(STRENGTH_SCALE & !0xfff, 0x1000);
        e.set_global(STRENGTH_SCALE, f64::from(1.6f32));
        e.map(TURBULENCE_SCALE & !0xfff, 0x1000);
        e.set_global(TURBULENCE_SCALE, 500.0f64);
        e.register(TRANSFORM_INVERT, |_, _| Ret::default());
        let object_bits: Vec<u8> = object.iter().flat_map(|v| v.to_le_bytes()).collect();
        e.register_double(TRANSFORM_MULTIPLY, move |e, a| {
            e.mem.write(a[1] + 0x24, &object_bits);
            Ret::default()
        });
        e.register(MATRIX_TIMES_VECTOR, |e, a| {
            let v = e.mem.bytes(a[2], 12);
            e.mem.write(a[1], &v);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        fn normalize(e: &mut Engine, p: u32) -> f64 {
            let v: Vec<f64> = (0..3).map(|i| f64::from(e.mem.f32(p + 4 * i))).collect();
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (i, c) in v.iter().enumerate() {
                e.mem.set_f32(p + 4 * i as u32, (c / length) as f32);
            }
            length
        }
        e.register(UNITIZE, |e, a| {
            normalize(e, a[0]);
            Ret::default()
        });
        e.register(UNITIZE_GET_LENGTH, |e, a| Ret {
            st0: normalize(e, a[0]),
            ..Ret::default()
        });
        e.register(RANDOM_FLOAT, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(EXP, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).exp(),
            ..Ret::default()
        });
        let this = e.new_object::<NiPSysGravityModifier>();
        let gravity_object = e.mem.alloc(0xa0);
        let system = e.mem.alloc(0xa0);
        e.mem.set_u32(this.addr() + 0x10, system);
        e.set(this, NiPSysGravityModifier::m_pkGravityObj, gravity_object);
        e.set(this, NiPSysGravityModifier::m_eType, kind);
        e.set(this, NiPSysGravityModifier::m_fStrength, 1.0f32.to_bits());
        e.set(this, NiPSysGravityModifier::m_fScale, 1.0f32.to_bits());
        for (i, v) in axis.iter().enumerate() {
            e.mem.set_f32(this.addr() + 0x1c + 4 * i as u32, *v);
        }
        let data = e.new_object::<NiPSysData>();
        let positions = e.mem.alloc(12);
        for (i, v) in position.iter().enumerate() {
            e.mem.set_f32(positions + 4 * i as u32, *v);
        }
        let info = e.mem.alloc(0x1c);
        e.set(data, NiPSysData::m_pkVertex, positions);
        e.set(data, NiPSysData::m_pkParticleInfo, info);
        e.set(data, NiPSysData::m_usActive, 1);
        Scene { e, this, data }
    }

    impl Scene {
        fn velocity(&self) -> [f32; 3] {
            let info = self.e.get(self.data, NiPSysData::m_pkParticleInfo);
            [0, 4, 8].map(|o| self.e.mem.f32(info + o))
        }

        fn update(&mut self, time: f32) {
            let (this, data) = (self.this, self.data);
            self.e.call(0x00c2_2b70, &args![this, time, data]);
        }
    }

    const S: f32 = 1.6;

    #[test]
    fn update_needs_particles_and_a_gravity_object() {
        let mut s = scene(1, [0.0, 0.0, 1.0], [0.0, 0.0, 10.0], [0.0; 3]);
        s.e.set(s.data, NiPSysData::m_usActive, 0);
        s.e.call_log = Some(vec![]);
        s.update(1.0);
        assert!(log_of(&s.e, TRANSFORM_INVERT).is_empty());
        s.e.set(s.data, NiPSysData::m_usActive, 1);
        s.e.set(s.this, NiPSysGravityModifier::m_pkGravityObj, 0);
        s.update(1.0);
        assert!(log_of(&s.e, TRANSFORM_INVERT).is_empty());
        // Other types compute the axis and stop.
        let mut s = scene(2, [0.0, 0.0, 1.0], [0.0, 0.0, 10.0], [0.0; 3]);
        s.update(1.0);
        assert_eq!(s.velocity(), [0.0; 3]);
    }

    #[test]
    fn spherical_pulls_toward_the_object() {
        let mut s = scene(1, [1.0, 0.0, 0.0], [0.0, 0.0, 10.0], [0.0; 3]);
        s.e.call_log = Some(vec![]);
        s.update(1.0);
        assert_eq!(s.velocity(), [0.0, 0.0, S]);
        assert_eq!(log_of(&s.e, UNITIZE).len(), 2);
    }

    #[test]
    fn spherical_with_decay_falls_off_with_distance() {
        let mut s = scene(1, [1.0, 0.0, 0.0], [0.0, 0.0, 4.0], [0.0; 3]);
        s.e.set(s.this, NiPSysGravityModifier::m_fDecay, 0.5f32.to_bits());
        s.update(1.0);
        let falloff = (-2.0f64).exp() as f32;
        let k = (f64::from(falloff) * f64::from(S)) as f32;
        assert_eq!(s.velocity(), [0.0, 0.0, k]);
    }

    #[test]
    fn spherical_with_decay_and_turbulence_uses_the_axis() {
        let mut s = scene(1, [1.0, 0.0, 0.0], [0.0, 0.0, 4.0], [0.0; 3]);
        s.e.set(s.this, NiPSysGravityModifier::m_fDecay, 0.5f32.to_bits());
        s.e.set(
            s.this,
            NiPSysGravityModifier::m_fTurbulence,
            0.002f32.to_bits(),
        );
        s.update(1.0);
        let falloff = (-2.0f64).exp() as f32;
        let along = (f64::from(S) * f64::from(falloff)) as f32;
        let tf = (0.002f64 * 500.0) as f32;
        let t = (0.5 * f64::from(tf)) as f32;
        let v = s.velocity();
        assert_eq!(v[0], (f64::from(along) + f64::from(t)) as f32);
        assert_eq!(v[1], t);
        assert_eq!(v[2], t);
    }

    #[test]
    fn planar_pushes_along_the_axis() {
        let mut s = scene(0, [0.0, 2.0, 0.0], [5.0, 5.0, 5.0], [0.0; 3]);
        s.update(0.5);
        assert_eq!(s.velocity(), [0.0, 0.8, 0.0]);
    }

    #[test]
    fn planar_turbulence_adds_random_pushes() {
        let mut s = scene(0, [0.0, 1.0, 0.0], [5.0, 5.0, 5.0], [0.0; 3]);
        s.e.set(
            s.this,
            NiPSysGravityModifier::m_fTurbulence,
            0.002f32.to_bits(),
        );
        s.update(1.0);
        let tf = (0.002f64 * 500.0) as f32;
        let t = (0.5 * f64::from(tf)) as f32;
        let v = s.velocity();
        assert_eq!(v[0], t);
        assert_eq!(v[1], (f64::from(S) + f64::from(t)) as f32);
    }

    #[test]
    fn planar_decay_uses_the_distance_along_the_axis_either_side() {
        for object in [[0.0f32, 4.0, 0.0], [0.0, -4.0, 0.0]] {
            let mut s = scene(0, [0.0, 1.0, 0.0], object, [0.0; 3]);
            s.e.set(s.this, NiPSysGravityModifier::m_fDecay, 0.5f32.to_bits());
            s.e.call_log = Some(vec![]);
            s.update(1.0);
            let exp = log_of(&s.e, EXP);
            assert_eq!(exp.len(), 1);
            let argument = f64::from_bits(u64::from(exp[0][0]) | u64::from(exp[0][1]) << 32);
            assert_eq!(argument, -2.0);
            let falloff = (-2.0f64).exp() as f32;
            let k = (f64::from(falloff) * f64::from(S)) as f32;
            assert_eq!(s.velocity(), [0.0, k, 0.0]);
        }
        // With turbulence too.
        let mut s = scene(0, [0.0, 1.0, 0.0], [0.0, 4.0, 0.0], [0.0; 3]);
        s.e.set(s.this, NiPSysGravityModifier::m_fDecay, 0.5f32.to_bits());
        s.e.set(
            s.this,
            NiPSysGravityModifier::m_fTurbulence,
            0.002f32.to_bits(),
        );
        s.update(1.0);
        let falloff = (-2.0f64).exp() as f32;
        let along = (f64::from(S) * f64::from(falloff)) as f32;
        let tf = (0.002f64 * 500.0) as f32;
        let t = (0.5 * f64::from(tf)) as f32;
        assert_eq!(s.velocity()[1], (f64::from(along) + f64::from(t)) as f32);
    }

    #[test]
    fn spherical_turbulence_only() {
        let mut s = scene(1, [1.0, 0.0, 0.0], [0.0, 0.0, 10.0], [0.0; 3]);
        s.e.set(
            s.this,
            NiPSysGravityModifier::m_fTurbulence,
            0.002f32.to_bits(),
        );
        s.update(1.0);
        let tf = (0.002f64 * 500.0) as f32;
        let t = (0.5 * f64::from(tf)) as f32;
        let v = s.velocity();
        assert_eq!(v[0], t);
        assert_eq!(v[2], (f64::from(S) + f64::from(t)) as f32);
    }

    #[test]
    fn world_aligned_turns_the_axis_by_the_object_matrix() {
        let mut s = scene(0, [0.0, 1.0, 0.0], [5.0, 5.0, 5.0], [0.0; 3]);
        s.e.set(s.this, NiPSysGravityModifier::m_bWorldAligned, 1);
        s.e.call_log = Some(vec![]);
        s.update(1.0);
        let object = s.e.get(s.this, NiPSysGravityModifier::m_pkGravityObj);
        let turn = log_of(&s.e, MATRIX_TIMES_VECTOR);
        assert_eq!(turn[0][0], object + 0x34);
        assert_eq!(turn[0][2], s.this.addr() + 0x1c);
    }

    #[test]
    fn load_binary_reads_by_version() {
        for (version, bs_version, turbulence, aligned) in [
            (0x0a00_010eu32, 0x15u32, true, true),
            (0x0a00_010d, 0x14, false, false),
        ] {
            let mut e = Engine::new();
            e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
            e.register(READ_LINK_ID, |_, _| Ret::default());
            e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
            let mut bytes: Vec<u8> = vec![];
            bytes.extend_from_slice(&0.5f32.to_le_bytes());
            bytes.extend_from_slice(&2.0f32.to_le_bytes());
            bytes.extend_from_slice(&1u32.to_le_bytes());
            if turbulence {
                bytes.extend_from_slice(&3.0f32.to_le_bytes());
                bytes.extend_from_slice(&4.0f32.to_le_bytes());
            }
            if aligned {
                bytes.push(1);
            }
            let stream = test_stream::new(&mut e, version, &bytes);
            e.mem.set_u32(stream + STREAM_BS_VERSION, bs_version);
            let this = e.new_object::<NiPSysGravityModifier>();
            e.call_log = Some(vec![]);
            e.call(0x00c2_36b0, &args![this, stream]);
            assert_eq!(
                log_of(&e, COLOR_LOAD_BINARY),
                vec![vec![this.addr() + 0x1c, stream]]
            );
            assert_eq!(log_of(&e, READ_LINK_ID), vec![vec![stream]]);
            let f = |e: &Engine, field| f32::from_bits(e.get(this, field));
            assert_eq!(f(&e, NiPSysGravityModifier::m_fDecay), 0.5);
            assert_eq!(f(&e, NiPSysGravityModifier::m_fStrength), 2.0);
            assert_eq!(e.get(this, NiPSysGravityModifier::m_eType), 1);
            let expected_turbulence = if turbulence { 3.0 } else { 0.0 };
            assert_eq!(
                f(&e, NiPSysGravityModifier::m_fTurbulence),
                expected_turbulence
            );
            assert_eq!(
                e.get(this, NiPSysGravityModifier::m_bWorldAligned),
                aligned as u8
            );
            assert_eq!(test_stream::position(&e, stream), bytes.len() as u32);
        }
    }
}

#[cfg(test)]
mod stream_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    fn gravity(e: &mut Engine) -> Ptr<NiPSysGravityModifier> {
        let g = e.new_object::<NiPSysGravityModifier>();
        e.set(g, NiPSysGravityModifier::m_fDecay, 0.5f32.to_bits());
        e.set(g, NiPSysGravityModifier::m_fStrength, 2.0f32.to_bits());
        e.set(g, NiPSysGravityModifier::m_eType, 1);
        e.set(g, NiPSysGravityModifier::m_fTurbulence, 3.0f32.to_bits());
        e.set(g, NiPSysGravityModifier::m_fScale, 4.0f32.to_bits());
        e.set(g, NiPSysGravityModifier::m_bWorldAligned, 1);
        g
    }

    #[test]
    fn is_equal_checks_object_axis_and_fields() {
        const OBJECT_IS_EQUAL: u32 = 0x0f60_0000;
        let mut e = Engine::new();
        e.register(MODIFIER_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(POINT_NOT_EQUAL, |_, _| Ret::default());
        e.register(OBJECT_IS_EQUAL, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0] + 4) == e.mem.u32(a[1] + 4)),
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + 0x5c, OBJECT_IS_EQUAL);
        let a = gravity(&mut e);
        let b = gravity(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c2_35f0, &args![a, b]).bool());
        assert_eq!(
            calls_to(&e, POINT_NOT_EQUAL),
            vec![vec![b.addr() + 0x1c, a.addr() + 0x1c]]
        );
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.set(a, NiPSysGravityModifier::m_pkGravityObj, object);
        assert!(!e.call(0x00c2_35f0, &args![a, b]).bool());
        e.set(b, NiPSysGravityModifier::m_pkGravityObj, object);
        assert!(e.call(0x00c2_35f0, &args![a, b]).bool());
        for (off, word) in [
            (0x28u32, 0x7fc0_0000u32),
            (0x2c, 1.0f32.to_bits()),
            (0x30, 0),
            (0x34, 1.0f32.to_bits()),
            (0x38, 1.0f32.to_bits()),
        ] {
            let c = gravity(&mut e);
            e.set(c, NiPSysGravityModifier::m_pkGravityObj, object);
            e.mem.set_u32(c.addr() + off, word);
            assert!(!e.call(0x00c2_35f0, &args![a, c]).bool(), "{off:#x}");
        }
        let c = gravity(&mut e);
        e.set(c, NiPSysGravityModifier::m_pkGravityObj, object);
        e.set(c, NiPSysGravityModifier::m_bWorldAligned, 0);
        assert!(!e.call(0x00c2_35f0, &args![a, c]).bool());
        e.register(POINT_NOT_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(!e.call(0x00c2_35f0, &args![a, b]).bool());
        e.register(MODIFIER_IS_EQUAL, |_, _| Ret::default());
        assert!(!e.call(0x00c2_35f0, &args![a, b]).bool());
    }

    #[test]
    fn save_binary_writes_link_axis_and_fields() {
        const SAVE_LINK: u32 = 0x0f60_0010;
        let mut e = Engine::new();
        e.register(MODIFIER_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_LINK, |_, _| Ret::default());
        e.register(POINT_SAVE_BINARY, |_, _| Ret::default());
        let writes = test_stream::writes(&mut e);
        let stream = test_stream::new_output(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(vtable + STREAM_SAVE_LINK_ID, SAVE_LINK);
        let this = gravity(&mut e);
        e.set(this, NiPSysGravityModifier::m_pkGravityObj, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x00c2_37c0, &args![this, stream]);
        assert_eq!(calls_to(&e, SAVE_LINK), vec![vec![stream, 0x7700]]);
        assert_eq!(
            calls_to(&e, POINT_SAVE_BINARY),
            vec![vec![this.addr() + 0x1c, stream]]
        );
        assert_eq!(
            *writes.borrow(),
            vec![
                (0.5f32.to_le_bytes().to_vec(), 4),
                (2.0f32.to_le_bytes().to_vec(), 4),
                (vec![1, 0, 0, 0], 4),
                (3.0f32.to_le_bytes().to_vec(), 4),
                (4.0f32.to_le_bytes().to_vec(), 4),
                (vec![1], 1),
            ]
        );
    }

    #[test]
    fn get_viewer_strings_names_the_force_type() {
        for (force, text) in [
            (0u32, TEXT_FORCE_PLANAR),
            (1, TEXT_FORCE_SPHERICAL),
            (2, TEXT_UNKNOWN),
        ] {
            let mut e = Engine::new();
            test_stream::global_word(&mut e, RTTI, 0x0abc_000a);
            e.register(MODIFIER_GET_VIEWER_STRINGS, |_, _| Ret::default());
            e.register(POINT_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x4444,
                ..Ret::default()
            });
            let stored = test_stream::viewer_strings(&mut e);
            let strings = test_stream::strings(&mut e);
            let this = gravity(&mut e);
            e.set(this, NiPSysGravityModifier::m_eType, force);
            e.call_log = Some(vec![]);
            e.call(0x00c2_38c0, &args![this, strings]);
            assert_eq!(
                *stored.borrow(),
                vec![
                    0x0abc_000a,
                    LABEL_GRAVITY_OBJECT,
                    0x4444,
                    LABEL_DECAY,
                    LABEL_STRENGTH,
                    LABEL_TURBULENCE,
                    LABEL_TURBULENCE_SCALE,
                    LABEL_FORCE_TYPE,
                    LABEL_WORLD_ALIGNED
                ]
            );
            assert_eq!(
                calls_to(&e, NI_GET_VIEWER_STRING_FIXED),
                vec![
                    vec![LABEL_GRAVITY_OBJECT, TEXT_NONE],
                    vec![LABEL_FORCE_TYPE, text]
                ]
            );
            assert_eq!(
                calls_to(&e, POINT_GET_VIEWER_STRING),
                vec![vec![this.addr() + 0x1c, LABEL_GRAVITY_AXIS]]
            );
            assert_eq!(
                calls_to(&e, NI_GET_VIEWER_STRING_FLOAT)[3],
                vec![LABEL_TURBULENCE_SCALE, 4.0f32.to_bits()]
            );
            assert_eq!(
                calls_to(&e, NI_GET_VIEWER_STRING_BOOL),
                vec![vec![LABEL_WORLD_ALIGNED, 1]]
            );
        }
    }
}
