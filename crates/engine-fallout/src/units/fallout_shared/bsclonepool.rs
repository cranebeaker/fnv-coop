//! `fallout shared/bsclonepool.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The scope timer's constructor (`thiscall` on the timer, `(0x31, 1, source file, line)`)
/// and destructor, and the source file name `...\BSClonePool.cpp` (`0101fca0`).
const TIMER_SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const TIMER_SCOPE_DESTROY: u32 = 0x0040_4ee0;
const CLONE_POOL_SOURCE_FILE_NAME: u32 = 0x0101_fca0;
/// The list walk (`008256d0` "stop", `006815c0` the item slot, `00726070` the next node;
/// `thiscall` on the node), the heap allocation `00401000` (`cdecl`, `(size)`), the pool
/// list's add `005ae3d0` (`thiscall`, `(&pool)`), `NiCloningProcess`'s constructor
/// (`004ad050`, `thiscall`, `(float)`) and destructor (`004ad270`), `NiObject::Clone`
/// (`00a5d2c0`, `thiscall`, `(&process)`), the smart pointer's constructor (`00633c90`,
/// `thiscall`, `(object)`) and destructor (`0045cec0`), and the pool's add of a clone
/// (`00631540`, `thiscall` on the pool's list at +4, `(&pointer)`).
const LIST_STOP_008256D0: u32 = 0x0082_56d0;
const LIST_ITEM_006815C0: u32 = 0x0068_15c0;
const LIST_FOLLOWING_00726070: u32 = 0x0072_6070;
const HEAP_ALLOCATE: u32 = 0x0040_1000;
const POOLS_ADD_005AE3D0: u32 = 0x005a_e3d0;
const CLONING_PROCESS_CONSTRUCT: u32 = 0x004a_d050;
const CLONING_PROCESS_DESTROY: u32 = 0x004a_d270;
const NI_OBJECT_CLONE: u32 = 0x00a5_d2c0;
const POINTER_CONSTRUCT_00633C90: u32 = 0x0063_3c90;
const POINTER_DESTROY_0045CEC0: u32 = 0x0045_cec0;
const POOL_ADD_CLONE_00631540: u32 = 0x0063_1540;
/// The pool's member constructors `004ee810` and `004ee920` (`thiscall` on `this + 4`).
const POOL_MEMBER_004EE810: u32 = 0x004e_e810;
const POOL_MEMBER_004EE920: u32 = 0x004e_e920;

// Translated from 004b0c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0xC-byte clone pool constructor (unnamed in the map): `004ee810` on `this + 4`, the
/// object word cleared, then `004ee920` on `this + 4`. Answers `this`.
// Translation incomplete: 004b0c80 - SEH exception unwinding is unsupported.
pub fn fn_004b0c80(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(POOL_MEMBER_004EE810, &args![this.addr() + 4]);
    e.mem.set_u32(this.addr(), 0);
    e.call(POOL_MEMBER_004EE920, &args![this.addr() + 4]);
    this
}

