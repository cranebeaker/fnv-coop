//! `fallout shared/havok/run_006319e0` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions of `ViewCaster.cpp` after `ViewCaster::GetWorld`: the
//! `ViewCaster` object (`NiPointer<bhkSimpleShapePhantom>` at `+0`, `bInit` at
//! `+4`, `iGroup` at `+8`, Xbox PDB) with its constructor, destructor, setup
//! and the pick (`fn_00631d60`: a linear cast through the phantom, then a
//! ray test per hit). It also holds the helpers placed beside them: a
//! 4x4 transform copy, the hit record copy, the blend-settings singleton.
//!
//! The compiler's exception-unwinding frames (`FS:[0]` registrations and the
//! `/GS` cookie check) are not modeled; the functions that have them carry a
//! `Translation incomplete` line.
//!
//! x87 note: a `float` loaded and stored again (`FLD`/`FSTP`) turns a
//! signalling NaN into a quiet one; `x87_quiet_bits` does that on the bit pattern.
//! Comparisons of `float`s follow the flag tests the code makes, NaN
//! included (every "greater/less" below is an ordered comparison).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ViewCaster` (Xbox PDB), 0xC bytes; the PC code uses the same offsets.
    pub struct ViewCaster: 0xc {
        /// `spShapePhantom` (Xbox PDB): `NiPointer<bhkSimpleShapePhantom>`.
        0x00 spShapePhantom: Ptr,
        /// `bInit` (Xbox PDB).
        0x04 bInit: bool,
        /// `iGroup` (Xbox PDB).
        0x08 iGroup: u32,
    }
}

