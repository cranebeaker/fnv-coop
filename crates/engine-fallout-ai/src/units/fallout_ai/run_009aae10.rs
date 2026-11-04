//! `fallout/ai/run_009aae10` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two world-state helpers (the array whose values `005bd5c0` reads and `009aad00` sets),
//! the two `ContinuousBeamProjectile` constructors (vtables `0108ea64` and `0108ea58`, RTTI
//! `.?AVContinuousBeamProjectile@@`) and one function of that class's vtable. None is named
//! in the engine map.

#[allow(unused_imports)]
use crate::prelude::*;

/// The table of 8-byte state entries (`0x011a55b0`); the byte at `+4` of entry `kind`
/// (`0x011a55b4`) says the state's value is a set of bits.
const STATE_IS_BITS: u32 = 0x011a_55b4;
/// `005bd5c0` (unnamed): `thiscall` on the world-state array, one word (`RET 4`): the value
/// of state `kind`.
const STATE_VALUE: u32 = 0x005b_d5c0;
/// `009aad00` (unnamed, `combatworldstate.cpp`): `thiscall`, two words (`RET 8`): sets the
/// value of state `kind`.
const SET_STATE_VALUE: u32 = 0x009a_ad00;
/// `009aa140` (unnamed, `run_009aa120`): `cdecl`, the name of state `kind`.
const STATE_NAME: u32 = 0x009a_a140;
/// `009aa1b0` (unnamed, `run_009aa120`): `cdecl`, the name of a value of state `kind`.
const VALUE_NAME: u32 = 0x009a_a1b0;
/// `00406d00` (unnamed): `cdecl` formatter (buffer, size, format, ...).
const FORMAT_INTO: u32 = 0x0040_6d00;
/// `0044a670` (unnamed, `tesobject.cpp` per the engine map): `cdecl`, one word; the
/// string's length.
const STRING_LENGTH: u32 = 0x0044_a670;
/// `00406d50` (unnamed): `cdecl` (buffer, room, text); appends the text.
const APPEND_STRING: u32 = 0x0040_6d50;
/// `"%s: %s"` (`0x01012600`), `", %s"` (`0x0108ea18`) and `"%s: None"` (`0x0108ea0c`).
const NAME_VALUE_FORMAT: u32 = 0x0101_2600;
const NEXT_VALUE_FORMAT: u32 = 0x0108_ea18;
const NO_VALUE_FORMAT: u32 = 0x0108_ea0c;

/// `Projectile::Projectile` (`009bbc70`, Xbox PDB): `thiscall`, one word (`RET 4`).
const PROJECTILE_CONSTRUCT: u32 = 0x009b_bc70;
/// `009bbef0` (unnamed, `projectile.cpp`): the other `Projectile` constructor
/// (`thiscall`, eight words, `RET 0x20`).
const PROJECTILE_CONSTRUCT_WITH: u32 = 0x009b_bef0;
/// The `ContinuousBeamProjectile` vtables (RTTI `.?AVContinuousBeamProjectile@@`): at `+0`
/// and at `+0x18`.
const VTABLE: u32 = 0x0108_ea64;
const SECOND_VTABLE: u32 = 0x0108_ea58;
/// `00633c90` (unnamed): `thiscall`, one word (`RET 4`); constructs the member at `+0x150`.
const MEMBER_CONSTRUCT: u32 = 0x0063_3c90;
/// `004f15a0` (unnamed): `thiscall`, one word (`RET 4`), given 0x69.
const SET_KIND: u32 = 0x004f_15a0;
/// `0066b0d0` (unnamed): `thiscall`, one word (`RET 4`), given 0, on the member at
/// `+0x150`.
const MEMBER_SET: u32 = 0x0066_b0d0;

