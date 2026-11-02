//! `NiMain/nispotlight.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSpotLight::ms_RTTI` (the class's `NiRTTI` record).
const NI_SPOT_LIGHT_RTTI: u32 = 0x011f_5ec4;

// Translated from 00a94d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_spot_light_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SPOT_LIGHT_RTTI)
}

/// `00a9b360`: the jump target of `00a94c60`.
const JUMP_TARGET_00A94C60: u32 = 0x00a9_b360;

/// `00a75c10`: the jump target of `00a94c70`.
const JUMP_TARGET_00A94C70: u32 = 0x00a7_5c10;

// Translated from 00a94c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::LinkObject` (Xbox PDB): a 5-byte jump to `00a9b360` (engine map: `NiAmbientLight::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiSpotLight@@UAAXAAVNiStream@@@Z`.
pub fn ni_spot_light_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A94C60, &args![this, stream]);
}

// Translated from 00a94c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a75c10` (engine map: `NiAmbientLight::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiSpotLight@@UAA_NAAVNiStream@@@Z`.
pub fn ni_spot_light_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A94C70, &args![this, stream]).bool()
}

/// `NiPointLight`'s functions (`nipointlight.cpp`, `__thiscall`): the constructor body
/// `00a7d670`, the member copy `00a7d6a0` (clone, cloning process), `IsEqual` (`00a7d730`),
/// `UpdateWorldData` (`00a7d7a0`), `LoadBinary` (`00a7d840`), `SaveBinary` (`00a7d8d0`) and
/// `GetViewerStrings` (`00a7d960`); `NiLight::~NiLight` (`00a72010`).
const POINT_LIGHT_CONSTRUCT: u32 = 0x00a7_d670;
const POINT_LIGHT_COPY_MEMBERS: u32 = 0x00a7_d6a0;
const POINT_LIGHT_IS_EQUAL: u32 = 0x00a7_d730;
const POINT_LIGHT_UPDATE_WORLD_DATA: u32 = 0x00a7_d7a0;
const POINT_LIGHT_LOAD_BINARY: u32 = 0x00a7_d840;
const POINT_LIGHT_SAVE_BINARY: u32 = 0x00a7_d8d0;
const POINT_LIGHT_GET_VIEWER_STRINGS: u32 = 0x00a7_d960;
const LIGHT_DESTRUCTOR: u32 = 0x00a7_2010;
/// The allocation `00aa13e0` and the sized delete `00aa1460` (cdecl).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiSpotLight`'s size and vtable (`010a01cc`).
const NI_SPOT_LIGHT_SIZE: u32 = 0x114;
const NI_SPOT_LIGHT_VTABLE: u32 = 0x010a_01cc;
/// `NiSpotLight` fields (Xbox PDB names, PC offsets as the code uses them): `m_kWorldDir`
/// (+0xfc, three floats), `m_fOuterSpotAngle` (+0x108), `m_fInnerSpotAngle` (+0x10c) and
/// `m_fSpotExponent` (+0x110); the byte +0x9d the constructor sets to 4, the world rotation's
/// first column (+0x68, +0x74, +0x80) and the counter +0xa8 `UpdateWorldData` raises.
const WORLD_DIR: u32 = 0xfc;
const OUTER_SPOT_ANGLE: u32 = 0x108;
const INNER_SPOT_ANGLE: u32 = 0x10c;
const SPOT_EXPONENT: u32 = 0x110;
/// `NiStream`'s file version (+0xd8), input (+0x24c, read function +8) and output (+0x250,
/// write function +0xc); both move their position (+4) by what the function answers. The
/// inner angle is in files from 20.2.0.5.
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const VERSION_20_2_0_5: u32 = 0x1402_0005;
/// `NiGetViewerString` (`00a7d330`, a name), `NiGetViewerString_ov9` (`00a7d550`, cdecl: a
/// label and a `float`), `NiPoint3::GetViewerString` (`00a59af0`, `__thiscall` on the point:
/// a label); the string array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`); the labels.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
const LABEL_WORLD_DIR: u32 = 0x0109_cce0;
const LABEL_OUTER_SPOT_ANGLE: u32 = 0x010a_02cc;
const LABEL_INNER_SPOT_ANGLE: u32 = 0x010a_02b8;
const LABEL_SPOT_EXPONENT: u32 = 0x010a_02a8;

