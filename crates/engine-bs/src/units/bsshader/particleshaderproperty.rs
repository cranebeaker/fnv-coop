//! `BSShader/particleshaderproperty.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ParticleShaderProperty`: its vtable is `010b8250`, whose `GetRTTI` (`fn_00ba7f10`) answers
//! the record `01200434` the static initializer `00fba130` names `"ParticleShaderProperty"`.
//! On PC it is 0x14c bytes (the clone's allocation), the Xbox PDB's fields 8 bytes lower as
//! the base `BSShaderProperty` is 0x60 bytes on PC (see [`ParticleShaderProperty`]).
//! `pInstanceData` is an array of the shader's particle count (`00ba52d0`) 0x30-byte
//! `ParticleShader::ParticleData` (Xbox PDB): position (+0), `fBirthTime` (+0xc), velocity
//! (+0x10), `fLifeAdjust` (+0x1c), rotation start and speed (+0x20, +0x24), `fSpeedMult`
//! (+0x28), `fTexCoords` (+0x2c). An unused particle has its position at `FLT_MAX` and its
//! birth time at -10000.
//!
//! x87: each `FADD`/`FSUB`/`FMUL`/`FDIV` goes through `crate::x87` in the exe's operand order;
//! `FLD`/`FSTP` copies quiet a signalling NaN.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::NiTArray;
use crate::units::bsshader::bsshaderproperty::BSShaderProperty;

layout! {
    /// `ParticleShaderProperty` (Xbox PDB), the fields used here at their PC offsets.
    pub struct ParticleShaderProperty: 0x14c {
        /// `pInstanceData` (Xbox PDB +0x68).
        0x60 pInstanceData: u32,
        /// `eEmitterType` (Xbox PDB +0x6c).
        0x64 eEmitterType: u32,
        /// `fRayLength` (Xbox PDB +0x70), as bits.
        0x68 fRayLength: u32,
        /// `bWorldspace` (Xbox PDB +0x74).
        0x6c bWorldspace: u8,
        /// `iParticleCount` (Xbox PDB +0x78).
        0x70 iParticleCount: i32,
        /// `fGenerateRemainder` (Xbox PDB +0x7c), as bits.
        0x74 fGenerateRemainder: u32,
        /// `fMaxParticleRatio` (Xbox PDB +0x80), as bits.
        0x78 fMaxParticleRatio: u32,
        /// `fLifetime` (Xbox PDB +0x84), as bits.
        0x7c fLifetime: u32,
        /// `fLifeVar` (Xbox PDB +0x88), as bits.
        0x80 fLifeVar: u32,
        /// `fNormalSpeed` (Xbox PDB +0x8c), as bits.
        0x84 fNormalSpeed: u32,
        /// `fNormalSpeedVar` (Xbox PDB +0x90), as bits.
        0x88 fNormalSpeedVar: u32,
        /// `fRotate` (Xbox PDB +0xb0), as bits.
        0xa8 fRotate: u32,
        /// `fRotateVar` (Xbox PDB +0xb4), as bits.
        0xac fRotateVar: u32,
        /// `fRotateSpeed` (Xbox PDB +0xb8), as bits.
        0xb0 fRotateSpeed: u32,
        /// `fRotateSpeedVar` (Xbox PDB +0xbc), as bits.
        0xb4 fRotateSpeedVar: u32,
        /// `fTimer` (Xbox PDB +0x10c), as bits.
        0x104 fTimer: u32,
        /// `spBaseTexture` (Xbox PDB +0x120).
        0x118 spBaseTexture: u32,
        /// `TargetArray` (Xbox PDB +0x124): an `NiTObjectArray<NiPointer<NiAVObject>>`.
        0x11c TargetArray: u32,
        /// `pMyGeometry` (Xbox PDB +0x134).
        0x12c pMyGeometry: u32,
        /// `fTextureCountX` (Xbox PDB +0x13c), as bits.
        0x134 fTextureCountX: u32,
        /// `fTextureCountY` (Xbox PDB +0x140), as bits.
        0x138 fTextureCountY: u32,
    }
}

type Particles = ParticleShaderProperty;

/// The vtable (`010b8250`) and the RTTI record (`01200434`).
const PARTICLE_PROPERTY_VTABLE: u32 = 0x010b_8250;
const PARTICLE_PROPERTY_RTTI: u32 = 0x0120_0434;
/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The particle count (`00ba52d0`, `cdecl`, no arguments).
const PARTICLE_COUNT_00BA52D0: u32 = 0x00ba_52d0;
/// `_rand` (`00ecadb8`, `cdecl`, no arguments) and `_ftol2_sse` (`00ec62c0`, the value in ST0:
/// a leading `f64` argument).
const RAND: u32 = 0x00ec_adb8;
const FTOL: u32 = 0x00ec_62c0;
/// The memory manager (`011f6238`) and its allocation (`00aa3e40`) and deallocation
/// (`00aa4060`), `thiscall`, one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The allocation `00aa13e0` (`cdecl`, one word) and the sized free `00aa1460` (`cdecl`, two
/// words).
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;
/// `_memset` (`00ec61c0`, `cdecl`, three words).
const MEMSET: u32 = 0x00ec_61c0;
/// `BSShaderProperty`'s constructor (`00ba8e50`), destructor body (`00baa0e0`) and member copy
/// (`00ba8b10`, two words: the clone and the cloning process), `thiscall`.
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00ba_8e50;
const SHADER_PROPERTY_DESTROY: u32 = 0x00ba_a0e0;
const SHADER_PROPERTY_COPY_MEMBERS: u32 = 0x00ba_8b10;
/// `BSShaderProperty::CheckCreateRenderPassArray` (`00ba95e0`, `thiscall`, one word) and
/// `BSShaderProperty::RenderPassArray::Add` (`00ba9ee0`, `thiscall`, eight words).
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// `BSShaderProperty::GetViewerStrings` (`00ba9890`, `thiscall`, one word), `NiGetViewerString`
/// (`00a7d330`, `cdecl`, one word) and the string array's grow (`0096ad30`) and set
/// (`0096ae90`), `thiscall`.
const SHADER_PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00ba_9890;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const ARRAY_SET_SIZE_0096AD30: u32 = 0x0096_ad30;
const ARRAY_SET_AT_0096AE90: u32 = 0x0096_ae90;
/// The target array's `RemoveAll` (`004dffa0`, engine map), `Compact` (`004afc80`, engine
/// map), add (`00a5eb20`, unit `ninode.cpp`; one word: the address of an `NiPointer`) and the
/// element allocation `NiTNewInterface<NiPointer<NiAVObject>>::Allocate` (`005e0ba0`, `cdecl`,
/// one word: the count); its vtables while constructed (`0101fbcc`) and after (`0101fbd4`).
const TARGET_ARRAY_REMOVE_ALL: u32 = 0x004d_ffa0;
const TARGET_ARRAY_COMPACT: u32 = 0x004a_fc80;
const TARGET_ARRAY_ADD: u32 = 0x00a5_eb20;
const POINTER_ARRAY_ALLOCATE: u32 = 0x005e_0ba0;
const TARGET_ARRAY_VTABLE_0101FBCC: u32 = 0x0101_fbcc;
const TARGET_ARRAY_VTABLE_0101FBD4: u32 = 0x0101_fbd4;
/// The compiler's `eh vector destructor iterator` (`00ec5fce`) and the `NiPointer` destructor
/// (`0045cec0`).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const POINTER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// `NiAVObject::GetProperty` (`00a59d30`, `thiscall`, one word).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// `strncmp` (`00ec8a19`), `"Bip"` (`010205ac`), the null name handle word (`0109b220`) and
/// `"Bip01"` (`0101e460`, compared inline).
const STRNCMP: u32 = 0x00ec_8a19;
const BIP_PREFIX_010205AC: u32 = 0x0102_05ac;
const NULL_NAME_0109B220: u32 = 0x0109_b220;
const BIP01_NAME_0101E460: u32 = 0x0101_e460;
/// `NiTriBasedGeom::ms_RTTI` (`011f5b70`) and `BSShaderLightingProperty::ms_RTTI`
/// (`011fa560`; the static initializers `00fb5ff0` and `00fb9a40` name them).
const TRI_BASED_GEOM_RTTI: u32 = 0x011f_5b70;
const LIGHTING_PROPERTY_RTTI: u32 = 0x011f_a560;
/// The shared instance-index block (`01200420`) and the number of particle properties that
/// use it (`01200424`).
const INSTANCE_INDEX_01200420: u32 = 0x0120_0420;
const INSTANCE_USERS_01200424: u32 = 0x0120_0424;
/// The function-local static `BSRandom` (`011c4180`), its guard bit (`011c4b4c`), its
/// constructor (`00aa5140`, `thiscall`) and `BSRandom::UnsignedInt` (`00aa5230`, `thiscall`,
/// one word: the bound).
const BS_RANDOM_011C4180: u32 = 0x011c_4180;
const BS_RANDOM_GUARD_011C4B4C: u32 = 0x011c_4b4c;
const BS_RANDOM_CONSTRUCT: u32 = 0x00aa_5140;
const BS_RANDOM_UNSIGNED_INT: u32 = 0x00aa_5230;
/// The constants the particle code reads: `float` `FLT_MAX` (`01016970`), `float` -10000
/// (`0107cc20`), `float` 2^32 (`01099378`), `double` 2^-32 (`010229c0`), `double` pi/180
/// (`01023128`), `double` 100 (`01017a40`), `double` 32767 (`01029790`), `double` 1
/// (`01012070`) and `double` `FLT_MAX` (`010231b0`).
const FLOAT_MAX_01016970: u32 = 0x0101_6970;
const UNUSED_BIRTH_0107CC20: u32 = 0x0107_cc20;
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const TWO_TO_MINUS_32_010229C0: u32 = 0x0102_29c0;
const DEGREES_TO_RADIANS_01023128: u32 = 0x0102_3128;
const HUNDRED_01017A40: u32 = 0x0101_7a40;
const RAND_MAX_01029790: u32 = 0x0102_9790;
const ONE_01012070: u32 = 0x0101_2070;
const DOUBLE_MAX_010231B0: u32 = 0x0102_31b0;
/// The zero point (`011f426c`, three floats) the emitters start from.
const ZERO_POINT_011F426C: u32 = 0x011f_426c;
/// The point helpers: `004a0c10` (unit `bgsdecalmanager.cpp`; `thiscall` on a point, no
/// words: it normalises the point), `004b4500` (unit `globalfunc.cpp`) and `00524c40` (unit
/// `tesobjectweap.cpp`), both `thiscall` on the object's `+0x68` with two words (the result and
/// a point) answering the result.
const POINT_UNITIZE_004A0C10: u32 = 0x004a_0c10;
const POINT_ROTATE_004B4500: u32 = 0x004b_4500;
const POINT_TRANSFORM_00524C40: u32 = 0x0052_4c40;
/// The geometry data's lock (`00a67360`, one word), its two stream getters (`00a67410`,
/// `00a675b0`, one word: a three-word block) and unlock (`00a673d0`), unit `nigeometrydata.cpp`,
/// `thiscall`.
const GEOMETRY_DATA_LOCK: u32 = 0x00a6_7360;
const GEOMETRY_DATA_POSITIONS: u32 = 0x00a6_7410;
const GEOMETRY_DATA_NORMALS: u32 = 0x00a6_75b0;
const GEOMETRY_DATA_UNLOCK: u32 = 0x00a6_73d0;