/// `00979260` (unnamed): `thiscall`, a float in `ST0`.
const BEAM_LENGTH: u32 = 0x0097_9260;
/// `1.0` (`double`, `0x01012070`).
const ONE: u32 = 0x0101_2070;
/// `009abd00` (unnamed, `run_009ab9c0`): `thiscall`, three words (`RET 0xc`).
const UPDATE_BEAM: u32 = 0x009a_bd00;
/// `00874480` (unnamed, `main.cpp` per the engine map): `thiscall`, a word.
const SOURCE_OBJECT: u32 = 0x0087_4480;
/// `0043d410` (unnamed, `modelloader.cpp`): `thiscall`, three words (`RET 0xc`); constructs
/// a point.
const POINT_CONSTRUCT: u32 = 0x0043_d410;
/// `00461130` (unnamed): `thiscall`, returns `this + 0x68` (the node's world rotation).
const WORLD_ROTATE: u32 = 0x0046_1130;
/// `NiMatrix3::Inverse` (`004b45b0`, Xbox PDB): `thiscall`, one word (`RET 4`): the out
/// matrix; returns it.
const MATRIX_INVERSE: u32 = 0x004b_45b0;
/// `NiMatrix3::operator*` (`0043f8d0`, engine map `NiMatrix3::operatorP`): `thiscall`, two
/// words (`RET 8`): the out matrix and the other; returns the out matrix.
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;

// Translated from 009aae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): sets state `kind` to `value`
/// (`009aad00`); for a state whose value is a set of bits (`0x011a55b4`), `value` is a bit
/// number added to the current set (`1 << value`, the count masked as `SHL` does).
pub fn fn_009aae10(e: &mut Engine, this: Ptr, kind: u32, value: u32) {
    if e.mem.u8(kind.wrapping_mul(8).wrapping_add(STATE_IS_BITS)) != 0 {
        let current = e.call(STATE_VALUE, &args![this, kind]).u32();
        let bits = 1u32.wrapping_shl(value) | current;
        e.call(SET_STATE_VALUE, &args![this, kind, bits]);
    } else {
        e.call(SET_STATE_VALUE, &args![this, kind, value]);
    }
}

// Translated from 009aae70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009aae70 - the /GS cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): writes state `kind` and its value
/// (`005bd5c0`) into `buffer` (`size` bytes). A plain state is `"%s: %s"` (`009aa140`'s and
/// `009aa1b0`'s names); a set of bits lists the name of each set bit (the first with
/// `"%s: %s"`, then each as `", %s"` formatted into a 0x104-byte stack buffer and appended,
/// `00406d50` with the room left after `0044a670`), or `"%s: None"` when none is set.
pub fn fn_009aae70(e: &mut Engine, this: Ptr, buffer: u32, size: u32, kind: u32) {
    let current = e.call(STATE_VALUE, &args![this, kind]).u32();
    if e.mem.u8(kind.wrapping_mul(8).wrapping_add(STATE_IS_BITS)) != 0 {
        let mut written = false;
        let mut bit = 0u32;
        while bit < 0x20 {
            if 1u32.wrapping_shl(bit) & current != 0 {
                if !written {
                    let value_name = e.call(VALUE_NAME, &args![kind, bit]).u32();
                    let state_name = e.call(STATE_NAME, &args![kind]).u32();
                    e.call(
                        FORMAT_INTO,
                        &args![buffer, size, NAME_VALUE_FORMAT, state_name, value_name],
                    );
                } else {
                    let value_name = e.call(VALUE_NAME, &args![kind, bit]).u32();
                    e.with_stack(0x104, |e, text| {
                        e.call(
                            FORMAT_INTO,
                            &args![text, 0x104u32, NEXT_VALUE_FORMAT, value_name],
                        );
                        let length = e.call(STRING_LENGTH, &args![buffer]).u32();
                        e.call(
                            APPEND_STRING,
                            &args![buffer, size.wrapping_sub(length), text],
                        );
                    });
                }
                written = true;
            }
            bit = bit.wrapping_add(1);
        }
        if !written {
            let state_name = e.call(STATE_NAME, &args![kind]).u32();
            e.call(
                FORMAT_INTO,
                &args![buffer, size, NO_VALUE_FORMAT, state_name],
            );
        }
    } else {
        let value_name = e.call(VALUE_NAME, &args![kind, current]).u32();
        let state_name = e.call(STATE_NAME, &args![kind]).u32();
        e.call(
            FORMAT_INTO,
            &args![buffer, size, NAME_VALUE_FORMAT, state_name, value_name],
        );
    }
}

/// The part both constructors share after the base constructor: the two vtables, the
/// member at `+0x150` (`00633c90` with 0), `004f15a0` with 0x69 and `0066b0d0` with 0 on
/// the member.
fn finish_construct(e: &mut Engine, this: Ptr) {
    let p = this.addr();
    e.mem.set_u32(p, VTABLE);
    e.mem.set_u32(p + 0x18, SECOND_VTABLE);
    e.call(MEMBER_CONSTRUCT, &args![p + 0x150, 0u32]);
    e.call(SET_KIND, &args![this, 0x69u32]);
    e.call(MEMBER_SET, &args![p + 0x150, 0u32]);
}

