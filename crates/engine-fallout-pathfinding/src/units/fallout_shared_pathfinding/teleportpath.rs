//! `fallout shared/pathfinding/teleportpath.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TeleportPath` (Xbox PDB, 0x38 bytes): the cells and worldspaces a path
//! crosses (`Spaces`, a `BSSimpleArray<ParentSpaceNode>`), the teleport
//! doors between them (`Doors`, a `BSSimpleArray<TeleportLink>`), and the
//! start and end points. The file also holds the two arrays' constructors,
//! destructors and the `TeleportLink` array's allocation and
//! initialization.
//!
//! The array vtables (RTTI read from the exe):
//! `0106d974` `.?AV?$BSSimpleArray@UParentSpaceNode@@$0EAA@@@` (slots
//! `006f4be0`, `0069af80`, `006db090`),
//! `0106d988` `.?AV?$BSSimpleArray@UTeleportLink@@$0EAA@@@` (slots
//! `006f4c10`, `006f4bb0`, `006db090`).
//!
//! The compiler's exception-unwinding frames (`FS:[0]`) of the constructor
//! and destructor are not translated; those two are marked incomplete.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiPoint3};

layout! {
    /// `TeleportPath` (Xbox PDB), 0x38 bytes; the PC offsets match.
    pub struct TeleportPath: 0x38 {
        /// `Spaces` (Xbox PDB): `BSSimpleArray<ParentSpaceNode,1024>`
        /// (0x0C-byte elements).
        0x00 Spaces: Inline<BSSimpleArray>,
        /// `Doors` (Xbox PDB): `BSSimpleArray<TeleportLink,1024>` (0x10-byte
        /// [`TeleportLink`] elements).
        0x10 Doors: Inline<BSSimpleArray>,
        /// `Start` (Xbox PDB).
        0x20 Start: Inline<NiPoint3>,
        /// `End` (Xbox PDB).
        0x2C End: Inline<NiPoint3>,
    }

    /// `TeleportLink` (Xbox PDB), 0x10 bytes.
    pub struct TeleportLink: 0x10 {
        /// `pDoor` (Xbox PDB): `TESObjectREFR*`.
        0x00 pDoor: Ptr,
        /// `DoorTeleportLocation` (Xbox PDB).
        0x04 DoorTeleportLocation: Inline<NiPoint3>,
    }
}

/// `BSSimpleArray<ParentSpaceNode,1024>`'s vtable (RTTI).
const SPACES_VTABLE: u32 = 0x0106_d974;
/// `BSSimpleArray<TeleportLink,1024>`'s vtable (RTTI).
const DOORS_VTABLE: u32 = 0x0106_d988;

/// `006b3f40` (thiscall `(array, capacity, count)`, `RET 8`; engine map unit
/// `pathsmootherpov.cpp`): the `Spaces` array's initializer.
const SPACES_INITIALIZE: u32 = 0x006b_3f40;
/// `008454f0` (thiscall `(array, word)`, `RET 4`): run with 1 by the
/// arrays' destructors and by `TeleportPath::Clear`.
const ARRAY_008454F0: u32 = 0x0084_54f0;
/// `006815c0` (thiscall, no stack words): the `NiPoint3` constructor.
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// `006dbcf0` (thiscall `(array, buffer, count)`, `RET 8`): constructs
/// `count` `TeleportLink`s in `buffer`.
const DOORS_CONSTRUCT_ELEMENTS: u32 = 0x006d_bcf0;
/// `00401000` (cdecl `(size)`): `operator new`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, cdecl `(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `0069cfe0` (thiscall `(target, source)`, `RET 4`;
/// `BSSimpleArray<CombatSearchDoor,1024>::SetSize` in the engine map).
const FN_0069CFE0: u32 = 0x0069_cfe0;

