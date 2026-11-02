//! `fallout shared/pathfinding/run_006e3c90` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0`: placement `operator new` (`cdecl`: size, address; returns
/// the address).
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `00692710`: the 0x24-byte element's constructor (`thiscall`).
const ELEMENT_CONSTRUCT: u32 = 0x0069_2710;

// Translated from 006e3c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e3c90 - the C++ exception frame (FS:[0]) is not translated
/// No map name: constructs `count` 0x24-byte elements at `elements`
/// (placement `new`, then `00692710` on each non-null result). `this` is
/// kept but not read.
pub fn fn_006e3c90(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let element = e
            .call(
                PLACEMENT_NEW,
                &args![0x24u32, index.wrapping_mul(0x24).wrapping_add(elements)],
            )
            .u32();
        if element != 0 {
            e.call(ELEMENT_CONSTRUCT, &args![element]);
        }
        index = index.wrapping_add(1);
    }
}

/// `006e3e70` (`thiscall`, `RET 0xc`: destination, source, count): moves
/// 0x24-byte elements in the safe direction.
const MOVE_ELEMENTS_006E3E70: u32 = 0x006e_3e70;
/// `006a8500` (`thiscall`): run on the array once the elements have moved
/// to the new buffer.
const ARRAY_RELEASE_006A8500: u32 = 0x006a_8500;

// Translated from 006e3d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the array of 0x24-byte elements (buffer `+4`, capacity `+0xc`) a
/// buffer of `capacity` (`thiscall`; capacity, count; `RET 8`; no name in
/// the engine map): without a buffer, virtual slot 4 with the capacity,
/// which is also stored; when `count` equals the capacity, slot 0xc with the
/// old buffer and the capacity; else a new buffer from slot 4, the `count`
/// elements moved (`006e3e70`), `006a8500` on the array and the new buffer
/// kept.
pub fn fn_006e3d30(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    let base = this.addr();
    if e.mem.u32(base + 4) == 0 {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    } else if count == e.mem.u32(base + 0xc) {
        let old = e.mem.u32(base + 4);
        let buffer = e.vcall(base, 0xc, &args![old, capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
    } else {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        let old = e.mem.u32(base + 4);
        e.call(MOVE_ELEMENTS_006E3E70, &args![this, buffer, old, count]);
        e.call(ARRAY_RELEASE_006A8500, &args![this]);
        e.mem.set_u32(base + 4, buffer);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_3c90, fn_006e3c90(Ptr, u32, u32)),
        entry!(0x006e_3d30, fn_006e3d30(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_each_element() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| a[1].into_ret());
        e.register(ELEMENT_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006e_3c90, &args![0u32, 0x5000u32, 2u32]);
        let log = e.call_log.take().unwrap();
        let built: Vec<_> = log
            .iter()
            .filter(|c| c.0 == ELEMENT_CONSTRUCT)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(built, vec![0x5000, 0x5024]);
        e.register(PLACEMENT_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006e_3c90, &args![0u32, 0x5000u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != ELEMENT_CONSTRUCT));
    }

    #[test]
    fn at_006e3d30_allocates_reallocates_or_moves() {
        let mut e = Engine::new();
        e.register(MOVE_ELEMENTS_006E3E70, |_, _| Ret::default());
        e.register(ARRAY_RELEASE_006A8500, |_, _| Ret::default());
        e.register(0x0f00_0004, |_, a| (0x9000 + a[1]).into_ret());
        e.register(0x0f00_000c, |_, a| (0xa000 + a[2]).into_ret());
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_0004, 0, 0x0f00_000c]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0f00_1000);
        e.call(0x006e_3d30, &args![this, 6u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0x9006);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        e.call(0x006e_3d30, &args![this, 9u32, 6u32]);
        assert_eq!(e.mem.u32(this + 4), 0xa009);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        e.call_log = Some(vec![]);
        e.call(0x006e_3d30, &args![this, 0x10u32, 3u32]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x006e_3d30)
            .collect();
        assert_eq!(
            log,
            vec![
                (0x0f00_0004, vec![this, 0x10]),
                (MOVE_ELEMENTS_006E3E70, vec![this, 0x9010, 0xa009, 3]),
                (ARRAY_RELEASE_006A8500, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this + 4), 0x9010);
    }
}