// Translated from 009aaff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009aaff0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `ContinuousBeamProjectile`'s constructor (unnamed in the engine map; `thiscall`,
/// `RET 4`): `Projectile::Projectile` (`009bbc70`) with the byte `flag`, then its vtables
/// and members (`finish_construct`). Returns `this`.
pub fn fn_009aaff0(e: &mut Engine, this: Ptr, flag: u8) -> Ptr {
    e.call(PROJECTILE_CONSTRUCT, &args![this, u32::from(flag)]);
    finish_construct(e, this);
    this
}

// Translated from 009ab090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ab090 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `ContinuousBeamProjectile`'s other constructor (unnamed in the engine map; `thiscall`,
/// `RET 0x20`): the `Projectile` constructor `009bbef0` with the three words, the point
/// (three words, by value) and the two floats (copied through `FLD`/`FSTP`), then its
/// vtables and members (`finish_construct`). Returns `this`.
pub fn fn_009ab090(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    point_x: u32,
    point_y: u32,
    point_z: u32,
    float_a: f32,
    float_b: f32,
) -> Ptr {
    e.call(
        PROJECTILE_CONSTRUCT_WITH,
        &args![
            this,
            first,
            second,
            third,
            point_x,
            point_y,
            point_z,
            x87_quiet_bits(float_a.to_bits()),
            x87_quiet_bits(float_b.to_bits())
        ],
    );
    finish_construct(e, this);
    this
}

