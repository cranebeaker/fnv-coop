//! `BSShader/mtrenderingsystem.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte that enables the second rendering task thread (`011f94b4`).
const SECOND_THREAD_ENABLED_011F94B4: u32 = 0x011f_94b4;
/// The word passed as `AddThread`'s second argument (`01200080`).
const THREAD_ARGUMENT_01200080: u32 = 0x0120_0080;
/// `MTRenderingSystem::AddThread` (engine map, `00ba2e00`): five stack words, cleaned by the
/// callee.
const ADD_THREAD: u32 = 0x00ba_2e00;
/// The thread's code (`00ba3190`), passed as an address.
const THREAD_CODE_00BA3190: u32 = 0x00ba_3190;
/// `"Rendering Task Thread 2"` (`010b7f84`).
const THREAD_NAME: u32 = 0x010b_7f84;

// Translated from 00ba3720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MTRenderingSystem::CreateThreads` (Xbox PDB). No stack arguments; it never touches ECX, so the
/// caller's ECX (the rendering system `01200088` at its one call, 00b57750) reaches
/// `MTRenderingSystem::AddThread` (`00ba2e00`) as its `this`. When the byte at `011f94b4` is not
/// 0, calls AddThread with 1, the word at
/// `01200080`, the code address `00ba3190`, the name `"Rendering Task Thread 2"`
/// (`010b7f84`) and 0x20000.
pub fn mt_rendering_system_create_threads(e: &mut Engine, this: Ptr) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) != 0 {
        let word = e.global::<u32>(THREAD_ARGUMENT_01200080);
        e.call(
            ADD_THREAD,
            &args![
                this.addr(),
                1u32,
                word,
                THREAD_CODE_00BA3190,
                THREAD_NAME,
                0x20000u32
            ],
        );
    }
}

/// `ReleaseSemaphore` (import slot): handle, count, previous count.
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
/// `InterlockedIncrement` (import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `WaitForSingleObject` (import slot): handle, milliseconds.
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
/// The compiler's array constructor helper, stdcall (array, element size,
/// count, element constructor, element destructor).
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// The compiler's array destructor helper (engine map: `eh vector
/// destructor iterator`), stdcall (array, element size, count, element
/// destructor).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
/// The semaphore holder's constructor and destructor (`run_00ba2d40.rs`),
/// passed as addresses.
const SEMAPHORE_CONSTRUCT: u32 = 0x00ba_2d40;
const SEMAPHORE_DESTRUCT: u32 = 0x00ba_2d70;
/// The task's constructor and destructor (this unit), passed as addresses.
const TASK_CONSTRUCT: u32 = 0x00ba_30e0;
const TASK_DESTRUCT: u32 = 0x00ba_3030;
/// The allocator object the task's frustum comes from (`011f6238`).
const FRUSTUM_ALLOCATOR: u32 = 0x011f_6238;
/// Thiscall on the allocator (size), `RET 4`: the block.
const ALLOCATOR_ALLOCATE: u32 = 0x00aa_3e40;
/// `BSCompoundFrustum::BSCompoundFrustum` (engine map), thiscall.
const COMPOUND_FRUSTUM_CONSTRUCT: u32 = 0x00c4_7660;
/// `BSCompoundFrustum::CopyMembers` (engine map), thiscall (destination),
/// `RET 4`.
const COMPOUND_FRUSTUM_COPY_MEMBERS: u32 = 0x00c4_77f0;
/// The `NiCullingProcess` constructor (`nicullingprocess.rs`), thiscall
/// (word), `RET 4`.
const CULLING_PROCESS_CONSTRUCT: u32 = 0x00a6_9400;
/// `BSCullingProcess`'s vtable (RTTI `.?AVBSCullingProcess@@`).
const BS_CULLING_PROCESS_VTABLE: u32 = 0x0101_e2ec;
/// `BSShaderUtil::AccumulateScene` (engine map), cdecl (camera, scene,
/// culling process).
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
/// `BSShaderUtil::AccumulateSceneList` (engine map), cdecl (camera, list,
/// culling process).
const ACCUMULATE_SCENE_LIST: u32 = 0x00b6_bfc0;
/// `BSCullingProcess::~BSCullingProcess` (engine map), thiscall.
const BS_CULLING_PROCESS_DESTRUCT: u32 = 0x004a_0f60;

/// Releases the reference-counted object at `object` (`InterlockedDecrement`
/// on its count at +4; at zero its vtable slot +4, `DeleteThis`).
fn release_reference(e: &mut Engine, object: u32) {
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// `NiPointer` assignment at `slot`: when the new object differs, the old
/// one is released, the new one stored and its count incremented.
fn assign_reference(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    release_reference(e, old);
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00ba2f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the threads are enabled (`011f94b4`),
/// clears the two words at +0xC and +0x10, then for each of the two thread
/// objects (`this[0]`, `this[1]`) releases its semaphore (+0x34) once and
/// counts it (+0x30).
pub fn fn_00ba2f30(e: &mut Engine, this: Ptr) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) == 0 {
        return;
    }
    for index in 0..2u32 {
        e.mem.set_u32(this.addr() + 0xc + 4 * index, 0);
    }
    for index in 0..2u32 {
        let thread = e.mem.u32(this.addr() + 4 * index);
        if thread != 0 {
            let semaphore = e.mem.u32(thread + 0x34);
            e.call(RELEASE_SEMAPHORE, &args![semaphore, 1u32, 0u32]);
            e.call(INTERLOCKED_INCREMENT, &args![thread + 0x30]);
        }
    }
}

