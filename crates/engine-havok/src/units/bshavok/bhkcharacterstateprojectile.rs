//! `BSHavok/bhkcharacterstateprojectile.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `Update` carries a `/GS` cookie check (`00ec408c`) that is not modeled, and the input record
//! it builds for `calculateMovement` has stack words the function never writes.

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkCharacterController::UpdateState` (Xbox PDB, `00c6cba0`, `this` = the controller) and
/// `GetPosition_ov2` (Xbox PDB, `00c6e300`, `(controller, out)`; `out` takes four floats).
const UPDATE_STATE: u32 = 0x00c6_cba0;
const GET_POSITION: u32 = 0x00c6_e300;
/// `hkpCharacterMovementUtil::calculateMovement` (Xbox PDB, `00d6aef0`, cdecl `(input,
/// output)`).
const CALCULATE_MOVEMENT: u32 = 0x00d6_aef0;
/// Constants in the exe image: the floats -1e8 (`01070218`), 50000.0 (`01017130`) and 0.0
/// (`01011d78`), and the vector whose first lane is 0.0 (`010c7160`).
const NEGATIVE_LIMIT: u32 = 0x0107_0218;
const SPEED_LIMIT: u32 = 0x0101_7130;
const ZERO_CONSTANT: u32 = 0x0101_1d78;
const PROJECTION_VECTOR: u32 = 0x010c_7160;
/// The class vtable stored by `00cd3c90`.
const VTABLE_CLASS: u32 = 0x010c_b344;

fn words(e: &Engine, at: u32) -> [u32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.u32(at + 4 * i))
}

fn set_words(e: &mut Engine, at: u32, value: [u32; 4]) {
    for (i, word) in value.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, word);
    }
}

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    words(e, at).map(f32::from_bits)
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    set_words(e, at, value.map(f32::to_bits));
}

/// The sum the code makes of a lane-wise product: `p2 + (p1 + p0)`.
fn sum_of_lanes(product: [f32; 4]) -> f32 {
    product[2] + (product[1] + product[0])
}

// Translated from 00cd3c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterStateProjectile::getType` (Xbox PDB): takes no arguments and returns 6.
pub fn bhk_character_state_projectile_get_type(_e: &mut Engine) -> u32 {
    6
}

// Translated from 00cd3c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the constructor, `this` only): stores 1 in the 16-bit word at `+6` and the class
/// vtable at `+0`. Answers `this`.
pub fn fn_00cd3c90(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    this
}

