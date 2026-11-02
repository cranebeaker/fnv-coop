//! `Havok SDK/hkobjectresource.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions
//! in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The class is not in the Xbox PDB; its layout is read from the PC code: vtable at `+0`, the
//! reference-counted base's size word at `+4` and count at `+6` (`u16`s), the contents pointer
//! at `+8`, the contents' `hkClass` at `+0xC`, and two reference-counted pointers at `+0x10`
//! and `+0x14` (the first one's virtual slot `0x10` finds a class by name, the second's slot
//! `0x14` is called with the contents and the class name when the contents are destroyed).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject::addReference` and `removeReference` (the locked variants), `this` in
/// `ECX`.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkClass::getName` (`00d46d40`, the word at `+0`) and `getObjectSize` (`00d470d0`, the word
/// at `+8`), `this` = the class in `ECX`.
const CLASS_NAME: u32 = 0x00d4_6d40;
const CLASS_OBJECT_SIZE: u32 = 0x00d4_70d0;
/// `00d46d80` (`this` = a class, `(out byte, other class)`, returns `out`): whether the other
/// class's parent chain contains `this`.
const CLASS_CHAIN_CONTAINS: u32 = 0x00d4_6d80;
/// The `hkClass` record at `0x01268748` the destructor tests the contents' class against, and
/// the global object pointer `0x01268b08` whose slots `0x10` and `0xC` the constructor calls.
const REFERENCED_OBJECT_CLASS: u32 = 0x0126_8748;
const OBJECT_REGISTRY_GLOBAL: u32 = 0x0126_8b08;
/// The vtable the constructor and the destructor install while the object is alive
/// (`0x010ea4dc`) and the base vtable the destructor leaves (`0x0102e388`).
const OBJECT_RESOURCE_VTABLE: u32 = 0x010e_a4dc;
const BASE_VTABLE: u32 = 0x0102_e388;

// Translated from 00e04e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectResource::getName` (Xbox PDB): returns 0 (a null name).
pub fn hk_object_resource_get_name(_e: &mut Engine) -> u32 {
    0
}

// Translated from 00e04e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectResource::getImportsExports` (Xbox PDB): does nothing; it takes `this` and two
/// stack words it never reads.
pub fn hk_object_resource_get_imports_exports(
    _e: &mut Engine,
    _unused_0: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) {
}

// Translated from 00e04e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectResource::stealContentsPointer` (Xbox PDB), `(a, b)`: calls virtual slot `0x18`
/// with both words, then clears the contents pointer (`+8`) and the class (`+0xC`).
/// Answers what the slot `0x18` call answered (`stealContentsPointer` returns `void*` in the
/// Xbox PDB; `EAX` is the `CALL EAX` result of 00e04e84 at the RET 00e04e8f). No exe reader of
/// the slot +0x20 was found.
pub fn hk_object_resource_steal_contents_pointer(e: &mut Engine, this: Ptr, a: u32, b: u32) -> u32 {
    let contents = e.vcall(this.addr(), 0x18, &args![a, b]).u32();
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0xc, 0);
    contents
}

// Translated from 00e04ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectResource::getContentsTypeName` (Xbox PDB): the name of the class at `+0xC`
/// (`hkClass::getName`, a tail call).
pub fn hk_object_resource_get_contents_type_name(e: &mut Engine, this: Ptr) -> u32 {
    let class = e.mem.u32(this.addr() + 0xc);
    e.call(CLASS_NAME, &args![class]).u32()
}

/// Replaces the reference-counted pointer at `this + offset` by `value`: adds a reference to
/// the new one when it is not null, removes one from the old one when it is not null.
fn set_reference(e: &mut Engine, this: Ptr, offset: u32, value: Ptr) {
    if value.addr() != 0 {
        e.call(ADD_REFERENCE, &args![value]);
    }
    let old = e.mem.u32(this.addr() + offset);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(this.addr() + offset, value.addr());
}

