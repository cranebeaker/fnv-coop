//! `fallout shared/exteriorcellloader.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

const TASK_VTABLE: u32 = 0x0102_d028;
const TES_SINGLETON_GLOBAL: u32 = 0x011d_ea10;
const DATA_HANDLER_GLOBAL: u32 = 0x011c_3f2c;

// Translated from 00527c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `ExteriorCellLoaderTask` from its loader and cell-data pointers.
pub fn fn_00527c00(e: &mut Engine, this: Ptr, loader: Ptr, data: Ptr) -> Ptr {
    e.call(0x0044_0540, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), TASK_VTABLE);
    e.mem.set_u32(this.addr() + 0x18, loader.addr());
    e.mem.set_u32(this.addr() + 0x1c, data.addr());
    this
}

// Translated from 00527c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the task destructor and frees the object when flag bit zero is set.
pub fn fn_00527c40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00527c70(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00527c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the task's cell-data object and its task base.
pub fn fn_00527c70(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TASK_VTABLE);
    let data = e.mem.u32(this.addr() + 0x1c);
    e.call(0x0040_1030, &args![data]);
    e.call(0x0044_0640, &args![this]);
}

// Translated from 00527cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a cell's data, then queues its references.
pub fn exterior_cell_loader_task_run(e: &mut Engine, this: Ptr) {
    let loader = e.mem.u32(this.addr() + 0x18);
    let data = e.mem.u32(this.addr() + 0x1c);
    fn_00528620(e, Ptr::new(loader), Ptr::new(data));
    // 00527ccb-00527cd4: `ECX = [[this+0x1c]+0xc]`, the cell itself, is the `this` of
    // `TESObjectCELL::QueueReferences` (not the word at the cell's +0xc, its form id).
    let data = e.mem.u32(this.addr() + 0x1c);
    let cell = e.mem.u32(data + 0x0c);
    e.call(0x0055_71a0, &args![cell, 1u32]);
}

// Translated from 00527ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Submits this task to `IOManager::AddPostProcessTask`.
pub fn fn_00527ce0(e: &mut Engine, this: Ptr) {
    e.call(0x0043_d690, &args![this]);
}

// Translated from 00527d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attaches completed cell data when the game is active, clears its ready
/// flag, and notifies the loader with the cell coordinates.
pub fn exterior_cell_loader_task_post_process(e: &mut Engine, this: Ptr) {
    let data = e.mem.u32(this.addr() + 0x1c);
    let loader = e.mem.u32(this.addr() + 0x18);
    let cell = e.mem.u32(data + 0x0c);
    let ready = e.mem.u8(data + 0x10) != 0;
    if !e.call(0x0044_9130, &args![this]).bool() && cell != 0 && ready {
        exterior_cell_loader_attach_loaded_cell(e, Ptr::new(loader), cell);
    }
    e.mem.set_u8(data + 0x10, 0);
    let x = e.mem.u16(data);
    let y = e.mem.u16(data + 4);
    fn_00528560(e, Ptr::new(loader), x, y);
}

// Translated from 00527d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unloads an already-loaded cell, then notifies the loader with its coordinates.
pub fn exterior_cell_loader_task_cancel(e: &mut Engine, this: Ptr) {
    let data = e.mem.u32(this.addr() + 0x1c);
    let loader = e.mem.u32(this.addr() + 0x18);
    let cell = e.mem.u32(data + 0x0c);
    if cell != 0
        && e.call(
            0x0045_11e0,
            &args![e.global::<u32>(TES_SINGLETON_GLOBAL), cell, 0u32],
        )
        .bool()
    {
        e.call(
            0x0046_2290,
            &args![e.global::<u32>(DATA_HANDLER_GLOBAL), cell],
        );
    }
    let x = e.mem.u16(data);
    let y = e.mem.u16(data + 4);
    fn_00528560(e, Ptr::new(loader), x, y);
}

// Translated from 00527df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Formats the task description into the caller's bounded output buffer.
pub fn exterior_cell_loader_task_get_description(
    e: &mut Engine,
    this: Ptr,
    output: Ptr,
    capacity: u32,
) -> bool {
    let data = e.mem.u32(this.addr() + 0x1c);
    let x = e.mem.u32(data);
    let y = e.mem.u32(data + 4);
    e.call(0x0040_6d00, &args![output, capacity, 0x0102_d048u32, x, y]);
    true
}

// Translated from 00527e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears outstanding cell data, then destroys the task-map storage. The
/// compiler's SEH registration and unwind bookkeeping are omitted.
pub fn fn_00527e50(e: &mut Engine, this: Ptr) {
    fn_00527fc0(e, this);
    fn_00528be0(e, this);
}

// Translated from 00527eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lazily allocates and constructs the global exterior-cell loader.
pub fn fn_00527eb0(e: &mut Engine) {
    const LOADER: u32 = 0x011c_9618;
    if e.global::<u32>(LOADER) == 0 {
        let allocation = e.call(0x0040_1000, &args![0x40u32]).u32();
        let loader = if allocation == 0 {
            0
        } else {
            fn_00527e30(e, Ptr::new(allocation)).addr()
        };
        e.set_global(LOADER, loader);
    }
}

// Translated from 00527f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys and clears the global exterior-cell loader if it exists.
pub fn fn_00527f40(e: &mut Engine) {
    const LOADER: u32 = 0x011c_9618;
    let loader = e.global::<u32>(LOADER);
    if loader != 0 {
        fn_00527f90(e, Ptr::new(loader), 1);
        e.set_global(LOADER, 0u32);
    }
}

// Translated from 00527e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the loader's task-map storage with three buckets of each kind.
pub fn fn_00527e30(e: &mut Engine, this: Ptr) -> Ptr {
    fn_005289e0(e, this, 2, 0x25, 0x0c)
}

// Translated from 00527f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys loader storage and optionally frees it.
pub fn fn_00527f90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00527e50(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00528540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cancels all outstanding cell loads by clearing the loader's data.
pub fn exterior_cell_loader_cancel_all_cell_loads(e: &mut Engine, this: Ptr) {
    fn_00527fc0(e, this);
}

// Translated from 00528560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forms a cell coordinate key and calls the loader vtable's completion slot.
pub fn fn_00528560(e: &mut Engine, this: Ptr, x: u16, y: u16) {
    let key = e.call(0x0058_7410, &args![x, y]).u32();
    e.vcall(this.addr(), 0x14, &args![key]);
}

// Translated from 005285a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the object's vtable slot at byte offset `0x44` and answers its value
/// (the exe ends on that call and leaves its `EAX`, which the callers test).
pub fn fn_005285a0(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr(), 0x44, &args![]).u32()
}

// Translated from 005285c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates and initializes cell-load data for a non-null worldspace.
pub fn fn_005285c0(e: &mut Engine, _loader: Ptr, worldspace: u32, x: u32, y: u32) -> Ptr {
    if worldspace == 0 {
        return Ptr::NULL;
    }
    let data = e.call(0x0040_1000, &args![0x14u32]).u32();
    // 005285e0-0052860d: no test of the allocation; the fields go in this order.
    e.mem.set_u32(data.wrapping_add(8), worldspace);
    e.mem.set_u32(data, x);
    e.mem.set_u32(data.wrapping_add(4), y);
    e.mem.set_u8(data.wrapping_add(0x10), 0);
    e.mem.set_u32(data.wrapping_add(0x0c), 0);
    Ptr::new(data)
}