/// A `float` field copied through the x87 stack (a signalling NaN is quieted).
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

// Translated from 00a94c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiSpotLight`'s member copy (`__thiscall`: the clone, the cloning
/// process): `NiPointLight`'s (`00a7d6a0`), then the world direction (three words) and the
/// outer angle, inner angle and exponent (through the x87 stack).
pub fn fn_00a94c00(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(POINT_LIGHT_COPY_MEMBERS, &args![this, clone, cloning]);
    let (from, to) = (this.addr(), clone.addr());
    for offset in [WORLD_DIR, WORLD_DIR + 4, WORLD_DIR + 8] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
    for offset in [OUTER_SPOT_ANGLE, INNER_SPOT_ANGLE, SPOT_EXPONENT] {
        copy_float(e, from + offset, to.wrapping_add(offset));
    }
}

// Translated from 00a94c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::IsEqual` (Xbox PDB): `NiPointLight::IsEqual` (`00a7d730`), then the same outer
/// angle, exponent and inner angle (ordered equal compares: a NaN is unequal).
pub fn ni_spot_light_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(POINT_LIGHT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    for offset in [OUTER_SPOT_ANGLE, SPOT_EXPONENT, INNER_SPOT_ANGLE] {
        let mine = x87_load32(e.mem.u32(this.addr() + offset));
        let theirs = x87_load32(e.mem.u32(other.addr() + offset));
        // `FUCOMPP; TEST AH,0x44; JP`: only an ordered equal compare goes on.
        if theirs != mine {
            return false;
        }
    }
    true
}

// Translated from 00a94ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiSpotLight` constructor body: `NiPointLight`'s
/// (`00a7d670`), the vtable `010a01cc`, the world direction (1, 0, 0), the byte +0x9d set to 4,
/// the exponent 1 and both angles 0. Answers `this`.
pub fn fn_00a94ce0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(POINT_LIGHT_CONSTRUCT, &args![this]);
    let one = x87_store32(1.0);
    let zero = x87_store32(0.0);
    e.mem.set_u32(base, NI_SPOT_LIGHT_VTABLE);
    e.mem.set_u32(base + WORLD_DIR, one);
    e.mem.set_u8(base + 0x9d, 4);
    e.mem.set_u32(base + SPOT_EXPONENT, one);
    e.mem.set_u32(base + WORLD_DIR + 4, zero);
    e.mem.set_u32(base + WORLD_DIR + 8, zero);
    e.mem.set_u32(base + OUTER_SPOT_ANGLE, zero);
    e.mem.set_u32(base + INNER_SPOT_ANGLE, zero);
    this
}

// Translated from 00a94d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::_scalar_deleting_destructor_` (Xbox PDB): `NiLight::~NiLight` (`00a72010`),
/// then the sized delete of 0x114 bytes (`00aa1460`) when bit 0 of `flags` is set. Answers
/// `this`.
pub fn ni_spot_light_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(LIGHT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, NI_SPOT_LIGHT_SIZE]);
    }
    this
}

// Translated from 00a94d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::UpdateWorldData` (Xbox PDB): `NiPointLight::UpdateWorldData` (`00a7d7a0`),
/// the world direction from the world rotation's first column (+0x68, +0x74, +0x80, through
/// the x87 stack), and the counter at +0xa8 up by one.
pub fn ni_spot_light_update_world_data(e: &mut Engine, this: Ptr, word: u32) {
    let base = this.addr();
    e.call(POINT_LIGHT_UPDATE_WORLD_DATA, &args![this, word]);
    copy_float(e, base + 0x68, base + WORLD_DIR);
    copy_float(e, base + 0x74, base + WORLD_DIR + 4);
    copy_float(e, base + 0x80, base + WORLD_DIR + 8);
    let count = e.mem.u32(base + 0xa8).wrapping_add(1);
    e.mem.set_u32(base + 0xa8, count);
}

