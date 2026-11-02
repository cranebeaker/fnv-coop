//! `Havok SDK/hkpbreakableconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpBreakableConstraintData` (the Xbox PDB has no record of the Havok classes, so the field
//! names below are unconfirmed): wraps another constraint data. `+0x0C` is the 12-byte
//! atom of its own, `+0x18` the wrapped constraint data, `+0x1C` (`u16`) the offset of its
//! runtime flag byte (1 once broken) inside the runtime block, `+0x1E` (`u16`) the number of
//! solver results (8 bytes each, at the start of the runtime block), `+0x20` the break
//! threshold, `+0x24` the "remove when broken" byte and `+0x25` the "revert back velocity on
//! break" byte. After the flag byte the runtime block keeps a snapshot of the two bodies'
//! velocities (twelve floats from `flag + 4`). SSE code computes lane by lane in `f32`; the x87
//! code computes in double precision and rounds at each `float` store.

use super::hkpsphereshape::{load_vec, store_vec, zip_with, Vec4, ZERO_F32};
#[allow(unused_imports)]
use crate::prelude::*;

const WRAPPED: u32 = 0x18;
const RUNTIME_OFFSET: u32 = 0x1c;
const RUNTIME_COUNT: u32 = 0x1e;
const THRESHOLD: u32 = 0x20;
const REMOVE_WHEN_BROKEN: u32 = 0x24;
const REVERT_VELOCITY: u32 = 0x25;

/// The solver-atom builder `00d6f2b0` (cdecl, four words), the break and repair notifier
/// `00d4e020` (this member, three words), the simple contact jacobian builder `00d72190`
/// (cdecl, four words), the generic jacobian builder `00d6f460` (cdecl, four words) and
/// `hkpWorld::removeConstraint` (`00c97e80`, `this` the world, two words).
const BUILD_ATOMS: u32 = 0x00d6_f2b0;
const SET_BROKEN: u32 = 0x00d4_e020;
const BUILD_CONTACT_JACOBIAN: u32 = 0x00d7_2190;
const BUILD_JACOBIAN_FROM_ATOMS: u32 = 0x00d6_f460;
const WORLD_REMOVE_CONSTRAINT: u32 = 0x00c9_7e80;
/// The event listeners of an instance and of the world: the "broken" callback loops
/// (`00d85fa0` for the instance, `00d00b40` for the world) and the "repaired" ones
/// (`00d86050`, `00d00bf0`); the instance ones take the event, the world ones the world and
/// the event (all cdecl).
const INSTANCE_BROKEN: u32 = 0x00d8_5fa0;
const WORLD_BROKEN: u32 = 0x00d0_0b40;
const INSTANCE_REPAIRED: u32 = 0x00d8_6050;
const WORLD_REPAIRED: u32 = 0x00d0_0bf0;
/// The type word of a contact atom, and the constraint event word at `+8` of the event.
const CONTACT_ATOM_TYPE: u16 = 0x16;
const EVENT_TYPE: u32 = 0x0126_9c44;
/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// The vector of four `1.0` the revert blend subtracts the factor from.
const ONES_VECTOR: u32 = 0x010e_c7f0;

// Translated from 00d4de30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the jacobian of a broken constraint (no engine-map name; two stack words, popped by
/// the callee): `00d6f2b0` with the two words, 0 and 8.
pub fn fn_00d4de30(e: &mut Engine, params: u32, third: u32) {
    e.call(BUILD_ATOMS, &args![params, third, 0u32, 8u32]);
}

// Translated from 00d4de50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBreakableConstraintData::getConstraintInfo` (engine map): lets the wrapped constraint
/// data (slot `0x3C`) fill `info`, then sets the atoms pointer (`+0x10`) to `this + 0xC` and the
/// size of the atoms (`+0x14`) to 0xC.
pub fn hkp_breakable_constraint_data_get_constraint_info(e: &mut Engine, this: Ptr, info: Ptr) {
    let wrapped = e.mem.u32(this.addr() + WRAPPED);
    e.vcall(wrapped, 0x3c, &args![info]);
    e.mem.set_u32(info.addr() + 0x10, this.addr() + 0xc);
    e.mem.set_u32(info.addr() + 0x14, 0xc);
}

