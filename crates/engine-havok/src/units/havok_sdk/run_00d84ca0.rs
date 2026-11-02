//! `Havok SDK/run_00d84ca0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The constraint-data type groups the `00d84ca0` family of functions tell apart through the
/// jump tables at `00d84ce8`, `00d84d7c`, `00d84e94`, `00d84ff8` and `00d8511c` (all five
/// tables agree). The type numbers are the exe's; their names are not confirmed.
enum Kind {
    /// Types 0, 8 and 15.
    Translation,
    /// Types 1, 2, 6, 7, 9 and 14.
    Rotation,
    /// Type 12: the wrapped data is the pointer at `+0x18`.
    Wrapper,
    /// Type 13: the wrapped data is what `00cdf6d0`/`00cdf6c0` (the word at `+0xc`) answers.
    Malleable,
    /// Types 3, 4, 5, 10, 11 and everything above 15.
    Nothing,
}

fn kind(constraint_type: u32) -> Kind {
    match constraint_type {
        0 | 8 | 15 => Kind::Translation,
        1 | 2 | 6 | 7 | 9 | 14 => Kind::Rotation,
        12 => Kind::Wrapper,
        13 => Kind::Malleable,
        _ => Kind::Nothing,
    }
}

/// The slot `+0x20` of the data object's vtable: its constraint type.
fn constraint_type(e: &mut Engine, data: u32) -> u32 {
    e.vcall(data, 0x20, &args![]).u32()
}

/// `hkpMalleableConstraintData::getWrappedConstraintData` (engine map name, `00cdf6c0`) and its
/// twin `00cdf6d0` (identical code): `thiscall`, the word at `this + 0xc`.
const GET_WRAPPED_DATA: u32 = 0x00cd_f6c0;
const GET_WRAPPED_DATA_TWIN: u32 = 0x00cd_f6d0;
/// The 16-byte vector at `01267e30` the getters answer for types without a pivot (a pointer
/// to it).
const DEFAULT_PIVOT: u32 = 0x0126_7e30;
/// `00c6cd60` (`bhkcharactercontroller.obj` in the engine map): `thiscall`, one word; copies a
/// rotation from the pointer it is given into `this`.
const COPY_ROTATION: u32 = 0x00c6_cd60;
/// `hkpSphereShape::hkpSphereShape` (engine map name): `thiscall`, one float (the radius).
const SPHERE_SHAPE_CONSTRUCT: u32 = 0x00cd_8160;
/// `00cbfba0` (range `Havok SDK`, no name): `thiscall`, no stack words.
const COLLECTOR_CONSTRUCT: u32 = 0x00cb_fba0;
/// `00cfa860` (range `Havok SDK`, no name): `thiscall`, no stack words; destroys the object
/// `00cbfba0` constructed.
const COLLECTOR_DESTRUCT: u32 = 0x00cf_a860;
/// The sphere radius `0.01` (`01013ea4`), the large float `0x7f7fffee` (`0104da48`) and the
/// float `0x7effffee` (`010cc3b0`) the query passes.
const SPHERE_RADIUS: u32 = 0x0101_3ea4;
const QUERY_DISTANCE: u32 = 0x0104_da48;
const QUERY_TOLERANCE: u32 = 0x010c_c3b0;
/// The default transform (`01268370`) and the vtable (`010c476c`) the query structure points to.
const DEFAULT_TRANSFORM: u32 = 0x0126_8370;
const QUERY_OUTPUT_VTABLE: u32 = 0x010c_476c;
/// The five words stored at `+0x30`, `+0x38`, `+0x3c`, `+0x40` and `+0x44` of the frame.
const COLLECTOR_TABLE: [u32; 5] = [
    0x010c_646c,
    0x010c_6460,
    0x010c_644c,
    0x010c_6440,
    0x010c_6434,
];

/// A raw 16-byte copy (`MOVAPS` load and store).
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for offset in [0u32, 4, 8, 12] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

