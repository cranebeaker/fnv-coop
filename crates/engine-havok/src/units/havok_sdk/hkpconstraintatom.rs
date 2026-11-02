//! `Havok SDK/hkpconstraintatom.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::sse::{add, addps, mulps, subps};

// Translated from 00d22030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpIgnoreModifierConstraintAtom::getConstraintInfo` (named by the exe's symbol table; the
/// atom type it handles, 0x1b, is the jump-table target of `00d22090`). `this` is an atom
/// (ECX); `info` is a four-word constraint-info record. Follows the chain through the word at
/// +0x14 while the type word is at least `0x17` (modifier atoms), remembering the word at
/// +0x12 of the last modifier; when the atom it ends on is not type `0x16` or `1`, asks
/// `00d21a60(atom, word, local record)` for that atom's sizes and subtracts the record's
/// words 1 to 3 from words 1 to 3 of `info`. Returns 0.
pub fn fn_00d22030(e: &mut Engine, this: Ptr, info: Ptr) -> u32 {
    let mut atom = this.addr();
    let mut word = 0u32;
    if e.mem.u16(atom) >= 0x17 {
        loop {
            word = u32::from(e.mem.u16(atom + 0x12));
            atom = e.mem.u32(atom + 0x14);
            if e.mem.u16(atom) < 0x17 {
                break;
            }
        }
    }
    let kind = e.mem.u16(atom);
    if kind != 0x16 && kind != 1 {
        e.with_stack(0x18, |e, local| {
            e.call(0x00d2_1a60, &args![atom, word, local]);
            for offset in [0xc, 8, 4] {
                let value = e.mem.u32(info.addr() + offset);
                let taken = e.mem.u32(local.addr() + offset);
                e.mem
                    .set_u32(info.addr() + offset, value.wrapping_sub(taken));
            }
        });
    }
    0
}

// Translated from 00d22090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpModifierConstraintAtom::addModifierDataToConstraintInfo` (Xbox PDB). `this` is the atom
/// (ECX), `info` a constraint-info record, `flags` points to a flag byte. By atom type word:
/// `0x17` sets flag 8; `0x18` adds `0x50` to `info` word 1 and sets flag 1; `0x19` sets flag
/// 0x10; `0x1a` adds `0x50` to word 1 and sets flag 4; `0x1b` runs `00d22030(atom, info)`; `0x1c`
/// adds `0xd0` to word 1 and sets flag 2; any other type does nothing. Returns 0 in every case.
pub fn hkp_modifier_constraint_atom_add_modifier_data_to_constraint_info(
    e: &mut Engine,
    this: Ptr,
    info: Ptr,
    flags: Ptr,
) -> u32 {
    let index = u32::from(e.mem.u16(this.addr())).wrapping_sub(0x17);
    let (add, flag) = match index {
        0 => (0, 8),
        1 => (0x50, 1),
        2 => (0, 0x10),
        3 => (0x50, 4),
        4 => return fn_00d22030(e, this, info),
        5 => (0xd0, 2),
        _ => return 0,
    };
    if add != 0 {
        let value = e.mem.u32(info.addr() + 4);
        e.mem.set_u32(info.addr() + 4, value.wrapping_add(add));
    }
    let byte = e.mem.u8(flags.addr());
    e.mem.set_u8(flags.addr(), byte | flag);
    0
}

// Translated from 00d22120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpModifierConstraintAtom::addAllModifierDataToConstraintInfo` (Xbox PDB), cdecl
/// `(atom, info, flags)`: while the atom's type word is at least `0x17` it runs
/// `addModifierDataToConstraintInfo(atom, info, flags)` and moves on to the atom at +0x14,
/// returning the OR of the results.
pub fn hkp_modifier_constraint_atom_add_all_modifier_data_to_constraint_info(
    e: &mut Engine,
    atom: Ptr,
    info: Ptr,
    flags: Ptr,
) -> u32 {
    let mut result = 0;
    let mut atom = atom.addr();
    if e.mem.u16(atom) >= 0x17 {
        loop {
            result |= hkp_modifier_constraint_atom_add_modifier_data_to_constraint_info(
                e,
                Ptr::new(atom),
                info,
                flags,
            );
            atom = e.mem.u32(atom + 0x14);
            if e.mem.u16(atom) < 0x17 {
                break;
            }
        }
    }
    result
}

