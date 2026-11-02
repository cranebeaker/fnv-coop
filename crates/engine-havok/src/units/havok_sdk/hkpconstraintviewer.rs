//! `Havok SDK/hkpconstraintviewer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The viewer is a `hkpWorldViewerBase` that registers itself as a world post-simulation
//! listener (the listener sub-object sits at `+0x28`, the world at `+0x24`, and an array of
//! words at `+0x2c`: data, size, capacity word). `draw` and `postSimulationCallback` bracket
//! their work in two timer records of the thread's profiling stream (`TlsGetValue` of the index
//! at `0126835c`); the `RDTSC` time stamp comes from the host clock.

use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of the profiling stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// The label of the record that ends a timed section, and the labels `"Ttdraw"` and
/// `"TthkpConstraintViewer"`.
const LABEL_END: u32 = 0x010c_41f8;
const LABEL_DRAW: u32 = 0x010d_2160;
const LABEL_POST_SIMULATION: u32 = 0x010d_2168;
/// The memory router (`00c85750`, no arguments): its `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The process id the viewer was registered under, the process factory, and the float that
/// scales every constraint drawer.
const PROCESS_ID: u32 = 0x0126_901c;
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const DRAWER_SCALE: u32 = 0x011b_6168;
/// `hkProcessFactory::registerProcess` (`00dd9380`, thiscall: the name, the creation function),
/// the process name `"Constraints"` at `010cb200` and the creation function `00d699c0`.
const REGISTER_PROCESS: u32 = 0x00dd_9380;
const PROCESS_NAME: u32 = 0x010c_b200;
const PROCESS_CREATE: u32 = 0x00d6_99c0;
/// The monitor lock the drawing runs under: `00c90490` enters (a recursive critical section),
/// `00c904e0` leaves.
const LOCK_ENTER: u32 = 0x00c9_0490;
const LOCK_LEAVE: u32 = 0x00c9_04e0;
/// The constraint drawer constructor `00de4470` (this), `setScale` `00de47d0` (this, the
/// float).
const DRAWER_CONSTRUCT: u32 = 0x00de_4470;
const DRAWER_SET_SCALE: u32 = 0x00de_47d0;
/// The geometry holders of the two bigger drawers: `00de4a80` builds the one in the one-geometry
/// drawer (at `+0xd0`), `00de5450` the three limit geometries (`+0xd0`, `+0x160`, `+0x1f0`) and
/// `00de4ff0` the fourth (`+0x280`) of the ragdoll drawer; `hkDisplayGeometry::~hkDisplayGeometry`
/// (`00da2560`) takes them down again.
const GEOMETRY_CONSTRUCT_ONE: u32 = 0x00de_4a80;
const GEOMETRY_CONSTRUCT_LIMIT: u32 = 0x00de_5450;
const GEOMETRY_CONSTRUCT_LAST: u32 = 0x00de_4ff0;
const GEOMETRY_DESTRUCT: u32 = 0x00da_2560;
/// `hkpConstraintInstance` constructor (`00cf3ea0`: the instance, two entities, the data, the
/// priority) and destructor (`00cf40b0`), and the malleable data's wrapped-data getter
/// (`00cdf6c0`, no arguments).
const INSTANCE_CONSTRUCT: u32 = 0x00cf_3ea0;
const INSTANCE_DESTRUCT: u32 = 0x00cf_40b0;
const WRAPPED_DATA: u32 = 0x00cd_f6c0;
/// `hkpWorldViewerBase` constructor (`00ddccb0`: this, the process contexts) and destructor
/// (`00ddcc60`), `hkpWorld::addWorldPostSimulationListener` (`00c93920`: world, listener) and
/// the matching removal (`00c918e0`).
const WORLD_VIEWER_CONSTRUCT: u32 = 0x00dd_ccb0;
const WORLD_VIEWER_DESTRUCT: u32 = 0x00dd_cc60;
const ADD_LISTENER: u32 = 0x00c9_3920;
const REMOVE_LISTENER: u32 = 0x00c9_18e0;
/// The vtable pointers the viewer's constructor and destructor store: the three live ones at
/// `+0`, `+8` and `+0x20`, the listener's at `+0x28`, and the listener's base-class one.
const VTABLE_VIEWER: u32 = 0x010d_21c4;
const VTABLE_PROCESS: u32 = 0x010d_21a4;
const VTABLE_FILTER: u32 = 0x010d_2194;
const VTABLE_LISTENER: u32 = 0x010d_2184;
const VTABLE_LISTENER_BASE: u32 = 0x010c_cf7c;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in (only the profiling records take it).
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte profiling record `(label, time stamp)` to the thread's stream while its
/// cursor (+4) is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
}

/// How a constraint type is drawn: which drawer object is built for it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Flavour {
    /// The plain drawer (`00de4470`).
    Plain,
    /// The drawer with one geometry holder (`00d69a20`).
    OneGeometry,
    /// The ragdoll drawer with four geometry holders (`00d69a40`).
    Ragdoll,
}

/// The drawer flavour and the address of its `drawConstraint` for a constraint type (the
/// virtual type of the data, vtable slot `0x20`): 0 ball and socket (`hkpBallSocketDrawer`), 1
/// hinge, 2 limited hinge, 3 point to path, 6 prismatic, 7 ragdoll, 8 stiff spring
/// (`hkpStiffSpringDrawer`), 9 wheel, 14 point to plane, 15, 18 and 19 (the Xbox constraint
/// kinds the exe's jump table follows) and the three chain types `100` to `102`
/// (`hkpConstraintChainDrawer`). Types 12 and 13 recurse into the wrapped data; every other
/// type draws nothing.
fn drawing_for(kind: u32) -> Option<(Flavour, u32)> {
    Some(match kind {
        0 => (Flavour::Plain, 0x00de_8090),
        1 => (Flavour::Plain, 0x00de_7f20),
        2 => (Flavour::OneGeometry, 0x00de_7a70),
        3 => (Flavour::Plain, 0x00de_7020),
        6 => (Flavour::Plain, 0x00de_7730),
        7 => (Flavour::Ragdoll, 0x00de_6730),
        8 => (Flavour::Plain, 0x00de_7580),
        9 => (Flavour::Plain, 0x00de_71d0),
        14 => (Flavour::Plain, 0x00de_6d80),
        15 => (Flavour::Plain, 0x00de_6550),
        18 => (Flavour::OneGeometry, 0x00de_5fe0),
        19 => (Flavour::Ragdoll, 0x00de_59f0),
        100..=102 => (Flavour::Plain, 0x00de_6390),
        _ => return None,
    })
}

