//! `Havok SDK/hkpcollisiondispatcher.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpCollisionDispatcher` keeps the tables that map a pair of shape types
//! to the agent that handles it: `m_agent2Types` / `m_agent3Types` (32x32
//! bytes each) with their predictive twins, the agent function tables, the
//! debug tables (three bytes per pair: the registered types and a
//! specialisation depth, 100 when unset) and the collision-quality table.
//! Float code that goes through x87 rounds at each `float` store; a float
//! copied through `FLD`/`FSTP` goes through [`x87_quiet`]. The `hkArray`
//! `m_shapeInheritance` is the three words at +0x1E24 (data, size,
//! capacity and flags).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpCollisionDispatcher` (Xbox PDB), 0x1E50 bytes on the Xbox (the
    /// offsets below are the ones the unit's code uses on PC).
    pub struct HkpCollisionDispatcher: 0x1e50 {
        /// The vtable (`010cc7c0`).
        0x00 vtable: u32,
        /// `m_memSizeAndFlags` (Xbox PDB).
        0x04 m_memSizeAndFlags: u16,
        /// `m_referenceCount` (Xbox PDB).
        0x06 m_referenceCount: i16,
        /// `m_defaultCollisionAgent` (Xbox PDB).
        0x08 m_defaultCollisionAgent: u32,
        // +0x0C `m_contactMgrFactory` (Xbox PDB): 64 `hkpContactMgrFactory*`.
        // +0x110 `m_hasAlternateType` (Xbox PDB): 32 words.
        /// `m_numAgent2Types` (Xbox PDB).
        0x190 m_numAgent2Types: i32,
        // +0x1A0 `m_agent2Types`, +0x5A0 `m_agent2TypesPred` (Xbox PDB): 32x32 bytes.
        // +0x9A0 `m_agent2Func` (Xbox PDB): 64 `AgentFuncs` of 0x14 bytes.
        /// `m_numAgent3Types` (Xbox PDB).
        0xEA0 m_numAgent3Types: i32,
        // +0xEB0 `m_agent3Types`, +0x12B0 `m_agent3TypesPred` (Xbox PDB): 32x32 bytes.
        // +0x16B0 `m_agent3Func` (Xbox PDB): 16 `Agent3FuncsIntern` of 0x50 bytes.
        // +0x1BB0 `m_collisionQualityTable` (Xbox PDB): 10x10 signed bytes.
        // +0x1C20 `m_collisionQualityInfo` (Xbox PDB): 8 `hkpCollisionQualityInfo` of 0x40 bytes.
        /// `m_collisionAgentRegistered` (Xbox PDB).
        0x1E20 m_collisionAgentRegistered: bool,
        /// `m_agent3Registered` (Xbox PDB).
        0x1E21 m_agent3Registered: bool,
        /// `m_midphaseAgent3Registered` (Xbox PDB).
        0x1E22 m_midphaseAgent3Registered: bool,
        /// `m_checkEnabled` (Xbox PDB).
        0x1E23 m_checkEnabled: bool,
        /// `m_shapeInheritance.m_data` (Xbox PDB): `ShapeInheritance*`, entries of
        /// (`m_primaryType`, `m_alternateType`).
        0x1E24 m_shapeInheritanceData: u32,
        /// `m_shapeInheritance.m_size` (Xbox PDB).
        0x1E28 m_shapeInheritanceSize: i32,
        /// `m_shapeInheritance.m_capacityAndFlags` (Xbox PDB): the capacity in
        /// the low 30 bits, bit 31 set when the buffer is not owned.
        0x1E2C m_shapeInheritanceCapacityAndFlags: u32,
        /// `m_debugAgent2Table` (Xbox PDB).
        0x1E30 m_debugAgent2Table: u32,
        /// `m_debugAgent2TablePred` (Xbox PDB).
        0x1E34 m_debugAgent2TablePred: u32,
        /// `m_debugAgent3Table` (Xbox PDB).
        0x1E38 m_debugAgent3Table: u32,
        /// `m_debugAgent3TablePred` (Xbox PDB).
        0x1E3C m_debugAgent3TablePred: u32,
        /// `m_expectedMaxLinearVelocity` (Xbox PDB).
        0x1E40 m_expectedMaxLinearVelocity: f32,
        /// `m_expectedMinPsiDeltaTime` (Xbox PDB).
        0x1E44 m_expectedMinPsiDeltaTime: f32,
    }
}

/// `hkpCollisionDispatcher`'s vtable and the base class `hkReferencedObject`
/// one the destructor leaves behind.
const DISPATCHER_VTABLE: u32 = 0x010c_c7c0;
const REFERENCED_OBJECT_VTABLE: u32 = 0x0102_e388;

// Offsets of the tables (see the layout).
const HAS_ALTERNATE_TYPE: u32 = 0x110;
const AGENT2_TYPES: u32 = 0x1a0;
const AGENT2_TYPES_PRED: u32 = 0x5a0;
const AGENT2_FUNC: u32 = 0x9a0;
const AGENT3_TYPES: u32 = 0xeb0;
const AGENT3_TYPES_PRED: u32 = 0x12b0;
const AGENT3_FUNC: u32 = 0x16b0;
const COLLISION_QUALITY_TABLE: u32 = 0x1bb0;
const COLLISION_QUALITY_INFO: u32 = 0x1c20;
/// Size of an `AgentFuncs` (0x14), an `Agent3FuncsIntern` (0x50) and a
/// `hkpCollisionQualityInfo` (0x40).
const AGENT2_FUNCS_SIZE: u32 = 0x14;
const AGENT3_FUNCS_SIZE: u32 = 0x50;
const QUALITY_INFO_SIZE: u32 = 0x40;
/// Entries in the tables: 32 shape types.
const TYPE_COUNT: u32 = 32;
/// The wildcard shape type of a registration.
const ANY_TYPE: i32 = -1;
/// The priority a debug entry holds until something is registered.
const UNSET_PRIORITY: u8 = 100;

/// The addresses of the default agent's four functions and the default
/// agent-3 stubs the initialisation stores (code in `.text`).
const DEFAULT_AGENT2_FUNCS: [u32; 4] = [0x00d1_0790, 0x00d1_0770, 0x00d1_0760, 0x00d1_0780];
const DEFAULT_AGENT3_FIRST: u32 = 0x00cf_b050;
const DEFAULT_AGENT3_SECOND: u32 = 0x00cf_b060;
const DEFAULT_AGENT3_THIRD: u32 = 0x00cf_b070;

/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the heap allocator
/// (slot 8 frees `(block, size)`, slot 0x10 frees a buffer), its `+0x14`
/// the allocator the debug tables come from.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Grows an `hkArray` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// Increments the reference count at +6 of the object in `this`
/// (`00c90510`) and decrements it, deleting the object at 1 (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The default agent-3 registration (`00d923b0`, cdecl `(dispatcher)`).
const REGISTER_DEFAULT_AGENT3: u32 = 0x00d9_23b0;
/// `hkGetShapeTypeName` (Xbox PDB, cdecl `(type)`) and `hkString::snprintf`
/// (Xbox PDB, cdecl `(buffer, size, format, ...)`).
const GET_SHAPE_TYPE_NAME: u32 = 0x00d4_64f0;
const STRING_SNPRINTF: u32 = 0x00cb_0f60;
/// `hkStatisticsCollector::getAllocatedSize` (Xbox PDB, cdecl `(block,
/// size)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `memset` (cdecl `(destination, value, size)`).
const MEMSET: u32 = 0x00ec_61c0;
/// The warning's format string: `"Agent handling types <%s-%s> would
/// override more specialized agent <%s-%s>..."`.
const OVERRIDE_WARNING_FORMAT: u32 = 0x010c_c698;
/// The strings `calcContentStatistics` reports: `"shapeInheritanceArray"` and
/// `"contactMgrFactory"`.
const SHAPE_INHERITANCE_NAME: u32 = 0x010c_c7a4;
const CONTACT_MGR_FACTORY_NAME: u32 = 0x010c_c790;

/// `double` constants of the collision-quality set-up (`01011588` 0.5,
/// `0106d4c8` -2.0, `0101a6b0` -1.0, `01011598` -0.5, `010cc778` -0.3,
/// `01020998` 5.0, `01023b88` 0.005, `010cc770` -0.35, `010cc768` -0.6,
/// `010cc760` -0.42, `010cc780` -1e20) and `float` ones (`0104da48`
/// the largest finite value below `FLT_MAX`, `010cc78c`, `010cc788`,
/// `0102295c` -0.5, `0104fd68` -0.25, `0104fd30` -0.4, `0104f52c` -0.2).
const HALF_DOUBLE: u32 = 0x0101_1588;
const MINUS_TWO_DOUBLE: u32 = 0x0106_d4c8;
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
const MINUS_HALF_DOUBLE: u32 = 0x0101_1598;
const MINUS_THREE_TENTHS_DOUBLE: u32 = 0x010c_c778;
const FIVE_DOUBLE: u32 = 0x0102_0998;
const FIVE_THOUSANDTHS_DOUBLE: u32 = 0x0102_3b88;
const MINUS_THIRTY_FIVE_HUNDREDTHS_DOUBLE: u32 = 0x010c_c770;
const MINUS_SIX_TENTHS_DOUBLE: u32 = 0x010c_c768;
const MINUS_FORTY_TWO_HUNDREDTHS_DOUBLE: u32 = 0x010c_c760;
const MINUS_HUGE_DOUBLE: u32 = 0x010c_c780;
const LARGE_FLOAT: u32 = 0x0104_da48;
const KEEP_CONTACT_FLOAT_A: u32 = 0x010c_c78c;
const KEEP_CONTACT_FLOAT_B: u32 = 0x010c_c788;
const MINUS_HALF_FLOAT: u32 = 0x0102_295c;
const MINUS_QUARTER_FLOAT: u32 = 0x0104_fd68;
const MINUS_TWO_FIFTHS_FLOAT: u32 = 0x0104_fd30;
const MINUS_ONE_FIFTH_FLOAT: u32 = 0x0104_f52c;

fn dispatcher_address(this: Ptr<HkpCollisionDispatcher>, offset: u32) -> u32 {
    this.addr().wrapping_add(offset)
}

/// The address of entry `index` of `m_collisionQualityInfo`.
fn quality_slot(this: Ptr<HkpCollisionDispatcher>, index: u32) -> u32 {
    dispatcher_address(this, COLLISION_QUALITY_INFO + QUALITY_INFO_SIZE * index)
}

/// A copy of a `hkpCollisionQualityInfo` (`REP MOVSD` of 16 words).
fn copy_quality_info(e: &mut Engine, from: u32, to: u32) {
    for word in 0..QUALITY_INFO_SIZE / 4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// Reads entry `index` of `m_shapeInheritance`: (primary, alternate type).
fn shape_inheritance_entry(
    e: &Engine,
    this: Ptr<HkpCollisionDispatcher>,
    index: i32,
) -> (u32, u32) {
    let data = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceData);
    let entry = data.wrapping_add((index as u32).wrapping_mul(8));
    (e.mem.u32(entry), e.mem.u32(entry + 4))
}

