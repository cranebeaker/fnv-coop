//! `NiParticle/nipsysemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c220a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x54 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_emitter_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x54u32]);
    }
    this
}

// Translated from 00c21fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2540.
pub fn ni_p_sys_emitter_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2540
}

use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysmodifier::{stream_read, NiPSysModifier, STREAM_VERSION};

layout! {
    /// `NiPSysEmitter` (Xbox PDB), 0x54 bytes; the PC code uses the same
    /// offsets. Float fields are declared as their bit patterns.
    pub struct NiPSysEmitter: 0x54 {
        /// `m_fSpeed` (Xbox PDB).
        0x18 m_fSpeed: u32,
        /// `m_fSpeedVar` (Xbox PDB).
        0x1C m_fSpeedVar: u32,
        /// `m_fDeclination` (Xbox PDB).
        0x20 m_fDeclination: u32,
        /// `m_fDeclinationVar` (Xbox PDB).
        0x24 m_fDeclinationVar: u32,
        /// `m_fPlanarAngle` (Xbox PDB).
        0x28 m_fPlanarAngle: u32,
        /// `m_fPlanarAngleVar` (Xbox PDB).
        0x2C m_fPlanarAngleVar: u32,
        /// `m_kInitialColor` (Xbox PDB): an `NiColorA` (four floats).
        0x30 m_kInitialColor: u32,
        /// `m_fInitialRadius` (Xbox PDB).
        0x40 m_fInitialRadius: u32,
        /// `m_fRadiusVar` (Xbox PDB).
        0x44 m_fRadiusVar: u32,
        /// `m_fLifeSpan` (Xbox PDB).
        0x48 m_fLifeSpan: u32,
        /// `m_fLifeSpanVar` (Xbox PDB).
        0x4C m_fLifeSpanVar: u32,
        /// `m_fScale` (Xbox PDB).
        0x50 m_fScale: u32,
    }
}

/// `_rand` (`00ecadb8`) and `_ftol2` (`00ec62c0`: truncates the `ST0`
/// value, passed as a leading `f64`).
const RAND: u32 = 0x00ec_adb8;
const FTOL: u32 = 0x00ec_62c0;
/// `double` constants: 32767.0 (`RAND_MAX`), 0.5, 1.0 and 512.0.
const RAND_MAX_DOUBLE: u32 = 0x0102_9790;
const HALF: u32 = 0x0101_1588;
const ONE: u32 = 0x0101_2070;
const TABLE_SIZE: u32 = 0x0102_8338;
/// The `float` 2π the angle-to-table-index scale divides by (`011aeae8`).
const TWO_PI: u32 = 0x011a_eae8;
/// The two 512-entry `float` tables the emitter reads by angle index:
/// the one at `011f52e0` gives the factors the decompiler shows multiplying
/// into x and y, the one at `011f4ae0` those for z and x (the fast sine and
/// cosine tables, by their use; not confirmed by name).
const TABLE_A: u32 = 0x011f_52e0;
const TABLE_B: u32 = 0x011f_4ae0;
/// `NiPSysData::INVALID_PARTICLE` (the `u16` at `010be248`).
const INVALID_PARTICLE: u32 = 0x010b_e248;
/// `NiParticleSystem::InitializeNewParticle` (`00c1aee0`, `thiscall`, the
/// particle index).
const INITIALIZE_NEW_PARTICLE: u32 = 0x00c1_aee0;
/// `NiPSysModifier::LoadBinary` (`00c318b0`) and `NiColorA::LoadBinary`
/// (`00a69800`, `thiscall` on the colour, the stream).
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const COLOR_LOAD_BINARY: u32 = 0x00a6_9800;
/// From file version 10.3.0.2 the radius variation is stored.
const VERSION_RADIUS_VAR: u32 = 0x0a03_0002;
/// `NiParticleSystem`'s model data (+0xb8, the `NiPSysData`) as the PC code
/// reads it.
const SYSTEM_DATA: u32 = 0xb8;

/// `rand() / 32767.0` rounded to a `float` (`FILD`, `FDIV qword`, `FSTP
/// dword`).
fn random_unit(e: &mut Engine) -> u32 {
    let r = e.call(RAND, &args![]).i32();
    let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
    x87_store32(x87_div(f64::from(r), rand_max))
}

/// `(2 * rand()) / 32767.0 - 1.0` rounded to a `float` (`FILD`, `FADD
/// ST0,ST0`, `FDIV qword`, `FSUB qword`, `FSTP dword`).
fn random_signed(e: &mut Engine) -> u32 {
    let r = f64::from(e.call(RAND, &args![]).i32());
    let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
    let one: f64 = e.global(ONE);
    x87_store32(x87_sub(x87_div(x87_add(r, r), rand_max), one))
}

