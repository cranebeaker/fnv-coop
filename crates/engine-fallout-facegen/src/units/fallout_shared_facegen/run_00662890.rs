//! `fallout shared/facegen/run_00662890` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The statistical morph collection's size (`0065cc30`, `ECX` = it) and
/// the CRT's invalid-index report (`00ec7c56`).
const STATISTICAL_COUNT: u32 = 0x0065_cc30;
const INVALID_INDEX: u32 = 0x00ec_7c56;
/// `NiNode`'s constructor body (`00a5ecb0`: `this`; 0).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `BSFaceGenNiNode`'s vtable (RTTI `.?AVBSFaceGenNiNode@@`).
const FACE_GEN_NI_NODE_VTABLE: u32 = 0x0106_60dc;
/// The member at +0xAC: its constructor (`00633c90`: `this`; 0) and its
/// setter run again after the flags (`0066b0d0`: `this`; 0).
const MEMBER_CONSTRUCT: u32 = 0x0063_3c90;
const MEMBER_SET: u32 = 0x0066_b0d0;
/// The constructor run on +0xB0 (`006815c0`, result unused).
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// The `float` -1.0 stored at +0xDC.
const MINUS_ONE: u32 = 0x0101_2054;
/// The nine words copied to +0xB0 (`011a9448`).
const DEFAULT_WORDS: u32 = 0x011a_9448;

// Translated from 00662890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of a statistical morph collection (0x38 bytes each,
/// array at +0xC): reports an index at or past the count to the CRT
/// (`00ec7c56`) and returns the address anyway.
pub fn fn_00662890(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let count = e.call(STATISTICAL_COUNT, &args![this]).u32();
    if index >= count {
        e.call(INVALID_INDEX, &[]);
    }
    index
        .wrapping_mul(0x38)
        .wrapping_add(e.mem.u32(this.addr().wrapping_add(0xc)))
}

// Translated from 006628c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenNiNode`'s constructor: the `NiNode` part, the vtable, the
/// member at +0xAC, the flags at +0xD4..+0xD8 (0, 0, 1, 1, 0), -1.0 at
/// +0xDC, two more cleared flags, the member's setter, the nine default
/// words at +0xB0 (`011a9448`), and +0xE2, +0xE8, +0xEC cleared. Returns
/// `this`.
// Translation incomplete: 006628c0 - SEH exception frame (FS:[0]) that destroys the NiNode part on unwinding is not modelled.
pub fn fn_006628c0(e: &mut Engine, this: Ptr) -> Ptr {
    let at = |offset: u32| this.addr().wrapping_add(offset);
    e.call(NI_NODE_CONSTRUCT, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), FACE_GEN_NI_NODE_VTABLE);
    e.call(MEMBER_CONSTRUCT, &args![at(0xac), 0u32]);
    e.call(POINT_CONSTRUCT, &args![at(0xb0)]);
    e.mem.set_u8(at(0xd5), 0);
    e.mem.set_u8(at(0xd4), 0);
    e.mem.set_u8(at(0xd6), 1);
    e.mem.set_u8(at(0xd7), 1);
    e.mem.set_u8(at(0xd8), 0);
    let value = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    e.mem.set_u32(at(0xdc), value);
    e.mem.set_u8(at(0xe0), 0);
    e.mem.set_u8(at(0xe1), 0);
    e.call(MEMBER_SET, &args![at(0xac), 0u32]);
    for i in 0..9u32 {
        let word = e.mem.u32(DEFAULT_WORDS + i * 4);
        e.mem.set_u32(at(0xb0 + i * 4), word);
    }
    e.mem.set_u8(at(0xe2), 0);
    e.mem.set_u32(at(0xe8), 0);
    e.mem.set_u8(at(0xec), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_2890, fn_00662890(Ptr, u32) -> u32),
        entry!(0x0066_28c0, fn_006628c0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statistical_entry_reports_a_bad_index() {
        let mut e = Engine::new();
        e.register(STATISTICAL_COUNT, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(INVALID_INDEX, |_, _| Ret::default());
        let collection = e.mem.alloc(0x10);
        e.mem.set_u32(collection, 1);
        e.mem.set_u32(collection + 0xc, 0x2000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_2890, &args![collection, 0u32]).u32(), 0x2000);
        assert_eq!(fn_00662890(&mut e, Ptr::new(collection), 3), 0x20a8);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|(a, _)| *a == INVALID_INDEX).count(), 1);
    }

    #[test]
    fn node_constructor_sets_the_defaults() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.set_global(MINUS_ONE, -1.0f32);
        for i in 0..9u32 {
            e.mem.set_u32(DEFAULT_WORDS + i * 4, 0x100 + i);
        }
        for f in [
            NI_NODE_CONSTRUCT,
            MEMBER_CONSTRUCT,
            MEMBER_SET,
            POINT_CONSTRUCT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        let node = e.mem.alloc(0xf0);
        e.mem.write(node, &[0xffu8; 0xf0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_28c0, &args![node]).u32(), node);
        assert_eq!(e.mem.u32(node), FACE_GEN_NI_NODE_VTABLE);
        assert_eq!(e.mem.bytes(node + 0xd4, 5), vec![0, 0, 1, 1, 0]);
        assert_eq!(e.mem.f32(node + 0xdc), -1.0);
        assert_eq!(e.mem.u32(node + 0xb0), 0x100);
        assert_eq!(e.mem.u32(node + 0xd0), 0x108);
        assert_eq!(e.mem.u32(node + 0xe8), 0);
        assert_eq!(e.mem.u8(node + 0xec), 0);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                0x0066_28c0,
                NI_NODE_CONSTRUCT,
                MEMBER_CONSTRUCT,
                POINT_CONSTRUCT,
                MEMBER_SET
            ]
        );
    }
}
