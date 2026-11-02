//! `(unplaced)/run_00a85580` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the destructor puts back (`01069b14`; its RTTI type descriptor
/// is `.?AVNiSkinInstance@@`).
const VTABLE_NI_SKIN_INSTANCE: u32 = 0x0106_9b14;
/// `00aa10f0`: frees the block at `this + 0x14` (`cdecl`, one word).
const FREE_00AA10F0: u32 = 0x00aa_10f0;
/// The global object (with a vtable) told about the destroyed instance
/// through its slot `0x104`, when there is one.
const GLOBAL_011F4748: u32 = 0x011f_4748;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00a5d3d0`: the base-class destructor the function ends in (`thiscall`).
const BASE_DESTRUCTOR: u32 = 0x00a5_d3d0;

/// Releases the reference-counted object in `field` (an `NiPointer`): when
/// the decremented count (`+4`) is zero, its virtual slot 4 deletes it.
fn release(e: &mut Engine, field: u32) {
    let object = e.mem.u32(field);
    if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00a85580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSkinInstance` destructor body (the vtable it restores is
/// `NiSkinInstance`'s): frees the block at `+0x14` (`00aa10f0`), tells the
/// object at `011f4748` (slot `0x104`, `this`) when there is one, releases
/// the references at `+0x0c` and `+0x08`, then runs the base destructor
/// (`00a5d3d0`).
pub fn fn_00a85580(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let block = e.mem.u32(base + 0x14);
    e.mem.set_u32(base, VTABLE_NI_SKIN_INSTANCE);
    e.call(FREE_00AA10F0, &args![block]);
    let listener = e.global::<u32>(GLOBAL_011F4748);
    if listener != 0 {
        e.vcall(listener, 0x104, &args![base]);
    }
    release(e, base + 0x0c);
    release(e, base + 0x08);
    e.call(BASE_DESTRUCTOR, &args![base]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_5580, fn_00a85580(Ptr)),
        entry!(0x00a8_6280, fn_00a86280(u32, u32) -> i32),
        entry!(
            0x00a8_55f0,
            fn_00a855f0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn ref_object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + 4, 0x7000_0004);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn destructor_frees_tells_releases_and_chains() {
        let mut e = Engine::new();
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        e.register(0x7000_0004, |_, _| Ret::default());
        e.register(0x7000_0104, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let listener = e.mem.alloc(4);
        let listener_vtable = e.mem.alloc(0x108);
        e.mem.set_u32(listener, listener_vtable);
        e.mem.set_u32(listener_vtable + 0x104, 0x7000_0104);
        e.map(GLOBAL_011F4748, 4);
        e.set_global(GLOBAL_011F4748, listener);
        let this = e.mem.alloc(0x40);
        let last = ref_object(&mut e, 1);
        let shared = ref_object(&mut e, 2);
        e.mem.set_u32(this + 0x0c, last);
        e.mem.set_u32(this + 0x08, shared);
        e.mem.set_u32(this + 0x14, 0xabc0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_5580, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_NI_SKIN_INSTANCE);
        assert_eq!(calls_to(&e, FREE_00AA10F0), vec![vec![0xabc0]]);
        assert_eq!(calls_to(&e, 0x7000_0104), vec![vec![listener, this]]);
        assert_eq!(calls_to(&e, 0x7000_0004), vec![vec![last]]);
        assert_eq!(e.mem.u32(shared + 4), 1);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR), vec![vec![this]]);

        // No listener, no references.
        e.set_global(GLOBAL_011F4748, 0u32);
        e.mem.set_u32(this + 0x0c, 0);
        e.mem.set_u32(this + 0x08, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_5580, &args![this]);
        assert!(calls_to(&e, 0x7000_0104).is_empty());
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR).len(), 1);
    }
}

