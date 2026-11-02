//! `fallout shared/pathfinding/pathingrequestlos.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `PathingRequestLOS` is a `PathingRequest` (0xB0 bytes) plus the
//! line-of-sight grid map, the locations to ignore, the ignore radius and
//! the search centre and radius: 0xD8 bytes on PC as in the Xbox PDB, at
//! the PDB's offsets (the scalar deleting destructor frees 0xD8 bytes).
//!
//! The compiler's exception-unwinding frames (`FS:[0]` chain and the cookie
//! word `011c16bc` stored in them) of the constructors and the destructor
//! are not translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiPoint3};

layout! {
    /// `PathingRequestLOS` (Xbox PDB), 0xD8 bytes; only the members after
    /// the `PathingRequest` base.
    pub struct PathingRequestLOS: 0xD8 {
        /// `spLOSGridMap` (Xbox PDB): `NiPointer<PathingLOSGridMap>`.
        0xB0 spLOSGridMap: Ptr,
        /// `IgnoreLocations` (Xbox PDB): `BSSimpleArray<NiPoint3,1024>`.
        0xB4 IgnoreLocations: Inline<BSSimpleArray>,
        /// `fIgnoreLocationRadius` (Xbox PDB).
        0xC4 fIgnoreLocationRadius: f32,
        /// `SearchCenter` (Xbox PDB).
        0xC8 SearchCenter: Inline<NiPoint3>,
        /// `fSearchCenterRadius` (Xbox PDB).
        0xD4 fSearchCenterRadius: f32,
    }
}

/// `PathingRequestLOS`'s vtable (slot 0 is
/// [`pathing_request_los_scalar_deleting_destructor`], slot `+8`
/// [`pathing_request_los_copy_to`], `+0x18` [`pathing_request_los_save_game`],
/// `+0x1C` [`pathing_request_los_load_game`]).
const VTABLE: u32 = 0x0106_d38c;
/// The `float` the constructor gives `fIgnoreLocationRadius` (`500.0` in
/// the exe's data).
const DEFAULT_IGNORE_RADIUS: u32 = 0x0101_3d84;

