//! `BSShader/run_00bc1ba0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// The bolt shader property these functions work on: `BoltShaderProperty` by its layout
    /// (the Xbox PDB's `BoltShaderProperty`, 0x198 bytes, matches the code field by field with
    /// every offset 8 lower, the PC `BSShaderProperty` being 0x60 bytes; its vtable `010ba528`
    /// has no RTTI locator, so the name rests on the layout and on `BoltShader::GetBoltGeometry`
    /// constructing it). 0x190 bytes on PC.
    pub struct BoltShaderProperty: 0x190 {
        /// `pSegmentData` (Xbox PDB): `BoltShader::SegmentData*`, 0x10 bytes each (x, y, z,
        /// unused), `iSegmentsPerBolt` per bolt.
        0x60 pSegmentData: u32,
        /// `TargetLocation` (Xbox PDB): an `NiPoint3`.
        0x64 TargetLocation_x: u32,
        0x68 TargetLocation_y: u32,
        0x6c TargetLocation_z: u32,
        /// `fTimer` (Xbox PDB).
        0x70 fTimer: u32,
        /// `iLookBack` (Xbox PDB).
        0x74 iLookBack: i32,
        /// `iBoltStartSegment` (Xbox PDB).
        0x78 iBoltStartSegment: i32,
        /// `iBoltEndSegment` (Xbox PDB).
        0x7c iBoltEndSegment: i32,
        /// `iTotalSegmentCount` (Xbox PDB).
        0x80 iTotalSegmentCount: i32,
        /// `fDistanceLong` (Xbox PDB).
        0x84 fDistanceLong: u32,
        /// `fTortuosity` (Xbox PDB): 40 `float`s.
        0x88 fTortuosity: u32,
        /// `iNumBolts` (Xbox PDB).
        0x128 iNumBolts: i32,
        /// `fBoltLength` (Xbox PDB).
        0x12c fBoltLength: u32,
        /// `fBoltsRadius` (Xbox PDB).
        0x130 fBoltsRadius: u32,
        /// `fBoltsRadiusStrength` (Xbox PDB).
        0x134 fBoltsRadiusStrength: u32,
        /// `fBoltSmallWidth` (Xbox PDB).
        0x138 fBoltSmallWidth: u32,
        /// `fBoltGrowWidth` (Xbox PDB).
        0x13c fBoltGrowWidth: u32,
        /// `iSegmentsPerBolt` (Xbox PDB).
        0x140 iSegmentsPerBolt: i32,
        /// `fSegmentLength` (Xbox PDB).
        0x144 fSegmentLength: u32,
        /// `fSegmentVariance` (Xbox PDB).
        0x148 fSegmentVariance: u32,
        /// `iSubSegments` (Xbox PDB).
        0x14c iSubSegments: u32,
        /// `fSubSegmentVariance` (Xbox PDB).
        0x150 fSubSegmentVariance: u32,
        /// `CoreColor` and `GlowColor` (Xbox PDB): `NiColorA`s.
        0x154 CoreColor: u32,
        0x164 GlowColor: u32,
        /// `bWrapped` (Xbox PDB).
        0x174 bWrapped: u8,
        /// `bMoving` (Xbox PDB).
        0x175 bMoving: u8,
        /// `bGuided` (Xbox PDB).
        0x176 bGuided: u8,
        /// `bStartFromPoint` (Xbox PDB).
        0x177 bStartFromPoint: u8,
        /// `pMyGeometry` (Xbox PDB).
        0x178 pMyGeometry: u32,
        /// `iBoltMidSegment` (Xbox PDB).
        0x17c iBoltMidSegment: i32,
        /// `fDistanceShort` (Xbox PDB).
        0x180 fDistanceShort: u32,
        /// `iBoltCurrentSegmentCount` (Xbox PDB).
        0x184 iBoltCurrentSegmentCount: i32,
        /// `iTotalAddedSegments` (Xbox PDB).
        0x188 iTotalAddedSegments: i32,
        /// `fAlpha` (Xbox PDB, the class's own).
        0x18c fAlpha: u32,
    }
}

/// The class's vtable (`010ba528`) and `GetRTTI`'s record (`01201f68`; its first word is the
/// class name), the instance count at `01201f64`, the size (0x190).
const BOLT_SHADER_PROPERTY_VTABLE: u32 = 0x010b_a528;
const BOLT_SHADER_PROPERTY_RTTI: u32 = 0x0120_1f68;
const INSTANCE_COUNT: u32 = 0x0120_1f64;
const BOLT_SHADER_PROPERTY_SIZE: u32 = 0x190;
/// `BSShaderProperty`'s constructor (`00ba8e50`), destructor (`00baa0e0`), member copy
/// (`00ba8b10`, thiscall on the source: clone, cloning process) and `GetViewerStrings`
/// (`00ba9890`).
const SHADER_PROPERTY_CONSTRUCT: u32 = 0x00ba_8e50;
const SHADER_PROPERTY_DESTRUCT: u32 = 0x00ba_a0e0;
const SHADER_PROPERTY_COPY_MEMBERS: u32 = 0x00ba_8b10;
const SHADER_PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00ba_9890;
/// `BSShaderProperty::CheckCreateRenderPassArray` (`00ba95e0`, `RET 4`) and
/// `RenderPassArray::Add` (`00ba9ee0`, `RET 0x20`), engine map names.
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// `BoltShader`'s `00bc0b40` (no arguments: the segment count, computed once into
/// `01201f50`), `GetSegmentIndices` (`00bc0b70`, engine map) and `00bc0c50` (no arguments:
/// run when the last instance goes).
const BOLT_SEGMENT_COUNT_00BC0B40: u32 = 0x00bc_0b40;
const BOLT_GET_SEGMENT_INDICES: u32 = 0x00bc_0b70;
const BOLT_RELEASE_00BC0C50: u32 = 0x00bc_0c50;
/// The memory manager (`011f6238`), its `Allocate` (`00aa3e40`) and `Deallocate`
/// (`00aa4060`); `NiNew`'s allocator (`00aa13e0`, cdecl) and the sized free (`00aa1460`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// CRT: `_memset` (`00ec61c0`), `_rand` (`00ecadb8`), `_ftol2` (`00ec62c0`, `ST0` in, a
/// leading `f64`), `ceil` (`00ec9e10`, a `double` on the stack, `ST0` out) and `_CIsqrt`
/// (`00ec6040`, `ST0` in and out).
const MEMSET: u32 = 0x00ec_61c0;
const RAND: u32 = 0x00ec_adb8;
const FTOL: u32 = 0x00ec_62c0;
const CEIL: u32 = 0x00ec_9e10;
const CRT_SQRT: u32 = 0x00ec_6040;
/// `NiPoint3::Normalize` (`004a0c10`, thiscall on the point).
const POINT_NORMALIZE: u32 = 0x004a_0c10;
/// Constants: `32767.0`, `0.5`, `1.0`, `0.0`, `0.1` (`double`s), `0.1f`, `2048.0f`,
/// `100.0f`, `3.0f`; the int at `011ae61c` (the sub-segment default), the colour at
/// `011a9be0` (four words) and the point at `011f426c`.
const RAND_MAX_01029790: u32 = 0x0102_9790;
const HALF_01011588: u32 = 0x0101_1588;
const ONE_01012070: u32 = 0x0101_2070;
const ZERO_01012060: u32 = 0x0101_2060;
const STEP_LIMIT_0101FFA0: u32 = 0x0101_ffa0;
const STEP_LIMIT_F32_0101E2BC: u32 = 0x0101_e2bc;
const BOLT_LENGTH_01018BFC: u32 = 0x0101_8bfc;
const BOLTS_RADIUS_01016410: u32 = 0x0101_6410;
const RADIUS_STRENGTH_01017718: u32 = 0x0101_7718;
const SUB_SEGMENTS_011AE61C: u32 = 0x011a_e61c;
const COLOUR_011A9BE0: u32 = 0x011a_9be0;
const POINT_011F426C: u32 = 0x011f_426c;
/// The string array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`), `NiGetViewerString`
/// (`00a7d330`).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;

/// The address of segment `index` (`pSegmentData + index * 0x10`).
fn segment(e: &Engine, this: Ptr<BoltShaderProperty>, index: i32) -> u32 {
    e.get(this, BoltShaderProperty::pSegmentData)
        .wrapping_add((index as u32).wrapping_mul(0x10))
}

/// `_rand() / 32767.0` as the exe stores it: `FILD`, `FDIV` by the `double`, `FSTP`.
fn random_fraction(e: &mut Engine) -> u32 {
    let random = e.call(RAND, &[]).i32();
    let divisor = f64::from_bits(e.global::<u64>(RAND_MAX_01029790));
    x87_store32(x87_div(f64::from(random), divisor))
}

/// `_ftol2` of `value` (in `ST0`).
fn float_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

// Translated from 00bc1b90 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `BoltShaderProperty::SetupGeometry` (the vtable's slot +0x9c; thiscall, `RET 4`): the
/// geometry becomes `pMyGeometry`. The answer's low byte is 1, its upper bytes the
/// geometry's (`MOV EAX,[ESP+4]` then `MOV AL,1`).
pub fn bolt_shader_property_setup_geometry(
    e: &mut Engine,
    this: Ptr<BoltShaderProperty>,
    geometry: u32,
) -> u32 {
    e.set(this, BoltShaderProperty::pMyGeometry, geometry);
    (geometry & 0xffff_ff00) | 1
}

