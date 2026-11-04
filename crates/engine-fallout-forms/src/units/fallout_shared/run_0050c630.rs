//! `fallout shared/run_0050c630` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0050c6c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050c6c0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form
/// type 0x1e (0xc8 bytes; `tesdatahandler.rs` lists it); the RTTI name of the
/// vtable it stores at `+0` (`01028ee4`) is `.?AVTESObjectLIGH@@`. It runs
/// the base constructor (`0050f6f0`) and the component constructors by
/// address (`00402d00` at `+0x30`, `0048a3d0` at `+0x3c`, `00402c20` at
/// `+0x5c`, `0047a130` at `+0x68`, `0048cc50` at `+0x78`, `0048eac0` at
/// `+0x84`, `0048e7a0` at `+0x8c`, `00475350` at `+0x94`), stores the nine
/// vtable pointers, runs `00416870` on `+0xbc` with three `float` zeros,
/// registers the form type 0x1e (`004f15a0`) and calls `0050c990`. Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_0050c6c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0047_a130, &args![base + 0x68]);
    e.call(0x0048_cc50, &args![base + 0x78]);
    e.call(0x0048_eac0, &args![base + 0x84]);
    e.call(0x0048_e7a0, &args![base + 0x8c]);
    e.call(0x0047_5350, &args![base + 0x94]);
    e.mem.set_u32(base, 0x0102_8ee4);
    e.mem.set_u32(base + 0x30, 0x0102_8ecc);
    e.mem.set_u32(base + 0x3c, 0x0102_8ea8);
    e.mem.set_u32(base + 0x5c, 0x0102_8e88);
    e.mem.set_u32(base + 0x68, 0x0102_8e74);
    e.mem.set_u32(base + 0x78, 0x0102_8e60);
    e.mem.set_u32(base + 0x84, 0x0102_8e4c);
    e.mem.set_u32(base + 0x8c, 0x0102_8e2c);
    e.mem.set_u32(base + 0x94, 0x0102_8e18);
    e.call(0x0041_6870, &args![base + 0xbc, 0.0f32, 0.0f32, 0.0f32]);
    e.call(0x004f_15a0, &args![this, 0x1e]);
    e.call(0x0050_c990, &args![this]);
    this
}

// Translated from 0050c840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0050c870` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0050c840(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_c870, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// Run after each of the three `float` setters (`004bc2c0`, `this` only).
const AFTER_FLOAT_SET: u32 = 0x004b_c2c0;

/// `FLD`/`FSTP` of the `float` argument into `this + offset` (a signalling
/// NaN comes out quiet), then `004bc2c0` on `this`.
fn set_float_then_update(e: &mut Engine, this: Ptr, offset: u32, value: f32) {
    e.mem
        .set_u32(this.addr() + offset, x87_quiet_bits(value.to_bits()));
    e.call(AFTER_FLOAT_SET, &args![this]);
}

// Translated from 0050c630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the race and sex menu calls it): stores `value` at
/// `+0xF0`, then runs `004bc2c0` on `this`; `RET 4`.
pub fn fn_0050c630(e: &mut Engine, this: Ptr, value: f32) {
    set_float_then_update(e, this, 0xf0, value);
}

// Translated from 0050c660 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: stores `value` at `+0xF4`, then runs `004bc2c0` on `this`;
/// `RET 4`.
pub fn fn_0050c660(e: &mut Engine, this: Ptr, value: f32) {
    set_float_then_update(e, this, 0xf4, value);
}

// Translated from 0050c690 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: stores `value` at `+0xF8`, then runs `004bc2c0` on `this`;
/// `RET 4`.
pub fn fn_0050c690(e: &mut Engine, this: Ptr, value: f32) {
    set_float_then_update(e, this, 0xf8, value);
}

/// The component destructors `0050c870` runs, in its order: the offset in
/// the object and the destructor.
const COMPONENT_DESTRUCTORS: [(u32, u32); 7] = [
    (0x94, 0x0047_5380),
    (0x8c, 0x0048_e7d0),
    (0x84, 0x0048_eaf0),
    (0x68, 0x0047_a1a0),
    (0x5c, 0x0040_2c40),
    (0x3c, 0x0048_a490),
    (0x30, 0x0059_1300),
];

// Translated from 0050c870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0050c870 - the exception-unwinding frame (FS:[0]) is not translated.
/// The destructor body `0050c840` runs (vtable `01028ee4`, RTTI
/// `.?AVTESObjectLIGH@@`): stores the nine vtables as the constructor
/// `0050c6c0` does, runs the components' destructors
/// (`COMPONENT_DESTRUCTORS`, last component first) and then `0050f740` on
/// the object.
pub fn fn_0050c870(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, vtable) in [
        (0x00, 0x0102_8ee4),
        (0x30, 0x0102_8ecc),
        (0x3c, 0x0102_8ea8),
        (0x5c, 0x0102_8e88),
        (0x68, 0x0102_8e74),
        (0x78, 0x0102_8e60),
        (0x84, 0x0102_8e4c),
        (0x8c, 0x0102_8e2c),
        (0x94, 0x0102_8e18),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    for (offset, destructor) in COMPONENT_DESTRUCTORS {
        e.call(destructor, &args![base + offset]);
    }
    e.call(0x0050_f740, &args![this]);
}

