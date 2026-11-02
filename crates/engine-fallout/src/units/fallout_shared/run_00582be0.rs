//! `fallout shared/run_00582be0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00582c20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00582c20 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form
/// type 0x41 (0xec bytes); `ECX` is the object. The engine map gives it no
/// name (a `range-sub` function), so the class is unconfirmed. It runs the
/// `TESForm` constructor (`00483370`) and the component constructors by
/// address, stores the three vtable pointers (`+0`, `+0x18`, `+0x24`),
/// registers the form type (`004f15a0`), and sets `+0x3c`, `+0x34` and
/// `+0x68` to null. It allocates a 0x10-byte object (`operator new`) and,
/// when the allocation succeeds, constructs it with `00589000(object,
/// 0x25)` and keeps the result at `+0x30` (null otherwise). It copies the
/// floats at `01016970` into `+0xa0` and `+0xa4` and the one at `01015f5c`
/// into `+0xa8` and `+0xac`, then calls `00583070` on the object.
pub fn fn_00582c20(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_e270, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0103_195c);
    e.mem.set_u32(base + 0x18, 0x0103_1948);
    e.mem.set_u32(base + 0x24, 0x0103_1928);
    e.call(0x0057_cd10, &args![base + 0x38]);
    e.call(0x0058_8fa0, &args![base + 0x50, 0x25u32]);
    e.call(0x0096_a2d0, &args![base + 0x60]);
    e.call(0x0063_3c90, &args![base + 0x6c, 0u32]);
    e.call(0x0068_15c0, &args![base + 0xa0]);
    e.call(0x0068_15c0, &args![base + 0xa8]);
    e.call(0x0058_8fd0, &args![base + 0xb0, 0x25u32]);
    e.call(0x0040_37b0, &args![base + 0xc0]);
    e.call(0x0048_e270, &args![base + 0xd4]);
    e.call(0x0048_e270, &args![base + 0xe0]);
    e.call(0x004f_15a0, &args![this, 0x41u32]);
    e.mem.set_u32(base + 0x3c, 0);
    let allocated = e.call(0x0040_1000, &args![0x10u32]).u32();
    let object = if allocated != 0 {
        e.call(0x0058_9000, &args![allocated, 0x25u32]).u32()
    } else {
        0
    };
    e.mem.set_u32(base + 0x30, object);
    e.mem.set_u32(base + 0x68, 0);
    e.mem.set_u32(base + 0x34, 0);
    let first = e.global::<f32>(0x0101_6970);
    e.mem.set_f32(base + 0xa4, first);
    e.mem.set_f32(base + 0xa0, first);
    let second = e.global::<f32>(0x0101_5f5c);
    e.mem.set_f32(base + 0xac, second);
    e.mem.set_f32(base + 0xa8, second);
    e.call(0x0058_3070, &args![this]);
    this
}

// Translated from 00582e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00582e30` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00582e00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_2e30, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00583040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `006fc980` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00583040(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x006f_c980, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESWorldSpace` fields (Xbox PDB names, PC offsets as `tesworldspace.rs`
/// lists them): `pCellMap`, `pPersistentCell`, `pTerrainManager`,
/// `pClimate`, `pImageSpace`, `pImpactSwap`, `cFlags`, `sParentUseFlags`,
/// `pParentWorld`, `pWorldWater`, `pLODWater`, `pMusicType`,
/// `pEncounterZone`.
const CELL_MAP: u32 = 0x30;
const PERSISTENT_CELL: u32 = 0x34;
const TERRAIN_MANAGER: u32 = 0x3c;
const CLIMATE: u32 = 0x40;
const IMAGE_SPACE: u32 = 0x44;
const IMPACT_SWAP: u32 = 0x48;
const FLAGS: u32 = 0x4c;
const PARENT_USE_FLAGS: u32 = 0x4e;
const PARENT_WORLD: u32 = 0x70;
const WORLD_WATER: u32 = 0x74;
const LOD_WATER: u32 = 0x78;
const MUSIC_TYPE: u32 = 0x9c;
const ENCOUNTER_ZONE: u32 = 0xd0;

