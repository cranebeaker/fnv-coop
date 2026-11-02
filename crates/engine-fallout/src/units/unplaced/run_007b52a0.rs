//! `(unplaced)/run_007b52a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007b52d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`; the `this` register is saved and never read.
/// Answers the dword at the start of the node (its next-node pointer), or null for a null node.
pub fn fn_007b52d0(e: &mut Engine, _unused_this: Ptr, node: Ptr) -> u32 {
    if node.is_null() {
        0
    } else {
        e.mem.u32(node.addr())
    }
}

/// `00730490` (`cdecl`: pointer, size; it hands both to `00ecbc5b`).
const BUFFER_RELEASE_00730490: u32 = 0x0073_0490;

// Translated from 007b52a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words) on a pointer at
/// +0 with a 16-bit length at +4: when the pointer is set, hands it and the
/// length plus one to `00730490`. The pointer and length are left as they
/// are.
pub fn fn_007b52a0(e: &mut Engine, this: Ptr) {
    let buffer = e.mem.u32(this.addr());
    if buffer != 0 {
        let length = u32::from(e.mem.u16(this.addr() + 4)) + 1;
        e.call(BUFFER_RELEASE_00730490, &args![buffer, length]);
    }
}

// Translated from 007b5300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the `this` register is
/// saved and never read): the word at +4 of `node` (its item, next to the
/// next-node pointer `007b52d0` reads), or null for a null node.
pub fn fn_007b5300(e: &mut Engine, _unused_this: Ptr, node: Ptr) -> u32 {
    if node.is_null() {
        0
    } else {
        e.mem.u32(node.addr() + 4)
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007b_52d0, fn_007b52d0(Ptr, Ptr) -> u32),
        entry!(0x007b52a0, fn_007b52a0(Ptr)),
        entry!(0x007b5300, fn_007b5300(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn releases_a_set_buffer_with_its_length_plus_one() {
        let mut e = Engine::new();
        e.register(BUFFER_RELEASE_00730490, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x007b_52a0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == BUFFER_RELEASE_00730490));
        e.mem.set_u32(this, 0x4000);
        e.mem.set_u16(this + 4, 0xffff);
        e.call_log = Some(vec![]);
        e.call(0x007b_52a0, &args![this]);
        let log = e.call_log.take().unwrap();
        let released: Vec<_> = log
            .iter()
            .filter(|c| c.0 == BUFFER_RELEASE_00730490)
            .collect();
        assert_eq!(released[0].1, [0x4000, 0x1_0000]);
        assert_eq!(e.mem.u32(this), 0x4000);
    }

    #[test]
    fn answers_the_item_of_the_node_or_null() {
        let mut e = Engine::new();
        let node = e.mem.alloc(8);
        e.mem.set_u32(node + 4, 0x8000);
        assert_eq!(e.call(0x007b_5300, &args![0u32, node]).u32(), 0x8000);
        assert_eq!(e.call(0x007b_5300, &args![node, 0u32]).u32(), 0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_007b52d0_answers_the_first_dword_of_the_node_or_null() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, 0x7000);
        e.mem.set_u32(node + 4, 0x8000);
        assert_eq!(e.call(0x007b_52d0, &args![0x1234u32, node]).u32(), 0x7000);
        // A null node is answered with null, without reading memory (address 0 is unmapped).
        assert_eq!(e.call(0x007b_52d0, &args![node, 0u32]).u32(), 0);
    }
}