// Translated from 00528620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a cell or creates it through the data handler, then loads its land,
/// water and cell 3D data. The compiler's SEH bookkeeping is omitted.
pub fn fn_00528620(e: &mut Engine, _loader: Ptr, data: Ptr) {
    if data.is_null() {
        return;
    }
    let x = e.mem.u32(data.addr());
    let y = e.mem.u32(data.addr() + 4);
    let worldspace = e.mem.u32(data.addr() + 8);
    let mut cell = e.mem.u32(data.addr() + 0x0c);
    if cell == 0 {
        cell = e.call(0x0058_5b30, &args![worldspace, x, y]).u32();
        e.mem.set_u32(data.addr() + 0x0c, cell);
    }
    if cell == 0 {
        let handler = e.global::<u32>(DATA_HANDLER_GLOBAL);
        cell = e
            .call(0x0046_1330, &args![handler, 0u32, x, y, worldspace])
            .u32();
        e.mem.set_u32(data.addr() + 0x0c, cell);
    }
    e.call(0x0054_5cf0, &args![cell]);
    let land = e.call(0x0054_6fb0, &args![cell]).u32();
    if land != 0 {
        e.call(0x0053_5b90, &args![land, 1u32]);
        let tes = e.global::<u32>(TES_SINGLETON_GLOBAL);
        let water_present = fn_00528790(e, Ptr::new(tes));
        e.call(0x0053_fa40, &args![land, u32::from(water_present)]);
    }
    if !e.call(0x0045_1cd0, &args![cell]).bool() {
        e.with_stack(8, |e, heights| {
            // 005286e0-005286f1: `heights` is pushed for 0053f440; GetLand
            // (00546fb0, plain RET) takes only the cell in ECX.
            let land = e.call(0x0054_6fb0, &args![cell]).u32();
            e.call(0x0053_f440, &args![land, heights]);
            let water_height = e.call(0x0054_71e0, &args![cell]).f32();
            let minimum_height = e.mem.f32(heights.addr());
            e.call(
                0x0045_1250,
                &args![cell, u32::from(minimum_height < water_height)],
            );
        });
        // The address is the game setting object read by this function.
        let setting = e.call(0x0040_8d60, &args![0x011c_7adcu32]).u32();
        // 00528731: the byte is read through 00408d60's answer with no null test.
        if e.mem.u8(setting) != 0
            && e.call(0x0045_18e0, &args![cell]).bool()
            && !e.call(0x0045_1cd0, &args![cell]).bool()
        {
            e.call(0x0049_c860, &args![cell]);
        }
        e.call(0x0096_1f30, &args![cell, 1u32]);
    }
    e.mem.set_u8(data.addr() + 0x10, 1);
}

// Translated from 00528790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the water-present flag at byte offset `0x60`.
pub fn fn_00528790(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x60) != 0
}

// Translated from 005287b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Buffers the cell when it has a 3D object.
pub fn exterior_cell_loader_attach_loaded_cell(e: &mut Engine, _loader: Ptr, cell: u32) {
    if cell != 0 && e.call(0x0054_5cb0, &args![cell]).u32() != 0 {
        let tes = e.global::<u32>(TES_SINGLETON_GLOBAL);
        e.call(0x0045_4b90, &args![tes, cell]);
    }
}

// Translated from 00527fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Repeatedly removes tasks from the loader map and cancels them through the
/// global task manager. SEH cleanup tables are omitted.
pub fn fn_00527fc0(e: &mut Engine, this: Ptr) {
    e.with_stack(16, |e, stack| {
        fn_00528c70(e, stack);
        while !e.call(0x006e_bde0, &args![stack]).bool() {
            e.with_stack(8, |e, item| {
                e.call(0x0052_8cb0, &args![item, 0u32]);
                let removed = e
                    .call(
                        0x0090_6570,
                        &args![this, stack, item.addr() + 4, item, 1u32],
                    )
                    .bool();
                if removed {
                    let task = e.call(0x0055_9450, &args![item]).u32();
                    let manager = e.global::<u32>(0x0120_2d98);
                    e.call(0x0044_ac40, &args![manager, task, 0u32]);
                }
                e.call(0x0044_cbf0, &args![item]);
            });
        }
        fn_00528800(e, stack);
    });
}

// Translated from 00528080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up a coordinate key in the loader's task map using its iterator
/// lookup slot and destroys the temporary iterator.
pub fn fn_00528080(e: &mut Engine, this: Ptr, x: u16, y: u16) -> bool {
    let key = e.call(0x0058_7410, &args![x, y]).u32();
    e.with_stack(16, |e, iter| {
        e.call(0x0052_8cb0, &args![iter, 0u32]);
        // 005280d6..005280e1: the loader itself is the map (`MOV ECX,[EBP-0x1c]`, its `this`;
        // `MOV EDX,[ECX]` its vtable), not a pointer stored at its +0.
        let found = e.vcall(this.addr(), 8, &args![key, iter]).bool();
        e.call(0x0044_cbf0, &args![iter]);
        found
    })
}

// Translated from 00528110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks tasks, invoking vtable slot `0x14` on tasks whose state is 5.
/// Exception registration and unwinding are compiler generated and omitted.
pub fn fn_00528110(e: &mut Engine, this: Ptr) {
    e.with_stack(16, |e, stack| {
        fn_00528c70(e, stack);
        while !e.call(0x006e_bde0, &args![stack]).bool() {
            e.with_stack(8, |e, item| {
                e.call(0x0052_8cb0, &args![item, 0u32]);
                let removed = e
                    .call(
                        0x0090_6570,
                        &args![this, stack, item.addr() + 4, item, 1u32],
                    )
                    .bool();
                if removed {
                    let task = e.call(0x0055_9450, &args![item]).u32();
                    if e.call(0x0052_87e0, &args![task]).bool() {
                        // 0052819e-005281b4: the task is read again from the pointer.
                        let task = e.call(0x0055_9450, &args![item]).u32();
                        e.vcall(task, 0x14, &args![]);
                    }
                }
                e.call(0x0044_cbf0, &args![item]);
            });
        }
        fn_00528800(e, stack);
    });
}

// Translated from 005281f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tries to cancel every task and returns the number that refused cancellation.
/// Exception registration and unwinding are compiler generated and omitted.
pub fn exterior_cell_loader_try_cancel_tasks(e: &mut Engine, this: Ptr) -> i32 {
    let mut refused = 0;
    e.with_stack(16, |e, stack| {
        fn_00528c70(e, stack);
        while !e.call(0x006e_bde0, &args![stack]).bool() {
            e.with_stack(8, |e, item| {
                e.call(0x0052_8cb0, &args![item, 0u32]);
                let removed = e
                    .call(
                        0x0090_6570,
                        &args![this, stack, item.addr() + 4, item, 1u32],
                    )
                    .bool();
                if removed {
                    let task = e.call(0x0055_9450, &args![item]).u32();
                    let manager = e.global::<u32>(0x0120_2d98);
                    if !fn_00528870(e, Ptr::new(manager), Ptr::new(task), 0) {
                        refused += 1;
                    }
                }
                e.call(0x0044_cbf0, &args![item]);
            });
        }
        fn_00528800(e, stack);
    });
    refused
}

// Translated from 005282d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits for each extracted task and invokes vtable slot `0x14` when its
/// state reaches 5. Exception registration and unwinding are omitted.
pub fn fn_005282d0(e: &mut Engine, this: Ptr) {
    e.with_stack(16, |e, stack| {
        fn_00528c70(e, stack);
        while !e.call(0x006e_bde0, &args![stack]).bool() {
            e.with_stack(8, |e, item| {
                e.call(0x0052_8cb0, &args![item, 0u32]);
                let removed = e
                    .call(
                        0x0090_6570,
                        &args![this, stack, item.addr() + 4, item, 1u32],
                    )
                    .bool();
                if removed {
                    let task = e.call(0x0055_9450, &args![item]).u32();
                    let manager = e.global::<u32>(0x0120_2d98);
                    fn_005289b0(e, Ptr::new(manager), Ptr::new(task));
                    // 00528360-0052838c: the task is read from the pointer for each use.
                    let task = e.call(0x0055_9450, &args![item]).u32();
                    if e.call(0x0052_87e0, &args![task]).bool() {
                        let task = e.call(0x0055_9450, &args![item]).u32();
                        e.vcall(task, 0x14, &args![]);
                    }
                }
                e.call(0x0044_cbf0, &args![item]);
            });
        }
        fn_00528800(e, stack);
    });
}

// Translated from 005283c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues an exterior cell load when the target is not already loaded or
/// present in the loader's task map.
pub fn exterior_cell_loader_queue_cell_load(
    e: &mut Engine,
    this: Ptr,
    worldspace: u32,
    x: u32,
    y: u32,
) {
    if worldspace == 0 {
        return;
    }
    let cell = e.call(0x0058_75a0, &args![worldspace, x, y]).u32();
    let tes = e.global::<u32>(TES_SINGLETON_GLOBAL);
    if cell != 0 && e.call(0x0045_3490, &args![tes, cell]).bool() {
        e.call(0x0045_4b90, &args![tes, cell]);
        return;
    }
    if fn_00528080(e, this, x as u16, y as u16) {
        return;
    }
    let data = fn_005285c0(e, this, worldspace, x, y);
    if data.is_null() {
        return;
    }
    let allocation = e.call(0x0040_1000, &args![0x20u32]).u32();
    // 00528487-005284a9: a failed allocation gives a null task, which goes on below like
    // any other (the exe does not return here).
    let task = if allocation == 0 {
        Ptr::NULL
    } else {
        fn_00527c00(e, Ptr::new(allocation), this, data)
    };
    // 005284b6..0052851e: the task is held by a smart pointer at [EBP-0x18] (`00528cb0`); the
    // map (the loader itself: `MOV ECX,[EBP-0x2c]`, its `this`) is given the key, the pointer's
    // address and 1 (slot 0x10); the manager (0x48) is given the raw task back from it
    // (`00559450`), and the pointer is let go (`0044cbf0`).
    e.with_stack(4, |e, holder| {
        e.call(0x0052_8cb0, &args![holder, task]);
        let key = e.call(0x0058_7410, &args![x as u16, y as u16]).u32();
        e.vcall(this.addr(), 0x10, &args![key, holder, 1u32]);
        let held = e.call(0x0055_9450, &args![holder]).u32();
        let manager = e.global::<u32>(0x0120_2d98);
        e.vcall(manager, 0x48, &args![held]);
        e.call(0x0044_cbf0, &args![holder]);
    });
}

