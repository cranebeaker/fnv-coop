//! `fallout shared/havok/run_00622470` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009938b0` (`fallout/ai/combat/combatmanager.cpp` in the engine map): `thiscall` on an array,
/// plain `RET`; answers its element count.
const ARRAY_COUNT: u32 = 0x0099_38b0;
/// `00877a30` (`fallout/misc/main.cpp` in the engine map): `thiscall` on an array, `RET 4`;
/// answers the element at an index.
const ARRAY_AT: u32 = 0x0087_7a30;
/// `00537bb0` (`fallout shared/tesland.cpp` in the engine map): `thiscall`, plain `RET`.
const BASE_CONSTRUCT: u32 = 0x0053_7bb0;
/// `006815c0` (unnamed in the engine map): `thiscall`, plain `RET`.
const RECORD_CONSTRUCT: u32 = 0x0068_15c0;
/// `004b4d50` (unnamed in the engine map): `thiscall` with four `float`s, `RET 0x10`.
const SET_FOUR_FLOATS: u32 = 0x004b_4d50;
/// `00559450` (unnamed in the engine map): `thiscall`, plain `RET`.
const ELEMENT_OBJECT: u32 = 0x0055_9450;
/// `005610f0` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, `RET 4`, the
/// address of three `float`s.
const OBJECT_SET_POSITION: u32 = 0x0056_10f0;
/// `00561150` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, `RET 4`, the
/// address of the record `00622730` filled.
const OBJECT_SET_ROTATION: u32 = 0x0056_1150;
/// `00621ef0` (`fallout shared/havok/cellmopp.cpp` in the engine map): `thiscall`, plain `RET`.
const CELL_CHECK: u32 = 0x0062_1ef0;
/// `0056d2c0` (`fallout shared/tesobjectrefr.cpp` in the engine map): `thiscall`, `RET 4`.
const WORLD_ADD_OBJECT: u32 = 0x0056_d2c0;
/// `hkColor::rgbFromChars` (Xbox PDB), `00cabb00`: `cdecl`, four words.
const RGB_FROM_CHARS: u32 = 0x00ca_bb00;
/// `bhkWorld::SetDebugObjectColor` (Xbox PDB), `00c68e30`: `thiscall`, `RET 8`.
const SET_DEBUG_OBJECT_COLOR: u32 = 0x00c6_8e30;
/// `004169a0` (`fallout shared/extradatalist.cpp` in the engine map): `cdecl`, a `float` and two
/// addresses.
const HALF_ANGLE_PAIR: u32 = 0x0041_69a0;
/// `004b0220` (Xbox PDB name `NiPointer<BSTempEffect>::_vector_deleting_destructor_`, possibly
/// a folded body): `cdecl`, one word.
const RELEASE_HELD: u32 = 0x004b_0220;
/// The `float` the word at `+0x1c` starts as (`-1.0` in the exe).
const INITIAL_SCALE: u32 = 0x0101_2054;
/// The `float` angle `00622590` hands to `00622730` (`pi / 2` in the exe).
const QUARTER_TURN: u32 = 0x0101_ff38;
/// The `float` the x offset starts each row at (`-2048.0` in the exe).
const FIRST_COLUMN: u32 = 0x0104_dc8c;
/// The `double` step between columns and rows (`2048.0` in the exe).
const GRID_STEP: u32 = 0x0101_6968;
/// The `double` factor `00622730` scales the angle with (`0.5` in the exe).
const HALF: u32 = 0x0101_1588;
/// The vtable `00622780` stores (its first slot is `00622810`).
const RELEASED_VTABLE: u32 = 0x0104_dc94;

// Translated from 00622470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: whether the array at `this + 8` has elements
/// (`009938b0` answers a count other than zero).
pub fn fn_00622470(e: &mut Engine, this: Ptr) -> bool {
    e.call(ARRAY_COUNT, &args![this.addr().wrapping_add(8)])
        .u32()
        != 0
}

