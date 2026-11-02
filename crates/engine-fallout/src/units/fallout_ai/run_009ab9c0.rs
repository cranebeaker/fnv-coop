//! `fallout/ai/run_009ab9c0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009c5870` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const FORWARD_TARGET_009C5870: u32 = 0x009c_5870;

// Translated from 009abef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 0108c448, 0108eae8, 0108f378 and 0108f6f8):
/// forwards `this` and one pointer to `009c5870` (`projectile.cpp` per the engine map), which
/// calls `00932d60` and a virtual call on the word, then walks the list at `this+0x88`.
/// Returns nothing: its list loop ends with no value.
pub fn fn_009abef0(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(FORWARD_TARGET_009C5870, &args![this, argument]);
}

/// `009c58e0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const FORWARD_TARGET_009C58E0: u32 = 0x009c_58e0;

// Translated from 009abf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 0108c434 and 0108ead4): forwards `this` and one
/// pointer to `009c58e0` (`projectile.cpp` per the engine map), which calls `00932f60` and
/// fills fields from the list at `this+0x88`. Returns nothing: it ends with a conditional
/// virtual call and no result.
pub fn fn_009abf10(e: &mut Engine, this: Ptr, argument: Ptr) {
    e.call(FORWARD_TARGET_009C58E0, &args![this, argument]);
}

/// `004181e0` (unnamed, `extradatalist.cpp` per the engine map): `thiscall`, a word.
const SHOOTER_OF_004181E0: u32 = 0x0041_81e0;
/// `009c2c80` (unnamed, `projectile.cpp`): `cdecl`, four words; a `bool`.
const IMPACT_ALLOWED: u32 = 0x009c_2c80;
/// `00444d00` (unnamed, `modelloader.cpp`): `thiscall`, one word (`RET 4`); a `bool` (a
/// flag test, given 0x20).
const HAS_FLAG: u32 = 0x0044_4d00;
/// `Projectile::AddImpact` (`009bf5a0`, Xbox PDB): `thiscall`, six words (`RET 0x18`).
const ADD_IMPACT: u32 = 0x009b_f5a0;
/// `00470470` (unnamed): `thiscall`, no stack words; called on `+0x88` when the byte at
/// `+0x90` is set.
const RELEASE_0X88: u32 = 0x0047_0470;
/// `00439ef0` (unnamed, `multiboundmarkerdata.cpp`): `thiscall`, two words (`RET 8`): an
/// out point and a word; returns the point.
const POINT_FROM: u32 = 0x0043_9ef0;
/// `00457990` (unnamed, `tes.cpp`): `thiscall` on a point; a float in `ST0`.
const POINT_LENGTH: u32 = 0x0045_7990;
/// `009795c0` (unnamed): `thiscall`, one float (`RET 4`).
const SET_LENGTH: u32 = 0x0097_95c0;