// Translated from 005287e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns whether the task state field equals 5.
pub fn fn_005287e0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x0c) == 5
}

// Translated from 00528800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the lock-free map iterator vtable.
pub fn fn_00528800(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_d078);
}

// Translated from 00528820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the iterator's current-entry field.
pub fn fn_00528820(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 8, 0);
}

// Translated from 00528840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the iterator destructor and optionally frees it.
pub fn fn_00528840(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00528800(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00528870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attempts to move a task into the cancelled state, runs its cancellation
/// callback on success, and waits while it is transitioning through state 3.
pub fn fn_00528870(e: &mut Engine, manager: Ptr, task: Ptr, context: u32) -> bool {
    loop {
        let state = e.mem.u32(task.addr() + 0x0c);
        match state {
            0 | 1 | 5 => {
                if !e.call(0x0044_9190, &args![task, state, 6u32]).bool() {
                    continue;
                }
                if state == 1 {
                    e.call(0x0092_c870, &args![task]);
                    e.call(0x0044_e050, &args![manager, task]);
                    e.vcall(task.addr(), 0x0c, &args![state, context]);
                    e.call(0x0044_dd60, &args![task]);
                } else {
                    e.vcall(task.addr(), 0x0c, &args![state, context]);
                }
                return true;
            }
            2 => {
                while e.mem.u32(task.addr() + 0x0c) == 3 {
                    e.call(0x0040_fca0, &args![0u32]);
                }
            }
            3 | 4 => return false,
            6.. => return true,
        }
    }
}

// Translated from 005289b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits while a task is in the running or cancellation-transition state.
pub fn fn_005289b0(e: &mut Engine, _manager: Ptr, task: Ptr) {
    while matches!(e.mem.u32(task.addr() + 0x0c), 3 | 5) {
        e.call(0x0040_fca0, &args![0u32]);
    }
}

// Translated from 00528b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Locks the object, gets its map (00449f80), looks the key up in it with
/// 0044c000, then releases the lock and returns the lookup's byte result.
pub fn fn_00528b60(e: &mut Engine, this: Ptr, first: u32, second: u32, mode: u8) -> bool {
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    let map = e.call(0x0044_9f80, &args![this]).u32();
    let result = e.call(0x0044_c000, &args![map, first, second, mode]).bool();
    e.call(0x0040_fba0, &args![this.addr() + 0x20]);
    result
}

// Translated from 00528bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Obtains the coordinate's map key through the object's vtable and passes it
/// to the map insertion helper, answering the helper's result (the exe ends on
/// that call and leaves its `EAX`; the callers keep `AL`). The exe pushes
/// `key` twice: the second push is the virtual's one word, the first stays for
/// `00528ce0`.
pub fn fn_00528bb0(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let object = e.mem.u32(this.addr());
    let result = e.vcall(object, 0x24, &args![key]).u32();
    fn_00528ce0(e, this, result, key)
}

// Translated from 00528be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the concurrent map and frees its two allocated arrays.
/// Compiler SEH registration and unwind bookkeeping are omitted.
pub fn fn_00528be0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_d084);
    e.call(0x0052_8e40, &args![this, 1u32]);
    let buckets = e.mem.u32(this.addr() + 0x0c);
    let nodes = e.mem.u32(this.addr() + 4);
    e.call(0x0040_1030, &args![buckets]);
    e.call(0x0040_1030, &args![nodes]);
    e.call(0x0044_b4b0, &args![this]);
}

// Translated from 00528c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an empty task-stack iterator.
pub fn fn_00528c70(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0102_d078);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u8(this.addr() + 0x0c, 0);
    this
}

// Translated from 005289e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the task map, its bucket storage, and the quick-bit-set state.
/// Compiler SEH registration and unwind bookkeeping are omitted.
pub fn fn_005289e0(
    e: &mut Engine,
    this: Ptr,
    map_size: u32,
    bucket_count: u32,
    item_size: u32,
) -> Ptr {
    e.call(0x0044_d7f0, &args![this]);
    e.mem.set_u32(this.addr(), 0x0102_d084);
    e.call(0x0096_a2d0, &args![this.addr() + 0x20]);
    e.with_stack(4, |e, debug_state| {
        e.call(
            0x0040_4eb0,
            &args![debug_state, 6u32, 1u32, 0x0101_73f8u32, 0x390u32],
        );
        e.mem.set_u32(this.addr() + 0x18, 0);
        e.mem.set_u32(this.addr() + 8, bucket_count);
        let elements = e
            .call(0x0040_1000, &args![bucket_count.wrapping_mul(4)])
            .u32();
        let elements = if elements == 0 {
            0
        } else {
            e.call(
                0x0040_1050,
                &args![elements, 4u32, bucket_count, 0x0044_dee0u32],
            );
            elements
        };
        e.mem.set_u32(this.addr() + 0x0c, elements);
        let nodes = e
            .call(
                0x0040_1000,
                &args![map_size.wrapping_mul(3).wrapping_mul(4)],
            )
            .u32();
        e.mem.set_u32(this.addr() + 4, nodes);
        e.mem.set_u32(this.addr() + 0x10, item_size);
        let storage = e.call(0x0040_1000, &args![0x10u32]).u32();
        let storage = if storage == 0 {
            0
        } else {
            e.call(0x006c_73b0, &args![storage, map_size]).u32()
        };
        e.mem.set_u32(this.addr() + 0x14, storage);
        e.call(0x0040_4ee0, &args![debug_state]);
    });
    this
}