/// Releases an `NiPointer`'s object (`InterlockedDecrement` on `+4`, its slot +4 at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

fn particle_count(e: &mut Engine) -> u32 {
    e.call(PARTICLE_COUNT_00BA52D0, &args![]).u32()
}

fn rand(e: &mut Engine) -> i32 {
    e.call(RAND, &args![]).i32()
}

/// `FILD r; FADD ST0,ST0; FDIV 32767; FSUB 1.0; FSTP float`: a random word in [-1, 1].
fn signed_unit(e: &mut Engine, r: i32) -> u32 {
    let k: f64 = e.mem.f64(RAND_MAX_01029790);
    let one: f64 = e.mem.f64(ONE_01012070);
    let v = f64::from(r);
    x87_store32(x87_sub(x87_div(x87_add(v, v), k), one))
}

/// `FILD r; FDIV 32767; FSTP float`.
fn unit(e: &mut Engine, r: i32) -> u32 {
    let k: f64 = e.mem.f64(RAND_MAX_01029790);
    x87_store32(x87_div(f64::from(r), k))
}

/// Walks an `NiRTTI` chain (the parent at `+4`) from `rtti`: whether `wanted` is in it.
fn rtti_chain_has(e: &Engine, rtti: u32, wanted: u32) -> bool {
    let mut rtti = rtti;
    while rtti != 0 {
        if rtti == wanted {
            return true;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    false
}

/// Adds `object` to the target array through an `NiPointer` temporary (counted, then
/// released).
fn add_target(e: &mut Engine, this: Ptr<Particles>, object: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), object);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
        }
        e.call(TARGET_ARRAY_ADD, &args![this.addr() + 0x11c, slot]);
        if object != 0 {
            release(e, object);
        }
    });
}

/// The emitters' common tail: particle `index` gets the position, the birth time `fTimer`
/// and the velocity (each `FLD`/`FSTP`, the instance data re-read for each word).
fn store_particle(
    e: &mut Engine,
    this: Ptr<Particles>,
    index: u32,
    position: [u32; 3],
    velocity: [u32; 3],
) {
    let offset = index.wrapping_mul(0x30);
    let words = [
        position[0],
        position[1],
        position[2],
        e.get(this, Particles::fTimer),
        velocity[0],
        velocity[1],
        velocity[2],
    ];
    for (slot, word) in (0u32..).zip(words) {
        let data = e.get(this, Particles::pInstanceData);
        e.mem.set_u32(
            data.wrapping_add(offset).wrapping_add(4 * slot),
            x87_quiet_bits(word),
        );
    }
}

fn zero_point(e: &Engine) -> [u32; 3] {
    [
        e.mem.u32(ZERO_POINT_011F426C),
        e.mem.u32(ZERO_POINT_011F426C + 4),
        e.mem.u32(ZERO_POINT_011F426C + 8),
    ]
}

fn read_point(e: &Engine, at: u32) -> [u32; 3] {
    [
        e.mem.u32(at),
        e.mem.u32(at.wrapping_add(4)),
        e.mem.u32(at.wrapping_add(8)),
    ]
}

fn write_point(e: &mut Engine, at: u32, point: [u32; 3]) {
    for (i, word) in (0u32..).zip(point) {
        e.mem.set_u32(at + 4 * i, word);
    }
}

/// A random entry of the target array: `_rand` modulo the 16-bit count (`+0x126`, unsigned
/// `DIV`), again until the entry is not null and `accept` takes it.
fn random_target(
    e: &mut Engine,
    this: Ptr<Particles>,
    accept: impl Fn(&Engine, u32) -> bool,
) -> u32 {
    loop {
        let r = rand(e) as u32;
        let count = u32::from(e.mem.u16(this.addr() + 0x126));
        let array = e.mem.u32(this.addr() + 0x120);
        let target = e.mem.u32(array.wrapping_add((r % count).wrapping_mul(4)));
        if target != 0 && accept(e, target) {
            return target;
        }
    }
}

/// The point `u * parent + t * node` the skin and node emitters compute from the world
/// translations (`+0x8c`) of a node and its parent, with `t = rand / 32767`, `u = 1 - t`, in
/// the exe's order (`FMULP ST2`/`FMUL ST1`/`FMUL m32`, then `FADD m32` of the stored words).
fn blend_translations(e: &mut Engine, node: u32, parent: u32) -> [u32; 3] {
    let r = rand(e);
    let t_bits = unit(e, r);
    let t = x87_load32(t_bits);
    let u_bits = x87_store32(x87_sub(1.0, t));
    let u = x87_load32(u_bits);
    let px = x87_store32(x87_mul(x87_load32(e.mem.u32(parent + 0x8c)), u));
    let py = x87_store32(x87_mul(x87_load32(e.mem.u32(parent + 0x90)), u));
    let pz = x87_store32(x87_mul(u, x87_m32(e.mem.u32(parent + 0x94))));
    let qx = x87_store32(x87_mul(x87_load32(e.mem.u32(node + 0x8c)), t));
    let qy = x87_store32(x87_mul(x87_load32(e.mem.u32(node + 0x90)), t));
    let qz = x87_store32(x87_mul(t, x87_m32(e.mem.u32(node + 0x94))));
    [
        x87_store32(x87_add(x87_load32(qx), x87_m32(px))),
        x87_store32(x87_add(x87_load32(qy), x87_m32(py))),
        x87_store32(x87_add(x87_load32(qz), x87_m32(pz))),
    ]
}

/// A point helper on the object's `+0x68` (`004b4500` or `00524c40`) with a result block and
/// `point`; answers the three words its result points to.
fn point_helper(e: &mut Engine, helper: u32, object: u32, point: [u32; 3]) -> [u32; 3] {
    e.with_stack(0x18, |e, block| {
        let input = block.addr();
        let output = block.addr() + 0xc;
        write_point(e, input, point);
        let result = e
            .call(helper, &args![object.wrapping_add(0x68), output, input])
            .u32();
        read_point(e, result)
    })
}

/// `004a0c10` on a stack copy of `point`; answers the normalised point.
fn unitize(e: &mut Engine, point: [u32; 3]) -> [u32; 3] {
    e.with_stack(0xc, |e, at| {
        write_point(e, at.addr(), point);
        e.call(POINT_UNITIZE_004A0C10, &args![at]);
        read_point(e, at.addr())
    })
}

/// The `BSRandom` of `InitParticleData`, constructed on first use.
fn bs_random(e: &mut Engine, bound: u32) -> u32 {
    let guard = e.mem.u32(BS_RANDOM_GUARD_011C4B4C);
    if guard & 1 == 0 {
        e.mem.set_u32(BS_RANDOM_GUARD_011C4B4C, guard | 1);
        e.call(BS_RANDOM_CONSTRUCT, &args![BS_RANDOM_011C4180]);
    }
    e.call(BS_RANDOM_UNSIGNED_INT, &args![BS_RANDOM_011C4180, bound])
        .u32()
}

/// `FILD` of an unsigned word: loaded signed, 2^32 (`01099378`) added when negative.
fn load_unsigned(e: &Engine, value: u32) -> f64 {
    let loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        x87_add(loaded, x87_m32(e.mem.u32(TWO_TO_32_01099378)))
    } else {
        loaded
    }
}

/// `low + (high - low) * random * 2^-32` as `InitParticleData` computes it: `FLD high; FLD low;
/// FLD ST0; FSUBP ST2; FILD r (+2^32); FMULP ST2; FXCH; FMUL 2^-32; FADDP`.
fn random_between(e: &Engine, low: u32, high: u32, random: u32) -> f64 {
    let high = x87_load32(high);
    let low = x87_load32(low);
    let span = x87_sub(high, low);
    let scaled = x87_mul(span, load_unsigned(e, random));
    let scaled = x87_mul(scaled, e.mem.f64(TWO_TO_MINUS_32_010229C0));
    x87_add(low, scaled)
}