// Translated from 00d4de80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBreakableConstraintData::getRuntimeInfo` (engine map): two stack words, the first
/// never read. Writes the size of the runtime block (`+0x1C` plus 0x34) to `info` and the
/// number of solver results (`+0x1E`) to `info + 4`.
pub fn hkp_breakable_constraint_data_get_runtime_info(
    e: &mut Engine,
    this: Ptr,
    _unused: u32,
    info: Ptr,
) {
    let data = this.addr();
    let results = u32::from(e.mem.u16(data + RUNTIME_COUNT));
    e.mem.set_u32(info.addr() + 4, results);
    let size = u32::from(e.mem.u16(data + RUNTIME_OFFSET)) + 0x34;
    e.mem.set_u32(info.addr(), size);
}

// Translated from 00d4dea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBreakableConstraintData::isValid` (engine map): calls the wrapped constraint data's
/// slot `0x1C` with the word and returns the word itself, not the result of that call.
pub fn hkp_breakable_constraint_data_is_valid(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    let wrapped = e.mem.u32(this.addr() + WRAPPED);
    e.vcall(wrapped, 0x1c, &args![word]);
    word
}

// Translated from 00d4dec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBreakableConstraintData::getType` (engine map): the constraint type, 0xC.
pub fn hkp_breakable_constraint_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0xc
}

// Translated from 00d4e020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Breaks or repairs the constraint (no engine-map name; `ECX` = data, the instance, the new
/// state as a byte and the impulse as a `float` on the stack). Does nothing when the runtime
/// flag byte (at `+0x1C` plus the instance's internal block's runtime pointer at `+0x1C`)
/// already says as much (the byte is sign-extended before the compare, so only 0 and 1 can
/// match). Otherwise stores the byte, sets bit 2 of the internal block's byte `+0x12` when
/// repairing and clears it when breaking, then notifies the instance's listeners (when its
/// count at `+0x24` is not 0) and the world's with a 0x18-byte event on the stack: the world,
/// the instance, the type word `01269c44`, 0, and, when breaking, the impulse and the break
/// threshold. When repairing the exe leaves the last two words of the event unset.
pub fn fn_00d4e020(e: &mut Engine, this: Ptr, instance: Ptr, broken: u8, impulse: f32) {
    let data = this.addr();
    let instance = instance.addr();
    let internal = e.mem.u32(instance + 0x30);
    let flag = u32::from(e.mem.u16(data + RUNTIME_OFFSET)) + e.mem.u32(internal + 0x1c);
    let was_broken = i32::from(e.mem.u8(flag) != 0);
    if i32::from(broken as i8) == was_broken {
        return;
    }
    e.mem.set_u8(flag, broken);
    let internal = e.mem.u32(instance + 0x30);
    let state = e.mem.u8(internal + 0x12);
    e.mem.set_u8(
        internal + 0x12,
        if broken == 0 { state | 4 } else { state & 0xfb },
    );
    let world = e.mem.u32(e.mem.u32(instance + 8) + 0x18);
    e.with_stack(0x18, |e, event| {
        let event = event.addr();
        e.mem.set_u32(event, world);
        e.mem.set_u32(event + 4, instance);
        e.mem.set_u32(event + 8, EVENT_TYPE);
        e.mem.set_u32(event + 0xc, 0);
        let has_listeners = e.mem.u16(instance + 0x24) != 0;
        if broken != 0 {
            e.mem
                .set_u32(event + 0x10, x87_quiet_bits(impulse.to_bits()));
            let threshold = x87_quiet_bits(e.mem.u32(data + THRESHOLD));
            e.mem.set_u32(event + 0x14, threshold);
            if has_listeners {
                e.call(INSTANCE_BROKEN, &args![event]);
            }
            e.call(WORLD_BROKEN, &args![world, event]);
        } else {
            if has_listeners {
                e.call(INSTANCE_REPAIRED, &args![event]);
            }
            e.call(WORLD_REPAIRED, &args![world, event]);
        }
    });
}