// Translated from 00528ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts a key/value pair into the task map. On contention it retries the
/// map operation, and marks the map as dirty after insertion.
pub fn fn_00528ce0(e: &mut Engine, this: Ptr, item: u32, key: u32) -> bool {
    loop {
        if !e.call(0x0052_9030, &args![this, item, key]).bool() {
            e.call(0x0052_9000, &args![this]);
            return false;
        }
        let inserted = e.with_stack(12, |e, temporaries| {
            let lock0 = e.call(0x0044_dec0, &args![this.addr() + 0x18]).u32();
            e.call(0x006e_cba0, &args![temporaries, 0u32, lock0]);
            let lock1 = e.call(0x0044_dec0, &args![this.addr() + 0x18]).u32();
            e.call(
                0x006e_cba0,
                &args![Ptr::<()>::new(temporaries.addr() + 4), 1u32, lock1],
            );
            let first = e.call(0x0055_9450, &args![temporaries]).u32();
            let second = e
                .call(0x0055_9450, &args![Ptr::<()>::new(temporaries.addr() + 4)])
                .u32();
            let bucket = e.call(0x0044_df00, &args![this.addr() + 0x14]).u32();
            if !e
                .call(0x0044_91c0, &args![bucket + 8, second, first])
                .bool()
            {
                return None;
            }
            let lock2 = e.call(0x0044_dec0, &args![this.addr() + 0x14]).u32();
            e.call(
                0x006e_cba0,
                &args![Ptr::<()>::new(temporaries.addr() + 8), 0u32, lock2],
            );
            let third = e
                .call(0x0055_9450, &args![Ptr::<()>::new(temporaries.addr() + 8)])
                .u32();
            let fourth = e.call(0x0055_9450, &args![temporaries]).u32();
            let map_item_size = e.mem.u32(this.addr() + 0x10);
            Some(
                e.call(0x0044_91c0, &args![map_item_size, fourth, third])
                    .bool(),
            )
        });
        let Some(inserted) = inserted else {
            continue;
        };
        if inserted {
            let lock = e.call(0x0044_dec0, &args![this.addr() + 0x14]).u32();
            e.call(0x0052_92a0, &args![this, lock]);
        } else {
            e.call(0x0052_9030, &args![this, item, key]);
        }
        e.vcall(e.mem.u32(this.addr()), 0x40, &args![]);
        e.call(0x0052_9000, &args![this]);
        return true;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00527c00, fn_00527c00(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00527c40, fn_00527c40(Ptr, u32) -> Ptr),
        entry!(0x00527c70, fn_00527c70(Ptr)),
        entry!(0x00527cb0, exterior_cell_loader_task_run(Ptr)),
        entry!(0x00527ce0, fn_00527ce0(Ptr)),
        entry!(0x00527d00, exterior_cell_loader_task_post_process(Ptr)),
        entry!(0x00527d80, exterior_cell_loader_task_cancel(Ptr)),
        entry!(0x00527df0, exterior_cell_loader_task_get_description(Ptr, Ptr, u32) -> bool),
        entry!(0x00527e30, fn_00527e30(Ptr) -> Ptr),
        entry!(0x00527e50, fn_00527e50(Ptr)),
        entry!(0x00527eb0, fn_00527eb0()),
        entry!(0x00527f40, fn_00527f40()),
        entry!(0x00527f90, fn_00527f90(Ptr, u32) -> Ptr),
        entry!(0x00528540, exterior_cell_loader_cancel_all_cell_loads(Ptr)),
        entry!(0x00528560, fn_00528560(Ptr, u16, u16)),
        entry!(0x005285a0, fn_005285a0(Ptr) -> u32),
        entry!(0x005285c0, fn_005285c0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00528620, fn_00528620(Ptr, Ptr)),
        entry!(0x00528790, fn_00528790(Ptr) -> bool),
        entry!(
            0x005287b0,
            exterior_cell_loader_attach_loaded_cell(Ptr, u32)
        ),
        entry!(0x005287e0, fn_005287e0(Ptr) -> bool),
        entry!(0x00528800, fn_00528800(Ptr)),
        entry!(0x00528820, fn_00528820(Ptr)),
        entry!(0x00528840, fn_00528840(Ptr, u32) -> Ptr),
        entry!(0x00528870, fn_00528870(Ptr, Ptr, u32) -> bool),
        entry!(0x005289b0, fn_005289b0(Ptr, Ptr)),
        entry!(0x00528b60, fn_00528b60(Ptr, u32, u32, u8) -> bool),
        entry!(0x00528bb0, fn_00528bb0(Ptr, u32) -> bool),
        entry!(0x00528be0, fn_00528be0(Ptr)),
        entry!(0x00528c70, fn_00528c70(Ptr) -> Ptr),
        entry!(0x005289e0, fn_005289e0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00528ce0, fn_00528ce0(Ptr, u32, u32) -> bool),
        entry!(0x00527fc0, fn_00527fc0(Ptr)),
        entry!(0x00528080, fn_00528080(Ptr, u16, u16) -> bool),
        entry!(0x00528110, fn_00528110(Ptr)),
        entry!(0x005281f0, exterior_cell_loader_try_cancel_tasks(Ptr) -> i32),
        entry!(0x005282d0, fn_005282d0(Ptr)),
        entry!(
            0x005283c0,
            exterior_cell_loader_queue_cell_load(Ptr, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x011c_9000, 0x011d_e000, 0x0120_2000, 0x0600_0000] {
            e.map(page, 0x1000);
        }
        e
    }

    fn stub(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret::default());
    }

    fn returning(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    #[test]
    fn constructor_stores_loader_and_data() {
        let mut e = engine();
        stub(&mut e, 0x0044_0540);
        let this = Ptr::new(e.mem.alloc(0x20));
        let loader = Ptr::new(0x0600_0100);
        let data = Ptr::new(0x0600_0200);
        assert_eq!(fn_00527c00(&mut e, this, loader, data), this);
        assert_eq!(e.mem.u32(this.addr()), TASK_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x18), loader.addr());
        assert_eq!(e.mem.u32(this.addr() + 0x1c), data.addr());
    }

    #[test]
    fn deleting_destructor_runs_task_and_base_destructors() {
        let mut e = engine();
        stub(&mut e, 0x0040_1030);
        stub(&mut e, 0x0044_0640);
        let this = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 0x1c, 0x0600_0100);
        assert_eq!(fn_00527c40(&mut e, this, 1), this);
        assert_eq!(e.mem.u32(this.addr()), TASK_VTABLE);
    }

    #[test]
    fn postprocess_clears_ready_and_notifies_the_coordinate_key() {
        let mut e = engine();
        returning(&mut e, 0x0044_9130, 1);
        returning(&mut e, 0x0058_7410, 9);
        let loader = Ptr::<()>::new(e.mem.alloc(4));
        let vtable = 0x0600_0300;
        e.put_vtable(vtable, &[0, 0, 0, 0, 0, 0x0600_0400]);
        e.mem.set_u32(loader.addr(), vtable);
        stub(&mut e, 0x0600_0400);
        let data = Ptr::<()>::new(e.mem.alloc(0x14));
        e.mem.set_u32(data.addr(), 2);
        e.mem.set_u32(data.addr() + 4, 3);
        e.mem.set_u32(data.addr() + 0x0c, 0x0600_0500);
        e.mem.set_u8(data.addr() + 0x10, 1);
        let this = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 0x18, loader.addr());
        e.mem.set_u32(this.addr() + 0x1c, data.addr());
        exterior_cell_loader_task_post_process(&mut e, this);
        assert_eq!(e.mem.u8(data.addr() + 0x10), 0);
    }

    #[test]
    fn cancel_skips_unload_for_missing_cell() {
        let mut e = engine();
        returning(&mut e, 0x0058_7410, 9);
        let loader = Ptr::<()>::new(e.mem.alloc(4));
        let vtable = 0x0600_0300;
        e.put_vtable(vtable, &[0, 0, 0, 0, 0, 0x0600_0400]);
        e.mem.set_u32(loader.addr(), vtable);
        stub(&mut e, 0x0600_0400);
        let data = Ptr::<()>::new(e.mem.alloc(0x14));
        let this = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 0x18, loader.addr());
        e.mem.set_u32(this.addr() + 0x1c, data.addr());
        exterior_cell_loader_task_cancel(&mut e, this);
        assert_eq!(e.mem.u32(data.addr() + 0x0c), 0);
    }

    #[test]
    fn description_formats_both_cell_coordinates() {
        let mut e = engine();
        let this = Ptr::new(e.mem.alloc(0x20));
        let data = Ptr::<()>::new(e.mem.alloc(0x14));
        let output = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(data.addr(), 12);
        e.mem.set_u32(data.addr() + 4, 34);
        e.mem.set_u32(this.addr() + 0x1c, data.addr());
        stub(&mut e, 0x0040_6d00);
        e.call_log = Some(vec![]);
        assert!(exterior_cell_loader_task_get_description(
            &mut e, this, output, 0x40
        ));
        let log = e.call_log.take().unwrap();
        let format = log.iter().find(|(a, _)| *a == 0x0040_6d00).unwrap();
        assert_eq!(format.1, vec![output.addr(), 0x40, 0x0102_d048, 12, 34]);
    }

    #[test]
    fn postprocess_task_dispatches_to_io_manager() {
        let mut e = engine();
        stub(&mut e, 0x0043_d690);
        let task = Ptr::new(0x0600_0100);
        fn_00527ce0(&mut e, task);
        assert_eq!(e.call_log, None);
    }

    #[test]
    fn lazy_loader_initializer_does_not_allocate_twice() {
        let mut e = engine();
        e.set_global(0x011c_9618, 0x0600_0100u32);
        fn_00527eb0(&mut e);
        assert_eq!(e.global::<u32>(0x011c_9618), 0x0600_0100);
    }

    #[test]
    fn loader_finalizer_clears_the_global_pointer() {
        let mut e = engine();
        e.set_global(0x011c_9618, 0);
        fn_00527f40(&mut e);
        assert_eq!(e.global::<u32>(0x011c_9618), 0);
    }

    #[test]
    fn coordinate_data_allocation_initializes_fields() {
        let mut e = engine();
        e.map(0x0601_0000, 0x1000);
        returning(&mut e, 0x0040_1000, 0x0601_0000);
        let data = fn_005285c0(&mut e, Ptr::NULL, 0x0600_0100, 7, 8);
        assert_eq!(e.mem.u32(data.addr()), 7);
        assert_eq!(e.mem.u32(data.addr() + 4), 8);
        assert_eq!(e.mem.u32(data.addr() + 8), 0x0600_0100);
        assert_eq!(e.mem.u8(data.addr() + 0x10), 0);
    }

    #[test]
    fn coordinate_data_fields_are_stored_in_the_exe_order() {
        // 005285e9-0052860d: world space (+8), x (+0), y (+4), the done byte
        // (+0x10), the cell (+0xc).
        let mut e = engine();
        e.map(0x0601_0000, 0x1000);
        for offset in 0..0x14 {
            e.mem.set_u8(0x0601_0000 + offset, 0xff);
        }
        returning(&mut e, 0x0040_1000, 0x0601_0000);
        e.mem.watch(0x0601_0000, 0x14, false);
        let data = fn_005285c0(&mut e, Ptr::NULL, 0x0600_0100, 7, 8);
        let stores: Vec<u32> = e
            .mem
            .take_watch_hits()
            .iter()
            .map(|hit| hit.addr - data.addr())
            .collect();
        assert_eq!(stores, vec![8, 0, 4, 0x10, 0x0c]);
        assert_eq!(e.mem.u32(data.addr() + 0x0c), 0);
    }

    #[test]
    #[should_panic(expected = "write to unmapped memory at 00000008")]
    fn coordinate_data_allocation_is_not_tested() {
        // 005285dd-005285ef: the block goes straight to the stores.
        let mut e = engine();
        returning(&mut e, 0x0040_1000, 0);
        fn_005285c0(&mut e, Ptr::NULL, 0x0600_0100, 7, 8);
    }

    #[test]
    fn cell_data_load_returns_for_a_null_argument() {
        let mut e = engine();
        fn_00528620(&mut e, Ptr::NULL, Ptr::NULL);
    }

    #[test]
    fn water_flag_reads_offset_sixty() {
        let mut e = engine();
        let object = Ptr::new(e.mem.alloc(0x64));
        e.mem.set_u8(object.addr() + 0x60, 1);
        assert!(fn_00528790(&mut e, object));
    }

    #[test]
    fn attach_loaded_cell_ignores_a_null_cell() {
        exterior_cell_loader_attach_loaded_cell(&mut engine(), Ptr::NULL, 0);
    }

    #[test]
    fn task_state_five_is_recognized() {
        let mut e = engine();
        let task = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(task.addr() + 0x0c, 5);
        assert!(fn_005287e0(&mut e, task));
    }

    #[test]
    fn map_iterator_constructor_and_clear_write_fields() {
        let mut e = engine();
        let iterator = Ptr::new(e.mem.alloc(0x10));
        fn_00528820(&mut e, iterator);
        assert_eq!(e.mem.u32(iterator.addr() + 8), 0);
        fn_00528800(&mut e, iterator);
        assert_eq!(e.mem.u32(iterator.addr()), 0x0102_d078);
    }

    #[test]
    fn task_cancellation_refuses_state_four() {
        let mut e = engine();
        let task = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(task.addr() + 0x0c, 4);
        assert!(!fn_00528870(&mut e, Ptr::NULL, task, 0));
    }

    #[test]
    fn task_wait_returns_when_state_is_not_transitional() {
        let mut e = engine();
        let task = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(task.addr() + 0x0c, 4);
        fn_005289b0(&mut e, Ptr::NULL, task);
    }

    #[test]
    fn map_constructor_initializes_empty_allocations() {
        let mut e = engine();
        for address in [0x0044_d7f0, 0x0096_a2d0, 0x0040_4eb0, 0x0040_4ee0] {
            stub(&mut e, address);
        }
        returning(&mut e, 0x0040_1000, 0);
        let map = Ptr::new(e.mem.alloc(0x30));
        fn_005289e0(&mut e, map, 2, 0x25, 0x0c);
        assert_eq!(e.mem.u32(map.addr()), 0x0102_d084);
        assert_eq!(e.mem.u32(map.addr() + 8), 0x25);
        assert_eq!(e.mem.u32(map.addr() + 0x18), 0);
    }

    #[test]
    fn locked_map_lookup_returns_false_when_precheck_fails() {
        let mut e = engine();
        returning(&mut e, 0x0052_9030, 0);
        stub(&mut e, 0x0052_9000);
        assert!(!fn_00528ce0(&mut e, Ptr::NULL, 1, 2));
    }

    #[test]
    fn queue_cell_load_ignores_null_worldspace() {
        exterior_cell_loader_queue_cell_load(&mut engine(), Ptr::NULL, 0, 0, 0);
    }

    #[test]
    fn task_clearing_exits_when_the_iterator_is_empty() {
        let mut e = engine();
        returning(&mut e, 0x006e_bde0, 1);
        e.call_log = Some(vec![]);
        fn_00527fc0(&mut e, Ptr::NULL);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].0, 0x006e_bde0);
        assert_eq!(log[0].1.len(), 1);
    }

    #[test]
    fn update_task_walk_exits_when_iterator_is_empty() {
        let mut e = engine();
        returning(&mut e, 0x006e_bde0, 1);
        fn_00528110(&mut e, Ptr::NULL);
    }

    #[test]
    fn try_cancel_task_walk_exits_when_iterator_is_empty() {
        let mut e = engine();
        returning(&mut e, 0x006e_bde0, 1);
        e.call_log = Some(vec![]);
        assert_eq!(exterior_cell_loader_try_cancel_tasks(&mut e, Ptr::NULL), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].0, 0x006e_bde0);
    }

    #[test]
    fn wait_task_walk_exits_when_iterator_is_empty() {
        let mut e = engine();
        returning(&mut e, 0x006e_bde0, 1);
        e.call_log = Some(vec![]);
        fn_005282d0(&mut e, Ptr::NULL);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].0, 0x006e_bde0);
    }

    /// One task in the map for the walks: the iterator's end test answers
    /// false once, the removal (`00906570`) puts `task` in the walk's pointer
    /// local, `00559450` reads that local, the state test says 5 and the
    /// task's slot `0x14` is the double at `0x0600_0400`.
    fn one_task_walk(e: &mut Engine, task: u32) {
        e.register_double(0x006e_bde0, {
            let mut calls = 0;
            move |_, _| {
                calls += 1;
                Ret {
                    eax: u32::from(calls > 1),
                    ..Ret::default()
                }
            }
        });
        stub(e, 0x0052_8cb0);
        e.register_double(0x0090_6570, move |e, a| {
            e.mem.set_u32(a[3], task);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0055_9450, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        returning(e, 0x0052_87e0, 1);
        stub(e, 0x0044_cbf0);
        let vtable = 0x0600_0300;
        e.put_vtable(vtable, &[0, 0, 0, 0, 0, 0x0600_0400]);
        e.mem.set_u32(task, vtable);
        stub(e, 0x0600_0400);
    }

    #[test]
    fn update_task_walk_reads_the_task_again_before_its_update() {
        // 00528188-005281b4: `00559450` for the state test, again for the call.
        let mut e = engine();
        let task = 0x0600_0100;
        one_task_walk(&mut e, task);
        e.call_log = Some(vec![]);
        fn_00528110(&mut e, Ptr::new(0x0600_0800));
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().map(|(address, _)| *address).collect::<Vec<_>>(),
            [
                0x006e_bde0,
                0x0052_8cb0,
                0x0090_6570,
                0x0055_9450,
                0x0052_87e0,
                0x0055_9450,
                0x0600_0400,
                0x0044_cbf0,
                0x006e_bde0,
            ]
        );
        assert_eq!(log[2].1[0], 0x0600_0800);
        assert_eq!(log[4].1, vec![task]);
        assert_eq!(log[6].1, vec![task]);
    }

    #[test]
    fn wait_task_walk_reads_the_task_for_each_use() {
        // 0052834c-0052838c: `00559450` for the wait, the state test and the call.
        let mut e = engine();
        let task = 0x0600_0100;
        one_task_walk(&mut e, task);
        e.mem.set_u32(task + 0x0c, 4);
        let manager = 0x0600_0200;
        e.mem.set_u32(0x0120_2d98, manager);
        e.call_log = Some(vec![]);
        fn_005282d0(&mut e, Ptr::new(0x0600_0800));
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().map(|(address, _)| *address).collect::<Vec<_>>(),
            [
                0x006e_bde0,
                0x0052_8cb0,
                0x0090_6570,
                0x0055_9450,
                0x0055_9450,
                0x0052_87e0,
                0x0055_9450,
                0x0600_0400,
                0x0044_cbf0,
                0x006e_bde0,
            ]
        );
        assert_eq!(log[5].1, vec![task]);
        assert_eq!(log[7].1, vec![task]);
    }

    #[test]
    fn task_queue_request_is_submitted_to_io_manager() {
        let mut e = engine();
        stub(&mut e, 0x0043_d690);
        fn_00527ce0(&mut e, Ptr::new(0x0600_0100));
    }

    #[test]
    fn loader_constructor_sets_its_map_state() {
        let mut e = engine();
        for address in [0x0044_d7f0, 0x0096_a2d0, 0x0040_4eb0, 0x0040_4ee0] {
            stub(&mut e, address);
        }
        returning(&mut e, 0x0040_1000, 0);
        let loader = Ptr::new(e.mem.alloc(0x40));
        assert_eq!(fn_00527e30(&mut e, loader), loader);
        assert_eq!(e.mem.u32(loader.addr()), 0x0102_d084);
    }

    #[test]
    fn loader_destructor_clears_map_storage() {
        let mut e = engine();
        returning(&mut e, 0x006e_bde0, 1);
        stub(&mut e, 0x0052_8e40);
        stub(&mut e, 0x0040_1030);
        stub(&mut e, 0x0044_b4b0);
        let loader = Ptr::new(e.mem.alloc(0x40));
        fn_00527e50(&mut e, loader);
        assert_eq!(e.mem.u32(loader.addr()), 0x0102_d084);
    }

    #[test]
    fn deleting_loader_runs_destructor_and_returns_same_pointer() {
        let mut e = engine();
        returning(&mut e, 0x006e_bde0, 1);
        stub(&mut e, 0x0052_8e40);
        stub(&mut e, 0x0040_1030);
        stub(&mut e, 0x0044_b4b0);
        let loader = Ptr::new(e.mem.alloc(0x40));
        assert_eq!(fn_00527f90(&mut e, loader, 0), loader);
    }

    #[test]
    fn cancel_all_cell_loads_runs_the_empty_clear_loop() {
        let mut e = engine();
        returning(&mut e, 0x006e_bde0, 1);
        exterior_cell_loader_cancel_all_cell_loads(&mut e, Ptr::NULL);
    }

    #[test]
    fn cancel_all_cell_loads_pops_and_dispatches_a_task() {
        let mut e = engine();
        e.register_double(0x006e_bde0, {
            let mut calls = 0;
            move |_, _| {
                calls += 1;
                Ret {
                    eax: u32::from(calls > 1),
                    ..Ret::default()
                }
            }
        });
        stub(&mut e, 0x0052_8cb0);
        stub(&mut e, 0x0044_ac40);
        let task = 0x0600_0100;
        e.register_double(0x0090_6570, move |e, a| {
            e.mem.set_u32(a[2], task);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        returning(&mut e, 0x0055_9450, task);
        stub(&mut e, 0x0044_cbf0);
        let manager = 0x0600_0200;
        e.mem.set_u32(0x0120_2d98, manager);
        let manager_vtable = 0x0600_0300;
        e.put_vtable(manager_vtable, &[0x0600_0400]);
        e.mem.set_u32(manager, manager_vtable);
        stub(&mut e, 0x0600_0400);
        e.call_log = Some(vec![]);

        exterior_cell_loader_cancel_all_cell_loads(&mut e, Ptr::NULL);

        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().map(|(address, _)| *address).collect::<Vec<_>>(),
            [
                0x006e_bde0,
                0x0052_8cb0,
                0x0090_6570,
                0x0055_9450,
                0x0044_ac40,
                0x0044_cbf0,
                0x006e_bde0,
            ]
        );
        let dispatch = log
            .iter()
            .find(|(address, _)| *address == 0x0044_ac40)
            .unwrap();
        assert_eq!(dispatch.1, vec![manager, task, 0]);
        let pop = log
            .iter()
            .find(|(address, _)| *address == 0x0090_6570)
            .unwrap();
        assert_eq!(pop.1.len(), 5);
        assert_eq!(pop.1[0], 0);
        assert_eq!(pop.1[2], pop.1[3] + 4);
        assert_eq!(pop.1[4], 1);
    }

    #[test]
    fn coordinate_completion_calls_the_loader_vtable() {
        let mut e = engine();
        returning(&mut e, 0x0058_7410, 7);
        let loader = Ptr::new(e.mem.alloc(4));
        let vtable = 0x0600_0300;
        e.put_vtable(vtable, &[0, 0, 0, 0, 0, 0x0600_0400]);
        e.mem.set_u32(loader.addr(), vtable);
        stub(&mut e, 0x0600_0400);
        fn_00528560(&mut e, loader, 11, 12);
    }

    #[test]
    fn virtual_task_cleanup_calls_slot_forty_four() {
        let mut e = engine();
        let object = Ptr::new(e.mem.alloc(4));
        let vtable = 0x0600_0300;
        let mut slots = vec![0; 18];
        slots[17] = 0x0600_0400;
        e.put_vtable(vtable, &slots);
        e.mem.set_u32(object.addr(), vtable);
        stub(&mut e, 0x0600_0400);
        fn_005285a0(&mut e, object);
    }

    #[test]
    fn null_cell_attach_skips_cell_queries() {
        exterior_cell_loader_attach_loaded_cell(&mut engine(), Ptr::NULL, 0);
    }

    #[test]
    fn iterator_deleting_destructor_preserves_pointer_without_delete_flag() {
        let mut e = engine();
        let object = Ptr::new(e.mem.alloc(0x10));
        assert_eq!(fn_00528840(&mut e, object, 0), object);
        assert_eq!(e.mem.u32(object.addr()), 0x0102_d078);
    }

    #[test]
    fn locked_map_guard_releases_lock_and_returns_result() {
        let mut e = engine();
        stub(&mut e, 0x0040_fbf0);
        returning(&mut e, 0x0044_9f80, 0x0600_0100);
        returning(&mut e, 0x0044_c000, 1);
        stub(&mut e, 0x0040_fba0);
        let map = Ptr::new(e.mem.alloc(0x30));
        e.call_log = Some(vec![]);
        assert!(fn_00528b60(&mut e, map, 1, 2, 3));
        let log = e.call_log.take().unwrap();
        // 00449f80 takes only `this` (plain RET); the three pushed words are
        // 0044c000's (RET 0xc), called on 00449f80's result.
        let get = log.iter().find(|(a, _)| *a == 0x0044_9f80).unwrap();
        assert_eq!(get.1, vec![map.addr()]);
        let find = log.iter().find(|(a, _)| *a == 0x0044_c000).unwrap();
        assert_eq!(find.1, vec![0x0600_0100, 1, 2, 3]);
    }

    #[test]
    fn map_insertion_uses_virtual_key_and_stops_on_failed_precheck() {
        let mut e = engine();
        let object = Ptr::<()>::new(e.mem.alloc(4));
        let map = Ptr::<()>::new(e.mem.alloc(4));
        let vtable = 0x0600_0300;
        let mut slots = vec![0; 10];
        slots[9] = 0x0600_0400;
        e.put_vtable(vtable, &slots);
        e.mem.set_u32(object.addr(), map.addr());
        e.mem.set_u32(map.addr(), vtable);
        returning(&mut e, 0x0600_0400, 5);
        returning(&mut e, 0x0052_9030, 0);
        stub(&mut e, 0x0052_9000);
        fn_00528bb0(&mut e, object, 5);
    }

    #[test]
    fn map_insertion_publishes_item_and_clears_its_lock_guard() {
        let mut e = engine();
        let wrapper = Ptr::<()>::new(e.mem.alloc(4));
        let map = Ptr::<()>::new(e.mem.alloc(0x30));
        let map_vtable = 0x0600_0300;
        let mut slots = vec![0; 17];
        slots[9] = 0x0600_0400;
        slots[16] = 0x0600_0500;
        e.put_vtable(map_vtable, &slots);
        e.mem.set_u32(wrapper.addr(), map.addr());
        e.mem.set_u32(map.addr(), map_vtable);
        e.mem.set_u32(wrapper.addr() + 0x10, 0x0c);
        returning(&mut e, 0x0600_0400, 0x0600_0600);
        returning(&mut e, 0x0052_9030, 1);
        returning(&mut e, 0x0044_dec0, 0x0600_0700);
        e.register_double(0x006e_cba0, {
            let mut next = 0;
            move |e, a| {
                let value = [0x11, 0x22, 0x33][next];
                next += 1;
                e.mem.set_u32(a[0], value);
                Ret::default()
            }
        });
        e.register(0x0055_9450, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        returning(&mut e, 0x0044_df00, 0x0600_0800);
        returning(&mut e, 0x0044_91c0, 1);
        returning(&mut e, 0x0052_92a0, 0);
        stub(&mut e, 0x0600_0500);
        stub(&mut e, 0x0052_9000);
        e.call_log = Some(vec![]);

        fn_00528bb0(&mut e, wrapper, 0x55);

        let log = e.call_log.take().unwrap();
        let locks = log
            .iter()
            .filter(|(address, _)| *address == 0x0044_91c0)
            .collect::<Vec<_>>();
        assert_eq!(locks.len(), 2);
        assert_eq!(locks[0].1, vec![0x0600_0808, 0x22, 0x11]);
        assert_eq!(locks[1].1, vec![0x0c, 0x11, 0x33]);
        assert_eq!(
            log.iter().map(|(address, _)| *address).collect::<Vec<_>>(),
            [
                0x0600_0400,
                0x0052_9030,
                0x0044_dec0,
                0x006e_cba0,
                0x0044_dec0,
                0x006e_cba0,
                0x0055_9450,
                0x0055_9450,
                0x0044_df00,
                0x0044_91c0,
                0x0044_dec0,
                0x006e_cba0,
                0x0055_9450,
                0x0055_9450,
                0x0044_91c0,
                0x0044_dec0,
                0x0052_92a0,
                0x0600_0500,
                0x0052_9000,
            ]
        );
        let publish = log
            .iter()
            .find(|(address, _)| *address == 0x0052_92a0)
            .unwrap();
        assert_eq!(publish.1, vec![wrapper.addr(), 0x0600_0700]);
    }

    #[test]
    fn concurrent_map_destructor_frees_empty_storage() {
        let mut e = engine();
        stub(&mut e, 0x0052_8e40);
        stub(&mut e, 0x0040_1030);
        stub(&mut e, 0x0044_b4b0);
        let map = Ptr::new(e.mem.alloc(0x30));
        fn_00528be0(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), 0x0102_d084);
    }

    #[test]
    fn iterator_constructor_sets_empty_fields() {
        let mut e = engine();
        let iterator = Ptr::new(e.mem.alloc(0x10));
        fn_00528c70(&mut e, iterator);
        assert_eq!(e.mem.u32(iterator.addr()), 0x0102_d078);
        assert_eq!(e.mem.u32(iterator.addr() + 4), 0);
        assert_eq!(e.mem.u32(iterator.addr() + 8), 0);
        assert_eq!(e.mem.u8(iterator.addr() + 0x0c), 0);
    }

    #[test]
    fn cancellation_walk_counts_a_task_that_refuses_cancel() {
        let mut e = engine();
        e.register_double(0x006e_bde0, {
            let mut count = 0;
            move |_, _| {
                count += 1;
                Ret {
                    eax: u32::from(count > 1),
                    ..Ret::default()
                }
            }
        });
        stub(&mut e, 0x0052_8cb0);
        returning(&mut e, 0x0090_6570, 1);
        returning(&mut e, 0x0055_9450, 0x0600_0100);
        stub(&mut e, 0x0044_cbf0);
        let task = Ptr::<()>::new(0x0600_0100);
        e.mem.set_u32(task.addr() + 0x0c, 3);
        e.call_log = Some(vec![]);
        assert_eq!(exterior_cell_loader_try_cancel_tasks(&mut e, Ptr::NULL), 1);
        let log = e.call_log.take().unwrap();
        let pop = log.iter().find(|(a, _)| *a == 0x0090_6570).unwrap();
        assert_eq!(pop.1.len(), 5);
        assert_eq!(pop.1[0], 0);
        assert_eq!(pop.1[2], pop.1[3] + 4); // result and iterator locals are adjacent
        assert_eq!(pop.1[4], 1);
        assert_eq!(
            log.iter().map(|(a, _)| *a).collect::<Vec<_>>(),
            [
                0x006e_bde0,
                0x0052_8cb0,
                0x0090_6570,
                0x0055_9450,
                0x0044_cbf0,
                0x006e_bde0,
            ]
        );
    }

    #[test]
    fn cancellation_state_zero_cas_runs_the_task_callback() {
        let mut e = engine();
        returning(&mut e, 0x0044_9190, 1);
        let task = Ptr::<()>::new(e.mem.alloc(0x10));
        e.mem.set_u32(task.addr() + 0x0c, 0);
        let vtable = 0x0600_0300;
        e.put_vtable(vtable, &[0, 0, 0, 0x0600_0400]);
        e.mem.set_u32(task.addr(), vtable);
        stub(&mut e, 0x0600_0400);
        e.call_log = Some(vec![]);
        assert!(fn_00528870(&mut e, Ptr::NULL, task, 0x55));
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[0].0, 0x0044_9190);
        assert_eq!(log[0].1, vec![task.addr(), 0, 6]);
        assert_eq!(log[1].0, 0x0600_0400);
        assert_eq!(log[1].1, vec![task.addr(), 0, 0x55]);
    }

    #[test]
    fn cancellation_state_one_locks_the_manager_around_callback() {
        let mut e = engine();
        returning(&mut e, 0x0044_9190, 1);
        for address in [0x0092_c870, 0x0044_e050, 0x0044_dd60] {
            stub(&mut e, address);
        }
        let task = Ptr::<()>::new(e.mem.alloc(0x10));
        e.mem.set_u32(task.addr() + 0x0c, 1);
        let vtable = 0x0600_0300;
        e.put_vtable(vtable, &[0, 0, 0, 0x0600_0400]);
        e.mem.set_u32(task.addr(), vtable);
        stub(&mut e, 0x0600_0400);
        let manager = Ptr::<()>::new(0x0600_0100);
        e.call_log = Some(vec![]);
        assert!(fn_00528870(&mut e, manager, task, 0x55));
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().map(|(a, _)| *a).collect::<Vec<_>>(),
            [
                0x0044_9190,
                0x0092_c870,
                0x0044_e050,
                0x0600_0400,
                0x0044_dd60
            ]
        );
        assert_eq!(log[0].1, vec![task.addr(), 1, 6]);
        assert_eq!(log[2].1, vec![manager.addr(), task.addr()]);
        assert_eq!(log[3].1, vec![task.addr(), 1, 0x55]);
        assert_eq!(log[4].1, vec![task.addr()]);
    }

    #[test]
    /// The loader is the map itself (005280d6: slot 8 is called on its `this`).
    fn coordinate_task_lookup_calls_the_map_lookup_slot() {
        let mut e = engine();
        returning(&mut e, 0x0058_7410, 0x1234);
        let loader = Ptr::<()>::new(e.mem.alloc(4));
        let vtable = 0x0600_0300;
        let mut slots = vec![0; 3];
        slots[2] = 0x0600_0400;
        e.put_vtable(vtable, &slots);
        e.mem.set_u32(loader.addr(), vtable);
        returning(&mut e, 0x0600_0400, 1);
        stub(&mut e, 0x0052_8cb0);
        stub(&mut e, 0x0044_cbf0);
        e.call_log = Some(vec![]);
        assert!(fn_00528080(&mut e, loader, 17, 19));
        let log = e.call_log.take().unwrap();
        assert_eq!(log[0].0, 0x0058_7410);
        assert_eq!(log[0].1, vec![17, 19]);
        let lookup = log.iter().find(|(a, _)| *a == 0x0600_0400).unwrap();
        assert_eq!(lookup.1[0], loader.addr());
        assert_eq!(lookup.1[1], 0x1234);
    }

    /// 005284b6..0052851e: the queued task goes through a smart pointer: the map (the loader
    /// itself, slot 0x10) gets the key, the pointer's address and 1; the manager (slot 0x48)
    /// gets the task the pointer holds; then the pointer is let go.
    #[test]
    fn queue_cell_load_hands_the_task_through_a_smart_pointer() {
        let mut e = engine();
        returning(&mut e, 0x0058_75a0, 0);
        returning(&mut e, 0x0058_7410, 0x1234);
        // operator new: the coordinate data (0x14 bytes, 005285c0), then the task (0x20).
        e.register_double(0x0040_1000, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        stub(&mut e, 0x0044_0540);
        let loader = Ptr::<()>::new(e.mem.alloc(0x10));
        let vtable = 0x0600_0300;
        let mut slots = vec![0; 5];
        slots[2] = 0x0600_0400;
        slots[4] = 0x0600_0500;
        e.put_vtable(vtable, &slots);
        e.mem.set_u32(loader.addr(), vtable);
        returning(&mut e, 0x0600_0400, 0);
        returning(&mut e, 0x0600_0500, 1);
        let manager = e.mem.alloc(4);
        let manager_vtable = 0x0600_0600;
        let mut manager_slots = vec![0; 0x13];
        manager_slots[0x12] = 0x0600_0700;
        e.put_vtable(manager_vtable, &manager_slots);
        e.mem.set_u32(manager, manager_vtable);
        e.mem.set_u32(0x0120_2d98, manager);
        stub(&mut e, 0x0600_0700);
        e.register_double(0x0052_8cb0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(0x0055_9450, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        stub(&mut e, 0x0044_cbf0);
        e.call_log = Some(vec![]);
        exterior_cell_loader_queue_cell_load(&mut e, loader, 0x0600_0100, 7, 8);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().map(|(a, w)| (*a, w.clone())).collect();
        // The coordinate data, then the task.
        let sizes: Vec<_> = calls
            .iter()
            .filter(|(a, _)| *a == 0x0040_1000)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(sizes, vec![0x14, 0x20]);
        // The pointer holds the task (00527c00 stored the task vtable in it).
        let hold = calls
            .iter()
            .rfind(|(a, _)| *a == 0x0052_8cb0)
            .unwrap()
            .1
            .clone();
        let (holder, task_block) = (hold[0], hold[1]);
        assert_eq!(e.mem.u32(task_block), TASK_VTABLE);
        let set = calls.iter().find(|(a, _)| *a == 0x0600_0500).unwrap();
        assert_eq!(set.1, vec![loader.addr(), 0x1234, holder, 1]);
        let queued = calls.iter().find(|(a, _)| *a == 0x0600_0700).unwrap();
        assert_eq!(queued.1, vec![manager, task_block]);
        let released = calls.iter().rfind(|(a, _)| *a == 0x0044_cbf0).unwrap();
        assert_eq!(released.1, vec![holder]);
    }

    #[test]
    fn task_run_loads_data_before_queuing_references() {
        let mut e = engine();
        returning(&mut e, 0x0054_5cf0, 0);
        returning(&mut e, 0x0054_6fb0, 0);
        returning(&mut e, 0x0045_1cd0, 1);
        stub(&mut e, 0x0055_71a0);
        let task = Ptr::<()>::new(e.mem.alloc(0x20));
        let data = Ptr::<()>::new(e.mem.alloc(0x14));
        let cell = Ptr::<()>::new(e.mem.alloc(0x20));
        e.mem.set_u32(data.addr() + 0x0c, cell.addr());
        e.mem.set_u32(cell.addr() + 0x0c, 0x0600_0500);
        e.mem.set_u32(task.addr() + 0x18, 0x0600_0600);
        e.mem.set_u32(task.addr() + 0x1c, data.addr());
        e.call_log = Some(vec![]);
        exterior_cell_loader_task_run(&mut e, task);
        assert_eq!(e.mem.u8(data.addr() + 0x10), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().map(|(a, _)| *a).collect::<Vec<_>>(),
            [0x0054_5cf0, 0x0054_6fb0, 0x0045_1cd0, 0x0055_71a0]
        );
        // 00527ccb-00527cd4: QueueReferences is called on the cell, not on its +0xc word.
        assert_eq!(log[3].1, vec![cell.addr(), 1]);
    }

    /// 00528731: the water setting's byte is read through 00408d60's answer with no null
    /// test, so a null answer faults on the read as in the exe.
    #[test]
    #[should_panic(expected = "read of unmapped memory at 00000000")]
    fn cell_data_load_reads_the_water_setting_byte_unchecked() {
        let mut e = engine();
        returning(&mut e, 0x0054_5cf0, 0);
        returning(&mut e, 0x0054_6fb0, 0);
        returning(&mut e, 0x0045_1cd0, 0);
        stub(&mut e, 0x0053_f440);
        stub(&mut e, 0x0054_71e0);
        stub(&mut e, 0x0045_1250);
        returning(&mut e, 0x0040_8d60, 0);
        let data = Ptr::<()>::new(e.mem.alloc(0x14));
        e.mem.set_u32(data.addr() + 0x0c, 0x0600_0500);
        fn_00528620(&mut e, Ptr::NULL, data);
    }

    /// 00528487-0052851e: a failed task allocation gives a null task, which still goes
    /// through the smart pointer, the map (slot 0x10) and the manager (slot 0x48).
    #[test]
    fn queue_cell_load_hands_on_a_null_task_when_its_allocation_fails() {
        let mut e = engine();
        returning(&mut e, 0x0058_75a0, 0);
        returning(&mut e, 0x0058_7410, 0x1234);
        // operator new: the coordinate data (0x14 bytes) is given, the task (0x20) is not.
        e.register_double(0x0040_1000, |e, a| Ret {
            eax: if a[0] == 0x20 { 0 } else { e.mem.alloc(a[0]) },
            ..Ret::default()
        });
        stub(&mut e, 0x0044_0540);
        let loader = Ptr::<()>::new(e.mem.alloc(0x10));
        let vtable = 0x0600_0300;
        let mut slots = vec![0; 5];
        slots[2] = 0x0600_0400;
        slots[4] = 0x0600_0500;
        e.put_vtable(vtable, &slots);
        e.mem.set_u32(loader.addr(), vtable);
        returning(&mut e, 0x0600_0400, 0);
        returning(&mut e, 0x0600_0500, 1);
        let manager = e.mem.alloc(4);
        let manager_vtable = 0x0600_0600;
        let mut manager_slots = vec![0; 0x13];
        manager_slots[0x12] = 0x0600_0700;
        e.put_vtable(manager_vtable, &manager_slots);
        e.mem.set_u32(manager, manager_vtable);
        e.mem.set_u32(0x0120_2d98, manager);
        stub(&mut e, 0x0600_0700);
        e.register_double(0x0052_8cb0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register_double(0x0055_9450, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        stub(&mut e, 0x0044_cbf0);
        e.call_log = Some(vec![]);
        exterior_cell_loader_queue_cell_load(&mut e, loader, 0x0600_0100, 7, 8);
        let log = e.call_log.take().unwrap();
        // No task was constructed.
        assert!(!log.iter().any(|(a, _)| *a == 0x0044_0540));
        let hold = log.iter().rfind(|(a, _)| *a == 0x0052_8cb0).unwrap();
        let holder = hold.1[0];
        assert_eq!(hold.1[1], 0);
        let set = log.iter().find(|(a, _)| *a == 0x0600_0500).unwrap();
        assert_eq!(set.1, vec![loader.addr(), 0x1234, holder, 1]);
        let queued = log.iter().find(|(a, _)| *a == 0x0600_0700).unwrap();
        assert_eq!(queued.1, vec![manager, 0]);
        let released = log.iter().rfind(|(a, _)| *a == 0x0044_cbf0).unwrap();
        assert_eq!(released.1, vec![holder]);
    }

    #[test]
    fn virtual_slot_forty_four_answer_is_passed_back() {
        let mut e = engine();
        let object = Ptr::new(e.mem.alloc(4));
        let vtable = 0x0600_0300;
        let mut slots = vec![0; 18];
        slots[17] = 0x0600_0400;
        e.put_vtable(vtable, &slots);
        e.mem.set_u32(object.addr(), vtable);
        returning(&mut e, 0x0600_0400, 9);
        assert_eq!(fn_005285a0(&mut e, object), 9);
    }

    #[test]
    fn map_insertion_answers_the_helper_and_gives_the_virtual_one_word() {
        let mut e = engine();
        let object = Ptr::<()>::new(e.mem.alloc(4));
        let map = Ptr::<()>::new(e.mem.alloc(4));
        let vtable = 0x0600_0300;
        let mut slots = vec![0; 10];
        slots[9] = 0x0600_0400;
        e.put_vtable(vtable, &slots);
        e.mem.set_u32(object.addr(), map.addr());
        e.mem.set_u32(map.addr(), vtable);
        returning(&mut e, 0x0600_0400, 5);
        returning(&mut e, 0x0052_9030, 0);
        stub(&mut e, 0x0052_9000);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0052_8bb0, &args![object, 7u32]).bool());
        let log = e.call_log.take().unwrap();
        let key = log.iter().find(|(a, _)| *a == 0x0600_0400).unwrap();
        assert_eq!(key.1, vec![map.addr(), 7]);
        let insert = log.iter().find(|(a, _)| *a == 0x0052_9030).unwrap();
        assert_eq!(insert.1, vec![object.addr(), 5, 7]);
    }
}
