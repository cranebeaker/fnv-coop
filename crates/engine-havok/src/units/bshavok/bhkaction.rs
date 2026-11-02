//! `BSHavok/bhkaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c87be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAction::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00c878c0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `01268158`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn bhk_action_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00c8_78c0;
    const CLASS_NAME_POINTER: u32 = 0x0126_8158;
    const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// Per-thread word (`TLS + 0x2b4`) that the allocating functions set to 0x10 around their work.
const SCOPE_WORD: u32 = 0x2b4;
/// `bhkSerializable::Kill` (Xbox PDB) and `bhkSerializable::~bhkSerializable` (Xbox PDB).
const KILL: u32 = 0x00c8_76a0;
const SERIALIZABLE_DESTRUCTOR: u32 = 0x00c8_77b0;
/// `bhkRefObject::Link` (Xbox PDB, `(this, link)`).
const REF_OBJECT_LINK: u32 = 0x00c8_7c90;
/// `bhkWorld` functions: `Add_ov3` (Xbox PDB, `(world, action)`) and the unnamed
/// `bhkworld.obj` function `00c69580` (`(world, action)`, answers a byte).
const WORLD_ADD_ACTION: u32 = 0x00c6_b3c0;
const WORLD_REMOVE_ACTION: u32 = 0x00c6_9580;
/// `bhkAction` class vtable and object counter.
const VTABLE_ACTION: u32 = 0x0102_016c;
const COUNT_ACTION: u32 = 0x0126_8154;
/// Offset of the `bhkWorld` pointer in the Havok world object.
const WORLD_BHK_WORLD: u32 = 0x350;

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

// Translated from 00c87970 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c87970 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAction::~bhkAction` (Xbox PDB): stores the class vtable, runs `Kill` (`00c876a0`),
/// decrements the class counter and runs `bhkSerializable::~bhkSerializable` (`00c877b0`).
pub fn bhk_action_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_ACTION);
    e.call(KILL, &args![this]);
    let count: u32 = e.global(COUNT_ACTION);
    e.set_global(COUNT_ACTION, count.wrapping_sub(1));
    e.call(SERIALIZABLE_DESTRUCTOR, &args![this]);
}

// Translated from 00c879e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: stores at `out` the word at `+0x10` of the Havok action (`this + 8`) -- the back
/// pointer `Link` (`00c87a10`) stores -- or 0 when `this` or the Havok action is 0.
/// Answers the stored word.
pub fn fn_00c879e0(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let mut value = 0;
    if this.addr() != 0 {
        let action = e.mem.u32(this.addr() + 8);
        if action != 0 {
            value = e.mem.u32(action + 0x10);
        }
    }
    e.mem.set_u32(out.addr(), value);
    value
}

// Translated from 00c87a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkAction::Link` (Xbox PDB): when `this` and its Havok action (`+8`) exist, stores
/// `this` (or 0 when the low byte of `link` is 0) at `+0x10` of the Havok action, then
/// continues in `bhkRefObject::Link` (`00c87c90`, a tail jump with the same words).
pub fn bhk_action_link(e: &mut Engine, this: Ptr, link: u32) {
    if this.addr() != 0 {
        let action = e.mem.u32(this.addr() + 8);
        if action != 0 {
            let back_pointer = if link as u8 != 0 { this.addr() } else { 0 };
            e.mem.set_u32(action + 0x10, back_pointer);
        }
    }
    e.call(REF_OBJECT_LINK, &args![this, link]);
}

// Translated from 00c87a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::GethkWorld` (Xbox PDB): the word at `+8` of the Havok object (`this + 8`),
/// 0 when there is no Havok object. (`this` itself is not tested.)
pub fn bhk_world_object_get_hk_world(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.mem.u32(this.addr() + 8);
    if object == 0 {
        0
    } else {
        e.mem.u32(object + 8)
    }
}

