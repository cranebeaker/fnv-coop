//! `fallout shared/bsbipnode.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The flag test `00456630` (`thiscall (object, mask)`: whether the word at
/// +0x30 has a bit of `mask`).
const HAS_FLAG: u32 = 0x0045_6630;
/// The object's first time controller (`0043b230`, `thiscall`) and a
/// controller's next (`004a8a90`, `thiscall`).
const FIRST_CONTROLLER: u32 = 0x0043_b230;
const NEXT_CONTROLLER: u32 = 0x004a_8a90;
/// The word at an address (`00559450`, `thiscall`) and the list walk
/// `0057cbe0` (`thiscall (list, &position)`: the address of the element at
/// the position, the position moved on).
const FIRST_WORD: u32 = 0x0055_9450;
const LIST_GET_NEXT: u32 = 0x0057_cbe0;
/// `NiObject::IsKindOf`-style test (`0045bad0`, `cdecl (rtti, object)`,
/// answers a byte).
const IS_KIND_OF: u32 = 0x0045_bad0;
/// The `NiRTTI` records of `NiMultiTargetTransformController` (`011f36bc`)
/// and `BSMultiTargetTreadTransfController` (`011f36e4`) (their names come
/// from the static initializers at `00fb4ac0` and `00fb4b00`).
const MULTI_TARGET_TRANSFORM_RTTI: u32 = 0x011f_36bc;
const TREAD_TRANSFORM_RTTI: u32 = 0x011f_36e4;
/// The update data's constructor (`0043d410`, `thiscall (data, time, a,
/// b)`, 0xC bytes).
const UPDATE_DATA_CONSTRUCTOR: u32 = 0x0043_d410;
/// The TLS word the bip update keeps its state in (+0x2C0).
const TLS_UPDATE_STATE: u32 = 0x2c0;
/// `2.0` (`double`, `01011590`) and `FLT_MAX` (`float`, `01023900`).
const TWO: u32 = 0x0101_1590;
const LARGEST: u32 = 0x0102_3900;
/// The matrix column setter `004b4660` (`thiscall (matrix, column, three
/// floats)`).
const MATRIX_SET_COLUMN: u32 = 0x004b_4660;
/// The transform constructor (`00494260`, `thiscall`), its parts' validity
/// tests (translation `004942c0`; rotation `fn_004f02e0`; scale
/// `fn_004f0320`) and getters (translation `006815c0`, rotation `0048d150`,
/// scale `00495460` in ST0), and the target's setters (translation
/// `00440460`, scale `00440490`, both one word).
const TRANSFORM_CONSTRUCTOR: u32 = 0x0049_4260;
const TRANSLATE_VALID: u32 = 0x0049_42c0;
const TRANSLATE_OF: u32 = 0x0068_15c0;
const ROTATE_OF: u32 = 0x0048_d150;
const SCALE_OF: u32 = 0x0049_5460;
const SET_TRANSLATE: u32 = 0x0044_0460;
const SET_SCALE: u32 = 0x0044_0490;
/// The controller's own test `004f05a0` (`thiscall`, a byte) and the
/// rotation's +4 float `006b9130` (`thiscall`, in ST0).
const CONTROLLER_ACTIVE: u32 = 0x004f_05a0;
const ROTATION_SECOND: u32 = 0x006b_9130;
/// The multi-target controller's targets: the count (+0x3C, 16-bit), the
/// target array (+0x38) and the interpolators (+0x34, 0x30 bytes each).
const TARGET_COUNT: u32 = 0x3c;
const TARGETS: u32 = 0x38;
const INTERPOLATORS: u32 = 0x34;
const INTERPOLATOR_SIZE: u32 = 0x30;

// Translated from 004f0000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the object's flags (+0x30) have bit
/// 0x40000 (`00456630`).
pub fn fn_004f0000(e: &mut Engine, this: Ptr) -> bool {
    e.call(HAS_FLAG, &args![this, 0x0004_0000u32]).bool()
}

// Translated from 004f0140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the object's flags (+0x30) have bit 2
/// (`00456630`).
pub fn fn_004f0140(e: &mut Engine, this: Ptr) -> bool {
    e.call(HAS_FLAG, &args![this, 2u32]).bool()
}

