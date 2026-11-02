//! `BSMain/queuedfiles.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c3c520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `010c14e8` (RTTI `.?AVQueuedChildren@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00c3d930`. One word, plain `RET`, returns nothing.
pub fn fn_00c3c520(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010c_14e8);
    e.call(0x00c3_d930, &args![this]);
}

// Translated from 00c3c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `010c1510` (RTTI `.?AVQueuedParents@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00c3d930`. One word, plain `RET`, returns nothing.
pub fn fn_00c3c570(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010c_1510);
    e.call(0x00c3_d930, &args![this]);
}

/// The vtables (RTTI): `QueuedFile`, `QueuedFileEntry`, `BSTask<__int64>`,
/// `BSSimpleArray<NiPointer<QueuedFile>,1024>`, `QueuedChildren`, `QueuedParents`.
const QUEUED_FILE_VTABLE: u32 = 0x010c_1524;
const QUEUED_FILE_ENTRY_VTABLE: u32 = 0x010c_1554;
const BS_TASK_VTABLE: u32 = 0x0101_7138;
const SIMPLE_ARRAY_VTABLE: u32 = 0x010c_14fc;
const QUEUED_CHILDREN_VTABLE: u32 = 0x010c_14e8;
const QUEUED_PARENTS_VTABLE: u32 = 0x010c_1510;
/// The memory manager singleton, its `Allocate` and `Deallocate` (`thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The imported `InterlockedIncrement`, `InterlockedDecrement` and
/// `InterlockedCompareExchange` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
/// `00440540` (`thiscall`, one word; unit `fallout shared/modelloader.cpp`): the `IOTask`
/// base constructor, by its use here.
const IO_TASK_CONSTRUCT: u32 = 0x0044_0540;
/// `IOTask::GenerateKey` (engine map, `0043db70`; `thiscall`).
const IO_TASK_GENERATE_KEY: u32 = 0x0043_db70;
/// `0044dd60` (`thiscall` on the object, no stack words): the `NiPointer` release of a
/// `QueuedFile` (by its use: each pointer given up is passed to it).
const POINTER_RELEASE: u32 = 0x0044_dd60;
/// The counter `~QueuedFile` decrements (`.data`; by the PDB's `IOTask::iStaticCounter`,
/// unconfirmed) and the counter `GenerateKey` increments.
const TASK_COUNTER: u32 = 0x0120_2d94;
const KEY_COUNTER: u32 = 0x0120_2d9c;
/// The `IOManager` pointer (`.data`).
const IO_MANAGER: u32 = 0x0120_2d98;
/// `IOManager::RequeueTask` (engine map, `00c3df40`; `thiscall`, two words) and `0044ac40`
/// (`thiscall`, two words), called on the manager.
const IO_MANAGER_REQUEUE_TASK: u32 = 0x00c3_df40;
const IO_MANAGER_0044AC40: u32 = 0x0044_ac40;
/// `00c3d5e0` (`thiscall`, two words): removes the element at an index (by its use here).
const SIMPLE_ARRAY_REMOVE_AT: u32 = 0x00c3_d5e0;
/// `0042fb60` (`thiscall`, three words: (destination, source, count)): the element copy of a
/// reallocation.
const SIMPLE_ARRAY_COPY: u32 = 0x0042_fb60;
/// The TLS word `QueuedFile::QueuedFile` reads (`*(FS:[0x2c])[_tls_index] + 0x2b4`).
const TLS_MEMORY_CONTEXT: u32 = 0x2b4;
/// CRT `_strlen` and `_strcpy_s` (`cdecl`).
const STRLEN: u32 = 0x00ec_6130;
const STRCPY_S: u32 = 0x00ec_65a6;
/// `BSSystem/archive.cpp` and `filefinder.cpp` callees (`cdecl`): `00af4250` (name,
/// buffer, size), `00afd270` (path, &out, &out), `00af6540` (archive, &a, &b, path),
/// `00af6340` (archive, entry, -1, 0) and `FileFinder::GetFile_ov2` (`00afdf20`).
const ARCHIVE_PATH: u32 = 0x00af_4250;
const SPLIT_PATH: u32 = 0x00af_d270;
const ARCHIVE_FIND_ENTRY: u32 = 0x00af_6540;
const ARCHIVE_GET_FILE: u32 = 0x00af_6340;
const FILE_FINDER_GET_FILE: u32 = 0x00af_df20;
/// `__allshr` and `__allshl` (`00ec77e0`, `00ec7810`; a 64-bit value and a count).
const ALLSHR: u32 = 0x00ec_77e0;
const ALLSHL: u32 = 0x00ec_7810;

/// `QueuedFile` (Xbox PDB) members, PC offsets as on the Xbox.
const REF_COUNT: u32 = 0x8;
const STATE: u32 = 0xc;
const KEY: u32 = 0x10;
const CONTEXT: u32 = 0x18;
const PARENT: u32 = 0x1c;
const CHILDREN: u32 = 0x20;
const ADDITIONAL_PARENTS: u32 = 0x24;
/// `QueuedFileEntry::pFileName` and `pFileEntry` (Xbox PDB).
const FILE_NAME: u32 = 0x28;
const FILE_ENTRY: u32 = 0x2c;
/// `BSSimpleArray::pBuffer`, `iSize`, `iReservedSize` and `QueuedChildren::iNumChildrenFinished`.
const BUFFER: u32 = 0x4;
const SIZE: u32 = 0x8;
const RESERVED: u32 = 0xc;
const CHILDREN_FINISHED: u32 = 0x10;

/// The inlined `NiPointer<QueuedFile>` assignment of null to the pointer at `slot`, as the
/// exe writes it (only when it is set: release the old object, store null, then the dead
/// increment of the new, null, object).
fn clear_pointer(e: &mut Engine, slot: u32) {
    if e.mem.u32(slot) == 0 {
        return;
    }
    let old = e.mem.u32(slot);
    if old != 0 {
        e.call(POINTER_RELEASE, &args![old]);
    }
    e.mem.set_u32(slot, 0);
    let new = e.mem.u32(slot);
    if new != 0 {
        // Never taken: the pointer was just set to null.
        e.call(INTERLOCKED_INCREMENT, &args![new + REF_COUNT]);
    }
}

/// The inlined placement construction of a null `NiPointer` at `slot` (the null check of
/// the address and the dead increment included).
fn construct_null_pointer(e: &mut Engine, slot: u32) {
    if slot == 0 {
        return;
    }
    e.mem.set_u32(slot, 0);
    let object = e.mem.u32(slot);
    if object != 0 {
        // Never taken: the pointer was just set to null.
        e.call(INTERLOCKED_INCREMENT, &args![object + REF_COUNT]);
    }
}

/// The inlined deleting destructor call through vtable slot +0 with the flags 1, when the
/// object is not null.
fn delete_object(e: &mut Engine, object: u32) {
    if object != 0 {
        e.vcall(object, 0x0, &args![1u32]);
    }
}

// Translated from 00c3c590 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `QueuedFile::QueuedFile` (Xbox PDB; `thiscall`, one word, `RET 4`): the `IOTask` base
/// (`00440540` with the word), the `QueuedFile` vtable, a null `spParent` (the inlined
/// `NiPointer` constructor), no children or additional parents, and `eContext` from the
/// thread's memory context (TLS +0x2B4). Answers `this`.
pub fn queued_file_queued_file(e: &mut Engine, this: u32, task_argument: u32) -> u32 {
    e.call(IO_TASK_CONSTRUCT, &args![this, task_argument]);
    e.mem.set_u32(this, QUEUED_FILE_VTABLE);
    construct_null_pointer(e, this + PARENT);
    e.mem.set_u32(this + CHILDREN, 0);
    e.mem.set_u32(this + ADDITIONAL_PARENTS, 0);
    let context = e.tls() + TLS_MEMORY_CONTEXT;
    let context = e.mem.u32(context);
    e.mem.set_u32(this + CONTEXT, context);
    this
}

