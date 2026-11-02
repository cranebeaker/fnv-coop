//! `fallout/ai/run_00978ed0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00978ef0` calls on the member (`00559450`).
const CALLEE_00559450: u32 = 0x0055_9450;

// Translated from 00978ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00559450` with `ECX` = `this + 0x120` and passes on its result.
pub fn fn_00978ef0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00559450, &args![this.byte_add(0x120)]).u32()
}

// Translated from 00978ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `pointer` to the `NiPointer` member at `this + 0x120` through
/// `NiPointer<ActorCause>::operator=` (`0042f780`, engine map name; the body
/// is a smart-pointer assignment). One stack word. The answer is the
/// assignment's (the exe ends on that call and keeps its `EAX`; the virtual
/// slot's `ActorCause*`).
pub fn fn_00978ed0(e: &mut Engine, this: Ptr, pointer: u32) -> u32 {
    e.call(0x0042_f780, &args![this.byte_add(0x120), pointer])
        .u32()
}

// Translated from 00978f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual that returns the constant 1 (`ECX` only, no stack words).
pub fn fn_00978f30(_e: &mut Engine, _this: Ptr) -> u32 {
    1
}

// Translated from 00979260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at `this + 0xD4`, loaded into `ST0` (`FLD dword`).
pub fn fn_00979260(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0xd4))
}

// ---- n028: `BeamProjectile` (vtables `0108c3c4` and `0108c3b8`, RTTI read
// at the locators) ----

/// The `double` 1.0 the beam length is compared with (`.rdata`).
const DOUBLE_ONE: u32 = 0x0101_2070;
/// `BeamProjectile`'s two vtables (at +0 and +0x18).
const BEAM_VTABLE: u32 = 0x0108_c3c4;
const BEAM_VTABLE_2: u32 = 0x0108_c3b8;
/// The `NiPointer` member at +0x150: constructed by `00633c90`
/// (`thiscall(value)`), set by `0066b0d0` (`thiscall(value)`), read by
/// `00559450` and released by `0045cec0`.
const BEAM_POINTER: u32 = 0x150;
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_SET: u32 = 0x0066_b0d0;
const POINTER_RELEASE: u32 = 0x0045_cec0;

// Translated from 00978f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `this` to `0077eac0` and passes its `EAX` back (a plain `RET`
/// right after the call).
pub fn fn_00978f10(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0077_eac0, &args![this]).u32()
}

// Translated from 00978f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BeamProjectile` scalar deleting destructor: `fn_00978f70`, then
/// `operator delete(this)` (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00978f40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00978f70(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00978f70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00978f70 - the compiler's exception frame (FS:[0]) is not translated
/// The `BeamProjectile` destructor body: the pointer at +0x150 released
/// (`0045cec0`), then the `Projectile` destructor body (`009bc470`).
pub fn fn_00978f70(e: &mut Engine, this: Ptr) {
    e.call(POINTER_RELEASE, &args![this.addr() + BEAM_POINTER]);
    e.call(0x009b_c470, &args![this]);
}

/// The common end of the two `BeamProjectile` constructors: the vtables,
/// the pointer at +0x150 constructed empty, `004f15a0(0x3f)` and the
/// pointer set to 0.
fn finish_beam(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, BEAM_VTABLE);
    e.mem.set_u32(at + 0x18, BEAM_VTABLE_2);
    e.call(POINTER_CONSTRUCT, &args![at + BEAM_POINTER, 0u32]);
    e.call(0x004f_15a0, &args![this, 0x3fu32]);
    e.call(POINTER_SET, &args![at + BEAM_POINTER, 0u32]);
    this
}

// Translated from 00978fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00978fd0 - the compiler's exception frame (FS:[0]) that destroys the half-built object when a step throws is not translated
/// A `BeamProjectile` constructor: `Projectile::Projectile` (`009bbc70`
/// with the low byte of `flag`), then `finish_beam`. Returns `this`.
pub fn fn_00978fd0(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    e.call(0x009b_bc70, &args![this, flag & 0xff]);
    finish_beam(e, this)
}