// Translated from 0050c990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start values of the constructor `0050c6c0`: the word at `+0xB8`
/// cleared (`0050c9e0` with 0), 0x18 zero bytes at `+0x9C` (`00403d30`, a
/// `memset` wrapper), 1.0 (`FLD1`) at `+0xB4` and the `float` at `010249d8`
/// at `+0xB0`.
pub fn fn_0050c990(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    fn_0050c9e0(e, this, 0);
    e.call(0x0040_3d30, &args![base + 0x9c, 0u32, 0x18u32]);
    e.mem.set_u32(base + 0xb4, 1.0f32.to_bits());
    let value = x87_quiet_bits(e.global::<u32>(0x0102_49d8));
    e.mem.set_u32(base + 0xb0, value);
}

// Translated from 0050c9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `+0xB8`; `RET 4`.
pub fn fn_0050c9e0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xb8, value);
}

// Translated from 0050c820 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: returns the address `this + 0xBC` (the block the
/// constructor `0050c6c0` hands to `00416870`).
pub fn fn_0050c820(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0xbc)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050c6c0, fn_0050c6c0(Ptr) -> Ptr),
        entry!(0x0050_c840, fn_0050c840(Ptr, u32) -> Ptr),
        entry!(0x0050_c630, fn_0050c630(Ptr, f32)),
        entry!(0x0050_c660, fn_0050c660(Ptr, f32)),
        entry!(0x0050_c690, fn_0050c690(Ptr, f32)),
        entry!(0x0050_c870, fn_0050c870(Ptr)),
        entry!(0x0050_c990, fn_0050c990(Ptr)),
        entry!(0x0050_c9e0, fn_0050c9e0(Ptr, u32)),
        entry!(0x0050_c820, fn_0050c820(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_0050c820_returns_the_block_at_0xbc() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0050_c820, &[0x1000]).u32(), 0x10bc);
        assert_eq!(e.call(0x0050_c820, &[0xffff_ff80]).u32(), 0x3c);
    }
}

#[cfg(test)]
mod b023_tests {
    use super::*;

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != skip)
            .collect()
    }

    #[test]
    fn float_setters_store_then_update() {
        let mut e = Engine::new();
        e.register(AFTER_FLOAT_SET, |_, _| Ret::default());
        let this = e.mem.alloc(0x100);
        for (addr, offset) in [
            (0x0050_c630, 0xf0),
            (0x0050_c660, 0xf4),
            (0x0050_c690, 0xf8),
        ] {
            e.call_log = Some(vec![]);
            e.call(addr, &[this, 2.5f32.to_bits()]);
            assert_eq!(calls(&mut e, addr), vec![(AFTER_FLOAT_SET, vec![this])]);
            assert_eq!(e.mem.f32(this + offset), 2.5);
            e.call(addr, &[this, 0xff80_0001]);
            assert_eq!(e.mem.u32(this + offset), 0xffc0_0001);
        }
    }

    #[test]
    fn destructor_body_and_start_values() {
        let mut e = Engine::new();
        for (_, d) in COMPONENT_DESTRUCTORS {
            e.register(d, |_, _| Ret::default());
        }
        e.register(0x0050_f740, |_, _| Ret::default());
        e.register(0x0040_3d30, |_, _| Ret::default());
        let this = e.mem.alloc(0xc8);
        e.call_log = Some(vec![]);
        e.call(0x0050_c870, &[this]);
        let mut expected: Vec<(u32, Vec<u32>)> = COMPONENT_DESTRUCTORS
            .iter()
            .map(|(o, d)| (*d, vec![this + o]))
            .collect();
        expected.push((0x0050_f740, vec![this]));
        assert_eq!(calls(&mut e, 0x0050_c870), expected);
        assert_eq!(e.mem.u32(this), 0x0102_8ee4);
        assert_eq!(e.mem.u32(this + 0x94), 0x0102_8e18);

        e.map(0x0102_4000, 0x1000);
        e.set_global(0x0102_49d8, 0.75f32);
        e.mem.set_u32(this + 0xb8, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x0050_c990, &[this]);
        assert_eq!(
            calls(&mut e, 0x0050_c990),
            vec![(0x0040_3d30, vec![this + 0x9c, 0, 0x18])]
        );
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        assert_eq!(e.mem.f32(this + 0xb4), 1.0);
        assert_eq!(e.mem.f32(this + 0xb0), 0.75);
        e.call(0x0050_c9e0, &[this, 0x42]);
        assert_eq!(e.mem.u32(this + 0xb8), 0x42);
    }
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0050c6c0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_c6c0,
            size: 0xc8,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0047_a130, &[A::T(0x68)]),
                (0x0048_cc50, &[A::T(0x78)]),
                (0x0048_eac0, &[A::T(0x84)]),
                (0x0048_e7a0, &[A::T(0x8c)]),
                (0x0047_5350, &[A::T(0x94)]),
                (0x0041_6870, &[A::T(0xbc), A::V(0), A::V(0), A::V(0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1e)]),
                (0x0050_c990, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_8ee4),
                (0x30, 4, 0x0102_8ecc),
                (0x3c, 4, 0x0102_8ea8),
                (0x5c, 4, 0x0102_8e88),
                (0x68, 4, 0x0102_8e74),
                (0x78, 4, 0x0102_8e60),
                (0x84, 4, 0x0102_8e4c),
                (0x8c, 4, 0x0102_8e2c),
                (0x94, 4, 0x0102_8e18),
            ],
        });
    }
    #[test]
    fn at_0050c840_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_c870, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_c840, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_c840)
                .collect();
            let mut expected = vec![(0x0050_c870, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
