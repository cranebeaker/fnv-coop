//! `fallout shared/sky/skyobject.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `SkyObject` is the base of the sky's parts (atmosphere, clouds, sun, stars, moons):
//! a vtable and the part's root node (`spRoot`). The sky code is compiled without
//! optimisation, so every `NiPointer` access is a call (`get` `00559450`, assignment
//! `0066b0d0`, construction `00633c90`, destruction `0045cec0`); the translations make
//! those calls as the code does.

#[allow(unused_imports)]
use crate::prelude::*;

/// `SkyObject`'s vtable (`0104f0f8`).
pub(crate) const SKY_OBJECT_VTABLE: u32 = 0x0104_f0f8;
/// `NiPointer<T>::get` (`00559450`, `thiscall`, no stack words), `NiPointer<T>::operator=`
/// (`0066b0d0`, `thiscall`, one word), the `NiPointer` constructor from a raw pointer
/// (`00633c90`, `thiscall`, one word) and the `NiPointer` destructor (`0045cec0`,
/// `thiscall`).
pub(crate) const NI_POINTER_GET: u32 = 0x0055_9450;
pub(crate) const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
pub(crate) const NI_POINTER_FROM_RAW: u32 = 0x0063_3c90;
pub(crate) const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
pub(crate) const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiNode` constructor `00a5ecb0` (`thiscall`, one word: the child count; 0xac bytes).
pub(crate) const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The two node flag setters `Initialize` calls with 1 (`004902f0`, `00635fe0`;
/// `thiscall`, one word).
pub(crate) const NODE_SET_FLAG_004902F0: u32 = 0x0049_02f0;
pub(crate) const NODE_SET_FLAG_00635FE0: u32 = 0x0063_5fe0;

layout! {
    /// `SkyObject` (Xbox PDB), 8 bytes.
    pub struct SkyObject: 0x08 {
        /// `spRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x04 spRoot: u32,
    }
}

/// `NiPointer<T>::get` on the pointer at `slot`.
pub(crate) fn pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

/// `NiPointer<T>::operator=` on the pointer at `slot`.
pub(crate) fn pointer_assign(e: &mut Engine, slot: u32, object: u32) {
    e.call(NI_POINTER_ASSIGN, &args![slot, object]);
}

// Translated from 0063f830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyObject::SkyObject` (Xbox PDB name from the engine map; `thiscall`, no stack words):
/// the vtable, `spRoot` constructed from 0 and assigned 0. Answers `this`.
// Translation incomplete: 0063f830 - the C++ exception frame (FS:[0] chain, unwind state 0 for spRoot) is not translated.
pub fn sky_object_sky_object(e: &mut Engine, this: Ptr<SkyObject>) -> Ptr<SkyObject> {
    let base = this.addr();
    e.mem.set_u32(base, SKY_OBJECT_VTABLE);
    e.call(NI_POINTER_FROM_RAW, &args![base + 4, 0u32]);
    pointer_assign(e, base + 4, 0);
    this
}

// Translated from 0063f8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyObject::~SkyObject` (Xbox PDB name from the engine map; `thiscall`, no stack words):
/// the vtable, `spRoot` assigned 0, then its `NiPointer` destructor.
// Translation incomplete: 0063f8d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for spRoot) is not translated.
pub fn sky_object_destructor(e: &mut Engine, this: Ptr<SkyObject>) {
    let base = this.addr();
    e.mem.set_u32(base, SKY_OBJECT_VTABLE);
    pointer_assign(e, base + 4, 0);
    e.call(NI_POINTER_DESTRUCT, &args![base + 4]);
}

