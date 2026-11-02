//! `SpeedTree/lightingengine.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SpeedTreeRT's lighting engine: the materials of the branches, leaves and fronds, their
//! parse from the tree file and the static lighting of the leaves.

#[allow(unused_imports)]
use crate::prelude::*;

use super::speedtreert::{
    fld, fstp, operand, parse_int, pushed_float, throw_parse_error, PARSE_FLOAT,
};

layout! {
    /// `CLightingEngine` (Xbox PDB), 0xB0 bytes on the PC as on the Xbox (no `std::vector`
    /// member); the offsets agree with this unit's code.
    pub struct CLightingEngine: 0xB0 {
        /// `m_eBranchLightingMethod` (Xbox PDB).
        0x00 m_eBranchLightingMethod: u32,
        /// `m_afBranchMaterial` (Xbox PDB): 13 floats.
        0x04 m_afBranchMaterial: f32,
        /// `m_eLeafLightingMethod` (Xbox PDB).
        0x38 m_eLeafLightingMethod: u32,
        /// `m_afLeafMaterial` (Xbox PDB): 13 floats.
        0x3C m_afLeafMaterial: f32,
        /// `m_fLeafLightingAdjustmentScalar` (Xbox PDB).
        0x70 m_fLeafLightingAdjustmentScalar: f32,
        /// `m_eStaticLightingStyle` (Xbox PDB): bit 0 colours the leaves, bit 1 adjusts them.
        0x74 m_eStaticLightingStyle: u32,
        /// `m_eFrondLightingMethod` (Xbox PDB).
        0x78 m_eFrondLightingMethod: u32,
        /// `m_afFrondMaterial` (Xbox PDB): 13 floats.
        0x7C m_afFrondMaterial: f32,
    }
}

/// The statics of the light set-up (the addresses are matched by their uses): the eight
/// enabled bytes (`011f8d50`) and the table of eight 0x40-byte light entries (`011ad168`):
/// direction or position `+0` (three floats), colour coefficients `+0xC`, `+0x18` (three floats
/// each), a type float at `+0x30` (0: directional) and the attenuation terms at `+0x34`,
/// `+0x38` and `+0x3C`.
const LIGHT_ENABLED: u32 = 0x011f_8d50;
const LIGHT_TABLE: u32 = 0x011a_d168;
/// The calls the static lighting makes: `CVec3::Normalize` (`00b12790`), the dot product
/// (`00b2a180`), the difference `this - other` (`00b13040`: this, a result, other), the sum and
/// the scaled product of `speedtreert.cpp` (`00b0cf90`, `00b0cff0`) and the distance
/// (`00b04d40`), the vector iterator of the leaf lists (`00b42980` construct, `00b2ab50`
/// dereference, `00b2abc0` advance, `00b2ac30` comparison) and the billboard leaf's
/// `GetColor` (`00b40430`), `SetColor` (`00b404c0`) and `AdjustStaticLighting` (`00b40210`).
const NORMALIZE: u32 = 0x00b1_2790;
const DOT_PRODUCT: u32 = 0x00b2_a180;
const VEC3_DIFFERENCE: u32 = 0x00b1_3040;
const VEC3_SUM: u32 = 0x00b0_cf90;
const VEC3_SCALED: u32 = 0x00b0_cff0;
const VEC3_DISTANCE: u32 = 0x00b0_4d40;
const ITERATOR_CONSTRUCT: u32 = 0x00b4_2980;
const ITERATOR_DEREFERENCE: u32 = 0x00b2_ab50;
const ITERATOR_ADVANCE: u32 = 0x00b2_abc0;
const ITERATOR_AT_END: u32 = 0x00b2_ac30;
const LEAF_GET_COLOR: u32 = 0x00b4_0430;
const LEAF_SET_COLOR: u32 = 0x00b4_04c0;
const LEAF_ADJUST_STATIC_LIGHTING: u32 = 0x00b4_0210;

// Translated from 00b2a1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec3::operator+=` (unnamed in the engine map): adds the three floats of `other` to `this`,
/// one component at a time. Returns `this`.
pub fn fn_00b2a1c0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    for axis in 0..3u32 {
        let sum = x87_add(
            fld(e, this.addr() + 4 * axis),
            operand(e, other.addr() + 4 * axis),
        );
        fstp(e, this.addr() + 4 * axis, sum);
    }
    this
}