/// `0076b610` (thiscall): whether the array is empty (`iSize == 0`).
const IS_EMPTY: u32 = 0x0076_b610;
/// `0044ddc0` (thiscall): the array's `iSize`.
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `00558dd0` (thiscall `(array, index)`, `RET 4`): the address of a
/// 0x10-byte element.
const ELEMENT_AT: u32 = 0x0055_8dd0;
/// `006a1420` (thiscall): the address of the first element (`00558dd0(0)`).
const FIRST_ELEMENT: u32 = 0x006a_1420;
/// `006cedb0` (thiscall): the address of the last 0x10-byte element.
const LAST_ELEMENT: u32 = 0x006c_edb0;
/// `00439ef0` (thiscall `(a, result, b)`, `RET 8`): `*result = a - b`,
/// returns `result`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00457990` (thiscall): the point's length, in `ST0`.
const POINT_LENGTH: u32 = 0x0045_7990;
/// The door reference's virtual slot +0x1F4: its position (a pointer to a
/// `NiPoint3`).
const REFERENCE_POSITION_SLOT: u32 = 0x1f4;
/// `-1.0` (`float`): the length of a path without spaces.
const NO_PATH_LENGTH: u32 = 0x0101_2054;

// Translated from 006f48b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TeleportPath::TeleportPath` (Xbox PDB): constructs both arrays
/// ([`fn_006f4b10`], [`fn_006f4b60`]) and the start and end points.
/// Returns `this`.
// Translation incomplete: 006f48b0 - FS:[0] exception frame (unwinding of the arrays) is not translated.
pub fn teleport_path_teleport_path(e: &mut Engine, this: Ptr<TeleportPath>) -> Ptr<TeleportPath> {
    fn_006f4b10(e, this.at(TeleportPath::Spaces));
    fn_006f4b60(e, this.at(TeleportPath::Doors));
    e.call(POINT_CONSTRUCT, &args![this.at(TeleportPath::Start)]);
    e.call(POINT_CONSTRUCT, &args![this.at(TeleportPath::End)]);
    this
}

// Translated from 006f4930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TeleportPath::~TeleportPath` (Xbox PDB): destroys `Doors`
/// ([`fn_006f4b90`]), then `Spaces` ([`fn_006f4b40`]).
// Translation incomplete: 006f4930 - FS:[0] exception frame (unwinding of the arrays) is not translated.
pub fn teleport_path_destructor(e: &mut Engine, this: Ptr<TeleportPath>) {
    fn_006f4b90(e, this.at(TeleportPath::Doors));
    fn_006f4b40(e, this.at(TeleportPath::Spaces));
}

// Translated from 006f4990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TeleportPath::Clear` (Xbox PDB): `008454f0(array, 1)` on `Spaces`, then
/// on `Doors`.
pub fn teleport_path_clear(e: &mut Engine, this: Ptr<TeleportPath>) {
    e.call(ARRAY_008454F0, &args![this.at(TeleportPath::Spaces), 1u32]);
    e.call(ARRAY_008454F0, &args![this.at(TeleportPath::Doors), 1u32]);
}

/// `00457990` on `a - b` (through `00439ef0` into the local `diff`).
fn distance(e: &mut Engine, a: u32, b: u32, diff: Ptr) -> f64 {
    let diff = e.call(POINT_SUBTRACT, &args![a, diff, b]).u32();
    e.call(POINT_LENGTH, &args![diff]).f64()
}