// Translated from 00c3c620 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; by its vtables `~QueuedFile`):
/// deletes the children and additional parents arrays (vtable slot +0 with 1, when set),
/// releases `spParent` (`0044dd60`), stores the `BSTask<__int64>` vtable and decrements the
/// counter at `01202d94`.
// Translation incomplete: 00c3c620 - the FS:[0] exception frame (running the base destructor on unwinding) is not translated
pub fn fn_00c3c620(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, QUEUED_FILE_VTABLE);
    let children = e.mem.u32(this + CHILDREN);
    delete_object(e, children);
    let parents = e.mem.u32(this + ADDITIONAL_PARENTS);
    delete_object(e, parents);
    let parent = e.mem.u32(this + PARENT);
    if parent != 0 {
        e.call(POINTER_RELEASE, &args![parent]);
    }
    e.mem.set_u32(this, BS_TASK_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![TASK_COUNTER]);
}

// Translated from 00c3c4e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; by its vtables the
/// `QueuedChildren` constructor): the `BSSimpleArray` vtable, `fn_00c3d780(5, 0)`, the
/// `QueuedChildren` vtable and no finished children. Answers `this`.
pub fn fn_00c3c4e0(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, SIMPLE_ARRAY_VTABLE);
    fn_00c3d780(e, this, 5, 0);
    e.mem.set_u32(this, QUEUED_CHILDREN_VTABLE);
    e.mem.set_u32(this + CHILDREN_FINISHED, 0);
    this
}

// Translated from 00c3c540 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; by its vtables the
/// `QueuedParents` constructor): the `BSSimpleArray` vtable, `fn_00c3d780(5, 0)` and the
/// `QueuedParents` vtable. Answers `this`.
pub fn fn_00c3c540(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, SIMPLE_ARRAY_VTABLE);
    fn_00c3d780(e, this, 5, 0);
    e.mem.set_u32(this, QUEUED_PARENTS_VTABLE);
    this
}

/// Adds `file` to the array at `array` through a counted local pointer, as `fn_00c3c700`
/// and `fn_00c3c7e0` do (the local counted up, `fn_00c3d4f0`, the local released).
fn add_counted(e: &mut Engine, array: u32, file: u32) {
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, file);
        if file != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![file + REF_COUNT]);
        }
        fn_00c3d4f0(e, array, local);
        let held = e.mem.u32(local);
        if held != 0 {
            e.call(POINTER_RELEASE, &args![held]);
        }
    });
}

// Translated from 00c3c700 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; adds a child): makes the
/// `QueuedChildren` array (0x14 bytes from the memory manager, `fn_00c3c4e0`; 0 when the
/// allocation fails) when there is none, then adds `child` to it.
// Translation incomplete: 00c3c700 - the FS:[0] exception frame (freeing the array or releasing the local pointer on unwinding) is not translated
pub fn fn_00c3c700(e: &mut Engine, this: u32, child: u32) {
    if e.mem.u32(this + CHILDREN) == 0 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
            .u32();
        let array = if block != 0 { fn_00c3c4e0(e, block) } else { 0 };
        e.mem.set_u32(this + CHILDREN, array);
    }
    let array = e.mem.u32(this + CHILDREN);
    add_counted(e, array, child);
}

// Translated from 00c3c7e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; adds an additional parent):
/// makes the `QueuedParents` array (0x10 bytes, `fn_00c3c540`) when there is none; otherwise
/// nothing happens when `parent` is already in it. Adds `parent`, then makes `this` a child
/// of it (`fn_00c3c700` on `parent`).
// Translation incomplete: 00c3c7e0 - the FS:[0] exception frame (freeing the array or releasing the local pointer on unwinding) is not translated
pub fn fn_00c3c7e0(e: &mut Engine, this: u32, parent: u32) {
    if e.mem.u32(this + ADDITIONAL_PARENTS) == 0 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
            .u32();
        let array = if block != 0 { fn_00c3c540(e, block) } else { 0 };
        e.mem.set_u32(this + ADDITIONAL_PARENTS, array);
    } else {
        let array = e.mem.u32(this + ADDITIONAL_PARENTS);
        let mut index = 0u32;
        let mut found = u32::MAX;
        while index < e.mem.u32(array + SIZE) {
            let buffer = e.mem.u32(array + BUFFER);
            if buffer != 0 && e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4))) == parent {
                found = index;
                break;
            }
            index += 1;
        }
        if found != u32::MAX {
            return;
        }
    }
    let array = e.mem.u32(this + ADDITIONAL_PARENTS);
    add_counted(e, array, parent);
    fn_00c3c700(e, parent, this);
}

// Translated from 00c3c930 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `QueuedFile::CheckFinished` (Xbox PDB; `thiscall`, no stack words): only for a file with
/// a parent or additional parents, whose `eState` is at least 4 (signed) and whose children
/// (when it has an array) have all finished (`iSize == iNumChildrenFinished`): holds itself
/// (count up), notifies the parent (`NotifyChildFinished`, vtable slot +0x24, with `this`)
/// and clears `spParent`, notifies each additional parent the same way and deletes that
/// array, then releases itself (`0044dd60`).
pub fn queued_file_check_finished(e: &mut Engine, this: u32) {
    if e.mem.u32(this + PARENT) == 0 && e.mem.u32(this + ADDITIONAL_PARENTS) == 0 {
        return;
    }
    if e.mem.i32(this + STATE) < 4 {
        return;
    }
    let children = e.mem.u32(this + CHILDREN);
    let all_finished = if children != 0 {
        e.mem.u32(children + SIZE) == e.mem.u32(children + CHILDREN_FINISHED)
    } else {
        true
    };
    if !all_finished {
        return;
    }
    e.call(INTERLOCKED_INCREMENT, &args![this + REF_COUNT]);
    if e.mem.u32(this + PARENT) != 0 {
        let parent = e.mem.u32(this + PARENT);
        e.vcall(parent, 0x24, &args![this]);
        clear_pointer(e, this + PARENT);
    }
    if e.mem.u32(this + ADDITIONAL_PARENTS) != 0 {
        let mut index = 0u32;
        loop {
            let array = e.mem.u32(this + ADDITIONAL_PARENTS);
            if index >= e.mem.u32(array + SIZE) {
                break;
            }
            let buffer = e.mem.u32(array + BUFFER);
            let parent = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
            e.vcall(parent, 0x24, &args![this]);
            index += 1;
        }
        let array = e.mem.u32(this + ADDITIONAL_PARENTS);
        delete_object(e, array);
        e.mem.set_u32(this + ADDITIONAL_PARENTS, 0);
    }
    e.call(POINTER_RELEASE, &args![this]);
}

// Translated from 00c3cae0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `QueuedFile::Requeue` (Xbox PDB; `thiscall`, one word, `RET 4`): requeues each child
/// (vtable slot +0x1C with `priority`), then itself through `IOManager::RequeueTask(this,
/// priority)` on the manager at `01202d98`.
pub fn queued_file_requeue(e: &mut Engine, this: u32, priority: u32) {
    let children = e.mem.u32(this + CHILDREN);
    if children != 0 {
        let count = e.mem.u32(children + SIZE);
        let mut index = 0u32;
        while index < count {
            let array = e.mem.u32(this + CHILDREN);
            let buffer = e.mem.u32(array + BUFFER);
            let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
            if child != 0 {
                e.vcall(child, 0x1c, &args![priority]);
            }
            index += 1;
        }
    }
    let manager = e.mem.u32(IO_MANAGER);
    e.call(IO_MANAGER_REQUEUE_TASK, &args![manager, this, priority]);
}