// Translated from 006224f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`, plain `RET`. Constructs the records at `+0` and `+0x20`
/// (`006815c0`), hands `1.0, 1.0, 1.0, 0.0` to `004b4d50` on `this`, sets the words at `+0x10`
/// and `+0x14` to 2, the `float` at `+0x18` to 0 and the one at `+0x1c` to the constant at
/// `01012054`, and clears the byte at `+0x20` (`00622570`). Answers `this`.
pub fn fn_006224f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(RECORD_CONSTRUCT, &args![base]);
    e.call(RECORD_CONSTRUCT, &args![base.wrapping_add(0x20)]);
    e.call(
        SET_FOUR_FLOATS,
        &args![
            this,
            1.0f32.to_bits(),
            1.0f32.to_bits(),
            1.0f32.to_bits(),
            0.0f32.to_bits()
        ],
    );
    e.mem.set_u32(base.wrapping_add(0x10), 2);
    e.mem.set_u32(base.wrapping_add(0x14), 2);
    e.mem.set_u32(base.wrapping_add(0x18), 0.0f32.to_bits());
    let scale = x87_quiet_bits(e.mem.u32(INITIAL_SCALE));
    e.mem.set_u32(base.wrapping_add(0x1c), scale);
    fn_00622570(e, Ptr::new(base.wrapping_add(0x20)), 0);
    this
}

// Translated from 006224c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`, plain `RET`. Runs `006224f0` on `this`, then clears
/// the word at `+0x30` and the `float` at `+0x34`. Answers `this`.
pub fn fn_006224c0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_006224f0(e, this);
    e.mem.set_u32(this.addr().wrapping_add(0x30), 0);
    e.mem
        .set_u32(this.addr().wrapping_add(0x34), 0.0f32.to_bits());
    this
}

// Translated from 00622490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`, plain `RET`. Runs `00537bb0` on `this` and `006224c0`
/// on `this + 0x10`. Answers `this`.
pub fn fn_00622490(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    fn_006224c0(e, Ptr::new(this.addr().wrapping_add(0x10)));
    this
}

// Translated from 00622570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__thiscall`: stores the byte `value` at `this` and returns `this`.
pub fn fn_00622570(e: &mut Engine, this: Ptr, value: u8) -> Ptr {
    e.mem.set_u8(this.addr(), value);
    this
}

// Translated from 00622730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`, a `float`. Rounds `angle * 0.5` (the `double`
/// at `01011588`) to a `float` and hands it with `this + 4` and `this` to `004169a0` (`cdecl`),
/// then clears the `float`s at `+0xc` and `+8`.
pub fn fn_00622730(e: &mut Engine, this: Ptr, angle: f32) {
    let half: f64 = e.global(HALF);
    let scaled = x87_store32(x87_mul(f64::from(x87_quiet(angle)), half));
    e.call(
        HALF_ANGLE_PAIR,
        &args![scaled, this.addr().wrapping_add(4), this],
    );
    e.mem
        .set_u32(this.addr().wrapping_add(0xc), 0.0f32.to_bits());
    e.mem.set_u32(this.addr().wrapping_add(8), 0.0f32.to_bits());
}

