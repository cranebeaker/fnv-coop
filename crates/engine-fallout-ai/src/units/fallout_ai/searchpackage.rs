//! `fallout/ai/searchpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESPackage::TESPackage` (`006707c0`, engine map; `ECX` only).
const PACKAGE_CONSTRUCT: u32 = 0x0067_07c0;
/// `006708b0` (`ECX` only): the `TESPackage` destructor body.
const PACKAGE_DESTRUCT_006708B0: u32 = 0x0067_08b0;
/// The `SearchPackage` vtable (`01093674`).
const SEARCH_PACKAGE_VTABLE: u32 = 0x0109_3674;
/// `006815c0` (`ECX` only): returns `this` (a point's trivial constructor).
const POINT_CONSTRUCT_006815C0: u32 = 0x0068_15c0;
/// `PathingRequestCover::PathingRequestCover` (`006e4080`, engine map; `ECX` only).
const PATHING_REQUEST_COVER_CONSTRUCT: u32 = 0x006e_4080;
/// `006dad70` (`ECX` only, `pathfind.cpp`): the cover request's destructor body.
const PATHING_REQUEST_COVER_DESTRUCT_006DAD70: u32 = 0x006d_ad70;
/// `006dae60` (`ECX` only, `pathfind.cpp`): constructs the cover location array.
const COVER_ARRAY_CONSTRUCT_006DAE60: u32 = 0x006d_ae60;
/// `BSSimpleArray<PathingCoverLocation,1024>::Clear` (`006dae90`, engine map;
/// `ECX` only): its destructor body here.
const COVER_ARRAY_CLEAR: u32 = 0x006d_ae90;
/// `PathingLocation::PathingLocation_ov7` (`006dcee0`, engine map;
/// `thiscall`, `RET 0xC`: the address of a point, the cell, the worldspace).
const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_cee0;
/// `004ff7e0` (`ECX` only): the `PathingLocation` destructor body.
const PATHING_LOCATION_DESTRUCT_004FF7E0: u32 = 0x004f_f7e0;
/// `006d3ac0` (`thiscall` on the cover request, `RET 4`): sets its start
/// location.
const REQUEST_SET_START_006D3AC0: u32 = 0x006d_3ac0;
/// `006dad40` (`thiscall` on the cover request, `RET 4`: the address of a
/// point): sets the point the cover is searched around.
const REQUEST_SET_POINT_006DAD40: u32 = 0x006d_ad40;
/// `006d3b00` (`thiscall` on the cover request, `RET 4`: a `float`): sets
/// the search distance.
const REQUEST_SET_DISTANCE_006D3B00: u32 = 0x006d_3b00;
/// `006d62e0` (cdecl: the cover request, the cover location array): runs
/// the search into the array.
const FIND_COVER_006D62E0: u32 = 0x006d_62e0;
/// `1000.0` (`float`).
const THOUSAND: u32 = 0x0101_3974;
/// `800.0` (`float`).
const EIGHT_HUNDRED: u32 = 0x0102_2454;
/// `300.0` (`float`).
const THREE_HUNDRED: u32 = 0x0101_3dc0;
/// `004390c0` (`thiscall`, `RET 4`): whether two points are equal.
const POINT_EQUALS_004390C0: u32 = 0x0043_90c0;
/// The point at `011f426c` the stored point is compared with.
const POINT_011F426C: u32 = 0x011f_426c;
/// The actor's virtual slot (byte offset) that returns the address of its
/// position.
const SLOT_POSITION: u32 = 0x1f4;
/// `008e3dc0` (cdecl, seven words: the result, a point by value, two
/// `float`s and a word; `highprocess.cpp`): a point near the given one;
/// returns the result's address.
const POINT_NEAR_008E3DC0: u32 = 0x008e_3dc0;
/// `008d6f30` (`ECX` only): a reference's cell.
const REFERENCE_CELL_008D6F30: u32 = 0x008d_6f30;
/// `TESObjectREFR::GetWorldSpace` (`00575d70`, engine map; `ECX` only).
const REFERENCE_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `Pathing::FindPointOnNavMesh` (`006d7110`, engine map; cdecl: the
/// worldspace, the cell, the address of a point, a `float` and the address
/// of the result).
const FIND_POINT_ON_NAV_MESH: u32 = 0x006d_7110;
/// `0076b610` (`ECX` only): whether an array's size (+8) is zero.
const ARRAY_IS_EMPTY_0076B610: u32 = 0x0076_b610;
/// `0044ddc0` (`ECX` only): an array's size.
const ARRAY_SIZE_0044DDC0: u32 = 0x0044_ddc0;
/// `009a3280` (`thiscall`, `RET 4`): the address of a cover location by index.
const COVER_AT_009A3280: u32 = 0x009a_3280;
/// `PathingCoverLocation::PathingCoverLocation` (`0099aa80`, engine map;
/// `thiscall`, `RET 4`: the one copied).
const COVER_LOCATION_COPY: u32 = 0x0099_aa80;
/// `005c3420` (`thiscall`, `RET 4`: the result): the cover location's point;
/// returns the result's address.
const COVER_LOCATION_POINT_005C3420: u32 = 0x005c_3420;
/// `006d32c0` (`ECX` only): the cover location's destructor body.
const COVER_LOCATION_DESTRUCT_006D32C0: u32 = 0x006d_32c0;

/// `SearchPackage` (Xbox PDB names where known; PC offsets as the code uses
/// them): the search point (three `float`s) at +0x94, the word at +0xA0,
/// the cover request at +0xA8, the found cover locations at +0x194 and the
/// index of the next one at +0x8C.
const SEARCH_POINT: u32 = 0x94;
const COVER_REQUEST: u32 = 0xa8;
const COVER_LOCATIONS: u32 = 0x194;
const NEXT_COVER: u32 = 0x8c;

fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// A `float` constant as `FLD dword; FSTP dword [ESP]` pushes it.
fn pushed_float(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(address))
}