// Translated from 00bc1ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`: a `float`): adds the value to the y of the
/// live segments of every bolt, from `iBoltStartSegment` to `iBoltEndSegment` through the
/// ring of `iSegmentsPerBolt`: while wrapped the walk runs to the ring's end and then on
/// from 0 unwrapped; unwrapped it stops past the end segment (signed compares, the counts
/// read at each step).
pub fn fn_00bc1ba0(e: &mut Engine, this: Ptr<BoltShaderProperty>, delta: f32) {
    let delta = x87_load32(delta.to_bits());
    let mut wrapped = e.get(this, BoltShaderProperty::bWrapped);
    let mut index = e.get(this, BoltShaderProperty::iBoltStartSegment);
    let mut from_top = true;
    loop {
        let per_bolt = e.get(this, BoltShaderProperty::iSegmentsPerBolt);
        let runs = from_top && wrapped != 0 && index < per_bolt;
        if !runs && index > e.get(this, BoltShaderProperty::iBoltEndSegment) {
            break;
        }
        let mut bolt = 0i32;
        while bolt < e.get(this, BoltShaderProperty::iNumBolts) {
            let per_bolt = e.get(this, BoltShaderProperty::iSegmentsPerBolt);
            let y =
                segment(e, this, per_bolt.wrapping_mul(bolt).wrapping_add(index)).wrapping_add(4);
            bolt = bolt.wrapping_add(1);
            let value = x87_store32(x87_add(x87_load32(e.mem.u32(y)), delta));
            e.mem.set_u32(y, value);
        }
        index = index.wrapping_add(1);
        if index < e.get(this, BoltShaderProperty::iSegmentsPerBolt) {
            from_top = true;
            continue;
        }
        if wrapped == 0 {
            from_top = false;
            continue;
        }
        index = 0;
        wrapped = 0;
        from_top = true;
    }
}

// Translated from 00bc1c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`: a segment count): pulls the end segment back
/// by `count % iSegmentsPerBolt` (signed division); a wrapped ring whose end went below 0 gets
/// its start moved up by a ring and is unwrapped. `iBoltCurrentSegmentCount` drops by the
/// same remainder. Still wrapped: answers the quotient. Otherwise the end is not left before
/// the start (set to it unless above it) and the answer is the start.
pub fn fn_00bc1c10(e: &mut Engine, this: Ptr<BoltShaderProperty>, count: i32) -> i32 {
    let per_bolt = e.get(this, BoltShaderProperty::iSegmentsPerBolt);
    let quotient = count.wrapping_div(per_bolt);
    let remainder = count.wrapping_rem(per_bolt);
    let end = e
        .get(this, BoltShaderProperty::iBoltEndSegment)
        .wrapping_sub(remainder);
    e.set(this, BoltShaderProperty::iBoltEndSegment, end);
    let current = |e: &mut Engine| {
        let left = e
            .get(this, BoltShaderProperty::iBoltCurrentSegmentCount)
            .wrapping_sub(remainder);
        e.set(this, BoltShaderProperty::iBoltCurrentSegmentCount, left);
    };
    if e.get(this, BoltShaderProperty::bWrapped) != 0 {
        if end < 0 {
            let start = e
                .get(this, BoltShaderProperty::iBoltStartSegment)
                .wrapping_add(per_bolt);
            e.set(this, BoltShaderProperty::iBoltStartSegment, start);
            e.set(this, BoltShaderProperty::bWrapped, 0);
        }
        if e.get(this, BoltShaderProperty::bWrapped) != 0 {
            current(e);
            return quotient;
        }
    }
    let start = e.get(this, BoltShaderProperty::iBoltStartSegment);
    current(e);
    if end > start {
        return start;
    }
    e.set(this, BoltShaderProperty::iBoltEndSegment, start);
    start
}

// Translated from 00bc1c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`: a segment count): moves the start segment on
/// by `count % iSegmentsPerBolt` (signed); a wrapped ring whose start reached the ring's
/// length comes back by a ring and is unwrapped. `iBoltCurrentSegmentCount` drops by the
/// remainder. Unwrapped, a start at or past the end is set to the end.
pub fn fn_00bc1c70(e: &mut Engine, this: Ptr<BoltShaderProperty>, count: i32) {
    let per_bolt = e.get(this, BoltShaderProperty::iSegmentsPerBolt);
    let remainder = count.wrapping_rem(per_bolt);
    let start = e
        .get(this, BoltShaderProperty::iBoltStartSegment)
        .wrapping_add(remainder);
    e.set(this, BoltShaderProperty::iBoltStartSegment, start);
    if e.get(this, BoltShaderProperty::bWrapped) != 0 && start >= per_bolt {
        e.set(
            this,
            BoltShaderProperty::iBoltStartSegment,
            start.wrapping_sub(per_bolt),
        );
        e.set(this, BoltShaderProperty::bWrapped, 0);
    }
    let left = e
        .get(this, BoltShaderProperty::iBoltCurrentSegmentCount)
        .wrapping_sub(remainder);
    e.set(this, BoltShaderProperty::iBoltCurrentSegmentCount, left);
    if e.get(this, BoltShaderProperty::bWrapped) == 0 {
        let end = e.get(this, BoltShaderProperty::iBoltEndSegment);
        if e.get(this, BoltShaderProperty::iBoltStartSegment) >= end {
            e.set(this, BoltShaderProperty::iBoltStartSegment, end);
        }
    }
}

// Translated from 00bc1cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 8`: two `float`s `a` and `b`): moves the start
/// segment on by `n % iSegmentsPerBolt`, `n` being `_ftol2(a * (iTotalAddedSegments /
/// fTimer))` but at least 1 (unwrapping as [`fn_00bc1c70`] does; unwrapped, a start at or past
/// the end is set to the end), lowers `iBoltCurrentSegmentCount` by the remainder and adds
/// `a * b` (stored as a `float` in the first argument's stack word) to `fDistanceShort`.
pub fn fn_00bc1cc0(e: &mut Engine, this: Ptr<BoltShaderProperty>, a: f32, b: f32) {
    let a = x87_load32(a.to_bits());
    let product = x87_store32(x87_mul(a, x87_m32(b.to_bits())));
    let added = f64::from(e.get(this, BoltShaderProperty::iTotalAddedSegments));
    let rate = x87_div(added, x87_m32(e.get(this, BoltShaderProperty::fTimer)));
    let mut count = float_to_int(e, x87_mul(a, rate));
    if count <= 0 {
        count = 1;
    }
    let per_bolt = e.get(this, BoltShaderProperty::iSegmentsPerBolt);
    let remainder = count.wrapping_rem(per_bolt);
    let start = e
        .get(this, BoltShaderProperty::iBoltStartSegment)
        .wrapping_add(remainder);
    e.set(this, BoltShaderProperty::iBoltStartSegment, start);
    let mut clamp = true;
    if e.get(this, BoltShaderProperty::bWrapped) != 0 {
        if start >= per_bolt {
            e.set(
                this,
                BoltShaderProperty::iBoltStartSegment,
                start.wrapping_sub(per_bolt),
            );
            e.set(this, BoltShaderProperty::bWrapped, 0);
        }
        clamp = e.get(this, BoltShaderProperty::bWrapped) == 0;
    }
    if clamp {
        let end = e.get(this, BoltShaderProperty::iBoltEndSegment);
        if e.get(this, BoltShaderProperty::iBoltStartSegment) >= end {
            e.set(this, BoltShaderProperty::iBoltStartSegment, end);
        }
    }
    let distance = x87_load32(e.get(this, BoltShaderProperty::fDistanceShort));
    let left = e
        .get(this, BoltShaderProperty::iBoltCurrentSegmentCount)
        .wrapping_sub(remainder);
    e.set(this, BoltShaderProperty::iBoltCurrentSegmentCount, left);
    let distance = x87_store32(x87_add(distance, x87_m32(product)));
    e.set(this, BoltShaderProperty::fDistanceShort, distance);
}