// Translated from 00ba2fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the threads are enabled, waits for each
/// of the two thread objects' handle (+0x40) without a time-out and counts
/// it down (+0x3C).
pub fn fn_00ba2fa0(e: &mut Engine, this: Ptr) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) == 0 {
        return;
    }
    for index in 0..2u32 {
        let thread = e.mem.u32(this.addr() + 4 * index);
        if thread != 0 {
            let handle = e.mem.u32(thread + 0x40);
            e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
            e.call(INTERLOCKED_DECREMENT, &args![thread + 0x3c]);
        }
    }
}

// Translated from 00ba3030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba3030 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 between the releases) is not translated.
/// Unnamed in the engine map: the 0x20-byte accumulation task's destructor:
/// releases its three reference-counted objects, at +0x14, +8 and +0 in
/// that order.
pub fn fn_00ba3030(e: &mut Engine, this: Ptr) {
    for offset in [0x14, 0x8, 0x0] {
        let object = e.mem.u32(this.addr() + offset);
        release_reference(e, object);
    }
}

// Translated from 00ba30e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here): the 0x20-byte accumulation
/// task's constructor: clears its three object pointers (+0, +8, +0x14).
/// Returns `this`.
pub fn fn_00ba30e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0x14, 0);
    this
}

// Translated from 00ba30f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MTRenderingSystem::SetThreadStage` (engine map): when the threads are
/// enabled and `thread` is below 2, releases that thread's stage semaphore
/// (the holder at `this + 0x18 + 0xC * thread`: its handle +4) once and
/// counts it (holder +0). The second word is not read.
pub fn mt_rendering_system_set_thread_stage(
    e: &mut Engine,
    this: Ptr,
    thread: i32,
    _unused_2: u32,
) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) == 0 || thread >= 2 {
        return;
    }
    let holder = this
        .addr()
        .wrapping_add((thread.wrapping_mul(3).wrapping_add(6) as u32).wrapping_mul(4));
    let semaphore = e.mem.u32(holder + 4);
    e.call(RELEASE_SEMAPHORE, &args![semaphore, 1u32, 0u32]);
    e.call(INTERLOCKED_INCREMENT, &args![holder]);
}

// Translated from 00ba3130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the threads are enabled, waits on
/// `thread`'s stage semaphore (handle at `this + 0x1C + 0xC * thread`,
/// counted down at `+0x18 + 0xC * thread`) until the thread's stage
/// counter (`this + 0xC + 4 * thread`) reaches `stage`, counting it up
/// once per wait.
pub fn fn_00ba3130(e: &mut Engine, this: Ptr, thread: i32, stage: i32) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) == 0 {
        return;
    }
    let counter = this.addr().wrapping_add((thread as u32).wrapping_mul(4)) + 0xc;
    let holder = this
        .addr()
        .wrapping_add((thread.wrapping_mul(3).wrapping_add(6) as u32).wrapping_mul(4));
    while e.mem.i32(counter) < stage {
        let handle = e.mem.u32(holder + 4);
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![handle, 0xffff_ffffu32]);
        e.call(INTERLOCKED_DECREMENT, &args![holder]);
        let value = e.mem.i32(counter);
        e.mem.set_i32(counter, value.wrapping_add(1));
    }
}

