//! `fallout/ai/run_009b7a20` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `009bbef0` (`thiscall`, `RET 0x20`; unit `fallout/ai/combat/projectile.cpp`): the base
/// constructor `009b7af0` runs first.
const BASE_CONSTRUCTOR: u32 = 0x009b_bef0;
/// `009bc470` (`thiscall`, no stack word; unit `fallout/ai/combat/projectile.cpp`): the base
/// destructor `009b7bc0` runs last.
const BASE_DESTRUCTOR: u32 = 0x009b_c470;
/// `004f15a0` (`thiscall`, `RET 4`).
const SET_KIND: u32 = 0x004f_15a0;
/// The class's two vtables (+0 and +0x18), set by the constructor and the destructor.
const VTABLE: u32 = 0x0108_fa44;
const SECOND_VTABLE: u32 = 0x0108_fa38;
/// `004181e0` (`thiscall`, no stack word; unit `fallout shared/extradatalist.cpp`): the
/// object the next call is made on.
const BASE_OBJECT: u32 = 0x0041_81e0;
/// `0043fcd0` (`thiscall`, no stack word; unit `fallout shared/modelloader.cpp`).
const GET_NODE: u32 = 0x0043_fcd0;
/// `009b7cb0` (no arguments): returns the word at `011c6278`.
const GET_GLOBAL_KEY: u32 = 0x009b_7cb0;
/// `009611e0` (`thiscall`, no stack word; unit `fallout/ai/playercharacter.cpp`).
const GET_OWNER: u32 = 0x0096_11e0;
/// `004fd420` (`thiscall`, `RET 4`).
const HAS_FLAG: u32 = 0x004f_d420;

