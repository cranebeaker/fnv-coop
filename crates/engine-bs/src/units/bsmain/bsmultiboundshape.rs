//! `BSMain/bsmultiboundshape.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSMultiBoundShape` (0xc bytes) is a `NiObject` with one more field, the
//! cull result at +8. Eleven functions of the unit are its constructor, its
//! scalar deleting destructor, `GetRTTI` and the vtable slots +0x48..+0x60;
//! most of them forward to the `NiObject` base. The slot numbers below are
//! read from the class's vtable at `010c2324`; the engine map names two of
//! the bodies after other slots because identical bodies were folded.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BSMultiBoundShape` (Xbox PDB), 0xc bytes on PC.
    pub struct BSMultiBoundShape: 0x0C {
        /// `kCullResult` (Xbox PDB): `BSMultiBoundShape::BSMBCullResult`.
        0x08 kCullResult: u32,
    }
}

/// `BSMultiBoundShape`'s vtable.
const VTABLE: u32 = 0x010c_2324;
/// `BSMultiBoundShape::ms_RTTI` (`GetRTTI` returns its address).
const RTTI: u32 = 0x0120_30c0;

/// `00a5d3a0` (engine map: `NiMain/niobject.cpp`): the `NiObject`
/// constructor (stores its vtable and a zero reference count, counts the
/// object).
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// `00a5d3d0` (engine map: `NiMain/niobject.cpp`): the `NiObject` destructor.
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `00a5d450` (engine map: `NiMain/niobject.cpp`): `NiObject`'s copy of the
/// members, called from the member copy with (target, cloning process).
const NI_OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// `NiObject::LoadBinary` (`00a5d610`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `NiObject::RegisterStreamables` (`00a5d2f0`).
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
/// `NiObject::IsEqual` (`00a5d470`).
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
/// `NiObject::GetViewerStrings` (`00a5d6e0`).
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// `00a89af0`: an empty function (`RET 4`) taking one stack word.
const EMPTY_ONE_WORD: u32 = 0x00a8_9af0;
/// `operator delete` with a size (`00aa1460`, cdecl: pointer, size).
const OPERATOR_DELETE_SIZED: u32 = 0x00aa_1460;

// Translated from 00c4f980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundShape::BSMultiBoundShape` (Xbox PDB): runs the `NiObject`
/// constructor and stores `BSMultiBoundShape`'s vtable. Returns `this`.
pub fn bs_multi_bound_shape_constructor(e: &mut Engine, this: Ptr<BSMultiBoundShape>) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    this.cast()
}

// Translated from 00c4f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Vtable slot +0x48 (`CreateClone` in the Xbox
/// PDB's slot order): returns `this`; the one stack word is never read.
pub fn fn_00c4f9a0(_e: &mut Engine, this: Ptr<BSMultiBoundShape>, _unused_1: u32) -> Ptr {
    this.cast()
}

// Translated from 00c4f9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundShape::CopyMembers` (Xbox PDB): the `NiObject` member copy
/// into `target` with the cloning process, then the cull result (+8) is
/// copied to `target`.
pub fn bs_multi_bound_shape_copy_members(
    e: &mut Engine,
    this: Ptr<BSMultiBoundShape>,
    target: Ptr<BSMultiBoundShape>,
    cloning_process: u32,
) {
    e.call(
        NI_OBJECT_COPY_MEMBERS,
        &args![this, target, cloning_process],
    );
    let cull_result = e.get(this, BSMultiBoundShape::kCullResult);
    e.set(target, BSMultiBoundShape::kCullResult, cull_result);
}

// Translated from 00c4f9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundShape::LoadBinary` (Xbox PDB), vtable slot +0x4c: forwards
/// the stream to `NiObject::LoadBinary`.
pub fn bs_multi_bound_shape_load_binary(e: &mut Engine, this: Ptr<BSMultiBoundShape>, stream: u32) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
}

// Translated from 00c4fa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Vtable slot +0x50 (`LinkObject` in the Xbox
/// PDB's slot order): forwards the stream to the empty function `00a89af0`.
pub fn fn_00c4fa10(e: &mut Engine, this: Ptr<BSMultiBoundShape>, stream: u32) {
    e.call(EMPTY_ONE_WORD, &args![this, stream]);
}