/// The part of constraint data `00d84ca0` and `00d84d30` look for: follows wrappers (type 12
/// through the word at `+0x18`, type 13 through `00cdf6d0`) until the type is a
/// translation-like one (`data + translation_offset`), a rotation-like one
/// (`data + rotation_offset`) or without a part (the vector at `01267e30`).
fn locate_part(
    e: &mut Engine,
    mut data: u32,
    translation_offset: u32,
    rotation_offset: u32,
) -> u32 {
    let mut ty = constraint_type(e, data);
    loop {
        match kind(ty) {
            Kind::Translation => return data.wrapping_add(translation_offset),
            Kind::Rotation => return data.wrapping_add(rotation_offset),
            Kind::Nothing => return DEFAULT_PIVOT,
            Kind::Wrapper => data = e.mem.u32(data.wrapping_add(0x18)),
            Kind::Malleable => data = e.call(GET_WRAPPED_DATA_TWIN, &args![data]).u32(),
        }
        ty = constraint_type(e, data);
    }
}

// Translated from 00d84ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(data)`: the pointer to the first pivot-like block of a constraint's data: `data +
/// 0x20` for the translation-like types, `data + 0x50` for the rotation-like ones, after
/// unwrapping wrapper types; the vector at `01267e30` when the type has none. The class is
/// unconfirmed (`hkpconstraintinstance.rs` calls it for pivot A).
pub fn fn_00d84ca0(e: &mut Engine, data: u32) -> u32 {
    locate_part(e, data, 0x20, 0x50)
}

// Translated from 00d84d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00d84ca0` with the second block: `data + 0x30` for the translation-like types and
/// `data + 0x90` for the rotation-like ones (`hkpconstraintinstance.rs` calls it for pivot B).
pub fn fn_00d84d30(e: &mut Engine, data: u32) -> u32 {
    locate_part(e, data, 0x30, 0x90)
}

// Translated from 00d84dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(data, vector, index)`: stores the 16-byte vector at `vector` into a constraint's
/// data after unwrapping wrapper types (type 12 through the word at `+0x18`, type 13 through
/// `00cdf6c0`): at `data + 16 * (index + 2)` for the translation-like types and
/// `data + 64 * index + 0x50` for the rotation-like ones; the other types store nothing.
pub fn fn_00d84dc0(e: &mut Engine, data: u32, vector: u32, index: u32) {
    let mut data = data;
    let mut ty = constraint_type(e, data);
    loop {
        match kind(ty) {
            Kind::Translation => {
                copy_vector(e, vector, data.wrapping_add(index.wrapping_add(2) << 4));
                return;
            }
            Kind::Rotation => {
                copy_vector(e, vector, data.wrapping_add(index << 6).wrapping_add(0x50));
                return;
            }
            Kind::Nothing => return,
            Kind::Wrapper => data = e.mem.u32(data.wrapping_add(0x18)),
            Kind::Malleable => data = e.call(GET_WRAPPED_DATA, &args![data]).u32(),
        }
        ty = constraint_type(e, data);
    }
}

/// Sets the 4x4 block at `out` to the identity (three 16-byte columns with a 1.0 on the
/// diagonal) and zeroes the translation vector at `out + 0x30`.
fn reset_to_identity(e: &mut Engine, out: u32) {
    for offset in 0..12u32 {
        e.mem.set_u32(out.wrapping_add(4 * offset), 0);
    }
    e.mem.set_f32(out, 1.0);
    e.mem.set_f32(out.wrapping_add(0x14), 1.0);
    e.mem.set_f32(out.wrapping_add(0x28), 1.0);
    for offset in 0..4u32 {
        e.mem.set_u32(out.wrapping_add(0x30 + 4 * offset), 0);
    }
}

