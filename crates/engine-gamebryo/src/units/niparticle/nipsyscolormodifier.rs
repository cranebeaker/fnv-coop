//! `NiParticle/nipsyscolormodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2d170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColorModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2778.
pub fn ni_p_sys_color_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2778
}

use crate::units::nimain::nimainsdm::{assign_ref_counted, interlocked_decrement};
use crate::units::niparticle::nipsysdata::add_viewer_string;

layout! {
    /// `NiPSysColorModifier` (Xbox PDB), 0x24 bytes; the PC code uses the
    /// same offsets. Floats are declared as their bit patterns.
    pub struct NiPSysColorModifier: 0x24 {
        /// `m_spColorData` (Xbox PDB): `NiPointer<NiColorData>`.
        0x18 m_spColorData: u32,
        /// `m_fLoKeyTime` (Xbox PDB).
        0x1C m_fLoKeyTime: u32,
        /// `m_fHiKeyTime` (Xbox PDB).
        0x20 m_fHiKeyTime: u32,
    }
}

/// `NiColorData`'s keys as the PC code reads them: the count (+8), the keys
/// (+0xC), the key type (+0x10) and the key size (+0x14, a byte); a key's
/// time is its first `float`.
const COLOR_DATA_COUNT: u32 = 0x8;
const COLOR_DATA_KEYS: u32 = 0xc;
const COLOR_DATA_TYPE: u32 = 0x10;
const COLOR_DATA_SIZE: u32 = 0x14;
/// The particle data (`NiPSysData`) fields the update reads: the colours
/// (+0x28, 16 bytes each), the active count (+0x50) and the particle infos
/// (+0x70, 0x1c bytes each: age at +0xC, life span at +0x10).
const DATA_COLORS: u32 = 0x28;
const DATA_ACTIVE: u32 = 0x50;
const DATA_INFOS: u32 = 0x70;
const INFO_SIZE: u32 = 0x1c;
const INFO_AGE: u32 = 0xc;
const INFO_LIFE_SPAN: u32 = 0x10;
/// `NiColorKey::GenInterp` (`00a296c0`, Xbox PDB name; `cdecl`: result,
/// time, keys, key type, count, address of the last index, a word whose low
/// byte is the key size; the callee reads that byte only).
const COLOR_KEY_GEN_INTERP: u32 = 0x00a2_96c0;
/// `NiPSysModifier`'s constructor (`00c31720`), destructor body
/// (`00c31750`) and member copy (`00c31780`: clone, process), `LinkObject`
/// (`00c31650`, one word), `IsEqual` (`00c31830`) and `GetViewerStrings`
/// (`00c319e0`) (the last three Xbox PDB names; all `thiscall`).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LINK_OBJECT: u32 = 0x00c3_1650;
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, Xbox PDB name; `thiscall`
/// on the stream, no words).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00aa13e0` (`cdecl`, one word): the object allocation; `00aa1460`
/// (`cdecl`: block, size): the sized delete.
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
/// The vtable the constructors store (`010bffe4`).
const VTABLE: u32 = 0x010b_ffe4;
/// `NiObject`'s `IsEqual` (`+0x5C`) and `GetViewerStrings` (`+0x60`) and
/// `NiRefObject::DeleteThis` (`+4`) slots (Xbox PDB).
const IS_EQUAL_SLOT: u32 = 0x5c;
const GET_VIEWER_STRINGS_SLOT: u32 = 0x60;
const DELETE_THIS_SLOT: u32 = 0x4;
/// `NiGetViewerString` (`00a7d330`, Xbox PDB name; `cdecl`, the text).
const VIEWER_STRING: u32 = 0x00a7_d330;
/// `NiPSysColorModifier::ms_RTTI` (its name is the first word).
const RTTI: u32 = 0x0120_2778;

