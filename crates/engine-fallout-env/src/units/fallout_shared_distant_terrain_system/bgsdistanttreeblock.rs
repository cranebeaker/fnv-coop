//! `fallout shared/distant terrain system/bgsdistanttreeblock.cpp` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006f99e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map-item allocator: allocates twelve bytes without initializing them.
pub fn allocate_item(e: &mut Engine, _this: Ptr) -> Ptr {
    e.call(0x00401000, &args![12u32]).ptr()
}

/// The function `fn_006f9080` calls on the member (`0045cec0`).
const CALLEE_0045CEC0: u32 = 0x0045_cec0;

// Translated from 006f9080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0045cec0` with `ECX` = `this + 0x1c` and passes on nothing (the callee returns no value).
pub fn fn_006f9080(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0045CEC0, &args![this.byte_add(0x1c)]);
}

// Translated from 006f94a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0106ded0` (RTTI `.?AVBGSDistantTreeBlockLoadTask@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00c3cea0`. One word, plain `RET`, returns nothing.
pub fn fn_006f94a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_ded0);
    e.call(0x00c3_cea0, &args![this]);
}

/// The constructor body `006fa180` (`this` plus two stack words, `RET 8`).
const CONSTRUCTOR_BODY_006FA180: u32 = 0x006f_a180;
/// The vtable stored at `+0` (RTTI class `BSSimpleArray<BGSDistantTreeBlock::TreeType,256>`).
const VTABLE_006FA100: u32 = 0x0106_dfe0;

// Translated from 006fa100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bgsdistanttreeblock.cpp`): stores the vtable `0106dfe0`
/// (RTTI class `BSSimpleArray<BGSDistantTreeBlock::TreeType,...>`), then calls
/// `006fa180(this, 0, 0)`. Returns `this`.
pub fn fn_006fa100(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_006FA100);
    e.call(CONSTRUCTOR_BODY_006FA180, &args![this, 0u32, 0u32]);
    this
}

// The functions below: `BGSDistantTreeBlock` (Xbox PDB layout, size 0x4c: `TreeGroups` at
// `+4`, `InstanceMap` at `+0x14`, `TreesQueuedToHide` at `+0x34`), `BGSDistantTreeBlockLoadTask`
// (size 0x48: `pBlock` `+0x30`, `bFinishedQueuing` `+0x34`, `iX` `+0x38`, `iY` `+0x3c`, `iLevel`
// `+0x40`, `bCanceled` `+0x44`, `bFileExists` `+0x45`; the PC code uses the same offsets) and the
// `BSMap`/`BSSimpleArray` template code of this unit. `InstanceData` keeps `uiGroup` at `+0x18`
// and `bHidden` at `+0x1c`; `TreeGroup` keeps `spGeometry` at `+4`, `Instances` at `+8` and
// `bShaderPropertyUpToDate` at `+0x1c` (Xbox PDB).

/// `operator new` (`00401000`) and `operator delete` (`00401030`), cdecl, one word.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00559450` (thiscall, no stack words): the object a smart pointer holds.
const POINTER_GET_00559450: u32 = 0x0055_9450;
/// `0066b0d0` (thiscall, one word): assigns a smart pointer.
const POINTER_SET_0066B0D0: u32 = 0x0066_b0d0;
/// `0044ddc0` (thiscall, no stack words): an array's count.
const ARRAY_COUNT_0044DDC0: u32 = 0x0044_ddc0;
/// `00877a30` (thiscall, one word: the index): the address of an array entry.
const ARRAY_AT_00877A30: u32 = 0x0087_7a30;
/// `008454f0` (thiscall, one word): frees an array (called with 1).
const ARRAY_FREE_008454F0: u32 = 0x0084_54f0;
/// `00406d00` (cdecl, variable arguments: buffer, size, format, ...): formats into a buffer.
const FORMAT_00406D00: u32 = 0x0040_6d00;
/// The smart pointers and the flag the static data lives in (`.data`; the Xbox PDB lists the
/// statics `spSharedStencil`, `spSharedAlpha`, `spAtlasTexture`, `TreeTypes` and
/// `bStaticDataLoaded`, which address is which is not confirmed).
const STATIC_POINTER_011D85BC: u32 = 0x011d_85bc;
const STATIC_POINTER_011D85C4: u32 = 0x011d_85c4;
const STATIC_TEXTURE_011D85E0: u32 = 0x011d_85e0;
const STATIC_ARRAY_011D85D0: u32 = 0x011d_85d0;
const STATIC_LOADED_011D85A0: u32 = 0x011d_85a0;
/// The smart pointer `006f7a60` sets (`.data`).
const STATIC_POINTER_012007C4: u32 = 0x0120_07c4;
/// The security cookie (`.data`) and `__security_check_cookie` (`00ec408c`, fastcall).
const SECURITY_COOKIE_011C16BC: u32 = 0x011c_16bc;
const SECURITY_CHECK_COOKIE: u32 = 0x00ec_408c;

// Translated from 006f77a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f77a0 - the compiler's exception frame (FS:[0] chain) is not translated
/// `BGSDistantTreeBlock::AllocateStaticData` (the map's name, from its strings; cdecl, one
/// word: the worldspace). Formats `"Data\Meshes\Landscape\LOD\%s\Trees\TreeTypes.lst"`
/// (`0106de08`) with the worldspace's slot `0x130` into a 0x104-byte stack buffer, opens it
/// (`00afdf00(path, 0, 0x800)`, cdecl) and when that gives a file reads the tree types
/// (`006f8f80`, cdecl) and deletes the file (its slot 0 with 1). The texture is
/// `"TreeDeadLOD.dds"` (`0106ddf8`), or `"TreeSwampLOD.dds"` (`0106dddc`) when the name starts
/// with `"DLC4"` (`strncmp`, `00ec8a19`, 4 characters); the path `"\Data\Textures\Landscape\
/// Trees\%s"` (`0106ddb8`) goes in a second buffer, and the texture manager `011dea10` makes it
/// into `011d85e0` (`004568c0`, four words: path, `011d85e0`, 1, 1). Without a texture it
/// reports `"LOD tree texture atlas not found!"` (`005b5e40`, `0106dd94`). Then
/// [`fn_006f7a60`] with the texture, `011d85a0` set to 1, a 0x24-byte object (`00aa13e0`,
/// constructed by `0049ec80`, or 0) into `011d85bc` with `0049ee30(3)` on it, and a 0x1c-byte
/// object (constructed by `004391c0`, or 0) into `011d85c4` with `004393c0(1)`, `0049ed90(0)`,
/// `0094db80(0x80)` and `004393e0(4)` on it. Ends with the cookie check.
pub fn bgs_distant_tree_block_allocate_static_data(e: &mut Engine, world: u32) {
    let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC);
    e.with_stack(0x210, |e, buffers| {
        let list_path = buffers.addr();
        let texture_path = buffers.addr() + 0x108;
        let name = e.vcall(world, 0x130, &args![]).u32();
        e.call(
            FORMAT_00406D00,
            &args![list_path, 0x104u32, 0x0106_de08u32, name],
        );
        let file = e.call(0x00af_df00, &args![list_path, 0u32, 0x800u32]).u32();
        if file != 0 {
            e.call(0x006f_8f80, &args![file]);
            if file != 0 {
                e.vcall(file, 0, &args![1u32]);
            }
        }
        let mut texture = 0x0106_ddf8u32;
        let name = e.vcall(world, 0x130, &args![]).u32();
        if e.call(0x00ec_8a19, &args![name, 0x0106_ddf0u32, 4u32])
            .u32()
            == 0
        {
            texture = 0x0106_dddc;
        }
        e.call(
            FORMAT_00406D00,
            &args![texture_path, 0x104u32, 0x0106_ddb8u32, texture],
        );
        let manager = e.global::<u32>(0x011d_ea10);
        e.call(
            0x0045_68c0,
            &args![manager, texture_path, STATIC_TEXTURE_011D85E0, 1u32, 1u32],
        );
    });
    if e.call(POINTER_GET_00559450, &args![STATIC_TEXTURE_011D85E0])
        .u32()
        == 0
    {
        e.call(0x005b_5e40, &args![0x0106_dd94u32]);
    }
    let texture = e
        .call(POINTER_GET_00559450, &args![STATIC_TEXTURE_011D85E0])
        .u32();
    fn_006f7a60(e, texture);
    e.mem.set_u8(STATIC_LOADED_011D85A0, 1);
    let block = e.call(0x00aa_13e0, &args![0x24u32]).u32();
    let object = if block != 0 {
        e.call(0x0049_ec80, &args![block]).u32()
    } else {
        0
    };
    e.call(
        POINTER_SET_0066B0D0,
        &args![STATIC_POINTER_011D85BC, object],
    );
    let held = e
        .call(POINTER_GET_00559450, &args![STATIC_POINTER_011D85BC])
        .u32();
    e.call(0x0049_ee30, &args![held, 3u32]);
    let block = e.call(0x00aa_13e0, &args![0x1cu32]).u32();
    let object = if block != 0 {
        e.call(0x0043_91c0, &args![block]).u32()
    } else {
        0
    };
    e.call(
        POINTER_SET_0066B0D0,
        &args![STATIC_POINTER_011D85C4, object],
    );
    for (callee, value) in [
        (0x0043_93c0u32, 1u32),
        (0x0049_ed90, 0),
        (0x0094_db80, 0x80),
        (0x0043_93e0, 4),
    ] {
        let held = e
            .call(POINTER_GET_00559450, &args![STATIC_POINTER_011D85C4])
            .u32();
        e.call(callee, &args![held, value]);
    }
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
}

// Translated from 006f7a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): assigns the word to the smart pointer at `012007c4` (`0066b0d0`).
pub fn fn_006f7a60(e: &mut Engine, object: u32) {
    e.call(
        POINTER_SET_0066B0D0,
        &args![STATIC_POINTER_012007C4, object],
    );
}

// Translated from 006f83f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the static data's release. For each entry of the array at
/// `011d85d0` (count `0044ddc0`, entry `006b3120`) clears the smart pointer at the entry's
/// `+0x1c`; clears the smart pointers at `011d85bc`, `011d85c4` and `011d85e0`, runs
/// [`fn_006f7a60`] with 0 and clears the flag at `011d85a0`.
pub fn fn_006f83f0(e: &mut Engine) {
    let mut index = 0u32;
    while index
        < e.call(ARRAY_COUNT_0044DDC0, &args![STATIC_ARRAY_011D85D0])
            .u32()
    {
        let entry = e
            .call(0x006b_3120, &args![STATIC_ARRAY_011D85D0, index])
            .u32();
        e.call(POINTER_SET_0066B0D0, &args![entry.wrapping_add(0x1c), 0u32]);
        index = index.wrapping_add(1);
    }
    e.call(POINTER_SET_0066B0D0, &args![STATIC_POINTER_011D85BC, 0u32]);
    e.call(POINTER_SET_0066B0D0, &args![STATIC_POINTER_011D85C4, 0u32]);
    e.call(POINTER_SET_0066B0D0, &args![STATIC_TEXTURE_011D85E0, 0u32]);
    fn_006f7a60(e, 0);
    e.mem.set_u8(STATIC_LOADED_011D85A0, 0);
}

// Translated from 006f86e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall on a `BGSDistantTreeBlock`): for each tree group of `TreeGroups` (`+4`)
/// frees its `Instances` (`008454f0(group + 8, 1)`); when its `spGeometry` (`+4`) holds an
/// object, slot `0xe8` of `004ea9e0(00559450(008041a0(this)))` is called with it and the
/// pointer is cleared; a set group is deleted (`006f8800(group, 1)`). Then `TreeGroups` is freed
/// (`008454f0(this + 4, 1)`).
pub fn fn_006f86e0(e: &mut Engine, this: Ptr) {
    let groups = this.addr() + 4;
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT_0044DDC0, &args![groups]).u32() {
        let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
        let group = e.mem.u32(group_at);
        e.call(ARRAY_FREE_008454F0, &args![group.wrapping_add(8), 1u32]);
        let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
        let group = e.mem.u32(group_at);
        if e.call(POINTER_GET_00559450, &args![group.wrapping_add(4)])
            .u32()
            != 0
        {
            let first = e.call(0x0080_41a0, &args![this]).u32();
            let second = e.call(POINTER_GET_00559450, &args![first]).u32();
            let parent = e.call(0x004e_a9e0, &args![second]).u32();
            let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
            let group = e.mem.u32(group_at);
            let geometry = e
                .call(POINTER_GET_00559450, &args![group.wrapping_add(4)])
                .u32();
            e.vcall(parent, 0xe8, &args![geometry]);
            let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
            let group = e.mem.u32(group_at);
            e.call(POINTER_SET_0066B0D0, &args![group.wrapping_add(4), 0u32]);
        }
        let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
        let group = e.mem.u32(group_at);
        if group != 0 {
            e.call(0x006f_8800, &args![group, 1u32]);
        }
        index = index.wrapping_add(1);
    }
    e.call(ARRAY_FREE_008454F0, &args![groups, 1u32]);
}

// Translated from 006f8890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map names this body `BGSDistantTreeBlockLoadTask::BGSDistantTreeBlockLoadTask` (from its
/// string); it formats the block's file name (cdecl, five words: buffer, worldspace, level,
/// x, y): `"Data\Meshes\Landscape\LOD\%s\Trees\%s.Level%i.X%i.Y%i.DTL"` (`0106de90`, 0x104
/// bytes) with the worldspace's slot `0x130` twice, the level and the two 16-bit coordinates
/// sign-extended.
pub fn fn_006f8890(e: &mut Engine, buffer: u32, world: u32, level: u32, x: u32, y: u32) {
    let y = y as u16 as i16 as i32 as u32;
    let x = x as u16 as i16 as i32 as u32;
    let first = e.vcall(world, 0x130, &args![]).u32();
    let second = e.vcall(world, 0x130, &args![]).u32();
    e.call(
        FORMAT_00406D00,
        &args![buffer, 0x104u32, 0x0106_de90u32, second, first, level, x, y],
    );
}

/// The shape of [`fn_006f9230`] and [`fn_006f92a0`]: looks the id up in `InstanceMap` (`+0x14`;
/// `00853130`, thiscall, two words: the id and the address of a result local set to 0); when
/// found sets `bHidden` (`+0x1c`) of the instance to `hidden` and clears
/// `bShaderPropertyUpToDate` (`+0x1c`) of its group (entry `uiGroup`, `+0x18`, of `TreeGroups`).
/// Returns whether it was found.
fn set_instance_hidden(e: &mut Engine, this: Ptr, id: u32, hidden: u8) -> bool {
    let base = this.addr();
    e.with_stack(4, |e, result| {
        let found = e.call(0x0085_3130, &args![base + 0x14, id, result]).u32() as u8;
        if found == 0 {
            return false;
        }
        let instance = e.mem.u32(result.addr());
        e.mem.set_u8(instance + 0x1c, hidden);
        let group_index = e.mem.u32(instance + 0x18);
        let group_at = e
            .call(ARRAY_AT_00877A30, &args![base + 4, group_index])
            .u32();
        let group = e.mem.u32(group_at);
        e.mem.set_u8(group + 0x1c, 0);
        true
    })
}

// Translated from 006f9230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall on a `BGSDistantTreeBlock`, two words: the instance id and a byte; `RET
/// 8`): hides the instance (see [`set_instance_hidden`]); when it is not in the map and the
/// byte is set, adds the id (by the address of its argument word) to `TreesQueuedToHide`
/// (`+0x34`; `007cb2e0`, thiscall).
pub fn fn_006f9230(e: &mut Engine, this: Ptr, id: u32, queue: u32) {
    if set_instance_hidden(e, this, id, 1) {
        return;
    }
    if queue as u8 != 0 {
        e.with_stack(4, |e, argument| {
            e.mem.set_u32(argument.addr(), id);
            e.call(0x007c_b2e0, &args![this.addr() + 0x34, argument]);
        });
    }
}

// Translated from 006f92a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall on a `BGSDistantTreeBlock`, one word: the instance id; `RET 4`): shows the
/// instance (see [`set_instance_hidden`] with 0).
pub fn fn_006f92a0(e: &mut Engine, this: Ptr, id: u32) {
    set_instance_hidden(e, this, id, 0);
}

// Translated from 006f9360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f9360 - the compiler's exception frame (FS:[0] chain) is not translated
/// Unnamed: the `BGSDistantTreeBlockLoadTask` constructor (by its vtable `0106ded0`; thiscall,
/// six words, `RET 0x18`, the last not read). The base constructor `00c3ce60(this, 3)`; the
/// vtable; `pBlock`, `bFinishedQueuing` 0, `iX`, `iY`, `iLevel`, `bCanceled` 0; the file name
/// ([`fn_006f8890`] with the worldspace, the level and the low 16 bits of x and y) into a
/// 0x108-byte stack buffer, of which `00c3cee0` (thiscall) gets the part after its first five
/// characters; `00c3cf60(this, 0)`; `bFileExists` 0. Ends with the cookie check. Returns
/// `this`.
pub fn fn_006f9360(
    e: &mut Engine,
    this: Ptr,
    block: u32,
    world: u32,
    x: u32,
    y: u32,
    level: u32,
    _unused_6: u32,
) -> Ptr {
    let cookie = e.global::<u32>(SECURITY_COOKIE_011C16BC);
    let base = this.addr();
    e.call(0x00c3_ce60, &args![base, 3u32]);
    e.mem.set_u32(base, VTABLE_LOAD_TASK_0106DED0);
    e.mem.set_u32(base + 0x30, block);
    e.mem.set_u8(base + 0x34, 0);
    e.mem.set_u32(base + 0x38, x);
    e.mem.set_u32(base + 0x3c, y);
    e.mem.set_u32(base + 0x40, level);
    e.mem.set_u8(base + 0x44, 0);
    e.with_stack(0x108, |e, buffer| {
        fn_006f8890(e, buffer.addr(), world, level, x & 0xffff, y & 0xffff);
        e.call(0x00c3_cee0, &args![base, buffer.addr() + 5]);
    });
    e.call(0x00c3_cf60, &args![base, 0u32]);
    e.mem.set_u8(base + 0x45, 0);
    e.call(SECURITY_CHECK_COOKIE, &args![cookie]);
    this
}

/// The vtable of `BGSDistantTreeBlockLoadTask` (RTTI).
const VTABLE_LOAD_TASK_0106DED0: u32 = 0x0106_ded0;

