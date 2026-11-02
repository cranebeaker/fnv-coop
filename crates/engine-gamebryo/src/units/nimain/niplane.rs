//! `NiMain/niplane.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiPlane;

// Translated from 00a69970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPlane::NiPlane_ov2` (engine map): the normal copied as three words from `normal`, the
/// constant through `FLD`/`FSTP` (a signalling NaN is stored quiet). Returns `this`.
pub fn ni_plane_ni_plane_ov2(
    e: &mut Engine,
    this: Ptr<NiPlane>,
    normal: Ptr,
    constant: f32,
) -> Ptr<NiPlane> {
    let base = this.addr();
    for i in 0..2u32 {
        let word = e.mem.u32(normal.addr() + 4 * i);
        e.mem.set_u32(base + 4 * i, word);
    }
    let z = e.mem.u32(normal.addr() + 8);
    e.mem
        .set_u32(base + 0xc, x87_quiet_bits(constant.to_bits()));
    e.mem.set_u32(base + 8, z);
    this
}

// Translated from 00a69990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPlane::NiPlane_ov3` (engine map): the normal copied as three words from `normal`, the
/// constant the dot product of the normal and `point`, as the x87 code sums it:
/// `(ny·py + nx·px) + nz·pz`, rounded to a float. Returns `this`.
pub fn ni_plane_ni_plane_ov3(
    e: &mut Engine,
    this: Ptr<NiPlane>,
    normal: Ptr,
    point: Ptr,
) -> Ptr<NiPlane> {
    let base = this.addr();
    let n = normal.addr();
    let p = point.addr();
    for i in 0..3u32 {
        let word = e.mem.u32(n + 4 * i);
        e.mem.set_u32(base + 4 * i, word);
    }
    let y = x87_mul(x87_load32(e.mem.u32(n + 4)), x87_m32(e.mem.u32(p + 4)));
    let x = x87_mul(x87_load32(e.mem.u32(n)), x87_m32(e.mem.u32(p)));
    let sum = x87_add(y, x);
    let z = x87_mul(x87_load32(e.mem.u32(n + 8)), x87_m32(e.mem.u32(p + 8)));
    let constant = x87_add(sum, z);
    e.mem.set_u32(base + 0xc, x87_store32(constant));
    this
}

/// `NiPoint3::UnitCross` (engine map, `0053d1a0`; `thiscall` on the first point, two words:
/// the result and the second point; `RET 8`; answers the result's address).
const POINT_UNIT_CROSS: u32 = 0x0053_d1a0;

/// `FLD dword [minuend]; FSUB dword [subtrahend]; FSTP dword`: one component of a
/// difference, as its bits.
fn component_difference(e: &Engine, minuend: u32, subtrahend: u32) -> u32 {
    x87_store32(x87_sub(
        x87_load32(e.mem.u32(minuend)),
        x87_m32(e.mem.u32(subtrahend)),
    ))
}

// Translated from 00a699d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPlane::NiPlane_ov4` (engine map; `thiscall`, three points, `RET 0xc`): the plane
/// through `p0`, `p1` and `p2`. The normal is `UnitCross(p1 - p0, p2 - p1)` (each
/// difference rounded to floats on the stack), copied as three words; the constant is the
/// normal's dot product with `p0` as the x87 code sums it, `(ny·p0y + p0x·nx) + nz·p0z`,
/// rounded to a float. Returns `this`.
pub fn ni_plane_ni_plane_ov4(
    e: &mut Engine,
    this: Ptr<NiPlane>,
    p0: Ptr,
    p1: Ptr,
    p2: Ptr,
) -> Ptr<NiPlane> {
    let base = this.addr();
    let (p0, p1, p2) = (p0.addr(), p1.addr(), p2.addr());
    // The frame: the second difference at +0, the first at +0xC, the result at +0x18.
    e.with_stack(0x24, |e, frame| {
        let second = frame.addr();
        let first = second + 0xc;
        let result = second + 0x18;
        for i in 0..3u32 {
            let value = component_difference(e, p1 + 4 * i, p0 + 4 * i);
            e.mem.set_u32(first + 4 * i, value);
        }
        for i in 0..3u32 {
            let value = component_difference(e, p2 + 4 * i, p1 + 4 * i);
            e.mem.set_u32(second + 4 * i, value);
        }
        let normal = e
            .call(POINT_UNIT_CROSS, &args![first, result, second])
            .u32();
        for i in 0..3u32 {
            let word = e.mem.u32(normal + 4 * i);
            e.mem.set_u32(base + 4 * i, word);
        }
    });
    let y = x87_mul(x87_load32(e.mem.u32(base + 4)), x87_m32(e.mem.u32(p0 + 4)));
    let x = x87_mul(x87_load32(e.mem.u32(p0)), x87_m32(e.mem.u32(base)));
    let sum = x87_add(y, x);
    let z = x87_mul(x87_load32(e.mem.u32(base + 8)), x87_m32(e.mem.u32(p0 + 8)));
    let constant = x87_add(sum, z);
    e.mem.set_u32(base + 0xc, x87_store32(constant));
    this
}