// Translated from 00bc1d40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc1d40 - the C++ exception frame (FS:[0] chain, unwind state 0 after the base constructor) is not translated.
/// `BoltShaderProperty`'s constructor (by the vtable it stores; fastcall): the
/// `BSShaderProperty` constructor (`00ba8e50`), the vtable, both colours, the distances,
/// timer and segment indices zero, `TargetLocation` (0, the `float` then at `fBoltLength`'s
/// offset, read before it is set, 0), unwrapped, guided and starting from a point, a
/// look-back of 4, `iSegmentsPerBolt` from `00bc0b40`, segment length and both variances
/// 1.0, `iSubSegments` from `011ae61c`, one bolt of length 2048, radius 100, radius strength
/// and small width 3.0, grow width 1.0, both colours copied from `011a9be0`, moving; then
/// `BoltShader::GetSegmentIndices`, no geometry, a zeroed segment array of `00bc0b40`
/// segments from the memory manager (all ones on overflow), 40 tortuosity values
/// `_rand() / 32767.0`, the instance count raised, `fAlpha` 1.0, no added segments,
/// `iShader` 0x19 and `iShaderPropertyType` 0xf. Answers `this`.
pub fn fn_00bc1d40(e: &mut Engine, this: Ptr<BoltShaderProperty>) -> Ptr<BoltShaderProperty> {
    let base = this.addr();
    e.call(SHADER_PROPERTY_CONSTRUCT, &args![this]);
    let zero = x87_store32(0.0);
    e.mem.set_u32(base, BOLT_SHADER_PROPERTY_VTABLE);
    for word in 0..8u32 {
        e.mem.set_u32(base + 0x154 + 4 * word, zero);
    }
    e.set(this, BoltShaderProperty::fDistanceLong, zero);
    e.set(this, BoltShaderProperty::iTotalSegmentCount, 0);
    e.set(this, BoltShaderProperty::fDistanceShort, zero);
    e.set(this, BoltShaderProperty::iBoltCurrentSegmentCount, 0);
    e.set(this, BoltShaderProperty::fTimer, zero);
    e.set(this, BoltShaderProperty::iBoltStartSegment, 0);
    e.set(this, BoltShaderProperty::iBoltEndSegment, 0);
    let length = x87_quiet_bits(e.get(this, BoltShaderProperty::fBoltLength));
    e.set(this, BoltShaderProperty::TargetLocation_x, zero);
    e.set(this, BoltShaderProperty::TargetLocation_y, length);
    e.set(this, BoltShaderProperty::bWrapped, 0);
    e.set(this, BoltShaderProperty::bGuided, 1);
    e.set(this, BoltShaderProperty::bStartFromPoint, 1);
    e.set(this, BoltShaderProperty::TargetLocation_z, zero);
    e.set(this, BoltShaderProperty::iLookBack, 4);
    let segments = e.call(BOLT_SEGMENT_COUNT_00BC0B40, &[]).i32();
    let one = x87_store32(1.0);
    e.set(this, BoltShaderProperty::fSegmentLength, one);
    e.set(this, BoltShaderProperty::iSegmentsPerBolt, segments);
    e.set(this, BoltShaderProperty::fSegmentVariance, one);
    let sub_segments = e.global::<u32>(SUB_SEGMENTS_011AE61C);
    e.set(this, BoltShaderProperty::fSubSegmentVariance, one);
    let bolt_length = x87_quiet_bits(e.global::<u32>(BOLT_LENGTH_01018BFC));
    e.set(this, BoltShaderProperty::iSubSegments, sub_segments);
    e.set(this, BoltShaderProperty::fBoltLength, bolt_length);
    e.set(this, BoltShaderProperty::iNumBolts, 1);
    let radius = x87_quiet_bits(e.global::<u32>(BOLTS_RADIUS_01016410));
    e.set(this, BoltShaderProperty::iTotalAddedSegments, 0);
    e.set(this, BoltShaderProperty::fBoltsRadius, radius);
    let strength = x87_quiet_bits(e.global::<u32>(RADIUS_STRENGTH_01017718));
    e.set(this, BoltShaderProperty::fBoltsRadiusStrength, strength);
    e.set(this, BoltShaderProperty::fBoltSmallWidth, strength);
    e.set(this, BoltShaderProperty::fBoltGrowWidth, one);
    for colour in [0x154u32, 0x164] {
        for word in 0..4u32 {
            let value = e.global::<u32>(COLOUR_011A9BE0 + 4 * word);
            e.mem.set_u32(base + colour + 4 * word, value);
        }
    }
    e.set(this, BoltShaderProperty::bMoving, 1);
    e.call(BOLT_GET_SEGMENT_INDICES, &[]);
    e.set(this, BoltShaderProperty::pMyGeometry, 0);
    let count = e.call(BOLT_SEGMENT_COUNT_00BC0B40, &[]).u32();
    let bytes = count.checked_mul(0x10).unwrap_or(u32::MAX);
    let array = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32();
    e.set(this, BoltShaderProperty::pSegmentData, array);
    e.call(MEMSET, &args![array, 0u32, count.wrapping_shl(4)]);
    for index in 0..40u32 {
        let value = random_fraction(e);
        e.mem.set_u32(base + 0x88 + 4 * index, value);
    }
    let instances = e.global::<u32>(INSTANCE_COUNT).wrapping_add(1);
    e.set_global(INSTANCE_COUNT, instances);
    e.set(this, BoltShaderProperty::fAlpha, one);
    e.set(this, BoltShaderProperty::iTotalAddedSegments, 0);
    // `BSShaderProperty::iShader` (+0x58) and `NiShadeProperty::iShaderPropertyType`
    // (+0x1c), PC offsets.
    e.mem.set_u32(base + 0x58, 0x19);
    e.mem.set_u32(base + 0x1c, 0xf);
    this
}

// Translated from 00bc1f90 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `BoltShaderProperty::GetRTTI` (the vtable's slot +8): the address of the record at
/// `01201f68`.
pub fn bolt_shader_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BOLT_SHADER_PROPERTY_RTTI)
}

// Translated from 00bc1fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc1fa0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the base destructor) is not translated.
/// `BoltShaderProperty`'s destructor body (fastcall): the vtable, the segment array given
/// back to the memory manager, the instance count lowered (`00bc0c50` when it reaches 0),
/// then `BSShaderProperty`'s destructor (`00baa0e0`).
pub fn fn_00bc1fa0(e: &mut Engine, this: Ptr<BoltShaderProperty>) {
    e.mem.set_u32(this.addr(), BOLT_SHADER_PROPERTY_VTABLE);
    let array = e.get(this, BoltShaderProperty::pSegmentData);
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, array]);
    let instances = e.global::<u32>(INSTANCE_COUNT).wrapping_sub(1);
    e.set_global(INSTANCE_COUNT, instances);
    if instances == 0 {
        e.call(BOLT_RELEASE_00BC0C50, &[]);
    }
    e.call(SHADER_PROPERTY_DESTRUCT, &args![this]);
}