// Translated from 004f0020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): stores `state` in the
/// thread's TLS block at +0x2C0.
pub fn fn_004f0020(e: &mut Engine, state: u32) {
    let at = e.tls() + TLS_UPDATE_STATE;
    e.mem.set_u32(at, state);
}

// Translated from 004f00e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the three words of `point` to `this +
/// 0x8C`.
pub fn fn_004f00e0(e: &mut Engine, this: Ptr, point: Ptr) {
    for i in 0..3u32 {
        let value = e.mem.u32(point.addr() + 4 * i);
        e.mem.set_u32(this.addr().wrapping_add(0x8c + 4 * i), value);
    }
}

// Translated from 004f0110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the nine words of `matrix` to `this +
/// 0x2C`.
pub fn fn_004f0110(e: &mut Engine, this: Ptr, matrix: Ptr) {
    for i in 0..9u32 {
        let value = e.mem.u32(matrix.addr() + 4 * i);
        e.mem.set_u32(this.addr().wrapping_add(0x2c + 4 * i), value);
    }
}

// Translated from 004f0160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the rotation matrix at `this + 0x34` from
/// `quaternion` (`NiQuaternion::ToRotation`).
pub fn fn_004f0160(e: &mut Engine, this: Ptr, quaternion: Ptr) {
    ni_quaternion_to_rotation(e, quaternion, this.byte_add(0x34));
}

// Translated from 004f0360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: 1 when the 16-bit word at +8 shares a bit
/// with the low 16 bits of `mask`, else 0.
pub fn fn_004f0360(e: &mut Engine, this: Ptr, mask: u32) -> u32 {
    let flags = e.mem.u16(this.addr().wrapping_add(8));
    u32::from(flags & mask as u16 != 0)
}

// Translated from 004f0320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the transform's scale test): whether the
/// float at +0x1C is not `-FLT_MAX` (a NaN counts as not equal).
pub fn fn_004f0320(e: &mut Engine, this: Ptr) -> bool {
    let invalid = -x87_load32(e.mem.u32(LARGEST));
    x87_load32(e.mem.u32(this.addr().wrapping_add(0x1c))) != invalid
}

// Translated from 004f02e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the transform's rotation test): whether the
/// rotation's second float (`006b9130` on `this + 0xC`) is not `-FLT_MAX`.
pub fn fn_004f02e0(e: &mut Engine, this: Ptr) -> bool {
    let value = e
        .call(ROTATION_SECOND, &args![this.addr().wrapping_add(0xc)])
        .f64();
    let invalid = -x87_load32(e.mem.u32(LARGEST));
    invalid != value
}

// Translated from 004f0180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternion::ToRotation` (Xbox PDB): the rotation matrix of the
/// quaternion `(w, x, y, z)` written column by column (`004b4660`): with
/// `x2 = 2x`, `y2 = 2y`, `z2 = 2z` and the products `wx = x2 w`, `wy = y2 w`,
/// `wz = z2 w`, `xx = x2 x`, `xy = y2 x`, `xz = z2 x`, `yy = y2 y`,
/// `yz = z2 y`, `zz = z2 z` (each rounded to a float, as the exe stores
/// them), column 0 is `(1 - (yy + zz), xy + wz, xz - wy)`, column 1
/// `(xy - wz, 1 - (xx + zz), yz + wx)`, column 2 `(xz + wy, yz - wx,
/// 1 - (xx + yy))`.
pub fn ni_quaternion_to_rotation(e: &mut Engine, this: Ptr, matrix: Ptr) {
    let q = this.addr();
    let two = f64::from_bits(e.global::<u64>(TWO));
    let load = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let operand = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
    let double = |e: &Engine, at: u32| x87_store32(x87_mul(load(e, at), two));
    let (x2, y2, z2) = (double(e, q + 4), double(e, q + 8), double(e, q + 0xc));
    let product =
        |e: &Engine, factor: u32, at: u32| x87_store32(x87_mul(x87_load32(factor), operand(e, at)));
    let wx = product(e, x2, q);
    let wy = product(e, y2, q);
    let wz = product(e, z2, q);
    let xx = product(e, x2, q + 4);
    let xy = product(e, y2, q + 4);
    let xz = product(e, z2, q + 4);
    let yy = product(e, y2, q + 8);
    let yz = product(e, z2, q + 8);
    let zz = product(e, z2, q + 0xc);
    let sub = |a: u32, b: u32| x87_store32(x87_sub(x87_load32(a), x87_m32(b)));
    let add = |a: u32, b: u32| x87_store32(x87_add(x87_load32(a), x87_m32(b)));
    let one_minus = |a: u32, b: u32| x87_store32(x87_sub(1.0, x87_add(x87_load32(a), x87_m32(b))));
    // Each column: its third value is computed (and pushed) first.
    let c2 = sub(xz, wy);
    let c1 = add(xy, wz);
    let c0 = one_minus(yy, zz);
    e.call(
        MATRIX_SET_COLUMN,
        &args![
            matrix,
            0u32,
            x87_quiet_bits(c0),
            x87_quiet_bits(c1),
            x87_quiet_bits(c2)
        ],
    );
    let c2 = add(yz, wx);
    let c1 = one_minus(xx, zz);
    let c0 = sub(xy, wz);
    e.call(
        MATRIX_SET_COLUMN,
        &args![
            matrix,
            1u32,
            x87_quiet_bits(c0),
            x87_quiet_bits(c1),
            x87_quiet_bits(c2)
        ],
    );
    let c2 = one_minus(xx, yy);
    let c1 = sub(yz, wx);
    let c0 = add(xz, wy);
    e.call(
        MATRIX_SET_COLUMN,
        &args![
            matrix,
            2u32,
            x87_quiet_bits(c0),
            x87_quiet_bits(c1),
            x87_quiet_bits(c2)
        ],
    );
}

