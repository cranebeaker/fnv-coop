//! `NiXenonRenderer/nid3dgeometrygroupmanager.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocation `00aa13e0` (`cdecl`, size), the base constructor `00a7f810` (unnamed;
/// `thiscall`, no stack words) and the class's virtual table (RTTI
/// `NiD3DGeometryGroupManager`).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const BASE_CONSTRUCT_00A7F810: u32 = 0x00a7_f810;
const NI_D3D_GEOMETRY_GROUP_MANAGER_VTABLE: u32 = 0x010e_f718;

// Translated from 00e88eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dgeometrygroupmanager.cpp`): makes an
/// `NiD3DGeometryGroupManager` (0x1c bytes; the RTTI of the virtual table it stores), `cdecl`:
/// the device and a second word. A made object gets the base constructor, the class's virtual
/// table and +4 to +0x14 and the byte +0x18 cleared; null when the allocation fails, and the
/// stores below still go to it. The device (+0x10) is counted (`AddRef`, its virtual slot 4)
/// and the second word kept at +0x14. Answers the object.
pub fn fn_00e88eb0(e: &mut Engine, device: u32, second: u32) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x1cu32]).u32();
    let manager = if block != 0 {
        e.call(BASE_CONSTRUCT_00A7F810, &args![block]);
        e.mem.set_u32(block, NI_D3D_GEOMETRY_GROUP_MANAGER_VTABLE);
        for offset in [4u32, 8, 0xc, 0x10, 0x14] {
            e.mem.set_u32(block + offset, 0);
        }
        e.mem.set_u8(block + 0x18, 0);
        block
    } else {
        0
    };
    e.mem.set_u32(manager.wrapping_add(0x10), device);
    e.vcall(device, 4, &args![]);
    e.mem.set_u32(manager.wrapping_add(0x14), second);
    manager
}

// Translated from 00e88c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGeometryGroupManager::DestroyGroup` (engine map), `thiscall` with the group. Nothing
/// (0) for a null group or when the manager's virtual slot 0xc (`IsGroupEmpty`) answers 0 for
/// it. Otherwise the group is looked up in the set (array +4, count +0xc; -1 when absent), the
/// count goes down and the last element moves into its place (index -1 writes the word before
/// the array), the group is deleted (its slot 0x24 with 1) and 1 answered.
pub fn ni_d3d_geometry_group_manager_destroy_group(e: &mut Engine, this: Ptr, group: u32) -> bool {
    let base = this.addr();
    if group == 0 {
        return false;
    }
    if !e.vcall(base, 0xc, &args![group]).bool() {
        return false;
    }
    let count = e.mem.u32(base.wrapping_add(0xc));
    let array = e.mem.u32(base.wrapping_add(4));
    let mut index = 0xffff_ffffu32;
    for at in 0..count {
        if e.mem.u32(array.wrapping_add(at.wrapping_mul(4))) == group {
            index = at;
            break;
        }
    }
    let count = e.mem.u32(base.wrapping_add(0xc)).wrapping_sub(1);
    e.mem.set_u32(base.wrapping_add(0xc), count);
    let array = e.mem.u32(base.wrapping_add(4));
    let last = e.mem.u32(array.wrapping_add(count.wrapping_mul(4)));
    e.mem
        .set_u32(array.wrapping_add(index.wrapping_mul(4)), last);
    e.vcall(group, 0x24, &args![1u32]);
    true
}

// Translated from 00e88c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGeometryGroupManager::IsGroupEmpty` (engine map), `thiscall` with the group (`this`
/// not read): whether the group's count (+4) is 0.
pub fn ni_d3d_geometry_group_manager_is_group_empty(
    e: &mut Engine,
    _this: Ptr,
    group: u32,
) -> bool {
    e.mem.u32(group.wrapping_add(4)) == 0
}