// Translated from 00c3cb70 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `QueuedFile::Cancel` (Xbox PDB; `thiscall`, two words, `RET 8`): holds itself, then
/// with a `parent` given: clears `spParent` when it is that parent, or else removes it from
/// the additional parents (`00c3d5e0(index, 1)` at its first match) and deletes that array
/// once empty; with no parent given: clears `spParent` and deletes the additional parents.
/// While the file still has a parent: state 1 hands it to the manager's vtable slot +0x48,
/// any other state is stored with `InterlockedCompareExchange(&eState, state, eState)`.
/// Otherwise each child is handed to the manager's `0044ac40(child, this)`. Then releases
/// itself (`0044dd60`).
pub fn queued_file_cancel(e: &mut Engine, this: u32, state: u32, parent: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![this + REF_COUNT]);
    if parent != 0 {
        if e.mem.u32(this + PARENT) == parent {
            clear_pointer(e, this + PARENT);
        } else if e.mem.u32(this + ADDITIONAL_PARENTS) != 0 {
            let mut index = 0u32;
            loop {
                let array = e.mem.u32(this + ADDITIONAL_PARENTS);
                if index >= e.mem.u32(array + SIZE) {
                    break;
                }
                let buffer = e.mem.u32(array + BUFFER);
                if e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4))) == parent {
                    e.call(SIMPLE_ARRAY_REMOVE_AT, &args![array, index, 1u32]);
                    break;
                }
                index += 1;
            }
            let array = e.mem.u32(this + ADDITIONAL_PARENTS);
            if e.mem.u32(array + SIZE) == 0 {
                delete_object(e, array);
                e.mem.set_u32(this + ADDITIONAL_PARENTS, 0);
            }
        }
    } else {
        clear_pointer(e, this + PARENT);
        let array = e.mem.u32(this + ADDITIONAL_PARENTS);
        delete_object(e, array);
        e.mem.set_u32(this + ADDITIONAL_PARENTS, 0);
    }
    if e.mem.u32(this + PARENT) != 0 || e.mem.u32(this + ADDITIONAL_PARENTS) != 0 {
        if state == 1 {
            let manager = e.mem.u32(IO_MANAGER);
            e.vcall(manager, 0x48, &args![this]);
        } else {
            let current = e.mem.u32(this + STATE);
            // The answer (whether the exchange happened) is kept in a local nobody reads.
            e.call(
                INTERLOCKED_COMPARE_EXCHANGE,
                &args![this + STATE, state, current],
            );
        }
    } else {
        let children = e.mem.u32(this + CHILDREN);
        if children != 0 {
            let count = e.mem.u32(children + SIZE);
            let mut index = 0u32;
            while index < count {
                let array = e.mem.u32(this + CHILDREN);
                let buffer = e.mem.u32(array + BUFFER);
                let child = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
                if child != 0 {
                    let manager = e.mem.u32(IO_MANAGER);
                    e.call(IO_MANAGER_0044AC40, &args![manager, child, this]);
                }
                index += 1;
            }
        }
    }
    e.call(POINTER_RELEASE, &args![this]);
}

// Translated from 00c3ce60 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; by its vtable the
/// `QueuedFileEntry` constructor): `QueuedFile::QueuedFile(task_argument)`, the
/// `QueuedFileEntry` vtable, no file name and no file entry. Answers `this`.
pub fn fn_00c3ce60(e: &mut Engine, this: u32, task_argument: u32) -> u32 {
    queued_file_queued_file(e, this, task_argument);
    e.mem.set_u32(this, QUEUED_FILE_ENTRY_VTABLE);
    e.mem.set_u32(this + FILE_NAME, 0);
    e.mem.set_u32(this + FILE_ENTRY, 0);
    this
}

// Translated from 00c3cea0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; by its vtable
/// `~QueuedFileEntry`): the `QueuedFileEntry` vtable, the file name freed to the memory
/// manager (also when null), then `fn_00c3c620`.
pub fn fn_00c3cea0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, QUEUED_FILE_ENTRY_VTABLE);
    let name = e.mem.u32(this + FILE_NAME);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, name]);
    fn_00c3c620(e, this);
}

// Translated from 00c3cee0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; sets the file name): for a
/// name that is not null, a copy (`_strlen` + 1 bytes from the memory manager, `_strcpy_s`)
/// becomes `pFileName`.
pub fn fn_00c3cee0(e: &mut Engine, this: u32, name: u32) {
    if name == 0 {
        return;
    }
    let size = e.call(STRLEN, &args![name]).u32().wrapping_add(1);
    let copy = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.mem.set_u32(this + FILE_NAME, copy);
    let copy = e.mem.u32(this + FILE_NAME);
    e.call(STRCPY_S, &args![copy, size, name]);
}

// Translated from 00c3cf40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): stores `pFileEntry`.
pub fn fn_00c3cf40(e: &mut Engine, this: u32, entry: u32) {
    e.mem.set_u32(this + FILE_ENTRY, entry);
}

// Translated from 00c3cf60 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): with a file name, builds the
/// path for `archive` (`fn_00c3d090`, a 0x104-byte buffer on the stack), splits it
/// (`00afd270(path, &a, &b)`) and stores what `00af6540(archive, &a, &b, path)` answers as
/// `pFileEntry`.
// Translation incomplete: 00c3cf60 - the /GS cookie check 00ec408c is not translated
pub fn fn_00c3cf60(e: &mut Engine, this: u32, archive: u32) {
    if e.mem.u32(this + FILE_NAME) == 0 {
        return;
    }
    e.with_stack(0x120, |e, frame| {
        let path = frame.addr();
        let first = path + 0x10c;
        let second = path + 0x118;
        fn_00c3d090(e, this, archive, path);
        e.call(SPLIT_PATH, &args![path, first, second]);
        let entry = e
            .call(ARCHIVE_FIND_ENTRY, &args![archive, first, second, path])
            .u32();
        fn_00c3cf40(e, this, entry);
    });
}

// Translated from 00c3cff0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `QueuedFileEntry::GetFile` (Xbox PDB; `thiscall`, two words, `RET 8`): with a file entry,
/// `00af6340(archive, pFileEntry, -1, 0)`; when that gave nothing and there is a file
/// name, the path (`fn_00c3d090`) through `FileFinder::GetFile_ov2(path, 0, -1, mode)`.
/// Answers the file (0 when neither applies).
// Translation incomplete: 00c3cff0 - the /GS cookie check 00ec408c is not translated
pub fn queued_file_entry_get_file(e: &mut Engine, this: u32, archive: u32, mode: u32) -> u32 {
    let mut file = 0u32;
    let entry = e.mem.u32(this + FILE_ENTRY);
    if entry != 0 {
        file = e
            .call(ARCHIVE_GET_FILE, &args![archive, entry, u32::MAX, 0u32])
            .u32();
    }
    if e.mem.u32(this + FILE_NAME) != 0 && file == 0 {
        file = e.with_stack(0x104, |e, buffer| {
            let path = buffer.addr();
            fn_00c3d090(e, this, archive, path);
            e.call(FILE_FINDER_GET_FILE, &args![path, 0u32, u32::MAX, mode])
                .u32()
        });
    }
    file
}

// Translated from 00c3d090 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words, `RET 8`): the path of the file name
/// into the 0x104-byte `buffer`: without an archive through `00af4250(name, buffer,
/// 0x104)`, with one a plain `_strcpy_s(buffer, 0x104, name)`.
pub fn fn_00c3d090(e: &mut Engine, this: u32, archive: u32, buffer: u32) {
    let name = e.mem.u32(this + FILE_NAME);
    if archive == 0 {
        e.call(ARCHIVE_PATH, &args![name, buffer, 0x104u32]);
    } else {
        e.call(STRCPY_S, &args![buffer, 0x104u32, name]);
    }
}

