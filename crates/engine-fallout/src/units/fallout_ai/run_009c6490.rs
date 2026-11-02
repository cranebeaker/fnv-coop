//! `fallout/ai/run_009c6490` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009c6a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3e0 (`SUB ECX,0x3e0`) and jumps to `008d1130`. `008d1130` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_009c6a20(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x008d_1130, &args![this.addr().wrapping_sub(0x3e0), flags])
        .u32()
}

/// The base constructor `0096a2d0` (`this` only, plain `RET`).
const BASE_CONSTRUCTOR_0096A2D0: u32 = 0x0096_a2d0;
/// `00633c90` and `0066b0d0` (each `this + 0x34` plus one stack word, `RET 4`).
const MEMBER_CONSTRUCTOR_00633C90: u32 = 0x0063_3c90;
const MEMBER_CONSTRUCTOR_0066B0D0: u32 = 0x0066_b0d0;
/// A global word the constructor clears (`.data`).
const CLEARED_GLOBAL_011F21B8: u32 = 0x011f_21b8;

// Translated from 009c6a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c6a30 - the compiler's exception frame (FS:[0] chain and the
// unwinding that would destroy the member at +0x34) is not translated.
/// Unnamed in the engine map: runs `0096a2d0` on `this`, calls `00633c90(this + 0x34, 0)`,
/// clears the words at `+0x08`, `+0x0c`, `+0x10`, the floats `0.0` at `+0x14` and `+0x18`, the
/// words at `+0x1c` to `+0x2c` and the global word `011f21b8`, calls
/// `0066b0d0(this + 0x34, 0)`, stores the byte 1 at `+0x38` and clears the words at `+0x3c`,
/// `+0x40` and `+0x44`. Returns `this`.
pub fn fn_009c6a30(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCTOR_0096A2D0, &args![this]);
    e.call(MEMBER_CONSTRUCTOR_00633C90, &args![base + 0x34, 0u32]);
    e.mem.set_u32(base + 0x8, 0);
    e.set_global(CLEARED_GLOBAL_011F21B8, 0u32);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_f32(base + 0x14, 0.0);
    e.mem.set_f32(base + 0x18, 0.0);
    for offset in [0x1c, 0x20, 0x24, 0x28, 0x2c] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(MEMBER_CONSTRUCTOR_0066B0D0, &args![base + 0x34, 0u32]);
    e.mem.set_u8(base + 0x38, 1);
    e.mem.set_u32(base + 0x3c, 0);
    e.mem.set_u32(base + 0x40, 0);
    e.mem.set_u32(base + 0x44, 0);
    this
}

/// `009c6ba0` (`thiscall`, no stack word; engine map `VATS::ClearData`).
const CLEAR_DATA: u32 = 0x009c_6ba0;
/// `0045cec0` (`thiscall`, no stack word): the member destructor run on +0x34.
const MEMBER_DESTRUCTOR_0045CEC0: u32 = 0x0045_cec0;
/// `0046ffb0` (`thiscall`, no stack word): the base destructor.
const BASE_DESTRUCTOR_0046FFB0: u32 = 0x0046_ffb0;

// Translated from 009c6b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c6b30 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f1b483, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): the destructor of the object
/// [`fn_009c6a30`] builds. Runs `009c6ba0` (engine map `VATS::ClearData`) on `this`, the
/// member destructor `0045cec0` on +0x34 and the base destructor `0046ffb0` on `this`.
pub fn fn_009c6b30(e: &mut Engine, this: Ptr) {
    e.call(CLEAR_DATA, &args![this]);
    e.call(MEMBER_DESTRUCTOR_0045CEC0, &args![this.addr() + 0x34]);
    e.call(BASE_DESTRUCTOR_0046FFB0, &args![this]);
}