// Translated from 00e88d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGeometryGroupManager::RemoveSkinPartitionFromGroup` (engine map), `thiscall` with the
/// skin partition (`this` not read). The group is read from the first partition's buffer data
/// (partitions at +0xc, 0x2c bytes each, count +8; buffer data +0x28, group +8) before
/// anything is checked. Each partition with buffer data is removed from its group (the group's
/// slot 0x10, `thiscall` with the partition); when one was, and the group is now empty (+4 is
/// 0), the group's slot 0 runs. Answers whether one was removed.
pub fn ni_d3d_geometry_group_manager_remove_skin_partition_from_group(
    e: &mut Engine,
    _this: Ptr,
    skin_partition: u32,
) -> bool {
    let count = e.mem.u32(skin_partition.wrapping_add(8));
    let mut partition = e.mem.u32(skin_partition.wrapping_add(0xc));
    let first_data = e.mem.u32(partition.wrapping_add(0x28));
    let group = e.mem.u32(first_data.wrapping_add(8));
    let mut removed = false;
    if count == 0 {
        return removed;
    }
    for _ in 0..count {
        let data = e.mem.u32(partition.wrapping_add(0x28));
        if data != 0 {
            let owner = e.mem.u32(data.wrapping_add(8));
            e.vcall(owner, 0x10, &args![partition]);
            removed = true;
        }
        partition = partition.wrapping_add(0x2c);
    }
    if removed && e.mem.u32(group.wrapping_add(4)) == 0 {
        e.vcall(group, 0, &args![]);
    }
    removed
}

/// What the destructor `00e88d60` runs: `00e94290` (unnamed, no arguments; frees the
/// renderer's shared group buffers), the free `00aa10f0` (`cdecl`, one word) and the base
/// destructor `00a7f820` (unnamed; `thiscall`, no stack words).
const SHARED_CLEAN_UP_00E94290: u32 = 0x00e9_4290;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const BASE_DESTRUCT_00A7F820: u32 = 0x00a7_f820;

// Translated from 00e88d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dgeometrygroupmanager.cpp`): the
/// `NiD3DGeometryGroupManager` destructor, `thiscall`, no stack words. Stores the class's
/// table, deletes every group of the set (slot 0x24 with 1; the count +0xc read once), runs
/// `00e94290`, releases the device (+0x10, slot 8, `stdcall`) when set, frees the set's array
/// (+4) and runs the base destructor `00a7f820` (a tail jump).
pub fn fn_00e88d60(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let count = e.mem.u32(base.wrapping_add(0xc));
    e.mem.set_u32(base, NI_D3D_GEOMETRY_GROUP_MANAGER_VTABLE);
    for index in 0..count {
        let array = e.mem.u32(base.wrapping_add(4));
        let group = e.mem.u32(array.wrapping_add(index.wrapping_mul(4)));
        if group != 0 {
            e.vcall(group, 0x24, &args![1u32]);
        }
    }
    e.call(SHARED_CLEAN_UP_00E94290, &args![]);
    let device = e.mem.u32(base.wrapping_add(0x10));
    if device != 0 {
        e.vcall(device, 8, &args![]);
    }
    let array = e.mem.u32(base.wrapping_add(4));
    e.call(FREE_00AA10F0, &args![array]);
    e.call(BASE_DESTRUCT_00A7F820, &args![base]);
}

// Translated from 00e88dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGeometryGroupManager::AddObjectToGroup` (engine map `_ov2`), `thiscall` with six words
/// (`this` not read): the group, the geometry data, the skin instance, a byte flag and two
/// words never read. With a skin instance, the flag set and a skin partition (+0xc): 0 when the
/// first partition already has buffer data (+0x28); otherwise each partition (+0xc, 0x2c bytes
/// each, count +8) is added to the group (its slot 8, `thiscall`: data, skin instance,
/// partition) and 1 answered. Otherwise: 0 when the data already has buffer data (+0x34),
/// else the group's slot 8 with the data, the skin instance and 0, and 1.
#[allow(clippy::too_many_arguments)]
pub fn ni_d3d_geometry_group_manager_add_object_to_group_ov2(
    e: &mut Engine,
    _this: Ptr,
    group: u32,
    data: u32,
    skin_instance: u32,
    use_partitions: u8,
    _unused_5: u32,
    _unused_6: u32,
) -> bool {
    if skin_instance != 0 && use_partitions != 0 {
        let skin_partition = e.mem.u32(skin_instance.wrapping_add(0xc));
        if skin_partition != 0 {
            let mut partition = e.mem.u32(skin_partition.wrapping_add(0xc));
            if e.mem.u32(partition.wrapping_add(0x28)) != 0 {
                return false;
            }
            let count = e.mem.u32(skin_partition.wrapping_add(8));
            for _ in 0..count {
                e.vcall(group, 8, &args![data, skin_instance, partition]);
                partition = partition.wrapping_add(0x2c);
            }
            return true;
        }
    }
    if e.mem.u32(data.wrapping_add(0x34)) != 0 {
        return false;
    }
    e.vcall(group, 8, &args![data, skin_instance, 0u32]);
    true
}

