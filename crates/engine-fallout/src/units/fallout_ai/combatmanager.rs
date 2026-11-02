//! `fallout/ai/combat/combatmanager.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00993bf0` calls on the member (`00993cd0`).
const CALLEE_00993CD0: u32 = 0x0099_3cd0;

// Translated from 00993bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiTMap<Actor*,__int64>::NewItem` (Xbox PDB name in the engine map). Thin wrapper: calls `00993cd0` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_00993bf0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_00993CD0, &args![this.byte_add(0xc)]).ptr()
}

use crate::types::NiTArray;

/// The vtables of this unit's container templates (RTTI names):
/// `NiTArray<CombatGroup*,NiTMallocInterface<..>>` (`0108db4c`),
/// `NiTPrimitiveArray<CombatGroup*>` (`0108db54`), `NiTMap<Actor*,__int64>`
/// (`0108db2c`) and its `NiTMapBase<DFALL<..>,Actor*,__int64>` (`0108db5c`).
const COMBAT_GROUP_ARRAY_VTABLE: u32 = 0x0108_db4c;
const COMBAT_GROUP_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0108_db54;
const ACTOR_MAP_VTABLE: u32 = 0x0108_db2c;
const ACTOR_MAP_BASE_VTABLE: u32 = 0x0108_db5c;
/// `0096afc0` (`cdecl`, one word: a count): allocates the array's elements.
const ARRAY_ALLOCATE: u32 = 0x0096_afc0;
/// `004ede70` (`cdecl`, one word): frees an array's elements.
const ARRAY_FREE: u32 = 0x004e_de70;
/// `00aa1070` (`cdecl`, one word: bytes) and `00aa10f0` (`cdecl`, one word):
/// the map's bucket block allocation and release.
const BUCKETS_ALLOCATE: u32 = 0x00aa_1070;
const BUCKETS_FREE: u32 = 0x00aa_10f0;
/// `00403d30` (`cdecl`: destination, value, count): forwards to `memset`.
const MEMSET: u32 = 0x0040_3d30;
/// `00438af0` (`thiscall`): run on the map by both map destructors.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `00401030` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x0040_1030;

/// The global holding the player character (read at each use).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The global holding the object `0042ce10` tests (`thiscall`, a byte).
const OBJECT_POINTER_011DDF38: u32 = 0x011d_df38;
const OBJECT_TEST: u32 = 0x0042_ce10;
/// `00658930` (`thiscall` on the manager): the number of combat groups;
/// `00877a30` (`thiscall`, one word: an index): the address of that group's
/// pointer.
const GROUP_COUNT: u32 = 0x0065_8930;
const GROUP_SLOT: u32 = 0x0087_7a30;
/// `CombatGroup::IsTarget` (Xbox PDB name of `009865b0`; `thiscall`, one
/// word: the actor).
const GROUP_IS_TARGET: u32 = 0x0098_65b0;
/// `00990890` (`thiscall` on a group): the number of its members;
/// `00903600` (`thiscall`, one word: an index): that member.
const GROUP_MEMBER_COUNT: u32 = 0x0099_0890;
const GROUP_MEMBER: u32 = 0x0090_3600;
/// `00989090` (`thiscall` on a group: out, actor): fills a 4-byte time
/// stamp; `00435e00` (`thiscall` on it): the time since, in `ST0`.
const GROUP_TIME_STAMP: u32 = 0x0098_9090;
const TIME_SINCE: u32 = 0x0043_5e00;
/// `00403e20` (`thiscall` on a setting): the address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings `00992d90` and `009931c0` read (addresses of the setting
/// objects).
const SETTING_011CE528: u32 = 0x011c_e528;
const SETTING_011CEEC0: u32 = 0x011c_eec0;
const SETTING_011CE04C: u32 = 0x011c_e04c;
const SETTING_011CE454: u32 = 0x011c_e454;
const SETTING_011CFE5C: u32 = 0x011c_fe5c;
const SETTING_011CE72C: u32 = 0x011c_e72c;
const SETTING_011CFC14: u32 = 0x011c_fc14;

/// The value word of the setting at `setting` (`00403e20`).
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

/// The group pointer at index `i` of the manager's groups.
fn group_at(e: &mut Engine, this: Ptr, i: u32) -> u32 {
    let slot = e.call(GROUP_SLOT, &args![this, i]).u32();
    e.mem.u32(slot)
}

/// `00989090` into a stack time stamp, then `00435e00` on it: the time since
/// the group's stamp for `actor`, as the `double` the exe stores it as.
fn group_time_since(e: &mut Engine, group: u32, actor: u32) -> f64 {
    e.with_stack(4, |e, stamp| {
        e.call(GROUP_TIME_STAMP, &args![group, stamp, actor]);
        e.call(TIME_SINCE, &args![stamp]).f64()
    })
}

// Translated from 00993cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `NiTArray<CombatGroup*>` constructor (unnamed in the engine map;
/// `thiscall`, `RET 8`): the vtable `0108db4c`, `m_usMaxSize` = `max_size`,
/// `m_usGrowBy` = `grow_by`, both other counts 0, and `m_pBase` from
/// `0096afc0(max_size)` when `max_size` is not zero, else null. Returns
/// `this`.
pub fn fn_00993cf0(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), COMBAT_GROUP_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    let max_size = e.get(this, NiTArray::m_usMaxSize);
    let base = if max_size > 0 {
        e.call(ARRAY_ALLOCATE, &args![u32::from(max_size)]).u32()
    } else {
        0
    };
    e.set(this, NiTArray::m_pBase, base);
    this
}

// Translated from 00993960 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `NiTPrimitiveArray<CombatGroup*>` constructor (unnamed in the engine
/// map; `thiscall`, `RET 8`): the `NiTArray` constructor (`00993cf0`), then
/// the vtable `0108db54`. Returns `this`.
pub fn fn_00993960(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    fn_00993cf0(e, this, max_size, grow_by);
    e.mem
        .set_u32(this.addr(), COMBAT_GROUP_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 009938b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the array's
/// `m_usESize` (`+0xc`), the number of non-null elements.
pub fn fn_009938b0(e: &mut Engine, this: Ptr<NiTArray>) -> u16 {
    e.get(this, NiTArray::m_usESize)
}

// Translated from 00993880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<CombatGroup*>` destructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): the vtable `0108db4c`, then `004ede70` frees
/// `m_pBase`.
pub fn fn_00993880(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), COMBAT_GROUP_ARRAY_VTABLE);
    let base = e.get(this, NiTArray::m_pBase);
    e.call(ARRAY_FREE, &args![base]);
}

// Translated from 00993990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<..,Actor*,__int64>` constructor (unnamed in the engine
/// map; `thiscall`, `RET 4`): the vtable `0108db5c`, the hash size at `+4`,
/// the count at `+0xc` cleared, and a bucket block of `hash_size * 4`
/// bytes (`00aa1070`) at `+8`, zeroed (`00403d30`). Returns `this`.
pub fn fn_00993990(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, ACTOR_MAP_BASE_VTABLE);
    e.mem.set_u32(me.wrapping_add(4), hash_size);
    e.mem.set_u32(me.wrapping_add(0xc), 0);
    let bytes = e.mem.u32(me.wrapping_add(4)).wrapping_shl(2);
    let buckets = e.call(BUCKETS_ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(me.wrapping_add(8), buckets);
    let bytes = e.mem.u32(me.wrapping_add(4)).wrapping_shl(2);
    let buckets = e.mem.u32(me.wrapping_add(8));
    e.call(MEMSET, &args![buckets, 0u32, bytes]);
    this
}

// Translated from 00993820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMap<Actor*,__int64>` constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): the base constructor (`00993990`), then the vtable
/// `0108db2c`. Returns `this`.
pub fn fn_00993820(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_00993990(e, this, hash_size);
    e.mem.set_u32(this.addr(), ACTOR_MAP_VTABLE);
    this
}

// Translated from 00993bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<..,Actor*,__int64>` destructor (unnamed in the engine
/// map; `thiscall`, plain `RET`): the vtable `0108db5c`, `00438af0` on the
/// map, then `00aa10f0` frees the bucket block at `+8`.
pub fn fn_00993bc0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ACTOR_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr().wrapping_add(8));
    e.call(BUCKETS_FREE, &args![buckets]);
}

// Translated from 00993b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00993b60 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The `NiTMap<Actor*,__int64>` destructor (unnamed in the engine map;
/// `thiscall`, plain `RET`): the vtable `0108db2c`, `00438af0` on the map,
/// then the base destructor (`00993bc0`).
pub fn fn_00993b60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ACTOR_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_00993bc0(e, this);
}

// Translated from 00993850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<Actor*,__int64>::scalar deleting destructor` (Xbox PDB name in
/// the engine map; `thiscall`, `RET 4`): the destructor (`00993b60`), then
/// `00401030` frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_actor_int64_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00993b60(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00992920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`; the manager and an
/// actor). Nothing when `0042ce10` answers true for the object at
/// `011ddf38`. Otherwise every combat group gets `00987820` with the actor;
/// when the actor's virtual slot `+0x100` answers true, `00992a30` (on the
/// manager) and `009a41e0` (on the object at `+0x1c`) get it too; last
/// `009a51e0` on the object at `+0x14` with the actor.
pub fn fn_00992920(e: &mut Engine, this: Ptr, actor: Ptr) {
    let me = this.addr();
    let object = e.mem.u32(OBJECT_POINTER_011DDF38);
    if e.call(OBJECT_TEST, &args![object]).bool() {
        return;
    }
    let mut chosen = 0u32;
    if e.vcall(actor.addr(), 0x100, &[]).bool() {
        chosen = actor.addr();
    }
    let mut i = 0u32;
    while i < e.call(GROUP_COUNT, &args![this]).u32() {
        let group = group_at(e, this, i);
        if group != 0 {
            e.call(0x0098_7820, &args![group, actor]);
        }
        i = i.wrapping_add(1);
    }
    if chosen != 0 {
        e.call(0x0099_2a30, &args![this, chosen]);
        let target_map = e.mem.u32(me.wrapping_add(0x1c));
        e.call(0x009a_41e0, &args![target_map, chosen]);
    }
    let threat_map = e.mem.u32(me.wrapping_add(0x14));
    e.call(0x009a_51e0, &args![threat_map, actor]);
}

