//! `NiMain/nibspnode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSPNode::ms_RTTI` (the class's `NiRTTI` record).
const NI_BSP_NODE_RTTI: u32 = 0x011f_5f54;

// Translated from 00a9adc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bsp_node_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BSP_NODE_RTTI)
}

// Translated from 00a9add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a5e930`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xcc bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_bsp_node_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_e930;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xccu32]);
    }
    this
}

// The node's model plane (`NiPlane`: normal +0xac, +0xb0, +0xb4, constant +0xb8) and world
// plane (normal +0xbc, +0xc0, +0xc4, constant +0xc8), the `NiAVObject` local rotation (+0x34),
// translation (+0x58) and scale (+0x64) and world rotation (+0x68), translation (+0x8c) and
// scale (+0x98), and the children array (`m_kChildren`, base +0xa0, count +0xa6), as the
// functions below use them.
const MODEL_PLANE: u32 = 0xac;
const WORLD_PLANE: u32 = 0xbc;

/// `NiNode::LoadBinary` (`00a5dd40`), `SaveBinary` (`00a5e880`), `IsEqual` (`00a5e600`),
/// `GetViewerStrings` (`00a5ea70`), its constructor (`00a5ecb0`, one word) and member copy
/// (`00a5dc80`, (clone, cloning process)), all `thiscall`.
const NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
const NODE_SAVE_BINARY: u32 = 0x00a5_e880;
const NODE_IS_EQUAL: u32 = 0x00a5_e600;
const NODE_GET_VIEWER_STRINGS: u32 = 0x00a5_ea70;
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NODE_COPY_MEMBERS: u32 = 0x00a5_dc80;
/// The plane's `LoadBinary` (`00a69ac0`) and `SaveBinary` (`00eb5410`; the engine map names
/// both `NiSphere`, folded code), `NiPlane::NiPlane` (`00a69940`), `NiPlane::GetViewerString`
/// (`00a69a60`, the name), the plane comparison `00a926b0` (the other plane; the node answers
/// its negation) and the side test `0049da80` (the point; 0, 1 or 2), all `thiscall` on the
/// plane.
const PLANE_LOAD_BINARY: u32 = 0x00a6_9ac0;
const PLANE_SAVE_BINARY: u32 = 0x00eb_5410;
const PLANE_CONSTRUCT: u32 = 0x00a6_9940;
const PLANE_GET_VIEWER_STRING: u32 = 0x00a6_9a60;
const PLANE_COMPARE: u32 = 0x00a9_26b0;
const PLANE_WHICH_SIDE: u32 = 0x0049_da80;
/// `NiAVObject::UpdateWorldData` (`00a68c60`, one word) and `NiAVObject::Cull` (`00a59e00`,
/// the culling process).
const AV_OBJECT_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
const AV_OBJECT_CULL: u32 = 0x00a5_9e00;
/// `NiMatrix3::operator*` (`0043f8d0`, `thiscall`, (result, other), answers the result),
/// `NiMatrix3::TransposeTimes` (`00a588a0`, `thiscall`, (result, other)) and
/// `NiMatrix3::Inverse_ov2` (`004b46a0`, `thiscall`, the result).
const MATRIX_TIMES: u32 = 0x0043_f8d0;
const MATRIX_TRANSPOSE_TIMES: u32 = 0x00a5_88a0;
const MATRIX_INVERSE: u32 = 0x004b_46a0;
/// `00aa13e0` (`cdecl`, the size): the object allocation; `0060bef0` (`thiscall` on the
/// children array, one word): sizes it.
const NI_NEW: u32 = 0x00aa_13e0;
const ARRAY_SIZE_TO: u32 = 0x0060_bef0;
/// `NiBSPNode`'s vtable (the one its clone gets).
const BSP_NODE_VTABLE: u32 = 0x010a_1024;
/// `NiGetViewerString` of a class name (`00a7d330`); the plane names `0109fee4` and
/// `010a100c`.
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NAME_MODEL_PLANE: u32 = 0x0109_fee4;
const NAME_WORLD_PLANE: u32 = 0x010a_100c;
/// `0096ad30` grows an `NiTArray`; `0096ae90` stores an element (both `thiscall`).
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;

