//! `fallout shared/pathfinding/navmeshsearchslpoint.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator delete` (`00401030`, cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The vtable of `NavMeshSearchSLPoint` (`0106b96c`, stored by its
/// destructor).
const SEARCH_SL_POINT_VTABLE: u32 = 0x0106_b96c;
/// `004ff7e0` (unnamed in the engine map): the `PathingLocation` destructor
/// body.
const LOCATION_DTOR_004FF7E0: u32 = 0x004f_f7e0;
/// `006a6b00` (navmeshsearch unit): the `NavMeshSearch` destructor body.
const NAVMESH_SEARCH_DTOR_006A6B00: u32 = 0x006a_6b00;
/// `006a9540` (unnamed in the engine map): the request's goal location.
const REQUEST_GOAL_006A9540: u32 = 0x006a_9540;
/// `PathingLocation::operator=` (`006dcce0`, engine map; `RET 4`).
const LOCATION_ASSIGN_006DCCE0: u32 = 0x006d_cce0;
/// `009b0bd0`, `006d2c40`, `006d2c20` (unnamed in the engine map): three
/// floats of the request (ST0).
const REQUEST_VALUE_009B0BD0: u32 = 0x009b_0bd0;
const REQUEST_VALUE_006D2C40: u32 = 0x006d_2c40;
const REQUEST_VALUE_006D2C20: u32 = 0x006d_2c20;
/// `0069ef80` (navmeshpovsearch unit): the request's float at +0x68 (ST0).
const REQUEST_VALUE_0069EF80: u32 = 0x0069_ef80;
/// `006ad1b0`, `006ad250` (unnamed in the engine map; `RET 0xC`): the
/// constructors of a node and a link scrap array with a reserve.
const NODE_SCRAP_ARRAY_CTOR_006AD1B0: u32 = 0x006a_d1b0;
const LINK_SCRAP_ARRAY_CTOR_006AD250: u32 = 0x006a_d250;
/// `006abde0` (unnamed in the engine map): the link scrap array's
/// destructor body; `006abc50` (navmeshsearchmaxcost unit): the node scrap
/// array's.
const LINK_SCRAP_ARRAY_DTOR_006ABDE0: u32 = 0x006a_bde0;
const NODE_SCRAP_ARRAY_DTOR_006ABC50: u32 = 0x006a_bc50;
/// `006a6b70` (navmeshsearch unit; `RET 0x18`): `NavMeshSearch`'s search
/// run.
const SEARCH_RUN_006A6B70: u32 = 0x006a_6b70;
/// `005c3420` (unnamed in the engine map; `RET 4`: out): a location's
/// position into `out`; returns `out`.
const LOCATION_POSITION_005C3420: u32 = 0x005c_3420;
/// `006815c0` (unnamed in the engine map): returns its `this`.
const IDENTITY_006815C0: u32 = 0x0068_15c0;
/// `00697440` (unnamed in the engine map; `RET 8`: triangle, out): a point
/// of a triangle for the search into `out`.
const TRIANGLE_POINT_00697440: u32 = 0x0069_7440;
/// `004578c0` (unnamed in the engine map; `RET 4`): subtracts a 3D vector
/// from `this`.
const POINT3_SUBTRACT_004578C0: u32 = 0x0045_78c0;
/// `0063c8a0` (unnamed in the engine map; `RET 4`): adds a 3D vector to
/// `this`.
const POINT3_ADD_0063C8A0: u32 = 0x0063_c8a0;
/// `00595c80` (unnamed in the engine map): a 3D vector's horizontal length
/// (ST0).
const LENGTH_2D_00595C80: u32 = 0x0059_5c80;
/// `00457990` (unnamed in the engine map): a 3D vector's length (ST0).
const POINT3_LENGTH_00457990: u32 = 0x0045_7990;
/// `NiPoint3::UnitizeGetLength` (`00457910`, engine map; ST0).
const UNITIZE_GET_LENGTH_00457910: u32 = 0x0045_7910;
/// `005582f0` (unnamed in the engine map; `RET 4`): the triangle record of
/// a navmesh; `005582d0` (`RET 4`): a 16-bit vertex index of the record.
const TRIANGLE_005582F0: u32 = 0x0055_82f0;
const TRIANGLE_VERTEX_005582D0: u32 = 0x0055_82d0;
/// `0068f0a0` (unnamed in the engine map; `RET 4`): the address of a
/// navmesh vertex's position.
const VERTEX_POSITION_0068F0A0: u32 = 0x0068_f0a0;
/// `0068f160` (unnamed in the engine map; `RET 0x10`: triangle, three
/// outs): the addresses of a triangle's three vertex positions.
const TRIANGLE_VERTICES_0068F160: u32 = 0x0068_f160;
/// `006a6eb0` (navmeshsearch unit; `RET 4`: out): a search node's center
/// into `out`; returns `out`.
const NODE_CENTER_006A6EB0: u32 = 0x006a_6eb0;
/// `NiPoint3::Dot` (`004b6190`, engine map; `RET 4`; ST0).
const DOT_004B6190: u32 = 0x004b_6190;
/// `0045bb20` (unnamed in the engine map; `RET 8`: out, scale): the 3D
/// product `this * scale` into `out`; returns `out`.
const POINT3_SCALE_0045BB20: u32 = 0x0045_bb20;
/// `00439e90` (unnamed in the engine map; `RET 8`: out, other): the 3D sum
/// `this + other` into `out`; returns `out`.
const POINT3_PLUS_00439E90: u32 = 0x0043_9e90;
/// `00439ef0` (unnamed in the engine map; `RET 8`: out, other): the 3D
/// difference `this - other` into `out`; returns `out`.
const POINT3_MINUS_00439EF0: u32 = 0x0043_9ef0;
/// `004e4450` (unnamed in the engine map; cdecl, a float; ST0).
const ROUND_UP_004E4450: u32 = 0x004e_4450;
/// `_ftol2_sse` (`00ec62c0`): truncates the `f64` it is given.
const FTOL: u32 = 0x00ec_62c0;
/// The `double` (`01011588`) the first step along the segment is scaled by.
const HALF_STEP: u32 = 0x0101_1588;
/// `0.0` (double).
const ZERO: u32 = 0x0101_2060;
/// The `double` under which the segment counts as too short (`01032980`)
/// and the `double` then added to its start's x (`01020758`).
const MIN_LENGTH: u32 = 0x0103_2980;
const LENGTH_NUDGE: u32 = 0x0102_0758;
/// `NiPoint3::UnitCross` (`0053d1a0`, engine map; `RET 8`: out, other).
const UNIT_CROSS_0053D1A0: u32 = 0x0053_d1a0;
/// The vector (`011a9484`) the segment is crossed with.
const UP_011A9484: u32 = 0x011a_9484;
/// `0068efb0` (unnamed in the engine map): the navmesh's info.
const NAVMESH_INFO_0068EFB0: u32 = 0x0068_efb0;
/// `006dcf20` (unnamed in the engine map; `RET 8`: position, info): a
/// `PathingLocation` constructor.
const LOCATION_CTOR_006DCF20: u32 = 0x006d_cf20;
/// `006dd0d0` (unnamed in the engine map; `RET 8`: position, location): a
/// `PathingLocation` constructor from a position and another location.
const LOCATION_FROM_006DD0D0: u32 = 0x006d_d0d0;
/// `006d7350` (unnamed in the engine map; cdecl, four words): whether the
/// straight line between two locations is clear (AL).
const STRAIGHT_LINE_006D7350: u32 = 0x006d_7350;
/// `0042fb00`, `0042fa40` (unnamed in the engine map): the navmesh holder's
/// constructor and destructor.
const HOLDER_CTOR_0042FB00: u32 = 0x0042_fb00;
const HOLDER_DTOR_0042FA40: u32 = 0x0042_fa40;
/// `PathingLocation::GetNavMeshAndTriangle` (`006dd640`, engine map; `RET 8`).
const GET_NAVMESH_AND_TRIANGLE_006DD640: u32 = 0x006d_d640;
/// `00559450` (unnamed in the engine map): the word at +0 of its `this`.
const FIRST_OF_00559450: u32 = 0x0055_9450;
/// `00697730` (unnamed in the engine map; `RET 0x14`: triangle, point,
/// height out, two floats): the height of a point over a triangle (AL).
const TRIANGLE_HEIGHT_00697730: u32 = 0x0069_7730;
/// The float (`0106b158`) `00697730` gets twice.
const HEIGHT_TOLERANCE: u32 = 0x0106_b158;
/// `00439180` (unnamed in the engine map; `RET 4`): scales `this` by a
/// float.
const POINT3_SCALE_IN_PLACE_00439180: u32 = 0x0043_9180;
/// `00452dc0` (unnamed in the engine map; `RET 8`): a 2D point from two
/// floats.
const POINT2_SET_00452DC0: u32 = 0x0045_2dc0;
/// `004e7560` (unnamed in the engine map): a 2D vector's length (ST0).
const POINT2_LENGTH_004E7560: u32 = 0x004e_7560;
/// `004579e0` (unnamed in the engine map; cdecl, a float): its square root
/// (ST0).
const SQRT_004579E0: u32 = 0x0045_79e0;
/// `006ad180` (unnamed in the engine map; `RET 4`): scales a 2D vector to a
/// length.
const POINT2_SET_LENGTH_006AD180: u32 = 0x006a_d180;

