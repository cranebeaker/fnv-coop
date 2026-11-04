//! `fallout/ai/run_009b30b0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The getter `fn_009b3100` reads through (`00560d30`): given the member at
/// `this + 0x10` and an index, it returns the address of a `float`.
const FLOAT_SLOT_GETTER: u32 = 0x0056_0d30;
/// `00622570`: called with the result object and a `bool` (one stack word).
const RESULT_FROM_BOOL: u32 = 0x0062_2570;
/// `00401030`: the memory release the scalar deleting destructors call
/// (cdecl, one word).
const RELEASE: u32 = 0x0040_1030;

// Translated from 009b30b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares the values `fn_009b3100` reads from `this` and from `other`:
/// sets up `result` through `00622570` with `other's > this's` (false when
/// either is a NaN: `FCOMP`, then `TEST AH,0x41`), and returns `result`.
/// Two stack words (`result`, `other`), `RET 8`.
pub fn fn_009b30b0(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    let mine = f64::from(fn_009b3100(e, this));
    let theirs = f64::from(fn_009b3100(e, other));
    let greater = theirs > mine;
    e.call(RESULT_FROM_BOOL, &args![result, greater as u32]);
    result
}

// Translated from 009b3100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at the address `00560d30` gives for the member at
/// `this + 0x10` and index 3, loaded into `ST0`.
pub fn fn_009b3100(e: &mut Engine, this: Ptr) -> f32 {
    let slot = e
        .call(FLOAT_SLOT_GETTER, &args![this.byte_add(0x10), 3u32])
        .u32();
    x87_quiet(e.mem.f32(slot))
}

// Translated from 009b3120 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual that returns the constant 2 (`ECX` only, no stack words).
pub fn fn_009b3120(_e: &mut Engine, _this: Ptr) -> u32 {
    2
}

// Translated from 009b3130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: runs the destructor `009b3f80`, then
/// releases the object (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009b3130(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x009b_3f80, &args![this]);
    if flags & 1 != 0 {
        e.call(RELEASE, &args![this]);
    }
    this
}

// Translated from 009b3490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: runs the destructor `009bb940`, then
/// releases the object (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_009b3490(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x009b_b940, &args![this]);
    if flags & 1 != 0 {
        e.call(RELEASE, &args![this]);
    }
    this
}

/// `Projectile::Projectile` (`009bbc70`, Xbox PDB): `thiscall`, one word (`RET 4`).
const PROJECTILE_CONSTRUCT: u32 = 0x009b_bc70;
/// `009bbef0` (unnamed, `projectile.cpp`): the other `Projectile` constructor
/// (`thiscall`, eight words, `RET 0x20`).
const PROJECTILE_CONSTRUCT_WITH: u32 = 0x009b_bef0;
/// The `FlameProjectile` vtables (RTTI `.?AVFlameProjectile@@`): at `+0` and at `+0x18`.
const FLAME_VTABLE: u32 = 0x0108_f2f4;
const FLAME_SECOND_VTABLE: u32 = 0x0108_f2e8;
/// `004f15a0` (unnamed): `thiscall`, one word (`RET 4`), given 0x40.
const SET_KIND: u32 = 0x004f_15a0;
/// `005de0a0` (unnamed, `tesscriptfunctions.cpp` per the engine map): `thiscall`, two
/// words (`RET 8`): a flag mask and whether to set it.
const SET_FLAG: u32 = 0x005d_e0a0;
/// `009bda10` (unnamed, `projectile.cpp`): `thiscall`, no stack words.
const PROJECTILE_INITIALIZE: u32 = 0x009b_da10;
/// `00874480` (unnamed, `main.cpp` per the engine map): `thiscall`; the source object.
const SOURCE_OBJECT: u32 = 0x0087_4480;
/// `0043d410` (unnamed, `modelloader.cpp`): `thiscall`, three words (`RET 0xc`);
/// constructs a point.
const POINT_CONSTRUCT: u32 = 0x0043_d410;
/// `00461130` (unnamed): `thiscall`, returns `this + 0x68` (the node's world rotation).
const WORLD_ROTATE: u32 = 0x0046_1130;
/// `NiMatrix3::Inverse` (`004b45b0`, Xbox PDB): `thiscall`, one word (`RET 4`).
const MATRIX_INVERSE: u32 = 0x004b_45b0;
/// `NiMatrix3::operator*` (`0043f8d0`, engine map `NiMatrix3::operatorP`): `thiscall`,
/// two words (`RET 8`).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `009b3da0` (unnamed, `run_009b36f0`): `cdecl`, one word; a float in `ST0`.
const NODE_EXTENT: u32 = 0x009b_3da0;
/// The `double` the extent is scaled by (`0x0106b9e8`, the float 0.9 widened).
const EXTENT_SCALE: u32 = 0x0106_b9e8;
/// `008256d0` (unnamed): `thiscall`; whether the list at `+0x88` is empty.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (unnamed): `thiscall`, returns `this` (the list's first item slot).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `0063f7b0` (unnamed): `thiscall`; removes the list's first node.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// `009c2ff0` (unnamed, `projectile.cpp`): `thiscall`, no stack words.
const PROJECTILE_CLEAR_IMPACTS: u32 = 0x009c_2ff0;