// Translated from 00c87a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c87a70 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAction::Add` (Xbox PDB), with the allocation scope word set to 0x10: answers false
/// when `this` or its Havok action (`+8`) is 0. With a `world`: false when virtual slot
/// 0x94 (`GethkWorld`) already answers the Havok world at `world + 8`; otherwise runs
/// virtual slot 0xA0 (`Remove`), adds the Havok action to the world (`00c6b3c0`, `this` =
/// the `bhkWorld`) and answers true. Without a `world` it only runs `Remove` and answers
/// false.
pub fn bhk_action_add(e: &mut Engine, this: Ptr, world: Ptr) -> bool {
    let (word, saved) = enter_scope(e);
    let mut added = false;
    let action = if this.addr() == 0 {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    if action != 0 {
        if world.addr() == 0 {
            e.vcall(this.addr(), 0xa0, &args![]);
        } else {
            let current = e.vcall(this.addr(), 0x94, &args![]).u32();
            if current != e.mem.u32(world.addr() + 8) {
                e.vcall(this.addr(), 0xa0, &args![]);
                e.call(WORLD_ADD_ACTION, &args![world, action]);
                added = true;
            }
        }
    }
    e.mem.set_u32(word, saved);
    added
}

// Translated from 00c87b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c87b40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkAction::Remove` (Xbox PDB), with the allocation scope word set to 0x10: takes the Havok
/// world from virtual slot 0x94 (`GethkWorld`); when it exists and the `bhkWorld` at its
/// `+0x350` is not 0, removes the Havok action (`this + 8`) from it (`00c69580`) and answers
/// that call's byte; otherwise answers false.
pub fn bhk_action_remove(e: &mut Engine, this: Ptr) -> bool {
    let (word, saved) = enter_scope(e);
    let hk_world = e.vcall(this.addr(), 0x94, &args![]).u32();
    let world = if hk_world == 0 {
        0
    } else {
        e.mem.u32(hk_world + WORLD_BHK_WORLD)
    };
    let removed = if world == 0 {
        false
    } else {
        let action = e.mem.u32(this.addr() + 8);
        e.call(WORLD_REMOVE_ACTION, &args![world, action]).bool()
    };
    e.mem.set_u32(word, saved);
    removed
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c87970, bhk_action_destructor(Ptr)),
        entry!(0x00c879e0, fn_00c879e0(Ptr, Ptr) -> u32),
        entry!(0x00c87a10, bhk_action_link(Ptr, u32)),
        entry!(0x00c87a40, bhk_world_object_get_hk_world(Ptr) -> u32),
        entry!(0x00c87a70, bhk_action_add(Ptr, Ptr) -> bool),
        entry!(0x00c87b40, bhk_action_remove(Ptr) -> bool),
        entry!(0x00c87be0, bhk_action_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn bhk_action_get_viewer_strings_appends_the_class_name_growing_a_full_array() {
        const PARENT: u32 = 0x00c8_78c0;
        const NAME_POINTER: u32 = 0x0126_8158;
        const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
        const GROW: u32 = 0x0096_ad30;
        const SET_AT: u32 = 0x0096_ae90;
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(NAME_POINTER, 4);
            e.set_global(NAME_POINTER, 0x5151_0001u32);
            let this = Ptr::new(e.mem.alloc(0x40));
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.register(PARENT, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            bhk_action_get_viewer_strings(&mut e, this, Ptr::new(strings));
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().map(|c| c.0).collect();
            let size = u32::from(size);
            if capacity > 3 {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, SET_AT]);
            } else {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, GROW, SET_AT]);
                let grow = log.iter().find(|c| c.0 == GROW).unwrap();
                assert_eq!(grow.1, [strings, 5 + size]);
            }
            let parent = log.iter().find(|c| c.0 == PARENT).unwrap();
            assert_eq!(parent.1, [this.addr(), strings]);
            let name = log.iter().find(|c| c.0 == NI_GET_VIEWER_STRING).unwrap();
            assert_eq!(name.1, [0x5151_0001]);
            let set = log.iter().find(|c| c.0 == SET_AT).unwrap();
            assert_eq!(&set.1[..2], [strings, size]);
            assert_eq!(stored.get(), 0x7777_0001);
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An object with a fresh vtable holding the given `(byte offset, target)` slots, a Havok
    /// action at `+8` (when `with_action`) and room for the fields.
    fn object(e: &mut Engine, slots: &[(u32, u32)], with_action: bool) -> (u32, u32) {
        let table = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        let action = if with_action { e.mem.alloc(0x20) } else { 0 };
        e.mem.set_u32(object + 8, action);
        (object, action)
    }

    #[test]
    fn destructor_stores_the_vtable_kills_and_destroys_the_base() {
        let mut e = Engine::new();
        e.map(COUNT_ACTION, 4);
        e.set_global(COUNT_ACTION, 3u32);
        e.register(KILL, nothing);
        e.register(SERIALIZABLE_DESTRUCTOR, nothing);
        let (this, _) = object(&mut e, &[], false);
        e.call_log = Some(vec![]);
        e.call(0x00c87970, &args![this]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00c87970, KILL, SERIALIZABLE_DESTRUCTOR]);
        assert_eq!(e.mem.u32(this), VTABLE_ACTION);
        assert_eq!(e.global::<u32>(COUNT_ACTION), 2);
    }

    #[test]
    fn fn_00c879e0_stores_the_action_word_or_zero() {
        let mut e = Engine::new();
        let (this, action) = object(&mut e, &[], true);
        e.mem.set_u32(action + 0x10, 0xabcd);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xffff);
        assert_eq!(e.call(0x00c879e0, &args![this, out]).u32(), 0xabcd);
        assert_eq!(e.mem.u32(out), 0xabcd);
        let (bare, _) = object(&mut e, &[], false);
        assert_eq!(e.call(0x00c879e0, &args![bare, out]).u32(), 0);
        assert_eq!(e.mem.u32(out), 0);
        e.mem.set_u32(out, 0xffff);
        e.call(0x00c879e0, &args![0u32, out]);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn link_stores_this_for_a_nonzero_low_byte_and_continues_in_the_base() {
        let mut e = Engine::new();
        e.register(REF_OBJECT_LINK, nothing);
        let (this, action) = object(&mut e, &[], true);
        e.call_log = Some(vec![]);
        e.call(0x00c87a10, &args![this, 0x1_0001u32]);
        assert_eq!(e.mem.u32(action + 0x10), this);
        e.call(0x00c87a10, &args![this, 0x100u32]);
        assert_eq!(e.mem.u32(action + 0x10), 0);
        assert_eq!(
            calls(&e, REF_OBJECT_LINK),
            [[this, 0x1_0001], [this, 0x100]]
        );
        let (bare, _) = object(&mut e, &[], false);
        e.call(0x00c87a10, &args![bare, 1u32]);
        assert_eq!(calls(&e, REF_OBJECT_LINK).len(), 3);
    }

    #[test]
    fn get_hk_world_reads_through_the_havok_object() {
        let mut e = Engine::new();
        let (this, action) = object(&mut e, &[], true);
        e.mem.set_u32(action + 8, 0x7070);
        assert_eq!(e.call(0x00c87a40, &args![this]).u32(), 0x7070);
        let (bare, _) = object(&mut e, &[], false);
        assert_eq!(e.call(0x00c87a40, &args![bare]).u32(), 0);
    }

    #[test]
    fn add_moves_the_action_to_a_different_world() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        e.register(WORLD_ADD_ACTION, nothing);
        e.register(0x0900_00a0, |e, _| {
            let scope = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(scope), 0x10);
            Ret::default()
        });
        e.register(0x0900_0094, |_, _| Ret {
            eax: 0x1111,
            ..Ret::default()
        });
        let (this, action) = object(&mut e, &[(0x94, 0x0900_0094), (0xa0, 0x0900_00a0)], true);
        let world = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        // Already in that world: nothing happens.
        e.mem.set_u32(world + 8, 0x1111);
        assert_eq!(e.call(0x00c87a70, &args![this, world]).u32(), 0);
        assert!(calls(&e, WORLD_ADD_ACTION).is_empty());
        // A different world: Remove, then Add.
        e.mem.set_u32(world + 8, 0x2222);
        assert_eq!(e.call(0x00c87a70, &args![this, world]).u32(), 1);
        assert_eq!(calls(&e, WORLD_ADD_ACTION), [[world, action]]);
        assert_eq!(calls(&e, 0x0900_00a0).len(), 1);
        // No world: Remove only.
        assert_eq!(e.call(0x00c87a70, &args![this, 0u32]).u32(), 0);
        assert_eq!(calls(&e, 0x0900_00a0).len(), 2);
        // No Havok action: nothing runs.
        let (bare, _) = object(&mut e, &[(0xa0, 0x0900_00a0)], false);
        assert_eq!(e.call(0x00c87a70, &args![bare, world]).u32(), 0);
        assert_eq!(calls(&e, 0x0900_00a0).len(), 2);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    #[test]
    fn remove_removes_the_action_from_the_bhk_world_of_its_havok_world() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        let hk_world = e.mem.alloc(0x400);
        e.register_double(0x0900_0094, move |_, _| Ret {
            eax: hk_world,
            ..Ret::default()
        });
        e.register(WORLD_REMOVE_ACTION, |_, _| Ret {
            eax: 0xffff_ff01,
            ..Ret::default()
        });
        let (this, action) = object(&mut e, &[(0x94, 0x0900_0094)], true);
        e.call_log = Some(vec![]);
        // No bhkWorld behind the Havok world.
        assert_eq!(e.call(0x00c87b40, &args![this]).u32(), 0);
        assert!(calls(&e, WORLD_REMOVE_ACTION).is_empty());
        e.mem.set_u32(hk_world + 0x350, 0x3333);
        assert_eq!(e.call(0x00c87b40, &args![this]).u32(), 1);
        assert_eq!(calls(&e, WORLD_REMOVE_ACTION), [[0x3333, action]]);
        // No Havok world at all.
        e.register(0x0900_0094, |_, _| Ret::default());
        assert_eq!(e.call(0x00c87b40, &args![this]).u32(), 0);
        assert_eq!(calls(&e, WORLD_REMOVE_ACTION).len(), 1);
        assert_eq!(e.mem.u32(word), 0x55);
    }
}