/// `NavMeshSearchSLPoint` fields (PC offsets; the PC layout leaves out the
/// Xbox's debug arrays, so these are named by their use): the target
/// location (+0x20A4, a `PathingLocation`), the found point (+0x20CC), the
/// side direction (+0x20D8), the test start point (+0x20E4) and the
/// request's floats `006ac220` keeps: +0x20F8 from
/// `009b0bd0`, +0x20F4 and +0x20F0 its sums with `006d2c40`'s and
/// `006d2c20`'s (the far and near distances of `006ac3f0`'s test), and
/// +0x20FC from `0069ef80` (the width the side tests shift by).
const TARGET_LOCATION: u32 = 0x20a4;
const FOUND_POINT: u32 = 0x20cc;
const SIDE: u32 = 0x20d8;
const TEST_START: u32 = 0x20e4;
const NEAR_DISTANCE: u32 = 0x20f0;
const FAR_DISTANCE: u32 = 0x20f4;
const BASE_DISTANCE: u32 = 0x20f8;
const WIDTH: u32 = 0x20fc;

fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 006ac1b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ac1b0 - its FS:[0] exception frame (the unwind state of the target location) is not translated
/// The destructor body of `NavMeshSearchSLPoint` (by the vtable it stores):
/// the target location (+0x20A4, `004ff7e0`), then the `NavMeshSearch`
/// base (`006a6b00`).
pub fn fn_006ac1b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SEARCH_SL_POINT_VTABLE);
    e.call(
        LOCATION_DTOR_004FF7E0,
        &args![this.addr() + TARGET_LOCATION],
    );
    e.call(NAVMESH_SEARCH_DTOR_006A6B00, &args![this]);
}

// Translated from 006ac180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchSLPoint::scalar deleting destructor` (Xbox PDB):
/// `006ac1b0`, then `operator delete` when bit 0 of `flags` is set.
pub fn navmesh_search_sl_point_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006ac1b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// The shared set-up of `006ac220` and `006ac340`: the target location from
/// the request's goal (`006dcce0`), and the request's floats: +0x20F8 from
/// `009b0bd0`, +0x20F4 its sum with `006d2c40`'s, +0x20F0 its sum with
/// `006d2c20`'s, +0x20FC from `0069ef80`.
fn take_request(e: &mut Engine, this: u32, request: u32) {
    let goal = e.call(REQUEST_GOAL_006A9540, &args![request]).u32();
    e.call(
        LOCATION_ASSIGN_006DCCE0,
        &args![this + TARGET_LOCATION, goal],
    );
    let base_range = e.call(REQUEST_VALUE_009B0BD0, &args![request]).f64();
    e.mem.set_u32(this + BASE_DISTANCE, x87_store32(base_range));
    let outer = e.call(REQUEST_VALUE_006D2C40, &args![request]).f64();
    let base_range = e.call(REQUEST_VALUE_009B0BD0, &args![request]).f64();
    e.mem
        .set_u32(this + FAR_DISTANCE, x87_store32(x87_add(base_range, outer)));
    let inner = e.call(REQUEST_VALUE_006D2C20, &args![request]).f64();
    let base_range = e.call(REQUEST_VALUE_009B0BD0, &args![request]).f64();
    e.mem.set_u32(
        this + NEAR_DISTANCE,
        x87_store32(x87_add(base_range, inner)),
    );
    let radius = e.call(REQUEST_VALUE_0069EF80, &args![request]).f64();
    e.mem.set_u32(this + WIDTH, x87_store32(radius));
}