// Translated from 00a86280 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `qsort` comparison of two string pointers (cdecl; passed to `qsort`,
/// `00ec6f20`, by `GetViewerStrings`; Ghidra made no function of it): 1
/// when `first` is null, -1 when `second` is, otherwise the inlined
/// `strcmp` of `*first` and `*second` (0 when equal, else -1 or 1 by the
/// first differing byte, unsigned).
pub fn fn_00a86280(e: &mut Engine, first: u32, second: u32) -> i32 {
    if first == 0 {
        return 1;
    }
    if second == 0 {
        return -1;
    }
    let mut right = e.mem.u32(second);
    let mut left = e.mem.u32(first);
    loop {
        let a = e.mem.u8(left);
        let b = e.mem.u8(right);
        if a != b {
            return if a < b { -1 } else { 1 };
        }
        if a == 0 {
            return 0;
        }
        let a = e.mem.u8(left + 1);
        let b = e.mem.u8(right + 1);
        if a != b {
            return if a < b { -1 } else { 1 };
        }
        left = left.wrapping_add(2);
        right = right.wrapping_add(2);
        if a == 0 {
            return 0;
        }
    }
}

#[cfg(test)]
mod n079_tests {
    use super::*;

    #[test]
    fn strings_compare_like_strcmp() {
        let mut e = Engine::new();
        let text = |e: &mut Engine, s: &[u8]| {
            let p = e.mem.alloc(s.len() as u32 + 1);
            e.mem.write(p, s);
            let holder = e.mem.alloc(4);
            e.mem.set_u32(holder, p);
            holder
        };
        let abc = text(&mut e, b"abc");
        let abd = text(&mut e, b"abd");
        let ab = text(&mut e, b"ab");
        let high = text(&mut e, &[0xe9]);
        let compare = |e: &mut Engine, a: u32, b: u32| e.call(0x00a8_6280, &[a, b]).i32();
        assert_eq!(compare(&mut e, abc, abc), 0);
        assert_eq!(compare(&mut e, abc, abd), -1);
        assert_eq!(compare(&mut e, abd, abc), 1);
        assert_eq!(compare(&mut e, ab, abc), -1);
        assert_eq!(compare(&mut e, abc, ab), 1);
        assert_eq!(compare(&mut e, high, abc), 1);
        assert_eq!(compare(&mut e, 0, abc), 1);
        assert_eq!(compare(&mut e, abc, 0), -1);
        assert_eq!(compare(&mut e, 0, 0), 1);
    }
}

/// `NiTransform::InvertNonUniform` (`004b4880`, thiscall, one word: the out
/// transform).
const INVERT_NON_UNIFORM: u32 = 0x004b_4880;
/// `0062c250`: the `NiTransform` product (thiscall: out, right-hand side;
/// answers `out`).
const TRANSFORM_PRODUCT: u32 = 0x0062_c250;
/// `_memset` (`00ec61c0`, cdecl).
const MEMSET: u32 = 0x00ec_61c0;
/// `NiPoint3::UnitizeVectors` (`00a7e960`, cdecl: vectors, count, stride).
const UNITIZE_VECTORS: u32 = 0x00a7_e960;
/// The point (three floats) the outputs start from when their stride is not
/// 12 (`011f426c`, `011f4270`, `011f4274`).
const START_POINT_011F426C: u32 = 0x011f_426c;