/// `PathingRequest::PathingRequest` (`006e2420`, Xbox PDB; thiscall).
const PATHING_REQUEST_CONSTRUCTOR: u32 = 0x006e_2420;
/// `PathingRequest::PathingRequest_ov2` (`006e26a0`, Xbox PDB; thiscall
/// `(this, source)`).
const PATHING_REQUEST_COPY_CONSTRUCTOR: u32 = 0x006e_26a0;
/// `006e2620` (thiscall): the base class teardown the destructor ends with.
const PATHING_REQUEST_TEARDOWN: u32 = 0x006e_2620;
/// `PathingRequest::CopyTo` (`006e2750`, Xbox PDB; `(this, dest)`).
const PATHING_REQUEST_COPY_TO: u32 = 0x006e_2750;
/// `PathingRequest::SaveGame` (`006e3230`, Xbox PDB; `(this, buffer)`).
const PATHING_REQUEST_SAVE_GAME: u32 = 0x006e_3230;
/// `PathingRequest::LoadGame` (`006e34f0`, Xbox PDB; `(this, buffer)`).
const PATHING_REQUEST_LOAD_GAME: u32 = 0x006e_34f0;
/// `00633c90(pointer, object)`: `NiPointer` constructor from a raw pointer
/// (acquires a non-null object).
const NI_POINTER_FROM_RAW: u32 = 0x0063_3c90;
/// `00559a40(pointer, &other)`: `NiPointer` copy constructor.
const NI_POINTER_COPY: u32 = 0x0055_9a40;
/// `0045cec0(pointer)`: `NiPointer` destructor (releases a non-null object).
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `NiPointer<..>::operator=` (`006e5cc0`, Xbox PDB name of the shared
/// body; `(this, &source)`).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// `0049f1c0(array)` / `0049f1f0(array)`: constructor and destructor of
/// `IgnoreLocations`.
const POINT_ARRAY_CONSTRUCTOR: u32 = 0x0049_f1c0;
const POINT_ARRAY_DESTRUCTOR: u32 = 0x0049_f1f0;
/// `006815c0(point)`: the constructor run on `SearchCenter`.
const POINT_CONSTRUCTOR: u32 = 0x0068_15c0;
/// `BSSimpleArray<CombatSearchDoor,1024>::SetSize` (`0069cfe0`, Xbox PDB
/// name of the folded body; thiscall `(source, dest)`): copies a 12-byte
/// element array into `dest`.
const POINT_ARRAY_COPY_TO: u32 = 0x0069_cfe0;
/// `0069cea0(array, size, keep)`: sizes `IgnoreLocations` on load.
const POINT_ARRAY_SET_SIZE: u32 = 0x0069_cea0;
/// `00979260(request)`: `fSearchCenterRadius` (`+0xD4`), in ST0.
const SEARCH_CENTER_RADIUS: u32 = 0x0097_9260;
/// `006ab150(request, out)`: copies `SearchCenter` (`+0xC8`) to `out`,
/// returns `out`.
const SEARCH_CENTER: u32 = 0x006a_b150;
/// `00aa1460(block, size)` (cdecl): frees a block through the memory
/// manager.
const DEALLOCATE: u32 = 0x00aa_1460;
/// `0044ddc0(array)`: an array's `iSize`.
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `006a7af0(array, index)`: the address of a 12-byte element.
const POINT_ELEMENT: u32 = 0x006a_7af0;
/// `00865e50(buffer, data, size, 0)`: writes bytes to the save buffer.
const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (`00865f60`, Xbox PDB).
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
/// `00864980(buffer, data, size)`: reads bytes from the load buffer.
const LOAD_DATA: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (`00864a60`, Xbox PDB).
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;

/// Offsets for address arithmetic (the layout's fields).
const LOS_GRID_MAP: u32 = 0xb0;
const IGNORE_LOCATIONS: u32 = 0xb4;
const IGNORE_LOCATION_RADIUS: u32 = 0xc4;
const SEARCH_CENTER_OFFSET: u32 = 0xc8;
const SEARCH_CENTER_RADIUS_OFFSET: u32 = 0xd4;

// Translated from 006e57e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e57e0 - FS:[0] exception frame (unwinding of the base and members) is not translated.
/// `PathingRequestLOS::PathingRequestLOS` (Xbox PDB): runs the
/// `PathingRequest` constructor, stores the vtable, constructs the grid map
/// pointer (null, `00633c90`), the ignore array and the search centre, sets
/// the ignore radius to the `float` at `01013d84` (500.0) and the search
/// radius to 0.0. Returns `this`.
pub fn pathing_request_los_pathing_request_los(
    e: &mut Engine,
    this: Ptr<PathingRequestLOS>,
) -> Ptr<PathingRequestLOS> {
    let t = this.addr();
    e.call(PATHING_REQUEST_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(t, VTABLE);
    e.call(NI_POINTER_FROM_RAW, &args![t + LOS_GRID_MAP, 0u32]);
    e.call(POINT_ARRAY_CONSTRUCTOR, &args![t + IGNORE_LOCATIONS]);
    e.call(POINT_CONSTRUCTOR, &args![t + SEARCH_CENTER_OFFSET]);
    let radius = x87_quiet(e.global::<f32>(DEFAULT_IGNORE_RADIUS));
    e.set(this, PathingRequestLOS::fIgnoreLocationRadius, radius);
    e.set(this, PathingRequestLOS::fSearchCenterRadius, 0.0);
    this
}

// Translated from 006e5890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestLOS::__vecDelDtor` (Xbox PDB): runs the destructor and,
/// when bit 0 of `flags` is set, frees the 0xD8 bytes (`00aa1460`).
/// Returns `this`. `RET 4`.
pub fn pathing_request_los_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PathingRequestLOS>,
    flags: u32,
) -> Ptr<PathingRequestLOS> {
    pathing_request_los_destructor(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this, 0xd8u32]);
    }
    this
}

// Translated from 006e58d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e58d0 - FS:[0] exception frame (unwinding of the members) is not translated.
/// `PathingRequestLOS::~PathingRequestLOS` (Xbox PDB): destroys the ignore
/// array (`0049f1f0`) and the grid map pointer (`0045cec0`), then runs the
/// base teardown (`006e2620`). It does not store the vtable.
pub fn pathing_request_los_destructor(e: &mut Engine, this: Ptr<PathingRequestLOS>) {
    let t = this.addr();
    e.call(POINT_ARRAY_DESTRUCTOR, &args![t + IGNORE_LOCATIONS]);
    e.call(NI_POINTER_DESTRUCT, &args![t + LOS_GRID_MAP]);
    e.call(PATHING_REQUEST_TEARDOWN, &args![this]);
}