// Translated from 00e88e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGeometryGroupManager::RemoveObjectFromGroup` (engine map `_ov2`), `thiscall` with two
/// words: the geometry data and the skin instance. With a skin instance that has a skin
/// partition (+0xc) answers `RemoveSkinPartitionFromGroup` on it (with the same `this`).
/// Otherwise 0 when the data has no buffer data (+0x34); else the buffer data's group (+8)
/// removes the data (slot 0x14, `thiscall` with the data), runs its slot 0 when it is now
/// empty (+4 is 0), and 1 is answered.
pub fn ni_d3d_geometry_group_manager_remove_object_from_group_ov2(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    skin_instance: u32,
) -> bool {
    if skin_instance != 0 {
        let skin_partition = e.mem.u32(skin_instance.wrapping_add(0xc));
        if skin_partition != 0 {
            return ni_d3d_geometry_group_manager_remove_skin_partition_from_group(
                e,
                this,
                skin_partition,
            );
        }
    }
    let buffer_data = e.mem.u32(data.wrapping_add(0x34));
    if buffer_data == 0 {
        return false;
    }
    let group = e.mem.u32(buffer_data.wrapping_add(8));
    e.vcall(group, 0x14, &args![data]);
    if e.mem.u32(group.wrapping_add(4)) == 0 {
        e.vcall(group, 0, &args![]);
    }
    true
}

/// The Ni free `00aa1460` (`cdecl`: block, size).
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e88e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGeometryGroupManager`'s scalar deleting destructor (engine map), `thiscall` with the
/// flags: the destructor `00e88d60`, then with bit 0 set the 0x1c bytes freed (`00aa1460`).
/// Answers `this`.
pub fn ni_d3d_geometry_group_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00e88d60(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0x1cu32]);
    }
    this
}

/// The group makers `CreateGroup` picks from (no arguments, each answers the group): `00e94c00`
/// for type 0 (unit `nistaticgeometrygroup.cpp`), `00e94d70` for 1 and `00e95b10` for 2 (unit
/// `nidynamicgeometrygroup.cpp`), and `NiTSet<..>::Realloc` (`00a311e0`, `thiscall` on the
/// set, one word: the new capacity).
const MAKE_GROUP_00E94C00: u32 = 0x00e9_4c00;
const MAKE_GROUP_00E94D70: u32 = 0x00e9_4d70;
const MAKE_GROUP_00E95B10: u32 = 0x00e9_5b10;
const SET_REALLOC_00A311E0: u32 = 0x00a3_11e0;

// Translated from 00e88f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGeometryGroupManager::CreateGroup` (engine map), `thiscall` with the group type: 0,
/// 1 and 2 make the group through `00e94c00`, `00e94d70` and `00e95b10`; any other type
/// answers 0. A group already in the set (array +4, capacity +8, count +0xc) at a non-negative
/// index is answered as is; otherwise it is appended (the set grown through `00a311e0` to twice
/// its capacity, 1 when 0, when full), its +8 takes the device (+0x10), which is counted (slot
/// 4, `stdcall`). Answers the group (0 when the maker answers 0).
pub fn ni_d3d_geometry_group_manager_create_group(
    e: &mut Engine,
    this: Ptr,
    group_type: u32,
) -> u32 {
    let base = this.addr();
    let maker = match group_type {
        0 => MAKE_GROUP_00E94C00,
        1 => MAKE_GROUP_00E94D70,
        2 => MAKE_GROUP_00E95B10,
        _ => return 0,
    };
    let group = e.call(maker, &args![]).u32();
    if group == 0 {
        return 0;
    }
    let count = e.mem.u32(base.wrapping_add(0xc));
    let array = e.mem.u32(base.wrapping_add(4));
    for index in 0..count {
        if e.mem.u32(array.wrapping_add(index.wrapping_mul(4))) == group {
            // `TEST EAX,EAX; JGE`: an index with the top bit set counts as absent.
            if (index as i32) >= 0 {
                return group;
            }
            break;
        }
    }
    let set = base.wrapping_add(4);
    let capacity = e.mem.u32(set.wrapping_add(4));
    if count == capacity {
        let grown = if capacity != 0 {
            capacity.wrapping_add(capacity)
        } else {
            1
        };
        e.call(SET_REALLOC_00A311E0, &args![set, grown]);
    }
    let used = e.mem.u32(set.wrapping_add(8));
    let array = e.mem.u32(set);
    e.mem
        .set_u32(array.wrapping_add(used.wrapping_mul(4)), group);
    let used = e.mem.u32(set.wrapping_add(8)).wrapping_add(1);
    e.mem.set_u32(set.wrapping_add(8), used);
    let device = e.mem.u32(base.wrapping_add(0x10));
    e.mem.set_u32(group.wrapping_add(8), device);
    e.vcall(device, 4, &args![]);
    group
}

