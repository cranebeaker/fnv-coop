//! `fallout/ai/combat/combatgroupstrategy.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds the four combat group strategies (`CombatGroupStrategy`
//! and its subclasses `FallBack`, `Advance`, `Flank`, `Ambush`; class names
//! confirmed from the vtables' RTTI), the table of the four strategy
//! objects the game creates at start-up, the choice of a group's strategy,
//! and `CombatGroupData`, the per-choice summary of a group's members that
//! the strategies read.
//!
//! Conventions of this file:
//! - `CombatGroupData` and `CombatGroup` are the layouts of
//!   `units/fallout_ai/combatgroup.rs`; the `CombatGroupData` fields that
//!   layout does not declare yet are addressed through the `DATA_*` offset
//!   constants (Xbox PDB names, offsets checked against the PC code).
//! - The two `BSSimpleArray<GroupMemberData>`s of `CombatGroupData`
//!   (`RangedMembers` at +0x18, `MeleeMembers` at +0x28) are used through
//!   the game's own accessors, called by address (`0044ddc0` size,
//!   `00844160` / `006950d0` element, `006db840` append). A
//!   `GroupMemberData` (Xbox PDB) is 8 bytes: `pActor` at +0, `iIndex` (the
//!   member's index in the `CombatGroup`) at +4.
//! - The compiler's exception-unwinding frames (`0098fdf0`, `009900f0`,
//!   `00990320`) and stack-cookie checks (`00990250`, `00990640`) are not
//!   translated; those functions are marked incomplete for that reason only.
//! - x87 note: comparisons follow the disassembly's flag tests (a NaN fails
//!   `<` and `>` and passes the `TEST AH,0x41` "not greater" tests).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::crt::glue::alloca16_esp;
use crate::units::fallout_ai::combatgroup::{CombatGroup, CombatGroupData};

/// Takes a byte count and returns a heap block or null (cdecl; the game's
/// `new`).
const ALLOCATE: u32 = 0x0040_1000;
/// Frees a block from `ALLOCATE` (cdecl; the game's `delete`).
const DEALLOCATE: u32 = 0x0040_1030;
/// cdecl, printf-style: formats and prints a debug message (`005b5e40`).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `"COMBAT: Could not create combat group strategy of type %i"`.
const CREATE_FAILED_FORMAT: u32 = 0x0108_d8fc;

/// Vtables (class names from their RTTI complete object locators).
const VTABLE_STRATEGY: u32 = 0x0108_d94c;
const VTABLE_FLANK: u32 = 0x0108_d93c;
const VTABLE_FALL_BACK: u32 = 0x0108_d95c;
const VTABLE_AMBUSH: u32 = 0x0108_d96c;
const VTABLE_ADVANCE: u32 = 0x0108_d97c;
/// Vtable of `BSSimpleArray<GroupMemberData>` the array constructor stores.
const VTABLE_MEMBER_ARRAY: u32 = 0x0108_d758;

/// The four strategy objects (`CombatGroupStrategy*[4]`) that `00990070`
/// creates and `009900b0` frees.
const STRATEGIES: u32 = 0x011f_18b4;
/// The four strategy names (`const char*[4]`, the first is
/// `"COMBAT_GROUP_STRATEGY_FALLBACK"`).
const STRATEGY_NAMES: u32 = 0x011a_4ed0;
/// Object whose `00408d60` result (a byte) disables the strategy choice
/// when nonzero.
const CHOICE_DISABLED_HOLDER: u32 = 0x011f_18d8;
/// Initial value of `SortRangedMembers`' centre (three words copied as an
/// `NiPoint3`).
const SORT_CENTER_START: u32 = 0x011f_426c;

/// `float` 30.0 in the exe (passed to `0098a470` by Flank and FallBack).
const TIME_FLANK_FALL_BACK: u32 = 0x0101_8f5c;
/// `float` 60.0 in the exe (passed to `0098a470` by Ambush and Advance).
const TIME_AMBUSH_ADVANCE: u32 = 0x0103_4294;
/// `double` 0.5: the ranged share above which Flank sends ranged members.
const FLANK_RANGED_SHARE: u32 = 0x0101_1588;
/// `double` (about 0.33): FallBack's member-health limit.
const FALL_BACK_HEALTH: u32 = 0x0102_6998;
/// `double` (1048576.0, 1024 squared): Advance's limit on a ranged member's squared distance to
/// its combat target.
const ADVANCE_DISTANCE: u32 = 0x0101_ece0;
/// `float` 384.0 in the exe (passed to `00989700` by Flank's update).
const FLANK_UPDATE_DISTANCE: u32 = 0x0105_0b98;

/// `0044ddc0`: size of a `BSSimpleArray` (thiscall).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `00844160`: address of element `i` of a `BSSimpleArray<GroupMemberData>`.
const ARRAY_ELEMENT: u32 = 0x0084_4160;
/// `006950d0`: address of element `i` (the sort's swap uses this one).
const ARRAY_ELEMENT_REF: u32 = 0x0069_50d0;
/// `006db840`: appends a copy of the element at the argument.
const ARRAY_APPEND: u32 = 0x006d_b840;
/// `006dc440`: the array's buffer set-up (two words, both 0 here).
const ARRAY_INIT: u32 = 0x006d_c440;
/// `004b0680`: `GroupMemberData` constructor (this, actor, index).
const MEMBER_DATA_INIT: u32 = 0x004b_0680;
/// `009863b0`: `CombatGroupData` destructor.
const DATA_DESTROY: u32 = 0x0098_63b0;
/// `00726070`: `CombatGroupData` member count (thiscall, `int`).
const DATA_MEMBER_COUNT_GET: u32 = 0x0072_6070;
/// `00990500` (`CombatGroup::ComputeMemberHealthPercentage` in the map, a
/// `CombatGroupData` method): the members' health share in ST0.
const DATA_HEALTH_PERCENTAGE: u32 = 0x0099_0500;
/// `009899a0` `CombatGroup::CheckMemberTargetIntersection` (Xbox PDB).
const CHECK_MEMBER_TARGET_INTERSECTION: u32 = 0x0098_99a0;
/// `00903600`: the group's member actor `i` (thiscall on the group).
const GROUP_MEMBER_ACTOR: u32 = 0x0090_3600;
/// `0098a340`: the strategy assignment of member `i`.
const GROUP_MEMBER_ASSIGNMENT: u32 = 0x0098_a340;
/// `0098a390` (`CombatGroup::SetMemberGroupStrategyAssignment`): sets
/// member `i`'s assignment.
const GROUP_SET_ASSIGNMENT: u32 = 0x0098_a390;
/// `0098a410` `CombatGroup::FindMemberWithStrategyAssignment` (Xbox PDB).
const GROUP_FIND_ASSIGNMENT: u32 = 0x0098_a410;
/// `0098a470`: the group's check (strategy index, `float` time) that a
/// strategy may be chosen again.
const GROUP_STRATEGY_ALLOWED: u32 = 0x0098_a470;
/// `0098a3c0`: called on the group before a newly chosen strategy runs.
const GROUP_BEFORE_EXECUTE: u32 = 0x0098_a3c0;
/// `0045cd60`: the group's current strategy.
const GROUP_CURRENT_STRATEGY: u32 = 0x0045_cd60;
/// `00989700`: group check on a position (position, `float`, 0).
const GROUP_POSITION_CHECK: u32 = 0x0098_9700;
/// `0098e980`: strategy `i` of the table (cdecl).
const STRATEGY_AT: u32 = 0x0098_e980;

/// `00586150`, `005e3fa0`, `009b73b0`, `004030b0`, `00575650`: callees on
/// a member's process / combat objects, by address.
const MEMBER_COMBAT_OBJECT: u32 = 0x0058_6150;
const TARGET_DETECTION: u32 = 0x005e_3fa0;
const MEMBER_EXCLUDED: u32 = 0x009b_73b0;
const COMBAT_TARGET_OF: u32 = 0x0040_30b0;
const ACTOR_POSITION_INTO: u32 = 0x0057_5650;
/// Vtable byte offsets on an actor: the object `0x428` returns (the
/// member's combat state, passed on to `00586150` / `004030b0`), and the
/// position (`0x1f4`, pointer to three `float`s).
const ACTOR_SLOT_COMBAT: u32 = 0x428;
const ACTOR_SLOT_POSITION: u32 = 0x1f4;

/// `NiPoint3` operations: `0063c8a0` `+=`, `004941c0` `/=` (float),
/// `00439ef0` difference into an out point, `004a7290` squared length (ST0).
const POINT_ADD: u32 = 0x0063_c8a0;
const POINT_DIVIDE: u32 = 0x0049_41c0;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;

/// String helpers: `00406d30` bounded copy (dst, size, src), `00eca7f4`
/// `toupper`, `004812f0` compare (nonzero when equal).
const STRING_COPY: u32 = 0x0040_6d30;
const TO_UPPER: u32 = 0x00ec_a7f4;
const STRING_EQUAL: u32 = 0x0048_12f0;
/// `00408d60` on `CHOICE_DISABLED_HOLDER`.
const CHOICE_DISABLED: u32 = 0x0040_8d60;

/// `CombatGroupData` fields `combatgroup.rs` does not declare (Xbox PDB
/// names): `iMemberCount`, `iMembersTargetsIntersecting` (-1 until
/// computed), `iMinimumTargetDetection` (`i32::MAX` until computed),
/// `iMaximumTargetDetection` (`i32::MIN` until computed), and the arrays
/// `RangedMembers` / `MeleeMembers`.
const DATA_MEMBER_COUNT: u32 = 0x04;
const DATA_TARGETS_INTERSECTING: u32 = 0x08;
const DATA_MIN_DETECTION: u32 = 0x0c;
const DATA_MAX_DETECTION: u32 = 0x10;
const DATA_RANGED_MEMBERS: u32 = 0x18;
const DATA_MELEE_MEMBERS: u32 = 0x28;

layout! {
    /// `CombatGroupStrategy` (Xbox PDB), 0x8 bytes; the four subclasses
    /// add no fields. Vtable slots: +0 `CanExecute`, +4 `Execute`,
    /// +8 `Update`.
    pub struct CombatGroupStrategy: 0x08 {
        /// `eType` (Xbox PDB): `COMBAT_GROUP_STRATEGY`.
        0x04 eType: u32,
    }
}

