//! `NiMain/run_00a744a0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiTriBasedGeom` default constructor (`00a89740`, `this` only).
const NI_TRI_BASED_GEOM_CONSTRUCT: u32 = 0x00a8_9740;
/// The `NiTriShape` vtable (its RTTI locator names `.?AVNiTriShape@@`).
const NI_TRI_SHAPE_VTABLE: u32 = 0x0109_d454;
/// The Gamebryo allocator (`00aa13e0`, cdecl, the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiTriShape` object size.
const NI_TRI_SHAPE_SIZE: u32 = 0xc4;

// Translated from 00a744a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTriShape` default constructor: the `NiTriBasedGeom` part, then
/// the class's vtable. Returns `this`.
pub fn fn_00a744a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_TRI_BASED_GEOM_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_TRI_SHAPE_VTABLE);
    this
}

// Translated from 00a744d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTriShape::CreateObject` (the class's stream factory; Ghidra made no
/// function of it): a new 0xC4-byte object from the Gamebryo allocator
/// with the `NiTriBasedGeom` constructor and the class's vtable; null when
/// the allocation fails.
pub fn ni_tri_shape_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![NI_TRI_SHAPE_SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_TRI_BASED_GEOM_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_TRI_SHAPE_VTABLE);
    Ptr::new(object)
}

/// `00a9b140`: the jump target of `00a744c0` (engine map: `NiParticleMeshes::CopyMembers`,
/// itself a jump; `thiscall`, clone and cloning process).
const JUMP_TARGET_00A744C0: u32 = 0x00a9_b140;

// Translated from 00a744c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra names it `CopyMembers`): a 5-byte jump to `00a9b140`,
/// passing on `this` and its two words (the clone and the cloning process, as the jump
/// target takes them).
pub fn fn_00a744c0(e: &mut Engine, this: Ptr, clone: Ptr, cloning_process: Ptr) {
    e.call(JUMP_TARGET_00A744C0, &args![this, clone, cloning_process]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a744a0, fn_00a744a0(Ptr) -> Ptr),
        entry!(0x00a744d0, ni_tri_shape_create_object() -> Ptr),
        entry!(0x00a7_44c0, fn_00a744c0(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tri_shape_constructor_and_factory() {
        let mut e = Engine::new();
        e.register(NI_TRI_BASED_GEOM_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_44a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TRI_SHAPE_VTABLE);
        let object = e.call(0x00a7_44d0, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_TRI_SHAPE_VTABLE);
        let log = e.call_log.take().unwrap();
        let constructed: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == NI_TRI_BASED_GEOM_CONSTRUCT)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(constructed, vec![this, object]);
        assert!(log.contains(&(NI_ALLOCATE, vec![0xc4])));
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a7_44d0, &args![]).u32(), 0);
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    #[test]
    fn thunk_00a744c0_forwards_its_words_to_the_jump_target() {
        let mut e = Engine::new();
        e.register(JUMP_TARGET_00A744C0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a7_44c0, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == JUMP_TARGET_00A744C0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }
}