/// The three words `NiPlane::NiPlane` copies into the normal (`011f426c`, `011f4270`,
/// `011f4274`; a global point the engine map does not name).
const DEFAULT_NORMAL: u32 = 0x011f_426c;

// Translated from 00a69940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPlane::NiPlane` (engine map; `thiscall`, no stack word): the normal copied as three
/// words from the global point at `011f426c`, the constant `0.0` (`FLDZ`/`FSTP`, stored
/// between the second and the third word, as the exe orders the stores). Returns `this`.
pub fn ni_plane_ni_plane(e: &mut Engine, this: Ptr<NiPlane>) -> Ptr<NiPlane> {
    let base = this.addr();
    let x = e.mem.u32(DEFAULT_NORMAL);
    e.mem.set_u32(base, x);
    let y = e.mem.u32(DEFAULT_NORMAL + 4);
    e.mem.set_u32(base + 4, y);
    let z = e.mem.u32(DEFAULT_NORMAL + 8);
    e.mem.set_u32(base + 0xc, x87_store32(0.0));
    e.mem.set_u32(base + 8, z);
    this
}

/// The `cdecl` allocation (`00aa1070`: size), `NiSprintf` (`00a305b0`, cdecl: buffer, size,
/// format, values) and `"(%g,%g,%g), %g"` (`0109c380`).
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_SPRINTF: u32 = 0x00a3_05b0;
const VIEWER_FORMAT: u32 = 0x0109_c380;