// Translated from 00d84ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(data, index, out)`: reads a constraint's frame into the 4x4 block at `out`. It is
/// first set to the identity with a zero translation; then, after unwrapping wrapper types
/// (type 12 through the word at `+0x18`, type 13 through `00cdf6d0`; the identity is written
/// again after each step), the translation-like types copy the vector at
/// `data + 16 * (index + 2)` into `out + 0x30` and the rotation-like types hand
/// `data + 64 * index + 0x20` to `00c6cd60` on `out`; the other types leave the identity.
pub fn fn_00d84ee0(e: &mut Engine, data: u32, index: u32, out: u32) {
    let mut data = data;
    reset_to_identity(e, out);
    let mut ty = constraint_type(e, data);
    loop {
        match kind(ty) {
            Kind::Translation => {
                copy_vector(
                    e,
                    data.wrapping_add(index.wrapping_add(2) << 4),
                    out.wrapping_add(0x30),
                );
                return;
            }
            Kind::Rotation => {
                let source = data.wrapping_add(index << 6).wrapping_add(0x20);
                e.call(COPY_ROTATION, &args![out, source]);
                return;
            }
            Kind::Nothing => return,
            Kind::Wrapper => {
                data = e.mem.u32(data.wrapping_add(0x18));
                reset_to_identity(e, out);
            }
            Kind::Malleable => {
                data = e.call(GET_WRAPPED_DATA_TWIN, &args![data]).u32();
                reset_to_identity(e, out);
            }
        }
        ty = constraint_type(e, data);
    }
}

// Translated from 00d85040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(data, transform, index)`: the writing counterpart of `00d84ee0`. After unwrapping
/// wrapper types (type 12 through the word at `+0x18`, type 13 through `00cdf6c0`) the
/// translation-like types store the vector at `transform + 0x30` at `data + 16 * (index + 2)`
/// and the rotation-like types call `00c6cd60` on `data + 64 * index + 0x20` with `transform`;
/// the other types do nothing.
pub fn fn_00d85040(e: &mut Engine, data: u32, transform: u32, index: u32) {
    let mut data = data;
    let mut ty = constraint_type(e, data);
    loop {
        match kind(ty) {
            Kind::Translation => {
                copy_vector(
                    e,
                    transform.wrapping_add(0x30),
                    data.wrapping_add(index.wrapping_add(2) << 4),
                );
                return;
            }
            Kind::Rotation => {
                let target = data.wrapping_add(index << 6).wrapping_add(0x20);
                e.call(COPY_ROTATION, &args![target, transform]);
                return;
            }
            Kind::Nothing => return,
            Kind::Wrapper => data = e.mem.u32(data.wrapping_add(0x18)),
            Kind::Malleable => data = e.call(GET_WRAPPED_DATA, &args![data]).u32(),
        }
        ty = constraint_type(e, data);
    }
}