// Translated from 00d69260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintViewer::draw`: draws `constraint` (an `hkpConstraintInstance`) for `entity`
/// to the display handler `display`, between two timer records and under the monitor lock
/// (`00c90490` / `00c904e0`). The data's virtual type (slot `0x20`) selects the drawer (see
/// [`drawing_for`]): it is built in a local frame (`00de4470`, or the in-file constructors for
/// the two bigger drawers), scaled with `setScale` by the float at `011b6168`, asked to
/// `drawConstraint` (the constraint, `display`, `entity + 0x10`, the process id at
/// `0126901c`) and, for the bigger drawers, taken down again. Types 12 (wrapper: the data at
/// `+0x18` of the data) and 13 (malleable: its wrapped data) build a temporary constraint
/// instance for the same two entities (`[constraint + 0x14]`, `[constraint + 0x18]`, priority 1)
/// and draw that instead (this function again), then destruct it. `this` is only passed on to
/// the recursive call.
pub fn draw(e: &mut Engine, this: Ptr, constraint: Ptr, entity: Ptr, display: u32) {
    profile_record(e, LABEL_DRAW);
    let at = constraint.addr();
    let data = e.mem.u32(at + 0xc);
    let kind = e.vcall(data, 0x20, &args![]).u32();
    e.call(LOCK_ENTER, &args![]);
    e.with_stack(0x600, |e, frame| {
        let base = frame.addr();
        let drawer = base + 0x54;
        let ragdoll = base + 0x1c4;
        let instance = base + 0x1c;
        let outline = entity.addr().wrapping_add(0x10);
        match kind {
            12 => {
                let data = e.mem.u32(at + 0xc);
                let wrapped = e.mem.u32(data + 0x18);
                let first = e.mem.u32(at + 0x14);
                let second = e.mem.u32(at + 0x18);
                e.call(
                    INSTANCE_CONSTRUCT,
                    &args![instance, first, second, wrapped, 1u32],
                );
                draw(e, this, Ptr::new(instance), entity, display);
                e.call(INSTANCE_DESTRUCT, &args![instance]);
            }
            13 => {
                let data = e.mem.u32(at + 0xc);
                let second = e.mem.u32(at + 0x18);
                let first = e.mem.u32(at + 0x14);
                let wrapped = e.call(WRAPPED_DATA, &args![data]).u32();
                e.call(
                    INSTANCE_CONSTRUCT,
                    &args![instance, first, second, wrapped, 1u32],
                );
                draw(e, this, Ptr::new(instance), entity, display);
                e.call(INSTANCE_DESTRUCT, &args![instance]);
            }
            _ => {
                if let Some((flavour, draw_address)) = drawing_for(kind) {
                    let object = match flavour {
                        Flavour::Plain => {
                            e.call(DRAWER_CONSTRUCT, &args![drawer]);
                            drawer
                        }
                        Flavour::OneGeometry => {
                            construct_one_geometry_drawer(e, Ptr::new(drawer));
                            drawer
                        }
                        Flavour::Ragdoll => {
                            construct_ragdoll_drawer(e, Ptr::new(ragdoll));
                            ragdoll
                        }
                    };
                    let scale = x87_quiet_bits(e.mem.u32(DRAWER_SCALE));
                    e.call(DRAWER_SET_SCALE, &args![object, scale]);
                    let process: u32 = e.global(PROCESS_ID);
                    e.call(
                        draw_address,
                        &args![object, constraint, display, outline, process],
                    );
                    match flavour {
                        Flavour::Plain => {}
                        Flavour::OneGeometry => {
                            e.call(GEOMETRY_DESTRUCT, &args![drawer + 0xd0]);
                        }
                        Flavour::Ragdoll => destruct_ragdoll_drawer(e, Ptr::new(ragdoll)),
                    }
                }
            }
        }
    });
    e.call(LOCK_LEAVE, &args![]);
    profile_record(e, LABEL_END);
}

