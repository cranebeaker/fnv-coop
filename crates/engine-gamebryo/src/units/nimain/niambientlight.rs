//! `NiMain/niambientlight.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAmbientLight::ms_RTTI` (the class's `NiRTTI` record).
const NI_AMBIENT_LIGHT_RTTI: u32 = 0x011f_5f6c;

// Translated from 00a9b390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_ambient_light_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_AMBIENT_LIGHT_RTTI)
}

/// `00a72080`: the jump target of `00a75c00`.
const JUMP_TARGET_00A75C00: u32 = 0x00a7_2080;

/// `00a90010`: the jump target of `00a75c10`.
const JUMP_TARGET_00A75C10: u32 = 0x00a9_0010;

/// `00a71f50`: the jump target of `00a9b360`.
const JUMP_TARGET_00A9B360: u32 = 0x00a7_1f50;

/// `00a720f0`: the jump target of `00a9b370`.
const JUMP_TARGET_00A9B370: u32 = 0x00a7_20f0;

/// `00a72160`: the jump target of `00a9b380`.
const JUMP_TARGET_00A9B380: u32 = 0x00a7_2160;

// Translated from 00a75c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::LoadBinary` (Xbox PDB): a 5-byte jump to `00a72080` (engine map: `NiLight::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiAmbientLight@@UAAXAAVNiStream@@@Z`.
pub fn ni_ambient_light_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A75C00, &args![this, stream]);
}

// Translated from 00a75c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a90010` (engine map: `NiLight::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiAmbientLight@@UAA_NAAVNiStream@@@Z`.
pub fn ni_ambient_light_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A75C10, &args![this, stream]).bool()
}

// Translated from 00a9b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::LinkObject` (Xbox PDB): a 5-byte jump to `00a71f50` (engine map: `NiLight::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiAmbientLight@@UAAXAAVNiStream@@@Z`.
pub fn ni_ambient_light_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B360, &args![this, stream]);
}

// Translated from 00a9b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::SaveBinary` (Xbox PDB): a 5-byte jump to `00a720f0` (engine map: `NiLight::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiAmbientLight@@UAAXAAVNiStream@@@Z`.
pub fn ni_ambient_light_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B370, &args![this, stream]);
}

// Translated from 00a9b380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::IsEqual` (Xbox PDB): a 5-byte jump to `00a72160` (engine map: `NiLight::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiAmbientLight@@UAA_NPAVNiObject@@@Z`.
pub fn ni_ambient_light_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A9B380, &args![this, other]).bool()
}

/// `NiLight::~NiLight` (`00a72010`, engine map), the destructor body the scalar deleting
/// destructor runs; `thiscall`, no stack word.
const LIGHT_DESTRUCTOR: u32 = 0x00a7_2010;
/// `00aa1460`: the sized delete (`cdecl`: block, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00a71f60`: the base light constructor `CreateClone` runs on the new object before it
/// stores the `NiAmbientLight` virtual table (`thiscall`, no stack word).
const LIGHT_CONSTRUCTOR: u32 = 0x00a7_1f60;
/// `00a71ec0`: copies the light members into the clone (`thiscall` on the source:
/// clone, cloning process).
const LIGHT_COPY_MEMBERS: u32 = 0x00a7_1ec0;
/// The `NiAmbientLight` virtual table (`010a12cc`).
const AMBIENT_LIGHT_VTABLE: u32 = 0x010a_12cc;
/// The size of an `NiAmbientLight` (0xF0 bytes, from the allocation and the sized delete).
const AMBIENT_LIGHT_SIZE: u32 = 0xf0;

// Translated from 00a9b3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight`'s scalar deleting destructor (unnamed in the engine map; `thiscall`,
/// `RET 4`): runs `NiLight::~NiLight` (`00a72010`) and, when bit 0 of `flags` is set,
/// frees the 0xF0 bytes (`00aa1460`). Answers `this`.
pub fn ni_ambient_light_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(LIGHT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, AMBIENT_LIGHT_SIZE]);
    }
    this
}

// Translated from 00a9b3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::CreateClone` (Xbox PDB; `thiscall`, `RET 4`): allocates 0xF0 bytes;
/// when that succeeds runs the base constructor (`00a71f60`), stores the `NiAmbientLight`
/// virtual table (`010a12cc`) and clears the byte at `+0x9D`. Then the source's member copy
/// (`00a71ec0`, on `this`) fills the clone (also called, with a null clone, when the
/// allocation failed). Answers the clone (null on failure).
pub fn ni_ambient_light_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = e.call(OPERATOR_NEW, &args![AMBIENT_LIGHT_SIZE]).u32();
    if clone == 0 {
        e.call(LIGHT_COPY_MEMBERS, &args![this, 0u32, cloning]);
        return Ptr::new(0);
    }
    e.call(LIGHT_CONSTRUCTOR, &args![clone]);
    e.mem.set_u32(clone, AMBIENT_LIGHT_VTABLE);
    e.mem.set_u8(clone + 0x9d, 0);
    e.call(LIGHT_COPY_MEMBERS, &args![this, clone, cloning]);
    Ptr::new(clone)
}