// Translated from 00c4fa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundShape::RegisterStreamables` (Xbox PDB), vtable slot +0x54:
/// `NiObject::RegisterStreamables`; true when it returned a nonzero byte.
pub fn bs_multi_bound_shape_register_streamables(
    e: &mut Engine,
    this: Ptr<BSMultiBoundShape>,
    stream: u32,
) -> bool {
    e.call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00c4fa60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x58 (`SaveBinary` in the Xbox PDB's slot order; the engine
/// map calls this body `BSMultiBoundShape::LinkObject`, a name of the folded
/// identical body of slot +0x50): forwards the stream to the empty function
/// `00a89af0`. Which of the two names is right for slot +0x58 is not
/// confirmed from the exe, so it is named by address.
pub fn fn_00c4fa60(e: &mut Engine, this: Ptr<BSMultiBoundShape>, stream: u32) {
    e.call(EMPTY_ONE_WORD, &args![this, stream]);
}

// Translated from 00c4fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundShape::IsEqual` (Xbox PDB), vtable slot +0x5c: the result
/// byte of `NiObject::IsEqual`.
pub fn bs_multi_bound_shape_is_equal(
    e: &mut Engine,
    this: Ptr<BSMultiBoundShape>,
    other: Ptr<BSMultiBoundShape>,
) -> bool {
    e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool()
}

// Translated from 00c4fab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x60 (`GetViewerStrings`; the engine map names the body
/// `BSMultiBound::GetViewerStrings`): forwards the string array to
/// `NiObject::GetViewerStrings`.
pub fn bs_multi_bound_shape_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BSMultiBoundShape>,
    strings: u32,
) {
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
}

// Translated from 00c4fad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundShape::_scalar_deleting_destructor_` (Xbox PDB), vtable slot
/// 0: runs the `NiObject` destructor and, when bit 0 of `flags` is set, frees
/// the 0xc bytes. Returns `this`.
pub fn bs_multi_bound_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSMultiBoundShape>,
    flags: u32,
) -> Ptr {
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, 0xcu32]);
    }
    this.cast()
}

