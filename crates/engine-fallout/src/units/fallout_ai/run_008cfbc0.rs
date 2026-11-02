//! `fallout/ai/run_008cfbc0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d0270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor of the object `00906dc0` builds (unnamed in the engine
/// map; the class is unconfirmed). `ECX` is the object. It stores the virtual
/// table `01085e2c`, runs `008c5e20` on the member at `+4`, clears the word at
/// `+0x2c` and returns `this`.
pub fn fn_008d0270(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0108_5e2c);
    e.call(0x008c_5e20, &args![base + 4]);
    e.mem.set_u32(base + 0x2c, 0);
    this
}

/// Callees of `fn_008cffa0`: `00620b80` (`thiscall`, no arguments, answers an object),
/// `00a29680` (`thiscall`, no arguments; an empty function in this build), `005609f0`
/// (`thiscall`, no arguments, answers a byte) and, for `fn_008cfff0`, `00622570`
/// (`thiscall`, `(flag)`).
const OBJECT_00620B80: u32 = 0x0062_0b80;
const EMPTY_00A29680: u32 = 0x00a2_9680;
const BYTE_OF_005609F0: u32 = 0x0056_09f0;
const SET_FLAG_00622570: u32 = 0x0062_2570;

// Translated from 008cfff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(&result)`): `00622570` on `result` with whether the
/// word at `this + 8` is non-zero. Answers `result`.
pub fn fn_008cfff0(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let set = u32::from(e.mem.u32(this.addr() + 8) != 0);
    e.call(SET_FLAG_00622570, &args![result, set]);
    result
}

// Translated from 008cffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments): false when `00620b80` answers no object;
/// otherwise, between two calls of `00a29680` on `this`, the byte `005609f0` answers for the
/// local `fn_008cfff0` fills from the object.
pub fn fn_008cffa0(e: &mut Engine, this: Ptr) -> u8 {
    let object = e.call(OBJECT_00620B80, &args![this]).u32();
    if object == 0 {
        return 0;
    }
    e.call(EMPTY_00A29680, &args![this]);
    let answer = e.with_stack(4, |e, local| {
        let filled = fn_008cfff0(e, Ptr::new(object), local);
        e.call(BYTE_OF_005609F0, &args![filled]).u8()
    });
    e.call(EMPTY_00A29680, &args![this]);
    answer
}

// Translated from 008d01e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float getter (unnamed in the map; `thiscall`, no arguments): the float at
/// `this + 0x98`, in ST0.
pub fn fn_008d01e0(e: &mut Engine, this: Ptr) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(0x98))))
}

// ---- n028: the arrow's ray against a node tree and its push ----

/// `bPostForce` of `ArrowProjectile` (Xbox PDB +0xD3, PC +0xC3).
const POST_FORCE: u32 = 0xc3;
/// `psCollisionData` (PC +0x88) and its record's `pCollidee` (+0x28).
const COLLISION: u32 = 0x88;
const COLLISION_COLLIDEE: u32 = 0x28;
/// The three-float vector at `011f426c` (zero in the file's data).
const ZERO_POINT: u32 = 0x011f_426c;
/// `00403e20` (`thiscall` on a `Setting`): the address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The no-op constructor of an `NiPoint3` (`006815c0`) and `004a3e00`
/// (cdecl `(destination, source)`), which fills the 16-byte vector at the
/// destination from a point.
const CONSTRUCT_IN_PLACE: u32 = 0x0068_15c0;
const VECTOR_FROM_POINT: u32 = 0x004a_3e00;
/// `0043b4f0` (`thiscall(out)` on a rigid body): its filter word into `out`;
/// `0043b4d0` (`thiscall`): the low 7 bits of that word; `006240b0`
/// (`thiscall`): another field of it.
const BODY_GET_FILTER: u32 = 0x0043_b4f0;
const FILTER_LOW_BITS: u32 = 0x0043_b4d0;
const FILTER_PART: u32 = 0x0062_40b0;
/// `bhkCharacterProxy::operatorP` (engine map name of `004ae750`,
/// `thiscall`): the object the code passes on.
const BODY_OBJECT: u32 = 0x004a_e750;