// Translated from 006f49c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TeleportPath::ComputeLength` (Xbox PDB): the path's length in `ST0`.
/// -1.0 (`01012054`) without spaces; without doors the distance from
/// `Start` to `End` (`00457990`'s result unrounded); otherwise the sum, kept
/// in a `float`, of the distances from `Start` to the first door's position
/// (virtual slot +0x1F4), from each door's position to the previous door's
/// `DoorTeleportLocation`, and from `End` to the last door's
/// `DoorTeleportLocation`.
pub fn teleport_path_compute_length(e: &mut Engine, this: Ptr<TeleportPath>) -> f64 {
    let doors = this.at(TeleportPath::Doors);
    let start = this.at(TeleportPath::Start).addr();
    let end = this.at(TeleportPath::End).addr();
    if e.call(IS_EMPTY, &args![this]).bool() {
        return x87_quiet(e.global::<f32>(NO_PATH_LENGTH)) as f64;
    }
    e.with_stack(0x30, |e, locals| {
        if e.call(IS_EMPTY, &args![doors]).bool() {
            return distance(e, end, start, locals);
        }
        let mut total = 0.0f32;
        let first = e.call(FIRST_ELEMENT, &args![doors]).u32();
        let door = e.mem.u32(first);
        let position = e.vcall(door, REFERENCE_POSITION_SLOT, &args![]).u32();
        let length = distance(e, position, start, locals.byte_add(0xc));
        total = (length + total as f64) as f32;
        let mut i = 1u32;
        while i < e.call(ARRAY_SIZE, &args![doors]).u32() {
            let link = e.call(ELEMENT_AT, &args![doors, i]).u32();
            let door = e.mem.u32(link);
            let previous = e.call(ELEMENT_AT, &args![doors, i - 1]).u32();
            let position = e.vcall(door, REFERENCE_POSITION_SLOT, &args![]).u32();
            let teleport = previous + TeleportLink::DoorTeleportLocation.off;
            let length = distance(e, position, teleport, locals.byte_add(0x18));
            total = (length + total as f64) as f32;
            i += 1;
        }
        let last = e.call(LAST_ELEMENT, &args![doors]).u32();
        let teleport = last + TeleportLink::DoorTeleportLocation.off;
        let length = distance(e, end, teleport, locals.byte_add(0x24));
        total = (length + total as f64) as f32;
        total as f64
    })
}

// Translated from 006f4b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<ParentSpaceNode,1024>`
/// constructor: stores its vtable (`0106d974`) and runs
/// `006b3f40(this, 0, 0)`. Returns `this`.
pub fn fn_006f4b10(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), SPACES_VTABLE);
    e.call(SPACES_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006f4b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<ParentSpaceNode,1024>`
/// destructor: stores its vtable (`0106d974`) and runs
/// `008454f0(this, 1)`.
pub fn fn_006f4b40(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), SPACES_VTABLE);
    e.call(ARRAY_008454F0, &args![this, 1u32]);
}

// Translated from 006f4b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<TeleportLink,1024>`
/// constructor: stores its vtable (`0106d988`) and initializes it empty
/// ([`fn_006f4c40`]). Returns `this`.
pub fn fn_006f4b60(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), DOORS_VTABLE);
    fn_006f4c40(e, this, 0, 0);
    this
}

// Translated from 006f4b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<TeleportLink,1024>`
/// destructor: stores its vtable (`0106d988`) and runs
/// `008454f0(this, 1)`.
pub fn fn_006f4b90(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), DOORS_VTABLE);
    e.call(ARRAY_008454F0, &args![this, 1u32]);
}

// Translated from 006f4bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `TeleportLink` array's allocation (vtable
/// slot +4): `operator new(count * 0x10)`. `this` is never read. `RET 4`.
pub fn fn_006f4bb0(e: &mut Engine, _this: Ptr, count: u32) -> Ptr {
    e.call(OPERATOR_NEW, &args![count << 4]).ptr()
}

// Translated from 006f4be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ParentSpaceNode,1024>::`scalar deleting destructor'`
/// (Xbox PDB): the destructor ([`fn_006f4b40`]), then `operator delete`
/// when bit 0 of `flags` is set. Returns `this`. `RET 4`.
pub fn bs_simple_array_parent_space_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_006f4b40(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006f4c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TeleportLink,1024>::`scalar deleting destructor'`
/// (Xbox PDB): the destructor ([`fn_006f4b90`]), then `operator delete`
/// when bit 0 of `flags` is set. Returns `this`. `RET 4`.
pub fn bs_simple_array_teleport_link_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_006f4b90(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006f4c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: initializes the `TeleportLink` array: clears
/// buffer, size and capacity; reserves `max(capacity, count)` elements
/// through vtable slot +4 when that is not 0; when `count` is not 0,
/// constructs that many (`006dbcf0(this, buffer, count)`) and sets `iSize`
/// to it. `RET 8`.
pub fn fn_006f4c40(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let reserved = capacity.max(count);
    if reserved != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserved]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserved);
    }
    if count != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(DOORS_CONSTRUCT_ELEMENTS, &args![this, buffer, count]);
        e.set(this, BSSimpleArray::iSize, count);
    }
}