/// `00537b10` (unnamed, `tesland.cpp`): `thiscall`; the node's local transform (its scale
/// float at `+0xc`).
const LOCAL_TRANSFORM: u32 = 0x0053_7b10;
/// `00439180` (unnamed, `multiboundmarkerdata.cpp`): `thiscall`, one float (`RET 4`).
const SCALE_BY: u32 = 0x0043_9180;
/// `005495f0` (unnamed, `tesobjectcell.cpp`): `thiscall`; the geometry's data.
const GEOMETRY_DATA: u32 = 0x0054_95f0;
/// `0043b300` (unnamed, `modelloader.cpp`): `cdecl` (an `NiRTTI`, an object); whether the
/// object is of that class.
const IS_KIND_OF: u32 = 0x0043_b300;
/// The `NiRTTI` objects of `NiPSysData` (`0x01202558`), `NiParticleSystem`
/// (`0x012024e0`), `NiPSysCylinderEmitter` (`0x01202508`) and `NiGeometry` (`0x011f4acc`),
/// by the names their static initializers give them.
const RTTI_NI_PSYS_DATA: u32 = 0x0120_2558;
const RTTI_NI_PARTICLE_SYSTEM: u32 = 0x0120_24e0;
const RTTI_NI_PSYS_CYLINDER_EMITTER: u32 = 0x0120_2508;
const RTTI_NI_GEOMETRY: u32 = 0x011f_4acc;
/// `004d7d50` / `004d7d70` (unnamed): `thiscall`; the particle system's modifier count, and
/// modifier `index` (`RET 4`).
const MODIFIER_COUNT: u32 = 0x004d_7d50;
const MODIFIER_AT: u32 = 0x004d_7d70;
/// `00639b20` (unnamed, `precipitation.cpp`): `thiscall`; the emitter's float in `ST0`.
const EMITTER_RADIUS: u32 = 0x0063_9b20;
/// `004ee4b0` (unnamed, `bgsdecalnode.cpp`): `thiscall`, one float (`RET 4`).
const SET_EMITTER_RADIUS: u32 = 0x004e_e4b0;
/// `004a8a90` (unnamed): `thiscall`; an object of the data with a slot `+0x8c` test.
const DATA_OWNER: u32 = 0x004a_8a90;
/// `00534550` (unnamed, `tesland.cpp`): constructs the 0x14-byte stack local (`ECX` only).
const LOCK_CONSTRUCT: u32 = 0x0053_4550;
/// `00a67360` / `00a67410` / `00a673d0` (unnamed, `nigeometrydata.cpp`): `thiscall`; a
/// lock test (one word, `RET 4`), fills the local (one word, `RET 4`), and unlocks.
const DATA_LOCK: u32 = 0x00a6_7360;
const DATA_FILL: u32 = 0x00a6_7410;
const DATA_UNLOCK: u32 = 0x00a6_73d0;
/// `0047d370` (unnamed, `tesactorbasedata.cpp`): `thiscall`; a 16-bit count.
const VERTEX_COUNT: u32 = 0x0047_d370;
/// The three words copied into the point before each vertex is read (`0x011f426c`).
const ZERO_POINT: u32 = 0x011f_426c;
/// `00534570` / `0065b390` (unnamed): `thiscall`, two words (`RET 8`): read and write
/// point `index` of the local.
const READ_POINT: u32 = 0x0053_4570;
const WRITE_POINT: u32 = 0x0065_b390;
/// `007af430` / `0055b980` (named `BGSSaveFormBuffer::GetForm` / unnamed; folded
/// getters): `thiscall`; two arrays of the data (12-byte and 8-byte elements).
const FIRST_ARRAY: u32 = 0x007a_f430;
const SECOND_ARRAY: u32 = 0x0055_b980;
/// `004a8ae0` (named `TESActorBaseData::GetFatigue`; a folded getter): `thiscall`; a
/// 16-bit count.
const ARRAY_LENGTH: u32 = 0x004a_8ae0;
/// `0043c4b0` (unnamed, `modelloader.cpp`): no words; a word for the node's slot `+0xd8`.
const UPDATE_ARGUMENT: u32 = 0x0043_c4b0;

/// `0043c490` (unnamed, `modelloader.cpp`): `thiscall`, returns `this + 0x58` (the local
/// translation); `00440460` sets it from a point (`RET 4`).
const LOCAL_TRANSLATE: u32 = 0x0043_c490;
const SET_LOCAL_TRANSLATE: u32 = 0x0044_0460;
/// `0.0` (`double`, `0x01012060`).
const ZERO: u32 = 0x0101_2060;
/// `00979840` (unnamed): `thiscall`, one word (`RET 4`); a float in `ST0`.
const GEOMETRY_SIZE: u32 = 0x0097_9840;
/// `0043b480` / `0043b4a0` (unnamed, `modelloader.cpp`): `thiscall`; the node's child count
/// and child `index` (`RET 4`).
const CHILD_COUNT: u32 = 0x0043_b480;
const CHILD_AT: u32 = 0x0043_b4a0;
/// `NiGeometryData::SetConsistency` (`00a67050`, Xbox PDB): `thiscall`, one word (`RET 4`).
const SET_CONSISTENCY: u32 = 0x00a6_7050;

