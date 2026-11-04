//! FalloutNV.exe 1.4.0.525 functions from `fallout shared/pathfinding/pathmanager.cpp`.

#[allow(unused_imports)]
use crate::prelude::*;

const PATHING_TASK_DATA_VTABLE: u32 = 0x0106_d550;
const PATH_MANAGER_IMPL_STATE: u32 = 0x0c24;
const PATH_MANAGER_IMPL_PAUSE_RETRIES: u32 = 0x0c28;
const PATH_MANAGER_IMPL_CANCEL_PATHS: u32 = 0x0c34;

// Translated from 006e9bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPathingMessage::ActorPathingMessage` (Xbox PDB): initializes its
/// message type and empty solution pointer.
pub fn actor_pathing_message_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), u32::MAX);
    e.call(0x0063_3c90, &args![this.addr() + 4, 0u32]);
    this
}

// Translated from 006e9c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a message from a type and solution pointer.
pub fn fn_006e9c00(e: &mut Engine, this: Ptr, message_type: u32, solution: u32) -> Ptr {
    e.mem.set_u32(this.addr(), message_type);
    e.call(0x0055_9a40, &args![this.addr() + 4, solution]);
    this
}

// Translated from 006e9c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the type and reference-counted pointer in a small pathing closure.
pub fn fn_006e9c30(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), e.mem.u32(source.addr()));
    e.call(0x0055_9a40, &args![this.addr() + 4, source.addr() + 4]);
    this
}

// Translated from 006e9c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the type and `NiPointer<PathingDebugGeometryData>` member.
pub fn fn_006e9c60(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), e.mem.u32(source.addr()));
    e.call(0x006e_5cc0, &args![this.addr() + 4, source.addr() + 4]);
    this
}

// Translated from 006e9c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the data then frees it only when the scalar delete flag is set.
pub fn pathing_task_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006e9e20(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006e9cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e9cc0 - compiler-generated exception cleanup remains unimplemented
// open: exceptional exit cleanup of constructed members and by-value token is omitted.
/// Constructs the pathing task data object. PC code initializes its embedded
/// arrays at `+0x40`, `+0x50`, and quick-bit storage at `+0x60`.
pub fn pathing_task_data_construct(e: &mut Engine, this: Ptr, request: u32, source: u32) -> Ptr {
    e.call(0x006c_7850, &args![this]);
    e.mem.set_u32(this.addr(), PATHING_TASK_DATA_VTABLE);
    e.call(0x0063_3c90, &args![this.addr() + 0x18, 0u32]);
    e.call(0x0063_3c90, &args![this.addr() + 0x1c, 0u32]);
    e.call(0x0096_a2d0, &args![this.addr() + 0x20]);
    e.call(0x006e_be90, &args![this.addr() + 0x40]);
    e.call(0x006e_bee0, &args![this.addr() + 0x50]);
    e.call(0x006e_bf60, &args![this.addr() + 0x60, 0u32]);
    e.mem.set_u8(this.addr() + 0x64, 0);
    e.with_stack(8, |e, temporaries| {
        e.mem.set_u32(temporaries.addr() + 4, source);
        e.call(
            0x006e_bf30,
            &args![temporaries, Ptr::<()>::new(temporaries.addr() + 4)],
        );
        fn_006e9db0(e, this, Ptr::new(request), e.mem.u32(temporaries.addr()));
        e.call(0x006e_bf90, &args![Ptr::<()>::new(temporaries.addr() + 4)]);
    });
    this
}

// Translated from 006e9db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e9db0 - compiler-generated exception cleanup remains unimplemented
// open: exceptional exit cleanup of constructed members and by-value token is omitted.
/// Assigns the request pointer and copies the cancellation source into the
/// task data's embedded state.
pub fn fn_006e9db0(e: &mut Engine, this: Ptr, request: Ptr, source: u32) {
    e.call(0x006e_5cc0, &args![this.addr() + 0x18, request]);
    e.with_stack(4, |e, source_slot| {
        e.mem.set_u32(source_slot.addr(), source);
        e.call(0x006e_bfb0, &args![this.addr() + 0x60, source_slot]);
        e.call(0x006e_bf90, &args![source_slot]);
    });
}

// Translated from 006e9e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases each non-null pathing request and ray-cast result, then destroys
/// the PC object's embedded arrays and synchronization state.
pub fn fn_006e9e20(e: &mut Engine, this: Ptr) {
    let count = e.call(0x0044_ddc0, &args![this.addr() + 0x40]).u32();
    for index in 0..count {
        let request = e.call(0x0087_7a30, &args![this.addr() + 0x40, index]).u32();
        if request != 0 {
            pathing_task_data_request_deleting_destructor(e, Ptr::new(request), 1);
        }
        let path = e.call(0x0087_7a30, &args![this.addr() + 0x50, index]).u32();
        if path != 0 {
            pathing_task_data_path_deleting_destructor(e, Ptr::new(path), 1);
        }
    }
    e.call(0x006e_bf90, &args![this.addr() + 0x60]);
    e.call(0x006e_bf10, &args![this.addr() + 0x50]);
    e.call(0x006e_bec0, &args![this.addr() + 0x40]);
    e.call(0x0045_cec0, &args![this.addr() + 0x1c]);
    e.call(0x0045_cec0, &args![this.addr() + 0x18]);
    e.call(0x00b0_0ec0, &args![this]);
}

// Translated from 006e9f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys one path request and frees it when the scalar delete bit is set.
pub fn pathing_task_data_request_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006c_b410, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006e9f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys one ray-cast path and frees it when the scalar delete bit is set.
pub fn pathing_task_data_path_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006c_b4a0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006e9fc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e9fc0 - compiler-generated SEH cleanup remains unimplemented
/// Processes a pathing request and sends the resulting success, failure, or
/// cancellation message to its owning actor and the manager's message queue.
/// The compiler-generated SEH cleanup state is omitted.
pub fn pathing_task_data_process(e: &mut Engine, this: Ptr) {
    e.with_stack(0x10, |e, logging| {
        e.call(
            0x0040_4eb0,
            &args![logging, 0x2du32, 1u32, 0x0106_d560u32, 0x163u32],
        );
        let category = e.call(0x0047_d0b0, &args![]).u32();
        let category = e.call(0x0071_7e50, &args![category]).u32();
        e.call(0x0048_3710, &args![category]);
        let category = e.call(0x0047_d0b0, &args![]).u32();
        let category = e.call(0x0055_9450, &args![category]).u32();
        let was_signaled = fn_006eb790(e, Ptr::new(category));

        if was_signaled {
            if !fn_006ea490(e, this) {
                fn_006ea330(e, this);
            } else {
                let allocation = e.call(0x00aa_13e0, &args![0x44u32]).u32();
                let token = if allocation == 0 {
                    0
                } else {
                    e.call(0x006e_7650, &args![allocation]).u32()
                };
                e.call(0x0066_b0d0, &args![this.addr() + 0x1c, token]);
                let request_owner = e.call(0x0055_9450, &args![this.addr() + 0x1c]).u32();
                let request = e.call(0x0055_9450, &args![this.addr() + 0x18]).u32();
                let request_finished = e.call(0x006d_0900, &args![request, request_owner]).bool();
                if !e.call(0x005d_c980, &args![this]).bool() {
                    if request_finished {
                        e.mem.set_u8(this.addr() + 4, 0);
                        let queue = e.call(0x0055_9450, &args![this.addr() + 0x60]).u32();
                        e.with_stack(8, |e, message| {
                            fn_006e9c00(e, message, 0, this.addr() + 0x1c);
                            e.vcall(queue, 4, &args![message]);
                            e.call(0x0080_0190, &args![message]);
                        });
                        e.with_stack(8, |e, message| {
                            let queued = e.call(0x004b_0680, &args![message, 0u32, this]).u32();
                            let category = e.call(0x0047_d0b0, &args![]).u32();
                            let queue = e.call(0x0055_9450, &args![category]).u32();
                            fn_006eb7f0(e, Ptr::new(queue), queued);
                            e.call(0x0048_3710, &args![message]);
                        });
                    } else {
                        let queue = e.call(0x0055_9450, &args![this.addr() + 0x60]).u32();
                        e.with_stack(8, |e, message| {
                            fn_006e9c00(e, message, 1, this.addr() + 0x1c);
                            e.vcall(queue, 4, &args![message]);
                            e.call(0x0080_0190, &args![message]);
                        });
                        e.with_stack(8, |e, message| {
                            let queued = e.call(0x004b_0680, &args![message, 1u32, this]).u32();
                            let category = e.call(0x0047_d0b0, &args![]).u32();
                            let queue = e.call(0x0055_9450, &args![category]).u32();
                            fn_006eb7f0(e, Ptr::new(queue), queued);
                            e.call(0x0048_3710, &args![message]);
                        });
                    }
                }
            }
            let category = e.call(0x0047_d0b0, &args![]).u32();
            let category = e.call(0x0055_9450, &args![category]).u32();
            fn_006eb7b0(e, Ptr::new(category));
        } else {
            let category = e.call(0x0047_d0b0, &args![]).u32();
            let category = e.call(0x0055_9450, &args![category]).u32();
            if fn_006eb7d0(e, Ptr::new(category)) {
                let queue = e.call(0x0055_9450, &args![this.addr() + 0x60]).u32();
                e.with_stack(8, |e, message| {
                    fn_006e9c00(e, message, 1, this.addr() + 0x1c);
                    e.vcall(queue, 4, &args![message]);
                    e.call(0x0080_0190, &args![message]);
                });
                e.with_stack(8, |e, message| {
                    let queued = e.call(0x004b_0680, &args![message, 1u32, this]).u32();
                    let category = e.call(0x0047_d0b0, &args![]).u32();
                    let queue = e.call(0x0055_9450, &args![category]).u32();
                    fn_006eb7f0(e, Ptr::new(queue), queued);
                    e.call(0x0048_3710, &args![message]);
                });
            } else {
                fn_006ea330(e, this);
            }
        }

        let category = e.call(0x0047_d0b0, &args![]).u32();
        let category = e.call(0x0071_7e50, &args![category]).u32();
        e.call(0x0048_3710, &args![category]);
        e.call(0x0040_4ee0, &args![logging]);
    });
}

// Translated from 006ea330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the path-data yield flag.
pub fn fn_006ea330(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 4, 1);
}

// Translated from 006ea350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds pending ray-cast paths until the queue is exhausted or the world
/// space budget exceeds ten.
pub fn fn_006ea350(e: &mut Engine, this: Ptr) -> i32 {
    let mut built = 0i32;
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    let count = e.call(0x0044_ddc0, &args![this.addr() + 0x40]).u32();
    // The ray cast object 006e61a0 builds is 0x94 bytes (its last float at +0x90); the exe
    // keeps it at EBP-0xac (`EBP + 0xffffff54`) of a `SUB ESP,0xa4` frame.
    e.with_stack(0x94, |e, ray_cast| {
        for index in 0..count {
            let request_slot = e.call(0x0087_7a30, &args![this.addr() + 0x40, index]).u32();
            let request = e.mem.u32(request_slot);
            if e.mem.u32(request + 4) != 0 {
                continue;
            }
            e.call(0x006e_61a0, &args![ray_cast]);
            let path_slot = e.call(0x0087_7a30, &args![this.addr() + 0x50, index]).u32();
            let path = e.mem.u32(path_slot);
            // 006ea410: the request is read again (`00877a30`, RET 4, takes the pushed
            // index); `006e6320` (RET 8) takes the request and the path.
            let request_slot = e.call(0x0087_7a30, &args![this.addr() + 0x40, index]).u32();
            let request = e.mem.u32(request_slot);
            e.call(0x006e_6320, &args![ray_cast, request, path]);
            built += e.call(0x004f_d3e0, &args![ray_cast]).i32();
            e.call(0x006e_62b0, &args![ray_cast]);
            if built > 10 {
                break;
            }
        }
    });
    e.call(0x0040_fba0, &args![this.addr() + 0x20]);
    built
}

// Translated from 006ea490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns false if any current pathing request has no result yet.
pub fn fn_006ea490(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    let count = e.call(0x0044_ddc0, &args![this.addr() + 0x50]).u32();
    for index in 0..count {
        let path_slot = e.call(0x0087_7a30, &args![this.addr() + 0x50, index]).u32();
        let path = e.mem.u32(path_slot);
        if e.mem.u32(path + 4) == 0 {
            e.call(0x0040_fba0, &args![this.addr() + 0x20]);
            return false;
        }
    }
    e.call(0x0040_fba0, &args![this.addr() + 0x20]);
    true
}

// Translated from 006ea510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ea510 - compiler-generated allocation exception cleanup remains unimplemented
// open: allocation-construction exception handlers are omitted.
/// Finds a matching ray-cast request or appends a new request and result slot.
/// The executable's tolerance is read from its float constant at `0x0101e2bc`.
pub fn fn_006ea510(e: &mut Engine, this: Ptr, start: Ptr, goal: Ptr, result_out: u32) -> i32 {
    let mut result = 0i32;
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    let mut next_index = 0u32;
    while next_index < e.call(0x0044_ddc0, &args![this.addr() + 0x50]).u32() {
        let index = next_index;
        next_index = next_index.wrapping_add(1);
        let path_slot = e.call(0x0087_7a30, &args![this.addr() + 0x50, index]).u32();
        let path = e.mem.u32(path_slot);
        if e.mem.u32(path) != 2 {
            continue;
        }
        let request_slot = e.call(0x0087_7a30, &args![this.addr() + 0x40, index]).u32();
        let request = e.mem.u32(request_slot);
        let tolerance = e.global::<f32>(0x0101_e2bc);
        if !e
            .call(0x006d_eeb0, &args![request + 8, start, tolerance])
            .bool()
        {
            continue;
        }
        let tolerance = e.global::<f32>(0x0101_e2bc);
        let request_slot = e.call(0x0087_7a30, &args![this.addr() + 0x40, index]).u32();
        let request = e.mem.u32(request_slot);
        if !e
            .call(0x006d_eeb0, &args![request + 0x30, goal, tolerance])
            .bool()
        {
            continue;
        }
        let path_slot = e.call(0x0087_7a30, &args![this.addr() + 0x50, index]).u32();
        result = e.mem.u32(e.mem.u32(path_slot) + 4) as i32;
        match result {
            0 => fn_006ea330(e, this),
            1 => {
                let result_slot = e.call(0x0087_7a30, &args![this.addr() + 0x50, index]).u32();
                // The earlier PUSH of result_out survives the array accessor.
                e.call(0x006c_f940, &args![e.mem.u32(result_slot) + 8, result_out]);
            }
            2 => {}
            _ => result = 2,
        }
        e.call(0x0040_fba0, &args![this.addr() + 0x20]);
        return result;
    }

    let request_storage = e.call(0x0040_1000, &args![0x68u32]).u32();
    let request = if request_storage == 0 {
        0
    } else {
        e.call(0x006c_b3a0, &args![request_storage]).u32()
    };
    let request_pointer = Ptr::<()>::new(request);
    let request_owner = e.call(0x0055_9450, &args![this.addr() + 0x18]).u32();
    e.mem.set_u32(request + 4, request_owner);
    e.call(0x006d_cce0, &args![request + 8, start]);
    e.call(0x006d_cce0, &args![request + 0x30, goal]);
    e.mem.set_u32(request, 2);
    let path_storage = e.call(0x0040_1000, &args![0x18u32]).u32();
    let path = if path_storage == 0 {
        0
    } else {
        e.call(0x006c_b480, &args![path_storage]).u32()
    };
    e.mem.set_u32(path, e.mem.u32(request));
    e.mem.set_u32(path + 4, 0);
    e.with_stack(8, |e, holders| {
        e.mem.set_u32(holders.addr(), request_pointer.addr());
        e.mem.set_u32(holders.addr() + 4, path);
        e.call(0x007c_b2e0, &args![this.addr() + 0x40, holders.addr()]);
        e.call(0x007c_b2e0, &args![this.addr() + 0x50, holders.addr() + 4]);
    });
    e.call(0x0040_fba0, &args![this.addr() + 0x20]);
    e.call(0x0060_d4b0, &args![this, 1u32]);
    result
}

// Translated from 006ea780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ea780 - compiler-generated allocation exception cleanup remains unimplemented
// open: allocation-construction exception handlers are omitted.
/// Finds a ray-cast request by its navmesh mode and goal, or appends a new
/// request and path result. The mode is stored as `!navmesh_mode` by the PC
/// executable.
pub fn fn_006ea780(
    e: &mut Engine,
    this: Ptr,
    start: Ptr,
    goal: Ptr,
    result_out: u32,
    navmesh_mode: u8,
) -> i32 {
    let mut result = 0i32;
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    let path_kind = u32::from(navmesh_mode == 0);
    let mut next_index = 0u32;
    while next_index < e.call(0x0044_ddc0, &args![this.addr() + 0x50]).u32() {
        let index = next_index;
        next_index = next_index.wrapping_add(1);
        let path_slot = e.call(0x0087_7a30, &args![this.addr() + 0x50, index]).u32();
        let path = e.mem.u32(path_slot);
        if e.mem.u32(path) != path_kind {
            continue;
        }
        let request_slot = e.call(0x0087_7a30, &args![this.addr() + 0x40, index]).u32();
        let request = e.mem.u32(request_slot);
        let tolerance = e.global::<f32>(0x0101_e2bc);
        if !e
            .call(0x006d_eeb0, &args![request + 0x30, goal, tolerance])
            .bool()
        {
            continue;
        }
        let path_slot = e.call(0x0087_7a30, &args![this.addr() + 0x50, index]).u32();
        result = e.mem.u32(e.mem.u32(path_slot) + 4) as i32;
        match result {
            0 => fn_006ea330(e, this),
            1 => {
                let result_slot = e.call(0x0087_7a30, &args![this.addr() + 0x50, index]).u32();
                // The earlier PUSH of result_out survives the array accessor.
                e.call(0x006c_f940, &args![e.mem.u32(result_slot) + 8, result_out]);
            }
            2 => {}
            _ => result = 2,
        }
        e.call(0x0040_fba0, &args![this.addr() + 0x20]);
        return result;
    }

    let request_storage = e.call(0x0040_1000, &args![0x68u32]).u32();
    let request = if request_storage == 0 {
        0
    } else {
        e.call(0x006c_b3a0, &args![request_storage]).u32()
    };
    let request_pointer = Ptr::<()>::new(request);
    let request_owner = e.call(0x0055_9450, &args![this.addr() + 0x18]).u32();
    e.mem.set_u32(request + 4, request_owner);
    e.call(0x006d_cce0, &args![request + 0x30, goal]);
    e.call(0x006c_f8a0, &args![start, request + 0x58]);
    e.mem.set_u32(request, path_kind);
    let path_storage = e.call(0x0040_1000, &args![0x18u32]).u32();
    let path = if path_storage == 0 {
        0
    } else {
        e.call(0x006c_b480, &args![path_storage]).u32()
    };
    e.mem.set_u32(path, e.mem.u32(request));
    e.mem.set_u32(path + 4, 0);
    e.with_stack(8, |e, holders| {
        e.mem.set_u32(holders.addr(), request_pointer.addr());
        e.mem.set_u32(holders.addr() + 4, path);
        e.call(0x007c_b2e0, &args![this.addr() + 0x40, holders.addr()]);
        e.call(0x007c_b2e0, &args![this.addr() + 0x50, holders.addr() + 4]);
    });
    e.call(0x0040_fba0, &args![this.addr() + 0x20]);
    e.call(0x0060_d4b0, &args![this, 1u32]);
    result
}

// Translated from 006ea9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the two-word completion result used by the path-manager worker.
pub fn fn_006ea9d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), u32::MAX);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 006eaa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies both words of the path-manager completion result.
pub fn fn_006eaa00(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), e.mem.u32(source.addr()));
    e.mem.set_u32(this.addr() + 4, e.mem.u32(source.addr() + 4));
    this
}

// Translated from 006eaa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs `PathManagerImpl`, its queues, locks and request map. The state
/// offsets match the PC disassembly and `PathManagerImpl` (Xbox PDB).
pub fn path_manager_impl_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0044_dee0, &args![this]);
    e.call(0x006e_c000, &args![this.addr() + 4]);
    e.call(0x006e_c090, &args![this.addr() + 0x0c10, this.addr() + 4]);
    e.call(0x00aa_5fc0, &args![this.addr() + 0x0c2c]);
    e.call(0x0044_fae0, &args![this.addr() + 0x0c38]);
    e.call(0x006e_c340, &args![this.addr() + 0x0c50]);
    e.call(0x006e_c340, &args![this.addr() + 0x1064]);
    e.call(
        0x006e_c660,
        &args![this.addr() + 0x1480, 0x0cu32, 0x25u32, 0x0cu32],
    );
    e.mem.set_u32(this.addr() + PATH_MANAGER_IMPL_STATE, 0);
    e.mem
        .set_u32(this.addr() + PATH_MANAGER_IMPL_PAUSE_RETRIES, 0);
    e.mem
        .set_u8(this.addr() + PATH_MANAGER_IMPL_CANCEL_PATHS, 0);
    this
}