// Translated from 00d69730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintViewer::postSimulationCallback`: `this` is the listener sub-object (the viewer
/// is at `this - 0x28`, its display handler word at `this - 0x10`). Draws every constraint of
/// every entity of every island of the world's two island lists (`+0x28` / `+0x2c` and `+0x34` /
/// `+0x38`: array of island pointers, signed count). An island's entities are the pointer
/// array at `+0x48` (count at `+0x4c`); an entity's constraints are the `0x30`-byte records at
/// `[entity + 0xac]` (count `u16` at `+0xb0`), the first word of each the constraint.
pub fn post_simulation_callback(e: &mut Engine, this: Ptr, world: Ptr) {
    profile_record(e, LABEL_POST_SIMULATION);
    let listener = this.addr();
    let viewer = listener.wrapping_sub(0x28);
    let world = world.addr();
    for (list_field, count_field) in [(0x28, 0x2c), (0x34, 0x38)] {
        if (e.mem.u32(world + count_field) as i32) > 0 {
            let mut island_index = 0u32;
            loop {
                let islands = e.mem.u32(world + list_field);
                let mut island = e
                    .mem
                    .u32(islands.wrapping_add(island_index.wrapping_mul(4)));
                let mut entity_index = 0u32;
                if (e.mem.u32(island + 0x4c) as i32) > 0 {
                    loop {
                        let entities = e.mem.u32(island + 0x48);
                        let entity = e
                            .mem
                            .u32(entities.wrapping_add(entity_index.wrapping_mul(4)));
                        if e.mem.u16(entity + 0xb0) != 0 {
                            let mut record = 0u32;
                            loop {
                                let display = e.mem.u32(listener.wrapping_sub(0x10));
                                let records = e.mem.u32(entity + 0xac);
                                let constraint =
                                    e.mem.u32(records.wrapping_add(record.wrapping_mul(0x30)));
                                draw(
                                    e,
                                    Ptr::new(viewer),
                                    Ptr::new(constraint),
                                    Ptr::new(entity),
                                    display,
                                );
                                record += 1;
                                if (record as i32) >= i32::from(e.mem.u16(entity + 0xb0)) {
                                    break;
                                }
                            }
                        }
                        let islands = e.mem.u32(world + list_field);
                        island = e
                            .mem
                            .u32(islands.wrapping_add(island_index.wrapping_mul(4)));
                        entity_index += 1;
                        if (entity_index as i32) >= (e.mem.u32(island + 0x4c) as i32) {
                            break;
                        }
                    }
                }
                island_index += 1;
                if (island_index as i32) >= (e.mem.u32(world + count_field) as i32) {
                    break;
                }
            }
        }
    }
    profile_record(e, LABEL_END);
}

// Translated from 00d698b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The viewer's constructor (`hkpConstraintViewer`; the name is not confirmed): runs the
/// `hkpWorldViewerBase` constructor (`00ddccb0`, with the process `contexts`, which finds the
/// world and stores it at `+0x24`), stores the viewer's vtable pointers (`+0`, `+8`, `+0x20`,
/// the listener's at `+0x28`), empties the array at `+0x2c` (capacity word `0x80000000`), and
/// registers the listener (`this + 0x28`) with each of the `[world + 0x70]` entries of the
/// array at `[world + 0x6c]` (`00c93920`). Answers `this`.
pub fn construct_viewer(e: &mut Engine, this: Ptr, contexts: Ptr) -> Ptr {
    let at = this.addr();
    e.call(WORLD_VIEWER_CONSTRUCT, &args![this, contexts]);
    let listener = at + 0x28;
    e.mem.set_u32(at + 0x28, VTABLE_LISTENER_BASE);
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at + 8, VTABLE_PROCESS);
    e.mem.set_u32(at + 0x20, VTABLE_FILTER);
    e.mem.set_u32(listener, VTABLE_LISTENER);
    e.mem.set_u32(at + 0x2c, 0);
    e.mem.set_u32(at + 0x30, 0);
    e.mem.set_u32(at + 0x34, 0x8000_0000);
    let world = e.mem.u32(at + 0x24);
    if world != 0 && (e.mem.u32(world + 0x70) as i32) > 0 {
        let mut index = 0u32;
        loop {
            let world = e.mem.u32(at + 0x24);
            let worlds = e.mem.u32(world + 0x6c);
            let target = e.mem.u32(worlds.wrapping_add(index.wrapping_mul(4)));
            e.call(ADD_LISTENER, &args![target, listener]);
            let world = e.mem.u32(at + 0x24);
            index += 1;
            if (index as i32) >= (e.mem.u32(world + 0x70) as i32) {
                break;
            }
        }
    }
    this
}

