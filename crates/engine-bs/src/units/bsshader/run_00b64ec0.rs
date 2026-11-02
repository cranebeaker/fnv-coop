//! `BSShader/run_00b64ec0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleList;

/// The memory manager object the code passes as `this` to its free.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's free method (thiscall, one word: the block).
const MEMORY_FREE: u32 = 0x00aa_4060;

// Translated from 00b64ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clear of a `BSSimpleList<unsigned short>` (unnamed in the engine map;
/// `BSBatchRenderer::kActivePassIndexList`): while the list has a next
/// node, unlinks it (keeping the node's own next), clears the node's list
/// (this function again) and frees it, then clears the 16-bit item.
pub fn fn_00b64ec0(e: &mut Engine, this: Ptr<BSSimpleList>) {
    if e.get(this, BSSimpleList::m_pkNext) != 0 {
        loop {
            let node = e.get(this, BSSimpleList::m_pkNext);
            let after = e.mem.u32(node + 4);
            e.mem.set_u32(node + 4, 0);
            let node = e.get(this, BSSimpleList::m_pkNext);
            if node != 0 {
                fn_00b64ec0(e, Ptr::new(node));
                e.call(MEMORY_FREE, &[MEMORY_MANAGER, node]);
            }
            e.set(this, BSSimpleList::m_pkNext, after);
            if after == 0 {
                break;
            }
        }
    }
    e.mem.set_u16(this.addr(), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b6_4ec0, fn_00b64ec0(Ptr<BSSimpleList>))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_frees_every_node_and_clears_the_item() {
        let mut e = Engine::new();
        e.register(MEMORY_FREE, |_, _| 0u32.into_ret());
        let list = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(list, 0xaaaa_0005);
        e.mem.set_u32(list + 4, first);
        e.mem.set_u32(first, 6);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, 7);
        e.mem.set_u32(second + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b6_4ec0, &[list]);
        let frees: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMORY_FREE)
            .map(|(_, w)| w[1])
            .collect();
        assert_eq!(frees, vec![first, second]);
        assert_eq!(e.mem.u32(list + 4), 0);
        // Only the 16-bit item is cleared; the word's upper half stays.
        assert_eq!(e.mem.u32(list), 0xaaaa_0000);
        // Each freed node had its next link cleared and its item cleared.
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u16(first), 0);
        assert_eq!(e.mem.u16(second), 0);
        // An empty list only clears its item.
        e.mem.set_u16(list, 9);
        e.call_log = Some(vec![]);
        e.call(0x00b6_4ec0, &[list]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u16(list), 0);
    }
}