// Translated from 00cfb080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the agent tables of the dispatcher (no Xbox PDB name; reached
/// from the constructor): one agent-2 and one agent-3 type registered, the
/// type tables cleared and every debug priority set to 100, the default
/// agent-2 functions and the default agent-3 stubs stored, then the default
/// agent-3 registration (`00d923b0`) run and `m_collisionAgentRegistered`
/// cleared. The debug tables are touched only where they exist.
pub fn fn_00cfb080(e: &mut Engine, this: Ptr<HkpCollisionDispatcher>) {
    e.set(this, HkpCollisionDispatcher::m_numAgent2Types, 1);
    e.set(this, HkpCollisionDispatcher::m_numAgent3Types, 1);
    for index in 0..TYPE_COUNT * TYPE_COUNT {
        if e.get(this, HkpCollisionDispatcher::m_debugAgent2Table) != 0 {
            for table in [
                HkpCollisionDispatcher::m_debugAgent2Table,
                HkpCollisionDispatcher::m_debugAgent2TablePred,
                HkpCollisionDispatcher::m_debugAgent3Table,
                HkpCollisionDispatcher::m_debugAgent3TablePred,
            ] {
                let base = e.get(this, table);
                e.mem
                    .set_u8(base.wrapping_add(3 * index + 2), UNSET_PRIORITY);
            }
        }
        for table in [
            AGENT2_TYPES,
            AGENT3_TYPES,
            AGENT2_TYPES_PRED,
            AGENT3_TYPES_PRED,
        ] {
            e.mem.set_u8(dispatcher_address(this, table + index), 0);
        }
    }
    for (slot, function) in DEFAULT_AGENT2_FUNCS.into_iter().enumerate() {
        e.mem.set_u32(
            dispatcher_address(this, AGENT2_FUNC + 4 * slot as u32),
            function,
        );
    }
    // `m_isFlipped` and `m_isPredictive` of the default agent.
    e.mem
        .set_u8(dispatcher_address(this, AGENT2_FUNC + 0x10), 0);
    e.mem
        .set_u8(dispatcher_address(this, AGENT2_FUNC + 0x11), 1);
    e.mem
        .set_u32(dispatcher_address(this, AGENT3_FUNC), DEFAULT_AGENT3_FIRST);
    e.mem.set_u32(
        dispatcher_address(this, AGENT3_FUNC + 4),
        DEFAULT_AGENT3_SECOND,
    );
    for offset in [0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x28, 0x20, 0x24] {
        e.mem
            .set_u32(dispatcher_address(this, AGENT3_FUNC + offset), 0);
    }
    e.mem.set_u32(
        dispatcher_address(this, AGENT3_FUNC + 0x2c),
        DEFAULT_AGENT3_THIRD,
    );
    e.mem
        .set_u32(dispatcher_address(this, AGENT3_FUNC + 0x40), 0);
    e.call(REGISTER_DEFAULT_AGENT3, &args![this]);
    e.set(
        this,
        HkpCollisionDispatcher::m_collisionAgentRegistered,
        false,
    );
    if e.get(this, HkpCollisionDispatcher::m_debugAgent3Table) != 0 {
        for index in 0..TYPE_COUNT * TYPE_COUNT {
            for table in [
                HkpCollisionDispatcher::m_debugAgent3Table,
                HkpCollisionDispatcher::m_debugAgent3TablePred,
            ] {
                let base = e.get(this, table);
                e.mem
                    .set_u8(base.wrapping_add(3 * index + 2), UNSET_PRIORITY);
            }
        }
    }
}

// Translated from 00cfb2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionDispatcher::internalRegisterCollisionAgent` (Xbox PDB):
/// stores `value` (the agent index) in `table[type_a][type_b]` (a 32x32
/// byte table) for the pair and for every pair reached through
/// `m_shapeInheritance` (an entry whose alternate type is `type_a`
/// registers its primary type in `type_a`'s place, likewise for `type_b`),
/// with `depth` + 1 on each such step. A type of -1 means every type from 1
/// to 31 and counts one more towards the specialisation depth. When a
/// `debug_table` is given, each pair's three bytes become
/// (`original_a`, `original_b`, depth); when `m_checkEnabled` is set and
/// the depth is above the priority already there, the "would override more
/// specialized agent" message is formatted (into a local buffer that is
/// not used). Sets `m_collisionAgentRegistered`.
#[allow(clippy::too_many_arguments)]
pub fn hkp_collision_dispatcher_internal_register_collision_agent(
    e: &mut Engine,
    this: Ptr<HkpCollisionDispatcher>,
    table: u32,
    value: u32,
    type_a: i32,
    type_b: i32,
    original_a: u32,
    original_b: u32,
    debug_table: u32,
    depth: i32,
) {
    e.set(
        this,
        HkpCollisionDispatcher::m_collisionAgentRegistered,
        true,
    );
    let mut index = 0;
    while index < e.get(this, HkpCollisionDispatcher::m_shapeInheritanceSize) {
        let (primary, alternate) = shape_inheritance_entry(e, this, index);
        if alternate == type_a as u32 {
            hkp_collision_dispatcher_internal_register_collision_agent(
                e,
                this,
                table,
                value,
                primary as i32,
                type_b,
                original_a,
                original_b,
                debug_table,
                depth + 1,
            );
        }
        if alternate == type_b as u32 {
            hkp_collision_dispatcher_internal_register_collision_agent(
                e,
                this,
                table,
                value,
                type_a,
                primary as i32,
                original_a,
                original_b,
                debug_table,
                depth + 1,
            );
        }
        index += 1;
    }

    let (mut first_a, mut end_a) = (type_a, type_a.wrapping_add(1));
    let (mut first_b, mut end_b) = (type_b, type_b.wrapping_add(1));
    let mut total_depth = depth;
    if type_a == ANY_TYPE {
        total_depth += 1;
        first_a = 1;
        end_a = TYPE_COUNT as i32;
    }
    if type_b == ANY_TYPE {
        total_depth += 1;
        first_b = 1;
        end_b = TYPE_COUNT as i32;
    }
    for a in first_a..end_a {
        for b in first_b..end_b {
            let pair = (a as u32).wrapping_mul(TYPE_COUNT).wrapping_add(b as u32);
            e.mem.set_u8(table.wrapping_add(pair), value as u8);
            if debug_table == 0 {
                continue;
            }
            let entry = debug_table.wrapping_add(pair.wrapping_mul(3));
            let existing = e.mem.i8(entry + 2) as i32;
            if e.get(this, HkpCollisionDispatcher::m_checkEnabled) && total_depth > existing {
                format_override_warning(e, entry, type_a, type_b);
            }
            e.mem.set_u8(entry, original_a as u8);
            e.mem.set_u8(entry + 1, original_b as u8);
            e.mem.set_u8(entry + 2, total_depth as u8);
        }
    }
}

/// Formats the "would override more specialized agent" message for the pair
/// `type_a`, `type_b` and the debug `entry` (the types already registered)
/// with `hkString::snprintf` into a 1024-byte local; the text is not used.
fn format_override_warning(e: &mut Engine, entry: u32, type_a: i32, type_b: i32) {
    let registered_a = e.mem.i8(entry) as i32;
    let registered_b = e.mem.i8(entry + 1) as i32;
    let name_registered_a = e.call(GET_SHAPE_TYPE_NAME, &args![registered_a]).u32();
    let name_registered_b = e.call(GET_SHAPE_TYPE_NAME, &args![registered_b]).u32();
    let name_a = e.call(GET_SHAPE_TYPE_NAME, &args![type_a]).u32();
    let name_b = e.call(GET_SHAPE_TYPE_NAME, &args![type_b]).u32();
    e.with_stack(0x400, |e, buffer| {
        e.call(
            STRING_SNPRINTF,
            &args![
                buffer,
                1000u32,
                OVERRIDE_WARNING_FORMAT,
                name_a,
                name_b,
                name_registered_a,
                name_registered_b
            ],
        );
    });
}

// Translated from 00cfb510 (decompiled, FalloutNV.exe 1.4.0.525)
/// ORs `m_hasAlternateType[source]` into `m_hasAlternateType[target]` and
/// repeats it for every `m_shapeInheritance` entry whose alternate type is
/// `target` (target = the entry's primary type, source = `target`), with
/// `depth` + 1 (no Xbox PDB name). The exe passes `depth` + 1 down but never
/// reads it otherwise.
#[allow(clippy::only_used_in_recursion)]
pub fn fn_00cfb510(
    e: &mut Engine,
    this: Ptr<HkpCollisionDispatcher>,
    target: u32,
    source: u32,
    depth: u32,
) {
    let source_flags = e
        .mem
        .u32(dispatcher_address(this, HAS_ALTERNATE_TYPE + 4 * source));
    let target_word = dispatcher_address(this, HAS_ALTERNATE_TYPE + 4 * target);
    let merged = e.mem.u32(target_word) | source_flags;
    e.mem.set_u32(target_word, merged);
    let mut index = 0;
    while index < e.get(this, HkpCollisionDispatcher::m_shapeInheritanceSize) {
        let (primary, alternate) = shape_inheritance_entry(e, this, index);
        if alternate == target {
            fn_00cfb510(e, this, primary, alternate, depth + 1);
        }
        index += 1;
    }
}

// Translated from 00cfb570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the collision-quality information from an
/// `InitCollisionQualityInfo` (Xbox PDB layout: `m_gravityLength` +0x00,
/// `m_collisionTolerance` +0x04, `m_minDeltaTime` +0x08,
/// `m_maxLinearVelocity` +0x0C, the four `m_numToisTillAllowedPenetration*`
/// floats +0x10..+0x1C, four constraint priorities +0x20..+0x26,
/// `m_wantContinuousCollisionDetection` +0x28, `m_enableNegativeManifoldTims`
/// +0x29, `m_enableNegativeToleranceToCreateNon4dContacts` +0x2A,
/// `m_enableToiWeldRejection` +0x2B); no Xbox PDB name for the function.
///
/// Stores `m_expectedMinPsiDeltaTime` and `m_expectedMaxLinearVelocity`,
/// builds `m_collisionQualityInfo[1]` (tolerances, the two negative-tolerance
/// variants from `0.5 * gravity * dt * dt`, large separations and 1.0 time
/// steps), copies it to entries 7, 2, 3, 4 and 5, and, when continuous
/// collision detection is wanted, fills entries 2, 3, 4 and 5 with the
/// time-of-impact thresholds `-0.5 / (n - 1)`, `-0.35 / (n - 1)` and
/// friends (entry 5 with its own constants); entry 6 is a copy of entry 5.
/// Finally `m_collisionQualityTable` is set to 1 and a fixed set of its pairs
/// to the quality levels the code lists. The computations run in `f64` and
/// round to `float` at each store, like the x87 code.
pub fn fn_00cfb570(e: &mut Engine, this: Ptr<HkpCollisionDispatcher>, info: Ptr) {
    let i = info.addr();
    let input = |e: &Engine, offset: u32| e.mem.f32(i + offset);
    let min_delta_time = x87_quiet(input(e, 8));
    e.set(
        this,
        HkpCollisionDispatcher::m_expectedMinPsiDeltaTime,
        min_delta_time,
    );
    let max_linear_velocity = x87_quiet(input(e, 0xc));
    e.set(
        this,
        HkpCollisionDispatcher::m_expectedMaxLinearVelocity,
        max_linear_velocity,
    );

    let tolerance = x87_quiet(input(e, 4));
    let half: f64 = e.global(HALF_DOUBLE);
    // `((gravity * 0.5) * dt) * dt`, stored as a `float`.
    let fall = (((input(e, 0) as f64 * half) * input(e, 8) as f64) * input(e, 8) as f64) as f32;
    let first = quality_slot(this, 1);
    e.mem.set_u8(first + 0x3e, e.mem.u8(i + 0x2b));
    for offset in [0u32, 4, 0xc] {
        e.mem.set_f32(first + offset, tolerance);
    }
    if e.mem.u8(i + 0x29) != 0 {
        let minus_two: f64 = e.global(MINUS_TWO_DOUBLE);
        e.mem.set_f32(first + 0xc, (minus_two * fall as f64) as f32);
    }
    e.mem.set_f32(first + 8, tolerance);
    if e.mem.u8(i + 0x2a) != 0 {
        let minus_one: f64 = e.global(MINUS_ONE_DOUBLE);
        e.mem.set_f32(first + 8, (minus_one * fall as f64) as f32);
    }
    let large = x87_quiet(e.global::<f32>(LARGE_FLOAT));
    e.mem.set_f32(first + 0x34, large);
    e.mem.set_u32(first + 0x10, 0);
    e.mem.set_u8(first + 0x14, 0);
    let priority = e.mem.u16(i + 0x20);
    e.mem.set_u16(first + 0x3c, priority);
    let floor = x87_quiet(e.global::<f32>(KEEP_CONTACT_FLOAT_A));
    e.mem.set_f32(first + 0x18, floor);
    e.mem.set_f32(first + 0x1c, floor);
    let ceiling = x87_quiet(e.global::<f32>(KEEP_CONTACT_FLOAT_B));
    e.mem.set_f32(first + 0x28, ceiling);
    e.mem.set_f32(first + 0x2c, ceiling);
    let huge: f64 = e.global(MINUS_HUGE_DOUBLE);
    e.mem.set_f32(first + 0x30, huge.abs() as f32);
    for offset in [0x20u32, 0x24, 0x38] {
        e.mem.set_f32(first + offset, 1.0);
    }

    for index in [7, 2, 3, 4, 5] {
        copy_quality_info(e, first, quality_slot(this, index));
    }
    let seventh = quality_slot(this, 7);
    for offset in [0xcu32, 4, 8] {
        e.mem.set_f32(seventh + offset, tolerance);
    }

    let wants_continuous = e.mem.u8(i + 0x28) != 0;
    if wants_continuous {
        fill_time_of_impact_entry(e, quality_slot(this, 2), i, 0x10, 0x22, true);
        fill_time_of_impact_entry(e, quality_slot(this, 3), i, 0x14, 0x22, false);
        fill_time_of_impact_entry_third(e, quality_slot(this, 4), i);
    }

    let fifth = quality_slot(this, 5);
    e.mem.set_f32(fifth, tolerance);
    if wants_continuous {
        fill_forced_time_of_impact_entry(e, fifth, i);
    }
    let sixth = quality_slot(this, 6);
    copy_quality_info(e, fifth, sixth);
    e.mem.set_f32(sixth + 0xc, tolerance);
    e.mem.set_f32(sixth + 8, tolerance);

    let table = dispatcher_address(this, COLLISION_QUALITY_TABLE);
    e.call(MEMSET, &args![table, 1u32, 100u32]);
    for (offset, quality) in COLLISION_QUALITY_PAIRS {
        e.mem.set_u8(table + offset, quality);
    }
}