// Translated from 00e88ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGeometryGroupManager::AddObjectToGroup` (engine map), `thiscall` with two words
/// (`this` not read): the group and an object. 0 when the object already has a group entry
/// (+0x1c); else the group's slot 4 with the object, and 1.
pub fn ni_d3d_geometry_group_manager_add_object_to_group(
    e: &mut Engine,
    _this: Ptr,
    group: u32,
    object: u32,
) -> bool {
    if e.mem.u32(object.wrapping_add(0x1c)) != 0 {
        return false;
    }
    e.vcall(group, 4, &args![object]);
    true
}

// Translated from 00e88cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGeometryGroupManager::RemoveObjectFromGroup` (engine map), `thiscall` with one word
/// (`this` not read): the object. 0 when it has no group entry (+0x1c); else the entry's group
/// (+8) removes it (slot 0xc), runs its slot 0 when it is now empty (+4 is 0), and 1.
pub fn ni_d3d_geometry_group_manager_remove_object_from_group(
    e: &mut Engine,
    _this: Ptr,
    object: u32,
) -> bool {
    let entry = e.mem.u32(object.wrapping_add(0x1c));
    if entry == 0 {
        return false;
    }
    let group = e.mem.u32(entry.wrapping_add(8));
    e.vcall(group, 0xc, &args![object]);
    if e.mem.u32(group.wrapping_add(4)) == 0 {
        e.vcall(group, 0, &args![]);
    }
    true
}

