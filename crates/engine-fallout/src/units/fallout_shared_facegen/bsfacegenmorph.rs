//! `fallout shared/facegen/bsfacegenmorph.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSFaceGenMorph` (vtable `01065cb8`) and its two kinds:
//! `BSFaceGenMorphDifferential` (`01065cc4`: vertex offsets at +4, their
//! count at +8) and `BSFaceGenMorphStatistical` (`01065cd0`: vertex
//! indices at +4, their count at +8, the first basis row at +0xC). Their
//! `ApplyMorph` (slot +4) moves an object's vertices by `weight` times the
//! morph.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtables (RTTI `.?AVBSFaceGenMorph@@`,
/// `.?AVBSFaceGenMorphDifferential@@`, `.?AVBSFaceGenMorphStatistical@@`).
const MORPH_VTABLE: u32 = 0x0106_5cb8;
const DIFFERENTIAL_VTABLE: u32 = 0x0106_5cc4;
const STATISTICAL_VTABLE: u32 = 0x0106_5cd0;
/// The `double` 0.0 and 1.0.
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
/// `operator new` (cdecl: size) and `operator delete` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `_vector constructor iterator` (`00401050`, stdcall: array, size,
/// count, constructor) and `NiPoint3`'s constructor (`006815c0`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// The object's geometry (`005495f0`), its vertex count (`004a8ae0`,
/// `AX`) and its vertex iterator start (`00a67410`: geometry; iterator).
const OBJECT_GEOMETRY: u32 = 0x0054_95f0;
const GEOMETRY_VERTEX_COUNT: u32 = 0x004a_8ae0;
const GEOMETRY_BEGIN: u32 = 0x00a6_7410;
/// The vertex iterator: constructor (`00534550`), read (`00656fd0`:
/// iterator; point), write (`006533f0`: iterator; point), advance
/// (`00657be0`), indexed read (`00534570`: iterator; index, point) and
/// indexed write (`0065b390`: iterator; index, point).
const ITERATOR_CONSTRUCT: u32 = 0x0053_4550;
const ITERATOR_READ: u32 = 0x0065_6fd0;
const ITERATOR_WRITE: u32 = 0x0065_33f0;
const ITERATOR_NEXT: u32 = 0x0065_7be0;
const ITERATOR_READ_AT: u32 = 0x0053_4570;
const ITERATOR_WRITE_AT: u32 = 0x0065_b390;
/// `NiPoint3` arithmetic: scale (`0045bb20`: point; out, float), add in
/// place (`0063c8a0`: point; other), difference (`00439ef0`: point; out,
/// other).
const POINT_SCALE: u32 = 0x0045_bb20;
const POINT_ADD: u32 = 0x0063_c8a0;
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
/// The word at +0xC (`0084e3a0`): the statistical basis of a base.
const WORD_AT_C: u32 = 0x0084_e3a0;

/// `weight` in 0..=1 as the exe tests it: false when not above 0.0
/// (`TEST AH,0x41; JNP`) or above 1.0; NaN passes.
fn weight_in_range(e: &Engine, weight: u32) -> bool {
    let w = x87_m32(weight);
    !(w <= e.global::<f64>(ZERO)) && !(w > e.global::<f64>(ONE))
}

// Translated from 00660690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorph`'s constructor body: its vtable. Returns `this`.
pub fn fn_00660690(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), MORPH_VTABLE);
    this
}

// Translated from 006606e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorph`'s destructor body: its vtable.
pub fn fn_006606e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MORPH_VTABLE);
}

// Translated from 00660700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphDifferential`'s constructor: the base, the vtable, the
/// count (+8) and, for a count, an array of that many `NiPoint3`s (+4;
/// null when the allocation failed or for no count).
// Translation incomplete: 00660700 - SEH exception frame (FS:[0]) that frees the array on unwinding is not modelled.
pub fn fn_00660700(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    fn_00660690(e, this);
    e.mem.set_u32(this.addr(), DIFFERENTIAL_VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(8), count);
    if e.mem.u32(this.addr().wrapping_add(8)) == 0 {
        e.mem.set_u32(this.addr().wrapping_add(4), 0);
        return this;
    }
    let count = e.mem.u32(this.addr().wrapping_add(8));
    let size = count.checked_mul(0xc).unwrap_or(u32::MAX);
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    let array = if block != 0 {
        e.call(
            VECTOR_CONSTRUCT,
            &args![block, 0xcu32, count, POINT_CONSTRUCT],
        );
        block
    } else {
        0
    };
    e.mem.set_u32(this.addr().wrapping_add(4), array);
    this
}