/// The first two time-of-impact entries (2 and 3) of `fn_00cfb570`: `slot`
/// receives the thresholds computed from the input float at `float_at`
/// (`n`), the input word at `priority_at` and the maximum linear velocity
/// (`v`): `-0.5` and `g = -0.5 / (n - 1)`; `h = -0.35 / (n - 1)` and
/// `h * -0.3`; `-0.25`; `-g`; `5.0 / v` and `0.005 / v`; `g * -2.0 / v`.
/// The first entry also sets the byte at +0x14.
fn fill_time_of_impact_entry(
    e: &mut Engine,
    slot: u32,
    info: u32,
    float_at: u32,
    priority_at: u32,
    sets_flag_byte: bool,
) {
    let velocity = e.mem.f32(info + 0xc) as f64;
    let n = e.mem.f32(info + float_at) as f64;
    let one = 1.0f64;
    let (minus_half, minus_thirty_five, minus_three_tenths): (f64, f64, f64) = (
        e.global(MINUS_HALF_DOUBLE),
        e.global(MINUS_THIRTY_FIVE_HUNDREDTHS_DOUBLE),
        e.global(MINUS_THREE_TENTHS_DOUBLE),
    );
    let (five, thousandth, minus_two, minus_one): (f64, f64, f64, f64) = (
        e.global(FIVE_DOUBLE),
        e.global(FIVE_THOUSANDTHS_DOUBLE),
        e.global(MINUS_TWO_DOUBLE),
        e.global(MINUS_ONE_DOUBLE),
    );
    e.mem
        .set_f32(slot + 0x18, x87_quiet(e.global::<f32>(MINUS_HALF_FLOAT)));
    let g = (minus_half / (n - one)) as f32;
    e.mem.set_f32(slot + 0x1c, g);
    e.mem
        .set_f32(slot + 0x28, x87_quiet(e.global::<f32>(MINUS_QUARTER_FLOAT)));
    let h = (minus_thirty_five / (n - one)) as f32;
    e.mem.set_f32(slot + 0x2c, h);
    e.mem
        .set_f32(slot + 0x30, (h as f64 * minus_three_tenths) as f32);
    e.mem.set_f32(slot + 0x34, (g as f64 * minus_one) as f32);
    e.mem.set_f32(slot + 0x20, (five / velocity) as f32);
    e.mem.set_f32(slot + 0x24, (thousandth / velocity) as f32);
    e.mem
        .set_f32(slot + 0x38, ((g as f64 * minus_two) / velocity) as f32);
    let priority = e.mem.u16(info + priority_at);
    e.mem.set_u16(slot + 0x3c, priority);
    e.mem.set_u32(slot + 0x10, 1);
    if sets_flag_byte {
        e.mem.set_u8(slot + 0x14, 1);
    }
}

/// Entry 4 of `fn_00cfb570`: as [`fill_time_of_impact_entry`] from the input
/// float at +0x18 and the priority word at +0x24, but the last threshold
/// (+0x38) is `-g / v`.
fn fill_time_of_impact_entry_third(e: &mut Engine, slot: u32, info: u32) {
    let velocity = e.mem.f32(info + 0xc) as f64;
    let n = e.mem.f32(info + 0x18) as f64;
    let one = 1.0f64;
    let (minus_half, minus_thirty_five, minus_three_tenths): (f64, f64, f64) = (
        e.global(MINUS_HALF_DOUBLE),
        e.global(MINUS_THIRTY_FIVE_HUNDREDTHS_DOUBLE),
        e.global(MINUS_THREE_TENTHS_DOUBLE),
    );
    let (five, thousandth, minus_one): (f64, f64, f64) = (
        e.global(FIVE_DOUBLE),
        e.global(FIVE_THOUSANDTHS_DOUBLE),
        e.global(MINUS_ONE_DOUBLE),
    );
    e.mem
        .set_f32(slot + 0x18, x87_quiet(e.global::<f32>(MINUS_HALF_FLOAT)));
    let g = (minus_half / (n - one)) as f32;
    e.mem.set_f32(slot + 0x1c, g);
    e.mem
        .set_f32(slot + 0x28, x87_quiet(e.global::<f32>(MINUS_QUARTER_FLOAT)));
    let h = (minus_thirty_five / (n - one)) as f32;
    e.mem.set_f32(slot + 0x2c, h);
    e.mem
        .set_f32(slot + 0x30, (h as f64 * minus_three_tenths) as f32);
    let minus_g = (g as f64 * minus_one) as f32;
    e.mem.set_f32(slot + 0x34, minus_g);
    e.mem.set_f32(slot + 0x20, (five / velocity) as f32);
    e.mem.set_f32(slot + 0x24, (thousandth / velocity) as f32);
    e.mem
        .set_f32(slot + 0x38, (minus_g as f64 / velocity) as f32);
    let priority = e.mem.u16(info + 0x24);
    e.mem.set_u16(slot + 0x3c, priority);
    e.mem.set_u32(slot + 0x10, 1);
}

/// Entry 5 of `fn_00cfb570` (the forced time-of-impact entry): own
/// constants (`-0.4`, `-0.6 / (n - 1)`, `-0.2`, `-0.42 / (n - 1)` with `n`
/// the input float at +0x1C) and the priority word at +0x26.
fn fill_forced_time_of_impact_entry(e: &mut Engine, slot: u32, info: u32) {
    let velocity = e.mem.f32(info + 0xc) as f64;
    let n = e.mem.f32(info + 0x1c) as f64;
    let one = 1.0f64;
    let (minus_six_tenths, minus_forty_two, minus_three_tenths): (f64, f64, f64) = (
        e.global(MINUS_SIX_TENTHS_DOUBLE),
        e.global(MINUS_FORTY_TWO_HUNDREDTHS_DOUBLE),
        e.global(MINUS_THREE_TENTHS_DOUBLE),
    );
    let (five, thousandth, minus_one): (f64, f64, f64) = (
        e.global(FIVE_DOUBLE),
        e.global(FIVE_THOUSANDTHS_DOUBLE),
        e.global(MINUS_ONE_DOUBLE),
    );
    e.mem.set_f32(
        slot + 0x18,
        x87_quiet(e.global::<f32>(MINUS_TWO_FIFTHS_FLOAT)),
    );
    let g = (minus_six_tenths / (n - one)) as f32;
    e.mem.set_f32(slot + 0x1c, g);
    e.mem.set_f32(
        slot + 0x28,
        x87_quiet(e.global::<f32>(MINUS_ONE_FIFTH_FLOAT)),
    );
    let h = (minus_forty_two / (n - one)) as f32;
    e.mem.set_f32(slot + 0x2c, h);
    e.mem
        .set_f32(slot + 0x30, (minus_three_tenths * h as f64) as f32);
    let minus_g = (minus_one * g as f64) as f32;
    e.mem.set_f32(slot + 0x34, minus_g);
    e.mem.set_f32(slot + 0x20, (five / velocity) as f32);
    e.mem.set_f32(slot + 0x24, (thousandth / velocity) as f32);
    e.mem
        .set_f32(slot + 0x38, (minus_g as f64 / velocity) as f32);
    let priority = e.mem.u16(info + 0x26);
    e.mem.set_u16(slot + 0x3c, priority);
    e.mem.set_u32(slot + 0x10, 1);
}

/// (offset in `m_collisionQualityTable`, value) pairs `fn_00cfb570` sets
/// after filling the 100 bytes with 1; its code stores them in this order
/// (81 byte stores, 00cfb947..00cfbb2d).
const COLLISION_QUALITY_PAIRS: [(u32, u8); 81] = [
    (0, 0),
    (1, 0),
    (9, 3),
    (2, 1),
    (3, 2),
    (4, 4),
    (5, 5),
    (6, 4),
    (8, 6),
    (10, 0),
    (11, 0),
    (19, 3),
    (12, 1),
    (13, 1),
    (14, 3),
    (15, 3),
    (16, 3),
    (18, 3),
    // Row 9, 00cfb9b5..00cfb9e5: 99, 96 and 98 are 3 too.
    (90, 3),
    (91, 3),
    (99, 3),
    (92, 1),
    (93, 1),
    (94, 3),
    (95, 3),
    (96, 3),
    (98, 3),
    (20, 1),
    (21, 1),
    (29, 1),
    (22, 1),
    (23, 1),
    (24, 1),
    (25, 1),
    (26, 3),
    (28, 1),
    (30, 2),
    (31, 1),
    (39, 1),
    (32, 1),
    (33, 1),
    (34, 1),
    (35, 1),
    (36, 3),
    (38, 1),
    (40, 4),
    (41, 3),
    (49, 3),
    (42, 1),
    (43, 1),
    (44, 1),
    (45, 3),
    (46, 3),
    (48, 3),
    (50, 5),
    (51, 3),
    (59, 3),
    (52, 1),
    (53, 1),
    (54, 3),
    (55, 3),
    (56, 3),
    (58, 3),
    (60, 4),
    (82, 1),
    (83, 1),
    (61, 3),
    (69, 3),
    (62, 3),
    (63, 3),
    (64, 3),
    (65, 3),
    (66, 3),
    (68, 3),
    (80, 6),
    (81, 3),
    (89, 3),
    (84, 3),
    (85, 3),
    (86, 3),
    (88, 3),
];