type Lanes = [f32; 4];

/// A `MOVAPS` load of four lanes.
fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

/// A `MOVAPS` store of four lanes.
fn store_lanes(e: &mut Engine, addr: u32, lanes: Lanes) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// `FLD dword` / `FMUL dword` / `FSTP dword` of two floats given as bits: both operands are
/// quieted when loaded, a NaN operand wins (the larger significand, then the positive
/// one, when both are NaN), otherwise the product rounded to a float.
fn x87_mul_bits(first: u32, second: u32) -> u32 {
    // Both operands loaded (quiet), the x87 NaN choice (#199); the product of two floats is
    // exact in f64, so it is rounded once.
    x87_store32(x87_mul(x87_load32(first), x87_load32(second)))
}

/// The 16-byte constant the soft-contact blend subtracts its factor from (four lanes).
const BLEND_ONES: u32 = 0x010e_c7f0;
/// The soft-contact modifier's saved vectors: two 32-byte blocks of eight lanes each.
const SAVED_VECTORS_A: u32 = 0x0126_89a0;
const SAVED_VECTORS_B: u32 = 0x0126_89c0;

// Translated from 00d21ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBridgeConstraintAtom::init` (Xbox PDB): `this` is the atom (ECX), `data` the constraint
/// data. Sets `m_buildJacobianFunc` (+4) to the code address `0x00d21cc0` (no function is
/// recorded there in the exe's analysis) and `m_constraintData` (+8) to `data`.
pub fn hkp_bridge_constraint_atom_init(e: &mut Engine, this: Ptr, data: Ptr) {
    e.mem.set_u32(this.addr() + 8, data.addr());
    e.mem.set_u32(this.addr() + 4, 0x00d2_1cc0);
}

// Translated from 00d21d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. `this` is the atom (ECX), then six words (one of them a float
/// the caller stores): the function only returns. It is the target of the dispatch in
/// `00d92fd0` (`hkpSimpleContactConstraintData::collisionResponseEndCallback`) for modifier
/// atom type `0x18` (`TYPE_MODIFIER_MASS_CHANGER`, Xbox PDB), whose begin callback is `00d21e00`.
pub fn fn_00d21d00(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
}

// Translated from 00d21d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. `this` is the atom (ECX); the dispatch in `00d92fd0` calls it
/// for modifier atom type `0x17` (`TYPE_MODIFIER_SOFT_CONTACT`, Xbox PDB), the end callback
/// whose begin callback is `00d21f10`. Of its six words only `first` (word 3) and `second`
/// (word 5) are read: each points to two 16-byte vectors, and every one of them becomes
/// `(ones - f) * saved + current * f`, lane by lane, where `f` is the atom's float at +0x20
/// (loaded and stored through the x87 stack, so a signalling NaN is quieted), `ones` the
/// 16-byte constant at `010ec7f0` and `saved` the matching vector `00d21f10` stored at
/// `012689a0 + 0x10 * n`.
pub fn fn_00d21d10(
    e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    first: Ptr,
    _unused_4: u32,
    second: Ptr,
) {
    let factor = x87_quiet(e.mem.f32(this.addr() + 0x20));
    for (saved, target) in [
        (SAVED_VECTORS_A, first.addr()),
        (SAVED_VECTORS_A + 0x10, first.addr() + 0x10),
        (SAVED_VECTORS_B, second.addr()),
        (SAVED_VECTORS_B + 0x10, second.addr() + 0x10),
    ] {
        let ones = load_lanes(e, BLEND_ONES);
        let weights = load_lanes(e, saved);
        let current = load_lanes(e, target);
        // `SUBPS`, `MULPS`, `ADDPS` with the SSE NaN rules, the destination first.
        let splat = [factor; 4];
        let blended = addps(mulps(subps(ones, splat), weights), mulps(current, splat));
        store_lanes(e, target, blended);
    }
}

// Translated from 00d21de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). `this` is a record (ECX); when `flag` is not
/// zero the byte at +10 becomes `(byte[+9] + byte[+8] + 5) * 4`, all in 8-bit arithmetic.
/// Returns `this` (the exe leaves it in `EAX`).
pub fn fn_00d21de0(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    if flag != 0 {
        let sum = e
            .mem
            .u8(this.addr() + 9)
            .wrapping_add(e.mem.u8(this.addr() + 8))
            .wrapping_add(5);
        e.mem.set_u8(this.addr() + 10, sum.wrapping_mul(4));
    }
    this
}