// Translated from 00979070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00979070 - the compiler's exception frame (FS:[0]) that destroys the half-built object when a step throws is not translated
/// The other `BeamProjectile` constructor (eight words, `RET 0x20`): the
/// `Projectile` constructor `009bbef0` with the same words (the two floats
/// reloaded, `FLD`/`FSTP`), then `finish_beam`. Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00979070(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    point_x: u32,
    point_y: u32,
    point_z: u32,
    value_1: f32,
    value_2: f32,
) -> Ptr {
    let value_1 = x87_quiet_bits(value_1.to_bits());
    let value_2 = x87_quiet_bits(value_2.to_bits());
    e.call(
        0x009b_bef0,
        &args![this, first, second, third, point_x, point_y, point_z, value_1, value_2],
    );
    finish_beam(e, this)
}

// Translated from 00979140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lines the beam's 3D up with its source: with 3D (slot 0x1D0) and a
/// source node (slot 0x20C of `00874480`'s object), the 3D is updated
/// (slot 0xB8 with a `0043d410(0.0, 0, 0)` record) and the matrix at +0x94
/// becomes the 3D's rotation times the inverse of the source's
/// (`NiMatrix3::Inverse`, `NiMatrix3::operatorP`). Then `00979a70` runs on
/// the 3D with the float at +0xD4 (at least 1.0) and 0.0.
pub fn fn_00979140(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let node = e.vcall(at, 0x1d0, &[]).u32();
    if node == 0 {
        return;
    }
    let source = if e.call(0x0087_4480, &args![this]).u32() == 0 {
        0
    } else {
        let owner = e.call(0x0087_4480, &args![this]).u32();
        e.vcall(owner, 0x20c, &[]).u32()
    };
    if source != 0 {
        e.with_stack(0x58, |e, frame| {
            // The update record (EBP-0x18), the inverse (-0x3C) and the
            // product (-0x60).
            let update = frame.addr();
            let inverse = frame.addr() + 0x10;
            let product = frame.addr() + 0x34;
            e.call(0x0043_d410, &args![update, 0.0f32, 0u32, 0u32]);
            e.vcall(node, 0xb8, &args![update]);
            let rotation = e.call(0x0046_1130, &args![source]).u32();
            let inverted = e.call(0x004b_45b0, &args![rotation, inverse]).u32();
            let own = e.vcall(at, 0x1d0, &[]).u32();
            let own = e.call(0x0046_1130, &args![own]).u32();
            let result = e.call(0x0043_f8d0, &args![own, product, inverted]).u32();
            for i in 0..9 {
                let word = e.mem.u32(result + 4 * i);
                e.mem.set_u32(at + 0x94 + 4 * i, word);
            }
        });
    }
    let mut length = fn_00979260(e, this).to_bits();
    // `FCOMP 1.0; TEST AH,5; JP`: below 1.0 (ordered) becomes 1.0.
    if x87_load32(length) < e.mem.f64(DOUBLE_ONE) {
        length = 1.0f32.to_bits();
    }
    e.call(0x0097_9a70, &args![this, node, length, 0.0f32]);
}