// Translated from 00d4e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBreakableConstraintData::buildJacobian` (engine map): `ECX` = data, the build
/// parameters and one more word on the stack. When the runtime flag byte (`+0x1C` into the
/// block at `params + 0x64`) says broken, builds the broken jacobian (`00d4de30`) and, when
/// `+0x24` is set, has the instance's world (`[[instance + 0x14] + 8]`, the instance being
/// `params + 0x60`) remove the instance, the result slot being a stack word holding the third
/// word. Otherwise snapshots twelve floats of the two bodies (`params + 0x30` and `+ 0x34`:
/// `+0x10..` and `+0x20..`) into the runtime block after the flag byte, lets the wrapped
/// data (slot `0x3C`) describe its atoms in a 0x18-byte info on the stack and builds the
/// jacobian from them: the contact builder (`00d72190`, with the byte 1 in the low byte of the
/// word that held the parameters pointer) when the first atom has the type 0x16, else the
/// generic one (`00d6f460`).
pub fn hkp_breakable_constraint_data_build_jacobian(
    e: &mut Engine,
    this: Ptr,
    params: Ptr,
    third: u32,
) {
    let data = this.addr();
    let params_address = params.addr();
    let flag = u32::from(e.mem.u16(data + RUNTIME_OFFSET)) + e.mem.u32(params_address + 0x64);
    if e.mem.u8(flag) != 0 {
        e.call(0x00d4_de30, &args![params, third]);
        if e.mem.u8(data + REMOVE_WHEN_BROKEN) != 0 {
            let instance = e.mem.u32(params_address + 0x60);
            let world = e.mem.u32(e.mem.u32(instance + 0x14) + 8);
            e.with_stack(4, |e, result| {
                e.mem.set_u32(result.addr(), third);
                e.call(WORLD_REMOVE_CONSTRAINT, &args![world, result, instance]);
            });
        }
        return;
    }
    let body_a = e.mem.u32(params_address + 0x30);
    let body_b = e.mem.u32(params_address + 0x34);
    let snapshot = [
        (0x04, body_a, 0x10),
        (0x08, body_a, 0x14),
        (0x0c, body_a, 0x18),
        (0x10, body_b, 0x10),
        (0x14, body_b, 0x14),
        (0x18, body_b, 0x18),
        (0x1c, body_a, 0x20),
        (0x20, body_a, 0x24),
        (0x24, body_a, 0x28),
        (0x28, body_b, 0x20),
        (0x2c, body_b, 0x24),
        (0x30, body_b, 0x28),
    ];
    for (to, body, from) in snapshot {
        let word = x87_quiet_bits(e.mem.u32(body + from));
        e.mem.set_u32(flag + to, word);
    }
    let wrapped = e.mem.u32(data + WRAPPED);
    e.with_stack(0x18, |e, info| {
        e.vcall(wrapped, 0x3c, &args![info]);
        let atoms = e.mem.u32(info.addr() + 0x10);
        let size = e.mem.u32(info.addr() + 0x14);
        if e.mem.u16(atoms) == CONTACT_ATOM_TYPE {
            let marked = (params_address & 0xffff_ff00) | 1;
            e.call(BUILD_CONTACT_JACOBIAN, &args![atoms, params, marked, third]);
        } else {
            e.call(
                BUILD_JACOBIAN_FROM_ATOMS,
                &args![atoms, size, params, third],
            );
        }
    });
}

/// `row` blended: `saved * (1 - factor) + row * factor` per lane, stored back.
fn blend_row(e: &mut Engine, row: u32, saved: Vec4, factor: f32) {
    let ones = load_vec(e, ONES_VECTOR);
    let remaining = zip_with(ones, [factor; 4], |x, y| x - y);
    let kept = zip_with(saved, remaining, |x, y| x * y);
    let current = zip_with(load_vec(e, row), [factor; 4], |x, y| x * y);
    store_vec(e, row, zip_with(kept, current, |x, y| x + y));
}

