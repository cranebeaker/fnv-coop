//! `fallout/ai/lipsyncbackgroundmanager.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00905e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that hands `this` (`ECX`) to `00906160` with the three words 10, 0x25 and 0xc
/// (pushed 0xc, 0x25, 0xa, so 10 is the first argument), and returns `this`. The Ghidra
/// signature shows one word; the function ends in a plain `RET`.
pub fn fn_00905e30(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00906160(e, this, 10, 0x25, 0xc);
    this
}

// Translated from 00906160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00906160 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the object `00905e30` builds (the engine map names it only by its source
/// unit). Parameters: three words after `this`, `RET 0xc`: `rows` (the first), `slots` (the
/// second) and `extra` (the third). Order of events: `0044d7f0(this)` (vtable `01017458`),
/// the vtable `0108864c` at `+0`, `0096a2d0` on the member at `+0x20`, then, between the scope
/// guard `00404eb0` .. `00404ee0` (id 6, mode 1, source file and line 0x390 of
/// `ThreadSafeStructures.inl`): `+0x18` = 0, `+8` = `slots`, `+0xc` = an array of `slots`
/// words (size `slots * 4`, `0xFFFFFFFF` on overflow; each word cleared by the vector
/// constructor iterator `00401050` with the constructor `0044dee0`, none if the allocation
/// failed), `+4` = a block of `rows * 3 * 4` bytes (the product of the first two factors
/// wraps at 32 bits, the last multiplication saturates), `+0x10` = `extra`, and `+0x14` = a
/// 16-byte object built by `006c73b0(object, rows)` (0 if the allocation failed). The
/// exception-unwinding frame is not translated. Returns `this`.
pub fn fn_00906160(e: &mut Engine, this: Ptr, rows: u32, slots: u32, extra: u32) -> Ptr {
    let base = this.addr();
    e.call(0x0044_d7f0, &args![this]);
    e.mem.set_u32(base, 0x0108_864c);
    e.call(0x0096_a2d0, &args![base + 0x20]);
    e.with_stack(4, |e, scope| {
        e.call(
            0x0040_4eb0,
            &args![scope, 6u32, 1u32, 0x0101_73f8u32, 0x390u32],
        );
        e.mem.set_u32(base + 0x18, 0);
        e.mem.set_u32(base + 8, slots);
        let slot_count = e.mem.u32(base + 8);
        let slot_bytes = slot_count.saturating_mul(4);
        let slot_block = e.call(0x0040_1000, &args![slot_bytes]).u32();
        if slot_block != 0 {
            e.call(
                0x0040_1050,
                &args![slot_block, 4u32, slot_count, 0x0044_dee0u32],
            );
        }
        e.mem.set_u32(base + 0xc, slot_block);
        let row_bytes = rows.wrapping_mul(3).saturating_mul(4);
        let row_block = e.call(0x0040_1000, &args![row_bytes]).u32();
        e.mem.set_u32(base + 4, row_block);
        e.mem.set_u32(base + 0x10, extra);
        let object = e.call(0x0040_1000, &args![0x10u32]).u32();
        let built = if object != 0 {
            e.call(0x006c_73b0, &args![object, rows]).u32()
        } else {
            0
        };
        e.mem.set_u32(base + 0x14, built);
        e.call(0x0040_4ee0, &args![scope]);
    });
    this
}

// Translated from 00905c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00c3cea0`, which stores the vtable `010c1554` at
/// `+0`, calls `00aa4060` on the word at `+0x28` and ends with `00c3c620`, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `00c3cea0` ends with `CALL 00c3c620` and no result of its
/// own (Ghidra types both void).
pub fn fn_00905c30(e: &mut Engine, this: Ptr) {
    e.call(0x00c3_cea0, &args![this]);
}

/// `LockFreeMap<TESObjectREFR*,NiPointer<QueuedReference>>`'s vtable (RTTI,
/// read through its complete object locator; the name is cut at 64 bytes).
const QUEUED_REFERENCE_MAP_VTABLE: u32 = 0x0101_747c;
/// `LockFreeMap<AnimIdle*,NiPointer<QueuedAnimIdle>>`'s vtable (RTTI).
const QUEUED_ANIM_IDLE_MAP_VTABLE: u32 = 0x0101_74cc;
/// The `LockFreeMapIterator` vtable of `LockFreeMap<MobileObject*,
/// NiPointer<LipTask>>` (RTTI).
const LIP_TASK_MAP_ITERATOR_VTABLE: u32 = 0x0108_8640;
/// `operator new` (cdecl(size)).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (cdecl(block)).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `NiPointer` construction from a raw pointer (`00528cb0`, thiscall(raw)).
const NI_POINTER_CONSTRUCT: u32 = 0x0052_8cb0;
/// `NiPointer` destructor (`0044cbf0`, thiscall).
const NI_POINTER_DESTRUCT: u32 = 0x0044_cbf0;
/// The pointer a `NiPointer` holds (`00559450`, thiscall).
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `BSTaskManager<__int64>::CancelTask` (engine map, thiscall(task, 0)).
const TASK_MANAGER_CANCEL_TASK: u32 = 0x0044_ac40;
/// The task manager `CancelTask` runs on (a pointer global).
const TASK_MANAGER_POINTER: u32 = 0x0120_2d98;
/// The map's removal of a mobile object's entry (`009060f0`,
/// thiscall(mobile object)).
const MAP_REMOVE_ENTRY: u32 = 0x0090_60f0;
/// The map's iterator step (`00906570`, thiscall(iterator, key out,
/// value out, 1), bool).
const MAP_ITERATE: u32 = 0x0090_6570;
/// The map's call after the cancel loop (`009064e0`, thiscall).
const MAP_AFTER_CANCEL: u32 = 0x0090_64e0;
/// The iterator's end test (`006ebde0`, thiscall, bool).
const ITERATOR_AT_END: u32 = 0x006e_bde0;