// Translated from 006eab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the request map, queues, lock and tasklet group in reverse order.
pub fn path_manager_impl_destruct(e: &mut Engine, this: Ptr) {
    e.call(0x006e_c870, &args![this.addr() + 0x1480]);
    e.call(0x006e_c3f0, &args![this.addr() + 0x1064]);
    e.call(0x006e_c3f0, &args![this.addr() + 0x0c50]);
    e.call(0x0044_fb00, &args![this.addr() + 0x0c38]);
    e.call(0x006e_c130, &args![this.addr() + 0x0c10]);
    e.call(0x006e_bd30, &args![this.addr() + 4]);
    e.call(0x0048_3710, &args![this]);
}

// Translated from 006eabe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the task group and resets the manager's state after both manager
/// and group creation succeed.
pub fn path_manager_impl_initialize(e: &mut Engine, this: Ptr) -> bool {
    // 006eabea pushes `this` before `00b00a00`, which takes no stack word (plain `RET`): it
    // stays on the stack as `CreateTaskGroup`'s word.
    let manager = e.call(0x00b0_0a00, &args![]).u32();
    if !e.call(0x00b0_0a80, &args![manager, this]).bool() {
        return false;
    }
    if !e.call(0x006c_08e0, &args![this]).bool() {
        return false;
    }
    e.mem.set_u32(this.addr() + PATH_MANAGER_IMPL_STATE, 0);
    e.mem
        .set_u32(this.addr() + PATH_MANAGER_IMPL_PAUSE_RETRIES, 0);
    e.mem
        .set_u8(this.addr() + PATH_MANAGER_IMPL_CANCEL_PATHS, 0);
    true
}

// Translated from 006eac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the path-manager task group after cancelling it.
pub fn path_manager_impl_destroy(e: &mut Engine, this: Ptr) {
    e.call(0x006c_09d0, &args![this]);
    // 006eac53: `00b00a00` takes no stack word (plain RET, no stack read); the pushed
    // `this` is the word of `00b00c20` (RET 4).
    let manager = e.call(0x00b0_0a00, &args![]).u32();
    e.call(0x00b0_0c20, &args![manager, this]);
}

// Translated from 006eac70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006eac70 - compiler-generated SEH cleanup remains unimplemented
/// Builds task data from the two stack-owned request sources and reports the
/// cancellation result. SEH and compiler-generated cancellation cleanup are
/// omitted; source ownership is passed through the observed helper calls.
pub fn fn_006eac70(e: &mut Engine, this: Ptr, source_a: u32, source_b: u32) -> bool {
    e.with_stack(0x50, |e, locals| {
        let request_slot = locals.addr();
        let token_slot = locals.addr() + 4;
        let token_copy = locals.addr() + 8;
        let data_ref = locals.addr() + 12;
        let guard = locals.addr() + 16;
        // The two arguments are by-value smart-pointer words. Native LEA
        // instructions pass their stack slots to accessors and destructors.
        e.mem.set_u32(request_slot, source_a);
        e.mem.set_u32(token_slot, source_b);
        let allocation = e.call(0x0040_1000, &args![0x80u32]).u32();
        let data = if allocation == 0 {
            0
        } else {
            e.call(0x006e_bf30, &args![token_copy, token_slot]);
            pathing_task_data_construct(
                e,
                Ptr::new(allocation),
                request_slot,
                e.mem.u32(token_copy),
            )
            .addr()
        };
        e.mem.set_u32(data_ref, data);
        e.call(0x006c_5e40, &args![guard]);
        e.call(0x006e_cd40, &args![guard, data]);
        let key = e.call(0x0055_9450, &args![request_slot]).u32();
        let map = this.addr() + 0x1480;
        e.vcall(map, 0x10, &args![key, data_ref, 1u32]);
        let cancelled = e.call(0x006c_5ec0, &args![this, guard, 0u32]).bool();
        if !cancelled {
            let key = e.call(0x0055_9450, &args![request_slot]).u32();
            e.vcall(map, 0x14, &args![key]);
            let retained = e.mem.u32(data_ref);
            if retained != 0 {
                e.vcall(retained, 0, &args![1u32]);
            }
        }
        e.call(0x006c_5e70, &args![guard]);
        e.call(0x0045_cec0, &args![request_slot]);
        e.call(0x006e_bf90, &args![token_slot]);
        cancelled
    })
}

// Translated from 006eae40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006eae40 - compiler-generated SEH cleanup remains unimplemented
/// Updates pause state, processes task completions, drains add/remove queues,
/// and notifies the path listener when the queue changed. SEH cleanup is omitted.
pub fn fn_006eae40(e: &mut Engine, this: Ptr) {
    e.call(0x0045_38a0, &args![this.addr() + 0x0c38, 0x0106_d5b0u32]);
    match e.mem.u32(this.addr() + PATH_MANAGER_IMPL_STATE) {
        1 => {
            if e.call(0x00aa_6220, &args![this.addr() + 0x0c2c]).bool() {
                e.mem.set_u32(this.addr() + PATH_MANAGER_IMPL_STATE, 2);
            } else {
                let retries = e.mem.u32(this.addr() + PATH_MANAGER_IMPL_PAUSE_RETRIES);
                e.mem.set_u32(
                    this.addr() + PATH_MANAGER_IMPL_PAUSE_RETRIES,
                    retries.wrapping_add(1),
                );
            }
        }
        3 => {
            e.call(0x00aa_62e0, &args![this.addr() + 0x0c2c]);
            e.mem.set_u32(this.addr() + PATH_MANAGER_IMPL_STATE, 0);
            e.mem
                .set_u32(this.addr() + PATH_MANAGER_IMPL_PAUSE_RETRIES, 0);
        }
        _ => {}
    }
    e.with_stack(0x70, |e, locals| {
        let task_stack = locals.addr();
        let completion = locals.addr() + 0x20;
        let status_out = locals.addr() + 0x30;
        let task_out = locals.addr() + 0x34;
        let path_holder = locals.addr() + 0x38;
        let changed = locals.addr() + 0x3c;
        let point = locals.addr() + 0x40;
        e.mem.set_u32(locals.addr() + 0x48, 0);
        e.call(0x006e_c970, &args![task_stack]);
        while !e.call(0x006e_bde0, &args![task_stack]).bool() {
            let popped = e
                .call(
                    0x0044_c640,
                    &args![this.addr() + 0x1480, task_stack, status_out, task_out, 1u32],
                )
                .bool();
            if popped {
                // disassembly reads the third c640 out-argument at EBP-0x34;
                // the preceding out-argument at EBP-0x38 is not task data.
                let task_data = Ptr::new(e.mem.u32(task_out));
                if e.call(0x005d_c980, &args![task_data]).bool() {
                    let built = fn_006ea350(e, task_data);
                    let total = e.mem.u32(locals.addr() + 0x48).wrapping_add(built as u32);
                    e.mem.set_u32(locals.addr() + 0x48, total);
                    if total > 10 {
                        break;
                    }
                    if fn_006ea490(e, task_data) {
                        e.call(0x0060_d4b0, &args![task_data, 0u32]);
                        e.call(0x006c_5e40, &args![point]);
                        e.call(0x006e_cd40, &args![point, task_data]);
                        e.call(0x006c_5ec0, &args![this, point, 0u32]);
                        e.call(0x006c_5e70, &args![point]);
                    }
                }
            }
        }
        fn_006ea9d0(e, Ptr::new(completion));
        while e
            .vcall(this.addr() + 0x0c10, 0x10, &args![completion])
            .bool()
        {
            let kind = e.mem.i32(completion);
            if (0..=1).contains(&kind) {
                // PC loads the task-data pointer from the completion before +0x18.
                let data = e.mem.u32(completion + 4);
                let key = e.call(0x0055_9450, &args![data + 0x18]).u32();
                e.vcall(this.addr() + 0x1480, 0x14, &args![key]);
                let reference = e.mem.u32(completion + 4);
                if reference != 0 {
                    e.vcall(reference, 0, &args![1u32]);
                }
            }
        }
        e.mem.set_u8(changed, 0);
        e.call(0x0063_3c90, &args![path_holder, 0u32]);
        while e
            .vcall(this.addr() + 0x1064, 0x10, &args![path_holder])
            .bool()
        {
            if e.call(0x008c_7aa0, &args![]).bool() {
                let item = e.call(0x0055_9450, &args![path_holder]).u32();
                // 006eb09a: `004537b0` takes no word (plain RET, a global); the pushed
                // item is the word of `0087ac60` (RET 4).
                let resolved = e.call(0x0045_37b0, &args![]).u32();
                e.call(0x0087_ac60, &args![resolved, item]);
            } else {
                let listener = e.call(0x0055_9450, &args![0x011d_6e5cu32]).u32();
                let item = e.call(0x0055_9450, &args![path_holder]).u32();
                e.vcall(listener, 0xe8, &args![item]);
            }
            e.mem.set_u8(changed, 1);
        }
        while e
            .vcall(this.addr() + 0x0c50, 0x10, &args![path_holder])
            .bool()
        {
            if e.call(0x008c_7aa0, &args![]).bool() {
                let global = e.call(0x0055_9450, &args![0x011d_6e5cu32]).u32();
                let item = e.call(0x0055_9450, &args![path_holder]).u32();
                // 006eb11a: as above, the item, global, 1 and 0 are the four words of
                // `0087ac00` (RET 0x10).
                let resolved = e.call(0x0045_37b0, &args![]).u32();
                e.call(0x0087_ac00, &args![resolved, item, global, 1u32, 0u32]);
            } else {
                let listener = e.call(0x0055_9450, &args![0x011d_6e5cu32]).u32();
                let item = e.call(0x0055_9450, &args![path_holder]).u32();
                e.vcall(listener, 0xdc, &args![item, 1u32]);
            }
            e.mem.set_u8(changed, 1);
        }
        if e.mem.u8(changed) != 0 {
            if e.call(0x008c_7aa0, &args![]).bool() {
                let global = e.call(0x0055_9450, &args![0x011d_6e5cu32]).u32();
                // 006eb178: as above, the global is the word of `0087ad50` (RET 4).
                let resolved = e.call(0x0045_37b0, &args![]).u32();
                e.call(0x0087_ad50, &args![resolved, global]);
            } else {
                e.call(0x0043_d410, &args![point, 0.0f32, 0u32, 0u32]);
                let global = e.call(0x0055_9450, &args![0x011d_6e5cu32]).u32();
                e.call(0x00a5_9c60, &args![global, point]);
            }
        }
        e.call(0x0045_38c0, &args![this.addr() + 0x0c38]);
        e.call(0x0045_cec0, &args![path_holder]);
        e.call(0x0048_3710, &args![completion]);
        e.call(0x006e_bdc0, &args![task_stack]);
    });
}
// Translated from 006eb200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes a path message to the regular queue or ray-cast queue by mode.
pub fn fn_006eb200(e: &mut Engine, this: Ptr, message: u32, ray_cast: u8) -> bool {
    let offset = if ray_cast == 0 { 0x1064 } else { 0x0c50 };
    e.vcall(this.addr() + offset, 4, &args![message]);
    true
}

// Translated from 006eb250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds request task data in the manager map, returning status 2 if the key
/// has no task data.
pub fn fn_006eb250(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    start: Ptr,
    goal: Ptr,
    result_out: u32,
    navmesh_mode: u8,
) -> i32 {
    e.with_stack(4, |e, task_data| {
        e.mem.set_u32(task_data.addr(), 0);
        if !e
            .vcall(this.addr() + 0x1480, 8, &args![key, task_data])
            .bool()
        {
            2
        } else {
            fn_006ea780(
                e,
                Ptr::new(e.mem.u32(task_data.addr())),
                start,
                goal,
                result_out,
                navmesh_mode,
            )
        }
    })
}

// Translated from 006eb2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds request task data in the manager map, returning status 2 if the key
/// has no task data.
pub fn fn_006eb2b0(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    start: Ptr,
    goal: Ptr,
    result_out: u32,
) -> i32 {
    e.with_stack(4, |e, task_data| {
        e.mem.set_u32(task_data.addr(), 0);
        if !e
            .vcall(this.addr() + 0x1480, 8, &args![key, task_data])
            .bool()
        {
            2
        } else {
            fn_006ea510(
                e,
                Ptr::new(e.mem.u32(task_data.addr())),
                start,
                goal,
                result_out,
            )
        }
    })
}

// Translated from 006eb310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pauses the manager and clears the pause retry counter when running.
pub fn fn_006eb310(e: &mut Engine, this: Ptr) {
    e.call(0x0045_38a0, &args![this.addr() + 0x0c38, 0x0106_d5c8u32]);
    match e.mem.u32(this.addr() + PATH_MANAGER_IMPL_STATE) {
        0 => {
            e.mem.set_u32(this.addr() + PATH_MANAGER_IMPL_STATE, 1);
            e.mem
                .set_u32(this.addr() + PATH_MANAGER_IMPL_PAUSE_RETRIES, 0);
        }
        3 => e.mem.set_u32(this.addr() + PATH_MANAGER_IMPL_STATE, 2),
        _ => {}
    }
    e.call(0x0045_38c0, &args![this.addr() + 0x0c38]);
}

// Translated from 006eb390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reports whether the path manager is in paused state 2.
pub fn fn_006eb390(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + PATH_MANAGER_IMPL_STATE) == 2
}

// Translated from 006eb3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resumes a paused manager, retaining the intermediate state if it was 2.
pub fn fn_006eb3b0(e: &mut Engine, this: Ptr) {
    e.call(0x0045_38a0, &args![this.addr() + 0x0c38, 0x0106_d5e0u32]);
    match e.mem.u32(this.addr() + PATH_MANAGER_IMPL_STATE) {
        1 => e.mem.set_u32(this.addr() + PATH_MANAGER_IMPL_STATE, 0),
        2 => e.mem.set_u32(this.addr() + PATH_MANAGER_IMPL_STATE, 3),
        _ => {}
    }
    e.call(0x0045_38c0, &args![this.addr() + 0x0c38]);
}

// Translated from 006eb420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reports whether the path manager is in running state 0.
pub fn fn_006eb420(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + PATH_MANAGER_IMPL_STATE) == 0
}

// Translated from 006eb440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006eb440 - compiler-generated SEH cleanup remains unimplemented
/// Cancels queued completions and tasks, then enters paused state. SEH cleanup is omitted.
pub fn fn_006eb440(e: &mut Engine, this: Ptr) {
    e.call(0x0045_38a0, &args![this.addr() + 0x0c38, 0x0106_d5f8u32]);
    e.call(0x006c_09d0, &args![this]);
    e.mem
        .set_u8(this.addr() + PATH_MANAGER_IMPL_CANCEL_PATHS, 1);
    fn_006eb670(e, this, 0);
    e.with_stack(0x70, |e, locals| {
        let completion = locals.addr();
        fn_006ea9d0(e, Ptr::new(completion));
        loop {
            if !e
                .vcall(this.addr() + 0x0c10, 0x10, &args![completion])
                .bool()
            {
                break;
            }
            let kind = e.mem.i32(completion);
            if (0..=1).contains(&kind) {
                // PC loads the task-data pointer from the completion before +0x18.
                let data = e.mem.u32(completion + 4);
                let key = e.call(0x0055_9450, &args![data + 0x18]).u32();
                e.vcall(this.addr() + 0x1480, 0x14, &args![key]);
                let reference = e.mem.u32(completion + 4);
                if reference != 0 {
                    e.vcall(reference, 0, &args![1u32]);
                }
            }
        }
        let task_stack = locals.addr() + 0x20;
        let first_out = locals.addr() + 0x40;
        let task_out = locals.addr() + 0x44;
        let actor_message = locals.addr() + 0x50;
        e.call(0x006e_c970, &args![task_stack]);
        while !e.call(0x006e_bde0, &args![task_stack]).bool() {
            if e.call(
                0x0044_c640,
                &args![this.addr() + 0x1480, task_stack, first_out, task_out, 1u32],
            )
            .bool()
            {
                let data = e.mem.u32(task_out);
                let listener = e.call(0x0055_9450, &args![data + 0x60]).u32();
                fn_006e9c00(e, Ptr::new(actor_message), 1, data + 0x1c);
                e.vcall(listener, 4, &args![actor_message]);
                e.call(0x0080_0190, &args![actor_message]);
                if data != 0 {
                    e.vcall(data, 0, &args![1u32]);
                }
            }
        }
        e.call(0x0044_c480, &args![this.addr() + 0x1480, 0u32]);
        e.call(0x00aa_6060, &args![this.addr() + 0x0c2c]);
        e.mem.set_u32(this.addr() + PATH_MANAGER_IMPL_STATE, 2);
        e.mem
            .set_u8(this.addr() + PATH_MANAGER_IMPL_CANCEL_PATHS, 0);
        e.call(0x006c_08e0, &args![this]);
        e.call(0x0045_38c0, &args![this.addr() + 0x0c38]);
        e.call(0x006e_bdc0, &args![task_stack]);
        e.call(0x0048_3710, &args![completion]);
    });
}

// Translated from 006eb670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the cancellation object's vtable slot 4 unless it is already complete.
pub fn fn_006eb670(e: &mut Engine, this: Ptr, action: u8) {
    let cancellation = e.mem.u32(this.addr());
    if cancellation != 0 && !e.call(0x004f_1540, &args![cancellation]).bool() {
        e.vcall(cancellation, 4, &args![action]);
    }
}

// Translated from 006eb6b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006eb6b0 - compiler-generated SEH cleanup remains unimplemented
/// Clears the matching request key from queued task cancellation data. SEH cleanup is omitted.
pub fn fn_006eb6b0(e: &mut Engine, this: Ptr, key: u32) {
    e.call(0x0045_38a0, &args![this.addr() + 0x0c38, 0x0106_d5f8u32]);
    e.with_stack(0x30, |e, locals| {
        let task_stack = locals.addr();
        let first_out = locals.addr() + 0x20;
        let task_out = locals.addr() + 0x24;
        e.call(0x006e_c970, &args![task_stack]);
        while !e.call(0x006e_bde0, &args![task_stack]).bool() {
            if e.call(
                0x0044_c640,
                &args![this.addr() + 0x1480, task_stack, task_out, first_out, 1u32],
            )
            .bool()
            {
                let data = e.mem.u32(task_out);
                let cancel_data = e.call(0x006c_9900, &args![data]).u32();
                if e.mem.u32(cancel_data + 4) == key {
                    let cancel_data = e.call(0x006c_9900, &args![data]).u32();
                    e.mem.set_u32(cancel_data + 4, 0);
                    let cancel_data = e.call(0x006c_9900, &args![data]).u32();
                    e.mem.set_u32(cancel_data, 0);
                }
            }
        }
        e.call(0x0045_38c0, &args![this.addr() + 0x0c38]);
        e.call(0x006e_bdc0, &args![task_stack]);
    });
}

// Translated from 006eb790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the manager pause event's signal state.
pub fn fn_006eb790(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x00aa_6120, &args![this.addr() + 0x0c2c]).bool()
}

// Translated from 006eb7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Signals the manager pause event.
pub fn fn_006eb7b0(e: &mut Engine, this: Ptr) {
    e.call(0x00aa_62c0, &args![this.addr() + 0x0c2c]);
}

// Translated from 006eb7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the path cancellation flag.
pub fn fn_006eb7d0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + PATH_MANAGER_IMPL_CANCEL_PATHS) != 0
}

// Translated from 006eb7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Enqueues a message on the path manager's task queue.
pub fn fn_006eb7f0(e: &mut Engine, this: Ptr, message: u32) {
    e.vcall(this.addr() + 0x0c10, 4, &args![message]);
}

// Translated from 006eb820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the named Pathing/BuildPath message and clears its state word.
pub fn fn_006eb820(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        0x0045_d260,
        &args![this.addr() + 4, 0x0103_a1bcu32, 0x0106_d624u32, 0u32, 1u32],
    );
    e.mem.set_u32(this.addr(), 0);
    this
}

// Translated from 006eb850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stops the owned task with the flag passed by the executable when non-null.
pub fn fn_006eb850(e: &mut Engine, this: Ptr) {
    let task = e.mem.u32(this.addr());
    if task != 0 {
        e.call(0x006e_b890, &args![task, 1u32]);
    }
}