/// The constructor the stream factory and `CreateClone` inline:
/// `NiPSysModifier`'s constructor, the vtable, no colour data and key times
/// of 0.0.
fn construct(e: &mut Engine, block: u32) -> Ptr<NiPSysColorModifier> {
    type M = NiPSysColorModifier;
    let this = Ptr::<M>::new(block);
    e.call(MODIFIER_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, VTABLE);
    e.set(this, M::m_spColorData, 0);
    e.set(this, M::m_fLoKeyTime, 0.0f32.to_bits());
    e.set(this, M::m_fHiKeyTime, 0.0f32.to_bits());
    this
}

// Translated from 00c2cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColorModifier::Update` (Xbox PDB): with a colour array in the
/// particle data, each active particle's colour becomes the colour keys'
/// value (`NiColorKey::GenInterp`, last index 0) at its age over its life
/// span, raised to the low key time when not above it and lowered to the
/// high key time when not below it (a NaN takes the key time).
pub fn ni_p_sys_color_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysColorModifier>,
    _time: f32,
    data: Ptr,
) {
    type M = NiPSysColorModifier;
    let data = data.addr();
    let mut color = e.mem.u32(data + DATA_COLORS);
    let mut info = e.mem.u32(data + DATA_INFOS);
    if color == 0 {
        return;
    }
    let active = e.mem.u16(data + DATA_ACTIVE);
    let color_data = e.get(this, M::m_spColorData);
    let key_type = e.mem.u32(color_data + COLOR_DATA_TYPE);
    let count = e.mem.u32(color_data + COLOR_DATA_COUNT);
    let size = e.mem.u8(color_data + COLOR_DATA_SIZE);
    let keys = e.mem.u32(color_data + COLOR_DATA_KEYS);
    if active == 0 {
        return;
    }
    e.with_stack(0x18, |e, locals| {
        let last_index = locals.addr();
        let result = locals.addr() + 4;
        for _ in 0..active {
            let age = x87_load32(e.mem.u32(info + INFO_AGE));
            let life_span = x87_m32(e.mem.u32(info + INFO_LIFE_SPAN));
            let mut t = x87_store32(x87_div(age, life_span));
            let low = x87_quiet_bits(e.get(this, M::m_fLoKeyTime));
            if !(x87_load32(low) < x87_load32(t)) {
                t = low;
            }
            let high = x87_quiet_bits(e.get(this, M::m_fHiKeyTime));
            if !(x87_load32(high) > x87_load32(t)) {
                t = high;
            }
            e.mem.set_u32(last_index, 0);
            e.call(
                COLOR_KEY_GEN_INTERP,
                &args![
                    result,
                    x87_quiet_bits(t),
                    keys,
                    key_type,
                    count,
                    last_index,
                    u32::from(size)
                ],
            );
            let words = e.mem.bytes(result, 16);
            e.mem.write(color, &words);
            color = color.wrapping_add(16);
            info = info.wrapping_add(INFO_SIZE);
        }
    });
}

// Translated from 00c2d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColorModifier::IsEqual` (Xbox PDB): `NiPSysModifier`'s, then
/// this one's colour data's `IsEqual` (slot `+0x5C`) given the other's.
pub fn ni_p_sys_color_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysColorModifier>,
    other: Ptr<NiPSysColorModifier>,
) -> bool {
    type M = NiPSysColorModifier;
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let ours = e.get(this, M::m_spColorData);
    let theirs = e.get(other, M::m_spColorData);
    e.vcall(ours, IS_EQUAL_SLOT, &args![theirs]).bool()
}