// Translated from 006f9470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask`'s scalar deleting destructor (the map's name; thiscall, one
/// word, `RET 4`): [`fn_006f94a0`], then with flag bit 0 `operator delete`. Returns `this`.
pub fn bgs_distant_tree_block_load_task_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006f94a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006f94c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::QueueMe` (the map's name; thiscall): sets `bFinishedQueuing`
/// and calls its own slot `0x28`.
pub fn bgs_distant_tree_block_load_task_queue_me(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x34, 1);
    e.vcall(this.addr(), 0x28, &args![]);
}

// Translated from 006f94e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::CheckFinished` (the map's name; thiscall): when `0043caa0`
/// answers true and `bFinishedQueuing` is set: with `0069b080` true the queue at `01202d98`
/// gets it (its slot `0x48`), otherwise the base `00c3c930` (thiscall).
pub fn bgs_distant_tree_block_load_task_check_finished(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(0x0043_caa0, &args![base]).u32() as u8 == 0 || e.mem.u8(base + 0x34) == 0 {
        return;
    }
    if e.call(0x0069_b080, &args![base]).u32() as u8 != 0 {
        let queue = e.global::<u32>(0x0120_2d98);
        e.vcall(queue, 0x48, &args![base]);
    } else {
        e.call(0x00c3_c930, &args![base]);
    }
}

// Translated from 006f9540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::Cancel` (the map's name; thiscall, two words, `RET 8`): sets
/// `bCanceled`, then the base `00c3cb70` with both words.
pub fn bgs_distant_tree_block_load_task_cancel(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.mem.set_u8(this.addr() + 0x44, 1);
    e.call(0x00c3_cb70, &args![this, first, second]);
}

// Translated from 006f9570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::Run` (the map's name; thiscall): opens the file
/// (`00afdf20(0045cd60(this), 0, 0x800, 1)`, cdecl); with a file and `bCanceled` clear the block
/// reads it (`006f8b40(pBlock, file)`), the file is deleted (its slot 0 with 1), `bFileExists`
/// is set and the block's `006f9300` runs; otherwise `bFileExists` is cleared.
pub fn bgs_distant_tree_block_load_task_run(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let name = e.call(0x0045_cd60, &args![base]).u32();
    let file = e
        .call(0x00af_df20, &args![name, 0u32, 0x800u32, 1u32])
        .u32();
    if file == 0 || e.mem.u8(base + 0x44) != 0 {
        e.mem.set_u8(base + 0x45, 0);
        return;
    }
    let block = e.mem.u32(base + 0x30);
    e.call(0x006f_8b40, &args![block, file]);
    if file != 0 {
        e.vcall(file, 0, &args![1u32]);
    }
    e.mem.set_u8(base + 0x45, 1);
    let block = e.mem.u32(base + 0x30);
    e.call(0x006f_9300, &args![block]);
}

// Translated from 006f9610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::Finish` (the map's name; thiscall): with `bFileExists` set
/// and `bCanceled` clear, the block's `006f88e0` (thiscall) runs.
pub fn bgs_distant_tree_block_load_task_finish(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + 0x45) != 0 && e.mem.u8(base + 0x44) == 0 {
        let block = e.mem.u32(base + 0x30);
        e.call(0x006f_88e0, &args![block]);
    }
}

/// The vtables of `BSMap<unsigned int, InstanceData*>`, `BSMap<unsigned int, TreeGroup*>`,
/// their `BSMapBase` bases, and `BSSimpleArray<TreeGroup*, 1024>` (RTTI).
const VTABLE_INSTANCE_MAP_0106DF38: u32 = 0x0106_df38;
const VTABLE_GROUP_MAP_0106DF58: u32 = 0x0106_df58;
const VTABLE_INSTANCE_MAP_BASE_0106DFA0: u32 = 0x0106_dfa0;
const VTABLE_GROUP_MAP_BASE_0106DFC0: u32 = 0x0106_dfc0;
const VTABLE_GROUP_ARRAY_0106DF8C: u32 = 0x0106_df8c;

// Translated from 006f9680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMap<unsigned int, InstanceData*>` constructor (by its vtable; thiscall, one
/// word: the bucket count; `RET 4`): the base [`fn_006f98c0`], then the vtable `0106df38`.
pub fn fn_006f9680(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    fn_006f98c0(e, this, buckets);
    e.mem.set_u32(this.addr(), VTABLE_INSTANCE_MAP_0106DF38);
    this
}

// Translated from 006f96b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMap<unsigned int, TreeGroup*>` constructor (by its vtable; thiscall, one
/// word: the bucket count; `RET 4`): the base [`fn_006f9a00`], then the vtable `0106df58`.
pub fn fn_006f96b0(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    fn_006f9a00(e, this, buckets);
    e.mem.set_u32(this.addr(), VTABLE_GROUP_MAP_0106DF58);
    this
}

// Translated from 006f9790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an array resize for 0x20-byte elements (thiscall, one word: the new size; `RET 4`;
/// the array keeps its buffer at `+4`, size at `+8` and capacity at `+0xc`). A size other than
/// the capacity: entries past it are destroyed first (`0072ba80(buffer + size * 0x20, old -
/// size)`, thiscall) and the size lowered, then the buffer is resized ([`fn_006f9eb0`] with the
/// new size and the size) and the capacity set.
pub fn fn_006f9790(e: &mut Engine, this: Ptr, count: u32) {
    let base = this.addr();
    if count == e.mem.u32(base + 0xc) {
        return;
    }
    if count < e.mem.u32(base + 8) {
        let excess = e.mem.u32(base + 8).wrapping_sub(count);
        let start = (count << 5).wrapping_add(e.mem.u32(base + 4));
        e.call(0x0072_ba80, &args![base, start, excess]);
        e.mem.set_u32(base + 8, count);
    }
    let size = e.mem.u32(base + 8);
    fn_006f9eb0(e, this, count, size);
    e.mem.set_u32(base + 0xc, count);
}

// Translated from 006f9870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSSimpleArray<TreeGroup*, 1024>` constructor (by its vtable; thiscall): the
/// vtable `0106df8c`, then `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_006f9870(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_GROUP_ARRAY_0106DF8C);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006f98a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSSimpleArray<TreeGroup*, 1024>` destructor (thiscall): the vtable `0106df8c`,
/// then `008454f0(this, 1)`.
pub fn fn_006f98a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_GROUP_ARRAY_0106DF8C);
    e.call(ARRAY_FREE_008454F0, &args![this, 1u32]);
}

/// The `BSMapBase` constructor shape (thiscall, one word: the bucket count): the vtable, the
/// count at `+4`, 0 at `+0xc`; with buckets, a zeroed table of `count * 4` bytes at `+8`
/// (`operator new`, `memset` `00ec61c0`). Returns `this`.
fn map_base_construct(e: &mut Engine, this: Ptr, buckets: u32, vtable: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, vtable);
    e.mem.set_u32(base + 4, buckets);
    e.mem.set_u32(base + 0xc, 0);
    if buckets != 0 {
        let bytes = e.mem.u32(base + 4) << 2;
        let table = e.call(OPERATOR_NEW, &args![bytes]).u32();
        e.mem.set_u32(base + 8, table);
        let bytes = e.mem.u32(base + 4) << 2;
        let table = e.mem.u32(base + 8);
        e.call(0x00ec_61c0, &args![table, 0u32, bytes]);
    }
    this
}

// Translated from 006f98c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMapBase<unsigned int, InstanceData*>` constructor (by its vtable `0106dfa0`;
/// thiscall, one word, `RET 4`): see [`map_base_construct`].
pub fn fn_006f98c0(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    map_base_construct(e, this, buckets, VTABLE_INSTANCE_MAP_BASE_0106DFA0)
}

// Translated from 006f9940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f9940 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `BSMap<unsigned int, InstanceData*>` destructor (thiscall): the vtable
/// `0106df38`, the entries removed (`00438af0`, thiscall), then the base [`fn_006f99a0`].
pub fn fn_006f9940(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_INSTANCE_MAP_0106DF38);
    e.call(0x0043_8af0, &args![this]);
    fn_006f99a0(e, this);
}

// Translated from 006f99a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMapBase<unsigned int, InstanceData*>` destructor (thiscall): the vtable
/// `0106dfa0`; a set bucket table (`+8`) is freed (`operator delete`).
pub fn fn_006f99a0(e: &mut Engine, this: Ptr) {
    map_base_destroy(e, this, VTABLE_INSTANCE_MAP_BASE_0106DFA0);
}

/// The `BSMapBase` destructor shape: the vtable, then the bucket table at `+8` freed when set.
fn map_base_destroy(e: &mut Engine, this: Ptr, vtable: u32) {
    let base = this.addr();
    e.mem.set_u32(base, vtable);
    let table = e.mem.u32(base + 8);
    if table != 0 {
        e.call(OPERATOR_DELETE, &args![table]);
    }
}

// Translated from 006f9a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMapBase<unsigned int, TreeGroup*>` constructor (by its vtable `0106dfc0`;
/// thiscall, one word, `RET 4`): see [`map_base_construct`].
pub fn fn_006f9a00(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    map_base_construct(e, this, buckets, VTABLE_GROUP_MAP_BASE_0106DFC0)
}

// Translated from 006f9a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<unsigned int, InstanceData*>::SetAt` (the map's name; thiscall, two words: key
/// and value; `RET 8`). The bucket is the map's slot 4 on the key; its chain (`+0` the next
/// item, `+4` the key) is searched with slot 8 (key, item key; result in `AL`). A match is
/// cleared by slot `0x10` and set again by slot `0xc` (item, key, value); otherwise a new item
/// (slot `0x14`) is set by slot `0xc`, put in front of the bucket and the count (`+0xc`) grows.
pub fn bs_map_base_set_at(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let base = this.addr();
    let bucket = e.vcall(base, 4, &args![key]).u32();
    let bucket_at = |e: &Engine| e.mem.u32(base + 8).wrapping_add(bucket.wrapping_mul(4));
    let mut item = e.mem.u32(bucket_at(e));
    while item != 0 {
        let item_key = e.mem.u32(item + 4);
        if e.vcall(base, 8, &args![key, item_key]).u32() as u8 != 0 {
            e.vcall(base, 0x10, &args![item]);
            e.vcall(base, 0xc, &args![item, key, value]);
            return;
        }
        item = e.mem.u32(item);
    }
    let item = e.vcall(base, 0x14, &args![]).u32();
    e.vcall(base, 0xc, &args![item, key, value]);
    let head = e.mem.u32(bucket_at(e));
    e.mem.set_u32(item, head);
    let at = bucket_at(e);
    e.mem.set_u32(at, item);
    let count = e.mem.u32(base + 0xc).wrapping_add(1);
    e.mem.set_u32(base + 0xc, count);
}

// Translated from 006f9b80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f9b80 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `BSMap<unsigned int, TreeGroup*>` destructor (thiscall): the vtable `0106df58`,
/// the entries removed (`00438af0`, thiscall), then the base [`fn_006f9be0`].
pub fn fn_006f9b80(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_GROUP_MAP_0106DF58);
    e.call(0x0043_8af0, &args![this]);
    fn_006f9be0(e, this);
}

// Translated from 006f9be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSMapBase<unsigned int, TreeGroup*>` destructor (thiscall): see
/// [`map_base_destroy`] with the vtable `0106dfc0`.
pub fn fn_006f9be0(e: &mut Engine, this: Ptr) {
    map_base_destroy(e, this, VTABLE_GROUP_MAP_BASE_0106DFC0);
}

// Translated from 006f9c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an array clear (thiscall, one word: a byte; `RET 4`): with a buffer (`+4`), its
/// entries are destroyed (`006fa090(this, buffer, size)`); with the byte set the buffer is freed
/// (`006a8500`, thiscall) and the buffer and capacity cleared; the size is cleared.
pub fn fn_006f9c70(e: &mut Engine, this: Ptr, free: u32) {
    let base = this.addr();
    if e.mem.u32(base + 4) == 0 {
        return;
    }
    let buffer = e.mem.u32(base + 4);
    let size = e.mem.u32(base + 8);
    e.call(0x006f_a090, &args![base, buffer, size]);
    if free as u8 != 0 {
        e.call(0x006a_8500, &args![base]);
        e.mem.set_u32(base + 4, 0);
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base + 8, 0);
}