// Translated from 00c3d380 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `QueuedChildren` scalar deleting destructor (Xbox PDB; `thiscall`, one word, `RET 4`):
/// `fn_00c3c520`, then frees `this` to the memory manager when bit 0 of `flags` is set.
/// Answers `this`.
pub fn queued_children_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_00c3c520(e, Ptr::new(this));
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c3d3b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `QueuedParents` scalar deleting destructor (Xbox PDB; `thiscall`, one word, `RET 4`):
/// `fn_00c3c570`, then frees `this` when bit 0 of `flags` is set. Answers `this`.
pub fn queued_parents_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_00c3c570(e, Ptr::new(this));
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c3d3e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `QueuedFile` scalar deleting destructor (Xbox PDB; `thiscall`, one word, `RET 4`):
/// `fn_00c3c620`, then frees `this` when bit 0 of `flags` is set. Answers `this`.
pub fn queued_file_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_00c3c620(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c3d440 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `QueuedFileEntry::GenerateKey` (Xbox PDB; `thiscall`, no stack words): with a file entry
/// the 64-bit `Key` becomes `GetFileIndex` (vtable slot +0x2C, sign-extended) << 56 | (the
/// entry's word +0xC & 0x7FFFFFFF) << 24 | (byte 0 of `Key >> 16`, `__allshr`) << 16 | the
/// low 16 bits of the incremented counter at `01202d9c`; without one, `IOTask::GenerateKey`.
pub fn queued_file_entry_generate_key(e: &mut Engine, this: u32) {
    let entry = e.mem.u32(this + FILE_ENTRY);
    if entry == 0 {
        e.call(IO_TASK_GENERATE_KEY, &args![this]);
        return;
    }
    let counter = e.call(INTERLOCKED_INCREMENT, &args![KEY_COUNTER]).u16();
    let key = e.mem.u64(this + KEY);
    let shifted = e.call(ALLSHR, &args![key, 0x10u8]).u8();
    let entry = e.mem.u32(this + FILE_ENTRY);
    let index = e.mem.u32(entry + 0xc) & 0x7fff_ffff;
    let file_index = e.vcall(this, 0x2c, &[]).u8() as i8;
    let high = e
        .call(ALLSHL, &args![i64::from(file_index) as u64, 0x38u8])
        .u64();
    let middle = e.call(ALLSHL, &args![u64::from(index), 0x18u8]).u64();
    let low = e.call(ALLSHL, &args![u64::from(shifted), 0x10u8]).u64();
    let key = high | middle | low | u64::from(counter);
    e.mem.set_u32(this + KEY, key as u32);
    e.mem.set_u32(this + KEY + 4, (key >> 32) as u32);
}

// Translated from 00c3d4f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; `BSSimpleArray<NiPointer<
/// QueuedFile>>` add): a new slot (`fn_00c3d6c0`), constructed as a null pointer, then
/// assigned the pointer `*file` holds (the old one released, the new one counted up when
/// they differ). Answers the index.
pub fn fn_00c3d4f0(e: &mut Engine, this: u32, file: u32) -> u32 {
    let index = fn_00c3d6c0(e, this);
    let slot = e.mem.u32(this + BUFFER).wrapping_add(index.wrapping_mul(4));
    construct_null_pointer(e, slot);
    let slot = e.mem.u32(this + BUFFER).wrapping_add(index.wrapping_mul(4));
    let wanted = e.mem.u32(file);
    if e.mem.u32(slot) != wanted {
        let old = e.mem.u32(slot);
        if old != 0 {
            e.call(POINTER_RELEASE, &args![old]);
        }
        let wanted = e.mem.u32(file);
        e.mem.set_u32(slot, wanted);
        let new = e.mem.u32(slot);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new + REF_COUNT]);
        }
    }
    index
}

// Translated from 00c3d6c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; `BSSimpleArray` add-index): when
/// the array is full, an empty one gets 4 slots (`_Allocate`, vtable slot +4) and a full
/// one grows to `reserved + 0x400` above 0x400 slots or else to twice (`fn_00c3d870`).
/// Then `iSize` grows by one; answers the new last index.
pub fn fn_00c3d6c0(e: &mut Engine, this: u32) -> u32 {
    if e.mem.u32(this + SIZE) == e.mem.u32(this + RESERVED) {
        let reserved = e.mem.u32(this + RESERVED);
        if reserved == 0 {
            let buffer = e.vcall(this, 0x4, &args![4u32]).u32();
            e.mem.set_u32(this + BUFFER, buffer);
            e.mem.set_u32(this + RESERVED, 4);
        } else {
            let grown = if reserved > 0x400 {
                reserved.wrapping_add(0x400)
            } else {
                reserved << 1
            };
            let size = e.mem.u32(this + SIZE);
            fn_00c3d870(e, this, grown, size);
            e.mem.set_u32(this + RESERVED, grown);
        }
    }
    let size = e.mem.u32(this + SIZE).wrapping_add(1);
    e.mem.set_u32(this + SIZE, size);
    e.mem.u32(this + SIZE).wrapping_sub(1)
}

// Translated from 00c3d780 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words, `RET 8`; the `BSSimpleArray`
/// constructor body): empty; reserves `max(reserve, count)` slots (`_Allocate`, vtable slot
/// +4) when that is not zero, and constructs `count` null pointers as the size.
pub fn fn_00c3d780(e: &mut Engine, this: u32, reserve: u32, count: u32) {
    e.mem.set_u32(this + BUFFER, 0);
    e.mem.set_u32(this + SIZE, 0);
    e.mem.set_u32(this + RESERVED, 0);
    let reserve = if count > reserve { count } else { reserve };
    if reserve > 0 {
        let buffer = e.vcall(this, 0x4, &args![reserve]).u32();
        e.mem.set_u32(this + BUFFER, buffer);
        e.mem.set_u32(this + RESERVED, reserve);
    }
    if count > 0 {
        let buffer = e.mem.u32(this + BUFFER);
        let mut index = 0u32;
        while index < count {
            construct_null_pointer(e, buffer.wrapping_add(index.wrapping_mul(4)));
            index += 1;
        }
        e.mem.set_u32(this + SIZE, count);
    }
}

// Translated from 00c3d870 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words, `RET 8`; the `BSSimpleArray` resize):
/// with a buffer, a full one is reallocated in place (`_Reallocate`, vtable slot +0xC,
/// (buffer, capacity)), otherwise a new buffer (`_Allocate`, slot +4) gets the `count`
/// elements (`0042fb60`) and the old one is freed (`_Deallocate`, slot +8). Without a buffer,
/// a new one of `capacity` slots and `iReservedSize = capacity`.
pub fn fn_00c3d870(e: &mut Engine, this: u32, capacity: u32, count: u32) {
    let buffer = e.mem.u32(this + BUFFER);
    if buffer == 0 {
        let buffer = e.vcall(this, 0x4, &args![capacity]).u32();
        e.mem.set_u32(this + BUFFER, buffer);
        e.mem.set_u32(this + RESERVED, capacity);
        return;
    }
    if count == e.mem.u32(this + RESERVED) {
        let buffer = e.mem.u32(this + BUFFER);
        let buffer = e.vcall(this, 0xc, &args![buffer, capacity]).u32();
        e.mem.set_u32(this + BUFFER, buffer);
    } else {
        let fresh = e.vcall(this, 0x4, &args![capacity]).u32();
        let old = e.mem.u32(this + BUFFER);
        e.call(SIMPLE_ARRAY_COPY, &args![this, fresh, old, count]);
        let old = e.mem.u32(this + BUFFER);
        e.vcall(this, 0x8, &args![old]);
        e.mem.set_u32(this + BUFFER, 0);
        e.mem.set_u32(this + BUFFER, fresh);
    }
}

// Translated from 00c3d930 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; the `BSSimpleArray` destructor):
/// the `BSSimpleArray` vtable, then `fn_00c3d980(1)`.
pub fn fn_00c3d930(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, SIMPLE_ARRAY_VTABLE);
    fn_00c3d980(e, this, 1);
}