// Translated from 006e5940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e5940 - FS:[0] exception frame (unwinding of the base and members) is not translated.
/// Unnamed in the engine map: the copy constructor: `PathingRequest_ov2(this,
/// source)`, the vtable, the grid map pointer copied (`00559a40`), the
/// ignore array constructed, the ignore radius copied (through x87), the
/// search centre constructed, `source`'s ignore locations copied into this
/// one's (`0069cfe0`), and the search centre and radius set from
/// `source`'s (`006ab150`, `00979260`) by [`fn_006e5a40`]. Returns `this`.
/// `RET 4`.
pub fn fn_006e5940(
    e: &mut Engine,
    this: Ptr<PathingRequestLOS>,
    source: Ptr<PathingRequestLOS>,
) -> Ptr<PathingRequestLOS> {
    let t = this.addr();
    let s = source.addr();
    e.call(PATHING_REQUEST_COPY_CONSTRUCTOR, &args![this, source]);
    e.mem.set_u32(t, VTABLE);
    e.call(NI_POINTER_COPY, &args![t + LOS_GRID_MAP, s + LOS_GRID_MAP]);
    e.call(POINT_ARRAY_CONSTRUCTOR, &args![t + IGNORE_LOCATIONS]);
    let radius = x87_quiet(e.get(source, PathingRequestLOS::fIgnoreLocationRadius));
    e.set(this, PathingRequestLOS::fIgnoreLocationRadius, radius);
    e.call(POINT_CONSTRUCTOR, &args![t + SEARCH_CENTER_OFFSET]);
    e.call(
        POINT_ARRAY_COPY_TO,
        &args![s + IGNORE_LOCATIONS, t + IGNORE_LOCATIONS],
    );
    let search_radius = e.call(SEARCH_CENTER_RADIUS, &args![source]).f32();
    e.with_stack(0xc, |e, center| {
        let copy = e.call(SEARCH_CENTER, &args![source, center]).u32();
        let x = e.mem.u32(copy);
        let y = e.mem.u32(copy + 4);
        let z = e.mem.u32(copy + 8);
        fn_006e5a40(e, this, x, y, z, search_radius);
    });
    this
}

// Translated from 006e5a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the search centre (the three words of
/// an `NiPoint3` passed by value) and the search radius (through x87).
/// `RET 0x10`.
pub fn fn_006e5a40(
    e: &mut Engine,
    this: Ptr<PathingRequestLOS>,
    x: u32,
    y: u32,
    z: u32,
    radius: f32,
) {
    let t = this.addr();
    e.mem.set_u32(t + SEARCH_CENTER_OFFSET, x);
    e.mem.set_u32(t + SEARCH_CENTER_OFFSET + 4, y);
    e.mem.set_u32(t + SEARCH_CENTER_OFFSET + 8, z);
    e.set(
        this,
        PathingRequestLOS::fSearchCenterRadius,
        x87_quiet(radius),
    );
}

// Translated from 006e5a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestLOS::CopyTo` (Xbox PDB): copies the base
/// (`PathingRequest::CopyTo`), assigns the grid map pointer (`006e5cc0`),
/// copies the ignore locations (`0069cfe0`), the ignore radius (through
/// x87) and the search centre and radius ([`fn_006e5a40`]) into `dest`.
/// `RET 4`.
pub fn pathing_request_los_copy_to(
    e: &mut Engine,
    this: Ptr<PathingRequestLOS>,
    dest: Ptr<PathingRequestLOS>,
) {
    let t = this.addr();
    let d = dest.addr();
    e.call(PATHING_REQUEST_COPY_TO, &args![this, dest]);
    e.call(
        NI_POINTER_ASSIGN,
        &args![d + LOS_GRID_MAP, t + LOS_GRID_MAP],
    );
    e.call(
        POINT_ARRAY_COPY_TO,
        &args![t + IGNORE_LOCATIONS, d + IGNORE_LOCATIONS],
    );
    let radius = x87_quiet(e.get(this, PathingRequestLOS::fIgnoreLocationRadius));
    e.set(dest, PathingRequestLOS::fIgnoreLocationRadius, radius);
    let search_radius = x87_quiet(e.get(this, PathingRequestLOS::fSearchCenterRadius));
    let x = e.mem.u32(t + SEARCH_CENTER_OFFSET);
    let y = e.mem.u32(t + SEARCH_CENTER_OFFSET + 4);
    let z = e.mem.u32(t + SEARCH_CENTER_OFFSET + 8);
    fn_006e5a40(e, dest, x, y, z, search_radius);
}