/// `FLD dword` of the float at `at`.
fn fld(e: &Engine, at: u32) -> f64 {
    x87_load32(e.mem.u32(at))
}

/// A `float` memory operand at `at`.
fn mem32(e: &Engine, at: u32) -> f64 {
    x87_m32(e.mem.u32(at))
}

/// `FSTP dword` of `value` to `at`.
fn fstp(e: &mut Engine, at: u32, value: f64) {
    e.mem.set_u32(at, x87_store32(value));
}

/// `value` through a `float` stack slot (`FSTP dword` then `FLD dword`).
fn through_float(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

// Translated from 00a9a710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::LoadBinary` (Xbox PDB): `NiNode::LoadBinary`, then the model plane (+0xac,
/// `00a69ac0`).
pub fn ni_bsp_node_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NODE_LOAD_BINARY, &args![this, stream]);
    e.call(PLANE_LOAD_BINARY, &args![this.addr() + MODEL_PLANE, stream]);
}

// Translated from 00a9a740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::SaveBinary` (Xbox PDB): `NiNode::SaveBinary`, then the model plane (+0xac,
/// `00eb5410`).
pub fn ni_bsp_node_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NODE_SAVE_BINARY, &args![this, stream]);
    e.call(PLANE_SAVE_BINARY, &args![this.addr() + MODEL_PLANE, stream]);
}

// Translated from 00a9a760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::UpdateWorldPlane` (Xbox PDB): the world plane from the model plane: its normal
/// is the world rotation (+0x68) times the model normal (each row summed in the exe's order,
/// each component a `float`), its constant the world translation (+0x8c) dotted with the new
/// normal plus the model constant times the world scale (+0x98), rounded to a `float` before
/// the sum and after. x87 arithmetic, one helper per instruction.
pub fn ni_bsp_node_update_world_plane(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let row = |e: &Engine, first: u32, second: u32, third: u32, n0: u32, n1: u32, n2: u32| {
        let a = x87_mul(fld(e, t + first), mem32(e, t + n0));
        let b = x87_mul(fld(e, t + second), mem32(e, t + n1));
        let sum = x87_add(a, b);
        x87_add(sum, x87_mul(fld(e, t + third), mem32(e, t + n2)))
    };
    // Row 0 starts with the product of the second element: `m01 * ny + nx * m00`.
    let a = x87_mul(fld(e, t + 0x6c), mem32(e, t + 0xb0));
    let b = x87_mul(fld(e, t + 0xac), mem32(e, t + 0x68));
    let sum = x87_add(a, b);
    let w0 = x87_store32(x87_add(sum, x87_mul(fld(e, t + 0x70), mem32(e, t + 0xb4))));
    let w1 = x87_store32(row(e, 0x74, 0x78, 0x7c, 0xac, 0xb0, 0xb4));
    let w2 = x87_store32(row(e, 0x80, 0x84, 0x88, 0xac, 0xb0, 0xb4));
    let p = x87_mul(fld(e, t + 0x90), x87_m32(w1));
    let q = x87_mul(fld(e, t + 0x8c), x87_m32(w0));
    let sum = x87_add(p, q);
    e.mem.set_u32(t + 0xbc, w0);
    let r = x87_mul(fld(e, t + 0x94), x87_m32(w2));
    e.mem.set_u32(t + 0xc0, w1);
    e.mem.set_u32(t + 0xc4, w2);
    let distance = through_float(x87_add(sum, r));
    let constant = x87_mul(fld(e, t + 0xb8), mem32(e, t + 0x98));
    let total = through_float(x87_add(distance, constant));
    fstp(e, t + 0xc8, total);
}

// Translated from 00a9a840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::UpdateWorldData` (Xbox PDB): `NiAVObject::UpdateWorldData`, then
/// [`ni_bsp_node_update_world_plane`].
pub fn ni_bsp_node_update_world_data(e: &mut Engine, this: Ptr, word: u32) {
    e.call(AV_OBJECT_UPDATE_WORLD_DATA, &args![this, word]);
    ni_bsp_node_update_world_plane(e, this);
}