// Translated from 00a94dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::CreateClone` (Xbox PDB; `__thiscall`, the cloning process): a new spot light
/// (0x114 bytes, `fn_00a94ce0`) or null when the allocation fails, then the member copy
/// (`fn_00a94c00`, also with the null clone). Answers the clone.
pub fn ni_spot_light_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![NI_SPOT_LIGHT_SIZE]).u32();
    let clone = if block != 0 {
        fn_00a94ce0(e, Ptr::new(block))
    } else {
        Ptr::NULL
    };
    fn_00a94c00(e, this, clone, cloning);
    clone
}

// Translated from 00a94e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; translated from the disassembled bytes), cdecl, no
/// words: a new `NiSpotLight` (0x114 bytes from `00aa13e0`, then a tail jump to the
/// constructor body `fn_00a94ce0`), or 0 when the allocation fails. `fn_00a89cc0`
/// (`nimainsdm.rs`) registers it as the loader for the class name `"NiSpotLight"`
/// (`0109f010`).
pub fn fn_00a94e20(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![NI_SPOT_LIGHT_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00a94ce0(e, Ptr::new(block))
}

/// One transfer through the stream's input (`side` +0x24c, function +8) or output (+0x250,
/// function +0xc): four bytes at `buffer` as one 4-byte component.
fn transfer_word(e: &mut Engine, stream: u32, side: u32, function: u32, buffer: u32) {
    let file = e.mem.u32(stream + side);
    let call = e.mem.u32(file + function);
    let count = e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 4);
        e.call(call, &args![file, buffer, 4u32, sizes, 1u32]).u32()
    });
    let position = e.mem.u32(file + 4).wrapping_add(count);
    e.mem.set_u32(file + 4, position);
}

// Translated from 00a94e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::LoadBinary` (Xbox PDB): `NiPointLight::LoadBinary` (`00a7d840`), the outer
/// angle, from 20.2.0.5 the inner angle, then the exponent (4 bytes each).
pub fn ni_spot_light_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (base, s) = (this.addr(), stream.addr());
    e.call(POINT_LIGHT_LOAD_BINARY, &args![this, stream]);
    transfer_word(e, s, STREAM_INPUT, 8, base + OUTER_SPOT_ANGLE);
    if e.mem.u32(s + STREAM_VERSION) >= VERSION_20_2_0_5 {
        transfer_word(e, s, STREAM_INPUT, 8, base + INNER_SPOT_ANGLE);
    }
    transfer_word(e, s, STREAM_INPUT, 8, base + SPOT_EXPONENT);
}

// Translated from 00a94ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::SaveBinary` (Xbox PDB): `NiPointLight::SaveBinary` (`00a7d8d0`), then the outer
/// angle, the inner angle and the exponent (4 bytes each).
pub fn ni_spot_light_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let (base, s) = (this.addr(), stream.addr());
    e.call(POINT_LIGHT_SAVE_BINARY, &args![this, stream]);
    for offset in [OUTER_SPOT_ANGLE, INNER_SPOT_ANGLE, SPOT_EXPONENT] {
        transfer_word(e, s, STREAM_OUTPUT, 0xc, base + offset);
    }
}

/// The inlined `NiTArray<char *>::Add`: grown to its size plus `m_usGrowBy` when full, then
/// `SetAt` at the old size through a stack word.
fn add_viewer_string(e: &mut Engine, strings: u32, value: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let grow_by = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + size]);
    }
    e.with_stack(4, |e, element| {
        e.mem.set_u32(element.addr(), value);
        e.call(STRING_ARRAY_SET_AT, &args![strings, size, element]);
    });
}

