//! `(unplaced)/run_007b5370` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions work on a list with the head node at `+0`, the tail node at `+4` and a
//! count at `+8`; a node holds its next node at `+0`, its previous node at `+4` and its
//! item at `+8` (as the code uses them; the class was not confirmed).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_facegen::bsfacegenkeyframe::speech_check;

/// `004a49e0` (`thiscall`, `RET 4`, no unit in the engine map): frees a node of the list.
const FREE_NODE: u32 = 0x004a_49e0;

// Translated from 007b5370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the address of the tail node's item, `tail + 8`.
pub fn fn_007b5370(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 4).wrapping_add(8)
}

// Translated from 007b5390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): removes the head node and answers its item.
/// The next node becomes the head (its previous link cleared), or the tail is cleared when
/// there is none; the old head is freed (`004a49e0`) and the count goes down by one.
pub fn fn_007b5390(e: &mut Engine, this: Ptr) -> u32 {
    let list = this.addr();
    // `NV_SPEECH_CHECK` (diagnostic only, reads memory): a watched list's head removed.
    speech_check::removing(e, list);
    let node = e.mem.u32(list);
    let next = e.mem.u32(e.mem.u32(list));
    e.mem.set_u32(list, next);
    if e.mem.u32(list) == 0 {
        e.mem.set_u32(list + 4, 0);
    } else {
        let head = e.mem.u32(list);
        e.mem.set_u32(head + 4, 0);
    }
    let item = e.mem.u32(node + 8);
    e.call(FREE_NODE, &args![this, node]);
    let count = e.mem.u32(list + 8).wrapping_sub(1);
    e.mem.set_u32(list + 8, count);
    speech_check::removed(e, list);
    item
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007b_5370, fn_007b5370(Ptr) -> u32),
        entry!(0x007b_5390, fn_007b5390(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_tail_item_address() {
        let mut e = Engine::new();
        let list = e.mem.alloc(0xc);
        e.mem.set_u32(list + 4, 0x6000);
        assert_eq!(e.call(0x007b_5370, &args![list]).u32(), 0x6008);
    }

    #[test]
    fn popping_the_head() {
        let mut e = Engine::new();
        e.register(FREE_NODE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let (first, second) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(second + 4, first);
        e.mem.set_u32(second + 8, 0x22);
        let list = e.mem.alloc(0xc);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        assert_eq!(e.call(0x007b_5390, &args![list]).u32(), 0x11);
        assert_eq!((e.mem.u32(list), e.mem.u32(second + 4)), (second, 0));
        assert_eq!(e.mem.u32(list + 8), 1);
        assert_eq!(e.call(0x007b_5390, &args![list]).u32(), 0x22);
        assert_eq!((e.mem.u32(list), e.mem.u32(list + 4)), (0, 0));
        assert_eq!(e.mem.u32(list + 8), 0);
        let freed: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == FREE_NODE)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(freed, vec![first, second]);
    }
}
