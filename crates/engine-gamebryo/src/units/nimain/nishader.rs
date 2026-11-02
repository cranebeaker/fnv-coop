//! `NiMain/nishader.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiShader::ms_RTTI` (the class's `NiRTTI` record).
const NI_SHADER_RTTI: u32 = 0x011f_5d08;

// Translated from 00a8f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_shader_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SHADER_RTTI)
}

// Translated from 00a8f3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::Initialize` (Xbox PDB name in the engine map; `thiscall`, no
/// stack words): sets the byte at `+0x10` to 1 and answers 1 (`AL`).
pub fn ni_shader_initialize(e: &mut Engine, this: Ptr) -> bool {
    e.mem.set_u8(this.addr() + 0x10, 1);
    true
}

// Translated from 00a8f410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual method shared by many vtables
/// (`thiscall`, `RET 8`: two stack words, neither read) that answers 0.
pub fn fn_00a8f410(_e: &mut Engine, _this: Ptr, _unused_0: u32, _unused_1: u32) -> u32 {
    0
}

/// `NiShader`'s vtable (`0109f584`, RTTI `.?AVNiShader@@`).
const NI_SHADER_VTABLE: u32 = 0x0109_f584;
/// `NiRefObject`'s vtable (`0101dce4`, RTTI `.?AVNiRefObject@@`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
/// `NiRefObject::ms_uiObjects` (`011f4400`), the live object count.
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// The empty `NiFixedString` handle (`0109b220`), never released.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// The `InterlockedDecrement` import slot (`kernel32`, stdcall, one word).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a8f490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiShader`'s destructor body (`thiscall`, no
/// stack words): stores `NiShader`'s vtable, releases the name handle at `+0x8`
/// (unless it equals the handle in `0109b220`: `InterlockedDecrement` of the
/// count 8 bytes before the text), then stores `NiRefObject`'s vtable and
/// decrements `NiRefObject`'s object count `011f4400`.
pub fn fn_00a8f490(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_SHADER_VTABLE);
    let name = e.mem.u32(this.addr() + 0x8);
    if name != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![NI_REF_OBJECT_COUNT]);
}

// Translated from 00a8f3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual method of `NiShader` (`thiscall`, `RET 0x1c`: seven
/// stack words, none read) that answers 0.
pub fn fn_00a8f3e0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    0
}

// Translated from 00a8f3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual method of `NiShader` (`thiscall`, `RET 0x20`: eight
/// stack words, none read) that answers 0.
pub fn fn_00a8f3f0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
) -> u32 {
    0
}

// Translated from 00a8f400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::PrepareGeometryForRendering` (Xbox PDB; `thiscall`, `RET 0x10`: four stack
/// words, none read): answers 0.
pub fn ni_shader_prepare_geometry_for_rendering(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    0
}

// Translated from 00a8f420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::SetAlphaOverride` (Xbox PDB; `thiscall`, `RET 0x18`: six stack words, none
/// read): answers false (`XOR AL,AL`; only the low byte is set).
pub fn ni_shader_set_alpha_override(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> bool {
    false
}

/// `00aa1460` (`cdecl`, (block, size)): the sized delete.
const SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00a8f4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::_scalar_deleting_destructor_` (Xbox PDB): the destructor body inlined (as
/// [`fn_00a8f490`]: `NiShader`'s vtable, the name handle at +8 released unless it is the
/// empty one, `NiRefObject`'s vtable, its object count decremented), then the sized delete
/// of 0x14 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_shader_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_SHADER_VTABLE);
    let name = e.mem.u32(this.addr() + 0x8);
    if name != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![NI_REF_OBJECT_COUNT]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a8_f3e0,
            fn_00a8f3e0(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a8_f3f0,
            fn_00a8f3f0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a8_f400,
            ni_shader_prepare_geometry_for_rendering(Ptr, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a8_f420,
            ni_shader_set_alpha_override(Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00a8_f4d0,
            ni_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a8f480, ni_shader_get_rtti() -> Ptr),
        entry!(0x00a8_f3d0, ni_shader_initialize(Ptr) -> bool),
        entry!(0x00a8_f410, fn_00a8f410(Ptr, u32, u32) -> u32),
        entry!(0x00a8_f490, fn_00a8f490(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_shader_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8f480, &args![]).u32(), NI_SHADER_RTTI);
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn ni_shader_initialize_sets_the_flag() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00a8_f3d0, &args![this]).u32() & 0xff, 1);
        assert_eq!(e.mem.u8(this + 0x10), 1);
    }

    #[test]
    fn fn_00a8f410_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8_f410, &args![0x1000u32, 1u32, 2u32]).u32(), 0);
    }

    #[test]
    fn fn_00a8f490_releases_the_name_unless_empty() {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global::<u32>(FIXED_STRING_EMPTY, 0x4444);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        let run = |e: &mut Engine, name: u32| -> Vec<Vec<u32>> {
            e.mem.set_u32(this + 8, name);
            e.call_log = Some(vec![]);
            e.call(0x00a8_f490, &args![this]);
            e.call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 == INTERLOCKED_DECREMENT)
                .map(|c| c.1)
                .collect()
        };
        assert_eq!(
            run(&mut e, 0x8008),
            vec![vec![0x8000], vec![NI_REF_OBJECT_COUNT]]
        );
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(run(&mut e, 0x4444), vec![vec![NI_REF_OBJECT_COUNT]]);
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;

    #[test]
    fn the_stub_methods_answer_zero() {
        let mut e = Engine::new();
        let words = |n: u32| -> Vec<u32> { (0..=n).map(|i| 0x1000 + i).collect() };
        assert_eq!(e.call(0x00a8_f3e0, &words(7)).u32(), 0);
        assert_eq!(e.call(0x00a8_f3f0, &words(8)).u32(), 0);
        assert_eq!(e.call(0x00a8_f400, &words(4)).u32(), 0);
        assert!(!e.call(0x00a8_f420, &words(6)).bool());
    }

    #[test]
    fn scalar_deleting_destructor_releases_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global::<u32>(FIXED_STRING_EMPTY, 0x4444);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, 0x8008);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_f4d0, &args![this, 0u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let tail: Vec<_> = log[1..].to_vec();
        assert_eq!(
            tail,
            vec![
                (INTERLOCKED_DECREMENT, vec![0x8000]),
                (INTERLOCKED_DECREMENT, vec![NI_REF_OBJECT_COUNT]),
            ]
        );
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        // The empty name is not released; bit 0 deletes 0x14 bytes.
        e.mem.set_u32(this + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f4d0, &args![this, 0xffu32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (INTERLOCKED_DECREMENT, vec![NI_REF_OBJECT_COUNT]),
                (SIZED_DELETE, vec![this, 0x14]),
            ]
        );
    }
}
