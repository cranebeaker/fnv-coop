//! `NiParticle/nipsysrotationmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c206a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRotationModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2528.
pub fn ni_p_sys_rotation_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2528
}

use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysmodifier::{stream_read, NiPSysModifier, STREAM_VERSION};

layout! {
    /// `NiPSysRotationModifier` (Xbox PDB), 0x38 bytes; the PC code uses the
    /// same offsets. Floats are declared as their bit patterns.
    pub struct NiPSysRotationModifier: 0x38 {
        /// `m_fInitialRotSpeed` (Xbox PDB).
        0x18 m_fInitialRotSpeed: u32,
        /// `m_fInitialRotSpeedVar` (Xbox PDB).
        0x1C m_fInitialRotSpeedVar: u32,
        /// `m_fInitialRotAngle` (Xbox PDB).
        0x20 m_fInitialRotAngle: u32,
        /// `m_fInitialRotAngleVar` (Xbox PDB).
        0x24 m_fInitialRotAngleVar: u32,
        /// `m_kInitialAxis` (Xbox PDB): an `NiPoint3`.
        0x28 m_kInitialAxis: u32,
        /// `m_bRandomInitialAxis` (Xbox PDB).
        0x34 m_bRandomInitialAxis: u8,
        /// `m_bRandomRotSpeedSign` (Xbox PDB).
        0x35 m_bRandomRotSpeedSign: u8,
    }
}

/// The vtable of `NiPSysRotationModifier` (RTTI
/// `.?AVNiPSysRotationModifier@@`, `010bdaec`).
const VTABLE: u32 = 0x010b_daec;
/// The axis the constructor gives `m_kInitialAxis` (three `float`s at
/// `011a946c`, copied by `MOV`).
const DEFAULT_AXIS: u32 = 0x011a_946c;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// The `NiPSysModifier` constructor (`00c31720`), member copy (`00c31780`,
/// `(dest, process)`) and `LoadBinary` (`00c318b0`).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
/// `00a89af0` (unnamed): the base post-link step, `thiscall`, one word.
const BASE_POST_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiColor::LoadBinary` (`00a59be0`, `thiscall` on the colour, the
/// stream): reads the initial axis (three floats).
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `_rand` (`00ecadb8`), `_ftol2` (`00ec62c0`) and the square root helper
/// `00ec6040` (both take the `ST0` value as a leading `f64`).
const RAND: u32 = 0x00ec_adb8;
const FTOL: u32 = 0x00ec_62c0;
const SQRT: u32 = 0x00ec_6040;
/// `double` constants: 32767.0 (`RAND_MAX`), 1.0, 512.0 and π (the double
/// `400921fb60000000`, the float π widened).
const RAND_MAX_DOUBLE: u32 = 0x0102_9790;
const ONE: u32 = 0x0101_2070;
const TABLE_SIZE: u32 = 0x0102_8338;
const PI_DOUBLE: u32 = 0x0101_ff40;
/// The `float` 0.5 (`01016248`) and 2π (`011aea98`).
const HALF_FLOAT: u32 = 0x0101_6248;
const TWO_PI: u32 = 0x011a_ea98;
/// The two 512-entry `float` tables read by angle index.
const TABLE_A: u32 = 0x011f_52e0;
const TABLE_B: u32 = 0x011f_4ae0;
/// `Update`'s function-local `static float` limit (`01202520`), its guard
/// word (bit 0, `01202524`) and the `float` it is initialised from
/// (`010bdb90`, 31.415928).
const STATIC_LIMIT: u32 = 0x0120_2520;
const STATIC_GUARD: u32 = 0x0120_2524;
const STATIC_LIMIT_INIT: u32 = 0x010b_db90;
/// From file version 20.0.0.2 the speed variation, angle, angle variation
/// and random-sign flag are stored, and `PostLinkObject` no longer sets the
/// speeds.
const VERSION_ROTATION_FIELDS: u32 = 0x1400_0002;
/// `NiParticleSystem`'s model data (+0xb8, the `NiPSysData`), as the PC
/// code reads it.
const SYSTEM_DATA: u32 = 0xb8;

/// `rand() / 32767.0` rounded to a `float`.
fn random_unit(e: &mut Engine) -> u32 {
    let r = e.call(RAND, &args![]).i32();
    let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
    x87_store32(x87_div(f64::from(r), rand_max))
}

/// `(2 * rand()) / 32767.0 - 1.0` rounded to a `float`.
fn random_signed(e: &mut Engine) -> u32 {
    let r = f64::from(e.call(RAND, &args![]).i32());
    let rand_max: f64 = e.global(RAND_MAX_DOUBLE);
    let one: f64 = e.global(ONE);
    x87_store32(x87_sub(x87_div(x87_add(r, r), rand_max), one))
}

/// The table index of an angle: `ftol(scale * angle) & 0x1ff`.
fn table_index(e: &mut Engine, scale: f64, angle: u32) -> u32 {
    e.call(FTOL, &args![x87_mul(scale, x87_m32(angle))]).u32() & 0x1ff
}

/// The constructor the stream factory and `CreateClone` inline after the
/// `NiPSysModifier` constructor: the vtable, speeds and angles 0.0, the
/// default axis, a random initial axis and no random speed sign.
fn construct_fields(e: &mut Engine, object: Ptr<NiPSysRotationModifier>) {
    let zero = 0.0f32.to_bits();
    e.set(object, NiPSysRotationModifier::m_fInitialRotSpeed, zero);
    e.mem.set_u32(object.addr(), VTABLE);
    e.set(object, NiPSysRotationModifier::m_fInitialRotSpeedVar, zero);
    e.set(object, NiPSysRotationModifier::m_fInitialRotAngle, zero);
    e.set(object, NiPSysRotationModifier::m_fInitialRotAngleVar, zero);
    for word in 0..3 {
        let value = e.global::<u32>(DEFAULT_AXIS + 4 * word);
        e.mem.set_u32(
            object.addr() + NiPSysRotationModifier::m_kInitialAxis.off + 4 * word,
            value,
        );
    }
    e.set(object, NiPSysRotationModifier::m_bRandomInitialAxis, 1);
    e.set(object, NiPSysRotationModifier::m_bRandomRotSpeedSign, 0);
}

// Translated from 00c205e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysRotationModifier` member copy into
/// `dest` (`thiscall`, `RET 8`: `dest`, cloning process). The
/// `NiPSysModifier` member copy, then the speed and its variation, the
/// random-sign flag, the angle and its variation, the random-axis flag and
/// the axis (by `MOV`); the floats go through the x87 stack.
pub fn fn_00c205e0(
    e: &mut Engine,
    this: Ptr<NiPSysRotationModifier>,
    dest: Ptr<NiPSysRotationModifier>,
    process: Ptr,
) {
    e.call(MODIFIER_COPY_MEMBERS, &args![this, dest, process]);
    let copy_float = |e: &mut Engine, field| {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(dest, field, value);
    };
    copy_float(e, NiPSysRotationModifier::m_fInitialRotSpeed);
    copy_float(e, NiPSysRotationModifier::m_fInitialRotSpeedVar);
    let sign = e.get(this, NiPSysRotationModifier::m_bRandomRotSpeedSign);
    e.set(dest, NiPSysRotationModifier::m_bRandomRotSpeedSign, sign);
    copy_float(e, NiPSysRotationModifier::m_fInitialRotAngle);
    copy_float(e, NiPSysRotationModifier::m_fInitialRotAngleVar);
    let axis = e.get(this, NiPSysRotationModifier::m_bRandomInitialAxis);
    e.set(dest, NiPSysRotationModifier::m_bRandomInitialAxis, axis);
    for word in 0..3 {
        let off = NiPSysRotationModifier::m_kInitialAxis.off + 4 * word;
        let value = e.mem.u32(this.addr() + off);
        e.mem.set_u32(dest.addr() + off, value);
    }
}