/// `FLD dword` of the float at `addr`.
fn fld(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// A `float` memory operand at `addr` (`FMUL`/`FADD dword ptr`).
fn fmem(e: &Engine, addr: u32) -> f64 {
    x87_m32(e.mem.u32(addr))
}

/// One x87 row `((a0 * a1) + (b0 * b1)) + (c0 * c1)`, each pair as its `FMUL`
/// sees it (the stack value first).
fn row(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    x87_add(
        x87_add(x87_mul(a.0, a.1), x87_mul(b.0, b.1)),
        x87_mul(c.0, c.1),
    )
}

/// `count` points of `stride` bytes from each of `outputs` set to the start
/// point (`011f426c`), output by output within each step and each word read
/// again for every store.
fn fill_start_point(e: &mut Engine, outputs: &mut [u32], count: u32, stride: u32) {
    for _ in 0..count {
        for slot in outputs.iter_mut() {
            for word in 0..3 {
                let value = e.mem.u32(START_POINT_011F426C + word * 4);
                e.mem.set_u32(slot.wrapping_add(word * 4), value);
            }
            *slot = slot.wrapping_add(stride);
        }
    }
}

/// `v + out` stored back into each of the three floats at `out`
/// (`FLD v; FADD out; FSTP out`).
fn add_to_output(e: &mut Engine, out: u32, v: [u32; 3]) {
    for (i, value) in (0u32..).zip(v) {
        let at = out.wrapping_add(i * 4);
        let sum = x87_add(x87_load32(value), fmem(e, at));
        e.mem.set_u32(at, x87_store32(sum));
    }
}

/// The three stored floats `x87_mul(v, weight)` (`FLD v; FMUL ST1`, the
/// weight in `ST1`).
fn weighted(v: [u32; 3], weight: f64) -> [u32; 3] {
    v.map(|value| x87_store32(x87_mul(x87_load32(value), weight)))
}

/// The bone matrix as the loops use it: the frame holding it (rotation
/// `+0x5c`..`+0x7c` by rows, translation `+0x80`, scale `+0x8c`), the
/// rotation times the scale (nine stored floats) and the five rotation
/// entries the four- and two-output loops keep on the x87 stack (`m00`,
/// `m01`, `m02`, `m10`, `m11`); the other four they load from the frame
/// each time.
struct BoneMatrix {
    frame: u32,
    scaled: [u32; 9],
    stack: [f64; 5],
}

impl BoneMatrix {
    /// `FLD dword` of rotation entry `i` (row-major) from the frame.
    fn entry(&self, e: &Engine, i: u32) -> f64 {
        fld(e, self.frame + 0x5c + i * 4)
    }

    /// Scaled rotation entry `i` as an `FLD` loads it.
    fn scaled(&self, i: usize) -> f64 {
        x87_load32(self.scaled[i])
    }

    /// Scaled rotation entry `i` as a memory operand.
    fn scaled_operand(&self, i: usize) -> f64 {
        x87_m32(self.scaled[i])
    }

    /// Translation `i` as a memory operand (`FADD dword [+0x80 + 4i]`).
    fn translate(&self, e: &Engine, i: u32) -> f64 {
        fmem(e, self.frame + 0x80 + i * 4)
    }
}

/// The bone's matrix: `skin-to-root (frame +0xf4) * bone world (+0x68)`
/// into `outs.0`, that times the bone data entry's transform into `outs.1`,
/// the 13 words the second product answers copied to the frame at `+0x5c`;
/// then the scaled rotation. `positions_only`: the one-output loop
/// multiplies `m22` as a memory operand (`FMUL dword [+0x7c]`).
fn bone_matrix(
    e: &mut Engine,
    frame: u32,
    this: u32,
    bone: u32,
    entry: u32,
    outs: (u32, u32),
    positions_only: bool,
) -> BoneMatrix {
    let bones = e.mem.u32(this + 0x14); // NiSkinInstance::m_ppkBones (Xbox PDB) +0x14
    let node = e.mem.u32(bones.wrapping_add(bone.wrapping_mul(4)));
    let first = e
        .call(
            TRANSFORM_PRODUCT,
            &args![frame + 0xf4, outs.0, node.wrapping_add(0x68)],
        )
        .u32();
    let second = e
        .call(TRANSFORM_PRODUCT, &args![first, outs.1, entry])
        .u32();
    for word in 0..13 {
        let value = e.mem.u32(second.wrapping_add(word * 4));
        e.mem.set_u32(frame + 0x5c + word * 4, value);
    }
    let m: Vec<f64> = (0..9).map(|i| fld(e, frame + 0x5c + i * 4)).collect();
    let scale = fld(e, frame + 0x8c);
    let mut scaled = [0u32; 9];
    // m00: FLD m00; FLD s; FLD ST0; FMULP ST2. m01..m21: FMUL by the scale
    // in a lower register.
    for i in 0..8 {
        scaled[i] = x87_store32(x87_mul(m[i], scale));
    }
    scaled[8] = if positions_only {
        x87_store32(x87_mul(scale, fmem(e, frame + 0x7c)))
    } else {
        // FLD m22; FMULP ST5 (the scale): s * m22.
        x87_store32(x87_mul(scale, m[8]))
    };
    BoneMatrix {
        frame,
        scaled,
        stack: [m[0], m[1], m[2], m[3], m[4]],
    }
}

/// The scaled, rotated and translated point at `p` in the four- and
/// two-output loops (`FLD a01; FMUL y; FLD x; FMUL a00; FADDP; ..`), as
/// stored floats.
fn skinned_point(e: &Engine, m: &BoneMatrix, p: u32) -> [u32; 3] {
    let x = (fld(e, p), m.scaled_operand(0));
    let rows = [
        row(
            (m.scaled(1), fmem(e, p + 4)),
            x,
            (m.scaled(2), fmem(e, p + 8)),
        ),
        row(
            (m.scaled(4), fmem(e, p + 4)),
            (m.scaled(3), fmem(e, p)),
            (m.scaled(5), fmem(e, p + 8)),
        ),
        row(
            (m.scaled(7), fmem(e, p + 4)),
            (m.scaled(6), fmem(e, p)),
            (m.scaled(8), fmem(e, p + 8)),
        ),
    ];
    [
        x87_store32(x87_add(rows[0], m.translate(e, 0))),
        x87_store32(x87_add(rows[1], m.translate(e, 1))),
        x87_store32(x87_add(rows[2], m.translate(e, 2))),
    ]
}

/// The vector at `v` times the unscaled rotation, rows `m00*x + m01*y +
/// m02*z` and so on (normals and the third set), as stored floats.
fn rotated(e: &Engine, m: &BoneMatrix, v: u32) -> [u32; 3] {
    [
        x87_store32(row(
            (m.stack[0], fmem(e, v)),
            (m.stack[1], fmem(e, v + 4)),
            (m.stack[2], fmem(e, v + 8)),
        )),
        x87_store32(row(
            (m.stack[3], fmem(e, v)),
            (m.stack[4], fmem(e, v + 4)),
            (m.entry(e, 5), fmem(e, v + 8)),
        )),
        x87_store32(row(
            (m.entry(e, 6), fmem(e, v)),
            (m.entry(e, 7), fmem(e, v + 4)),
            (m.entry(e, 8), fmem(e, v + 8)),
        )),
    ]
}

/// The second vector set's rotation, whose rows the exe sums in another
/// order (`m01*y + x*m00 + m02*z`, `m11*y + m10*x + m12*z`, `m21*y +
/// m20*x + m22*z`).
fn rotated_second(e: &Engine, m: &BoneMatrix, v: u32) -> [u32; 3] {
    [
        x87_store32(row(
            (m.stack[1], fmem(e, v + 4)),
            (fld(e, v), m.stack[0]),
            (m.stack[2], fmem(e, v + 8)),
        )),
        x87_store32(row(
            (m.stack[4], fmem(e, v + 4)),
            (m.stack[3], fmem(e, v)),
            (m.entry(e, 5), fmem(e, v + 8)),
        )),
        x87_store32(row(
            (m.entry(e, 7), fmem(e, v + 4)),
            (m.entry(e, 6), fmem(e, v)),
            (m.entry(e, 8), fmem(e, v + 8)),
        )),
    ]
}

/// The weight of one `{u16 vertex, f32 weight}` entry after its
/// `FLD; FSTP` to the frame, and the vertex.
fn vertex_weight(e: &Engine, weights: u32, k: u32) -> (u32, u32) {
    let at = weights.wrapping_add(k.wrapping_mul(8));
    (
        u32::from(e.mem.u16(at)),
        x87_store32(fld(e, at.wrapping_add(4))),
    )
}

// Translated from 00a855f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Software skinning of `count` vertices (`this` has the `NiSkinInstance`
/// layout of the Xbox PDB: skin data `+0x08`, root parent `+0x10`, bones
/// `+0x14`; the map names no function here). The skin-to-root transform is
/// the skin data's transform at `+0x0c` times the inverse
/// (`NiTransform::InvertNonUniform`) of the root parent's world transform
/// (`+0x68`). For each bone data entry (skin data `+0x40`, count `+0x44`,
/// entries of `0x4c` bytes) the bone's matrix is that times the bone node's
/// world transform times the entry's own transform (`+0x00`), and each of
/// its weights (`+0x44` pointer, `+0x48` count of 8-byte `{u16 vertex, f32
/// weight}`) adds the weighted point (rotation times scale, plus
/// translation) to `out_positions` and the weighted rotated vectors
/// (rotation only) to the vector outputs, at `vertex * stride`.
///
/// The vector sources follow one another from `normals` (`count` points of
/// 12 bytes each: the first, second and third sets). With `normals` and
/// `out_normals` given and both `out_second` and `out_third`, all four
/// outputs are skinned and the three vector outputs unitized
/// (`NiPoint3::UnitizeVectors`); with only `normals` and `out_normals`, the
/// positions and normals (normals unitized); otherwise the positions only.
/// The outputs start at zero (`_memset`) when `stride` is 12, else at the
/// point at `011f426c`. The x87 arithmetic is the exe's, instruction by
/// instruction (operand order, which values are stored as floats and which
/// stay on the x87 stack).
pub fn fn_00a855f0(
    e: &mut Engine,
    this: Ptr,
    positions: u32,
    normals: u32,
    count: u32,
    out_positions: u32,
    out_normals: u32,
    out_second: u32,
    out_third: u32,
    stride: u32,
) {
    let this = this.addr();
    e.with_stack(0x190, |e, frame| {
        let frame = frame.addr();
        let root_parent = e.mem.u32(this + 0x10); // NiSkinInstance::m_pkRootParent (Xbox PDB) +0x10
        e.call(
            INVERT_NON_UNIFORM,
            &args![root_parent.wrapping_add(0x68), frame + 0xf4],
        );
        let skin_data = e.mem.u32(this + 0x08); // NiSkinInstance::m_spSkinData (Xbox PDB) +0x08
        let product = e
            .call(
                TRANSFORM_PRODUCT,
                &args![skin_data.wrapping_add(0x0c), frame + 0x5c, frame + 0xf4],
            )
            .u32();
        for word in 0..13 {
            let value = e.mem.u32(product.wrapping_add(word * 4));
            e.mem.set_u32(frame + 0xf4 + word * 4, value);
        }
        let skin_data = e.mem.u32(this + 0x08);
        let bone_data = e.mem.u32(skin_data.wrapping_add(0x40)); // NiSkinData::m_pkBoneData (PC +0x40)
        let bone_count = e.mem.u32(skin_data.wrapping_add(0x44)); // NiSkinData::m_uiBones (PC +0x44)
        let size = count.wrapping_mul(12);
        if normals != 0 && out_normals != 0 {
            if out_second != 0 && out_third != 0 {
                let second = normals.wrapping_add(size);
                let third = second.wrapping_add(size);
                let outputs = [out_positions, out_normals, out_second, out_third];
                if stride == 12 {
                    for out in outputs {
                        e.call(MEMSET, &args![out, 0u32, size]);
                    }
                } else {
                    let mut cursors = outputs;
                    fill_start_point(e, &mut cursors, count, stride);
                }
                for bone in 0..bone_count {
                    let entry = bone_data.wrapping_add(bone.wrapping_mul(0x4c));
                    let vertices = u32::from(e.mem.u16(entry.wrapping_add(0x48))); // BoneData::m_usVerts
                    let m = bone_matrix(
                        e,
                        frame,
                        this,
                        bone,
                        entry,
                        (frame + 0x15c, frame + 0x128),
                        false,
                    );
                    let weights = e.mem.u32(entry.wrapping_add(0x44)); // BoneData::m_pkBoneVertData
                    for k in 0..vertices {
                        let (vertex, weight) = vertex_weight(e, weights, k);
                        let weight = x87_load32(weight);
                        let dst = vertex.wrapping_mul(stride);
                        let src = vertex.wrapping_mul(12);
                        let point = skinned_point(e, &m, positions.wrapping_add(src));
                        let normal = rotated(e, &m, normals.wrapping_add(src));
                        let second_v = rotated_second(e, &m, second.wrapping_add(src));
                        let third_v = rotated(e, &m, third.wrapping_add(src));
                        add_to_output(e, out_positions.wrapping_add(dst), weighted(point, weight));
                        add_to_output(e, out_normals.wrapping_add(dst), weighted(normal, weight));
                        add_to_output(e, out_second.wrapping_add(dst), weighted(second_v, weight));
                        // The last product is `FMUL dword` with the weight on
                        // the stack; the sums load the output first for x.
                        let v = [
                            x87_store32(x87_mul(x87_load32(third_v[0]), weight)),
                            x87_store32(x87_mul(x87_load32(third_v[1]), weight)),
                            x87_store32(x87_mul(weight, x87_m32(third_v[2]))),
                        ];
                        let out = out_third.wrapping_add(dst);
                        let x = x87_add(fld(e, out), x87_m32(v[0]));
                        e.mem.set_u32(out, x87_store32(x));
                        let y = x87_add(x87_load32(v[1]), fmem(e, out.wrapping_add(4)));
                        e.mem.set_u32(out.wrapping_add(4), x87_store32(y));
                        let z = x87_add(x87_load32(v[2]), fmem(e, out.wrapping_add(8)));
                        e.mem.set_u32(out.wrapping_add(8), x87_store32(z));
                    }
                }
                for out in [out_normals, out_second, out_third] {
                    e.call(UNITIZE_VECTORS, &args![out, count, stride]);
                }
                return;
            }
            if stride == 12 {
                e.call(MEMSET, &args![out_positions, 0u32, size]);
                e.call(MEMSET, &args![out_normals, 0u32, size]);
            } else {
                fill_start_point(e, &mut [out_positions, out_normals], count, stride);
            }
            for bone in 0..bone_count {
                let entry = bone_data.wrapping_add(bone.wrapping_mul(0x4c));
                let vertices = u32::from(e.mem.u16(entry.wrapping_add(0x48)));
                let m = bone_matrix(
                    e,
                    frame,
                    this,
                    bone,
                    entry,
                    (frame + 0x128, frame + 0x15c),
                    false,
                );
                let weights = e.mem.u32(entry.wrapping_add(0x44));
                for k in 0..vertices {
                    let (vertex, weight) = vertex_weight(e, weights, k);
                    let weight = x87_load32(weight);
                    let dst = vertex.wrapping_mul(stride);
                    let src = vertex.wrapping_mul(12);
                    let point = skinned_point(e, &m, positions.wrapping_add(src));
                    let normal = rotated(e, &m, normals.wrapping_add(src));
                    add_to_output(e, out_positions.wrapping_add(dst), weighted(point, weight));
                    // The last product is `FMUL dword` with the weight on the
                    // stack.
                    let v = [
                        x87_store32(x87_mul(x87_load32(normal[0]), weight)),
                        x87_store32(x87_mul(x87_load32(normal[1]), weight)),
                        x87_store32(x87_mul(weight, x87_m32(normal[2]))),
                    ];
                    add_to_output(e, out_normals.wrapping_add(dst), v);
                }
            }
            e.call(UNITIZE_VECTORS, &args![out_normals, count, stride]);
            return;
        }
        if stride == 12 {
            e.call(MEMSET, &args![out_positions, 0u32, size]);
        } else {
            fill_start_point(e, &mut [out_positions], count, stride);
        }
        for bone in 0..bone_count {
            let entry = bone_data.wrapping_add(bone.wrapping_mul(0x4c));
            let vertices = u32::from(e.mem.u16(entry.wrapping_add(0x48)));
            let m = bone_matrix(
                e,
                frame,
                this,
                bone,
                entry,
                (frame + 0x128, frame + 0x15c),
                true,
            );
            let weights = e.mem.u32(entry.wrapping_add(0x44));
            for k in 0..vertices {
                let (vertex, weight) = vertex_weight(e, weights, k);
                let weight = x87_m32(weight);
                let dst = out_positions.wrapping_add(vertex.wrapping_mul(stride));
                let p = positions.wrapping_add(vertex.wrapping_mul(12));
                // FLD x; FMUL a00; FLD a01; FMUL y; FADDP: x first here.
                let r0 = x87_store32(x87_add(
                    row(
                        (fld(e, p), m.scaled_operand(0)),
                        (m.scaled(1), fmem(e, p + 4)),
                        (m.scaled(2), fmem(e, p + 8)),
                    ),
                    m.translate(e, 0),
                ));
                let r1 = x87_store32(x87_add(
                    row(
                        (m.scaled(4), fmem(e, p + 4)),
                        (m.scaled(3), fmem(e, p)),
                        (m.scaled(5), fmem(e, p + 8)),
                    ),
                    m.translate(e, 1),
                ));
                let r2 = x87_store32(x87_add(
                    row(
                        (m.scaled(7), fmem(e, p + 4)),
                        (m.scaled(6), fmem(e, p)),
                        (m.scaled(8), fmem(e, p + 8)),
                    ),
                    m.translate(e, 2),
                ));
                // The weighted values stay on the x87 stack (not stored),
                // and y and z of the output are read before x is stored.
                let x = x87_mul(x87_load32(r0), weight);
                let y = x87_mul(x87_load32(r1), weight);
                let x = x87_add(x, fmem(e, dst));
                let z = x87_mul(x87_load32(r2), weight);
                let y = x87_add(y, fmem(e, dst.wrapping_add(4)));
                e.mem.set_u32(dst, x87_store32(x));
                let z = x87_add(z, fmem(e, dst.wrapping_add(8)));
                e.mem.set_u32(dst.wrapping_add(4), x87_store32(y));
                e.mem.set_u32(dst.wrapping_add(8), x87_store32(z));
            }
        }
    });
}

#[cfg(test)]
mod n086_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The bone matrix every product answers: rotation rows (1 2 0) (0 1 0)
    /// (0 0 1), translation (1 2 3), scale 2.
    const MATRIX: [f32; 13] = [
        1.0, 2.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 2.0,
    ];

    struct Skin {
        this: u32,
        skin_data: u32,
        node: u32,
        entry: u32,
        root: u32,
        positions: u32,
        normals: u32,
    }

    fn floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in (0u32..).zip(values) {
            e.mem.set_f32(at + i * 4, *v);
        }
    }

    fn read3(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn setup(e: &mut Engine) -> Skin {
        e.register(INVERT_NON_UNIFORM, |_, _| Ret::default());
        e.register(TRANSFORM_PRODUCT, |e, a| {
            for (i, v) in (0u32..).zip(MATRIX) {
                e.mem.set_f32(a[1] + i * 4, v);
            }
            a[1].into_ret()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        e.register(UNITIZE_VECTORS, |_, _| Ret::default());
        e.map(START_POINT_011F426C, 12);
        floats(e, START_POINT_011F426C, &[10.0, 20.0, 30.0]);
        let this = e.mem.alloc(0x34);
        let skin_data = e.mem.alloc(0x60);
        let root = e.mem.alloc(0xa0);
        let node = e.mem.alloc(0xa0);
        let bones = e.mem.alloc(4);
        let entry = e.mem.alloc(0x4c);
        let weights = e.mem.alloc(16);
        e.mem.set_u32(this + 0x08, skin_data);
        e.mem.set_u32(this + 0x10, root);
        e.mem.set_u32(this + 0x14, bones);
        e.mem.set_u32(bones, node);
        e.mem.set_u32(skin_data + 0x40, entry);
        e.mem.set_u32(skin_data + 0x44, 1);
        e.mem.set_u32(entry + 0x44, weights);
        e.mem.set_u16(entry + 0x48, 2);
        e.mem.set_u16(weights, 1);
        e.mem.set_f32(weights + 4, 0.5);
        e.mem.set_u16(weights + 8, 0);
        e.mem.set_f32(weights + 12, 1.0);
        let positions = e.mem.alloc(24);
        floats(e, positions, &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
        // Two vertices of each of the three vector sets, one after another.
        let normals = e.mem.alloc(72);
        floats(
            e,
            normals,
            &[
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0,
            ],
        );
        Skin {
            this,
            skin_data,
            node,
            entry,
            root,
            positions,
            normals,
        }
    }

    fn output(e: &mut Engine, stride: u32) -> u32 {
        let out = e.mem.alloc(2 * stride);
        for i in 0..stride / 2 {
            e.mem.set_f32(out + i * 4, 99.0);
        }
        out
    }

    fn deform(e: &mut Engine, s: &Skin, normals: u32, outs: [u32; 4], stride: u32) {
        e.call_log = Some(vec![]);
        e.call(
            0x00a8_55f0,
            &args![
                s.this,
                s.positions,
                normals,
                2u32,
                outs[0],
                outs[1],
                outs[2],
                outs[3],
                stride
            ],
        );
    }

    #[test]
    fn deform_skins_all_four_outputs() {
        let mut e = Engine::new();
        let s = setup(&mut e);
        let outs = [
            output(&mut e, 12),
            output(&mut e, 12),
            output(&mut e, 12),
            output(&mut e, 12),
        ];
        deform(&mut e, &s, s.normals, outs, 12);
        assert_eq!(calls_to(&e, INVERT_NON_UNIFORM)[0][0], s.root + 0x68);
        let products = calls_to(&e, TRANSFORM_PRODUCT);
        assert_eq!(products.len(), 3);
        assert_eq!(products[0][0], s.skin_data + 0x0c);
        assert_eq!(products[1][2], s.node + 0x68);
        // The second product starts from what the first answered.
        assert_eq!(products[2][0], products[1][1]);
        assert_eq!(products[2][2], s.entry);
        assert_eq!(calls_to(&e, MEMSET).len(), 4);
        // Vertex 1 (weight 0.5): the point (1 1 1) is (7 4 5) in the bone.
        assert_eq!(read3(&e, outs[0] + 12), [3.5, 2.0, 2.5]);
        // Vertex 0 (weight 1, at the origin): the translation.
        assert_eq!(read3(&e, outs[0]), [1.0, 2.0, 3.0]);
        assert_eq!(read3(&e, outs[1] + 12), [0.0, 0.0, 0.5]);
        assert_eq!(read3(&e, outs[2] + 12), [0.5, 0.0, 0.0]);
        assert_eq!(read3(&e, outs[3] + 12), [1.0, 0.5, 0.0]);
        let unitized = calls_to(&e, UNITIZE_VECTORS);
        assert_eq!(
            unitized,
            vec![
                vec![outs[1], 2, 12],
                vec![outs[2], 2, 12],
                vec![outs[3], 2, 12]
            ]
        );

        // Another stride starts from the point at 011f426c.
        let outs = [
            output(&mut e, 16),
            output(&mut e, 16),
            output(&mut e, 16),
            output(&mut e, 16),
        ];
        deform(&mut e, &s, s.normals, outs, 16);
        assert!(calls_to(&e, MEMSET).is_empty());
        assert_eq!(read3(&e, outs[0] + 16), [13.5, 22.0, 32.5]);
        assert_eq!(read3(&e, outs[0]), [11.0, 22.0, 33.0]);
        assert_eq!(read3(&e, outs[3] + 16), [11.0, 20.5, 30.0]);
        assert_eq!(read3(&e, outs[2]), [10.0, 20.0, 30.0]);
    }

    #[test]
    fn deform_skins_positions_and_normals() {
        let mut e = Engine::new();
        let s = setup(&mut e);
        let outs = [output(&mut e, 12), output(&mut e, 12), 0, 0];
        deform(&mut e, &s, s.normals, outs, 12);
        assert_eq!(calls_to(&e, MEMSET).len(), 2);
        assert_eq!(read3(&e, outs[0] + 12), [3.5, 2.0, 2.5]);
        assert_eq!(read3(&e, outs[1] + 12), [0.0, 0.0, 0.5]);
        assert_eq!(read3(&e, outs[1]), [0.0, 0.0, 0.0]);
        assert_eq!(calls_to(&e, UNITIZE_VECTORS), vec![vec![outs[1], 2, 12]]);

        // A third output alone does not make the four-output case.
        let outs = [
            output(&mut e, 16),
            output(&mut e, 16),
            output(&mut e, 16),
            0,
        ];
        deform(&mut e, &s, s.normals, outs, 16);
        assert_eq!(read3(&e, outs[1] + 16), [10.0, 20.0, 30.5]);
        assert_eq!(read3(&e, outs[2]), [99.0, 99.0, 99.0]);
        assert_eq!(calls_to(&e, UNITIZE_VECTORS).len(), 1);
    }

    #[test]
    fn deform_skins_positions_only() {
        let mut e = Engine::new();
        let s = setup(&mut e);
        let outs = [output(&mut e, 12), output(&mut e, 12), 0, 0];
        deform(&mut e, &s, 0, outs, 12);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![outs[0], 0, 24]]);
        assert_eq!(read3(&e, outs[0] + 12), [3.5, 2.0, 2.5]);
        assert_eq!(read3(&e, outs[0]), [1.0, 2.0, 3.0]);
        assert_eq!(read3(&e, outs[1]), [99.0, 99.0, 99.0]);
        assert!(calls_to(&e, UNITIZE_VECTORS).is_empty());

        // No normal output: positions only too, from the start point.
        let outs = [output(&mut e, 16), 0, 0, 0];
        deform(&mut e, &s, s.normals, outs, 16);
        assert_eq!(read3(&e, outs[0] + 16), [13.5, 22.0, 32.5]);
        assert_eq!(read3(&e, outs[0]), [11.0, 22.0, 33.0]);
    }
}
