//! `NiParticle/nipsysairfieldmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c2e520`: the jump target of `00c2f4e0`.
const JUMP_TARGET_00C2F4E0: u32 = 0x00c2_e520;

/// `00c28b60`: the jump target of `00c2f4f0`.
const JUMP_TARGET_00C2F4F0: u32 = 0x00c2_8b60;

// Translated from 00c2f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::LinkObject` (Xbox PDB): a 5-byte jump to `00c2e520` (engine map: `NiPSysBombModifier::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysAirFieldModifier@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_modifier_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C2F4E0, &args![this, stream]);
}

// Translated from 00c2f4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c28b60` (engine map: `NiPSysAgeDeathModifier::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysAirFieldModifier@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_air_field_modifier_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C2F4F0, &args![this, stream]).bool()
}

// Translated from 00c2f590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27a8.
pub fn ni_p_sys_air_field_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27a8
}

/// `NiPSysModifier::ProcessClone` (`00c317e0`, `thiscall`, one word).
const MODIFIER_PROCESS_CLONE: u32 = 0x00c3_17e0;
/// `00a53c30`: the cloning process's map lookup, `thiscall` on the
/// process's first word, `(key, &value)`; answers whether it found the key.
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiPSysFieldModifier::m_pkFieldObj` (Xbox PDB) +0x18: `NiAVObject*`.
const FIELD_OBJECT: u32 = 0x18;

// Translated from 00c2c110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::ProcessClone` (engine map): `NiPSysModifier`'s
/// `ProcessClone`, then the clone of `this` from the process's map (the
/// lookup's answer is not checked: the out word starts at 0) and, when the
/// field object is set, its clone from the map (the out word still holding
/// this object's clone) as the clone's field object, or the field object
/// itself when the map has none. The clone is not checked for null.
pub fn ni_p_sys_air_field_modifier_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(MODIFIER_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, out| {
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, out]);
        let clone = e.mem.u32(out.addr());
        let field = e.mem.u32(this.addr() + FIELD_OBJECT);
        if field == 0 {
            return;
        }
        let map = e.mem.u32(process.addr());
        if e.call(CLONE_MAP_LOOKUP, &args![map, field, out]).bool() {
            let field_clone = e.mem.u32(out.addr());
            e.mem.set_u32(clone + FIELD_OBJECT, field_clone);
        } else {
            let field = e.mem.u32(this.addr() + FIELD_OBJECT);
            e.mem.set_u32(clone + FIELD_OBJECT, field);
        }
    });
}

use crate::units::niparticle::nipsysdata::{add_viewer_string, stream_write};
use crate::units::niparticle::nipsysmodifier::stream_read;

layout! {
    /// `NiPSysAirFieldModifier` (Xbox PDB), 0x68 bytes; the PC code uses
    /// the same offsets. Its own fields (floats as their bit patterns);
    /// those of `NiPSysFieldModifier` (Xbox PDB) are read at their offsets.
    pub struct NiPSysAirFieldModifier: 0x68 {
        /// `m_kLastFieldPosition` (Xbox PDB): an `NiPoint3`.
        0x30 m_kLastFieldPosition: u32,
        /// `m_fLastUpdateTime` (Xbox PDB).
        0x3C m_fLastUpdateTime: u32,
        /// `m_kDirection` (Xbox PDB): an `NiPoint3`.
        0x40 m_kDirection: u32,
        /// `m_kUnitDirection` (Xbox PDB): an `NiPoint3`.
        0x4C m_kUnitDirection: u32,
        /// `m_fAirFriction` (Xbox PDB).
        0x58 m_fAirFriction: u32,
        /// `m_fInheritVelocity` (Xbox PDB).
        0x5C m_fInheritVelocity: u32,
        /// `m_bInheritRotation` (Xbox PDB).
        0x60 m_bInheritRotation: u8,
        /// `m_bComponentOnly` (Xbox PDB).
        0x61 m_bComponentOnly: u8,
        /// `m_bEnableSpread` (Xbox PDB).
        0x62 m_bEnableSpread: u8,
        /// `m_fSpread` (Xbox PDB).
        0x64 m_fSpread: u32,
    }
}