// Translated from 00c20640 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project (registered through `push
/// 00c1acb7`): the `NiPSysRotationModifier` stream factory (`cdecl`, no
/// arguments): a 0x38-byte object from `00aa13e0`, constructed (the
/// `NiPSysModifier` constructor, the vtable `010bdaec`, speeds and angles
/// 0.0, the axis from `011a946c`, a random initial axis, no random speed
/// sign); 0 when the allocation fails.
pub fn fn_00c20640(e: &mut Engine) -> Ptr<NiPSysRotationModifier> {
    let object = Ptr::<NiPSysRotationModifier>::new(e.call(NI_NEW, &args![0x38u32]).u32());
    if object.is_null() {
        return Ptr::NULL;
    }
    e.call(MODIFIER_CONSTRUCTOR, &args![object]);
    construct_fields(e, object);
    object
}

// Translated from 00c206b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRotationModifier::Update` (Xbox PDB): `thiscall` (`this` is not
/// read), `RET 8` (time, particle data). With rotation angles, each active
/// particle's angle advances by `(time - last update) * rotation speed`;
/// when the angle passes the function-local `static float` limit (set to
/// the `float` at `010bdb90`, 31.415928, the first time, its guard word
/// written back after the loop) it becomes 0, else while it is above 2π it
/// loses 2π (the first test before any subtraction; a NaN angle stays). The
/// x87 operations are done instruction by instruction.
pub fn ni_p_sys_rotation_modifier_update(
    e: &mut Engine,
    _this: Ptr<NiPSysRotationModifier>,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    let angles = e.get(data, NiPSysData::m_pfRotationAngles);
    if angles == 0 {
        return;
    }
    let speeds = e.get(data, NiPSysData::m_pfRotationSpeeds);
    if e.get(data, NiPSysData::m_usActive) == 0 {
        return;
    }
    let now = x87_load32(time.to_bits());
    let two_pi = x87_load32(e.global::<u32>(TWO_PI));
    let mut guard = e.global::<u32>(STATIC_GUARD);
    let limit_init = x87_load32(e.global::<u32>(STATIC_LIMIT_INIT));
    let mut i: u16 = 0;
    loop {
        let k = u32::from(i);
        let info = e.get(data, NiPSysData::m_pkParticleInfo);
        let last_update = e.mem.u32(info + k * 0x1c + 0x14);
        let elapsed = x87_store32(x87_sub(now, x87_m32(last_update)));
        let slot = angles + 4 * k;
        let advanced = x87_store32(x87_add(
            x87_mul(x87_load32(elapsed), x87_m32(e.mem.u32(speeds + 4 * k))),
            x87_m32(e.mem.u32(slot)),
        ));
        let angle = x87_load32(advanced);
        e.mem.set_u32(slot, x87_store32(angle));
        if guard & 1 == 0 {
            guard |= 1;
            e.set_global(STATIC_LIMIT, x87_store32(limit_init));
        }
        let limit = x87_load32(e.global::<u32>(STATIC_LIMIT));
        if limit < angle {
            e.mem.set_u32(slot, 0.0f32.to_bits());
        } else if two_pi < angle {
            loop {
                let value = x87_load32(e.mem.u32(slot));
                e.mem.set_u32(slot, x87_store32(x87_sub(value, two_pi)));
                let value = x87_load32(e.mem.u32(slot));
                if !(value > two_pi) {
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
        if i >= e.get(data, NiPSysData::m_usActive) {
            break;
        }
    }
    e.set_global(STATIC_GUARD, guard);
}

// Translated from 00c207a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRotationModifier::Initialize` (Xbox PDB): `thiscall`, `RET 8`
/// (particle data, particle index in the low word). With rotation axes, the
/// particle's axis is the initial axis, or with a random initial axis a
/// random unit vector: `z` the table-`011f4ae0` entry at a random angle in
/// `[0, π]`, the radius `sqrt(1 - z²)` (`00ec6040`), and `x`/`y` the radius
/// times the two tables' entries at a random angle in `[0, 2π]`. With
/// rotation angles, the particle's angle is the initial angle plus its
/// variation times a random number in `[-1, 1]`, and its rotation speed
/// (stored without checking the speed array) likewise, negated with a
/// random speed sign when another `rand()/32767` is not above 0.5. The x87
/// operations are done instruction by instruction.
pub fn ni_p_sys_rotation_modifier_initialize(
    e: &mut Engine,
    this: Ptr<NiPSysRotationModifier>,
    data: Ptr<NiPSysData>,
    index: u16,
) {
    let k = u32::from(index);
    let axes = e.get(data, NiPSysData::m_pkRotationAxes);
    if axes != 0 {
        let axis = axes + k * 0xc;
        if e.get(this, NiPSysRotationModifier::m_bRandomInitialAxis) == 0 {
            for word in 0..3 {
                let value = e
                    .mem
                    .u32(this.addr() + NiPSysRotationModifier::m_kInitialAxis.off + 4 * word);
                e.mem.set_u32(axis + 4 * word, value);
            }
        } else {
            let unit = random_unit(e);
            let pi: f64 = e.global(PI_DOUBLE);
            let theta = x87_store32(x87_mul(x87_load32(unit), pi));
            let two_pi = x87_load32(e.global::<u32>(TWO_PI));
            let table_size: f64 = e.global(TABLE_SIZE);
            let index = table_index(e, x87_div(table_size, two_pi), theta);
            let z = x87_store32(x87_load32(e.global::<u32>(TABLE_B + 4 * index)));
            let zz = x87_load32(z);
            let w = x87_store32(x87_sub(1.0, x87_mul(zz, zz)));
            let root = e.call(SQRT, &args![x87_load32(w)]).f64();
            let radius = x87_quiet_bits(x87_store32(root));
            let unit = random_unit(e);
            let phi = x87_store32(x87_mul(two_pi, x87_m32(unit)));
            let table_size = x87_load64(e.global::<u64>(TABLE_SIZE));
            let index = table_index(e, x87_div(table_size, two_pi), phi);
            let b = x87_store32(x87_load32(e.global::<u32>(TABLE_B + 4 * index)));
            let a = x87_load32(e.global::<u32>(TABLE_A + 4 * index));
            let r = x87_load32(radius);
            e.mem.set_u32(axis, x87_store32(x87_mul(a, r)));
            e.mem.set_u32(axis + 4, x87_store32(x87_mul(r, x87_m32(b))));
            e.mem.set_u32(axis + 8, x87_quiet_bits(z));
        }
    }
    let angles = e.get(data, NiPSysData::m_pfRotationAngles);
    if angles == 0 {
        return;
    }
    let field = |e: &Engine, f| x87_m32(e.get(this, f));
    let loaded = |e: &Engine, f| x87_load32(e.get(this, f));
    let r = random_signed(e);
    let angle = x87_store32(x87_add(
        x87_mul(
            loaded(e, NiPSysRotationModifier::m_fInitialRotAngleVar),
            x87_m32(r),
        ),
        field(e, NiPSysRotationModifier::m_fInitialRotAngle),
    ));
    let angles = e.get(data, NiPSysData::m_pfRotationAngles);
    e.mem.set_u32(angles + 4 * k, x87_quiet_bits(angle));
    let r = random_signed(e);
    let mut speed = x87_store32(x87_add(
        x87_mul(
            loaded(e, NiPSysRotationModifier::m_fInitialRotSpeedVar),
            x87_m32(r),
        ),
        field(e, NiPSysRotationModifier::m_fInitialRotSpeed),
    ));
    if e.get(this, NiPSysRotationModifier::m_bRandomRotSpeedSign) != 0 {
        let unit = x87_load32(random_unit(e));
        let half = x87_m32(e.global::<u32>(HALF_FLOAT));
        // `FCOMP` with 0.5, `TEST AH,0x41; JZ`: negated unless above 0.5.
        if !(unit > half) {
            speed = x87_store32(-x87_load32(speed));
        }
    }
    let speeds = e.get(data, NiPSysData::m_pfRotationSpeeds);
    e.mem.set_u32(speeds + 4 * k, x87_quiet_bits(speed));
}

// Translated from 00c20980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRotationModifier::CreateClone` (Xbox PDB): a 0x38-byte object from
/// `00aa13e0`, constructed as the stream factory does, then the member
/// copy `00c205e0` into it; when the allocation fails the member copy still
/// runs, into a null `dest`, and 0 comes back.
pub fn ni_p_sys_rotation_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysRotationModifier>,
    process: Ptr,
) -> Ptr<NiPSysRotationModifier> {
    let clone = Ptr::<NiPSysRotationModifier>::new(e.call(NI_NEW, &args![0x38u32]).u32());
    if clone.is_null() {
        fn_00c205e0(e, this, Ptr::NULL, process);
        return Ptr::NULL;
    }
    e.call(MODIFIER_CONSTRUCTOR, &args![clone]);
    construct_fields(e, clone);
    fn_00c205e0(e, this, clone, process);
    clone
}

// Translated from 00c20a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRotationModifier::PostLinkObject` (Xbox PDB): the base post-link
/// step (`00a89af0`); for files before 20.0.0.2, when the target system's
/// data has rotation speeds, every active particle's speed becomes the
/// initial speed (the data and its count re-read through the target each
/// time round).
pub fn ni_p_sys_rotation_modifier_post_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysRotationModifier>,
    stream: Ptr,
) {
    e.call(BASE_POST_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_ROTATION_FIELDS {
        return;
    }
    let system_data = |e: &Engine| {
        let target = e.mem.u32(this.addr() + NiPSysModifier::m_pkTarget.off);
        Ptr::<NiPSysData>::new(e.mem.u32(target.wrapping_add(SYSTEM_DATA)))
    };
    let data = system_data(e);
    let speeds = e.get(data, NiPSysData::m_pfRotationSpeeds);
    if speeds == 0 {
        return;
    }
    let mut i: u16 = 0;
    if i >= e.get(data, NiPSysData::m_usActive) {
        return;
    }
    loop {
        let speed = x87_quiet_bits(e.get(this, NiPSysRotationModifier::m_fInitialRotSpeed));
        e.mem.set_u32(speeds + 4 * u32::from(i), speed);
        i = i.wrapping_add(1);
        let data = system_data(e);
        if i >= e.get(data, NiPSysData::m_usActive) {
            break;
        }
    }
}

// Translated from 00c20ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRotationModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, the initial speed; from file version 20.0.0.2 the speed
/// variation, the angle, its variation and the random-sign flag (a byte
/// read into the stream argument's stack word); then the random-axis flag
/// (a byte read into byte 3 of a local) and the axis (`NiColor::LoadBinary`
/// on it). The component-size words are the exe's: the argument's stack
/// word for the speed and the two flags, a local for the rest.
pub fn ni_p_sys_rotation_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysRotationModifier>,
    stream: Ptr,
) {
    e.call(MODIFIER_LOAD_BINARY, &args![this, stream]);
    let s = stream.addr();
    e.with_stack(0x10, |e, frame| {
        let low_local = frame.addr();
        let size_local = frame.addr() + 4;
        let argument_slot = frame.addr() + 0xc;
        e.mem.set_u32(argument_slot, s);
        let field = |f: Field<NiPSysRotationModifier, u32>| this.addr() + f.off;
        stream_read(
            e,
            s,
            field(NiPSysRotationModifier::m_fInitialRotSpeed),
            4,
            4,
            argument_slot,
        );
        if e.mem.u32(s + STREAM_VERSION) >= VERSION_ROTATION_FIELDS {
            for f in [
                NiPSysRotationModifier::m_fInitialRotSpeedVar,
                NiPSysRotationModifier::m_fInitialRotAngle,
                NiPSysRotationModifier::m_fInitialRotAngleVar,
            ] {
                stream_read(e, s, field(f), 4, 4, size_local);
            }
            stream_read(e, s, argument_slot, 1, 1, size_local);
            let sign = (e.mem.u8(argument_slot) != 0) as u8;
            e.set(this, NiPSysRotationModifier::m_bRandomRotSpeedSign, sign);
        }
        stream_read(e, s, low_local + 3, 1, 1, argument_slot);
        let random_axis = (e.mem.u8(low_local + 3) != 0) as u8;
        e.set(
            this,
            NiPSysRotationModifier::m_bRandomInitialAxis,
            random_axis,
        );
    });
    e.call(
        COLOR_LOAD_BINARY,
        &args![this.addr() + NiPSysRotationModifier::m_kInitialAxis.off, s],
    );
}

/// `NiPSysModifier::IsEqual` (`00c31830`), `SaveBinary` (`00c31950`) and
/// `GetViewerStrings` (`00c319e0`): `thiscall`, one word each.
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// `00439090` (unnamed, `thiscall` on a point, one word: the other point):
/// answers nonzero when the two `NiPoint3`s differ.
const POINT_DIFFERS: u32 = 0x0043_9090;
/// `NiColor::SaveBinary` (`00a69780`, `thiscall` on the three floats, the
/// stream): writes the initial axis.
const COLOR_SAVE_BINARY: u32 = 0x00a6_9780;
/// `NiPoint3::GetViewerString` (`00a59af0`, `thiscall` on the point, one
/// word: the name).
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the class name) and its `float`
/// (`00a7d550`) and `bool` (`00a7d370`) overloads (`cdecl`, `(name, value)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const NI_GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
/// The viewer-string array's grow (`0096ad30`) and store (`0096ae90`).
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The `NiRTTI` record of the class (its first word is the class name).
const ROTATION_MODIFIER_RTTI: u32 = 0x0120_2528;
/// The viewer-string names: "Initial Rotation Speed", "Initial Rotation
/// Speed Variation", "Random Rot Speed Sign", "Initial Rotation Angle",
/// "Initial Rotation Angle Variation", "Random Initial Axis", "Initial Axis".
const NAME_SPEED: u32 = 0x010b_dc30;
const NAME_SPEED_VARIATION: u32 = 0x010b_dc0c;
const NAME_RANDOM_SIGN: u32 = 0x010b_dbf4;
const NAME_ANGLE: u32 = 0x010b_dbdc;
const NAME_ANGLE_VARIATION: u32 = 0x010b_dbb8;
const NAME_RANDOM_AXIS: u32 = 0x010b_dba4;
const NAME_AXIS: u32 = 0x010b_db94;
/// `NiStream`'s output stream (+0x250); its write function is at +0xC
/// (`cdecl (stream, source, size, &count, 1)`), its position at +4.
const STREAM_OUTPUT: u32 = 0x250;