// Translated from 004b0910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSClonePools::MakeClones` (Xbox PDB), `(object, count)`: nothing for a null object.
/// Within a scope timer (`BSClonePool.cpp` line 0x38): the pool for `object` is looked up in
/// the list (`this`, an item whose first word is the object); without one a 0xC-byte pool
/// is allocated (`00401000`), constructed (`fn_004b0c80`; null when the allocation fails),
/// given the object (written through the pointer even when null, as the exe does) and added
/// (`005ae3d0`). Then `count` times: a cloning process (`004ad050(1.0)`), the object's clone
/// (`NiObject::Clone`) held in a smart pointer and added to the pool (`00631540`), the
/// pointer and the process destroyed.
// Translation incomplete: 004b0910 - SEH exception unwinding is unsupported.
pub fn bs_clone_pools_make_clones(e: &mut Engine, this: Ptr, object: Ptr, count: u32) {
    if object.is_null() {
        return;
    }
    // The frame below EBP: the pointer at +0x04, the cloning process at +0x14, the pool
    // at +0x3C, the timer at +0x40.
    e.with_stack(0x50, |e, frame| {
        let frame = frame.addr();
        let (pointer, process, found, timer) =
            (frame + 4, frame + 0x14, frame + 0x3c, frame + 0x40);
        e.call(
            TIMER_SCOPE_CONSTRUCT,
            &args![timer, 0x31u32, 1u32, CLONE_POOL_SOURCE_FILE_NAME, 0x38u32],
        );
        e.mem.set_u32(found, 0);
        let mut node = this.addr();
        while node != 0 && e.mem.u32(found) == 0 && !e.call(LIST_STOP_008256D0, &args![node]).bool()
        {
            let slot = e.call(LIST_ITEM_006815C0, &args![node]).u32();
            let item = e.mem.u32(slot);
            if item != 0 && e.mem.u32(item) == object.addr() {
                e.mem.set_u32(found, item);
            }
            node = e.call(LIST_FOLLOWING_00726070, &args![node]).u32();
        }
        if e.mem.u32(found) == 0 {
            let block = e.call(HEAP_ALLOCATE, &args![0xcu32]).u32();
            let pool = if block != 0 {
                fn_004b0c80(e, Ptr::new(block)).addr()
            } else {
                0
            };
            e.mem.set_u32(found, pool);
            e.mem.set_u32(pool, object.addr());
            e.call(POOLS_ADD_005AE3D0, &args![this, found]);
        }
        let mut index = 0u32;
        while index < count {
            e.call(CLONING_PROCESS_CONSTRUCT, &args![process, x87_store32(1.0)]);
            let clone = e.call(NI_OBJECT_CLONE, &args![object, process]).u32();
            e.call(POINTER_CONSTRUCT_00633C90, &args![pointer, clone]);
            let pool = e.mem.u32(found);
            e.call(
                POOL_ADD_CLONE_00631540,
                &args![pool.wrapping_add(4), pointer],
            );
            e.call(POINTER_DESTROY_0045CEC0, &args![pointer]);
            e.call(CLONING_PROCESS_DESTROY, &args![process]);
            index += 1;
        }
        e.call(TIMER_SCOPE_DESTROY, &args![timer]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b0c80, fn_004b0c80(Ptr) -> Ptr),
        entry!(0x004b0910, bs_clone_pools_make_clones(Ptr, Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn pool_constructor_clears_the_object() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(POOL_MEMBER_004EE810, |_, _| Ret::default());
        e.register(POOL_MEMBER_004EE920, |_, _| Ret::default());
        let pool = e.mem.alloc(0xc);
        e.mem.set_u32(pool, 9);
        assert_eq!(e.call(0x004b0c80, &args![pool]).u32(), pool);
        assert_eq!(e.mem.u32(pool), 0);
        assert_eq!(calls_at(&e, POOL_MEMBER_004EE810), vec![vec![pool + 4]]);
        assert_eq!(calls_at(&e, POOL_MEMBER_004EE920), vec![vec![pool + 4]]);
    }

    /// Doubles for everything `MakeClones` calls; the list is the nodes at `node + 0` (item
    /// slot) and `node + 4` (next), a null next ending it.
    fn clone_engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for addr in [
            TIMER_SCOPE_CONSTRUCT,
            TIMER_SCOPE_DESTROY,
            POOLS_ADD_005AE3D0,
            CLONING_PROCESS_CONSTRUCT,
            CLONING_PROCESS_DESTROY,
            POINTER_CONSTRUCT_00633C90,
            POINTER_DESTROY_0045CEC0,
            POOL_ADD_CLONE_00631540,
            POOL_MEMBER_004EE810,
            POOL_MEMBER_004EE920,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(LIST_STOP_008256D0, |_, _| answer(0));
        e.register(LIST_ITEM_006815C0, |_, a| answer(a[0]));
        e.register(LIST_FOLLOWING_00726070, |e, a| answer(e.mem.u32(a[0] + 4)));
        e.register(NI_OBJECT_CLONE, |_, _| answer(0x9900));
        e.register(HEAP_ALLOCATE, |e, a| answer(e.mem.alloc(a[0])));
        e
    }

    #[test]
    fn make_clones_uses_the_existing_pool() {
        let mut e = clone_engine();
        let object = 0x8000u32;
        let pool = e.mem.alloc(0xc);
        e.mem.set_u32(pool, object);
        let other = e.mem.alloc(0xc);
        e.mem.set_u32(other, 0x8100);
        // this: item `other`, next node with item `pool`.
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, pool);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, other);
        e.mem.set_u32(this + 4, second);
        e.call(0x004b0910, &args![this, object, 2u32]);
        assert!(calls_at(&e, HEAP_ALLOCATE).is_empty());
        let adds = calls_at(&e, POOL_ADD_CLONE_00631540);
        assert_eq!(adds.len(), 2);
        assert_eq!(adds[0][0], pool + 4);
        let pointers = calls_at(&e, POINTER_CONSTRUCT_00633C90);
        assert_eq!(pointers[0][1], 0x9900);
        assert_eq!(adds[0][1], pointers[0][0]);
        let processes = calls_at(&e, CLONING_PROCESS_CONSTRUCT);
        assert_eq!(processes[0][1], 1.0f32.to_bits());
        assert_eq!(
            calls_at(&e, NI_OBJECT_CLONE)[0],
            vec![object, processes[0][0]]
        );
        assert_eq!(calls_at(&e, CLONING_PROCESS_DESTROY).len(), 2);
        let timers = calls_at(&e, TIMER_SCOPE_CONSTRUCT);
        assert_eq!(timers[0][1..], [0x31, 1, CLONE_POOL_SOURCE_FILE_NAME, 0x38]);
        assert_eq!(calls_at(&e, TIMER_SCOPE_DESTROY).len(), 1);
    }

    #[test]
    fn make_clones_makes_a_pool_when_missing() {
        let mut e = clone_engine();
        let this = e.mem.alloc(8);
        e.call(0x004b0910, &args![this, 0x8000u32, 1u32]);
        let blocks = calls_at(&e, HEAP_ALLOCATE);
        assert_eq!(blocks, vec![vec![0xc]]);
        let added = calls_at(&e, POOLS_ADD_005AE3D0);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], this);
        let pool = e.mem.u32(added[0][1]);
        assert_eq!(e.mem.u32(pool), 0x8000);
        assert_eq!(calls_at(&e, POOL_ADD_CLONE_00631540)[0][0], pool + 4);
        // A null object does nothing.
        e.call_log = Some(vec![]);
        e.call(0x004b0910, &args![this, 0u32, 1u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