// Translated from 006eb890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the manager implementation and conditionally frees its storage.
pub fn fn_006eb890(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    path_manager_impl_destruct(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006eb8c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006eb8c0 - compiler-generated exception cleanup remains unimplemented
// open: exceptional exit cleanup is not represented by the engine ABI.
/// `PathManager::Initialize` (Xbox PDB): allocates, constructs and initializes its implementation.
pub fn fn_006eb8c0(e: &mut Engine, this: Ptr) -> bool {
    e.with_stack(4, |e, guard| {
        e.call(
            0x0040_4eb0,
            &args![guard, 0x2du32, 1u32, 0x0106_d560u32, 0x56au32],
        );
        let allocation = e.call(0x0040_1000, &args![0x14c0u32]).u32();
        let implementation = if allocation == 0 {
            Ptr::NULL
        } else {
            path_manager_impl_construct(e, Ptr::new(allocation))
        };
        e.mem.set_u32(this.addr(), implementation.addr());
        let initialized = path_manager_impl_initialize(e, implementation);
        e.call(0x0040_4ee0, &args![guard]);
        initialized
    })
}

// Translated from 006eb980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stops the task group, reloads the owned implementation, deletes it and clears ownership.
pub fn fn_006eb980(e: &mut Engine, this: Ptr) {
    path_manager_impl_destroy(e, Ptr::new(e.mem.u32(this.addr())));
    let implementation = e.mem.u32(this.addr());
    if implementation != 0 {
        fn_006eb890(e, Ptr::new(implementation), 1);
    }
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 006eb9d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006eb9d0 - compiler-generated exception cleanup remains unimplemented
// open: exceptional exits do not destroy the active smart-pointer temporaries.
/// `PathManager::BuildPath` (Xbox PDB): queues a copied request when both
/// asynchronous gates are set; otherwise builds and dispatches a result now.
pub fn fn_006eb9d0(
    e: &mut Engine,
    this: Ptr,
    request: u32,
    listener: u32,
    asynchronous: u8,
) -> bool {
    e.with_stack(0x20, |e, locals| {
        let request_slot = locals.addr();
        let listener_slot = locals.addr() + 4;
        let request_copy = locals.addr() + 8;
        let listener_copy = locals.addr() + 12;
        let solution_slot = locals.addr() + 16;
        let message = Ptr::new(locals.addr() + 20);
        e.mem.set_u32(request_slot, request);
        e.mem.set_u32(listener_slot, listener);
        let setting = e.call(0x0040_8d60, &args![0x011d_77ccu32]).u32();
        let accepted = if e.mem.u8(setting) != 0 && asynchronous != 0 {
            e.call(0x006e_bf30, &args![listener_copy, listener_slot]);
            e.call(0x0055_9a40, &args![request_copy, request_slot]);
            fn_006eac70(
                e,
                Ptr::new(e.mem.u32(this.addr())),
                e.mem.u32(request_copy),
                e.mem.u32(listener_copy),
            )
        } else {
            let allocation = e.call(0x00aa_13e0, &args![0x44u32]).u32();
            let solution = if allocation == 0 {
                0
            } else {
                e.call(0x006e_7650, &args![allocation]).u32()
            };
            e.call(0x0063_3c90, &args![solution_slot, solution]);
            let solution = e.call(0x0055_9450, &args![solution_slot]).u32();
            let request = e.call(0x0055_9450, &args![request_slot]).u32();
            let built = e.call(0x006d_0900, &args![request, solution]).bool();
            let listener = e.call(0x0055_9450, &args![listener_slot]).u32();
            fn_006e9c00(e, message, u32::from(!built), solution_slot);
            e.vcall(listener, 4, &args![message]);
            e.call(0x0080_0190, &args![message]);
            e.call(0x0045_cec0, &args![solution_slot]);
            true
        };
        e.call(0x0045_cec0, &args![request_slot]);
        e.call(0x006e_bf90, &args![listener_slot]);
        accepted
    })
}

// Translated from 006ebbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Routes a message through the owned implementation, preserving AL.
pub fn fn_006ebbd0(e: &mut Engine, this: Ptr, message: u32, mode: u8) -> bool {
    fn_006eb200(e, Ptr::new(e.mem.u32(this.addr())), message, mode)
}

// Translated from 006ebbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathManager::GetRayCastPathToNavMesh` (Xbox PDB): forwards all five stack words.
pub fn fn_006ebbf0(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    start: Ptr,
    goal: Ptr,
    output: u32,
    mode: u8,
) -> i32 {
    fn_006eb250(
        e,
        Ptr::new(e.mem.u32(this.addr())),
        key,
        start,
        goal,
        output,
        mode,
    )
}

// Translated from 006ebc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathManager::GetRayCastPath` (Xbox PDB): forwards all four stack words.
pub fn fn_006ebc20(e: &mut Engine, this: Ptr, key: u32, start: Ptr, goal: Ptr, output: u32) -> i32 {
    fn_006eb2b0(
        e,
        Ptr::new(e.mem.u32(this.addr())),
        key,
        start,
        goal,
        output,
    )
}

// Translated from 006ebc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the owned implementation.
pub fn fn_006ebc50(e: &mut Engine, this: Ptr) {
    fn_006eae40(e, Ptr::new(e.mem.u32(this.addr())))
}

// Translated from 006ebc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pauses the owned implementation.
pub fn fn_006ebc70(e: &mut Engine, this: Ptr) {
    fn_006eb310(e, Ptr::new(e.mem.u32(this.addr())))
}

// Translated from 006ebc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queries the paused state of the owned implementation.
pub fn fn_006ebc90(e: &mut Engine, this: Ptr) -> bool {
    fn_006eb390(e, Ptr::new(e.mem.u32(this.addr())))
}

// Translated from 006ebcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queries the running state of the owned implementation.
pub fn fn_006ebcb0(e: &mut Engine, this: Ptr) -> bool {
    fn_006eb420(e, Ptr::new(e.mem.u32(this.addr())))
}

// Translated from 006ebcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resumes the owned implementation.
pub fn fn_006ebcd0(e: &mut Engine, this: Ptr) {
    fn_006eb3b0(e, Ptr::new(e.mem.u32(this.addr())))
}

// Translated from 006ebcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cancels the owned implementation.
pub fn fn_006ebcf0(e: &mut Engine, this: Ptr) {
    fn_006eb440(e, Ptr::new(e.mem.u32(this.addr())))
}

// Translated from 006ebd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathManager::ClearInventoryPointersFromCurrentRequests` (Xbox PDB): forwards the key.
pub fn fn_006ebd10(e: &mut Engine, this: Ptr, key: u32) {
    fn_006eb6b0(e, Ptr::new(e.mem.u32(this.addr())), key)
}

// Translated from 006ebd30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ebd30 - compiler-generated exception cleanup remains unimplemented
// open: exceptional exit cleanup is not represented by the engine ABI.
/// Destroys the 256 embedded messages, then the queue base.
pub fn fn_006ebd30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_d634);
    e.call(
        0x00ec_5fce,
        &args![this.addr() + 12, 12u32, 256u32, 0x0048_3710u32],
    );
    fn_006ebda0(e, this);
}

// Translated from 006ebda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the base vtable without releasing storage.
pub fn fn_006ebda0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_d63c);
}

// Translated from 006ebdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the base vtable without releasing storage.
pub fn fn_006ebdc0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_d644);
}

// Translated from 006ebde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the iterator exhausted flag, bit 1 of its byte at +12.
pub fn fn_006ebde0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 12) & 2 != 0
}

// Translated from 006ebe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the object destructor, freeing storage only for delete flag bit zero.
pub fn fn_006ebe00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ebd30(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006ebe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the object destructor, freeing storage only for delete flag bit zero.
pub fn fn_006ebe30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ebda0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006ebe60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the object destructor, freeing storage only for delete flag bit zero.
pub fn fn_006ebe60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ebdc0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006ebe90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the empty array.
pub fn fn_006ebe90(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0106_d650);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006ebec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restores the array vtable and releases its buffer.
pub fn fn_006ebec0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_d650);
    e.call(0x0084_54f0, &args![this, 1u32]);
}

// Translated from 006ebee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the empty array.
pub fn fn_006ebee0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0106_d664);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006ebf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restores the array vtable and releases its buffer.
pub fn fn_006ebf10(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_d664);
    e.call(0x0084_54f0, &args![this, 1u32]);
}

// Translated from 006ebfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns a token pointer, releasing the old owner before reloading and retaining the source.
pub fn fn_006ebfb0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    if e.mem.u32(this.addr()) != e.mem.u32(source.addr()) {
        let previous = e.mem.u32(this.addr());
        if previous != 0 {
            e.call(0x0040_1970, &args![previous + 0x14]);
        }
        let replacement = e.mem.u32(source.addr());
        e.mem.set_u32(this.addr(), replacement);
        if replacement != 0 {
            e.call(0x0040_f6e0, &args![replacement + 0x14]);
        }
    }
    this
}

// Translated from 006ec000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ec000 - compiler-generated exception cleanup remains unimplemented
// open: exceptional exit cleanup is not represented by the engine ABI.
/// Constructs the fixed 256-element message pool and gives its storage to the base.
pub fn fn_006ec000(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x006e_cc40, &args![this]);
    e.mem.set_u32(this.addr(), 0x0106_d634);
    e.call(
        0x00ec_782f,
        &args![
            this.addr() + 12,
            12u32,
            256u32,
            0x006e_d0e0u32,
            0x0048_3710u32
        ],
    );
    e.call(0x006e_d040, &args![this, 256u32, this.addr() + 12]);
    this
}

// Translated from 006ec090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an empty linked message queue with the supplied free-list owner.
pub fn fn_006ec090(e: &mut Engine, this: Ptr, pool: u32) -> Ptr {
    e.call(0x006e_cb40, &args![this]);
    e.mem.set_u32(this.addr(), 0x0106_d678);
    e.mem.set_u32(this.addr() + 8, pool);
    e.mem.set_u32(this.addr() + 12, 0);
    e.mem.set_u32(this.addr() + 16, this.addr() + 12);
    this
}

// Translated from 006ec130 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ec130 - compiler-generated exception cleanup remains unimplemented
// open: exceptional exit cleanup is not represented by the engine ABI.
/// Drains the linked queue, destroys its local message, then destroys the base.
pub fn fn_006ec130(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_d678);
    e.with_stack(8, |e, message| {
        fn_006ea9d0(e, message);
        while e.call(0x006e_c390, &args![this, message]).bool() {}
        e.call(0x0048_3710, &args![message]);
    });
    e.call(0x006e_c1c0, &args![this]);
}

// Translated from 006ec230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTMessageQueue<PathingMessage>` scalar deleting destructor (Xbox PDB).
pub fn fn_006ec230(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006e_c1e0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006ec260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonLLMessageQueue<PathingMessage>::DoTryPush` (Xbox PDB): appends an allocated node.
pub fn fn_006ec260(e: &mut Engine, this: Ptr, message: u32) -> bool {
    let pool = e.mem.u32(this.addr() + 8);
    if pool == 0 {
        return false;
    }
    e.with_stack(4, |e, node| {
        if !e.call(0x006e_cd00, &args![pool, node, message]).bool() {
            return false;
        }
        if e.mem.u32(this.addr() + 12) == 0 {
            e.mem.set_u32(this.addr() + 16, this.addr() + 12);
        }
        let tail = e.mem.u32(this.addr() + 16);
        let allocated = e.mem.u32(node.addr());
        e.mem.set_u32(tail, allocated);
        e.mem.set_u32(allocated + 8, 0);
        e.mem.set_u32(this.addr() + 16, allocated + 8);
        true
    })
}

// Translated from 006ec2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonLLMessageQueue<PathingMessage>::DoTryPop` (Xbox PDB): copies a message and returns its node.
pub fn fn_006ec2e0(e: &mut Engine, this: Ptr, output: Ptr) -> bool {
    let head = e.mem.u32(this.addr() + 12);
    if head == 0 {
        return false;
    }
    e.with_stack(4, |e, node| {
        e.mem.set_u32(node.addr(), head);
        let source = e.call(0x0068_15c0, &args![head]).ptr::<()>();
        fn_006eaa00(e, output, source);
        e.mem.set_u32(this.addr() + 12, e.mem.u32(head + 8));
        let pool = e.mem.u32(this.addr() + 8);
        e.call(0x006e_cb10, &args![pool, node]);
    });
    true
}

// Translated from 006ec340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the 256-slot static pointer queue with zero count and cursors.
pub fn fn_006ec340(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x006e_cb70, &args![this]);
    e.mem.set_u32(this.addr(), 0x0106_d6d0);
    for offset in [0x408, 0x40c, 0x410] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    this
}

// Translated from 006ec3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the static queue vtable before delegating base destruction.
pub fn fn_006ec3f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_d6d0);
    e.call(0x006e_c410, &args![this]);
}

// Translated from 006ec4b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ec4b0 - compiler-generated exception cleanup remains unimplemented
// open: exceptional exit cleanup is not represented by the engine ABI.
/// `BSTCommonStaticMessageQueue<NiPointer<NiAVObject>,256>::DoTryPush` (Xbox PDB).
pub fn fn_006ec4b0(e: &mut Engine, this: Ptr, source: u32) -> bool {
    if e.mem.u32(this.addr() + 0x408) >= 256 {
        return false;
    }
    let index = e.mem.u32(this.addr() + 0x40c);
    let slot = this
        .addr()
        .wrapping_add(8)
        .wrapping_add(index.wrapping_mul(4));
    let destination = e.call(0x006e_6da0, &args![4u32, slot]).u32();
    if destination != 0 {
        e.call(0x0055_9a40, &args![destination, source]);
    }
    let next = e.mem.u32(this.addr() + 0x40c).wrapping_add(1) % 256;
    e.mem.set_u32(this.addr() + 0x40c, next);
    let count = e.mem.u32(this.addr() + 0x408).wrapping_add(1);
    e.mem.set_u32(this.addr() + 0x408, count);
    true
}

// Translated from 006ec5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonStaticMessageQueue<NiPointer<NiAVObject>_256>::DoTryPop` (Xbox PDB).
pub fn fn_006ec5b0(e: &mut Engine, this: Ptr, output: Ptr) -> bool {
    if e.mem.u32(this.addr() + 0x408) == 0 {
        return false;
    }
    let index = e.mem.u32(this.addr() + 0x410);
    let source = this
        .addr()
        .wrapping_add(8)
        .wrapping_add(index.wrapping_mul(4));
    e.call(0x006e_5cc0, &args![output, source]);
    e.call(0x0082_28f0, &args![source, 0u32]);
    let next = e.mem.u32(this.addr() + 0x410).wrapping_add(1);
    e.mem.set_u32(this.addr() + 0x410, next);
    e.mem.set_u32(this.addr() + 0x410, next % 256);
    let count = e.mem.u32(this.addr() + 0x408);
    e.mem.set_u32(this.addr() + 0x408, count.wrapping_sub(1));
    true
}

// Translated from 006ec660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ec660 - compiler-generated SEH cleanup is not represented by the engine ABI
/// Constructs the path-manager implementation's fixed-size pathing state.
// open: constructor failure cleanup for partially allocated arrays and synchronization state.
pub fn fn_006ec660(
    e: &mut Engine,
    this: Ptr,
    request_count: u32,
    path_count: u32,
    config: u32,
) -> Ptr {
    e.call(0x0044_d7f0, &args![this]);
    e.mem.set_u32(this.addr(), 0x0106_d72c);
    e.call(0x0096_a2d0, &args![this.addr() + 0x20]);
    e.with_stack(0x10, |e, guard| {
        e.call(
            0x0040_4eb0,
            &args![guard, 6u32, 1u32, 0x0101_73f8u32, 0x390u32],
        );
        e.mem.set_u32(this.addr() + 0x18, 0);
        e.mem.set_u32(this.addr() + 8, path_count);
        let slot_bytes = path_count.saturating_mul(4);
        let slots = e.call(0x0040_1000, &args![slot_bytes]).u32();
        if slots != 0 {
            e.call(0x0040_1050, &args![slots, 4u32, path_count, 0x0044_dee0u32]);
        }
        e.mem.set_u32(this.addr() + 12, slots);
        let capacity = request_count.wrapping_mul(3).saturating_mul(4);
        let storage = e.call(0x0040_1000, &args![capacity]).u32();
        e.mem.set_u32(this.addr() + 4, storage);
        e.mem.set_u32(this.addr() + 16, config);
        let allocator = e.call(0x0040_1000, &args![0x10u32]).u32();
        let data = if allocator == 0 {
            0
        } else {
            e.call(0x006c_73b0, &args![allocator, request_count]).u32()
        };
        e.mem.set_u32(this.addr() + 20, data);
        e.call(0x0040_4ee0, &args![guard]);
    });
    this
}

// Translated from 006ec7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the request predicate and result operation while holding the manager lock.
pub fn fn_006ec7e0(
    e: &mut Engine,
    this: Ptr,
    manager: u32,
    request: u32,
    context: u32,
    mode: u8,
) -> bool {
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    // 006ec80a: `00449f80` takes no stack word (plain RET); the four pushed words are
    // those of `0044d240` (RET 0x10).
    let candidate = e.call(0x0044_9f80, &args![this]).u32();
    let result = e
        .call(
            0x0044_d240,
            &args![candidate, manager, request, context, mode],
        )
        .bool();
    e.call(0x0040_fba0, &args![this.addr() + 0x20]);
    result
}

// Translated from 006ec830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Invokes the queue's virtual item operation, then reports its result and
/// answers that report's value (the exe ends on the `006ec9b0` call and
/// leaves its `EAX`; the callers keep `AL`).
pub fn fn_006ec830(e: &mut Engine, this: Ptr, item: u32, context: Ptr) -> bool {
    let receiver = e.mem.u32(this.addr());
    let value = e.vcall(receiver, 0x24, &args![item]).u32();
    fn_006ec9b0(e, this, value, item, context)
}

// Translated from 006ec870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ec870 - compiler-generated SEH cleanup is not represented by the engine ABI
/// Releases the manager's allocated arrays and synchronization state.
// open: exceptional cleanup around lock teardown and array deallocation.
pub fn fn_006ec870(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_d72c);
    e.call(0x0044_c480, &args![this, 1u32]);
    let data = e.mem.u32(this.addr() + 12);
    e.call(0x0040_1030, &args![data]);
    let storage = e.mem.u32(this.addr() + 4);
    e.call(0x0040_1030, &args![storage]);
    e.call(0x0044_b4b0, &args![this]);
}

// Translated from 006ec900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests each registered request against the supplied item under the manager lock.
pub fn fn_006ec900(e: &mut Engine, this: Ptr, item: u32) -> bool {
    e.call(0x0040_fbf0, &args![this.addr() + 0x20, 0u32]);
    let mut matched = false;
    let mut index = 0u32;
    while index < e.mem.u32(this.addr() + 8) && !matched {
        if e.vcall(this.addr(), 0x18, &args![index, item]).bool() {
            matched = true;
        }
        index = index.wrapping_add(1);
    }
    e.call(0x0040_fba0, &args![this.addr() + 0x20]);
    matched
}

// Translated from 006ec9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies a returned item and suppresses success when its resulting state rejects it.
pub fn fn_006ec9b0(e: &mut Engine, this: Ptr, first: u32, second: u32, output: Ptr) -> bool {
    let mut success = e.call(0x0066_62a0, &args![this, first, second]).bool();
    if success {
        let exception = e.call(0x0044_df00, &args![this.addr() + 0x14]).u32();
        e.mem.set_u32(output.addr(), e.mem.u32(exception + 4));
        e.call(0x0040_fbe0, &args![]);
        let exception = e.call(0x0044_df00, &args![this.addr() + 0x14]).u32();
        success &= !e.call(0x0044_df20, &args![exception]).bool();
    }
    e.call(0x0052_9000, &args![this]);
    success
}

// Translated from 006ec970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the task stack's vtable, empty top, and byte-sized state.
pub fn task_stack_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0106_d644);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u8(this.addr() + 12, 0);
    this
}

// Translated from 006eca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<RayCastPathRequest_P_1024>::_scalar_deleting_destructor_` (Xbox PDB).
pub fn path_request_array_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ebec0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006eca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<RayCastPath_P_1024>::_scalar_deleting_destructor_` (Xbox PDB).
pub fn path_result_array_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ebf10(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006eca80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCommonLLMessageQueue<PathingMessage>::_scalar_deleting_destructor_` (Xbox PDB).
pub fn linked_message_queue_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ec130(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006ecab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Static message queue scalar deleting destructor.
pub fn static_message_queue_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ec3f0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006ecae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Manager scalar deleting destructor.
pub fn path_manager_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006ec870(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006ecb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns a node to its free list and then releases the supplied list lock.
pub fn fn_006ecb10(e: &mut Engine, this: Ptr, node: Ptr) {
    let item = e.mem.u32(node.addr());
    fn_006ecce0(e, Ptr::<()>::new(item));
    fn_006ecc70(e, this, node);
}

// Translated from 006ecb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the common linked queue base and its empty head.
pub fn fn_006ecb40(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006ecc00(e, this);
    e.mem.set_u32(this.addr(), 0x0106_d698);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 006ecb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the static message queue base and its empty count.
pub fn fn_006ecb70(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006ecc20(e, this);
    e.mem.set_u32(this.addr(), 0x0106_d6f0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 006ecba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a free-list item from its value and one-byte mode.
pub fn fn_006ecba0(e: &mut Engine, this: Ptr, mode: u8, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), value);
    e.call(0x0044_e010, &args![this, mode]);
    this
}

// Translated from 006ecbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a list node from a tag and a copied value.
pub fn fn_006ecbd0(e: &mut Engine, this: Ptr, tag: u32, value: Ptr) -> Ptr {
    e.call(0x0044_dee0, &args![this.addr() + 8]);
    e.mem.set_u32(this.addr(), tag);
    e.mem.set_u32(this.addr() + 4, e.mem.u32(value.addr()));
    this
}

// Translated from 006ecc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the free-list item base vtable.
pub fn fn_006ecc00(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0106_d6b8);
    this
}

// Translated from 006ecc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the list-node base vtable.
pub fn fn_006ecc20(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0106_d710);
    this
}

// Translated from 006ecc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the empty spin-locked free-list state.
pub fn fn_006ecc40(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0106_d63c);
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u32(this.addr() + 8, 0);
    this
}