// Translated from 008cfbc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008cfbc0 - the /GS cookie check (00ec408c) is not translated
/// The arrow's ray (cdecl, nine words): from `origin` along `direction`
/// for `*distance` against `node` and its children (slot 0xC, `0043b480`
/// count, `0043b4a0` child, depth first). A node is tested when its
/// collision object (`bhkCollisionObject::GetbhkCollisionObject`) has a body
/// (`006fa820`) with a shape (`004ae6a0`) whose filter's low bits are 8 and
/// whose `006240b0` part is neither 0x12 nor 0x13 (nor 1 when `skip_1`):
/// the ray's ends go into the body's space (its transform from slot 0xF4,
/// or for an object of the `01267e64` kind, unless `008cffa0` holds and
/// `004a7bd0` is not 1, the node's rotation through `NI2HK`, combined
/// (`hkTransform::setMul`) for a body of the `012683c8` kind with the
/// transform `00632b40`/`008cd6a0` build from it) and are cast against the
/// shape (`00632c30`); a hit (`00632c60`, `005609f0`) scales `*distance` by
/// its fraction and makes this node the answer. Returns the first node
/// that was hit (children first), or null.
#[allow(clippy::too_many_arguments)]
pub fn fn_008cfbc0(
    e: &mut Engine,
    node: Ptr,
    origin_x: u32,
    origin_y: u32,
    origin_z: u32,
    direction_x: u32,
    direction_y: u32,
    direction_z: u32,
    distance: Ptr,
    skip_1: bool,
) -> Ptr {
    if node.is_null() {
        return Ptr::NULL;
    }
    // The frame (offsets are the exe's `EBP - n`; the stack arguments are
    // copied to `args` so their addresses can be passed as the exe does).
    let hit = e.with_stack(0x1f8, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x1e0 - offset;
        let arguments = frame.addr() + 0x1e0;
        let origin = arguments;
        let direction = arguments + 0xc;
        for (i, word) in [
            origin_x,
            origin_y,
            origin_z,
            direction_x,
            direction_y,
            direction_z,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_u32(arguments + 4 * i as u32, word);
        }
        let object = e.call(0x0043_b610, &args![node]).u32();
        if object == 0 {
            return false;
        }
        let body = e.call(0x006f_a820, &args![object]).u32();
        let shape = if body == 0 {
            0
        } else {
            e.call(0x004a_e6a0, &args![body]).u32()
        };
        if shape == 0 {
            return false;
        }
        let filter = e.call(BODY_GET_FILTER, &args![body, local(0x1c)]).u32();
        if e.call(FILTER_LOW_BITS, &args![filter]).u32() != 8 {
            return false;
        }
        let filter = e.call(BODY_GET_FILTER, &args![body, local(0x20)]).u32();
        if e.call(FILTER_PART, &args![filter]).u32() == 0x12 {
            return false;
        }
        let filter = e.call(BODY_GET_FILTER, &args![body, local(0x24)]).u32();
        if e.call(FILTER_PART, &args![filter]).u32() == 0x13 {
            return false;
        }
        if skip_1 {
            let filter = e.call(BODY_GET_FILTER, &args![body, local(0x28)]).u32();
            if e.call(FILTER_PART, &args![filter]).u32() == 1 {
                return false;
            }
        }
        let from = local(0x40);
        e.call(CONSTRUCT_IN_PLACE, &args![from]);
        e.call(VECTOR_FROM_POINT, &args![from, origin]);
        let to = local(0x50);
        e.call(CONSTRUCT_IN_PLACE, &args![to]);
        let reach = x87_quiet_bits(e.mem.u32(distance.addr()));
        let along = e
            .call(0x0045_bb20, &args![direction, local(0x5c), reach])
            .u32();
        let end = e
            .call(0x0043_9e90, &args![origin, local(0x68), along])
            .u32();
        e.call(VECTOR_FROM_POINT, &args![to, end]);
        let input = local(0xa0);
        let output = local(0xf0);
        let transform = local(0x130);
        e.call(0x0063_2ca0, &args![input]);
        e.call(0x004a_3d20, &args![output]);
        e.call(0x0056_ded0, &args![transform]);
        let mut own_transform = true;
        if e.call(0x0043_b300, &args![0x0126_7e64u32, object]).bool() {
            own_transform = false;
            let distance_object = object;
            if fn_008cffa0(e, Ptr::new(body)) != 0 {
                let traveled = e.call(0x004a_7bd0, &args![distance_object]).f64();
                // `FCOMP 1.0; TEST AH,0x44; JP`: anything but exactly 1.
                if traveled != 1.0 {
                    own_transform = true;
                }
            }
            if !own_transform {
                let rotation = e.call(0x0046_1130, &args![node]).u32();
                e.call(0x0056_df80, &args![transform, rotation]);
                if e.call(0x0043_b300, &args![0x0126_83c8u32, body]).bool() {
                    let local_transform = local(0x180);
                    let product = local(0x1c0);
                    e.call(0x0056_ded0, &args![local_transform]);
                    e.call(0x0063_2b40, &args![product, transform]);
                    let position = e.call(0x0043_6aa0, &args![body]).u32();
                    let rotation = e.call(0x0089_1170, &args![body]).u32();
                    e.call(0x008c_d6a0, &args![local_transform, rotation, position]);
                    e.call(0x00cb_2240, &args![transform, product, local_transform]);
                }
            }
        }
        if own_transform {
            e.vcall(body, 0xf4, &args![transform]);
        }
        e.call(0x00c7_f690, &args![input, transform, from]);
        e.call(0x00c7_f690, &args![input + 0x10, transform, to]);
        let cast = e.call(BODY_OBJECT, &args![shape]).u32();
        e.call(0x0063_2c30, &args![cast, local(0x1c1), input, output]);
        let result = e.call(0x0063_2c60, &args![output, local(0x1c2)]).u32();
        if !e.call(0x0056_09f0, &args![result]).bool() {
            return false;
        }
        let scaled = x87_mul(
            x87_load32(e.mem.u32(distance.addr())),
            x87_m32(e.mem.u32(output + 0x10)),
        );
        e.mem.set_u32(distance.addr(), x87_store32(scaled));
        true
    });
    let children = e.vcall(node.addr(), 0xc, &[]).u32();
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(0x0043_b480, &args![children]).u32() {
            let child = e.call(0x0043_b4a0, &args![children, index]).u32();
            let found = fn_008cfbc0(
                e,
                Ptr::new(child),
                origin_x,
                origin_y,
                origin_z,
                direction_x,
                direction_y,
                direction_z,
                distance,
                skip_1,
            );
            if !found.is_null() {
                return found;
            }
            index += 1;
        }
    }
    if hit {
        node
    } else {
        Ptr::NULL
    }
}