// Translated from 00d69920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The viewer's destructor (name not confirmed; `ECX` = `this`): puts the vtable pointers
/// back, removes the listener (`00c918e0`) from each of the `[world + 0x70]` entries of the
/// array at `[world + 0x6c]`, empties the array at `+0x2c` (freeing its data through the
/// allocator, slot `0x10`, when the capacity word is not negative: `(capacity & 0x3fffffff) * 4`
/// bytes), stores the listener's base vtable pointer and ends in the `hkpWorldViewerBase`
/// destructor (`00ddcc60`), whose result it answers.
pub fn destruct_viewer(e: &mut Engine, this: Ptr) -> u32 {
    let at = this.addr();
    let listener = at + 0x28;
    let world = e.mem.u32(at + 0x24);
    e.mem.set_u32(at, VTABLE_VIEWER);
    e.mem.set_u32(at + 8, VTABLE_PROCESS);
    e.mem.set_u32(at + 0x20, VTABLE_FILTER);
    e.mem.set_u32(listener, VTABLE_LISTENER);
    if world != 0 && (e.mem.u32(world + 0x70) as i32) > 0 {
        let mut index = 0u32;
        loop {
            let world = e.mem.u32(at + 0x24);
            let worlds = e.mem.u32(world + 0x6c);
            let target = e.mem.u32(worlds.wrapping_add(index.wrapping_mul(4)));
            e.call(REMOVE_LISTENER, &args![target, listener]);
            let world = e.mem.u32(at + 0x24);
            index += 1;
            if (index as i32) >= (e.mem.u32(world + 0x70) as i32) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(at + 0x34);
    e.mem.set_u32(at + 0x30, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(at + 0x2c);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(at + 0x2c, 0);
    e.mem.set_u32(at + 0x34, 0x8000_0000);
    e.mem.set_u32(listener, VTABLE_LISTENER_BASE);
    e.call(WORLD_VIEWER_DESTRUCT, &args![this]).u32()
}

// Translated from 00d69a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintViewer::registerViewer`: registers the viewer's creation function (`00d699c0`)
/// as the process `"Constraints"` with the process factory at `01269cd0` and keeps the answered
/// id at `0126901c`.
pub fn register_viewer(e: &mut Engine) {
    let factory: u32 = e.global(PROCESS_FACTORY);
    let id = e
        .call(
            REGISTER_PROCESS,
            &args![factory, PROCESS_NAME, PROCESS_CREATE],
        )
        .u32();
    e.set_global(PROCESS_ID, id);
}

// Translated from 00d69a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the constraint drawer with one geometry holder (`ECX` = `this`; the
/// class name is not known): the plain drawer constructor on `this`, the geometry holder's
/// constructor (`00de4a80`) on `this + 0xd0`. Answers `this`.
pub fn construct_one_geometry_drawer(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(DRAWER_CONSTRUCT, &args![this]);
    e.call(GEOMETRY_CONSTRUCT_ONE, &args![this.addr() + 0xd0]);
    this
}

// Translated from 00d69a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpRagdollDrawer` constructor (`ECX` = `this`): the plain drawer constructor on `this`,
/// the limit geometry constructor (`00de5450`) on `this + 0xd0`, `+0x160` and `+0x1f0`, and the
/// last holder's (`00de4ff0`) on `this + 0x280`. Answers `this`.
pub fn construct_ragdoll_drawer(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.call(DRAWER_CONSTRUCT, &args![this]);
    for offset in [0xd0, 0x160, 0x1f0] {
        e.call(GEOMETRY_CONSTRUCT_LIMIT, &args![at + offset]);
    }
    e.call(GEOMETRY_CONSTRUCT_LAST, &args![at + 0x280]);
    this
}

// Translated from 00d69a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollDrawer::~hkpRagdollDrawer` (`ECX` = `this`): `hkDisplayGeometry::~hkDisplayGeometry`
/// (`00da2560`) on `this + 0x280`, `+0x1f0`, `+0x160` and `+0xd0`.
pub fn destruct_ragdoll_drawer(e: &mut Engine, this: Ptr) {
    for offset in [0x280, 0x1f0, 0x160, 0xd0] {
        e.call(GEOMETRY_DESTRUCT, &args![this.addr() + offset]);
    }
}

// Translated from 00d69ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id of the registered viewer (`0126901c`).
pub fn viewer_process_id(e: &mut Engine) -> u32 {
    e.global(PROCESS_ID)
}

// Translated from 00d69af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_9920`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d69af0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_9920, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d69ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x20` (`SUB ECX,0x20`) and jumps to `00d69af0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d69ac0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_9af0, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d69ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00d69af0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d69ad0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_9af0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d69ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x28` (`SUB ECX,0x28`) and jumps to `00d69af0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00d69ae0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_9af0, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

// Translated from 00d699c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the viewer or process class of this unit is unconfirmed): a
/// process creation function, the kind `hkProcessFactory::registerProcess` (`00dd9380`) is given.
/// It takes one stack word, the process contexts (cdecl, nothing popped). Allocates 0x38 bytes through the memory router's allocator (`00c85750`; the
/// virtual function at +4 of the object at router + 0x10), stores the size as a 16-bit word at +4,
/// runs the constructor `00d698b0` on the block with the contexts, and answers that object + 8 when it is not null, else null.
/// Ghidra made no function here; the address is pushed at 00d69a06.
pub fn fn_00d699c0(e: &mut Engine, process_contexts: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x38]).u32();
    e.mem.set_u16(block + 4, 0x38);
    let object = e.call(0x00d6_98b0, &args![block, process_contexts]).u32();
    if object != 0 {
        object.wrapping_add(8)
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_9260, draw(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d6_9730, post_simulation_callback(Ptr, Ptr)),
        entry!(0x00d6_98b0, construct_viewer(Ptr, Ptr) -> Ptr),
        entry!(0x00d6_9920, destruct_viewer(Ptr) -> u32),
        entry!(0x00d6_9a00, register_viewer()),
        entry!(0x00d6_9a20, construct_one_geometry_drawer(Ptr) -> Ptr),
        entry!(0x00d6_9a40, construct_ragdoll_drawer(Ptr) -> Ptr),
        entry!(0x00d6_9a80, destruct_ragdoll_drawer(Ptr)),
        entry!(0x00d6_9ab0, viewer_process_id() -> u32),
        entry!(0x00d6_9af0, deleting_destructor_00d69af0(Ptr, u8) -> Ptr),
        entry!(0x00d6_9ac0, fn_00d69ac0(Ptr, u32) -> u32),
        entry!(0x00d6_9ad0, fn_00d69ad0(Ptr, u32) -> u32),
        entry!(0x00d6_9ae0, fn_00d69ae0(Ptr, u32) -> u32),
        entry!(0x00d6_99c0, fn_00d699c0(u32) -> u32),
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
    fn deleting_destructor_00d69af0_frees_only_when_bit_zero_is_set() {
        check(0x00d6_9af0, 0x00d6_9920);
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
    fn thunk_00d69ac0() {
        assert_eq!(check(0x00d6_9ac0, 0x00d6_9af0, 0x20), 0xdead_beef);
    }

    #[test]
    fn thunk_00d69ad0() {
        assert_eq!(check(0x00d6_9ad0, 0x00d6_9af0, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00d69ae0() {
        assert_eq!(check(0x00d6_9ae0, 0x00d6_9af0, 0x28), 0xdead_beef);
    }
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine that logs every call, with the exe's data range mapped, a profiling stream
    /// with room for 16 records (answered too), the drawer scale 0.75 and the process id
    /// `0x1234`.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        e.set_global(PROFILE_SLOT, 5u32);
        let stream = e.mem.alloc(0x10);
        let records = e.mem.alloc(12 * 16);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 12 * 16);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        e.mem.set_f32(DRAWER_SCALE, 0.75);
        e.set_global(PROCESS_ID, 0x1234u32);
        (e, stream)
    }

    /// The argument lists of the logged calls, in order, restricted to `addresses`.
    fn calls_in(e: &Engine, addresses: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(callee, _)| addresses.contains(callee))
            .cloned()
            .collect()
    }

    /// Registers doubles that answer nothing for all the callees of `draw` and returns the
    /// list of their addresses.
    fn draw_callees(e: &mut Engine) -> Vec<u32> {
        let mut addresses = vec![
            LOCK_ENTER,
            LOCK_LEAVE,
            DRAWER_CONSTRUCT,
            DRAWER_SET_SCALE,
            GEOMETRY_CONSTRUCT_ONE,
            GEOMETRY_CONSTRUCT_LIMIT,
            GEOMETRY_CONSTRUCT_LAST,
            GEOMETRY_DESTRUCT,
            INSTANCE_DESTRUCT,
        ];
        for kind in 0..0x70 {
            if let Some((_, address)) = drawing_for(kind) {
                if !addresses.contains(&address) {
                    addresses.push(address);
                }
            }
        }
        for address in &addresses {
            e.register(*address, |_, _| Ret::default());
        }
        addresses
    }

    /// A constraint instance whose data object answers the virtual type `kind` (the double at
    /// `0x0900_1000 + kind`).
    fn constraint_of(e: &mut Engine, kind: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x20, 0x0900_1000 + kind);
        e.register_double(0x0900_1000 + kind, move |_, _| Ret {
            eax: kind,
            ..Ret::default()
        });
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data, vtable);
        let constraint = e.mem.alloc(0x40);
        e.mem.set_u32(constraint + 0xc, data);
        constraint
    }

    #[test]
    fn draw_builds_the_drawer_of_each_constraint_type_around_its_draw_constraint() {
        for kind in (0..0x70u32).filter(|kind| *kind != 12 && *kind != 13) {
            let (mut e, stream) = engine();
            let interesting = draw_callees(&mut e);
            let constraint = constraint_of(&mut e, kind);
            let entity = e.mem.alloc(0x100);
            let first = e.mem.u32(stream + 4);
            e.call(
                0x00d6_9260,
                &args![0x7777u32, constraint, entity, 0x9999u32],
            );
            let calls = calls_in(&e, &interesting);
            assert_eq!(calls.first().unwrap(), &(LOCK_ENTER, vec![]));
            assert_eq!(calls.last().unwrap(), &(LOCK_LEAVE, vec![]));
            let scale = 0.75f32.to_bits();
            let args_of = |index: usize| calls[index].1.clone();
            match drawing_for(kind) {
                None => assert_eq!(calls.len(), 2, "type {kind}"),
                Some((Flavour::Plain, address)) => {
                    let drawer = args_of(1)[0];
                    assert_eq!(
                        calls[1..calls.len() - 1],
                        [
                            (DRAWER_CONSTRUCT, vec![drawer]),
                            (DRAWER_SET_SCALE, vec![drawer, scale]),
                            (
                                address,
                                vec![drawer, constraint, 0x9999, entity + 0x10, 0x1234]
                            ),
                        ],
                        "type {kind}"
                    );
                }
                Some((Flavour::OneGeometry, address)) => {
                    let drawer = args_of(1)[0];
                    assert_eq!(
                        calls[1..calls.len() - 1],
                        [
                            (DRAWER_CONSTRUCT, vec![drawer]),
                            (GEOMETRY_CONSTRUCT_ONE, vec![drawer + 0xd0]),
                            (DRAWER_SET_SCALE, vec![drawer, scale]),
                            (
                                address,
                                vec![drawer, constraint, 0x9999, entity + 0x10, 0x1234]
                            ),
                            (GEOMETRY_DESTRUCT, vec![drawer + 0xd0]),
                        ],
                        "type {kind}"
                    );
                }
                Some((Flavour::Ragdoll, address)) => {
                    let drawer = args_of(1)[0];
                    assert_eq!(
                        calls[1..calls.len() - 1],
                        [
                            (DRAWER_CONSTRUCT, vec![drawer]),
                            (GEOMETRY_CONSTRUCT_LIMIT, vec![drawer + 0xd0]),
                            (GEOMETRY_CONSTRUCT_LIMIT, vec![drawer + 0x160]),
                            (GEOMETRY_CONSTRUCT_LIMIT, vec![drawer + 0x1f0]),
                            (GEOMETRY_CONSTRUCT_LAST, vec![drawer + 0x280]),
                            (DRAWER_SET_SCALE, vec![drawer, scale]),
                            (
                                address,
                                vec![drawer, constraint, 0x9999, entity + 0x10, 0x1234]
                            ),
                            (GEOMETRY_DESTRUCT, vec![drawer + 0x280]),
                            (GEOMETRY_DESTRUCT, vec![drawer + 0x1f0]),
                            (GEOMETRY_DESTRUCT, vec![drawer + 0x160]),
                            (GEOMETRY_DESTRUCT, vec![drawer + 0xd0]),
                        ],
                        "type {kind}"
                    );
                }
            }
            // The timer records: start and end.
            let cursor = e.mem.u32(stream + 4);
            assert_eq!(cursor, first + 24);
            assert_eq!(e.mem.u32(first), LABEL_DRAW);
            assert_eq!(e.mem.u32(first + 12), LABEL_END);
        }
    }

    #[test]
    fn draw_quiets_a_signalling_scale() {
        let (mut e, _) = engine();
        let interesting = draw_callees(&mut e);
        let constraint = constraint_of(&mut e, 0);
        let entity = e.mem.alloc(0x100);
        e.mem.set_u32(DRAWER_SCALE, 0x7f80_0001);
        e.call(0x00d6_9260, &args![0u32, constraint, entity, 0u32]);
        let scale = calls_in(&e, &interesting)
            .into_iter()
            .find(|(address, _)| *address == DRAWER_SET_SCALE)
            .unwrap();
        assert_eq!(scale.1[1], 0x7fc0_0001);
    }

    /// The wrapper types build a temporary instance for the same entities around the wrapped
    /// data, draw it, and destruct it.
    #[test]
    fn draw_of_a_wrapper_or_malleable_constraint_draws_a_temporary_instance() {
        for kind in [12u32, 13] {
            let (mut e, stream) = engine();
            let interesting = draw_callees(&mut e);
            let constraint = constraint_of(&mut e, kind);
            let entity = e.mem.alloc(0x100);
            let inner = constraint_of(&mut e, 0);
            let inner_data = e.mem.u32(inner + 0xc);
            let inner_vtable = e.mem.u32(inner_data);
            let outer_data = e.mem.u32(constraint + 0xc);
            // The wrapper keeps its data at +0x18; the malleable data's getter answers an
            // object of its own. Both are of type 0.
            e.mem.set_u32(outer_data + 0x18, inner_data);
            let wrapped = e.mem.alloc(0x40);
            e.mem.set_u32(wrapped, inner_vtable);
            e.register_double(WRAPPED_DATA, move |_, _| Ret {
                eax: wrapped,
                ..Ret::default()
            });
            // The temporary instance is the first argument of the constructor, which stores
            // the data it was given.
            e.register_double(INSTANCE_CONSTRUCT, |e, a| {
                e.mem.set_u32(a[0] + 0xc, a[3]);
                Ret::default()
            });
            e.mem.set_u32(constraint + 0x14, 0xa1);
            e.mem.set_u32(constraint + 0x18, 0xb2);
            let first = e.mem.u32(stream + 4);
            e.call(0x00d6_9260, &args![0u32, constraint, entity, 0x55u32]);
            let mut all = interesting;
            all.extend([INSTANCE_CONSTRUCT, WRAPPED_DATA]);
            let calls = calls_in(&e, &all);
            let construct = calls
                .iter()
                .find(|(address, _)| *address == INSTANCE_CONSTRUCT)
                .unwrap();
            let instance = construct.1[0];
            let data = if kind == 12 { inner_data } else { wrapped };
            assert_eq!(construct.1[1..], [0xa1, 0xb2, data, 1]);
            assert_eq!(
                calls
                    .iter()
                    .any(|call| *call == (WRAPPED_DATA, vec![outer_data])),
                kind == 13
            );
            // The nested draw is of type 0 for the temporary instance, between the locks.
            let draw_call = calls
                .iter()
                .find(|(address, _)| *address == 0x00de_8090)
                .unwrap();
            assert_eq!(draw_call.1[1], instance);
            assert_eq!(draw_call.1[2..], [0x55, entity + 0x10, 0x1234]);
            assert_eq!(calls.last().unwrap(), &(LOCK_LEAVE, vec![]));
            assert_eq!(
                calls
                    .iter()
                    .filter(|(address, _)| *address == LOCK_ENTER)
                    .count(),
                2
            );
            assert!(calls.contains(&(INSTANCE_DESTRUCT, vec![instance])));
            // Four timer records: a pair for each of the two draws, nested.
            let labels: Vec<u32> = (0..4).map(|index| e.mem.u32(first + 12 * index)).collect();
            assert_eq!(labels, [LABEL_DRAW, LABEL_DRAW, LABEL_END, LABEL_END]);
            assert_eq!(e.mem.u32(stream + 4), first + 48);
        }
    }

    #[test]
    fn post_simulation_callback_draws_every_constraint_of_every_entity_of_both_lists() {
        let (mut e, stream) = engine();
        let _ = draw_callees(&mut e);
        let constraints: Vec<u32> = (0..4).map(|_| constraint_of(&mut e, 0)).collect();
        // An entity with the given constraints: 0x30-byte records.
        let entity_with = |e: &mut Engine, own: &[u32]| {
            let entity = e.mem.alloc(0x100);
            let records = e.mem.alloc(0x30 * own.len().max(1) as u32);
            for (index, constraint) in own.iter().enumerate() {
                e.mem.set_u32(records + 0x30 * index as u32, *constraint);
            }
            e.mem.set_u32(entity + 0xac, records);
            e.mem.set_u16(entity + 0xb0, own.len() as u16);
            entity
        };
        let entities = [
            entity_with(&mut e, &[constraints[0], constraints[1]]),
            entity_with(&mut e, &[]),
            entity_with(&mut e, &[constraints[2]]),
            entity_with(&mut e, &[constraints[3]]),
        ];
        // The islands: list one has one island with entities 0 and 1 and one with entity 2,
        // list two one island with entity 3.
        let island_of = |e: &mut Engine, members: &[u32]| {
            let island = e.mem.alloc(0x80);
            let array = e.mem.alloc(4 * members.len() as u32);
            for (index, member) in members.iter().enumerate() {
                e.mem.set_u32(array + 4 * index as u32, *member);
            }
            e.mem.set_u32(island + 0x48, array);
            e.mem.set_u32(island + 0x4c, members.len() as u32);
            island
        };
        let islands_one = [
            island_of(&mut e, &entities[..2]),
            island_of(&mut e, &entities[2..3]),
        ];
        let island_two = island_of(&mut e, &entities[3..]);
        let world = e.mem.alloc(0x80);
        let list_one = e.mem.alloc(8);
        e.mem.set_u32(list_one, islands_one[0]);
        e.mem.set_u32(list_one + 4, islands_one[1]);
        let list_two = e.mem.alloc(4);
        e.mem.set_u32(list_two, island_two);
        e.mem.set_u32(world + 0x28, list_one);
        e.mem.set_u32(world + 0x2c, 2);
        e.mem.set_u32(world + 0x34, list_two);
        e.mem.set_u32(world + 0x38, 1);
        let viewer = e.mem.alloc(0x80);
        e.mem.set_u32(viewer + 0x18, 0xd15b);
        let first = e.mem.u32(stream + 4);
        e.call(0x00d6_9730, &args![viewer + 0x28, world]);
        let draws: Vec<(u32, u32, u32)> = calls_in(&e, &[0x00de_8090])
            .into_iter()
            .map(|(_, a)| (a[1], a[2], a[3]))
            .collect();
        assert_eq!(
            draws,
            [
                (constraints[0], 0xd15b, entities[0] + 0x10),
                (constraints[1], 0xd15b, entities[0] + 0x10),
                (constraints[2], 0xd15b, entities[2] + 0x10),
                (constraints[3], 0xd15b, entities[3] + 0x10),
            ]
        );
        // The timer records: the callback's pair around the four draws' pairs.
        let labels: Vec<u32> = (0..10).map(|index| e.mem.u32(first + 12 * index)).collect();
        assert_eq!(labels[0], LABEL_POST_SIMULATION);
        assert_eq!(labels[1], LABEL_DRAW);
        assert_eq!(labels[9], LABEL_END);
    }

    #[test]
    fn post_simulation_callback_with_empty_lists_only_writes_the_timer_records() {
        let (mut e, stream) = engine();
        let world = e.mem.alloc(0x80);
        let viewer = e.mem.alloc(0x80);
        let first = e.mem.u32(stream + 4);
        e.call(0x00d6_9730, &args![viewer + 0x28, world]);
        assert_eq!(e.mem.u32(stream + 4), first + 24);
        assert_eq!(e.mem.u32(first), LABEL_POST_SIMULATION);
        assert_eq!(e.mem.u32(first + 12), LABEL_END);
    }

    /// A world with `listeners` entries in its array at `+0x6c`; the base constructor double
    /// stores it into the viewer at `+0x24`.
    fn viewer_rig(e: &mut Engine, listeners: u32) -> (u32, u32, Vec<u32>) {
        let world = e.mem.alloc(0x80);
        let array = e.mem.alloc(4 * listeners.max(1));
        let targets: Vec<u32> = (0..listeners).map(|_| e.mem.alloc(0x10)).collect();
        for (index, target) in targets.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *target);
        }
        e.mem.set_u32(world + 0x6c, array);
        e.mem.set_u32(world + 0x70, listeners);
        let viewer = e.mem.alloc(0x80);
        (viewer, world, targets)
    }

    #[test]
    fn construct_viewer_registers_its_listener_with_every_world_entry() {
        let (mut e, _) = engine();
        let (viewer, world, targets) = viewer_rig(&mut e, 2);
        e.register_double(WORLD_VIEWER_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0] + 0x24, world);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(ADD_LISTENER, |_, _| Ret::default());
        let contexts = e.mem.alloc(0x10);
        let r = e.call(0x00d6_98b0, &args![viewer, contexts]);
        assert_eq!(r.u32(), viewer);
        assert_eq!(
            calls_in(&e, &[WORLD_VIEWER_CONSTRUCT, ADD_LISTENER]),
            [
                (WORLD_VIEWER_CONSTRUCT, vec![viewer, contexts]),
                (ADD_LISTENER, vec![targets[0], viewer + 0x28]),
                (ADD_LISTENER, vec![targets[1], viewer + 0x28]),
            ]
        );
        assert_eq!(e.mem.u32(viewer), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(viewer + 8), VTABLE_PROCESS);
        assert_eq!(e.mem.u32(viewer + 0x20), VTABLE_FILTER);
        assert_eq!(e.mem.u32(viewer + 0x28), VTABLE_LISTENER);
        assert_eq!(
            [
                e.mem.u32(viewer + 0x2c),
                e.mem.u32(viewer + 0x30),
                e.mem.u32(viewer + 0x34)
            ],
            [0, 0, 0x8000_0000]
        );
    }

    #[test]
    fn construct_viewer_without_a_world_or_listeners_registers_nothing() {
        let (mut e, _) = engine();
        e.register(WORLD_VIEWER_CONSTRUCT, |_, _| Ret::default());
        e.register(ADD_LISTENER, |_, _| Ret::default());
        let viewer = e.mem.alloc(0x80);
        e.call(0x00d6_98b0, &args![viewer, 0u32]);
        assert!(calls_in(&e, &[ADD_LISTENER]).is_empty());
        // A world with no entries.
        let (viewer, world, _) = viewer_rig(&mut e, 0);
        e.register_double(WORLD_VIEWER_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0] + 0x24, world);
            Ret::default()
        });
        e.call(0x00d6_98b0, &args![viewer, 0u32]);
        assert!(calls_in(&e, &[ADD_LISTENER]).is_empty());
    }

    /// The memory router with an allocator whose slot `0x10` logs `(data, bytes)`.
    fn router(e: &mut Engine) -> (u32, Rc<RefCell<Vec<(u32, u32)>>>) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        let freed = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&freed);
        e.register_double(0x0900_0010, move |_, a| {
            assert_eq!(a[0], allocator);
            sink.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        (allocator, freed)
    }

    #[test]
    fn destruct_viewer_unregisters_frees_the_array_and_runs_the_base_destructor() {
        let (mut e, _) = engine();
        let (viewer, world, targets) = viewer_rig(&mut e, 2);
        let (_, freed) = router(&mut e);
        e.register(REMOVE_LISTENER, |_, _| Ret::default());
        e.register(WORLD_VIEWER_DESTRUCT, |_, _| Ret {
            eax: 0xbeef,
            ..Ret::default()
        });
        e.mem.set_u32(viewer + 0x24, world);
        e.mem.set_u32(viewer + 0x2c, 0xda7a);
        e.mem.set_u32(viewer + 0x30, 3);
        e.mem.set_u32(viewer + 0x34, 0x0000_0010);
        let r = e.call(0x00d6_9920, &args![viewer]);
        assert_eq!(r.u32(), 0xbeef);
        assert_eq!(
            calls_in(&e, &[REMOVE_LISTENER, WORLD_VIEWER_DESTRUCT]),
            [
                (REMOVE_LISTENER, vec![targets[0], viewer + 0x28]),
                (REMOVE_LISTENER, vec![targets[1], viewer + 0x28]),
                (WORLD_VIEWER_DESTRUCT, vec![viewer]),
            ]
        );
        assert_eq!(*freed.borrow(), [(0xda7a, 0x40)]);
        assert_eq!(e.mem.u32(viewer), VTABLE_VIEWER);
        assert_eq!(e.mem.u32(viewer + 8), VTABLE_PROCESS);
        assert_eq!(e.mem.u32(viewer + 0x20), VTABLE_FILTER);
        assert_eq!(e.mem.u32(viewer + 0x28), VTABLE_LISTENER_BASE);
        assert_eq!(
            [
                e.mem.u32(viewer + 0x2c),
                e.mem.u32(viewer + 0x30),
                e.mem.u32(viewer + 0x34)
            ],
            [0, 0, 0x8000_0000]
        );
    }

    #[test]
    fn destruct_viewer_keeps_an_array_with_a_negative_capacity_and_a_missing_world() {
        let (mut e, _) = engine();
        let (_, freed) = router(&mut e);
        e.register(REMOVE_LISTENER, |_, _| Ret::default());
        e.register(WORLD_VIEWER_DESTRUCT, |_, _| Ret::default());
        let viewer = e.mem.alloc(0x80);
        e.mem.set_u32(viewer + 0x2c, 0xda7a);
        e.mem.set_u32(viewer + 0x34, 0x8000_0004);
        e.call(0x00d6_9920, &args![viewer]);
        assert!(freed.borrow().is_empty());
        assert!(calls_in(&e, &[REMOVE_LISTENER]).is_empty());
        assert_eq!(e.mem.u32(viewer + 0x2c), 0);
    }

    #[test]
    fn register_viewer_keeps_the_process_id_the_factory_answers() {
        let (mut e, _) = engine();
        e.set_global(PROCESS_FACTORY, 0xfac7u32);
        e.register(REGISTER_PROCESS, |_, _| Ret {
            eax: 0x4242,
            ..Ret::default()
        });
        e.call(0x00d6_9a00, &args![]);
        assert_eq!(
            calls_in(&e, &[REGISTER_PROCESS]),
            [(REGISTER_PROCESS, vec![0xfac7, 0x010c_b200, 0x00d6_99c0])]
        );
        assert_eq!(e.global::<u32>(PROCESS_ID), 0x4242);
        assert_eq!(e.call(0x00d6_9ab0, &args![]).u32(), 0x4242);
    }

    #[test]
    fn the_two_bigger_drawers_build_and_destroy_their_geometry_holders() {
        let (mut e, _) = engine();
        for address in [
            DRAWER_CONSTRUCT,
            GEOMETRY_CONSTRUCT_ONE,
            GEOMETRY_CONSTRUCT_LIMIT,
            GEOMETRY_CONSTRUCT_LAST,
            GEOMETRY_DESTRUCT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let drawer = e.mem.alloc(0x300);
        let r = e.call(0x00d6_9a20, &args![drawer]);
        assert_eq!(r.u32(), drawer);
        let r = e.call(0x00d6_9a40, &args![drawer]);
        assert_eq!(r.u32(), drawer);
        e.call(0x00d6_9a80, &args![drawer]);
        let all = [
            DRAWER_CONSTRUCT,
            GEOMETRY_CONSTRUCT_ONE,
            GEOMETRY_CONSTRUCT_LIMIT,
            GEOMETRY_CONSTRUCT_LAST,
            GEOMETRY_DESTRUCT,
        ];
        assert_eq!(
            calls_in(&e, &all),
            [
                (DRAWER_CONSTRUCT, vec![drawer]),
                (GEOMETRY_CONSTRUCT_ONE, vec![drawer + 0xd0]),
                (DRAWER_CONSTRUCT, vec![drawer]),
                (GEOMETRY_CONSTRUCT_LIMIT, vec![drawer + 0xd0]),
                (GEOMETRY_CONSTRUCT_LIMIT, vec![drawer + 0x160]),
                (GEOMETRY_CONSTRUCT_LIMIT, vec![drawer + 0x1f0]),
                (GEOMETRY_CONSTRUCT_LAST, vec![drawer + 0x280]),
                (GEOMETRY_DESTRUCT, vec![drawer + 0x280]),
                (GEOMETRY_DESTRUCT, vec![drawer + 0x1f0]),
                (GEOMETRY_DESTRUCT, vec![drawer + 0x160]),
                (GEOMETRY_DESTRUCT, vec![drawer + 0xd0]),
            ]
        );
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the constructor double saw: its argument words and the size word at +4 of the block.
    type ConstructorCall = (Vec<u32>, u16);

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    /// The constructor sees the block with its size already stored; the answer is the object + 8.
    #[test]
    fn fn_00d699c0_builds_the_object_and_answers_its_second_base() {
        let (mut e, sizes) = allocator_engine();
        let seen: Rc<RefCell<Vec<ConstructorCall>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d6_98b0, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.call(0x00d6_99c0, &args![0x1234u32]).u32();
        assert_eq!(*sizes.borrow(), vec![0x38]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let block = seen[0].0[0];
        assert_eq!(seen[0].0, vec![block, 0x1234]);
        assert_eq!(seen[0].1, 0x38);
        assert_eq!(object, block + 8);
    }

    /// A constructor that answers null gives a null answer.
    #[test]
    fn fn_00d699c0_answers_null_when_the_constructor_does() {
        let (mut e, _) = allocator_engine();
        e.register_double(0x00d6_98b0, |_, _| Ret::default());
        assert_eq!(e.call(0x00d6_99c0, &args![0x1234u32]).u32(), 0);
    }
}