// Translated from 009b7a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor. Runs
/// the destructor [`fn_009b7bc0`] and, for bit 0 of `flags`, `operator delete` on `this`.
/// Returns `this`.
pub fn fn_009b7a20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009b7bc0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009b7af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b7af0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1ad08, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0x20`): a constructor. Runs the base
/// constructor `009bbef0` with the eight words as they came (three words, three words passed
/// by value, two floats copied through the x87 stack), sets the vtables `0108fa44` (+0) and
/// `0108fa38` (+0x18), calls `004f15a0(0x3d)`, clears the word at +0x150 and the byte at
/// +0x154, and sets the floats at +0x158 and +0x15c to 0.0. Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_009b7af0(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    point_x: u32,
    point_y: u32,
    point_z: u32,
    first_float: f32,
    second_float: f32,
) -> Ptr {
    e.call(
        BASE_CONSTRUCTOR,
        &args![
            this,
            first,
            second,
            third,
            point_x,
            point_y,
            point_z,
            x87_quiet_bits(first_float.to_bits()),
            x87_quiet_bits(second_float.to_bits())
        ],
    );
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base + 0x18, SECOND_VTABLE);
    e.call(SET_KIND, &args![this, 0x3du32]);
    e.mem.set_u32(base + 0x150, 0);
    e.mem.set_u8(base + 0x154, 0);
    e.mem.set_u32(base + 0x158, 0.0f32.to_bits());
    e.mem.set_u32(base + 0x15c, 0.0f32.to_bits());
    this
}

// Translated from 009b7bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b7bc0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1add8, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): the destructor. Sets the vtables
/// `0108fa44` (+0) and `0108fa38` (+0x18); when [`fn_009b7c90`] on the base object
/// (`004181e0`) answers nonzero and `0043fcd0` gives a node, asks the node (vtable slot
/// +0x9c) for the entry keyed by the word `009b7cb0` returns; with an entry, calls vtable
/// slot +0xe8 of the entry's owner (`009611e0`) with the entry. Last, the base destructor
/// `009bc470`.
pub fn fn_009b7bc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base + 0x18, SECOND_VTABLE);
    let object = e.call(BASE_OBJECT, &args![this]).u32();
    if fn_009b7c90(e, Ptr::new(object)) as u8 != 0 && e.call(GET_NODE, &args![this]).u32() != 0 {
        let node = e.call(GET_NODE, &args![this]).u32();
        let key = e.call(GET_GLOBAL_KEY, &args![]).u32();
        let entry = e.vcall(node, 0x9c, &args![key]).u32();
        if entry != 0 {
            let owner = e.call(GET_OWNER, &args![entry]).u32();
            e.vcall(owner, 0xe8, &args![entry]);
        }
    }
    e.call(BASE_DESTRUCTOR, &args![this]);
}

// Translated from 009b7c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `004fd420(0x100)` on `this`; its
/// `EAX` is passed on (callers test the low byte).
pub fn fn_009b7c90(e: &mut Engine, this: Ptr) -> u32 {
    e.call(HAS_FLAG, &args![this, 0x100u32]).u32()
}

/// `0044ddc0` (`thiscall` on `011f2250`, no stack word).
const GLOBAL_STATE: u32 = 0x0044_ddc0;
/// The object `0044ddc0` is called on (`011f2250`).
const STATE_OWNER: u32 = 0x011f_2250;
/// `004fdf60` (`thiscall`, no stack word; unit `fallout shared/bgsprojectile.cpp`).
const FIRST_TEST: u32 = 0x004f_df60;
/// `009a7f80` (`thiscall`, no stack word; unit `fallout/ai/combat/combatutilities.cpp`).
const SECOND_TEST: u32 = 0x009a_7f80;
/// `00966980` (`thiscall`, no stack word; returns a float in ST0).
const AMOUNT: u32 = 0x0096_6980;
/// `0.0` (`double`) the amount is compared with.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `005de0a0` (`thiscall`, `RET 8`: a flag mask and a value; unit
/// `fallout shared/tesscriptfunctions.cpp`).
const SET_FLAG: u32 = 0x005d_e0a0;
/// `009bda10` (`thiscall`, no stack word; unit `fallout/ai/combat/projectile.cpp`).
const AFTER_FLAGS: u32 = 0x009b_da10;

// Translated from 009b7cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): sets nine flags through
/// `005de0a0(mask, value)`. With `first` the low byte `004fdf60` gives on the base object
/// (`004181e0`), `second` 0 when `0044ddc0` on `011f2250` answers 4 and otherwise the low
/// byte of `009a7f80` on the base object, and `positive` whether the amount `00966980`
/// gives on the base object is above the `double` at `01012060` (ordered): mask 0x10 gets
/// `first`, 1 and 8 get `second`, 0x20 `first && second`, 2 `!first && second`, 0x40
/// `positive && !second`, 0x100 gets 1, 0x8000 `second == 0`, 0x2000 `second`. Then
/// `009bda10`.
pub fn fn_009b7cc0(e: &mut Engine, this: Ptr) {
    let in_state_four = e.call(GLOBAL_STATE, &args![STATE_OWNER]).u32() == 4;
    let object = e.call(BASE_OBJECT, &args![this]).u32();
    let first = e.call(FIRST_TEST, &args![object]).u8();
    let second = if in_state_four {
        0u8
    } else {
        let object = e.call(BASE_OBJECT, &args![this]).u32();
        e.call(SECOND_TEST, &args![object]).u8()
    };
    let object = e.call(BASE_OBJECT, &args![this]).u32();
    let amount = e.call(AMOUNT, &args![object]).f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    let positive = amount > zero;
    let flags: [(u32, u32); 9] = [
        (0x10, first as u32),
        (1, second as u32),
        (8, second as u32),
        (0x20, (first != 0 && second != 0) as u32),
        (2, (first == 0 && second != 0) as u32),
        (0x40, (positive && second == 0) as u32),
        (0x100, 1),
        (0x8000, (second == 0) as u32),
        (0x2000, second as u32),
    ];
    for (mask, value) in flags {
        e.call(SET_FLAG, &args![this, mask, value]);
    }
    e.call(AFTER_FLAGS, &args![this]);
}

/// `0043b300` (`cdecl`, two words; unit `fallout shared/modelloader.cpp`): tests the node
/// against the word given (`011f4428`).
const NODE_TEST: u32 = 0x0043_b300;
const NODE_TEST_KEY: u32 = 0x011f_4428;
/// `00450f90` (`thiscall`, `RET 4`; unit `fallout shared/tes.cpp`).
const NODE_SET: u32 = 0x0045_0f90;
/// `006a78f0` (`thiscall`, no stack word; returns a float in ST0).
const BASE_AMOUNT: u32 = 0x006a_78f0;
/// `0045bc00` (`thiscall`, `RET 4`; unit `fallout shared/tes.cpp`).
const NODE_CHILD_PRESENT: u32 = 0x0045_bc00;
/// `00444d00` (`thiscall`, `RET 4`; unit `fallout shared/modelloader.cpp`): a flag test
/// (low byte of `EAX`).
const TEST_FLAG: u32 = 0x0044_4d00;
/// `0043b4a0` (`thiscall`, `RET 4`; unit `fallout shared/modelloader.cpp`).
const NODE_CHILD: u32 = 0x0043_b4a0;
/// `0043d450` (`thiscall`, no stack word; engine map `NiAVObject::GetWorldBound`): the
/// address of the node's four-word bound.
const WORLD_BOUND: u32 = 0x0043_d450;
/// `006815c0` (`thiscall`, no stack word).
const BOUND_CENTER: u32 = 0x0068_15c0;
/// `0043c490` (`thiscall`, no stack word; unit `fallout shared/modelloader.cpp`).
const NODE_TRANSFORM: u32 = 0x0043_c490;
/// `00439ef0` (`thiscall`, `RET 8`; unit `fallout shared/multiboundmarkerdata.cpp`).
const TRANSFORM_APPLY: u32 = 0x0043_9ef0;
/// `00457990` (`thiscall`, no stack word; returns a float in ST0).
const LOCAL_LENGTH: u32 = 0x0045_7990;
/// `0084d030` (`thiscall`, no stack word; returns a float in ST0).
const BOUND_RADIUS: u32 = 0x0084_d030;
/// `009669c0` (`thiscall`, no stack word; returns a float in ST0).
const DIVISOR: u32 = 0x0096_69c0;
/// `00416870` (`thiscall`, `RET 0xc`: three floats): builds a point and returns its address.
const MAKE_POINT: u32 = 0x0041_6870;
/// `009bf370` (`thiscall`, `RET 0x10`: a float and a point by value).
const SET_SCALED_POINT: u32 = 0x009b_f370;
/// `009bf470` (`thiscall`, `RET 8`; engine map `Projectile::Sync3DWithReference`).
const SYNC_WITH_REFERENCE: u32 = 0x009b_f470;
/// `0043d410` (`thiscall`, `RET 0xc`: a float and two words): builds the 0xc-byte local
/// passed to the node.
const MAKE_UPDATE: u32 = 0x0043_d410;
/// `00508070` and `00979260` (`thiscall`, no stack word; floats in ST0): compared at the
/// end.
const LIMIT: u32 = 0x0050_8070;
const ELAPSED: u32 = 0x0097_9260;

// Translated from 009b7e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word). Gets a node from vtable slot
/// +0x1d0; with a node that passes `0043b300(011f4428, node)`: `00450f90(0)` on it; when the
/// amount `006a78f0` gives on the base object (`004181e0`) is above the `double` at
/// `01012060` (ordered), for child 0 and child 1 present (`0045bc00`) sets the child
/// (`0043b4a0`, then `00450f90`) to `00444d00(0x10)` (child 0) or its negation (child 1).
/// Then, when `00444d00(0x10)` answers nonzero: copies the node's world bound (`0043d450`,
/// four words), applies the node's transform (`0043c490`, `00439ef0` with a local and the
/// bound's centre `006815c0`), computes `size = (float)(radius + length)` from `0084d030` on
/// the bound and `00457990` on the local (x87, rounded to `float`) and
/// `scale = (float)(size / 009669c0())`, calls `009bf370(scale, point(0, size, 0))` (the
/// point from `00416870`), `009bf470(0, 1)`, builds a local with `0043d410(0.0, 0, 0)` and
/// passes it to the node's vtable slot +0xb8; last, when the value `00979260` gives is below
/// the one `00508070` gives (ordered), `00450f90(1)` on the node.
pub fn fn_009b7e40(e: &mut Engine, this: Ptr) {
    let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    if node == 0 {
        return;
    }
    if e.call(NODE_TEST, &args![NODE_TEST_KEY, node]).u8() == 0 {
        return;
    }
    e.call(NODE_SET, &args![node, 0u32]);
    let object = e.call(BASE_OBJECT, &args![this]).u32();
    let amount = e.call(BASE_AMOUNT, &args![object]).f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    if amount > zero {
        if e.call(NODE_CHILD_PRESENT, &args![node, 0u32]).u32() != 0 {
            let value = e.call(TEST_FLAG, &args![this, 0x10u32]).u8() as u32;
            let child = e.call(NODE_CHILD, &args![node, 0u32]).u32();
            e.call(NODE_SET, &args![child, value]);
        }
        if e.call(NODE_CHILD_PRESENT, &args![node, 1u32]).u32() != 0 {
            let value = (e.call(TEST_FLAG, &args![this, 0x10u32]).u8() == 0) as u32;
            let child = e.call(NODE_CHILD, &args![node, 1u32]).u32();
            e.call(NODE_SET, &args![child, value]);
        }
    }
    if e.call(TEST_FLAG, &args![this, 0x10u32]).u8() == 0 {
        return;
    }
    // The frame below `EBP`: offsets are `0x58 - k` for the exe's `[EBP - k]`.
    e.with_stack(0x58, |e, frame| {
        let at = |k: u32| frame.addr() + 0x58 - k;
        let bound = e.call(WORLD_BOUND, &args![node]).u32();
        for word in 0..4u32 {
            let value = e.mem.u32(bound + 4 * word);
            e.mem.set_u32(at(0x1c) + 4 * word, value);
        }
        let center = e.call(BOUND_CENTER, &args![at(0x1c)]).u32();
        let transform = e.call(NODE_TRANSFORM, &args![node]).u32();
        e.call(TRANSFORM_APPLY, &args![transform, at(0x28), center]);
        let length = e.call(LOCAL_LENGTH, &args![at(0x28)]).f64();
        let radius = e.call(BOUND_RADIUS, &args![at(0x1c)]).f64();
        let size = x87_store32(x87_add(radius, length));
        let divisor = e.call(DIVISOR, &args![this]).f64();
        let scale = x87_store32(x87_div(x87_m32(size), divisor));
        let point = e.call(MAKE_POINT, &args![at(0x44), 0u32, size, 0u32]).u32();
        let point_x = e.mem.u32(point);
        let point_y = e.mem.u32(point + 4);
        let point_z = e.mem.u32(point + 8);
        e.call(
            SET_SCALED_POINT,
            &args![this, scale, point_x, point_y, point_z],
        );
        e.call(SYNC_WITH_REFERENCE, &args![this, 0u32, 1u32]);
        e.call(MAKE_UPDATE, &args![at(0x38), 0u32, 0u32, 0u32]);
        e.vcall(node, 0xb8, &args![at(0x38)]);
    });
    let limit = e.call(LIMIT, &args![this]).f64();
    let elapsed = e.call(ELAPSED, &args![this]).f64();
    if elapsed < limit {
        e.call(NODE_SET, &args![node, 1u32]);
    }
}

/// `Projectile::Projectile` (`009bbc70`, Xbox PDB): `thiscall`, one word (`RET 4`).
const PROJECTILE_CONSTRUCT: u32 = 0x009b_bc70;
/// The word `fn_009b7cb0` returns (`0x011c6278`).
const GLOBAL_KEY: u32 = 0x011c_6278;

// Translated from 009b7a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b7a50 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 4`): the other constructor of this class.
/// `Projectile::Projectile` (`009bbc70`) with the byte `flag`, the vtables `0108fa44`
/// (+0) and `0108fa38` (+0x18), `004f15a0(0x3d)`, the word at +0x150 and the byte at
/// +0x154 cleared and the floats at +0x158 and +0x15c set to 0.0. Returns `this`.
pub fn fn_009b7a50(e: &mut Engine, this: Ptr, flag: u8) -> Ptr {
    let p = this.addr();
    e.call(PROJECTILE_CONSTRUCT, &args![this, u32::from(flag)]);
    e.mem.set_u32(p, VTABLE);
    e.mem.set_u32(p + 0x18, SECOND_VTABLE);
    e.call(SET_KIND, &args![this, 0x3du32]);
    e.mem.set_u32(p + 0x150, 0);
    e.mem.set_u8(p + 0x154, 0);
    e.mem.set_u32(p + 0x158, 0.0f32.to_bits());
    e.mem.set_u32(p + 0x15c, 0.0f32.to_bits());
    this
}