// Translated from 006ac220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ac220 - its FS:[0] exception frame (the unwind state of the local scrap arrays) is not translated
/// Unnamed in the engine map (`RET 0xC`: the request, two words): takes the
/// request (as `006ac340`) and runs the search (`006a6b70(request, first,
/// second, nodes, links, 0)`) with local node and link scrap arrays
/// (`006ad1b0`, `006ad250` with a reserve of 0x20). Returns its result.
pub fn fn_006ac220(e: &mut Engine, this: Ptr, request: u32, first: u32, second: u32) -> bool {
    let base = this.addr();
    take_request(e, base, request);
    e.with_stack(0x28, |e, locals| {
        let nodes = locals.addr();
        let links = locals.addr() + 0x14;
        e.call(
            NODE_SCRAP_ARRAY_CTOR_006AD1B0,
            &args![nodes, 0x20u32, 0u32, 0u32],
        );
        e.call(
            LINK_SCRAP_ARRAY_CTOR_006AD250,
            &args![links, 0x20u32, 0u32, 0u32],
        );
        let found = e
            .call(
                SEARCH_RUN_006A6B70,
                &args![this, request, first, second, nodes, links, 0u32],
            )
            .u8();
        e.call(LINK_SCRAP_ARRAY_DTOR_006ABDE0, &args![links]);
        e.call(NODE_SCRAP_ARRAY_DTOR_006ABC50, &args![nodes]);
        found != 0
    })
}

// Translated from 006ac340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0x14`: the request, four words): takes
/// the request (target location and floats) and returns what
/// `006a6b70(request, a, b, c, d, 0)` returns (EAX).
pub fn fn_006ac340(e: &mut Engine, this: Ptr, request: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    take_request(e, this.addr(), request);
    e.call(SEARCH_RUN_006A6B70, &args![this, request, a, b, c, d, 0u32])
        .u32()
}

/// Destroys the locations `006ac8b0` has made, in the order given.
fn destroy_locations(e: &mut Engine, locations: &[u32]) {
    for location in locations {
        e.call(LOCATION_DTOR_004FF7E0, &args![*location]);
    }
}