// Translated from 006f9eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an array buffer resize (thiscall, two words: the new capacity and the count to
/// keep; `RET 8`). Without a buffer it allocates one (slot 4) and sets the capacity (`+0xc`);
/// when the count equals the capacity it reallocates (slot `0xc` with the buffer and the new
/// capacity); otherwise it allocates a new buffer (slot 4), copies the count over
/// (`006dc640(this, new, old, count)`), frees the old one (`006a8500`) and keeps the new.
pub fn fn_006f9eb0(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    let base = this.addr();
    if e.mem.u32(base + 4) == 0 {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    } else if count == e.mem.u32(base + 0xc) {
        let old = e.mem.u32(base + 4);
        let buffer = e.vcall(base, 0xc, &args![old, capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
    } else {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        let old = e.mem.u32(base + 4);
        e.call(0x006d_c640, &args![base, buffer, old, count]);
        e.call(0x006a_8500, &args![base]);
        e.mem.set_u32(base + 4, buffer);
    }
}

// Translated from 006fa180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an array constructor body (thiscall, two words: capacity and size; `RET 8`):
/// clears buffer, size and capacity; the capacity is raised to the size when below it; with a
/// capacity a buffer comes from slot 4 and the capacity is kept; with a size its entries are
/// constructed (`006f9ff0(this, buffer, size)`) and the size kept.
pub fn fn_006fa180(e: &mut Engine, this: Ptr, capacity: u32, size: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let capacity = if size > capacity { size } else { capacity };
    if capacity > 0 {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    }
    if size > 0 {
        let buffer = e.mem.u32(base + 4);
        e.call(0x006f_9ff0, &args![base, buffer, size]);
        e.mem.set_u32(base + 8, size);
    }
}

// The functions below (n089). `TreeType` (0x20 bytes, the entries of `TreeTypes` at `011d85d0`):
// width `+4`, height `+8`, the four texture coordinates `+0xc..+0x18`, the geometry data's smart
// pointer `+0x1c`. `TreeGroup` (0x20 bytes): the tree type `+0`, `spGeometry` `+4`, `Instances`
// (a `BSSimpleArray` of 0x20-byte `InstanceData`) `+8`, its index in `TreeGroups` `+0x18`,
// `bShaderPropertyUpToDate` `+0x1c`. `InstanceData` (0x20 bytes): position `+0`, rotation and scale
// up to `+0x10`, the instance id `+0x14`, `uiGroup` `+0x18`, `bHidden` `+0x1c` (offsets from the
// PC code). The block also keeps `TreeGroupMap` (`BSMap<unsigned int, TreeGroup*>`) at `+0x24`,
// a loaded flag at `+0x48` and an attached flag at `+0x49`.

/// The memory-context scope guard (`00404eb0`, thiscall on a 4-byte stack local, four words: the
/// context `0x25`, 1, the source file name and its line) and its end (`00404ee0`, thiscall).
const SCOPE_GUARD_ENTER_00404EB0: u32 = 0x0040_4eb0;
const SCOPE_GUARD_LEAVE_00404EE0: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\...\BGSDistantTreeBlock.cpp"` (`.rdata`).
const SOURCE_FILE_0106DD30: u32 = 0x0106_dd30;
/// `BSDistantTreeShader::GetMaxInstances` (the map's name; no arguments).
const GET_MAX_INSTANCES_00BB6030: u32 = 0x00bb_6030;
/// `00aa13e0` (cdecl, one word): the Gamebryo allocator (`NiNew`).
const NI_ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `006b3120` (thiscall, one word: the index): the address of a `BSSimpleArray` entry.
const ARRAY_ENTRY_006B3120: u32 = 0x006b_3120;
/// `00462d80` (thiscall on the file, two words: destination and byte count): reads from a file.
const FILE_READ_00462D80: u32 = 0x0046_2d80;
/// `00633c90` (thiscall, one word): constructs a smart pointer holding the word.
const POINTER_CONSTRUCT_00633C90: u32 = 0x0063_3c90;
/// `0045cec0` (thiscall): a smart pointer's destructor.
const POINTER_DESTROY_0045CEC0: u32 = 0x0045_cec0;
/// `006e5cc0` (thiscall, one word: the source smart pointer's address): assigns a smart pointer.
const POINTER_ASSIGN_006E5CC0: u32 = 0x006e_5cc0;
/// `0043d410` (thiscall, three words): sets a 3-float vector.
const VECTOR_SET_0043D410: u32 = 0x0043_d410;
/// `00a59c60` (thiscall, one word: a vector's address): sets an object's local translation.
const SET_TRANSLATE_00A59C60: u32 = 0x00a5_9c60;
/// `00439410` (thiscall, one word): attaches a property (`NiAVObject::AttachProperty`, the map's
/// name at `006f8c40`'s call sites).
const ATTACH_PROPERTY_00439410: u32 = 0x0043_9410;
/// `008041a0` (thiscall): the block's node smart pointer.
const NODE_POINTER_008041A0: u32 = 0x0080_41a0;
/// The `BSSimpleArray<BGSDistantTreeBlock::InstanceData,1024>` vtable (RTTI) that `006f9740` and
/// `006f9770` store.
const VTABLE_INSTANCE_ARRAY_0106DF78: u32 = 0x0106_df78;
/// `0.5` (`double`, `.rdata`).
const HALF_01011588: u32 = 0x0101_1588;
/// `-1.0` (`float`, `.rdata`): the first component of every billboard normal.
const NORMAL_X_01012054: u32 = 0x0101_2054;
/// `2.0` (`float`, `.rdata`): the argument of `004579e0`.
const TWO_010162C0: u32 = 0x0101_62c0;
/// `FLT_MAX` (`float`, `.rdata`).
const FLOAT_MAX_01016970: u32 = 0x0101_6970;

/// `count * size` as the compiler's `operator new[]` size check computes it: `0xffffffff` when
/// the product overflows 32 bits (`MUL`, `SETO`, `NEG`, `OR`).
fn array_bytes(count: u32, size: u32) -> u32 {
    let product = u64::from(count) * u64::from(size);
    if product >> 32 != 0 {
        0xffff_ffff
    } else {
        product as u32
    }
}

/// `new T[count]` for a `size`-byte element whose constructor is `006815c0`: `operator new`, then
/// with a block the vector constructor iterator (`00401050`, stdcall: block, size, count,
/// constructor). Returns the block or 0.
fn new_vector(e: &mut Engine, count: u32, size: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![array_bytes(count, size)]).u32();
    if block == 0 {
        return 0;
    }
    e.call(0x0040_1050, &args![block, size, count, 0x0068_15c0u32]);
    block
}

/// The quieted bit pattern of the `float` at `addr` (`FLD dword`/`FSTP dword`).
fn float_bits(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

// Translated from 006f7a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f7a80 - the compiler's exception frame (FS:[0] chain) is not translated
/// `BGSDistantTreeBlock::AllocateStaticDataForTreeType` (Xbox PDB; cdecl, one word: the
/// `TreeType`). Builds the billboard geometry for `GetMaxInstances()` trees: eight vertices per
/// tree (two crossed quads of the type's half width and height, `width * 0.5` through the
/// `double` at `01011588`) in three arrays made by `new_vector` (positions and normals 12 bytes,
/// texture coordinates 8), and six 16-bit indices per quad. The exe writes every one of a tree's
/// eight normals into the tree's first normal (`-1, 1, 1`, `004a0c10` on it each time), as
/// translated. The tree index of every vertex (`vertex >> 3`, a `float`) goes into an
/// `NiAdditionalGeometryData` (`00aa13e0(0x2c)`, `00537bf0` with the 16-bit vertex count; one
/// block `00a72ca0(1)`, one stream `00a730e0(1)`, `00a72bb0(0, data, bytes, 1)`,
/// `00a726a0(0, 0, 0, 1, count, 4, 4)`) and the temporary array is freed. The `NiTriShapeData`
/// (`00aa13e0(0x58)`, `00a7b630` with the counts and arrays) goes into a stack smart pointer,
/// gets the additional data (`00a67260`) and is assigned to the type's `+0x1c`. The half height
/// (`-0x34`) and the result of `004579e0(2.0)` are stored to locals the function never reads.
pub fn bgs_distant_tree_block_allocate_static_data_for_tree_type(e: &mut Engine, tree_type: u32) {
    e.with_stack(8, |e, frame| {
        let shape_pointer = frame.addr();
        let guard = frame.addr() + 4;
        e.call(
            SCOPE_GUARD_ENTER_00404EB0,
            &args![guard, 0x25u32, 1u32, SOURCE_FILE_0106DD30, 0x95u32],
        );
        let quads = e.call(GET_MAX_INSTANCES_00BB6030, &args![]).u32() << 2;
        let vertices = e.call(GET_MAX_INSTANCES_00BB6030, &args![]).u32() << 3;
        let positions = new_vector(e, vertices, 0xc);
        let uvs = new_vector(e, vertices, 8);
        let normals = new_vector(e, vertices, 0xc);
        let indices = e
            .call(OPERATOR_NEW, &args![array_bytes(quads.wrapping_mul(3), 2)])
            .u32();
        let half_constant: f64 = e.global(HALF_01011588);
        let width = float_bits(e, tree_type.wrapping_add(4));
        let half = x87_store32(x87_mul(x87_load32(width), half_constant));
        let height = float_bits(e, tree_type.wrapping_add(8));
        let _half_height = x87_store32(x87_mul(x87_load32(height), half_constant));
        let u_left = float_bits(e, tree_type.wrapping_add(0xc));
        let v_top = float_bits(e, tree_type.wrapping_add(0x10));
        let u_right = float_bits(e, tree_type.wrapping_add(0x14));
        let v_bottom = float_bits(e, tree_type.wrapping_add(0x18));
        let two = float_bits(e, TWO_010162C0);
        let _unused_result = e.call(0x0045_79e0, &args![two]).f64();
        let neg_half = half ^ 0x8000_0000;
        let normal_x = float_bits(e, NORMAL_X_01012054);
        let one = 1.0f32.to_bits();
        // (x, y, z, u, v) of the eight vertices.
        let corners = [
            (neg_half, 0, height, u_left, v_top),
            (half, 0, height, u_right, v_top),
            (neg_half, 0, 0, u_left, v_bottom),
            (half, 0, 0, u_right, v_bottom),
            (0, neg_half, height, u_left, v_top),
            (0, half, height, u_right, v_top),
            (0, neg_half, 0, u_left, v_bottom),
            (0, half, 0, u_right, v_bottom),
        ];
        let mut vertex = 0u32;
        while vertex < vertices {
            let normal = normals.wrapping_add(vertex.wrapping_mul(0xc));
            for (k, (x, y, z, u, v)) in (0u32..).zip(corners) {
                let position = positions.wrapping_add(vertex.wrapping_add(k).wrapping_mul(0xc));
                e.mem.set_u32(position, x);
                e.mem.set_u32(position.wrapping_add(4), y);
                e.mem.set_u32(position.wrapping_add(8), z);
                let uv = uvs.wrapping_add(vertex.wrapping_mul(8)).wrapping_add(k * 8);
                e.mem.set_u32(uv, u);
                e.mem.set_u32(uv.wrapping_add(4), v);
                e.mem.set_u32(normal, normal_x);
                e.mem.set_u32(normal.wrapping_add(4), one);
                e.mem.set_u32(normal.wrapping_add(8), one);
                e.call(0x004a_0c10, &args![normal]);
            }
            vertex = vertex.wrapping_add(8);
        }
        let mut first = 0u16;
        let mut index = 0u32;
        while index < quads.wrapping_mul(3) {
            let at = indices.wrapping_add(index.wrapping_mul(2));
            for (k, offset) in (0u32..).zip([0u16, 1, 2, 1, 3, 2]) {
                e.mem
                    .set_u16(at.wrapping_add(k * 2), first.wrapping_add(offset));
            }
            first = first.wrapping_add(4);
            index = index.wrapping_add(6);
        }
        let tree_indices = e.call(OPERATOR_NEW, &args![array_bytes(vertices, 4)]).u32();
        let mut vertex = 0u32;
        while vertex < vertices {
            e.mem.set_u32(
                tree_indices.wrapping_add(vertex.wrapping_mul(4)),
                x87_store32(f64::from(vertex >> 3)),
            );
            vertex = vertex.wrapping_add(1);
        }
        let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x2cu32]).u32();
        let additional = if block != 0 {
            e.call(0x0053_7bf0, &args![block, vertices & 0xffff]).u32()
        } else {
            0
        };
        e.call(0x00a7_2ca0, &args![additional, 1u32]);
        e.call(0x00a7_30e0, &args![additional, 1u32]);
        e.call(
            0x00a7_2bb0,
            &args![additional, 0u32, tree_indices, vertices << 2, 1u32],
        );
        e.call(
            0x00a7_26a0,
            &args![
                additional,
                0u32,
                0u32,
                0u32,
                1u32,
                vertices & 0xffff,
                4u32,
                4u32
            ],
        );
        e.call(OPERATOR_DELETE, &args![tree_indices]);
        let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x58u32]).u32();
        let shape = if block != 0 {
            e.call(
                0x00a7_b630,
                &args![
                    block,
                    vertices & 0xffff,
                    positions,
                    normals,
                    0u32,
                    uvs,
                    1u32,
                    0u32,
                    quads & 0xffff,
                    indices
                ],
            )
            .u32()
        } else {
            0
        };
        e.call(POINTER_CONSTRUCT_00633C90, &args![shape_pointer, shape]);
        let held = e.call(POINTER_GET_00559450, &args![shape_pointer]).u32();
        e.call(0x00a6_7260, &args![held, additional]);
        e.call(
            POINTER_ASSIGN_006E5CC0,
            &args![tree_type.wrapping_add(0x1c), shape_pointer],
        );
        e.call(POINTER_SET_0066B0D0, &args![shape_pointer, 0u32]);
        e.call(POINTER_DESTROY_0045CEC0, &args![shape_pointer]);
        e.call(SCOPE_GUARD_LEAVE_00404EE0, &args![guard]);
    });
}

// Translated from 006f8470 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f8470 - the compiler's exception frame (FS:[0] chain) is not translated
/// `BGSDistantTreeBlock::AddTree` (Xbox PDB; thiscall, two words: the tree type and the address
/// of an `InstanceData`; `RET 8`). Inside the memory-context guard (line `0x14b`): the type's
/// group comes from `TreeGroupMap` (`+0x24`, `00853130` into a local set to 0); when there is
/// none or it is full (`GetMaxInstances` instances), a type past `TreeTypes` is reported
/// (`005b5e40`, `"Invalid LOD tree type %i found. ..."` `0106de40`) and the function ends;
/// otherwise a new group (`operator new(0x20)`, [`fn_006f8650`]) gets the type's entry, is added
/// to `TreeGroups` (`+4`, `007cb2e0`) and the map ([`bs_map_base_set_at`]), takes its index, has
/// its instances sized to `GetMaxInstances` ([`fn_006f9790`]) and its shader property marked out
/// of date. The instance is copied into the group ([`fn_006f9800`]), takes the group's index and
/// shown, and is put into `InstanceMap` (`+0x14`) under its id (`+0x14`).
pub fn bgs_distant_tree_block_add_tree(e: &mut Engine, this: Ptr, tree_type: u32, instance: u32) {
    let base = this.addr();
    e.with_stack(8, |e, frame| {
        let group_slot = frame.addr();
        let guard = frame.addr() + 4;
        e.call(
            SCOPE_GUARD_ENTER_00404EB0,
            &args![guard, 0x25u32, 1u32, SOURCE_FILE_0106DD30, 0x14bu32],
        );
        e.mem.set_u32(group_slot, 0);
        let found = e
            .call(0x0085_3130, &args![base + 0x24, tree_type, group_slot])
            .u8();
        let mut needs_group = true;
        if found != 0 && e.mem.u32(group_slot) != 0 {
            let group = e.mem.u32(group_slot);
            let count = e
                .call(ARRAY_COUNT_0044DDC0, &args![group.wrapping_add(8)])
                .u32();
            let max = e.call(GET_MAX_INSTANCES_00BB6030, &args![]).u32();
            needs_group = count == max;
        }
        if needs_group {
            let types = e
                .call(ARRAY_COUNT_0044DDC0, &args![STATIC_ARRAY_011D85D0])
                .u32();
            if tree_type >= types {
                e.call(0x005b_5e40, &args![0x0106_de40u32, tree_type]);
                e.call(SCOPE_GUARD_LEAVE_00404EE0, &args![guard]);
                return;
            }
            let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
            let group = if block != 0 {
                fn_006f8650(e, Ptr::new(block)).addr()
            } else {
                0
            };
            e.mem.set_u32(group_slot, group);
            let entry = e
                .call(
                    ARRAY_ENTRY_006B3120,
                    &args![STATIC_ARRAY_011D85D0, tree_type],
                )
                .u32();
            let group = e.mem.u32(group_slot);
            e.mem.set_u32(group, entry);
            e.call(0x007c_b2e0, &args![base + 4, group_slot]);
            let group = e.mem.u32(group_slot);
            bs_map_base_set_at(e, Ptr::new(base + 0x24), tree_type, group);
            let index = e
                .call(ARRAY_COUNT_0044DDC0, &args![base + 4])
                .u32()
                .wrapping_sub(1);
            let group = e.mem.u32(group_slot);
            e.mem.set_u32(group.wrapping_add(0x18), index);
            let max = e.call(GET_MAX_INSTANCES_00BB6030, &args![]).u32();
            let group = e.mem.u32(group_slot);
            fn_006f9790(e, Ptr::new(group.wrapping_add(8)), max);
            let group = e.mem.u32(group_slot);
            e.mem.set_u8(group.wrapping_add(0x1c), 0);
        }
        let instances = e.mem.u32(group_slot).wrapping_add(8);
        fn_006f9800(e, Ptr::new(instances), instance);
        let instances = e.mem.u32(group_slot).wrapping_add(8);
        let last = e
            .call(ARRAY_COUNT_0044DDC0, &args![instances])
            .u32()
            .wrapping_sub(1);
        let instances = e.mem.u32(group_slot).wrapping_add(8);
        let added = e.call(ARRAY_ENTRY_006B3120, &args![instances, last]).u32();
        let group = e.mem.u32(group_slot);
        let group_index = e.mem.u32(group.wrapping_add(0x18));
        e.mem.set_u32(added.wrapping_add(0x18), group_index);
        e.mem.set_u8(added.wrapping_add(0x1c), 0);
        let id = e.mem.u32(added.wrapping_add(0x14));
        bs_map_base_set_at(e, Ptr::new(base + 0x14), id, added);
        e.call(SCOPE_GUARD_LEAVE_00404EE0, &args![guard]);
    });
}

// Translated from 006f8650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f8650 - the compiler's exception frame (FS:[0] chain) is not translated
/// Unnamed: the `TreeGroup` constructor (thiscall): `spGeometry` (`+4`) made empty
/// (`00633c90(0)`), `Instances` (`+8`) constructed ([`fn_006f9740`]). Returns `this`.
pub fn fn_006f8650(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(POINTER_CONSTRUCT_00633C90, &args![this.addr() + 4, 0u32]);
    fn_006f9740(e, Ptr::new(this.addr() + 8));
    this
}

// Translated from 006f86b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word: a `TreeType`'s address): appends a copy of it to `TreeTypes`
/// (`011d85d0`, [`fn_006f9c20`]) and builds the static data of the last entry (`006b3140`, then
/// [`bgs_distant_tree_block_allocate_static_data_for_tree_type`]).
pub fn fn_006f86b0(e: &mut Engine, tree_type: u32) {
    fn_006f9c20(e, Ptr::new(STATIC_ARRAY_011D85D0), tree_type);
    let last = e.call(0x006b_3140, &args![STATIC_ARRAY_011D85D0]).u32();
    bgs_distant_tree_block_allocate_static_data_for_tree_type(e, last);
}

// Translated from 006f8800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `TreeGroup` scalar deleting destructor (thiscall, one word, `RET 4`): the
/// destructor [`fn_006f8830`], then with flag bit 0 `operator delete`. Returns `this`.
pub fn fn_006f8800(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006f8830(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006f8830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f8830 - the compiler's exception frame (FS:[0] chain) is not translated
/// Unnamed: the `TreeGroup` destructor (thiscall): `Instances` (`+8`, [`fn_006f9770`]), then the
/// smart pointer at `+4` (`0045cec0`).
pub fn fn_006f8830(e: &mut Engine, this: Ptr) {
    fn_006f9770(e, Ptr::new(this.addr() + 8));
    e.call(POINTER_DESTROY_0045CEC0, &args![this.addr() + 4]);
}

// Translated from 006f88e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall on a `BGSDistantTreeBlock`): sets the loaded flag (`+0x48`), calls
/// `00929260(1)` on what `00559450` gives for the word at `+0x44` (the word itself is `ECX`), then
/// `006f74f0(this, 0)` (the map names it `NiPointer<QueuedFile>::operator=`).
pub fn fn_006f88e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 0x48, 1);
    let pointer = e.mem.u32(base + 0x44);
    let held = e.call(POINTER_GET_00559450, &args![pointer]).u32();
    e.call(0x0092_9260, &args![held, 1u32]);
    e.call(0x006f_74f0, &args![base, 0u32]);
}

// Translated from 006f8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall, two words: the result's address and a `TreeGroup`; `RET 8`; `this` is not
/// read): the group's bound. A bound local (`006240d0`); the minimum starts at `FLT_MAX`
/// (`01016970`) and the maximum at `-FLT_MAX` (`00416870`); over the group's instances each
/// component goes through `0040ebd0` (minimum) and `00404010` (maximum), cdecl on two floats.
/// The centre comes from `00439ef0(max, min)`, `0053d280` on that with the float `2.0` at
/// `010162c0`, and `00439e90(min, ..)`; the radius is `00457990` of `00439ef0(max, centre)` plus
/// `0084d030` of the geometry's slot `0xf8` (`FADD qword`); `004a51d0` sets the bound and its 16 bytes are copied to the result.
/// Returns the result's address.
pub fn fn_006f8910(e: &mut Engine, _this: Ptr, result: u32, group: u32) -> u32 {
    e.with_stack(0x70, |e, frame| {
        // Offsets from the frame of the exe's locals (EBP - 0x70).
        let at = |offset: u32| frame.addr() + 0x70 - offset;
        let bound = at(0x28);
        let min = at(0x18);
        let max = at(0x40);
        let centre = at(0xc);
        e.call(0x0062_40d0, &args![bound]);
        let limit = float_bits(e, FLOAT_MAX_01016970);
        e.call(0x0041_6870, &args![min, limit, limit, limit]);
        let negative = limit ^ 0x8000_0000;
        e.call(0x0041_6870, &args![max, negative, negative, negative]);
        let instances = group.wrapping_add(8);
        let count = e.call(ARRAY_COUNT_0044DDC0, &args![instances]).u32();
        let mut index = 0u32;
        while index < count {
            for component in [0u32, 4, 8] {
                for (callee, extreme) in [(0x0040_ebd0u32, min), (0x0040_4010, max)] {
                    let entry = e.call(ARRAY_ENTRY_006B3120, &args![instances, index]).u32();
                    let value = float_bits(e, entry.wrapping_add(component));
                    let current = float_bits(e, extreme + component);
                    let chosen = e.call(callee, &args![current, value]).f64();
                    e.mem.set_u32(extreme + component, x87_store32(chosen));
                }
            }
            index = index.wrapping_add(1);
        }
        let factor = float_bits(e, TWO_010162C0);
        let extent = e.call(0x0043_9ef0, &args![max, at(0x50), min]).u32();
        let scaled = e.call(0x0053_d280, &args![extent, at(0x5c), factor]).u32();
        e.call(0x0043_9e90, &args![min, centre, scaled]);
        let geometry = e
            .call(POINTER_GET_00559450, &args![group.wrapping_add(4)])
            .u32();
        let object = e.vcall(geometry, 0xf8, &args![]).u32();
        // `FSTP qword` into a local, read back by `FADD qword`.
        let scale = e.call(0x0084_d030, &args![object]).f64();
        let offset = e.call(0x0043_9ef0, &args![max, at(0x68), centre]).u32();
        let length = e.call(0x0045_7990, &args![offset]).f64();
        let radius = x87_store32(x87_add(length, scale));
        e.call(0x004a_51d0, &args![bound, centre, radius]);
        for word in [0u32, 4, 8, 0xc] {
            let value = e.mem.u32(bound + word);
            e.mem.set_u32(result.wrapping_add(word), value);
        }
        result
    })
}