// Translated from 009f66b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f66b0 - its FS:[0] exception frame (the unwind states that destroy the package and the cover request when a later constructor throws) is not translated
/// `SearchPackage::SearchPackage` (engine map; `RET 0x14`): the
/// `TESPackage` constructor, the class's vtable, the point at +0x94
/// (`006815c0`), the cover request at +0xA8 (`PathingRequestCover`), the
/// cover location array at +0x194 (`006dae60`); 0.0 at +0x80, 1 at +0x84,
/// 0.0 at +0x88 and +0x90; the point `x`, `y`, `z` (copied through the x87
/// stack) at +0x94; `word` at +0xA0, 1 in the byte at +0xA4 and 0 at +0x8C.
/// The fifth stack word (a byte its callers push) is not read. Returns
/// `this`.
pub fn search_package_search_package(
    e: &mut Engine,
    this: Ptr,
    x: f32,
    y: f32,
    z: f32,
    word: u32,
    _unused_5: u32,
) -> Ptr {
    let base = this.addr();
    e.call(PACKAGE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, SEARCH_PACKAGE_VTABLE);
    e.call(POINT_CONSTRUCT_006815C0, &args![base + SEARCH_POINT]);
    e.call(
        PATHING_REQUEST_COVER_CONSTRUCT,
        &args![base + COVER_REQUEST],
    );
    e.call(
        COVER_ARRAY_CONSTRUCT_006DAE60,
        &args![base + COVER_LOCATIONS],
    );
    e.mem.set_u32(base + 0x80, x87_store32(0.0));
    e.mem.set_u32(base + 0x84, 1);
    e.mem.set_u32(base + 0x88, x87_store32(0.0));
    e.mem.set_u32(base + 0x90, x87_store32(0.0));
    e.mem.set_u32(base + 0x98, x87_quiet_bits(y.to_bits()));
    e.mem.set_u32(base + 0x94, x87_quiet_bits(x.to_bits()));
    e.mem.set_u32(base + 0x9c, x87_quiet_bits(z.to_bits()));
    e.mem.set_u32(base + 0xa0, word);
    e.mem.set_u8(base + 0xa4, 1);
    e.mem.set_u32(base + NEXT_COVER, 0);
    this
}

