//! `NiMain/nibillboardnode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a5e5b0`: the jump target of `00a93c40`.
const JUMP_TARGET_00A93C40: u32 = 0x00a5_e5b0;

/// `00a5e9d0`: the jump target of `00a9a730`.
const JUMP_TARGET_00A9A730: u32 = 0x00a5_e9d0;

// Translated from 00a93c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a5e5b0` (engine map: `NiNode::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiBillboardNode@@UAA_NAAVNiStream@@@Z`.
pub fn ni_billboard_node_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A93C40, &args![this, stream]).bool()
}

// Translated from 00a9a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::LinkObject` (Xbox PDB): a 5-byte jump to `00a5e9d0` (engine map: `NiNode::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiBillboardNode@@UAAXAAVNiStream@@@Z`.
pub fn ni_billboard_node_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9A730, &args![this, stream]);
}

layout! {
    /// `NiBillboardNode` (Xbox PDB), 0xb4 bytes on PC (the size its
    /// constructors allocate; the Xbox build aligns the transforms to 16
    /// bytes). Inherited fields at their PC offsets.
    pub struct NiBillboardNode: 0xb4 {
        /// `NiAVObject::m_pkParent` (Xbox PDB).
        0x18 m_pkParent: u32,
        /// `NiAVObject::m_kWorldBound` (Xbox PDB): a pointer on PC.
        0x20 m_kWorldBound: u32,
        /// `NiNode::m_kChildren.m_pBase` (Xbox PDB).
        0xa0 m_pkChildren: u32,
        /// `NiNode::m_kChildren.m_usSize` (Xbox PDB).
        0xa6 m_usChildrenSize: u16,
        /// `NiNode::m_kChildren.m_usESize` (Xbox PDB).
        0xa8 m_usChildrenESize: u16,
        /// `m_uFlags` (Xbox PDB): the low three bits are the billboard mode,
        /// bit 3 a flag passed down to the children.
        0xac m_uFlags: u16,
        /// `m_fSavedTime` (Xbox PDB).
        0xb0 m_fSavedTime: f32,
    }
}

/// `NiAVObject::m_kLocal` and `m_kWorld` (Xbox PDB), PC offsets: rotation
/// (nine floats, row-major), translation, scale.
const LOCAL_TRANSFORM: u32 = 0x34;
const WORLD_ROTATE: u32 = 0x68;
const WORLD_TRANSLATE: u32 = 0x8c;
const WORLD_SCALE: u32 = 0x98;

/// The vtable `0102bf44` (RTTI `.?AVNiBillboardNode@@`).
const BILLBOARD_NODE_VTABLE: u32 = 0x0102_bf44;
/// `00aa13e0` (`cdecl`, one word): allocates an object of that size.
const NEW_OBJECT: u32 = 0x00aa_13e0;
/// `00a5ecb0` (`NiMain/ninode.cpp`, `thiscall`, one word): the `NiNode`
/// constructor body.
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;

/// `FLD dword` of the float at `addr`.
fn load(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// A `float` memory operand at `addr`.
fn operand(e: &Engine, addr: u32) -> f64 {
    x87_m32(e.mem.u32(addr))
}

/// `FSTP dword` of `value` to `addr`.
fn store(e: &mut Engine, addr: u32, value: f64) {
    e.mem.set_u32(addr, x87_store32(value));
}

/// `count` words from `from` to `to`.
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// Builds a fresh `NiBillboardNode` in `node`: the `NiNode` body with 0,
/// 0.0 at `+0xb0`, flags 9 (mode 1, bit 3), the vtable.
fn construct_default(e: &mut Engine, node: u32) {
    e.call(NI_NODE_CONSTRUCT, &args![node, 0u32]);
    store(e, node.wrapping_add(0xb0), 0.0);
    e.mem.set_u16(node.wrapping_add(0xac), 9);
    e.mem.set_u32(node, BILLBOARD_NODE_VTABLE);
}

// Translated from 00a7e770 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project and unnamed in the engine map
/// (plain `RET`, no arguments); the object factory registered next to the
/// name at `0109dec4` (`nimainsdm.rs`): allocates 0xb4 bytes (`00aa13e0`)
/// and, when that succeeds, builds an `NiBillboardNode` in it (the
/// `NiNode` body with 0, 0.0 at `+0xb0`, flags 9, the vtable `0102bf44`).
/// Returns the object or null.
pub fn fn_00a7e770(e: &mut Engine) -> Ptr<NiBillboardNode> {
    let node = e.call(NEW_OBJECT, &args![0xb4u32]).u32();
    if node == 0 {
        return Ptr::NULL;
    }
    construct_default(e, node);
    Ptr::new(node)
}

// Translated from 00a7e710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::CreateClone` (Xbox PDB; `thiscall`, `RET 4`): a new
/// node built as `00a7e770` does (null when the allocation fails), then
/// `00a5dc80` on `this` with the clone and `cloning`, and the clone's flag
/// word set to this node's. The flag word is written through the clone
/// pointer even when it is null, as the exe does. Returns the clone.
pub fn ni_billboard_node_create_clone(
    e: &mut Engine,
    this: Ptr<NiBillboardNode>,
    cloning: u32,
) -> Ptr<NiBillboardNode> {
    let block = e.call(NEW_OBJECT, &args![0xb4u32]).u32();
    let clone = if block != 0 {
        construct_default(e, block);
        block
    } else {
        0
    };
    e.call(0x00a5_dc80, &args![this, clone, cloning]);
    let flags = e.get(this, NiBillboardNode::m_uFlags);
    e.set(
        Ptr::<NiBillboardNode>::new(clone),
        NiBillboardNode::m_uFlags,
        flags,
    );
    Ptr::new(clone)
}

// Translated from 00a7e7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`):
/// `NiNode::LoadBinary` (`00a5dd40`), then the flag word by file version
/// (`stream + 0xd8`): from `0x0a000102` on, read from the stream's file
/// (`stream + 0x24c`) by its reader function at `+8` (`cdecl`: file,
/// destination, 2, the address of a stack word holding 2, 1), the result
/// added to the file's word at `+4`; before that version taken from the
/// half-word at `stream + 0x284`, with bits 13 to 15 moved up one place
/// for versions before `0x04020003`, then shifted down by 6. Either way
/// bit 3 is then set.
pub fn ni_billboard_node_load_binary(e: &mut Engine, this: Ptr<NiBillboardNode>, stream: Ptr) {
    e.call(0x00a5_dd40, &args![this, stream]);
    let flags_at = this.addr().wrapping_add(0xac);
    let version = e.mem.u32(stream.addr().wrapping_add(0xd8));
    if version < 0x0a00_0102 {
        let mut flags = u32::from(e.mem.u16(stream.addr().wrapping_add(0x284)));
        e.mem.set_u16(flags_at, flags as u16);
        if version < 0x0402_0003 {
            flags = ((flags & 0xe000) << 1) | (flags & 0x1fff);
            e.mem.set_u16(flags_at, flags as u16);
        }
        let shifted = e.mem.u16(flags_at) >> 6;
        e.mem.set_u16(flags_at, shifted);
    } else {
        let file = e.mem.u32(stream.addr().wrapping_add(0x24c));
        let read = e.mem.u32(file.wrapping_add(8));
        let got = e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), 2);
            e.call(read, &args![file, flags_at, 2u32, size, 1u32]).u32()
        });
        let total = e.mem.u32(file.wrapping_add(4)).wrapping_add(got);
        e.mem.set_u32(file.wrapping_add(4), total);
    }
    let flags = e.mem.u16(flags_at) | 8;
    e.mem.set_u16(flags_at, flags);
}