/// The constructor body `0044b750` and `0044ba10` share with `00906160`
/// (each with its own vtable at `+0`): `0044d7f0(this)`, the vtable,
/// `0096a2d0` on `+0x20`, then between the scope guard `00404eb0` ..
/// `00404ee0` (id 6, mode 1, `ThreadSafeStructures.inl`, line 0x390):
/// `+0x18` = 0, `+8` = `slots`, `+0xC` = `slots` words (size `slots * 4`,
/// all ones on overflow; each cleared by `00401050` with `0044dee0`, null
/// when the allocation failed), `+4` = `rows * 3 * 4` bytes (the first
/// product wraps, the second gives all ones on overflow), `+0x10` =
/// `extra`, `+0x14` = a 16-byte object from `006c73b0(object, rows)` (null
/// when the allocation failed).
fn lock_free_map_construct(
    e: &mut Engine,
    this: Ptr,
    vtable: u32,
    rows: u32,
    slots: u32,
    extra: u32,
) -> Ptr {
    let base = this.addr();
    e.call(0x0044_d7f0, &args![this]);
    e.mem.set_u32(base, vtable);
    e.call(0x0096_a2d0, &args![base + 0x20]);
    e.with_stack(4, |e, scope| {
        e.call(
            0x0040_4eb0,
            &args![scope, 6u32, 1u32, 0x0101_73f8u32, 0x390u32],
        );
        e.mem.set_u32(base + 0x18, 0);
        e.mem.set_u32(base + 8, slots);
        let slot_count = e.mem.u32(base + 8);
        let slot_bytes = slot_count as u64 * 4;
        let slot_size = (slot_bytes as u32)
            | if slot_bytes > u32::MAX as u64 {
                u32::MAX
            } else {
                0
            };
        let slot_block = e.call(OPERATOR_NEW, &args![slot_size]).u32();
        if slot_block != 0 {
            e.call(
                0x0040_1050,
                &args![slot_block, 4u32, slot_count, 0x0044_dee0u32],
            );
        }
        e.mem.set_u32(base + 0xc, slot_block);
        let row_bytes = rows.wrapping_mul(3) as u64 * 4;
        let row_size = (row_bytes as u32)
            | if row_bytes > u32::MAX as u64 {
                u32::MAX
            } else {
                0
            };
        let row_block = e.call(OPERATOR_NEW, &args![row_size]).u32();
        e.mem.set_u32(base + 4, row_block);
        e.mem.set_u32(base + 0x10, extra);
        let object = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let built = if object != 0 {
            e.call(0x006c_73b0, &args![object, rows]).u32()
        } else {
            0
        };
        e.mem.set_u32(base + 0x14, built);
        e.call(0x0040_4ee0, &args![scope]);
    });
    this
}

// Translated from 0044b730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<MobileObject*,NiPointer<LipTask>>::DecrementCount` (Xbox
/// PDB): `004019a0` (cdecl) on the count at `+0x18` (the engine map's
/// `WaitForSingleObjectEx` name sits on that folded body).
pub fn lock_free_map_decrement_count(e: &mut Engine, this: Ptr) {
    e.call(0x0040_19a0, &args![this.addr().wrapping_add(0x18)]);
}

// Translated from 0044b750 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044b750 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The constructor of `LockFreeMap<TESObjectREFR*,NiPointer<QueuedReference>>`
/// (its vtable `0101747c`; the body `00906160` has for the lip-task map):
/// thiscall(`rows`, `slots`, `extra`), returns `this`.
pub fn fn_0044b750(e: &mut Engine, this: Ptr, rows: u32, slots: u32, extra: u32) -> Ptr {
    lock_free_map_construct(e, this, QUEUED_REFERENCE_MAP_VTABLE, rows, slots, extra)
}

// Translated from 0044b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map's iteration step (thiscall(iterator, value out, key, keep
/// going), bool): under the lock at `+0x20` (`0040fbf0(0)` .. `0040fba0`),
/// nothing when the iterator is at its end (`006ebde0`); otherwise, from
/// the iterator's bucket index (`+4`) and entry (`+8`), the lookup through
/// `00449f80(this)` is `0044e550` (then `0044de40(iterator, 1)`) when
/// `0044de20` says no, else `0044e680`. On a hit the iterator's entry goes
/// to `*out`; on a miss the index moves on, and the walk ends at the
/// bucket count (`0044ddc0`, then `0044de80(iterator, 1)`) or when
/// `keep_going` is 0, else continues after `0044de40(iterator, 0)`. A hit
/// is finally checked with `00528bb0(00449f80(this), *out)`.
pub fn fn_0044b8d0(
    e: &mut Engine,
    this: Ptr,
    iterator: Ptr,
    out: Ptr,
    key: u32,
    keep_going: u8,
) -> bool {
    let lock = this.addr().wrapping_add(0x20);
    e.call(0x0040_fbf0, &args![lock, 0u32]);
    let iter = iterator.addr();
    let mut found = false;
    if !e.call(ITERATOR_AT_END, &args![iter]).bool() {
        // `MOV ECX,1; TEST ECX,ECX; JZ`: the loop condition is constant.
        loop {
            if !e.call(0x0044_de20, &args![iter]).bool() {
                let index = e.mem.u32(iter.wrapping_add(4));
                let buckets = e.call(0x0044_9f80, &args![this]).u32();
                found = e
                    .call(
                        0x0044_e550,
                        &args![buckets, index, iter.wrapping_add(8), key],
                    )
                    .bool();
                e.call(0x0044_de40, &args![iter, 1u32]);
            } else {
                let index = e.mem.u32(iter.wrapping_add(4));
                let buckets = e.call(0x0044_9f80, &args![this]).u32();
                found = e
                    .call(
                        0x0044_e680,
                        &args![buckets, index, iter.wrapping_add(8), key],
                    )
                    .bool();
            }
            if found {
                let entry = e.mem.u32(iter.wrapping_add(8));
                e.mem.set_u32(out.addr(), entry);
                break;
            }
            let index = e.mem.u32(iter.wrapping_add(4)).wrapping_add(1);
            e.mem.set_u32(iter.wrapping_add(4), index);
            let count = e.call(0x0044_ddc0, &args![this]).u32();
            if e.mem.u32(iter.wrapping_add(4)) >= count {
                e.call(0x0044_de80, &args![iter, 1u32]);
                break;
            }
            if keep_going == 0 {
                break;
            }
            e.call(0x0044_de40, &args![iter, 0u32]);
        }
    }
    if found {
        let value = e.mem.u32(out.addr());
        let buckets = e.call(0x0044_9f80, &args![this]).u32();
        found = e.call(0x0052_8bb0, &args![buckets, value]).bool();
    }
    e.call(0x0040_fba0, &args![lock]);
    found
}

// Translated from 0044ba10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044ba10 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The constructor of `LockFreeMap<AnimIdle*,NiPointer<QueuedAnimIdle>>`
/// (its vtable `010174cc`; the same body as `0044b750`): thiscall(`rows`,
/// `slots`, `extra`), returns `this`.
pub fn fn_0044ba10(e: &mut Engine, this: Ptr, rows: u32, slots: u32, extra: u32) -> Ptr {
    lock_free_map_construct(e, this, QUEUED_ANIM_IDLE_MAP_VTABLE, rows, slots, extra)
}