/// The model normal turned by the inverse matrix at `inverse` (`n'_j = sum_i N_ij n_i`, in the
/// exe's operand orders: `first_swapped` puts the normal first in the first product of the
/// second and third components) and stored, then the constant moved by `translate` (three
/// floats at `point`): `n' . point` rounded to a `float`, plus the old constant.
fn transform_plane(e: &mut Engine, t: u32, inverse: u32, point: u32, first_swapped: bool) {
    let n = |i: u32| t + MODEL_PLANE + 4 * i;
    let m = |i: u32, j: u32| inverse + 4 * (3 * i + j);
    let a = x87_mul(fld(e, m(1, 0)), mem32(e, n(1)));
    let b = x87_mul(fld(e, n(0)), mem32(e, m(0, 0)));
    let sum = x87_add(a, b);
    let n0 = x87_store32(x87_add(sum, x87_mul(fld(e, m(2, 0)), mem32(e, n(2)))));
    let first = |e: &Engine, j: u32| {
        if first_swapped {
            x87_mul(fld(e, n(0)), mem32(e, m(0, j)))
        } else {
            x87_mul(fld(e, m(0, j)), mem32(e, n(0)))
        }
    };
    let a = first(e, 1);
    let b = x87_mul(fld(e, m(1, 1)), mem32(e, n(1)));
    let sum = x87_add(a, b);
    let n1 = x87_store32(x87_add(sum, x87_mul(fld(e, m(2, 1)), mem32(e, n(2)))));
    let a = first(e, 2);
    let b = x87_mul(fld(e, m(1, 2)), mem32(e, n(1)));
    let sum = x87_add(a, b);
    let last = x87_mul(fld(e, m(2, 2)), mem32(e, n(2)));
    e.mem.set_u32(t + 0xac, n0);
    e.mem.set_u32(t + 0xb0, n1);
    let n2 = x87_store32(x87_add(sum, last));
    e.mem.set_u32(t + 0xb4, n2);
    let a = x87_mul(fld(e, t + 0xb0), mem32(e, point + 4));
    let b = x87_mul(fld(e, t + 0xac), mem32(e, point));
    let sum = x87_add(a, b);
    let distance = x87_add(sum, x87_mul(fld(e, t + 0xb4), mem32(e, point + 8)));
    let distance = through_float(distance);
    let constant = x87_add(distance, mem32(e, t + 0xb8));
    fstp(e, t + 0xb8, constant);
}

/// Each child (base +0xa0, count +0xa6 read again each step) gets `ApplyTransform` (its slot
/// +0x90) with the matrix, the translation and 1.
fn apply_to_children(e: &mut Engine, t: u32, matrix: u32, translate: u32) {
    let mut index = 0u32;
    if e.mem.u16(t + 0xa6) == 0 {
        return;
    }
    loop {
        let child = e.mem.u32(e.mem.u32(t + 0xa0) + 4 * index);
        if child != 0 {
            e.vcall(child, 0x90, &args![matrix, translate, 1u32]);
        }
        index += 1;
        if index >= u32::from(e.mem.u16(t + 0xa6)) {
            break;
        }
    }
}