// Translated from 00a7da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::UpdateSelectedDownwardPass` (Xbox PDB; `thiscall`,
/// `RET 8`): keeps the update's time (the float at `data`) at `+0xb0`, runs
/// `NiNode::UpdateSelectedDownwardPass` (`00a5ded0`) and the node's virtual
/// slot `+0xbc`.
pub fn ni_billboard_node_update_selected_downward_pass(
    e: &mut Engine,
    this: Ptr<NiBillboardNode>,
    data: Ptr,
    flags: u32,
) {
    let time = load(e, data.addr());
    store(e, this.addr().wrapping_add(0xb0), time);
    e.call(0x00a5_ded0, &args![this, data, flags]);
    e.vcall(this.addr(), 0xbc, &[]);
}

// Translated from 00a7dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::UpdateDownwardPass` (Xbox PDB; `thiscall`, `RET 8`):
/// keeps the update's time at `+0xb0`, copies the update's byte at
/// `data + 4` into bit 3 of the flag word, then runs
/// `NiNode::UpdateDownwardPass` (`00a5dd70`) and the virtual slot `+0xbc`.
pub fn ni_billboard_node_update_downward_pass(
    e: &mut Engine,
    this: Ptr<NiBillboardNode>,
    data: Ptr,
    flags: u32,
) {
    let time = load(e, data.addr());
    store(e, this.addr().wrapping_add(0xb0), time);
    let own = e.get(this, NiBillboardNode::m_uFlags);
    let own = if e.mem.u8(data.addr().wrapping_add(4)) != 0 {
        own | 8
    } else {
        own & 0xfff7
    };
    e.set(this, NiBillboardNode::m_uFlags, own);
    e.call(0x00a5_dd70, &args![this, data, flags]);
    e.vcall(this.addr(), 0xbc, &[]);
}

// Translated from 00a7e610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::OnVisible` (Xbox PDB; `thiscall`, `RET 4`): turns the
/// node to the culler's camera (`RotateToCamera` with the word at
/// `culler + 0xc`), then `NiNode::OnVisible` (`00a5dbe0`).
pub fn ni_billboard_node_on_visible(e: &mut Engine, this: Ptr<NiBillboardNode>, culler: Ptr) {
    let camera = Ptr::new(e.mem.u32(culler.addr().wrapping_add(0xc)));
    ni_billboard_node_rotate_to_camera(e, this, camera);
    e.call(0x00a5_dbe0, &args![this, culler]);
}

// Translated from 00a7e630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::UpdateWorldBound` (Xbox PDB; `thiscall`, plain `RET`):
/// nothing without children (`m_kChildren.m_usESize`). Without a world
/// bound one is made (`00aa3e40` on the allocator at `011f6238` with 0x10,
/// given to the node's virtual slot `+0x78`) with the centre from
/// `011f426c` and radius 0. Then the bound is re-centred on the world
/// translation and its radius grows by the distance it moved (`00ec6040`,
/// the square root of the summed squares).
pub fn ni_billboard_node_update_world_bound(e: &mut Engine, this: Ptr<NiBillboardNode>) {
    if e.get(this, NiBillboardNode::m_usChildrenESize) == 0 {
        return;
    }
    let me = this.addr();
    if e.get(this, NiBillboardNode::m_kWorldBound) == 0 {
        let bound = e.call(0x00aa_3e40, &args![0x011f_6238u32, 0x10u32]).u32();
        e.vcall(me, 0x78, &args![bound]);
        let bound = e.get(this, NiBillboardNode::m_kWorldBound);
        copy_words(e, 0x011f_426c, bound, 2);
        store(e, bound.wrapping_add(0xc), 0.0);
        copy_words(e, 0x011f_4274, bound.wrapping_add(8), 1);
    }
    let bound = e.get(this, NiBillboardNode::m_kWorldBound);
    let translate = me.wrapping_add(WORLD_TRANSLATE);
    let mut moved = [0u32; 3];
    for (i, word) in moved.iter_mut().enumerate() {
        let offset = 4 * i as u32;
        *word = x87_store32(x87_sub(
            load(e, bound.wrapping_add(offset)),
            operand(e, translate.wrapping_add(offset)),
        ));
    }
    let radius = x87_store32(load(e, bound.wrapping_add(0xc)));
    copy_words(e, translate, bound, 3);
    let [d0, d1, d2] = moved.map(x87_load32);
    let partial = x87_add(x87_mul(d0, d0), x87_mul(d1, d1));
    let sum = x87_store32(x87_add(x87_mul(d2, d2), partial));
    let length = e.call(0x00ec_6040, &args![x87_load32(sum)]).f64();
    let length = x87_store32(length);
    store(
        e,
        bound.wrapping_add(0xc),
        x87_add(x87_load32(length), x87_m32(radius)),
    );
}

// Translated from 00a7dc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`stdcall`, `RET 0x14`: camera, position and
/// three output vectors). False when the squared distance from `position`
/// to the camera's world translation is below the float at `01017d00`.
/// Otherwise the direction `d` to the camera is normalised (`004a0c10`),
/// `forward` is the camera's first rotation column negated, `up` and
/// `right` its second and third columns; when `forward . d` is below the
/// double at `0109deb0` they are turned onto `d`: the axis
/// `d.UnitCross(forward)` (`0053d1a0`), the angle `00a58510(dot)`, the
/// rotation `004168a0`, `forward` becomes `d` and `up`/`right` are rotated
/// (`004b4500`). True.
pub fn fn_00a7dc40(
    e: &mut Engine,
    camera: Ptr,
    position: Ptr,
    forward: Ptr,
    up: Ptr,
    right: Ptr,
) -> bool {
    let cam = camera.addr();
    let pos = position.addr();
    // The frame: d at +0, temporaries at +0xc and +0x18 (the axis), the
    // rotation matrix at +0x24.
    e.with_stack(0x48, |e, frame| {
        let f = frame.addr();
        for i in 0..3u32 {
            let d = x87_sub(
                load(e, cam.wrapping_add(WORLD_TRANSLATE + 4 * i)),
                operand(e, pos.wrapping_add(4 * i)),
            );
            store(e, f + 4 * i, d);
        }
        let (d0, d1, d2) = (load(e, f), load(e, f + 4), load(e, f + 8));
        let partial = x87_add(x87_mul(d0, d0), x87_mul(d1, d1));
        let distance = x87_store32(x87_add(partial, x87_mul(d2, d2)));
        let near: f32 = e.global(0x0101_7d00);
        if x87_load32(distance) < f64::from(near) {
            return false;
        }
        e.call(0x004a_0c10, &args![frame]);
        let out_a = forward.addr();
        for (i, offset) in [0x68u32, 0x74, 0x80].iter().enumerate() {
            store(
                e,
                f + 0xc + 4 * i as u32,
                load(e, cam.wrapping_add(*offset)),
            );
        }
        for i in 0..3u32 {
            let column = load(e, f + 0xc + 4 * i);
            store(e, out_a.wrapping_add(4 * i), -column);
        }
        for (out, offsets) in [
            (up.addr(), [0x6cu32, 0x78, 0x84]),
            (right.addr(), [0x70, 0x7c, 0x88]),
        ] {
            for (i, offset) in offsets.iter().enumerate() {
                let value = load(e, cam.wrapping_add(*offset));
                store(e, out.wrapping_add(4 * i as u32), value);
            }
        }
        let p1 = x87_mul(load(e, out_a + 4), operand(e, f + 4));
        let p0 = x87_mul(load(e, out_a), operand(e, f));
        let p2 = x87_mul(load(e, out_a + 8), operand(e, f + 8));
        let dot = x87_store32(x87_add(x87_add(p1, p0), p2));
        let aligned: f64 = e.global(0x0109_deb0);
        if !(x87_load32(dot) < aligned) {
            return true;
        }
        let axis = f + 0x18;
        e.call(0x0053_d1a0, &args![frame, axis, forward]);
        let angle = e.call(0x00a5_8510, &args![x87_quiet_bits(dot)]).f64();
        let words: Vec<u32> = (0..3)
            .map(|i| x87_quiet_bits(e.mem.u32(axis + 4 * i)))
            .collect();
        let matrix = f + 0x24;
        e.call(
            0x0041_68a0,
            &args![matrix, x87_store32(angle), words[0], words[1], words[2]],
        );
        copy_words(e, f, out_a, 3);
        for out in [up.addr(), right.addr()] {
            let rotated = e.call(0x004b_4500, &args![matrix, axis, out]).u32();
            copy_words(e, rotated, out, 3);
        }
        true
    })
}