// Translated from 006f8b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f8b40 - the compiler's exception frame (FS:[0] chain) is not translated
/// `BGSDistantTreeBlock::Read` (Xbox PDB; thiscall, one word: the file; `RET 4`). Inside the
/// memory-context guard (line `0x204`): the block is cleared ([`fn_006f86e0`]), then the file
/// gives a type count and for each type its id and an instance count, and each instance (0x20
/// bytes into a local constructed by `006240d0`) is added
/// ([`bgs_distant_tree_block_add_tree`]). Reads go through `00462d80` (thiscall on the file).
pub fn bgs_distant_tree_block_read(e: &mut Engine, this: Ptr, file: u32) {
    e.with_stack(0x30, |e, frame| {
        let instance = frame.addr();
        let tree_type = frame.addr() + 0x20;
        let instance_count = frame.addr() + 0x24;
        let type_count = frame.addr() + 0x28;
        let guard = frame.addr() + 0x2c;
        e.call(
            SCOPE_GUARD_ENTER_00404EB0,
            &args![guard, 0x25u32, 1u32, SOURCE_FILE_0106DD30, 0x204u32],
        );
        fn_006f86e0(e, this);
        e.call(FILE_READ_00462D80, &args![file, type_count, 4u32]);
        let mut type_index = 0u32;
        while type_index < e.mem.u32(type_count) {
            e.call(FILE_READ_00462D80, &args![file, tree_type, 4u32]);
            e.call(FILE_READ_00462D80, &args![file, instance_count, 4u32]);
            let mut instance_index = 0u32;
            while instance_index < e.mem.u32(instance_count) {
                e.call(0x0062_40d0, &args![instance]);
                e.call(FILE_READ_00462D80, &args![file, instance, 0x20u32]);
                let id = e.mem.u32(tree_type);
                bgs_distant_tree_block_add_tree(e, this, id, instance);
                instance_index = instance_index.wrapping_add(1);
            }
            type_index = type_index.wrapping_add(1);
        }
        e.call(SCOPE_GUARD_LEAVE_00404EE0, &args![guard]);
    });
}

// Translated from 006f8c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f8c40 - the compiler's exception frame (FS:[0] chain) is not translated
/// Unnamed (thiscall on a `BGSDistantTreeBlock`), inside the memory-context guard (line
/// `0x240`). Loaded (`+0x48`) and not yet attached (`+0x49`): for each tree group a geometry
/// (`00aa13e0(0xd8)`, `00a86dd0` with the type's shape data, or 0) goes into `spGeometry`
/// (`0066b0d0`), gets the active count (`00705b10(count * 4)`), a new shader property
/// (`00aa13e0(0x68)`, `00bb7170`, or 0) set up by [`bgs_distant_tree_block_setup_shader_property`]
/// and attached (`00439410`), `00b57e30(geometry, 0, 0)`, the two static properties at `011d85bc`
/// and `011d85c4` attached, is attached to the block's node (`008041a0`, `004ea9e0`, slot `0xdc`
/// with 1), updates its properties (`00a5a040`), gets a zero translation (`0043d410`,
/// `00a59c60`), the node (`005585e0`) a zero translation too, the geometry its bound
/// ([`fn_006f8910`], slot `0xf4` with the 16 bytes) and a zero translation again; then the
/// attached flag is set. Loaded and attached: [`bgs_distant_tree_block_hide_queued_trees`].
pub fn fn_006f8c40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x44, |e, frame| {
        let guard = frame.addr();
        let translation_a = frame.addr() + 4;
        let translation_b = frame.addr() + 0x10;
        let translation_c = frame.addr() + 0x1c;
        let bound = frame.addr() + 0x28;
        e.call(
            SCOPE_GUARD_ENTER_00404EB0,
            &args![guard, 0x25u32, 1u32, SOURCE_FILE_0106DD30, 0x240u32],
        );
        if e.mem.u8(base + 0x48) != 0 && e.mem.u8(base + 0x49) == 0 {
            let groups = base + 4;
            let mut index = 0u32;
            while index < e.call(ARRAY_COUNT_0044DDC0, &args![groups]).u32() {
                let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
                let group = e.mem.u32(group_at);
                let block = e.call(NI_ALLOCATE_00AA13E0, &args![0xd8u32]).u32();
                let geometry = if block != 0 {
                    let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
                    let tree_type = e.mem.u32(e.mem.u32(group_at));
                    let data = e
                        .call(POINTER_GET_00559450, &args![tree_type.wrapping_add(0x1c)])
                        .u32();
                    e.call(0x00a8_6dd0, &args![block, data]).u32()
                } else {
                    0
                };
                let held_at = group.wrapping_add(4);
                e.call(POINTER_SET_0066B0D0, &args![held_at, geometry]);
                let active = e
                    .call(ARRAY_COUNT_0044DDC0, &args![group.wrapping_add(8)])
                    .u32()
                    << 2;
                let held = e.call(POINTER_GET_00559450, &args![held_at]).u32();
                e.call(0x0070_5b10, &args![held, active]);
                let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x68u32]).u32();
                let property = if block != 0 {
                    e.call(0x00bb_7170, &args![block]).u32()
                } else {
                    0
                };
                bgs_distant_tree_block_setup_shader_property(e, this, property, group);
                let held = e.call(POINTER_GET_00559450, &args![held_at]).u32();
                e.call(ATTACH_PROPERTY_00439410, &args![held, property]);
                let held = e.call(POINTER_GET_00559450, &args![held_at]).u32();
                e.call(0x00b5_7e30, &args![held, 0u32, 0u32]);
                for shared in [STATIC_POINTER_011D85BC, STATIC_POINTER_011D85C4] {
                    let shared = e.call(POINTER_GET_00559450, &args![shared]).u32();
                    let held = e.call(POINTER_GET_00559450, &args![held_at]).u32();
                    e.call(ATTACH_PROPERTY_00439410, &args![held, shared]);
                }
                let node = e.call(NODE_POINTER_008041A0, &args![base]).u32();
                let node = e.call(POINTER_GET_00559450, &args![node]).u32();
                let parent = e.call(0x004e_a9e0, &args![node]).u32();
                let held = e.call(POINTER_GET_00559450, &args![held_at]).u32();
                e.vcall(parent, 0xdc, &args![held, 1u32]);
                let held = e.call(POINTER_GET_00559450, &args![held_at]).u32();
                e.call(0x00a5_a040, &args![held]);
                e.call(VECTOR_SET_0043D410, &args![translation_a, 0u32, 0u32, 0u32]);
                let held = e.call(POINTER_GET_00559450, &args![held_at]).u32();
                e.call(SET_TRANSLATE_00A59C60, &args![held, translation_a]);
                e.call(VECTOR_SET_0043D410, &args![translation_b, 0u32, 0u32, 0u32]);
                let node = e.call(NODE_POINTER_008041A0, &args![base]).u32();
                let node = e.call(POINTER_GET_00559450, &args![node]).u32();
                let object = e.call(0x0055_85e0, &args![node]).u32();
                e.call(SET_TRANSLATE_00A59C60, &args![object, translation_b]);
                let held = e.call(POINTER_GET_00559450, &args![held_at]).u32();
                let computed = fn_006f8910(e, this, bound, group);
                let words: Vec<u32> = (0..4)
                    .map(|k| e.mem.u32(computed.wrapping_add(k * 4)))
                    .collect();
                e.vcall(held, 0xf4, &words);
                e.call(VECTOR_SET_0043D410, &args![translation_c, 0u32, 0u32, 0u32]);
                let held = e.call(POINTER_GET_00559450, &args![held_at]).u32();
                e.call(SET_TRANSLATE_00A59C60, &args![held, translation_c]);
                index = index.wrapping_add(1);
            }
            e.mem.set_u8(base + 0x49, 1);
        } else if e.mem.u8(base + 0x48) != 0 && e.mem.u8(base + 0x49) != 0 {
            bgs_distant_tree_block_hide_queued_trees(e, this);
        }
        e.call(SCOPE_GUARD_LEAVE_00404EE0, &args![guard]);
    });
}

// Translated from 006f8f80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f8f80 - the compiler's exception frame (FS:[0] chain) is not translated
/// `BGSDistantTreeBlock::ReadTypes` (Xbox PDB; cdecl, one word: the file). Inside the
/// memory-context guard (line `0x27e`): `TreeTypes` is cleared and freed ([`fn_006f9c70`] with
/// 1), then the file gives a count and for each type 0x20 bytes into a local `TreeType`
/// ([`fn_006f9060`]); its smart pointer (`+0x1c`, just overwritten by the read) is set to 0
/// (`0066b0d0`), the type is added ([`fn_006f86b0`]) and the local destroyed ([`fn_006f9080`]).
pub fn bgs_distant_tree_block_read_types(e: &mut Engine, file: u32) {
    e.with_stack(0x28, |e, frame| {
        let tree_type = frame.addr();
        let count = frame.addr() + 0x20;
        let guard = frame.addr() + 0x24;
        e.call(
            SCOPE_GUARD_ENTER_00404EB0,
            &args![guard, 0x25u32, 1u32, SOURCE_FILE_0106DD30, 0x27eu32],
        );
        fn_006f9c70(e, Ptr::new(STATIC_ARRAY_011D85D0), 1);
        e.mem.set_u32(count, 0);
        e.call(FILE_READ_00462D80, &args![file, count, 4u32]);
        let mut index = 0u32;
        while index < e.mem.u32(count) {
            fn_006f9060(e, Ptr::new(tree_type));
            e.call(FILE_READ_00462D80, &args![file, tree_type, 0x20u32]);
            e.call(POINTER_SET_0066B0D0, &args![tree_type + 0x1c, 0u32]);
            fn_006f86b0(e, tree_type);
            fn_006f9080(e, Ptr::new(tree_type));
            index = index.wrapping_add(1);
        }
        e.call(SCOPE_GUARD_LEAVE_00404EE0, &args![guard]);
    });
}

// Translated from 006f9060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `TreeType` constructor (thiscall): the smart pointer at `+0x1c` made empty
/// (`00633c90(0)`). Returns `this`.
pub fn fn_006f9060(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(POINTER_CONSTRUCT_00633C90, &args![this.addr() + 0x1c, 0u32]);
    this
}

// Translated from 006f90a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlock::SetupShaderProperty` (Xbox PDB; thiscall, two words: the shader property
/// and a `TreeGroup`; `RET 8`; `this` is not read). For the first `0042f5a0(count,
/// GetMaxInstances())` instances `00bb7290` (thiscall on the property, six words) gets the index,
/// the four floats at `+0`..`+0xc` and the float at `+0x10`, or 0.0 for a hidden instance
/// (`+0x1c`); the remaining slots up to `GetMaxInstances` get zeros. Then the group's
/// `bShaderPropertyUpToDate` (`+0x1c`) is set.
pub fn bgs_distant_tree_block_setup_shader_property(
    e: &mut Engine,
    _this: Ptr,
    property: u32,
    group: u32,
) {
    let instances = group.wrapping_add(8);
    let max = e.call(GET_MAX_INSTANCES_00BB6030, &args![]).u32();
    let count = e.call(ARRAY_COUNT_0044DDC0, &args![instances]).u32();
    let used = e.call(0x0042_f5a0, &args![count, max]).u32();
    let mut index = 0u32;
    while index < used {
        let entry = e.call(ARRAY_ENTRY_006B3120, &args![instances, index]).u32();
        let last = if e.mem.u8(entry.wrapping_add(0x1c)) != 0 {
            0
        } else {
            float_bits(e, entry.wrapping_add(0x10))
        };
        let words = [0u32, 4, 8, 0xc].map(|offset| float_bits(e, entry.wrapping_add(offset)));
        e.call(
            0x00bb_7290,
            &args![property, index, words[0], words[1], words[2], words[3], last],
        );
        index = index.wrapping_add(1);
    }
    let mut index = used;
    while index < e.call(GET_MAX_INSTANCES_00BB6030, &args![]).u32() {
        e.call(
            0x00bb_7290,
            &args![property, index, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        index = index.wrapping_add(1);
    }
    e.mem.set_u8(group.wrapping_add(0x1c), 1);
}

// Translated from 006f91a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlock::UpdateShaderProperties` (Xbox PDB; thiscall): for each tree group whose
/// shader property is out of date and that has a geometry, the geometry's property of the type
/// `00438220` gives (`00a59d30`, the map's `NiAVObject::GetProperty`) is set up
/// ([`bgs_distant_tree_block_setup_shader_property`]) when there is one.
pub fn bgs_distant_tree_block_update_shader_properties(e: &mut Engine, this: Ptr) {
    let groups = this.addr() + 4;
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT_0044DDC0, &args![groups]).u32() {
        let group_at = e.call(ARRAY_AT_00877A30, &args![groups, index]).u32();
        let group = e.mem.u32(group_at);
        if e.mem.u8(group.wrapping_add(0x1c)) == 0
            && e.call(POINTER_GET_00559450, &args![group.wrapping_add(4)])
                .u32()
                != 0
        {
            let kind = e.call(0x0043_8220, &args![]).u32();
            let geometry = e
                .call(POINTER_GET_00559450, &args![group.wrapping_add(4)])
                .u32();
            let property = e.call(0x00a5_9d30, &args![geometry, kind]).u32();
            if property != 0 {
                bgs_distant_tree_block_setup_shader_property(e, this, property, group);
            }
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 006f9300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlock::HideQueuedTrees` (Xbox PDB; thiscall): hides each id of
/// `TreesQueuedToHide` (`+0x34`) without queueing it again ([`fn_006f9230`] with 0), then
/// `006f7780(this)`.
pub fn bgs_distant_tree_block_hide_queued_trees(e: &mut Engine, this: Ptr) {
    let queue = this.addr() + 0x34;
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT_0044DDC0, &args![queue]).u32() {
        let at = e.call(ARRAY_AT_00877A30, &args![queue, index]).u32();
        let id = e.mem.u32(at);
        fn_006f9230(e, this, id, 0);
        index = index.wrapping_add(1);
    }
    e.call(0x006f_7780, &args![this]);
}

// Translated from 006f9640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantTreeBlockLoadTask::GetDescription` (the map's name; thiscall, two words: buffer
/// and size; `RET 8`): formats `"Tree LOD Task for block ( %i, %i ) at level ( %i )"`
/// (`0106df00`) with `iX`, `iY` and `iLevel` (`00406d00`). Returns true (`AL` = 1).
pub fn bgs_distant_tree_block_load_task_get_description(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
) -> bool {
    let base = this.addr();
    let x = e.mem.u32(base + 0x38);
    let y = e.mem.u32(base + 0x3c);
    let level = e.mem.u32(base + 0x40);
    e.call(
        FORMAT_00406D00,
        &args![buffer, size, 0x0106_df00u32, x, y, level],
    );
    true
}

/// The scalar deleting destructor shape (thiscall, one word: the flags; `RET 4`): the
/// destructor, then with flag bit 0 `operator delete`. Returns `this`.
fn scalar_deleting(e: &mut Engine, this: Ptr, flags: u32, destroy: fn(&mut Engine, Ptr)) -> Ptr {
    destroy(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006f96e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<unsigned int, BGSDistantTreeBlock::InstanceData*>`'s scalar deleting destructor (the
/// map's name): [`fn_006f9940`], see [`scalar_deleting`].
pub fn bs_map_instance_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006f9940)
}

// Translated from 006f9710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<unsigned int, BGSDistantTreeBlock::TreeGroup*>`'s scalar deleting destructor (the map's
/// name): [`fn_006f9b80`], see [`scalar_deleting`].
pub fn bs_map_tree_group_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, fn_006f9b80)
}

// Translated from 006f9740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSSimpleArray<InstanceData, 1024>` constructor (by its vtable `0106df78`;
/// thiscall): the vtable, then [`fn_006f9f60`] with 0 and 0. Returns `this`.
pub fn fn_006f9740(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_INSTANCE_ARRAY_0106DF78);
    fn_006f9f60(e, this, 0, 0);
    this
}

// Translated from 006f9770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSSimpleArray<InstanceData, 1024>` destructor (thiscall): the vtable `0106df78`,
/// then `008454f0(this, 1)`.
pub fn fn_006f9770(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_INSTANCE_ARRAY_0106DF78);
    e.call(ARRAY_FREE_008454F0, &args![this, 1u32]);
}

// Translated from 006f9800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: appends an `InstanceData` (thiscall on the array, one word: its address; `RET 4`):
/// a new slot (`006b4060`), constructed ([`fn_006f9e10`] with one element), gets the 0x20 bytes
/// (`REP MOVSD`). Returns the slot's index.
pub fn fn_006f9800(e: &mut Engine, this: Ptr, item: u32) -> u32 {
    let base = this.addr();
    let index = e.call(0x006b_4060, &args![base]).u32();
    let slot = (index << 5).wrapping_add(e.mem.u32(base + 4));
    fn_006f9e10(e, this, slot, 1);
    let slot = (index << 5).wrapping_add(e.mem.u32(base + 4));
    for word in 0..8u32 {
        let value = e.mem.u32(item.wrapping_add(word * 4));
        e.mem.set_u32(slot.wrapping_add(word * 4), value);
    }
    index
}

// Translated from 006f9850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an array's reallocation for 0x20-byte elements (thiscall, two words: the buffer and
/// the count; `RET 8`; `this` is not read): `0042f5d0(buffer, count * 0x20)` (cdecl), whose
/// answer is left in EAX up to the `RET` (`006f9862`..`006f986d`) and is the new buffer
/// `006f9eb0` stores (vtable slot 0xC).
pub fn fn_006f9850(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) -> u32 {
    e.call(0x0042_f5d0, &args![buffer, count << 5]).u32()
}

// Translated from 006f9c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: appends a `TreeType` (thiscall on the array, one word: its address; `RET 4`): a new
/// slot (`006b4060`), constructed ([`fn_006f9ff0`] with one element), assigned from it
/// ([`fn_006f9d90`]). Returns the slot's index.
pub fn fn_006f9c20(e: &mut Engine, this: Ptr, item: u32) -> u32 {
    let base = this.addr();
    let index = e.call(0x006b_4060, &args![base]).u32();
    let slot = (index << 5).wrapping_add(e.mem.u32(base + 4));
    fn_006f9ff0(e, this, slot, 1);
    let slot = (index << 5).wrapping_add(e.mem.u32(base + 4));
    fn_006f9d90(e, Ptr::new(slot), item);
    index
}

// Translated from 006f9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSDistantTreeBlock::InstanceData,1024>`'s scalar deleting destructor (the
/// map's name): [`fn_006f9770`], see [`scalar_deleting`].
pub fn bs_simple_array_instance_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006f9770)
}

// Translated from 006f9d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSDistantTreeBlock::TreeGroup*,1024>`'s scalar deleting destructor (the map's
/// name): [`fn_006f98a0`], see [`scalar_deleting`].
pub fn bs_simple_array_tree_group_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006f98a0)
}

// Translated from 006f9d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<unsigned int, BGSDistantTreeBlock::InstanceData*>`'s scalar deleting destructor
/// (the map's name): [`fn_006f99a0`], see [`scalar_deleting`].
pub fn bs_map_base_instance_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006f99a0)
}

// Translated from 006f9d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<unsigned int, BGSDistantTreeBlock::TreeGroup*>`'s scalar deleting destructor (the
/// map's name): [`fn_006f9be0`], see [`scalar_deleting`].
pub fn bs_map_base_tree_group_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, fn_006f9be0)
}