// Translated from 00c2d100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`SetColorData` by its body): assigns the
/// colour data (the reference-counted assignment inlined), then the key
/// times: 0.0, or the first and the last key's times when the data has
/// keys.
pub fn fn_00c2d100(e: &mut Engine, this: Ptr<NiPSysColorModifier>, color_data: u32) {
    type M = NiPSysColorModifier;
    let slot = this.addr() + M::m_spColorData.off;
    assign_ref_counted(e, slot, color_data);
    e.set(this, M::m_fLoKeyTime, 0.0f32.to_bits());
    e.set(this, M::m_fHiKeyTime, 0.0f32.to_bits());
    let data = e.get(this, M::m_spColorData);
    if data == 0 {
        return;
    }
    let count = e.mem.u32(data + COLOR_DATA_COUNT);
    let size = u32::from(e.mem.u8(data + COLOR_DATA_SIZE));
    let keys = e.mem.u32(data + COLOR_DATA_KEYS);
    if count == 0 {
        return;
    }
    e.set(this, M::m_fLoKeyTime, x87_quiet_bits(e.mem.u32(keys)));
    let last = count.wrapping_sub(1).wrapping_mul(size).wrapping_add(keys);
    e.set(this, M::m_fHiKeyTime, x87_quiet_bits(e.mem.u32(last)));
}

// Translated from 00c2d180 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project: the `NiPSysColorModifier` stream
/// factory (`cdecl`, no arguments; registered under its class name): a
/// 0x24-byte object from `00aa13e0` given `NiPSysModifier`'s constructor,
/// the vtable, no colour data and key times of 0.0; null when the
/// allocation fails.
pub fn fn_00c2d180(e: &mut Engine) -> Ptr<NiPSysColorModifier> {
    let block = e.call(NI_NEW, &args![0x24u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct(e, block)
}

// Translated from 00c2d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColorModifier::LinkObject` (Xbox PDB): `NiPSysModifier`'s, then
/// the colour data from the stream's next link (`GetObjectFromLinkID`)
/// through `00c2d100`.
pub fn ni_p_sys_color_modifier_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysColorModifier>,
    stream: Ptr,
) {
    e.call(MODIFIER_LINK_OBJECT, &args![this, stream]);
    let color_data = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    fn_00c2d100(e, this, color_data);
}

// Translated from 00c2d1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColorModifier::_scalar_deleting_destructor_` (Xbox PDB): the
/// colour data released (`DeleteThis`, slot `+4`, when its count reaches
/// 0), `NiPSysModifier`'s destructor body, then the sized delete of 0x24
/// bytes when bit 0 of `flags` is set. Returns `this`.
pub fn ni_p_sys_color_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPSysColorModifier>,
    flags: u32,
) -> Ptr<NiPSysColorModifier> {
    let color_data = e.get(this, NiPSysColorModifier::m_spColorData);
    if color_data != 0 && interlocked_decrement(e, color_data + 4) == 0 {
        e.vcall(color_data, DELETE_THIS_SLOT, &args![]);
    }
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x24u32]);
    }
    this
}