/// `004b3ae0` (`cdecl`: out, vector, matrix): the vector times the
/// rotation, written to `out` and returned.
const VECTOR_TIMES_MATRIX: u32 = 0x004b_3ae0;
/// `0053d280` (`thiscall` on a vector: out, a float): used with the world
/// scale.
const VECTOR_SCALED: u32 = 0x0053_d280;
/// `00ec6040`: the square root of `ST0`.
const SQUARE_ROOT: u32 = 0x00ec_6040;
/// The floats at `010959f4` and `0109deb8` the lengths are compared with,
/// and the matrix at `011a9448` the rotation starts from.
const MIN_LENGTH_010959F4: u32 = 0x0109_59f4;
const MIN_LENGTH_0109DEB8: u32 = 0x0109_deb8;
const START_MATRIX: u32 = 0x011a_9448;

/// `004b3ae0` of the vector at `vector` (into `out`), copied back over it.
fn rotate_in_place(e: &mut Engine, out: u32, vector: u32, rotation: u32) {
    let result = e
        .call(VECTOR_TIMES_MATRIX, &args![out, vector, rotation])
        .u32();
    copy_words(e, result, vector, 3);
}

/// The camera's rotation columns into three frame vectors: column 0
/// negated into `negated`, column 1 into `second`, column 2 into `third`,
/// each through an `FLD`/`FSTP` copy (and `FCHS` for the first).
fn camera_columns(e: &mut Engine, camera: u32, negated: u32, second: u32, third: u32) {
    for (i, (first, middle, last)) in [
        (0x68u32, 0x6cu32, 0x70u32),
        (0x74, 0x78, 0x7c),
        (0x80, 0x84, 0x88),
    ]
    .iter()
    .enumerate()
    {
        let i = 4 * i as u32;
        let value = load(e, camera.wrapping_add(*first));
        store(e, negated + i, -value);
        let value = load(e, camera.wrapping_add(*middle));
        store(e, second + i, value);
        let value = load(e, camera.wrapping_add(*last));
        store(e, third + i, value);
    }
}

/// The direction from the node to the camera (world translations), taken
/// into the node's frame (`004b3ae0`, then `0053d280` with the world scale)
/// into `out`; `scratch` takes the intermediate vector.
fn camera_direction(e: &mut Engine, me: u32, camera: u32, delta: u32, scratch: u32, out: u32) {
    for i in 0..3u32 {
        let d = x87_sub(
            load(e, camera.wrapping_add(WORLD_TRANSLATE + 4 * i)),
            operand(e, me.wrapping_add(WORLD_TRANSLATE + 4 * i)),
        );
        store(e, delta + 4 * i, d);
    }
    let scale = x87_quiet_bits(e.mem.u32(me.wrapping_add(WORLD_SCALE)));
    let rotation = me.wrapping_add(WORLD_ROTATE);
    let rotated = e
        .call(VECTOR_TIMES_MATRIX, &args![scratch, delta, rotation])
        .u32();
    e.call(VECTOR_SCALED, &args![rotated, out, scale]);
}

/// `00ec6040` of the float `sum` and the result's `FSTP`/`FLD` copies: the
/// length's bits.
fn square_root_bits(e: &mut Engine, sum: f64) -> u32 {
    let sum = x87_store32(sum);
    let root = e.call(SQUARE_ROOT, &args![x87_load32(sum)]).f64();
    x87_store32(x87_load32(x87_store32(root)))
}