// Translated from 00e04eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: sets the reference-counted pointer at `+0x10` (see
/// `set_reference`).
pub fn fn_00e04eb0(e: &mut Engine, this: Ptr, value: Ptr) {
    set_reference(e, this, 0x10, value);
}

// Translated from 00e04ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: sets the reference-counted pointer at `+0x14` (see
/// `set_reference`).
pub fn fn_00e04ee0(e: &mut Engine, this: Ptr, value: Ptr) {
    set_reference(e, this, 0x14, value);
}

// Translated from 00e04f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the constructor, `this` and a pointer to two words (the
/// contents and its class). Reference count 1 (`+6`), vtable `0x010ea4dc`, the two words at
/// `+8` and `+0xC`; then the object held in the global `0x01268b08` is asked (virtual slot
/// `0x10`, then `0xC`, no arguments) for the two objects kept at `+0x10` and `+0x14`, each
/// gets a reference when it is not null. Returns `this`.
pub fn fn_00e04f10(e: &mut Engine, this: Ptr, pair: Ptr) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), OBJECT_RESOURCE_VTABLE);
    let contents = e.mem.u32(pair.addr());
    e.mem.set_u32(this.addr() + 8, contents);
    let class = e.mem.u32(pair.addr() + 4);
    e.mem.set_u32(this.addr() + 0xc, class);
    let registry = e.mem.u32(OBJECT_REGISTRY_GLOBAL);
    let first = e.vcall(registry, 0x10, &args![]).u32();
    if first != 0 {
        e.call(ADD_REFERENCE, &args![first]);
    }
    e.mem.set_u32(this.addr() + 0x10, first);
    let registry = e.mem.u32(OBJECT_REGISTRY_GLOBAL);
    let second = e.vcall(registry, 0xc, &args![]).u32();
    if second != 0 {
        e.call(ADD_REFERENCE, &args![second]);
    }
    e.mem.set_u32(this.addr() + 0x14, second);
    this
}