// Translated from 00ba6e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParticleShaderProperty::InitParticleData` (Xbox PDB; `thiscall`, one float: the time).
/// With `a = fLifeVar / fLifetime` and `b = fNormalSpeedVar / fNormalSpeed` (each stored as a
/// `float`), `fTimer` becomes the time; then every particle is made unused (position
/// `FLT_MAX`, birth -10000, velocity 0) and gets, from the static `BSRandom` (`011c4180`, made
/// on first use; `UnsignedInt` with the bound -1 unless said): `fLifeAdjust` between `1 - a`
/// and `1 + a`; the rotation start and speed between `fRotate - fRotateVar` and `fRotate +
/// fRotateVar` (and the speed's pair) in degrees, times pi/180; `fSpeedMult` between `1 - b`
/// and `1 + b`; and `fTexCoords` from two draws bounded by `fTextureCountX` and `fTextureCountY`
/// (each truncated by `_ftol2`): `(y / countY) * 100 + x / countX`. `iParticleCount` becomes 0.
///
/// The exception frame (`FS:[0]`, for the `BSRandom` static) is not translated.
// Translation incomplete: 00ba6e00 - the FS:[0] exception frame (unwinding the static BSRandom's construction) is not translated
pub fn particle_shader_property_init_particle_data(
    e: &mut Engine,
    this: Ptr<Particles>,
    time: f32,
) {
    let count = particle_count(e);
    let a = x87_store32(x87_div(
        x87_load32(e.get(this, Particles::fLifeVar)),
        x87_m32(e.get(this, Particles::fLifetime)),
    ));
    let b = x87_store32(x87_div(
        x87_load32(e.get(this, Particles::fNormalSpeedVar)),
        x87_m32(e.get(this, Particles::fNormalSpeed)),
    ));
    e.set(this, Particles::fTimer, x87_quiet_bits(time.to_bits()));
    if count == 0 {
        e.set(this, Particles::iParticleCount, 0);
        return;
    }
    // FLD a; FLD ST0; FLD1; FLD ST0; FSUBRP ST2; FXCH; FSTP (1 - a); FADD ST1; FXCH; FSTP
    // (a + 1); then the same for b with FSUBR ST0,ST2 and FADDP.
    let a = x87_load32(a);
    let life_low = x87_store32(x87_sub(1.0, a));
    let life_high = x87_store32(x87_add(a, 1.0));
    let b = x87_load32(b);
    let speed_low = x87_store32(x87_sub(1.0, b));
    let speed_high = x87_store32(x87_add(1.0, b));
    let max = x87_store32(x87_load32(e.mem.u32(FLOAT_MAX_01016970)));
    let unused_birth = x87_store32(x87_load32(e.mem.u32(UNUSED_BIRTH_0107CC20)));
    let degrees: f64 = e.mem.f64(DEGREES_TO_RADIANS_01023128);
    let hundred: f64 = e.mem.f64(HUNDRED_01017A40);
    let mut offset = 0u32;
    for _ in 0..count {
        let set = |e: &mut Engine, field: u32, value: u32| {
            let data = e.get(this, Particles::pInstanceData);
            e.mem
                .set_u32(data.wrapping_add(offset).wrapping_add(field), value);
        };
        set(e, 0, max);
        set(e, 4, max);
        set(e, 8, max);
        set(e, 0xc, unused_birth);
        set(e, 0x10, 0);
        set(e, 0x14, 0);
        set(e, 0x18, 0);

        let random = bs_random(e, 0xffff_ffff);
        let life = x87_store32(random_between(e, life_low, life_high, random));
        set(e, 0x1c, x87_quiet_bits(life));

        for (field, value, variance) in [
            (0x20u32, Particles::fRotate, Particles::fRotateVar),
            (0x24, Particles::fRotateSpeed, Particles::fRotateSpeedVar),
        ] {
            let middle = e.get(this, value);
            let spread = e.get(this, variance);
            let low = x87_store32(x87_sub(x87_load32(middle), x87_m32(spread)));
            let high = x87_store32(x87_add(x87_load32(spread), x87_m32(middle)));
            let random = bs_random(e, 0xffff_ffff);
            let angle = x87_store32(random_between(e, low, high, random));
            let radians = x87_store32(x87_mul(x87_load32(angle), degrees));
            set(e, field, x87_quiet_bits(radians));
        }

        let random = bs_random(e, 0xffff_ffff);
        let speed = x87_store32(random_between(e, speed_low, speed_high, random));
        set(e, 0x28, x87_quiet_bits(speed));

        let count_x = e.get(this, Particles::fTextureCountX);
        let bound_x = e.call(FTOL, &args![x87_load32(count_x)]).u32();
        let x = bs_random(e, bound_x);
        let x = x87_store32(f64::from(x as i32));
        let count_y = e.get(this, Particles::fTextureCountY);
        let bound_y = e.call(FTOL, &args![x87_load32(count_y)]).u32();
        let y = bs_random(e, bound_y);
        let y = x87_store32(f64::from(y as i32));
        let y = x87_store32(x87_div(
            x87_load32(y),
            x87_m32(e.get(this, Particles::fTextureCountY)),
        ));
        let row = x87_mul(x87_load32(y), hundred);
        let column = x87_store32(x87_div(
            x87_load32(x),
            x87_m32(e.get(this, Particles::fTextureCountX)),
        ));
        let coords = x87_store32(x87_add(row, x87_m32(column)));
        set(e, 0x2c, x87_quiet_bits(coords));
        offset = offset.wrapping_add(0x30);
    }
    e.set(this, Particles::iParticleCount, 0);
}

// Translated from 00ba71d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the emitter of type 2 (`thiscall`, one word: the particle
/// index). Without targets (`+0x126`) the particle starts at the zero point (`011f426c`) with
/// it as velocity too. With a first target: the position is `(0, fRayLength * rand/32767,
/// 0)` and the velocity `(x, 0, z)` from two more draws in [-1, 1] (`z` first), normalised
/// (`004a0c10`); in world space (`bWorldspace`) both go through the target's `+0x68`
/// (`00524c40` for the position, `004b4500` for the velocity). Then the particle is stored
/// with the birth time `fTimer`.
///
/// With a null first target the exe stores its stack words uninitialised; the translation's
/// stack block starts zeroed instead.
// Translation incomplete: 00ba71d0 - with a null first target the exe stores uninitialised stack words (zero here)
pub fn fn_00ba71d0(e: &mut Engine, this: Ptr<Particles>, index: u32) {
    let mut position = [0u32; 3];
    let mut velocity = [0u32; 3];
    if e.mem.u16(this.addr() + 0x126) == 0 {
        position = zero_point(e);
        velocity = position;
    } else {
        let targets = e.mem.u32(this.addr() + 0x120);
        let first = e.mem.u32(targets);
        if first != 0 {
            let r = rand(e);
            let fraction = unit(e, r);
            let zero = x87_store32(0.0);
            let ray = x87_mul(
                x87_load32(e.get(this, Particles::fRayLength)),
                x87_m32(fraction),
            );
            position = [zero, x87_store32(ray), zero];
            let r = rand(e);
            let z = signed_unit(e, r);
            let r = rand(e);
            let x = signed_unit(e, r);
            velocity = unitize(e, [x87_quiet_bits(x), zero, x87_quiet_bits(z)]);
            if e.get(this, Particles::bWorldspace) != 0 {
                position = point_helper(e, POINT_TRANSFORM_00524C40, first, position);
                velocity = point_helper(e, POINT_ROTATE_004B4500, first, velocity);
            }
        }
    }
    store_particle(e, this, index, position, velocity);
}

// Translated from 00ba73a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParticleShaderProperty::GetInstanceIndex` (Xbox PDB; `cdecl`, no arguments): the shared
/// instance-index block (`01200420`), made on first use: `count * 4 * 4` bytes (all ones when
/// the size overflows) from the memory manager, each particle's four floats its index (`FILD`,
/// plus 2^32 when negative, stored as a `float`).
pub fn particle_shader_property_get_instance_index(e: &mut Engine) -> u32 {
    let block = e.mem.u32(INSTANCE_INDEX_01200420);
    if block != 0 {
        return block;
    }
    let count = particle_count(e);
    let size = u64::from(count.wrapping_mul(4)) * 4;
    let size = if size >> 32 != 0 {
        u32::MAX
    } else {
        size as u32
    };
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.mem.set_u32(INSTANCE_INDEX_01200420, block);
    let mut at = block.wrapping_add(8);
    let mut index = 0u32;
    while index < count {
        let value = x87_store32(load_unsigned(e, index));
        index += 1;
        at = at.wrapping_add(0x10);
        for back in [0x18u32, 0x14, 0x10, 0xc] {
            e.mem.set_u32(at.wrapping_sub(back), x87_quiet_bits(value));
        }
    }
    e.mem.u32(INSTANCE_INDEX_01200420)
}