// Translated from 00d85160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(world, body, position)`: asks the collision dispatcher whether a small sphere at the
/// 16-byte `position` touches `body`. The 0x1a0-byte stack frame holds, as the exe lays it out
/// (offsets from the frame start, the stack pointer after the sphere constructor): the body
/// record `+0x10` (the body, -1, the default transform `01268370`, 0, a pointer to the sphere,
/// -1, a pointer to an identity rotation at `+0x60`, 0), the sphere shape at `+0xa0` (radius
/// `0.01`, built by `hkpSphereShape::hkpSphereShape`), its position copied from `position` at
/// `+0x90`, a result record at `+0xc0` (vtable `010c476c`, the float `0x7f7fffee` at `+0xc4`
/// and `+0xec`, zero at `+0xf0`), the 0x50 bytes at `[world + 0x78]` copied to `+0x140` with the
/// float at `010cc3b0` over its third word, and an object constructed at `+0x30` by `00cbfba0`
/// and given five table words and 1 at `+0x50`. The dispatcher function is the pointer at
/// `dispatcher + 0x9a8 + 0x14 * m`, with `dispatcher` the word at `[world + 0x80]` and `m` the
/// byte at `dispatcher + 32 * (shape type + 0xd) + [body + 0xc]` (the shape type is the word the
/// sphere constructor leaves at `+0xc`); it is called with `(frame + 0x20, frame + 0x10,
/// frame + 0x140, frame + 0xc0)` (cdecl). Afterwards the object at `+0x30` is destroyed
/// (`00cfa860`) and the answer is the word at `+0x11c` when the word at `+0xf0` is not zero,
/// otherwise 0.
pub fn fn_00d85160(e: &mut Engine, world: u32, body: u32, position: u32) -> u32 {
    let radius = x87_quiet_bits(e.mem.u32(SPHERE_RADIUS));
    e.with_stack(0x1a0, |e, frame| {
        let frame = frame.addr();
        e.call(
            SPHERE_SHAPE_CONSTRUCT,
            &args![frame + 0xa0, f32::from_bits(radius)],
        );
        copy_vector(e, position, frame + 0x90);
        let shape_type = e.mem.u32(frame + 0xac);
        for offset in 0..4u32 {
            e.mem.set_u32(frame + 0x60 + 4 * offset, 0);
        }
        e.mem.set_f32(frame + 0x60, 1.0);
        for offset in 0..4u32 {
            e.mem.set_u32(frame + 0x70 + 4 * offset, 0);
        }
        e.mem.set_f32(frame + 0x74, 1.0);
        e.mem.set_u32(frame + 0x28, frame + 0x60);
        for offset in 0..4u32 {
            e.mem.set_u32(frame + 0x80 + 4 * offset, 0);
        }
        e.mem.set_f32(frame + 0x88, 1.0);
        let distance = x87_quiet_bits(e.mem.u32(QUERY_DISTANCE));
        e.mem.set_u32(frame + 0x24, 0xffff_ffff);
        e.mem.set_u32(frame + 0x14, 0xffff_ffff);
        e.mem.set_u32(frame + 0xec, distance);
        e.mem.set_u32(frame + 0xc4, distance);
        let tolerance = x87_quiet_bits(e.mem.u32(QUERY_TOLERANCE));
        e.mem.set_u32(frame + 0x20, frame + 0xa0);
        e.mem.set_u32(frame + 0x2c, 0);
        e.mem.set_u32(frame + 0x10, body);
        e.mem.set_u32(frame + 0x1c, 0);
        e.mem.set_u32(frame + 0xf0, 0);
        let dispatcher = e.mem.u32(world.wrapping_add(0x80));
        let source = e.mem.u32(world.wrapping_add(0x78));
        let table_row = shape_type
            .wrapping_add(0xd)
            .wrapping_shl(5)
            .wrapping_add(e.mem.u32(body.wrapping_add(0xc)));
        e.mem.set_u32(frame + 0x18, DEFAULT_TRANSFORM);
        e.mem.set_u32(frame + 0xc0, QUERY_OUTPUT_VTABLE);
        let selector = u32::from(e.mem.u8(table_row.wrapping_add(dispatcher)));
        let function = e.mem.u32(
            dispatcher
                .wrapping_add(selector.wrapping_mul(5).wrapping_mul(4))
                .wrapping_add(0x9a8),
        );
        for word in 0..0x14u32 {
            let value = e.mem.u32(source.wrapping_add(4 * word));
            e.mem.set_u32(frame + 0x140 + 4 * word, value);
        }
        e.mem.set_u32(frame + 0x148, tolerance);
        e.call(COLLECTOR_CONSTRUCT, &args![frame + 0x30]);
        e.mem.set_u32(frame + 0x14c, frame + 0x30);
        e.mem.set_u32(frame + 0x30, COLLECTOR_TABLE[0]);
        e.mem.set_u32(frame + 0x38, COLLECTOR_TABLE[1]);
        e.mem.set_u32(frame + 0x3c, COLLECTOR_TABLE[2]);
        e.mem.set_u32(frame + 0x40, COLLECTOR_TABLE[3]);
        e.mem.set_u32(frame + 0x44, COLLECTOR_TABLE[4]);
        e.mem.set_u32(frame + 0x50, 1);
        e.call(
            function,
            &args![frame + 0x20, frame + 0x10, frame + 0x140, frame + 0xc0],
        );
        if e.mem.u32(frame + 0xf0) != 0 {
            let answer = e.mem.u32(frame + 0x11c);
            e.call(COLLECTOR_DESTRUCT, &args![frame + 0x30]);
            answer
        } else {
            e.call(COLLECTOR_DESTRUCT, &args![frame + 0x30]);
            0
        }
    })
}