/// Appends `text` to the viewer-string array (the inlined `Add`).
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

/// One write through the stream's output: `size` bytes from `source`, the
/// count word at `count` set to `size` first; the result is added to the
/// output's position.
fn stream_write(e: &mut Engine, stream: Ptr, source: u32, size: u32, count: Ptr) {
    let output = e.mem.u32(stream.addr() + STREAM_OUTPUT);
    let write = e.mem.u32(output + 0xc);
    e.mem.set_u32(count.addr(), size);
    let put = e
        .call(write, &args![output, source, size, count, 1u32])
        .u32();
    let position = e.mem.u32(output + 4);
    e.mem.set_u32(output + 4, position.wrapping_add(put));
}

// Translated from 00c20a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRotationModifier::IsEqual` (Xbox PDB): the modifier comparison
/// (`00c31830`), then the speed, its variation (floats, `FUCOMPP`: NaN is
/// unequal), the random-sign byte, the angle, its variation, the random-axis
/// byte, and last the axes (`00439090` on the other's axis, which must
/// answer 0).
pub fn ni_p_sys_rotation_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysRotationModifier>,
    other: Ptr<NiPSysRotationModifier>,
) -> bool {
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let float_equal =
        |e: &Engine, off: u32| e.mem.f32(other.addr() + off) == e.mem.f32(this.addr() + off);
    if !float_equal(e, NiPSysRotationModifier::m_fInitialRotSpeed.off)
        || !float_equal(e, NiPSysRotationModifier::m_fInitialRotSpeedVar.off)
        || e.get(other, NiPSysRotationModifier::m_bRandomRotSpeedSign)
            != e.get(this, NiPSysRotationModifier::m_bRandomRotSpeedSign)
        || !float_equal(e, NiPSysRotationModifier::m_fInitialRotAngle.off)
        || !float_equal(e, NiPSysRotationModifier::m_fInitialRotAngleVar.off)
        || e.get(other, NiPSysRotationModifier::m_bRandomInitialAxis)
            != e.get(this, NiPSysRotationModifier::m_bRandomInitialAxis)
    {
        return false;
    }
    let axis = NiPSysRotationModifier::m_kInitialAxis.off;
    !e.call(
        POINT_DIFFERS,
        &args![other.addr() + axis, this.addr() + axis],
    )
    .bool()
}

