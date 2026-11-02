//! `Havok SDK/hkpvehicledefaultaerodynamics.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no vehicle types: the layout below and its field names are taken from what the
//! code reads (the names follow the Havok SDK's `hkpVehicleDefaultAerodynamics`, unconfirmed).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVehicleDefaultAerodynamics` (names unconfirmed; vtable `010e6394`: slot `0xC` is
    /// `calcAerodynamics`, slot `0x10` `calcAerodynamicDrag`, slot `0x14`
    /// `calcAerodynamicLift`).
    pub struct HkpVehicleDefaultAerodynamics: 0x30 {
        /// `m_airDensity`.
        0x08 m_air_density: f32,
        /// `m_frontalArea`.
        0x0c m_frontal_area: f32,
        /// `m_dragCoefficient`.
        0x10 m_drag_coefficient: f32,
        /// `m_liftCoefficient`.
        0x14 m_lift_coefficient: f32,
    }
}

/// The vector `m_extraGravityws` (unconfirmed name) at `+0x20` of the object.
const EXTRA_GRAVITY: u32 = 0x20;
/// `hkpMotion::getMass` (Xbox PDB, `this` = the motion, the mass comes back in `ST0`).
const MOTION_GET_MASS: u32 = 0x00c8_6430;
/// The doubles `0.5` (`01011588`) and `-0.5` (`01011598`) the two functions multiply by.
const HALF: f64 = 0.5;
const MINUS_HALF: f64 = -0.5;

// Translated from 00e442e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultAerodynamics::calcAerodynamicLift` (Xbox PDB), `(this, x)`: with `p` =
/// `airDensity * 0.5 * liftCoefficient * frontalArea` (every step in double precision),
/// `p * x * x` rounded to `float`.
pub fn calc_aerodynamic_lift(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultAerodynamics>,
    x: f32,
) -> f32 {
    // `FLD airDensity; FMUL 0.5; FMUL dword liftCoefficient; FMUL dword frontalArea;
    // FLD x; FLD ST0; FMULP ST2; FMULP`, with the x87 NaN choice (#199).
    let density = f64::from(x87_quiet(
        e.get(this, HkpVehicleDefaultAerodynamics::m_air_density),
    ));
    let lift = x87_m32(
        e.get(this, HkpVehicleDefaultAerodynamics::m_lift_coefficient)
            .to_bits(),
    );
    let area = x87_m32(
        e.get(this, HkpVehicleDefaultAerodynamics::m_frontal_area)
            .to_bits(),
    );
    let product = x87_mul(x87_mul(x87_mul(density, HALF), lift), area);
    let x = f64::from(x87_quiet(x));
    let result = x87_mul(x87_mul(product, x), x) as f32;
    x87_quiet(result)
}

// Translated from 00e44340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultAerodynamics::calcAerodynamicDrag` (Xbox PDB), `(this, x)`: with `p` =
/// `airDensity * -0.5 * dragCoefficient * frontalArea` (double precision) and `|x|` stored as
/// a `float` first, `x * (|x| * p)` rounded to `float`.
pub fn calc_aerodynamic_drag(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultAerodynamics>,
    x: f32,
) -> f32 {
    // `FLD x; FLD ST0; FABS; FSTP/FLD |x|; FLD airDensity; FMUL -0.5; FMUL dword
    // dragCoefficient; FMUL dword frontalArea; FMULP; FMULP`, with the x87 NaN choice (#199).
    let x = x87_quiet(x);
    let magnitude = x.abs();
    let density = f64::from(x87_quiet(
        e.get(this, HkpVehicleDefaultAerodynamics::m_air_density),
    ));
    let drag = x87_m32(
        e.get(this, HkpVehicleDefaultAerodynamics::m_drag_coefficient)
            .to_bits(),
    );
    let area = x87_m32(
        e.get(this, HkpVehicleDefaultAerodynamics::m_frontal_area)
            .to_bits(),
    );
    let product = x87_mul(x87_mul(x87_mul(density, MINUS_HALF), drag), area);
    let result = x87_mul(f64::from(x), x87_mul(f64::from(magnitude), product)) as f32;
    x87_quiet(result)
}

type Vec4 = [f32; 4];

fn read_vec4(e: &Engine, address: u32) -> Vec4 {
    std::array::from_fn(|i| e.mem.f32(address + 4 * i as u32))
}

fn write_vec4(e: &mut Engine, address: u32, value: Vec4) {
    for (i, lane) in value.iter().enumerate() {
        e.mem.set_f32(address + 4 * i as u32, *lane);
    }
}

/// `MULPS`.
fn mul(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|i| a[i] * b[i])
}