// Translated from 006ac8b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ac8b0 - its FS:[0] exception frame (the unwind state of its eight locations and the holder) is not translated
/// Unnamed in the engine map (`RET 0x10`: from, to, the navmesh, out): tests
/// a straight-line point between `from` and `to`. The segment (nudged in x
/// by the double at `01020758` when shorter than the one at `01032980`)
/// gives the test point at the inner range (+0x20F4) and the test start at
/// the outer range plus the radius from `to`; the side direction (+0x20D8)
/// is the unit cross with `011a9484`. The straight lines (`006d7350`)
/// between the test start's and `to`'s locations, and both shifted by the
/// radius to either side, must be clear, and the test point's location must
/// give a navmesh and triangle (`006dd640`); otherwise false. Below the
/// test point (`00697730` height negative) and with `to` above the test
/// start, the point moves horizontally by what the range circle leaves
/// (false when the circles do not meet). The point is raised by the height
/// and written to `out`. Every made location is destroyed (`004ff7e0`) on
/// each exit.
pub fn fn_006ac8b0(e: &mut Engine, this: Ptr, from: Ptr, to: Ptr, navmesh: u32, out: Ptr) -> bool {
    let base = this.addr();
    let words = [from.addr(), to.addr(), navmesh, out.addr()];
    e.with_stack(0x210 + 8 + 4 * words.len() as u32, |e, block| {
        let ebp = block.addr() + 0x210;
        let from = from.addr();
        let to = to.addr();
        let segment = ebp - 0x44;
        copy_words(e, segment, from, 3);
        e.call(POINT3_SUBTRACT_004578C0, &args![segment, to]);
        let length = e.call(POINT3_LENGTH_00457990, &args![segment]).f64();
        e.mem.set_u32(ebp - 0x10, x87_store32(length));
        let min_length: f64 = e.global(MIN_LENGTH);
        if x87_load32(e.mem.u32(ebp - 0x10)) < min_length {
            copy_words(e, segment, from, 3);
            let nudge: f64 = e.global(LENGTH_NUDGE);
            let x = x87_add(x87_load32(e.mem.u32(segment)), nudge);
            e.mem.set_u32(segment, x87_store32(x));
            e.call(POINT3_SUBTRACT_004578C0, &args![segment, to]);
            let length = e.call(POINT3_LENGTH_00457990, &args![segment]).f64();
            e.mem.set_u32(ebp - 0x10, x87_store32(length));
        }
        let length_bits = e.mem.u32(ebp - 0x10);
        let inner = x87_div(
            x87_load32(e.mem.u32(base + FAR_DISTANCE)),
            x87_m32(length_bits),
        );
        e.mem.set_u32(ebp - 0x8c, x87_store32(inner));
        let outer = x87_add(
            x87_load32(e.mem.u32(base + FAR_DISTANCE)),
            x87_m32(e.mem.u32(base + WIDTH)),
        );
        let outer = x87_div(outer, x87_m32(length_bits));
        e.mem.set_u32(ebp - 0x70, x87_store32(outer));
        let point = ebp - 0xe8;
        let scale = x87_quiet_bits(e.mem.u32(ebp - 0x8c));
        e.call(POINT3_SCALE_0045BB20, &args![segment, point, scale]);
        let scale = x87_quiet_bits(e.mem.u32(ebp - 0x70));
        let start = e
            .call(POINT3_SCALE_0045BB20, &args![segment, ebp - 0x1bc, scale])
            .u32();
        copy_words(e, base + TEST_START, start, 3);
        let target = ebp - 0x7c;
        copy_words(e, target, to, 3);
        e.call(POINT3_ADD_0063C8A0, &args![point, to]);
        e.call(POINT3_ADD_0063C8A0, &args![base + TEST_START, to]);
        let side = e
            .call(
                UNIT_CROSS_0053D1A0,
                &args![segment, ebp - 0x1c8, UP_011A9484],
            )
            .u32();
        copy_words(e, base + SIDE, side, 3);
        let offset = ebp - 0x88;
        let radius = x87_quiet_bits(e.mem.u32(base + WIDTH));
        e.call(POINT3_SCALE_0045BB20, &args![base + SIDE, offset, radius]);

        let info = e.call(NAVMESH_INFO_0068EFB0, &args![navmesh]).u32();
        let start_location = ebp - 0x38;
        e.call(
            LOCATION_CTOR_006DCF20,
            &args![start_location, base + TEST_START, info],
        );
        let target_location = ebp - 0x138;
        e.call(
            LOCATION_FROM_006DD0D0,
            &args![target_location, target, base + TARGET_LOCATION],
        );
        let clear = e
            .call(
                STRAIGHT_LINE_006D7350,
                &args![start_location, target_location, 0u32, 0u32],
            )
            .u8();
        if clear == 0 {
            destroy_locations(e, &[target_location, start_location]);
            return false;
        }
        let left_start = ebp - 0x110;
        let shifted = e
            .call(
                POINT3_MINUS_00439EF0,
                &args![base + TEST_START, ebp - 0x1d8, offset],
            )
            .u32();
        e.call(
            LOCATION_FROM_006DD0D0,
            &args![left_start, shifted, start_location],
        );
        let left_target = ebp - 0x6c;
        let shifted = e
            .call(POINT3_MINUS_00439EF0, &args![target, ebp - 0x1e4, offset])
            .u32();
        e.call(
            LOCATION_FROM_006DD0D0,
            &args![left_target, shifted, target_location],
        );
        let clear = e
            .call(
                STRAIGHT_LINE_006D7350,
                &args![left_start, left_target, 0u32, 0u32],
            )
            .u8();
        if clear == 0 {
            destroy_locations(
                e,
                &[left_target, left_start, target_location, start_location],
            );
            return false;
        }
        let right_start = ebp - 0xb4;
        let shifted = e
            .call(
                POINT3_PLUS_00439E90,
                &args![base + TEST_START, ebp - 0x1f4, offset],
            )
            .u32();
        e.call(
            LOCATION_FROM_006DD0D0,
            &args![right_start, shifted, start_location],
        );
        let right_target = ebp - 0xdc;
        let shifted = e
            .call(POINT3_PLUS_00439E90, &args![target, ebp - 0x200, offset])
            .u32();
        e.call(
            LOCATION_FROM_006DD0D0,
            &args![right_target, shifted, target_location],
        );
        let clear = e
            .call(
                STRAIGHT_LINE_006D7350,
                &args![right_start, right_target, 0u32, 0u32],
            )
            .u8();
        if clear == 0 {
            destroy_locations(
                e,
                &[
                    right_target,
                    right_start,
                    left_target,
                    left_start,
                    target_location,
                    start_location,
                ],
            );
            return false;
        }
        let point_location = ebp - 0x160;
        e.call(
            LOCATION_FROM_006DD0D0,
            &args![point_location, point, start_location],
        );
        let holder = ebp - 0x168;
        e.call(HOLDER_CTOR_0042FB00, &args![holder]);
        let all = [
            point_location,
            right_target,
            right_start,
            left_target,
            left_start,
            target_location,
            start_location,
        ];
        let found = e
            .call(
                GET_NAVMESH_AND_TRIANGLE_006DD640,
                &args![point_location, holder, ebp - 0x170],
            )
            .u8();
        e.mem.set_u8(ebp - 0x169, found);
        if e.mem.u8(ebp - 0x169) == 0 {
            e.call(HOLDER_DTOR_0042FA40, &args![holder]);
            destroy_locations(e, &all);
            return false;
        }
        let tolerance = x87_quiet_bits(e.global::<u32>(HEIGHT_TOLERANCE));
        let tolerance_again = x87_quiet_bits(e.global::<u32>(HEIGHT_TOLERANCE));
        let triangle_navmesh = e.call(FIRST_OF_00559450, &args![holder]).u32();
        let height = ebp - 0x164;
        let over = e
            .call(
                TRIANGLE_HEIGHT_00697730,
                &args![
                    triangle_navmesh,
                    u32::from(e.mem.u16(ebp - 0x170)),
                    point,
                    height,
                    tolerance,
                    tolerance_again
                ],
            )
            .u8();
        e.mem.set_u8(ebp - 0x171, over);
        let zero: f64 = e.global(ZERO);
        if x87_load32(e.mem.u32(height)) < zero {
            let above = x87_sub(
                x87_load32(e.mem.u32(target + 8)),
                x87_m32(e.mem.u32(base + TEST_START + 8)),
            );
            if above > zero {
                let rest = ebp - 0x180;
                copy_words(e, rest, to, 3);
                e.call(POINT3_SUBTRACT_004578C0, &args![rest, point]);
                let distance = e.call(POINT3_LENGTH_00457990, &args![rest]).f64();
                e.mem.set_u32(ebp - 0x184, x87_store32(distance));
                let left = x87_sub(
                    x87_load32(e.mem.u32(ebp - 0x184)),
                    x87_m32(e.mem.u32(base + BASE_DISTANCE)),
                );
                let left = x87_sub(left, x87_m32(e.mem.u32(base + WIDTH)));
                e.mem.set_u32(ebp - 0x18c, x87_store32(left));
                let fraction = x87_div(
                    x87_load32(e.mem.u32(ebp - 0x18c)),
                    x87_m32(e.mem.u32(ebp - 0x184)),
                );
                e.mem.set_u32(ebp - 0x210, x87_store32(fraction));
                let fraction = x87_quiet_bits(e.mem.u32(ebp - 0x210));
                e.call(POINT3_SCALE_IN_PLACE_00439180, &args![rest, fraction]);
                e.call(POINT3_PLUS_00439E90, &args![point, ebp - 0x1b0, rest]);
                let flat = ebp - 0x19c;
                let x = x87_quiet_bits(e.mem.u32(rest));
                let y = x87_quiet_bits(e.mem.u32(rest + 4));
                e.call(POINT2_SET_00452DC0, &args![flat, x, y]);
                let flat_length = e.call(POINT2_LENGTH_004E7560, &args![flat]).f64();
                e.mem.set_u32(ebp - 0x190, x87_store32(flat_length));
                let squared = x87_mul(
                    x87_load32(e.mem.u32(ebp - 0x18c)),
                    x87_m32(e.mem.u32(ebp - 0x18c)),
                );
                e.mem.set_u32(ebp - 0x1a0, x87_store32(squared));
                let rise = x87_add(x87_load32(e.mem.u32(point + 8)), x87_m32(e.mem.u32(height)));
                let rise = x87_sub(x87_m32(e.mem.u32(ebp - 0x1a8)), rise);
                e.mem.set_u32(ebp - 0x1a4, x87_store32(rise));
                let rise = x87_mul(
                    x87_load32(e.mem.u32(ebp - 0x1a4)),
                    x87_m32(e.mem.u32(ebp - 0x1a4)),
                );
                e.mem.set_u32(ebp - 0x1a4, x87_store32(rise));
                if x87_load32(e.mem.u32(ebp - 0x1a0)) < x87_load32(e.mem.u32(ebp - 0x1a4)) {
                    e.call(HOLDER_DTOR_0042FA40, &args![holder]);
                    destroy_locations(e, &all);
                    return false;
                }
                let across = x87_sub(
                    x87_load32(e.mem.u32(ebp - 0x1a0)),
                    x87_m32(e.mem.u32(ebp - 0x1a4)),
                );
                e.mem.set_u32(ebp - 0x188, x87_store32(across));
                let across = x87_quiet_bits(e.mem.u32(ebp - 0x188));
                let root = e.call(SQRT_004579E0, &args![across]).f64();
                e.mem.set_u32(ebp - 0x188, x87_store32(root));
                let flat_length = x87_load32(e.mem.u32(ebp - 0x190));
                let root = x87_load32(e.mem.u32(ebp - 0x188));
                if flat_length <= root {
                    e.call(HOLDER_DTOR_0042FA40, &args![holder]);
                    destroy_locations(e, &all);
                    return false;
                }
                let shift = x87_sub(
                    x87_load32(e.mem.u32(ebp - 0x190)),
                    x87_m32(e.mem.u32(ebp - 0x188)),
                );
                e.mem.set_u32(ebp - 0x194, x87_store32(shift));
                let shift = x87_quiet_bits(e.mem.u32(ebp - 0x194));
                e.call(POINT2_SET_LENGTH_006AD180, &args![flat, shift]);
                let x = x87_add(x87_load32(e.mem.u32(point)), x87_m32(e.mem.u32(flat)));
                e.mem.set_u32(point, x87_store32(x));
                let y = x87_add(
                    x87_load32(e.mem.u32(point + 4)),
                    x87_m32(e.mem.u32(flat + 4)),
                );
                e.mem.set_u32(point + 4, x87_store32(y));
            }
        }
        let z = x87_add(x87_load32(e.mem.u32(point + 8)), x87_m32(e.mem.u32(height)));
        e.mem.set_u32(point + 8, x87_store32(z));
        copy_words(e, out.addr(), point, 3);
        e.call(HOLDER_DTOR_0042FA40, &args![holder]);
        destroy_locations(e, &all);
        true
    })
}

