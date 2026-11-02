//! `BSShader/bsfadenodeculler.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAVObject::GetWorldBound` (`0043d450`, `thiscall`): the bound (centre
/// +0, radius +0xC).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// The object's vtable slot +0x10 (`IsFadeNode` in the Xbox PDB).
const IS_FADE_NODE_SLOT: u32 = 0x10;
/// The six planes the culler tests (+0x2C: normal x, y, z and the constant,
/// 0x10 bytes each, as the PC code reads them).
const PLANES: u32 = 0x2c;
/// `NiAVObject::m_uFlags` (Xbox PDB) +0x30.
const OBJECT_FLAGS: u32 = 0x30;

// Translated from 00bc2b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFadeNodeCuller::Process` (Xbox PDB): for a fade node (the object's
/// slot +0x10), sets bit 12 of its flags, then tests its world bound against
/// the six planes: when the bound lies wholly behind one (the plane's
/// distance of the centre at most minus the radius) bit 20 of the flags is
/// set, otherwise cleared. Other objects are left alone. The x87 operations
/// follow the exe (the distance passes through a `float` slot twice).
pub fn bs_fade_node_culler_process(e: &mut Engine, this: Ptr, object: u32) {
    let node = e.vcall(object, IS_FADE_NODE_SLOT, &[]).u32();
    if node == 0 {
        return;
    }
    let flags = e.mem.u32(node + OBJECT_FLAGS) | 0x1000;
    e.mem.set_u32(node + OBJECT_FLAGS, flags);
    let flags = e.mem.u32(node + OBJECT_FLAGS);
    let mut plane = 0u32;
    while plane < 6 {
        let bound = e.call(GET_WORLD_BOUND, &args![node]).u32();
        let normal = this.addr() + PLANES + 0x10 * plane;
        let y = x87_mul(
            x87_load32(e.mem.u32(normal + 4)),
            x87_m32(e.mem.u32(bound + 4)),
        );
        let x = x87_mul(x87_load32(e.mem.u32(normal)), x87_m32(e.mem.u32(bound)));
        let sum = x87_add(y, x);
        let z = x87_mul(
            x87_load32(e.mem.u32(bound + 8)),
            x87_m32(e.mem.u32(normal + 8)),
        );
        let dot = x87_store32(x87_add(sum, z));
        let distance = x87_store32(x87_sub(x87_load32(dot), x87_m32(e.mem.u32(normal + 0xc))));
        let below = -x87_load32(e.mem.u32(bound + 0xc));
        // `FCHS; FCOMPP; TEST AH,1; JZ`: behind when minus the radius is
        // not below the distance (ordered).
        if below >= x87_load32(distance) {
            break;
        }
        plane += 1;
    }
    let flags = if plane == 6 {
        flags & 0xffef_ffff
    } else {
        flags | 0x0010_0000
    };
    e.mem.set_u32(node + OBJECT_FLAGS, flags);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_2b30, bs_fade_node_culler_process(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A culler whose six planes all face +x with constant `-10` (a point
    /// is in front when x > -10), a fade node with its bound at
    /// `(x, 0, 0)` radius 1.
    fn culler(x: f32, fade: bool) -> (Engine, Ptr, u32, u32) {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x120));
        for plane in 0..6 {
            e.mem.set_f32(this.addr() + PLANES + 0x10 * plane, 1.0);
            e.mem
                .set_f32(this.addr() + PLANES + 0x10 * plane + 0xc, -10.0);
        }
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound, x);
        e.mem.set_f32(bound + 0xc, 1.0);
        e.register_double(GET_WORLD_BOUND, move |_, _| bound.into_ret());
        let node = e.mem.alloc(0x100);
        e.mem.set_u32(node + OBJECT_FLAGS, 0x0010_0000);
        let answer = if fade { node } else { 0 };
        e.register_double(0x0040_1010, move |_, _| answer.into_ret());
        e.put_vtable(0x0090_0000, &[0, 0, 0, 0, 0x0040_1010]);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0090_0000);
        (e, this, object, node)
    }

    #[test]
    fn a_bound_in_front_of_every_plane_clears_bit_20() {
        let (mut e, this, object, node) = culler(0.0, true);
        e.call_log = Some(vec![]);
        e.call(0x00bc_2b30, &args![this, object]);
        assert_eq!(e.mem.u32(node + OBJECT_FLAGS), 0x1000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == GET_WORLD_BOUND).count(), 6);
    }

    #[test]
    fn a_bound_behind_a_plane_sets_bit_20() {
        // Centre at -11, radius 1: distance -1 is at most -1.
        let (mut e, this, object, node) = culler(-20.0, true);
        e.mem.set_u32(node + OBJECT_FLAGS, 0);
        e.call_log = Some(vec![]);
        e.call(0x00bc_2b30, &args![this, object]);
        assert_eq!(e.mem.u32(node + OBJECT_FLAGS), 0x0010_1000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == GET_WORLD_BOUND).count(), 1);
    }

    #[test]
    fn other_objects_are_left_alone() {
        let (mut e, this, object, node) = culler(0.0, false);
        e.call(0x00bc_2b30, &args![this, object]);
        assert_eq!(e.mem.u32(node + OBJECT_FLAGS), 0x0010_0000);
    }
}