// Translated from 00a7de40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::RotateToCamera` (Xbox PDB; `thiscall`, `RET 4`). The
/// world transform becomes the parent's world transform (identity without
/// a parent, `NiTransform::MakeIdentity`) times the local one (`0062c250`);
/// then a rotation `M` (starting as the matrix at `011a9448`) is built by
/// the billboard mode (flag bits 0 to 2), from the camera's rotation
/// columns or its direction, taken into the node's frame (`004b3ae0`):
/// - 0 and 3: `forward`, `up`, `right` (mode 3 through `00a7dc40`, which
///   can leave `M` as it is); `M`'s third column is `forward`, its first
///   two the camera's `right` and `up` turned about `forward` so `up` lies
///   in the y-z plane (scaled by the inverse length of its (y, z) part),
///   or both negated when that length is not above the float at `010959f4`;
/// - 1: the direction to the camera, `M` a turn about y towards it (when its
///   x-z length is at least the float at `0109deb8`);
/// - 2 and 4: the three vectors as `M`'s columns (mode 4 through
///   `00a7dc40`);
/// - 5: the world rotation is reset to identity first; then a turn about
///   z towards the direction (x-y length at least the float at
///   `0109deb8`);
/// - other values leave `M` unchanged.
///
/// The world rotation becomes itself times `M` (`NiMatrix3::operator*`),
/// and each child gets virtual slot `+0xa4` with an update record (the
/// float at `+0xb0`, then flag bit 3 and four zero bytes) and 0.
pub fn ni_billboard_node_rotate_to_camera(e: &mut Engine, this: Ptr<NiBillboardNode>, camera: Ptr) {
    let me = this.addr();
    let cam = camera.addr();
    let rotation = me.wrapping_add(WORLD_ROTATE);
    e.with_stack(0x140, |e, frame| {
        let o = |offset: u32| frame.addr() + offset;
        let parent = e.get(this, NiBillboardNode::m_pkParent);
        if parent != 0 {
            copy_words(e, parent.wrapping_add(WORLD_ROTATE), o(0xd8), 13);
        } else {
            e.call(0x00a8_6bf0, &args![o(0xd8)]);
        }
        let world = e
            .call(
                0x0062_c250,
                &args![o(0xd8), o(0x10c), me.wrapping_add(LOCAL_TRANSFORM)],
            )
            .u32();
        let mode = e.get(this, NiBillboardNode::m_uFlags) & 7;
        copy_words(e, world, rotation, 13);
        copy_words(e, START_MATRIX, o(0x48), 9);
        // `M` at +0x48, row-major: element `k` at +0x48 + 4k.
        let m = |k: u32| o(0x48) + 4 * k;
        match mode {
            0 | 3 => {
                let built = if mode == 3 {
                    fn_00a7dc40(
                        e,
                        camera,
                        Ptr::new(me.wrapping_add(WORLD_TRANSLATE)),
                        Ptr::new(o(0x10)),
                        Ptr::new(o(0x38)),
                        Ptr::new(o(0x2c)),
                    )
                } else {
                    camera_columns(e, cam, o(0x10), o(0x38), o(0x2c));
                    true
                };
                if built {
                    rotate_in_place(e, o(0xb4), o(0x10), rotation);
                    rotate_in_place(e, o(0x78), o(0x38), rotation);
                    rotate_in_place(e, o(0x90), o(0x2c), rotation);
                    let (b1, c1) = (load(e, o(0x3c)), load(e, o(0x30)));
                    let sum = x87_add(x87_mul(b1, b1), x87_mul(c1, c1));
                    let length = x87_load32(square_root_bits(e, sum));
                    let limit = load(e, MIN_LENGTH_010959F4);
                    if length > limit {
                        let inverse = x87_load32(x87_store32(x87_div(1.0, length)));
                        let p = x87_load32(x87_store32(x87_mul(inverse, b1)));
                        let q = x87_load32(x87_store32(x87_mul(inverse, -c1)));
                        let (b0, b2) = (load(e, o(0x38)), load(e, o(0x40)));
                        let (c0, c2) = (load(e, o(0x2c)), load(e, o(0x34)));
                        store(e, m(0), x87_add(x87_mul(c0, p), x87_mul(b0, q)));
                        store(e, m(3), x87_add(x87_mul(q, b1), x87_mul(p, c1)));
                        store(e, m(6), x87_add(x87_mul(b2, q), x87_mul(c2, p)));
                        store(e, m(1), x87_sub(x87_mul(b0, p), x87_mul(c0, q)));
                        store(e, m(4), x87_sub(x87_mul(b1, p), x87_mul(c1, q)));
                        let p_b2 = x87_mul(p, operand(e, o(0x40)));
                        store(e, m(7), x87_sub(p_b2, x87_mul(q, c2)));
                    } else {
                        for (from, to) in [
                            (0x2c, 0),
                            (0x30, 3),
                            (0x34, 6),
                            (0x38, 1),
                            (0x3c, 4),
                            (0x40, 7),
                        ] {
                            let value = load(e, o(from));
                            store(e, m(to), -value);
                        }
                    }
                    for (from, to) in [(0x10, 2), (0x14, 5), (0x18, 8)] {
                        let value = load(e, o(from));
                        store(e, m(to), value);
                    }
                }
            }
            1 => {
                camera_direction(e, me, cam, o(0x20), o(0xc0), o(0x10));
                let (a2, a0) = (load(e, o(0x18)), load(e, o(0x10)));
                let sum = x87_add(x87_mul(a2, a2), x87_mul(a0, a0));
                let length = x87_load32(square_root_bits(e, sum));
                if !(length < load(e, MIN_LENGTH_0109DEB8)) {
                    let inverse = x87_load32(x87_store32(x87_div(1.0, length)));
                    store(e, o(0x10), x87_mul(a0, inverse));
                    store(e, o(0x18), x87_mul(inverse, operand(e, o(0x18))));
                    let z = load(e, o(0x18));
                    let x = load(e, o(0x10));
                    store(e, m(0), z);
                    store(e, m(3), 0.0);
                    store(e, m(6), -x);
                    store(e, m(1), 0.0);
                    store(e, m(4), 1.0);
                    store(e, m(7), 0.0);
                    store(e, m(5), 0.0);
                    store(e, m(2), x);
                    store(e, m(8), z);
                }
            }
            2 | 4 => {
                let built = if mode == 4 {
                    fn_00a7dc40(
                        e,
                        camera,
                        Ptr::new(me.wrapping_add(WORLD_TRANSLATE)),
                        Ptr::new(o(0x2c)),
                        Ptr::new(o(0x38)),
                        Ptr::new(o(0x10)),
                    )
                } else {
                    camera_columns(e, cam, o(0x2c), o(0x38), o(0x10));
                    true
                };
                if built {
                    rotate_in_place(e, o(0xa8), o(0x2c), rotation);
                    rotate_in_place(e, o(0xcc), o(0x38), rotation);
                    rotate_in_place(e, o(0x84), o(0x10), rotation);
                    for (from, to) in [
                        (0x10, 0),
                        (0x14, 3),
                        (0x18, 6),
                        (0x38, 1),
                        (0x3c, 4),
                        (0x40, 7),
                        (0x2c, 2),
                        (0x30, 5),
                        (0x34, 8),
                    ] {
                        let value = load(e, o(from));
                        store(e, m(to), value);
                    }
                }
            }
            5 => {
                for (k, value) in [
                    (0, 1.0),
                    (3, 0.0),
                    (6, 0.0),
                    (1, 0.0),
                    (7, 0.0),
                    (4, 1.0),
                    (2, 0.0),
                    (5, 0.0),
                    (8, 1.0),
                ] {
                    store(e, rotation + 4 * k, value);
                }
                camera_direction(e, me, cam, o(0x20), o(0x9c), o(0x10));
                let (a1, a0) = (load(e, o(0x14)), load(e, o(0x10)));
                let sum = x87_add(x87_mul(a1, a1), x87_mul(a0, a0));
                let length = x87_load32(square_root_bits(e, sum));
                if !(length < load(e, MIN_LENGTH_0109DEB8)) {
                    let inverse = x87_load32(x87_store32(x87_div(1.0, length)));
                    store(e, o(0x10), x87_mul(a0, inverse));
                    store(e, o(0x14), x87_mul(inverse, operand(e, o(0x14))));
                    let y = load(e, o(0x14));
                    let x = load(e, o(0x10));
                    store(e, m(0), -y);
                    store(e, m(3), x);
                    store(e, m(6), 0.0);
                    store(e, m(7), 0.0);
                    store(e, m(2), 0.0);
                    store(e, m(5), 0.0);
                    store(e, m(1), x);
                    store(e, m(4), y);
                    store(e, m(8), 1.0);
                }
            }
            _ => {}
        }
        let product = e
            .call(0x0043_f8d0, &args![rotation, o(0x10c), o(0x48)])
            .u32();
        let time = load(e, me.wrapping_add(0xb0));
        let flag = (e.mem.u8(me.wrapping_add(0xac)) >> 3) & 1;
        copy_words(e, product, rotation, 9);
        store(e, o(0x6c), time);
        e.mem.set_u8(o(0x70), flag);
        for i in 0..4 {
            e.mem.set_u8(o(0x71) + i, 0);
        }
        let mut i = 0u32;
        if e.get(this, NiBillboardNode::m_usChildrenSize) != 0 {
            loop {
                let children = e.get(this, NiBillboardNode::m_pkChildren);
                let child = e.mem.u32(children.wrapping_add(4 * i));
                if child != 0 {
                    e.vcall(child, 0xa4, &args![o(0x6c), 0u32]);
                }
                i = i.wrapping_add(1);
                if i >= u32::from(e.get(this, NiBillboardNode::m_usChildrenSize)) {
                    break;
                }
            }
        }
    });
}

