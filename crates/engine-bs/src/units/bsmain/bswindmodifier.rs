//! `BSMain/bswindmodifier.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};
use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysmodifier::NiPSysModifier;

/// `00c31650` (named `NiPSysModifier::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C31650: u32 = 0x00c3_1650;

// Translated from 00c467f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c1cbc of the table whose `CreateClone` is
/// `BSWindModifier::CreateClone` (010c1cb4), the slot after `LoadBinary` (010c1cb8). Forwards
/// `this` and the stream to `00c31650` (`NiPSysModifier::LinkObject`). Returns nothing: Xbox
/// PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream`
/// reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c467f0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C31650, &args![this, stream]);
}

/// `00c31670` (named `NiPSysModifier::RegisterStreamables` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00C31670: u32 = 0x00c3_1670;

// Translated from 00c46810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c1cc0 of the `BSWindModifier` table, after the
/// `LinkObject` word 010c1cbc. Forwards `this` and the stream to `00c31670`
/// (`NiPSysModifier::RegisterStreamables`). Returns the `bool` in `AL`: Xbox PDB
/// `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z` returns `_N`; `stream` is the
/// `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c46810(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00C31670, &args![this, stream])
        .bool()
}

// Translated from 00c46960 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202e60 without reading the receiver. The map names this body `BSWindModifier::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c46960(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2e60
}

/// `100.0` (`double`, `01017a40`), the rotation-matrix maker `004a0c90`
/// (`thiscall (matrix, angle)`), the point-times-matrix product `004b3ae0`
/// (`cdecl (&result, &point, &matrix)`, answers the result) and the global
/// `NiPoint3` the result goes to (`01202e68`).
const WIND_SCALE: u32 = 0x0101_7a40;
const MAKE_ROTATION: u32 = 0x004a_0c90;
const POINT_TIMES_MATRIX: u32 = 0x004b_3ae0;
const WIND_VECTOR: u32 = 0x0120_2e68;

// Translated from 00c468c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the global wind vector (the engine map has no name; `cdecl
/// (strength, angle)`): the point `(0, strength * 100.0, 0)` (the product
/// an x87 `FMUL` by the `double`, stored as a `float`) times the rotation
/// matrix `004a0c90` makes from `angle`, its three words stored at
/// `01202e68`.
pub fn fn_00c468c0(e: &mut Engine, strength: f32, angle: f32) {
    let scale = f64::from_bits(e.global::<u64>(WIND_SCALE));
    let scaled = x87_store32(x87_mul(x87_load32(strength.to_bits()), scale));
    e.with_stack(0x3c, |e, frame| {
        let point = frame.addr();
        let matrix = point + 0xc;
        let result = point + 0x30;
        e.mem.set_f32(point, 0.0);
        e.mem.set_u32(point + 4, x87_quiet_bits(scaled));
        e.mem.set_f32(point + 8, 0.0);
        e.call(
            MAKE_ROTATION,
            &args![matrix, x87_quiet_bits(angle.to_bits())],
        );
        let product = e
            .call(POINT_TIMES_MATRIX, &args![result, point, matrix])
            .u32();
        for i in 0..3 {
            let value = e.mem.u32(product + 4 * i);
            e.set_global(WIND_VECTOR + 4 * i, value);
        }
    });
}

layout! {
    /// `BSWindModifier` (Xbox PDB), 0x1C bytes: an `NiPSysModifier` (0x18 bytes) and the
    /// strength; the PC code uses the same offsets.
    pub struct BSWindModifier: 0x1C {
        /// `fStrength` (Xbox PDB), as its bit pattern (the code copies it through the x87
        /// stack).
        0x18 fStrength: u32,
    }
}