// Translated from 00a69a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPlane::GetViewerString` (engine map; thiscall, one word): a new block of
/// `strlen(prefix) + 0x38` bytes (`00aa1070`) holding the normal and the constant as
/// `"(%g,%g,%g), %g"` (`NiSprintf`, each float widened to a `double`); the prefix itself is
/// not printed. Answers the block.
pub fn ni_plane_get_viewer_string(e: &mut Engine, this: Ptr<NiPlane>, prefix: u32) -> u32 {
    let mut at = prefix;
    while e.mem.u8(at) != 0 {
        at = at.wrapping_add(1);
    }
    let size = at.wrapping_sub(prefix).wrapping_add(0x38);
    let block = e.call(NI_ALLOC, &args![size]).u32();
    let word = |e: &Engine, offset: u32| x87_load32(e.mem.u32(this.addr().wrapping_add(offset)));
    let constant = word(e, 0xc);
    let z = word(e, 8);
    let y = word(e, 4);
    let x = word(e, 0);
    e.call(
        NI_SPRINTF,
        &args![block, size, VIEWER_FORMAT, x, y, z, constant],
    );
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a6_9a60,
            ni_plane_get_viewer_string(Ptr<NiPlane>, u32) -> u32
        ),
        entry!(0x00a6_9940, ni_plane_ni_plane(Ptr<NiPlane>) -> Ptr<NiPlane>),
        entry!(
            0x00a6_99d0,
            ni_plane_ni_plane_ov4(Ptr<NiPlane>, Ptr, Ptr, Ptr) -> Ptr<NiPlane>
        ),
        entry!(
            0x00a6_9970,
            ni_plane_ni_plane_ov2(Ptr<NiPlane>, Ptr, f32) -> Ptr<NiPlane>
        ),
        entry!(
            0x00a6_9990,
            ni_plane_ni_plane_ov3(Ptr<NiPlane>, Ptr, Ptr) -> Ptr<NiPlane>
        ),
    ]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn words(e: &Engine, p: u32) -> Vec<u32> {
        (0..4).map(|i| e.mem.u32(p + 4 * i)).collect()
    }

    fn point(e: &mut Engine, values: [f32; 3]) -> u32 {
        let p = e.mem.alloc(0xc);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(p + 4 * i as u32, v.to_bits());
        }
        p
    }

    #[test]
    fn plane_from_normal_and_constant() {
        let mut e = Engine::new();
        let n = e.mem.alloc(0xc);
        e.mem.set_u32(n, 0x7f80_0001);
        e.mem.set_u32(n + 4, 5);
        e.mem.set_u32(n + 8, 6);
        let this = e.mem.alloc(0x10);
        let r = e.call(0x00a6_9970, &[this, n, 0x7f80_0002]);
        assert_eq!(r.u32(), this);
        assert_eq!(words(&e, this), vec![0x7f80_0001, 5, 6, 0x7fc0_0002]);
        e.call(0x00a6_9970, &[this, n, 2.5f32.to_bits()]);
        assert_eq!(e.mem.u32(this + 0xc), 2.5f32.to_bits());
    }

    #[test]
    fn plane_from_normal_and_point() {
        let mut e = Engine::new();
        let n = point(&mut e, [0.0, 0.6, 0.8]);
        let p = point(&mut e, [10.0, 2.0, -3.0]);
        let this = e.mem.alloc(0x10);
        let r = e.call(0x00a6_9990, &args![this, n, p]);
        assert_eq!(r.u32(), this);
        // The x term is an exact 0.
        let want = (f64::from(0.6f32) * 2.0 + f64::from(0.8f32) * -3.0) as f32;
        assert_eq!(
            words(&e, this),
            vec![0, 0.6f32.to_bits(), 0.8f32.to_bits(), want.to_bits()]
        );
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    fn point(e: &mut Engine, values: [f32; 3]) -> u32 {
        let p = e.mem.alloc(0xc);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *v);
        }
        p
    }

    fn floats(e: &Engine, p: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(p + 4 * i)).collect()
    }

    #[test]
    fn plane_through_three_points() {
        let mut e = Engine::new();
        // The cross product double (unnormalised: the points are chosen so it is a unit).
        e.register(POINT_UNIT_CROSS, |e, a| {
            let u: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[2] + 4 * i)).collect();
            let cross = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            for (i, c) in cross.iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *c);
            }
            a[1].into_ret()
        });
        // Three points on the plane z = 5.
        let p0 = point(&mut e, [0.0, 0.0, 5.0]);
        let p1 = point(&mut e, [1.0, 0.0, 5.0]);
        let p2 = point(&mut e, [1.0, 1.0, 5.0]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let back = e.call(0x00a6_99d0, &args![this, p0, p1, p2]).u32();
        assert_eq!(back, this);
        assert_eq!(floats(&e, this, 4), vec![0.0, 0.0, 1.0, 5.0]);
        let log = e.call_log.take().unwrap();
        let cross = log.iter().find(|(a, _)| *a == POINT_UNIT_CROSS).unwrap();
        // The first point is p1 - p0, the second p2 - p1.
        assert_eq!(cross.1[0] + 0xc, cross.1[1]);
        assert_eq!(cross.1[2] + 0xc, cross.1[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_plane_takes_the_default_normal_and_a_zero_constant() {
        let mut e = Engine::new();
        e.map(DEFAULT_NORMAL & !0xfff, 0x1000);
        e.map(0x0000_1000, 0x100);
        for (i, bits) in [0x3f80_0000u32, 0xffc0_0001, 0x8000_0000]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(DEFAULT_NORMAL + 4 * i as u32, bits);
        }
        e.mem.set_u32(0x100c, 0xdead_beef);
        assert_eq!(e.call(0x00a6_9940, &args![0x1000u32]).u32(), 0x1000);
        assert_eq!(e.mem.u32(0x1000), 0x3f80_0000);
        assert_eq!(e.mem.u32(0x1004), 0xffc0_0001, "copied unchanged");
        assert_eq!(e.mem.u32(0x1008), 0x8000_0000);
        assert_eq!(e.mem.u32(0x100c), 0);
    }
}

#[cfg(test)]
mod lookahead2_tests {
    use super::*;

    #[test]
    fn default_plane_copies_the_global_normal_and_zeroes_the_constant() {
        let mut e = Engine::new();
        e.map(DEFAULT_NORMAL, 0xc);
        e.mem.set_u32(DEFAULT_NORMAL, 0x7f80_0001);
        e.mem.set_u32(DEFAULT_NORMAL + 4, 2);
        e.mem.set_u32(DEFAULT_NORMAL + 8, 3);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x1234);
        let r = e.call(0x00a6_9940, &[this]);
        assert_eq!(r.u32(), this);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0x7f80_0001, 2, 3, 0]);
    }
}

#[cfg(test)]
mod n083_tests {
    use super::*;

    #[test]
    fn viewer_string_prints_the_plane() {
        let mut e = Engine::new();
        e.register(NI_ALLOC, |_, _| 0x7000u32.into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let plane = e.new_object::<NiPlane>();
        e.set(plane, NiPlane::m_kNormal_x, 1.0);
        e.set(plane, NiPlane::m_kNormal_y, -2.5);
        e.set(plane, NiPlane::m_kNormal_z, 0.5);
        e.set(plane, NiPlane::m_fConstant, 3.0);
        let prefix = e.mem.alloc(8);
        e.mem.set_cstr(prefix, b"abc");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_9a60, &args![plane, prefix]).u32(), 0x7000);
        let log = e.call_log.take().unwrap();
        let alloc: Vec<_> = log.iter().filter(|c| c.0 == NI_ALLOC).collect();
        assert_eq!(alloc[0].1, [0x3b]);
        let print: Vec<_> = log.iter().filter(|c| c.0 == NI_SPRINTF).collect();
        let mut expected = vec![0x7000, 0x3b, VIEWER_FORMAT];
        expected.extend(args![1.0f64, -2.5f64, 0.5f64, 3.0f64]);
        assert_eq!(print[0].1, expected);
    }
}