// Translated from 00a7dab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::UpdateRigidDownwardPass` (Xbox PDB; `thiscall`, `RET 8`): keeps the
/// update's time (the float at `data`) at `+0xb0`, runs `NiNode::UpdateRigidDownwardPass`
/// (`00a5e1f0`) and the node's virtual slot `+0xbc`.
pub fn ni_billboard_node_update_rigid_downward_pass(
    e: &mut Engine,
    this: Ptr<NiBillboardNode>,
    data: Ptr,
    flags: u32,
) {
    let time = load(e, data.addr());
    store(e, this.addr().wrapping_add(0xb0), time);
    e.call(0x00a5_e1f0, &args![this, data, flags]);
    e.vcall(this.addr(), 0xbc, &[]);
}

/// `NiNode::IsEqual` (`00a5e600`, `thiscall`, the other object).
const NODE_IS_EQUAL: u32 = 0x00a5_e600;

// Translated from 00a7db30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::IsEqual` (Xbox PDB): `NiNode::IsEqual`, then the same face mode (bits 0
/// to 2 of the flag word +0xac).
pub fn ni_billboard_node_is_equal(e: &mut Engine, this: Ptr<NiBillboardNode>, other: Ptr) -> bool {
    if !e.call(NODE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    (e.mem.u8(other.addr() + 0xac) ^ e.mem.u8(this.addr() + 0xac)) & 7 == 0
}

/// `00aa1070` (unnamed, `cdecl`, one word): allocates the text of a viewer string.
const ALLOCATE_TEXT: u32 = 0x00aa_1070;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;

// Translated from 00a7db60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::GetViewerString` (Xbox PDB), `cdecl`: a text of `strlen(name) + 0x16`
/// bytes (`00aa1070`) written as `"<name> = ALWAYS_FACE_CAMERA"`, `ROTATE_ABOUT_UP`,
/// `RIGID_FACE_CAMERA`, `ALWAYS_FACE_CENTER`, `RIGID_FACE_CENTER` or `BSROTATE_ABOUT_UP` for a
/// mode 0 to 5 (the formats at `0109de98` down to `0109de20`, the jump table as the `match`);
/// for any other mode the text is answered unwritten.
pub fn ni_billboard_node_get_viewer_string(e: &mut Engine, name: Ptr, mode: u32) -> Ptr {
    let size = (e.mem.cstr(name.addr()).len() as u32).wrapping_add(0x16);
    let text = e.call(ALLOCATE_TEXT, &args![size]).u32();
    let format = match mode {
        0 => Some(0x0109_de98u32),
        1 => Some(0x0109_de80),
        2 => Some(0x0109_de68),
        3 => Some(0x0109_de50),
        4 => Some(0x0109_de38),
        5 => Some(0x0109_de20),
        _ => None,
    };
    if let Some(format) = format {
        e.call(NI_SPRINTF, &args![text, size, format, name]);
    }
    Ptr::new(text)
}

// Translated from 00a7e850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::SaveBinary` (Xbox PDB): `NiNode::SaveBinary` (`00a5e880`), then the flag
/// word (+0xac, 2 bytes) through the stream's output (`+0x250`, its write function at +0xc; the
/// stream argument's stack word is the byte-count word).
pub fn ni_billboard_node_save_binary(e: &mut Engine, this: Ptr<NiBillboardNode>, stream: Ptr) {
    e.call(0x00a5_e880, &args![this, stream]);
    let output = e.mem.u32(stream.addr() + 0x250);
    let write = e.mem.u32(output + 0xc);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 2);
        e.call(write, &args![output, this.addr() + 0xac, 2u32, count, 1u32])
            .u32()
    });
    let position = e.mem.u32(output + 4).wrapping_add(done);
    e.mem.set_u32(output + 4, position);
}

/// `0096ad30` (`thiscall`, the new size) grows an `NiTArray`; `0096ae90` (`thiscall`, index
/// and the address of the element) stores an element.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;

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