/// `NiPSysModifier::m_pkTarget` (+0x10) and the `NiPSysFieldModifier`
/// fields (Xbox PDB names, PC offsets): `m_pkFieldObj` (+0x18),
/// `m_fMagnitude` (+0x1C), `m_fAttenuation` (+0x20), `m_bUseMaxDistance`
/// (+0x24), `m_fMaxDistance` (+0x28) and `m_fMaxDistanceSqr` (+0x2C).
const TARGET: u32 = 0x10;
const MAGNITUDE: u32 = 0x1c;
const ATTENUATION: u32 = 0x20;
const USE_MAX_DISTANCE: u32 = 0x24;
const MAX_DISTANCE: u32 = 0x28;
const MAX_DISTANCE_SQUARED: u32 = 0x2c;
/// `NiPoint3::ZERO`'s three words (`011f426c`), the axis a zero direction
/// becomes (`011a946c`), the "never updated" position (`011af220`, three
/// `-FLT_MAX` by the exe's data) and the `float` the constructor gives the
/// last update time (`01016970`).
const ZERO_POINT: u32 = 0x011f_426c;
const DEFAULT_AXIS: u32 = 0x011a_946c;
const NO_POSITION: u32 = 0x011a_f220;
const DEFAULT_LAST_UPDATE: u32 = 0x0101_6970;
/// The constants the update compares and scales with: the `double` at
/// `010231b0` (a last update time meaning none), the `float` 0 at
/// `01011d78`, the spread's factor (`011af218`, a `float`), its divisor
/// (`011af21c`, a `float`) and the table size (`01028338`, a `double`); the
/// 512-entry `float` table the spread angle indexes (`011f4ae0`).
const NO_UPDATE_TIME: u32 = 0x0102_31b0;
const FLOAT_ZERO: u32 = 0x0101_1d78;
const SPREAD_FACTOR: u32 = 0x011a_f218;
const SPREAD_DIVISOR: u32 = 0x011a_f21c;
const TABLE_SIZE: u32 = 0x0102_8338;
const SPREAD_TABLE: u32 = 0x011f_4ae0;
/// The particle data (`NiPSysData`) fields the updates read: the active
/// count (+0x50), the positions (+0x20, 12 bytes each) and the particle
/// infos (+0x70, 0x1c bytes each: velocity at +0, last update at +0x14).
const DATA_ACTIVE: u32 = 0x50;
const DATA_VERTICES: u32 = 0x20;
const DATA_INFOS: u32 = 0x70;
const INFO_SIZE: u32 = 0x1c;
const INFO_LAST_UPDATE: u32 = 0x14;
/// `NiAVObject`'s world transform (`m_kWorld`, +0x68 as the PC code reads
/// it, 0x34 bytes; its translation at +0x24).
const WORLD_TRANSFORM: u32 = 0x68;
const TRANSFORM_SIZE: u32 = 0x34;
const TRANSLATION: u32 = 0x24;
/// `NiTransform::InvertNonUniform` (`004b4880`, Xbox PDB name), `0062c250`
/// (the transform product: result, right-hand transform), `004b4500` (a
/// point turned by the transform: result, point; returns the result),
/// `004a0c10` (normalizes a point in place, by its use here), `00439090`
/// (nonzero when two points differ, by its use here), `0053d280` (a point
/// divided by a `float`: result, divisor; returns the result) and
/// `NiPoint3::UnitizeGetLength` (`00457910`, Xbox PDB name: the length in
/// `ST0`); all `thiscall`.
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
const TRANSFORM_POINT: u32 = 0x004b_4500;
const POINT_UNITIZE: u32 = 0x004a_0c10;
const POINT_UNEQUAL: u32 = 0x0043_9090;
const POINT_DIVIDE: u32 = 0x0053_d280;
const POINT_UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
/// The CRT's square root (`00ec6040`), `_CIpow` (`00ec9b00`: base and
/// exponent as leading `f64`s) and `_ftol2` (`00ec62c0`).
const SQRT: u32 = 0x00ec_6040;
const POW: u32 = 0x00ec_9b00;
const FTOL: u32 = 0x00ec_62c0;
/// `00aa13e0` (`cdecl`, one word): the object allocation; `00aa1460`
/// (`cdecl`: block, size): the sized delete.
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiPSysFieldModifier`'s constructor (`00c31ff0`), member copy
/// (`00c32020`: clone, process), `IsEqual` (`00c32060`), `LoadBinary`
/// (`00c32120`), `SaveBinary` (`00c321f0`) and `GetViewerStrings`
/// (`00c322b0`), and `NiPSysModifier`'s destructor body (`00c31750`); all
/// `thiscall`.
const FIELD_MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1ff0;
const FIELD_MODIFIER_COPY_MEMBERS: u32 = 0x00c3_2020;
const FIELD_MODIFIER_IS_EQUAL: u32 = 0x00c3_2060;
const FIELD_MODIFIER_LOAD_BINARY: u32 = 0x00c3_2120;
const FIELD_MODIFIER_SAVE_BINARY: u32 = 0x00c3_21f0;
const FIELD_MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_22b0;
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
/// `NiColor::LoadBinary` (`00a59be0`), `NiColor::SaveBinary` (`00a69780`)
/// and `NiPoint3::GetViewerString` (`00a59af0`, a label) (Xbox PDB names;
/// `thiscall` on the point).
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// The vtable the constructor stores (`010c04cc`).
const VTABLE: u32 = 0x010c_04cc;
/// `NiGetViewerString` (`00a7d330`, the text), `_ov2` (`00a7d370`: a label
/// and a `bool`) and `_ov9` (`00a7d550`: a label and a `float`) (Xbox PDB
/// names; `cdecl`).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// `NiPSysAirFieldModifier::ms_RTTI` (its name is the first word).
const RTTI: u32 = 0x0120_27a8;
/// The labels "Direction" (`01044b44`), "Unit Direction" (`010c05cc`),
/// "AirFriction" (`010c05c0`), "Inherit Velocity" (`010c05ac`), "Inherit
/// Rotation" (`010c0598`), "Component Only" (`010c0588`), "Enable Spread"
/// (`010c0578`) and "Spread" (`010c0570`).
const LABEL_DIRECTION: u32 = 0x0104_4b44;
const LABEL_UNIT_DIRECTION: u32 = 0x010c_05cc;
const LABEL_AIR_FRICTION: u32 = 0x010c_05c0;
const LABEL_INHERIT_VELOCITY: u32 = 0x010c_05ac;
const LABEL_INHERIT_ROTATION: u32 = 0x010c_0598;
const LABEL_COMPONENT_ONLY: u32 = 0x010c_0588;
const LABEL_ENABLE_SPREAD: u32 = 0x010c_0578;
const LABEL_SPREAD: u32 = 0x010c_0570;

/// The three words of the point at `addr`.
fn point_words(e: &Engine, addr: u32) -> [u32; 3] {
    [e.mem.u32(addr), e.mem.u32(addr + 4), e.mem.u32(addr + 8)]
}

/// The three words `words` stored at `addr`.
fn set_point_words(e: &mut Engine, addr: u32, words: [u32; 3]) {
    for (i, word) in words.into_iter().enumerate() {
        e.mem.set_u32(addr + 4 * i as u32, word);
    }
}

/// The field object's frame the updates share: the inverse of the
/// particle system's world transform times the field object's (written to
/// `product`, 0x34 bytes; the copies and the inverse in `frame`, 0x68
/// bytes).
fn field_transform(e: &mut Engine, this: u32, frame: u32, product: u32) {
    let field_world = frame;
    let inverse = frame + TRANSFORM_SIZE;
    let field_object = e.mem.u32(this + FIELD_OBJECT);
    let words = e.mem.bytes(field_object + WORLD_TRANSFORM, TRANSFORM_SIZE);
    e.mem.write(field_world, &words);
    let target = e.mem.u32(this + TARGET);
    let words = e.mem.bytes(target + WORLD_TRANSFORM, TRANSFORM_SIZE);
    e.mem.write(product, &words);
    e.call(TRANSFORM_INVERT, &args![product, inverse]);
    e.call(TRANSFORM_MULTIPLY, &args![inverse, product, field_world]);
}

/// The field's direction in the system's space: the unit direction, turned
/// by `product` (`004b4500`, its result in `turned`) with inherited
/// rotation.
fn field_direction(
    e: &mut Engine,
    this: Ptr<NiPSysAirFieldModifier>,
    product: u32,
    turned: u32,
) -> [u32; 3] {
    type M = NiPSysAirFieldModifier;
    let unit = this.addr() + M::m_kUnitDirection.off;
    if e.get(this, M::m_bInheritRotation) != 0 {
        let result = e.call(TRANSFORM_POINT, &args![product, turned, unit]).u32();
        point_words(e, result)
    } else {
        point_words(e, unit)
    }
}

/// `direction * magnitude` as the updates compute it (`FMULP ST2`,
/// `FMUL ST1`, `FMUL dword`), each component stored as a `float`.
fn scaled_direction(e: &Engine, this: u32, direction: [u32; 3]) -> [u32; 3] {
    let magnitude = x87_load32(x87_quiet_bits(e.mem.u32(this + MAGNITUDE)));
    [
        x87_store32(x87_mul(x87_load32(direction[0]), magnitude)),
        x87_store32(x87_mul(x87_load32(direction[1]), magnitude)),
        x87_store32(x87_mul(magnitude, x87_m32(direction[2]))),
    ]
}

// Translated from 00c2f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`SetDirection` by its body): the direction
/// and the unit direction become `*direction`, the unit direction is
/// normalized (`004a0c10`), and a unit direction equal to `NiPoint3::ZERO`
/// (`FUCOMPP` each: a NaN differs) becomes the axis at `011a946c`.
pub fn fn_00c2f500(e: &mut Engine, this: Ptr<NiPSysAirFieldModifier>, direction: u32) {
    type M = NiPSysAirFieldModifier;
    let words = point_words(e, direction);
    set_point_words(e, this.addr() + M::m_kDirection.off, words);
    let unit = this.addr() + M::m_kUnitDirection.off;
    let words = point_words(e, direction);
    set_point_words(e, unit, words);
    e.call(POINT_UNITIZE, &args![unit]);
    for i in 0..3 {
        let component = x87_load32(e.mem.u32(unit + 4 * i));
        let zero = x87_load32(e.global::<u32>(ZERO_POINT + 4 * i));
        if zero != component {
            return;
        }
    }
    let axis = [
        e.global::<u32>(DEFAULT_AXIS),
        e.global::<u32>(DEFAULT_AXIS + 4),
        e.global::<u32>(DEFAULT_AXIS + 8),
    ];
    set_point_words(e, unit, axis);
}

// Translated from 00c2f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::_scalar_deleting_destructor_` (Xbox PDB):
/// `NiPSysModifier`'s destructor body, then the sized delete of 0x68 bytes
/// when bit 0 of `flags` is set. Returns `this`.
pub fn ni_p_sys_air_field_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPSysAirFieldModifier>,
    flags: u32,
) -> Ptr<NiPSysAirFieldModifier> {
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x68u32]);
    }
    this
}