// Translated from 009b7cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words): the word at `0x011c6278`.
pub fn fn_009b7cb0(e: &mut Engine) -> u32 {
    e.mem.u32(GLOBAL_KEY)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009b_7a50, fn_009b7a50(Ptr, u8) -> Ptr),
        entry!(0x009b_7cb0, fn_009b7cb0() -> u32),
        entry!(0x009b_7a20, fn_009b7a20(Ptr, u32) -> Ptr),
        entry!(
            0x009b_7af0,
            fn_009b7af0(Ptr, u32, u32, u32, u32, u32, u32, f32, f32) -> Ptr
        ),
        entry!(0x009b_7bc0, fn_009b7bc0(Ptr)),
        entry!(0x009b_7c90, fn_009b7c90(Ptr) -> u32),
        entry!(0x009b_7cc0, fn_009b7cc0(Ptr)),
        entry!(0x009b_7e40, fn_009b7e40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn constructor_passes_the_words_and_initializes_the_fields() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(SET_KIND, |_, _| Ret::default());
        let this = e.mem.alloc(0x160);
        e.mem.set_u8(this + 0x154, 9);
        e.mem.set_u32(this + 0x158, 0xffff_ffff);
        e.call_log = Some(vec![]);
        let words = args![
            this,
            1u32,
            2u32,
            3u32,
            4u32,
            5u32,
            6u32,
            0x7f80_0001u32,
            2.5f32
        ];
        assert_eq!(e.call(0x009b_7af0, &words).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, BASE_CONSTRUCTOR),
            vec![vec![this, 1, 2, 3, 4, 5, 6, 0x7fc0_0001, 2.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, SET_KIND), vec![vec![this, 0x3d]]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), SECOND_VTABLE);
        assert_eq!(e.mem.u8(this + 0x154), 0);
        assert_eq!(e.mem.u32(this + 0x158), 0);
    }

    fn destructor_fixture(flag: u32, node: u32) -> Engine {
        let mut e = Engine::new();
        e.register(BASE_OBJECT, |_, _| 0x5000u32.into_ret());
        e.register_double(HAS_FLAG, move |_, _| flag.into_ret());
        e.register_double(GET_NODE, move |_, _| node.into_ret());
        e.register(GET_GLOBAL_KEY, |_, _| 0x66u32.into_ret());
        e.register(GET_OWNER, |_, _| 0x0150_2000u32.into_ret());
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0; 0x40]);
        e.mem.set_u32(0x0150_009c, 0x0150_1000);
        e.register(0x0150_1000, |_, _| 0x0150_2000u32.into_ret());
        e.map(0x0150_2000, 0x10);
        e.mem.set_u32(0x0150_2000, 0x0150_0000);
        e.mem.set_u32(0x0150_00e8, 0x0150_1100);
        e.register(0x0150_1100, |_, _| Ret::default());
        e
    }

    #[test]
    fn destructor_hands_the_entry_to_its_owner_then_runs_the_base() {
        let mut e = destructor_fixture(0x0000_0101, 0x0150_2000);
        let this = e.mem.alloc(0x160);
        e.call_log = Some(vec![]);
        e.call(0x009b_7bc0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, HAS_FLAG), vec![vec![0x5000, 0x100]]);
        assert_eq!(calls_to(&log, 0x0150_1000), vec![vec![0x0150_2000, 0x66]]);
        assert_eq!(
            calls_to(&log, 0x0150_1100),
            vec![vec![0x0150_2000, 0x0150_2000]]
        );
        assert_eq!(calls_to(&log, BASE_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), SECOND_VTABLE);
    }

    #[test]
    fn destructor_skips_the_entry_when_the_flag_byte_is_clear_or_no_node() {
        // Only the low byte counts: 0x100 is "clear".
        let mut e = destructor_fixture(0x100, 0x0150_2000);
        let this = e.mem.alloc(0x160);
        e.call_log = Some(vec![]);
        e.call(0x009b_7bc0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, GET_NODE).is_empty());
        assert_eq!(calls_to(&log, BASE_DESTRUCTOR).len(), 1);
        let mut e = destructor_fixture(1, 0);
        let this = e.mem.alloc(0x160);
        e.call_log = Some(vec![]);
        e.call(0x009b_7bc0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, GET_NODE).len(), 1);
        assert!(calls_to(&log, 0x0150_1000).is_empty());
    }

    #[test]
    fn deleting_destructor_deletes_on_bit_0() {
        let mut e = destructor_fixture(0, 0);
        let this = e.mem.alloc(0x160);
        for flags in [1u32, 0] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009b_7a20, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, BASE_DESTRUCTOR).len(), 1);
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), flags as usize);
        }
    }

    #[test]
    fn fn_009b7c90_passes_the_flag_answer_on() {
        let mut e = Engine::new();
        e.register(HAS_FLAG, |_, _| 0x1234_5601u32.into_ret());
        assert_eq!(e.call(0x009b_7c90, &args![0x40u32]).u32(), 0x1234_5601);
    }

    fn flag_values(state: u32, first: u32, second: u32, amount: f64) -> Vec<(u32, u32)> {
        let mut e = Engine::new();
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.register_double(GLOBAL_STATE, move |_, _| state.into_ret());
        e.register(BASE_OBJECT, |_, _| 0x5000u32.into_ret());
        e.register_double(FIRST_TEST, move |_, _| first.into_ret());
        e.register_double(SECOND_TEST, move |_, _| second.into_ret());
        e.register_double(AMOUNT, move |_, _| amount.into_ret());
        e.register(SET_FLAG, |_, _| Ret::default());
        e.register(AFTER_FLAGS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009b_7cc0, &args![0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, AFTER_FLAGS), vec![vec![0x7000]]);
        calls_to(&log, SET_FLAG)
            .into_iter()
            .map(|words| (words[1], words[2]))
            .collect()
    }

    #[test]
    fn fn_009b7cc0_sets_the_nine_flags() {
        assert_eq!(
            flag_values(0, 2, 1, 1.0),
            vec![
                (0x10, 2),
                (1, 1),
                (8, 1),
                (0x20, 1),
                (2, 0),
                (0x40, 0),
                (0x100, 1),
                (0x8000, 0),
                (0x2000, 1),
            ]
        );
        // State 4: the second test is not asked and counts as 0.
        assert_eq!(
            flag_values(4, 0, 1, 1.0),
            vec![
                (0x10, 0),
                (1, 0),
                (8, 0),
                (0x20, 0),
                (2, 0),
                (0x40, 1),
                (0x100, 1),
                (0x8000, 1),
                (0x2000, 0),
            ]
        );
        // A zero or NaN amount does not count as positive.
        assert_eq!(flag_values(4, 0, 0, 0.0)[5], (0x40, 0));
        assert_eq!(flag_values(4, 0, 0, f64::NAN)[5], (0x40, 0));
        // Only the low byte of the tests counts.
        assert_eq!(flag_values(0, 0x100, 0x200, 1.0)[3], (0x20, 0));
    }

    struct Update {
        node_ok: u32,
        amount: f64,
        flag: u32,
        limit: f64,
        elapsed: f64,
    }

    fn run_update(update: Update) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        let this = e.mem.alloc(0x200);
        e.put_vtable(0x0150_0000, &[0; 0x80]);
        e.mem.set_u32(this, 0x0150_0000);
        e.mem.set_u32(0x0150_01d0, 0x0150_1000);
        e.register(0x0150_1000, |_, _| 0x0150_2000u32.into_ret());
        e.map(0x0150_2000, 0x10);
        e.mem.set_u32(0x0150_2000, 0x0150_0000);
        e.mem.set_u32(0x0150_00b8, 0x0150_1100);
        e.register(0x0150_1100, |_, _| Ret::default());
        let node_ok = update.node_ok;
        e.register_double(NODE_TEST, move |_, _| node_ok.into_ret());
        e.register(NODE_SET, |_, _| Ret::default());
        e.register(BASE_OBJECT, |_, _| 0x5000u32.into_ret());
        let amount = update.amount;
        e.register_double(BASE_AMOUNT, move |_, _| amount.into_ret());
        e.register(NODE_CHILD_PRESENT, |_, a| (a[1] + 1).into_ret());
        let flag = update.flag;
        e.register_double(TEST_FLAG, move |_, _| flag.into_ret());
        e.register(NODE_CHILD, |_, a| (0x6000 + a[1]).into_ret());
        let bound = e.mem.alloc(0x10);
        for word in 0..4u32 {
            e.mem.set_u32(bound + 4 * word, 0x100 + word);
        }
        e.register_double(WORLD_BOUND, move |_, _| bound.into_ret());
        e.register(BOUND_CENTER, |_, _| 0x7100u32.into_ret());
        e.register(NODE_TRANSFORM, |_, _| 0x7200u32.into_ret());
        e.register(TRANSFORM_APPLY, |_, _| Ret::default());
        e.register(LOCAL_LENGTH, |_, _| 1.5f64.into_ret());
        e.register(BOUND_RADIUS, |e, a| {
            assert_eq!(e.mem.u32(a[0] + 0xc), 0x103);
            2.0f64.into_ret()
        });
        e.register(DIVISOR, |_, _| 2.0f64.into_ret());
        let point = e.mem.alloc(0xc);
        e.register_double(MAKE_POINT, move |e, a| {
            e.mem.set_u32(point, a[1]);
            e.mem.set_u32(point + 4, a[2]);
            e.mem.set_u32(point + 8, a[3]);
            point.into_ret()
        });
        e.register(SET_SCALED_POINT, |_, _| Ret::default());
        e.register(SYNC_WITH_REFERENCE, |_, _| Ret::default());
        e.register(MAKE_UPDATE, |_, _| Ret::default());
        let (limit, elapsed) = (update.limit, update.elapsed);
        e.register_double(LIMIT, move |_, _| limit.into_ret());
        e.register_double(ELAPSED, move |_, _| elapsed.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009b_7e40, &args![this]);
        let mut log = e.call_log.take().unwrap();
        log.retain(|call| call.0 != 0x009b_7e40);
        log
    }

    #[test]
    fn fn_009b7e40_updates_the_node_children_and_scale() {
        let log = run_update(Update {
            node_ok: 1,
            amount: 1.0,
            flag: 1,
            limit: 3.0,
            elapsed: 2.0,
        });
        assert_eq!(
            calls_to(&log, NODE_SET),
            vec![
                vec![0x0150_2000, 0],
                vec![0x6000, 1],
                vec![0x6001, 0],
                vec![0x0150_2000, 1]
            ]
        );
        assert_eq!(
            calls_to(&log, NODE_TEST),
            vec![vec![NODE_TEST_KEY, 0x0150_2000]]
        );
        // size = 2.0 + 1.5, scale = 3.5 / 2.0.
        assert_eq!(
            calls_to(&log, SET_SCALED_POINT)[0][1..],
            [1.75f32.to_bits(), 0, 3.5f32.to_bits(), 0]
        );
        assert_eq!(calls_to(&log, SYNC_WITH_REFERENCE)[0][1..], [0, 1]);
        assert_eq!(calls_to(&log, MAKE_UPDATE)[0][1..], [0, 0, 0]);
        let local = calls_to(&log, MAKE_UPDATE)[0][0];
        assert_eq!(calls_to(&log, 0x0150_1100), vec![vec![0x0150_2000, local]]);
        assert_eq!(calls_to(&log, TRANSFORM_APPLY)[0][0], 0x7200);
        assert_eq!(calls_to(&log, TRANSFORM_APPLY)[0][2], 0x7100);
    }

    #[test]
    fn fn_009b7e40_stops_early_on_each_test() {
        let base = || Update {
            node_ok: 1,
            amount: 0.0,
            flag: 0,
            limit: 1.0,
            elapsed: 1.0,
        };
        // The node test fails: nothing set.
        let log = run_update(Update {
            node_ok: 0x100,
            ..base()
        });
        assert!(calls_to(&log, NODE_SET).is_empty());
        // No positive amount, flag clear: only the first set, no children.
        let log = run_update(base());
        assert_eq!(calls_to(&log, NODE_SET), vec![vec![0x0150_2000, 0]]);
        assert!(calls_to(&log, NODE_CHILD_PRESENT).is_empty());
        assert!(calls_to(&log, WORLD_BOUND).is_empty());
        // Flag set, elapsed equal to the limit: no final set.
        let log = run_update(Update { flag: 1, ..base() });
        assert_eq!(calls_to(&log, NODE_SET), vec![vec![0x0150_2000, 0]]);
        assert_eq!(calls_to(&log, SET_SCALED_POINT).len(), 1);
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

    #[test]
    fn fn_009b7a50_constructs_with_a_flag() {
        let mut e = Engine::new();
        e.register(PROJECTILE_CONSTRUCT, |_, _| Ret::default());
        e.register(SET_KIND, |_, _| Ret::default());
        let this = e.mem.alloc(0x160);
        e.mem.write(this + 0x150, &[0xcd; 0x10]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_7a50, &args![this, 0x301u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let order: Vec<_> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (PROJECTILE_CONSTRUCT, vec![this, 1]),
                (SET_KIND, vec![this, 0x3d])
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), SECOND_VTABLE);
        assert_eq!(e.mem.u32(this + 0x150), 0);
        assert_eq!(e.mem.u8(this + 0x154), 0);
        assert_eq!(e.mem.u8(this + 0x155), 0xcd);
        assert_eq!(e.mem.u64(this + 0x158), 0);
    }

    #[test]
    fn fn_009b7cb0_reads_the_global() {
        let mut e = Engine::new();
        e.map(GLOBAL_KEY & !0xfff, 0x1000);
        e.mem.set_u32(GLOBAL_KEY, 0x4321);
        assert_eq!(e.call(0x009b_7cb0, &[]).u32(), 0x4321);
    }
}