// Translated from 00a9b430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAmbientLight::GetViewerStrings` (Xbox PDB; `thiscall`, `RET 4`): `NiLight::GetViewerStrings`
/// (`00a721e0`, `this` passed on in `ECX`), then the class name (`NiGetViewerString`, `00a7d330`,
/// of the first word of the `NiRTTI` at `011f5f6c`), kept in the `strings` argument's stack word
/// and appended (grown to count + `m_usGrowBy` by `0096ad30` when full, stored by `0096ae90`).
pub fn ni_ambient_light_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(0x00a7_21e0, &args![this, strings]);
    let class_name = e.global::<u32>(NI_AMBIENT_LIGHT_RTTI);
    let text = e.call(0x00a7_d330, &args![class_name]).u32();
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(0x0096_ad30, &args![strings, grow_by + count]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(0x0096_ae90, &args![strings, count, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9b390, ni_ambient_light_get_rtti() -> Ptr),
        entry!(0x00a7_5c00, ni_ambient_light_load_binary(Ptr, Ptr)),
        entry!(0x00a7_5c10, ni_ambient_light_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a9_b360, ni_ambient_light_link_object(Ptr, Ptr)),
        entry!(0x00a9_b370, ni_ambient_light_save_binary(Ptr, Ptr)),
        entry!(0x00a9_b380, ni_ambient_light_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_b3a0, ni_ambient_light_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a9_b3d0, ni_ambient_light_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a9_b430, ni_ambient_light_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_ambient_light_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9b390, &args![]).u32(), NI_AMBIENT_LIGHT_RTTI);
    }

    #[test]
    fn thunk_00a75c00_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_5c00;
        const TARGET: u32 = 0x00a7_2080;
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
    fn thunk_00a75c10_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a7_5c10;
        const TARGET: u32 = 0x00a9_0010;
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
    fn thunk_00a9b360_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b360;
        const TARGET: u32 = 0x00a7_1f50;
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
    fn thunk_00a9b370_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b370;
        const TARGET: u32 = 0x00a7_20f0;
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
    fn thunk_00a9b380_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_b380;
        const TARGET: u32 = 0x00a7_2160;
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
mod b037_tests {
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

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(LIGHT_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_b3a0, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00a9_b3a0, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![0x5000, 0xf0]]);
        assert_eq!(calls(&e, LIGHT_DESTRUCTOR), vec![vec![0x5000]; 2]);
    }

    #[test]
    fn create_clone_builds_the_light_and_copies_the_members() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0xf0);
        e.mem.set_u8(block + 0x9d, 7);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0xf0);
            block.into_ret()
        });
        e.register(LIGHT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(LIGHT_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a9_b3d0, &args![0x6000u32, 0x7000u32]).u32(),
            block
        );
        assert_eq!(e.mem.u32(block), AMBIENT_LIGHT_VTABLE);
        assert_eq!(e.mem.u8(block + 0x9d), 0);
        assert_eq!(calls(&e, LIGHT_CONSTRUCTOR), vec![vec![block]]);
        assert_eq!(
            calls(&e, LIGHT_COPY_MEMBERS),
            vec![vec![0x6000, block, 0x7000]]
        );
    }

    #[test]
    fn create_clone_still_copies_into_a_null_clone() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(LIGHT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(LIGHT_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_b3d0, &args![0x6000u32, 0x7000u32]).u32(), 0);
        assert!(calls(&e, LIGHT_CONSTRUCTOR).is_empty());
        assert_eq!(calls(&e, LIGHT_COPY_MEMBERS), vec![vec![0x6000, 0, 0x7000]]);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    #[test]
    fn viewer_strings_add_the_class_name() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(NI_AMBIENT_LIGHT_RTTI, 0x0102_0000);
        e.register(0x00a7_21e0, |_, _| Ret::default());
        e.register(0x00a7_d330, |_, _| 0x7000u32.into_ret());
        e.register(0x0096_ad30, |_, _| Ret::default());
        e.register(0x0096_ae90, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(0x011f_5f00, value);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 3);
        e.call_log = Some(vec![]);
        e.call(0x00a9_b430, &args![0x4000u32, strings]);
        assert_eq!(calls_to(&e, 0x00a7_21e0), vec![vec![0x4000, strings]]);
        assert_eq!(calls_to(&e, 0x00a7_d330), vec![vec![0x0102_0000]]);
        assert_eq!(calls_to(&e, 0x0096_ad30), vec![vec![strings, 5]]);
        assert_eq!(calls_to(&e, 0x0096_ae90)[0][1], 2);
        assert_eq!(e.mem.u32(0x011f_5f00), 0x7000);
    }
}