// Translated from 00ba7420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: vtable slot +0xa0 (`GetRenderPasses` in the Xbox PDB's slot
/// list; `thiscall`, six words, only the geometry read). The pass list is made
/// (`CheckCreateRenderPassArray` with -1); when it is empty two passes are added for the
/// geometry, 0x41 then 0x241 (first-pass flag 1, no lights), the byte `+6` of the last pass
/// set to 0xe after the first and to 1 after the second. Answers the list.
pub fn fn_00ba7420(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![this, 0xffff_ffffu32]);
    let list = e.get(this, BSShaderProperty::pRenderPassList);
    if e.mem.u32(list.wrapping_add(0x10)) != 0 {
        return list;
    }
    for (pass, mark) in [(0x41u32, 0xeu8), (0x241, 1)] {
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        e.call(
            RENDER_PASS_ARRAY_ADD,
            &args![list, geometry, pass, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        let size = e.mem.u32(list.wrapping_add(0x10));
        let passes = e.mem.u32(list.wrapping_add(4));
        let last = e
            .mem
            .u32(passes.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
        e.mem.set_u8(last.wrapping_add(6), mark);
    }
    e.get(this, BSShaderProperty::pRenderPassList)
}

// Translated from 00ba74a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the emitter of type 0 (`thiscall`, one word: the particle
/// index). The position and velocity start at the zero point (`011f426c`). With targets, a
/// random one with geometry data (`+0xb8`) is picked; then:
/// - with a skin instance (`+0xbc`): a random bone of it (its `+8`'s count `+0x44`, the bones at
///   its `+0x14`); the position is between the bone's and its parent's world translations
///   (`u * parent + t * bone`), the velocity a normalised `(x, 0, z)` from two draws in [-1, 1]
///   put through the target's `+0x68` (`004b4500`);
/// - otherwise a random vertex (below the data's 16-bit count `+8`): from the locked streams
///   when the data's `+0x30` object answers true to its slot +0x8c and the lock (`00a67360` with
///   1) succeeds (`00a67410` and `00a675b0` fill a pointer, stride and flag each; `00a673d0`
///   unlocks), else from the data's position (`+0x20`) and normal (`+0x24`) arrays (the zero
///   point for a null one); in world space both go through the target's `+0x68`
///   (`00524c40`, `004b4500`).
///
/// Then the particle is stored with the birth time `fTimer`. The two stream blocks are
/// cleared on the stack except the upper bytes of their flag words, which the exe leaves
/// uninitialised (zero here).
// Translation incomplete: 00ba74a0 - the stream blocks' flag words keep three uninitialised stack bytes in the exe (zero here)
pub fn fn_00ba74a0(e: &mut Engine, this: Ptr<Particles>, index: u32) {
    let mut position = zero_point(e);
    let mut velocity = position;
    if e.mem.u16(this.addr() + 0x126) == 0 {
        store_particle(e, this, index, position, velocity);
        return;
    }
    let target = random_target(e, this, |e, target| e.mem.u32(target + 0xb8) != 0);
    let skin = e.mem.u32(target + 0xbc);
    if skin != 0 {
        let bones = e.mem.u32(e.mem.u32(skin + 8) + 0x44);
        let r = rand(e) as u32;
        let bone_list = e.mem.u32(skin + 0x14);
        let bone = e
            .mem
            .u32(bone_list.wrapping_add((r % bones).wrapping_mul(4)));
        let parent = e.mem.u32(bone + 0x18);
        position = blend_translations(e, bone, parent);
        let r = rand(e);
        let z = signed_unit(e, r);
        let r = rand(e);
        let x = signed_unit(e, r);
        let direction = unitize(e, [x87_quiet_bits(x), x87_store32(0.0), x87_quiet_bits(z)]);
        velocity = point_helper(e, POINT_ROTATE_004B4500, target, direction);
        store_particle(e, this, index, position, velocity);
        return;
    }
    let vertices = u32::from(e.mem.u16(e.mem.u32(target + 0xb8) + 8));
    let r = rand(e) as u32;
    let vertex = r % vertices;
    let data = e.mem.u32(target + 0xb8);
    if data == 0 {
        store_particle(e, this, index, position, velocity);
        return;
    }
    let streams = e.mem.u32(data + 0x30);
    if streams != 0 && e.vcall(streams, 0x8c, &args![]).bool() {
        e.with_stack(0x18, |e, blocks| {
            let positions = blocks.addr();
            let normals = blocks.addr() + 0xc;
            if e.call(GEOMETRY_DATA_LOCK, &args![data, 1u32]).bool() {
                e.call(GEOMETRY_DATA_POSITIONS, &args![data, positions]);
                e.call(GEOMETRY_DATA_NORMALS, &args![data, normals]);
                let at = e
                    .mem
                    .u32(positions + 4)
                    .wrapping_mul(vertex)
                    .wrapping_add(e.mem.u32(positions));
                position = read_point(e, at);
                let at = e
                    .mem
                    .u32(normals + 4)
                    .wrapping_mul(vertex)
                    .wrapping_add(e.mem.u32(normals));
                velocity = read_point(e, at);
                e.call(GEOMETRY_DATA_UNLOCK, &args![data]);
            }
        });
    } else {
        let from = |array: u32| {
            if array != 0 {
                array.wrapping_add(vertex.wrapping_mul(0xc))
            } else {
                ZERO_POINT_011F426C
            }
        };
        let at = from(e.mem.u32(data + 0x20));
        position = read_point(e, at);
        let at = from(e.mem.u32(data + 0x24));
        velocity = read_point(e, at);
    }
    if e.get(this, Particles::bWorldspace) != 0 {
        position = point_helper(e, POINT_TRANSFORM_00524C40, target, position);
        velocity = point_helper(e, POINT_ROTATE_004B4500, target, velocity);
    }
    store_particle(e, this, index, position, velocity);
}

// Translated from 00ba7860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the emitter of type 1 (`thiscall`, one word: the particle
/// index). Without targets the particle starts at the zero point (`011f426c`) with it as
/// velocity too. Otherwise a random non-null target is picked: with a 16-bit word at `+0xa8`
/// and a parent (`+0x18`) the position is between their world translations (`u * parent + t *
/// target`) and the velocity a normalised `(x, 0, z)` from two draws put through the target's
/// `+0x68` (`004b4500`); otherwise the position is the target's world translation (`+0x8c`)
/// and the velocity a normalised `(x, y, z)` from three draws (`z`, `y`, `x` in that order).
/// Then the particle is stored with the birth time `fTimer`.
pub fn fn_00ba7860(e: &mut Engine, this: Ptr<Particles>, index: u32) {
    let position;
    let velocity;
    if e.mem.u16(this.addr() + 0x126) == 0 {
        position = zero_point(e);
        velocity = position;
    } else {
        let target = random_target(e, this, |_, _| true);
        let parent = e.mem.u32(target + 0x18);
        if e.mem.u16(target + 0xa8) != 0 && parent != 0 {
            position = blend_translations(e, target, parent);
            let r = rand(e);
            let z = signed_unit(e, r);
            let r = rand(e);
            let x = signed_unit(e, r);
            let direction = unitize(e, [x87_quiet_bits(x), x87_store32(0.0), x87_quiet_bits(z)]);
            velocity = point_helper(e, POINT_ROTATE_004B4500, target, direction);
        } else {
            position = read_point(e, target + 0x8c);
            let r = rand(e);
            let z = signed_unit(e, r);
            let r = rand(e);
            let y = signed_unit(e, r);
            let r = rand(e);
            let x = signed_unit(e, r);
            velocity = unitize(e, [x87_quiet_bits(x), x87_quiet_bits(y), x87_quiet_bits(z)]);
        }
    }
    store_particle(e, this, index, position, velocity);
}

/// Whether the C string at `name` is `"Bip01"` (`0101e460`; the code's inline `strcmp`, two
/// bytes a step).
fn is_bip01(e: &Engine, name: u32) -> bool {
    let mut offset = 0u32;
    loop {
        let a = e.mem.u8(name.wrapping_add(offset));
        let b = e.mem.u8(BIP01_NAME_0101E460 + offset);
        if a != b {
            return false;
        }
        if a == 0 {
            return true;
        }
        offset += 1;
    }
}

// Translated from 00ba7b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParticleShaderProperty::UpdateParticles` (Xbox PDB; `thiscall`, three words: the time and
/// two flag bytes, `follow` and `emit`).
/// - `dt = |time - fTimer|` (stored as `float`s); the most particles alive is the count
///   (`00ba52d0`) or, when smaller, `fMaxParticleRatio * count` (`_ftol2`); `fGenerateRemainder +
///   dt / fLifetime * most` gives how many to make (`_ftol2`) and the new remainder (the rest).
/// - With `follow`: the first ancestor of `pMyGeometry` (through `+0x18`) named `"Bip01"` gives
///   the offset of its world translation (`+0x8c`) from its parent's.
/// - Each particle whose age (`time - fBirthTime`) is below 0 or whose `age * fLifeAdjust`
///   exceeds `fLifetime` (a NaN keeps it) is made unused when it is not already (position
///   `FLT_MAX`, birth -10000, velocity 0, `iParticleCount` down by one); then, with `emit`,
///   particles left to make and `iParticleCount` below the most, the emitter of `eEmitterType`
///   (0: `fn_00ba74a0`, 1: `fn_00ba7860`, 2: `fn_00ba71d0`, others none) remakes it, the count
///   to make goes down and `iParticleCount` up, and with `follow` the offset is subtracted from
///   its position.
/// - `fTimer` becomes the time.
pub fn particle_shader_property_update_particles(
    e: &mut Engine,
    this: Ptr<Particles>,
    time: f32,
    follow: u8,
    emit: u8,
) {
    let time_bits = time.to_bits();
    let dt = x87_store32(x87_sub(
        x87_load32(time_bits),
        x87_m32(e.get(this, Particles::fTimer)),
    ));
    let dt = x87_store32(x87_load32(dt).abs());
    let dt = x87_quiet_bits(dt);
    let count = particle_count(e) as i32;
    let ratio = x87_mul(
        x87_load32(e.get(this, Particles::fMaxParticleRatio)),
        f64::from(count),
    );
    let ratio_count = e.call(FTOL, &args![ratio]).i32();
    let most = if count < ratio_count {
        count
    } else {
        ratio_count
    };
    let generate = x87_div(x87_load32(dt), x87_m32(e.get(this, Particles::fLifetime)));
    let generate = x87_mul(generate, f64::from(most));
    let generate = x87_add(
        generate,
        x87_m32(e.get(this, Particles::fGenerateRemainder)),
    );
    let generate = x87_load32(x87_store32(generate));
    let mut to_make = e.call(FTOL, &args![generate]).i32();
    let remainder = x87_store32(x87_sub(generate, f64::from(to_make)));
    e.set(this, Particles::fGenerateRemainder, remainder);

    let mut offset = zero_point(e);
    if follow != 0 {
        let geometry = e.get(this, Particles::pMyGeometry);
        let mut node = e.mem.u32(geometry + 0x18);
        while node != 0 {
            let name = e.mem.u32(node + 8);
            if is_bip01(e, name) {
                let parent = e.mem.u32(node + 0x18);
                for (i, field) in [0x8cu32, 0x90, 0x94].into_iter().enumerate() {
                    let difference = x87_sub(
                        x87_load32(e.mem.u32(node + field)),
                        x87_m32(e.mem.u32(parent + field)),
                    );
                    offset[i] = x87_store32(difference);
                }
                break;
            }
            node = e.mem.u32(node + 0x18);
        }
    }

    let max = x87_load32(e.mem.u32(FLOAT_MAX_01016970));
    let unused_birth = x87_store32(x87_load32(e.mem.u32(UNUSED_BIRTH_0107CC20)));
    let double_max: f64 = e.mem.f64(DOUBLE_MAX_010231B0);
    let mut index = 0i32;
    let mut at = 0u32;
    while index < count {
        let data = e.get(this, Particles::pInstanceData);
        let particle = data.wrapping_add(at);
        let age = x87_store32(x87_sub(
            x87_load32(time_bits),
            x87_m32(e.mem.u32(particle + 0xc)),
        ));
        let age = x87_load32(age);
        // FCOM 0.0 / TEST AH,5 / JNP: below 0 (ordered) dies; FCOMPP / TEST AH,5 / JP: the
        // lifetime not below the adjusted age (or unordered) lives.
        let dies = if age < 0.0 {
            true
        } else {
            let adjusted = x87_mul(age, x87_m32(e.mem.u32(particle + 0x1c)));
            let lifetime = x87_load32(e.get(this, Particles::fLifetime));
            lifetime < adjusted
        };
        if dies {
            if x87_load32(e.mem.u32(particle)) != double_max {
                let max_bits = x87_store32(max);
                e.mem.set_u32(particle, max_bits);
                for (field, value) in [
                    (4u32, max_bits),
                    (8, max_bits),
                    (0xc, unused_birth),
                    (0x10, 0),
                    (0x14, 0),
                    (0x18, 0),
                ] {
                    let data = e.get(this, Particles::pInstanceData);
                    e.mem
                        .set_u32(data.wrapping_add(at).wrapping_add(field), value);
                }
                let alive = e.get(this, Particles::iParticleCount);
                e.set(this, Particles::iParticleCount, alive.wrapping_sub(1));
            }
            if emit != 0 && to_make > 0 && e.get(this, Particles::iParticleCount) < most {
                match e.get(this, Particles::eEmitterType) {
                    0 => fn_00ba74a0(e, this, index as u32),
                    1 => fn_00ba7860(e, this, index as u32),
                    2 => fn_00ba71d0(e, this, index as u32),
                    _ => {}
                }
                to_make = to_make.wrapping_sub(1);
                let alive = e.get(this, Particles::iParticleCount);
                e.set(this, Particles::iParticleCount, alive.wrapping_add(1));
                if follow != 0 {
                    for (i, word) in offset.into_iter().enumerate() {
                        let data = e.get(this, Particles::pInstanceData);
                        let slot = data.wrapping_add(at).wrapping_add(4 * i as u32);
                        let moved = x87_sub(x87_load32(e.mem.u32(slot)), x87_m32(word));
                        e.mem.set_u32(slot, x87_store32(moved));
                    }
                }
            }
        }
        index += 1;
        at = at.wrapping_add(0x30);
    }
    e.set(this, Particles::fTimer, x87_quiet_bits(time_bits));
}

// Translated from 00ba7df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor body (`thiscall`). The vtable `010b8250`; the base
/// texture released and cleared; the target array emptied (`004dffa0`); the instance data
/// given back to the memory manager; the users of the shared instance-index block counted
/// down and the block given back and cleared at zero; then the target array's destruction
/// (its vtable `0101fbcc`, the compiler's array destructor over its elements, the count kept
/// in the word before them, and that block given back); the base texture released again
/// when set; then `BSShaderProperty`'s destructor body (`00baa0e0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba7df0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00ba7df0(e: &mut Engine, this: Ptr<Particles>) {
    let base = this.addr();
    e.mem.set_u32(base, PARTICLE_PROPERTY_VTABLE);
    let texture = e.get(this, Particles::spBaseTexture);
    if texture != 0 {
        release(e, texture);
        e.set(this, Particles::spBaseTexture, 0);
    }
    let array = base + 0x11c;
    e.call(TARGET_ARRAY_REMOVE_ALL, &args![array]);
    let data = e.get(this, Particles::pInstanceData);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, data]);
    let users = e.mem.u32(INSTANCE_USERS_01200424).wrapping_sub(1);
    e.mem.set_u32(INSTANCE_USERS_01200424, users);
    if users == 0 {
        let block = e.mem.u32(INSTANCE_INDEX_01200420);
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
        e.mem.set_u32(INSTANCE_INDEX_01200420, 0);
    }
    e.mem.set_u32(array, TARGET_ARRAY_VTABLE_0101FBCC);
    let elements = e.mem.u32(array + 4);
    if elements != 0 {
        let count = e.mem.u32(elements.wrapping_sub(4));
        e.call(
            VECTOR_DESTRUCT,
            &args![elements, 4u32, count, POINTER_DESTRUCT_0045CEC0],
        );
        e.call(
            MEMORY_DEALLOCATE,
            &args![MEMORY_MANAGER, elements.wrapping_sub(4)],
        );
    }
    let texture = e.get(this, Particles::spBaseTexture);
    if texture != 0 {
        release(e, texture);
    }
    e.call(SHADER_PROPERTY_DESTROY, &args![base]);
}