// Translated from 00d85300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall(this, source)`, `RET 4`: copies a 0x90-byte record: the 16-bit word at `+0`, then
/// the eight 16-byte vectors at `+0x10` to `+0x80`. Returns `this`. The class is unconfirmed
/// (`hkpconstraintdatacloningutil.rs` calls it as the atoms' base copy).
pub fn fn_00d85300(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let word = e.mem.u16(source);
    e.mem.set_u16(this.addr(), word);
    for offset in (0x10..=0x80u32).step_by(0x10) {
        copy_vector(
            e,
            source.wrapping_add(offset),
            this.addr().wrapping_add(offset),
        );
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d8_4ca0, fn_00d84ca0(u32) -> u32),
        entry!(0x00d8_4d30, fn_00d84d30(u32) -> u32),
        entry!(0x00d8_4dc0, fn_00d84dc0(u32, u32, u32)),
        entry!(0x00d8_4ee0, fn_00d84ee0(u32, u32, u32)),
        entry!(0x00d8_5040, fn_00d85040(u32, u32, u32)),
        entry!(0x00d8_5160, fn_00d85160(u32, u32, u32) -> u32),
        entry!(0x00d8_5300, fn_00d85300(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const TYPE_SLOT: u32 = 0x0900_0020;
    const AGENT_FUNCTION: u32 = 0x0900_0100;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        // Every data object shares one vtable whose slot 0x20 answers the type kept at +0x1c.
        e.put_vtable(0x0900_0000, &[0; 9]);
        e.mem.set_u32(0x0900_0000 + 0x20, TYPE_SLOT);
        e.register(TYPE_SLOT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x1c),
            ..Ret::default()
        });
        e
    }

    /// A constraint-data object of type `ty` with recognisable words at the offsets used.
    fn data(e: &mut Engine, ty: u32) -> u32 {
        let data = e.mem.alloc(0x200);
        e.mem.set_u32(data, 0x0900_0000);
        e.mem.set_u32(data + 0x1c, ty);
        data
    }

    fn put_vector(e: &mut Engine, address: u32, base: u32) {
        for lane in 0..4u32 {
            e.mem.set_u32(address + 4 * lane, base + lane);
        }
    }

    fn read_vector(e: &Engine, address: u32) -> [u32; 4] {
        std::array::from_fn(|lane| e.mem.u32(address + 4 * lane as u32))
    }

    #[test]
    fn locators_pick_the_block_by_type_and_default_for_the_rest() {
        let mut e = engine();
        for (ty, first, second) in [
            (0u32, 0x20u32, 0x30u32),
            (8, 0x20, 0x30),
            (15, 0x20, 0x30),
            (1, 0x50, 0x90),
            (2, 0x50, 0x90),
            (6, 0x50, 0x90),
            (7, 0x50, 0x90),
            (9, 0x50, 0x90),
            (14, 0x50, 0x90),
        ] {
            let object = data(&mut e, ty);
            assert_eq!(e.call(0x00d8_4ca0, &args![object]).u32(), object + first);
            assert_eq!(e.call(0x00d8_4d30, &args![object]).u32(), object + second);
        }
        for ty in [3u32, 4, 5, 10, 11, 16, 0xffff] {
            let object = data(&mut e, ty);
            assert_eq!(e.call(0x00d8_4ca0, &args![object]).u32(), DEFAULT_PIVOT);
            assert_eq!(e.call(0x00d8_4d30, &args![object]).u32(), DEFAULT_PIVOT);
        }
    }

    #[test]
    fn locators_follow_both_kinds_of_wrapper() {
        let mut e = engine();
        e.register(GET_WRAPPED_DATA_TWIN, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        let inner = data(&mut e, 1);
        let wrapper = data(&mut e, 12);
        e.mem.set_u32(wrapper + 0x18, inner);
        assert_eq!(e.call(0x00d8_4ca0, &args![wrapper]).u32(), inner + 0x50);
        let malleable = data(&mut e, 13);
        e.mem.set_u32(malleable + 0xc, wrapper);
        assert_eq!(e.call(0x00d8_4d30, &args![malleable]).u32(), inner + 0x90);
        // A wrapper around data without a part answers the default.
        let empty = data(&mut e, 3);
        e.mem.set_u32(wrapper + 0x18, empty);
        assert_eq!(e.call(0x00d8_4ca0, &args![wrapper]).u32(), DEFAULT_PIVOT);
    }

    #[test]
    fn store_vector_writes_to_the_slot_the_type_selects() {
        let mut e = engine();
        let vector = e.mem.alloc(16);
        put_vector(&mut e, vector, 0x100);
        let translation = data(&mut e, 0);
        e.call(0x00d8_4dc0, &args![translation, vector, 3u32]);
        assert_eq!(
            read_vector(&e, translation + 0x50),
            [0x100, 0x101, 0x102, 0x103]
        );
        let rotation = data(&mut e, 14);
        e.call(0x00d8_4dc0, &args![rotation, vector, 2u32]);
        assert_eq!(
            read_vector(&e, rotation + 0xd0),
            [0x100, 0x101, 0x102, 0x103]
        );
        let nothing = data(&mut e, 5);
        e.call(0x00d8_4dc0, &args![nothing, vector, 1u32]);
        assert_eq!(read_vector(&e, nothing + 0x30), [0, 0, 0, 0]);
        assert_eq!(read_vector(&e, nothing + 0x90), [0, 0, 0, 0]);
    }

    #[test]
    fn store_vector_unwraps_wrappers_first() {
        let mut e = engine();
        e.register(GET_WRAPPED_DATA, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        let vector = e.mem.alloc(16);
        put_vector(&mut e, vector, 0x200);
        let inner = data(&mut e, 8);
        let wrapper = data(&mut e, 12);
        e.mem.set_u32(wrapper + 0x18, inner);
        let malleable = data(&mut e, 13);
        e.mem.set_u32(malleable + 0xc, wrapper);
        e.call(0x00d8_4dc0, &args![malleable, vector, 0u32]);
        assert_eq!(read_vector(&e, inner + 0x20), [0x200, 0x201, 0x202, 0x203]);
    }

    fn identity_block(e: &mut Engine) -> u32 {
        let out = e.mem.alloc(0x40);
        for word in 0..16u32 {
            e.mem.set_u32(out + 4 * word, 0xdead_0000 + word);
        }
        out
    }

    fn assert_identity(e: &Engine, out: u32) {
        for column in 0..3u32 {
            for lane in 0..4u32 {
                let expected = if column == lane { 1.0f32.to_bits() } else { 0 };
                assert_eq!(e.mem.u32(out + 16 * column + 4 * lane), expected);
            }
        }
    }

    #[test]
    fn frame_reader_starts_from_the_identity_and_copies_a_translation() {
        let mut e = engine();
        let out = identity_block(&mut e);
        let object = data(&mut e, 8);
        put_vector(&mut e, object + 0x50, 0x300);
        e.call(0x00d8_4ee0, &args![object, 3u32, out]);
        assert_identity(&e, out);
        assert_eq!(read_vector(&e, out + 0x30), [0x300, 0x301, 0x302, 0x303]);
        // The other types leave the identity with a zero translation.
        let out = identity_block(&mut e);
        let nothing = data(&mut e, 4);
        e.call(0x00d8_4ee0, &args![nothing, 0u32, out]);
        assert_identity(&e, out);
        assert_eq!(read_vector(&e, out + 0x30), [0, 0, 0, 0]);
    }

    #[test]
    fn frame_reader_hands_a_rotation_to_the_copy_function_and_resets_after_each_unwrap() {
        let mut e = engine();
        e.register(GET_WRAPPED_DATA_TWIN, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(COPY_ROTATION, move |e, a| {
            // The identity is in place when the copy runs; the copy overwrites the translation.
            log.borrow_mut()
                .push((a.to_vec(), e.mem.u32(a[0]), e.mem.u32(a[0] + 0x30)));
            e.mem.set_u32(a[0] + 0x30, 0x77);
            Ret::default()
        });
        let out = identity_block(&mut e);
        let inner = data(&mut e, 2);
        let wrapper = data(&mut e, 13);
        e.mem.set_u32(wrapper + 0xc, inner);
        e.call(0x00d8_4ee0, &args![wrapper, 2u32, out]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, vec![out, inner + 0x80 + 0x20]);
        assert_eq!(seen[0].1, 1.0f32.to_bits());
        assert_eq!(seen[0].2, 0);
    }

    #[test]
    fn frame_writer_stores_the_translation_or_calls_the_rotation_copy() {
        let mut e = engine();
        let transform = e.mem.alloc(0x40);
        put_vector(&mut e, transform + 0x30, 0x400);
        let translation = data(&mut e, 15);
        e.call(0x00d8_5040, &args![translation, transform, 1u32]);
        assert_eq!(
            read_vector(&e, translation + 0x30),
            [0x400, 0x401, 0x402, 0x403]
        );
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(COPY_ROTATION, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let rotation = data(&mut e, 9);
        e.call(0x00d8_5040, &args![rotation, transform, 2u32]);
        assert_eq!(*seen.borrow(), [vec![rotation + 0x80 + 0x20, transform]]);
        let nothing = data(&mut e, 10);
        e.call(0x00d8_5040, &args![nothing, transform, 2u32]);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn frame_writer_unwraps_both_kinds_of_wrapper() {
        let mut e = engine();
        e.register(GET_WRAPPED_DATA, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0xc),
            ..Ret::default()
        });
        let transform = e.mem.alloc(0x40);
        put_vector(&mut e, transform + 0x30, 0x500);
        let inner = data(&mut e, 0);
        let wrapper = data(&mut e, 12);
        e.mem.set_u32(wrapper + 0x18, inner);
        let malleable = data(&mut e, 13);
        e.mem.set_u32(malleable + 0xc, wrapper);
        e.call(0x00d8_5040, &args![malleable, transform, 0u32]);
        assert_eq!(read_vector(&e, inner + 0x20), [0x500, 0x501, 0x502, 0x503]);
    }

    #[test]
    fn record_copy_moves_the_word_and_eight_vectors() {
        let mut e = engine();
        let source = e.mem.alloc(0x90);
        for word in 0..0x24u32 {
            e.mem.set_u32(source + 4 * word, 0x1000_0000 + word);
        }
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 2, 0xffff_ffff);
        assert_eq!(e.call(0x00d8_5300, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u16(this), 0x0000);
        // Only the low 16-bit word at +0 is copied: the bytes at +2.. keep their value.
        assert_eq!(e.mem.u16(this + 2), 0xffff);
        for word in 4..0x24u32 {
            assert_eq!(e.mem.u32(this + 4 * word), 0x1000_0000 + word);
        }
        // Nothing past the record.
        assert_eq!(e.mem.u32(this + 0x90), 0);
    }

    /// A world whose `+0x78` points at 0x50 recognisable bytes and whose `+0x80` points at a
    /// dispatcher with agent function `AGENT_FUNCTION` for selector 3, found through the table
    /// byte at `(1 + 0xd) * 32 + body_word`; returns `(world, body, position)`.
    fn query_setup(e: &mut Engine) -> (u32, u32, u32) {
        e.set_global(SPHERE_RADIUS, 0x3c23_d70au32);
        e.set_global(QUERY_DISTANCE, 0x7f7f_ffeeu32);
        e.set_global(QUERY_TOLERANCE, 0x7eff_ffeeu32);
        let world = e.mem.alloc(0x100);
        let source = e.mem.alloc(0x50);
        for word in 0..0x14u32 {
            e.mem.set_u32(source + 4 * word, 0x5000 + word);
        }
        let dispatcher = e.mem.alloc(0xb00);
        let body = e.mem.alloc(0x40);
        e.mem.set_u32(body + 0xc, 2);
        e.mem.set_u8(dispatcher + (0xe << 5) + 2, 3);
        e.mem.set_u32(dispatcher + 0x9a8 + 3 * 0x14, AGENT_FUNCTION);
        e.mem.set_u32(world + 0x78, source);
        e.mem.set_u32(world + 0x80, dispatcher);
        let position = e.mem.alloc(16);
        put_vector(e, position, 0x600);
        e.register(SPHERE_SHAPE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 1);
            e.mem.set_u32(a[0] + 0x10, a[1]);
            Ret::default()
        });
        e.register(COLLECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(COLLECTOR_DESTRUCT, |_, _| Ret::default());
        (world, body, position)
    }

    /// What the agent function saw: its arguments and the frame words around them.
    type Frame = Rc<RefCell<Vec<(Vec<u32>, Vec<u32>)>>>;

    fn agent(e: &mut Engine, found: bool) -> Frame {
        let seen: Frame = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(AGENT_FUNCTION, move |e, a| {
            let frame = a[0] - 0x20;
            let words = (0..0x68u32).map(|i| e.mem.u32(frame + 4 * i)).collect();
            log.borrow_mut().push((a.to_vec(), words));
            if found {
                e.mem.set_u32(frame + 0xf0, 1);
                e.mem.set_u32(frame + 0x11c, 0xabcd);
            }
            Ret::default()
        });
        seen
    }

    #[test]
    fn contact_query_builds_the_frame_and_answers_the_found_word() {
        let mut e = engine();
        let (world, body, position) = query_setup(&mut e);
        let seen = agent(&mut e, true);
        e.call_log = Some(vec![]);
        let answer = e.call(0x00d8_5160, &args![world, body, position]).u32();
        assert_eq!(answer, 0xabcd);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (arguments, words) = &seen[0];
        let frame = arguments[0] - 0x20;
        assert_eq!(arguments[1..], [frame + 0x10, frame + 0x140, frame + 0xc0]);
        let word = |offset: u32| words[(offset / 4) as usize];
        // The body record, the sphere pointer, the identity pointer.
        assert_eq!(word(0x10), body);
        assert_eq!(word(0x14), 0xffff_ffff);
        assert_eq!(word(0x18), DEFAULT_TRANSFORM);
        assert_eq!(word(0x1c), 0);
        assert_eq!(word(0x20), frame + 0xa0);
        assert_eq!(word(0x24), 0xffff_ffff);
        assert_eq!(word(0x28), frame + 0x60);
        assert_eq!(word(0x2c), 0);
        // The collector object and its table.
        assert_eq!(word(0x30), COLLECTOR_TABLE[0]);
        assert_eq!(word(0x38), COLLECTOR_TABLE[1]);
        assert_eq!(word(0x3c), COLLECTOR_TABLE[2]);
        assert_eq!(word(0x40), COLLECTOR_TABLE[3]);
        assert_eq!(word(0x44), COLLECTOR_TABLE[4]);
        assert_eq!(word(0x50), 1);
        // The identity rotation, the position, the sphere (radius passed to its constructor).
        assert_eq!(word(0x60), 1.0f32.to_bits());
        assert_eq!(word(0x74), 1.0f32.to_bits());
        assert_eq!(word(0x88), 1.0f32.to_bits());
        assert_eq!(word(0x64), 0);
        assert_eq!(
            [word(0x90), word(0x94), word(0x98), word(0x9c)],
            [0x600, 0x601, 0x602, 0x603]
        );
        assert_eq!(word(0xac), 1);
        assert_eq!(word(0xb0), 0x3c23_d70a);
        // The result record.
        assert_eq!(word(0xc0), QUERY_OUTPUT_VTABLE);
        assert_eq!(word(0xc4), 0x7f7f_ffee);
        assert_eq!(word(0xec), 0x7f7f_ffee);
        assert_eq!(word(0xf0), 0);
        // The copied bytes with the tolerance over the third word, and the collector pointer.
        let copied = |index: u32| e.mem.u32(frame + 0x140 + 4 * index);
        assert_eq!(copied(0), 0x5000);
        assert_eq!(copied(2), 0x7eff_ffee);
        assert_eq!(copied(0x13), 0x5013);
        assert_eq!(e.mem.u32(frame + 0x14c), frame + 0x30);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            [
                0x00d8_5160,
                SPHERE_SHAPE_CONSTRUCT,
                COLLECTOR_CONSTRUCT,
                AGENT_FUNCTION,
                COLLECTOR_DESTRUCT
            ]
        );
        assert_eq!(log[1].1, vec![frame + 0xa0, 0x3c23_d70a]);
        assert_eq!(log[2].1, vec![frame + 0x30]);
        assert_eq!(log[4].1, vec![frame + 0x30]);
    }

    #[test]
    fn contact_query_answers_zero_when_nothing_was_found() {
        let mut e = engine();
        let (world, body, position) = query_setup(&mut e);
        let seen = agent(&mut e, false);
        let answer = e.call(0x00d8_5160, &args![world, body, position]).u32();
        assert_eq!(answer, 0);
        assert_eq!(seen.borrow().len(), 1);
    }
}