/// `*this` (`MOV EAX,[ECX]`), the `NiPointer` getter.
const POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer` assignment (`0066b0d0`, takes the new pointer) and the plain
/// initializer that only stores it and adds a reference (`00633c90`).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_SET: u32 = 0x0063_3c90;
/// `NiPointer` destructor (`0045cec0`).
const POINTER_RELEASE: u32 = 0x0045_cec0;
/// Scope object around the work (`00404eb0` takes `(0x10, 1, file, line)`,
/// `00404ee0` ends it); the file is the string at `0104e588`, `ViewCaster.cpp`.
const SCOPE_BEGIN: u32 = 0x0040_4eb0;
const SCOPE_END: u32 = 0x0040_4ee0;
const VIEW_CASTER_FILE: u32 = 0x0104_e588;
/// The allocator: `operator new(size)`.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `bhkSphereShape::bhkSphereShape` (engine map): `(this, radius, scaled)`.
const SPHERE_SHAPE_CONSTRUCT: u32 = 0x0056_e950;
/// The sphere shape's pointer getter (engine map `bhkCharacterProxy::operatorP`,
/// a wrong match: the body returns `0x00620b80(this)` or 0 for a null `this`).
const SHAPE_WORD: u32 = 0x004a_e750;
/// Small collision-filter word object: constructor (`008c71b0`, takes the
/// first value), `4a39f0` (replaces the low 7 bits) and `59ce80` (replaces the
/// bits above 16).
const FILTER_CONSTRUCT: u32 = 0x008c_71b0;
const FILTER_SET_LAYER: u32 = 0x004a_39f0;
const FILTER_SET_GROUP: u32 = 0x0059_ce80;
/// Phantom cinfo (`bhkWorldObjectCinfo`-shaped): base constructor, the
/// constructor of the part at `+0x20`, destructor; and the phantom itself.
const CINFO_CONSTRUCT: u32 = 0x0056_e090;
const CINFO_CONSTRUCT_PART: u32 = 0x0056_e120;
const CINFO_DESTRUCT: u32 = 0x0056_ea90;
const PHANTOM_CONSTRUCT: u32 = 0x0056_e2d0;
/// `43b4f0(out)`: writes the value of a pointer's target into `out`; and
/// `626cb0(value)`, which hands it to the object.
const WRITE_FILTER_WORD: u32 = 0x0043_b4f0;
const APPLY_FILTER_WORD: u32 = 0x0062_6cb0;
/// The phantom's virtual function at `+0x9c` (one argument).
const PHANTOM_SLOT_9C: u32 = 0x9c;
/// `hkColor::rgbFromChars` (engine map name for `00cabb00`, cdecl, four
/// bytes) and `bhkWorld::SetDebugObjectColor` (engine map name for `00c68e30`).
const RGB_FROM_CHARS: u32 = 0x00ca_bb00;
const SET_DEBUG_OBJECT_COLOR: u32 = 0x00c6_8e30;

// Translated from 006319e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006319e0 - the FS:[0] exception-unwinding frame is not modeled.
/// The `ViewCaster` constructor (layout from the Xbox PDB; the function has
/// no map name): the phantom pointer is stored null twice (`00633c90` and
/// `0066b0d0` with 0), `bInit` and `iGroup` are cleared. Returns `this`.
pub fn fn_006319e0(e: &mut Engine, this: Ptr<ViewCaster>) -> Ptr<ViewCaster> {
    e.call(POINTER_SET, &args![this, 0u32]);
    e.call(POINTER_ASSIGN, &args![this, 0u32]);
    e.set(this, ViewCaster::bInit, false);
    e.set(this, ViewCaster::iGroup, 0);
    this
}

// Translated from 00631a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00631a50 - the FS:[0] exception-unwinding frame is not modeled.
/// The `ViewCaster` destructor: drops the phantom (`0066b0d0` with 0) and
/// destroys the `NiPointer` member.
pub fn fn_00631a50(e: &mut Engine, this: Ptr<ViewCaster>) {
    e.call(POINTER_ASSIGN, &args![this, 0u32]);
    e.call(POINTER_RELEASE, &args![this]);
}

// Translated from 00631ab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00631ab0 - the FS:[0] exception-unwinding frame is not modeled.
/// Sets up the caster: builds a sphere shape of `radius` (scaled, flag 1),
/// a phantom cinfo whose collision filter word has layer 0x28 and group
/// `group`, creates the `bhkSimpleShapePhantom` from it, stores it in
/// `spShapePhantom`, sets `bInit` and `iGroup`.
pub fn fn_00631ab0(e: &mut Engine, this: Ptr<ViewCaster>, radius: f32, group: u32) {
    // The game's stack frame: `EBP` is `frame + 0xa0`.
    e.with_stack(0xa0, |e, frame| {
        let ebp = frame.addr() + 0xa0;
        let scope = ebp - 0x18;
        let cinfo = ebp - 0x90;
        let filter = ebp - 0x94;
        e.call(
            SCOPE_BEGIN,
            &args![scope, 0x10u32, 1u32, VIEW_CASTER_FILE, 0x49u32],
        );
        let sphere_memory = e.call(ALLOCATE, &args![0x14u32]).u32();
        let sphere = if sphere_memory == 0 {
            0
        } else {
            let bits = x87_quiet_bits(radius.to_bits());
            e.call(SPHERE_SHAPE_CONSTRUCT, &args![sphere_memory, bits, 1u32])
                .u32()
        };
        e.call(CINFO_CONSTRUCT, &args![cinfo]);
        e.call(FILTER_CONSTRUCT, &args![filter, 0u32]);
        e.call(FILTER_SET_LAYER, &args![filter, 0x28u32]);
        e.call(FILTER_SET_GROUP, &args![filter, group]);
        e.set(this, ViewCaster::iGroup, group);
        let filter_word = e.call(POINTER_GET, &args![filter]).u32();
        e.mem.set_u32(cinfo, filter_word);
        let shape_word = e.call(SHAPE_WORD, &args![sphere]).u32();
        e.mem.set_u32(cinfo + 4, shape_word);
        e.call(CINFO_CONSTRUCT_PART, &args![cinfo + 0x20]);
        let phantom_memory = e.call(ALLOCATE, &args![0x18u32]).u32();
        let phantom = if phantom_memory == 0 {
            0
        } else {
            e.call(PHANTOM_CONSTRUCT, &args![phantom_memory, cinfo])
                .u32()
        };
        e.call(POINTER_ASSIGN, &args![this, phantom]);
        e.set(this, ViewCaster::bInit, true);
        e.call(CINFO_DESTRUCT, &args![cinfo]);
        e.call(SCOPE_END, &args![scope]);
    });
}

// Translated from 00631c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the caster's group: writes `group` into the filter word that
/// `0043b4f0` reads from the phantom, stores it in `iGroup`, and hands the
/// word back to the phantom (`00626cb0`, which ends with a virtual call at
/// `+0xc4`).
pub fn fn_00631c50(e: &mut Engine, this: Ptr<ViewCaster>, group: u32) {
    e.with_stack(4, |e, word| {
        let phantom = e.call(POINTER_GET, &args![this]).u32();
        e.call(WRITE_FILTER_WORD, &args![phantom, word]);
        e.call(FILTER_SET_GROUP, &args![word, group]);
        e.set(this, ViewCaster::iGroup, group);
        let phantom = e.call(POINTER_GET, &args![this]).u32();
        let filter_word = e.mem.u32(word.addr());
        e.call(APPLY_FILTER_WORD, &args![phantom, filter_word]);
    });
}

// Translated from 00631ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00631ca0 - the FS:[0] exception-unwinding frame is not modeled.
/// When the caster has a phantom: calls the phantom's virtual function at
/// `+0x9c` with `world`, and, when `world` is not null, gives `world` the
/// debug colour `rgbFromChars(0x58, 0xad, 0x56, 0x19)` for the phantom
/// (`bhkWorld::SetDebugObjectColor`, `world` being `this`, the phantom
/// the argument).
pub fn fn_00631ca0(e: &mut Engine, this: Ptr<ViewCaster>, world: u32) {
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_BEGIN,
            &args![scope, 0x10u32, 1u32, VIEW_CASTER_FILE, 0x74u32],
        );
        if e.call(POINTER_GET, &args![this]).u32() != 0 {
            let phantom = e.call(POINTER_GET, &args![this]).u32();
            e.vcall(phantom, PHANTOM_SLOT_9C, &args![world]);
            if world != 0 {
                let color = e
                    .call(RGB_FROM_CHARS, &args![0x58u32, 0xadu32, 0x56u32, 0x19u32])
                    .u32();
                let phantom = e.call(POINTER_GET, &args![this]).u32();
                e.call(SET_DEBUG_OBJECT_COLOR, &args![world, phantom, color]);
            }
        }
        e.call(SCOPE_END, &args![scope]);
    });
}

// ---------------------------------------------------------------------------
// fn_00631d60: the pick.

/// `ViewCaster::GetWorld` (engine map): the physics world of the phantom.
const GET_WORLD: u32 = 0x0062_1440;
/// `0x004f1540(this)`: nonzero when the caster may run.
const CAN_RUN: u32 = 0x004f_1540;
/// The cast collector's constructor (`00620ff0`), destructor (`006211b0`),
/// hits array getter (`00460140`, the array sits at `+0x10`), the array's
/// count (`00726070`) and element getter (`00807770`), the "has hits" test
/// (`00621210` writes a flag byte through `out` and returns it, `005609f0`
/// reads a byte), and the sort (`hkpAllCdPointCollector::sortHits`).
const COLLECTOR_CONSTRUCT: u32 = 0x0062_0ff0;
const COLLECTOR_DESTRUCT: u32 = 0x0062_11b0;
const HITS_ARRAY: u32 = 0x0046_0140;
const ARRAY_COUNT: u32 = 0x0072_6070;
const ARRAY_AT: u32 = 0x0080_7770;
const COLLECTOR_HAS_HITS: u32 = 0x0062_1210;
const BYTE_IS_SET: u32 = 0x0056_09f0;
const COLLECTOR_SORT_HITS: u32 = 0x00ca_bad0;
/// Two vector helpers: `45bb20(this, out, scale)` and `439e90(this, out,
/// vector)` fill the vector `out` from `this`; `4a3e00(dest, source)` copies
/// a vector into a 16-byte one (cdecl); `4a3f10` sets the cast input's end.
const VECTOR_SCALED: u32 = 0x0045_bb20;
const VECTOR_FROM: u32 = 0x0043_9e90;
const VECTOR4_ASSIGN: u32 = 0x004a_3e00;
const CAST_INPUT_SET_END: u32 = 0x004a_3f10;
/// 16-byte vector default constructor (`006815c0`, returns `this`).
const VECTOR4_CONSTRUCT: u32 = 0x0068_15c0;
/// Constructor of the cast input (`00621290`).
const CAST_INPUT_CONSTRUCT: u32 = 0x0062_1290;
/// Phantom method called with `(start, input, collector, 0)` (`00c9eba0`; the
/// engine map's name for it, `hkpAllCdPointCollector::hkpAllCdPointCollector`,
/// is a folded-code match; the call is a linear cast query, unconfirmed).
const PHANTOM_CAST: u32 = 0x00c9_eba0;
/// `NiPick` constructor and destructor (engine map), `NiPick` setters
/// `00458b30(flag)` and `fn_00632d20`, results (`00500940` gives the
/// results list; `00968670(index)` the entry; `0044edb0` the count).
const PICK_CONSTRUCT: u32 = 0x00e9_8f20;
const PICK_DESTRUCT: u32 = 0x00e9_8fa0;
const PICK_SET_FLAG: u32 = 0x0045_8b30;
const PICK_RESULTS: u32 = 0x0050_0940;
const RESULTS_COUNT: u32 = 0x0044_edb0;
const RESULTS_AT: u32 = 0x0096_8670;
const RESULT_OBJECT: u32 = 0x0045_8b50;
const ACTOR_KARMA: u32 = 0x0047_c860;
/// Settings object whose first byte is the "draw rays too" switch
/// (`00408d60(this)` returns a pointer to it).
const SWITCH_OBJECT: u32 = 0x011c_c628;
const SWITCH_ADDRESS: u32 = 0x0040_8d60;
/// Three counters' limits, read through `0043d4d0(this)` (returns a pointer
/// to an `int`): the group tests, the ground tests and the rays.
const LIMIT_OBJECT_GROUP: u32 = 0x011c_c61c;
const LIMIT_OBJECT_REFERENCE: u32 = 0x011c_c644;
const LIMIT_OBJECT_RAY: u32 = 0x011c_c668;
const LIMIT_ADDRESS: u32 = 0x0043_d4d0;
/// `GetAVObjectForCollidable` (engine map), `TESObjectREFR::FindReferenceFor3D`
/// (engine map): cdecl, one argument each.
const AV_OBJECT_FOR_COLLIDABLE: u32 = 0x004b_5820;
const REFERENCE_FOR_AV_OBJECT: u32 = 0x0056_f930;
/// The reference the player's own tests skip (`011dea3c`) and the form the
/// `TESObjectREFR` getter `007af430` is compared with (`011ca240`).
const SKIPPED_REFERENCE: u32 = 0x011d_ea3c;
const COMPARED_FORM: u32 = 0x011c_a240;
const OBJECT_FORM: u32 = 0x007a_f430;
/// Walks of the scene graph: `00456610(node)` test and `009611e0(node)` parent.
const NODE_TEST: u32 = 0x0045_6610;
const NODE_PARENT: u32 = 0x0096_11e0;
/// The group id of a collidable (`004b59f0`, cdecl) and the "ground" value of
/// a group (`004b5a80`, cdecl).
const COLLIDABLE_GROUP: u32 = 0x004b_59f0;
const GROUP_VALUE: u32 = 0x004b_5a80;
/// Type code of a form-like object (`0043b540` makes the wrapper's value,
/// `008c71b0` stores it, `0043b4d0` returns the code, `0043b230` is the
/// target's code).
const TYPE_VALUE: u32 = 0x0043_b540;
const TYPE_CODE: u32 = 0x0043_b4d0;
const TARGET_CODE: u32 = 0x0043_b230;
const FORM_TYPE: u32 = 0x0040_1170;
const FORM_TEST: u32 = 0x0046_f070;
/// The mover and process getters used for the actor case.
const ACTOR_SLOT_100: u32 = 0x100;
const ACTOR_SLOT_1D0: u32 = 0x1d0;
const ACTOR_SLOT_4BC: u32 = 0x4bc;
const SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
const CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// Transform and ray-test helpers.
const BODY_TRANSFORM: u32 = 0x004b_4f00;
const VECTOR_SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
const RAY_RESULT_CONSTRUCT: u32 = 0x004a_3d20;
const RAY_RESULT_PREPARE: u32 = 0x0062_a2d0;
const VECTOR_SUBTRACT: u32 = 0x0045_8a10;
const VECTOR_NORMALIZE: u32 = 0x0062_aea0;
const VECTOR_PROJECT: u32 = 0x0062_b0a0;
const VECTOR_ASSIGN_FROM: u32 = 0x0062_7920;
const VECTOR_LENGTH_SOURCE: u32 = 0x0063_0b40;
const VECTOR_LENGTH_SQUARED: u32 = 0x0045_86d0;
const SQUARE_ROOT: u32 = 0x0045_87d0;
/// The ray cast input and its collector.
const RAY_INPUT_CONSTRUCT: u32 = 0x004a_3c20;
const RAY_INPUT_SET_FILTER: u32 = 0x004a_3f70;
const RAY_INPUT_SET_START: u32 = 0x0056_d340;
const RAY_INPUT_SET_END: u32 = 0x004a_3f40;
const RAY_INPUT_SET_COLLECTOR: u32 = 0x004a_3fb0;
const RAY_COLLECTOR_CONSTRUCT: u32 = 0x004a_3a70;
const RAY_COLLECTOR_DESTRUCT: u32 = 0x004a_3bc0;
const RAY_HIT_AT: u32 = 0x004a_46b0;
const WORLD_CAST_RAY_SLOT: u32 = 0xc8;
/// `004b2780(pick, start, direction, range, name, 0)` (cdecl, six words).
const PICK_ADD: u32 = 0x004b_2780;
/// Globals: the group of the last "ground" hit, and a step number the code
/// stores as it goes (`0xdad` to `0xdba`; its use is not confirmed).
const LAST_GROUND_VALUE: u32 = 0x011c_c5ec;
const STEP_NUMBER: u32 = 0x011d_8a90;
/// `float` constants: the largest `float` (`01016970`) and `0.01` (`01013ea4`).
const FLOAT_MAX: u32 = 0x0101_6970;
const CAST_RADIUS: u32 = 0x0101_3ea4;

/// Fields of the hit record `fn_00632d40` copies (offsets from its start):
/// the collidable pointer is at `+0x48`.
const HIT_COLLIDABLE: u32 = 0x48;

// Translated from 00631d60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00631d60 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// The pick: casts the phantom from `start` along `direction * range`, then
/// walks the hits nearest first. Returns the reference found (0 for none),
/// writes its distance through `out_distance` (the largest `float` when none)
/// and 1 or 0 through `out_flag` (1 when the result is the "ground" hit
/// found by the ray tests). Nothing runs when `004f1540` refuses, the
/// phantom pointer is null, or the world is null (the result is then 0 and
/// the outputs are untouched).
///
/// Roles of the locals are described by what the code does; the names of
/// the many helpers are not in the Xbox PDB and are given by address.
pub fn fn_00631d60(
    e: &mut Engine,
    this: Ptr<ViewCaster>,
    start: u32,
    direction: u32,
    range: f32,
    out_distance: u32,
    out_flag: u32,
) -> u32 {
    e.with_stack(0xb00, |e, frame| {
        pick(
            e,
            frame.addr() + 0xb00,
            this.addr(),
            start,
            direction,
            range,
            out_distance,
            out_flag,
        )
    })
}

/// The body of `fn_00631d60`; `ebp` is the game's frame pointer, the objects
/// sit at their `EBP`-relative offsets.
#[allow(clippy::too_many_arguments)]
// The virtual call stays an `if` in the arm: a match guard would hide its side effect.
#[allow(clippy::collapsible_match)]
fn pick(
    e: &mut Engine,
    ebp: u32,
    this: u32,
    start: u32,
    direction: u32,
    range: f32,
    out_distance: u32,
    out_flag: u32,
) -> u32 {
    let at = |down: u32| ebp - down;
    let scope = at(0x1c);
    let collector = at(0x3c0);
    let end_point = at(0x3d0);
    let far_point = at(0x3dc);
    let picker = at(0x410);
    let origin4 = at(0x420);
    let target4 = at(0x430);
    let cast_input = at(0x460);
    let hit = at(0x510);
    let ray_start = at(0x570);
    let ray_end = at(0x560);
    let transform = at(0x5c0);
    let ray_result = at(0x610);
    let ray_hit_flag = at(0x616);
    let shape_flag = at(0x615);
    let delta = at(0x640);
    let direction4 = at(0x650);
    let projected = at(0x660);
    let offset = at(0x670);
    let ray_input = at(0x750);
    let ray_collector = at(0xa80);

    e.call(
        SCOPE_BEGIN,
        &args![scope, 0x10u32, 1u32, VIEW_CASTER_FILE, 0x9bu32],
    );
    e.set_global(LAST_GROUND_VALUE, 0u32);
    let mut result = 0u32;
    let runs = e.call(CAN_RUN, &args![this]).bool()
        && e.call(POINTER_GET, &args![this]).u32() != 0
        && e.call(GET_WORLD, &args![this]).u32() != 0;
    if runs {
        e.call(COLLECTOR_CONSTRUCT, &args![collector]);
        let world = e.call(GET_WORLD, &args![this]).u32();
        let scaled = e
            .call(
                VECTOR_SCALED,
                &args![direction, end_point, x87_quiet_bits(range.to_bits())],
            )
            .u32();
        e.call(VECTOR_FROM, &args![start, far_point, scaled]);
        e.mem.set_u8(out_flag, 0);
        let max = x87_quiet_bits(e.mem.u32(FLOAT_MAX));
        e.mem.set_u32(out_distance, max);
        e.call(PICK_CONSTRUCT, &args![picker, 0u32, 8u32]);
        fn_00632d20(e, Ptr::new(picker), 0);
        e.call(PICK_SET_FLAG, &args![picker, 1u32]);
        e.set_global(STEP_NUMBER, 0xdadu32);
        e.call(VECTOR4_CONSTRUCT, &args![origin4]);
        e.call(VECTOR4_ASSIGN, &args![origin4, start]);
        e.call(VECTOR4_CONSTRUCT, &args![target4]);
        e.call(VECTOR4_ASSIGN, &args![target4, far_point]);
        e.mem.set_u8(at(0x431), 0);
        e.call(CAST_INPUT_CONSTRUCT, &args![cast_input]);
        let radius = x87_quiet_bits(e.mem.u32(CAST_RADIUS));
        e.mem.set_u32(at(0x44c), radius);
        let radius = x87_quiet_bits(e.mem.u32(CAST_RADIUS));
        e.mem.set_u32(at(0x450), radius);
        e.call(CAST_INPUT_SET_END, &args![cast_input, target4]);
        fn_00632df0(e, Ptr::new(this));
        let phantom = e.call(POINTER_GET, &args![this]).u32();
        e.call(
            PHANTOM_CAST,
            &args![phantom, origin4, cast_input, collector, 0u32],
        );
        let has_hits = e
            .call(COLLECTOR_HAS_HITS, &args![collector, at(0x461)])
            .u32();
        if e.call(BYTE_IS_SET, &args![has_hits]).bool() {
            e.call(COLLECTOR_SORT_HITS, &args![collector]);
            e.mem.set_u8(at(0x431), 1);
        }

        // Best hit so far (reference and distance), the nearest "ground"
        // candidate (reference, its distances and group value) and the
        // counters of the three limits.
        let mut best_reference = 0u32;
        let mut best_distance = x87_quiet(f32::from_bits(e.mem.u32(FLOAT_MAX)));
        let mut ground_reference = 0u32;
        let mut ground_distance = x87_quiet(f32::from_bits(e.mem.u32(FLOAT_MAX)));
        let mut ground_nearest = x87_quiet(f32::from_bits(e.mem.u32(FLOAT_MAX)));
        let mut ground_value = 0u32;
        let switch = e.call(SWITCH_ADDRESS, &args![SWITCH_OBJECT]).u32();
        let rays_enabled = e.mem.u8(switch);
        let mut ray_count = 0i32;
        let mut group_count = 0i32;
        let mut reference_count = 0i32;

        if e.mem.u8(at(0x431)) != 0 {
            e.set_global(STEP_NUMBER, 0xdaeu32);
            let mut last_reference = 0u32;
            let mut current_reference = 0u32;
            let mut last_group = 0u32;
            let mut index = 0i32;
            loop {
                let hits = e.call(HITS_ARRAY, &args![collector]).u32();
                let total = e.call(ARRAY_COUNT, &args![hits]).i32();
                if index >= total {
                    break;
                }
                e.set_global(STEP_NUMBER, 0xdafu32);
                let hits = e.call(HITS_ARRAY, &args![collector]).u32();
                let entry = e.call(ARRAY_AT, &args![hits, index as u32]).u32();
                fn_00632d40(e, Ptr::new(hit), Ptr::new(entry));
                'body: {
                    let collidable = e.mem.u32(hit + HIT_COLLIDABLE);
                    if collidable == 0 {
                        break 'body;
                    }
                    e.set_global(STEP_NUMBER, 0xdb0u32);
                    let av_object = e.call(AV_OBJECT_FOR_COLLIDABLE, &args![collidable]).u32();
                    e.set_global(STEP_NUMBER, 0xdb1u32);
                    if av_object != 0 {
                        current_reference =
                            e.call(REFERENCE_FOR_AV_OBJECT, &args![av_object]).u32();
                    }
                    if current_reference == 0 || current_reference == e.mem.u32(SKIPPED_REFERENCE) {
                        break 'body;
                    }
                    // Walk from the object up to the reference's own root.
                    let mut found = 0u8;
                    let mut node = av_object;
                    let root = e.vcall(current_reference, ACTOR_SLOT_1D0, &args![]).u32();
                    while found == 0 && node != 0 && node != root {
                        let tested = e.call(NODE_TEST, &args![node]).u8();
                        found |= tested;
                        node = e.call(NODE_PARENT, &args![node]).u32();
                    }
                    if found != 0 {
                        break 'body;
                    }
                    let group = e.call(COLLIDABLE_GROUP, &args![collidable]).u32();
                    let mut interesting = false;
                    let type_value = e.call(TYPE_VALUE, &args![collidable]).u32();
                    e.call(FILTER_CONSTRUCT, &args![at(0x530), type_value]);
                    let type_code = e.call(TYPE_CODE, &args![at(0x530)]).i32();
                    if type_code == 1 {
                        let target = e.call(POINTER_GET, &args![collidable]).u32();
                        if target != 0 && e.call(TARGET_CODE, &args![target]).i32() == 10 {
                            interesting = true;
                        }
                    } else if type_code > 3 && type_code <= 6 {
                        interesting = true;
                    }
                    if e.call(OBJECT_FORM, &args![current_reference]).u32()
                        == e.mem.u32(COMPARED_FORM)
                    {
                        interesting = true;
                    }

                    if group != 0 && group != last_group && interesting {
                        last_group = group;
                        let limit = e.call(LIMIT_ADDRESS, &args![LIMIT_OBJECT_GROUP]).u32();
                        if group_count < e.mem.i32(limit) {
                            group_count += 1;
                            fn_00632ca0(e, Ptr::new(ray_start));
                            let holder = e.call(POINTER_GET, &args![this]).u32();
                            let word = e.call(WRITE_FILTER_WORD, &args![holder, at(0x574)]).u32();
                            let filter_word = e.call(POINTER_GET, &args![word]).u32();
                            e.mem.set_u32(at(0x550), filter_word);
                            let body_transform = e.call(BODY_TRANSFORM, &args![group]).u32();
                            fn_00632b40(e, Ptr::new(transform), Ptr::new(body_transform));
                            e.call(
                                VECTOR_SET_TRANSFORMED_INVERSE_POS,
                                &args![ray_start, transform, origin4],
                            );
                            e.call(
                                VECTOR_SET_TRANSFORMED_INVERSE_POS,
                                &args![ray_end, transform, target4],
                            );
                            e.call(RAY_RESULT_CONSTRUCT, &args![ray_result]);
                            e.call(RAY_RESULT_PREPARE, &args![ray_result]);
                            let shape = e.call(POINTER_GET, &args![collidable]).u32();
                            fn_00632c30(
                                e,
                                Ptr::new(shape),
                                Ptr::new(shape_flag),
                                Ptr::new(ray_start),
                                Ptr::new(ray_result),
                            );
                            let flag = fn_00632c60(e, Ptr::new(ray_result), Ptr::new(ray_hit_flag));
                            if e.call(BYTE_IS_SET, &args![flag]).bool() {
                                let hit_distance = e.mem.f32(ray_result + 0x10);
                                if best_distance > hit_distance {
                                    best_reference = current_reference;
                                    best_distance = x87_quiet(hit_distance);
                                }
                            }
                        }
                    } else {
                        e.set_global(STEP_NUMBER, 0xdb2u32);
                        if last_reference != current_reference
                            && e.vcall(current_reference, ACTOR_SLOT_1D0, &args![]).u32() != 0
                        {
                            let limit = e.call(LIMIT_ADDRESS, &args![LIMIT_OBJECT_REFERENCE]).u32();
                            if reference_count < e.mem.i32(limit) {
                                reference_count += 1;
                                let name =
                                    e.vcall(current_reference, ACTOR_SLOT_1D0, &args![]).u32();
                                e.call(
                                    PICK_ADD,
                                    &args![
                                        picker,
                                        start,
                                        direction,
                                        x87_quiet_bits(range.to_bits()),
                                        name,
                                        0u32
                                    ],
                                );
                            }
                        }
                    }

                    // Which kinds of reference never count as ground.
                    let mut counts = true;
                    let kind = e.call(TYPE_CODE, &args![at(0x530)]).i32().wrapping_sub(1) as u32;
                    if kind <= 0x1d {
                        match kind {
                            0 | 12 | 16 | 17 => counts = false,
                            5 => {
                                if group == 0 {
                                    counts = false;
                                }
                            }
                            29 => {
                                if e.vcall(current_reference, ACTOR_SLOT_100, &args![]).bool() {
                                    let process = e
                                        .call(SAVED_ACQUIRE_OBJECT, &args![current_reference])
                                        .u32();
                                    let controller =
                                        e.call(CHAR_CONTROLLER, &args![current_reference]).u32();
                                    if process != 0
                                        && controller != 0
                                        && fn_00632d00(e, Ptr::new(controller)) == 1
                                    {
                                        let state =
                                            e.vcall(process, ACTOR_SLOT_4BC, &args![]).u32();
                                        if state == 9 || state == 4 {
                                            counts = false;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }

                    if counts && rays_enabled != 0 && world != 0 {
                        // Vector from the start to the hit, and the length
                        // measured along the direction.
                        e.call(VECTOR4_CONSTRUCT, &args![delta]);
                        let hit_point = e.call(VECTOR4_CONSTRUCT, &args![hit]).u32();
                        e.call(VECTOR_SUBTRACT, &args![delta, hit_point, origin4]);
                        e.call(VECTOR4_CONSTRUCT, &args![direction4]);
                        e.call(VECTOR_SUBTRACT, &args![direction4, target4, origin4]);
                        e.call(VECTOR_NORMALIZE, &args![direction4]);
                        let along = e
                            .call(VECTOR_PROJECT, &args![direction4, projected, delta])
                            .u32();
                        e.call(VECTOR_ASSIGN_FROM, &args![direction4, along]);
                        e.call(VECTOR4_CONSTRUCT, &args![offset]);
                        e.call(VECTOR_SUBTRACT, &args![offset, delta, direction4]);
                        let length_source = e
                            .call(VECTOR_LENGTH_SOURCE, &args![offset, at(0x680)])
                            .u32();
                        let squared = e.call(VECTOR_LENGTH_SQUARED, &args![length_source]).f32();
                        let distance = e.call(SQUARE_ROOT, &args![squared.to_bits()]).f32();

                        let may_cast = ground_nearest > distance && {
                            let limit = e.call(LIMIT_ADDRESS, &args![LIMIT_OBJECT_RAY]).u32();
                            ray_count < e.mem.i32(limit)
                        };
                        if may_cast {
                            if ground_reference != 0 && ground_reference == current_reference {
                                let source =
                                    e.call(VECTOR_LENGTH_SOURCE, &args![delta, at(0x6a0)]).u32();
                                let squared = e.call(VECTOR_LENGTH_SQUARED, &args![source]).f32();
                                ground_distance =
                                    e.call(SQUARE_ROOT, &args![squared.to_bits()]).f32();
                                ground_nearest = x87_quiet(distance);
                            } else {
                                ray_count += 1;
                                e.call(RAY_INPUT_CONSTRUCT, &args![ray_input]);
                                let holder = e.call(POINTER_GET, &args![this]).u32();
                                let word_pointer =
                                    e.call(WRITE_FILTER_WORD, &args![holder, at(0x754)]).u32();
                                let word = e.mem.u32(word_pointer);
                                e.call(RAY_INPUT_SET_FILTER, &args![ray_input, word]);
                                e.call(RAY_INPUT_SET_START, &args![ray_input, origin4]);
                                let hit_point = e.call(VECTOR4_CONSTRUCT, &args![hit]).u32();
                                e.call(RAY_INPUT_SET_END, &args![ray_input, hit_point]);
                                e.call(RAY_COLLECTOR_CONSTRUCT, &args![ray_collector]);
                                e.call(RAY_INPUT_SET_COLLECTOR, &args![ray_input, ray_collector]);
                                e.vcall(world, WORLD_CAST_RAY_SLOT, &args![ray_input]);
                                if fn_00632ce0(e, Ptr::new(ray_input)) == 0 {
                                    let hits = e.call(HITS_ARRAY, &args![ray_collector]).u32();
                                    let ray_hits = e.call(ARRAY_COUNT, &args![hits]).i32();
                                    if ray_hits < 1 {
                                        ground_reference = current_reference;
                                        let source = e
                                            .call(VECTOR_LENGTH_SOURCE, &args![delta, at(0xab0)])
                                            .u32();
                                        let squared =
                                            e.call(VECTOR_LENGTH_SQUARED, &args![source]).f32();
                                        ground_distance =
                                            e.call(SQUARE_ROOT, &args![squared.to_bits()]).f32();
                                        ground_nearest = x87_quiet(distance);
                                        ground_value = e.call(GROUP_VALUE, &args![group]).u32();
                                    } else {
                                        let hits = e.call(HITS_ARRAY, &args![ray_collector]).u32();
                                        let first = e.call(RAY_HIT_AT, &args![hits, 0u32]).u32();
                                        let ray_collidable = e.mem.u32(first + 0x50);
                                        let ray_object = e
                                            .call(AV_OBJECT_FOR_COLLIDABLE, &args![ray_collidable])
                                            .u32();
                                        let mut ray_reference = 0;
                                        if ray_object != 0 {
                                            ray_reference = e
                                                .call(REFERENCE_FOR_AV_OBJECT, &args![ray_object])
                                                .u32();
                                        }
                                        if ray_object != 0
                                            && ray_reference != 0
                                            && ray_reference == current_reference
                                        {
                                            ground_reference = current_reference;
                                            let source = e
                                                .call(
                                                    VECTOR_LENGTH_SOURCE,
                                                    &args![delta, at(0xaa0)],
                                                )
                                                .u32();
                                            let squared =
                                                e.call(VECTOR_LENGTH_SQUARED, &args![source]).f32();
                                            ground_distance = e
                                                .call(SQUARE_ROOT, &args![squared.to_bits()])
                                                .f32();
                                            ground_nearest = x87_quiet(distance);
                                            ground_value = e.call(GROUP_VALUE, &args![group]).u32();
                                        }
                                    }
                                }
                                e.call(RAY_COLLECTOR_DESTRUCT, &args![ray_collector]);
                            }
                        }
                    }
                    e.set_global(STEP_NUMBER, 0xdb3u32);
                    last_reference = current_reference;
                }
                index += 1;
            }
        }

        // The nearest hit's distance is scaled by the range.
        best_distance = (f64::from(best_distance) * f64::from(range)) as f32;
        e.set_global(STEP_NUMBER, 0xdb5u32);
        let results = e.call(PICK_RESULTS, &args![picker]).u32();
        if e.call(RESULTS_COUNT, &args![results]).u32() > 0 {
            e.set_global(STEP_NUMBER, 0xdb6u32);
            let results = e.call(PICK_RESULTS, &args![picker]).u32();
            let entry = e.call(RESULTS_AT, &args![results, 0u32]).u32();
            let picked = e.call(RESULT_OBJECT, &args![entry]).u32();
            e.set_global(STEP_NUMBER, 0xdb7u32);
            result = e.call(REFERENCE_FOR_AV_OBJECT, &args![picked]).u32();
            e.set_global(STEP_NUMBER, 0xdb8u32);
            let results = e.call(PICK_RESULTS, &args![picker]).u32();
            let entry = e.call(RESULTS_AT, &args![results, 0u32]).u32();
            let karma = e.call(ACTOR_KARMA, &args![entry]).f32();
            e.mem.set_f32(out_distance, karma);
            e.set_global(STEP_NUMBER, 0xdb9u32);
            if best_distance < e.mem.f32(out_distance) {
                e.mem
                    .set_u32(out_distance, x87_quiet_bits(best_distance.to_bits()));
                result = best_reference;
            }
        } else if best_reference != 0 {
            e.mem
                .set_u32(out_distance, x87_quiet_bits(best_distance.to_bits()));
            result = best_reference;
        }

        e.set_global(STEP_NUMBER, 0xdbau32);
        if rays_enabled != 0 {
            let mut allowed = true;
            if result != 0 {
                let form = e.call(OBJECT_FORM, &args![result]).u32();
                if form != 0 {
                    allowed = false;
                    let form = e.call(OBJECT_FORM, &args![result]).u32();
                    let kind = e.call(FORM_TYPE, &args![form]).u32().wrapping_sub(0x1e);
                    if kind <= 7 {
                        match kind {
                            0 => {
                                let form = e.call(OBJECT_FORM, &args![result]).u32();
                                if !e.call(FORM_TEST, &args![form]).bool() {
                                    allowed = true;
                                }
                            }
                            2 | 3 | 7 => allowed = true,
                            _ => {}
                        }
                    }
                }
            }
            if allowed && ground_reference != 0 && result != ground_reference {
                e.mem
                    .set_u32(out_distance, x87_quiet_bits(ground_distance.to_bits()));
                result = ground_reference;
                e.mem.set_u8(out_flag, 1);
                e.set_global(LAST_GROUND_VALUE, ground_value);
            }
        }
        e.call(PICK_DESTRUCT, &args![picker]);
        e.call(COLLECTOR_DESTRUCT, &args![collector]);
    }
    e.call(SCOPE_END, &args![scope]);
    result
}

// Translated from 00632b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00632b40 - the /GS cookie check is not modeled.
/// Builds a 4x4 transform in `this` from `source`: after the two base
/// constructors (`0056df00(this)` and `006815c0(this + 0x30)`), the four
/// rows of `source` (`004b4cf0(i)`) are copied into temporaries
/// (`004a3c90`) and then stored into the four rows of `this`
/// (`004a3f10`). Returns `this`.
pub fn fn_00632b40(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.with_stack(0x50, |e, frame| {
        let ebp = frame.addr() + 0x50;
        e.call(0x0056_df00, &args![this]);
        e.call(0x0068_15c0, &args![this.byte_add(0x30)]);
        for row in 0..4u32 {
            let source_row = e.call(0x004b_4cf0, &args![source, row]).u32();
            e.call(0x004a_3c90, &args![ebp - 0x20 - 0x10 * row, source_row]);
        }
        for row in 0..4u32 {
            let own_row = e.call(0x004b_4cf0, &args![this, row]).u32();
            e.call(0x004a_3f10, &args![own_row, ebp - 0x20 - 0x10 * row]);
        }
    });
    this
}

// Translated from 00632c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the virtual function at `+0x20` of `this` with the three words and
/// returns the first one.
pub fn fn_00632c30(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, third: Ptr) -> Ptr {
    e.vcall(this.addr(), 0x20, &args![first, second, third]);
    first
}

// Translated from 00632c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores whether the `float` at `this + 0x10` is below 1.0 (the `double` at
/// `01012070`; false for a NaN) as the flag `00622570` writes into `out`.
/// Returns `out`.
pub fn fn_00632c60(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let value = e.mem.f32(this.addr() + 0x10);
    let one: f64 = e.global(0x0101_2070);
    let below = f64::from(value) < one;
    e.call(0x0062_2570, &args![out, below]);
    out
}

// Translated from 00632ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the two 16-byte vectors at `+0` and `+0x10` and clears the
/// words at `+0x20` and `+0x24`. Returns `this`. (The engine map's library
/// name, `_String_val<>`, is a wrong match.)
pub fn fn_00632ca0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0068_15c0, &args![this]);
    e.call(0x0068_15c0, &args![this.byte_add(0x10)]);
    e.mem.set_u32(this.addr() + 0x20, 0);
    e.mem.set_u32(this.addr() + 0x24, 0);
    this
}

// Translated from 00632ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0xac`.
pub fn fn_00632ce0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xac)
}

// Translated from 00632d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x600`.
pub fn fn_00632d00(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x600)
}