// Translated from 00d21e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMassChangerModifierConstraintAtom::collisionResponseBeginCallback` (named by the engine
/// map; modifier atom type `0x18`). `this` is the atom (ECX); five words of which only `body_a`
/// (word 1) and `body_b` (word 3) are read. For each body, with the four-float factor at atom
/// +0x20 (body A) or +0x30 (body B): the four floats at body +0x40 are multiplied by the factor
/// lane by lane; then, when the first three lanes of the factor are equal, the function at
/// `00cd4c70` runs on body +0x10 with a pointer to a copy of the factor; otherwise the matrix
/// routines run on that block and on the matrix pointed to by body +0x50 (`00cd4bf0`,
/// `00cd55b0`, the diagonal scaled by the first three factor lanes through the x87 stack,
/// `00cd5610`, `00cd55b0`).
pub fn hkp_mass_changer_modifier_constraint_atom_collision_response_begin_callback(
    e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    body_a: Ptr,
    _unused_2: u32,
    body_b: Ptr,
    _unused_4: u32,
) {
    let mut factor_at = this.addr() + 0x20;
    for body in [body_a.addr(), body_b.addr()] {
        let words: [u32; 4] = std::array::from_fn(|lane| e.mem.u32(factor_at + 4 * lane as u32));
        let factor = words.map(f32::from_bits);
        let scaled = load_lanes(e, body + 0x40);
        store_lanes(
            e,
            body + 0x40,
            std::array::from_fn(|lane| scaled[lane] * factor[lane]),
        );
        let basis = body + 0x10;
        if factor[1] == factor[0] && factor[2] == factor[1] {
            e.with_stack(0x10, |e, copy| {
                for (lane, word) in words.into_iter().enumerate() {
                    e.mem.set_u32(copy.addr() + 4 * lane as u32, word);
                }
                e.call(0x00cd_4c70, &args![basis, copy]);
            });
        } else {
            let matrix_source = e.mem.u32(body + 0x50);
            e.with_stack(0x60, |e, frame| {
                let first = frame.addr();
                let second = frame.addr() + 0x30;
                e.call(0x00cd_4bf0, &args![second, matrix_source, basis]);
                e.call(0x00cd_55b0, &args![first, second, matrix_source]);
                for (offset, word) in [(0u32, words[0]), (0x14, words[1]), (0x28, words[2])] {
                    let diagonal = e.mem.u32(first + offset);
                    e.mem.set_u32(first + offset, x87_mul_bits(diagonal, word));
                }
                e.call(0x00cd_5610, &args![second, first, matrix_source]);
                e.call(0x00cd_55b0, &args![basis, matrix_source, second]);
            });
        }
        factor_at = this.addr() + 0x30;
    }
}

// Translated from 00d21f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The begin callback the dispatch in `00d92830`
/// (`hkpSimpleContactConstraintData::collisionResponseBeginCallback`) calls for modifier atom
/// type `0x17` (`TYPE_MODIFIER_SOFT_CONTACT`, Xbox PDB). `this` is the atom (ECX, unused); of the
/// five words only `source_a` (word 2) and `source_b` (word 4) are read: their two 16-byte
/// vectors each are saved bit for bit in the globals at `012689a0 .. 012689df` that
/// `00d21d10` blends with.
pub fn fn_00d21f10(
    e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    source_a: Ptr,
    _unused_3: u32,
    source_b: Ptr,
) {
    for (to, from) in [
        (SAVED_VECTORS_A, source_a.addr()),
        (SAVED_VECTORS_A + 0x10, source_a.addr() + 0x10),
        (SAVED_VECTORS_B, source_b.addr()),
        (SAVED_VECTORS_B + 0x10, source_b.addr() + 0x10),
    ] {
        for word in 0..4 {
            let value = e.mem.u32(from + 4 * word);
            e.mem.set_u32(to + 4 * word, value);
        }
    }
}

