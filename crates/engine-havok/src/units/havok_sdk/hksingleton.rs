//! `Havok SDK/hksingleton.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `hkSingletonInitNode` is four words: the singleton's name (a C string), a word nobody here
//! reads, the next node (`+8`) and the address of the variable that holds the singleton
//! pointer (`+0xc`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The string map's constructor (`thiscall`, one stack word the constructor ignores) and destructor.
const STRING_MAP_CONSTRUCTOR: u32 = 0x00d8_7790;
const STRING_MAP_DESTRUCTOR: u32 = 0x00d8_77d0;
/// `hkCachedHashMap<hkStringMapOperations,...>::insert` (`thiscall`: key, value).
const STRING_MAP_INSERT: u32 = 0x00d8_74d0;
/// `hkCachedHashMap<hkStringMapOperations,...>::getWithDefault` (`thiscall`: key, default).
const STRING_MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
/// `hkReferencedObject::addReferenceLockUnchecked` and `removeReferenceLockUnchecked`
/// (`thiscall`, no stack words).
const ADD_REFERENCE_LOCK_UNCHECKED: u32 = 0x00c9_03a0;
const REMOVE_REFERENCE_LOCK_UNCHECKED: u32 = 0x00c9_03b0;

/// Builds the string map of `sources` (name to node) in a local map at `map`, as both functions
/// do first: the constructor (its one stack word is the byte `0` over uninitialised stack bytes;
/// the constructor never reads it) and an `insert` for every node of the list.
fn build_map(e: &mut Engine, map: u32, sources: u32) {
    e.call(STRING_MAP_CONSTRUCTOR, &args![map, 0u32]);
    let mut node = sources;
    while node != 0 {
        let name = e.mem.u32(node);
        e.call(STRING_MAP_INSERT, &args![map, name, node]);
        node = e.mem.u32(node + 8);
    }
}

// Translated from 00cf6a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSingletonInitNode::populate` (Xbox PDB), cdecl, two words `(targets, sources)`: builds a
/// string map of the `sources` list (name to node), then for every node of `targets` whose name
/// is in the map stores the singleton the source node's variable holds into the target node's
/// variable and takes a reference on it when it is not null
/// (`addReferenceLockUnchecked`). Destroys the map.
pub fn hk_singleton_init_node_populate(e: &mut Engine, targets: u32, sources: u32) {
    // The exe's frame (`SUB ESP,0x14`, one push): 0x18 bytes, the map at +0x8 (00cf6a0e).
    e.with_stack(0x18, |e, frame| {
        let map = frame.addr() + 8;
        build_map(e, map, sources);
        let mut node = targets;
        while node != 0 {
            let name = e.mem.u32(node);
            let found = e
                .call(STRING_MAP_GET_WITH_DEFAULT, &args![map, name, 0u32])
                .u32();
            if found != 0 {
                let source_variable = e.mem.u32(found + 0xc);
                let singleton = e.mem.u32(source_variable);
                let target_variable = e.mem.u32(node + 0xc);
                e.mem.set_u32(target_variable, singleton);
                let target_variable = e.mem.u32(node + 0xc);
                if e.mem.u32(target_variable) != 0 {
                    let singleton = e.mem.u32(target_variable);
                    e.call(ADD_REFERENCE_LOCK_UNCHECKED, &args![singleton]);
                }
            }
            node = e.mem.u32(node + 8);
        }
        e.call(STRING_MAP_DESTRUCTOR, &args![map]);
    });
}