// Translated from 00632d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at `+0x10`.
pub fn fn_00632d20(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x10, value);
}

// Translated from 00632d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 0x70-byte hit record `other` into `this`: the two vectors
/// through `fn_00632dc0`, then the words at `+0x20`, `+0x24`, the eight at
/// `+0x28`, `+0x48`, `+0x4c` and the eight at `+0x50`. Returns `this`.
pub fn fn_00632d40(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    fn_00632dc0(e, this, other);
    for offset in (0x20..0x70).step_by(4) {
        let word = e.mem.u32(other.addr() + offset);
        e.mem.set_u32(this.addr() + offset, word);
    }
    this
}

// Translated from 00632dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the two 16-byte vectors of `other` (at `+0` and `+0x10`) to those
/// of `this` with `004a3c90`. Returns `this`.
pub fn fn_00632dc0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(0x004a_3c90, &args![this, other]);
    e.call(
        0x004a_3c90,
        &args![this.byte_add(0x10), other.byte_add(0x10)],
    );
    this
}

// Translated from 00632df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00c82870(0x28, 0x700eef7e, 0)` (the engine map's name for it is
/// `bhkCollisionFilter::SetMultiLayerCollision`) and returns its result.
/// `this` is not used.
pub fn fn_00632df0(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(0x00c8_2870, &args![0x28u32, 0x700e_ef7eu32, 0u32])
        .u32()
}

