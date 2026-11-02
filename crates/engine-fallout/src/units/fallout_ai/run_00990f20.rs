//! `fallout/ai/run_00990f20` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The range holds the life cycle of the combat manager, the 0x3C-byte
//! object whose pointer sits at `011f1958`. Identity: `00990f20` and
//! `00991500` pass the source file name `...\AI\Combat\CombatManager.cpp`
//! (`0108d9f0`) to a scope guard, and the Xbox PDB `CombatManager` is 0x3C
//! bytes with its fields at the same offsets the constructor writes
//! (+0x20, +0x24, +0x2C, +0x34, +0x38). The engine map names none of these
//! functions, so they keep their address names.
//!
//! Functions that open a C++ exception frame (`FS:[0]`) carry a
//! `Translation incomplete` marker: the unwinding states are not translated.
//! The x87 loads of float globals are `FLD`/`FSTP` copies, which quiet a
//! signalling NaN (`x87_quiet`).

#[allow(unused_imports)]
use crate::prelude::*;

/// Allocator (`MemoryManager::Allocate`-like, cdecl `(size)`).
const ALLOCATE: u32 = 0x0040_1000;
/// Deallocator (cdecl `(block)`).
const FREE: u32 = 0x0040_1030;
/// Scope guard constructor on a 4-byte stack object:
/// `(this, 0x2b, 1, source file name, line)`.
const SCOPE_GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// Element count of an `NiTPrimitiveArray` (the word at +0xA).
const ARRAY_COUNT: u32 = 0x0065_8930;
/// Address of element `(index)` of an array (`this[+4] + 4 * index`).
const ARRAY_SLOT: u32 = 0x0087_7a30;
/// `NiPointer` getter (reads the pointer at `this`).
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer` assignment `(this, new pointer)`.
const NI_POINTER_SET: u32 = 0x0066_b0d0;
/// Reads the word at +0x18 of a node.
const NODE_PARENT: u32 = 0x0096_11e0;
/// Number of members of a combat group (`group + 0x18`'s array count).
const MEMBER_COUNT: u32 = 0x0099_0890;
/// Member `(index)` of a combat group.
const MEMBER_AT: u32 = 0x0090_3600;
/// Combat group predicate used before updating or deleting a group.
const GROUP_PREDICATE: u32 = 0x0098_6c40;
/// Reads the word at +4 of a combat group (its id).
const GROUP_ID: u32 = 0x0072_6070;

/// The combat manager singleton pointer.
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout\AI\Combat\CombatManager.cpp"`.
const SOURCE_FILE: u32 = 0x0108_d9f0;
/// `FLT_MAX`: the constructor starts the music time stamp at its negation.
const FLOAT_MAX: u32 = 0x0108_d9b0;
/// Global float that `009915e0` accumulates into.
const ACCUMULATED_VALUE: u32 = 0x011f_1bf0;
/// Object whose float at +0xC `0084d030` returns (the update step).
const STEP_SOURCE: u32 = 0x011f_6394;
/// The player character pointer.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// The process lists object.
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// A settings object whose first byte enables the debug drawing (`00408d60`
/// returns the address of that byte).
const DEBUG_DRAW_SETTING: u32 = 0x011f_197c;
/// RGBA components of the debug colors (`float` constants).
const COLOR_QUARTER: u32 = 0x0101_622c;
const COLOR_HALF: u32 = 0x0101_6248;
const COLOR_THREE_QUARTERS: u32 = 0x0101_6264;
const COLOR_GREEN_FACTOR: u32 = 0x0108_da38;
const COLOR_BLUE_FACTOR: u32 = 0x0108_aff0;
/// `32.0` (`double`): added to the target's radius word.
const RADIUS_PADDING: u32 = 0x0102_f070;
/// `2.0` (`float`): arrow size.
const ARROW_SIZE: u32 = 0x0101_62c0;

layout! {
    /// `CombatManager` (Xbox PDB), 0x3C bytes. The `CombatGroups` array
    /// (`NiTPrimitiveArray<CombatGroup *>`) starts at +0 and is passed as
    /// `this` to the array accessors; the two `CombatTimer`s (+0x24, +0x2C)
    /// are addressed as `this + offset`.
    pub struct CombatManager: 0x3c {
        /// `pLocalActorTargetingMap` (Xbox PDB): `NiTMap<Actor *, __int64>*`.
        0x10 pLocalActorTargetingMap: Ptr,
        /// `pThreatMap` (Xbox PDB): `CombatThreatMap*`.
        0x14 pThreatMap: Ptr,
        /// `pCoverMap` (Xbox PDB): `CombatCoverMap*`.
        0x18 pCoverMap: Ptr,
        /// `pTargetMap` (Xbox PDB): `CombatTargetMap*`.
        0x1c pTargetMap: Ptr,
        /// `fCombatMusicStartedTimeStamp` (Xbox PDB).
        0x20 fCombatMusicStartedTimeStamp: f32,
        /// `iNextGroupID` (Xbox PDB).
        0x34 iNextGroupID: u32,
        /// `spDebugGeometry` (Xbox PDB): `NiPointer<NiNode>`.
        0x38 spDebugGeometry: Ptr,
    }
}

/// A float global as `FLD`/`FSTP` copies it.
fn float_global(e: &Engine, address: u32) -> f32 {
    x87_quiet(e.global::<f32>(address))
}

fn group_count(e: &mut Engine, manager: u32) -> u32 {
    e.call(ARRAY_COUNT, &args![manager]).u32()
}

/// The group pointer stored in slot `index`.
fn group_at(e: &mut Engine, manager: u32, index: u32) -> u32 {
    let slot = e.call(ARRAY_SLOT, &args![manager, index]).u32();
    e.mem.u32(slot)
}

fn member_count(e: &mut Engine, group: u32) -> u32 {
    e.call(MEMBER_COUNT, &args![group]).u32()
}

/// Allocates `size` bytes and, when the allocation succeeded, constructs the
/// object with `constructor(block, extra...)`; 0 when the allocation failed.
fn new_object(e: &mut Engine, size: u32, constructor: u32, extra: &[u32]) -> u32 {
    let block = e.call(ALLOCATE, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let mut words = vec![block];
    words.extend_from_slice(extra);
    e.call(constructor, &words).u32()
}

/// The shared tail of `00991310` and `00991770`: detaches the debug node
/// from the object it hangs on and clears the `NiPointer` at +0x38.
fn clear_debug_geometry(e: &mut Engine, manager: u32) {
    let pointer = manager.wrapping_add(0x38);
    if e.call(NI_POINTER_GET, &args![pointer]).u32() != 0 {
        let node = e.call(NI_POINTER_GET, &args![pointer]).u32();
        if e.call(NODE_PARENT, &args![node]).u32() != 0 {
            let node = e.call(NI_POINTER_GET, &args![pointer]).u32();
            let parent = e.call(NODE_PARENT, &args![node]).u32();
            let child = e.call(NI_POINTER_GET, &args![pointer]).u32();
            // Virtual slot +0xE8 of the node's parent, with the node.
            e.vcall(parent, 0xe8, &args![child]);
        }
    }
    e.call(NI_POINTER_SET, &args![pointer, 0u32]);
}

// Translated from 00990f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the combat manager when there is none: allocates and constructs
/// the 0x3C-byte object (`00991080`), stores it at `011f1958`, then runs the
/// initializers `0097a990`, `00984a50`, `00990070`, `009a6050` and
/// `00983910`. The whole body runs inside a scope guard.
// Translation incomplete: 00990f20 - the C++ exception unwinding frame (FS:[0]) is not translated.
pub fn fn_00990f20(e: &mut Engine) {
    if e.global::<u32>(COMBAT_MANAGER) != 0 {
        return;
    }
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![guard, 0x2bu32, 1u32, SOURCE_FILE, 0x43u32],
        );
        let manager = new_object(e, 0x3c, 0x0099_1080, &[]);
        e.set_global(COMBAT_MANAGER, manager);
        e.call(0x0097_a990, &args![]);
        e.call(0x0098_4a50, &args![]);
        e.call(0x0099_0070, &args![]);
        e.call(0x009a_6050, &args![]);
        e.call(0x0098_3910, &args![]);
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
    });
}