// Translated from 009ab160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only; a `ContinuousBeamProjectile` function): with a
/// node (slot `+0x1d0`), updates the beam (`009abd00` with the node, the length `00979260`
/// raised to 1.0 when below it, and 0.0); then with a source object (`00874480`, its slot
/// `+0x20c`) passes a zero point (`0043d410`) to the node's slot `+0xb8` and stores at
/// `+0x94` the node's world rotation (`00461130` of slot `+0x1d0` again) times the inverse
/// of the source's (`004b45b0`, `0043f8d0`; nine words copied).
pub fn fn_009ab160(e: &mut Engine, this: Ptr) {
    let p = this.addr();
    let node = e.vcall(p, 0x1d0, &[]).u32();
    if node == 0 {
        return;
    }
    let length = e.call(BEAM_LENGTH, &args![this]).f64();
    let mut length = x87_store32(length);
    let one = f64::from_bits(e.mem.u64(ONE));
    if x87_load32(length) < one {
        length = 1.0f32.to_bits();
    }
    e.call(
        UPDATE_BEAM,
        &args![this, node, x87_quiet_bits(length), 0.0f32.to_bits()],
    );
    let source = if e.call(SOURCE_OBJECT, &args![this]).u32() != 0 {
        let object = e.call(SOURCE_OBJECT, &args![this]).u32();
        e.vcall(object, 0x20c, &[]).u32()
    } else {
        0
    };
    if source == 0 {
        return;
    }
    e.with_stack(0x60, |e, frame| {
        let point = frame.addr();
        let inverse_out = frame.addr() + 0xc;
        let product_out = frame.addr() + 0x30;
        e.call(POINT_CONSTRUCT, &args![point, 0.0f32.to_bits(), 0u32, 0u32]);
        e.vcall(node, 0xb8, &args![point]);
        let source_rotate = e.call(WORLD_ROTATE, &args![source]).u32();
        let inverse = e
            .call(MATRIX_INVERSE, &args![source_rotate, inverse_out])
            .u32();
        let own = e.vcall(p, 0x1d0, &[]).u32();
        let own_rotate = e.call(WORLD_ROTATE, &args![own]).u32();
        let product = e
            .call(MATRIX_MULTIPLY, &args![own_rotate, product_out, inverse])
            .u32();
        let words = e.mem.bytes(product, 0x24);
        e.mem.write(p + 0x94, &words);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_ae10, fn_009aae10(Ptr, u32, u32)),
        entry!(0x009a_ae70, fn_009aae70(Ptr, u32, u32, u32)),
        entry!(0x009a_aff0, fn_009aaff0(Ptr, u8) -> Ptr),
        entry!(
            0x009a_b090,
            fn_009ab090(Ptr, u32, u32, u32, u32, u32, u32, f32, f32) -> Ptr
        ),
        entry!(0x009a_b160, fn_009ab160(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn bits_kind(e: &mut Engine, kind: u32) {
        e.map(STATE_IS_BITS & !0xfff, 0x1000);
        e.mem.set_u8(STATE_IS_BITS + 8 * kind, 1);
    }

    #[test]
    fn fn_009aae10_sets_a_value_or_adds_a_bit() {
        let mut e = Engine::new();
        bits_kind(&mut e, 5);
        e.register(STATE_VALUE, |_, _| ret(0b1001));
        e.register(SET_STATE_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009a_ae10, &args![0x4000u32, 5u32, 2u32]);
        e.call(0x009a_ae10, &args![0x4000u32, 4u32, 2u32]);
        // The shift count is masked to five bits.
        e.call(0x009a_ae10, &args![0x4000u32, 5u32, 33u32]);
        assert_eq!(
            calls(&e, SET_STATE_VALUE),
            vec![
                vec![0x4000, 5, 0b1101],
                vec![0x4000, 4, 2],
                vec![0x4000, 5, 0b1011]
            ]
        );
    }

    fn text_doubles(e: &mut Engine) {
        e.register(STATE_NAME, |_, a| ret(0x5000 + a[0]));
        e.register(VALUE_NAME, |_, a| ret(0x6000 + a[1]));
        e.register(FORMAT_INTO, |_, _| Ret::default());
        e.register(STRING_LENGTH, |_, _| ret(10));
        e.register(APPEND_STRING, |_, _| Ret::default());
    }

    #[test]
    fn fn_009aae70_writes_a_plain_state() {
        let mut e = Engine::new();
        bits_kind(&mut e, 5);
        text_doubles(&mut e);
        e.register(STATE_VALUE, |_, _| ret(3));
        e.call_log = Some(vec![]);
        e.call(0x009a_ae70, &args![0x4000u32, 0x7000u32, 0x40u32, 4u32]);
        assert_eq!(
            calls(&e, FORMAT_INTO),
            vec![vec![0x7000, 0x40, NAME_VALUE_FORMAT, 0x5004, 0x6003]]
        );
    }

    #[test]
    fn fn_009aae70_lists_the_set_bits() {
        let mut e = Engine::new();
        bits_kind(&mut e, 5);
        text_doubles(&mut e);
        e.register(STATE_VALUE, |_, _| ret(0x8000_0005));
        e.call_log = Some(vec![]);
        e.call(0x009a_ae70, &args![0x4000u32, 0x7000u32, 0x40u32, 5u32]);
        let formats = calls(&e, FORMAT_INTO);
        assert_eq!(formats.len(), 3);
        assert_eq!(
            formats[0],
            vec![0x7000, 0x40, NAME_VALUE_FORMAT, 0x5005, 0x6000]
        );
        assert_eq!(&formats[1][1..], &[0x104, NEXT_VALUE_FORMAT, 0x6002]);
        assert_eq!(&formats[2][1..], &[0x104, NEXT_VALUE_FORMAT, 0x601f]);
        let appends = calls(&e, APPEND_STRING);
        assert_eq!(appends.len(), 2);
        assert_eq!(appends[0][..2], [0x7000, 0x40 - 10]);
        assert_eq!(appends[0][2], formats[1][0]);
        // No bit set: "%s: None".
        e.register(STATE_VALUE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x009a_ae70, &args![0x4000u32, 0x7000u32, 0x40u32, 5u32]);
        assert_eq!(
            calls(&e, FORMAT_INTO),
            vec![vec![0x7000, 0x40, NO_VALUE_FORMAT, 0x5005]]
        );
    }

    fn construct_doubles(e: &mut Engine) {
        for addr in [
            PROJECTILE_CONSTRUCT,
            PROJECTILE_CONSTRUCT_WITH,
            MEMBER_CONSTRUCT,
            SET_KIND,
            MEMBER_SET,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_009aaff0_constructs_the_projectile() {
        let mut e = Engine::new();
        construct_doubles(&mut e);
        let this = e.mem.alloc(0x160);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a_aff0, &args![this, 0x101u32]).u32(), this);
        assert_eq!(calls(&e, PROJECTILE_CONSTRUCT), vec![vec![this, 1]]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), SECOND_VTABLE);
        assert_eq!(calls(&e, MEMBER_CONSTRUCT), vec![vec![this + 0x150, 0]]);
        assert_eq!(calls(&e, SET_KIND), vec![vec![this, 0x69]]);
        assert_eq!(calls(&e, MEMBER_SET), vec![vec![this + 0x150, 0]]);
    }

    #[test]
    fn fn_009ab090_constructs_with_the_point_and_floats() {
        let mut e = Engine::new();
        construct_doubles(&mut e);
        let this = e.mem.alloc(0x160);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x009a_b090,
            &args![
                this,
                1u32,
                2u32,
                3u32,
                4u32,
                5u32,
                6u32,
                0x7f80_0001u32,
                2.5f32
            ],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(
            calls(&e, PROJECTILE_CONSTRUCT_WITH),
            vec![vec![this, 1, 2, 3, 4, 5, 6, 0x7fc0_0001, 2.5f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(calls(&e, SET_KIND), vec![vec![this, 0x69]]);
    }

    #[test]
    fn fn_009ab160_updates_the_beam_and_rotation() {
        let mut e = Engine::new();
        e.map(ONE & !0xfff, 0x1000);
        e.mem.set_u64(ONE, 1.0f64.to_bits());
        e.put_vtable(0x7000, &vec![0x7100; 0x210 / 4]);
        e.mem.set_u32(0x7000 + 0x1d0, 0x71d0);
        e.mem.set_u32(0x7000 + 0x20c, 0x720c);
        e.mem.set_u32(0x7000 + 0xb8, 0x70b8);
        let this = e.mem.alloc(0xc0);
        let node = e.mem.alloc(0x100);
        let source = e.mem.alloc(0x100);
        e.mem.set_u32(this, 0x7000);
        e.mem.set_u32(node, 0x7000);
        e.mem.set_u32(source, 0x7000);
        e.mem.set_u32(this + 0xbc, node);
        e.mem.set_u32(this + 0xb8, source);
        e.mem.set_f32(this + 0xb4, 0.25);
        e.register(0x71d0, |e, a| ret(e.mem.u32(a[0] + 0xbc)));
        e.register(0x720c, |_, a| ret(a[0]));
        e.register(0x70b8, |_, _| Ret::default());
        e.register(BEAM_LENGTH, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0xb4)),
            ..Ret::default()
        });
        e.register(UPDATE_BEAM, |_, _| Ret::default());
        e.register(SOURCE_OBJECT, |e, a| ret(e.mem.u32(a[0] + 0xb8)));
        e.register(POINT_CONSTRUCT, |_, _| Ret::default());
        e.register(WORLD_ROTATE, |_, a| ret(a[0] + 0x68));
        e.register(MATRIX_INVERSE, |_, a| ret(a[1]));
        e.register(MATRIX_MULTIPLY, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[1] + 4 * i, 0x100 + i);
            }
            ret(a[1])
        });
        e.call_log = Some(vec![]);
        e.call(0x009a_b160, &args![this]);
        assert_eq!(
            calls(&e, UPDATE_BEAM),
            vec![vec![this, node, 1.0f32.to_bits(), 0]]
        );
        let multiply = calls(&e, MATRIX_MULTIPLY);
        assert_eq!(multiply[0][0], node + 0x68);
        let inverse = calls(&e, MATRIX_INVERSE);
        assert_eq!(inverse[0][0], source + 0x68);
        assert_eq!(multiply[0][2], inverse[0][1]);
        assert_eq!(
            (0..9)
                .map(|i| e.mem.u32(this + 0x94 + 4 * i))
                .collect::<Vec<_>>(),
            (0x100..0x109).collect::<Vec<u32>>()
        );
        // A length of at least 1.0 is passed as it is; without a source nothing more.
        e.mem.set_f32(this + 0xb4, 2.0);
        e.mem.set_u32(this + 0xb8, 0);
        e.call_log = Some(vec![]);
        e.call(0x009a_b160, &args![this]);
        assert_eq!(calls(&e, UPDATE_BEAM)[0][2], 2.0f32.to_bits());
        assert!(calls(&e, POINT_CONSTRUCT).is_empty());
    }
}