// Translated from 0063f940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyObject::Initialize` (Xbox PDB name from the engine map; `thiscall`, one word: the
/// parent node): a new `NiNode` (0xac bytes, no children; 0 when the allocation fails) as
/// `spRoot`, the two flag setters `004902f0` and `00635fe0` with 1 on it, and the parent's
/// vtable slot +0xdc (`AttachChild` in the Xbox PDB's `NiNode` slots) with (root, 1).
// Translation incomplete: 0063f940 - the C++ exception frame (FS:[0] chain, unwind state 0 for the node allocation) is not translated.
pub fn sky_object_initialize(e: &mut Engine, this: Ptr<SkyObject>, parent: u32) {
    let base = this.addr();
    let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    pointer_assign(e, base + 4, node);
    let root = pointer_get(e, base + 4);
    e.call(NODE_SET_FLAG_004902F0, &args![root, 1u32]);
    let root = pointer_get(e, base + 4);
    e.call(NODE_SET_FLAG_00635FE0, &args![root, 1u32]);
    let root = pointer_get(e, base + 4);
    e.vcall(parent, 0xdc, &args![root, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0063_f830,
            sky_object_sky_object(Ptr<SkyObject>) -> Ptr<SkyObject>
        ),
        entry!(0x0063_f8d0, sky_object_destructor(Ptr<SkyObject>)),
        entry!(0x0063_f940, sky_object_initialize(Ptr<SkyObject>, u32)),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
}

/// Test helpers the sky units share.
#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    pub(crate) fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    pub(crate) fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    /// `NiPointer` get, assignment and construction as plain memory operations.
    pub(crate) fn plain_pointers(e: &mut Engine) {
        e.register(NI_POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }

    /// A word answered as `eax`.
    pub(crate) fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A `float` answered in `ST0`.
    pub(crate) fn answer_float(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    /// An object whose vtable has `slots` (byte offset, target) set.
    pub(crate) fn object_with(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        let object = e.mem.alloc(size.max(4));
        e.mem.set_u32(object, vtable);
        object
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;

    #[test]
    fn constructor_and_destructor_clear_the_root() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(&mut e, &[NI_POINTER_DESTRUCT]);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_f830, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SKY_OBJECT_VTABLE);
        assert_eq!(calls(&e, NI_POINTER_FROM_RAW), vec![vec![this + 4, 0]]);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN), vec![vec![this + 4, 0]]);
        e.mem.set_u32(this, 0);
        e.mem.set_u32(this + 4, 0x1234);
        e.call(0x0063_f8d0, &args![this]);
        assert_eq!(e.mem.u32(this), SKY_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT), vec![vec![this + 4]]);
    }

    #[test]
    fn initialize_makes_the_root_and_attaches_it() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[NODE_SET_FLAG_004902F0, NODE_SET_FLAG_00635FE0, 0x0000_7770],
        );
        e.register(NI_ALLOCATE, |_, _| answer(0x0050_0000));
        e.register(NI_NODE_CONSTRUCT, |_, a| answer(a[0] + 0x10));
        let parent = object_with(&mut e, 4, &[(0xdc, 0x0000_7770)]);
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0063_f940, &args![this, parent]);
        let node = 0x0050_0010;
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0xac]]);
        assert_eq!(calls(&e, NI_NODE_CONSTRUCT), vec![vec![0x0050_0000, 0]]);
        assert_eq!(e.mem.u32(this + 4), node);
        assert_eq!(calls(&e, NODE_SET_FLAG_004902F0), vec![vec![node, 1]]);
        assert_eq!(calls(&e, NODE_SET_FLAG_00635FE0), vec![vec![node, 1]]);
        assert_eq!(calls(&e, 0x0000_7770), vec![vec![parent, node, 1]]);
        // A failed allocation leaves a null root, still attached.
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0063_f940, &args![this, parent]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert!(calls(&e, NI_NODE_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, 0x0000_7770), vec![vec![parent, 0, 1]]);
    }
}

/// `00401030` (`cdecl`, one word): frees a block.
const FREE_00401030: u32 = 0x0040_1030;

// Translated from 0063f8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SkyObject` scalar deleting destructor (engine map; `thiscall`,
/// `RET 4`): `SkyObject::~SkyObject` (`0063f8d0`), then `00401030` frees
/// `this` when bit 0 of `flags` is set. Returns `this`.
pub fn sky_object_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<SkyObject>,
    flags: u32,
) -> Ptr<SkyObject> {
    sky_object_destructor(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00401030, &args![this]);
    }
    this
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0063_f8a0,
        sky_object_scalar_deleting_destructor(Ptr<SkyObject>, u32) -> Ptr<SkyObject>
    )]
}

#[cfg(test)]
mod n086_tests {
    use super::test_support::*;
    use super::*;

    #[test]
    fn scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(&mut e, &[NI_POINTER_DESTRUCT, FREE_00401030]);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_f8a0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), SKY_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT), vec![vec![this + 4]]);
        assert!(calls(&e, FREE_00401030).is_empty());
        e.call(0x0063_f8a0, &args![this, 3u32]);
        assert_eq!(calls(&e, FREE_00401030), vec![vec![this]]);
    }
}