// Translated from 006ecc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Prepends a free-list item under the list's embedded spin lock.
pub fn fn_006ecc70(e: &mut Engine, this: Ptr, item_slot: Ptr) {
    loop {
        if e.call(0x0043_b460, &args![this.addr() + 4, 0u32, 1u32])
            .u32()
            == 0
        {
            break;
        }
    }
    e.call(0x0040_fbe0, &args![]);
    let node = e.mem.u32(item_slot.addr());
    e.mem.set_u32(node + 8, e.mem.u32(this.addr() + 8));
    e.mem.set_u32(this.addr() + 8, node);
    e.mem.set_u32(item_slot.addr(), 0);
    e.call(0x0040_fbe0, &args![]);
    e.call(0x0043_b460, &args![this.addr() + 4, 1u32, 0u32]);
}

// Translated from 006ecce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the pathing list's thread-local data.
pub fn fn_006ecce0(e: &mut Engine, this: Ptr) {
    e.call(0x007b_3fa0, &args![this, 0u32]);
}

// Translated from 006ecd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTFreeList<PathingMessage>::Allocate<PathingMessage>` (Xbox PDB).
pub fn fn_006ecd00(e: &mut Engine, pool: Ptr, output: Ptr, message: u32) -> bool {
    if !e.call(0x006e_cd60, &args![pool, output]).bool() {
        return false;
    }
    let node = e.mem.u32(output.addr());
    e.call(0x006e_ce00, &args![node, message]);
    true
}

// Translated from 00800190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorPathingMessage::~ActorPathingMessage` (Xbox PDB).
pub fn actor_pathing_message_destruct(e: &mut Engine, this: Ptr) {
    e.call(0x0045_cec0, &args![this.addr() + 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_b9d0, fn_006eb9d0(Ptr, u32, u32, u8) -> bool),
        entry!(0x006e_b890, fn_006eb890(Ptr, u32) -> Ptr),
        entry!(0x006e_b8c0, fn_006eb8c0(Ptr) -> bool),
        entry!(0x006e_b980, fn_006eb980(Ptr)),
        entry!(0x006e_bbd0, fn_006ebbd0(Ptr, u32, u8) -> bool),
        entry!(0x006e_bbf0, fn_006ebbf0(Ptr, u32, Ptr, Ptr, u32, u8) -> i32),
        entry!(0x006e_bc20, fn_006ebc20(Ptr, u32, Ptr, Ptr, u32) -> i32),
        entry!(0x006e_bc50, fn_006ebc50(Ptr)),
        entry!(0x006e_bc70, fn_006ebc70(Ptr)),
        entry!(0x006e_bc90, fn_006ebc90(Ptr) -> bool),
        entry!(0x006e_bcb0, fn_006ebcb0(Ptr) -> bool),
        entry!(0x006e_bcd0, fn_006ebcd0(Ptr)),
        entry!(0x006e_bcf0, fn_006ebcf0(Ptr)),
        entry!(0x006e_bd10, fn_006ebd10(Ptr, u32)),
        entry!(0x006e_bd30, fn_006ebd30(Ptr)),
        entry!(0x006e_bda0, fn_006ebda0(Ptr)),
        entry!(0x006e_bdc0, fn_006ebdc0(Ptr)),
        entry!(0x006e_bde0, fn_006ebde0(Ptr) -> bool),
        entry!(0x006e_be00, fn_006ebe00(Ptr, u32) -> Ptr),
        entry!(0x006e_be30, fn_006ebe30(Ptr, u32) -> Ptr),
        entry!(0x006e_be60, fn_006ebe60(Ptr, u32) -> Ptr),
        entry!(0x006e_be90, fn_006ebe90(Ptr) -> Ptr),
        entry!(0x006e_bec0, fn_006ebec0(Ptr)),
        entry!(0x006e_bee0, fn_006ebee0(Ptr) -> Ptr),
        entry!(0x006e_bf10, fn_006ebf10(Ptr)),
        entry!(0x006e_bfb0, fn_006ebfb0(Ptr, Ptr) -> Ptr),
        entry!(0x006e_c000, fn_006ec000(Ptr) -> Ptr),
        entry!(0x006e_c090, fn_006ec090(Ptr, u32) -> Ptr),
        entry!(0x006e_c130, fn_006ec130(Ptr)),
        entry!(0x006e_c230, fn_006ec230(Ptr, u32) -> Ptr),
        entry!(0x006e_c260, fn_006ec260(Ptr, u32) -> bool),
        entry!(0x006e_c2e0, fn_006ec2e0(Ptr, Ptr) -> bool),
        entry!(0x006e_c340, fn_006ec340(Ptr) -> Ptr),
        entry!(0x006e_c3f0, fn_006ec3f0(Ptr)),
        entry!(0x006e_c4b0, fn_006ec4b0(Ptr, u32) -> bool),
        entry!(0x006e_c5b0, fn_006ec5b0(Ptr, Ptr) -> bool),
        entry!(0x006e_c660, fn_006ec660(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006e_c7e0, fn_006ec7e0(Ptr, u32, u32, u32, u8) -> bool),
        entry!(0x006e_c830, fn_006ec830(Ptr, u32, Ptr) -> bool),
        entry!(0x006e_c870, fn_006ec870(Ptr)),
        entry!(0x006e_c900, fn_006ec900(Ptr, u32) -> bool),
        entry!(0x006e_c9b0, fn_006ec9b0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x006e_c970, task_stack_construct(Ptr) -> Ptr),
        entry!(0x006e_ca20, path_request_array_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_ca50, path_result_array_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_ca80, linked_message_queue_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_cab0, static_message_queue_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_cae0, path_manager_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_cb10, fn_006ecb10(Ptr, Ptr)),
        entry!(0x006e_cb40, fn_006ecb40(Ptr) -> Ptr),
        entry!(0x006e_cb70, fn_006ecb70(Ptr) -> Ptr),
        entry!(0x006e_cba0, fn_006ecba0(Ptr, u8, u32) -> Ptr),
        entry!(0x006e_cbd0, fn_006ecbd0(Ptr, u32, Ptr) -> Ptr),
        entry!(0x006e_cc00, fn_006ecc00(Ptr) -> Ptr),
        entry!(0x006e_cc20, fn_006ecc20(Ptr) -> Ptr),
        entry!(0x006e_cc40, fn_006ecc40(Ptr) -> Ptr),
        entry!(0x006e_cc70, fn_006ecc70(Ptr, Ptr)),
        entry!(0x006e_cce0, fn_006ecce0(Ptr)),
        entry!(0x006e_cd00, fn_006ecd00(Ptr, Ptr, u32) -> bool),
        entry!(0x006e_9bd0, actor_pathing_message_construct(Ptr) -> Ptr),
        entry!(0x0080_0190, actor_pathing_message_destruct(Ptr)),
        entry!(0x006e_9c00, fn_006e9c00(Ptr, u32, u32) -> Ptr),
        entry!(0x006e_9c30, fn_006e9c30(Ptr, Ptr) -> Ptr),
        entry!(0x006e_9c60, fn_006e9c60(Ptr, Ptr) -> Ptr),
        entry!(0x006e_9c90, pathing_task_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_9cc0, pathing_task_data_construct(Ptr, u32, u32) -> Ptr),
        entry!(0x006e_9db0, fn_006e9db0(Ptr, Ptr, u32)),
        entry!(0x006e_9e20, fn_006e9e20(Ptr)),
        entry!(0x006e_9f60, pathing_task_data_request_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_9f90, pathing_task_data_path_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_9fc0, pathing_task_data_process(Ptr)),
        entry!(0x006e_a330, fn_006ea330(Ptr)),
        entry!(0x006e_a350, fn_006ea350(Ptr) -> i32),
        entry!(0x006e_a490, fn_006ea490(Ptr) -> bool),
        entry!(0x006e_a510, fn_006ea510(Ptr, Ptr, Ptr, u32) -> i32),
        entry!(0x006e_a780, fn_006ea780(Ptr, Ptr, Ptr, u32, u8) -> i32),
        entry!(0x006e_a9d0, fn_006ea9d0(Ptr) -> Ptr),
        entry!(0x006e_aa00, fn_006eaa00(Ptr, Ptr) -> Ptr),
        entry!(0x006e_aa30, path_manager_impl_construct(Ptr) -> Ptr),
        entry!(0x006e_ab30, path_manager_impl_destruct(Ptr)),
        entry!(0x006e_abe0, path_manager_impl_initialize(Ptr) -> bool),
        entry!(0x006e_ac40, path_manager_impl_destroy(Ptr)),
        entry!(0x006e_ac70, fn_006eac70(Ptr, u32, u32) -> bool),
        entry!(0x006e_ae40, fn_006eae40(Ptr)),
        entry!(0x006e_b200, fn_006eb200(Ptr, u32, u8) -> bool),
        entry!(0x006e_b250, fn_006eb250(Ptr, u32, Ptr, Ptr, u32, u8) -> i32),
        entry!(0x006e_b2b0, fn_006eb2b0(Ptr, u32, Ptr, Ptr, u32) -> i32),
        entry!(0x006e_b310, fn_006eb310(Ptr)),
        entry!(0x006e_b390, fn_006eb390(Ptr) -> bool),
        entry!(0x006e_b3b0, fn_006eb3b0(Ptr)),
        entry!(0x006e_b420, fn_006eb420(Ptr) -> bool),
        entry!(0x006e_b440, fn_006eb440(Ptr)),
        entry!(0x006e_b670, fn_006eb670(Ptr, u8)),
        entry!(0x006e_b6b0, fn_006eb6b0(Ptr, u32)),
        entry!(0x006e_b790, fn_006eb790(Ptr) -> bool),
        entry!(0x006e_b7b0, fn_006eb7b0(Ptr)),
        entry!(0x006e_b7d0, fn_006eb7d0(Ptr) -> bool),
        entry!(0x006e_b7f0, fn_006eb7f0(Ptr, u32)),
        entry!(0x006e_b820, fn_006eb820(Ptr) -> Ptr),
        entry!(0x006e_b850, fn_006eb850(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    fn result(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn noop_addresses(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    #[test]
    fn actor_message_constructor_sets_empty_type_and_constructs_pointer_member() {
        let mut e = Engine::new();
        let object = 0x1000;
        e.mem.map(object, 8);
        e.register(0x0063_3c90, |_, _| Ret::default());
        actor_pathing_message_construct(&mut e, Ptr::new(object));
        assert_eq!(e.mem.u32(object), u32::MAX);
        assert_eq!(e.call_log, None);
    }

    #[test]
    fn message_from_type_writes_type_and_passes_solution_pointer() {
        let mut e = Engine::new();
        let object = 0x1100;
        e.mem.map(object, 8);
        e.register(0x0055_9a40, |_, _| Ret::default());
        fn_006e9c00(&mut e, Ptr::new(object), 1, 0x3344);
        assert_eq!(e.mem.u32(object), 1);
        assert_eq!(e.mem.u32(object + 4), 0);
    }

    #[test]
    fn completion_value_constructor_and_copy_preserve_both_words() {
        let mut e = Engine::new();
        let object = 0x1200;
        let source = 0x1300;
        e.mem.map(object, 8);
        e.mem.map(source, 8);
        fn_006ea9d0(&mut e, Ptr::new(object));
        assert_eq!(e.mem.u32(object), u32::MAX);
        assert_eq!(e.mem.u32(object + 4), 0);
        e.mem.set_u32(source, 1);
        e.mem.set_u32(source + 4, 0x5678);
        fn_006eaa00(&mut e, Ptr::new(object), Ptr::new(source));
        assert_eq!(e.mem.u32(object), 1);
        assert_eq!(e.mem.u32(object + 4), 0x5678);
    }

    #[test]
    fn manager_state_queries_and_pause_resume_transitions() {
        let mut e = Engine::new();
        let manager = 0x2000;
        e.mem.map(manager, 0x1500);
        e.mem.set_u32(manager + PATH_MANAGER_IMPL_STATE, 0);
        e.register(0x0045_38a0, |_, _| Ret::default());
        e.register(0x0045_38c0, |_, _| Ret::default());
        assert!(fn_006eb420(&mut e, Ptr::new(manager)));
        fn_006eb310(&mut e, Ptr::new(manager));
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_STATE), 1);
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_PAUSE_RETRIES), 0);
        assert!(!fn_006eb420(&mut e, Ptr::new(manager)));
        fn_006eb3b0(&mut e, Ptr::new(manager));
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_STATE), 0);
        e.mem.set_u32(manager + PATH_MANAGER_IMPL_STATE, 2);
        assert!(fn_006eb390(&mut e, Ptr::new(manager)));
        fn_006eb3b0(&mut e, Ptr::new(manager));
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_STATE), 3);
    }

    #[test]
    fn manager_pause_retry_and_intermediate_state_transitions() {
        let mut e = Engine::new();
        let manager = 0x4000;
        e.mem.map(manager, 0x1500);
        e.mem.set_u32(manager + PATH_MANAGER_IMPL_STATE, 1);
        e.mem.set_u32(manager + PATH_MANAGER_IMPL_PAUSE_RETRIES, 4);
        e.register(0x0045_38a0, |_, _| Ret::default());
        e.register(0x0045_38c0, |_, _| Ret::default());
        e.register(0x00aa_6220, |_, _| Ret::default());
        fn_006eb310(&mut e, Ptr::new(manager));
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_STATE), 1);
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_PAUSE_RETRIES), 4);
        e.mem.set_u32(manager + PATH_MANAGER_IMPL_STATE, 3);
        fn_006eb310(&mut e, Ptr::new(manager));
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_STATE), 2);
    }

    #[test]
    fn cancellation_state_flag_queries_and_event_helper_return() {
        let mut e = Engine::new();
        let manager = 0x6000;
        e.mem.map(manager, 0x1500);
        e.mem.set_u8(manager + PATH_MANAGER_IMPL_CANCEL_PATHS, 1);
        e.mem.set_u32(manager + 0x0c2c, 0x1234);
        assert!(fn_006eb7d0(&mut e, Ptr::new(manager)));
        e.register(0x00aa_6120, |_, _| result(1));
        assert!(fn_006eb790(&mut e, Ptr::new(manager)));
    }

    #[test]
    fn update_adds_task_data_to_map_with_key_and_owner_and_removes_when_not_cancelled() {
        let mut e = Engine::new();
        let manager = 0x8000;
        let map = manager + 0x1480;
        e.mem.map(manager, 0x1500);
        e.mem.map(0x4000, 4);
        e.mem.set_u32(0x4000, 0x8765);
        const INSERT: u32 = 0x0077_1000;
        const REMOVE: u32 = 0x0077_1010;
        let map_vtable = 0x0017_0000;
        e.mem.set_u32(map, map_vtable);
        e.put_vtable(map_vtable, &[0, 0, 0, 0, INSERT, REMOVE]);
        let inserted = Rc::new(Cell::new(false));
        let inserted_by_call = inserted.clone();
        e.register_double(INSERT, move |e, args| {
            assert_eq!(args[0], map);
            assert_eq!(args[1], 0x8765);
            let task_data = e.mem.u32(args[2]);
            assert_eq!(e.mem.u32(task_data), PATHING_TASK_DATA_VTABLE);
            assert_eq!(args[3], 1);
            inserted_by_call.set(true);
            result(1)
        });
        let removed = Rc::new(Cell::new(false));
        let removed_by_call = removed.clone();
        e.register_double(REMOVE, move |_, args| {
            assert_eq!(args[0], map);
            assert_eq!(args[1], 0x8765);
            removed_by_call.set(true);
            Ret::default()
        });
        let allocated = 0x0016_0000;
        e.mem.map(allocated, 0x80);
        e.mem.map(PATHING_TASK_DATA_VTABLE, 0x20);
        e.mem.set_u32(PATHING_TASK_DATA_VTABLE, 0x0077_1020);
        let release_flags = Rc::new(Cell::new(0));
        let release_flags_for_double = release_flags.clone();
        e.register_double(0x0077_1020, move |_, args| {
            assert_eq!(args[0], allocated);
            release_flags_for_double.set(args[1]);
            Ret::default()
        });
        e.register_double(0x0040_1000, move |_, _| result(allocated));
        e.register(0x0055_9450, |_, _| result(0x8765));
        e.register(0x006c_5ec0, |_, _| Ret::default());
        noop_addresses(
            &mut e,
            &[
                0x006c_5e40,
                0x006e_cd40,
                0x006c_5e70,
                0x0045_cec0,
                0x006e_bf90,
                0x006c_7850,
                0x0063_3c90,
                0x0096_a2d0,
                0x006e_be90,
                0x006e_bee0,
                0x006e_bf60,
                0x006e_bf30,
                0x006e_5cc0,
                0x006e_bfb0,
            ],
        );
        assert!(!fn_006eac70(&mut e, Ptr::new(manager), 0x4000, 0x5000));
        assert!(inserted.get());
        assert!(removed.get());
        assert_eq!(release_flags.get(), 1);
    }

    #[test]
    fn cancel_all_drains_a_nonempty_completion_and_task_queue() {
        let mut e = Engine::new();
        let manager = 0x10000;
        let completion_queue = manager + 0x0c10;
        let map = manager + 0x1480;
        let task_data = 0x12000;
        let listener = 0x13000;
        let retained = 0x14000;
        let completion_count = Rc::new(Cell::new(0));
        let completion_count_for_double = completion_count.clone();
        const POP_COMPLETION: u32 = 0x0077_2010;
        const POP_TASK: u32 = 0x0077_2020;
        const REMOVE_KEY: u32 = 0x0077_2030;
        const RELEASE: u32 = 0x0077_2040;
        e.mem.map(manager, 0x1500);
        e.mem.map(task_data, 0x80);
        e.mem.map(listener, 8);
        e.mem.map(retained, 8);
        e.mem.set_u32(task_data + 0x60, listener);
        e.mem.set_u32(task_data + 0x1c, 0xabcdef);
        let completion_vtable = 0x0017_1000;
        let map_vtable = 0x0017_1100;
        let listener_vtable = 0x0017_1200;
        let retained_vtable = 0x0017_1300;
        let task_vtable = 0x0017_1400;
        e.mem.set_u32(completion_queue, completion_vtable);
        e.mem.set_u32(map, map_vtable);
        e.mem.set_u32(listener, listener_vtable);
        e.mem.set_u32(retained, retained_vtable);
        e.mem.set_u32(task_data, task_vtable);
        e.put_vtable(completion_vtable, &[0, 0, 0, 0, POP_COMPLETION]);
        e.put_vtable(map_vtable, &[0, 0, 0, 0, 0, REMOVE_KEY]);
        e.put_vtable(listener_vtable, &[0, POP_TASK, 0, 0, 0]);
        e.put_vtable(retained_vtable, &[RELEASE]);
        e.put_vtable(task_vtable, &[RELEASE]);
        e.register(0x0045_38a0, |_, _| Ret::default());
        e.register(0x0045_38c0, |_, _| Ret::default());
        e.register(0x006c_09d0, |_, _| Ret::default());
        e.register(0x004f_1540, |_, _| result(1));
        e.register_double(POP_COMPLETION, move |e, args| {
            let index = completion_count_for_double.get();
            completion_count_for_double.set(index + 1);
            if index == 0 {
                e.mem.set_u32(args[1], 1);
                e.mem.set_u32(args[1] + 4, retained);
                e.mem.set_u32(retained + 0x18, 0x9988);
                e.mem.set_u32(args[1] + 0x18, 0xdeadbeef);
                result(1)
            } else {
                Ret::default()
            }
        });
        let removed_key = Rc::new(Cell::new(false));
        let removed_key_for_double = removed_key.clone();
        e.register_double(REMOVE_KEY, move |_, args| {
            assert_eq!(args[1], 0x9988);
            removed_key_for_double.set(true);
            Ret::default()
        });
        let got_task = Rc::new(Cell::new(false));
        let got_task_for_predicate = got_task.clone();
        e.register(0x006e_c970, |_, _| Ret::default());
        e.register_double(0x006e_bde0, move |_, _| {
            if got_task_for_predicate.get() {
                result(1)
            } else {
                result(0)
            }
        });
        let got_task_for_double = got_task.clone();
        e.register_double(0x0044_c640, move |e, args| {
            if !got_task_for_double.replace(true) {
                // 006eb551/006eb555: data comes from the fourth argument.
                e.mem.set_u32(args[2], 0xdeadbeef);
                e.mem.set_u32(args[3], task_data);
                result(1)
            } else {
                Ret::default()
            }
        });
        e.register(0x0055_9450, |e, args| result(e.mem.u32(args[0])));
        e.register(0x0055_9a40, |e, args| {
            e.mem.set_u32(args[0], e.mem.u32(args[1]));
            Ret::default()
        });
        let dispatched = Rc::new(Cell::new(false));
        let dispatched_by_double = dispatched.clone();
        e.register_double(POP_TASK, move |e, args| {
            assert_eq!(e.mem.u32(args[1]), 1);
            assert_eq!(e.mem.u32(args[1] + 4), 0xabcdef);
            dispatched_by_double.set(true);
            Ret::default()
        });
        let releases = Rc::new(Cell::new(0));
        let releases_by_double = releases.clone();
        e.register_double(RELEASE, move |_, args| {
            assert_eq!(args[1], 1);
            releases_by_double.set(releases_by_double.get() + 1);
            Ret::default()
        });
        noop_addresses(
            &mut e,
            &[
                0x0080_0190,
                0x0044_c480,
                0x00aa_6060,
                0x006c_08e0,
                0x006e_bdc0,
                0x0048_3710,
            ],
        );
        fn_006eb440(&mut e, Ptr::new(manager));
        assert_eq!(completion_count.get(), 2);
        assert!(got_task.get());
        assert!(removed_key.get());
        assert!(dispatched.get());
        assert_eq!(releases.get(), 2);
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_STATE), 2);
        assert_eq!(e.mem.u8(manager + PATH_MANAGER_IMPL_CANCEL_PATHS), 0);
    }
}

