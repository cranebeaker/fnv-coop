//! `fallout/ai/combat/combattargetmap.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00853130(this, key, &out)` (unit `bgssaveloadreferencesmap`): the map
/// lookup; returns success in `AL` and writes the value to `out` only on
/// success.
const MAP_LOOKUP: u32 = 0x0085_3130;

// Translated from 009a4250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTargetMap::GetTargetAttackerArray` (engine map name): looks `key`
/// up in the map with `00853130` and returns the value found, or 0 when the
/// lookup fails.
pub fn combat_target_map_get_target_attacker_array(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        if e.call(MAP_LOOKUP, &args![this, key, found]).bool() {
            e.mem.u32(found.addr())
        } else {
            0
        }
    })
}

/// `CombatTargetMap`'s vtable (`0108e1e8`, named by its RTTI locator).
const COMBAT_TARGET_MAP_VTABLE: u32 = 0x0108_e1e8;
/// `NiTMap<Actor*, CombatTargetData*>`'s vtable (`0108e258`).
const NI_TMAP_VTABLE: u32 = 0x0108_e258;
/// `NiTMapBase<DFALL<NiTMapItem<Actor*, CombatTargetData*>>, Actor*,
/// CombatTargetData*>`'s vtable (`0108e278`).
const NI_TMAP_BASE_VTABLE: u32 = 0x0108_e278;
/// `NiTMapBase` fields (Xbox PDB names): `m_uiHashSize` +0x04,
/// `m_ppkHashTable` +0x08, the allocator's `m_uiCount` +0x0C.
const HASH_SIZE: u32 = 0x04;
const HASH_TABLE: u32 = 0x08;
const MAP_COUNT: u32 = 0x0c;
/// The allocator (`00aa1070`, `cdecl(size)`).
const MAP_ALLOCATE: u32 = 0x00aa_1070;
/// The game's `memset` wrapper (`00403d30`, `cdecl(dst, value, count)`).
const MEMSET: u32 = 0x0040_3d30;
/// `NiTMapBase::RemoveAll` of this instance (`00438af0`, `thiscall`).
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `GetFirstPos` (`004b9ba0`, `thiscall`) and `GetNext` (`006b7f20`,
/// `thiscall(position pointer, key pointer, value pointer)`).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_GET_NEXT: u32 = 0x006b_7f20;
/// The scalar deleting destructor of a target's data (`009a3fb0`,
/// `thiscall(flags)`).
const TARGET_DATA_DELETE: u32 = 0x009a_3fb0;
/// The `NiTMapBase` destructor body of this instance (`009a4480`).
const NI_TMAP_BASE_DESTRUCT: u32 = 0x009a_4480;
/// `operator delete` (`00401030`, `cdecl(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 009a3ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatTargetMap::scalar deleting destructor` (Xbox PDB): the destructor
/// body (`009a3ef0`), then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn combat_target_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009a3ef0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009a3ef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a3ef0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatTargetMap` destructor body (by its vtable, `0108e1e8`): deletes
/// every target's data the map holds (`GetFirstPos`/`GetNext`, `009a3fb0(1)`
/// on each non-null value), empties the map (`00438af0`) and destroys the
/// `NiTMap` base ([`fn_009a4420`]).
pub fn fn_009a3ef0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), COMBAT_TARGET_MAP_VTABLE);
    // The game's locals: value at EBP-0x18, key at EBP-0x14, position at
    // EBP-0x10.
    e.with_stack(0xc, |e, locals| {
        let value = locals.addr();
        let key = value + 4;
        let position = value + 8;
        let first = e.call(MAP_FIRST_POSITION, &args![this]).u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.mem.set_u32(key, 0);
            e.mem.set_u32(value, 0);
            e.call(MAP_GET_NEXT, &args![this, position, key, value]);
            let data = e.mem.u32(value);
            if data != 0 {
                e.call(TARGET_DATA_DELETE, &args![data, 1u32]);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_009a4420(e, this);
}

// Translated from 009a4290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<Actor*, CombatTargetData*>` constructor (by the vtable it sets,
/// `0108e258`): the `NiTMapBase` part ([`fn_009a43b0`]), then the vtable.
/// Returns `this`.
pub fn fn_009a4290(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_009a43b0(e, this, hash_size);
    e.mem.set_u32(this.addr(), NI_TMAP_VTABLE);
    this
}

// Translated from 009a43b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., Actor*, CombatTargetData*>` constructor (by the vtable
/// it sets, `0108e278`): `m_uiHashSize = hash_size`, count 0, and a zeroed
/// table of `hash_size` pointers (`00aa1070`, `00403d30`). Returns `this`.
pub fn fn_009a43b0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_TMAP_BASE_VTABLE);
    e.mem.set_u32(base + HASH_SIZE, hash_size);
    e.mem.set_u32(base + MAP_COUNT, 0);
    let bytes = e.mem.u32(base + HASH_SIZE) << 2;
    let table = e.call(MAP_ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(base + HASH_TABLE, table);
    let bytes = e.mem.u32(base + HASH_SIZE) << 2;
    let table = e.mem.u32(base + HASH_TABLE);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 009a4420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a4420 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `NiTMap<Actor*, CombatTargetData*>` destructor body: sets its vtable,
/// empties the map (`00438af0`) and destroys the `NiTMapBase` part
/// (`009a4480`).
pub fn fn_009a4420(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_TMAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.call(NI_TMAP_BASE_DESTRUCT, &args![this]);
}

// ---- Batch n025 ---------------------------------------------------------------------------

/// The vtable of the `BSSimpleArray` instance with RTTI name
/// `.?AV?$BSSimpleArray@PAVActor@@$0EAA@@` (read at the word before the vtable).
const ACTOR_ARRAY_VTABLE: u32 = 0x0108_4fec;
/// `operator new` (`00401000`, `cdecl(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `NiPointer` getter (`00559450`, `thiscall`: the word at `this`).
const POINTER_GET: u32 = 0x0055_9450;
/// The memory-context scope guard: constructor `00404eb0(guard, 0x2b, 1, file, line)`,
/// destructor `00404ee0(guard)`.
const SCOPE_GUARD_ENTER: u32 = 0x0040_4eb0;
const SCOPE_GUARD_LEAVE: u32 = 0x0040_4ee0;
/// The source file name `D:\_Fallout3\Platforms\Common\Code\Fallout\AI\Combat\CombatTargetMap.cpp`
/// (`0108e208`) the guard of `009a3fe0` names.
const TARGET_MAP_SOURCE_FILE: u32 = 0x0108_e208;
/// `NiTMapBase::RemoveAt` of this instance (`00405430`, `thiscall(key)`).
const MAP_REMOVE_AT: u32 = 0x0040_5430;

// Translated from 009a3db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a3db0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `CombatTargetData` destructor (by its layout: `Attackers` at +0, `spDebugGeometry`
/// at +0x10; `thiscall`): when the debug geometry is set and has a parent (`009611e0`),
/// detaches it: through the parent's virtual slot `+0xe8` when `008c7aa0` is false, else
/// through `0087ac60` on the object `004537b0` returns. Then destroys the pointer
/// (`0045cec0`) and the array (`008c1cb0`).
pub fn fn_009a3db0(e: &mut Engine, this: Ptr) {
    let geometry = this.addr() + 0x10;
    if e.call(POINTER_GET, &args![geometry]).u32() != 0 {
        let node = e.call(POINTER_GET, &args![geometry]).u32();
        if e.call(0x0096_11e0, &args![node]).u32() != 0 {
            if !e.call(0x008c_7aa0, &args![]).bool() {
                let node = e.call(POINTER_GET, &args![geometry]).u32();
                let parent = e.call(0x0096_11e0, &args![node]).u32();
                let child = e.call(POINTER_GET, &args![geometry]).u32();
                e.vcall(parent, 0xe8, &args![child]);
            } else {
                let child = e.call(POINTER_GET, &args![geometry]).u32();
                let owner = e.call(0x0045_37b0, &args![]).u32();
                e.call(0x0087_ac60, &args![owner, child]);
            }
        }
    }
    e.call(0x0045_cec0, &args![geometry]);
    e.call(0x008c_1cb0, &args![this]);
}

// Translated from 009a3fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CombatTargetData` scalar deleting destructor (`thiscall`, `RET 4`): `009a3db0`, then
/// `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_009a3fb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009a3db0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009a3fe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a3fe0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// `CombatTargetMap::AddTargetAttacker` (Xbox PDB; `thiscall`, `RET 8`): under a memory
/// scope guard (`00404eb0` with 0x2b, 1, this unit's file name and line 0x7c), looks
/// `target` up (`00853130`); a missing target gets a new `CombatTargetData` (0x14 bytes,
/// `00401000`, `009a40c0`) stored with `00844700`. `attacker` is then added to the data's
/// attackers (`007cb2e0` with the address of the `attacker` argument).
pub fn combat_target_map_add_target_attacker(
    e: &mut Engine,
    this: Ptr,
    target: u32,
    attacker: u32,
) {
    // The game's frame: the guard at EBP-0x10, the found data at EBP-0x14; the attacker
    // argument's stack slot follows.
    e.with_stack(0xc, |e, frame| {
        let guard = frame.addr();
        let found = frame.addr() + 4;
        let attacker_slot = frame.addr() + 8;
        e.mem.set_u32(attacker_slot, attacker);
        e.call(
            SCOPE_GUARD_ENTER,
            &args![guard, 0x2bu32, 1u32, TARGET_MAP_SOURCE_FILE, 0x7cu32],
        );
        e.mem.set_u32(found, 0);
        if !e.call(MAP_LOOKUP, &args![this, target, found]).bool() {
            let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
            let data = if block != 0 {
                fn_009a40c0(e, Ptr::new(block)).addr()
            } else {
                0
            };
            e.mem.set_u32(found, data);
            e.call(0x0084_4700, &args![this, target, data]);
        }
        let data = e.mem.u32(found);
        e.call(0x007c_b2e0, &args![data, attacker_slot]);
        e.call(SCOPE_GUARD_LEAVE, &args![guard]);
    });
}

// Translated from 009a40c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009a40c0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `CombatTargetData` constructor (`thiscall`): the attackers array (`009a42f0` with 5,
/// 0) and an empty debug geometry pointer (`00633c90` with 0). Returns `this`.
pub fn fn_009a40c0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_009a42f0(e, this, 5, 0);
    e.call(0x0063_3c90, &args![this.addr() + 0x10, 0u32]);
    this
}

// Translated from 009a4120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): removes `attacker` from `target`'s
/// attackers (`00853130`, then each entry through `006a7ad0`): as the last one, the data is
/// deleted (`009a3fb0` with 1) and the target removed (`00405430`); otherwise the entry is
/// removed (`009a4320` with 1).
pub fn fn_009a4120(e: &mut Engine, this: Ptr, target: u32, attacker: u32) {
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        if !e.call(MAP_LOOKUP, &args![this, target, found]).bool() {
            return;
        }
        let mut index = 0u32;
        loop {
            let data = e.mem.u32(found.addr());
            if index >= e.call(0x0044_ddc0, &args![data]).u32() {
                return;
            }
            let data = e.mem.u32(found.addr());
            let entry = e.call(0x006a_7ad0, &args![data, index]).u32();
            if e.mem.u32(entry) == attacker {
                let data = e.mem.u32(found.addr());
                if e.call(0x0044_ddc0, &args![data]).u32() == 1 {
                    let data = e.mem.u32(found.addr());
                    if data != 0 {
                        fn_009a3fb0(e, Ptr::new(data), 1);
                    }
                    e.call(MAP_REMOVE_AT, &args![this, target]);
                } else {
                    let data = e.mem.u32(found.addr());
                    e.call(0x009a_4320, &args![data, index, 1u32]);
                }
                return;
            }
            index += 1;
        }
    });
}