// Translated from 0044bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<MobileObject*,NiPointer<LipTask>>::IncrementCount` (Xbox
/// PDB): `0040b460` (cdecl) on the count at `+0x18`.
pub fn lock_free_map_increment_count(e: &mut Engine, this: Ptr) {
    e.call(0x0040_b460, &args![this.addr().wrapping_add(0x18)]);
}

// Translated from 0063fa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LipTask::scalar deleting destructor` (Xbox PDB): the destructor body
/// (`00905c30`), then `operator delete(this)` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn lip_task_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00905c30(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00905e50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00905e50 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Cancels every lip task of the map: with a stack iterator (`00906690`),
/// while it is not at its end, each step (`00906570(iterator, key, task, 1)`
/// into a stack mobile-object word and a stack `NiPointer` built empty by
/// `00528cb0`) that gives an entry removes it (`009060f0(key)`) and cancels
/// its task (`BSTaskManager::CancelTask(task, 0)` on the manager at
/// `01202d98`); the `NiPointer` is released (`0044cbf0`) after each step.
/// Then the iterator's destructor (`00906110`) and `009064e0(this)`.
pub fn fn_00905e50(e: &mut Engine, this: Ptr) {
    e.with_stack(0x10, |e, iterator| {
        fn_00906690(e, iterator);
        while !e.call(ITERATOR_AT_END, &args![iterator]).bool() {
            e.with_stack(4, |e, key| {
                e.with_stack(4, |e, task| {
                    e.mem.set_u32(key.addr(), 0);
                    e.call(NI_POINTER_CONSTRUCT, &args![task, 0u32]);
                    let found = e
                        .call(MAP_ITERATE, &args![this, iterator, key, task, 1u32])
                        .bool();
                    if found {
                        let mobile = e.mem.u32(key.addr());
                        e.call(MAP_REMOVE_ENTRY, &args![this, mobile]);
                        let raw = e.call(NI_POINTER_GET, &args![task]).u32();
                        let manager = e.global::<u32>(TASK_MANAGER_POINTER);
                        e.call(TASK_MANAGER_CANCEL_TASK, &args![manager, raw, 0u32]);
                    }
                    e.call(NI_POINTER_DESTRUCT, &args![task]);
                });
            });
        }
        fn_00906110(e, iterator);
    });
    e.call(MAP_AFTER_CANCEL, &args![this]);
}

// Translated from 00906050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00906050 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `LipSyncBackgroundManager::CancelLipFile` (Xbox PDB): looks the mobile
/// object up through the map's virtual `+8` (into a stack `NiPointer` built
/// empty); when found, removes its entry (`009060f0`) and cancels its task
/// (`BSTaskManager::CancelTask(task, 0)` on the manager at `01202d98`).
/// The `NiPointer` is released at the end.
pub fn lip_sync_background_manager_cancel_lip_file(e: &mut Engine, this: Ptr, mobile: u32) {
    e.with_stack(4, |e, task| {
        e.call(NI_POINTER_CONSTRUCT, &args![task, 0u32]);
        if e.vcall(this.addr(), 8, &args![mobile, task]).bool() {
            e.call(MAP_REMOVE_ENTRY, &args![this, mobile]);
            let raw = e.call(NI_POINTER_GET, &args![task]).u32();
            let manager = e.global::<u32>(TASK_MANAGER_POINTER);
            e.call(TASK_MANAGER_CANCEL_TASK, &args![manager, raw, 0u32]);
        }
        e.call(NI_POINTER_DESTRUCT, &args![task]);
    });
}

// Translated from 00906110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lip-task map iterator's destructor body: stores its vtable.
pub fn fn_00906110(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LIP_TASK_MAP_ITERATOR_VTABLE);
}

// Translated from 00906690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lip-task map iterator's constructor: its vtable, `+4` and `+8`
/// cleared, the byte at `+0xC` cleared. Returns `this`.
pub fn fn_00906690(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), LIP_TASK_MAP_ITERATOR_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u8(this.addr() + 0xc, 0);
    this
}

/// The word at +0x68 of an object (`008d8520`, thiscall; the engine map's
/// `MiddleHighProcess::GetSavedAcquireObject` name sits on that body). Lip
/// tasks call it on their mobile object and use the result's virtual
/// +0x370, +0x374 and +0x380.
const OBJECT_WORD_68: u32 = 0x008d_8520;
/// The `LipTask` vtable `00905ba0` stores.
const LIP_TASK_VTABLE: u32 = 0x0108_85b8;
/// The source file name the lip-task functions pass to the scope guard
/// (`"D:\_Fallout3\...\LipSyncBackgroundManager.cpp"`).
const LIP_SYNC_SOURCE_FILE: u32 = 0x0108_85e8;
/// The scope guard's constructor (`00404eb0`, thiscall(id, mode, file,
/// line)) and destructor (`00404ee0`, thiscall).
const SCOPE_GUARD_BEGIN: u32 = 0x0040_4eb0;
const SCOPE_GUARD_END: u32 = 0x0040_4ee0;

// Translated from 0044bb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lip-task map's lookup forwarder (thiscall(`first`, `second`), `RET
/// 8`, bool): `0044c8e0(00449f80(this), first, second)`.
pub fn fn_0044bb90(e: &mut Engine, this: Ptr, first: u32, second: u32) -> bool {
    let buckets = e.call(0x0044_9f80, &args![this]).u32();
    e.call(0x0044_c8e0, &args![buckets, first, second]).bool()
}

// Translated from 00905ba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00905ba0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `LipTask` constructor (thiscall(`manager`, `mobile`, `third`), `RET
/// 0xC`): the base constructor `00c3ce60(this, 0)`, the vtable `010885b8`,
/// `+0x30` = `mobile`, `+0x34` = 0, `+0x38` = `manager`, then
/// `00c3cee0(this, third)` and `00c3cf60(this, 4)`. Returns `this`.
pub fn fn_00905ba0(e: &mut Engine, this: Ptr, manager: u32, mobile: u32, third: u32) -> Ptr {
    let base = this.addr();
    e.call(0x00c3_ce60, &args![this, 0u32]);
    e.mem.set_u32(base, LIP_TASK_VTABLE);
    e.mem.set_u32(base + 0x30, mobile);
    e.mem.set_u32(base + 0x34, 0);
    e.mem.set_u32(base + 0x38, manager);
    e.call(0x00c3_cee0, &args![this, third]);
    e.call(0x00c3_cf60, &args![this, 4u32]);
    this
}