// Translated from 00ba3390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba3390 - the C++ exception frame (FS:[0] chain, unwind state 0 around the frustum's allocation and 1 around the stack culling process) is not translated.
/// `MTRenderingSystem::AddAccumTask` (engine map). With the threads
/// enabled, fills task `index` (0x20 bytes at `this + 0x3C + 0x20 *
/// index`): `camera` (+0, a reference), the frustum cleared (+4), `scene`
/// (+8, a reference), `list` and `second_list` (+0xC, +0x10), `object`
/// (+0x14, a reference), `value` (+0x18) and `last` (+0x1C); with a
/// `frustum`, a copy of it (`BSCompoundFrustum`, 0xA4 bytes from the
/// allocator at `011f6238`) becomes the task's +4. Without the threads it
/// accumulates at once: a `BSCullingProcess` on the stack (the
/// `NiCullingProcess` constructor with 0, then the vtable and its fields at
/// +0x90, +0xBC, +0xC0, +0xC4: `value`, 0, `frustum`, `object` with a
/// reference) is passed to `BSShaderUtil::AccumulateScene(camera, scene)`
/// and `AccumulateSceneList(camera, list)` and `(camera, second_list)` for
/// each non-null one, then destroyed.
pub fn mt_rendering_system_add_accum_task(
    e: &mut Engine,
    this: Ptr,
    camera: u32,
    frustum: u32,
    scene: u32,
    list: u32,
    second_list: u32,
    object: u32,
    value: u32,
    index: u32,
    last: u32,
) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) != 0 {
        let task = this.addr().wrapping_add(index << 5);
        assign_reference(e, task + 0x3c, camera);
        let frustum_slot = this.addr().wrapping_add(index.wrapping_add(2) << 5);
        e.mem.set_u32(frustum_slot, 0);
        assign_reference(e, task + 0x44, scene);
        e.mem.set_u32(task + 0x48, list);
        e.mem.set_u32(task + 0x4c, second_list);
        assign_reference(e, task + 0x50, object);
        e.mem.set_u32(task + 0x54, value);
        e.mem.set_u32(task + 0x58, last);
        if frustum != 0 {
            let block = e
                .call(ALLOCATOR_ALLOCATE, &args![FRUSTUM_ALLOCATOR, 0xa4u32])
                .u32();
            let copy = if block != 0 {
                e.call(COMPOUND_FRUSTUM_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(frustum_slot, copy);
            e.call(COMPOUND_FRUSTUM_COPY_MEMBERS, &args![frustum, copy]);
        }
        return;
    }
    e.with_stack(0xc8, |e, process| {
        let process = process.addr();
        e.call(CULLING_PROCESS_CONSTRUCT, &args![process, 0u32]);
        e.mem.set_u32(process, BS_CULLING_PROCESS_VTABLE);
        e.mem.set_u32(process + 0x90, 0);
        e.mem.set_u32(process + 0xbc, 0);
        e.mem.set_u32(process + 0xc0, 0);
        e.mem.set_u32(process + 0xc4, 0);
        if object != 0 {
            e.mem.set_u32(process + 0xc4, object);
            e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
        }
        e.mem.set_u32(process + 0xc0, frustum);
        e.mem.set_u32(process + 0x90, value);
        if scene != 0 {
            e.call(ACCUMULATE_SCENE, &args![camera, scene, process]);
        }
        if list != 0 {
            e.call(ACCUMULATE_SCENE_LIST, &args![camera, list, process]);
        }
        if second_list != 0 {
            e.call(ACCUMULATE_SCENE_LIST, &args![camera, second_list, process]);
        }
        e.call(BS_CULLING_PROCESS_DESTRUCT, &args![process]);
    });
}

// Translated from 00ba3620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba3620 - the C++ exception frame (FS:[0] chain, unwind state 0 between the two arrays) is not translated.
/// Unnamed in the engine map: the rendering system's destructor: the 0x18
/// tasks of 0x20 bytes at +0x3C (`00ba3030`), then the three semaphore
/// holders of 0xC bytes at +0x18 (`00ba2d70`), through the compiler's array
/// destructor helper.
pub fn fn_00ba3620(e: &mut Engine, this: Ptr) {
    e.call(
        VECTOR_DESTRUCT,
        &args![this.addr() + 0x3c, 0x20u32, 0x18u32, TASK_DESTRUCT],
    );
    e.call(
        VECTOR_DESTRUCT,
        &args![this.addr() + 0x18, 0xcu32, 3u32, SEMAPHORE_DESTRUCT],
    );
}

// Translated from 00ba3690 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba3690 - the C++ exception frame (FS:[0] chain, unwind state 0 after the semaphores are built) is not translated.
/// Unnamed in the engine map: the rendering system's constructor: three
/// semaphore holders of 0xC bytes at +0x18 (`00ba2d40`/`00ba2d70`) and
/// 0x18 tasks of 0x20 bytes at +0x3C (`00ba30e0`/`00ba3030`) through the
/// compiler's array constructor helper, then the three words at +0 and the
/// three at +0xC cleared. Returns `this`.
pub fn fn_00ba3690(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            this.addr() + 0x18,
            0xcu32,
            3u32,
            SEMAPHORE_CONSTRUCT,
            SEMAPHORE_DESTRUCT
        ],
    );
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            this.addr() + 0x3c,
            0x20u32,
            0x18u32,
            TASK_CONSTRUCT,
            TASK_DESTRUCT
        ],
    );
    for index in 0..3u32 {
        e.mem.set_u32(this.addr() + 4 * index, 0);
        e.mem.set_u32(this.addr() + 4 * index + 0xc, 0);
    }
    this
}

/// The thread object's constructor (`008c76a0`, unit `ailineartaskthreadmanager.cpp`;
/// `thiscall`, no words), its vtable (`010b7f50`) and `BSThread`'s destructor body (`00aa63c0`,
/// unit `bsthread_xenon.cpp`; `thiscall`).
const THREAD_CONSTRUCT_008C76A0: u32 = 0x008c_76a0;
const RENDERING_THREAD_VTABLE: u32 = 0x010b_7f50;
const BS_THREAD_DESTROY_00AA63C0: u32 = 0x00aa_63c0;
/// `BSThread::Initialize` (engine map, `00aa6430`; `thiscall`, three words),
/// `SetThreadProcessor` (`00aa6550`, one word), `SetThreadPriority` (`00aa6560`, one word) and
/// `SetName` (`00aa6540`, one word: the name).
const BS_THREAD_INITIALIZE: u32 = 0x00aa_6430;
const BS_THREAD_SET_THREAD_PROCESSOR: u32 = 0x00aa_6550;
const BS_THREAD_SET_THREAD_PRIORITY: u32 = 0x00aa_6560;
const BS_THREAD_SET_NAME: u32 = 0x00aa_6540;
/// The formatter `00406d00` (`cdecl`: buffer, size, format, ...) and the format
/// `"Rendering Task Thread DEF %i"` (`010b7f64`).
const FORMAT_00406D00: u32 = 0x0040_6d00;
const DEFAULT_THREAD_NAME_FORMAT: u32 = 0x010b_7f64;
/// The memory manager's deallocation (`00aa4060`, `thiscall` on `011f6238`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `CloseHandle` (import slot).
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// The rendering system the thread code works on (`01200088`): its accumulation tasks from
/// the second on (`012000f0` is task 1's `+0xc`, up to `012003d0`), and the stage semaphore
/// holder of thread 1 (its count at `012000ac`, its handle at `012000b0`).
const RENDERING_SYSTEM_01200088: u32 = 0x0120_0088;
const TASK_1_LIST_012000F0: u32 = 0x0120_00f0;
const TASKS_END_012003D0: u32 = 0x0120_03d0;
const THREAD_1_STAGE_COUNT_012000AC: u32 = 0x0120_00ac;
const THREAD_1_STAGE_HANDLE_012000B0: u32 = 0x0120_00b0;
/// The word of the thread block the thread code sets to each task's last word (`+0x2bc`, the
/// allocation context).
const TLS_ALLOCATION_CONTEXT: u32 = 0x2bc;
/// `BSCompoundFrustum::~BSCompoundFrustum` (engine map, `00c47770`; `thiscall`).
const COMPOUND_FRUSTUM_DESTRUCT: u32 = 0x00c4_7770;