// Translated from 006e5b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestLOS::SaveGame` (Xbox PDB): saves the base, the number of
/// ignore locations (`SaveVariableSizedValue`; 0 when the array's address
/// is 0) and each location (12 bytes), then the ignore radius (4), the
/// search centre (12) and the search radius (4). `RET 4`.
pub fn pathing_request_los_save_game(e: &mut Engine, this: Ptr<PathingRequestLOS>, buffer: Ptr) {
    let t = this.addr();
    e.call(PATHING_REQUEST_SAVE_GAME, &args![this, buffer]);
    let array = t.wrapping_add(IGNORE_LOCATIONS);
    let count = if array == 0 {
        0
    } else {
        e.call(ARRAY_SIZE, &args![array]).u32()
    };
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count]);
    let mut index = 0u32;
    while index < count {
        let item = e.call(POINT_ELEMENT, &args![array, index]).u32();
        e.call(SAVE_DATA, &args![buffer, item, 0xcu32, 0u32]);
        index += 1;
    }
    for (offset, size) in [
        (IGNORE_LOCATION_RADIUS, 4u32),
        (SEARCH_CENTER_OFFSET, 0xc),
        (SEARCH_CENTER_RADIUS_OFFSET, 4),
    ] {
        e.call(
            SAVE_DATA,
            &args![buffer, t.wrapping_add(offset), size, 0u32],
        );
    }
}