// Translated from 00632e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The blend-settings collection singleton: creates it when needed
/// (`fn_00632e30`) and returns the global.
pub fn fn_00632e20(e: &mut Engine) -> u32 {
    fn_00632e30(e);
    e.global(0x011c_c694)
}

// Translated from 00632e30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00632e30 - the FS:[0] exception-unwinding frame is not modeled.
/// Creates the `BlendSettingCollection` (RTTI of its vtable `0104e6ac` names
/// the class) when the global at `011cc694` is null: allocates 0x114 bytes
/// (`00401000`), constructs it there with `fn_00632ec0` (null when the
/// allocation failed) and stores the result in the global.
pub fn fn_00632e30(e: &mut Engine) {
    if e.global::<u32>(0x011c_c694) != 0 {
        return;
    }
    let memory = e.call(0x0040_1000, &args![0x114u32]).u32();
    let collection = if memory == 0 {
        0
    } else {
        fn_00632ec0(e, Ptr::new(memory)).addr()
    };
    e.set_global(0x011c_c694, collection);
}

// Translated from 00632ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BlendSettingCollection` constructor (class from the vtable's RTTI
/// name): the base constructor (`0044f600`) and the vtable `0104e6ac`.
/// Returns `this`.
pub fn fn_00632ec0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0044_f600, &args![this]);
    e.mem.set_u32(this.addr(), 0x0104_e6ac);
    this
}