// Translated from 00ba2e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MTRenderingSystem::AddThread` (Xbox PDB; `thiscall` on the rendering system, five words:
/// the thread index, a processor word, the thread code, a name and a stack size). For an
/// index below 2 (signed; the code also tests below 3 first) whose slot (`this + 4 * index`) is
/// empty: a 0x50-byte thread from the memory manager (`00aa3e40` on `011f6238`) is constructed
/// when the allocation gave one (`008c76a0`, then the vtable `010b7f50` and the code at
/// `+0x4c`) and kept in the slot (null otherwise); then on the slot's thread
/// `BSThread::Initialize` (the stack size, the name, 0), `SetThreadProcessor` (the processor
/// word) and `SetThreadPriority` (0) run, and without a name the thread is named
/// `"Rendering Task Thread DEF %i"` with the index (`00406d00` into a 0x104-byte stack buffer,
/// then `SetName`).
///
/// The exception frame (`FS:[0]`) and the /GS cookie check are not translated.
// Translation incomplete: 00ba2e00 - the FS:[0] exception frame and the /GS cookie check (00ec408c) of its stack buffer are not translated
pub fn mt_rendering_system_add_thread(
    e: &mut Engine,
    this: Ptr,
    index: i32,
    processor: u32,
    code: u32,
    name: u32,
    stack_size: u32,
) {
    // The code compares with 3, then with 2 (signed); only the second can decide.
    if index >= 2 {
        return;
    }
    let slot = this.addr().wrapping_add((index as u32).wrapping_mul(4));
    if e.mem.u32(slot) != 0 {
        return;
    }
    let block = e
        .call(ALLOCATOR_ALLOCATE, &args![FRUSTUM_ALLOCATOR, 0x50u32])
        .u32();
    let thread = if block != 0 {
        e.call(THREAD_CONSTRUCT_008C76A0, &args![block]);
        e.mem.set_u32(block, RENDERING_THREAD_VTABLE);
        e.mem.set_u32(block + 0x4c, code);
        block
    } else {
        0
    };
    e.mem.set_u32(slot, thread);
    e.call(BS_THREAD_INITIALIZE, &args![thread, stack_size, name, 0u32]);
    let thread = e.mem.u32(slot);
    e.call(BS_THREAD_SET_THREAD_PROCESSOR, &args![thread, processor]);
    let thread = e.mem.u32(slot);
    e.call(BS_THREAD_SET_THREAD_PRIORITY, &args![thread, 0u32]);
    if name == 0 {
        e.with_stack(0x104, |e, buffer| {
            e.call(
                FORMAT_00406D00,
                &args![buffer, 0x104u32, DEFAULT_THREAD_NAME_FORMAT, index],
            );
            let thread = e.mem.u32(slot);
            e.call(BS_THREAD_SET_NAME, &args![thread, buffer]);
        });
    }
}

// Translated from 00ba2ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the rendering thread's scalar deleting destructor (slot 0 of its
/// vtable `010b7f50`; `thiscall`, one word: the flags). Closes the handles at `+0x40` and
/// `+0x34` (`CloseHandle`), runs `BSThread`'s destructor body (`00aa63c0`) and with bit 0 of
/// the flags gives the thread back to the memory manager. Answers `this`.
pub fn fn_00ba2ff0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    let handle = e.mem.u32(base + 0x40);
    e.call(CLOSE_HANDLE, &args![handle]);
    let handle = e.mem.u32(base + 0x34);
    e.call(CLOSE_HANDLE, &args![handle]);
    e.call(BS_THREAD_DESTROY_00AA63C0, &args![base]);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![FRUSTUM_ALLOCATOR, base]);
    }
    this
}

/// Releases the `NiPointer` at `slot` when it holds an object and clears it.
fn clear_reference(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        release_reference(e, object);
        e.mem.set_u32(slot, 0);
    }
}

