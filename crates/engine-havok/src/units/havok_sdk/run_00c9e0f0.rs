//! `Havok SDK/run_00c9e0f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00c9e1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00c9_db80`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c9e1b0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00c9_db80, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The SSE sign-clearing mask (four words, `0x7fffffff`) the `ANDPS` below loads.
const ABS_MASK: u32 = 0x0101_7d10;
/// The SSE sign-flipping mask (four words, `0x80000000`) the `XORPS` below loads.
const SIGN_MASK: u32 = 0x010c_44d0;

type Lanes = [f32; 4];

fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

/// `ANDPS` / `XORPS` of `value` with the four mask words at `mask_addr`.
fn mask_lanes(e: &Engine, mask_addr: u32, value: Lanes, xor: bool) -> Lanes {
    std::array::from_fn(|lane| {
        let mask = e.mem.u32(mask_addr + 4 * lane as u32);
        let bits = value[lane].to_bits();
        f32::from_bits(if xor { bits ^ mask } else { bits & mask })
    })
}

/// `MULPS a, b` (the SSE NaN rules: `a` is the destination).
fn mul_lanes(a: Lanes, b: Lanes) -> Lanes {
    crate::sse::mulps(a, b)
}

/// `ADDPS a, b` (the SSE NaN rules).
fn add_lanes(a: Lanes, b: Lanes) -> Lanes {
    crate::sse::addps(a, b)
}

// Translated from 00c9e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `min(a, b)` on two `float` arguments (cdecl, result in `ST0`, rounded through a `float`
/// store): `a` when `b > a`, else `b` (so `b` when equal or either is a NaN). The map gives
/// it no name.
pub fn fn_00c9e0f0(_e: &mut Engine, a: f32, b: f32) -> f32 {
    if b > a {
        x87_quiet(a)
    } else {
        x87_quiet(b)
    }
}

// Translated from 00c9e120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The axis-aligned box around a transformed box (cdecl, no map name; its callers are
/// `getAabbImpl` and `queryObb`): with the transform's columns `c0..c3` (`c3` the
/// translation), `extent = |y * c1| + |x * c0| + (|z * c2| + margin)` per lane (the
/// absolute value is the `ANDPS` with the mask at `01017d10`), then `out[0..4] = c3 -
/// extent` (the sign flip is the `XORPS` with `010c44d0`) and `out[4..8] = c3 + extent`.
pub fn fn_00c9e120(e: &mut Engine, transform: Ptr, half_extents: Ptr, margin: f32, out: Ptr) {
    let base = transform.addr();
    let v = load_lanes(e, half_extents.addr());
    let (c0, c1, c2) = (
        load_lanes(e, base),
        load_lanes(e, base + 0x10),
        load_lanes(e, base + 0x20),
    );
    let by_y = mask_lanes(e, ABS_MASK, mul_lanes([v[1]; 4], c1), false);
    let by_x = mask_lanes(e, ABS_MASK, mul_lanes([v[0]; 4], c0), false);
    let first = add_lanes(by_y, by_x);
    let by_z = mask_lanes(e, ABS_MASK, mul_lanes([v[2]; 4], c2), false);
    let second = add_lanes(by_z, [margin; 4]);
    let extent = add_lanes(first, second);
    let translation = load_lanes(e, base + 0x30);
    let low = add_lanes(mask_lanes(e, SIGN_MASK, extent, true), translation);
    let high = add_lanes(translation, extent);
    for lane in 0..4 {
        e.mem
            .set_f32(out.addr() + 4 * lane as u32 + 0x10, high[lane]);
        e.mem.set_f32(out.addr() + 4 * lane as u32, low[lane]);
    }
}

// Translated from 00c9e1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (vtable slot of `010c70cc`, no map name) that ignores its two arguments
/// and returns `0x30`.
pub fn fn_00c9e1a0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0x30
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_e0f0, fn_00c9e0f0(f32, f32) -> f32),
        entry!(0x00c9_e120, fn_00c9e120(Ptr, Ptr, f32, Ptr)),
        entry!(0x00c9_e1a0, fn_00c9e1a0(Ptr, u32, u32) -> u32),
        entry!(0x00c9_e1b0, deleting_destructor_00c9e1b0(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00c9e1b0_frees_only_when_bit_zero_is_set() {
        check(0x00c9_e1b0, 0x00c9_db80);
    }

    #[test]
    fn min_returns_a_only_when_b_is_greater() {
        let mut e = Engine::new();
        let call = |e: &mut Engine, a: f32, b: f32| e.call(0x00c9_e0f0, &args![a, b]).f32();
        assert_eq!(call(&mut e, 1.0, 2.0), 1.0);
        assert_eq!(call(&mut e, 3.0, 2.0), 2.0);
        // equal values: the second argument, so -0.0 is kept from the second
        assert_eq!(call(&mut e, 0.0, -0.0).to_bits(), (-0.0f32).to_bits());
        // unordered: the second argument
        assert_eq!(call(&mut e, f32::NAN, 5.0), 5.0);
        assert!(call(&mut e, 5.0, f32::NAN).is_nan());
    }

    #[test]
    fn aabb_of_a_transformed_box_spreads_around_the_translation() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let transform = e.mem.alloc(0x40);
        let columns: [Lanes; 4] = [
            [1.0, -2.0, 0.5, 0.0],
            [0.0, 3.0, -1.0, 0.0],
            [-1.0, 0.0, 2.0, 0.0],
            [10.0, 20.0, 30.0, 1.0],
        ];
        for (i, c) in columns.iter().enumerate() {
            for (lane, value) in c.iter().enumerate() {
                e.mem
                    .set_f32(transform + 16 * i as u32 + 4 * lane as u32, *value);
            }
        }
        let half = e.mem.alloc(0x10);
        for (lane, value) in [2.0f32, 1.0, 4.0, 0.0].iter().enumerate() {
            e.mem.set_f32(half + 4 * lane as u32, *value);
        }
        let out = e.mem.alloc(0x20);
        e.call(0x00c9_e120, &args![transform, half, 0.25f32, out]);
        // |y c1| + |x c0| + (|z c2| + margin), lane by lane
        let extent = [
            (1.0 * 0.0f32).abs() + (2.0f32 * 1.0).abs() + ((-4.0f32).abs() + 0.25),
            (1.0 * 3.0f32).abs() + (2.0f32 * -2.0).abs() + ((4.0f32 * 0.0).abs() + 0.25),
            (-1.0f32).abs() + (2.0f32 * 0.5).abs() + ((4.0f32 * 2.0).abs() + 0.25),
            0.25,
        ];
        let translation = [10.0f32, 20.0, 30.0, 1.0];
        for lane in 0..4 {
            let low = e.mem.f32(out + 4 * lane as u32);
            let high = e.mem.f32(out + 0x10 + 4 * lane as u32);
            assert_eq!(low, -extent[lane] + translation[lane]);
            assert_eq!(high, translation[lane] + extent[lane]);
        }
    }

    #[test]
    fn constant_method_returns_0x30() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c9_e1a0, &args![0u32, 1u32, 2u32]).u32(), 0x30);
    }
}