// Translated from 009d9ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `BSSimpleArray<ParentSpaceNode,1024>::CopyTo<1024>` in the engine
/// map, but the body (shared by identical-code folding) is
/// `0069cfe0(target, this + 0xB4)`: it passes a member at +0xB4 of `this`,
/// which is not a `BSSimpleArray` offset. `RET 4`.
pub fn fn_009d9ee0(e: &mut Engine, this: Ptr, target: Ptr) {
    e.call(FN_0069CFE0, &args![target, this.addr() + 0xb4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006f48b0,
            teleport_path_teleport_path(Ptr<TeleportPath>) -> Ptr<TeleportPath>
        ),
        entry!(0x006f4930, teleport_path_destructor(Ptr<TeleportPath>)),
        entry!(0x006f4990, teleport_path_clear(Ptr<TeleportPath>)),
        entry!(
            0x006f49c0,
            teleport_path_compute_length(Ptr<TeleportPath>) -> f64
        ),
        entry!(
            0x006f4b10,
            fn_006f4b10(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006f4b40, fn_006f4b40(Ptr<BSSimpleArray>)),
        entry!(
            0x006f4b60,
            fn_006f4b60(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006f4b90, fn_006f4b90(Ptr<BSSimpleArray>)),
        entry!(0x006f4bb0, fn_006f4bb0(Ptr, u32) -> Ptr),
        entry!(
            0x006f4be0,
            bs_simple_array_parent_space_node_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006f4c10,
            bs_simple_array_teleport_link_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(0x006f4c40, fn_006f4c40(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x009d9ee0, fn_009d9ee0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn constructor_builds_both_arrays_and_points() {
        let mut e = Engine::new();
        e.register(SPACES_INITIALIZE, nothing);
        e.register(POINT_CONSTRUCT, nothing);
        let this: Ptr<TeleportPath> = e.new_object();
        for off in [0x14u32, 0x18, 0x1c] {
            e.mem.set_u32(this.addr() + off, 0x99);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f48b0, &args![this]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr()), SPACES_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x10), DOORS_VTABLE);
        for off in [0x14u32, 0x18, 0x1c] {
            assert_eq!(e.mem.u32(this.addr() + off), 0);
        }
        assert_eq!(
            calls(&log, SPACES_INITIALIZE),
            vec![vec![this.addr(), 0, 0]]
        );
        assert_eq!(
            calls(&log, POINT_CONSTRUCT),
            vec![vec![this.addr() + 0x20], vec![this.addr() + 0x2c]]
        );
    }

    #[test]
    fn destructor_destroys_doors_then_spaces() {
        let mut e = Engine::new();
        e.register(ARRAY_008454F0, nothing);
        let this: Ptr<TeleportPath> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006f4930, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, ARRAY_008454F0),
            vec![vec![this.addr() + 0x10, 1], vec![this.addr(), 1]]
        );
        assert_eq!(e.mem.u32(this.addr()), SPACES_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x10), DOORS_VTABLE);
    }

    #[test]
    fn clear_empties_spaces_then_doors() {
        let mut e = Engine::new();
        e.register(ARRAY_008454F0, nothing);
        let this: Ptr<TeleportPath> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006f4990, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, ARRAY_008454F0),
            vec![vec![this.addr(), 1], vec![this.addr() + 0x10, 1]]
        );
    }

    /// Doubles for the point and array helpers `ComputeLength` uses; the
    /// door references' slot +0x1F4 returns the reference + 0x10.
    fn length_engine() -> Engine {
        let mut e = Engine::new();
        e.register(IS_EMPTY, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ELEMENT_AT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x10).into_ret()
        });
        e.register(FIRST_ELEMENT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LAST_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + (e.mem.u32(a[0] + 8) - 1) * 0x10).into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) - e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_LENGTH, |e, a| {
            let (x, y, z) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4), e.mem.f32(a[0] + 8));
            ((x as f64 * x as f64 + y as f64 * y as f64 + z as f64 * z as f64).sqrt()).into_ret()
        });
        e.register(0x0f00_01f4, |_, a| (a[0] + 0x10).into_ret());
        e.put_vtable(0x0f10_0000, &[0x0f00_01f4; 0x7e]);
        e.map(0x0101_2000, 0x1000);
        e.set_global(NO_PATH_LENGTH, -1.0f32);
        e
    }

    fn set_point(e: &mut Engine, at: u32, p: [f32; 3]) {
        for (k, v) in p.iter().enumerate() {
            e.mem.set_f32(at + 4 * k as u32, *v);
        }
    }

    /// A door reference standing at `p`.
    fn door(e: &mut Engine, p: [f32; 3]) -> u32 {
        let door = e.mem.alloc(0x20);
        e.mem.set_u32(door, 0x0f10_0000);
        set_point(e, door + 0x10, p);
        door
    }

    #[test]
    fn length_is_minus_one_without_spaces() {
        let mut e = length_engine();
        let this: Ptr<TeleportPath> = e.new_object();
        assert_eq!(e.call(0x006f49c0, &args![this]).f64(), -1.0);
    }

    #[test]
    fn length_without_doors_is_start_to_end() {
        let mut e = length_engine();
        let this: Ptr<TeleportPath> = e.new_object();
        e.mem.set_u32(this.addr() + 8, 1);
        set_point(&mut e, this.addr() + 0x20, [1.0, 1.0, 1.0]);
        set_point(&mut e, this.addr() + 0x2c, [4.0, 5.0, 1.0]);
        assert_eq!(e.call(0x006f49c0, &args![this]).f64(), 5.0);
        // The unrounded `ST0` of `00457990` comes back as it is.
        set_point(&mut e, this.addr() + 0x2c, [2.0, 2.0, 1.0]);
        assert_eq!(e.call(0x006f49c0, &args![this]).f64(), 2f64.sqrt());
    }

    #[test]
    fn length_sums_the_legs_through_the_doors() {
        let mut e = length_engine();
        let this: Ptr<TeleportPath> = e.new_object();
        e.mem.set_u32(this.addr() + 8, 1);
        set_point(&mut e, this.addr() + 0x20, [0.0, 0.0, 0.0]);
        set_point(&mut e, this.addr() + 0x2c, [100.0, 0.0, 0.0]);
        let links = e.mem.alloc(0x20);
        // Door 0 at x=3 teleports to x=50; door 1 at x=54 teleports to 90.
        let first = door(&mut e, [3.0, 4.0, 0.0]);
        e.mem.set_u32(links, first);
        set_point(&mut e, links + 4, [50.0, 0.0, 0.0]);
        let second = door(&mut e, [54.0, 3.0, 0.0]);
        e.mem.set_u32(links + 0x10, second);
        set_point(&mut e, links + 0x14, [90.0, 0.0, 0.0]);
        e.mem.set_u32(this.addr() + 0x14, links);
        e.mem.set_u32(this.addr() + 0x18, 2);
        // 5 (start to door 0) + 5 (x=50 to door 1) + 10 (x=90 to end).
        assert_eq!(e.call(0x006f49c0, &args![this]).f64(), 20.0);
        // The sum is a `float`.
        set_point(&mut e, links + 0x14, [99.9, 0.0, 0.0]);
        let leg = (100.0f32 - 99.9f32) as f64;
        let expected = ((10.0 + leg) as f32) as f64;
        assert_eq!(e.call(0x006f49c0, &args![this]).f64(), expected);
    }

    #[test]
    fn array_constructors_and_destructors_store_their_vtables() {
        let mut e = Engine::new();
        e.register(SPACES_INITIALIZE, nothing);
        e.register(ARRAY_008454F0, nothing);
        let a: Ptr<BSSimpleArray> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f4b10, &args![a]).u32(), a.addr());
        assert_eq!(e.mem.u32(a.addr()), SPACES_VTABLE);
        e.mem.set_u32(a.addr(), 0);
        e.call(0x006f4b40, &args![a]);
        assert_eq!(e.mem.u32(a.addr()), SPACES_VTABLE);
        assert_eq!(e.call(0x006f4b60, &args![a]).u32(), a.addr());
        assert_eq!(e.mem.u32(a.addr()), DOORS_VTABLE);
        e.mem.set_u32(a.addr(), 0);
        e.call(0x006f4b90, &args![a]);
        assert_eq!(e.mem.u32(a.addr()), DOORS_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SPACES_INITIALIZE), vec![vec![a.addr(), 0, 0]]);
        assert_eq!(
            calls(&log, ARRAY_008454F0),
            vec![vec![a.addr(), 1], vec![a.addr(), 1]]
        );
    }

    #[test]
    fn allocation_asks_for_16_bytes_per_link() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (0x8000 + a[0]).into_ret());
        assert_eq!(e.call(0x006f4bb0, &args![0u32, 5u32]).u32(), 0x8050);
    }

    #[test]
    fn scalar_deleting_destructors_free_only_with_bit_0() {
        for (addr, vtable) in [(0x006f4be0u32, SPACES_VTABLE), (0x006f4c10, DOORS_VTABLE)] {
            let mut e = Engine::new();
            e.register(ARRAY_008454F0, nothing);
            e.register(OPERATOR_DELETE, nothing);
            let a: Ptr<BSSimpleArray> = e.new_object();
            for (flags, frees) in [(0u32, 0usize), (1, 1), (2, 0)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(addr, &args![a, flags]).u32(), a.addr());
                let log = e.call_log.take().unwrap();
                assert_eq!(calls(&log, ARRAY_008454F0), vec![vec![a.addr(), 1]]);
                assert_eq!(calls(&log, OPERATOR_DELETE).len(), frees);
                assert_eq!(e.mem.u32(a.addr()), vtable);
            }
        }
    }

    #[test]
    fn initializer_reserves_the_larger_and_constructs_count() {
        for (capacity, count, reserved) in [(0u32, 0u32, 0u32), (4, 0, 4), (2, 5, 5), (7, 3, 7)] {
            let mut e = Engine::new();
            e.register(0x0f00_0004, |_, a| (0x6000 + a[1]).into_ret());
            e.put_vtable(0x0f20_0000, &[0, 0x0f00_0004]);
            e.register(DOORS_CONSTRUCT_ELEMENTS, nothing);
            let a: Ptr<BSSimpleArray> = e.new_object();
            e.mem.set_u32(a.addr(), 0x0f20_0000);
            for off in [4u32, 8, 0xc] {
                e.mem.set_u32(a.addr() + off, 0x99);
            }
            e.call_log = Some(vec![]);
            e.call(0x006f4c40, &args![a, capacity, count]);
            let log = e.call_log.take().unwrap();
            let buffer = if reserved == 0 { 0 } else { 0x6000 + reserved };
            assert_eq!(e.get(a, BSSimpleArray::pBuffer), buffer);
            assert_eq!(e.get(a, BSSimpleArray::iReservedSize), reserved);
            assert_eq!(e.get(a, BSSimpleArray::iSize), count);
            assert_eq!(calls(&log, 0x0f00_0004).len(), (reserved != 0) as usize);
            if count == 0 {
                assert!(calls(&log, DOORS_CONSTRUCT_ELEMENTS).is_empty());
            } else {
                assert_eq!(
                    calls(&log, DOORS_CONSTRUCT_ELEMENTS),
                    vec![vec![a.addr(), buffer, count]]
                );
            }
        }
    }

    #[test]
    fn folded_body_passes_the_member_at_b4() {
        let mut e = Engine::new();
        e.register(FN_0069CFE0, nothing);
        e.call_log = Some(vec![]);
        e.call(0x009d9ee0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FN_0069CFE0), vec![vec![0x2000, 0x10b4]]);
    }
}
