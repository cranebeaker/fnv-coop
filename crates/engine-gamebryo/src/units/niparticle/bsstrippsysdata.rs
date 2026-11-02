//! `NiParticle/bsstrippsysdata.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a7b7d0`: the jump target of `00a817a0`.
const JUMP_TARGET_00A817A0: u32 = 0x00a7_b7d0;

/// `00a75f00`: the jump target of `00a817b0`.
const JUMP_TARGET_00A817B0: u32 = 0x00a7_5f00;

// Translated from 00a817a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::LinkObject` (Xbox PDB): a 5-byte jump to `00a7b7d0` (engine map: `NiPSysData::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSStripPSysData@@UAAXAAVNiStream@@@Z`.
pub fn bs_strip_p_sys_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A817A0, &args![this, stream]);
}

// Translated from 00a817b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a75f00` (engine map: `NiPSysData::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@BSStripPSysData@@UAA_NAAVNiStream@@@Z`.
pub fn bs_strip_p_sys_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A817B0, &args![this, stream]).bool()
}

// Translated from 00c25ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2680.
pub fn b_s_strip_p_sys_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2680
}

layout! {
    /// `BSStripPSysData` (Xbox PDB, 0x90 bytes; the PC code uses the same offsets).
    pub struct BSStripPSysData: 0x90 {
        /// `pStripList` (Xbox PDB): one 0x20-byte `StripData` per particle.
        0x7c pStripList: u32,
        /// `usMaxPointCount` (Xbox PDB).
        0x80 usMaxPointCount: u16,
        /// `fStartCapSize` (Xbox PDB).
        0x84 fStartCapSize: u32,
        /// `fEndCapSize` (Xbox PDB).
        0x88 fEndCapSize: u32,
        /// `bDoZPrePass` (Xbox PDB).
        0x8c bDoZPrePass: u8,
    }
}

/// `BSStripPSysData::StripData` (Xbox PDB, 0x20 bytes): `pPointList` +0, `usPointCount` +4,
/// `usHighestPointCount` +6, `iStartIndex` +8, `iStopIndex` +0xc, `usMaxPointCount` +0x10 (a
/// word), `iFirstIndex` +0x14, `iLastIndex` +0x18, `fLastTime` +0x1c.
const STRIP_POINTS: u32 = 0;
const STRIP_POINT_COUNT: u32 = 4;
const STRIP_HIGHEST_POINT_COUNT: u32 = 6;
const STRIP_START_INDEX: u32 = 8;
const STRIP_STOP_INDEX: u32 = 0xc;
const STRIP_MAX_POINT_COUNT: u32 = 0x10;
const STRIP_FIRST_INDEX: u32 = 0x14;
const STRIP_LAST_INDEX: u32 = 0x18;
const STRIP_LAST_TIME: u32 = 0x1c;
/// `BSStripPSysData::PointData` (Xbox PDB, 0x84 bytes): `kPoint` +0, `kVertex` +0xc, `kDir`
/// +0x3c, `kNormal` +0x48, `kColor` +0x6c, `fSize` +0x7c, `fRadii` +0x80 (the words at +0x54
/// lie inside `kNormal`).
const POINT_SIZE: u32 = 0x84;
/// The `float` a strip's last time starts at (`01015f5c`) and the colour answered without
/// particle colours (`011a9be0`).
const INITIAL_TIME_01015F5C: u32 = 0x0101_5f5c;
const DEFAULT_COLOR_011A9BE0: u32 = 0x011a_9be0;
/// `NiPSysData`'s functions it builds on (`thiscall`): `IsEqual` (`00c24de0`, one word),
/// `RemoveParticle` (`00c24cc0`, one word), the constructor (`00c24c90`), the destructor
/// (`00c24ef0`), `CopyMembers` (`00c24f20`, two words), `LoadBinary` (`00c25340`),
/// `SaveBinary` (`00c25560`) and `GetViewerStrings` (`00c255b0`, one word each).
const PSYS_DATA_IS_EQUAL_00C24DE0: u32 = 0x00c2_4de0;
const PSYS_DATA_REMOVE_PARTICLE_00C24CC0: u32 = 0x00c2_4cc0;
const PSYS_DATA_CONSTRUCT_00C24C90: u32 = 0x00c2_4c90;
const PSYS_DATA_DESTRUCT_00C24EF0: u32 = 0x00c2_4ef0;
const PSYS_DATA_COPY_MEMBERS_00C24F20: u32 = 0x00c2_4f20;
const PSYS_DATA_LOAD_BINARY_00C25340: u32 = 0x00c2_5340;
const PSYS_DATA_SAVE_BINARY_00C25560: u32 = 0x00c2_5560;
const PSYS_DATA_GET_VIEWER_STRINGS_00C255B0: u32 = 0x00c2_55b0;
/// The strip geometry step (`00a62be0`, `cdecl`, seven words: the address of three point
/// words from the point before, the point, a position or 0, the size, and the addresses of
/// the point's `kVertex`, `kNormal` and `kDir`).
const STRIP_GEOMETRY_00A62BE0: u32 = 0x00a6_2be0;
/// The Ni allocation (`00aa13e0`, `cdecl`: size), the plain allocation (`00aa1070`, `cdecl`:
/// size), the plain free (`00aa10f0`, `cdecl`: block), the sized free (`00aa1460`, `cdecl`:
/// block, size) and `memcpy` (`00ec44d0`, `cdecl`).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const NI_FREE: u32 = 0x00aa_1460;
const MEMCPY: u32 = 0x00ec_44d0;
/// `BSStripPSysData`'s vtable, size and RTTI record (its first word the class name).
const BS_STRIP_P_SYS_DATA_VTABLE: u32 = 0x010b_e514;
const BS_STRIP_P_SYS_DATA_SIZE: u32 = 0x90;
const BS_STRIP_P_SYS_DATA_RTTI: u32 = 0x0120_2680;
/// The stream's input and output binary streams (+0x24c, +0x250) with their read (word +8)
/// and write (word +0xc) functions (`cdecl`: the object, the buffer, the count, the address of
/// the component size and the component count); the bytes moved are added to the object's
/// word +4.
const STREAM_INPUT_BINARY_STREAM: u32 = 0x24c;
const STREAM_OUTPUT_BINARY_STREAM: u32 = 0x250;
/// The viewer string makers (`cdecl`): `NiGetViewerString` (`00a7d330`, one word), of a
/// 16-bit word (`00a7d490`), of a `float` (`00a7d550`) and of a byte (`00a7d370`), each with
/// a name; the string array's grow (`0096ad30`) and set-at (`0096ae90`); the names.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_SHORT_00A7D490: u32 = 0x00a7_d490;
const VIEWER_STRING_FLOAT_00A7D550: u32 = 0x00a7_d550;
const VIEWER_STRING_BOOL_00A7D370: u32 = 0x00a7_d370;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
const MAX_POINT_COUNT_NAME: u32 = 0x010b_e5ec;
const START_CAP_SIZE_NAME: u32 = 0x010b_e5dc;
const END_CAP_SIZE_NAME: u32 = 0x010b_e5d0;
const DO_Z_PRE_PASS_NAME: u32 = 0x010b_e5c4;