/// The part both `FlameProjectile` constructors share after the base constructor: the two
/// vtables, `004f15a0` with 0x40, and 0.0 in the floats at `+0x150` and `+0x154`.
fn flame_finish_construct(e: &mut Engine, this: Ptr) {
    let p = this.addr();
    e.mem.set_u32(p, FLAME_VTABLE);
    e.mem.set_u32(p + 0x18, FLAME_SECOND_VTABLE);
    e.call(SET_KIND, &args![this, 0x40u32]);
    e.mem.set_u32(p + 0x150, 0.0f32.to_bits());
    e.mem.set_u32(p + 0x154, 0.0f32.to_bits());
}

// Translated from 009b3160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b3160 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `FlameProjectile`'s constructor (unnamed in the engine map; `thiscall`, `RET 4`):
/// `Projectile::Projectile` (`009bbc70`) with the byte `flag`, then its vtables and fields
/// (`flame_finish_construct`). Returns `this`.
pub fn fn_009b3160(e: &mut Engine, this: Ptr, flag: u8) -> Ptr {
    e.call(PROJECTILE_CONSTRUCT, &args![this, u32::from(flag)]);
    flame_finish_construct(e, this);
    this
}

// Translated from 009b31f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b31f0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// `FlameProjectile`'s other constructor (unnamed in the engine map; `thiscall`,
/// `RET 0x20`): the `Projectile` constructor `009bbef0` with the three words, the point
/// (three words, by value) and the two floats (copied through `FLD`/`FSTP`), then its
/// vtables and fields (`flame_finish_construct`). Returns `this`.
pub fn fn_009b31f0(
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
    flame_finish_construct(e, this);
    this
}

// Translated from 009b32b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only; a `FlameProjectile` function): sets its flags
/// (`005de0a0`: 0x10, 1, 2 and 0x20 cleared, 8, 0x80 and 0x100 set, in that order), then
/// `009bda10`.
pub fn fn_009b32b0(e: &mut Engine, this: Ptr) {
    for (mask, set) in [
        (0x10u32, 0u32),
        (0x1, 0),
        (0x2, 0),
        (0x8, 1),
        (0x20, 0),
        (0x80, 1),
        (0x100, 1),
    ] {
        e.call(SET_FLAG, &args![this, mask, set]);
    }
    e.call(PROJECTILE_INITIALIZE, &args![this]);
}