// Translated from 009ab9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x18`; a beam projectile's impact): when
/// `009c2c80` allows it (`first`, `fourth`, `this`, `004181e0`'s word), the flag 0x20 is
/// clear (`00444d00`) and `Projectile::AddImpact` takes the six words, it releases `+0x88`
/// (`00470470`) when the byte at `+0x90` is set, clears that byte and sets the length
/// (`009795c0`) to the length (`00457990`) of the point `00439ef0` makes from `second` and
/// slot `+0x1f4`'s word; true then, false otherwise.
pub fn fn_009ab9c0(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> bool {
    let p = this.addr();
    let word = e.call(SHOOTER_OF_004181E0, &args![this]).u32();
    if e.call(IMPACT_ALLOWED, &args![first, fourth, this, word])
        .u8()
        == 0
    {
        return false;
    }
    if e.call(HAS_FLAG, &args![this, 0x20u32]).u8() != 0 {
        return false;
    }
    if e.call(
        ADD_IMPACT,
        &args![this, first, second, third, fourth, fifth, sixth],
    )
    .u8()
        == 0
    {
        return false;
    }
    if e.mem.u8(p + 0x90) != 0 {
        e.call(RELEASE_0X88, &args![p + 0x88]);
    }
    e.mem.set_u8(p + 0x90, 0);
    let target = e.vcall(p, 0x1f4, &[]).u32();
    e.with_stack(0xc, |e, point| {
        let made = e.call(POINT_FROM, &args![second, point, target]).u32();
        let length = e.call(POINT_LENGTH, &args![made]).f64();
        e.call(SET_LENGTH, &args![this, x87_store32(length)]);
    });
    true
}

/// `*address *= factor` as `FLD dword; FMUL dword; FSTP dword`.
fn scale_float(e: &mut Engine, address: u32, factor: f32) {
    let value = x87_load32(e.mem.u32(address));
    let scaled = x87_mul(value, x87_m32(factor.to_bits()));
    e.mem.set_u32(address, x87_store32(scaled));
}

// Translated from 009aba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; `this` unread): scales the geometry
/// `node` by `factor`: its local scale (`00537b10` `+0xc`) and `00439180` on the local
/// transform; then, with geometry data (`005495f0`): for `NiPSysData` (`0043b300`), when
/// the node is an `NiParticleSystem`, the float of each `NiPSysCylinderEmitter` modifier
/// (`00639b20`, `004ee4b0`); otherwise, when the data's owner (`004a8a90`) passes its slot
/// `+0x8c` test and the data locks (`00a67360` with 1), the second float of every point
/// (`00534570`, `0065b390`, starting from the words at `0x011f426c`; then `00a673d0`);
/// otherwise, with the data's first array (`007af430`), the float at `+4` of each of its
/// 12-byte and 8-byte elements (`0055b980`), then the node's slot `+0xd8` with
/// `0043c4b0`'s word.
pub fn fn_009aba90(e: &mut Engine, _this: Ptr, node: u32, factor: f32) {
    if node == 0 {
        return;
    }
    let transform = e.call(LOCAL_TRANSFORM, &args![node]).u32();
    scale_float(e, transform + 0xc, factor);
    let transform = e.call(LOCAL_TRANSFORM, &args![node]).u32();
    e.call(
        SCALE_BY,
        &args![transform, x87_quiet_bits(factor.to_bits())],
    );
    let data = e.call(GEOMETRY_DATA, &args![node]).u32();
    if data == 0 {
        return;
    }
    if e.call(IS_KIND_OF, &args![RTTI_NI_PSYS_DATA, data]).u8() != 0 {
        if e.call(IS_KIND_OF, &args![RTTI_NI_PARTICLE_SYSTEM, node])
            .u8()
            != 0
        {
            let mut index = 0u32;
            while index < e.call(MODIFIER_COUNT, &args![node]).u32() {
                let modifier = e.call(MODIFIER_AT, &args![node, index]).u32();
                if e.call(IS_KIND_OF, &args![RTTI_NI_PSYS_CYLINDER_EMITTER, modifier])
                    .u8()
                    != 0
                {
                    let radius = e.call(EMITTER_RADIUS, &args![modifier]).f64();
                    let scaled = x87_store32(x87_mul(radius, x87_m32(factor.to_bits())));
                    e.call(SET_EMITTER_RADIUS, &args![modifier, x87_quiet_bits(scaled)]);
                }
                index = index.wrapping_add(1);
            }
        }
        return;
    }
    let owner_passes = if e.call(DATA_OWNER, &args![data]).u32() != 0 {
        let owner = e.call(DATA_OWNER, &args![data]).u32();
        e.vcall(owner, 0x8c, &[]).u8() != 0
    } else {
        false
    };
    if owner_passes {
        e.with_stack(0x20, |e, frame| {
            let lock = frame.addr();
            let point = frame.addr() + 0x14;
            e.call(LOCK_CONSTRUCT, &args![lock]);
            let owner = e.call(DATA_OWNER, &args![data]).u32();
            if e.call(DATA_LOCK, &args![data, 1u32]).u8() != 0 {
                e.call(DATA_FILL, &args![data, lock]);
                let mut index = 0i32;
                while index < i32::from(e.call(VERTEX_COUNT, &args![owner]).u16()) {
                    for word in 0..3 {
                        let value = e.mem.u32(ZERO_POINT + 4 * word);
                        e.mem.set_u32(point + 4 * word, value);
                    }
                    e.call(READ_POINT, &args![lock, index as u32, point]);
                    scale_float(e, point + 4, factor);
                    e.call(WRITE_POINT, &args![lock, index as u32, point]);
                    index = index.wrapping_add(1);
                }
                e.call(DATA_UNLOCK, &args![data]);
            }
        });
        return;
    }
    if e.call(FIRST_ARRAY, &args![data]).u32() == 0 {
        return;
    }
    let mut index = 0i32;
    while index < i32::from(e.call(ARRAY_LENGTH, &args![data]).u16()) {
        let first = e.call(FIRST_ARRAY, &args![data]).u32();
        let element = first.wrapping_add((index as u32).wrapping_mul(0xc)) + 0x4;
        scale_float(e, element, factor);
        let second = e.call(SECOND_ARRAY, &args![data]).u32();
        let element = second.wrapping_add((index as u32).wrapping_mul(8)) + 0x4;
        scale_float(e, element, factor);
        index = index.wrapping_add(1);
    }
    let argument = e.call(UPDATE_ARGUMENT, &[]).u32();
    e.vcall(node, 0xd8, &args![argument]);
}

// Translated from 009abd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): fits `node` and its children to
/// `length` over `size`. With a `size` above 0.0 the node's local translation
/// (`0043c490`) is scaled (`00439180`) by `length / size` and stored back (`00440460`).
/// An `NiGeometry` (`0043b300`) is then scaled (`fn_009aba90`) by `length` over `size`
/// (or, when `size` is not above 0.0, over `00979840`'s size of it) unless that is 0.0;
/// any other node passes `length` and `size` on to each child of its slot `+0xc` node
/// (`0043b480`, `0043b4a0`).
pub fn fn_009abd00(e: &mut Engine, this: Ptr, node: u32, length: f32, size: f32) {
    if node == 0 {
        return;
    }
    let zero = f64::from_bits(e.mem.u64(ZERO));
    let size_value = x87_load32(size.to_bits());
    if size_value > zero {
        let ratio = x87_store32(x87_div(
            x87_load32(length.to_bits()),
            x87_m32(size.to_bits()),
        ));
        let translate = e.call(LOCAL_TRANSLATE, &args![node]).u32();
        let words = e.mem.bytes(translate, 0xc);
        e.with_stack(0xc, |e, point| {
            e.mem.write(point.addr(), &words);
            e.call(SCALE_BY, &args![point, x87_quiet_bits(ratio)]);
            e.call(SET_LOCAL_TRANSLATE, &args![node, point]);
        });
    }
    if e.call(IS_KIND_OF, &args![RTTI_NI_GEOMETRY, node]).u8() != 0 {
        let divisor = if x87_load32(size.to_bits()) > zero {
            x87_quiet_bits(size.to_bits())
        } else {
            let value = e.call(GEOMETRY_SIZE, &args![this, node]).f64();
            x87_store32(value)
        };
        let divisor = x87_quiet_bits(divisor);
        // `TEST AH,0x44; JNP`: skipped only when C3 alone is set (equal to 0.0); an
        // unordered compare sets C2 and C3 and goes on.
        if x87_load32(divisor) != zero {
            let factor = x87_store32(x87_div(x87_load32(length.to_bits()), x87_m32(divisor)));
            fn_009aba90(e, this, node, f32::from_bits(x87_quiet_bits(factor)));
        }
        return;
    }
    let children = e.vcall(node, 0xc, &[]).u32();
    if children == 0 {
        return;
    }
    let mut index = 0u32;
    while index < e.call(CHILD_COUNT, &args![children]).u32() {
        let child = e.call(CHILD_AT, &args![children, index]).u32();
        fn_009abd00(
            e,
            this,
            child,
            f32::from_bits(x87_quiet_bits(length.to_bits())),
            f32::from_bits(x87_quiet_bits(size.to_bits())),
        );
        index = index.wrapping_add(1);
    }
}

// Translated from 009abe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): marks the geometry data of `node` and
/// of every geometry below it changed: for an `NiGeometry` (`0043b300`) its data
/// (`005495f0`) gets `NiGeometryData::SetConsistency(0)`; any other node passes each child
/// of its slot `+0xc` node on (`0043b480`, `0043b4a0`).
pub fn fn_009abe40(e: &mut Engine, this: Ptr, node: u32) {
    if node == 0 {
        return;
    }
    if e.call(IS_KIND_OF, &args![RTTI_NI_GEOMETRY, node]).u8() != 0 {
        let data = e.call(GEOMETRY_DATA, &args![node]).u32();
        if data != 0 {
            e.call(SET_CONSISTENCY, &args![data, 0u32]);
        }
        return;
    }
    let children = e.vcall(node, 0xc, &[]).u32();
    if children == 0 {
        return;
    }
    let mut index = 0u32;
    while index < e.call(CHILD_COUNT, &args![children]).u32() {
        let child = e.call(CHILD_AT, &args![children, index]).u32();
        fn_009abe40(e, this, child);
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_bef0, fn_009abef0(Ptr, Ptr)),
        entry!(0x009a_bf10, fn_009abf10(Ptr, Ptr)),
        entry!(
            0x009a_b9c0,
            fn_009ab9c0(Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x009a_ba90, fn_009aba90(Ptr, u32, f32)),
        entry!(0x009a_bd00, fn_009abd00(Ptr, u32, f32, f32)),
        entry!(0x009a_be40, fn_009abe40(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_009abef0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(FORWARD_TARGET_009C5870, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009a_bef0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == FORWARD_TARGET_009C5870)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_009abf10_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(FORWARD_TARGET_009C58E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009a_bf10, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == FORWARD_TARGET_009C58E0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod beam_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn impact_doubles(e: &mut Engine) {
        e.put_vtable(0x7000, &vec![0x7100; 0x1f8 / 4]);
        e.mem.set_u32(0x7000 + 0x1f4, 0x71f4);
        e.register(0x71f4, |_, _| ret(0x4444));
        e.register(SHOOTER_OF_004181E0, |_, _| ret(0x55));
        e.register(IMPACT_ALLOWED, |e, a| ret(e.mem.u32(a[2] + 0x98)));
        e.register(HAS_FLAG, |_, _| ret(0));
        e.register(ADD_IMPACT, |_, _| ret(1));
        e.register(RELEASE_0X88, |_, _| Ret::default());
        e.register(POINT_FROM, |_, a| ret(a[1]));
        e.register(POINT_LENGTH, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.register(SET_LENGTH, |_, _| Ret::default());
    }

    #[test]
    fn fn_009ab9c0_adds_the_impact_and_sets_the_length() {
        let mut e = Engine::new();
        impact_doubles(&mut e);
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this, 0x7000);
        e.mem.set_u32(this + 0x98, 1);
        e.mem.set_u8(this + 0x90, 1);
        e.call_log = Some(vec![]);
        let words = args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32];
        assert!(e.call(0x009a_b9c0, &words).bool());
        assert_eq!(calls(&e, IMPACT_ALLOWED), vec![vec![1, 4, this, 0x55]]);
        assert_eq!(calls(&e, ADD_IMPACT), vec![vec![this, 1, 2, 3, 4, 5, 6]]);
        assert_eq!(calls(&e, RELEASE_0X88), vec![vec![this + 0x88]]);
        assert_eq!(e.mem.u8(this + 0x90), 0);
        let from = calls(&e, POINT_FROM);
        assert_eq!((from[0][0], from[0][2]), (2, 0x4444));
        assert_eq!(calls(&e, SET_LENGTH), vec![vec![this, 2.5f32.to_bits()]]);
        // Not allowed, or flagged: false and nothing added.
        e.mem.set_u32(this + 0x98, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x009a_b9c0, &words).bool());
        assert!(calls(&e, ADD_IMPACT).is_empty());
        e.mem.set_u32(this + 0x98, 1);
        e.register(HAS_FLAG, |_, _| ret(1));
        assert!(!e.call(0x009a_b9c0, &words).bool());
    }

    /// The doubles answer from the node's words: local transform at `+0x40` (its scale at
    /// `+0x4c`), data at `+0x80`, the kind tests from a class word at `+0x84`.
    fn scale_doubles(e: &mut Engine) {
        e.register(LOCAL_TRANSFORM, |_, a| ret(a[0] + 0x40));
        e.register(SCALE_BY, |_, _| Ret::default());
        e.register(GEOMETRY_DATA, |e, a| ret(e.mem.u32(a[0] + 0x80)));
        e.register(IS_KIND_OF, |e, a| {
            ret((e.mem.u32(a[1] + 0x84) == a[0]) as u32)
        });
    }

    fn object(e: &mut Engine, class: u32) -> u32 {
        let p = e.mem.alloc(0x100);
        e.mem.set_u32(p + 0x84, class);
        p
    }

    #[test]
    fn fn_009aba90_scales_the_emitters_of_a_particle_system() {
        let mut e = Engine::new();
        scale_doubles(&mut e);
        let node = object(&mut e, RTTI_NI_PARTICLE_SYSTEM);
        let data = object(&mut e, RTTI_NI_PSYS_DATA);
        e.mem.set_u32(node + 0x80, data);
        e.mem.set_f32(node + 0x4c, 2.0);
        let emitter = object(&mut e, RTTI_NI_PSYS_CYLINDER_EMITTER);
        let other = object(&mut e, 0);
        e.mem.set_u32(node + 0x90, emitter);
        e.mem.set_u32(node + 0x94, other);
        e.register(MODIFIER_COUNT, |_, _| ret(2));
        e.register(MODIFIER_AT, |e, a| ret(e.mem.u32(a[0] + 0x90 + 4 * a[1])));
        e.register(EMITTER_RADIUS, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        e.register(SET_EMITTER_RADIUS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009a_ba90, &args![0u32, node, 1.5f32]);
        assert_eq!(e.mem.f32(node + 0x4c), 3.0);
        assert_eq!(
            calls(&e, SCALE_BY),
            vec![vec![node + 0x40, 1.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, SET_EMITTER_RADIUS),
            vec![vec![emitter, 6.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_009aba90_scales_locked_points() {
        let mut e = Engine::new();
        scale_doubles(&mut e);
        let node = object(&mut e, RTTI_NI_GEOMETRY);
        let data = object(&mut e, 0);
        e.mem.set_u32(node + 0x80, data);
        e.put_vtable(0x7000, &vec![0x7100; 0x90 / 4]);
        e.mem.set_u32(0x7000 + 0x8c, 0x718c);
        e.register(0x718c, |_, _| ret(1));
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, 0x7000);
        e.mem.set_u32(data + 0x88, owner);
        e.register(DATA_OWNER, |e, a| ret(e.mem.u32(a[0] + 0x88)));
        e.register(LOCK_CONSTRUCT, |_, _| Ret::default());
        e.register(DATA_LOCK, |_, _| ret(1));
        e.register(DATA_FILL, |_, _| Ret::default());
        e.register(DATA_UNLOCK, |_, _| Ret::default());
        e.register(VERTEX_COUNT, |_, _| ret(0x1_0002));
        e.map(ZERO_POINT & !0xfff, 0x1000);
        // Each read gives the point (index, 2 + index, 0); the write records the y.
        e.register(READ_POINT, |e, a| {
            e.mem.set_f32(a[2], a[1] as f32);
            e.mem.set_f32(a[2] + 4, 2.0 + a[1] as f32);
            Ret::default()
        });
        e.map(0x7e00, 0x100);
        e.register(WRITE_POINT, |e, a| {
            let y = e.mem.u32(a[2] + 4);
            e.mem.set_u32(0x7e00 + 4 * a[1], y);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009a_ba90, &args![0u32, node, 0.5f32]);
        assert_eq!(e.mem.f32(0x7e00), 1.0);
        assert_eq!(e.mem.f32(0x7e04), 1.5);
        assert_eq!(calls(&e, WRITE_POINT).len(), 2);
        assert_eq!(calls(&e, DATA_UNLOCK), vec![vec![data]]);
        assert_eq!(calls(&e, DATA_LOCK), vec![vec![data, 1]]);
    }

    #[test]
    fn fn_009aba90_scales_the_arrays_otherwise() {
        let mut e = Engine::new();
        scale_doubles(&mut e);
        let node = object(&mut e, RTTI_NI_GEOMETRY);
        let data = object(&mut e, 0);
        e.mem.set_u32(node + 0x80, data);
        e.register(DATA_OWNER, |_, _| ret(0));
        let first = e.mem.alloc(0x18);
        let second = e.mem.alloc(0x10);
        e.mem.set_f32(first + 4, 1.0);
        e.mem.set_f32(first + 0x10, 2.0);
        e.mem.set_f32(second + 4, 3.0);
        e.mem.set_f32(second + 0xc, 4.0);
        e.mem.set_u32(data + 0x90, first);
        e.mem.set_u32(data + 0x94, second);
        e.register(FIRST_ARRAY, |e, a| ret(e.mem.u32(a[0] + 0x90)));
        e.register(SECOND_ARRAY, |e, a| ret(e.mem.u32(a[0] + 0x94)));
        e.register(ARRAY_LENGTH, |_, _| ret(2));
        e.register(UPDATE_ARGUMENT, |_, _| ret(0x31));
        e.put_vtable(0x7000, &vec![0x7100; 0xdc / 4]);
        e.mem.set_u32(0x7000 + 0xd8, 0x71d8);
        e.register(0x71d8, |_, _| Ret::default());
        e.mem.set_u32(node, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x009a_ba90, &args![0u32, node, 2.0f32]);
        assert_eq!(e.mem.f32(first + 4), 2.0);
        assert_eq!(e.mem.f32(first + 0x10), 4.0);
        assert_eq!(e.mem.f32(second + 4), 6.0);
        assert_eq!(e.mem.f32(second + 0xc), 8.0);
        assert_eq!(calls(&e, 0x71d8), vec![vec![node, 0x31]]);
        // No geometry data: only the transform is scaled.
        e.mem.set_u32(node + 0x80, 0);
        e.call_log = Some(vec![]);
        e.call(0x009a_ba90, &args![0u32, node, 2.0f32]);
        assert!(calls(&e, IS_KIND_OF).is_empty());
        assert_eq!(calls(&e, SCALE_BY).len(), 1);
    }

    fn tree_doubles(e: &mut Engine) {
        scale_doubles(e);
        e.map(ZERO & !0xfff, 0x1000);
        e.mem.set_u64(ZERO, 0);
        e.put_vtable(0x7000, &[0, 0, 0, 0x700c]);
        e.register(0x700c, |e, a| ret(e.mem.u32(a[0] + 0x88)));
        e.register(CHILD_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(CHILD_AT, |e, a| ret(e.mem.u32(a[0] + 4 + 4 * a[1])));
        e.register(LOCAL_TRANSLATE, |_, a| ret(a[0] + 0x58));
        e.register(SET_LOCAL_TRANSLATE, |_, _| Ret::default());
        e.register(GEOMETRY_SIZE, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
    }

    /// A node with the vtable whose slot `+0xc` gives the children list at `+0x88`.
    fn parent(e: &mut Engine, children: &[u32]) -> u32 {
        let node = object(e, 0);
        e.mem.set_u32(node, 0x7000);
        let list = e.mem.alloc(4 + 4 * children.len() as u32);
        e.mem.set_u32(list, children.len() as u32);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(list + 4 + 4 * i as u32, *c);
        }
        e.mem.set_u32(node + 0x88, list);
        node
    }

    #[test]
    fn fn_009abd00_fits_geometry_below_a_node() {
        let mut e = Engine::new();
        tree_doubles(&mut e);
        let geometry = object(&mut e, RTTI_NI_GEOMETRY);
        let root = parent(&mut e, &[geometry]);
        e.call_log = Some(vec![]);
        e.call(0x009a_bd00, &args![0x9000u32, root, 6.0f32, 2.0f32]);
        // Size above 0: both translations scaled by 3; the geometry scaled by 3.
        let scaled = calls(&e, SCALE_BY);
        assert_eq!(scaled.len(), 3);
        assert!(scaled.iter().all(|c| c[1] == 3.0f32.to_bits()));
        assert_eq!(calls(&e, SET_LOCAL_TRANSLATE).len(), 2);
        assert_eq!(e.mem.f32(geometry + 0x4c), 0.0);
        assert!(calls(&e, GEOMETRY_SIZE).is_empty());
        // Size 0: no translation; the geometry's own size (4.0) gives 1.5.
        e.call_log = Some(vec![]);
        e.call(0x009a_bd00, &args![0x9000u32, root, 6.0f32, 0.0f32]);
        assert_eq!(calls(&e, GEOMETRY_SIZE), vec![vec![0x9000, geometry]]);
        assert_eq!(
            calls(&e, SCALE_BY),
            vec![vec![geometry + 0x40, 1.5f32.to_bits()]]
        );
        assert!(calls(&e, SET_LOCAL_TRANSLATE).is_empty());
    }

    #[test]
    fn fn_009abe40_marks_the_geometry_data() {
        let mut e = Engine::new();
        tree_doubles(&mut e);
        e.register(SET_CONSISTENCY, |_, _| Ret::default());
        let geometry = object(&mut e, RTTI_NI_GEOMETRY);
        let data = object(&mut e, 0);
        e.mem.set_u32(geometry + 0x80, data);
        let bare = object(&mut e, RTTI_NI_GEOMETRY);
        let inner = parent(&mut e, &[geometry, bare]);
        let root = parent(&mut e, &[inner]);
        e.call_log = Some(vec![]);
        e.call(0x009a_be40, &args![0u32, root]);
        assert_eq!(calls(&e, SET_CONSISTENCY), vec![vec![data, 0]]);
        assert_eq!(calls(&e, CHILD_AT).len(), 3);
    }
}