#[cfg(test)]
mod remaining_tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    fn value(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn noops(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    #[test]
    fn actor_message_copy_and_path_data_construction_execute() {
        let mut e = Engine::new();
        let source = 0x3000;
        let target = 0x3100;
        let task = 0x3200;
        e.mem.map(source, 8);
        e.mem.map(target, 8);
        e.mem.map(task, 0x80);
        e.mem.set_u32(source, 7);
        noops(
            &mut e,
            &[
                0x0055_9a40,
                0x006e_5cc0,
                0x006c_7850,
                0x0063_3c90,
                0x0096_a2d0,
                0x006e_be90,
                0x006e_bee0,
                0x006e_bf60,
                0x006e_bf30,
                0x006e_bf90,
                0x006e_bfb0,
            ],
        );
        fn_006e9c30(&mut e, Ptr::new(target), Ptr::new(source));
        assert_eq!(e.mem.u32(target), 7);
        fn_006e9c60(&mut e, Ptr::new(target), Ptr::new(source));
        assert_eq!(e.mem.u32(target), 7);
        pathing_task_data_construct(&mut e, Ptr::new(task), source, source + 4);
        assert_eq!(e.mem.u32(task), PATHING_TASK_DATA_VTABLE);
        assert_eq!(e.mem.u8(task + 0x64), 0);
        fn_006e9db0(&mut e, Ptr::new(task), Ptr::new(source), source + 4);
    }

    #[test]
    fn request_path_and_task_data_destructors_run_their_cleanup_paths() {
        let mut e = Engine::new();
        let task = 0x5000;
        let request = 0x5100;
        let path = 0x5200;
        e.mem.map(task, 0x80);
        e.mem.map(request, 0x20);
        e.mem.map(path, 0x20);
        e.register(0x0044_ddc0, |_, _| value(0));
        noops(
            &mut e,
            &[
                0x0040_1030,
                0x006c_b410,
                0x006c_b4a0,
                0x006e_bf90,
                0x006e_bf10,
                0x006e_bec0,
                0x0045_cec0,
                0x00b0_0ec0,
            ],
        );
        pathing_task_data_request_deleting_destructor(&mut e, Ptr::new(request), 1);
        pathing_task_data_path_deleting_destructor(&mut e, Ptr::new(path), 1);
        pathing_task_data_scalar_deleting_destructor(&mut e, Ptr::new(task), 0);
        fn_006e9e20(&mut e, Ptr::new(task));
    }

    #[test]
    fn raycast_task_pending_and_new_request_paths_execute() {
        let mut e = Engine::new();
        let task = 0x7000;
        e.mem.map(task, 0x80);
        let alloc = Rc::new(Cell::new(0x0018_0000u32));
        let alloc_for_double = alloc.clone();
        e.register_double(0x0040_1000, move |e, args| {
            let ptr = alloc_for_double.get();
            alloc_for_double.set(ptr + 0x100);
            e.mem.map(ptr, args[0].max(0x80));
            value(ptr)
        });
        e.register(0x0044_ddc0, |_, _| value(0));
        e.register(0x006c_b3a0, |_, args| value(args[0]));
        e.register(0x006c_b480, |_, args| value(args[0]));
        noops(
            &mut e,
            &[
                0x0040_fbf0,
                0x0040_fba0,
                0x0055_9450,
                0x006d_cce0,
                0x007c_b2e0,
                0x0060_d4b0,
                0x006c_f8a0,
            ],
        );
        fn_006ea330(&mut e, Ptr::new(task));
        assert_eq!(e.mem.u8(task + 4), 1);
        assert_eq!(fn_006ea350(&mut e, Ptr::new(task)), 0);
        assert!(fn_006ea490(&mut e, Ptr::new(task)));
        assert_eq!(
            fn_006ea510(
                &mut e,
                Ptr::new(task),
                Ptr::new(0x7300),
                Ptr::new(0x7400),
                0
            ),
            0
        );
        assert_eq!(
            fn_006ea780(
                &mut e,
                Ptr::new(task),
                Ptr::new(0x7300),
                Ptr::new(0x7400),
                0,
                1
            ),
            0
        );
    }

    #[test]
    fn raycast_builder_reads_the_request_array_and_builds_nonempty_path() {
        let mut e = Engine::new();
        let task = 0x7500;
        let request = 0x7600;
        let path = 0x7700;
        let request_slot = 0x7800;
        let path_slot = 0x7810;
        e.mem.map(task, 0x80);
        e.mem.map(request, 0x68);
        e.mem.map(path, 0x18);
        e.mem.map(request_slot, 4);
        e.mem.map(path_slot, 4);
        e.mem.set_u32(request_slot, request);
        e.mem.set_u32(path_slot, path);
        let lock_entries = Rc::new(Cell::new(0));
        let lock_entries_by_double = lock_entries.clone();
        e.register_double(0x0040_fbf0, move |_, args| {
            assert_eq!(args[0], task + 0x20);
            lock_entries_by_double.set(lock_entries_by_double.get() + 1);
            Ret::default()
        });
        e.register(0x0040_fba0, |_, _| Ret::default());
        e.register_double(0x0044_ddc0, move |_, args| {
            assert_eq!(args[0], task + 0x40);
            value(1)
        });
        e.register_double(0x0087_7a30, move |_, args| {
            assert_eq!(args[1], 0);
            value(if args[0] == task + 0x40 {
                request_slot
            } else {
                assert_eq!(args[0], task + 0x50);
                path_slot
            })
        });
        let constructed = Rc::new(Cell::new(false));
        let constructed_by_double = constructed.clone();
        e.register_double(0x006e_61a0, move |_, args| {
            assert_eq!(args.len(), 1);
            constructed_by_double.set(true);
            Ret::default()
        });
        let built = Rc::new(Cell::new(false));
        let built_by_double = built.clone();
        e.register_double(0x006e_6320, move |_, args| {
            assert_eq!(args.len(), 3);
            assert_eq!(args[1], request);
            assert_eq!(args[2], path);
            built_by_double.set(true);
            Ret::default()
        });
        e.register(0x004f_d3e0, |_, _| value(3));
        e.register(0x006e_62b0, |_, _| Ret::default());

        assert_eq!(fn_006ea350(&mut e, Ptr::new(task)), 3);
        assert_eq!(lock_entries.get(), 1);
        assert!(constructed.get());
        assert!(built.get());
    }

    #[test]
    fn manager_lifecycle_construct_initialize_and_destroy_paths_execute() {
        let mut e = Engine::new();
        let manager = 0x8000;
        e.mem.map(manager, 0x1500);
        noops(
            &mut e,
            &[
                0x0044_dee0,
                0x006e_c000,
                0x006e_c090,
                0x00aa_5fc0,
                0x0044_fae0,
                0x006e_c340,
                0x006e_c660,
                0x006e_c870,
                0x006e_c3f0,
                0x0044_fb00,
                0x006e_c130,
                0x006e_bd30,
                0x006c_09d0,
                0x00b0_0c20,
            ],
        );
        e.register(0x00b0_0a00, |_, _| value(0x9000));
        e.register(0x00b0_0a80, |_, _| value(1));
        e.register(0x006c_08e0, |_, _| value(1));
        path_manager_impl_construct(&mut e, Ptr::new(manager));
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_STATE), 0);
        assert!(path_manager_impl_initialize(&mut e, Ptr::new(manager)));
        path_manager_impl_destroy(&mut e, Ptr::new(manager));
        path_manager_impl_destruct(&mut e, Ptr::new(manager));
    }

    #[test]
    fn initialize_failure_does_not_reset_state() {
        let mut e = Engine::new();
        let manager = 0xa000;
        e.mem.map(manager, 0x1500);
        e.mem.set_u32(manager + PATH_MANAGER_IMPL_STATE, 3);
        e.register(0x00b0_0a00, |_, _| value(0x9000));
        e.register(0x00b0_0a80, |_, _| Ret::default());
        assert!(!path_manager_impl_initialize(&mut e, Ptr::new(manager)));
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_STATE), 3);
    }

    #[test]
    fn request_lookup_misses_return_status_two_for_both_modes() {
        let mut e = Engine::new();
        let manager = 0xb000;
        let map = manager + 0x1480;
        let vtable = 0x0019_0000;
        const LOOKUP: u32 = 0x0077_3000;
        e.mem.map(manager, 0x1500);
        e.mem.set_u32(map, vtable);
        e.put_vtable(vtable, &[0, 0, LOOKUP]);
        e.register(LOOKUP, |_, _| Ret::default());
        assert_eq!(
            fn_006eb250(
                &mut e,
                Ptr::new(manager),
                9,
                Ptr::new(0x10),
                Ptr::new(0x20),
                0,
                1
            ),
            2
        );
        assert_eq!(
            fn_006eb2b0(
                &mut e,
                Ptr::new(manager),
                9,
                Ptr::new(0x10),
                Ptr::new(0x20),
                0
            ),
            2
        );
    }

    #[test]
    fn update_advances_pause_retry_and_exits_empty_queues() {
        let mut e = Engine::new();
        let manager = 0xc000;
        e.mem.map(manager, 0x1500);
        e.mem.set_u32(manager + PATH_MANAGER_IMPL_STATE, 1);
        let queue_vtable = 0x0019_1000;
        const POP: u32 = 0x0077_3040;
        for offset in [0x0c10, 0x0c50, 0x1064] {
            e.mem.set_u32(manager + offset, queue_vtable);
        }
        e.put_vtable(queue_vtable, &[0, 0, 0, 0, POP]);
        e.register(POP, |_, _| Ret::default());
        e.register(0x0045_38a0, |_, _| Ret::default());
        e.register(0x0045_38c0, |_, _| Ret::default());
        e.register(0x00aa_6220, |_, _| Ret::default());
        e.register(0x006e_c970, |_, _| Ret::default());
        e.register(0x006e_bde0, |_, _| value(1));
        noops(
            &mut e,
            &[0x00aa_62e0, 0x006e_bdc0, 0x0045_cec0, 0x0048_3710],
        );
        fn_006eae40(&mut e, Ptr::new(manager));
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_STATE), 1);
        assert_eq!(e.mem.u32(manager + PATH_MANAGER_IMPL_PAUSE_RETRIES), 1);
    }

    #[test]
    fn request_message_routes_to_selected_embedded_queue() {
        let mut e = Engine::new();
        let manager = 0xd000;
        e.mem.map(manager, 0x1500);
        const PUSH: u32 = 0x0077_3010;
        let push_calls = Rc::new(Cell::new(0));
        let calls = push_calls.clone();
        for offset in [0x0c50, 0x1064] {
            let queue = manager + offset;
            let table = 0x0019_2000 + offset;
            e.mem.set_u32(queue, table);
            e.put_vtable(table, &[0, PUSH]);
        }
        e.register_double(PUSH, move |_, args| {
            assert_eq!(args.len(), 2);
            if args[1] == 0x1111 {
                assert_eq!(args[0], manager + 0x1064);
            } else {
                assert_eq!(args[1], 0x2222);
                assert_eq!(args[0], manager + 0x0c50);
            }
            calls.set(calls.get() + 1);
            value(0)
        });
        assert!(fn_006eb200(&mut e, Ptr::new(manager), 0x1111, 0));
        assert!(fn_006eb200(&mut e, Ptr::new(manager), 0x2222, 1));
        assert_eq!(push_calls.get(), 2);
    }

    #[test]
    fn process_cancellation_flag_false_marks_task_data_for_retry() {
        let mut e = Engine::new();
        let task = 0xe000;
        let context = 0xe100;
        e.mem.map(task, 0x80);
        e.mem.map(context, 0x0d00);
        e.register(0x0040_4eb0, |_, _| Ret::default());
        e.register(0x0040_4ee0, |_, _| Ret::default());
        e.register_double(0x0047_d0b0, move |_, _| value(context));
        e.register_double(0x0071_7e50, move |_, _| value(context));
        e.register_double(0x0055_9450, move |_, _| value(context));
        e.register(0x0048_3710, |_, _| Ret::default());
        e.register(0x00aa_6120, |_, _| Ret::default());
        pathing_task_data_process(&mut e, Ptr::new(task));
        assert_eq!(e.mem.u8(task + 4), 1);
    }

    #[test]
    fn process_incomplete_signaled_path_yields_and_signals_pause_event() {
        let mut e = Engine::new();
        let task = 0xe800;
        let manager = 0xe900;
        let path = 0xea00;
        let path_slot = 0xeb00;
        e.mem.map(task, 0x80);
        e.mem.map(manager, 0x1500);
        e.mem.map(path, 0x18);
        e.mem.map(path_slot, 4);
        e.mem.set_u32(path_slot, path);
        e.register(0x0040_4eb0, |_, _| Ret::default());
        e.register(0x0040_4ee0, |_, _| Ret::default());
        e.register_double(0x0047_d0b0, move |_, _| value(manager));
        e.register(0x0071_7e50, |_, args| value(args[0]));
        e.register(0x0048_3710, |_, _| Ret::default());
        e.register(0x0055_9450, |_, args| value(args[0]));
        e.register(0x00aa_6120, |_, _| value(1));
        let signal_count = Rc::new(Cell::new(0));
        let signal_count_for_double = signal_count.clone();
        e.register_double(0x00aa_62c0, move |_, args| {
            assert_eq!(args[0], manager + 0x0c2c);
            signal_count_for_double.set(signal_count_for_double.get() + 1);
            Ret::default()
        });
        e.register(0x0040_fbf0, |_, _| Ret::default());
        e.register(0x0040_fba0, |_, _| Ret::default());
        e.register_double(0x0044_ddc0, move |_, args| {
            if args[0] == task + 0x50 {
                value(1)
            } else {
                value(0)
            }
        });
        e.register_double(0x0087_7a30, move |_, args| {
            assert_eq!(args[0], task + 0x50);
            value(path_slot)
        });

        pathing_task_data_process(&mut e, Ptr::new(task));
        assert_eq!(e.mem.u8(task + 4), 1);
        assert_eq!(signal_count.get(), 1);
    }

    #[test]
    fn cancel_matching_request_clears_both_cancellation_words() {
        let mut e = Engine::new();
        let manager = 0xf000;
        let task_data = 0x11000;
        let cancel_data = task_data + 0x10;
        e.mem.map(manager, 0x1500);
        e.mem.map(task_data, 0x80);
        e.mem.set_u32(cancel_data, 0x5566);
        e.mem.set_u32(cancel_data + 4, 0x7788);
        e.register(0x0045_38a0, |_, _| Ret::default());
        e.register(0x0045_38c0, |_, _| Ret::default());
        e.register(0x006e_c970, |_, _| Ret::default());
        let done = Rc::new(Cell::new(false));
        let done_for_pop = done.clone();
        e.register_double(
            0x006e_bde0,
            move |_, _| if done.get() { value(1) } else { value(0) },
        );
        e.register_double(0x0044_c640, move |e, args| {
            if !done_for_pop.replace(true) {
                e.mem.set_u32(args[2], task_data);
                value(1)
            } else {
                Ret::default()
            }
        });
        e.register(0x006c_9900, |_, args| value(args[0] + 0x10));
        noops(&mut e, &[0x006e_bdc0]);
        fn_006eb6b0(&mut e, Ptr::new(manager), 0x7788);
        assert_eq!(e.mem.u32(cancel_data), 0);
        assert_eq!(e.mem.u32(cancel_data + 4), 0);
    }

    #[test]
    fn cancel_token_wrapper_and_debug_message_lifecycle_execute() {
        let mut e = Engine::new();
        let holder = 0x12000;
        let token = 0x12100;
        let debug = 0x12200;
        e.mem.map(holder, 4);
        e.mem.map(token, 4);
        e.mem.map(debug, 8);
        e.mem.set_u32(holder, token);
        let table = 0x0019_3000;
        const CANCEL: u32 = 0x0077_3020;
        e.mem.set_u32(token, table);
        e.put_vtable(table, &[0, CANCEL, 0, 0, 0]);
        e.register(0x004f_1540, |_, _| Ret::default());
        let cancelled = Rc::new(Cell::new(false));
        let cancelled_by_double = cancelled.clone();
        e.register_double(CANCEL, move |_, args| {
            assert_eq!(args[1], 2);
            cancelled_by_double.set(true);
            Ret::default()
        });
        fn_006eb670(&mut e, Ptr::new(holder), 2);
        let stopped = Rc::new(Cell::new(false));
        let stopped_by_double = stopped.clone();
        e.register_double(0x006e_b890, move |_, args| {
            assert_eq!(args[0], 0x1234);
            assert_eq!(args[1], 1);
            stopped_by_double.set(true);
            Ret::default()
        });
        noops(&mut e, &[0x0045_d260]);
        fn_006eb820(&mut e, Ptr::new(debug));
        assert_eq!(e.mem.u32(debug), 0);
        e.mem.set_u32(debug, 0x1234);
        fn_006eb850(&mut e, Ptr::new(debug));
        assert!(cancelled.get());
        assert!(stopped.get());
    }

    #[test]
    fn task_queue_and_pause_event_helpers_call_their_observed_slots() {
        let mut e = Engine::new();
        let manager = 0x13000;
        e.mem.map(manager, 0x1500);
        let queue_table = 0x0019_4000;
        const PUSH: u32 = 0x0077_3030;
        e.mem.set_u32(manager + 0x0c10, queue_table);
        e.put_vtable(queue_table, &[0, PUSH]);
        let queue_calls = Rc::new(Cell::new(0));
        let queue_calls_for_double = queue_calls.clone();
        e.register_double(PUSH, move |_, args| {
            assert_eq!(args[1], 0x9090);
            queue_calls_for_double.set(1);
            Ret::default()
        });
        let event_calls = Rc::new(Cell::new(0));
        let event_calls_for_double = event_calls.clone();
        e.register_double(0x00aa_62c0, move |_, args| {
            assert_eq!(args[0], manager + 0x0c2c);
            event_calls_for_double.set(1);
            Ret::default()
        });
        fn_006eb7f0(&mut e, Ptr::new(manager), 0x9090);
        fn_006eb7b0(&mut e, Ptr::new(manager));
        assert_eq!(queue_calls.get(), 1);
        assert_eq!(event_calls.get(), 1);
    }
}

