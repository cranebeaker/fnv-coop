//! `Havok SDK/run_00e04d90` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::allocator;

// Translated from 00e04d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: runs `hkClass::updateMetadataInplace` (`00e04d30`, with the
/// version `version`) over every class of a null-terminated array of class pointers (`classes`).
/// A three-word `hkMap` (`{0, 0, -1}`: no storage, no elements, hash mask `-1`) on the stack is
/// reserved first through `hkMapBase<unsigned long,unsigned long>::reserve` (`00cb1fe0`, `this`
/// = the map, the allocator and 0), passed to every update, then cleared and deallocated
/// (`00cb1a90`, `this` = the map, the allocator) and finished with `00a29680` (`this` = the map).
/// The function takes two stack words and pops none (`RET`) and returns nothing.
pub fn fn_00e04d90(e: &mut Engine, classes: Ptr, version: u32) {
    e.with_stack(12, |e, map| {
        e.mem.set_u32(map.addr(), 0);
        e.mem.set_u32(map.addr() + 4, 0);
        e.mem.set_u32(map.addr() + 8, 0xffff_ffff);
        let heap = allocator(e);
        e.call(0x00cb_1fe0, &args![map, heap, 0u32]);
        let mut index = 0u32;
        let mut class = e.mem.u32(classes.addr());
        while class != 0 {
            e.call(0x00e0_4d30, &args![class, map, version]);
            class = e.mem.u32(classes.addr() + index * 4 + 4);
            index += 1;
        }
        let heap = allocator(e);
        e.call(0x00cb_1a90, &args![map, heap]);
        e.call(0x00a2_9680, &args![map]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e0_4d90, fn_00e04d90(Ptr, u32))]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles over every callee that log (address, words); the map's three words are logged at
    /// the reserve call.
    fn engine(log: &Log, allocator: u32) -> Engine {
        let mut e = Engine::new();
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [0x00cb_1fe0, 0x00e0_4d30, 0x00cb_1a90, 0x00a2_9680] {
            let calls = Rc::clone(log);
            e.register_double(address, move |e, a| {
                let mut words = a.to_vec();
                if address == 0x00cb_1fe0 {
                    words.extend([e.mem.u32(a[0]), e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8)]);
                }
                calls.borrow_mut().push((address, words));
                Ret::default()
            });
        }
        e
    }

    /// Two classes: reserve, two updates in order (the same map, the version), then clear and
    /// the final call.
    #[test]
    fn fn_00e04d90_updates_every_class_between_reserve_and_clear() {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let mut e = engine(&log, 0xa110);
        let classes = e.mem.alloc(0x10);
        e.mem.set_u32(classes, 0x1111);
        e.mem.set_u32(classes + 4, 0x2222);
        e.mem.set_u32(classes + 8, 0);
        e.mem.set_u32(classes + 12, 0x3333);
        e.call(0x00e0_4d90, &args![classes, 7u32]);
        let log = log.borrow();
        assert_eq!(log.len(), 5);
        let map = log[0].1[0];
        assert_eq!(
            log[0],
            (0x00cb_1fe0, vec![map, 0xa110, 0, 0, 0, 0xffff_ffff])
        );
        assert_eq!(log[1], (0x00e0_4d30, vec![0x1111, map, 7]));
        assert_eq!(log[2], (0x00e0_4d30, vec![0x2222, map, 7]));
        assert_eq!(log[3], (0x00cb_1a90, vec![map, 0xa110]));
        assert_eq!(log[4], (0x00a2_9680, vec![map]));
    }

    /// An empty list updates nothing but still reserves and clears.
    #[test]
    fn fn_00e04d90_empty_list_only_reserves_and_clears() {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let mut e = engine(&log, 0xa110);
        let classes = e.mem.alloc(0x10);
        e.call(0x00e0_4d90, &args![classes, 7u32]);
        let addresses: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(addresses, vec![0x00cb_1fe0, 0x00cb_1a90, 0x00a2_9680]);
    }
}