/// The address of strip `index` (`pStripList + 0x20 * index`).
fn strip(e: &Engine, this: u32, index: u32) -> u32 {
    e.mem.u32(this + 0x7c).wrapping_add((index & 0xffff) << 5)
}

/// The address of point `point` of strip `index`.
fn strip_point(e: &Engine, this: u32, index: u32, point: u32) -> u32 {
    let points = e.mem.u32(strip(e, this, index) + STRIP_POINTS);
    points.wrapping_add((point & 0xffff).wrapping_mul(POINT_SIZE))
}

/// `(value) mod modulus`, made non-negative (`IDIV`, then `+ modulus` when the remainder is
/// negative).
fn wrap_index(value: i32, modulus: i32) -> i32 {
    let rest = value.wrapping_rem(modulus);
    if rest < 0 {
        rest.wrapping_add(modulus)
    } else {
        rest
    }
}

/// Resets strip `index`: no points, `usMaxPointCount`, start and stop index from the data's
/// `usMaxPointCount`, first index -1, last index 0, last time the `float` at `01015f5c`.
fn reset_strip(e: &mut Engine, this: u32, index: u32) {
    let start = x87_quiet_bits(e.global::<u32>(INITIAL_TIME_01015F5C));
    let offset = (index & 0xffff) << 5;
    let at = |e: &Engine| e.mem.u32(this + 0x7c).wrapping_add(offset);
    let address = at(e);
    e.mem.set_u16(address + STRIP_POINT_COUNT, 0);
    let address = at(e);
    e.mem.set_u16(address + STRIP_HIGHEST_POINT_COUNT, 0);
    let maximum = u32::from(e.mem.u16(this + 0x80));
    let address = at(e);
    e.mem.set_u32(address + STRIP_MAX_POINT_COUNT, maximum);
    let address = at(e);
    e.mem.set_u32(address + STRIP_LAST_TIME, start);
    let address = at(e);
    e.mem.set_u32(address + STRIP_FIRST_INDEX, u32::MAX);
    let address = at(e);
    e.mem.set_u32(address + STRIP_LAST_INDEX, 0);
    let maximum = u32::from(e.mem.u16(this + 0x80));
    let address = at(e);
    e.mem.set_u32(address + STRIP_STOP_INDEX, maximum);
    let maximum = u32::from(e.mem.u16(this + 0x80));
    let address = at(e);
    e.mem.set_u32(address + STRIP_START_INDEX, maximum);
}

// Translated from 00c25cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::IsEqual` (Xbox PDB, `thiscall`, one word: the other data):
/// `NiPSysData::IsEqual`, the same `usMaxPointCount`, `fStartCapSize` and `fEndCapSize`
/// (`FUCOMPP`, unordered counts as different) and `bDoZPrePass`.
pub fn bs_strip_p_sys_data_is_equal(
    e: &mut Engine,
    this: Ptr<BSStripPSysData>,
    other: Ptr<BSStripPSysData>,
) -> bool {
    if !e
        .call(PSYS_DATA_IS_EQUAL_00C24DE0, &args![this, other])
        .bool()
    {
        return false;
    }
    if e.get(other, BSStripPSysData::usMaxPointCount)
        != e.get(this, BSStripPSysData::usMaxPointCount)
    {
        return false;
    }
    for field in [BSStripPSysData::fStartCapSize, BSStripPSysData::fEndCapSize] {
        let theirs = x87_load32(e.get(other, field));
        let mine = x87_load32(e.get(this, field));
        if mine != theirs {
            return false;
        }
    }
    e.get(other, BSStripPSysData::bDoZPrePass) == e.get(this, BSStripPSysData::bDoZPrePass)
}

// Translated from 00c25d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map's own column (its call-graph name is
/// `BSStripPSysData::InitializePoint`; `thiscall`, one word, its low 16 bits the particle
/// index): resets the particle's strip (no points, the data's `usMaxPointCount` as its maximum
/// and as start and stop index, first index -1, last index 0, last time the `float` at
/// `01015f5c`).
pub fn fn_00c25d40(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32) {
    reset_strip(e, this.addr(), index);
}

// Translated from 00c25db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the particle index, 16 bits): the strip's
/// `usPointCount`.
pub fn fn_00c25db0(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32) -> u16 {
    let at = strip(e, this.addr(), index);
    e.mem.u16(at + STRIP_POINT_COUNT)
}

// Translated from 00c25dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): `usMaxPointCount`.
pub fn fn_00c25dd0(e: &mut Engine, this: Ptr<BSStripPSysData>) -> u16 {
    e.get(this, BSStripPSysData::usMaxPointCount)
}

// Translated from 00c25de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the particle index, 16 bits): the strip's
/// `fLastTime` (in ST0).
pub fn fn_00c25de0(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32) -> f64 {
    let at = strip(e, this.addr(), index);
    x87_load32(e.mem.u32(at + STRIP_LAST_TIME))
}

// Translated from 00c25e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the particle and the point index, 16
/// bits each): the address of the strip's point.
pub fn fn_00c25e00(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32, point: u32) -> u32 {
    strip_point(e, this.addr(), index, point)
}

// Translated from 00c25e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the particle and the point index): the
/// point's `fSize` (in ST0).
pub fn fn_00c25e20(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32, point: u32) -> f64 {
    let at = strip_point(e, this.addr(), index, point);
    x87_load32(e.mem.u32(at.wrapping_add(0x7c)))
}

// Translated from 00c25e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the particle and the point index): the
/// point's `fRadii` (in ST0).
pub fn fn_00c25e40(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32, point: u32) -> f64 {
    let at = strip_point(e, this.addr(), index, point);
    x87_load32(e.mem.u32(at.wrapping_add(0x80)))
}

// Translated from 00c25e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the particle and the point index): the
/// address of the point's `kVertex` (+0xc).
pub fn fn_00c25e70(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32, point: u32) -> u32 {
    strip_point(e, this.addr(), index, point).wrapping_add(0xc)
}

// Translated from 00c25e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the particle and the point index): the
/// address of the point's `kNormal` (+0x48).
pub fn fn_00c25e90(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32, point: u32) -> u32 {
    strip_point(e, this.addr(), index, point).wrapping_add(0x48)
}

// Translated from 00c25eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the particle index, 16 bits): `1 /
/// (usHighestPointCount - 1)` of the strip (`FILD`, `FLD1`, `FSUB ST1,ST0`, `FDIVRP`), stored
/// as a `float` and answered in ST0.
pub fn fn_00c25eb0(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32) -> f64 {
    let at = strip(e, this.addr(), index);
    let highest = f64::from(e.mem.u16(at + STRIP_HIGHEST_POINT_COUNT));
    let span = x87_sub(highest, 1.0);
    let step = x87_store32(x87_div(1.0, span));
    x87_load32(step)
}