/// `FISTP qword` with truncation, low word: NaN and values outside the
/// 64-bit range store the integer indefinite `0x8000000000000000`.
fn fistp_truncate_low(value: f64) -> u32 {
    let bound = (1u64 << 63) as f64;
    if value.is_nan() || !(-bound..bound).contains(&value) {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

/// A `float` constant pushed as an argument (`FLD` / `FSTP [ESP]`).
fn pushed_float(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(addr))
}

/// A strategy subclass constructor (`this`, type).
type StrategyConstructor =
    fn(&mut Engine, Ptr<CombatGroupStrategy>, u32) -> Ptr<CombatGroupStrategy>;

// Translated from 0098fdf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0098fdf0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Mapped to `CombatGroupStrategy::InitGroupStrategies` (Xbox PDB name of
/// this range); the body creates one strategy object of `strategy_type`
/// (0 FallBack, 1 Advance, 2 Flank, 3 Ambush) with the game's allocator,
/// null if the allocation fails; another type prints
/// `"COMBAT: Could not create combat group strategy of type %i"` and
/// returns null.
pub fn fn_0098fdf0(e: &mut Engine, strategy_type: u32) -> Ptr<CombatGroupStrategy> {
    let construct: StrategyConstructor = match strategy_type {
        0 => fn_0098ffe0,
        1 => fn_00990040,
        2 => fn_0098ff80,
        3 => fn_00990010,
        _ => {
            e.call(PRINT_MESSAGE, &args![CREATE_FAILED_FORMAT, strategy_type]);
            return Ptr::NULL;
        }
    };
    let block = e.call(ALLOCATE, &args![8u32]).ptr::<CombatGroupStrategy>();
    if block.is_null() {
        return Ptr::NULL;
    }
    construct(e, block, strategy_type)
}

// Translated from 0098ff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyFlank` constructor: the base constructor, then the
/// Flank vtable.
pub fn fn_0098ff80(
    e: &mut Engine,
    this: Ptr<CombatGroupStrategy>,
    strategy_type: u32,
) -> Ptr<CombatGroupStrategy> {
    fn_0098ffb0(e, this, strategy_type);
    e.mem.set_u32(this.addr(), VTABLE_FLANK);
    this
}

// Translated from 0098ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategy` constructor: the base vtable and `eType`.
pub fn fn_0098ffb0(
    e: &mut Engine,
    this: Ptr<CombatGroupStrategy>,
    strategy_type: u32,
) -> Ptr<CombatGroupStrategy> {
    e.mem.set_u32(this.addr(), VTABLE_STRATEGY);
    e.set(this, CombatGroupStrategy::eType, strategy_type);
    this
}

// Translated from 0098ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyFallBack` constructor.
pub fn fn_0098ffe0(
    e: &mut Engine,
    this: Ptr<CombatGroupStrategy>,
    strategy_type: u32,
) -> Ptr<CombatGroupStrategy> {
    fn_0098ffb0(e, this, strategy_type);
    e.mem.set_u32(this.addr(), VTABLE_FALL_BACK);
    this
}

// Translated from 00990010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyAmbush` constructor.
pub fn fn_00990010(
    e: &mut Engine,
    this: Ptr<CombatGroupStrategy>,
    strategy_type: u32,
) -> Ptr<CombatGroupStrategy> {
    fn_0098ffb0(e, this, strategy_type);
    e.mem.set_u32(this.addr(), VTABLE_AMBUSH);
    this
}

// Translated from 00990040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyAdvance` constructor.
pub fn fn_00990040(
    e: &mut Engine,
    this: Ptr<CombatGroupStrategy>,
    strategy_type: u32,
) -> Ptr<CombatGroupStrategy> {
    fn_0098ffb0(e, this, strategy_type);
    e.mem.set_u32(this.addr(), VTABLE_ADVANCE);
    this
}

// Translated from 00990070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the four strategies (`fn_0098fdf0` of types 0 to 3) into the
/// strategy table.
pub fn fn_00990070(e: &mut Engine) {
    for index in 0..4u32 {
        let strategy = fn_0098fdf0(e, index);
        e.set_global(STRATEGIES + index * 4, strategy.addr());
    }
}

// Translated from 009900b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the four strategies of the table (`00401030`; the entries are
/// left as they are).
pub fn fn_009900b0(e: &mut Engine) {
    for index in 0..4u32 {
        let strategy: u32 = e.global(STRATEGIES + index * 4);
        e.call(DEALLOCATE, &args![strategy]);
    }
}

// Translated from 009900f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009900f0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Chooses the group's strategy: null when the choice is disabled
/// (`00408d60` on `011f18d8` gives a nonzero byte) or the group has fewer
/// than two members; otherwise the first table strategy (in table order)
/// that is the current one (returned as it is) or whose `CanExecute`
/// accepts the group, after `0098a3c0` on the group and its `Execute`;
/// null if none. The `CombatGroupData` is built on the stack and destroyed
/// (`009863b0`) on every path past it.
pub fn fn_009900f0(e: &mut Engine, group: Ptr<CombatGroup>) -> Ptr<CombatGroupStrategy> {
    let disabled = e
        .call(CHOICE_DISABLED, &args![CHOICE_DISABLED_HOLDER])
        .u32();
    if e.mem.u8(disabled) != 0 {
        return Ptr::NULL;
    }
    e.with_stack(CombatGroupData::SIZE, |e, block| {
        let data = block.cast::<CombatGroupData>();
        fn_00990320(e, data, group);
        let current = e.call(GROUP_CURRENT_STRATEGY, &args![group]).u32();
        let result = if e.call(DATA_MEMBER_COUNT_GET, &args![data]).i32() < 2 {
            0
        } else {
            let mut chosen = 0;
            for index in 0..4u32 {
                let strategy = e.call(STRATEGY_AT, &args![index]).u32();
                if strategy == 0 {
                    continue;
                }
                if strategy == current {
                    chosen = current;
                    break;
                }
                if e.vcall(strategy, 0x00, &args![group, data]).bool() {
                    e.call(GROUP_BEFORE_EXECUTE, &args![group]);
                    e.vcall(strategy, 0x04, &args![group, data]);
                    chosen = strategy;
                    break;
                }
            }
            chosen
        };
        e.call(DATA_DESTROY, &args![data]);
        Ptr::new(result)
    })
}

// Translated from 00990250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00990250 - the stack-cookie check (00ec408c) is not translated
/// `CombatGroupStrategy::GetCombatGroupStrategyByName` (Xbox PDB): copies
/// `name` into a 0x104-byte buffer, upper-cases it (`toupper` per byte) and
/// returns table strategy `i` (`0098e980`) for the first of the four
/// strategy names that compares equal (`004812f0`), null if none does.
pub fn combat_group_strategy_get_combat_group_strategy_by_name(
    e: &mut Engine,
    name: Ptr,
) -> Ptr<CombatGroupStrategy> {
    e.with_stack(0x104, |e, buffer| {
        e.call(STRING_COPY, &args![buffer, 0x104u32, name]);
        let mut at = buffer.addr();
        while e.mem.u8(at) != 0 {
            let upper = e.call(TO_UPPER, &args![e.mem.u8(at) as i8 as i32]).u8();
            e.mem.set_u8(at, upper);
            at += 1;
        }
        for index in 0..4u32 {
            let strategy_name: u32 = e.global(STRATEGY_NAMES + index * 4);
            if e.call(STRING_EQUAL, &args![strategy_name, buffer]).u32() != 0 {
                return e.call(STRATEGY_AT, &args![index]).ptr();
            }
        }
        Ptr::NULL
    })
}

// Translated from 00990320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00990320 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `CombatGroupData` constructor: the group, the "not computed" values
/// (`iMembersTargetsIntersecting` -1, minimum `i32::MAX`, maximum
/// `i32::MIN`, health 0.0), the two empty member arrays, then one
/// `GroupMemberData` (actor, index) per group member whose combat object
/// (actor slot 0x428, then `00586150`) `009b73b0` does not exclude: counted
/// in `iMemberCount`, appended to `MeleeMembers` when `fn_00990480` holds
/// for that object, to `RangedMembers` otherwise.
pub fn fn_00990320(
    e: &mut Engine,
    this: Ptr<CombatGroupData>,
    group: Ptr<CombatGroup>,
) -> Ptr<CombatGroupData> {
    let base = this.addr();
    e.set(this, CombatGroupData::pCombatGroup, group);
    e.mem.set_u32(base + DATA_MEMBER_COUNT, 0);
    e.mem.set_u32(base + DATA_TARGETS_INTERSECTING, u32::MAX);
    e.mem.set_u32(base + DATA_MIN_DETECTION, i32::MAX as u32);
    e.mem.set_u32(base + DATA_MAX_DETECTION, i32::MIN as u32);
    e.set(this, CombatGroupData::fMemberHealthPercentage, 0.0);
    fn_00990ef0(e, this.byte_add(DATA_RANGED_MEMBERS));
    fn_00990ef0(e, this.byte_add(DATA_MELEE_MEMBERS));
    let count = fn_00990890(e, group.cast());
    for index in 0..count {
        let actor = e.call(GROUP_MEMBER_ACTOR, &args![group, index]).u32();
        let combat = e.vcall(actor, ACTOR_SLOT_COMBAT, &[]).u32();
        let object = e.call(MEMBER_COMBAT_OBJECT, &args![combat]).u32();
        if e.call(MEMBER_EXCLUDED, &args![object]).bool() {
            continue;
        }
        let members = e.mem.u32(base + DATA_MEMBER_COUNT);
        e.mem
            .set_u32(base + DATA_MEMBER_COUNT, members.wrapping_add(1));
        let array = if fn_00990480(e, Ptr::new(object)) {
            DATA_MELEE_MEMBERS
        } else {
            DATA_RANGED_MEMBERS
        };
        e.with_stack(8, |e, temporary| {
            let entry = e
                .call(MEMBER_DATA_INIT, &args![temporary, actor, index])
                .u32();
            e.call(ARRAY_APPEND, &args![base + array, entry]);
        });
    }
    this
}

// Translated from 00990480 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the `float` at +0x5c of `this` (the member combat object
/// `00586150` returns) is below the one at +0x58 (false on NaN);
/// `CombatGroupData`'s constructor puts such members in `MeleeMembers`.
pub fn fn_00990480(e: &mut Engine, this: Ptr) -> bool {
    let first = e.mem.f32(this.addr() + 0x58);
    let second = e.mem.f32(this.addr() + 0x5c);
    second < first
}

// Translated from 009904c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupData::GetMembersTargetsIntersecting` (Xbox PDB): computes
/// `iMembersTargetsIntersecting` once (`CombatGroup::CheckMemberTargetIntersection`
/// on the group, 0 or 1) and returns whether it is 1.
pub fn combat_group_data_get_members_targets_intersecting(
    e: &mut Engine,
    this: Ptr<CombatGroupData>,
) -> bool {
    let field = this.addr() + DATA_TARGETS_INTERSECTING;
    if e.mem.u32(field) == u32::MAX {
        let group = e.get(this, CombatGroupData::pCombatGroup);
        let intersecting = e
            .call(CHECK_MEMBER_TARGET_INTERSECTION, &args![group])
            .bool();
        e.mem.set_u32(field, intersecting as u32);
    }
    e.mem.u32(field) == 1
}

// Translated from 00990540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `iMinimumTargetDetection`, computed (`fn_009905a0`) while it is still
/// `i32::MAX`.
pub fn fn_00990540(e: &mut Engine, this: Ptr<CombatGroupData>) -> i32 {
    if e.mem.u32(this.addr() + DATA_MIN_DETECTION) == i32::MAX as u32 {
        fn_009905a0(e, this);
    }
    e.mem.u32(this.addr() + DATA_MIN_DETECTION) as i32
}

// Translated from 00990570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Mapped to `CombatGroupData::ComputeMinMaxTargetDetection`; the body
/// returns `iMaximumTargetDetection`, computed (`fn_009905a0`) while it is
/// still `i32::MIN`.
pub fn fn_00990570(e: &mut Engine, this: Ptr<CombatGroupData>) -> i32 {
    if e.mem.u32(this.addr() + DATA_MAX_DETECTION) == i32::MIN as u32 {
        fn_009905a0(e, this);
    }
    e.mem.u32(this.addr() + DATA_MAX_DETECTION) as i32
}

// Translated from 009905a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes the target detection bounds over the group's members that have
/// a combat object (actor slot 0x428 nonzero): the value `005e3fa0` gives
/// for `00586150` of it. Both fields keep the smaller value (the exe
/// compares `<` for the maximum as well).
pub fn fn_009905a0(e: &mut Engine, this: Ptr<CombatGroupData>) {
    let base = this.addr();
    let group = e.get(this, CombatGroupData::pCombatGroup);
    let count = fn_00990890(e, group.cast());
    for index in 0..count {
        let actor = e.call(GROUP_MEMBER_ACTOR, &args![group, index]).u32();
        let combat = e.vcall(actor, ACTOR_SLOT_COMBAT, &[]).u32();
        if combat == 0 {
            continue;
        }
        let object = e.call(MEMBER_COMBAT_OBJECT, &args![combat]).u32();
        let detection = e.call(TARGET_DETECTION, &args![object]).i32();
        if detection < e.mem.u32(base + DATA_MIN_DETECTION) as i32 {
            e.mem.set_u32(base + DATA_MIN_DETECTION, detection as u32);
        }
        if detection < e.mem.u32(base + DATA_MAX_DETECTION) as i32 {
            e.mem.set_u32(base + DATA_MAX_DETECTION, detection as u32);
        }
    }
}

/// `SortRangedMembers`' fixed frame under `EBP`: `SUB ESP,0x64` and two pushes.
const SORT_FIXED_FRAME: u32 = 0x6c;

// Translated from 00990640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00990640 - the stack-cookie check (00ec408c) is not translated
/// `CombatGroupData::SortRangedMembers` (Xbox PDB): sorts `RangedMembers`
/// by each actor's squared distance to their centre, nearest first (selection
/// sort; ties keep the first). The positions (actor slot 0x1f4) and squared
/// distances live in two `_alloca16` tables (`00ec5ec0`), placed in the exe's
/// frame with [`alloca16_esp`]: the positions from `ESP = EBP-0x6c`, the
/// distances under them, so their addresses and alignment are the exe's. With no
/// ranged member the exe would run its outer loop over `0xffffffff` indices;
/// callers never do that.
pub fn combat_group_data_sort_ranged_members(e: &mut Engine, this: Ptr<CombatGroupData>) {
    let ranged = this.addr() + DATA_RANGED_MEMBERS;
    let count = fn_00990890(e, this.cast());
    // The exe's frame: `EBP` is the top of one block holding the fixed locals
    // (`SUB ESP,0x64` and two pushes: `ESP = EBP-0x6c` before the first
    // `_alloca16`), both tables and both alignment pads.
    let block_size = SORT_FIXED_FRAME
        .wrapping_add(count.wrapping_mul(0xc + 4))
        .wrapping_add(0x20);
    e.with_stack(block_size, |e, block| {
        let ebp = block.addr().wrapping_add(block_size);
        let center = ebp - 0xc;
        let difference = ebp - 0x50;
        let positions = alloca16_esp(ebp - SORT_FIXED_FRAME, count.wrapping_mul(0xc));
        for word in 0..3 {
            let value: u32 = e.global(SORT_CENTER_START + word * 4);
            e.mem.set_u32(center + word * 4, value);
        }
        for index in 0..count {
            let entry = e.call(ARRAY_ELEMENT, &args![ranged, index]).u32();
            let actor = e.mem.u32(entry);
            let position = e.vcall(actor, ACTOR_SLOT_POSITION, &[]).u32();
            let slot = positions + index * 0xc;
            for word in 0..3 {
                let value = e.mem.u32(position + word * 4);
                e.mem.set_u32(slot + word * 4, value);
            }
            e.call(POINT_ADD, &args![center, slot]);
        }
        e.call(POINT_DIVIDE, &args![center, (count as f32).to_bits()]);
        let distances = alloca16_esp(positions, count.wrapping_mul(4));
        for index in 0..count {
            let slot = positions + index * 0xc;
            let result = e
                .call(POINT_SUBTRACT, &args![slot, difference, center])
                .u32();
            let length = e.call(POINT_LENGTH_SQUARED, &args![result]).f32();
            e.mem.set_f32(distances + index * 4, length);
        }
        let distance = |e: &Engine, i: u32| e.mem.u32(distances + i * 4);
        let mut first = 0u32;
        while first < count.wrapping_sub(1) {
            let mut nearest = first;
            let mut nearest_distance = f32::from_bits(x87_quiet_bits(distance(e, first)));
            let mut other = first + 1;
            while other < count {
                let value = f32::from_bits(distance(e, other));
                if value < nearest_distance {
                    nearest = other;
                    nearest_distance = x87_quiet(value);
                }
                other += 1;
            }
            if first != nearest {
                let saved = x87_quiet_bits(distance(e, first));
                let moved = x87_quiet_bits(distance(e, nearest));
                e.mem.set_u32(distances + first * 4, moved);
                e.mem.set_u32(distances + nearest * 4, saved);
                let a = e.call(ARRAY_ELEMENT_REF, &args![ranged, first]).u32();
                let first_entry = (e.mem.u32(a), e.mem.u32(a + 4));
                let b = e.call(ARRAY_ELEMENT_REF, &args![ranged, nearest]).u32();
                let nearest_entry = (e.mem.u32(b), e.mem.u32(b + 4));
                let a = e.call(ARRAY_ELEMENT_REF, &args![ranged, first]).u32();
                e.mem.set_u32(a, nearest_entry.0);
                e.mem.set_u32(a + 4, nearest_entry.1);
                let b = e.call(ARRAY_ELEMENT_REF, &args![ranged, nearest]).u32();
                e.mem.set_u32(b, first_entry.0);
                e.mem.set_u32(b + 4, first_entry.1);
            }
            first += 1;
        }
    });
}

// Translated from 00990890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size of the `BSSimpleArray` at +0x18 (`0044ddc0`): a `CombatGroup`'s
/// members, or a `CombatGroupData`'s `RangedMembers`; both kinds of caller
/// use it.
pub fn fn_00990890(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ARRAY_SIZE, &args![this.addr() + 0x18]).u32()
}