/// The three floats at `source` through the x87 stack (a quiet copy of each) to `target`.
fn copy_vector(e: &mut Engine, target: u32, source: u32) {
    for axis in 0..3u32 {
        let bits = pushed_float(e, source + 4 * axis);
        e.mem.set_u32(target + 4 * axis, bits);
    }
}

/// Clamps a colour component to 0..1 as the exe's tests do: a value below zero becomes 0, one
/// above 1 becomes 1, a NaN stays (the other values are copied through the x87 stack).
fn clamp_component(e: &mut Engine, address: u32) {
    let zero: f64 = e.global(0x0101_2060);
    let one: f64 = e.global(0x0101_2070);
    let value = fld(e, address);
    if value < zero {
        e.mem.set_f32(address, 0.0);
    } else if value > one {
        e.mem.set_f32(address, 1.0);
    } else {
        let bits = pushed_float(e, address);
        e.mem.set_u32(address, bits);
    }
}

// Translated from 00b2a210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CLightingEngine::ComputeLeafStaticLighting` (Xbox PDB): with the leaf lighting method 1 and
/// a static lighting style set, for each of `count` leaf levels (`lods`: 0x18-byte
/// `std::vector`s of leaf pointers) and each leaf in it, sums over the enabled lights
/// (`011f8d50`) the colour the light gives the leaf at its position: the light vector (the
/// direction itself for a type-0 light, else the light minus the leaf position) is normalised and
/// dotted with the leaf's normal (`+0x1C`); the largest dot keeps its raw light vector for the
/// adjustment; a negative dot is zeroed and the dot is blended with the adjustment scalar (`+0x70`).
/// With style bit 0 each light adds `(ambient + (leaf colour * (light colour * dot) + diffuse) *
/// attenuation)` where the diffuse term is the leaf material (`+0x48`) times the light's diffuse
/// colour and the attenuation `1 / (c0 + c1 d + c2 d d)` (1 when the type is 0 or the divisor
/// zero); the sum is clamped to 0..1 and set as the leaf's colour (`00b404c0`); with style bit
/// 1 the leaf's `AdjustStaticLighting` (`00b40210`: the centre, the best light vector, the
/// scalar) runs.
pub fn c_lighting_engine_compute_leaf_static_lighting(
    e: &mut Engine,
    this: Ptr<CLightingEngine>,
    centre: u32,
    lods: u32,
    count: i32,
) {
    if e.get(this, CLightingEngine::m_eLeafLightingMethod) != 1
        || e.get(this, CLightingEngine::m_eStaticLightingStyle) == 0
    {
        return;
    }
    let base = this.addr();
    let mut level = 0i32;
    while level < count {
        // The frame: two iterators (8 bytes each), then vectors of 12 bytes: the leaf colour,
        // the best light vector, the sum, the light, its normalisation, the difference, the
        // ambient and diffuse terms and the working results.
        e.with_stack(0xA0, |e, frame| {
            let f = frame.addr();
            let (iterator, end) = (f, f + 8);
            let (colour, best, sum, light) = (f + 0x10, f + 0x1C, f + 0x28, f + 0x34);
            let (direction, difference) = (f + 0x40, f + 0x4C);
            let (ambient, diffuse) = (f + 0x58, f + 0x64);
            let (term, scaled, accumulated, shifted) = (f + 0x70, f + 0x7C, f + 0x88, f + 0x94);
            let vector = lods.wrapping_add((level as u32).wrapping_mul(0x18));
            let first = e.mem.u32(vector + 0x0C);
            e.call(ITERATOR_CONSTRUCT, &args![iterator, first, vector]);
            loop {
                let last = e.mem.u32(vector + 0x10);
                e.call(ITERATOR_CONSTRUCT, &args![end, last, vector]);
                if e.call(ITERATOR_AT_END, &args![iterator, end]).bool() {
                    break;
                }
                let leaf_of = |e: &mut Engine| {
                    let slot = e.call(ITERATOR_DEREFERENCE, &args![iterator]).u32();
                    e.mem.u32(slot)
                };
                let normal = leaf_of(e) + 0x1C;
                let position = leaf_of(e) + 4;
                let leaf = leaf_of(e);
                e.call(LEAF_GET_COLOR, &args![leaf, colour]);
                for offset in [0u32, 4, 8] {
                    e.mem.set_f32(best + offset, 0.0);
                }
                let mut best_dot = pushed_float(e, 0x0101_2054);
                for offset in [0u32, 4, 8] {
                    e.mem.set_f32(sum + offset, 0.0);
                }
                for light_index in 0..8u32 {
                    if e.global::<u8>(LIGHT_ENABLED + light_index) == 0 {
                        continue;
                    }
                    let entry = LIGHT_TABLE + (light_index << 6);
                    copy_vector(e, light, entry);
                    let zero: f64 = e.global(0x0101_2060);
                    let source = if fld(e, entry + 0x30) == zero {
                        light
                    } else {
                        e.call(VEC3_DIFFERENCE, &args![light, difference, position])
                            .u32()
                    };
                    for offset in [0u32, 4, 8] {
                        let word = e.mem.u32(source + offset);
                        e.mem.set_u32(direction + offset, word);
                    }
                    e.call(NORMALIZE, &args![direction]);
                    let mut dot = x87_store32(e.call(DOT_PRODUCT, &args![direction, normal]).f64());
                    // `FCOMPP` of the best against the new dot: it takes over unless best > dot.
                    if x87_load32(best_dot) <= x87_load32(dot) {
                        best_dot = dot;
                        for offset in [0u32, 4, 8] {
                            let word = e.mem.u32(light + offset);
                            e.mem.set_u32(best + offset, word);
                        }
                    }
                    if x87_load32(dot) < zero {
                        dot = 0.0f32.to_bits();
                    }
                    let adjustment = pushed_float(e, base + 0x70);
                    let weighted = x87_mul(x87_load32(dot), x87_sub(1.0, x87_load32(adjustment)));
                    dot = x87_store32(x87_add(weighted, x87_m32(adjustment)));
                    if e.get(this, CLightingEngine::m_eStaticLightingStyle) & 1 == 0 {
                        continue;
                    }
                    copy_vector(e, ambient, base + 0x60);
                    let mut products = [0u32; 3];
                    for (axis, product) in products.iter_mut().enumerate() {
                        let offset = 4 * axis as u32;
                        *product = x87_store32(x87_mul(
                            fld(e, base + 0x48 + offset),
                            operand(e, entry + 0x18 + offset),
                        ));
                    }
                    for (axis, bits) in products.into_iter().enumerate() {
                        e.mem
                            .set_u32(diffuse + 4 * axis as u32, x87_quiet_bits(bits));
                    }
                    let mut attenuation = 1.0f32.to_bits();
                    if fld(e, entry + 0x30) != zero {
                        let distance = e.call(VEC3_DISTANCE, &args![position, light]).f64();
                        let distance = x87_store32(distance);
                        let linear = x87_add(
                            x87_mul(fld(e, entry + 0x38), x87_m32(distance)),
                            operand(e, entry + 0x34),
                        );
                        let quadratic = x87_mul(
                            x87_mul(fld(e, entry + 0x3C), x87_m32(distance)),
                            x87_m32(distance),
                        );
                        let divisor = x87_store32(x87_add(linear, quadratic));
                        if x87_load32(divisor) != zero {
                            attenuation = x87_store32(x87_div(1.0, x87_load32(divisor)));
                        }
                    }
                    let mut terms = [0u32; 3];
                    for (axis, term_bits) in terms.iter_mut().enumerate() {
                        let offset = 4 * axis as u32;
                        let coefficient = x87_mul(fld(e, entry + 0xC + offset), x87_m32(dot));
                        *term_bits = x87_store32(x87_mul(fld(e, colour + offset), coefficient));
                    }
                    for (axis, bits) in terms.into_iter().enumerate() {
                        e.mem.set_u32(term + 4 * axis as u32, x87_quiet_bits(bits));
                    }
                    let lit = e.call(VEC3_SUM, &args![diffuse, scaled, term]).u32();
                    let faded = e
                        .call(
                            VEC3_SCALED,
                            &args![lit, accumulated, x87_quiet_bits(attenuation)],
                        )
                        .u32();
                    let total = e.call(VEC3_SUM, &args![ambient, shifted, faded]).u32();
                    fn_00b2a1c0(e, Ptr::new(sum), Ptr::new(total));
                }
                for offset in [0u32, 4, 8] {
                    clamp_component(e, sum + offset);
                }
                let style = e.get(this, CLightingEngine::m_eStaticLightingStyle);
                if style & 1 != 0 {
                    let leaf = leaf_of(e);
                    e.call(LEAF_SET_COLOR, &args![leaf, sum, 0u32]);
                }
                let style = e.get(this, CLightingEngine::m_eStaticLightingStyle);
                if style & 2 != 0 {
                    let leaf = leaf_of(e);
                    let adjustment = pushed_float(e, base + 0x70);
                    e.call(
                        LEAF_ADJUST_STATIC_LIGHTING,
                        &args![leaf, centre, best, adjustment],
                    );
                }
                e.call(ITERATOR_ADVANCE, &args![iterator]);
            }
        });
        level += 1;
    }
}