// Translated from 00a9a860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::ApplyTransform` (Xbox PDB; `thiscall`, `RET 0xc`: a matrix, a translation, a
/// flag byte). Without the flag the model plane is moved by the matrix's inverse and the
/// translation and every child gets the same transform. With it the transform is first taken
/// into the node's local frame: `P = R^T (M R)` (`operator*`, `TransposeTimes`, `R` the local
/// rotation at +0x34), the local translation `L` (+0x58) becomes `d = (M L + T) - L`, then
/// `R^T d` scaled by `1 / s` (the local scale +0x64) is the translation `k`; the plane is moved
/// by `P`'s inverse and `k`, and the children get `P`, `k` and 1. x87 arithmetic, one helper
/// per instruction in the exe's order.
pub fn ni_bsp_node_apply_transform(
    e: &mut Engine,
    this: Ptr,
    matrix: Ptr,
    translate: Ptr,
    flag: u32,
) {
    let t = this.addr();
    let mm = matrix.addr();
    let tt = translate.addr();
    e.with_stack(0x6c, |e, frame| {
        // The frame: the translation k (+0), the scratch floats (+0xc..+0x20), the first
        // matrix (+0x24, later the inverse) and the second (+0x48).
        let f = frame.addr();
        let inverse = f + 0x24;
        if flag as u8 == 0 {
            e.call(MATRIX_INVERSE, &args![matrix, inverse]);
            transform_plane(e, t, inverse, tt, true);
            apply_to_children(e, t, mm, tt);
            return;
        }
        let local = t + 0x34;
        let product = e.call(MATRIX_TIMES, &args![matrix, f + 0x24, local]).u32();
        e.call(MATRIX_TRANSPOSE_TIMES, &args![local, f + 0x48, product]);
        // v = M L, the first product of each row in the exe's order.
        let a = x87_mul(fld(e, mm + 4), mem32(e, t + 0x5c));
        let b = x87_mul(fld(e, mm), mem32(e, t + 0x58));
        let sum = x87_add(a, b);
        fstp(
            e,
            f + 0xc,
            x87_add(sum, x87_mul(fld(e, mm + 8), mem32(e, t + 0x60))),
        );
        for (row, out) in [(1u32, f + 0x10), (2, f + 0x14)] {
            let a = x87_mul(fld(e, mm + 0xc * row), mem32(e, t + 0x58));
            let b = x87_mul(fld(e, mm + 0xc * row + 4), mem32(e, t + 0x5c));
            let sum = x87_add(a, b);
            let value = x87_add(sum, x87_mul(fld(e, mm + 0xc * row + 8), mem32(e, t + 0x60)));
            fstp(e, out, value);
        }
        for i in 0..3 {
            let value = x87_add(fld(e, tt + 4 * i), mem32(e, f + 0xc + 4 * i));
            fstp(e, f + 0x18 + 4 * i, value);
        }
        for i in 0..3 {
            let value = x87_sub(fld(e, f + 0x18 + 4 * i), mem32(e, t + 0x58 + 4 * i));
            fstp(e, f + 0xc + 4 * i, value);
        }
        // R^T d with the x87 stack's operand orders.
        let r = |e: &Engine, i: u32, j: u32| fld(e, local + 4 * (3 * i + j));
        let d0 = fld(e, f + 0xc);
        let d1 = fld(e, f + 0x10);
        let d2 = fld(e, f + 0x14);
        let sum = x87_add(x87_mul(r(e, 0, 0), d0), x87_mul(r(e, 1, 0), d1));
        fstp(e, f + 0x18, x87_add(x87_mul(r(e, 2, 0), d2), sum));
        let sum = x87_add(x87_mul(r(e, 1, 1), d1), x87_mul(r(e, 0, 1), d0));
        fstp(e, f + 0x1c, x87_add(sum, x87_mul(r(e, 2, 1), d2)));
        let first = x87_mul(d1, r(e, 1, 2));
        let second = x87_mul(d0, r(e, 0, 2));
        let sum = x87_add(second, first);
        let last = x87_mul(d2, mem32(e, local + 0x20));
        fstp(e, f + 0x20, x87_add(sum, last));
        let inverse_scale = through_float(x87_div(1.0, fld(e, t + 0x64)));
        fstp(e, f, x87_mul(inverse_scale, mem32(e, f + 0x18)));
        fstp(e, f + 4, x87_mul(fld(e, f + 0x1c), inverse_scale));
        fstp(e, f + 8, x87_mul(inverse_scale, mem32(e, f + 0x20)));
        e.call(MATRIX_INVERSE, &args![f + 0x48, inverse]);
        transform_plane(e, t, inverse, f, false);
        apply_to_children(e, t, f + 0x48, f);
    });
}