// Translated from 00905c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00905c50 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `LipTask::Run` (Xbox PDB): inside the scope guard (id 0x35, mode 1,
/// `LipSyncBackgroundManager.cpp`, line 0x15) takes the file
/// (`QueuedFileEntry::GetFile`, `00c3cff0(this, 4, 0x10)`); without one,
/// the mobile object's `008d8520` object gets virtual +0x370(0) and then
/// +0x374(1). `+0x34` becomes `004d5350(file)` (cdecl; the engine map's
/// `LipSynchAnim::LoadFromFile_ov2`), null file included.
pub fn lip_task_run(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_GUARD_BEGIN,
            &args![scope, 0x35u32, 1u32, LIP_SYNC_SOURCE_FILE, 0x15u32],
        );
        let file = e.call(0x00c3_cff0, &args![this, 4u32, 0x10u32]).u32();
        if file == 0 {
            let mobile = e.mem.u32(base + 0x30);
            let object = e.call(OBJECT_WORD_68, &args![mobile]).u32();
            e.vcall(object, 0x370, &args![0u32]);
            let mobile = e.mem.u32(base + 0x30);
            let object = e.call(OBJECT_WORD_68, &args![mobile]).u32();
            e.vcall(object, 0x374, &args![1u32]);
        }
        let anim = e.call(0x004d_5350, &args![file]).u32();
        e.mem.set_u32(base + 0x34, anim);
        e.call(SCOPE_GUARD_END, &args![scope]);
    });
}

// Translated from 00905d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LipTask::Finish` (Xbox PDB): when the mobile object's `008d8520`
/// object is not null, it gets virtual +0x380(`+0x34`) and +0x370(0), and,
/// when `+0x34` is null, +0x374(1) (the object fetched again for each
/// call). Then the manager (`+0x38`) drops the mobile object's entry
/// (`009060f0`).
pub fn lip_task_finish(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mobile = e.mem.u32(base + 0x30);
    if e.call(OBJECT_WORD_68, &args![mobile]).u32() != 0 {
        let mobile = e.mem.u32(base + 0x30);
        let object = e.call(OBJECT_WORD_68, &args![mobile]).u32();
        let anim = e.mem.u32(base + 0x34);
        e.vcall(object, 0x380, &args![anim]);
        let mobile = e.mem.u32(base + 0x30);
        let object = e.call(OBJECT_WORD_68, &args![mobile]).u32();
        e.vcall(object, 0x370, &args![0u32]);
        if e.mem.u32(base + 0x34) == 0 {
            let mobile = e.mem.u32(base + 0x30);
            let object = e.call(OBJECT_WORD_68, &args![mobile]).u32();
            e.vcall(object, 0x374, &args![1u32]);
        }
    }
    let mobile = e.mem.u32(base + 0x30);
    let manager = Ptr::new(e.mem.u32(base + 0x38));
    fn_009060f0(e, manager, mobile);
}

// Translated from 00905dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LipTask::Cancel` (Xbox PDB; thiscall(`state`, `second`), `RET 8`):
/// `QueuedFile::Cancel` (`00c3cb70(this, state, second)`), then, unless
/// `state` is 5, the manager (`+0x38`) drops the mobile object's entry
/// (`009060f0(+0x30)`).
pub fn lip_task_cancel(e: &mut Engine, this: Ptr, state: u32, second: u32) {
    let base = this.addr();
    e.call(0x00c3_cb70, &args![this, state, second]);
    if state != 5 {
        let mobile = e.mem.u32(base + 0x30);
        let manager = Ptr::new(e.mem.u32(base + 0x38));
        fn_009060f0(e, manager, mobile);
    }
}

// Translated from 00905e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LipTask::GetDescription` (Xbox PDB; thiscall(`buffer`, `size`), `RET
/// 8`): `QueuedFileEntry::GetDescription(this, buffer, size, 01088634)`
/// (the last word the address of the task's description text), whose answer is this one's (the
/// exe ends on that call and keeps its `AL`; the PDB's `bool`).
pub fn lip_task_get_description(e: &mut Engine, this: Ptr, buffer: u32, size: u32) -> bool {
    e.call(0x00c3_d0e0, &args![this, buffer, size, 0x0108_8634u32])
        .bool()
}

// Translated from 00905f30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00905f30 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Queues a lip file for a mobile object (thiscall(`mobile`, `third`),
/// `RET 8`): inside the scope guard (id 0x35, mode 1,
/// `LipSyncBackgroundManager.cpp`, line 0x61), cancels the object's lip
/// file (`CancelLipFile`), builds a `LipTask` (0x40 bytes,
/// `00905ba0(task, this, mobile, third)`; null when the allocation
/// failed) held in a stack `NiPointer` (`00528cb0`), adds it through the
/// map's virtual +0x10(`mobile`, `&pointer`, 0) (its result unused),
/// releases the `NiPointer` (`0044cbf0`), calls virtual +0x374(0) on the
/// mobile object's `008d8520` object and hands the task to the task
/// manager at `01202d98` (its virtual +0x48).
pub fn fn_00905f30(e: &mut Engine, this: Ptr, mobile: u32, third: u32) {
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_GUARD_BEGIN,
            &args![scope, 0x35u32, 1u32, LIP_SYNC_SOURCE_FILE, 0x61u32],
        );
        lip_sync_background_manager_cancel_lip_file(e, this, mobile);
        let block = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
        let task = if block != 0 {
            fn_00905ba0(e, Ptr::new(block), this.addr(), mobile, third).addr()
        } else {
            0
        };
        e.with_stack(4, |e, pointer| {
            e.call(NI_POINTER_CONSTRUCT, &args![pointer, task]);
            e.vcall(this.addr(), 0x10, &args![mobile, pointer, 0u32]);
            e.call(NI_POINTER_DESTRUCT, &args![pointer]);
        });
        let object = e.call(OBJECT_WORD_68, &args![mobile]).u32();
        e.vcall(object, 0x374, &args![0u32]);
        let manager = e.global::<u32>(TASK_MANAGER_POINTER);
        e.vcall(manager, 0x48, &args![task]);
        e.call(SCOPE_GUARD_END, &args![scope]);
    });
}

// Translated from 009060f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops a mobile object's entry from the lip-task map (thiscall(`key`),
/// `RET 4`): the map's virtual +0x14(`key`).
pub fn fn_009060f0(e: &mut Engine, this: Ptr, key: u32) {
    e.vcall(this.addr(), 0x14, &args![key]);
}