// Translated from 009a41e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): removes `target` (`00853130`, then
/// `00405430`) and deletes its data (`009a3fb0` with 1) when there was any.
pub fn fn_009a41e0(e: &mut Engine, this: Ptr, target: u32) {
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        if !e.call(MAP_LOOKUP, &args![this, target, found]).bool() {
            return;
        }
        e.call(MAP_REMOVE_AT, &args![this, target]);
        let data = e.mem.u32(found.addr());
        if data != 0 {
            fn_009a3fb0(e, Ptr::new(data), 1);
        }
    });
}

// Translated from 009a42c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<Actor*, CombatTargetData*>::scalar deleting destructor` (Xbox PDB; `thiscall`,
/// `RET 4`): the destructor body (`009a4420`), then `operator delete` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn ni_tmap_actor_combat_target_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_009a4420(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009a42f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray` instance `.?AV?$BSSimpleArray@PAVActor@@$0EAA@@`
/// (`thiscall`, `RET 8`): its vtable (`01084fec`), then `006b3eb0(this, reserve, size)`.
/// Returns `this`.
pub fn fn_009a42f0(e: &mut Engine, this: Ptr, reserve: u32, size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ACTOR_ARRAY_VTABLE);
    e.call(0x006b_3eb0, &args![this, reserve, size]);
    this
}