// Translated from 00990ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the combat manager: runs the teardown functions `0097aa40`,
/// `00984a90`, `009900b0` and `00983940`, deletes the object at `011f1958`
/// through its scalar deleting destructor (`00991050`, flag 1) and clears
/// the pointer.
pub fn fn_00990ff0(e: &mut Engine) {
    e.call(0x0097_aa40, &args![]);
    e.call(0x0098_4a90, &args![]);
    e.call(0x0099_00b0, &args![]);
    e.call(0x0098_3940, &args![]);
    let manager = e.global::<u32>(COMBAT_MANAGER);
    if manager != 0 {
        fn_00991050(e, Ptr::new(manager), 1);
    }
    e.set_global(COMBAT_MANAGER, 0u32);
}

// Translated from 00991050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the 0x3C-byte combat manager object: runs
/// the destructor (`00991150`) and frees the block when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_00991050(e: &mut Engine, this: Ptr<CombatManager>, flags: u32) -> Ptr<CombatManager> {
    fn_00991150(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00991080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the combat manager object: base array constructor
/// (`00993960(this, 0, 1)`), music time stamp `-FLT_MAX`, the two
/// `CombatTimer`s cleared (`00906f60` at +0x24 and +0x2C), `iNextGroupID` 1,
/// the debug geometry `NiPointer` at +0x38 set to null, then the four
/// sub-objects (`009911c0`). Returns `this`.
// Translation incomplete: 00991080 - the C++ exception unwinding frame (FS:[0]) is not translated.
pub fn fn_00991080(e: &mut Engine, this: Ptr<CombatManager>) -> Ptr<CombatManager> {
    e.call(0x0099_3960, &args![this, 0u32, 1u32]);
    let stamp = -float_global(e, FLOAT_MAX);
    e.call(0x0043_5de0, &args![this.addr() + 0x20, stamp]);
    e.call(0x0090_6f60, &args![this.addr() + 0x24]);
    e.call(0x0090_6f60, &args![this.addr() + 0x2c]);
    e.set(this, CombatManager::iNextGroupID, 1);
    e.call(0x0063_3c90, &args![this.addr() + 0x38, 0u32]);
    fn_009911c0(e, this);
    this
}

// Translated from 00991130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the destructor of the base array part (`00993880`).
pub fn fn_00991130(e: &mut Engine, this: Ptr<CombatManager>) {
    e.call(0x0099_3880, &args![this]);
}

// Translated from 00991150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the combat manager object: destroys the groups and
/// sub-objects without the per-group call (`00991310(this, 0)`), releases the
/// debug geometry `NiPointer` (`0045cec0` at +0x38) and runs the base
/// destructor (`00991130`).
// Translation incomplete: 00991150 - the C++ exception unwinding frame (FS:[0]) is not translated.
pub fn fn_00991150(e: &mut Engine, this: Ptr<CombatManager>) {
    fn_00991310(e, this, 0);
    e.call(0x0045_cec0, &args![this.addr() + 0x38]);
    fn_00991130(e, this);
}

// Translated from 009911c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the four sub-objects of the manager: the 0x1C-byte object
/// (`00983080`) into +0x18, the 0x18-byte object (`009a4fb0`) into +0x14,
/// the 0x10-byte object (`009a3e90`) into +0x1C and the 0x10-byte object
/// (`00993820(0x25)`) into +0x10. Each stays 0 when its allocation fails.
// Translation incomplete: 009911c0 - the C++ exception unwinding frame (FS:[0]) is not translated.
pub fn fn_009911c0(e: &mut Engine, this: Ptr<CombatManager>) {
    let cover_map = new_object(e, 0x1c, 0x0098_3080, &[]);
    e.set(this, CombatManager::pCoverMap, Ptr::new(cover_map));
    let threat_map = new_object(e, 0x18, 0x009a_4fb0, &[]);
    e.set(this, CombatManager::pThreatMap, Ptr::new(threat_map));
    let target_map = new_object(e, 0x10, 0x009a_3e90, &[]);
    e.set(this, CombatManager::pTargetMap, Ptr::new(target_map));
    let targeting_map = new_object(e, 0x10, 0x0099_3820, &[0x25]);
    e.set(
        this,
        CombatManager::pLocalActorTargetingMap,
        Ptr::new(targeting_map),
    );
}

// Translated from 00991310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tears the manager's contents down: deletes every group in the array
/// (scalar deleting destructor `009914d0`, flag 1; with `per_group_call`
/// nonzero `00986b70` runs on each group first), runs `005e03d0`, deletes
/// the sub-objects at +0x18, +0x14 and +0x1C through their virtual slot 0
/// (deleting destructor, flag 1), runs `00438af0` on the object at +0x10
/// (without a null test), deletes it too, and clears the debug geometry.
pub fn fn_00991310(e: &mut Engine, this: Ptr<CombatManager>, per_group_call: u8) {
    let manager = this.addr();
    let mut index = 0u32;
    while index < group_count(e, manager) {
        let group = group_at(e, manager, index);
        if group != 0 {
            if per_group_call != 0 {
                e.call(0x0098_6b70, &args![group]);
            }
            fn_009914d0(e, Ptr::new(group), 1);
        }
        index += 1;
    }
    e.call(0x005e_03d0, &args![manager]);
    // The cover map (+0x18), the threat map (+0x14) and the target map (+0x1C).
    for offset in [0x18u32, 0x14, 0x1c] {
        let sub_object = e.mem.u32(manager + offset);
        if sub_object != 0 {
            e.vcall(sub_object, 0, &args![1u32]);
        }
    }
    let targeting_map = e.get(this, CombatManager::pLocalActorTargetingMap);
    e.call(0x0043_8af0, &args![targeting_map]);
    if targeting_map.addr() != 0 {
        e.vcall(targeting_map.addr(), 0, &args![1u32]);
    }
    clear_debug_geometry(e, manager);
}

// Translated from 009914d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of a combat group: runs the destructor
/// (`00985f90`) and frees the block when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009914d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0098_5f90, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00991500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame update of the manager, inside a scope guard. With
/// `skip_update` zero: adds the float at +0xC of the object at `011f6394`
/// (`0084d030`) to the accumulator at `011f1bf0` (`009915e0`), runs
/// `00986150` on every group for which `00986c40` is false and finishes
/// with `009a50b0` on the threat map.
// Translation incomplete: 00991500 - the C++ exception unwinding frame (FS:[0]) is not translated.
pub fn fn_00991500(e: &mut Engine, this: Ptr<CombatManager>, skip_update: u8) {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![guard, 0x2bu32, 1u32, SOURCE_FILE, 0x8bu32],
        );
        if skip_update == 0 {
            let step = e.call(0x0084_d030, &args![STEP_SOURCE]).f32();
            fn_009915e0(e, step);
            let manager = this.addr();
            let mut index = 0u32;
            while index < group_count(e, manager) {
                let group = group_at(e, manager, index);
                if group != 0 && !e.call(GROUP_PREDICATE, &args![group]).bool() {
                    e.call(0x0098_6150, &args![group]);
                }
                index += 1;
            }
            let threat_map = e.get(this, CombatManager::pThreatMap);
            e.call(0x009a_50b0, &args![threat_map]);
        }
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
    });
}

// Translated from 009915e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `amount` to the global float at `011f1bf0`.
pub fn fn_009915e0(e: &mut Engine, amount: f32) {
    let total = e.global::<f32>(ACCUMULATED_VALUE);
    e.set_global(
        ACCUMULATED_VALUE,
        (f64::from(total) + f64::from(amount)) as f32,
    );
}