// Translated from 00bc2020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bc2020 - the C++ exception frame (FS:[0] chain, unwind state 0 around the constructor) is not translated.
/// `BoltShaderProperty::CreateClone` (the vtable's slot +0x48; thiscall, `RET 4`: the cloning
/// process): 0x190 bytes (`00aa13e0`) constructed ([`fn_00bc1d40`]; null when the allocation
/// fails), then `BSShaderProperty`'s member copy into it (`00ba8b10`, into null too).
pub fn bolt_shader_property_create_clone(
    e: &mut Engine,
    this: Ptr<BoltShaderProperty>,
    cloning: u32,
) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![BOLT_SHADER_PROPERTY_SIZE]).u32();
    let clone = if block != 0 {
        fn_00bc1d40(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.call(SHADER_PROPERTY_COPY_MEMBERS, &args![this, clone, cloning]);
    clone
}

// Translated from 00bc20a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x10`: bolt, segment, previous and look-back
/// segment indices): places a new segment of the bolt. The direction from the look-back
/// segment to the previous one (replaced by the point at `011f426c` when its y is below 0)
/// gets 1.0 added to its y, is normalised (`004a0c10`) and scaled by `fSegmentLength`; the new
/// segment is the previous one plus that step, with x and z each moved by
/// `fSegmentVariance * _rand() / 32767.0 - fSegmentVariance * 0.5`. Then, with `f` the
/// segment index over `iSegmentsPerBolt`, the horizontal offset from the segment to
/// `f * TargetLocation` (x and z) is measured (`_CIsqrt`); beyond `fBoltsRadius` the segment is
/// pulled towards it by `fBoltsRadiusStrength * (d - radius) / d`, or by `(d - radius) / d`
/// when that pull times `d` exceeds the radius. The exe's argument words are reused as
/// scratch `float`s, as here; each x87 step follows the exe.
pub fn fn_00bc20a0(
    e: &mut Engine,
    this: Ptr<BoltShaderProperty>,
    bolt: i32,
    index: i32,
    previous: i32,
    back: i32,
) {
    let per_bolt = e.get(this, BoltShaderProperty::iSegmentsPerBolt);
    let row = per_bolt.wrapping_mul(bolt);
    let target = row.wrapping_add(index);
    let from = segment(e, this, row.wrapping_add(previous));
    let p = [e.mem.u32(from), e.mem.u32(from + 4), e.mem.u32(from + 8)];
    let per_bolt_now = e.get(this, BoltShaderProperty::iSegmentsPerBolt);
    let to = segment(e, this, per_bolt_now.wrapping_mul(bolt).wrapping_add(back));
    let q = [e.mem.u32(to), e.mem.u32(to + 4), e.mem.u32(to + 8)];
    let mut d = [0u32; 3];
    for i in 0..3 {
        d[i] = x87_store32(x87_sub(x87_load32(p[i]), x87_m32(q[i])));
    }
    // `FLDZ; FCOMP dy; TEST AH,0x41; JNZ`: replaced only when 0 > dy (NaN keeps it).
    if 0.0 > x87_m32(d[1]) {
        d = [
            e.global::<u32>(POINT_011F426C),
            e.global::<u32>(POINT_011F426C + 4),
            e.global::<u32>(POINT_011F426C + 8),
        ];
    }
    let one = f64::from_bits(e.global::<u64>(ONE_01012070));
    d[1] = x87_store32(x87_add(x87_load32(d[1]), one));
    let d = e.with_stack(0xc, |e, local| {
        let local = local.addr();
        for (i, word) in d.iter().enumerate() {
            e.mem.set_u32(local + 4 * i as u32, *word);
        }
        e.call(POINT_NORMALIZE, &args![local]);
        [e.mem.u32(local), e.mem.u32(local + 4), e.mem.u32(local + 8)]
    });
    let length = x87_load32(x87_store32(x87_load32(
        e.get(this, BoltShaderProperty::fSegmentLength),
    )));
    let step = [
        x87_store32(x87_mul(length, x87_m32(d[0]))),
        x87_store32(x87_mul(length, x87_m32(d[1]))),
        x87_store32(x87_mul(length, x87_m32(d[2]))),
    ];
    let fraction = x87_store32(x87_div(f64::from(index), f64::from(per_bolt_now)));
    let half = f64::from_bits(e.global::<u64>(HALF_01011588));
    let offset = (target as u32).wrapping_mul(0x10);
    let scatter = |e: &mut Engine, step: u32, base: u32| {
        let random = random_fraction(e);
        let variance = e.get(this, BoltShaderProperty::fSegmentVariance);
        let spread = x87_mul(x87_load32(variance), x87_m32(random));
        let moved = x87_add(x87_load32(step), x87_m32(base));
        let sum = x87_add(spread, moved);
        let variance = e.get(this, BoltShaderProperty::fSegmentVariance);
        x87_quiet_bits(x87_store32(x87_sub(
            sum,
            x87_mul(x87_load32(variance), half),
        )))
    };
    let x = scatter(e, step[0], p[0]);
    let buffer = e.get(this, BoltShaderProperty::pSegmentData);
    e.mem.set_u32(buffer.wrapping_add(offset), x);
    let y = x87_quiet_bits(x87_store32(x87_add(x87_load32(step[1]), x87_m32(p[1]))));
    let buffer = e.get(this, BoltShaderProperty::pSegmentData);
    e.mem
        .set_u32(buffer.wrapping_add(offset).wrapping_add(4), y);
    let z = scatter(e, step[2], p[2]);
    let buffer = e.get(this, BoltShaderProperty::pSegmentData);
    e.mem
        .set_u32(buffer.wrapping_add(offset).wrapping_add(8), z);
    let placed = e
        .get(this, BoltShaderProperty::pSegmentData)
        .wrapping_add(offset);
    let target_x = x87_load32(e.get(this, BoltShaderProperty::TargetLocation_x));
    let f = x87_load32(fraction);
    let reach_x = x87_store32(x87_sub(x87_mul(target_x, f), x87_load32(e.mem.u32(placed))));
    let reach_z = x87_store32(x87_sub(
        x87_mul(
            f,
            x87_m32(e.get(this, BoltShaderProperty::TargetLocation_z)),
        ),
        x87_m32(e.mem.u32(placed.wrapping_add(8))),
    ));
    let rz = x87_load32(reach_z);
    let rx = x87_load32(reach_x);
    let squared = x87_store32(x87_add(x87_mul(rz, rz), x87_mul(rx, rx)));
    let root = e.call(CRT_SQRT, &args![x87_load32(squared)]).st0;
    let distance = x87_load32(x87_store32(x87_load32(x87_store32(root))));
    let radius = x87_load32(e.get(this, BoltShaderProperty::fBoltsRadius));
    // `FLD radius; FCOMP; TEST AH,5; JP`: on only when radius < d.
    if !(radius < distance) {
        return;
    }
    let beyond = x87_sub(
        distance,
        x87_m32(e.get(this, BoltShaderProperty::fBoltsRadius)),
    );
    let strength = x87_load32(e.get(this, BoltShaderProperty::fBoltsRadiusStrength));
    let pull = x87_load32(x87_store32(x87_div(x87_mul(strength, beyond), distance)));
    let reach = x87_load32(x87_quiet_bits(x87_store32(
        x87_load32(x87_store32(x87_mul(pull, distance))).abs(),
    )));
    let radius = x87_load32(e.get(this, BoltShaderProperty::fBoltsRadius));
    let factor = if radius < reach {
        x87_load32(x87_store32(x87_div(beyond, distance)))
    } else {
        pull
    };
    let move_x = x87_store32(x87_mul(x87_load32(reach_x), factor));
    let move_z = x87_store32(x87_mul(factor, x87_m32(reach_z)));
    let placed = e
        .get(this, BoltShaderProperty::pSegmentData)
        .wrapping_add(offset);
    let x = x87_store32(x87_add(x87_load32(e.mem.u32(placed)), x87_m32(move_x)));
    e.mem.set_u32(placed, x);
    let placed = e
        .get(this, BoltShaderProperty::pSegmentData)
        .wrapping_add(offset)
        .wrapping_add(8);
    let z = x87_store32(x87_add(x87_load32(e.mem.u32(placed)), x87_m32(move_z)));
    e.mem.set_u32(placed, z);
}

// Translated from 00bc2360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (fastcall; called from `Init3D`, `Start3D` and `008139f0`): resets
/// the bolt. `fTimer` 0.0 and unwrapped; when `iNumBolts * iSegmentsPerBolt` exceeds the
/// segment count of `00bc0b40`, `iNumBolts` drops by `_ftol2(ceil(excess / iSegmentsPerBolt))`.
/// `iLookBack` becomes `min(iSegmentsPerBolt, 4)`, `iTotalSegmentCount` the bolts times the
/// segments; the first `iNumBolts` segments of the array are zeroed when starting from a
/// point, else each coordinate set to `fBoltsRadius * _rand() / 32767.0 - fBoltsRadius * 0.5`.
/// Start and end segment 0.
pub fn fn_00bc2360(e: &mut Engine, this: Ptr<BoltShaderProperty>) {
    let available = e.call(BOLT_SEGMENT_COUNT_00BC0B40, &[]).i32();
    let zero = x87_store32(0.0);
    let per_bolt = e.get(this, BoltShaderProperty::iSegmentsPerBolt);
    e.set(this, BoltShaderProperty::fTimer, zero);
    let wanted = e
        .get(this, BoltShaderProperty::iNumBolts)
        .wrapping_mul(per_bolt);
    e.set(this, BoltShaderProperty::bWrapped, 0);
    if available < wanted {
        let excess = wanted.wrapping_sub(available);
        let share = x87_store32(x87_div(f64::from(excess), f64::from(per_bolt)));
        let rounded = e.call(CEIL, &args![x87_load32(share)]).st0;
        let rounded = x87_load32(x87_store32(x87_load32(x87_store32(rounded))));
        let drop = float_to_int(e, rounded);
        let bolts = e
            .get(this, BoltShaderProperty::iNumBolts)
            .wrapping_sub(drop);
        e.set(this, BoltShaderProperty::iNumBolts, bolts);
    }
    let per_bolt = e.get(this, BoltShaderProperty::iSegmentsPerBolt);
    let look_back = if per_bolt > 4 { 4 } else { per_bolt };
    e.set(this, BoltShaderProperty::iLookBack, look_back);
    let bolts = e.get(this, BoltShaderProperty::iNumBolts);
    e.set(
        this,
        BoltShaderProperty::iTotalSegmentCount,
        bolts.wrapping_mul(per_bolt),
    );
    let half = f64::from_bits(e.global::<u64>(HALF_01011588));
    let mut index = 0i32;
    while index < e.get(this, BoltShaderProperty::iNumBolts) {
        let offset = (index as u32).wrapping_mul(0x10);
        if e.get(this, BoltShaderProperty::bStartFromPoint) != 0 {
            for word in [0u32, 4, 8] {
                let buffer = e.get(this, BoltShaderProperty::pSegmentData);
                e.mem.set_u32(buffer.wrapping_add(offset + word), zero);
            }
        } else {
            for word in [0u32, 4, 8] {
                let random = random_fraction(e);
                let radius = e.get(this, BoltShaderProperty::fBoltsRadius);
                let spread = x87_mul(x87_load32(radius), x87_m32(random));
                let radius = e.get(this, BoltShaderProperty::fBoltsRadius);
                let value = x87_quiet_bits(x87_store32(x87_sub(
                    spread,
                    x87_mul(x87_load32(radius), half),
                )));
                let buffer = e.get(this, BoltShaderProperty::pSegmentData);
                e.mem.set_u32(buffer.wrapping_add(offset + word), value);
            }
        }
        index = index.wrapping_add(1);
    }
    e.set(this, BoltShaderProperty::iBoltEndSegment, 0);
    e.set(this, BoltShaderProperty::iBoltStartSegment, 0);
}

// Translated from 00bc2510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoltShaderProperty`'s scalar deleting destructor (the vtable's slot 0; thiscall,
/// `RET 4`): the destructor body ([`fn_00bc1fa0`]) and, with bit 0 of `flags`, the 0x190 bytes
/// given back (`00aa1460`). Answers `this`.
pub fn bolt_shader_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BoltShaderProperty>,
    flags: u8,
) -> Ptr<BoltShaderProperty> {
    fn_00bc1fa0(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, BOLT_SHADER_PROPERTY_SIZE]);
    }
    this
}

// Translated from 00bc2540 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// `BoltShaderProperty::GetRenderPasses` (the vtable's slot +0xa0; thiscall, `RET 0x18`: the
/// geometry and five unread words). Makes the render pass list when needed (`00ba95e0` with
/// 1); when it is empty, adds pass 0x242 for the geometry (`00ba9ee0` with 1 and five zeros)
/// and marks the last pass's `eAccumulationHint` byte (+6) 1. Answers the list.
pub fn bolt_shader_property_get_render_passes(
    e: &mut Engine,
    this: Ptr<BoltShaderProperty>,
    geometry: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![this, 1u32]);
    // `BSShaderProperty::pRenderPassList` (+0x3c); the array's `iSize` (+0x10), buffer (+4).
    let list = e.mem.u32(this.addr() + 0x3c);
    if e.mem.u32(list.wrapping_add(0x10)) != 0 {
        return list;
    }
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![list, geometry, 0x242u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    let list = e.mem.u32(this.addr() + 0x3c);
    let count = e.mem.u32(list.wrapping_add(0x10));
    let buffer = e.mem.u32(list.wrapping_add(4));
    let last = e
        .mem
        .u32(buffer.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4));
    e.mem.set_u8(last.wrapping_add(6), 1);
    e.mem.u32(this.addr() + 0x3c)
}

// Translated from 00bc2590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 4`: a count): grows the bolts `count` times (none
/// for a count of 0 or below): the end segment moves on around the ring (the start pushed on
/// when the end reaches it, also around the ring), `bWrapped` becomes whether the start is
/// past the end, and each bolt gets its new end segment placed ([`fn_00bc20a0`]) from the
/// segment before it and the one `iLookBack` back (both taken around the ring when wrapped,
/// else 0 at the front).
pub fn fn_00bc2590(e: &mut Engine, this: Ptr<BoltShaderProperty>, count: i32) {
    if count <= 0 {
        return;
    }
    let mut left = count;
    loop {
        let end = e
            .get(this, BoltShaderProperty::iBoltEndSegment)
            .wrapping_add(1);
        e.set(this, BoltShaderProperty::iBoltEndSegment, end);
        let per_bolt = e.get(this, BoltShaderProperty::iSegmentsPerBolt);
        if end >= per_bolt {
            e.set(this, BoltShaderProperty::iBoltEndSegment, 0);
        }
        let end = e.get(this, BoltShaderProperty::iBoltEndSegment);
        let start = e.get(this, BoltShaderProperty::iBoltStartSegment);
        if end == start {
            let start = start.wrapping_add(1);
            e.set(this, BoltShaderProperty::iBoltStartSegment, start);
            if start >= per_bolt {
                e.set(
                    this,
                    BoltShaderProperty::iBoltStartSegment,
                    start.wrapping_sub(per_bolt),
                );
            }
        }
        let wrapped = e.get(this, BoltShaderProperty::iBoltStartSegment) > end;
        e.set(this, BoltShaderProperty::bWrapped, u8::from(wrapped));
        let mut bolt = 0i32;
        while bolt < e.get(this, BoltShaderProperty::iNumBolts) {
            let wrapped = e.get(this, BoltShaderProperty::bWrapped) != 0;
            let previous = if end != 0 {
                end.wrapping_sub(1)
            } else if wrapped {
                e.get(this, BoltShaderProperty::iSegmentsPerBolt)
                    .wrapping_sub(1)
            } else {
                0
            };
            let look_back = e.get(this, BoltShaderProperty::iLookBack);
            let back = if end >= look_back {
                end.wrapping_sub(look_back)
            } else if wrapped {
                e.get(this, BoltShaderProperty::iSegmentsPerBolt)
                    .wrapping_sub(look_back)
                    .wrapping_add(end)
            } else {
                0
            };
            fn_00bc20a0(e, this, bolt, end, previous, back);
            bolt = bolt.wrapping_add(1);
        }
        left = left.wrapping_sub(1);
        if left == 0 {
            break;
        }
    }
}