// Translated from 009a44b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<Actor*, CombatTargetData*>>, Actor*, CombatTargetData*>::
/// scalar deleting destructor` (Xbox PDB; `thiscall`, `RET 4`): the destructor body
/// (`009a4480`), then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_tmap_base_actor_combat_target_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(NI_TMAP_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_3db0, fn_009a3db0(Ptr)),
        entry!(0x009a_3fb0, fn_009a3fb0(Ptr, u32) -> Ptr),
        entry!(
            0x009a_3fe0,
            combat_target_map_add_target_attacker(Ptr, u32, u32)
        ),
        entry!(0x009a_40c0, fn_009a40c0(Ptr) -> Ptr),
        entry!(0x009a_4120, fn_009a4120(Ptr, u32, u32)),
        entry!(0x009a_41e0, fn_009a41e0(Ptr, u32)),
        entry!(
            0x009a_42c0,
            ni_tmap_actor_combat_target_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x009a_42f0, fn_009a42f0(Ptr, u32, u32) -> Ptr),
        entry!(
            0x009a_44b0,
            ni_tmap_base_actor_combat_target_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x009a4250,
            combat_target_map_get_target_attacker_array(Ptr, u32) -> u32
        ),
        entry!(
            0x009a_3ec0,
            combat_target_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x009a_3ef0, fn_009a3ef0(Ptr)),
        entry!(0x009a_4290, fn_009a4290(Ptr, u32) -> Ptr),
        entry!(0x009a_43b0, fn_009a43b0(Ptr, u32) -> Ptr),
        entry!(0x009a_4420, fn_009a4420(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hit_returns_the_value_a_miss_returns_zero() {
        let mut e = Engine::new();
        e.register(MAP_LOOKUP, |e, a| {
            if a[1] == 7 {
                e.mem.set_u32(a[2], 0x1234);
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            } else {
                // A failed lookup may leave a stale low byte in EAX's upper
                // bits; only AL counts.
                Ret {
                    eax: 0x100,
                    ..Ret::default()
                }
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a4250, &args![0x5000u32, 7u32]).u32(), 0x1234);
        assert_eq!(e.call(0x009a4250, &args![0x5000u32, 8u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            (log[1].0, log[1].1[..2].to_vec()),
            (MAP_LOOKUP, vec![0x5000, 7])
        );
    }
}

#[cfg(test)]
mod target_map_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            MAP_REMOVE_ALL,
            MEMSET,
            TARGET_DATA_DELETE,
            NI_TMAP_BASE_DESTRUCT,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MAP_ALLOCATE, |_, a| (0x0800_0000 + a[0]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn map_constructors_set_the_fields_and_vtables() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + MAP_COUNT, 3);
        assert_eq!(e.call(0x009a_4290, &args![this, 0x25u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TMAP_VTABLE);
        assert_eq!(e.mem.u32(this + HASH_SIZE), 0x25);
        assert_eq!(e.mem.u32(this + MAP_COUNT), 0);
        assert_eq!(e.mem.u32(this + HASH_TABLE), 0x0800_0094);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x0800_0094, 0, 0x94]]);
        assert_eq!(e.call(0x009a_43b0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TMAP_BASE_VTABLE);
    }

    #[test]
    fn map_destructor_empties_then_destroys_the_base() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.call(0x009a_4420, &args![this]);
        assert_eq!(e.mem.u32(this), NI_TMAP_VTABLE);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![this]]);
        assert_eq!(calls(&e, NI_TMAP_BASE_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn target_map_destructor_deletes_each_value() {
        let mut e = engine();
        let values = Rc::new(RefCell::new(vec![0x6000u32, 0, 0x6100]));
        e.register(MAP_FIRST_POSITION, |_, _| 1u32.into_ret());
        let next = values.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let value = next.borrow_mut().remove(0);
            e.mem.set_u32(a[3], value);
            let left = u32::from(!next.borrow().is_empty());
            e.mem.set_u32(a[1], left);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x009a_3ec0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, TARGET_DATA_DELETE),
            vec![vec![0x6000, 1], vec![0x6100, 1]]
        );
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![this], vec![this]]);
        assert_eq!(calls(&e, NI_TMAP_BASE_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_TMAP_VTABLE);
        // Flag clear and an empty map: no delete at all.
        e.register(MAP_FIRST_POSITION, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009a_3ec0, &args![this, 2u32]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert!(calls(&e, MAP_GET_NEXT).is_empty());
        e.call(0x009a_3ef0, &args![this]);
        assert_eq!(calls(&e, NI_TMAP_BASE_DESTRUCT).len(), 2);
    }
}