// Translated from 006e5c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestLOS::LoadGame` (Xbox PDB): loads what
/// [`pathing_request_los_save_game`] saves: the base, the count
/// (`LoadVariableSizedValue`), the array sized to it (`0069cea0(array, n,
/// 1)`) and each location, then the three fields. `RET 4`.
pub fn pathing_request_los_load_game(e: &mut Engine, this: Ptr<PathingRequestLOS>, buffer: Ptr) {
    let t = this.addr();
    e.call(PATHING_REQUEST_LOAD_GAME, &args![this, buffer]);
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let array = t.wrapping_add(IGNORE_LOCATIONS);
    e.call(POINT_ARRAY_SET_SIZE, &args![array, count, 1u32]);
    let mut index = 0u32;
    while index < count {
        let item = e.call(POINT_ELEMENT, &args![array, index]).u32();
        e.call(LOAD_DATA, &args![buffer, item, 0xcu32]);
        index += 1;
    }
    for (offset, size) in [
        (IGNORE_LOCATION_RADIUS, 4u32),
        (SEARCH_CENTER_OFFSET, 0xc),
        (SEARCH_CENTER_RADIUS_OFFSET, 4),
    ] {
        e.call(LOAD_DATA, &args![buffer, t.wrapping_add(offset), size]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006e57e0,
            pathing_request_los_pathing_request_los(
                Ptr<PathingRequestLOS>,
            ) -> Ptr<PathingRequestLOS>
        ),
        entry!(
            0x006e5890,
            pathing_request_los_scalar_deleting_destructor(
                Ptr<PathingRequestLOS>,
                u32,
            ) -> Ptr<PathingRequestLOS>
        ),
        entry!(
            0x006e58d0,
            pathing_request_los_destructor(Ptr<PathingRequestLOS>)
        ),
        entry!(
            0x006e5940,
            fn_006e5940(Ptr<PathingRequestLOS>, Ptr<PathingRequestLOS>) -> Ptr<PathingRequestLOS>
        ),
        entry!(
            0x006e5a40,
            fn_006e5a40(Ptr<PathingRequestLOS>, u32, u32, u32, f32)
        ),
        entry!(
            0x006e5a80,
            pathing_request_los_copy_to(Ptr<PathingRequestLOS>, Ptr<PathingRequestLOS>)
        ),
        entry!(
            0x006e5b20,
            pathing_request_los_save_game(Ptr<PathingRequestLOS>, Ptr)
        ),
        entry!(
            0x006e5c00,
            pathing_request_los_load_game(Ptr<PathingRequestLOS>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet_doubles(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn constructor_sets_defaults() {
        let mut e = Engine::new();
        e.map(DEFAULT_IGNORE_RADIUS & !0xfff, 0x1000);
        e.set_global(DEFAULT_IGNORE_RADIUS, 500.0f32);
        quiet_doubles(
            &mut e,
            &[
                PATHING_REQUEST_CONSTRUCTOR,
                NI_POINTER_FROM_RAW,
                POINT_ARRAY_CONSTRUCTOR,
                POINT_CONSTRUCTOR,
            ],
        );
        let this: Ptr<PathingRequestLOS> = e.new_object();
        let t = this.addr();
        e.mem.set_f32(t + 0xd4, 3.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e57e0, &args![this]).u32(), t);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, PATHING_REQUEST_CONSTRUCTOR), vec![vec![t]]);
        assert_eq!(calls(&log, NI_POINTER_FROM_RAW), vec![vec![t + 0xb0, 0]]);
        assert_eq!(calls(&log, POINT_ARRAY_CONSTRUCTOR), vec![vec![t + 0xb4]]);
        assert_eq!(calls(&log, POINT_CONSTRUCTOR), vec![vec![t + 0xc8]]);
        assert_eq!(e.mem.u32(t), VTABLE);
        assert_eq!(e.get(this, PathingRequestLOS::fIgnoreLocationRadius), 500.0);
        assert_eq!(e.get(this, PathingRequestLOS::fSearchCenterRadius), 0.0);
    }

    #[test]
    fn destructors() {
        let mut e = Engine::new();
        quiet_doubles(
            &mut e,
            &[
                POINT_ARRAY_DESTRUCTOR,
                NI_POINTER_DESTRUCT,
                PATHING_REQUEST_TEARDOWN,
                DEALLOCATE,
            ],
        );
        let this: Ptr<PathingRequestLOS> = e.new_object();
        let t = this.addr();
        e.mem.set_u32(t, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x006e58d0, &args![this]);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (POINT_ARRAY_DESTRUCTOR, vec![t + 0xb4]),
                (NI_POINTER_DESTRUCT, vec![t + 0xb0]),
                (PATHING_REQUEST_TEARDOWN, vec![t]),
            ]
        );
        // The vtable is left as it was.
        assert_eq!(e.mem.u32(t), 0x1234);
        for (flags, frees) in [(1u32, 1usize), (0, 0), (3, 1), (2, 0)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006e5890, &args![this, flags]).u32(), t);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, PATHING_REQUEST_TEARDOWN).len(), 1);
            let freed = calls(&log, DEALLOCATE);
            assert_eq!(freed.len(), frees);
            if frees == 1 {
                assert_eq!(freed[0], vec![t, 0xd8]);
            }
        }
    }

    fn source(e: &mut Engine) -> Ptr<PathingRequestLOS> {
        let s: Ptr<PathingRequestLOS> = e.new_object();
        let a = s.addr();
        // A signalling NaN radius comes out quiet.
        e.mem.set_u32(a + 0xc4, 0x7f80_0001);
        e.mem.set_u32(a + 0xc8, 0x11);
        e.mem.set_u32(a + 0xcc, 0x22);
        e.mem.set_u32(a + 0xd0, 0x33);
        e.mem.set_f32(a + 0xd4, 6.5);
        s
    }

    #[test]
    fn copy_constructor_and_copy_to() {
        let mut e = Engine::new();
        quiet_doubles(
            &mut e,
            &[
                PATHING_REQUEST_COPY_CONSTRUCTOR,
                NI_POINTER_COPY,
                POINT_ARRAY_CONSTRUCTOR,
                POINT_CONSTRUCTOR,
                POINT_ARRAY_COPY_TO,
                PATHING_REQUEST_COPY_TO,
                NI_POINTER_ASSIGN,
            ],
        );
        e.register(SEARCH_CENTER_RADIUS, |e, a| {
            e.mem.f32(a[0] + 0xd4).into_ret()
        });
        e.register(SEARCH_CENTER, |e, a| {
            for i in 0..3 {
                let v = e.mem.u32(a[0] + 0xc8 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        let s = source(&mut e);
        let this: Ptr<PathingRequestLOS> = e.new_object();
        let t = this.addr();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e5940, &args![this, s]).u32(), t);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(t), VTABLE);
        assert_eq!(
            calls(&log, PATHING_REQUEST_COPY_CONSTRUCTOR),
            vec![vec![t, s.addr()]]
        );
        assert_eq!(
            calls(&log, NI_POINTER_COPY),
            vec![vec![t + 0xb0, s.addr() + 0xb0]]
        );
        assert_eq!(
            calls(&log, POINT_ARRAY_COPY_TO),
            vec![vec![s.addr() + 0xb4, t + 0xb4]]
        );
        assert_eq!(e.mem.u32(t + 0xc4), 0x7fc0_0001);
        assert_eq!(e.mem.u32(t + 0xcc), 0x22);
        assert_eq!(e.get(this, PathingRequestLOS::fSearchCenterRadius), 6.5);

        let dest: Ptr<PathingRequestLOS> = e.new_object();
        let d = dest.addr();
        e.call_log = Some(vec![]);
        e.call(0x006e5a80, &args![s, dest]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, PATHING_REQUEST_COPY_TO),
            vec![vec![s.addr(), d]]
        );
        assert_eq!(
            calls(&log, NI_POINTER_ASSIGN),
            vec![vec![d + 0xb0, s.addr() + 0xb0]]
        );
        assert_eq!(
            calls(&log, POINT_ARRAY_COPY_TO),
            vec![vec![s.addr() + 0xb4, d + 0xb4]]
        );
        assert_eq!(e.mem.u32(d + 0xc4), 0x7fc0_0001);
        assert_eq!(e.mem.u32(d + 0xc8), 0x11);
        assert_eq!(e.mem.u32(d + 0xd0), 0x33);
        assert_eq!(e.get(dest, PathingRequestLOS::fSearchCenterRadius), 6.5);
    }

    #[test]
    fn set_search_center() {
        let mut e = Engine::new();
        let this: Ptr<PathingRequestLOS> = e.new_object();
        e.call(0x006e5a40, &args![this, 1u32, 2u32, 3u32, 0x7f80_0005u32]);
        assert_eq!(e.mem.u32(this.addr() + 0xc8), 1);
        assert_eq!(e.mem.u32(this.addr() + 0xd0), 3);
        assert_eq!(e.mem.u32(this.addr() + 0xd4), 0x7fc0_0005);
    }

    #[test]
    fn save_and_load() {
        let mut e = Engine::new();
        quiet_doubles(
            &mut e,
            &[
                PATHING_REQUEST_SAVE_GAME,
                PATHING_REQUEST_LOAD_GAME,
                SAVE_DATA,
                SAVE_VARIABLE_SIZED_VALUE,
                LOAD_DATA,
                POINT_ARRAY_SET_SIZE,
            ],
        );
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(POINT_ELEMENT, |_, a| (0xa000 + 12 * a[1]).into_ret());
        e.register(LOAD_VARIABLE_SIZED_VALUE, |_, _| 2u32.into_ret());
        let this: Ptr<PathingRequestLOS> = e.new_object();
        let t = this.addr();
        e.mem.set_u32(t + 0xb4 + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x006e5b20, &args![this, 0x8000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, PATHING_REQUEST_SAVE_GAME),
            vec![vec![t, 0x8000]]
        );
        assert_eq!(calls(&log, ARRAY_SIZE), vec![vec![t + 0xb4]]);
        assert_eq!(
            calls(&log, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x8000, 2]]
        );
        assert_eq!(
            calls(&log, SAVE_DATA),
            vec![
                vec![0x8000, 0xa000, 12, 0],
                vec![0x8000, 0xa00c, 12, 0],
                vec![0x8000, t + 0xc4, 4, 0],
                vec![0x8000, t + 0xc8, 12, 0],
                vec![0x8000, t + 0xd4, 4, 0],
            ]
        );

        e.call_log = Some(vec![]);
        e.call(0x006e5c00, &args![this, 0x8000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, PATHING_REQUEST_LOAD_GAME),
            vec![vec![t, 0x8000]]
        );
        assert_eq!(
            calls(&log, POINT_ARRAY_SET_SIZE),
            vec![vec![t + 0xb4, 2, 1]]
        );
        assert_eq!(
            calls(&log, LOAD_DATA),
            vec![
                vec![0x8000, 0xa000, 12],
                vec![0x8000, 0xa00c, 12],
                vec![0x8000, t + 0xc4, 4],
                vec![0x8000, t + 0xc8, 12],
                vec![0x8000, t + 0xd4, 4],
            ]
        );

        // The array at address 0 (this = -0xb4) saves a count of 0 without
        // asking its size.
        let wrapped = Ptr::<PathingRequestLOS>::new(0u32.wrapping_sub(0xb4));
        e.call_log = Some(vec![]);
        e.call(0x006e5b20, &args![wrapped, 0x8000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, ARRAY_SIZE).is_empty());
        assert_eq!(
            calls(&log, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x8000, 0]]
        );
    }
}