// Translated from 00bc2660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 8`: a time step and a distance, `float`s): adds
/// the step to `fTimer`; the segments to add are `ceil((distance - fDistanceLong) /
/// fSegmentLength)`, `fDistanceLong` moves on by that many lengths and the bolt grows by them
/// ([`fn_00bc2590`] with `_ftol2` of it), `iTotalAddedSegments` counting them; then
/// `fDistanceShort` becomes `fDistanceLong` less the smaller of itself and the ring's length
/// (`iSegmentsPerBolt * fSegmentLength`), not below 0. The exe's argument words hold the
/// intermediate `float`s.
pub fn fn_00bc2660(e: &mut Engine, this: Ptr<BoltShaderProperty>, step: f32, distance: f32) {
    let timer = x87_store32(x87_add(
        x87_load32(step.to_bits()),
        x87_m32(e.get(this, BoltShaderProperty::fTimer)),
    ));
    e.set(this, BoltShaderProperty::fTimer, timer);
    let ahead = x87_store32(x87_sub(
        x87_load32(distance.to_bits()),
        x87_m32(e.get(this, BoltShaderProperty::fDistanceLong)),
    ));
    let share = x87_store32(x87_div(
        x87_load32(ahead),
        x87_m32(e.get(this, BoltShaderProperty::fSegmentLength)),
    ));
    let rounded = e.call(CEIL, &args![x87_load32(share)]).st0;
    let rounded = x87_load32(x87_store32(x87_load32(x87_store32(rounded))));
    let long = x87_store32(x87_add(
        x87_mul(
            rounded,
            x87_m32(e.get(this, BoltShaderProperty::fSegmentLength)),
        ),
        x87_m32(e.get(this, BoltShaderProperty::fDistanceLong)),
    ));
    e.set(this, BoltShaderProperty::fDistanceLong, long);
    let added = float_to_int(e, rounded);
    fn_00bc2590(e, this, added);
    let long = x87_quiet_bits(e.get(this, BoltShaderProperty::fDistanceLong));
    let ring = x87_store32(x87_mul(
        f64::from(e.get(this, BoltShaderProperty::iSegmentsPerBolt)),
        x87_m32(e.get(this, BoltShaderProperty::fSegmentLength)),
    ));
    let long_value = x87_load32(long);
    let ring_value = x87_load32(ring);
    // `FLD long; FLD ring; FCOM; TEST AH,0x41; JNZ`: `long` only when the ring is greater.
    let smaller = x87_store32(if ring_value > long_value {
        long_value
    } else {
        ring_value
    });
    let short = x87_store32(x87_sub(
        x87_load32(e.get(this, BoltShaderProperty::fDistanceLong)),
        x87_m32(smaller),
    ));
    let short_value = x87_load32(short);
    let zero = f64::from_bits(e.global::<u64>(ZERO_01012060));
    let short_value = if short_value < zero { 0.0 } else { short_value };
    let total = e
        .get(this, BoltShaderProperty::iTotalAddedSegments)
        .wrapping_add(added);
    e.set(this, BoltShaderProperty::iTotalAddedSegments, total);
    let short = x87_quiet_bits(x87_store32(short_value));
    e.set(this, BoltShaderProperty::fDistanceShort, short);
}

// Translated from 00bc2740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x10`: a time step, a distance, a speed and an
/// unread word; called from `Update3D` and `00812700`): moves the bolt's segments by the step
/// (at most 0.1) times the speed ([`fn_00bc1ba0`]); the wanted distance is the given one but
/// at least `fSegmentLength`. `fDistanceLong` and `fDistanceShort` become the end and start
/// segments' y; a start at or below `-fSegmentLength` drops `_ftol2(-y / length)` segments at
/// the front ([`fn_00bc1c70`]) and `fDistanceShort` is read again. Then a bolt shorter than
/// the wanted distance grows by `_ftol2((wanted - long) / length)` segments ([`fn_00bc2590`]),
/// one at or past the wanted distance plus a length is cut back by `_ftol2((long - (wanted +
/// length)) / length)` ([`fn_00bc1c10`]), each followed by `fDistanceLong` read again from the
/// end segment. The exe's argument words hold the intermediate `float`s.
pub fn fn_00bc2740(
    e: &mut Engine,
    this: Ptr<BoltShaderProperty>,
    step: f32,
    distance: f32,
    speed: f32,
    _unused_4: u32,
) {
    let step_value = x87_load32(step.to_bits());
    let limit = f64::from_bits(e.global::<u64>(STEP_LIMIT_0101FFA0));
    // `FCOM limit; TEST AH,0x41; JNZ`: cut only when greater.
    let step = if step_value > limit {
        x87_store32(x87_load32(e.global::<u32>(STEP_LIMIT_F32_0101E2BC)))
    } else {
        x87_store32(step_value)
    };
    let moved = x87_store32(x87_mul(x87_load32(step), x87_m32(speed.to_bits())));
    let moved = x87_quiet_bits(x87_store32(x87_load32(moved)));
    fn_00bc1ba0(e, this, f32::from_bits(moved));
    let length = x87_load32(x87_store32(x87_load32(
        e.get(this, BoltShaderProperty::fSegmentLength),
    )));
    let distance_value = x87_load32(distance.to_bits());
    // `FLD length; FLD distance; FCOM; TEST AH,5; JP`: the length when distance < length.
    let wanted = if distance_value < length {
        x87_load32(x87_store32(length))
    } else {
        distance_value
    };
    let end = e.get(this, BoltShaderProperty::iBoltEndSegment);
    let start = e.get(this, BoltShaderProperty::iBoltStartSegment);
    let long = x87_quiet_bits(e.mem.u32(segment(e, this, end).wrapping_add(4)));
    e.set(this, BoltShaderProperty::fDistanceLong, long);
    let short = x87_load32(x87_store32(x87_load32(
        e.mem.u32(segment(e, this, start).wrapping_add(4)),
    )));
    e.set(this, BoltShaderProperty::fDistanceShort, x87_store32(short));
    let negative_length = -x87_load32(e.get(this, BoltShaderProperty::fSegmentLength));
    // `FLD -length; FCOMP short; TEST AH,1; JNZ`: on when not -length < short (NaN skips).
    let unordered = negative_length.is_nan() || short.is_nan();
    if !(unordered || negative_length < short) {
        let segments = x87_store32(x87_div(
            -short,
            x87_m32(e.get(this, BoltShaderProperty::fSegmentLength)),
        ));
        let drop = float_to_int(e, x87_load32(segments));
        fn_00bc1c70(e, this, drop);
        let start = e.get(this, BoltShaderProperty::iBoltStartSegment);
        let short = x87_quiet_bits(e.mem.u32(segment(e, this, start).wrapping_add(4)));
        e.set(this, BoltShaderProperty::fDistanceShort, short);
    }
    let long = x87_load32(e.get(this, BoltShaderProperty::fDistanceLong));
    // `FLD long; FCOMP wanted; TEST AH,5; JP`: grow when long < wanted.
    if long < wanted {
        let segments = x87_store32(x87_div(
            x87_sub(
                wanted,
                x87_m32(e.get(this, BoltShaderProperty::fDistanceLong)),
            ),
            x87_m32(e.get(this, BoltShaderProperty::fSegmentLength)),
        ));
        let grow = float_to_int(e, x87_load32(segments));
        fn_00bc2590(e, this, grow);
        let end = e.get(this, BoltShaderProperty::iBoltEndSegment);
        let long = x87_quiet_bits(e.mem.u32(segment(e, this, end).wrapping_add(4)));
        e.set(this, BoltShaderProperty::fDistanceLong, long);
        return;
    }
    let reach = x87_add(
        wanted,
        x87_m32(e.get(this, BoltShaderProperty::fSegmentLength)),
    );
    let long = x87_load32(e.get(this, BoltShaderProperty::fDistanceLong));
    // `FLD long; FCOMP reach; TEST AH,1; JNZ`: cut when not long < reach (NaN skips).
    if long.is_nan() || reach.is_nan() || long < reach {
        return;
    }
    let segments = x87_store32(x87_div(
        x87_sub(
            x87_m32(e.get(this, BoltShaderProperty::fDistanceLong)),
            reach,
        ),
        x87_m32(e.get(this, BoltShaderProperty::fSegmentLength)),
    ));
    let cut = float_to_int(e, x87_load32(segments));
    fn_00bc1c10(e, this, cut);
    let end = e.get(this, BoltShaderProperty::iBoltEndSegment);
    let long = x87_quiet_bits(e.mem.u32(segment(e, this, end).wrapping_add(4)));
    e.set(this, BoltShaderProperty::fDistanceLong, long);
}

// Translated from 00bc28b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x10`: three `float`s and a flag word; called
/// from `00812b60`, `008133f0` and `00814d40`): with the flag's low byte clear,
/// [`fn_00bc2660`] with the first and second `float`, else [`fn_00bc1cc0`] with the first and
/// third (each copied through `FLD`/`FSTP`).
pub fn fn_00bc28b0(
    e: &mut Engine,
    this: Ptr<BoltShaderProperty>,
    first: f32,
    second: f32,
    third: f32,
    flag: u32,
) {
    let first = f32::from_bits(x87_quiet_bits(first.to_bits()));
    if flag as u8 == 0 {
        let second = f32::from_bits(x87_quiet_bits(second.to_bits()));
        fn_00bc2660(e, this, first, second);
    } else {
        let third = f32::from_bits(x87_quiet_bits(third.to_bits()));
        fn_00bc1cc0(e, this, first, third);
    }
}