// Translated from 00991600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the groups for which `00986c40` is true: each is deleted
/// (`009914d0`, flag 1) and its slot set to null (`0096ae90(index, &0)`);
/// when `009938b0` (the word at +0xC) is then 0 the array is compacted
/// (`00600bc0`) and `00438af0` runs on the object at +0x10. Unless
/// `skip_member_pass` is nonzero, every member of every remaining group
/// then gets its virtual slot +0x428 called and `0097ef50` runs on a
/// nonzero result. Ends with `00991770`.
pub fn fn_00991600(e: &mut Engine, this: Ptr<CombatManager>, skip_member_pass: u8) {
    let manager = this.addr();
    let count = group_count(e, manager);
    if count != 0 {
        for index in 0..count {
            let group = group_at(e, manager, index);
            if group != 0 && e.call(GROUP_PREDICATE, &args![group]).bool() {
                fn_009914d0(e, Ptr::new(group), 1);
                e.with_stack(4, |e, null_slot| {
                    e.mem.set_u32(null_slot.addr(), 0);
                    e.call(0x0096_ae90, &args![manager, index, null_slot]);
                });
            }
        }
        if e.call(0x0099_38b0, &args![manager]).u32() == 0 {
            e.call(0x0060_0bc0, &args![manager]);
            let targeting_map = e.get(this, CombatManager::pLocalActorTargetingMap);
            e.call(0x0043_8af0, &args![targeting_map]);
        }
    }
    if skip_member_pass == 0 {
        let mut index = 0u32;
        while index < group_count(e, manager) {
            let group = group_at(e, manager, index);
            if group != 0 {
                let members = member_count(e, group);
                for member_index in 0..members {
                    let member = e.call(MEMBER_AT, &args![group, member_index]).u32();
                    let target = e.vcall(member, 0x428, &args![]).u32();
                    if target != 0 {
                        e.call(0x0097_ef50, &args![target]);
                    }
                }
            }
            index += 1;
        }
    }
    fn_00991770(e, this);
}

// Translated from 00991770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the manager's debug drawing. Runs `00483710` on the threat map,
/// cover map and target map; for every group runs `009878b0(group, 0)` and
/// calls the members' virtual slot +0x428, passing a nonzero result to
/// `004534f0(result, 0)`; clears the debug geometry node. When the player
/// character has a process whose `00524cf0` answers nonzero and the setting
/// byte at `011f197c` is set, it creates the node (`NiNode`, 0xAC bytes,
/// `00a5ecb0(0)`) when missing, and for the player and every actor of the
/// process lists for which virtual slot +0x100 answers true it adds a sphere
/// (`00c56680`, radius = the target's first word + 32.0) at the target's
/// position and an arrow (`00c5a0f0`) from the actor's location (virtual
/// slot +0x1F4) to it. Finally, with a node present, it attaches a property
/// (0x1C bytes, `004391c0`), prepares the node and attaches it to the object
/// of the player's process, and updates it.
///
/// The three color objects (`00414430` RGBA): A (0, 0.25, 0.25, 0.5) for the
/// arrow's first color, B (0, 0.75, 0.75, 0.5) for the sphere and the arrow's
/// second color, and a third color (1.0, ...) that is built but never used.
// Translation incomplete: 00991770 - the C++ exception unwinding frame (FS:[0]) is not translated.
pub fn fn_00991770(e: &mut Engine, this: Ptr<CombatManager>) {
    let manager = this.addr();
    // The threat map (+0x14), the cover map (+0x18) and the target map (+0x1C).
    for offset in [0x14u32, 0x18, 0x1c] {
        let map = e.mem.u32(manager + offset);
        e.call(0x0048_3710, &args![map]);
    }
    let mut index = 0u32;
    while index < group_count(e, manager) {
        let group = group_at(e, manager, index);
        if group != 0 {
            e.call(0x0098_78b0, &args![group, 0u32]);
            let members = member_count(e, group);
            for member_index in 0..members {
                let member = e.call(MEMBER_AT, &args![group, member_index]).u32();
                if member != 0 {
                    let target = e.vcall(member, 0x428, &args![]).u32();
                    if target != 0 {
                        e.call(0x0045_34f0, &args![target, 0u32]);
                    }
                }
            }
        }
        index += 1;
    }
    clear_debug_geometry(e, manager);

    let player = e.global::<u32>(PLAYER_CHARACTER);
    if e.call(0x008d_6f30, &args![player]).u32() == 0 {
        return;
    }
    let player_process = e.call(0x008d_6f30, &args![player]).u32();
    if e.call(0x0052_4cf0, &args![player_process]).u32() == 0 {
        return;
    }
    let setting = e.call(0x0040_8d60, &args![DEBUG_DRAW_SETTING]).u32();
    if e.mem.u8(setting) != 0 {
        draw_debug_targets(e, this);
    }
    if e.call(NI_POINTER_GET, &args![manager + 0x38]).u32() != 0 {
        finish_debug_node(e, this);
    }
}

/// The first half of `00991770`'s drawing: makes sure the debug node exists
/// and adds the sphere and arrow of every qualifying actor.
fn draw_debug_targets(e: &mut Engine, this: Ptr<CombatManager>) {
    let manager = this.addr();
    let pointer = manager + 0x38;
    if e.call(NI_POINTER_GET, &args![pointer]).u32() == 0 {
        let block = e.call(0x00aa_13e0, &args![0xacu32]).u32();
        let node = if block == 0 {
            0
        } else {
            e.call(0x00a5_ecb0, &args![block, 0u32]).u32()
        };
        e.call(NI_POINTER_SET, &args![pointer, node]);
    }
    let process_lists = e.call(0x0071_7e50, &args![PROCESS_LISTS]).u32();
    let actor_count = e.call(0x005b_e5c0, &args![process_lists, 0u32]).u32();
    let mut k = 0u32;
    while k < actor_count.wrapping_add(1) {
        let actor = if k == actor_count {
            e.global::<u32>(PLAYER_CHARACTER)
        } else {
            let process_lists = e.call(0x0071_7e50, &args![PROCESS_LISTS]).u32();
            e.call(0x0096_8670, &args![process_lists, k]).u32()
        };
        if actor != 0 && e.vcall(actor, 0x100, &args![]).bool() {
            draw_debug_actor(e, manager, actor);
        }
        k = k.wrapping_add(1);
    }
}

/// One actor's sphere and arrow in `00991770`.
fn draw_debug_actor(e: &mut Engine, manager: u32, actor: u32) {
    let process = e.call(0x008d_8520, &args![actor]).u32();
    let target = e.vcall(process, 0xf8, &args![actor]).u32();
    if target == 0 {
        return;
    }
    let pointer = manager + 0x38;
    e.with_stack(16, |e, color_a| {
        e.with_stack(16, |e, color_b| {
            e.with_stack(16, |e, color_c| {
                let quarter = float_global(e, COLOR_QUARTER);
                let half = float_global(e, COLOR_HALF);
                e.call(0x0041_4430, &args![color_a, 0.0f32, quarter, quarter, half]);
                let three_quarters = float_global(e, COLOR_THREE_QUARTERS);
                e.call(
                    0x0041_4430,
                    &args![color_b, 0.0f32, three_quarters, three_quarters, half],
                );
                let green = float_global(e, COLOR_GREEN_FACTOR);
                let blue = float_global(e, COLOR_BLUE_FACTOR);
                e.call(0x0041_4430, &args![color_c, 1.0f32, green, blue, half]);

                let radius_word = e.mem.i32(target);
                let padding = e.global::<f64>(RADIUS_PADDING);
                let radius = (f64::from(radius_word) + padding) as f32;
                let sphere = e
                    .call(0x00c5_6680, &args![radius, 10u32, 10u32, color_b])
                    .u32();
                e.call(0x0044_0460, &args![sphere, target + 4]);
                let node = e.call(NI_POINTER_GET, &args![pointer]).u32();
                e.vcall(node, 0xdc, &args![sphere, 1u32]);

                let location = e.vcall(actor, 0x1f4, &args![]).u32();
                e.with_stack(12, |e, start| {
                    for word in 0..3 {
                        let value = e.mem.u32(location + 4 * word);
                        e.mem.set_u32(start.addr() + 4 * word, value);
                    }
                    let arrow_size = float_global(e, ARROW_SIZE);
                    let arrow = e
                        .call(
                            0x00c5_a0f0,
                            &args![start, target + 4, arrow_size, arrow_size, color_a, color_b],
                        )
                        .u32();
                    let node = e.call(NI_POINTER_GET, &args![pointer]).u32();
                    e.vcall(node, 0xdc, &args![arrow, 1u32]);
                });
            });
        });
    });
}