// Translated from 009f67b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f67b0 - its FS:[0] exception frame (the unwind states that finish destroying the members when one destructor throws) is not translated
/// The `SearchPackage` destructor body (unnamed in the engine map; `ECX`
/// only): the class's vtable, then the cover location array (`006dae90`),
/// the cover request (`006dad70`) and the `TESPackage` part (`006708b0`).
pub fn fn_009f67b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SEARCH_PACKAGE_VTABLE);
    e.call(COVER_ARRAY_CLEAR, &args![base + COVER_LOCATIONS]);
    e.call(
        PATHING_REQUEST_COVER_DESTRUCT_006DAD70,
        &args![base + COVER_REQUEST],
    );
    e.call(PACKAGE_DESTRUCT_006708B0, &args![this]);
}

// Translated from 009f6830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f6830 - its FS:[0] exception frame (the unwind state that destroys the local PathingLocation) is not translated
/// Unnamed in the engine map (`RET 0x20`): searches cover. Its eight stack
/// words are a start point and a search point (three words each, by value),
/// a cell and a worldspace. A local `PathingLocation` (`006dcee0` with the
/// address of the start point, the cell and the worldspace) becomes the
/// cover request's start (`006d3ac0`) and is destroyed (`004ff7e0`); the
/// request gets the search point (`006dad40`, its address) and the distance
/// 1000.0 (`006d3b00`), and `006d62e0` runs it into the array at +0x194.
pub fn fn_009f6830(
    e: &mut Engine,
    this: Ptr,
    start_x: u32,
    start_y: u32,
    start_z: u32,
    point_x: u32,
    point_y: u32,
    point_z: u32,
    cell: u32,
    world: u32,
) {
    let base = this.addr();
    let request = base + COVER_REQUEST;
    // The stack words (EBP+8 on) and the local PathingLocation (EBP-0x34).
    e.with_stack(0x48, |e, frame| {
        let words = frame.addr();
        let location = words + 0x20;
        let stack = [
            start_x, start_y, start_z, point_x, point_y, point_z, cell, world,
        ];
        for (i, word) in stack.iter().enumerate() {
            e.mem.set_u32(words + 4 * i as u32, *word);
        }
        let start = e
            .call(
                PATHING_LOCATION_CONSTRUCT,
                &args![location, words, cell, world],
            )
            .u32();
        e.call(REQUEST_SET_START_006D3AC0, &args![request, start]);
        e.call(PATHING_LOCATION_DESTRUCT_004FF7E0, &args![location]);
        e.call(REQUEST_SET_POINT_006DAD40, &args![request, words + 0xc]);
    });
    let distance = pushed_float(e, THOUSAND);
    e.call(REQUEST_SET_DISTANCE_006D3B00, &args![request, distance]);
    e.call(FIND_COVER_006D62E0, &args![request, base + COVER_LOCATIONS]);
}

/// `FindPointOnNavMesh(world, cell, point, 300.0, result)` with the actor's
/// worldspace and cell (asked for in the exe's order: the cell first).
fn find_point_on_nav_mesh(e: &mut Engine, actor: u32, point: u32, result: u32) {
    let distance = pushed_float(e, THREE_HUNDRED);
    let cell = e.call(REFERENCE_CELL_008D6F30, &args![actor]).u32();
    let world = e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32();
    e.call(
        FIND_POINT_ON_NAV_MESH,
        &args![world, cell, point, distance, result],
    );
}