// Translated from 00cfbb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the four debug tables (0xC00 bytes each, through slot 8 of the
/// allocator at `+0x14` of the memory router) and clears their pointers,
/// when the first one exists (no Xbox PDB name).
pub fn fn_00cfbb40(e: &mut Engine, this: Ptr<HkpCollisionDispatcher>) {
    if e.get(this, HkpCollisionDispatcher::m_debugAgent2Table) == 0 {
        return;
    }
    for table in [
        HkpCollisionDispatcher::m_debugAgent2Table,
        HkpCollisionDispatcher::m_debugAgent2TablePred,
        HkpCollisionDispatcher::m_debugAgent3Table,
        HkpCollisionDispatcher::m_debugAgent3TablePred,
    ] {
        let block = e.get(this, table);
        let router = e.call(MEMORY_ROUTER, &[]).u32();
        let allocator = e.mem.u32(router + 0x14);
        e.vcall(allocator, 8, &args![block, 0xc00u32]);
    }
    for table in [
        HkpCollisionDispatcher::m_debugAgent2Table,
        HkpCollisionDispatcher::m_debugAgent2TablePred,
        HkpCollisionDispatcher::m_debugAgent3Table,
        HkpCollisionDispatcher::m_debugAgent3TablePred,
    ] {
        e.set(this, table, 0);
    }
}

/// Copies the five words of an `AgentFuncs` at `funcs` into
/// `m_agent2Func[m_numAgent2Types]`.
fn store_agent2_funcs(e: &mut Engine, this: Ptr<HkpCollisionDispatcher>, funcs: Ptr) {
    let index = e.get(this, HkpCollisionDispatcher::m_numAgent2Types) as u32;
    let entry = dispatcher_address(
        this,
        AGENT2_FUNC.wrapping_add(index.wrapping_mul(AGENT2_FUNCS_SIZE)),
    );
    for word in 0..5 {
        let value = e.mem.u32(funcs.addr() + 4 * word);
        e.mem.set_u32(entry + 4 * word, value);
    }
}

// Translated from 00cfbbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers an agent-2 for the shape type pair (`type_a`, `type_b`) (no
/// Xbox PDB name; it matches Havok's `registerCollisionAgent`, unconfirmed):
/// copies the `AgentFuncs` at `funcs` into `m_agent2Func[m_numAgent2Types]`,
/// registers agent index 1 in the agent-3 type table and the new agent-2
/// index in the agent-2 table, and the same in the two predictive tables
/// when `m_isPredictive` (+0x11) is set; then increments
/// `m_numAgent2Types`.
pub fn fn_00cfbbe0(
    e: &mut Engine,
    this: Ptr<HkpCollisionDispatcher>,
    funcs: Ptr,
    type_a: i32,
    type_b: i32,
) {
    store_agent2_funcs(e, this, funcs);
    let register = hkp_collision_dispatcher_internal_register_collision_agent;
    let debug = e.get(this, HkpCollisionDispatcher::m_debugAgent3Table);
    register(
        e,
        this,
        dispatcher_address(this, AGENT3_TYPES),
        1,
        type_a,
        type_b,
        type_a as u32,
        type_b as u32,
        debug,
        0,
    );
    let index = e.get(this, HkpCollisionDispatcher::m_numAgent2Types) as u32;
    let debug = e.get(this, HkpCollisionDispatcher::m_debugAgent2Table);
    register(
        e,
        this,
        dispatcher_address(this, AGENT2_TYPES),
        index,
        type_a,
        type_b,
        type_a as u32,
        type_b as u32,
        debug,
        0,
    );
    if e.mem.u8(funcs.addr() + 0x11) != 0 {
        let debug = e.get(this, HkpCollisionDispatcher::m_debugAgent3TablePred);
        register(
            e,
            this,
            dispatcher_address(this, AGENT3_TYPES_PRED),
            1,
            type_a,
            type_b,
            type_a as u32,
            type_b as u32,
            debug,
            0,
        );
        let index = e.get(this, HkpCollisionDispatcher::m_numAgent2Types) as u32;
        let debug = e.get(this, HkpCollisionDispatcher::m_debugAgent2TablePred);
        register(
            e,
            this,
            dispatcher_address(this, AGENT2_TYPES_PRED),
            index,
            type_a,
            type_b,
            type_a as u32,
            type_b as u32,
            debug,
            0,
        );
    }
    let count = e.get(this, HkpCollisionDispatcher::m_numAgent2Types);
    e.set(
        this,
        HkpCollisionDispatcher::m_numAgent2Types,
        count.wrapping_add(1),
    );
}

// Translated from 00cfbcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `fn_00cfbbe0` without the agent-3 type tables: copies the
/// `AgentFuncs`, registers the new agent-2 index in `m_agent2Types` (and in
/// `m_agent2TypesPred` when `m_isPredictive` is set) and increments
/// `m_numAgent2Types` (no Xbox PDB name).
pub fn fn_00cfbcc0(
    e: &mut Engine,
    this: Ptr<HkpCollisionDispatcher>,
    funcs: Ptr,
    type_a: i32,
    type_b: i32,
) {
    store_agent2_funcs(e, this, funcs);
    let register = hkp_collision_dispatcher_internal_register_collision_agent;
    let index = e.get(this, HkpCollisionDispatcher::m_numAgent2Types) as u32;
    let debug = e.get(this, HkpCollisionDispatcher::m_debugAgent2Table);
    register(
        e,
        this,
        dispatcher_address(this, AGENT2_TYPES),
        index,
        type_a,
        type_b,
        type_a as u32,
        type_b as u32,
        debug,
        0,
    );
    if e.mem.u8(funcs.addr() + 0x11) != 0 {
        let index = e.get(this, HkpCollisionDispatcher::m_numAgent2Types) as u32;
        let debug = e.get(this, HkpCollisionDispatcher::m_debugAgent2TablePred);
        register(
            e,
            this,
            dispatcher_address(this, AGENT2_TYPES_PRED),
            index,
            type_a,
            type_b,
            type_a as u32,
            type_b as u32,
            debug,
            0,
        );
    }
    let count = e.get(this, HkpCollisionDispatcher::m_numAgent2Types);
    e.set(
        this,
        HkpCollisionDispatcher::m_numAgent2Types,
        count.wrapping_add(1),
    );
}

// Translated from 00cfbd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers an agent-3 (an `Agent3Funcs`, 0x40 bytes, at `funcs`) for the
/// shape type pair (`type_a`, `type_b`) and returns its index in
/// `m_agent3Func` (no Xbox PDB name; it matches Havok's `registerAgent3`,
/// unconfirmed). Sets `m_agent3Registered`.
///
/// When the types differ and `m_ignoreSymmetricVersion` (+0x31) is clear, a
/// copy with `m_symmetric` 2 is stored at `m_numAgent3Types` and registered
/// for the swapped pair (and in the predictive table when `m_isPredictive`,
/// +0x30, is set), `m_numAgent3Types` advances, and the copy that follows
/// has `m_symmetric` 1. Unless `m_reusePreviousEntry` (+0x32) is set the
/// copy is then stored at the next index (advancing `m_numAgent3Types`),
/// else the last stored index is reused. The entry's words +0x44..+0x4C come
/// from uninitialised stack in the game and are left unwritten here.
pub fn fn_00cfbd60(
    e: &mut Engine,
    this: Ptr<HkpCollisionDispatcher>,
    funcs: Ptr,
    type_a: i32,
    type_b: i32,
) -> u32 {
    e.set(this, HkpCollisionDispatcher::m_agent3Registered, true);
    let words: [u32; 16] = std::array::from_fn(|word| e.mem.u32(funcs.addr() + 4 * word as u32));
    let predictive = words[0xc] as u8 != 0;
    let ignore_symmetric = (words[0xc] >> 8) as u8 != 0;
    let reuse_previous = (words[0xc] >> 16) as u8 != 0;
    let register = hkp_collision_dispatcher_internal_register_collision_agent;
    let mut symmetric = 0u32;
    if type_a != type_b && !ignore_symmetric {
        symmetric = 2;
        let index = e.get(this, HkpCollisionDispatcher::m_numAgent3Types) as u32;
        store_agent3_funcs(e, this, index, &words, symmetric);
        let debug = e.get(this, HkpCollisionDispatcher::m_debugAgent3Table);
        register(
            e,
            this,
            dispatcher_address(this, AGENT3_TYPES),
            index,
            type_b,
            type_a,
            type_b as u32,
            type_a as u32,
            debug,
            0,
        );
        if predictive {
            let index = e.get(this, HkpCollisionDispatcher::m_numAgent3Types) as u32;
            let debug = e.get(this, HkpCollisionDispatcher::m_debugAgent3TablePred);
            register(
                e,
                this,
                dispatcher_address(this, AGENT3_TYPES_PRED),
                index,
                type_b,
                type_a,
                type_b as u32,
                type_a as u32,
                debug,
                0,
            );
        }
        let count = e.get(this, HkpCollisionDispatcher::m_numAgent3Types);
        e.set(
            this,
            HkpCollisionDispatcher::m_numAgent3Types,
            count.wrapping_add(1),
        );
        symmetric = 1;
    }
    let mut index = e.get(this, HkpCollisionDispatcher::m_numAgent3Types) as u32;
    if !reuse_previous {
        e.set(
            this,
            HkpCollisionDispatcher::m_numAgent3Types,
            (index as i32).wrapping_add(1),
        );
        store_agent3_funcs(e, this, index, &words, symmetric);
    } else {
        index = index.wrapping_sub(1);
    }
    let debug = e.get(this, HkpCollisionDispatcher::m_debugAgent3Table);
    register(
        e,
        this,
        dispatcher_address(this, AGENT3_TYPES),
        index,
        type_a,
        type_b,
        type_a as u32,
        type_b as u32,
        debug,
        0,
    );
    if predictive {
        let debug = e.get(this, HkpCollisionDispatcher::m_debugAgent3TablePred);
        register(
            e,
            this,
            dispatcher_address(this, AGENT3_TYPES_PRED),
            index,
            type_a,
            type_b,
            type_a as u32,
            type_b as u32,
            debug,
            0,
        );
    }
    index
}

/// Stores the 16 words of an `Agent3Funcs` and `symmetric` (`m_symmetric`,
/// +0x40) as `m_agent3Func[index]`.
fn store_agent3_funcs(
    e: &mut Engine,
    this: Ptr<HkpCollisionDispatcher>,
    index: u32,
    words: &[u32; 16],
    symmetric: u32,
) {
    let entry = dispatcher_address(
        this,
        AGENT3_FUNC.wrapping_add(index.wrapping_mul(AGENT3_FUNCS_SIZE)),
    );
    for (word, value) in words.iter().enumerate() {
        e.mem.set_u32(entry + 4 * word as u32, *value);
    }
    e.mem.set_u32(entry + 0x40, symmetric);
}

// Translated from 00cfbed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionDispatcher::calcContentStatistics` (Xbox PDB): reports the
/// `m_shapeInheritance` array to the collector (its slot 0x14: kind 3, the
/// name `"shapeInheritanceArray"`, the data, its used bytes and
/// `hkStatisticsCollector::getAllocatedSize` of the capacity) when the
/// array is owned (capacity word not negative) or the collector's byte at
/// +0x10 is set, then reports each of the 64 `m_contactMgrFactory` pointers
/// (slot 0xC: `"contactMgrFactory"`, the pointer, 0).
pub fn hkp_collision_dispatcher_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpCollisionDispatcher>,
    collector: u32,
    _unused_1: u32,
) {
    let capacity_and_flags = e.get(
        this,
        HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags,
    );
    if (capacity_and_flags as i32) >= 0 || e.mem.u8(collector + 0x10) != 0 {
        let data = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceData);
        let size = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceSize);
        let capacity_bytes = (capacity_and_flags & 0x3fff_ffff).wrapping_mul(8);
        let allocated = e
            .call(GET_ALLOCATED_SIZE, &args![data, capacity_bytes])
            .u32();
        e.vcall(
            collector,
            0x14,
            &args![
                3u32,
                SHAPE_INHERITANCE_NAME,
                data,
                (size as u32).wrapping_mul(8),
                allocated
            ],
        );
    }
    for index in 0..CONTACT_MGR_FACTORY_COUNT {
        let factory = e.mem.u32(dispatcher_address(this, 0xc + 4 * index));
        e.vcall(
            collector,
            0xc,
            &args![CONTACT_MGR_FACTORY_NAME, factory, 0u32],
        );
    }
}

