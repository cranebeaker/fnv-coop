//! `Havok SDK/hkpdisableentitycollisionfilterclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dff930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_6900`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dff930(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_6900, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dff970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0xc` (`SUB ECX,0xc`) and jumps to `00dff930`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dff970(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_f930, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

/// The vtables the destructor installs on its way (the object has sub-objects at `+8`, `+0xC`,
/// `+0x10`, `+0x14` and `+0x30`) and the ones it ends with.
const INTERIM_VTABLES: [(u32, u32); 6] = [
    (0x00, 0x010e_7700),
    (0x08, 0x010e_76f4),
    (0x0c, 0x010e_76e0),
    (0x10, 0x010e_76d4),
    (0x14, 0x010e_76c8),
    (0x30, 0x010e_76ac),
];
const FINAL_VTABLES: [(u32, u32); 6] = [
    (0x30, 0x0102_f13c),
    (0x14, 0x010c_3b8c),
    (0x10, 0x010c_3bac),
    (0x0c, 0x010c_3b98),
    (0x08, 0x010c_3b80),
    (0x00, 0x0102_e388),
];
/// The routine `00c9c670` (`this` = entity in `ECX`, one stack word: a listener) that removes a
/// listener from an entity's listener array.
const ENTITY_REMOVE_LISTENER: u32 = 0x00c9_c670;
/// The `hkArray` empty capacity word (no storage, not owned).
const EMPTY_CAPACITY: u32 = 0x8000_0000;

// Translated from 00e46900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor `deleting_destructor_00dff930` runs (no name in the engine map; `ECX` =
/// object, nothing on the stack; the class is unconfirmed). Sets the interim vtables. For every
/// entity of its array (data `+0x34`, size `+0x38`, capacity word `+0x3c`) it makes sure the
/// entity has its 16-byte listener record (the pointer at entity `+0x214`, allocated through the
/// memory router's allocator, slot `4`, when it is 0), looks for the sub-object at `this + 0x30`
/// in that record's array of 16-bit size at `+0xC` (data at `+8`) and, if it is there, calls
/// `00c9c670` on the entity with it. Then the entity array is freed (slot `0x10`, capacity times
/// 4 bytes, unless bit 31 of the capacity word says it is not owned), the array is reset and the
/// final vtables are installed.
pub fn fn_00e46900(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    for (offset, vtable) in INTERIM_VTABLES {
        e.mem.set_u32(me + offset, vtable);
    }
    let listener = me + 0x30;
    let mut index = 0u32;
    if e.mem.i32(me + 0x38) > 0 {
        loop {
            let entity = e.mem.u32(e.mem.u32(me + 0x34) + 4 * index);
            if e.mem.u32(entity + 0x214) == 0 {
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let allocator = e.mem.u32(router + 0x10);
                let record = e.vcall(allocator, 4, &args![0x10u32]).u32();
                if record != 0 {
                    e.mem.set_u32(record, 0);
                    e.mem.set_u16(record + 4, 0);
                    e.mem.set_u16(record + 6, 0x8000);
                    e.mem.set_u32(record + 8, 0);
                    e.mem.set_u16(record + 0xc, 0);
                    e.mem.set_u16(record + 0xe, 0x8000);
                }
                e.mem.set_u32(entity + 0x214, record);
            }
            let record = e.mem.u32(entity + 0x214);
            let count = u32::from(e.mem.u16(record + 0xc));
            let data = e.mem.u32(record + 8);
            let found = (0..count).any(|slot| e.mem.u32(data + 4 * slot) == listener);
            if found {
                let entity = e.mem.u32(e.mem.u32(me + 0x34) + 4 * index);
                e.call(ENTITY_REMOVE_LISTENER, &args![entity, listener]);
            }
            index += 1;
            if index as i32 >= e.mem.i32(me + 0x38) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x3c);
    e.mem.set_u32(me + 0x38, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(me + 0x34);
        let bytes = (capacity & 0x3fff_ffff) * 4;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(me + 0x34, 0);
    e.mem.set_u32(me + 0x3c, EMPTY_CAPACITY);
    for (offset, vtable) in FINAL_VTABLES {
        e.mem.set_u32(me + offset, vtable);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_f930, deleting_destructor_00dff930(Ptr, u8) -> Ptr),
        entry!(0x00e4_6900, fn_00e46900(Ptr)),
        entry!(0x00df_f970, fn_00dff970(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dff930_frees_only_when_bit_zero_is_set() {
        check(0x00df_f930, 0x00e4_6900);
    }

    #[test]
    fn destructor_00e46900_detaches_the_listener_and_frees_the_entity_array() {
        let mut e = engine();

        let log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        // The record the allocator hands out for the entity that has none yet.
        let fresh = e.mem.alloc(0x10);
        for word in 0..4 {
            e.mem.set_u32(fresh + 4 * word, 0xdddd_dddd);
        }
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0004, move |_, a| {
            sink.borrow_mut().push(vec![4, a[0], a[1]]);
            Ret {
                eax: fresh,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0010, move |_, a| {
            sink.borrow_mut().push(vec![0x10, a[0], a[1], a[2]]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(0x00c9_c670, move |_, a| {
            sink.borrow_mut().push(vec![0x670, a[0], a[1]]);
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        let bare = e.mem.alloc(0x220);
        let attached = e.mem.alloc(0x220);
        // `attached` already has a record whose array holds another listener and ours.
        let record = e.mem.alloc(0x10);
        let listeners = e.mem.alloc(8);
        e.mem.set_u32(listeners, 0x4444);
        e.mem.set_u32(listeners + 4, this + 0x30);
        e.mem.set_u32(record + 8, listeners);
        e.mem.set_u16(record + 0xc, 2);
        e.mem.set_u32(attached + 0x214, record);
        let entities = e.mem.alloc(8);
        e.mem.set_u32(entities, bare);
        e.mem.set_u32(entities + 4, attached);
        e.mem.set_u32(this + 0x34, entities);
        e.mem.set_u32(this + 0x38, 2);
        e.mem.set_u32(this + 0x3c, 6);
        e.call(0x00e4_6900, &args![this]);

        assert_eq!(
            *log.borrow(),
            vec![
                vec![4, allocator, 0x10],
                vec![0x670, attached, this + 0x30],
                vec![0x10, allocator, entities, 24],
            ]
        );
        // The fresh record was initialised as an empty small-array pair.
        assert_eq!(e.mem.u32(bare + 0x214), fresh);
        assert_eq!(e.mem.u32(fresh), 0);
        assert_eq!(e.mem.u16(fresh + 4), 0);
        assert_eq!(e.mem.u16(fresh + 6), 0x8000);
        assert_eq!(e.mem.u32(fresh + 8), 0);
        assert_eq!(e.mem.u16(fresh + 0xc), 0);
        assert_eq!(e.mem.u16(fresh + 0xe), 0x8000);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0x8000_0000);
        let vtables: Vec<u32> = [0, 8, 0xc, 0x10, 0x14, 0x30]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            vtables,
            vec![
                0x0102_e388,
                0x010c_3b80,
                0x010c_3b98,
                0x010c_3bac,
                0x010c_3b8c,
                0x0102_f13c
            ]
        );
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00dff970() {
        assert_eq!(check(0x00df_f970, 0x00df_f930, 0xc), 0xdead_beef);
    }
}