// Translated from 009908b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyFlank::CanExecute` (Xbox PDB): at least three
/// members and one ranged member, the group allows strategy 2 (`0098a470`
/// with 30.0), the maximum target detection is above 0 and the members'
/// targets do not intersect.
pub fn combat_group_strategy_flank_can_execute(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
    data: Ptr<CombatGroupData>,
) -> bool {
    if e.call(DATA_MEMBER_COUNT_GET, &args![data]).i32() < 3 {
        return false;
    }
    if (fn_00990890(e, data.cast()) as i32) < 1 {
        return false;
    }
    let time = pushed_float(e, TIME_FLANK_FALL_BACK);
    if !e
        .call(GROUP_STRATEGY_ALLOWED, &args![group, 2u32, time])
        .bool()
    {
        return false;
    }
    if fn_00990570(e, data) <= 0 {
        return false;
    }
    !combat_group_data_get_members_targets_intersecting(e, data)
}

// Translated from 00990930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyFlank::Execute` (Xbox PDB): every melee member gets
/// assignment 1; when the ranged share of the members is above 0.5, the
/// ranged members are sorted by squared distance to their centre and the farthest
/// `trunc((share - 0.5) * ranged)` of them get assignment 1 too.
pub fn combat_group_strategy_flank_execute(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
    data: Ptr<CombatGroupData>,
) {
    let melee = fn_00990a60(e, data);
    for index in 0..melee {
        let entry = fn_00990a80(e, data, index).addr();
        let member = e.mem.u32(entry + 4);
        e.call(GROUP_SET_ASSIGNMENT, &args![group, member, 1u32]);
    }
    let ranged = fn_00990890(e, data.cast());
    let total = e.call(DATA_MEMBER_COUNT_GET, &args![data]).u32();
    let share = (ranged as f64 / total as f64) as f32;
    let limit: f64 = e.global(FLANK_RANGED_SHARE);
    let mut flankers = 0u32;
    if share as f64 > limit {
        flankers = fistp_truncate_low((share as f64 - limit) * ranged as f64);
    }
    if flankers != 0 {
        combat_group_data_sort_ranged_members(e, data);
        let mut index = ranged.wrapping_sub(flankers);
        while index < ranged {
            let entry = fn_00990aa0(e, data, index).addr();
            let member = e.mem.u32(entry + 4);
            e.call(GROUP_SET_ASSIGNMENT, &args![group, member, 1u32]);
            index += 1;
        }
    }
}

// Translated from 00990a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Size of `CombatGroupData::MeleeMembers` (`0044ddc0`).
pub fn fn_00990a60(e: &mut Engine, this: Ptr<CombatGroupData>) -> u32 {
    e.call(ARRAY_SIZE, &args![this.addr() + DATA_MELEE_MEMBERS])
        .u32()
}

// Translated from 00990a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element `index` of `CombatGroupData::MeleeMembers` (`00844160`).
pub fn fn_00990a80(e: &mut Engine, this: Ptr<CombatGroupData>, index: u32) -> Ptr {
    e.call(
        ARRAY_ELEMENT,
        &args![this.addr() + DATA_MELEE_MEMBERS, index],
    )
    .ptr()
}

// Translated from 00990aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Element `index` of `CombatGroupData::RangedMembers` (`00844160`).
pub fn fn_00990aa0(e: &mut Engine, this: Ptr<CombatGroupData>, index: u32) -> Ptr {
    e.call(
        ARRAY_ELEMENT,
        &args![this.addr() + DATA_RANGED_MEMBERS, index],
    )
    .ptr()
}