// Translated from 00c220c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::EmitParticles` (Xbox PDB): `thiscall`, `RET 0xc`: the
/// current time (`float`), the number of particles (`u16` in a word) and
/// the address of their ages (`float`s). For each particle: the life span
/// is `(rand()/32767 - 0.5) * m_fLifeSpanVar + m_fLifeSpan`; when the life
/// span is not below the age (a NaN goes on), a particle is added to the
/// target's data (vtable slot +0xa8 of the data; at `INVALID_PARTICLE` the
/// function returns) and set up: speed, declination and planar angle from
/// their variations, the direction from the two tables at the angles'
/// indices (`ftol(angle * 512/2π) & 0x1ff`), velocity = direction * speed,
/// age, life span, generation 0; the emitter's position-and-velocity
/// (vtable slot +0xa8, with the vertex and the info); the initial colour,
/// radius `(m_fRadiusVar * r + m_fInitialRadius) * m_fScale`, size 1 and a
/// random texture index (`rand() % m_ucSubTextureOffsetCount`) when the
/// data has those arrays; the last update time is `time - age`; then
/// `NiParticleSystem::InitializeNewParticle`. The x87 arithmetic is done
/// instruction by instruction in the exe's order.
pub fn ni_p_sys_emitter_emit_particles(
    e: &mut Engine,
    this: Ptr<NiPSysEmitter>,
    time: f32,
    count: u16,
    ages: Ptr,
) {
    let target = e.mem.u32(this.addr() + NiPSysModifier::m_pkTarget.off);
    let data = Ptr::<NiPSysData>::new(e.mem.u32(target.wrapping_add(SYSTEM_DATA)));
    let vertices = e.get(data, NiPSysData::m_pkVertex);
    let colors = e.get(data, NiPSysData::m_pkColor);
    let radii = e.get(data, NiPSysData::m_pfRadii);
    let sizes = e.get(data, NiPSysData::m_pfSizes);
    let texture_indices = e.get(data, NiPSysData::m_pucTextureIndices);
    let sub_texture_count = e.get(data, NiPSysData::m_ucSubTextureOffsetCount);
    // A field as a `float` memory operand, and as loaded by `FLD dword`.
    let field = |e: &Engine, f| x87_m32(e.get(this, f));
    let loaded = |e: &Engine, f| x87_load32(e.get(this, f));

    let mut i: u16 = 0;
    while i < count {
        let age_bits = x87_quiet_bits(e.mem.u32(ages.addr() + 4 * u32::from(i)));
        let unit = random_unit(e);
        let half: f64 = e.global(HALF);
        let life_span = x87_store32(x87_add(
            x87_mul(
                x87_sub(x87_load32(unit), half),
                field(e, NiPSysEmitter::m_fLifeSpanVar),
            ),
            field(e, NiPSysEmitter::m_fLifeSpan),
        ));
        // `FLD age; FLD lifespan; FCOMPP`: skipped when the life span is
        // below the age.
        if x87_load32(life_span) < x87_load32(age_bits) {
            i = i.wrapping_add(1);
            continue;
        }
        let index = e.vcall(data.addr(), 0xa8, &args![]).u16();
        if index == e.global::<u16>(INVALID_PARTICLE) {
            return;
        }
        let k = u32::from(index);
        let info = e.get(data, NiPSysData::m_pkParticleInfo) + k * 0x1c;

        let unit = random_unit(e);
        let half: f64 = e.global(HALF);
        let speed = x87_store32(x87_add(
            x87_mul(
                x87_sub(x87_load32(unit), half),
                field(e, NiPSysEmitter::m_fSpeedVar),
            ),
            field(e, NiPSysEmitter::m_fSpeed),
        ));
        let r = random_signed(e);
        let declination = x87_store32(x87_add(
            x87_mul(loaded(e, NiPSysEmitter::m_fDeclinationVar), x87_m32(r)),
            field(e, NiPSysEmitter::m_fDeclination),
        ));
        let r = random_signed(e);
        let planar = x87_store32(x87_add(
            x87_mul(loaded(e, NiPSysEmitter::m_fPlanarAngleVar), x87_m32(r)),
            field(e, NiPSysEmitter::m_fPlanarAngle),
        ));
        let table_size: f64 = e.global(TABLE_SIZE);
        let scale = x87_div(table_size, x87_load32(e.global::<u32>(TWO_PI)));
        let declination_index = e
            .call(FTOL, &args![x87_mul(x87_load32(declination), scale)])
            .u32()
            & 0x1ff;
        let a_declination = x87_load32(e.global::<u32>(TABLE_A + 4 * declination_index));
        let a_declination_bits = x87_store32(a_declination);
        let planar_index = e.call(FTOL, &args![x87_mul(scale, x87_m32(planar))]).u32() & 0x1ff;
        let b_planar = x87_load32(e.global::<u32>(TABLE_B + 4 * planar_index));
        let a_declination = x87_load32(a_declination_bits);
        let x_direction = x87_store32(x87_mul(b_planar, a_declination));
        let y_direction = x87_store32(x87_mul(
            a_declination,
            x87_m32(e.global::<u32>(TABLE_A + 4 * planar_index)),
        ));
        let z_direction = x87_store32(x87_load32(e.global::<u32>(TABLE_B + 4 * declination_index)));
        let speed = x87_load32(speed);
        let vx = x87_store32(x87_mul(x87_load32(x_direction), speed));
        e.mem.set_u32(info, vx);
        let vy = x87_store32(x87_mul(x87_load32(y_direction), speed));
        e.mem.set_u32(info + 4, vy);
        let vz = x87_store32(x87_mul(speed, x87_m32(z_direction)));
        e.mem.set_u32(info + 8, vz);
        e.mem.set_u32(info + 0xc, age_bits);
        e.mem.set_u16(info + 0x18, 0);
        e.mem.set_u32(info + 0x10, x87_quiet_bits(life_span));

        e.vcall(this.addr(), 0xa8, &args![vertices + k * 0xc, info]);
        if colors != 0 {
            let color = colors + (k << 4);
            for word in 0..4 {
                let value = e
                    .mem
                    .u32(this.addr() + NiPSysEmitter::m_kInitialColor.off + 4 * word);
                e.mem.set_u32(color + 4 * word, value);
            }
        }
        if radii != 0 {
            let r = random_signed(e);
            let radius = x87_store32(x87_mul(
                x87_add(
                    x87_mul(loaded(e, NiPSysEmitter::m_fRadiusVar), x87_m32(r)),
                    field(e, NiPSysEmitter::m_fInitialRadius),
                ),
                field(e, NiPSysEmitter::m_fScale),
            ));
            e.mem.set_u32(radii + 4 * k, radius);
        }
        if sizes != 0 {
            e.mem.set_u32(sizes + 4 * k, 1.0f32.to_bits());
        }
        if texture_indices != 0 {
            let r = e.call(RAND, &args![]).i32();
            let index = r % i32::from(sub_texture_count);
            e.mem.set_u8(k + texture_indices, index as u8);
        }
        let last_update = x87_store32(x87_sub(
            x87_load32(time.to_bits()),
            x87_m32(e.mem.u32(info + 0xc)),
        ));
        e.mem.set_u32(info + 0x14, last_update);
        e.call(INITIALIZE_NEW_PARTICLE, &args![target, k]);
        i = i.wrapping_add(1);
    }
}