// Translated from 006ac3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshSearchSLPoint::IsGoalNode` (Xbox PDB; `RET 4`): whether a point
/// on the node's triangle has a clear straight line to the target. The
/// target's horizontal distance to the triangle's point (`00697440`) must
/// be within the outer range plus the distance from the triangle's first
/// vertex (squared, ordered), and not inside the inner range (when that is
/// positive). The node's center is tried first (`006ac8b0`); then, when the
/// vertex is farther than the radius, points along the triangle's extent
/// across the side direction (its vertices' extreme dot products, `00457910`
/// for the length, `004e4450` for the count) at half-step offsets. A clear
/// point is kept at +0x20CC.
pub fn navmesh_search_sl_point_is_goal_node(e: &mut Engine, this: Ptr, node: Ptr) -> bool {
    let base = this.addr();
    e.with_stack(0x100, |e, locals| {
        let ebp = locals.addr() + 0x100;
        let navmesh = e.mem.u32(node.addr() + 8);
        e.mem.set_u32(ebp - 8, navmesh);
        let triangle = e.mem.u16(node.addr() + 0xc);
        e.mem.set_u16(ebp - 4, triangle);
        let target = ebp - 0x30;
        e.call(
            LOCATION_POSITION_005C3420,
            &args![base + TARGET_LOCATION, target],
        );
        let offset = ebp - 0x48;
        copy_words(e, offset, target, 3);
        let near = ebp - 0x3c;
        e.call(IDENTITY_006815C0, &args![near]);
        e.call(
            TRIANGLE_POINT_00697440,
            &args![e.mem.u32(ebp - 8), u32::from(e.mem.u16(ebp - 4)), near],
        );
        e.call(POINT3_SUBTRACT_004578C0, &args![offset, near]);
        let horizontal = e.call(LENGTH_2D_00595C80, &args![offset]).f64();
        e.mem.set_u32(ebp - 0x1c, x87_store32(horizontal));
        let record = e
            .call(
                TRIANGLE_005582F0,
                &args![e.mem.u32(ebp - 8), u32::from(e.mem.u16(ebp - 4))],
            )
            .u32();
        let vertex = e.call(TRIANGLE_VERTEX_005582D0, &args![record, 0u32]).u32() & 0xffff;
        let position = e
            .call(VERTEX_POSITION_0068F0A0, &args![e.mem.u32(ebp - 8), vertex])
            .u32();
        let corner = ebp - 0x14;
        copy_words(e, corner, position, 3);
        e.call(POINT3_SUBTRACT_004578C0, &args![corner, near]);
        let reach = e.call(POINT3_LENGTH_00457990, &args![corner]).f64();
        e.mem.set_u32(ebp - 0x24, x87_store32(reach));
        let outer = x87_add(
            x87_load32(e.mem.u32(base + FAR_DISTANCE)),
            x87_m32(e.mem.u32(ebp - 0x24)),
        );
        e.mem.set_u32(ebp - 0x20, x87_store32(outer));
        let squared = x87_mul(
            x87_load32(e.mem.u32(ebp - 0x20)),
            x87_m32(e.mem.u32(ebp - 0x20)),
        );
        e.mem.set_u32(ebp - 0x18, x87_store32(squared));
        let squared = x87_load32(e.mem.u32(ebp - 0x18));
        let distance = x87_load32(e.mem.u32(ebp - 0x1c));
        if !(squared >= distance) {
            return false;
        }
        let inner = x87_sub(
            x87_load32(e.mem.u32(base + NEAR_DISTANCE)),
            x87_m32(e.mem.u32(ebp - 0x24)),
        );
        e.mem.set_u32(ebp - 0x4c, x87_store32(inner));
        let zero: f64 = e.global(ZERO);
        if x87_load32(e.mem.u32(ebp - 0x4c)) > zero {
            let inner_squared = x87_mul(
                x87_load32(e.mem.u32(ebp - 0x4c)),
                x87_m32(e.mem.u32(ebp - 0x4c)),
            );
            if x87_load32(e.mem.u32(ebp - 0x1c)) < inner_squared {
                return false;
            }
        }
        let center = ebp - 0x64;
        e.call(NODE_CENTER_006A6EB0, &args![node.addr() + 8, center]);
        let found = ebp - 0x58;
        e.call(IDENTITY_006815C0, &args![found]);
        if fn_006ac8b0(
            e,
            this,
            Ptr::new(center),
            Ptr::new(target),
            e.mem.u32(ebp - 8),
            Ptr::new(found),
        ) {
            copy_words(e, base + FOUND_POINT, found, 3);
            return true;
        }
        let radius = x87_load32(e.mem.u32(base + WIDTH));
        if !(radius < x87_load32(e.mem.u32(ebp - 0x24))) {
            return false;
        }
        let vertices = ebp - 0xa0;
        e.call(
            TRIANGLE_VERTICES_0068F160,
            &args![
                e.mem.u32(ebp - 8),
                u32::from(e.mem.u16(ebp - 4)),
                vertices,
                vertices + 4,
                vertices + 8
            ],
        );
        let relative = ebp - 0x94;
        copy_words(e, relative, e.mem.u32(vertices), 3);
        e.call(POINT3_SUBTRACT_004578C0, &args![relative, target]);
        let dot = e.call(DOT_004B6190, &args![relative, base + SIDE]).f64();
        e.mem.set_u32(ebp - 0xc4, x87_store32(dot));
        let low = x87_quiet_bits(e.mem.u32(ebp - 0xc4));
        e.mem.set_u32(ebp - 0x78, low);
        let mut index = 1i32;
        while index < 3 {
            let vertex = e.mem.u32(vertices.wrapping_add((index as u32) * 4));
            copy_words(e, relative, vertex, 3);
            e.call(POINT3_SUBTRACT_004578C0, &args![relative, target]);
            let dot = e.call(DOT_004B6190, &args![relative, base + SIDE]).f64();
            e.mem.set_u32(ebp - 0xd4, x87_store32(dot));
            let value = x87_load32(e.mem.u32(ebp - 0xd4));
            if x87_load32(e.mem.u32(ebp - 0xc4)) < value {
                let value = x87_quiet_bits(e.mem.u32(ebp - 0xd4));
                e.mem.set_u32(ebp - 0xc4, value);
            } else if x87_load32(e.mem.u32(ebp - 0x78)) > value {
                let value = x87_quiet_bits(e.mem.u32(ebp - 0xd4));
                e.mem.set_u32(ebp - 0x78, value);
            }
            index = index.wrapping_add(1);
        }
        let low = x87_quiet_bits(e.mem.u32(ebp - 0x78));
        let scaled = e
            .call(POINT3_SCALE_0045BB20, &args![base + SIDE, ebp - 0xe4, low])
            .u32();
        let first = ebp - 0x88;
        e.call(
            POINT3_PLUS_00439E90,
            &args![base + TEST_START, first, scaled],
        );
        let high = x87_quiet_bits(e.mem.u32(ebp - 0xc4));
        let scaled = e
            .call(POINT3_SCALE_0045BB20, &args![base + SIDE, ebp - 0xf0, high])
            .u32();
        e.call(
            POINT3_PLUS_00439E90,
            &args![base + TEST_START, ebp - 0x74, scaled],
        );
        let span = ebp - 0xbc;
        copy_words(e, span, ebp - 0x74, 3);
        e.call(POINT3_SUBTRACT_004578C0, &args![span, first]);
        let length = e.call(UNITIZE_GET_LENGTH_00457910, &args![span]).f64();
        e.mem.set_u32(ebp - 0xcc, x87_store32(length));
        let steps = x87_div(
            x87_load32(e.mem.u32(ebp - 0xcc)),
            x87_m32(e.mem.u32(base + WIDTH)),
        );
        e.mem.set_u32(ebp - 0xb0, x87_store32(steps));
        let steps = x87_quiet_bits(e.mem.u32(ebp - 0xb0));
        let rounded = e.call(ROUND_UP_004E4450, &args![steps]).f64();
        e.mem.set_u32(ebp - 0xc0, x87_store32(rounded));
        let count = e
            .call(FTOL, &args![x87_load32(e.mem.u32(ebp - 0xc0))])
            .i32();
        let step = x87_div(
            x87_load32(e.mem.u32(ebp - 0xcc)),
            x87_m32(e.mem.u32(ebp - 0xc0)),
        );
        e.mem.set_u32(ebp - 0x7c, x87_store32(step));
        let half: f64 = e.global(HALF_STEP);
        let along = x87_mul(x87_load32(e.mem.u32(ebp - 0x7c)), half);
        e.mem.set_u32(ebp - 0x68, x87_store32(along));
        let candidate = ebp - 0xac;
        e.call(IDENTITY_006815C0, &args![candidate]);
        let mut index = 0i32;
        while index < count {
            let along = x87_quiet_bits(e.mem.u32(ebp - 0x68));
            let offset = e
                .call(POINT3_SCALE_0045BB20, &args![span, ebp - 0xfc, along])
                .u32();
            copy_words(e, candidate, offset, 3);
            e.call(POINT3_ADD_0063C8A0, &args![candidate, first]);
            if fn_006ac8b0(
                e,
                this,
                Ptr::new(candidate),
                Ptr::new(target),
                e.mem.u32(ebp - 8),
                Ptr::new(found),
            ) {
                copy_words(e, base + FOUND_POINT, found, 3);
                return true;
            }
            let along = x87_add(
                x87_load32(e.mem.u32(ebp - 0x68)),
                x87_m32(e.mem.u32(ebp - 0x7c)),
            );
            e.mem.set_u32(ebp - 0x68, x87_store32(along));
            index = index.wrapping_add(1);
        }
        false
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_c1b0, fn_006ac1b0(Ptr)),
        entry!(
            0x006a_c180,
            navmesh_search_sl_point_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006a_c220, fn_006ac220(Ptr, u32, u32, u32) -> bool),
        entry!(0x006a_c340, fn_006ac340(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x006a_c8b0, fn_006ac8b0(Ptr, Ptr, Ptr, u32, Ptr) -> bool),
        entry!(
            0x006a_c3f0,
            navmesh_search_sl_point_is_goal_node(Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod n031_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn logged(e: &mut Engine, run: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        run(e);
        e.call_log.take().unwrap()
    }

    fn point(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
        let p = e.mem.alloc(0xc);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        e.mem.set_f32(p + 8, z);
        p
    }

    /// The straight-line test's doubles: vector helpers on floats, the
    /// side direction (1, 0, 0), straight lines and the navmesh lookup as
    /// given, a height of 0.5.
    fn straight_lines(e: &mut Engine, clear: bool, found: bool) {
        e.register(POINT3_SUBTRACT_004578C0, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        e.register(POINT3_ADD_0063C8A0, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        e.register(POINT3_MINUS_00439EF0, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT3_PLUS_00439E90, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT3_SCALE_0045BB20, |e, a| {
            let scale = f32::from_bits(a[2]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * scale;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT3_LENGTH_00457990, |e, a| {
            let v = [e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8)];
            f64::from((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()).into_ret()
        });
        e.register(UNIT_CROSS_0053D1A0, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[1] + 4, 0.0);
            e.mem.set_f32(a[1] + 8, 0.0);
            a[1].into_ret()
        });
        stub(
            e,
            &[
                NAVMESH_INFO_0068EFB0,
                LOCATION_CTOR_006DCF20,
                LOCATION_FROM_006DD0D0,
                LOCATION_DTOR_004FF7E0,
                HOLDER_CTOR_0042FB00,
                HOLDER_DTOR_0042FA40,
            ],
        );
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        if clear {
            e.register(STRAIGHT_LINE_006D7350, |_, _| 1u32.into_ret());
        } else {
            e.register(STRAIGHT_LINE_006D7350, |_, _| Ret::default());
        }
        if found {
            e.register(GET_NAVMESH_AND_TRIANGLE_006DD640, |_, _| 1u32.into_ret());
        } else {
            e.register(GET_NAVMESH_AND_TRIANGLE_006DD640, |_, _| Ret::default());
        }
        e.register(TRIANGLE_HEIGHT_00697730, |e, a| {
            e.mem.set_f32(a[3], 0.5);
            1u32.into_ret()
        });
        e.map(0x0101_0000, 0x10_0000);
        e.set_global::<f64>(MIN_LENGTH, 0.01);
        e.set_global::<f64>(HALF_STEP, 0.5);
    }

    fn sl_search(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x2178);
        e.mem.set_f32(this + FAR_DISTANCE, 4.0);
        e.mem.set_f32(this + NEAR_DISTANCE, 0.5);
        e.mem.set_f32(this + WIDTH, 1.0);
        this
    }

    #[test]
    fn at_006ac1b0_and_006ac180_destroy_the_search() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                LOCATION_DTOR_004FF7E0,
                NAVMESH_SEARCH_DTOR_006A6B00,
                OPERATOR_DELETE,
            ],
        );
        let this = e.mem.alloc(0x2178);
        let log = logged(&mut e, |e| {
            e.call(0x006a_c1b0, &args![this]);
        });
        assert_eq!(e.mem.u32(this), SEARCH_SL_POINT_VTABLE);
        assert_eq!(
            calls_to(&log, LOCATION_DTOR_004FF7E0),
            vec![vec![this + TARGET_LOCATION]]
        );
        assert_eq!(
            calls_to(&log, NAVMESH_SEARCH_DTOR_006A6B00),
            vec![vec![this]]
        );
        for flags in [0u32, 1] {
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(0x006a_c180, &args![this, flags]).u32(), this);
            });
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), flags as usize);
        }
    }

    fn request_floats(e: &mut Engine) {
        stub(e, &[REQUEST_GOAL_006A9540, LOCATION_ASSIGN_006DCCE0]);
        e.register(REQUEST_VALUE_009B0BD0, |_, _| 1.0f64.into_ret());
        e.register(REQUEST_VALUE_006D2C40, |_, _| 2.0f64.into_ret());
        e.register(REQUEST_VALUE_006D2C20, |_, _| 4.0f64.into_ret());
        e.register(REQUEST_VALUE_0069EF80, |_, _| 0.5f64.into_ret());
    }

    #[test]
    fn at_006ac220_takes_the_request_and_searches() {
        let mut e = Engine::new();
        request_floats(&mut e);
        stub(
            &mut e,
            &[
                NODE_SCRAP_ARRAY_CTOR_006AD1B0,
                LINK_SCRAP_ARRAY_CTOR_006AD250,
                LINK_SCRAP_ARRAY_DTOR_006ABDE0,
                NODE_SCRAP_ARRAY_DTOR_006ABC50,
            ],
        );
        e.register(SEARCH_RUN_006A6B70, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x2178);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(0x006a_c220, &args![this, 0x40u32, 1u32, 2u32])
                .bool());
        });
        assert_eq!(e.mem.f32(this + BASE_DISTANCE), 1.0);
        assert_eq!(e.mem.f32(this + FAR_DISTANCE), 3.0);
        assert_eq!(e.mem.f32(this + NEAR_DISTANCE), 5.0);
        assert_eq!(e.mem.f32(this + WIDTH), 0.5);
        let runs = calls_to(&log, SEARCH_RUN_006A6B70);
        assert_eq!(runs.len(), 1);
        assert_eq!(&runs[0][..4], &[this, 0x40, 1, 2]);
        assert_eq!(runs[0][5], runs[0][4] + 0x14);
        assert_eq!(runs[0][6], 0);
        let nodes = calls_to(&log, NODE_SCRAP_ARRAY_CTOR_006AD1B0);
        assert_eq!(&nodes[0][1..], &[0x20, 0, 0]);
    }

    #[test]
    fn at_006ac340_returns_the_search_run() {
        let mut e = Engine::new();
        request_floats(&mut e);
        e.register(SEARCH_RUN_006A6B70, |_, _| 0x1234u32.into_ret());
        let this = e.mem.alloc(0x2178);
        let log = logged(&mut e, |e| {
            assert_eq!(
                e.call(0x006a_c340, &args![this, 0x40u32, 1u32, 2u32, 3u32, 4u32])
                    .u32(),
                0x1234
            );
        });
        assert_eq!(
            calls_to(&log, SEARCH_RUN_006A6B70),
            vec![vec![this, 0x40, 1, 2, 3, 4, 0]]
        );
        assert_eq!(e.mem.f32(this + FAR_DISTANCE), 3.0);
    }

    #[test]
    fn at_006ac8b0_tests_the_straight_lines() {
        let mut e = Engine::new();
        straight_lines(&mut e, false, true);
        let this = sl_search(&mut e);
        let from = point(&mut e, 0.0, 10.0, 0.0);
        let to = point(&mut e, 0.0, 0.0, 0.0);
        let out = e.mem.alloc(0xc);
        // The first line blocked: false, its two locations destroyed.
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(0x006a_c8b0, &args![this, from, to, 0x5000u32, out])
                .bool());
        });
        assert_eq!(calls_to(&log, LOCATION_DTOR_004FF7E0).len(), 2);
        assert_eq!(e.mem.f32(this + TEST_START + 4), 5.0);
        assert_eq!(e.mem.f32(this + SIDE), 1.0);
        // Clear, but no navmesh at the test point: false, all destroyed.
        straight_lines(&mut e, true, false);
        let log = logged(&mut e, |e| {
            assert!(!e
                .call(0x006a_c8b0, &args![this, from, to, 0x5000u32, out])
                .bool());
        });
        assert_eq!(calls_to(&log, STRAIGHT_LINE_006D7350).len(), 3);
        assert_eq!(calls_to(&log, LOCATION_DTOR_004FF7E0).len(), 7);
        assert_eq!(calls_to(&log, HOLDER_DTOR_0042FA40).len(), 1);
        // Found: the test point raised by the height.
        straight_lines(&mut e, true, true);
        assert!(e
            .call(0x006a_c8b0, &args![this, from, to, 0x5000u32, out])
            .bool());
        assert_eq!(
            (e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)),
            (0.0, 4.0, 0.5)
        );
    }

    #[test]
    fn at_006ac3f0_checks_the_ranges_then_the_center() {
        let mut e = Engine::new();
        straight_lines(&mut e, true, true);
        e.register(LOCATION_POSITION_005C3420, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            a[1].into_ret()
        });
        stub(
            &mut e,
            &[
                IDENTITY_006815C0,
                TRIANGLE_POINT_00697440,
                TRIANGLE_005582F0,
                TRIANGLE_VERTEX_005582D0,
            ],
        );
        e.register(LENGTH_2D_00595C80, |e, a| {
            let (x, y) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            f64::from((x * x + y * y).sqrt()).into_ret()
        });
        let corner = point(&mut e, 1.0, 0.0, 0.0);
        e.register_double(VERTEX_POSITION_0068F0A0, move |_, _| corner.into_ret());
        e.register(NODE_CENTER_006A6EB0, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 8 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            a[1].into_ret()
        });
        let this = sl_search(&mut e);
        let node = e.mem.alloc(0x3c);
        e.mem.set_f32(node + 0x14, 10.0);
        // Too far: outer (4 + 1) squared below the distance.
        e.mem.set_f32(this + TARGET_LOCATION, 30.0);
        assert!(!e.call(0x006a_c3f0, &args![this, node]).bool());
        // In range: the center passes, its point kept.
        e.mem.set_f32(this + TARGET_LOCATION, 3.0);
        assert!(e.call(0x006a_c3f0, &args![this, node]).bool());
        assert_eq!(e.mem.f32(this + FOUND_POINT + 8), 0.5);
        // Blocked center and a vertex not beyond the width: false.
        e.register(STRAIGHT_LINE_006D7350, |_, _| Ret::default());
        assert!(!e.call(0x006a_c3f0, &args![this, node]).bool());
    }

    #[test]
    fn at_006ac8b0_moves_a_low_point_across_the_circle() {
        let mut e = Engine::new();
        straight_lines(&mut e, true, true);
        e.register(TRIANGLE_HEIGHT_00697730, |e, a| {
            e.mem.set_f32(a[3], -0.1);
            1u32.into_ret()
        });
        e.register(POINT3_SCALE_IN_PLACE_00439180, |e, a| {
            let scale = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * scale;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        e.register(POINT2_SET_00452DC0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(POINT2_LENGTH_004E7560, |e, a| {
            let (x, y) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            f64::from((x * x + y * y).sqrt()).into_ret()
        });
        e.register(SQRT_004579E0, |_, a| {
            f64::from(f32::from_bits(a[0]).sqrt()).into_ret()
        });
        stub(&mut e, &[POINT2_SET_LENGTH_006AD180]);
        let this = sl_search(&mut e);
        let from = point(&mut e, 0.0, 6.0, -8.0);
        let to = point(&mut e, 0.0, 0.0, 0.0);
        let out = e.mem.alloc(0xc);
        let log = logged(&mut e, |e| {
            assert!(e
                .call(0x006a_c8b0, &args![this, from, to, 0x5000u32, out])
                .bool());
        });
        assert_eq!(calls_to(&log, POINT2_SET_LENGTH_006AD180).len(), 1);
        // The flat step (0, -1.8) moves the point (0, 2.4, -3.2); the height
        // (-0.1) is added to its z.
        assert_eq!(e.mem.f32(out), 0.0);
        assert!((e.mem.f32(out + 4) - 0.6).abs() < 1e-5);
        assert!((e.mem.f32(out + 8) + 3.3).abs() < 1e-5);
        // The circles do not meet: false.
        e.register(POINT2_LENGTH_004E7560, |_, _| 1.0f64.into_ret());
        assert!(!e
            .call(0x006a_c8b0, &args![this, from, to, 0x5000u32, out])
            .bool());
    }
}