// Translated from 008d0020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008d0020 - the /GS cookie check (00ec408c) is not translated
/// The arrow pushes `body` at `point` along `direction` (seven words), once
/// (`bPostForce`, `fn_008d0220(1)`): with the object of slot 0x98, the
/// speed (`fn_008d01e0`) times the setting for the body's filter bits (8:
/// `011cecac`, 10: `011cf9d4`, 14: `011ce3e4`, else `011ce798`), scaled by
/// `mass / limit` when the mass (`004b5400`) is below the `011cf114`
/// setting, times the `011ce224` setting, goes as an impulse (`0062aea0`,
/// `005dbf20`, `00627920`) along the direction to `0081f1e0` on the body's
/// `004ae750` object at the point, between two `00483710` calls on the
/// object. Without a body, or already pushed, nothing happens.
#[allow(clippy::too_many_arguments)]
pub fn fn_008d0020(
    e: &mut Engine,
    this: Ptr,
    point_x: u32,
    point_y: u32,
    point_z: u32,
    direction_x: u32,
    direction_y: u32,
    direction_z: u32,
    body: Ptr,
) {
    if body.is_null() || fn_008d0200(e, this) != 0 {
        return;
    }
    let lock = e.vcall(body.addr(), 0x98, &[]).u32();
    if lock != 0 {
        e.with_stack(0x68, |e, frame| {
            // The frame (offsets are the exe's `EBP - n`) and the stack arguments
            // copied after it (point, then direction), as the exe passes their
            // addresses.
            let local = |offset: u32| frame.addr() + 0x50 - offset;
            let point = frame.addr() + 0x50;
            let direction = point + 0xc;
            for (i, word) in [
                point_x,
                point_y,
                point_z,
                direction_x,
                direction_y,
                direction_z,
            ]
            .into_iter()
            .enumerate()
            {
                e.mem.set_u32(point + 4 * i as u32, word);
            }
            let mut speed = fn_008d01e0(e, this).to_bits();
            let filter = local(0x14);
            e.call(BODY_GET_FILTER, &args![body, filter]);
            let setting = match e.call(FILTER_LOW_BITS, &args![filter]).u32() {
                8 => 0x011c_ecac,
                10 => 0x011c_f9d4,
                14 => 0x011c_e3e4,
                _ => 0x011c_e798,
            };
            let factor = e.call(SETTING_VALUE, &args![setting]).u32();
            speed = x87_store32(x87_mul(x87_load32(speed), x87_m32(e.mem.u32(factor))));
            let mass = e.call(0x004b_5400, &args![body]).f64();
            let mass = x87_store32(mass);
            let limit = e.call(SETTING_VALUE, &args![0x011c_f114u32]).u32();
            // `FCOMPP; TEST AH,0x41; JNZ`: only a mass below the limit.
            if x87_load32(e.mem.u32(limit)) > x87_load32(mass) {
                let limit = e.call(SETTING_VALUE, &args![0x011c_f114u32]).u32();
                let ratio = x87_div(x87_load32(mass), x87_m32(e.mem.u32(limit)));
                speed = x87_store32(x87_mul(ratio, x87_m32(speed)));
            }
            let factor = e.call(SETTING_VALUE, &args![0x011c_e224u32]).u32();
            speed = x87_store32(x87_mul(x87_load32(speed), x87_m32(e.mem.u32(factor))));
            let along = local(0x30);
            e.call(CONSTRUCT_IN_PLACE, &args![along]);
            e.call(VECTOR_FROM_POINT, &args![along, direction]);
            let at = local(0x40);
            e.call(CONSTRUCT_IN_PLACE, &args![at]);
            e.call(VECTOR_FROM_POINT, &args![at, point]);
            e.call(0x0062_aea0, &args![along]);
            let impulse = local(0x50);
            e.call(0x005d_bf20, &args![impulse, x87_quiet_bits(speed)]);
            e.call(0x0062_7920, &args![along, impulse]);
            e.call(0x0048_3710, &args![lock]);
            let object = e.call(BODY_OBJECT, &args![body]).u32();
            e.call(0x0081_f1e0, &args![object, along, at]);
            e.call(0x0048_3710, &args![lock]);
        });
    }
    fn_008d0220(e, this, 1);
}