/// The last block of `00991770`: gives the debug node its property, prepares
/// it, attaches it to the player's process object and updates it.
fn finish_debug_node(e: &mut Engine, this: Ptr<CombatManager>) {
    let pointer = this.addr() + 0x38;
    let block = e.call(0x00aa_13e0, &args![0x1cu32]).u32();
    let property = if block == 0 {
        0
    } else {
        e.call(0x0043_91c0, &args![block]).u32()
    };
    e.call(0x0049_ed90, &args![property, 1u32]);
    let node = e.call(NI_POINTER_GET, &args![pointer]).u32();
    e.call(0x0043_9410, &args![node, property]);
    let node = e.call(NI_POINTER_GET, &args![pointer]).u32();
    e.call(0x004b_6e90, &args![node]);
    let node = e.call(NI_POINTER_GET, &args![pointer]).u32();
    e.call(0x00b5_7e30, &args![node, 0u32, 0u32]);
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let process = e.call(0x008d_6f30, &args![player]).u32();
    let attach_target = e.call(0x0052_4cf0, &args![process]).u32();
    let node = e.call(NI_POINTER_GET, &args![pointer]).u32();
    e.vcall(attach_target, 0xdc, &args![node, 1u32]);
    let node = e.call(NI_POINTER_GET, &args![pointer]).u32();
    e.call(0x00a5_a040, &args![node]);
    e.with_stack(12, |e, update| {
        e.call(0x0043_d410, &args![update, 0.0f32, 0u32, 0u32]);
        let node = e.call(NI_POINTER_GET, &args![pointer]).u32();
        e.call(0x00a5_9c60, &args![node, update]);
    });
}

// Translated from 00991d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first group whose `00726070` value (the word at +4) equals `id`, or
/// null.
pub fn fn_00991d60(e: &mut Engine, this: Ptr<CombatManager>, id: u32) -> Ptr {
    let manager = this.addr();
    let mut index = 0u32;
    while index < group_count(e, manager) {
        let group = group_at(e, manager, index);
        if group != 0 && e.call(GROUP_ID, &args![group]).u32() == id {
            return Ptr::new(group);
        }
        index += 1;
    }
    Ptr::new(0)
}