// Translated from 00cd3cb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd3cb0 - the /GS cookie check (00ec408c) is not modeled, and three words of the input record for calculateMovement (offsets +4, +8, +0xC) and the last three words of its +0x60 vector are never written by the function (uninitialised stack words).
/// `bhkCharacterStateProjectile::Update` (Xbox PDB), virtual `(this, controller)`; `this` is
/// not used. Offsets are those of the controller.
///
/// The velocity is the vector at `+0x4f0`. A 0x80-byte input record for
/// `calculateMovement` (`00d6aef0`, `(input, +0x4f0)`) is built on the stack from: word 0
/// (1.0 when the first velocity float is below the float -1e8 at `01070218`, in which case
/// that float is first set to 0.0; else the float at `+0x534`), the vectors at `+0x4d0`,
/// `+0x4c0` (twice), the velocity before the call, the lanes (`+0x514`, `+0x510`, `+0x518`,
/// 0.0) of the vector at `+0x510`, 50000.0 (`01017130`) and the vector at `+0x4a0`. When
/// the velocity float was not below -1e8: if the float at `+0x54c` is above 0 the up vector
/// at `+0x4c0` is projected out of the velocity (its component along the vector is removed
/// down to the first lane of `010c7160`, then added back from the velocity before the call);
/// then the position (`GetPosition_ov2`, third float) is compared with the float at
/// `+0x53c`, and when it is above that float the vector at `+0x10` of what virtual slot
/// 0x94 answers on its second call (slot 0x94 runs three times) times the float at `+0x54c`
/// times the float at `+0x4e8` is added to the velocity. Finally, when the word at `+0x520`
/// is not 0xB, `UpdateState` (`00c6cba0`) runs.
pub fn bhk_character_state_projectile_update(e: &mut Engine, _this: Ptr, controller: Ptr) {
    let c = controller.addr();
    let velocity_at = c + 0x4f0;
    let zero = e.global::<u32>(ZERO_CONSTANT);
    let limit = x87_quiet(e.mem.f32(NEGATIVE_LIMIT));
    let first = e.mem.f32(velocity_at);
    let below_limit = limit > first;
    let forward = words(e, c + 0x4d0);
    let up = words(e, c + 0x4c0);
    let first_word = if below_limit {
        e.mem.set_f32(velocity_at, 0.0);
        1.0f32.to_bits()
    } else {
        x87_quiet_bits(e.mem.u32(c + 0x534))
    };
    let velocity_before = words(e, velocity_at);
    let direction = words(e, c + 0x510);
    let record_direction = [
        direction[1],
        x87_quiet_bits(direction[0]),
        x87_quiet_bits(direction[2]),
        zero,
    ];
    let speed = x87_quiet_bits(e.mem.u32(SPEED_LIMIT));
    let surface = words(e, c + 0x4a0);
    let before = e.with_stack(0x80, |e, record| {
        let r = record.addr();
        e.mem.set_u32(r, first_word);
        set_words(e, r + 0x10, forward);
        set_words(e, r + 0x20, up);
        set_words(e, r + 0x30, up);
        set_words(e, r + 0x40, velocity_before);
        set_words(e, r + 0x50, record_direction);
        e.mem.set_u32(r + 0x60, speed);
        set_words(e, r + 0x70, surface);
        e.call(CALCULATE_MOVEMENT, &args![record, velocity_at]);
        lanes(e, r + 0x40)
    });
    if !below_limit {
        let gravity = x87_quiet(e.mem.f32(c + 0x54c));
        if 0.0 < gravity {
            let normal = lanes(e, c + 0x4c0);
            let velocity = lanes(e, velocity_at);
            let product = [0, 1, 2, 3].map(|i| normal[i] * velocity[i]);
            let target = e.mem.f32(PROJECTION_VECTOR);
            let gap = target - sum_of_lanes(product);
            let moved = [0, 1, 2, 3].map(|i| gap * normal[i] + velocity[i]);
            set_lanes(e, velocity_at, moved);
            let normal = lanes(e, c + 0x4c0);
            let product = [0, 1, 2, 3].map(|i| normal[i] * before[i]);
            let along = sum_of_lanes(product);
            let velocity = lanes(e, velocity_at);
            let restored = [0, 1, 2, 3].map(|i| along * normal[i] + velocity[i]);
            set_lanes(e, velocity_at, restored);
        }
        let position_height = e.with_stack(0x10, |e, position| {
            e.call(GET_POSITION, &args![controller, position]);
            x87_quiet(e.mem.f32(position.addr() + 8))
        });
        let bound = x87_quiet(e.mem.f32(c + 0x53c));
        if bound < position_height {
            e.vcall(c, 0x94, &args![]);
            let second = e.vcall(c, 0x94, &args![]).u32();
            let vector = lanes(e, second + 0x10);
            e.vcall(c, 0x94, &args![]);
            let step = x87_quiet(e.mem.f32(c + 0x4e8));
            let velocity = lanes(e, velocity_at);
            let moved = [0, 1, 2, 3].map(|i| (gravity * vector[i]) * step + velocity[i]);
            set_lanes(e, velocity_at, moved);
        }
    }
    if e.mem.u32(c + 0x520) != 0xb {
        e.call(UPDATE_STATE, &args![controller]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd3c80, bhk_character_state_projectile_get_type() -> u32),
        entry!(0x00cd3c90, fn_00cd3c90(Ptr) -> Ptr),
        entry!(0x00cd3cb0, bhk_character_state_projectile_update(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Words = Rc<RefCell<Vec<u32>>>;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn get_type_answers_six() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cd3c80, &args![]).u32(), 6);
    }

    #[test]
    fn constructor_stores_the_type_word_and_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u16(this + 6, 9);
        assert_eq!(e.call(0x00cd3c90, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
    }

    struct Setup {
        e: Engine,
        controller: u32,
        record: Words,
        positions: Rc<Cell<u32>>,
    }

    /// A controller (0x640 bytes, virtual slot 0x94 answering an object whose `+0x10`
    /// vector is (1, 2, 3, 4)) with `calculateMovement` copying the record and adding 100
    /// to the first velocity float.
    fn setup() -> Setup {
        let mut e = Engine::new();
        for (address, bits) in [
            (NEGATIVE_LIMIT, (-1.0e8f32).to_bits()),
            (SPEED_LIMIT, 50000.0f32.to_bits()),
            (ZERO_CONSTANT, 0),
            (PROJECTION_VECTOR, 0),
        ] {
            e.map(address, 16);
            e.set_global(address, bits);
        }
        let helper = e.mem.alloc(0x40);
        for lane in 0..4u32 {
            e.mem.set_f32(helper + 0x10 + 4 * lane, 1.0 + lane as f32);
        }
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(table + 0x94, 0x0900_0094);
        e.register_double(0x0900_0094, move |_, _| Ret {
            eax: helper,
            ..Ret::default()
        });
        let controller = e.mem.alloc(0x640);
        e.mem.set_u32(controller, table);
        let record: Words = Rc::new(RefCell::new(vec![]));
        let record_in = record.clone();
        e.register_double(CALCULATE_MOVEMENT, move |e, a| {
            *record_in.borrow_mut() = (0..0x20u32).map(|n| e.mem.u32(a[0] + 4 * n)).collect();
            let first = e.mem.f32(a[1]);
            e.mem.set_f32(a[1], first + 100.0);
            Ret::default()
        });
        e.register(UPDATE_STATE, |_, _| Ret::default());
        let positions = Rc::new(Cell::new(0u32));
        let positions_in = positions.clone();
        e.register_double(GET_POSITION, move |e, a| {
            positions_in.set(positions_in.get() + 1);
            for lane in 0..4u32 {
                e.mem.set_f32(a[1] + 4 * lane, 10.0 * (1.0 + lane as f32));
            }
            Ret::default()
        });
        Setup {
            e,
            controller,
            record,
            positions,
        }
    }

    fn fill(e: &mut Engine, controller: u32) {
        let fill: [(u32, [f32; 4]); 5] = [
            (0x4a0, [0.5, 0.25, 0.125, 0.0]),
            (0x4c0, [0.0, 1.0, 0.0, 0.0]),
            (0x4d0, [0.0, 0.0, 1.0, 0.0]),
            (0x4f0, [3.0, 4.0, 5.0, 6.0]),
            (0x510, [7.0, 8.0, 9.0, 10.0]),
        ];
        for (offset, value) in fill {
            set_lanes(e, controller + offset, value);
        }
        e.mem.set_f32(controller + 0x534, 2.5);
        e.mem.set_f32(controller + 0x54c, 0.0);
        e.mem.set_f32(controller + 0x53c, 100.0);
        e.mem.set_f32(controller + 0x4e8, 0.5);
        e.mem.set_u32(controller + 0x520, 0xb);
    }

    #[test]
    fn update_below_the_limit_zeroes_the_first_float_and_only_calculates() {
        let mut s = setup();
        let c = s.controller;
        fill(&mut s.e, c);
        s.e.mem.set_f32(c + 0x4f0, -2.0e8);
        s.e.mem.set_f32(c + 0x54c, 9.0);
        s.e.call_log = Some(vec![]);
        s.e.call(0x00cd3cb0, &args![0x1234u32, c]);
        let record = s.record.borrow();
        assert_eq!(f32::from_bits(record[0]), 1.0);
        assert_eq!(&record[4..8], [0, 0, 0x3f80_0000, 0]);
        assert_eq!(&record[8..12], [0, 0x3f80_0000, 0, 0]);
        assert_eq!(&record[12..16], [0, 0x3f80_0000, 0, 0]);
        // The velocity before the call, with the first float zeroed.
        let velocity = [0.0f32, 4.0, 5.0, 6.0].map(f32::to_bits);
        assert_eq!(&record[16..20], velocity);
        let direction = [8.0f32, 7.0, 9.0, 0.0].map(f32::to_bits);
        assert_eq!(&record[20..24], direction);
        assert_eq!(f32::from_bits(record[24]), 50000.0);
        let surface = [0.5f32, 0.25, 0.125, 0.0].map(f32::to_bits);
        assert_eq!(&record[28..32], surface);
        // The output: 0 + 100, nothing else (no projection, no position).
        assert_eq!(lanes(&s.e, c + 0x4f0), [100.0, 4.0, 5.0, 6.0]);
        assert_eq!(s.positions.get(), 0);
        assert!(calls(&s.e, 0x0900_0094).is_empty());
        assert!(calls(&s.e, UPDATE_STATE).is_empty());
    }

    #[test]
    fn update_projects_the_up_vector_and_adds_the_lift_above_the_bound() {
        let mut s = setup();
        let c = s.controller;
        fill(&mut s.e, c);
        s.e.mem.set_f32(c + 0x54c, 2.0);
        // The position's third float is 30.0; the bound is below it.
        s.e.mem.set_f32(c + 0x53c, 25.0);
        s.e.call_log = Some(vec![]);
        s.e.call(0x00cd3cb0, &args![0x1234u32, c]);
        let record = s.record.borrow();
        assert_eq!(f32::from_bits(record[0]), 2.5);
        // calculateMovement leaves (103, 4, 5, 6); the up vector (0, 1, 0, 0) gives
        // 0 - 4 = -4: (103, 0, 5, 6); then the old 4 is added back: (103, 4, 5, 6).
        // The lift is gravity 2.0 * (1, 2, 3, 4) * step 0.5 on top.
        assert_eq!(lanes(&s.e, c + 0x4f0), [104.0, 6.0, 8.0, 10.0]);
        assert_eq!(s.positions.get(), 1);
        assert_eq!(calls(&s.e, 0x0900_0094).len(), 3);
        assert!(calls(&s.e, UPDATE_STATE).is_empty());
    }

    #[test]
    fn update_skips_the_lift_below_the_bound_and_updates_the_state() {
        let mut s = setup();
        let c = s.controller;
        fill(&mut s.e, c);
        s.e.mem.set_f32(c + 0x54c, 2.0);
        s.e.mem.set_f32(c + 0x53c, 35.0);
        s.e.mem.set_u32(c + 0x520, 4);
        s.e.call_log = Some(vec![]);
        s.e.call(0x00cd3cb0, &args![0x1234u32, c]);
        assert_eq!(lanes(&s.e, c + 0x4f0), [103.0, 4.0, 5.0, 6.0]);
        assert_eq!(s.positions.get(), 1);
        assert!(calls(&s.e, 0x0900_0094).is_empty());
        assert_eq!(calls(&s.e, UPDATE_STATE), [[c]]);
    }

    #[test]
    fn update_with_no_gravity_still_checks_the_position() {
        let mut s = setup();
        let c = s.controller;
        fill(&mut s.e, c);
        s.e.mem.set_f32(c + 0x53c, 35.0);
        s.e.call_log = Some(vec![]);
        s.e.call(0x00cd3cb0, &args![0x1234u32, c]);
        assert_eq!(lanes(&s.e, c + 0x4f0), [103.0, 4.0, 5.0, 6.0]);
        assert_eq!(s.positions.get(), 1);
    }
}