// Translated from 00ba3190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the second rendering thread's code (the address
/// `MTRenderingSystem::CreateThreads` hands to `AddThread`; no arguments). For each
/// accumulation task from the second to the 24th of the rendering system at `01200088` (stage
/// 1 for the second, counting up): waits for thread 0 to reach the stage (`fn_00ba3130`); when
/// the task has its object (`+0x14`), sets the thread block's `+0x2bc` to the task's last word
/// (`+0x1c`), builds a `BSCullingProcess` on the stack as `AddAccumTask` does without threads
/// (the `NiCullingProcess` constructor with 0, the vtable `0101e2ec`, `+0x90`, `+0xbc`, `+0xc0`,
/// `+0xc4` cleared, the object counted into `+0xc4`, the task's `+0x18` into `+0x90` and its
/// frustum into `+0xc0`), accumulates the task's scene (`AccumulateScene`) and two lists
/// (`AccumulateSceneList`) with the task's camera when each is set, then clears the task: the
/// last word and the two lists cleared, the object, camera and scene released and cleared,
/// the frustum destroyed (`~BSCompoundFrustum`), given back to the memory manager and cleared,
/// and the culling process destroyed (`~BSCullingProcess`). With the threads enabled
/// (`011f94b4`) thread 1's stage semaphore is then released once and counted.
///
/// The exception frame (`FS:[0]`) is not translated, nor the `EAX` the code leaves at the end
/// (its last call's, the thread's exit word through `fn_00ba2d30`).
// Translation incomplete: 00ba3190 - the FS:[0] exception frame and the EAX left at the end (the last call's) are not translated
pub fn fn_00ba3190(e: &mut Engine) {
    let mut stage = 1i32;
    let mut task = TASK_1_LIST_012000F0;
    loop {
        fn_00ba3130(e, Ptr::new(RENDERING_SYSTEM_01200088), 0, stage);
        if e.mem.u32(task + 8) != 0 {
            let tls = e.tls();
            let last = e.mem.u32(task + 0x10);
            e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, last);
            e.with_stack(0xc8, |e, process| {
                let process = process.addr();
                e.call(CULLING_PROCESS_CONSTRUCT, &args![process, 0u32]);
                e.mem.set_u32(process, BS_CULLING_PROCESS_VTABLE);
                e.mem.set_u32(process + 0x90, 0);
                e.mem.set_u32(process + 0xbc, 0);
                e.mem.set_u32(process + 0xc0, 0);
                e.mem.set_u32(process + 0xc4, 0);
                let object = e.mem.u32(task + 8);
                if object != 0 {
                    e.mem.set_u32(process + 0xc4, object);
                    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
                }
                let scene = e.mem.u32(task - 4);
                let value = e.mem.u32(task + 0xc);
                let frustum = e.mem.u32(task - 8);
                e.mem.set_u32(process + 0x90, value);
                e.mem.set_u32(process + 0xc0, frustum);
                if scene != 0 {
                    let camera = e.mem.u32(task - 0xc);
                    e.call(ACCUMULATE_SCENE, &args![camera, scene, process]);
                }
                let list = e.mem.u32(task);
                if list != 0 {
                    let camera = e.mem.u32(task - 0xc);
                    e.call(ACCUMULATE_SCENE_LIST, &args![camera, list, process]);
                }
                let second = e.mem.u32(task + 4);
                if second != 0 {
                    let camera = e.mem.u32(task - 0xc);
                    e.call(ACCUMULATE_SCENE_LIST, &args![camera, second, process]);
                }
                e.mem.set_u32(task + 0x10, 0);
                e.mem.set_u32(task, 0);
                e.mem.set_u32(task + 4, 0);
                clear_reference(e, task + 8);
                clear_reference(e, task - 0xc);
                clear_reference(e, task - 4);
                let frustum = e.mem.u32(task - 8);
                if frustum != 0 {
                    e.call(COMPOUND_FRUSTUM_DESTRUCT, &args![frustum]);
                    e.call(MEMORY_DEALLOCATE, &args![FRUSTUM_ALLOCATOR, frustum]);
                    e.mem.set_u32(task - 8, 0);
                }
                e.call(BS_CULLING_PROCESS_DESTRUCT, &args![process]);
            });
        }
        if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) != 0 {
            let handle = e.mem.u32(THREAD_1_STAGE_HANDLE_012000B0);
            e.call(RELEASE_SEMAPHORE, &args![handle, 1u32, 0u32]);
            e.call(INTERLOCKED_INCREMENT, &args![THREAD_1_STAGE_COUNT_012000AC]);
        }
        task += 0x20;
        stage += 1;
        if (task as i32) >= TASKS_END_012003D0 as i32 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_2e00,
            mt_rendering_system_add_thread(Ptr, i32, u32, u32, u32, u32)
        ),
        entry!(0x00ba_2ff0, fn_00ba2ff0(Ptr, u8) -> Ptr),
        entry!(0x00ba_3190, fn_00ba3190()),
        entry!(0x00ba_3720, mt_rendering_system_create_threads(Ptr)),
        entry!(0x00ba_2f30, fn_00ba2f30(Ptr)),
        entry!(0x00ba_2fa0, fn_00ba2fa0(Ptr)),
        entry!(0x00ba_3030, fn_00ba3030(Ptr)),
        entry!(0x00ba_30e0, fn_00ba30e0(Ptr) -> Ptr),
        entry!(
            0x00ba_30f0,
            mt_rendering_system_set_thread_stage(Ptr, i32, u32)
        ),
        entry!(0x00ba_3130, fn_00ba3130(Ptr, i32, i32)),
        entry!(
            0x00ba_3390,
            mt_rendering_system_add_accum_task(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00ba_3620, fn_00ba3620(Ptr)),
        entry!(0x00ba_3690, fn_00ba3690(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_thread_calls(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ADD_THREAD)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn create_threads_adds_the_second_thread_only_when_enabled() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.set_global(THREAD_ARGUMENT_01200080, 0x4242u32);
        e.register(ADD_THREAD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ba_3720, &args![RENDERING_SYSTEM_01200088]);
        assert!(add_thread_calls(&e).is_empty());

        e.set_global(SECOND_THREAD_ENABLED_011F94B4, 1u8);
        e.call(0x00ba_3720, &args![RENDERING_SYSTEM_01200088]);
        assert_eq!(
            add_thread_calls(&e),
            vec![vec![
                RENDERING_SYSTEM_01200088,
                1,
                0x4242,
                0x00ba_3190,
                0x010b_7f84,
                0x20000
            ]]
        );
    }
}

#[cfg(test)]
mod rendering_system_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the enable byte mapped (set when `enabled`) and the
    /// imports doubled; `InterlockedDecrement` returns the decremented count.
    fn engine(enabled: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(SECOND_THREAD_ENABLED_011F94B4, u8::from(enabled));
        for addr in [RELEASE_SEMAPHORE, WAIT_FOR_SINGLE_OBJECT, 0x0000_7004] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            eax(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            eax(value)
        });
        e.put_vtable(0x0f00_0000, &[0, 0x0000_7004]);
        e
    }

    /// A reference-counted object with the test vtable and `count`.
    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0f00_0000);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn threads(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x400);
        let thread = e.mem.alloc(0x50);
        e.mem.set_u32(thread + 0x34, 0x51);
        e.mem.set_u32(thread + 0x40, 0x52);
        e.mem.set_u32(this, thread);
        (this, thread)
    }

    #[test]
    fn fn_00ba2f30_releases_the_thread_semaphores() {
        let mut e = engine(false);
        let (this, thread) = threads(&mut e);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        e.call(0x00ba_2f30, &args![this]);
        assert_eq!(e.mem.u32(this + 0xc), 5);
        e.set_global(SECOND_THREAD_ENABLED_011F94B4, 1u8);
        e.mem.set_u32(this + 0x10, 6);
        e.call(0x00ba_2f30, &args![this]);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (0, 0));
        assert_eq!(calls_to(&e, RELEASE_SEMAPHORE), vec![vec![0x51, 1, 0]]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![thread + 0x30]]
        );
    }

    #[test]
    fn fn_00ba2fa0_waits_for_the_threads() {
        let mut e = engine(false);
        let (this, thread) = threads(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00ba_2fa0, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.set_global(SECOND_THREAD_ENABLED_011F94B4, 1u8);
        e.call(0x00ba_2fa0, &args![this]);
        assert_eq!(
            calls_to(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x52, 0xffff_ffff]]
        );
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![thread + 0x3c]]
        );
    }

    #[test]
    fn fn_00ba3030_releases_the_three_references() {
        let mut e = engine(true);
        let task = e.mem.alloc(0x20);
        let last = object(&mut e, 1);
        let kept = object(&mut e, 3);
        e.mem.set_u32(task + 0x14, last);
        e.mem.set_u32(task, kept);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3030, &args![task]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![last + 4], vec![kept + 4]]
        );
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![last]]);
        assert_eq!(e.mem.u32(kept + 4), 2);
    }

    #[test]
    fn fn_00ba30e0_clears_the_task_pointers() {
        let mut e = engine(true);
        let task = e.mem.alloc(0x20);
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(task + offset, 9);
        }
        assert_eq!(e.call(0x00ba_30e0, &args![task]).u32(), task);
        assert_eq!(
            (e.mem.u32(task), e.mem.u32(task + 4), e.mem.u32(task + 8)),
            (0, 9, 0)
        );
        assert_eq!(e.mem.u32(task + 0x14), 0);
    }

    #[test]
    fn set_thread_stage_releases_one_stage() {
        let mut e = engine(true);
        let this = e.mem.alloc(0x400);
        e.mem.set_u32(this + 0x18 + 0xc + 4, 0x61);
        e.call_log = Some(vec![]);
        e.call(0x00ba_30f0, &args![this, 1u32, 0x99u32]);
        assert_eq!(calls_to(&e, RELEASE_SEMAPHORE), vec![vec![0x61, 1, 0]]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![this + 0x18 + 0xc]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00ba_30f0, &args![this, 2u32, 0u32]);
        e.set_global(SECOND_THREAD_ENABLED_011F94B4, 0u8);
        e.call(0x00ba_30f0, &args![this, 0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn fn_00ba3130_waits_until_the_stage_is_reached() {
        let mut e = engine(true);
        let this = e.mem.alloc(0x400);
        e.mem.set_u32(this + 0x18 + 4, 0x71);
        e.mem.set_u32(this + 0x18, 5);
        e.mem.set_u32(this + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3130, &args![this, 0u32, 3u32]);
        assert_eq!(
            calls_to(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x71, 0xffff_ffff], vec![0x71, 0xffff_ffff]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(e.mem.u32(this + 0x18), 3);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3130, &args![this, 0u32, 3u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn add_accum_task_fills_the_task_when_threaded() {
        let mut e = engine(true);
        e.register(ALLOCATOR_ALLOCATE, |_, _| eax(0x8800));
        e.register(COMPOUND_FRUSTUM_CONSTRUCT, |_, a| eax(a[0]));
        e.register(COMPOUND_FRUSTUM_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x400);
        let camera = object(&mut e, 0);
        let scene = object(&mut e, 0);
        let old = object(&mut e, 1);
        let task = this + 0x20;
        e.mem.set_u32(task + 0x3c, old);
        e.mem.set_u32(task + 0x40, 0x1234);
        e.call_log = Some(vec![]);
        e.call(
            0x00ba_3390,
            &args![this, camera, 0x4400u32, scene, 0x11u32, 0x12u32, 0u32, 0x13u32, 1u32, 0x14u32],
        );
        assert_eq!(e.mem.u32(task + 0x3c), camera);
        assert_eq!(e.mem.u32(camera + 4), 1);
        assert_eq!(calls_to(&e, 0x0000_7004), vec![vec![old]]);
        assert_eq!(e.mem.u32(task + 0x40), 0x8800);
        assert_eq!(e.mem.u32(task + 0x44), scene);
        assert_eq!(e.mem.u32(task + 0x48), 0x11);
        assert_eq!(e.mem.u32(task + 0x4c), 0x12);
        assert_eq!(e.mem.u32(task + 0x50), 0);
        assert_eq!(e.mem.u32(task + 0x54), 0x13);
        assert_eq!(e.mem.u32(task + 0x58), 0x14);
        assert_eq!(
            calls_to(&e, ALLOCATOR_ALLOCATE),
            vec![vec![FRUSTUM_ALLOCATOR, 0xa4]]
        );
        assert_eq!(
            calls_to(&e, COMPOUND_FRUSTUM_COPY_MEMBERS),
            vec![vec![0x4400, 0x8800]]
        );

        // The same camera again: no reference change; no frustum: cleared.
        e.call_log = Some(vec![]);
        e.call(
            0x00ba_3390,
            &args![this, camera, 0u32, scene, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
        assert_eq!(e.mem.u32(camera + 4), 1);
        assert_eq!(e.mem.u32(task + 0x40), 0);
        assert!(calls_to(&e, ALLOCATOR_ALLOCATE).is_empty());
    }

    #[test]
    fn add_accum_task_accumulates_at_once_without_threads() {
        let mut e = engine(false);
        e.register(CULLING_PROCESS_CONSTRUCT, |_, _| Ret::default());
        e.register_double(ACCUMULATE_SCENE, |e, a| {
            let process = a[2];
            assert_eq!(e.mem.u32(process), BS_CULLING_PROCESS_VTABLE);
            assert_eq!(e.mem.u32(process + 0x90), 0x13);
            assert_eq!(e.mem.u32(process + 0xc0), 0x4400);
            assert_ne!(e.mem.u32(process + 0xc4), 0);
            Ret::default()
        });
        e.register(ACCUMULATE_SCENE_LIST, |_, _| Ret::default());
        e.register(BS_CULLING_PROCESS_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x400);
        let held = object(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x00ba_3390,
            &args![this, 0x2000u32, 0x4400u32, 0x3000u32, 0u32, 0x12u32, held, 0x13u32, 1u32, 0u32],
        );
        assert_eq!(e.mem.u32(held + 4), 1);
        let scene = calls_to(&e, ACCUMULATE_SCENE);
        assert_eq!(scene.len(), 1);
        assert_eq!(scene[0][..2], [0x2000, 0x3000]);
        let lists = calls_to(&e, ACCUMULATE_SCENE_LIST);
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0][..2], [0x2000, 0x12]);
        assert_eq!(calls_to(&e, CULLING_PROCESS_CONSTRUCT)[0][1], 0);
        assert_eq!(
            calls_to(&e, BS_CULLING_PROCESS_DESTRUCT),
            vec![vec![scene[0][2]]]
        );
    }

    #[test]
    fn fn_00ba3620_destroys_the_arrays() {
        let mut e = engine(true);
        e.register(VECTOR_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x400);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3620, &args![this]);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![
                vec![this + 0x3c, 0x20, 0x18, 0x00ba_3030],
                vec![this + 0x18, 0xc, 3, 0x00ba_2d70]
            ]
        );
    }

    #[test]
    fn fn_00ba3690_builds_the_arrays_and_clears() {
        let mut e = engine(true);
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x400);
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(this + offset, 7);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_3690, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![
                vec![this + 0x18, 0xc, 3, 0x00ba_2d40, 0x00ba_2d70],
                vec![this + 0x3c, 0x20, 0x18, 0x00ba_30e0, 0x00ba_3030]
            ]
        );
        for offset in (0..0x18).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &addr in addresses {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn add_thread_makes_and_sets_up_the_thread() {
        let mut e = Engine::new();
        e.register(ALLOCATOR_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        quiet(
            &mut e,
            &[
                THREAD_CONSTRUCT_008C76A0,
                BS_THREAD_INITIALIZE,
                BS_THREAD_SET_THREAD_PROCESSOR,
                BS_THREAD_SET_THREAD_PRIORITY,
                BS_THREAD_SET_NAME,
                FORMAT_00406D00,
            ],
        );
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(
            0x00ba_2e00,
            &args![this, 1i32, 0x42u32, 0x00ba_3190u32, 0x900u32, 0x20000u32],
        );
        let thread = e.mem.u32(this + 4);
        assert_ne!(thread, 0);
        assert_eq!(e.mem.u32(thread), RENDERING_THREAD_VTABLE);
        assert_eq!(e.mem.u32(thread + 0x4c), 0x00ba_3190);
        assert_eq!(
            calls(&e, ALLOCATOR_ALLOCATE),
            vec![vec![FRUSTUM_ALLOCATOR, 0x50]]
        );
        assert_eq!(
            calls(&e, BS_THREAD_INITIALIZE),
            vec![vec![thread, 0x20000, 0x900, 0]]
        );
        assert_eq!(
            calls(&e, BS_THREAD_SET_THREAD_PROCESSOR),
            vec![vec![thread, 0x42]]
        );
        assert_eq!(
            calls(&e, BS_THREAD_SET_THREAD_PRIORITY),
            vec![vec![thread, 0]]
        );
        // A name was given: no default name.
        assert!(calls(&e, FORMAT_00406D00).is_empty());
        // The slot is taken now: nothing more.
        e.call(
            0x00ba_2e00,
            &args![this, 1i32, 0x42u32, 0x00ba_3190u32, 0u32, 0x20000u32],
        );
        assert_eq!(calls(&e, ALLOCATOR_ALLOCATE).len(), 1);
        // Index 2 is refused; index 0 without a name gets the default one.
        e.call(0x00ba_2e00, &args![this, 2i32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(calls(&e, ALLOCATOR_ALLOCATE).len(), 1);
        e.call(0x00ba_2e00, &args![this, 0i32, 0u32, 0u32, 0u32, 0u32]);
        let format = calls(&e, FORMAT_00406D00);
        assert_eq!(format.len(), 1);
        assert_eq!(&format[0][1..], &[0x104, DEFAULT_THREAD_NAME_FORMAT, 0]);
        let named = calls(&e, BS_THREAD_SET_NAME);
        assert_eq!(named, vec![vec![e.mem.u32(this), format[0][0]]]);
    }

    #[test]
    fn fn_00ba2ff0_closes_the_handles_and_frees_on_bit_0() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[CLOSE_HANDLE, BS_THREAD_DESTROY_00AA63C0, MEMORY_DEALLOCATE],
        );
        let thread = e.mem.alloc(0x50);
        e.mem.set_u32(thread + 0x34, 0x34);
        e.mem.set_u32(thread + 0x40, 0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ba_2ff0, &args![thread, 0u32]).u32(), thread);
        assert_eq!(calls(&e, CLOSE_HANDLE), vec![vec![0x40], vec![0x34]]);
        assert_eq!(calls(&e, BS_THREAD_DESTROY_00AA63C0), vec![vec![thread]]);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00ba_2ff0, &args![thread, 1u32]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![FRUSTUM_ALLOCATOR, thread]]
        );
    }

    #[test]
    fn thread_code_runs_and_clears_the_tasks() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
        e.put_vtable(0x0f00_0000, &[0, 0x0000_7004]);
        quiet(
            &mut e,
            &[
                0x0000_7004,
                CULLING_PROCESS_CONSTRUCT,
                ACCUMULATE_SCENE,
                ACCUMULATE_SCENE_LIST,
                BS_CULLING_PROCESS_DESTRUCT,
                COMPOUND_FRUSTUM_DESTRUCT,
                MEMORY_DEALLOCATE,
                RELEASE_SEMAPHORE,
            ],
        );
        let object = |e: &mut Engine| {
            let o = e.mem.alloc(8);
            e.mem.set_u32(o, 0x0f00_0000);
            e.mem.set_u32(o + 4, 1);
            o
        };
        // Task 3 (the third the code visits) has everything.
        let task = TASK_1_LIST_012000F0 + 2 * 0x20 - 0xc;
        let (camera, scene, held) = (object(&mut e), object(&mut e), object(&mut e));
        e.mem.set_u32(task, camera);
        e.mem.set_u32(task + 4, 0x4400);
        e.mem.set_u32(task + 8, scene);
        e.mem.set_u32(task + 0xc, 0xc00);
        e.mem.set_u32(task + 0x10, 0x1000);
        e.mem.set_u32(task + 0x14, held);
        e.mem.set_u32(task + 0x18, 0x18);
        e.mem.set_u32(task + 0x1c, 0x1c);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3190, &args![]);
        // Threads disabled: fn_00ba3130 returns at once; one task ran.
        let scene_calls = calls(&e, ACCUMULATE_SCENE);
        assert_eq!(scene_calls.len(), 1);
        let process = scene_calls[0][2];
        assert_eq!(&scene_calls[0][..2], &[camera, scene]);
        assert_eq!(
            calls(&e, ACCUMULATE_SCENE_LIST),
            vec![vec![camera, 0xc00, process], vec![camera, 0x1000, process]]
        );
        assert_eq!(calls(&e, CULLING_PROCESS_CONSTRUCT), vec![vec![process, 0]]);
        for offset in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x1c] {
            assert_eq!(e.mem.u32(task + offset), 0, "task word {offset:#x}");
        }
        assert_eq!(e.mem.u32(task + 0x18), 0x18);
        // Camera and scene go; the object keeps the count the culling process took (its
        // destructor is a double here).
        assert_eq!(calls(&e, 0x0000_7004), vec![vec![camera], vec![scene]]);
        assert_eq!(e.mem.u32(held + 4), 1);
        assert_eq!(calls(&e, COMPOUND_FRUSTUM_DESTRUCT), vec![vec![0x4400]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![FRUSTUM_ALLOCATOR, 0x4400]]
        );
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_ALLOCATION_CONTEXT), 0x1c);
        assert!(calls(&e, RELEASE_SEMAPHORE).is_empty());
        // Threads enabled: each of the 23 tasks releases thread 1's stage semaphore.
        e.set_global(SECOND_THREAD_ENABLED_011F94B4, 1u8);
        e.set_global(THREAD_1_STAGE_HANDLE_012000B0, 0x77u32);
        // Thread 0 is far ahead, so no waits.
        e.set_global(RENDERING_SYSTEM_01200088 + 0xc, 100u32);
        e.call_log = Some(vec![]);
        e.call(0x00ba_3190, &args![]);
        assert_eq!(calls(&e, RELEASE_SEMAPHORE).len(), 23);
        assert_eq!(e.global::<u32>(THREAD_1_STAGE_COUNT_012000AC), 23);
    }
}