// Translated from 00c223b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::LoadBinary` (Xbox PDB): `NiPSysModifier::LoadBinary`,
/// then 4 bytes each of speed, speed variation, declination, declination
/// variation, planar angle and its variation, the initial colour
/// (`NiColorA::LoadBinary`), the initial radius, the radius variation
/// (from file version 10.3.0.2), the life span and its variation. The scale
/// is not read. The component-size word is the stream argument's stack
/// word.
pub fn ni_p_sys_emitter_load_binary(e: &mut Engine, this: Ptr<NiPSysEmitter>, stream: Ptr) {
    e.call(MODIFIER_LOAD_BINARY, &args![this, stream]);
    let stream = stream.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let read = |e: &mut Engine, off: u32| stream_read(e, stream, this.addr() + off, 4, 4, slot);
        read(e, NiPSysEmitter::m_fSpeed.off);
        read(e, NiPSysEmitter::m_fSpeedVar.off);
        read(e, NiPSysEmitter::m_fDeclination.off);
        read(e, NiPSysEmitter::m_fDeclinationVar.off);
        read(e, NiPSysEmitter::m_fPlanarAngle.off);
        read(e, NiPSysEmitter::m_fPlanarAngleVar.off);
        e.call(
            COLOR_LOAD_BINARY,
            &args![this.addr() + NiPSysEmitter::m_kInitialColor.off, stream],
        );
        read(e, NiPSysEmitter::m_fInitialRadius.off);
        if e.mem.u32(stream + STREAM_VERSION) >= VERSION_RADIUS_VAR {
            read(e, NiPSysEmitter::m_fRadiusVar.off);
        }
        read(e, NiPSysEmitter::m_fLifeSpan.off);
        read(e, NiPSysEmitter::m_fLifeSpanVar.off);
    });
}

use crate::units::niparticle::nipsysdata::{add_viewer_string, stream_write};
use crate::units::niparticle::nipsysmodifier::fixed_string_release;

/// The vtable `NiPSysEmitter`'s constructor stores (`010bde84`).
const VTABLE: u32 = 0x010b_de84;
/// `00a5b690` (unnamed, `cdecl`, a C string): the `NiFixedString` handle
/// for a C string; and the null handle's global (`0109b220`).
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
const NULL_FIXED_STRING: u32 = 0x0109_b220;
/// `NiPSysModifier`'s constructor with a name and an order (`00c316a0`,
/// `thiscall`: the address of an `NiFixedString`, the order).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_16a0;
/// `NiPSysModifier`'s `SaveBinary` (`00c31950`), `IsEqual` (`00c31830`) and
/// `GetViewerStrings` (`00c319e0`) (Xbox PDB names; `thiscall`, one word).
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// `NiColorA::SaveBinary` (`00a698a0`, one word: the stream) and
/// `NiColorA::GetViewerString` (`00a69720`, one word: the label) (Xbox PDB
/// names; `thiscall` on the colour), and `004a4c90` (`NiColorA::operator_`
/// in the engine map; `thiscall` on a colour, the other colour; a nonzero
/// answer counts as unequal here).
const COLOR_SAVE_BINARY: u32 = 0x00a6_98a0;
const COLOR_GET_VIEWER_STRING: u32 = 0x00a6_9720;
const COLOR_UNEQUAL: u32 = 0x004a_4c90;
/// `NiGetViewerString` (`00a7d330`, the text) and `NiGetViewerString_ov9`
/// (`00a7d550`: a label and a `float`) (Xbox PDB names; `cdecl`).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// `NiPSysEmitter::ms_RTTI` (its name is the first word).
const RTTI: u32 = 0x0120_2540;
/// The labels: "Speed" (`01044b18`), "Speed Variation" (`010bdfd0`),
/// "Declination" (`010bdfc4`), "Declination Variation" (`010bdfac`),
/// "Planar Angle" (`010bdf9c`), "Planar Angle Variation" (`010bdf84`),
/// "Initial Color" (`010bdf74`), "Initial Radius" (`010bdf64`), "Radius
/// Variation" (`010bdf50`), "Life Span" (`010bdf44`) and "Life Span
/// Variation" (`010bdf30`).
const LABEL_SPEED: u32 = 0x0104_4b18;
const LABEL_SPEED_VAR: u32 = 0x010b_dfd0;
const LABEL_DECLINATION: u32 = 0x010b_dfc4;
const LABEL_DECLINATION_VAR: u32 = 0x010b_dfac;
const LABEL_PLANAR_ANGLE: u32 = 0x010b_df9c;
const LABEL_PLANAR_ANGLE_VAR: u32 = 0x010b_df84;
const LABEL_INITIAL_COLOR: u32 = 0x010b_df74;
const LABEL_INITIAL_RADIUS: u32 = 0x010b_df64;
const LABEL_RADIUS_VAR: u32 = 0x010b_df50;
const LABEL_LIFE_SPAN: u32 = 0x010b_df44;
const LABEL_LIFE_SPAN_VAR: u32 = 0x010b_df30;