// Translated from 00990ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyFlank::Update` (Xbox PDB): for each member with
/// assignment 1, the group checks the actor's position (`00575650` into a
/// local point, then `00989700` with 384.0 and 0); members that pass get
/// assignment 0. Returns whether any assignment-1 member failed the check.
pub fn combat_group_strategy_flank_update(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
) -> bool {
    let mut still_flanking = false;
    let count = fn_00990890(e, group.cast());
    for index in 0..count {
        if e.call(GROUP_MEMBER_ASSIGNMENT, &args![group, index]).u32() != 1 {
            continue;
        }
        let actor = e.call(GROUP_MEMBER_ACTOR, &args![group, index]).u32();
        let distance = pushed_float(e, FLANK_UPDATE_DISTANCE);
        let passed = e.with_stack(0xc, |e, point| {
            let position = e.call(ACTOR_POSITION_INTO, &args![actor, point]).u32();
            e.call(
                GROUP_POSITION_CHECK,
                &args![group, position, distance, 0u32],
            )
            .u32()
        });
        if passed != 0 {
            e.call(GROUP_SET_ASSIGNMENT, &args![group, index, 0u32]);
        } else {
            still_flanking = true;
        }
    }
    still_flanking
}

// Translated from 00990b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyFallBack::CanExecute` (Xbox PDB): at least two
/// members, the group allows strategy 0 (`0098a470` with 30.0) and the
/// members' health share is not above the limit (NaN passes).
pub fn combat_group_strategy_fall_back_can_execute(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
    data: Ptr<CombatGroupData>,
) -> bool {
    if e.call(DATA_MEMBER_COUNT_GET, &args![data]).i32() < 2 {
        return false;
    }
    let time = pushed_float(e, TIME_FLANK_FALL_BACK);
    if !e
        .call(GROUP_STRATEGY_ALLOWED, &args![group, 0u32, time])
        .bool()
    {
        return false;
    }
    let health = e.call(DATA_HEALTH_PERCENTAGE, &args![data]).f64();
    let limit: f64 = e.global(FALL_BACK_HEALTH);
    health <= limit || health.is_nan()
}

// Translated from 00990bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyFallBack::Execute` (Xbox PDB): every group member
/// gets assignment 2 (the member count is read again on every pass).
pub fn combat_group_strategy_fall_back_execute(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
    _data: Ptr<CombatGroupData>,
) {
    let mut index = 0u32;
    while index < fn_00990890(e, group.cast()) {
        e.call(GROUP_SET_ASSIGNMENT, &args![group, index, 2u32]);
        index += 1;
    }
}

// Translated from 00990c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyFallBack::Update` (Xbox PDB): whether a member still
/// has assignment 2.
pub fn combat_group_strategy_fall_back_update(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
) -> bool {
    e.call(GROUP_FIND_ASSIGNMENT, &args![group, 2u32]).u32() != 0
}

// Translated from 00990c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyAmbush::CanExecute` (Xbox PDB): at least two
/// members, the group allows strategy 3 (`0098a470` with 60.0), the maximum
/// target detection is above 0 and the members' targets do not intersect.
pub fn combat_group_strategy_ambush_can_execute(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
    data: Ptr<CombatGroupData>,
) -> bool {
    if e.call(DATA_MEMBER_COUNT_GET, &args![data]).i32() < 2 {
        return false;
    }
    let time = pushed_float(e, TIME_AMBUSH_ADVANCE);
    if !e
        .call(GROUP_STRATEGY_ALLOWED, &args![group, 3u32, time])
        .bool()
    {
        return false;
    }
    if fn_00990570(e, data) <= 0 {
        return false;
    }
    !combat_group_data_get_members_targets_intersecting(e, data)
}

// Translated from 00990cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyAmbush::Execute` (Xbox PDB): the first half of the
/// group's members (count / 2, rounded down) get assignment 2, the rest
/// assignment 3 (the count is read again on every pass of the second loop).
pub fn combat_group_strategy_ambush_execute(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
    _data: Ptr<CombatGroupData>,
) {
    let half = fn_00990890(e, group.cast()) >> 1;
    for index in 0..half {
        e.call(GROUP_SET_ASSIGNMENT, &args![group, index, 2u32]);
    }
    let mut index = half;
    while index < fn_00990890(e, group.cast()) {
        e.call(GROUP_SET_ASSIGNMENT, &args![group, index, 3u32]);
        index += 1;
    }
}

// Translated from 00990d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyAmbush::Update` (Xbox PDB): whether a member still
/// has assignment 3.
pub fn combat_group_strategy_ambush_update(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
) -> bool {
    e.call(GROUP_FIND_ASSIGNMENT, &args![group, 3u32]).u32() != 0
}

// Translated from 00990d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyAdvance::CanExecute` (Xbox PDB): at least two ranged
/// members, the group allows strategy 1 (`0098a470` with 60.0), the
/// minimum target detection is above 0, and no ranged member with a
/// combat target (actor slot 0x428, then `004030b0`) has a squared distance to it
/// below 1024 squared (NaN passes).
pub fn combat_group_strategy_advance_can_execute(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
    data: Ptr<CombatGroupData>,
) -> bool {
    if (fn_00990890(e, data.cast()) as i32) < 2 {
        return false;
    }
    let time = pushed_float(e, TIME_AMBUSH_ADVANCE);
    if !e
        .call(GROUP_STRATEGY_ALLOWED, &args![group, 1u32, time])
        .bool()
    {
        return false;
    }
    if fn_00990540(e, data) <= 0 {
        return false;
    }
    let count = fn_00990890(e, data.cast());
    for index in 0..count {
        let entry = fn_00990aa0(e, data, index).addr();
        let actor = e.mem.u32(entry);
        let combat = e.vcall(actor, ACTOR_SLOT_COMBAT, &[]).u32();
        let target = e.call(COMBAT_TARGET_OF, &args![combat]).u32();
        if target == 0 {
            continue;
        }
        let target_position = e.vcall(target, ACTOR_SLOT_POSITION, &[]).u32();
        let actor = e.mem.u32(entry);
        let actor_position = e.vcall(actor, ACTOR_SLOT_POSITION, &[]).u32();
        let distance = e.with_stack(0xc, |e, difference| {
            e.call(
                POINT_SUBTRACT,
                &args![actor_position, difference, target_position],
            );
            e.call(POINT_LENGTH_SQUARED, &args![difference]).f64()
        });
        let limit: f64 = e.global(ADVANCE_DISTANCE);
        if distance < limit {
            return false;
        }
    }
    true
}

// Translated from 00990e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyAdvance::Execute` (Xbox PDB): every ranged member
/// gets assignment 4 (signed loop over the ranged count, read again on
/// every pass).
pub fn combat_group_strategy_advance_execute(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
    data: Ptr<CombatGroupData>,
) {
    let mut index = 0i32;
    while index < fn_00990890(e, data.cast()) as i32 {
        let entry = fn_00990aa0(e, data, index as u32).addr();
        let member = e.mem.u32(entry + 4);
        e.call(GROUP_SET_ASSIGNMENT, &args![group, member, 4u32]);
        index += 1;
    }
}

// Translated from 00990ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CombatGroupStrategyAdvance::Update` (Xbox PDB): whether a member still
/// has assignment 4.
pub fn combat_group_strategy_advance_update(
    e: &mut Engine,
    _this: Ptr<CombatGroupStrategy>,
    group: Ptr<CombatGroup>,
) -> bool {
    e.call(GROUP_FIND_ASSIGNMENT, &args![group, 4u32]).u32() != 0
}