// Translated from 00ba7f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: `GetRTTI` (vtable slot +8): answers the RTTI record
/// `01200434`.
pub fn fn_00ba7f10(_e: &mut Engine, _this: Ptr<Particles>) -> u32 {
    PARTICLE_PROPERTY_RTTI
}

// Translated from 00ba7f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot +0x60 (`GetViewerStrings` in the Xbox PDB's slot
/// list; `thiscall`, one word: the string array). `BSShaderProperty::GetViewerStrings` runs;
/// then the viewer string of the class name (the name word of `01200434`) is added at the
/// array's end (grown by `m_usGrowBy` first when full).
pub fn fn_00ba7f20(e: &mut Engine, this: Ptr<Particles>, strings: u32) {
    e.call(SHADER_PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(PARTICLE_PROPERTY_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    let array: Ptr<NiTArray> = Ptr::new(strings);
    let size = u32::from(e.get(array, NiTArray::m_usSize));
    let capacity = u32::from(e.get(array, NiTArray::m_usMaxSize));
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), text);
        if size >= capacity {
            let grow = u32::from(e.get(array, NiTArray::m_usGrowBy));
            e.call(
                ARRAY_SET_SIZE_0096AD30,
                &args![strings, grow.wrapping_add(size)],
            );
        }
        e.call(ARRAY_SET_AT_0096AE90, &args![strings, size, element]);
    });
}

// Translated from 00ba7f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor (`thiscall`, no words). `BSShaderProperty`'s
/// constructor (`00ba8e50`); the vtable `010b8250`; the colours (`+0xc4`..`+0xf0`) 0.0; no base
/// texture; the target array (vtable `0101fbcc`, capacity and grow-by 10, size and count 0, ten
/// elements from `005e0ba0`, then the vtable `0101fbd4`); the base texture released when set
/// (it is not) and cleared; then the members: `iParticleCount` 0, `fGenerateRemainder` 0.0,
/// `eEmitterType` 0, `bWorldspace` true, `fMaxParticleRatio` 1.0, `fTimer` and the floats
/// `+0x7c`..`+0xb4` 0.0, the colours again 0.0, `+0x130`..`+0x138` 1.0 and `+0x13c`..`+0x148`
/// 0.0 (the PC offsets of the Xbox PDB's `fAlpha`, texture counts, `WindPoint` and
/// `fWindSpeed`); the target array emptied (`004dffa0`); the shared instance-index block made
/// (`fn_00ba73a0`); no geometry; the instance data (`count * 0x30` bytes, all ones when the
/// size overflows, from the memory manager, cleared by `_memset`); one more user of the
/// shared block; `iShader` 0x18 and `iShaderPropertyType` 0x11. Answers `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba7f70 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00ba7f70(e: &mut Engine, this: Ptr<Particles>) -> Ptr<Particles> {
    let base = this.addr();
    e.call(SHADER_PROPERTY_CONSTRUCT, &args![base]);
    let zero = x87_store32(0.0);
    let one = x87_store32(1.0);
    e.mem.set_u32(base, PARTICLE_PROPERTY_VTABLE);
    for offset in (0xc4u32..=0xf0).step_by(4) {
        e.mem.set_u32(base + offset, zero);
    }
    e.set(this, Particles::spBaseTexture, 0);
    let array = base + 0x11c;
    e.mem.set_u16(array + 8, 10);
    e.mem.set_u32(array, TARGET_ARRAY_VTABLE_0101FBCC);
    e.mem.set_u16(array + 0xe, 10);
    e.mem.set_u16(array + 0xa, 0);
    e.mem.set_u16(array + 0xc, 0);
    let elements = e.call(POINTER_ARRAY_ALLOCATE, &args![10u32]).u32();
    e.mem.set_u32(array + 4, elements);
    e.mem.set_u32(array, TARGET_ARRAY_VTABLE_0101FBD4);
    let texture = e.get(this, Particles::spBaseTexture);
    if texture != 0 {
        release(e, texture);
        e.set(this, Particles::spBaseTexture, 0);
    }
    e.set(this, Particles::iParticleCount, 0);
    e.set(this, Particles::fGenerateRemainder, zero);
    e.set(this, Particles::eEmitterType, 0);
    e.set(this, Particles::bWorldspace, 1);
    e.set(this, Particles::fMaxParticleRatio, one);
    e.set(this, Particles::fTimer, zero);
    for offset in (0x7cu32..=0xb4).step_by(4) {
        e.mem.set_u32(base + offset, zero);
    }
    for offset in (0xc4u32..=0xf0).step_by(4) {
        e.mem.set_u32(base + offset, zero);
    }
    for offset in [0x130u32, 0x134, 0x138] {
        e.mem.set_u32(base + offset, one);
    }
    for offset in [0x13cu32, 0x140, 0x144, 0x148] {
        e.mem.set_u32(base + offset, zero);
    }
    e.call(TARGET_ARRAY_REMOVE_ALL, &args![array]);
    particle_shader_property_get_instance_index(e);
    e.set(this, Particles::pMyGeometry, 0);
    let count = particle_count(e);
    let size = u64::from(count) * 0x30;
    let size = if size >> 32 != 0 {
        u32::MAX
    } else {
        size as u32
    };
    let data = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.set(this, Particles::pInstanceData, data);
    e.call(MEMSET, &args![data, 0u32, count.wrapping_mul(0x30)]);
    let users = e.mem.u32(INSTANCE_USERS_01200424).wrapping_add(1);
    e.mem.set_u32(INSTANCE_USERS_01200424, users);
    let property: Ptr<BSShaderProperty> = this.cast();
    e.set(property, BSShaderProperty::iShader, 0x18);
    e.set(property, BSShaderProperty::iShaderPropertyType, 0x11);
    this
}

// Translated from 00ba8270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor (`thiscall`, one word: the
/// flags): `fn_00ba7df0`, then with bit 0 of the flags the 0x14c bytes freed (`00aa1460`).
/// Answers `this`.
pub fn fn_00ba8270(e: &mut Engine, this: Ptr<Particles>, flags: u8) -> Ptr<Particles> {
    fn_00ba7df0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x14cu32]);
    }
    this
}

// Translated from 00ba82a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot +0x48 (`CreateClone` in the Xbox PDB's slot list;
/// `thiscall`, one word: the cloning process). 0x14c bytes from `00aa13e0` constructed by
/// `fn_00ba7f70` (null when the allocation gave none) get `BSShaderProperty`'s members
/// (`00ba8b10` with the clone and the process; the particle fields are not copied). Answers
/// the clone.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba82a0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00ba82a0(e: &mut Engine, this: Ptr<Particles>, process: u32) -> u32 {
    let block: Ptr<Particles> = e.call(ALLOCATE_00AA13E0, &args![0x14cu32]).ptr();
    let clone = if block.is_null() {
        0
    } else {
        fn_00ba7f70(e, block).addr()
    };
    e.call(SHADER_PROPERTY_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

// Translated from 00ba8320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParticleShaderProperty::CollectGeometryRecursive` (Xbox PDB; `thiscall`, one word: a
/// scene-graph object). For a non-null object that is an `NiTriBasedGeom` (its RTTI chain):
/// when its property 3 (`GetProperty`) is a `BSShaderLightingProperty`, it is added to the
/// target array. Otherwise, for a node (slot +0xc) whose name (`+8`) is the null handle (the
/// word at `0109b220`) or does not start with `"Bip"` (`strncmp`, 3), each child (the `+0xa6`
/// 16-bit count of `+0xa0`) is collected in turn.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba8320 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn particle_shader_property_collect_geometry_recursive(
    e: &mut Engine,
    this: Ptr<Particles>,
    object: u32,
) {
    if object == 0 {
        return;
    }
    let rtti = e.vcall(object, 8, &args![]).u32();
    if rtti_chain_has(e, rtti, TRI_BASED_GEOM_RTTI) {
        let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
        if property == 0 {
            return;
        }
        let rtti = e.vcall(property, 8, &args![]).u32();
        if rtti_chain_has(e, rtti, LIGHTING_PROPERTY_RTTI) {
            add_target(e, this, object);
        }
        return;
    }
    let node = e.vcall(object, 0xc, &args![]).u32();
    if node == 0 {
        return;
    }
    let name = e.mem.u32(node + 8);
    if name != e.mem.u32(NULL_NAME_0109B220)
        && e.call(STRNCMP, &args![name, BIP_PREFIX_010205AC, 3u32])
            .u32()
            == 0
    {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node + 0xa6)) {
        let children = e.mem.u32(node + 0xa0);
        let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
        particle_shader_property_collect_geometry_recursive(e, this, child);
        index += 1;
    }
}

// Translated from 00ba8480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the target collection of emitter type 1 (`thiscall`, one word:
/// a scene-graph object). For a non-null object: when its word `+0x1c` is set it must be a
/// node (slot +0xc) and is added to the target array; then each child of a node (the `+0xa6`
/// 16-bit count of `+0xa0`) is collected in turn.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba8480 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00ba8480(e: &mut Engine, this: Ptr<Particles>, object: u32) {
    if object == 0 {
        return;
    }
    let flagged = e.mem.u32(object + 0x1c);
    let node = e.vcall(object, 0xc, &args![]).u32();
    if flagged != 0 {
        if node == 0 {
            return;
        }
        add_target(e, this, object);
    }
    if node == 0 {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node + 0xa6)) {
        let children = e.mem.u32(node + 0xa0);
        let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
        fn_00ba8480(e, this, child);
        index += 1;
    }
}