/// `TESWorldSpace`'s vtable (RTTI `.?AVTESWorldSpace@@`) and those of its
/// bases at +0x18 and +0x24.
const WORLD_SPACE_VTABLE: u32 = 0x0103_195c;
const WORLD_SPACE_FULL_NAME_VTABLE: u32 = 0x0103_1948;
const WORLD_SPACE_TEXTURE_VTABLE: u32 = 0x0103_1928;
/// A static `TESWorldSpace *` the destructor clears when it is this one
/// (`011c3f44`).
const STATIC_WORLD_SPACE: u32 = 0x011c_3f44;
/// The float constant `00583070` copies into `fDefaultLandHeight`
/// (`01031920`, -2048.0 in the exe).
const DEFAULT_LAND_HEIGHT: u32 = 0x0103_1920;
/// `00559450` (`thiscall`): the word at the object (an `NiPointer`'s
/// pointer, a `BSStringT`'s text).
const FIRST_WORD: u32 = 0x0055_9450;
/// `0066b0d0` (`thiscall`, `RET 4`): assigns an `NiPointer` from a raw
/// pointer.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `00aa13e0` (`cdecl`): allocates from the Ni heap; `00c5a9d0`: the
/// constructor of the 0x78-byte object kept at +0x6c.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NODE_CONSTRUCT: u32 = 0x00c5_a9d0;
/// `00726070`: the word at +4 (the object's reference count).
const WORD_AT_4: u32 = 0x0072_6070;
/// `00450b80` (`cdecl`, one word) answers an object, `00454b30` on it
/// answers what `00583270` compares with the object at +0x6c, and
/// `00b5ddf0` (`RET 4`) on it is given 0.
const SCENE_GRAPH: u32 = 0x0045_0b80;
const SCENE_GRAPH_NODE: u32 = 0x0045_4b30;
const SCENE_SET_NODE: u32 = 0x00b5_ddf0;
/// `0040f2a0` (`thiscall`, `RET 4`): the deleting destructor of the object
/// at +0x48 (`pImpactSwap`).
const IMPACT_SWAP_DELETE: u32 = 0x0040_f2a0;
/// `004037f0` (`thiscall`, `RET 8`): sets a `BSStringT` (text, 0).
const STRING_SET: u32 = 0x0040_37f0;
/// The empty string `005832f0` answers without an editor id (`01011584`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `00484e40` (`thiscall`): a `bool`; when false `005861d0` (`RET 4`) runs
/// with 1.
const FORM_TEST: u32 = 0x0048_4e40;
const WORLD_SET_FLAG: u32 = 0x0058_61d0;
/// `00586480` (`thiscall`): the world space's next initialization step.
const WORLD_INIT_STEP: u32 = 0x0058_6480;
/// The destructor's callees: `00587f40`, `00587bb0`, `006fca70` (the
/// terrain manager's release), `005877e0`, `00588c80`, the base and member
/// destructors (`0048e2e0` textures, `004037d0` the string, `00589230` and
/// `00589130` the maps, `0045cec0` the `NiPointer`, `0046ffb0` the list,
/// `0057ce90` +0x38, `00591300` the full name, `00483630` `TESForm`).
const WORLD_CLEAR_CELLS: u32 = 0x0058_7f40;
const WORLD_CLEAR_REFERENCES: u32 = 0x0058_7bb0;
const TERRAIN_RELEASE: u32 = 0x006f_ca70;
const WORLD_CLEAR_OFFSETS: u32 = 0x0058_77e0;
const WORLD_CLEAR_MAPS: u32 = 0x0058_8c80;
const TEXTURE_DESTRUCT: u32 = 0x0048_e2e0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const OFFSET_MAP_DESTRUCT: u32 = 0x0058_9230;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
const REFERENCE_MAP_DESTRUCT: u32 = 0x0058_9130;
const MEMBER_38_DESTRUCT: u32 = 0x0057_ce90;
const FULL_NAME_DESTRUCT: u32 = 0x0059_1300;
const FORM_DESTRUCT: u32 = 0x0048_3630;

// Translated from 00582be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two pointers): a sort comparison on
/// the word at +4 of each, -1 when the first's is above the second's, 1
/// when below, 0 when equal (unsigned).
pub fn fn_00582be0(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let a = e.mem.u32(first.addr().wrapping_add(4));
    let b = e.mem.u32(second.addr().wrapping_add(4));
    if a > b {
        -1
    } else {
        i32::from(a < b)
    }
}