/// `BSWindModifier`'s virtual table (RTTI `.?AVBSWindModifier@@`).
const BS_WIND_MODIFIER_VTABLE: u32 = 0x010c_1c6c;
/// The size of a `BSWindModifier`.
const BS_WIND_MODIFIER_SIZE: u32 = 0x1c;
/// `NiPSysModifier`'s default constructor (`00c31720`, `thiscall`), its destructor
/// (`00c31750`, `NiPSysModifier::~NiPSysModifier` in the engine map), its member copy
/// (`00c31780`, `thiscall` on the source, `RET 8`: clone, cloning process), `IsEqual`
/// (`00c31830`), `LoadBinary` (`00c318b0`) and `SaveBinary` (`00c31950`), the last three
/// `thiscall`, `RET 4`.
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
/// `NiNew` (`00aa13e0`, `cdecl`: size) and the sized delete (`00aa1460`, `cdecl`: block,
/// size).
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
/// The stream read of one 4-byte value (`00a26df0`, `cdecl`: stream, destination).
const STREAM_LOAD_WORD: u32 = 0x00a2_6df0;
/// `0045bb20` and `0063c8a0` as `BSWindModifier::Update` calls them: the point at `this`
/// times a `float` into a result (`thiscall`, `RET 8`: result address, factor; answers the
/// result) and the point at `this` plus the point given (`thiscall`, `RET 4`).
const POINT_TIMES_SCALAR: u32 = 0x0045_bb20;
const POINT_ADD_TO: u32 = 0x0063_c8a0;
/// `NiParticleInfo` (Xbox PDB): 0x1C bytes, `m_kVelocity` at +0, `m_fLastUpdate` at +0x14.
const PARTICLE_INFO_SIZE: u32 = 0x1c;
const PARTICLE_LAST_UPDATE: u32 = 0x14;
/// `NiPSysModifier::m_bActive` (Xbox PDB) +0x14, written as the byte the caller passes.
const MODIFIER_ACTIVE: u32 = 0x14;

// Translated from 00a3da50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStreamSaveBinary<float>` (engine map; `cdecl`: stream, address of the value): writes
/// the 4 bytes at `value` through the stream's output (`NiStream::m_pkOstr`) as one
/// component of size 4 (the output's write function at +0xC) and advances the output's
/// position (+4) by what it answers. The exe keeps the component size in the stream
/// argument's own stack slot (it overwrites it with 4 after reading it); here it is a stack
/// local. The body writes any 4-byte value: the bone LOD controller saves integers with it.
pub fn fn_00a3da50(e: &mut Engine, stream: Ptr, value: Ptr) {
    let output = Ptr::<NiBinaryStream>::new(e.get(stream.cast::<NiStream>(), NiStream::m_pkOstr));
    // `m_pfnWrite` (`cdecl` `(stream, buffer, size, &component size, component count)`,
    // answers the bytes written).
    let write = e.get(output, NiBinaryStream::m_pfnWrite);
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 4);
        let written = e
            .call(write, &args![output, value, 4u32, sizes, 1u32])
            .u32();
        let position = e
            .get(output, NiBinaryStream::m_uiAbsoluteCurrentPos)
            .wrapping_add(written);
        e.set(output, NiBinaryStream::m_uiAbsoluteCurrentPos, position);
    });
}

// Translated from 00c1f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::SetActive` (engine map; `thiscall`, `RET 4`): stores the low byte of its
/// stack word as `m_bActive` (+0x14).
pub fn ni_p_sys_modifier_set_active(e: &mut Engine, this: Ptr, active: u8) {
    e.mem
        .set_u8(this.addr().wrapping_add(MODIFIER_ACTIVE), active);
}

// Translated from 00c465a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWindModifier`'s constructor (unnamed in the engine map; `thiscall`, plain `RET`):
/// `NiPSysModifier`'s constructor (`00c31720`), the class's virtual table, strength 1.0 and
/// order 4000 (`m_uiOrder`). Answers `this`.
pub fn fn_00c465a0(e: &mut Engine, this: Ptr<BSWindModifier>) -> Ptr<BSWindModifier> {
    e.call(MODIFIER_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), BS_WIND_MODIFIER_VTABLE);
    e.set(this, BSWindModifier::fStrength, 1.0f32.to_bits());
    e.set(
        this.cast::<NiPSysModifier>(),
        NiPSysModifier::m_uiOrder,
        0xfa0,
    );
    this
}