// Translated from 00bc28f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BoltShaderProperty::GetViewerStrings` (the vtable's slot +0x60; thiscall, `RET 4`):
/// `BSShaderProperty::GetViewerStrings` (`00ba9890`), then the class name (the first word of
/// the record at `01201f68`), held in the argument word and added through `SetSize`/`SetAt`.
pub fn bolt_shader_property_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BoltShaderProperty>,
    strings: u32,
) {
    e.call(SHADER_PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(BOLT_SHADER_PROPERTY_RTTI);
    let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        let index = u32::from(e.mem.u16(strings + 0xa));
        if index >= u32::from(e.mem.u16(strings + 8)) {
            let grow = u32::from(e.mem.u16(strings + 0xe));
            e.call(
                STRING_ARRAY_SET_SIZE,
                &args![strings, grow.wrapping_add(index)],
            );
        }
        e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00bc_1b90,
            bolt_shader_property_setup_geometry(Ptr<BoltShaderProperty>, u32) -> u32
        ),
        entry!(0x00bc_1ba0, fn_00bc1ba0(Ptr<BoltShaderProperty>, f32)),
        entry!(
            0x00bc_1c10,
            fn_00bc1c10(Ptr<BoltShaderProperty>, i32) -> i32
        ),
        entry!(0x00bc_1c70, fn_00bc1c70(Ptr<BoltShaderProperty>, i32)),
        entry!(0x00bc_1cc0, fn_00bc1cc0(Ptr<BoltShaderProperty>, f32, f32)),
        entry!(
            0x00bc_1d40,
            fn_00bc1d40(Ptr<BoltShaderProperty>) -> Ptr<BoltShaderProperty>
        ),
        entry!(0x00bc_1f90, bolt_shader_property_get_rtti() -> Ptr),
        entry!(0x00bc_1fa0, fn_00bc1fa0(Ptr<BoltShaderProperty>)),
        entry!(
            0x00bc_2020,
            bolt_shader_property_create_clone(Ptr<BoltShaderProperty>, u32) -> u32
        ),
        entry!(
            0x00bc_20a0,
            fn_00bc20a0(Ptr<BoltShaderProperty>, i32, i32, i32, i32)
        ),
        entry!(0x00bc_2360, fn_00bc2360(Ptr<BoltShaderProperty>)),
        entry!(
            0x00bc_2510,
            bolt_shader_property_scalar_deleting_destructor(
                Ptr<BoltShaderProperty>,
                u8,
            ) -> Ptr<BoltShaderProperty>
        ),
        entry!(
            0x00bc_2540,
            bolt_shader_property_get_render_passes(
                Ptr<BoltShaderProperty>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> u32
        ),
        entry!(0x00bc_2590, fn_00bc2590(Ptr<BoltShaderProperty>, i32)),
        entry!(0x00bc_2660, fn_00bc2660(Ptr<BoltShaderProperty>, f32, f32)),
        entry!(
            0x00bc_2740,
            fn_00bc2740(Ptr<BoltShaderProperty>, f32, f32, f32, u32)
        ),
        entry!(
            0x00bc_28b0,
            fn_00bc28b0(Ptr<BoltShaderProperty>, f32, f32, f32, u32)
        ),
        entry!(
            0x00bc_28f0,
            bolt_shader_property_get_viewer_strings(Ptr<BoltShaderProperty>, u32)
        ),
    ]
}