// Translated from 00cf6a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSingletonInitNode::depopulate` (Xbox PDB), cdecl, two words `(targets, sources)`: builds
/// the same map, then for every node of `targets` whose name is in it releases the singleton
/// its variable holds (`removeReferenceLockUnchecked`, when not null) and sets the variable to
/// 0. Destroys the map.
pub fn hk_singleton_init_node_depopulate(e: &mut Engine, targets: u32, sources: u32) {
    // The exe's frame (`SUB ESP,0x14`, one push): 0x18 bytes, the map at +0x8 (00cf6a9e).
    e.with_stack(0x18, |e, frame| {
        let map = frame.addr() + 8;
        build_map(e, map, sources);
        let mut node = targets;
        while node != 0 {
            let name = e.mem.u32(node);
            let found = e
                .call(STRING_MAP_GET_WITH_DEFAULT, &args![map, name, 0u32])
                .u32();
            if found != 0 {
                let variable = e.mem.u32(node + 0xc);
                if e.mem.u32(variable) != 0 {
                    let singleton = e.mem.u32(variable);
                    e.call(REMOVE_REFERENCE_LOCK_UNCHECKED, &args![singleton]);
                }
                let variable = e.mem.u32(node + 0xc);
                e.mem.set_u32(variable, 0);
            }
            node = e.mem.u32(node + 8);
        }
        e.call(STRING_MAP_DESTRUCTOR, &args![map]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf6a00, hk_singleton_init_node_populate(u32, u32)),
        entry!(0x00cf6a90, hk_singleton_init_node_depopulate(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    /// An engine whose string map is a hash map keyed by the name pointer's text.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let map: Rc<RefCell<HashMap<Vec<u8>, u32>>> = Rc::new(RefCell::new(HashMap::new()));
        e.register(STRING_MAP_CONSTRUCTOR, |_, _| Ret::default());
        e.register(STRING_MAP_DESTRUCTOR, |_, _| Ret::default());
        let inserts = Rc::clone(&map);
        e.register_double(STRING_MAP_INSERT, move |e, a| {
            let key = e.mem.cstr(a[1]);
            inserts.borrow_mut().insert(key, a[2]);
            Ret::default()
        });
        e.register_double(STRING_MAP_GET_WITH_DEFAULT, move |e, a| {
            let key = e.mem.cstr(a[1]);
            Ret {
                eax: map.borrow().get(&key).copied().unwrap_or(a[2]),
                ..Ret::default()
            }
        });
        e.register(ADD_REFERENCE_LOCK_UNCHECKED, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE_LOCK_UNCHECKED, |_, _| Ret::default());
        e
    }

    /// A node with `name`, `next` and a variable holding `value`; returns the node and the
    /// variable.
    fn node(e: &mut Engine, name: &str, next: u32, value: u32) -> (u32, u32) {
        let text = e.mem.alloc(name.len() as u32 + 1);
        e.mem.set_cstr(text, name.as_bytes());
        let variable = e.mem.alloc(4);
        e.mem.set_u32(variable, value);
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, text);
        e.mem.set_u32(node + 8, next);
        e.mem.set_u32(node + 0xc, variable);
        (node, variable)
    }

    #[test]
    fn populate_copies_matching_singletons_and_takes_references() {
        let mut e = engine();
        // Sources: "memory" holds 0x5001, "stream" holds 0 (nothing to copy).
        let (stream_source, _) = node(&mut e, "stream", 0, 0);
        let (memory_source, _) = node(&mut e, "memory", stream_source, 0x5001);
        // Targets: "memory", "stream" and "unknown" (not among the sources).
        let (unknown, unknown_variable) = node(&mut e, "unknown", 0, 0x7777);
        let (stream, stream_variable) = node(&mut e, "stream", unknown, 0x6666);
        let (memory, memory_variable) = node(&mut e, "memory", stream, 0);
        e.call_log = Some(vec![]);
        // The map sits at +0x8 of the exe's 0x18-byte frame.
        let frame = e.next_stack_local(0x18);
        e.call(0x00cf_6a00, &args![memory, memory_source]);
        let lookup = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|c| c.0 == STRING_MAP_GET_WITH_DEFAULT)
            .unwrap()
            .1[0];
        assert_eq!(lookup, frame + 8);
        assert_eq!(e.mem.u32(memory_variable), 0x5001);
        // The matching node takes the (null) singleton of its source; a null one takes no reference.
        assert_eq!(e.mem.u32(stream_variable), 0);
        // A node whose name is not among the sources is left alone.
        assert_eq!(e.mem.u32(unknown_variable), 0x7777);
        let log = e.call_log.take().unwrap();
        let added: Vec<_> = log
            .iter()
            .filter(|c| c.0 == ADD_REFERENCE_LOCK_UNCHECKED)
            .collect();
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].1, vec![0x5001]);
        // The map is built (constructor, two inserts) and destroyed once.
        assert_eq!(
            log.iter().filter(|c| c.0 == STRING_MAP_CONSTRUCTOR).count(),
            1
        );
        assert_eq!(log.iter().filter(|c| c.0 == STRING_MAP_INSERT).count(), 2);
        assert_eq!(
            log.iter().filter(|c| c.0 == STRING_MAP_DESTRUCTOR).count(),
            1
        );
    }

    #[test]
    fn populate_with_nothing_to_do_only_builds_and_destroys_the_map() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x00cf_6a00, &args![0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let addresses: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            addresses,
            vec![0x00cf_6a00, STRING_MAP_CONSTRUCTOR, STRING_MAP_DESTRUCTOR]
        );
    }

    #[test]
    fn depopulate_releases_and_clears_the_matching_singletons() {
        let mut e = engine();
        let (memory_source, _) = node(&mut e, "memory", 0, 0x1);
        let (stream_source, _) = node(&mut e, "stream", memory_source, 0x2);
        // Targets: "memory" holds 0x5001, "stream" holds 0 (cleared without a release),
        // "unknown" is not among the sources.
        let (unknown, unknown_variable) = node(&mut e, "unknown", 0, 0x7777);
        let (stream, stream_variable) = node(&mut e, "stream", unknown, 0);
        let (memory, memory_variable) = node(&mut e, "memory", stream, 0x5001);
        e.call_log = Some(vec![]);
        let frame = e.next_stack_local(0x18);
        e.call(0x00cf_6a90, &args![memory, stream_source]);
        let lookup = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|c| c.0 == STRING_MAP_GET_WITH_DEFAULT)
            .unwrap()
            .1[0];
        assert_eq!(lookup, frame + 8);
        assert_eq!(e.mem.u32(memory_variable), 0);
        assert_eq!(e.mem.u32(stream_variable), 0);
        assert_eq!(e.mem.u32(unknown_variable), 0x7777);
        let log = e.call_log.take().unwrap();
        let removed: Vec<_> = log
            .iter()
            .filter(|c| c.0 == REMOVE_REFERENCE_LOCK_UNCHECKED)
            .collect();
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0].1, vec![0x5001]);
        assert_eq!(
            log.iter().filter(|c| c.0 == STRING_MAP_DESTRUCTOR).count(),
            1
        );
    }
}