// Translated from 00e88fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DGeometryGroupManager::AddObjectToGroup` (engine map `_ov3`), `thiscall` with three
/// words (group, geometry data, skin instance): the `_ov2` form with the byte flag at
/// `this`+0x18 and two zero words. Answers what it answers.
pub fn ni_d3d_geometry_group_manager_add_object_to_group_ov3(
    e: &mut Engine,
    this: Ptr,
    group: u32,
    data: u32,
    skin_instance: u32,
) -> bool {
    let flag = e.mem.u8(this.addr() + 0x18);
    ni_d3d_geometry_group_manager_add_object_to_group_ov2(
        e,
        this,
        group,
        data,
        skin_instance,
        flag,
        0,
        0,
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e8_8ca0,
            ni_d3d_geometry_group_manager_add_object_to_group(Ptr, u32, u32) -> bool
        ),
        entry!(
            0x00e8_8cc0,
            ni_d3d_geometry_group_manager_remove_object_from_group(Ptr, u32) -> bool
        ),
        entry!(
            0x00e8_8fa0,
            ni_d3d_geometry_group_manager_add_object_to_group_ov3(Ptr, u32, u32, u32) -> bool
        ),
        entry!(0x00e8_8eb0, fn_00e88eb0(u32, u32) -> u32),
        entry!(
            0x00e8_8c30,
            ni_d3d_geometry_group_manager_destroy_group(Ptr, u32) -> bool
        ),
        entry!(
            0x00e8_8c90,
            ni_d3d_geometry_group_manager_is_group_empty(Ptr, u32) -> bool
        ),
        entry!(
            0x00e8_8d00,
            ni_d3d_geometry_group_manager_remove_skin_partition_from_group(Ptr, u32) -> bool
        ),
        entry!(0x00e8_8d60, fn_00e88d60(Ptr)),
        entry!(
            0x00e8_8dc0,
            ni_d3d_geometry_group_manager_add_object_to_group_ov2(
                Ptr, u32, u32, u32, u8, u32, u32
            ) -> bool
        ),
        entry!(
            0x00e8_8e40,
            ni_d3d_geometry_group_manager_remove_object_from_group_ov2(Ptr, u32, u32) -> bool
        ),
        entry!(
            0x00e8_8e90,
            ni_d3d_geometry_group_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e8_8f10,
            ni_d3d_geometry_group_manager_create_group(Ptr, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn fn_00e88eb0_makes_the_manager_and_counts_the_device() {
        let mut e = Engine::new();
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT_00A7F810, |_, a| a[0].into_ret());
        e.put_vtable(0x6000, &[0, 0x6100]);
        e.register(0x6100, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        e.call_log = Some(vec![]);
        let manager = e.call(0x00e8_8eb0, &args![device, 0x77u32]).u32();
        assert_eq!(e.mem.u32(manager), NI_D3D_GEOMETRY_GROUP_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(manager + 0x10), device);
        assert_eq!(e.mem.u32(manager + 0x14), 0x77);
        assert_eq!(e.mem.u8(manager + 0x18), 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x6100, vec![device])));
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    const GROUP_VTABLE: u32 = 0x0e40_0000;
    const GROUP_SLOT_0: u32 = 0x0e41_0000;
    const GROUP_ADD: u32 = 0x0e41_0008;
    const GROUP_REMOVE_PARTITION: u32 = 0x0e41_0010;
    const GROUP_REMOVE: u32 = 0x0e41_0014;
    const GROUP_DELETE: u32 = 0x0e41_0024;
    const MANAGER_VTABLE: u32 = 0x0e42_0000;
    const IS_EMPTY: u32 = 0x0e43_000c;
    const DEVICE_VTABLE: u32 = 0x0e44_0000;
    const ADD_REF: u32 = 0x0e45_0004;
    const RELEASE: u32 = 0x0e45_0008;

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
        let mut slots = vec![0u32; 10];
        slots[0] = GROUP_SLOT_0;
        slots[2] = GROUP_ADD;
        slots[4] = GROUP_REMOVE_PARTITION;
        slots[5] = GROUP_REMOVE;
        slots[9] = GROUP_DELETE;
        e.put_vtable(GROUP_VTABLE, &slots);
        e.put_vtable(MANAGER_VTABLE, &[0, 0, 0, IS_EMPTY]);
        e.put_vtable(DEVICE_VTABLE, &[0, ADD_REF, RELEASE]);
        for no_op in [
            GROUP_SLOT_0,
            GROUP_ADD,
            GROUP_REMOVE_PARTITION,
            GROUP_REMOVE,
            GROUP_DELETE,
            ADD_REF,
            RELEASE,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e
    }

    fn group(e: &mut Engine, count: u32) -> u32 {
        let group = e.mem.alloc(0x10);
        e.mem.set_u32(group, GROUP_VTABLE);
        e.mem.set_u32(group + 4, count);
        group
    }

    /// A manager with a set of the given groups (capacity `capacity`, one spare word before
    /// the array) and a device.
    fn manager(e: &mut Engine, groups: &[u32], capacity: u32) -> u32 {
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this, MANAGER_VTABLE);
        let array = e.mem.alloc(4 * capacity.max(1) + 4) + 4;
        for (index, group) in groups.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *group);
        }
        e.mem.set_u32(this + 4, array);
        e.mem.set_u32(this + 8, capacity);
        e.mem.set_u32(this + 0xc, groups.len() as u32);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        e.mem.set_u32(this + 0x10, device);
        this
    }

    #[test]
    fn destroy_group_removes_an_empty_group() {
        let mut e = engine();
        e.register(IS_EMPTY, |_, _| 1u32.into_ret());
        let (a, b, c) = (group(&mut e, 0), group(&mut e, 0), group(&mut e, 0));
        let this = manager(&mut e, &[a, b, c], 4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e8_8c30, &args![this, a]).bool());
        let array = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(array), c);
        assert_eq!(calls(&e, GROUP_DELETE), vec![vec![a, 1]]);
        assert_eq!(calls(&e, IS_EMPTY), vec![vec![this, a]]);
        // Absent: the word before the array takes the last one.
        let d = group(&mut e, 0);
        e.call(0x00e8_8c30, &args![this, d]);
        assert_eq!(e.mem.u32(array - 4), b);
        // Not empty, or null: 0.
        e.register(IS_EMPTY, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e8_8c30, &args![this, c]).bool());
        assert!(!e.call(0x00e8_8c30, &args![this, 0u32]).bool());
    }

    #[test]
    fn is_group_empty_reads_the_count() {
        let mut e = engine();
        let empty = group(&mut e, 0);
        let full = group(&mut e, 3);
        assert!(e.call(0x00e8_8c90, &args![0u32, empty]).bool());
        assert!(!e.call(0x00e8_8c90, &args![0u32, full]).bool());
    }

    /// A skin partition of `count` partitions; those listed in `with_data` get buffer data
    /// owned by `owner`.
    fn skin_partition(e: &mut Engine, count: u32, with_data: &[u32], owner: u32) -> u32 {
        let partitions = e.mem.alloc(0x2c * count.max(1));
        for index in with_data {
            let data = e.mem.alloc(0xc);
            e.mem.set_u32(data + 8, owner);
            e.mem.set_u32(partitions + 0x2c * index + 0x28, data);
        }
        let skin = e.mem.alloc(0x10);
        e.mem.set_u32(skin + 8, count);
        e.mem.set_u32(skin + 0xc, partitions);
        skin
    }

    #[test]
    fn remove_skin_partition_from_group_removes_and_deletes_an_empty_group() {
        let mut e = engine();
        let owner = group(&mut e, 0);
        let skin = skin_partition(&mut e, 3, &[0, 2], owner);
        let partitions = e.mem.u32(skin + 0xc);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e8_8d00, &args![0u32, skin]).bool());
        assert_eq!(
            calls(&e, GROUP_REMOVE_PARTITION),
            vec![vec![owner, partitions], vec![owner, partitions + 0x58]]
        );
        assert_eq!(calls(&e, GROUP_SLOT_0), vec![vec![owner]]);
        // A group still holding something is kept.
        e.mem.set_u32(owner + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e8_8d00, &args![0u32, skin]);
        assert!(calls(&e, GROUP_SLOT_0).is_empty());
        // No partitions (the first one's data is still read): 0.
        e.mem.set_u32(skin + 8, 0);
        assert!(!e.call(0x00e8_8d00, &args![0u32, skin]).bool());
    }

    #[test]
    fn destructor_deletes_the_groups_and_releases_the_device() {
        let mut e = engine();
        e.register(SHARED_CLEAN_UP_00E94290, |_, _| Ret::default());
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.register(BASE_DESTRUCT_00A7F820, |_, _| Ret::default());
        e.register(NI_FREE_00AA1460, |_, _| Ret::default());
        let a = group(&mut e, 0);
        let this = manager(&mut e, &[a, 0], 2);
        let device = e.mem.u32(this + 0x10);
        let array = e.mem.u32(this + 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_8e90, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, GROUP_DELETE), vec![vec![a, 1]]);
        assert_eq!(calls(&e, SHARED_CLEAN_UP_00E94290), vec![Vec::<u32>::new()]);
        assert_eq!(calls(&e, RELEASE), vec![vec![device]]);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![array]]);
        assert_eq!(calls(&e, BASE_DESTRUCT_00A7F820), vec![vec![this]]);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![this, 0x1c]]);
        assert_eq!(e.mem.u32(this), NI_D3D_GEOMETRY_GROUP_MANAGER_VTABLE);
        // Without bit 0 and without a device.
        e.mem.set_u32(this + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_8e90, &args![this, 0u32]);
        assert!(calls(&e, RELEASE).is_empty());
        assert!(calls(&e, NI_FREE_00AA1460).is_empty());
    }

    #[test]
    fn add_object_to_group_adds_partitions_or_the_data() {
        let mut e = engine();
        let target = group(&mut e, 0);
        let skin = skin_partition(&mut e, 2, &[], 0);
        let partitions = e.mem.u32(skin + 0xc);
        let instance = e.mem.alloc(0x10);
        e.mem.set_u32(instance + 0xc, skin);
        let data = e.mem.alloc(0x38);
        let add = |e: &mut Engine, instance: u32, flag: u32| {
            e.call(
                0x00e8_8dc0,
                &args![0u32, target, data, instance, flag, 0u32, 0u32],
            )
            .bool()
        };
        e.call_log = Some(vec![]);
        assert!(add(&mut e, instance, 1));
        assert_eq!(
            calls(&e, GROUP_ADD),
            vec![
                vec![target, data, instance, partitions],
                vec![target, data, instance, partitions + 0x2c]
            ]
        );
        // The first partition already has data: 0.
        e.mem.set_u32(partitions + 0x28, 1);
        assert!(!add(&mut e, instance, 1));
        // The flag clear: the data itself.
        e.call_log = Some(vec![]);
        assert!(add(&mut e, instance, 0));
        assert_eq!(calls(&e, GROUP_ADD), vec![vec![target, data, instance, 0]]);
        e.mem.set_u32(data + 0x34, 1);
        assert!(!add(&mut e, 0, 1));
    }

    #[test]
    fn remove_object_from_group_removes_the_data_or_the_partitions() {
        let mut e = engine();
        let owner = group(&mut e, 0);
        let buffer_data = e.mem.alloc(0xc);
        e.mem.set_u32(buffer_data + 8, owner);
        let data = e.mem.alloc(0x38);
        e.mem.set_u32(data + 0x34, buffer_data);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e8_8e40, &args![0u32, data, 0u32]).bool());
        assert_eq!(calls(&e, GROUP_REMOVE), vec![vec![owner, data]]);
        assert_eq!(calls(&e, GROUP_SLOT_0), vec![vec![owner]]);
        // A skin instance with a partition goes to RemoveSkinPartitionFromGroup.
        let skin = skin_partition(&mut e, 1, &[0], owner);
        let instance = e.mem.alloc(0x10);
        e.mem.set_u32(instance + 0xc, skin);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e8_8e40, &args![0u32, data, instance]).bool());
        assert_eq!(calls(&e, GROUP_REMOVE_PARTITION).len(), 1);
        assert!(calls(&e, GROUP_REMOVE).is_empty());
        // No buffer data: 0.
        e.mem.set_u32(data + 0x34, 0);
        assert!(!e.call(0x00e8_8e40, &args![0u32, data, 0u32]).bool());
    }

    #[test]
    fn create_group_appends_a_new_group() {
        let mut e = engine();
        let made = group(&mut e, 0);
        e.register_double(MAKE_GROUP_00E94D70, move |_, _| made.into_ret());
        e.register(MAKE_GROUP_00E95B10, |_, _| 0u32.into_ret());
        e.register(SET_REALLOC_00A311E0, |e, a| {
            let array = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0]);
            let used = e.mem.u32(a[0] + 8);
            let bytes = e.mem.bytes(old, 4 * used);
            e.mem.write(array, &bytes);
            e.mem.set_u32(a[0], array);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        let this = manager(&mut e, &[], 0);
        let device = e.mem.u32(this + 0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_8f10, &args![this, 1u32]).u32(), made);
        assert_eq!(calls(&e, SET_REALLOC_00A311E0), vec![vec![this + 4, 1]]);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        let array = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(array), made);
        assert_eq!(e.mem.u32(made + 8), device);
        assert_eq!(calls(&e, ADD_REF), vec![vec![device]]);
        // Already there: answered as is.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_8f10, &args![this, 1u32]).u32(), made);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert!(calls(&e, ADD_REF).is_empty());
        // A maker answering 0, and an unknown type.
        assert_eq!(e.call(0x00e8_8f10, &args![this, 2u32]).u32(), 0);
        assert_eq!(e.call(0x00e8_8f10, &args![this, 3u32]).u32(), 0);
    }
}