// Translated from 009b3320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only; a `FlameProjectile` function): with a node (slot
/// `+0x1d0`), when the source object (`00874480`, no null test) gives one through its slot
/// `+0x20c`, passes a zero point (`0043d410`) to the node's slot `+0xb8` and stores at
/// `+0x94` the node's world rotation (`00461130` of slot `+0x1d0` again) times the inverse
/// of the source's (`004b45b0`, `0043f8d0`; nine words copied); then the float at `+0x154`
/// becomes the node's extent (`009b3da0`) times the `double` at `0x0106b9e8`.
pub fn fn_009b3320(e: &mut Engine, this: Ptr) {
    let p = this.addr();
    let node = e.vcall(p, 0x1d0, &[]).u32();
    if node == 0 {
        return;
    }
    let object = e.call(SOURCE_OBJECT, &args![this]).u32();
    let source = e.vcall(object, 0x20c, &[]).u32();
    if source != 0 {
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
    let extent = e.call(NODE_EXTENT, &args![node]).f64();
    let scale = f64::from_bits(e.mem.u64(EXTENT_SCALE));
    e.mem
        .set_u32(p + 0x154, x87_store32(x87_mul(extent, scale)));
}

// Translated from 009b3400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): clears the float at `+0x150`, empties the list
/// at `+0x88` (each item destroyed and freed through `fn_009b3490` with 1, null items
/// skipped; `0063f7b0` drops the node), then `009c2ff0` and clears the byte at `+0x90`.
pub fn fn_009b3400(e: &mut Engine, this: Ptr) {
    let p = this.addr();
    e.mem.set_u32(p + 0x150, 0.0f32.to_bits());
    while e.call(LIST_IS_EMPTY, &args![p + 0x88]).u8() == 0 {
        let slot = e.call(NODE_ITEM, &args![p + 0x88]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            fn_009b3490(e, Ptr::new(item), 1);
        }
        e.call(LIST_REMOVE_HEAD, &args![p + 0x88]);
    }
    e.call(PROJECTILE_CLEAR_IMPACTS, &args![this]);
    e.mem.set_u8(p + 0x90, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009b_30b0, fn_009b30b0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x009b_3100, fn_009b3100(Ptr) -> f32),
        entry!(0x009b_3120, fn_009b3120(Ptr) -> u32),
        entry!(0x009b_3130, fn_009b3130(Ptr, u32) -> Ptr),
        entry!(0x009b_3490, fn_009b3490(Ptr, u32) -> Ptr),
        entry!(0x009b_3160, fn_009b3160(Ptr, u8) -> Ptr),
        entry!(
            0x009b_31f0,
            fn_009b31f0(Ptr, u32, u32, u32, u32, u32, u32, f32, f32) -> Ptr
        ),
        entry!(0x009b_32b0, fn_009b32b0(Ptr)),
        entry!(0x009b_3320, fn_009b3320(Ptr)),
        entry!(0x009b_3400, fn_009b3400(Ptr)),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    /// The getter double: the float lives at `member + 0x40 + 4 * index`.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(FLOAT_SLOT_GETTER, |_, a| Ret {
            eax: a[0] + 0x40 + 4 * a[1],
            ..Ret::default()
        });
        e.register(RESULT_FROM_BOOL, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e
    }

    fn with_value(e: &mut Engine, value: f32) -> u32 {
        let object = e.mem.alloc(0x80);
        e.mem.set_f32(object + 0x10 + 0x40 + 12, value);
        object
    }

    #[test]
    fn fn_009b3100_reads_slot_3_of_the_member() {
        let mut e = engine();
        let object = with_value(&mut e, 4.5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_3100, &args![object]).f32(), 4.5);
        let log = e.call_log.take().unwrap();
        let getter: Vec<_> = log.iter().filter(|c| c.0 == FLOAT_SLOT_GETTER).collect();
        assert_eq!(getter[0].1, vec![object + 0x10, 3]);
    }

    #[test]
    fn fn_009b30b0_is_true_when_the_other_value_is_greater() {
        let mut e = engine();
        let result = e.mem.alloc(4);
        let low = with_value(&mut e, 1.0);
        let high = with_value(&mut e, 2.0);
        let nan = with_value(&mut e, f32::NAN);
        let back = e.call(0x009b_30b0, &args![low, result, high]).u32();
        assert_eq!(back, result);
        assert_eq!(e.mem.u32(result), 1);
        e.call(0x009b_30b0, &args![high, result, low]);
        assert_eq!(e.mem.u32(result), 0);
        e.call(0x009b_30b0, &args![low, result, low]);
        assert_eq!(e.mem.u32(result), 0);
        e.call(0x009b_30b0, &args![low, result, nan]);
        assert_eq!(e.mem.u32(result), 0);
    }

    #[test]
    fn fn_009b3120_returns_two() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009b_3120, &args![0x4000u32]).u32(), 2);
    }

    #[test]
    fn deleting_destructors_release_only_with_bit_0() {
        for (addr, dtor) in [(0x009b_3130, 0x009b_3f80), (0x009b_3490, 0x009b_b940)] {
            let mut e = Engine::new();
            e.register(dtor, |_, _| Ret::default());
            e.register(RELEASE, |_, _| Ret::default());
            for flags in [0u32, 1, 2, 3] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(addr, &args![0x5000u32, flags]).u32(), 0x5000);
                let log = e.call_log.take().unwrap();
                let dtors = log.iter().filter(|c| c.0 == dtor).count();
                let releases: Vec<_> = log.iter().filter(|c| c.0 == RELEASE).collect();
                assert_eq!(dtors, 1);
                assert_eq!(releases.len(), (flags & 1) as usize);
                if let Some(release) = releases.first() {
                    assert_eq!(release.1, vec![0x5000]);
                }
            }
        }
    }
}