#[cfg(test)]
mod n014_tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The leading `f64` argument of a double.
    fn arg_f64(a: &[u32]) -> f64 {
        f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))
    }

    /// An engine with the constants and the CRT and point helpers these functions call:
    /// `_rand` answers 0, `_ftol2` truncates, `ceil`, `_CIsqrt` and the normalisation are
    /// exact.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for (address, value) in [
            (RAND_MAX_01029790, 32767.0f64),
            (HALF_01011588, 0.5),
            (ONE_01012070, 1.0),
            (ZERO_01012060, 0.0),
            (STEP_LIMIT_0101FFA0, 0.1),
        ] {
            e.mem.map(address, 8);
            e.mem.set_u64(address, value.to_bits());
        }
        e.mem.map(STEP_LIMIT_F32_0101E2BC, 4);
        e.mem.set_f32(STEP_LIMIT_F32_0101E2BC, 0.1);
        e.mem.map(POINT_011F426C, 12);
        e.register(RAND, |_, _| 0u32.into_ret());
        e.register(FTOL, |_, a| (arg_f64(a) as i32).into_ret());
        e.register(CEIL, |_, a| arg_f64(a).ceil().into_ret());
        e.register(CRT_SQRT, |_, a| arg_f64(a).sqrt().into_ret());
        e.register(POINT_NORMALIZE, |e, a| {
            let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (i, value) in v.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, value / length);
            }
            Ret::default()
        });
        e
    }

    /// A bolt property with `bolts` bolts of `per_bolt` segments, segment length 1, no
    /// variance, radius 10.
    fn bolt(e: &mut Engine, bolts: i32, per_bolt: i32) -> Ptr<BoltShaderProperty> {
        let this = Ptr::<BoltShaderProperty>::new(e.mem.alloc(0x190));
        let segments = e.mem.alloc(0x10 * (bolts * per_bolt).max(1) as u32);
        e.set(this, BoltShaderProperty::pSegmentData, segments);
        e.set(this, BoltShaderProperty::iNumBolts, bolts);
        e.set(this, BoltShaderProperty::iSegmentsPerBolt, per_bolt);
        e.set(this, BoltShaderProperty::iLookBack, 2);
        e.set(this, BoltShaderProperty::fSegmentLength, 1.0f32.to_bits());
        e.set(this, BoltShaderProperty::fBoltsRadius, 10.0f32.to_bits());
        this
    }

    fn set_segment(e: &mut Engine, this: Ptr<BoltShaderProperty>, index: i32, xyz: [f32; 3]) {
        let at = segment(e, this, index);
        for (i, value) in xyz.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *value);
        }
    }

    fn segment_xyz(e: &Engine, this: Ptr<BoltShaderProperty>, index: i32) -> [f32; 3] {
        let at = segment(e, this, index);
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn float(
        e: &Engine,
        this: Ptr<BoltShaderProperty>,
        field: Field<BoltShaderProperty, u32>,
    ) -> f32 {
        f32::from_bits(e.get(this, field))
    }

    #[test]
    fn setup_geometry_keeps_the_geometry_and_answers_one_in_the_low_byte() {
        let mut e = engine();
        let this = bolt(&mut e, 1, 2);
        assert_eq!(
            e.call(0x00bc_1b90, &args![this, 0x1234_5678u32]).u32(),
            0x1234_5601
        );
        assert_eq!(e.get(this, BoltShaderProperty::pMyGeometry), 0x1234_5678);
    }

    #[test]
    fn segments_move_over_the_live_range_of_every_bolt() {
        let mut e = engine();
        let this = bolt(&mut e, 2, 3);
        e.set(this, BoltShaderProperty::iBoltStartSegment, 1);
        e.set(this, BoltShaderProperty::iBoltEndSegment, 2);
        e.call(0x00bc_1ba0, &args![this, 0.5f32]);
        let ys: Vec<f32> = (0..6).map(|i| segment_xyz(&e, this, i)[1]).collect();
        assert_eq!(ys, vec![0.0, 0.5, 0.5, 0.0, 0.5, 0.5]);
        // Wrapped: from the start to the ring's end, then from 0 to the end.
        e.set(this, BoltShaderProperty::bWrapped, 1);
        e.set(this, BoltShaderProperty::iBoltStartSegment, 2);
        e.set(this, BoltShaderProperty::iBoltEndSegment, 0);
        e.call(0x00bc_1ba0, &args![this, 0.25f32]);
        let ys: Vec<f32> = (0..6).map(|i| segment_xyz(&e, this, i)[1]).collect();
        assert_eq!(ys, vec![0.25, 0.5, 0.75, 0.25, 0.5, 0.75]);
        // The walk keeps the flag in a register: the object stays wrapped.
        assert_eq!(e.get(this, BoltShaderProperty::bWrapped), 1);
    }

    #[test]
    fn cut_back_pulls_the_end_and_unwraps() {
        let mut e = engine();
        let this = bolt(&mut e, 1, 4);
        e.set(this, BoltShaderProperty::iBoltStartSegment, 1);
        e.set(this, BoltShaderProperty::iBoltEndSegment, 3);
        e.set(this, BoltShaderProperty::iBoltCurrentSegmentCount, 10);
        // Unwrapped: end 3 - 5 % 4 = 2, above the start: answers the start.
        assert_eq!(e.call(0x00bc_1c10, &args![this, 5u32]).i32(), 1);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltEndSegment), 2);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltCurrentSegmentCount), 9);
        // Cut below the start: the end is set to it.
        assert_eq!(e.call(0x00bc_1c10, &args![this, 2u32]).i32(), 1);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltEndSegment), 1);
        // Wrapped, the end going below 0: the start moves up by a ring, unwrapped.
        e.set(this, BoltShaderProperty::bWrapped, 1);
        e.set(this, BoltShaderProperty::iBoltStartSegment, 3);
        e.set(this, BoltShaderProperty::iBoltEndSegment, 0);
        assert_eq!(e.call(0x00bc_1c10, &args![this, 1u32]).i32(), 7);
        assert_eq!(e.get(this, BoltShaderProperty::bWrapped), 0);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltEndSegment), 7);
        // Wrapped and staying so: answers the quotient.
        e.set(this, BoltShaderProperty::bWrapped, 1);
        e.set(this, BoltShaderProperty::iBoltEndSegment, 3);
        assert_eq!(e.call(0x00bc_1c10, &args![this, 9u32]).i32(), 2);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltEndSegment), 2);
    }

    #[test]
    fn front_drop_moves_the_start_and_unwraps() {
        let mut e = engine();
        let this = bolt(&mut e, 1, 4);
        e.set(this, BoltShaderProperty::iBoltStartSegment, 0);
        e.set(this, BoltShaderProperty::iBoltEndSegment, 3);
        e.call(0x00bc_1c70, &args![this, 6u32]);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltStartSegment), 2);
        assert_eq!(
            e.get(this, BoltShaderProperty::iBoltCurrentSegmentCount),
            -2
        );
        // Past the end: set to the end.
        e.call(0x00bc_1c70, &args![this, 3u32]);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltStartSegment), 3);
        // Wrapped: past the ring it comes back and unwraps.
        e.set(this, BoltShaderProperty::bWrapped, 1);
        e.set(this, BoltShaderProperty::iBoltEndSegment, 1);
        e.call(0x00bc_1c70, &args![this, 2u32]);
        assert_eq!(e.get(this, BoltShaderProperty::bWrapped), 0);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltStartSegment), 1);
    }

    #[test]
    fn timed_drop_uses_the_added_rate_and_adds_the_distance() {
        let mut e = engine();
        let this = bolt(&mut e, 1, 8);
        e.set(this, BoltShaderProperty::iBoltEndSegment, 7);
        e.set(this, BoltShaderProperty::iTotalAddedSegments, 6);
        e.set(this, BoltShaderProperty::fTimer, 2.0f32.to_bits());
        e.set(this, BoltShaderProperty::fDistanceShort, 1.0f32.to_bits());
        // a = 1: 1 * 6 / 2 = 3 segments; b = 4: the distance grows by 4.
        e.call(0x00bc_1cc0, &args![this, 1.0f32, 4.0f32]);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltStartSegment), 3);
        assert_eq!(
            e.get(this, BoltShaderProperty::iBoltCurrentSegmentCount),
            -3
        );
        assert_eq!(float(&e, this, BoltShaderProperty::fDistanceShort), 5.0);
        // A rate under one segment still moves one.
        e.call(0x00bc_1cc0, &args![this, 0.1f32, 0.0f32]);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltStartSegment), 4);
    }

    #[test]
    fn constructor_sets_the_defaults_and_the_segment_array() {
        let mut e = engine();
        e.register(SHADER_PROPERTY_CONSTRUCT, |_, _| Ret::default());
        e.register(BOLT_SEGMENT_COUNT_00BC0B40, |_, _| 6u32.into_ret());
        e.register(BOLT_GET_SEGMENT_INDICES, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        let counter = Rc::new(Cell::new(0u32));
        let seen = counter.clone();
        e.register_double(RAND, move |_, _| {
            seen.set(seen.get() + 1);
            (32767 * (seen.get() % 2)).into_ret()
        });
        for (address, value) in [
            (BOLT_LENGTH_01018BFC, 2048.0f32),
            (BOLTS_RADIUS_01016410, 100.0),
            (RADIUS_STRENGTH_01017718, 3.0),
        ] {
            e.mem.map(address, 4);
            e.mem.set_f32(address, value);
        }
        e.mem.map(SUB_SEGMENTS_011AE61C, 4);
        e.mem.set_u32(SUB_SEGMENTS_011AE61C, 7);
        e.mem.map(COLOUR_011A9BE0, 16);
        for i in 0..4 {
            e.mem.set_u32(COLOUR_011A9BE0 + 4 * i, 0x100 + i);
        }
        e.mem.map(INSTANCE_COUNT, 4);
        let this = Ptr::<BoltShaderProperty>::new(e.mem.alloc(0x190));
        e.set(this, BoltShaderProperty::fBoltLength, 9.0f32.to_bits());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_1d40, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BOLT_SHADER_PROPERTY_VTABLE);
        // The target's y is the length as it was before the constructor set it.
        assert_eq!(float(&e, this, BoltShaderProperty::TargetLocation_y), 9.0);
        assert_eq!(float(&e, this, BoltShaderProperty::fBoltLength), 2048.0);
        assert_eq!(e.get(this, BoltShaderProperty::iSegmentsPerBolt), 6);
        assert_eq!(e.get(this, BoltShaderProperty::iSubSegments), 7);
        assert_eq!(float(&e, this, BoltShaderProperty::fBoltSmallWidth), 3.0);
        assert_eq!(float(&e, this, BoltShaderProperty::fBoltGrowWidth), 1.0);
        assert_eq!(e.get(this, BoltShaderProperty::iLookBack), 4);
        assert_eq!(
            (
                e.get(this, BoltShaderProperty::bWrapped),
                e.get(this, BoltShaderProperty::bMoving),
                e.get(this, BoltShaderProperty::bGuided),
                e.get(this, BoltShaderProperty::bStartFromPoint)
            ),
            (0, 1, 1, 1)
        );
        assert_eq!(e.mem.u32(this.addr() + 0x164 + 12), 0x103);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x60]]
        );
        let array = e.get(this, BoltShaderProperty::pSegmentData);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![array, 0, 0x60]]);
        assert_eq!(counter.get(), 40);
        assert_eq!(e.mem.f32(this.addr() + 0x88), 1.0);
        assert_eq!(e.mem.f32(this.addr() + 0x8c), 0.0);
        assert_eq!(e.mem.u32(INSTANCE_COUNT), 1);
        assert_eq!(
            (e.mem.u32(this.addr() + 0x58), e.mem.u32(this.addr() + 0x1c)),
            (0x19, 0xf)
        );
        assert_eq!(float(&e, this, BoltShaderProperty::fAlpha), 1.0);
    }

    #[test]
    fn get_rtti_answers_the_record() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00bc_1f90, &args![]).u32(),
            BOLT_SHADER_PROPERTY_RTTI
        );
    }

    #[test]
    fn destructor_frees_the_segments_and_counts_the_instances() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register(BOLT_RELEASE_00BC0C50, |_, _| Ret::default());
        e.register(SHADER_PROPERTY_DESTRUCT, |_, _| Ret::default());
        e.mem.map(INSTANCE_COUNT, 4);
        e.mem.set_u32(INSTANCE_COUNT, 2);
        let this = bolt(&mut e, 1, 2);
        let array = e.get(this, BoltShaderProperty::pSegmentData);
        e.call_log = Some(vec![]);
        e.call(0x00bc_1fa0, &args![this]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, array]]
        );
        assert!(calls_to(&e, BOLT_RELEASE_00BC0C50).is_empty());
        e.call(0x00bc_1fa0, &args![this]);
        assert_eq!(calls_to(&e, BOLT_RELEASE_00BC0C50).len(), 1);
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_DESTRUCT),
            vec![vec![this.addr()]; 2]
        );
        assert_eq!(e.mem.u32(this.addr()), BOLT_SHADER_PROPERTY_VTABLE);
    }

    #[test]
    fn create_clone_constructs_and_copies_the_base() {
        let mut e = engine();
        for address in [
            SHADER_PROPERTY_CONSTRUCT,
            BOLT_GET_SEGMENT_INDICES,
            MEMSET,
            SHADER_PROPERTY_COPY_MEMBERS,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(BOLT_SEGMENT_COUNT_00BC0B40, |_, _| 2u32.into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        for address in [
            BOLT_LENGTH_01018BFC,
            BOLTS_RADIUS_01016410,
            RADIUS_STRENGTH_01017718,
            SUB_SEGMENTS_011AE61C,
            INSTANCE_COUNT,
        ] {
            e.mem.map(address, 4);
        }
        e.mem.map(COLOUR_011A9BE0, 16);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00bc_2020, &args![0x1000u32, 0x2000u32]).u32();
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x190]]);
        assert_eq!(e.mem.u32(clone), BOLT_SHADER_PROPERTY_VTABLE);
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_COPY_MEMBERS),
            vec![vec![0x1000, clone, 0x2000]]
        );
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00bc_2020, &args![0x1000u32, 0x2000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_COPY_MEMBERS)[1],
            vec![0x1000, 0, 0x2000]
        );
    }

    #[test]
    fn new_segment_steps_from_the_previous_and_is_pulled_to_the_target() {
        let mut e = engine();
        let this = bolt(&mut e, 1, 4);
        set_segment(&mut e, this, 0, [0.0, 0.0, 0.0]);
        set_segment(&mut e, this, 1, [0.0, 1.0, 0.0]);
        e.set(
            this,
            BoltShaderProperty::TargetLocation_x,
            16.0f32.to_bits(),
        );
        e.set(this, BoltShaderProperty::fBoltsRadius, 1.0f32.to_bits());
        e.set(
            this,
            BoltShaderProperty::fBoltsRadiusStrength,
            0.5f32.to_bits(),
        );
        // Step (0, 1, 0); target 0.5 * 16 = 8 away; pull 0.5 * 7 / 8 = 0.4375, times 8 above the
        // radius, so (8 - 1) / 8 = 0.875 instead: x moves by 7.
        e.call(0x00bc_20a0, &args![this, 0u32, 2u32, 1u32, 0u32]);
        assert_eq!(segment_xyz(&e, this, 2), [7.0, 2.0, 0.0]);
        // A weak pull stays: 0.0625 * 7 / 8 = 0.0546875, times 8 is 0.4375, within the radius.
        e.set(
            this,
            BoltShaderProperty::fBoltsRadiusStrength,
            0.0625f32.to_bits(),
        );
        e.call(0x00bc_20a0, &args![this, 0u32, 2u32, 1u32, 0u32]);
        assert_eq!(segment_xyz(&e, this, 2), [0.4375, 2.0, 0.0]);
        // Within the radius: no pull. A downward direction is replaced by the zero point.
        e.set(this, BoltShaderProperty::fBoltsRadius, 10.0f32.to_bits());
        set_segment(&mut e, this, 1, [0.0, -1.0, 0.0]);
        e.call(0x00bc_20a0, &args![this, 0u32, 2u32, 1u32, 0u32]);
        assert_eq!(segment_xyz(&e, this, 2), [0.0, 0.0, 0.0]);
        // The variance scatters x and z by `V * r / 32767 - V / 2` (here r = 0: -0.5).
        e.set(this, BoltShaderProperty::fSegmentVariance, 1.0f32.to_bits());
        e.call(0x00bc_20a0, &args![this, 0u32, 2u32, 1u32, 0u32]);
        assert_eq!(segment_xyz(&e, this, 2), [-0.5, 0.0, -0.5]);
    }

    #[test]
    fn reset_trims_the_bolts_to_the_segment_count() {
        let mut e = engine();
        e.register(BOLT_SEGMENT_COUNT_00BC0B40, |_, _| 8u32.into_ret());
        let this = bolt(&mut e, 3, 4);
        e.set(this, BoltShaderProperty::bWrapped, 1);
        e.set(this, BoltShaderProperty::iBoltStartSegment, 2);
        e.set(this, BoltShaderProperty::bStartFromPoint, 1);
        set_segment(&mut e, this, 1, [1.0, 1.0, 1.0]);
        set_segment(&mut e, this, 2, [2.0, 2.0, 2.0]);
        e.call(0x00bc_2360, &args![this]);
        assert_eq!(e.get(this, BoltShaderProperty::iNumBolts), 2);
        assert_eq!(e.get(this, BoltShaderProperty::iLookBack), 4);
        assert_eq!(e.get(this, BoltShaderProperty::iTotalSegmentCount), 8);
        assert_eq!(e.get(this, BoltShaderProperty::bWrapped), 0);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltStartSegment), 0);
        // The first two segments (one per bolt index) are cleared, the third kept.
        assert_eq!(segment_xyz(&e, this, 1), [0.0; 3]);
        assert_eq!(segment_xyz(&e, this, 2), [2.0; 3]);
        // Not from a point: `radius * 0 - radius * 0.5`.
        e.set(this, BoltShaderProperty::bStartFromPoint, 0);
        e.call(0x00bc_2360, &args![this]);
        assert_eq!(segment_xyz(&e, this, 1), [-5.0; 3]);
        e.set(this, BoltShaderProperty::iSegmentsPerBolt, 3);
        e.call(0x00bc_2360, &args![this]);
        assert_eq!(e.get(this, BoltShaderProperty::iLookBack), 3);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_on_request() {
        let mut e = engine();
        for address in [
            MEMORY_MANAGER_DEALLOCATE,
            BOLT_RELEASE_00BC0C50,
            SHADER_PROPERTY_DESTRUCT,
            NI_FREE_SIZED,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.mem.map(INSTANCE_COUNT, 4);
        e.mem.set_u32(INSTANCE_COUNT, 5);
        let this = bolt(&mut e, 1, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_2510, &args![this, 0u32]).u32(), this.addr());
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
        e.call(0x00bc_2510, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_FREE_SIZED), vec![vec![this.addr(), 0x190]]);
    }

    #[test]
    fn get_render_passes_adds_the_bolt_pass_once() {
        let mut e = engine();
        e.register(CHECK_CREATE_RENDER_PASS_ARRAY, |_, _| Ret::default());
        e.register(RENDER_PASS_ARRAY_ADD, |e, a| {
            let pass = e.mem.alloc(0x10);
            let count = e.mem.u32(a[0] + 0x10);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * count, pass);
            e.mem.set_u32(a[0] + 0x10, count + 1);
            Ret::default()
        });
        let this = bolt(&mut e, 1, 2);
        let list = e.mem.alloc(0x14);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(list + 4, buffer);
        e.mem.set_u32(this.addr() + 0x3c, list);
        e.call_log = Some(vec![]);
        let words = args![this, 0x5000u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        assert_eq!(e.call(0x00bc_2540, &words).u32(), list);
        assert_eq!(
            calls_to(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![list, 0x5000, 0x242, 1, 0, 0, 0, 0, 0]]
        );
        assert_eq!(e.mem.u8(e.mem.u32(buffer) + 6), 1);
        assert_eq!(e.call(0x00bc_2540, &words).u32(), list);
        assert_eq!(calls_to(&e, RENDER_PASS_ARRAY_ADD).len(), 1);
    }

    #[test]
    fn growth_moves_the_ring_and_places_each_new_end() {
        let mut e = engine();
        let this = bolt(&mut e, 1, 4);
        e.set(this, BoltShaderProperty::fBoltsRadius, 0.0f32.to_bits());
        e.set(this, BoltShaderProperty::iBoltStartSegment, 0);
        e.set(this, BoltShaderProperty::iBoltEndSegment, 3);
        set_segment(&mut e, this, 2, [0.0, 2.0, 0.0]);
        set_segment(&mut e, this, 3, [0.0, 3.0, 0.0]);
        // The end wraps to 0 and pushes the start to 1: wrapped; segment 0 steps on from 3
        // (the one before, around the ring) away from 2 (two back, around the ring).
        e.call(0x00bc_2590, &args![this, 1u32]);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltEndSegment), 0);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltStartSegment), 1);
        assert_eq!(e.get(this, BoltShaderProperty::bWrapped), 1);
        assert_eq!(segment_xyz(&e, this, 0), [0.0, 4.0, 0.0]);
        // A count of 0 or below does nothing.
        e.call(0x00bc_2590, &args![this, 0u32]);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltEndSegment), 0);
    }

    #[test]
    fn distance_growth_adds_whole_segments_and_sets_the_short_distance() {
        let mut e = engine();
        let this = bolt(&mut e, 1, 8);
        e.set(this, BoltShaderProperty::fBoltsRadius, 0.0f32.to_bits());
        e.set(this, BoltShaderProperty::fTimer, 1.0f32.to_bits());
        e.set(this, BoltShaderProperty::fDistanceLong, 1.0f32.to_bits());
        // 3.5 - 1 over a length of 1: ceil 3 segments; the long distance becomes 4.
        e.call(0x00bc_2660, &args![this, 0.5f32, 3.5f32]);
        assert_eq!(float(&e, this, BoltShaderProperty::fTimer), 1.5);
        assert_eq!(float(&e, this, BoltShaderProperty::fDistanceLong), 4.0);
        assert_eq!(e.get(this, BoltShaderProperty::iTotalAddedSegments), 3);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltEndSegment), 3);
        // Short: 4 less min(4, 8 * 1) = 0.
        assert_eq!(float(&e, this, BoltShaderProperty::fDistanceShort), 0.0);
        // Longer than the ring: 12 - 8 = 4.
        e.set(this, BoltShaderProperty::fDistanceLong, 12.0f32.to_bits());
        e.call(0x00bc_2660, &args![this, 0.0f32, 12.0f32]);
        assert_eq!(float(&e, this, BoltShaderProperty::fDistanceShort), 4.0);
    }

    #[test]
    fn update_moves_then_grows_or_cuts_to_the_wanted_distance() {
        let mut e = engine();
        let this = bolt(&mut e, 1, 8);
        e.set(this, BoltShaderProperty::fBoltsRadius, 0.0f32.to_bits());
        e.set(this, BoltShaderProperty::iBoltEndSegment, 1);
        set_segment(&mut e, this, 0, [0.0, 0.0, 0.0]);
        set_segment(&mut e, this, 1, [0.0, 1.0, 0.0]);
        // The step is cut to 0.1 and moves the live segments by 0.1 * 2 = 0.2; the end at
        // 1.2 is short of 3.5 by 2.3: two segments grow.
        e.call(0x00bc_2740, &args![this, 0.5f32, 3.5f32, 2.0f32, 0u32]);
        assert_eq!(segment_xyz(&e, this, 0)[1], 0.2);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltEndSegment), 3);
        assert_eq!(float(&e, this, BoltShaderProperty::fDistanceLong), 3.2);
        assert_eq!(float(&e, this, BoltShaderProperty::fDistanceShort), 0.2);
        // Wanted 1.0 (the given 0.5 is below a length): 3.2 is past 1 + 1 by 1.2: one segment
        // is cut back.
        e.call(0x00bc_2740, &args![this, 0.0f32, 0.5f32, 0.0f32, 0u32]);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltEndSegment), 2);
        // A start at or below -length drops segments at the front.
        set_segment(&mut e, this, 0, [0.0, -2.5, 0.0]);
        e.call(0x00bc_2740, &args![this, 0.0f32, 2.5f32, 0.0f32, 0u32]);
        assert_eq!(e.get(this, BoltShaderProperty::iBoltStartSegment), 2);
    }

    #[test]
    fn dispatch_picks_growth_or_the_timed_drop_by_the_flag() {
        let mut e = engine();
        let this = bolt(&mut e, 1, 8);
        e.set(this, BoltShaderProperty::fBoltsRadius, 0.0f32.to_bits());
        e.set(this, BoltShaderProperty::fTimer, 1.0f32.to_bits());
        e.call(0x00bc_28b0, &args![this, 0.5f32, 0.0f32, 9.0f32, 0x100u32]);
        assert_eq!(float(&e, this, BoltShaderProperty::fTimer), 1.5);
        e.set(this, BoltShaderProperty::iBoltEndSegment, 7);
        e.call(0x00bc_28b0, &args![this, 1.0f32, 0.0f32, 2.0f32, 1u32]);
        assert_eq!(float(&e, this, BoltShaderProperty::fTimer), 1.5);
        assert_eq!(float(&e, this, BoltShaderProperty::fDistanceShort), 2.0);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = engine();
        e.mem.map(BOLT_SHADER_PROPERTY_RTTI, 4);
        e.mem.set_u32(BOLT_SHADER_PROPERTY_RTTI, 0x0109_0000);
        e.register(SHADER_PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 4);
        let this = bolt(&mut e, 1, 2);
        e.call_log = Some(vec![]);
        e.call(0x00bc_28f0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(e.mem.u32(e.mem.u32(strings + 4)), 0x0109_0001);
    }
}