// Translated from 00a94f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSpotLight::GetViewerStrings` (Xbox PDB): `NiPointLight`'s strings (`00a7d960`), then the
/// class name, `"m_kWorldDir"` (`NiPoint3::GetViewerString` on the direction) and the outer
/// angle, inner angle and exponent (`NiGetViewerString_ov9`, each `float` pushed through the
/// x87 stack), each appended to the array.
pub fn ni_spot_light_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let (base, strings) = (this.addr(), strings.addr());
    e.call(POINT_LIGHT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(NI_SPOT_LIGHT_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    add_viewer_string(e, strings, text);
    let text = e
        .call(
            POINT_GET_VIEWER_STRING,
            &args![base + WORLD_DIR, LABEL_WORLD_DIR],
        )
        .u32();
    add_viewer_string(e, strings, text);
    for (label, offset) in [
        (LABEL_OUTER_SPOT_ANGLE, OUTER_SPOT_ANGLE),
        (LABEL_INNER_SPOT_ANGLE, INNER_SPOT_ANGLE),
        (LABEL_SPOT_EXPONENT, SPOT_EXPONENT),
    ] {
        let value = x87_quiet_bits(e.mem.u32(base + offset));
        let text = e
            .call(NI_GET_VIEWER_STRING_FLOAT, &args![label, value])
            .u32();
        add_viewer_string(e, strings, text);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a94d50, ni_spot_light_get_rtti() -> Ptr),
        entry!(0x00a9_4c60, ni_spot_light_link_object(Ptr, Ptr)),
        entry!(0x00a9_4c70, ni_spot_light_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a9_4c00, fn_00a94c00(Ptr, Ptr, Ptr)),
        entry!(0x00a9_4c80, ni_spot_light_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_4ce0, fn_00a94ce0(Ptr) -> Ptr),
        entry!(0x00a9_4d60, ni_spot_light_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a9_4d90, ni_spot_light_update_world_data(Ptr, u32)),
        entry!(0x00a9_4dd0, ni_spot_light_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a9_4e20, fn_00a94e20() -> Ptr),
        entry!(0x00a9_4e40, ni_spot_light_load_binary(Ptr, Ptr)),
        entry!(0x00a9_4ee0, ni_spot_light_save_binary(Ptr, Ptr)),
        entry!(0x00a9_4f70, ni_spot_light_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_spot_light_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a94d50, &args![]).u32(), NI_SPOT_LIGHT_RTTI);
    }

    #[test]
    fn thunk_00a94c60_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_4c60;
        const TARGET: u32 = 0x00a9_b360;
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
    fn thunk_00a94c70_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_4c70;
        const TARGET: u32 = 0x00a7_5c10;
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
}

#[cfg(test)]
mod n019_tests {
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

    const SIGNALLING: u32 = 0x7f80_0001;