// Translated from 00d4e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBreakableConstraintData::buildJacobianCallback` (engine map): `ECX` = data, the
/// callback parameters on the stack. When the runtime flag byte (at `+0x1C` into the block at
/// `params + 0x64`) is clear, sums the squares of the solver results (every 8 bytes from the
/// start of the block, `+0x1E` of them; each step is `v * v + sum` in double precision rounded
/// to a `float`) and when that exceeds the threshold squared, breaks the constraint
/// (`00d4e020`, with the instance at `params + 0x60`, the byte 1 and the square root
/// of the sum). With `+0x25` set it then pulls the bodies' velocities back toward the
/// snapshot kept after the flag byte: the factor is `(1 / magnitude) * threshold`, each of
/// the four vectors `body + 0x10` and `+ 0x20` of both bodies (`params + 0x30` and `+ 0x34`)
/// becoming `snapshot * (1 - factor) + current * factor` (the fourth lane of a snapshot
/// being the `float` zero at `01011d78`). Finally, in every case, clears the solver results.
pub fn hkp_breakable_constraint_data_build_jacobian_callback(
    e: &mut Engine,
    this: Ptr,
    params: Ptr,
) {
    let data = this.addr();
    let params_address = params.addr();
    let results = e.mem.u32(params_address + 0x64);
    let flag = u32::from(e.mem.u16(data + RUNTIME_OFFSET)) + results;
    let count = u32::from(e.mem.u16(data + RUNTIME_COUNT));
    if e.mem.u8(flag) == 0 {
        let mut sum = 0.0f32;
        for index in 0..count {
            let value = f64::from(x87_quiet(e.mem.f32(results + 8 * index)));
            sum = (value * value + f64::from(sum)) as f32;
        }
        let threshold = f64::from(e.mem.f32(data + THRESHOLD));
        if threshold * threshold < f64::from(sum) {
            let magnitude = e.call(X87_SQRT, &args![f64::from(sum)]).f64() as f32;
            let instance = e.mem.u32(params_address + 0x60);
            e.call(SET_BROKEN, &args![this, instance, 1u32, magnitude]);
            if e.mem.u8(data + REVERT_VELOCITY) != 0 {
                let reciprocal = (1.0f64 / f64::from(magnitude)) as f32;
                let factor =
                    (f64::from(reciprocal) * f64::from(e.mem.f32(data + THRESHOLD))) as f32;
                let zero: f32 = e.global(ZERO_F32);
                let body_a = e.mem.u32(params_address + 0x30);
                let body_b = e.mem.u32(params_address + 0x34);
                let saved = |e: &Engine, from: u32| -> Vec4 {
                    [
                        e.mem.f32(flag + from),
                        e.mem.f32(flag + from + 4),
                        e.mem.f32(flag + from + 8),
                        zero,
                    ]
                };
                let a_first = saved(e, 0x04);
                let b_first = saved(e, 0x10);
                let a_second = saved(e, 0x1c);
                let b_second = saved(e, 0x28);
                blend_row(e, body_a + 0x10, a_first, factor);
                blend_row(e, body_b + 0x10, b_first, factor);
                blend_row(e, body_a + 0x20, a_second, factor);
                blend_row(e, body_b + 0x20, b_second, factor);
            }
        }
    }
    for index in 0..count {
        e.mem.set_u32(results + 8 * index, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_de30, fn_00d4de30(u32, u32)),
        entry!(
            0x00d4_de50,
            hkp_breakable_constraint_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(
            0x00d4_de80,
            hkp_breakable_constraint_data_get_runtime_info(Ptr, u32, Ptr)
        ),
        entry!(0x00d4_dea0, hkp_breakable_constraint_data_is_valid(Ptr, u32) -> u32),
        entry!(0x00d4_dec0, hkp_breakable_constraint_data_get_type(Ptr) -> u32),
        entry!(0x00d4_e020, fn_00d4e020(Ptr, Ptr, u8, f32)),
        entry!(
            0x00d4_e0f0,
            hkp_breakable_constraint_data_build_jacobian(Ptr, Ptr, u32)
        ),
        entry!(
            0x00d4_e1e0,
            hkp_breakable_constraint_data_build_jacobian_callback(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SLOT_INFO: u32 = 0x0900_0201;
    const SLOT_VALID: u32 = 0x0900_0202;

    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.register_double(X87_SQRT, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: x.sqrt(),
                ..Ret::default()
            }
        });
        (e, Rc::default())
    }

    /// Logs every call to the given addresses with its argument words.
    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An object whose vtable (at `+0`) holds `slots` at the given byte offsets.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x80);
        for &(offset, target) in slots {
            e.mem.set_u32(table + offset, target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    /// A breakable constraint data: wrapped object `wrapped`, flag offset `0x40`, `count`
    /// solver results, threshold `threshold`, the two option bytes.
    fn data(e: &mut Engine, wrapped: u32, count: u16, threshold: f32, options: [u8; 2]) -> u32 {
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + WRAPPED, wrapped);
        e.mem.set_u16(data + RUNTIME_OFFSET, 0x40);
        e.mem.set_u16(data + RUNTIME_COUNT, count);
        e.mem.set_f32(data + THRESHOLD, threshold);
        e.mem.set_u8(data + REMOVE_WHEN_BROKEN, options[0]);
        e.mem.set_u8(data + REVERT_VELOCITY, options[1]);
        data
    }

    #[test]
    fn broken_jacobian_builder_appends_zero_and_eight() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[BUILD_ATOMS]);
        e.call(0x00d4_de30, &args![0x1111u32, 0x2222u32]);
        assert_eq!(
            *log.borrow(),
            vec![(BUILD_ATOMS, vec![0x1111, 0x2222, 0, 8])]
        );
    }

    #[test]
    fn constraint_info_comes_from_the_wrapped_data_then_names_the_own_atom() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SLOT_INFO]);
        let wrapped = object_with_slots(&mut e, &[(0x3c, SLOT_INFO)]);
        let this = data(&mut e, wrapped, 0, 1.0, [0, 0]);
        let info = e.mem.alloc(0x18);
        e.mem.set_u32(info + 0x10, 0xdead);
        e.call(0x00d4_de50, &args![this, info]);
        assert_eq!(*log.borrow(), vec![(SLOT_INFO, vec![wrapped, info])]);
        assert_eq!(e.mem.u32(info + 0x10), this + 0xc);
        assert_eq!(e.mem.u32(info + 0x14), 0xc);
    }

    #[test]
    fn runtime_info_adds_0x34_to_the_flag_offset() {
        let (mut e, _) = engine();
        let this = data(&mut e, 0, 7, 1.0, [0, 0]);
        let info = e.mem.alloc(8);
        e.call(0x00d4_de80, &args![this, 0xffffu32, info]);
        assert_eq!(e.mem.u32(info), 0x40 + 0x34);
        assert_eq!(e.mem.u32(info + 4), 7);
    }

    #[test]
    fn is_valid_forwards_to_the_wrapped_data_and_returns_its_word() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SLOT_VALID]);
        let wrapped = object_with_slots(&mut e, &[(0x1c, SLOT_VALID)]);
        let this = data(&mut e, wrapped, 0, 1.0, [0, 0]);
        assert_eq!(e.call(0x00d4_dea0, &args![this, 0x5555u32]).u32(), 0x5555);
        assert_eq!(*log.borrow(), vec![(SLOT_VALID, vec![wrapped, 0x5555])]);
    }

    #[test]
    fn type_is_0xc() {
        let (mut e, _) = engine();
        assert_eq!(e.call(0x00d4_dec0, &args![0u32]).u32(), 0xc);
    }

    /// An instance with a world, an internal block with the runtime block, and a data
    /// object whose flag byte lives in that block.
    struct Break {
        data: u32,
        instance: u32,
        internal: u32,
        runtime: u32,
        world: u32,
    }

    fn break_setup(e: &mut Engine, log: &Log, listeners: u16) -> Break {
        // The event is copied when the callee runs, as the stack block is freed afterwards.
        for address in [
            INSTANCE_BROKEN,
            INSTANCE_REPAIRED,
            WORLD_BROKEN,
            WORLD_REPAIRED,
        ] {
            let sink = Rc::clone(log);
            let event_at = if address == INSTANCE_BROKEN || address == INSTANCE_REPAIRED {
                0
            } else {
                1
            };
            e.register_double(address, move |e, a| {
                let event: Vec<u32> = (0..6).map(|w| e.mem.u32(a[event_at] + 4 * w)).collect();
                sink.borrow_mut()
                    .push((address, [a.to_vec(), event].concat()));
                Ret::default()
            });
        }
        let world = e.mem.alloc(0x20);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 0x18, world);
        let runtime = e.mem.alloc(0x80);
        let internal = e.mem.alloc(0x40);
        e.mem.set_u32(internal + 0x1c, runtime);
        e.mem.set_u8(internal + 0x12, 0x5a);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 8, owner);
        e.mem.set_u32(instance + 0x30, internal);
        e.mem.set_u16(instance + 0x24, listeners);
        let data = data(e, 0, 0, 3.0, [0, 0]);
        Break {
            data,
            instance,
            internal,
            runtime,
            world,
        }
    }

    #[test]
    fn breaking_notifies_the_instance_then_the_world() {
        let (mut e, log) = engine();
        let b = break_setup(&mut e, &log, 2);
        e.mem.set_u8(b.internal + 0x12, 0x5e);
        e.call(0x00d4_e020, &args![b.data, b.instance, 1u8, 7.5f32]);
        assert_eq!(e.mem.u8(b.runtime + 0x40), 1);
        assert_eq!(e.mem.u8(b.internal + 0x12), 0x5a);
        let event = |tail: [u32; 2]| vec![b.world, b.instance, EVENT_TYPE, 0, tail[0], tail[1]];
        let expected_event = event([7.5f32.to_bits(), 3.0f32.to_bits()]);
        let first = &calls_to(&log, INSTANCE_BROKEN)[0];
        assert_eq!(first[1..], expected_event[..]);
        let second = &calls_to(&log, WORLD_BROKEN)[0];
        assert_eq!(second[2..], expected_event[..]);
        assert_eq!(second[..2], [b.world, first[0]]);
        assert!(calls_to(&log, INSTANCE_REPAIRED).is_empty());
        assert!(calls_to(&log, WORLD_REPAIRED).is_empty());
    }

    #[test]
    fn breaking_an_instance_without_listeners_only_tells_the_world() {
        let (mut e, log) = engine();
        let b = break_setup(&mut e, &log, 0);
        e.call(0x00d4_e020, &args![b.data, b.instance, 1u8, 1.0f32]);
        assert!(calls_to(&log, INSTANCE_BROKEN).is_empty());
        assert_eq!(calls_to(&log, WORLD_BROKEN).len(), 1);
    }

    #[test]
    fn repairing_sets_bit_2_and_uses_the_repaired_listeners() {
        let (mut e, log) = engine();
        let b = break_setup(&mut e, &log, 1);
        e.mem.set_u8(b.runtime + 0x40, 1);
        e.call(0x00d4_e020, &args![b.data, b.instance, 0u8, 9.0f32]);
        assert_eq!(e.mem.u8(b.runtime + 0x40), 0);
        assert_eq!(e.mem.u8(b.internal + 0x12), 0x5a | 4);
        let first = &calls_to(&log, INSTANCE_REPAIRED)[0];
        assert_eq!(first[1..5], [b.world, b.instance, EVENT_TYPE, 0]);
        let second = &calls_to(&log, WORLD_REPAIRED)[0];
        assert_eq!(second[2..6], [b.world, b.instance, EVENT_TYPE, 0]);
        assert!(calls_to(&log, INSTANCE_BROKEN).is_empty());
        assert!(calls_to(&log, WORLD_BROKEN).is_empty());
    }

    #[test]
    fn an_unchanged_state_does_nothing() {
        let (mut e, log) = engine();
        let b = break_setup(&mut e, &log, 1);
        e.call(0x00d4_e020, &args![b.data, b.instance, 0u8, 1.0f32]);
        e.mem.set_u8(b.runtime + 0x40, 7);
        e.call(0x00d4_e020, &args![b.data, b.instance, 1u8, 1.0f32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u8(b.runtime + 0x40), 7);
        assert_eq!(e.mem.u8(b.internal + 0x12), 0x5a);
    }

    #[test]
    fn a_negative_state_byte_differs_from_both_flag_values() {
        let (mut e, log) = engine();
        let b = break_setup(&mut e, &log, 0);
        e.mem.set_u8(b.runtime + 0x40, 1);
        e.mem.set_u8(b.internal + 0x12, 0x5e);
        e.call(0x00d4_e020, &args![b.data, b.instance, 0xffu8, 2.0f32]);
        assert_eq!(e.mem.u8(b.runtime + 0x40), 0xff);
        assert_eq!(e.mem.u8(b.internal + 0x12), 0x5a);
        assert_eq!(calls_to(&log, WORLD_BROKEN).len(), 1);
    }

    /// Build parameters: instance at `+0x60`, runtime block at `+0x64`, bodies at `+0x30` and
    /// `+0x34` whose words from `+0x10` count up from `1.0`, `100.0` for the second body.
    struct Params {
        params: u32,
        runtime: u32,
        instance: u32,
        world: u32,
        body_a: u32,
        body_b: u32,
    }

    fn params(e: &mut Engine) -> Params {
        let params = e.mem.alloc(0x80);
        let runtime = e.mem.alloc(0x100);
        let world = e.mem.alloc(0x20);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 8, world);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 0x14, owner);
        let body_a = e.mem.alloc(0x40);
        let body_b = e.mem.alloc(0x40);
        for word in 0..12 {
            e.mem.set_f32(body_a + 0x10 + 4 * word, 1.0 + word as f32);
            e.mem.set_f32(body_b + 0x10 + 4 * word, 100.0 + word as f32);
        }
        e.mem.set_u32(params + 0x30, body_a);
        e.mem.set_u32(params + 0x34, body_b);
        e.mem.set_u32(params + 0x60, instance);
        e.mem.set_u32(params + 0x64, runtime);
        Params {
            params,
            runtime,
            instance,
            world,
            body_a,
            body_b,
        }
    }

    #[test]
    fn build_jacobian_of_a_broken_constraint_removes_the_instance_when_asked() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[BUILD_ATOMS]);
        let p = params(&mut e);
        let sink = Rc::clone(&log);
        e.register_double(WORLD_REMOVE_CONSTRAINT, move |e, a| {
            let slot = e.mem.u32(a[1]);
            sink.borrow_mut()
                .push((WORLD_REMOVE_CONSTRAINT, vec![a[0], slot, a[2]]));
            Ret::default()
        });
        let this = data(&mut e, 0, 0, 1.0, [1, 0]);
        e.mem.set_u8(p.runtime + 0x40, 1);
        e.call(0x00d4_e0f0, &args![this, p.params, 0x3333u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (BUILD_ATOMS, vec![p.params, 0x3333, 0, 8]),
                (WORLD_REMOVE_CONSTRAINT, vec![p.world, 0x3333, p.instance]),
            ]
        );
    }

    #[test]
    fn build_jacobian_of_a_broken_constraint_keeps_the_instance_otherwise() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[BUILD_ATOMS, WORLD_REMOVE_CONSTRAINT]);
        let p = params(&mut e);
        let this = data(&mut e, 0, 0, 1.0, [0, 0]);
        e.mem.set_u8(p.runtime + 0x40, 1);
        e.call(0x00d4_e0f0, &args![this, p.params, 0x3333u32]);
        assert_eq!(calls_to(&log, BUILD_ATOMS).len(), 1);
        assert!(calls_to(&log, WORLD_REMOVE_CONSTRAINT).is_empty());
    }

    /// Doubles for a wrapped data whose info points at `atom_type` and size 0x30, and for
    /// both jacobian builders.
    fn wrapped_with_atom(e: &mut Engine, log: &Log, atom_type: u16) -> u32 {
        log_callees(e, log, &[BUILD_CONTACT_JACOBIAN, BUILD_JACOBIAN_FROM_ATOMS]);
        let atoms = e.mem.alloc(0x40);
        e.mem.set_u16(atoms, atom_type);
        e.register_double(SLOT_INFO, move |e, a| {
            e.mem.set_u32(a[1] + 0x10, atoms);
            e.mem.set_u32(a[1] + 0x14, 0x30);
            Ret::default()
        });
        object_with_slots(e, &[(0x3c, SLOT_INFO)])
    }

    #[test]
    fn build_jacobian_snapshots_the_bodies_and_picks_the_contact_builder() {
        let (mut e, log) = engine();
        let p = params(&mut e);
        let wrapped = wrapped_with_atom(&mut e, &log, 0x16);
        let this = data(&mut e, wrapped, 0, 1.0, [0, 0]);
        e.call(0x00d4_e0f0, &args![this, p.params, 0x3333u32]);
        let flag = p.runtime + 0x40;
        let expected = [
            (0x04, p.body_a + 0x10),
            (0x08, p.body_a + 0x14),
            (0x0c, p.body_a + 0x18),
            (0x10, p.body_b + 0x10),
            (0x14, p.body_b + 0x14),
            (0x18, p.body_b + 0x18),
            (0x1c, p.body_a + 0x20),
            (0x20, p.body_a + 0x24),
            (0x24, p.body_a + 0x28),
            (0x28, p.body_b + 0x20),
            (0x2c, p.body_b + 0x24),
            (0x30, p.body_b + 0x28),
        ];
        for (to, from) in expected {
            assert_eq!(e.mem.u32(flag + to), e.mem.u32(from));
        }
        let calls = calls_to(&log, BUILD_CONTACT_JACOBIAN);
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0][1..],
            [p.params, (p.params & 0xffff_ff00) | 1, 0x3333]
        );
        assert!(calls_to(&log, BUILD_JACOBIAN_FROM_ATOMS).is_empty());
    }

    #[test]
    fn build_jacobian_uses_the_generic_builder_for_other_atoms() {
        let (mut e, log) = engine();
        let p = params(&mut e);
        let wrapped = wrapped_with_atom(&mut e, &log, 0x17);
        let this = data(&mut e, wrapped, 0, 1.0, [0, 0]);
        e.call(0x00d4_e0f0, &args![this, p.params, 0x3333u32]);
        let calls = calls_to(&log, BUILD_JACOBIAN_FROM_ATOMS);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][1..], [0x30, p.params, 0x3333]);
        assert!(calls_to(&log, BUILD_CONTACT_JACOBIAN).is_empty());
    }

    fn callback_setup(
        e: &mut Engine,
        log: &Log,
        results: &[f32],
        threshold: f32,
        revert: u8,
    ) -> (u32, Params) {
        log_callees(e, log, &[SET_BROKEN]);
        let p = params(e);
        for (i, value) in results.iter().enumerate() {
            e.mem.set_f32(p.runtime + 8 * i as u32, *value);
            e.mem.set_f32(p.runtime + 8 * i as u32 + 4, 55.0);
        }
        let this = data(e, 0, results.len() as u16, threshold, [0, revert]);
        (this, p)
    }

    #[test]
    fn callback_breaks_when_the_squares_exceed_the_threshold() {
        let (mut e, log) = engine();
        let (this, p) = callback_setup(&mut e, &log, &[3.0, 4.0], 4.0, 0);
        e.call(0x00d4_e1e0, &args![this, p.params]);
        assert_eq!(
            *log.borrow(),
            vec![(SET_BROKEN, vec![this, p.instance, 1, 5.0f32.to_bits()])]
        );
        assert_eq!(e.mem.u32(p.runtime), 0);
        assert_eq!(e.mem.u32(p.runtime + 8), 0);
        assert_eq!(e.mem.f32(p.runtime + 4), 55.0);
    }

    #[test]
    fn callback_sums_every_result_including_the_unrolled_blocks() {
        let (mut e, log) = engine();
        let results = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let (this, p) = callback_setup(&mut e, &log, &results, 7.0, 0);
        e.call(0x00d4_e1e0, &args![this, p.params]);
        // 1 + 4 + 9 + 16 + 25 + 36 = 91 > 49.
        assert_eq!(
            *log.borrow(),
            vec![(
                SET_BROKEN,
                vec![this, p.instance, 1, (91.0f64.sqrt() as f32).to_bits()]
            )]
        );
        for index in 0..6 {
            assert_eq!(e.mem.u32(p.runtime + 8 * index), 0);
        }
    }

    #[test]
    fn callback_does_not_break_at_or_below_the_threshold() {
        let (mut e, log) = engine();
        let (this, p) = callback_setup(&mut e, &log, &[3.0, 4.0], 5.0, 1);
        e.call(0x00d4_e1e0, &args![this, p.params]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(p.runtime), 0);
        assert_eq!(e.mem.u32(p.runtime + 8), 0);
    }

    #[test]
    fn callback_does_not_break_on_a_nan_sum() {
        let (mut e, log) = engine();
        let (this, p) = callback_setup(&mut e, &log, &[f32::NAN, 4.0], 1.0, 0);
        e.call(0x00d4_e1e0, &args![this, p.params]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(p.runtime), 0);
    }

    #[test]
    fn callback_of_a_broken_constraint_only_clears_the_results() {
        let (mut e, log) = engine();
        let (this, p) = callback_setup(&mut e, &log, &[30.0, 40.0], 4.0, 1);
        e.mem.set_u8(p.runtime + 0x40, 1);
        e.call(0x00d4_e1e0, &args![this, p.params]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(p.runtime), 0);
        assert_eq!(e.mem.u32(p.runtime + 8), 0);
    }

    #[test]
    fn callback_with_no_results_breaks_nothing() {
        let (mut e, log) = engine();
        let (this, p) = callback_setup(&mut e, &log, &[], 0.0, 0);
        e.call(0x00d4_e1e0, &args![this, p.params]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn callback_blends_the_body_velocities_back_toward_the_snapshot() {
        let (mut e, log) = engine();
        let (this, p) = callback_setup(&mut e, &log, &[3.0, 4.0], 4.0, 1);
        let flag = p.runtime + 0x40;
        for word in 0..12 {
            e.mem
                .set_f32(flag + 4 + 4 * word, 10.0 * (word as f32 + 1.0));
        }
        for lane in 0..4 {
            e.mem.set_f32(ONES_VECTOR + 4 * lane, 1.0);
        }
        e.call(0x00d4_e1e0, &args![this, p.params]);
        // magnitude 5, factor (1 / 5) * 4 rounded to float.
        let factor = ((1.0f64 / 5.0) as f32 as f64 * 4.0) as f32;
        let current = |address: u32| -> [f32; 4] {
            std::array::from_fn(|lane| e.mem.f32(address + 4 * lane as u32))
        };
        let blended = |saved: [f32; 3], old: [f32; 4]| -> [f32; 4] {
            let saved = [saved[0], saved[1], saved[2], 0.0];
            std::array::from_fn(|lane| saved[lane] * (1.0 - factor) + old[lane] * factor)
        };
        let old = |base: f32| -> [f32; 4] { [base, base + 1.0, base + 2.0, base + 3.0] };
        assert_eq!(
            current(p.body_a + 0x10),
            blended([10.0, 20.0, 30.0], old(1.0))
        );
        assert_eq!(
            current(p.body_b + 0x10),
            blended([40.0, 50.0, 60.0], old(100.0))
        );
        assert_eq!(
            current(p.body_a + 0x20),
            blended([70.0, 80.0, 90.0], old(5.0))
        );
        assert_eq!(
            current(p.body_b + 0x20),
            blended([100.0, 110.0, 120.0], old(104.0))
        );
    }
}