// Translated from 00991dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls virtual slot +0x430 on every member of every group. The member
/// count is re-read after each call; when it changed the walk steps back one
/// member before continuing.
pub fn fn_00991dc0(e: &mut Engine, this: Ptr<CombatManager>) {
    let manager = this.addr();
    let mut index = 0u32;
    while index < group_count(e, manager) {
        let group = group_at(e, manager, index);
        if group != 0 {
            let mut members = member_count(e, group);
            let mut member_index = 0u32;
            while member_index < members {
                let member = e.call(MEMBER_AT, &args![group, member_index]).u32();
                e.vcall(member, 0x430, &args![]);
                let now = member_count(e, group);
                if now != members {
                    members = now;
                    member_index = member_index.wrapping_sub(1);
                }
                member_index = member_index.wrapping_add(1);
            }
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00990f20, fn_00990f20()),
        entry!(0x00990ff0, fn_00990ff0()),
        entry!(
            0x00991050,
            fn_00991050(Ptr<CombatManager>, u32) -> Ptr<CombatManager>
        ),
        entry!(
            0x00991080,
            fn_00991080(Ptr<CombatManager>) -> Ptr<CombatManager>
        ),
        entry!(0x00991130, fn_00991130(Ptr<CombatManager>)),
        entry!(0x00991150, fn_00991150(Ptr<CombatManager>)),
        entry!(0x009911c0, fn_009911c0(Ptr<CombatManager>)),
        entry!(0x00991310, fn_00991310(Ptr<CombatManager>, u8)),
        entry!(0x009914d0, fn_009914d0(Ptr, u32) -> Ptr),
        entry!(0x00991500, fn_00991500(Ptr<CombatManager>, u8)),
        entry!(0x009915e0, fn_009915e0(f32)),
        entry!(0x00991600, fn_00991600(Ptr<CombatManager>, u8)),
        entry!(0x00991770, fn_00991770(Ptr<CombatManager>)),
        entry!(0x00991d60, fn_00991d60(Ptr<CombatManager>, u32) -> Ptr),
        entry!(0x00991dc0, fn_00991dc0(Ptr<CombatManager>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Vec<Vec<u32>>;

    /// Every function outside this file that the translations call.
    const EXTERNAL: &[u32] = &[
        SCOPE_GUARD_CONSTRUCT,
        SCOPE_GUARD_DESTRUCT,
        0x0097_a990,
        0x0098_4a50,
        0x0099_0070,
        0x009a_6050,
        0x0098_3910,
        0x0097_aa40,
        0x0098_4a90,
        0x0099_00b0,
        0x0098_3940,
        0x0099_3960,
        0x0043_5de0,
        0x0090_6f60,
        0x0063_3c90,
        0x0099_3880,
        0x0045_cec0,
        0x0098_3080,
        0x009a_4fb0,
        0x009a_3e90,
        0x0099_3820,
        0x0098_6b70,
        0x005e_03d0,
        0x0043_8af0,
        0x0098_5f90,
        0x0084_d030,
        0x0098_6150,
        0x009a_50b0,
        0x0096_ae90,
        0x0099_38b0,
        0x0060_0bc0,
        0x0097_ef50,
        0x0048_3710,
        0x0045_34f0,
        0x0098_78b0,
        0x008d_6f30,
        0x0052_4cf0,
        0x0040_8d60,
        0x0071_7e50,
        0x005b_e5c0,
        0x0096_8670,
        0x0041_4430,
        0x00c5_6680,
        0x0044_0460,
        0x008d_8520,
        0x00c5_a0f0,
        0x0049_ed90,
        0x0043_9410,
        0x004b_6e90,
        0x00b5_7e30,
        0x00a5_a040,
        0x0043_d410,
        0x00a5_9c60,
    ];

    /// Code addresses for virtual slots, outside the heap.
    const SLOT_TARGET: u32 = 0x0700_0000;
    const SLOT_TARGET_B: u32 = 0x0700_0010;
    /// Slot +0xDC of the debug node (attach child).
    const NODE_ATTACH: u32 = 0x0700_0100;
    /// Slot +0xDC of the player's process object.
    const PLAYER_ATTACH: u32 = 0x0700_0110;
    const TARGET_GETTER: u32 = 0x0700_0120;
    const LOCATION_GETTER: u32 = 0x0700_0130;
    const QUALIFIES: u32 = 0x0700_0140;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn zero(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn one(_: &mut Engine, _: &[u32]) -> Ret {
        ret(1)
    }

    fn array_count(e: &mut Engine, a: &[u32]) -> Ret {
        ret(u32::from(e.mem.u16(a[0] + 10)))
    }

    fn array_slot(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0] + 4) + 4 * a[1])
    }

    fn pointer_get(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0]))
    }

    fn pointer_set(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[0], a[1]);
        Ret::default()
    }

    fn identity(_: &mut Engine, a: &[u32]) -> Ret {
        ret(a[0])
    }

    fn allocate(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.alloc(a[0]))
    }

    fn release(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.free(a[0]);
        Ret::default()
    }

    /// The word at +4 of a group (its id).
    fn group_id(e: &mut Engine, a: &[u32]) -> Ret {
        ret(e.mem.u32(a[0] + 4))
    }

    /// `00903600(group, index)`: the member table of the group at +0x20.
    fn member_at(e: &mut Engine, a: &[u32]) -> Ret {
        let table = e.mem.u32(a[0] + 0x20);
        ret(e.mem.u32(table + 4 * a[1]))
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &address in EXTERNAL {
            e.register(address, zero);
        }
        e.register(ALLOCATE, allocate);
        e.register(FREE, release);
        e.register(ARRAY_COUNT, array_count);
        e.register(ARRAY_SLOT, array_slot);
        e.register(NI_POINTER_GET, pointer_get);
        e.register(NI_POINTER_SET, pointer_set);
        e.register(NODE_PARENT, zero);
        e.register(MEMBER_COUNT, zero);
        e.register(MEMBER_AT, member_at);
        e.register(GROUP_PREDICATE, zero);
        e.register(GROUP_ID, group_id);
        for address in [
            COMBAT_MANAGER,
            ACCUMULATED_VALUE,
            FLOAT_MAX,
            PLAYER_CHARACTER,
            COLOR_QUARTER,
            COLOR_HALF,
            COLOR_THREE_QUARTERS,
            COLOR_GREEN_FACTOR,
            COLOR_BLUE_FACTOR,
            RADIUS_PADDING,
            ARROW_SIZE,
        ] {
            e.map(address, 8);
        }
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    /// The addresses of the logged calls that are in `wanted`, in order.
    fn order(e: &Engine, wanted: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|call| call.0)
            .filter(|address| wanted.contains(address))
            .collect()
    }

    /// A manager whose group array holds `groups`.
    fn manager(e: &mut Engine, groups: &[u32]) -> Ptr<CombatManager> {
        let manager = e.new_object::<CombatManager>();
        let table = e.mem.alloc(4 * groups.len().max(1) as u32);
        for (index, group) in groups.iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *group);
        }
        e.mem.set_u16(manager.addr() + 10, groups.len() as u16);
        e.mem.set_u32(manager.addr() + 4, table);
        manager
    }

    /// An object whose vtable holds `slots` (byte offset, target).
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x500);
        e.mem.set_u32(object, vtable);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        object
    }

    #[test]
    fn creates_the_manager_once() {
        let mut e = engine();
        e.register(0x0099_1080, identity);
        e.call(0x0099_0f20, &[]);
        let manager = e.global::<u32>(COMBAT_MANAGER);
        assert_ne!(manager, 0);
        assert_eq!(calls_to(&e, 0x0099_1080), vec![vec![manager]]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x3c]]);
        let guard = calls_to(&e, SCOPE_GUARD_CONSTRUCT)[0].clone();
        assert_eq!(guard[1..], [0x2b, 1, SOURCE_FILE, 0x43]);
        assert_eq!(calls_to(&e, SCOPE_GUARD_DESTRUCT), vec![vec![guard[0]]]);
        let steps = [
            SCOPE_GUARD_CONSTRUCT,
            ALLOCATE,
            0x0099_1080,
            0x0097_a990,
            0x0098_4a50,
            0x0099_0070,
            0x009a_6050,
            0x0098_3910,
            SCOPE_GUARD_DESTRUCT,
        ];
        assert_eq!(order(&e, &steps), steps);
        // With a manager present nothing but the call itself happens.
        e.call_log = Some(vec![]);
        e.call(0x0099_0f20, &[]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.global::<u32>(COMBAT_MANAGER), manager);
    }

    #[test]
    fn a_failed_allocation_stores_null_and_still_initializes() {
        let mut e = engine();
        e.register(ALLOCATE, zero);
        e.call(0x0099_0f20, &[]);
        assert_eq!(e.global::<u32>(COMBAT_MANAGER), 0);
        assert!(calls_to(&e, 0x0099_1080).is_empty());
        assert_eq!(calls_to(&e, 0x0098_3910).len(), 1);
    }

    #[test]
    fn destroys_the_manager_and_clears_the_pointer() {
        let mut e = engine();
        let m = manager(&mut e, &[]);
        e.set_global(COMBAT_MANAGER, m.addr());
        e.call(0x0099_0ff0, &[]);
        assert_eq!(e.global::<u32>(COMBAT_MANAGER), 0);
        let steps = [
            0x0097_aa40,
            0x0098_4a90,
            0x0099_00b0,
            0x0098_3940,
            0x005e_03d0,
            0x0099_3880,
            FREE,
        ];
        assert_eq!(order(&e, &steps), steps);
        assert_eq!(calls_to(&e, FREE), vec![vec![m.addr()]]);
        assert_eq!(e.mem.block_size(m.addr()), None);
    }

    #[test]
    fn without_a_manager_destroying_only_runs_the_teardowns() {
        let mut e = engine();
        e.call(0x0099_0ff0, &[]);
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(calls_to(&e, 0x0098_3940).len(), 1);
        assert_eq!(e.global::<u32>(COMBAT_MANAGER), 0);
    }

    #[test]
    fn the_scalar_deleting_destructor_frees_on_flag_one_only() {
        let mut e = engine();
        let m = manager(&mut e, &[]);
        let result = e.call(0x0099_1050, &args![m, 0u32]).u32();
        assert_eq!(result, m.addr());
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(calls_to(&e, 0x0099_3880), vec![vec![m.addr()]]);
        let result = e.call(0x0099_1050, &args![m, 3u32]).u32();
        assert_eq!(result, m.addr());
        assert_eq!(calls_to(&e, FREE), vec![vec![m.addr()]]);
    }

    #[test]
    fn the_constructor_fills_the_fields_and_creates_the_sub_objects() {
        let mut e = engine();
        for constructor in [0x0098_3080, 0x009a_4fb0, 0x009a_3e90, 0x0099_3820] {
            e.register(constructor, identity);
        }
        e.set_global(FLOAT_MAX, f32::MAX);
        let m = e.new_object::<CombatManager>();
        let result = e.call(0x0099_1080, &args![m]).u32();
        assert_eq!(result, m.addr());
        assert_eq!(calls_to(&e, 0x0099_3960), vec![vec![m.addr(), 0, 1]]);
        assert_eq!(
            calls_to(&e, 0x0043_5de0),
            vec![vec![m.addr() + 0x20, (-f32::MAX).to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x0090_6f60),
            vec![vec![m.addr() + 0x24], vec![m.addr() + 0x2c]]
        );
        assert_eq!(e.get(m, CombatManager::iNextGroupID), 1);
        assert_eq!(calls_to(&e, 0x0063_3c90), vec![vec![m.addr() + 0x38, 0]]);
        for field in [
            CombatManager::pCoverMap,
            CombatManager::pThreatMap,
            CombatManager::pTargetMap,
            CombatManager::pLocalActorTargetingMap,
        ] {
            assert_ne!(e.get(m, field).addr(), 0);
        }
    }

    #[test]
    fn the_music_time_stamp_quiets_a_signalling_nan() {
        let mut e = engine();
        e.set_global(FLOAT_MAX, f32::from_bits(0x7fa0_0000));
        let m = e.new_object::<CombatManager>();
        e.call(0x0099_1080, &args![m]);
        assert_eq!(calls_to(&e, 0x0043_5de0)[0][1], 0xffe0_0000);
    }

    #[test]
    fn the_base_destructor_forwards() {
        let mut e = engine();
        e.call(0x0099_1130, &args![0x1234u32]);
        assert_eq!(calls_to(&e, 0x0099_3880), vec![vec![0x1234]]);
    }

    #[test]
    fn the_destructor_empties_then_releases_the_pointer_then_the_base() {
        let mut e = engine();
        let m = manager(&mut e, &[]);
        e.call(0x0099_1150, &args![m]);
        let steps = [0x005e_03d0, 0x0045_cec0, 0x0099_3880];
        assert_eq!(order(&e, &steps), steps);
        assert_eq!(calls_to(&e, 0x0045_cec0), vec![vec![m.addr() + 0x38]]);
        // The per-group call is off in the destructor.
        assert!(calls_to(&e, 0x0098_6b70).is_empty());
    }

    #[test]
    fn creating_the_sub_objects_keeps_zero_for_a_failed_allocation() {
        let mut e = engine();
        for constructor in [0x0098_3080, 0x009a_4fb0, 0x009a_3e90, 0x0099_3820] {
            e.register(constructor, identity);
        }
        e.register_double(ALLOCATE, |e, a| {
            if a[0] == 0x18 {
                Ret::default()
            } else {
                ret(e.mem.alloc(a[0]))
            }
        });
        let m = e.new_object::<CombatManager>();
        e.call(0x0099_11c0, &args![m]);
        assert_ne!(e.get(m, CombatManager::pCoverMap).addr(), 0);
        assert_eq!(e.get(m, CombatManager::pThreatMap).addr(), 0);
        assert_ne!(e.get(m, CombatManager::pTargetMap).addr(), 0);
        assert!(calls_to(&e, 0x009a_4fb0).is_empty());
        let targeting = e.get(m, CombatManager::pLocalActorTargetingMap).addr();
        assert_ne!(targeting, 0);
        assert_eq!(calls_to(&e, 0x0099_3820), vec![vec![targeting, 0x25]]);
        let steps = [0x0098_3080, 0x009a_3e90, 0x0099_3820];
        assert_eq!(order(&e, &steps), steps);
    }

    /// A manager holding `groups` and four sub-objects whose deleting
    /// destructor (slot 0) is `SLOT_TARGET`.
    fn teardown_setup(e: &mut Engine, groups: &[u32]) -> Ptr<CombatManager> {
        e.register(SLOT_TARGET, zero);
        let m = manager(e, groups);
        for field in [
            CombatManager::pCoverMap,
            CombatManager::pThreatMap,
            CombatManager::pTargetMap,
            CombatManager::pLocalActorTargetingMap,
        ] {
            let object = object_with_slots(e, &[(0, SLOT_TARGET)]);
            e.set(m, field, Ptr::new(object));
        }
        m
    }

    #[test]
    fn tearing_down_deletes_groups_and_sub_objects_in_order() {
        let mut e = engine();
        let group_a = e.mem.alloc(0x20);
        let group_b = e.mem.alloc(0x20);
        let m = teardown_setup(&mut e, &[group_a, 0, group_b]);
        let cover = e.get(m, CombatManager::pCoverMap).addr();
        let threat = e.get(m, CombatManager::pThreatMap).addr();
        let target = e.get(m, CombatManager::pTargetMap).addr();
        let targeting = e.get(m, CombatManager::pLocalActorTargetingMap).addr();
        e.call(0x0099_1310, &args![m, 1u8]);
        assert_eq!(
            calls_to(&e, 0x0098_6b70),
            vec![vec![group_a], vec![group_b]]
        );
        assert_eq!(
            calls_to(&e, 0x0098_5f90),
            vec![vec![group_a], vec![group_b]]
        );
        assert_eq!(calls_to(&e, FREE), vec![vec![group_a], vec![group_b]]);
        assert_eq!(
            order(&e, &[0x0098_6b70, 0x0098_5f90, 0x005e_03d0]),
            vec![
                0x0098_6b70,
                0x0098_5f90,
                0x0098_6b70,
                0x0098_5f90,
                0x005e_03d0
            ]
        );
        // Deleting destructor calls, flag 1, in the order +0x18, +0x14,
        // +0x1C and +0x10, with 00438af0 on the +0x10 object just before
        // its own.
        assert_eq!(
            calls_to(&e, SLOT_TARGET),
            vec![
                vec![cover, 1],
                vec![threat, 1],
                vec![target, 1],
                vec![targeting, 1]
            ]
        );
        assert_eq!(calls_to(&e, 0x0043_8af0), vec![vec![targeting]]);
        let log = e.call_log.as_ref().unwrap();
        let notify = log.iter().position(|c| c.0 == 0x0043_8af0).unwrap();
        let last = log.iter().rposition(|c| c.0 == SLOT_TARGET).unwrap();
        let third = log
            .iter()
            .position(|c| c.0 == SLOT_TARGET && c.1[0] == target)
            .unwrap();
        assert!(third < notify && notify < last);
        assert_eq!(calls_to(&e, NI_POINTER_SET), vec![vec![m.addr() + 0x38, 0]]);
    }

    #[test]
    fn tearing_down_without_the_per_group_call_skips_null_sub_objects() {
        let mut e = engine();
        let group = e.mem.alloc(0x20);
        let m = teardown_setup(&mut e, &[group]);
        e.set(m, CombatManager::pThreatMap, Ptr::new(0));
        e.set(m, CombatManager::pLocalActorTargetingMap, Ptr::new(0));
        e.call(0x0099_1310, &args![m, 0u8]);
        assert!(calls_to(&e, 0x0098_6b70).is_empty());
        assert_eq!(calls_to(&e, 0x0098_5f90), vec![vec![group]]);
        // Only the cover map and the target map are deleted, and 00438af0
        // still runs on the null targeting map.
        assert_eq!(calls_to(&e, SLOT_TARGET).len(), 2);
        assert_eq!(calls_to(&e, 0x0043_8af0), vec![vec![0]]);
    }

    #[test]
    fn tearing_down_detaches_a_debug_node_from_its_parent() {
        let mut e = engine();
        let m = teardown_setup(&mut e, &[]);
        e.register(NODE_PARENT, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(SLOT_TARGET_B, zero);
        let parent = object_with_slots(&mut e, &[(0xe8, SLOT_TARGET_B)]);
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node + 0x18, parent);
        e.mem.set_u32(m.addr() + 0x38, node);
        e.call(0x0099_1310, &args![m, 0u8]);
        assert_eq!(calls_to(&e, SLOT_TARGET_B), vec![vec![parent, node]]);
        assert_eq!(e.mem.u32(m.addr() + 0x38), 0);
    }

    #[test]
    fn a_node_without_a_parent_is_only_released() {
        let mut e = engine();
        let m = teardown_setup(&mut e, &[]);
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(m.addr() + 0x38, node);
        e.call(0x0099_1310, &args![m, 0u8]);
        assert_eq!(calls_to(&e, NODE_PARENT).len(), 1);
        assert_eq!(e.mem.u32(m.addr() + 0x38), 0);
    }

    #[test]
    fn the_group_scalar_deleting_destructor_frees_on_flag_one_only() {
        let mut e = engine();
        let group = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0099_14d0, &args![group, 2u32]).u32(), group);
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(calls_to(&e, 0x0098_5f90), vec![vec![group]]);
        assert_eq!(e.call(0x0099_14d0, &args![group, 1u32]).u32(), group);
        assert_eq!(calls_to(&e, FREE), vec![vec![group]]);
    }

    #[test]
    fn the_update_does_nothing_but_the_guard_when_skipped() {
        let mut e = engine();
        let m = manager(&mut e, &[0x100]);
        e.call(0x0099_1500, &args![m, 1u8]);
        let log = e.call_log.as_ref().unwrap();
        let addresses: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            addresses,
            vec![0x0099_1500, SCOPE_GUARD_CONSTRUCT, SCOPE_GUARD_DESTRUCT]
        );
        assert_eq!(
            calls_to(&e, SCOPE_GUARD_CONSTRUCT)[0][1..],
            [0x2b, 1, SOURCE_FILE, 0x8b]
        );
    }

    #[test]
    fn the_update_adds_the_step_and_updates_the_groups_the_predicate_rejects() {
        let mut e = engine();
        let group_a = e.mem.alloc(0x20);
        let group_b = e.mem.alloc(0x20);
        let m = manager(&mut e, &[group_a, 0, group_b]);
        let threat_map = e.mem.alloc(0x20);
        e.set(m, CombatManager::pThreatMap, Ptr::new(threat_map));
        e.set_global(ACCUMULATED_VALUE, 1.0f32);
        e.register(0x0084_d030, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register_double(GROUP_PREDICATE, move |_, a| ret(u32::from(a[0] == group_a)));
        e.call(0x0099_1500, &args![m, 0u8]);
        assert_eq!(calls_to(&e, 0x0084_d030), vec![vec![STEP_SOURCE]]);
        assert_eq!(e.global::<f32>(ACCUMULATED_VALUE), 1.5);
        assert_eq!(calls_to(&e, 0x0098_6150), vec![vec![group_b]]);
        assert_eq!(calls_to(&e, GROUP_PREDICATE).len(), 2);
        assert_eq!(calls_to(&e, 0x009a_50b0), vec![vec![threat_map]]);
        let steps = [
            SCOPE_GUARD_CONSTRUCT,
            0x0098_6150,
            0x009a_50b0,
            SCOPE_GUARD_DESTRUCT,
        ];
        assert_eq!(order(&e, &steps), steps);
    }

    #[test]
    fn the_accumulator_adds_in_double_precision_and_rounds_to_float() {
        let mut e = engine();
        e.set_global(ACCUMULATED_VALUE, 16_777_216.0f32);
        e.call(0x0099_15e0, &args![1.0f32]);
        assert_eq!(e.global::<f32>(ACCUMULATED_VALUE), 16_777_216.0);
        e.set_global(ACCUMULATED_VALUE, 0.25f32);
        e.call(0x0099_15e0, &args![-1.5f32]);
        assert_eq!(e.global::<f32>(ACCUMULATED_VALUE), -1.25);
    }

    /// A group with an id at +4 and a member table at +0x20 (read by
    /// `member_at`).
    fn group_with_members(e: &mut Engine, id: u32, members: &[u32]) -> u32 {
        let group = e.mem.alloc(0x40);
        let table = e.mem.alloc(4 * members.len().max(1) as u32);
        for (index, member) in members.iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *member);
        }
        e.mem.set_u32(group + 4, id);
        e.mem.set_u32(group + 0x20, table);
        group
    }

    /// A member object whose slots +0x428 and +0x430 are `SLOT_TARGET` and
    /// `SLOT_TARGET_B`.
    fn member(e: &mut Engine) -> u32 {
        object_with_slots(e, &[(0x428, SLOT_TARGET), (0x430, SLOT_TARGET_B)])
    }

    #[test]
    fn removal_deletes_the_predicate_groups_and_nulls_their_slots() {
        let mut e = engine();
        let doomed = e.mem.alloc(0x20);
        let kept = e.mem.alloc(0x20);
        let m = manager(&mut e, &[doomed, 0, kept]);
        e.register_double(GROUP_PREDICATE, move |_, a| ret(u32::from(a[0] == doomed)));
        e.register(0x0096_ae90, |e, a| {
            // `(this, index, &value)`: stores the value into the slot.
            let table = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(table + 4 * a[1], value);
            Ret::default()
        });
        e.call(0x0099_1600, &args![m, 1u8]);
        assert_eq!(calls_to(&e, 0x0098_5f90)[0], vec![doomed]);
        assert_eq!(calls_to(&e, FREE), vec![vec![doomed]]);
        assert_eq!(calls_to(&e, 0x0096_ae90).len(), 1);
        assert_eq!(calls_to(&e, 0x0096_ae90)[0][..2], [m.addr(), 0]);
        let table = e.mem.u32(m.addr() + 4);
        assert_eq!(e.mem.u32(table), 0);
        assert_eq!(e.mem.u32(table + 8), kept);
        // 009938b0 answers 0: compact and notify the targeting map.
        assert_eq!(calls_to(&e, 0x0060_0bc0), vec![vec![m.addr()]]);
        let targeting = e.get(m, CombatManager::pLocalActorTargetingMap).addr();
        assert_eq!(calls_to(&e, 0x0043_8af0), vec![vec![targeting]]);
        // The member pass is skipped and the debug pass (00991770) runs.
        assert_eq!(calls_to(&e, 0x0048_3710).len(), 3);
    }

    #[test]
    fn removal_skips_the_compaction_when_the_word_at_0xc_is_nonzero() {
        let mut e = engine();
        let kept = e.mem.alloc(0x20);
        let m = manager(&mut e, &[kept]);
        e.register(0x0099_38b0, |_, _| ret(3));
        e.call(0x0099_1600, &args![m, 1u8]);
        assert!(calls_to(&e, 0x0060_0bc0).is_empty());
        assert!(calls_to(&e, 0x0096_ae90).is_empty());
        assert!(calls_to(&e, 0x0043_8af0).is_empty());
    }

    #[test]
    fn removal_without_groups_goes_straight_to_the_debug_pass() {
        let mut e = engine();
        let m = manager(&mut e, &[]);
        e.call(0x0099_1600, &args![m, 0u8]);
        assert!(calls_to(&e, 0x0099_38b0).is_empty());
        assert!(calls_to(&e, 0x0060_0bc0).is_empty());
        assert_eq!(calls_to(&e, 0x0048_3710).len(), 3);
    }

    #[test]
    fn the_member_pass_calls_slot_0x428_and_follows_up_on_results() {
        let mut e = engine();
        // The first member answers a target, the second none.
        e.register(SLOT_TARGET, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(MEMBER_COUNT, |_, _| ret(2));
        let first = member(&mut e);
        let second = member(&mut e);
        e.mem.set_u32(first + 8, 0x7777);
        let group = group_with_members(&mut e, 5, &[first, second]);
        let m = manager(&mut e, &[group]);
        e.call(0x0099_1600, &args![m, 0u8]);
        assert_eq!(calls_to(&e, 0x0097_ef50), vec![vec![0x7777]]);
        // Two members in the member pass, two more in the debug pass.
        assert_eq!(calls_to(&e, SLOT_TARGET).len(), 4);
        assert_eq!(calls_to(&e, 0x0045_34f0), vec![vec![0x7777, 0]]);
        assert_eq!(calls_to(&e, 0x0098_78b0), vec![vec![group, 0]]);
    }

    #[test]
    fn the_debug_pass_stops_without_a_player_process() {
        let mut e = engine();
        let m = manager(&mut e, &[]);
        e.call(0x0099_1770, &args![m]);
        assert_eq!(
            calls_to(&e, 0x0048_3710),
            vec![
                vec![e.get(m, CombatManager::pThreatMap).addr()],
                vec![e.get(m, CombatManager::pCoverMap).addr()],
                vec![e.get(m, CombatManager::pTargetMap).addr()],
            ]
        );
        assert_eq!(calls_to(&e, 0x008d_6f30).len(), 1);
        assert!(calls_to(&e, 0x0040_8d60).is_empty());
        assert_eq!(calls_to(&e, NI_POINTER_SET), vec![vec![m.addr() + 0x38, 0]]);
    }

    #[test]
    fn the_debug_pass_stops_when_the_process_has_no_object() {
        let mut e = engine();
        let m = manager(&mut e, &[]);
        e.set_global(PLAYER_CHARACTER, 0x5555u32);
        e.register(0x008d_6f30, |_, _| ret(0x6666));
        e.call(0x0099_1770, &args![m]);
        assert_eq!(calls_to(&e, 0x008d_6f30).len(), 2);
        assert_eq!(calls_to(&e, 0x0052_4cf0), vec![vec![0x6666]]);
        assert!(calls_to(&e, 0x0040_8d60).is_empty());
    }

    /// The drawing scene: a player with a process and one actor.
    struct Scene {
        manager: Ptr<CombatManager>,
        node: u32,
        process: u32,
        target: u32,
    }

    /// `player_draws`: the loop covers only the player, who qualifies;
    /// otherwise it covers one qualifying actor and the player, who does
    /// not.
    fn scene(e: &mut Engine, setting: u8, player_draws: bool) -> Scene {
        e.register(NODE_ATTACH, zero);
        e.register(PLAYER_ATTACH, zero);
        e.register(SLOT_TARGET_B, zero);
        e.register(QUALIFIES, one);
        e.set_global(COLOR_QUARTER, 0.25f32);
        e.set_global(COLOR_HALF, 0.5f32);
        e.set_global(COLOR_THREE_QUARTERS, 0.75f32);
        e.set_global(COLOR_GREEN_FACTOR, 0.625f32);
        e.set_global(COLOR_BLUE_FACTOR, 0.125f32);
        e.set_global(RADIUS_PADDING, 32.0f64);
        e.set_global(ARROW_SIZE, 2.0f32);
        let manager = manager(e, &[]);
        let node = object_with_slots(e, &[(0xdc, NODE_ATTACH)]);
        e.register_double(0x00a5_ecb0, move |_, _| ret(node));
        let target = e.mem.alloc(0x10);
        e.mem.set_i32(target, 100);
        e.register_double(TARGET_GETTER, move |_, _| ret(target));
        let location = e.mem.alloc(0x10);
        e.mem.set_u32(location, 0x1111);
        e.mem.set_u32(location + 4, 0x2222);
        e.mem.set_u32(location + 8, 0x3333);
        e.register_double(LOCATION_GETTER, move |_, _| ret(location));
        let drawn_slots = [(0x100, QUALIFIES), (0x1f4, LOCATION_GETTER)];
        let player = if player_draws {
            object_with_slots(e, &drawn_slots)
        } else {
            object_with_slots(e, &[(0x100, SLOT_TARGET_B)])
        };
        e.set_global(PLAYER_CHARACTER, player);
        let process = object_with_slots(e, &[(0xdc, PLAYER_ATTACH)]);
        e.register_double(0x008d_6f30, move |_, a| {
            ret(if a[0] == player { process } else { 0 })
        });
        e.register_double(0x0052_4cf0, move |_, _| ret(process));
        let setting_byte = e.mem.alloc(1);
        e.mem.set_u8(setting_byte, setting);
        e.register_double(0x0040_8d60, move |_, _| ret(setting_byte));
        e.register(0x0071_7e50, identity);
        e.register_double(0x005b_e5c0, move |_, _| ret(u32::from(!player_draws)));
        let actor = object_with_slots(e, &drawn_slots);
        e.register_double(0x0096_8670, move |_, _| ret(actor));
        let actor_process = object_with_slots(e, &[(0xf8, TARGET_GETTER)]);
        e.register_double(0x008d_8520, move |_, _| ret(actor_process));
        e.register(0x00c5_6680, |_, _| ret(0xaaaa));
        e.register(0x00c5_a0f0, |_, _| ret(0xbbbb));
        e.register(0x0043_91c0, identity);
        e.register(0x00aa_13e0, allocate);
        Scene {
            manager,
            node,
            process,
            target,
        }
    }

    #[test]
    fn the_debug_pass_draws_a_sphere_and_an_arrow_for_each_qualifying_actor() {
        let mut e = engine();
        let s = scene(&mut e, 1, false);
        e.call(0x0099_1770, &args![s.manager]);
        // The node was missing: an 0xAC-byte block constructed with 0.
        assert_eq!(calls_to(&e, 0x00aa_13e0)[0], vec![0xac]);
        assert_eq!(calls_to(&e, 0x00a5_ecb0)[0][1], 0);
        assert_eq!(
            calls_to(&e, NI_POINTER_SET)[1],
            vec![s.manager.addr() + 0x38, s.node]
        );
        // The loop covers the process lists' actor and the player.
        assert_eq!(calls_to(&e, 0x005b_e5c0), vec![vec![PROCESS_LISTS, 0]]);
        assert_eq!(calls_to(&e, 0x0096_8670), vec![vec![PROCESS_LISTS, 0]]);
        assert_eq!(calls_to(&e, 0x008d_8520).len(), 1);
        // Three colors; the sphere gets radius + 32.0 and the second color.
        let colors = calls_to(&e, 0x0041_4430);
        assert_eq!(colors.len(), 3);
        assert_eq!(
            colors[0][1..],
            [0, 0.25f32.to_bits(), 0.25f32.to_bits(), 0.5f32.to_bits()]
        );
        assert_eq!(
            colors[1][1..],
            [0, 0.75f32.to_bits(), 0.75f32.to_bits(), 0.5f32.to_bits()]
        );
        assert_eq!(
            colors[2][1..],
            [
                1.0f32.to_bits(),
                0.625f32.to_bits(),
                0.125f32.to_bits(),
                0.5f32.to_bits()
            ]
        );
        assert_eq!(
            calls_to(&e, 0x00c5_6680),
            vec![vec![132.0f32.to_bits(), 10, 10, colors[1][0]]]
        );
        assert_eq!(calls_to(&e, 0x0044_0460), vec![vec![0xaaaa, s.target + 4]]);
        let arrows = calls_to(&e, 0x00c5_a0f0);
        assert_eq!(arrows.len(), 1);
        assert_eq!(
            arrows[0][1..],
            [
                s.target + 4,
                2.0f32.to_bits(),
                2.0f32.to_bits(),
                colors[0][0],
                colors[1][0]
            ]
        );
        // Both are attached to the node with first-available 1.
        assert_eq!(
            calls_to(&e, NODE_ATTACH),
            vec![vec![s.node, 0xaaaa, 1], vec![s.node, 0xbbbb, 1]]
        );
    }

    #[test]
    fn the_arrow_starts_at_a_copy_of_the_actors_location() {
        let mut e = engine();
        let s = scene(&mut e, 1, false);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(0x00c5_a0f0, move |e, a| {
            sink.borrow_mut()
                .push([e.mem.u32(a[0]), e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8)]);
            ret(0xbbbb)
        });
        e.call(0x0099_1770, &args![s.manager]);
        assert_eq!(*seen.borrow(), vec![[0x1111, 0x2222, 0x3333]]);
    }

    #[test]
    fn an_actor_without_a_target_draws_nothing() {
        let mut e = engine();
        let s = scene(&mut e, 1, false);
        e.register(TARGET_GETTER, zero);
        e.call(0x0099_1770, &args![s.manager]);
        assert!(calls_to(&e, 0x0041_4430).is_empty());
        assert!(calls_to(&e, 0x00c5_6680).is_empty());
        assert!(calls_to(&e, 0x00c5_a0f0).is_empty());
        assert_eq!(calls_to(&e, 0x008d_8520).len(), 1);
    }

    #[test]
    fn the_player_is_drawn_when_the_count_is_reached() {
        let mut e = engine();
        let s = scene(&mut e, 1, true);
        e.call(0x0099_1770, &args![s.manager]);
        // No process-lists lookup: the loop ran once, with the player.
        assert!(calls_to(&e, 0x0096_8670).is_empty());
        assert_eq!(calls_to(&e, 0x00c5_6680).len(), 1);
        assert_eq!(calls_to(&e, 0x00c5_a0f0).len(), 1);
    }

    #[test]
    fn an_actor_that_is_not_an_actor_is_skipped() {
        let mut e = engine();
        let s = scene(&mut e, 1, false);
        e.register(QUALIFIES, zero);
        e.call(0x0099_1770, &args![s.manager]);
        assert!(calls_to(&e, 0x008d_8520).is_empty());
        assert!(calls_to(&e, 0x00c5_6680).is_empty());
    }

    #[test]
    fn the_node_is_finished_even_when_the_setting_is_off() {
        let mut e = engine();
        let s = scene(&mut e, 0, false);
        // Keep the node that the clearing would drop.
        e.set(s.manager, CombatManager::spDebugGeometry, Ptr::new(s.node));
        e.register(NI_POINTER_SET, zero);
        e.call(0x0099_1770, &args![s.manager]);
        assert!(calls_to(&e, 0x005b_e5c0).is_empty());
        assert_eq!(calls_to(&e, 0x00aa_13e0), vec![vec![0x1c]]);
        let property = calls_to(&e, 0x0043_91c0)[0][0];
        assert_eq!(calls_to(&e, 0x0049_ed90), vec![vec![property, 1]]);
        assert_eq!(calls_to(&e, 0x0043_9410), vec![vec![s.node, property]]);
        assert_eq!(calls_to(&e, 0x004b_6e90), vec![vec![s.node]]);
        assert_eq!(calls_to(&e, 0x00b5_7e30), vec![vec![s.node, 0, 0]]);
        assert_eq!(
            calls_to(&e, PLAYER_ATTACH),
            vec![vec![s.process, s.node, 1]]
        );
        assert_eq!(calls_to(&e, 0x00a5_a040), vec![vec![s.node]]);
        let update = calls_to(&e, 0x0043_d410);
        assert_eq!(update[0][1..], [0, 0, 0]);
        assert_eq!(calls_to(&e, 0x00a5_9c60), vec![vec![s.node, update[0][0]]]);
        let steps = [
            0x0043_91c0,
            0x0049_ed90,
            0x0043_9410,
            0x004b_6e90,
            0x00b5_7e30,
        ];
        assert_eq!(order(&e, &steps), steps);
    }

    #[test]
    fn a_failed_node_allocation_assigns_null_and_ends_the_pass() {
        let mut e = engine();
        let s = scene(&mut e, 1, false);
        // Nobody qualifies, so the loop draws nothing into the missing node.
        e.register(QUALIFIES, zero);
        e.register(0x00aa_13e0, zero);
        e.call(0x0099_1770, &args![s.manager]);
        assert!(calls_to(&e, 0x00a5_ecb0).is_empty());
        assert_eq!(
            calls_to(&e, NI_POINTER_SET)[1],
            vec![s.manager.addr() + 0x38, 0]
        );
        // The node stays null: nothing is drawn and nothing is finished.
        assert!(calls_to(&e, 0x0043_91c0).is_empty());
        assert!(calls_to(&e, 0x00c5_6680).is_empty());
    }

    #[test]
    fn finds_the_group_with_the_given_id() {
        let mut e = engine();
        let a = group_with_members(&mut e, 10, &[]);
        let b = group_with_members(&mut e, 20, &[]);
        let m = manager(&mut e, &[0, a, b]);
        assert_eq!(e.call(0x0099_1d60, &args![m, 20u32]).u32(), b);
        assert_eq!(e.call(0x0099_1d60, &args![m, 10u32]).u32(), a);
        assert_eq!(e.call(0x0099_1d60, &args![m, 99u32]).u32(), 0);
    }

    #[test]
    fn the_member_walk_steps_back_when_the_count_changes() {
        let mut e = engine();
        let objects: Vec<u32> = (0..3).map(|_| member(&mut e)).collect();
        e.register(SLOT_TARGET_B, zero);
        let group = group_with_members(&mut e, 1, &objects);
        let m = manager(&mut e, &[group]);
        // Counts seen: 3 (start), 3 after member 0, 2 after member 1, 2 after
        // the repeated member 1.
        let counts = Rc::new(RefCell::new(vec![3u32, 3, 2, 2].into_iter()));
        let source = Rc::clone(&counts);
        e.register_double(MEMBER_COUNT, move |_, _| {
            ret(source.borrow_mut().next().unwrap())
        });
        e.call(0x0099_1dc0, &args![m]);
        // Member 1 is visited twice, member 2 never (the count dropped to 2).
        let visited: Vec<u32> = calls_to(&e, SLOT_TARGET_B).iter().map(|c| c[0]).collect();
        assert_eq!(visited, vec![objects[0], objects[1], objects[1]]);
    }
}