// Translated from 00e04f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectResource::~hkObjectResource` (Xbox PDB): with the contents class (`+0xC`) set, a
/// contents class that derives from the `hkClass` record at `0x01268748` has the reference
/// to its contents (`+8`) removed; any other has the object at `+0x14` told (its virtual
/// slot `0x14`, `(contents, class name)`) when there is one, and the contents block given back
/// to the allocator (slot 8) with the class's object size. Then the two held objects get a
/// reference removed (when not null) and are cleared, and the base vtable `0x0102e388` is left.
pub fn hk_object_resource_destructor(e: &mut Engine, this: Ptr) {
    let class = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(this.addr(), OBJECT_RESOURCE_VTABLE);
    if class != 0 {
        let derives = e.with_stack(4, |e, out| {
            let result = e
                .call(
                    CLASS_CHAIN_CONTAINS,
                    &args![REFERENCED_OBJECT_CLASS, out, class],
                )
                .u32();
            e.mem.u8(result)
        });
        if derives != 0 {
            let contents = e.mem.u32(this.addr() + 8);
            e.call(REMOVE_REFERENCE, &args![contents]);
        } else {
            let listener = e.mem.u32(this.addr() + 0x14);
            if listener != 0 {
                let class = e.mem.u32(this.addr() + 0xc);
                let name = e.call(CLASS_NAME, &args![class]).u32();
                let contents = e.mem.u32(this.addr() + 8);
                e.vcall(listener, 0x14, &args![contents, name]);
            }
            let class = e.mem.u32(this.addr() + 0xc);
            let size = e.call(CLASS_OBJECT_SIZE, &args![class]).u32();
            let contents = e.mem.u32(this.addr() + 8);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.vcall(allocator, 8, &args![contents, size]);
        }
    }
    let listener = e.mem.u32(this.addr() + 0x14);
    if listener != 0 {
        e.call(REMOVE_REFERENCE, &args![listener]);
    }
    e.mem.set_u32(this.addr() + 0x14, 0);
    let registry_object = e.mem.u32(this.addr() + 0x10);
    if registry_object != 0 {
        e.call(REMOVE_REFERENCE, &args![registry_object]);
    }
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

// Translated from 00e05030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectResource::getContentsPointer` (Xbox PDB), `(type_name, _unused)`: the contents
/// pointer (`+8`), or 0 when `type_name` is not null and the class it names does not derive
/// from the contents' class. Both classes are found by name through the object at `+0x10`
/// (virtual slot `0x10`, one word): the contents class by its `getName`, the requested one by
/// `type_name`; the derivation test is `00d46d80`.
pub fn hk_object_resource_get_contents_pointer(
    e: &mut Engine,
    this: Ptr,
    type_name: u32,
    _unused_2: u32,
) -> u32 {
    if type_name != 0 {
        let finder = e.mem.u32(this.addr() + 0x10);
        let class = e.mem.u32(this.addr() + 0xc);
        let own_name = e.call(CLASS_NAME, &args![class]).u32();
        let own_class = e.vcall(finder, 0x10, &args![own_name]).u32();
        let requested = e.vcall(finder, 0x10, &args![type_name]).u32();
        let derives = e.with_stack(4, |e, out| {
            let result = e
                .call(CLASS_CHAIN_CONTAINS, &args![requested, out, own_class])
                .u32();
            e.mem.u8(result)
        });
        if derives == 0 {
            return 0;
        }
    }
    e.mem.u32(this.addr() + 8)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_4e50, hk_object_resource_get_name() -> u32),
        entry!(
            0x00e0_4e60,
            hk_object_resource_get_imports_exports(Ptr, u32, u32)
        ),
        entry!(
            0x00e0_4e70,
            hk_object_resource_steal_contents_pointer(Ptr, u32, u32) -> u32
        ),
        entry!(0x00e0_4ea0, hk_object_resource_get_contents_type_name(Ptr) -> u32),
        entry!(0x00e0_4eb0, fn_00e04eb0(Ptr, Ptr)),
        entry!(0x00e0_4ee0, fn_00e04ee0(Ptr, Ptr)),
        entry!(0x00e0_4f10, fn_00e04f10(Ptr, Ptr) -> Ptr),
        entry!(0x00e0_4f80, hk_object_resource_destructor(Ptr)),
        entry!(0x00e0_5030, hk_object_resource_get_contents_pointer(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<String>>>;

    const FINDER_SLOT: u32 = 0x0900_0010;
    const NOTIFY_SLOT: u32 = 0x0900_0020;
    const FREE_SLOT: u32 = 0x0900_0030;
    const STEAL_SLOT: u32 = 0x0900_0040;
    const REGISTRY_FIRST: u32 = 0x0900_0050;
    const REGISTRY_SECOND: u32 = 0x0900_0060;

    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x2000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(ADD_REFERENCE, move |_, a| {
            seen.borrow_mut().push(format!("add:{:x}", a[0]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            seen.borrow_mut().push(format!("remove:{:x}", a[0]));
            Ret::default()
        });
        e.register(CLASS_NAME, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(CLASS_OBJECT_SIZE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        // Class chain: the double says the chain of `other` contains `this` when `other`'s
        // word at +4 is `this`.
        e.register(CLASS_CHAIN_CONTAINS, |e, a| {
            let found = e.mem.u32(a[2] + 4) == a[0] || a[2] == a[0];
            e.mem.set_u8(a[1], u8::from(found));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        (e, log)
    }

    /// An object whose virtual slots are the given (slot, double) pairs.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    fn resource(e: &mut Engine) -> u32 {
        e.mem.alloc(0x20)
    }

    #[test]
    fn get_name_is_null() {
        let (mut e, _) = engine();
        assert_eq!(e.call(0x00e0_4e50, &args![0x1234u32]).u32(), 0);
    }

    #[test]
    fn get_imports_exports_does_nothing() {
        let (mut e, log) = engine();
        e.call_log = Some(vec![]);
        e.call(0x00e0_4e60, &args![0x1234u32, 1u32, 2u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn steal_contents_pointer_calls_slot_0x18_then_clears_the_contents() {
        let (mut e, log) = engine();
        let seen = Rc::clone(&log);
        e.register_double(STEAL_SLOT, move |_, a| {
            seen.borrow_mut()
                .push(format!("steal:{:x}:{:x}", a[1], a[2]));
            Ret::default()
        });
        let this = object(&mut e, &[(0x18, STEAL_SLOT)]);
        e.mem.set_u32(this + 8, 0x77);
        e.mem.set_u32(this + 0xc, 0x88);
        e.call(0x00e0_4e70, &args![this, 5u32, 6u32]);
        assert_eq!(*log.borrow(), vec!["steal:5:6".to_string()]);
        assert_eq!((e.mem.u32(this + 8), e.mem.u32(this + 0xc)), (0, 0));
    }

    #[test]
    fn steal_contents_pointer_answers_what_slot_0x18_answered() {
        let (mut e, _) = engine();
        e.register_double(STEAL_SLOT, |_, _| Ret {
            eax: 0x4321,
            ..Ret::default()
        });
        let this = object(&mut e, &[(0x18, STEAL_SLOT)]);
        // EAX is the slot 0x18 result (00e04e84) at the RET 00e04e8f.
        assert_eq!(e.call(0x00e0_4e70, &args![this, 5u32, 6u32]).u32(), 0x4321);
    }

    #[test]
    fn get_contents_type_name_is_the_class_name() {
        let (mut e, _) = engine();
        let this = resource(&mut e);
        let class = e.mem.alloc(0x20);
        e.mem.set_u32(class, 0x5151);
        e.mem.set_u32(this + 0xc, class);
        assert_eq!(e.call(0x00e0_4ea0, &args![this]).u32(), 0x5151);
    }

    #[test]
    fn setters_add_the_new_reference_before_removing_the_old_one() {
        for (address, offset) in [(0x00e0_4eb0, 0x10), (0x00e0_4ee0, 0x14)] {
            let (mut e, log) = engine();
            let this = resource(&mut e);
            e.mem.set_u32(this + offset, 0xaaa0);
            e.call(address, &args![this, 0xbbb0u32]);
            assert_eq!(
                *log.borrow(),
                vec!["add:bbb0".to_string(), "remove:aaa0".to_string()]
            );
            assert_eq!(e.mem.u32(this + offset), 0xbbb0);
            log.borrow_mut().clear();
            e.call(address, &args![this, 0u32]);
            assert_eq!(*log.borrow(), vec!["remove:bbb0".to_string()]);
            assert_eq!(e.mem.u32(this + offset), 0);
            log.borrow_mut().clear();
            e.call(address, &args![this, 0u32]);
            assert!(log.borrow().is_empty());
        }
    }

    #[test]
    fn constructor_takes_the_pair_and_references_the_registry_objects() {
        let (mut e, log) = engine();
        e.register(REGISTRY_FIRST, |_, _| Ret {
            eax: 0x1111,
            ..Ret::default()
        });
        e.register(REGISTRY_SECOND, |_, _| Ret::default());
        let registry = object(&mut e, &[(0x10, REGISTRY_FIRST), (0xc, REGISTRY_SECOND)]);
        e.mem.set_u32(OBJECT_REGISTRY_GLOBAL, registry);
        let this = resource(&mut e);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 0x2222);
        e.mem.set_u32(pair + 4, 0x3333);
        let got = e.call(0x00e0_4f10, &args![this, pair]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), OBJECT_RESOURCE_VTABLE);
        assert_eq!(
            (e.mem.u32(this + 8), e.mem.u32(this + 0xc)),
            (0x2222, 0x3333)
        );
        assert_eq!(
            (e.mem.u32(this + 0x10), e.mem.u32(this + 0x14)),
            (0x1111, 0)
        );
        assert_eq!(*log.borrow(), vec!["add:1111".to_string()]);
    }

    fn destructor_setup(e: &mut Engine, derives: bool) -> (u32, u32) {
        let class = e.mem.alloc(0x20);
        e.mem.set_u32(class, 0x5252);
        e.mem.set_u32(class + 8, 0x40);
        e.mem
            .set_u32(class + 4, if derives { REFERENCED_OBJECT_CLASS } else { 0 });
        let this = resource(e);
        e.mem.set_u32(this + 8, 0x9000);
        e.mem.set_u32(this + 0xc, class);
        (this, class)
    }

    #[test]
    fn destructor_releases_a_referenced_contents_object() {
        let (mut e, log) = engine();
        let (this, _) = destructor_setup(&mut e, true);
        let listener = object(&mut e, &[]);
        e.mem.set_u32(this + 0x14, listener);
        e.mem.set_u32(this + 0x10, 0x7777);
        e.call(0x00e0_4f80, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                "remove:9000".to_string(),
                format!("remove:{listener:x}"),
                "remove:7777".to_string()
            ]
        );
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!((e.mem.u32(this + 0x10), e.mem.u32(this + 0x14)), (0, 0));
    }

    #[test]
    fn destructor_tells_the_listener_and_frees_plain_contents() {
        let (mut e, log) = engine();
        let (this, _) = destructor_setup(&mut e, false);
        let seen = Rc::clone(&log);
        e.register_double(NOTIFY_SLOT, move |_, a| {
            seen.borrow_mut()
                .push(format!("notify:{:x}:{:x}", a[1], a[2]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_SLOT, move |_, a| {
            seen.borrow_mut()
                .push(format!("free:{:x}:{:x}", a[1], a[2]));
            Ret::default()
        });
        let listener = object(&mut e, &[(0x14, NOTIFY_SLOT)]);
        e.mem.set_u32(this + 0x14, listener);
        let allocator = object(&mut e, &[(8, FREE_SLOT)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.call(0x00e0_4f80, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                "notify:9000:5252".to_string(),
                "free:9000:40".to_string(),
                format!("remove:{listener:x}")
            ]
        );
    }

    #[test]
    fn destructor_without_a_class_only_releases_the_held_objects() {
        let (mut e, log) = engine();
        let this = resource(&mut e);
        e.mem.set_u32(this + 0x10, 0x7777);
        e.call(0x00e0_4f80, &args![this]);
        assert_eq!(*log.borrow(), vec!["remove:7777".to_string()]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn get_contents_pointer_checks_the_requested_class() {
        let (mut e, _) = engine();
        // The finder returns the class record whose address is the name it is given.
        e.register(FINDER_SLOT, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let finder = object(&mut e, &[(0x10, FINDER_SLOT)]);
        let this = resource(&mut e);
        // The contents' class and a derived (requested) class whose parent is it.
        let own = e.mem.alloc(0x20);
        e.mem.set_u32(own, own);
        let requested = e.mem.alloc(0x20);
        e.mem.set_u32(requested + 4, own);
        e.mem.set_u32(this + 8, 0x4444);
        e.mem.set_u32(this + 0xc, own);
        e.mem.set_u32(this + 0x10, finder);
        // No name: the pointer.
        assert_eq!(e.call(0x00e0_5030, &args![this, 0u32, 0u32]).u32(), 0x4444);
        // `own`'s chain does not contain `requested`: 0.
        assert_eq!(e.call(0x00e0_5030, &args![this, requested, 0u32]).u32(), 0);
        // The requested class is the contents' own: the pointer.
        assert_eq!(e.call(0x00e0_5030, &args![this, own, 0u32]).u32(), 0x4444);
    }
}