// Translated from 00c20c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRotationModifier::SaveBinary` (Xbox PDB): the modifier part
/// (`00c31950`), then the speed, its variation, the angle and its variation
/// (four bytes each, the count word in the argument slot), the random-sign
/// byte and the random-axis byte (one byte each, copied into the low byte of
/// the argument slot, the count in the local word), then the axis
/// (`NiColor::SaveBinary`, `00a69780`).
pub fn ni_p_sys_rotation_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysRotationModifier>,
    stream: Ptr,
) {
    e.call(MODIFIER_SAVE_BINARY, &args![this, stream]);
    // [0] the local count word, [4] the argument slot (holding `stream`).
    e.with_stack(8, |e, frame| {
        let local = frame;
        let slot = Ptr::new(frame.addr() + 4);
        e.mem.set_u32(slot.addr(), stream.addr());
        for field in [
            NiPSysRotationModifier::m_fInitialRotSpeed.off,
            NiPSysRotationModifier::m_fInitialRotSpeedVar.off,
            NiPSysRotationModifier::m_fInitialRotAngle.off,
            NiPSysRotationModifier::m_fInitialRotAngleVar.off,
        ] {
            stream_write(e, stream, this.addr() + field, 4, slot);
        }
        for field in [
            NiPSysRotationModifier::m_bRandomRotSpeedSign,
            NiPSysRotationModifier::m_bRandomInitialAxis,
        ] {
            let byte = e.get(this, field);
            e.mem.set_u8(slot.addr(), byte);
            stream_write(e, stream, slot.addr(), 1, local);
        }
    });
    e.call(
        COLOR_SAVE_BINARY,
        &args![
            this.addr() + NiPSysRotationModifier::m_kInitialAxis.off,
            stream
        ],
    );
}