// Translated from 004eff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject::UpdateObjectControllers` (Xbox PDB): with `properties` set
/// and the object's flag 0x40000 (`fn_004f0000`), `Update` (vtable +0x90)
/// with `data` of every property in the property list (+0x24) that has
/// controllers; then `Update` (vtable +0x94) with `data` of each of the
/// object's controllers.
pub fn ni_av_object_update_object_controllers(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    properties: u8,
) {
    if properties != 0 && fn_004f0000(e, this) {
        let list = this.addr().wrapping_add(0x24);
        e.with_stack(4, |e, position| {
            let head = e.call(FIRST_WORD, &args![list]).u32();
            e.mem.set_u32(position.addr(), head);
            while e.mem.u32(position.addr()) != 0 {
                let element = e.call(LIST_GET_NEXT, &args![list, position]).u32();
                let property = e.call(FIRST_WORD, &args![element]).u32();
                if property != 0 && e.call(FIRST_CONTROLLER, &args![property]).u32() != 0 {
                    e.vcall(property, 0x90, &args![data]);
                }
            }
        });
    }
    let mut controller = e.call(FIRST_CONTROLLER, &args![this]).u32();
    while controller != 0 {
        e.vcall(controller, 0x94, &args![data]);
        controller = e.call(NEXT_CONTROLLER, &args![controller]).u32();
    }
}

// Translated from 004efe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBipNode::UpdateBipOnly` (Xbox PDB): with the thread's update state 3
/// (`fn_004f0020`), walks the controllers until the first that is neither a
/// `NiMultiTargetTransformController` nor a
/// `BSMultiTargetTreadTransfController` (`0045bad0`) and updates (vtable
/// +0x94) only that one with update data for `time`; then with a `node`,
/// updates its controllers (`UpdateObjectControllers` with properties) with
/// update data for `time` (second word 1); the state goes back to 0.
pub fn bs_bip_node_update_bip_only(e: &mut Engine, this: Ptr, time: f32, node: Ptr) {
    fn_004f0020(e, 3);
    let time_bits = x87_quiet_bits(time.to_bits());
    let mut controller = e.call(FIRST_CONTROLLER, &args![this]).u32();
    while controller != 0 {
        let multi = e
            .call(IS_KIND_OF, &args![MULTI_TARGET_TRANSFORM_RTTI, controller])
            .u8();
        let tread = e
            .call(IS_KIND_OF, &args![TREAD_TRANSFORM_RTTI, controller])
            .u8();
        if (multi | tread) == 0 {
            e.with_stack(0xc, |e, data| {
                e.call(UPDATE_DATA_CONSTRUCTOR, &args![data, time_bits, 0u32, 0u32]);
                e.vcall(controller, 0x94, &args![data]);
            });
            break;
        }
        controller = e.call(NEXT_CONTROLLER, &args![controller]).u32();
    }
    if !node.is_null() {
        e.with_stack(0xc, |e, data| {
            e.call(UPDATE_DATA_CONSTRUCTOR, &args![data, time_bits, 1u32, 0u32]);
            ni_av_object_update_object_controllers(e, node, data, 1);
        });
    }
    fn_004f0020(e, 0);
}