// Translated from 00c2f5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysAirFieldModifier` constructor.
/// `NiPSysFieldModifier`'s constructor, the vtable, the direction set
/// (`00c2f500`) to the negated axis at `011a946c` (`FCHS` on each loaded
/// `float`), the last field position from `011af220` (by `MOV`) and the
/// last update time from `01016970`. Returns `this`.
pub fn fn_00c2f5c0(
    e: &mut Engine,
    this: Ptr<NiPSysAirFieldModifier>,
) -> Ptr<NiPSysAirFieldModifier> {
    type M = NiPSysAirFieldModifier;
    e.call(FIELD_MODIFIER_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.with_stack(12, |e, local| {
        for i in 0..3 {
            let axis = x87_load32(e.global::<u32>(DEFAULT_AXIS + 4 * i));
            e.mem.set_u32(local.addr() + 4 * i, x87_store32(-axis));
        }
        fn_00c2f500(e, this, local.addr());
    });
    let position = [
        e.global::<u32>(NO_POSITION),
        e.global::<u32>(NO_POSITION + 4),
        e.global::<u32>(NO_POSITION + 8),
    ];
    let last_update = x87_quiet_bits(e.global::<u32>(DEFAULT_LAST_UPDATE));
    set_point_words(e, this.addr() + M::m_kLastFieldPosition.off, position);
    e.set(this, M::m_fLastUpdateTime, last_update);
    this
}

// Translated from 00c2f630 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project: the `NiPSysAirFieldModifier`
/// stream factory (`cdecl`, no arguments; registered under its class
/// name): a 0x68-byte object from `00aa13e0` given the constructor
/// `00c2f5c0` (a tail jump); null when the allocation fails.
pub fn fn_00c2f630(e: &mut Engine) -> Ptr<NiPSysAirFieldModifier> {
    let block = e.call(NI_NEW, &args![0x68u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00c2f5c0(e, Ptr::new(block))
}

// Translated from 00c2f650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the air field update without attenuation,
/// inherited velocity or spread, with a distance limit. With a magnitude
/// other than 0: the last update time becomes `time` and the last field
/// position the field object's position in the system's space; the wind is
/// the direction (turned with inherited rotation) times the magnitude. Each
/// active particle with an elapsed time other than 0, within
/// `m_fMaxDistanceSqr` (squared) of the field object, and whose velocity
/// relative to the wind points against it (dot product below the `float`
/// at `01011d78`) gains `wind * air friction * elapsed`. The x87
/// arithmetic is done instruction by instruction in the exe's order.
pub fn fn_00c2f650(e: &mut Engine, this: Ptr<NiPSysAirFieldModifier>, time: f32, data: Ptr) {
    air_update_without_attenuation(e, this, time, data.addr(), true);
}

// Translated from 00c2f8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the air field update of `00c2f650` without
/// the distance limit (and a register 0.0 for the dot product's test; the
/// last velocity component is added as `velocity + change` rather than
/// `change + velocity`).
pub fn fn_00c2f8d0(e: &mut Engine, this: Ptr<NiPSysAirFieldModifier>, time: f32, data: Ptr) {
    air_update_without_attenuation(e, this, time, data.addr(), false);
}

/// The body of `00c2f650` (`limited`) and `00c2f8d0`, which differ in the
/// distance test, the dot product's zero and the order of the second and
/// third additions.
fn air_update_without_attenuation(
    e: &mut Engine,
    this: Ptr<NiPSysAirFieldModifier>,
    time: f32,
    data: u32,
    limited: bool,
) {
    type M = NiPSysAirFieldModifier;
    if 0.0 == x87_m32(e.mem.u32(this.addr() + MAGNITUDE)) {
        return;
    }
    e.with_stack(0xc0, |e, frame| {
        let product = frame.addr() + 0x68;
        let turned = frame.addr() + 0xa0;
        field_transform(e, this.addr(), frame.addr(), product);
        e.set(this, M::m_fLastUpdateTime, x87_quiet_bits(time.to_bits()));
        let origin = point_words(e, product + TRANSLATION);
        set_point_words(e, this.addr() + M::m_kLastFieldPosition.off, origin);
        let direction = field_direction(e, this, product, turned);
        let wind = scaled_direction(e, this.addr(), direction).map(x87_load32);
        let mut index: u16 = 0;
        while index < e.mem.u16(data + DATA_ACTIVE) {
            let info = e
                .mem
                .u32(data + DATA_INFOS)
                .wrapping_add(u32::from(index) * INFO_SIZE);
            let current = index;
            index = index.wrapping_add(1);
            let elapsed = x87_sub(
                x87_load32(time.to_bits()),
                x87_m32(e.mem.u32(info + INFO_LAST_UPDATE)),
            );
            let elapsed = x87_load32(x87_store32(elapsed));
            if elapsed == 0.0 {
                continue;
            }
            if limited {
                let vertex = e
                    .mem
                    .u32(data + DATA_VERTICES)
                    .wrapping_add(u32::from(current) * 12);
                let position = point_words(e, vertex);
                let offset: [f64; 3] = std::array::from_fn(|i| {
                    x87_load32(x87_store32(x87_sub(
                        x87_load32(position[i]),
                        x87_m32(origin[i]),
                    )))
                });
                let xx = x87_mul(offset[0], offset[0]);
                let yy = x87_mul(offset[1], offset[1]);
                let zz = x87_mul(offset[2], offset[2]);
                let squared = x87_store32(x87_add(x87_add(yy, xx), zz));
                let limit = x87_load32(e.mem.u32(this.addr() + MAX_DISTANCE_SQUARED));
                if !(limit >= x87_load32(squared)) {
                    continue;
                }
            }
            let velocity = point_words(e, info);
            let relative: [f64; 3] = std::array::from_fn(|i| {
                x87_load32(x87_store32(x87_sub(x87_load32(velocity[i]), wind[i])))
            });
            let along = x87_add(x87_mul(relative[1], wind[1]), x87_mul(relative[0], wind[0]));
            let dot = x87_store32(x87_add(along, x87_mul(relative[2], wind[2])));
            let zero = if limited {
                x87_m32(e.global::<u32>(FLOAT_ZERO))
            } else {
                0.0
            };
            if !(x87_load32(dot) < zero) {
                continue;
            }
            let friction = x87_load32(x87_quiet_bits(e.get(this, M::m_fAirFriction)));
            let push = [
                x87_store32(x87_mul(friction, wind[0])),
                x87_store32(x87_mul(friction, wind[1])),
                x87_store32(x87_mul(friction, wind[2])),
            ];
            let change = [
                x87_store32(x87_mul(x87_load32(push[0]), elapsed)),
                x87_store32(x87_mul(x87_load32(push[1]), elapsed)),
                x87_store32(x87_mul(elapsed, x87_m32(push[2]))),
            ];
            let x = x87_add(x87_load32(e.mem.u32(info)), x87_m32(change[0]));
            e.mem.set_u32(info, x87_store32(x));
            if limited {
                let y = x87_add(x87_load32(e.mem.u32(info + 4)), x87_m32(change[1]));
                e.mem.set_u32(info + 4, x87_store32(y));
                let z = x87_add(x87_load32(change[2]), x87_m32(e.mem.u32(info + 8)));
                e.mem.set_u32(info + 8, x87_store32(z));
            } else {
                let y = x87_add(x87_load32(change[1]), x87_m32(e.mem.u32(info + 4)));
                e.mem.set_u32(info + 4, x87_store32(y));
                let z = x87_add(x87_load32(e.mem.u32(info + 8)), x87_m32(change[2]));
                e.mem.set_u32(info + 8, x87_store32(z));
            }
        }
    });
}

// Translated from 00c2fad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::CreateClone` (Xbox PDB): a 0x68-byte object
/// from `00aa13e0` through the constructor `00c2f5c0` (null when the
/// allocation fails), `NiPSysFieldModifier`'s member copy into it, then
/// this one's direction set on it (`00c2f500`). Returns the clone.
pub fn ni_p_sys_air_field_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysAirFieldModifier>,
    process: Ptr,
) -> Ptr<NiPSysAirFieldModifier> {
    let block = e.call(NI_NEW, &args![0x68u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        fn_00c2f5c0(e, Ptr::new(block))
    };
    e.call(FIELD_MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let direction = this.addr() + NiPSysAirFieldModifier::m_kDirection.off;
    fn_00c2f500(e, clone, direction);
    clone
}

// Translated from 00c2fb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::IsEqual` (Xbox PDB): `NiPSysFieldModifier`'s,
/// then equal directions (`00439090` on the other's answering 0).
pub fn ni_p_sys_air_field_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysAirFieldModifier>,
    other: Ptr<NiPSysAirFieldModifier>,
) -> bool {
    if !e.call(FIELD_MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let offset = NiPSysAirFieldModifier::m_kDirection.off;
    !e.call(
        POINT_UNEQUAL,
        &args![other.addr() + offset, this.addr() + offset],
    )
    .bool()
}

// Translated from 00c2fb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::Update` (Xbox PDB): nothing without particles
/// or a field object. Without attenuation, inherited velocity and spread
/// the simpler updates run (`00c2f650` with a distance limit, `00c2f8d0`
/// without). Otherwise: the field object's position in the system's space;
/// the wind is the field's own motion since the last update (the position
/// change times the inherited velocity, divided by the elapsed time, when
/// there was a last update at another time) plus the direction (turned
/// with inherited rotation) times the magnitude; the last update time and
/// position are refreshed. With a wind of length 0 nothing more happens.
/// With spread, the cosine table entry of `spread * K / (2π / 512)` limits
/// the particles to those whose direction from the field object is within
/// that angle of the wind, and their wind points that way. Each particle
/// with an elapsed time other than 0, within the distance limit (when
/// used) and moving slower than the wind along it gains `wind * air
/// friction * elapsed`, attenuated by `(1 - distance / max distance) ^
/// attenuation` with a distance limit and an attenuation. The x87
/// arithmetic is done instruction by instruction in the exe's order.
pub fn ni_p_sys_air_field_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysAirFieldModifier>,
    time: f32,
    data: Ptr,
) {
    type M = NiPSysAirFieldModifier;
    let data_addr = data.addr();
    if e.mem.u16(data_addr + DATA_ACTIVE) == 0 {
        return;
    }
    if e.mem.u32(this.addr() + FIELD_OBJECT) == 0 {
        return;
    }
    if 0.0 == x87_m32(e.mem.u32(this.addr() + ATTENUATION))
        && 0.0 == x87_m32(e.get(this, M::m_fInheritVelocity))
        && e.get(this, M::m_bEnableSpread) == 0
    {
        let time = f32::from_bits(x87_quiet_bits(time.to_bits()));
        if e.mem.u8(this.addr() + USE_MAX_DISTANCE) != 0 {
            fn_00c2f650(e, this, time, data);
        } else {
            fn_00c2f8d0(e, this, time, data);
        }
        return;
    }
    e.with_stack(0x100, |e, frame| {
        let product = frame.addr() + 0x68;
        let turned = frame.addr() + 0xa0;
        let motion = frame.addr() + 0xb0;
        let divided = frame.addr() + 0xc0;
        let unit = frame.addr() + 0xd0;
        let toward = frame.addr() + 0xe0;
        field_transform(e, this.addr(), frame.addr(), product);
        let direction = field_direction(e, this, product, turned);
        let mut wind = point_words(e, ZERO_POINT);
        let last_position = this.addr() + M::m_kLastFieldPosition.off;
        let origin = point_words(e, product + TRANSLATION);
        let t = x87_load32(time.to_bits());
        let moved = e
            .call(POINT_UNEQUAL, &args![last_position, NO_POSITION])
            .bool();
        if moved {
            let last = x87_load32(e.get(this, M::m_fLastUpdateTime));
            if !(last == e.global::<f64>(NO_UPDATE_TIME)) && !(t == last) {
                let delta: [u32; 3] = std::array::from_fn(|i| {
                    let previous = x87_m32(e.mem.u32(last_position + 4 * i as u32));
                    x87_store32(x87_sub(x87_load32(origin[i]), previous))
                });
                let inherit = x87_load32(x87_quiet_bits(e.get(this, M::m_fInheritVelocity)));
                let scaled = [
                    x87_store32(x87_mul(x87_load32(delta[0]), inherit)),
                    x87_store32(x87_mul(x87_load32(delta[1]), inherit)),
                    x87_store32(x87_mul(inherit, x87_m32(delta[2]))),
                ];
                set_point_words(e, motion, scaled);
                let elapsed = x87_store32(x87_sub(t, x87_m32(e.get(this, M::m_fLastUpdateTime))));
                let result = e
                    .call(
                        POINT_DIVIDE,
                        &args![motion, divided, x87_quiet_bits(elapsed)],
                    )
                    .u32();
                wind = point_words(e, result);
            }
        }
        e.set(this, M::m_fLastUpdateTime, x87_store32(t));
        set_point_words(e, last_position, origin);
        let push = scaled_direction(e, this.addr(), direction);
        wind = [
            x87_store32(x87_add(x87_load32(push[0]), x87_m32(wind[0]))),
            x87_store32(x87_add(x87_load32(wind[1]), x87_m32(push[1]))),
            x87_store32(x87_add(x87_load32(wind[2]), x87_m32(push[2]))),
        ];
        let w = wind.map(x87_load32);
        let xx = x87_mul(w[0], w[0]);
        let yy = x87_mul(w[1], w[1]);
        let zz = x87_mul(w[2], w[2]);
        let squared = x87_store32(x87_add(zz, x87_add(xx, yy)));
        if x87_load32(squared) == x87_m32(e.global::<u32>(FLOAT_ZERO)) {
            return;
        }
        set_point_words(e, unit, wind);
        let length = x87_store32(e.call(POINT_UNITIZE_GET_LENGTH, &args![unit]).st0);
        let mut cos_spread = 0.0f32.to_bits();
        if e.get(this, M::m_bEnableSpread) != 0 {
            let spread = x87_mul(
                x87_load32(e.get(this, M::m_fSpread)),
                x87_m32(e.global::<u32>(SPREAD_FACTOR)),
            );
            let spread = x87_load32(x87_store32(spread));
            let step = x87_div(
                e.global::<f64>(TABLE_SIZE),
                x87_load32(e.global::<u32>(SPREAD_DIVISOR)),
            );
            let angle = x87_mul(spread, step);
            let index = e.call(FTOL, &args![angle]).u32() & 0x1ff;
            cos_spread = x87_quiet_bits(e.global::<u32>(SPREAD_TABLE + index * 4));
        }
        let mut index: u16 = 0;
        while index < e.mem.u16(data_addr + DATA_ACTIVE) {
            let current = index;
            index = index.wrapping_add(1);
            let info = e
                .mem
                .u32(data_addr + DATA_INFOS)
                .wrapping_add(u32::from(current) * INFO_SIZE);
            let elapsed = x87_store32(x87_sub(
                x87_load32(time.to_bits()),
                x87_m32(e.mem.u32(info + INFO_LAST_UPDATE)),
            ));
            if 0.0 == x87_m32(elapsed) {
                continue;
            }
            let vertex = e
                .mem
                .u32(data_addr + DATA_VERTICES)
                .wrapping_add(u32::from(current) * 12);
            let position = point_words(e, vertex);
            let offset: [u32; 3] = std::array::from_fn(|i| {
                x87_store32(x87_sub(x87_load32(position[i]), x87_m32(origin[i])))
            });
            let o = offset.map(x87_load32);
            let xx = x87_mul(o[0], o[0]);
            let yy = x87_mul(o[1], o[1]);
            let zz = x87_mul(o[2], o[2]);
            let distance_squared = x87_store32(x87_add(x87_add(yy, xx), zz));
            let limited = e.mem.u8(this.addr() + USE_MAX_DISTANCE);
            if limited != 0 {
                let limit = x87_load32(e.mem.u32(this.addr() + MAX_DISTANCE_SQUARED));
                if !(limit >= x87_load32(distance_squared)) {
                    continue;
                }
            }
            let mut flow = wind;
            if e.get(this, M::m_bEnableSpread) != 0 {
                set_point_words(e, toward, offset);
                e.call(POINT_UNITIZE, &args![toward]);
                let q = point_words(e, toward);
                let u = point_words(e, unit);
                let yy = x87_mul(x87_load32(u[1]), x87_load32(q[1]));
                let xx = x87_mul(x87_load32(q[0]), x87_m32(u[0]));
                let sum = x87_add(yy, xx);
                let zz = x87_mul(x87_load32(u[2]), x87_load32(q[2]));
                let cosine = x87_store32(x87_add(zz, sum));
                if x87_load32(cos_spread) > x87_load32(cosine) {
                    continue;
                }
                let scale = x87_load32(length);
                flow = [
                    x87_store32(x87_mul(x87_load32(q[0]), scale)),
                    x87_store32(x87_mul(x87_load32(q[1]), scale)),
                    x87_store32(x87_mul(x87_load32(q[2]), scale)),
                ];
            }
            let velocity = point_words(e, info);
            let f = flow.map(x87_load32);
            let relative: [u32; 3] =
                std::array::from_fn(|i| x87_store32(x87_sub(x87_load32(velocity[i]), f[i])));
            let along = x87_add(
                x87_mul(x87_load32(relative[1]), f[1]),
                x87_mul(x87_load32(relative[0]), f[0]),
            );
            let dot = x87_store32(x87_add(along, x87_mul(x87_load32(relative[2]), f[2])));
            if !(0.0 > x87_load32(dot)) {
                continue;
            }
            let mut factor = x87_quiet_bits(elapsed);
            if limited != 0 && !(0.0 == x87_m32(e.mem.u32(this.addr() + ATTENUATION))) {
                let root = e.call(SQRT, &args![x87_load32(distance_squared)]).st0;
                let ratio = x87_div(
                    x87_load32(x87_store32(root)),
                    x87_m32(e.mem.u32(this.addr() + MAX_DISTANCE)),
                );
                let base = x87_load32(x87_store32(x87_sub(1.0, ratio)));
                let exponent = x87_load32(e.mem.u32(this.addr() + ATTENUATION));
                let power = e.call(POW, &args![base, exponent]).st0;
                let power = x87_load32(x87_store32(power));
                factor = x87_store32(x87_mul(power, x87_m32(elapsed)));
            }
            let friction = x87_load32(x87_quiet_bits(e.get(this, M::m_fAirFriction)));
            let gust = [
                x87_store32(x87_mul(f[0], friction)),
                x87_store32(x87_mul(f[1], friction)),
                x87_store32(x87_mul(friction, f[2])),
            ];
            let factor = x87_load32(factor);
            let change = [
                x87_store32(x87_mul(x87_load32(gust[0]), factor)),
                x87_store32(x87_mul(x87_load32(gust[1]), factor)),
                x87_store32(x87_mul(factor, x87_m32(gust[2]))),
            ];
            for i in 0..3 {
                let slot = info + 4 * i as u32;
                let value = x87_add(x87_load32(change[i]), x87_m32(velocity[i]));
                e.mem.set_u32(slot, x87_store32(value));
            }
        }
    });
}

// Translated from 00c30170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::LoadBinary` (Xbox PDB): `NiPSysFieldModifier`'s,
/// the direction (`NiColor::LoadBinary`) then set (`00c2f500`), the air
/// friction and inherited velocity (4 bytes each), the inherit-rotation,
/// component-only and enable-spread flags (a byte each, stored as 1 or 0)
/// and the spread (4 bytes).
pub fn ni_p_sys_air_field_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysAirFieldModifier>,
    stream: Ptr,
) {
    type M = NiPSysAirFieldModifier;
    let stream = stream.addr();
    e.call(FIELD_MODIFIER_LOAD_BINARY, &args![this, stream]);
    let direction = this.addr() + M::m_kDirection.off;
    e.call(POINT_LOAD_BINARY, &args![direction, stream]);
    fn_00c2f500(e, this, direction);
    e.with_stack(8, |e, locals| {
        let (byte, slot) = (locals.addr(), locals.addr() + 4);
        for field in [M::m_fAirFriction, M::m_fInheritVelocity] {
            stream_read(e, stream, this.addr() + field.off, 4, 4, slot);
        }
        for field in [
            M::m_bInheritRotation,
            M::m_bComponentOnly,
            M::m_bEnableSpread,
        ] {
            stream_read(e, stream, byte, 1, 1, slot);
            let flag = u8::from(e.mem.u8(byte) != 0);
            e.set(this, field, flag);
        }
        let spread = this.addr() + M::m_fSpread.off;
        stream_read(e, stream, spread, 4, 4, slot);
    });
}

// Translated from 00c302a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::SaveBinary` (Xbox PDB): `NiPSysFieldModifier`'s,
/// the direction (`NiColor::SaveBinary`), the air friction and inherited
/// velocity (4 bytes each), the three flags (a byte each) and the spread
/// (4 bytes).
pub fn ni_p_sys_air_field_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysAirFieldModifier>,
    stream: Ptr,
) {
    type M = NiPSysAirFieldModifier;
    let stream = stream.addr();
    e.call(FIELD_MODIFIER_SAVE_BINARY, &args![this, stream]);
    let direction = this.addr() + M::m_kDirection.off;
    e.call(POINT_SAVE_BINARY, &args![direction, stream]);
    e.with_stack(8, |e, locals| {
        let (byte, slot) = (locals.addr(), locals.addr() + 4);
        for field in [M::m_fAirFriction, M::m_fInheritVelocity] {
            stream_write(e, stream, this.addr() + field.off, 4, 4, slot);
        }
        for field in [
            M::m_bInheritRotation,
            M::m_bComponentOnly,
            M::m_bEnableSpread,
        ] {
            let flag = e.get(this, field);
            e.mem.set_u8(byte, flag);
            stream_write(e, stream, byte, 1, 1, slot);
        }
        let spread = this.addr() + M::m_fSpread.off;
        stream_write(e, stream, spread, 4, 4, slot);
    });
}

// Translated from 00c303c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysAirFieldModifier::GetViewerStrings` (Xbox PDB):
/// `NiPSysFieldModifier`'s strings, the class name, the direction and the
/// unit direction (`NiPoint3::GetViewerString`), the air friction, the
/// inherited velocity, the three flags and the spread.
pub fn ni_p_sys_air_field_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysAirFieldModifier>,
    strings: Ptr,
) {
    type M = NiPSysAirFieldModifier;
    let strings = strings.addr();
    e.call(FIELD_MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, string);
    for (label, offset) in [
        (LABEL_DIRECTION, M::m_kDirection.off),
        (LABEL_UNIT_DIRECTION, M::m_kUnitDirection.off),
    ] {
        let point = this.addr() + offset;
        let string = e.call(POINT_GET_VIEWER_STRING, &args![point, label]).u32();
        add_viewer_string(e, strings, string);
    }
    for (label, field) in [
        (LABEL_AIR_FRICTION, M::m_fAirFriction),
        (LABEL_INHERIT_VELOCITY, M::m_fInheritVelocity),
    ] {
        let value = x87_quiet_bits(e.get(this, field));
        let string = e.call(VIEWER_STRING_FLOAT, &args![label, value]).u32();
        add_viewer_string(e, strings, string);
    }
    for (label, field) in [
        (LABEL_INHERIT_ROTATION, M::m_bInheritRotation),
        (LABEL_COMPONENT_ONLY, M::m_bComponentOnly),
        (LABEL_ENABLE_SPREAD, M::m_bEnableSpread),
    ] {
        let flag = u32::from(e.get(this, field));
        let string = e.call(VIEWER_STRING_BOOL, &args![label, flag]).u32();
        add_viewer_string(e, strings, string);
    }
    let spread = x87_quiet_bits(e.get(this, M::m_fSpread));
    let string = e
        .call(VIEWER_STRING_FLOAT, &args![LABEL_SPREAD, spread])
        .u32();
    add_viewer_string(e, strings, string);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2f590, ni_p_sys_air_field_modifier_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_f4e0,
            ni_p_sys_air_field_modifier_link_object(Ptr, Ptr)
        ),
        entry!(0x00c2_f4f0, ni_p_sys_air_field_modifier_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_c110,
            ni_p_sys_air_field_modifier_process_clone(Ptr, Ptr)
        ),
        entry!(0x00c2_f500, fn_00c2f500(Ptr<NiPSysAirFieldModifier>, u32)),
        entry!(
            0x00c2_f5a0,
            ni_p_sys_air_field_modifier_scalar_deleting_destructor(
                Ptr<NiPSysAirFieldModifier>,
                u32,
            )
                -> Ptr<NiPSysAirFieldModifier>
        ),
        entry!(
            0x00c2_f5c0,
            fn_00c2f5c0(Ptr<NiPSysAirFieldModifier>) -> Ptr<NiPSysAirFieldModifier>
        ),
        entry!(0x00c2_f630, fn_00c2f630() -> Ptr<NiPSysAirFieldModifier>),
        entry!(
            0x00c2_f650,
            fn_00c2f650(Ptr<NiPSysAirFieldModifier>, f32, Ptr)
        ),
        entry!(
            0x00c2_f8d0,
            fn_00c2f8d0(Ptr<NiPSysAirFieldModifier>, f32, Ptr)
        ),
        entry!(
            0x00c2_fad0,
            ni_p_sys_air_field_modifier_create_clone(
                Ptr<NiPSysAirFieldModifier>,
                Ptr,
            ) -> Ptr<NiPSysAirFieldModifier>
        ),
        entry!(
            0x00c2_fb10,
            ni_p_sys_air_field_modifier_is_equal(
                Ptr<NiPSysAirFieldModifier>,
                Ptr<NiPSysAirFieldModifier>,
            ) -> bool
        ),
        entry!(
            0x00c2_fb40,
            ni_p_sys_air_field_modifier_update(Ptr<NiPSysAirFieldModifier>, f32, Ptr)
        ),
        entry!(
            0x00c3_0170,
            ni_p_sys_air_field_modifier_load_binary(Ptr<NiPSysAirFieldModifier>, Ptr)
        ),
        entry!(
            0x00c3_02a0,
            ni_p_sys_air_field_modifier_save_binary(Ptr<NiPSysAirFieldModifier>, Ptr)
        ),
        entry!(
            0x00c3_03c0,
            ni_p_sys_air_field_modifier_get_viewer_strings(Ptr<NiPSysAirFieldModifier>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c2f4e0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_f4e0;
        const TARGET: u32 = 0x00c2_e520;
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
    fn thunk_00c2f4f0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_f4f0;
        const TARGET: u32 = 0x00c2_8b60;
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
    fn ni_p_sys_air_field_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_f590, &args![]).u32(), 0x0120_27a8);
    }

    /// The clone map double: key 0x5000 (this) maps to the clone, the
    /// field object 0x7100 to 0x7200; other keys are absent.
    fn process_clone_engine() -> (Engine, Ptr, Ptr, u32) {
        let mut e = Engine::new();
        e.register(MODIFIER_PROCESS_CLONE, |_, _| Ret::default());
        let clone = e.mem.alloc(0x68);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let value = match a[1] {
                0x5000 => clone,
                0x7100 => 0x7200,
                _ => return Ret::default(),
            };
            e.mem.set_u32(a[2], value);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let this = Ptr::new(e.mem.alloc(0x68));
        let process = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(process.addr(), 0x0066_0000);
        (e, this, process, clone)
    }

    #[test]
    fn process_clone_maps_the_field_object() {
        let (mut e, _, process, clone) = process_clone_engine();
        // `this` must be the mapped key 0x5000: use a fresh block there.
        e.map(0x5000, 0x100);
        let this: Ptr = Ptr::new(0x5000);
        e.mem.set_u32(this.addr() + FIELD_OBJECT, 0x7100);
        e.call_log = Some(vec![]);
        e.call(0x00c2_c110, &args![this, process]);
        assert_eq!(e.mem.u32(clone + FIELD_OBJECT), 0x7200);
        let log = e.call_log.take().unwrap();
        let lookups: Vec<_> = log.iter().filter(|c| c.0 == CLONE_MAP_LOOKUP).collect();
        assert_eq!(lookups.len(), 2);
        assert_eq!(lookups[0].1[..2], [0x0066_0000, 0x5000]);
        assert_eq!(lookups[1].1[..2], [0x0066_0000, 0x7100]);
        let base: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MODIFIER_PROCESS_CLONE)
            .collect();
        assert_eq!(base[0].1, [0x5000, process.addr()]);
    }

    #[test]
    fn process_clone_keeps_an_unmapped_field_object_and_skips_a_null_one() {
        let (mut e, _, process, clone) = process_clone_engine();
        e.map(0x5000, 0x100);
        let this: Ptr = Ptr::new(0x5000);
        e.mem.set_u32(this.addr() + FIELD_OBJECT, 0x7300);
        e.call(0x00c2_c110, &args![this, process]);
        assert_eq!(e.mem.u32(clone + FIELD_OBJECT), 0x7300);

        let (mut e, this, process, _) = process_clone_engine();
        e.call_log = Some(vec![]);
        e.call(0x00c2_c110, &args![this, process]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == CLONE_MAP_LOOKUP).count(), 1);
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to, quiet};
    use crate::units::niparticle::nipsysmodifier::test_stream;

    type M = NiPSysAirFieldModifier;

    fn set_floats(e: &mut Engine, addr: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(addr + 4 * i as u32, *value);
        }
    }

    fn floats(e: &Engine, addr: u32) -> Vec<f32> {
        (0..3).map(|i| e.mem.f32(addr + 4 * i)).collect()
    }

    fn f64_arg(a: &[u32], i: usize) -> f64 {
        f64::from_bits(u64::from(a[i]) | (u64::from(a[i + 1]) << 32))
    }

    fn normalize(e: &mut Engine, addr: u32) -> f32 {
        let v = floats(e, addr);
        let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        set_floats(e, addr, &[v[0] / length, v[1] / length, v[2] / length]);
        length
    }

    /// The doubles and globals the updates use: the product is the field
    /// object's world transform, points turn into themselves, normalizing
    /// and the CRT helpers are exact, `_ftol2` answers 5.
    fn helpers(e: &mut Engine) {
        quiet(e, &[TRANSFORM_INVERT]);
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            let words = e.mem.bytes(a[2], TRANSFORM_SIZE);
            e.mem.write(a[1], &words);
            Ret::default()
        });
        e.register(TRANSFORM_POINT, |e, a| {
            let words = e.mem.bytes(a[2], 12);
            e.mem.write(a[1], &words);
            answer(a[1])
        });
        e.register(POINT_UNITIZE, |e, a| {
            normalize(e, a[0]);
            Ret::default()
        });
        e.register(POINT_UNITIZE_GET_LENGTH, |e, a| Ret {
            st0: f64::from(normalize(e, a[0])),
            ..Ret::default()
        });
        e.register(POINT_UNEQUAL, |e, a| {
            answer(u32::from(e.mem.bytes(a[0], 12) != e.mem.bytes(a[1], 12)))
        });
        e.register(POINT_DIVIDE, |e, a| {
            let v = floats(e, a[0]);
            let d = f32::from_bits(a[2]);
            set_floats(e, a[1], &[v[0] / d, v[1] / d, v[2] / d]);
            answer(a[1])
        });
        e.register(SQRT, |_, a| Ret {
            st0: f64_arg(a, 0).sqrt(),
            ..Ret::default()
        });
        e.register(POW, |_, a| Ret {
            st0: f64_arg(a, 0).powf(f64_arg(a, 2)),
            ..Ret::default()
        });
        e.register(FTOL, |_, _| answer(5));
        e.map(ZERO_POINT, 12);
        e.map(DEFAULT_AXIS, 12);
        set_floats(e, DEFAULT_AXIS, &[1.0, 2.0, 3.0]);
        e.map(NO_POSITION, 12);
        set_floats(e, NO_POSITION, &[f32::MIN; 3]);
        e.map(DEFAULT_LAST_UPDATE, 4);
        e.mem.set_f32(DEFAULT_LAST_UPDATE, f32::MIN);
        e.map(NO_UPDATE_TIME, 8);
        e.set_global::<f64>(NO_UPDATE_TIME, f64::from(f32::MIN));
        e.map(FLOAT_ZERO, 4);
        e.map(SPREAD_FACTOR, 8);
        e.map(TABLE_SIZE, 8);
        e.set_global::<f64>(TABLE_SIZE, 512.0);
        e.map(SPREAD_TABLE, 0x800);
        e.mem.set_f32(SPREAD_TABLE + 5 * 4, 0.5);
    }

    /// A modifier with magnitude 2, unit direction x, air friction 0.5, no
    /// last position, and one particle at `position` with velocity 0, last
    /// updated at 0.
    fn scene(e: &mut Engine, position: [f32; 3]) -> (Ptr<M>, Ptr, u32) {
        let this = e.new_object::<M>();
        let field = e.mem.alloc(0x100);
        let system = e.mem.alloc(0x100);
        e.mem.set_u32(this.addr() + FIELD_OBJECT, field);
        e.mem.set_u32(this.addr() + TARGET, system);
        e.mem.set_f32(this.addr() + MAGNITUDE, 2.0);
        set_floats(e, this.addr() + 0x4c, &[1.0, 0.0, 0.0]);
        e.set(this, M::m_fAirFriction, 0.5f32.to_bits());
        set_floats(e, this.addr() + 0x30, &[f32::MIN; 3]);
        let data: Ptr = Ptr::new(e.mem.alloc(0x80));
        e.mem.set_u16(data.addr() + DATA_ACTIVE, 1);
        let vertices = e.mem.alloc(24);
        set_floats(e, vertices, &position);
        e.mem.set_u32(data.addr() + DATA_VERTICES, vertices);
        let infos = e.mem.alloc(2 * INFO_SIZE);
        e.mem.set_u32(data.addr() + DATA_INFOS, infos);
        (this, data, infos)
    }

    #[test]
    fn set_direction_keeps_a_unit_direction() {
        let mut e = Engine::new();
        helpers(&mut e);
        let this = e.new_object::<M>();
        let direction = e.mem.alloc(12);
        set_floats(&mut e, direction, &[0.0, 3.0, 4.0]);
        e.call(0x00c2_f500, &args![this, direction]);
        assert_eq!(floats(&e, this.addr() + 0x40), vec![0.0, 3.0, 4.0]);
        assert_eq!(floats(&e, this.addr() + 0x4c), vec![0.0, 0.6, 0.8]);
        // A direction that normalizes to zero takes the default axis.
        e.register(POINT_UNITIZE, |e, a| {
            set_floats(e, a[0], &[0.0; 3]);
            Ret::default()
        });
        e.call(0x00c2_f500, &args![this, direction]);
        assert_eq!(floats(&e, this.addr() + 0x4c), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn constructor_factory_and_destructor() {
        let mut e = Engine::new();
        helpers(&mut e);
        e.register(NI_NEW, |e, a| answer(e.mem.alloc(a[0])));
        quiet(
            &mut e,
            &[FIELD_MODIFIER_CONSTRUCTOR, MODIFIER_DESTRUCTOR, NI_DELETE],
        );
        e.call_log = Some(vec![]);
        let made = e.call(0x00c2_f630, &args![]).u32();
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x68]]);
        assert_eq!(calls_to(&e, FIELD_MODIFIER_CONSTRUCTOR), vec![vec![made]]);
        assert_eq!(e.mem.u32(made), VTABLE);
        assert_eq!(floats(&e, made + 0x40), vec![-1.0, -2.0, -3.0]);
        assert_eq!(floats(&e, made + 0x30), vec![f32::MIN; 3]);
        assert_eq!(e.mem.f32(made + 0x3c), f32::MIN);
        assert_eq!(e.call(0x00c2_f5a0, &args![made, 1u32]).u32(), made);
        assert_eq!(calls_to(&e, MODIFIER_DESTRUCTOR), vec![vec![made]]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![made, 0x68]]);
        e.call(0x00c2_f5a0, &args![made, 0u32]);
        assert_eq!(calls_to(&e, NI_DELETE).len(), 1);
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_f630, &args![]).u32(), 0);
    }

    #[test]
    fn limited_update_pushes_slower_particles_within_range() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [1.0, 0.0, 0.0]);
        e.mem.set_f32(this.addr() + MAX_DISTANCE_SQUARED, 4.0);
        e.call(0x00c2_f650, &args![this, 1.0f32, data]);
        // Wind (2, 0, 0); relative velocity -2 along it: + 0.5 * 2 * 1.
        assert_eq!(floats(&e, infos), vec![1.0, 0.0, 0.0]);
        assert_eq!(e.get(this, M::m_fLastUpdateTime), 1.0f32.to_bits());
        assert_eq!(floats(&e, this.addr() + 0x30), vec![0.0; 3]);
        // Faster than the wind: kept.
        set_floats(&mut e, infos, &[3.0, 0.0, 0.0]);
        e.call(0x00c2_f650, &args![this, 1.0f32, data]);
        assert_eq!(floats(&e, infos), vec![3.0, 0.0, 0.0]);
        // Out of range, or no elapsed time: kept.
        set_floats(&mut e, infos, &[0.0; 3]);
        e.mem.set_f32(this.addr() + MAX_DISTANCE_SQUARED, 0.5);
        e.call(0x00c2_f650, &args![this, 1.0f32, data]);
        assert_eq!(floats(&e, infos), vec![0.0; 3]);
        e.mem.set_f32(this.addr() + MAX_DISTANCE_SQUARED, 4.0);
        e.call(0x00c2_f650, &args![this, 0.0f32, data]);
        assert_eq!(floats(&e, infos), vec![0.0; 3]);
        // No magnitude: nothing at all.
        e.mem.set_f32(this.addr() + MAGNITUDE, 0.0);
        e.call(0x00c2_f650, &args![this, 7.0f32, data]);
        assert_eq!(e.get(this, M::m_fLastUpdateTime), 0);
    }

    #[test]
    fn unlimited_update_ignores_the_distance() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [100.0, 0.0, 0.0]);
        e.set(this, M::m_bInheritRotation, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_f8d0, &args![this, 1.0f32, data]);
        assert_eq!(calls_to(&e, TRANSFORM_POINT).len(), 1);
        assert_eq!(floats(&e, infos), vec![1.0, 0.0, 0.0]);
        set_floats(&mut e, infos, &[2.0, 0.0, 0.0]);
        e.call(0x00c2_f8d0, &args![this, 1.0f32, data]);
        assert_eq!(floats(&e, infos), vec![2.0, 0.0, 0.0]);
    }

    #[test]
    fn update_picks_the_simple_paths() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [100.0, 0.0, 0.0]);
        e.mem.set_u8(this.addr() + USE_MAX_DISTANCE, 1);
        e.mem.set_f32(this.addr() + MAX_DISTANCE_SQUARED, 4.0);
        e.call(0x00c2_fb40, &args![this, 1.0f32, data]);
        assert_eq!(floats(&e, infos), vec![0.0; 3]);
        assert_eq!(e.get(this, M::m_fLastUpdateTime), 1.0f32.to_bits());
        e.mem.set_u8(this.addr() + USE_MAX_DISTANCE, 0);
        e.call(0x00c2_fb40, &args![this, 1.0f32, data]);
        assert_eq!(floats(&e, infos), vec![1.0, 0.0, 0.0]);
        // No particles or no field object: nothing.
        set_floats(&mut e, infos, &[0.0; 3]);
        e.mem.set_u32(this.addr() + FIELD_OBJECT, 0);
        e.call(0x00c2_fb40, &args![this, 1.0f32, data]);
        assert_eq!(floats(&e, infos), vec![0.0; 3]);
    }

    #[test]
    fn update_attenuates_with_the_distance() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [1.0, 0.0, 0.0]);
        e.mem.set_f32(this.addr() + ATTENUATION, 1.0);
        e.mem.set_u8(this.addr() + USE_MAX_DISTANCE, 1);
        e.mem.set_f32(this.addr() + MAX_DISTANCE, 2.0);
        e.mem.set_f32(this.addr() + MAX_DISTANCE_SQUARED, 4.0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_fb40, &args![this, 1.0f32, data]);
        // (1 - 1/2)^1 * elapsed 1 * friction 0.5 * wind 2.
        assert_eq!(floats(&e, infos), vec![0.5, 0.0, 0.0]);
        assert_eq!(calls_to(&e, POW).len(), 1);
        assert_eq!(e.get(this, M::m_fLastUpdateTime), 1.0f32.to_bits());
        // Beyond the squared limit: kept.
        set_floats(&mut e, infos, &[0.0; 3]);
        e.mem.set_f32(this.addr() + MAX_DISTANCE_SQUARED, 0.5);
        e.call(0x00c2_fb40, &args![this, 1.0f32, data]);
        assert_eq!(floats(&e, infos), vec![0.0; 3]);
    }

    #[test]
    fn update_spreads_within_the_angle() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [1.0, 0.0, 0.0]);
        e.set(this, M::m_bEnableSpread, 1);
        e.mem.set_u16(data.addr() + DATA_ACTIVE, 2);
        let vertices = e.mem.u32(data.addr() + DATA_VERTICES);
        set_floats(&mut e, vertices + 12, &[0.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_fb40, &args![this, 1.0f32, data]);
        assert_eq!(calls_to(&e, FTOL).len(), 1);
        // The first particle lies along the wind; the second, at 90
        // degrees, is outside the cosine 0.5.
        assert_eq!(floats(&e, infos), vec![1.0, 0.0, 0.0]);
        assert_eq!(floats(&e, infos + INFO_SIZE), vec![0.0; 3]);
    }

    #[test]
    fn update_inherits_the_field_motion() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [1.0, 0.0, 0.0]);
        e.mem.set_f32(this.addr() + MAGNITUDE, 0.0);
        e.set(this, M::m_fInheritVelocity, 1.0f32.to_bits());
        // Not moved yet: no wind, nothing happens.
        e.call(0x00c2_fb40, &args![this, 1.0f32, data]);
        assert_eq!(floats(&e, infos), vec![0.0; 3]);
        assert_eq!(floats(&e, this.addr() + 0x30), vec![0.0; 3]);
        // Moved 1 along x in 0.5: wind (2, 0, 0).
        set_floats(&mut e, this.addr() + 0x30, &[-1.0, 0.0, 0.0]);
        e.set(this, M::m_fLastUpdateTime, 0.5f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x00c2_fb40, &args![this, 1.0f32, data]);
        let divided = calls_to(&e, POINT_DIVIDE);
        assert_eq!(divided.len(), 1);
        assert_eq!(divided[0][2], 0.5f32.to_bits());
        assert_eq!(floats(&e, infos), vec![1.0, 0.0, 0.0]);
        assert_eq!(e.get(this, M::m_fLastUpdateTime), 1.0f32.to_bits());
    }

    #[test]
    fn create_clone_and_is_equal() {
        let mut e = Engine::new();
        helpers(&mut e);
        e.register(NI_NEW, |e, a| answer(e.mem.alloc(a[0])));
        quiet(
            &mut e,
            &[FIELD_MODIFIER_CONSTRUCTOR, FIELD_MODIFIER_COPY_MEMBERS],
        );
        e.register(FIELD_MODIFIER_IS_EQUAL, |_, _| answer(1));
        let this = e.new_object::<M>();
        set_floats(&mut e, this.addr() + 0x40, &[0.0, 0.0, 2.0]);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_fad0, &args![this, 0x900u32]).u32();
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x900]]
        );
        assert_eq!(floats(&e, clone + 0x40), vec![0.0, 0.0, 2.0]);
        assert_eq!(floats(&e, clone + 0x4c), vec![0.0, 0.0, 1.0]);
        let clone: Ptr = Ptr::new(clone);
        assert!(e.call(0x00c2_fb10, &args![this, clone]).bool());
        e.mem.set_f32(clone.addr() + 0x40, 1.0);
        assert!(!e.call(0x00c2_fb10, &args![this, clone]).bool());
        e.register(FIELD_MODIFIER_IS_EQUAL, |_, _| answer(0));
        e.mem.set_f32(clone.addr() + 0x40, 0.0);
        assert!(!e.call(0x00c2_fb10, &args![this, clone]).bool());
    }

    #[test]
    fn load_and_save_binary_carry_the_settings() {
        let mut e = Engine::new();
        helpers(&mut e);
        quiet(
            &mut e,
            &[
                FIELD_MODIFIER_LOAD_BINARY,
                FIELD_MODIFIER_SAVE_BINARY,
                POINT_SAVE_BINARY,
            ],
        );
        e.register(POINT_LOAD_BINARY, |e, a| {
            set_floats(e, a[0], &[0.0, 0.0, 3.0]);
            Ret::default()
        });
        let mut bytes = 0.25f32.to_le_bytes().to_vec();
        bytes.extend(0.75f32.to_le_bytes());
        bytes.extend([2, 0, 1]);
        bytes.extend(9.0f32.to_le_bytes());
        let stream = test_stream::new(&mut e, 0x1402_0007, &bytes);
        let this = e.new_object::<M>();
        e.call(0x00c3_0170, &args![this, stream]);
        assert_eq!(floats(&e, this.addr() + 0x4c), vec![0.0, 0.0, 1.0]);
        assert_eq!(e.get(this, M::m_fAirFriction), 0.25f32.to_bits());
        assert_eq!(e.get(this, M::m_fInheritVelocity), 0.75f32.to_bits());
        assert_eq!(e.get(this, M::m_bInheritRotation), 1);
        assert_eq!(e.get(this, M::m_bComponentOnly), 0);
        assert_eq!(e.get(this, M::m_bEnableSpread), 1);
        assert_eq!(e.get(this, M::m_fSpread), 9.0f32.to_bits());
        assert_eq!(test_stream::position(&e, stream), 15);
        let out = viewer_test::out_stream(&mut e);
        e.call(0x00c3_02a0, &args![this, out]);
        let mut expected = 0.25f32.to_le_bytes().to_vec();
        expected.extend(0.75f32.to_le_bytes());
        expected.extend([1, 0, 1]);
        expected.extend(9.0f32.to_le_bytes());
        assert_eq!(viewer_test::written(&e, out), (expected, 15));
    }

    #[test]
    fn viewer_strings_name_every_setting() {
        let mut e = Engine::new();
        quiet(&mut e, &[FIELD_MODIFIER_GET_VIEWER_STRINGS]);
        viewer_test::counting(
            &mut e,
            &[
                VIEWER_STRING,
                POINT_GET_VIEWER_STRING,
                VIEWER_STRING_FLOAT,
                VIEWER_STRING_BOOL,
            ],
        );
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_0140);
        let this = e.new_object::<M>();
        e.set(this, M::m_bComponentOnly, 1);
        e.set(this, M::m_fSpread, 0.5f32.to_bits());
        let strings = viewer_test::strings(&mut e, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c3_03c0, &args![this, strings]);
        assert_eq!(
            viewer_test::collected(&e, strings),
            (1..=9).collect::<Vec<u32>>()
        );
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![
                vec![this.addr() + 0x40, LABEL_DIRECTION],
                vec![this.addr() + 0x4c, LABEL_UNIT_DIRECTION]
            ]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_BOOL),
            vec![
                vec![LABEL_INHERIT_ROTATION, 0],
                vec![LABEL_COMPONENT_ONLY, 1],
                vec![LABEL_ENABLE_SPREAD, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FLOAT),
            vec![
                vec![LABEL_AIR_FRICTION, 0],
                vec![LABEL_INHERIT_VELOCITY, 0],
                vec![LABEL_SPREAD, 0.5f32.to_bits()]
            ]
        );
    }
}