// Translated from 00c2d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColorModifier::CreateClone` (Xbox PDB): a 0x24-byte object from
/// `00aa13e0` constructed as the stream factory does (null when the
/// allocation fails), `NiPSysModifier`'s member copy into it, then this
/// one's colour data through `00c2d100`. Returns the clone.
pub fn ni_p_sys_color_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysColorModifier>,
    process: Ptr,
) -> Ptr<NiPSysColorModifier> {
    let block = e.call(NI_NEW, &args![0x24u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        construct(e, block)
    };
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let color_data = e.get(this, NiPSysColorModifier::m_spColorData);
    fn_00c2d100(e, clone, color_data);
    clone
}

// Translated from 00c2d2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColorModifier::GetViewerStrings` (Xbox PDB): `NiPSysModifier`'s
/// strings, the class name, then the colour data's strings (slot `+0x60`).
pub fn ni_p_sys_color_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysColorModifier>,
    strings: Ptr,
) {
    e.call(MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings.addr(), string);
    let color_data = e.get(this, NiPSysColorModifier::m_spColorData);
    e.vcall(color_data, GET_VIEWER_STRINGS_SLOT, &args![strings]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2d170, ni_p_sys_color_modifier_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_cfb0,
            ni_p_sys_color_modifier_update(Ptr<NiPSysColorModifier>, f32, Ptr)
        ),
        entry!(
            0x00c2_d0d0,
            ni_p_sys_color_modifier_is_equal(
                Ptr<NiPSysColorModifier>,
                Ptr<NiPSysColorModifier>,
            ) -> bool
        ),
        entry!(0x00c2_d100, fn_00c2d100(Ptr<NiPSysColorModifier>, u32)),
        entry!(0x00c2_d180, fn_00c2d180() -> Ptr<NiPSysColorModifier>),
        entry!(
            0x00c2_d1c0,
            ni_p_sys_color_modifier_link_object(Ptr<NiPSysColorModifier>, Ptr)
        ),
        entry!(
            0x00c2_d1f0,
            ni_p_sys_color_modifier_scalar_deleting_destructor(
                Ptr<NiPSysColorModifier>,
                u32,
            )
                -> Ptr<NiPSysColorModifier>
        ),
        entry!(
            0x00c2_d240,
            ni_p_sys_color_modifier_create_clone(
                Ptr<NiPSysColorModifier>,
                Ptr,
            ) -> Ptr<NiPSysColorModifier>
        ),
        entry!(
            0x00c2_d2a0,
            ni_p_sys_color_modifier_get_viewer_strings(Ptr<NiPSysColorModifier>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_color_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_d170, &args![]).u32(), 0x0120_2778);
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to, quiet};
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::INTERLOCKED_DECREMENT;

    type M = NiPSysColorModifier;

    /// Colour data with `count` keys of `size` bytes whose times are 1.0,
    /// 2.0, ...; its vtable answers `slots`.
    fn color_data(e: &mut Engine, count: u32, size: u8, slots: &[(u32, u32)]) -> u32 {
        let data = viewer_test::object(e, slots);
        let keys = e.mem.alloc(count * u32::from(size) + 4);
        for i in 0..count {
            e.mem.set_f32(keys + i * u32::from(size), i as f32 + 1.0);
        }
        e.mem.set_u32(data + COLOR_DATA_COUNT, count);
        e.mem.set_u32(data + COLOR_DATA_KEYS, keys);
        e.mem.set_u32(data + COLOR_DATA_TYPE, 2);
        e.mem.set_u8(data + COLOR_DATA_SIZE, size);
        e.mem.set_u32(data + 4, 1);
        data
    }

    #[test]
    fn set_color_data_counts_it_and_takes_the_key_times() {
        let mut e = Engine::new();
        test_stream::interlocked(&mut e);
        const DELETE: u32 = 0x0000_c004;
        quiet(&mut e, &[DELETE]);
        let this = e.new_object::<M>();
        let old = color_data(&mut e, 1, 8, &[(0x4, DELETE)]);
        e.set(this, M::m_spColorData, old);
        e.set(this, M::m_fLoKeyTime, 9.0f32.to_bits());
        let new = color_data(&mut e, 3, 0x14, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d100, &args![this, new]);
        assert_eq!(calls_to(&e, DELETE), vec![vec![old]]);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(e.get(this, M::m_spColorData), new);
        assert_eq!(e.get(this, M::m_fLoKeyTime), 1.0f32.to_bits());
        assert_eq!(e.get(this, M::m_fHiKeyTime), 3.0f32.to_bits());
        // The same data again: no counting, the times are taken again.
        e.set(this, M::m_fHiKeyTime, 0);
        e.call(0x00c2_d100, &args![this, new]);
        assert_eq!(e.mem.u32(new + 4), 2);
        assert_eq!(e.get(this, M::m_fHiKeyTime), 3.0f32.to_bits());
        // No keys, then no data: times 0.
        e.mem.set_u32(new + COLOR_DATA_COUNT, 0);
        e.call(0x00c2_d100, &args![this, new]);
        assert_eq!(e.get(this, M::m_fLoKeyTime), 0);
        e.call(0x00c2_d100, &args![this, 0u32]);
        assert_eq!(e.get(this, M::m_spColorData), 0);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(e.get(this, M::m_fHiKeyTime), 0);
    }

    #[test]
    fn update_interpolates_each_colour_at_the_clamped_age() {
        let mut e = Engine::new();
        e.register(COLOR_KEY_GEN_INTERP, |e, a| {
            // The colour is (time, key type, count, size byte).
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[3]);
            e.mem.set_u32(a[0] + 8, a[4]);
            e.mem.set_u32(a[0] + 12, a[6] & 0xff);
            assert_eq!(e.mem.u32(a[5]), 0);
            e.mem.set_u32(a[5], 7);
            Ret::default()
        });
        let this = e.new_object::<M>();
        let keys = color_data(&mut e, 3, 0x14, &[]);
        e.set(this, M::m_spColorData, keys);
        e.set(this, M::m_fLoKeyTime, 0.25f32.to_bits());
        e.set(this, M::m_fHiKeyTime, 0.75f32.to_bits());
        let data = e.mem.alloc(0x80);
        let colors = e.mem.alloc(16 * 4);
        let infos = e.mem.alloc(INFO_SIZE * 4);
        e.mem.set_u32(data + DATA_COLORS, colors);
        e.mem.set_u32(data + DATA_INFOS, infos);
        e.mem.set_u16(data + DATA_ACTIVE, 3);
        for (i, age) in [1.0f32, 3.0, 4.0].iter().enumerate() {
            let info = infos + INFO_SIZE * i as u32;
            e.mem.set_f32(info + INFO_AGE, *age);
            e.mem.set_f32(info + INFO_LIFE_SPAN, 4.0);
        }
        e.call(0x00c2_cfb0, &args![this, 9.0f32, data]);
        for (i, t) in [0.25f32, 0.75, 0.75].iter().enumerate() {
            let color = colors + 16 * i as u32;
            assert_eq!(e.mem.u32(color), t.to_bits());
            assert_eq!(e.mem.u32(color + 4), 2);
            assert_eq!(e.mem.u32(color + 8), 3);
            assert_eq!(e.mem.u32(color + 12), 0x14);
        }
        // Inside the range the age ratio itself; the fourth colour is untouched.
        e.mem.set_f32(infos + INFO_AGE, 2.0);
        e.call(0x00c2_cfb0, &args![this, 9.0f32, data]);
        assert_eq!(e.mem.f32(colors), 0.5);
        assert_eq!(e.mem.u32(colors + 48), 0);
        // No colours: nothing.
        e.mem.set_u32(data + DATA_COLORS, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_cfb0, &args![this, 9.0f32, data]);
        assert!(calls_to(&e, COLOR_KEY_GEN_INTERP).is_empty());
    }

    #[test]
    fn is_equal_asks_the_colour_data() {
        const EQUAL: u32 = 0x0000_c05c;
        let mut e = Engine::new();
        e.register(MODIFIER_IS_EQUAL, |_, _| answer(1));
        e.register(EQUAL, |_, a| answer(u32::from(a[1] == 0x4444)));
        let this = e.new_object::<M>();
        let other = e.new_object::<M>();
        let ours = color_data(&mut e, 0, 4, &[(0x5c, EQUAL)]);
        e.set(this, M::m_spColorData, ours);
        e.set(other, M::m_spColorData, 0x4444);
        let check = |e: &mut Engine| e.call(0x00c2_d0d0, &args![this, other]).bool();
        assert!(check(&mut e));
        e.set(other, M::m_spColorData, 0x4445);
        assert!(!check(&mut e));
        e.register(MODIFIER_IS_EQUAL, |_, _| answer(0));
        e.call_log = Some(vec![]);
        assert!(!check(&mut e));
        assert!(calls_to(&e, EQUAL).is_empty());
    }

    #[test]
    fn factory_and_clone_construct_the_modifier() {
        let mut e = Engine::new();
        test_stream::interlocked(&mut e);
        e.register(NI_NEW, |e, a| answer(e.mem.alloc(a[0])));
        quiet(&mut e, &[MODIFIER_CONSTRUCTOR, MODIFIER_COPY_MEMBERS]);
        e.call_log = Some(vec![]);
        let made = e.call(0x00c2_d180, &args![]).u32();
        assert_eq!(e.mem.u32(made), VTABLE);
        assert_eq!(calls_to(&e, MODIFIER_CONSTRUCTOR), vec![vec![made]]);
        let this = e.new_object::<M>();
        let keys = color_data(&mut e, 2, 0x14, &[]);
        e.set(this, M::m_spColorData, keys);
        let clone = e.call(0x00c2_d240, &args![this, 0x900u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x900]]
        );
        assert_eq!(e.mem.u32(clone + 0x18), keys);
        assert_eq!(e.mem.u32(clone + 0x20), 2.0f32.to_bits());
        assert_eq!(e.mem.u32(keys + 4), 2);
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_d180, &args![]).u32(), 0);
    }

    #[test]
    fn link_object_takes_the_linked_colour_data() {
        let mut e = Engine::new();
        test_stream::interlocked(&mut e);
        quiet(&mut e, &[MODIFIER_LINK_OBJECT]);
        let keys = color_data(&mut e, 1, 0x14, &[]);
        e.map(0x0f80_0000, 4);
        e.mem.set_u32(0x0f80_0000, keys);
        e.register(GET_OBJECT_FROM_LINK_ID, |e, _| {
            answer(e.mem.u32(0x0f80_0000))
        });
        let this = e.new_object::<M>();
        e.call_log = Some(vec![]);
        e.call(0x00c2_d1c0, &args![this, 0x700u32]);
        assert_eq!(
            calls_to(&e, MODIFIER_LINK_OBJECT),
            vec![vec![this.addr(), 0x700]]
        );
        assert_eq!(calls_to(&e, GET_OBJECT_FROM_LINK_ID), vec![vec![0x700]]);
        assert_eq!(e.get(this, M::m_spColorData), keys);
        assert_eq!(e.get(this, M::m_fHiKeyTime), 1.0f32.to_bits());
    }

    #[test]
    fn destructor_releases_the_colour_data() {
        const DELETE: u32 = 0x0000_c104;
        let mut e = Engine::new();
        test_stream::interlocked(&mut e);
        quiet(&mut e, &[DELETE, MODIFIER_DESTRUCTOR, NI_DELETE]);
        let this = e.new_object::<M>();
        let keys = color_data(&mut e, 1, 0x14, &[(0x4, DELETE)]);
        e.set(this, M::m_spColorData, keys);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_d1f0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, DELETE), vec![vec![keys]]);
        assert_eq!(calls_to(&e, MODIFIER_DESTRUCTOR), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x24]]);
        // Still counted elsewhere: not deleted; no data: nothing released.
        e.mem.set_u32(keys + 4, 2);
        e.call(0x00c2_d1f0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, DELETE).len(), 1);
        e.set(this, M::m_spColorData, 0);
        e.call(0x00c2_d1f0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 2);
        assert_eq!(calls_to(&e, NI_DELETE).len(), 1);
    }

    #[test]
    fn viewer_strings_add_the_name_and_the_colour_data() {
        const STRINGS: u32 = 0x0000_c060;
        let mut e = Engine::new();
        quiet(&mut e, &[MODIFIER_GET_VIEWER_STRINGS, STRINGS]);
        viewer_test::counting(&mut e, &[VIEWER_STRING]);
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_00d0);
        let this = e.new_object::<M>();
        let keys = color_data(&mut e, 1, 0x14, &[(0x60, STRINGS)]);
        e.set(this, M::m_spColorData, keys);
        let strings = viewer_test::strings(&mut e, 8);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d2a0, &args![this, strings]);
        assert_eq!(viewer_test::collected(&e, strings), vec![1]);
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x0100_00d0]]);
        assert_eq!(calls_to(&e, STRINGS), vec![vec![keys, strings]]);
    }
}