// Translated from 004f0040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBipNode::UpdateAllButBip` (Xbox PDB): for each controller, by its
/// `GetRTTI` (vtable +8): a `NiMultiTargetTransformController` gets
/// `UpdateAllButTarget` (`004f0380`) with `time` and 0; a
/// `BSMultiTargetTreadTransfController` gets `Update` (vtable +0x94) with
/// update data for `time`.
pub fn bs_bip_node_update_all_but_bip(e: &mut Engine, this: Ptr, time: f32) {
    let time_bits = x87_quiet_bits(time.to_bits());
    let mut controller = e.call(FIRST_CONTROLLER, &args![this]).u32();
    while controller != 0 {
        if e.vcall(controller, 8, &args![]).u32() == MULTI_TARGET_TRANSFORM_RTTI {
            bs_trans_con_update_all_but_target(
                e,
                Ptr::new(controller),
                f32::from_bits(time_bits),
                0,
            );
        } else if e.vcall(controller, 8, &args![]).u32() == TREAD_TRANSFORM_RTTI {
            e.with_stack(0xc, |e, data| {
                e.call(UPDATE_DATA_CONSTRUCTOR, &args![data, time_bits, 0u32, 0u32]);
                e.vcall(controller, 0x94, &args![data]);
            });
        }
        controller = e.call(NEXT_CONTROLLER, &args![controller]).u32();
    }
}

/// Updates the target from interpolator `index`'s transform at `time`
/// (vtable +0x8C of the 0x30-byte interpolator): its translation, rotation
/// and scale when valid. False when the interpolator gave nothing.
fn update_target(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    time_bits: u32,
    target: u32,
    transform: Ptr,
) -> bool {
    let interpolators = e.mem.u32(this.addr().wrapping_add(INTERPOLATORS));
    let interpolator = interpolators.wrapping_add(index.wrapping_mul(INTERPOLATOR_SIZE));
    if !e
        .vcall(interpolator, 0x8c, &args![time_bits, target, transform])
        .bool()
    {
        return false;
    }
    if e.call(TRANSLATE_VALID, &args![transform]).bool() {
        let translate = e.call(TRANSLATE_OF, &args![transform]).u32();
        e.call(SET_TRANSLATE, &args![target, translate]);
    }
    if fn_004f02e0(e, transform) {
        let rotate = e.call(ROTATE_OF, &args![transform]).ptr();
        fn_004f0160(e, Ptr::new(target), rotate);
    }
    if fn_004f0320(e, transform) {
        let scale = e.call(SCALE_OF, &args![transform]).f64();
        e.call(SET_SCALE, &args![target, x87_store32(scale)]);
    }
    true
}

