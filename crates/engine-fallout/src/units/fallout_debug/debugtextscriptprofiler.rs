//! `fallout/debug/debugtextpages/debugtextscriptprofiler.cpp` (Xbox PDB source unit), subsystem `fallout/debug`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The list's node free (`004a49e0`, `thiscall (list, node)`).
const FREE_NODE: u32 = 0x004a_49e0;
/// The list's node allocation (`0049fa80`, `thiscall (list)`, answers the
/// node) and the head insertion (`0049fa20`, `thiscall (list, node)`).
const NEW_NODE: u32 = 0x0049_fa80;
const ADD_NODE_HEAD: u32 = 0x0049_fa20;

// Translated from 004ed900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>, Script*>::RemoveAll`
/// (Xbox PDB): frees every node from the head (`004a49e0`, each node's next
/// read before it is freed), then clears the count (+8), head (+0) and tail
/// (+4).
pub fn ni_t_pointer_list_base_remove_all(e: &mut Engine, this: Ptr) {
    let mut node = e.mem.u32(this.addr());
    while node != 0 {
        let current = node;
        node = e.mem.u32(node);
        e.call(FREE_NODE, &args![this, current]);
    }
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
}

// Translated from 0076b660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>, Script*>::AddHead`
/// (Xbox PDB): a new node (`0049fa80`) holding the element `*element` (at
/// node +8), put at the head (`0049fa20`).
pub fn ni_t_pointer_list_base_add_head(e: &mut Engine, this: Ptr, element: Ptr) {
    let node = e.call(NEW_NODE, &args![this]).u32();
    let value = e.mem.u32(element.addr());
    e.mem.set_u32(node + 8, value);
    e.call(ADD_NODE_HEAD, &args![this, node]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004ed900, ni_t_pointer_list_base_remove_all(Ptr)),
        entry!(0x0076b660, ni_t_pointer_list_base_add_head(Ptr, Ptr)),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_all_frees_every_node_then_clears_the_list() {
        let mut e = Engine::new();
        e.register(FREE_NODE, |e, a| {
            // The freed node's link is gone: the walk must have read it first.
            e.mem.set_u32(a[1], 0xdead_0000);
            Ret::default()
        });
        let list = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x004ed900, &args![list]);
        let frees: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == FREE_NODE)
            .map(|c| c.1)
            .collect();
        assert_eq!(frees, vec![vec![list, first], vec![list, second]]);
        assert_eq!(
            (e.mem.u32(list), e.mem.u32(list + 4), e.mem.u32(list + 8)),
            (0, 0, 0)
        );
    }

    #[test]
    fn add_head_stores_the_element_in_a_new_node() {
        let mut e = Engine::new();
        e.map(0x7300_0000, 0x10);
        e.register(NEW_NODE, |_, _| 0x7300_0000u32.into_ret());
        e.register(ADD_NODE_HEAD, |_, _| Ret::default());
        let list = e.mem.alloc(0xc);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0076b660, &args![list, element]);
        assert_eq!(e.mem.u32(0x7300_0008), 0x1234);
        let log = e.call_log.take().unwrap();
        let add = log.iter().find(|c| c.0 == ADD_NODE_HEAD).unwrap();
        assert_eq!(add.1, [list, 0x7300_0000]);
    }
}

/// The list's insertion before a node (`004ede90`, `thiscall (list,
/// before, node)`).
const INSERT_NODE_BEFORE: u32 = 0x004e_de90;

// Translated from 007b5330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>, Script*>::InsertBefore`
/// (engine map): a new node (`0049fa80`) holding the element `*element` (at
/// node +8), inserted before `before` (`004ede90`), whose answer is passed
/// on.
pub fn ni_t_pointer_list_base_insert_before(
    e: &mut Engine,
    this: Ptr,
    before: u32,
    element: Ptr,
) -> u32 {
    let node = e.call(NEW_NODE, &args![this]).u32();
    let value = e.mem.u32(element.addr());
    e.mem.set_u32(node + 8, value);
    e.call(INSERT_NODE_BEFORE, &args![this, before, node]).u32()
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x007b_5330,
        ni_t_pointer_list_base_insert_before(Ptr, u32, Ptr) -> u32
    )]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

    #[test]
    fn insert_before_links_a_new_node() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0xc);
        e.register_double(NEW_NODE, move |_, _| node.into_ret());
        e.register(INSERT_NODE_BEFORE, |_, a| a[2].into_ret());
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 0x5c5c);
        e.call_log = Some(vec![]);
        let answer = e.call(0x007b_5330, &args![0x100u32, 0x200u32, element]);
        assert_eq!(answer.u32(), node);
        assert_eq!(e.mem.u32(node + 8), 0x5c5c);
        let inserted: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == INSERT_NODE_BEFORE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(inserted, vec![vec![0x100, 0x200, node]]);
    }
}