// Translated from 00c25ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData`'s destructor (`thiscall`): the vtable `010be514`, each of the
/// `m_usVertices` strips' point list freed (`00aa10f0`) and cleared, the strip list freed,
/// then the `NiPSysData` destructor (`00c24ef0`, a jump).
pub fn fn_00c25ef0(e: &mut Engine, this: Ptr<BSStripPSysData>) {
    let base = this.addr();
    let count = u32::from(e.mem.u16(base + 8));
    e.mem.set_u32(base, BS_STRIP_P_SYS_DATA_VTABLE);
    let mut offset = 0u32;
    for _ in 0..count {
        let points = e.mem.u32(e.mem.u32(base + 0x7c).wrapping_add(offset));
        e.call(FREE_00AA10F0, &args![points]);
        let strips = e.mem.u32(base + 0x7c);
        e.mem.set_u32(strips.wrapping_add(offset), 0);
        offset = offset.wrapping_add(0x20);
    }
    let strips = e.mem.u32(base + 0x7c);
    e.call(FREE_00AA10F0, &args![strips]);
    e.call(PSYS_DATA_DESTRUCT_00C24EF0, &args![this]);
}

// Translated from 00c25f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::RemoveParticle` (vtable slot +0xa4, Xbox PDB; `thiscall`, one word: the
/// particle index, 16 bits). A strip with more than one point loses its first point (first
/// index one on, modulo the strip's maximum; point count and stop index one down). Otherwise
/// `NiPSysData::RemoveParticle` runs (the argument's stack word first set to `m_usActive - 1`,
/// the last particle) and, unless the particle was the last one, its strip and the last
/// particle's strip swap places.
pub fn bs_strip_p_sys_data_remove_particle(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32) {
    let base = this.addr();
    let offset = (index & 0xffff) << 5;
    let at = e.mem.u32(base + 0x7c).wrapping_add(offset);
    if e.mem.u16(at + STRIP_POINT_COUNT) > 1 {
        let first = e.mem.i32(at + STRIP_FIRST_INDEX).wrapping_add(1);
        let maximum = e.mem.i32(at + STRIP_MAX_POINT_COUNT);
        e.mem
            .set_u32(at + STRIP_FIRST_INDEX, wrap_index(first, maximum) as u32);
        let at = e.mem.u32(base + 0x7c).wrapping_add(offset);
        let count = e.mem.u16(at + STRIP_POINT_COUNT).wrapping_add(0xffff);
        e.mem.set_u16(at + STRIP_POINT_COUNT, count);
        let at = e.mem.u32(base + 0x7c).wrapping_add(offset);
        let stop = e.mem.u32(at + STRIP_STOP_INDEX).wrapping_sub(1);
        e.mem.set_u32(at + STRIP_STOP_INDEX, stop);
        return;
    }
    let last = u32::from(e.mem.u16(base + 0x50).wrapping_sub(1));
    e.call(PSYS_DATA_REMOVE_PARTICLE_00C24CC0, &args![this, index]);
    if index & 0xffff == last {
        return;
    }
    let strips = e.mem.u32(base + 0x7c);
    let own = strips.wrapping_add(offset);
    let other = strips.wrapping_add(last << 5);
    let saved: Vec<u32> = (0..8).map(|word| e.mem.u32(own + 4 * word)).collect();
    for word in 0..8 {
        let value = e.mem.u32(other + 4 * word);
        e.mem.set_u32(own + 4 * word, value);
    }
    let other = e.mem.u32(base + 0x7c).wrapping_add(last << 5);
    for (word, value) in saved.into_iter().enumerate() {
        e.mem.set_u32(other + 4 * word as u32, value);
    }
}

// Translated from 00c26000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::CopyMembers` (`thiscall`, two words: the clone and the cloning process):
/// `NiPSysData::CopyMembers`, the four strip settings copied (the floats with `FLD`/`FSTP`),
/// then a new strip list (`00aa1070`, `m_usVertices` strips) whose strips copy this data's
/// counts, indexes and last time, each with a new point list of `usMaxPointCount` points
/// holding a copy (`memcpy`) of the `usPointCount` points.
pub fn bs_strip_p_sys_data_copy_members(
    e: &mut Engine,
    this: Ptr<BSStripPSysData>,
    clone: Ptr<BSStripPSysData>,
    cloning: u32,
) {
    let (base, target) = (this.addr(), clone.addr());
    e.call(
        PSYS_DATA_COPY_MEMBERS_00C24F20,
        &args![this, clone, cloning],
    );
    let maximum = e.mem.u16(base + 0x80);
    e.mem.set_u16(target + 0x80, maximum);
    for offset in [0x84u32, 0x88] {
        let value = x87_quiet_bits(e.mem.u32(base + offset));
        e.mem.set_u32(target + offset, value);
    }
    let pre_pass = e.mem.u8(base + 0x8c);
    e.mem.set_u8(target + 0x8c, pre_pass);
    let count = u32::from(e.mem.u16(base + 8));
    let strips = e.call(ALLOCATE_00AA1070, &args![count << 5]).u32();
    let mut offset = 0u32;
    for _ in 0..count {
        let new = strips.wrapping_add(offset);
        let old = |e: &Engine| e.mem.u32(base + 0x7c).wrapping_add(offset);
        let time = x87_quiet_bits(e.mem.u32(old(e) + STRIP_LAST_TIME));
        e.mem.set_u32(new + STRIP_LAST_TIME, time);
        let value = e.mem.u16(old(e) + STRIP_POINT_COUNT);
        e.mem.set_u16(new + STRIP_POINT_COUNT, value);
        let value = e.mem.u16(old(e) + STRIP_HIGHEST_POINT_COUNT);
        e.mem.set_u16(new + STRIP_HIGHEST_POINT_COUNT, value);
        for field in [
            STRIP_STOP_INDEX,
            STRIP_START_INDEX,
            STRIP_MAX_POINT_COUNT,
            STRIP_FIRST_INDEX,
        ] {
            let value = e.mem.u32(old(e) + field);
            e.mem.set_u32(new + field, value);
        }
        let points = e
            .mem
            .u32(new + STRIP_MAX_POINT_COUNT)
            .wrapping_mul(POINT_SIZE);
        let last = e.mem.u32(old(e) + STRIP_LAST_INDEX);
        e.mem.set_u32(new + STRIP_LAST_INDEX, last);
        let list = e.call(ALLOCATE_00AA1070, &args![points]).u32();
        e.mem.set_u32(new + STRIP_POINTS, list);
        let source = old(e);
        let bytes = u32::from(e.mem.u16(source + STRIP_POINT_COUNT)).wrapping_mul(POINT_SIZE);
        let from = e.mem.u32(source + STRIP_POINTS);
        e.call(MEMCPY, &args![list, from, bytes]);
        offset = offset.wrapping_add(0x20);
    }
    e.mem.set_u32(target + 0x7c, strips);
}

/// Calls the strip geometry step (`00a62be0`) for `point`, with the size `fRadii * fSize`
/// (`FLD`, `FMUL`, stored as a `float`), the point's `kVertex`, `kNormal` and `kDir`, and the
/// three words at `previous` (the addresses from the point before, or zeros).
fn strip_geometry(e: &mut Engine, previous: u32, point: u32, position: u32) {
    let radius = x87_load32(e.mem.u32(point.wrapping_add(0x80)));
    let size = x87_store32(x87_mul(
        radius,
        x87_m32(e.mem.u32(point.wrapping_add(0x7c))),
    ));
    e.call(
        STRIP_GEOMETRY_00A62BE0,
        &args![
            previous,
            point,
            position,
            size,
            point.wrapping_add(0xc),
            point.wrapping_add(0x48),
            point.wrapping_add(0x3c)
        ],
    );
}