/// The six floats before the colour and the four after it, in the order
/// the save, the comparison and the viewer strings take them.
const FLOATS_BEFORE_COLOR: [Field<NiPSysEmitter, u32>; 6] = [
    NiPSysEmitter::m_fSpeed,
    NiPSysEmitter::m_fSpeedVar,
    NiPSysEmitter::m_fDeclination,
    NiPSysEmitter::m_fDeclinationVar,
    NiPSysEmitter::m_fPlanarAngle,
    NiPSysEmitter::m_fPlanarAngleVar,
];
const FLOATS_AFTER_COLOR: [Field<NiPSysEmitter, u32>; 4] = [
    NiPSysEmitter::m_fInitialRadius,
    NiPSysEmitter::m_fRadiusVar,
    NiPSysEmitter::m_fLifeSpan,
    NiPSysEmitter::m_fLifeSpanVar,
];

// Translated from 00c21fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPSysEmitter` constructor with its settings (no PDB name):
/// `NiPSysModifier`'s constructor with the name's `NiFixedString` (the null
/// handle for a null name, else `00a5b690`, in a local released after) and
/// `order`; then the vtable, the speed, declination and planar angle with
/// their variations, the initial colour (four words), the initial radius,
/// life span and its variation, the radius variation, and a scale of 1.0.
/// Returns `this`.
pub fn fn_00c21fe0(
    e: &mut Engine,
    this: Ptr<NiPSysEmitter>,
    name: u32,
    order: u32,
    speed: f32,
    speed_var: f32,
    declination: f32,
    declination_var: f32,
    planar_angle: f32,
    planar_angle_var: f32,
    color_r: u32,
    color_g: u32,
    color_b: u32,
    color_a: u32,
    initial_radius: f32,
    life_span: f32,
    life_span_var: f32,
    radius_var: f32,
) -> Ptr<NiPSysEmitter> {
    let handle = if name == 0 {
        e.global::<u32>(NULL_FIXED_STRING)
    } else {
        e.call(FIXED_STRING_FROM_TEXT, &args![name]).u32()
    };
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), handle);
        e.call(MODIFIER_CONSTRUCTOR, &args![this, local, order]);
        fixed_string_release(e, local.addr());
    });
    let float = |value: f32| x87_quiet_bits(value.to_bits());
    e.set(this, NiPSysEmitter::m_fSpeed, float(speed));
    e.set(this, NiPSysEmitter::m_fSpeedVar, float(speed_var));
    e.set(this, NiPSysEmitter::m_fDeclination, float(declination));
    e.set(
        this,
        NiPSysEmitter::m_fDeclinationVar,
        float(declination_var),
    );
    e.set(this, NiPSysEmitter::m_fPlanarAngle, float(planar_angle));
    e.set(
        this,
        NiPSysEmitter::m_fPlanarAngleVar,
        float(planar_angle_var),
    );
    let color = this.addr() + NiPSysEmitter::m_kInitialColor.off;
    e.mem.set_u32(color, color_r);
    e.mem.set_u32(color + 4, color_g);
    e.mem.set_u32(color + 8, color_b);
    e.mem.set_u32(color + 12, color_a);
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, NiPSysEmitter::m_fInitialRadius, float(initial_radius));
    e.set(this, NiPSysEmitter::m_fRadiusVar, float(radius_var));
    e.set(this, NiPSysEmitter::m_fLifeSpan, float(life_span));
    e.set(this, NiPSysEmitter::m_fLifeSpanVar, float(life_span_var));
    e.set(this, NiPSysEmitter::m_fScale, 1.0f32.to_bits());
    this
}