// Translated from 00b2a940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2a940 - the FS:[0] exception frame (destroying the message string
// when a parse error is thrown) is not translated.
/// `CLightingEngine::Parse` (Xbox PDB): reads tagged entries of the lighting chunk until the tag
/// 8001: 8002 the branch lighting method (`+0`), 8003 its 13 material floats (`+4`), 8004 the
/// leaf method (`+0x38`), 8005 its materials (`+0x3C`), 8006 the leaf adjustment scalar
/// (`+0x70`, a float), 8007 the static lighting style (`+0x74`), 8008 the frond method (`+0x78`)
/// and 8009 its materials (`+0x7C`); any other tag throws "malformed lighting information".
pub fn c_lighting_engine_parse(e: &mut Engine, this: Ptr<CLightingEngine>, access: Ptr) {
    let base = this.addr();
    let materials = |e: &mut Engine, offset: u32| {
        for index in 0..13u32 {
            let value = e.call(PARSE_FLOAT, &args![access]).f64();
            fstp(e, base + offset + 4 * index, value);
        }
    };
    let mut tag = parse_int(e, access);
    loop {
        match tag.wrapping_sub(0x1F42) as u32 {
            0 => {
                let method = parse_int(e, access);
                e.mem.set_i32(base, method);
            }
            1 => materials(e, 4),
            2 => {
                let method = parse_int(e, access);
                e.mem.set_i32(base + 0x38, method);
            }
            3 => materials(e, 0x3C),
            4 => {
                let value = e.call(PARSE_FLOAT, &args![access]).f64();
                fstp(e, base + 0x70, value);
            }
            5 => {
                let style = parse_int(e, access);
                e.mem.set_i32(base + 0x74, style);
            }
            6 => {
                let method = parse_int(e, access);
                e.mem.set_i32(base + 0x78, method);
            }
            7 => materials(e, 0x7C),
            _ => throw_parse_error(e, 0x010a_5630),
        }
        tag = parse_int(e, access);
        if tag == 0x1F41 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b2a1c0, fn_00b2a1c0(Ptr, Ptr) -> Ptr),
        entry!(
            0x00b2a210,
            c_lighting_engine_compute_leaf_static_lighting(Ptr<CLightingEngine>, u32, u32, i32)
        ),
        entry!(
            0x00b2a940,
            c_lighting_engine_parse(Ptr<CLightingEngine>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn stub(e: &mut Engine, calls: &Calls, address: u32, eax: u32) {
        let log = calls.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn calls_to(calls: &Calls, address: u32) -> Vec<Vec<u32>> {
        calls
            .borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn global(e: &mut Engine, address: u32, bits: u32) {
        e.map(address, 4);
        e.mem.set_u32(address, bits);
    }

    fn global_f64(e: &mut Engine, address: u32, value: f64) {
        e.map(address, 8);
        e.mem.set_f64(address, value);
    }

    fn floats(e: &mut Engine, address: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(address + 4 * i as u32, *value);
        }
    }

    fn read3(e: &Engine, address: u32) -> [f32; 3] {
        [0, 4, 8].map(|offset| e.mem.f32(address + offset))
    }

    /// `CTreeFileAccess` doubles: `ParseInt` returns `tags` in turn (then 0), `ParseFloat`
    /// `values`; both advance the index.
    fn file_doubles(e: &mut Engine, tags: Vec<i32>, values: Vec<f64>) {
        let mut queue = tags.into_iter();
        e.register_double(0x00b3_ed80, move |e, a| {
            let index = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], index + 1);
            Ret {
                eax: queue.next().unwrap_or(0) as u32,
                ..Ret::default()
            }
        });
        let mut values = values.into_iter();
        e.register_double(0x00b3_ed20, move |e, a| {
            let index = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], index + 1);
            Ret {
                st0: values.next().unwrap_or(0.0),
                ..Ret::default()
            }
        });
    }

    #[test]
    fn a_vector_is_added_in_place() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xC);
        let other = e.mem.alloc(0xC);
        floats(&mut e, this, &[1.0, 2.0, 3.0]);
        floats(&mut e, other, &[0.5, 0.25, 0.125]);
        assert_eq!(e.call(0x00b2_a1c0, &args![this, other]).u32(), this);
        assert_eq!(read3(&e, this), [1.5, 2.25, 3.125]);
        assert_eq!(read3(&e, other), [0.5, 0.25, 0.125]);
    }

    #[test]
    fn the_lighting_chunk_fills_the_engine() {
        let mut e = Engine::new();
        let tags = vec![
            8002, 5, 8003, 8004, 6, 8005, 8006, 8007, 7, 8008, 9, 8009, 8001,
        ];
        file_doubles(&mut e, tags, (1..=40).map(f64::from).collect());
        let lighting = e.new_object::<CLightingEngine>();
        let access = e.mem.alloc(0x14);
        e.call(0x00b2_a940, &args![lighting, access]);
        assert_eq!(e.get(lighting, CLightingEngine::m_eBranchLightingMethod), 5);
        assert_eq!(e.get(lighting, CLightingEngine::m_eLeafLightingMethod), 6);
        assert_eq!(e.get(lighting, CLightingEngine::m_eStaticLightingStyle), 7);
        assert_eq!(e.get(lighting, CLightingEngine::m_eFrondLightingMethod), 9);
        let base = lighting.addr();
        let at = |e: &Engine, offset: u32, count: u32| -> Vec<f32> {
            (0..count)
                .map(|i| e.mem.f32(base + offset + 4 * i))
                .collect()
        };
        assert_eq!(
            at(&e, 4, 13),
            (1..=13).map(|i| i as f32).collect::<Vec<_>>()
        );
        assert_eq!(
            at(&e, 0x3C, 13),
            (14..=26).map(|i| i as f32).collect::<Vec<_>>()
        );
        assert_eq!(
            e.get(lighting, CLightingEngine::m_fLeafLightingAdjustmentScalar),
            27.0
        );
        assert_eq!(
            at(&e, 0x7C, 13),
            (28..=40).map(|i| i as f32).collect::<Vec<_>>()
        );
    }

    #[test]
    fn the_lighting_chunk_throws_on_an_unknown_tag() {
        let mut e = Engine::new();
        let calls: Calls = Rc::default();
        for address in [
            0x0044_a4c0,
            0x0044_a730,
            0x0044_a580,
            0x00ad_4210,
            0x00ad_42e0,
            0x00ec_78e0,
        ] {
            stub(&mut e, &calls, address, 0);
        }
        e.register(0x00ec_7894, |_, _| panic!("thrown"));
        e.map(0x0101_73f0, 4);
        file_doubles(&mut e, vec![8010], vec![]);
        let lighting = e.new_object::<CLightingEngine>();
        let access = e.mem.alloc(0x14);
        let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00b2_a940, &args![lighting, access]);
        }));
        assert!(matches!(
            thrown,
            Err(payload) if payload.downcast_ref::<&str>() == Some(&"thrown")
        ));
        assert_eq!(calls_to(&calls, 0x0044_a4c0)[0][1], 0x010a_5630);
    }

    /// The doubles of the static lighting: the vector iterator of the leaf lists, the billboard
    /// leaf's colour (1, 1, 1), the recorded `SetColor` and `AdjustStaticLighting`, `Normalize`,
    /// the dot product and the difference as real arithmetic; the statics of the lights.
    fn lighting_fixture(e: &mut Engine, calls: &Calls) {
        e.register_double(ITERATOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register_double(ITERATOR_AT_END, |e, a| Ret {
            eax: (e.mem.u32(a[0] + 4) == e.mem.u32(a[1] + 4)) as u32,
            ..Ret::default()
        });
        e.register_double(ITERATOR_DEREFERENCE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register_double(ITERATOR_ADVANCE, |e, a| {
            let position = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, position + 4);
            Ret::default()
        });
        e.register_double(LEAF_GET_COLOR, |e, a| {
            floats(e, a[1], &[1.0, 1.0, 1.0]);
            Ret::default()
        });
        let log = calls.clone();
        e.register_double(LEAF_SET_COLOR, move |e, a| {
            let mut words = vec![a[0]];
            words.extend([0, 4, 8].map(|offset| e.mem.u32(a[1] + offset)));
            words.push(a[2]);
            log.borrow_mut().push((LEAF_SET_COLOR, words));
            Ret::default()
        });
        let log = calls.clone();
        e.register_double(LEAF_ADJUST_STATIC_LIGHTING, move |e, a| {
            let mut words = vec![a[0], a[1]];
            words.extend([0, 4, 8].map(|offset| e.mem.u32(a[2] + offset)));
            words.push(a[3]);
            log.borrow_mut().push((LEAF_ADJUST_STATIC_LIGHTING, words));
            Ret::default()
        });
        e.register_double(NORMALIZE, |e, a| {
            let v = [0, 4, 8].map(|offset| f64::from(e.mem.f32(a[0] + offset)));
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (i, value) in v.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, (value / length) as f32);
            }
            Ret::default()
        });
        e.register_double(DOT_PRODUCT, |e, a| {
            let dot: f64 = (0..3u32)
                .map(|i| f64::from(e.mem.f32(a[0] + 4 * i)) * f64::from(e.mem.f32(a[1] + 4 * i)))
                .sum();
            Ret {
                st0: dot,
                ..Ret::default()
            }
        });
        e.register_double(VEC3_DIFFERENCE, |e, a| {
            for i in 0..3u32 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.map(LIGHT_ENABLED, 8);
        e.map(LIGHT_TABLE, 0x200);
        global(e, 0x0101_2054, (-1.0f32).to_bits());
        global_f64(e, 0x0101_2060, 0.0);
        global_f64(e, 0x0101_2070, 1.0);
    }

    /// A lighting engine with leaf method 1, the given style and the leaf material: diffuse
    /// (0.5, 0.5, 0.5) at `+0x48`, ambient `ambient` at `+0x60`.
    fn lighting_engine(e: &mut Engine, style: u32, ambient: [f32; 3]) -> Ptr<CLightingEngine> {
        let lighting = e.new_object::<CLightingEngine>();
        e.set(lighting, CLightingEngine::m_eLeafLightingMethod, 1);
        e.set(lighting, CLightingEngine::m_eStaticLightingStyle, style);
        floats(e, lighting.addr() + 0x48, &[0.5, 0.5, 0.5]);
        floats(e, lighting.addr() + 0x60, &ambient);
        lighting
    }

    /// Enables light `index`: the vector, the type (0 directional), the colour coefficients, the
    /// diffuse colour and the attenuation terms.
    fn light(e: &mut Engine, index: u32, vector: [f32; 3], kind: f32, attenuation: [f32; 3]) {
        e.mem.set_u8(LIGHT_ENABLED + index, 1);
        let entry = LIGHT_TABLE + 0x40 * index;
        floats(e, entry, &vector);
        floats(e, entry + 0xC, &[0.5, 0.25, 0.125]);
        floats(e, entry + 0x18, &[1.0, 1.0, 1.0]);
        e.mem.set_f32(entry + 0x30, kind);
        floats(e, entry + 0x34, &attenuation);
    }

    /// `levels` leaf levels, each with one leaf (normal (0, 0, 1), position (0, 0, 0)): the
    /// array of lists and the leaves.
    fn leaf_levels(e: &mut Engine, levels: u32) -> (u32, Vec<u32>) {
        let lists = e.mem.alloc(0x18 * levels);
        let mut leaves = vec![];
        for level in 0..levels {
            let leaf = e.mem.alloc(0x40);
            floats(e, leaf + 0x1C, &[0.0, 0.0, 1.0]);
            let storage = e.mem.alloc(4);
            e.mem.set_u32(storage, leaf);
            let list = lists + 0x18 * level;
            e.mem.set_u32(list + 0x0C, storage);
            e.mem.set_u32(list + 0x10, storage + 4);
            leaves.push(leaf);
        }
        (lists, leaves)
    }

    #[test]
    fn a_directional_light_colours_and_adjusts_the_leaf() {
        let mut e = Engine::new();
        let calls: Calls = Rc::default();
        lighting_fixture(&mut e, &calls);
        let lighting = lighting_engine(&mut e, 3, [0.25, 0.0, 0.5]);
        light(&mut e, 0, [0.0, 0.0, 1.0], 0.0, [0.0; 3]);
        let (lists, leaves) = leaf_levels(&mut e, 1);
        let centre = e.mem.alloc(0xC);
        e.call(0x00b2_a210, &args![lighting, centre, lists, 1i32]);
        // dot = 1; colour = ambient + (leaf colour * (coefficients * dot) + diffuse) * 1,
        // clamped to 1.
        let one = 1.0f32.to_bits();
        assert_eq!(
            calls_to(&calls, LEAF_SET_COLOR),
            vec![vec![leaves[0], one, 0.75f32.to_bits(), one, 0]]
        );
        // The best light vector (0, 0, 1) and the adjustment scalar go to the adjustment.
        assert_eq!(
            calls_to(&calls, LEAF_ADJUST_STATIC_LIGHTING),
            vec![vec![leaves[0], centre, 0, 0, one, 0]]
        );
    }

    #[test]
    fn a_positional_light_is_attenuated_by_distance() {
        let calls: Calls = Rc::default();
        let mut e = Engine::new();
        lighting_fixture(&mut e, &calls);
        // Style 1: colour only.
        let lighting = lighting_engine(&mut e, 1, [0.25, 0.0, 0.5]);
        // The light sits 2 above the leaf; attenuation 1 / (1 + 0 * d + 0.25 * d * d) = 0.5.
        light(&mut e, 0, [0.0, 0.0, 2.0], 1.0, [1.0, 0.0, 0.25]);
        let (lists, leaves) = leaf_levels(&mut e, 1);
        let centre = e.mem.alloc(0xC);
        e.call(0x00b2_a210, &args![lighting, centre, lists, 1i32]);
        // (colour * coefficients + diffuse) * 0.5 + ambient.
        assert_eq!(
            calls_to(&calls, LEAF_SET_COLOR),
            vec![vec![
                leaves[0],
                0.75f32.to_bits(),
                0.375f32.to_bits(),
                0.8125f32.to_bits(),
                0
            ]]
        );
        assert!(calls_to(&calls, LEAF_ADJUST_STATIC_LIGHTING).is_empty());
    }

    #[test]
    fn a_zero_divisor_leaves_the_attenuation_at_one() {
        let calls: Calls = Rc::default();
        let mut e = Engine::new();
        lighting_fixture(&mut e, &calls);
        let lighting = lighting_engine(&mut e, 1, [0.0, 0.0, 0.0]);
        light(&mut e, 0, [0.0, 0.0, 2.0], 1.0, [0.0, 0.0, 0.0]);
        let (lists, leaves) = leaf_levels(&mut e, 1);
        let centre = e.mem.alloc(0xC);
        e.call(0x00b2_a210, &args![lighting, centre, lists, 1i32]);
        // colour * coefficients + diffuse = (1, 0.75, 0.625) unattenuated.
        assert_eq!(
            calls_to(&calls, LEAF_SET_COLOR),
            vec![vec![
                leaves[0],
                1.0f32.to_bits(),
                0.75f32.to_bits(),
                0.625f32.to_bits(),
                0
            ]]
        );
    }

    #[test]
    fn the_brightest_light_is_kept_for_the_adjustment() {
        let calls: Calls = Rc::default();
        let mut e = Engine::new();
        lighting_fixture(&mut e, &calls);
        // Style 2: only the adjustment.
        let lighting = lighting_engine(&mut e, 2, [0.0; 3]);
        light(&mut e, 0, [0.0, 1.0, 0.0], 0.0, [0.0; 3]);
        light(&mut e, 1, [0.0, 0.0, 1.0], 0.0, [0.0; 3]);
        light(&mut e, 2, [0.0, 0.6, 0.8], 0.0, [0.0; 3]);
        let (lists, leaves) = leaf_levels(&mut e, 1);
        let centre = e.mem.alloc(0xC);
        e.call(0x00b2_a210, &args![lighting, centre, lists, 1i32]);
        assert!(calls_to(&calls, LEAF_SET_COLOR).is_empty());
        assert_eq!(
            calls_to(&calls, LEAF_ADJUST_STATIC_LIGHTING),
            vec![vec![leaves[0], centre, 0, 0, 1.0f32.to_bits(), 0]]
        );
    }

    #[test]
    fn colours_are_clamped_and_every_level_is_lit() {
        let calls: Calls = Rc::default();
        let mut e = Engine::new();
        lighting_fixture(&mut e, &calls);
        // No lights enabled: the sum stays 0; a negative ambient cannot make it negative
        // because no light adds it. With a light and an ambient of -4 the sum is clamped to 0.
        let lighting = lighting_engine(&mut e, 1, [-4.0, -4.0, -4.0]);
        let (lists, leaves) = leaf_levels(&mut e, 2);
        let centre = e.mem.alloc(0xC);
        e.call(0x00b2_a210, &args![lighting, centre, lists, 2i32]);
        let zero = 0.0f32.to_bits();
        assert_eq!(
            calls_to(&calls, LEAF_SET_COLOR),
            vec![
                vec![leaves[0], zero, zero, zero, 0],
                vec![leaves[1], zero, zero, zero, 0]
            ]
        );
        light(&mut e, 0, [0.0, 0.0, 1.0], 0.0, [0.0; 3]);
        e.call(0x00b2_a210, &args![lighting, centre, lists, 1i32]);
        let last = calls_to(&calls, LEAF_SET_COLOR).pop().unwrap();
        assert_eq!(last, vec![leaves[0], zero, zero, zero, 0]);
    }

    #[test]
    fn a_negative_dot_is_zeroed_and_the_adjustment_scalar_blends_it() {
        let calls: Calls = Rc::default();
        let mut e = Engine::new();
        lighting_fixture(&mut e, &calls);
        let lighting = lighting_engine(&mut e, 1, [0.0; 3]);
        e.set(
            lighting,
            CLightingEngine::m_fLeafLightingAdjustmentScalar,
            0.5,
        );
        // The light points away from the leaf's normal: dot -1 becomes 0, then 0 * 0.5 + 0.5.
        light(&mut e, 0, [0.0, 0.0, -1.0], 0.0, [0.0; 3]);
        let (lists, leaves) = leaf_levels(&mut e, 1);
        let centre = e.mem.alloc(0xC);
        e.call(0x00b2_a210, &args![lighting, centre, lists, 1i32]);
        // dot = 0.5: terms (0.25, 0.125, 0.0625) + diffuse 0.5.
        assert_eq!(
            calls_to(&calls, LEAF_SET_COLOR),
            vec![vec![
                leaves[0],
                0.75f32.to_bits(),
                0.625f32.to_bits(),
                0.5625f32.to_bits(),
                0
            ]]
        );
    }

    #[test]
    fn leaves_are_left_alone_without_leaf_lighting() {
        let calls: Calls = Rc::default();
        let mut e = Engine::new();
        lighting_fixture(&mut e, &calls);
        let (lists, _) = leaf_levels(&mut e, 1);
        let centre = e.mem.alloc(0xC);
        for (method, style) in [(0, 3), (1, 0)] {
            let lighting = lighting_engine(&mut e, style, [0.0; 3]);
            e.set(lighting, CLightingEngine::m_eLeafLightingMethod, method);
            e.call(0x00b2_a210, &args![lighting, centre, lists, 1i32]);
        }
        assert!(calls_to(&calls, LEAF_SET_COLOR).is_empty());
        assert!(calls_to(&calls, LEAF_ADJUST_STATIC_LIGHTING).is_empty());
    }
}