/// The three words at `previous`: the addresses of `kDir`, `kNormal` and the words at +0x54
/// of the point before `point` in the strip (`(point - 1) mod maximum`).
fn previous_point(e: &mut Engine, strip_at: u32, point: i32, previous: u32) {
    let maximum = e.mem.i32(strip_at + STRIP_MAX_POINT_COUNT);
    let before = wrap_index(point.wrapping_sub(1), maximum) as u32;
    let offset = before.wrapping_mul(POINT_SIZE);
    for (slot, field) in [0x3cu32, 0x48, 0x54].into_iter().enumerate() {
        let points = e.mem.u32(strip_at + STRIP_POINTS);
        e.mem.set_u32(
            previous + 4 * slot as u32,
            points.wrapping_add(offset).wrapping_add(field),
        );
    }
}

// Translated from 00c26120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the particle index, 16 bits, and one not
/// read): adds the particle's current state as a point of its strip, unless its
/// `NiParticleInfo` (`m_pkParticleInfo` +0x70, 0x1c bytes each) has a lifespan (+0x10) below
/// its age (+0xc). With points, the last point's geometry is redone first (`00a62be0` with the
/// point's position, the point before it when there are two or more); without, the first
/// index becomes 0. The point at the last index takes the particle's position
/// (`m_pkVertex`), size (`m_pfSizes`), radius (`m_pfRadii`) and, when the data has colours,
/// colour; its geometry is made (position 0). Then the count grows (with the highest count;
/// the start index one down) while below `usMaxPointCount`, else the first index moves on;
/// the last index moves on (both modulo the strip's maximum).
pub fn fn_00c26120(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32, _unused_1: u32) {
    let base = this.addr();
    let particle = index & 0xffff;
    let info = e
        .mem
        .u32(base + 0x70)
        .wrapping_add(particle.wrapping_mul(0x1c));
    let age = x87_load32(e.mem.u32(info + 0xc));
    let lifespan = x87_load32(e.mem.u32(info + 0x10));
    if lifespan < age {
        return;
    }
    e.with_stack(0xc, |e, previous| {
        let previous = previous.addr();
        let strip_at = e.mem.u32(base + 0x7c).wrapping_add(particle << 5);
        for slot in 0..3 {
            e.mem.set_u32(previous + 4 * slot, 0);
        }
        let last_index = e.mem.i32(strip_at + STRIP_LAST_INDEX);
        let maximum = e.mem.i32(strip_at + STRIP_MAX_POINT_COUNT);
        let last = wrap_index(last_index.wrapping_sub(1), maximum);
        let count = e.mem.u16(strip_at + STRIP_POINT_COUNT);
        if count == 0 {
            e.mem.set_u32(strip_at + STRIP_FIRST_INDEX, 0);
        } else {
            if count > 1 {
                previous_point(e, strip_at, last, previous);
            }
            let point = e
                .mem
                .u32(strip_at + STRIP_POINTS)
                .wrapping_add((last as u32).wrapping_mul(POINT_SIZE));
            let vertices = e.mem.u32(base + 0x20);
            let position = vertices.wrapping_add(particle.wrapping_mul(0xc));
            strip_geometry(e, previous, point, position);
        }
        let last_index = e.mem.i32(strip_at + STRIP_LAST_INDEX);
        if e.mem.u16(strip_at + STRIP_POINT_COUNT) == 0 {
            for slot in 0..3 {
                e.mem.set_u32(previous + 4 * slot, 0);
            }
        } else {
            previous_point(e, strip_at, last_index, previous);
        }
        let offset = (last_index as u32).wrapping_mul(POINT_SIZE);
        let vertices = e.mem.u32(base + 0x20);
        let position = vertices.wrapping_add(particle.wrapping_mul(0xc));
        let point = e.mem.u32(strip_at + STRIP_POINTS).wrapping_add(offset);
        for word in 0..3 {
            let value = e.mem.u32(position + 4 * word);
            e.mem.set_u32(point + 4 * word, value);
        }
        let sizes = e.mem.u32(base + 0x54);
        let size = x87_quiet_bits(e.mem.u32(sizes.wrapping_add(particle * 4)));
        let point = e.mem.u32(strip_at + STRIP_POINTS).wrapping_add(offset);
        e.mem.set_u32(point + 0x7c, size);
        let radii = e.mem.u32(base + 0x4c);
        let radius = x87_quiet_bits(e.mem.u32(radii.wrapping_add(particle * 4)));
        let point = e.mem.u32(strip_at + STRIP_POINTS).wrapping_add(offset);
        e.mem.set_u32(point + 0x80, radius);
        let colors = e.mem.u32(base + 0x28);
        if colors != 0 {
            let color = colors.wrapping_add(particle << 4);
            let point = e.mem.u32(strip_at + STRIP_POINTS).wrapping_add(offset);
            for word in 0..4 {
                let value = e.mem.u32(color + 4 * word);
                e.mem.set_u32(point + 0x6c + 4 * word, value);
            }
        }
        let point = e.mem.u32(strip_at + STRIP_POINTS).wrapping_add(offset);
        strip_geometry(e, previous, point, 0);
        let count = e.mem.u16(strip_at + STRIP_POINT_COUNT);
        if count < e.mem.u16(base + 0x80) {
            let highest = e
                .mem
                .u16(strip_at + STRIP_HIGHEST_POINT_COUNT)
                .wrapping_add(1);
            e.mem.set_u16(strip_at + STRIP_HIGHEST_POINT_COUNT, highest);
            let start = e.mem.u32(strip_at + STRIP_START_INDEX).wrapping_sub(1);
            e.mem.set_u32(strip_at + STRIP_START_INDEX, start);
            e.mem
                .set_u16(strip_at + STRIP_POINT_COUNT, count.wrapping_add(1));
        } else {
            let first = e.mem.i32(strip_at + STRIP_FIRST_INDEX).wrapping_add(1);
            let maximum = e.mem.i32(strip_at + STRIP_MAX_POINT_COUNT);
            e.mem.set_u32(
                strip_at + STRIP_FIRST_INDEX,
                wrap_index(first, maximum) as u32,
            );
        }
        let next = e.mem.i32(strip_at + STRIP_LAST_INDEX).wrapping_add(1);
        let maximum = e.mem.i32(strip_at + STRIP_MAX_POINT_COUNT);
        e.mem.set_u32(
            strip_at + STRIP_LAST_INDEX,
            wrap_index(next, maximum) as u32,
        );
    });
}

// Translated from 00c26350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the particle and the point index): the
/// address of the point's `kColor` (+0x6c) when the data has colours (`m_pkColor`, +0x28),
/// else the default colour at `011a9be0`.
pub fn fn_00c26350(e: &mut Engine, this: Ptr<BSStripPSysData>, index: u32, point: u32) -> u32 {
    if e.mem.u32(this.addr() + 0x28) == 0 {
        return DEFAULT_COLOR_011A9BE0;
    }
    strip_point(e, this.addr(), index, point).wrapping_add(0x6c)
}

// Translated from 00c26380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData`'s scalar deleting destructor (vtable slot 0, `thiscall`, one word: the
/// flags): [`fn_00c25ef0`], then with bit 0 the block freed (`00aa1460`, 0x90 bytes).
/// Answers `this`.
pub fn bs_strip_p_sys_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSStripPSysData>,
    flags: u32,
) -> Ptr<BSStripPSysData> {
    fn_00c25ef0(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE, &args![this, BS_STRIP_P_SYS_DATA_SIZE]);
    }
    this
}