// Translated from 00622590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 0xc` (the third word is never read). With a
/// `world` and elements in the array at `this + 8` (`00622470`): builds a record on the stack
/// (`006815c0`, then `00622730` with the `float` at `0101ff38`); places the first four elements'
/// objects (`00877a30`, `00559450`) on a 2 by 2 grid around `position`: x offsets from the
/// `float` at `0104dc8c`, y offsets from 0, both stepping by the `double` at `01016968`, z kept
/// (`005610f0`), each given the stack record (`00561150`). Then, when `00621ef0` answers 0, adds
/// every element's object to `world` (`0056d2c0`) and gives it the debug colour
/// `rgbFromChars(0xdc, 0x96, 0x28, 0xff)` (`SetDebugObjectColor`). Always answers 0.
pub fn fn_00622590(e: &mut Engine, this: Ptr, world: u32, position: Ptr, _unused_3: u32) -> bool {
    if world == 0 || !fn_00622470(e, this) {
        return false;
    }
    let array = this.addr().wrapping_add(8);
    // The stack frame: the point at `-0x38` (0xc bytes) and the record at `-0x14` (0x10).
    e.with_stack(0x20, |e, frame| {
        let point = frame.addr();
        let record = frame.addr().wrapping_add(0x10);
        e.call(RECORD_CONSTRUCT, &args![record]);
        let angle = f32::from_bits(x87_quiet_bits(e.mem.u32(QUARTER_TURN)));
        fn_00622730(e, Ptr::new(record), angle);
        let step: f64 = e.global(GRID_STEP);
        let mut row_offset = 0.0f32.to_bits();
        let mut index = 0u32;
        let mut row = 0u32;
        while row < 2 {
            let mut column_offset = x87_quiet_bits(e.mem.u32(FIRST_COLUMN));
            let mut column = 0u32;
            while column < 2 {
                let element = e.call(ARRAY_AT, &args![array, index]).u32();
                index = index.wrapping_add(1);
                let object = e.call(ELEMENT_OBJECT, &args![element]).u32();
                let px = e.mem.u32(position.addr());
                let py = e.mem.u32(position.addr().wrapping_add(4));
                let pz = e.mem.u32(position.addr().wrapping_add(8));
                let x = x87_store32(x87_add(x87_load32(px), x87_m32(column_offset)));
                let y = x87_store32(x87_add(x87_load32(py), x87_m32(row_offset)));
                e.mem.set_u32(point, x);
                e.mem.set_u32(point.wrapping_add(4), y);
                e.mem.set_u32(point.wrapping_add(8), pz);
                e.call(OBJECT_SET_POSITION, &args![object, point]);
                e.call(OBJECT_SET_ROTATION, &args![object, record]);
                column_offset = x87_store32(x87_add(x87_load32(column_offset), step));
                column += 1;
            }
            row_offset = x87_store32(x87_add(x87_load32(row_offset), step));
            row += 1;
        }
    });
    if e.call(CELL_CHECK, &args![this]).u32() == 0 {
        let mut index = 0u32;
        while index < e.call(ARRAY_COUNT, &args![array]).u32() {
            let element = e.call(ARRAY_AT, &args![array, index]).u32();
            let object = e.call(ELEMENT_OBJECT, &args![element]).u32();
            e.call(WORLD_ADD_OBJECT, &args![world, object]);
            let color = e
                .call(RGB_FROM_CHARS, &args![0xdcu32, 0x96u32, 0x28u32, 0xffu32])
                .u32();
            e.call(SET_DEBUG_OBJECT_COLOR, &args![world, object, color]);
            index = index.wrapping_add(1);
        }
    }
    false
}

