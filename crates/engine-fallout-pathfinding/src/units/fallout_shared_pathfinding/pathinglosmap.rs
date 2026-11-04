//! `fallout shared/pathfinding/pathinglosmap.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e20e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map base constructor: installs its vtable, allocates buckets and
/// clears them. The bucket count is reread after allocation.
pub fn construct_base(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0106cf88);
    e.mem.set_u32(this.addr().wrapping_add(4), buckets);
    e.mem.set_u32(this.addr().wrapping_add(12), 0);
    let size = e.mem.u32(this.addr().wrapping_add(4)).wrapping_mul(4);
    let table = e.call(0x00aa1070, &args![size]).u32();
    e.mem.set_u32(this.addr().wrapping_add(8), table);
    let size = e.mem.u32(this.addr().wrapping_add(4)).wrapping_mul(4);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    e.call(0x00403d30, &args![table, 0u32, size]);
    this
}

// Translated from 006e2080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map constructor: constructs the base then installs its vtable.
pub fn construct(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    construct_base(e, this, buckets);
    e.mem.set_u32(this.addr(), 0x0106cf68);
    this
}

/// `NiTMap<unsigned int, unsigned int>`'s vtable (`0106cf68`) and its
/// base `NiTMapBase<DFALL<NiTMapItem<unsigned int, unsigned int>>, ...>`'s
/// (`0106cf88`), by their RTTI.
const VTABLE_NI_T_MAP: u32 = 0x0106_cf68;
const VTABLE_NI_T_MAP_BASE: u32 = 0x0106_cf88;
/// `00438af0`: removes every item of the map (`thiscall`).
const REMOVE_ALL: u32 = 0x0043_8af0;
/// `00aa10f0`: frees the bucket table (`cdecl`, one word).
const FREE_TABLE: u32 = 0x00aa_10f0;
/// The game's `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 006e20b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, unsigned int>`'s scalar deleting destructor
/// (engine map): [`fn_006e2150`], then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn ni_t_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006e2150(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e2150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e2150 - the C++ exception frame (FS:[0]) is not translated
/// The `NiTMap<unsigned int, unsigned int>` destructor (by the vtable it
/// sets): removes every item (`00438af0`), then the base destructor
/// ([`fn_006e21b0`]).
pub fn fn_006e2150(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_T_MAP);
    e.call(REMOVE_ALL, &args![this]);
    fn_006e21b0(e, this);
}

// Translated from 006e21b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` destructor (by the vtable it sets): removes every item
/// (`00438af0`), then frees the bucket table at `+8` (`00aa10f0`).
pub fn fn_006e21b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_T_MAP_BASE);
    e.call(REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr() + 8);
    e.call(FREE_TABLE, &args![table]);
}

// ---- Batch n032: the LOS map --------------------------------------------------

layout! {
    /// `PathingLOSMap` (Xbox PDB), 0x50 bytes (the size its deleting
    /// destructor frees). `NavMeshToOffsetMap` (+0x08) is an
    /// `NiTMap<unsigned int,unsigned int>` and `NavMeshes` (+0x1C) a
    /// `BSSimpleArray<NavMeshPtr,1024>`.
    pub struct PathingLOSMap: 0x50 {
        /// `pTriangleScores` (Xbox PDB): one signed byte per triangle.
        0x18 pTriangleScores: u32,
        /// `iTotalTriangleCount` (Xbox PDB).
        0x2C iTotalTriangleCount: u32,
        /// `iTotalTrianglesSeen` (Xbox PDB).
        0x30 iTotalTrianglesSeen: u32,
        /// `iTotalTrianglesUnseen` (Xbox PDB).
        0x34 iTotalTrianglesUnseen: u32,
        /// `Center` (Xbox PDB): an `NiPoint3`.
        0x3C Center: Inline<crate::types::NiPoint3>,
        /// `fRadius` (Xbox PDB).
        0x48 fRadius: f32,
        /// `fSightRadius` (Xbox PDB).
        0x4C fSightRadius: f32,
    }
}

/// `PathingLOSMap::NavMeshes` (Xbox PDB).
const LOS_MAP_NAV_MESHES: u32 = 0x1c;
/// The `PathingLOSMap` vtable (by its RTTI).
const VTABLE_LOS_MAP: u32 = 0x0106_cf5c;
/// The heap free (`00aa1460`, `cdecl`: block, size).
const FREE_SIZED: u32 = 0x00aa_1460;
/// `MemoryManager` allocation (`00401000`, `cdecl`: size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `BSSimpleArray<NavMeshPtr,1024>` destructor (`0042f830`) and add
/// (`0042f850`, `RET 4`: the element's address).
const NAV_MESH_POINTERS_DESTRUCT: u32 = 0x0042_f830;
const NAV_MESH_POINTERS_ADD: u32 = 0x0042_f850;
/// `NiRefObject` destructor body (`00496910`).
const REF_OBJECT_DESTRUCT: u32 = 0x0049_6910;
/// `0044ddc0` (`thiscall`): a `BSSimpleArray`'s size (the word at +8).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `00877a30` (`thiscall`, `RET 4`): the address of word element `i`.
const WORD_ELEMENT: u32 = 0x0087_7a30;
/// `00559450` (`thiscall`): the word at `this` (an `NiPointer`'s target).
const WORD_AT: u32 = 0x0055_9450;
/// `0084e3a0` (`thiscall`): a navmesh's key in the offset map.
const NAV_MESH_KEY: u32 = 0x0084_e3a0;
/// `00558200` (`thiscall`): a navmesh's triangle count (the size of its
/// array at +0x38).
const NAV_MESH_TRIANGLE_COUNT: u32 = 0x0055_8200;
/// `NiTMapBase<...CombatThreat *>::SetAt` (engine map; folded),
/// `thiscall`, `RET 8`: (key, value).
const MAP_SET_AT: u32 = 0x0084_4700;
/// `00853130` (`thiscall`, `RET 8`): an `NiTMap` lookup (key, value out);
/// answers whether the key is there.
const MAP_GET_AT: u32 = 0x0085_3130;
/// `0076b610` (`thiscall`): whether a `BSSimpleArray` is empty.
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// `0042f4c0` (`thiscall`, `RET 4`): `NiPointer` assignment from another.
const NI_POINTER_ASSIGN: u32 = 0x0042_f4c0;
/// `0055b980` (`thiscall`): `iTotalTriangleCount` (+0x2C).
const TOTAL_TRIANGLE_COUNT: u32 = 0x0055_b980;
/// `006d7ee0` (`thiscall`, `RET 4`): the score byte of a triangle; and
/// `006d8970` (`thiscall`, `RET 8`): sets it.
const TRIANGLE_SCORE: u32 = 0x006d_7ee0;
const SET_TRIANGLE_SCORE: u32 = 0x006d_8970;
/// `0060bef0` (`thiscall`, `RET 4`): sizes a node's children array (+0x9C).
const CHILDREN_SET_SIZE: u32 = 0x0060_bef0;
/// The `BSSimpleArray<unsigned int,1024>` constructor (`00438660`) and its
/// `AddUninitialized`-named add (`007cb2e0`, engine map; folded, `RET 4`:
/// the address of the value).
const UINT_ARRAY_CONSTRUCT: u32 = 0x0043_8660;
const UINT_ARRAY_ADD: u32 = 0x007c_b2e0;