// Translated from 00c263b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::CreateClone` (vtable slot +0x48, `thiscall`, one word: the cloning
/// process): 0x90 bytes from `00aa13e0`; when that succeeds, the `NiPSysData` constructor,
/// the vtable `010be514` and a null strip list; then
/// [`bs_strip_p_sys_data_copy_members`] into it (0 when the allocation failed, as the exe
/// does). Answers the clone.
pub fn bs_strip_p_sys_data_create_clone(
    e: &mut Engine,
    this: Ptr<BSStripPSysData>,
    cloning: u32,
) -> u32 {
    let clone = e.call(NI_ALLOCATE, &args![BS_STRIP_P_SYS_DATA_SIZE]).u32();
    if clone != 0 {
        e.call(PSYS_DATA_CONSTRUCT_00C24C90, &args![clone]);
        e.mem.set_u32(clone, BS_STRIP_P_SYS_DATA_VTABLE);
        e.mem.set_u32(clone + 0x7c, 0);
    }
    bs_strip_p_sys_data_copy_members(e, this, Ptr::new(clone), cloning);
    clone
}

/// Moves `count` bytes between `field` and the stream's binary stream at `slot_offset`
/// (`+0x24c` input with the function at +8, `+0x250` output with the function at +0xc); the
/// component size goes through `size_slot` (the argument's stack word).
fn stream_bytes(
    e: &mut Engine,
    stream: u32,
    which: (u32, u32),
    field: u32,
    count: u32,
    size_slot: u32,
) {
    let binary = e.mem.u32(stream + which.0);
    let function = e.mem.u32(binary + which.1);
    e.mem.set_u32(size_slot, count);
    let moved = e
        .call(function, &args![binary, field, count, size_slot, 1u32])
        .u32();
    let total = e.mem.u32(binary + 4).wrapping_add(moved);
    e.mem.set_u32(binary + 4, total);
}

// Translated from 00c26410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::LoadBinary` (vtable slot +0x4c, `thiscall`, one word: the stream):
/// `NiPSysData::LoadBinary`, then `usMaxPointCount` (2 bytes), `fStartCapSize`,
/// `fEndCapSize` (4 bytes each) and `bDoZPrePass` (1 byte) read from the stream's input
/// binary stream (the component size kept in the argument's stack word), then a new strip
/// list (`00aa1070`, `m_usVertices` strips), each strip with a new point list of
/// `usMaxPointCount` points and reset as [`fn_00c25d40`] does.
pub fn bs_strip_p_sys_data_load_binary(e: &mut Engine, this: Ptr<BSStripPSysData>, stream: Ptr) {
    let base = this.addr();
    e.call(PSYS_DATA_LOAD_BINARY_00C25340, &args![this, stream]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let input = (STREAM_INPUT_BINARY_STREAM, 8);
        stream_bytes(e, stream.addr(), input, base + 0x80, 2, slot);
        stream_bytes(e, stream.addr(), input, base + 0x84, 4, slot);
        stream_bytes(e, stream.addr(), input, base + 0x88, 4, slot);
        stream_bytes(e, stream.addr(), input, base + 0x8c, 1, slot);
    });
    let count = u32::from(e.mem.u16(base + 8));
    let strips = e.call(ALLOCATE_00AA1070, &args![count << 5]).u32();
    e.mem.set_u32(base + 0x7c, strips);
    for index in 0..count {
        let points = u32::from(e.mem.u16(base + 0x80)).wrapping_mul(POINT_SIZE);
        let list = e.call(ALLOCATE_00AA1070, &args![points]).u32();
        let strips = e.mem.u32(base + 0x7c);
        e.mem.set_u32(strips.wrapping_add(index << 5), list);
        reset_strip(e, base, index);
    }
}

// Translated from 00c26570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::SaveBinary` (vtable slot +0x58, `thiscall`, one word: the stream):
/// `NiPSysData::SaveBinary`, then `usMaxPointCount`, `fStartCapSize`, `fEndCapSize` and
/// `bDoZPrePass` written to the stream's output binary stream (the component size kept in
/// the argument's stack word).
pub fn bs_strip_p_sys_data_save_binary(e: &mut Engine, this: Ptr<BSStripPSysData>, stream: Ptr) {
    let base = this.addr();
    e.call(PSYS_DATA_SAVE_BINARY_00C25560, &args![this, stream]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let output = (STREAM_OUTPUT_BINARY_STREAM, 0xc);
        stream_bytes(e, stream.addr(), output, base + 0x80, 2, slot);
        stream_bytes(e, stream.addr(), output, base + 0x84, 4, slot);
        stream_bytes(e, stream.addr(), output, base + 0x88, 4, slot);
        stream_bytes(e, stream.addr(), output, base + 0x8c, 1, slot);
    });
}

/// Adds the string at `slot` to the viewer string array (grown to growth + size when full).
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step.wrapping_add(size)]);
    }
    e.call(STRINGS_SET_AT, &args![strings, size, slot]);
}