// Translated from 00906130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<MobileObject*,NiPointer<LipTask>>::LockFreeMapIterator::
/// scalar deleting destructor` (Xbox PDB): the destructor body
/// (`00906110`), then `operator delete(this)` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn lip_task_map_iterator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00906110(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_bb90, fn_0044bb90(Ptr, u32, u32) -> bool),
        entry!(0x0090_5ba0, fn_00905ba0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0090_5c50, lip_task_run(Ptr)),
        entry!(0x0090_5d20, lip_task_finish(Ptr)),
        entry!(0x0090_5dc0, lip_task_cancel(Ptr, u32, u32)),
        entry!(0x0090_5e00, lip_task_get_description(Ptr, u32, u32) -> bool),
        entry!(0x0090_5f30, fn_00905f30(Ptr, u32, u32)),
        entry!(0x0090_60f0, fn_009060f0(Ptr, u32)),
        entry!(
            0x0090_6130,
            lip_task_map_iterator_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0090_5c30, fn_00905c30(Ptr)),
        entry!(0x0090_5e30, fn_00905e30(Ptr) -> Ptr),
        entry!(0x0090_6160, fn_00906160(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044_b730, lock_free_map_decrement_count(Ptr)),
        entry!(0x0044_b750, fn_0044b750(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044_b8d0, fn_0044b8d0(Ptr, Ptr, Ptr, u32, u8) -> bool),
        entry!(0x0044_ba10, fn_0044ba10(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044_bbc0, lock_free_map_increment_count(Ptr)),
        entry!(
            0x0063_fa90,
            lip_task_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0090_5e50, fn_00905e50(Ptr)),
        entry!(
            0x0090_6050,
            lip_sync_background_manager_cancel_lip_file(Ptr, u32)
        ),
        entry!(0x0090_6110, fn_00906110(Ptr)),
        entry!(0x0090_6690, fn_00906690(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_00905c30_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00c3_cea0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0090_5c30, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00c3_cea0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod process_list_tests {
    use super::*;

    const NEW: u32 = 0x0040_1000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Doubles for every callee outside this file; `NEW` hands out 0x100-byte blocks.
    fn doubles(e: &mut Engine) {
        for address in [
            0x0044_d7f0,
            0x0096_a2d0,
            0x0040_4eb0,
            0x0040_4ee0,
            0x0040_1050,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x006c_73b0, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        e.register_double(NEW, |e, _| Ret {
            eax: e.mem.alloc(0x100),
            ..Ret::default()
        });
    }

    #[test]
    fn constructor_forwards_ten_0x25_and_0xc() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        let result = e.call(0x0090_5e30, &args![this]).u32();
        assert_eq!(result, this);
        // 0x25 slots of four bytes, ten rows of three words.
        assert_eq!(
            calls_to(&e, NEW).iter().map(|w| w[0]).collect::<Vec<_>>(),
            vec![0x25 * 4, 10 * 3 * 4, 0x10]
        );
        assert_eq!(e.mem.u32(this + 8), 0x25);
        assert_eq!(e.mem.u32(this + 0x10), 0xc);
        assert_eq!(calls_to(&e, 0x006c_73b0)[0][1], 10);
    }

    #[test]
    fn body_fills_the_fields_between_the_scope_guard() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x0090_6160, &args![this, 2u32, 3u32, 7u32]);
        assert_eq!(e.mem.u32(this), 0x0108_864c);
        assert_eq!(calls_to(&e, 0x0044_d7f0), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x0096_a2d0), vec![vec![this + 0x20]]);
        let guard = calls_to(&e, 0x0040_4eb0);
        assert_eq!(guard[0][1..], [6, 1, 0x0101_73f8, 0x390]);
        assert_eq!(calls_to(&e, 0x0040_4ee0), vec![vec![guard[0][0]]]);
        let slot_block = e.mem.u32(this + 0xc);
        assert_eq!(
            calls_to(&e, 0x0040_1050),
            vec![vec![slot_block, 4, 3, 0x0044_dee0]]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0x10), 7);
        let object = e.mem.u32(this + 0x14);
        assert_eq!(calls_to(&e, 0x006c_73b0), vec![vec![object - 0x1000, 2]]);
    }

    #[test]
    fn sizes_saturate_and_failed_allocations_skip_the_dependents() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(NEW, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0090_6160,
            &args![this, 0x4000_0000u32, 0x4000_0000u32, 1u32],
        );
        // 0x4000_0000 * 3 wraps to 0xC000_0000 and times four overflows: saturated.
        assert_eq!(
            calls_to(&e, NEW).iter().map(|w| w[0]).collect::<Vec<_>>(),
            vec![0xffff_ffff, 0xffff_ffff, 0x10]
        );
        assert!(calls_to(&e, 0x0040_1050).is_empty());
        assert!(calls_to(&e, 0x006c_73b0).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }
}

#[cfg(test)]
mod b016_tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    #[test]
    fn counts_go_through_the_word_at_0x18() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0040_19a0, 0x0040_b460]);
        e.call_log = Some(vec![]);
        e.call(0x0044_b730, &args![0x1000u32]);
        e.call(0x0044_bbc0, &args![0x2000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0040_19a0), vec![vec![0x1018]]);
        assert_eq!(calls(&log, 0x0040_b460), vec![vec![0x2018]]);
    }

    fn constructor_doubles(e: &mut Engine) {
        quiet(
            e,
            &[
                0x0044_d7f0,
                0x0096_a2d0,
                0x0040_4eb0,
                0x0040_4ee0,
                0x0040_1050,
            ],
        );
        e.register(0x006c_73b0, |_, a| (a[0] + 0x1000).into_ret());
        e.register(OPERATOR_NEW, |e, _| e.mem.alloc(0x100).into_ret());
    }

    #[test]
    fn map_constructors_store_their_own_vtables() {
        for (address, vtable) in [
            (0x0044_b750u32, QUEUED_REFERENCE_MAP_VTABLE),
            (0x0044_ba10, QUEUED_ANIM_IDLE_MAP_VTABLE),
        ] {
            let mut e = Engine::new();
            constructor_doubles(&mut e);
            let this = e.mem.alloc(0x40);
            e.call_log = Some(vec![]);
            let back = e.call(address, &args![this, 2u32, 3u32, 7u32]).u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(back, this);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(
                calls(&log, OPERATOR_NEW)
                    .iter()
                    .map(|w| w[0])
                    .collect::<Vec<_>>(),
                vec![12, 24, 0x10]
            );
            let slots = e.mem.u32(this + 0xc);
            assert_eq!(
                calls(&log, 0x0040_1050),
                vec![vec![slots, 4, 3, 0x0044_dee0]]
            );
            assert_eq!(e.mem.u32(this + 8), 3);
            assert_eq!(e.mem.u32(this + 0x10), 7);
            let object = e.mem.u32(this + 0x14);
            assert_eq!(calls(&log, 0x006c_73b0), vec![vec![object - 0x1000, 2]]);
            let guard = calls(&log, 0x0040_4eb0);
            assert_eq!(guard[0][1..], [6, 1, 0x0101_73f8, 0x390]);
        }
    }

    #[test]
    fn map_constructor_with_failed_allocations() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_b750,
            &args![this, 0x4000_0000u32, 0x4000_0000u32, 1u32],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, OPERATOR_NEW)
                .iter()
                .map(|w| w[0])
                .collect::<Vec<_>>(),
            vec![u32::MAX, u32::MAX, 0x10]
        );
        assert!(calls(&log, 0x0040_1050).is_empty());
        assert!(calls(&log, 0x006c_73b0).is_empty());
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    /// Doubles for the iteration step; `0044ddc0` gives 2 buckets.
    fn iteration_doubles(e: &mut Engine) {
        quiet(
            e,
            &[
                0x0040_fbf0,
                0x0040_fba0,
                ITERATOR_AT_END,
                0x0044_de20,
                0x0044_de40,
                0x0044_de80,
                0x0044_e550,
                0x0044_e680,
            ],
        );
        e.register(0x0044_9f80, |_, a| (a[0] + 0x100).into_ret());
        e.register(0x0044_ddc0, |_, _| 2u32.into_ret());
        e.register(0x0052_8bb0, |_, _| 1u32.into_ret());
    }

    #[test]
    fn iteration_step_finds_an_entry() {
        let mut e = Engine::new();
        iteration_doubles(&mut e);
        e.register(0x0044_e550, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x40);
        let iterator = e.mem.alloc(0x10);
        e.mem.set_u32(iterator + 8, 0x0e0e);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let found = e
            .call(0x0044_b8d0, &args![this, iterator, out, 0x55u32, 1u32])
            .bool();
        let log = e.call_log.take().unwrap();
        assert!(found);
        assert_eq!(e.mem.u32(out), 0x0e0e);
        assert_eq!(
            calls(&log, 0x0044_e550),
            vec![vec![this + 0x100, 0, iterator + 8, 0x55]]
        );
        assert_eq!(calls(&log, 0x0044_de40), vec![vec![iterator, 1]]);
        assert_eq!(calls(&log, 0x0052_8bb0), vec![vec![this + 0x100, 0x0e0e]]);
        assert_eq!(calls(&log, 0x0040_fbf0), vec![vec![this + 0x20, 0]]);
        assert_eq!(calls(&log, 0x0040_fba0), vec![vec![this + 0x20]]);
    }

    #[test]
    fn iteration_step_walks_to_the_end_or_stops() {
        let mut e = Engine::new();
        iteration_doubles(&mut e);
        // 0044de20 says yes: the 0044e680 lookup, never a hit.
        e.register(0x0044_de20, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x40);
        let iterator = e.mem.alloc(0x10);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let found = e
            .call(0x0044_b8d0, &args![this, iterator, out, 0x55u32, 1u32])
            .bool();
        let log = e.call_log.take().unwrap();
        assert!(!found);
        // Index 0, then 1; at 2 the walk ends.
        assert_eq!(calls(&log, 0x0044_e680).len(), 2);
        assert_eq!(calls(&log, 0x0044_de40), vec![vec![iterator, 0]]);
        assert_eq!(calls(&log, 0x0044_de80), vec![vec![iterator, 1]]);
        assert_eq!(e.mem.u32(iterator + 4), 2);
        assert!(calls(&log, 0x0052_8bb0).is_empty());

        // Without keep-going: one lookup.
        e.mem.set_u32(iterator + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_b8d0, &args![this, iterator, out, 0x55u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0044_e680).len(), 1);
        assert!(calls(&log, 0x0044_de80).is_empty());

        // At its end: nothing but the lock.
        e.register(ITERATOR_AT_END, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0044_b8d0, &args![this, iterator, out, 0x55u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x0044_de20).is_empty());
        assert_eq!(calls(&log, 0x0040_fba0).len(), 1);
    }

    #[test]
    fn lip_task_deleting_destructor() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x00c3_cea0, OPERATOR_DELETE]);
        e.call_log = Some(vec![]);
        let back = e.call(0x0063_fa90, &args![0x5000u32, 0u32]).u32();
        e.call(0x0063_fa90, &args![0x5000u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0x5000);
        assert_eq!(calls(&log, 0x00c3_cea0).len(), 2);
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![0x5000]]);
    }

    fn cancel_doubles(e: &mut Engine) {
        quiet(
            e,
            &[
                NI_POINTER_CONSTRUCT,
                NI_POINTER_DESTRUCT,
                TASK_MANAGER_CANCEL_TASK,
                MAP_REMOVE_ENTRY,
                MAP_AFTER_CANCEL,
            ],
        );
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.map(0x0120_2000, 0x1000);
        e.set_global(TASK_MANAGER_POINTER, 0x0abc_0000u32);
    }

    #[test]
    fn cancel_all_cancels_each_found_task() {
        let mut e = Engine::new();
        cancel_doubles(&mut e);
        // Two steps, the first finds mobile 0x77 with task 0x88; then the end.
        e.register_double(ITERATOR_AT_END, {
            let mut step = 0;
            move |_, _| {
                step += 1;
                ((step > 2) as u32).into_ret()
            }
        });
        e.register_double(MAP_ITERATE, {
            let mut step = 0;
            move |e, a| {
                step += 1;
                if step == 1 {
                    e.mem.set_u32(a[2], 0x77);
                    e.mem.set_u32(a[3], 0x88);
                }
                ((step == 1) as u32).into_ret()
            }
        });
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x0090_5e50, &args![this]);
        let log = e.call_log.take().unwrap();
        let iterate = calls(&log, MAP_ITERATE);
        assert_eq!(iterate.len(), 2);
        assert_eq!(iterate[0][0], this);
        assert_eq!(iterate[0][4], 1);
        assert_eq!(calls(&log, MAP_REMOVE_ENTRY), vec![vec![this, 0x77]]);
        assert_eq!(
            calls(&log, TASK_MANAGER_CANCEL_TASK),
            vec![vec![0x0abc_0000, 0x88, 0]]
        );
        assert_eq!(calls(&log, NI_POINTER_DESTRUCT).len(), 2);
        assert_eq!(calls(&log, MAP_AFTER_CANCEL), vec![vec![this]]);
    }

    #[test]
    fn cancel_lip_file_cancels_a_found_task() {
        let mut e = Engine::new();
        cancel_doubles(&mut e);
        e.register(0x0012_3400, |e, a| {
            e.mem.set_u32(a[2], 0x99);
            ((a[1] == 0x66) as u32).into_ret()
        });
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x0012_3400);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, vtable);
        e.call_log = Some(vec![]);
        e.call(0x0090_6050, &args![this, 0x66u32]);
        e.call(0x0090_6050, &args![this, 0x67u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, MAP_REMOVE_ENTRY), vec![vec![this, 0x66]]);
        assert_eq!(
            calls(&log, TASK_MANAGER_CANCEL_TASK),
            vec![vec![0x0abc_0000, 0x99, 0]]
        );
        assert_eq!(calls(&log, NI_POINTER_DESTRUCT).len(), 2);
    }

    #[test]
    fn iterator_constructor_and_destructor() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for offset in (0..0x10).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        let back = e.call(0x0090_6690, &args![this]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), LIP_TASK_MAP_ITERATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0xffff_ff00);
        e.mem.set_u32(this, 0);
        e.call(0x0090_6110, &args![this]);
        assert_eq!(e.mem.u32(this), LIP_TASK_MAP_ITERATOR_VTABLE);
    }
}