// Translated from 004f0380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTransCon::UpdateAllButTarget` (engine map name; the body updates a
/// multi-target transform controller's targets): when active (`004f05a0`)
/// with targets, each target but the last (non-null) whose flag 2
/// (`fn_004f0140`) is set when `inverse` is 0, or clear when it is not, is
/// updated from its interpolator's transform at `time`; then the last
/// target, when non-null and (with `inverse` 0) flag 2 is set, the loop
/// ending there when its interpolator gives nothing. One transform
/// (`00494260`) is reused for all.
pub fn bs_trans_con_update_all_but_target(e: &mut Engine, this: Ptr, time: f32, inverse: u8) {
    if !e.call(CONTROLLER_ACTIVE, &args![this]).bool()
        || e.mem.u16(this.addr().wrapping_add(TARGET_COUNT)) == 0
    {
        return;
    }
    let time_bits = x87_quiet_bits(time.to_bits());
    e.with_stack(0x20, |e, transform| {
        e.call(TRANSFORM_CONSTRUCTOR, &args![transform]);
        let last = i32::from(e.mem.u16(this.addr().wrapping_add(TARGET_COUNT))) - 1;
        let target_at = |e: &Engine, i: i32| {
            let targets = e.mem.u32(this.addr().wrapping_add(TARGETS));
            e.mem.u32(targets.wrapping_add((i as u32).wrapping_mul(4)))
        };
        let mut i = 0i32;
        while i < last {
            let target = target_at(e, i);
            let pick = target != 0
                && if inverse == 0 {
                    fn_004f0140(e, Ptr::new(target))
                } else {
                    !fn_004f0140(e, Ptr::new(target))
                };
            if pick {
                update_target(e, this, i as u32, time_bits, target, transform);
            }
            i += 1;
        }
        let target = target_at(e, i);
        if target == 0 || (inverse == 0 && !fn_004f0140(e, Ptr::new(target))) {
            return;
        }
        update_target(e, this, i as u32, time_bits, target, transform);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f0000, fn_004f0000(Ptr) -> bool),
        entry!(0x004f0140, fn_004f0140(Ptr) -> bool),
        entry!(0x004f0020, fn_004f0020(u32)),
        entry!(0x004f00e0, fn_004f00e0(Ptr, Ptr)),
        entry!(0x004f0110, fn_004f0110(Ptr, Ptr)),
        entry!(0x004f0160, fn_004f0160(Ptr, Ptr)),
        entry!(0x004f0360, fn_004f0360(Ptr, u32) -> u32),
        entry!(0x004f0320, fn_004f0320(Ptr) -> bool),
        entry!(0x004f02e0, fn_004f02e0(Ptr) -> bool),
        entry!(0x004f0180, ni_quaternion_to_rotation(Ptr, Ptr)),
        entry!(
            0x004eff50,
            ni_av_object_update_object_controllers(Ptr, Ptr, u8)
        ),
        entry!(0x004efe80, bs_bip_node_update_bip_only(Ptr, f32, Ptr)),
        entry!(0x004f0040, bs_bip_node_update_all_but_bip(Ptr, f32)),
        entry!(0x004f0380, bs_trans_con_update_all_but_target(Ptr, f32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const VTABLE: u32 = 0x7000_0000;
    const UPDATE_90: u32 = 0x7200_0090;
    const UPDATE_94: u32 = 0x7200_0094;
    const UPDATE_8C: u32 = 0x7200_008c;
    const GET_RTTI: u32 = 0x7200_0008;

    /// Objects with the test vtable: +0x40 holds the next controller, +0x44
    /// the RTTI the object answers, +0x48 its `Update` answer.
    fn engine() -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x98 / 4];
        slots[2] = GET_RTTI;
        slots[0x8c / 4] = UPDATE_8C;
        slots[0x90 / 4] = UPDATE_90;
        slots[0x94 / 4] = UPDATE_94;
        e.put_vtable(VTABLE, &slots);
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 0x44).into_ret());
        e.register(UPDATE_8C, |e, a| e.mem.u32(a[0] + 0x48).into_ret());
        e.register(UPDATE_90, |_, _| Ret::default());
        e.register(UPDATE_94, |_, _| Ret::default());
        e.register(FIRST_CONTROLLER, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(NEXT_CONTROLLER, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(IS_KIND_OF, |e, a| {
            (e.mem.u32(a[1] + 0x44) == a[0]).into_ret()
        });
        e.register(UPDATE_DATA_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret::default()
        });
        e
    }

    fn object(e: &mut Engine, rtti: u32, next: u32) -> u32 {
        let o = e.mem.alloc(0x50);
        e.mem.set_u32(o, VTABLE);
        e.mem.set_u32(o + 0x40, next);
        e.mem.set_u32(o + 0x44, rtti);
        o
    }

    #[test]
    fn flag_tests_tls_and_copies() {
        let mut e = engine();
        e.register(HAS_FLAG, |_, a| (a[1] == 2).into_ret());
        assert!(!e.call(0x004f0000, &args![0x100u32]).bool());
        assert!(e.call(0x004f0140, &args![0x100u32]).bool());
        e.call(0x004f0020, &args![7u32]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + 0x2c0), 7);
        let this = e.mem.alloc(0x98);
        let source = e.mem.alloc(0x24);
        for i in 0..9u32 {
            e.mem.set_u32(source + 4 * i, i + 1);
        }
        e.call(0x004f00e0, &args![this, source]);
        e.call(0x004f0110, &args![this, source]);
        assert_eq!(e.mem.u32(this + 0x8c + 8), 3);
        assert_eq!(e.mem.u32(this + 0x2c + 0x20), 9);
        e.mem.set_u16(this + 8, 0x0101);
        assert_eq!(e.call(0x004f0360, &args![this, 0xffff_0100u32]).u32(), 1);
        assert_eq!(e.call(0x004f0360, &args![this, 0x0010u32]).u32(), 0);
    }

    #[test]
    fn validity_tests() {
        let mut e = engine();
        e.map(0x0102_3000, 0x1000);
        e.mem.set_f32(LARGEST, f32::MAX);
        let t = e.mem.alloc(0x20);
        e.mem.set_f32(t + 0x1c, f32::MIN);
        assert!(!e.call(0x004f0320, &args![t]).bool());
        e.mem.set_f32(t + 0x1c, 1.0);
        assert!(e.call(0x004f0320, &args![t]).bool());
        e.mem.set_u32(t + 0x1c, 0x7fc0_0000);
        assert!(e.call(0x004f0320, &args![t]).bool());
        e.register(ROTATION_SECOND, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 4)),
            ..Ret::default()
        });
        e.mem.set_f32(t + 0x10, f32::MIN);
        assert!(!e.call(0x004f02e0, &args![t]).bool());
        e.mem.set_f32(t + 0x10, 0.5);
        assert!(e.call(0x004f02e0, &args![t]).bool());
    }

    fn rotation(e: &mut Engine, q: [f32; 4]) -> Vec<f32> {
        let quaternion = e.mem.alloc(0x10);
        for (i, v) in q.iter().enumerate() {
            e.mem.set_f32(quaternion + 4 * i as u32, *v);
        }
        let matrix = e.mem.alloc(0x24);
        e.call(0x004f0180, &args![quaternion, matrix]);
        (0..9).map(|i| e.mem.f32(matrix + 4 * i)).collect()
    }

    #[test]
    fn quaternion_to_rotation() {
        let mut e = engine();
        e.map(0x0101_1000, 0x1000);
        e.set_global(TWO, 2.0f64);
        e.register(MATRIX_SET_COLUMN, |e, a| {
            for k in 0..3 {
                e.mem.set_u32(a[0] + 0xc * k + 4 * a[1], a[2 + k as usize]);
            }
            Ret::default()
        });
        assert_eq!(
            rotation(&mut e, [1.0, 0.0, 0.0, 0.0]),
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
        // Half a turn about x.
        assert_eq!(
            rotation(&mut e, [0.0, 1.0, 0.0, 0.0]),
            [1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0]
        );
        // A quarter turn about z: column 0 is (0, 1, 0).
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let m = rotation(&mut e, [s, 0.0, 0.0, s]);
        assert!((m[0]).abs() < 1e-6 && (m[3] - 1.0).abs() < 1e-6, "{m:?}");
    }

    #[test]
    fn update_object_controllers_updates_properties_then_controllers() {
        let mut e = engine();
        e.register(HAS_FLAG, |_, a| (a[1] == 0x0004_0000).into_ret());
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        // The list's position words point at (element, next) nodes.
        e.register(LIST_GET_NEXT, |e, a| {
            let node = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], e.mem.u32(node + 4));
            node.into_ret()
        });
        let controlled = object(&mut e, 0, 0);
        e.mem.set_u32(controlled + 0x40, 0x9999);
        let uncontrolled = object(&mut e, 0, 0);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, uncontrolled);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, controlled);
        e.mem.set_u32(first + 4, second);
        let c2 = object(&mut e, 0, 0);
        let c1 = object(&mut e, 0, c2);
        let this = object(&mut e, 0, c1);
        e.mem.set_u32(this + 0x24, first);
        e.call_log = Some(vec![]);
        e.call(0x004eff50, &args![this, 0x5000u32, 1u32]);
        assert_eq!(calls(&e, UPDATE_90), vec![vec![controlled, 0x5000]]);
        assert_eq!(
            calls(&e, UPDATE_94),
            vec![vec![c1, 0x5000], vec![c2, 0x5000]]
        );
        // Without the properties argument only the controllers.
        e.call_log = Some(vec![]);
        e.call(0x004eff50, &args![this, 0x5000u32, 0u32]);
        assert!(calls(&e, UPDATE_90).is_empty());
        assert_eq!(calls(&e, UPDATE_94).len(), 2);
    }

    #[test]
    fn update_bip_only_updates_the_first_plain_controller_and_the_node() {
        let mut e = engine();
        e.register(HAS_FLAG, |_, _| false.into_ret());
        let plain2 = object(&mut e, 0x77, 0);
        let plain = object(&mut e, 0x66, plain2);
        let multi = object(&mut e, MULTI_TARGET_TRANSFORM_RTTI, plain);
        let this = object(&mut e, 0, multi);
        let node_controller = object(&mut e, 0, 0);
        let node = object(&mut e, 0, node_controller);
        e.call_log = Some(vec![]);
        e.call(0x004efe80, &args![this, 2.5f32, node]);
        let updates = calls(&e, UPDATE_94);
        assert_eq!(updates.len(), 2);
        assert_eq!(updates[0][0], plain);
        assert_eq!(updates[1][0], node_controller);
        let datas = calls(&e, UPDATE_DATA_CONSTRUCTOR);
        assert_eq!(&datas[0][1..], [2.5f32.to_bits(), 0, 0]);
        assert_eq!(&datas[1][1..], [2.5f32.to_bits(), 1, 0]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + 0x2c0), 0);
    }

    #[test]
    fn update_all_but_bip_dispatches_by_class() {
        let mut e = engine();
        e.register(CONTROLLER_ACTIVE, |_, _| false.into_ret());
        let other = object(&mut e, 0x55, 0);
        let tread = object(&mut e, TREAD_TRANSFORM_RTTI, other);
        let multi = object(&mut e, MULTI_TARGET_TRANSFORM_RTTI, tread);
        let this = object(&mut e, 0, multi);
        e.call_log = Some(vec![]);
        e.call(0x004f0040, &args![this, 1.5f32]);
        assert_eq!(calls(&e, CONTROLLER_ACTIVE), vec![vec![multi]]);
        let updates = calls(&e, UPDATE_94);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0][0], tread);
    }

    #[test]
    fn update_all_but_target_picks_targets_by_flag() {
        for inverse in [0u8, 1] {
            let mut e = engine();
            e.register(CONTROLLER_ACTIVE, |_, _| true.into_ret());
            e.register(TRANSFORM_CONSTRUCTOR, |_, _| Ret::default());
            // Targets with flag 2 at +0x30 bit 1.
            e.register(HAS_FLAG, |e, a| {
                (e.mem.u32(a[0] + 0x30) & a[1] != 0).into_ret()
            });
            e.register(TRANSLATE_VALID, |_, _| true.into_ret());
            e.register(TRANSLATE_OF, |_, a| a[0].into_ret());
            e.register(SET_TRANSLATE, |_, _| Ret::default());
            e.register(UPDATE_8C, |_, _| true.into_ret());
            e.register(SCALE_OF, |_, _| Ret {
                st0: 1.0,
                ..Ret::default()
            });
            e.register(SET_SCALE, |_, _| Ret::default());
            e.map(0x0102_3000, 0x1000);
            e.mem.set_f32(LARGEST, f32::MAX);
            e.register(ROTATION_SECOND, |_, _| Ret {
                st0: f64::from(f32::MIN),
                ..Ret::default()
            });
            let flagged = e.mem.alloc(0x40);
            e.mem.set_u32(flagged + 0x30, 2);
            let plain = e.mem.alloc(0x40);
            let last_plain = e.mem.alloc(0x40);
            let targets = e.mem.alloc(0xc);
            for (i, t) in [flagged, plain, last_plain].iter().enumerate() {
                e.mem.set_u32(targets + 4 * i as u32, *t);
            }
            let interpolators = e.mem.alloc(3 * INTERPOLATOR_SIZE);
            for i in 0..3 {
                let at = interpolators + i * INTERPOLATOR_SIZE;
                e.mem.set_u32(at, VTABLE);
            }
            let this = e.mem.alloc(0x40);
            e.mem.set_u16(this + TARGET_COUNT, 3);
            e.mem.set_u32(this + TARGETS, targets);
            e.mem.set_u32(this + INTERPOLATORS, interpolators);
            e.call_log = Some(vec![]);
            e.call(0x004f0380, &args![this, 1.0f32, u32::from(inverse)]);
            let set: Vec<u32> = calls(&e, SET_TRANSLATE).iter().map(|c| c[0]).collect();
            if inverse == 0 {
                assert_eq!(set, [flagged]);
            } else {
                assert_eq!(set, [plain, last_plain]);
            }
        }
    }
}