/// `f - dot(vector, f) * f` for the moving-surface modifier: `f` is the atom's four floats at
/// +0x20 (copied bit for bit), `vector` the 16-byte vector at `record + 0x10`; the dot product
/// sums the first three lanes of `vector * f` as `z + (y + x)`.
fn moving_surface_term(e: &Engine, atom: Ptr, record: Ptr) -> Lanes {
    // `MULPS`, `ADDSS`, `SUBPS` with the SSE NaN rules, the destination first.
    let factor = load_lanes(e, atom.addr() + 0x20);
    let vector = load_lanes(e, record.addr() + 0x10);
    let product = mulps(vector, factor);
    let dot = add(product[2], add(product[1], product[0]));
    subps(factor, mulps([dot; 4], factor))
}

// Translated from 00d21f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The begin callback of modifier atom type `0x1a`
/// (`TYPE_MODIFIER_MOVING_SURFACE`, Xbox PDB) in the dispatch of `00d92830`. `this` is the atom
/// (ECX); five words of which `record` (word 0) and `target` (word 4) are read. With the atom's
/// vector `f` at +0x20 and `d = dot3(record[+0x10] * f)`, adds `f - d * f` to the 16-byte vector
/// at `target`, lane by lane.
pub fn fn_00d21f50(
    e: &mut Engine,
    this: Ptr,
    record: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    target: Ptr,
) {
    let term = moving_surface_term(e, this, record);
    let current = load_lanes(e, target.addr());
    store_lanes(e, target.addr(), addps(term, current));
}