// Translated from 009f6900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f6900 - its FS:[0] exception frame (the unwind state that destroys the local PathingCoverLocation) is not translated
/// `SearchPackage::CalculateLocationToExamine` (engine map; `RET 8`): the
/// point to examine into `result`, which it returns. When the search point
/// (+0x94) equals the point at `011f426c`, it becomes a point near the
/// actor's position (`008e3dc0` with 800.0, 1000.0 and 0), moved onto the
/// navmesh (`FindPointOnNavMesh` from the actor's position with 300.0 into
/// the search point itself), and is the result. Otherwise, with no cover
/// found yet, cover is searched (`009f6830` with the actor's position, the
/// search point, the actor's cell and worldspace). Then the next unused
/// cover location (+0x8C, counted up), copied into a local
/// (`0099aa80`), gives the point (`005c3420`; the local destroyed by
/// `006d32c0`); without one, a point near the search point (`008e3dc0` with
/// 800.0, 300.0 and 0) moved onto the navmesh in place.
pub fn search_package_calculate_location_to_examine(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    actor: Ptr,
) -> Ptr {
    let base = this.addr();
    let actor = actor.addr();
    let search_point = base + SEARCH_POINT;
    let covers = base + COVER_LOCATIONS;
    // The frame from EBP-0x9C; `at(n)` is EBP-n. The point (EBP-0x18), the
    // cover location copy (EBP-0x74), the results of `008e3dc0` (EBP-0x80,
    // EBP-0x98) and of `005c3420` (EBP-0x8C).
    e.with_stack(0x90, |e, frame| {
        let origin = frame.addr();
        let at = move |offset: u32| origin + 0x9c - offset;
        let point = at(0x18);
        copy_words(e, point, search_point, 3);
        if e.call(POINT_EQUALS_004390C0, &args![search_point, POINT_011F426C])
            .u8()
            != 0
        {
            let near_distance = pushed_float(e, THOUSAND);
            let spread = pushed_float(e, EIGHT_HUNDRED);
            let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
            let (x, y, z) = (
                e.mem.u32(position),
                e.mem.u32(position + 4),
                e.mem.u32(position + 8),
            );
            let near = e
                .call(
                    POINT_NEAR_008E3DC0,
                    &args![at(0x80), x, y, z, spread, near_distance, 0u32],
                )
                .u32();
            copy_words(e, search_point, near, 3);
            let distance = pushed_float(e, THREE_HUNDRED);
            let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
            let cell = e.call(REFERENCE_CELL_008D6F30, &args![actor]).u32();
            let world = e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32();
            e.call(
                FIND_POINT_ON_NAV_MESH,
                &args![world, cell, position, distance, search_point],
            );
            copy_words(e, result.addr(), search_point, 3);
            return;
        }
        if e.call(ARRAY_IS_EMPTY_0076B610, &args![covers]).u8() != 0 {
            let world = e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32();
            let cell = e.call(REFERENCE_CELL_008D6F30, &args![actor]).u32();
            let (point_x, point_y, point_z) = (
                e.mem.u32(search_point),
                e.mem.u32(search_point + 4),
                e.mem.u32(search_point + 8),
            );
            let position = e.vcall(actor, SLOT_POSITION, &args![]).u32();
            let (x, y, z) = (
                e.mem.u32(position),
                e.mem.u32(position + 4),
                e.mem.u32(position + 8),
            );
            fn_009f6830(e, this, x, y, z, point_x, point_y, point_z, cell, world);
        }
        let has_next = e.call(ARRAY_IS_EMPTY_0076B610, &args![covers]).u8() == 0
            && e.mem.u32(base + NEXT_COVER) < e.call(ARRAY_SIZE_0044DDC0, &args![covers]).u32();
        if has_next {
            let index = e.mem.u32(base + NEXT_COVER);
            let cover = e.call(COVER_AT_009A3280, &args![covers, index]).u32();
            let copy = at(0x74);
            e.call(COVER_LOCATION_COPY, &args![copy, cover]);
            let next = e.mem.u32(base + NEXT_COVER).wrapping_add(1);
            e.mem.set_u32(base + NEXT_COVER, next);
            let found = e
                .call(COVER_LOCATION_POINT_005C3420, &args![copy, at(0x8c)])
                .u32();
            copy_words(e, point, found, 3);
            e.call(COVER_LOCATION_DESTRUCT_006D32C0, &args![copy]);
        } else {
            let near_distance = pushed_float(e, THREE_HUNDRED);
            let spread = pushed_float(e, EIGHT_HUNDRED);
            let (x, y, z) = (
                e.mem.u32(search_point),
                e.mem.u32(search_point + 4),
                e.mem.u32(search_point + 8),
            );
            let near = e
                .call(
                    POINT_NEAR_008E3DC0,
                    &args![at(0x98), x, y, z, spread, near_distance, 0u32],
                )
                .u32();
            copy_words(e, point, near, 3);
            find_point_on_nav_mesh(e, actor, point, point);
        }
        copy_words(e, result.addr(), point, 3);
    });
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009f_66b0,
            search_package_search_package(Ptr, f32, f32, f32, u32, u32) -> Ptr
        ),
        entry!(0x009f_67b0, fn_009f67b0(Ptr)),
        entry!(
            0x009f_6830,
            fn_009f6830(Ptr, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x009f_6900,
            search_package_calculate_location_to_examine(Ptr, Ptr, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn constants(e: &mut Engine) {
        e.map(0x0101_3000, 0x1000);
        e.map(0x0102_2000, 0x1000);
        e.set_global(THOUSAND, 1000.0f32);
        e.set_global(EIGHT_HUNDRED, 800.0f32);
        e.set_global(THREE_HUNDRED, 300.0f32);
    }

    #[test]
    fn at_009f66b0_constructs_the_members_and_the_point() {
        let mut e = Engine::new();
        for callee in [
            PACKAGE_CONSTRUCT,
            POINT_CONSTRUCT_006815C0,
            PATHING_REQUEST_COVER_CONSTRUCT,
            COVER_ARRAY_CONSTRUCT_006DAE60,
        ] {
            e.register(callee, nothing);
        }
        let this = e.mem.alloc(0x1a4);
        e.mem.set_u32(this + 0x8c, 9);
        e.call_log = Some(vec![]);
        let result = e.call(
            0x009f_66b0,
            &args![this, 1.5f32, 2.5f32, 3.5f32, 0x77u32, 0xffu32],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), SEARCH_PACKAGE_VTABLE);
        assert_eq!(calls_to(&e, PACKAGE_CONSTRUCT), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, POINT_CONSTRUCT_006815C0),
            vec![vec![this + 0x94]]
        );
        assert_eq!(
            calls_to(&e, PATHING_REQUEST_COVER_CONSTRUCT),
            vec![vec![this + 0xa8]]
        );
        assert_eq!(
            calls_to(&e, COVER_ARRAY_CONSTRUCT_006DAE60),
            vec![vec![this + 0x194]]
        );
        assert_eq!(e.mem.f32(this + 0x94), 1.5);
        assert_eq!(e.mem.f32(this + 0x98), 2.5);
        assert_eq!(e.mem.f32(this + 0x9c), 3.5);
        assert_eq!(e.mem.u32(this + 0xa0), 0x77);
        assert_eq!(e.mem.u8(this + 0xa4), 1);
        assert_eq!(e.mem.u32(this + 0x84), 1);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
        // A signalling NaN is quieted by the FLD/FSTP copy.
        e.call(
            0x009f_66b0,
            &args![
                this,
                f32::from_bits(0x7f80_0001),
                0.0f32,
                0.0f32,
                0u32,
                0u32
            ],
        );
        assert_eq!(e.mem.u32(this + 0x94), 0x7fc0_0001);
    }

    #[test]
    fn at_009f67b0_destroys_the_members_then_the_package() {
        let mut e = Engine::new();
        for callee in [
            COVER_ARRAY_CLEAR,
            PATHING_REQUEST_COVER_DESTRUCT_006DAD70,
            PACKAGE_DESTRUCT_006708B0,
        ] {
            e.register(callee, nothing);
        }
        let this = e.mem.alloc(0x1a4);
        e.call_log = Some(vec![]);
        e.call(0x009f_67b0, &args![this]);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (COVER_ARRAY_CLEAR, vec![this + 0x194]),
                (PATHING_REQUEST_COVER_DESTRUCT_006DAD70, vec![this + 0xa8]),
                (PACKAGE_DESTRUCT_006708B0, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), SEARCH_PACKAGE_VTABLE);
    }

    #[test]
    fn at_009f6830_sets_up_and_runs_the_cover_request() {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(PATHING_LOCATION_CONSTRUCT, |e, a| {
            // The start point is the first three stack words, the search
            // point the next three.
            assert_eq!(e.mem.u32(a[1]), 1);
            assert_eq!(e.mem.u32(a[1] + 0xc), 4);
            a[0].into_ret()
        });
        e.register(REQUEST_SET_POINT_006DAD40, |e, a| {
            assert_eq!(
                (e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8)),
                (4, 5, 6)
            );
            Ret::default()
        });
        for callee in [
            REQUEST_SET_START_006D3AC0,
            PATHING_LOCATION_DESTRUCT_004FF7E0,
            REQUEST_SET_DISTANCE_006D3B00,
            FIND_COVER_006D62E0,
        ] {
            e.register(callee, nothing);
        }
        let this = e.mem.alloc(0x1a4);
        e.call_log = Some(vec![]);
        e.call(
            0x009f_6830,
            &args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 0xc0u32, 0xd0u32],
        );
        let location = calls_to(&e, PATHING_LOCATION_CONSTRUCT)[0].clone();
        assert_eq!(&location[2..], &[0xc0, 0xd0]);
        assert_eq!(
            calls_to(&e, REQUEST_SET_START_006D3AC0),
            vec![vec![this + 0xa8, location[0]]]
        );
        assert_eq!(
            calls_to(&e, PATHING_LOCATION_DESTRUCT_004FF7E0),
            vec![vec![location[0]]]
        );
        assert_eq!(
            calls_to(&e, REQUEST_SET_DISTANCE_006D3B00),
            vec![vec![this + 0xa8, 1000.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, FIND_COVER_006D62E0),
            vec![vec![this + 0xa8, this + 0x194]]
        );
    }

    /// The doubles `CalculateLocationToExamine` uses; the actor stands at
    /// (10, 20, 30), its cell is 0xC0 and its worldspace 0xD0.
    fn examine_world(e: &mut Engine) -> (u32, u32) {
        constants(e);
        e.map(0x011f_4000, 0x1000);
        e.register(POINT_EQUALS_004390C0, |e, a| {
            let same = (0..3).all(|i| e.mem.u32(a[0] + 4 * i) == e.mem.u32(a[1] + 4 * i));
            same.into_ret()
        });
        let position = e.mem.alloc(12);
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + SLOT_POSITION, 0x0660_0000);
        e.register_double(0x0660_0000, move |_, _| position.into_ret());
        let actor = e.mem.alloc(4);
        e.mem.set_u32(actor, vtable);
        e.register(REFERENCE_CELL_008D6F30, |_, _| 0xc0u32.into_ret());
        e.register(REFERENCE_GET_WORLD_SPACE, |_, _| 0xd0u32.into_ret());
        // A point near: the given point plus the first float in x.
        e.register(POINT_NEAR_008E3DC0, |e, a| {
            let x = f32::from_bits(a[1]) + f32::from_bits(a[4]);
            e.mem.set_f32(a[0], x);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            a[0].into_ret()
        });
        // Onto the navmesh: the result gets z = -1.
        e.register(FIND_POINT_ON_NAV_MESH, |e, a| {
            e.mem.set_f32(a[4] + 8, -1.0);
            Ret::default()
        });
        e.register(ARRAY_IS_EMPTY_0076B610, |e, a| {
            (e.mem.u32(a[0] + 8) == 0).into_ret()
        });
        e.register(ARRAY_SIZE_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        let this = e.mem.alloc(0x1a4);
        (this, actor)
    }

    #[test]
    fn at_009f6900_an_unset_point_starts_near_the_actor() {
        let mut e = Engine::new();
        let (this, actor) = examine_world(&mut e);
        let result = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        let answer = e.call(0x009f_6900, &args![this, result, actor]).u32();
        assert_eq!(answer, result);
        assert_eq!(e.mem.f32(this + 0x94), 810.0);
        assert_eq!(e.mem.f32(this + 0x9c), -1.0);
        assert_eq!(e.mem.f32(result), 810.0);
        assert_eq!(e.mem.f32(result + 4), 20.0);
        assert_eq!(e.mem.f32(result + 8), -1.0);
        let near = calls_to(&e, POINT_NEAR_008E3DC0);
        assert_eq!(&near[0][4..], &[800.0f32.to_bits(), 1000.0f32.to_bits(), 0]);
        let find = calls_to(&e, FIND_POINT_ON_NAV_MESH);
        assert_eq!(find.len(), 1);
        assert_eq!(find[0][0..2], [0xd0, 0xc0]);
        assert_eq!(find[0][3..], [300.0f32.to_bits(), this + 0x94]);
    }

    #[test]
    fn at_009f6900_searches_cover_then_takes_the_next_location() {
        let mut e = Engine::new();
        let (this, actor) = examine_world(&mut e);
        e.mem.set_f32(this + 0x94, 5.0);
        // The cover search finds two locations, whose points are (0x100 + i, 0, 0).
        e.register(PATHING_LOCATION_CONSTRUCT, |_, a| a[0].into_ret());
        for callee in [
            REQUEST_SET_START_006D3AC0,
            PATHING_LOCATION_DESTRUCT_004FF7E0,
            REQUEST_SET_POINT_006DAD40,
            REQUEST_SET_DISTANCE_006D3B00,
            COVER_LOCATION_DESTRUCT_006D32C0,
        ] {
            e.register(callee, nothing);
        }
        e.register(FIND_COVER_006D62E0, |e, a| {
            e.mem.set_u32(a[1] + 8, 2);
            Ret::default()
        });
        e.register(COVER_AT_009A3280, |_, a| (0x100 + a[1]).into_ret());
        e.register(COVER_LOCATION_COPY, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(COVER_LOCATION_POINT_005C3420, |e, a| {
            let which = e.mem.u32(a[0]);
            e.mem.set_f32(a[1], which as f32);
            e.mem.set_u32(a[1] + 4, 0);
            e.mem.set_u32(a[1] + 8, 0);
            a[1].into_ret()
        });
        let result = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        e.call(0x009f_6900, &args![this, result, actor]);
        assert_eq!(calls_to(&e, FIND_COVER_006D62E0).len(), 1);
        assert_eq!(e.mem.f32(result), 256.0);
        assert_eq!(e.mem.u32(this + 0x8c), 1);
        // The next call takes the second location without a new search.
        e.call_log = Some(vec![]);
        e.call(0x009f_6900, &args![this, result, actor]);
        assert!(calls_to(&e, FIND_COVER_006D62E0).is_empty());
        assert_eq!(e.mem.f32(result), 257.0);
        // All used: a point near the search point, onto the navmesh.
        e.call_log = Some(vec![]);
        e.call(0x009f_6900, &args![this, result, actor]);
        assert_eq!(e.mem.f32(result), 805.0);
        assert_eq!(e.mem.f32(result + 8), -1.0);
        let near = calls_to(&e, POINT_NEAR_008E3DC0);
        assert_eq!(&near[0][4..], &[800.0f32.to_bits(), 300.0f32.to_bits(), 0]);
        let find = calls_to(&e, FIND_POINT_ON_NAV_MESH);
        assert_eq!(find[0][2], find[0][4]);
        assert_eq!(e.mem.f32(this + 0x94), 5.0);
    }
}