// Translated from 008d0200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bPostForce` (the byte at +0xC3).
pub fn fn_008d0200(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + POST_FORCE)
}

// Translated from 008d0220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bPostForce` (the byte at +0xC3) to the low byte of the argument.
pub fn fn_008d0220(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + POST_FORCE, value);
}

// Translated from 008d0240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the collision record's collidee (when there is a record).
pub fn fn_008d0240(e: &mut Engine, this: Ptr) {
    let record = e.mem.u32(this.addr() + COLLISION);
    if record != 0 {
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0);
    }
}

// Translated from 008d02a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the vector at `011f426c` to `result` and returns it; the second
/// stack word (`RET 8`) is not read.
pub fn fn_008d02a0(e: &mut Engine, _this: Ptr, result: Ptr, _unused_2: u32) -> Ptr {
    for i in 0..3 {
        let word = e.mem.u32(ZERO_POINT + 4 * i);
        e.mem.set_u32(result.addr() + 4 * i, word);
    }
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_0270, fn_008d0270(Ptr) -> Ptr),
        entry!(0x008cfff0, fn_008cfff0(Ptr, Ptr) -> Ptr),
        entry!(0x008cffa0, fn_008cffa0(Ptr) -> u8),
        entry!(0x008d01e0, fn_008d01e0(Ptr) -> f32),
        entry!(0x008c_fbc0, fn_008cfbc0(Ptr, u32, u32, u32, u32, u32, u32, Ptr, bool) -> Ptr),
        entry!(
            0x008d_0020,
            fn_008d0020(Ptr, u32, u32, u32, u32, u32, u32, Ptr)
        ),
        entry!(0x008d_0200, fn_008d0200(Ptr) -> u8),
        entry!(0x008d_0220, fn_008d0220(Ptr, u8)),
        entry!(0x008d_0240, fn_008d0240(Ptr)),
        entry!(0x008d_02a0, fn_008d02a0(Ptr, Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_008d0270_builds_the_base() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x008d_0270,
            size: 0x30,
            globals: &[],
            returns: &[],
            calls: &[(0x008c_5e20, &[A::T(4)])],
            stores: &[(0x00, 4, 0x0108_5e2c), (0x2c, 4, 0)],
        });
    }

    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_008cfff0_passes_whether_the_word_is_set() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(SET_FLAG_00622570, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x008cfff0, &args![this, 0x7000u32]).u32(), 0x7000);
        e.mem.set_u32(this + 8, 0x55);
        e.call(0x008cfff0, &args![this, 0x7000u32]);
        assert_eq!(
            calls_at(&e, SET_FLAG_00622570),
            vec![vec![0x7000, 0], vec![0x7000, 1]]
        );
    }

    #[test]
    fn fn_008cffa0_reads_the_objects_byte() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(OBJECT_00620B80, |_, _| Ret::default());
        e.register(EMPTY_00A29680, |_, _| Ret::default());
        e.register(SET_FLAG_00622570, |e, a| {
            e.mem.set_u8(a[0], 0x40 + a[1] as u8);
            Ret::default()
        });
        e.register(BYTE_OF_005609F0, |e, a| Ret {
            eax: 0x100 | u32::from(e.mem.u8(a[0])),
            ..Ret::default()
        });
        assert_eq!(e.call(0x008cffa0, &args![0x5000u32]).u8(), 0);
        assert!(calls_at(&e, EMPTY_00A29680).is_empty());
        let object = e.mem.alloc(0xc);
        e.mem.set_u32(object + 8, 1);
        e.register_double(OBJECT_00620B80, move |_, _| Ret {
            eax: object,
            ..Ret::default()
        });
        assert_eq!(e.call(0x008cffa0, &args![0x5000u32]).u8(), 0x41);
        assert_eq!(calls_at(&e, EMPTY_00A29680), vec![vec![0x5000]; 2]);
    }

    #[test]
    fn fn_008d01e0_reads_the_float_at_0x98() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x9c);
        e.mem.set_f32(this + 0x98, -2.0);
        assert_eq!(e.call(0x008d01e0, &args![this]).f64(), -2.0);
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_f(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub_all(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    fn stub(e: &mut Engine, callee: u32, eax: u32) {
        e.register_double(callee, move |_, _| ret(eax));
    }

    fn object(e: &mut Engine, size: u32, base: u32) -> u32 {
        let this = e.mem.alloc(size);
        let table = e.mem.alloc(0x100);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
        }
        e.mem.set_u32(this, table);
        this
    }

    /// Two nodes: the root (no collision object) with one child that the
    /// cast hits at fraction 0.25.
    fn ray_setup(e: &mut Engine) -> (u32, u32) {
        let root = object(e, 0x10, 0x7100_0000);
        let child = object(e, 0x10, 0x7200_0000);
        e.register_double(0x0043_b610, move |_, a| {
            ret(if a[0] == child { 0x0c00 } else { 0 })
        });
        let body = object(e, 0x10, 0x7300_0000);
        stub(e, 0x006f_a820, body);
        stub(e, 0x004a_e6a0, 0x5a00);
        e.register(BODY_GET_FILTER, |_, a| ret(a[1]));
        stub(e, FILTER_LOW_BITS, 8);
        stub(e, FILTER_PART, 2);
        stub(e, 0x7100_000c, 0x1c00);
        stub(e, 0x7200_000c, 0);
        stub(e, 0x0043_b480, 1);
        stub(e, 0x0043_b4a0, child);
        e.register(0x0045_bb20, |_, a| ret(a[1]));
        e.register(0x0043_9e90, |_, a| ret(a[1]));
        stub_all(
            e,
            &[
                CONSTRUCT_IN_PLACE,
                VECTOR_FROM_POINT,
                0x0063_2ca0,
                0x004a_3d20,
                0x0056_ded0,
                0x7300_00f4,
                0x00c7_f690,
                0x0063_2c30,
            ],
        );
        stub(e, 0x0043_b300, 0);
        stub(e, BODY_OBJECT, 0x5a01);
        e.register(0x0063_2c60, |e, a| {
            e.mem.set_f32(a[0] + 0x10, 0.25);
            ret(a[0])
        });
        stub(e, 0x0056_09f0, 1);
        (root, child)
    }

    fn ray(e: &mut Engine, root: u32, distance: u32, skip_1: u32) -> u32 {
        let words = [
            root,
            1.0f32.to_bits(),
            2.0f32.to_bits(),
            3.0f32.to_bits(),
            0,
            1.0f32.to_bits(),
            0,
            distance,
            skip_1,
        ];
        e.call(0x008c_fbc0, &words).u32()
    }

    #[test]
    fn fn_008cfbc0_finds_the_child_the_cast_hits() {
        let mut e = Engine::new();
        let (root, child) = ray_setup(&mut e);
        let distance = e.mem.alloc(4);
        e.mem.set_f32(distance, 512.0);
        e.call_log = Some(vec![]);
        assert_eq!(ray(&mut e, root, distance, 0), child);
        assert_eq!(e.mem.f32(distance), 128.0);
        let scale = calls_to(&e, 0x0045_bb20)[0].clone();
        assert_eq!(
            e.mem.bytes(scale[0], 12),
            [0, 1.0f32.to_bits(), 0]
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect::<Vec<u8>>()
        );
        assert_eq!(scale[2], 512.0f32.to_bits());
        let cast = calls_to(&e, 0x0063_2c30)[0].clone();
        assert_eq!(cast[0], 0x5a01);
        let transformed = calls_to(&e, 0x00c7_f690);
        assert_eq!(transformed[0][0], cast[2]);
        assert_eq!(transformed[1][0], cast[2] + 0x10);
        assert_eq!(calls_to(&e, 0x7300_00f4).len(), 1);
    }

    #[test]
    fn fn_008cfbc0_skips_bodies_of_other_layers_and_null() {
        let mut e = Engine::new();
        let (root, _) = ray_setup(&mut e);
        let distance = e.mem.alloc(4);
        e.mem.set_f32(distance, 512.0);
        stub(&mut e, FILTER_PART, 1);
        assert_eq!(ray(&mut e, root, distance, 1), 0);
        assert_eq!(e.mem.f32(distance), 512.0);
        stub(&mut e, FILTER_LOW_BITS, 7);
        assert_eq!(ray(&mut e, root, distance, 0), 0);
        e.call_log = Some(vec![]);
        assert_eq!(ray(&mut e, 0, distance, 0), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_008cfbc0_uses_the_node_rotation_for_kind_objects() {
        let mut e = Engine::new();
        let (root, _) = ray_setup(&mut e);
        let distance = e.mem.alloc(4);
        e.mem.set_f32(distance, 100.0);
        e.register(0x0043_b300, |_, _| ret(1));
        // `008cffa0`'s callees: an object and the byte its fill answers.
        let fill = e.mem.alloc(0x10);
        stub(&mut e, 0x0062_0b80, fill);
        stub_all(
            &mut e,
            &[
                0x00a2_9680,
                0x0062_2570,
                0x0056_df80,
                0x0063_2b40,
                0x008c_d6a0,
                0x00cb_2240,
            ],
        );
        e.register(0x004a_7bd0, |_, _| ret_f(1.0));
        stub(&mut e, NODE_ROTATION_FOR_TEST, 0x7070);
        stub(&mut e, 0x0043_6aa0, 0x0e00);
        stub(&mut e, 0x0089_1170, 0x0e01);
        e.call_log = Some(vec![]);
        ray(&mut e, root, distance, 0);
        assert!(calls_to(&e, 0x7300_00f4).is_empty());
        let transform = calls_to(&e, 0x0056_df80)[0][0];
        assert_eq!(calls_to(&e, 0x0056_df80)[0][1], 0x7070);
        let built = calls_to(&e, 0x008c_d6a0)[0].clone();
        assert_eq!(built[1..], [0x0e01, 0x0e00]);
        assert_eq!(calls_to(&e, 0x00cb_2240)[0][0], transform);
        assert_eq!(calls_to(&e, 0x00cb_2240)[0][2], built[0]);
    }

    const NODE_ROTATION_FOR_TEST: u32 = 0x0046_1130;

    /// The settings the push reads, each at its own address with `value`.
    fn settings(e: &mut Engine, values: &[(u32, f32)]) {
        let table: Vec<(u32, u32)> = values
            .iter()
            .map(|&(setting, value)| {
                let at = e.mem.alloc(4);
                e.mem.set_f32(at, value);
                (setting, at)
            })
            .collect();
        e.register_double(SETTING_VALUE, move |_, a| {
            ret(table
                .iter()
                .find(|(s, _)| *s == a[0])
                .map(|(_, at)| *at)
                .unwrap())
        });
    }

    #[test]
    fn fn_008d0020_pushes_once_with_the_scaled_speed() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc8);
        e.mem.set_f32(this + 0x98, 10.0);
        let body = object(&mut e, 0x10, 0x7300_0000);
        stub(&mut e, 0x7300_0098, 0x1100);
        stub(&mut e, BODY_GET_FILTER, 0);
        stub(&mut e, FILTER_LOW_BITS, 10);
        settings(
            &mut e,
            &[(0x011c_f9d4, 2.0), (0x011c_f114, 4.0), (0x011c_e224, 3.0)],
        );
        e.register(0x004b_5400, |_, _| ret_f(1.0));
        stub_all(
            &mut e,
            &[
                CONSTRUCT_IN_PLACE,
                VECTOR_FROM_POINT,
                0x0062_aea0,
                0x005d_bf20,
                0x0062_7920,
                0x0048_3710,
                0x0081_f1e0,
            ],
        );
        stub(&mut e, BODY_OBJECT, 0x5a01);
        let words = [this, 1, 2, 3, 4, 5, 6, body];
        e.call_log = Some(vec![]);
        e.call(0x008d_0020, &words);
        // 10 * 2 = 20, mass 1 below 4: * 1/4 = 5, * 3 = 15.
        let impulse = calls_to(&e, 0x005d_bf20)[0].clone();
        assert_eq!(impulse[1], 15.0f32.to_bits());
        let fills = calls_to(&e, VECTOR_FROM_POINT);
        assert_eq!(
            e.mem.bytes(fills[0][1], 12),
            [4u32, 5, 6]
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect::<Vec<u8>>()
        );
        assert_eq!(
            e.mem.bytes(fills[1][1], 12),
            [1u32, 2, 3]
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect::<Vec<u8>>()
        );
        assert_eq!(
            calls_to(&e, 0x0081_f1e0),
            vec![vec![0x5a01, fills[0][0], fills[1][0]]]
        );
        assert_eq!(calls_to(&e, 0x0048_3710), vec![vec![0x1100], vec![0x1100]]);
        assert_eq!(e.mem.u8(this + POST_FORCE), 1);
        // Pushed already: nothing more.
        e.call_log = Some(vec![]);
        e.call(0x008d_0020, &words);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_008d0020_without_the_object_only_marks_it() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc8);
        let body = object(&mut e, 0x10, 0x7300_0000);
        stub(&mut e, 0x7300_0098, 0);
        e.call(0x008d_0020, &[this, 0, 0, 0, 0, 0, 0, body]);
        assert_eq!(e.mem.u8(this + POST_FORCE), 1);
        e.mem.set_u8(this + POST_FORCE, 0);
        e.call_log = Some(vec![]);
        e.call(0x008d_0020, &[this, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(e.mem.u8(this + POST_FORCE), 0);
    }

    #[test]
    fn post_force_getter_setter_and_collidee_clear() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc8);
        e.call(0x008d_0220, &[this, 0x1ff]);
        assert_eq!(e.call(0x008d_0200, &[this]).u8(), 0xff);
        e.call(0x008d_0240, &[this]);
        let record = e.mem.alloc(0x54);
        e.mem.set_u32(record + COLLISION_COLLIDEE, 0x7a00);
        e.mem.set_u32(this + COLLISION, record);
        e.call(0x008d_0240, &[this]);
        assert_eq!(e.mem.u32(record + COLLISION_COLLIDEE), 0);
    }

    #[test]
    fn fn_008d02a0_copies_the_zero_vector() {
        let mut e = Engine::new();
        e.map(ZERO_POINT, 12);
        e.mem.set_f32(ZERO_POINT + 4, 2.5);
        let result = e.mem.alloc(12);
        e.mem.set_u32(result, 9);
        assert_eq!(e.call(0x008d_02a0, &[0, result, 7]).u32(), result);
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(e.mem.f32(result + 4), 2.5);
    }
}