// Translated from 006f9d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `TreeType` assignment (thiscall, one word: the source; `RET 4`): the word at `+0`,
/// the six floats `+4..+0x18` (`FLD`/`FSTP`) and the smart pointer at `+0x1c` (`006e5cc0`).
/// Returns `this`.
pub fn fn_006f9d90(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let base = this.addr();
    let first = e.mem.u32(source);
    e.mem.set_u32(base, first);
    for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x18] {
        let value = float_bits(e, source.wrapping_add(offset));
        e.mem.set_u32(base + offset, value);
    }
    e.call(
        POINTER_ASSIGN_006E5CC0,
        &args![base + 0x1c, source.wrapping_add(0x1c)],
    );
    this
}

/// The placement-construct loop shape of [`fn_006f9e10`] and [`fn_006f9ff0`]: for each of `count`
/// 0x20-byte elements from `buffer`, placement `new` (`006e6da0(0x20, element)`, cdecl) and, with
/// a result, the constructor.
fn construct_elements(e: &mut Engine, buffer: u32, count: u32, construct: fn(&mut Engine, u32)) {
    let mut index = 0u32;
    while index < count {
        let element = (index << 5).wrapping_add(buffer);
        let place = e.call(0x006e_6da0, &args![0x20u32, element]).u32();
        if place != 0 {
            construct(e, place);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 006f9e10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f9e10 - the compiler's exception frame (FS:[0] chain) is not translated
/// Unnamed: constructs `InstanceData` elements in place (thiscall, two words: buffer and count;
/// `RET 8`; `this` is not read): see [`construct_elements`], the constructor `006240d0`.
pub fn fn_006f9e10(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) {
    construct_elements(e, buffer, count, |e, place| {
        e.call(0x0062_40d0, &args![place]);
    });
}

// Translated from 006f9f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSSimpleArray<InstanceData>` constructor body (thiscall, two words: capacity and
/// size; `RET 8`): clears buffer, size and capacity; the capacity is raised to the size when below
/// it; with a capacity a buffer comes from slot 4 and the capacity is kept; with a size its
/// entries are constructed ([`fn_006f9e10`]) and the size kept.
pub fn fn_006f9f60(e: &mut Engine, this: Ptr, capacity: u32, size: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let capacity = if size > capacity { size } else { capacity };
    if capacity > 0 {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    }
    if size > 0 {
        let buffer = e.mem.u32(base + 4);
        fn_006f9e10(e, this, buffer, size);
        e.mem.set_u32(base + 8, size);
    }
}

// Translated from 006f9ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f9ff0 - the compiler's exception frame (FS:[0] chain) is not translated
/// Unnamed: constructs `TreeType` elements in place (thiscall, two words: buffer and count;
/// `RET 8`; `this` is not read): see [`construct_elements`], the constructor [`fn_006f9060`].
pub fn fn_006f9ff0(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) {
    construct_elements(e, buffer, count, |e, place| {
        fn_006f9060(e, Ptr::new(place));
    });
}

// Translated from 006fa090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: destroys `TreeType` elements (thiscall, two words: buffer and count; `RET 8`; `this`
/// is not read): [`fn_006fa0d0`] with 0 on each 0x20-byte element.
pub fn fn_006fa090(e: &mut Engine, _this: Ptr, buffer: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        fn_006fa0d0(e, Ptr::new((index << 5).wrapping_add(buffer)), 0);
        index = index.wrapping_add(1);
    }
}

// Translated from 006fa0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `TreeType` scalar deleting destructor (thiscall, one word: the flags; `RET 4`):
/// [`fn_006f9080`], see [`scalar_deleting`].
pub fn fn_006fa0d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, fn_006f9080)
}

// Translated from 006fa130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSDistantTreeBlock::TreeType,1024>`'s scalar deleting destructor (the map's
/// name): its destructor `006fa160` (thiscall), then with flag bit 0 `operator delete`. Returns
/// `this`.
pub fn bs_simple_array_tree_type_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, |e, this| {
        e.call(0x006f_a160, &args![this]);
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_94a0, fn_006f94a0(Ptr)),
        entry!(0x006f99e0, allocate_item(Ptr) -> Ptr),
        entry!(0x006f_9080, fn_006f9080(Ptr)),
        entry!(0x006f_a100, fn_006fa100(Ptr) -> Ptr),
        entry!(
            0x006f_77a0,
            bgs_distant_tree_block_allocate_static_data(u32)
        ),
        entry!(0x006f_7a60, fn_006f7a60(u32)),
        entry!(0x006f_83f0, fn_006f83f0()),
        entry!(0x006f_86e0, fn_006f86e0(Ptr)),
        entry!(0x006f_8890, fn_006f8890(u32, u32, u32, u32, u32)),
        entry!(0x006f_9230, fn_006f9230(Ptr, u32, u32)),
        entry!(0x006f_92a0, fn_006f92a0(Ptr, u32)),
        entry!(0x006f_9360, fn_006f9360(Ptr, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(
            0x006f_9470,
            bgs_distant_tree_block_load_task_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006f_94c0, bgs_distant_tree_block_load_task_queue_me(Ptr)),
        entry!(
            0x006f_94e0,
            bgs_distant_tree_block_load_task_check_finished(Ptr)
        ),
        entry!(
            0x006f_9540,
            bgs_distant_tree_block_load_task_cancel(Ptr, u32, u32)
        ),
        entry!(0x006f_9570, bgs_distant_tree_block_load_task_run(Ptr)),
        entry!(0x006f_9610, bgs_distant_tree_block_load_task_finish(Ptr)),
        entry!(0x006f_9680, fn_006f9680(Ptr, u32) -> Ptr),
        entry!(0x006f_96b0, fn_006f96b0(Ptr, u32) -> Ptr),
        entry!(0x006f_9790, fn_006f9790(Ptr, u32)),
        entry!(0x006f_9870, fn_006f9870(Ptr) -> Ptr),
        entry!(0x006f_98a0, fn_006f98a0(Ptr)),
        entry!(0x006f_98c0, fn_006f98c0(Ptr, u32) -> Ptr),
        entry!(0x006f_9940, fn_006f9940(Ptr)),
        entry!(0x006f_99a0, fn_006f99a0(Ptr)),
        entry!(0x006f_9a00, fn_006f9a00(Ptr, u32) -> Ptr),
        entry!(0x006f_9a80, bs_map_base_set_at(Ptr, u32, u32)),
        entry!(0x006f_9b80, fn_006f9b80(Ptr)),
        entry!(0x006f_9be0, fn_006f9be0(Ptr)),
        entry!(0x006f_9c70, fn_006f9c70(Ptr, u32)),
        entry!(0x006f_9eb0, fn_006f9eb0(Ptr, u32, u32)),
        entry!(0x006f_a180, fn_006fa180(Ptr, u32, u32)),
        entry!(
            0x006f_7a80,
            bgs_distant_tree_block_allocate_static_data_for_tree_type(u32)
        ),
        entry!(0x006f_8470, bgs_distant_tree_block_add_tree(Ptr, u32, u32)),
        entry!(0x006f_8650, fn_006f8650(Ptr) -> Ptr),
        entry!(0x006f_86b0, fn_006f86b0(u32)),
        entry!(0x006f_8800, fn_006f8800(Ptr, u32) -> Ptr),
        entry!(0x006f_8830, fn_006f8830(Ptr)),
        entry!(0x006f_88e0, fn_006f88e0(Ptr)),
        entry!(0x006f_8910, fn_006f8910(Ptr, u32, u32) -> u32),
        entry!(0x006f_8b40, bgs_distant_tree_block_read(Ptr, u32)),
        entry!(0x006f_8c40, fn_006f8c40(Ptr)),
        entry!(0x006f_8f80, bgs_distant_tree_block_read_types(u32)),
        entry!(0x006f_9060, fn_006f9060(Ptr) -> Ptr),
        entry!(
            0x006f_90a0,
            bgs_distant_tree_block_setup_shader_property(Ptr, u32, u32)
        ),
        entry!(
            0x006f_91a0,
            bgs_distant_tree_block_update_shader_properties(Ptr)
        ),
        entry!(0x006f_9300, bgs_distant_tree_block_hide_queued_trees(Ptr)),
        entry!(
            0x006f_9640,
            bgs_distant_tree_block_load_task_get_description(Ptr, u32, u32) -> bool
        ),
        entry!(
            0x006f_96e0,
            bs_map_instance_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_9710,
            bs_map_tree_group_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006f_9740, fn_006f9740(Ptr) -> Ptr),
        entry!(0x006f_9770, fn_006f9770(Ptr)),
        entry!(0x006f_9800, fn_006f9800(Ptr, u32) -> u32),
        entry!(0x006f_9850, fn_006f9850(Ptr, u32, u32) -> u32),
        entry!(0x006f_9c20, fn_006f9c20(Ptr, u32) -> u32),
        entry!(
            0x006f_9cd0,
            bs_simple_array_instance_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_9d00,
            bs_simple_array_tree_group_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_9d30,
            bs_map_base_instance_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x006f_9d60,
            bs_map_base_tree_group_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006f_9d90, fn_006f9d90(Ptr, u32) -> Ptr),
        entry!(0x006f_9e10, fn_006f9e10(Ptr, u32, u32)),
        entry!(0x006f_9f60, fn_006f9f60(Ptr, u32, u32)),
        entry!(0x006f_9ff0, fn_006f9ff0(Ptr, u32, u32)),
        entry!(0x006f_a090, fn_006fa090(Ptr, u32, u32)),
        entry!(0x006f_a0d0, fn_006fa0d0(Ptr, u32) -> Ptr),
        entry!(
            0x006f_a130,
            bs_simple_array_tree_type_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006f9080_passes_this_plus_1c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0045CEC0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_9080, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0045CEC0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x1c]);
    }

    #[test]
    fn fn_006f94a0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00c3_cea0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006f_94a0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00c3_cea0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_ded0);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_ded0,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_the_vtable_before_calling_the_body() {
        let mut e = Engine::new();
        e.register(CONSTRUCTOR_BODY_006FA180, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 8, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_a100, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_006FA180),
            vec![vec![this, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_006FA100);
    }
}