/// `bhkUtilFunctions::GetNiAVObject` (`00c7fa90`) and
/// `TESObjectREFR::FindReferenceFor3D` (`0056f930`), Xbox PDB: `cdecl`, one word each.
const GET_NI_AV_OBJECT: u32 = 0x00c7_fa90;
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `006815c0` (unnamed): `thiscall`, returns `this` (also the no-op constructor of point and
/// matrix locals).
const SELF_006815C0: u32 = 0x0068_15c0;
/// `00460140` (unnamed): `thiscall`, returns `this + 0x10`.
const MEMBER_AT_10: u32 = 0x0046_0140;
/// `00458620` (unnamed, `tes.cpp`) and `004a3970` (unnamed, `bgsdecalmanager.cpp`):
/// `cdecl` (destination, source); copy a point.
const COPY_POINT: u32 = 0x0045_8620;
const COPY_POINT_004A3970: u32 = 0x004a_3970;
/// `004b5950` / `004b5a80` (unnamed, `globalfunc.cpp`): `cdecl`, one word each; a
/// collidable's object and its havok body.
const COLLIDABLE_OBJECT: u32 = 0x004b_5950;
const OBJECT_BODY: u32 = 0x004b_5a80;
/// `0043b4f0` (`thiscall`, one word, `RET 4`) then `0043b4d0` (`thiscall`): a body's layer.
const LAYER_SOURCE: u32 = 0x0043_b4f0;
const LAYER: u32 = 0x0043_b4d0;
/// `Script::SetActionFlag` (`005ac750`, Xbox PDB): `cdecl`, three words.
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `005d43c0` (unnamed): `thiscall`; the target's extra-data list.
const EXTRA_LIST: u32 = 0x005d_43c0;
/// `00476c90` (unnamed, `bgsdestructibleobjectform.cpp`): `thiscall`; a word.
const PROJECTILE_00476C90: u32 = 0x0047_6c90;
/// `0043b300` (unnamed): `cdecl` (an `NiRTTI`, an object); whether the object is of that
/// class. The `NiRTTI` at `0x01268444` (`bhkCollisionBox` by its static initializer's name).
const IS_KIND_OF: u32 = 0x0043_b300;
const RTTI_0X01268444: u32 = 0x0126_8444;
/// `FindFirstCollisionObject` (`004b5260`, Xbox PDB) and `006fa820` (its rigid body).
const FIND_FIRST_COLLISION_OBJECT: u32 = 0x004b_5260;
const RIGID_BODY: u32 = 0x006f_a820;
/// `009c19e0` (unnamed, `projectile.cpp`): `thiscall` on a layer word; a `bool`.
const LAYER_TEST: u32 = 0x009c_19e0;
/// The ray helpers (see `run_009b36f0`).
const RAY_CONSTRUCT: u32 = 0x004a_3c20;
const MATRIX_ROTATION: u32 = 0x004a_0c90;
const GET_LOOKING: u32 = 0x0093_1d70;
const MAKE_X_ROTATION: u32 = 0x0052_4ac0;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const MATRIX_COLUMN: u32 = 0x0043_9f50;
const POINT_NORMALIZE: u32 = 0x004a_0c10;
const POINT_SCALED: u32 = 0x0045_bb20;
const POINT_ADD: u32 = 0x0043_9e90;
const RAY_SET_START: u32 = 0x004a_3da0;
const RAY_SET_END: u32 = 0x007f_68a0;
const RAY_SET_COLLECTOR: u32 = 0x004a_3fb0;
const COLLECTOR_CONSTRUCT: u32 = 0x004a_3a70;
const COLLECTOR_DESTRUCT: u32 = 0x004a_3bc0;
const RAY_CAST: u32 = 0x009c_5ba0;
const RAY_HITS: u32 = 0x008c_dd90;
const HIT_COUNT: u32 = 0x0072_6070;
const HIT_AT: u32 = 0x004a_46b0;
const HIT_COPY: u32 = 0x0069_6c30;
/// `1000.0` (`float`, `0x01013974`): the ray's length.
const RAY_LENGTH: u32 = 0x0101_3974;

// Translated from 009c6490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c6490 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated.
/// Unnamed in the engine map (`thiscall`, `RET 8`; the first word is not read; a contact
/// listener of the projectile at `+0x240`): with a projectile, reports the contact `hit`
/// (its point `006815c0`, normal `00460140`, collidables `+0x48` and `+0x4c`) to the
/// projectile's slot `+0x32c` with the target reference (`fn_009c69e0`) and 0x24. A target
/// whose body has layer 0x17 or 0x18 only gets the script action flags 0x80 (and 0x100
/// for `00476c90`'s script). A `bhkCollisionBox` body or layer 0x1e reports nothing unless
/// the target's rigid body passes `009c19e0` or has no slot `+0x94` object and a 1000-long
/// ray along the projectile (`009c5ba0`) hits: then the hit's point (the fraction at `+0x10`
/// along the ray), normal and collidable (`+0x50`) are reported (the last valid shape key
/// of the hit's eight at `+0x20` is found and not used).
pub fn fn_009c6490(e: &mut Engine, this: Ptr, _unused_1: u32, hit: u32) {
    let projectile = e.mem.u32(this.addr() + 0x240);
    if projectile == 0 {
        return;
    }
    let collidable_word = e.mem.u32(hit + 0x48);
    let target = fn_009c69e0(e, collidable_word);
    e.with_stack(0x580, |e, frame| {
        let base = frame.addr() + 0x580;
        let at = |offset: u32| base - offset;
        let point = at(0x2c);
        let normal = at(0x38);
        e.call(SELF_006815C0, &args![point]);
        let source = e.call(SELF_006815C0, &args![hit]).u32();
        e.call(COPY_POINT, &args![point, source]);
        e.call(SELF_006815C0, &args![normal]);
        let source = e.call(MEMBER_AT_10, &args![hit]).u32();
        e.call(COPY_POINT_004A3970, &args![normal, source]);
        let mut first = e.mem.u32(hit + 0x48);
        let second = e.mem.u32(hit + 0x4c);
        let object = e.call(COLLIDABLE_OBJECT, &args![first]).u32();
        let body = if object != 0 {
            e.call(OBJECT_BODY, &args![object]).u32()
        } else {
            0
        };
        if target != 0 && body != 0 {
            let source = e.call(LAYER_SOURCE, &args![body, at(0x50)]).u32();
            let mut flagged = e.call(LAYER, &args![source]).u32() == 0x17;
            if !flagged {
                let source = e.call(LAYER_SOURCE, &args![body, at(0x54)]).u32();
                flagged = e.call(LAYER, &args![source]).u32() == 0x18;
            }
            if flagged {
                let list = e.call(EXTRA_LIST, &args![target]).u32();
                e.call(SET_ACTION_FLAG, &args![projectile, list, 0x80u32]);
                if e.call(PROJECTILE_00476C90, &args![projectile]).u32() != 0 {
                    let list = e.call(EXTRA_LIST, &args![target]).u32();
                    let script = e.call(PROJECTILE_00476C90, &args![projectile]).u32();
                    e.call(SET_ACTION_FLAG, &args![script, list, 0x100u32]);
                }
                return;
            }
        }
        let mut skip = false;
        if body != 0 {
            let mut special = e.call(IS_KIND_OF, &args![RTTI_0X01268444, body]).u8() != 0;
            if !special {
                let source = e.call(LAYER_SOURCE, &args![body, at(0x5c)]).u32();
                special = e.call(LAYER, &args![source]).u32() == 0x1e;
            }
            if special {
                skip = true;
                let target_node = if target != 0 {
                    e.vcall(target, 0x1d0, &[]).u32()
                } else {
                    0
                };
                let collision = e
                    .call(FIND_FIRST_COLLISION_OBJECT, &args![target_node])
                    .u32();
                let rigid = if collision != 0 {
                    e.call(RIGID_BODY, &args![collision]).u32()
                } else {
                    0
                };
                let mut cast = false;
                if rigid != 0 {
                    let source = e.call(LAYER_SOURCE, &args![rigid, at(0x74)]).u32();
                    cast = e.call(LAYER_TEST, &args![source]).u8() != 0
                        || e.vcall(rigid, 0x94, &[]).u32() == 0;
                }
                if cast {
                    let ray = at(0x130);
                    let start = at(0x13c);
                    let end = at(0x148);
                    let direction = at(0x154);
                    let matrix = at(0x178);
                    let looking = at(0x19c);
                    e.call(RAY_CONSTRUCT, &args![ray]);
                    e.call(SELF_006815C0, &args![start]);
                    e.call(SELF_006815C0, &args![end]);
                    e.call(SELF_006815C0, &args![direction]);
                    let position = e.vcall(projectile, 0x1f4, &[]).u32();
                    let words = e.mem.bytes(position, 0xc);
                    e.mem.write(start, &words);
                    e.call(SELF_006815C0, &args![matrix]);
                    let facing = e.vcall(projectile, 0x2bc, &args![0u32]).f64();
                    e.call(MATRIX_ROTATION, &args![matrix, x87_store32(facing)]);
                    e.call(SELF_006815C0, &args![looking]);
                    let look = e.call(GET_LOOKING, &args![projectile]).f64();
                    e.call(MAKE_X_ROTATION, &args![looking, x87_store32(look)]);
                    let product = e
                        .call(MATRIX_MULTIPLY, &args![matrix, at(0x1c0), looking])
                        .u32();
                    let words = e.mem.bytes(product, 0x24);
                    e.mem.write(matrix, &words);
                    e.call(MATRIX_COLUMN, &args![matrix, 1u32, direction]);
                    e.call(POINT_NORMALIZE, &args![direction]);
                    let length = x87_quiet_bits(e.mem.u32(RAY_LENGTH));
                    let scaled = e
                        .call(POINT_SCALED, &args![direction, at(0x1cc), length])
                        .u32();
                    let words = e.mem.bytes(scaled, 0xc);
                    e.mem.write(end, &words);
                    e.call(RAY_SET_START, &args![ray, start]);
                    e.call(RAY_SET_END, &args![ray, end]);
                    let collector = at(0x4f0);
                    e.call(COLLECTOR_CONSTRUCT, &args![collector]);
                    e.call(RAY_SET_COLLECTOR, &args![ray, collector]);
                    let hit_something =
                        e.call(RAY_CAST, &args![ray, target_node, 0u32, 1u32]).u8() != 0;
                    if hit_something {
                        let holder = e.call(RAY_HITS, &args![ray]).u32();
                        let hits = e.call(MEMBER_AT_10, &args![holder]).u32();
                        if e.call(HIT_COUNT, &args![hits]).i32() > 0 {
                            skip = false;
                            let holder = e.call(RAY_HITS, &args![ray]).u32();
                            let hits = e.call(MEMBER_AT_10, &args![holder]).u32();
                            let first_hit = e.call(HIT_AT, &args![hits, 0u32]).u32();
                            let copy = at(0x560);
                            e.call(HIT_COPY, &args![copy, first_hit]);
                            let fraction = x87_quiet_bits(e.mem.u32(copy + 0x10));
                            let along =
                                e.call(POINT_SCALED, &args![end, at(0x56c), fraction]).u32();
                            let sum = e.call(POINT_ADD, &args![start, at(0x578), along]).u32();
                            let words = e.mem.bytes(sum, 0xc);
                            e.mem.write(point, &words);
                            e.call(COPY_POINT_004A3970, &args![normal, copy]);
                            first = e.mem.u32(copy + 0x50);
                            let mut _shape_key = 0xffff_ffffu32;
                            let mut index = 0i32;
                            while index < 8 {
                                let key = e.mem.u32(copy + 0x20 + 4 * index as u32);
                                if key == 0xffff_ffff {
                                    break;
                                }
                                _shape_key = key;
                                index += 1;
                            }
                        }
                    }
                    e.call(COLLECTOR_DESTRUCT, &args![collector]);
                }
            }
        }
        if !skip {
            e.vcall(
                projectile,
                0x32c,
                &args![target, point, normal, first, second, 0x24u32],
            );
        }
    });
}

// Translated from 009c69e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the reference of a collidable's object
/// (`bhkUtilFunctions::GetNiAVObject`, `TESObjectREFR::FindReferenceFor3D`), 0 when it has
/// none.
pub fn fn_009c69e0(e: &mut Engine, collidable: u32) -> u32 {
    let object = e.call(GET_NI_AV_OBJECT, &args![collidable]).u32();
    if object != 0 {
        e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32()
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009c_6490, fn_009c6490(Ptr, u32, u32)),
        entry!(0x009c_69e0, fn_009c69e0(u32) -> u32),
        entry!(0x009c_6a20, fn_009c6a20(Ptr, u32) -> u32),
        entry!(0x009c_6a30, fn_009c6a30(Ptr) -> Ptr),
        entry!(0x009c_6b30, fn_009c6b30(Ptr)),
    ]
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_009c6a20() {
        assert_eq!(
            check(0x009c_6a20, 0x008d_1130, 0x3e0, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn calls_in_order_and_clears_the_fields() {
        let mut e = Engine::new();
        e.map(CLEARED_GLOBAL_011F21B8 & !0xfff, 0x1000);
        e.set_global(CLEARED_GLOBAL_011F21B8, 0x55u32);
        e.register(BASE_CONSTRUCTOR_0096A2D0, |_, _| Ret::default());
        e.register(MEMBER_CONSTRUCTOR_00633C90, |_, _| Ret::default());
        e.register(MEMBER_CONSTRUCTOR_0066B0D0, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        for i in 0..0x13u32 {
            e.mem.set_u32(this + i * 4, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009c_6a30, &args![this]).u32(), this);
        let order: Vec<u32> = e
            .call_log
            .clone()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect();
        assert_eq!(
            order,
            vec![
                0x009c_6a30,
                BASE_CONSTRUCTOR_0096A2D0,
                MEMBER_CONSTRUCTOR_00633C90,
                MEMBER_CONSTRUCTOR_0066B0D0
            ]
        );
        assert_eq!(
            calls_to(&mut e, MEMBER_CONSTRUCTOR_00633C90),
            vec![vec![this + 0x34, 0]]
        );
        assert_eq!(
            calls_to(&mut e, MEMBER_CONSTRUCTOR_0066B0D0),
            vec![vec![this + 0x34, 0]]
        );
        assert_eq!(e.global::<u32>(CLEARED_GLOBAL_011F21B8), 0);
        for offset in [
            0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x3c, 0x40, 0x44,
        ] {
            assert_eq!(e.mem.u32(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x38), 1);
        // The words between the cleared ones (+0 to +7, +0x30 to +0x37) are not written here.
        assert_eq!(e.mem.u32(this), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x30), 0xffff_ffff);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_009c6b30_clears_then_destroys_member_and_base() {
        let mut e = Engine::new();
        for callee in [
            CLEAR_DATA,
            MEMBER_DESTRUCTOR_0045CEC0,
            BASE_DESTRUCTOR_0046FFB0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x009c_6b30, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (CLEAR_DATA, vec![this]),
                (MEMBER_DESTRUCTOR_0045CEC0, vec![this + 0x34]),
                (BASE_DESTRUCTOR_0046FFB0, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod contact_tests {
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

    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x400);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        let p = e.mem.alloc(0x200);
        e.mem.set_u32(p, table);
        p
    }

    #[test]
    fn fn_009c69e0_finds_the_reference() {
        let mut e = Engine::new();
        e.register(GET_NI_AV_OBJECT, |e, a| ret(e.mem.u32(a[0])));
        e.register(FIND_REFERENCE_FOR_3D, |_, a| ret(a[0] + 1));
        let collidable = e.mem.alloc(4);
        assert_eq!(e.call(0x009c_69e0, &args![collidable]).u32(), 0);
        e.mem.set_u32(collidable, 0x5000);
        assert_eq!(e.call(0x009c_69e0, &args![collidable]).u32(), 0x5001);
    }

    /// Doubles: a collidable `c` has its object at `[c]` (the reference at `+0x10` of it,
    /// the body at `+0x14`); a body's layer is its `+0x10`, its kind word `+0x14`.
    fn doubles(e: &mut Engine) {
        e.register(GET_NI_AV_OBJECT, |e, a| ret(e.mem.u32(a[0])));
        e.register(FIND_REFERENCE_FOR_3D, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(SELF_006815C0, |_, a| ret(a[0]));
        e.register(MEMBER_AT_10, |_, a| ret(a[0] + 0x10));
        e.register(COLLIDABLE_OBJECT, |e, a| ret(e.mem.u32(a[0])));
        e.register(OBJECT_BODY, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(LAYER_SOURCE, |_, a| ret(a[0]));
        e.register(LAYER, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(EXTRA_LIST, |_, a| ret(a[0] + 0x100));
        e.register(PROJECTILE_00476C90, |_, _| ret(0x6600));
        e.register(IS_KIND_OF, |e, a| {
            ret((e.mem.u32(a[1] + 0x14) == a[0]) as u32)
        });
        e.register(FIND_FIRST_COLLISION_OBJECT, |_, _| ret(0x4c00));
        e.register(RIGID_BODY, |e, _| ret(e.mem.u32(0x7e00)));
        e.register(LAYER_TEST, |_, _| ret(1));
        e.register(MATRIX_MULTIPLY, |_, a| ret(a[1]));
        e.register(GET_LOOKING, |_, _| Ret::default());
        e.register(POINT_SCALED, |_, a| ret(a[1]));
        e.register(POINT_ADD, |e, a| {
            e.mem.set_u32(a[1], 0x1234);
            ret(a[1])
        });
        e.register(RAY_CAST, |_, _| ret(1));
        e.register(RAY_HITS, |_, _| ret(0x7e40));
        e.register(HIT_COUNT, |_, _| ret(1));
        e.register(HIT_AT, |_, _| ret(0x7e80));
        e.register(HIT_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x60);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        for addr in [
            COPY_POINT,
            COPY_POINT_004A3970,
            SET_ACTION_FLAG,
            RAY_CONSTRUCT,
            MATRIX_ROTATION,
            MAKE_X_ROTATION,
            MATRIX_COLUMN,
            POINT_NORMALIZE,
            RAY_SET_START,
            RAY_SET_END,
            RAY_SET_COLLECTOR,
            COLLECTOR_CONSTRUCT,
            COLLECTOR_DESTRUCT,
            0x732c,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(0x71f4, |_, a| ret(a[0] + 0x40));
        e.register(0x72bc, |_, _| Ret::default());
        e.register(0x71d0, |_, _| ret(0x0d00));
        e.map(0x7e00, 0x200);
        e.map(RAY_LENGTH & !0xfff, 0x1000);
        e.mem.set_f32(RAY_LENGTH, 1000.0);
    }

    struct Contact {
        listener: u32,
        projectile: u32,
        hit: u32,
        body: u32,
        reference: u32,
    }

    fn contact(e: &mut Engine) -> Contact {
        let projectile = object(e, &[(0x32c, 0x732c), (0x1f4, 0x71f4), (0x2bc, 0x72bc)]);
        let listener = e.mem.alloc(0x250);
        e.mem.set_u32(listener + 0x240, projectile);
        let reference = object(e, &[(0x1d0, 0x71d0)]);
        let body = e.mem.alloc(0x20);
        let av = e.mem.alloc(0x20);
        e.mem.set_u32(av + 0x10, reference);
        e.mem.set_u32(av + 0x14, body);
        let collidable = e.mem.alloc(4);
        e.mem.set_u32(collidable, av);
        let hit = e.mem.alloc(0x60);
        e.mem.set_u32(hit + 0x48, collidable);
        e.mem.set_u32(hit + 0x4c, 0x77);
        Contact {
            listener,
            projectile,
            hit,
            body,
            reference,
        }
    }

    #[test]
    fn fn_009c6490_reports_the_contact() {
        let mut e = Engine::new();
        doubles(&mut e);
        let c = contact(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x009c_6490, &args![c.listener, 0u32, c.hit]);
        let reported = calls(&e, 0x732c);
        assert_eq!(reported.len(), 1);
        assert_eq!(reported[0][0], c.projectile);
        assert_eq!(reported[0][1], c.reference);
        assert_eq!(reported[0][4..], [e.mem.u32(c.hit + 0x48), 0x77, 0x24]);
        assert_eq!(calls(&e, COPY_POINT)[0][1], c.hit);
        assert_eq!(calls(&e, COPY_POINT_004A3970)[0][1], c.hit + 0x10);
        // No projectile: nothing.
        e.mem.set_u32(c.listener + 0x240, 0);
        e.call_log = Some(vec![]);
        e.call(0x009c_6490, &args![c.listener, 0u32, c.hit]);
        assert!(calls(&e, 0x732c).is_empty());
    }

    #[test]
    fn fn_009c6490_flags_the_script_on_layer_0x17() {
        let mut e = Engine::new();
        doubles(&mut e);
        let c = contact(&mut e);
        e.mem.set_u32(c.body + 0x10, 0x18);
        e.call_log = Some(vec![]);
        e.call(0x009c_6490, &args![c.listener, 0u32, c.hit]);
        assert_eq!(
            calls(&e, SET_ACTION_FLAG),
            vec![
                vec![c.projectile, c.reference + 0x100, 0x80],
                vec![0x6600, c.reference + 0x100, 0x100]
            ]
        );
        assert!(calls(&e, 0x732c).is_empty());
    }

    #[test]
    fn fn_009c6490_casts_a_ray_for_a_collision_box() {
        let mut e = Engine::new();
        doubles(&mut e);
        let c = contact(&mut e);
        e.mem.set_u32(c.body + 0x14, RTTI_0X01268444);
        e.mem.set_u32(0x7e00, 0x6100);
        e.mem.set_f32(0x7e80 + 0x10, 0.25);
        e.mem.set_u32(0x7e80 + 0x50, 0x5550);
        e.call_log = Some(vec![]);
        e.call(0x009c_6490, &args![c.listener, 0u32, c.hit]);
        let reported = calls(&e, 0x732c);
        assert_eq!(reported.len(), 1);
        assert_eq!(reported[0][4], 0x5550);
        assert_eq!(e.mem.u32(reported[0][2]), 0x1234);
        let scaled: Vec<u32> = calls(&e, POINT_SCALED).iter().map(|s| s[2]).collect();
        assert_eq!(scaled, vec![1000.0f32.to_bits(), 0.25f32.to_bits()]);
        assert_eq!(calls(&e, RAY_CAST)[0][1], 0x0d00);
        // No hit: nothing reported.
        e.register(RAY_CAST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x009c_6490, &args![c.listener, 0u32, c.hit]);
        assert!(calls(&e, 0x732c).is_empty());
        assert_eq!(calls(&e, COLLECTOR_DESTRUCT).len(), 1);
    }
}