// Translated from 00c465e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWindModifier::Update` (engine map; `thiscall`, `RET 8`: time, particle data). Nothing
/// without active particles (`m_usActive`, read twice). Otherwise for each active particle
/// (`m_pkParticleInfo`, 0x1C bytes each): `(time - m_fLastUpdate) * fStrength` (x87, stored
/// as a `float`) scales the global wind vector `01202e68` into a stack local (`0045bb20`),
/// which is added to the particle's velocity (`0063c8a0`).
pub fn bs_wind_modifier_update(
    e: &mut Engine,
    this: Ptr<BSWindModifier>,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    if e.get(data, NiPSysData::m_usActive) == 0 {
        return;
    }
    let mut particle = e.get(data, NiPSysData::m_pkParticleInfo);
    let mut remaining = u32::from(e.get(data, NiPSysData::m_usActive));
    while remaining != 0 {
        remaining -= 1;
        let last_update = e.mem.u32(particle.wrapping_add(PARTICLE_LAST_UPDATE));
        let strength = e.get(this, BSWindModifier::fStrength);
        let elapsed = x87_sub(x87_load32(time.to_bits()), x87_m32(last_update));
        let factor = x87_store32(x87_mul(elapsed, x87_m32(strength)));
        let pushed = x87_quiet_bits(factor);
        e.with_stack(0xc, |e, scaled| {
            e.call(POINT_TIMES_SCALAR, &args![WIND_VECTOR, scaled, pushed]);
            e.call(POINT_ADD_TO, &args![particle, scaled]);
        });
        particle = particle.wrapping_add(PARTICLE_INFO_SIZE);
    }
}

// Translated from 00c46700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWindModifier::CopyMembers` (unnamed in the engine map; `thiscall` on the source,
/// `RET 8`: clone, cloning process): `NiPSysModifier`'s copy (`00c31780`), then the strength
/// (copied through the x87 stack, so a signalling NaN arrives quiet).
pub fn fn_00c46700(
    e: &mut Engine,
    this: Ptr<BSWindModifier>,
    clone: Ptr<BSWindModifier>,
    cloning: Ptr,
) {
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, cloning]);
    let strength = x87_quiet_bits(e.get(this, BSWindModifier::fStrength));
    e.set(clone, BSWindModifier::fStrength, x87_quiet_bits(strength));
}