#[cfg(test)]
mod continuation_tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};
    const OBJECT: u32 = 0x20000;
    const OWNER: u32 = 0x22000;
    fn result(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }
    fn noops(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }
    fn fixture() -> Engine {
        let mut e = Engine::new();
        e.mem.map(OBJECT, 0x4000);
        e.mem.set_u32(OWNER, OBJECT);
        e.call_log = Some(vec![]);
        for address in [
            0x0040_1030,
            0x0040_4eb0,
            0x0040_4ee0,
            0x0044_dee0,
            0x00aa_5fc0,
            0x0044_fae0,
            0x006e_c660,
            0x006e_c870,
            0x0044_fb00,
            0x0048_3710,
            0x006c_09d0,
            0x00b0_0c20,
            0x0045_38a0,
            0x0045_38c0,
            0x00aa_6220,
            0x00aa_62e0,
            0x0045_cec0,
            0x0063_3c90,
            0x0044_c480,
            0x00aa_6060,
            0x006e_cc40,
            0x00ec_782f,
            0x006e_d040,
            0x006e_cb40,
            0x006e_cb70,
            0x006e_c410,
            0x006e_c390,
            0x006e_c1c0,
            0x00ec_5fce,
            0x006e_c1e0,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.register_double(0x006e_c970, move |e, a| {
            e.mem.set_u8(a[0] + 12, 2);
            Ret::default()
        });
        e.register_double(0x00b0_0a00, move |_, _| result(0x24000));
        e.register_double(0x00b0_0a80, move |_, _| result(1));
        e.register_double(0x006c_08e0, move |_, _| result(1));
        e.register_double(0x0077_0000, move |_, _| Ret::default());
        e.put_vtable(0x30000, &[0, 0x0077_0000, 0x0077_0000, 0, 0x0077_0000]);
        for offset in [0xc10, 0xc50, 0x1064, 0x1480] {
            e.mem.set_u32(OBJECT + offset, 0x30000);
        }
        e
    }
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }
    #[test]
    fn delete_flags_006eb890() {
        for flags in [0, 2, 1, 3] {
            let mut e = fixture();
            assert_eq!(fn_006eb890(&mut e, Ptr::new(OBJECT), flags).addr(), OBJECT);
            assert_eq!(
                calls(&e, 0x0040_1030),
                if flags & 1 != 0 {
                    vec![vec![OBJECT]]
                } else {
                    vec![]
                }
            );
        }
    }

    #[test]
    fn delete_flags_006ebe00() {
        for flags in [0, 2, 1, 3] {
            let mut e = fixture();
            assert_eq!(fn_006ebe00(&mut e, Ptr::new(OBJECT), flags).addr(), OBJECT);
            assert_eq!(
                calls(&e, 0x0040_1030),
                if flags & 1 != 0 {
                    vec![vec![OBJECT]]
                } else {
                    vec![]
                }
            );
        }
    }

    #[test]
    fn delete_flags_006ebe30() {
        for flags in [0, 2, 1, 3] {
            let mut e = fixture();
            assert_eq!(fn_006ebe30(&mut e, Ptr::new(OBJECT), flags).addr(), OBJECT);
            assert_eq!(
                calls(&e, 0x0040_1030),
                if flags & 1 != 0 {
                    vec![vec![OBJECT]]
                } else {
                    vec![]
                }
            );
        }
    }

    #[test]
    fn delete_flags_006ebe60() {
        for flags in [0, 2, 1, 3] {
            let mut e = fixture();
            assert_eq!(fn_006ebe60(&mut e, Ptr::new(OBJECT), flags).addr(), OBJECT);
            assert_eq!(
                calls(&e, 0x0040_1030),
                if flags & 1 != 0 {
                    vec![vec![OBJECT]]
                } else {
                    vec![]
                }
            );
        }
    }

    #[test]
    fn delete_flags_006ec230() {
        for flags in [0, 2, 1, 3] {
            let mut e = fixture();
            assert_eq!(fn_006ec230(&mut e, Ptr::new(OBJECT), flags).addr(), OBJECT);
            assert_eq!(
                calls(&e, 0x0040_1030),
                if flags & 1 != 0 {
                    vec![vec![OBJECT]]
                } else {
                    vec![]
                }
            );
        }
    }

    #[test]
    fn initialize_allocation_success_and_null_failure() {
        for allocation in [OBJECT, 0] {
            let mut e = fixture();
            e.register_double(0x0040_1000, move |_, a| {
                assert_eq!(a, &[0x14c0]);
                result(allocation)
            });
            if allocation == 0 {
                e.register_double(0x00b0_0a80, move |_, _| result(0));
            }
            assert_eq!(fn_006eb8c0(&mut e, Ptr::new(OWNER)), allocation != 0);
            assert_eq!(e.mem.u32(OWNER), allocation);
            let guards = calls(&e, 0x0040_4eb0);
            assert_eq!(&guards[0][1..], &[0x2d, 1, 0x0106_d560, 0x56a]);
            assert_eq!(calls(&e, 0x0040_4ee0), vec![vec![guards[0][0]]]);
        }
    }

    #[test]
    fn destroy_reloads_owner_after_stop_callback() {
        let mut e = fixture();
        e.register_double(0x006c_09d0, move |e, a| {
            assert_eq!(a, &[OBJECT]);
            e.mem.set_u32(OWNER, 0);
            Ret::default()
        });
        fn_006eb980(&mut e, Ptr::new(OWNER));
        assert_eq!(e.mem.u32(OWNER), 0);
        assert!(calls(&e, 0x0040_1030).is_empty());
        e.mem.set_u32(OWNER, OBJECT);
        e.register_double(0x006c_09d0, move |_, _| Ret::default());
        fn_006eb980(&mut e, Ptr::new(OWNER));
        assert_eq!(calls(&e, 0x0040_1030), vec![vec![OBJECT]]);
        assert_eq!(e.mem.u32(OWNER), 0);
    }

    #[test]
    fn route_wrapper_preserves_mode_and_return() {
        let mut e = fixture();
        for mode in [0, 1, 255] {
            assert!(fn_006ebbd0(&mut e, Ptr::new(OWNER), 0x1234, mode));
        }
        assert_eq!(
            calls(&e, 0x0077_0000),
            vec![
                vec![OBJECT + 0x1064, 0x1234],
                vec![OBJECT + 0xc50, 0x1234],
                vec![OBJECT + 0xc50, 0x1234]
            ]
        );
    }

    #[test]
    fn lookup_wrapper_006ebbf0() {
        let mut e = fixture();
        assert_eq!(
            fn_006ebbf0(
                &mut e,
                Ptr::new(OWNER),
                9,
                Ptr::new(0x100),
                Ptr::new(0x200),
                0x300,
                7
            ),
            2
        );
        let lookup = calls(&e, 0x0077_0000);
        assert_eq!(&lookup[0][..2], &[OBJECT + 0x1480, 9]);
    }

    #[test]
    fn lookup_wrapper_006ebc20() {
        let mut e = fixture();
        assert_eq!(
            fn_006ebc20(
                &mut e,
                Ptr::new(OWNER),
                9,
                Ptr::new(0x100),
                Ptr::new(0x200),
                0x300
            ),
            2
        );
        let lookup = calls(&e, 0x0077_0000);
        assert_eq!(&lookup[0][..2], &[OBJECT + 0x1480, 9]);
    }

    #[test]
    fn update_wrapper_advances_retry() {
        let mut e = fixture();
        e.mem.set_u32(OBJECT + PATH_MANAGER_IMPL_STATE, 1);
        fn_006ebc50(&mut e, Ptr::new(OWNER));
        assert_eq!(e.mem.u32(OBJECT + PATH_MANAGER_IMPL_PAUSE_RETRIES), 1);
    }

    #[test]
    fn state_wrapper_006ebc70() {
        let mut e = fixture();
        e.mem.set_u32(OBJECT + PATH_MANAGER_IMPL_STATE, 0);
        fn_006ebc70(&mut e, Ptr::new(OWNER));
        assert_eq!(e.mem.u32(OBJECT + PATH_MANAGER_IMPL_STATE), 1);
    }

    #[test]
    fn state_wrapper_006ebcd0() {
        let mut e = fixture();
        e.mem.set_u32(OBJECT + PATH_MANAGER_IMPL_STATE, 2);
        fn_006ebcd0(&mut e, Ptr::new(OWNER));
        assert_eq!(e.mem.u32(OBJECT + PATH_MANAGER_IMPL_STATE), 3);
    }

    #[test]
    fn state_wrapper_006ebcf0() {
        let mut e = fixture();
        e.mem.set_u32(OBJECT + PATH_MANAGER_IMPL_STATE, 0);
        fn_006ebcf0(&mut e, Ptr::new(OWNER));
        assert_eq!(e.mem.u32(OBJECT + PATH_MANAGER_IMPL_STATE), 2);
    }

    #[test]
    fn query_wrapper_006ebc90() {
        let mut e = fixture();
        for state in 0..4 {
            e.mem.set_u32(OBJECT + PATH_MANAGER_IMPL_STATE, state);
            assert_eq!(fn_006ebc90(&mut e, Ptr::new(OWNER)), state == 2);
        }
    }

    #[test]
    fn query_wrapper_006ebcb0() {
        let mut e = fixture();
        for state in 0..4 {
            e.mem.set_u32(OBJECT + PATH_MANAGER_IMPL_STATE, state);
            assert_eq!(fn_006ebcb0(&mut e, Ptr::new(OWNER)), state == 0);
        }
    }

    #[test]
    fn clear_wrapper_locks_implementation() {
        let mut e = fixture();
        fn_006ebd10(&mut e, Ptr::new(OWNER), 0x1234);
        assert_eq!(
            calls(&e, 0x0045_38a0),
            vec![vec![OBJECT + 0xc38, 0x0106_d5f8]]
        );
        assert_eq!(calls(&e, 0x0045_38c0), vec![vec![OBJECT + 0xc38]]);
    }

    #[test]
    fn pool_destructor_uses_exact_vector_extent() {
        let mut e = fixture();
        fn_006ebd30(&mut e, Ptr::new(OBJECT));
        assert_eq!(
            calls(&e, 0x00ec_5fce),
            vec![vec![OBJECT + 12, 12, 256, 0x0048_3710]]
        );
        assert_eq!(e.mem.u32(OBJECT), 0x0106_d63c);
    }

    #[test]
    fn base_vtable_006ebda0() {
        let mut e = fixture();
        e.mem.set_u32(OBJECT + 4, 0xaabbccdd);
        fn_006ebda0(&mut e, Ptr::new(OBJECT));
        assert_eq!(e.mem.u32(OBJECT), 0x0106_d63c);
        assert_eq!(e.mem.u32(OBJECT + 4), 0xaabbccdd);
    }

    #[test]
    fn base_vtable_006ebdc0() {
        let mut e = fixture();
        e.mem.set_u32(OBJECT + 4, 0xaabbccdd);
        fn_006ebdc0(&mut e, Ptr::new(OBJECT));
        assert_eq!(e.mem.u32(OBJECT), 0x0106_d644);
        assert_eq!(e.mem.u32(OBJECT + 4), 0xaabbccdd);
    }

    #[test]
    fn iterator_exhaustion_reads_only_bit_one() {
        let mut e = fixture();
        for byte in [0, 1, 2, 3, 128, 255] {
            e.mem.set_u8(OBJECT + 12, byte);
            assert_eq!(fn_006ebde0(&mut e, Ptr::new(OBJECT)), byte & 2 != 0);
        }
    }

    #[test]
    fn array_lifecycle_006ebe90() {
        let mut e = fixture();
        e.register_double(0x006b_3eb0, move |e, a| {
            assert_eq!(a, &[OBJECT, 0, 0]);
            assert_eq!(e.mem.u32(OBJECT), 0x0106_d650);
            Ret::default()
        });
        fn_006ebe90(&mut e, Ptr::new(OBJECT));
        assert_eq!(calls(&e, 0x006b_3eb0).len(), 1);
    }

    #[test]
    fn array_lifecycle_006ebec0() {
        let mut e = fixture();
        e.register_double(0x0084_54f0, move |e, a| {
            assert_eq!(a, &[OBJECT, 1]);
            assert_eq!(e.mem.u32(OBJECT), 0x0106_d650);
            Ret::default()
        });
        fn_006ebec0(&mut e, Ptr::new(OBJECT));
        assert_eq!(calls(&e, 0x0084_54f0).len(), 1);
    }

    #[test]
    fn array_lifecycle_006ebee0() {
        let mut e = fixture();
        e.register_double(0x006b_3eb0, move |e, a| {
            assert_eq!(a, &[OBJECT, 0, 0]);
            assert_eq!(e.mem.u32(OBJECT), 0x0106_d664);
            Ret::default()
        });
        fn_006ebee0(&mut e, Ptr::new(OBJECT));
        assert_eq!(calls(&e, 0x006b_3eb0).len(), 1);
    }

    #[test]
    fn array_lifecycle_006ebf10() {
        let mut e = fixture();
        e.register_double(0x0084_54f0, move |e, a| {
            assert_eq!(a, &[OBJECT, 1]);
            assert_eq!(e.mem.u32(OBJECT), 0x0106_d664);
            Ret::default()
        });
        fn_006ebf10(&mut e, Ptr::new(OBJECT));
        assert_eq!(calls(&e, 0x0084_54f0).len(), 1);
    }

    #[test]
    fn token_assignment_reloads_source_after_release_and_handles_alias() {
        let mut e = fixture();
        e.mem.set_u32(OBJECT, 0x50000);
        e.mem.set_u32(OBJECT + 4, 0x60000);
        e.register_double(0x0040_1970, move |e, a| {
            assert_eq!(a, &[0x50014]);
            e.mem.set_u32(OBJECT + 4, 0x70000);
            Ret::default()
        });
        e.register_double(0x0040_f6e0, move |_, a| {
            assert_eq!(a, &[0x70014]);
            Ret::default()
        });
        assert_eq!(
            fn_006ebfb0(&mut e, Ptr::new(OBJECT), Ptr::new(OBJECT + 4)).addr(),
            OBJECT
        );
        assert_eq!(e.mem.u32(OBJECT), 0x70000);
        fn_006ebfb0(&mut e, Ptr::new(OBJECT), Ptr::new(OBJECT));
        assert_eq!(calls(&e, 0x0040_1970).len(), 1);
        e.mem.set_u32(OBJECT, 0);
        e.mem.set_u32(OBJECT + 4, 0);
        fn_006ebfb0(&mut e, Ptr::new(OBJECT), Ptr::new(OBJECT + 4));
        assert_eq!(calls(&e, 0x0040_f6e0).len(), 1);
    }

    #[test]
    fn pool_constructor_supplies_element_ctor_and_dtor() {
        let mut e = fixture();
        assert_eq!(fn_006ec000(&mut e, Ptr::new(OBJECT)).addr(), OBJECT);
        assert_eq!(
            calls(&e, 0x00ec_782f),
            vec![vec![OBJECT + 12, 12, 256, 0x006e_d0e0, 0x0048_3710]]
        );
        assert_eq!(calls(&e, 0x006e_d040), vec![vec![OBJECT, 256, OBJECT + 12]]);
        assert_eq!(e.mem.u32(OBJECT), 0x0106_d634);
    }

    #[test]
    fn linked_queue_constructor_initializes_empty_tail() {
        let mut e = fixture();
        assert_eq!(
            fn_006ec090(&mut e, Ptr::new(OBJECT), 0x50000).addr(),
            OBJECT
        );
        assert_eq!(e.mem.u32(OBJECT), 0x0106_d678);
        assert_eq!(e.mem.u32(OBJECT + 8), 0x50000);
        assert_eq!(e.mem.u32(OBJECT + 12), 0);
        assert_eq!(e.mem.u32(OBJECT + 16), OBJECT + 12);
    }

    #[test]
    fn linked_destructor_drains_until_false_before_cleanup() {
        let mut e = fixture();
        let pops = Rc::new(RefCell::new(0));
        let observed = pops.clone();
        e.register_double(0x006e_c390, move |_, a| {
            assert_eq!(a[0], OBJECT);
            let mut n = observed.borrow_mut();
            *n += 1;
            result(u32::from(*n < 3))
        });
        fn_006ec130(&mut e, Ptr::new(OBJECT));
        assert_eq!(*pops.borrow(), 3);
        assert_eq!(calls(&e, 0x0048_3710).len(), 1);
        assert_eq!(calls(&e, 0x006e_c1c0), vec![vec![OBJECT]]);
    }

    #[test]
    fn linked_push_handles_missing_pool_failure_empty_and_nonempty() {
        let mut e = fixture();
        assert!(!fn_006ec260(&mut e, Ptr::new(OBJECT), 11));
        e.mem.set_u32(OBJECT + 8, 0x50000);
        e.register_double(0x006e_cd00, move |_, _| result(0));
        assert!(!fn_006ec260(&mut e, Ptr::new(OBJECT), 11));
        e.register_double(0x006e_cd00, move |e, a| {
            assert_eq!(a[0], 0x50000);
            e.mem.set_u32(a[1], a[2]);
            result(1)
        });
        assert!(fn_006ec260(&mut e, Ptr::new(OBJECT), OBJECT + 0x100));
        assert_eq!(e.mem.u32(OBJECT + 12), OBJECT + 0x100);
        assert!(fn_006ec260(&mut e, Ptr::new(OBJECT), OBJECT + 0x200));
        assert_eq!(e.mem.u32(OBJECT + 0x108), OBJECT + 0x200);
        assert_eq!(e.mem.u32(OBJECT + 16), OBJECT + 0x208);
        assert_eq!(e.mem.u32(OBJECT + 0x208), 0);
    }

    #[test]
    fn linked_pop_copies_payload_then_releases_node() {
        let mut e = fixture();
        assert!(!fn_006ec2e0(
            &mut e,
            Ptr::new(OBJECT),
            Ptr::new(OBJECT + 0x200)
        ));
        e.mem.set_u32(OBJECT + 8, 0x50000);
        e.mem.set_u32(OBJECT + 12, OBJECT + 0x100);
        e.mem.set_u32(OBJECT + 0x100, 7);
        e.mem.set_u32(OBJECT + 0x104, 0xabcdef01);
        e.mem.set_u32(OBJECT + 0x108, OBJECT + 0x180);
        e.register_double(0x0068_15c0, move |_, a| result(a[0]));
        e.register_double(0x006e_cb10, move |e, a| {
            assert_eq!(a[0], 0x50000);
            assert_eq!(e.mem.u32(a[1]), OBJECT + 0x100);
            assert_eq!(e.mem.u32(OBJECT + 12), OBJECT + 0x180);
            Ret::default()
        });
        assert!(fn_006ec2e0(
            &mut e,
            Ptr::new(OBJECT),
            Ptr::new(OBJECT + 0x200)
        ));
        assert_eq!(e.mem.u32(OBJECT + 0x200), 7);
        assert_eq!(e.mem.u32(OBJECT + 0x204), 0xabcdef01);
    }

    #[test]
    fn static_queue_constructor_resets_all_three_words() {
        let mut e = fixture();
        for offset in [0x408, 0x40c, 0x410] {
            e.mem.set_u32(OBJECT + offset, 99);
        }
        assert_eq!(fn_006ec340(&mut e, Ptr::new(OBJECT)).addr(), OBJECT);
        assert_eq!(e.mem.u32(OBJECT), 0x0106_d6d0);
        for offset in [0x408, 0x40c, 0x410] {
            assert_eq!(e.mem.u32(OBJECT + offset), 0);
        }
    }

    #[test]
    fn static_queue_destructor_sets_vtable_before_base_call() {
        let mut e = fixture();
        e.register_double(0x006e_c410, move |e, a| {
            assert_eq!(a, &[OBJECT]);
            assert_eq!(e.mem.u32(OBJECT), 0x0106_d6d0);
            Ret::default()
        });
        fn_006ec3f0(&mut e, Ptr::new(OBJECT));
        assert_eq!(calls(&e, 0x006e_c410).len(), 1);
    }

    #[test]
    fn static_push_full_wrap_null_placement_and_callback_reload() {
        let mut e = fixture();
        e.mem.set_u32(OBJECT + 0x408, 256);
        assert!(!fn_006ec4b0(&mut e, Ptr::new(OBJECT), 0x50000));
        assert!(calls(&e, 0x006e_6da0).is_empty());
        e.mem.set_u32(OBJECT + 0x408, 255);
        e.mem.set_u32(OBJECT + 0x40c, 255);
        e.register_double(0x006e_6da0, move |_, a| {
            assert_eq!(a, &[4, OBJECT + 8 + 255 * 4]);
            result(0)
        });
        assert!(fn_006ec4b0(&mut e, Ptr::new(OBJECT), 0x50000));
        assert_eq!(e.mem.u32(OBJECT + 0x408), 256);
        assert_eq!(e.mem.u32(OBJECT + 0x40c), 0);
        e.mem.set_u32(OBJECT + 0x408, 0);
        e.register_double(0x006e_6da0, move |_, a| result(a[1]));
        e.register_double(0x0055_9a40, move |e, a| {
            assert_eq!(a, &[OBJECT + 8, 0x50000]);
            e.mem.set_u32(OBJECT + 0x408, 40);
            e.mem.set_u32(OBJECT + 0x40c, 99);
            Ret::default()
        });
        assert!(fn_006ec4b0(&mut e, Ptr::new(OBJECT), 0x50000));
        assert_eq!(e.mem.u32(OBJECT + 0x408), 41);
        assert_eq!(e.mem.u32(OBJECT + 0x40c), 100);
    }
    #[test]
    fn task_submission_passes_pointer_values_and_owns_argument_slots() {
        // 006eaccb passes the request slot; 006ead05 passes the data value.
        // 006ead4c reloads the key, and 006e9d67 copies the token argument.
        for accepted in [false, true] {
            let mut e = fixture();
            const DATA: u32 = 0x25000;
            const REQUEST: u32 = 0x50000;
            const TOKEN: u32 = 0x60000;
            e.mem.map(DATA, 0x80);
            e.put_vtable(PATHING_TASK_DATA_VTABLE, &[0x0077_0050]);
            e.register_double(0x0077_0050, move |_, a| {
                assert_eq!(a, &[DATA, 1]);
                Ret::default()
            });
            e.register_double(0x0040_1000, move |_, a| {
                assert_eq!(a, &[0x80]);
                result(DATA)
            });
            for address in [
                0x006c_7850,
                0x0096_a2d0,
                0x006b_3eb0,
                0x006c_5e40,
                0x006c_5e70,
                0x0040_f6e0,
            ] {
                e.register_double(address, |_, _| Ret::default());
            }
            e.register_double(0x006e_bf60, move |e, a| {
                assert_eq!(a, &[DATA + 0x60, 0]);
                e.mem.set_u32(a[0], 0);
                result(a[0])
            });
            e.register_double(0x006e_bf30, move |e, a| {
                assert_ne!(a[1], TOKEN);
                assert_eq!(e.mem.u32(a[1]), TOKEN);
                e.mem.set_u32(a[0], e.mem.u32(a[1]));
                result(a[0])
            });
            e.register_double(0x006e_5cc0, move |e, a| {
                assert_eq!(a[0], DATA + 0x18);
                assert_eq!(e.mem.u32(a[1]), REQUEST);
                e.mem.set_u32(a[0], e.mem.u32(a[1]));
                result(a[0])
            });
            e.register_double(0x006e_cd40, move |_, a| {
                assert_eq!(a[1], DATA);
                Ret::default()
            });
            let request_slot = Rc::new(RefCell::new(0));
            let seen_slot = request_slot.clone();
            e.register_double(0x0055_9450, move |e, a| {
                assert_ne!(a[0], REQUEST);
                *seen_slot.borrow_mut() = a[0];
                result(e.mem.u32(a[0]))
            });
            e.put_vtable(0x30000, &[0, 0, 0, 0, 0x0077_0060, 0x0077_0070]);
            e.register_double(0x0077_0060, move |e, a| {
                assert_eq!(a[1], REQUEST);
                assert_eq!(e.mem.u32(a[2]), DATA);
                result(1)
            });
            e.register_double(0x0077_0070, move |_, a| {
                assert_eq!(a[1], REQUEST + 4);
                Ret::default()
            });
            e.register_double(0x006c_5ec0, move |e, _| {
                e.mem.set_u32(*request_slot.borrow(), REQUEST + 4);
                result(u32::from(accepted))
            });
            e.register_double(0x0045_cec0, move |e, a| {
                assert_eq!(e.mem.u32(a[0]), REQUEST + 4);
                Ret::default()
            });
            e.register_double(0x006e_bf90, move |e, a| {
                assert_ne!(a[0], TOKEN);
                assert_eq!(e.mem.u32(a[0]), TOKEN);
                Ret::default()
            });
            assert_eq!(
                fn_006eac70(&mut e, Ptr::new(OBJECT), REQUEST, TOKEN),
                accepted
            );
            assert_eq!(e.mem.u32(DATA + 0x18), REQUEST);
            assert_eq!(e.mem.u32(DATA + 0x60), TOKEN);
            assert_eq!(calls(&e, 0x0077_0050).len(), usize::from(!accepted));
            assert_eq!(calls(&e, 0x006e_bf90).len(), 3);
        }
    }

    #[test]
    fn build_path_synchronous_gates_result_kind_and_owned_cleanup() {
        for (setting, mode) in [(0, 1), (1, 0)] {
            for built in [false, true] {
                for allocation in [0, 0x25000] {
                    let mut e = fixture();
                    e.mem.set_u8(OBJECT + 0x180, setting);
                    e.register_double(0x0040_8d60, move |_, a| {
                        assert_eq!(a, &[0x011d_77cc]);
                        result(OBJECT + 0x180)
                    });
                    e.register_double(0x00aa_13e0, move |_, a| {
                        assert_eq!(a, &[0x44]);
                        result(allocation)
                    });
                    e.register_double(0x006e_7650, move |_, a| {
                        assert_eq!(a, &[allocation]);
                        result(allocation)
                    });
                    e.register_double(0x0063_3c90, move |e, a| {
                        e.mem.set_u32(a[0], a[1]);
                        result(a[0])
                    });
                    e.register_double(0x0055_9450, move |e, a| result(e.mem.u32(a[0])));
                    e.register_double(0x0055_9a40, move |e, a| {
                        e.mem.set_u32(a[0], e.mem.u32(a[1]));
                        result(a[0])
                    });
                    e.register_double(0x006d_0900, move |_, a| {
                        assert_eq!(a, &[0x50000, allocation]);
                        result(u32::from(built))
                    });
                    e.mem.set_u32(OBJECT + 0x100, 0x31000);
                    e.put_vtable(0x31000, &[0, 0x0077_0080]);
                    e.register_double(0x0077_0080, move |e, a| {
                        assert_eq!(a[0], OBJECT + 0x100);
                        assert_eq!(e.mem.u32(a[1]), u32::from(!built));
                        assert_eq!(e.mem.u32(a[1] + 4), allocation);
                        Ret::default()
                    });
                    e.register_double(0x0080_0190, move |_, _| Ret::default());
                    e.register_double(0x006e_bf90, move |e, a| {
                        assert_eq!(e.mem.u32(a[0]), OBJECT + 0x100);
                        Ret::default()
                    });
                    assert!(fn_006eb9d0(
                        &mut e,
                        Ptr::new(OWNER),
                        0x50000,
                        OBJECT + 0x100,
                        mode
                    ));
                    assert_eq!(calls(&e, 0x0077_0080).len(), 1);
                    assert_eq!(calls(&e, 0x0045_cec0).len(), 2);
                    assert_eq!(calls(&e, 0x006e_bf90).len(), 1);
                    assert_eq!(calls(&e, 0x006e_7650).len(), usize::from(allocation != 0));
                }
            }
        }
    }

    #[test]
    fn build_path_asynchronous_copies_words_and_preserves_submission_result() {
        for accepted in [false, true] {
            let mut e = fixture();
            e.mem.set_u8(OBJECT + 0x180, 1);
            e.register_double(0x0040_8d60, move |_, _| result(OBJECT + 0x180));
            for address in [0x006e_bf30, 0x0055_9a40] {
                e.register_double(address, |e, a| {
                    e.mem.set_u32(a[0], e.mem.u32(a[1]));
                    result(a[0])
                });
            }
            e.register_double(0x0040_1000, move |_, a| {
                assert_eq!(a, &[0x80]);
                result(0)
            });
            e.register_double(0x006c_5e40, move |_, _| Ret::default());
            e.register_double(0x006c_5e70, move |_, _| Ret::default());
            e.register_double(0x006e_cd40, move |_, a| {
                assert_eq!(a[1], 0);
                Ret::default()
            });
            e.register_double(0x0055_9450, move |e, a| {
                assert_eq!(e.mem.u32(a[0]), 0x50000);
                result(e.mem.u32(a[0]))
            });
            e.put_vtable(0x30000, &[0, 0, 0, 0, 0x0077_0090, 0x0077_00a0]);
            e.register_double(0x0077_0090, move |e, a| {
                assert_eq!(a[1], 0x50000);
                assert_eq!(e.mem.u32(a[2]), 0);
                result(1)
            });
            e.register_double(0x0077_00a0, move |_, a| {
                assert_eq!(a[1], 0x50000);
                Ret::default()
            });
            e.register_double(0x006c_5ec0, move |_, _| result(u32::from(accepted)));
            e.register_double(0x0045_cec0, move |e, a| {
                assert_eq!(e.mem.u32(a[0]), 0x50000);
                Ret::default()
            });
            e.register_double(0x006e_bf90, move |e, a| {
                assert_eq!(e.mem.u32(a[0]), 0x60000);
                Ret::default()
            });
            assert_eq!(
                fn_006eb9d0(&mut e, Ptr::new(OWNER), 0x50000, 0x60000, 255),
                accepted
            );
            assert_eq!(calls(&e, 0x0045_cec0).len(), 2);
            assert_eq!(calls(&e, 0x006e_bf90).len(), 2);
            assert!(calls(&e, 0x00aa_13e0).is_empty());
        }
    }
    #[test]
    fn update_completion_uses_pointee_key_and_preserves_output_slot_order() {
        // 006eaf25/006eaf29: task output is EBP-0x34, the fourth ABI word;
        // 006eaffd loads the completion's second word before adding 0x18.
        let mut e = fixture();
        let popped = Rc::new(RefCell::new(false));
        let seen = popped.clone();
        e.register_double(0x006e_c970, move |_, _| Ret::default());
        let visited = Rc::new(RefCell::new(false));
        let predicate = visited.clone();
        e.register_double(0x006e_bde0, move |_, _| {
            result(u32::from(*predicate.borrow()))
        });
        e.register_double(0x0044_c640, move |e, a| {
            assert_eq!(a[0], OBJECT + 0x1480);
            e.mem.set_u32(a[2], 0xdeadbeef);
            e.mem.set_u32(a[3], OBJECT + 0x200);
            *visited.borrow_mut() = true;
            result(1)
        });
        e.register_double(0x005d_c980, move |_, a| {
            assert_eq!(a, &[OBJECT + 0x200]);
            result(0)
        });
        e.put_vtable(0x30000, &[0, 0, 0, 0, 0x0077_0100, 0x0077_0110]);
        e.register_double(0x0077_0100, move |e, a| {
            if a[0] == OBJECT + 0xc10 && !*seen.borrow() {
                *seen.borrow_mut() = true;
                e.mem.set_u32(a[1], 0);
                e.mem.set_u32(a[1] + 4, OBJECT + 0x100);
                e.mem.set_u32(a[1] + 0x18, 0xdeadbeef);
                result(1)
            } else {
                result(0)
            }
        });
        e.mem.set_u32(OBJECT + 0x118, 0x12345678);
        e.mem.set_u32(OBJECT + 0x100, 0x31000);
        e.put_vtable(0x31000, &[0x0077_0120]);
        e.register_double(0x0055_9450, move |e, a| {
            assert_eq!(a, &[OBJECT + 0x118]);
            result(e.mem.u32(a[0]))
        });
        e.register_double(0x0077_0110, move |_, a| {
            assert_eq!(a, &[OBJECT + 0x1480, 0x12345678]);
            Ret::default()
        });
        e.register_double(0x0077_0120, move |_, a| {
            assert_eq!(a, &[OBJECT + 0x100, 1]);
            Ret::default()
        });
        fn_006eae40(&mut e, Ptr::new(OBJECT));
        assert!(*popped.borrow());
        assert_eq!(calls(&e, 0x0077_0110).len(), 1);
        assert_eq!(calls(&e, 0x0077_0120).len(), 1);
    }

    #[test]
    fn process_nonempty_notifications_keep_pending_argument_off_singleton_call() {
        // At 006ea15c/006ea1e8/006ea2c6 PUSH EAX belongs to 006eb7f0,
        // not the intervening zero-argument singleton getter 0047d0b0.
        for (signaled, built) in [(true, true), (true, false), (false, false)] {
            let mut e = fixture();
            const TASK: u32 = 0x25000;
            const LISTENER: u32 = 0x26000;
            const FORWARDED: u32 = 0x27000;
            e.mem.map(TASK, 0x100);
            e.mem.map(LISTENER, 4);
            e.mem.set_u8(TASK + 4, 1);
            e.mem.set_u32(TASK + 0x18, 0x50000);
            e.mem.set_u32(TASK + 0x60, LISTENER);
            e.mem.set_u8(OBJECT + PATH_MANAGER_IMPL_CANCEL_PATHS, 1);
            e.register_double(0x0047_d0b0, move |_, a| {
                assert!(a.is_empty());
                result(OBJECT)
            });
            e.register_double(0x0071_7e50, move |_, a| {
                assert_eq!(a, &[OBJECT]);
                result(OBJECT)
            });
            e.register_double(0x0055_9450, move |e, a| {
                if a[0] == OBJECT {
                    result(OBJECT)
                } else {
                    result(e.mem.u32(a[0]))
                }
            });
            e.register_double(0x00aa_6120, move |_, _| result(u32::from(signaled)));
            e.register_double(0x00aa_62c0, move |_, _| Ret::default());
            for address in [
                0x0040_fbf0,
                0x0040_fba0,
                0x0044_ddc0,
                0x00aa_13e0,
                0x005d_c980,
                0x0080_0190,
            ] {
                e.register_double(address, |_, _| Ret::default());
            }
            e.register_double(0x0066_b0d0, move |e, a| {
                assert_eq!(a, &[TASK + 0x1c, 0]);
                e.mem.set_u32(a[0], a[1]);
                result(a[0])
            });
            e.register_double(0x006d_0900, move |_, a| {
                assert_eq!(a, &[0x50000, 0]);
                result(u32::from(built))
            });
            e.register_double(0x0055_9a40, move |e, a| {
                e.mem.set_u32(a[0], e.mem.u32(a[1]));
                result(a[0])
            });
            e.mem.set_u32(LISTENER, 0x31000);
            e.put_vtable(0x31000, &[0, 0x0077_0200]);
            e.register_double(0x0077_0200, move |e, a| {
                assert_eq!(a[0], LISTENER);
                assert_eq!(e.mem.u32(a[1]), u32::from(!built));
                Ret::default()
            });
            e.register_double(0x004b_0680, move |_, a| {
                assert_eq!(a[1..], [u32::from(!built), TASK]);
                result(FORWARDED)
            });
            e.register_double(0x0077_0000, move |_, a| {
                assert_eq!(a, &[OBJECT + 0xc10, FORWARDED]);
                Ret::default()
            });
            pathing_task_data_process(&mut e, Ptr::new(TASK));
            assert_eq!(calls(&e, 0x0077_0200).len(), 1);
            assert_eq!(calls(&e, 0x0077_0000).len(), 1);
            assert_eq!(calls(&e, 0x00aa_62c0).len(), usize::from(signaled));
            assert_eq!(e.mem.u8(TASK + 4), u8::from(!built));
        }
    }
    #[test]
    fn update_resubmits_completed_task_using_pointer_value() {
        let mut e = fixture();
        let visited = Rc::new(RefCell::new(false));
        let predicate = visited.clone();
        e.register_double(0x006e_c970, move |_, _| Ret::default());
        e.register_double(0x006e_bde0, move |_, _| {
            result(u32::from(*predicate.borrow()))
        });
        e.register_double(0x0044_c640, move |e, a| {
            e.mem.set_u32(a[2], 0xdeadbeef);
            e.mem.set_u32(a[3], OBJECT + 0x200);
            *visited.borrow_mut() = true;
            result(1)
        });
        e.register_double(0x005d_c980, move |_, a| {
            assert_eq!(a, &[OBJECT + 0x200]);
            result(1)
        });
        for address in [
            0x0040_fbf0,
            0x0040_fba0,
            0x0044_ddc0,
            0x006c_5e40,
            0x006c_5e70,
            0x006c_5ec0,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.register_double(0x0060_d4b0, move |_, a| {
            assert_eq!(a, &[OBJECT + 0x200, 0]);
            Ret::default()
        });
        e.register_double(0x006e_cd40, move |_, a| {
            assert_eq!(a[1], OBJECT + 0x200);
            Ret::default()
        });
        fn_006eae40(&mut e, Ptr::new(OBJECT));
        assert_eq!(calls(&e, 0x0060_d4b0).len(), 1);
        assert_eq!(calls(&e, 0x006e_cd40).len(), 1);
        assert_eq!(calls(&e, 0x006c_5ec0).len(), 1);
    }

    #[test]
    fn update_drains_both_change_queues_and_notifies_each_listener_mode() {
        for scene_mode in [false, true] {
            let mut e = fixture();
            const LISTENER: u32 = OBJECT + 0x200;
            const ITEM: u32 = 0x50000;
            let pops = Rc::new(RefCell::new([false; 2]));
            e.register_double(0x0077_0000, move |e, a| {
                let index = match a[0] {
                    x if x == OBJECT + 0x1064 => 0,
                    x if x == OBJECT + 0xc50 => 1,
                    _ => return result(0),
                };
                if pops.borrow()[index] {
                    return result(0);
                }
                pops.borrow_mut()[index] = true;
                e.mem.set_u32(a[1], ITEM + index as u32);
                result(1)
            });
            e.register_double(0x008c_7aa0, move |_, a| {
                assert!(a.is_empty());
                result(u32::from(scene_mode))
            });
            let getter_order = Rc::new(RefCell::new(Vec::new()));
            let getter_log = getter_order.clone();
            e.register_double(0x0055_9450, move |e, a| {
                let global = a[0] == 0x011d_6e5c;
                getter_log.borrow_mut().push(global);
                result(if global { LISTENER } else { e.mem.u32(a[0]) })
            });
            e.mem.set_u32(LISTENER, 0x31000);
            let mut table = vec![0; 0x3b];
            table[0xe8 / 4] = 0x0077_0300;
            table[0xdc / 4] = 0x0077_0310;
            e.put_vtable(0x31000, &table);
            e.register_double(0x0077_0300, move |_, a| {
                assert_eq!(a, &[LISTENER, ITEM]);
                Ret::default()
            });
            e.register_double(0x0077_0310, move |_, a| {
                assert_eq!(a, &[LISTENER, ITEM + 1, 1]);
                Ret::default()
            });
            e.register_double(0x0043_d410, move |e, a| {
                assert_eq!(a[1..], [0, 0, 0]);
                for offset in [0, 4, 8] {
                    e.mem.set_u32(a[0] + offset, 0);
                }
                result(a[0])
            });
            e.register_double(0x00a5_9c60, move |e, a| {
                assert_eq!(a[0], LISTENER);
                assert_eq!(e.mem.u32(a[1]), 0);
                Ret::default()
            });
            e.register_double(0x0045_37b0, move |_, a| {
                assert!(a.is_empty());
                result(0x7a00)
            });
            e.register_double(0x0087_ac60, move |_, a| {
                assert_eq!(a, &[0x7a00, ITEM]);
                Ret::default()
            });
            e.register_double(0x0087_ac00, move |_, a| {
                assert_eq!(a, &[0x7a00, ITEM + 1, LISTENER, 1, 0]);
                Ret::default()
            });
            e.register_double(0x0087_ad50, move |_, a| {
                assert_eq!(a, &[0x7a00, LISTENER]);
                Ret::default()
            });
            fn_006eae40(&mut e, Ptr::new(OBJECT));
            if scene_mode {
                // 006eb106 resolves the global before the pending item at 006eb111.
                assert_eq!(*getter_order.borrow(), vec![false, true, false, true]);
                assert_eq!(calls(&e, 0x0087_ad50).len(), 1);
            } else {
                assert_eq!(*getter_order.borrow(), vec![true, false, true, false, true]);
                assert_eq!(calls(&e, 0x00a5_9c60).len(), 1);
            }
        }
    }
    #[test]
    fn successful_ray_wrappers_reload_slots_and_pass_output_to_copy() {
        // 006ea622/006ea86a push the output before 00877a30, but it belongs
        // to the later 006cf940 call at 006ea637/006ea87f.
        for navmesh in [false, true] {
            let mut e = fixture();
            const TASK: u32 = 0x25000;
            const PATH: u32 = 0x26000;
            const REQUEST: u32 = 0x27000;
            const SLOTS: u32 = 0x28000;
            const START: u32 = 0x29000;
            const GOAL: u32 = 0x29100;
            const OUTPUT: u32 = 0x29200;
            e.mem.map(TASK, 0x4000);
            e.mem.set_u32(SLOTS, REQUEST);
            e.mem.set_u32(SLOTS + 4, PATH);
            e.mem.set_u32(PATH, if navmesh { 1 } else { 2 });
            e.mem.set_u32(PATH + 4, 0);
            e.mem.set_u32(PATH + 0x84, 1);
            e.mem.map(0x0101_e000, 0x1000);
            e.set_global(0x0101_e2bc, 0.125f32);
            e.register_double(0x0077_0000, move |e, a| {
                assert_eq!(&a[..2], &[OBJECT + 0x1480, 0x1234]);
                e.mem.set_u32(a[2], TASK);
                result(1)
            });
            for address in [0x0040_fbf0, 0x0040_fba0] {
                e.register_double(address, |_, _| Ret::default());
            }
            e.register_double(0x0044_ddc0, move |_, a| {
                assert_eq!(a, &[TASK + 0x50]);
                result(1)
            });
            e.register_double(0x0087_7a30, move |_, a| {
                assert_eq!(a.len(), 2);
                assert_eq!(a[1], 0);
                result(if a[0] == TASK + 0x40 {
                    SLOTS
                } else {
                    assert_eq!(a[0], TASK + 0x50);
                    SLOTS + 4
                })
            });
            let compared = Rc::new(RefCell::new(0));
            let count = compared.clone();
            e.register_double(0x006d_eeb0, move |e, a| {
                assert_eq!(a[2], 0.125f32.to_bits());
                let index = *count.borrow();
                if navmesh {
                    assert_eq!(&a[..2], &[REQUEST + 0x30, GOAL]);
                } else if index == 0 {
                    assert_eq!(&a[..2], &[REQUEST + 8, START]);
                } else {
                    assert_eq!(&a[..2], &[REQUEST + 0xb0, GOAL]);
                }
                *count.borrow_mut() += 1;
                e.mem.set_u32(SLOTS, REQUEST + 0x80);
                e.mem.set_u32(SLOTS + 4, PATH + 0x80);
                result(1)
            });
            e.register_double(0x006c_f940, move |_, a| {
                assert_eq!(a, &[PATH + 0x88, OUTPUT]);
                Ret::default()
            });
            let status = if navmesh {
                fn_006ebbf0(
                    &mut e,
                    Ptr::new(OWNER),
                    0x1234,
                    Ptr::new(START),
                    Ptr::new(GOAL),
                    OUTPUT,
                    0,
                )
            } else {
                fn_006ebc20(
                    &mut e,
                    Ptr::new(OWNER),
                    0x1234,
                    Ptr::new(START),
                    Ptr::new(GOAL),
                    OUTPUT,
                )
            };
            assert_eq!(status, 1);
            assert_eq!(*compared.borrow(), if navmesh { 1 } else { 2 });
            assert_eq!(calls(&e, 0x006c_f940).len(), 1);
            assert_eq!(calls(&e, 0x0040_fba0), vec![vec![TASK + 0x20]]);
        }
    }

    #[test]
    fn static_queue_pop_copies_and_releases_the_selected_ring_slot() {
        let mut e = fixture();
        let queue = OBJECT + 0x500;
        let output = OBJECT + 0x700;
        e.mem.set_u32(queue + 0x408, 1);
        e.mem.set_u32(queue + 0x410, 255);
        e.mem.set_u32(queue + 8 + 255 * 4, 0x1234);
        e.register_double(0x006e_5cc0, move |e, a| {
            assert_eq!(a, &[output, queue + 8 + 255 * 4]);
            e.mem.set_u32(a[0], e.mem.u32(a[1]));
            e.mem.set_u32(queue + 0x410, 4);
            e.mem.set_u32(queue + 0x408, 7);
            result(a[0])
        });
        e.register_double(0x0082_28f0, move |_, a| {
            assert_eq!(a, &[queue + 8 + 255 * 4, 0]);
            Ret::default()
        });
        assert!(fn_006ec5b0(&mut e, Ptr::new(queue), Ptr::new(output)));
        assert_eq!(e.mem.u32(output), 0x1234);
        assert_eq!(e.mem.u32(queue + 0x410), 5);
        assert_eq!(e.mem.u32(queue + 0x408), 6);
        e.mem.set_u32(queue + 0x408, 0);
        assert!(!fn_006ec5b0(&mut e, Ptr::new(queue), Ptr::new(output)));
        assert_eq!(calls(&e, 0x006e_5cc0).len(), 1);

        e.mem.set_u32(queue + 0x408, 1);
        e.mem.set_u32(queue + 0x410, 0x4000_0000);
        e.register_double(0x006e_5cc0, move |e, a| {
            assert_eq!(a, &[output, queue + 8]);
            e.mem.set_u32(a[0], e.mem.u32(a[1]));
            result(a[0])
        });
        e.register_double(0x0082_28f0, move |_, a| {
            assert_eq!(a, &[queue + 8, 0]);
            Ret::default()
        });
        assert!(fn_006ec5b0(&mut e, Ptr::new(queue), Ptr::new(output)));
        assert_eq!(e.mem.u32(queue + 0x410), 1);
        assert_eq!(e.mem.u32(queue + 0x408), 0);
    }

    #[test]
    fn fixed_state_constructor_uses_pc_offsets_and_allocates_each_backing_array() {
        let mut e = fixture();
        let object = OBJECT + 0x800;
        noops(&mut e, &[0x0044_d7f0, 0x0096_a2d0]);
        let guard = Rc::new(RefCell::new(None));
        let initialized_guard = guard.clone();
        let stack_field = object + 0x10;
        e.register_double(0x0040_4eb0, move |_, a| {
            assert_ne!(a[0], stack_field);
            assert_eq!(a[1..], [6, 1, 0x0101_73f8, 0x390]);
            *initialized_guard.borrow_mut() = Some(a[0]);
            Ret::default()
        });
        let destroyed_guard = guard.clone();
        e.register_double(0x0040_4ee0, move |_, a| {
            assert_eq!(a, &[*destroyed_guard.borrow().as_ref().unwrap()]);
            Ret::default()
        });
        e.register_double(0x0040_1000, move |_, a| {
            assert_eq!(a.len(), 1);
            result(match a[0] {
                12 => 0x31000,
                60 => 0x32000,
                16 => 0x33000,
                n => panic!("unexpected allocation size {n}"),
            })
        });
        e.register_double(0x0040_1050, move |_, a| {
            assert_eq!(a, &[0x31000, 4, 3, 0x0044_dee0]);
            Ret::default()
        });
        e.register_double(0x006c_73b0, move |_, a| {
            assert_eq!(a, &[0x33000, 5]);
            result(0x34000)
        });
        fn_006ec660(&mut e, Ptr::new(object), 5, 3, 0x7777);
        assert_eq!(e.mem.u32(object), 0x0106_d72c);
        assert_eq!(e.mem.u32(object + 4), 0x32000);
        assert_eq!(e.mem.u32(object + 8), 3);
        assert_eq!(e.mem.u32(object + 12), 0x31000);
        assert_eq!(e.mem.u32(object + 16), 0x7777);
        assert_eq!(e.mem.u32(object + 20), 0x34000);
    }

    #[test]
    fn fixed_state_allocation_sizes_match_native_overflow_rules() {
        for (requests, paths, expected) in [
            (0x5555_5556, 0x4000_0000, [u32::MAX, 8, 16]),
            (0x2000_0000, 3, [12, u32::MAX, 16]),
        ] {
            let mut e = fixture();
            let object = OBJECT + 0x2800;
            noops(
                &mut e,
                &[0x0044_d7f0, 0x0096_a2d0, 0x0040_4eb0, 0x0040_4ee0],
            );
            let sizes = Rc::new(RefCell::new(Vec::new()));
            let observed = sizes.clone();
            e.register_double(0x0040_1000, move |_, a| {
                observed.borrow_mut().push(a[0]);
                result(0)
            });
            e.register_double(0x0040_1050, move |_, _| {
                panic!("zero allocation must skip array construction")
            });
            fn_006ec660(&mut e, Ptr::new(object), requests, paths, 0x9999);
            assert_eq!(*sizes.borrow(), expected);
            assert_eq!(e.mem.u32(object + 0x10), 0x9999);
        }
    }

    #[test]
    fn list_node_constructor_copies_payload_and_runs_member_constructor() {
        let mut e = fixture();
        let node = OBJECT + 0x900;
        let input = OBJECT + 0xa00;
        e.mem.set_u32(input, 0x12345678);
        e.register_double(0x0044_dee0, move |_, a| {
            assert_eq!(a, &[node + 8]);
            Ret::default()
        });
        fn_006ecbd0(&mut e, Ptr::new(node), 0xABCD, Ptr::new(input));
        assert_eq!(e.mem.u32(node), 0xABCD);
        assert_eq!(e.mem.u32(node + 4), 0x12345678);
    }

    #[test]
    fn base_constructors_set_vtables_and_clear_their_fields() {
        let mut e = fixture();
        let first = OBJECT + 0x1a00;
        let second = OBJECT + 0x1b00;
        let third = OBJECT + 0x1c00;
        for address in [first, second, third] {
            e.mem.set_u32(address, 0xaaaa);
            e.mem.set_u32(address + 4, 0xbbbb);
            e.mem.set_u32(address + 8, 0xcccc);
        }
        fn_006ecc00(&mut e, Ptr::new(first));
        fn_006ecc20(&mut e, Ptr::new(second));
        fn_006ecc40(&mut e, Ptr::new(third));
        assert_eq!(e.mem.u32(first), 0x0106_d6b8);
        assert_eq!(e.mem.u32(second), 0x0106_d710);
        assert_eq!(e.mem.u32(third), 0x0106_d63c);
        assert_eq!(e.mem.u32(third + 4), 0);
        assert_eq!(e.mem.u32(third + 8), 0);
    }

    #[test]
    fn queue_base_constructors_initialize_empty_heads_and_counts() {
        let mut e = fixture();
        let linked = OBJECT + 0x1d00;
        let static_queue = OBJECT + 0x1e00;
        e.mem.set_u32(linked + 4, 0xaaaa);
        e.mem.set_u32(static_queue + 4, 0xbbbb);
        fn_006ecb40(&mut e, Ptr::new(linked));
        fn_006ecb70(&mut e, Ptr::new(static_queue));
        assert_eq!(e.mem.u32(linked), 0x0106_d698);
        assert_eq!(e.mem.u32(linked + 4), 0);
        assert_eq!(e.mem.u32(static_queue), 0x0106_d6f0);
        assert_eq!(e.mem.u32(static_queue + 4), 0);
    }

    #[test]
    fn free_list_item_constructor_stores_value_and_passes_byte_mode() {
        let mut e = fixture();
        let item = OBJECT + 0x1f00;
        e.register_double(0x0044_e010, move |_, a| {
            assert_eq!(a, &[item, 3]);
            Ret::default()
        });
        fn_006ecba0(&mut e, Ptr::new(item), 3, 0x9988);
        assert_eq!(e.mem.u32(item), 0x9988);
    }

    #[test]
    fn task_list_context_and_actor_message_destructor_forward_their_arguments() {
        let mut e = fixture();
        let context = OBJECT + 0x2000;
        let node_slot = OBJECT + 0x2100;
        e.mem.set_u32(node_slot, context);
        e.register_double(0x007b_3fa0, move |_, a| {
            assert_eq!(a, &[context, 0]);
            Ret::default()
        });
        e.register_double(0x0043_b460, move |_, _| result(0));
        e.register_double(0x0040_fbe0, move |_, _| Ret::default());
        fn_006ecb10(&mut e, Ptr::new(OBJECT + 0x2200), Ptr::new(node_slot));
        assert_eq!(e.mem.u32(node_slot), 0);

        let message = OBJECT + 0x2300;
        e.register_double(0x0045_cec0, move |_, a| {
            assert_eq!(a, &[message + 4]);
            Ret::default()
        });
        actor_pathing_message_destruct(&mut e, Ptr::new(message));
    }

    #[test]
    fn free_list_allocator_constructs_node_only_after_allocation_succeeds() {
        for allocated in [false, true] {
            let mut e = fixture();
            let pool = OBJECT + 0x2400;
            let output = OBJECT + 0x2500;
            let node = OBJECT + 0x2600;
            e.register_double(0x006e_cd60, move |e, a| {
                assert_eq!(a, &[pool, output]);
                if allocated {
                    e.mem.set_u32(output, node);
                }
                result(u32::from(allocated))
            });
            e.register_double(0x006e_ce00, move |_, a| {
                assert_eq!(a, &[node, 0x7788]);
                Ret::default()
            });
            assert_eq!(
                fn_006ecd00(&mut e, Ptr::new(pool), Ptr::new(output), 0x7788),
                allocated
            );
            assert_eq!(calls(&e, 0x006e_ce00).len(), usize::from(allocated));
        }
    }

    #[test]
    fn manager_destructor_releases_owned_arrays_and_lock() {
        let mut e = fixture();
        let manager = OBJECT + 0x2700;
        e.mem.set_u32(manager + 4, 0x30100);
        e.mem.set_u32(manager + 12, 0x30200);
        for address in [0x0044_c480, 0x0040_1030, 0x0044_b4b0] {
            e.register_double(address, |_, _| Ret::default());
        }
        fn_006ec870(&mut e, Ptr::new(manager));
        assert_eq!(e.mem.u32(manager), 0x0106_d72c);
        assert_eq!(calls(&e, 0x0040_1030), vec![vec![0x30200], vec![0x30100]]);
        assert_eq!(calls(&e, 0x0044_b4b0), vec![vec![manager]]);

        let mut scalar = fixture();
        scalar.mem.set_u32(manager + 4, 0x30100);
        scalar.mem.set_u32(manager + 12, 0x30200);
        for address in [0x0044_c480, 0x0040_1030, 0x0044_b4b0] {
            scalar.register(address, |_, _| Ret::default());
        }
        path_manager_deleting_destructor(&mut scalar, Ptr::new(manager), 1);
        assert_eq!(
            calls(&scalar, 0x0040_1030),
            vec![vec![0x30200], vec![0x30100], vec![manager]]
        );
    }

    #[test]
    fn free_list_insert_clears_input_slot_and_updates_head_under_interlocked_lock() {
        let mut e = fixture();
        let list = OBJECT + 0xb00;
        let slot = OBJECT + 0xc00;
        let node = OBJECT + 0xd10;
        e.mem.set_u32(list + 8, 0x1111);
        e.mem.set_u32(slot, OBJECT + 0xd00);
        let acquired = Rc::new(RefCell::new(0u32));
        let count = acquired.clone();
        e.register_double(0x0043_b460, move |_, a| {
            if a == [list + 4, 0, 1] {
                *count.borrow_mut() += 1;
                result(if *count.borrow() == 1 { 0x100 } else { 0 })
            } else {
                assert_eq!(a, &[list + 4, 1, 0]);
                Ret::default()
            }
        });
        let barriers = Rc::new(RefCell::new(0u8));
        let barrier_count = barriers.clone();
        e.register_double(0x0040_fbe0, move |e, a| {
            assert!(a.is_empty());
            match *barrier_count.borrow_mut() {
                0 => {
                    e.mem.set_u32(slot, node);
                    e.mem.set_u32(list + 8, 0x2222);
                }
                1 => {
                    assert_eq!(e.mem.u32(node + 8), 0x2222);
                    assert_eq!(e.mem.u32(list + 8), node);
                    assert_eq!(e.mem.u32(slot), 0);
                }
                _ => panic!("unexpected barrier"),
            }
            *barrier_count.borrow_mut() += 1;
            Ret::default()
        });
        fn_006ecc70(&mut e, Ptr::new(list), Ptr::new(slot));
        assert_eq!(e.mem.u32(node + 8), 0x2222);
        assert_eq!(e.mem.u32(list + 8), node);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(calls(&e, 0x0043_b460).len(), 3);
        assert_eq!(calls(&e, 0x0040_fbe0).len(), 2);
    }

    #[test]
    fn task_stack_constructor_zeros_its_pc_fields() {
        let mut e = fixture();
        let stack = OBJECT + 0xe00;
        e.mem.set_u32(stack + 4, 0xaaaa);
        e.mem.set_u32(stack + 8, 0xbbbb);
        e.mem.set_u8(stack + 12, 0xcc);
        assert_eq!(
            task_stack_construct(&mut e, Ptr::new(stack)),
            Ptr::new(stack)
        );
        assert_eq!(e.mem.u32(stack), 0x0106_d644);
        assert_eq!(e.mem.u32(stack + 4), 0);
        assert_eq!(e.mem.u32(stack + 8), 0);
        assert_eq!(e.mem.u8(stack + 12), 0);
    }

    #[test]
    fn request_scan_reloads_count_after_callbacks_and_stops_on_first_match() {
        for (shrink, grow) in [(false, false), (true, false), (false, true)] {
            let mut e = fixture();
            let scanner = OBJECT + 0xf00;
            let mut table = vec![0; 7];
            table[0x18 / 4] = 0x0077_0400;
            e.mem.set_u32(scanner, 0x31000);
            e.mem.set_u32(scanner + 8, if grow { 1 } else { 4 });
            e.put_vtable(0x31000, &table);
            e.register_double(0x0077_0400, move |e, a| {
                assert_eq!(a[0], scanner);
                assert_eq!(a[2], 0x99);
                if a[1] == 0 && shrink {
                    e.mem.set_u32(scanner + 8, 1);
                    result(0)
                } else if a[1] == 0 && grow {
                    e.mem.set_u32(scanner + 8, 2);
                    result(0)
                } else {
                    result(u32::from(a[1] == 1))
                }
            });
            e.register_double(0x0040_fbf0, move |_, a| {
                assert_eq!(a, &[scanner + 0x20, 0]);
                Ret::default()
            });
            e.register_double(0x0040_fba0, move |_, a| {
                assert_eq!(a, &[scanner + 0x20]);
                Ret::default()
            });
            assert_eq!(fn_006ec900(&mut e, Ptr::new(scanner), 0x99), !shrink);
            assert_eq!(calls(&e, 0x0077_0400).len(), if shrink { 1 } else { 2 });
            assert_eq!(calls(&e, 0x0040_fba0).len(), 1);
        }
    }

    #[test]
    fn manager_item_operation_forwards_both_inputs_and_result_context() {
        let mut e = fixture();
        let wrapper = OBJECT + 0x1000;
        let receiver = OBJECT + 0x1100;
        let manager = OBJECT + 0x1200;
        let request = OBJECT + 0x1300;
        let exception = OBJECT + 0x1400;
        e.mem.set_u32(exception + 4, 0xAABBCCDD);
        e.mem.set_u32(wrapper, receiver);
        e.mem.set_u32(receiver, 0x31000);
        let mut table = vec![0; 10];
        table[0x24 / 4] = 0x0077_0410;
        e.put_vtable(0x31000, &table);
        e.register_double(0x0077_0410, move |_, a| {
            assert_eq!(a, &[receiver, manager]);
            result(0x5151)
        });
        e.register_double(0x0066_62a0, move |_, a| {
            assert_eq!(a, &[wrapper, 0x5151, manager]);
            result(1)
        });
        e.register_double(0x0044_df00, move |_, a| {
            assert_eq!(a, &[wrapper + 0x14]);
            result(exception)
        });
        e.register_double(0x0044_df20, move |_, a| {
            assert_eq!(a, &[exception]);
            result(0)
        });
        noops(&mut e, &[0x0040_fbe0, 0x0052_9000]);
        fn_006ec830(&mut e, Ptr::new(wrapper), manager, Ptr::new(request));
        assert_eq!(e.mem.u32(request), 0xAABBCCDD);
    }

    #[test]
    fn locked_request_operation_uses_returned_candidate_and_releases_lock() {
        let mut e = fixture();
        let this = Ptr::new(OBJECT + 0x1500);
        let manager = 0x51000;
        let request = 0x52000;
        let context = 0x53000;
        let mode = 2u8;
        e.register_double(0x0040_fbf0, move |_, a| {
            assert_eq!(a, &[this.addr() + 0x20, 0]);
            Ret::default()
        });
        e.register_double(0x0044_9f80, move |_, a| {
            assert_eq!(a, &[this.addr()]);
            result(0x54000)
        });
        e.register_double(0x0044_d240, move |_, a| {
            assert_eq!(a, &[0x54000, manager, request, context, u32::from(mode)]);
            result(1)
        });
        e.register_double(0x0040_fba0, move |_, a| {
            assert_eq!(a, &[this.addr() + 0x20]);
            Ret::default()
        });
        assert!(fn_006ec7e0(&mut e, this, manager, request, context, mode));
        assert_eq!(calls(&e, 0x0040_fba0).len(), 1);
    }

    #[test]
    fn successful_manager_item_operation_stores_result_and_accepts_clean_state() {
        let this = Ptr::new(OBJECT + 0x1600);
        let output = OBJECT + 0x1700;
        let exception = OBJECT + 0x1800;
        for (initial_success, rejects_state) in [(false, false), (true, false), (true, true)] {
            let mut e = fixture();
            e.mem.set_u32(exception + 4, 0xAABBCCDD);
            e.register_double(0x0066_62a0, move |_, a| {
                assert_eq!(a, &[this.addr(), 0x11, 0x22]);
                result(u32::from(initial_success))
            });
            e.register_double(0x0044_df00, move |_, a| {
                assert_eq!(a, &[this.addr() + 0x14]);
                result(exception)
            });
            e.register_double(0x0044_df20, move |_, a| {
                assert_eq!(a, &[exception]);
                result(u32::from(rejects_state))
            });
            noops(&mut e, &[0x0040_fbe0, 0x0052_9000]);
            assert_eq!(
                fn_006ec9b0(&mut e, this, 0x11, 0x22, Ptr::new(output)),
                initial_success && !rejects_state
            );
            assert_eq!(
                e.mem.u32(output),
                if initial_success { 0xAABBCCDD } else { 0 }
            );
            assert_eq!(
                calls(&e, 0x0044_df00).len(),
                usize::from(initial_success) * 2
            );
            assert_eq!(calls(&e, 0x0044_df20).len(), usize::from(initial_success));
            assert_eq!(calls(&e, 0x0052_9000).len(), 1);
        }
    }

    #[test]
    fn trivial_deleting_destructors_free_only_when_delete_flag_is_set() {
        for function in [
            path_request_array_deleting_destructor as fn(&mut Engine, Ptr, u32) -> Ptr,
            path_result_array_deleting_destructor,
        ] {
            let mut e = fixture();
            e.register_double(0x0084_54f0, move |_, a| {
                assert_eq!(a, &[OBJECT, 1]);
                Ret::default()
            });
            e.register_double(0x0040_1030, move |_, a| {
                assert_eq!(a, &[OBJECT]);
                Ret::default()
            });
            function(&mut e, Ptr::new(OBJECT), 0);
            assert!(calls(&e, 0x0040_1030).is_empty());
            function(&mut e, Ptr::new(OBJECT), 1);
            assert_eq!(calls(&e, 0x0040_1030).len(), 1);
            assert_eq!(calls(&e, 0x0084_54f0).len(), 2);
        }

        let mut linked = fixture();
        linked.register(0x006e_c390, |_, _| result(0));
        noops(&mut linked, &[0x0048_3710, 0x006e_c1c0, 0x0040_1030]);
        linked_message_queue_deleting_destructor(&mut linked, Ptr::new(OBJECT), 1);
        assert_eq!(calls(&linked, 0x0040_1030).len(), 1);

        let mut static_queue = fixture();
        noops(&mut static_queue, &[0x006e_c410, 0x0040_1030]);
        static_message_queue_deleting_destructor(&mut static_queue, Ptr::new(OBJECT), 1);
        assert_eq!(calls(&static_queue, 0x0040_1030).len(), 1);
    }

    #[test]
    fn manager_item_operation_answers_the_report() {
        let mut e = fixture();
        let wrapper = OBJECT + 0x1000;
        let receiver = OBJECT + 0x1100;
        let manager = OBJECT + 0x1200;
        let request = OBJECT + 0x1300;
        let exception = OBJECT + 0x1400;
        e.mem.set_u32(wrapper, receiver);
        e.mem.set_u32(receiver, 0x31000);
        let mut table = vec![0; 10];
        table[0x24 / 4] = 0x0077_0410;
        e.put_vtable(0x31000, &table);
        e.register_double(0x0077_0410, |_, _| result(0x5151));
        e.register_double(0x0044_df00, move |_, _| result(exception));
        e.register_double(0x0044_df20, |_, _| result(0));
        noops(&mut e, &[0x0040_fbe0, 0x0052_9000]);
        e.register_double(0x0066_62a0, |_, _| result(1));
        assert!(fn_006ec830(
            &mut e,
            Ptr::new(wrapper),
            manager,
            Ptr::new(request)
        ));
        e.register_double(0x0066_62a0, |_, _| result(0));
        assert!(!fn_006ec830(
            &mut e,
            Ptr::new(wrapper),
            manager,
            Ptr::new(request)
        ));
    }
}