// Translated from 00c20d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRotationModifier::GetViewerStrings` (Xbox PDB): the modifier
/// strings (`00c319e0`), then the class name, the speed, its variation, the
/// random-sign flag, the angle, its variation, the random-axis flag (floats
/// pushed by `FLD`/`FSTP`, quieting a signalling NaN; flags zero-extended)
/// and the axis (`00a59af0`), each appended to the array.
pub fn ni_p_sys_rotation_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysRotationModifier>,
    strings: Ptr,
) {
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(ROTATION_MODIFIER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let float_string = |e: &mut Engine, name: u32, value: u32| {
        let value = x87_quiet_bits(value);
        e.call(NI_GET_VIEWER_STRING_FLOAT, &args![name, value])
            .u32()
    };
    let bool_string = |e: &mut Engine, name: u32, value: u8| {
        e.call(NI_GET_VIEWER_STRING_BOOL, &args![name, u32::from(value)])
            .u32()
    };
    let value = e.get(this, NiPSysRotationModifier::m_fInitialRotSpeed);
    let text = float_string(e, NAME_SPEED, value);
    push_viewer_string(e, strings, text);
    let value = e.get(this, NiPSysRotationModifier::m_fInitialRotSpeedVar);
    let text = float_string(e, NAME_SPEED_VARIATION, value);
    push_viewer_string(e, strings, text);
    let value = e.get(this, NiPSysRotationModifier::m_bRandomRotSpeedSign);
    let text = bool_string(e, NAME_RANDOM_SIGN, value);
    push_viewer_string(e, strings, text);
    let value = e.get(this, NiPSysRotationModifier::m_fInitialRotAngle);
    let text = float_string(e, NAME_ANGLE, value);
    push_viewer_string(e, strings, text);
    let value = e.get(this, NiPSysRotationModifier::m_fInitialRotAngleVar);
    let text = float_string(e, NAME_ANGLE_VARIATION, value);
    push_viewer_string(e, strings, text);
    let value = e.get(this, NiPSysRotationModifier::m_bRandomInitialAxis);
    let text = bool_string(e, NAME_RANDOM_AXIS, value);
    push_viewer_string(e, strings, text);
    let axis = this.addr() + NiPSysRotationModifier::m_kInitialAxis.off;
    let text = e
        .call(POINT_GET_VIEWER_STRING, &args![axis, NAME_AXIS])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_0a60,
            ni_p_sys_rotation_modifier_is_equal(
                Ptr<NiPSysRotationModifier>,
                Ptr<NiPSysRotationModifier>,
            ) -> bool
        ),
        entry!(
            0x00c2_0c00,
            ni_p_sys_rotation_modifier_save_binary(Ptr<NiPSysRotationModifier>, Ptr)
        ),
        entry!(
            0x00c2_0d00,
            ni_p_sys_rotation_modifier_get_viewer_strings(Ptr<NiPSysRotationModifier>, Ptr)
        ),
        entry!(
            0x00c2_05e0,
            fn_00c205e0(Ptr<NiPSysRotationModifier>, Ptr<NiPSysRotationModifier>, Ptr)
        ),
        entry!(0x00c2_0640, fn_00c20640() -> Ptr<NiPSysRotationModifier>),
        entry!(
            0x00c2_06b0,
            ni_p_sys_rotation_modifier_update(Ptr<NiPSysRotationModifier>, f32, Ptr<NiPSysData>)
        ),
        entry!(
            0x00c2_07a0,
            ni_p_sys_rotation_modifier_initialize(
                Ptr<NiPSysRotationModifier>,
                Ptr<NiPSysData>,
                u16,
            )
        ),
        entry!(
            0x00c2_0980,
            ni_p_sys_rotation_modifier_create_clone(
                Ptr<NiPSysRotationModifier>,
                Ptr,
            ) -> Ptr<NiPSysRotationModifier>
        ),
        entry!(
            0x00c2_0a00,
            ni_p_sys_rotation_modifier_post_link_object(Ptr<NiPSysRotationModifier>, Ptr)
        ),
        entry!(
            0x00c2_0ae0,
            ni_p_sys_rotation_modifier_load_binary(Ptr<NiPSysRotationModifier>, Ptr)
        ),
        entry!(0x00c206a0, ni_p_sys_rotation_modifier_get_r_t_t_i() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_rotation_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_06a0, &args![]).u32(), 0x0120_2528);
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

    fn modifier(e: &mut Engine) -> Ptr<NiPSysRotationModifier> {
        e.new_object::<NiPSysRotationModifier>()
    }

    #[test]
    fn copy_members_copies_the_rotation_fields() {
        let mut e = Engine::new();
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = modifier(&mut e);
        let dest = modifier(&mut e);
        for off in (0x18..0x34).step_by(4) {
            e.mem.set_u32(this.addr() + off, 0x4000_0000 + off);
        }
        e.set(
            this,
            NiPSysRotationModifier::m_fInitialRotAngle,
            0xff80_0001,
        );
        e.set(this, NiPSysRotationModifier::m_kInitialAxis, 0x7f80_0001);
        e.set(this, NiPSysRotationModifier::m_bRandomInitialAxis, 1);
        e.set(this, NiPSysRotationModifier::m_bRandomRotSpeedSign, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_05e0, &args![this, dest, 0x4000u32]);
        assert_eq!(
            log_of(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![this.addr(), dest.addr(), 0x4000]]
        );
        assert_eq!(
            e.get(dest, NiPSysRotationModifier::m_fInitialRotAngle),
            0xffc0_0001
        );
        assert_eq!(
            e.get(dest, NiPSysRotationModifier::m_kInitialAxis),
            0x7f80_0001
        );
        for off in [0x18u32, 0x1c, 0x24, 0x2c, 0x30] {
            assert_eq!(e.mem.u32(dest.addr() + off), 0x4000_0000 + off);
        }
        assert_eq!(e.get(dest, NiPSysRotationModifier::m_bRandomInitialAxis), 1);
        assert_eq!(
            e.get(dest, NiPSysRotationModifier::m_bRandomRotSpeedSign),
            1
        );
    }

    fn construction_doubles(e: &mut Engine) {
        e.map(DEFAULT_AXIS & !0xfff, 0x1000);
        e.set_global(DEFAULT_AXIS, 1.0f32);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
    }

    #[test]
    fn factory_constructs_the_defaults_or_answers_null() {
        let mut e = Engine::new();
        construction_doubles(&mut e);
        let made = e
            .call(0x00c2_0640, &args![])
            .ptr::<NiPSysRotationModifier>();
        assert_eq!(e.mem.u32(made.addr()), VTABLE);
        assert_eq!(
            e.get(made, NiPSysRotationModifier::m_kInitialAxis),
            1.0f32.to_bits()
        );
        assert_eq!(e.get(made, NiPSysRotationModifier::m_bRandomInitialAxis), 1);
        assert_eq!(
            e.get(made, NiPSysRotationModifier::m_bRandomRotSpeedSign),
            0
        );
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_0640, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_constructs_then_copies() {
        let mut e = Engine::new();
        construction_doubles(&mut e);
        let this = modifier(&mut e);
        e.set(
            this,
            NiPSysRotationModifier::m_fInitialRotSpeed,
            3.0f32.to_bits(),
        );
        let clone = e
            .call(0x00c2_0980, &args![this, 0x4000u32])
            .ptr::<NiPSysRotationModifier>();
        assert_eq!(e.mem.u32(clone.addr()), VTABLE);
        assert_eq!(
            e.get(clone, NiPSysRotationModifier::m_fInitialRotSpeed),
            3.0f32.to_bits()
        );
    }

    fn data_with(e: &mut Engine, angles: &[f32], speeds: &[f32], last: f32) -> Ptr<NiPSysData> {
        let data = e.new_object::<NiPSysData>();
        let n = angles.len() as u32;
        let a = e.mem.alloc(4 * n);
        let s = e.mem.alloc(4 * n);
        let info = e.mem.alloc(0x1c * n);
        for i in 0..n {
            e.mem.set_f32(a + 4 * i, angles[i as usize]);
            e.mem.set_f32(s + 4 * i, speeds[i as usize]);
            e.mem.set_f32(info + 0x1c * i + 0x14, last);
        }
        e.set(data, NiPSysData::m_pfRotationAngles, a);
        e.set(data, NiPSysData::m_pfRotationSpeeds, s);
        e.set(data, NiPSysData::m_pkParticleInfo, info);
        e.set(data, NiPSysData::m_usActive, n as u16);
        data
    }

    #[test]
    fn update_advances_wraps_and_resets_the_angles() {
        let mut e = Engine::new();
        e.map(TWO_PI & !0xfff, 0x1000);
        e.set_global(TWO_PI, std::f32::consts::TAU);
        e.map(STATIC_LIMIT_INIT & !0xfff, 0x1000);
        e.set_global(STATIC_LIMIT_INIT, 31.415928f32);
        e.map(STATIC_LIMIT & !0xfff, 0x1000);
        let data = data_with(
            &mut e,
            &[1.0, 40.0, 7.0, f32::NAN],
            &[1.0, 1.0, 1.0, 1.0],
            0.0,
        );
        e.call(0x00c2_06b0, &args![0u32, 0.5f32, data]);
        let a = e.get(data, NiPSysData::m_pfRotationAngles);
        assert_eq!(e.mem.f32(a), 1.5);
        assert_eq!(e.mem.f32(a + 4), 0.0);
        let wrapped = (7.5f32 as f64 - std::f32::consts::TAU as f64) as f32;
        assert_eq!(e.mem.f32(a + 8), wrapped);
        assert!(e.mem.f32(a + 12).is_nan());
        assert_eq!(e.global::<u32>(STATIC_GUARD) & 1, 1);
        assert_eq!(e.global::<f32>(STATIC_LIMIT), 31.415928);
        // Without angles nothing happens (and the guard is not touched).
        e.set_global(STATIC_GUARD, 0u32);
        e.set(data, NiPSysData::m_pfRotationAngles, 0);
        e.call(0x00c2_06b0, &args![0u32, 0.5f32, data]);
        assert_eq!(e.global::<u32>(STATIC_GUARD), 0);
    }

    fn initialize_constants(e: &mut Engine, rands: Vec<u32>) {
        e.map(0x0101_1000, 0x2_0000);
        e.set_global(RAND_MAX_DOUBLE, 32767.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(TABLE_SIZE, 512.0f64);
        e.set_global(PI_DOUBLE, f64::from(std::f32::consts::PI));
        e.set_global(HALF_FLOAT, 0.5f32);
        e.map(TWO_PI & !0xfff, 0x1000);
        e.set_global(TWO_PI, std::f32::consts::TAU);
        e.map(TABLE_B, 0x1000);
        e.set_global(TABLE_A, 0.25f32);
        e.set_global(TABLE_B, 0.5f32);
        let mut values = rands.into_iter();
        e.register_double(RAND, move |_, _| Ret {
            eax: values.next().unwrap(),
            ..Ret::default()
        });
    }

    #[test]
    fn initialize_copies_the_axis_and_sets_angle_and_speed() {
        let mut e = Engine::new();
        initialize_constants(&mut e, vec![32767, 0]);
        let this = modifier(&mut e);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(this.addr() + 0x28 + 4 * i as u32, *v);
        }
        e.set(
            this,
            NiPSysRotationModifier::m_fInitialRotAngle,
            1.0f32.to_bits(),
        );
        e.set(
            this,
            NiPSysRotationModifier::m_fInitialRotAngleVar,
            0.5f32.to_bits(),
        );
        e.set(
            this,
            NiPSysRotationModifier::m_fInitialRotSpeed,
            2.0f32.to_bits(),
        );
        e.set(
            this,
            NiPSysRotationModifier::m_fInitialRotSpeedVar,
            1.0f32.to_bits(),
        );
        let data = data_with(&mut e, &[0.0, 0.0], &[0.0, 0.0], 0.0);
        let axes = e.mem.alloc(24);
        e.set(data, NiPSysData::m_pkRotationAxes, axes);
        e.call(0x00c2_07a0, &args![this, data, 0xffff_0001u32]);
        assert_eq!(e.mem.f32(axes + 0xc), 1.0);
        assert_eq!(e.mem.f32(axes + 0x14), 3.0);
        let a = e.get(data, NiPSysData::m_pfRotationAngles);
        let s = e.get(data, NiPSysData::m_pfRotationSpeeds);
        assert_eq!(e.mem.f32(a + 4), 1.5);
        assert_eq!(e.mem.f32(s + 4), 1.0);
    }

    #[test]
    fn initialize_random_axis_and_random_sign() {
        let mut e = Engine::new();
        initialize_constants(&mut e, vec![0, 0, 32767, 32767, 0]);
        let this = modifier(&mut e);
        e.set(this, NiPSysRotationModifier::m_bRandomInitialAxis, 1);
        e.set(this, NiPSysRotationModifier::m_bRandomRotSpeedSign, 1);
        e.set(
            this,
            NiPSysRotationModifier::m_fInitialRotSpeed,
            2.0f32.to_bits(),
        );
        let data = data_with(&mut e, &[0.0], &[0.0], 0.0);
        let axes = e.mem.alloc(12);
        e.set(data, NiPSysData::m_pkRotationAxes, axes);
        e.call_log = Some(vec![]);
        e.call(0x00c2_07a0, &args![this, data, 0u32]);
        let radius = 0.75f64.sqrt() as f32;
        assert_eq!(e.mem.f32(axes), (0.25 * f64::from(radius)) as f32);
        assert_eq!(e.mem.f32(axes + 4), (f64::from(radius) * 0.5) as f32);
        assert_eq!(e.mem.f32(axes + 8), 0.5);
        assert_eq!(log_of(&e, SQRT).len(), 1);
        let s = e.get(data, NiPSysData::m_pfRotationSpeeds);
        assert_eq!(e.mem.f32(s), -2.0);
    }

    #[test]
    fn post_link_object_sets_the_speeds_for_old_files_only() {
        for (version, expected) in [(0x1400_0001u32, 4.0f32), (0x1400_0002, 0.0)] {
            let mut e = Engine::new();
            e.register(BASE_POST_LINK_OBJECT, |_, _| Ret::default());
            let this = modifier(&mut e);
            e.set(
                this,
                NiPSysRotationModifier::m_fInitialRotSpeed,
                4.0f32.to_bits(),
            );
            let data = data_with(&mut e, &[0.0, 0.0], &[0.0, 0.0], 0.0);
            let system = e.mem.alloc(0xc0);
            e.mem.set_u32(system + SYSTEM_DATA, data.addr());
            e.mem.set_u32(this.addr() + 0x10, system);
            let stream = test_stream::new(&mut e, version, &[]);
            e.call(0x00c2_0a00, &args![this, stream]);
            let s = e.get(data, NiPSysData::m_pfRotationSpeeds);
            assert_eq!(e.mem.f32(s), expected);
            assert_eq!(e.mem.f32(s + 4), expected);
        }
    }

    #[test]
    fn load_binary_reads_the_fields_by_version() {
        for version in [0x1400_0002u32, 0x1400_0001] {
            let mut e = Engine::new();
            e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
            e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
            let mut bytes: Vec<u8> = 1.0f32.to_le_bytes().to_vec();
            if version >= VERSION_ROTATION_FIELDS {
                for v in [2.0f32, 3.0, 4.0] {
                    bytes.extend_from_slice(&v.to_le_bytes());
                }
                bytes.push(1);
            }
            bytes.push(0);
            let stream = test_stream::new(&mut e, version, &bytes);
            let this = modifier(&mut e);
            e.set(this, NiPSysRotationModifier::m_bRandomInitialAxis, 1);
            e.call_log = Some(vec![]);
            e.call(0x00c2_0ae0, &args![this, stream]);
            assert_eq!(
                log_of(&e, COLOR_LOAD_BINARY),
                vec![vec![this.addr() + 0x28, stream]]
            );
            let f = |e: &Engine, field| f32::from_bits(e.get(this, field));
            assert_eq!(f(&e, NiPSysRotationModifier::m_fInitialRotSpeed), 1.0);
            assert_eq!(e.get(this, NiPSysRotationModifier::m_bRandomInitialAxis), 0);
            if version >= VERSION_ROTATION_FIELDS {
                assert_eq!(f(&e, NiPSysRotationModifier::m_fInitialRotAngleVar), 4.0);
                assert_eq!(
                    e.get(this, NiPSysRotationModifier::m_bRandomRotSpeedSign),
                    1
                );
            } else {
                assert_eq!(f(&e, NiPSysRotationModifier::m_fInitialRotAngle), 0.0);
            }
            assert_eq!(test_stream::position(&e, stream), bytes.len() as u32);
        }
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

    fn modifier(e: &mut Engine) -> Ptr<NiPSysRotationModifier> {
        let m = Ptr::new(e.mem.alloc(0x38));
        for (i, off) in [0x18u32, 0x1c, 0x20, 0x24].iter().enumerate() {
            e.mem.set_f32(m.addr() + off, i as f32 + 0.5);
        }
        e.mem.set_u8(m.addr() + 0x34, 1);
        e.mem.set_u8(m.addr() + 0x35, 0);
        m
    }

    #[test]
    fn is_equal_checks_fields_in_order_and_the_axis_last() {
        let mut e = Engine::new();
        let this = modifier(&mut e);
        let other = modifier(&mut e);
        e.register(MODIFIER_IS_EQUAL, |_, _| word(1));
        e.register(POINT_DIFFERS, |_, _| word(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c2_0a60, &args![this, other]).bool());
        assert_eq!(
            calls_to(&e, POINT_DIFFERS),
            vec![vec![other.addr() + 0x28, this.addr() + 0x28]]
        );
        e.register(POINT_DIFFERS, |_, _| word(1));
        assert!(!e.call(0x00c2_0a60, &args![this, other]).bool());
        e.register(POINT_DIFFERS, |_, _| word(0));
        e.mem.set_u8(other.addr() + 0x35, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c2_0a60, &args![this, other]).bool());
        assert!(calls_to(&e, POINT_DIFFERS).is_empty());
        e.mem.set_u8(other.addr() + 0x35, 0);
        e.mem.set_u32(other.addr() + 0x24, 0x7fc0_0000);
        assert!(!e.call(0x00c2_0a60, &args![this, other]).bool());
        e.register(MODIFIER_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00c2_0a60, &args![this, this]).bool());
    }

    #[test]
    fn save_binary_writes_four_floats_two_bytes_and_the_axis() {
        const WRITE: u32 = 0x0100_0040;
        let mut e = Engine::new();
        e.register(MODIFIER_SAVE_BINARY, |_, _| Ret::default());
        e.register(COLOR_SAVE_BINARY, |_, _| Ret::default());
        let written = Rc::new(RefCell::new(vec![]));
        let log = written.clone();
        e.register_double(WRITE, move |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            log.borrow_mut().push((a[2], e.mem.u32(a[3]), bytes));
            word(a[2])
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = modifier(&mut e);
        e.mem.set_u8(this.addr() + 0x35, 7);
        e.call_log = Some(vec![]);
        e.call(0x00c2_0c00, &args![this, stream]);
        let written = written.borrow();
        assert_eq!(written.len(), 6);
        assert_eq!(written[0], (4, 4, 0.5f32.to_le_bytes().to_vec()));
        assert_eq!(written[3], (4, 4, 3.5f32.to_le_bytes().to_vec()));
        assert_eq!(written[4], (1, 1, vec![7]));
        assert_eq!(written[5], (1, 1, vec![1]));
        assert_eq!(e.mem.u32(output + 4), 18);
        assert_eq!(
            calls_to(&e, COLOR_SAVE_BINARY),
            vec![vec![this.addr() + 0x28, stream]]
        );
        assert_eq!(
            calls_to(&e, MODIFIER_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
    }

    #[test]
    fn get_viewer_strings_appends_eight_strings() {
        let mut e = Engine::new();
        e.map(ROTATION_MODIFIER_RTTI, 4);
        e.set_global(ROTATION_MODIFIER_RTTI, 0x0555_0000u32);
        e.register(MODIFIER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0000));
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, a| word(a[1]));
        e.register(NI_GET_VIEWER_STRING_BOOL, |_, a| word(0x6000_0000 | a[1]));
        e.register(POINT_GET_VIEWER_STRING, |_, _| word(0x5000_0000));
        e.register(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let stored = Rc::new(RefCell::new(vec![]));
        let seen = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[2]));
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let this = modifier(&mut e);
        e.mem.set_u32(this.addr() + 0x18, 0x7f80_0001);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_0d00, &args![this, strings]);
        assert_eq!(
            *stored.borrow(),
            vec![
                0x7000_0000,
                0x7fc0_0001,
                1.5f32.to_bits(),
                0x6000_0000,
                2.5f32.to_bits(),
                3.5f32.to_bits(),
                0x6000_0001,
                0x5000_0000,
            ]
        );
        assert_eq!(calls_to(&e, STRINGS_GROW).len(), 8);
        let names: Vec<u32> = calls_to(&e, NI_GET_VIEWER_STRING_FLOAT)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(
            names,
            [
                NAME_SPEED,
                NAME_SPEED_VARIATION,
                NAME_ANGLE,
                NAME_ANGLE_VARIATION
            ]
        );
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![vec![this.addr() + 0x28, NAME_AXIS]]
        );
    }
}