// The debug geometry's callees (006e1800).
/// `00aa13e0` (`cdecl`: size): the allocation the Gamebryo objects use.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00a5ecb0` (`thiscall`, `RET 4`): the `NiNode` constructor (child count).
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `004391c0` (`thiscall`): the constructor of the first property (0x1C bytes).
const FIRST_PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
/// `00633c90` (`thiscall`, `RET 4`): an `NiPointer` from a pointer;
/// `0045cec0`: its destructor.
const NI_POINTER_FROM: u32 = 0x0063_3c90;
const NI_POINTER_RELEASE: u32 = 0x0045_cec0;
/// The null `NiPointer` constructor (`0042fb00`) and its destructor (`0042fa40`).
const NI_POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const NI_POINTER_DESTRUCT: u32 = 0x0042_fa40;
/// `0049ed90` (`thiscall`, `RET 4`): set on the first property with 1.
const FIRST_PROPERTY_SET: u32 = 0x0049_ed90;
/// `NiMaterialProperty::NiMaterialProperty` (engine map), 0x4C bytes.
const MATERIAL_PROPERTY_CONSTRUCT: u32 = 0x00a7_5650;
/// `00416870` (`thiscall`, `RET 0xC`): an `NiColor` from three floats.
const COLOR_CONSTRUCT: u32 = 0x0041_6870;
/// `004bc450` (`thiscall`, `RET 4`): the material's colour from an `NiColor`.
const MATERIAL_SET_COLOR: u32 = 0x004b_c450;
/// `0068c9f0` (`thiscall`, `RET 4`): the material's alpha.
const MATERIAL_SET_ALPHA: u32 = 0x0068_c9f0;
/// `NiAVObject::AttachProperty` (engine map), `thiscall`, `RET 4`.
const ATTACH_PROPERTY: u32 = 0x0043_9410;
/// `00414430` (`thiscall`, `RET 0x10`): an `NiColorA` from four floats.
const COLOR_A_CONSTRUCT: u32 = 0x0041_4430;
/// `00401050` (`stdcall`, `RET 0x10`): the compiler's vector constructor
/// iterator (array, element size, count, constructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `006815c0`: the `NiPoint3` constructor the iterator runs.
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// `005582f0` (`thiscall`, `RET 4`): a navmesh's triangle by index.
const NAV_MESH_TRIANGLE: u32 = 0x0055_82f0;
/// `00691140` (`thiscall`, `RET 4`): a triangle flag test.
const TRIANGLE_HAS_FLAG: u32 = 0x0069_1140;
/// `005582d0` (`thiscall`, `RET 4`): a triangle's vertex index (16 bits).
const TRIANGLE_VERTEX: u32 = 0x0055_82d0;
/// `0068f0a0` (`thiscall`, `RET 4`): a navmesh's vertex position.
const NAV_MESH_VERTEX: u32 = 0x0068_f0a0;
/// `MakeTriangle` (engine map), `cdecl`: three points by value, a colour,
/// a flag.
const MAKE_TRIANGLE: u32 = 0x004b_3570;
/// `00c58f00` (`cdecl`: radius, segments, colour) and `00c58d30` (`cdecl`:
/// three floats, two counts, colour): the circle and the marker geometry.
const MAKE_CIRCLE: u32 = 0x00c5_8f00;
const MAKE_MARKER: u32 = 0x00c5_8d30;
/// `004bc1f0` (`thiscall`, `RET 0xC`): an object's translation from x, y, z.
const SET_TRANSLATE_XYZ: u32 = 0x004b_c1f0;
/// `00440460` (`thiscall`, `RET 4`): an object's translation from a point.
const SET_TRANSLATE: u32 = 0x0044_0460;
/// `AddNoLightingPropertyRecurse` (engine map), `cdecl`.
const ADD_NO_LIGHTING_PROPERTY_RECURSE: u32 = 0x004b_6e90;
/// `BSShaderManager::PrepareObject` (engine map), `cdecl`.
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// `NiAVObject::UpdateProperties` (engine map), `thiscall`.
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `0043d410` (`thiscall`, `RET 0xC`): the update data (time, two words).
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// `00a59c60` (`thiscall`, `RET 4`): the node's update with the update data.
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `NiNode`'s child attach slot (vtable +0xDC: child, flag).
const ATTACH_CHILD_SLOT: u32 = 0xdc;
/// The constants it reads: `float`s at `01016264` (the material's and the
/// triangles' alpha), `01016248` (the circles' alpha), `0101e57c` and
/// `010311a8` (the marker's sizes); `double`s at `01011588` (the score
/// scale and offset), `01020998` (the height added to each vertex),
/// `01012060` (the sight radius compared against) and `0104f2f0` (the
/// circles' height step).
const ALPHA: u32 = 0x0101_6264;
const CIRCLE_ALPHA: u32 = 0x0101_6248;
const MARKER_SIZE: u32 = 0x0101_e57c;
const MARKER_HEIGHT: u32 = 0x0103_11a8;
const SCORE_SCALE: u32 = 0x0101_1588;
const VERTEX_RAISE: u32 = 0x0102_0998;
const NO_SIGHT_RADIUS: u32 = 0x0101_2060;
const CIRCLE_STEP: u32 = 0x0104_f2f0;

/// The `float` bits of `FLD float [address]; FSTP float [ESP]` (a
/// signalling NaN goes quiet).
fn pushed_float(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

// Translated from 006e1440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingLOSMap::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor ([`fn_006e1470`]), then frees 0x50 bytes when bit 0 of
/// `flags` is set. Returns `this`.
pub fn pathing_los_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PathingLOSMap>,
    flags: u32,
) -> Ptr<PathingLOSMap> {
    fn_006e1470(e, this);
    if flags & 1 != 0 {
        e.call(FREE_SIZED, &args![this, 0x50u32]);
    }
    this
}