// Translated from 00c22530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::SaveBinary` (Xbox PDB): `NiPSysModifier`'s, the speed,
/// declination and planar angle with their variations (4 bytes each), the
/// initial colour (`NiColorA::SaveBinary`), then the initial radius, the
/// radius variation, the life span and its variation.
pub fn ni_p_sys_emitter_save_binary(e: &mut Engine, this: Ptr<NiPSysEmitter>, stream: Ptr) {
    e.call(MODIFIER_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, slot| {
        for field in FLOATS_BEFORE_COLOR {
            let value = this.addr() + field.off;
            stream_write(e, stream.addr(), value, 4, 4, slot.addr());
        }
        let color = this.addr() + NiPSysEmitter::m_kInitialColor.off;
        e.call(COLOR_SAVE_BINARY, &args![color, stream]);
        for field in FLOATS_AFTER_COLOR {
            let value = this.addr() + field.off;
            stream_write(e, stream.addr(), value, 4, 4, slot.addr());
        }
    });
}

// Translated from 00c226a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::IsEqual` (Xbox PDB): `NiPSysModifier`'s, then equal
/// speed, declination and planar angle with their variations (`FUCOMPP`: a
/// NaN is unequal), equal initial colours (`004a4c90` on the other's colour
/// answering 0), and equal initial radius, radius variation, life span and
/// life span variation.
pub fn ni_p_sys_emitter_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysEmitter>,
    other: Ptr<NiPSysEmitter>,
) -> bool {
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    for field in FLOATS_BEFORE_COLOR {
        if x87_m32(e.get(this, field)) != x87_m32(e.get(other, field)) {
            return false;
        }
    }
    let ours = this.addr() + NiPSysEmitter::m_kInitialColor.off;
    let theirs = other.addr() + NiPSysEmitter::m_kInitialColor.off;
    if e.call(COLOR_UNEQUAL, &args![theirs, ours]).bool() {
        return false;
    }
    for field in FLOATS_AFTER_COLOR {
        if x87_m32(e.get(this, field)) != x87_m32(e.get(other, field)) {
            return false;
        }
    }
    true
}