// Translated from 006607f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphDifferential::_scalar_deleting_destructor_` (Xbox PDB):
/// the destructor body, then `operator delete` with bit 0 of `flags`.
pub fn bs_face_gen_morph_differential_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00660820(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00660820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphDifferential`'s destructor body: the vtable, the offset
/// array freed when set, then the `BSFaceGenMorph` part.
pub fn fn_00660820(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), DIFFERENTIAL_VTABLE);
    let array = e.mem.u32(this.addr().wrapping_add(4));
    if array != 0 {
        e.call(OPERATOR_DELETE, &args![array]);
    }
    fn_006606e0(e, this);
}

// Translated from 00660860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphDifferential::ApplyMorph` (Xbox PDB): for a weight in
/// 0..=1, adds `weight` times each offset to the object's vertices, for
/// the smaller of the geometry's vertex count and the morph's count. The
/// second word is not read. (The geometry's count call leaves the pushed
/// morph count on the stack; `00660940` takes both.)
pub fn bs_face_gen_morph_differential_apply_morph(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    _unused_2: u32,
    weight: f32,
) {
    if object == 0 || !weight_in_range(e, weight.to_bits()) {
        return;
    }
    let weight = x87_quiet_bits(weight.to_bits());
    let geometry = e.call(OBJECT_GEOMETRY, &args![object]).u32();
    e.with_stack(0xc, |e, iterator| {
        e.call(ITERATOR_CONSTRUCT, &args![iterator]);
        e.call(GEOMETRY_BEGIN, &args![geometry, iterator]);
        e.with_stack(0xc, |e, point| {
            e.call(POINT_CONSTRUCT, &args![point]);
            let count = e.mem.u32(this.addr().wrapping_add(8));
            let vertices = e.call(GEOMETRY_VERTEX_COUNT, &args![geometry]).u16();
            let n = fn_00660940(e, vertices, count);
            let mut i = 0u32;
            while i < n {
                e.call(ITERATOR_READ, &args![iterator, point]);
                e.with_stack(0xc, |e, scaled| {
                    let offset = e
                        .mem
                        .u32(this.addr().wrapping_add(4))
                        .wrapping_add(i.wrapping_mul(0xc));
                    let r = e.call(POINT_SCALE, &args![offset, scaled, weight]).u32();
                    e.call(POINT_ADD, &args![point, r]);
                });
                e.call(ITERATOR_WRITE, &args![iterator, point]);
                i += 1;
                e.call(ITERATOR_NEXT, &args![iterator]);
            }
        });
    });
}

// Translated from 00660940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The smaller of a 16-bit `a` (zero-extended) and `b` (cdecl, unsigned).
pub fn fn_00660940(_e: &mut Engine, a: u16, b: u32) -> u32 {
    if u32::from(a) < b {
        u32::from(a)
    } else {
        b
    }
}

// Translated from 00660970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphStatistical`'s constructor: the base, the vtable, the
/// count (+8), the basis (+0xC) and, for a count, an index array (+4).
// Translation incomplete: 00660970 - SEH exception frame (FS:[0]) for the base part's unwinding is not modelled.
pub fn fn_00660970(e: &mut Engine, this: Ptr, count: u32, basis: u32) -> Ptr {
    fn_00660690(e, this);
    e.mem.set_u32(this.addr(), STATISTICAL_VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(8), count);
    e.mem.set_u32(this.addr().wrapping_add(0xc), basis);
    if e.mem.u32(this.addr().wrapping_add(8)) == 0 {
        e.mem.set_u32(this.addr().wrapping_add(4), 0);
        return this;
    }
    let count = e.mem.u32(this.addr().wrapping_add(8));
    let size = count.checked_mul(4).unwrap_or(u32::MAX);
    let array = e.call(OPERATOR_NEW, &args![size]).u32();
    e.mem.set_u32(this.addr().wrapping_add(4), array);
    this
}

// Translated from 00660a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphStatistical::_scalar_deleting_destructor_` (Xbox PDB).
pub fn bs_face_gen_morph_statistical_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00660a50(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00660a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphStatistical`'s destructor body: the vtable, the index
/// array freed when set, then the `BSFaceGenMorph` part.
pub fn fn_00660a50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), STATISTICAL_VTABLE);
    let array = e.mem.u32(this.addr().wrapping_add(4));
    if array != 0 {
        e.call(OPERATOR_DELETE, &args![array]);
    }
    fn_006606e0(e, this);
}

// Translated from 00660a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorphStatistical::ApplyMorph` (Xbox PDB): for a weight in
/// 0..=1 and a base with a basis (`base + 0xC`), moves each indexed vertex
/// by `weight` times the difference between basis row `+0xC + i` and the
/// vertex's own row.
pub fn bs_face_gen_morph_statistical_apply_morph(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    base: u32,
    weight: f32,
) {
    if object == 0 || !weight_in_range(e, weight.to_bits()) {
        return;
    }
    if e.call(WORD_AT_C, &args![base]).u32() == 0 {
        return;
    }
    let weight = x87_quiet_bits(weight.to_bits());
    let basis = e.call(WORD_AT_C, &args![base]).u32();
    let geometry = e.call(OBJECT_GEOMETRY, &args![object]).u32();
    e.with_stack(0xc, |e, iterator| {
        e.call(ITERATOR_CONSTRUCT, &args![iterator]);
        e.call(GEOMETRY_BEGIN, &args![geometry, iterator]);
        e.with_stack(0xc, |e, point| {
            e.call(POINT_CONSTRUCT, &args![point]);
            let mut i = 0u32;
            while i < e.mem.u32(this.addr().wrapping_add(8)) {
                let indices = e.mem.u32(this.addr().wrapping_add(4));
                let index = e.mem.u32(indices.wrapping_add(i.wrapping_mul(4)));
                e.call(ITERATOR_READ_AT, &args![iterator, index, point]);
                e.with_stack(0x18, |e, scratch| {
                    let difference = scratch;
                    let scaled = scratch.byte_add(0xc);
                    let row = e
                        .mem
                        .u32(this.addr().wrapping_add(0xc))
                        .wrapping_add(i)
                        .wrapping_mul(0xc)
                        .wrapping_add(basis);
                    let own = index.wrapping_mul(0xc).wrapping_add(basis);
                    let r = e.call(POINT_DIFFERENCE, &args![row, difference, own]).u32();
                    let r = e.call(POINT_SCALE, &args![r, scaled, weight]).u32();
                    e.call(POINT_ADD, &args![point, r]);
                });
                e.call(ITERATOR_WRITE_AT, &args![iterator, index, point]);
                i += 1;
            }
        });
    });
}

// Translated from 006606b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenMorph::_scalar_deleting_destructor_` (Xbox PDB, `RET 4`): the destructor
/// body (`fn_006606e0`), then `operator delete` with bit 0 of `flags`. Returns `this`.
pub fn bs_face_gen_morph_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006606e0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_0690, fn_00660690(Ptr) -> Ptr),
        entry!(0x0066_06e0, fn_006606e0(Ptr)),
        entry!(0x0066_06b0, bs_face_gen_morph_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0066_0700, fn_00660700(Ptr, u32) -> Ptr),
        entry!(
            0x0066_07f0,
            bs_face_gen_morph_differential_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0066_0820, fn_00660820(Ptr)),
        entry!(
            0x0066_0860,
            bs_face_gen_morph_differential_apply_morph(Ptr, u32, u32, f32)
        ),
        entry!(0x0066_0940, fn_00660940(u16, u32) -> u32),
        entry!(0x0066_0970, fn_00660970(Ptr, u32, u32) -> Ptr),
        entry!(
            0x0066_0a20,
            bs_face_gen_morph_statistical_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0066_0a50, fn_00660a50(Ptr)),
        entry!(
            0x0066_0a90,
            bs_face_gen_morph_statistical_apply_morph(Ptr, u32, u32, f32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn base_parts_set_the_vtable() {
        let mut e = Engine::new();
        let morph = e.mem.alloc(8);
        assert_eq!(e.call(0x0066_0690, &args![morph]).u32(), morph);
        assert_eq!(e.mem.u32(morph), MORPH_VTABLE);
        e.mem.set_u32(morph, 0);
        e.call(0x0066_06e0, &args![morph]);
        assert_eq!(e.mem.u32(morph), MORPH_VTABLE);
    }

    #[test]
    fn differential_construct_and_destroy() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        let morph = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_0700, &args![morph, 3u32]).u32(), morph);
        assert_eq!(e.mem.u32(morph), DIFFERENTIAL_VTABLE);
        assert_eq!(e.mem.u32(morph + 8), 3);
        let array = e.mem.u32(morph + 4);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![36]]);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![array, 0xc, 3, POINT_CONSTRUCT]]
        );
        e.call(0x0066_07f0, &args![morph, 1u32]);
        assert_eq!(
            calls_to(&e, OPERATOR_DELETE),
            vec![vec![array], vec![morph]]
        );
        assert_eq!(e.mem.u32(morph), MORPH_VTABLE);
        // No count: no array.
        fn_00660700(&mut e, Ptr::new(morph), 0);
        assert_eq!(e.mem.u32(morph + 4), 0);
        e.call_log = Some(vec![]);
        fn_00660820(&mut e, Ptr::new(morph));
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    /// Doubles for the vertex iterator (its first word is the current
    /// vertex's address) and plain `NiPoint3` arithmetic; the geometry has
    /// three vertices. Each test registers where the iterator starts.
    fn morph_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.register(OBJECT_GEOMETRY, |_, _| ret(0x00d3_0000));
        e.register(GEOMETRY_VERTEX_COUNT, |_, _| ret(3));
        e.register(ITERATOR_CONSTRUCT, |_, _| Ret::default());
        e.register(GEOMETRY_BEGIN, |_, _| Ret::default());
        e.register(POINT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ITERATOR_READ, |e, a| {
            let at = e.mem.u32(a[0]);
            for i in 0..3u32 {
                let w = e.mem.u32(at + i * 4);
                e.mem.set_u32(a[1] + i * 4, w);
            }
            Ret::default()
        });
        e.register(ITERATOR_WRITE, |e, a| {
            let at = e.mem.u32(a[0]);
            for i in 0..3u32 {
                let w = e.mem.u32(a[1] + i * 4);
                e.mem.set_u32(at + i * 4, w);
            }
            Ret::default()
        });
        e.register(ITERATOR_NEXT, |e, a| {
            let at = e.mem.u32(a[0]) + 0xc;
            e.mem.set_u32(a[0], at);
            Ret::default()
        });
        e.register(POINT_SCALE, |e, a| {
            let s = f32::from_bits(a[2]);
            for i in 0..3u32 {
                let v = e.mem.f32(a[0] + i * 4) * s;
                e.mem.set_f32(a[1] + i * 4, v);
            }
            ret(a[1])
        });
        e.register(POINT_ADD, |e, a| {
            for i in 0..3u32 {
                let v = e.mem.f32(a[0] + i * 4) + e.mem.f32(a[1] + i * 4);
                e.mem.set_f32(a[0] + i * 4, v);
            }
            ret(a[0])
        });
        e.register(POINT_DIFFERENCE, |e, a| {
            for i in 0..3u32 {
                let v = e.mem.f32(a[0] + i * 4) - e.mem.f32(a[2] + i * 4);
                e.mem.set_f32(a[1] + i * 4, v);
            }
            ret(a[1])
        });
        e
    }

    fn vertices_of(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(values.len() as u32 * 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + i as u32 * 4, *v);
        }
        at
    }

    #[test]
    fn differential_moves_the_first_vertices() {
        let mut e = morph_engine();
        let vertices = vertices_of(&mut e, &[1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0]);
        e.register_double(GEOMETRY_BEGIN, move |e, a| {
            e.mem.set_u32(a[1], vertices);
            Ret::default()
        });
        let offsets = vertices_of(&mut e, &[2.0, 4.0, 6.0, 8.0, 10.0, 12.0]);
        let morph = e.mem.alloc(0xc);
        e.mem.set_u32(morph + 4, offsets);
        e.mem.set_u32(morph + 8, 2);
        e.call_log = Some(vec![]);
        let words = args![morph, 0x900u32, 0u32, 0.5f32];
        e.call(0x0066_0860, &words);
        let moved: Vec<f32> = (0..9).map(|i| e.mem.f32(vertices + i * 4)).collect();
        assert_eq!(moved, vec![2.0, 3.0, 4.0, 6.0, 7.0, 8.0, 3.0, 3.0, 3.0]);
        // Weights at 0 or above 1, or no object: nothing.
        e.call_log = Some(vec![]);
        for w in [0.0f32, -1.0, 1.5] {
            bs_face_gen_morph_differential_apply_morph(&mut e, Ptr::new(morph), 0x900, 0, w);
        }
        bs_face_gen_morph_differential_apply_morph(&mut e, Ptr::new(morph), 0, 0, 0.5);
        assert!(calls_to(&e, OBJECT_GEOMETRY).is_empty());
    }

    #[test]
    fn min_of_a_word_and_a_count() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_0940, &args![0x1_0005u32, 9u32]).u32(), 5);
        assert_eq!(e.call(0x0066_0940, &args![7u32, 3u32]).u32(), 3);
    }

    #[test]
    fn statistical_construct_and_destroy() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let morph = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let words = args![morph, 4u32, 0x55u32];
        assert_eq!(e.call(0x0066_0970, &words).u32(), morph);
        assert_eq!(e.mem.u32(morph), STATISTICAL_VTABLE);
        assert_eq!(e.mem.u32(morph + 0xc), 0x55);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![16]]);
        let array = e.mem.u32(morph + 4);
        e.call(0x0066_0a20, &args![morph, 0u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![array]]);
        e.call(0x0066_0a50, &args![morph]);
        assert_eq!(e.mem.u32(morph), MORPH_VTABLE);
    }

    #[test]
    fn statistical_moves_the_indexed_vertices() {
        let mut e = morph_engine();
        let vertices = vertices_of(&mut e, &[1.0; 9]);
        e.register_double(ITERATOR_READ_AT, move |e, a| {
            for i in 0..3u32 {
                let w = e.mem.u32(vertices + a[1] * 0xc + i * 4);
                e.mem.set_u32(a[2] + i * 4, w);
            }
            Ret::default()
        });
        e.register_double(ITERATOR_WRITE_AT, move |e, a| {
            for i in 0..3u32 {
                let w = e.mem.u32(a[2] + i * 4);
                e.mem.set_u32(vertices + a[1] * 0xc + i * 4, w);
            }
            Ret::default()
        });
        // Basis rows: row 0..2 the vertices' own, rows 3.. the targets.
        let basis = vertices_of(
            &mut e,
            &[
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 5.0,
            ],
        );
        let base = e.mem.alloc(0x10);
        e.mem.set_u32(base + 0xc, basis);
        e.register(WORD_AT_C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        let indices = e.mem.alloc(8);
        e.mem.set_u32(indices, 2);
        e.mem.set_u32(indices + 4, 0);
        let morph = e.mem.alloc(0x10);
        e.mem.set_u32(morph + 4, indices);
        e.mem.set_u32(morph + 8, 2);
        e.mem.set_u32(morph + 0xc, 3);
        let words = args![morph, 0x900u32, base, 0.5f32];
        e.call(0x0066_0a90, &words);
        // Vertex 2: 1 + 0.5 * (row 3 - row 2) = 2; vertex 0: 1 + 0.5 * 5.
        let moved: Vec<f32> = (0..9).map(|i| e.mem.f32(vertices + i * 4)).collect();
        assert_eq!(moved, vec![3.5, 3.5, 3.5, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0]);
        // A base without a basis: nothing.
        e.mem.set_u32(base + 0xc, 0);
        e.call_log = Some(vec![]);
        bs_face_gen_morph_statistical_apply_morph(&mut e, Ptr::new(morph), 0x900, base, 0.5);
        assert!(calls_to(&e, OBJECT_GEOMETRY).is_empty());
    }

    #[test]
    fn morph_scalar_deleting_destructor() {
        let mut e = Engine::new();
        let morph = e.mem.alloc(8);
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_06b0, &args![morph, 2u32]).u32(), morph);
        assert_eq!(e.mem.u32(morph), MORPH_VTABLE);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0066_06b0, &args![morph, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![morph]]);
    }
}