// Translated from 00632ee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00632ee0 - the FS:[0] exception-unwinding frame is not modeled.
/// Constructor of a `SettingT<BlendSettingCollection>` (the RTTI name of its
/// vtable `0104e6e8`): the base constructor (`0040e130(name, value)`), the
/// vtable, and the registration with the collection's virtual function at
/// `+4`. Returns `this`.
pub fn fn_00632ee0(e: &mut Engine, this: Ptr, name: u32, value: f32) -> Ptr {
    e.call(
        0x0040_e130,
        &args![this, name, x87_quiet_bits(value.to_bits())],
    );
    e.mem.set_u32(this.addr(), 0x0104_e6e8);
    let collection = fn_00632e20(e);
    e.vcall(collection, 4, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006319e0, fn_006319e0(Ptr<ViewCaster>) -> Ptr<ViewCaster>),
        entry!(0x00631a50, fn_00631a50(Ptr<ViewCaster>)),
        entry!(0x00631ab0, fn_00631ab0(Ptr<ViewCaster>, f32, u32)),
        entry!(0x00631c50, fn_00631c50(Ptr<ViewCaster>, u32)),
        entry!(0x00631ca0, fn_00631ca0(Ptr<ViewCaster>, u32)),
        entry!(
            0x00631d60,
            fn_00631d60(Ptr<ViewCaster>, u32, u32, f32, u32, u32) -> u32
        ),
        entry!(0x00632b40, fn_00632b40(Ptr, Ptr) -> Ptr),
        entry!(0x00632c30, fn_00632c30(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00632c60, fn_00632c60(Ptr, Ptr) -> Ptr),
        entry!(0x00632ca0, fn_00632ca0(Ptr) -> Ptr),
        entry!(0x00632ce0, fn_00632ce0(Ptr) -> u8),
        entry!(0x00632d00, fn_00632d00(Ptr) -> u32),
        entry!(0x00632d20, fn_00632d20(Ptr, u8)),
        entry!(0x00632d40, fn_00632d40(Ptr, Ptr) -> Ptr),
        entry!(0x00632dc0, fn_00632dc0(Ptr, Ptr) -> Ptr),
        entry!(0x00632df0, fn_00632df0(Ptr) -> u32),
        entry!(0x00632e20, fn_00632e20() -> u32),
        entry!(0x00632e30, fn_00632e30()),
        entry!(0x00632ec0, fn_00632ec0(Ptr) -> Ptr),
        entry!(0x00632ee0, fn_00632ee0(Ptr, u32, f32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn float_ret(value: f32) -> Ret {
        Ret {
            st0: f64::from(value),
            ..Ret::default()
        }
    }

    /// An engine with the pages the unit's globals live on and a call log.
    fn caster_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_3000,
            0x0101_6000,
            0x011c_a000,
            0x011c_c000,
            0x011d_8000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(0x0101_2070, 1.0f64);
        e.call_log = Some(vec![]);
        e
    }

    fn logged(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(addr, _)| *addr)
            .collect()
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Registers a no-op (returning 0) for each address.
    fn stub(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| ret(0));
        }
    }

    // -- The ViewCaster object ------------------------------------------

    fn caster(e: &mut Engine) -> Ptr<ViewCaster> {
        let this: Ptr<ViewCaster> = e.new_object();
        e.set(this, ViewCaster::spShapePhantom, Ptr::new(0xaaaa));
        e.set(this, ViewCaster::bInit, true);
        e.set(this, ViewCaster::iGroup, 7);
        this
    }

    #[test]
    fn constructor_clears_the_caster() {
        let mut e = caster_engine();
        stub(&mut e, &[POINTER_SET, POINTER_ASSIGN]);
        let this = caster(&mut e);
        let result = e.call(0x0063_19e0, &args![this]);
        assert_eq!(result.u32(), this.addr());
        assert!(!e.get(this, ViewCaster::bInit));
        assert_eq!(e.get(this, ViewCaster::iGroup), 0);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [
                (POINTER_SET, vec![this.addr(), 0]),
                (POINTER_ASSIGN, vec![this.addr(), 0])
            ]
        );
    }

    #[test]
    fn destructor_drops_the_phantom_then_the_pointer() {
        let mut e = caster_engine();
        stub(&mut e, &[POINTER_RELEASE, POINTER_ASSIGN]);
        e.call(0x0063_1a50, &args![0x4000u32]);
        assert_eq!(
            e.call_log.as_ref().unwrap(),
            &vec![
                (0x0063_1a50, vec![0x4000]),
                (POINTER_ASSIGN, vec![0x4000, 0]),
                (POINTER_RELEASE, vec![0x4000]),
            ]
        );
    }

    fn setup_doubles(e: &mut Engine) {
        stub(
            e,
            &[
                SCOPE_BEGIN,
                SCOPE_END,
                CINFO_CONSTRUCT,
                CINFO_CONSTRUCT_PART,
                CINFO_DESTRUCT,
            ],
        );
        e.register(SPHERE_SHAPE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(PHANTOM_CONSTRUCT, |_, a| ret(a[0]));
        e.register(FILTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(FILTER_SET_LAYER, |e, a| {
            let word = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], (word & !0x7f) | (a[1] & 0x7f));
            ret(0)
        });
        e.register(FILTER_SET_GROUP, |e, a| {
            let word = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], (word & 0xffff) | (a[1] << 16));
            ret(0)
        });
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(SHAPE_WORD, |_, a| ret(a[0] + 1));
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
    }

    #[test]
    fn setup_builds_the_sphere_phantom_and_marks_the_caster() {
        let mut e = caster_engine();
        setup_doubles(&mut e);
        let this: Ptr<ViewCaster> = e.new_object();
        e.call(0x0063_1ab0, &args![this, 2.5f32, 5u32]);
        assert!(e.get(this, ViewCaster::bInit));
        assert_eq!(e.get(this, ViewCaster::iGroup), 5);
        let sphere = calls_to(&e, SPHERE_SHAPE_CONSTRUCT)[0].clone();
        assert_eq!(sphere[1..], [2.5f32.to_bits(), 1]);
        let cinfo = calls_to(&e, CINFO_CONSTRUCT)[0][0];
        // Filter word: group above 16 bits, layer 0x28 in the low 7 bits.
        assert_eq!(e.mem.u32(cinfo), (5 << 16) | 0x28);
        assert_eq!(e.mem.u32(cinfo + 4), sphere[0] + 1);
        assert_eq!(calls_to(&e, CINFO_CONSTRUCT_PART), vec![vec![cinfo + 0x20]]);
        let phantom = calls_to(&e, PHANTOM_CONSTRUCT)[0].clone();
        assert_eq!(phantom[1], cinfo);
        assert_eq!(e.get(this, ViewCaster::spShapePhantom).addr(), phantom[0]);
        let scope = calls_to(&e, SCOPE_BEGIN)[0].clone();
        assert_eq!(scope[1..], [0x10, 1, VIEW_CASTER_FILE, 0x49]);
        assert_eq!(calls_to(&e, SCOPE_END), vec![vec![scope[0]]]);
        let order = logged(&e);
        let position = |addr: u32| order.iter().position(|a| *a == addr).unwrap();
        assert!(position(CINFO_CONSTRUCT) < position(PHANTOM_CONSTRUCT));
        assert!(position(PHANTOM_CONSTRUCT) < position(CINFO_DESTRUCT));
    }

    #[test]
    fn setup_keeps_going_when_an_allocation_fails() {
        let mut e = caster_engine();
        setup_doubles(&mut e);
        e.register(ALLOCATE, |_, _| ret(0));
        let this: Ptr<ViewCaster> = e.new_object();
        e.call(0x0063_1ab0, &args![this, 1.0f32, 3u32]);
        assert!(e.get(this, ViewCaster::bInit));
        assert!(calls_to(&e, SPHERE_SHAPE_CONSTRUCT).is_empty());
        assert!(calls_to(&e, PHANTOM_CONSTRUCT).is_empty());
        assert_eq!(e.get(this, ViewCaster::spShapePhantom).addr(), 0);
        let cinfo = calls_to(&e, CINFO_CONSTRUCT)[0][0];
        assert_eq!(
            e.mem.u32(cinfo + 4),
            1,
            "the shape word of a null shape is 0, plus one"
        );
    }

    #[test]
    fn set_group_writes_the_filter_word_and_hands_it_to_the_phantom() {
        let mut e = caster_engine();
        setup_doubles(&mut e);
        e.register(WRITE_FILTER_WORD, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            ret(a[1])
        });
        e.register(APPLY_FILTER_WORD, |_, _| ret(0));
        let this = caster(&mut e);
        e.call(0x0063_1c50, &args![this, 5u32]);
        assert_eq!(e.get(this, ViewCaster::iGroup), 5);
        let apply = calls_to(&e, APPLY_FILTER_WORD)[0].clone();
        assert_eq!(apply[1..], [(5 << 16) | 0x1234]);
        assert_eq!(
            calls_to(&e, WRITE_FILTER_WORD)[0][0],
            e.mem.u32(this.addr())
        );
    }

    fn debug_color_engine() -> (Engine, Ptr<ViewCaster>) {
        let mut e = caster_engine();
        stub(&mut e, &[SCOPE_BEGIN, SCOPE_END, SET_DEBUG_OBJECT_COLOR]);
        e.register(RGB_FROM_CHARS, |_, _| ret(0xc0102030));
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x7000, |_, _| ret(0));
        e.put_vtable(0x6000, &[0; 0x30]);
        e.mem.set_u32(0x6000 + PHANTOM_SLOT_9C, 0x7000);
        e.map(0x5000, 0x1000);
        e.mem.set_u32(0x5100, 0x6000);
        let this: Ptr<ViewCaster> = e.new_object();
        e.set(this, ViewCaster::spShapePhantom, Ptr::new(0x5100));
        (e, this)
    }

    #[test]
    fn debug_color_does_nothing_without_a_phantom() {
        let mut e = caster_engine();
        stub(&mut e, &[SCOPE_BEGIN, SCOPE_END]);
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        let this: Ptr<ViewCaster> = e.new_object();
        e.call(0x0063_1ca0, &args![this, 0x9999u32]);
        assert_eq!(
            logged(&e),
            vec![0x0063_1ca0, SCOPE_BEGIN, POINTER_GET, SCOPE_END]
        );
    }

    #[test]
    fn debug_color_with_a_null_world_only_calls_the_phantom() {
        let (mut e, this) = debug_color_engine();
        e.call(0x0063_1ca0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, 0x7000), vec![vec![0x5100, 0]]);
        assert!(calls_to(&e, RGB_FROM_CHARS).is_empty());
        assert!(calls_to(&e, SET_DEBUG_OBJECT_COLOR).is_empty());
    }

    #[test]
    fn debug_color_with_a_world_sets_the_color_on_the_phantom() {
        let (mut e, this) = debug_color_engine();
        e.call(0x0063_1ca0, &args![this, 0x8888u32]);
        assert_eq!(calls_to(&e, 0x7000), vec![vec![0x5100, 0x8888]]);
        assert_eq!(
            calls_to(&e, RGB_FROM_CHARS),
            vec![vec![0x58, 0xad, 0x56, 0x19]]
        );
        assert_eq!(
            calls_to(&e, SET_DEBUG_OBJECT_COLOR),
            vec![vec![0x8888, 0x5100, 0xc010_2030]]
        );
    }

    // -- The helpers ----------------------------------------------------

    #[test]
    fn transform_copy_goes_through_four_temporaries() {
        let mut e = caster_engine();
        stub(
            &mut e,
            &[0x0056_df00, 0x0068_15c0, 0x004a_3c90, 0x004a_3f10],
        );
        // Row `i` of an object is its address plus `0x100 * i`.
        e.register(0x004b_4cf0, |_, a| ret(a[0] + 0x100 * a[1]));
        let result = e.call(0x0063_2b40, &args![0x2000u32, 0x3000u32]);
        assert_eq!(result.u32(), 0x2000);
        let log = e.call_log.as_ref().unwrap();
        let addrs: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addrs,
            vec![
                0x0063_2b40,
                0x0056_df00,
                0x0068_15c0,
                0x004b_4cf0,
                0x004a_3c90,
                0x004b_4cf0,
                0x004a_3c90,
                0x004b_4cf0,
                0x004a_3c90,
                0x004b_4cf0,
                0x004a_3c90,
                0x004b_4cf0,
                0x004a_3f10,
                0x004b_4cf0,
                0x004a_3f10,
                0x004b_4cf0,
                0x004a_3f10,
                0x004b_4cf0,
                0x004a_3f10,
            ]
        );
        assert_eq!(log[1].1, vec![0x2000]);
        assert_eq!(log[2].1, vec![0x2030]);
        // Source rows 0..3 are read first, each copied to a temporary 0x10
        // apart; the same temporaries then feed the rows of `this`.
        let temporaries: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x004a_3c90)
            .map(|(_, args)| args[0])
            .collect();
        let sources: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x004a_3c90)
            .map(|(_, args)| args[1])
            .collect();
        assert_eq!(sources, vec![0x3000, 0x3100, 0x3200, 0x3300]);
        assert_eq!(temporaries[0] - temporaries[1], 0x10);
        assert_eq!(temporaries[1] - temporaries[2], 0x10);
        assert_eq!(temporaries[2] - temporaries[3], 0x10);
        let stored: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == 0x004a_3f10)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(
            stored,
            vec![
                vec![0x2000, temporaries[0]],
                vec![0x2100, temporaries[1]],
                vec![0x2200, temporaries[2]],
                vec![0x2300, temporaries[3]],
            ]
        );
    }

    #[test]
    fn virtual_forwarder_calls_slot_0x20_and_returns_the_first_word() {
        let mut e = caster_engine();
        e.put_vtable(0x6000, &[0; 0x10]);
        e.mem.set_u32(0x6000 + 0x20, 0x7000);
        e.register(0x7000, |_, _| ret(99));
        e.map(0x5000, 0x1000);
        e.mem.set_u32(0x5100, 0x6000);
        let result = e.call(0x0063_2c30, &args![0x5100u32, 11u32, 22u32, 33u32]);
        assert_eq!(result.u32(), 11);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (0x7000, vec![0x5100, 11, 22, 33])
        );
    }

    #[test]
    fn below_one_flag_follows_the_ordered_comparison() {
        for (value, expected) in [
            (0.5f32, 1u32),
            (1.0, 0),
            (2.0, 0),
            (f32::NAN, 0),
            (f32::NEG_INFINITY, 1),
        ] {
            let mut e = caster_engine();
            e.register(0x0062_2570, |_, a| ret(a[1]));
            let this = e.mem.alloc(0x20);
            e.mem.set_f32(this + 0x10, value);
            let result = e.call(0x0063_2c60, &args![this, 0x4444u32]);
            assert_eq!(result.u32(), 0x4444);
            assert_eq!(
                e.call_log.as_ref().unwrap()[1],
                (0x0062_2570, vec![0x4444, expected]),
                "value {value}"
            );
        }
    }

    #[test]
    fn two_vector_object_is_constructed_and_cleared() {
        let mut e = caster_engine();
        stub(&mut e, &[0x0068_15c0]);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x20, 0xdead);
        e.mem.set_u32(this + 0x24, 0xbeef);
        let result = e.call(0x0063_2ca0, &args![this]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [(0x0068_15c0, vec![this]), (0x0068_15c0, vec![this + 0x10])]
        );
    }

    #[test]
    fn small_getters_and_setter() {
        let mut e = caster_engine();
        let object = e.mem.alloc(0x700);
        e.mem.set_u8(object + 0xac, 0x7f);
        e.mem.set_u32(object + 0x600, 0x1234_5678);
        assert_eq!(e.call(0x0063_2ce0, &args![object]).u8(), 0x7f);
        assert_eq!(e.call(0x0063_2d00, &args![object]).u32(), 0x1234_5678);
        e.call(0x0063_2d20, &args![object, 9u32]);
        assert_eq!(e.mem.u8(object + 0x10), 9);
    }

    #[test]
    fn hit_record_copy_assigns_the_vectors_and_copies_the_words() {
        let mut e = caster_engine();
        stub(&mut e, &[0x004a_3c90]);
        let source = e.mem.alloc(0x70);
        let target = e.mem.alloc(0x70);
        for offset in (0x20..0x70).step_by(4) {
            e.mem.set_u32(source + offset, 0x1000 + offset);
        }
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(target + offset, 0xabcd);
        }
        let result = e.call(0x0063_2d40, &args![target, source]);
        assert_eq!(result.u32(), target);
        for offset in (0x20..0x70).step_by(4) {
            assert_eq!(e.mem.u32(target + offset), 0x1000 + offset);
        }
        // The first 0x20 bytes are the two vectors, assigned by the helper.
        assert_eq!(e.mem.u32(target), 0xabcd);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [
                (0x004a_3c90, vec![target, source]),
                (0x004a_3c90, vec![target + 0x10, source + 0x10])
            ]
        );
    }

    #[test]
    fn vector_pair_assignment_returns_this() {
        let mut e = caster_engine();
        stub(&mut e, &[0x004a_3c90]);
        let result = e.call(0x0063_2dc0, &args![0x100u32, 0x200u32]);
        assert_eq!(result.u32(), 0x100);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1..],
            [
                (0x004a_3c90, vec![0x100, 0x200]),
                (0x004a_3c90, vec![0x110, 0x210])
            ]
        );
    }

    #[test]
    fn collision_filter_setup_passes_three_constants() {
        let mut e = caster_engine();
        e.register(0x00c8_2870, |_, _| ret(0x5a5a));
        let result = e.call(0x0063_2df0, &args![0u32]);
        assert_eq!(result.u32(), 0x5a5a);
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (0x00c8_2870, vec![0x28, 0x700e_ef7e, 0])
        );
    }

    #[test]
    fn blend_settings_singleton_is_created_once() {
        let mut e = caster_engine();
        e.register(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0044_f600, |_, _| ret(0));
        let first = e.call(0x0063_2e20, &args![]).u32();
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(first), 0x0104_e6ac);
        assert_eq!(e.global::<u32>(0x011c_c694), first);
        let second = e.call(0x0063_2e20, &args![]).u32();
        assert_eq!(second, first);
        assert_eq!(
            calls_to(&e, 0x0040_1000),
            vec![vec![0x114]],
            "allocated once"
        );
    }

    #[test]
    fn blend_settings_singleton_stays_null_when_the_allocation_fails() {
        let mut e = caster_engine();
        e.register(0x0040_1000, |_, _| ret(0));
        e.call(0x0063_2e30, &args![]);
        assert_eq!(e.global::<u32>(0x011c_c694), 0);
        assert!(calls_to(&e, 0x0044_f600).is_empty());
    }

    #[test]
    fn blend_settings_collection_constructor_sets_its_vtable() {
        let mut e = caster_engine();
        e.register(0x0044_f600, |_, _| ret(0));
        let this = e.mem.alloc(0x114);
        let result = e.call(0x0063_2ec0, &args![this]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0104_e6ac);
        assert_eq!(e.call_log.as_ref().unwrap()[1], (0x0044_f600, vec![this]));
    }

    #[test]
    fn setting_constructor_registers_itself_with_the_collection() {
        let mut e = caster_engine();
        e.register(0x0040_e130, |_, _| ret(0));
        e.register(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register(0x0044_f600, |_, _| ret(0));
        // The collection exists already; its virtual function at +4 records the call.
        let collection = e.mem.alloc(0x114);
        e.put_vtable(0x6000, &[0; 4]);
        e.mem.set_u32(0x6004, 0x7000);
        e.register(0x7000, |_, _| ret(0));
        e.mem.set_u32(collection, 0x6000);
        e.set_global(0x011c_c694, collection);
        let this = e.mem.alloc(0x40);
        let result = e.call(0x0063_2ee0, &args![this, 0x1111u32, 2.5f32]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0104_e6e8);
        assert_eq!(
            calls_to(&e, 0x0040_e130),
            vec![vec![this, 0x1111, 2.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x7000), vec![vec![collection, this]]);
    }

    #[test]
    fn signalling_nans_are_quieted_by_the_x87_copy() {
        assert_eq!(x87_quiet_bits(0x7f80_0001), 0x7fc0_0001);
        assert_eq!(x87_quiet_bits(0xff80_0001), 0xffc0_0001);
        assert_eq!(x87_quiet_bits(0x7fc0_0000), 0x7fc0_0000);
        assert_eq!(x87_quiet_bits(0x7f80_0000), 0x7f80_0000);
        assert_eq!(x87_quiet_bits(1.5f32.to_bits()), 1.5f32.to_bits());
    }

    // -- The pick -------------------------------------------------------

    /// Test inputs the doubles read from memory.
    const TEST_HIT_COUNT: u32 = 0x011c_cf00;
    const TEST_HIT_RECORD: u32 = 0x011c_cf04;
    const TEST_SWITCH: u32 = 0x011c_cf10;
    const TEST_GROUP: u32 = 0x011c_cf14;
    const TEST_TYPE_CODE: u32 = 0x011c_cf18;
    const TEST_RAY_DISTANCE: u32 = 0x011c_cf1c;
    const TEST_RESULT_COUNT: u32 = 0x011c_cf20;
    const TEST_KARMA: u32 = 0x011c_cf24;
    const TEST_FORM: u32 = 0x011c_cf28;
    const TEST_FORM_TYPE: u32 = 0x011c_cf2c;
    const TEST_ROOT: u32 = 0x011c_cf30;
    const TEST_SQUARED: u32 = 0x011c_cf34;
    const TEST_STATE: u32 = 0x011c_cf38;
    const TEST_FORM_TEST: u32 = 0x011c_cf3c;
    /// The caster, its phantom, the world, the reference and the shape.
    const CASTER_ADDRESS: u32 = 0x5100;
    const WORLD_ADDRESS: u32 = 0x6100;
    const REFERENCE_ADDRESS: u32 = 0x8200;
    const SHAPE_ADDRESS: u32 = 0x7300;
    /// The reference the picker result resolves to.
    const OTHER_REFERENCE: u32 = 0x8300;

    /// Every callee of the pick, with the behaviour the tests need.
    fn pick_engine() -> Engine {
        let mut e = caster_engine();
        stub(
            &mut e,
            &[
                SCOPE_BEGIN,
                SCOPE_END,
                VECTOR_SCALED,
                VECTOR_FROM,
                VECTOR4_ASSIGN,
                CAST_INPUT_SET_END,
                VECTOR4_CONSTRUCT,
                CAST_INPUT_CONSTRUCT,
                PHANTOM_CAST,
                PICK_CONSTRUCT,
                PICK_DESTRUCT,
                PICK_SET_FLAG,
                COLLECTOR_DESTRUCT,
                COLLECTOR_SORT_HITS,
                PICK_ADD,
                VECTOR_SET_TRANSFORMED_INVERSE_POS,
                RAY_RESULT_CONSTRUCT,
                RAY_RESULT_PREPARE,
                VECTOR_SUBTRACT,
                VECTOR_NORMALIZE,
                VECTOR_ASSIGN_FROM,
                VECTOR_PROJECT,
                VECTOR_LENGTH_SOURCE,
                RAY_INPUT_SET_FILTER,
                RAY_INPUT_SET_START,
                RAY_INPUT_SET_END,
                RAY_INPUT_SET_COLLECTOR,
                RAY_COLLECTOR_DESTRUCT,
                RAY_HIT_AT,
                NODE_TEST,
                NODE_PARENT,
                SAVED_ACQUIRE_OBJECT,
                CHAR_CONTROLLER,
                TARGET_CODE,
                0x0056_df00,
                0x004b_4cf0,
                0x0068_15c0,
                0x004a_3c90,
                0x004a_3f10,
                0x00c8_2870,
                0x0062_2570,
            ],
        );
        e.register(CAN_RUN, |_, _| ret(1));
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(GET_WORLD, |_, _| ret(WORLD_ADDRESS));
        e.register(COLLECTOR_CONSTRUCT, |e, a| {
            let count = e.mem.u32(TEST_HIT_COUNT);
            let record = e.mem.u32(TEST_HIT_RECORD);
            e.mem.set_u32(a[0] + 0x10, count);
            e.mem.set_u32(a[0] + 0x14, record);
            ret(a[0])
        });
        e.register(RAY_INPUT_CONSTRUCT, |e, a| {
            e.mem.set_u8(a[0] + 0xac, 0);
            ret(0)
        });
        e.register(RAY_COLLECTOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x10, 0);
            ret(a[0])
        });
        e.register(HITS_ARRAY, |_, a| ret(a[0] + 0x10));
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(ARRAY_AT, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 0x70));
        e.register(COLLECTOR_HAS_HITS, |e, a| {
            let count = e.mem.u32(TEST_HIT_COUNT);
            e.mem.set_u8(a[1], u8::from(count > 0));
            ret(a[1])
        });
        e.register(BYTE_IS_SET, |e, a| ret(u32::from(e.mem.u8(a[0]) != 0)));
        e.register(0x0062_2570, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            ret(a[0])
        });
        e.register(SWITCH_ADDRESS, |_, _| ret(TEST_SWITCH));
        e.register(LIMIT_ADDRESS, |_, a| ret(a[0]));
        e.register(AV_OBJECT_FOR_COLLIDABLE, |_, a| ret(a[0] + 0x100));
        e.register(REFERENCE_FOR_AV_OBJECT, |_, a| {
            ret(if a[0] == 0x7210 {
                OTHER_REFERENCE
            } else {
                REFERENCE_ADDRESS
            })
        });
        e.register(COLLIDABLE_GROUP, |e, _| ret(e.mem.u32(TEST_GROUP)));
        e.register(GROUP_VALUE, |_, a| ret(a[0] + 0x70));
        e.register(BODY_TRANSFORM, |_, _| ret(0x3000));
        e.register(TYPE_VALUE, |_, _| ret(0));
        e.register(FILTER_CONSTRUCT, |_, a| ret(a[0]));
        e.register(TYPE_CODE, |e, _| ret(e.mem.u32(TEST_TYPE_CODE)));
        e.register(OBJECT_FORM, |e, _| ret(e.mem.u32(TEST_FORM)));
        e.register(FORM_TYPE, |e, _| ret(e.mem.u32(TEST_FORM_TYPE)));
        e.register(FORM_TEST, |e, _| ret(e.mem.u32(TEST_FORM_TEST)));
        e.register(WRITE_FILTER_WORD, |e, a| {
            e.mem.set_u32(a[1], 0x0007_0001);
            ret(a[1])
        });
        e.register(RAY_INPUT_SET_FILTER, |_, _| ret(0));
        e.register(PICK_RESULTS, |_, _| ret(0x5000));
        e.register(RESULTS_COUNT, |e, _| ret(e.mem.u32(TEST_RESULT_COUNT)));
        e.register(RESULTS_AT, |_, _| ret(0x5200));
        e.register(RESULT_OBJECT, |_, _| ret(0x7210));
        e.register(ACTOR_KARMA, |e, _| float_ret(e.mem.f32(TEST_KARMA)));
        e.register(VECTOR_LENGTH_SQUARED, |e, _| {
            float_ret(e.mem.f32(TEST_SQUARED))
        });
        e.register(SQUARE_ROOT, |_, a| float_ret(f32::from_bits(a[0]).sqrt()));
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(CAST_RADIUS, 0.01f32);
        e.set_global(SKIPPED_REFERENCE, 0x1234_5678u32);
        e.set_global(COMPARED_FORM, 0x7777_7777u32);
        // Limits (group, reference, ray): three each.
        for limit in [LIMIT_OBJECT_GROUP, LIMIT_OBJECT_REFERENCE, LIMIT_OBJECT_RAY] {
            e.set_global(limit, 3i32);
        }
        e.set_global(TEST_GROUP, 5u32);
        e.set_global(TEST_TYPE_CODE, 5u32);
        e.set_global(TEST_RAY_DISTANCE, 0.25f32);
        e.set_global(TEST_SQUARED, 9.0f32);
        e.set_global(TEST_KARMA, 100.0f32);
        e.set_global(TEST_FORM_TYPE, 0x20u32);
        // The caster, its phantom, the world, the reference and the shape.
        for page in [0x5000, 0x6000, 0x7000, 0x8000] {
            e.map(page, 0x1000);
        }
        e.mem.set_u32(CASTER_ADDRESS, 0x6800); // phantom pointer
        e.put_vtable(0x6200, &[0; 0x40]);
        e.mem.set_u32(WORLD_ADDRESS, 0x6200);
        e.register(0x6300, |_, _| ret(0));
        e.mem.set_u32(0x6200 + WORLD_CAST_RAY_SLOT, 0x6300);
        // The reference: slot 0x1d0 gives the root, 0x100 the "is actor" flag.
        e.put_vtable(0x8800, &[0; 0x140]);
        e.mem.set_u32(REFERENCE_ADDRESS, 0x8800);
        e.register(0x8900, |e, _| ret(e.mem.u32(TEST_ROOT)));
        e.mem.set_u32(0x8800 + ACTOR_SLOT_1D0, 0x8900);
        e.register(0x8904, |_, _| ret(0));
        e.mem.set_u32(0x8800 + ACTOR_SLOT_100, 0x8904);
        // The shape the ray is cast against; its slot 0x20 stores a distance.
        e.put_vtable(0x7a00, &[0; 0x10]);
        e.mem.set_u32(SHAPE_ADDRESS, 0x7a00);
        e.register(0x7b00, |e, a| {
            let distance = e.mem.f32(TEST_RAY_DISTANCE);
            e.mem.set_f32(a[3] + 0x10, distance);
            ret(0)
        });
        e.mem.set_u32(0x7a00 + 0x20, 0x7b00);
        // One hit record whose collidable (+0x48) holds the shape.
        e.mem.set_u32(0x7100, SHAPE_ADDRESS);
        e.mem.set_u32(TEST_HIT_RECORD, 0x7400);
        e.mem.set_u32(0x7400 + 0x48, 0x7100);
        e.set_global(TEST_HIT_COUNT, 1u32);
        e.set_global(TEST_ROOT, 0u32);
        e
    }

    fn run_pick(e: &mut Engine) -> (u32, f32, u8) {
        let out = e.mem.alloc(8);
        e.mem.set_f32(out, 7.0);
        e.mem.set_u8(out + 4, 9);
        let result = e.call(
            0x0063_1d60,
            &args![CASTER_ADDRESS, 0x4000u32, 0x4100u32, 8.0f32, out, out + 4],
        );
        (result.u32(), e.mem.f32(out), e.mem.u8(out + 4))
    }

    #[test]
    fn pick_refuses_to_run_without_permission_a_phantom_or_a_world() {
        let mut e = pick_engine();
        e.register(CAN_RUN, |_, _| ret(0));
        e.set_global(LAST_GROUND_VALUE, 9u32);
        let (result, distance, flag) = run_pick(&mut e);
        assert_eq!((result, distance, flag), (0, 7.0, 9));
        assert_eq!(e.global::<u32>(LAST_GROUND_VALUE), 0);
        assert_eq!(
            logged(&e),
            vec![0x0063_1d60, SCOPE_BEGIN, CAN_RUN, SCOPE_END]
        );
        let scope = calls_to(&e, SCOPE_BEGIN)[0].clone();
        assert_eq!(scope[1..], [0x10, 1, VIEW_CASTER_FILE, 0x9b]);

        let mut e = pick_engine();
        e.mem.set_u32(CASTER_ADDRESS, 0);
        let (result, distance, _) = run_pick(&mut e);
        assert_eq!((result, distance), (0, 7.0));
        assert!(
            calls_to(&e, GET_WORLD).is_empty(),
            "the phantom check comes first"
        );

        let mut e = pick_engine();
        e.register(GET_WORLD, |_, _| ret(0));
        let (result, distance, _) = run_pick(&mut e);
        assert_eq!((result, distance), (0, 7.0));
        assert!(calls_to(&e, COLLECTOR_CONSTRUCT).is_empty());
    }

    #[test]
    fn pick_without_hits_reports_nothing_and_cleans_up() {
        let mut e = pick_engine();
        e.set_global(TEST_HIT_COUNT, 0u32);
        let (result, distance, flag) = run_pick(&mut e);
        assert_eq!(result, 0);
        assert_eq!(distance, f32::MAX);
        assert_eq!(flag, 0);
        assert!(calls_to(&e, COLLECTOR_SORT_HITS).is_empty());
        let order = logged(&e);
        let position = |addr: u32| order.iter().position(|a| *a == addr).unwrap();
        assert!(position(PICK_DESTRUCT) < position(COLLECTOR_DESTRUCT));
        assert_eq!(*order.last().unwrap(), SCOPE_END);
        // The cast: the phantom gets (start, input, collector, 0).
        let cast = calls_to(&e, PHANTOM_CAST)[0].clone();
        assert_eq!(cast.len(), 5);
        assert_eq!(cast[0], 0x6800);
        assert_eq!(cast[4], 0);
        // The cast input's radius words (+0x10 and +0x14 of the input) hold 0.01.
        let input = cast[2];
        assert_eq!(e.mem.f32(input + 0x14), 0.01);
        assert_eq!(e.mem.f32(input + 0x10), 0.01);
        // The scaled-vector helper is called with this = the direction.
        assert_eq!(calls_to(&e, VECTOR_SCALED)[0][0], 0x4100);
        assert_eq!(calls_to(&e, VECTOR_SCALED)[0][2], 8.0f32.to_bits());
        assert_eq!(calls_to(&e, VECTOR_FROM)[0][0], 0x4000);
    }

    #[test]
    fn pick_takes_the_nearest_group_hit_scaled_by_the_range() {
        let mut e = pick_engine();
        let (result, distance, flag) = run_pick(&mut e);
        // Hit distance 0.25 times the range 8.
        assert_eq!(result, REFERENCE_ADDRESS);
        assert_eq!(distance, 2.0);
        assert_eq!(flag, 0);
        assert_eq!(calls_to(&e, COLLECTOR_SORT_HITS).len(), 1);
        assert_eq!(calls_to(&e, BODY_TRANSFORM), vec![vec![5]]);
        assert!(calls_to(&e, RAY_INPUT_CONSTRUCT).is_empty(), "rays are off");
    }

    #[test]
    fn pick_ignores_a_group_hit_that_is_not_nearer_or_not_a_hit() {
        // A distance above 1.0 is not "below one", so the flag stays clear.
        let mut e = pick_engine();
        e.set_global(TEST_RAY_DISTANCE, 1.5f32);
        let (result, distance, _) = run_pick(&mut e);
        assert_eq!((result, distance), (0, f32::MAX));

        // A NaN distance never beats the initial best.
        let mut e = pick_engine();
        e.set_global(TEST_RAY_DISTANCE, f32::NAN);
        let (result, distance, _) = run_pick(&mut e);
        assert_eq!((result, distance), (0, f32::MAX));
    }

    #[test]
    fn pick_skips_the_hits_it_must_not_count() {
        // The skipped reference.
        let mut e = pick_engine();
        e.set_global(SKIPPED_REFERENCE, REFERENCE_ADDRESS);
        let (result, _, _) = run_pick(&mut e);
        assert_eq!(result, 0);
        assert!(calls_to(&e, COLLIDABLE_GROUP).is_empty());

        // A hit without a collidable.
        let mut e = pick_engine();
        e.mem.set_u32(0x7400 + 0x48, 0);
        let (result, _, _) = run_pick(&mut e);
        assert_eq!(result, 0);
        assert!(calls_to(&e, AV_OBJECT_FOR_COLLIDABLE).is_empty());

        // The scene-graph walk finds a node that stops the hit.
        let mut e = pick_engine();
        e.register(NODE_TEST, |_, _| ret(1));
        let (result, _, _) = run_pick(&mut e);
        assert_eq!(result, 0);
        assert!(calls_to(&e, COLLIDABLE_GROUP).is_empty());
        // The walk went up one node (the test hit at once, then the parent).
        assert_eq!(calls_to(&e, NODE_PARENT).len(), 1);
    }

    #[test]
    fn pick_kinds_that_never_count_as_ground() {
        // With rays on and no group, a ray is cast (the projection helper
        // runs) unless the reference type makes it never count.
        for (code, casts) in [
            (1u32, false),
            (6, false), // group 0
            (13, false),
            (17, false),
            (18, false),
            (2, true),
            (5, true),
            (31, true), // above the switch range
        ] {
            let mut e = pick_engine();
            e.mem.set_u8(TEST_SWITCH, 1);
            e.set_global(TEST_TYPE_CODE, code);
            e.set_global(TEST_GROUP, 0u32);
            run_pick(&mut e);
            assert_eq!(
                !calls_to(&e, VECTOR_PROJECT).is_empty(),
                casts,
                "type code {code}"
            );
        }
    }

    #[test]
    fn pick_prefers_the_picker_result_only_when_the_hit_is_not_nearer() {
        // The picker has one result (another reference) whose value 100.0 is
        // above the group hit's 2.0: the group hit wins.
        let mut e = pick_engine();
        e.set_global(TEST_RESULT_COUNT, 1u32);
        let (result, distance, _) = run_pick(&mut e);
        assert_eq!(result, REFERENCE_ADDRESS);
        assert_eq!(distance, 2.0);

        // A value of 1.0 is below 2.0: the picker reference wins.
        let mut e = pick_engine();
        e.set_global(TEST_RESULT_COUNT, 1u32);
        e.set_global(TEST_KARMA, 1.0f32);
        let (result, distance, _) = run_pick(&mut e);
        assert_eq!(result, OTHER_REFERENCE);
        assert_eq!(distance, 1.0);
        assert_eq!(calls_to(&e, RESULTS_AT).len(), 2);

        // A value that is not a number is never above the best: the
        // comparison is ordered, so the picker reference stays.
        let mut e = pick_engine();
        e.set_global(TEST_RESULT_COUNT, 1u32);
        e.set_global(TEST_KARMA, f32::NAN);
        let (result, distance, _) = run_pick(&mut e);
        assert_eq!(result, OTHER_REFERENCE);
        assert!(distance.is_nan());
    }

    fn ground_engine() -> Engine {
        let mut e = pick_engine();
        e.mem.set_u8(TEST_SWITCH, 1);
        e.set_global(TEST_GROUP, 0u32);
        e.set_global(TEST_TYPE_CODE, 2u32);
        e
    }

    /// The address of the double standing for the world cast function.
    const WORLD_CAST_RAY_DOUBLE: u32 = 0x6300;

    #[test]
    fn pick_casts_a_ray_and_reports_the_ground_hit() {
        let mut e = ground_engine();
        let (result, distance, flag) = run_pick(&mut e);
        // sqrt(9.0) = 3.0 is the ground distance; there is no better hit.
        assert_eq!(result, REFERENCE_ADDRESS);
        assert_eq!(distance, 3.0);
        assert_eq!(flag, 1);
        // The group value of the hit (the double adds 0x70 to the group, 0).
        assert_eq!(e.global::<u32>(LAST_GROUND_VALUE), 0x70);
        assert_eq!(calls_to(&e, RAY_INPUT_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&e, RAY_COLLECTOR_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&e, WORLD_CAST_RAY_DOUBLE).len(), 1);
    }

    #[test]
    fn pick_ground_ray_needs_a_shorter_distance_and_stays_within_the_limit() {
        // Limit of rays 0: no ray is cast.
        let mut e = ground_engine();
        e.set_global(LIMIT_OBJECT_RAY, 0i32);
        let (result, distance, flag) = run_pick(&mut e);
        assert_eq!((result, distance, flag), (0, f32::MAX, 0));
        assert!(calls_to(&e, RAY_INPUT_CONSTRUCT).is_empty());

        // A distance that is not a number is never below the largest float.
        let mut e = ground_engine();
        e.set_global(TEST_SQUARED, f32::NAN);
        let (result, _, flag) = run_pick(&mut e);
        assert_eq!((result, flag), (0, 0));
        assert!(calls_to(&e, RAY_INPUT_CONSTRUCT).is_empty());
    }

    #[test]
    fn pick_ground_ray_blocked_by_another_object_is_not_ground() {
        // The world cast reports a hit on another reference: not ground.
        let mut e = ground_engine();
        e.register(RAY_COLLECTOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x10, 1);
            e.mem.set_u32(a[0] + 0x14, 0x7500);
            ret(a[0])
        });
        e.register(RAY_HIT_AT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.mem.set_u32(0x7500 + 0x50, 0x7150);
        // The ray collidable resolves to the other reference.
        e.register(REFERENCE_FOR_AV_OBJECT, |_, a| {
            ret(if a[0] == 0x7250 {
                OTHER_REFERENCE
            } else {
                REFERENCE_ADDRESS
            })
        });
        let (result, distance, flag) = run_pick(&mut e);
        assert_eq!((result, distance, flag), (0, f32::MAX, 0));
        assert_eq!(calls_to(&e, GROUP_VALUE).len(), 0);

        // The cast hits the same reference: it is ground.
        let mut e = ground_engine();
        e.register(RAY_COLLECTOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x10, 1);
            e.mem.set_u32(a[0] + 0x14, 0x7500);
            ret(a[0])
        });
        e.register(RAY_HIT_AT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.mem.set_u32(0x7500 + 0x50, 0x7100);
        let (result, distance, flag) = run_pick(&mut e);
        assert_eq!((result, distance, flag), (REFERENCE_ADDRESS, 3.0, 1));
    }

    #[test]
    fn pick_ground_hit_replaces_a_picker_result_only_for_some_form_types() {
        // (form, form type, form test result, replaced)
        for (form, form_type, test, replaced) in [
            (0x55u32, 0x20u32, 0u32, true),
            (0x55, 0x1f, 0, false),
            (0x55, 0x25, 0, true),
            (0x55, 0x1e, 0, true),
            (0x55, 0x1e, 1, false),
            (0x55, 0x23, 0, false),
            (0x55, 0x2f, 0, false),
            (0, 0x1f, 0, true),
        ] {
            let mut e = ground_engine();
            // The picker result is another reference with value 100.0.
            e.set_global(TEST_RESULT_COUNT, 1u32);
            e.set_global(TEST_FORM, form);
            e.set_global(TEST_FORM_TYPE, form_type);
            e.set_global(TEST_FORM_TEST, test);
            let (result, distance, flag) = run_pick(&mut e);
            if replaced {
                assert_eq!((result, distance, flag), (REFERENCE_ADDRESS, 3.0, 1));
            } else {
                assert_eq!((result, distance, flag), (OTHER_REFERENCE, 100.0, 0));
            }
        }
    }

    #[test]
    fn pick_actor_in_a_blocking_state_never_counts_as_ground() {
        for (state, casts) in [(9u32, false), (4, false), (3, true)] {
            let mut e = ground_engine();
            e.set_global(TEST_TYPE_CODE, 0x1eu32);
            // Slot 0x100 says the reference is an actor.
            e.register(0x8904, |_, _| ret(1));
            e.register(SAVED_ACQUIRE_OBJECT, |_, _| ret(0xa100));
            e.register(CHAR_CONTROLLER, |_, _| ret(0xb000));
            for page in [0xa000, 0xb000] {
                e.map(page, 0x1000);
            }
            e.put_vtable(0xa200, &[0; 0x140]);
            e.mem.set_u32(0xa100, 0xa200);
            e.mem.set_u32(0xa200 + ACTOR_SLOT_4BC, 0xa300);
            e.set_global(TEST_STATE, state);
            e.register(0xa300, |e, _| ret(e.mem.u32(TEST_STATE)));
            // `fn_00632d00` reads the controller word at +0x600.
            e.mem.set_u32(0xb000 + 0x600, 1);
            run_pick(&mut e);
            assert_eq!(
                !calls_to(&e, RAY_INPUT_CONSTRUCT).is_empty(),
                casts,
                "state {state}"
            );
        }

        // A controller word other than 1 does not block.
        let mut e = ground_engine();
        e.set_global(TEST_TYPE_CODE, 0x1eu32);
        e.register(0x8904, |_, _| ret(1));
        e.register(SAVED_ACQUIRE_OBJECT, |_, _| ret(0xa100));
        e.register(CHAR_CONTROLLER, |_, _| ret(0xb000));
        for page in [0xa000, 0xb000] {
            e.map(page, 0x1000);
        }
        e.mem.set_u32(0xb000 + 0x600, 2);
        run_pick(&mut e);
        assert_eq!(calls_to(&e, RAY_INPUT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn pick_names_a_reference_for_the_picker_when_its_group_is_unseen() {
        // Group 0 and a reference whose slot 0x1d0 gives a non-null name.
        let mut e = pick_engine();
        e.set_global(TEST_GROUP, 0u32);
        e.register(0x8900, |_, _| ret(0x9990));
        run_pick(&mut e);
        let added = calls_to(&e, PICK_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].len(), 6);
        assert_eq!(added[0][1..4], [0x4000, 0x4100, 8.0f32.to_bits()]);
        assert_eq!(added[0][4..], [0x9990, 0]);

        // The limit of references is 0: nothing is added.
        let mut e = pick_engine();
        e.set_global(TEST_GROUP, 0u32);
        e.register(0x8900, |_, _| ret(0x9990));
        e.set_global(LIMIT_OBJECT_REFERENCE, 0i32);
        run_pick(&mut e);
        assert!(calls_to(&e, PICK_ADD).is_empty());
    }
}