#[cfg(test)]
mod n025_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Every callee outside this file that the batch's translations call, and those of
    /// this file's older functions they call (`00438af0`).
    const EXTERNAL: &[u32] = &[
        0x0040_1000,
        0x0040_1030,
        0x0040_4eb0,
        0x0040_4ee0,
        0x0040_5430,
        0x0044_ddc0,
        0x0045_37b0,
        0x0045_cec0,
        0x0055_9450,
        0x0063_3c90,
        0x006a_7ad0,
        0x006b_3eb0,
        0x007c_b2e0,
        0x0084_4700,
        0x0085_3130,
        0x0087_ac60,
        0x008c_1cb0,
        0x008c_7aa0,
        0x0096_11e0,
        0x009a_4320,
        0x009a_4480,
        0x0043_8af0,
    ];

    const NODE_VTABLE: u32 = 0x0200_0000;
    const NODE_SLOTS: u32 = 0x0300_0000;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| ret(eax));
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn logged(e: &mut Engine, f: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        f(e);
        e.call_log.take().unwrap()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &addr in EXTERNAL {
            stub(&mut e, addr, 0);
        }
        let slots: Vec<u32> = (0..0x40).map(|s| NODE_SLOTS + 4 * s).collect();
        for &slot in &slots {
            stub(&mut e, slot, 0);
        }
        e.put_vtable(NODE_VTABLE, &slots);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e
    }

    /// A lookup double that finds `data` for `target` only.
    fn lookup(e: &mut Engine, target: u32, data: u32) {
        e.register_double(MAP_LOOKUP, move |e, a| {
            if a[1] == target {
                e.mem.set_u32(a[2], data);
                ret(1)
            } else {
                ret(0)
            }
        });
    }

    #[test]
    fn target_data_destructor_detaches_its_geometry() {
        let mut e = engine();
        let data = e.mem.alloc(0x14);
        // No geometry: only the destructors.
        let log = logged(&mut e, |e| {
            e.call(0x009a_3db0, &args![data]);
        });
        assert_eq!(calls(&log, 0x0045_cec0), vec![vec![data + 0x10]]);
        assert_eq!(calls(&log, 0x008c_1cb0), vec![vec![data]]);
        assert!(calls(&log, 0x0096_11e0).is_empty());
        // Geometry with a parent, off the main path: the parent's slot 0xe8.
        stub(&mut e, 0x0055_9450, 0x700);
        let parent = e.mem.alloc(8);
        e.mem.set_u32(parent, NODE_VTABLE);
        stub(&mut e, 0x0096_11e0, parent);
        let log = logged(&mut e, |e| {
            e.call(0x009a_3db0, &args![data]);
        });
        assert_eq!(calls(&log, NODE_SLOTS + 0xe8), vec![vec![parent, 0x700]]);
        // `008c7aa0` set: through `0087ac60` on `004537b0`'s object.
        stub(&mut e, 0x008c_7aa0, 1);
        stub(&mut e, 0x0045_37b0, 0x900);
        let log = logged(&mut e, |e| {
            e.call(0x009a_3fb0, &args![data, 1u32]);
        });
        assert_eq!(calls(&log, 0x0087_ac60), vec![vec![0x900, 0x700]]);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![data]]);
        let log = logged(&mut e, |e| {
            e.call(0x009a_3fb0, &args![data, 0u32]);
        });
        assert!(calls(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn add_target_attacker_creates_data_once() {
        let mut e = engine();
        let added = Rc::new(RefCell::new(vec![]));
        let keep = added.clone();
        e.register_double(0x007c_b2e0, move |e, a| {
            keep.borrow_mut().push((a[0], e.mem.u32(a[1])));
            ret(0)
        });
        let log = logged(&mut e, |e| {
            e.call(0x009a_3fe0, &args![0x5000u32, 0x10u32, 0x20u32]);
        });
        let stored = calls(&log, 0x0084_4700);
        assert_eq!(stored.len(), 1);
        let data = stored[0][2];
        assert_eq!(stored[0][..2], [0x5000, 0x10]);
        assert_eq!(e.mem.u32(data), ACTOR_ARRAY_VTABLE);
        assert_eq!(calls(&log, 0x006b_3eb0), vec![vec![data, 5, 0]]);
        assert_eq!(calls(&log, 0x0063_3c90), vec![vec![data + 0x10, 0]]);
        assert_eq!(*added.borrow(), vec![(data, 0x20)]);
        let guard = calls(&log, SCOPE_GUARD_ENTER);
        assert_eq!(guard[0][1..], [0x2b, 1, TARGET_MAP_SOURCE_FILE, 0x7c]);
        assert_eq!(calls(&log, SCOPE_GUARD_LEAVE), vec![vec![guard[0][0]]]);
        // Found: added to the existing data.
        lookup(&mut e, 0x10, 0x7000);
        let log = logged(&mut e, |e| {
            e.call(0x009a_3fe0, &args![0x5000u32, 0x10u32, 0x30u32]);
        });
        assert!(calls(&log, 0x0084_4700).is_empty());
        assert_eq!(added.borrow()[1], (0x7000, 0x30));
    }

    #[test]
    fn remove_target_attacker_removes_the_entry_or_the_target() {
        let mut e = engine();
        let entry = e.mem.alloc(4);
        e.mem.set_u32(entry, 0x20);
        let data = e.mem.alloc(0x14);
        lookup(&mut e, 0x10, data);
        stub(&mut e, 0x006a_7ad0, entry);
        stub(&mut e, 0x0044_ddc0, 2);
        let log = logged(&mut e, |e| {
            e.call(0x009a_4120, &args![0x5000u32, 0x10u32, 0x20u32]);
        });
        assert_eq!(calls(&log, 0x009a_4320), vec![vec![data, 0, 1]]);
        assert!(calls(&log, MAP_REMOVE_AT).is_empty());
        // The last attacker: the data deleted and the target removed.
        stub(&mut e, 0x0044_ddc0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x009a_4120, &args![0x5000u32, 0x10u32, 0x20u32]);
        });
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![data]]);
        assert_eq!(calls(&log, MAP_REMOVE_AT), vec![vec![0x5000, 0x10]]);
        // Another attacker or target: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x009a_4120, &args![0x5000u32, 0x10u32, 0x21u32]);
            e.call(0x009a_4120, &args![0x5000u32, 0x11u32, 0x20u32]);
        });
        assert!(calls(&log, MAP_REMOVE_AT).is_empty());
        assert!(calls(&log, 0x009a_4320).is_empty());
    }

    #[test]
    fn remove_target_deletes_its_data() {
        let mut e = engine();
        let data = e.mem.alloc(0x14);
        lookup(&mut e, 0x10, data);
        let log = logged(&mut e, |e| {
            e.call(0x009a_41e0, &args![0x5000u32, 0x10u32]);
            e.call(0x009a_41e0, &args![0x5000u32, 0x11u32]);
        });
        assert_eq!(calls(&log, MAP_REMOVE_AT), vec![vec![0x5000, 0x10]]);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![data]]);
    }

    #[test]
    fn map_deleting_destructors() {
        let mut e = engine();
        let map = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009a_42c0, &args![map, 1u32]).u32(), map);
        });
        assert_eq!(e.mem.u32(map), NI_TMAP_VTABLE);
        assert_eq!(calls(&log, MAP_REMOVE_ALL), vec![vec![map]]);
        assert_eq!(calls(&log, NI_TMAP_BASE_DESTRUCT), vec![vec![map]]);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![map]]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009a_44b0, &args![map, 1u32]).u32(), map);
            e.call(0x009a_44b0, &args![map, 0u32]);
        });
        assert_eq!(calls(&log, NI_TMAP_BASE_DESTRUCT).len(), 2);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![map]]);
        assert!(calls(&log, MAP_REMOVE_ALL).is_empty());
    }

    #[test]
    fn actor_array_constructor_sets_its_vtable() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x009a_42f0, &args![array, 4u32, 2u32]).u32(), array);
        });
        assert_eq!(e.mem.u32(array), ACTOR_ARRAY_VTABLE);
        assert_eq!(calls(&log, 0x006b_3eb0), vec![vec![array, 4, 2]]);
    }
}