#[cfg(test)]
mod b046_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, quiet);
        }
    }

    /// An object whose vtable has `slots` entries, the given ones pointing at the given doubles.
    fn object_with(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let count = slots
            .iter()
            .map(|(slot, _)| slot / 4 + 1)
            .max()
            .unwrap_or(1);
        let mut table = vec![0u32; count as usize];
        for (slot, target) in slots {
            table[(slot / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &table);
        let object = e.mem.alloc(0x50);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn b046_006f77a0_loads_the_static_data() {
        let mut e = Engine::new();
        for global in [
            SECURITY_COOKIE_011C16BC,
            0x011d_ea10,
            STATIC_LOADED_011D85A0,
            STATIC_POINTER_011D85BC,
            STATIC_POINTER_011D85C4,
            STATIC_TEXTURE_011D85E0,
            STATIC_POINTER_012007C4,
        ] {
            e.map(global, 4);
        }
        e.set_global(SECURITY_COOKIE_011C16BC, 0x00c0_0c1eu32);
        e.set_global(0x011d_ea10, 0x0ade_0000u32);
        stub(
            &mut e,
            &[
                FORMAT_00406D00,
                0x006f_8f80,
                0x0045_68c0,
                0x005b_5e40,
                POINTER_SET_0066B0D0,
                0x0049_ee30,
                0x0043_93c0,
                0x0049_ed90,
                0x0094_db80,
                0x0043_93e0,
                SECURITY_CHECK_COOKIE,
                0x0330_1000,
                0x0330_1130,
            ],
        );
        let world = object_with(&mut e, 0x0330_0000, &[(0x130, 0x0330_1130)]);
        e.register(0x0330_1130, |_, _| 0x0aa0_0000u32.into_ret());
        let file = object_with(&mut e, 0x0331_0000, &[(0, 0x0330_1000)]);
        e.register_double(0x00af_df00, move |_, _| file.into_ret());
        e.register(0x00ec_8a19, |_, _| 0u32.into_ret());
        e.register(POINTER_GET_00559450, |_, a| (a[0] + 0x100).into_ret());
        e.register(0x00aa_13e0, |_, a| (a[0] << 8).into_ret());
        e.register(0x0049_ec80, |_, a| (a[0] + 1).into_ret());
        e.register(0x0043_91c0, |_, a| (a[0] + 2).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_77a0, &args![world]);
        let formats = calls_to(&e, FORMAT_00406D00);
        assert_eq!(formats.len(), 2);
        assert_eq!(formats[0][1..], [0x104, 0x0106_de08, 0x0aa0_0000]);
        assert_eq!(formats[1][1..], [0x104, 0x0106_ddb8, 0x0106_dddc]);
        assert_eq!(
            calls_to(&e, 0x00af_df00),
            vec![vec![formats[0][0], 0, 0x800]]
        );
        assert_eq!(calls_to(&e, 0x006f_8f80), vec![vec![file]]);
        assert_eq!(calls_to(&e, 0x0330_1000), vec![vec![file, 1]]);
        assert_eq!(
            calls_to(&e, 0x00ec_8a19),
            vec![vec![0x0aa0_0000, 0x0106_ddf0, 4]]
        );
        assert_eq!(
            calls_to(&e, 0x0045_68c0),
            vec![vec![
                0x0ade_0000,
                formats[1][0],
                STATIC_TEXTURE_011D85E0,
                1,
                1
            ]]
        );
        assert!(calls_to(&e, 0x005b_5e40).is_empty());
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![
                vec![STATIC_POINTER_012007C4, STATIC_TEXTURE_011D85E0 + 0x100],
                vec![STATIC_POINTER_011D85BC, 0x2401],
                vec![STATIC_POINTER_011D85C4, 0x1c02]
            ]
        );
        assert_eq!(e.mem.u8(STATIC_LOADED_011D85A0), 1);
        let held = STATIC_POINTER_011D85C4 + 0x100;
        assert_eq!(
            calls_to(&e, 0x0049_ee30),
            vec![vec![STATIC_POINTER_011D85BC + 0x100, 3]]
        );
        assert_eq!(calls_to(&e, 0x0043_93c0), vec![vec![held, 1]]);
        assert_eq!(calls_to(&e, 0x0049_ed90), vec![vec![held, 0]]);
        assert_eq!(calls_to(&e, 0x0094_db80), vec![vec![held, 0x80]]);
        assert_eq!(calls_to(&e, 0x0043_93e0), vec![vec![held, 4]]);
        assert_eq!(calls_to(&e, SECURITY_CHECK_COOKIE), vec![vec![0x00c0_0c1e]]);
        // No file, another worldspace, no texture, failed allocations.
        e.register(0x00af_df00, |_, _| 0u32.into_ret());
        e.register(0x00ec_8a19, |_, _| 1u32.into_ret());
        e.register(POINTER_GET_00559450, |_, _| 0u32.into_ret());
        e.register(0x00aa_13e0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_77a0, &args![world]);
        assert!(calls_to(&e, 0x006f_8f80).is_empty());
        assert_eq!(calls_to(&e, FORMAT_00406D00)[1][3], 0x0106_ddf8);
        assert_eq!(calls_to(&e, 0x005b_5e40), vec![vec![0x0106_dd94]]);
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0)[1..],
            [
                vec![STATIC_POINTER_011D85BC, 0],
                vec![STATIC_POINTER_011D85C4, 0]
            ]
        );
    }

    #[test]
    fn b046_006f83f0_and_006f7a60_release_the_static_data() {
        let mut e = Engine::new();
        e.map(STATIC_LOADED_011D85A0, 4);
        e.mem.set_u8(STATIC_LOADED_011D85A0, 1);
        stub(&mut e, &[POINTER_SET_0066B0D0]);
        e.register(ARRAY_COUNT_0044DDC0, |_, _| 2u32.into_ret());
        e.register(0x006b_3120, |_, a| (0x1000 * (a[1] + 1)).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_83f0, &args![]);
        assert_eq!(
            calls_to(&e, 0x006b_3120),
            vec![
                vec![STATIC_ARRAY_011D85D0, 0],
                vec![STATIC_ARRAY_011D85D0, 1]
            ]
        );
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![
                vec![0x101c, 0],
                vec![0x201c, 0],
                vec![STATIC_POINTER_011D85BC, 0],
                vec![STATIC_POINTER_011D85C4, 0],
                vec![STATIC_TEXTURE_011D85E0, 0],
                vec![STATIC_POINTER_012007C4, 0]
            ]
        );
        assert_eq!(e.mem.u8(STATIC_LOADED_011D85A0), 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_7a60, &args![0x77u32]);
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![vec![STATIC_POINTER_012007C4, 0x77]]
        );
    }

    #[test]
    fn b046_006f86e0_detaches_and_deletes_the_groups() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                ARRAY_FREE_008454F0,
                POINTER_SET_0066B0D0,
                0x006f_8800,
                0x0332_10e8,
            ],
        );
        let groups = [e.mem.alloc(0x20), 0, e.mem.alloc(0x20)];
        let entries = e.mem.alloc(0xc);
        for (i, group) in groups.iter().enumerate() {
            e.mem.set_u32(entries + 4 * i as u32, *group);
        }
        e.register(ARRAY_COUNT_0044DDC0, |_, _| 3u32.into_ret());
        e.register_double(ARRAY_AT_00877A30, move |_, a| {
            (entries + 4 * a[1]).into_ret()
        });
        // Only the first group has geometry.
        let geometry_of = groups[0] + 4;
        e.register_double(POINTER_GET_00559450, move |_, a| {
            match a[0] {
                at if at == geometry_of => 0x6600u32,
                0x7700 => 0x7710,
                _ => 0,
            }
            .into_ret()
        });
        let parent = object_with(&mut e, 0x0332_0000, &[(0xe8, 0x0332_10e8)]);
        e.register(0x0080_41a0, |_, _| 0x0000_7700u32.into_ret());
        e.register_double(0x004e_a9e0, move |_, _| parent.into_ret());
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        e.call(0x006f_86e0, &args![this]);
        assert_eq!(
            calls_to(&e, ARRAY_FREE_008454F0),
            vec![
                vec![groups[0] + 8, 1],
                vec![8, 1],
                vec![groups[2] + 8, 1],
                vec![this + 4, 1]
            ]
        );
        assert_eq!(calls_to(&e, 0x0080_41a0), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x004e_a9e0), vec![vec![0x7710]]);
        assert_eq!(calls_to(&e, 0x0332_10e8), vec![vec![parent, 0x6600]]);
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![vec![groups[0] + 4, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x006f_8800),
            vec![vec![groups[0], 1], vec![groups[2], 1]]
        );
    }

    #[test]
    fn b046_006f8890_formats_the_block_path() {
        let mut e = Engine::new();
        e.register(FORMAT_00406D00, quiet);
        let world = object_with(&mut e, 0x0333_0000, &[(0x130, 0x0333_1130)]);
        e.register(0x0333_1130, |_, _| 0x0bb0_0000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x006f_8890,
            &args![0x9000u32, world, 4u32, 0x1_fffeu32, 0x0003u32],
        );
        assert_eq!(
            calls_to(&e, FORMAT_00406D00),
            vec![vec![
                0x9000,
                0x104,
                0x0106_de90,
                0x0bb0_0000,
                0x0bb0_0000,
                4,
                0xffff_fffe,
                3
            ]]
        );
    }

    fn block_world(e: &mut Engine, found: bool) -> (u32, u32, u32) {
        let instance = e.mem.alloc(0x20);
        e.mem.set_u32(instance + 0x18, 2);
        e.mem.set_u8(instance + 0x1c, 0xcc);
        let group = e.mem.alloc(0x20);
        e.mem.set_u8(group + 0x1c, 1);
        let entries = e.mem.alloc(0x10);
        e.mem.set_u32(entries + 8, group);
        e.register_double(0x0085_3130, move |e, a| {
            if found {
                e.mem.set_u32(a[2], instance);
            }
            (found as u32 | 0x100).into_ret()
        });
        e.register_double(ARRAY_AT_00877A30, move |_, a| {
            (entries + 4 * a[1]).into_ret()
        });
        e.register(0x007c_b2e0, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x55);
            Ret::default()
        });
        (e.mem.alloc(0x4c), instance, group)
    }

    #[test]
    fn b046_006f9230_and_006f92a0_hide_show_or_queue() {
        let mut e = Engine::new();
        let (this, instance, group) = block_world(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x006f_9230, &args![this, 0x55u32, 1u32]);
        let lookup = calls_to(&e, 0x0085_3130);
        assert_eq!(lookup[0][..2], [this + 0x14, 0x55]);
        assert_eq!(e.mem.u8(instance + 0x1c), 1);
        assert_eq!(e.mem.u8(group + 0x1c), 0);
        assert_eq!(calls_to(&e, ARRAY_AT_00877A30), vec![vec![this + 4, 2]]);
        assert!(calls_to(&e, 0x007c_b2e0).is_empty());
        e.mem.set_u8(group + 0x1c, 1);
        e.call(0x006f_92a0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u8(instance + 0x1c), 0);
        assert_eq!(e.mem.u8(group + 0x1c), 0);
        let mut e = Engine::new();
        let (this, instance, _) = block_world(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x006f_9230, &args![this, 0x55u32, 0x100u32]);
        assert!(calls_to(&e, 0x007c_b2e0).is_empty());
        e.call(0x006f_9230, &args![this, 0x55u32, 1u32]);
        assert_eq!(calls_to(&e, 0x007c_b2e0).len(), 1);
        assert_eq!(calls_to(&e, 0x007c_b2e0)[0][0], this + 0x34);
        e.call(0x006f_92a0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u8(instance + 0x1c), 0xcc);
    }

    #[test]
    fn b046_006f9360_constructs_the_load_task() {
        let mut e = Engine::new();
        e.map(SECURITY_COOKIE_011C16BC, 4);
        e.set_global(SECURITY_COOKIE_011C16BC, 0x00c0_0c1eu32);
        stub(
            &mut e,
            &[
                0x00c3_ce60,
                0x00c3_cee0,
                0x00c3_cf60,
                SECURITY_CHECK_COOKIE,
                FORMAT_00406D00,
            ],
        );
        let world = object_with(&mut e, 0x0334_0000, &[(0x130, 0x0334_1130)]);
        e.register(0x0334_1130, |_, _| 0x0cc0_0000u32.into_ret());
        let this = e.mem.alloc(0x48);
        e.mem.write(this, &[0xcc; 0x48]);
        e.call_log = Some(vec![]);
        let words = args![
            this,
            0xb10cu32,
            world,
            0x1_0005u32,
            0x2_0006u32,
            3u32,
            0x99u32
        ];
        assert_eq!(e.call(0x006f_9360, &words).u32(), this);
        assert_eq!(calls_to(&e, 0x00c3_ce60), vec![vec![this, 3]]);
        assert_eq!(e.mem.u32(this), VTABLE_LOAD_TASK_0106DED0);
        assert_eq!(e.mem.u32(this + 0x30), 0xb10c);
        assert_eq!(e.mem.u8(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0x1_0005);
        assert_eq!(e.mem.u32(this + 0x3c), 0x2_0006);
        assert_eq!(e.mem.u32(this + 0x40), 3);
        assert_eq!(e.mem.u8(this + 0x44), 0);
        assert_eq!(e.mem.u8(this + 0x45), 0);
        let format = calls_to(&e, FORMAT_00406D00);
        assert_eq!(format[0][5..], [3, 5, 6]);
        assert_eq!(
            calls_to(&e, 0x00c3_cee0),
            vec![vec![this, format[0][0] + 5]]
        );
        assert_eq!(calls_to(&e, 0x00c3_cf60), vec![vec![this, 0]]);
        assert_eq!(calls_to(&e, SECURITY_CHECK_COOKIE), vec![vec![0x00c0_0c1e]]);
    }

    #[test]
    fn b046_load_task_virtuals() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                0x00c3_cea0,
                OPERATOR_DELETE,
                0x0335_1028,
                0x00c3_c930,
                0x00c3_cb70,
                0x0336_1048,
                0x006f_8b40,
                0x0337_1000,
                0x006f_9300,
                0x006f_88e0,
            ],
        );
        let this = object_with(&mut e, 0x0335_0000, &[(0x28, 0x0335_1028)]);
        // 006f9470: the base destructor (`006f94a0`) and the delete.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9470, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x00c3_cea0), vec![vec![this]]);
        e.call(0x006f_9470, &args![this, 0u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 1);
        // 006f94c0: QueueMe.
        let this = object_with(&mut e, 0x0335_0000, &[(0x28, 0x0335_1028)]);
        e.call_log = Some(vec![]);
        e.call(0x006f_94c0, &args![this]);
        assert_eq!(e.mem.u8(this + 0x34), 1);
        assert_eq!(calls_to(&e, 0x0335_1028), vec![vec![this]]);
        // 006f94e0: CheckFinished, by `0043caa0`, the flag and `0069b080`.
        e.map(0x0120_2d98, 4);
        let queue = object_with(&mut e, 0x0336_0000, &[(0x48, 0x0336_1048)]);
        e.set_global(0x0120_2d98, queue);
        e.register(0x0043_caa0, |_, _| 1u32.into_ret());
        e.register(0x0069_b080, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_94e0, &args![this]);
        assert_eq!(calls_to(&e, 0x0336_1048), vec![vec![queue, this]]);
        e.register(0x0069_b080, |_, _| 0x100u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_94e0, &args![this]);
        assert_eq!(calls_to(&e, 0x00c3_c930), vec![vec![this]]);
        e.mem.set_u8(this + 0x34, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_94e0, &args![this]);
        assert!(calls_to(&e, 0x00c3_c930).is_empty());
        // 006f9540: Cancel.
        e.call_log = Some(vec![]);
        e.call(0x006f_9540, &args![this, 7u32, 8u32]);
        assert_eq!(e.mem.u8(this + 0x44), 1);
        assert_eq!(calls_to(&e, 0x00c3_cb70), vec![vec![this, 7, 8]]);
        // 006f9570: Run, canceled then not; 006f9610: Finish.
        let file = object_with(&mut e, 0x0337_0000, &[(0, 0x0337_1000)]);
        e.register(0x0045_cd60, |_, _| 0x0dd0_0000u32.into_ret());
        e.register_double(0x00af_df20, move |_, _| file.into_ret());
        e.mem.set_u32(this + 0x30, 0xb10c);
        e.call_log = Some(vec![]);
        e.call(0x006f_9570, &args![this]);
        assert_eq!(
            calls_to(&e, 0x00af_df20),
            vec![vec![0x0dd0_0000, 0, 0x800, 1]]
        );
        assert!(calls_to(&e, 0x006f_8b40).is_empty());
        assert_eq!(e.mem.u8(this + 0x45), 0);
        e.call(0x006f_9610, &args![this]);
        assert!(calls_to(&e, 0x006f_88e0).is_empty());
        e.mem.set_u8(this + 0x44, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_9570, &args![this]);
        assert_eq!(calls_to(&e, 0x006f_8b40), vec![vec![0xb10c, file]]);
        assert_eq!(calls_to(&e, 0x0337_1000), vec![vec![file, 1]]);
        assert_eq!(calls_to(&e, 0x006f_9300), vec![vec![0xb10c]]);
        assert_eq!(e.mem.u8(this + 0x45), 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_9610, &args![this]);
        assert_eq!(calls_to(&e, 0x006f_88e0), vec![vec![0xb10c]]);
        e.register(0x00af_df20, |_, _| 0u32.into_ret());
        e.call(0x006f_9570, &args![this]);
        assert_eq!(e.mem.u8(this + 0x45), 0);
    }

    #[test]
    fn b046_map_constructors_and_destructors() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (0x4000 + a[0]).into_ret());
        stub(&mut e, &[0x00ec_61c0, 0x0043_8af0, OPERATOR_DELETE]);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcc; 0x10]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9680, &args![this, 0x25u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_INSTANCE_MAP_0106DF38);
        assert_eq!(e.mem.u32(this + 4), 0x25);
        assert_eq!(e.mem.u32(this + 8), 0x4094);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls_to(&e, 0x00ec_61c0), vec![vec![0x4094, 0, 0x94]]);
        e.mem.set_u32(this + 8, 0xcccc_cccc);
        assert_eq!(e.call(0x006f_96b0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_MAP_0106DF58);
        assert_eq!(e.mem.u32(this + 8), 0xcccc_cccc);
        assert_eq!(e.call(0x006f_98c0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_INSTANCE_MAP_BASE_0106DFA0);
        assert_eq!(e.call(0x006f_9a00, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_MAP_BASE_0106DFC0);
        e.call_log = Some(vec![]);
        e.call(0x006f_9940, &args![this]);
        assert_eq!(calls_to(&e, 0x0043_8af0), vec![vec![this]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x4004]]);
        assert_eq!(e.mem.u32(this), VTABLE_INSTANCE_MAP_BASE_0106DFA0);
        e.call_log = Some(vec![]);
        e.call(0x006f_9b80, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_MAP_BASE_0106DFC0);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 1);
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_99a0, &args![this]);
        e.call(0x006f_9be0, &args![this]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn b046_006f9a80_sets_or_adds_the_item() {
        let mut e = Engine::new();
        let this = object_with(
            &mut e,
            0x0338_0000,
            &[
                (4, 0x0338_1004),
                (8, 0x0338_1008),
                (0xc, 0x0338_100c),
                (0x10, 0x0338_1010),
                (0x14, 0x0338_1014),
            ],
        );
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, table);
        e.register(0x0338_1004, |_, a| (a[1] & 3).into_ret());
        e.register(0x0338_1008, |_, a| {
            ((a[1] == a[2]) as u32 | 0x100).into_ret()
        });
        e.register(0x0338_100c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(0x0338_1010, quiet);
        e.register(0x0338_1014, |e, _| e.mem.alloc(0xc).into_ret());
        e.call(0x006f_9a80, &args![this, 5u32, 0x50u32]);
        e.call(0x006f_9a80, &args![this, 9u32, 0x90u32]);
        let first = e.mem.u32(table + 4);
        assert_eq!(e.mem.u32(first + 4), 9);
        assert_eq!(e.mem.u32(e.mem.u32(first) + 4), 5);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        e.call_log = Some(vec![]);
        e.call(0x006f_9a80, &args![this, 5u32, 0x55u32]);
        let second = e.mem.u32(first);
        assert_eq!(calls_to(&e, 0x0338_1010), vec![vec![this, second]]);
        assert_eq!(e.mem.u32(second + 8), 0x55);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert!(calls_to(&e, 0x0338_1014).is_empty());
    }

    #[test]
    fn b046_array_resize_clear_and_construct() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                0x0072_ba80,
                0x006d_c640,
                0x006a_8500,
                0x006f_a090,
                0x006f_9ff0,
            ],
        );
        let this = object_with(&mut e, 0x0339_0000, &[(4, 0x0339_1004), (0xc, 0x0339_100c)]);
        e.register(0x0339_1004, |_, a| (0x7000 + a[1]).into_ret());
        e.register(0x0339_100c, |_, a| (a[1] + 0x10).into_ret());
        // 006fa180: capacity 2 raised to the size 3.
        e.call_log = Some(vec![]);
        e.call(0x006f_a180, &args![this, 2u32, 3u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7003);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(calls_to(&e, 0x006f_9ff0), vec![vec![this, 0x7003, 3]]);
        e.call(0x006f_a180, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // 006f9eb0: no buffer; same count as capacity (reallocate); otherwise copy.
        e.call(0x006f_9eb0, &args![this, 4u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7004);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        e.call(0x006f_9eb0, &args![this, 6u32, 4u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7014);
        e.call_log = Some(vec![]);
        e.call(0x006f_9eb0, &args![this, 8u32, 2u32]);
        assert_eq!(
            calls_to(&e, 0x006d_c640),
            vec![vec![this, 0x7008, 0x7014, 2]]
        );
        assert_eq!(calls_to(&e, 0x006a_8500), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), 0x7008);
        // 006f9790: shrink below the size, then the same capacity does nothing.
        e.mem.set_u32(this + 8, 5);
        e.mem.set_u32(this + 0xc, 4);
        e.call_log = Some(vec![]);
        e.call(0x006f_9790, &args![this, 2u32]);
        assert_eq!(
            calls_to(&e, 0x0072_ba80),
            vec![vec![this, 0x7008 + 0x40, 3]]
        );
        assert_eq!(e.mem.u32(this + 8), 2);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        e.call_log = Some(vec![]);
        e.call(0x006f_9790, &args![this, 2u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // 006f9c70: clear without and with freeing.
        e.call_log = Some(vec![]);
        e.call(0x006f_9c70, &args![this, 0x100u32]);
        assert_eq!(calls_to(&e, 0x006f_a090), vec![vec![this, 0x7002, 2]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 4), 0x7002);
        e.call_log = Some(vec![]);
        e.call(0x006f_9c70, &args![this, 1u32]);
        assert_eq!(calls_to(&e, 0x006a_8500), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_9c70, &args![this, 1u32]);
        assert!(calls_to(&e, 0x006f_a090).is_empty());
    }

    #[test]
    fn b046_006f9870_and_006f98a0_group_array() {
        let mut e = Engine::new();
        stub(&mut e, &[0x006b_3eb0, ARRAY_FREE_008454F0]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9870, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_ARRAY_0106DF8C);
        assert_eq!(calls_to(&e, 0x006b_3eb0), vec![vec![this, 0, 0]]);
        e.mem.set_u32(this, 0);
        e.call(0x006f_98a0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_ARRAY_0106DF8C);
        assert_eq!(calls_to(&e, ARRAY_FREE_008454F0), vec![vec![this, 1]]);
    }
}

#[cfg(test)]
mod n089_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, quiet);
        }
    }

    /// Generic array doubles: count (`+8`), entry of a 0x20-byte array, entry of a pointer
    /// array, a new slot, and a pointer append (`+4` buffer, `+8` size).
    fn array_doubles(e: &mut Engine) {
        e.register(ARRAY_COUNT_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ARRAY_ENTRY_006B3120, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x20).into_ret()
        });
        e.register(ARRAY_AT_00877A30, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 4).into_ret()
        });
        e.register(0x006b_4060, |e, a| {
            if e.mem.u32(a[0] + 4) == 0 {
                let buffer = e.mem.alloc(0x100);
                e.mem.set_u32(a[0] + 4, buffer);
            }
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        e.register(0x007c_b2e0, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let value = e.mem.u32(a[1]);
            let at = e.mem.u32(a[0] + 4) + size * 4;
            e.mem.set_u32(at, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        e.register(0x006e_6da0, |_, a| a[1].into_ret());
        // The last entry of a 0x20-byte array.
        e.register(0x006b_3140, |e, a| {
            (e.mem.u32(a[0] + 4) + (e.mem.u32(a[0] + 8) - 1) * 0x20).into_ret()
        });
    }

    /// An array (`+4` buffer of `bytes`, `+8` size) at `at`.
    fn array_at(e: &mut Engine, at: u32, bytes: u32, size: u32) -> u32 {
        let buffer = e.mem.alloc(bytes);
        e.mem.set_u32(at + 4, buffer);
        e.mem.set_u32(at + 8, size);
        buffer
    }

    /// Every callee of the static data builder, quiet; the allocators give real memory.
    fn static_data_doubles(e: &mut Engine, max: u32) {
        stub(
            e,
            &[
                SCOPE_GUARD_ENTER_00404EB0,
                SCOPE_GUARD_LEAVE_00404EE0,
                0x0040_1050,
                0x0045_79e0,
                0x004a_0c10,
                0x00a7_2ca0,
                0x00a7_30e0,
                0x00a7_2bb0,
                0x00a7_26a0,
                0x00a6_7260,
                POINTER_ASSIGN_006E5CC0,
                POINTER_SET_0066B0D0,
                POINTER_DESTROY_0045CEC0,
                POINTER_CONSTRUCT_00633C90,
                OPERATOR_DELETE,
            ],
        );
        e.register_double(GET_MAX_INSTANCES_00BB6030, move |_, _| max.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(NI_ALLOCATE_00AA13E0, |_, a| (a[0] << 12).into_ret());
        e.register(0x0053_7bf0, |_, a| (a[0] + 1).into_ret());
        e.register(0x00a7_b630, |_, a| (a[0] + 2).into_ret());
        e.register(POINTER_GET_00559450, |_, a| (a[0] + 0x100).into_ret());
        for (address, value) in [
            (HALF_01011588, 0x3fe0_0000_0000_0000u64),
            (NORMAL_X_01012054, 0xbf80_0000),
            (TWO_010162C0, 0x4000_0000),
            (FLOAT_MAX_01016970, 0x7f7f_ffff),
        ] {
            e.map(address, 8);
            e.mem.set_u64(address, value);
        }
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    #[test]
    fn n089_006f7a80_builds_the_billboard_geometry() {
        let mut e = Engine::new();
        static_data_doubles(&mut e, 2);
        let tree = e.mem.alloc(0x20);
        for (offset, value) in [
            (4u32, 10.0f32),
            (8, 20.0),
            (0xc, 0.1),
            (0x10, 0.2),
            (0x14, 0.3),
            (0x18, 0.4),
        ] {
            e.mem.set_u32(tree + offset, bits(value));
        }
        e.call_log = Some(vec![]);
        e.call(0x006f_7a80, &args![tree]);
        let vectors = calls_to(&e, 0x0040_1050);
        assert_eq!(vectors.len(), 3);
        assert_eq!(vectors[0][1..], [0xc, 16, 0x0068_15c0]);
        assert_eq!(vectors[1][1..], [8, 16, 0x0068_15c0]);
        assert_eq!(vectors[2][1..], [0xc, 16, 0x0068_15c0]);
        let (positions, uvs, normals) = (vectors[0][0], vectors[1][0], vectors[2][0]);
        let vertex = |e: &Engine, n: u32| {
            [0, 4, 8].map(|k| f32::from_bits(e.mem.u32(positions + n * 0xc + k)))
        };
        assert_eq!(vertex(&e, 0), [-5.0, 0.0, 20.0]);
        assert_eq!(vertex(&e, 3), [5.0, 0.0, 0.0]);
        assert_eq!(vertex(&e, 5), [0.0, 5.0, 20.0]);
        assert_eq!(vertex(&e, 14), [0.0, -5.0, 0.0]);
        assert_eq!(e.mem.u32(uvs + 8), bits(0.3));
        assert_eq!(e.mem.u32(uvs + 12), bits(0.2));
        assert_eq!(e.mem.u32(uvs + 8 * 15 + 4), bits(0.4));
        // Every normal of a tree goes into its first one.
        assert_eq!(e.mem.u32(normals), bits(-1.0));
        assert_eq!(e.mem.u32(normals + 8 * 0xc + 4), bits(1.0));
        assert_eq!(e.mem.u32(normals + 0xc), 0);
        let normalized = calls_to(&e, 0x004a_0c10);
        assert_eq!(normalized.len(), 16);
        assert_eq!(normalized[7], vec![normals]);
        assert_eq!(normalized[8], vec![normals + 8 * 0xc]);
        assert_eq!(calls_to(&e, 0x0045_79e0), vec![vec![0x4000_0000]]);
        let shape = calls_to(&e, 0x00a7_b630);
        assert_eq!(shape.len(), 1);
        let indices = shape[0][9];
        assert_eq!(
            shape[0][..9],
            [0x58 << 12, 16, positions, normals, 0, uvs, 1, 0, 8]
        );
        let words: Vec<u16> = (0..12).map(|k| e.mem.u16(indices + k * 2)).collect();
        assert_eq!(words, [0, 1, 2, 1, 3, 2, 4, 5, 6, 5, 7, 6]);
        let block = calls_to(&e, 0x00a7_2bb0);
        let tree_indices = block[0][2];
        assert_eq!(block[0], vec![(0x2c << 12) + 1, 0, tree_indices, 64, 1]);
        assert_eq!(e.mem.u32(tree_indices + 7 * 4), bits(0.0));
        assert_eq!(e.mem.u32(tree_indices + 9 * 4), bits(1.0));
        assert_eq!(
            calls_to(&e, 0x00a7_26a0),
            vec![vec![(0x2c << 12) + 1, 0, 0, 0, 1, 16, 4, 4]]
        );
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![tree_indices]]);
        let pointer = calls_to(&e, POINTER_CONSTRUCT_00633C90)[0][0];
        assert_eq!(
            calls_to(&e, POINTER_CONSTRUCT_00633C90),
            vec![vec![pointer, (0x58 << 12) + 2]]
        );
        assert_eq!(
            calls_to(&e, 0x00a6_7260),
            vec![vec![pointer + 0x100, (0x2c << 12) + 1]]
        );
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN_006E5CC0),
            vec![vec![tree + 0x1c, pointer]]
        );
        assert_eq!(calls_to(&e, POINTER_SET_0066B0D0), vec![vec![pointer, 0]]);
        assert_eq!(
            calls_to(&e, SCOPE_GUARD_ENTER_00404EB0)[0][1..],
            [0x25, 1, SOURCE_FILE_0106DD30, 0x95]
        );
        // Failed allocations: no constructors run, the pointer holds 0.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.register(NI_ALLOCATE_00AA13E0, |_, _| 0u32.into_ret());
        // With no instances nothing is written into the null arrays.
        e.register(GET_MAX_INSTANCES_00BB6030, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_7a80, &args![tree]);
        assert!(calls_to(&e, 0x0040_1050).is_empty());
        assert!(calls_to(&e, 0x00a7_b630).is_empty());
        assert!(calls_to(&e, 0x0053_7bf0).is_empty());
        assert_eq!(calls_to(&e, 0x00a7_2ca0), vec![vec![0, 1]]);
        assert_eq!(calls_to(&e, POINTER_CONSTRUCT_00633C90)[0][1], 0);
    }

    /// A map object (vtable slots 4 hash 0, 8 compare, 0xc set, 0x10 clear, 0x14 new item) with
    /// one empty bucket.
    fn map_at(e: &mut Engine, at: u32) {
        e.put_vtable(
            0x0333_0000,
            &[
                0,
                0x0333_1004,
                0x0333_1008,
                0x0333_100c,
                0x0333_1010,
                0x0333_1014,
            ],
        );
        e.register(0x0333_1004, |_, _| 0u32.into_ret());
        e.register(0x0333_1008, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(0x0333_100c, quiet);
        e.register(0x0333_1010, quiet);
        e.register(0x0333_1014, |e, _| e.mem.alloc(0xc).into_ret());
        e.mem.set_u32(at, 0x0333_0000);
        let table = e.mem.alloc(4);
        e.mem.set_u32(at + 8, table);
    }

    #[test]
    fn n089_006f8470_adds_a_tree_to_a_new_or_existing_group() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(
            &mut e,
            &[
                SCOPE_GUARD_ENTER_00404EB0,
                SCOPE_GUARD_LEAVE_00404EE0,
                POINTER_CONSTRUCT_00633C90,
                0x005b_5e40,
                0x0062_40d0,
            ],
        );
        e.register(GET_MAX_INSTANCES_00BB6030, |_, _| 4u32.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        // The instance arrays allocate through their vtable's slot 4.
        e.put_vtable(VTABLE_INSTANCE_ARRAY_0106DF78, &[0, 0x0334_0004]);
        e.register(0x0334_0004, |e, a| e.mem.alloc(a[1] * 0x20).into_ret());
        e.map(STATIC_ARRAY_011D85D0, 0x10);
        let types = array_at(&mut e, STATIC_ARRAY_011D85D0, 0x80, 3);
        let block = e.mem.alloc(0x50);
        array_at(&mut e, block + 4, 0x10, 0);
        map_at(&mut e, block + 0x14);
        map_at(&mut e, block + 0x24);
        // No group of type 2 yet.
        e.register(0x0085_3130, |_, _| 0u32.into_ret());
        let instance = e.mem.alloc(0x20);
        e.mem.set_u32(instance + 0x14, 0x1d);
        e.mem.set_u32(instance + 0x1c, 0x0101_0101);
        e.call_log = Some(vec![]);
        e.call(0x006f_8470, &args![block, 2u32, instance]);
        assert_eq!(
            calls_to(&e, SCOPE_GUARD_ENTER_00404EB0)[0][1..],
            [0x25, 1, SOURCE_FILE_0106DD30, 0x14b]
        );
        let lookup = calls_to(&e, 0x0085_3130);
        assert_eq!(lookup[0][..2], [block + 0x24, 2]);
        let group = e.mem.u32(e.mem.u32(block + 8));
        assert_eq!(e.mem.u32(block + 0xc), 1);
        assert_eq!(e.mem.u32(group), types + 0x40);
        assert_eq!(e.mem.u32(group + 8), VTABLE_INSTANCE_ARRAY_0106DF78);
        assert_eq!(e.mem.u32(group + 0x14), 4, "capacity GetMaxInstances");
        assert_eq!(e.mem.u32(group + 0x18), 0);
        assert_eq!(e.mem.u8(group + 0x1c), 0);
        let added = e.mem.u32(group + 0xc);
        assert_eq!(e.mem.u32(added + 0x14), 0x1d);
        assert_eq!(e.mem.u32(added + 0x18), 0);
        assert_eq!(e.mem.u8(added + 0x1c), 0);
        let sets = calls_to(&e, 0x0333_100c);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[0][2..], [2, group]);
        assert_eq!(sets[1][2..], [0x1d, added]);
        assert_eq!(calls_to(&e, SCOPE_GUARD_LEAVE_00404EE0).len(), 1);
        // The group exists and has room: no new group.
        e.register_double(0x0085_3130, move |e, a| {
            e.mem.set_u32(a[2], group);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x006f_8470, &args![block, 2u32, instance]);
        assert_eq!(e.mem.u32(block + 0xc), 1);
        assert_eq!(e.mem.u32(group + 0x10), 2);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        // An unknown type is reported.
        e.register(0x0085_3130, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_8470, &args![block, 3u32, instance]);
        assert_eq!(calls_to(&e, 0x005b_5e40), vec![vec![0x0106_de40, 3]]);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&e, SCOPE_GUARD_LEAVE_00404EE0).len(), 1);
    }

    #[test]
    fn n089_006f8650_constructs_a_tree_group() {
        let mut e = Engine::new();
        stub(&mut e, &[POINTER_CONSTRUCT_00633C90]);
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xcc; 0x20]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_8650, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&e, POINTER_CONSTRUCT_00633C90),
            vec![vec![this + 4, 0]]
        );
        assert_eq!(e.mem.u32(this + 8), VTABLE_INSTANCE_ARRAY_0106DF78);
        assert_eq!(e.mem.bytes(this + 0xc, 0xc), vec![0; 0xc]);
    }

    #[test]
    fn n089_006f86b0_adds_a_type_and_builds_its_data() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        static_data_doubles(&mut e, 0);
        e.map(STATIC_ARRAY_011D85D0, 0x10);
        let types = array_at(&mut e, STATIC_ARRAY_011D85D0, 0x40, 0);
        let source = e.mem.alloc(0x20);
        e.mem.set_u32(source, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x006f_86b0, &args![source]);
        assert_eq!(e.mem.u32(types), 0x77);
        assert_eq!(e.mem.u32(STATIC_ARRAY_011D85D0 + 8), 1);
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN_006E5CC0)[0],
            vec![types + 0x1c, source + 0x1c]
        );
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN_006E5CC0)[1][0],
            types + 0x1c,
            "the static data of the entry just added"
        );
        assert_eq!(calls_to(&e, 0x006b_3140), vec![vec![STATIC_ARRAY_011D85D0]]);
    }

    #[test]
    fn n089_006f8800_and_006f8830_destroy_a_tree_group() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                ARRAY_FREE_008454F0,
                POINTER_DESTROY_0045CEC0,
                OPERATOR_DELETE,
            ],
        );
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_8800, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 8), VTABLE_INSTANCE_ARRAY_0106DF78);
        assert_eq!(calls_to(&e, ARRAY_FREE_008454F0), vec![vec![this + 8, 1]]);
        assert_eq!(calls_to(&e, POINTER_DESTROY_0045CEC0), vec![vec![this + 4]]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x006f_8800, &args![this, 3u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.call_log = Some(vec![]);
        e.call(0x006f_8830, &args![this]);
        assert_eq!(calls_to(&e, POINTER_DESTROY_0045CEC0), vec![vec![this + 4]]);
    }

    #[test]
    fn n089_006f88e0_marks_the_block_loaded() {
        let mut e = Engine::new();
        stub(&mut e, &[0x0092_9260, 0x006f_74f0]);
        e.register(POINTER_GET_00559450, |_, a| (a[0] + 0x100).into_ret());
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x44, 0x4400);
        e.call_log = Some(vec![]);
        e.call(0x006f_88e0, &args![this]);
        assert_eq!(e.mem.u8(this + 0x48), 1);
        assert_eq!(calls_to(&e, POINTER_GET_00559450), vec![vec![0x4400]]);
        assert_eq!(calls_to(&e, 0x0092_9260), vec![vec![0x4500, 1]]);
        assert_eq!(calls_to(&e, 0x006f_74f0), vec![vec![this, 0]]);
    }

    fn min_max_doubles(e: &mut Engine) {
        e.register(0x0041_6870, |e, a| {
            for k in 0..3 {
                e.mem.set_u32(a[0] + k * 4, a[1 + k as usize]);
            }
            Ret::default()
        });
        e.register(0x0040_ebd0, |_, a| {
            f32::from_bits(a[0]).min(f32::from_bits(a[1])).into_ret()
        });
        e.register(0x0040_4010, |_, a| {
            f32::from_bits(a[0]).max(f32::from_bits(a[1])).into_ret()
        });
        // The vector helpers return their result's address.
        e.register(0x0043_9ef0, |_, a| a[1].into_ret());
        e.register(0x0053_d280, |_, a| a[1].into_ret());
        // The sum copies `this` (the minimum) into its result.
        e.register(0x0043_9e90, |e, a| {
            for k in 0..3 {
                let value = e.mem.u32(a[0] + k * 4);
                e.mem.set_u32(a[1] + k * 4, value);
            }
            a[1].into_ret()
        });
        e.register(0x0045_7990, |_, _| 3.0f32.into_ret());
        e.register(0x0084_d030, |_, _| 0.5f32.into_ret());
        e.register(0x0062_40d0, quiet);
        e.register(0x004a_51d0, |e, a| {
            for k in 0..3 {
                let value = e.mem.u32(a[1] + k * 4);
                e.mem.set_u32(a[0] + k * 4, value);
            }
            e.mem.set_u32(a[0] + 0xc, a[2]);
            Ret::default()
        });
        let mut slots = vec![0u32; 0x3f];
        slots[0xf8 / 4] = 0x0335_10f8;
        e.put_vtable(0x0335_0000, &slots);
        e.register(0x0335_10f8, |_, a| (a[0] + 1).into_ret());
    }

    #[test]
    fn n089_006f8910_computes_a_group_bound() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        min_max_doubles(&mut e);
        for (address, value) in [
            (TWO_010162C0, 0x4000_0000u32),
            (FLOAT_MAX_01016970, 0x7f7f_ffff),
        ] {
            e.map(address, 4);
            e.mem.set_u32(address, value);
        }
        let geometry = e.mem.alloc(8);
        e.mem.set_u32(geometry, 0x0335_0000);
        e.register_double(POINTER_GET_00559450, move |_, _| geometry.into_ret());
        let group = e.mem.alloc(0x20);
        let instances = array_at(&mut e, group + 8, 0x40, 2);
        for (k, value) in [1.0f32, -2.0, 3.0, 4.0, 5.0, -6.0].iter().enumerate() {
            let at = instances + (k as u32 / 3) * 0x20 + (k as u32 % 3) * 4;
            e.mem.set_u32(at, value.to_bits());
        }
        let result = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006f_8910, &args![0u32, result, group]).u32(),
            result
        );
        let words: Vec<f32> = (0..4).map(|k| e.mem.f32(result + k * 4)).collect();
        // The centre is the minimum (the doubles return their result unchanged).
        assert_eq!(words, [1.0, -2.0, -6.0, 3.5]);
        let centre = calls_to(&e, 0x004a_51d0)[0][1];
        assert_eq!(calls_to(&e, 0x0053_d280)[0][2], 0x4000_0000);
        assert_eq!(calls_to(&e, 0x0043_9e90)[0][1], centre);
        assert_eq!(calls_to(&e, 0x0084_d030), vec![vec![geometry + 1]]);
        assert_eq!(calls_to(&e, 0x0041_6870)[1][1], 0xff7f_ffff);
        assert_eq!(calls_to(&e, 0x0040_ebd0).len(), 6);
    }

    /// A file whose reads (`00462d80`) take the words at `stream` one by one (0x20-byte reads
    /// take a fill word).
    fn file_doubles(e: &mut Engine, words: &[u32]) {
        let stream = e.mem.alloc(4 + 4 * words.len() as u32);
        for (k, word) in words.iter().enumerate() {
            e.mem.set_u32(stream + 4 + 4 * k as u32, *word);
        }
        e.register_double(FILE_READ_00462D80, move |e, a| {
            let cursor = e.mem.u32(stream);
            let word = e.mem.u32(stream + 4 + cursor * 4);
            e.mem.set_u32(stream, cursor + 1);
            for k in 0..a[2] / 4 {
                e.mem.set_u32(a[1] + k * 4, word);
            }
            Ret::default()
        });
    }

    #[test]
    fn n089_006f8b40_reads_the_block_instances() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(
            &mut e,
            &[
                SCOPE_GUARD_ENTER_00404EB0,
                SCOPE_GUARD_LEAVE_00404EE0,
                ARRAY_FREE_008454F0,
                0x005b_5e40,
                0x0062_40d0,
            ],
        );
        // No tree types, so every instance is reported as an invalid type.
        e.map(STATIC_ARRAY_011D85D0, 0x10);
        e.register(0x0085_3130, |_, _| 0u32.into_ret());
        let block = e.mem.alloc(0x50);
        array_at(&mut e, block + 4, 4, 0);
        file_doubles(&mut e, &[1, 7, 2, 0xaa, 0xbb]);
        e.call_log = Some(vec![]);
        e.call(0x006f_8b40, &args![block, 0xf11eu32]);
        assert_eq!(
            calls_to(&e, SCOPE_GUARD_ENTER_00404EB0)[0][1..],
            [0x25, 1, SOURCE_FILE_0106DD30, 0x204]
        );
        assert_eq!(calls_to(&e, ARRAY_FREE_008454F0), vec![vec![block + 4, 1]]);
        let reads = calls_to(&e, FILE_READ_00462D80);
        assert_eq!(reads.len(), 5);
        assert_eq!(reads[4][2], 0x20);
        assert_eq!(reads[0][0], 0xf11e);
        assert_eq!(calls_to(&e, 0x005b_5e40), vec![vec![0x0106_de40, 7]; 2]);
        assert_eq!(calls_to(&e, 0x0062_40d0).len(), 2);
    }

    #[test]
    fn n089_006f8c40_attaches_the_geometry_or_hides_queued_trees() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        min_max_doubles(&mut e);
        for (address, value) in [
            (TWO_010162C0, 0x4000_0000u32),
            (FLOAT_MAX_01016970, 0x7f7f_ffff),
        ] {
            e.map(address, 4);
            e.mem.set_u32(address, value);
        }
        stub(
            &mut e,
            &[
                SCOPE_GUARD_ENTER_00404EB0,
                SCOPE_GUARD_LEAVE_00404EE0,
                POINTER_SET_0066B0D0,
                0x0070_5b10,
                ATTACH_PROPERTY_00439410,
                0x00b5_7e30,
                0x00a5_a040,
                VECTOR_SET_0043D410,
                SET_TRANSLATE_00A59C60,
                0x0335_10dc,
                0x0335_10f4,
                0x006f_7780,
            ],
        );
        e.register(GET_MAX_INSTANCES_00BB6030, |_, _| 0u32.into_ret());
        e.register(0x0042_f5a0, |_, a| a[0].min(a[1]).into_ret());
        e.register(NI_ALLOCATE_00AA13E0, |_, a| (a[0] << 12).into_ret());
        e.register(0x00a8_6dd0, |_, a| (a[0] + 1).into_ret());
        e.register(0x00bb_7170, |_, a| (a[0] + 2).into_ret());
        e.register(NODE_POINTER_008041A0, |_, a| (a[0] + 0x40).into_ret());
        e.register(0x005585e0, |_, a| (a[0] + 3).into_ret());
        let parent = e.mem.alloc(8);
        e.register_double(0x004e_a9e0, move |_, _| parent.into_ret());
        let geometry = e.mem.alloc(8);
        e.mem.set_u32(geometry, 0x0335_0000);
        e.mem.set_u32(parent, 0x0335_0000);
        e.mem.set_u32(0x0335_0000 + 0xdc, 0x0335_10dc);
        e.mem.set_u32(0x0335_0000 + 0xf4, 0x0335_10f4);
        e.register_double(POINTER_GET_00559450, move |_, _| geometry.into_ret());
        let tree_type = e.mem.alloc(0x20);
        let group = e.mem.alloc(0x20);
        e.mem.set_u32(group, tree_type);
        array_at(&mut e, group + 8, 0x20, 0);
        let block = e.mem.alloc(0x50);
        let groups = array_at(&mut e, block + 4, 4, 1);
        e.mem.set_u32(groups, group);
        e.mem.set_u8(block + 0x48, 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_8c40, &args![block]);
        assert_eq!(
            calls_to(&e, SCOPE_GUARD_ENTER_00404EB0)[0][1..],
            [0x25, 1, SOURCE_FILE_0106DD30, 0x240]
        );
        assert_eq!(e.mem.u8(block + 0x49), 1);
        assert_eq!(calls_to(&e, 0x00a8_6dd0), vec![vec![0xd8 << 12, geometry]]);
        assert_eq!(
            calls_to(&e, POINTER_SET_0066B0D0),
            vec![vec![group + 4, (0xd8 << 12) + 1]]
        );
        assert_eq!(calls_to(&e, 0x0070_5b10), vec![vec![geometry, 0]]);
        assert_eq!(e.mem.u8(group + 0x1c), 1, "the property was set up");
        assert_eq!(
            calls_to(&e, ATTACH_PROPERTY_00439410),
            vec![
                vec![geometry, (0x68 << 12) + 2],
                vec![geometry, geometry],
                vec![geometry, geometry]
            ]
        );
        assert_eq!(calls_to(&e, 0x00b5_7e30), vec![vec![geometry, 0, 0]]);
        assert_eq!(calls_to(&e, 0x0335_10dc), vec![vec![parent, geometry, 1]]);
        assert_eq!(calls_to(&e, SET_TRANSLATE_00A59C60).len(), 3);
        assert_eq!(calls_to(&e, SET_TRANSLATE_00A59C60)[1][0], geometry + 3);
        assert_eq!(calls_to(&e, 0x0335_10f4)[0].len(), 5);
        assert!(calls_to(&e, 0x006f_7780).is_empty());
        // Attached: the queued trees are hidden.
        e.call_log = Some(vec![]);
        e.call(0x006f_8c40, &args![block]);
        assert_eq!(calls_to(&e, 0x006f_7780), vec![vec![block]]);
        assert!(calls_to(&e, NI_ALLOCATE_00AA13E0).is_empty());
        // Not loaded: nothing.
        e.mem.set_u8(block + 0x48, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_8c40, &args![block]);
        assert!(calls_to(&e, 0x006f_7780).is_empty());
        assert_eq!(calls_to(&e, SCOPE_GUARD_LEAVE_00404EE0).len(), 1);
    }

    #[test]
    fn n089_006f8f80_reads_the_tree_types() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        static_data_doubles(&mut e, 0);
        e.map(STATIC_ARRAY_011D85D0, 0x10);
        array_at(&mut e, STATIC_ARRAY_011D85D0, 0x40, 1);
        stub(&mut e, &[0x006a_8500]);
        file_doubles(&mut e, &[2, 0x11, 0x22]);
        e.call_log = Some(vec![]);
        e.call(0x006f_8f80, &args![0xf11eu32]);
        assert_eq!(
            calls_to(&e, SCOPE_GUARD_ENTER_00404EB0)[0][1..],
            [0x25, 1, SOURCE_FILE_0106DD30, 0x27e]
        );
        // The old entry was destroyed and the buffer freed.
        assert_eq!(calls_to(&e, 0x006a_8500), vec![vec![STATIC_ARRAY_011D85D0]]);
        assert_eq!(calls_to(&e, FILE_READ_00462D80).len(), 3);
        let local = calls_to(&e, FILE_READ_00462D80)[1][1];
        // Each local: constructed, its pointer cleared, added, destroyed.
        assert!(calls_to(&e, POINTER_SET_0066B0D0).contains(&vec![local + 0x1c, 0]));
        assert_eq!(
            calls_to(&e, POINTER_DESTROY_0045CEC0)
                .iter()
                .filter(|c| c[0] == local + 0x1c)
                .count(),
            2
        );
        assert_eq!(e.mem.u32(STATIC_ARRAY_011D85D0 + 8), 2);
        let types = e.mem.u32(STATIC_ARRAY_011D85D0 + 4);
        assert_eq!(e.mem.u32(types + 0x20), 0x22);
    }

    #[test]
    fn n089_006f9060_constructs_a_tree_type() {
        let mut e = Engine::new();
        stub(&mut e, &[POINTER_CONSTRUCT_00633C90]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9060, &args![0x5000u32]).u32(), 0x5000);
        assert_eq!(
            calls_to(&e, POINTER_CONSTRUCT_00633C90),
            vec![vec![0x501c, 0]]
        );
    }

    #[test]
    fn n089_006f90a0_fills_the_shader_instances() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(&mut e, &[0x00bb_7290]);
        e.register(GET_MAX_INSTANCES_00BB6030, |_, _| 3u32.into_ret());
        e.register(0x0042_f5a0, |_, a| a[0].min(a[1]).into_ret());
        let group = e.mem.alloc(0x20);
        let instances = array_at(&mut e, group + 8, 0x40, 2);
        for k in 0..5 {
            e.mem.set_u32(instances + k * 4, 0x100 + k);
            e.mem.set_u32(instances + 0x20 + k * 4, 0x200 + k);
        }
        // A signalling NaN is quieted by the FLD/FSTP copy.
        e.mem.set_u32(instances + 0x20, 0x7f80_0001);
        e.mem.set_u8(instances + 0x3c, 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_90a0, &args![0u32, 0x9900u32, group]);
        assert_eq!(calls_to(&e, 0x0042_f5a0), vec![vec![2, 3]]);
        assert_eq!(
            calls_to(&e, 0x00bb_7290),
            vec![
                vec![0x9900, 0, 0x100, 0x101, 0x102, 0x103, 0x104],
                vec![0x9900, 1, 0x7fc0_0001, 0x201, 0x202, 0x203, 0],
                vec![0x9900, 2, 0, 0, 0, 0, 0]
            ]
        );
        assert_eq!(e.mem.u8(group + 0x1c), 1);
    }

    #[test]
    fn n089_006f91a0_updates_out_of_date_properties() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        e.register(GET_MAX_INSTANCES_00BB6030, |_, _| 0u32.into_ret());
        e.register(0x0042_f5a0, |_, _| 0u32.into_ret());
        e.register(0x0043_8220, |_, _| 0x0ead_0000u32.into_ret());
        e.register(POINTER_GET_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x00a5_9d30, |_, a| (a[0] & 0xff).into_ret());
        let block = e.mem.alloc(0x50);
        let groups = array_at(&mut e, block + 4, 0x10, 4);
        let mut made = vec![];
        // Up to date; no geometry; no property; one to set up.
        for (flag, geometry) in [(1u8, 0x1100u32), (0, 0), (0, 0x2200), (0, 0x3301)] {
            let group = e.mem.alloc(0x20);
            e.mem.set_u8(group + 0x1c, flag);
            e.mem.set_u32(group + 4, geometry);
            array_at(&mut e, group + 8, 4, 0);
            made.push(group);
        }
        for (k, group) in made.iter().enumerate() {
            e.mem.set_u32(groups + 4 * k as u32, *group);
        }
        e.call_log = Some(vec![]);
        e.call(0x006f_91a0, &args![block]);
        assert_eq!(
            calls_to(&e, 0x00a5_9d30),
            vec![vec![0x2200, 0x0ead_0000], vec![0x3301, 0x0ead_0000]]
        );
        assert_eq!(e.mem.u8(made[2] + 0x1c), 0);
        assert_eq!(e.mem.u8(made[3] + 0x1c), 1);
        assert_eq!(e.mem.u8(made[1] + 0x1c), 0);
    }

    #[test]
    fn n089_006f9300_hides_the_queued_trees() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(&mut e, &[0x006f_7780, 0x007c_b2e0]);
        e.register(0x0085_3130, |_, _| 0u32.into_ret());
        let block = e.mem.alloc(0x50);
        let queue = array_at(&mut e, block + 0x34, 8, 2);
        e.mem.set_u32(queue, 0x51);
        e.mem.set_u32(queue + 4, 0x52);
        e.call_log = Some(vec![]);
        e.call(0x006f_9300, &args![block]);
        let lookups = calls_to(&e, 0x0085_3130);
        assert_eq!(lookups.len(), 2);
        assert_eq!(lookups[1][..2], [block + 0x14, 0x52]);
        assert!(calls_to(&e, 0x007c_b2e0).is_empty(), "not queued again");
        assert_eq!(calls_to(&e, 0x006f_7780), vec![vec![block]]);
    }

    #[test]
    fn n089_006f9640_describes_the_task() {
        let mut e = Engine::new();
        stub(&mut e, &[FORMAT_00406D00]);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + 0x38, 3);
        e.mem.set_u32(this + 0x3c, 0xffff_fffe);
        e.mem.set_u32(this + 0x40, 4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006f_9640, &args![this, 0xb0ffu32, 0x80u32]).bool());
        assert_eq!(
            calls_to(&e, FORMAT_00406D00),
            vec![vec![0xb0ff, 0x80, 0x0106_df00, 3, 0xffff_fffe, 4]]
        );
    }

    /// Calls a scalar deleting destructor with flags 0 and 1 and checks the inner call and the
    /// delete.
    fn check_scalar_deleting(address: u32, inner: u32, inner_args: fn(u32) -> Vec<u32>) {
        let mut e = Engine::new();
        stub(&mut e, &[inner, OPERATOR_DELETE]);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![this, 2u32]).u32(), this);
        assert_eq!(calls_to(&e, inner), vec![inner_args(this)]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call_log = Some(vec![]);
        e.call(address, &args![this, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn n089_006f96e0_deletes_the_instance_map() {
        // 006f9940 runs for real: its callee 00438af0 is the double.
        check_scalar_deleting(0x006f_96e0, 0x0043_8af0, |this| vec![this]);
    }

    #[test]
    fn n089_006f9710_deletes_the_group_map() {
        check_scalar_deleting(0x006f_9710, 0x0043_8af0, |this| vec![this]);
    }

    #[test]
    fn n089_006f9740_and_006f9770_instance_array() {
        let mut e = Engine::new();
        stub(&mut e, &[ARRAY_FREE_008454F0]);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcc; 0x10]);
        assert_eq!(e.call(0x006f_9740, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_INSTANCE_ARRAY_0106DF78);
        assert_eq!(e.mem.bytes(this + 4, 0xc), vec![0; 0xc]);
        e.mem.set_u32(this, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_9770, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_INSTANCE_ARRAY_0106DF78);
        assert_eq!(calls_to(&e, ARRAY_FREE_008454F0), vec![vec![this, 1]]);
    }

    #[test]
    fn n089_006f9800_appends_an_instance() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(&mut e, &[0x0062_40d0]);
        let array = e.mem.alloc(0x10);
        let buffer = array_at(&mut e, array, 0x60, 1);
        let item = e.mem.alloc(0x20);
        for k in 0..8 {
            e.mem.set_u32(item + k * 4, 0xa0 + k);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9800, &args![array, item]).u32(), 1);
        assert_eq!(calls_to(&e, 0x006e_6da0), vec![vec![0x20, buffer + 0x20]]);
        assert_eq!(calls_to(&e, 0x0062_40d0), vec![vec![buffer + 0x20]]);
        assert_eq!(e.mem.u32(buffer + 0x3c), 0xa7);
        assert_eq!(e.mem.u32(buffer + 0x40), 0);
    }

    #[test]
    fn n089_006f9850_reallocates_and_answers_the_new_buffer() {
        let mut e = Engine::new();
        e.register(0x0042_f5d0, |_, _| 0x7000u32.into_ret());
        e.call_log = Some(vec![]);
        let answer = e.call(0x006f_9850, &args![0u32, 0x6000u32, 3u32]).u32();
        assert_eq!(calls_to(&e, 0x0042_f5d0), vec![vec![0x6000, 0x60]]);
        assert_eq!(answer, 0x7000, "the buffer 0042f5d0 leaves in EAX");
    }

    #[test]
    fn n089_006f9c20_appends_a_tree_type() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        stub(
            &mut e,
            &[POINTER_CONSTRUCT_00633C90, POINTER_ASSIGN_006E5CC0],
        );
        let array = e.mem.alloc(0x10);
        let buffer = array_at(&mut e, array, 0x60, 2);
        let item = e.mem.alloc(0x20);
        e.mem.set_u32(item, 0x55);
        e.mem.set_u32(item + 0x18, 0x3f80_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9c20, &args![array, item]).u32(), 2);
        let slot = buffer + 0x40;
        assert_eq!(
            calls_to(&e, POINTER_CONSTRUCT_00633C90),
            vec![vec![slot + 0x1c, 0]]
        );
        assert_eq!(e.mem.u32(slot), 0x55);
        assert_eq!(e.mem.u32(slot + 0x18), 0x3f80_0000);
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN_006E5CC0),
            vec![vec![slot + 0x1c, item + 0x1c]]
        );
    }

    #[test]
    fn n089_006f9cd0_deletes_the_instance_array() {
        check_scalar_deleting(0x006f_9cd0, ARRAY_FREE_008454F0, |this| vec![this, 1]);
    }

    #[test]
    fn n089_006f9d00_deletes_the_group_array() {
        check_scalar_deleting(0x006f_9d00, ARRAY_FREE_008454F0, |this| vec![this, 1]);
    }

    #[test]
    fn n089_006f9d30_deletes_the_instance_map_base() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x7700);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9d30, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_INSTANCE_MAP_BASE_0106DFA0);
        assert_eq!(
            calls_to(&e, OPERATOR_DELETE),
            vec![vec![0x7700], vec![this]]
        );
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_9d30, &args![this, 0u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn n089_006f9d60_deletes_the_group_map_base() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE]);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x006f_9d60, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), VTABLE_GROUP_MAP_BASE_0106DFC0);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn n089_006f9d90_assigns_a_tree_type() {
        let mut e = Engine::new();
        stub(&mut e, &[POINTER_ASSIGN_006E5CC0]);
        let source = e.mem.alloc(0x20);
        for k in 0..7 {
            e.mem.set_u32(source + k * 4, 0x3f00_0000 + k);
        }
        e.mem.set_u32(source, 0x7f80_0001);
        e.mem.set_u32(source + 8, 0xff80_0002);
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_9d90, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x7f80_0001, "a plain word copy");
        assert_eq!(e.mem.u32(this + 8), 0xffc0_0002, "a float copy");
        assert_eq!(e.mem.u32(this + 0x18), 0x3f00_0006);
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN_006E5CC0),
            vec![vec![this + 0x1c, source + 0x1c]]
        );
    }

    #[test]
    fn n089_006f9e10_constructs_instances_in_place() {
        let mut e = Engine::new();
        stub(&mut e, &[0x0062_40d0]);
        e.register(0x006e_6da0, |_, a| {
            (if a[1] == 0x8020 { 0 } else { a[1] }).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x006f_9e10, &args![0u32, 0x8000u32, 3u32]);
        assert_eq!(calls_to(&e, 0x006e_6da0).len(), 3);
        assert_eq!(calls_to(&e, 0x0062_40d0), vec![vec![0x8000], vec![0x8040]]);
    }

    #[test]
    fn n089_006f9f60_constructs_the_instance_array() {
        let mut e = Engine::new();
        stub(&mut e, &[0x0062_40d0]);
        e.register(0x006e_6da0, |_, a| a[1].into_ret());
        e.put_vtable(0x0337_0000, &[0, 0x0337_1004]);
        e.register(0x0337_1004, |_, a| (0x9000 + a[1]).into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0337_0000);
        e.call_log = Some(vec![]);
        e.call(0x006f_9f60, &args![this, 1u32, 2u32]);
        assert_eq!(calls_to(&e, 0x0337_1004), vec![vec![this, 2]]);
        assert_eq!(e.mem.u32(this + 4), 0x9002);
        assert_eq!(e.mem.u32(this + 8), 2);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(calls_to(&e, 0x0062_40d0), vec![vec![0x9002], vec![0x9022]]);
        e.call_log = Some(vec![]);
        e.call(0x006f_9f60, &args![this, 0u32, 0u32]);
        assert!(calls_to(&e, 0x0337_1004).is_empty());
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn n089_006f9ff0_constructs_tree_types_in_place() {
        let mut e = Engine::new();
        stub(&mut e, &[POINTER_CONSTRUCT_00633C90]);
        e.register(0x006e_6da0, |_, a| {
            (if a[1] == 0x8000 { 0 } else { a[1] }).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x006f_9ff0, &args![0u32, 0x8000u32, 2u32]);
        assert_eq!(
            calls_to(&e, POINTER_CONSTRUCT_00633C90),
            vec![vec![0x803c, 0]]
        );
    }

    #[test]
    fn n089_006fa090_and_006fa0d0_destroy_tree_types() {
        let mut e = Engine::new();
        stub(&mut e, &[CALLEE_0045CEC0, OPERATOR_DELETE]);
        e.call_log = Some(vec![]);
        e.call(0x006f_a090, &args![0u32, 0x8000u32, 2u32]);
        assert_eq!(
            calls_to(&e, CALLEE_0045CEC0),
            vec![vec![0x801c], vec![0x803c]]
        );
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_a0d0, &args![0x8000u32, 1u32]).u32(), 0x8000);
        assert_eq!(calls_to(&e, CALLEE_0045CEC0), vec![vec![0x801c]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x8000]]);
    }

    #[test]
    fn n089_006fa130_deletes_the_tree_type_array() {
        check_scalar_deleting(0x006f_a130, 0x006f_a160, |this| vec![this]);
    }
}