    #[test]
    fn copy_compare_and_construct() {
        let mut e = Engine::new();
        e.register(POINT_LIGHT_COPY_MEMBERS, |_, _| Ret::default());
        e.register(POINT_LIGHT_IS_EQUAL, |_, _| word(1));
        e.register(POINT_LIGHT_CONSTRUCT, |_, _| Ret::default());
        let a = e.mem.alloc(NI_SPOT_LIGHT_SIZE);
        assert_eq!(e.call(0x00a9_4ce0, &args![a]).u32(), a);
        assert_eq!(e.mem.u32(a), NI_SPOT_LIGHT_VTABLE);
        assert_eq!(e.mem.f32(a + WORLD_DIR), 1.0);
        assert_eq!(e.mem.f32(a + SPOT_EXPONENT), 1.0);
        assert_eq!(e.mem.u8(a + 0x9d), 4);
        e.mem.set_f32(a + OUTER_SPOT_ANGLE, 0.5);
        e.mem.set_u32(a + INNER_SPOT_ANGLE, SIGNALLING);
        e.mem.set_u32(a + WORLD_DIR + 4, SIGNALLING);
        let b = e.mem.alloc(NI_SPOT_LIGHT_SIZE);
        e.call_log = Some(vec![]);
        e.call(0x00a9_4c00, &args![a, b, 0x99u32]);
        assert_eq!(
            calls_to(&e, POINT_LIGHT_COPY_MEMBERS),
            vec![vec![a, b, 0x99]]
        );
        assert_eq!(e.mem.u32(b + WORLD_DIR + 4), SIGNALLING);
        assert_eq!(e.mem.u32(b + INNER_SPOT_ANGLE), 0x7fc0_0001);
        assert_eq!(e.mem.f32(b + OUTER_SPOT_ANGLE), 0.5);
        // The NaN inner angle makes them unequal; equal once it is a number.
        assert!(!e.call(0x00a9_4c80, &args![a, b]).bool());
        e.mem.set_f32(a + INNER_SPOT_ANGLE, 0.25);
        e.mem.set_f32(b + INNER_SPOT_ANGLE, 0.25);
        assert!(e.call(0x00a9_4c80, &args![a, b]).bool());
        e.mem.set_f32(b + SPOT_EXPONENT, 2.0);
        assert!(!e.call(0x00a9_4c80, &args![a, b]).bool());
        e.mem.set_f32(b + SPOT_EXPONENT, 1.0);
        e.register(POINT_LIGHT_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a9_4c80, &args![a, b]).bool());
    }

    #[test]
    fn create_clone_object_and_destructor() {
        let mut e = Engine::new();
        e.register(POINT_LIGHT_COPY_MEMBERS, |_, _| Ret::default());
        e.register(POINT_LIGHT_CONSTRUCT, |_, _| Ret::default());
        e.register(LIGHT_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| word(e.mem.alloc(a[0])));
        let light = e.mem.alloc(NI_SPOT_LIGHT_SIZE);
        e.mem.set_f32(light + SPOT_EXPONENT, 3.0);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a9_4dd0, &args![light, 0x55u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_SPOT_LIGHT_VTABLE);
        assert_eq!(e.mem.f32(clone + SPOT_EXPONENT), 3.0);
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![NI_SPOT_LIGHT_SIZE]]);
        let made = e.call(0x00a9_4e20, &args![]).u32();
        assert_eq!(e.mem.f32(made + SPOT_EXPONENT), 1.0);
        e.register(NI_ALLOCATE, |_, _| word(0));
        assert_eq!(e.call(0x00a9_4e20, &args![]).u32(), 0);
        // A failed allocation still copies, into null.
        e.register(POINT_LIGHT_COPY_MEMBERS, |_, _| Ret::default());
        e.map(0, 0x200);
        assert_eq!(e.call(0x00a9_4dd0, &args![light, 0x55u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, POINT_LIGHT_COPY_MEMBERS)[1],
            vec![light, 0, 0x55]
        );
        assert_eq!(e.call(0x00a9_4d60, &args![light, 0u32]).u32(), light);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 0);
        e.call(0x00a9_4d60, &args![light, 1u32]);
        assert_eq!(
            calls_to(&e, SIZED_DELETE),
            vec![vec![light, NI_SPOT_LIGHT_SIZE]]
        );
        assert_eq!(calls_to(&e, LIGHT_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn update_world_data_takes_the_direction() {
        let mut e = Engine::new();
        e.register(POINT_LIGHT_UPDATE_WORLD_DATA, |_, _| Ret::default());
        let light = e.mem.alloc(NI_SPOT_LIGHT_SIZE);
        e.mem.set_f32(light + 0x68, 0.5);
        e.mem.set_f32(light + 0x74, -1.0);
        e.mem.set_u32(light + 0x80, SIGNALLING);
        e.mem.set_u32(light + 0xa8, 6);
        e.call_log = Some(vec![]);
        e.call(0x00a9_4d90, &args![light, 0x1234u32]);
        assert_eq!(
            calls_to(&e, POINT_LIGHT_UPDATE_WORLD_DATA),
            vec![vec![light, 0x1234]]
        );
        assert_eq!(e.mem.f32(light + WORLD_DIR), 0.5);
        assert_eq!(e.mem.f32(light + WORLD_DIR + 4), -1.0);
        assert_eq!(e.mem.u32(light + WORLD_DIR + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(light + 0xa8), 7);
    }

    type Moves = Rc<RefCell<Vec<u32>>>;

    /// A stream whose input and output record the address each transfer touches.
    fn stream(e: &mut Engine, version: u32) -> (u32, Moves) {
        const MOVE_DOUBLE: u32 = 0x0071_0000;
        let s = e.mem.alloc(0x260);
        e.mem.set_u32(s + STREAM_VERSION, version);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, MOVE_DOUBLE);
        e.mem.set_u32(file + 0xc, MOVE_DOUBLE);
        e.mem.set_u32(s + STREAM_INPUT, file);
        e.mem.set_u32(s + STREAM_OUTPUT, file);
        let moves: Moves = Rc::new(RefCell::new(vec![]));
        let log = moves.clone();
        e.register_double(MOVE_DOUBLE, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            log.borrow_mut().push(a[1]);
            word(4)
        });
        (s, moves)
    }

    #[test]
    fn load_and_save_the_angles() {
        let mut e = Engine::new();
        e.register(POINT_LIGHT_LOAD_BINARY, |_, _| Ret::default());
        e.register(POINT_LIGHT_SAVE_BINARY, |_, _| Ret::default());
        let light = e.mem.alloc(NI_SPOT_LIGHT_SIZE);
        let (old, moves) = stream(&mut e, 0x1402_0004);
        e.call(0x00a9_4e40, &args![light, old]);
        assert_eq!(
            *moves.borrow(),
            vec![light + OUTER_SPOT_ANGLE, light + SPOT_EXPONENT]
        );
        let (new, moves) = stream(&mut e, VERSION_20_2_0_5);
        e.call(0x00a9_4e40, &args![light, new]);
        assert_eq!(
            *moves.borrow(),
            vec![
                light + OUTER_SPOT_ANGLE,
                light + INNER_SPOT_ANGLE,
                light + SPOT_EXPONENT
            ]
        );
        let (out, moves) = stream(&mut e, 0);
        e.call(0x00a9_4ee0, &args![light, out]);
        assert_eq!(
            *moves.borrow(),
            vec![
                light + OUTER_SPOT_ANGLE,
                light + INNER_SPOT_ANGLE,
                light + SPOT_EXPONENT
            ]
        );
        let file = e.mem.u32(out + STREAM_OUTPUT);
        assert_eq!(e.mem.u32(file + 4), 12);
    }

    #[test]
    fn get_viewer_strings_appends_direction_and_angles() {
        let mut e = Engine::new();
        e.map(NI_SPOT_LIGHT_RTTI, 4);
        e.mem.set_u32(NI_SPOT_LIGHT_RTTI, 0x010a_0300);
        e.register(POINT_LIGHT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| word(a[0]));
        e.register(POINT_GET_VIEWER_STRING, |_, a| word(a[0]));
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, a| word(a[1]));
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(e.mem.u32(a[0] + 4) + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let array = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, array);
        e.mem.set_u16(strings + 8, 8);
        let light = e.mem.alloc(NI_SPOT_LIGHT_SIZE);
        e.mem.set_f32(light + OUTER_SPOT_ANGLE, 0.5);
        e.mem.set_u32(light + INNER_SPOT_ANGLE, SIGNALLING);
        e.mem.set_f32(light + SPOT_EXPONENT, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x00a9_4f70, &args![light, strings]);
        let values: Vec<u32> = (0..5).map(|i| e.mem.u32(array + 4 * i)).collect();
        assert_eq!(
            values,
            vec![
                0x010a_0300,
                light + WORLD_DIR,
                0.5f32.to_bits(),
                0x7fc0_0001,
                2.0f32.to_bits()
            ]
        );
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![vec![light + WORLD_DIR, LABEL_WORLD_DIR]]
        );
        let labels: Vec<u32> = calls_to(&e, NI_GET_VIEWER_STRING_FLOAT)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(
            labels,
            vec![
                LABEL_OUTER_SPOT_ANGLE,
                LABEL_INNER_SPOT_ANGLE,
                LABEL_SPOT_EXPONENT
            ]
        );
    }
}