// Translated from 00ba8560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParticleShaderProperty::CollectTargets` (Xbox PDB; `thiscall`, one word: a scene-graph
/// object). The target array is emptied (`004dffa0`); by `eEmitterType`, 0 collects geometry
/// (`CollectGeometryRecursive`), 1 collects through `fn_00ba8480`, 2 adds the object itself
/// (through an `NiPointer` temporary, also when null); others collect nothing. Then the array
/// is compacted (`004afc80`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00ba8560 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn particle_shader_property_collect_targets(e: &mut Engine, this: Ptr<Particles>, object: u32) {
    let array = this.addr() + 0x11c;
    e.call(TARGET_ARRAY_REMOVE_ALL, &args![array]);
    match e.get(this, Particles::eEmitterType) {
        0 => particle_shader_property_collect_geometry_recursive(e, this, object),
        1 => fn_00ba8480(e, this, object),
        2 => add_target(e, this, object),
        _ => {}
    }
    e.call(TARGET_ARRAY_COMPACT, &args![array]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_6e00,
            particle_shader_property_init_particle_data(Ptr<Particles>, f32)
        ),
        entry!(0x00ba_71d0, fn_00ba71d0(Ptr<Particles>, u32)),
        entry!(0x00ba_73a0, particle_shader_property_get_instance_index() -> u32),
        entry!(
            0x00ba_7420,
            fn_00ba7420(Ptr<BSShaderProperty>, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00ba_74a0, fn_00ba74a0(Ptr<Particles>, u32)),
        entry!(0x00ba_7860, fn_00ba7860(Ptr<Particles>, u32)),
        entry!(
            0x00ba_7b60,
            particle_shader_property_update_particles(Ptr<Particles>, f32, u8, u8)
        ),
        entry!(0x00ba_7df0, fn_00ba7df0(Ptr<Particles>)),
        entry!(0x00ba_7f10, fn_00ba7f10(Ptr<Particles>) -> u32),
        entry!(0x00ba_7f20, fn_00ba7f20(Ptr<Particles>, u32)),
        entry!(0x00ba_7f70, fn_00ba7f70(Ptr<Particles>) -> Ptr<Particles>),
        entry!(
            0x00ba_8270,
            fn_00ba8270(Ptr<Particles>, u8) -> Ptr<Particles>
        ),
        entry!(0x00ba_82a0, fn_00ba82a0(Ptr<Particles>, u32) -> u32),
        entry!(
            0x00ba_8320,
            particle_shader_property_collect_geometry_recursive(Ptr<Particles>, u32)
        ),
        entry!(0x00ba_8480, fn_00ba8480(Ptr<Particles>, u32)),
        entry!(
            0x00ba_8560,
            particle_shader_property_collect_targets(Ptr<Particles>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const STUB: AbiFn = |_, _| Ret::default();
    const VTABLE: u32 = 0x0f00_1000;
    const DELETE: u32 = 0x0f00_0004;
    const GET_RTTI: u32 = 0x0f00_0008;
    const IS_NODE: u32 = 0x0f00_000c;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn f(bits: u32) -> f32 {
        f32::from_bits(bits)
    }

    fn set_text(e: &mut Engine, at: u32, text: &[u8]) {
        for (i, b) in (0u32..).zip(text) {
            e.mem.set_u8(at + i, *b);
        }
    }

    /// The constants mapped, reference counting, the shared doubles, a particle count of
    /// `count`, `_rand` answering `rands` in turn, objects with [`VTABLE`] (slot +4 deletes,
    /// slot +8 answers the word at +0x40, slot +0xc the word at +0x44) and the call log.
    fn rig(count: u32, rands: Vec<i32>) -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0102_2000,
            0x0102_3000,
            0x0102_9000,
            0x0107_c000,
            0x0109_9000,
            0x011c_4000,
            0x011f_4000,
            0x0120_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(FLOAT_MAX_01016970, f32::MAX);
        e.mem.set_f32(UNUSED_BIRTH_0107CC20, -10000.0);
        e.mem.set_f32(TWO_TO_32_01099378, 4_294_967_296.0);
        e.mem.set_f64(TWO_TO_MINUS_32_010229C0, 2f64.powi(-32));
        e.mem.set_f64(DEGREES_TO_RADIANS_01023128, 0.5);
        e.mem.set_f64(HUNDRED_01017A40, 100.0);
        e.mem.set_f64(RAND_MAX_01029790, 32767.0);
        e.mem.set_f64(ONE_01012070, 1.0);
        e.mem.set_f64(DOUBLE_MAX_010231B0, f64::from(f32::MAX));
        e.register_double(PARTICLE_COUNT_00BA52D0, move |_, _| count.into_ret());
        let mut queue = rands.into_iter();
        e.register_double(RAND, move |_, _| queue.next().unwrap_or(0).into_ret());
        e.register(FTOL, |_, a| {
            let value = f64::take(a, &mut 0);
            (value as i32).into_ret()
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        for quiet in [
            MEMORY_DEALLOCATE,
            FREE_SIZED_00AA1460,
            MEMSET,
            SHADER_PROPERTY_CONSTRUCT,
            SHADER_PROPERTY_DESTROY,
            SHADER_PROPERTY_COPY_MEMBERS,
            CHECK_CREATE_RENDER_PASS_ARRAY,
            TARGET_ARRAY_REMOVE_ALL,
            TARGET_ARRAY_COMPACT,
            VECTOR_DESTRUCT,
            POINT_UNITIZE_004A0C10,
            DELETE,
        ] {
            e.register(quiet, STUB);
        }
        e.register(TARGET_ARRAY_ADD, |e, a| {
            // Record the added object (the NiPointer's word) at the array's next slot.
            let object = e.mem.u32(a[1]);
            let size = e.mem.u16(a[0] + 0xa);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * u32::from(size), object);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        // The point helpers answer their input point plus (1, 2, 3) in their result block.
        for helper in [POINT_ROTATE_004B4500, POINT_TRANSFORM_00524C40] {
            e.register(helper, |e, a| {
                for i in 0..3 {
                    let v = e.mem.f32(a[2] + 4 * i) + (i + 1) as f32;
                    e.mem.set_f32(a[1] + 4 * i, v);
                }
                a[1].into_ret()
            });
        }
        e.put_vtable(VTABLE, &[0, DELETE, GET_RTTI, IS_NODE]);
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(IS_NODE, |e, a| e.mem.u32(a[0] + 0x44).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let o = e.mem.alloc(0xc0);
        e.mem.set_u32(o, VTABLE);
        e.mem.set_u32(o + 4, 1);
        o
    }

    /// A particle property with `count` particles of instance data and the given targets.
    fn property(e: &mut Engine, count: u32, targets: &[u32]) -> Ptr<Particles> {
        let this = e.new_object::<Particles>();
        let data = e.mem.alloc(count.max(1) * 0x30);
        e.set(this, Particles::pInstanceData, data);
        let array = e.mem.alloc(4 * targets.len().max(1) as u32);
        for (i, t) in targets.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *t);
        }
        e.mem.set_u32(this.addr() + 0x120, array);
        e.mem.set_u16(this.addr() + 0x126, targets.len() as u16);
        this
    }

    fn particle(e: &Engine, this: Ptr<Particles>, index: u32) -> Vec<f32> {
        let data = e.get(this, Particles::pInstanceData) + 0x30 * index;
        (0..12).map(|i| e.mem.f32(data + 4 * i)).collect()
    }

    #[test]
    fn init_particle_data_resets_and_draws_each_particle() {
        let mut e = rig(2, vec![]);
        e.register(BS_RANDOM_CONSTRUCT, STUB);
        // UnsignedInt: half the range for the -1 bound, else the bound less one.
        e.register(BS_RANDOM_UNSIGNED_INT, |_, a| {
            if a[1] == u32::MAX {
                0x8000_0000u32.into_ret()
            } else {
                (a[1] - 1).into_ret()
            }
        });
        let this = property(&mut e, 2, &[]);
        for (field, value) in [
            (Particles::fLifetime, 4.0f32),
            (Particles::fLifeVar, 1.0),
            (Particles::fNormalSpeed, 2.0),
            (Particles::fNormalSpeedVar, 1.0),
            (Particles::fRotate, 10.0),
            (Particles::fRotateVar, 2.0),
            (Particles::fRotateSpeed, 4.0),
            (Particles::fRotateSpeedVar, 4.0),
            (Particles::fTextureCountX, 4.0),
            (Particles::fTextureCountY, 2.0),
        ] {
            e.set(this, field, value.to_bits());
        }
        e.set(this, Particles::iParticleCount, 9);
        e.call(0x00ba_6e00, &args![this, 7.5f32]);
        assert_eq!(f(e.get(this, Particles::fTimer)), 7.5);
        assert_eq!(e.get(this, Particles::iParticleCount), 0);
        for index in 0..2 {
            let p = particle(&e, this, index);
            assert_eq!(&p[..3], &[f32::MAX; 3]);
            assert_eq!(p[3], -10000.0);
            assert_eq!(&p[4..7], &[0.0; 3]);
            // Halfway: life 1 +- 0.25 gives 1, rotation (8..12) * 0.5 = 5, speed (0..8) * 0.5
            // = 2, speed 1 +- 0.5 gives 1, texture (1 / 2) * 100 + 3 / 4.
            assert_eq!(p[7], 1.0);
            assert_eq!(p[8], 5.0);
            assert_eq!(p[9], 2.0);
            assert_eq!(p[10], 1.0);
            assert_eq!(p[11], 50.75);
        }
        // The static BSRandom is made once.
        assert_eq!(calls(&e, BS_RANDOM_CONSTRUCT).len(), 1);
        assert_eq!(e.mem.u32(BS_RANDOM_GUARD_011C4B4C) & 1, 1);
        // No particles: only the timer and the count.
        let mut e = rig(0, vec![]);
        let this = property(&mut e, 1, &[]);
        e.set(this, Particles::iParticleCount, 3);
        e.call(0x00ba_6e00, &args![this, 1.0f32]);
        assert_eq!(e.get(this, Particles::iParticleCount), 0);
        assert_eq!(f(e.get(this, Particles::fTimer)), 1.0);
    }

    #[test]
    fn ray_emitter_places_along_the_ray() {
        // Draws: the ray fraction 1, then z 1 and x -1 for the direction.
        let mut e = rig(1, vec![32767, 32767, 0]);
        let target = object(&mut e);
        let this = property(&mut e, 1, &[target]);
        e.set(this, Particles::fRayLength, 8.0f32.to_bits());
        e.set(this, Particles::fTimer, 3.0f32.to_bits());
        e.call(0x00ba_71d0, &args![this, 0u32]);
        let p = particle(&e, this, 0);
        assert_eq!(&p[..4], &[0.0, 8.0, 0.0, 3.0]);
        // The direction (x, 0, z) = (-1, 0, 1) (the normaliser is a double here).
        assert_eq!(&p[4..7], &[-1.0, 0.0, 1.0]);
        // In world space both go through the target's +0x68.
        let mut e = rig(1, vec![0, 32767, 0]);
        let target = object(&mut e);
        let this = property(&mut e, 1, &[target]);
        e.set(this, Particles::bWorldspace, 1);
        e.call(0x00ba_71d0, &args![this, 0u32]);
        let p = particle(&e, this, 0);
        assert_eq!(&p[..3], &[1.0, 2.0, 3.0]);
        assert_eq!(&p[4..7], &[0.0, 2.0, 4.0]);
        assert_eq!(calls(&e, POINT_TRANSFORM_00524C40)[0][0], target + 0x68);
        // Without targets: the zero point.
        let mut e = rig(1, vec![]);
        let this = property(&mut e, 1, &[]);
        e.mem.set_f32(ZERO_POINT_011F426C, 5.0);
        e.call(0x00ba_71d0, &args![this, 0u32]);
        let p = particle(&e, this, 0);
        assert_eq!((p[0], p[4]), (5.0, 5.0));
    }

    #[test]
    fn instance_index_is_made_once() {
        let mut e = rig(3, vec![]);
        let block = e.call(0x00ba_73a0, &args![]).u32();
        assert_eq!(e.mem.u32(INSTANCE_INDEX_01200420), block);
        for i in 0..3 {
            for j in 0..4 {
                assert_eq!(e.mem.f32(block + 0x10 * i + 4 * j), i as f32);
            }
        }
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x30]]);
        assert_eq!(e.call(0x00ba_73a0, &args![]).u32(), block);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
    }

    #[test]
    fn render_passes_are_two_marked_passes() {
        let mut e = rig(1, vec![]);
        let this = e.new_object::<BSShaderProperty>();
        let list = e.mem.alloc(0x14);
        let passes = e.mem.alloc(8);
        e.mem.set_u32(list + 4, passes);
        e.mem.set_u32(this.addr() + 0x3c, list);
        let pass_objects = [e.mem.alloc(8), e.mem.alloc(8)];
        e.register_double(RENDER_PASS_ARRAY_ADD, move |e, a| {
            let size = e.mem.u32(a[0] + 0x10);
            let array = e.mem.u32(a[0] + 4);
            e.mem.set_u32(array + 4 * size, pass_objects[size as usize]);
            e.mem.set_u32(a[0] + 0x10, size + 1);
            Ret::default()
        });
        let r = e.call(
            0x00ba_7420,
            &args![this, 0x900u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), list);
        let added = calls(&e, RENDER_PASS_ARRAY_ADD);
        assert_eq!(added[0], vec![list, 0x900, 0x41, 1, 0, 0, 0, 0, 0]);
        assert_eq!(added[1][2], 0x241);
        assert_eq!(e.mem.u8(pass_objects[0] + 6), 0xe);
        assert_eq!(e.mem.u8(pass_objects[1] + 6), 1);
        assert_eq!(
            calls(&e, CHECK_CREATE_RENDER_PASS_ARRAY),
            vec![vec![this.addr(), u32::MAX]]
        );
        // A list with passes stays as it is.
        e.call(
            0x00ba_7420,
            &args![this, 0x900u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(calls(&e, RENDER_PASS_ARRAY_ADD).len(), 2);
    }

    #[test]
    fn geometry_emitter_takes_a_vertex() {
        // The first target has no geometry data and is skipped; vertex 1 of the second.
        let mut e = rig(1, vec![0, 1, 3]);
        let skipped = object(&mut e);
        let target = object(&mut e);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0xb8, data);
        e.mem.set_u16(data + 8, 2);
        let positions = e.mem.alloc(0x18);
        let normals = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_f32(positions + 4 * i, i as f32);
            e.mem.set_f32(normals + 4 * i, 10.0 + i as f32);
        }
        e.mem.set_u32(data + 0x20, positions);
        e.mem.set_u32(data + 0x24, normals);
        let this = property(&mut e, 1, &[skipped, target]);
        e.call(0x00ba_74a0, &args![this, 0u32]);
        let p = particle(&e, this, 0);
        assert_eq!(&p[..3], &[3.0, 4.0, 5.0]);
        assert_eq!(&p[4..7], &[13.0, 14.0, 15.0]);
        // Null arrays give the zero point, put through the helpers in world space.
        let mut e = rig(1, vec![1, 0]);
        let target = object(&mut e);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0xb8, data);
        e.mem.set_u16(data + 8, 1);
        let this = property(&mut e, 1, &[target]);
        e.set(this, Particles::bWorldspace, 1);
        e.call(0x00ba_74a0, &args![this, 0u32]);
        let p = particle(&e, this, 0);
        assert_eq!(&p[..3], &[1.0, 2.0, 3.0]);
        assert_eq!(&p[4..7], &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn geometry_emitter_reads_locked_streams_and_bones() {
        const STREAMS_READY: u32 = 0x0f00_008c;
        let mut e = rig(1, vec![0, 0]);
        let target = object(&mut e);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0xb8, data);
        e.mem.set_u16(data + 8, 1);
        let streams = e.mem.alloc(8);
        let mut slots = vec![0u32; 0x24];
        slots[0x23] = STREAMS_READY;
        e.put_vtable(0x0f00_2000, &slots);
        e.register(STREAMS_READY, |_, _| true.into_ret());
        e.mem.set_u32(streams, 0x0f00_2000);
        e.mem.set_u32(data + 0x30, streams);
        let block = e.mem.alloc(0x20);
        for i in 0..6 {
            e.mem.set_f32(block + 4 * i, 20.0 + i as f32);
        }
        e.register(GEOMETRY_DATA_LOCK, |_, _| true.into_ret());
        e.register_double(GEOMETRY_DATA_POSITIONS, move |e, a| {
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 4, 0xc);
            Ret::default()
        });
        e.register_double(GEOMETRY_DATA_NORMALS, move |e, a| {
            e.mem.set_u32(a[1], block + 0xc);
            e.mem.set_u32(a[1] + 4, 0xc);
            Ret::default()
        });
        e.register(GEOMETRY_DATA_UNLOCK, STUB);
        let this = property(&mut e, 1, &[target]);
        e.call(0x00ba_74a0, &args![this, 0u32]);
        let p = particle(&e, this, 0);
        assert_eq!(&p[..3], &[20.0, 21.0, 22.0]);
        assert_eq!(&p[4..7], &[23.0, 24.0, 25.0]);
        assert_eq!(calls(&e, GEOMETRY_DATA_LOCK), vec![vec![data, 1]]);
        assert_eq!(calls(&e, GEOMETRY_DATA_UNLOCK), vec![vec![data]]);

        // A skinned target: between a bone and its parent.
        let mut e = rig(1, vec![0, 0, 0, 32767, 0]);
        let target = object(&mut e);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0xb8, data);
        let skin = e.mem.alloc(0x20);
        let skin_data = e.mem.alloc(0x50);
        e.mem.set_u32(skin_data + 0x44, 1);
        e.mem.set_u32(skin + 8, skin_data);
        let bone = e.mem.alloc(0xa0);
        let parent = e.mem.alloc(0xa0);
        e.mem.set_u32(bone + 0x18, parent);
        e.mem.set_f32(bone + 0x8c, 2.0);
        e.mem.set_f32(parent + 0x8c, 4.0);
        let bones = e.mem.alloc(4);
        e.mem.set_u32(bones, bone);
        e.mem.set_u32(skin + 0x14, bones);
        e.mem.set_u32(target + 0xbc, skin);
        let this = property(&mut e, 1, &[target]);
        e.call(0x00ba_74a0, &args![this, 0u32]);
        let p = particle(&e, this, 0);
        // t = 0: all from the parent.
        assert_eq!(&p[..3], &[4.0, 0.0, 0.0]);
        // Direction (-1, 0, 1) through the helper: plus (1, 2, 3).
        assert_eq!(&p[4..7], &[0.0, 2.0, 4.0]);
    }

    #[test]
    fn node_emitter_blends_or_takes_the_translation() {
        let mut e = rig(1, vec![0, 32767, 32767, 0]);
        let target = object(&mut e);
        let parent = object(&mut e);
        e.mem.set_u16(target + 0xa8, 1);
        e.mem.set_u32(target + 0x18, parent);
        e.mem.set_f32(target + 0x8c, 6.0);
        e.mem.set_f32(parent + 0x90, 2.0);
        let this = property(&mut e, 1, &[target]);
        e.call(0x00ba_7860, &args![this, 0u32]);
        let p = particle(&e, this, 0);
        // t = 1: all from the target.
        assert_eq!(&p[..3], &[6.0, 0.0, 0.0]);
        assert_eq!(&p[4..7], &[0.0, 2.0, 4.0]);
        // Without the word at +0xa8: the target's translation and three draws.
        let mut e = rig(1, vec![0, 0, 32767, 0]);
        let target = object(&mut e);
        e.mem.set_f32(target + 0x94, 9.0);
        let this = property(&mut e, 1, &[target]);
        e.call(0x00ba_7860, &args![this, 0u32]);
        let p = particle(&e, this, 0);
        assert_eq!(&p[..3], &[0.0, 0.0, 9.0]);
        assert_eq!(&p[4..7], &[-1.0, 1.0, -1.0]);
    }

    #[test]
    fn update_particles_kills_old_ones_and_emits() {
        let mut e = rig(3, vec![]);
        let this = property(&mut e, 3, &[]);
        e.set(this, Particles::fLifetime, 2.0f32.to_bits());
        e.set(this, Particles::fMaxParticleRatio, 1.0f32.to_bits());
        e.set(this, Particles::fTimer, 9.0f32.to_bits());
        e.set(this, Particles::eEmitterType, 2);
        e.set(this, Particles::iParticleCount, 2);
        let data = e.get(this, Particles::pInstanceData);
        // Particle 0: alive (age 1); 1: too old; 2: born in the future.
        for (i, birth) in [9.0f32, 5.0, 11.0].iter().enumerate() {
            let at = data + 0x30 * i as u32;
            e.mem.set_f32(at + 0xc, *birth);
            e.mem.set_f32(at + 0x1c, 1.0);
        }
        e.call(0x00ba_7b60, &args![this, 10.0f32, 0u32, 1u32]);
        // dt 1 / lifetime 2 * 3 = 1.5: one made, 0.5 kept.
        assert_eq!(f(e.get(this, Particles::fGenerateRemainder)), 0.5);
        assert_eq!(f(e.get(this, Particles::fTimer)), 10.0);
        assert_eq!(particle(&e, this, 0)[3], 9.0);
        // Particle 1 is remade by the ray emitter (no targets: the zero point, born at the
        // old timer).
        let p1 = particle(&e, this, 1);
        assert_eq!(&p1[..4], &[0.0, 0.0, 0.0, 9.0]);
        // Particle 2 is made unused; no more to make.
        let p2 = particle(&e, this, 2);
        assert_eq!(&p2[..4], &[f32::MAX, f32::MAX, f32::MAX, -10000.0]);
        // Two died, one was made.
        assert_eq!(e.get(this, Particles::iParticleCount), 1);
    }

    #[test]
    fn update_particles_follows_bip01() {
        let mut e = rig(1, vec![]);
        let this = property(&mut e, 1, &[]);
        e.set(this, Particles::fLifetime, 1.0f32.to_bits());
        e.set(this, Particles::fMaxParticleRatio, 1.0f32.to_bits());
        e.set(this, Particles::eEmitterType, 2);
        e.map(0x0101_e000, 0x1000);
        set_text(&mut e, BIP01_NAME_0101E460, b"Bip01\0");
        let name = e.mem.alloc(8);
        set_text(&mut e, name, b"Bip01\0");
        let other = e.mem.alloc(8);
        set_text(&mut e, other, b"Bip02\0");
        let geometry = e.mem.alloc(0xa0);
        let first = e.mem.alloc(0xa0);
        let bip = e.mem.alloc(0xa0);
        let root = e.mem.alloc(0xa0);
        e.mem.set_u32(geometry + 0x18, first);
        e.mem.set_u32(first + 8, other);
        e.mem.set_u32(first + 0x18, bip);
        e.mem.set_u32(bip + 8, name);
        e.mem.set_u32(bip + 0x18, root);
        e.mem.set_f32(bip + 0x8c, 5.0);
        e.mem.set_f32(root + 0x8c, 1.0);
        e.set(this, Particles::pMyGeometry, geometry);
        // The particle is in the future: it dies and is remade (dt 2 / 1 * 1).
        let data = e.get(this, Particles::pInstanceData);
        e.mem.set_f32(data + 0xc, 5.0);
        e.call(0x00ba_7b60, &args![this, 2.0f32, 1u32, 1u32]);
        let p = particle(&e, this, 0);
        assert_eq!(&p[..3], &[-4.0, 0.0, 0.0]);
    }

    #[test]
    fn destructor_and_constructor() {
        let mut e = rig(2, vec![]);
        e.register(POINTER_ARRAY_ALLOCATE, |e, a| {
            let block = e.mem.alloc(4 * a[0] + 4);
            (block + 4).into_ret()
        });
        let this = e.new_object::<Particles>();
        assert_eq!(e.call(0x00ba_7f70, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), PARTICLE_PROPERTY_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x11c), TARGET_ARRAY_VTABLE_0101FBD4);
        assert_eq!(e.mem.u16(this.addr() + 0x124), 10);
        assert_eq!(e.mem.u16(this.addr() + 0x12a), 10);
        assert_eq!(e.get(this, Particles::bWorldspace), 1);
        assert_eq!(f(e.get(this, Particles::fMaxParticleRatio)), 1.0);
        assert_eq!(e.mem.f32(this.addr() + 0x134), 1.0);
        assert_eq!(e.mem.u32(this.addr() + 0x58), 0x18);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 0x11);
        let data = e.get(this, Particles::pInstanceData);
        assert_ne!(data, 0);
        assert_eq!(e.mem.u32(INSTANCE_USERS_01200424), 1);
        let block = e.mem.u32(INSTANCE_INDEX_01200420);
        assert_ne!(block, 0);
        assert_eq!(calls(&e, MEMSET), vec![vec![data, 0, 0x60]]);
        // The destructor gives everything back; the last user frees the shared block.
        let texture = object(&mut e);
        e.set(this, Particles::spBaseTexture, texture);
        let elements = e.mem.u32(this.addr() + 0x120);
        e.mem.set_u32(elements - 4, 10);
        e.call(0x00ba_7df0, &args![this]);
        assert_eq!(e.get(this, Particles::spBaseTexture), 0);
        assert_eq!(calls(&e, DELETE), vec![vec![texture]]);
        assert_eq!(e.mem.u32(INSTANCE_USERS_01200424), 0);
        assert_eq!(e.mem.u32(INSTANCE_INDEX_01200420), 0);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT),
            vec![vec![elements, 4, 10, POINTER_DESTRUCT_0045CEC0]]
        );
        let freed = calls(&e, MEMORY_DEALLOCATE);
        assert!(freed.contains(&vec![MEMORY_MANAGER, data]));
        assert!(freed.contains(&vec![MEMORY_MANAGER, block]));
        assert!(freed.contains(&vec![MEMORY_MANAGER, elements - 4]));
        assert_eq!(calls(&e, SHADER_PROPERTY_DESTROY), vec![vec![this.addr()]]);
    }

    #[test]
    fn small_slots() {
        let mut e = rig(1, vec![]);
        e.register(POINTER_ARRAY_ALLOCATE, |e, a| {
            e.mem.alloc(4 * a[0]).into_ret()
        });
        let this = e.new_object::<Particles>();
        assert_eq!(e.call(0x00ba_7f10, &args![this]).u32(), 0x0120_0434);
        // The scalar deleting destructor frees 0x14c bytes on bit 0.
        e.call(0x00ba_7f70, &args![this]);
        e.call(0x00ba_8270, &args![this, 1u32]);
        assert_eq!(
            calls(&e, FREE_SIZED_00AA1460),
            vec![vec![this.addr(), 0x14c]]
        );
        // CreateClone constructs and copies the base members only.
        let clone = e.call(0x00ba_82a0, &args![this, 0x55u32]).u32();
        assert_eq!(e.mem.u32(clone), PARTICLE_PROPERTY_VTABLE);
        assert_eq!(
            calls(&e, SHADER_PROPERTY_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x55]]
        );
        // The viewer strings get the class name.
        e.register(SHADER_PROPERTY_GET_VIEWER_STRINGS, STUB);
        e.register(NI_GET_VIEWER_STRING, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_0096AD30, STUB);
        e.register(ARRAY_SET_AT_0096AE90, STUB);
        e.mem.set_u32(PARTICLE_PROPERTY_RTTI, 0x0102_0000);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00ba_7f20, &args![this, strings]);
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0102_0000]]);
        assert_eq!(calls(&e, ARRAY_SET_SIZE_0096AD30), vec![vec![strings, 3]]);
        assert_eq!(calls(&e, ARRAY_SET_AT_0096AE90)[0][..2], [strings, 1]);
    }

    #[test]
    fn collect_targets_by_emitter_type() {
        let mut e = rig(1, vec![]);
        e.register(GET_PROPERTY, |e, a| e.mem.u32(a[0] + 0x48).into_ret());
        // strncmp: 0 when the name starts with "Bip".
        e.register(STRNCMP, |e, a| {
            let bip =
                e.mem.u8(a[0]) == b'B' && e.mem.u8(a[0] + 1) == b'i' && e.mem.u8(a[0] + 2) == b'p';
            u32::from(!bip).into_ret()
        });
        e.map(0x0109_b000, 0x1000);
        let this = e.new_object::<Particles>();
        let elements = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 0x120, elements);
        // A node with a lit geometry child, a "Bip" child node holding a lit geometry, and a
        // geometry without a lighting property.
        let node = object(&mut e);
        e.mem.set_u32(node + 0x44, node);
        let geometry = object(&mut e);
        let tri_rtti = e.mem.alloc(8);
        e.mem.set_u32(tri_rtti + 4, TRI_BASED_GEOM_RTTI);
        e.mem.set_u32(geometry + 0x40, tri_rtti);
        let lit = object(&mut e);
        e.mem.set_u32(lit + 0x40, LIGHTING_PROPERTY_RTTI);
        e.mem.set_u32(geometry + 0x48, lit);
        let unlit = object(&mut e);
        e.mem.set_u32(unlit + 0x40, TRI_BASED_GEOM_RTTI);
        let bip = object(&mut e);
        e.mem.set_u32(bip + 0x44, bip);
        let bip_name = e.mem.alloc(8);
        set_text(&mut e, bip_name, b"Bip01\0");
        e.mem.set_u32(bip + 8, bip_name);
        let hidden = object(&mut e);
        e.mem.set_u32(hidden + 0x40, TRI_BASED_GEOM_RTTI);
        e.mem.set_u32(hidden + 0x48, lit);
        let bip_children = e.mem.alloc(4);
        e.mem.set_u32(bip_children, hidden);
        e.mem.set_u32(bip + 0xa0, bip_children);
        e.mem.set_u16(bip + 0xa6, 1);
        let name = e.mem.alloc(8);
        set_text(&mut e, name, b"Scene\0");
        e.mem.set_u32(node + 8, name);
        let children = e.mem.alloc(12);
        for (i, c) in [geometry, bip, unlit].iter().enumerate() {
            e.mem.set_u32(children + 4 * i as u32, *c);
        }
        e.mem.set_u32(node + 0xa0, children);
        e.mem.set_u16(node + 0xa6, 3);
        e.call(0x00ba_8560, &args![this, node]);
        assert_eq!(e.mem.u16(this.addr() + 0x126), 1);
        assert_eq!(e.mem.u32(elements), geometry);
        // The temporary's count went up and down again.
        assert_eq!(e.mem.u32(geometry + 4), 1);
        assert_eq!(calls(&e, TARGET_ARRAY_COMPACT).len(), 1);
        // Type 1: flagged nodes (+0x1c) are added, then their children walked.
        e.set(this, Particles::eEmitterType, 1);
        e.mem.set_u16(this.addr() + 0x126, 0);
        e.mem.set_u32(node + 0x1c, 1);
        e.mem.set_u32(bip + 0x1c, 1);
        e.call(0x00ba_8560, &args![this, node]);
        assert_eq!(e.mem.u16(this.addr() + 0x126), 2);
        assert_eq!((e.mem.u32(elements), e.mem.u32(elements + 4)), (node, bip));
        // Type 2: the object itself.
        e.set(this, Particles::eEmitterType, 2);
        e.mem.set_u16(this.addr() + 0x126, 0);
        e.call(0x00ba_8560, &args![this, unlit]);
        assert_eq!(e.mem.u32(elements), unlit);
        // Type 3: nothing, still compacted.
        e.set(this, Particles::eEmitterType, 3);
        e.mem.set_u16(this.addr() + 0x126, 0);
        e.call(0x00ba_8560, &args![this, unlit]);
        assert_eq!(e.mem.u16(this.addr() + 0x126), 0);
        assert_eq!(calls(&e, TARGET_ARRAY_COMPACT).len(), 4);
    }
}