// Translated from 00c46670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c46670 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e64f, state variable) is not translated
/// `BSWindModifier::CreateClone` (engine map; `thiscall`, `RET 4`: cloning process): a new
/// 0x1C-byte modifier (`NiNew`, constructed by [`fn_00c465a0`]; null when the allocation
/// fails) filled by [`fn_00c46700`] (also when null). Answers the clone.
pub fn bs_wind_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<BSWindModifier>,
    cloning: Ptr,
) -> Ptr<BSWindModifier> {
    let block = e.call(NI_NEW, &args![BS_WIND_MODIFIER_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::new(0)
    } else {
        fn_00c465a0(e, Ptr::new(block))
    };
    fn_00c46700(e, this, clone, cloning);
    clone
}

// Translated from 00c46740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c46740 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e679, state variable) is not translated
/// The class's create function (unnamed in the engine map; no arguments, plain `RET`): a
/// new 0x1C-byte modifier (`NiNew`, [`fn_00c465a0`]), null when the allocation fails.
pub fn fn_00c46740(e: &mut Engine) -> Ptr<BSWindModifier> {
    let block = e.call(NI_NEW, &args![BS_WIND_MODIFIER_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00c465a0(e, Ptr::new(block))
}

// Translated from 00c467c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWindModifier::LoadBinary` (unnamed in the engine map; `thiscall`, `RET 4`: stream):
/// `NiPSysModifier::LoadBinary` (`00c318b0`), then the strength (`00a26df0`).
pub fn fn_00c467c0(e: &mut Engine, this: Ptr<BSWindModifier>, stream: Ptr) {
    e.call(MODIFIER_LOAD_BINARY, &args![this, stream]);
    e.call(
        STREAM_LOAD_WORD,
        &args![stream, this.addr().wrapping_add(0x18)],
    );
}

// Translated from 00c46830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWindModifier::SaveBinary` (unnamed in the engine map; `thiscall`, `RET 4`: stream):
/// `NiPSysModifier::SaveBinary` (`00c31950`), then the strength ([`fn_00a3da50`]).
pub fn fn_00c46830(e: &mut Engine, this: Ptr<BSWindModifier>, stream: Ptr) {
    e.call(MODIFIER_SAVE_BINARY, &args![this, stream]);
    fn_00a3da50(e, stream, Ptr::new(this.addr().wrapping_add(0x18)));
}

// Translated from 00c46860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWindModifier::IsEqual` (engine map; `thiscall`, `RET 4`: the other object): false when
/// `NiPSysModifier::IsEqual` (`00c31830`) is; otherwise whether both strengths are equal
/// (x87 `FUCOMPP`: a NaN is unequal).
pub fn bs_wind_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<BSWindModifier>,
    other: Ptr<BSWindModifier>,
) -> bool {
    if e.call(MODIFIER_IS_EQUAL, &args![this, other]).u8() == 0 {
        return false;
    }
    let mine = x87_load32(x87_quiet_bits(e.get(this, BSWindModifier::fStrength)));
    let theirs = x87_load32(e.get(other, BSWindModifier::fStrength));
    theirs == mine
}

// Translated from 00c46930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWindModifier::_scalar_deleting_destructor_` (engine map; `thiscall`, `RET 4`):
/// `NiPSysModifier`'s destructor (`00c31750`) and, when bit 0 of `flags` is set, the sized
/// delete of 0x1C bytes. Answers `this`.
pub fn bs_wind_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSWindModifier>,
    flags: u32,
) -> Ptr<BSWindModifier> {
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, BS_WIND_MODIFIER_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3_da50, fn_00a3da50(Ptr, Ptr)),
        entry!(0x00c1_f4e0, ni_p_sys_modifier_set_active(Ptr, u8)),
        entry!(
            0x00c4_65a0,
            fn_00c465a0(Ptr<BSWindModifier>) -> Ptr<BSWindModifier>
        ),
        entry!(
            0x00c4_65e0,
            bs_wind_modifier_update(Ptr<BSWindModifier>, f32, Ptr<NiPSysData>)
        ),
        entry!(
            0x00c4_6670,
            bs_wind_modifier_create_clone(Ptr<BSWindModifier>, Ptr) -> Ptr<BSWindModifier>
        ),
        entry!(
            0x00c4_6700,
            fn_00c46700(Ptr<BSWindModifier>, Ptr<BSWindModifier>, Ptr)
        ),
        entry!(0x00c4_6740, fn_00c46740() -> Ptr<BSWindModifier>),
        entry!(0x00c4_67c0, fn_00c467c0(Ptr<BSWindModifier>, Ptr)),
        entry!(0x00c4_6830, fn_00c46830(Ptr<BSWindModifier>, Ptr)),
        entry!(
            0x00c4_6860,
            bs_wind_modifier_is_equal(Ptr<BSWindModifier>, Ptr<BSWindModifier>) -> bool
        ),
        entry!(
            0x00c4_6930,
            bs_wind_modifier_scalar_deleting_destructor(
                Ptr<BSWindModifier>,
                u32,
            ) -> Ptr<BSWindModifier>
        ),
        entry!(0x00c4_6960, fn_00c46960(Ptr) -> u32),
        entry!(0x00c4_67f0, fn_00c467f0(Ptr, Ptr)),
        entry!(0x00c4_6810, fn_00c46810(Ptr, Ptr) -> bool),
        entry!(0x00c4_68c0, fn_00c468c0(f32, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c467f0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C31650, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_67f0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C31650)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c46810_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00C31670, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c4_6810, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00C31670)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c46960_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_6960, &args![0x1000u32]).u32(), 0x0120_2e60);
        assert_eq!(e.call(0x00c4_6960, &args![0u32]).u32(), 0x0120_2e60);
    }

    #[test]
    fn wind_vector_is_the_rotated_scaled_point() {
        let mut e = Engine::new();
        e.map(WIND_SCALE, 8);
        e.set_global(WIND_SCALE, 100.0f64);
        e.map(WIND_VECTOR, 0xc);
        e.register(MAKE_ROTATION, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // The "product" adds the matrix's first word (the angle) to each
        // coordinate of the point.
        e.register(POINT_TIMES_MATRIX, |e, a| {
            let angle = e.mem.f32(a[2]);
            for i in 0..3 {
                let value = e.mem.f32(a[1] + 4 * i) + angle;
                e.mem.set_f32(a[0] + 4 * i, value);
            }
            a[0].into_ret()
        });
        e.call(0x00c4_68c0, &args![0.25f32, 1.5f32]);
        assert_eq!(e.global::<f32>(WIND_VECTOR), 1.5);
        assert_eq!(e.global::<f32>(WIND_VECTOR + 4), 26.5);
        assert_eq!(e.global::<f32>(WIND_VECTOR + 8), 1.5);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    fn recorder(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn save_binary_of_a_word_writes_four_bytes_and_moves_the_position() {
        let mut e = Engine::new();
        let stream = e.mem.alloc(0x260);
        let output = e.mem.alloc(0x10);
        let value = e.mem.alloc(4);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 4, 10);
        e.mem.set_u32(output + 0xc, 0x00ee_1000);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = seen.clone();
        e.register_double(0x00ee_1000, move |e, a| {
            inner.borrow_mut().push((a.to_vec(), e.mem.u32(a[3])));
            4u32.into_ret()
        });
        e.call(0x00a3_da50, &args![stream, value]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[0], output);
        assert_eq!(seen[0].0[1], value);
        assert_eq!(seen[0].0[2], 4);
        assert_eq!(seen[0].0[4], 1);
        assert_eq!(seen[0].1, 4);
        assert_eq!(e.mem.u32(output + 4), 14);
    }

    #[test]
    fn set_active_stores_the_low_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.call(0x00c1_f4e0, &args![this, 0x101u32]);
        assert_eq!(e.mem.u8(this + 0x14), 1);
        e.call(0x00c1_f4e0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x14), 0);
    }

    #[test]
    fn constructor_sets_vtable_strength_and_order() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        recorder(&mut e, &log, MODIFIER_CONSTRUCTOR, 0);
        let this = e.mem.alloc(0x1c);
        let result = e.call(0x00c4_65a0, &args![this]).u32();
        assert_eq!(result, this);
        assert_eq!(calls_to(&log, MODIFIER_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), BS_WIND_MODIFIER_VTABLE);
        assert_eq!(e.mem.f32(this + 0x18), 1.0);
        assert_eq!(e.mem.u32(this + 0xc), 4000);
    }

    #[test]
    fn update_pushes_wind_onto_every_active_particle() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.mem.set_f32(this + 0x18, 2.0);
        let data = e.mem.alloc(0x7c);
        let particles = e.mem.alloc(0x1c * 2);
        e.mem.set_u32(data + 0x70, particles);
        e.mem.set_f32(particles + 0x14, 1.0);
        e.mem.set_f32(particles + 0x1c + 0x14, 2.5);
        let log: Log = Default::default();
        let scales = log.clone();
        e.register_double(POINT_TIMES_SCALAR, move |e, a| {
            scales.borrow_mut().push((POINT_TIMES_SCALAR, a.to_vec()));
            e.mem.set_u32(a[1], a[2]);
            a[1].into_ret()
        });
        let adds = log.clone();
        e.register_double(POINT_ADD_TO, move |e, a| {
            adds.borrow_mut().push((POINT_ADD_TO, a.to_vec()));
            let v = e.mem.f32(a[0]) + e.mem.f32(a[1]);
            e.mem.set_f32(a[0], v);
            Ret::default()
        });
        // No active particle: nothing happens.
        e.call(0x00c4_65e0, &args![this, 3.0f32, data]);
        assert!(log.borrow().is_empty());
        e.mem.set_u16(data + 0x50, 2);
        e.call(0x00c4_65e0, &args![this, 3.0f32, data]);
        let scaled = calls_to(&log, POINT_TIMES_SCALAR);
        assert_eq!(scaled.len(), 2);
        assert_eq!(scaled[0][0], WIND_VECTOR);
        assert_eq!(f32::from_bits(scaled[0][2]), 4.0);
        assert_eq!(f32::from_bits(scaled[1][2]), 1.0);
        let added = calls_to(&log, POINT_ADD_TO);
        assert_eq!(added[0][0], particles);
        assert_eq!(added[1][0], particles + 0x1c);
        assert_eq!(e.mem.f32(particles), 4.0);
        assert_eq!(e.mem.f32(particles + 0x1c), 1.0);
    }

    #[test]
    fn create_clone_builds_and_copies_even_when_allocation_fails() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        recorder(&mut e, &log, MODIFIER_CONSTRUCTOR, 0);
        recorder(&mut e, &log, MODIFIER_COPY_MEMBERS, 0);
        let block = e.mem.alloc(0x1c);
        recorder(&mut e, &log, NI_NEW, block);
        let this = e.mem.alloc(0x1c);
        e.mem.set_f32(this + 0x18, 7.5);
        let clone = e.call(0x00c4_6670, &args![this, 0x4444u32]).u32();
        assert_eq!(clone, block);
        assert_eq!(calls_to(&log, NI_NEW), vec![vec![0x1c]]);
        assert_eq!(
            calls_to(&log, MODIFIER_COPY_MEMBERS),
            vec![vec![this, block, 0x4444]]
        );
        assert_eq!(e.mem.f32(block + 0x18), 7.5);
        assert_eq!(e.mem.u32(block), BS_WIND_MODIFIER_VTABLE);
        // A failed allocation still runs the copy, on a null clone (which then faults
        // on the strength's store).
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        e.register(MODIFIER_COPY_MEMBERS, |_, a| {
            assert_eq!(a[1], 0);
            Ret::default()
        });
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00c4_6670, &args![this, 0u32])
        }));
        assert!(result.is_err());
    }

    #[test]
    fn create_function_answers_null_or_a_constructed_modifier() {
        let mut e = Engine::new();
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c4_6740, &args![]).u32(), 0);
        let block = e.mem.alloc(0x1c);
        e.register_double(NI_NEW, move |_, _| block.into_ret());
        assert_eq!(e.call(0x00c4_6740, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), BS_WIND_MODIFIER_VTABLE);
    }

    #[test]
    fn copy_members_quiets_a_signalling_strength() {
        let mut e = Engine::new();
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        let clone = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 0x7f80_0001);
        e.call(0x00c4_6700, &args![this, clone, 0u32]);
        assert_eq!(e.mem.u32(clone + 0x18), 0x7fc0_0001);
    }

    #[test]
    fn load_and_save_binary_stream_the_strength_after_the_base() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        recorder(&mut e, &log, MODIFIER_LOAD_BINARY, 0);
        recorder(&mut e, &log, STREAM_LOAD_WORD, 0);
        recorder(&mut e, &log, MODIFIER_SAVE_BINARY, 0);
        let this = e.mem.alloc(0x1c);
        let stream = e.mem.alloc(0x260);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x00ee_2000);
        recorder(&mut e, &log, 0x00ee_2000, 4);
        e.call(0x00c4_67c0, &args![this, stream]);
        assert_eq!(
            calls_to(&log, MODIFIER_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(
            calls_to(&log, STREAM_LOAD_WORD),
            vec![vec![stream, this + 0x18]]
        );
        e.call(0x00c4_6830, &args![this, stream]);
        assert_eq!(
            calls_to(&log, MODIFIER_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls_to(&log, 0x00ee_2000)[0][1], this + 0x18);
    }

    #[test]
    fn is_equal_needs_the_base_and_equal_strengths() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        e.mem.set_f32(this + 0x18, 1.5);
        e.mem.set_f32(other + 0x18, 1.5);
        e.register(MODIFIER_IS_EQUAL, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00c4_6860, &args![this, other]).bool());
        e.register(MODIFIER_IS_EQUAL, |_, _| 1u32.into_ret());
        assert!(e.call(0x00c4_6860, &args![this, other]).bool());
        e.mem.set_f32(other + 0x18, 2.5);
        assert!(!e.call(0x00c4_6860, &args![this, other]).bool());
        e.mem.set_f32(this + 0x18, f32::NAN);
        e.mem.set_f32(other + 0x18, f32::NAN);
        assert!(!e.call(0x00c4_6860, &args![this, other]).bool());
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        recorder(&mut e, &log, MODIFIER_DESTRUCTOR, 0);
        recorder(&mut e, &log, NI_DELETE, 0);
        let this = e.mem.alloc(0x1c);
        assert_eq!(e.call(0x00c4_6930, &args![this, 2u32]).u32(), this);
        assert!(calls_to(&log, NI_DELETE).is_empty());
        e.call(0x00c4_6930, &args![this, 3u32]);
        assert_eq!(calls_to(&log, NI_DELETE), vec![vec![this, 0x1c]]);
        assert_eq!(calls_to(&log, MODIFIER_DESTRUCTOR).len(), 2);
    }
}
