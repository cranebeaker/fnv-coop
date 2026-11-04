//! `fallout/ai/run_008d6af0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiPointer` assignment (`0066b0d0`, `thiscall (pointer, object)`),
/// the `NiPointer` destructor (`0045cec0`, `thiscall`), the pointer getter
/// (`00559450`, `thiscall`, answers the word at `this`) and the member
/// destructor of +8 (`00872f30`, `thiscall`).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
const POINTER_GET: u32 = 0x0055_9450;
const MEMBER_DESTRUCTOR: u32 = 0x0087_2f30;
/// `operator new` (`00aa13e0`, `cdecl (size)`) and the constructors of
/// the three objects `008d6bb0` makes: `BSMultiBoundSphere` (`00c4fb10`),
/// `BSMultiBound` (`00c35e00`) and `BSMultiBoundNode` (`00c46970`), all
/// `thiscall`.
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const MULTI_BOUND_SPHERE_CONSTRUCTOR: u32 = 0x00c4_fb10;
const MULTI_BOUND_CONSTRUCTOR: u32 = 0x00c3_5e00;
const MULTI_BOUND_NODE_CONSTRUCTOR: u32 = 0x00c4_6970;
/// The member at +8's two setters (`0041fd00` and `00a694a0`, `thiscall
/// (member, value)`), the value read from the object (`0045bbe0`,
/// `thiscall`), and the two links between the new objects (`004addc0` and
/// `00439920`, `thiscall (object, other)`).
const FIRST_SETTER: u32 = 0x0041_fd00;
const OBJECT_VALUE: u32 = 0x0045_bbe0;
const SECOND_SETTER: u32 = 0x00a6_94a0;
const LINK_SHAPE: u32 = 0x004a_ddc0;
const LINK_BOUND: u32 = 0x0043_9920;
/// The three `NiPointer`s: the sphere (+0xA0), the bound (+0x9C) and the
/// node (+0x98).
const SPHERE: u32 = 0xa0;
const BOUND: u32 = 0x9c;
const NODE: u32 = 0x98;

// Translated from 008d6af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d6af0 - its C++ exception frame (FS:[0]) is not translated
/// A destructor body (the engine map has no name): the vtable `01082cc0`,
/// the three `NiPointer`s at +0xA0, +0x9C and +0x98 set to null and then
/// destroyed in that order, then the member at +8 destroyed (`00872f30`).
pub fn fn_008d6af0(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, 0x0108_2cc0);
    for member in [SPHERE, BOUND, NODE] {
        e.call(POINTER_ASSIGN, &args![object + member, 0u32]);
    }
    for member in [SPHERE, BOUND, NODE] {
        e.call(POINTER_DESTRUCTOR, &args![object + member]);
    }
    e.call(MEMBER_DESTRUCTOR, &args![object + 8]);
}

/// `operator new(size)` then `constructor` on the block; null when the
/// allocation fails.
fn new_object(e: &mut Engine, size: u32, constructor: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    e.call(constructor, &args![block]).u32()
}

// Translated from 008d6bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d6bb0 - its C++ exception frame (FS:[0], the new objects' unwind) is not translated
/// The full set-up for `object` (the engine map has no name): the member
/// at +8 is given the object (`0041fd00`) and the object's value
/// (`0045bbe0`, `00a694a0`); a new `BSMultiBoundSphere` (0x1C bytes), a new
/// `BSMultiBound` (0x10) and a new `BSMultiBoundNode` (0xB4) go into the
/// pointers at +0xA0, +0x9C and +0x98 (null when an allocation fails); the
/// bound gets the sphere (`004addc0`) and the node the bound (`00439920`).
pub fn fn_008d6bb0(e: &mut Engine, this: Ptr, target: u32) {
    let object = this.addr();
    e.call(FIRST_SETTER, &args![object + 8, target]);
    let value = e.call(OBJECT_VALUE, &args![target]).u32();
    e.call(SECOND_SETTER, &args![object + 8, value]);
    let sphere = new_object(e, 0x1c, MULTI_BOUND_SPHERE_CONSTRUCTOR);
    e.call(POINTER_ASSIGN, &args![object + SPHERE, sphere]);
    let bound = new_object(e, 0x10, MULTI_BOUND_CONSTRUCTOR);
    e.call(POINTER_ASSIGN, &args![object + BOUND, bound]);
    let node = new_object(e, 0xb4, MULTI_BOUND_NODE_CONSTRUCTOR);
    e.call(POINTER_ASSIGN, &args![object + NODE, node]);
    let sphere = e.call(POINTER_GET, &args![object + SPHERE]).u32();
    let bound = e.call(POINTER_GET, &args![object + BOUND]).u32();
    e.call(LINK_SHAPE, &args![bound, sphere]);
    let bound = e.call(POINTER_GET, &args![object + BOUND]).u32();
    let node = e.call(POINTER_GET, &args![object + NODE]).u32();
    e.call(LINK_BOUND, &args![node, bound]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_6af0, fn_008d6af0(Ptr)),
        entry!(0x008d_6bb0, fn_008d6bb0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_clears_and_destroys_the_three_pointers() {
        let mut e = Engine::new();
        for addr in [POINTER_ASSIGN, POINTER_DESTRUCTOR, MEMBER_DESTRUCTOR] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xa4);
        e.call_log = Some(vec![]);
        e.call(0x008d_6af0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0108_2cc0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (POINTER_ASSIGN, vec![this + 0xa0, 0]),
                (POINTER_ASSIGN, vec![this + 0x9c, 0]),
                (POINTER_ASSIGN, vec![this + 0x98, 0]),
                (POINTER_DESTRUCTOR, vec![this + 0xa0]),
                (POINTER_DESTRUCTOR, vec![this + 0x9c]),
                (POINTER_DESTRUCTOR, vec![this + 0x98]),
                (MEMBER_DESTRUCTOR, vec![this + 8])
            ]
        );
    }

    #[test]
    fn set_up_makes_and_links_the_three_objects() {
        let mut e = Engine::new();
        for addr in [FIRST_SETTER, SECOND_SETTER, LINK_SHAPE, LINK_BOUND] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(OBJECT_VALUE, |_, _| 0x77u32.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for addr in [
            MULTI_BOUND_SPHERE_CONSTRUCTOR,
            MULTI_BOUND_CONSTRUCTOR,
            MULTI_BOUND_NODE_CONSTRUCTOR,
        ] {
            e.register(addr, |_, a| a[0].into_ret());
        }
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0xa4);
        e.call_log = Some(vec![]);
        e.call(0x008d_6bb0, &args![this, 0x5000u32]);
        let (sphere, bound, node) = (
            e.mem.u32(this + 0xa0),
            e.mem.u32(this + 0x9c),
            e.mem.u32(this + 0x98),
        );
        assert!(sphere != 0 && bound != 0 && node != 0);
        let log = e.call_log.take().unwrap();
        let sizes: Vec<u32> = log
            .iter()
            .filter(|call| call.0 == OPERATOR_NEW)
            .map(|call| call.1[0])
            .collect();
        assert_eq!(sizes, vec![0x1c, 0x10, 0xb4]);
        assert!(log
            .iter()
            .any(|call| *call == (SECOND_SETTER, vec![this + 8, 0x77])));
        assert!(log
            .iter()
            .any(|call| *call == (LINK_SHAPE, vec![bound, sphere])));
        assert!(log
            .iter()
            .any(|call| *call == (LINK_BOUND, vec![node, bound])));
        // Failed allocations leave null pointers.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call(0x008d_6bb0, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(this + 0xa0), 0);
        assert_eq!(e.mem.u32(this + 0x98), 0);
    }
}