#[cfg(test)]
mod n026_tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];
    type Logged = Vec<(u32, Vec<u32>)>;

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// A process-like object whose virtual +0x370, +0x374 and +0x380 are
    /// recorded at the addresses 0x0abc_0370 .. 0x0abc_0380.
    const PROCESS_VTABLE: u32 = 0x0abd_0000;

    fn process(e: &mut Engine) -> u32 {
        let mut slots = vec![0u32; 0x384 / 4];
        slots[0x370 / 4] = 0x0abc_0370;
        slots[0x374 / 4] = 0x0abc_0374;
        slots[0x380 / 4] = 0x0abc_0380;
        e.put_vtable(PROCESS_VTABLE, &slots);
        quiet(e, &[0x0abc_0370, 0x0abc_0374, 0x0abc_0380]);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, PROCESS_VTABLE);
        object
    }

    /// A map whose virtual +8, +0x10 and +0x14 are recorded as 0x0abe_0008,
    /// 0x0abe_0010 and 0x0abe_0014 (+8 finds nothing).
    fn map(e: &mut Engine) -> u32 {
        e.put_vtable(
            0x0abf_0000,
            &[0, 0, 0x0abe_0008, 0, 0x0abe_0010, 0x0abe_0014],
        );
        quiet(e, &[0x0abe_0008, 0x0abe_0010, 0x0abe_0014]);
        let map = e.mem.alloc(0x40);
        e.mem.set_u32(map, 0x0abf_0000);
        map
    }

    #[test]
    fn lookup_forwards_to_the_bucket_object() {
        let mut e = Engine::new();
        e.register(0x0044_9f80, |_, a| (a[0] + 0x100).into_ret());
        e.register(0x0044_c8e0, |_, a| u32::from(a[1] == 7).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_bb90, &args![0x1000u32, 7u32, 9u32]).bool());
        assert!(!e.call(0x0044_bb90, &args![0x1000u32, 8u32, 9u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0044_c8e0)[0], vec![0x1100, 7, 9]);
    }

    #[test]
    fn task_constructor_fills_its_fields() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x00c3_ce60, 0x00c3_cee0, 0x00c3_cf60]);
        let task = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        let back = e.call(0x0090_5ba0, &args![task, 0x11u32, 0x22u32, 0x33u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(back.u32(), task);
        assert_eq!(e.mem.u32(task), LIP_TASK_VTABLE);
        assert_eq!(e.mem.u32(task + 0x30), 0x22);
        assert_eq!(e.mem.u32(task + 0x34), 0);
        assert_eq!(e.mem.u32(task + 0x38), 0x11);
        assert_eq!(calls(&log, 0x00c3_ce60), vec![vec![task, 0]]);
        assert_eq!(calls(&log, 0x00c3_cee0), vec![vec![task, 0x33]]);
        assert_eq!(calls(&log, 0x00c3_cf60), vec![vec![task, 4]]);
    }

    fn run_with_file(file: u32) -> (Engine, u32, Logged) {
        let mut e = Engine::new();
        let object = process(&mut e);
        quiet(&mut e, &[SCOPE_GUARD_BEGIN, SCOPE_GUARD_END]);
        e.register_double(0x00c3_cff0, move |_, _| file.into_ret());
        e.register_double(OBJECT_WORD_68, move |_, _| object.into_ret());
        e.register(0x004d_5350, |_, a| (a[0] | 0x5000).into_ret());
        let task = e.mem.alloc(0x40);
        e.mem.set_u32(task + 0x30, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x0090_5c50, &args![task]);
        let log = e.call_log.take().unwrap();
        (e, task, log)
    }

    #[test]
    fn run_loads_the_file_and_flags_a_missing_one() {
        let (e, task, log) = run_with_file(0x40);
        assert_eq!(e.mem.u32(task + 0x34), 0x5040);
        assert_eq!(calls(&log, 0x00c3_cff0), vec![vec![task, 4, 0x10]]);
        assert!(calls(&log, 0x0abc_0370).is_empty());
        let guard = calls(&log, SCOPE_GUARD_BEGIN);
        assert_eq!(guard[0][1..], [0x35, 1, LIP_SYNC_SOURCE_FILE, 0x15]);
        assert_eq!(calls(&log, SCOPE_GUARD_END), vec![vec![guard[0][0]]]);

        let (e, task, log) = run_with_file(0);
        assert_eq!(e.mem.u32(task + 0x34), 0x5000);
        assert_eq!(calls(&log, OBJECT_WORD_68), vec![vec![0x77], vec![0x77]]);
        assert_eq!(calls(&log, 0x0abc_0370)[0][1], 0);
        assert_eq!(calls(&log, 0x0abc_0374)[0][1], 1);
        assert_eq!(calls(&log, 0x004d_5350), vec![vec![0]]);
    }

    fn finish(object: Option<u32>, anim: u32) -> (u32, Logged) {
        let mut e = Engine::new();
        let real = process(&mut e);
        let object = object.unwrap_or(real);
        e.register_double(OBJECT_WORD_68, move |_, _| object.into_ret());
        let manager = map(&mut e);
        let task = e.mem.alloc(0x40);
        e.mem.set_u32(task + 0x30, 0x77);
        e.mem.set_u32(task + 0x34, anim);
        e.mem.set_u32(task + 0x38, manager);
        e.call_log = Some(vec![]);
        e.call(0x0090_5d20, &args![task]);
        (manager, e.call_log.take().unwrap())
    }

    #[test]
    fn finish_hands_the_animation_over_and_drops_the_entry() {
        let (manager, log) = finish(None, 0x99);
        assert_eq!(calls(&log, 0x0abc_0380)[0][1], 0x99);
        assert_eq!(calls(&log, 0x0abc_0370)[0][1], 0);
        assert!(calls(&log, 0x0abc_0374).is_empty());
        assert_eq!(calls(&log, 0x0abe_0014), vec![vec![manager, 0x77]]);

        let (_, log) = finish(None, 0);
        assert_eq!(calls(&log, 0x0abc_0374)[0][1], 1);

        let (manager, log) = finish(Some(0), 0);
        assert!(calls(&log, 0x0abc_0380).is_empty());
        assert_eq!(calls(&log, OBJECT_WORD_68).len(), 1);
        assert_eq!(calls(&log, 0x0abe_0014), vec![vec![manager, 0x77]]);
    }

    #[test]
    fn cancel_drops_the_entry_unless_the_state_is_five() {
        for (state, drops) in [(4u32, true), (5, false), (6, true)] {
            let mut e = Engine::new();
            quiet(&mut e, &[0x00c3_cb70]);
            let manager = map(&mut e);
            let task = e.mem.alloc(0x40);
            e.mem.set_u32(task + 0x30, 0x77);
            e.mem.set_u32(task + 0x38, manager);
            e.call_log = Some(vec![]);
            e.call(0x0090_5dc0, &args![task, state, 0x12u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, 0x00c3_cb70), vec![vec![task, state, 0x12]]);
            assert_eq!(calls(&log, 0x0abe_0014).len(), usize::from(drops));
        }
    }

    #[test]
    fn description_answers_the_queued_entry_s_byte() {
        let mut e = Engine::new();
        e.register(0x00c3_d0e0, |_, _| 1u32.into_ret());
        assert!(e
            .call(0x0090_5e00, &args![0x1000u32, 0x2000u32, 0x40u32])
            .bool());
        e.register(0x00c3_d0e0, |_, _| 0u32.into_ret());
        assert!(!e
            .call(0x0090_5e00, &args![0x1000u32, 0x2000u32, 0x40u32])
            .bool());
    }

    #[test]
    fn description_passes_the_task_text() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x00c3_d0e0]);
        e.call_log = Some(vec![]);
        e.call(0x0090_5e00, &args![0x1000u32, 0x2000u32, 0x40u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, 0x00c3_d0e0),
            vec![vec![0x1000, 0x2000, 0x40, 0x0108_8634]]
        );
    }

    fn queue(allocates: bool) -> (u32, Logged) {
        let mut e = Engine::new();
        let object = process(&mut e);
        e.register_double(OBJECT_WORD_68, move |_, _| object.into_ret());
        let manager = map(&mut e);
        quiet(
            &mut e,
            &[
                SCOPE_GUARD_BEGIN,
                SCOPE_GUARD_END,
                NI_POINTER_CONSTRUCT,
                NI_POINTER_DESTRUCT,
                0x00c3_ce60,
                0x00c3_cee0,
                0x00c3_cf60,
                0x0abe_0048,
            ],
        );
        let mut slots = vec![0u32; 0x48 / 4];
        slots.push(0x0abe_0048);
        e.put_vtable(0x0abf_1000, &slots);
        let task_manager = e.mem.alloc(4);
        e.mem.set_u32(task_manager, 0x0abf_1000);
        e.map(TASK_MANAGER_POINTER, 4);
        e.mem.set_u32(TASK_MANAGER_POINTER, task_manager);
        e.register_double(OPERATOR_NEW, move |e, _| {
            if allocates {
                e.mem.alloc(0x40).into_ret()
            } else {
                Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x0090_5f30, &args![manager, 0x77u32, 0x88u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![0x40]]);
        assert_eq!(calls(&log, 0x0abe_0008)[0][..2], [manager, 0x77]);
        // The first `NiPointer` is `CancelLipFile`'s, the second holds the task.
        let pointer = calls(&log, NI_POINTER_CONSTRUCT)[1].clone();
        let added = calls(&log, 0x0abe_0010);
        assert_eq!(added, vec![vec![manager, 0x77, pointer[0], 0]]);
        assert_eq!(calls(&log, NI_POINTER_DESTRUCT)[1], vec![pointer[0]]);
        assert_eq!(calls(&log, 0x0abc_0374)[0][1], 0);
        let guard = calls(&log, SCOPE_GUARD_BEGIN);
        assert_eq!(guard[0][1..], [0x35, 1, LIP_SYNC_SOURCE_FILE, 0x61]);
        let handed = calls(&log, 0x0abe_0048);
        assert_eq!(handed, vec![vec![task_manager, pointer[1]]]);
        (pointer[1], log)
    }

    #[test]
    fn queueing_builds_a_task_and_hands_it_to_the_manager() {
        let (task, log) = queue(true);
        assert_ne!(task, 0);
        assert_eq!(calls(&log, 0x00c3_ce60), vec![vec![task, 0]]);
        assert_eq!(calls(&log, 0x00c3_cee0), vec![vec![task, 0x88]]);
        let (task, log) = queue(false);
        assert_eq!(task, 0);
        assert!(calls(&log, 0x00c3_ce60).is_empty());
    }

    #[test]
    fn map_removal_goes_through_virtual_0x14() {
        let mut e = Engine::new();
        let manager = map(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0090_60f0, &args![manager, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0abe_0014), vec![vec![manager, 0x77]]);
    }

    #[test]
    fn iterator_deleting_destructor_frees_on_bit_zero() {
        for flags in [0u32, 1, 2, 3] {
            let mut e = Engine::new();
            quiet(&mut e, &[OPERATOR_DELETE]);
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            let back = e.call(0x0090_6130, &args![this, flags]).u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(back, this);
            assert_eq!(e.mem.u32(this), LIP_TASK_MAP_ITERATOR_VTABLE);
            assert_eq!(calls(&log, OPERATOR_DELETE).len(), (flags & 1) as usize);
        }
    }
}