// Translated from 00d21fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The end callback of modifier atom type `0x1a`
/// (`TYPE_MODIFIER_MOVING_SURFACE`, Xbox PDB) in the dispatch of `00d92fd0`: the inverse of
/// `00d21f50`. `this` is the atom (ECX); six words of which `record` (word 0) and `target`
/// (word 5) are read; subtracts `f - d * f` from the 16-byte vector at `target`.
pub fn fn_00d21fc0(
    e: &mut Engine,
    this: Ptr,
    record: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    target: Ptr,
) {
    let term = moving_surface_term(e, this, record);
    let current = load_lanes(e, target.addr());
    store_lanes(e, target.addr(), subps(current, term));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_1ce0, hkp_bridge_constraint_atom_init(Ptr, Ptr)),
        entry!(0x00d2_1d00, fn_00d21d00(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x00d2_1d10, fn_00d21d10(Ptr, u32, u32, u32, Ptr, u32, Ptr)),
        entry!(0x00d2_1de0, fn_00d21de0(Ptr, u32) -> Ptr),
        entry!(
            0x00d2_1e00,
            hkp_mass_changer_modifier_constraint_atom_collision_response_begin_callback(
                Ptr, u32, Ptr, u32, Ptr, u32
            )
        ),
        entry!(0x00d2_1f10, fn_00d21f10(Ptr, u32, u32, Ptr, u32, Ptr)),
        entry!(0x00d2_1f50, fn_00d21f50(Ptr, Ptr, u32, u32, u32, Ptr)),
        entry!(0x00d2_1fc0, fn_00d21fc0(Ptr, Ptr, u32, u32, u32, u32, Ptr)),
        entry!(0x00d2_2030, fn_00d22030(Ptr, Ptr) -> u32),
        entry!(0x00d2_2090, hkp_modifier_constraint_atom_add_modifier_data_to_constraint_info(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d2_2120, hkp_modifier_constraint_atom_add_all_modifier_data_to_constraint_info(Ptr, Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    #[test]
    fn modifier_types_update_the_info_and_flags() {
        let mut e = test_engine();
        let info = e.mem.alloc(0x10);
        let flags = e.mem.alloc(4);
        let atom = e.mem.alloc(0x20);
        for (kind, add, flag) in [
            (0x17u16, 0u32, 8u8),
            (0x18, 0x50, 1),
            (0x19, 0, 0x10),
            (0x1a, 0x50, 4),
            (0x1c, 0xd0, 2),
        ] {
            e.mem.set_u32(info + 4, 7);
            e.mem.set_u8(flags, 0x40);
            e.mem.set_u16(atom, kind);
            let got = e.call(0x00d2_2090, &args![atom, info, flags]).u32();
            assert_eq!(got, 0);
            assert_eq!(e.mem.u32(info + 4), 7 + add);
            assert_eq!(e.mem.u8(flags), 0x40 | flag);
        }
        // Other types do nothing.
        e.mem.set_u8(flags, 0);
        e.mem.set_u16(atom, 0x1d);
        e.call(0x00d2_2090, &args![atom, info, flags]);
        e.mem.set_u16(atom, 3);
        e.call(0x00d2_2090, &args![atom, info, flags]);
        assert_eq!(e.mem.u8(flags), 0);
    }

    #[test]
    fn the_ignore_modifier_subtracts_the_sizes_of_the_atom_it_ends_on() {
        let mut e = test_engine();
        let calls: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&calls);
        e.register_double(0x00d2_1a60, move |e, a| {
            sink.borrow_mut().push((a[0], a[1]));
            e.mem.set_u32(a[2] + 4, 1);
            e.mem.set_u32(a[2] + 8, 2);
            e.mem.set_u32(a[2] + 0xc, 3);
            Ret::default()
        });
        let info = e.mem.alloc(0x10);
        for (i, v) in [100u32, 100, 100, 100].into_iter().enumerate() {
            e.mem.set_u32(info + 4 * i as u32, v);
        }
        let last = e.mem.alloc(0x20);
        e.mem.set_u16(last, 5);
        let modifier = e.mem.alloc(0x20);
        e.mem.set_u16(modifier, 0x1b);
        e.mem.set_u16(modifier + 0x12, 0x77);
        e.mem.set_u32(modifier + 0x14, last);
        let got = e.call(0x00d2_2030, &args![modifier, info]).u32();
        assert_eq!(got, 0);
        assert_eq!(*calls.borrow(), vec![(last, 0x77)]);
        assert_eq!(e.mem.u32(info), 100);
        assert_eq!(e.mem.u32(info + 4), 99);
        assert_eq!(e.mem.u32(info + 8), 98);
        assert_eq!(e.mem.u32(info + 0xc), 97);
        // Ending on type 0x16 or 1 asks nothing.
        calls.borrow_mut().clear();
        e.mem.set_u16(last, 0x16);
        e.call(0x00d2_2030, &args![modifier, info]);
        e.mem.set_u16(last, 1);
        e.call(0x00d2_2030, &args![modifier, info]);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn all_modifiers_walks_the_chain() {
        let mut e = test_engine();
        let info = e.mem.alloc(0x10);
        let flags = e.mem.alloc(4);
        let end = e.mem.alloc(0x20);
        e.mem.set_u16(end, 2);
        let second = e.mem.alloc(0x20);
        e.mem.set_u16(second, 0x19);
        e.mem.set_u32(second + 0x14, end);
        let first = e.mem.alloc(0x20);
        e.mem.set_u16(first, 0x17);
        e.mem.set_u32(first + 0x14, second);
        e.call(0x00d2_2120, &args![first, info, flags]);
        assert_eq!(e.mem.u8(flags), 0x18);
        // A plain atom is left alone.
        e.mem.set_u8(flags, 0);
        e.call(0x00d2_2120, &args![end, info, flags]);
        assert_eq!(e.mem.u8(flags), 0);
    }
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn put_lanes(e: &mut Engine, addr: u32, lanes: [f32; 4]) {
        for (lane, value) in lanes.into_iter().enumerate() {
            e.mem.set_f32(addr + 4 * lane as u32, value);
        }
    }

    fn lanes_at(e: &Engine, addr: u32) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
    }

    #[test]
    fn bridge_init_stores_the_data_and_the_build_function() {
        let mut e = engine();
        let atom = e.mem.alloc(0x10);
        e.call(0x00d2_1ce0, &args![atom, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(atom + 8), 0x1234_5678);
        assert_eq!(e.mem.u32(atom + 4), 0x00d2_1cc0);
        assert_eq!(e.mem.u32(atom), 0);
    }

    #[test]
    fn mass_changer_end_callback_does_nothing() {
        let mut e = engine();
        let atom = e.mem.alloc(0x40);
        e.mem.set_u32(atom + 0x20, 0x3f80_0000);
        let ret = e.call(
            0x00d2_1d00,
            &args![atom, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32],
        );
        assert_eq!(ret.u32(), 0);
        assert_eq!(e.mem.u32(atom + 0x20), 0x3f80_0000);
    }

    #[test]
    fn soft_contact_saves_the_vectors_and_blends_with_them() {
        let mut e = engine();
        put_lanes(&mut e, 0x010e_c7f0, [1.0; 4]);
        let atom = e.mem.alloc(0x40);
        let source_a = e.mem.alloc(0x20);
        let source_b = e.mem.alloc(0x20);
        let rows = [
            [4.0, 8.0, 12.0, 16.0],
            [2.0, 4.0, 6.0, 8.0],
            [1.0, 3.0, 5.0, 7.0],
            [0.5, 1.5, 2.5, 3.5],
        ];
        put_lanes(&mut e, source_a, rows[0]);
        put_lanes(&mut e, source_a + 0x10, rows[1]);
        put_lanes(&mut e, source_b, rows[2]);
        put_lanes(&mut e, source_b + 0x10, rows[3]);
        e.call(
            0x00d2_1f10,
            &args![atom, 0u32, 0u32, source_a, 0u32, source_b],
        );
        for (index, row) in rows.iter().enumerate() {
            assert_eq!(lanes_at(&e, 0x0126_89a0 + 0x10 * index as u32), *row);
        }

        e.mem.set_f32(atom + 0x20, 0.25);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        let currents = [
            [8.0, 8.0, 8.0, 8.0],
            [4.0, 4.0, 4.0, 4.0],
            [16.0, 0.0, -16.0, 32.0],
            [-4.0, 4.0, -8.0, 8.0],
        ];
        put_lanes(&mut e, first, currents[0]);
        put_lanes(&mut e, first + 0x10, currents[1]);
        put_lanes(&mut e, second, currents[2]);
        put_lanes(&mut e, second + 0x10, currents[3]);
        e.call(
            0x00d2_1d10,
            &args![atom, 0u32, 0u32, 0u32, first, 0u32, second],
        );
        let targets = [first, first + 0x10, second, second + 0x10];
        for index in 0..4 {
            let expected: [f32; 4] = std::array::from_fn(|lane| {
                (1.0f32 - 0.25) * rows[index][lane] + currents[index][lane] * 0.25
            });
            assert_eq!(lanes_at(&e, targets[index]), expected);
        }
        assert_eq!(lanes_at(&e, first), [5.0, 8.0, 11.0, 14.0]);
    }

    #[test]
    fn record_byte_is_computed_only_for_a_nonzero_flag() {
        let mut e = engine();
        let record = e.mem.alloc(0x10);
        e.mem.set_u8(record + 8, 0x40);
        e.mem.set_u8(record + 9, 0x40);
        e.mem.set_u8(record + 10, 0x77);
        assert_eq!(e.call(0x00d2_1de0, &args![record, 0u32]).u32(), record);
        assert_eq!(e.mem.u8(record + 10), 0x77);
        assert_eq!(e.call(0x00d2_1de0, &args![record, 1u32]).u32(), record);
        // (0x40 + 0x40 + 5) * 4 = 0x214, kept in 8 bits.
        assert_eq!(e.mem.u8(record + 10), 0x14);
    }

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles for the five matrix routines the mass changer calls; the first `00cd55b0` call
    /// writes the diagonal 2, 3, 4 into its output, `00cd5610` records the diagonal it is given.
    fn matrix_doubles(e: &mut Engine) -> (CallLog, Rc<RefCell<Vec<f32>>>) {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let diagonal = Rc::new(RefCell::new(Vec::new()));
        for address in [0x00cd_4c70, 0x00cd_4bf0] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let sink = Rc::clone(&log);
        let mut multiplies = 0;
        e.register_double(0x00cd_55b0, move |e, a| {
            sink.borrow_mut().push((0x00cd_55b0, a.to_vec()));
            multiplies += 1;
            if multiplies == 1 {
                e.mem.set_f32(a[0], 2.0);
                e.mem.set_f32(a[0] + 0x14, 3.0);
                e.mem.set_f32(a[0] + 0x28, 4.0);
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        let seen = Rc::clone(&diagonal);
        e.register_double(0x00cd_5610, move |e, a| {
            sink.borrow_mut().push((0x00cd_5610, a.to_vec()));
            seen.borrow_mut()
                .extend([0, 0x14, 0x28].map(|offset| e.mem.f32(a[1] + offset)));
            Ret::default()
        });
        (log, diagonal)
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn mass_changer_scales_the_body_and_picks_the_uniform_or_matrix_path() {
        let mut e = engine();
        let (log, diagonal) = matrix_doubles(&mut e);
        let atom = e.mem.alloc(0x40);
        put_lanes(&mut e, atom + 0x20, [1.0, 2.0, 3.0, 4.0]);
        put_lanes(&mut e, atom + 0x30, [0.5, 0.5, 0.5, 2.0]);
        let body_a = e.mem.alloc(0x60);
        let body_b = e.mem.alloc(0x60);
        for body in [body_a, body_b] {
            put_lanes(&mut e, body + 0x40, [1.0, 2.0, 3.0, 4.0]);
        }
        e.mem.set_u32(body_a + 0x50, 0xaaaa_0000);
        e.call(0x00d2_1e00, &args![atom, 0u32, body_a, 0u32, body_b, 0u32]);
        // Body A: factors differ, so the matrix path; the lanes at +0x40 are multiplied.
        assert_eq!(lanes_at(&e, body_a + 0x40), [1.0, 4.0, 9.0, 16.0]);
        let make = calls_to(&log, 0x00cd_4bf0);
        assert_eq!(make.len(), 1);
        let scratch = make[0][0];
        assert_eq!(make[0][1..], [0xaaaa_0000, body_a + 0x10]);
        let products = calls_to(&log, 0x00cd_55b0);
        assert_eq!(products.len(), 2);
        assert_eq!(products[0][1..], [scratch, 0xaaaa_0000]);
        assert_eq!(products[1], [body_a + 0x10, 0xaaaa_0000, scratch]);
        let inverse = calls_to(&log, 0x00cd_5610);
        assert_eq!(inverse, [[scratch, products[0][0], 0xaaaa_0000]]);
        // The diagonal 2, 3, 4 was scaled by the first three factor lanes 1, 2, 3.
        assert_eq!(*diagonal.borrow(), [2.0, 6.0, 12.0]);
        // Body B: equal factors, the uniform path with a copy of the four lanes.
        assert_eq!(lanes_at(&e, body_b + 0x40), [0.5, 1.0, 1.5, 8.0]);
        let uniform = calls_to(&log, 0x00cd_4c70);
        assert_eq!(uniform.len(), 1);
        assert_eq!(uniform[0][0], body_b + 0x10);
    }

    #[test]
    fn mass_changer_treats_a_nan_factor_as_not_equal() {
        let mut e = engine();
        let (log, _) = matrix_doubles(&mut e);
        let atom = e.mem.alloc(0x40);
        let nan = f32::NAN;
        put_lanes(&mut e, atom + 0x20, [nan, nan, nan, 1.0]);
        put_lanes(&mut e, atom + 0x30, [2.0, 2.0, 2.0, 1.0]);
        let body_a = e.mem.alloc(0x60);
        let body_b = e.mem.alloc(0x60);
        e.call(0x00d2_1e00, &args![atom, 0u32, body_a, 0u32, body_b, 0u32]);
        assert_eq!(calls_to(&log, 0x00cd_4bf0).len(), 1);
        assert_eq!(calls_to(&log, 0x00cd_4c70).len(), 1);
    }

    #[test]
    fn moving_surface_adds_and_removes_the_same_term() {
        let mut e = engine();
        let atom = e.mem.alloc(0x40);
        put_lanes(&mut e, atom + 0x20, [0.5, 0.25, 0.5, 1.0]);
        let record = e.mem.alloc(0x20);
        put_lanes(&mut e, record + 0x10, [1.0, 2.0, 4.0, 8.0]);
        let target = e.mem.alloc(0x10);
        put_lanes(&mut e, target, [10.0, 20.0, 30.0, 40.0]);
        // dot = 2 + (0.5 + 0.5) = 3, term = f - 3 f = -2 f.
        e.call(0x00d2_1f50, &args![atom, record, 0u32, 0u32, 0u32, target]);
        assert_eq!(lanes_at(&e, target), [9.0, 19.5, 29.0, 38.0]);
        e.call(
            0x00d2_1fc0,
            &args![atom, record, 0u32, 0u32, 0u32, 0u32, target],
        );
        assert_eq!(lanes_at(&e, target), [10.0, 20.0, 30.0, 40.0]);
    }
}