/// `ADDPS`.
fn add(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|i| a[i] + b[i])
}

/// `SHUFPS` of a register with itself, one lane copied to all four.
fn splat(a: Vec4, lane: usize) -> Vec4 {
    [a[lane]; 4]
}

// Translated from 00e44370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultAerodynamics::calcAerodynamics` (Xbox PDB), `(this, _, instance,
/// output)`; the first stack word is not read. All vectors are four `float` lanes (`MOVAPS`
/// wants 16-byte aligned addresses). `body` = the word at `instance + 0x18` (an `hkpEntity`
/// whose motion is at `+0xE0`: the rotation columns `r0`, `r1`, `r2` at `body + 0xF0`, `0x100`,
/// `0x110` and the linear velocity at `body + 0x1B0`), `data` = the word at `instance + 0x1C`
/// (the vehicle data: vectors `u` at `+0x40` and `w` at `+0x30`; layout unconfirmed).
/// 1. `a = u.x * r0 + u.y * r1 + u.z * r2`, `s = (p.y + p.x) + p.z` with `p` = velocity * `a`;
/// 2. `drag` = slot `0x10` of `this` with `s` (as a `float` argument, the answer in `ST0`),
///    `lift` = slot `0x14` with `s`;
/// 3. `force = lift * (w.x * r0 + w.y * r1 + w.z * r2) + drag * a`;
/// 4. the mass of the motion (`hkpMotion::getMass`) times the vector at `this + 0x20` is
///    added: `output[0] = mass * gravity + force`, and `output[1]` is set to zero.
pub fn calc_aerodynamics(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultAerodynamics>,
    _unused_1: u32,
    instance: Ptr,
    output: Ptr,
) {
    let body = e.mem.u32(instance.addr() + 0x18);
    let data = e.mem.u32(instance.addr() + 0x1c);
    let u = read_vec4(e, data + 0x40);
    let r0 = read_vec4(e, body + 0xf0);
    let r1 = read_vec4(e, body + 0x100);
    let r2 = read_vec4(e, body + 0x110);
    let a = add(
        add(mul(splat(u, 0), r0), mul(splat(u, 1), r1)),
        mul(splat(u, 2), r2),
    );
    let w = read_vec4(e, data + 0x30);
    let w_z = mul(splat(w, 2), r2);
    let w_xy = add(mul(splat(w, 0), r0), mul(splat(w, 1), r1));
    let velocity = read_vec4(e, body + 0x1b0);
    let p = mul(velocity, a);
    let s = (p[1] + p[0]) + p[2];
    let drag_slot_argument = x87_quiet_bits(s.to_bits());
    let this_word = this.addr();
    let drag = e.vcall(this_word, 0x10, &args![drag_slot_argument]).f32();
    let drag = x87_quiet(drag);
    let lift = e.vcall(this_word, 0x14, &args![drag_slot_argument]).f32();
    let lift = x87_quiet(lift);
    let drag_force = mul([drag; 4], a);
    let force = add(mul([lift; 4], add(w_xy, w_z)), drag_force);
    write_vec4(e, output.addr(), force);
    let mass = e.call(MOTION_GET_MASS, &args![body + 0xe0]).f32();
    let mass = x87_quiet(mass);
    let gravity = mul([mass; 4], read_vec4(e, this_word + EXTRA_GRAVITY));
    let stored = read_vec4(e, output.addr());
    write_vec4(e, output.addr(), add(gravity, stored));
    write_vec4(e, output.addr() + 0x10, [0.0; 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e4_42e0,
            calc_aerodynamic_lift(Ptr<HkpVehicleDefaultAerodynamics>, f32) -> f32
        ),
        entry!(
            0x00e4_4340,
            calc_aerodynamic_drag(Ptr<HkpVehicleDefaultAerodynamics>, f32) -> f32
        ),
        entry!(
            0x00e4_4370,
            calc_aerodynamics(Ptr<HkpVehicleDefaultAerodynamics>, u32, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, new_log};

    /// An object with density 2, frontal area 3, drag 0.5, lift 4 and extra gravity (0, 0, -10, 0).
    fn aerodynamics(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x30);
        e.mem.set_f32(this + 8, 2.0);
        e.mem.set_f32(this + 0xc, 3.0);
        e.mem.set_f32(this + 0x10, 0.5);
        e.mem.set_f32(this + 0x14, 4.0);
        write_vec4(e, this + EXTRA_GRAVITY, [0.0, 0.0, -10.0, 0.0]);
        this
    }

    #[test]
    fn lift_is_the_scaled_square() {
        let mut e = engine();
        let this = aerodynamics(&mut e);
        // 2 * 0.5 * 4 * 3 = 12; 12 * 5 * 5 = 300 whatever the sign of x.
        assert_eq!(e.call(0x00e4_42e0, &args![this, 5.0f32]).f32(), 300.0);
        assert_eq!(e.call(0x00e4_42e0, &args![this, -5.0f32]).f32(), 300.0);
        assert!(e.call(0x00e4_42e0, &args![this, f32::NAN]).f32().is_nan());
    }

    #[test]
    fn drag_opposes_the_direction() {
        let mut e = engine();
        let this = aerodynamics(&mut e);
        // 2 * -0.5 * 0.5 * 3 = -1.5; x * (|x| * -1.5): 5 gives -37.5, -5 gives 37.5.
        assert_eq!(e.call(0x00e4_4340, &args![this, 5.0f32]).f32(), -37.5);
        assert_eq!(e.call(0x00e4_4340, &args![this, -5.0f32]).f32(), 37.5);
        assert_eq!(e.call(0x00e4_4340, &args![this, 0.0f32]).f32(), 0.0);
    }

    #[test]
    fn drag_keeps_the_x87_nan_when_x_and_its_magnitude_meet() {
        // #199: x = 0xffc00000 and |x| are two NaNs; FMULP keeps the positive one.
        let mut e = engine();
        let this = aerodynamics(&mut e);
        let result = e.call(0x00e4_4340, &args![this, 0xffc0_0000u32]).f32();
        assert_eq!(result.to_bits(), 0x7fc0_0000);
    }

    #[test]
    fn lift_keeps_the_x87_nan_when_two_nans_meet() {
        // Oracle vector (#199): density -qNaN, frontal area -0.0, lift +qNaN, x = 1.5;
        // FMUL keeps the positive NaN.
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 8, 0xffff_ffff);
        e.mem.set_u32(this + 0xc, 0x8000_0000);
        e.mem.set_u32(this + 0x10, 0);
        e.mem.set_u32(this + 0x14, 0x7fff_ffff);
        let result = e.call(0x00e4_42e0, &args![this, 1.5f32]).f32();
        assert_eq!(result.to_bits(), 0x7fff_ffff);
    }

    #[test]
    fn the_force_adds_lift_drag_and_gravity() {
        let mut e = engine();
        let log = new_log();
        let this = aerodynamics(&mut e);
        // A vtable whose slots 0x10 and 0x14 are doubles answering 0.5 and 0.25 in ST0.
        let vtable = e.mem.alloc(0x20);
        for (slot, value) in [(0x10u32, 0.5f64), (0x14, 0.25)] {
            e.mem.set_u32(vtable + slot, 0x0900_0000 + slot);
            let log = std::rc::Rc::clone(&log);
            e.register_double(0x0900_0000 + slot, move |_, a| {
                log.borrow_mut().push((0x0900_0000 + slot, a.to_vec()));
                Ret {
                    st0: value,
                    ..Ret::default()
                }
            });
        }
        e.mem.set_u32(this, vtable);
        // The mass comes from the motion at body + 0xE0.
        let body = e.mem.alloc(0x200);
        let data = e.mem.alloc(0x80);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 0x18, body);
        e.mem.set_u32(instance + 0x1c, data);
        write_vec4(&mut e, body + 0xf0, [1.0, 2.0, 3.0, 4.0]);
        write_vec4(&mut e, body + 0x100, [5.0, 6.0, 7.0, 8.0]);
        write_vec4(&mut e, body + 0x110, [9.0, 10.0, 11.0, 12.0]);
        write_vec4(&mut e, body + 0x1b0, [1.0, 1.0, 1.0, 1.0]);
        write_vec4(&mut e, data + 0x40, [1.0, 2.0, 3.0, 0.5]);
        write_vec4(&mut e, data + 0x30, [1.0, 0.0, 2.0, 0.0]);
        e.register_double(MOTION_GET_MASS, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        let output = e.mem.alloc(0x20);
        write_vec4(&mut e, output + 0x10, [7.0; 4]);
        e.call(0x00e4_4370, &args![this, 0x1234u32, instance, output]);
        // a = (38, 44, 50, 56); s = 44 + 38 + 50 = 132 goes to both slots.
        let bits = 132.0f32.to_bits();
        assert_eq!(calls(&log, 0x0900_0010), vec![vec![this, bits]]);
        assert_eq!(calls(&log, 0x0900_0014), vec![vec![this, bits]]);
        // 0.25 * (19, 22, 25, 28) + 0.5 * a + 2 * (0, 0, -10, 0).
        assert_eq!(read_vec4(&e, output), [23.75, 27.5, 31.25 - 20.0, 35.0]);
        assert_eq!(read_vec4(&e, output + 0x10), [0.0; 4]);
    }
}