#[cfg(test)]
mod n007_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A group whose slots 0, 4, 8 and 0xc are doubles at `0x034e_0000 + slot`.
    fn group(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x10);
        for slot in [0u32, 4, 8, 0xc] {
            e.mem.set_u32(vtable + slot, 0x034e_0000 + slot);
            e.register(0x034e_0000 + slot, |_, _| Ret::default());
        }
        let group = e.mem.alloc(8);
        e.mem.set_u32(group, vtable);
        group
    }

    #[test]
    fn add_and_remove_object() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let group = group(&mut e);
        let object = e.mem.alloc(0x20);
        assert!(e.call(0x00e8_8ca0, &args![0u32, group, object]).bool());
        assert_eq!(calls(&e, 0x034e_0004), vec![vec![group, object]]);
        let entry = e.mem.alloc(0xc);
        e.mem.set_u32(entry + 8, group);
        e.mem.set_u32(object + 0x1c, entry);
        assert!(!e.call(0x00e8_8ca0, &args![0u32, group, object]).bool());
        // Removal: the group drops it; with objects left (+4) it is kept.
        e.mem.set_u32(group + 4, 1);
        assert!(e.call(0x00e8_8cc0, &args![0u32, object]).bool());
        assert_eq!(calls(&e, 0x034e_000c), vec![vec![group, object]]);
        assert!(calls(&e, 0x034e_0000).is_empty());
        e.mem.set_u32(group + 4, 0);
        assert!(e.call(0x00e8_8cc0, &args![0u32, object]).bool());
        assert_eq!(calls(&e, 0x034e_0000), vec![vec![group]]);
        e.mem.set_u32(object + 0x1c, 0);
        assert!(!e.call(0x00e8_8cc0, &args![0u32, object]).bool());
    }

    #[test]
    fn add_with_the_managers_flag() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let group = group(&mut e);
        let manager = e.mem.alloc(0x1c);
        e.mem.set_u8(manager + 0x18, 1);
        // A skin instance with a partition list of one partition: added through slot 8.
        let partitions = e.mem.alloc(0x2c);
        let skin_partition = e.mem.alloc(0x10);
        e.mem.set_u32(skin_partition + 8, 1);
        e.mem.set_u32(skin_partition + 0xc, partitions);
        let skin = e.mem.alloc(0x10);
        e.mem.set_u32(skin + 0xc, skin_partition);
        let data = e.mem.alloc(0x38);
        assert!(e
            .call(0x00e8_8fa0, &args![manager, group, data, skin])
            .bool());
        assert_eq!(
            calls(&e, 0x034e_0008),
            vec![vec![group, data, skin, partitions]]
        );
        // Flag clear: the data itself.
        e.mem.set_u8(manager + 0x18, 0);
        assert!(e
            .call(0x00e8_8fa0, &args![manager, group, data, skin])
            .bool());
        assert_eq!(calls(&e, 0x034e_0008)[1], vec![group, data, skin, 0]);
    }
}