// Translated from 00a9ac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::OnVisible` (Xbox PDB): the side of the world plane (+0xbc, `0049da80`) the
/// culling process's camera (+0xc) is on (its world translation +0x8c); side 0 asks again
/// with the camera's world direction (column 0 of its world rotation, copied to a local).
/// Then both children cull with the process (`NiAVObject::Cull`), the second child first on
/// side 2 and the first child first otherwise; null children are skipped.
pub fn ni_bsp_node_on_visible(e: &mut Engine, this: Ptr, culler: Ptr) {
    let camera = e.mem.u32(culler.addr() + 0xc);
    let plane = this.addr() + WORLD_PLANE;
    let mut side = e.call(PLANE_WHICH_SIDE, &args![plane, camera + 0x8c]).u32();
    if side == 0 {
        side = e.with_stack(0xc, |e, direction| {
            for (index, offset) in [0x68u32, 0x74, 0x80].into_iter().enumerate() {
                let value = x87_quiet_bits(e.mem.u32(camera + offset));
                e.mem.set_u32(direction.addr() + 4 * index as u32, value);
            }
            e.call(PLANE_WHICH_SIDE, &args![plane, direction]).u32()
        });
    }
    let children = e.mem.u32(this.addr() + 0xa0);
    let first = e.mem.u32(children);
    let second = e.mem.u32(children + 4);
    let order = if side == 2 {
        [second, first]
    } else {
        [first, second]
    };
    for child in order {
        if child != 0 {
            e.call(AV_OBJECT_CULL, &args![child, culler]);
        }
    }
}

// Translated from 00a9acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::IsEqual` (Xbox PDB): `NiNode::IsEqual`, then the negation of the model planes'
/// comparison (`00a926b0`).
pub fn ni_bsp_node_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NODE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    !e.call(
        PLANE_COMPARE,
        &args![this.addr() + MODEL_PLANE, other.addr() + MODEL_PLANE],
    )
    .bool()
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array (grown to
/// count + `m_usGrowBy` by `0096ad30` when full, stored by `0096ae90`).
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

// Translated from 00a9acf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::GetViewerStrings` (Xbox PDB): `NiNode::GetViewerStrings`, the class name (first
/// word of the `NiRTTI` at `011f5f54`), then the model plane (`0109fee4`) and the world plane
/// (`010a100c`) through `NiPlane::GetViewerString`, each kept in the `strings` argument's stack
/// word and appended.
pub fn ni_bsp_node_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(NI_BSP_NODE_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot.addr());
        for (plane, name) in [
            (MODEL_PLANE, NAME_MODEL_PLANE),
            (WORLD_PLANE, NAME_WORLD_PLANE),
        ] {
            let text = e
                .call(PLANE_GET_VIEWER_STRING, &args![this.addr() + plane, name])
                .u32();
            e.mem.set_u32(slot.addr(), text);
            append_viewer_string(e, strings, slot.addr());
        }
    });
}