// Translated from 009931c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatManager::GetCombatantCount` (Xbox PDB; `thiscall`, `RET 8`):
/// sums the member counts (`00990890`) of the combat groups that have
/// `target` as a target (`CombatGroup::IsTarget`; every group when `target`
/// is null), one less for the group the player's virtual slot `+0x3f8`
/// answers (when its count is not zero). With a target and `recent` not
/// null, `*recent` (cleared first) also gets the counts of the groups whose
/// time since their stamp for the target (`00989090`, `00435e00`) is above
/// the setting at `011ce528`.
pub fn combat_manager_get_combatant_count(
    e: &mut Engine,
    this: Ptr,
    target: Ptr,
    recent: Ptr,
) -> u32 {
    let mut total = 0u32;
    if !recent.is_null() {
        e.mem.set_u32(recent.addr(), 0);
    }
    let mut i = 0u32;
    while i < e.call(GROUP_COUNT, &args![this]).u32() {
        let group = group_at(e, this, i);
        if group != 0 && (target.is_null() || e.call(GROUP_IS_TARGET, &args![group, target]).bool())
        {
            let mut count = e.call(GROUP_MEMBER_COUNT, &args![group]).u32();
            if count != 0 {
                let player = e.mem.u32(PLAYER_POINTER);
                if e.vcall(player, 0x3f8, &[]).u32() == group {
                    count = count.wrapping_sub(1);
                }
            }
            total = total.wrapping_add(count);
            if !target.is_null() && !recent.is_null() {
                let since = group_time_since(e, group, target.addr());
                let limit = x87_load32(setting_bits(e, SETTING_011CE528));
                if limit < since {
                    let sum = e.mem.u32(recent.addr()).wrapping_add(count);
                    e.mem.set_u32(recent.addr(), sum);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    total
}

// Translated from 009932d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatManager::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): writes to the
/// save buffer the word at `+0x34` (`00865e50`, 4 bytes); a variable-sized
/// block (`StartVariableSizedValue` .. `SaveVariableSizedValue`) with, for
/// each combat group, the words from `00559450` and `00726070` and the
/// group's own `SaveGame`, then its count; the threat map's `SaveGame`
/// (object at `+0x14`); a variable-sized block with the form id of each key
/// of the actor map at `+0x10` (walked with `004b9ba0` and `GetNext`), then
/// its count; the time stamp at `+0x20` and the timers at `+0x24` and
/// `+0x2c`.
pub fn combat_manager_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let me = this.addr();
    e.call(
        0x0086_5e50,
        &args![buffer, me.wrapping_add(0x34), 4u32, 0u32],
    );
    // The frame's locals: iterator +0, key +4, value +8 (8 bytes), two
    // words +0x10 and +0x14.
    e.with_stack(0x18, |e, frame| {
        let f = frame.addr();
        let mut saved = 0u32;
        let start = e.call(0x0086_5f20, &args![buffer]).u32();
        let mut i = 0u32;
        while i < e.call(GROUP_COUNT, &args![this]).u32() {
            let group = group_at(e, this, i);
            if group != 0 {
                let first = e.call(0x0055_9450, &args![group]).u32();
                e.mem.set_u32(f + 0x10, first);
                e.call(0x0086_5e50, &args![buffer, f + 0x10, 4u32, 0u32]);
                let second = e.call(0x0072_6070, &args![group]).u32();
                e.mem.set_u32(f + 0x14, second);
                e.call(0x0086_5e50, &args![buffer, f + 0x14, 4u32, 0u32]);
                e.call(0x0098_de90, &args![group, buffer]);
                saved = saved.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        e.call(0x0086_5ff0, &args![buffer, saved, start]);
        let threat_map = e.mem.u32(me.wrapping_add(0x14));
        e.call(0x009a_59e0, &args![threat_map, buffer]);
        let mut keys = 0u32;
        let start = e.call(0x0086_5f20, &args![buffer]).u32();
        let actor_map = e.mem.u32(me.wrapping_add(0x10));
        let first = e.call(0x004b_9ba0, &args![actor_map]).u32();
        e.mem.set_u32(f, first);
        while e.mem.u32(f) != 0 {
            let actor_map = e.mem.u32(me.wrapping_add(0x10));
            e.call(0x0099_3a80, &args![actor_map, f, f + 4, f + 8]);
            let key = e.mem.u32(f + 4);
            e.call(0x0086_5df0, &args![buffer, key, 0u32]);
            keys = keys.wrapping_add(1);
        }
        e.call(0x0086_5ff0, &args![buffer, keys, start]);
    });
    e.call(0x009a_5f90, &args![me.wrapping_add(0x20), buffer]);
    e.call(0x009a_5eb0, &args![me.wrapping_add(0x24), buffer]);
    e.call(0x009a_5eb0, &args![me.wrapping_add(0x2c), buffer]);
}

/// `006a7f50` (`thiscall`): the float at the object, in `ST0`; `0108d9b0`:
/// the float whose negation marks "not set".
const READ_FLOAT: u32 = 0x006a_7f50;
const UNSET_MAGNITUDE: u32 = 0x0108_d9b0;
/// `008d7f80` (`thiscall`, a byte) and `008d7f40` (`thiscall`, one float)
/// on the timers at `+0x2c` and `+0x24`.
const TIMER_TEST: u32 = 0x008d_7f80;
const TIMER_SET: u32 = 0x008d_7f40;
/// `00910b10`, `0097fa50` (`thiscall` on the timer at `+0x24`).
const TIMER_RESET: u32 = 0x0091_0b10;
const TIMER_RUNNING: u32 = 0x0097_fa50;
/// `00435dd0` (no arguments): the current time, in `ST0`; `00435de0`
/// (`thiscall`, one float): stores it into a time stamp.
const CURRENT_TIME: u32 = 0x0043_5dd0;
const STAMP_SET: u32 = 0x0043_5de0;

/// Whether the float at `+0x20` (`006a7f50`) differs from the negated
/// float at `0108d9b0` (`FUCOMPP`; a NaN differs).
fn stamp_is_set(e: &mut Engine, this: Ptr) -> bool {
    let stamp = e
        .call(READ_FLOAT, &args![this.addr().wrapping_add(0x20)])
        .f64();
    let unset = -x87_load32(e.mem.u32(UNSET_MAGNITUDE));
    unset != stamp
}

/// `00435de0` on a stack stamp with the float `bits`: the stamp's word.
fn stamp_word(e: &mut Engine, bits: u32) -> u32 {
    e.with_stack(4, |e, stamp| {
        e.call(STAMP_SET, &args![stamp, bits]);
        e.mem.u32(stamp.addr())
    })
}

// Translated from 00992d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`; the combat manager).
/// "Set" means the float at `+0x20` is not the negated float at `0108d9b0`.
/// Answers whether it is set; when the timer at `+0x2c` passes `008d7f80`
/// and `0042ce10` answers false for the object at `011ddf38`, it first
/// restarts that timer (`008d7f40`, setting `011ceec0`) and looks for a
/// reason to set it, group by group (groups that `0097ef30` marks, that
/// `005a4320` answers 0 for or that `0098a2e0` marks are passed over):
/// - a group with the player as a target, stamped (`00989090`, `00435e00`)
///   less than the setting `011ce528` ago: a reason when already set;
///   otherwise when, with `total` the sum of `00453700(00586150(..))` over
///   the members the player passes `009670f0` for and whose slot `+0x428`
///   answers non-null, `total > 0` and the setting `011ce04c` is above
///   `strength / total` (`strength`: the player's `008acbe0` with the
///   float at `01012054`);
/// - another group that `009931a0` passes: when set, a reason when
///   `00989880` with the player and the setting `011ce454` answers
///   non-zero; when not set and `00989880` with the setting `011cfe5c`
///   answers non-zero, `0098cb30(0)` is added to a running sum and, once a
///   member that passes `009670f0` has been seen, the sum above `strength`
///   times the setting `011ce72c` is a reason.
///
/// When the answer changes: becoming set restarts the timer at `+0x24`
/// (`00910b10`) and stamps `+0x20` with the current time (`00435dd0`,
/// `00435de0`); becoming unset either restarts that timer with the setting
/// `011cfc14` (`008d7f40`, when `0097fa50` answers false) or, when
/// `008d7f80` passes, stamps `+0x20` "unset" and resets the timer. It
/// answers the state after that.
pub fn fn_00992d90(e: &mut Engine, this: Ptr) -> bool {
    let me = this.addr();
    let set = stamp_is_set(e, this);
    if !e.call(TIMER_TEST, &args![me.wrapping_add(0x2c)]).bool() {
        return set;
    }
    let object = e.mem.u32(OBJECT_POINTER_011DDF38);
    if e.call(OBJECT_TEST, &args![object]).bool() {
        return set;
    }
    let delay = x87_quiet_bits(setting_bits(e, SETTING_011CEEC0));
    e.call(TIMER_SET, &args![me.wrapping_add(0x2c), delay]);
    let mut reason = false;
    let mut sum_bits = x87_store32(0.0);
    let mut seen = false;
    let player = e.mem.u32(PLAYER_POINTER);
    let strength_argument = x87_quiet_bits(e.mem.u32(0x0101_2054));
    let strength = e.call(0x008a_cbe0, &args![player, strength_argument]).f64();
    let strength_bits = x87_store32(strength);
    let mut i = 0u32;
    while i < e.call(GROUP_COUNT, &args![this]).u32() {
        let group = group_at(e, this, i);
        if group != 0 {
            if e.call(0x0097_ef30, &args![group]).bool()
                || e.call(0x005a_4320, &args![group]).u32() == 0
                || e.call(0x0098_a2e0, &args![group]).bool()
            {
                // Straight to the next group, without the check below.
                i = i.wrapping_add(1);
                continue;
            }
            let player = e.mem.u32(PLAYER_POINTER);
            if e.call(GROUP_IS_TARGET, &args![group, player]).bool() {
                let player = e.mem.u32(PLAYER_POINTER);
                let since = group_time_since(e, group, player);
                let limit = x87_load32(setting_bits(e, SETTING_011CE528));
                if limit > since {
                    if set {
                        reason = true;
                    } else {
                        let mut total_bits = x87_store32(0.0);
                        let members = e.call(GROUP_MEMBER_COUNT, &args![group]).u32();
                        let mut j = 0u32;
                        while j < members {
                            let member = e.call(GROUP_MEMBER, &args![group, j]).u32();
                            let player = e.mem.u32(PLAYER_POINTER);
                            if e.call(0x0096_70f0, &args![player, member]).bool() {
                                let process = e.vcall(member, 0x428, &[]).u32();
                                if process != 0 {
                                    let inner = e.call(0x0058_6150, &args![process]).u32();
                                    let value = e.call(0x0045_3700, &args![inner]).f64();
                                    total_bits = x87_store32(x87_add(value, x87_m32(total_bits)));
                                }
                            }
                            j = j.wrapping_add(1);
                        }
                        let zero: f64 = e.global(0x0101_2060);
                        if x87_load32(total_bits) > zero {
                            let ratio_bits = x87_store32(x87_div(
                                x87_load32(strength_bits),
                                x87_m32(total_bits),
                            ));
                            let limit = x87_load32(setting_bits(e, SETTING_011CE04C));
                            if limit > x87_load32(ratio_bits) {
                                reason = true;
                            }
                        }
                    }
                }
            } else if e.call(0x0099_31a0, &args![group]).bool() {
                if set {
                    let radius = x87_quiet_bits(setting_bits(e, SETTING_011CE454));
                    let player = e.mem.u32(PLAYER_POINTER);
                    if e.call(0x0098_9880, &args![group, player, radius]).u32() != 0 {
                        reason = true;
                    }
                } else {
                    let radius = x87_quiet_bits(setting_bits(e, SETTING_011CFE5C));
                    let player = e.mem.u32(PLAYER_POINTER);
                    if e.call(0x0098_9880, &args![group, player, radius]).u32() != 0 {
                        if !seen {
                            let members = e.call(GROUP_MEMBER_COUNT, &args![group]).u32();
                            let mut j = 0u32;
                            while j < members {
                                let member = e.call(GROUP_MEMBER, &args![group, j]).u32();
                                let player = e.mem.u32(PLAYER_POINTER);
                                if e.call(0x0096_70f0, &args![player, member]).bool() {
                                    seen = true;
                                    break;
                                }
                                j = j.wrapping_add(1);
                            }
                        }
                        let value = e.call(0x0098_cb30, &args![group, 0u32]).f64();
                        sum_bits = x87_store32(x87_add(value, x87_m32(sum_bits)));
                        if seen {
                            let factor = setting_bits(e, SETTING_011CE72C);
                            let product = x87_mul(x87_load32(strength_bits), x87_m32(factor));
                            if x87_load32(sum_bits) > product {
                                reason = true;
                            }
                        }
                    }
                }
            }
        }
        if reason {
            break;
        }
        i = i.wrapping_add(1);
    }
    if set != reason {
        let timer = me.wrapping_add(0x24);
        if reason {
            e.call(TIMER_RESET, &args![timer]);
            let now = e.call(CURRENT_TIME, &[]).f64();
            let word = stamp_word(e, x87_store32(now));
            e.mem.set_u32(me.wrapping_add(0x20), word);
        } else if !e.call(TIMER_RUNNING, &args![timer]).bool() {
            let delay = x87_quiet_bits(setting_bits(e, SETTING_011CFC14));
            e.call(TIMER_SET, &args![timer, delay]);
        } else if e.call(TIMER_TEST, &args![timer]).bool() {
            let unset = x87_store32(-x87_load32(e.mem.u32(UNSET_MAGNITUDE)));
            let word = stamp_word(e, unset);
            e.mem.set_u32(me.wrapping_add(0x20), word);
            e.call(TIMER_RESET, &args![timer]);
        }
    }
    stamp_is_set(e, this)
}

// ---- Batch n023 ----------------------------------------------------------------------------

use crate::types::NiTPointerMap;

/// `operator new` (`00401000`, `cdecl(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The debug print `005b5e40` (`cdecl`: a format and its words).
const DEBUG_PRINT: u32 = 0x005b_5e40;
/// The scope object `00404eb0` (`thiscall`, `RET 0x10`: 0x2b, 1, a source file name, a line)
/// a function keeps on its stack, and `00404ee0` (`thiscall`) that ends it.
const SCOPE_BEGIN: u32 = 0x0040_4eb0;
const SCOPE_END: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout\AI\Combat\CombatManager.cpp"`.
const SOURCE_FILE: u32 = 0x0108_d9f0;
/// `"COMBAT: In CombatManager, MAX_NUM_COMBAT_GROUPS( %i ) has been exceeded to %i. ..."`.
const TOO_MANY_GROUPS_FORMAT: u32 = 0x0108_da40;
/// `"COMBAT: Could not move player teammate %s (%08X) to the player's new group in
/// CombatManager::SplitPlayerCombatGroup()"`.
const TEAMMATE_NOT_MOVED_FORMAT: u32 = 0x0108_dab0;
/// `00985d10` (`thiscall`, one word): the constructor run on a 0x15C-byte combat group block.
const GROUP_CONSTRUCT: u32 = 0x0098_5d10;
/// `00537e90` (`thiscall`, one word): gives a group its index in the manager's array.
const GROUP_SET_INDEX: u32 = 0x0053_7e90;
/// `CombatGroup::AddMember` (`009867d0`) and `CombatGroup::AddTarget` (`00986410`), engine-map
/// names (`thiscall`, one word, a byte result); `009869a0` (`thiscall`, one word: an actor).
const GROUP_ADD_MEMBER: u32 = 0x0098_67d0;
const GROUP_ADD_TARGET: u32 = 0x0098_6410;
const GROUP_REMOVE: u32 = 0x0098_69a0;
/// `CombatController::CombatController` (engine-map name of `0097d3a0`; `thiscall`, `RET 0xc`)
/// on a 0x188-byte block.
const CONTROLLER_CONSTRUCT: u32 = 0x0097_d3a0;
/// `004fb070` (`thiscall`): the group of a combat controller.
const CONTROLLER_GROUP: u32 = 0x004f_b070;
/// `00559450` (`thiscall`): a number the group answers (used as a bit index into the actor
/// map's 64-bit masks).
const GROUP_NUMBER: u32 = 0x0055_9450;
/// `__allshl` (`00ec7810`): the 64-bit value (two words) shifted left by the count (third word).
const SHIFT_LEFT_64: u32 = 0x00ec_7810;
/// `005a4320` (`thiscall` on a group): the number of its targets; `00968630` (`thiscall`, one
/// word: an index): that target.
const GROUP_TARGET_COUNT: u32 = 0x005a_4320;
const GROUP_TARGET: u32 = 0x0096_8630;
/// `004b9ba0` (`thiscall` on a map): the first position of a walk over the map.
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
/// `00865e50` (save buffer, `RET 0xc`: data, size, 0) and `00864980` (load buffer, `RET 8`:
/// data, size); `00864a60` (load buffer): a variable-sized count; `008648e0` (load buffer, one
/// word): reads a form id into the word.
const LOAD_DATA: u32 = 0x0086_4980;
const LOAD_COUNT: u32 = 0x0086_4a60;
const LOAD_FORM_ID: u32 = 0x0086_48e0;
/// `NiTArray<..>::SetAtGrow` (engine-map name of `00470000`; `thiscall`, `RET 8`: index, the
/// address of the element).
const ARRAY_SET_AT_GROW: u32 = 0x0047_0000;
/// The settings `00992720` reads (addresses of the setting objects).
const SETTING_011CF3F0: u32 = 0x011c_f3f0;
const SETTING_011CEA74: u32 = 0x011c_ea74;
/// The object `0043d4d0` (`thiscall`) answers the address of an integer limit for.
const LIMIT_OBJECT_011CE3CC: u32 = 0x011c_e3cc;

/// Runs `f` inside the scope object (`00404eb0` with 0x2b, 1, the source file and `line`;
/// `00404ee0` after `f`), as the exe keeps it on its stack.
fn in_scope<R>(e: &mut Engine, line: u32, f: impl FnOnce(&mut Engine) -> R) -> R {
    e.with_stack(4, |e, scope| {
        e.call(SCOPE_BEGIN, &args![scope, 0x2bu32, 1u32, SOURCE_FILE, line]);
        let r = f(e);
        e.call(SCOPE_END, &args![scope]);
        r
    })
}

/// A 0x188-byte block (`00401000`) given to the combat controller constructor with `actor`,
/// `group` and `flag`; null when the allocation fails.
fn new_controller(e: &mut Engine, actor: Ptr, group: u32, flag: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x188u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(CONTROLLER_CONSTRUCT, &args![block, actor, group, flag])
        .u32()
}

/// The actor's name for a debug message: its virtual slot `+0x130` when `00474cb0` answers
/// non-zero, `0055d520` otherwise.
fn actor_name(e: &mut Engine, actor: u32) -> u32 {
    if e.call(0x0047_4cb0, &args![actor]).u32() != 0 {
        e.vcall(actor, 0x130, &[]).u32()
    } else {
        e.call(0x0055_d520, &args![actor]).u32()
    }
}

// Translated from 00844310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<Actor*,__int64>>,Actor*,__int64>::SetAt` (Xbox PDB; `thiscall`,
/// `RET 0xc`: the key and the two words of the value): the bucket from the map's virtual slot
/// `+4` (the hash); an item whose key slot `+8` matches gets the value (`+8`, `+0xc`);
/// otherwise a new item (slot `+0x14`) is filled through slot `+0xc` (item, key, value), put
/// at the head of the bucket, and the count (`+0xc`) goes up.
pub fn ni_t_map_base_actor_int64_set_at(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    key: u32,
    value_low: u32,
    value_high: u32,
) {
    let me = this.addr();
    let bucket = e.vcall(me, 4, &args![key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(me, 8, &args![key, item_key]).bool() {
            e.mem.set_u32(item.wrapping_add(8), value_low);
            e.mem.set_u32(item.wrapping_add(0xc), value_high);
            return;
        }
        item = e.mem.u32(item);
    }
    let item = e.vcall(me, 0x14, &[]).u32();
    e.vcall(me, 0xc, &args![item, key, value_low, value_high]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let slot = table.wrapping_add(bucket.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(item, head);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.mem
        .set_u32(table.wrapping_add(bucket.wrapping_mul(4)), item);
    let count = e.get(this, NiTPointerMap::m_uiCount);
    e.set(this, NiTPointerMap::m_uiCount, count.wrapping_add(1));
}

// Translated from 00993a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor map's `GetAt` (unnamed in the engine map; `thiscall`, `RET 8`): walks the key's
/// bucket (virtual slot `+4`, the hash) comparing keys through slot `+8`; on a match copies
/// the item's two value words into `value` and answers true, otherwise false.
pub fn fn_00993a00(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, value: Ptr) -> bool {
    let me = this.addr();
    let bucket = e.vcall(me, 4, &args![key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(me, 8, &args![key, item_key]).bool() {
            let low = e.mem.u32(item.wrapping_add(8));
            e.mem.set_u32(value.addr(), low);
            let high = e.mem.u32(item.wrapping_add(0xc));
            e.mem.set_u32(value.addr().wrapping_add(4), high);
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

// Translated from 00993a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..,Actor*,__int64>::GetNext` (Xbox PDB; `thiscall`, `RET 0xc`): copies the key
/// (`+4`) and value (`+8`, `+0xc`) of the item at `*position` into `key` and `value`, then
/// moves `*position` to the item's next (`+0`) or to the head of the next non-empty bucket
/// after the item's own (virtual slot `+4` on its key), or to null past the last bucket.
pub fn ni_t_map_base_actor_int64_get_next(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    position: Ptr,
    key: Ptr,
    value: Ptr,
) {
    let item = e.mem.u32(position.addr());
    let item_key = e.mem.u32(item.wrapping_add(4));
    e.mem.set_u32(key.addr(), item_key);
    let low = e.mem.u32(item.wrapping_add(8));
    e.mem.set_u32(value.addr(), low);
    let high = e.mem.u32(item.wrapping_add(0xc));
    e.mem.set_u32(value.addr().wrapping_add(4), high);
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let item_key = e.mem.u32(item.wrapping_add(4));
    let mut bucket = e.vcall(this.addr(), 4, &args![item_key]).u32();
    bucket = bucket.wrapping_add(1);
    while bucket < e.get(this, NiTPointerMap::m_uiHashSize) {
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let head = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
        if head != 0 {
            e.mem.set_u32(position.addr(), head);
            return;
        }
        bucket = bucket.wrapping_add(1);
    }
    e.mem.set_u32(position.addr(), 0);
}

// Translated from 00993b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..,Actor*,__int64>::SetValue` (Xbox PDB; `thiscall`, `RET 0x10`, `this` not
/// read): the item's key (`+4`) and two value words (`+8`, `+0xc`).
pub fn ni_t_map_base_actor_int64_set_value(
    e: &mut Engine,
    _this: Ptr,
    item: Ptr,
    key: u32,
    value_low: u32,
    value_high: u32,
) {
    e.mem.set_u32(item.addr().wrapping_add(4), key);
    e.mem.set_u32(item.addr().wrapping_add(8), value_low);
    e.mem.set_u32(item.addr().wrapping_add(0xc), value_high);
}

// Translated from 00993c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<Actor*,__int64>::DeleteItem` (Xbox PDB; `thiscall`, `RET 4`): clears the item's
/// value words (`+8`, `+0xc`) and hands it to `006b8310` with `ECX` = `this + 0xc`.
pub fn ni_t_map_actor_int64_delete_item(e: &mut Engine, this: Ptr, item: Ptr) {
    e.mem.set_u32(item.addr().wrapping_add(8), 0);
    e.mem.set_u32(item.addr().wrapping_add(0xc), 0);
    e.call(0x006b_8310, &args![this.byte_add(0xc), item]);
}

// Translated from 00993cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor map's item allocation (unnamed in the engine map; `thiscall`, plain `RET`, `this`
/// not read): a 0x10-byte block from `00401000`.
pub fn fn_00993cd0(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(OPERATOR_NEW, &args![0x10u32]).u32()
}

// Translated from 00993c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<CombatGroup*>` deleting destructor (by its body; `thiscall`, `RET 4`): the
/// destructor `00993880`, then `00401030` frees `this` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_00993c40(e: &mut Engine, this: Ptr<NiTArray>, flags: u32) -> Ptr<NiTArray> {
    fn_00993880(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00993c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A deleting destructor (unnamed in the engine map; `thiscall`, `RET 4`): `00991130` on
/// `this`, then `00401030` frees `this` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00993c70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0099_1130, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00993ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..,Actor*,__int64>::scalar deleting destructor` (Xbox PDB name in the engine
/// map; `thiscall`, `RET 4`): the base destructor (`00993bc0`), then `00401030` frees `this`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_actor_int64_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00993bc0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 009938d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<CombatGroup*>` add (unnamed in the engine map; `thiscall`, `RET 4`): -1 for a
/// null `*element`; otherwise the element goes into the first null slot below `m_usSize`
/// (`m_usESize` goes up) and its index is answered, or, with no such slot, `SetAtGrow`
/// (`00470000`) at `m_usSize` answers.
pub fn fn_009938d0(e: &mut Engine, this: Ptr<NiTArray>, element: Ptr) -> u32 {
    if e.mem.u32(element.addr()) == 0 {
        return 0xffff_ffff;
    }
    let mut index = 0u16;
    while index < e.get(this, NiTArray::m_usSize) {
        let base = e.get(this, NiTArray::m_pBase);
        let slot = base.wrapping_add(u32::from(index) * 4);
        if e.mem.u32(slot) == 0 {
            let value = e.mem.u32(element.addr());
            e.mem.set_u32(slot, value);
            let count = e.get(this, NiTArray::m_usESize);
            e.set(this, NiTArray::m_usESize, count.wrapping_add(1));
            return u32::from(index);
        }
        index = index.wrapping_add(1);
    }
    let size = e.get(this, NiTArray::m_usSize);
    e.call(ARRAY_SET_AT_GROW, &args![this, u32::from(size), element])
        .u32()
}

// Translated from 00991e80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00991e80 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `CombatManager::CreateCombatGroup` (Xbox PDB; `thiscall`, plain `RET`): inside the scope
/// object (line 0x161), a 0x15C-byte block (`00401000`) constructed by `00985d10` with the
/// manager's next group number (`+0x34`, which goes up), or null when the allocation fails;
/// the group goes into the manager's group array (`009938d0` on `this`); an index above 0x40
/// prints `"COMBAT: In CombatManager, MAX_NUM_COMBAT_GROUPS( %i ) ..."` (`005b5e40`), and
/// `00537e90` gives the group its index. Returns the group.
pub fn combat_manager_create_combat_group(e: &mut Engine, this: Ptr) -> u32 {
    let me = this.addr();
    in_scope(e, 0x161, |e| {
        let block = e.call(OPERATOR_NEW, &args![0x15cu32]).u32();
        let group = if block != 0 {
            let number = e.mem.u32(me.wrapping_add(0x34));
            e.mem.set_u32(me.wrapping_add(0x34), number.wrapping_add(1));
            e.call(GROUP_CONSTRUCT, &args![block, number]).u32()
        } else {
            0
        };
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), group);
            let index = fn_009938d0(e, Ptr::new(me), slot);
            if index > 0x40 {
                e.call(DEBUG_PRINT, &args![TOO_MANY_GROUPS_FORMAT, 0x40u32, index]);
            }
            let group = e.mem.u32(slot.addr());
            e.call(GROUP_SET_INDEX, &args![group, index]);
            e.mem.u32(slot.addr())
        })
    })
}

// Translated from 00991f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatManager::SplitPlayerCombatGroup` (Xbox PDB; `thiscall`, `RET 4`): takes `target`
/// off the player's group (the player's virtual slot `+0x3f8`; `009869a0`). When that group
/// has more than one member (`00990890`), the player leaves it for a new group
/// (`CreateCombatGroup`, `AddMember`, the player's slot `+0x3fc`, `00986740` with the old
/// group), and each member of the old group that `00566950` answers true for moves along
/// (`AddMember`; on success `009869a0` on the old group, the member's slot `+0x3fc`, and the
/// index steps back), a failure printing `"COMBAT: Could not move player teammate ..."` with
/// the member's name and `0084e3a0`. Then the player's (possibly new) group gets `target` as a
/// target, and `target` becomes the member of another new group, which is returned.
pub fn combat_manager_split_player_combat_group(e: &mut Engine, this: Ptr, target: Ptr) -> u32 {
    let player = e.mem.u32(PLAYER_POINTER);
    let mut group = e.vcall(player, 0x3f8, &[]).u32();
    e.call(GROUP_REMOVE, &args![group, target]);
    if e.call(GROUP_MEMBER_COUNT, &args![group]).u32() > 1 {
        let old = group;
        let player = e.mem.u32(PLAYER_POINTER);
        e.call(GROUP_REMOVE, &args![old, player]);
        group = combat_manager_create_combat_group(e, this);
        let player = e.mem.u32(PLAYER_POINTER);
        e.call(GROUP_ADD_MEMBER, &args![group, player]);
        let player = e.mem.u32(PLAYER_POINTER);
        e.vcall(player, 0x3fc, &args![group]);
        e.call(0x0098_6740, &args![group, old]);
        let mut i = 0u32;
        while i < e.call(GROUP_MEMBER_COUNT, &args![old]).u32() {
            let member = e.call(GROUP_MEMBER, &args![old, i]).u32();
            if e.call(0x0056_6950, &args![member]).bool() {
                if e.call(GROUP_ADD_MEMBER, &args![group, member]).bool() {
                    e.call(GROUP_REMOVE, &args![old, member]);
                    e.vcall(member, 0x3fc, &args![group]);
                    i = i.wrapping_sub(1);
                } else {
                    let name = actor_name(e, member);
                    let id = e.call(0x0084_e3a0, &args![member]).u32();
                    e.call(DEBUG_PRINT, &args![TEAMMATE_NOT_MOVED_FORMAT, name, id]);
                }
            }
            i = i.wrapping_add(1);
        }
    }
    e.call(GROUP_ADD_TARGET, &args![group, target]);
    let new_group = combat_manager_create_combat_group(e, this);
    e.call(GROUP_ADD_MEMBER, &args![new_group, target]);
    new_group
}

// Translated from 00992110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00992110 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `CombatManager::AddCombatant` (Xbox PDB; `thiscall`, `RET 0x10`; the last two words are not
/// read): inside the scope object (line 0x1a7). An actor with a combat controller (virtual
/// slot `+0x428`) adds `target` to its controller's group (`004fb070`, `AddTarget`) and
/// answers the controller. Otherwise, when `008b0670(actor, target)` answers true, a new group
/// gets the actor (`CreateCombatGroup`, `AddMember`) and a new controller is made for it
/// (`0097d3a0` with 0 on 0x188 bytes); when `AddTarget` then fails the controller's slot
/// `+0x10` runs with 1 (when it exists) and null is answered, else the controller. Null when
/// `008b0670` answers false.
pub fn combat_manager_add_combatant(
    e: &mut Engine,
    this: Ptr,
    actor: Ptr,
    target: Ptr,
    _unused_3: u32,
    _unused_4: u32,
) -> u32 {
    in_scope(e, 0x1a7, |e| {
        let controller = e.vcall(actor.addr(), 0x428, &[]).u32();
        if controller != 0 {
            let group = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            e.call(GROUP_ADD_TARGET, &args![group, target]);
            return controller;
        }
        if !e.call(0x008b_0670, &args![actor, target]).bool() {
            return 0;
        }
        let group = combat_manager_create_combat_group(e, this);
        e.call(GROUP_ADD_MEMBER, &args![group, actor]);
        let controller = new_controller(e, actor, group, 0);
        if !e.call(GROUP_ADD_TARGET, &args![group, target]).bool() {
            if controller != 0 {
                e.vcall(controller, 0x10, &args![1u32]);
            }
            return 0;
        }
        controller
    })
}

// Translated from 009922b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009922b0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `CombatManager::AddGroupMember` (Xbox PDB; `thiscall`, `RET 8`): inside the scope object
/// (line 0x1c6). An actor with a combat controller (virtual slot `+0x428`) whose group
/// (`004fb070`) is not `group` and that `group` merges (`CombatGroup::MergeGroup`, engine-map
/// name of `00987530`) runs `00986bd0` on its old group; an actor without one gets a new
/// controller (`0097d3a0` with 0) once `AddMember` accepts it. Answers the controller (null
/// when there is none).
pub fn combat_manager_add_group_member(e: &mut Engine, _this: Ptr, actor: Ptr, group: Ptr) -> u32 {
    in_scope(e, 0x1c6, |e| {
        let mut controller = e.vcall(actor.addr(), 0x428, &[]).u32();
        if controller != 0 {
            let old = e.call(CONTROLLER_GROUP, &args![controller]).u32();
            if old != group.addr() && e.call(0x0098_7530, &args![group, old]).bool() {
                e.call(0x0098_6bd0, &args![old]);
            }
        } else if e.call(GROUP_ADD_MEMBER, &args![group, actor]).bool() {
            controller = new_controller(e, actor, group.addr(), 0);
        }
        controller
    })
}

// Translated from 009923d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009923d0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `CombatManager::AddPretendCombatant` (Xbox PDB; `thiscall`, `RET 8`): a new group with the
/// actor (`CreateCombatGroup`, `AddMember`) and a new controller for them (`0097d3a0` with
/// `flag` on 0x188 bytes); answers the controller (null when the allocation fails).
pub fn combat_manager_add_pretend_combatant(
    e: &mut Engine,
    this: Ptr,
    actor: Ptr,
    flag: u32,
) -> u32 {
    let group = combat_manager_create_combat_group(e, this);
    e.call(GROUP_ADD_MEMBER, &args![group, actor]);
    new_controller(e, actor, group, flag)
}

// Translated from 00992480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): nothing when `actor` is `other`.
/// Otherwise, with `process` the actor's `008d8520` and `object` its slot `+0x22c`: when
/// `object` is null or `0041ca90` on it is not 0x18, the actor's slot `+0x40c` runs and
/// `object` is read again; an `object` whose `0041ca90` is 0x18 gets `009f8330(other)`; last
/// `009929e0(actor)` on the manager.
pub fn fn_00992480(e: &mut Engine, this: Ptr, actor: Ptr, other: Ptr) {
    if actor == other {
        return;
    }
    let process = e.call(0x008d_8520, &args![actor]).u32();
    let mut object = e.vcall(process, 0x22c, &[]).u32();
    if object == 0 || e.call(0x0041_ca90, &args![object]).u32() != 0x18 {
        e.vcall(actor.addr(), 0x40c, &[]);
        object = e.vcall(process, 0x22c, &[]).u32();
    }
    if object != 0 && e.call(0x0041_ca90, &args![object]).u32() == 0x18 {
        e.call(0x009f_8330, &args![object, other]);
    }
    fn_009929e0(e, this, actor);
}

// Translated from 00992530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatManager::GetWanttoHelpGroup` (Xbox PDB; `thiscall`, `RET 8`): false when
/// `Actor::GetShouldHelp` (engine-map name of `008b0970`) answers false for `helper` and
/// `other`; true when either is null or `other` has no group (slot `+0x3f8`); false when the
/// helper should help one of the group's targets (`005a4320`, `00968630`) or should attack one
/// of its members (`008b06d0(member, 0, &word, 0)`, the word cleared first); true otherwise.
pub fn combat_manager_get_want_to_help_group(
    e: &mut Engine,
    _this: Ptr,
    helper: Ptr,
    other: Ptr,
) -> bool {
    if !e.call(0x008b_0970, &args![helper, other]).bool() {
        return false;
    }
    if helper.is_null() || other.is_null() {
        return true;
    }
    let group = e.vcall(other.addr(), 0x3f8, &[]).u32();
    if group == 0 {
        return true;
    }
    let targets = e.call(GROUP_TARGET_COUNT, &args![group]).u32();
    let mut i = 0u32;
    while i < targets {
        let target = e.call(GROUP_TARGET, &args![group, i]).u32();
        if e.call(0x008b_0970, &args![helper, target]).bool() {
            return false;
        }
        i = i.wrapping_add(1);
    }
    let members = e.call(GROUP_MEMBER_COUNT, &args![group]).u32();
    let mut i = 0u32;
    while i < members {
        let member = e.call(GROUP_MEMBER, &args![group, i]).u32();
        let attacks = e.with_stack(4, |e, word| {
            e.call(0x008b_06d0, &args![helper, member, 0u32, word, 0u32])
                .bool()
        });
        if attacks {
            return false;
        }
        i = i.wrapping_add(1);
    }
    true
}

// Translated from 00992640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): whether `helper` wants to help
/// (`GetWanttoHelpGroup`) one of the targets (`005a4320`, `00968630`) of `actor`'s group
/// (slot `+0x3f8`); false without a group.
pub fn fn_00992640(e: &mut Engine, this: Ptr, helper: Ptr, actor: Ptr) -> bool {
    let group = e.vcall(actor.addr(), 0x3f8, &[]).u32();
    if group == 0 {
        return false;
    }
    let targets = e.call(GROUP_TARGET_COUNT, &args![group]).u32();
    let mut i = 0u32;
    while i < targets {
        let target = e.call(GROUP_TARGET, &args![group, i]).u32();
        if combat_manager_get_want_to_help_group(e, this, helper, Ptr::new(target)) {
            return true;
        }
        i = i.wrapping_add(1);
    }
    false
}

// Translated from 009926c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `TargetLocalActor` (`00992a50`) with the
/// two words swapped (`second`, then `first`).
pub fn fn_009926c0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    combat_manager_target_localactor(e, this, second, first);
}

// Translated from 009926e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `00992b00` with the two words swapped
/// (`second`, then `first`).
pub fn fn_009926e0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    fn_00992b00(e, this, second, first);
}

// Translated from 00992700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`, the first word not read): `009929e0` with
/// the second word.
pub fn fn_00992700(e: &mut Engine, this: Ptr, _unused_1: u32, actor: Ptr) {
    fn_009929e0(e, this, actor);
}

// Translated from 009929e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): an actor not yet in the manager's actor map
/// (`+0x10`; `00993a00`) is put in it with a zero mask (`SetAt`).
pub fn fn_009929e0(e: &mut Engine, this: Ptr, actor: Ptr) {
    let me = this.addr();
    let found = e.with_stack(8, |e, value| {
        let map = Ptr::new(e.mem.u32(me.wrapping_add(0x10)));
        fn_00993a00(e, map, actor.addr(), value)
    });
    if !found {
        let map = Ptr::new(e.mem.u32(me.wrapping_add(0x10)));
        ni_t_map_base_actor_int64_set_at(e, map, actor.addr(), 0, 0);
    }
}

/// The bit of `group` as `00992a50` and `00992b00` build it: the `int` `1 << n` (`SHL`, the
/// count taken modulo 32) for `n` = `00559450(group)`, sign-extended to 64 bits (`CDQ`).
fn group_bit_shifted(e: &mut Engine, group: u32) -> u64 {
    let n = e.call(GROUP_NUMBER, &args![group]).u32();
    let low = 1u32.wrapping_shl(n);
    low as i32 as i64 as u64
}

// Translated from 00992a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatManager::TargetLocalActor` (Xbox PDB; `thiscall`, `RET 8`): sets the bit of `group`
/// (`1 << 00559450(group)` as an `int`, sign-extended to 64 bits) in the actor's mask in the
/// actor map (`+0x10`; a missing actor counts as an empty mask) and stores the mask back
/// (`SetAt`) when the bit was clear.
pub fn combat_manager_target_localactor(e: &mut Engine, this: Ptr, actor: Ptr, group: Ptr) {
    let me = this.addr();
    let mut mask = e.with_stack(8, |e, value| {
        let map = Ptr::new(e.mem.u32(me.wrapping_add(0x10)));
        if fn_00993a00(e, map, actor.addr(), value) {
            u64::from(e.mem.u32(value.addr())) | u64::from(e.mem.u32(value.addr() + 4)) << 32
        } else {
            0
        }
    });
    let bit = group_bit_shifted(e, group.addr());
    if mask & bit == 0 {
        mask |= bit;
        let map = Ptr::new(e.mem.u32(me.wrapping_add(0x10)));
        ni_t_map_base_actor_int64_set_at(e, map, actor.addr(), mask as u32, (mask >> 32) as u32);
    }
}

// Translated from 00992b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): the counterpart of `TargetLocalActor`:
/// for an actor in the actor map whose mask has the bit of `group` (as there), clears the bit
/// and stores the mask back (`SetAt`).
pub fn fn_00992b00(e: &mut Engine, this: Ptr, actor: Ptr, group: Ptr) {
    let me = this.addr();
    let found = e.with_stack(8, |e, value| {
        let map = Ptr::new(e.mem.u32(me.wrapping_add(0x10)));
        if fn_00993a00(e, map, actor.addr(), value) {
            Some(u64::from(e.mem.u32(value.addr())) | u64::from(e.mem.u32(value.addr() + 4)) << 32)
        } else {
            None
        }
    });
    let Some(mask) = found else {
        return;
    };
    let bit = group_bit_shifted(e, group.addr());
    if mask & bit != 0 {
        let mask = !bit & mask;
        let map = Ptr::new(e.mem.u32(me.wrapping_add(0x10)));
        ni_t_map_base_actor_int64_set_at(e, map, actor.addr(), mask as u32, (mask >> 32) as u32);
    }
}

// Translated from 00992a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): removes the key from the actor map at
/// `+0x10` (`00405430`, engine-map name `NiTMapBase<..,unsigned int,CombatThreat*>::RemoveAt`).
pub fn fn_00992a30(e: &mut Engine, this: Ptr, actor: Ptr) {
    let map = e.mem.u32(this.addr().wrapping_add(0x10));
    e.call(0x0040_5430, &args![map, actor]);
}

// Translated from 009928c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether the word at `+0xa8` has any bit of
/// `mask`.
pub fn fn_009928c0(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr().wrapping_add(0xa8)) & mask != 0
}

// Translated from 009928a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): `009928c0` with the mask 0x20.
pub fn fn_009928a0(e: &mut Engine, this: Ptr) -> bool {
    fn_009928c0(e, this, 0x20)
}

// Translated from 009928e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): `009928c0` with the mask 0x40.
pub fn fn_009928e0(e: &mut Engine, this: Ptr) -> bool {
    fn_009928c0(e, this, 0x40)
}

// Translated from 00992900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): `009928c0` with the mask 0x80.
pub fn fn_00992900(e: &mut Engine, this: Ptr) -> bool {
    fn_009928c0(e, this, 0x80)
}

// Translated from 009931a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): 1 when the signed byte at `+0x154` is
/// 1, else 0 (the whole word).
pub fn fn_009931a0(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u8(this.addr().wrapping_add(0x154)) as i8 == 1)
}

// Translated from 00993800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`, the word not read): `00991310(1)` then
/// `009911c0` on `this`.
pub fn fn_00993800(e: &mut Engine, this: Ptr, _unused_1: u32) {
    e.call(0x0099_1310, &args![this, 1u32]);
    e.call(0x0099_11c0, &args![this]);
}

// Translated from 00993790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): `0098eb30(word)` on every combat group,
/// then `009a5cd0(word)` on the object at `+0x14`.
pub fn fn_00993790(e: &mut Engine, this: Ptr, word: u32) {
    let mut i = 0u32;
    while i < e.call(GROUP_COUNT, &args![this]).u32() {
        let group = group_at(e, this, i);
        if group != 0 {
            e.call(0x0098_eb30, &args![group, word]);
        }
        i = i.wrapping_add(1);
    }
    let threat_map = e.mem.u32(this.addr().wrapping_add(0x14));
    e.call(0x009a_5cd0, &args![threat_map, word]);
}

// Translated from 00992ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x28`): counts the actors of the actor map
/// (`+0x10`, walked with `004b9ba0` and `GetNext`) within `radius` of the point (three words
/// by value): an actor passes when its slot `+0x22c(0)`, `00440da0` and `00440d80` all answer
/// false, and the squared radius (`float`) is above `004a7290` of the point minus its slot
/// `+0x4d8(.., height)` (`00439ef0`). Each counted actor is `actor` itself, a member of
/// `actor`'s group (slot `+0x3f8`), one whose mask has the bit of that group (`00559450`,
/// `__allshl`), or another; those four counts go to the non-null out words, the total is
/// answered.
pub fn fn_00992ba0(
    e: &mut Engine,
    this: Ptr,
    point_x: u32,
    point_y: u32,
    point_z: u32,
    radius: f32,
    height: f32,
    actor: Ptr,
    out_self: Ptr,
    out_group: Ptr,
    out_flagged: Ptr,
    out_other: Ptr,
) -> u32 {
    let me = this.addr();
    let mut total = 0u32;
    let mut own = 0u32;
    let mut same_group = 0u32;
    let mut flagged = 0u32;
    let mut other = 0u32;
    let group = e.vcall(actor.addr(), 0x3f8, &[]).u32();
    let radius_bits = radius.to_bits();
    let radius_squared = x87_store32(x87_mul(x87_load32(radius_bits), x87_m32(radius_bits)));
    let mut mask = 0u64;
    if group != 0 {
        let shift = e.call(GROUP_NUMBER, &args![group]).u32();
        mask = e.call(SHIFT_LEFT_64, &args![1u64, shift]).u64();
    }
    // The frame: position +0, key +4, value +8 (two words), the actor's point +0x10, the
    // difference +0x1c, and the point argument +0x28 (where the exe has it on its stack).
    e.with_stack(0x34, |e, frame| {
        let f = frame.addr();
        e.mem.set_u32(f + 0x28, point_x);
        e.mem.set_u32(f + 0x2c, point_y);
        e.mem.set_u32(f + 0x30, point_z);
        let map = e.mem.u32(me.wrapping_add(0x10));
        let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
        e.mem.set_u32(f, first);
        while e.mem.u32(f) != 0 {
            e.mem.set_u32(f + 4, 0);
            e.mem.set_u32(f + 8, 0);
            e.mem.set_u32(f + 0xc, 0);
            let map = Ptr::new(e.mem.u32(me.wrapping_add(0x10)));
            ni_t_map_base_actor_int64_get_next(
                e,
                map,
                Ptr::new(f),
                Ptr::new(f + 4),
                Ptr::new(f + 8),
            );
            let key = e.mem.u32(f + 4);
            if key == 0 {
                continue;
            }
            // The exe tests the key again here (null jumps straight to the measuring); it
            // cannot be null at this point.
            if key != 0
                && (e.vcall(key, 0x22c, &args![0u32]).bool()
                    || e.call(0x0044_0da0, &args![key]).bool()
                    || e.call(0x0044_0d80, &args![key]).bool())
            {
                continue;
            }
            e.vcall(
                key,
                0x4d8,
                &args![f + 0x10, x87_quiet_bits(height.to_bits())],
            );
            e.call(0x0043_9ef0, &args![f + 0x28, f + 0x1c, f + 0x10]);
            let distance = e.call(0x004a_7290, &args![f + 0x1c]).f64();
            if x87_load32(radius_squared) > distance {
                total = total.wrapping_add(1);
                if key == actor.addr() {
                    own = own.wrapping_add(1);
                } else if e.vcall(key, 0x3f8, &[]).u32() == group {
                    same_group = same_group.wrapping_add(1);
                } else {
                    let value = u64::from(e.mem.u32(f + 8)) | u64::from(e.mem.u32(f + 0xc)) << 32;
                    if value & mask != 0 {
                        flagged = flagged.wrapping_add(1);
                    } else {
                        other = other.wrapping_add(1);
                    }
                }
            }
        }
    });
    for (out, count) in [
        (out_self, own),
        (out_group, same_group),
        (out_flagged, flagged),
        (out_other, other),
    ] {
        if !out.is_null() {
            e.mem.set_u32(out.addr(), count);
        }
    }
    total
}

// Translated from 00992720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`): whether an explosion at `point` may go
/// ahead for `actor`. True without an explosion, when the setting `011cf3f0` is above the
/// explosion's `006a78f0`, or when the setting `011cea74` is above `006479a0` of the
/// explosion's radius (`00477900`), that value and the setting `011cf3f0`. Otherwise the
/// actors near the point are counted (`00992ba0` with that reach and `height`), and with the
/// actor's `008a7f80` answer: false when the actor itself is caught and `009928a0` refuses,
/// when its group is caught and `009928e0` refuses, or when others are caught, the actor's
/// object at `+0xa4` answers (slot `+8`, 3) at least the integer `0043d4d0(011ce3cc)` points at
/// and `00992900` refuses; true otherwise.
pub fn fn_00992720(
    e: &mut Engine,
    this: Ptr,
    actor: Ptr,
    explosion: Ptr,
    point: Ptr,
    height: f32,
) -> bool {
    if explosion.is_null() {
        return true;
    }
    let blast = x87_store32(e.call(0x0047_7900, &args![explosion]).f64());
    let inner = x87_store32(e.call(0x006a_78f0, &args![explosion]).f64());
    let limit = e.call(SETTING_VALUE, &args![SETTING_011CF3F0]).u32();
    if x87_load32(e.mem.u32(limit)) > x87_load32(inner) {
        return true;
    }
    let limit = e.call(SETTING_VALUE, &args![SETTING_011CF3F0]).u32();
    let limit_bits = x87_quiet_bits(e.mem.u32(limit));
    let reach = e
        .call(
            0x0064_79a0,
            &args![x87_quiet_bits(blast), x87_quiet_bits(inner), limit_bits],
        )
        .f64();
    let reach = x87_store32(reach);
    let floor = e.call(SETTING_VALUE, &args![SETTING_011CEA74]).u32();
    if x87_load32(e.mem.u32(floor)) > x87_load32(reach) {
        return true;
    }
    let p = point.addr();
    let (x, y, z) = (e.mem.u32(p), e.mem.u32(p + 4), e.mem.u32(p + 8));
    let (own, same_group, other) = e.with_stack(0x10, |e, counts| {
        let c = counts.addr();
        fn_00992ba0(
            e,
            this,
            x,
            y,
            z,
            f32::from_bits(x87_quiet_bits(reach)),
            f32::from_bits(x87_quiet_bits(height.to_bits())),
            actor,
            Ptr::new(c),
            Ptr::new(c + 4),
            Ptr::new(c + 8),
            Ptr::new(c + 0xc),
        );
        (e.mem.u32(c), e.mem.u32(c + 4), e.mem.u32(c + 0xc))
    });
    let reaction = Ptr::new(e.call(0x008a_7f80, &args![actor]).u32());
    if own != 0 && !fn_009928a0(e, reaction) {
        return false;
    }
    if same_group != 0 && !fn_009928e0(e, reaction) {
        return false;
    }
    if other != 0 {
        let holder = actor.addr().wrapping_add(0xa4);
        let caught = e.vcall(holder, 8, &args![3u32]).i32();
        let limit = e.call(0x0043_d4d0, &args![LIMIT_OBJECT_011CE3CC]).u32();
        if caught >= e.mem.u32(limit) as i32 && !fn_00992900(e, reaction) {
            return false;
        }
    }
    true
}

// Translated from 00993440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00993440 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `CombatManager::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): reads the word at `+0x34`
/// (`00864980`, 4 bytes); a counted block (`00864a60`) of groups, each an index and a second
/// word (4 bytes each), a new 0x15C-byte group (`00985d10` with 0) given the index
/// (`00537e90`), the second word (`006ecd40`) and its own `LoadGame` (`0098e3d0`), and put at
/// the index (`SetAtGrow`); the threat map's load (`009a5a70`, object at `+0x14`); a counted
/// block of form ids (`008648e0`) each put in the actor map with a zero mask (`SetAt`); the
/// time stamp at `+0x20` (`009a5ff0`) and the timers at `+0x24` and `+0x2c` (`009a5f20`).
pub fn combat_manager_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let me = this.addr();
    e.call(LOAD_DATA, &args![buffer, me.wrapping_add(0x34), 4u32]);
    let groups = e.call(LOAD_COUNT, &args![buffer]).u32();
    // The frame: the index +0, the second word +4, the group +8.
    e.with_stack(0xc, |e, frame| {
        let f = frame.addr();
        let mut i = 0u32;
        while i < groups {
            e.mem.set_u32(f, 0);
            e.call(LOAD_DATA, &args![buffer, f, 4u32]);
            e.mem.set_u32(f + 4, 0);
            e.call(LOAD_DATA, &args![buffer, f + 4, 4u32]);
            let block = e.call(OPERATOR_NEW, &args![0x15cu32]).u32();
            let group = if block != 0 {
                e.call(GROUP_CONSTRUCT, &args![block, 0u32]).u32()
            } else {
                0
            };
            e.mem.set_u32(f + 8, group);
            let index = e.mem.u32(f);
            e.call(GROUP_SET_INDEX, &args![group, index]);
            let second = e.mem.u32(f + 4);
            e.call(0x006e_cd40, &args![group, second]);
            e.call(0x0098_e3d0, &args![group, buffer]);
            let index = e.mem.u32(f);
            e.call(ARRAY_SET_AT_GROW, &args![this, index, f + 8]);
            i = i.wrapping_add(1);
        }
    });
    let threat_map = e.mem.u32(me.wrapping_add(0x14));
    e.call(0x009a_5a70, &args![threat_map, buffer]);
    let actors = e.call(LOAD_COUNT, &args![buffer]).u32();
    let mut i = 0u32;
    while i < actors {
        let id = e.with_stack(4, |e, word| {
            e.call(LOAD_FORM_ID, &args![buffer, word]);
            e.mem.u32(word.addr())
        });
        let map = Ptr::new(e.mem.u32(me.wrapping_add(0x10)));
        ni_t_map_base_actor_int64_set_at(e, map, id, 0, 0);
        i = i.wrapping_add(1);
    }
    e.call(0x009a_5ff0, &args![me.wrapping_add(0x20), buffer]);
    e.call(0x009a_5f20, &args![me.wrapping_add(0x24), buffer]);
    e.call(0x009a_5f20, &args![me.wrapping_add(0x2c), buffer]);
}

// Translated from 00993600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00993600 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `CombatManager::InitLoadGame` (Xbox PDB; `thiscall`, `RET 4`): replaces the actor map
/// (`+0x10`) by a new one (0x10 bytes, `00993820` with 0x25 buckets; null when the allocation
/// fails) and moves every entry of the old map whose key (form ids after a load) resolves
/// (`004839c0`, then `__RTDynamicCast(.., 0, 01183028, 011846d4, 0)`) into it (`SetAt`); the
/// old map is emptied (`00438af0`) and deleted (its slot `+0` with 1); then every combat
/// group's `InitLoadGame` (`0098e990`) and the threat map's (`009a5c40`, object at `+0x14`).
pub fn combat_manager_init_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let me = this.addr();
    let old = e.mem.u32(me.wrapping_add(0x10));
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let map = if block != 0 {
        fn_00993820(e, Ptr::new(block), 0x25).addr()
    } else {
        0
    };
    e.mem.set_u32(me.wrapping_add(0x10), map);
    // The frame: position +0, key +4, value +8 (two words).
    e.with_stack(0x10, |e, frame| {
        let f = frame.addr();
        let first = e.call(MAP_FIRST_POSITION, &args![old]).u32();
        e.mem.set_u32(f, first);
        while e.mem.u32(f) != 0 {
            ni_t_map_base_actor_int64_get_next(
                e,
                Ptr::new(old),
                Ptr::new(f),
                Ptr::new(f + 4),
                Ptr::new(f + 8),
            );
            let key = e.mem.u32(f + 4);
            let actor = if key != 0 {
                let object = e.call(0x0048_39c0, &args![key]).u32();
                e.call(
                    0x00ec_43fb,
                    &args![object, 0u32, 0x0118_3028u32, 0x0118_46d4u32, 0u32],
                )
                .u32()
            } else {
                0
            };
            e.mem.set_u32(f + 4, actor);
            if actor != 0 {
                let low = e.mem.u32(f + 8);
                let high = e.mem.u32(f + 0xc);
                let map = Ptr::new(e.mem.u32(me.wrapping_add(0x10)));
                ni_t_map_base_actor_int64_set_at(e, map, actor, low, high);
            }
        }
    });
    e.call(MAP_REMOVE_ALL, &args![old]);
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let mut i = 0u32;
    while i < e.call(GROUP_COUNT, &args![this]).u32() {
        let group = group_at(e, this, i);
        if group != 0 {
            e.call(0x0098_e990, &args![group, buffer]);
        }
        i = i.wrapping_add(1);
    }
    let threat_map = e.mem.u32(me.wrapping_add(0x14));
    e.call(0x009a_5c40, &args![threat_map, buffer]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0099_3bf0, fn_00993bf0(Ptr) -> Ptr),
        entry!(
            0x0099_3cf0,
            fn_00993cf0(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(
            0x0099_3960,
            fn_00993960(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
        entry!(0x0099_38b0, fn_009938b0(Ptr<NiTArray>) -> u16),
        entry!(0x0099_3880, fn_00993880(Ptr<NiTArray>)),
        entry!(0x0099_3990, fn_00993990(Ptr, u32) -> Ptr),
        entry!(0x0099_3820, fn_00993820(Ptr, u32) -> Ptr),
        entry!(0x0099_3bc0, fn_00993bc0(Ptr)),
        entry!(0x0099_3b60, fn_00993b60(Ptr)),
        entry!(
            0x0099_3850,
            ni_t_map_actor_int64_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0099_2920, fn_00992920(Ptr, Ptr)),
        entry!(
            0x0099_31c0,
            combat_manager_get_combatant_count(Ptr, Ptr, Ptr) -> u32
        ),
        entry!(0x0099_32d0, combat_manager_save_game(Ptr, Ptr)),
        entry!(0x0099_2d90, fn_00992d90(Ptr) -> bool),
        entry!(
            0x0084_4310,
            ni_t_map_base_actor_int64_set_at(Ptr<NiTPointerMap>, u32, u32, u32)
        ),
        entry!(
            0x0099_3a00,
            fn_00993a00(Ptr<NiTPointerMap>, u32, Ptr) -> bool
        ),
        entry!(
            0x0099_3a80,
            ni_t_map_base_actor_int64_get_next(Ptr<NiTPointerMap>, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x0099_3b30,
            ni_t_map_base_actor_int64_set_value(Ptr, Ptr, u32, u32, u32)
        ),
        entry!(0x0099_3c10, ni_t_map_actor_int64_delete_item(Ptr, Ptr)),
        entry!(0x0099_3cd0, fn_00993cd0(Ptr) -> u32),
        entry!(
            0x0099_3c40,
            fn_00993c40(Ptr<NiTArray>, u32) -> Ptr<NiTArray>
        ),
        entry!(0x0099_3c70, fn_00993c70(Ptr, u32) -> Ptr),
        entry!(
            0x0099_3ca0,
            ni_t_map_base_actor_int64_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0099_38d0, fn_009938d0(Ptr<NiTArray>, Ptr) -> u32),
        entry!(0x0099_1e80, combat_manager_create_combat_group(Ptr) -> u32),
        entry!(
            0x0099_1f80,
            combat_manager_split_player_combat_group(Ptr, Ptr) -> u32
        ),
        entry!(
            0x0099_2110,
            combat_manager_add_combatant(Ptr, Ptr, Ptr, u32, u32) -> u32
        ),
        entry!(
            0x0099_22b0,
            combat_manager_add_group_member(Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x0099_23d0,
            combat_manager_add_pretend_combatant(Ptr, Ptr, u32) -> u32
        ),
        entry!(0x0099_2480, fn_00992480(Ptr, Ptr, Ptr)),
        entry!(
            0x0099_2530,
            combat_manager_get_want_to_help_group(Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x0099_2640, fn_00992640(Ptr, Ptr, Ptr) -> bool),
        entry!(0x0099_26c0, fn_009926c0(Ptr, Ptr, Ptr)),
        entry!(0x0099_26e0, fn_009926e0(Ptr, Ptr, Ptr)),
        entry!(0x0099_2700, fn_00992700(Ptr, u32, Ptr)),
        entry!(0x0099_29e0, fn_009929e0(Ptr, Ptr)),
        entry!(0x0099_2a50, combat_manager_target_localactor(Ptr, Ptr, Ptr)),
        entry!(0x0099_2b00, fn_00992b00(Ptr, Ptr, Ptr)),
        entry!(0x0099_2a30, fn_00992a30(Ptr, Ptr)),
        entry!(0x0099_28c0, fn_009928c0(Ptr, u32) -> bool),
        entry!(0x0099_28a0, fn_009928a0(Ptr) -> bool),
        entry!(0x0099_28e0, fn_009928e0(Ptr) -> bool),
        entry!(0x0099_2900, fn_00992900(Ptr) -> bool),
        entry!(0x0099_31a0, fn_009931a0(Ptr) -> u32),
        entry!(0x0099_3800, fn_00993800(Ptr, u32)),
        entry!(0x0099_3790, fn_00993790(Ptr, u32)),
        entry!(
            0x0099_2ba0,
            fn_00992ba0(Ptr, u32, u32, u32, f32, f32, Ptr, Ptr, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(0x0099_2720, fn_00992720(Ptr, Ptr, Ptr, Ptr, f32) -> bool),
        entry!(0x0099_3440, combat_manager_load_game(Ptr, Ptr)),
        entry!(0x0099_3600, combat_manager_init_load_game(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00993bf0_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00993CD0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0099_3bf0, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00993CD0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
}

#[cfg(test)]
mod b015_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    #[test]
    fn array_constructors_set_counts_and_allocate_only_when_sized() {
        let mut e = Engine::new();
        e.register(ARRAY_ALLOCATE, |_, a| (0x0070_0000 + a[0]).into_ret());
        let a: Ptr<NiTArray> = e.new_object();
        e.mem.write(a.addr(), &[0xcc; 0x10]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_3cf0, &args![a, 5u32, 3u32]).u32(), a.addr());
        assert_eq!(e.mem.u32(a.addr()), COMBAT_GROUP_ARRAY_VTABLE);
        assert_eq!(e.get(a, NiTArray::m_usMaxSize), 5);
        assert_eq!(e.get(a, NiTArray::m_usGrowBy), 3);
        assert_eq!(e.get(a, NiTArray::m_usSize), 0);
        assert_eq!(e.get(a, NiTArray::m_usESize), 0);
        assert_eq!(e.get(a, NiTArray::m_pBase), 0x0070_0005);
        e.call(0x0099_3960, &args![a, 0u32, 1u32]);
        assert_eq!(e.mem.u32(a.addr()), COMBAT_GROUP_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(e.get(a, NiTArray::m_pBase), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_ALLOCATE), vec![vec![5]]);
    }

    #[test]
    fn array_effective_size_and_destructor() {
        let mut e = Engine::new();
        noop(&mut e, &[ARRAY_FREE]);
        let a: Ptr<NiTArray> = e.new_object();
        e.set(a, NiTArray::m_usESize, 0xbeef);
        e.set(a, NiTArray::m_pBase, 0x1234);
        assert_eq!(e.call(0x0099_38b0, &args![a]).u32(), 0xbeef);
        e.call_log = Some(vec![]);
        e.call(0x0099_3880, &args![a]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(a.addr()), COMBAT_GROUP_ARRAY_VTABLE);
        assert_eq!(calls_to(&log, ARRAY_FREE), vec![vec![0x1234]]);
    }

    #[test]
    fn map_constructors_allocate_and_zero_the_buckets() {
        let mut e = Engine::new();
        e.register(BUCKETS_ALLOCATE, |_, _| 0x0071_0000u32.into_ret());
        noop(&mut e, &[MEMSET]);
        let m = e.mem.alloc(0x10);
        e.mem.set_u32(m + 0xc, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_3820, &[m, 37]).u32(), m);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(m), ACTOR_MAP_VTABLE);
        assert_eq!(e.mem.u32(m + 4), 37);
        assert_eq!(e.mem.u32(m + 8), 0x0071_0000);
        assert_eq!(e.mem.u32(m + 0xc), 0);
        assert_eq!(calls_to(&log, BUCKETS_ALLOCATE), vec![vec![37 * 4]]);
        assert_eq!(calls_to(&log, MEMSET), vec![vec![0x0071_0000, 0, 37 * 4]]);
        e.call(0x0099_3990, &[m, 2]);
        assert_eq!(e.mem.u32(m), ACTOR_MAP_BASE_VTABLE);
    }

    #[test]
    fn map_destructors_empty_free_and_delete_on_flag() {
        let mut e = Engine::new();
        noop(&mut e, &[MAP_REMOVE_ALL, BUCKETS_FREE, FREE]);
        let m = e.mem.alloc(0x10);
        e.mem.set_u32(m + 8, 0x0071_0000);
        for (flags, deletes) in [(0u32, false), (1, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0099_3850, &[m, flags]).u32(), m);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, MAP_REMOVE_ALL), vec![vec![m], vec![m]]);
            assert_eq!(calls_to(&log, BUCKETS_FREE), vec![vec![0x0071_0000]]);
            assert_eq!(calls_to(&log, FREE).len(), usize::from(deletes));
            assert_eq!(e.mem.u32(m), ACTOR_MAP_BASE_VTABLE);
        }
    }

    /// A manager with `slots` group slots (all empty; see `set_group`).
    fn manager(e: &mut Engine, slots: u32) -> Ptr {
        let this = Ptr::new(e.mem.alloc(0x40));
        let table = e.mem.alloc(4 * slots + 4);
        e.mem.set_u32(this.addr() + 0x38, table);
        e.mem.set_u32(this.addr() + 0x3c, slots);
        e.register(GROUP_COUNT, |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        e.register(GROUP_SLOT, |e, a| {
            (e.mem.u32(a[0] + 0x38) + 4 * a[1]).into_ret()
        });
        this
    }

    fn set_group(e: &mut Engine, this: Ptr, i: u32, group: u32) {
        let table = e.mem.u32(this.addr() + 0x38);
        e.mem.set_u32(table + 4 * i, group);
    }

    fn object_test(e: &mut Engine, answer: bool) {
        e.map(OBJECT_POINTER_011DDF38, 4);
        e.set_global(OBJECT_POINTER_011DDF38, 0x0055_0000u32);
        if answer {
            e.register(OBJECT_TEST, |_, _| 1u32.into_ret());
        } else {
            e.register(OBJECT_TEST, |_, _| Ret::default());
        }
    }

    #[test]
    fn actor_update_reaches_every_group_and_the_maps() {
        let mut e = Engine::new();
        object_test(&mut e, false);
        let this = manager(&mut e, 3);
        set_group(&mut e, this, 0, 0x0060_0000);
        set_group(&mut e, this, 2, 0x0060_0100);
        e.mem.set_u32(this.addr() + 0x1c, 0x0061_0000);
        e.mem.set_u32(this.addr() + 0x14, 0x0062_0000);
        noop(
            &mut e,
            &[0x0098_7820, 0x0099_2a30, 0x009a_41e0, 0x009a_51e0],
        );
        let actor = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0x0090_1000; 0x41]);
        e.mem.set_u32(actor, 0x0090_0000);
        for answer in [1u32, 0] {
            e.register_double(0x0090_1000, move |_, _| answer.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0099_2920, &[this.addr(), actor]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, 0x0098_7820),
                vec![vec![0x0060_0000, actor], vec![0x0060_0100, actor]]
            );
            let chosen = if answer != 0 {
                vec![vec![this.addr(), actor]]
            } else {
                vec![]
            };
            assert_eq!(calls_to(&log, 0x0099_2a30), chosen);
            assert_eq!(calls_to(&log, 0x009a_41e0).len(), usize::from(answer != 0));
            assert_eq!(calls_to(&log, 0x009a_51e0), vec![vec![0x0062_0000, actor]]);
        }
        // The object test stops everything.
        object_test(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0099_2920, &[this.addr(), actor]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x009a_51e0).is_empty());
    }

    /// Setting objects at their addresses, the value at `+4`.
    fn settings(e: &mut Engine, values: &[(u32, f32)]) {
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        for (setting, value) in values {
            e.map(*setting, 8);
            e.mem.set_f32(*setting + 4, *value);
        }
    }

    /// The player; its slot `+0x3f8` answers `own_group`.
    fn player(e: &mut Engine, own_group: u32) -> u32 {
        let p = e.mem.alloc(8);
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(p, table);
        e.mem.set_u32(table + 0x3f8, 0x0090_2000);
        e.register_double(0x0090_2000, move |_, _| own_group.into_ret());
        e.map(PLAYER_POINTER, 4);
        e.set_global(PLAYER_POINTER, p);
        p
    }

    /// A group: its member count at `+0`, "targets the given actor" at `+4`,
    /// the time since its stamp at `+8`.
    fn group(e: &mut Engine, members: u32, targets: bool, since: f32) -> u32 {
        let g = e.mem.alloc(0x10);
        e.mem.set_u32(g, members);
        e.mem.set_u32(g + 4, targets as u32);
        e.mem.set_f32(g + 8, since);
        g
    }

    fn group_doubles(e: &mut Engine) {
        e.register(GROUP_MEMBER_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GROUP_IS_TARGET, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(GROUP_TIME_STAMP, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            Ret::default()
        });
        e.register(TIME_SINCE, |e, a| {
            let g = e.mem.u32(a[0]);
            Ret {
                st0: f64::from(e.mem.f32(g + 8)),
                ..Ret::default()
            }
        });
    }

    #[test]
    fn combatant_count_sums_targeting_groups_less_the_players_own() {
        let mut e = Engine::new();
        group_doubles(&mut e);
        settings(&mut e, &[(SETTING_011CE528, 5.0)]);
        let a = group(&mut e, 3, true, 10.0);
        let b = group(&mut e, 4, true, 1.0);
        let c = group(&mut e, 7, false, 10.0);
        player(&mut e, b);
        let this = manager(&mut e, 4);
        set_group(&mut e, this, 0, a);
        set_group(&mut e, this, 2, b);
        set_group(&mut e, this, 3, c);
        let recent = e.mem.alloc(4);
        e.mem.set_u32(recent, 99);
        let target = e.mem.alloc(4);
        // With a target: a (3) + b (4 - 1); recent only a (10 > 5).
        assert_eq!(e.call(0x0099_31c0, &[this.addr(), target, recent]).u32(), 6);
        assert_eq!(e.mem.u32(recent), 3);
        // Without a target every group counts and `recent` stays cleared.
        assert_eq!(e.call(0x0099_31c0, &[this.addr(), 0, recent]).u32(), 13);
        assert_eq!(e.mem.u32(recent), 0);
        assert_eq!(e.call(0x0099_31c0, &[this.addr(), 0, 0]).u32(), 13);
    }

    #[test]
    fn save_game_writes_groups_keys_and_timers_in_order() {
        let mut e = Engine::new();
        let this = manager(&mut e, 3);
        set_group(&mut e, this, 0, 0x0060_0000);
        set_group(&mut e, this, 2, 0x0060_0100);
        e.mem.set_u32(this.addr() + 0x14, 0x0062_0000);
        e.mem.set_u32(this.addr() + 0x10, 0x0063_0000);
        let buffer = 0x0064_0000u32;
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let w = written.clone();
        e.register_double(0x0086_5e50, move |e, a| {
            w.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.register(0x0086_5f20, |_, _| 0x77u32.into_ret());
        e.register(0x0055_9450, |_, a| (a[0] + 1).into_ret());
        e.register(0x0072_6070, |_, a| (a[0] + 2).into_ret());
        noop(
            &mut e,
            &[
                0x0098_de90,
                0x0086_5ff0,
                0x009a_59e0,
                0x0086_5df0,
                0x009a_5f90,
                0x009a_5eb0,
            ],
        );
        // The actor map walk: two keys, 0x15 then 0x14.
        e.register(0x004b_9ba0, |_, _| 2u32.into_ret());
        e.register(0x0099_3a80, |e, a| {
            let left = e.mem.u32(a[1]);
            e.mem.set_u32(a[2], 0x13 + left);
            e.mem.set_u32(a[1], left - 1);
            Ret::default()
        });
        e.mem.set_u32(this.addr() + 0x34, 0xabcd);
        e.call_log = Some(vec![]);
        e.call(0x0099_32d0, &[this.addr(), buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            *written.borrow(),
            vec![0xabcd, 0x0060_0001, 0x0060_0002, 0x0060_0101, 0x0060_0102]
        );
        assert_eq!(
            calls_to(&log, 0x0098_de90),
            vec![vec![0x0060_0000, buffer], vec![0x0060_0100, buffer]]
        );
        assert_eq!(
            calls_to(&log, 0x0086_5ff0),
            vec![vec![buffer, 2, 0x77], vec![buffer, 2, 0x77]]
        );
        assert_eq!(
            calls_to(&log, 0x0086_5df0),
            vec![vec![buffer, 0x15, 0], vec![buffer, 0x14, 0]]
        );
        assert_eq!(calls_to(&log, 0x009a_59e0), vec![vec![0x0062_0000, buffer]]);
        assert_eq!(
            calls_to(&log, 0x009a_5f90),
            vec![vec![this.addr() + 0x20, buffer]]
        );
        assert_eq!(
            calls_to(&log, 0x009a_5eb0),
            vec![
                vec![this.addr() + 0x24, buffer],
                vec![this.addr() + 0x2c, buffer]
            ]
        );
    }

    /// The engine for `00992d90` with `slots` empty group slots: the timers
    /// pass, the object test fails, the stamp at `+0x20` holds `stamp`,
    /// strength 10.
    fn combat_state(slots: u32, stamp: f32) -> (Engine, Ptr) {
        let mut e = Engine::new();
        object_test(&mut e, false);
        e.map(UNSET_MAGNITUDE, 4);
        e.mem.set_f32(UNSET_MAGNITUDE, f32::MAX);
        e.map(0x0101_2054, 4);
        e.mem.set_f32(0x0101_2054, -1.0);
        e.map(0x0101_2060, 8);
        e.set_global(0x0101_2060, 0.0f64);
        e.register(READ_FLOAT, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.register(TIMER_TEST, |_, _| 1u32.into_ret());
        e.register(0x008a_cbe0, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        e.register(CURRENT_TIME, |_, _| Ret {
            st0: 42.0,
            ..Ret::default()
        });
        e.register(STAMP_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        noop(&mut e, &[TIMER_SET, TIMER_RESET, 0x0097_ef30, 0x0098_a2e0]);
        e.register(0x005a_4320, |_, _| 1u32.into_ret());
        group_doubles(&mut e);
        settings(
            &mut e,
            &[
                (SETTING_011CEEC0, 2.0),
                (SETTING_011CE528, 5.0),
                (SETTING_011CE04C, 3.0),
                (SETTING_011CE454, 100.0),
                (SETTING_011CFE5C, 200.0),
                (SETTING_011CE72C, 0.5),
                (SETTING_011CFC14, 7.0),
            ],
        );
        player(&mut e, 0);
        let this = manager(&mut e, slots);
        e.mem.set_f32(this.addr() + 0x20, stamp);
        (e, this)
    }

    #[test]
    fn combat_state_skips_the_search_when_the_timer_has_not_run_out() {
        let (mut e, this) = combat_state(0, 1.0);
        e.register(TIMER_TEST, |_, _| Ret::default());
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
        let (mut e, this) = combat_state(0, -f32::MAX);
        e.register(TIMER_TEST, |_, _| Ret::default());
        assert!(!e.call(0x0099_2d90, &[this.addr()]).bool());
        // A NaN differs from "unset".
        let (mut e, this) = combat_state(0, f32::NAN);
        object_test(&mut e, true);
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
    }

    #[test]
    fn a_recent_target_group_with_weak_members_sets_the_state() {
        // Not set; the group targets the player and was stamped 1 < 5 ago;
        // members: one that passes with 2.0, one without a process.
        let (mut e, this) = combat_state(1, -f32::MAX);
        let g = group(&mut e, 2, true, 1.0);
        set_group(&mut e, this, 0, g);
        let member_table = e.mem.alloc(0x430);
        e.mem.set_u32(member_table + 0x428, 0x0090_3000);
        let with_process = e.mem.alloc(4);
        e.mem.set_u32(with_process, member_table);
        let without = e.mem.alloc(4);
        e.mem.set_u32(without, member_table);
        e.register_double(GROUP_MEMBER, move |_, a| {
            [with_process, without][a[1] as usize].into_ret()
        });
        e.register(0x0096_70f0, |_, _| 1u32.into_ret());
        e.register_double(0x0090_3000, move |_, a| {
            let process = if a[0] == with_process {
                0x0066_0000u32
            } else {
                0
            };
            process.into_ret()
        });
        e.register(0x0058_6150, |_, a| a[0].into_ret());
        e.register(0x0045_3700, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        // strength / total = 10 / 2 = 5, not below the setting 3: no reason.
        assert!(!e.call(0x0099_2d90, &[this.addr()]).bool());
        // With the setting at 6 it is.
        e.mem.set_f32(SETTING_011CE04C + 4, 6.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(this.addr() + 0x20), 42.0);
        assert_eq!(calls_to(&log, TIMER_RESET), vec![vec![this.addr() + 0x24]]);
        assert_eq!(
            calls_to(&log, TIMER_SET),
            vec![vec![this.addr() + 0x2c, 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn a_nearby_group_keeps_the_state_set() {
        let (mut e, this) = combat_state(1, 1.0);
        let g = group(&mut e, 0, false, 0.0);
        set_group(&mut e, this, 0, g);
        e.register(0x0099_31a0, |_, _| 1u32.into_ret());
        e.register(0x0098_9880, |_, a| {
            u32::from(f32::from_bits(a[2]) == 100.0).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
        let log = e.call_log.take().unwrap();
        // Still set, so nothing changes.
        assert!(calls_to(&log, TIMER_RESET).is_empty());
        let player = e.global::<u32>(PLAYER_POINTER);
        assert_eq!(
            calls_to(&log, 0x0098_9880),
            vec![vec![g, player, 100.0f32.to_bits()]]
        );
    }

    #[test]
    fn losing_the_reason_restarts_or_clears_the_stamp() {
        // Set, but the only slot is empty: no reason.
        let (mut e, this) = combat_state(1, 1.0);
        e.register(TIMER_RUNNING, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, TIMER_SET)[1],
            vec![this.addr() + 0x24, 7.0f32.to_bits()]
        );
        // With the timer running and passing: the stamp becomes "unset".
        e.register(TIMER_RUNNING, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0099_2d90, &[this.addr()]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(this.addr() + 0x20), -f32::MAX);
        assert_eq!(calls_to(&log, TIMER_RESET), vec![vec![this.addr() + 0x24]]);
    }

    #[test]
    fn a_far_group_adds_its_strength_once_a_member_was_seen() {
        let (mut e, this) = combat_state(2, -f32::MAX);
        let g = group(&mut e, 1, false, 0.0);
        set_group(&mut e, this, 0, g);
        set_group(&mut e, this, 1, g);
        e.register(0x0099_31a0, |_, _| 1u32.into_ret());
        e.register(0x0098_9880, |_, a| {
            u32::from(f32::from_bits(a[2]) == 200.0).into_ret()
        });
        e.register(GROUP_MEMBER, |_, _| 0x0067_0000u32.into_ret());
        e.register(0x0096_70f0, |_, _| 1u32.into_ret());
        e.register(0x0098_cb30, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        // 3 is not above 10 * 0.5; 6 (second group) is.
        e.call_log = Some(vec![]);
        assert!(e.call(0x0099_2d90, &[this.addr()]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0098_cb30), vec![vec![g, 0], vec![g, 0]]);
        // The member search runs only until one was seen.
        assert_eq!(calls_to(&log, GROUP_MEMBER).len(), 1);
    }
}

#[cfg(test)]
mod n023_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    /// The vtable every test object has; slot `offset` calls [`slot`]`(offset)`.
    const VTABLE: u32 = 0x0e00_0000;

    const fn slot(offset: u32) -> u32 {
        0x0f00_0000 + offset
    }

    /// An object with [`VTABLE`] and `words` from `+4` on.
    fn object(e: &mut Engine, words: &[u32]) -> u32 {
        if !e.mem.is_mapped(VTABLE) {
            let slots: Vec<u32> = (0..0x140).map(|i| slot(4 * i)).collect();
            e.put_vtable(VTABLE, &slots);
        }
        let o = e.mem.alloc(4 + 4 * words.len() as u32);
        e.mem.set_u32(o, VTABLE);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(o + 4 + 4 * i as u32, *w);
        }
        o
    }

    /// The vtable of the test maps: hash `key % buckets`, equal keys, the real `SetValue`, a
    /// zeroed 0x10-byte item.
    fn map_vtable(e: &mut Engine, at: u32) {
        e.put_vtable(
            at,
            &[
                slot(0x800),
                slot(0x804),
                slot(0x808),
                0x0099_3b30,
                0,
                slot(0x814),
            ],
        );
        e.register(slot(0x804), |e, a| (a[1] % e.mem.u32(a[0] + 4)).into_ret());
        e.register(slot(0x808), |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(slot(0x814), |e, _| e.mem.alloc(0x10).into_ret());
    }

    const MAP_VTABLE: u32 = 0x0e01_0000;

    fn actor_map(e: &mut Engine, buckets: u32) -> u32 {
        map_vtable(e, MAP_VTABLE);
        let map = e.mem.alloc(0x10);
        let table = e.mem.alloc(4 * buckets);
        e.mem.set_u32(map, MAP_VTABLE);
        e.mem.set_u32(map + 4, buckets);
        e.mem.set_u32(map + 8, table);
        map
    }

    fn map_value(e: &mut Engine, map: u32, key: u32) -> Option<u64> {
        e.with_stack(8, |e, v| {
            if fn_00993a00(e, Ptr::new(map), key, v) {
                Some(e.mem.u64(v.addr()))
            } else {
                None
            }
        })
    }

    /// A manager whose actor map (`+0x10`) is a fresh map.
    fn manager(e: &mut Engine) -> u32 {
        let m = e.mem.alloc(0x40);
        let map = actor_map(e, 7);
        e.mem.set_u32(m + 0x10, map);
        m
    }

    #[test]
    fn set_at_adds_new_keys_and_updates_old_ones() {
        let mut e = Engine::new();
        let map = actor_map(&mut e, 3);
        e.call(0x0084_4310, &args![map, 4u32, 0x11u32, 0x22u32]);
        e.call(0x0084_4310, &args![map, 7u32, 0x33u32, 0x44u32]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        let head = e.mem.u32(e.mem.u32(map + 8) + 4);
        assert_eq!(e.mem.u32(head + 4), 7);
        let second = e.mem.u32(head);
        assert_eq!(
            (
                e.mem.u32(second + 4),
                e.mem.u32(second + 8),
                e.mem.u32(second + 0xc)
            ),
            (4, 0x11, 0x22)
        );
        assert_eq!(e.mem.u32(second), 0);
        e.call(0x0084_4310, &args![map, 4u32, 0x55u32, 0x66u32]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        assert_eq!(
            (e.mem.u32(second + 8), e.mem.u32(second + 0xc)),
            (0x55, 0x66)
        );
    }

    #[test]
    fn get_at_copies_the_value_of_a_present_key() {
        let mut e = Engine::new();
        let map = actor_map(&mut e, 5);
        e.call(0x0084_4310, &args![map, 6u32, 1u32, 2u32]);
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0xaa);
        assert!(!e.call(0x0099_3a00, &args![map, 11u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0xaa);
        assert!(e.call(0x0099_3a00, &args![map, 6u32, out]).bool());
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (1, 2));
    }

    #[test]
    fn get_next_walks_chains_then_later_buckets() {
        let mut e = Engine::new();
        let map = actor_map(&mut e, 5);
        for (k, v) in [(1u32, 10u32), (3, 30), (8, 80)] {
            e.call(0x0084_4310, &args![map, k, v, 0u32]);
        }
        let table = e.mem.u32(map + 8);
        let position = e.mem.alloc(4);
        let first = e.mem.u32(table + 4);
        e.mem.set_u32(position, first);
        let key = e.mem.alloc(4);
        let value = e.mem.alloc(8);
        let mut seen = vec![];
        while e.mem.u32(position) != 0 {
            e.call(0x0099_3a80, &args![map, position, key, value]);
            seen.push((e.mem.u32(key), e.mem.u32(value)));
        }
        assert_eq!(seen, vec![(1, 10), (8, 80), (3, 30)]);
    }

    #[test]
    fn set_value_fills_the_item() {
        let mut e = Engine::new();
        let item = e.mem.alloc(0x10);
        e.call(0x0099_3b30, &args![0u32, item, 5u32, 6u32, 7u32]);
        assert_eq!(
            e.mem.bytes(item + 4, 12),
            [5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0]
        );
    }

    #[test]
    fn delete_item_clears_the_value_and_frees_through_the_allocator() {
        let mut e = Engine::new();
        noop(&mut e, &[0x006b_8310]);
        let item = e.mem.alloc(0x10);
        e.mem.write(item, &[0xff; 0x10]);
        e.call_log = Some(vec![]);
        e.call(0x0099_3c10, &args![0x5000u32, item]);
        assert_eq!(e.mem.u64(item + 8), 0);
        assert_eq!(e.mem.u32(item + 4), 0xffff_ffff);
        assert_eq!(calls(&e, 0x006b_8310), vec![vec![0x500c, item]]);
    }

    #[test]
    fn new_items_are_sixteen_bytes() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (0x7000 + a[0]).into_ret());
        assert_eq!(e.call(0x0099_3cd0, &args![0x1234u32]).u32(), 0x7010);
    }

    #[test]
    fn the_array_deleting_destructor_frees_on_bit_0() {
        let mut e = Engine::new();
        noop(&mut e, &[ARRAY_FREE, FREE]);
        let a: Ptr<NiTArray> = e.new_object();
        for (flags, frees) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0099_3c40, &args![a, flags]).u32(), a.addr());
            assert_eq!(calls(&e, ARRAY_FREE).len(), 1);
            assert_eq!(calls(&e, FREE).len(), frees);
        }
        assert_eq!(e.mem.u32(a.addr()), COMBAT_GROUP_ARRAY_VTABLE);
    }

    #[test]
    fn the_00991130_deleting_destructor_frees_on_bit_0() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0099_1130, FREE]);
        for (flags, frees) in [(2u32, 0usize), (3, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0099_3c70, &args![0x5000u32, flags]).u32(), 0x5000);
            assert_eq!(calls(&e, 0x0099_1130), vec![vec![0x5000]]);
            assert_eq!(calls(&e, FREE).len(), frees);
        }
    }

    #[test]
    fn the_map_base_deleting_destructor_frees_on_bit_0() {
        let mut e = Engine::new();
        noop(&mut e, &[MAP_REMOVE_ALL, BUCKETS_FREE, FREE]);
        let m = e.mem.alloc(0x10);
        e.mem.set_u32(m + 8, 0x0071_0000);
        for (flags, frees) in [(0u32, 0usize), (1, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0099_3ca0, &args![m, flags]).u32(), m);
            assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![m]]);
            assert_eq!(calls(&e, BUCKETS_FREE), vec![vec![0x0071_0000]]);
            assert_eq!(calls(&e, FREE).len(), frees);
            assert_eq!(e.mem.u32(m), ACTOR_MAP_BASE_VTABLE);
        }
    }

    #[test]
    fn add_fills_the_first_empty_slot_or_grows() {
        let mut e = Engine::new();
        e.register(ARRAY_SET_AT_GROW, |_, _| 7u32.into_ret());
        let a: Ptr<NiTArray> = e.new_object();
        let base = e.mem.alloc(0x10);
        e.set(a, NiTArray::m_pBase, base);
        e.set(a, NiTArray::m_usSize, 3);
        e.set(a, NiTArray::m_usESize, 2);
        e.mem.set_u32(base, 1);
        e.mem.set_u32(base + 8, 2);
        let element = e.mem.alloc(4);
        e.mem.set_u32(element, 9);
        assert_eq!(e.call(0x0099_38d0, &args![a, element]).u32(), 1);
        assert_eq!(e.mem.u32(base + 4), 9);
        assert_eq!(e.get(a, NiTArray::m_usESize), 3);
        e.mem.set_u32(element, 0);
        assert_eq!(e.call(0x0099_38d0, &args![a, element]).u32(), 0xffff_ffff);
        e.mem.set_u32(element, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_38d0, &args![a, element]).u32(), 7);
        assert_eq!(
            calls(&e, ARRAY_SET_AT_GROW),
            vec![vec![a.addr(), 3, element]]
        );
    }

    /// Doubles for `CreateCombatGroup`: the scope object, the allocator, the group
    /// constructor (answers the block, number stored at `+0x158`) and the index setter.
    fn group_creation(e: &mut Engine) {
        noop(e, &[SCOPE_BEGIN, SCOPE_END, GROUP_SET_INDEX, DEBUG_PRINT]);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(GROUP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x158, a[1]);
            a[0].into_ret()
        });
    }

    #[test]
    fn create_combat_group_numbers_and_stores_the_group() {
        let mut e = Engine::new();
        group_creation(&mut e);
        e.register(ARRAY_SET_AT_GROW, |_, _| 0x41u32.into_ret());
        let m = e.mem.alloc(0x40);
        let base = e.mem.alloc(8);
        e.mem.set_u32(m + 4, base);
        e.mem.set_u16(m + 0xa, 2);
        e.mem.set_u32(base, 0x1234);
        e.mem.set_u32(m + 0x34, 5);
        e.call_log = Some(vec![]);
        let group = e.call(0x0099_1e80, &args![m]).u32();
        assert_eq!(e.mem.u32(base + 4), group);
        assert_eq!(e.mem.u32(m + 0x34), 6);
        assert_eq!(calls(&e, GROUP_CONSTRUCT), vec![vec![group, 5]]);
        assert_eq!(calls(&e, GROUP_SET_INDEX), vec![vec![group, 1]]);
        let begin = calls(&e, SCOPE_BEGIN);
        assert_eq!(begin[0][1..], [0x2b, 1, SOURCE_FILE, 0x161]);
        assert_eq!(calls(&e, SCOPE_END), vec![vec![begin[0][0]]]);
        assert!(calls(&e, DEBUG_PRINT).is_empty());
        // No free slot left: the grown index 0x41 is reported.
        e.call_log = Some(vec![]);
        e.call(0x0099_1e80, &args![m]);
        assert_eq!(
            calls(&e, DEBUG_PRINT),
            vec![vec![TOO_MANY_GROUPS_FORMAT, 0x40, 0x41]]
        );
        // A failed allocation adds nothing (-1) and reports it too.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0099_1e80, &args![m]).u32(), 0);
        assert_eq!(e.mem.u32(m + 0x34), 7);
        assert_eq!(calls(&e, GROUP_SET_INDEX), vec![vec![0, 0xffff_ffff]]);
        assert_eq!(calls(&e, DEBUG_PRINT).len(), 1);
    }

    /// A group is a block of `[count, members...]`; `AddMember` appends (refusing `refused`),
    /// `009869a0` removes.
    fn group_lists(e: &mut Engine, refused: u32) {
        e.register(GROUP_MEMBER_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GROUP_MEMBER, |e, a| {
            e.mem.u32(a[0] + 4 + 4 * a[1]).into_ret()
        });
        e.register_double(GROUP_ADD_MEMBER, move |e, a| {
            if a[1] == refused {
                return Ret::default();
            }
            let n = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4 + 4 * n, a[1]);
            e.mem.set_u32(a[0], n + 1);
            1u32.into_ret()
        });
        e.register(GROUP_REMOVE, |e, a| {
            let n = e.mem.u32(a[0]);
            let members: Vec<u32> = (0..n).map(|i| e.mem.u32(a[0] + 4 + 4 * i)).collect();
            let kept: Vec<u32> = members.into_iter().filter(|m| *m != a[1]).collect();
            for (i, m) in kept.iter().enumerate() {
                e.mem.set_u32(a[0] + 4 + 4 * i as u32, *m);
            }
            e.mem.set_u32(a[0], kept.len() as u32);
            Ret::default()
        });
    }

    fn group_of(e: &mut Engine, members: &[u32]) -> u32 {
        let g = e.mem.alloc(0x40);
        e.mem.set_u32(g, members.len() as u32);
        for (i, m) in members.iter().enumerate() {
            e.mem.set_u32(g + 4 + 4 * i as u32, *m);
        }
        g
    }

    #[test]
    fn splitting_moves_the_player_and_teammates_to_a_new_group() {
        let mut e = Engine::new();
        group_creation(&mut e);
        e.register(ARRAY_SET_AT_GROW, |_, _| 1u32.into_ret());
        noop(&mut e, &[GROUP_ADD_TARGET, 0x0098_6740, slot(0x3fc)]);
        e.register(slot(0x3f8), |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(slot(0x130), |_, _| 0x4e4du32.into_ret());
        // Teammates: words +8 = follows (`00566950`), +0xc = named (`00474cb0`).
        e.register(0x0056_6950, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0047_4cb0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(0x0055_d520, |_, _| 0x5e5eu32.into_ret());
        e.register(0x0084_e3a0, |_, a| (a[0] & 0xffff).into_ret());
        let m = e.mem.alloc(0x40);
        let player = object(&mut e, &[0, 0, 0]);
        let follower = object(&mut e, &[0, 1, 0]);
        let stayer = object(&mut e, &[0, 0, 0]);
        let stuck = object(&mut e, &[0, 1, 1]);
        group_lists(&mut e, stuck);
        let old = group_of(&mut e, &[player, follower, stayer, stuck]);
        e.mem.set_u32(player + 4, old);
        e.map(PLAYER_POINTER, 4);
        e.set_global(PLAYER_POINTER, player);
        let target = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let made = e.call(0x0099_1f80, &args![m, target]).u32();
        let created = calls(&e, GROUP_CONSTRUCT);
        assert_eq!(created.len(), 2);
        let (first, second) = (created[0][0], created[1][0]);
        assert_eq!(made, second);
        assert_eq!(e.mem.u32(first), 2);
        assert_eq!(e.mem.u32(first + 4), player);
        assert_eq!(e.mem.u32(first + 8), follower);
        assert_eq!(e.mem.bytes(old, 12), group_bytes(&[stayer, stuck]));
        assert_eq!(
            calls(&e, slot(0x3fc)),
            vec![vec![player, first], vec![follower, first]]
        );
        assert_eq!(calls(&e, 0x0098_6740), vec![vec![first, old]]);
        assert_eq!(
            calls(&e, DEBUG_PRINT),
            vec![vec![TEAMMATE_NOT_MOVED_FORMAT, 0x4e4d, stuck & 0xffff]]
        );
        assert_eq!(calls(&e, GROUP_ADD_TARGET), vec![vec![first, target]]);
        assert_eq!(e.mem.u32(second + 4), target);
        // A player alone in the group keeps it.
        let alone = group_of(&mut e, &[player]);
        e.mem.set_u32(player + 4, alone);
        e.call_log = Some(vec![]);
        e.call(0x0099_1f80, &args![m, target]);
        assert_eq!(calls(&e, GROUP_ADD_TARGET), vec![vec![alone, target]]);
        assert_eq!(calls(&e, GROUP_CONSTRUCT).len(), 1);
    }

    fn group_bytes(members: &[u32]) -> Vec<u8> {
        let mut out = (members.len() as u32).to_le_bytes().to_vec();
        for m in members {
            out.extend(m.to_le_bytes());
        }
        out
    }

    /// Doubles for the controller paths: an actor's slot `+0x428` answers its word `+4`, the
    /// controller constructor answers its block (with the test vtable), `004fb070` the word
    /// `+4` of the controller.
    fn controller_doubles(e: &mut Engine) {
        group_creation(e);
        e.register(ARRAY_SET_AT_GROW, |_, _| 1u32.into_ret());
        e.register(slot(0x428), |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(CONTROLLER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            a[0].into_ret()
        });
        e.register(CONTROLLER_GROUP, |e, a| e.mem.u32(a[0] + 4).into_ret());
        noop(e, &[slot(0x10)]);
    }

    #[test]
    fn add_combatant_reuses_or_builds_a_controller() {
        let mut e = Engine::new();
        controller_doubles(&mut e);
        e.register(GROUP_ADD_MEMBER, |_, _| 1u32.into_ret());
        let m = e.mem.alloc(0x40);
        let target = e.mem.alloc(4);
        // An existing controller: its group gets the target.
        let controller = object(&mut e, &[0x7000]);
        let fighter = object(&mut e, &[controller]);
        noop(&mut e, &[GROUP_ADD_TARGET]);
        e.call_log = Some(vec![]);
        let r = e
            .call(0x0099_2110, &args![m, fighter, target, 9u32, 9u32])
            .u32();
        assert_eq!(r, controller);
        assert_eq!(calls(&e, GROUP_ADD_TARGET), vec![vec![0x7000, target]]);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert_eq!(calls(&e, SCOPE_BEGIN)[0][4], 0x1a7);
        assert_eq!(calls(&e, SCOPE_END).len(), 1);
        // No controller and `008b0670` refuses: nothing.
        let newcomer = object(&mut e, &[0]);
        e.register(0x008b_0670, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0099_2110, &args![m, newcomer, target, 0u32, 0u32])
                .u32(),
            0
        );
        assert!(calls(&e, GROUP_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, SCOPE_END).len(), 1);
        // Accepted: a new group and controller.
        e.register(0x008b_0670, |_, _| 1u32.into_ret());
        e.register(GROUP_ADD_TARGET, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let r = e
            .call(0x0099_2110, &args![m, newcomer, target, 0u32, 0u32])
            .u32();
        let group = calls(&e, GROUP_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, CONTROLLER_CONSTRUCT),
            vec![vec![r, newcomer, group, 0]]
        );
        assert_eq!(calls(&e, GROUP_ADD_MEMBER), vec![vec![group, newcomer]]);
        // The target refused: the controller deletes itself and null comes back.
        e.register(GROUP_ADD_TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0099_2110, &args![m, newcomer, target, 0u32, 0u32])
                .u32(),
            0
        );
        let made = calls(&e, CONTROLLER_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, slot(0x10)), vec![vec![made, 1]]);
    }

    #[test]
    fn add_group_member_merges_or_builds_a_controller() {
        let mut e = Engine::new();
        controller_doubles(&mut e);
        e.register(0x0098_7530, |_, _| 1u32.into_ret());
        noop(&mut e, &[0x0098_6bd0]);
        let m = e.mem.alloc(0x40);
        let group = e.mem.alloc(0x40);
        // Controlled, in another group: merged.
        let controller = object(&mut e, &[0x7100]);
        let member = object(&mut e, &[controller]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0099_22b0, &args![m, member, group]).u32(),
            controller
        );
        assert_eq!(calls(&e, 0x0098_7530), vec![vec![group, 0x7100]]);
        assert_eq!(calls(&e, 0x0098_6bd0), vec![vec![0x7100]]);
        assert_eq!(calls(&e, SCOPE_BEGIN)[0][4], 0x1c6);
        // Already in this group: nothing to merge.
        e.mem.set_u32(controller + 4, group);
        e.call_log = Some(vec![]);
        e.call(0x0099_22b0, &args![m, member, group]);
        assert!(calls(&e, 0x0098_7530).is_empty());
        // Not controlled: a controller once the group takes the actor.
        let loner = object(&mut e, &[0]);
        e.register(GROUP_ADD_MEMBER, |_, _| Ret::default());
        assert_eq!(e.call(0x0099_22b0, &args![m, loner, group]).u32(), 0);
        e.register(GROUP_ADD_MEMBER, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(0x0099_22b0, &args![m, loner, group]).u32();
        assert_eq!(
            calls(&e, CONTROLLER_CONSTRUCT),
            vec![vec![r, loner, group, 0]]
        );
    }

    #[test]
    fn a_pretend_combatant_gets_a_group_and_a_controller_with_the_flag() {
        let mut e = Engine::new();
        controller_doubles(&mut e);
        noop(&mut e, &[GROUP_ADD_MEMBER]);
        let m = e.mem.alloc(0x40);
        let actor = object(&mut e, &[0]);
        e.call_log = Some(vec![]);
        let r = e.call(0x0099_23d0, &args![m, actor, 3u32]).u32();
        let group = calls(&e, GROUP_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, GROUP_ADD_MEMBER), vec![vec![group, actor]]);
        assert_eq!(
            calls(&e, CONTROLLER_CONSTRUCT),
            vec![vec![r, actor, group, 3]]
        );
        e.register(OPERATOR_NEW, |e, a| {
            if a[0] == 0x188 {
                Ret::default()
            } else {
                e.mem.alloc(a[0]).into_ret()
            }
        });
        assert_eq!(e.call(0x0099_23d0, &args![m, actor, 3u32]).u32(), 0);
    }

    #[test]
    fn the_acquired_object_gets_the_other_actor_and_the_actor_joins_the_map() {
        let mut e = Engine::new();
        let m = manager(&mut e);
        // The process (word +4 of the actor) answers its word +4 through slot +0x22c.
        e.register(0x008d_8520, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(slot(0x22c), |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0041_ca90, |e, a| e.mem.u32(a[0]).into_ret());
        noop(&mut e, &[0x009f_8330, slot(0x40c)]);
        let kind_18 = e.mem.alloc(4);
        e.mem.set_u32(kind_18, 0x18);
        let process = object(&mut e, &[kind_18]);
        let actor = object(&mut e, &[process]);
        let other = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x0099_2480, &args![m, actor, other]);
        assert_eq!(calls(&e, 0x009f_8330), vec![vec![kind_18, other]]);
        assert!(calls(&e, slot(0x40c)).is_empty());
        let map = e.mem.u32(m + 0x10);
        assert_eq!(map_value(&mut e, map, actor), Some(0));
        // No object: the actor's slot +0x40c runs and nothing is told.
        e.mem.set_u32(process + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0099_2480, &args![m, actor, other]);
        assert_eq!(calls(&e, slot(0x40c)), vec![vec![actor]]);
        assert!(calls(&e, 0x009f_8330).is_empty());
        // The same actor: nothing at all.
        e.call_log = Some(vec![]);
        e.call(0x0099_2480, &args![m, actor, actor]);
        assert!(calls(&e, 0x008d_8520).is_empty());
    }

    /// Doubles for the helping questions: `GetShouldHelp(helper, x)` answers the word +8 of
    /// `x`, `GetShouldAttackActor(helper, x, ..)` the word +0xc of `x`; groups list their
    /// targets as `[count, targets...]` at `+0x20` and members as `[count, members...]`.
    fn help_doubles(e: &mut Engine) {
        e.register(0x008b_0970, |e, a| e.mem.u32(a[1] + 8).into_ret());
        e.register(0x008b_06d0, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (0, 0, 0));
            e.mem.u32(a[1] + 0xc).into_ret()
        });
        e.register(slot(0x3f8), |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(GROUP_TARGET_COUNT, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(GROUP_TARGET, |e, a| {
            e.mem.u32(a[0] + 0x24 + 4 * a[1]).into_ret()
        });
        e.register(GROUP_MEMBER_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GROUP_MEMBER, |e, a| {
            e.mem.u32(a[0] + 4 + 4 * a[1]).into_ret()
        });
    }

    fn helping_group(e: &mut Engine, members: &[u32], targets: &[u32]) -> u32 {
        let g = group_of(e, members);
        e.mem.set_u32(g + 0x20, targets.len() as u32);
        for (i, t) in targets.iter().enumerate() {
            e.mem.set_u32(g + 0x24 + 4 * i as u32, *t);
        }
        g
    }

    #[test]
    fn want_to_help_a_group_unless_helping_its_targets_or_attacking_it() {
        let mut e = Engine::new();
        help_doubles(&mut e);
        let helper = e.mem.alloc(4);
        let calm = object(&mut e, &[0, 0, 0]);
        let liked = object(&mut e, &[0, 1, 0]);
        let hated = object(&mut e, &[0, 0, 1]);
        let groupless = object(&mut e, &[0, 1, 0]);
        // Not wanted at all.
        assert!(!e.call(0x0099_2530, &args![0u32, helper, calm]).bool());
        assert!(e.call(0x0099_2530, &args![0u32, helper, groupless]).bool());
        let other = object(&mut e, &[0, 1, 0]);
        let group = helping_group(&mut e, &[calm], &[calm]);
        e.mem.set_u32(other + 4, group);
        assert!(e.call(0x0099_2530, &args![0u32, helper, other]).bool());
        let group = helping_group(&mut e, &[calm], &[liked]);
        e.mem.set_u32(other + 4, group);
        assert!(!e.call(0x0099_2530, &args![0u32, helper, other]).bool());
        let group = helping_group(&mut e, &[calm, hated], &[calm]);
        e.mem.set_u32(other + 4, group);
        assert!(!e.call(0x0099_2530, &args![0u32, helper, other]).bool());
        // A null helper is not asked further.
        let group = helping_group(&mut e, &[hated], &[liked]);
        e.mem.set_u32(other + 4, group);
        assert!(e.call(0x0099_2530, &args![0u32, 0u32, other]).bool());
    }

    #[test]
    fn helping_one_of_the_groups_targets() {
        let mut e = Engine::new();
        help_doubles(&mut e);
        let helper = e.mem.alloc(4);
        let wanted = object(&mut e, &[0, 1, 0]);
        let unwanted = object(&mut e, &[0, 0, 0]);
        let actor = object(&mut e, &[0]);
        assert!(!e.call(0x0099_2640, &args![0u32, helper, actor]).bool());
        let group = helping_group(&mut e, &[], &[unwanted]);
        e.mem.set_u32(actor + 4, group);
        assert!(!e.call(0x0099_2640, &args![0u32, helper, actor]).bool());
        let group = helping_group(&mut e, &[], &[unwanted, wanted]);
        e.mem.set_u32(actor + 4, group);
        assert!(e.call(0x0099_2640, &args![0u32, helper, actor]).bool());
    }

    #[test]
    fn targeting_an_actor_sets_the_groups_bit_sign_extended() {
        let mut e = Engine::new();
        let m = manager(&mut e);
        e.register(GROUP_NUMBER, |e, a| e.mem.u32(a[0]).into_ret());
        let map = e.mem.u32(m + 0x10);
        let actor = 0x0002_0000u32;
        let group = |e: &mut Engine, n: u32| {
            let g = e.mem.alloc(4);
            e.mem.set_u32(g, n);
            g
        };
        let g3 = group(&mut e, 3);
        let g33 = group(&mut e, 33);
        let g31 = group(&mut e, 31);
        e.call(0x0099_2a50, &args![m, actor, g3]);
        assert_eq!(map_value(&mut e, map, actor), Some(8));
        e.call(0x0099_2a50, &args![m, actor, g33]);
        assert_eq!(map_value(&mut e, map, actor), Some(0xa));
        e.call(0x0099_2a50, &args![m, actor, g31]);
        assert_eq!(map_value(&mut e, map, actor), Some(0xffff_ffff_8000_000a));
        // A bit already set changes nothing (no new item either).
        e.call(0x0099_2a50, &args![m, actor, g3]);
        assert_eq!(e.mem.u32(map + 0xc), 1);
    }

    #[test]
    fn clearing_a_groups_bit_needs_the_actor_and_the_bit() {
        let mut e = Engine::new();
        let m = manager(&mut e);
        e.register(GROUP_NUMBER, |e, a| e.mem.u32(a[0]).into_ret());
        let map = e.mem.u32(m + 0x10);
        let g1 = e.mem.alloc(4);
        e.mem.set_u32(g1, 1);
        e.call(0x0099_2b00, &args![m, 0x30u32, g1]);
        assert_eq!(map_value(&mut e, map, 0x30), None);
        e.call(0x0084_4310, &args![map, 0x30u32, 6u32, 1u32]);
        e.call(0x0099_2b00, &args![m, 0x30u32, g1]);
        assert_eq!(map_value(&mut e, map, 0x30), Some(0x1_0000_0004));
        e.call(0x0099_2b00, &args![m, 0x30u32, g1]);
        assert_eq!(map_value(&mut e, map, 0x30), Some(0x1_0000_0004));
    }

    #[test]
    fn the_swapping_forwarders_reach_the_map_with_the_second_word_as_key() {
        let mut e = Engine::new();
        let m = manager(&mut e);
        e.register(GROUP_NUMBER, |e, a| e.mem.u32(a[0]).into_ret());
        let map = e.mem.u32(m + 0x10);
        let g = e.mem.alloc(4);
        e.mem.set_u32(g, 4);
        e.call(0x0099_26c0, &args![m, g, 0x40u32]);
        assert_eq!(map_value(&mut e, map, 0x40), Some(0x10));
        e.call(0x0099_26e0, &args![m, g, 0x40u32]);
        assert_eq!(map_value(&mut e, map, 0x40), Some(0));
    }

    #[test]
    fn the_second_word_forwarder_adds_an_empty_mask() {
        let mut e = Engine::new();
        let m = manager(&mut e);
        let map = e.mem.u32(m + 0x10);
        e.call(0x0099_2700, &args![m, 0x99u32, 0x50u32]);
        assert_eq!(map_value(&mut e, map, 0x50), Some(0));
        assert_eq!(map_value(&mut e, map, 0x99), None);
    }

    #[test]
    fn an_actor_joins_the_map_once() {
        let mut e = Engine::new();
        let m = manager(&mut e);
        let map = e.mem.u32(m + 0x10);
        e.call(0x0084_4310, &args![map, 0x60u32, 5u32, 0u32]);
        e.call(0x0099_29e0, &args![m, 0x60u32]);
        assert_eq!(map_value(&mut e, map, 0x60), Some(5));
        e.call(0x0099_29e0, &args![m, 0x61u32]);
        assert_eq!(map_value(&mut e, map, 0x61), Some(0));
        assert_eq!(e.mem.u32(map + 0xc), 2);
    }

    #[test]
    fn removing_an_actor_goes_to_the_map() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0040_5430]);
        let m = e.mem.alloc(0x20);
        e.mem.set_u32(m + 0x10, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x0099_2a30, &args![m, 0x60u32]);
        assert_eq!(calls(&e, 0x0040_5430), vec![vec![0x7700, 0x60]]);
    }

    #[test]
    fn the_reaction_flags_test_their_bits() {
        let mut e = Engine::new();
        let r = e.mem.alloc(0xac);
        e.mem.set_u32(r + 0xa8, 0x60);
        assert!(e.call(0x0099_28c0, &args![r, 0x41u32]).bool());
        assert!(!e.call(0x0099_28c0, &args![r, 0x81u32]).bool());
        assert!(e.call(0x0099_28a0, &args![r]).bool());
        assert!(e.call(0x0099_28e0, &args![r]).bool());
        assert!(!e.call(0x0099_2900, &args![r]).bool());
    }

    #[test]
    fn a_group_state_of_one_answers_one() {
        let mut e = Engine::new();
        let g = e.mem.alloc(0x158);
        for (byte, answer) in [(1u8, 1u32), (0, 0), (2, 0), (0x81, 0)] {
            e.mem.set_u8(g + 0x154, byte);
            assert_eq!(e.call(0x0099_31a0, &args![g]).u32(), answer);
        }
    }

    #[test]
    fn the_two_calls_run_in_order_on_this() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0099_1310, 0x0099_11c0]);
        e.call_log = Some(vec![]);
        e.call(0x0099_3800, &args![0x5000u32, 0x77u32]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x0099_3800)
            .collect();
        assert_eq!(
            log,
            vec![(0x0099_1310, vec![0x5000, 1]), (0x0099_11c0, vec![0x5000])]
        );
    }

    #[test]
    fn every_group_and_the_threat_map_get_the_word() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0098_eb30, 0x009a_5cd0]);
        let m = e.mem.alloc(0x40);
        let table = e.mem.alloc(12);
        e.mem.set_u32(table, 0x6000);
        e.mem.set_u32(table + 8, 0x6100);
        e.mem.set_u32(m + 0x38, table);
        e.mem.set_u32(m + 0x3c, 3);
        e.mem.set_u32(m + 0x14, 0x6200);
        e.register(GROUP_COUNT, |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        e.register(GROUP_SLOT, |e, a| {
            (e.mem.u32(a[0] + 0x38) + 4 * a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0099_3790, &args![m, 0x99u32]);
        assert_eq!(
            calls(&e, 0x0098_eb30),
            vec![vec![0x6000, 0x99], vec![0x6100, 0x99]]
        );
        assert_eq!(calls(&e, 0x009a_5cd0), vec![vec![0x6200, 0x99]]);
    }

    /// Doubles for `00992ba0`: an actor's words are +4 skip (slot `+0x22c`), +8 group (slot
    /// `+0x3f8`), +0xc its x position (slot `+0x4d8` writes `(x, 0, 0)`); `00439ef0` subtracts,
    /// `004a7290` answers the squared length; group numbers are their first word.
    fn nearby_doubles(e: &mut Engine) {
        e.register(slot(0x22c), |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(slot(0x3f8), |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(slot(0x4d8), |e, a| {
            assert_eq!(a[2], 0.5f32.to_bits());
            let x = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[1], x);
            e.mem.set_u32(a[1] + 4, 0);
            e.mem.set_u32(a[1] + 8, 0);
            Ret::default()
        });
        e.register(0x0043_9ef0, |e, a| {
            for k in 0..3 {
                let d = e.mem.f32(a[0] + 4 * k) - e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, d);
            }
            Ret::default()
        });
        e.register(0x004a_7290, |e, a| {
            let s: f32 = (0..3).map(|k| e.mem.f32(a[0] + 4 * k).powi(2)).sum();
            Ret {
                st0: f64::from(s),
                ..Ret::default()
            }
        });
        noop(e, &[0x0044_0da0, 0x0044_0d80]);
        e.register(GROUP_NUMBER, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(SHIFT_LEFT_64, |_, a| {
            let value = u64::from(a[0]) | (u64::from(a[1]) << 32);
            (value << (a[2] as u8)).into_ret()
        });
        e.register(MAP_FIRST_POSITION, |e, a| {
            let table = e.mem.u32(a[0] + 8);
            (0..e.mem.u32(a[0] + 4))
                .map(|i| e.mem.u32(table + 4 * i))
                .find(|h| *h != 0)
                .unwrap_or(0)
                .into_ret()
        });
    }

    /// A manager whose actor map holds `(actor, mask)`.
    fn crowd(e: &mut Engine, members: &[(u32, u64)]) -> u32 {
        let m = manager(e);
        let map = e.mem.u32(m + 0x10);
        for (actor, mask) in members {
            e.call(
                0x0084_4310,
                &args![map, *actor, *mask as u32, (*mask >> 32) as u32],
            );
        }
        m
    }

    fn at(e: &mut Engine, skip: u32, group: u32, x: f32) -> u32 {
        object(e, &[skip, group, x.to_bits()])
    }

    #[test]
    fn nearby_actors_are_counted_by_kind() {
        let mut e = Engine::new();
        nearby_doubles(&mut e);
        let group = e.mem.alloc(4);
        e.mem.set_u32(group, 3);
        let me = at(&mut e, 0, group, 1.0);
        let friend = at(&mut e, 0, group, 2.0);
        let marked = at(&mut e, 0, 0x55, 3.0);
        let stranger = at(&mut e, 0, 0x55, 4.0);
        let far = at(&mut e, 0, 0x55, 100.0);
        let busy = at(&mut e, 1, 0x55, 1.0);
        let m = crowd(
            &mut e,
            &[
                (me, 0),
                (friend, 0),
                (marked, 8),
                (stranger, 0x10),
                (far, 0),
                (busy, 0),
                (0, 0),
            ],
        );
        let outs = e.mem.alloc(16);
        let total = e
            .call(
                0x0099_2ba0,
                &args![
                    m,
                    0.0f32,
                    0.0f32,
                    0.0f32,
                    10.0f32,
                    0.5f32,
                    me,
                    outs,
                    outs + 4,
                    outs + 8,
                    outs + 12
                ],
            )
            .u32();
        assert_eq!(total, 4);
        let counts: Vec<u32> = (0..4).map(|i| e.mem.u32(outs + 4 * i)).collect();
        assert_eq!(counts, vec![1, 1, 1, 1]);
        // Null outs are skipped; no group means no mask (every stranger is "other").
        let loner = at(&mut e, 0, 0, 1.0);
        let m = crowd(&mut e, &[(loner, 0), (marked, 8)]);
        e.mem.set_u32(outs + 12, 0);
        let total = e
            .call(
                0x0099_2ba0,
                &args![
                    m,
                    0.0f32,
                    0.0f32,
                    0.0f32,
                    10.0f32,
                    0.5f32,
                    loner,
                    0u32,
                    0u32,
                    0u32,
                    outs + 12
                ],
            )
            .u32();
        assert_eq!(total, 2);
        assert_eq!(e.mem.u32(outs + 12), 1);
    }

    #[test]
    fn an_explosion_goes_ahead_unless_someone_protected_is_caught() {
        let mut e = Engine::new();
        nearby_doubles(&mut e);
        e.register(0x0047_7900, |_, _| Ret {
            st0: 6.0,
            ..Ret::default()
        });
        e.register(0x006a_78f0, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.register(0x0064_79a0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.map(SETTING_011CF3F0, 8);
        e.mem.set_f32(SETTING_011CF3F0 + 4, 2.0);
        e.map(SETTING_011CEA74, 8);
        e.mem.set_f32(SETTING_011CEA74 + 4, 1.0);
        let reaction = e.mem.alloc(0xac);
        e.register_double(0x008a_7f80, move |_, _| reaction.into_ret());
        let limit = e.mem.alloc(4);
        e.mem.set_u32(limit, 2);
        e.register_double(0x0043_d4d0, move |_, _| limit.into_ret());
        e.register(slot(0x8), |e, a| e.mem.u32(a[0] + 4).into_ret());
        // The actor: words +4 skip, +8 group, +0xc x; its object at +0xa4 answers word +0xa8.
        let actor = object(&mut e, &[0; 0x2c]);
        e.mem.set_f32(actor + 0xc, 1.0);
        e.mem.set_u32(actor + 0xa4, VTABLE);
        e.mem.set_u32(actor + 0xa8, 2);
        let explosion = e.mem.alloc(4);
        e.mem.set_f32(explosion, 3.0);
        let point = e.mem.alloc(12);
        let m = crowd(&mut e, &[(actor, 0)]);
        assert!(e
            .call(0x0099_2720, &args![m, actor, 0u32, point, 0.5f32])
            .bool());
        // The setting is above the explosion's inner value: allowed at once.
        e.mem.set_f32(explosion, 1.0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0099_2720, &args![m, actor, explosion, point, 0.5f32])
            .bool());
        assert!(calls(&e, 0x0064_79a0).is_empty());
        e.mem.set_f32(explosion, 3.0);
        // The reach is below the second setting: allowed.
        e.mem.set_f32(SETTING_011CEA74 + 4, 7.0);
        assert!(e
            .call(0x0099_2720, &args![m, actor, explosion, point, 0.5f32])
            .bool());
        e.mem.set_f32(SETTING_011CEA74 + 4, 1.0);
        // The actor itself is caught: it needs the 0x20 flag.
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0099_2720, &args![m, actor, explosion, point, 0.5f32])
            .bool());
        assert_eq!(
            calls(&e, 0x0064_79a0),
            vec![vec![6.0f32.to_bits(), 3.0f32.to_bits(), 2.0f32.to_bits()]]
        );
        e.mem.set_u32(reaction + 0xa8, 0x20);
        assert!(e
            .call(0x0099_2720, &args![m, actor, explosion, point, 0.5f32])
            .bool());
        // Another actor caught too: at the limit it needs the 0x80 flag.
        let stranger = at(&mut e, 0, 0x55, 2.0);
        let m = crowd(&mut e, &[(actor, 0), (stranger, 0)]);
        assert!(!e
            .call(0x0099_2720, &args![m, actor, explosion, point, 0.5f32])
            .bool());
        e.mem.set_u32(reaction + 0xa8, 0xa0);
        assert!(e
            .call(0x0099_2720, &args![m, actor, explosion, point, 0.5f32])
            .bool());
        e.mem.set_u32(reaction + 0xa8, 0x20);
        e.mem.set_u32(limit, 3);
        assert!(e
            .call(0x0099_2720, &args![m, actor, explosion, point, 0.5f32])
            .bool());
    }

    /// The load buffer's words, read in order by `00864980`, `00864a60` and `008648e0`.
    fn load_queue(e: &mut Engine, words: &[u32]) {
        let queue = Rc::new(RefCell::new(
            words.iter().copied().collect::<VecDeque<u32>>(),
        ));
        let q = queue.clone();
        e.register_double(LOAD_DATA, move |e, a| {
            assert_eq!(a[2], 4);
            let w = q.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[1], w);
            Ret::default()
        });
        let q = queue.clone();
        e.register_double(LOAD_COUNT, move |_, _| {
            q.borrow_mut().pop_front().unwrap().into_ret()
        });
        let q = queue;
        e.register_double(LOAD_FORM_ID, move |e, a| {
            let w = q.borrow_mut().pop_front().unwrap();
            e.mem.set_u32(a[1], w);
            1u32.into_ret()
        });
    }

    #[test]
    fn load_game_rebuilds_groups_and_the_actor_map() {
        let mut e = Engine::new();
        load_queue(&mut e, &[0x77, 2, 3, 0x30, 5, 0x50, 2, 0x100, 0x200]);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(GROUP_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_AT_GROW, |e, a| {
            assert_eq!(e.mem.u32(a[2]) & 0xf000_0000, 0x2000_0000);
            Ret::default()
        });
        noop(
            &mut e,
            &[
                GROUP_SET_INDEX,
                0x006e_cd40,
                0x0098_e3d0,
                0x009a_5a70,
                0x009a_5ff0,
                0x009a_5f20,
            ],
        );
        let m = manager(&mut e);
        e.mem.set_u32(m + 0x14, 0x6200);
        let buffer = 0x9000u32;
        e.call_log = Some(vec![]);
        e.call(0x0099_3440, &args![m, buffer]);
        assert_eq!(e.mem.u32(m + 0x34), 0x77);
        let groups: Vec<u32> = calls(&e, GROUP_CONSTRUCT).iter().map(|c| c[0]).collect();
        assert_eq!(
            calls(&e, GROUP_CONSTRUCT),
            vec![vec![groups[0], 0], vec![groups[1], 0]]
        );
        assert_eq!(
            calls(&e, GROUP_SET_INDEX),
            vec![vec![groups[0], 3], vec![groups[1], 5]]
        );
        assert_eq!(
            calls(&e, 0x006e_cd40),
            vec![vec![groups[0], 0x30], vec![groups[1], 0x50]]
        );
        assert_eq!(
            calls(&e, 0x0098_e3d0),
            vec![vec![groups[0], buffer], vec![groups[1], buffer]]
        );
        let grown = calls(&e, ARRAY_SET_AT_GROW);
        assert_eq!(
            grown.iter().map(|c| (c[0], c[1])).collect::<Vec<_>>(),
            vec![(m, 3), (m, 5)]
        );
        assert_eq!(calls(&e, 0x009a_5a70), vec![vec![0x6200, buffer]]);
        let map = e.mem.u32(m + 0x10);
        assert_eq!(map_value(&mut e, map, 0x100), Some(0));
        assert_eq!(map_value(&mut e, map, 0x200), Some(0));
        assert_eq!(calls(&e, 0x009a_5ff0), vec![vec![m + 0x20, buffer]]);
        assert_eq!(
            calls(&e, 0x009a_5f20),
            vec![vec![m + 0x24, buffer], vec![m + 0x2c, buffer]]
        );
    }

    #[test]
    fn init_load_game_moves_resolved_keys_into_a_new_map() {
        let mut e = Engine::new();
        nearby_doubles(&mut e);
        map_vtable(&mut e, ACTOR_MAP_VTABLE);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BUCKETS_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        noop(
            &mut e,
            &[
                MEMSET,
                MAP_REMOVE_ALL,
                slot(0x900),
                0x0098_e990,
                0x009a_5c40,
            ],
        );
        e.register(0x0048_39c0, |_, a| (a[0] + 1).into_ret());
        e.register(0x00ec_43fb, |_, a| {
            assert_eq!(a[1..], [0, 0x0118_3028, 0x0118_46d4, 0]);
            u32::from(a[0] == 0x101).wrapping_mul(0x9101).into_ret()
        });
        let m = crowd(&mut e, &[(0x100, 5), (0x200, 6)]);
        e.mem.set_u32(MAP_VTABLE, slot(0x900));
        let old = e.mem.u32(m + 0x10);
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, 0x6000);
        e.mem.set_u32(m + 0x38, table);
        e.mem.set_u32(m + 0x3c, 2);
        e.mem.set_u32(m + 0x14, 0x6200);
        e.register(GROUP_COUNT, |e, a| e.mem.u32(a[0] + 0x3c).into_ret());
        e.register(GROUP_SLOT, |e, a| {
            (e.mem.u32(a[0] + 0x38) + 4 * a[1]).into_ret()
        });
        let buffer = 0x9000u32;
        e.call_log = Some(vec![]);
        e.call(0x0099_3600, &args![m, buffer]);
        let map = e.mem.u32(m + 0x10);
        assert_ne!(map, old);
        assert_eq!(e.mem.u32(map), ACTOR_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 0x25);
        assert_eq!(map_value(&mut e, map, 0x9101), Some(5));
        assert_eq!(e.mem.u32(map + 0xc), 1);
        assert_eq!(calls(&e, MAP_REMOVE_ALL), vec![vec![old]]);
        assert_eq!(calls(&e, slot(0x900)), vec![vec![old, 1]]);
        assert_eq!(calls(&e, 0x0098_e990), vec![vec![0x6000, buffer]]);
        assert_eq!(calls(&e, 0x009a_5c40), vec![vec![0x6200, buffer]]);
    }
}