// Translated from 00583180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the `NiPointer` at +0x6c (`0066b0d0`
/// with null) and deletes `pImpactSwap` (`0040f2a0` with 1) when set, then
/// clears it.
pub fn fn_00583180(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(NI_POINTER_ASSIGN, &args![base.wrapping_add(0x6c), 0u32]);
    let swap = e.mem.u32(base.wrapping_add(IMPACT_SWAP));
    if swap != 0 {
        e.call(IMPACT_SWAP_DELETE, &args![swap, 1u32]);
    }
    e.mem.set_u32(base.wrapping_add(IMPACT_SWAP), 0);
}

// Translated from 005831d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the object behind the `NiPointer` at +0x6c,
/// made first when there is none (0x78 bytes from `00aa13e0`, constructed by
/// `00c5a9d0` unless the allocation failed, assigned with `0066b0d0`).
// Translation incomplete: 005831d0 - the SEH frame (FS:[0], unwind of the allocation) is not translated.
pub fn fn_005831d0(e: &mut Engine, this: Ptr) -> u32 {
    let holder = this.addr().wrapping_add(0x6c);
    if e.call(FIRST_WORD, &args![holder]).u32() == 0 {
        let block = e.call(NI_ALLOCATE, &args![0x78u32]).u32();
        let object = if block != 0 {
            e.call(NODE_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.call(NI_POINTER_ASSIGN, &args![holder, object]);
    }
    e.call(FIRST_WORD, &args![holder]).u32()
}

// Translated from 00583270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when the object at +0x6c exists and its
/// count (+4) is 1, it is taken out of the scene object `00450b80(0)`
/// answers (`00b5ddf0` with 0) if that object's `00454b30` is it, and the
/// `NiPointer` is cleared (`0066b0d0` with null).
pub fn fn_00583270(e: &mut Engine, this: Ptr) {
    let holder = this.addr().wrapping_add(0x6c);
    if e.call(FIRST_WORD, &args![holder]).u32() == 0 {
        return;
    }
    let object = e.call(FIRST_WORD, &args![holder]).u32();
    if e.call(WORD_AT_4, &args![object]).u32() != 1 {
        return;
    }
    let scene = e.call(SCENE_GRAPH, &args![0u32]).u32();
    let node = e.call(SCENE_GRAPH_NODE, &args![scene]).u32();
    if node == e.call(FIRST_WORD, &args![holder]).u32() {
        let scene = e.call(SCENE_GRAPH, &args![0u32]).u32();
        e.call(SCENE_SET_NODE, &args![scene, 0u32]);
    }
    e.call(NI_POINTER_ASSIGN, &args![holder, 0u32]);
}

// Translated from 005832f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the editor id's text (`cEditorID`, +0xc0,
/// through `00559450`), or the empty string `01011584` without one.
pub fn fn_005832f0(e: &mut Engine, this: Ptr) -> u32 {
    let name = this.addr().wrapping_add(0xc0);
    if e.call(FIRST_WORD, &args![name]).u32() != 0 {
        e.call(FIRST_WORD, &args![name]).u32()
    } else {
        EMPTY_STRING
    }
}

// Translated from 00583330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): sets the editor id (`cEditorID`,
/// +0xc0, `004037f0` with the text and 0); answers true.
pub fn fn_00583330(e: &mut Engine, this: Ptr, name: u32) -> bool {
    e.call(
        STRING_SET,
        &args![this.addr().wrapping_add(0xc0), name, 0u32],
    );
    true
}

// Translated from 00583070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets a `TESWorldSpace`'s fields to their
/// start values, in the exe's order: no parent world or encounter zone,
/// flags and parent-use flags 0; `005861d0(1)` unless `00484e40` answers
/// true; no climate, image space or waters, LOD water height 0.0, map scale
/// 1.0 (`FLD1`) and offsets 0.0; `00586480`; no music or impact swap; the
/// default land height from `01031920` and water height 0.0; the
/// `NiPointer` at +0x6c cleared; then slot 0 of each texture's vtable
/// (+0xd4, +0xe0).
pub fn fn_00583070(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(PARENT_WORLD), 0);
    e.mem.set_u32(base.wrapping_add(ENCOUNTER_ZONE), 0);
    e.mem.set_u8(base.wrapping_add(FLAGS), 0);
    e.mem.set_u16(base.wrapping_add(PARENT_USE_FLAGS), 0);
    if !e.call(FORM_TEST, &args![this]).bool() {
        e.call(WORLD_SET_FLAG, &args![this, 1u32]);
    }
    e.mem.set_u32(base.wrapping_add(CLIMATE), 0);
    e.mem.set_u32(base.wrapping_add(IMAGE_SPACE), 0);
    e.mem.set_u32(base.wrapping_add(WORLD_WATER), 0);
    e.mem.set_u32(base.wrapping_add(LOD_WATER), 0);
    let zero = x87_store32(0.0);
    e.mem.set_u32(base.wrapping_add(0x7c), zero);
    e.mem.set_u32(base.wrapping_add(0x90), x87_store32(1.0));
    e.mem.set_u32(base.wrapping_add(0x94), zero);
    e.mem.set_u32(base.wrapping_add(0x98), zero);
    e.call(WORLD_INIT_STEP, &args![this]);
    e.mem.set_u32(base.wrapping_add(MUSIC_TYPE), 0);
    e.mem.set_u32(base.wrapping_add(IMPACT_SWAP), 0);
    let height = x87_quiet_bits(e.global::<u32>(DEFAULT_LAND_HEIGHT));
    e.mem.set_u32(base.wrapping_add(0xc8), height);
    e.mem.set_u32(base.wrapping_add(0xcc), zero);
    e.call(NI_POINTER_ASSIGN, &args![base.wrapping_add(0x6c), 0u32]);
    e.vcall(base.wrapping_add(0xd4), 0, &args![]);
    e.vcall(base.wrapping_add(0xe0), 0, &args![]);
}

// Translated from 00582e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESWorldSpace` destructor body (unnamed in the engine map; its
/// vtables by RTTI): sets the three vtables, forgets this world in the
/// static at `011c3f44`, runs `00587f40` and `00587bb0`; the terrain
/// manager (+0x3c) is released (`006fca70`) and deleted (`00583040` with 1)
/// and cleared; `00583180`, `005877e0`; the cell map (+0x30, vtable slot 0)
/// and the persistent cell (+0x34, slot 0x10) are deleted with 1 and
/// cleared; `00588c80`; then the members and bases in reverse order.
// Translation incomplete: 00582e30 - the SEH frame (FS:[0], unwind of the members and bases) is not translated.
pub fn fn_00582e30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, WORLD_SPACE_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x18), WORLD_SPACE_FULL_NAME_VTABLE);
    e.mem
        .set_u32(base.wrapping_add(0x24), WORLD_SPACE_TEXTURE_VTABLE);
    if e.global::<u32>(STATIC_WORLD_SPACE) == base {
        e.set_global::<u32>(STATIC_WORLD_SPACE, 0);
    }
    e.call(WORLD_CLEAR_CELLS, &args![this]);
    e.call(WORLD_CLEAR_REFERENCES, &args![this]);
    let terrain = e.mem.u32(base.wrapping_add(TERRAIN_MANAGER));
    if terrain != 0 {
        e.call(TERRAIN_RELEASE, &args![terrain]);
        let terrain = e.mem.u32(base.wrapping_add(TERRAIN_MANAGER));
        if terrain != 0 {
            fn_00583040(e, Ptr::new(terrain), 1);
        }
        e.mem.set_u32(base.wrapping_add(TERRAIN_MANAGER), 0);
    }
    fn_00583180(e, this);
    e.call(WORLD_CLEAR_OFFSETS, &args![this]);
    let cells = e.mem.u32(base.wrapping_add(CELL_MAP));
    if cells != 0 {
        e.vcall(cells, 0, &args![1u32]);
    }
    e.mem.set_u32(base.wrapping_add(CELL_MAP), 0);
    let cell = e.mem.u32(base.wrapping_add(PERSISTENT_CELL));
    if cell != 0 {
        e.vcall(cell, 0x10, &args![1u32]);
    }
    e.mem.set_u32(base.wrapping_add(PERSISTENT_CELL), 0);
    e.call(WORLD_CLEAR_MAPS, &args![this]);
    for (offset, destructor) in [
        (0xe0u32, TEXTURE_DESTRUCT),
        (0xd4, TEXTURE_DESTRUCT),
        (0xc0, STRING_DESTRUCT),
        (0xb0, OFFSET_MAP_DESTRUCT),
        (0x6c, NI_POINTER_DESTRUCT),
        (0x60, LIST_DESTRUCT),
        (0x50, REFERENCE_MAP_DESTRUCT),
        (0x38, MEMBER_38_DESTRUCT),
        (0x24, TEXTURE_DESTRUCT),
        (0x18, FULL_NAME_DESTRUCT),
    ] {
        e.call(destructor, &args![base.wrapping_add(offset)]);
    }
    e.call(FORM_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_2c20, fn_00582c20(Ptr) -> Ptr),
        entry!(0x0058_2e00, fn_00582e00(Ptr, u32) -> Ptr),
        entry!(0x0058_3040, fn_00583040(Ptr, u32) -> Ptr),
        entry!(0x0058_2be0, fn_00582be0(Ptr, Ptr) -> i32),
        entry!(0x0058_3180, fn_00583180(Ptr)),
        entry!(0x0058_31d0, fn_005831d0(Ptr) -> u32),
        entry!(0x0058_3270, fn_00583270(Ptr)),
        entry!(0x0058_32f0, fn_005832f0(Ptr) -> u32),
        entry!(0x0058_3330, fn_00583330(Ptr, u32) -> bool),
        entry!(0x0058_3070, fn_00583070(Ptr)),
        entry!(0x0058_2e30, fn_00582e30(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00582c20_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_2c20,
            size: 0xec,
            globals: &[(0x0101_6970, 0x1000000), (0x0101_5f5c, 0x1000001)],
            returns: &[(0x0040_1000, 0x2000_0000), (0x0058_9000, 0x2000_1000)],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_e270, &[A::T(0x24)]),
                (0x0057_cd10, &[A::T(0x38)]),
                (0x0058_8fa0, &[A::T(0x50), A::V(0x25)]),
                (0x0096_a2d0, &[A::T(0x60)]),
                (0x0063_3c90, &[A::T(0x6c), A::V(0x0)]),
                (0x0068_15c0, &[A::T(0xa0)]),
                (0x0068_15c0, &[A::T(0xa8)]),
                (0x0058_8fd0, &[A::T(0xb0), A::V(0x25)]),
                (0x0040_37b0, &[A::T(0xc0)]),
                (0x0048_e270, &[A::T(0xd4)]),
                (0x0048_e270, &[A::T(0xe0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x41)]),
                (0x0040_1000, &[A::V(0x10)]),
                (0x0058_9000, &[A::V(0x2000_0000), A::V(0x25)]),
                (0x0058_3070, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_195c),
                (0x18, 4, 0x0103_1948),
                (0x24, 4, 0x0103_1928),
                (0x3c, 4, 0x0),
                (0x30, 4, 0x2000_1000),
                (0x68, 4, 0x0),
                (0x34, 4, 0x0),
                (0xa4, 4, 0x1000000),
                (0xa0, 4, 0x1000000),
                (0xac, 4, 0x1000001),
                (0xa8, 4, 0x1000001),
            ],
        });
    }

    #[test]
    fn fn_00582c20_without_memory_leaves_the_object_null() {
        // `operator new` answers null: the object is not constructed and
        // `+0x30` stays null.
        let mut e = Engine::new();
        let this = e.mem.alloc(0xec);
        e.mem.write(this, &[0xcd; 0xec]);
        e.mem.map(0x0101_6970, 4);
        e.mem.map(0x0101_5f5c, 4);
        for a in [
            0x0048_3370,
            0x0040_2d00,
            0x0048_e270,
            0x0057_cd10,
            0x0058_8fa0,
            0x0096_a2d0,
            0x0063_3c90,
            0x0068_15c0,
            0x0058_8fd0,
            0x0040_37b0,
            0x004f_15a0,
            0x0040_1000,
            0x0058_9000,
            0x0058_3070,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0058_2c20, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != 0x0058_9000));
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }
    #[test]
    fn at_00582e00_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_2e30, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_2e00, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_2e00)
                .collect();
            let mut expected = vec![(0x0058_2e30, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00583040_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x006f_c980, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_3040, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_3040)
                .collect();
            let mut expected = vec![(0x006f_c980, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    const VTABLE: u32 = 0x00ab_0000;
    const SLOT_0_FN: u32 = 0x00ab_1000;
    const SLOT_10_FN: u32 = 0x00ab_1004;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(WORD_AT_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let mut slots = vec![0u32; 8];
        slots[0] = SLOT_0_FN;
        slots[4] = SLOT_10_FN;
        e.put_vtable(VTABLE, &slots);
        for addr in [
            SLOT_0_FN,
            SLOT_10_FN,
            IMPACT_SWAP_DELETE,
            STRING_SET,
            WORLD_SET_FLAG,
            WORLD_INIT_STEP,
            SCENE_SET_NODE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn fn_00582be0_compares_the_words_at_4() {
        let mut e = Engine::new();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        for (x, y, expected) in [
            (5u32, 3u32, -1i32),
            (3, 5, 1),
            (4, 4, 0),
            (0xffff_ffff, 1, -1),
        ] {
            e.mem.set_u32(a + 4, x);
            e.mem.set_u32(b + 4, y);
            assert_eq!(e.call(0x0058_2be0, &args![a, b]).i32(), expected);
        }
    }

    #[test]
    fn pointer_at_6c_is_made_released_and_cleared() {
        let mut e = engine();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 1);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0xec);
        let made = e.call(0x0058_31d0, &args![this]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.mem.u32(this + 0x6c), made);
        // Already there: kept.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_31d0, &args![this]).u32(), made);
        assert!(calls_to(&e, NI_ALLOCATE).is_empty());
        // Count 1 and the scene's node: taken out, then cleared.
        e.register(SCENE_GRAPH, |_, _| 0x7000u32.into_ret());
        e.register(SCENE_GRAPH_NODE, |e, _| e.mem.u32(0x0055_0000).into_ret());
        e.map(0x0055_0000, 4);
        e.mem.set_u32(0x0055_0000, made);
        e.call_log = Some(vec![]);
        e.call(0x0058_3270, &args![this]);
        assert_eq!(calls_to(&e, SCENE_SET_NODE), vec![vec![0x7000, 0]]);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        // A count above 1 keeps it.
        e.mem.set_u32(this + 0x6c, made);
        e.mem.set_u32(made + 4, 2);
        e.call(0x0058_3270, &args![this]);
        assert_eq!(e.mem.u32(this + 0x6c), made);
        // Another scene node: cleared without taking it out.
        e.mem.set_u32(made + 4, 1);
        e.mem.set_u32(0x0055_0000, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0058_3270, &args![this]);
        assert!(calls_to(&e, SCENE_SET_NODE).is_empty());
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        // 00583180 clears it and deletes the impact swap.
        e.mem.set_u32(this + 0x6c, made);
        e.mem.set_u32(this + 0x48, 0x4848);
        e.call_log = Some(vec![]);
        e.call(0x0058_3180, &args![this]);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(calls_to(&e, IMPACT_SWAP_DELETE), vec![vec![0x4848, 1]]);
    }

    #[test]
    fn editor_id_get_and_set() {
        let mut e = engine();
        let this = e.mem.alloc(0xec);
        assert_eq!(e.call(0x0058_32f0, &args![this]).u32(), EMPTY_STRING);
        e.mem.set_u32(this + 0xc0, 0x6000);
        assert_eq!(e.call(0x0058_32f0, &args![this]).u32(), 0x6000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0058_3330, &args![this, 0x6100u32]).bool());
        assert_eq!(calls_to(&e, STRING_SET), vec![vec![this + 0xc0, 0x6100, 0]]);
    }

    #[test]
    fn fn_00583070_sets_the_start_values() {
        let mut e = engine();
        e.map(0x0103_1000, 0x1000);
        e.set_global::<f32>(DEFAULT_LAND_HEIGHT, -2048.0);
        e.register(FORM_TEST, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0xec);
        e.mem.write(this, &[0xff; 0xec]);
        e.mem.set_u32(this + 0xd4, VTABLE);
        e.mem.set_u32(this + 0xe0, VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x0058_3070, &args![this]);
        assert_eq!(calls_to(&e, WORLD_SET_FLAG), vec![vec![this, 1]]);
        assert_eq!(
            calls_to(&e, SLOT_0_FN),
            vec![vec![this + 0xd4], vec![this + 0xe0]]
        );
        for offset in [
            0x70u32, 0xd0, 0x40, 0x44, 0x74, 0x78, 0x7c, 0x94, 0x98, 0x9c, 0x48, 0xcc, 0x6c,
        ] {
            assert_eq!(e.mem.u32(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x4c), 0);
        assert_eq!(e.mem.u16(this + 0x4e), 0);
        assert_eq!(e.mem.f32(this + 0x90), 1.0);
        assert_eq!(e.mem.f32(this + 0xc8), -2048.0);
        assert_eq!(e.mem.u32(this + 0xa0), 0xffff_ffff);
        e.register(FORM_TEST, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0058_3070, &args![this]);
        assert!(calls_to(&e, WORLD_SET_FLAG).is_empty());
    }

    #[test]
    fn fn_00582e30_destroys_the_world_space() {
        let mut e = engine();
        e.map(0x011c_3000, 0x1000);
        for addr in [
            WORLD_CLEAR_CELLS,
            WORLD_CLEAR_REFERENCES,
            TERRAIN_RELEASE,
            WORLD_CLEAR_OFFSETS,
            WORLD_CLEAR_MAPS,
            TEXTURE_DESTRUCT,
            STRING_DESTRUCT,
            OFFSET_MAP_DESTRUCT,
            NI_POINTER_DESTRUCT,
            LIST_DESTRUCT,
            REFERENCE_MAP_DESTRUCT,
            MEMBER_38_DESTRUCT,
            FULL_NAME_DESTRUCT,
            FORM_DESTRUCT,
            0x006f_c980,
            0x0040_1030,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xec);
        let cells = e.mem.alloc(8);
        let cell = e.mem.alloc(8);
        e.mem.set_u32(cells, VTABLE);
        e.mem.set_u32(cell, VTABLE);
        e.mem.set_u32(this + 0x30, cells);
        e.mem.set_u32(this + 0x34, cell);
        e.mem.set_u32(this + 0x3c, 0x3c3c);
        e.set_global::<u32>(STATIC_WORLD_SPACE, this);
        e.call_log = Some(vec![]);
        e.call(0x0058_2e30, &args![this]);
        assert_eq!(e.global::<u32>(STATIC_WORLD_SPACE), 0);
        assert_eq!(
            [
                e.mem.u32(this),
                e.mem.u32(this + 0x18),
                e.mem.u32(this + 0x24)
            ],
            [
                WORLD_SPACE_VTABLE,
                WORLD_SPACE_FULL_NAME_VTABLE,
                WORLD_SPACE_TEXTURE_VTABLE
            ]
        );
        assert_eq!(calls_to(&e, TERRAIN_RELEASE), vec![vec![0x3c3c]]);
        assert_eq!(calls_to(&e, 0x006f_c980), vec![vec![0x3c3c]]);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![0x3c3c]]);
        assert_eq!(calls_to(&e, SLOT_0_FN), vec![vec![cells, 1]]);
        assert_eq!(calls_to(&e, SLOT_10_FN), vec![vec![cell, 1]]);
        for offset in [0x30u32, 0x34, 0x3c, 0x48, 0x6c] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(
            calls_to(&e, TEXTURE_DESTRUCT),
            vec![vec![this + 0xe0], vec![this + 0xd4], vec![this + 0x24]]
        );
        assert_eq!(calls_to(&e, FORM_DESTRUCT), vec![vec![this]]);
        // Another static world space stays.
        e.set_global::<u32>(STATIC_WORLD_SPACE, 0x1234);
        e.call(0x0058_2e30, &args![this]);
        assert_eq!(e.global::<u32>(STATIC_WORLD_SPACE), 0x1234);
    }
}