// Translated from 006e1470 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e1470 - the C++ exception frame (FS:[0]) that finishes the member destructors on unwinding is not translated
/// The `PathingLOSMap` destructor (by the vtable it sets): deletes the
/// triangle scores when set, then destroys the navmesh array, the offset
/// map ([`fn_006e2150`]) and the `NiRefObject` base.
pub fn fn_006e1470(e: &mut Engine, this: Ptr<PathingLOSMap>) {
    e.mem.set_u32(this.addr(), VTABLE_LOS_MAP);
    let scores = e.get(this, PathingLOSMap::pTriangleScores);
    if scores != 0 {
        e.call(OPERATOR_DELETE, &args![scores]);
    }
    e.call(
        NAV_MESH_POINTERS_DESTRUCT,
        &args![this.byte_add(LOS_MAP_NAV_MESHES)],
    );
    fn_006e2150(e, this.byte_add(8));
    e.call(REF_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 006e1500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSMap`): sets the map up from a navmesh array: each
/// navmesh's key maps to the running triangle total (its offset in the
/// scores), the total grows by its triangle count (`00558200`) and the
/// navmesh is added to `NavMeshes`; then the old scores are deleted and new
/// ones allocated (one byte per triangle), all `0x7F`. Returns true.
pub fn fn_006e1500(e: &mut Engine, this: Ptr<PathingLOSMap>, nav_meshes: Ptr) -> bool {
    e.set(this, PathingLOSMap::iTotalTriangleCount, 0);
    let mut i = 0u32;
    while i < e.call(ARRAY_SIZE, &args![nav_meshes]).u32() {
        let offset = e.get(this, PathingLOSMap::iTotalTriangleCount);
        let slot = e.call(WORD_ELEMENT, &args![nav_meshes, i]).u32();
        let mesh = e.call(WORD_AT, &args![slot]).u32();
        let key = e.call(NAV_MESH_KEY, &args![mesh]).u32();
        e.call(MAP_SET_AT, &args![this.byte_add(8), key, offset]);
        let slot = e.call(WORD_ELEMENT, &args![nav_meshes, i]).u32();
        let mesh = e.call(WORD_AT, &args![slot]).u32();
        let count = e.call(NAV_MESH_TRIANGLE_COUNT, &args![mesh]).u32();
        let total = count.wrapping_add(e.get(this, PathingLOSMap::iTotalTriangleCount));
        e.set(this, PathingLOSMap::iTotalTriangleCount, total);
        let slot = e.call(WORD_ELEMENT, &args![nav_meshes, i]).u32();
        e.call(
            NAV_MESH_POINTERS_ADD,
            &args![this.byte_add(LOS_MAP_NAV_MESHES), slot],
        );
        i = i.wrapping_add(1);
    }
    let scores = e.get(this, PathingLOSMap::pTriangleScores);
    if scores != 0 {
        e.call(OPERATOR_DELETE, &args![scores]);
    }
    let total = e.get(this, PathingLOSMap::iTotalTriangleCount);
    let scores = e.call(ALLOCATE, &args![total]).u32();
    e.set(this, PathingLOSMap::pTriangleScores, scores);
    let mut j = 0u32;
    while j < e.get(this, PathingLOSMap::iTotalTriangleCount) {
        let scores = e.get(this, PathingLOSMap::pTriangleScores);
        e.mem.set_u8(scores.wrapping_add(j), 0x7f);
        j = j.wrapping_add(1);
    }
    true
}

// Translated from 006e1610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSMap`): the address of a navmesh's first score (the
/// scores plus the key's offset), 0 when the key is not in the map.
pub fn fn_006e1610(e: &mut Engine, this: Ptr<PathingLOSMap>, key: u32) -> u32 {
    e.with_stack(4, |e, offset| {
        if e.call(MAP_GET_AT, &args![this.byte_add(8), key, offset])
            .bool()
        {
            let scores = e.get(this, PathingLOSMap::pTriangleScores);
            scores.wrapping_add(e.mem.u32(offset.addr()))
        } else {
            0
        }
    })
}

// Translated from 006e1650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSMap`): the offset of a key into the out word
/// (`00853130` on the offset map); returns that lookup's result word.
pub fn fn_006e1650(e: &mut Engine, this: Ptr<PathingLOSMap>, key: u32, offset: Ptr) -> u32 {
    e.call(MAP_GET_AT, &args![this.byte_add(8), key, offset])
        .u32()
}

// Translated from 006e1670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSMap`): the navmesh and triangle of a score index.
/// False with no navmeshes, or when a navmesh's key is missing from the
/// offset map. Walks the navmeshes until one starts past `index`; the
/// previous one (index `i - 1`, wrapping) goes into the `NiPointer` out
/// argument (`0042f4c0`) and `index` minus its offset into the 16-bit
/// triangle; past the last navmesh the last one is taken the same way.
pub fn fn_006e1670(
    e: &mut Engine,
    this: Ptr<PathingLOSMap>,
    index: u32,
    nav_mesh: Ptr,
    triangle: Ptr,
) -> bool {
    let nav_meshes = this.byte_add(LOS_MAP_NAV_MESHES);
    if e.call(ARRAY_IS_EMPTY, &args![nav_meshes]).bool() {
        return false;
    }
    let mut previous = 0u32;
    let mut i = 0u32;
    let offset_slot = e.mem.alloc(4);
    loop {
        if i >= e.call(ARRAY_SIZE, &args![nav_meshes]).u32() {
            break;
        }
        let slot = e.call(WORD_ELEMENT, &args![nav_meshes, i]).u32();
        let mesh = e.call(WORD_AT, &args![slot]).u32();
        let key = e.call(NAV_MESH_KEY, &args![mesh]).u32();
        if !e
            .call(MAP_GET_AT, &args![this.byte_add(8), key, offset_slot])
            .bool()
        {
            e.mem.free(offset_slot);
            return false;
        }
        let offset = e.mem.u32(offset_slot);
        if offset > index {
            break;
        }
        previous = offset;
        i = i.wrapping_add(1);
    }
    e.mem.free(offset_slot);
    e.mem
        .set_u16(triangle.addr(), index.wrapping_sub(previous) as u16);
    let slot = e
        .call(WORD_ELEMENT, &args![nav_meshes, i.wrapping_sub(1)])
        .u32();
    e.call(NI_POINTER_ASSIGN, &args![nav_mesh, slot]);
    true
}

// Translated from 006e1770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSMap`): false with no navmeshes; else the first
/// triangle whose score is not positive (signed) and whose navmesh and
/// triangle [`fn_006e1670`] finds goes into the out arguments: true;
/// false when there is none.
pub fn fn_006e1770(e: &mut Engine, this: Ptr<PathingLOSMap>, nav_mesh: Ptr, triangle: Ptr) -> bool {
    if e.call(ARRAY_IS_EMPTY, &args![this.byte_add(LOS_MAP_NAV_MESHES)])
        .bool()
    {
        return false;
    }
    let mut i = 0u32;
    while i < e.call(TOTAL_TRIANGLE_COUNT, &args![this]).u32() {
        let score = e.call(TRIANGLE_SCORE, &args![this, i]).u8() as i8;
        if score <= 0 && fn_006e1670(e, this, i, nav_mesh, triangle) {
            return true;
        }
        i = i.wrapping_add(1);
    }
    false
}

/// The colour channel of a score: `score / bound * 0.5 + 0.5` on the x87
/// stack (`FILD`, `FIDIV`, `FMUL`/`FADD double`), stored as a `double`
/// then as a `float`.
fn score_channel(e: &Engine, score: i8, bound: i8) -> u32 {
    let scale: f64 = e.global(SCORE_SCALE);
    let ratio = x87_div(f64::from(score), f64::from(bound));
    let value = x87_add(x87_mul(ratio, scale), scale);
    x87_store32(value)
}

// Translated from 006e1800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e1800 - the C++ exception frame (FS:[0]) that frees the objects and releases the NiPointers on unwinding is not translated
/// Unnamed (`PathingLOSMap`): builds the LOS map's debug geometry and
/// returns its node. A new `NiNode` (0xAC bytes) gets two properties (a
/// 0x1C-byte one set with 1, and a material of colour 1,1,1 and the alpha
/// at `01016264`). Every triangle with a score (not `0x7F`) is drawn: its
/// colour's red is the negative scores' share of the lowest score, green
/// the positive scores' share of the highest (`score / bound * 0.5 +
/// 0.5`), blue 0; the triangle is raised by the constant at `01020998`
/// and skipped when it has flag `0x20`. With an `fRadius` other than
/// the constant at `01012060` (NaN included), a second node gets five
/// circles of that radius (its `float` bits), stacked upwards from the
/// center's z, and a marker at the center. Then the node's properties and an update.
pub fn fn_006e1800(e: &mut Engine, this: Ptr<PathingLOSMap>) -> u32 {
    // The function's frame: the locals the callees reach by address, at
    // their `EBP` offsets.
    const FRAME: u32 = 0x100;
    let frame = e.mem.alloc(FRAME);
    let local = |offset: u32| frame + FRAME - offset;
    let (first, material) = (local(0x18), local(0x14));

    let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    let property = if block != 0 {
        e.call(FIRST_PROPERTY_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(NI_POINTER_FROM, &args![first, property]);
    let target = e.call(WORD_AT, &args![first]).u32();
    e.call(FIRST_PROPERTY_SET, &args![target, 1u32]);
    let block = e.call(NI_ALLOCATE, &args![0x4cu32]).u32();
    let property = if block != 0 {
        e.call(MATERIAL_PROPERTY_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(NI_POINTER_FROM, &args![material, property]);
    let one = 1.0f32.to_bits();
    let color = e
        .call(COLOR_CONSTRUCT, &args![local(0xd0), one, one, one])
        .u32();
    let target = e.call(WORD_AT, &args![material]).u32();
    e.call(MATERIAL_SET_COLOR, &args![target, color]);
    let alpha = pushed_float(e, ALPHA);
    let target = e.call(WORD_AT, &args![material]).u32();
    e.call(MATERIAL_SET_ALPHA, &args![target, alpha]);
    let target = e.call(WORD_AT, &args![first]).u32();
    e.call(ATTACH_PROPERTY, &args![node, target]);
    let target = e.call(WORD_AT, &args![material]).u32();
    e.call(ATTACH_PROPERTY, &args![node, target]);

    // The highest and lowest scores (signed bytes; 0x7F is "no score").
    let mut highest = 0x81u8 as i8;
    let mut lowest = 0x7fi8;
    let mut i = 0u32;
    while i < e.call(TOTAL_TRIANGLE_COUNT, &args![this]).u32() {
        let score = e.call(TRIANGLE_SCORE, &args![this, i]).u8() as i8;
        if score != 0x7f {
            highest = fn_006e1fb0(e, highest, score);
            lowest = fn_006e1f80(e, lowest, score);
        }
        i = i.wrapping_add(1);
    }
    let count = e.call(TOTAL_TRIANGLE_COUNT, &args![this]).u32();
    fn_006e1fe0(e, Ptr::new(node), count.wrapping_add(10));

    let (mesh_pointer, triangle_slot) = (local(0x60), local(0x5c));
    let (color_a, points, geometry) = (local(0x54), local(0x84), local(0x88));
    let mut i = 0u32;
    while i < e.call(TOTAL_TRIANGLE_COUNT, &args![this]).u32() {
        e.call(NI_POINTER_CONSTRUCT, &args![mesh_pointer]);
        if !fn_006e1670(e, this, i, Ptr::new(mesh_pointer), Ptr::new(triangle_slot)) {
            e.call(NI_POINTER_DESTRUCT, &args![mesh_pointer]);
            i = i.wrapping_add(1);
            continue;
        }
        let score = e.call(TRIANGLE_SCORE, &args![this, i]).u8() as i8;
        if score == 0x7f {
            e.call(NI_POINTER_DESTRUCT, &args![mesh_pointer]);
            i = i.wrapping_add(1);
            continue;
        }
        let red = if score < 0 {
            score_channel(e, score, lowest)
        } else {
            0
        };
        let green = if score > 0 {
            score_channel(e, score, highest)
        } else {
            0
        };
        let alpha = pushed_float(e, ALPHA);
        e.call(COLOR_A_CONSTRUCT, &args![color_a, red, green, 0u32, alpha]);
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![points, 0xcu32, 3u32, POINT_CONSTRUCT],
        );
        let triangle_index = u32::from(e.mem.u16(triangle_slot));
        let mesh = e.call(WORD_AT, &args![mesh_pointer]).u32();
        let triangle = e
            .call(NAV_MESH_TRIANGLE, &args![mesh, triangle_index])
            .u32();
        if !e.call(TRIANGLE_HAS_FLAG, &args![triangle, 0x20u32]).bool() {
            let raise: f64 = e.global(VERTEX_RAISE);
            for k in 0..3u32 {
                let vertex = e.call(TRIANGLE_VERTEX, &args![triangle, k]).u32() & 0xffff;
                let mesh = e.call(WORD_AT, &args![mesh_pointer]).u32();
                let position = e.call(NAV_MESH_VERTEX, &args![mesh, vertex]).u32();
                let point = points + 0xc * k;
                for w in 0..3 {
                    let word = e.mem.u32(position + 4 * w);
                    e.mem.set_u32(point + 4 * w, word);
                }
                let z = x87_add(x87_load32(e.mem.u32(point + 8)), raise);
                e.mem.set_u32(point + 8, x87_store32(z));
            }
            let mut words = Vec::with_capacity(11);
            for w in 0..9 {
                words.push(e.mem.u32(points + 4 * w));
            }
            words.push(color_a);
            words.push(1);
            let made = e.call(MAKE_TRIANGLE, &words).u32();
            e.call(NI_POINTER_FROM, &args![geometry, made]);
            let child = e.call(WORD_AT, &args![geometry]).u32();
            e.vcall(node, ATTACH_CHILD_SLOT, &args![child, 1u32]);
            e.call(NI_POINTER_RELEASE, &args![geometry]);
        }
        e.call(NI_POINTER_DESTRUCT, &args![mesh_pointer]);
        i = i.wrapping_add(1);
    }

    // FCOMP double; TEST AH,0x44; JNP: skipped only when equal.
    let no_sight: f64 = e.global(NO_SIGHT_RADIUS);
    let radius = e.get(this, PathingLOSMap::fRadius);
    if f64::from(radius) != no_sight {
        let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
        let circles = if block != 0 {
            e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
        } else {
            0
        };
        let circle_color = local(0x9c);
        let circle_alpha = pushed_float(e, CIRCLE_ALPHA);
        e.call(
            COLOR_A_CONSTRUCT,
            &args![circle_color, one, one, 0u32, circle_alpha],
        );
        let step: f64 = e.global(CIRCLE_STEP);
        for k in 0..5u32 {
            let radius = pushed_float(e, this.addr() + 0x48);
            let circle = e
                .call(MAKE_CIRCLE, &args![radius, 0x24u32, circle_color])
                .u32();
            // FILD qword (k zero-extended), FMUL double, FADD float [+0x44].
            let height = x87_add(
                x87_mul(f64::from(k), step),
                x87_m32(e.mem.u32(this.addr() + 0x44)),
            );
            let height = x87_store32(height);
            let x = pushed_float(e, this.addr() + 0x3c);
            let y = pushed_float(e, this.addr() + 0x40);
            e.call(SET_TRANSLATE_XYZ, &args![circle, x, y, height]);
            e.vcall(circles, ATTACH_CHILD_SLOT, &args![circle, 1u32]);
        }
        let size = pushed_float(e, MARKER_SIZE);
        let height = pushed_float(e, MARKER_HEIGHT);
        let marker = e
            .call(
                MAKE_MARKER,
                &args![size, size, height, 10u32, 10u32, circle_color],
            )
            .u32();
        e.call(SET_TRANSLATE, &args![marker, this.addr() + 0x3c]);
        e.vcall(circles, ATTACH_CHILD_SLOT, &args![marker, 1u32]);
        e.call(ADD_NO_LIGHTING_PROPERTY_RECURSE, &args![circles]);
        e.call(PREPARE_OBJECT, &args![circles, 0u32, 0u32]);
        e.vcall(node, ATTACH_CHILD_SLOT, &args![circles, 1u32]);
    }
    e.call(UPDATE_PROPERTIES, &args![node]);
    let update = local(0x28);
    e.call(UPDATE_DATA_CONSTRUCT, &args![update, 0u32, 0u32, 0u32]);
    e.call(NODE_UPDATE, &args![node, update]);
    e.call(NI_POINTER_RELEASE, &args![material]);
    e.call(NI_POINTER_RELEASE, &args![first]);
    e.mem.free(frame);
    node
}

// Translated from 006e1f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`): the smaller of two signed bytes (`b` when equal).
pub fn fn_006e1f80(_e: &mut Engine, a: i8, b: i8) -> i8 {
    if a < b {
        a
    } else {
        b
    }
}

// Translated from 006e1fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`): the larger of two signed bytes (`b` when equal).
pub fn fn_006e1fb0(_e: &mut Engine, a: i8, b: i8) -> i8 {
    if a > b {
        a
    } else {
        b
    }
}

// Translated from 006e1fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on an `NiNode`, by its caller): sizes the children
/// array at +0x9C (`0060bef0`) for `count`.
pub fn fn_006e1fe0(e: &mut Engine, this: Ptr, count: u32) {
    e.call(CHILDREN_SET_SIZE, &args![this.byte_add(0x9c), count]);
}

// Translated from 006e2000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSMap`): every triangle whose score is not positive
/// (signed) gets the score `0x7F` (`006d8970`).
pub fn fn_006e2000(e: &mut Engine, this: Ptr<PathingLOSMap>) {
    let mut i = 0u32;
    while i < e.call(TOTAL_TRIANGLE_COUNT, &args![this]).u32() {
        let score = e.call(TRIANGLE_SCORE, &args![this, i]).u8() as i8;
        if score <= 0 {
            e.call(SET_TRIANGLE_SCORE, &args![this, i, 0x7fu32]);
        }
        i = i.wrapping_add(1);
    }
}

// Translated from 006e2050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingLOSMap`): whether the offset map has the key (the
/// lookup's result word; the offset goes into a local and is dropped).
pub fn fn_006e2050(e: &mut Engine, this: Ptr<PathingLOSMap>, key: u32) -> u32 {
    e.with_stack(4, |e, offset| {
        e.call(MAP_GET_AT, &args![this.byte_add(8), key, offset])
            .u32()
    })
}

// Translated from 006e21e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int,unsigned int>>,unsigned int,unsigned int>::_scalar_deleting_destructor_`
/// (engine map): the base destructor ([`fn_006e21b0`]), then `operator
/// delete` when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006e21b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00852950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00852950 - the C++ exception frame (FS:[0]) that frees an array whose constructor throws is not translated
/// Named `PathingLOSMap::Setup` by the engine map, but the body works on
/// an `NiTMap` of `BSSimpleArray<unsigned int,1024>` pointers (`this`):
/// the array of `key` (`00853130`), made when missing (0x10 bytes,
/// `00438660`, null when the allocation fails) and set in the map
/// (`00844700`); then `value` is added to it (`007cb2e0`, which takes the
/// address of the argument word: here a stack word holding `value`).
pub fn fn_00852950(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let array = e.with_stack(4, |e, array| {
        if !e.call(MAP_GET_AT, &args![this, key, array]).bool() {
            let block = e.call(ALLOCATE, &args![0x10u32]).u32();
            let made = if block != 0 {
                e.call(UINT_ARRAY_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(array.addr(), made);
            e.call(MAP_SET_AT, &args![this, key, made]);
        }
        e.mem.u32(array.addr())
    });
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), value);
        e.call(UINT_ARRAY_ADD, &args![array, argument]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e2080, construct(Ptr, u32) -> Ptr),
        entry!(0x006e20e0, construct_base(Ptr, u32) -> Ptr),
        entry!(0x006e_20b0, ni_t_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006e_2150, fn_006e2150(Ptr)),
        entry!(0x006e_21b0, fn_006e21b0(Ptr)),
        entry!(
            0x006e_1440,
            pathing_los_map_scalar_deleting_destructor(
                Ptr<PathingLOSMap>,
                u32,
            ) -> Ptr<PathingLOSMap>
        ),
        entry!(0x006e_1470, fn_006e1470(Ptr<PathingLOSMap>)),
        entry!(0x006e_1500, fn_006e1500(Ptr<PathingLOSMap>, Ptr) -> bool),
        entry!(0x006e_1610, fn_006e1610(Ptr<PathingLOSMap>, u32) -> u32),
        entry!(
            0x006e_1650,
            fn_006e1650(Ptr<PathingLOSMap>, u32, Ptr) -> u32
        ),
        entry!(
            0x006e_1670,
            fn_006e1670(Ptr<PathingLOSMap>, u32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x006e_1770,
            fn_006e1770(Ptr<PathingLOSMap>, Ptr, Ptr) -> bool
        ),
        entry!(0x006e_1800, fn_006e1800(Ptr<PathingLOSMap>) -> u32),
        entry!(0x006e_1f80, fn_006e1f80(i8, i8) -> i8),
        entry!(0x006e_1fb0, fn_006e1fb0(i8, i8) -> i8),
        entry!(0x006e_1fe0, fn_006e1fe0(Ptr, u32)),
        entry!(0x006e_2000, fn_006e2000(Ptr<PathingLOSMap>)),
        entry!(0x006e_2050, fn_006e2050(Ptr<PathingLOSMap>, u32) -> u32),
        entry!(0x006e_21e0, ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0085_2950, fn_00852950(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_reloads_bucket_count_after_allocator_and_wraps_sizes() {
        for (address, final_vtable) in [(0x006e20e0, 0x0106cf88), (0x006e2080, 0x0106cf68)] {
            let mut e = Engine::new();
            let map = e.mem.alloc(20);
            let table = e.mem.alloc(8);
            e.mem.write(map, &[0xaa; 20]);
            e.mem.write(table, &[0xbb; 8]);
            e.register_double(0x00aa1070, move |e, a| {
                assert_eq!(a, &[4]); // 0x40000001 * 4 wraps.
                assert_eq!(e.mem.u32(map), 0x0106cf88);
                assert_eq!(e.mem.u32(map + 12), 0);
                e.mem.set_u32(map + 4, 2);
                table.into_ret()
            });
            assert_eq!(e.call(address, &args![map, 0x40000001u32]).u32(), map);
            assert_eq!(e.mem.u32(map), final_vtable);
            assert_eq!(e.mem.u32(map + 4), 2);
            assert_eq!(e.mem.u32(map + 8), table);
            assert_eq!(e.mem.u32(map + 16), 0xaaaaaaaa);
            assert_eq!(e.mem.bytes(table, 8), [0; 8]);
        }
    }
}

#[cfg(test)]
mod b040_tests {
    use super::*;

    #[test]
    fn destructors_clear_free_and_delete() {
        let mut e = Engine::new();
        e.register_double(REMOVE_ALL, |e, a| {
            // The vtable in place while the items go.
            let vtable = e.mem.u32(a[0]);
            assert!(vtable == VTABLE_NI_T_MAP || vtable == VTABLE_NI_T_MAP_BASE);
            Ret::default()
        });
        e.register(FREE_TABLE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x4240);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_20b0, &args![map, 1u32]).u32(), map);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (REMOVE_ALL, vec![map]),
                (REMOVE_ALL, vec![map]),
                (FREE_TABLE, vec![0x4240]),
                (OPERATOR_DELETE, vec![map]),
            ]
        );
        assert_eq!(e.mem.u32(map), VTABLE_NI_T_MAP_BASE);
        e.call_log = Some(vec![]);
        e.call(0x006e_20b0, &args![map, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != OPERATOR_DELETE));
    }
}

#[cfg(test)]
mod n032_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn take(e: &mut Engine) -> Log {
        e.call_log.take().unwrap()
    }

    fn array_doubles(e: &mut Engine) {
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(WORD_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4).wrapping_add(4u32.wrapping_mul(a[1]))).into_ret()
        });
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ARRAY_IS_EMPTY, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
    }

    /// The score doubles: `iTotalTriangleCount` and the score bytes.
    fn score_doubles(e: &mut Engine) {
        e.register(TOTAL_TRIANGLE_COUNT, |e, a| {
            e.mem.u32(a[0] + 0x2c).into_ret()
        });
        e.register(TRIANGLE_SCORE, |e, a| {
            u32::from(e.mem.u8(e.mem.u32(a[0] + 0x18) + a[1])).into_ret()
        });
        e.register(SET_TRIANGLE_SCORE, |e, a| {
            let scores = e.mem.u32(a[0] + 0x18);
            e.mem.set_u8(scores + a[1], a[2] as u8);
            Ret::default()
        });
    }

    fn scores(e: &mut Engine, map: u32, bytes: &[u8]) {
        let scores = e.mem.alloc(bytes.len() as u32);
        e.mem.write(scores, bytes);
        e.mem.set_u32(map + 0x18, scores);
        e.mem.set_u32(map + 0x2c, bytes.len() as u32);
    }

    #[test]
    fn deleting_destructor_and_destructor_release_every_member() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            stub(
                &mut e,
                &[
                    OPERATOR_DELETE,
                    NAV_MESH_POINTERS_DESTRUCT,
                    REMOVE_ALL,
                    FREE_TABLE,
                    REF_OBJECT_DESTRUCT,
                    FREE_SIZED,
                ],
            );
            let map = e.mem.alloc(0x50);
            e.mem.set_u32(map + 0x18, 0x4400);
            e.mem.set_u32(map + 0x10, 0x4500);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006e_1440, &args![map, flags]).u32(), map);
            let log = take(&mut e);
            let order: Vec<u32> = log[1..].iter().map(|c| c.0).collect();
            let mut expected = vec![
                OPERATOR_DELETE,
                NAV_MESH_POINTERS_DESTRUCT,
                REMOVE_ALL,
                REMOVE_ALL,
                FREE_TABLE,
                REF_OBJECT_DESTRUCT,
            ];
            if flags == 1 {
                expected.push(FREE_SIZED);
            }
            assert_eq!(order, expected);
            assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![0x4400]]);
            assert_eq!(calls(&log, FREE_TABLE), vec![vec![0x4500]]);
            assert_eq!(e.mem.u32(map), VTABLE_LOS_MAP);
        }
        // No scores: nothing deleted.
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                OPERATOR_DELETE,
                NAV_MESH_POINTERS_DESTRUCT,
                REMOVE_ALL,
                FREE_TABLE,
                REF_OBJECT_DESTRUCT,
            ],
        );
        let map = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        e.call(0x006e_1470, &args![map]);
        assert!(calls(&take(&mut e), OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn set_up_maps_offsets_and_fills_the_scores() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(
            &mut e,
            &[MAP_SET_AT, NAV_MESH_POINTERS_ADD, OPERATOR_DELETE],
        );
        e.register(NAV_MESH_KEY, |_, a| (a[0] + 1).into_ret());
        e.register(NAV_MESH_TRIANGLE_COUNT, |_, a| (a[0] >> 8).into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        // Navmeshes with 2 and 3 triangles (their "addresses" carry the count).
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, 0x200);
        e.mem.set_u32(pointers + 4, 0x300);
        let meshes = e.mem.alloc(0x10);
        e.mem.set_u32(meshes + 4, pointers);
        e.mem.set_u32(meshes + 8, 2);
        let map = e.mem.alloc(0x50);
        e.mem.set_u32(map + 0x18, 0x4400);
        e.mem.set_u32(map + 0x2c, 77);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006e_1500, &args![map, meshes]).bool());
        let log = take(&mut e);
        assert_eq!(
            calls(&log, MAP_SET_AT),
            vec![vec![map + 8, 0x201, 0], vec![map + 8, 0x301, 2]]
        );
        assert_eq!(
            calls(&log, NAV_MESH_POINTERS_ADD),
            vec![vec![map + 0x1c, pointers], vec![map + 0x1c, pointers + 4]]
        );
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![0x4400]]);
        assert_eq!(calls(&log, ALLOCATE), vec![vec![5]]);
        assert_eq!(e.mem.u32(map + 0x2c), 5);
        let scores = e.mem.u32(map + 0x18);
        assert_eq!(e.mem.bytes(scores, 5), [0x7f; 5]);
    }

    #[test]
    fn first_score_address_needs_the_key() {
        let mut e = Engine::new();
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 3);
            (a[1] == 9).into_ret()
        });
        let map = e.mem.alloc(0x50);
        e.mem.set_u32(map + 0x18, 0x4400);
        assert_eq!(e.call(0x006e_1610, &args![map, 9u32]).u32(), 0x4403);
        assert_eq!(e.call(0x006e_1610, &args![map, 8u32]).u32(), 0);
    }

    #[test]
    fn offset_lookup_passes_the_out_word() {
        let mut e = Engine::new();
        e.register(MAP_GET_AT, |_, a| (a[1] + 0x100).into_ret());
        let map = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006e_1650, &args![map, 5u32, 0x880u32]).u32(),
            0x105
        );
        assert_eq!(
            calls(&take(&mut e), MAP_GET_AT),
            vec![vec![map + 8, 5, 0x880]]
        );
    }

    /// A map whose navmeshes `0x200`, `0x300` start at offsets 0 and 4.
    fn located_map(e: &mut Engine, missing_key: u32) -> u32 {
        array_doubles(e);
        stub(e, &[NI_POINTER_ASSIGN]);
        e.register(NAV_MESH_KEY, |_, a| (a[0] + 1).into_ret());
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], if a[1] == 0x201 { 0 } else { 4 });
            (a[1] != missing_key).into_ret()
        });
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, 0x200);
        e.mem.set_u32(pointers + 4, 0x300);
        let map = e.mem.alloc(0x50);
        e.mem.set_u32(map + 0x1c + 4, pointers);
        e.mem.set_u32(map + 0x1c + 8, 2);
        map
    }

    #[test]
    fn locate_finds_the_nav_mesh_and_triangle_of_an_index() {
        let mut e = Engine::new();
        let map = located_map(&mut e, 0);
        let pointers = e.mem.u32(map + 0x20);
        let triangle = e.mem.alloc(4);
        // Index 2: inside the first navmesh (the second starts at 4).
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x006e_1670, &args![map, 2u32, 0x880u32, triangle])
            .bool());
        assert_eq!(e.mem.u16(triangle), 2);
        assert_eq!(
            calls(&take(&mut e), NI_POINTER_ASSIGN),
            vec![vec![0x880, pointers]]
        );
        // Index 6: past the last navmesh, the last one at triangle 2.
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x006e_1670, &args![map, 6u32, 0x880u32, triangle])
            .bool());
        assert_eq!(e.mem.u16(triangle), 2);
        assert_eq!(
            calls(&take(&mut e), NI_POINTER_ASSIGN),
            vec![vec![0x880, pointers + 4]]
        );
        // A missing key, then no navmeshes: false.
        let mut e = Engine::new();
        let map = located_map(&mut e, 0x301);
        assert!(!e
            .call(0x006e_1670, &args![map, 6u32, 0x880u32, triangle])
            .bool());
        e.mem.set_u32(map + 0x24, 0);
        assert!(!e
            .call(0x006e_1670, &args![map, 0u32, 0x880u32, triangle])
            .bool());
    }

    #[test]
    fn first_unscored_triangle_skips_positive_scores() {
        let mut e = Engine::new();
        let map = located_map(&mut e, 0);
        score_doubles(&mut e);
        scores(&mut e, map, &[5, 1, 0x80, 0]);
        let triangle = e.mem.alloc(4);
        assert!(e.call(0x006e_1770, &args![map, 0x880u32, triangle]).bool());
        assert_eq!(e.mem.u16(triangle), 2);
        // All positive: false.
        scores(&mut e, map, &[5, 1]);
        assert!(!e.call(0x006e_1770, &args![map, 0x880u32, triangle]).bool());
        // No navmeshes: false before any score.
        e.mem.set_u32(map + 0x24, 0);
        scores(&mut e, map, &[0]);
        assert!(!e.call(0x006e_1770, &args![map, 0x880u32, triangle]).bool());
    }

    #[test]
    fn signed_byte_min_and_max() {
        let mut e = Engine::new();
        for (a, b, min, max) in [
            (1i8, 2i8, 1i8, 2i8),
            (-1, 1, -1, 1),
            (-128, 127, -128, 127),
            (3, 3, 3, 3),
        ] {
            let low = e
                .call(0x006e_1f80, &args![a as u8 as u32, b as u8 as u32])
                .u8() as i8;
            let high = e
                .call(0x006e_1fb0, &args![a as u8 as u32, b as u8 as u32])
                .u8() as i8;
            assert_eq!((low, high), (min, max));
        }
    }

    #[test]
    fn children_sizing_forwards_to_the_array() {
        let mut e = Engine::new();
        stub(&mut e, &[CHILDREN_SET_SIZE]);
        e.call_log = Some(vec![]);
        e.call(0x006e_1fe0, &args![0x1000u32, 12u32]);
        assert_eq!(
            calls(&take(&mut e), CHILDREN_SET_SIZE),
            vec![vec![0x109c, 12]]
        );
    }

    #[test]
    fn clearing_scores_marks_the_non_positive_ones() {
        let mut e = Engine::new();
        score_doubles(&mut e);
        let map = e.mem.alloc(0x50);
        scores(&mut e, map, &[5, 0, 0x81, 0x7f, 1]);
        e.call(0x006e_2000, &args![map]);
        let bytes = e.mem.bytes(e.mem.u32(map + 0x18), 5);
        assert_eq!(bytes, [5, 0x7f, 0x7f, 0x7f, 1]);
    }

    #[test]
    fn has_key_returns_the_lookup() {
        let mut e = Engine::new();
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 9);
            (a[1] == 4).into_ret()
        });
        let map = e.mem.alloc(0x50);
        assert_eq!(e.call(0x006e_2050, &args![map, 4u32]).u32(), 1);
        assert_eq!(e.call(0x006e_2050, &args![map, 5u32]).u32(), 0);
    }

    #[test]
    fn base_map_deleting_destructor() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            stub(&mut e, &[REMOVE_ALL, FREE_TABLE, OPERATOR_DELETE]);
            let map = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006e_21e0, &args![map, flags]).u32(), map);
            let log = take(&mut e);
            assert_eq!(e.mem.u32(map), VTABLE_NI_T_MAP_BASE);
            assert_eq!(calls(&log, REMOVE_ALL).len(), 1);
            assert_eq!(calls(&log, OPERATOR_DELETE).len(), flags as usize);
        }
    }

    #[test]
    fn add_to_key_array_makes_the_array_when_missing() {
        let mut e = Engine::new();
        stub(&mut e, &[MAP_SET_AT]);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(UINT_ARRAY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(UINT_ARRAY_ADD, |e, a| e.mem.u32(a[1]).into_ret());
        e.register(MAP_GET_AT, |e, a| {
            if a[1] == 1 {
                e.mem.set_u32(a[2], 0x7700);
            }
            (a[1] == 1).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0085_2950, &args![0x900u32, 1u32, 0x55u32]);
        let log = take(&mut e);
        assert!(calls(&log, MAP_SET_AT).is_empty());
        assert_eq!(calls(&log, UINT_ARRAY_ADD)[0][0], 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x0085_2950, &args![0x900u32, 2u32, 0x66u32]);
        let log = take(&mut e);
        let set = calls(&log, MAP_SET_AT);
        assert_eq!((set[0][0], set[0][1]), (0x900, 2));
        assert_eq!(calls(&log, UINT_ARRAY_ADD)[0][0], set[0][2]);
        assert_eq!(calls(&log, ALLOCATE), vec![vec![0x10]]);
    }

    #[test]
    fn add_to_key_array_passes_the_value_by_address() {
        let mut e = Engine::new();
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 0x7700);
            true.into_ret()
        });
        e.register_double(UINT_ARRAY_ADD, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x55);
            Ret::default()
        });
        e.call(0x0085_2950, &args![0x900u32, 1u32, 0x55u32]);
    }

    /// Every callee of the debug geometry, recorded; the node's vtable
    /// slot +0xDC at `0x5100`.
    fn geometry_doubles(e: &mut Engine) {
        stub(
            e,
            &[
                FIRST_PROPERTY_SET,
                MATERIAL_SET_COLOR,
                MATERIAL_SET_ALPHA,
                ATTACH_PROPERTY,
                CHILDREN_SET_SIZE,
                NI_POINTER_CONSTRUCT,
                NI_POINTER_DESTRUCT,
                NI_POINTER_RELEASE,
                NI_POINTER_ASSIGN,
                COLOR_A_CONSTRUCT,
                VECTOR_CONSTRUCTOR_ITERATOR,
                SET_TRANSLATE_XYZ,
                SET_TRANSLATE,
                ADD_NO_LIGHTING_PROPERTY_RECURSE,
                PREPARE_OBJECT,
                UPDATE_PROPERTIES,
                UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE,
                0x5100,
            ],
        );
        e.put_vtable(0x5000, &[0; 0x38]);
        e.mem.set_u32(0x5000 + ATTACH_CHILD_SLOT, 0x5100);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x5000);
            a[0].into_ret()
        });
        e.register(FIRST_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MATERIAL_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NI_POINTER_FROM, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(COLOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MAKE_TRIANGLE, |_, _| 0x6600u32.into_ret());
        e.register(MAKE_CIRCLE, |_, _| 0x6700u32.into_ret());
        e.register(MAKE_MARKER, |_, _| 0x6800u32.into_ret());
        for (address, value) in [
            (ALPHA, 0.75f32.to_bits()),
            (CIRCLE_ALPHA, 0.5f32.to_bits()),
            (MARKER_SIZE, 16.0f32.to_bits()),
            (MARKER_HEIGHT, 400.0f32.to_bits()),
        ] {
            e.map(address, 4);
            e.mem.set_u32(address, value);
        }
        for (address, value) in [
            (SCORE_SCALE, 0.5f64),
            (VERTEX_RAISE, 5.0),
            (NO_SIGHT_RADIUS, 0.0),
            (CIRCLE_STEP, 25.0),
        ] {
            e.map(address, 8);
            e.mem.set_f64(address, value);
        }
    }

    #[test]
    fn debug_geometry_colours_scored_triangles() {
        let mut e = Engine::new();
        geometry_doubles(&mut e);
        score_doubles(&mut e);
        // Every triangle is in navmesh 0x7000 at its own index.
        let mesh = e.mem.alloc(4);
        e.register_double(ARRAY_IS_EMPTY, |_, _| false.into_ret());
        e.register(ARRAY_SIZE, |_, _| 1u32.into_ret());
        e.register_double(WORD_ELEMENT, move |_, _| mesh.into_ret());
        e.register(NAV_MESH_KEY, |_, _| 1u32.into_ret());
        e.register(MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 0);
            true.into_ret()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], 0x7000);
            Ret::default()
        });
        e.register(NAV_MESH_TRIANGLE, |_, a| (0x100 + a[1]).into_ret());
        // Triangle 0x102 has flag 0x20.
        e.register(TRIANGLE_HAS_FLAG, |_, a| (a[0] == 0x102).into_ret());
        e.register(TRIANGLE_VERTEX, |_, a| (0x1_0000 + a[1]).into_ret());
        let vertices = e.mem.alloc(0x24);
        for k in 0..3u32 {
            e.mem.set_f32(vertices + 0xc * k, k as f32);
            e.mem.set_f32(vertices + 0xc * k + 4, 10.0);
            e.mem.set_f32(vertices + 0xc * k + 8, 100.0);
        }
        e.register_double(NAV_MESH_VERTEX, move |_, a| {
            assert_eq!(a[0], 0x7000);
            (vertices + 0xc * a[1]).into_ret()
        });
        let map = e.mem.alloc(0x50);
        // Scores: -4 (lowest), 0x7f (none), 2 (flagged), 8 (highest), 0.
        scores(&mut e, map, &[0xfc, 0x7f, 2, 8, 0]);
        e.call_log = Some(vec![]);
        let node = e.call(0x006e_1800, &args![map]).u32();
        let log = take(&mut e);
        assert_eq!(e.mem.u32(node), 0x5000);
        assert_eq!(calls(&log, CHILDREN_SET_SIZE), vec![vec![node + 0x9c, 15]]);
        assert_eq!(calls(&log, MATERIAL_SET_ALPHA)[0][1], 0.75f32.to_bits());
        let colors = calls(&log, COLOR_A_CONSTRUCT);
        let alpha = 0.75f32.to_bits();
        // -4 / -4 * 0.5 + 0.5 = 1 red; 2 / 8 -> 0.625 green; 8 / 8 -> 1; 0 -> none.
        assert_eq!(colors.len(), 4);
        assert_eq!(colors[0][1..], [1.0f32.to_bits(), 0, 0, alpha]);
        assert_eq!(colors[1][1..], [0, 0.625f32.to_bits(), 0, alpha]);
        assert_eq!(colors[2][1..], [0, 1.0f32.to_bits(), 0, alpha]);
        assert_eq!(colors[3][1..], [0, 0, 0, alpha]);
        // Three triangles drawn (the flagged one is skipped), raised by 5.
        let made = calls(&log, MAKE_TRIANGLE);
        assert_eq!(made.len(), 3);
        assert_eq!(made[0].len(), 11);
        assert_eq!(made[0][2], 105.0f32.to_bits());
        assert_eq!(made[0][3], 1.0f32.to_bits());
        assert_eq!(made[0][10], 1);
        let attached = calls(&log, 0x5100);
        assert_eq!(attached, vec![vec![node, 0x6600, 1]; 3]);
        // fRadius 0: no circles.
        assert!(calls(&log, MAKE_CIRCLE).is_empty());
        assert_eq!(calls(&log, NODE_UPDATE).len(), 1);
        assert_eq!(calls(&log, NI_POINTER_RELEASE).len(), 2 + 3);
    }

    #[test]
    fn debug_geometry_adds_the_circles_with_a_radius() {
        let mut e = Engine::new();
        geometry_doubles(&mut e);
        score_doubles(&mut e);
        let map = e.mem.alloc(0x50);
        scores(&mut e, map, &[]);
        e.mem.set_f32(map + 0x3c, 1.0);
        e.mem.set_f32(map + 0x40, 2.0);
        e.mem.set_f32(map + 0x44, 3.0);
        e.mem.set_f32(map + 0x48, 600.0);
        e.call_log = Some(vec![]);
        let node = e.call(0x006e_1800, &args![map]).u32();
        let log = take(&mut e);
        let circles = calls(&log, MAKE_CIRCLE);
        assert_eq!(circles.len(), 5);
        assert_eq!(circles[0][..2], [600.0f32.to_bits(), 0x24]);
        let moved = calls(&log, SET_TRANSLATE_XYZ);
        let heights: Vec<u32> = moved.iter().map(|m| m[3]).collect();
        let expected: Vec<u32> = [3.0f32, 28.0, 53.0, 78.0, 103.0]
            .iter()
            .map(|h| h.to_bits())
            .collect();
        assert_eq!(heights, expected);
        assert_eq!(moved[0][1..3], [1.0f32.to_bits(), 2.0f32.to_bits()]);
        let marker = calls(&log, MAKE_MARKER);
        assert_eq!(
            marker[0][..5],
            [
                16.0f32.to_bits(),
                16.0f32.to_bits(),
                400.0f32.to_bits(),
                10,
                10
            ]
        );
        assert_eq!(calls(&log, SET_TRANSLATE), vec![vec![0x6800, map + 0x3c]]);
        let attached = calls(&log, 0x5100);
        assert_eq!(attached.len(), 7);
        assert_eq!(attached[6][0], node);
        let circles_node = attached[6][1];
        assert_eq!(attached[0], vec![circles_node, 0x6700, 1]);
        assert_eq!(calls(&log, PREPARE_OBJECT), vec![vec![circles_node, 0, 0]]);
        // A NaN radius also draws them.
        e.mem.set_u32(map + 0x48, 0x7fc0_0000);
        e.call_log = Some(vec![]);
        e.call(0x006e_1800, &args![map]);
        assert_eq!(calls(&take(&mut e), MAKE_CIRCLE).len(), 5);
    }
}