// Translated from 00c26630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripPSysData::GetViewerStrings` (vtable slot +0x60, `thiscall`, one word: the string
/// array): `NiPSysData::GetViewerStrings`, then the class name (RTTI `01202680`),
/// `usMaxPointCount`, `fStartCapSize`, `fEndCapSize` (`FLD`/`FSTP`) and `bDoZPrePass`, each
/// added to the array (the string kept in the argument's stack word).
pub fn bs_strip_p_sys_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BSStripPSysData>,
    strings: Ptr,
) {
    let base = this.addr();
    e.call(PSYS_DATA_GET_VIEWER_STRINGS_00C255B0, &args![this, strings]);
    let strings = strings.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let name = e.global::<u32>(BS_STRIP_P_SYS_DATA_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let maximum = u32::from(e.mem.u16(base + 0x80));
        let text = e
            .call(
                VIEWER_STRING_SHORT_00A7D490,
                &args![MAX_POINT_COUNT_NAME, maximum],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        for (field, name) in [(0x84u32, START_CAP_SIZE_NAME), (0x88, END_CAP_SIZE_NAME)] {
            let value = x87_quiet_bits(e.mem.u32(base + field));
            let text = e
                .call(VIEWER_STRING_FLOAT_00A7D550, &args![name, value])
                .u32();
            e.mem.set_u32(slot, text);
            add_viewer_string(e, strings, slot);
        }
        let pre_pass = u32::from(e.mem.u8(base + 0x8c));
        let text = e
            .call(
                VIEWER_STRING_BOOL_00A7D370,
                &args![DO_Z_PRE_PASS_NAME, pre_pass],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c25ee0, b_s_strip_p_sys_data_get_r_t_t_i() -> u32),
        entry!(0x00a8_17a0, bs_strip_p_sys_data_link_object(Ptr, Ptr)),
        entry!(0x00a8_17b0, bs_strip_p_sys_data_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_5cd0,
            bs_strip_p_sys_data_is_equal(Ptr<BSStripPSysData>, Ptr<BSStripPSysData>) -> bool
        ),
        entry!(0x00c2_5d40, fn_00c25d40(Ptr<BSStripPSysData>, u32)),
        entry!(0x00c2_5db0, fn_00c25db0(Ptr<BSStripPSysData>, u32) -> u16),
        entry!(0x00c2_5dd0, fn_00c25dd0(Ptr<BSStripPSysData>) -> u16),
        entry!(0x00c2_5de0, fn_00c25de0(Ptr<BSStripPSysData>, u32) -> f64),
        entry!(
            0x00c2_5e00,
            fn_00c25e00(Ptr<BSStripPSysData>, u32, u32) -> u32
        ),
        entry!(
            0x00c2_5e20,
            fn_00c25e20(Ptr<BSStripPSysData>, u32, u32) -> f64
        ),
        entry!(
            0x00c2_5e40,
            fn_00c25e40(Ptr<BSStripPSysData>, u32, u32) -> f64
        ),
        entry!(
            0x00c2_5e70,
            fn_00c25e70(Ptr<BSStripPSysData>, u32, u32) -> u32
        ),
        entry!(
            0x00c2_5e90,
            fn_00c25e90(Ptr<BSStripPSysData>, u32, u32) -> u32
        ),
        entry!(0x00c2_5eb0, fn_00c25eb0(Ptr<BSStripPSysData>, u32) -> f64),
        entry!(0x00c2_5ef0, fn_00c25ef0(Ptr<BSStripPSysData>)),
        entry!(
            0x00c2_5f40,
            bs_strip_p_sys_data_remove_particle(Ptr<BSStripPSysData>, u32)
        ),
        entry!(
            0x00c2_6000,
            bs_strip_p_sys_data_copy_members(Ptr<BSStripPSysData>, Ptr<BSStripPSysData>, u32)
        ),
        entry!(0x00c2_6120, fn_00c26120(Ptr<BSStripPSysData>, u32, u32)),
        entry!(
            0x00c2_6350,
            fn_00c26350(Ptr<BSStripPSysData>, u32, u32) -> u32
        ),
        entry!(
            0x00c2_6380,
            bs_strip_p_sys_data_scalar_deleting_destructor(
                Ptr<BSStripPSysData>,
                u32,
            ) -> Ptr<BSStripPSysData>
        ),
        entry!(
            0x00c2_63b0,
            bs_strip_p_sys_data_create_clone(Ptr<BSStripPSysData>, u32) -> u32
        ),
        entry!(
            0x00c2_6410,
            bs_strip_p_sys_data_load_binary(Ptr<BSStripPSysData>, Ptr)
        ),
        entry!(
            0x00c2_6570,
            bs_strip_p_sys_data_save_binary(Ptr<BSStripPSysData>, Ptr)
        ),
        entry!(
            0x00c2_6630,
            bs_strip_p_sys_data_get_viewer_strings(Ptr<BSStripPSysData>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a817a0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_17a0;
        const TARGET: u32 = 0x00a7_b7d0;
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
    fn thunk_00a817b0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_17b0;
        const TARGET: u32 = 0x00a7_5f00;
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
    fn b_s_strip_p_sys_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_5ee0, &args![]).u32(), 0x0120_2680);
    }
}

#[cfg(test)]
mod n063_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// Strip data with `strips` strips of `points` points each (point `p` of strip `s` has
    /// `fSize` `s + p` and `fRadii` 2.0), the maximum point count `points`.
    fn data(e: &mut Engine, strips: u32, points: u32) -> Ptr<BSStripPSysData> {
        let this: Ptr<BSStripPSysData> = e.new_object();
        let list = e.mem.alloc(0x20 * strips);
        for s in 0..strips {
            let block = e.mem.alloc(POINT_SIZE * points);
            for p in 0..points {
                let point = block + POINT_SIZE * p;
                e.mem.set_f32(point + 0x7c, (s + p) as f32);
                e.mem.set_f32(point + 0x80, 2.0);
            }
            e.mem.set_u32(list + 0x20 * s, block);
            e.mem
                .set_u32(list + 0x20 * s + STRIP_MAX_POINT_COUNT, points);
        }
        e.set(this, BSStripPSysData::pStripList, list);
        e.set(this, BSStripPSysData::usMaxPointCount, points as u16);
        e.mem.set_u16(this.addr() + 8, strips as u16);
        this
    }

    fn strip_word(e: &Engine, this: Ptr<BSStripPSysData>, index: u32, field: u32) -> u32 {
        e.mem.u32(strip(e, this.addr(), index) + field)
    }

    #[test]
    fn is_equal_compares_the_strip_settings() {
        let mut e = Engine::new();
        e.register(PSYS_DATA_IS_EQUAL_00C24DE0, |_, _| 1u32.into_ret());
        let this = data(&mut e, 1, 2);
        let other = data(&mut e, 1, 2);
        for object in [this, other] {
            e.set(object, BSStripPSysData::fStartCapSize, 1.5f32.to_bits());
            e.set(object, BSStripPSysData::bDoZPrePass, 1);
        }
        assert!(e.call(0x00c2_5cd0, &args![this, other]).bool());
        e.set(other, BSStripPSysData::fEndCapSize, 0x7fc0_0000);
        e.set(this, BSStripPSysData::fEndCapSize, 0x7fc0_0000);
        assert!(!e.call(0x00c2_5cd0, &args![this, other]).bool());
        e.set(other, BSStripPSysData::fEndCapSize, 0);
        e.set(this, BSStripPSysData::fEndCapSize, 0);
        e.set(other, BSStripPSysData::bDoZPrePass, 0);
        assert!(!e.call(0x00c2_5cd0, &args![this, other]).bool());
        e.set(other, BSStripPSysData::bDoZPrePass, 1);
        e.set(other, BSStripPSysData::usMaxPointCount, 3);
        assert!(!e.call(0x00c2_5cd0, &args![this, other]).bool());
        e.set(other, BSStripPSysData::usMaxPointCount, 2);
        e.register(PSYS_DATA_IS_EQUAL_00C24DE0, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00c2_5cd0, &args![this, other]).bool());
    }

    #[test]
    fn strip_reset_and_accessors() {
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x1000);
        e.set_global(INITIAL_TIME_01015F5C, 0xff7f_ffffu32);
        let this = data(&mut e, 2, 3);
        e.call(0x00c2_5d40, &args![this, 0x0001_0001u32]);
        assert_eq!(strip_word(&e, this, 1, STRIP_POINT_COUNT), 0);
        assert_eq!(strip_word(&e, this, 1, STRIP_MAX_POINT_COUNT), 3);
        assert_eq!(strip_word(&e, this, 1, STRIP_START_INDEX), 3);
        assert_eq!(strip_word(&e, this, 1, STRIP_STOP_INDEX), 3);
        assert_eq!(strip_word(&e, this, 1, STRIP_FIRST_INDEX), u32::MAX);
        assert_eq!(strip_word(&e, this, 1, STRIP_LAST_INDEX), 0);
        assert_eq!(strip_word(&e, this, 1, STRIP_LAST_TIME), 0xff7f_ffff);
        let at = strip(&e, this.addr(), 1);
        e.mem.set_u16(at + STRIP_POINT_COUNT, 2);
        e.mem.set_u16(at + STRIP_HIGHEST_POINT_COUNT, 5);
        assert_eq!(e.call(0x00c2_5db0, &args![this, 1u32]).u32() & 0xffff, 2);
        assert_eq!(e.call(0x00c2_5dd0, &args![this]).u32() & 0xffff, 3);
        assert_eq!(
            e.call(0x00c2_5de0, &args![this, 1u32]).f64(),
            f64::from(f32::MIN)
        );
        let point = e.call(0x00c2_5e00, &args![this, 1u32, 2u32]).u32();
        assert_eq!(point, e.mem.u32(at) + 2 * POINT_SIZE);
        assert_eq!(e.call(0x00c2_5e20, &args![this, 1u32, 2u32]).f64(), 3.0);
        assert_eq!(e.call(0x00c2_5e40, &args![this, 1u32, 2u32]).f64(), 2.0);
        assert_eq!(
            e.call(0x00c2_5e70, &args![this, 1u32, 2u32]).u32(),
            point + 0xc
        );
        assert_eq!(
            e.call(0x00c2_5e90, &args![this, 1u32, 2u32]).u32(),
            point + 0x48
        );
        // 1 / (5 - 1), as a float.
        assert_eq!(e.call(0x00c2_5eb0, &args![this, 1u32]).f64(), 0.25);
        e.map(0x011a_9000, 0x1000);
        assert_eq!(
            e.call(0x00c2_6350, &args![this, 1u32, 2u32]).u32(),
            DEFAULT_COLOR_011A9BE0
        );
        e.mem.set_u32(this.addr() + 0x28, 0x500);
        assert_eq!(
            e.call(0x00c2_6350, &args![this, 1u32, 2u32]).u32(),
            point + 0x6c
        );
    }

    #[test]
    fn destructors_free_each_point_list() {
        let mut e = Engine::new();
        noop(
            &mut e,
            &[FREE_00AA10F0, PSYS_DATA_DESTRUCT_00C24EF0, NI_FREE],
        );
        let this = data(&mut e, 2, 1);
        let list = e.get(this, BSStripPSysData::pStripList);
        let first = e.mem.u32(list);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_6380, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BS_STRIP_P_SYS_DATA_VTABLE);
        let freed: Vec<u32> = calls(&e, FREE_00AA10F0).iter().map(|c| c[0]).collect();
        assert_eq!(freed.len(), 3);
        assert_eq!((freed[0], freed[2]), (first, list));
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(
            calls(&e, PSYS_DATA_DESTRUCT_00C24EF0),
            vec![vec![this.addr()]]
        );
        assert_eq!(calls(&e, NI_FREE), vec![vec![this.addr(), 0x90]]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5ef0, &args![this]);
        assert!(calls(&e, NI_FREE).is_empty());
    }

    #[test]
    fn remove_particle_drops_a_point_or_swaps_strips() {
        let mut e = Engine::new();
        noop(&mut e, &[PSYS_DATA_REMOVE_PARTICLE_00C24CC0]);
        let this = data(&mut e, 3, 4);
        let at = strip(&e, this.addr(), 0);
        e.mem.set_u16(at + STRIP_POINT_COUNT, 3);
        e.mem.set_u32(at + STRIP_FIRST_INDEX, 3);
        e.mem.set_u32(at + STRIP_STOP_INDEX, 9);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5f40, &args![this, 0u32]);
        assert_eq!(strip_word(&e, this, 0, STRIP_FIRST_INDEX), 0);
        assert_eq!(e.mem.u16(at + STRIP_POINT_COUNT), 2);
        assert_eq!(strip_word(&e, this, 0, STRIP_STOP_INDEX), 8);
        assert!(calls(&e, PSYS_DATA_REMOVE_PARTICLE_00C24CC0).is_empty());
        // One point: the particle goes and its strip swaps with the last particle's (2 of 3).
        e.mem.set_u16(this.addr() + 0x50, 3);
        let one = strip(&e, this.addr(), 1);
        e.mem.set_u16(one + STRIP_POINT_COUNT, 1);
        let lists = (e.mem.u32(one), e.mem.u32(strip(&e, this.addr(), 2)));
        e.call_log = Some(vec![]);
        e.call(0x00c2_5f40, &args![this, 0x0005_0001u32]);
        assert_eq!(
            calls(&e, PSYS_DATA_REMOVE_PARTICLE_00C24CC0),
            vec![vec![this.addr(), 0x0005_0001]]
        );
        assert_eq!(e.mem.u32(one), lists.1);
        assert_eq!(e.mem.u32(strip(&e, this.addr(), 2)), lists.0);
        // The last particle keeps its strip.
        e.call(0x00c2_5f40, &args![this, 2u32]);
        assert_eq!(e.mem.u32(strip(&e, this.addr(), 2)), lists.0);
    }

    #[test]
    fn copy_members_and_clone_copy_the_strips() {
        let mut e = Engine::new();
        noop(
            &mut e,
            &[
                PSYS_DATA_COPY_MEMBERS_00C24F20,
                PSYS_DATA_CONSTRUCT_00C24C90,
            ],
        );
        e.register(ALLOCATE_00AA1070, |e, a| {
            e.mem.alloc(a[0].max(4)).into_ret()
        });
        e.register(MEMCPY, |e, a| {
            for byte in 0..a[2] {
                let value = e.mem.u8(a[1] + byte);
                e.mem.set_u8(a[0] + byte, value);
            }
            Ret::default()
        });
        let this = data(&mut e, 2, 3);
        e.set(this, BSStripPSysData::fStartCapSize, 0x7f80_0001);
        e.set(this, BSStripPSysData::bDoZPrePass, 1);
        let at = strip(&e, this.addr(), 1);
        e.mem.set_u16(at + STRIP_POINT_COUNT, 2);
        e.mem.set_u32(at + STRIP_LAST_INDEX, 2);
        e.mem.set_u32(at + STRIP_LAST_TIME, 0x4000_0000);
        let clone: Ptr<BSStripPSysData> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c2_6000, &args![this, clone, 0x55u32]);
        assert_eq!(
            calls(&e, PSYS_DATA_COPY_MEMBERS_00C24F20),
            vec![vec![this.addr(), clone.addr(), 0x55]]
        );
        assert_eq!(e.get(clone, BSStripPSysData::fStartCapSize), 0x7fc0_0001);
        assert_eq!(e.get(clone, BSStripPSysData::usMaxPointCount), 3);
        assert_eq!(e.get(clone, BSStripPSysData::bDoZPrePass), 1);
        assert_eq!(strip_word(&e, clone, 1, STRIP_LAST_INDEX), 2);
        assert_eq!(strip_word(&e, clone, 1, STRIP_LAST_TIME), 0x4000_0000);
        assert_eq!(e.mem.u16(strip(&e, clone.addr(), 1) + STRIP_POINT_COUNT), 2);
        let copied = strip_word(&e, clone, 1, STRIP_POINTS);
        assert_ne!(copied, strip_word(&e, this, 1, STRIP_POINTS));
        assert_eq!(e.mem.f32(copied + POINT_SIZE + 0x7c), 2.0);
        let sizes: Vec<u32> = calls(&e, MEMCPY).iter().map(|c| c[2]).collect();
        assert_eq!(sizes, vec![0, 2 * POINT_SIZE]);
        // The clone: allocated, constructed, its list cleared, then the members copied.
        let block = e.mem.alloc(0x90);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_63b0, &args![this, 0x66u32]).u32(), block);
        assert_eq!(calls(&e, PSYS_DATA_CONSTRUCT_00C24C90), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), BS_STRIP_P_SYS_DATA_VTABLE);
        assert_ne!(e.mem.u32(block + 0x7c), 0);
        assert_eq!(calls(&e, PSYS_DATA_COPY_MEMBERS_00C24F20)[0][1], block);
    }

    #[test]
    fn add_point_fills_the_strip() {
        let mut e = Engine::new();
        noop(&mut e, &[STRIP_GEOMETRY_00A62BE0]);
        let this = data(&mut e, 1, 3);
        let base = this.addr();
        // The particle's info: age 1.0, lifespan 2.0; position, size, radius.
        let info = e.mem.alloc(0x1c);
        e.mem.set_f32(info + 0xc, 1.0);
        e.mem.set_f32(info + 0x10, 2.0);
        e.mem.set_u32(base + 0x70, info);
        let vertices = e.mem.alloc(0xc);
        for word in 0..3 {
            e.mem.set_u32(vertices + 4 * word, 0x10 + word);
        }
        e.mem.set_u32(base + 0x20, vertices);
        let sizes = e.mem.alloc(4);
        e.mem.set_f32(sizes, 4.0);
        e.mem.set_u32(base + 0x54, sizes);
        let radii = e.mem.alloc(4);
        e.mem.set_f32(radii, 0.5);
        e.mem.set_u32(base + 0x4c, radii);
        let at = strip(&e, base, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_6120, &args![this, 0u32, 0u32]);
        // An empty strip: first index 0, the point at the last index (0) filled and made.
        let points = e.mem.u32(at);
        assert_eq!(e.mem.u32(points + 8), 0x12);
        assert_eq!(e.mem.f32(points + 0x7c), 4.0);
        let geometry = calls(&e, STRIP_GEOMETRY_00A62BE0);
        assert_eq!(geometry.len(), 1);
        assert_eq!(&geometry[0][1..4], &[points, 0, 2.0f32.to_bits()]);
        assert_eq!(strip_word(&e, this, 0, STRIP_FIRST_INDEX), 0);
        assert_eq!(e.mem.u16(at + STRIP_POINT_COUNT), 1);
        assert_eq!(e.mem.u16(at + STRIP_HIGHEST_POINT_COUNT), 1);
        assert_eq!(strip_word(&e, this, 0, STRIP_START_INDEX), u32::MAX);
        assert_eq!(strip_word(&e, this, 0, STRIP_LAST_INDEX), 1);
        // A second point redoes the first with the position, then makes the new one with
        // the first as the point before.
        e.call_log = Some(vec![]);
        e.call(0x00c2_6120, &args![this, 0u32, 0u32]);
        let geometry = calls(&e, STRIP_GEOMETRY_00A62BE0);
        assert_eq!(geometry.len(), 2);
        assert_eq!(&geometry[0][1..3], &[points, vertices]);
        assert_eq!(geometry[1][1], points + POINT_SIZE);
        assert_eq!(e.mem.u16(at + STRIP_POINT_COUNT), 2);
        // A full strip moves its first index on.
        e.mem.set_u16(at + STRIP_POINT_COUNT, 3);
        e.mem.set_u32(at + STRIP_FIRST_INDEX, 2);
        e.call(0x00c2_6120, &args![this, 0u32, 0u32]);
        assert_eq!(strip_word(&e, this, 0, STRIP_FIRST_INDEX), 0);
        assert_eq!(strip_word(&e, this, 0, STRIP_LAST_INDEX), 0);
        // An expired particle adds nothing.
        e.mem.set_f32(info + 0x10, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x00c2_6120, &args![this, 0u32, 0u32]);
        assert!(calls(&e, STRIP_GEOMETRY_00A62BE0).is_empty());
    }

    #[test]
    fn binary_streaming_and_viewer_strings() {
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.set_global(BS_STRIP_P_SYS_DATA_RTTI, 0x0abc_0000u32);
        noop(
            &mut e,
            &[
                PSYS_DATA_LOAD_BINARY_00C25340,
                PSYS_DATA_SAVE_BINARY_00C25560,
                PSYS_DATA_GET_VIEWER_STRINGS_00C255B0,
                STRINGS_SET_AT,
                STRINGS_GROW,
            ],
        );
        e.register(ALLOCATE_00AA1070, |e, a| {
            e.mem.alloc(a[0].max(4)).into_ret()
        });
        // The read function fills each field with 0x11 bytes; both answer the count.
        e.register(0x0e60_0000, |e, a| {
            for byte in 0..a[2] {
                e.mem.set_u8(a[1] + byte, 0x11);
            }
            a[2].into_ret()
        });
        e.register(0x0e60_0010, |_, a| a[2].into_ret());
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 8, 0x0e60_0000);
        e.mem.set_u32(binary + 0xc, 0x0e60_0010);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_INPUT_BINARY_STREAM, binary);
        e.mem.set_u32(stream + STREAM_OUTPUT_BINARY_STREAM, binary);
        let this: Ptr<BSStripPSysData> = e.new_object();
        e.mem.set_u16(this.addr() + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c2_6410, &args![this, stream]);
        assert_eq!(e.get(this, BSStripPSysData::usMaxPointCount), 0x1111);
        assert_eq!(e.get(this, BSStripPSysData::bDoZPrePass), 0x11);
        assert_eq!(e.mem.u32(binary + 4), 11);
        let sizes: Vec<u32> = calls(&e, ALLOCATE_00AA1070).iter().map(|c| c[0]).collect();
        assert_eq!(sizes, vec![0x40, 0x1111 * POINT_SIZE, 0x1111 * POINT_SIZE]);
        assert_eq!(strip_word(&e, this, 1, STRIP_START_INDEX), 0x1111);
        assert_eq!(strip_word(&e, this, 1, STRIP_FIRST_INDEX), u32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x00c2_6570, &args![this, stream]);
        let written: Vec<(u32, u32)> = calls(&e, 0x0e60_0010)
            .iter()
            .map(|c| (c[1], c[2]))
            .collect();
        let base = this.addr();
        assert_eq!(
            written,
            vec![
                (base + 0x80, 2),
                (base + 0x84, 4),
                (base + 0x88, 4),
                (base + 0x8c, 1)
            ]
        );
        assert_eq!(e.mem.u32(binary + 4), 22);
        // Viewer strings: the class name and the four settings.
        for (address, tag) in [
            (NI_GET_VIEWER_STRING, 1u32),
            (VIEWER_STRING_SHORT_00A7D490, 2),
            (VIEWER_STRING_FLOAT_00A7D550, 3),
            (VIEWER_STRING_BOOL_00A7D370, 4),
        ] {
            e.register_double(address, move |_, _| tag.into_ret());
        }
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 9);
        e.call_log = Some(vec![]);
        e.call(0x00c2_6630, &args![this, strings]);
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0abc_0000]]);
        assert_eq!(
            calls(&e, VIEWER_STRING_FLOAT_00A7D550),
            vec![
                vec![START_CAP_SIZE_NAME, 0x1111_1111],
                vec![END_CAP_SIZE_NAME, 0x1111_1111]
            ]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_BOOL_00A7D370),
            vec![vec![DO_Z_PRE_PASS_NAME, 0x11]]
        );
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 5);
    }
}