// Translated from 00a9ae00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSPNode::CreateClone` (Xbox PDB): a 0xcc-byte node from `00aa13e0` (the `NiNode`
/// constructor with 0, the vtable `010a1024`, both planes constructed, the children array
/// sized to 2 by `0060bef0`, the word at +0xaa cleared; null when the allocation fails, the
/// stores below still going to it), `NiNode`'s member copy (`00a5dc80`), then this node's
/// model plane (four words). Answers the clone.
pub fn ni_bsp_node_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let block = e.call(NI_NEW, &args![0xccu32]).u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]);
        e.mem.set_u32(block, BSP_NODE_VTABLE);
        e.call(PLANE_CONSTRUCT, &args![block + MODEL_PLANE]);
        e.call(PLANE_CONSTRUCT, &args![block + WORLD_PLANE]);
        e.call(ARRAY_SIZE_TO, &args![block + 0x9c, 2u32]);
        e.mem.set_u16(block + 0xaa, 0);
        block
    };
    e.call(NODE_COPY_MEMBERS, &args![this, clone, cloning_process]);
    for word in 0..4 {
        let value = e.mem.u32(this.addr() + MODEL_PLANE + 4 * word);
        e.mem
            .set_u32(clone.wrapping_add(MODEL_PLANE + 4 * word), value);
    }
    Ptr::new(clone)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9adc0, ni_bsp_node_get_rtti() -> Ptr),
        entry!(0x00a9add0, ni_bsp_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a9_a710, ni_bsp_node_load_binary(Ptr, Ptr)),
        entry!(0x00a9_a740, ni_bsp_node_save_binary(Ptr, Ptr)),
        entry!(0x00a9_a760, ni_bsp_node_update_world_plane(Ptr)),
        entry!(0x00a9_a840, ni_bsp_node_update_world_data(Ptr, u32)),
        entry!(0x00a9_a860, ni_bsp_node_apply_transform(Ptr, Ptr, Ptr, u32)),
        entry!(0x00a9_ac10, ni_bsp_node_on_visible(Ptr, Ptr)),
        entry!(0x00a9_acb0, ni_bsp_node_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_acf0, ni_bsp_node_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a9_ae00, ni_bsp_node_create_clone(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bsp_node_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9adc0, &args![]).u32(), NI_BSP_NODE_RTTI);
    }

    #[test]
    fn ni_bsp_node_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_e930, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a9add0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a9add0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xcc]]);
        assert_eq!(calls_to(&e, 0x00a5_e930), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    fn set_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// A quarter turn about z, row-major.
    const TURN: [f32; 9] = [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    const IDENTITY: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

    #[test]
    fn load_and_save_pass_the_model_plane() {
        let mut e = Engine::new();
        for double in [
            NODE_LOAD_BINARY,
            NODE_SAVE_BINARY,
            PLANE_LOAD_BINARY,
            PLANE_SAVE_BINARY,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00a9_a710, &args![0x4000u32, 0x55u32]);
        e.call(0x00a9_a740, &args![0x4000u32, 0x55u32]);
        assert_eq!(calls_to(&e, NODE_LOAD_BINARY), vec![vec![0x4000, 0x55]]);
        assert_eq!(calls_to(&e, PLANE_LOAD_BINARY), vec![vec![0x40ac, 0x55]]);
        assert_eq!(calls_to(&e, NODE_SAVE_BINARY), vec![vec![0x4000, 0x55]]);
        assert_eq!(calls_to(&e, PLANE_SAVE_BINARY), vec![vec![0x40ac, 0x55]]);
    }

    #[test]
    fn world_plane_follows_the_world_transform() {
        let mut e = Engine::new();
        e.register(AV_OBJECT_UPDATE_WORLD_DATA, |_, _| Ret::default());
        let this = e.mem.alloc(0xcc);
        set_floats(&mut e, this + 0x68, &TURN);
        set_floats(&mut e, this + 0x8c, &[1.0, 2.0, 3.0, 2.0]);
        set_floats(&mut e, this + 0xac, &[1.0, 0.0, 0.0, 0.5]);
        e.call(0x00a9_a760, &args![this]);
        assert_eq!(floats(&e, this + 0xbc, 4), vec![0.0, 1.0, 0.0, 3.0]);
        // Through UpdateWorldData.
        set_floats(&mut e, this + 0xac, &[0.0, 1.0, 0.0, 1.0]);
        e.call_log = Some(vec![]);
        e.call(0x00a9_a840, &args![this, 9u32]);
        assert_eq!(
            calls_to(&e, AV_OBJECT_UPDATE_WORLD_DATA),
            vec![vec![this, 9]]
        );
        assert_eq!(floats(&e, this + 0xbc, 4), vec![-1.0, 0.0, 0.0, 1.0]);
    }

    const APPLY: u32 = 0x0300_0090;

    /// A node with one child whose `ApplyTransform` records the translation it gets.
    fn apply_engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0300_1000, 0x100);
        e.register(APPLY, |e, a| {
            let k = [e.mem.u32(a[2]), e.mem.u32(a[2] + 4), e.mem.u32(a[2] + 8)];
            for (index, word) in k.into_iter().enumerate() {
                e.mem.set_u32(0x0300_1000 + 4 * index as u32, word);
            }
            Ret::default()
        });
        let identity = |e: &mut Engine, at: u32| {
            for (index, value) in IDENTITY.iter().enumerate() {
                e.mem.set_f32(at + 4 * index as u32, *value);
            }
        };
        e.register(MATRIX_TIMES, |_, a| a[1].into_ret());
        e.register_double(MATRIX_TRANSPOSE_TIMES, move |e, a| {
            identity(e, a[1]);
            Ret::default()
        });
        e.register_double(MATRIX_INVERSE, move |e, a| {
            identity(e, a[1]);
            Ret::default()
        });
        let child_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(child_vtable + 0x90, APPLY);
        let child = e.mem.alloc(0x10);
        e.mem.set_u32(child, child_vtable);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children + 4, child);
        let this = e.mem.alloc(0xcc);
        e.mem.set_u32(this + 0xa0, children);
        e.mem.set_u16(this + 0xa6, 2);
        e.call_log = Some(vec![]);
        (e, this, child)
    }

    #[test]
    fn apply_transform_without_the_flag_moves_the_plane() {
        let (mut e, this, child) = apply_engine();
        set_floats(&mut e, this + 0xac, &[0.0, 0.0, 1.0, 2.0]);
        let matrix = e.mem.alloc(0x24);
        let translate = e.mem.alloc(0xc);
        set_floats(&mut e, translate, &[1.0, 2.0, 3.0]);
        e.call(0x00a9_a860, &args![this, matrix, translate, 0x100u32]);
        assert_eq!(floats(&e, this + 0xac, 4), vec![0.0, 0.0, 1.0, 5.0]);
        assert_eq!(calls_to(&e, MATRIX_INVERSE)[0][0], matrix);
        assert_eq!(calls_to(&e, APPLY), vec![vec![child, matrix, translate, 1]]);
        assert_eq!(floats(&e, 0x0300_1000, 3), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn apply_transform_with_the_flag_works_in_the_node_frame() {
        let (mut e, this, child) = apply_engine();
        set_floats(&mut e, this + 0x34, &TURN);
        set_floats(&mut e, this + 0x58, &[1.0, 1.0, 1.0, 2.0]);
        set_floats(&mut e, this + 0xac, &[0.0, 1.0, 0.0, 0.5]);
        let matrix = e.mem.alloc(0x24);
        set_floats(&mut e, matrix, &IDENTITY);
        let translate = e.mem.alloc(0xc);
        set_floats(&mut e, translate, &[2.0, 0.0, 0.0]);
        e.call(0x00a9_a860, &args![this, matrix, translate, 1u32]);
        let times = calls_to(&e, MATRIX_TIMES);
        assert_eq!((times[0][0], times[0][2]), (matrix, this + 0x34));
        assert_eq!(calls_to(&e, MATRIX_TRANSPOSE_TIMES)[0][2], times[0][1]);
        // k = R^T ((M L + T) - L) / s = (0, -1, 0).
        assert_eq!(floats(&e, 0x0300_1000, 3), vec![0.0, -1.0, 0.0]);
        assert_eq!(floats(&e, this + 0xac, 4), vec![0.0, 1.0, 0.0, -0.5]);
        let apply = calls_to(&e, APPLY);
        assert_eq!((apply[0][0], apply[0][3]), (child, 1));
        assert_eq!(apply[0][1], calls_to(&e, MATRIX_TRANSPOSE_TIMES)[0][1]);
    }

    #[test]
    fn on_visible_culls_both_children_by_side() {
        let mut e = Engine::new();
        e.map(0x0300_2000, 0x10);
        e.register_double(PLANE_WHICH_SIDE, |e, _| {
            let side = e.mem.u32(0x0300_2000);
            e.mem.set_u32(0x0300_2000, e.mem.u32(0x0300_2004));
            side.into_ret()
        });
        e.register(AV_OBJECT_CULL, |_, _| Ret::default());
        let camera = e.mem.alloc(0xa0);
        set_floats(&mut e, camera + 0x68, &TURN);
        let culler = e.mem.alloc(0x10);
        e.mem.set_u32(culler + 0xc, camera);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children, 0x1111);
        e.mem.set_u32(children + 4, 0x2222);
        let this = e.mem.alloc(0xcc);
        e.mem.set_u32(this + 0xa0, children);
        let culled =
            |e: &Engine| -> Vec<u32> { calls_to(e, AV_OBJECT_CULL).iter().map(|c| c[0]).collect() };
        // Side 2: the second child first.
        e.mem.set_u32(0x0300_2000, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a9_ac10, &args![this, culler]);
        assert_eq!(culled(&e), vec![0x2222, 0x1111]);
        assert_eq!(
            calls_to(&e, PLANE_WHICH_SIDE),
            vec![vec![this + 0xbc, camera + 0x8c]]
        );
        // Side 0 asks again with the direction; 1 then keeps the order.
        e.mem.set_u32(0x0300_2000, 0);
        e.mem.set_u32(0x0300_2004, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a9_ac10, &args![this, culler]);
        assert_eq!(culled(&e), vec![0x1111, 0x2222]);
        assert_eq!(calls_to(&e, PLANE_WHICH_SIDE).len(), 2);
        // A missing child is skipped.
        e.mem.set_u32(children, 0);
        e.mem.set_u32(0x0300_2000, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a9_ac10, &args![this, culler]);
        assert_eq!(culled(&e), vec![0x2222]);
    }

    #[test]
    fn is_equal_negates_the_plane_comparison() {
        let mut e = Engine::new();
        e.register(NODE_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(PLANE_COMPARE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a9_acb0, &args![0x4000u32, 0x5000u32]).bool());
        assert_eq!(calls_to(&e, PLANE_COMPARE), vec![vec![0x40ac, 0x50ac]]);
        e.register(PLANE_COMPARE, |_, _| 1u32.into_ret());
        assert!(!e.call(0x00a9_acb0, &args![0x4000u32, 0x5000u32]).bool());
        e.register(NODE_IS_EQUAL, |_, _| 0u32.into_ret());
        e.register(PLANE_COMPARE, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a9_acb0, &args![0x4000u32, 0x5000u32]).bool());
    }

    #[test]
    fn viewer_strings_add_class_and_both_planes() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(NI_BSP_NODE_RTTI, 0x0102_0000);
        e.register(NODE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x7000u32.into_ret());
        e.register(PLANE_GET_VIEWER_STRING, |_, a| (a[0] & 0xff).into_ret());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            let count = e.mem.u16(a[0] + 0xa);
            e.mem.set_u32(0x011f_5f00 + 4 * u32::from(count), value);
            e.mem.set_u16(a[0] + 0xa, count + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        let this = e.mem.alloc(0xcc);
        e.call_log = Some(vec![]);
        e.call(0x00a9_acf0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, PLANE_GET_VIEWER_STRING),
            vec![
                vec![this + 0xac, NAME_MODEL_PLANE],
                vec![this + 0xbc, NAME_WORLD_PLANE]
            ]
        );
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![strings, 2]]);
        assert_eq!(e.mem.u32(0x011f_5f00), 0x7000);
        assert_eq!(e.mem.u32(0x011f_5f04), (this + 0xac) & 0xff);
    }

    #[test]
    fn create_clone_builds_a_node_and_copies_the_plane() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for double in [
            NODE_CONSTRUCT,
            PLANE_CONSTRUCT,
            ARRAY_SIZE_TO,
            NODE_COPY_MEMBERS,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xcc);
        set_floats(&mut e, this + 0xac, &[1.0, 2.0, 3.0, 4.0]);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a9_ae00, &args![this, 0x66u32]).u32();
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0xcc]]);
        assert_eq!(calls_to(&e, NODE_CONSTRUCT), vec![vec![clone, 0]]);
        assert_eq!(e.mem.u32(clone), BSP_NODE_VTABLE);
        assert_eq!(
            calls_to(&e, PLANE_CONSTRUCT),
            vec![vec![clone + 0xac], vec![clone + 0xbc]]
        );
        assert_eq!(calls_to(&e, ARRAY_SIZE_TO), vec![vec![clone + 0x9c, 2]]);
        assert_eq!(
            calls_to(&e, NODE_COPY_MEMBERS),
            vec![vec![this, clone, 0x66]]
        );
        assert_eq!(floats(&e, clone + 0xac, 4), vec![1.0, 2.0, 3.0, 4.0]);
    }
}