// Translated from 00990ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<GroupMemberData>` constructor: its vtable, then
/// `006dc440` with (0, 0).
pub fn fn_00990ef0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_MEMBER_ARRAY);
    e.call(ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0098fdf0, fn_0098fdf0(u32) -> Ptr<CombatGroupStrategy>),
        entry!(
            0x0098ff80,
            fn_0098ff80(Ptr<CombatGroupStrategy>, u32) -> Ptr<CombatGroupStrategy>
        ),
        entry!(
            0x0098ffb0,
            fn_0098ffb0(Ptr<CombatGroupStrategy>, u32) -> Ptr<CombatGroupStrategy>
        ),
        entry!(
            0x0098ffe0,
            fn_0098ffe0(Ptr<CombatGroupStrategy>, u32) -> Ptr<CombatGroupStrategy>
        ),
        entry!(
            0x00990010,
            fn_00990010(Ptr<CombatGroupStrategy>, u32) -> Ptr<CombatGroupStrategy>
        ),
        entry!(
            0x00990040,
            fn_00990040(Ptr<CombatGroupStrategy>, u32) -> Ptr<CombatGroupStrategy>
        ),
        entry!(0x00990070, fn_00990070()),
        entry!(0x009900b0, fn_009900b0()),
        entry!(
            0x009900f0,
            fn_009900f0(Ptr<CombatGroup>) -> Ptr<CombatGroupStrategy>
        ),
        entry!(
            0x00990250,
            combat_group_strategy_get_combat_group_strategy_by_name(Ptr) -> Ptr<CombatGroupStrategy>
        ),
        entry!(
            0x00990320,
            fn_00990320(Ptr<CombatGroupData>, Ptr<CombatGroup>) -> Ptr<CombatGroupData>
        ),
        entry!(0x00990480, fn_00990480(Ptr) -> bool),
        entry!(
            0x009904c0,
            combat_group_data_get_members_targets_intersecting(Ptr<CombatGroupData>) -> bool
        ),
        entry!(0x00990540, fn_00990540(Ptr<CombatGroupData>) -> i32),
        entry!(0x00990570, fn_00990570(Ptr<CombatGroupData>) -> i32),
        entry!(0x009905a0, fn_009905a0(Ptr<CombatGroupData>)),
        entry!(
            0x00990640,
            combat_group_data_sort_ranged_members(Ptr<CombatGroupData>)
        ),
        entry!(0x00990890, fn_00990890(Ptr) -> u32),
        entry!(
            0x009908b0,
            combat_group_strategy_flank_can_execute(
                Ptr<CombatGroupStrategy>,
                Ptr<CombatGroup>,
                Ptr<CombatGroupData>,
            ) -> bool
        ),
        entry!(
            0x00990930,
            combat_group_strategy_flank_execute(
                Ptr<CombatGroupStrategy>,
                Ptr<CombatGroup>,
                Ptr<CombatGroupData>,
            )
        ),
        entry!(0x00990a60, fn_00990a60(Ptr<CombatGroupData>) -> u32),
        entry!(0x00990a80, fn_00990a80(Ptr<CombatGroupData>, u32) -> Ptr),
        entry!(0x00990aa0, fn_00990aa0(Ptr<CombatGroupData>, u32) -> Ptr),
        entry!(
            0x00990ac0,
            combat_group_strategy_flank_update(Ptr<CombatGroupStrategy>, Ptr<CombatGroup>) -> bool
        ),
        entry!(
            0x00990b70,
            combat_group_strategy_fall_back_can_execute(
                Ptr<CombatGroupStrategy>,
                Ptr<CombatGroup>,
                Ptr<CombatGroupData>,
            ) -> bool
        ),
        entry!(
            0x00990bd0,
            combat_group_strategy_fall_back_execute(
                Ptr<CombatGroupStrategy>,
                Ptr<CombatGroup>,
                Ptr<CombatGroupData>,
            )
        ),
        entry!(
            0x00990c10,
            combat_group_strategy_fall_back_update(
                Ptr<CombatGroupStrategy>,
                Ptr<CombatGroup>,
            ) -> bool
        ),
        entry!(
            0x00990c40,
            combat_group_strategy_ambush_can_execute(
                Ptr<CombatGroupStrategy>,
                Ptr<CombatGroup>,
                Ptr<CombatGroupData>,
            ) -> bool
        ),
        entry!(
            0x00990cb0,
            combat_group_strategy_ambush_execute(
                Ptr<CombatGroupStrategy>,
                Ptr<CombatGroup>,
                Ptr<CombatGroupData>,
            )
        ),
        entry!(
            0x00990d30,
            combat_group_strategy_ambush_update(Ptr<CombatGroupStrategy>, Ptr<CombatGroup>) -> bool
        ),
        entry!(
            0x00990d60,
            combat_group_strategy_advance_can_execute(
                Ptr<CombatGroupStrategy>,
                Ptr<CombatGroup>,
                Ptr<CombatGroupData>,
            ) -> bool
        ),
        entry!(
            0x00990e70,
            combat_group_strategy_advance_execute(
                Ptr<CombatGroupStrategy>,
                Ptr<CombatGroup>,
                Ptr<CombatGroupData>,
            )
        ),
        entry!(
            0x00990ec0,
            combat_group_strategy_advance_update(
                Ptr<CombatGroupStrategy>,
                Ptr<CombatGroup>,
            ) -> bool
        ),
        entry!(0x00990ef0, fn_00990ef0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    /// Vtable of the test actors: slot 0x428 returns the word at +4 (the
    /// combat object), slot 0x1f4 the address +8 (the position).
    const ACTOR_VTABLE: u32 = 0x0200_0000;
    const ACTOR_COMBAT_DOUBLE: u32 = 0x7000_0000;
    const ACTOR_POSITION_DOUBLE: u32 = 0x7000_0010;
    /// Vtable of the test strategies: `CanExecute` returns the byte at +4,
    /// `Execute` does nothing.
    const STRATEGY_VTABLE: u32 = 0x0200_1000;
    const CAN_EXECUTE_DOUBLE: u32 = 0x7000_0020;
    const EXECUTE_DOUBLE: u32 = 0x7000_0030;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A double that always returns `value` in EAX.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| eax(value));
    }

    /// A double that does nothing.
    fn ignore(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    /// The constants the code reads, the test vtables, and doubles of the
    /// `BSSimpleArray` accessors working on a real buffer (capacity 16,
    /// `pBuffer` at +4, `iSize` at +8), of the group's member accessor
    /// (actor of entry `i` of the group's array at +0x18) and of the
    /// combat-object call (returns its argument).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_8000,
            0x0101_e000,
            0x0102_6000,
            0x0103_4000,
            0x0105_0000,
            0x011a_4000,
            0x011f_1000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(TIME_FLANK_FALL_BACK, 30.0f32);
        e.set_global(TIME_AMBUSH_ADVANCE, 60.0f32);
        e.set_global(FLANK_RANGED_SHARE, 0.5f64);
        e.set_global(FALL_BACK_HEALTH, 0.33f64);
        e.set_global(ADVANCE_DISTANCE, 1_048_576.0f64);
        e.set_global(FLANK_UPDATE_DISTANCE, 384.0f32);

        e.map(ACTOR_VTABLE, 0x1000);
        e.mem
            .set_u32(ACTOR_VTABLE + ACTOR_SLOT_COMBAT, ACTOR_COMBAT_DOUBLE);
        e.mem
            .set_u32(ACTOR_VTABLE + ACTOR_SLOT_POSITION, ACTOR_POSITION_DOUBLE);
        e.register(ACTOR_COMBAT_DOUBLE, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(ACTOR_POSITION_DOUBLE, |_, a| eax(a[0] + 8));
        e.put_vtable(STRATEGY_VTABLE, &[CAN_EXECUTE_DOUBLE, EXECUTE_DOUBLE, 0]);
        e.register(CAN_EXECUTE_DOUBLE, |e, a| eax(e.mem.u8(a[0] + 4) as u32));
        ignore(&mut e, EXECUTE_DOUBLE);

        e.register(ARRAY_INIT, |e, a| {
            let buffer = e.mem.alloc(16 * 8);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.register(ARRAY_SIZE, |e, a| eax(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_ELEMENT, |e, a| eax(e.mem.u32(a[0] + 4) + a[1] * 8));
        e.register(
            ARRAY_ELEMENT_REF,
            |e, a| eax(e.mem.u32(a[0] + 4) + a[1] * 8),
        );
        e.register(ARRAY_APPEND, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let slot = e.mem.u32(a[0] + 4) + size * 8;
            let (actor, index) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            e.mem.set_u32(slot, actor);
            e.mem.set_u32(slot + 4, index);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(MEMBER_DATA_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            eax(a[0])
        });
        e.register(GROUP_MEMBER_ACTOR, |e, a| {
            eax(e.mem.u32(e.mem.u32(a[0] + 0x18 + 4) + a[1] * 8))
        });
        e.register(MEMBER_COMBAT_OBJECT, |_, a| eax(a[0]));
        e.call_log = Some(vec![]);
        e
    }

    /// Fills the array at `array` with `(actor, index)` entries.
    fn fill(e: &mut Engine, array: u32, entries: &[(u32, u32)]) {
        let buffer = e.mem.alloc(16 * 8);
        for (i, (actor, index)) in entries.iter().enumerate() {
            e.mem.set_u32(buffer + i as u32 * 8, *actor);
            e.mem.set_u32(buffer + i as u32 * 8 + 4, *index);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, entries.len() as u32);
    }

    /// An actor with a combat object (+4) and a position (+8).
    fn actor(e: &mut Engine, combat: u32, position: [f32; 3]) -> u32 {
        let actor = e.mem.alloc(0x20);
        e.mem.set_u32(actor, ACTOR_VTABLE);
        e.mem.set_u32(actor + 4, combat);
        for (i, value) in position.iter().enumerate() {
            e.mem.set_f32(actor + 8 + i as u32 * 4, *value);
        }
        actor
    }

    /// A member combat object: excluded byte at +0, detection at +0x10,
    /// the two floats `fn_00990480` compares at +0x58 / +0x5c.
    fn combat_object(e: &mut Engine, excluded: bool, detection: i32, melee: bool) -> u32 {
        let object = e.mem.alloc(0x60);
        e.mem.set_u8(object, excluded as u8);
        e.mem.set_u32(object + 0x10, detection as u32);
        e.mem.set_f32(object + 0x58, 2.0);
        e.mem.set_f32(object + 0x5c, if melee { 1.0 } else { 3.0 });
        object
    }

    fn group_with(e: &mut Engine, actors: &[u32]) -> Ptr<CombatGroup> {
        let group = e.new_object::<CombatGroup>();
        let entries: Vec<(u32, u32)> = actors.iter().map(|a| (*a, 0)).collect();
        fill(e, group.addr() + 0x18, &entries);
        group
    }

    fn data_for(e: &mut Engine, group: Ptr<CombatGroup>) -> Ptr<CombatGroupData> {
        let data = e.new_object::<CombatGroupData>();
        e.set(data, CombatGroupData::pCombatGroup, group);
        e.mem
            .set_u32(data.addr() + DATA_TARGETS_INTERSECTING, u32::MAX);
        e.mem
            .set_u32(data.addr() + DATA_MIN_DETECTION, i32::MAX as u32);
        e.mem
            .set_u32(data.addr() + DATA_MAX_DETECTION, i32::MIN as u32);
        fill(e, data.addr() + DATA_RANGED_MEMBERS, &[]);
        fill(e, data.addr() + DATA_MELEE_MEMBERS, &[]);
        data
    }

    fn strategy(e: &mut Engine, can_execute: bool) -> u32 {
        let strategy = e.mem.alloc(8);
        e.mem.set_u32(strategy, STRATEGY_VTABLE);
        e.mem.set_u8(strategy + 4, can_execute as u8);
        strategy
    }

    fn cstr(e: &mut Engine, text: &str) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(at, text.as_bytes());
        e.mem.set_u8(at + text.len() as u32, 0);
        at
    }

    fn read_cstr(e: &Engine, mut at: u32) -> Vec<u8> {
        let mut out = vec![];
        while e.mem.u8(at) != 0 {
            out.push(e.mem.u8(at));
            at += 1;
        }
        out
    }

    fn allocating(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| eax(e.mem.alloc(a[0])));
    }

    #[test]
    fn create_strategy_by_type() {
        let mut e = engine();
        allocating(&mut e);
        ignore(&mut e, PRINT_MESSAGE);
        for (kind, vtable) in [
            (0, VTABLE_FALL_BACK),
            (1, VTABLE_ADVANCE),
            (2, VTABLE_FLANK),
            (3, VTABLE_AMBUSH),
        ] {
            let s = fn_0098fdf0(&mut e, kind);
            assert_eq!(e.mem.u32(s.addr()), vtable);
            assert_eq!(e.get(s, CombatGroupStrategy::eType), kind);
        }
        assert_eq!(calls(&e, ALLOCATE), vec![vec![8]; 4]);
        assert!(fn_0098fdf0(&mut e, 7).is_null());
        assert_eq!(
            calls(&e, PRINT_MESSAGE),
            vec![vec![CREATE_FAILED_FORMAT, 7]]
        );
        assert_eq!(calls(&e, ALLOCATE).len(), 4);
        returns(&mut e, ALLOCATE, 0);
        assert!(fn_0098fdf0(&mut e, 2).is_null());
    }

    fn check_constructor(construct: StrategyConstructor, vtable: u32) {
        let mut e = engine();
        let s = e.new_object::<CombatGroupStrategy>();
        assert_eq!(construct(&mut e, s, 9), s);
        assert_eq!(e.mem.u32(s.addr()), vtable);
        assert_eq!(e.get(s, CombatGroupStrategy::eType), 9);
    }

    #[test]
    fn flank_constructor() {
        check_constructor(fn_0098ff80, VTABLE_FLANK);
    }

    #[test]
    fn base_constructor() {
        check_constructor(fn_0098ffb0, VTABLE_STRATEGY);
    }

    #[test]
    fn fall_back_constructor() {
        check_constructor(fn_0098ffe0, VTABLE_FALL_BACK);
    }

    #[test]
    fn ambush_constructor() {
        check_constructor(fn_00990010, VTABLE_AMBUSH);
    }

    #[test]
    fn advance_constructor() {
        check_constructor(fn_00990040, VTABLE_ADVANCE);
    }

    #[test]
    fn create_all_strategies_fills_the_table() {
        let mut e = engine();
        allocating(&mut e);
        fn_00990070(&mut e);
        let vtables: Vec<u32> = (0..4)
            .map(|i| {
                let s: u32 = e.global(STRATEGIES + i * 4);
                e.mem.u32(s)
            })
            .collect();
        assert_eq!(
            vtables,
            vec![
                VTABLE_FALL_BACK,
                VTABLE_ADVANCE,
                VTABLE_FLANK,
                VTABLE_AMBUSH
            ]
        );
    }

    #[test]
    fn free_all_strategies() {
        let mut e = engine();
        ignore(&mut e, DEALLOCATE);
        for i in 0..4 {
            e.set_global(STRATEGIES + i * 4, 0x100 + i);
        }
        fn_009900b0(&mut e);
        assert_eq!(
            calls(&e, DEALLOCATE),
            vec![vec![0x100], vec![0x101], vec![0x102], vec![0x103]]
        );
    }

    /// The choice's doubles: disabled byte, current strategy, member count
    /// and the strategy table (filled by `table` on the same engine).
    fn choice_engine(
        disabled: bool,
        members: u32,
        table: impl FnOnce(&mut Engine) -> (u32, [u32; 4]),
    ) -> (Engine, [u32; 4]) {
        let mut e = engine();
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, disabled as u8);
        returns(&mut e, CHOICE_DISABLED, flag);
        returns(&mut e, DATA_MEMBER_COUNT_GET, members);
        let (current, strategies) = table(&mut e);
        returns(&mut e, GROUP_CURRENT_STRATEGY, current);
        e.register_double(STRATEGY_AT, move |_, a| eax(strategies[a[0] as usize]));
        ignore(&mut e, DATA_DESTROY);
        ignore(&mut e, GROUP_BEFORE_EXECUTE);
        (e, strategies)
    }

    #[test]
    fn choose_strategy() {
        // Disabled: nothing is built.
        let (mut e, _) = choice_engine(true, 3, |_| (0, [0; 4]));
        let group = group_with(&mut e, &[]);
        assert!(fn_009900f0(&mut e, group).is_null());
        assert!(calls(&e, ARRAY_INIT).is_empty());
        assert_eq!(
            calls(&e, CHOICE_DISABLED),
            vec![vec![CHOICE_DISABLED_HOLDER]]
        );

        // Fewer than two members.
        let (mut e, _) = choice_engine(false, 1, |_| (0, [0; 4]));
        let group = group_with(&mut e, &[]);
        assert!(fn_009900f0(&mut e, group).is_null());
        assert_eq!(calls(&e, DATA_DESTROY).len(), 1);
        assert!(calls(&e, STRATEGY_AT).is_empty());

        // The first strategy that can execute runs.
        let (mut e, table) = choice_engine(false, 2, |e| {
            let refuses = strategy(e, false);
            let accepts = strategy(e, true);
            let later = strategy(e, true);
            (0, [0, refuses, accepts, later])
        });
        let accepts = table[2];
        let group = group_with(&mut e, &[]);
        assert_eq!(fn_009900f0(&mut e, group).addr(), accepts);
        let can = calls(&e, CAN_EXECUTE_DOUBLE);
        assert_eq!(can.len(), 2);
        assert_eq!(can[1][..2], [accepts, group.addr()]);
        let data = can[1][2];
        assert_eq!(calls(&e, GROUP_BEFORE_EXECUTE), vec![vec![group.addr()]]);
        assert_eq!(
            calls(&e, EXECUTE_DOUBLE),
            vec![vec![accepts, group.addr(), data]]
        );
        assert_eq!(calls(&e, DATA_DESTROY), vec![vec![data]]);

        // The current strategy is kept without asking it.
        let (mut e, table) = choice_engine(false, 2, |e| {
            let current = strategy(e, false);
            (current, [current, 0, 0, 0])
        });
        let group = group_with(&mut e, &[]);
        assert_eq!(fn_009900f0(&mut e, group).addr(), table[0]);
        assert!(calls(&e, CAN_EXECUTE_DOUBLE).is_empty());
        assert_eq!(calls(&e, DATA_DESTROY).len(), 1);

        // None can execute.
        let (mut e, _) = choice_engine(false, 2, |e| {
            let refuses = strategy(e, false);
            (0, [refuses; 4])
        });
        let group = group_with(&mut e, &[]);
        assert!(fn_009900f0(&mut e, group).is_null());
        assert_eq!(calls(&e, CAN_EXECUTE_DOUBLE).len(), 4);
        assert_eq!(calls(&e, DATA_DESTROY).len(), 1);
    }

    #[test]
    fn strategy_by_name() {
        let mut e = engine();
        for (i, name) in ["FALLBACK", "ADVANCE", "FLANK", "AMBUSH"]
            .iter()
            .enumerate()
        {
            let text = cstr(&mut e, name);
            e.set_global(STRATEGY_NAMES + i as u32 * 4, text);
        }
        e.register(STRING_COPY, |e, a| {
            let text = read_cstr(e, a[2]);
            e.mem.write(a[0], &text);
            e.mem.set_u8(a[0] + text.len() as u32, 0);
            Ret::default()
        });
        e.register(TO_UPPER, |_, a| {
            eax((a[0] as u8).to_ascii_uppercase() as u32)
        });
        e.register(STRING_EQUAL, |e, a| {
            eax((read_cstr(e, a[0]) == read_cstr(e, a[1])) as u32)
        });
        e.register(STRATEGY_AT, |_, a| eax(0x5000 + a[0]));
        let name = cstr(&mut e, "Flank");
        let found = combat_group_strategy_get_combat_group_strategy_by_name(&mut e, Ptr::new(name));
        assert_eq!(found.addr(), 0x5002);
        assert_eq!(calls(&e, STRATEGY_AT), vec![vec![2]]);
        assert_eq!(calls(&e, STRING_EQUAL).len(), 3);
        assert_eq!(calls(&e, STRING_COPY)[0][1..], [0x104, name]);
        assert_eq!(calls(&e, TO_UPPER).len(), 5);

        let name = cstr(&mut e, "charge");
        let found = combat_group_strategy_get_combat_group_strategy_by_name(&mut e, Ptr::new(name));
        assert!(found.is_null());
        assert_eq!(calls(&e, STRATEGY_AT).len(), 1);
    }

    #[test]
    fn group_data_constructor_sorts_members() {
        let mut e = engine();
        e.register(MEMBER_EXCLUDED, |e, a| eax(e.mem.u8(a[0]) as u32));
        let ranged = combat_object(&mut e, false, 0, false);
        let excluded = combat_object(&mut e, true, 0, true);
        let melee = combat_object(&mut e, false, 0, true);
        let a0 = actor(&mut e, ranged, [0.0; 3]);
        let a1 = actor(&mut e, excluded, [0.0; 3]);
        let a2 = actor(&mut e, melee, [0.0; 3]);
        let group = group_with(&mut e, &[a0, a1, a2]);
        let data = Ptr::<CombatGroupData>::new(e.mem.alloc(0x38));
        e.mem.set_f32(data.addr() + 0x14, 5.0);
        assert_eq!(fn_00990320(&mut e, data, group), data);
        let base = data.addr();
        assert_eq!(e.get(data, CombatGroupData::pCombatGroup), group);
        assert_eq!(e.mem.u32(base + DATA_MEMBER_COUNT), 2);
        assert_eq!(e.mem.u32(base + DATA_TARGETS_INTERSECTING), u32::MAX);
        assert_eq!(e.mem.u32(base + DATA_MIN_DETECTION), 0x7fff_ffff);
        assert_eq!(e.mem.u32(base + DATA_MAX_DETECTION), 0x8000_0000);
        assert_eq!(e.get(data, CombatGroupData::fMemberHealthPercentage), 0.0);
        for array in [DATA_RANGED_MEMBERS, DATA_MELEE_MEMBERS] {
            assert_eq!(e.mem.u32(base + array), VTABLE_MEMBER_ARRAY);
            assert_eq!(e.mem.u32(base + array + 8), 1);
        }
        let ranged_buffer = e.mem.u32(base + DATA_RANGED_MEMBERS + 4);
        assert_eq!(
            (e.mem.u32(ranged_buffer), e.mem.u32(ranged_buffer + 4)),
            (a0, 0)
        );
        let melee_buffer = e.mem.u32(base + DATA_MELEE_MEMBERS + 4);
        assert_eq!(
            (e.mem.u32(melee_buffer), e.mem.u32(melee_buffer + 4)),
            (a2, 2)
        );
        assert_eq!(
            calls(&e, MEMBER_EXCLUDED),
            vec![vec![ranged], vec![excluded], vec![melee]]
        );
    }

    #[test]
    fn melee_test_compares_the_two_floats() {
        let mut e = engine();
        let object = e.mem.alloc(0x60);
        let cases = [
            (2.0, 1.0, true),
            (1.0, 2.0, false),
            (1.0, 1.0, false),
            (f32::NAN, 1.0, false),
        ];
        for (first, second, melee) in cases {
            e.mem.set_f32(object + 0x58, first);
            e.mem.set_f32(object + 0x5c, second);
            assert_eq!(fn_00990480(&mut e, Ptr::new(object)), melee);
        }
    }

    #[test]
    fn members_targets_intersecting_is_cached() {
        let mut e = engine();
        returns(&mut e, CHECK_MEMBER_TARGET_INTERSECTION, 0x0000_0101);
        let group = group_with(&mut e, &[]);
        let data = data_for(&mut e, group);
        assert!(combat_group_data_get_members_targets_intersecting(
            &mut e, data
        ));
        assert_eq!(e.mem.u32(data.addr() + DATA_TARGETS_INTERSECTING), 1);
        assert!(combat_group_data_get_members_targets_intersecting(
            &mut e, data
        ));
        assert_eq!(
            calls(&e, CHECK_MEMBER_TARGET_INTERSECTION),
            vec![vec![group.addr()]]
        );

        // The low byte decides: 0x100 is false.
        returns(&mut e, CHECK_MEMBER_TARGET_INTERSECTION, 0x100);
        e.mem
            .set_u32(data.addr() + DATA_TARGETS_INTERSECTING, u32::MAX);
        assert!(!combat_group_data_get_members_targets_intersecting(
            &mut e, data
        ));
        assert_eq!(e.mem.u32(data.addr() + DATA_TARGETS_INTERSECTING), 0);
    }

    /// A group of three actors: no combat object, detection 5, detection 2.
    fn detection_group(e: &mut Engine) -> Ptr<CombatGroupData> {
        e.register(TARGET_DETECTION, |e, a| eax(e.mem.u32(a[0] + 0x10)));
        let five = combat_object(e, false, 5, false);
        let two = combat_object(e, false, 2, false);
        let a0 = actor(e, 0, [0.0; 3]);
        let a1 = actor(e, five, [0.0; 3]);
        let a2 = actor(e, two, [0.0; 3]);
        let group = group_with(e, &[a0, a1, a2]);
        data_for(e, group)
    }

    #[test]
    fn minimum_target_detection_is_computed_once() {
        let mut e = engine();
        let data = detection_group(&mut e);
        assert_eq!(fn_00990540(&mut e, data), 2);
        assert_eq!(calls(&e, TARGET_DETECTION).len(), 2);
        assert_eq!(fn_00990540(&mut e, data), 2);
        assert_eq!(calls(&e, TARGET_DETECTION).len(), 2);
    }

    #[test]
    fn maximum_target_detection_keeps_the_smaller_value() {
        let mut e = engine();
        let data = detection_group(&mut e);
        // The exe compares `<` for the maximum too: it stays `i32::MIN`
        // and is computed again on every call.
        assert_eq!(fn_00990570(&mut e, data), i32::MIN);
        assert_eq!(fn_00990570(&mut e, data), i32::MIN);
        assert_eq!(calls(&e, TARGET_DETECTION).len(), 4);
        e.mem.set_u32(data.addr() + DATA_MAX_DETECTION, 9);
        assert_eq!(fn_00990570(&mut e, data), 9);
        assert_eq!(calls(&e, TARGET_DETECTION).len(), 4);
    }

    #[test]
    fn compute_target_detection_skips_members_without_combat_object() {
        let mut e = engine();
        let data = detection_group(&mut e);
        e.mem.set_u32(data.addr() + DATA_MIN_DETECTION, 3);
        e.mem.set_u32(data.addr() + DATA_MAX_DETECTION, 4);
        fn_009905a0(&mut e, data);
        assert_eq!(e.mem.u32(data.addr() + DATA_MIN_DETECTION), 2);
        assert_eq!(e.mem.u32(data.addr() + DATA_MAX_DETECTION), 2);
        assert_eq!(calls(&e, MEMBER_COMBAT_OBJECT).len(), 2);
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn set_point(e: &mut Engine, at: u32, value: [f32; 3]) {
        for (i, v) in value.iter().enumerate() {
            e.mem.set_f32(at + i as u32 * 4, *v);
        }
    }

    /// Point doubles on three `float`s.
    fn point_doubles(e: &mut Engine) {
        e.register(POINT_ADD, |e, a| {
            let (p, q) = (point(e, a[0]), point(e, a[1]));
            set_point(e, a[0], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            Ret::default()
        });
        e.register(POINT_DIVIDE, |e, a| {
            let (p, d) = (point(e, a[0]), f32::from_bits(a[1]));
            set_point(e, a[0], [p[0] / d, p[1] / d, p[2] / d]);
            Ret::default()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            let (p, q) = (point(e, a[0]), point(e, a[2]));
            set_point(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            eax(a[1])
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            let p = point(e, a[0]);
            Ret {
                st0: (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]) as f64,
                ..Ret::default()
            }
        });
    }

    /// Ranged members at x = 0, 1, 2, 10 (member indices 20 to 23).
    fn ranged_data(e: &mut Engine) -> Ptr<CombatGroupData> {
        point_doubles(e);
        let actors = [0.0f32, 1.0, 2.0, 10.0].map(|x| actor(e, 0, [x, 0.0, 0.0]));
        let group = group_with(e, &[]);
        let data = data_for(e, group);
        let entries: Vec<(u32, u32)> = actors.iter().zip(20..).map(|(a, i)| (*a, i)).collect();
        fill(e, data.addr() + DATA_RANGED_MEMBERS, &entries);
        data
    }

    fn ranged_order(e: &Engine, data: Ptr<CombatGroupData>) -> Vec<u32> {
        let buffer = e.mem.u32(data.addr() + DATA_RANGED_MEMBERS + 4);
        let size = e.mem.u32(data.addr() + DATA_RANGED_MEMBERS + 8);
        (0..size).map(|i| e.mem.u32(buffer + i * 8 + 4)).collect()
    }

    #[test]
    fn sort_ranged_members_by_distance_to_centre() {
        let mut e = engine();
        let data = ranged_data(&mut e);
        combat_group_data_sort_ranged_members(&mut e, data);
        // Centre x = 3.25: squared distances 10.5625, 5.0625, 1.5625, 45.5625.
        assert_eq!(ranged_order(&e, data), vec![22, 21, 20, 23]);
        assert_eq!(calls(&e, POINT_DIVIDE)[0][1], 4.0f32.to_bits());
        // One swap (first and third), four element lookups.
        assert_eq!(calls(&e, ARRAY_ELEMENT_REF).len(), 4);
    }

    #[test]
    fn sort_ranged_members_places_its_alloca16_tables_as_the_exe_does() {
        let mut e = engine();
        let data = ranged_data(&mut e);
        // Four members: the fixed 0x6c bytes, 4 * 12 + 4 * 4 bytes of tables and
        // both pads. The frame is the next local on the thread's stack (its bytes stay there
        // after the call returns).
        let size = SORT_FIXED_FRAME + 4 * 16 + 0x20;
        let block = e.next_stack_local(size);
        let ebp = block + size;
        combat_group_data_sort_ranged_members(&mut e, data);
        // `_alloca16` from ESP = EBP-0x6c, then from the positions table.
        let positions = (ebp - 0x6c - 0x30) & !0xf;
        let distances = (positions - 0x10) & !0xf;
        assert!(distances >= block);
        let xs: Vec<f32> = (0..4).map(|i| e.mem.f32(positions + 12 * i)).collect();
        assert_eq!(xs, vec![0.0, 1.0, 2.0, 10.0]);
        let sorted: Vec<f32> = (0..4).map(|i| e.mem.f32(distances + 4 * i)).collect();
        assert_eq!(sorted, vec![1.5625, 5.0625, 10.5625, 45.5625]);
        // The centre and the difference are the locals at EBP-0xc and EBP-0x50.
        assert_eq!(e.mem.f32(ebp - 0xc), 3.25);
        assert!(calls(&e, POINT_SUBTRACT).iter().all(|c| c[1] == ebp - 0x50));
    }

    #[test]
    fn ranged_count_is_the_array_at_0x18() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        fill(&mut e, object + 0x18, &[(1, 0), (2, 1)]);
        assert_eq!(fn_00990890(&mut e, Ptr::new(object)), 2);
        assert_eq!(calls(&e, ARRAY_SIZE), vec![vec![object + 0x18]]);
    }

    /// `data` with `members` (`iMemberCount`, which the
    /// `DATA_MEMBER_COUNT_GET` double returns), `ranged` ranged entries and
    /// both detection bounds 1.
    fn can_execute_setup(
        e: &mut Engine,
        members: u32,
        ranged: u32,
        allowed: bool,
    ) -> (Ptr<CombatGroup>, Ptr<CombatGroupData>) {
        e.register(DATA_MEMBER_COUNT_GET, |e, a| {
            eax(e.mem.u32(a[0] + DATA_MEMBER_COUNT))
        });
        returns(e, GROUP_STRATEGY_ALLOWED, allowed as u32);
        returns(e, CHECK_MEMBER_TARGET_INTERSECTION, 0);
        let group = group_with(e, &[]);
        let data = data_for(e, group);
        e.mem.set_u32(data.addr() + DATA_MEMBER_COUNT, members);
        let entries: Vec<(u32, u32)> = (0..ranged).map(|i| (0, i)).collect();
        fill(e, data.addr() + DATA_RANGED_MEMBERS, &entries);
        e.mem.set_u32(data.addr() + DATA_MIN_DETECTION, 1);
        e.mem.set_u32(data.addr() + DATA_MAX_DETECTION, 1);
        (group, data)
    }

    type CanExecute =
        fn(&mut Engine, Ptr<CombatGroupStrategy>, Ptr<CombatGroup>, Ptr<CombatGroupData>) -> bool;

    #[test]
    fn flank_can_execute() {
        let f: CanExecute = combat_group_strategy_flank_can_execute;
        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 3, 1, true);
        assert!(f(&mut e, Ptr::NULL, g, d));
        assert_eq!(
            calls(&e, GROUP_STRATEGY_ALLOWED),
            vec![vec![g.addr(), 2, 30.0f32.to_bits()]]
        );
        // Members' targets intersecting.
        e.mem.set_u32(d.addr() + DATA_TARGETS_INTERSECTING, 1);
        assert!(!f(&mut e, Ptr::NULL, g, d));
        e.mem.set_u32(d.addr() + DATA_TARGETS_INTERSECTING, 0);
        // Maximum detection not above 0.
        e.mem.set_u32(d.addr() + DATA_MAX_DETECTION, 0);
        assert!(!f(&mut e, Ptr::NULL, g, d));

        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 2, 1, true);
        assert!(!f(&mut e, Ptr::NULL, g, d));
        assert!(calls(&e, GROUP_STRATEGY_ALLOWED).is_empty());
        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 3, 0, true);
        assert!(!f(&mut e, Ptr::NULL, g, d));
        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 3, 1, false);
        assert!(!f(&mut e, Ptr::NULL, g, d));
    }

    #[test]
    fn flank_execute() {
        let mut e = engine();
        ignore(&mut e, GROUP_SET_ASSIGNMENT);
        let data = ranged_data(&mut e);
        let group = e.get(data, CombatGroupData::pCombatGroup);
        fill(&mut e, data.addr() + DATA_MELEE_MEMBERS, &[(0, 5), (0, 6)]);
        // Five members, four ranged: share 0.8, trunc(0.3 * 4) = 1 flanker,
        // the farthest from the centre (x = 10, member 23).
        e.register(DATA_MEMBER_COUNT_GET, |_, _| eax(5));
        combat_group_strategy_flank_execute(&mut e, Ptr::NULL, group, data);
        let g = group.addr();
        assert_eq!(
            calls(&e, GROUP_SET_ASSIGNMENT),
            vec![vec![g, 5, 1], vec![g, 6, 1], vec![g, 23, 1]]
        );

        // Share 0.5 (eight members): melee only (none here), no sort.
        let mut e = engine();
        ignore(&mut e, GROUP_SET_ASSIGNMENT);
        let data = ranged_data(&mut e);
        e.register(DATA_MEMBER_COUNT_GET, |_, _| eax(8));
        combat_group_strategy_flank_execute(&mut e, Ptr::NULL, group, data);
        assert!(calls(&e, GROUP_SET_ASSIGNMENT).is_empty());
        assert!(calls(&e, POINT_ADD).is_empty());

        // No members: 4 / 0 is infinite and FISTP stores the integer
        // indefinite, whose low word is 0: no flanker.
        let mut e = engine();
        ignore(&mut e, GROUP_SET_ASSIGNMENT);
        let data = ranged_data(&mut e);
        e.register(DATA_MEMBER_COUNT_GET, |_, _| eax(0));
        combat_group_strategy_flank_execute(&mut e, Ptr::NULL, group, data);
        assert!(calls(&e, GROUP_SET_ASSIGNMENT).is_empty());
        assert!(calls(&e, POINT_ADD).is_empty());
    }

    #[test]
    fn melee_count() {
        let mut e = engine();
        let group = group_with(&mut e, &[]);
        let data = data_for(&mut e, group);
        fill(&mut e, data.addr() + DATA_MELEE_MEMBERS, &[(0, 0); 3]);
        assert_eq!(fn_00990a60(&mut e, data), 3);
        assert_eq!(calls(&e, ARRAY_SIZE), vec![vec![data.addr() + 0x28]]);
    }

    #[test]
    fn melee_element() {
        let mut e = engine();
        let group = group_with(&mut e, &[]);
        let data = data_for(&mut e, group);
        fill(&mut e, data.addr() + DATA_MELEE_MEMBERS, &[(7, 1), (8, 2)]);
        let entry = fn_00990a80(&mut e, data, 1);
        assert_eq!(e.mem.u32(entry.addr()), 8);
        assert_eq!(calls(&e, ARRAY_ELEMENT), vec![vec![data.addr() + 0x28, 1]]);
    }

    #[test]
    fn ranged_element() {
        let mut e = engine();
        let group = group_with(&mut e, &[]);
        let data = data_for(&mut e, group);
        fill(&mut e, data.addr() + DATA_RANGED_MEMBERS, &[(7, 1), (8, 2)]);
        let entry = fn_00990aa0(&mut e, data, 0);
        assert_eq!(e.mem.u32(entry.addr()), 7);
        assert_eq!(calls(&e, ARRAY_ELEMENT), vec![vec![data.addr() + 0x18, 0]]);
    }

    #[test]
    fn flank_update() {
        let mut e = engine();
        ignore(&mut e, GROUP_SET_ASSIGNMENT);
        let actors: Vec<u32> = (0..3).map(|_| actor(&mut e, 0, [0.0; 3])).collect();
        let group = group_with(&mut e, &actors);
        e.register(
            GROUP_MEMBER_ASSIGNMENT,
            |_, a| eax([1, 0, 1][a[1] as usize]),
        );
        e.register(ACTOR_POSITION_INTO, |_, a| eax(a[1]));
        let mut results = vec![1u32, 0].into_iter();
        e.register_double(GROUP_POSITION_CHECK, move |_, _| {
            eax(results.next().unwrap())
        });
        assert!(combat_group_strategy_flank_update(&mut e, Ptr::NULL, group));
        let g = group.addr();
        assert_eq!(calls(&e, GROUP_SET_ASSIGNMENT), vec![vec![g, 0, 0]]);
        let checks = calls(&e, GROUP_POSITION_CHECK);
        assert_eq!(checks.len(), 2);
        assert_eq!(checks[0][0], g);
        assert_eq!(checks[0][2..], [384.0f32.to_bits(), 0]);
        let into = calls(&e, ACTOR_POSITION_INTO);
        assert_eq!((into[0][0], into[1][0]), (actors[0], actors[2]));
        assert_eq!(checks[0][1], into[0][1]);

        returns(&mut e, GROUP_POSITION_CHECK, 1);
        assert!(!combat_group_strategy_flank_update(
            &mut e,
            Ptr::NULL,
            group
        ));
    }

    #[test]
    fn fall_back_can_execute() {
        let f: CanExecute = combat_group_strategy_fall_back_can_execute;
        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 2, 0, true);
        for (health, expected) in [(0.2, true), (0.5, false), (f64::NAN, true)] {
            e.register_double(DATA_HEALTH_PERCENTAGE, move |_, _| Ret {
                st0: health,
                ..Ret::default()
            });
            assert_eq!(f(&mut e, Ptr::NULL, g, d), expected);
        }
        assert_eq!(
            calls(&e, GROUP_STRATEGY_ALLOWED)[0],
            vec![g.addr(), 0, 30.0f32.to_bits()]
        );
        assert_eq!(calls(&e, DATA_HEALTH_PERCENTAGE)[0], vec![d.addr()]);

        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 1, 0, true);
        assert!(!f(&mut e, Ptr::NULL, g, d));
        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 2, 0, false);
        assert!(!f(&mut e, Ptr::NULL, g, d));
        assert!(calls(&e, DATA_HEALTH_PERCENTAGE).is_empty());
    }

    #[test]
    fn fall_back_execute() {
        let mut e = engine();
        ignore(&mut e, GROUP_SET_ASSIGNMENT);
        let group = group_with(&mut e, &[1, 2, 3]);
        combat_group_strategy_fall_back_execute(&mut e, Ptr::NULL, group, Ptr::NULL);
        let g = group.addr();
        assert_eq!(
            calls(&e, GROUP_SET_ASSIGNMENT),
            vec![vec![g, 0, 2], vec![g, 1, 2], vec![g, 2, 2]]
        );
        assert_eq!(calls(&e, ARRAY_SIZE).len(), 4);
    }

    type Update = fn(&mut Engine, Ptr<CombatGroupStrategy>, Ptr<CombatGroup>) -> bool;

    fn check_update(update: Update, assignment: u32) {
        let mut e = engine();
        let group = group_with(&mut e, &[]);
        returns(&mut e, GROUP_FIND_ASSIGNMENT, 0);
        assert!(!update(&mut e, Ptr::NULL, group));
        returns(&mut e, GROUP_FIND_ASSIGNMENT, 0x100);
        assert!(update(&mut e, Ptr::NULL, group));
        assert_eq!(
            calls(&e, GROUP_FIND_ASSIGNMENT),
            vec![vec![group.addr(), assignment]; 2]
        );
    }

    #[test]
    fn fall_back_update() {
        check_update(combat_group_strategy_fall_back_update, 2);
    }

    #[test]
    fn ambush_can_execute() {
        let f: CanExecute = combat_group_strategy_ambush_can_execute;
        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 2, 0, true);
        assert!(f(&mut e, Ptr::NULL, g, d));
        assert_eq!(
            calls(&e, GROUP_STRATEGY_ALLOWED),
            vec![vec![g.addr(), 3, 60.0f32.to_bits()]]
        );
        e.mem.set_u32(d.addr() + DATA_TARGETS_INTERSECTING, 1);
        assert!(!f(&mut e, Ptr::NULL, g, d));
        e.mem.set_u32(d.addr() + DATA_TARGETS_INTERSECTING, 0);
        e.mem.set_u32(d.addr() + DATA_MAX_DETECTION, (-3i32) as u32);
        assert!(!f(&mut e, Ptr::NULL, g, d));

        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 1, 0, true);
        assert!(!f(&mut e, Ptr::NULL, g, d));
        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 2, 0, false);
        assert!(!f(&mut e, Ptr::NULL, g, d));
    }

    #[test]
    fn ambush_execute() {
        let mut e = engine();
        ignore(&mut e, GROUP_SET_ASSIGNMENT);
        let group = group_with(&mut e, &[1, 2, 3, 4, 5]);
        combat_group_strategy_ambush_execute(&mut e, Ptr::NULL, group, Ptr::NULL);
        let g = group.addr();
        assert_eq!(
            calls(&e, GROUP_SET_ASSIGNMENT),
            vec![
                vec![g, 0, 2],
                vec![g, 1, 2],
                vec![g, 2, 3],
                vec![g, 3, 3],
                vec![g, 4, 3]
            ]
        );
    }

    #[test]
    fn ambush_update() {
        check_update(combat_group_strategy_ambush_update, 3);
    }

    #[test]
    fn advance_can_execute() {
        let f: CanExecute = combat_group_strategy_advance_can_execute;
        let mut e = engine();
        point_doubles(&mut e);
        e.register(COMBAT_TARGET_OF, |_, a| eax(a[0]));
        let far_target = actor(&mut e, 0, [2000.0, 0.0, 0.0]);
        let near_target = actor(&mut e, 0, [1000.0, 0.0, 0.0]);
        let no_target = actor(&mut e, 0, [0.0; 3]);
        let attacker = actor(&mut e, far_target, [0.0; 3]);
        let (g, d) = can_execute_setup(&mut e, 0, 0, true);
        fill(
            &mut e,
            d.addr() + DATA_RANGED_MEMBERS,
            &[(no_target, 0), (attacker, 1)],
        );
        assert!(f(&mut e, Ptr::NULL, g, d));
        assert_eq!(
            calls(&e, GROUP_STRATEGY_ALLOWED),
            vec![vec![g.addr(), 1, 60.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, COMBAT_TARGET_OF), vec![vec![0], vec![far_target]]);
        let sub = calls(&e, POINT_SUBTRACT);
        assert_eq!((sub[0][0], sub[0][2]), (attacker + 8, far_target + 8));

        // 1000 units from its target: 1e6 is below 1024 squared.
        e.mem.set_u32(attacker + 4, near_target);
        assert!(!f(&mut e, Ptr::NULL, g, d));
        // Minimum detection not above 0.
        e.mem.set_u32(attacker + 4, far_target);
        e.mem.set_u32(d.addr() + DATA_MIN_DETECTION, 0);
        assert!(!f(&mut e, Ptr::NULL, g, d));

        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 5, 1, true);
        assert!(!f(&mut e, Ptr::NULL, g, d));
        let mut e = engine();
        let (g, d) = can_execute_setup(&mut e, 5, 2, false);
        assert!(!f(&mut e, Ptr::NULL, g, d));
    }

    #[test]
    fn advance_execute() {
        let mut e = engine();
        ignore(&mut e, GROUP_SET_ASSIGNMENT);
        let group = group_with(&mut e, &[]);
        let data = data_for(&mut e, group);
        fill(&mut e, data.addr() + DATA_RANGED_MEMBERS, &[(0, 4), (0, 9)]);
        combat_group_strategy_advance_execute(&mut e, Ptr::NULL, group, data);
        let g = group.addr();
        assert_eq!(
            calls(&e, GROUP_SET_ASSIGNMENT),
            vec![vec![g, 4, 4], vec![g, 9, 4]]
        );
        assert_eq!(calls(&e, ARRAY_SIZE).len(), 3);
    }

    #[test]
    fn advance_update() {
        check_update(combat_group_strategy_advance_update, 4);
    }

    #[test]
    fn member_array_constructor() {
        let mut e = engine();
        ignore(&mut e, ARRAY_INIT);
        let array = Ptr::<()>::new(e.mem.alloc(0x10));
        assert_eq!(fn_00990ef0(&mut e, array), array);
        assert_eq!(e.mem.u32(array.addr()), VTABLE_MEMBER_ARRAY);
        assert_eq!(calls(&e, ARRAY_INIT), vec![vec![array.addr(), 0, 0]]);
    }

    #[test]
    fn x87_integer_store_of_out_of_range_values() {
        assert_eq!(fistp_truncate_low(1.9), 1);
        assert_eq!(fistp_truncate_low(f64::INFINITY), 0);
        assert_eq!(fistp_truncate_low(f64::NAN), 0);
        assert_eq!(fistp_truncate_low(4_294_967_297.5), 1);
    }
}