// Translated from 00c3d980 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; the `BSSimpleArray` clear):
/// with a buffer, releases each of the `iSize` pointers (`0044dd60`); with `free_buffer` set
/// frees the buffer (`_Deallocate`, vtable slot +8) and zeroes the buffer and reserve. The
/// size becomes 0 (only when there was a buffer).
pub fn fn_00c3d980(e: &mut Engine, this: u32, free_buffer: u32) {
    if e.mem.u32(this + BUFFER) == 0 {
        return;
    }
    let count = e.mem.u32(this + SIZE);
    let buffer = e.mem.u32(this + BUFFER);
    let mut index = 0u32;
    while index < count {
        let slot = buffer.wrapping_add(index.wrapping_mul(4));
        let object = e.mem.u32(slot);
        if object != 0 {
            e.call(POINTER_RELEASE, &args![object]);
        }
        // The element destructor's delete (`XOR EAX,EAX; AND EAX,1`) is never taken.
        let delete = 0u32;
        if delete & 1 != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, slot]);
        }
        index += 1;
    }
    if free_buffer as u8 != 0 {
        let buffer = e.mem.u32(this + BUFFER);
        e.vcall(this, 0x8, &args![buffer]);
        e.mem.set_u32(this + BUFFER, 0);
        e.mem.set_u32(this + BUFFER, 0);
        e.mem.set_u32(this + RESERVED, 0);
    }
    e.mem.set_u32(this + SIZE, 0);
}

/// `sprintf_s` (`00406d00`, `cdecl`: buffer, size, format, arguments) and `strcat_s`
/// (`00ec6bd2`, `cdecl`: destination, size, source).
const SPRINTF_S: u32 = 0x0040_6d00;
const STRCAT_S: u32 = 0x00ec_6bd2;
/// The description formats: `"Queued %s %s with file entry offset %i and size %i"`
/// (`010c15cc`), `"Queued %s with file entry offset %i and size %i"` (`010c159c`) and
/// `"Queued %s %s"` (`010c158c`), and the separator before a child's description
/// (`010c1584`, a line break then `" * "`).
const DESCRIPTION_WITH_NAME: u32 = 0x010c_15cc;
const DESCRIPTION_WITHOUT_NAME: u32 = 0x010c_159c;
const DESCRIPTION_NAME_ONLY: u32 = 0x010c_158c;
const CHILD_SEPARATOR: u32 = 0x010c_1584;
/// The size of the stack buffer a child's description is written into.
const CHILD_DESCRIPTION_SIZE: u32 = 0x32c8;
/// The archive entry's words the description prints: the offset (+0xC, low 31 bits) and the
/// size (+8, low 30 bits).
const ENTRY_OFFSET: u32 = 0xc;
const ENTRY_SIZE: u32 = 0x8;

// Translated from 00c3d0e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3d0e0 - the /GS stack cookie check (00ec408c) is not translated
/// `QueuedFileEntry::GetDescription` (engine map; `thiscall`, `RET 0xC`: buffer, size, a
/// text the formats print first). With an archive entry and a file name: `"Queued %s %s with
/// file entry offset %i and size %i"`; with the entry only: `"Queued %s with file entry
/// offset %i and size %i"`; otherwise `"Queued %s %s"` with the name (`sprintf_s`). Then for
/// each non-null child of the children array (+0x20) whose description (virtual `+0x10`,
/// given a 0x32C8-byte stack buffer and its size) answers true, the separator and that
/// description are appended (`strcat_s`). The exe's 0x331C-byte frame goes through the stack
/// probe (`_alloca_probe`, platform). Answers true.
pub fn queued_file_entry_get_description(
    e: &mut Engine,
    this: u32,
    buffer: u32,
    size: u32,
    text: u32,
) -> bool {
    let entry = e.mem.u32(this + FILE_ENTRY);
    let name = e.mem.u32(this + FILE_NAME);
    if entry != 0 && name != 0 {
        let offset = e.mem.u32(entry + ENTRY_OFFSET) & 0x7fff_ffff;
        let length = e.mem.u32(entry + ENTRY_SIZE) & 0x3fff_ffff;
        e.call(
            SPRINTF_S,
            &args![
                buffer,
                size,
                DESCRIPTION_WITH_NAME,
                text,
                name,
                offset,
                length
            ],
        );
    } else if entry != 0 {
        let length = e.mem.u32(entry + ENTRY_SIZE) & 0x3fff_ffff;
        let offset = e.mem.u32(entry + ENTRY_OFFSET) & 0x7fff_ffff;
        e.call(
            SPRINTF_S,
            &args![buffer, size, DESCRIPTION_WITHOUT_NAME, text, offset, length],
        );
    } else {
        e.call(
            SPRINTF_S,
            &args![buffer, size, DESCRIPTION_NAME_ONLY, text, name],
        );
    }
    if e.mem.u32(this + CHILDREN) == 0 {
        return true;
    }
    e.with_stack(CHILD_DESCRIPTION_SIZE, |e, child_text| {
        let mut index = 0u32;
        while index < e.mem.u32(e.mem.u32(this + CHILDREN) + SIZE) {
            let children = e.mem.u32(this + CHILDREN);
            let slot = e
                .mem
                .u32(children + BUFFER)
                .wrapping_add(index.wrapping_mul(4));
            let child = e.mem.u32(slot);
            if child != 0
                && e.vcall(child, 0x10, &args![child_text, CHILD_DESCRIPTION_SIZE])
                    .u8()
                    != 0
            {
                e.call(STRCAT_S, &args![buffer, size, CHILD_SEPARATOR]);
                e.call(STRCAT_S, &args![buffer, size, child_text]);
            }
            index = index.wrapping_add(1);
        }
    });
    true
}