#[cfg(test)]
mod flame_tests {
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

    fn construct_doubles(e: &mut Engine) {
        for addr in [PROJECTILE_CONSTRUCT, PROJECTILE_CONSTRUCT_WITH, SET_KIND] {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn fn_009b3160_constructs_the_flame_projectile() {
        let mut e = Engine::new();
        construct_doubles(&mut e);
        let this = e.mem.alloc(0x160);
        e.mem.write(this + 0x150, &[0xcd; 8]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_3160, &args![this, 0x201u32]).u32(), this);
        assert_eq!(calls(&e, PROJECTILE_CONSTRUCT), vec![vec![this, 1]]);
        assert_eq!(e.mem.u32(this), FLAME_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), FLAME_SECOND_VTABLE);
        assert_eq!(calls(&e, SET_KIND), vec![vec![this, 0x40]]);
        assert_eq!(e.mem.u64(this + 0x150), 0);
    }

    #[test]
    fn fn_009b31f0_constructs_with_the_point_and_floats() {
        let mut e = Engine::new();
        construct_doubles(&mut e);
        let this = e.mem.alloc(0x160);
        e.mem.write(this + 0x150, &[0xcd; 8]);
        e.call_log = Some(vec![]);
        let r = e.call(
            0x009b_31f0,
            &args![
                this,
                1u32,
                2u32,
                3u32,
                4u32,
                5u32,
                6u32,
                1.5f32,
                0xff80_0002u32
            ],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(
            calls(&e, PROJECTILE_CONSTRUCT_WITH),
            vec![vec![this, 1, 2, 3, 4, 5, 6, 1.5f32.to_bits(), 0xffc0_0002]]
        );
        assert_eq!(e.mem.u32(this), FLAME_VTABLE);
        assert_eq!(e.mem.u64(this + 0x150), 0);
    }

    #[test]
    fn fn_009b32b0_sets_the_flags_then_initializes() {
        let mut e = Engine::new();
        e.register(SET_FLAG, |_, _| Ret::default());
        e.register(PROJECTILE_INITIALIZE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009b_32b0, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<_> = log.iter().skip(1).cloned().collect();
        assert_eq!(
            order,
            vec![
                (SET_FLAG, vec![0x4000, 0x10, 0]),
                (SET_FLAG, vec![0x4000, 1, 0]),
                (SET_FLAG, vec![0x4000, 2, 0]),
                (SET_FLAG, vec![0x4000, 8, 1]),
                (SET_FLAG, vec![0x4000, 0x20, 0]),
                (SET_FLAG, vec![0x4000, 0x80, 1]),
                (SET_FLAG, vec![0x4000, 0x100, 1]),
                (PROJECTILE_INITIALIZE, vec![0x4000]),
            ]
        );
    }

    #[test]
    fn fn_009b3320_updates_rotation_and_extent() {
        let mut e = Engine::new();
        e.map(EXTENT_SCALE & !0xfff, 0x1000);
        e.mem.set_u64(EXTENT_SCALE, 0.5f64.to_bits());
        e.put_vtable(0x7000, &vec![0x7100; 0x210 / 4]);
        e.mem.set_u32(0x7000 + 0x1d0, 0x71d0);
        e.mem.set_u32(0x7000 + 0x20c, 0x720c);
        e.mem.set_u32(0x7000 + 0xb8, 0x70b8);
        let this = e.mem.alloc(0x160);
        let node = e.mem.alloc(0x100);
        let object = e.mem.alloc(0x100);
        for p in [this, node, object] {
            e.mem.set_u32(p, 0x7000);
        }
        e.mem.set_u32(this + 0x158, node);
        e.mem.set_u32(this + 0x15c, object);
        e.register(0x71d0, |e, a| ret(e.mem.u32(a[0] + 0x158)));
        e.register(0x720c, |e, a| ret(e.mem.u32(a[0] + 0x80)));
        e.register(0x70b8, |_, _| Ret::default());
        e.register(SOURCE_OBJECT, |e, a| ret(e.mem.u32(a[0] + 0x15c)));
        e.register(POINT_CONSTRUCT, |_, _| Ret::default());
        e.register(WORLD_ROTATE, |_, a| ret(a[0] + 0x68));
        e.register(MATRIX_INVERSE, |_, a| ret(a[1]));
        e.register(MATRIX_MULTIPLY, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[1] + 4 * i, 0x200 + i);
            }
            ret(a[1])
        });
        e.register(NODE_EXTENT, |_, _| Ret {
            st0: 6.0,
            ..Ret::default()
        });
        // The source object gives no source: only the extent is set.
        e.call_log = Some(vec![]);
        e.call(0x009b_3320, &args![this]);
        assert!(calls(&e, MATRIX_MULTIPLY).is_empty());
        assert_eq!(e.mem.f32(this + 0x154), 3.0);
        assert_eq!(calls(&e, NODE_EXTENT), vec![vec![node]]);
        // With a source, the rotation is stored too.
        let source = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0x80, source);
        e.call(0x009b_3320, &args![this]);
        assert_eq!(e.mem.u32(this + 0x94), 0x200);
        assert_eq!(e.mem.u32(this + 0x94 + 32), 0x208);
    }

    #[test]
    fn fn_009b3400_empties_the_impact_list() {
        let mut e = Engine::new();
        // The list double: a count at `+0x88`, the items in a table at `+0x8c`.
        e.register(LIST_IS_EMPTY, |e, a| ret((e.mem.u32(a[0]) == 0) as u32));
        e.register(NODE_ITEM, |e, a| {
            let count = e.mem.u32(a[0]);
            ret(e.mem.u32(a[0] + 4) + 4 * (count - 1))
        });
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let count = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], count - 1);
            Ret::default()
        });
        e.register(0x009b_b940, |_, _| Ret::default());
        e.register(RELEASE, |_, _| Ret::default());
        e.register(PROJECTILE_CLEAR_IMPACTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x160);
        let items = e.mem.alloc(8);
        e.mem.set_u32(items, 0x5000);
        e.mem.set_u32(items + 4, 0);
        e.mem.set_u32(this + 0x88, 2);
        e.mem.set_u32(this + 0x8c, items);
        e.mem.set_u8(this + 0x90, 1);
        e.mem.set_f32(this + 0x150, 2.0);
        e.call_log = Some(vec![]);
        e.call(0x009b_3400, &args![this]);
        assert_eq!(calls(&e, 0x009b_b940), vec![vec![0x5000]]);
        assert_eq!(calls(&e, RELEASE), vec![vec![0x5000]]);
        assert_eq!(calls(&e, LIST_REMOVE_HEAD).len(), 2);
        assert_eq!(calls(&e, PROJECTILE_CLEAR_IMPACTS), vec![vec![this]]);
        assert_eq!(e.mem.u8(this + 0x90), 0);
        assert_eq!(e.mem.u32(this + 0x150), 0);
    }
}