// Translated from 00c4fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundShape::GetRTTI` (Xbox PDB), vtable slot +8: the address of
/// the class's static RTTI (`012030c0`).
pub fn bs_multi_bound_shape_get_rtti(_e: &mut Engine) -> u32 {
    RTTI
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4f980,
            bs_multi_bound_shape_constructor(Ptr<BSMultiBoundShape>) -> Ptr
        ),
        entry!(0x00c4f9a0, fn_00c4f9a0(Ptr<BSMultiBoundShape>, u32) -> Ptr),
        entry!(
            0x00c4f9b0,
            bs_multi_bound_shape_copy_members(Ptr<BSMultiBoundShape>, Ptr<BSMultiBoundShape>, u32)
        ),
        entry!(
            0x00c4f9f0,
            bs_multi_bound_shape_load_binary(Ptr<BSMultiBoundShape>, u32)
        ),
        entry!(0x00c4fa10, fn_00c4fa10(Ptr<BSMultiBoundShape>, u32)),
        entry!(
            0x00c4fa30,
            bs_multi_bound_shape_register_streamables(Ptr<BSMultiBoundShape>, u32) -> bool
        ),
        entry!(0x00c4fa60, fn_00c4fa60(Ptr<BSMultiBoundShape>, u32)),
        entry!(
            0x00c4fa80,
            bs_multi_bound_shape_is_equal(Ptr<BSMultiBoundShape>, Ptr<BSMultiBoundShape>) -> bool
        ),
        entry!(
            0x00c4fab0,
            bs_multi_bound_shape_get_viewer_strings(Ptr<BSMultiBoundShape>, u32)
        ),
        entry!(
            0x00c4fad0,
            bs_multi_bound_shape_scalar_deleting_destructor(Ptr<BSMultiBoundShape>, u32) -> Ptr
        ),
        entry!(0x00c4fb00, bs_multi_bound_shape_get_rtti() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn test_bs_multi_bound_shape_constructor() {
        let mut e = Engine::new();
        stub(&mut e, &[NI_OBJECT_CONSTRUCTOR]);
        let shape = e.new_object::<BSMultiBoundShape>();
        e.call_log = Some(vec![]);
        let r = e.call(0x00c4f980, &args![shape]);
        assert_eq!(r.u32(), shape.addr());
        assert_eq!(
            calls_to(&e, NI_OBJECT_CONSTRUCTOR),
            vec![vec![shape.addr()]]
        );
        assert_eq!(e.mem.u32(shape.addr()), VTABLE);
    }

    #[test]
    fn test_fn_00c4f9a0_returns_this() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00c4f9a0, &args![0x1234u32, 0x5678u32]).u32(),
            0x1234
        );
    }

    #[test]
    fn test_bs_multi_bound_shape_copy_members() {
        let mut e = Engine::new();
        stub(&mut e, &[NI_OBJECT_COPY_MEMBERS]);
        let source = e.new_object::<BSMultiBoundShape>();
        let target = e.new_object::<BSMultiBoundShape>();
        e.set(source, BSMultiBoundShape::kCullResult, 3);
        e.set(target, BSMultiBoundShape::kCullResult, 9);
        e.call_log = Some(vec![]);
        e.call(0x00c4f9b0, &args![source, target, 0x77u32]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![source.addr(), target.addr(), 0x77]]
        );
        assert_eq!(e.get(target, BSMultiBoundShape::kCullResult), 3);
        assert_eq!(e.get(source, BSMultiBoundShape::kCullResult), 3);
    }

    #[test]
    fn test_bs_multi_bound_shape_load_binary_forwards() {
        let mut e = Engine::new();
        stub(&mut e, &[NI_OBJECT_LOAD_BINARY]);
        e.call_log = Some(vec![]);
        e.call(0x00c4f9f0, &args![0x10u32, 0x20u32]);
        assert_eq!(calls_to(&e, NI_OBJECT_LOAD_BINARY), vec![vec![0x10, 0x20]]);
    }

    #[test]
    fn test_fn_00c4fa10_forwards_to_the_empty_function() {
        let mut e = Engine::new();
        stub(&mut e, &[EMPTY_ONE_WORD]);
        e.call_log = Some(vec![]);
        e.call(0x00c4fa10, &args![0x10u32, 0x20u32]);
        assert_eq!(calls_to(&e, EMPTY_ONE_WORD), vec![vec![0x10, 0x20]]);
    }

    #[test]
    fn test_bs_multi_bound_shape_register_streamables_normalizes_the_byte() {
        let mut e = Engine::new();
        // A nonzero byte below the top of EAX still reads as true; zero
        // low byte with high bits set reads as false.
        for (eax, expected) in [(0x0000_0001u32, 1u32), (0x0000_0080, 1), (0x1234_5600, 0)] {
            e.register_double(NI_OBJECT_REGISTER_STREAMABLES, move |_, _| Ret {
                eax,
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            let r = e.call(0x00c4fa30, &args![0x10u32, 0x20u32]);
            assert_eq!(r.u32(), expected);
            assert_eq!(
                calls_to(&e, NI_OBJECT_REGISTER_STREAMABLES),
                vec![vec![0x10, 0x20]]
            );
        }
    }

    #[test]
    fn test_fn_00c4fa60_forwards_to_the_empty_function() {
        let mut e = Engine::new();
        stub(&mut e, &[EMPTY_ONE_WORD]);
        e.call_log = Some(vec![]);
        e.call(0x00c4fa60, &args![0x30u32, 0x40u32]);
        assert_eq!(calls_to(&e, EMPTY_ONE_WORD), vec![vec![0x30, 0x40]]);
    }

    #[test]
    fn test_bs_multi_bound_shape_is_equal_returns_the_byte() {
        let mut e = Engine::new();
        for (eax, expected) in [(1u32, true), (0, false), (0x100, false)] {
            e.register_double(NI_OBJECT_IS_EQUAL, move |_, _| Ret {
                eax,
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            let r = e.call(0x00c4fa80, &args![0x50u32, 0x60u32]);
            assert_eq!(r.bool(), expected);
            assert_eq!(calls_to(&e, NI_OBJECT_IS_EQUAL), vec![vec![0x50, 0x60]]);
        }
    }

    #[test]
    fn test_bs_multi_bound_shape_get_viewer_strings_forwards() {
        let mut e = Engine::new();
        stub(&mut e, &[NI_OBJECT_GET_VIEWER_STRINGS]);
        e.call_log = Some(vec![]);
        e.call(0x00c4fab0, &args![0x70u32, 0x80u32]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_GET_VIEWER_STRINGS),
            vec![vec![0x70, 0x80]]
        );
    }

    #[test]
    fn test_bs_multi_bound_shape_scalar_deleting_destructor() {
        let mut e = Engine::new();
        stub(&mut e, &[NI_OBJECT_DESTRUCTOR, OPERATOR_DELETE_SIZED]);
        e.call_log = Some(vec![]);
        let r = e.call(0x00c4fad0, &args![0x4000u32, 0u32]);
        assert_eq!(r.u32(), 0x4000);
        assert!(calls_to(&e, OPERATOR_DELETE_SIZED).is_empty());
        e.call(0x00c4fad0, &args![0x4000u32, 3u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE_SIZED), vec![vec![0x4000, 0xc]]);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCTOR).len(), 2);
        e.call_log = Some(vec![]);
        e.call(0x00c4fad0, &args![0x4000u32, 2u32]);
        assert!(
            calls_to(&e, OPERATOR_DELETE_SIZED).is_empty(),
            "only bit 0 frees"
        );
    }

    #[test]
    fn test_bs_multi_bound_shape_get_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4fb00, &args![]).u32(), 0x0120_30c0);
    }
}
