//! `(unplaced)/run_004ed8c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0049fa80` (`thiscall`, no stack word, unit `run_0049f8b0`): answers a new list node.
const NEW_NODE: u32 = 0x0049_fa80;
/// `00559a70` (`thiscall`, `RET 4`; engine-map unit `tesobjectcell.cpp`): links the node.
const LINK_NODE: u32 = 0x0055_9a70;

// Translated from 004ed8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`, the address of an item): a node from
/// `0049fa80` gets the item word at its `+8` and is handed to `00559a70` (both on `this`).
pub fn fn_004ed8c0(e: &mut Engine, this: Ptr, item: Ptr) {
    let node = e.call(NEW_NODE, &args![this]).u32();
    let value = e.mem.u32(item.addr());
    e.mem.set_u32(node + 8, value);
    e.call(LINK_NODE, &args![this, node]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004e_d8c0, fn_004ed8c0(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_new_node_gets_the_item_and_is_linked() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0x10);
        e.register_double(NEW_NODE, move |_, _| node.into_ret());
        e.register(LINK_NODE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0xabcd);
        e.call(0x004e_d8c0, &args![0x5000u32, item]);
        assert_eq!(e.mem.u32(node + 8), 0xabcd);
        let links: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == LINK_NODE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(links, vec![vec![0x5000, node]]);
    }
}