/// Slots in `m_contactMgrFactory` (8 x 8 shape-type classes).
const CONTACT_MGR_FACTORY_COUNT: u32 = 64;

// Translated from 00cfbf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers that shape type `alternate` can stand in for `primary` (no Xbox
/// PDB name; unconfirmed): removes any equal (`primary`, `alternate`) entry
/// from `m_shapeInheritance`, merges the `m_hasAlternateType` words with
/// `fn_00cfb510`, grows the array when it is full (`00c90940` with the
/// allocator at `+0x10` of the memory router) and appends the entry.
pub fn fn_00cfbf60(
    e: &mut Engine,
    this: Ptr<HkpCollisionDispatcher>,
    primary: u32,
    alternate: u32,
) {
    let mut index = 0;
    while index < e.get(this, HkpCollisionDispatcher::m_shapeInheritanceSize) {
        let (entry_primary, entry_alternate) = shape_inheritance_entry(e, this, index);
        if entry_primary == primary && entry_alternate == alternate {
            let size = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceSize) - 1;
            e.set(this, HkpCollisionDispatcher::m_shapeInheritanceSize, size);
            let data = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceData);
            for entry in index..size {
                let from = data.wrapping_add((entry as u32 + 1).wrapping_mul(8));
                let to = data.wrapping_add((entry as u32).wrapping_mul(8));
                let (first, second) = (e.mem.u32(from), e.mem.u32(from + 4));
                e.mem.set_u32(to, first);
                e.mem.set_u32(to + 4, second);
            }
        } else {
            index += 1;
        }
    }
    fn_00cfb510(e, this, primary, alternate, 0);
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let capacity_and_flags = e.get(
        this,
        HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags,
    );
    if e.get(this, HkpCollisionDispatcher::m_shapeInheritanceSize)
        == (capacity_and_flags & 0x3fff_ffff) as i32
    {
        let allocator = e.mem.u32(router + 0x10);
        let array = dispatcher_address(this, 0x1e24);
        e.call(ARRAY_GROW, &args![allocator, array, 8u32]);
    }
    let size = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceSize);
    let data = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceData);
    let entry = data.wrapping_add((size as u32).wrapping_mul(8));
    e.mem.set_u32(entry, primary);
    e.mem.set_u32(entry + 4, alternate);
    e.set(
        this,
        HkpCollisionDispatcher::m_shapeInheritanceSize,
        size.wrapping_add(1),
    );
}

// Translated from 00cfc030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor (no Xbox PDB name; unconfirmed): stores the default
/// agent, a reference count of 1 and the vtable, clears the agent function
/// flags and the agent-3 entries' tail, empties the shape inheritance array
/// and the debug table pointers, enables the check, stores
/// `contact_mgr_factory` in all 64 factory slots (taking a reference on
/// each when it is not null), sets `m_hasAlternateType[i]` to `1 << i`, runs
/// `fn_00cfb080` and clears `m_agent3Registered`. Returns `this`.
pub fn fn_00cfc030(
    e: &mut Engine,
    this: Ptr<HkpCollisionDispatcher>,
    default_agent: u32,
    contact_mgr_factory: u32,
) -> Ptr<HkpCollisionDispatcher> {
    e.set(
        this,
        HkpCollisionDispatcher::m_defaultCollisionAgent,
        default_agent,
    );
    e.set(this, HkpCollisionDispatcher::m_referenceCount, 1);
    e.set(this, HkpCollisionDispatcher::vtable, DISPATCHER_VTABLE);
    for index in 0..64 {
        let entry = dispatcher_address(this, AGENT2_FUNC + 0x10 + index * AGENT2_FUNCS_SIZE);
        e.mem.set_u8(entry, 0);
        e.mem.set_u8(entry + 1, 0);
    }
    for index in 0..16 {
        let entry = dispatcher_address(this, AGENT3_FUNC + index * AGENT3_FUNCS_SIZE);
        e.mem.set_u8(entry + 0x30, 0);
        e.mem.set_u8(entry + 0x31, 0);
        e.mem.set_u8(entry + 0x32, 0);
        for offset in [0x18, 0x1c, 0x20, 0x24, 0x28] {
            e.mem.set_u32(entry + offset, 0);
        }
    }
    e.set(this, HkpCollisionDispatcher::m_shapeInheritanceData, 0);
    e.set(this, HkpCollisionDispatcher::m_shapeInheritanceSize, 0);
    e.set(
        this,
        HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags,
        0x8000_0000,
    );
    e.set(this, HkpCollisionDispatcher::m_debugAgent2Table, 0);
    e.set(this, HkpCollisionDispatcher::m_debugAgent2TablePred, 0);
    e.set(this, HkpCollisionDispatcher::m_debugAgent3Table, 0);
    e.set(this, HkpCollisionDispatcher::m_debugAgent3TablePred, 0);
    e.set(
        this,
        HkpCollisionDispatcher::m_collisionAgentRegistered,
        false,
    );
    e.set(this, HkpCollisionDispatcher::m_checkEnabled, true);
    e.set(this, HkpCollisionDispatcher::m_numAgent3Types, 0);
    e.set(
        this,
        HkpCollisionDispatcher::m_midphaseAgent3Registered,
        false,
    );
    for index in 0..CONTACT_MGR_FACTORY_COUNT {
        e.mem.set_u32(
            dispatcher_address(this, 0xc + 4 * index),
            contact_mgr_factory,
        );
        if contact_mgr_factory != 0 {
            e.call(ADD_REFERENCE, &args![contact_mgr_factory]);
        }
    }
    for index in 0..TYPE_COUNT {
        e.mem.set_u32(
            dispatcher_address(this, HAS_ALTERNATE_TYPE + 4 * index),
            1 << index,
        );
    }
    fn_00cfb080(e, this);
    e.set(this, HkpCollisionDispatcher::m_agent3Registered, false);
    this
}

// Translated from 00cfc140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body (no Xbox PDB name): sets the vtable, frees the debug
/// tables (`fn_00cfbb40`), releases each non-null contact manager factory
/// (`00c905b0`), empties the shape inheritance array (freeing a buffer it
/// owns through slot 0x10 of the heap allocator) and leaves the base
/// class's vtable (`0102e388`).
pub fn fn_00cfc140(e: &mut Engine, this: Ptr<HkpCollisionDispatcher>) {
    e.set(this, HkpCollisionDispatcher::vtable, DISPATCHER_VTABLE);
    fn_00cfbb40(e, this);
    for index in 0..CONTACT_MGR_FACTORY_COUNT {
        let factory = e.mem.u32(dispatcher_address(this, 0xc + 4 * index));
        if factory != 0 {
            e.call(REMOVE_REFERENCE, &args![factory]);
        }
    }
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.set(this, HkpCollisionDispatcher::m_shapeInheritanceSize, 0);
    let capacity_and_flags = e.get(
        this,
        HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags,
    );
    if (capacity_and_flags as i32) >= 0 {
        let data = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceData);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity_and_flags & 0x3fff_ffff).wrapping_mul(8)],
        );
    }
    e.set(this, HkpCollisionDispatcher::m_shapeInheritanceData, 0);
    e.set(
        this,
        HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags,
        0x8000_0000,
    );
    e.set(
        this,
        HkpCollisionDispatcher::vtable,
        REFERENCED_OBJECT_VTABLE,
    );
}