// Translated from 00c3d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFileEntry::_scalar_deleting_destructor_` (engine map; `thiscall`, `RET 4`): the
/// destructor ([`fn_00c3cea0`]) and, when bit 0 of `flags` is set, `this` given back to the
/// memory manager. Answers `this`.
pub fn queued_file_entry_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_00c3cea0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c3d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<QueuedFile>>`'s removal of `count` elements at `index` (unnamed
/// in the engine map; `thiscall`, `RET 8`). Each removed non-null pointer is released
/// (`0044dd60`; the element destructor's delete, `XOR EDX,EDX; AND EDX,1`, is folded away and
/// never runs). Then the last `min(size - index - count, count)` elements are moved into the
/// gap (`0042fb60`: destination, source, count) and the size drops by `count`.
pub fn fn_00c3d5e0(e: &mut Engine, this: u32, index: u32, count: u32) {
    let tail = e
        .mem
        .u32(this + SIZE)
        .wrapping_sub(index)
        .wrapping_sub(count);
    let moved = tail.min(count);
    let gap = e.mem.u32(this + BUFFER).wrapping_add(index.wrapping_mul(4));
    let mut i = 0u32;
    while i < count {
        let slot = gap.wrapping_add(i.wrapping_mul(4));
        let object = e.mem.u32(slot);
        if object != 0 {
            e.call(POINTER_RELEASE, &args![object]);
        }
        i = i.wrapping_add(1);
    }
    let buffer = e.mem.u32(this + BUFFER);
    let source = buffer
        .wrapping_add(e.mem.u32(this + SIZE).wrapping_mul(4))
        .wrapping_sub(moved.wrapping_mul(4));
    let destination = buffer.wrapping_add(index.wrapping_mul(4));
    e.call(SIMPLE_ARRAY_COPY, &args![this, destination, source, moved]);
    let size = e.mem.u32(this + SIZE).wrapping_sub(count);
    e.mem.set_u32(this + SIZE, size);
}

// Translated from 00c3d950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<QueuedFile>,1024>::_scalar_deleting_destructor_` (engine map;
/// `thiscall`, `RET 4`): the destructor ([`fn_00c3d930`]) and, when bit 0 of `flags` is set,
/// `this` given back to the memory manager. Answers `this`.
pub fn bs_simple_array_scalar_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_00c3d930(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c3_d0e0,
            queued_file_entry_get_description(u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00c3_d410,
            queued_file_entry_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(0x00c3_d5e0, fn_00c3d5e0(u32, u32, u32)),
        entry!(
            0x00c3_d950,
            bs_simple_array_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(0x00c3_c570, fn_00c3c570(Ptr)),
        entry!(0x00c3_c520, fn_00c3c520(Ptr)),
        entry!(0x00c3_c590, queued_file_queued_file(u32, u32) -> u32),
        entry!(0x00c3_c620, fn_00c3c620(u32)),
        entry!(0x00c3_c4e0, fn_00c3c4e0(u32) -> u32),
        entry!(0x00c3_c540, fn_00c3c540(u32) -> u32),
        entry!(0x00c3_c700, fn_00c3c700(u32, u32)),
        entry!(0x00c3_c7e0, fn_00c3c7e0(u32, u32)),
        entry!(0x00c3_c930, queued_file_check_finished(u32)),
        entry!(0x00c3_cae0, queued_file_requeue(u32, u32)),
        entry!(0x00c3_cb70, queued_file_cancel(u32, u32, u32)),
        entry!(0x00c3_ce60, fn_00c3ce60(u32, u32) -> u32),
        entry!(0x00c3_cea0, fn_00c3cea0(u32)),
        entry!(0x00c3_cee0, fn_00c3cee0(u32, u32)),
        entry!(0x00c3_cf40, fn_00c3cf40(u32, u32)),
        entry!(0x00c3_cf60, fn_00c3cf60(u32, u32)),
        entry!(0x00c3_cff0, queued_file_entry_get_file(u32, u32, u32) -> u32),
        entry!(0x00c3_d090, fn_00c3d090(u32, u32, u32)),
        entry!(
            0x00c3_d380,
            queued_children_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x00c3_d3b0,
            queued_parents_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(
            0x00c3_d3e0,
            queued_file_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(0x00c3_d440, queued_file_entry_generate_key(u32)),
        entry!(0x00c3_d4f0, fn_00c3d4f0(u32, u32) -> u32),
        entry!(0x00c3_d6c0, fn_00c3d6c0(u32) -> u32),
        entry!(0x00c3_d780, fn_00c3d780(u32, u32, u32)),
        entry!(0x00c3_d870, fn_00c3d870(u32, u32, u32)),
        entry!(0x00c3_d930, fn_00c3d930(u32)),
        entry!(0x00c3_d980, fn_00c3d980(u32, u32)),
    ]
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Doubles for the imports, the release and the memory manager.
    fn doubles(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_COMPARE_EXCHANGE, |e, a| {
            let old = e.mem.u32(a[0]);
            if old == a[2] {
                e.mem.set_u32(a[0], a[1]);
            }
            old.into_ret()
        });
        e.register(POINTER_RELEASE, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        // Every vtable slot of the test objects (and of the real `BSSimpleArray` vtable the
        // constructors store) is a double answering 0 unless a test replaces it.
        let slots: Vec<u32> = (0..0x20).map(|i| 0x0095_0000 + i * 4).collect();
        for slot in &slots {
            e.register(*slot, |_, _| Ret::default());
        }
        e.put_vtable(SIMPLE_ARRAY_VTABLE, &slots[..5]);
        e.put_vtable(QUEUED_CHILDREN_VTABLE, &slots[..5]);
        e.put_vtable(QUEUED_PARENTS_VTABLE, &slots[..5]);
    }

    /// An object whose vtable slots are `0x0095_0000 + slot` (the doubles of `doubles`).
    fn object(e: &mut Engine, size: u32) -> u32 {
        let table = e.mem.alloc(0x80);
        for slot in (0..0x80).step_by(4) {
            e.mem.set_u32(table + slot, 0x0095_0000 + slot);
        }
        let this = e.mem.alloc(size);
        e.mem.set_u32(this, table);
        this
    }

    /// An array of the given pointers (`BSSimpleArray` layout, plus the finished count).
    fn array(e: &mut Engine, items: &[u32]) -> u32 {
        let this = object(e, 0x14);
        let buffer = e.mem.alloc(0x40);
        for (i, item) in (0u32..).zip(items) {
            e.mem.set_u32(buffer + i * 4, *item);
        }
        e.mem.set_u32(this + BUFFER, buffer);
        e.mem.set_u32(this + SIZE, items.len() as u32);
        e.mem.set_u32(this + RESERVED, 0x10);
        this
    }

    #[test]
    fn constructors_and_destructors_set_their_vtables() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(IO_TASK_CONSTRUCT, |_, _| Ret::default());
        let context = e.tls() + TLS_MEMORY_CONTEXT;
        e.mem.set_u32(context, 9);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + PARENT, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_ce60, &[this, 3]).u32(), this);
        assert_eq!(calls_to(&e, IO_TASK_CONSTRUCT), vec![vec![this, 3]]);
        assert_eq!(e.mem.u32(this), QUEUED_FILE_ENTRY_VTABLE);
        assert_eq!(e.mem.u32(this + PARENT), 0);
        assert_eq!(e.mem.u32(this + CONTEXT), 9);
        assert_eq!(e.mem.u32(this + FILE_NAME), 0);

        // The destructor: both arrays deleted, the parent released, the counter down.
        e.map(TASK_COUNTER & !0xfff, 0x1000);
        e.mem.set_u32(TASK_COUNTER, 5);
        let children = object(&mut e, 0x14);
        let parents = object(&mut e, 0x10);
        e.mem.set_u32(this + CHILDREN, children);
        e.mem.set_u32(this + ADDITIONAL_PARENTS, parents);
        e.mem.set_u32(this + PARENT, 0x4444);
        e.mem.set_u32(this + FILE_NAME, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x00c3_cea0, &[this]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x5555]]
        );
        assert_eq!(calls_to(&e, 0x0095_0000).len(), 2);
        assert_eq!(calls_to(&e, POINTER_RELEASE), vec![vec![0x4444]]);
        assert_eq!(e.mem.u32(this), BS_TASK_VTABLE);
        assert_eq!(e.mem.u32(TASK_COUNTER), 4);

        // The scalar deleting destructors free only with bit 0.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_d3e0, &[this, 0]).u32(), this);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00c3_d3e0, &[this, 1]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
        let children = array(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00c3_d380, &[children, 1]);
        assert_eq!(e.mem.u32(children), SIMPLE_ARRAY_VTABLE);
        e.call(0x00c3_d3b0, &[children, 0]);
        assert_eq!(calls_to(&e, MEMORY_DEALLOCATE).len(), 1);
    }

    #[test]
    fn simple_array_constructs_grows_and_clears() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = object(&mut e, 0x14);
        // _Allocate (slot +4) answers a fresh block of count * 4 bytes.
        e.register(0x0095_0004, |e, a| e.mem.alloc(a[1] * 4).into_ret());
        e.register(0x0095_000c, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        // QueuedChildren: 5 reserved, none used.
        assert_eq!(e.call(0x00c3_c4e0, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), QUEUED_CHILDREN_VTABLE);
        assert_eq!(e.mem.u32(this + RESERVED), 5);
        assert_eq!(e.mem.u32(this + SIZE), 0);
        assert_eq!(calls_to(&e, 0x0095_0004), vec![vec![this, 5]]);
        // Two counted elements up front.
        fn_00c3d780(&mut e, this, 1, 2);
        assert_eq!(e.mem.u32(this + RESERVED), 2);
        assert_eq!(e.mem.u32(this + SIZE), 2);
        // A full array of 2 grows to 4, reallocated in place (the count is the reserve).
        e.register(SIMPLE_ARRAY_COPY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_d6c0, &[this]).u32(), 2);
        assert_eq!(e.mem.u32(this + RESERVED), 4);
        assert_eq!(e.mem.u32(this + SIZE), 3);
        assert_eq!(calls_to(&e, 0x0095_000c).len(), 1);
        assert!(calls_to(&e, SIMPLE_ARRAY_COPY).is_empty());
        // A count below the reserve copies into a new buffer and frees the old one.
        fn_00c3d870(&mut e, this, 0x800, 1);
        assert_eq!(calls_to(&e, SIMPLE_ARRAY_COPY).len(), 1);
        assert_eq!(calls_to(&e, 0x0095_0008).len(), 1);
        // An empty array gets 4 slots.
        let empty = object(&mut e, 0x14);
        e.register(0x0095_0004, |e, a| e.mem.alloc(a[1] * 4).into_ret());
        assert_eq!(fn_00c3d6c0(&mut e, empty), 0);
        assert_eq!(e.mem.u32(empty + RESERVED), 4);

        // Adding a pointer counts it up; clearing releases each element.
        let file = e.mem.alloc(0x30);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, file);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_d4f0, &[empty, holder]).u32(), 1);
        assert_eq!(e.mem.u32(file + REF_COUNT), 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_d930, &[empty]);
        assert_eq!(calls_to(&e, POINTER_RELEASE), vec![vec![file]]);
        assert_eq!(e.mem.u32(empty + BUFFER), 0);
        assert_eq!(e.mem.u32(empty + SIZE), 0);
        assert_eq!(e.mem.u32(empty), SIMPLE_ARRAY_VTABLE);
        // Clearing without freeing keeps the buffer.
        let kept = array(&mut e, &[0, file]);
        fn_00c3d980(&mut e, kept, 0);
        assert_ne!(e.mem.u32(kept + BUFFER), 0);
        assert_eq!(e.mem.u32(kept + SIZE), 0);
    }

    #[test]
    fn add_parent_links_both_ways_once() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(0x0095_0004, |e, a| e.mem.alloc(a[1] * 4).into_ret());
        let this = object(&mut e, 0x30);
        let parent = object(&mut e, 0x30);
        e.call(0x00c3_c7e0, &[this, parent]);
        let parents = e.mem.u32(this + ADDITIONAL_PARENTS);
        assert_eq!(e.mem.u32(parents), QUEUED_PARENTS_VTABLE);
        assert_eq!(e.mem.u32(parents + SIZE), 1);
        let children = e.mem.u32(parent + CHILDREN);
        assert_eq!(e.mem.u32(children), QUEUED_CHILDREN_VTABLE);
        assert_eq!(e.mem.u32(e.mem.u32(children + BUFFER)), this);
        // Already a parent: nothing more.
        e.call(0x00c3_c7e0, &[this, parent]);
        assert_eq!(e.mem.u32(parents + SIZE), 1);
        assert_eq!(e.mem.u32(children + SIZE), 1);
    }

    #[test]
    fn check_finished_notifies_the_parents() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = object(&mut e, 0x30);
        let parent = object(&mut e, 0x30);
        let other = object(&mut e, 0x30);
        let parents = array(&mut e, &[other]);
        e.mem.set_u32(this + PARENT, parent);
        e.mem.set_u32(this + ADDITIONAL_PARENTS, parents);
        let children = array(&mut e, &[0x10, 0x20]);
        e.mem.set_u32(this + CHILDREN, children);
        e.mem.set_u32(children + CHILDREN_FINISHED, 1);
        e.mem.set_u32(this + STATE, 4);
        // Children unfinished: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00c3_c930, &[this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // State below 4: nothing.
        e.mem.set_u32(children + CHILDREN_FINISHED, 2);
        e.mem.set_u32(this + STATE, 3);
        e.call(0x00c3_c930, &[this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        e.mem.set_u32(this + STATE, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c3_c930, &[this]);
        assert_eq!(
            calls_to(&e, 0x0095_0024),
            vec![vec![parent, this], vec![other, this]]
        );
        assert_eq!(
            calls_to(&e, POINTER_RELEASE),
            vec![vec![parent], vec![this]]
        );
        assert_eq!(e.mem.u32(this + PARENT), 0);
        assert_eq!(e.mem.u32(this + ADDITIONAL_PARENTS), 0);
        assert_eq!(calls_to(&e, 0x0095_0000), vec![vec![parents, 1]]);
    }

    #[test]
    fn requeue_and_cancel_walk_the_children() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.map(IO_MANAGER & !0xfff, 0x1000);
        let manager = object(&mut e, 8);
        e.mem.set_u32(IO_MANAGER, manager);
        e.register(IO_MANAGER_REQUEUE_TASK, |_, _| Ret::default());
        e.register(IO_MANAGER_0044AC40, |_, _| Ret::default());
        e.register(SIMPLE_ARRAY_REMOVE_AT, |e, a| {
            let size = e.mem.u32(a[0] + SIZE) - 1;
            e.mem.set_u32(a[0] + SIZE, size);
            Ret::default()
        });
        let this = object(&mut e, 0x30);
        let child = object(&mut e, 0x30);
        let children = array(&mut e, &[child, 0]);
        e.mem.set_u32(this + CHILDREN, children);
        e.call_log = Some(vec![]);
        e.call(0x00c3_cae0, &[this, 7]);
        assert_eq!(calls_to(&e, 0x0095_001c), vec![vec![child, 7]]);
        assert_eq!(
            calls_to(&e, IO_MANAGER_REQUEUE_TASK),
            vec![vec![manager, this, 7]]
        );

        // Cancel with no parent at all: the children go to the manager.
        e.call_log = Some(vec![]);
        e.call(0x00c3_cb70, &[this, 2, 0]);
        assert_eq!(
            calls_to(&e, IO_MANAGER_0044AC40),
            vec![vec![manager, child, this]]
        );
        assert_eq!(calls_to(&e, POINTER_RELEASE).last(), Some(&vec![this]));

        // Cancel by one of two parents: the state is exchanged (still parented).
        let first = object(&mut e, 0x30);
        let second = object(&mut e, 0x30);
        e.mem.set_u32(this + PARENT, first);
        let parents = array(&mut e, &[second]);
        e.mem.set_u32(this + ADDITIONAL_PARENTS, parents);
        e.mem.set_u32(this + STATE, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c3_cb70, &[this, 6, first]);
        assert_eq!(e.mem.u32(this + PARENT), 0);
        assert_eq!(e.mem.u32(this + STATE), 6);
        // State 1 goes to the manager slot +0x48.
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + PARENT, first);
        e.call(0x00c3_cb70, &[this, 1, 0x9999]);
        assert_eq!(calls_to(&e, 0x0095_0048), vec![vec![manager, this]]);
        // Cancel by the last additional parent: removed and the array deleted.
        e.mem.set_u32(this + PARENT, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_cb70, &[this, 6, second]);
        assert_eq!(
            calls_to(&e, SIMPLE_ARRAY_REMOVE_AT),
            vec![vec![parents, 0, 1]]
        );
        assert_eq!(e.mem.u32(this + ADDITIONAL_PARENTS), 0);
        assert_eq!(calls_to(&e, IO_MANAGER_0044AC40).len(), 1);
    }

    #[test]
    fn file_name_entry_and_get_file() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(STRLEN, |_, _| 5u32.into_ret());
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(ARCHIVE_PATH, |_, _| Ret::default());
        e.register(SPLIT_PATH, |_, _| Ret::default());
        e.register(ARCHIVE_FIND_ENTRY, |_, _| 0x7700u32.into_ret());
        e.register(ARCHIVE_GET_FILE, |_, _| Ret::default());
        e.register(FILE_FINDER_GET_FILE, |_, _| 0x8800u32.into_ret());
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x00c3_cee0, &[this, 0]);
        assert!(calls_to(&e, STRLEN).is_empty());
        // No name: no entry search, and GetFile answers 0.
        e.call(0x00c3_cf60, &[this, 0x10]);
        assert!(calls_to(&e, ARCHIVE_FIND_ENTRY).is_empty());
        assert_eq!(e.call(0x00c3_cff0, &[this, 0x10, 2]).u32(), 0);
        e.call(0x00c3_cee0, &[this, 0x1000]);
        let name = e.mem.u32(this + FILE_NAME);
        assert_eq!(calls_to(&e, STRCPY_S), vec![vec![name, 6, 0x1000]]);
        // An archive copies the name; none builds the path.
        e.call_log = Some(vec![]);
        e.call(0x00c3_cf60, &[this, 0x10]);
        assert_eq!(e.mem.u32(this + FILE_ENTRY), 0x7700);
        assert_eq!(calls_to(&e, STRCPY_S)[0][1..], [0x104, name]);
        e.call(0x00c3_d090, &[this, 0, 0x2000]);
        assert_eq!(calls_to(&e, ARCHIVE_PATH), vec![vec![name, 0x2000, 0x104]]);
        // The entry gives nothing: the file finder answers.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_cff0, &[this, 0x10, 2]).u32(), 0x8800);
        assert_eq!(
            calls_to(&e, ARCHIVE_GET_FILE),
            vec![vec![0x10, 0x7700, u32::MAX, 0]]
        );
        let finder = calls_to(&e, FILE_FINDER_GET_FILE);
        assert_eq!(finder[0][1..], [0, u32::MAX, 2]);
    }

    #[test]
    fn generate_key_packs_the_entry() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.map(KEY_COUNTER & !0xfff, 0x1000);
        e.mem.set_u32(KEY_COUNTER, 0x1_0010);
        e.register(IO_TASK_GENERATE_KEY, |_, _| Ret::default());
        e.register(ALLSHR, |_, a| {
            (((u64::from(a[0]) | u64::from(a[1]) << 32) as i64 >> a[2]) as u64).into_ret()
        });
        e.register(ALLSHL, |_, a| {
            ((u64::from(a[0]) | u64::from(a[1]) << 32) << a[2]).into_ret()
        });
        let this = object(&mut e, 0x30);
        e.register(0x0095_002c, |_, _| 0xffu32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00c3_d440, &[this]);
        assert_eq!(calls_to(&e, IO_TASK_GENERATE_KEY), vec![vec![this]]);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 0xc, 0x8000_0123);
        e.mem.set_u32(this + FILE_ENTRY, entry);
        e.mem.set_u32(this + KEY, 0x00ab_0000);
        e.call(0x00c3_d440, &[this]);
        // File index -1 sign-extended fills the bits from 56 up.
        let key = u64::from(e.mem.u32(this + KEY)) | u64::from(e.mem.u32(this + KEY + 4)) << 32;
        assert_eq!(key, 0xff00_0001_23ab_0011);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_00c3c520_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00c3_d930, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c3_c520, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00c3_d930)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x010c_14e8);
        assert_eq!(
            e.mem.u32(this + 4),
            0x010c_14e8,
            "the vtable was stored before the call"
        );
    }

    #[test]
    fn fn_00c3c570_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00c3_d930, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c3_c570, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00c3_d930)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x010c_1510);
        assert_eq!(
            e.mem.u32(this + 4),
            0x010c_1510,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    /// Runs the description of an entry with the given file entry and name; answers the
    /// `sprintf_s` arguments and the `strcat_s` sources (as text).
    fn describe(entry: bool, name: bool, children: &[(u32, bool)]) -> (Vec<u32>, Vec<Vec<u8>>) {
        let mut e = Engine::new();
        let printed = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = printed.clone();
        e.register_double(SPRINTF_S, move |_, a| {
            inner.borrow_mut().extend_from_slice(a);
            0u32.into_ret()
        });
        let appended = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = appended.clone();
        e.register_double(STRCAT_S, move |e, a| {
            assert_eq!((a[0], a[1]), (0x900, 0x40));
            inner.borrow_mut().push(e.mem.cstr(a[2]));
            0u32.into_ret()
        });
        e.map(CHILD_SEPARATOR, 8);
        e.mem.set_cstr(CHILD_SEPARATOR, b"\n * ");
        let child_vtable = 0x0300_4000;
        e.put_vtable(child_vtable, &[0, 0, 0, 0, 0x00ee_8000]);
        e.register(0x00ee_8000, |e, a| {
            assert_eq!(a[2], CHILD_DESCRIPTION_SIZE);
            let tag = e.mem.u32(a[0] + 4);
            e.mem
                .set_cstr(a[1], format!("child {}", tag & 0xff).as_bytes());
            (tag >> 8).into_ret()
        });
        let this = e.mem.alloc(0x30);
        if entry {
            let record = e.mem.alloc(0x10);
            e.mem.set_u32(record + 8, 0xc000_0010);
            e.mem.set_u32(record + 0xc, 0x8000_0020);
            e.mem.set_u32(this + FILE_ENTRY, record);
        }
        if name {
            e.mem.set_u32(this + FILE_NAME, 0x0600_0000);
        }
        if !children.is_empty() {
            let array = e.mem.alloc(0x10);
            let buffer = e.mem.alloc(4 * children.len() as u32);
            for (i, (tag, present)) in children.iter().enumerate() {
                if *present {
                    let child = e.mem.alloc(8);
                    e.mem.set_u32(child, child_vtable);
                    e.mem.set_u32(child + 4, *tag);
                    e.mem.set_u32(buffer + 4 * i as u32, child);
                }
            }
            e.mem.set_u32(array + BUFFER, buffer);
            e.mem.set_u32(array + SIZE, children.len() as u32);
            e.mem.set_u32(this + CHILDREN, array);
        }
        assert!(e
            .call(0x00c3_d0e0, &args![this, 0x900u32, 0x40u32, 0x0700_0000u32])
            .bool());
        let printed = printed.borrow().clone();
        let appended = appended.borrow().clone();
        (printed, appended)
    }

    #[test]
    fn the_description_format_follows_entry_and_name() {
        assert_eq!(
            describe(true, true, &[]).0,
            vec![
                0x900,
                0x40,
                DESCRIPTION_WITH_NAME,
                0x0700_0000,
                0x0600_0000,
                0x20,
                0x10
            ]
        );
        assert_eq!(
            describe(true, false, &[]).0,
            vec![
                0x900,
                0x40,
                DESCRIPTION_WITHOUT_NAME,
                0x0700_0000,
                0x20,
                0x10
            ]
        );
        assert_eq!(
            describe(false, true, &[]).0,
            vec![0x900, 0x40, DESCRIPTION_NAME_ONLY, 0x0700_0000, 0x0600_0000]
        );
    }

    #[test]
    fn described_children_are_appended() {
        let (_, appended) = describe(false, true, &[(0x101, true), (0x2, false), (0x3, true)]);
        assert_eq!(appended, vec![b"\n * ".to_vec(), b"child 1".to_vec()]);
    }

    #[test]
    fn entry_deleting_destructor() {
        let mut e = Engine::new();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + FILE_NAME, 0x55);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_d410, &args![this, 0u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        // The destructor chain ends at the `BSTask` table.
        assert_eq!(e.mem.u32(this), BS_TASK_VTABLE);
        assert!(log.contains(&(MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, 0x55])));
        assert!(!log.contains(&(MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, this])));
        e.call_log = Some(vec![]);
        e.call(0x00c3_d410, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, this])));
    }

    #[test]
    fn remove_at_releases_and_moves_the_tail() {
        let mut e = Engine::new();
        e.register(POINTER_RELEASE, |_, _| Ret::default());
        e.register(SIMPLE_ARRAY_COPY, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(4 * 6);
        for (i, word) in [1u32, 2, 0, 4, 5, 6].iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *word);
        }
        e.mem.set_u32(this + BUFFER, buffer);
        e.mem.set_u32(this + SIZE, 6);
        e.call_log = Some(vec![]);
        e.call(0x00c3_d5e0, &args![this, 1u32, 2u32]);
        let log = e.call_log.take().unwrap();
        let released: Vec<_> = log.iter().filter(|c| c.0 == POINTER_RELEASE).collect();
        assert_eq!(released.len(), 1);
        assert_eq!(released[0].1, vec![2]);
        // Three elements follow the gap, but only two (the count) are moved: the last two.
        assert!(log.contains(&(SIMPLE_ARRAY_COPY, vec![this, buffer + 4, buffer + 16, 2])));
        assert_eq!(e.mem.u32(this + SIZE), 4);
        // At the end only the one element after the gap moves.
        e.call_log = Some(vec![]);
        e.call(0x00c3_d5e0, &args![this, 1u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(SIMPLE_ARRAY_COPY, vec![this, buffer + 4, buffer + 12, 1])));
        assert_eq!(e.mem.u32(this + SIZE), 2);
    }

    #[test]
    fn array_deleting_destructor() {
        let mut e = Engine::new();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_d950, &args![this, 3u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), SIMPLE_ARRAY_VTABLE);
        assert!(log.contains(&(MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, this])));
        e.call_log = Some(vec![]);
        e.call(0x00c3_d950, &args![this, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == MEMORY_DEALLOCATE));
    }
}