// Translated from 00a7e890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::GetViewerStrings` (Xbox PDB): `NiNode::GetViewerStrings` (`00a5ea70`),
/// then the class name (`NiGetViewerString`, `00a7d330`, of the first word of the `NiRTTI` at
/// `011f4aa0`) and `m_eMode` (`0109debc`, [`ni_billboard_node_get_viewer_string`] of bits 0 to
/// 2 of the flag word), each kept in the `strings` argument's stack word and appended.
pub fn ni_billboard_node_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiBillboardNode>,
    strings: Ptr,
) {
    e.call(0x00a5_ea70, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(0x011f_4aa0);
        let text = e.call(0x00a7_d330, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot.addr());
        let mode = u32::from(e.mem.u16(this.addr() + 0xac)) & 7;
        let text = ni_billboard_node_get_viewer_string(e, Ptr::new(0x0109_debc), mode);
        e.mem.set_u32(slot.addr(), text.addr());
        append_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_3c40, ni_billboard_node_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a9_a730, ni_billboard_node_link_object(Ptr, Ptr)),
        entry!(0x00a7_e770, fn_00a7e770() -> Ptr<NiBillboardNode>),
        entry!(
            0x00a7_e710,
            ni_billboard_node_create_clone(Ptr<NiBillboardNode>, u32) -> Ptr<NiBillboardNode>
        ),
        entry!(
            0x00a7_e7b0,
            ni_billboard_node_load_binary(Ptr<NiBillboardNode>, Ptr)
        ),
        entry!(
            0x00a7_da80,
            ni_billboard_node_update_selected_downward_pass(Ptr<NiBillboardNode>, Ptr, u32)
        ),
        entry!(
            0x00a7_dae0,
            ni_billboard_node_update_downward_pass(Ptr<NiBillboardNode>, Ptr, u32)
        ),
        entry!(
            0x00a7_e610,
            ni_billboard_node_on_visible(Ptr<NiBillboardNode>, Ptr)
        ),
        entry!(
            0x00a7_e630,
            ni_billboard_node_update_world_bound(Ptr<NiBillboardNode>)
        ),
        entry!(0x00a7_dc40, fn_00a7dc40(Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00a7_de40,
            ni_billboard_node_rotate_to_camera(Ptr<NiBillboardNode>, Ptr)
        ),
        entry!(
            0x00a7_dab0,
            ni_billboard_node_update_rigid_downward_pass(Ptr<NiBillboardNode>, Ptr, u32)
        ),
        entry!(
            0x00a7_db30,
            ni_billboard_node_is_equal(Ptr<NiBillboardNode>, Ptr) -> bool
        ),
        entry!(0x00a7_db60, ni_billboard_node_get_viewer_string(Ptr, u32) -> Ptr),
        entry!(
            0x00a7_e850,
            ni_billboard_node_save_binary(Ptr<NiBillboardNode>, Ptr)
        ),
        entry!(
            0x00a7_e890,
            ni_billboard_node_get_viewer_strings(Ptr<NiBillboardNode>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a93c40_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_3c40;
        const TARGET: u32 = 0x00a5_e5b0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a9a730_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_a730;
        const TARGET: u32 = 0x00a5_e9d0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod b015_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == callee)
            .map(|c| c.1.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn set_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn floats(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    const IDENTITY: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

    #[test]
    fn create_object_builds_a_default_node_or_answers_null() {
        let mut e = Engine::new();
        noop(&mut e, &[NI_NODE_CONSTRUCT]);
        let block = e.mem.alloc(0xb4);
        e.register_double(NEW_OBJECT, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_e770, &[]).u32(), block);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, NEW_OBJECT), vec![vec![0xb4]]);
        assert_eq!(calls_to(&log, NI_NODE_CONSTRUCT), vec![vec![block, 0]]);
        assert_eq!(e.mem.u32(block), BILLBOARD_NODE_VTABLE);
        assert_eq!(e.mem.u16(block + 0xac), 9);
        assert_eq!(e.mem.u32(block + 0xb0), 0);
        e.register(NEW_OBJECT, |_, _| Ret::default());
        assert_eq!(e.call(0x00a7_e770, &[]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_members_and_the_flag_word() {
        let mut e = Engine::new();
        noop(&mut e, &[NI_NODE_CONSTRUCT, 0x00a5_dc80]);
        let block = e.mem.alloc(0xb4);
        e.register_double(NEW_OBJECT, move |_, _| block.into_ret());
        let this = e.mem.alloc(0xb4);
        e.mem.set_u16(this + 0xac, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_e710, &[this, 0x77]).u32(), block);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00a5_dc80), vec![vec![this, block, 0x77]]);
        assert_eq!(e.mem.u16(block + 0xac), 0x1234);
        assert_eq!(e.mem.u32(block), BILLBOARD_NODE_VTABLE);
    }

    #[test]
    fn load_binary_reads_the_flags_by_version() {
        let mut e = Engine::new();
        noop(&mut e, &[0x00a5_dd40]);
        let this = e.mem.alloc(0xb4);
        let stream = e.mem.alloc(0x290);
        // Old files: the half-word at +0x284, bits 13-15 moved up before
        // 4.2.0.3, then shifted down 6, then bit 3.
        e.mem.set_u16(stream + 0x284, 0xe0c0);
        e.mem.set_u32(stream + 0xd8, 0x0402_0002);
        e.call(0x00a7_e7b0, &[this, stream]);
        assert_eq!(e.mem.u16(this + 0xac), ((0xc0c0u16) >> 6) | 8);
        e.mem.set_u32(stream + 0xd8, 0x0402_0003);
        e.call(0x00a7_e7b0, &[this, stream]);
        assert_eq!(e.mem.u16(this + 0xac), (0xe0c0u16 >> 6) | 8);
        // New files: read through the file's reader at +8.
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(file + 8, 0x0090_5000);
        e.mem.set_u32(file + 4, 10);
        e.register(0x0090_5000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 2);
            e.mem.set_u16(a[1], 0x0101);
            2u32.into_ret()
        });
        e.mem.set_u32(stream + 0xd8, 0x0a00_0102);
        e.call_log = Some(vec![]);
        e.call(0x00a7_e7b0, &[this, stream]);
        let log = e.call_log.take().unwrap();
        let read = calls_to(&log, 0x0090_5000);
        assert_eq!(read.len(), 1);
        assert_eq!(&read[0][..3], &[file, this + 0xac, 2]);
        assert_eq!(read[0][4], 1);
        assert_eq!(e.mem.u16(this + 0xac), 0x0109);
        assert_eq!(e.mem.u32(file + 4), 12);
    }

    #[test]
    fn downward_passes_keep_the_time_and_the_flag() {
        let mut e = Engine::new();
        noop(&mut e, &[0x00a5_ded0, 0x00a5_dd70, 0x0090_1000]);
        let this = e.mem.alloc(0xb4);
        e.put_vtable(0x0090_0000, &[0x0090_1000; 0x30]);
        e.mem.set_u32(this, 0x0090_0000);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x7f80_0001);
        e.mem.set_u8(data + 4, 1);
        e.mem.set_u16(this + 0xac, 0x0003);
        e.call_log = Some(vec![]);
        e.call(0x00a7_da80, &[this, data, 5]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0xb0), 0x7fc0_0001);
        assert_eq!(e.mem.u16(this + 0xac), 0x0003);
        assert_eq!(calls_to(&log, 0x00a5_ded0), vec![vec![this, data, 5]]);
        assert_eq!(calls_to(&log, 0x0090_1000), vec![vec![this]]);
        e.mem.set_f32(data, 2.5);
        e.call_log = Some(vec![]);
        e.call(0x00a7_dae0, &[this, data, 6]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(this + 0xb0), 2.5);
        assert_eq!(e.mem.u16(this + 0xac), 0x000b);
        assert_eq!(calls_to(&log, 0x00a5_dd70), vec![vec![this, data, 6]]);
        e.mem.set_u8(data + 4, 0);
        e.call(0x00a7_dae0, &[this, data, 6]);
        assert_eq!(e.mem.u16(this + 0xac), 0x0003);
    }

    fn sqrt_double(e: &mut Engine) {
        e.register(SQUARE_ROOT, |_, a| {
            st0(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt())
        });
    }

    #[test]
    fn world_bound_recentres_and_grows_by_the_move() {
        let mut e = Engine::new();
        sqrt_double(&mut e);
        let this = e.mem.alloc(0xb4);
        let bound = e.mem.alloc(0x10);
        set_floats(&mut e, bound, &[1.0, 2.0, 3.0, 1.0]);
        e.mem.set_u32(this + 0x20, bound);
        set_floats(&mut e, this + 0x8c, &[4.0, 6.0, 3.0]);
        // No children: nothing.
        e.call(0x00a7_e630, &[this]);
        assert_eq!(floats(&e, bound, 4), vec![1.0, 2.0, 3.0, 1.0]);
        e.mem.set_u16(this + 0xa8, 1);
        e.call(0x00a7_e630, &[this]);
        assert_eq!(floats(&e, bound, 4), vec![4.0, 6.0, 3.0, 6.0]);
    }

    #[test]
    fn world_bound_is_made_when_missing() {
        let mut e = Engine::new();
        sqrt_double(&mut e);
        let this = e.mem.alloc(0xb4);
        e.mem.set_u16(this + 0xa8, 1);
        set_floats(&mut e, this + 0x8c, &[3.0, 4.0, 0.0]);
        e.map(0x011f_426c, 0xc);
        let bound = e.mem.alloc(0x10);
        e.mem.write(bound, &[0xff; 0x10]);
        e.register_double(0x00aa_3e40, move |_, _| bound.into_ret());
        e.put_vtable(0x0090_0000, &[0x0090_1000; 0x1f]);
        e.mem.set_u32(this, 0x0090_0000);
        e.register(0x0090_1000, |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a7_e630, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00aa_3e40), vec![vec![0x011f_6238, 0x10]]);
        assert_eq!(calls_to(&log, 0x0090_1000), vec![vec![this, bound]]);
        assert_eq!(floats(&e, bound, 4), vec![3.0, 4.0, 0.0, 5.0]);
    }

    /// A camera object: identity rotation, translation `at`.
    fn camera(e: &mut Engine, at: [f32; 3]) -> u32 {
        let c = e.mem.alloc(0xa0);
        set_floats(e, c + 0x68, &IDENTITY);
        set_floats(e, c + 0x8c, &at);
        c
    }

    /// Doubles for the turn helpers of `00a7dc40`: normalising, the cross
    /// product, the angle, the rotation and the matrix product, which
    /// leaves vectors unchanged.
    fn turn_doubles(e: &mut Engine) {
        e.map(0x0101_7d00, 4);
        e.mem.set_f32(0x0101_7d00, 1.0);
        e.map(0x0109_deb0, 8);
        e.set_global(0x0109_deb0, 0.999_999f64);
        e.register(0x004a_0c10, |e, a| {
            let v = floats(e, a[0], 3);
            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            set_floats(e, a[0], &[v[0] / n, v[1] / n, v[2] / n]);
            Ret::default()
        });
        e.register(0x0053_d1a0, |e, a| {
            set_floats(e, a[1], &[0.0, 0.0, 1.0]);
            Ret::default()
        });
        e.register(0x00a5_8510, |_, _| st0(0.5));
        e.register(0x0041_68a0, |_, _| Ret::default());
        e.register(0x004b_4500, |_, a| a[2].into_ret());
    }

    #[test]
    fn camera_turn_answers_false_when_too_close() {
        let mut e = Engine::new();
        turn_doubles(&mut e);
        let cam = camera(&mut e, [0.5, 0.0, 0.0]);
        let pos = e.mem.alloc(0xc);
        let out = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00a7_dc40, &[cam, pos, out, out + 0xc, out + 0x18])
            .bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x004a_0c10).is_empty());
    }

    #[test]
    fn camera_turn_keeps_the_columns_when_already_facing() {
        let mut e = Engine::new();
        turn_doubles(&mut e);
        // The camera looks down -x (column 0 negated) and sits at -x: the
        // direction to it, (-1, 0, 0), is "forward".
        let cam = camera(&mut e, [-5.0, 0.0, 0.0]);
        let pos = e.mem.alloc(0xc);
        let out = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a7_dc40, &[cam, pos, out, out + 0xc, out + 0x18])
            .bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, 0x0053_d1a0).is_empty());
        assert_eq!(
            floats(&e, out, 9),
            vec![-1.0, -0.0, -0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn camera_turn_rotates_onto_the_direction() {
        let mut e = Engine::new();
        turn_doubles(&mut e);
        let cam = camera(&mut e, [0.0, 3.0, 0.0]);
        let pos = e.mem.alloc(0xc);
        let out = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a7_dc40, &[cam, pos, out, out + 0xc, out + 0x18])
            .bool());
        let log = e.call_log.take().unwrap();
        // forward . d = -0 < 0.999999: the axis, the angle of the dot, the
        // rotation with the angle and the axis.
        assert_eq!(calls_to(&log, 0x00a5_8510), vec![vec![0x8000_0000]]);
        let rotation = calls_to(&log, 0x0041_68a0);
        assert_eq!(
            &rotation[0][1..],
            &[0.5f32.to_bits(), 0, 0, 1.0f32.to_bits()]
        );
        assert_eq!(calls_to(&log, 0x004b_4500).len(), 2);
        // forward becomes the normalised direction.
        assert_eq!(floats(&e, out, 3), vec![0.0, 1.0, 0.0]);
    }

    const START: [f32; 9] = [9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];

    /// A node in billboard `mode` with two child slots (one empty) and a
    /// camera at `at`; the transform helpers are identities.
    fn rotation_setup(mode: u16, at: [f32; 3]) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        sqrt_double(&mut e);
        turn_doubles(&mut e);
        e.map(START_MATRIX, 0x24);
        set_floats(&mut e, START_MATRIX, &START);
        for global in [MIN_LENGTH_010959F4, MIN_LENGTH_0109DEB8] {
            e.map(global, 4);
            e.mem.set_f32(global, 1.0e-6);
        }
        e.register(0x00a8_6bf0, |_, _| Ret::default());
        e.register(0x0062_c250, |e, a| {
            set_floats(e, a[1], &IDENTITY);
            set_floats(e, a[1] + 0x24, &[0.0, 0.0, 0.0, 1.0]);
            a[1].into_ret()
        });
        e.register(VECTOR_TIMES_MATRIX, |e, a| {
            let v = floats(e, a[1], 3);
            set_floats(e, a[0], &v);
            a[0].into_ret()
        });
        e.register(VECTOR_SCALED, |e, a| {
            let v = floats(e, a[0], 3);
            set_floats(e, a[1], &v);
            Ret::default()
        });
        // The product hands back M itself.
        e.register(0x0043_f8d0, |_, a| a[2].into_ret());
        let this = e.mem.alloc(0xb4);
        e.mem.set_u16(this + 0xac, mode | 8);
        e.mem.set_f32(this + 0xb0, 1.5);
        let children = e.mem.alloc(8);
        let child = e.mem.alloc(4);
        e.put_vtable(0x0090_0000, &[0x0090_1000; 0x2a]);
        e.mem.set_u32(child, 0x0090_0000);
        e.register(0x0090_1000, |e, a| {
            // The update record: time, then flag bit 3.
            assert_eq!(e.mem.f32(a[1]), 1.5);
            assert_eq!(e.mem.u8(a[1] + 4), 1);
            assert_eq!(e.mem.u32(a[1] + 5), 0);
            Ret::default()
        });
        e.mem.set_u32(children + 4, child);
        e.mem.set_u32(this + 0xa0, children);
        e.mem.set_u16(this + 0xa6, 2);
        let cam = camera(&mut e, at);
        (e, this, cam, child)
    }

    fn rotate(mode: u16, at: [f32; 3]) -> Vec<f32> {
        let (mut e, this, cam, child) = rotation_setup(mode, at);
        e.call_log = Some(vec![]);
        e.call(0x00a7_de40, &[this, cam]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0090_1000).len(), 1);
        assert_eq!(calls_to(&log, 0x0090_1000)[0][0], child);
        assert_eq!(calls_to(&log, 0x0090_1000)[0][2], 0);
        assert_eq!(calls_to(&log, 0x00a8_6bf0).len(), 1);
        floats(&e, this + 0x68, 9)
    }

    fn close(a: &[f32], b: &[f32]) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1.0e-6)
    }

    #[test]
    fn mode_0_faces_the_camera_columns() {
        let m = rotate(0, [0.0, 0.0, 0.0]);
        assert!(
            close(&m, &[0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0]),
            "{m:?}"
        );
    }

    #[test]
    fn mode_0_negates_up_and_right_when_up_has_no_y_z_part() {
        let (mut e, this, cam, _) = rotation_setup(0, [0.0; 3]);
        // Column 1 = (1, 0, 0), column 2 = (0, 0, 1): B1 = C1 = 0.
        set_floats(
            &mut e,
            cam + 0x68,
            &[1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        );
        e.call(0x00a7_de40, &[this, cam]);
        let m = floats(&e, this + 0x68, 9);
        assert!(
            close(&m, &[-0.0, -1.0, -1.0, -0.0, -0.0, -0.0, -1.0, -0.0, -0.0]),
            "{m:?}"
        );
    }

    #[test]
    fn mode_1_turns_about_y() {
        let m = rotate(1, [3.0, 7.0, 4.0]);
        assert!(
            close(&m, &[0.8, 0.0, 0.6, 0.0, 1.0, 0.0, -0.6, 0.0, 0.8]),
            "{m:?}"
        );
        // Straight above: no x-z length, M stays the start matrix.
        let m = rotate(1, [0.0, 7.0, 0.0]);
        assert_eq!(m, START.to_vec());
    }

    #[test]
    fn mode_2_takes_the_columns() {
        let m = rotate(2, [0.0, 0.0, 0.0]);
        assert!(
            close(&m, &[0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0]),
            "{m:?}"
        );
    }

    #[test]
    fn modes_3_and_4_keep_the_start_matrix_when_the_camera_is_close() {
        assert_eq!(rotate(3, [0.1, 0.0, 0.0]), START.to_vec());
        assert_eq!(rotate(4, [0.1, 0.0, 0.0]), START.to_vec());
    }

    #[test]
    fn mode_5_turns_about_z() {
        let m = rotate(5, [3.0, 4.0, 9.0]);
        assert!(
            close(&m, &[-0.8, 0.6, 0.0, 0.6, 0.8, 0.0, 0.0, 0.0, 1.0]),
            "{m:?}"
        );
    }

    #[test]
    fn other_modes_keep_the_start_matrix() {
        assert_eq!(rotate(6, [3.0, 4.0, 9.0]), START.to_vec());
        assert_eq!(rotate(7, [3.0, 4.0, 9.0]), START.to_vec());
    }

    #[test]
    fn on_visible_turns_to_the_cullers_camera() {
        let (mut e, this, cam, _) = rotation_setup(7, [0.0; 3]);
        noop(&mut e, &[0x00a5_dbe0]);
        let culler = e.mem.alloc(0x10);
        e.mem.set_u32(culler + 0xc, cam);
        e.call_log = Some(vec![]);
        e.call(0x00a7_e610, &[this, culler]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00a5_dbe0), vec![vec![this, culler]]);
        assert_eq!(calls_to(&log, 0x0043_f8d0).len(), 1);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    #[test]
    fn update_rigid_downward_pass_keeps_the_time() {
        let mut e = Engine::new();
        e.register(0x00a5_e1f0, |_, _| Ret::default());
        e.register(0x0300_00bc, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x30];
        slots[0x2f] = 0x0300_00bc;
        e.put_vtable(0x0300_1000, &slots);
        let this = e.mem.alloc(0xb4);
        e.mem.set_u32(this, 0x0300_1000);
        let data = e.mem.alloc(4);
        e.mem.set_f32(data, 2.5);
        e.call_log = Some(vec![]);
        e.call(0x00a7_dab0, &args![this, data, 7u32]);
        assert_eq!(e.mem.f32(this + 0xb0), 2.5);
        assert_eq!(calls_to(&e, 0x00a5_e1f0), vec![vec![this, data, 7]]);
        assert_eq!(calls_to(&e, 0x0300_00bc), vec![vec![this]]);
    }

    #[test]
    fn is_equal_compares_the_mode() {
        let mut e = Engine::new();
        e.register(NODE_IS_EQUAL, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0xb4);
        let other = e.mem.alloc(0xb4);
        e.mem.set_u16(this + 0xac, 0xf3);
        e.mem.set_u16(other + 0xac, 0x0b);
        assert!(e.call(0x00a7_db30, &args![this, other]).bool());
        e.mem.set_u16(other + 0xac, 0x0c);
        assert!(!e.call(0x00a7_db30, &args![this, other]).bool());
        e.mem.set_u16(other + 0xac, 0x0b);
        e.register(NODE_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a7_db30, &args![this, other]).bool());
    }

    #[test]
    fn viewer_string_by_mode() {
        let mut e = Engine::new();
        e.register(ALLOCATE_TEXT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"mode");
        e.call_log = Some(vec![]);
        for (mode, format) in [(0u32, 0x0109_de98u32), (2, 0x0109_de68), (5, 0x0109_de20)] {
            let text = e.call(0x00a7_db60, &args![name, mode]).u32();
            assert_eq!(
                calls_to(&e, NI_SPRINTF).last().unwrap(),
                &vec![text, 0x1a, format, name]
            );
        }
        let text = e.call(0x00a7_db60, &args![name, 6u32]).u32();
        assert_ne!(text, 0);
        assert_eq!(calls_to(&e, NI_SPRINTF).len(), 3);
        assert_eq!(calls_to(&e, ALLOCATE_TEXT).len(), 4);
    }

    #[test]
    fn save_binary_writes_the_flags() {
        let mut e = Engine::new();
        e.register(0x00a5_e880, |_, _| Ret::default());
        e.register(0x0300_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (2, 2, 1));
            2u32.into_ret()
        });
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x0300_0000);
        let this = e.mem.alloc(0xb4);
        e.call_log = Some(vec![]);
        e.call(0x00a7_e850, &args![this, stream]);
        assert_eq!(calls_to(&e, 0x00a5_e880), vec![vec![this, stream]]);
        assert_eq!(calls_to(&e, 0x0300_0000)[0][..2], [output, this + 0xac]);
        assert_eq!(e.mem.u32(output + 4), 2);
    }

    #[test]
    fn viewer_strings_add_the_class_and_mode() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x0109_d000, 0x1000);
        e.mem.set_u32(0x011f_4aa0, 0x0109_d100);
        e.register(0x00a5_ea70, |_, _| Ret::default());
        e.register(0x00a7_d330, |_, _| 0x7000u32.into_ret());
        e.register(ALLOCATE_TEXT, |_, _| 0x8000u32.into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            let count = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, count);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(0x011f_4f00 + 4 * a[1], value);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 4);
        let this = e.mem.alloc(0xb4);
        e.mem.set_u16(this + 0xac, 0x0b);
        e.call_log = Some(vec![]);
        e.call(0x00a7_e890, &args![this, strings]);
        assert_eq!(calls_to(&e, 0x00a7_d330), vec![vec![0x0109_d100]]);
        assert_eq!(calls_to(&e, NI_SPRINTF)[0][2..], [0x0109_de50, 0x0109_debc]);
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![strings, 5]]);
        assert_eq!(
            (e.mem.u32(0x011f_4f00), e.mem.u32(0x011f_4f04)),
            (0x7000, 0x8000)
        );
    }
}