// Translated from 00cfc550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollisionDispatcher::__scalar_deleting_destructor` (Xbox PDB): the
/// destructor body, then, when bit 0 of `flags` is set, the object's memory
/// returned through slot 8 of the heap allocator with the size in
/// `m_memSizeAndFlags`. Returns `this`.
pub fn hkp_collision_dispatcher_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HkpCollisionDispatcher>,
    flags: u32,
) -> Ptr<HkpCollisionDispatcher> {
    fn_00cfc140(e, this);
    if flags & 1 != 0 {
        let size = e.get(this, HkpCollisionDispatcher::m_memSizeAndFlags) as u32;
        let router = e.call(MEMORY_ROUTER, &[]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cfb080, fn_00cfb080(Ptr<HkpCollisionDispatcher>)),
        entry!(
            0x00cfb2a0,
            hkp_collision_dispatcher_internal_register_collision_agent(
                Ptr<HkpCollisionDispatcher>,
                u32,
                u32,
                i32,
                i32,
                u32,
                u32,
                u32,
                i32,
            )
        ),
        entry!(
            0x00cfb510,
            fn_00cfb510(Ptr<HkpCollisionDispatcher>, u32, u32, u32)
        ),
        entry!(0x00cfb570, fn_00cfb570(Ptr<HkpCollisionDispatcher>, Ptr)),
        entry!(0x00cfbb40, fn_00cfbb40(Ptr<HkpCollisionDispatcher>)),
        entry!(
            0x00cfbbe0,
            fn_00cfbbe0(Ptr<HkpCollisionDispatcher>, Ptr, i32, i32)
        ),
        entry!(
            0x00cfbcc0,
            fn_00cfbcc0(Ptr<HkpCollisionDispatcher>, Ptr, i32, i32)
        ),
        entry!(
            0x00cfbd60,
            fn_00cfbd60(Ptr<HkpCollisionDispatcher>, Ptr, i32, i32) -> u32
        ),
        entry!(
            0x00cfbed0,
            hkp_collision_dispatcher_calc_content_statistics(Ptr<HkpCollisionDispatcher>, u32, u32)
        ),
        entry!(
            0x00cfbf60,
            fn_00cfbf60(Ptr<HkpCollisionDispatcher>, u32, u32)
        ),
        entry!(
            0x00cfc030,
            fn_00cfc030(Ptr<HkpCollisionDispatcher>, u32, u32) -> Ptr<HkpCollisionDispatcher>
        ),
        entry!(0x00cfc140, fn_00cfc140(Ptr<HkpCollisionDispatcher>)),
        entry!(
            0x00cfc550,
            hkp_collision_dispatcher_scalar_deleting_destructor(
                Ptr<HkpCollisionDispatcher>,
                u32,
            )
                -> Ptr<HkpCollisionDispatcher>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    /// The fake allocator object's vtable and its two handlers.
    const ALLOCATOR_VTABLE: u32 = 0x0200_0000;
    const FREE_BLOCK_HANDLER: u32 = 0x0200_0100;
    const FREE_BUFFER_HANDLER: u32 = 0x0200_0104;
    /// The fake statistics collector's vtable and its two handlers.
    const COLLECTOR_VTABLE: u32 = 0x0200_0200;
    const REPORT_FACTORY_HANDLER: u32 = 0x0200_0210;
    const REPORT_ARRAY_HANDLER: u32 = 0x0200_0214;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x0200_0000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn dispatcher(e: &mut Engine) -> Ptr<HkpCollisionDispatcher> {
        e.new_object::<HkpCollisionDispatcher>()
    }

    fn byte(e: &Engine, this: Ptr<HkpCollisionDispatcher>, offset: u32) -> u8 {
        e.mem.u8(this.addr() + offset)
    }

    /// Installs the memory router double (its `+0x10` and `+0x14` both hold
    /// one allocator object) and returns what slot 8 (free a block) and
    /// slot 0x10 (free a buffer) of the allocator were called with.
    fn install_allocator(e: &mut Engine) -> (Calls, Calls) {
        e.put_vtable(
            ALLOCATOR_VTABLE,
            &[0, 0, FREE_BLOCK_HANDLER, 0, FREE_BUFFER_HANDLER],
        );
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(router + 0x14, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        let blocks: Calls = Rc::default();
        let buffers: Calls = Rc::default();
        let log = blocks.clone();
        e.register_double(FREE_BLOCK_HANDLER, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let log = buffers.clone();
        e.register_double(FREE_BUFFER_HANDLER, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (blocks, buffers)
    }

    fn inheritance_array(
        e: &mut Engine,
        this: Ptr<HkpCollisionDispatcher>,
        entries: &[(u32, u32)],
        capacity: u32,
    ) {
        let data = e.mem.alloc(8 * capacity);
        for (index, (primary, alternate)) in entries.iter().enumerate() {
            e.mem.set_u32(data + 8 * index as u32, *primary);
            e.mem.set_u32(data + 8 * index as u32 + 4, *alternate);
        }
        e.set(this, HkpCollisionDispatcher::m_shapeInheritanceData, data);
        e.set(
            this,
            HkpCollisionDispatcher::m_shapeInheritanceSize,
            entries.len() as i32,
        );
        e.set(
            this,
            HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags,
            capacity,
        );
    }

    fn inheritance_entries(e: &Engine, this: Ptr<HkpCollisionDispatcher>) -> Vec<(u32, u32)> {
        let size = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceSize);
        (0..size)
            .map(|index| shape_inheritance_entry(e, this, index))
            .collect()
    }

    #[test]
    fn init_tables_resets_the_agent_tables() {
        let mut e = engine();
        e.register(REGISTER_DEFAULT_AGENT3, |_, _| Ret::default());
        let this = dispatcher(&mut e);
        let tables: Vec<u32> = (0..4).map(|_| e.mem.alloc(0xc00)).collect();
        e.set(this, HkpCollisionDispatcher::m_debugAgent2Table, tables[0]);
        e.set(
            this,
            HkpCollisionDispatcher::m_debugAgent2TablePred,
            tables[1],
        );
        e.set(this, HkpCollisionDispatcher::m_debugAgent3Table, tables[2]);
        e.set(
            this,
            HkpCollisionDispatcher::m_debugAgent3TablePred,
            tables[3],
        );
        e.mem.set_u8(this.addr() + AGENT2_TYPES + 5, 9);
        e.mem.set_u8(this.addr() + AGENT3_TYPES_PRED + 1023, 9);
        e.set(
            this,
            HkpCollisionDispatcher::m_collisionAgentRegistered,
            true,
        );
        e.call(0x00cfb080, &args![this]);
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent2Types), 1);
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent3Types), 1);
        for table in &tables {
            assert_eq!(e.mem.u8(table + 3 * 7 + 2), UNSET_PRIORITY);
            assert_eq!(e.mem.u8(table + 3 * 1023 + 2), UNSET_PRIORITY);
            assert_eq!(e.mem.u8(table + 3 * 7), 0);
        }
        assert_eq!(byte(&e, this, AGENT2_TYPES + 5), 0);
        assert_eq!(byte(&e, this, AGENT3_TYPES_PRED + 1023), 0);
        assert_eq!(e.mem.u32(this.addr() + AGENT2_FUNC), 0x00d1_0790);
        assert_eq!(e.mem.u32(this.addr() + AGENT2_FUNC + 0xc), 0x00d1_0780);
        assert_eq!(byte(&e, this, AGENT2_FUNC + 0x10), 0);
        assert_eq!(byte(&e, this, AGENT2_FUNC + 0x11), 1);
        assert_eq!(e.mem.u32(this.addr() + AGENT3_FUNC), 0x00cf_b050);
        assert_eq!(e.mem.u32(this.addr() + AGENT3_FUNC + 4), 0x00cf_b060);
        assert_eq!(e.mem.u32(this.addr() + AGENT3_FUNC + 0x2c), 0x00cf_b070);
        assert!(!e.get(this, HkpCollisionDispatcher::m_collisionAgentRegistered));
        assert_eq!(
            calls_to(&e, REGISTER_DEFAULT_AGENT3),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn init_tables_without_debug_tables() {
        let mut e = engine();
        e.register(REGISTER_DEFAULT_AGENT3, |_, _| Ret::default());
        let this = dispatcher(&mut e);
        e.call(0x00cfb080, &args![this]);
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent2Types), 1);
        assert_eq!(byte(&e, this, AGENT2_FUNC + 0x11), 1);
    }

    #[test]
    fn register_collision_agent_fills_the_pair() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        let table = e.mem.alloc(1024);
        e.call(
            0x00cfb2a0,
            &args![this, table, 7u32, 3i32, 4i32, 3u32, 4u32, 0u32, 0i32],
        );
        assert_eq!(e.mem.u8(table + 3 * 32 + 4), 7);
        assert_eq!(e.mem.u8(table + 4 * 32 + 3), 0);
        assert!(e.get(this, HkpCollisionDispatcher::m_collisionAgentRegistered));
    }

    #[test]
    fn register_collision_agent_wildcard_covers_every_type() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        let table = e.mem.alloc(1024);
        e.call(
            0x00cfb2a0,
            &args![
                this,
                table,
                9u32,
                -1i32,
                2i32,
                0xffff_ffffu32,
                2u32,
                0u32,
                0i32
            ],
        );
        assert_eq!(e.mem.u8(table + 2), 0);
        for first in 1..32 {
            assert_eq!(e.mem.u8(table + first * 32 + 2), 9);
            assert_eq!(e.mem.u8(table + first * 32 + 3), 0);
        }
    }

    #[test]
    fn register_collision_agent_follows_shape_inheritance() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        inheritance_array(&mut e, this, &[(9, 3), (12, 4)], 4);
        let table = e.mem.alloc(1024);
        e.call(
            0x00cfb2a0,
            &args![this, table, 5u32, 3i32, 4i32, 3u32, 4u32, 0u32, 0i32],
        );
        for (first, second) in [(3, 4), (9, 4), (3, 12), (9, 12)] {
            assert_eq!(e.mem.u8(table + first * 32 + second), 5, "{first},{second}");
        }
        assert_eq!(e.mem.u8(table + 4 * 32 + 3), 0);
    }

    #[test]
    fn register_collision_agent_writes_the_debug_entry() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        e.set(this, HkpCollisionDispatcher::m_checkEnabled, true);
        let table = e.mem.alloc(1024);
        let debug = e.mem.alloc(0xc00);
        let entry = debug + 3 * (3 * 32 + 4);
        e.mem.set_u8(entry + 2, UNSET_PRIORITY);
        e.call(
            0x00cfb2a0,
            &args![this, table, 5u32, 3i32, 4i32, 6u32, 7u32, debug, 0i32],
        );
        assert_eq!(e.mem.u8(entry), 6);
        assert_eq!(e.mem.u8(entry + 1), 7);
        assert_eq!(e.mem.u8(entry + 2), 0);
        assert!(calls_to(&e, GET_SHAPE_TYPE_NAME).is_empty());
    }

    #[test]
    fn register_collision_agent_warns_when_overriding_a_specialised_agent() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        e.set(this, HkpCollisionDispatcher::m_checkEnabled, true);
        e.register(GET_SHAPE_TYPE_NAME, |_, a| {
            ret(0x0100_1000u32.wrapping_add(a[0]))
        });
        e.register(STRING_SNPRINTF, |_, _| Ret::default());
        let table = e.mem.alloc(1024);
        let debug = e.mem.alloc(0xc00);
        let entry = debug + 3 * (3 * 32 + 2);
        e.mem.set_u8(entry, 8);
        e.mem.set_u8(entry + 1, 9);
        e.mem.set_u8(entry + 2, 0);
        e.call(
            0x00cfb2a0,
            &args![
                this,
                table,
                5u32,
                3i32,
                -1i32,
                3u32,
                0xffff_ffffu32,
                debug,
                0i32
            ],
        );
        let names = calls_to(&e, GET_SHAPE_TYPE_NAME);
        assert_eq!(names.len(), 4 * 31);
        let first_four: Vec<u32> = names.iter().take(4).map(|call| call[0]).collect();
        assert_eq!(first_four[2], 3);
        assert_eq!(first_four[3], 0xffff_ffff);
        let formats = calls_to(&e, STRING_SNPRINTF);
        assert_eq!(formats.len(), 31);
        assert_eq!(formats[0][1], 1000);
        assert_eq!(formats[0][2], OVERRIDE_WARNING_FORMAT);
        assert_eq!(formats[0][3], 0x0100_1000 + 3);
        assert_eq!(e.mem.u8(entry + 2), 1);
    }

    #[test]
    fn merge_alternate_type_flags() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        inheritance_array(&mut e, this, &[(5, 2)], 4);
        for index in 0..32 {
            e.mem
                .set_u32(this.addr() + HAS_ALTERNATE_TYPE + 4 * index, 1 << index);
        }
        e.call(0x00cfb510, &args![this, 2u32, 1u32, 0u32]);
        let flags =
            |e: &Engine, index: u32| e.mem.u32(this.addr() + HAS_ALTERNATE_TYPE + 4 * index);
        assert_eq!(flags(&e, 2), 0b110);
        assert_eq!(flags(&e, 5), 0b100000 | 0b110);
        assert_eq!(flags(&e, 1), 0b10);
    }

    /// Sets the constants of the quality set-up (read from the exe).
    fn quality_constants(e: &mut Engine) {
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(MINUS_TWO_DOUBLE, -2.0f64);
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        e.set_global(MINUS_HALF_DOUBLE, -0.5f64);
        e.set_global(MINUS_THREE_TENTHS_DOUBLE, -0.3f32 as f64);
        e.set_global(FIVE_DOUBLE, 5.0f64);
        e.set_global(FIVE_THOUSANDTHS_DOUBLE, 0.005f32 as f64);
        e.set_global(MINUS_THIRTY_FIVE_HUNDREDTHS_DOUBLE, -0.35f32 as f64);
        e.set_global(MINUS_SIX_TENTHS_DOUBLE, -0.6f32 as f64);
        e.set_global(MINUS_FORTY_TWO_HUNDREDTHS_DOUBLE, -0.42f64);
        e.set_global(MINUS_HUGE_DOUBLE, -1e20f64);
        e.set_global(LARGE_FLOAT, f32::from_bits(0x7f7f_ffee));
        e.set_global(KEEP_CONTACT_FLOAT_A, f32::from_bits(0xe02d_78ec));
        e.set_global(KEEP_CONTACT_FLOAT_B, f32::from_bits(0xdf0a_c723));
        e.set_global(MINUS_HALF_FLOAT, -0.5f32);
        e.set_global(MINUS_QUARTER_FLOAT, -0.25f32);
        e.set_global(MINUS_TWO_FIFTHS_FLOAT, f32::from_bits(0xbecc_cccd));
        e.set_global(MINUS_ONE_FIFTH_FLOAT, f32::from_bits(0xbe4c_cccd));
    }

    /// An `InitCollisionQualityInfo`: gravity 10, tolerance 0.1, time step
    /// 0.5, velocity 100, the four toi counts 3, 5, 2, 3, priorities
    /// 0x10..0x40 and the four flag bytes.
    fn quality_input(e: &mut Engine, continuous: bool) -> Ptr {
        let info = e.mem.alloc(0x30);
        for (offset, value) in [
            (0u32, 10.0f32),
            (4, 0.1),
            (8, 0.5),
            (0xc, 100.0),
            (0x10, 3.0),
            (0x14, 5.0),
            (0x18, 2.0),
            (0x1c, 3.0),
        ] {
            e.mem.set_f32(info + offset, value);
        }
        for (index, priority) in [0x10u16, 0x20, 0x30, 0x40].into_iter().enumerate() {
            e.mem.set_u16(info + 0x20 + 2 * index as u32, priority);
        }
        e.mem.set_u8(info + 0x28, continuous as u8);
        e.mem.set_u8(info + 0x29, 1);
        e.mem.set_u8(info + 0x2a, 1);
        e.mem.set_u8(info + 0x2b, 1);
        Ptr::new(info)
    }

    fn install_memset(e: &mut Engine) {
        e.register_double(MEMSET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            ret(a[0])
        });
    }

    #[test]
    fn quality_info_with_continuous_detection() {
        let mut e = engine();
        quality_constants(&mut e);
        install_memset(&mut e);
        let this = dispatcher(&mut e);
        let info = quality_input(&mut e, true);
        e.call(0x00cfb570, &args![this, info]);
        assert_eq!(
            e.get(this, HkpCollisionDispatcher::m_expectedMinPsiDeltaTime),
            0.5
        );
        assert_eq!(
            e.get(this, HkpCollisionDispatcher::m_expectedMaxLinearVelocity),
            100.0
        );
        let slot = |index: u32| quality_slot(this, index);
        let f = |e: &Engine, base: u32, offset: u32| e.mem.f32(base + offset);
        // Entry 1: tolerance 0.1, the two negative variants of 1.25.
        assert_eq!(f(&e, slot(1), 0), 0.1);
        assert_eq!(f(&e, slot(1), 4), 0.1);
        assert_eq!(f(&e, slot(1), 0xc), -2.5);
        assert_eq!(f(&e, slot(1), 8), -1.25);
        assert_eq!(e.mem.u32(slot(1) + 0x34), 0x7f7f_ffee);
        assert_eq!(e.mem.u32(slot(1) + 0x10), 0);
        assert_eq!(e.mem.u16(slot(1) + 0x3c), 0x10);
        assert_eq!(e.mem.u8(slot(1) + 0x3e), 1);
        assert_eq!(f(&e, slot(1), 0x30), 1e20);
        assert_eq!(f(&e, slot(1), 0x20), 1.0);
        assert_eq!(e.mem.u32(slot(1) + 0x18), 0xe02d_78ec);
        // Entry 7: a copy with the three tolerances.
        assert_eq!(f(&e, slot(7), 0xc), 0.1);
        assert_eq!(f(&e, slot(7), 8), 0.1);
        assert_eq!(f(&e, slot(7), 0x34), f32::from_bits(0x7f7f_ffee));
        // Entries 2, 3, 4: time-of-impact thresholds.
        assert_eq!(e.mem.u32(slot(2) + 0x10), 1);
        assert_eq!(e.mem.u8(slot(2) + 0x14), 1);
        assert_eq!(f(&e, slot(2), 0x1c), -0.25);
        assert_eq!(f(&e, slot(2), 0x18), -0.5);
        assert_eq!(f(&e, slot(2), 0x20), (5.0f64 / 100.0) as f32);
        assert_eq!(f(&e, slot(2), 0x34), 0.25);
        assert_eq!(f(&e, slot(2), 0x38), (0.5f64 / 100.0) as f32);
        assert_eq!(e.mem.u16(slot(2) + 0x3c), 0x20);
        assert_eq!(e.mem.u8(slot(3) + 0x14), 0);
        assert_eq!(f(&e, slot(3), 0x1c), -0.125);
        assert_eq!(e.mem.u16(slot(3) + 0x3c), 0x20);
        assert_eq!(f(&e, slot(4), 0x1c), -0.5);
        assert_eq!(f(&e, slot(4), 0x34), 0.5);
        assert_eq!(f(&e, slot(4), 0x38), (0.5f64 / 100.0) as f32);
        assert_eq!(e.mem.u16(slot(4) + 0x3c), 0x30);
        // Entries 5 and 6: the forced time of impact.
        let forced = ((-0.6f32 as f64) / 2.0) as f32;
        assert_eq!(f(&e, slot(5), 0x1c), forced);
        assert_eq!(f(&e, slot(5), 0x18), f32::from_bits(0xbecc_cccd));
        assert_eq!(f(&e, slot(5), 0x28), f32::from_bits(0xbe4c_cccd));
        assert_eq!(e.mem.u16(slot(5) + 0x3c), 0x40);
        assert_eq!(e.mem.u32(slot(5) + 0x10), 1);
        assert_eq!(f(&e, slot(6), 0x1c), forced);
        assert_eq!(f(&e, slot(6), 0xc), 0.1);
        assert_eq!(f(&e, slot(6), 8), 0.1);
        // The quality table.
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![vec![this.addr() + COLLISION_QUALITY_TABLE, 1, 100]]
        );
        let table = |index: u32| byte(&e, this, COLLISION_QUALITY_TABLE + index);
        assert_eq!(table(0), 0);
        assert_eq!(table(3), 2);
        assert_eq!(table(8), 6);
        assert_eq!(table(17), 1);
        assert_eq!(table(30), 2);
        assert_eq!(table(50), 5);
        assert_eq!(table(80), 6);
        // 00cfb9c1: [9][9] is 3; 00cfb9df / 00cfb9e5: [9][6] and [9][8] too.
        assert_eq!(table(99), 3);
        assert_eq!(table(96), 3);
        assert_eq!(table(98), 3);
        assert_eq!(table(97), 1);
    }

    #[test]
    fn quality_table_row_nine_matches_column_nine() {
        let mut e = engine();
        quality_constants(&mut e);
        install_memset(&mut e);
        let this = dispatcher(&mut e);
        let info = quality_input(&mut e, true);
        e.call(0x00cfb570, &args![this, info]);
        let table = |index: u32| byte(&e, this, COLLISION_QUALITY_TABLE + index);
        // Row 9 as stored at 00cfb9b5..00cfb9e5 (97 keeps the memset 1).
        let row: Vec<u8> = (90..100).map(&table).collect();
        assert_eq!(row, vec![3, 3, 1, 1, 3, 3, 3, 1, 3, 3]);
        let column: Vec<u8> = (0..10).map(|r| table(r * 10 + 9)).collect();
        assert_eq!(column, row);
    }

    #[test]
    fn quality_info_without_continuous_detection() {
        let mut e = engine();
        quality_constants(&mut e);
        install_memset(&mut e);
        let this = dispatcher(&mut e);
        let info = quality_input(&mut e, false);
        e.mem.set_u8(info.addr() + 0x29, 0);
        e.mem.set_u8(info.addr() + 0x2a, 0);
        e.call(0x00cfb570, &args![this, info]);
        let slot = |index: u32| quality_slot(this, index);
        assert_eq!(e.mem.f32(slot(1) + 0xc), 0.1);
        assert_eq!(e.mem.f32(slot(1) + 8), 0.1);
        assert_eq!(e.mem.u32(slot(2) + 0x10), 0);
        assert_eq!(e.mem.u32(slot(5) + 0x10), 0);
        assert_eq!(e.mem.f32(slot(5)), 0.1);
        assert_eq!(e.mem.f32(slot(2) + 0x1c), e.mem.f32(slot(1) + 0x1c));
    }

    #[test]
    fn free_debug_tables() {
        let mut e = engine();
        let (blocks, _) = install_allocator(&mut e);
        let this = dispatcher(&mut e);
        let tables: Vec<u32> = (0..4).map(|_| e.mem.alloc(0xc00)).collect();
        e.set(this, HkpCollisionDispatcher::m_debugAgent2Table, tables[0]);
        e.set(
            this,
            HkpCollisionDispatcher::m_debugAgent2TablePred,
            tables[1],
        );
        e.set(this, HkpCollisionDispatcher::m_debugAgent3Table, tables[2]);
        e.set(
            this,
            HkpCollisionDispatcher::m_debugAgent3TablePred,
            tables[3],
        );
        e.call(0x00cfbb40, &args![this]);
        let freed: Vec<u32> = blocks.borrow().iter().map(|call| call[1]).collect();
        assert_eq!(freed, tables);
        assert!(blocks.borrow().iter().all(|call| call[2] == 0xc00));
        assert_eq!(
            e.get(this, HkpCollisionDispatcher::m_debugAgent3TablePred),
            0
        );
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_debugAgent2Table), 0);
    }

    #[test]
    fn free_debug_tables_does_nothing_without_tables() {
        let mut e = engine();
        let (blocks, _) = install_allocator(&mut e);
        let this = dispatcher(&mut e);
        e.call(0x00cfbb40, &args![this]);
        assert!(blocks.borrow().is_empty());
        assert!(calls_to(&e, MEMORY_ROUTER).is_empty());
    }

    /// An `AgentFuncs` of five words (the last is the flipped and predictive
    /// bytes).
    fn agent2_funcs(e: &mut Engine, predictive: bool) -> Ptr {
        let funcs = e.mem.alloc(0x14);
        for (index, value) in [0x11u32, 0x22, 0x33, 0x44].into_iter().enumerate() {
            e.mem.set_u32(funcs + 4 * index as u32, value);
        }
        e.mem.set_u8(funcs + 0x11, predictive as u8);
        Ptr::new(funcs)
    }

    #[test]
    fn register_agent2_and_agent3_types() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        e.set(this, HkpCollisionDispatcher::m_numAgent2Types, 3);
        let funcs = agent2_funcs(&mut e, true);
        e.call(0x00cfbbe0, &args![this, funcs, 4i32, 5i32]);
        let entry = this.addr() + AGENT2_FUNC + 3 * AGENT2_FUNCS_SIZE;
        assert_eq!(e.mem.u32(entry), 0x11);
        assert_eq!(e.mem.u32(entry + 0xc), 0x44);
        assert_eq!(e.mem.u8(entry + 0x11), 1);
        let pair = 4 * 32 + 5;
        assert_eq!(byte(&e, this, AGENT3_TYPES + pair), 1);
        assert_eq!(byte(&e, this, AGENT2_TYPES + pair), 3);
        assert_eq!(byte(&e, this, AGENT3_TYPES_PRED + pair), 1);
        assert_eq!(byte(&e, this, AGENT2_TYPES_PRED + pair), 3);
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent2Types), 4);
    }

    #[test]
    fn register_agent2_and_agent3_types_not_predictive() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        let funcs = agent2_funcs(&mut e, false);
        e.call(0x00cfbbe0, &args![this, funcs, 4i32, 5i32]);
        let pair = 4 * 32 + 5;
        assert_eq!(byte(&e, this, AGENT2_TYPES + pair), 0);
        assert_eq!(byte(&e, this, AGENT3_TYPES + pair), 1);
        assert_eq!(byte(&e, this, AGENT3_TYPES_PRED + pair), 0);
        assert_eq!(byte(&e, this, AGENT2_TYPES_PRED + pair), 0);
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent2Types), 1);
    }

    #[test]
    fn register_agent2_types() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        e.set(this, HkpCollisionDispatcher::m_numAgent2Types, 2);
        let funcs = agent2_funcs(&mut e, true);
        e.call(0x00cfbcc0, &args![this, funcs, 6i32, 7i32]);
        let pair = 6 * 32 + 7;
        let entry = this.addr() + AGENT2_FUNC + 2 * AGENT2_FUNCS_SIZE;
        assert_eq!(e.mem.u32(entry + 4), 0x22);
        assert_eq!(byte(&e, this, AGENT2_TYPES + pair), 2);
        assert_eq!(byte(&e, this, AGENT2_TYPES_PRED + pair), 2);
        assert_eq!(byte(&e, this, AGENT3_TYPES + pair), 0);
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent2Types), 3);
    }

    /// An `Agent3Funcs`: word 0 is 0x1111, byte 0x30 predictive, 0x31
    /// ignore-symmetric, 0x32 reuse-previous.
    fn agent3_funcs(e: &mut Engine, predictive: bool, ignore_symmetric: bool, reuse: bool) -> Ptr {
        let funcs = e.mem.alloc(0x40);
        e.mem.set_u32(funcs, 0x1111);
        e.mem.set_u8(funcs + 0x30, predictive as u8);
        e.mem.set_u8(funcs + 0x31, ignore_symmetric as u8);
        e.mem.set_u8(funcs + 0x32, reuse as u8);
        Ptr::new(funcs)
    }

    #[test]
    fn register_agent3_with_symmetric_copy() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        e.set(this, HkpCollisionDispatcher::m_numAgent3Types, 1);
        let funcs = agent3_funcs(&mut e, true, false, false);
        let index = e.call(0x00cfbd60, &args![this, funcs, 2i32, 3i32]).u32();
        assert_eq!(index, 2);
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent3Types), 3);
        assert!(e.get(this, HkpCollisionDispatcher::m_agent3Registered));
        let entry = |index: u32| this.addr() + AGENT3_FUNC + index * AGENT3_FUNCS_SIZE;
        assert_eq!(e.mem.u32(entry(1)), 0x1111);
        assert_eq!(e.mem.u32(entry(1) + 0x40), 2);
        assert_eq!(e.mem.u32(entry(2)), 0x1111);
        assert_eq!(e.mem.u32(entry(2) + 0x40), 1);
        assert_eq!(byte(&e, this, AGENT3_TYPES + 3 * 32 + 2), 1);
        assert_eq!(byte(&e, this, AGENT3_TYPES + 2 * 32 + 3), 2);
        assert_eq!(byte(&e, this, AGENT3_TYPES_PRED + 3 * 32 + 2), 1);
        assert_eq!(byte(&e, this, AGENT3_TYPES_PRED + 2 * 32 + 3), 2);
    }

    #[test]
    fn register_agent3_same_types_is_not_mirrored() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        e.set(this, HkpCollisionDispatcher::m_numAgent3Types, 1);
        let funcs = agent3_funcs(&mut e, false, false, false);
        let index = e.call(0x00cfbd60, &args![this, funcs, 4i32, 4i32]).u32();
        assert_eq!(index, 1);
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent3Types), 2);
        let entry = this.addr() + AGENT3_FUNC + AGENT3_FUNCS_SIZE;
        assert_eq!(e.mem.u32(entry + 0x40), 0);
        assert_eq!(byte(&e, this, AGENT3_TYPES + 4 * 32 + 4), 1);
        assert_eq!(byte(&e, this, AGENT3_TYPES_PRED + 4 * 32 + 4), 0);
    }

    #[test]
    fn register_agent3_reusing_the_previous_entry() {
        let mut e = engine();
        let this = dispatcher(&mut e);
        e.set(this, HkpCollisionDispatcher::m_numAgent3Types, 5);
        let funcs = agent3_funcs(&mut e, false, true, true);
        let index = e.call(0x00cfbd60, &args![this, funcs, 2i32, 3i32]).u32();
        assert_eq!(index, 4);
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent3Types), 5);
        assert_eq!(
            e.mem.u32(this.addr() + AGENT3_FUNC + 4 * AGENT3_FUNCS_SIZE),
            0
        );
        assert_eq!(byte(&e, this, AGENT3_TYPES + 2 * 32 + 3), 4);
    }

    /// A statistics collector whose slot 0xC and 0x14 record their calls.
    fn collector(e: &mut Engine, owned_only: bool) -> (u32, Calls, Calls) {
        e.put_vtable(
            COLLECTOR_VTABLE,
            &[0, 0, 0, REPORT_FACTORY_HANDLER, 0, REPORT_ARRAY_HANDLER],
        );
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, COLLECTOR_VTABLE);
        e.mem.set_u8(object + 0x10, !owned_only as u8);
        let factories: Calls = Rc::default();
        let arrays: Calls = Rc::default();
        let log = factories.clone();
        e.register_double(REPORT_FACTORY_HANDLER, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let log = arrays.clone();
        e.register_double(REPORT_ARRAY_HANDLER, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (object, factories, arrays)
    }

    #[test]
    fn calc_content_statistics_reports_the_array_and_factories() {
        let mut e = engine();
        e.register(GET_ALLOCATED_SIZE, |_, _| ret(0x77));
        let this = dispatcher(&mut e);
        inheritance_array(&mut e, this, &[(1, 2), (3, 4)], 4);
        e.mem.set_u32(this.addr() + 0xc + 8, 0x5000);
        let (object, factories, arrays) = collector(&mut e, true);
        e.call(0x00cfbed0, &args![this, object, 0u32]);
        let data = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceData);
        assert_eq!(
            arrays.borrow().clone(),
            vec![vec![object, 3, SHAPE_INHERITANCE_NAME, data, 16, 0x77]]
        );
        assert_eq!(calls_to(&e, GET_ALLOCATED_SIZE), vec![vec![data, 32]]);
        assert_eq!(factories.borrow().len(), 64);
        assert_eq!(
            factories.borrow()[2],
            vec![object, CONTACT_MGR_FACTORY_NAME, 0x5000, 0]
        );
    }

    #[test]
    fn calc_content_statistics_skips_a_borrowed_array() {
        let mut e = engine();
        e.register(GET_ALLOCATED_SIZE, |_, _| ret(0x77));
        let this = dispatcher(&mut e);
        e.set(
            this,
            HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags,
            0x8000_0000,
        );
        let (object, factories, arrays) = collector(&mut e, true);
        e.call(0x00cfbed0, &args![this, object, 0u32]);
        assert!(arrays.borrow().is_empty());
        assert_eq!(factories.borrow().len(), 64);
        let (object, _, arrays) = collector(&mut e, false);
        e.call(0x00cfbed0, &args![this, object, 0u32]);
        assert_eq!(arrays.borrow().len(), 1);
    }

    #[test]
    fn register_alternate_shape_type_appends_and_grows() {
        let mut e = engine();
        install_allocator(&mut e);
        e.register_double(ARRAY_GROW, |e, a| {
            let data = e.mem.alloc(64);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        let this = dispatcher(&mut e);
        e.set(
            this,
            HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags,
            0x8000_0000,
        );
        for index in 0..32 {
            e.mem
                .set_u32(this.addr() + HAS_ALTERNATE_TYPE + 4 * index, 1 << index);
        }
        e.call(0x00cfbf60, &args![this, 5u32, 2u32]);
        assert_eq!(inheritance_entries(&e, this), vec![(5, 2)]);
        let grows = calls_to(&e, ARRAY_GROW);
        assert_eq!(grows.len(), 1);
        assert_eq!(grows[0][1], this.addr() + 0x1e24);
        assert_eq!(grows[0][2], 8);
        assert_eq!(
            e.mem.u32(this.addr() + HAS_ALTERNATE_TYPE + 20),
            0b100000 | 0b100
        );
    }

    #[test]
    fn register_alternate_shape_type_removes_duplicates() {
        let mut e = engine();
        install_allocator(&mut e);
        e.register(ARRAY_GROW, |_, _| Ret::default());
        let this = dispatcher(&mut e);
        inheritance_array(&mut e, this, &[(1, 2), (5, 2), (1, 2), (7, 8)], 8);
        e.call(0x00cfbf60, &args![this, 1u32, 2u32]);
        assert_eq!(inheritance_entries(&e, this), vec![(5, 2), (7, 8), (1, 2)]);
        assert!(calls_to(&e, ARRAY_GROW).is_empty());
    }

    #[test]
    fn constructor_initialises_the_dispatcher() {
        let mut e = engine();
        e.register(REGISTER_DEFAULT_AGENT3, |_, _| Ret::default());
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        let this = dispatcher(&mut e);
        e.mem
            .set_u8(this.addr() + AGENT2_FUNC + 0x10 + AGENT2_FUNCS_SIZE, 7);
        e.mem.set_u8(this.addr() + AGENT3_FUNC + 0x32, 7);
        let returned = e.call(0x00cfc030, &args![this, 0x1234u32, 0x5000u32]).u32();
        assert_eq!(returned, this.addr());
        assert_eq!(
            e.get(this, HkpCollisionDispatcher::vtable),
            DISPATCHER_VTABLE
        );
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_referenceCount), 1);
        assert_eq!(
            e.get(this, HkpCollisionDispatcher::m_defaultCollisionAgent),
            0x1234
        );
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0x5000);
        assert_eq!(e.mem.u32(this.addr() + 0xc + 63 * 4), 0x5000);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 64);
        assert_eq!(e.mem.u32(this.addr() + HAS_ALTERNATE_TYPE), 1);
        assert_eq!(
            e.mem.u32(this.addr() + HAS_ALTERNATE_TYPE + 31 * 4),
            0x8000_0000
        );
        assert_eq!(byte(&e, this, AGENT2_FUNC + 0x10 + AGENT2_FUNCS_SIZE), 0);
        assert_eq!(byte(&e, this, AGENT3_FUNC + 0x32), 0);
        assert_eq!(
            e.get(
                this,
                HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags
            ),
            0x8000_0000
        );
        assert!(e.get(this, HkpCollisionDispatcher::m_checkEnabled));
        assert!(!e.get(this, HkpCollisionDispatcher::m_agent3Registered));
        assert!(!e.get(this, HkpCollisionDispatcher::m_collisionAgentRegistered));
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent3Types), 1);
        assert_eq!(e.get(this, HkpCollisionDispatcher::m_numAgent2Types), 1);
        assert_eq!(byte(&e, this, AGENT2_FUNC + 0x11), 1);
    }

    #[test]
    fn constructor_without_a_factory_takes_no_references() {
        let mut e = engine();
        e.register(REGISTER_DEFAULT_AGENT3, |_, _| Ret::default());
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        let this = dispatcher(&mut e);
        e.call(0x00cfc030, &args![this, 0u32, 0u32]);
        assert!(calls_to(&e, ADD_REFERENCE).is_empty());
    }

    #[test]
    fn destructor_releases_factories_and_the_array() {
        let mut e = engine();
        let (_, buffers) = install_allocator(&mut e);
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        let this = dispatcher(&mut e);
        e.mem.set_u32(this.addr() + 0xc, 0x5000);
        e.mem.set_u32(this.addr() + 0xc + 4 * 9, 0x6000);
        inheritance_array(&mut e, this, &[(1, 2)], 4);
        let data = e.get(this, HkpCollisionDispatcher::m_shapeInheritanceData);
        e.call(0x00cfc140, &args![this]);
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![0x5000], vec![0x6000]]
        );
        let freed = buffers.borrow().clone();
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1..], [data, 32]);
        assert_eq!(
            e.get(this, HkpCollisionDispatcher::m_shapeInheritanceSize),
            0
        );
        assert_eq!(
            e.get(this, HkpCollisionDispatcher::m_shapeInheritanceData),
            0
        );
        assert_eq!(
            e.get(
                this,
                HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags
            ),
            0x8000_0000
        );
        assert_eq!(
            e.get(this, HkpCollisionDispatcher::vtable),
            REFERENCED_OBJECT_VTABLE
        );
    }

    #[test]
    fn destructor_does_not_free_a_borrowed_array() {
        let mut e = engine();
        let (blocks, buffers) = install_allocator(&mut e);
        let this = dispatcher(&mut e);
        e.set(
            this,
            HkpCollisionDispatcher::m_shapeInheritanceCapacityAndFlags,
            0x8000_0004,
        );
        e.call(0x00cfc140, &args![this]);
        assert!(buffers.borrow().is_empty());
        assert!(blocks.borrow().is_empty());
    }

    #[test]
    fn scalar_deleting_destructor_frees_the_object_on_request() {
        let mut e = engine();
        let (blocks, _) = install_allocator(&mut e);
        let this = dispatcher(&mut e);
        e.set(this, HkpCollisionDispatcher::m_memSizeAndFlags, 0x1e50);
        let returned = e.call(0x00cfc550, &args![this, 1u32]).u32();
        assert_eq!(returned, this.addr());
        let freed = blocks.borrow().clone();
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1..], [this.addr(), 0x1e50]);
        assert_eq!(
            e.get(this, HkpCollisionDispatcher::vtable),
            REFERENCED_OBJECT_VTABLE
        );
        let (blocks, _) = install_allocator(&mut e);
        e.call(0x00cfc550, &args![this, 0u32]);
        assert!(blocks.borrow().is_empty());
    }
}