// Translated from 00979280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the beam's flags (`005de0a0`: 0x10 off, 1 on, 2 off, 8, 0x20,
/// 0x100, 0x80 and 0x2000 on), runs `009bda10`, and hands the float at
/// +0xD4 to `005bfbd0` on the object of the pointer at +0x150 when it has
/// one.
pub fn fn_00979280(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    for (flag, on) in [
        (0x10u32, 0u32),
        (1, 1),
        (2, 0),
        (8, 1),
        (0x20, 1),
        (0x100, 1),
        (0x80, 1),
        (0x2000, 1),
    ] {
        e.call(0x005d_e0a0, &args![this, flag, on]);
    }
    e.call(0x009b_da10, &args![this]);
    if e.call(CALLEE_00559450, &args![at + BEAM_POINTER]).u32() != 0 {
        let value = fn_00979260(e, this).to_bits();
        let object = e.call(CALLEE_00559450, &args![at + BEAM_POINTER]).u32();
        e.call(0x005b_fbd0, &args![object, value]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_8f10, fn_00978f10(Ptr) -> u32),
        entry!(0x0097_8f40, fn_00978f40(Ptr, u32) -> Ptr),
        entry!(0x0097_8f70, fn_00978f70(Ptr)),
        entry!(0x0097_8fd0, fn_00978fd0(Ptr, u32) -> Ptr),
        entry!(0x0097_9070, fn_00979070(Ptr, u32, u32, u32, u32, u32, u32, f32, f32) -> Ptr),
        entry!(0x0097_9140, fn_00979140(Ptr)),
        entry!(0x0097_9280, fn_00979280(Ptr)),
        entry!(0x0097_8ef0, fn_00978ef0(Ptr) -> u32),
        entry!(0x0097_8ed0, fn_00978ed0(Ptr, u32) -> u32),
        entry!(0x0097_8f30, fn_00978f30(Ptr) -> u32),
        entry!(0x0097_9260, fn_00979260(Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00978ef0_passes_this_plus_120_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00559450, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_8ef0, &args![0x4100u32]).u32(), 0x1775);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00559450).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x120]);
    }
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_00978ed0_assigns_the_pointer_at_0x120() {
        let mut e = Engine::new();
        e.register(0x0042_f780, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0097_8ed0, &args![0x4100u32, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == 0x0042_f780).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x4220, 0x7000]);
    }

    #[test]
    fn fn_00978ed0_answers_the_assignment() {
        let mut e = Engine::new();
        e.register(0x0042_f780, |_, a| a[0].into_ret());
        assert_eq!(
            e.call(0x0097_8ed0, &args![0x4100u32, 0x7000u32]).u32(),
            0x4220
        );
    }

    #[test]
    fn fn_00978f30_returns_one() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0097_8f30, &args![0x4100u32]).u32(), 1);
    }

    #[test]
    fn fn_00979260_loads_the_float_at_0xd4() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x100);
        e.mem.set_f32(object + 0xd4, 2.5);
        assert_eq!(e.call(0x0097_9260, &args![object]).f32(), 2.5);
        // A signalling NaN comes back quiet, as `FLD` leaves it.
        e.mem.set_u32(object + 0xd4, 0x7f80_0001);
        let back = e.call(0x0097_9260, &args![object]).f32().to_bits();
        assert_eq!(back, 0x7fc0_0001);
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

    fn object(e: &mut Engine, size: u32, base: u32) -> u32 {
        let this = e.mem.alloc(size);
        let table = e.mem.alloc(0x400);
        for slot in (0..0x400).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
        }
        e.mem.set_u32(this, table);
        this
    }

    #[test]
    fn fn_00978f10_passes_the_callee_word_back() {
        let mut e = Engine::new();
        e.register(0x0077_eac0, |_, a| ret(a[0] + 1));
        assert_eq!(e.call(0x0097_8f10, &[0x9000]).u32(), 0x9001);
    }

    #[test]
    fn the_destructors_release_the_pointer_then_the_projectile() {
        let mut e = Engine::new();
        stub_all(&mut e, &[POINTER_RELEASE, 0x009b_c470, 0x0040_1030]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_8f40, &[0x9000, 1]).u32(), 0x9000);
        assert_eq!(calls_to(&e, POINTER_RELEASE), vec![vec![0x9150]]);
        assert_eq!(calls_to(&e, 0x009b_c470), vec![vec![0x9000]]);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![0x9000]]);
        e.call(0x0097_8f40, &[0x9000, 0]);
        assert_eq!(calls_to(&e, 0x0040_1030).len(), 1);
    }

    #[test]
    fn the_constructors_build_the_beam() {
        let mut e = Engine::new();
        stub_all(
            &mut e,
            &[
                0x009b_bc70,
                0x009b_bef0,
                POINTER_CONSTRUCT,
                0x004f_15a0,
                POINTER_SET,
            ],
        );
        let this = e.mem.alloc(0x160);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_8fd0, &[this, 0x0101]).u32(), this);
        assert_eq!(calls_to(&e, 0x009b_bc70), vec![vec![this, 1]]);
        assert_eq!(e.mem.u32(this), BEAM_VTABLE);
        assert_eq!(e.mem.u32(this + 0x18), BEAM_VTABLE_2);
        assert_eq!(calls_to(&e, POINTER_CONSTRUCT), vec![vec![this + 0x150, 0]]);
        assert_eq!(calls_to(&e, 0x004f_15a0), vec![vec![this, 0x3f]]);
        assert_eq!(calls_to(&e, POINTER_SET), vec![vec![this + 0x150, 0]]);
        let words = [this, 1, 2, 3, 4, 5, 6, 1.5f32.to_bits(), 0x7f80_0001];
        e.call(0x0097_9070, &words);
        assert_eq!(
            calls_to(&e, 0x009b_bef0),
            vec![vec![this, 1, 2, 3, 4, 5, 6, 1.5f32.to_bits(), 0x7fc0_0001]]
        );
    }

    #[test]
    fn fn_00979140_aligns_with_the_source_and_stretches() {
        let mut e = Engine::new();
        e.map(DOUBLE_ONE, 8);
        e.mem.set_f64(DOUBLE_ONE, 1.0);
        let this = object(&mut e, 0xd8, 0x7100_0000);
        e.mem.set_f32(this + 0xd4, 0.5);
        let node = object(&mut e, 0x10, 0x7200_0000);
        e.register_double(0x7100_01d0, move |_, _| ret(node));
        let owner = object(&mut e, 0x10, 0x7300_0000);
        e.register_double(0x0087_4480, move |_, _| ret(owner));
        e.register(0x7300_020c, |_, _| ret(0x5e00));
        stub_all(&mut e, &[0x0043_d410, 0x7200_00b8, 0x0097_9a70]);
        e.register(0x0046_1130, |_, a| ret(a[0] + 0x10));
        e.register(0x004b_45b0, |_, a| ret(a[1]));
        let product = e.mem.alloc(36);
        e.mem.set_f32(product + 32, 3.0);
        e.register_double(0x0043_f8d0, move |_, _| ret(product));
        e.call_log = Some(vec![]);
        e.call(0x0097_9140, &[this]);
        let update = calls_to(&e, 0x0043_d410)[0][0];
        assert_eq!(calls_to(&e, 0x7200_00b8), vec![vec![node, update]]);
        let inverse = calls_to(&e, 0x004b_45b0)[0].clone();
        assert_eq!(inverse[0], 0x5e10);
        assert_eq!(calls_to(&e, 0x0043_f8d0)[0][0], node + 0x10);
        assert_eq!(calls_to(&e, 0x0043_f8d0)[0][2], inverse[1]);
        assert_eq!(e.mem.f32(this + 0x94 + 32), 3.0);
        // 0.5 is raised to 1.0.
        assert_eq!(
            calls_to(&e, 0x0097_9a70),
            vec![vec![this, node, 1.0f32.to_bits(), 0]]
        );
    }

    #[test]
    fn fn_00979280_sets_the_flags_and_hands_on_the_length() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x160);
        e.mem.set_f32(this + 0xd4, 2.5);
        stub_all(&mut e, &[0x005d_e0a0, 0x009b_da10, 0x005b_fbd0]);
        e.register(CALLEE_00559450, |_, _| ret(0x0b0b));
        e.call_log = Some(vec![]);
        e.call(0x0097_9280, &[this]);
        let flags = calls_to(&e, 0x005d_e0a0);
        assert_eq!(flags.len(), 8);
        assert_eq!(flags[0], vec![this, 0x10, 0]);
        assert_eq!(flags[7], vec![this, 0x2000, 1]);
        assert_eq!(
            calls_to(&e, 0x005b_fbd0),
            vec![vec![0x0b0b, 2.5f32.to_bits()]]
        );
        e.register(CALLEE_00559450, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0097_9280, &[this]);
        assert!(calls_to(&e, 0x005b_fbd0).is_empty());
    }
}