// Translated from 00c22780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitter::GetViewerStrings` (Xbox PDB): `NiPSysModifier`'s strings,
/// the class name, the speed, declination and planar angle with their
/// variations, the initial colour (`NiColorA::GetViewerString`), the
/// initial radius, radius variation, life span and its variation.
pub fn ni_p_sys_emitter_get_viewer_strings(e: &mut Engine, this: Ptr<NiPSysEmitter>, strings: Ptr) {
    let strings = strings.addr();
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, string);
    let labels = [
        LABEL_SPEED,
        LABEL_SPEED_VAR,
        LABEL_DECLINATION,
        LABEL_DECLINATION_VAR,
        LABEL_PLANAR_ANGLE,
        LABEL_PLANAR_ANGLE_VAR,
    ];
    for (label, field) in labels.into_iter().zip(FLOATS_BEFORE_COLOR) {
        let value = x87_quiet_bits(e.get(this, field));
        let string = e.call(VIEWER_STRING_FLOAT, &args![label, value]).u32();
        add_viewer_string(e, strings, string);
    }
    let color = this.addr() + NiPSysEmitter::m_kInitialColor.off;
    let string = e
        .call(COLOR_GET_VIEWER_STRING, &args![color, LABEL_INITIAL_COLOR])
        .u32();
    add_viewer_string(e, strings, string);
    let labels = [
        LABEL_INITIAL_RADIUS,
        LABEL_RADIUS_VAR,
        LABEL_LIFE_SPAN,
        LABEL_LIFE_SPAN_VAR,
    ];
    for (label, field) in labels.into_iter().zip(FLOATS_AFTER_COLOR) {
        let value = x87_quiet_bits(e.get(this, field));
        let string = e.call(VIEWER_STRING_FLOAT, &args![label, value]).u32();
        add_viewer_string(e, strings, string);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_20c0,
            ni_p_sys_emitter_emit_particles(Ptr<NiPSysEmitter>, f32, u16, Ptr)
        ),
        entry!(
            0x00c2_23b0,
            ni_p_sys_emitter_load_binary(Ptr<NiPSysEmitter>, Ptr)
        ),
        entry!(0x00c21fd0, ni_p_sys_emitter_get_r_t_t_i() -> u32),
        entry!(0x00c220a0, ni_p_sys_emitter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c2_1fe0,
            fn_00c21fe0(
                Ptr<NiPSysEmitter>,
                u32,
                u32,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                u32,
                u32,
                u32,
                u32,
                f32,
                f32,
                f32,
                f32,
            ) -> Ptr<NiPSysEmitter>
        ),
        entry!(
            0x00c2_2530,
            ni_p_sys_emitter_save_binary(Ptr<NiPSysEmitter>, Ptr)
        ),
        entry!(
            0x00c2_26a0,
            ni_p_sys_emitter_is_equal(Ptr<NiPSysEmitter>, Ptr<NiPSysEmitter>) -> bool
        ),
        entry!(
            0x00c2_2780,
            ni_p_sys_emitter_get_viewer_strings(Ptr<NiPSysEmitter>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_emitter_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00c220a0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c220a0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x54]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_emitter_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_1fd0, &args![]).u32(), 0x0120_2540);
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

    const ADD_PARTICLE_FN: u32 = 0x0f40_0000;
    const POSITION_FN: u32 = 0x0f40_0010;

    /// An emitter over a particle system whose data has every optional
    /// array, the constants the code reads, and tables whose entry 0 is
    /// 0.5 (`011f52e0`) and 0.25 (`011f4ae0`). `rand` always answers 7.
    fn emitter(e: &mut Engine) -> (Ptr<NiPSysEmitter>, Ptr<NiPSysData>) {
        e.map(0x0101_1000, 0x2_0000);
        e.set_global(RAND_MAX_DOUBLE, 32767.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(TABLE_SIZE, 512.0f64);
        e.map(TWO_PI & !0xfff, 0x1000);
        e.set_global(TWO_PI, std::f32::consts::TAU);
        e.map(INVALID_PARTICLE & !0xfff, 0x1000);
        e.set_global(INVALID_PARTICLE, 0xffffu16);
        e.map(TABLE_B, 0x1000);
        e.set_global(TABLE_A, 0.5f32);
        e.set_global(TABLE_B, 0.25f32);
        e.register(RAND, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.register(INITIALIZE_NEW_PARTICLE, |_, _| Ret::default());
        e.register(POSITION_FN, |_, _| Ret::default());
        // The data hands out particles 0, 1, ... from a counter at +0x7c.
        e.register(ADD_PARTICLE_FN, |e, a| {
            let next = e.mem.u32(a[0] + 0x7c);
            e.mem.set_u32(a[0] + 0x7c, next + 1);
            Ret {
                eax: next,
                ..Ret::default()
            }
        });
        let data = Ptr::<NiPSysData>::new(e.mem.alloc(0x80));
        let data_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(data_vtable + 0xa8, ADD_PARTICLE_FN);
        e.mem.set_u32(data.addr(), data_vtable);
        let arrays = [
            (NiPSysData::m_pkVertex, 0xc * 4),
            (NiPSysData::m_pkColor, 0x10 * 4),
            (NiPSysData::m_pfRadii, 4 * 4),
            (NiPSysData::m_pfSizes, 4 * 4),
            (NiPSysData::m_pucTextureIndices, 4),
            (NiPSysData::m_pkParticleInfo, 0x1c * 4),
        ];
        for (field, size) in arrays {
            let block = e.mem.alloc(size);
            e.set(data, field, block);
        }
        e.set(data, NiPSysData::m_ucSubTextureOffsetCount, 4);
        let system = e.mem.alloc(0x100);
        e.mem.set_u32(system + SYSTEM_DATA, data.addr());
        let this = e.new_object::<NiPSysEmitter>();
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xa8, POSITION_FN);
        e.mem.set_u32(this.addr(), vtable);
        e.mem.set_u32(this.addr() + 0x10, system);
        e.set(this, NiPSysEmitter::m_fLifeSpan, 10.0f32.to_bits());
        e.set(this, NiPSysEmitter::m_fSpeed, 2.0f32.to_bits());
        e.set(this, NiPSysEmitter::m_fInitialRadius, 3.0f32.to_bits());
        e.set(this, NiPSysEmitter::m_fScale, 2.0f32.to_bits());
        for (i, value) in [0.1f32, 0.2, 0.3, 0.4].iter().enumerate() {
            e.mem
                .set_u32(this.addr() + 0x30 + 4 * i as u32, value.to_bits());
        }
        (this, data)
    }

    #[test]
    fn emit_particles_sets_up_each_living_particle() {
        let mut e = Engine::new();
        let (this, data) = emitter(&mut e);
        let ages = e.mem.alloc(8);
        e.mem.set_f32(ages, 0.0);
        e.mem.set_f32(ages + 4, 100.0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_20c0, &args![this, 5.0f32, 2u32, ages]);
        // The second particle's age is beyond its life span: one particle.
        assert_eq!(log_of(&e, ADD_PARTICLE_FN).len(), 1);
        let info = e.get(data, NiPSysData::m_pkParticleInfo);
        assert_eq!(e.mem.f32(info), 0.25);
        assert_eq!(e.mem.f32(info + 4), 0.5);
        assert_eq!(e.mem.f32(info + 8), 0.5);
        assert_eq!(e.mem.f32(info + 0xc), 0.0);
        assert_eq!(e.mem.f32(info + 0x10), 10.0);
        assert_eq!(e.mem.f32(info + 0x14), 5.0);
        assert_eq!(e.mem.u16(info + 0x18), 0);
        let vertices = e.get(data, NiPSysData::m_pkVertex);
        assert_eq!(
            log_of(&e, POSITION_FN),
            vec![vec![this.addr(), vertices, info]]
        );
        let colors = e.get(data, NiPSysData::m_pkColor);
        assert_eq!(e.mem.f32(colors + 0xc), 0.4);
        assert_eq!(e.mem.f32(e.get(data, NiPSysData::m_pfRadii)), 6.0);
        assert_eq!(e.mem.f32(e.get(data, NiPSysData::m_pfSizes)), 1.0);
        assert_eq!(e.mem.u8(e.get(data, NiPSysData::m_pucTextureIndices)), 3);
        let system = e.mem.u32(this.addr() + 0x10);
        assert_eq!(log_of(&e, INITIALIZE_NEW_PARTICLE), vec![vec![system, 0]]);
    }

    #[test]
    fn emit_particles_stops_when_no_particle_is_free() {
        let mut e = Engine::new();
        let (this, data) = emitter(&mut e);
        e.mem.set_u32(data.addr() + 0x7c, 0xffff);
        let ages = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00c2_20c0, &args![this, 5.0f32, 2u32, ages]);
        assert_eq!(log_of(&e, ADD_PARTICLE_FN).len(), 1);
        assert!(log_of(&e, POSITION_FN).is_empty());
        assert!(log_of(&e, INITIALIZE_NEW_PARTICLE).is_empty());
    }

    #[test]
    fn load_binary_reads_the_emitter_fields() {
        for (version, radius_var) in [(0x0a03_0002u32, 9.0f32), (0x0a03_0001, 0.0)] {
            let mut e = Engine::new();
            e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
            e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
            let values: Vec<f32> = if radius_var == 0.0 {
                vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 10.0, 11.0]
            } else {
                vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 9.0, 10.0, 11.0]
            };
            let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
            let stream = test_stream::new(&mut e, version, &bytes);
            let this = e.new_object::<NiPSysEmitter>();
            e.call_log = Some(vec![]);
            e.call(0x00c2_23b0, &args![this, stream]);
            assert_eq!(
                log_of(&e, COLOR_LOAD_BINARY),
                vec![vec![this.addr() + 0x30, stream]]
            );
            let f = |e: &Engine, field| f32::from_bits(e.get(this, field));
            assert_eq!(f(&e, NiPSysEmitter::m_fSpeed), 1.0);
            assert_eq!(f(&e, NiPSysEmitter::m_fPlanarAngleVar), 6.0);
            assert_eq!(f(&e, NiPSysEmitter::m_fInitialRadius), 7.0);
            assert_eq!(f(&e, NiPSysEmitter::m_fRadiusVar), radius_var);
            assert_eq!(f(&e, NiPSysEmitter::m_fLifeSpan), 10.0);
            assert_eq!(f(&e, NiPSysEmitter::m_fLifeSpanVar), 11.0);
            assert_eq!(f(&e, NiPSysEmitter::m_fScale), 0.0);
        }
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to};
    use crate::units::niparticle::nipsysdata::STREAM_OUTPUT;
    use crate::units::niparticle::nipsysmodifier::test_stream;

    /// An emitter whose ten floats are 1.0 to 10.0 in save order and whose
    /// colour words are 0x11..0x44.
    fn emitter(e: &mut Engine) -> Ptr<NiPSysEmitter> {
        let this = e.new_object::<NiPSysEmitter>();
        let fields = FLOATS_BEFORE_COLOR.into_iter().chain(FLOATS_AFTER_COLOR);
        for (i, field) in fields.enumerate() {
            e.set(this, field, (i as f32 + 1.0).to_bits());
        }
        for i in 0..4 {
            e.mem.set_u32(this.addr() + 0x30 + 4 * i, 0x11 * (i + 1));
        }
        this
    }

    #[test]
    fn constructor_stores_the_settings() {
        let mut e = Engine::new();
        test_stream::interlocked(&mut e);
        e.map(NULL_FIXED_STRING, 4);
        e.set_global::<u32>(NULL_FIXED_STRING, 0x0f60_0000);
        let handle = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(handle - 8, 1);
        e.map(0x0f60_0010, 8);
        e.mem.set_u32(0x0f60_0010, handle);
        e.register(FIXED_STRING_FROM_TEXT, |e, _| {
            answer(e.mem.u32(0x0f60_0010))
        });
        e.register(MODIFIER_CONSTRUCTOR, |e, a| {
            let name = e.mem.u32(a[1]);
            e.mem.set_u32(0x0f60_0014, name);
            Ret::default()
        });
        let this = e.new_object::<NiPSysEmitter>();
        e.call_log = Some(vec![]);
        let words = args![
            this,
            0x0100_0000u32,
            7u32,
            1.0f32,
            2.0f32,
            3.0f32,
            4.0f32,
            5.0f32,
            0x7f80_0001u32,
            0x11u32,
            0x22u32,
            0x33u32,
            0x44u32,
            6.0f32,
            7.0f32,
            8.0f32,
            9.0f32
        ];
        assert_eq!(e.call(0x00c2_1fe0, &words).u32(), this.addr());
        assert_eq!(
            calls_to(&e, FIXED_STRING_FROM_TEXT),
            vec![vec![0x0100_0000]]
        );
        let constructed = calls_to(&e, MODIFIER_CONSTRUCTOR);
        assert_eq!((constructed[0][0], constructed[0][2]), (this.addr(), 7));
        assert_eq!(e.mem.u32(0x0f60_0014), handle);
        assert_eq!(e.mem.u32(handle - 8), 0);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        let floats: Vec<u32> = [1.0f32, 2.0, 3.0, 4.0, 5.0]
            .iter()
            .map(|v| v.to_bits())
            .collect();
        for (field, value) in FLOATS_BEFORE_COLOR.iter().zip(&floats) {
            assert_eq!(e.get(this, *field), *value);
        }
        assert_eq!(e.get(this, NiPSysEmitter::m_fPlanarAngleVar), 0x7fc0_0001);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this.addr() + 0x30 + 4 * i), 0x11 * (i + 1));
        }
        assert_eq!(
            e.get(this, NiPSysEmitter::m_fInitialRadius),
            6.0f32.to_bits()
        );
        assert_eq!(e.get(this, NiPSysEmitter::m_fLifeSpan), 7.0f32.to_bits());
        assert_eq!(e.get(this, NiPSysEmitter::m_fLifeSpanVar), 8.0f32.to_bits());
        assert_eq!(e.get(this, NiPSysEmitter::m_fRadiusVar), 9.0f32.to_bits());
        assert_eq!(e.get(this, NiPSysEmitter::m_fScale), 1.0f32.to_bits());
        // A null name: the null handle, not released.
        e.call(0x00c2_1fe0, &{
            let mut words = words.clone();
            words[1] = 0;
            words
        });
        assert_eq!(e.mem.u32(0x0f60_0014), 0x0f60_0000);
        assert_eq!(calls_to(&e, FIXED_STRING_FROM_TEXT).len(), 1);
    }

    #[test]
    fn save_binary_writes_the_floats_around_the_colour() {
        let mut e = Engine::new();
        e.register(MODIFIER_SAVE_BINARY, |_, _| Ret::default());
        let stream = viewer_test::out_stream(&mut e);
        e.register(COLOR_SAVE_BINARY, |e, a| {
            let output = e.mem.u32(a[1] + STREAM_OUTPUT);
            let buffer = e.mem.u32(output + 0x10);
            let used = e.mem.u32(buffer);
            e.mem.write(buffer + 4 + used, b"COLR");
            e.mem.set_u32(buffer, used + 4);
            Ret::default()
        });
        let this = emitter(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_2530, &args![this, stream]);
        assert_eq!(
            calls_to(&e, COLOR_SAVE_BINARY),
            vec![vec![this.addr() + 0x30, stream]]
        );
        let mut bytes: Vec<u8> = (1..=6).flat_map(|i| (i as f32).to_le_bytes()).collect();
        bytes.extend(b"COLR");
        bytes.extend((7..=10).flat_map(|i| (i as f32).to_le_bytes()));
        let (written, position) = viewer_test::written(&e, stream);
        assert_eq!(written, bytes);
        assert_eq!(position, 40);
    }

    #[test]
    fn is_equal_compares_every_setting() {
        let mut e = Engine::new();
        e.register(MODIFIER_IS_EQUAL, |_, _| answer(1));
        e.register(COLOR_UNEQUAL, |e, a| {
            answer(u32::from(e.mem.bytes(a[0], 16) != e.mem.bytes(a[1], 16)))
        });
        let this = emitter(&mut e);
        let other = emitter(&mut e);
        let check = |e: &mut Engine| e.call(0x00c2_26a0, &args![this, other]).bool();
        e.call_log = Some(vec![]);
        assert!(check(&mut e));
        assert_eq!(
            calls_to(&e, COLOR_UNEQUAL),
            vec![vec![other.addr() + 0x30, this.addr() + 0x30]]
        );
        let fields = FLOATS_BEFORE_COLOR.into_iter().chain(FLOATS_AFTER_COLOR);
        for field in fields {
            let kept = e.get(other, field);
            e.set(other, field, 0.25f32.to_bits());
            assert!(!check(&mut e));
            e.set(other, field, kept);
        }
        e.mem.set_u32(other.addr() + 0x3c, 0);
        assert!(!check(&mut e));
        e.mem.set_u32(other.addr() + 0x3c, 0x44);
        e.set(this, NiPSysEmitter::m_fLifeSpanVar, f32::NAN.to_bits());
        e.set(other, NiPSysEmitter::m_fLifeSpanVar, f32::NAN.to_bits());
        assert!(!check(&mut e));
        e.register(MODIFIER_IS_EQUAL, |_, _| answer(0));
        assert!(!check(&mut e));
    }

    #[test]
    fn viewer_strings_name_every_setting() {
        let mut e = Engine::new();
        e.register(MODIFIER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        viewer_test::counting(
            &mut e,
            &[VIEWER_STRING, VIEWER_STRING_FLOAT, COLOR_GET_VIEWER_STRING],
        );
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_0070);
        let strings = viewer_test::strings(&mut e, 4);
        let this = emitter(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_2780, &args![this, strings]);
        assert_eq!(
            viewer_test::collected(&e, strings),
            (1..=12).collect::<Vec<u32>>()
        );
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x0100_0070]]);
        assert_eq!(
            calls_to(&e, COLOR_GET_VIEWER_STRING),
            vec![vec![this.addr() + 0x30, LABEL_INITIAL_COLOR]]
        );
        let labels = [
            LABEL_SPEED,
            LABEL_SPEED_VAR,
            LABEL_DECLINATION,
            LABEL_DECLINATION_VAR,
            LABEL_PLANAR_ANGLE,
            LABEL_PLANAR_ANGLE_VAR,
            LABEL_INITIAL_RADIUS,
            LABEL_RADIUS_VAR,
            LABEL_LIFE_SPAN,
            LABEL_LIFE_SPAN_VAR,
        ];
        let expected: Vec<Vec<u32>> = labels
            .iter()
            .enumerate()
            .map(|(i, &label)| vec![label, (i as f32 + 1.0).to_bits()])
            .collect();
        assert_eq!(calls_to(&e, VIEWER_STRING_FLOAT), expected);
    }
}