// Translated from 00622780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra matches it to a library destructor by shape only).
/// `fastcall`, plain `RET`: stores the vtable `0104dc94` and hands the word at `+4` to `004b0220`
/// (`cdecl`).
pub fn fn_00622780(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), RELEASED_VTABLE);
    let held = e.mem.u32(this.addr().wrapping_add(4));
    e.call(RELEASE_HELD, &args![held]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00622570, fn_00622570(Ptr, u8) -> Ptr),
        entry!(0x0062_2470, fn_00622470(Ptr) -> bool),
        entry!(0x0062_2490, fn_00622490(Ptr) -> Ptr),
        entry!(0x0062_24c0, fn_006224c0(Ptr) -> Ptr),
        entry!(0x0062_24f0, fn_006224f0(Ptr) -> Ptr),
        entry!(0x0062_2590, fn_00622590(Ptr, u32, Ptr, u32) -> bool),
        entry!(0x0062_2730, fn_00622730(Ptr, f32)),
        entry!(0x0062_2780, fn_00622780(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00622570_stores_the_byte_and_returns_this() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        let result = e.call(0x0062_2570, &args![Ptr::<()>::new(this), 0x5au8]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u8(this), 0x5a);
        assert_eq!(e.mem.u8(this + 1), 0);
    }

    type Placed = std::rc::Rc<std::cell::RefCell<Vec<(u32, f32, f32, u32)>>>;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn map_constants(e: &mut Engine) {
        e.map(0x0101_1000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x0101_f000, 0x1000);
        e.map(0x0104_d000, 0x1000);
        e.set_global(INITIAL_SCALE, -1.0f32);
        e.set_global(QUARTER_TURN, std::f32::consts::FRAC_PI_2);
        e.set_global(FIRST_COLUMN, -2048.0f32);
        e.set_global(GRID_STEP, 2048.0f64);
        e.set_global(HALF, 0.5f64);
    }

    #[test]
    fn at_00622470_answers_whether_the_array_has_elements() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ARRAY_COUNT, |_, _| 0u32.into_ret());
        assert!(!e.call(0x0062_2470, &args![0x4000u32]).bool());
        e.register(ARRAY_COUNT, |_, _| 3u32.into_ret());
        assert!(e.call(0x0062_2470, &args![0x4000u32]).bool());
        assert_eq!(calls(&e, ARRAY_COUNT), vec![vec![0x4008], vec![0x4008]]);
    }

    #[test]
    fn at_00622490_constructs_the_base_and_the_member() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        map_constants(&mut e);
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(RECORD_CONSTRUCT, |_, _| Ret::default());
        e.register(SET_FOUR_FLOATS, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        e.mem.set_u8(this + 0x30, 0x77);
        e.mem.set_u32(this + 0x40, 0x1234);
        e.mem.set_u32(this + 0x44, 0x1234);
        assert_eq!(e.call(0x0062_2490, &args![this]).u32(), this);
        assert_eq!(calls(&e, BASE_CONSTRUCT), vec![vec![this]]);
        let member = this + 0x10;
        assert_eq!(
            calls(&e, RECORD_CONSTRUCT),
            vec![vec![member], vec![member + 0x20]]
        );
        let one = 1.0f32.to_bits();
        assert_eq!(
            calls(&e, SET_FOUR_FLOATS),
            vec![vec![member, one, one, one, 0]]
        );
        assert_eq!(e.mem.u32(member + 0x10), 2);
        assert_eq!(e.mem.u32(member + 0x14), 2);
        assert_eq!(e.mem.u32(member + 0x18), 0);
        assert_eq!(e.mem.u32(member + 0x1c), (-1.0f32).to_bits());
        assert_eq!(e.mem.u8(member + 0x20), 0);
        assert_eq!(e.mem.u32(member + 0x30), 0);
        assert_eq!(e.mem.u32(member + 0x34), 0);
    }

    #[test]
    fn at_006224f0_quiets_a_signalling_scale_constant() {
        let mut e = Engine::new();
        map_constants(&mut e);
        e.set_global(INITIAL_SCALE, 0x7f80_0001u32);
        e.register(RECORD_CONSTRUCT, |_, _| Ret::default());
        e.register(SET_FOUR_FLOATS, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call(0x0062_24f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x7fc0_0001);
    }

    #[test]
    fn at_00622730_hands_the_half_angle_and_clears_the_tail() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        map_constants(&mut e);
        e.register(HALF_ANGLE_PAIR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1111);
        e.mem.set_u32(this + 0xc, 0x2222);
        e.call(0x0062_2730, &args![this, 3.0f32]);
        assert_eq!(
            calls(&e, HALF_ANGLE_PAIR),
            vec![vec![1.5f32.to_bits(), this + 4, this]]
        );
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn at_00622590_does_nothing_without_a_world_or_elements() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ARRAY_COUNT, |_, _| 0u32.into_ret());
        let empty = e.call(0x0062_2590, &args![0x4000u32, 0x5000u32, 0x6000u32, 0u32]);
        assert!(!empty.bool());
        let no_world = e.call(0x0062_2590, &args![0x4000u32, 0u32, 0x6000u32, 0u32]);
        assert!(!no_world.bool());
        assert_eq!(calls(&e, ARRAY_COUNT).len(), 1, "no world: no count asked");
        assert!(calls(&e, RECORD_CONSTRUCT).is_empty());
    }

    fn grid_doubles(e: &mut Engine, cell_check: u32) {
        e.register(ARRAY_COUNT, |_, _| 4u32.into_ret());
        e.register(ARRAY_AT, |_, a| (0x100 + a[1]).into_ret());
        e.register(ELEMENT_OBJECT, |_, a| (a[0] + 0x1000).into_ret());
        e.register(RECORD_CONSTRUCT, |_, _| Ret::default());
        e.register(HALF_ANGLE_PAIR, |_, _| Ret::default());
        e.register(OBJECT_SET_ROTATION, |_, _| Ret::default());
        e.register(WORLD_ADD_OBJECT, |_, _| Ret::default());
        e.register(RGB_FROM_CHARS, |_, _| 0xff28_96dcu32.into_ret());
        e.register(SET_DEBUG_OBJECT_COLOR, |_, _| Ret::default());
        e.register_double(CELL_CHECK, move |_, _| cell_check.into_ret());
    }

    #[test]
    fn at_00622590_places_four_objects_on_the_grid_and_colours_them() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        map_constants(&mut e);
        grid_doubles(&mut e, 0);
        let placed: Placed = Default::default();
        let seen = placed.clone();
        e.register_double(OBJECT_SET_POSITION, move |e, a| {
            let p = a[1];
            seen.borrow_mut().push((
                a[0],
                f32::from_bits(e.mem.u32(p)),
                f32::from_bits(e.mem.u32(p + 4)),
                e.mem.u32(p + 8),
            ));
            Ret::default()
        });
        let position = e.mem.alloc(0xc);
        e.mem.set_u32(position, 100.0f32.to_bits());
        e.mem.set_u32(position + 4, 10.0f32.to_bits());
        // z is copied as a word: a signalling NaN stays signalling.
        e.mem.set_u32(position + 8, 0x7f80_0001);
        let this = 0x4000u32;
        let answer = e.call(0x0062_2590, &args![this, 0x5000u32, position, 0x99u32]);
        assert!(!answer.bool());
        assert_eq!(
            *placed.borrow(),
            vec![
                (0x1100, -1948.0, 10.0, 0x7f80_0001),
                (0x1101, 100.0, 10.0, 0x7f80_0001),
                (0x1102, -1948.0, 2058.0, 0x7f80_0001),
                (0x1103, 100.0, 2058.0, 0x7f80_0001),
            ]
        );
        let half = (std::f32::consts::FRAC_PI_2 * 0.5).to_bits();
        let angle = calls(&e, HALF_ANGLE_PAIR);
        assert_eq!(angle.len(), 1);
        assert_eq!(angle[0][0], half);
        let record = angle[0][2];
        assert_eq!(calls(&e, OBJECT_SET_ROTATION)[0], vec![0x1100, record]);
        assert_eq!(calls(&e, CELL_CHECK), vec![vec![this]]);
        let added = calls(&e, WORLD_ADD_OBJECT);
        assert_eq!(added.len(), 4);
        assert_eq!(added[3], vec![0x5000, 0x1103]);
        assert_eq!(calls(&e, RGB_FROM_CHARS)[0], vec![0xdc, 0x96, 0x28, 0xff]);
        assert_eq!(
            calls(&e, SET_DEBUG_OBJECT_COLOR)[2],
            vec![0x5000, 0x1102, 0xff28_96dc]
        );
    }

    #[test]
    fn at_00622590_skips_the_colouring_when_the_cell_check_answers() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        map_constants(&mut e);
        grid_doubles(&mut e, 1);
        e.register(OBJECT_SET_POSITION, |_, _| Ret::default());
        let position = e.mem.alloc(0xc);
        e.call(0x0062_2590, &args![0x4000u32, 0x5000u32, position, 0u32]);
        assert_eq!(calls(&e, OBJECT_SET_POSITION).len(), 4);
        assert!(calls(&e, WORLD_ADD_OBJECT).is_empty());
        assert!(calls(&e, SET_DEBUG_OBJECT_COLOR).is_empty());
    }

    #[test]
    fn at_00622780_stores_the_vtable_and_releases_the_held_word() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(RELEASE_HELD, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x7777);
        e.call(0x0062_2780, &args![this]);
        assert_eq!(e.mem.u32(this), RELEASED_VTABLE);
        assert_eq!(calls(&e, RELEASE_HELD), vec![vec![0x7777]]);
    }
}
