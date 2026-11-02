//! `BSShader/imagespacemanager.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's word at +0x2b4 (the allocation context), 0x27 during the set-up.
const TLS_CONTEXT: u32 = 0x2b4;
const CONTEXT_IMAGE_SPACE: u32 = 0x27;
/// The manager's first-run byte (+0x1e4).
const INITIALIZED: u32 = 0x1e4;
/// The set-up steps, each `thiscall` on the manager without stack words: `00b94520` (engine
/// map unit `imagespacemanager.cpp`), `ImageSpaceManager::InitGeometry` (`00b8c170`),
/// `ImageSpaceManager::SetupEffects` (`00b8b360`) and `00b8ad70` (same unit).
const SET_UP_00B94520: u32 = 0x00b9_4520;
const INIT_GEOMETRY: u32 = 0x00b8_c170;
const SETUP_EFFECTS: u32 = 0x00b8_b360;
const SET_UP_00B8AD70: u32 = 0x00b8_ad70;
/// `ImageSpaceShaderParam::InitConstantMap` (engine map, `00b8ab10`, `thiscall` on the object
/// at `01202410`, `RET 8`).
const INIT_CONSTANT_MAP: u32 = 0x00b8_ab10;
const CONSTANT_MAP_01202410: u32 = 0x0120_2410;
/// The `float` stored at +0x304 (`01017870`).
const VALUE_01017870: u32 = 0x0101_7870;
/// `00b6e110` (unnamed, `BSShader`; `thiscall` on the object at `011f91a8`, `RET 0x14`), given
/// the word at `011f4748`, 3, 0, 0, 0; its answer goes to +0x318.
const MAKE_00B6E110: u32 = 0x00b6_e110;
const OBJECT_011F91A8: u32 = 0x011f_91a8;
const OBJECT_011F4748: u32 = 0x011f_4748;

// Translated from 00b977d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceManager::Initialize` (Xbox PDB). `thiscall`, no stack words. With the thread's
/// allocation context (+0x2b4 of the thread block) set to 0x27 for the duration (and put back
/// after): when the byte at +0x1e4 is clear, runs `00b94520` and `InitGeometry` (`00b8c170`),
/// sets the byte, runs `SetupEffects` (`00b8b360`), `ImageSpaceShaderParam::InitConstantMap`
/// on `01202410` with (0, 0) and `00b8ad70`; then stores, in the exe's order, 0.0 at +0x2f8,
/// +0x2fc, +0x308 and +0x300, 1.0 at +0x30c and +0x310, the `float` at `01017870` (through
/// the FPU, a signalling NaN quieted) at +0x304, clears +0, stores 0.0 at +0x314, and keeps
/// at +0x318 what `00b6e110` (on `011f91a8`, with the word at `011f4748`, 3, 0, 0, 0) answers.
// Translation incomplete: 00b977d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation context) is not translated.
pub fn image_space_manager_initialize(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem
        .set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_IMAGE_SPACE);
    if e.mem.u8(base.wrapping_add(INITIALIZED)) == 0 {
        e.call(SET_UP_00B94520, &args![this]);
        e.call(INIT_GEOMETRY, &args![this]);
        e.mem.set_u8(base.wrapping_add(INITIALIZED), 1);
        e.call(SETUP_EFFECTS, &args![this]);
        e.call(INIT_CONSTANT_MAP, &args![CONSTANT_MAP_01202410, 0u32, 0u32]);
        e.call(SET_UP_00B8AD70, &args![this]);
        let zero = x87_store32(0.0);
        let one = x87_store32(1.0);
        let loaded = x87_quiet_bits(e.global::<u32>(VALUE_01017870));
        for offset in [0x2f8u32, 0x2fc, 0x308, 0x300] {
            e.mem.set_u32(base.wrapping_add(offset), zero);
        }
        e.mem.set_u32(base.wrapping_add(0x30c), one);
        e.mem.set_u32(base.wrapping_add(0x310), one);
        e.mem.set_u32(base.wrapping_add(0x304), loaded);
        e.mem.set_u32(base, 0);
        e.mem.set_u32(base.wrapping_add(0x314), zero);
        let object = e.global::<u32>(OBJECT_011F91A8);
        let word = e.global::<u32>(OBJECT_011F4748);
        let made = e
            .call(MAKE_00B6E110, &args![object, word, 3u32, 0u32, 0u32, 0u32])
            .u32();
        e.mem.set_u32(base.wrapping_add(0x318), made);
    }
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
}

// Translated from 00b8b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceManager::SetupEffects` (Xbox PDB), `thiscall`, no stack words: for each of the
/// 0x22 effect slots of the array at +0x8 (re-read for every slot), calls a set effect's
/// vtable slot +0x8 with the manager and 0.
pub fn image_space_manager_setup_effects(e: &mut Engine, this: Ptr) {
    for index in 0..0x22u32 {
        let effects = e.mem.u32(this.addr() + 8);
        let effect = e.mem.u32(effects.wrapping_add(4 * index));
        if effect != 0 {
            e.vcall(effect, 8, &args![this, 0u32]);
        }
    }
}

/// The `float` `00b8ad70` keeps on the x87 stack besides 1.0 and 0.0 (`01016248`).
const HALF_01016248: u32 = 0x0101_6248;

/// `FLD dword` of the `float` at `address` stored by `FSTP`: its bits, a signalling NaN
/// quieted.
fn loaded(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(address))
}

// Translated from 00b8ad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), no arguments: stores the image
/// space defaults in the `float`s from `011ff940` to `011ff9d0`, in the exe's order, from the
/// x87 stack (`FLD`/`FST`/`FXCH` tracked; each `FLD` quiets a signalling NaN): the `float` at
/// `01016248` in `011ff940`, `011ff988`, `011ff98c`, `011ff998` and `011ff9a8`; 1.0 in
/// `011ff94c`, `011ff950`, `011ff954`, `011ff960`, `011ff974`, `011ff948` and `011ff9a4` to
/// `011ff9cc` but `011ff9a8` and `011ff9c0`; 0.0 in `011ff964`, `011ff968`, `011ff9c0`,
/// `011ff9d0`; one constant each in the rest (`01018f98` in `011ff944`, `01022154` in
/// `011ff958`, `0102212c` in `011ff95c`, `01016088` in `011ff96c` and `011ff970`, `01021c9c`
/// in `011ff97c`, `0101df44` in `011ff984`, `01018238` in `011ff980`, `0102d934` in
/// `011ff990`, `010184f4` in `011ff9a0`, `0102d930` in `011ff994`, `0102d92c` in `011ff99c`).
pub fn fn_00b8ad70(e: &mut Engine) {
    let one = 1.0f32.to_bits();
    let zero = 0.0f32.to_bits();
    let half = loaded(e, HALF_01016248);
    e.set_global::<u32>(0x011f_f940, half);
    let value = loaded(e, 0x0101_8f98);
    e.set_global::<u32>(0x011f_f944, value);
    for target in [0x011f_f94cu32, 0x011f_f950, 0x011f_f954] {
        e.set_global::<u32>(target, one);
    }
    let value = loaded(e, 0x0102_2154);
    e.set_global::<u32>(0x011f_f958, value);
    let value = loaded(e, 0x0102_212c);
    e.set_global::<u32>(0x011f_f95c, value);
    e.set_global::<u32>(0x011f_f960, one);
    e.set_global::<u32>(0x011f_f964, zero);
    e.set_global::<u32>(0x011f_f968, zero);
    let value = loaded(e, 0x0101_6088);
    e.set_global::<u32>(0x011f_f96c, value);
    e.set_global::<u32>(0x011f_f970, value);
    e.set_global::<u32>(0x011f_f974, one);
    e.set_global::<u32>(0x011f_f948, one);
    let value = loaded(e, 0x0102_1c9c);
    e.set_global::<u32>(0x011f_f97c, value);
    let value = loaded(e, 0x0101_df44);
    e.set_global::<u32>(0x011f_f984, value);
    let value = loaded(e, 0x0101_8238);
    e.set_global::<u32>(0x011f_f980, value);
    e.set_global::<u32>(0x011f_f988, half);
    e.set_global::<u32>(0x011f_f98c, half);
    let value = loaded(e, 0x0102_d934);
    e.set_global::<u32>(0x011f_f990, value);
    let value = loaded(e, 0x0101_84f4);
    e.set_global::<u32>(0x011f_f9a0, value);
    let value = loaded(e, 0x0102_d930);
    e.set_global::<u32>(0x011f_f994, value);
    e.set_global::<u32>(0x011f_f998, half);
    let value = loaded(e, 0x0102_d92c);
    e.set_global::<u32>(0x011f_f99c, value);
    for target in [
        0x011f_f9b0u32,
        0x011f_f9ac,
        0x011f_f9a4,
        0x011f_f9b4,
        0x011f_f9b8,
        0x011f_f9bc,
        0x011f_f9c4,
        0x011f_f9c8,
        0x011f_f9cc,
    ] {
        e.set_global::<u32>(target, one);
    }
    e.set_global::<u32>(0x011f_f9a8, half);
    e.set_global::<u32>(0x011f_f9c0, zero);
    e.set_global::<u32>(0x011f_f9d0, zero);
}

/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `InitGeometry`'s callees (engine map names; all `thiscall`):
/// `NiScreenElementsData::NiScreenElementsData` (`00a881e0`, nine words),
/// `NiScreenElements::NiScreenElements` (`00a87810`, the data), `NiScreenElementsData::Insert`
/// (`00a87ae0`, three words), `::SetRectangle` (`00a88020`: index and four `float`s),
/// `::SetTextures` (`00a88130`: two words and four `float`s), `::UpdateBound` (`00a87900`),
/// the name setter `00a5b950` (unnamed: the address of an `NiFixedString`),
/// `NiAVObject::UpdateProperties` (`00a5a040`) and the update `00a59c60` (unnamed: the address
/// of an update block); and the name maker `00a5b690` (`cdecl`, a string).
const SCREEN_ELEMENTS_DATA_CONSTRUCT: u32 = 0x00a8_81e0;
const SCREEN_ELEMENTS_CONSTRUCT: u32 = 0x00a8_7810;
const SCREEN_ELEMENTS_DATA_INSERT: u32 = 0x00a8_7ae0;
const SCREEN_ELEMENTS_DATA_SET_RECTANGLE: u32 = 0x00a8_8020;
const SCREEN_ELEMENTS_DATA_SET_TEXTURES: u32 = 0x00a8_8130;
const SCREEN_ELEMENTS_DATA_UPDATE_BOUND: u32 = 0x00a8_7900;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
const MAKE_FIXED_STRING_00A5B690: u32 = 0x00a5_b690;
/// `"Imagespace Effect Geometry"` (`010b04ac`) and the empty name word (`0109b220`).
const GEOMETRY_NAME_010B04AC: u32 = 0x010b_04ac;
const NULL_NAME_0109B220: u32 = 0x0109_b220;
/// The rectangle `float`s (`01012054`, `010162c0`, `0104a494`; each through `FLD`).
const RECTANGLE_X_01012054: u32 = 0x0101_2054;
const RECTANGLE_WIDTH_010162C0: u32 = 0x0101_62c0;
const RECTANGLE_HEIGHT_0104A494: u32 = 0x0104_a494;

/// `NiPointer<T>::operator=(T *)` expanded inline on a field of the manager.
fn assign_pointer(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 4]).u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00b8c170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceManager::InitGeometry` (Xbox PDB), `thiscall`, no stack words. Makes an
/// `NiScreenElementsData` (0x70 bytes, constructed with 0, 0, 1, 1, 1, 4, 1, 2, 1; null when the
/// allocation fails) into the `NiPointer` at +0x1f4 and an `NiScreenElements` on it (0xc4
/// bytes) into the one at +0x1f0. On the elements' data (+0xb8 of the elements, read again
/// for each call): `Insert(4, 0, 0)`, `SetRectangle(0, x, 1.0, width, height)` with the
/// `float`s at `01012054`, `010162c0`, `0104a494`, `SetTextures(0, 0, 0.0, 0.0, 1.0, 1.0)` and
/// `UpdateBound`. Then names the elements `"Imagespace Effect Geometry"` (a name from
/// `00a5b690` passed by address to `00a5b950`, released afterwards unless it is the empty
/// word at `0109b220`), runs `UpdateProperties` and the update `00a59c60` with a stack block of
/// a 0.0 `float` and five clear bytes.
// Translation incomplete: 00b8c170 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the allocations and the name) is not translated.
pub fn image_space_manager_init_geometry(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let block = e.call(NI_ALLOCATE, &args![0x70u32]).u32();
    let data = if block != 0 {
        e.call(
            SCREEN_ELEMENTS_DATA_CONSTRUCT,
            &args![block, 0u32, 0u32, 1u32, 1u32, 1u32, 4u32, 1u32, 2u32, 1u32],
        )
        .u32()
    } else {
        0
    };
    assign_pointer(e, base + 0x1f4, data);
    let block = e.call(NI_ALLOCATE, &args![0xc4u32]).u32();
    let elements = if block != 0 {
        let data = e.mem.u32(base + 0x1f4);
        e.call(SCREEN_ELEMENTS_CONSTRUCT, &args![block, data]).u32()
    } else {
        0
    };
    assign_pointer(e, base + 0x1f0, elements);
    let elements_data = |e: &Engine| e.mem.u32(e.mem.u32(base + 0x1f0) + 0xb8);
    let data = elements_data(e);
    e.call(SCREEN_ELEMENTS_DATA_INSERT, &args![data, 4u32, 0u32, 0u32]);
    let height = x87_quiet_bits(e.global::<u32>(RECTANGLE_HEIGHT_0104A494));
    let data = elements_data(e);
    let width = x87_quiet_bits(e.global::<u32>(RECTANGLE_WIDTH_010162C0));
    let x = x87_quiet_bits(e.global::<u32>(RECTANGLE_X_01012054));
    e.call(
        SCREEN_ELEMENTS_DATA_SET_RECTANGLE,
        &args![data, 0u32, x, 1.0f32.to_bits(), width, height],
    );
    let data = elements_data(e);
    e.call(
        SCREEN_ELEMENTS_DATA_SET_TEXTURES,
        &args![
            data,
            0u32,
            0u32,
            0.0f32.to_bits(),
            0.0f32.to_bits(),
            1.0f32.to_bits(),
            1.0f32.to_bits()
        ],
    );
    let data = elements_data(e);
    e.call(SCREEN_ELEMENTS_DATA_UPDATE_BOUND, &args![data]);
    e.with_stack(0x10, |e, locals| {
        let name_slot = locals.addr();
        let update = name_slot + 4;
        let name = e
            .call(MAKE_FIXED_STRING_00A5B690, &args![GEOMETRY_NAME_010B04AC])
            .u32();
        e.mem.set_u32(name_slot, name);
        let elements = e.mem.u32(base + 0x1f0);
        e.call(SET_NAME_00A5B950, &args![elements, name_slot]);
        let name = e.mem.u32(name_slot);
        if name != e.global::<u32>(NULL_NAME_0109B220) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
        }
        let elements = e.mem.u32(base + 0x1f0);
        e.call(UPDATE_PROPERTIES, &args![elements]);
        let elements = e.mem.u32(base + 0x1f0);
        e.mem.set_f32(update, 0.0);
        for offset in 4..9u32 {
            e.mem.set_u8(update + offset, 0);
        }
        e.call(UPDATE_00A59C60, &args![elements, update]);
    });
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// `00ba3aa0` (unnamed, same unit; `thiscall` on the manager's effect array at +0x4, `RET 8`):
/// stores the word the second argument points at under the index.
const ARRAY_SET_00BA3AA0: u32 = 0x00ba_3aa0;
/// The constructor of the 0x60-byte objects made in pairs by `00b94520` (`00ba4640`, unnamed,
/// `thiscall`).
const CONSTRUCT_00BA4640: u32 = 0x00ba_4640;

/// The first objects of `00b94520`, in the exe's order: the size given to the memory manager
/// (`00aa3e40` on `011f6238`), the constructor (`thiscall`) and the array index.
const SET_UP_MANAGED: [(u32, u32, u32); 20] = [
    (0x68, 0x00c0_5680, 0xd),
    (0x68, 0x00bd_93d0, 0x2),
    (0x7c, 0x00c0_3810, 0x8),
    (0x70, 0x00ba_d500, 0x1),
    (0x7c, 0x00c0_5420, 0x0),
    (0xc8, 0x00c0_4d20, 0x3),
    (0x58, 0x00bb_8900, 0xa),
    (0x58, 0x00bc_0600, 0xb),
    (0xbc, 0x00bd_7120, 0x4),
    (0x78, 0x00c0_49c0, 0xc),
    (0x88, 0x00ba_4880, 0x7),
    (0xd4, 0x00b5_a590, 0x5),
    (0x74, 0x00b5_a410, 0x6),
    (0x58, 0x00ba_a160, 0xe),
    (0x64, 0x00c0_4600, 0xf),
    (0xbc, 0x00ba_15e0, 0x1f),
    (0x74, 0x00ba_02f0, 0x20),
    (0x2bc, 0x00b9_b830, 0x21),
    (0x68, 0x00c0_33c0, 0x9),
    (0x58, 0x00bb_9180, 0x1e),
];

/// The `NiMemObject`s of `00b94520` stored through the inline `SetAt`, in the exe's order: the
/// size given to `00aa13e0`, the constructor (`thiscall`) and the array index; the element is
/// the object +0x6c.
const SET_UP_NI: [(u32, u32, u32); 78] = [
    (0xd0, 0x00b8_efa0, 0x22),
    (0xd0, 0x00b8_f010, 0x23),
    (0xd0, 0x00b8_f660, 0x24),
    (0xd0, 0x00b8_f520, 0x25),
    (0xd0, 0x00b8_efa0, 0x26),
    (0xd0, 0x00b8_f1b0, 0x27),
    (0xd0, 0x00b8_f360, 0x28),
    (0xd0, 0x00b9_2180, 0x29),
    (0xd0, 0x00b8_d330, 0x2a),
    (0xd0, 0x00b8_d850, 0x2b),
    (0xd0, 0x00b8_fea0, 0x2c),
    (0xd0, 0x00b8_d5b0, 0x2d),
    (0xd0, 0x00b8_d700, 0x2e),
    (0xd4, 0x00b9_22d0, 0x2f),
    (0xd0, 0x00b8_ee20, 0x32),
    (0xd0, 0x00b8_f7e0, 0x33),
    (0xd0, 0x00b8_fd40, 0x34),
    (0xd0, 0x00b9_17c0, 0x30),
    (0xd0, 0x00b9_1900, 0x31),
    (0xd0, 0x00b9_1b80, 0x35),
    (0xd0, 0x00b8_f930, 0x36),
    (0xd0, 0x00b8_fa80, 0x37),
    (0xd0, 0x00b8_fbe0, 0x38),
    (0xd0, 0x00b8_bea0, 0x39),
    (0xd0, 0x00b8_fff0, 0x3a),
    (0xd0, 0x00b9_0190, 0x3b),
    (0xd0, 0x00b9_0330, 0x3c),
    (0xd0, 0x00b9_0520, 0x3d),
    (0xd0, 0x00b9_06b0, 0x3e),
    (0xd0, 0x00b9_0830, 0x3f),
    (0xd0, 0x00b9_09c0, 0x40),
    (0xd0, 0x00b9_0b50, 0x41),
    (0xd0, 0x00b9_1000, 0x43),
    (0xd0, 0x00b9_11a0, 0x42),
    (0xd0, 0x00b9_1340, 0x45),
    (0xd0, 0x00b9_14e0, 0x44),
    (0xd0, 0x00b9_0ce0, 0x46),
    (0xd0, 0x00b9_0e70, 0x47),
    (0xd0, 0x00b9_1680, 0x48),
    (0xd0, 0x00b8_bce0, 0x49),
    (0xd0, 0x00b9_2500, 0x4b),
    (0xd0, 0x00b9_1a40, 0x4c),
    (0xd0, 0x00b8_d990, 0x4e),
    (0xd0, 0x00b8_dae0, 0x4f),
    (0xd0, 0x00b8_dc30, 0x50),
    (0xd0, 0x00b8_dd80, 0x51),
    (0xd0, 0x00b8_ded0, 0x52),
    (0xd0, 0x00b8_e020, 0x53),
    (0xd0, 0x00b8_e170, 0x54),
    (0xd0, 0x00b8_e2c0, 0x55),
    (0xd0, 0x00b8_e460, 0x56),
    (0xd0, 0x00b8_e600, 0x57),
    (0xd0, 0x00b8_e7a0, 0x58),
    (0xd0, 0x00b8_e940, 0x59),
    (0xd0, 0x00b8_eae0, 0x5a),
    (0xd0, 0x00b8_ec80, 0x5b),
    (0xd0, 0x00b9_32d0, 0x5d),
    (0xd0, 0x00b9_3600, 0x5e),
    (0xd0, 0x00b9_3950, 0x5f),
    (0xd0, 0x00b9_3aa0, 0x60),
    (0xd0, 0x00b9_3bf0, 0x61),
    (0xd0, 0x00b9_3d40, 0x62),
    (0xd0, 0x00b9_3e90, 0x63),
    (0xd0, 0x00b9_3fe0, 0x64),
    (0xd0, 0x00b9_4130, 0x65),
    (0xd0, 0x00b9_4280, 0x66),
    (0xd0, 0x00b9_32d0, 0x67),
    (0xd0, 0x00b9_43d0, 0x68),
    (0xd0, 0x00b9_2690, 0x69),
    (0xd0, 0x00b9_27e0, 0x6a),
    (0xd0, 0x00b9_2930, 0x6b),
    (0xd0, 0x00b9_2a80, 0x6c),
    (0xd0, 0x00b9_2bd0, 0x6d),
    (0xd0, 0x00b9_2d90, 0x6e),
    (0xd0, 0x00b9_2ee0, 0x6f),
    (0xd0, 0x00b9_3030, 0x70),
    (0xd0, 0x00b9_3180, 0x71),
    (0xd0, 0x00b9_1cf0, 0x72),
];

/// The last two `NiMemObject`s of `00b94520` (0xd0 bytes each), stored through `00ba3aa0`: the
/// constructor and the array index.
const SET_UP_NI_ARRAY: [(u32, u32); 2] = [(0x00b9_1e40, 0x73), (0x00b8_c070, 0x4d)];

/// The 16-bit size (+0xe of the manager) and element count (+0x10) of the effect array whose
/// element block pointer is at +0x8.
const EFFECTS: u32 = 0x8;
const EFFECTS_SIZE: u32 = 0xe;
const EFFECTS_COUNT: u32 = 0x10;

/// The element count of the effect array, moved by a wrapping 16-bit step.
fn effects_count_add(e: &mut Engine, base: u32, step: u16) {
    let count = e.mem.u16(base + EFFECTS_COUNT);
    e.mem
        .set_u16(base + EFFECTS_COUNT, count.wrapping_add(step));
}

// Translated from 00b94520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`; `ImageSpaceManager::Initialize`
/// runs it first), `thiscall` on the manager, no stack words, no answer. Makes the image-space
/// effects and puts them in the effect array at +0x4 (element block +0x8, 16-bit size +0xe and
/// count +0x10):
///
/// - the 20 objects of `SET_UP_MANAGED`: each allocated from the memory manager, constructed
///   when the allocation is not null (else null), kept in a stack word and stored through
///   `00ba3aa0` with its index and the word's address;
/// - for k from 1 to 7, two 0x60-byte objects (constructor `00ba4640`, null when the
///   allocation fails, and called through the vtable even then): vtable +0x20 with k, then
///   +0x24 with 0 (the first) or 1 (the second), stored at index k + 0xf (the first) and
///   k + 0x16 (the second) by a short inline `SetAt` (an index at or past the size makes the
///   size index + 1, and that or an empty slot adds one to the count, without a null check);
/// - the 78 objects of `SET_UP_NI` (allocated through `00aa13e0`), each element (the object
///   +0x6c, or null) stored by the full inline `SetAt` (past the size: the size grows to the
///   index + 1 and a non-null element counts; else a non-null element into an empty slot adds
///   one and a null one over a full slot takes one away);
/// - the two of `SET_UP_NI_ARRAY`, their element (object +0x6c, or null) stored through
///   `00ba3aa0`;
///
/// then, for the elements at +0x88 to +0x1cc of the block (indices 0x22 to 0x73, the block
/// pointer read again for each), when set, calls vtable +0x120 of the object (element - 0x6c).
// Translation incomplete: 00b94520 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0x65 for each allocation) and the stack cookie are not translated.
pub fn fn_00b94520(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let array = base + 4;
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for (size, construct, index) in SET_UP_MANAGED {
            let object = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
            e.mem.set_u32(local, object);
            let made = if object != 0 {
                e.call(construct, &args![object]).u32()
            } else {
                0
            };
            e.mem.set_u32(local, made);
            e.call(ARRAY_SET_00BA3AA0, &args![array, index, local]);
        }
        for k in 1..=7u32 {
            for (flag, index) in [(0u32, k + 0xf), (1, k + 0x16)] {
                let object = e
                    .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x60u32])
                    .u32();
                let element = if object != 0 {
                    e.call(CONSTRUCT_00BA4640, &args![object]).u32()
                } else {
                    0
                };
                e.vcall(element, 0x20, &args![k]);
                e.vcall(element, 0x24, &args![flag]);
                let size = u32::from(e.mem.u16(base + EFFECTS_SIZE));
                if index >= size {
                    e.mem.set_u16(base + EFFECTS_SIZE, (index + 1) as u16);
                    effects_count_add(e, base, 1);
                } else {
                    let block = e.mem.u32(base + EFFECTS);
                    if e.mem.u32(block + 4 * index) == 0 {
                        effects_count_add(e, base, 1);
                    }
                }
                let block = e.mem.u32(base + EFFECTS);
                e.mem.set_u32(block + 4 * index, element);
            }
        }
        for (size, construct, index) in SET_UP_NI {
            let object = e.call(NI_ALLOCATE, &args![size]).u32();
            let made = if object != 0 {
                e.call(construct, &args![object]).u32()
            } else {
                0
            };
            let element = if made != 0 { made + 0x6c } else { 0 };
            if index as u16 >= e.mem.u16(base + EFFECTS_SIZE) {
                e.mem.set_u16(base + EFFECTS_SIZE, (index + 1) as u16);
                if element != 0 {
                    effects_count_add(e, base, 1);
                }
            } else if element != 0 {
                let block = e.mem.u32(base + EFFECTS);
                if e.mem.u32(block + 4 * index) == 0 {
                    effects_count_add(e, base, 1);
                }
            } else {
                let block = e.mem.u32(base + EFFECTS);
                if e.mem.u32(block + 4 * index) != 0 {
                    effects_count_add(e, base, 0xffff);
                }
            }
            let block = e.mem.u32(base + EFFECTS);
            e.mem.set_u32(block + 4 * index, element);
        }
        for (construct, index) in SET_UP_NI_ARRAY {
            let object = e.call(NI_ALLOCATE, &args![0xd0u32]).u32();
            let made = if object != 0 {
                e.call(construct, &args![object]).u32()
            } else {
                0
            };
            let element = if made != 0 { made + 0x6c } else { 0 };
            e.mem.set_u32(local, element);
            e.call(ARRAY_SET_00BA3AA0, &args![array, index, local]);
        }
    });
    for offset in (0x88..0x1d0u32).step_by(4) {
        let block = e.mem.u32(base + EFFECTS);
        let element = e.mem.u32(block + offset);
        if element != 0 {
            let effect = if element != 0 { element - 0x6c } else { 0 };
            e.vcall(effect, 0x120, &args![]);
        }
    }
}

/// The `ImageSpaceShader` base constructor of the image-space shaders (`00c04570`, unnamed in
/// the engine map; `thiscall`, no stack words).
const IMAGE_SPACE_SHADER_CONSTRUCT_00C04570: u32 = 0x00c0_4570;
/// The `NiTArray` resize `00a90180` (unnamed in the engine map; `thiscall` on the array,
/// `RET 4`: the new size).
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The texture array of an `ImageSpaceShader` (`pTextures`, Xbox PDB, at the
/// `ImageSpaceEffect` base +0x6c, +0x28 in it: +0x94; the same on PC).
const SHADER_TEXTURES: u32 = 0x94;
/// The `ImageSpaceEffect` base of an `ImageSpaceShader` (Xbox PDB, +0x6c).
const SHADER_EFFECT_BASE: u32 = 0x6c;

/// The body every plain image-space shader constructor below has, in the exe's order: the
/// base constructor `00c04570`, the class's vtable at +0, its `ImageSpaceEffect` vtable at
/// +0x6c, then the texture array (+0x94) resized to `textures` by `00a90180`. Answers `this`.
fn construct_image_space_shader(
    e: &mut Engine,
    this: Ptr,
    vtable: u32,
    effect_vtable: u32,
    textures: u32,
) -> Ptr {
    let base = this.addr();
    e.call(IMAGE_SPACE_SHADER_CONSTRUCT_00C04570, &args![this]);
    e.mem.set_u32(base, vtable);
    e.mem.set_u32(base + SHADER_EFFECT_BASE, effect_vtable);
    e.call(
        ARRAY_SET_SIZE_00A90180,
        &args![base + SHADER_TEXTURES, textures],
    );
    this
}

/// The object `00b922d0` keeps a reference to (`NiPointer` at +0xd0; the word at `011f9520`).
const OBJECT_011F9520: u32 = 0x011f_9520;

// Translated from 00b922d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of the 0xd4-byte `ImageSpaceShader` subclass whose object +0x6c `00b94520`
/// stores at index 0x2f of the effect array (`EFFECT_SHADER_TEXTUREMASK` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed). Runs the base
/// constructor `00c04570`, sets the vtables `010b5978` (+0) and `010b5954` (+0x6c), clears
/// the pointer at +0xd0, sizes the texture array (+0x94) to two (`00a90180`), then assigns
/// the object at `011f9520` to the `NiPointer` at +0xd0 (inline: nothing when it is already
/// there; else the old one's reference given up, deleted through its vtable slot +4 at zero,
/// and a reference taken on the new one when it is not null). Answers `this`.
// Translation incomplete: 00b922d0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the pointer) is not translated.
pub fn fn_00b922d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(IMAGE_SPACE_SHADER_CONSTRUCT_00C04570, &args![this]);
    e.mem.set_u32(base, 0x010b_5978);
    e.mem.set_u32(base + SHADER_EFFECT_BASE, 0x010b_5954);
    e.mem.set_u32(base + 0xd0, 0);
    e.call(
        ARRAY_SET_SIZE_00A90180,
        &args![base + SHADER_TEXTURES, 2u32],
    );
    let object = e.global::<u32>(OBJECT_011F9520);
    assign_pointer(e, base + 0xd0, object);
    this
}

// Translated from 00b8bce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x49 of the effect array (`EFFECT_SHADER_INTERFACE_VATS_SCANLINES_BLEND` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010affc0` (+0) and `010affa0` (+0x6c) and sizes the texture array (+0x94) to five
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8bce0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8bce0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010a_ffc0, 0x010a_ffa0, 5)
}

// Translated from 00b8bea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x39 of the effect array (`EFFECT_SHADER_SUNBEAMS` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b0198` (+0) and `010b0174` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8bea0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8bea0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_0198, 0x010b_0174, 2)
}

// Translated from 00b8c070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x4d of the effect array (`EFFECT_SHADER_TV` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b0338` (+0) and `010b0318` (+0x6c) and sizes the texture array (+0x94) to four
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8c070 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8c070(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_0338, 0x010b_0318, 4)
}

// Translated from 00b8d330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x2a of the effect array (`EFFECT_SHADER_VOLUMETRIC_FOG` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b0518` (+0) and `010b04f4` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8d330 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8d330(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_0518, 0x010b_04f4, 3)
}

// Translated from 00b8d5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x2d of the effect array (`EFFECT_SHADER_ALPHAMULT` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b06c0` (+0) and `010b06a0` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8d5b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8d5b0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_06c0, 0x010b_06a0, 2)
}

// Translated from 00b8d700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x2e of the effect array (`EFFECT_SHADER_BLEND` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b0860` (+0) and `010b083c` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8d700 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8d700(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_0860, 0x010b_083c, 3)
}

// Translated from 00b8d850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x2b of the effect array (`EFFECT_SHADER_BLUR` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b0a00` (+0) and `010b09dc` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8d850 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8d850(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_0a00, 0x010b_09dc, 2)
}

// Translated from 00b8d990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x4e of the effect array (`EFFECT_SHADER_BLUR3` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b0b98` (+0) and `010b0b78` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8d990 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8d990(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_0b98, 0x010b_0b78, 2)
}

// Translated from 00b8dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x4f of the effect array (`EFFECT_SHADER_BLUR5` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b0d40` (+0) and `010b0d1c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8dae0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8dae0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_0d40, 0x010b_0d1c, 2)
}

// Translated from 00b8dc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x50 of the effect array (`EFFECT_SHADER_BLUR7` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b0ec0` (+0) and `010b0e9c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8dc30 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8dc30(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_0ec0, 0x010b_0e9c, 2)
}

// Translated from 00b8dd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x51 of the effect array (`EFFECT_SHADER_BLUR9` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b1040` (+0) and `010b101c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8dd80 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8dd80(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_1040, 0x010b_101c, 2)
}

// Translated from 00b8ded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x52 of the effect array (`EFFECT_SHADER_BLUR11` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b11c0` (+0) and `010b119c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8ded0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8ded0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_11c0, 0x010b_119c, 2)
}

// Translated from 00b8e020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x53 of the effect array (`EFFECT_SHADER_BLUR13` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b1340` (+0) and `010b1320` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8e020 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8e020(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_1340, 0x010b_1320, 2)
}

// Translated from 00b8e170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x54 of the effect array (`EFFECT_SHADER_BLUR15` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b14c0` (+0) and `010b14a0` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8e170 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8e170(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_14c0, 0x010b_14a0, 2)
}

// Translated from 00b8e2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x55 of the effect array (`EFFECT_SHADER_BRIGHTPASS_BLUR3` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b1640` (+0) and `010b1620` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8e2c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8e2c0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_1640, 0x010b_1620, 3)
}

// Translated from 00b8e460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x56 of the effect array (`EFFECT_SHADER_BRIGHTPASS_BLUR5` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b17f8` (+0) and `010b17d8` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8e460 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8e460(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_17f8, 0x010b_17d8, 3)
}

// Translated from 00b8e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x57 of the effect array (`EFFECT_SHADER_BRIGHTPASS_BLUR7` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b1980` (+0) and `010b1960` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8e600 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8e600(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_1980, 0x010b_1960, 3)
}

// Translated from 00b8e7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x58 of the effect array (`EFFECT_SHADER_BRIGHTPASS_BLUR9` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b1b08` (+0) and `010b1ae8` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8e7a0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8e7a0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_1b08, 0x010b_1ae8, 3)
}

// Translated from 00b8e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x59 of the effect array (`EFFECT_SHADER_BRIGHTPASS_BLUR11` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b1c90` (+0) and `010b1c70` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8e940 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8e940(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_1c90, 0x010b_1c70, 3)
}

// Translated from 00b8eae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x5a of the effect array (`EFFECT_SHADER_BRIGHTPASS_BLUR13` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b1e20` (+0) and `010b1dfc` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8eae0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8eae0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_1e20, 0x010b_1dfc, 3)
}

// Translated from 00b8ec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x5b of the effect array (`EFFECT_SHADER_BRIGHTPASS_BLUR15` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b1fb0` (+0) and `010b1f8c` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8ec80 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8ec80(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_1fb0, 0x010b_1f8c, 3)
}

// Translated from 00b8ee20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x32 of the effect array (`EFFECT_SHADER_CINEMATIC` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b2140` (+0) and `010b211c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8ee20 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8ee20(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_2140, 0x010b_211c, 2)
}

// Translated from 00b8efa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// indices 0x22 and 0x26 of the effect array (`EFFECT_SHADER_COPY` and `EFFECT_SHADER_COPYWITHALPHA` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b2a98` (+0) and `010b2a74` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8efa0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8efa0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_2a98, 0x010b_2a74, 2)
}

// Translated from 00b8f010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x23 of the effect array (`EFFECT_SHADER_COPY_RENDER_TARGET1` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b2308` (+0) and `010b22e8` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8f010 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8f010(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_2308, 0x010b_22e8, 2)
}

// Translated from 00b8f1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x27 of the effect array (`EFFECT_SHADER_STENCILCOPY` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b2490` (+0) and `010b2470` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8f1b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8f1b0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_2490, 0x010b_2470, 2)
}

// Translated from 00b8f360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x28 of the effect array (`EFFECT_SHADER_STENCILCOPY_NOTEX_NODEPTH` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b2610` (+0) and `010b25f0` (+0x6c) and sizes the texture array (+0x94) to one
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8f360 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8f360(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_2610, 0x010b_25f0, 1)
}

// Translated from 00b8f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x25 of the effect array (`EFFECT_SHADER_NULL` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b27a8` (+0) and `010b2784` (+0x6c) and sizes the texture array (+0x94) to one
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8f520 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8f520(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_27a8, 0x010b_2784, 1)
}

// Translated from 00b8f660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x24 of the effect array (`EFFECT_SHADER_RESTORE_EDRAM` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b2920` (+0) and `010b28fc` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8f660 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8f660(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_2920, 0x010b_28fc, 3)
}

// Translated from 00b8f7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x33 of the effect array (`EFFECT_SHADER_DEPTH_OF_FIELD` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b2c10` (+0) and `010b2bec` (+0x6c) and sizes the texture array (+0x94) to four
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8f7e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8f7e0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_2c10, 0x010b_2bec, 4)
}

// Translated from 00b8f930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x36 of the effect array (`EFFECT_SHADER_RADIAL_BLUR` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b2db0` (+0) and `010b2d8c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8f930 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8f930(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_2db0, 0x010b_2d8c, 2)
}

// Translated from 00b8fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x37 of the effect array (`EFFECT_SHADER_RADIAL_BLUR_MED` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b2f50` (+0) and `010b2f30` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8fa80 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8fa80(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_2f50, 0x010b_2f30, 2)
}

// Translated from 00b8fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x38 of the effect array (`EFFECT_SHADER_RADIAL_BLUR_HIGH` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b30e0` (+0) and `010b30c0` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8fbe0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8fbe0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_30e0, 0x010b_30c0, 2)
}

// Translated from 00b8fd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x34 of the effect array (`EFFECT_SHADER_DEPTH_OF_FIELD_MASK` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b3268` (+0) and `010b3244` (+0x6c) and sizes the texture array (+0x94) to five
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8fd40 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8fd40(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_3268, 0x010b_3244, 5)
}

// Translated from 00b8fea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x2c of the effect array (`EFFECT_SHADER_DOUBLEVIS` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b33f0` (+0) and `010b33cc` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8fea0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8fea0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_33f0, 0x010b_33cc, 3)
}

// Translated from 00b8fff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x3a of the effect array (`EFFECT_SHADER_HDR_BLENDINSHADER` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b3598` (+0) and `010b3574` (+0x6c) and sizes the texture array (+0x94) to four
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b8fff0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b8fff0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_3598, 0x010b_3574, 4)
}

// Translated from 00b90190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x3b of the effect array (`EFFECT_SHADER_HDR_BLENDINSHADER_CINEMATIC` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b37a8` (+0) and `010b3788` (+0x6c) and sizes the texture array (+0x94) to four
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b90190 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b90190(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_37a8, 0x010b_3788, 4)
}

// Translated from 00b90330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x3c of the effect array (`EFFECT_SHADER_HDR_BLENDINSHADER_CINEMATIC_ALPHAMASK` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b3958` (+0) and `010b3934` (+0x6c) and sizes the texture array (+0x94) to four
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b90330 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b90330(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_3958, 0x010b_3934, 4)
}

// Translated from 00b90520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x3d of the effect array (`EFFECT_SHADER_HDR_BLUR` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b3af0` (+0) and `010b3ad0` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b90520 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b90520(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_3af0, 0x010b_3ad0, 2)
}

// Translated from 00b906b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x3e of the effect array (`EFFECT_SHADER_HDR_BRIGHTPASSFILTER` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b3c78` (+0) and `010b3c58` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b906b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b906b0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_3c78, 0x010b_3c58, 2)
}

// Translated from 00b90830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x3f of the effect array (`EFFECT_SHADER_HDR_DOWNSAMPLE16` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b3e18` (+0) and `010b3df4` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b90830 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b90830(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_3e18, 0x010b_3df4, 2)
}

// Translated from 00b909c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x40 of the effect array (`EFFECT_SHADER_HDR_DOWNSAMPLE9` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b3fa8` (+0) and `010b3f88` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b909c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b909c0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_3fa8, 0x010b_3f88, 2)
}

// Translated from 00b90b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x41 of the effect array (`EFFECT_SHADER_HDR_DOWNSAMPLE4` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b4138` (+0) and `010b4114` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b90b50 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b90b50(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_4138, 0x010b_4114, 2)
}

// Translated from 00b90ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x46 of the effect array (`EFFECT_SHADER_HDR_LIGHTADAPT` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b42c8` (+0) and `010b42a4` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b90ce0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b90ce0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_42c8, 0x010b_42a4, 3)
}

// Translated from 00b90e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x47 of the effect array (`EFFECT_SHADER_HDR_LUMCLAMP` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b4460` (+0) and `010b443c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b90e70 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b90e70(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_4460, 0x010b_443c, 2)
}

// Translated from 00b91000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x43 of the effect array (`EFFECT_SHADER_HDR_DOWNSAMPLE4_LIGHTADAPT` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b4600` (+0) and `010b45dc` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b91000 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b91000(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_4600, 0x010b_45dc, 3)
}

// Translated from 00b911a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x42 of the effect array (`EFFECT_SHADER_HDR_DOWNSAMPLE4_LUMCLAMP` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b47a0` (+0) and `010b477c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b911a0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b911a0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_47a0, 0x010b_477c, 2)
}

// Translated from 00b91340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x45 of the effect array (`EFFECT_SHADER_HDR_DOWNSAMPLE16_LIGHTADAPT` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b4938` (+0) and `010b4914` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b91340 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b91340(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_4938, 0x010b_4914, 3)
}

// Translated from 00b914e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x44 of the effect array (`EFFECT_SHADER_HDR_DOWNSAMPLE16_LUMCLAMP` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b4ad0` (+0) and `010b4aac` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b914e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b914e0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_4ad0, 0x010b_4aac, 2)
}

// Translated from 00b91680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x48 of the effect array (`EFFECT_SHADER_INTERFACE_SCANLINES_BLEND` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b4c68` (+0) and `010b4c48` (+0x6c) and sizes the texture array (+0x94) to five
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b91680 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b91680(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_4c68, 0x010b_4c48, 5)
}

// Translated from 00b917c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x30 of the effect array (`EFFECT_SHADER_MAP` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b4de8` (+0) and `010b4dc8` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b917c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b917c0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_4de8, 0x010b_4dc8, 2)
}

// Translated from 00b91900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x31 of the effect array (`EFFECT_SHADER_MENUBG` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b4f80` (+0) and `010b4f60` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b91900 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b91900(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_4f80, 0x010b_4f60, 3)
}

// Translated from 00b91a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x4c of the effect array (`EFFECT_SHADER_INTERFACE_MENUUNPACK` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b5140` (+0) and `010b511c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b91a40 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b91a40(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_5140, 0x010b_511c, 2)
}

// Translated from 00b91b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x35 of the effect array (`EFFECT_SHADER_MOTION_BLUR` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b52c0` (+0) and `010b52a0` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b91b80 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b91b80(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_52c0, 0x010b_52a0, 3)
}

// Translated from 00b91cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x72 of the effect array (`EFFECT_SHADER_NOISE_SCROLL_AND_BLEND` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b5480` (+0) and `010b5460` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b91cf0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b91cf0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_5480, 0x010b_5460, 2)
}

// Translated from 00b91e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x73 of the effect array (`EFFECT_SHADER_NOISE_NORMALMAP` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b5640` (+0) and `010b5620` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b91e40 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b91e40(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_5640, 0x010b_5620, 2)
}

// Translated from 00b92180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x29 of the effect array (`EFFECT_SHADER_REFRACTION` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b57d0` (+0) and `010b57b0` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b92180 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b92180(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_57d0, 0x010b_57b0, 3)
}

// Translated from 00b92500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x4b of the effect array (`EFFECT_SHADER_INTERFACE_VATS_SCAN` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b5b10` (+0) and `010b5af0` (+0x6c) and sizes the texture array (+0x94) to five
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b92500 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b92500(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_5b10, 0x010b_5af0, 5)
}

// Translated from 00b92690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x69 of the effect array (`EFFECT_SHADER_WATER_DISPLACEMENT_CLEAR_SIMULATION` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b5cb8` (+0) and `010b5c94` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b92690 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b92690(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_5cb8, 0x010b_5c94, 2)
}

// Translated from 00b927e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x6a of the effect array (`EFFECT_SHADER_WATER_DISPLACEMENT_TEX_OFFSET` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b5ea8` (+0) and `010b5e84` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b927e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b927e0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_5ea8, 0x010b_5e84, 2)
}

// Translated from 00b92930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x6b of the effect array (`EFFECT_SHADER_WATER_DISPLACEMENT_WADING_RIPPLE` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b6048` (+0) and `010b6028` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b92930 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b92930(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_6048, 0x010b_6028, 2)
}

// Translated from 00b92a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x6c of the effect array (`EFFECT_SHADER_WATER_DISPLACEMENT_RAIN_RIPPLE` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b61f0` (+0) and `010b61d0` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b92a80 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b92a80(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_61f0, 0x010b_61d0, 2)
}

// Translated from 00b92bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x6d of the effect array (`EFFECT_SHADER_WATER_DISPLACEMENT_WADING_HEIGHTMAP` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b6398` (+0) and `010b6374` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b92bd0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b92bd0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_6398, 0x010b_6374, 2)
}

// Translated from 00b92d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x6e of the effect array (`EFFECT_SHADER_WATER_DISPLACEMENT_RAIN_HEIGHTMAP` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b6548` (+0) and `010b6524` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b92d90 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b92d90(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_6548, 0x010b_6524, 2)
}

// Translated from 00b92ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x6f of the effect array (`EFFECT_SHADER_WATER_DISPLACEMENT_BLEND_HEIGHTMAPS` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b66f0` (+0) and `010b66cc` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b92ee0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b92ee0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_66f0, 0x010b_66cc, 3)
}

// Translated from 00b93030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x70 of the effect array (`EFFECT_SHADER_WATER_DISPLACEMENT_SMOOTH_HEIGHTMAP` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b68a0` (+0) and `010b687c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b93030 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b93030(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_68a0, 0x010b_687c, 2)
}

// Translated from 00b93180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x71 of the effect array (`EFFECT_SHADER_WATER_DISPLACEMENT_NORMALS` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b6a40` (+0) and `010b6a20` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b93180 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b93180(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_6a40, 0x010b_6a20, 2)
}

// Translated from 00b932d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// indices 0x5d and 0x67 of the effect array (`EFFECT_SHADER_WATER_FFT_WATER_SPECTRUM` and `EFFECT_SHADER_WATER_FFT_DISPLAY_WATER_SPECTRUM` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b6bd8` (+0) and `010b6bb8` (+0x6c) and sizes the texture array (+0x94) to three
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b932d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b932d0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_6bd8, 0x010b_6bb8, 3)
}

// Translated from 00b93600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x5e of the effect array (`EFFECT_SHADER_WATER_FFT_HORIZONTAL_BUTTERFLY` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b6d90` (+0) and `010b6d70` (+0x6c) and sizes the texture array (+0x94) to four
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b93600 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b93600(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_6d90, 0x010b_6d70, 4)
}

// Translated from 00b93950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x5f of the effect array (`EFFECT_SHADER_WATER_FFT_VERTICAL_BUTTERFLY` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b6f38` (+0) and `010b6f14` (+0x6c) and sizes the texture array (+0x94) to four
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b93950 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b93950(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_6f38, 0x010b_6f14, 4)
}

// Translated from 00b93aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x60 of the effect array (`EFFECT_SHADER_WATER_FFT_HORIZONTAL_SCRAMBLE` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b70d8` (+0) and `010b70b8` (+0x6c) and sizes the texture array (+0x94) to four
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b93aa0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b93aa0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_70d8, 0x010b_70b8, 4)
}

// Translated from 00b93bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x61 of the effect array (`EFFECT_SHADER_WATER_FFT_VERTICAL_SCRAMBLE` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b7278` (+0) and `010b7258` (+0x6c) and sizes the texture array (+0x94) to four
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b93bf0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b93bf0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_7278, 0x010b_7258, 4)
}

// Translated from 00b93d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x62 of the effect array (`EFFECT_SHADER_WATER_FFT_NORMALS` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b7418` (+0) and `010b73f4` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b93d40 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b93d40(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_7418, 0x010b_73f4, 2)
}

// Translated from 00b93e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x63 of the effect array (`EFFECT_SHADER_WATER_FFT_FILTER` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b7598` (+0) and `010b7578` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b93e90 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b93e90(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_7598, 0x010b_7578, 2)
}

// Translated from 00b93fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x64 of the effect array (`EFFECT_SHADER_WATER_FFT_DISPLAY_NORMALS` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b7720` (+0) and `010b7700` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b93fe0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b93fe0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_7720, 0x010b_7700, 2)
}

// Translated from 00b94130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x65 of the effect array (`EFFECT_SHADER_WATER_FFT_DISPLAY_HIGHW` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b78b8` (+0) and `010b7898` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b94130 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b94130(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_78b8, 0x010b_7898, 2)
}

// Translated from 00b94280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x66 of the effect array (`EFFECT_SHADER_WATER_FFT_DISPLAY_AMPLITUDE` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b7a50` (+0) and `010b7a2c` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b94280 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b94280(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_7a50, 0x010b_7a2c, 2)
}

// Translated from 00b943d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// constructor of an `ImageSpaceShader` subclass whose object +0x6c `00b94520` stores at
/// index 0x68 of the effect array (`EFFECT_SHADER_WATER_FFT_DISPLAY_WATER_HEIGHT` in the Xbox PDB's
/// `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed, it keeps
/// `ImageSpaceShader`'s `GetRTTI`). Runs the base constructor `00c04570`, sets the vtables
/// `010b7bf0` (+0) and `010b7bcc` (+0x6c) and sizes the texture array (+0x94) to two
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00b943d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b943d0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_image_space_shader(e, this, 0x010b_7bf0, 0x010b_7bcc, 2)
}

/// The set-up `00b8b270` (this unit, unnamed; `thiscall` on the object at +0x58, two stack
/// words: an index and the address of a byte).
const SET_UP_00B8B270: u32 = 0x00b8_b270;

// Translated from 00b8bc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: calls
/// `00b8b270` on +0x58 with the indices 0 to 3, each with the address of a stack byte (the high
/// byte of the word the function saved `ECX` in) set just before the call to 1, 1, 1 and 0.
pub fn fn_00b8bc20(e: &mut Engine, this: Ptr) {
    let target = this.addr() + 0x58;
    e.with_stack(4, |e, local| {
        let flag = local.addr() + 3;
        for (index, value) in [(0u32, 1u8), (1, 1), (2, 1), (3, 0)] {
            e.mem.set_u8(flag, value);
            e.call(SET_UP_00B8B270, &args![target, index, flag]);
        }
    });
}

// Translated from 00b8b270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`): the set-at of an `NiTArray` of
/// bytes (base +4, size +0xa, element count +0xc), `thiscall` with two stack words: the index
/// and the address of the byte to store. At or past the size the size becomes the index plus one
/// and a set byte raises the element count. Below it a set byte replacing a clear one raises the
/// count and a clear byte replacing a set one lowers it. The byte is then stored at the index.
pub fn fn_00b8b270(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    let base = this.addr();
    let byte = e.mem.u8(value.addr());
    let count = e.mem.u16(base + 0xc);
    if index >= u32::from(e.mem.u16(base + 0xa)) {
        e.mem.set_u16(base + 0xa, index.wrapping_add(1) as u16);
        if byte != 0 {
            e.mem.set_u16(base + 0xc, count.wrapping_add(1));
        }
    } else {
        let held = e.mem.u8(e.mem.u32(base + 4).wrapping_add(index));
        if byte != 0 {
            if held == 0 {
                e.mem.set_u16(base + 0xc, count.wrapping_add(1));
            }
        } else if held != 0 {
            e.mem.set_u16(base + 0xc, count.wrapping_add(0xffff));
        }
    }
    let elements = e.mem.u32(base + 4);
    let byte = e.mem.u8(value.addr());
    e.mem.set_u8(elements.wrapping_add(index), byte);
}

/// `ImageSpaceShader::~ImageSpaceShader` (engine map, `00c03a20`; `thiscall`, no stack words).
const IMAGE_SPACE_SHADER_DESTRUCT_00C03A20: u32 = 0x00c0_3a20;
/// The sized Ni free `00aa1460` (unnamed, `NiSystem`; `cdecl`: the block and its size).
const NI_SIZED_FREE_00AA1460: u32 = 0x00aa_1460;

/// The body of the image-space shaders' deleting destructors below, in the exe's order: the
/// class's vtable at +0, its `ImageSpaceEffect` vtable at +0x6c, `~ImageSpaceShader`
/// (`00c03a20`), then, when bit 0 of the flags is set, the 0xd0 bytes freed through `00aa1460`.
/// Answers `this`.
fn delete_image_space_shader(
    e: &mut Engine,
    this: Ptr,
    vtable: u32,
    effect_vtable: u32,
    flags: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, vtable);
    e.mem.set_u32(base + SHADER_EFFECT_BASE, effect_vtable);
    e.call(IMAGE_SPACE_SHADER_DESTRUCT_00C03A20, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_FREE_00AA1460, &args![this, 0xd0u32]);
    }
    this
}

// Translated from 00b91b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): a deleting destructor that sets no vtable (slot 0 of the vtables `010b0338`, the one
/// `00b8c070` constructs with, `010b5140` and `010bc8f8`): `~ImageSpaceShader` (`00c03a20`),
/// then, when bit 0 of the flags is set, the 0xd0 bytes freed through `00aa1460`. Answers
/// `this`.
pub fn fn_00b91b50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(IMAGE_SPACE_SHADER_DESTRUCT_00C03A20, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_FREE_00AA1460, &args![this, 0xd0u32]);
    }
    this
}

// Translated from 00b91b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// `00b91b50` (slot 0 of the vtables `010b0318` and `010b511c`): `SUB ECX,0x6c`, then a jump to
/// it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b91b40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b91b50(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b92450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// destructor of the shader `00b922d0` constructs. Sets its vtables `010b5978` (+0) and
/// `010b5954` (+0x6c), gives up the reference of the `NiPointer` at +0xd0 when it is set (the
/// imported `InterlockedDecrement` on its +4; at zero, its vtable slot +4), then runs
/// `~ImageSpaceShader` (`00c03a20`).
// Translation incomplete: 00b92450 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b92450(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010b_5978);
    e.mem.set_u32(base + SHADER_EFFECT_BASE, 0x010b_5954);
    let object = e.mem.u32(base + 0xd0);
    if object != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![object + 4])
            .u32();
        if left == 0 {
            e.vcall(object, 4, &args![]);
        }
    }
    e.call(IMAGE_SPACE_SHADER_DESTRUCT_00C03A20, &args![this]);
}

// Translated from 00b924d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b922d0` constructs (slot 0 of `010b5978`):
/// `00b92450`, then, when bit 0 of the flags is set, the 0xd4 bytes freed through `00aa1460`.
/// Answers `this`.
pub fn fn_00b924d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b92450(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_FREE_00AA1460, &args![this, 0xd4u32]);
    }
    this
}

// Translated from 00b92440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// `00b924d0` (slot 0 of the vtable `010b5954`): `SUB ECX,0x6c`, then a jump to it with the same
/// stack word (the flags). Answers what it answers.
pub fn fn_00b92440(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b924d0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

/// `ImageSpaceShaderVolumetricFog::ReturnVolumetricFogTexture` (engine map, `00b63d10`;
/// `thiscall`, no stack words), `ImageSpaceTexture::~ImageSpaceTexture` (engine map, `00ba3a00`;
/// `thiscall`), the memory manager's free `00aa4060` (`thiscall` on `011f6238`, the block) and
/// the effect array's set-at `00ba3aa0` (on the texture array +0x94: the index and the address
/// of the word to store).
const RETURN_VOLUMETRIC_FOG_TEXTURE_00B63D10: u32 = 0x00b6_3d10;
const IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
const MEMORY_FREE_00AA4060: u32 = 0x00aa_4060;

// Translated from 00b8d4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderVolumetricFog::~ImageSpaceShaderVolumetricFog` (Xbox PDB), `thiscall`, no
/// stack words. Sets the vtables `010b0518` (+0) and `010b04f4` (+0x6c), runs
/// `ReturnVolumetricFogTexture` (`00b63d10`); when the texture array's (+0x94) element block
/// (+0x98) holds a texture at index 2, destroys it (`~ImageSpaceTexture`, `00ba3a00`) and frees
/// it through the memory manager (`00aa4060` on `011f6238`) when it is not null, and stores null
/// at index 2 through `00ba3aa0`; then runs `~ImageSpaceShader` (`00c03a20`).
// Translation incomplete: 00b8d4d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn image_space_shader_volumetric_fog_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010b_0518);
    e.mem.set_u32(base + SHADER_EFFECT_BASE, 0x010b_04f4);
    e.call(RETURN_VOLUMETRIC_FOG_TEXTURE_00B63D10, &args![this]);
    let block = e.mem.u32(base + 0x98);
    if e.mem.u32(block + 8) != 0 {
        let texture = e.mem.u32(block + 8);
        if texture != 0 {
            e.call(IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00, &args![texture]);
            e.call(MEMORY_FREE_00AA4060, &args![MEMORY_MANAGER, texture]);
        }
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), 0);
            e.call(
                ARRAY_SET_00BA3AA0,
                &args![base + SHADER_TEXTURES, 2u32, local],
            );
        });
    }
    e.call(IMAGE_SPACE_SHADER_DESTRUCT_00C03A20, &args![this]);
}

// Translated from 00b8d580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the volumetric fog shader (slot 0 of `010b0518`):
/// `~ImageSpaceShaderVolumetricFog` (`00b8d4d0`), then, when bit 0 of the flags is set, the
/// 0xd0 bytes freed through `00aa1460`. Answers `this`.
pub fn fn_00b8d580(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    image_space_shader_volumetric_fog_destructor(e, this);
    if flags & 1 != 0 {
        e.call(NI_SIZED_FREE_00AA1460, &args![this, 0xd0u32]);
    }
    this
}

// Translated from 00b8d430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// `00b8d580` (slot 0 of the vtable `010b04f4`): `SUB ECX,0x6c`, then a jump to it with the same
/// stack word (the flags). Answers what it answers.
pub fn fn_00b8d430(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8d580(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8bda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes `8a 41 04 c3`), `thiscall`, no stack words:
/// answers the byte at +4 (`MOV AL,[ECX+4]`). It is slot +0x18 of the `ImageSpaceEffect`
/// vtables of the image-space shaders (`010affb8`, `010b018c`, ...), the slot of
/// `ImageSpaceEffect::IsActive` in the Xbox PDB, whose `bIsActive` is +4 of the effect.
pub fn fn_00b8bda0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 4)
}

// Translated from 00b8bd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8bce0` constructs (vtables `010affc0` and
/// `010affa0`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8bd50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010a_ffc0, 0x010a_ffa0, flags)
}

// Translated from 00b8bfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8bea0` constructs (vtables `010b0198` and
/// `010b0174`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8bfa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_0198, 0x010b_0174, flags)
}

// Translated from 00b8d6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8d5b0` constructs (vtables `010b06c0` and
/// `010b06a0`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8d6c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_06c0, 0x010b_06a0, flags)
}

// Translated from 00b8d810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8d700` constructs (vtables `010b0860` and
/// `010b083c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8d810(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_0860, 0x010b_083c, flags)
}

// Translated from 00b8d950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8d850` constructs (vtables `010b0a00` and
/// `010b09dc`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8d950(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_0a00, 0x010b_09dc, flags)
}

// Translated from 00b8daa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8d990` constructs (vtables `010b0b98` and
/// `010b0b78`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8daa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_0b98, 0x010b_0b78, flags)
}

// Translated from 00b8dbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8dae0` constructs (vtables `010b0d40` and
/// `010b0d1c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8dbf0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_0d40, 0x010b_0d1c, flags)
}

// Translated from 00b8dd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8dc30` constructs (vtables `010b0ec0` and
/// `010b0e9c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8dd40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_0ec0, 0x010b_0e9c, flags)
}

// Translated from 00b8de90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8dd80` constructs (vtables `010b1040` and
/// `010b101c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8de90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_1040, 0x010b_101c, flags)
}

// Translated from 00b8dfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8ded0` constructs (vtables `010b11c0` and
/// `010b119c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8dfe0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_11c0, 0x010b_119c, flags)
}

// Translated from 00b8e130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8e020` constructs (vtables `010b1340` and
/// `010b1320`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8e130(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_1340, 0x010b_1320, flags)
}

// Translated from 00b8e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8e170` constructs (vtables `010b14c0` and
/// `010b14a0`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8e280(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_14c0, 0x010b_14a0, flags)
}

// Translated from 00b8e420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8e2c0` constructs (vtables `010b1640` and
/// `010b1620`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8e420(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_1640, 0x010b_1620, flags)
}

// Translated from 00b8e5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8e460` constructs (vtables `010b17f8` and
/// `010b17d8`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8e5c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_17f8, 0x010b_17d8, flags)
}

// Translated from 00b8e760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8e600` constructs (vtables `010b1980` and
/// `010b1960`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8e760(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_1980, 0x010b_1960, flags)
}

// Translated from 00b8e900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8e7a0` constructs (vtables `010b1b08` and
/// `010b1ae8`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8e900(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_1b08, 0x010b_1ae8, flags)
}

// Translated from 00b8eaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8e940` constructs (vtables `010b1c90` and
/// `010b1c70`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8eaa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_1c90, 0x010b_1c70, flags)
}

// Translated from 00b8ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8eae0` constructs (vtables `010b1e20` and
/// `010b1dfc`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8ec40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_1e20, 0x010b_1dfc, flags)
}

// Translated from 00b8ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8ec80` constructs (vtables `010b1fb0` and
/// `010b1f8c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8ede0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_1fb0, 0x010b_1f8c, flags)
}

// Translated from 00b8ef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8ee20` constructs (vtables `010b2140` and
/// `010b211c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8ef60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_2140, 0x010b_211c, flags)
}

// Translated from 00b8f120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8f010` constructs (vtables `010b2308` and
/// `010b22e8`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8f120(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_2308, 0x010b_22e8, flags)
}

// Translated from 00b8f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8f1b0` constructs (vtables `010b2490` and
/// `010b2470`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8f2b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_2490, 0x010b_2470, flags)
}

// Translated from 00b8f470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8f360` constructs (vtables `010b2610` and
/// `010b25f0`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8f470(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_2610, 0x010b_25f0, flags)
}

// Translated from 00b8f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8f520` constructs (vtables `010b27a8` and
/// `010b2784`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8f620(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_27a8, 0x010b_2784, flags)
}

// Translated from 00b8f6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8f660` constructs (vtables `010b2920` and
/// `010b28fc`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8f6d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_2920, 0x010b_28fc, flags)
}

// Translated from 00b8f7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8efa0` constructs (vtables `010b2a98` and
/// `010b2a74`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8f7a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_2a98, 0x010b_2a74, flags)
}

// Translated from 00b8f8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8f7e0` constructs (vtables `010b2c10` and
/// `010b2bec`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8f8f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_2c10, 0x010b_2bec, flags)
}

// Translated from 00b8fa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8f930` constructs (vtables `010b2db0` and
/// `010b2d8c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8fa40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_2db0, 0x010b_2d8c, flags)
}

// Translated from 00b8fba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8fa80` constructs (vtables `010b2f50` and
/// `010b2f30`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8fba0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_2f50, 0x010b_2f30, flags)
}

// Translated from 00b8fd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8fbe0` constructs (vtables `010b30e0` and
/// `010b30c0`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8fd00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_30e0, 0x010b_30c0, flags)
}

// Translated from 00b8fe60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8fd40` constructs (vtables `010b3268` and
/// `010b3244`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8fe60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_3268, 0x010b_3244, flags)
}

// Translated from 00b8ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8fea0` constructs (vtables `010b33f0` and
/// `010b33cc`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b8ffb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_33f0, 0x010b_33cc, flags)
}

// Translated from 00b90150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b8fff0` constructs (vtables `010b3598` and
/// `010b3574`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b90150(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_3598, 0x010b_3574, flags)
}

// Translated from 00b902f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b90190` constructs (vtables `010b37a8` and
/// `010b3788`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b902f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_37a8, 0x010b_3788, flags)
}

// Translated from 00b904e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b90330` constructs (vtables `010b3958` and
/// `010b3934`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b904e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_3958, 0x010b_3934, flags)
}

// Translated from 00b90670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b90520` constructs (vtables `010b3af0` and
/// `010b3ad0`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b90670(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_3af0, 0x010b_3ad0, flags)
}

// Translated from 00b907f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b906b0` constructs (vtables `010b3c78` and
/// `010b3c58`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b907f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_3c78, 0x010b_3c58, flags)
}

// Translated from 00b90980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b90830` constructs (vtables `010b3e18` and
/// `010b3df4`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b90980(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_3e18, 0x010b_3df4, flags)
}

// Translated from 00b90b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b909c0` constructs (vtables `010b3fa8` and
/// `010b3f88`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b90b10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_3fa8, 0x010b_3f88, flags)
}

// Translated from 00b90ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b90b50` constructs (vtables `010b4138` and
/// `010b4114`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b90ca0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_4138, 0x010b_4114, flags)
}

// Translated from 00b90e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b90ce0` constructs (vtables `010b42c8` and
/// `010b42a4`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b90e30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_42c8, 0x010b_42a4, flags)
}

// Translated from 00b90fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b90e70` constructs (vtables `010b4460` and
/// `010b443c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b90fc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_4460, 0x010b_443c, flags)
}

// Translated from 00b91160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b91000` constructs (vtables `010b4600` and
/// `010b45dc`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b91160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_4600, 0x010b_45dc, flags)
}

// Translated from 00b91300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b911a0` constructs (vtables `010b47a0` and
/// `010b477c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b91300(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_47a0, 0x010b_477c, flags)
}

// Translated from 00b914a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b91340` constructs (vtables `010b4938` and
/// `010b4914`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b914a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_4938, 0x010b_4914, flags)
}

// Translated from 00b91640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b914e0` constructs (vtables `010b4ad0` and
/// `010b4aac`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b91640(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_4ad0, 0x010b_4aac, flags)
}

// Translated from 00b91780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b91680` constructs (vtables `010b4c68` and
/// `010b4c48`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b91780(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_4c68, 0x010b_4c48, flags)
}

// Translated from 00b918c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b917c0` constructs (vtables `010b4de8` and
/// `010b4dc8`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b918c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_4de8, 0x010b_4dc8, flags)
}

// Translated from 00b91a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b91900` constructs (vtables `010b4f80` and
/// `010b4f60`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b91a00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_4f80, 0x010b_4f60, flags)
}

// Translated from 00b91cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b91b80` constructs (vtables `010b52c0` and
/// `010b52a0`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b91cb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_52c0, 0x010b_52a0, flags)
}

// Translated from 00b91e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b91cf0` constructs (vtables `010b5480` and
/// `010b5460`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b91e00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_5480, 0x010b_5460, flags)
}

// Translated from 00b91f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b91e40` constructs (vtables `010b5640` and
/// `010b5620`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b91f50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_5640, 0x010b_5620, flags)
}

// Translated from 00b92290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b92180` constructs (vtables `010b57d0` and
/// `010b57b0`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b92290(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_57d0, 0x010b_57b0, flags)
}

// Translated from 00b92570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b92500` constructs (vtables `010b5b10` and
/// `010b5af0`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b92570(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_5b10, 0x010b_5af0, flags)
}

// Translated from 00b927a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b92690` constructs (vtables `010b5cb8` and
/// `010b5c94`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b927a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_5cb8, 0x010b_5c94, flags)
}

// Translated from 00b928f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b927e0` constructs (vtables `010b5ea8` and
/// `010b5e84`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b928f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_5ea8, 0x010b_5e84, flags)
}

// Translated from 00b92a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b92930` constructs (vtables `010b6048` and
/// `010b6028`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b92a40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_6048, 0x010b_6028, flags)
}

// Translated from 00b92b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b92a80` constructs (vtables `010b61f0` and
/// `010b61d0`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b92b90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_61f0, 0x010b_61d0, flags)
}

// Translated from 00b92ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b92bd0` constructs (vtables `010b6398` and
/// `010b6374`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b92ce0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_6398, 0x010b_6374, flags)
}

// Translated from 00b92ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b92d90` constructs (vtables `010b6548` and
/// `010b6524`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b92ea0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_6548, 0x010b_6524, flags)
}

// Translated from 00b92ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b92ee0` constructs (vtables `010b66f0` and
/// `010b66cc`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b92ff0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_66f0, 0x010b_66cc, flags)
}

// Translated from 00b93140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b93030` constructs (vtables `010b68a0` and
/// `010b687c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b93140(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_68a0, 0x010b_687c, flags)
}

// Translated from 00b93290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b93180` constructs (vtables `010b6a40` and
/// `010b6a20`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b93290(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_6a40, 0x010b_6a20, flags)
}

// Translated from 00b933e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b932d0` constructs (vtables `010b6bd8` and
/// `010b6bb8`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b933e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_6bd8, 0x010b_6bb8, flags)
}

// Translated from 00b93710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b93600` constructs (vtables `010b6d90` and
/// `010b6d70`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b93710(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_6d90, 0x010b_6d70, flags)
}

// Translated from 00b93a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b93950` constructs (vtables `010b6f38` and
/// `010b6f14`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b93a60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_6f38, 0x010b_6f14, flags)
}

// Translated from 00b93bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b93aa0` constructs (vtables `010b70d8` and
/// `010b70b8`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b93bb0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_70d8, 0x010b_70b8, flags)
}

// Translated from 00b93d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b93bf0` constructs (vtables `010b7278` and
/// `010b7258`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b93d00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_7278, 0x010b_7258, flags)
}

// Translated from 00b93e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b93d40` constructs (vtables `010b7418` and
/// `010b73f4`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b93e50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_7418, 0x010b_73f4, flags)
}

// Translated from 00b93fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b93e90` constructs (vtables `010b7598` and
/// `010b7578`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b93fa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_7598, 0x010b_7578, flags)
}

// Translated from 00b940f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b93fe0` constructs (vtables `010b7720` and
/// `010b7700`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b940f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_7720, 0x010b_7700, flags)
}

// Translated from 00b94240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b94130` constructs (vtables `010b78b8` and
/// `010b7898`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b94240(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_78b8, 0x010b_7898, flags)
}

// Translated from 00b94390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b94280` constructs (vtables `010b7a50` and
/// `010b7a2c`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b94390(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_7a50, 0x010b_7a2c, flags)
}

// Translated from 00b944e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// flags): the deleting destructor of the shader `00b943d0` constructs (vtables `010b7bf0` and
/// `010b7bcc`), see `delete_image_space_shader`. Answers `this`.
pub fn fn_00b944e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_image_space_shader(e, this, 0x010b_7bf0, 0x010b_7bcc, flags)
}

// Translated from 00b8bd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8bd50` (slot 0 of the vtable `010affa0`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8bd80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8bd50(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8bfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8bfa0` (slot 0 of the vtable `010b0174`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8bfd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8bfa0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8d6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8d6c0` (slot 0 of the vtable `010b06a0`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8d6f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8d6c0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8d810` (slot 0 of the vtable `010b083c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8d840(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8d810(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8d980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8d950` (slot 0 of the vtable `010b09dc`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8d980(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8d950(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8dad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8daa0` (slot 0 of the vtable `010b0b78`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8dad0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8daa0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8dc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8dbf0` (slot 0 of the vtable `010b0d1c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8dc20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8dbf0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8dd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8dd40` (slot 0 of the vtable `010b0e9c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8dd70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8dd40(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8dec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8de90` (slot 0 of the vtable `010b101c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8dec0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8de90(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8e010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8dfe0` (slot 0 of the vtable `010b119c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8e010(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8dfe0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8e160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8e130` (slot 0 of the vtable `010b1320`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8e160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8e130(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8e2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8e280` (slot 0 of the vtable `010b14a0`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8e2b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8e280(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8e450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8e420` (slot 0 of the vtable `010b1620`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8e450(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8e420(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8e5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8e5c0` (slot 0 of the vtable `010b17d8`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8e5f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8e5c0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8e790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8e760` (slot 0 of the vtable `010b1960`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8e790(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8e760(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8e930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8e900` (slot 0 of the vtable `010b1ae8`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8e930(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8e900(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8ead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8eaa0` (slot 0 of the vtable `010b1c70`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8ead0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8eaa0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8ec70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8ec40` (slot 0 of the vtable `010b1dfc`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8ec70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8ec40(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8ee10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8ede0` (slot 0 of the vtable `010b1f8c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8ee10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8ede0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8ef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8ef60` (slot 0 of the vtable `010b211c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8ef90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8ef60(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8f150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8f120` (slot 0 of the vtable `010b22e8`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8f150(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8f120(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8f2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8f2b0` (slot 0 of the vtable `010b2470`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8f2e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8f2b0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8f4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8f470` (slot 0 of the vtable `010b25f0`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8f4a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8f470(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8f650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8f620` (slot 0 of the vtable `010b2784`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8f650(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8f620(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8f6d0` (slot 0 of the vtable `010b28fc`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8f700(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8f6d0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8f7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8f7a0` (slot 0 of the vtable `010b2a74`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8f7d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8f7a0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8f920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8f8f0` (slot 0 of the vtable `010b2bec`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8f920(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8f8f0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8fa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8fa40` (slot 0 of the vtable `010b2d8c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8fa70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8fa40(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8fbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8fba0` (slot 0 of the vtable `010b2f30`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8fbd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8fba0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8fd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8fd00` (slot 0 of the vtable `010b30c0`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8fd30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8fd00(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8fe90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8fe60` (slot 0 of the vtable `010b3244`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8fe90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8fe60(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b8ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b8ffb0` (slot 0 of the vtable `010b33cc`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b8ffe0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b8ffb0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b90180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b90150` (slot 0 of the vtable `010b3574`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b90180(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b90150(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b90320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b902f0` (slot 0 of the vtable `010b3788`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b90320(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b902f0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b90510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b904e0` (slot 0 of the vtable `010b3934`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b90510(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b904e0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b906a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b90670` (slot 0 of the vtable `010b3ad0`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b906a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b90670(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b90820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b907f0` (slot 0 of the vtable `010b3c58`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b90820(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b907f0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b909b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b90980` (slot 0 of the vtable `010b3df4`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b909b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b90980(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b90b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b90b10` (slot 0 of the vtable `010b3f88`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b90b40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b90b10(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b90cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b90ca0` (slot 0 of the vtable `010b4114`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b90cd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b90ca0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b90e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b90e30` (slot 0 of the vtable `010b42a4`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b90e60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b90e30(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b90ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b90fc0` (slot 0 of the vtable `010b443c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b90ff0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b90fc0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b91190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b91160` (slot 0 of the vtable `010b45dc`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b91190(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b91160(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b91330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b91300` (slot 0 of the vtable `010b477c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b91330(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b91300(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b914d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b914a0` (slot 0 of the vtable `010b4914`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b914d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b914a0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b91670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b91640` (slot 0 of the vtable `010b4aac`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b91670(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b91640(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b917b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b91780` (slot 0 of the vtable `010b4c48`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b917b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b91780(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b918f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b918c0` (slot 0 of the vtable `010b4dc8`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b918f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b918c0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b91a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b91a00` (slot 0 of the vtable `010b4f60`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b91a30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b91a00(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b91ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b91cb0` (slot 0 of the vtable `010b52a0`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b91ce0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b91cb0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b91e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b91e00` (slot 0 of the vtable `010b5460`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b91e30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b91e00(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b91f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b91f50` (slot 0 of the vtable `010b5620`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b91f80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b91f50(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b922c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b92290` (slot 0 of the vtable `010b57b0`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b922c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b92290(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b925a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b92570` (slot 0 of the vtable `010b5af0`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b925a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b92570(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b927d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b927a0` (slot 0 of the vtable `010b5c94`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b927d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b927a0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b92920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b928f0` (slot 0 of the vtable `010b5e84`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b92920(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b928f0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b92a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b92a40` (slot 0 of the vtable `010b6028`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b92a70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b92a40(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b92bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b92b90` (slot 0 of the vtable `010b61d0`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b92bc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b92b90(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b92d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b92ce0` (slot 0 of the vtable `010b6374`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b92d10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b92ce0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b92ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b92ea0` (slot 0 of the vtable `010b6524`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b92ed0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b92ea0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b93020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b92ff0` (slot 0 of the vtable `010b66cc`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b93020(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b92ff0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b93170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b93140` (slot 0 of the vtable `010b687c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b93170(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b93140(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b932c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b93290` (slot 0 of the vtable `010b6a20`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b932c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b93290(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b93410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b933e0` (slot 0 of the vtable `010b6bb8`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b93410(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b933e0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b93740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b93710` (slot 0 of the vtable `010b6d70`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b93740(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b93710(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b93a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b93a60` (slot 0 of the vtable `010b6f14`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b93a90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b93a60(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b93be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b93bb0` (slot 0 of the vtable `010b70b8`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b93be0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b93bb0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b93d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b93d00` (slot 0 of the vtable `010b7258`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b93d30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b93d00(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b93e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b93e50` (slot 0 of the vtable `010b73f4`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b93e80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b93e50(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b93fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b93fa0` (slot 0 of the vtable `010b7578`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b93fd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b93fa0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b94120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b940f0` (slot 0 of the vtable `010b7700`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b94120(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b940f0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b94270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b94240` (slot 0 of the vtable `010b7898`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b94270(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b94240(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b943c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b94390` (slot 0 of the vtable `010b7a2c`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b943c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b94390(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

// Translated from 00b94510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes): the `ImageSpaceEffect` (+0x6c) entry of
/// the deleting destructor `00b944e0` (slot 0 of the vtable `010b7bcc`): `SUB ECX,0x6c`, then a
/// jump to it with the same stack word (the flags). Answers what it answers.
pub fn fn_00b94510(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b944e0(
        e,
        Ptr::new(this.addr().wrapping_sub(SHADER_EFFECT_BASE)),
        flags,
    )
}

use crate::units::bsshader::geometrydecalshader::shader_inline::pass_render_state;

/// `_memset` (`00ec61c0`, `cdecl`: block, byte, size).
const MEMSET_00EC61C0: u32 = 0x00ec_61c0;
/// The pixel shader profile `00b4f380` (unnamed, unit `bsshadermanager.cpp`; `cdecl`, one word)
/// the reloads below pass as their pixel profile, given 1.
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
/// The byte the shader reloads test to pick their pixel shader file and name (`011f94fc`):
/// while it is clear they take the plain file, else the `_TEST` one.
const TEST_SHADERS_011F94FC: u32 = 0x011f_94fc;

/// Stores of the stack frame `frame` the exe makes between two calls: each word at its offset.
fn put(e: &mut Engine, frame: u32, words: &[(u32, u32)]) {
    for &(offset, value) in words {
        e.mem.set_u32(frame + offset, value);
    }
}

/// `REP MOVSD` of `count` words from `from` to `to`, lowest first.
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for index in 0..count {
        let word = e.mem.u32(from + 4 * index);
        e.mem.set_u32(to + 4 * index, word);
    }
}

/// The render states two of the shader reloads (`00b8bdb0`, `00b925b0`) set after loading, on
/// the pass the `NiPointer` at +0x5c holds (each through `pass_render_state`: the pass's state
/// block at +0x30 made by `00e7f5d0` when null, then `00e7f430` with the state, the value and
/// 1): 0x1b = 1 on the pass read first, then 0x13 = 5 and 0x14 = 6 on the pass read again
/// (once for both).
fn scanline_render_states(e: &mut Engine, this: Ptr) {
    let pass = e.mem.u32(this.addr() + 0x5c);
    pass_render_state(e, pass, 0x1b, 1);
    let pass = e.mem.u32(this.addr() + 0x5c);
    pass_render_state(e, pass, 0x13, 5);
    pass_render_state(e, pass, 0x14, 6);
}

/// The three render states `00b8d460` and `00b92d20` set: 0x34 = 1, 0x38 = 3 and 0x39 = 1, each
/// on the pass the `NiPointer` at +0x5c holds, read again for each (`pass_render_state`).
fn blend_render_states(e: &mut Engine, this: Ptr) {
    for (state, value) in [(0x34u32, 1u32), (0x38, 3), (0x39, 1)] {
        let pass = e.mem.u32(this.addr() + 0x5c);
        pass_render_state(e, pass, state, value);
    }
}

// Translated from 00b8d460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: slot
/// +0x150 of the shader `00b8d330` constructs (vtable `010b0518`; the slot of
/// `ImageSpaceShader::SetState` in the Xbox PDB once the PC's eight-byte shift of the other
/// slots is counted, unconfirmed): sets the render states 0x34 = 1, 0x38 = 3 and 0x39 = 1 on the
/// pass at +0x5c (`blend_render_states`).
pub fn fn_00b8d460(e: &mut Engine, this: Ptr) {
    blend_render_states(e, this);
}

/// The byte `00b92d20` tests before setting its render states (`011ad832`).
const FLAG_011AD832: u32 = 0x011a_d832;

// Translated from 00b92d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: slot +0x150
/// of the shaders `00b92930` and `00b92bd0` construct (vtables `010b6048` and `010b6398`): when
/// the byte at `011ad832` is set, sets the render states 0x34 = 1, 0x38 = 3 and 0x39 = 1 on the
/// pass at +0x5c (`blend_render_states`); else nothing.
pub fn fn_00b92d20(e: &mut Engine, this: Ptr) {
    if e.global::<u8>(FLAG_011AD832) != 0 {
        blend_render_states(e, this);
    }
}

// Translated from 00b8bdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8bce0` constructs (vtable `010affc0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\scanlines_blend.p.hlsl"` (defines `"BLURSRC1"` (null), `"VATS"`
/// (null)) (each 0x4c bytes: a file and up to nine define pairs, the rest cleared through
/// `memset` in the exe's order), calls its vtable slot +0x13c with them, the name
/// `"IFVATSSCANBLEND"` and the profiles `"vs_1_1"` and `"ps_2_0"`, then slot +0x14c, then the
/// render states of `scanline_render_states`.
pub fn fn_00b8bdb0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0130),
                (0x4, 0x010b_0124),
                (0x8, 0),
                (0xc, 0x0106_3538),
                (0x10, 0),
                (0x14, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x18, 0u32, 0x34u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_0114u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
        scanline_render_states(e, this);
    });
}

// Translated from 00b8bf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8bea0` constructs (vtable `010b0198`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\sunbeams.p.hlsl"` (no defines) (each 0x4c bytes: a file and up
/// to nine define pairs, the rest cleared through `memset` in the exe's order), calls its
/// vtable slot +0x13c with them, the name `"Sunbeams"` and the profiles `"vs_1_1"` and
/// `"ps_2_a"`, then slot +0x14c.
pub fn fn_00b8bf10(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(e, f, &[(0x0, 0x010b_02f8), (0x4, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x8, 0u32, 0x44u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_02ecu32, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8c0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8c070` constructs (vtable `010b0338`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\tv.p.hlsl"` (no defines) (each 0x4c bytes: a file and up to nine
/// define pairs, the rest cleared through `memset` in the exe's order), calls its vtable slot
/// +0x13c with them, the name `"TV"` and the profiles `"vs_1_1"` and `"ps_2_0"`, then slot
/// +0x14c.
pub fn fn_00b8c0e0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(e, f, &[(0x0, 0x010b_0490), (0x4, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x8, 0u32, 0x44u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_048cu32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8d3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8d330` constructs (vtable `010b0518`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\VolumetricFogBlend.p.hlsl"` (no defines) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"VFOG"` and the profiles `"vs_1_1"` and
/// `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b8d3a0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(e, f, &[(0x0, 0x010b_0674), (0x4, 0), (0x8, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0xc, 0u32, 0x40u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_066cu32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8d620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8d5b0` constructs (vtable `010b06c0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\1x\p\copy.p.hlsl"` (defines `"ALPHAMULT"` = `""`) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"ALPHAMULT"` and the profiles `"vs_1_1"`
/// and `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b8d620(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0820),
                (0x4, 0x010b_0814),
                (0x8, 0x0101_1584),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_0814u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8d770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8d700` constructs (vtable `010b0860`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (defines `"TEX2"` =
/// `""`) and the pixel one `"imagespace\2x\p\blend.p.hlsl"` (no defines) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"BLEND"` and the profiles `"vs_1_1"` and
/// `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b8d770(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0158),
                (0x4, 0x010b_09d4),
                (0x8, 0x0101_1584),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        put(e, f, &[(0x4c, 0x010b_09b4), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f, f + 0x4c, 0x0109_ab38u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8d8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8d850` constructs (vtable `010b0a00`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\blur.p.hlsl"` (no defines) (each 0x4c bytes: a file and up to
/// nine define pairs, the rest cleared through `memset` in the exe's order), calls its vtable
/// slot +0x13c with them, the name `"BLUR"` and the profiles `"vs_1_1"` and `"ps_2_a"`, then
/// slot +0x14c.
pub fn fn_00b8d8c0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(e, f, &[(0x0, 0x010b_0b5c), (0x4, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x8, 0u32, 0x44u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_0b54u32, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8da00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8d990` constructs (vtable `010b0b98`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\blur2.p.hlsl"` (defines `"TEXTAP"` = `"3"`) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"BLUR3"` and the profiles `"vs_1_1"` and
/// `"ps_2_a"`, then slot +0x14c.
pub fn fn_00b8da00(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0cfc),
                (0x4, 0x010b_0cf4),
                (0x8, 0x0106_3f30),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_0cecu32, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8db50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8dae0` constructs (vtable `010b0d40`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\blur2.p.hlsl"` (defines `"TEXTAP"` = `"5"`) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"BLUR5"` and the profiles `"vs_1_1"` and
/// `"ps_2_a"`, then slot +0x14c.
pub fn fn_00b8db50(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0cfc),
                (0x4, 0x010b_0cf4),
                (0x8, 0x0106_3f18),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_0e94u32, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8dca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8dc30` constructs (vtable `010b0ec0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\blur2.p.hlsl"` (defines `"TEXTAP"` = `"7"`) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"BLUR7"` and the profiles `"vs_1_1"` and
/// `"ps_2_a"`, then slot +0x14c.
pub fn fn_00b8dca0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0cfc),
                (0x4, 0x010b_0cf4),
                (0x8, 0x0106_3f00),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_1014u32, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8ddf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8dd80` constructs (vtable `010b1040`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\blur2.p.hlsl"` (defines `"TEXTAP"` = `"9"`) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"BLUR9"` and the profiles `"vs_1_1"` and
/// `"ps_2_a"`, then slot +0x14c.
pub fn fn_00b8ddf0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0cfc),
                (0x4, 0x010b_0cf4),
                (0x8, 0x0106_3ee8),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_1194u32, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8ded0` constructs (vtable `010b11c0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\blur2.p.hlsl"` (defines `"TEXTAP"` = `"11"`) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"BLUR11"` and the profiles `"vs_1_1"` and
/// `"ps_2_a"`, then slot +0x14c.
pub fn fn_00b8df40(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0cfc),
                (0x4, 0x010b_0cf4),
                (0x8, 0x010b_131c),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_1314u32, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8e090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8e020` constructs (vtable `010b1340`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\blur2.p.hlsl"` (defines `"TEXTAP"` = `"13"`) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"BLUR13"` and the profiles `"vs_1_1"` and
/// `"ps_2_a"`, then slot +0x14c.
pub fn fn_00b8e090(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0cfc),
                (0x4, 0x010b_0cf4),
                (0x8, 0x010b_149c),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_1494u32, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8e170` constructs (vtable `010b14c0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\blur2.p.hlsl"` (defines `"TEXTAP"` = `"15"`) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"BLUR15"` and the profiles `"vs_1_1"` and
/// `"ps_2_a"`, then slot +0x14c.
pub fn fn_00b8e1e0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0cfc),
                (0x4, 0x010b_0cf4),
                (0x8, 0x010b_161c),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_1614u32, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8ee90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8ee20` constructs (vtable `010b2140`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\cinematic.p.hlsl"` (defines `"TINT"` = `""`, `"SATURATION"` =
/// `""`, `"BRIGHT"` = `""`, `"CONTRAST"` = `""`) (each 0x4c bytes: a file and up to nine define
/// pairs, the rest cleared through `memset` in the exe's order), calls its vtable slot +0x13c
/// with them, the name `"Cinematic"` and the profiles `"vs_1_1"` and `"ps_2_a"`, then slot
/// +0x14c.
pub fn fn_00b8ee90(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x0101_1584),
                (0x10, 0x0101_1584),
                (0x18, 0x0101_1584),
                (0x20, 0x0101_1584),
                (0x28, 0),
                (0x2c, 0),
                (0x30, 0),
                (0x34, 0),
                (0x38, 0),
                (0x3c, 0),
                (0x40, 0),
                (0x44, 0),
                (0x48, 0),
                (0x0, 0x010b_22bc),
                (0x4, 0x010b_22b4),
                (0xc, 0x010b_22a8),
                (0x14, 0x010b_22a0),
                (0x1c, 0x010b_2294),
                (0x24, 0),
            ],
        );
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x0105_ea48u32, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8f010` constructs (vtable `010b2308`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\1x\p\copy.p.hlsl"` (defines `"TWO_TARGETS"` = `""`) (each 0x4c bytes:
/// a file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"COPYRT1"` and the profiles `"vs_1_1"` and
/// `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b8f080(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0820),
                (0x4, 0x010b_2464),
                (0x8, 0x0101_1584),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_245cu32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8f220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8f1b0` constructs (vtable `010b2490`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\1x\p\copy.p.hlsl"` (no defines) (each 0x4c bytes: a file and up to
/// nine define pairs, the rest cleared through `memset` in the exe's order), calls its vtable
/// slot +0x13c with them, the name `"STENCILCOPY"` and the profiles `"vs_1_1"` and the pixel
/// profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b8f220(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(e, f, &[(0x0, 0x010b_0820), (0x4, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x8, 0u32, 0x44u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_25e4u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8f3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8f360` constructs (vtable `010b2610`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\1x\p\copy.p.hlsl"` (defines `"NOTEX"` = `""`) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"STENCILCOPYNOTEXNODEPTH"` and the
/// profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b8f3d0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0820),
                (0x4, 0x010b_277c),
                (0x8, 0x0101_1584),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_2764u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8f590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8f520` constructs (vtable `010b27a8`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (defines `"NOTEX"` =
/// `""`) and the pixel one null (no defines) (each 0x4c bytes: a file and up to nine define
/// pairs, the rest cleared through `memset` in the exe's order), calls its vtable slot +0x13c
/// with them, the name `"NULL"` and the profiles `"vs_1_1"` and null, then slot +0x14c.
pub fn fn_00b8f590(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0158),
                (0x4, 0x010b_277c),
                (0x8, 0x0101_1584),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        put(e, f, &[(0x4c, 0), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f, f + 0x4c, 0x0101_f7a8u32, 0x010a_93ecu32, 0x0000_0000u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8f710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8efa0` constructs (vtable `010b2a98`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\1x\p\copy.p.hlsl"` (no defines) (each 0x4c bytes: a file and up to
/// nine define pairs, the rest cleared through `memset` in the exe's order), calls its vtable
/// slot +0x13c with them, the name `"COPY"` and the profiles `"vs_1_1"` and the pixel profile
/// `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b8f710(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(e, f, &[(0x0, 0x010b_0820), (0x4, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x8, 0u32, 0x44u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_22e0u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8f7e0` constructs (vtable `010b2c10`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (defines `"TEX2"` =
/// `""`) and the pixel one `"imagespace\2x\p\DepthOfField.p.hlsl"` (no defines) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name `"DOF"` and the profiles `"vs_1_1"`
/// and `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b8f850(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0158),
                (0x4, 0x010b_09d4),
                (0x8, 0x0101_1584),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        put(e, f, &[(0x4c, 0x010b_2d68), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f, f + 0x4c, 0x010b_2d64u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8f930` constructs (vtable `010b2db0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (defines `"TEX2"` =
/// `""`) and the pixel one `"imagespace\2x\p\RadialBlur.p.hlsl"` (no defines) (each 0x4c bytes:
/// a file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"RBLUR"` and the profiles `"vs_1_1"` and
/// `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b8f9a0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0158),
                (0x4, 0x010b_09d4),
                (0x8, 0x0101_1584),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        put(e, f, &[(0x4c, 0x010b_2f0c), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f, f + 0x4c, 0x010b_2f04u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8faf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8fa80` constructs (vtable `010b2f50`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (defines `"TEX2"` =
/// `""`) and the pixel one `"imagespace\2x\p\RadialBlur.p.hlsl"` (defines `"NUM_STEPS"` =
/// `"6.0f"`) (each 0x4c bytes: a file and up to nine define pairs, the rest cleared through
/// `memset` in the exe's order), calls its vtable slot +0x13c with them, the name `"RBLURM"`
/// and the profiles `"vs_1_1"` and `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b8faf0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x4c, 0x010b_0158),
                (0x50, 0x010b_09d4),
                (0x54, 0x0101_1584),
                (0x58, 0),
                (0x5c, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x60, 0u32, 0x38u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_2f0c),
                (0x4, 0x010b_30b4),
                (0x8, 0x010b_30ac),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_30a4u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8fbe0` constructs (vtable `010b30e0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (defines `"TEX2"` =
/// `""`) and the pixel one `"imagespace\2x\p\RadialBlur.p.hlsl"` (defines `"NUM_STEPS"` =
/// `"10.0f"`) (each 0x4c bytes: a file and up to nine define pairs, the rest cleared through
/// `memset` in the exe's order), calls its vtable slot +0x13c with them, the name `"RBLURH"`
/// and the profiles `"vs_1_1"` and `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b8fc50(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x4c, 0x010b_0158),
                (0x50, 0x010b_09d4),
                (0x54, 0x0101_1584),
                (0x58, 0),
                (0x5c, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x60, 0u32, 0x38u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_2f0c),
                (0x4, 0x010b_30b4),
                (0x8, 0x010b_323c),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_3234u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8fd40` constructs (vtable `010b3268`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (defines `"TEX2"` =
/// `""`) and the pixel one `"imagespace\2x\p\DepthOfField.p.hlsl"` (defines `"MASK"` = `""`)
/// (each 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in
/// the exe's order), calls its vtable slot +0x13c with them, the name `"DOFMASK"` and the
/// profiles `"vs_1_1"` and `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b8fdb0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x4c, 0x010b_0158),
                (0x50, 0x010b_09d4),
                (0x54, 0x0101_1584),
                (0x58, 0),
                (0x5c, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x60, 0u32, 0x38u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_2d68),
                (0x4, 0x010b_33c4),
                (0x8, 0x0101_1584),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_33bcu32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8ff10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b8fea0` constructs (vtable `010b33f0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (defines `"TEX2"` =
/// `""`) and the pixel one `"imagespace\2x\p\doublevision.p.hlsl"` (no defines) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name `"DOUBLEVIS"` and the profiles
/// `"vs_1_1"` and `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b8ff10(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0158),
                (0x4, 0x010b_09d4),
                (0x8, 0x0101_1584),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        put(e, f, &[(0x4c, 0x010b_3550), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f, f + 0x4c, 0x010b_3544u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b916f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b91680` constructs (vtable `010b4c68`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\scanlines_blend.p.hlsl"` (defines `"BLURSRC1"` = `""`) (each
/// 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in the
/// exe's order), calls its vtable slot +0x13c with them, the name `"IFSCANBLEND"` and the
/// profiles `"vs_1_1"` and `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b916f0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[(0x0, 0x010b_0130), (0x4, 0x010b_0124), (0x8, 0x0101_1584)],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0xc, 0u32, 0x40u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_4dbcu32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b91830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b917c0` constructs (vtable `010b4de8`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\map_P20.p.hlsl"` (no defines) (each 0x4c bytes: a file and up to
/// nine define pairs, the rest cleared through `memset` in the exe's order), calls its vtable
/// slot +0x13c with them, the name `"MAP"` and the profiles `"vs_1_1"` and `"ps_2_0"`, then
/// slot +0x14c.
pub fn fn_00b91830(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(e, f, &[(0x0, 0x010b_4f40), (0x4, 0), (0x8, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0xc, 0u32, 0x40u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_4f3cu32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b91970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b91900` constructs (vtable `010b4f80`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\menuBG.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\1x\p\menuBG.p.hlsl"` (no defines) (each 0x4c bytes: a file and up to
/// nine define pairs, the rest cleared through `memset` in the exe's order), calls its vtable
/// slot +0x13c with them, the name `"MENUBG"` and the profiles `"vs_2_0"` and `"ps_2_0"`, then
/// slot +0x14c.
pub fn fn_00b91970(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_50fc), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(e, f, &[(0x0, 0x010b_50dc), (0x4, 0), (0x8, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0xc, 0u32, 0x40u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_50d4u32, 0x010a_93e4u32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b91ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b91a40` constructs (vtable `010b5140`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\1x\p\copy.p.hlsl"` (no defines) (each 0x4c bytes: a file and up to
/// nine define pairs, the rest cleared through `memset` in the exe's order), calls its vtable
/// slot +0x13c with them, the name `"COPYUNPACK"` and the profiles `"vs_1_1"` and the pixel
/// profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b91ab0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(e, f, &[(0x0, 0x010b_0820), (0x4, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x8, 0u32, 0x44u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_5294u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b91bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b91b80` constructs (vtable `010b52c0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (defines `"TEX2"` =
/// `""`) and the pixel one `"imagespace\2x\p\MotionBlur.p.hlsl"` (defines `"NUM_SAMPLES"` =
/// `"40.0f"`, `"NO_VELOCITY"` = `""`) (each 0x4c bytes: a file and up to nine define pairs, the
/// rest cleared through `memset` in the exe's order), calls its vtable slot +0x13c with them,
/// the name `"MOTBLUR"` and the profiles `"vs_1_1"` and `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b91bf0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x4c, 0x010b_0158),
                (0x50, 0x010b_09d4),
                (0x54, 0x0101_1584),
                (0x58, 0),
                (0x5c, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x60, 0u32, 0x38u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_543c),
                (0x4, 0x010b_5430),
                (0x8, 0x010b_5428),
                (0xc, 0x010b_541c),
                (0x10, 0x0101_1584),
                (0x14, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x18, 0u32, 0x34u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_5414u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b91d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b91cf0` constructs (vtable `010b5480`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\noise_P20.p.hlsl"` (defines `"SCROLL_AND_BLEND"` (null)) (each
/// 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in the
/// exe's order), calls its vtable slot +0x13c with them, the name `"NOISESCROLLANDBLEND"` and
/// the profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b91d60(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_55fc),
                (0x4, 0x010b_55e8),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_55d4u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b91eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b91e40` constructs (vtable `010b5640`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\noise_P20.p.hlsl"` (defines `"NORMALMAP"` (null)) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name `"NOISENORMALMAP"` and the profiles
/// `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b91eb0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_55fc),
                (0x4, 0x010b_57a4),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_5794u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b921f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b92180` constructs (vtable `010b57d0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (defines `"TEX2"` =
/// `""`) and the pixel one `"imagespace\2x\p\refraction_P20.p.hlsl"` (no defines) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name `"REFRACT"` and the profiles
/// `"vs_1_1"` and `"ps_2_0"`, then slot +0x14c.
pub fn fn_00b921f0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0158),
                (0x4, 0x010b_09d4),
                (0x8, 0x0101_1584),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        put(e, f, &[(0x4c, 0x010b_592c), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f, f + 0x4c, 0x010b_5924u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b92390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b922d0` constructs (vtable `010b5978`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\copy.v.hlsl"` (defines `"MASK"` =
/// `""`) and the pixel one `"imagespace\1x\p\copy.p.hlsl"` (defines `"MASK"` = `""`) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name `"TEXMASK"` and the profiles
/// `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b92390(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x4c, 0x010b_5ad4),
                (0x50, 0x010b_33c4),
                (0x54, 0x0101_1584),
                (0x58, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x3cu32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_0820),
                (0x4, 0x010b_33c4),
                (0x8, 0x0101_1584),
                (0xc, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_5accu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b925b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b92500` constructs (vtable `010b5b10`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"imagespace\2x\p\vats_scan.p.hlsl"` (no defines) (each 0x4c bytes: a file and up
/// to nine define pairs, the rest cleared through `memset` in the exe's order), calls its
/// vtable slot +0x13c with them, the name `"IFVATSSCAN"` and the profiles `"vs_1_1"` and
/// `"ps_2_0"`, then slot +0x14c, then the render states of `scanline_render_states`.
pub fn fn_00b925b0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(e, f, &[(0x0, 0x010b_5c70), (0x4, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x8, 0u32, 0x44u32]);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_5c64u32, 0x010a_93ecu32, 0x010a_93ccu32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
        scanline_render_states(e, this);
    });
}

// Translated from 00b92700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b92690` constructs (vtable `010b5cb8`, slot +0x120). Builds on its
/// stack the vertex shader description `"water\2_ab\v\displace.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\displace.p.hlsl"` (defines `"CLEAR_SIMULATION"` (null)) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name
/// `"WATERDISPLACEMENTCLEARSIMULATION"` and the profiles `"vs_1_1"` and the pixel profile
/// `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b92700(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_5e64), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[(0x0, 0x010b_5e44), (0x4, 0x010b_5e30), (0x8, 0), (0xc, 0)],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_5e0cu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b92850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b927e0` constructs (vtable `010b5ea8`, slot +0x120). Builds on its
/// stack the vertex shader description `"water\2_ab\v\displace.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\displace.p.hlsl"` (defines `"TEXCOORD_OFFSET"` (null)) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name `"WATERDISPLACEMENTTEXOFFSET"` and
/// the profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b92850(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_5e64), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[(0x0, 0x010b_5e44), (0x4, 0x010b_6018), (0x8, 0), (0xc, 0)],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_5ffcu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b929a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b92930` constructs (vtable `010b6048`, slot +0x120). Builds on its
/// stack the vertex shader description `"water\2_ab\v\displace.v.hlsl"` (defines
/// `"RIPPLE_MAKER_WADING"` (null)) and the pixel one `"water\2_ab\p\displace.p.hlsl"` (defines
/// `"RIPPLE_MAKER_WADING"` (null)) (each 0x4c bytes: a file and up to nine define pairs, the
/// rest cleared through `memset` in the exe's order), calls its vtable slot +0x13c with them,
/// the name `"WATERDISPLACEMENTWADINGRIPPLE"` and the profiles `"vs_1_1"` and the pixel profile
/// `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b929a0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x4c, 0x010b_5e64),
                (0x50, 0x010b_61bc),
                (0x54, 0),
                (0x58, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x3cu32]);
        put(
            e,
            f,
            &[(0x0, 0x010b_5e44), (0x4, 0x010b_61bc), (0x8, 0), (0xc, 0)],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_619cu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b92af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b92a80` constructs (vtable `010b61f0`, slot +0x120). Builds on its
/// stack the vertex shader description `"water\2_ab\v\displace.v.hlsl"` (defines
/// `"RIPPLE_MAKER_RAIN"` (null)) and the pixel one `"water\2_ab\p\displace.p.hlsl"` (defines
/// `"RIPPLE_MAKER_RAIN"` (null)) (each 0x4c bytes: a file and up to nine define pairs, the rest
/// cleared through `memset` in the exe's order), calls its vtable slot +0x13c with them, the
/// name `"WATERDISPLACEMENTRAINRIPPLE"` and the profiles `"vs_1_1"` and the pixel profile
/// `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b92af0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x4c, 0x010b_5e64),
                (0x50, 0x010b_6360),
                (0x54, 0),
                (0x58, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x3cu32]);
        put(
            e,
            f,
            &[(0x0, 0x010b_5e44), (0x4, 0x010b_6360), (0x8, 0), (0xc, 0)],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_6344u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b92c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b92bd0` constructs (vtable `010b6398`, slot +0x120). Builds on its
/// stack the vertex shader description `"water\2_ab\v\displace.v.hlsl"` (defines
/// `"HEIGHTMAP_WADING"` (null)) and the pixel one `"water\2_ab\p\displace.p.hlsl"` (defines
/// `"HEIGHTMAP_WADING"` (null)) (each 0x4c bytes: a file and up to nine define pairs, the rest
/// cleared through `memset` in the exe's order), calls its vtable slot +0x13c with them, the
/// name `"WATERDISPLACEMENTWADINGHEIGHTMAP"` and the profiles `"vs_1_1"` and the pixel profile
/// `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b92c40(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x4c, 0x010b_5e64),
                (0x50, 0x010b_6510),
                (0x54, 0),
                (0x58, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x3cu32]);
        put(
            e,
            f,
            &[(0x0, 0x010b_5e44), (0x4, 0x010b_6510), (0x8, 0), (0xc, 0)],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_64ecu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b92e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b92d90` constructs (vtable `010b6548`, slot +0x120). Builds on its
/// stack the vertex shader description `"water\2_ab\v\displace.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\displace.p.hlsl"` (defines `"HEIGHTMAP_RAIN"` (null)) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name `"WATERDISPLACEMENTRAINHEIGHTMAP"`
/// and the profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot
/// +0x14c.
pub fn fn_00b92e00(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_5e64), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[(0x0, 0x010b_5e44), (0x4, 0x010b_66bc), (0x8, 0), (0xc, 0)],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_669cu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b92f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b92ee0` constructs (vtable `010b66f0`, slot +0x120). Builds on its
/// stack the vertex shader description `"water\2_ab\v\displace.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\displace.p.hlsl"` (defines `"BLEND_HEIGHTMAPS"` (null)) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name
/// `"WATERDISPLACEMENTBLENDHEIGHTMAPS"` and the profiles `"vs_1_1"` and the pixel profile
/// `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b92f50(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_5e64), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[(0x0, 0x010b_5e44), (0x4, 0x010b_6868), (0x8, 0), (0xc, 0)],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_6844u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b930a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b93030` constructs (vtable `010b68a0`, slot +0x120). Builds on its
/// stack the vertex shader description `"water\2_ab\v\displace.v.hlsl"` (defines
/// `"HEIGHTMAP_SMOOTH"` (null)) and the pixel one `"water\2_ab\p\displace.p.hlsl"` (defines
/// `"HEIGHTMAP_SMOOTH"` (null)) (each 0x4c bytes: a file and up to nine define pairs, the rest
/// cleared through `memset` in the exe's order), calls its vtable slot +0x13c with them, the
/// name `"WATERSMOOTHHEIGHTMAP"` and the profiles `"vs_1_1"` and the pixel profile `00b4f380`
/// answers for 1, then slot +0x14c.
pub fn fn_00b930a0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x4c, 0x010b_5e64),
                (0x50, 0x010b_6a0c),
                (0x54, 0),
                (0x58, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x3cu32]);
        put(
            e,
            f,
            &[(0x0, 0x010b_5e44), (0x4, 0x010b_6a0c), (0x8, 0), (0xc, 0)],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_69f4u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b931f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b93180` constructs (vtable `010b6a40`, slot +0x120). Builds on its
/// stack the vertex shader description `"water\2_ab\v\displace.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\displace.p.hlsl"` (defines `"NORMALS"` (null)) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"WATERDISPLACEMENTNORMALS"` and the
/// profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b931f0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_5e64), (0x50, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x54, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[(0x0, 0x010b_5e44), (0x4, 0x010b_6bb0), (0x8, 0), (0xc, 0)],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_6b94u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b93340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b932d0` constructs (vtable `010b6bd8`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"WATER_SPECTRUM"` (null)) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name `"FFTWATERSPECTRUM"` and the
/// profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b93340(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_6d40),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_6d2cu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b93670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b93600` constructs (vtable `010b6d90`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"HORIZONTAL_BUTTERFLY"` (null)) (each
/// 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in the
/// exe's order), calls its vtable slot +0x13c with them, the name `"FFTHORIZONTALBUTTERFLY"`
/// and the profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot
/// +0x14c.
pub fn fn_00b93670(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_6efc),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_6ee4u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b939c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b93950` constructs (vtable `010b6f38`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"VERTICAL_BUTTERFLY"` (null)) (each
/// 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in the
/// exe's order), calls its vtable slot +0x13c with them, the name `"FFTVERTICALBUTTERFLY"` and
/// the profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b939c0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_70a4),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_708cu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b93b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b93aa0` constructs (vtable `010b70d8`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"HORIZONTAL_SCRAMBLE"` (null)) (each
/// 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in the
/// exe's order), calls its vtable slot +0x13c with them, the name `"FFTHORIZONTALSCRAMBLE"` and
/// the profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b93b10(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_7244),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_722cu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b93c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b93bf0` constructs (vtable `010b7278`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"VERTICAL_SCRAMBLE"` (null)) (each
/// 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in the
/// exe's order), calls its vtable slot +0x13c with them, the name `"FFTVERTICALSCRAMBLE"` and
/// the profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b93c60(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_73e0),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_73ccu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b93db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b93d40` constructs (vtable `010b7418`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"NORMALS"` (null)) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"FFTNORMALS"` and the profiles `"vs_1_1"`
/// and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b93db0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_6bb0),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_756cu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b93f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b93e90` constructs (vtable `010b7598`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"FILTER"` (null)) (each 0x4c bytes: a
/// file and up to nine define pairs, the rest cleared through `memset` in the exe's order),
/// calls its vtable slot +0x13c with them, the name `"FFTFILTER"` and the profiles `"vs_1_1"`
/// and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b93f00(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_76f8),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_76ecu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b94050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b93fe0` constructs (vtable `010b7720`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"DISPLAY_NORMALS"` (null)) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name `"FFTDISPLAYNORMALS"` and the
/// profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b94050(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_7888),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_7874u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b941a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b94130` constructs (vtable `010b78b8`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"DISPLAY_HIGHW"` (null)) (each 0x4c
/// bytes: a file and up to nine define pairs, the rest cleared through `memset` in the exe's
/// order), calls its vtable slot +0x13c with them, the name `"FFTDISPLAYHIGHW"` and the
/// profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b941a0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_7a1c),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_7a0cu32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b942f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b94280` constructs (vtable `010b7a50`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"DISPLAY_AMPLITUDE"` (null)) (each
/// 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in the
/// exe's order), calls its vtable slot +0x13c with them, the name `"FFTDISPLAYAMPLITUDE"` and
/// the profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b942f0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_7bb8),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_7ba4u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b94440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; read from its bytes), `thiscall`, no stack words: the shader
/// reload of the shader `00b943d0` constructs (vtable `010b7bf0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and the
/// pixel one `"water\2_ab\p\waterhmap.p.hlsl"` (defines `"DISPLAY_WATER_HEIGHT"` (null)) (each
/// 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in the
/// exe's order), calls its vtable slot +0x13c with them, the name `"FFTDISPLAYWATERHEIGHT"` and
/// the profiles `"vs_1_1"` and the pixel profile `00b4f380` answers for 1, then slot +0x14c.
pub fn fn_00b94440(e: &mut Engine, this: Ptr) {
    e.with_stack(0x98, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x4c, 0x010b_0158), (0x50, 0), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x40u32]);
        put(
            e,
            f,
            &[
                (0x0, 0x010b_6d50),
                (0x4, 0x010b_7d5c),
                (0x8, 0),
                (0xc, 0),
                (0x10, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x38u32]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x4c, f, 0x010b_7d44u32, 0x010a_93ecu32, profile],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8e330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderBrightPassBlur3::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: the
/// shader reload of the shader `00b8e2c0` constructs (vtable `010b1640`, slot +0x120). Builds
/// on its stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and
/// the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\blur2.p.hlsl"` (defines `"BRIGHTPASS"` = `""`, `"TEXTAP"` = `"3"`) with
/// the name `"BPBLUR3"`, else `"imagespace\2x\p\blur2_TEST.p.hlsl"` (defines `"BRIGHTPASS"` =
/// `""`, `"TEXTAP"` = `"3"`) with the name `"BPBLUR3T"` (each 0x4c bytes: a file and up to nine
/// define pairs, the rest cleared through `memset` in the exe's order). Copies the pixel
/// description to another stack block, calls its vtable slot +0x13c with the two, the name and
/// the profiles `"vs_1_1"` and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_bright_pass_blur3_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_0158), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_17cc),
                (0xc, 0x0101_1584),
                (0x10, 0x010b_0cf4),
                (0x14, 0x0106_3f30),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_0cfc)]);
            0x010b_1794u32
        } else {
            put(e, f, &[(0x4, 0x010b_17a8)]);
            0x010b_179cu32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8e4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderBrightPassBlur5::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: the
/// shader reload of the shader `00b8e460` constructs (vtable `010b17f8`, slot +0x120). Builds
/// on its stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and
/// the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\blur2.p.hlsl"` (defines `"BRIGHTPASS"` = `""`, `"TEXTAP"` = `"5"`) with
/// the name `"BPBLUR5"`, else `"imagespace\2x\p\blur2_TEST.p.hlsl"` (defines `"BRIGHTPASS"` =
/// `""`, `"TEXTAP"` = `"5"`) with the name `"BPBLUR5T"` (each 0x4c bytes: a file and up to nine
/// define pairs, the rest cleared through `memset` in the exe's order). Copies the pixel
/// description to another stack block, calls its vtable slot +0x13c with the two, the name and
/// the profiles `"vs_1_1"` and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_bright_pass_blur5_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_0158), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_17cc),
                (0xc, 0x0101_1584),
                (0x10, 0x010b_0cf4),
                (0x14, 0x0106_3f18),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_0cfc)]);
            0x010b_194cu32
        } else {
            put(e, f, &[(0x4, 0x010b_17a8)]);
            0x010b_1954u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8e670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderBrightPassBlur7::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: the
/// shader reload of the shader `00b8e600` constructs (vtable `010b1980`, slot +0x120). Builds
/// on its stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and
/// the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\blur2.p.hlsl"` (defines `"BRIGHTPASS"` = `""`, `"TEXTAP"` = `"7"`) with
/// the name `"BPBLUR7"`, else `"imagespace\2x\p\blur2_TEST.p.hlsl"` (defines `"BRIGHTPASS"` =
/// `""`, `"TEXTAP"` = `"7"`) with the name `"BPBLUR7T"` (each 0x4c bytes: a file and up to nine
/// define pairs, the rest cleared through `memset` in the exe's order). Copies the pixel
/// description to another stack block, calls its vtable slot +0x13c with the two, the name and
/// the profiles `"vs_1_1"` and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_bright_pass_blur7_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_0158), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_17cc),
                (0xc, 0x0101_1584),
                (0x10, 0x010b_0cf4),
                (0x14, 0x0106_3f00),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_0cfc)]);
            0x010b_1ad4u32
        } else {
            put(e, f, &[(0x4, 0x010b_17a8)]);
            0x010b_1adcu32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderBrightPassBlur9::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: the
/// shader reload of the shader `00b8e7a0` constructs (vtable `010b1b08`, slot +0x120). Builds
/// on its stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no defines) and
/// the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\blur2.p.hlsl"` (defines `"BRIGHTPASS"` = `""`, `"TEXTAP"` = `"9"`) with
/// the name `"BPBLUR9"`, else `"imagespace\2x\p\blur2_TEST.p.hlsl"` (defines `"BRIGHTPASS"` =
/// `""`, `"TEXTAP"` = `"9"`) with the name `"BPBLUR9T"` (each 0x4c bytes: a file and up to nine
/// define pairs, the rest cleared through `memset` in the exe's order). Copies the pixel
/// description to another stack block, calls its vtable slot +0x13c with the two, the name and
/// the profiles `"vs_1_1"` and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_bright_pass_blur9_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_0158), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_17cc),
                (0xc, 0x0101_1584),
                (0x10, 0x010b_0cf4),
                (0x14, 0x0106_3ee8),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_0cfc)]);
            0x010b_1c5cu32
        } else {
            put(e, f, &[(0x4, 0x010b_17a8)]);
            0x010b_1c64u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderBrightPassBlur11::ReloadShaders` (Xbox PDB), `thiscall`, no stack words:
/// the shader reload of the shader `00b8e940` constructs (vtable `010b1c90`, slot +0x120).
/// Builds on its stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no
/// defines) and the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\blur2.p.hlsl"` (defines `"BRIGHTPASS"` = `""`, `"TEXTAP"` = `"11"`) with
/// the name `"BPBLUR11"`, else `"imagespace\2x\p\blur2_TEST.p.hlsl"` (defines `"BRIGHTPASS"` =
/// `""`, `"TEXTAP"` = `"11"`) with the name `"BPBLUR11T"` (each 0x4c bytes: a file and up to
/// nine define pairs, the rest cleared through `memset` in the exe's order). Copies the pixel
/// description to another stack block, calls its vtable slot +0x13c with the two, the name and
/// the profiles `"vs_1_1"` and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_bright_pass_blur11_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_0158), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_17cc),
                (0xc, 0x0101_1584),
                (0x10, 0x010b_0cf4),
                (0x14, 0x010b_131c),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_0cfc)]);
            0x010b_1de4u32
        } else {
            put(e, f, &[(0x4, 0x010b_17a8)]);
            0x010b_1df0u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderBrightPassBlur13::ReloadShaders` (Xbox PDB), `thiscall`, no stack words:
/// the shader reload of the shader `00b8eae0` constructs (vtable `010b1e20`, slot +0x120).
/// Builds on its stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no
/// defines) and the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\blur2.p.hlsl"` (defines `"BRIGHTPASS"` = `""`, `"TEXTAP"` = `"13"`) with
/// the name `"BPBLUR13"`, else `"imagespace\2x\p\blur2_TEST.p.hlsl"` (defines `"BRIGHTPASS"` =
/// `""`, `"TEXTAP"` = `"13"`) with the name `"BPBLUR13T"` (each 0x4c bytes: a file and up to
/// nine define pairs, the rest cleared through `memset` in the exe's order). Copies the pixel
/// description to another stack block, calls its vtable slot +0x13c with the two, the name and
/// the profiles `"vs_1_1"` and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_bright_pass_blur13_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_0158), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_17cc),
                (0xc, 0x0101_1584),
                (0x10, 0x010b_0cf4),
                (0x14, 0x010b_149c),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_0cfc)]);
            0x010b_1f74u32
        } else {
            put(e, f, &[(0x4, 0x010b_17a8)]);
            0x010b_1f80u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b8ecf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderBrightPassBlur15::ReloadShaders` (Xbox PDB), `thiscall`, no stack words:
/// the shader reload of the shader `00b8ec80` constructs (vtable `010b1fb0`, slot +0x120).
/// Builds on its stack the vertex shader description `"imagespace\1x\v\base.v.hlsl"` (no
/// defines) and the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\blur2.p.hlsl"` (defines `"BRIGHTPASS"` = `""`, `"TEXTAP"` = `"15"`) with
/// the name `"BPBLUR15"`, else `"imagespace\2x\p\blur2_TEST.p.hlsl"` (defines `"BRIGHTPASS"` =
/// `""`, `"TEXTAP"` = `"15"`) with the name `"BPBLUR15T"` (each 0x4c bytes: a file and up to
/// nine define pairs, the rest cleared through `memset` in the exe's order). Copies the pixel
/// description to another stack block, calls its vtable slot +0x13c with the two, the name and
/// the profiles `"vs_1_1"` and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_bright_pass_blur15_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_0158), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_17cc),
                (0xc, 0x0101_1584),
                (0x10, 0x010b_0cf4),
                (0x14, 0x010b_161c),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_0cfc)]);
            0x010b_2104u32
        } else {
            put(e, f, &[(0x4, 0x010b_17a8)]);
            0x010b_2110u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b90060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRBlendInShader::ReloadShaders` (Xbox PDB), `thiscall`, no stack words:
/// the shader reload of the shader `00b8fff0` constructs (vtable `010b3598`, slot +0x120).
/// Builds on its stack the vertex shader description `"imagespace\2x\v\HDRbase.v.hlsl"`
/// (defines `"TEX2"` (null)) and the pixel one, which depends on the byte at `011f94fc`: while
/// it is clear `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"SHBLEND"` (null), `"TONEMAP"`
/// (null)) with the name `"HDRBLENDINSHADER"`, else `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"`
/// (defines `"SHBLEND"` (null), `"TONEMAP"` (null)) with the name `"HDRBLENDINSHADERT"` (each
/// 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in the
/// exe's order). Copies the pixel description to another stack block, calls its vtable slot
/// +0x13c with the two, the name and the profiles `"vs_1_1"` and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_hdr_blend_in_shader_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf0, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x50, 0x010b_3768),
                (0x54, 0x010b_09d4),
                (0x58, 0),
                (0x5c, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x60, 0u32, 0x3cu32]);
        put(
            e,
            f,
            &[
                (0x4, 0x010b_3760),
                (0x8, 0),
                (0xc, 0x010b_3758),
                (0x10, 0),
                (0x14, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x0, 0x010b_3700)]);
            0x010b_36ecu32
        } else {
            put(e, f, &[(0x0, 0x010b_3734)]);
            0x010b_3720u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x18, 0u32, 0x34u32]);
        copy_words(e, f, f + 0xa0, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x50, f + 0xa0, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b90200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRBlendInShaderCinematic::ReloadShaders` (Xbox PDB), `thiscall`, no stack
/// words: the shader reload of the shader `00b90190` constructs (vtable `010b37a8`, slot
/// +0x120). Builds on its stack the vertex shader description
/// `"imagespace\2x\v\HDRbase.v.hlsl"` (defines `"TEX2"` (null)) and the pixel one, which
/// depends on the byte at `011f94fc`: while it is clear `"imagespace\2x\p\HDRBlur.p.hlsl"`
/// (defines `"CINEMATIC"` (null), `"SHBLEND"` (null), `"TONEMAP"` (null)) with the name
/// `"HDRBLENDINSHADERCIN"`, else `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"CINEMATIC"`
/// (null), `"SHBLEND"` (null), `"TONEMAP"` (null)) with the name `"HDRBLENDINSHADERCINT"` (each
/// 0x4c bytes: a file and up to nine define pairs, the rest cleared through `memset` in the
/// exe's order). Copies the pixel description to another stack block, calls its vtable slot
/// +0x13c with the two, the name and the profiles `"vs_1_1"` and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_hdr_blend_in_shader_cinematic_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf0, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x50, 0x010b_3768),
                (0x54, 0x010b_09d4),
                (0x58, 0),
                (0x5c, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x60, 0u32, 0x3cu32]);
        put(
            e,
            f,
            &[
                (0x4, 0x010b_3928),
                (0x8, 0),
                (0xc, 0x010b_3760),
                (0x10, 0),
                (0x14, 0x010b_3758),
                (0x18, 0),
                (0x1c, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x0, 0x010b_3700)]);
            0x010b_38fcu32
        } else {
            put(e, f, &[(0x0, 0x010b_3734)]);
            0x010b_3910u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x20, 0u32, 0x2cu32]);
        copy_words(e, f, f + 0xa0, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x50, f + 0xa0, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b903a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRBlendInShaderCinematicAlphamask::ReloadShaders` (Xbox PDB), `thiscall`,
/// no stack words: the shader reload of the shader `00b90330` constructs (vtable `010b3958`,
/// slot +0x120). Builds on its stack the vertex shader description
/// `"imagespace\2x\v\HDRbase.v.hlsl"` (defines `"TEX2"` (null)) and the pixel one, which
/// depends on the byte at `011f94fc`: while it is clear `"imagespace\2x\p\HDRBlur.p.hlsl"`
/// (defines `"ALPHAMASK"` (null), `"CINEMATIC"` (null), `"SHBLEND"` (null), `"TONEMAP"` (null))
/// with the name `"HDRBLENDINSHADERCINAM"`, else `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"`
/// (defines `"CINEMATIC"` (null), `"SHBLEND"` (null), `"TONEMAP"` (null)) with the name
/// `"HDRBLENDINSHADERCINT"` (each 0x4c bytes: a file and up to nine define pairs, the rest
/// cleared through `memset` in the exe's order). Copies the pixel description to another stack
/// block, calls its vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"` and
/// `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_hdr_blend_in_shader_cinematic_alphamask_reload_shaders(
    e: &mut Engine,
    this: Ptr,
) {
    e.with_stack(0xf0, |e, frame| {
        let f = frame.addr();
        put(
            e,
            f,
            &[
                (0x50, 0x010b_3768),
                (0x54, 0x010b_09d4),
                (0x58, 0),
                (0x5c, 0),
            ],
        );
        e.call(MEMSET_00EC61C0, &args![f + 0x60, 0u32, 0x3cu32]);
        put(e, f, &[(0x8, 0), (0x10, 0), (0x18, 0)]);
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(
                e,
                f,
                &[
                    (0x28, 0),
                    (0x2c, 0),
                    (0x30, 0),
                    (0x34, 0),
                    (0x38, 0),
                    (0x3c, 0),
                    (0x40, 0),
                    (0x44, 0),
                    (0x48, 0),
                    (0x0, 0x010b_3700),
                    (0x4, 0x010b_3ac4),
                    (0xc, 0x010b_3928),
                    (0x14, 0x010b_3760),
                    (0x1c, 0x010b_3758),
                    (0x20, 0),
                    (0x24, 0),
                ],
            );
            0x010b_3aacu32
        } else {
            put(
                e,
                f,
                &[
                    (0x0, 0x010b_3734),
                    (0x4, 0x010b_3928),
                    (0xc, 0x010b_3760),
                    (0x14, 0x010b_3758),
                    (0x1c, 0),
                ],
            );
            e.call(MEMSET_00EC61C0, &args![f + 0x20, 0u32, 0x2cu32]);
            0x010b_3910u32
        };
        copy_words(e, f, f + 0xa0, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x50, f + 0xa0, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b90590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRBlur::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: the shader
/// reload of the shader `00b90520` constructs (vtable `010b3af0`, slot +0x120). Builds on its
/// stack the vertex shader description `"imagespace\2x\v\HDRbase.v.hlsl"` (no defines) and the
/// pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"TEXTAP"` = `"15"`) with the name `"HDRBLUR"`,
/// else `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"TEXTAP"` = `"15"`) with the name
/// `"HDRBLURT"` (each 0x4c bytes: a file and up to nine define pairs, the rest cleared through
/// `memset` in the exe's order). Copies the pixel description to another stack block, calls its
/// vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"` and `"ps_2_a"`, then
/// slot +0x14c.
pub fn image_space_shader_hdr_blur_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_3768), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(e, f, &[(0x8, 0x010b_0cf4), (0xc, 0x010b_161c), (0x10, 0)]);
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_3700)]);
            0x010b_3c44u32
        } else {
            put(e, f, &[(0x4, 0x010b_3734)]);
            0x010b_3c4cu32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x3cu32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b90720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRBrightPassFilter::ReloadShaders` (Xbox PDB), `thiscall`, no stack words:
/// the shader reload of the shader `00b906b0` constructs (vtable `010b3c78`, slot +0x120).
/// Builds on its stack the vertex shader description `"imagespace\2x\v\HDRbase.v.hlsl"` (no
/// defines) and the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"BRIGHTFILTER"` (null)) with the name
/// `"HDRBRIGHT"`, else `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"BRIGHTFILTER"`
/// (null)) with the name `"HDRBRIGHTT"` (each 0x4c bytes: a file and up to nine define pairs,
/// the rest cleared through `memset` in the exe's order). Copies the pixel description to
/// another stack block, calls its vtable slot +0x13c with the two, the name and the profiles
/// `"vs_1_1"` and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_hdr_bright_pass_filter_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf0, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x50, 0x010b_3768), (0x54, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x58, 0u32, 0x44u32]);
        put(e, f, &[(0x4, 0x010b_3de4), (0x8, 0), (0xc, 0)]);
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x0, 0x010b_3700)]);
            0x010b_3dccu32
        } else {
            put(e, f, &[(0x0, 0x010b_3734)]);
            0x010b_3dd8u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x10, 0u32, 0x3cu32]);
        copy_words(e, f, f + 0xa0, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x50, f + 0xa0, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b908a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRDownSample16::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: the
/// shader reload of the shader `00b90830` constructs (vtable `010b3e18`, slot +0x120). Builds
/// on its stack the vertex shader description `"imagespace\2x\v\HDRbase.v.hlsl"` (no defines)
/// and the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"TEXTAP"` = `"16"`) with the name
/// `"HDRDOWN16"`, else `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"TEXTAP"` = `"16"`)
/// with the name `"HDRDOWN16T"` (each 0x4c bytes: a file and up to nine define pairs, the rest
/// cleared through `memset` in the exe's order). Copies the pixel description to another stack
/// block, calls its vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"` and
/// `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_hdr_down_sample16_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_3768), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(e, f, &[(0x8, 0x010b_0cf4), (0xc, 0x010b_3f84), (0x10, 0)]);
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_3700)]);
            0x010b_3f6cu32
        } else {
            put(e, f, &[(0x4, 0x010b_3734)]);
            0x010b_3f78u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x3cu32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b90a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRDownSample9::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: the
/// shader reload of the shader `00b909c0` constructs (vtable `010b3fa8`, slot +0x120). Builds
/// on its stack the vertex shader description `"imagespace\2x\v\HDRbase.v.hlsl"` (no defines)
/// and the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"TEXTAP"` = `"9"`) with the name `"HDRDOWN9"`,
/// else `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"TEXTAP"` = `"9"`) with the name
/// `"HDRDOWN9T"` (each 0x4c bytes: a file and up to nine define pairs, the rest cleared through
/// `memset` in the exe's order). Copies the pixel description to another stack block, calls its
/// vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"` and `"ps_2_a"`, then
/// slot +0x14c.
pub fn image_space_shader_hdr_down_sample9_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_3768), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(e, f, &[(0x8, 0x010b_0cf4), (0xc, 0x0106_3ee8), (0x10, 0)]);
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_3700)]);
            0x010b_40fcu32
        } else {
            put(e, f, &[(0x4, 0x010b_3734)]);
            0x010b_4108u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x3cu32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b90bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRDownSample4::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: the
/// shader reload of the shader `00b90b50` constructs (vtable `010b4138`, slot +0x120). Builds
/// on its stack the vertex shader description `"imagespace\2x\v\HDRbase.v.hlsl"` (no defines)
/// and the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"TEXTAP"` = `"4"`) with the name `"HDRDOWN4"`,
/// else `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"TEXTAP"` = `"4"`) with the name
/// `"HDRDOWN4T"` (each 0x4c bytes: a file and up to nine define pairs, the rest cleared through
/// `memset` in the exe's order). Copies the pixel description to another stack block, calls its
/// vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"` and `"ps_2_a"`, then
/// slot +0x14c.
pub fn image_space_shader_hdr_down_sample4_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_3768), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(e, f, &[(0x8, 0x010b_0cf4), (0xc, 0x0106_3f24), (0x10, 0)]);
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_3700)]);
            0x010b_428cu32
        } else {
            put(e, f, &[(0x4, 0x010b_3734)]);
            0x010b_4298u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x3cu32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b90d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRLightAdapt::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: the
/// shader reload of the shader `00b90ce0` constructs (vtable `010b42c8`, slot +0x120). Builds
/// on its stack the vertex shader description `"imagespace\2x\v\HDRbase.v.hlsl"` (no defines)
/// and the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"ADAPT"` = `""`) with the name `"HDRADAPT"`,
/// else `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"ADAPT"` = `""`) with the name
/// `"HDRADAPTT"` (each 0x4c bytes: a file and up to nine define pairs, the rest cleared through
/// `memset` in the exe's order). Copies the pixel description to another stack block, calls its
/// vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"` and `"ps_2_a"`, then
/// slot +0x14c.
pub fn image_space_shader_hdr_light_adapt_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_3768), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(e, f, &[(0x8, 0x010b_4434), (0xc, 0x0101_1584), (0x10, 0)]);
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_3700)]);
            0x010b_441cu32
        } else {
            put(e, f, &[(0x4, 0x010b_3734)]);
            0x010b_4428u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x3cu32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b90ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRLumClamp::ReloadShaders` (Xbox PDB), `thiscall`, no stack words: the
/// shader reload of the shader `00b90e70` constructs (vtable `010b4460`, slot +0x120). Builds
/// on its stack the vertex shader description `"imagespace\2x\v\HDRbase.v.hlsl"` (no defines)
/// and the pixel one, which depends on the byte at `011f94fc`: while it is clear
/// `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"LUMCLAMP"` = `""`) with the name
/// `"HDRLUMCLAMP"`, else `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"LUMCLAMP"` = `""`)
/// with the name `"HDRLUMCLAMPT"` (each 0x4c bytes: a file and up to nine define pairs, the
/// rest cleared through `memset` in the exe's order). Copies the pixel description to another
/// stack block, calls its vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"`
/// and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_hdr_lum_clamp_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_3768), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(e, f, &[(0x8, 0x010b_45d0), (0xc, 0x0101_1584), (0x10, 0)]);
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_3700)]);
            0x010b_45b4u32
        } else {
            put(e, f, &[(0x4, 0x010b_3734)]);
            0x010b_45c0u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x14, 0u32, 0x3cu32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b91070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRDownSample4LightAdapt::ReloadShaders` (Xbox PDB), `thiscall`, no stack
/// words: the shader reload of the shader `00b91000` constructs (vtable `010b4600`, slot
/// +0x120). Builds on its stack the vertex shader description
/// `"imagespace\2x\v\HDRbase.v.hlsl"` (no defines) and the pixel one, which depends on the byte
/// at `011f94fc`: while it is clear `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"TEXTAP"` =
/// `"4"`, `"DOWNADAPT"` = `""`) with the name `"HDRDS4ADAPT"`, else
/// `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"TEXTAP"` = `"4"`, `"DOWNADAPT"` = `""`)
/// with the name `"HDRDS4ADAPTT"` (each 0x4c bytes: a file and up to nine define pairs, the
/// rest cleared through `memset` in the exe's order). Copies the pixel description to another
/// stack block, calls its vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"`
/// and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_hdr_down_sample4_light_adapt_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_3768), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_0cf4),
                (0xc, 0x0106_3f24),
                (0x10, 0x010b_4770),
                (0x14, 0x0101_1584),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_3700)]);
            0x010b_4754u32
        } else {
            put(e, f, &[(0x4, 0x010b_3734)]);
            0x010b_4760u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b91210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRDownSample4LumClamp::ReloadShaders` (Xbox PDB), `thiscall`, no stack
/// words: the shader reload of the shader `00b911a0` constructs (vtable `010b47a0`, slot
/// +0x120). Builds on its stack the vertex shader description
/// `"imagespace\2x\v\HDRbase.v.hlsl"` (no defines) and the pixel one, which depends on the byte
/// at `011f94fc`: while it is clear `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"TEXTAP"` =
/// `"4"`, `"LUMCLAMP"` = `""`) with the name `"HDRDS4LUMCLAMP"`, else
/// `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"TEXTAP"` = `"4"`, `"LUMCLAMP"` = `""`)
/// with the name `"HDRDS4LUMCLAMPT"` (each 0x4c bytes: a file and up to nine define pairs, the
/// rest cleared through `memset` in the exe's order). Copies the pixel description to another
/// stack block, calls its vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"`
/// and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_hdr_down_sample4_lum_clamp_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_3768), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_0cf4),
                (0xc, 0x0106_3f24),
                (0x10, 0x010b_45d0),
                (0x14, 0x0101_1584),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_3700)]);
            0x010b_48f4u32
        } else {
            put(e, f, &[(0x4, 0x010b_3734)]);
            0x010b_4904u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b913b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRDownSample16LightAdapt::ReloadShaders` (Xbox PDB), `thiscall`, no stack
/// words: the shader reload of the shader `00b91340` constructs (vtable `010b4938`, slot
/// +0x120). Builds on its stack the vertex shader description
/// `"imagespace\2x\v\HDRbase.v.hlsl"` (no defines) and the pixel one, which depends on the byte
/// at `011f94fc`: while it is clear `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"TEXTAP"` =
/// `"16"`, `"DOWNADAPT"` = `""`) with the name `"HDRDS16ADAPT"`, else
/// `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"TEXTAP"` = `"16"`, `"DOWNADAPT"` = `""`)
/// with the name `"HDRDS16ADAPTT"` (each 0x4c bytes: a file and up to nine define pairs, the
/// rest cleared through `memset` in the exe's order). Copies the pixel description to another
/// stack block, calls its vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"`
/// and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_hdr_down_sample16_light_adapt_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_3768), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_0cf4),
                (0xc, 0x010b_3f84),
                (0x10, 0x010b_4770),
                (0x14, 0x0101_1584),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_3700)]);
            0x010b_4a8cu32
        } else {
            put(e, f, &[(0x4, 0x010b_3734)]);
            0x010b_4a9cu32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

// Translated from 00b91550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderHDRDownSample16LumClamp::ReloadShaders` (Xbox PDB), `thiscall`, no stack
/// words: the shader reload of the shader `00b914e0` constructs (vtable `010b4ad0`, slot
/// +0x120). Builds on its stack the vertex shader description
/// `"imagespace\2x\v\HDRbase.v.hlsl"` (no defines) and the pixel one, which depends on the byte
/// at `011f94fc`: while it is clear `"imagespace\2x\p\HDRBlur.p.hlsl"` (defines `"TEXTAP"` =
/// `"16"`, `"LUMCLAMP"` = `""`) with the name `"HDRDS16LUMCLAMP"`, else
/// `"imagespace\2x\p\HDRBlur_TEST.p.hlsl"` (defines `"TEXTAP"` = `"16"`, `"LUMCLAMP"` = `""`)
/// with the name `"HDRDS16LUMCLAMPT"` (each 0x4c bytes: a file and up to nine define pairs, the
/// rest cleared through `memset` in the exe's order). Copies the pixel description to another
/// stack block, calls its vtable slot +0x13c with the two, the name and the profiles `"vs_1_1"`
/// and `"ps_2_a"`, then slot +0x14c.
pub fn image_space_shader_hdr_down_sample16_lum_clamp_reload_shaders(e: &mut Engine, this: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        put(e, f, &[(0x54, 0x010b_3768), (0x58, 0)]);
        e.call(MEMSET_00EC61C0, &args![f + 0x5c, 0u32, 0x44u32]);
        put(
            e,
            f,
            &[
                (0x8, 0x010b_0cf4),
                (0xc, 0x010b_3f84),
                (0x10, 0x010b_45d0),
                (0x14, 0x0101_1584),
                (0x18, 0),
            ],
        );
        let name = if e.global::<u8>(TEST_SHADERS_011F94FC) == 0 {
            put(e, f, &[(0x4, 0x010b_3700)]);
            0x010b_4c24u32
        } else {
            put(e, f, &[(0x4, 0x010b_3734)]);
            0x010b_4c34u32
        };
        e.call(MEMSET_00EC61C0, &args![f + 0x1c, 0u32, 0x34u32]);
        copy_words(e, f + 0x4, f + 0xa4, 0x13);
        e.vcall(
            this.addr(),
            0x13c,
            &args![f + 0x54, f + 0xa4, name, 0x010a_93ecu32, 0x010a_93c4u32],
        );
        e.vcall(this.addr(), 0x14c, &args![]);
    });
}

use crate::units::bsshader::geometrydecalshader::shader_inline::stage_release;

// Translated from 00b8aea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: resets
/// the manager's blend values from the x87 stack (1.0 and 0.0, swapped between stores): for k
/// from 0 to 14, 1.0 at +0xb4 + 4k and 0.0 at +0x14c + 4k; 1.0 at +0xf0, +0xf4, +0xf8; 0.0 at
/// +0x188, +0x18c, +0x190; then for k from 0 to 11, 1.0 at +0x118 + 4k and 0.0 at +0x1b0 + 4k.
pub fn fn_00b8aea0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let one = 1.0f32.to_bits();
    for k in 0..15u32 {
        e.mem.set_u32(base + 0xb4 + 4 * k, one);
        e.mem.set_u32(base + 0x14c + 4 * k, 0);
    }
    for offset in [0xf0u32, 0xf4, 0xf8] {
        e.mem.set_u32(base + offset, one);
    }
    for offset in [0x188u32, 0x18c, 0x190] {
        e.mem.set_u32(base + offset, 0);
    }
    for k in 0..12u32 {
        e.mem.set_u32(base + 0x118 + 4 * k, one);
        e.mem.set_u32(base + 0x1b0 + 4 * k, 0);
    }
}

/// `FLD dword` then `FCOMP` of the field at `field` with the value on the stack and the exe's
/// `TEST AH,5` / `JP`: true when the field is below the value (neither is NaN).
fn field_below(e: &Engine, field: u32, value: f32) -> bool {
    let held = x87_load32(e.mem.u32(field));
    let given = x87_load32(value.to_bits());
    held < given
}

/// The value given to the keep-the-larger setters, stored by `FSTP` (a signalling NaN
/// quieted; one never reaches the store, as a NaN fails the compare).
fn keep_larger(e: &mut Engine, field: u32, value: f32) {
    if field_below(e, field, value) {
        e.mem.set_u32(field, x87_quiet_bits(value.to_bits()));
    }
}

/// `FLD` of the stack `float`, `FADD` of the field, `FSTP` to the field.
fn add_to_field(e: &mut Engine, field: u32, value: f32) {
    let sum = x87_add(x87_load32(value.to_bits()), x87_m32(e.mem.u32(field)));
    e.mem.set_u32(field, x87_store32(sum));
}

/// `FLD` of the field, `FADD` of the `float` at `operand`, `FSTP` to the field.
fn field_plus(e: &mut Engine, field: u32, operand: u32) {
    let sum = x87_add(x87_load32(e.mem.u32(field)), x87_m32(e.mem.u32(operand)));
    e.mem.set_u32(field, x87_store32(sum));
}

/// `FLD` of the `float` at `operand`, `FADD` of the field, `FSTP` to the field.
fn plus_field(e: &mut Engine, field: u32, operand: u32) {
    let sum = x87_add(x87_load32(e.mem.u32(operand)), x87_m32(e.mem.u32(field)));
    e.mem.set_u32(field, x87_store32(sum));
}

/// `FLD`/`FSTP` of a `float` from `from` to `to`: its bits, a signalling NaN quieted.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

// Translated from 00b8af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (a
/// `float`): keeps the larger of the `float` at +0x25c and the value (stored when the field is
/// below it).
pub fn fn_00b8af10(e: &mut Engine, this: Ptr, value: f32) {
    keep_larger(e, this.addr() + 0x25c, value);
}

// Translated from 00b8af40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (a
/// `float`): adds the value to the `float` at +0x260 (`FLD` value, `FADD` field).
pub fn fn_00b8af40(e: &mut Engine, this: Ptr, value: f32) {
    add_to_field(e, this.addr() + 0x260, value);
}

// Translated from 00b8af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (a
/// `float`): keeps the larger of the `float` at +0x2f0 and the value.
pub fn fn_00b8af60(e: &mut Engine, this: Ptr, value: f32) {
    keep_larger(e, this.addr() + 0x2f0, value);
}

// Translated from 00b8af90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (a
/// `float`): adds the value to the `float` at +0x2f4.
pub fn fn_00b8af90(e: &mut Engine, this: Ptr, value: f32) {
    add_to_field(e, this.addr() + 0x2f4, value);
}

// Translated from 00b8afb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (a
/// `float`): keeps the larger of the `float` at +0x264 and the value.
pub fn fn_00b8afb0(e: &mut Engine, this: Ptr, value: f32) {
    keep_larger(e, this.addr() + 0x264, value);
}

// Translated from 00b8afe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (a
/// `float`): adds the value to the `float` at +0x268.
pub fn fn_00b8afe0(e: &mut Engine, this: Ptr, value: f32) {
    add_to_field(e, this.addr() + 0x268, value);
}

// Translated from 00b8b000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, six stack words: a
/// `float` weight, four `float`s and the address of two words. When the `float` at +0x26c is
/// below the weight, stores the weight there, the four `float`s at +0x270 to +0x27c (through
/// the FPU) and the two words at +0x294 and +0x298.
pub fn fn_00b8b000(
    e: &mut Engine,
    this: Ptr,
    weight: f32,
    first: f32,
    second: f32,
    third: f32,
    fourth: f32,
    pair: Ptr,
) {
    let base = this.addr();
    if field_below(e, base + 0x26c, weight) {
        e.mem
            .set_u32(base + 0x26c, x87_quiet_bits(weight.to_bits()));
        for (offset, value) in [
            (0x270u32, first),
            (0x274, second),
            (0x278, third),
            (0x27c, fourth),
        ] {
            e.mem
                .set_u32(base + offset, x87_quiet_bits(value.to_bits()));
        }
        let low = e.mem.u32(pair.addr());
        e.mem.set_u32(base + 0x294, low);
        let high = e.mem.u32(pair.addr() + 4);
        e.mem.set_u32(base + 0x298, high);
    }
}

// Translated from 00b8b0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, five stack words: a
/// `float` weight, a mode and three `float`s. When the `float` at +0x2a4 is below the weight,
/// stores the weight there, clears the byte at +0x2cc (set instead when the mode is above 2,
/// which then loses 3), stores the mode at +0x2c4, the second and third `float`s at +0x2a8 and
/// +0x2ac and the first at +0x2b0.
pub fn fn_00b8b0d0(
    e: &mut Engine,
    this: Ptr,
    weight: f32,
    mode: u32,
    first: f32,
    second: f32,
    third: f32,
) {
    let base = this.addr();
    if field_below(e, base + 0x2a4, weight) {
        e.mem
            .set_u32(base + 0x2a4, x87_quiet_bits(weight.to_bits()));
        e.mem.set_u8(base + 0x2cc, 0);
        let mut mode = mode;
        if mode > 2 {
            e.mem.set_u8(base + 0x2cc, 1);
            mode = mode.wrapping_sub(3);
        }
        e.mem.set_u32(base + 0x2c4, mode);
        e.mem
            .set_u32(base + 0x2a8, x87_quiet_bits(second.to_bits()));
        e.mem.set_u32(base + 0x2ac, x87_quiet_bits(third.to_bits()));
        e.mem.set_u32(base + 0x2b0, x87_quiet_bits(first.to_bits()));
    }
}

// Translated from 00b8b130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, five stack words: a
/// `float`, a mode and three `float`s. Adds the first `float` to +0x2b4 (`FLD` field, `FADD`
/// value), clears the byte at +0x2cd, stores the mode at +0x2c8, and adds the second and third
/// `float`s to +0x2b8 and +0x2bc and the `float` after the mode to +0x2c0 (each as field + value).
pub fn fn_00b8b130(
    e: &mut Engine,
    this: Ptr,
    weight: f32,
    mode: u32,
    first: f32,
    second: f32,
    third: f32,
) {
    let base = this.addr();
    let field = |e: &Engine, offset: u32| x87_load32(e.mem.u32(base + offset));
    let sum = x87_add(field(e, 0x2b4), x87_m32(weight.to_bits()));
    e.mem.set_u8(base + 0x2cd, 0);
    e.mem.set_u32(base + 0x2c8, mode);
    e.mem.set_u32(base + 0x2b4, x87_store32(sum));
    for (offset, value) in [(0x2b8u32, second), (0x2bc, third), (0x2c0, first)] {
        let sum = x87_add(field(e, offset), x87_m32(value.to_bits()));
        e.mem.set_u32(base + offset, x87_store32(sum));
    }
}

/// `ImageSpaceTexture::GetTexture` (`00ba3780`, `thiscall`), `::BorrowRenderedTexture`
/// (`00ba3840`, `thiscall`, one word) and `::GetRenderedTexture` (`00ba3770`, `thiscall`)
/// (engine map names).
const TEXTURE_GET_00BA3780: u32 = 0x00ba_3780;
const TEXTURE_BORROW_RENDERED_00BA3840: u32 = 0x00ba_3840;
const TEXTURE_GET_RENDERED_00BA3770: u32 = 0x00ba_3770;

// Translated from 00b8b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceManager::GetDepthRenderedTexture` (Xbox PDB), `thiscall`, no stack words: on the
/// `ImageSpaceTexture` at +0x204, when `GetTexture` (`00ba3780`) answers null borrows a
/// rendered texture with 0x21 (`00ba3840`); answers what `GetRenderedTexture` (`00ba3770`,
/// reached by a jump) answers.
pub fn image_space_manager_get_depth_rendered_texture(e: &mut Engine, this: Ptr) -> u32 {
    let texture = this.addr() + 0x204;
    if e.call(TEXTURE_GET_00BA3780, &args![texture]).u32() == 0 {
        e.call(TEXTURE_BORROW_RENDERED_00BA3840, &args![texture, 0x21u32]);
    }
    e.call(TEXTURE_GET_RENDERED_00BA3770, &args![texture]).u32()
}

/// The byte `00b8b200` stores (`012023f0`).
const BYTE_SETTING_012023F0: u32 = 0x0120_23f0;

// Translated from 00b8b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall` (its caller in `InitSky`
/// sets `ECX`; the body never reads it), one stack word: stores its low byte at `012023f0`.
pub fn fn_00b8b200(e: &mut Engine, _this: Ptr, value: u32) {
    e.set_global::<u8>(BYTE_SETTING_012023F0, value as u8);
}

// Translated from 00b8b210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (a
/// mode), on an image-space effect: nothing when the mode is already the word at +0x58; else
/// sets the byte at +5, keeps the mode at +0x58 and stores 1.0 at +0x70 when the mode is 0 or 1
/// (else 0.0) and 1.0 at +0x74 when it is 0 or 2 (else 0.0), each through the FPU and the
/// stack word.
pub fn fn_00b8b210(e: &mut Engine, this: Ptr, mode: u32) {
    let base = this.addr();
    if mode == e.mem.u32(base + 0x58) {
        return;
    }
    e.mem.set_u8(base + 5, 1);
    e.mem.set_u32(base + 0x58, mode);
    let one = 1.0f32.to_bits();
    let first = if mode == 1 || mode == 0 { one } else { 0 };
    e.mem.set_u32(base + 0x70, first);
    let second = if mode == 2 || mode == 0 { one } else { 0 };
    e.mem.set_u32(base + 0x74, second);
}

// Translated from 00b8b2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words:
/// destroys the image-space effects. For the indices 0 to 0x73 of the effect array (element
/// block +0x8, read again for each), a set element gets its vtable slot +0xc, then its slot 0
/// (the deleting destructor) with 1, and null is stored at its index through `00ba3aa0` (on the
/// array at +0x4, with the address of a cleared stack word). Then every element below the
/// 16-bit size at +0xe (read again for each) is cleared, and the size and the count (+0x10)
/// become 0.
pub fn fn_00b8b2e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        for index in 0..0x74u32 {
            let block = e.mem.u32(base + EFFECTS);
            let element = e.mem.u32(block + 4 * index);
            if element != 0 {
                e.vcall(element, 0xc, &args![]);
                e.vcall(element, 0, &args![1u32]);
                e.mem.set_u32(local.addr(), 0);
                e.call(ARRAY_SET_00BA3AA0, &args![base + 4, index, local]);
            }
        }
    });
    let mut index = 0u16;
    if index < e.mem.u16(base + EFFECTS_SIZE) {
        loop {
            let block = e.mem.u32(base + EFFECTS);
            e.mem.set_u32(block + 4 * u32::from(index), 0);
            index = index.wrapping_add(1);
            if index >= e.mem.u16(base + EFFECTS_SIZE) {
                break;
            }
        }
    }
    e.mem.set_u16(base + EFFECTS_SIZE, 0);
    e.mem.set_u16(base + EFFECTS_COUNT, 0);
}

// Translated from 00b8b390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (an
/// index): when the effect array holds an element at the index, answers its vtable slot +0x18
/// (reached by a jump; `ImageSpaceEffect::IsActive` in the Xbox PDB's slot order), else 0
/// (`XOR AL,AL`).
pub fn fn_00b8b390(e: &mut Engine, this: Ptr, index: u32) -> u8 {
    let block = e.mem.u32(this.addr() + EFFECTS);
    let element = e.mem.u32(block.wrapping_add(index.wrapping_mul(4)));
    if element != 0 {
        e.vcall(element, 0x18, &args![]).u32() as u8
    } else {
        0
    }
}

/// The byte `00b8b3f0` tests (`011ad884`).
const COUNT_ACTIVE_011AD884: u32 = 0x011a_d884;

// Translated from 00b8b3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: stores
/// -1 at +0x1ec and 0 at +0x1e8; when the byte at `011ad884` is set, for the indices 0 to 0xc
/// of the effect array a set element whose vtable slot +0x18 answers a set byte adds one to
/// +0x1e8 and leaves its index at +0x1ec.
pub fn fn_00b8b3f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x1ec, u32::MAX);
    e.mem.set_u32(base + 0x1e8, 0);
    if e.global::<u8>(COUNT_ACTIVE_011AD884) != 0 {
        for index in 0..0xdu32 {
            let block = e.mem.u32(base + EFFECTS);
            let element = e.mem.u32(block + 4 * index);
            if element != 0 && e.vcall(element, 0x18, &args![]).u32() as u8 != 0 {
                let count = e.mem.u32(base + 0x1e8);
                e.mem.set_u32(base + 0x1e8, count.wrapping_add(1));
                e.mem.set_u32(base + 0x1ec, index);
            }
        }
    }
}

/// The globals `00b8b440` sets: `011ad87c`, `011ad880`, `011f9184` to `011f9190`.
const VALUE_011AD87C: u32 = 0x011a_d87c;
const VALUE_011AD880: u32 = 0x011a_d880;

// Translated from 00b8b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (a
/// flag byte). Hands parameter blocks to the effects of the array (element block +0x8, read
/// again for each): element 0 gets its vtable slot +0x20 with +0x50, element 1 slot +0x20 with
/// +0x14, element 8 slot +0x20 with +0x5c, element 3 slot +0x24 with +0x78 and the word at
/// +0xa8, then slot +0x28 with +0x68 (each when set). Then, with the flag clear, stores the
/// `float` at +0x34 at `011ad87c` and 1.0 at `011ad880`, else the `float` at +0x34 at
/// `011ad880`; and the `float`s at +0x40, +0x48, +0x44 and +0x1c at `011f9190`, `011f918c`,
/// `011f9188` and `011f9184`.
pub fn fn_00b8b440(e: &mut Engine, this: Ptr, flag: u32) {
    let base = this.addr();
    let element = |e: &Engine, index: u32| e.mem.u32(e.mem.u32(base + EFFECTS) + 4 * index);
    for (index, block) in [(0u32, 0x50u32), (1, 0x14), (8, 0x5c)] {
        if element(e, index) != 0 {
            let effect = element(e, index);
            e.vcall(effect, 0x20, &args![base + block]);
        }
    }
    if element(e, 3) != 0 {
        let effect = element(e, 3);
        let word = e.mem.u32(base + 0xa8);
        e.vcall(effect, 0x24, &args![base + 0x78, word]);
        let effect = element(e, 3);
        e.vcall(effect, 0x28, &args![base + 0x68]);
    }
    let value = x87_quiet_bits(e.mem.u32(base + 0x34));
    if flag as u8 == 0 {
        e.set_global::<u32>(VALUE_011AD87C, value);
        e.set_global::<u32>(VALUE_011AD880, 1.0f32.to_bits());
    } else {
        e.set_global::<u32>(VALUE_011AD880, value);
    }
    for (offset, global) in [
        (0x40u32, 0x011f_9190u32),
        (0x48, 0x011f_918c),
        (0x44, 0x011f_9188),
        (0x1c, 0x011f_9184),
    ] {
        let value = x87_quiet_bits(e.mem.u32(base + offset));
        e.set_global::<u32>(global, value);
    }
}

/// `_memcpy` (`00ec44d0`, `cdecl`: destination, source, size) and the default block `00b8b500`
/// copies (`011ff940`); the two `float`s it clears (`01202400`, `012003d0`).
const MEMCPY_00EC44D0: u32 = 0x00ec_44d0;
const DEFAULTS_011FF940: u32 = 0x011f_f940;
const VALUE_01202400: u32 = 0x0120_2400;
const VALUE_012003D0: u32 = 0x0120_03d0;

// Translated from 00b8b500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: copies
/// 0x98 bytes to +0x14 from the block the word at +0xb0 points at, else the one at +0xac, else
/// the defaults at `011ff940` (`memcpy`), then stores 0.0 at `01202400` and `012003d0`.
pub fn fn_00b8b500(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut source = e.mem.u32(base + 0xb0);
    if source == 0 {
        source = e.mem.u32(base + 0xac);
        if source == 0 {
            source = DEFAULTS_011FF940;
        }
    }
    e.call(MEMCPY_00EC44D0, &args![base + 0x14, source, 0x98u32]);
    e.set_global::<u32>(VALUE_01202400, 0);
    e.set_global::<u32>(VALUE_012003D0, 0);
}

/// The body of `00b8b540` and `00b8b610`: a colour of four `float`s at `color` (red, green,
/// blue, alpha) weighted by its alpha and added to the four `float`s at `sum`, the weight kept
/// at `weight` when the alpha is above it; in the exe's x87 order (the red through the stack
/// word, the colour's red, green and blue left multiplied).
fn add_weighted_color(e: &mut Engine, weight: u32, sum: u32, color: u32) {
    let alpha = |e: &Engine| e.mem.u32(color + 0xc);
    if x87_load32(e.mem.u32(weight)) < x87_load32(alpha(e)) {
        let bits = x87_quiet_bits(alpha(e));
        e.mem.set_u32(weight, bits);
    }
    let red = x87_store32(x87_mul(x87_load32(e.mem.u32(color)), x87_m32(alpha(e))));
    let red = x87_load32(red);
    e.mem.set_u32(color, x87_store32(red));
    let green = x87_mul(x87_load32(e.mem.u32(color + 4)), x87_m32(alpha(e)));
    e.mem.set_u32(color + 4, x87_store32(green));
    let blue = x87_mul(x87_load32(alpha(e)), x87_m32(e.mem.u32(color + 8)));
    e.mem.set_u32(color + 8, x87_store32(blue));
    let total = x87_add(red, x87_m32(e.mem.u32(sum)));
    e.mem.set_u32(sum, x87_store32(total));
    plus_field(e, sum + 4, color + 4);
    field_plus(e, sum + 8, color + 8);
    field_plus(e, sum + 0xc, color + 0xc);
}

// Translated from 00b8b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// address of a colour of four `float`s): `add_weighted_color` with the weight at +0x234 and the
/// sum at +0x214 (the colour itself left multiplied by its alpha).
pub fn fn_00b8b540(e: &mut Engine, this: Ptr, color: Ptr) {
    let base = this.addr();
    add_weighted_color(e, base + 0x234, base + 0x214, color.addr());
}

// Translated from 00b8b5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// address of four `float`s): adds them to the `float`s at +0x224 to +0x230 (the first as
/// field + value, the others as value + field).
pub fn fn_00b8b5c0(e: &mut Engine, this: Ptr, values: Ptr) {
    let base = this.addr();
    let values = values.addr();
    field_plus(e, base + 0x224, values);
    for k in 1..4u32 {
        plus_field(e, base + 0x224 + 4 * k, values + 4 * k);
    }
}

// Translated from 00b8b610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// address of a colour of four `float`s): `add_weighted_color` with the weight at +0x258 and the
/// sum at +0x238.
pub fn fn_00b8b610(e: &mut Engine, this: Ptr, color: Ptr) {
    let base = this.addr();
    add_weighted_color(e, base + 0x258, base + 0x238, color.addr());
}

// Translated from 00b8b690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word (the
/// address of four `float`s): adds them to the `float`s at +0x248 to +0x254 (the first as
/// field + value, the others as value + field).
pub fn fn_00b8b690(e: &mut Engine, this: Ptr, values: Ptr) {
    let base = this.addr();
    let values = values.addr();
    field_plus(e, base + 0x248, values);
    for k in 1..4u32 {
        plus_field(e, base + 0x248 + 4 * k, values + 4 * k);
    }
}

// Translated from 00b8b6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, six stack words: five
/// `float`s and the address of two more. Adds the five to the `float`s at +0x280 to +0x290 (the
/// first and fifth as value + field, the others as field + value) and the two to +0x29c and
/// +0x2a0 (value + field).
pub fn fn_00b8b6e0(
    e: &mut Engine,
    this: Ptr,
    first: f32,
    second: f32,
    third: f32,
    fourth: f32,
    fifth: f32,
    pair: Ptr,
) {
    let base = this.addr();
    add_to_field(e, base + 0x280, first);
    for (offset, value) in [(0x284u32, second), (0x288, third), (0x28c, fourth)] {
        let sum = x87_add(
            x87_load32(e.mem.u32(base + offset)),
            x87_m32(value.to_bits()),
        );
        e.mem.set_u32(base + offset, x87_store32(sum));
    }
    add_to_field(e, base + 0x290, fifth);
    plus_field(e, base + 0x29c, pair.addr());
    plus_field(e, base + 0x2a0, pair.addr() + 4);
}

// Translated from 00b8b9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: resets
/// the frame's accumulated values. With the `float` at +0x94 (through a stack word): it at
/// +0x234, the `float`s at +0x88 and +0x8c times it at +0x214 and +0x218, it times +0x90 at
/// +0x21c, +0x94 copied to +0x220; 0.0 in the `float`s and words from +0x224 to +0x2c0 (but the
/// modes) and at +0x2d0, +0x2f0, +0x2f4; 3 at +0x2c4 and +0x2c8, the bytes at +0x2cc and +0x2cd
/// cleared; then 0x130 bytes from +0xb4 cleared (`memset`).
pub fn fn_00b8b9a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let weight = x87_load32(x87_quiet_bits(e.mem.u32(base + 0x94)));
    e.mem.set_u32(base + 0x234, x87_store32(weight));
    let red = x87_mul(x87_load32(e.mem.u32(base + 0x88)), weight);
    e.mem.set_u32(base + 0x214, x87_store32(red));
    let green = x87_mul(x87_load32(e.mem.u32(base + 0x8c)), weight);
    e.mem.set_u32(base + 0x218, x87_store32(green));
    let blue = x87_mul(weight, x87_m32(e.mem.u32(base + 0x90)));
    e.mem.set_u32(base + 0x21c, x87_store32(blue));
    copy_float(e, base + 0x94, base + 0x220);
    for offset in [
        0x224u32, 0x258, 0x238, 0x228, 0x23c, 0x240, 0x22c, 0x244, 0x230, 0x248, 0x24c, 0x250,
        0x254, 0x25c, 0x264, 0x26c, 0x270, 0x274, 0x278, 0x27c, 0x2a4, 0x2a8, 0x294, 0x2ac, 0x298,
        0x2b0, 0x2d0, 0x2f0,
    ] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x2c4, 3);
    e.mem.set_u32(base + 0x260, 0);
    e.mem.set_u8(base + 0x2cc, 0);
    e.mem.set_u32(base + 0x268, 0);
    e.mem.set_u32(base + 0x2c8, 3);
    e.mem.set_u32(base + 0x280, 0);
    e.mem.set_u8(base + 0x2cd, 0);
    for offset in [
        0x284u32, 0x288, 0x28c, 0x290, 0x2b4, 0x29c, 0x2b8, 0x2bc, 0x2a0, 0x2c0, 0x2f4,
    ] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(MEMSET_00EC61C0, &args![base + 0xb4, 0u32, 0x130u32]);
}

/// Render states set one after the other on the pass the `NiPointer` at +0x5c holds, read
/// again for each (`pass_render_state`).
fn render_states_on_pass(e: &mut Engine, this: Ptr, states: &[(u32, u32)]) {
    for &(state, value) in states {
        let pass = e.mem.u32(this.addr() + 0x5c);
        pass_render_state(e, pass, state, value);
    }
}

// Translated from 00b8bfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: slot
/// +0x150 of the shader `00b8bea0` constructs (vtable `010b0198`): the render states 0x34 = 1,
/// 0x38 = 7, 0x39 = 0x80 and 0x37 = 1 on the pass at +0x5c (`render_states_on_pass`).
pub fn fn_00b8bfe0(e: &mut Engine, this: Ptr) {
    render_states_on_pass(e, this, &[(0x34, 1), (0x38, 7), (0x39, 0x80), (0x37, 1)]);
}

/// `NiPointer` release inline: when the object is set, the imported `InterlockedDecrement`
/// on its +4 and, at zero, its vtable slot +4.
fn release_object(e: &mut Engine, object: u32) {
    if object != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![object + 4])
            .u32();
        if left == 0 {
            e.vcall(object, 4, &args![]);
        }
    }
}

// Translated from 00b8c690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words:
/// releases the `NiPointer`s at +0x1f0, +0x1f8, +0x1f4 and +0x200 in that order, each cleared
/// after its release when it was set.
pub fn fn_00b8c690(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [0x1f0u32, 0x1f8, 0x1f4, 0x200] {
        let object = e.mem.u32(base + offset);
        if object != 0 {
            release_object(e, object);
            e.mem.set_u32(base + offset, 0);
        }
    }
}

/// `ImageSpaceTexture::SetTexture` (`00ba37a0`, `thiscall`, one word) and
/// `ImageSpaceEffect::SetTexture` (`00ba3cb0`, `thiscall`, three words) (engine map names).
const TEXTURE_SET_00BA37A0: u32 = 0x00ba_37a0;
const EFFECT_SET_TEXTURE_00BA3CB0: u32 = 0x00ba_3cb0;

/// The thread block's allocation context guard the manager's entry points put around their
/// body: the word at +0x2b4 kept, 0x27 set, `body`, the kept word put back.
fn with_image_space_context<R>(e: &mut Engine, body: impl FnOnce(&mut Engine) -> R) -> R {
    let tls = e.tls();
    let saved = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem
        .set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_IMAGE_SPACE);
    let result = body(e);
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved);
    result
}

// Translated from 00b8c830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceManager::RenderEffect_ov2` (Xbox PDB), `thiscall`, six stack words: an effect,
/// a word, two source textures, two more words. Within the allocation context guard: two
/// `ImageSpaceTexture`s on the stack (three clear bytes and three clear words each) are set from
/// the sources (`SetTexture`, `00ba37a0`: the first from the first source, the second from the
/// second); with an effect, `ImageSpaceEffect::SetTexture` (`00ba3cb0`) gives it the second at 0
/// and the first at 1, its vtable slot +4 runs with the `NiPointer` at +0x1f8 (when the byte at
/// +0x1fc is set) or +0x1f0, the word and the last two words, and both textures are taken back
/// (null at 0 and 1); then both stack textures are destroyed (`00ba3a00`), the second first.
// Translation incomplete: 00b8c830 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the two stack textures) is not translated.
pub fn image_space_manager_render_effect_ov2(
    e: &mut Engine,
    this: Ptr,
    effect: u32,
    word: u32,
    first_source: u32,
    second_source: u32,
    fifth: u32,
    sixth: u32,
) {
    let base = this.addr();
    with_image_space_context(e, |e| {
        e.with_stack(0x20, |e, locals| {
            let second = locals.addr();
            let first = second + 0x10;
            for texture in [first, second] {
                for offset in 0..3u32 {
                    e.mem.set_u8(texture + offset, 0);
                }
                for offset in [4u32, 8, 0xc] {
                    e.mem.set_u32(texture + offset, 0);
                }
            }
            e.call(TEXTURE_SET_00BA37A0, &args![first, first_source]);
            e.call(TEXTURE_SET_00BA37A0, &args![second, second_source]);
            if effect != 0 {
                e.call(
                    EFFECT_SET_TEXTURE_00BA3CB0,
                    &args![effect, 0u32, second, 0u32],
                );
                e.call(
                    EFFECT_SET_TEXTURE_00BA3CB0,
                    &args![effect, 1u32, first, 0u32],
                );
                let slot = if e.mem.u8(base + 0x1fc) != 0 {
                    base + 0x1f8
                } else {
                    base + 0x1f0
                };
                let target = e.mem.u32(slot);
                e.vcall(effect, 4, &args![target, word, fifth, sixth]);
                e.call(
                    EFFECT_SET_TEXTURE_00BA3CB0,
                    &args![effect, 0u32, 0u32, 0u32],
                );
                e.call(
                    EFFECT_SET_TEXTURE_00BA3CB0,
                    &args![effect, 1u32, 0u32, 0u32],
                );
            }
            e.call(IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00, &args![second]);
            e.call(IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00, &args![first]);
        });
    });
}

// Translated from 00b8cad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, one stack word: stores
/// it at +0xac, between a read of the thread's allocation context (+0x2b4) and a write of the
/// same word back (the guard with nothing in it).
pub fn fn_00b8cad0(e: &mut Engine, this: Ptr, value: u32) {
    let tls = e.tls();
    let saved = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem.set_u32(this.addr() + 0xac, value);
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved);
}

/// The blend of one value with two parameters, the exe's loop of two steps through its stack
/// words: the value (`running`, bits of a stored `float`) times the first, plus the second,
/// each step's result stored as a `float` and read back. Answers the result as the x87 stack
/// holds it.
fn blend_steps(first: u32, second: u32, running: u32) -> f64 {
    let product = x87_store32(x87_mul(x87_load32(first), x87_m32(running)));
    let running = x87_quiet_bits(product);
    let sum = x87_store32(x87_add(x87_load32(second), x87_m32(running)));
    x87_load32(sum)
}

// Translated from 00b8cb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, four stack words: a
/// kind, an index, the address of two `float`s and a `float` factor. Within the allocation
/// context guard, picks the `float` at +0x50 (kind 0), +0x14 (kind 1) or +0x78 (kind 3) plus 4
/// times the index (any other kind reads and writes address 0, as the exe does), blends it with
/// the two `float`s (`blend_steps`: times the first, plus the second) into r, then stores
/// field - (field - r) * factor (the difference rounded to a `float` in the stack word first).
pub fn fn_00b8cb30(e: &mut Engine, this: Ptr, kind: u32, index: u32, values: Ptr, factor: f32) {
    let base = this.addr();
    with_image_space_context(e, |e| {
        let field = match kind {
            0 => base.wrapping_add(0x50).wrapping_add(index.wrapping_mul(4)),
            1 => base.wrapping_add(0x14).wrapping_add(index.wrapping_mul(4)),
            3 => base.wrapping_add(0x78).wrapping_add(index.wrapping_mul(4)),
            _ => 0,
        };
        let running = x87_quiet_bits(e.mem.u32(field));
        let first = x87_quiet_bits(e.mem.u32(values.addr()));
        let second = x87_quiet_bits(e.mem.u32(values.addr() + 4));
        let result = blend_steps(first, second, running);
        let held = x87_load32(e.mem.u32(field));
        let difference = x87_store32(x87_sub(held, result));
        let scaled = x87_mul(x87_load32(difference), x87_m32(factor.to_bits()));
        e.mem.set_u32(field, x87_store32(x87_sub(held, scaled)));
    });
}

// Translated from 00b8cc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, four stack words: a
/// kind, an index, the address of two `float`s and a `float` factor. Within the allocation
/// context guard, for the two blocks at +0xb4 + 4 times the index and 0x98 bytes further, adds
/// the n-th `float` times the factor to the block's `float` at +0x3c (kind 0), +0 (kind 1) or
/// +0x64 (kind 3); any other kind changes nothing.
pub fn fn_00b8cc20(e: &mut Engine, this: Ptr, kind: u32, index: u32, values: Ptr, factor: f32) {
    let base = this.addr();
    let factor = x87_load32(factor.to_bits());
    with_image_space_context(e, |e| {
        let mut block = base.wrapping_add(0xb4).wrapping_add(index.wrapping_mul(4));
        for n in 0..2u32 {
            let offset = match kind {
                0 => Some(0x3c),
                1 => Some(0),
                3 => Some(0x64),
                _ => None,
            };
            if let Some(offset) = offset {
                let field = block.wrapping_add(offset);
                let value = x87_load32(e.mem.u32(values.addr() + 4 * n));
                let sum = x87_add(x87_mul(value, factor), x87_m32(e.mem.u32(field)));
                e.mem.set_u32(field, x87_store32(sum));
            }
            block = block.wrapping_add(0x98);
        }
    });
}

// Translated from 00b8ccb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: settles
/// the frame's values. For the 15 `float`s from +0x14, the 3 from +0x50 and the 12 from +0x78,
/// each within its own allocation context guard: the blend of the field with its parameters at
/// +0xa0 and +0x138 further (`blend_steps`) into r, stored as field - float(field - r). Then
/// `00b8b540` with +0x224 and `00b8b610` with +0x248; then the `float`s at +0x25c, +0x264 and
/// +0x26c take those at +0x260, +0x268 and +0x280 when below them (+0x26c also takes +0x284 to
/// +0x290 into +0x270 to +0x27c and the words at +0x29c, +0x2a0 into +0x294, +0x298); +0x2a4
/// takes +0x2b4 when below it, with the mode at +0x2c8 into +0x2c4 (above 2: the byte at +0x2cc
/// set and the mode less 3; else the byte cleared), +0x2b8 and +0x2bc into +0x2a8 and +0x2ac and
/// 0.0 into +0x2b0; and +0x2f0 takes +0x2f4 when below it.
pub fn fn_00b8ccb0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (start, count) in [(0x14u32, 15u32), (0x50, 3), (0x78, 12)] {
        for k in 0..count {
            let field = base + start + 4 * k;
            with_image_space_context(e, |e| {
                let first = x87_quiet_bits(e.mem.u32(field + 0xa0));
                let second = x87_quiet_bits(e.mem.u32(field + 0x138));
                let running = x87_quiet_bits(e.mem.u32(field));
                let result = blend_steps(first, second, running);
                let held = x87_load32(e.mem.u32(field));
                let difference = x87_store32(x87_sub(held, result));
                let settled = x87_sub(held, x87_m32(difference));
                e.mem.set_u32(field, x87_store32(settled));
            });
        }
    }
    fn_00b8b540(e, this, Ptr::new(base + 0x224));
    fn_00b8b610(e, this, Ptr::new(base + 0x248));
    for (field, source) in [(0x25cu32, 0x260u32), (0x264, 0x268)] {
        let value = x87_quiet_bits(e.mem.u32(base + source));
        if field_below(e, base + field, f32::from_bits(value)) {
            e.mem.set_u32(base + field, value);
        }
    }
    let value = x87_quiet_bits(e.mem.u32(base + 0x280));
    if field_below(e, base + 0x26c, f32::from_bits(value)) {
        let low = e.mem.u32(base + 0x29c);
        e.mem.set_u32(base + 0x26c, value);
        copy_float(e, base + 0x284, base + 0x270);
        let high = e.mem.u32(base + 0x2a0);
        e.mem.set_u32(base + 0x294, low);
        e.mem.set_u32(base + 0x298, high);
        copy_float(e, base + 0x288, base + 0x274);
        copy_float(e, base + 0x28c, base + 0x278);
        copy_float(e, base + 0x290, base + 0x27c);
    }
    let mut mode = e.mem.u32(base + 0x2c8);
    let weight = x87_quiet_bits(e.mem.u32(base + 0x2b4));
    let first = x87_quiet_bits(e.mem.u32(base + 0x2b8));
    let second = x87_quiet_bits(e.mem.u32(base + 0x2bc));
    if field_below(e, base + 0x2a4, f32::from_bits(weight)) {
        e.mem.set_u32(base + 0x2a4, weight);
        e.mem.set_u8(base + 0x2cc, 0);
        if mode > 2 {
            e.mem.set_u8(base + 0x2cc, 1);
            mode = mode.wrapping_sub(3);
        }
        e.mem.set_u32(base + 0x2c4, mode);
        e.mem.set_u32(base + 0x2a8, first);
        e.mem.set_u32(base + 0x2ac, second);
        e.mem.set_u32(base + 0x2b0, 0);
    }
    let value = x87_quiet_bits(e.mem.u32(base + 0x2f4));
    if field_below(e, base + 0x2f0, f32::from_bits(value)) {
        e.mem.set_u32(base + 0x2f0, value);
    }
}

/// `00b8bc80` (same unit, unnamed; `thiscall`, no stack words), the alternative of `00b8bc20`
/// that `00b8d020` runs on the effect at index 4 when the byte at +0x2cc is set.
const SET_UP_00B8BC80: u32 = 0x00b8_bc80;

/// The division of three `float`s by a fourth `00b8d020` makes twice: when the fourth (at
/// `last`) is above 0.0, the three before it are divided by it and it by itself (`FDIVR`),
/// each through the FPU with the divisor read through the stack word.
fn divide_by_last(e: &mut Engine, last: u32) {
    let bits = e.mem.u32(last);
    if 0.0 < x87_load32(bits) {
        let divisor = x87_load32(x87_quiet_bits(bits));
        for offset in [0xcu32, 8, 4] {
            let field = last - offset;
            let quotient = x87_div(x87_load32(e.mem.u32(field)), divisor);
            e.mem.set_u32(field, x87_store32(quotient));
        }
        let quotient = x87_div(x87_m32(e.mem.u32(last)), divisor);
        e.mem.set_u32(last, x87_store32(quotient));
    }
}

// Translated from 00b8d020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: hands
/// the frame's settled values to the effects. The colour sums at +0x214 and +0x238 are divided
/// by their weights at +0x220 and +0x244 when above 0.0 (`divide_by_last`), then +0x220 takes
/// +0x234 and +0x244 takes +0x258, and the sums go to +0x88..+0x94 and +0x98..+0xa4. The
/// `float`s at +0x264 and +0x25c go to `01202400` and `012003d0`. Effect 6 (the array's element
/// block +0x8) gets +0x26c to +0x27c at +0x58 to +0x68 and the words at +0x294, +0x298 at +0x6c,
/// +0x70. Effect 4 gets +0x2a4, +0x2a8, +0x2ac and +0x2b0 at +0x70, +0x68, +0x6c and +0x78 and
/// the mode at +0x2c4 at +0x74 when it differs (its byte +5 set with each), then `00b8bc80`
/// (byte +0x2cc set) or `00b8bc20` runs on it. Effect 5 gets +0x2d0 at +0x6c; with x its +0x9c
/// and y the `float` at +0x2dc, +0x84 takes y, +0x7c takes y - m and +0x80 m + y, where m is y
/// when y is above x, else x; then +0x9c takes the `float` z at +0x2e0, +0x7c takes y - z and
/// +0x80 y + z; then `00b8b210` with the mode at +0x2ec, and +0x8c, +0x90, +0x94 take +0x2d0,
/// +0x2d4, +0x2d8. Effect 12 gets +0x2f0 at +0x58. Each effect's byte +5 is set as the exe sets
/// it.
pub fn fn_00b8d020(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    divide_by_last(e, base + 0x220);
    let weight = x87_quiet_bits(e.mem.u32(base + 0x234));
    e.mem.set_u32(base + 0x220, weight);
    copy_float(e, base + 0x214, base + 0x88);
    copy_float(e, base + 0x218, base + 0x8c);
    copy_float(e, base + 0x21c, base + 0x90);
    e.mem.set_u32(base + 0x94, weight);
    divide_by_last(e, base + 0x244);
    let weight = x87_quiet_bits(e.mem.u32(base + 0x258));
    e.mem.set_u32(base + 0x244, weight);
    copy_float(e, base + 0x238, base + 0x98);
    copy_float(e, base + 0x23c, base + 0x9c);
    copy_float(e, base + 0x240, base + 0xa0);
    e.mem.set_u32(base + 0xa4, weight);
    let value = x87_quiet_bits(e.mem.u32(base + 0x264));
    e.set_global::<u32>(VALUE_01202400, value);
    let value = x87_quiet_bits(e.mem.u32(base + 0x25c));
    e.set_global::<u32>(VALUE_012003D0, value);
    let effect = |e: &Engine, index: u32| e.mem.u32(e.mem.u32(base + EFFECTS) + 4 * index);
    let target = effect(e, 6);
    copy_float(e, base + 0x26c, target + 0x58);
    for (from, to) in [
        (0x270u32, 0x5cu32),
        (0x274, 0x60),
        (0x278, 0x64),
        (0x27c, 0x68),
    ] {
        copy_float(e, base + from, target + to);
    }
    let low = e.mem.u32(base + 0x294);
    e.mem.set_u32(target + 0x6c, low);
    let high = e.mem.u32(base + 0x298);
    e.mem.set_u32(target + 0x70, high);
    let target = effect(e, 4);
    copy_float(e, base + 0x2a4, target + 0x70);
    e.mem.set_u8(target + 5, 1);
    e.mem.set_u8(target + 5, 1);
    copy_float(e, base + 0x2a8, target + 0x68);
    e.mem.set_u8(target + 5, 1);
    copy_float(e, base + 0x2ac, target + 0x6c);
    copy_float(e, base + 0x2b0, target + 0x78);
    let mode = e.mem.u32(base + 0x2c4);
    if mode != e.mem.u32(target + 0x74) {
        e.mem.set_u8(target + 5, 1);
        e.mem.set_u32(target + 0x74, mode);
    }
    if e.mem.u8(base + 0x2cc) != 0 {
        e.call(SET_UP_00B8BC80, &args![target]);
    } else {
        fn_00b8bc20(e, Ptr::new(target));
    }
    let target = effect(e, 5);
    e.mem.set_u8(target + 5, 1);
    copy_float(e, base + 0x2d0, target + 0x6c);
    let y = x87_load32(x87_quiet_bits(e.mem.u32(base + 0x2dc)));
    let x_bits = x87_quiet_bits(e.mem.u32(target + 0x9c));
    let x = x87_load32(e.mem.u32(target + 0x9c));
    // `FCOM` of y with x, `TEST AH,0x41`: y kept only when it is above x.
    let middle_bits = if y > x { x87_store32(y) } else { x_bits };
    e.mem.set_u32(target + 0x84, x87_store32(y));
    e.mem.set_u8(target + 5, 1);
    let middle = x87_load32(middle_bits);
    e.mem
        .set_u32(target + 0x7c, x87_store32(x87_sub(y, middle)));
    e.mem
        .set_u32(target + 0x80, x87_store32(x87_add(middle, y)));
    e.mem.set_u8(target + 5, 1);
    let z = x87_load32(x87_quiet_bits(e.mem.u32(base + 0x2e0)));
    e.mem.set_u32(target + 0x9c, x87_store32(z));
    e.mem.set_u32(target + 0x7c, x87_store32(x87_sub(y, z)));
    e.mem.set_u32(target + 0x80, x87_store32(x87_add(y, z)));
    let mode = e.mem.u32(base + 0x2ec);
    fn_00b8b210(e, Ptr::new(target), mode);
    let first = x87_quiet_bits(e.mem.u32(base + 0x2d4));
    let second = x87_quiet_bits(e.mem.u32(base + 0x2d8));
    copy_float(e, base + 0x2d0, target + 0x8c);
    e.mem.set_u32(target + 0x90, first);
    e.mem.set_u32(target + 0x94, second);
    let target = effect(e, 0xc);
    let value = x87_quiet_bits(e.mem.u32(base + 0x2f0));
    e.mem.set_u8(target + 5, 1);
    e.mem.set_u32(target + 0x58, value);
}

// Translated from 00b8f160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: slot
/// +0x150 of the shader `00b8f010` constructs (vtable `010b2308`): the render states 0xa8 = 0
/// and 0xbe = 0xf on the pass at +0x5c (`render_states_on_pass`).
pub fn fn_00b8f160(e: &mut Engine, this: Ptr) {
    render_states_on_pass(e, this, &[(0xa8, 0), (0xbe, 0xf)]);
}

/// The word `00b8f2f0` sets as render state 0x39 (`011ae680`).
const STATE_VALUE_011AE680: u32 = 0x011a_e680;

// Translated from 00b8f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: slot
/// +0x150 of the shader `00b8f1b0` constructs (vtable `010b2490`): the render states 0x34 = 1
/// and 0x38 = 4, then 0x39 = the word at `011ae680` (read after the pass) on the pass at +0x5c.
pub fn fn_00b8f2f0(e: &mut Engine, this: Ptr) {
    render_states_on_pass(e, this, &[(0x34, 1), (0x38, 4)]);
    let pass = e.mem.u32(this.addr() + 0x5c);
    let value = e.global::<u32>(STATE_VALUE_011AE680);
    pass_render_state(e, pass, 0x39, value);
}

// Translated from 00b8f4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: slot
/// +0x150 of the shader `00b8f360` constructs (vtable `010b2610`): the render states 0xe = 0,
/// 0x34 = 1 and 0x38 = 3 on the pass at +0x5c (`render_states_on_pass`).
pub fn fn_00b8f4b0(e: &mut Engine, this: Ptr) {
    render_states_on_pass(e, this, &[(0xe, 0), (0x34, 1), (0x38, 3)]);
}

/// `NiD3DTextureStage::CreateNewStage` (`00e7edf0`, `cdecl`: the address of a stage pointer),
/// `BSShader::SetupDefaultStage` (`00be0cf0`, `cdecl`: stage, index, three words),
/// `NiD3DPass::SetStage` (`00e802b0`, `thiscall` on the pass: index, stage) and the pass maker
/// `00e804d0` (`cdecl`: the address of the pass pointer) (engine map names).
const CREATE_NEW_STAGE_00E7EDF0: u32 = 0x00e7_edf0;
const SETUP_DEFAULT_STAGE_00BE0CF0: u32 = 0x00be_0cf0;
const PASS_SET_STAGE_00E802B0: u32 = 0x00e8_02b0;
const NEW_PASS_00E804D0: u32 = 0x00e8_04d0;
/// Where `00b91f90` leaves its pass (`0126f74c`).
const LAST_PASS_0126F74C: u32 = 0x0126_f74c;

/// The pass loader `00b91f90`, `00b93420` and `00b93750` share, in the exe's order: a pass made
/// into the `NiPointer` at +0x5c (`00e804d0`); for each index below the 16-bit word at +0x9c
/// (read again after each), a stage made into one stack pointer (`CreateNewStage`, cleared once
/// before the loop), set up with the index and (`first(index, count)`, 2, 1) and set on the pass
/// at its word +0x14; the render states 7, 0xe, 0x1b and 0xf = 0 and 0xa8 = 0xf; the vtable slot
/// +0x150; the pass's vertex (+0x5c) and pixel (+0x44) shader `NiPointer`s assigned from +0xc4
/// and +0xc8; with `keep_pass`, the pass left at `0126f74c`; then the stage released. Answers 1.
fn load_stages_and_passes(
    e: &mut Engine,
    this: Ptr,
    first: fn(u32, u32) -> u32,
    keep_pass: bool,
) -> u8 {
    let base = this.addr();
    let pass_slot = base + 0x5c;
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        e.call(NEW_PASS_00E804D0, &args![pass_slot]);
        let mut index = 0u32;
        if e.mem.u16(base + 0x9c) != 0 {
            loop {
                e.call(CREATE_NEW_STAGE_00E7EDF0, &args![local]);
                let count = u32::from(e.mem.u16(base + 0x9c));
                let stage = e.mem.u32(local);
                let setup = first(index, count);
                e.call(
                    SETUP_DEFAULT_STAGE_00BE0CF0,
                    &args![stage, index, setup, 2u32, 1u32],
                );
                let pass = e.mem.u32(pass_slot);
                let stage = e.mem.u32(local);
                let position = e.mem.u32(pass + 0x14);
                e.call(PASS_SET_STAGE_00E802B0, &args![pass, position, stage]);
                index += 1;
                if index >= u32::from(e.mem.u16(base + 0x9c)) {
                    break;
                }
            }
        }
        render_states_on_pass(
            e,
            this,
            &[(7, 0), (0xe, 0), (0x1b, 0), (0xf, 0), (0xa8, 0xf)],
        );
        e.vcall(base, 0x150, &args![]);
        let pass = e.mem.u32(pass_slot);
        let shader = e.mem.u32(base + 0xc4);
        assign_pointer(e, pass + 0x5c, shader);
        let pass = e.mem.u32(pass_slot);
        let shader = e.mem.u32(base + 0xc8);
        assign_pointer(e, pass + 0x44, shader);
        if keep_pass {
            let pass = e.mem.u32(pass_slot);
            e.set_global::<u32>(LAST_PASS_0126F74C, pass);
        }
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
    1
}

// Translated from 00b91f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: slot
/// +0x14c of the shaders with the vtables `010b5480` and `010b5640`: `load_stages_and_passes`
/// with each stage set up with 1, the pass kept at `0126f74c`. Answers 1.
// Translation incomplete: 00b91f90 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00b91f90(e: &mut Engine, this: Ptr) -> u8 {
    load_stages_and_passes(e, this, |_, _| 1, true)
}

// Translated from 00b93420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: slot
/// +0x14c of the shaders with the vtables `010b6bd8`, `010b7418` and `010b7598`:
/// `load_stages_and_passes` with each stage set up with 1, the pass not kept. Answers 1.
// Translation incomplete: 00b93420 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00b93420(e: &mut Engine, this: Ptr) -> u8 {
    load_stages_and_passes(e, this, |_, _| 1, false)
}

// Translated from 00b93750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: slot
/// +0x14c of the shaders with the vtables `010b6d90`, `010b6f38`, `010b70d8` and `010b7278`:
/// `load_stages_and_passes` with each stage but the last set up with 3 and the last with 1 (the
/// index against the 16-bit count at +0x9c less one), the pass not kept. Answers 1.
// Translation incomplete: 00b93750 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00b93750(e: &mut Engine, this: Ptr) -> u8 {
    load_stages_and_passes(
        e,
        this,
        |index, count| if index < count.wrapping_sub(1) { 3 } else { 1 },
        false,
    )
}

// Translated from 00b97550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, six stack words: an
/// effect index and five words. Within the allocation context guard, runs `RenderEffect_ov2`
/// (`00b8c830`) with the effect array's element at the index and the five words.
// Translation incomplete: 00b97550 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation context) is not translated.
pub fn fn_00b97550(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    word: u32,
    first_source: u32,
    second_source: u32,
    fifth: u32,
    sixth: u32,
) {
    with_image_space_context(e, |e| {
        let block = e.mem.u32(this.addr() + EFFECTS);
        let effect = e.mem.u32(block.wrapping_add(index.wrapping_mul(4)));
        image_space_manager_render_effect_ov2(
            e,
            this,
            effect,
            word,
            first_source,
            second_source,
            fifth,
            sixth,
        );
    });
}

/// `00b8a7e0` (unnamed, unit `imagespaceshaderparam.cpp`; `thiscall` on `01202410`, no stack
/// words), `BSTextureManager::ReturnRenderedTexture` (`00b6da10`, `thiscall` on the object at
/// `011f91a8`, one word) and the Ni free `00aa10f0` (`cdecl`, the block) (engine map).
const SHADER_PARAM_00B8A7E0: u32 = 0x00b8_a7e0;
const RETURN_RENDERED_TEXTURE_00B6DA10: u32 = 0x00b6_da10;
const NI_FREE_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 00b97690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespacemanager.cpp`), `thiscall`, no stack words: the
/// manager's destructor (`ImageSpaceManager::Initialize`'s reverse). When the byte at +0x1e4
/// is set: `00b8b2e0`, `00b8c690`, `00b8a7e0` on `01202410`, the byte cleared and the
/// texture at +0x318 handed back (`00b6da10` on the object at `011f91a8`). Then
/// `~ImageSpaceTexture` (`00ba3a00`) on +0x204, the `NiPointer`s at +0x200, +0x1f8, +0x1f4 and
/// +0x1f0 released, the vtable `010b9d24` at +0x4 and the effect block (+0x8) freed through
/// `00aa10f0`.
// Translation incomplete: 00b97690 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5 for the members) is not translated.
pub fn fn_00b97690(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + INITIALIZED) != 0 {
        fn_00b8b2e0(e, this);
        fn_00b8c690(e, this);
        e.call(SHADER_PARAM_00B8A7E0, &args![CONSTANT_MAP_01202410]);
        let texture = e.mem.u32(base + 0x318);
        e.mem.set_u8(base + INITIALIZED, 0);
        let manager = e.global::<u32>(OBJECT_011F91A8);
        e.call(RETURN_RENDERED_TEXTURE_00B6DA10, &args![manager, texture]);
    }
    e.call(IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00, &args![base + 0x204]);
    for offset in [0x200u32, 0x1f8, 0x1f4, 0x1f0] {
        let object = e.mem.u32(base + offset);
        release_object(e, object);
    }
    let block = e.mem.u32(base + EFFECTS);
    e.mem.set_u32(base + 4, 0x010b_9d24);
    e.call(NI_FREE_00AA10F0, &args![block]);
}

/// The camera (`011f917c`) whose world position (`+0x8c` to `+0x94`) the distance-based
/// requests measure from, the screen projection `00b59d30` (`cdecl`: the point, the address
/// of the screen coordinates, the address of a `float` it writes), the CRT square root
/// (`00ec6040`, ST0 in and out), and the request setters `00b8b000` (`thiscall`, six words)
/// and `00b8b0d0` (`thiscall`, five words).
const CAMERA_011F917C: u32 = 0x011f_917c;
const PROJECT_00B59D30: u32 = 0x00b5_9d30;
const CRT_SQRT: u32 = 0x00ec_6040;
const REQUEST_00B8B000: u32 = 0x00b8_b000;
const REQUEST_00B8B0D0: u32 = 0x00b8_b0d0;
/// The two words `00b8b1e0` stores (`012023f4`, `012023f8`).
const PAIR_012023F4: u32 = 0x0120_23f4;
/// The run-time type record `00b8bd90` answers (`0126f554`).
const RTTI_0126F554: u32 = 0x0126_f554;
/// The set `00b8b270` (`thiscall` on the manager's `+0x58`, two words: index, the address of
/// a flag byte) `00b8bc80` calls four times.
const SET_FLAG_00B8B270: u32 = 0x00b8_b270;
/// `ImageSpaceTexture`'s set calls (`00ba37f0` and `00ba37a0`, `thiscall`, one word), its
/// destructor (`00ba3a00`), `ImageSpaceTexture::GetTexture` (`00ba3780`) and
/// `ImageSpaceEffect::SetTexture` (`00ba3cb0`, `thiscall`: index, texture, a third word).
const IMAGE_SPACE_TEXTURE_SET_00BA37F0: u32 = 0x00ba_37f0;
const IMAGE_SPACE_TEXTURE_SET_00BA37A0: u32 = 0x00ba_37a0;
const IMAGE_SPACE_TEXTURE_DESTRUCTOR: u32 = 0x00ba_3a00;
const IMAGE_SPACE_TEXTURE_GET_TEXTURE: u32 = 0x00ba_3780;
const EFFECT_SET_TEXTURE: u32 = 0x00ba_3cb0;
/// `"Interface Effect Partial"` (`010b04c8`) and `-2.0` (`double`, `0106d4c8`).
const PARTIAL_GEOMETRY_NAME_010B04C8: u32 = 0x010b_04c8;
const MINUS_TWO_0106D4C8: u32 = 0x0106_d4c8;

// Translated from 00b8ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceParameterData::Endian` (engine map; the body matches), `thiscall`: swaps the
/// byte order of the words 0 to 14 (`+0` to `+0x38`), the three at `+0x3c`, the three at
/// `+0x48`, the twelve at `+0x64` and the one at `+0x94` (the four at `+0x54` stay).
pub fn image_space_parameter_data_endian(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let swap = |e: &mut Engine, at: u32| {
        let v = e.mem.u32(at);
        e.mem.set_u32(at, v.swap_bytes());
    };
    for k in 0..15u32 {
        swap(e, base + 4 * k);
    }
    for k in 0..3u32 {
        swap(e, base + 0x3c + 4 * k);
    }
    for k in 0..3u32 {
        swap(e, base + 0x48 + 4 * k);
    }
    for k in 0..12u32 {
        swap(e, base + 0x64 + 4 * k);
    }
    swap(e, base + 0x94);
}

// Translated from 00b8b060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with seven words (the depth/radial blur request the
/// game's modifiers use without a target): only when the `float` at `+0x2d0` is below the
/// first `float` (NaN takes nothing), it takes the first `float` and the request: `+0x2d4`
/// and `+0x2d8` the fifth and sixth `float`s, `+0x2dc` and `+0x2e0` the third and fourth (all
/// through `FLD`/`FSTP`), `+0x2e4` and `+0x2e8` the two words at the seventh word's address
/// (the screen point) and `+0x2ec` the second word.
pub fn fn_00b8b060(
    e: &mut Engine,
    this: Ptr,
    strength: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    point: u32,
) {
    let base = this.addr();
    if !(x87_m32(e.mem.u32(base + 0x2d0)) < x87_m32(strength)) {
        return;
    }
    e.mem.set_u32(base + 0x2d0, x87_quiet_bits(strength));
    e.mem.set_u32(base + 0x2d4, x87_quiet_bits(fifth));
    e.mem.set_u32(base + 0x2d8, x87_quiet_bits(sixth));
    let x = e.mem.u32(point);
    e.mem.set_u32(base + 0x2dc, x87_quiet_bits(third));
    e.mem.set_u32(base + 0x2e4, x);
    let y = e.mem.u32(point.wrapping_add(4));
    e.mem.set_u32(base + 0x2e0, x87_quiet_bits(fourth));
    e.mem.set_u32(base + 0x2e8, y);
    e.mem.set_u32(base + 0x2ec, second);
}

// Translated from 00b8b1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with two words (`this` not read): the screen
/// projection `00b59d30` of the first word with the second and the address of the second's
/// own stack word.
pub fn fn_00b8b1c0(e: &mut Engine, _this: u32, point: u32, screen: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), screen);
        e.call(PROJECT_00B59D30, &args![point, screen, slot]);
    });
}

// Translated from 00b8b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with one word (`this` not read): copies the two words
/// at that address to `012023f4` and `012023f8`.
pub fn fn_00b8b1e0(e: &mut Engine, _this: u32, pair: u32) {
    let v = e.mem.u32(pair);
    e.mem.set_u32(PAIR_012023F4, v);
    let v = e.mem.u32(pair.wrapping_add(4));
    e.mem.set_u32(PAIR_012023F4 + 4, v);
}

// Translated from 00b8b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` without words: for each non-null effect pointer from
/// `+0x88` to `+0x1cc` of the effect table (`+8`), calls slot +0x120 of the object 0x6c
/// before it (of null when the pointer is null).
pub fn fn_00b8b3b0(e: &mut Engine, this: Ptr) {
    let mut offset = 0x88u32;
    while offset < 0x1d0 {
        let slot = e.mem.u32(this.addr() + EFFECTS).wrapping_add(offset);
        if e.mem.u32(slot) != 0 {
            let effect = e.mem.u32(slot);
            let shader = if effect != 0 {
                effect.wrapping_sub(0x6c)
            } else {
                0
            };
            e.vcall(shader, 0x120, &args![]);
        }
        offset += 4;
    }
}

/// The squared distance the distance-based requests compute: each coordinate difference to
/// the camera stored as a `float` (`FSUB`, `FSTP`), then `dy*dy + dx*dx`, plus `dz*dz`, in
/// that order (one rounding per x87 instruction).
fn camera_distance_squared(e: &Engine, point: u32) -> f64 {
    let camera = e.mem.u32(CAMERA_011F917C);
    let mut d = [0u32; 3];
    for (k, v) in d.iter_mut().enumerate() {
        let offset = 4 * k as u32;
        *v = x87_store32(x87_sub(
            x87_m32(e.mem.u32(point.wrapping_add(offset))),
            x87_m32(e.mem.u32(camera.wrapping_add(0x8c + offset))),
        ));
    }
    let dx = x87_load32(d[0]);
    let dy = x87_load32(d[1]);
    let dz = x87_load32(d[2]);
    x87_add(x87_add(x87_mul(dy, dy), x87_mul(dx, dx)), x87_mul(dz, dz))
}

/// The camera distance of `point` as the requests take it: the squared distance stored as a
/// `float`, its square root (`00ec6040`) stored as a `float`.
fn camera_distance(e: &mut Engine, point: u32) -> u32 {
    let squared = x87_store32(camera_distance_squared(e, point));
    let root = e.call(CRT_SQRT, &args![x87_load32(squared)]).f64();
    x87_store32(root)
}

// Translated from 00b8b760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with six words (the radial blur request with a
/// target): projects the point (sixth word, `00b59d30`, the screen coordinates into a local
/// and a `float` into the sixth word's stack slot), then hands `00b8b000` the first two
/// words, that `float` plus the third, the fourth, that `float` plus the fifth (each sum
/// stored as a `float`) and the address of the screen coordinates.
pub fn fn_00b8b760(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    point: u32,
) {
    e.with_stack(0xc, |e, locals| {
        let screen = locals.addr();
        let slot = screen + 8;
        e.mem.set_u32(slot, point);
        e.call(PROJECT_00B59D30, &args![point, screen, slot]);
        let depth = x87_m32(e.mem.u32(slot));
        let far = x87_store32(x87_add(depth, x87_m32(fifth)));
        let near = x87_store32(x87_add(depth, x87_m32(third)));
        e.call(
            REQUEST_00B8B000,
            &args![
                this,
                x87_quiet_bits(first),
                x87_quiet_bits(second),
                near,
                x87_quiet_bits(fourth),
                far,
                screen
            ],
        );
    });
}

// Translated from 00b8b7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with five words (the depth/radial blur request with a
/// target): the camera distance of the point (fifth word, [`camera_distance`]), the point
/// projected (`00b59d30`, the screen coordinates into a local and a `float` into the fifth
/// word's stack slot), then [`fn_00b8b060`] with the first two words, the distance twice, the
/// third word, that `float` plus the fourth (stored as a `float`) and the screen coordinates.
pub fn fn_00b8b7d0(
    e: &mut Engine,
    this: Ptr,
    strength: u32,
    second: u32,
    third: u32,
    fourth: u32,
    point: u32,
) {
    let distance = camera_distance(e, point);
    e.with_stack(0xc, |e, locals| {
        let screen = locals.addr();
        let slot = screen + 8;
        e.mem.set_u32(slot, distance);
        e.call(PROJECT_00B59D30, &args![point, screen, slot]);
        let depth = x87_store32(x87_add(x87_m32(e.mem.u32(slot)), x87_m32(fourth)));
        let distance = x87_quiet_bits(distance);
        fn_00b8b060(
            e,
            this,
            x87_quiet_bits(strength),
            second,
            distance,
            distance,
            x87_quiet_bits(third),
            x87_quiet_bits(depth),
            screen,
        );
    });
}

// Translated from 00b8b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with four words (the depth of field request with a
/// target): `00b8b0d0` with the first two words, the third and the camera distance of the
/// point (fourth word, [`camera_distance`]) twice (all `float`s through `FLD`/`FSTP`).
pub fn fn_00b8b8d0(e: &mut Engine, this: Ptr, strength: u32, second: u32, third: u32, point: u32) {
    let distance = x87_quiet_bits(camera_distance(e, point));
    e.call(
        REQUEST_00B8B0D0,
        &args![
            this,
            x87_quiet_bits(strength),
            second,
            x87_quiet_bits(third),
            distance,
            distance
        ],
    );
}

// Translated from 00b8bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with two words (`this` not read; a point and the
/// address of two `float`s): both `float`s become the point's camera distance
/// ([`camera_distance`], copied through `FLD`/`FSTP` on the way).
pub fn fn_00b8bb70(e: &mut Engine, _this: u32, point: u32, out: u32) {
    let distance = x87_quiet_bits(x87_quiet_bits(camera_distance(e, point)));
    e.mem.set_u32(out, distance);
    e.mem.set_u32(out.wrapping_add(4), distance);
}

// Translated from 00b8bc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` without words: `00b8b270` on the manager's `+0x58`
/// for the indices 0 to 3 with the address of a local flag byte holding 1, 1, 0 and 1.
pub fn fn_00b8bc80(e: &mut Engine, this: Ptr) {
    let target = this.addr() + 0x58;
    e.with_stack(4, |e, flag| {
        for (index, value) in [1u8, 1, 0, 1].iter().enumerate() {
            e.mem.set_u8(flag.addr(), *value);
            e.call(SET_FLAG_00B8B270, &args![target, index as u32, flag]);
        }
    });
}

// Translated from 00b8bd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, no arguments: answers the run-time type record at `0126f554`
/// (the one `BSShaderManager`'s `00b557d0` looks for).
pub fn fn_00b8bd90(_e: &mut Engine) -> u32 {
    RTTI_0126F554
}

/// An `ImageSpaceTexture` local (0x10 bytes: three zero bytes and three zero words).
fn image_space_texture_local(e: &mut Engine, texture: u32) {
    for k in 0..3u32 {
        e.mem.set_u8(texture + k, 0);
    }
    for k in 1..4u32 {
        e.mem.set_u32(texture + 4 * k, 0);
    }
}

/// The geometry the effects render with: `+0x1f8` (the partial screen geometry) when the byte
/// `+0x1fc` is set, else `+0x1f0`.
fn effect_geometry(e: &Engine, this: u32) -> u32 {
    if e.mem.u8(this + 0x1fc) != 0 {
        e.mem.u32(this + 0x1f8)
    } else {
        e.mem.u32(this + 0x1f0)
    }
}

// Translated from 00b8c3b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b8c3b0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `ImageSpaceManager::CreatePartialScreenGeometry` (engine map; the body matches):
/// `thiscall`, three words (a width and a height as `float`s, the address of an `NiPointer`).
/// With the thread's allocation context at 0x27: an `NiScreenElementsData` (0x70 bytes, 0, 0,
/// 1, 1, 1, 4, 1, 2, 1; held in a local `NiPointer`) and an `NiScreenElements` on it (0xc4
/// bytes) into `+0x1f8`; on the elements' data `Insert(4, 0, 0)`, `SetRectangle(0, -1.0, 1.0,
/// 2 * width, height * -2.0)` (the -1.0 at `01012054`, the -2.0 `double` at `0106d4c8`),
/// `UpdateBound` and `SetTextures(0, 0, 0.0, 0.0, width, height)`; `UpdateProperties`, the
/// update `00a59c60` (a 0.0 `float` and five clear bytes) and the name `"Interface Effect
/// Partial"`; then the `NiPointer` argument takes the elements and the local data pointer is
/// released before the context is put back.
pub fn image_space_manager_create_partial_screen_geometry(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    result: u32,
) {
    let base = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, CONTEXT_IMAGE_SPACE);
    let block = e.call(NI_ALLOCATE, &args![0x70u32]).u32();
    let data = if block != 0 {
        e.call(
            SCREEN_ELEMENTS_DATA_CONSTRUCT,
            &args![block, 0u32, 0u32, 1u32, 1u32, 1u32, 4u32, 1u32, 2u32, 1u32],
        )
        .u32()
    } else {
        0
    };
    if data != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![data + 4]);
    }
    let block = e.call(NI_ALLOCATE, &args![0xc4u32]).u32();
    let elements = if block != 0 {
        e.call(SCREEN_ELEMENTS_CONSTRUCT, &args![block, data]).u32()
    } else {
        0
    };
    assign_pointer(e, base + 0x1f8, elements);
    let elements_data = |e: &Engine| e.mem.u32(e.mem.u32(base + 0x1f8) + 0xb8);
    let double_width = x87_store32(x87_add(x87_m32(width), x87_m32(width)));
    let minus_two: f64 = e.global(MINUS_TWO_0106D4C8);
    let scaled_height = x87_store32(x87_mul(x87_m32(height), minus_two));
    let target = elements_data(e);
    e.call(
        SCREEN_ELEMENTS_DATA_INSERT,
        &args![target, 4u32, 0u32, 0u32],
    );
    let target = elements_data(e);
    let x = x87_quiet_bits(e.global::<u32>(RECTANGLE_X_01012054));
    e.call(
        SCREEN_ELEMENTS_DATA_SET_RECTANGLE,
        &args![
            target,
            0u32,
            x,
            x87_store32(1.0),
            x87_quiet_bits(double_width),
            x87_quiet_bits(scaled_height)
        ],
    );
    let target = elements_data(e);
    e.call(SCREEN_ELEMENTS_DATA_UPDATE_BOUND, &args![target]);
    let target = elements_data(e);
    e.call(
        SCREEN_ELEMENTS_DATA_SET_TEXTURES,
        &args![
            target,
            0u32,
            0u32,
            x87_store32(0.0),
            x87_store32(0.0),
            x87_quiet_bits(width),
            x87_quiet_bits(height)
        ],
    );
    let elements = e.mem.u32(base + 0x1f8);
    e.call(UPDATE_PROPERTIES, &args![elements]);
    e.with_stack(0x10, |e, locals| {
        let update = locals.addr();
        let name_slot = update + 0xc;
        let elements = e.mem.u32(base + 0x1f8);
        e.mem.set_u32(update, x87_store32(0.0));
        for offset in 4..9u32 {
            e.mem.set_u8(update + offset, 0);
        }
        e.call(UPDATE_00A59C60, &args![elements, update]);
        let name = e
            .call(
                MAKE_FIXED_STRING_00A5B690,
                &args![PARTIAL_GEOMETRY_NAME_010B04C8],
            )
            .u32();
        e.mem.set_u32(name_slot, name);
        let elements = e.mem.u32(base + 0x1f8);
        e.call(SET_NAME_00A5B950, &args![elements, name_slot]);
        let name = e.mem.u32(name_slot);
        if name != e.global::<u32>(NULL_NAME_0109B220) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
        }
    });
    let elements = e.mem.u32(base + 0x1f8);
    assign_pointer(e, result, elements);
    if data != 0 {
        let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![data + 4]).u32();
        if left == 0 {
            e.vcall(data, 4, &args![]);
        }
    }
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
}

// Translated from 00b8c730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b8c730 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `ImageSpaceManager::RenderEffect` (engine map; the body matches): `thiscall`, five words (an
/// effect index, a renderer word, a texture, two more words). With the thread's allocation
/// context at 0x27, a local `ImageSpaceTexture` takes the texture (`00ba37a0`); when the
/// effect (`+8` table) exists it gets the local as texture 0 (`00ba3cb0`), is rendered (its
/// slot +4 with the effect geometry ([`effect_geometry`]), the second, fourth and fifth
/// words) and has texture 0 cleared. The local is destroyed (`00ba3a00`) and the context put
/// back.
pub fn image_space_manager_render_effect(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    renderer: u32,
    texture: u32,
    fourth: u32,
    fifth: u32,
) {
    let base = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, CONTEXT_IMAGE_SPACE);
    let table = e.mem.u32(base + EFFECTS);
    let effect = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    e.with_stack(0x10, |e, local| {
        let local = local.addr();
        image_space_texture_local(e, local);
        e.call(IMAGE_SPACE_TEXTURE_SET_00BA37A0, &args![local, texture]);
        if effect != 0 {
            e.call(EFFECT_SET_TEXTURE, &args![effect, 0u32, local, 0u32]);
            let geometry = effect_geometry(e, base);
            e.vcall(effect, 4, &args![geometry, renderer, fourth, fifth]);
            e.call(EFFECT_SET_TEXTURE, &args![effect, 0u32, 0u32, 0u32]);
        }
        e.call(IMAGE_SPACE_TEXTURE_DESTRUCTOR, &args![local]);
    });
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
}

// Translated from 00b8c980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b8c980 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, `thiscall` with six words (an effect, a renderer word, two
/// textures, two more words): the two-texture `RenderEffect`. With the thread's allocation
/// context at 0x27, two local `ImageSpaceTexture`s take the textures (the first through
/// `00ba37f0`, the second through `00ba37a0`); when the effect is not null it gets the second
/// local as texture 0 and the first as texture 1 (`00ba3cb0`), is rendered (slot +4 with the
/// effect geometry, the second, fifth and sixth words) and has both textures cleared. Both
/// locals are destroyed (`00ba3a00`, the second first) and the context put back.
pub fn fn_00b8c980(
    e: &mut Engine,
    this: Ptr,
    effect: u32,
    renderer: u32,
    first: u32,
    second: u32,
    fifth: u32,
    sixth: u32,
) {
    let base = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, CONTEXT_IMAGE_SPACE);
    e.with_stack(0x20, |e, locals| {
        let one = locals.addr();
        let two = one + 0x10;
        image_space_texture_local(e, one);
        image_space_texture_local(e, two);
        e.call(IMAGE_SPACE_TEXTURE_SET_00BA37F0, &args![one, first]);
        e.call(IMAGE_SPACE_TEXTURE_SET_00BA37A0, &args![two, second]);
        if effect != 0 {
            e.call(EFFECT_SET_TEXTURE, &args![effect, 0u32, two, 0u32]);
            e.call(EFFECT_SET_TEXTURE, &args![effect, 1u32, one, 0u32]);
            let geometry = effect_geometry(e, base);
            e.vcall(effect, 4, &args![geometry, renderer, fifth, sixth]);
            e.call(EFFECT_SET_TEXTURE, &args![effect, 0u32, 0u32, 0u32]);
            e.call(EFFECT_SET_TEXTURE, &args![effect, 1u32, 0u32, 0u32]);
        }
        e.call(IMAGE_SPACE_TEXTURE_DESTRUCTOR, &args![two]);
        e.call(IMAGE_SPACE_TEXTURE_DESTRUCTOR, &args![one]);
    });
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
}

// Translated from 00b8cb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with one word: stores it at `+0xb0`; the thread's
/// allocation context word is read before and written back unchanged after.
pub fn fn_00b8cb00(e: &mut Engine, this: Ptr, value: u32) {
    let tls = e.tls();
    let context = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(this.addr() + 0xb0, value);
    e.mem.set_u32(tls + TLS_CONTEXT, context);
}

// Translated from 00b8d440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; slot 6 of the vtable at `010b04f4`), `thiscall`
/// without words: 1 when the effect's texture 2 (`pTextures`, buffer at `+0x2c`) exists and
/// has a texture (`ImageSpaceTexture::GetTexture`), else 0.
pub fn fn_00b8d440(e: &mut Engine, this: Ptr) -> u32 {
    let textures = e.mem.u32(this.addr() + 0x2c);
    let texture = e.mem.u32(textures.wrapping_add(8));
    if texture == 0 {
        return 0;
    }
    let held = e.mem.u32(textures.wrapping_add(8));
    if e.call(IMAGE_SPACE_TEXTURE_GET_TEXTURE, &args![held]).u32() != 0 {
        1
    } else {
        0
    }
}

// Translated from 00b975f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b975f0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, `thiscall` with six words (an effect index, a renderer word, two
/// textures, two more words): with the thread's allocation context at 0x27, [`fn_00b8c980`]
/// with the indexed effect (`+8` table) and the other five words; the context put back.
pub fn fn_00b975f0(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    renderer: u32,
    first: u32,
    second: u32,
    fifth: u32,
    sixth: u32,
) {
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, CONTEXT_IMAGE_SPACE);
    let table = e.mem.u32(this.addr() + EFFECTS);
    let effect = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    fn_00b8c980(e, this, effect, renderer, first, second, fifth, sixth);
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b9_4520, fn_00b94520(Ptr)),
        entry!(0x00b8_c170, image_space_manager_init_geometry(Ptr)),
        entry!(0x00b8_ad70, fn_00b8ad70()),
        entry!(0x00b8_b360, image_space_manager_setup_effects(Ptr)),
        entry!(0x00b9_77d0, image_space_manager_initialize(Ptr)),
        entry!(0x00b9_22d0, fn_00b922d0(Ptr) -> Ptr),
        entry!(0x00b8_bce0, fn_00b8bce0(Ptr) -> Ptr),
        entry!(0x00b8_bea0, fn_00b8bea0(Ptr) -> Ptr),
        entry!(0x00b8_c070, fn_00b8c070(Ptr) -> Ptr),
        entry!(0x00b8_d330, fn_00b8d330(Ptr) -> Ptr),
        entry!(0x00b8_d5b0, fn_00b8d5b0(Ptr) -> Ptr),
        entry!(0x00b8_d700, fn_00b8d700(Ptr) -> Ptr),
        entry!(0x00b8_d850, fn_00b8d850(Ptr) -> Ptr),
        entry!(0x00b8_d990, fn_00b8d990(Ptr) -> Ptr),
        entry!(0x00b8_dae0, fn_00b8dae0(Ptr) -> Ptr),
        entry!(0x00b8_dc30, fn_00b8dc30(Ptr) -> Ptr),
        entry!(0x00b8_dd80, fn_00b8dd80(Ptr) -> Ptr),
        entry!(0x00b8_ded0, fn_00b8ded0(Ptr) -> Ptr),
        entry!(0x00b8_e020, fn_00b8e020(Ptr) -> Ptr),
        entry!(0x00b8_e170, fn_00b8e170(Ptr) -> Ptr),
        entry!(0x00b8_e2c0, fn_00b8e2c0(Ptr) -> Ptr),
        entry!(0x00b8_e460, fn_00b8e460(Ptr) -> Ptr),
        entry!(0x00b8_e600, fn_00b8e600(Ptr) -> Ptr),
        entry!(0x00b8_e7a0, fn_00b8e7a0(Ptr) -> Ptr),
        entry!(0x00b8_e940, fn_00b8e940(Ptr) -> Ptr),
        entry!(0x00b8_eae0, fn_00b8eae0(Ptr) -> Ptr),
        entry!(0x00b8_ec80, fn_00b8ec80(Ptr) -> Ptr),
        entry!(0x00b8_ee20, fn_00b8ee20(Ptr) -> Ptr),
        entry!(0x00b8_efa0, fn_00b8efa0(Ptr) -> Ptr),
        entry!(0x00b8_f010, fn_00b8f010(Ptr) -> Ptr),
        entry!(0x00b8_f1b0, fn_00b8f1b0(Ptr) -> Ptr),
        entry!(0x00b8_f360, fn_00b8f360(Ptr) -> Ptr),
        entry!(0x00b8_f520, fn_00b8f520(Ptr) -> Ptr),
        entry!(0x00b8_f660, fn_00b8f660(Ptr) -> Ptr),
        entry!(0x00b8_f7e0, fn_00b8f7e0(Ptr) -> Ptr),
        entry!(0x00b8_f930, fn_00b8f930(Ptr) -> Ptr),
        entry!(0x00b8_fa80, fn_00b8fa80(Ptr) -> Ptr),
        entry!(0x00b8_fbe0, fn_00b8fbe0(Ptr) -> Ptr),
        entry!(0x00b8_fd40, fn_00b8fd40(Ptr) -> Ptr),
        entry!(0x00b8_fea0, fn_00b8fea0(Ptr) -> Ptr),
        entry!(0x00b8_fff0, fn_00b8fff0(Ptr) -> Ptr),
        entry!(0x00b9_0190, fn_00b90190(Ptr) -> Ptr),
        entry!(0x00b9_0330, fn_00b90330(Ptr) -> Ptr),
        entry!(0x00b9_0520, fn_00b90520(Ptr) -> Ptr),
        entry!(0x00b9_06b0, fn_00b906b0(Ptr) -> Ptr),
        entry!(0x00b9_0830, fn_00b90830(Ptr) -> Ptr),
        entry!(0x00b9_09c0, fn_00b909c0(Ptr) -> Ptr),
        entry!(0x00b9_0b50, fn_00b90b50(Ptr) -> Ptr),
        entry!(0x00b9_0ce0, fn_00b90ce0(Ptr) -> Ptr),
        entry!(0x00b9_0e70, fn_00b90e70(Ptr) -> Ptr),
        entry!(0x00b9_1000, fn_00b91000(Ptr) -> Ptr),
        entry!(0x00b9_11a0, fn_00b911a0(Ptr) -> Ptr),
        entry!(0x00b9_1340, fn_00b91340(Ptr) -> Ptr),
        entry!(0x00b9_14e0, fn_00b914e0(Ptr) -> Ptr),
        entry!(0x00b9_1680, fn_00b91680(Ptr) -> Ptr),
        entry!(0x00b9_17c0, fn_00b917c0(Ptr) -> Ptr),
        entry!(0x00b9_1900, fn_00b91900(Ptr) -> Ptr),
        entry!(0x00b9_1a40, fn_00b91a40(Ptr) -> Ptr),
        entry!(0x00b9_1b80, fn_00b91b80(Ptr) -> Ptr),
        entry!(0x00b9_1cf0, fn_00b91cf0(Ptr) -> Ptr),
        entry!(0x00b9_1e40, fn_00b91e40(Ptr) -> Ptr),
        entry!(0x00b9_2180, fn_00b92180(Ptr) -> Ptr),
        entry!(0x00b9_2500, fn_00b92500(Ptr) -> Ptr),
        entry!(0x00b9_2690, fn_00b92690(Ptr) -> Ptr),
        entry!(0x00b9_27e0, fn_00b927e0(Ptr) -> Ptr),
        entry!(0x00b9_2930, fn_00b92930(Ptr) -> Ptr),
        entry!(0x00b9_2a80, fn_00b92a80(Ptr) -> Ptr),
        entry!(0x00b9_2bd0, fn_00b92bd0(Ptr) -> Ptr),
        entry!(0x00b9_2d90, fn_00b92d90(Ptr) -> Ptr),
        entry!(0x00b9_2ee0, fn_00b92ee0(Ptr) -> Ptr),
        entry!(0x00b9_3030, fn_00b93030(Ptr) -> Ptr),
        entry!(0x00b9_3180, fn_00b93180(Ptr) -> Ptr),
        entry!(0x00b9_32d0, fn_00b932d0(Ptr) -> Ptr),
        entry!(0x00b9_3600, fn_00b93600(Ptr) -> Ptr),
        entry!(0x00b9_3950, fn_00b93950(Ptr) -> Ptr),
        entry!(0x00b9_3aa0, fn_00b93aa0(Ptr) -> Ptr),
        entry!(0x00b9_3bf0, fn_00b93bf0(Ptr) -> Ptr),
        entry!(0x00b9_3d40, fn_00b93d40(Ptr) -> Ptr),
        entry!(0x00b9_3e90, fn_00b93e90(Ptr) -> Ptr),
        entry!(0x00b9_3fe0, fn_00b93fe0(Ptr) -> Ptr),
        entry!(0x00b9_4130, fn_00b94130(Ptr) -> Ptr),
        entry!(0x00b9_4280, fn_00b94280(Ptr) -> Ptr),
        entry!(0x00b9_43d0, fn_00b943d0(Ptr) -> Ptr),
        entry!(0x00b8_bc20, fn_00b8bc20(Ptr)),
        entry!(0x00b8_b270, fn_00b8b270(Ptr, u32, Ptr)),
        entry!(0x00b9_1b50, fn_00b91b50(Ptr, u32) -> Ptr),
        entry!(0x00b9_1b40, fn_00b91b40(Ptr, u32) -> Ptr),
        entry!(0x00b9_2450, fn_00b92450(Ptr)),
        entry!(0x00b9_24d0, fn_00b924d0(Ptr, u32) -> Ptr),
        entry!(0x00b9_2440, fn_00b92440(Ptr, u32) -> Ptr),
        entry!(
            0x00b8_d4d0,
            image_space_shader_volumetric_fog_destructor(Ptr)
        ),
        entry!(0x00b8_d580, fn_00b8d580(Ptr, u32) -> Ptr),
        entry!(0x00b8_d430, fn_00b8d430(Ptr, u32) -> Ptr),
        entry!(0x00b8_bda0, fn_00b8bda0(Ptr) -> u8),
        entry!(0x00b8_d460, fn_00b8d460(Ptr)),
        entry!(0x00b9_2d20, fn_00b92d20(Ptr)),
        entry!(0x00b8_aea0, fn_00b8aea0(Ptr)),
        entry!(0x00b8_af10, fn_00b8af10(Ptr, f32)),
        entry!(0x00b8_af40, fn_00b8af40(Ptr, f32)),
        entry!(0x00b8_af60, fn_00b8af60(Ptr, f32)),
        entry!(0x00b8_af90, fn_00b8af90(Ptr, f32)),
        entry!(0x00b8_afb0, fn_00b8afb0(Ptr, f32)),
        entry!(0x00b8_afe0, fn_00b8afe0(Ptr, f32)),
        entry!(0x00b8_b000, fn_00b8b000(Ptr, f32, f32, f32, f32, f32, Ptr)),
        entry!(0x00b8_b0d0, fn_00b8b0d0(Ptr, f32, u32, f32, f32, f32)),
        entry!(0x00b8_b130, fn_00b8b130(Ptr, f32, u32, f32, f32, f32)),
        entry!(0x00b8_b190, image_space_manager_get_depth_rendered_texture(Ptr) -> u32),
        entry!(0x00b8_b200, fn_00b8b200(Ptr, u32)),
        entry!(0x00b8_b210, fn_00b8b210(Ptr, u32)),
        entry!(0x00b8_b2e0, fn_00b8b2e0(Ptr)),
        entry!(0x00b8_b390, fn_00b8b390(Ptr, u32) -> u8),
        entry!(0x00b8_b3f0, fn_00b8b3f0(Ptr)),
        entry!(0x00b8_b440, fn_00b8b440(Ptr, u32)),
        entry!(0x00b8_b500, fn_00b8b500(Ptr)),
        entry!(0x00b8_b540, fn_00b8b540(Ptr, Ptr)),
        entry!(0x00b8_b5c0, fn_00b8b5c0(Ptr, Ptr)),
        entry!(0x00b8_b610, fn_00b8b610(Ptr, Ptr)),
        entry!(0x00b8_b690, fn_00b8b690(Ptr, Ptr)),
        entry!(0x00b8_b6e0, fn_00b8b6e0(Ptr, f32, f32, f32, f32, f32, Ptr)),
        entry!(0x00b8_b9a0, fn_00b8b9a0(Ptr)),
        entry!(0x00b8_bfe0, fn_00b8bfe0(Ptr)),
        entry!(0x00b8_c690, fn_00b8c690(Ptr)),
        entry!(
            0x00b8_c830,
            image_space_manager_render_effect_ov2(Ptr, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00b8_cad0, fn_00b8cad0(Ptr, u32)),
        entry!(0x00b8_cb30, fn_00b8cb30(Ptr, u32, u32, Ptr, f32)),
        entry!(0x00b8_cc20, fn_00b8cc20(Ptr, u32, u32, Ptr, f32)),
        entry!(0x00b8_ccb0, fn_00b8ccb0(Ptr)),
        entry!(0x00b8_d020, fn_00b8d020(Ptr)),
        entry!(0x00b8_f160, fn_00b8f160(Ptr)),
        entry!(0x00b8_f2f0, fn_00b8f2f0(Ptr)),
        entry!(0x00b8_f4b0, fn_00b8f4b0(Ptr)),
        entry!(0x00b9_1f90, fn_00b91f90(Ptr) -> u8),
        entry!(0x00b9_3420, fn_00b93420(Ptr) -> u8),
        entry!(0x00b9_3750, fn_00b93750(Ptr) -> u8),
        entry!(0x00b9_7550, fn_00b97550(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x00b9_7690, fn_00b97690(Ptr)),
        entry!(0x00b8_bdb0, fn_00b8bdb0(Ptr)),
        entry!(0x00b8_bf10, fn_00b8bf10(Ptr)),
        entry!(0x00b8_c0e0, fn_00b8c0e0(Ptr)),
        entry!(0x00b8_d3a0, fn_00b8d3a0(Ptr)),
        entry!(0x00b8_d620, fn_00b8d620(Ptr)),
        entry!(0x00b8_d770, fn_00b8d770(Ptr)),
        entry!(0x00b8_d8c0, fn_00b8d8c0(Ptr)),
        entry!(0x00b8_da00, fn_00b8da00(Ptr)),
        entry!(0x00b8_db50, fn_00b8db50(Ptr)),
        entry!(0x00b8_dca0, fn_00b8dca0(Ptr)),
        entry!(0x00b8_ddf0, fn_00b8ddf0(Ptr)),
        entry!(0x00b8_df40, fn_00b8df40(Ptr)),
        entry!(0x00b8_e090, fn_00b8e090(Ptr)),
        entry!(0x00b8_e1e0, fn_00b8e1e0(Ptr)),
        entry!(0x00b8_ee90, fn_00b8ee90(Ptr)),
        entry!(0x00b8_f080, fn_00b8f080(Ptr)),
        entry!(0x00b8_f220, fn_00b8f220(Ptr)),
        entry!(0x00b8_f3d0, fn_00b8f3d0(Ptr)),
        entry!(0x00b8_f590, fn_00b8f590(Ptr)),
        entry!(0x00b8_f710, fn_00b8f710(Ptr)),
        entry!(0x00b8_f850, fn_00b8f850(Ptr)),
        entry!(0x00b8_f9a0, fn_00b8f9a0(Ptr)),
        entry!(0x00b8_faf0, fn_00b8faf0(Ptr)),
        entry!(0x00b8_fc50, fn_00b8fc50(Ptr)),
        entry!(0x00b8_fdb0, fn_00b8fdb0(Ptr)),
        entry!(0x00b8_ff10, fn_00b8ff10(Ptr)),
        entry!(0x00b9_16f0, fn_00b916f0(Ptr)),
        entry!(0x00b9_1830, fn_00b91830(Ptr)),
        entry!(0x00b9_1970, fn_00b91970(Ptr)),
        entry!(0x00b9_1ab0, fn_00b91ab0(Ptr)),
        entry!(0x00b9_1bf0, fn_00b91bf0(Ptr)),
        entry!(0x00b9_1d60, fn_00b91d60(Ptr)),
        entry!(0x00b9_1eb0, fn_00b91eb0(Ptr)),
        entry!(0x00b9_21f0, fn_00b921f0(Ptr)),
        entry!(0x00b9_2390, fn_00b92390(Ptr)),
        entry!(0x00b9_25b0, fn_00b925b0(Ptr)),
        entry!(0x00b9_2700, fn_00b92700(Ptr)),
        entry!(0x00b9_2850, fn_00b92850(Ptr)),
        entry!(0x00b9_29a0, fn_00b929a0(Ptr)),
        entry!(0x00b9_2af0, fn_00b92af0(Ptr)),
        entry!(0x00b9_2c40, fn_00b92c40(Ptr)),
        entry!(0x00b9_2e00, fn_00b92e00(Ptr)),
        entry!(0x00b9_2f50, fn_00b92f50(Ptr)),
        entry!(0x00b9_30a0, fn_00b930a0(Ptr)),
        entry!(0x00b9_31f0, fn_00b931f0(Ptr)),
        entry!(0x00b9_3340, fn_00b93340(Ptr)),
        entry!(0x00b9_3670, fn_00b93670(Ptr)),
        entry!(0x00b9_39c0, fn_00b939c0(Ptr)),
        entry!(0x00b9_3b10, fn_00b93b10(Ptr)),
        entry!(0x00b9_3c60, fn_00b93c60(Ptr)),
        entry!(0x00b9_3db0, fn_00b93db0(Ptr)),
        entry!(0x00b9_3f00, fn_00b93f00(Ptr)),
        entry!(0x00b9_4050, fn_00b94050(Ptr)),
        entry!(0x00b9_41a0, fn_00b941a0(Ptr)),
        entry!(0x00b9_42f0, fn_00b942f0(Ptr)),
        entry!(0x00b9_4440, fn_00b94440(Ptr)),
        entry!(
            0x00b8_e330,
            image_space_shader_bright_pass_blur3_reload_shaders(Ptr)
        ),
        entry!(
            0x00b8_e4d0,
            image_space_shader_bright_pass_blur5_reload_shaders(Ptr)
        ),
        entry!(
            0x00b8_e670,
            image_space_shader_bright_pass_blur7_reload_shaders(Ptr)
        ),
        entry!(
            0x00b8_e810,
            image_space_shader_bright_pass_blur9_reload_shaders(Ptr)
        ),
        entry!(
            0x00b8_e9b0,
            image_space_shader_bright_pass_blur11_reload_shaders(Ptr)
        ),
        entry!(
            0x00b8_eb50,
            image_space_shader_bright_pass_blur13_reload_shaders(Ptr)
        ),
        entry!(
            0x00b8_ecf0,
            image_space_shader_bright_pass_blur15_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_0060,
            image_space_shader_hdr_blend_in_shader_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_0200,
            image_space_shader_hdr_blend_in_shader_cinematic_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_03a0,
            image_space_shader_hdr_blend_in_shader_cinematic_alphamask_reload_shaders(Ptr)
        ),
        entry!(0x00b9_0590, image_space_shader_hdr_blur_reload_shaders(Ptr)),
        entry!(
            0x00b9_0720,
            image_space_shader_hdr_bright_pass_filter_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_08a0,
            image_space_shader_hdr_down_sample16_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_0a30,
            image_space_shader_hdr_down_sample9_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_0bc0,
            image_space_shader_hdr_down_sample4_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_0d50,
            image_space_shader_hdr_light_adapt_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_0ee0,
            image_space_shader_hdr_lum_clamp_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_1070,
            image_space_shader_hdr_down_sample4_light_adapt_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_1210,
            image_space_shader_hdr_down_sample4_lum_clamp_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_13b0,
            image_space_shader_hdr_down_sample16_light_adapt_reload_shaders(Ptr)
        ),
        entry!(
            0x00b9_1550,
            image_space_shader_hdr_down_sample16_lum_clamp_reload_shaders(Ptr)
        ),
        entry!(0x00b8_bd50, fn_00b8bd50(Ptr, u32) -> Ptr),
        entry!(0x00b8_bfa0, fn_00b8bfa0(Ptr, u32) -> Ptr),
        entry!(0x00b8_d6c0, fn_00b8d6c0(Ptr, u32) -> Ptr),
        entry!(0x00b8_d810, fn_00b8d810(Ptr, u32) -> Ptr),
        entry!(0x00b8_d950, fn_00b8d950(Ptr, u32) -> Ptr),
        entry!(0x00b8_daa0, fn_00b8daa0(Ptr, u32) -> Ptr),
        entry!(0x00b8_dbf0, fn_00b8dbf0(Ptr, u32) -> Ptr),
        entry!(0x00b8_dd40, fn_00b8dd40(Ptr, u32) -> Ptr),
        entry!(0x00b8_de90, fn_00b8de90(Ptr, u32) -> Ptr),
        entry!(0x00b8_dfe0, fn_00b8dfe0(Ptr, u32) -> Ptr),
        entry!(0x00b8_e130, fn_00b8e130(Ptr, u32) -> Ptr),
        entry!(0x00b8_e280, fn_00b8e280(Ptr, u32) -> Ptr),
        entry!(0x00b8_e420, fn_00b8e420(Ptr, u32) -> Ptr),
        entry!(0x00b8_e5c0, fn_00b8e5c0(Ptr, u32) -> Ptr),
        entry!(0x00b8_e760, fn_00b8e760(Ptr, u32) -> Ptr),
        entry!(0x00b8_e900, fn_00b8e900(Ptr, u32) -> Ptr),
        entry!(0x00b8_eaa0, fn_00b8eaa0(Ptr, u32) -> Ptr),
        entry!(0x00b8_ec40, fn_00b8ec40(Ptr, u32) -> Ptr),
        entry!(0x00b8_ede0, fn_00b8ede0(Ptr, u32) -> Ptr),
        entry!(0x00b8_ef60, fn_00b8ef60(Ptr, u32) -> Ptr),
        entry!(0x00b8_f120, fn_00b8f120(Ptr, u32) -> Ptr),
        entry!(0x00b8_f2b0, fn_00b8f2b0(Ptr, u32) -> Ptr),
        entry!(0x00b8_f470, fn_00b8f470(Ptr, u32) -> Ptr),
        entry!(0x00b8_f620, fn_00b8f620(Ptr, u32) -> Ptr),
        entry!(0x00b8_f6d0, fn_00b8f6d0(Ptr, u32) -> Ptr),
        entry!(0x00b8_f7a0, fn_00b8f7a0(Ptr, u32) -> Ptr),
        entry!(0x00b8_f8f0, fn_00b8f8f0(Ptr, u32) -> Ptr),
        entry!(0x00b8_fa40, fn_00b8fa40(Ptr, u32) -> Ptr),
        entry!(0x00b8_fba0, fn_00b8fba0(Ptr, u32) -> Ptr),
        entry!(0x00b8_fd00, fn_00b8fd00(Ptr, u32) -> Ptr),
        entry!(0x00b8_fe60, fn_00b8fe60(Ptr, u32) -> Ptr),
        entry!(0x00b8_ffb0, fn_00b8ffb0(Ptr, u32) -> Ptr),
        entry!(0x00b9_0150, fn_00b90150(Ptr, u32) -> Ptr),
        entry!(0x00b9_02f0, fn_00b902f0(Ptr, u32) -> Ptr),
        entry!(0x00b9_04e0, fn_00b904e0(Ptr, u32) -> Ptr),
        entry!(0x00b9_0670, fn_00b90670(Ptr, u32) -> Ptr),
        entry!(0x00b9_07f0, fn_00b907f0(Ptr, u32) -> Ptr),
        entry!(0x00b9_0980, fn_00b90980(Ptr, u32) -> Ptr),
        entry!(0x00b9_0b10, fn_00b90b10(Ptr, u32) -> Ptr),
        entry!(0x00b9_0ca0, fn_00b90ca0(Ptr, u32) -> Ptr),
        entry!(0x00b9_0e30, fn_00b90e30(Ptr, u32) -> Ptr),
        entry!(0x00b9_0fc0, fn_00b90fc0(Ptr, u32) -> Ptr),
        entry!(0x00b9_1160, fn_00b91160(Ptr, u32) -> Ptr),
        entry!(0x00b9_1300, fn_00b91300(Ptr, u32) -> Ptr),
        entry!(0x00b9_14a0, fn_00b914a0(Ptr, u32) -> Ptr),
        entry!(0x00b9_1640, fn_00b91640(Ptr, u32) -> Ptr),
        entry!(0x00b9_1780, fn_00b91780(Ptr, u32) -> Ptr),
        entry!(0x00b9_18c0, fn_00b918c0(Ptr, u32) -> Ptr),
        entry!(0x00b9_1a00, fn_00b91a00(Ptr, u32) -> Ptr),
        entry!(0x00b9_1cb0, fn_00b91cb0(Ptr, u32) -> Ptr),
        entry!(0x00b9_1e00, fn_00b91e00(Ptr, u32) -> Ptr),
        entry!(0x00b9_1f50, fn_00b91f50(Ptr, u32) -> Ptr),
        entry!(0x00b9_2290, fn_00b92290(Ptr, u32) -> Ptr),
        entry!(0x00b9_2570, fn_00b92570(Ptr, u32) -> Ptr),
        entry!(0x00b9_27a0, fn_00b927a0(Ptr, u32) -> Ptr),
        entry!(0x00b9_28f0, fn_00b928f0(Ptr, u32) -> Ptr),
        entry!(0x00b9_2a40, fn_00b92a40(Ptr, u32) -> Ptr),
        entry!(0x00b9_2b90, fn_00b92b90(Ptr, u32) -> Ptr),
        entry!(0x00b9_2ce0, fn_00b92ce0(Ptr, u32) -> Ptr),
        entry!(0x00b9_2ea0, fn_00b92ea0(Ptr, u32) -> Ptr),
        entry!(0x00b9_2ff0, fn_00b92ff0(Ptr, u32) -> Ptr),
        entry!(0x00b9_3140, fn_00b93140(Ptr, u32) -> Ptr),
        entry!(0x00b9_3290, fn_00b93290(Ptr, u32) -> Ptr),
        entry!(0x00b9_33e0, fn_00b933e0(Ptr, u32) -> Ptr),
        entry!(0x00b9_3710, fn_00b93710(Ptr, u32) -> Ptr),
        entry!(0x00b9_3a60, fn_00b93a60(Ptr, u32) -> Ptr),
        entry!(0x00b9_3bb0, fn_00b93bb0(Ptr, u32) -> Ptr),
        entry!(0x00b9_3d00, fn_00b93d00(Ptr, u32) -> Ptr),
        entry!(0x00b9_3e50, fn_00b93e50(Ptr, u32) -> Ptr),
        entry!(0x00b9_3fa0, fn_00b93fa0(Ptr, u32) -> Ptr),
        entry!(0x00b9_40f0, fn_00b940f0(Ptr, u32) -> Ptr),
        entry!(0x00b9_4240, fn_00b94240(Ptr, u32) -> Ptr),
        entry!(0x00b9_4390, fn_00b94390(Ptr, u32) -> Ptr),
        entry!(0x00b9_44e0, fn_00b944e0(Ptr, u32) -> Ptr),
        entry!(0x00b8_bd80, fn_00b8bd80(Ptr, u32) -> Ptr),
        entry!(0x00b8_bfd0, fn_00b8bfd0(Ptr, u32) -> Ptr),
        entry!(0x00b8_d6f0, fn_00b8d6f0(Ptr, u32) -> Ptr),
        entry!(0x00b8_d840, fn_00b8d840(Ptr, u32) -> Ptr),
        entry!(0x00b8_d980, fn_00b8d980(Ptr, u32) -> Ptr),
        entry!(0x00b8_dad0, fn_00b8dad0(Ptr, u32) -> Ptr),
        entry!(0x00b8_dc20, fn_00b8dc20(Ptr, u32) -> Ptr),
        entry!(0x00b8_dd70, fn_00b8dd70(Ptr, u32) -> Ptr),
        entry!(0x00b8_dec0, fn_00b8dec0(Ptr, u32) -> Ptr),
        entry!(0x00b8_e010, fn_00b8e010(Ptr, u32) -> Ptr),
        entry!(0x00b8_e160, fn_00b8e160(Ptr, u32) -> Ptr),
        entry!(0x00b8_e2b0, fn_00b8e2b0(Ptr, u32) -> Ptr),
        entry!(0x00b8_e450, fn_00b8e450(Ptr, u32) -> Ptr),
        entry!(0x00b8_e5f0, fn_00b8e5f0(Ptr, u32) -> Ptr),
        entry!(0x00b8_e790, fn_00b8e790(Ptr, u32) -> Ptr),
        entry!(0x00b8_e930, fn_00b8e930(Ptr, u32) -> Ptr),
        entry!(0x00b8_ead0, fn_00b8ead0(Ptr, u32) -> Ptr),
        entry!(0x00b8_ec70, fn_00b8ec70(Ptr, u32) -> Ptr),
        entry!(0x00b8_ee10, fn_00b8ee10(Ptr, u32) -> Ptr),
        entry!(0x00b8_ef90, fn_00b8ef90(Ptr, u32) -> Ptr),
        entry!(0x00b8_f150, fn_00b8f150(Ptr, u32) -> Ptr),
        entry!(0x00b8_f2e0, fn_00b8f2e0(Ptr, u32) -> Ptr),
        entry!(0x00b8_f4a0, fn_00b8f4a0(Ptr, u32) -> Ptr),
        entry!(0x00b8_f650, fn_00b8f650(Ptr, u32) -> Ptr),
        entry!(0x00b8_f700, fn_00b8f700(Ptr, u32) -> Ptr),
        entry!(0x00b8_f7d0, fn_00b8f7d0(Ptr, u32) -> Ptr),
        entry!(0x00b8_f920, fn_00b8f920(Ptr, u32) -> Ptr),
        entry!(0x00b8_fa70, fn_00b8fa70(Ptr, u32) -> Ptr),
        entry!(0x00b8_fbd0, fn_00b8fbd0(Ptr, u32) -> Ptr),
        entry!(0x00b8_fd30, fn_00b8fd30(Ptr, u32) -> Ptr),
        entry!(0x00b8_fe90, fn_00b8fe90(Ptr, u32) -> Ptr),
        entry!(0x00b8_ffe0, fn_00b8ffe0(Ptr, u32) -> Ptr),
        entry!(0x00b9_0180, fn_00b90180(Ptr, u32) -> Ptr),
        entry!(0x00b9_0320, fn_00b90320(Ptr, u32) -> Ptr),
        entry!(0x00b9_0510, fn_00b90510(Ptr, u32) -> Ptr),
        entry!(0x00b9_06a0, fn_00b906a0(Ptr, u32) -> Ptr),
        entry!(0x00b9_0820, fn_00b90820(Ptr, u32) -> Ptr),
        entry!(0x00b9_09b0, fn_00b909b0(Ptr, u32) -> Ptr),
        entry!(0x00b9_0b40, fn_00b90b40(Ptr, u32) -> Ptr),
        entry!(0x00b9_0cd0, fn_00b90cd0(Ptr, u32) -> Ptr),
        entry!(0x00b9_0e60, fn_00b90e60(Ptr, u32) -> Ptr),
        entry!(0x00b9_0ff0, fn_00b90ff0(Ptr, u32) -> Ptr),
        entry!(0x00b9_1190, fn_00b91190(Ptr, u32) -> Ptr),
        entry!(0x00b9_1330, fn_00b91330(Ptr, u32) -> Ptr),
        entry!(0x00b9_14d0, fn_00b914d0(Ptr, u32) -> Ptr),
        entry!(0x00b9_1670, fn_00b91670(Ptr, u32) -> Ptr),
        entry!(0x00b9_17b0, fn_00b917b0(Ptr, u32) -> Ptr),
        entry!(0x00b9_18f0, fn_00b918f0(Ptr, u32) -> Ptr),
        entry!(0x00b9_1a30, fn_00b91a30(Ptr, u32) -> Ptr),
        entry!(0x00b9_1ce0, fn_00b91ce0(Ptr, u32) -> Ptr),
        entry!(0x00b9_1e30, fn_00b91e30(Ptr, u32) -> Ptr),
        entry!(0x00b9_1f80, fn_00b91f80(Ptr, u32) -> Ptr),
        entry!(0x00b9_22c0, fn_00b922c0(Ptr, u32) -> Ptr),
        entry!(0x00b9_25a0, fn_00b925a0(Ptr, u32) -> Ptr),
        entry!(0x00b9_27d0, fn_00b927d0(Ptr, u32) -> Ptr),
        entry!(0x00b9_2920, fn_00b92920(Ptr, u32) -> Ptr),
        entry!(0x00b9_2a70, fn_00b92a70(Ptr, u32) -> Ptr),
        entry!(0x00b9_2bc0, fn_00b92bc0(Ptr, u32) -> Ptr),
        entry!(0x00b9_2d10, fn_00b92d10(Ptr, u32) -> Ptr),
        entry!(0x00b9_2ed0, fn_00b92ed0(Ptr, u32) -> Ptr),
        entry!(0x00b9_3020, fn_00b93020(Ptr, u32) -> Ptr),
        entry!(0x00b9_3170, fn_00b93170(Ptr, u32) -> Ptr),
        entry!(0x00b9_32c0, fn_00b932c0(Ptr, u32) -> Ptr),
        entry!(0x00b9_3410, fn_00b93410(Ptr, u32) -> Ptr),
        entry!(0x00b9_3740, fn_00b93740(Ptr, u32) -> Ptr),
        entry!(0x00b9_3a90, fn_00b93a90(Ptr, u32) -> Ptr),
        entry!(0x00b9_3be0, fn_00b93be0(Ptr, u32) -> Ptr),
        entry!(0x00b9_3d30, fn_00b93d30(Ptr, u32) -> Ptr),
        entry!(0x00b9_3e80, fn_00b93e80(Ptr, u32) -> Ptr),
        entry!(0x00b9_3fd0, fn_00b93fd0(Ptr, u32) -> Ptr),
        entry!(0x00b9_4120, fn_00b94120(Ptr, u32) -> Ptr),
        entry!(0x00b9_4270, fn_00b94270(Ptr, u32) -> Ptr),
        entry!(0x00b9_43c0, fn_00b943c0(Ptr, u32) -> Ptr),
        entry!(0x00b9_4510, fn_00b94510(Ptr, u32) -> Ptr),
        entry!(0x00b8_ad00, image_space_parameter_data_endian(Ptr)),
        entry!(
            0x00b8_b060,
            fn_00b8b060(Ptr, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00b8_b1c0, fn_00b8b1c0(u32, u32, u32)),
        entry!(0x00b8_b1e0, fn_00b8b1e0(u32, u32)),
        entry!(0x00b8_b3b0, fn_00b8b3b0(Ptr)),
        entry!(0x00b8_b760, fn_00b8b760(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x00b8_b7d0, fn_00b8b7d0(Ptr, u32, u32, u32, u32, u32)),
        entry!(0x00b8_b8d0, fn_00b8b8d0(Ptr, u32, u32, u32, u32)),
        entry!(0x00b8_bb70, fn_00b8bb70(u32, u32, u32)),
        entry!(0x00b8_bc80, fn_00b8bc80(Ptr)),
        entry!(0x00b8_bd90, fn_00b8bd90() -> u32),
        entry!(
            0x00b8_c3b0,
            image_space_manager_create_partial_screen_geometry(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00b8_c730,
            image_space_manager_render_effect(Ptr, u32, u32, u32, u32, u32)
        ),
        entry!(0x00b8_c980, fn_00b8c980(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x00b8_cb00, fn_00b8cb00(Ptr, u32)),
        entry!(0x00b8_d440, fn_00b8d440(Ptr) -> u32),
        entry!(0x00b9_75f0, fn_00b975f0(Ptr, u32, u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x6000);
        e.map(0x0101_7000, 0x1000);
        e.set_global(OBJECT_011F91A8, 0x0f00_1000u32);
        e.set_global(OBJECT_011F4748, 0x0f00_2000u32);
        // A signalling NaN: the FLD/FSTP copy quiets it.
        e.set_global(VALUE_01017870, 0x7f80_0001u32);
        for addr in [
            SET_UP_00B94520,
            INIT_GEOMETRY,
            SETUP_EFFECTS,
            INIT_CONSTANT_MAP,
            SET_UP_00B8AD70,
        ] {
            e.register(addr, |e, _| {
                // Each step runs with the image-space context.
                let tls = e.tls();
                assert_eq!(e.mem.u32(tls + TLS_CONTEXT), CONTEXT_IMAGE_SPACE);
                Ret::default()
            });
        }
        e.register(MAKE_00B6E110, |_, _| 0x0f00_3000u32.into_ret());
        e
    }

    #[test]
    fn image_space_manager_initialize_sets_up_once() {
        let mut e = rig();
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        let this = e.mem.alloc(0x320);
        for offset in (0..0x320).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.mem.set_u8(this + INITIALIZED, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b9_77d0, &args![this]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .skip(1)
            .collect();
        assert_eq!(
            order,
            [
                SET_UP_00B94520,
                INIT_GEOMETRY,
                SETUP_EFFECTS,
                INIT_CONSTANT_MAP,
                SET_UP_00B8AD70,
                MAKE_00B6E110
            ]
        );
        assert_eq!(calls(&e, SET_UP_00B94520), vec![vec![this]]);
        assert_eq!(
            calls(&e, INIT_CONSTANT_MAP),
            vec![vec![CONSTANT_MAP_01202410, 0, 0]]
        );
        assert_eq!(
            calls(&e, MAKE_00B6E110),
            vec![vec![0x0f00_1000, 0x0f00_2000, 3, 0, 0, 0]]
        );
        assert_eq!(e.mem.u8(this + INITIALIZED), 1);
        for offset in [0x2f8u32, 0x2fc, 0x300, 0x308, 0x314] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.f32(this + 0x30c), 1.0);
        assert_eq!(e.mem.f32(this + 0x310), 1.0);
        assert_eq!(e.mem.u32(this + 0x304), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 0x318), 0x0f00_3000);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);

        // A second run does nothing but set and restore the context.
        e.mem.set_u32(this + 0x318, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x00b9_77d0, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this + 0x318), 0x55);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
    }
}

#[cfg(test)]
mod setup_effects_tests {
    use super::*;

    #[test]
    fn image_space_manager_setup_effects_calls_each_set_effect() {
        const SETUP: u32 = 0x0e40_0000;
        let mut e = Engine::new();
        e.register(SETUP, |_, _| Ret::default());
        e.put_vtable(0x0e41_0000, &[0, 0, SETUP]);
        let this = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x88);
        e.mem.set_u32(this + 8, effects);
        let mut set = vec![];
        for index in [0u32, 5, 0x21] {
            let effect = e.mem.alloc(4);
            e.mem.set_u32(effect, 0x0e41_0000);
            e.mem.set_u32(effects + 4 * index, effect);
            set.push(effect);
        }
        e.call_log = Some(vec![]);
        e.call(0x00b8_b360, &args![this]);
        let log = e.call_log.take().unwrap();
        let seen: Vec<_> = log
            .iter()
            .filter(|c| c.0 == SETUP)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            seen,
            set.iter().map(|&s| vec![s, this, 0]).collect::<Vec<_>>()
        );
    }
}

#[cfg(test)]
mod defaults_tests {
    use super::*;

    #[test]
    fn fn_00b8ad70_stores_every_default() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x18000);
        e.map(0x011f_f000, 0x1000);
        e.mem.write(0x011f_f940, &[0xee; 0x94]);
        // A signalling NaN is quieted by the load.
        e.set_global::<u32>(HALF_01016248, 0x7f80_0005);
        let sources = [
            0x0101_8f98u32,
            0x0102_2154,
            0x0102_212c,
            0x0101_6088,
            0x0102_1c9c,
            0x0101_df44,
            0x0101_8238,
            0x0102_d934,
            0x0101_84f4,
            0x0102_d930,
            0x0102_d92c,
        ];
        for (index, source) in sources.iter().enumerate() {
            e.set_global::<f32>(*source, 10.0 + index as f32);
        }
        e.call(0x00b8_ad70, &args![]);
        let half = 0x7fc0_0005;
        let one = 0x3f80_0000;
        let value = |index: u32| (10.0 + index as f32).to_bits();
        let expected = [
            (0x940u32, half),
            (0x944, value(0)),
            (0x948, one),
            (0x94c, one),
            (0x950, one),
            (0x954, one),
            (0x958, value(1)),
            (0x95c, value(2)),
            (0x960, one),
            (0x964, 0),
            (0x968, 0),
            (0x96c, value(3)),
            (0x970, value(3)),
            (0x974, one),
            (0x97c, value(4)),
            (0x980, value(6)),
            (0x984, value(5)),
            (0x988, half),
            (0x98c, half),
            (0x990, value(7)),
            (0x994, value(9)),
            (0x998, half),
            (0x99c, value(10)),
            (0x9a0, value(8)),
            (0x9a4, one),
            (0x9a8, half),
            (0x9ac, one),
            (0x9b0, one),
            (0x9b4, one),
            (0x9b8, one),
            (0x9bc, one),
            (0x9c0, 0),
            (0x9c4, one),
            (0x9c8, one),
            (0x9cc, one),
            (0x9d0, 0),
        ];
        for (offset, bits) in expected {
            assert_eq!(e.global::<u32>(0x011f_f000 + offset), bits, "{offset:#x}");
        }
        // 011ff978 is not written.
        assert_eq!(e.global::<u32>(0x011f_f978), 0xeeee_eeee);
    }
}

#[cfg(test)]
mod init_geometry_tests {
    use super::*;

    const DELETE: u32 = 0x0ef0_0000;
    const VTABLE: u32 = 0x0ef1_0000;
    const NAME: u32 = 0x0ef2_0008;
    const DATA_OF_ELEMENTS: u32 = 0x0ef3_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine(allocate: bool) -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.register_double(NI_ALLOCATE, move |e, a| {
            if allocate {
                let block = e.mem.alloc(a[0]);
                e.mem.set_u32(block, VTABLE);
                block.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(SCREEN_ELEMENTS_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SCREEN_ELEMENTS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xb8, DATA_OF_ELEMENTS);
            a[0].into_ret()
        });
        for addr in [
            SCREEN_ELEMENTS_DATA_INSERT,
            SCREEN_ELEMENTS_DATA_SET_RECTANGLE,
            SCREEN_ELEMENTS_DATA_SET_TEXTURES,
            SCREEN_ELEMENTS_DATA_UPDATE_BOUND,
            SET_NAME_00A5B950,
            UPDATE_PROPERTIES,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(UPDATE_00A59C60, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            assert_eq!(e.mem.bytes(a[1] + 4, 5), vec![0; 5]);
            Ret::default()
        });
        e.register(MAKE_FIXED_STRING_00A5B690, |_, _| NAME.into_ret());
        e.map(0x0101_2000, 0x5000);
        e.map(0x0104_a000, 0x1000);
        e.map(0x0109_b000, 0x1000);
        e.map(NAME - 8, 0x10);
        e.set_global::<f32>(RECTANGLE_X_01012054, -1.0);
        e.set_global::<f32>(RECTANGLE_WIDTH_010162C0, 2.0);
        e.set_global::<f32>(RECTANGLE_HEIGHT_0104A494, -2.0);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn image_space_manager_init_geometry_builds_the_screen_elements() {
        let mut e = engine(true);
        let this = e.mem.alloc(0x200);
        e.call(INIT_GEOMETRY, &args![this]);
        let data = e.mem.u32(this + 0x1f4);
        let elements = e.mem.u32(this + 0x1f0);
        assert_ne!(data, 0);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.mem.u32(elements + 4), 1);
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_CONSTRUCT),
            vec![vec![data, 0, 0, 1, 1, 1, 4, 1, 2, 1]]
        );
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_CONSTRUCT),
            vec![vec![elements, data]]
        );
        let d = DATA_OF_ELEMENTS;
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_INSERT),
            vec![vec![d, 4, 0, 0]]
        );
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_SET_RECTANGLE),
            vec![vec![
                d,
                0,
                (-1.0f32).to_bits(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                (-2.0f32).to_bits()
            ]]
        );
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_SET_TEXTURES),
            vec![vec![d, 0, 0, 0, 0, 1.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, SCREEN_ELEMENTS_DATA_UPDATE_BOUND), vec![vec![d]]);
        assert_eq!(
            calls(&e, MAKE_FIXED_STRING_00A5B690),
            vec![vec![GEOMETRY_NAME_010B04AC]]
        );
        let named = calls(&e, SET_NAME_00A5B950);
        assert_eq!(named.len(), 1);
        assert_eq!(named[0][0], elements);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![NAME - 8]]
        );
        assert_eq!(calls(&e, UPDATE_PROPERTIES), vec![vec![elements]]);
        assert_eq!(calls(&e, UPDATE_00A59C60).len(), 1);
    }

    #[test]
    fn image_space_manager_init_geometry_failed_allocations_leave_null_pointers() {
        let mut e = engine(false);
        let this = e.mem.alloc(0x200);
        // The elements' data is then read through a null pointer, as the exe does.
        e.map(0, 0x1000);
        e.call(INIT_GEOMETRY, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1f4), 0);
        assert_eq!(e.mem.u32(this + 0x1f0), 0);
        assert!(calls(&e, SCREEN_ELEMENTS_DATA_CONSTRUCT).is_empty());
        assert!(calls(&e, SCREEN_ELEMENTS_CONSTRUCT).is_empty());
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_INSERT),
            vec![vec![0, 4, 0, 0]]
        );
    }
}

#[cfg(test)]
mod set_up_00b94520_tests {
    use super::*;

    const VTABLE: u32 = 0x0e50_0000;
    const SLOT_20: u32 = 0x0e51_0020;
    const SLOT_24: u32 = 0x0e51_0024;
    const SLOT_120: u32 = 0x0e51_0120;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn rig(ni_objects: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x49];
        slots[0x20 / 4] = SLOT_20;
        slots[0x24 / 4] = SLOT_24;
        slots[0x120 / 4] = SLOT_120;
        e.put_vtable(VTABLE, &slots);
        for addr in [SLOT_20, SLOT_24, SLOT_120] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        if ni_objects {
            e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        } else {
            e.register(NI_ALLOCATE, |_, _| Ret::default());
        }
        let constructors = SET_UP_MANAGED
            .iter()
            .map(|c| c.1)
            .chain(SET_UP_NI.iter().map(|c| c.1))
            .chain(SET_UP_NI_ARRAY.iter().map(|c| c.0))
            .chain([CONSTRUCT_00BA4640]);
        for addr in constructors {
            e.register(addr, |e, a| {
                e.mem.set_u32(a[0], VTABLE);
                a[0].into_ret()
            });
        }
        // The array setter stores the pointed-at word under the index.
        e.register(ARRAY_SET_00BA3AA0, |e, a| {
            let block = e.mem.u32(a[0] + 4);
            let word = e.mem.u32(a[2]);
            e.mem.set_u32(block + 4 * a[1], word);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let block = e.mem.alloc(4 * 0x80);
        e.mem.set_u32(this + EFFECTS, block);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_00b94520_fills_the_effect_array() {
        let (mut e, this) = rig(true);
        e.call(0x00b9_4520, &args![this]);
        let managed = calls(&e, MEMORY_ALLOCATE);
        assert_eq!(managed.len(), 20 + 14);
        assert_eq!(managed[0], vec![MEMORY_MANAGER, 0x68]);
        assert_eq!(managed[20], vec![MEMORY_MANAGER, 0x60]);
        let ni = calls(&e, NI_ALLOCATE);
        assert_eq!(ni.len(), 80);
        assert_eq!(ni.iter().filter(|a| a[0] == 0xd4).count(), 1);
        let stored: Vec<Vec<u32>> = calls(&e, ARRAY_SET_00BA3AA0)
            .iter()
            .map(|a| a[..2].to_vec())
            .collect();
        assert_eq!(stored.len(), 22);
        assert_eq!(stored[0], vec![this + 4, 0xd]);
        assert_eq!(stored[10], vec![this + 4, 7]);
        assert_eq!(stored[20], vec![this + 4, 0x73]);
        assert_eq!(stored[21], vec![this + 4, 0x4d]);
        let block = e.mem.u32(this + EFFECTS);
        // The pairs: vtable +0x20 with k, +0x24 with 0 then 1.
        let ranks: Vec<u32> = calls(&e, SLOT_20).iter().map(|a| a[1]).collect();
        assert_eq!(ranks, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7]);
        let flags: Vec<u32> = calls(&e, SLOT_24).iter().map(|a| a[1]).collect();
        assert_eq!(flags, [0, 1].repeat(7));
        assert_eq!(calls(&e, SLOT_20)[0][0], e.mem.u32(block + 4 * 0x10));
        assert_eq!(calls(&e, SLOT_20)[13][0], e.mem.u32(block + 4 * 0x1d));
        // The inline stores: elements are the objects +0x6c; size 0x73, 14 + 78 counted.
        let first = ni[0].clone();
        assert_eq!(first, vec![0xd0]);
        assert_eq!(e.mem.u32(e.mem.u32(block + 4 * 0x22) - 0x6c), VTABLE);
        assert_eq!(e.mem.u16(this + EFFECTS_SIZE), 0x73);
        assert_eq!(e.mem.u16(this + EFFECTS_COUNT), 14 + 78);
        // The closing pass over indices 0x22 to 0x73 calls vtable +0x120 of each object.
        let set = (0x22..=0x73u32)
            .filter(|&i| e.mem.u32(block + 4 * i) != 0)
            .count();
        let finals = calls(&e, SLOT_120);
        assert_eq!(finals.len(), set);
        assert_eq!(finals.len(), 80);
        assert_eq!(finals[0], vec![e.mem.u32(block + 4 * 0x22) - 0x6c]);
    }

    #[test]
    fn fn_00b94520_null_elements_empty_full_slots() {
        let (mut e, this) = rig(false);
        let block = e.mem.u32(this + EFFECTS);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, VTABLE);
        for index in 0..0x80u32 {
            e.mem.set_u32(block + 4 * index, object + 0x6c);
        }
        e.mem.set_u16(this + EFFECTS_SIZE, 0x80);
        e.mem.set_u16(this + EFFECTS_COUNT, 0x80);
        e.call(0x00b9_4520, &args![this]);
        // Each null element over a full slot takes one away; the pairs land in full slots.
        assert_eq!(e.mem.u16(this + EFFECTS_SIZE), 0x80);
        assert_eq!(e.mem.u16(this + EFFECTS_COUNT), 0x80 - 78);
        assert!(calls(&e, ARRAY_SET_00BA3AA0).iter().all(|a| a[1] < 0x80));
        assert_eq!(e.mem.u32(block + 4 * 0x73), 0);
        assert_eq!(e.mem.u32(block + 4 * 0x4d), 0);
        // Only the slots nothing stored to are still set at the closing pass.
        let left = (0x22..=0x73u32)
            .filter(|&i| e.mem.u32(block + 4 * i) != 0)
            .count();
        assert_eq!(calls(&e, SLOT_120).len(), left);
        assert!(calls(&e, SLOT_120).iter().all(|a| a == &vec![object]));
    }
}

#[cfg(test)]
mod shader_constructor_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The base constructor writes marker vtables; the resize checks both were replaced
    /// before it runs.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMAGE_SPACE_SHADER_CONSTRUCT_00C04570, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            e.mem.set_u32(a[0] + SHADER_EFFECT_BASE, 0x2222);
            a[0].into_ret()
        });
        e.register(ARRAY_SET_SIZE_00A90180, |e, a| {
            let object = a[0] - SHADER_TEXTURES;
            assert_ne!(e.mem.u32(object), 0x1111);
            assert_ne!(e.mem.u32(object + SHADER_EFFECT_BASE), 0x2222);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn check(addr: u32, vtable: u32, effect_vtable: u32, textures: u32) {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        assert_eq!(e.call(addr, &args![this]).u32(), this);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                addr,
                IMAGE_SPACE_SHADER_CONSTRUCT_00C04570,
                ARRAY_SET_SIZE_00A90180
            ]
        );
        assert_eq!(
            calls(&e, IMAGE_SPACE_SHADER_CONSTRUCT_00C04570),
            vec![vec![this]]
        );
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + SHADER_TEXTURES, textures]]
        );
        assert_eq!(e.mem.u32(this), vtable);
        assert_eq!(e.mem.u32(this + SHADER_EFFECT_BASE), effect_vtable);
    }

    /// The engine, `this` and the object at `011f9520` (null without one; else with two
    /// references).
    fn texture_mask_rig(with_object: bool) -> (Engine, u32, u32) {
        let mut e = engine();
        let object = if with_object { e.mem.alloc(8) } else { 0 };
        if with_object {
            e.mem.set_u32(object + 4, 2);
        }
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.map(0x011f_9000, 0x1000);
        e.set_global(OBJECT_011F9520, object);
        let this = e.mem.alloc(0xd4);
        e.mem.set_u32(this + 0xd0, 0xdead_beef);
        (e, this, object)
    }

    #[test]
    fn fn_00b922d0_takes_a_reference_on_the_object() {
        let (mut e, this, object) = texture_mask_rig(true);
        assert_eq!(e.call(0x00b9_22d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010b_5978);
        assert_eq!(e.mem.u32(this + SHADER_EFFECT_BASE), 0x010b_5954);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + SHADER_TEXTURES, 2]]
        );
        assert_eq!(e.mem.u32(this + 0xd0), object);
        assert_eq!(e.mem.u32(object + 4), 3);
        // The cleared pointer had nothing to give up.
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn fn_00b922d0_with_no_object_keeps_the_pointer_clear() {
        let (mut e, this, _) = texture_mask_rig(false);
        e.call(0x00b9_22d0, &args![this]);
        assert_eq!(e.mem.u32(this + 0xd0), 0);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn fn_00b8bce0_constructs_the_shader() {
        check(0x00b8_bce0, 0x010a_ffc0, 0x010a_ffa0, 5);
    }

    #[test]
    fn fn_00b8bea0_constructs_the_shader() {
        check(0x00b8_bea0, 0x010b_0198, 0x010b_0174, 2);
    }

    #[test]
    fn fn_00b8c070_constructs_the_shader() {
        check(0x00b8_c070, 0x010b_0338, 0x010b_0318, 4);
    }

    #[test]
    fn fn_00b8d330_constructs_the_shader() {
        check(0x00b8_d330, 0x010b_0518, 0x010b_04f4, 3);
    }

    #[test]
    fn fn_00b8d5b0_constructs_the_shader() {
        check(0x00b8_d5b0, 0x010b_06c0, 0x010b_06a0, 2);
    }

    #[test]
    fn fn_00b8d700_constructs_the_shader() {
        check(0x00b8_d700, 0x010b_0860, 0x010b_083c, 3);
    }

    #[test]
    fn fn_00b8d850_constructs_the_shader() {
        check(0x00b8_d850, 0x010b_0a00, 0x010b_09dc, 2);
    }

    #[test]
    fn fn_00b8d990_constructs_the_shader() {
        check(0x00b8_d990, 0x010b_0b98, 0x010b_0b78, 2);
    }

    #[test]
    fn fn_00b8dae0_constructs_the_shader() {
        check(0x00b8_dae0, 0x010b_0d40, 0x010b_0d1c, 2);
    }

    #[test]
    fn fn_00b8dc30_constructs_the_shader() {
        check(0x00b8_dc30, 0x010b_0ec0, 0x010b_0e9c, 2);
    }

    #[test]
    fn fn_00b8dd80_constructs_the_shader() {
        check(0x00b8_dd80, 0x010b_1040, 0x010b_101c, 2);
    }

    #[test]
    fn fn_00b8ded0_constructs_the_shader() {
        check(0x00b8_ded0, 0x010b_11c0, 0x010b_119c, 2);
    }

    #[test]
    fn fn_00b8e020_constructs_the_shader() {
        check(0x00b8_e020, 0x010b_1340, 0x010b_1320, 2);
    }

    #[test]
    fn fn_00b8e170_constructs_the_shader() {
        check(0x00b8_e170, 0x010b_14c0, 0x010b_14a0, 2);
    }

    #[test]
    fn fn_00b8e2c0_constructs_the_shader() {
        check(0x00b8_e2c0, 0x010b_1640, 0x010b_1620, 3);
    }

    #[test]
    fn fn_00b8e460_constructs_the_shader() {
        check(0x00b8_e460, 0x010b_17f8, 0x010b_17d8, 3);
    }

    #[test]
    fn fn_00b8e600_constructs_the_shader() {
        check(0x00b8_e600, 0x010b_1980, 0x010b_1960, 3);
    }

    #[test]
    fn fn_00b8e7a0_constructs_the_shader() {
        check(0x00b8_e7a0, 0x010b_1b08, 0x010b_1ae8, 3);
    }

    #[test]
    fn fn_00b8e940_constructs_the_shader() {
        check(0x00b8_e940, 0x010b_1c90, 0x010b_1c70, 3);
    }

    #[test]
    fn fn_00b8eae0_constructs_the_shader() {
        check(0x00b8_eae0, 0x010b_1e20, 0x010b_1dfc, 3);
    }

    #[test]
    fn fn_00b8ec80_constructs_the_shader() {
        check(0x00b8_ec80, 0x010b_1fb0, 0x010b_1f8c, 3);
    }

    #[test]
    fn fn_00b8ee20_constructs_the_shader() {
        check(0x00b8_ee20, 0x010b_2140, 0x010b_211c, 2);
    }

    #[test]
    fn fn_00b8efa0_constructs_the_shader() {
        check(0x00b8_efa0, 0x010b_2a98, 0x010b_2a74, 2);
    }

    #[test]
    fn fn_00b8f010_constructs_the_shader() {
        check(0x00b8_f010, 0x010b_2308, 0x010b_22e8, 2);
    }

    #[test]
    fn fn_00b8f1b0_constructs_the_shader() {
        check(0x00b8_f1b0, 0x010b_2490, 0x010b_2470, 2);
    }

    #[test]
    fn fn_00b8f360_constructs_the_shader() {
        check(0x00b8_f360, 0x010b_2610, 0x010b_25f0, 1);
    }

    #[test]
    fn fn_00b8f520_constructs_the_shader() {
        check(0x00b8_f520, 0x010b_27a8, 0x010b_2784, 1);
    }

    #[test]
    fn fn_00b8f660_constructs_the_shader() {
        check(0x00b8_f660, 0x010b_2920, 0x010b_28fc, 3);
    }

    #[test]
    fn fn_00b8f7e0_constructs_the_shader() {
        check(0x00b8_f7e0, 0x010b_2c10, 0x010b_2bec, 4);
    }

    #[test]
    fn fn_00b8f930_constructs_the_shader() {
        check(0x00b8_f930, 0x010b_2db0, 0x010b_2d8c, 2);
    }

    #[test]
    fn fn_00b8fa80_constructs_the_shader() {
        check(0x00b8_fa80, 0x010b_2f50, 0x010b_2f30, 2);
    }

    #[test]
    fn fn_00b8fbe0_constructs_the_shader() {
        check(0x00b8_fbe0, 0x010b_30e0, 0x010b_30c0, 2);
    }

    #[test]
    fn fn_00b8fd40_constructs_the_shader() {
        check(0x00b8_fd40, 0x010b_3268, 0x010b_3244, 5);
    }

    #[test]
    fn fn_00b8fea0_constructs_the_shader() {
        check(0x00b8_fea0, 0x010b_33f0, 0x010b_33cc, 3);
    }

    #[test]
    fn fn_00b8fff0_constructs_the_shader() {
        check(0x00b8_fff0, 0x010b_3598, 0x010b_3574, 4);
    }

    #[test]
    fn fn_00b90190_constructs_the_shader() {
        check(0x00b9_0190, 0x010b_37a8, 0x010b_3788, 4);
    }

    #[test]
    fn fn_00b90330_constructs_the_shader() {
        check(0x00b9_0330, 0x010b_3958, 0x010b_3934, 4);
    }

    #[test]
    fn fn_00b90520_constructs_the_shader() {
        check(0x00b9_0520, 0x010b_3af0, 0x010b_3ad0, 2);
    }

    #[test]
    fn fn_00b906b0_constructs_the_shader() {
        check(0x00b9_06b0, 0x010b_3c78, 0x010b_3c58, 2);
    }

    #[test]
    fn fn_00b90830_constructs_the_shader() {
        check(0x00b9_0830, 0x010b_3e18, 0x010b_3df4, 2);
    }

    #[test]
    fn fn_00b909c0_constructs_the_shader() {
        check(0x00b9_09c0, 0x010b_3fa8, 0x010b_3f88, 2);
    }

    #[test]
    fn fn_00b90b50_constructs_the_shader() {
        check(0x00b9_0b50, 0x010b_4138, 0x010b_4114, 2);
    }

    #[test]
    fn fn_00b90ce0_constructs_the_shader() {
        check(0x00b9_0ce0, 0x010b_42c8, 0x010b_42a4, 3);
    }

    #[test]
    fn fn_00b90e70_constructs_the_shader() {
        check(0x00b9_0e70, 0x010b_4460, 0x010b_443c, 2);
    }

    #[test]
    fn fn_00b91000_constructs_the_shader() {
        check(0x00b9_1000, 0x010b_4600, 0x010b_45dc, 3);
    }

    #[test]
    fn fn_00b911a0_constructs_the_shader() {
        check(0x00b9_11a0, 0x010b_47a0, 0x010b_477c, 2);
    }

    #[test]
    fn fn_00b91340_constructs_the_shader() {
        check(0x00b9_1340, 0x010b_4938, 0x010b_4914, 3);
    }

    #[test]
    fn fn_00b914e0_constructs_the_shader() {
        check(0x00b9_14e0, 0x010b_4ad0, 0x010b_4aac, 2);
    }

    #[test]
    fn fn_00b91680_constructs_the_shader() {
        check(0x00b9_1680, 0x010b_4c68, 0x010b_4c48, 5);
    }

    #[test]
    fn fn_00b917c0_constructs_the_shader() {
        check(0x00b9_17c0, 0x010b_4de8, 0x010b_4dc8, 2);
    }

    #[test]
    fn fn_00b91900_constructs_the_shader() {
        check(0x00b9_1900, 0x010b_4f80, 0x010b_4f60, 3);
    }

    #[test]
    fn fn_00b91a40_constructs_the_shader() {
        check(0x00b9_1a40, 0x010b_5140, 0x010b_511c, 2);
    }

    #[test]
    fn fn_00b91b80_constructs_the_shader() {
        check(0x00b9_1b80, 0x010b_52c0, 0x010b_52a0, 3);
    }

    #[test]
    fn fn_00b91cf0_constructs_the_shader() {
        check(0x00b9_1cf0, 0x010b_5480, 0x010b_5460, 2);
    }

    #[test]
    fn fn_00b91e40_constructs_the_shader() {
        check(0x00b9_1e40, 0x010b_5640, 0x010b_5620, 2);
    }

    #[test]
    fn fn_00b92180_constructs_the_shader() {
        check(0x00b9_2180, 0x010b_57d0, 0x010b_57b0, 3);
    }

    #[test]
    fn fn_00b92500_constructs_the_shader() {
        check(0x00b9_2500, 0x010b_5b10, 0x010b_5af0, 5);
    }

    #[test]
    fn fn_00b92690_constructs_the_shader() {
        check(0x00b9_2690, 0x010b_5cb8, 0x010b_5c94, 2);
    }

    #[test]
    fn fn_00b927e0_constructs_the_shader() {
        check(0x00b9_27e0, 0x010b_5ea8, 0x010b_5e84, 2);
    }

    #[test]
    fn fn_00b92930_constructs_the_shader() {
        check(0x00b9_2930, 0x010b_6048, 0x010b_6028, 2);
    }

    #[test]
    fn fn_00b92a80_constructs_the_shader() {
        check(0x00b9_2a80, 0x010b_61f0, 0x010b_61d0, 2);
    }

    #[test]
    fn fn_00b92bd0_constructs_the_shader() {
        check(0x00b9_2bd0, 0x010b_6398, 0x010b_6374, 2);
    }

    #[test]
    fn fn_00b92d90_constructs_the_shader() {
        check(0x00b9_2d90, 0x010b_6548, 0x010b_6524, 2);
    }

    #[test]
    fn fn_00b92ee0_constructs_the_shader() {
        check(0x00b9_2ee0, 0x010b_66f0, 0x010b_66cc, 3);
    }

    #[test]
    fn fn_00b93030_constructs_the_shader() {
        check(0x00b9_3030, 0x010b_68a0, 0x010b_687c, 2);
    }

    #[test]
    fn fn_00b93180_constructs_the_shader() {
        check(0x00b9_3180, 0x010b_6a40, 0x010b_6a20, 2);
    }

    #[test]
    fn fn_00b932d0_constructs_the_shader() {
        check(0x00b9_32d0, 0x010b_6bd8, 0x010b_6bb8, 3);
    }

    #[test]
    fn fn_00b93600_constructs_the_shader() {
        check(0x00b9_3600, 0x010b_6d90, 0x010b_6d70, 4);
    }

    #[test]
    fn fn_00b93950_constructs_the_shader() {
        check(0x00b9_3950, 0x010b_6f38, 0x010b_6f14, 4);
    }

    #[test]
    fn fn_00b93aa0_constructs_the_shader() {
        check(0x00b9_3aa0, 0x010b_70d8, 0x010b_70b8, 4);
    }

    #[test]
    fn fn_00b93bf0_constructs_the_shader() {
        check(0x00b9_3bf0, 0x010b_7278, 0x010b_7258, 4);
    }

    #[test]
    fn fn_00b93d40_constructs_the_shader() {
        check(0x00b9_3d40, 0x010b_7418, 0x010b_73f4, 2);
    }

    #[test]
    fn fn_00b93e90_constructs_the_shader() {
        check(0x00b9_3e90, 0x010b_7598, 0x010b_7578, 2);
    }

    #[test]
    fn fn_00b93fe0_constructs_the_shader() {
        check(0x00b9_3fe0, 0x010b_7720, 0x010b_7700, 2);
    }

    #[test]
    fn fn_00b94130_constructs_the_shader() {
        check(0x00b9_4130, 0x010b_78b8, 0x010b_7898, 2);
    }

    #[test]
    fn fn_00b94280_constructs_the_shader() {
        check(0x00b9_4280, 0x010b_7a50, 0x010b_7a2c, 2);
    }

    #[test]
    fn fn_00b943d0_constructs_the_shader() {
        check(0x00b9_43d0, 0x010b_7bf0, 0x010b_7bcc, 2);
    }
}

#[cfg(test)]
mod b000n_tests {
    use super::*;

    #[test]
    fn fn_00b8bc20_sets_up_the_four_entries() {
        let mut e = Engine::new();
        // The double notes the index and the byte it was given.
        let seen = e.mem.alloc(0x10);
        e.register_double(SET_UP_00B8B270, move |e, a| {
            let value = e.mem.u8(a[2]);
            e.mem.set_u8(seen + a[1], value + 0x10);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x60);
        e.call(0x00b8_bc20, &args![this]);
        assert_eq!(e.mem.bytes(seen, 4), vec![0x11, 0x11, 0x11, 0x10]);
        let log = e.call_log.take().unwrap();
        let targets: Vec<(u32, u32)> = log
            .iter()
            .filter(|c| c.0 == SET_UP_00B8B270)
            .map(|c| (c.1[0], c.1[1]))
            .collect();
        assert_eq!(
            targets,
            vec![
                (this + 0x58, 0),
                (this + 0x58, 1),
                (this + 0x58, 2),
                (this + 0x58, 3)
            ]
        );
    }
}

#[cfg(test)]
mod b000p_tests {
    use super::*;

    #[test]
    fn fn_00b8b270_stores_bytes_and_counts_the_set_ones() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        let bytes = e.mem.alloc(8);
        e.mem.set_u32(array + 4, bytes);
        let value = e.mem.alloc(4);
        // Past the size: the size grows, a set byte counts.
        e.mem.set_u8(value, 1);
        e.call(0x00b8_b270, &args![array, 2u32, value]);
        assert_eq!(e.mem.u16(array + 0xa), 3);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(e.mem.u8(bytes + 2), 1);
        // A clear byte past the size: the size grows, no count.
        e.mem.set_u8(value, 0);
        e.call(0x00b8_b270, &args![array, 4u32, value]);
        assert_eq!(e.mem.u16(array + 0xa), 5);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        // Below the size: set over clear counts up, set over set does not, clear over set down.
        e.mem.set_u8(value, 7);
        e.call(0x00b8_b270, &args![array, 0u32, value]);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        e.call(0x00b8_b270, &args![array, 0u32, value]);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        assert_eq!(e.mem.u8(bytes), 7);
        e.mem.set_u8(value, 0);
        e.call(0x00b8_b270, &args![array, 2u32, value]);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(e.mem.u8(bytes + 2), 0);
        e.call(0x00b8_b270, &args![array, 1u32, value]);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(e.mem.u16(array + 0xa), 5);
    }
}

#[cfg(test)]
mod delete_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The doubles of `~ImageSpaceShader` and the free; the destructor double notes at +0xc8 the
    /// vtable it finds, so the tests see it was set before it runs.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMAGE_SPACE_SHADER_DESTRUCT_00C03A20, |e, a| {
            let vtable = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 0xc8, vtable);
            Ret::default()
        });
        e.register(NI_SIZED_FREE_00AA1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn check_delete(addr: u32, vtable: u32, effect_vtable: u32) {
        for flags in [0u32, 1, 0xfe, 0x101] {
            let mut e = engine();
            let this = e.mem.alloc(0xd0);
            assert_eq!(e.call(addr, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(this + 0xc8), vtable);
            assert_eq!(e.mem.u32(this + SHADER_EFFECT_BASE), effect_vtable);
            assert_eq!(
                calls(&e, IMAGE_SPACE_SHADER_DESTRUCT_00C03A20),
                vec![vec![this]]
            );
            let freed = if flags & 1 != 0 {
                vec![vec![this, 0xd0]]
            } else {
                vec![]
            };
            assert_eq!(calls(&e, NI_SIZED_FREE_00AA1460), freed);
        }
    }

    fn check_effect_delete(addr: u32, vtable: u32, effect_vtable: u32) {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let effect = this + SHADER_EFFECT_BASE;
        assert_eq!(e.call(addr, &args![effect, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), vtable);
        assert_eq!(e.mem.u32(effect), effect_vtable);
        assert_eq!(
            calls(&e, IMAGE_SPACE_SHADER_DESTRUCT_00C03A20),
            vec![vec![this]]
        );
        assert_eq!(calls(&e, NI_SIZED_FREE_00AA1460), vec![vec![this, 0xd0]]);
    }

    #[test]
    fn fn_00b91b50_destroys_without_vtables() {
        for flags in [0u32, 3] {
            let mut e = engine();
            let this = e.mem.alloc(0xd0);
            e.mem.set_u32(this, 0x77);
            assert_eq!(e.call(0x00b9_1b50, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), 0x77);
            assert_eq!(
                calls(&e, IMAGE_SPACE_SHADER_DESTRUCT_00C03A20),
                vec![vec![this]]
            );
            assert_eq!(
                calls(&e, NI_SIZED_FREE_00AA1460).len(),
                (flags & 1) as usize
            );
        }
    }

    #[test]
    fn fn_00b91b40_adjusts_to_the_shader() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        assert_eq!(e.call(0x00b9_1b40, &args![this + 0x6c, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, IMAGE_SPACE_SHADER_DESTRUCT_00C03A20),
            vec![vec![this]]
        );
        assert_eq!(calls(&e, NI_SIZED_FREE_00AA1460), vec![vec![this, 0xd0]]);
    }

    #[test]
    fn fn_00b92450_releases_the_pointer() {
        let mut e = engine();
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let left = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], left);
            left.into_ret()
        });
        e.register(0x0f00_0004, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0f00_0004);
        let held = e.mem.alloc(8);
        e.mem.set_u32(held, vtable);
        // Two references: one given up, nothing deleted.
        e.mem.set_u32(held + 4, 2);
        let this = e.mem.alloc(0xd4);
        e.mem.set_u32(this + 0xd0, held);
        e.call(0x00b9_2450, &args![this]);
        assert_eq!(e.mem.u32(this), 0x010b_5978);
        assert_eq!(e.mem.u32(this + 0x6c), 0x010b_5954);
        assert_eq!(e.mem.u32(held + 4), 1);
        assert!(calls(&e, 0x0f00_0004).is_empty());
        // The last reference: deleted through slot +4.
        e.call(0x00b9_2450, &args![this]);
        assert_eq!(calls(&e, 0x0f00_0004), vec![vec![held]]);
        // No pointer: no decrement.
        e.mem.set_u32(this + 0xd0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b9_2450, &args![this]);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(
            calls(&e, IMAGE_SPACE_SHADER_DESTRUCT_00C03A20),
            vec![vec![this]]
        );
    }

    #[test]
    fn fn_00b924d0_frees_0xd4_bytes() {
        let mut e = engine();
        let this = e.mem.alloc(0xd4);
        assert_eq!(e.call(0x00b9_24d0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010b_5978);
        assert_eq!(calls(&e, NI_SIZED_FREE_00AA1460), vec![vec![this, 0xd4]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b9_24d0, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, NI_SIZED_FREE_00AA1460).is_empty());
    }

    #[test]
    fn fn_00b92440_adjusts_to_the_shader() {
        let mut e = engine();
        let this = e.mem.alloc(0xd4);
        assert_eq!(e.call(0x00b9_2440, &args![this + 0x6c, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010b_5978);
        assert_eq!(calls(&e, NI_SIZED_FREE_00AA1460), vec![vec![this, 0xd4]]);
    }

    /// The volumetric fog shader with a texture block whose index 2 holds `texture`.
    fn fog(e: &mut Engine, texture: u32) -> u32 {
        e.register(RETURN_VOLUMETRIC_FOG_TEXTURE_00B63D10, |_, _| {
            Ret::default()
        });
        e.register(IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00, |_, _| Ret::default());
        e.register(MEMORY_FREE_00AA4060, |_, _| Ret::default());
        e.register(ARRAY_SET_00BA3AA0, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            let block = e.mem.u32(a[0] + 4);
            e.mem.set_u32(block + 4 * a[1], 0);
            Ret::default()
        });
        let this = e.mem.alloc(0xd0);
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x98, block);
        e.mem.set_u32(block + 8, texture);
        this
    }

    #[test]
    fn image_space_shader_volumetric_fog_destructor_drops_texture_two() {
        let mut e = engine();
        let this = fog(&mut e, 0x4400);
        e.call(0x00b8_d4d0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x010b_0518);
        assert_eq!(e.mem.u32(this + 0x6c), 0x010b_04f4);
        assert_eq!(
            calls(&e, RETURN_VOLUMETRIC_FOG_TEXTURE_00B63D10),
            vec![vec![this]]
        );
        assert_eq!(
            calls(&e, IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00),
            vec![vec![0x4400]]
        );
        assert_eq!(
            calls(&e, MEMORY_FREE_00AA4060),
            vec![vec![MEMORY_MANAGER, 0x4400]]
        );
        let set = calls(&e, ARRAY_SET_00BA3AA0);
        assert_eq!(set.len(), 1);
        assert_eq!(&set[0][..2], &[this + 0x94, 2]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(*order.last().unwrap(), IMAGE_SPACE_SHADER_DESTRUCT_00C03A20);
        // No texture at index 2: only the two destructor steps.
        let mut e = engine();
        let this = fog(&mut e, 0);
        e.call(0x00b8_d4d0, &args![this]);
        assert!(calls(&e, ARRAY_SET_00BA3AA0).is_empty());
        assert!(calls(&e, MEMORY_FREE_00AA4060).is_empty());
        assert_eq!(
            calls(&e, IMAGE_SPACE_SHADER_DESTRUCT_00C03A20),
            vec![vec![this]]
        );
    }

    #[test]
    fn fn_00b8d580_frees_the_fog_shader() {
        let mut e = engine();
        let this = fog(&mut e, 0);
        assert_eq!(e.call(0x00b8_d580, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010b_0518);
        assert_eq!(calls(&e, NI_SIZED_FREE_00AA1460), vec![vec![this, 0xd0]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b8_d580, &args![this, 2u32]).u32(), this);
        assert!(calls(&e, NI_SIZED_FREE_00AA1460).is_empty());
    }

    #[test]
    fn fn_00b8d430_adjusts_to_the_fog_shader() {
        let mut e = engine();
        let this = fog(&mut e, 0);
        assert_eq!(e.call(0x00b8_d430, &args![this + 0x6c, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010b_0518);
        assert_eq!(calls(&e, NI_SIZED_FREE_00AA1460), vec![vec![this, 0xd0]]);
    }

    #[test]
    fn fn_00b8bda0_answers_the_byte_at_4() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x1234_5601);
        assert_eq!(e.call(0x00b8_bda0, &args![this]).u32() & 0xff, 1);
        e.mem.set_u8(this + 4, 0);
        assert_eq!(e.call(0x00b8_bda0, &args![this]).u32() & 0xff, 0);
    }

    #[test]
    fn fn_00b8bd50_destroys_the_shader() {
        check_delete(0x00b8_bd50, 0x010a_ffc0, 0x010a_ffa0);
    }

    #[test]
    fn fn_00b8bfa0_destroys_the_shader() {
        check_delete(0x00b8_bfa0, 0x010b_0198, 0x010b_0174);
    }

    #[test]
    fn fn_00b8d6c0_destroys_the_shader() {
        check_delete(0x00b8_d6c0, 0x010b_06c0, 0x010b_06a0);
    }

    #[test]
    fn fn_00b8d810_destroys_the_shader() {
        check_delete(0x00b8_d810, 0x010b_0860, 0x010b_083c);
    }

    #[test]
    fn fn_00b8d950_destroys_the_shader() {
        check_delete(0x00b8_d950, 0x010b_0a00, 0x010b_09dc);
    }

    #[test]
    fn fn_00b8daa0_destroys_the_shader() {
        check_delete(0x00b8_daa0, 0x010b_0b98, 0x010b_0b78);
    }

    #[test]
    fn fn_00b8dbf0_destroys_the_shader() {
        check_delete(0x00b8_dbf0, 0x010b_0d40, 0x010b_0d1c);
    }

    #[test]
    fn fn_00b8dd40_destroys_the_shader() {
        check_delete(0x00b8_dd40, 0x010b_0ec0, 0x010b_0e9c);
    }

    #[test]
    fn fn_00b8de90_destroys_the_shader() {
        check_delete(0x00b8_de90, 0x010b_1040, 0x010b_101c);
    }

    #[test]
    fn fn_00b8dfe0_destroys_the_shader() {
        check_delete(0x00b8_dfe0, 0x010b_11c0, 0x010b_119c);
    }

    #[test]
    fn fn_00b8e130_destroys_the_shader() {
        check_delete(0x00b8_e130, 0x010b_1340, 0x010b_1320);
    }

    #[test]
    fn fn_00b8e280_destroys_the_shader() {
        check_delete(0x00b8_e280, 0x010b_14c0, 0x010b_14a0);
    }

    #[test]
    fn fn_00b8e420_destroys_the_shader() {
        check_delete(0x00b8_e420, 0x010b_1640, 0x010b_1620);
    }

    #[test]
    fn fn_00b8e5c0_destroys_the_shader() {
        check_delete(0x00b8_e5c0, 0x010b_17f8, 0x010b_17d8);
    }

    #[test]
    fn fn_00b8e760_destroys_the_shader() {
        check_delete(0x00b8_e760, 0x010b_1980, 0x010b_1960);
    }

    #[test]
    fn fn_00b8e900_destroys_the_shader() {
        check_delete(0x00b8_e900, 0x010b_1b08, 0x010b_1ae8);
    }

    #[test]
    fn fn_00b8eaa0_destroys_the_shader() {
        check_delete(0x00b8_eaa0, 0x010b_1c90, 0x010b_1c70);
    }

    #[test]
    fn fn_00b8ec40_destroys_the_shader() {
        check_delete(0x00b8_ec40, 0x010b_1e20, 0x010b_1dfc);
    }

    #[test]
    fn fn_00b8ede0_destroys_the_shader() {
        check_delete(0x00b8_ede0, 0x010b_1fb0, 0x010b_1f8c);
    }

    #[test]
    fn fn_00b8ef60_destroys_the_shader() {
        check_delete(0x00b8_ef60, 0x010b_2140, 0x010b_211c);
    }

    #[test]
    fn fn_00b8f120_destroys_the_shader() {
        check_delete(0x00b8_f120, 0x010b_2308, 0x010b_22e8);
    }

    #[test]
    fn fn_00b8f2b0_destroys_the_shader() {
        check_delete(0x00b8_f2b0, 0x010b_2490, 0x010b_2470);
    }

    #[test]
    fn fn_00b8f470_destroys_the_shader() {
        check_delete(0x00b8_f470, 0x010b_2610, 0x010b_25f0);
    }

    #[test]
    fn fn_00b8f620_destroys_the_shader() {
        check_delete(0x00b8_f620, 0x010b_27a8, 0x010b_2784);
    }

    #[test]
    fn fn_00b8f6d0_destroys_the_shader() {
        check_delete(0x00b8_f6d0, 0x010b_2920, 0x010b_28fc);
    }

    #[test]
    fn fn_00b8f7a0_destroys_the_shader() {
        check_delete(0x00b8_f7a0, 0x010b_2a98, 0x010b_2a74);
    }

    #[test]
    fn fn_00b8f8f0_destroys_the_shader() {
        check_delete(0x00b8_f8f0, 0x010b_2c10, 0x010b_2bec);
    }

    #[test]
    fn fn_00b8fa40_destroys_the_shader() {
        check_delete(0x00b8_fa40, 0x010b_2db0, 0x010b_2d8c);
    }

    #[test]
    fn fn_00b8fba0_destroys_the_shader() {
        check_delete(0x00b8_fba0, 0x010b_2f50, 0x010b_2f30);
    }

    #[test]
    fn fn_00b8fd00_destroys_the_shader() {
        check_delete(0x00b8_fd00, 0x010b_30e0, 0x010b_30c0);
    }

    #[test]
    fn fn_00b8fe60_destroys_the_shader() {
        check_delete(0x00b8_fe60, 0x010b_3268, 0x010b_3244);
    }

    #[test]
    fn fn_00b8ffb0_destroys_the_shader() {
        check_delete(0x00b8_ffb0, 0x010b_33f0, 0x010b_33cc);
    }

    #[test]
    fn fn_00b90150_destroys_the_shader() {
        check_delete(0x00b9_0150, 0x010b_3598, 0x010b_3574);
    }

    #[test]
    fn fn_00b902f0_destroys_the_shader() {
        check_delete(0x00b9_02f0, 0x010b_37a8, 0x010b_3788);
    }

    #[test]
    fn fn_00b904e0_destroys_the_shader() {
        check_delete(0x00b9_04e0, 0x010b_3958, 0x010b_3934);
    }

    #[test]
    fn fn_00b90670_destroys_the_shader() {
        check_delete(0x00b9_0670, 0x010b_3af0, 0x010b_3ad0);
    }

    #[test]
    fn fn_00b907f0_destroys_the_shader() {
        check_delete(0x00b9_07f0, 0x010b_3c78, 0x010b_3c58);
    }

    #[test]
    fn fn_00b90980_destroys_the_shader() {
        check_delete(0x00b9_0980, 0x010b_3e18, 0x010b_3df4);
    }

    #[test]
    fn fn_00b90b10_destroys_the_shader() {
        check_delete(0x00b9_0b10, 0x010b_3fa8, 0x010b_3f88);
    }

    #[test]
    fn fn_00b90ca0_destroys_the_shader() {
        check_delete(0x00b9_0ca0, 0x010b_4138, 0x010b_4114);
    }

    #[test]
    fn fn_00b90e30_destroys_the_shader() {
        check_delete(0x00b9_0e30, 0x010b_42c8, 0x010b_42a4);
    }

    #[test]
    fn fn_00b90fc0_destroys_the_shader() {
        check_delete(0x00b9_0fc0, 0x010b_4460, 0x010b_443c);
    }

    #[test]
    fn fn_00b91160_destroys_the_shader() {
        check_delete(0x00b9_1160, 0x010b_4600, 0x010b_45dc);
    }

    #[test]
    fn fn_00b91300_destroys_the_shader() {
        check_delete(0x00b9_1300, 0x010b_47a0, 0x010b_477c);
    }

    #[test]
    fn fn_00b914a0_destroys_the_shader() {
        check_delete(0x00b9_14a0, 0x010b_4938, 0x010b_4914);
    }

    #[test]
    fn fn_00b91640_destroys_the_shader() {
        check_delete(0x00b9_1640, 0x010b_4ad0, 0x010b_4aac);
    }

    #[test]
    fn fn_00b91780_destroys_the_shader() {
        check_delete(0x00b9_1780, 0x010b_4c68, 0x010b_4c48);
    }

    #[test]
    fn fn_00b918c0_destroys_the_shader() {
        check_delete(0x00b9_18c0, 0x010b_4de8, 0x010b_4dc8);
    }

    #[test]
    fn fn_00b91a00_destroys_the_shader() {
        check_delete(0x00b9_1a00, 0x010b_4f80, 0x010b_4f60);
    }

    #[test]
    fn fn_00b91cb0_destroys_the_shader() {
        check_delete(0x00b9_1cb0, 0x010b_52c0, 0x010b_52a0);
    }

    #[test]
    fn fn_00b91e00_destroys_the_shader() {
        check_delete(0x00b9_1e00, 0x010b_5480, 0x010b_5460);
    }

    #[test]
    fn fn_00b91f50_destroys_the_shader() {
        check_delete(0x00b9_1f50, 0x010b_5640, 0x010b_5620);
    }

    #[test]
    fn fn_00b92290_destroys_the_shader() {
        check_delete(0x00b9_2290, 0x010b_57d0, 0x010b_57b0);
    }

    #[test]
    fn fn_00b92570_destroys_the_shader() {
        check_delete(0x00b9_2570, 0x010b_5b10, 0x010b_5af0);
    }

    #[test]
    fn fn_00b927a0_destroys_the_shader() {
        check_delete(0x00b9_27a0, 0x010b_5cb8, 0x010b_5c94);
    }

    #[test]
    fn fn_00b928f0_destroys_the_shader() {
        check_delete(0x00b9_28f0, 0x010b_5ea8, 0x010b_5e84);
    }

    #[test]
    fn fn_00b92a40_destroys_the_shader() {
        check_delete(0x00b9_2a40, 0x010b_6048, 0x010b_6028);
    }

    #[test]
    fn fn_00b92b90_destroys_the_shader() {
        check_delete(0x00b9_2b90, 0x010b_61f0, 0x010b_61d0);
    }

    #[test]
    fn fn_00b92ce0_destroys_the_shader() {
        check_delete(0x00b9_2ce0, 0x010b_6398, 0x010b_6374);
    }

    #[test]
    fn fn_00b92ea0_destroys_the_shader() {
        check_delete(0x00b9_2ea0, 0x010b_6548, 0x010b_6524);
    }

    #[test]
    fn fn_00b92ff0_destroys_the_shader() {
        check_delete(0x00b9_2ff0, 0x010b_66f0, 0x010b_66cc);
    }

    #[test]
    fn fn_00b93140_destroys_the_shader() {
        check_delete(0x00b9_3140, 0x010b_68a0, 0x010b_687c);
    }

    #[test]
    fn fn_00b93290_destroys_the_shader() {
        check_delete(0x00b9_3290, 0x010b_6a40, 0x010b_6a20);
    }

    #[test]
    fn fn_00b933e0_destroys_the_shader() {
        check_delete(0x00b9_33e0, 0x010b_6bd8, 0x010b_6bb8);
    }

    #[test]
    fn fn_00b93710_destroys_the_shader() {
        check_delete(0x00b9_3710, 0x010b_6d90, 0x010b_6d70);
    }

    #[test]
    fn fn_00b93a60_destroys_the_shader() {
        check_delete(0x00b9_3a60, 0x010b_6f38, 0x010b_6f14);
    }

    #[test]
    fn fn_00b93bb0_destroys_the_shader() {
        check_delete(0x00b9_3bb0, 0x010b_70d8, 0x010b_70b8);
    }

    #[test]
    fn fn_00b93d00_destroys_the_shader() {
        check_delete(0x00b9_3d00, 0x010b_7278, 0x010b_7258);
    }

    #[test]
    fn fn_00b93e50_destroys_the_shader() {
        check_delete(0x00b9_3e50, 0x010b_7418, 0x010b_73f4);
    }

    #[test]
    fn fn_00b93fa0_destroys_the_shader() {
        check_delete(0x00b9_3fa0, 0x010b_7598, 0x010b_7578);
    }

    #[test]
    fn fn_00b940f0_destroys_the_shader() {
        check_delete(0x00b9_40f0, 0x010b_7720, 0x010b_7700);
    }

    #[test]
    fn fn_00b94240_destroys_the_shader() {
        check_delete(0x00b9_4240, 0x010b_78b8, 0x010b_7898);
    }

    #[test]
    fn fn_00b94390_destroys_the_shader() {
        check_delete(0x00b9_4390, 0x010b_7a50, 0x010b_7a2c);
    }

    #[test]
    fn fn_00b944e0_destroys_the_shader() {
        check_delete(0x00b9_44e0, 0x010b_7bf0, 0x010b_7bcc);
    }

    #[test]
    fn fn_00b8bd80_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_bd80, 0x010a_ffc0, 0x010a_ffa0);
    }

    #[test]
    fn fn_00b8bfd0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_bfd0, 0x010b_0198, 0x010b_0174);
    }

    #[test]
    fn fn_00b8d6f0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_d6f0, 0x010b_06c0, 0x010b_06a0);
    }

    #[test]
    fn fn_00b8d840_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_d840, 0x010b_0860, 0x010b_083c);
    }

    #[test]
    fn fn_00b8d980_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_d980, 0x010b_0a00, 0x010b_09dc);
    }

    #[test]
    fn fn_00b8dad0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_dad0, 0x010b_0b98, 0x010b_0b78);
    }

    #[test]
    fn fn_00b8dc20_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_dc20, 0x010b_0d40, 0x010b_0d1c);
    }

    #[test]
    fn fn_00b8dd70_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_dd70, 0x010b_0ec0, 0x010b_0e9c);
    }

    #[test]
    fn fn_00b8dec0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_dec0, 0x010b_1040, 0x010b_101c);
    }

    #[test]
    fn fn_00b8e010_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_e010, 0x010b_11c0, 0x010b_119c);
    }

    #[test]
    fn fn_00b8e160_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_e160, 0x010b_1340, 0x010b_1320);
    }

    #[test]
    fn fn_00b8e2b0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_e2b0, 0x010b_14c0, 0x010b_14a0);
    }

    #[test]
    fn fn_00b8e450_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_e450, 0x010b_1640, 0x010b_1620);
    }

    #[test]
    fn fn_00b8e5f0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_e5f0, 0x010b_17f8, 0x010b_17d8);
    }

    #[test]
    fn fn_00b8e790_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_e790, 0x010b_1980, 0x010b_1960);
    }

    #[test]
    fn fn_00b8e930_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_e930, 0x010b_1b08, 0x010b_1ae8);
    }

    #[test]
    fn fn_00b8ead0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_ead0, 0x010b_1c90, 0x010b_1c70);
    }

    #[test]
    fn fn_00b8ec70_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_ec70, 0x010b_1e20, 0x010b_1dfc);
    }

    #[test]
    fn fn_00b8ee10_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_ee10, 0x010b_1fb0, 0x010b_1f8c);
    }

    #[test]
    fn fn_00b8ef90_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_ef90, 0x010b_2140, 0x010b_211c);
    }

    #[test]
    fn fn_00b8f150_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_f150, 0x010b_2308, 0x010b_22e8);
    }

    #[test]
    fn fn_00b8f2e0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_f2e0, 0x010b_2490, 0x010b_2470);
    }

    #[test]
    fn fn_00b8f4a0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_f4a0, 0x010b_2610, 0x010b_25f0);
    }

    #[test]
    fn fn_00b8f650_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_f650, 0x010b_27a8, 0x010b_2784);
    }

    #[test]
    fn fn_00b8f700_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_f700, 0x010b_2920, 0x010b_28fc);
    }

    #[test]
    fn fn_00b8f7d0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_f7d0, 0x010b_2a98, 0x010b_2a74);
    }

    #[test]
    fn fn_00b8f920_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_f920, 0x010b_2c10, 0x010b_2bec);
    }

    #[test]
    fn fn_00b8fa70_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_fa70, 0x010b_2db0, 0x010b_2d8c);
    }

    #[test]
    fn fn_00b8fbd0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_fbd0, 0x010b_2f50, 0x010b_2f30);
    }

    #[test]
    fn fn_00b8fd30_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_fd30, 0x010b_30e0, 0x010b_30c0);
    }

    #[test]
    fn fn_00b8fe90_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_fe90, 0x010b_3268, 0x010b_3244);
    }

    #[test]
    fn fn_00b8ffe0_adjusts_to_the_shader() {
        check_effect_delete(0x00b8_ffe0, 0x010b_33f0, 0x010b_33cc);
    }

    #[test]
    fn fn_00b90180_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_0180, 0x010b_3598, 0x010b_3574);
    }

    #[test]
    fn fn_00b90320_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_0320, 0x010b_37a8, 0x010b_3788);
    }

    #[test]
    fn fn_00b90510_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_0510, 0x010b_3958, 0x010b_3934);
    }

    #[test]
    fn fn_00b906a0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_06a0, 0x010b_3af0, 0x010b_3ad0);
    }

    #[test]
    fn fn_00b90820_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_0820, 0x010b_3c78, 0x010b_3c58);
    }

    #[test]
    fn fn_00b909b0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_09b0, 0x010b_3e18, 0x010b_3df4);
    }

    #[test]
    fn fn_00b90b40_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_0b40, 0x010b_3fa8, 0x010b_3f88);
    }

    #[test]
    fn fn_00b90cd0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_0cd0, 0x010b_4138, 0x010b_4114);
    }

    #[test]
    fn fn_00b90e60_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_0e60, 0x010b_42c8, 0x010b_42a4);
    }

    #[test]
    fn fn_00b90ff0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_0ff0, 0x010b_4460, 0x010b_443c);
    }

    #[test]
    fn fn_00b91190_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_1190, 0x010b_4600, 0x010b_45dc);
    }

    #[test]
    fn fn_00b91330_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_1330, 0x010b_47a0, 0x010b_477c);
    }

    #[test]
    fn fn_00b914d0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_14d0, 0x010b_4938, 0x010b_4914);
    }

    #[test]
    fn fn_00b91670_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_1670, 0x010b_4ad0, 0x010b_4aac);
    }

    #[test]
    fn fn_00b917b0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_17b0, 0x010b_4c68, 0x010b_4c48);
    }

    #[test]
    fn fn_00b918f0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_18f0, 0x010b_4de8, 0x010b_4dc8);
    }

    #[test]
    fn fn_00b91a30_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_1a30, 0x010b_4f80, 0x010b_4f60);
    }

    #[test]
    fn fn_00b91ce0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_1ce0, 0x010b_52c0, 0x010b_52a0);
    }

    #[test]
    fn fn_00b91e30_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_1e30, 0x010b_5480, 0x010b_5460);
    }

    #[test]
    fn fn_00b91f80_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_1f80, 0x010b_5640, 0x010b_5620);
    }

    #[test]
    fn fn_00b922c0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_22c0, 0x010b_57d0, 0x010b_57b0);
    }

    #[test]
    fn fn_00b925a0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_25a0, 0x010b_5b10, 0x010b_5af0);
    }

    #[test]
    fn fn_00b927d0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_27d0, 0x010b_5cb8, 0x010b_5c94);
    }

    #[test]
    fn fn_00b92920_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_2920, 0x010b_5ea8, 0x010b_5e84);
    }

    #[test]
    fn fn_00b92a70_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_2a70, 0x010b_6048, 0x010b_6028);
    }

    #[test]
    fn fn_00b92bc0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_2bc0, 0x010b_61f0, 0x010b_61d0);
    }

    #[test]
    fn fn_00b92d10_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_2d10, 0x010b_6398, 0x010b_6374);
    }

    #[test]
    fn fn_00b92ed0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_2ed0, 0x010b_6548, 0x010b_6524);
    }

    #[test]
    fn fn_00b93020_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_3020, 0x010b_66f0, 0x010b_66cc);
    }

    #[test]
    fn fn_00b93170_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_3170, 0x010b_68a0, 0x010b_687c);
    }

    #[test]
    fn fn_00b932c0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_32c0, 0x010b_6a40, 0x010b_6a20);
    }

    #[test]
    fn fn_00b93410_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_3410, 0x010b_6bd8, 0x010b_6bb8);
    }

    #[test]
    fn fn_00b93740_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_3740, 0x010b_6d90, 0x010b_6d70);
    }

    #[test]
    fn fn_00b93a90_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_3a90, 0x010b_6f38, 0x010b_6f14);
    }

    #[test]
    fn fn_00b93be0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_3be0, 0x010b_70d8, 0x010b_70b8);
    }

    #[test]
    fn fn_00b93d30_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_3d30, 0x010b_7278, 0x010b_7258);
    }

    #[test]
    fn fn_00b93e80_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_3e80, 0x010b_7418, 0x010b_73f4);
    }

    #[test]
    fn fn_00b93fd0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_3fd0, 0x010b_7598, 0x010b_7578);
    }

    #[test]
    fn fn_00b94120_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_4120, 0x010b_7720, 0x010b_7700);
    }

    #[test]
    fn fn_00b94270_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_4270, 0x010b_78b8, 0x010b_7898);
    }

    #[test]
    fn fn_00b943c0_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_43c0, 0x010b_7a50, 0x010b_7a2c);
    }

    #[test]
    fn fn_00b94510_adjusts_to_the_shader() {
        check_effect_delete(0x00b9_4510, 0x010b_7bf0, 0x010b_7bcc);
    }
}

#[cfg(test)]
mod reload_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::{
        NEW_RENDER_STATES_00E7F5D0, SET_RENDER_STATE_00E7F430,
    };

    /// What a shader reload passes to its vtable slot +0x13c for one value of the byte at
    /// `011f94fc`: the two descriptions (read through their pointers), the name and the two
    /// profiles; and the sizes it gives `memset`, in order.
    struct Reload {
        flag: u8,
        vertex: &'static [u32],
        pixel: &'static [u32],
        words: [u32; 3],
        memsets: &'static [u32],
    }

    /// What the pixel profile double answers.
    const PROFILE: u32 = 0x0f00_0bad;
    /// The doubles in the vtable slots +0x13c and +0x14c.
    const LOAD_SHADERS: u32 = 0x0f00_013c;
    const AFTER_LOAD: u32 = 0x0f00_014c;

    type Seen = (Vec<u32>, Vec<u32>, Vec<u32>);

    fn engine(flag: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(TEST_SHADERS_011F94FC, flag);
        e.register(MEMSET_00EC61C0, |e, a| {
            for index in 0..a[2] {
                e.mem.set_u8(a[0] + index, a[1] as u8);
            }
            a[0].into_ret()
        });
        e.register(PIXEL_PROFILE_00B4F380, |_, a| {
            assert_eq!(a, [1]);
            PROFILE.into_ret()
        });
        e.register(AFTER_LOAD, |_, _| Ret::default());
        e.register(NEW_RENDER_STATES_00E7F5D0, |_, _| 0x4400u32.into_ret());
        e.register(SET_RENDER_STATE_00E7F430, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x160);
        e.mem.set_u32(vtable + 0x13c, LOAD_SHADERS);
        e.mem.set_u32(vtable + 0x14c, AFTER_LOAD);
        let this = e.mem.alloc(0xd0);
        e.mem.set_u32(this, vtable);
        let pass = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x5c, pass);
        (e, this)
    }

    fn check_reload(addr: u32, cases: &[Reload]) {
        let padded = |words: &[u32]| {
            let mut all = words.to_vec();
            all.resize(19, 0);
            all
        };
        for case in cases {
            let (mut e, this) = engine(case.flag);
            let seen = std::rc::Rc::new(std::cell::RefCell::new(None::<Seen>));
            let keep = seen.clone();
            e.register_double(LOAD_SHADERS, move |e, a| {
                let read = |e: &Engine, p: u32| (0..19).map(|i| e.mem.u32(p + 4 * i)).collect();
                *keep.borrow_mut() = Some((read(e, a[1]), read(e, a[2]), a[3..6].to_vec()));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(addr, &args![this]);
            let (vertex, pixel, words) = seen.borrow_mut().take().expect("slot +0x13c called");
            assert_eq!(
                vertex,
                padded(case.vertex),
                "{addr:#x} vertex, flag {}",
                case.flag
            );
            assert_eq!(
                pixel,
                padded(case.pixel),
                "{addr:#x} pixel, flag {}",
                case.flag
            );
            assert_eq!(words, case.words, "{addr:#x} words, flag {}", case.flag);
            let log = e.call_log.take().unwrap();
            let sizes: Vec<u32> = log
                .iter()
                .filter(|c| c.0 == MEMSET_00EC61C0)
                .map(|c| c.1[2])
                .collect();
            assert_eq!(sizes, case.memsets, "{addr:#x} memsets");
            let load = log.iter().position(|c| c.0 == LOAD_SHADERS).unwrap();
            let after = log.iter().position(|c| c.0 == AFTER_LOAD).unwrap();
            assert!(load < after);
            assert_eq!(log[after].1, vec![this]);
        }
    }

    fn render_states(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == SET_RENDER_STATE_00E7F430)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn scanline_reloads_set_their_render_states() {
        for addr in [0x00b8_bdb0u32, 0x00b9_25b0] {
            let (mut e, this) = engine(0);
            e.register(LOAD_SHADERS, |_, _| Ret::default());
            let pass = e.mem.u32(this + 0x5c);
            e.call_log = Some(vec![]);
            e.call(addr, &args![this]);
            // The block is made once, on the first state.
            assert_eq!(e.mem.u32(pass + 0x30), 0x4400);
            assert_eq!(
                render_states(&e),
                vec![
                    vec![0x4400, 0x1b, 1, 1],
                    vec![0x4400, 0x13, 5, 1],
                    vec![0x4400, 0x14, 6, 1]
                ]
            );
            let log = e.call_log.as_ref().unwrap();
            let after = log.iter().position(|c| c.0 == AFTER_LOAD).unwrap();
            let first = log
                .iter()
                .position(|c| c.0 == SET_RENDER_STATE_00E7F430)
                .unwrap();
            assert!(after < first);
            assert_eq!(
                log.iter()
                    .filter(|c| c.0 == NEW_RENDER_STATES_00E7F5D0)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn fn_00b8d460_sets_three_render_states() {
        let (mut e, this) = engine(0);
        let pass = e.mem.u32(this + 0x5c);
        e.mem.set_u32(pass + 0x30, 0x5500);
        e.call_log = Some(vec![]);
        e.call(0x00b8_d460, &args![this]);
        assert_eq!(
            render_states(&e),
            vec![
                vec![0x5500, 0x34, 1, 1],
                vec![0x5500, 0x38, 3, 1],
                vec![0x5500, 0x39, 1, 1]
            ]
        );
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|c| c.0 == NEW_RENDER_STATES_00E7F5D0));
    }

    #[test]
    fn fn_00b92d20_sets_them_only_with_its_flag() {
        let (mut e, this) = engine(0);
        e.map(0x011a_d000, 0x1000);
        e.set_global(FLAG_011AD832, 0u8);
        e.call_log = Some(vec![]);
        e.call(0x00b9_2d20, &args![this]);
        assert!(render_states(&e).is_empty());
        e.set_global(FLAG_011AD832, 2u8);
        e.call(0x00b9_2d20, &args![this]);
        assert_eq!(
            render_states(&e),
            vec![
                vec![0x4400, 0x34, 1, 1],
                vec![0x4400, 0x38, 3, 1],
                vec![0x4400, 0x39, 1, 1]
            ]
        );
    }

    #[test]
    fn fn_00b8bdb0_loads_its_shaders() {
        check_reload(
            0x00b8_bdb0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0130, 0x010b_0124, 0, 0x0106_3538],
                    words: [0x010b_0114, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0130, 0x010b_0124, 0, 0x0106_3538],
                    words: [0x010b_0114, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8bf10_loads_its_shaders() {
        check_reload(
            0x00b8_bf10,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_02f8],
                    words: [0x010b_02ec, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_02f8],
                    words: [0x010b_02ec, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8c0e0_loads_its_shaders() {
        check_reload(
            0x00b8_c0e0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0490],
                    words: [0x010b_048c, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0490],
                    words: [0x010b_048c, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8d3a0_loads_its_shaders() {
        check_reload(
            0x00b8_d3a0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0674],
                    words: [0x010b_066c, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x40, 0x40],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0674],
                    words: [0x010b_066c, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x40, 0x40],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8d620_loads_its_shaders() {
        check_reload(
            0x00b8_d620,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820, 0x010b_0814, 0x0101_1584],
                    words: [0x010b_0814, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820, 0x010b_0814, 0x0101_1584],
                    words: [0x010b_0814, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8d770_loads_its_shaders() {
        check_reload(
            0x00b8_d770,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_09b4],
                    words: [0x0109_ab38, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_09b4],
                    words: [0x0109_ab38, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8d8c0_loads_its_shaders() {
        check_reload(
            0x00b8_d8c0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0b5c],
                    words: [0x010b_0b54, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0b5c],
                    words: [0x010b_0b54, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8da00_loads_its_shaders() {
        check_reload(
            0x00b8_da00,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x0106_3f30],
                    words: [0x010b_0cec, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x0106_3f30],
                    words: [0x010b_0cec, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8db50_loads_its_shaders() {
        check_reload(
            0x00b8_db50,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x0106_3f18],
                    words: [0x010b_0e94, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x0106_3f18],
                    words: [0x010b_0e94, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8dca0_loads_its_shaders() {
        check_reload(
            0x00b8_dca0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x0106_3f00],
                    words: [0x010b_1014, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x0106_3f00],
                    words: [0x010b_1014, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8ddf0_loads_its_shaders() {
        check_reload(
            0x00b8_ddf0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x0106_3ee8],
                    words: [0x010b_1194, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x0106_3ee8],
                    words: [0x010b_1194, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8df40_loads_its_shaders() {
        check_reload(
            0x00b8_df40,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x010b_131c],
                    words: [0x010b_1314, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x010b_131c],
                    words: [0x010b_1314, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8e090_loads_its_shaders() {
        check_reload(
            0x00b8_e090,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x010b_149c],
                    words: [0x010b_1494, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x010b_149c],
                    words: [0x010b_1494, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8e1e0_loads_its_shaders() {
        check_reload(
            0x00b8_e1e0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x010b_161c],
                    words: [0x010b_1614, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0cfc, 0x010b_0cf4, 0x010b_161c],
                    words: [0x010b_1614, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8ee90_loads_its_shaders() {
        check_reload(
            0x00b8_ee90,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_22bc,
                        0x010b_22b4,
                        0x0101_1584,
                        0x010b_22a8,
                        0x0101_1584,
                        0x010b_22a0,
                        0x0101_1584,
                        0x010b_2294,
                        0x0101_1584,
                    ],
                    words: [0x0105_ea48, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_22bc,
                        0x010b_22b4,
                        0x0101_1584,
                        0x010b_22a8,
                        0x0101_1584,
                        0x010b_22a0,
                        0x0101_1584,
                        0x010b_2294,
                        0x0101_1584,
                    ],
                    words: [0x0105_ea48, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8f080_loads_its_shaders() {
        check_reload(
            0x00b8_f080,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820, 0x010b_2464, 0x0101_1584],
                    words: [0x010b_245c, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820, 0x010b_2464, 0x0101_1584],
                    words: [0x010b_245c, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8f220_loads_its_shaders() {
        check_reload(
            0x00b8_f220,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820],
                    words: [0x010b_25e4, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820],
                    words: [0x010b_25e4, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8f3d0_loads_its_shaders() {
        check_reload(
            0x00b8_f3d0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820, 0x010b_277c, 0x0101_1584],
                    words: [0x010b_2764, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820, 0x010b_277c, 0x0101_1584],
                    words: [0x010b_2764, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8f590_loads_its_shaders() {
        check_reload(
            0x00b8_f590,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158, 0x010b_277c, 0x0101_1584],
                    pixel: &[],
                    words: [0x0101_f7a8, 0x010a_93ec, 0],
                    memsets: &[0x3c, 0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158, 0x010b_277c, 0x0101_1584],
                    pixel: &[],
                    words: [0x0101_f7a8, 0x010a_93ec, 0],
                    memsets: &[0x3c, 0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8f710_loads_its_shaders() {
        check_reload(
            0x00b8_f710,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820],
                    words: [0x010b_22e0, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820],
                    words: [0x010b_22e0, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8f850_loads_its_shaders() {
        check_reload(
            0x00b8_f850,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_2d68],
                    words: [0x010b_2d64, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x40],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_2d68],
                    words: [0x010b_2d64, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x40],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8f9a0_loads_its_shaders() {
        check_reload(
            0x00b8_f9a0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_2f0c],
                    words: [0x010b_2f04, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x40],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_2f0c],
                    words: [0x010b_2f04, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x40],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8faf0_loads_its_shaders() {
        check_reload(
            0x00b8_faf0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_2f0c, 0x010b_30b4, 0x010b_30ac],
                    words: [0x010b_30a4, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_2f0c, 0x010b_30b4, 0x010b_30ac],
                    words: [0x010b_30a4, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8fc50_loads_its_shaders() {
        check_reload(
            0x00b8_fc50,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_2f0c, 0x010b_30b4, 0x010b_323c],
                    words: [0x010b_3234, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_2f0c, 0x010b_30b4, 0x010b_323c],
                    words: [0x010b_3234, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8fdb0_loads_its_shaders() {
        check_reload(
            0x00b8_fdb0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_2d68, 0x010b_33c4, 0x0101_1584],
                    words: [0x010b_33bc, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_2d68, 0x010b_33c4, 0x0101_1584],
                    words: [0x010b_33bc, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b8ff10_loads_its_shaders() {
        check_reload(
            0x00b8_ff10,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_3550],
                    words: [0x010b_3544, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x3c, 0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_3550],
                    words: [0x010b_3544, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x3c, 0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b916f0_loads_its_shaders() {
        check_reload(
            0x00b9_16f0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0130, 0x010b_0124, 0x0101_1584],
                    words: [0x010b_4dbc, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x40],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0130, 0x010b_0124, 0x0101_1584],
                    words: [0x010b_4dbc, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x40],
                },
            ],
        );
    }

    #[test]
    fn fn_00b91830_loads_its_shaders() {
        check_reload(
            0x00b9_1830,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_4f40],
                    words: [0x010b_4f3c, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x40, 0x40],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_4f40],
                    words: [0x010b_4f3c, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x40, 0x40],
                },
            ],
        );
    }

    #[test]
    fn fn_00b91970_loads_its_shaders() {
        check_reload(
            0x00b9_1970,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_50fc],
                    pixel: &[0x010b_50dc],
                    words: [0x010b_50d4, 0x010a_93e4, 0x010a_93cc],
                    memsets: &[0x40, 0x40],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_50fc],
                    pixel: &[0x010b_50dc],
                    words: [0x010b_50d4, 0x010a_93e4, 0x010a_93cc],
                    memsets: &[0x40, 0x40],
                },
            ],
        );
    }

    #[test]
    fn fn_00b91ab0_loads_its_shaders() {
        check_reload(
            0x00b9_1ab0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820],
                    words: [0x010b_5294, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_0820],
                    words: [0x010b_5294, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b91bf0_loads_its_shaders() {
        check_reload(
            0x00b9_1bf0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[
                        0x010b_543c,
                        0x010b_5430,
                        0x010b_5428,
                        0x010b_541c,
                        0x0101_1584,
                    ],
                    words: [0x010b_5414, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x34],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[
                        0x010b_543c,
                        0x010b_5430,
                        0x010b_5428,
                        0x010b_541c,
                        0x0101_1584,
                    ],
                    words: [0x010b_5414, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x34],
                },
            ],
        );
    }

    #[test]
    fn fn_00b91d60_loads_its_shaders() {
        check_reload(
            0x00b9_1d60,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_55fc, 0x010b_55e8],
                    words: [0x010b_55d4, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_55fc, 0x010b_55e8],
                    words: [0x010b_55d4, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b91eb0_loads_its_shaders() {
        check_reload(
            0x00b9_1eb0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_55fc, 0x010b_57a4],
                    words: [0x010b_5794, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_55fc, 0x010b_57a4],
                    words: [0x010b_5794, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b921f0_loads_its_shaders() {
        check_reload(
            0x00b9_21f0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_592c],
                    words: [0x010b_5924, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x40],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158, 0x010b_09d4, 0x0101_1584],
                    pixel: &[0x010b_592c],
                    words: [0x010b_5924, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x38, 0x40],
                },
            ],
        );
    }

    #[test]
    fn fn_00b92390_loads_its_shaders() {
        check_reload(
            0x00b9_2390,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_5ad4, 0x010b_33c4, 0x0101_1584],
                    pixel: &[0x010b_0820, 0x010b_33c4, 0x0101_1584],
                    words: [0x010b_5acc, 0x010a_93ec, PROFILE],
                    memsets: &[0x3c, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_5ad4, 0x010b_33c4, 0x0101_1584],
                    pixel: &[0x010b_0820, 0x010b_33c4, 0x0101_1584],
                    words: [0x010b_5acc, 0x010a_93ec, PROFILE],
                    memsets: &[0x3c, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b925b0_loads_its_shaders() {
        check_reload(
            0x00b9_25b0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_5c70],
                    words: [0x010b_5c64, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x44],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_5c70],
                    words: [0x010b_5c64, 0x010a_93ec, 0x010a_93cc],
                    memsets: &[0x44, 0x44],
                },
            ],
        );
    }

    #[test]
    fn fn_00b92700_loads_its_shaders() {
        check_reload(
            0x00b9_2700,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_5e64],
                    pixel: &[0x010b_5e44, 0x010b_5e30],
                    words: [0x010b_5e0c, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_5e64],
                    pixel: &[0x010b_5e44, 0x010b_5e30],
                    words: [0x010b_5e0c, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b92850_loads_its_shaders() {
        check_reload(
            0x00b9_2850,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_5e64],
                    pixel: &[0x010b_5e44, 0x010b_6018],
                    words: [0x010b_5ffc, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_5e64],
                    pixel: &[0x010b_5e44, 0x010b_6018],
                    words: [0x010b_5ffc, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b929a0_loads_its_shaders() {
        check_reload(
            0x00b9_29a0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_5e64, 0x010b_61bc],
                    pixel: &[0x010b_5e44, 0x010b_61bc],
                    words: [0x010b_619c, 0x010a_93ec, PROFILE],
                    memsets: &[0x3c, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_5e64, 0x010b_61bc],
                    pixel: &[0x010b_5e44, 0x010b_61bc],
                    words: [0x010b_619c, 0x010a_93ec, PROFILE],
                    memsets: &[0x3c, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b92af0_loads_its_shaders() {
        check_reload(
            0x00b9_2af0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_5e64, 0x010b_6360],
                    pixel: &[0x010b_5e44, 0x010b_6360],
                    words: [0x010b_6344, 0x010a_93ec, PROFILE],
                    memsets: &[0x3c, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_5e64, 0x010b_6360],
                    pixel: &[0x010b_5e44, 0x010b_6360],
                    words: [0x010b_6344, 0x010a_93ec, PROFILE],
                    memsets: &[0x3c, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b92c40_loads_its_shaders() {
        check_reload(
            0x00b9_2c40,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_5e64, 0x010b_6510],
                    pixel: &[0x010b_5e44, 0x010b_6510],
                    words: [0x010b_64ec, 0x010a_93ec, PROFILE],
                    memsets: &[0x3c, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_5e64, 0x010b_6510],
                    pixel: &[0x010b_5e44, 0x010b_6510],
                    words: [0x010b_64ec, 0x010a_93ec, PROFILE],
                    memsets: &[0x3c, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b92e00_loads_its_shaders() {
        check_reload(
            0x00b9_2e00,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_5e64],
                    pixel: &[0x010b_5e44, 0x010b_66bc],
                    words: [0x010b_669c, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_5e64],
                    pixel: &[0x010b_5e44, 0x010b_66bc],
                    words: [0x010b_669c, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b92f50_loads_its_shaders() {
        check_reload(
            0x00b9_2f50,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_5e64],
                    pixel: &[0x010b_5e44, 0x010b_6868],
                    words: [0x010b_6844, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_5e64],
                    pixel: &[0x010b_5e44, 0x010b_6868],
                    words: [0x010b_6844, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b930a0_loads_its_shaders() {
        check_reload(
            0x00b9_30a0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_5e64, 0x010b_6a0c],
                    pixel: &[0x010b_5e44, 0x010b_6a0c],
                    words: [0x010b_69f4, 0x010a_93ec, PROFILE],
                    memsets: &[0x3c, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_5e64, 0x010b_6a0c],
                    pixel: &[0x010b_5e44, 0x010b_6a0c],
                    words: [0x010b_69f4, 0x010a_93ec, PROFILE],
                    memsets: &[0x3c, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b931f0_loads_its_shaders() {
        check_reload(
            0x00b9_31f0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_5e64],
                    pixel: &[0x010b_5e44, 0x010b_6bb0],
                    words: [0x010b_6b94, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_5e64],
                    pixel: &[0x010b_5e44, 0x010b_6bb0],
                    words: [0x010b_6b94, 0x010a_93ec, PROFILE],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn fn_00b93340_loads_its_shaders() {
        check_reload(
            0x00b9_3340,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_6d40],
                    words: [0x010b_6d2c, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_6d40],
                    words: [0x010b_6d2c, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b93670_loads_its_shaders() {
        check_reload(
            0x00b9_3670,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_6efc],
                    words: [0x010b_6ee4, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_6efc],
                    words: [0x010b_6ee4, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b939c0_loads_its_shaders() {
        check_reload(
            0x00b9_39c0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_70a4],
                    words: [0x010b_708c, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_70a4],
                    words: [0x010b_708c, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b93b10_loads_its_shaders() {
        check_reload(
            0x00b9_3b10,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_7244],
                    words: [0x010b_722c, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_7244],
                    words: [0x010b_722c, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b93c60_loads_its_shaders() {
        check_reload(
            0x00b9_3c60,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_73e0],
                    words: [0x010b_73cc, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_73e0],
                    words: [0x010b_73cc, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b93db0_loads_its_shaders() {
        check_reload(
            0x00b9_3db0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_6bb0],
                    words: [0x010b_756c, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_6bb0],
                    words: [0x010b_756c, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b93f00_loads_its_shaders() {
        check_reload(
            0x00b9_3f00,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_76f8],
                    words: [0x010b_76ec, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_76f8],
                    words: [0x010b_76ec, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b94050_loads_its_shaders() {
        check_reload(
            0x00b9_4050,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_7888],
                    words: [0x010b_7874, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_7888],
                    words: [0x010b_7874, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b941a0_loads_its_shaders() {
        check_reload(
            0x00b9_41a0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_7a1c],
                    words: [0x010b_7a0c, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_7a1c],
                    words: [0x010b_7a0c, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b942f0_loads_its_shaders() {
        check_reload(
            0x00b9_42f0,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_7bb8],
                    words: [0x010b_7ba4, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_7bb8],
                    words: [0x010b_7ba4, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn fn_00b94440_loads_its_shaders() {
        check_reload(
            0x00b9_4440,
            &[
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_7d5c],
                    words: [0x010b_7d44, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[0x010b_6d50, 0x010b_7d5c],
                    words: [0x010b_7d44, 0x010a_93ec, PROFILE],
                    memsets: &[0x40, 0x38],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_bright_pass_blur3_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b8_e330,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_0cfc,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x0106_3f30,
                    ],
                    words: [0x010b_1794, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_17a8,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x0106_3f30,
                    ],
                    words: [0x010b_179c, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_bright_pass_blur5_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b8_e4d0,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_0cfc,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x0106_3f18,
                    ],
                    words: [0x010b_194c, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_17a8,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x0106_3f18,
                    ],
                    words: [0x010b_1954, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_bright_pass_blur7_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b8_e670,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_0cfc,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x0106_3f00,
                    ],
                    words: [0x010b_1ad4, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_17a8,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x0106_3f00,
                    ],
                    words: [0x010b_1adc, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_bright_pass_blur9_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b8_e810,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_0cfc,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x0106_3ee8,
                    ],
                    words: [0x010b_1c5c, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_17a8,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x0106_3ee8,
                    ],
                    words: [0x010b_1c64, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_bright_pass_blur11_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b8_e9b0,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_0cfc,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x010b_131c,
                    ],
                    words: [0x010b_1de4, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_17a8,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x010b_131c,
                    ],
                    words: [0x010b_1df0, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_bright_pass_blur13_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b8_eb50,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_0cfc,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x010b_149c,
                    ],
                    words: [0x010b_1f74, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_17a8,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x010b_149c,
                    ],
                    words: [0x010b_1f80, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_bright_pass_blur15_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b8_ecf0,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_0cfc,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x010b_161c,
                    ],
                    words: [0x010b_2104, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_0158],
                    pixel: &[
                        0x010b_17a8,
                        0x010b_17cc,
                        0x0101_1584,
                        0x010b_0cf4,
                        0x010b_161c,
                    ],
                    words: [0x010b_2110, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_blend_in_shader_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_0060,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768, 0x010b_09d4],
                    pixel: &[0x010b_3700, 0x010b_3760, 0, 0x010b_3758],
                    words: [0x010b_36ec, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x3c, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768, 0x010b_09d4],
                    pixel: &[0x010b_3734, 0x010b_3760, 0, 0x010b_3758],
                    words: [0x010b_3720, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x3c, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_blend_in_shader_cinematic_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_0200,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768, 0x010b_09d4],
                    pixel: &[0x010b_3700, 0x010b_3928, 0, 0x010b_3760, 0, 0x010b_3758],
                    words: [0x010b_38fc, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x3c, 0x2c],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768, 0x010b_09d4],
                    pixel: &[0x010b_3734, 0x010b_3928, 0, 0x010b_3760, 0, 0x010b_3758],
                    words: [0x010b_3910, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x3c, 0x2c],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_blend_in_shader_cinematic_alphamask_reload_shaders_loads_its_shaders()
    {
        check_reload(
            0x00b9_03a0,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768, 0x010b_09d4],
                    pixel: &[
                        0x010b_3700,
                        0x010b_3ac4,
                        0,
                        0x010b_3928,
                        0,
                        0x010b_3760,
                        0,
                        0x010b_3758,
                    ],
                    words: [0x010b_3aac, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x3c],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768, 0x010b_09d4],
                    pixel: &[0x010b_3734, 0x010b_3928, 0, 0x010b_3760, 0, 0x010b_3758],
                    words: [0x010b_3910, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x3c, 0x2c],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_blur_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_0590,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3700, 0x010b_0cf4, 0x010b_161c],
                    words: [0x010b_3c44, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3734, 0x010b_0cf4, 0x010b_161c],
                    words: [0x010b_3c4c, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_bright_pass_filter_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_0720,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3700, 0x010b_3de4],
                    words: [0x010b_3dcc, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3734, 0x010b_3de4],
                    words: [0x010b_3dd8, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_down_sample16_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_08a0,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3700, 0x010b_0cf4, 0x010b_3f84],
                    words: [0x010b_3f6c, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3734, 0x010b_0cf4, 0x010b_3f84],
                    words: [0x010b_3f78, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_down_sample9_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_0a30,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3700, 0x010b_0cf4, 0x0106_3ee8],
                    words: [0x010b_40fc, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3734, 0x010b_0cf4, 0x0106_3ee8],
                    words: [0x010b_4108, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_down_sample4_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_0bc0,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3700, 0x010b_0cf4, 0x0106_3f24],
                    words: [0x010b_428c, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3734, 0x010b_0cf4, 0x0106_3f24],
                    words: [0x010b_4298, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_light_adapt_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_0d50,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3700, 0x010b_4434, 0x0101_1584],
                    words: [0x010b_441c, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3734, 0x010b_4434, 0x0101_1584],
                    words: [0x010b_4428, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_lum_clamp_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_0ee0,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3700, 0x010b_45d0, 0x0101_1584],
                    words: [0x010b_45b4, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[0x010b_3734, 0x010b_45d0, 0x0101_1584],
                    words: [0x010b_45c0, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x3c],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_down_sample4_light_adapt_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_1070,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[
                        0x010b_3700,
                        0x010b_0cf4,
                        0x0106_3f24,
                        0x010b_4770,
                        0x0101_1584,
                    ],
                    words: [0x010b_4754, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[
                        0x010b_3734,
                        0x010b_0cf4,
                        0x0106_3f24,
                        0x010b_4770,
                        0x0101_1584,
                    ],
                    words: [0x010b_4760, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_down_sample4_lum_clamp_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_1210,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[
                        0x010b_3700,
                        0x010b_0cf4,
                        0x0106_3f24,
                        0x010b_45d0,
                        0x0101_1584,
                    ],
                    words: [0x010b_48f4, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[
                        0x010b_3734,
                        0x010b_0cf4,
                        0x0106_3f24,
                        0x010b_45d0,
                        0x0101_1584,
                    ],
                    words: [0x010b_4904, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_down_sample16_light_adapt_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_13b0,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[
                        0x010b_3700,
                        0x010b_0cf4,
                        0x010b_3f84,
                        0x010b_4770,
                        0x0101_1584,
                    ],
                    words: [0x010b_4a8c, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[
                        0x010b_3734,
                        0x010b_0cf4,
                        0x010b_3f84,
                        0x010b_4770,
                        0x0101_1584,
                    ],
                    words: [0x010b_4a9c, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }

    #[test]
    fn image_space_shader_hdr_down_sample16_lum_clamp_reload_shaders_loads_its_shaders() {
        check_reload(
            0x00b9_1550,
            &[
                Reload {
                    flag: 0,
                    vertex: &[0x010b_3768],
                    pixel: &[
                        0x010b_3700,
                        0x010b_0cf4,
                        0x010b_3f84,
                        0x010b_45d0,
                        0x0101_1584,
                    ],
                    words: [0x010b_4c24, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
                Reload {
                    flag: 1,
                    vertex: &[0x010b_3768],
                    pixel: &[
                        0x010b_3734,
                        0x010b_0cf4,
                        0x010b_3f84,
                        0x010b_45d0,
                        0x0101_1584,
                    ],
                    words: [0x010b_4c34, 0x010a_93ec, 0x010a_93c4],
                    memsets: &[0x44, 0x34],
                },
            ],
        );
    }
}

#[cfg(test)]
mod manager_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::{
        NEW_RENDER_STATES_00E7F5D0, SET_RENDER_STATE_00E7F430,
    };

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// A manager-sized block (0x320 bytes) filled with a marker word.
    fn manager(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x320);
        for offset in (0..0x320).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        this
    }

    fn set_f32(e: &mut Engine, at: u32, value: f32) {
        e.mem.set_u32(at, value.to_bits());
    }

    #[test]
    fn fn_00b8aea0_resets_the_blend_values() {
        let mut e = Engine::new();
        let this = manager(&mut e);
        e.call(0x00b8_aea0, &args![this]);
        for k in 0..15 {
            assert_eq!(e.mem.f32(this + 0xb4 + 4 * k), 1.0);
            assert_eq!(e.mem.u32(this + 0x14c + 4 * k), 0);
        }
        for k in 0..12 {
            assert_eq!(e.mem.f32(this + 0x118 + 4 * k), 1.0);
            assert_eq!(e.mem.u32(this + 0x1b0 + 4 * k), 0);
        }
        assert_eq!(e.mem.f32(this + 0xf8), 1.0);
        assert_eq!(e.mem.u32(this + 0x190), 0);
        assert_eq!(e.mem.u32(this + 0xb0), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x1e0), 0xdead_beef);
    }

    fn check_keep_larger(addr: u32, offset: u32) {
        let mut e = Engine::new();
        let this = manager(&mut e);
        set_f32(&mut e, this + offset, 2.0);
        e.call(addr, &args![this, bits(1.0)]);
        assert_eq!(e.mem.f32(this + offset), 2.0);
        e.call(addr, &args![this, bits(2.0)]);
        assert_eq!(e.mem.f32(this + offset), 2.0);
        e.call(addr, &args![this, bits(3.0)]);
        assert_eq!(e.mem.f32(this + offset), 3.0);
        // A NaN never compares above.
        e.call(addr, &args![this, 0x7fc0_0000u32]);
        assert_eq!(e.mem.f32(this + offset), 3.0);
    }

    fn check_add(addr: u32, offset: u32) {
        let mut e = Engine::new();
        let this = manager(&mut e);
        set_f32(&mut e, this + offset, 2.5);
        e.call(addr, &args![this, bits(1.25)]);
        assert_eq!(e.mem.f32(this + offset), 3.75);
    }

    #[test]
    fn fn_00b8af10_keeps_the_larger() {
        check_keep_larger(0x00b8_af10, 0x25c);
    }

    #[test]
    fn fn_00b8af40_adds() {
        check_add(0x00b8_af40, 0x260);
    }

    #[test]
    fn fn_00b8af60_keeps_the_larger() {
        check_keep_larger(0x00b8_af60, 0x2f0);
    }

    #[test]
    fn fn_00b8af90_adds() {
        check_add(0x00b8_af90, 0x2f4);
    }

    #[test]
    fn fn_00b8afb0_keeps_the_larger() {
        check_keep_larger(0x00b8_afb0, 0x264);
    }

    #[test]
    fn fn_00b8afe0_adds() {
        check_add(0x00b8_afe0, 0x268);
    }

    #[test]
    fn fn_00b8b000_takes_the_heavier_values() {
        let mut e = Engine::new();
        let this = manager(&mut e);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 0x11);
        e.mem.set_u32(pair + 4, 0x22);
        set_f32(&mut e, this + 0x26c, 1.0);
        let words = |w: f32| {
            args![
                this,
                bits(w),
                bits(5.0),
                bits(6.0),
                bits(7.0),
                bits(8.0),
                pair
            ]
        };
        e.call(0x00b8_b000, &words(0.5));
        assert_eq!(e.mem.f32(this + 0x26c), 1.0);
        assert_eq!(e.mem.u32(this + 0x270), 0xdead_beef);
        e.call(0x00b8_b000, &words(1.5));
        assert_eq!(e.mem.f32(this + 0x26c), 1.5);
        assert_eq!(e.mem.f32(this + 0x270), 5.0);
        assert_eq!(e.mem.f32(this + 0x27c), 8.0);
        assert_eq!(e.mem.u32(this + 0x294), 0x11);
        assert_eq!(e.mem.u32(this + 0x298), 0x22);
    }

    #[test]
    fn fn_00b8b0d0_takes_the_heavier_mode() {
        let mut e = Engine::new();
        let this = manager(&mut e);
        set_f32(&mut e, this + 0x2a4, 1.0);
        e.call(
            0x00b8_b0d0,
            &args![this, bits(0.5), 4u32, bits(5.0), bits(6.0), bits(7.0)],
        );
        assert_eq!(e.mem.u32(this + 0x2c4), 0xdead_beef);
        e.call(
            0x00b8_b0d0,
            &args![this, bits(2.0), 4u32, bits(5.0), bits(6.0), bits(7.0)],
        );
        assert_eq!(e.mem.f32(this + 0x2a4), 2.0);
        assert_eq!(e.mem.u8(this + 0x2cc), 1);
        assert_eq!(e.mem.u32(this + 0x2c4), 1);
        assert_eq!(e.mem.f32(this + 0x2a8), 6.0);
        assert_eq!(e.mem.f32(this + 0x2ac), 7.0);
        assert_eq!(e.mem.f32(this + 0x2b0), 5.0);
        e.call(
            0x00b8_b0d0,
            &args![this, bits(3.0), 2u32, bits(5.0), bits(6.0), bits(7.0)],
        );
        assert_eq!(e.mem.u8(this + 0x2cc), 0);
        assert_eq!(e.mem.u32(this + 0x2c4), 2);
    }

    #[test]
    fn fn_00b8b130_adds_the_blur_values() {
        let mut e = Engine::new();
        let this = manager(&mut e);
        for offset in [0x2b4u32, 0x2b8, 0x2bc, 0x2c0] {
            set_f32(&mut e, this + offset, 1.0);
        }
        e.call(
            0x00b8_b130,
            &args![this, bits(0.5), 9u32, bits(2.0), bits(3.0), bits(4.0)],
        );
        assert_eq!(e.mem.f32(this + 0x2b4), 1.5);
        assert_eq!(e.mem.u8(this + 0x2cd), 0);
        assert_eq!(e.mem.u32(this + 0x2c8), 9);
        assert_eq!(e.mem.f32(this + 0x2b8), 4.0);
        assert_eq!(e.mem.f32(this + 0x2bc), 5.0);
        assert_eq!(e.mem.f32(this + 0x2c0), 3.0);
    }

    #[test]
    fn get_depth_rendered_texture_borrows_when_empty() {
        for (held, borrows) in [(0u32, true), (0x77, false)] {
            let mut e = Engine::new();
            e.register_double(TEXTURE_GET_00BA3780, move |_, _| held.into_ret());
            e.register(TEXTURE_BORROW_RENDERED_00BA3840, |_, _| Ret::default());
            e.register(TEXTURE_GET_RENDERED_00BA3770, |_, _| 0x5150u32.into_ret());
            e.call_log = Some(vec![]);
            let this = manager(&mut e);
            assert_eq!(e.call(0x00b8_b190, &args![this]).u32(), 0x5150);
            let borrowed = calls(&e, TEXTURE_BORROW_RENDERED_00BA3840);
            if borrows {
                assert_eq!(borrowed, vec![vec![this + 0x204, 0x21]]);
            } else {
                assert!(borrowed.is_empty());
            }
            assert_eq!(
                calls(&e, TEXTURE_GET_RENDERED_00BA3770),
                vec![vec![this + 0x204]]
            );
        }
    }

    #[test]
    fn fn_00b8b200_stores_the_byte() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.call(0x00b8_b200, &args![0x4400u32, 0x1234_5601u32]);
        assert_eq!(e.global::<u8>(BYTE_SETTING_012023F0), 1);
    }

    #[test]
    fn fn_00b8b210_sets_the_mode_weights() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x58, 7);
        for (mode, first, second) in [
            (0u32, 1.0f32, 1.0f32),
            (1, 1.0, 0.0),
            (2, 0.0, 1.0),
            (3, 0.0, 0.0),
        ] {
            e.mem.set_u8(this + 5, 0);
            e.call(0x00b8_b210, &args![this, mode]);
            assert_eq!(e.mem.u8(this + 5), 1);
            assert_eq!(e.mem.u32(this + 0x58), mode);
            assert_eq!(e.mem.f32(this + 0x70), first);
            assert_eq!(e.mem.f32(this + 0x74), second);
        }
        // The same mode again: nothing.
        e.mem.set_u8(this + 5, 0);
        e.call(0x00b8_b210, &args![this, 3u32]);
        assert_eq!(e.mem.u8(this + 5), 0);
    }

    /// A manager with an effect block of `size` slots (16-bit size and count words set).
    fn with_effects(e: &mut Engine, size: u16) -> (u32, u32) {
        let this = manager(e);
        let block = e.mem.alloc(4 * 0x74);
        e.mem.set_u32(this + EFFECTS, block);
        e.mem.set_u16(this + EFFECTS_SIZE, size);
        e.mem.set_u16(this + EFFECTS_COUNT, 2);
        (this, block)
    }

    /// An effect object whose vtable slots +0, +0xc and +0x18 to +0x28 call the doubles
    /// 0x0f00_xx00 + slot.
    fn effect(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for slot in (0..0x40).step_by(4) {
            e.mem.set_u32(vtable + slot, 0x0f00_1000 + slot);
            e.register(0x0f00_1000 + slot, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0xa0);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn fn_00b8b2e0_destroys_every_effect() {
        let mut e = Engine::new();
        let (this, block) = with_effects(&mut e, 0x74);
        let first = effect(&mut e);
        let last = effect(&mut e);
        e.mem.set_u32(block + 4 * 3, first);
        e.mem.set_u32(block + 4 * 0x73, last);
        e.register(ARRAY_SET_00BA3AA0, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b8_b2e0, &args![this]);
        assert_eq!(calls(&e, 0x0f00_100c), vec![vec![first], vec![last]]);
        assert_eq!(calls(&e, 0x0f00_1000), vec![vec![first, 1], vec![last, 1]]);
        let set: Vec<(u32, u32)> = calls(&e, ARRAY_SET_00BA3AA0)
            .iter()
            .map(|c| (c[0], c[1]))
            .collect();
        assert_eq!(set, vec![(this + 4, 3), (this + 4, 0x73)]);
        assert_eq!(e.mem.u32(block + 4 * 3), 0);
        assert_eq!(e.mem.u32(block + 4 * 0x73), 0);
        assert_eq!(e.mem.u16(this + EFFECTS_SIZE), 0);
        assert_eq!(e.mem.u16(this + EFFECTS_COUNT), 0);
    }

    #[test]
    fn fn_00b8b390_asks_the_effect() {
        let mut e = Engine::new();
        let (this, block) = with_effects(&mut e, 0x74);
        let object = effect(&mut e);
        e.register(0x0f00_1018, |_, _| 0x0101u32.into_ret());
        e.mem.set_u32(block + 8, object);
        assert_eq!(e.call(0x00b8_b390, &args![this, 2u32]).u32() & 0xff, 1);
        assert_eq!(e.call(0x00b8_b390, &args![this, 3u32]).u32() & 0xff, 0);
    }

    #[test]
    fn fn_00b8b3f0_counts_the_active_effects() {
        let mut e = Engine::new();
        e.map(0x011a_d000, 0x1000);
        let (this, block) = with_effects(&mut e, 0x74);
        let active = effect(&mut e);
        e.mem.set_u8(active + 0x70, 1);
        let idle = effect(&mut e);
        e.register(0x0f00_1018, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x70)).into_ret()
        });
        e.mem.set_u8(idle + 0x70, 0);
        e.mem.set_u32(block + 4, active);
        e.mem.set_u32(block + 4 * 5, active);
        e.mem.set_u32(block + 4 * 9, idle);
        // Beyond index 0xc: never asked.
        e.mem.set_u32(block + 4 * 0xd, active);
        e.set_global(COUNT_ACTIVE_011AD884, 0u8);
        e.call(0x00b8_b3f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1ec), u32::MAX);
        assert_eq!(e.mem.u32(this + 0x1e8), 0);
        e.set_global(COUNT_ACTIVE_011AD884, 1u8);
        e.call(0x00b8_b3f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1e8), 2);
        assert_eq!(e.mem.u32(this + 0x1ec), 5);
    }

    #[test]
    fn fn_00b8b440_hands_out_the_parameters() {
        for flag in [0u32, 1] {
            let mut e = Engine::new();
            e.map(0x011a_d000, 0x1000);
            e.map(0x011f_9000, 0x1000);
            let (this, block) = with_effects(&mut e, 0x74);
            let objects: Vec<u32> = (0..4).map(|_| effect(&mut e)).collect();
            e.mem.set_u32(block, objects[0]);
            e.mem.set_u32(block + 4, objects[1]);
            e.mem.set_u32(block + 0x20, objects[2]);
            e.mem.set_u32(block + 0xc, objects[3]);
            e.mem.set_u32(this + 0xa8, 0x66);
            set_f32(&mut e, this + 0x34, 0.25);
            for (offset, value) in [(0x40u32, 1.0f32), (0x48, 2.0), (0x44, 3.0), (0x1c, 4.0)] {
                set_f32(&mut e, this + offset, value);
            }
            e.set_global(VALUE_011AD87C, 9u32);
            e.call_log = Some(vec![]);
            e.call(0x00b8_b440, &args![this, flag]);
            assert_eq!(
                calls(&e, 0x0f00_1020),
                vec![
                    vec![objects[0], this + 0x50],
                    vec![objects[1], this + 0x14],
                    vec![objects[2], this + 0x5c]
                ]
            );
            assert_eq!(
                calls(&e, 0x0f00_1024),
                vec![vec![objects[3], this + 0x78, 0x66]]
            );
            assert_eq!(calls(&e, 0x0f00_1028), vec![vec![objects[3], this + 0x68]]);
            if flag == 0 {
                assert_eq!(e.global::<f32>(VALUE_011AD87C), 0.25);
                assert_eq!(e.global::<f32>(VALUE_011AD880), 1.0);
            } else {
                assert_eq!(e.global::<u32>(VALUE_011AD87C), 9);
                assert_eq!(e.global::<f32>(VALUE_011AD880), 0.25);
            }
            assert_eq!(e.global::<f32>(0x011f_9190), 1.0);
            assert_eq!(e.global::<f32>(0x011f_918c), 2.0);
            assert_eq!(e.global::<f32>(0x011f_9188), 3.0);
            assert_eq!(e.global::<f32>(0x011f_9184), 4.0);
        }
    }

    #[test]
    fn fn_00b8b500_copies_the_first_set_block() {
        for (late, early, source) in [
            (0x10u32, 0x20u32, 0x10u32),
            (0, 0x20, 0x20),
            (0, 0, DEFAULTS_011FF940),
        ] {
            let mut e = Engine::new();
            e.map(0x0120_0000, 0x3000);
            e.register(MEMCPY_00EC44D0, |_, _| Ret::default());
            let this = manager(&mut e);
            e.mem.set_u32(this + 0xb0, late);
            e.mem.set_u32(this + 0xac, early);
            e.set_global(VALUE_01202400, 5u32);
            e.call_log = Some(vec![]);
            e.call(0x00b8_b500, &args![this]);
            assert_eq!(
                calls(&e, MEMCPY_00EC44D0),
                vec![vec![this + 0x14, source, 0x98]]
            );
            assert_eq!(e.global::<u32>(VALUE_01202400), 0);
            assert_eq!(e.global::<u32>(VALUE_012003D0), 0);
        }
    }

    fn check_weighted(addr: u32, weight: u32, sum: u32) {
        let mut e = Engine::new();
        let this = manager(&mut e);
        set_f32(&mut e, this + weight, 0.25);
        for k in 0..4 {
            set_f32(&mut e, this + sum + 4 * k, 1.0);
        }
        let color = e.mem.alloc(0x10);
        for (k, value) in [2.0f32, 4.0, 6.0, 0.5].into_iter().enumerate() {
            set_f32(&mut e, color + 4 * k as u32, value);
        }
        e.call(addr, &args![this, color]);
        assert_eq!(e.mem.f32(this + weight), 0.5);
        assert_eq!(e.mem.f32(color), 1.0);
        assert_eq!(e.mem.f32(color + 4), 2.0);
        assert_eq!(e.mem.f32(color + 8), 3.0);
        assert_eq!(e.mem.f32(this + sum), 2.0);
        assert_eq!(e.mem.f32(this + sum + 4), 3.0);
        assert_eq!(e.mem.f32(this + sum + 8), 4.0);
        assert_eq!(e.mem.f32(this + sum + 0xc), 1.5);
        // A lighter alpha leaves the weight.
        set_f32(&mut e, color + 0xc, 0.125);
        e.call(addr, &args![this, color]);
        assert_eq!(e.mem.f32(this + weight), 0.5);
    }

    #[test]
    fn fn_00b8b540_adds_the_weighted_colour() {
        check_weighted(0x00b8_b540, 0x234, 0x214);
    }

    #[test]
    fn fn_00b8b610_adds_the_weighted_colour() {
        check_weighted(0x00b8_b610, 0x258, 0x238);
    }

    fn check_add_four(addr: u32, sum: u32) {
        let mut e = Engine::new();
        let this = manager(&mut e);
        let values = e.mem.alloc(0x10);
        for k in 0..4 {
            set_f32(&mut e, this + sum + 4 * k, 1.0);
            set_f32(&mut e, values + 4 * k, (k + 1) as f32);
        }
        e.call(addr, &args![this, values]);
        for k in 0..4 {
            assert_eq!(e.mem.f32(this + sum + 4 * k), (k + 2) as f32);
        }
    }

    #[test]
    fn fn_00b8b5c0_adds_four() {
        check_add_four(0x00b8_b5c0, 0x224);
    }

    #[test]
    fn fn_00b8b690_adds_four() {
        check_add_four(0x00b8_b690, 0x248);
    }

    #[test]
    fn fn_00b8b6e0_adds_seven() {
        let mut e = Engine::new();
        let this = manager(&mut e);
        for offset in [0x280u32, 0x284, 0x288, 0x28c, 0x290, 0x29c, 0x2a0] {
            set_f32(&mut e, this + offset, 1.0);
        }
        let pair = e.mem.alloc(8);
        set_f32(&mut e, pair, 6.0);
        set_f32(&mut e, pair + 4, 7.0);
        e.call(
            0x00b8_b6e0,
            &args![
                this,
                bits(1.0),
                bits(2.0),
                bits(3.0),
                bits(4.0),
                bits(5.0),
                pair
            ],
        );
        for (offset, value) in [
            (0x280u32, 2.0f32),
            (0x284, 3.0),
            (0x288, 4.0),
            (0x28c, 5.0),
            (0x290, 6.0),
            (0x29c, 7.0),
            (0x2a0, 8.0),
        ] {
            assert_eq!(e.mem.f32(this + offset), value, "+{offset:#x}");
        }
    }

    #[test]
    fn fn_00b8b9a0_resets_the_frame() {
        let mut e = Engine::new();
        e.register(MEMSET_00EC61C0, |_, _| Ret::default());
        let this = manager(&mut e);
        set_f32(&mut e, this + 0x94, 0.5);
        set_f32(&mut e, this + 0x88, 2.0);
        set_f32(&mut e, this + 0x8c, 4.0);
        set_f32(&mut e, this + 0x90, 6.0);
        e.call_log = Some(vec![]);
        e.call(0x00b8_b9a0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x234), 0.5);
        assert_eq!(e.mem.f32(this + 0x214), 1.0);
        assert_eq!(e.mem.f32(this + 0x218), 2.0);
        assert_eq!(e.mem.f32(this + 0x21c), 3.0);
        assert_eq!(e.mem.f32(this + 0x220), 0.5);
        for offset in (0x224..0x2c4).step_by(4).filter(|o| *o != 0x234) {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x2c4), 3);
        assert_eq!(e.mem.u32(this + 0x2c8), 3);
        assert_eq!(e.mem.u16(this + 0x2cc), 0);
        for offset in [0x2d0u32, 0x2f0, 0x2f4] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(
            calls(&e, MEMSET_00EC61C0),
            vec![vec![this + 0xb4, 0, 0x130]]
        );
    }

    /// A shader with a pass whose render-state block is made on first use, and the doubles.
    fn shader_with_pass(e: &mut Engine) -> u32 {
        e.register(NEW_RENDER_STATES_00E7F5D0, |_, _| 0x4400u32.into_ret());
        e.register(SET_RENDER_STATE_00E7F430, |_, _| Ret::default());
        let this = e.mem.alloc(0xd0);
        let pass = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x5c, pass);
        this
    }

    fn states(e: &Engine) -> Vec<(u32, u32)> {
        calls(e, SET_RENDER_STATE_00E7F430)
            .iter()
            .map(|c| {
                assert_eq!((c[0], c[3]), (0x4400, 1));
                (c[1], c[2])
            })
            .collect()
    }

    fn check_states(addr: u32, expected: &[(u32, u32)]) {
        let mut e = Engine::new();
        let this = shader_with_pass(&mut e);
        e.call_log = Some(vec![]);
        e.call(addr, &args![this]);
        assert_eq!(states(&e), expected);
        assert_eq!(calls(&e, NEW_RENDER_STATES_00E7F5D0).len(), 1);
    }

    #[test]
    fn fn_00b8bfe0_sets_four_render_states() {
        check_states(
            0x00b8_bfe0,
            &[(0x34, 1), (0x38, 7), (0x39, 0x80), (0x37, 1)],
        );
    }

    #[test]
    fn fn_00b8f160_sets_two_render_states() {
        check_states(0x00b8_f160, &[(0xa8, 0), (0xbe, 0xf)]);
    }

    #[test]
    fn fn_00b8f2f0_sets_three_render_states() {
        let mut e = Engine::new();
        e.map(0x011a_e000, 0x1000);
        e.set_global(STATE_VALUE_011AE680, 0x55u32);
        let this = shader_with_pass(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b8_f2f0, &args![this]);
        assert_eq!(states(&e), vec![(0x34, 1), (0x38, 4), (0x39, 0x55)]);
    }

    #[test]
    fn fn_00b8f4b0_sets_three_render_states() {
        check_states(0x00b8_f4b0, &[(0xe, 0), (0x34, 1), (0x38, 3)]);
    }

    /// A reference-counted object whose vtable slot +4 is the double `0x0f00_2004`, with
    /// `count` references.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0f00_2004);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn reference_doubles(e: &mut Engine) {
        e.register(0x0f00_2004, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let left = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], left);
            left.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
    }

    #[test]
    fn fn_00b8c690_releases_the_four_pointers() {
        let mut e = Engine::new();
        reference_doubles(&mut e);
        let this = manager(&mut e);
        let last = counted(&mut e, 1);
        let shared = counted(&mut e, 2);
        e.mem.set_u32(this + 0x1f0, last);
        e.mem.set_u32(this + 0x1f8, shared);
        e.mem.set_u32(this + 0x1f4, 0);
        e.mem.set_u32(this + 0x200, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b8_c690, &args![this]);
        assert_eq!(calls(&e, 0x0f00_2004), vec![vec![last]]);
        assert_eq!(e.mem.u32(shared + 4), 1);
        for offset in [0x1f0u32, 0x1f8, 0x1f4, 0x200] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).len(), 2);
    }

    #[test]
    fn render_effect_ov2_runs_the_effect_on_two_textures() {
        for (effect_set, flag) in [(true, 0u8), (true, 1), (false, 0)] {
            let mut e = Engine::new();
            let tls = e.tls();
            e.mem.set_u32(tls + TLS_CONTEXT, 9);
            e.register(TEXTURE_SET_00BA37A0, |e, a| {
                let tls = e.tls();
                assert_eq!(e.mem.u32(tls + TLS_CONTEXT), CONTEXT_IMAGE_SPACE);
                e.mem.set_u32(a[0] + 4, a[1]);
                Ret::default()
            });
            e.register(EFFECT_SET_TEXTURE_00BA3CB0, |_, _| Ret::default());
            e.register(IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00, |_, _| Ret::default());
            let object = effect(&mut e);
            let this = manager(&mut e);
            e.mem.set_u8(this + 0x1fc, flag);
            e.mem.set_u32(this + 0x1f0, 0x1f0);
            e.mem.set_u32(this + 0x1f8, 0x1f8);
            let given = if effect_set { object } else { 0 };
            e.call_log = Some(vec![]);
            e.call(
                0x00b8_c830,
                &args![this, given, 0x22u32, 0x33u32, 0x44u32, 0x55u32, 0x66u32],
            );
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
            let set = calls(&e, TEXTURE_SET_00BA37A0);
            assert_eq!(set.len(), 2);
            let (first, second) = (set[0][0], set[1][0]);
            assert_eq!((set[0][1], set[1][1]), (0x33, 0x44));
            assert_eq!(
                calls(&e, IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00),
                vec![vec![second], vec![first]]
            );
            if effect_set {
                assert_eq!(
                    calls(&e, EFFECT_SET_TEXTURE_00BA3CB0),
                    vec![
                        vec![object, 0, second, 0],
                        vec![object, 1, first, 0],
                        vec![object, 0, 0, 0],
                        vec![object, 1, 0, 0]
                    ]
                );
                let target = if flag != 0 { 0x1f8 } else { 0x1f0 };
                assert_eq!(
                    calls(&e, 0x0f00_1004),
                    vec![vec![object, target, 0x22, 0x55, 0x66]]
                );
            } else {
                assert!(calls(&e, EFFECT_SET_TEXTURE_00BA3CB0).is_empty());
            }
        }
    }

    #[test]
    fn fn_00b8cad0_stores_the_word() {
        let mut e = Engine::new();
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        let this = manager(&mut e);
        e.call(0x00b8_cad0, &args![this, 0x77u32]);
        assert_eq!(e.mem.u32(this + 0xac), 0x77);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
    }

    #[test]
    fn fn_00b8cb30_blends_the_picked_value() {
        for (kind, offset) in [(0u32, 0x50u32), (1, 0x14), (3, 0x78)] {
            let mut e = Engine::new();
            let this = manager(&mut e);
            let field = this + offset + 4 * 2;
            set_f32(&mut e, field, 2.0);
            let values = e.mem.alloc(8);
            set_f32(&mut e, values, 3.0);
            set_f32(&mut e, values + 4, 4.0);
            let tls = e.tls();
            let before = e.mem.u32(tls + TLS_CONTEXT);
            e.call(0x00b8_cb30, &args![this, kind, 2u32, values, bits(0.5)]);
            // r = 2 * 3 + 4 = 10; 2 - (2 - 10) * 0.5 = 6.
            assert_eq!(e.mem.f32(field), 6.0, "kind {kind}");
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), before);
        }
    }

    #[test]
    fn fn_00b8cc20_adds_scaled_values_to_both_blocks() {
        for (kind, offset) in [
            (0u32, Some(0x3cu32)),
            (1, Some(0)),
            (3, Some(0x64)),
            (2, None),
        ] {
            let mut e = Engine::new();
            let this = manager(&mut e);
            let values = e.mem.alloc(8);
            set_f32(&mut e, values, 2.0);
            set_f32(&mut e, values + 4, 4.0);
            let start = this + 0xb4 + 4;
            for at in [0x3cu32, 0, 0x64] {
                set_f32(&mut e, start + at, 1.0);
                set_f32(&mut e, start + 0x98 + at, 1.0);
            }
            e.call(0x00b8_cc20, &args![this, kind, 1u32, values, bits(0.5)]);
            for at in [0x3cu32, 0, 0x64] {
                let (a, b) = if offset == Some(at) {
                    (2.0, 3.0)
                } else {
                    (1.0, 1.0)
                };
                assert_eq!(e.mem.f32(start + at), a, "kind {kind}");
                assert_eq!(e.mem.f32(start + 0x98 + at), b, "kind {kind}");
            }
        }
    }

    #[test]
    fn fn_00b8ccb0_settles_the_frame() {
        let mut e = Engine::new();
        let this = manager(&mut e);
        for offset in (0x14..0xa8).step_by(4) {
            set_f32(&mut e, this + offset, 1.0);
            set_f32(&mut e, this + offset + 0xa0, 2.0);
            set_f32(&mut e, this + offset + 0x138, 3.0);
        }
        // The colour sums and weights 00b8b540/00b8b610 use.
        for offset in [0x214u32, 0x218, 0x21c, 0x220, 0x238, 0x23c, 0x240, 0x244] {
            set_f32(&mut e, this + offset, 0.0);
        }
        for offset in [0x224u32, 0x228, 0x22c, 0x248, 0x24c, 0x250] {
            set_f32(&mut e, this + offset, 1.0);
        }
        set_f32(&mut e, this + 0x230, 0.5);
        set_f32(&mut e, this + 0x254, 0.5);
        set_f32(&mut e, this + 0x234, 0.0);
        set_f32(&mut e, this + 0x258, 1.0);
        for (field, source) in [
            (0x25cu32, 0x260u32),
            (0x264, 0x268),
            (0x26c, 0x280),
            (0x2a4, 0x2b4),
            (0x2f0, 0x2f4),
        ] {
            set_f32(&mut e, this + field, 1.0);
            set_f32(&mut e, this + source, 2.0);
        }
        set_f32(&mut e, this + 0x264, 3.0);
        e.mem.set_u32(this + 0x2c8, 5);
        e.mem.set_u32(this + 0x29c, 0x11);
        e.mem.set_u32(this + 0x2a0, 0x22);
        e.call(0x00b8_ccb0, &args![this]);
        // 1 - (1 - (1 * 2 + 3)) = 5.
        assert_eq!(e.mem.f32(this + 0x14), 5.0);
        assert_eq!(e.mem.f32(this + 0xa4), 5.0);
        assert_eq!(e.mem.f32(this + 0x234), 0.5);
        assert_eq!(e.mem.f32(this + 0x214), 0.5);
        assert_eq!(e.mem.f32(this + 0x258), 1.0);
        assert_eq!(e.mem.f32(this + 0x25c), 2.0);
        assert_eq!(e.mem.f32(this + 0x264), 3.0);
        assert_eq!(e.mem.f32(this + 0x26c), 2.0);
        assert_eq!(e.mem.u32(this + 0x294), 0x11);
        assert_eq!(e.mem.u32(this + 0x298), 0x22);
        assert_eq!(e.mem.f32(this + 0x2a4), 2.0);
        assert_eq!(e.mem.u8(this + 0x2cc), 1);
        assert_eq!(e.mem.u32(this + 0x2c4), 2);
        assert_eq!(e.mem.u32(this + 0x2b0), 0);
        assert_eq!(e.mem.f32(this + 0x2f0), 2.0);
    }

    #[test]
    fn fn_00b8d020_hands_the_values_to_the_effects() {
        for flag in [0u8, 1] {
            let mut e = Engine::new();
            e.map(0x0120_0000, 0x3000);
            e.register(SET_UP_00B8BC80, |_, _| Ret::default());
            e.register(SET_UP_00B8B270, |_, _| Ret::default());
            let (this, block) = with_effects(&mut e, 0x74);
            let effects: Vec<u32> = (0..13).map(|_| e.mem.alloc(0xa0)).collect();
            for (index, object) in effects.iter().enumerate() {
                e.mem.set_u32(block + 4 * index as u32, *object);
            }
            for offset in [0x214u32, 0x218, 0x21c, 0x238, 0x23c, 0x240] {
                set_f32(&mut e, this + offset, 2.0);
            }
            set_f32(&mut e, this + 0x220, 2.0);
            set_f32(&mut e, this + 0x244, 0.0);
            set_f32(&mut e, this + 0x234, 0.75);
            set_f32(&mut e, this + 0x258, 0.25);
            for offset in (0x25cu32..0x2f8).step_by(4) {
                set_f32(&mut e, this + offset, offset as f32);
            }
            e.mem.set_u32(this + 0x2c4, 3);
            e.mem.set_u8(this + 0x2cc, flag);
            e.mem.set_u32(this + 0x2ec, 7);
            let five = effects[5];
            set_f32(&mut e, five + 0x9c, 1000.0);
            e.call_log = Some(vec![]);
            e.call(0x00b8_d020, &args![this]);
            // The sums divided by 2.0 at +0x220, the second weight 0.0 so no division.
            assert_eq!(e.mem.f32(this + 0x214), 1.0);
            assert_eq!(e.mem.f32(this + 0x238), 2.0);
            assert_eq!(e.mem.f32(this + 0x88), 1.0);
            assert_eq!(e.mem.f32(this + 0x94), 0.75);
            assert_eq!(e.mem.f32(this + 0x220), 0.75);
            assert_eq!(e.mem.f32(this + 0xa4), 0.25);
            assert_eq!(e.global::<f32>(VALUE_01202400), 612.0);
            assert_eq!(e.global::<f32>(VALUE_012003D0), 604.0);
            let six = effects[6];
            assert_eq!(e.mem.f32(six + 0x58), 620.0);
            assert_eq!(e.mem.f32(six + 0x68), 636.0);
            let four = effects[4];
            assert_eq!(e.mem.f32(four + 0x70), 676.0);
            assert_eq!(e.mem.f32(four + 0x78), 688.0);
            assert_eq!(e.mem.u32(four + 0x74), 3);
            assert_eq!(e.mem.u8(four + 5), 1);
            let ran_alternative = !calls(&e, SET_UP_00B8BC80).is_empty();
            assert_eq!(ran_alternative, flag != 0);
            assert_eq!(
                calls(&e, SET_UP_00B8B270).len(),
                if flag != 0 { 0 } else { 4 }
            );
            // y = 0x2dc, x = 1000 (larger): m = x.
            let y = 732.0;
            let z = 736.0;
            assert_eq!(e.mem.f32(five + 0x84), y);
            assert_eq!(e.mem.f32(five + 0x9c), z);
            assert_eq!(e.mem.f32(five + 0x7c), y - z);
            assert_eq!(e.mem.f32(five + 0x80), y + z);
            assert_eq!(e.mem.u32(five + 0x58), 7);
            assert_eq!(e.mem.f32(five + 0x8c), 720.0);
            assert_eq!(e.mem.f32(five + 0x94), 728.0);
            assert_eq!(e.mem.f32(effects[12] + 0x58), 752.0);
        }
    }

    /// The doubles of the pass loader's callees; the stage double makes stage objects with a
    /// count of 1.
    fn loader(e: &mut Engine, count: u16) -> u32 {
        reference_doubles(e);
        e.register(NEW_PASS_00E804D0, |e, a| {
            let pass = e.mem.alloc(0x60);
            e.mem.set_u32(pass + 0x14, 0x14);
            e.mem.set_u32(a[0], pass);
            Ret::default()
        });
        e.register(CREATE_NEW_STAGE_00E7EDF0, |e, a| {
            let stage = e.mem.alloc(0x70);
            e.mem.set_u32(stage + 0x60, 1);
            e.mem.set_u32(a[0], stage);
            Ret::default()
        });
        e.register(SETUP_DEFAULT_STAGE_00BE0CF0, |_, _| Ret::default());
        e.register(PASS_SET_STAGE_00E802B0, |_, _| Ret::default());
        e.register(0x00e7_ecd0, |_, _| Ret::default());
        e.register(NEW_RENDER_STATES_00E7F5D0, |_, _| 0x4400u32.into_ret());
        e.register(SET_RENDER_STATE_00E7F430, |_, _| Ret::default());
        e.register(0x0f00_3150, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x160);
        e.mem.set_u32(vtable + 0x150, 0x0f00_3150);
        let this = e.mem.alloc(0xd0);
        e.mem.set_u32(this, vtable);
        e.mem.set_u16(this + 0x9c, count);
        let vertex = counted(e, 1);
        let pixel = counted(e, 1);
        e.mem.set_u32(this + 0xc4, vertex);
        e.mem.set_u32(this + 0xc8, pixel);
        this
    }

    fn check_loader(addr: u32, setups: &[u32], keeps: bool) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.set_global(LAST_PASS_0126F74C, 0x99u32);
        let this = loader(&mut e, setups.len() as u16);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(addr, &args![this]).u32() & 0xff, 1);
        let pass = e.mem.u32(this + 0x5c);
        let setup: Vec<(u32, u32)> = calls(&e, SETUP_DEFAULT_STAGE_00BE0CF0)
            .iter()
            .map(|c| (c[1], c[2]))
            .collect();
        let expected: Vec<(u32, u32)> = setups
            .iter()
            .enumerate()
            .map(|(i, s)| (i as u32, *s))
            .collect();
        assert_eq!(setup, expected);
        assert_eq!(calls(&e, PASS_SET_STAGE_00E802B0).len(), setups.len());
        assert_eq!(
            states(&e),
            vec![(7, 0), (0xe, 0), (0x1b, 0), (0xf, 0), (0xa8, 0xf)]
        );
        assert_eq!(calls(&e, 0x0f00_3150), vec![vec![this]]);
        assert_eq!(e.mem.u32(pass + 0x5c), e.mem.u32(this + 0xc4));
        assert_eq!(e.mem.u32(pass + 0x44), e.mem.u32(this + 0xc8));
        assert_eq!(e.mem.u32(e.mem.u32(this + 0xc4) + 4), 2);
        let kept = e.global::<u32>(LAST_PASS_0126F74C);
        assert_eq!(kept, if keeps { pass } else { 0x99 });
        // Only the last stage is released (made with one reference: deleted).
        let released = calls(&e, 0x00e7_ecd0).len();
        assert_eq!(released, usize::from(!setups.is_empty()));
    }

    #[test]
    fn fn_00b91f90_loads_stages_and_keeps_the_pass() {
        check_loader(0x00b9_1f90, &[1, 1, 1], true);
        check_loader(0x00b9_1f90, &[], true);
    }

    #[test]
    fn fn_00b93420_loads_stages() {
        check_loader(0x00b9_3420, &[1, 1], false);
    }

    #[test]
    fn fn_00b93750_sets_up_the_last_stage_apart() {
        check_loader(0x00b9_3750, &[3, 3, 1], false);
        check_loader(0x00b9_3750, &[1], false);
    }

    #[test]
    fn fn_00b97550_renders_the_indexed_effect() {
        let mut e = Engine::new();
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        e.register(TEXTURE_SET_00BA37A0, |_, _| Ret::default());
        e.register(EFFECT_SET_TEXTURE_00BA3CB0, |_, _| Ret::default());
        e.register(IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00, |_, _| Ret::default());
        let (this, block) = with_effects(&mut e, 0x74);
        let object = effect(&mut e);
        e.mem.set_u32(block + 4 * 7, object);
        e.mem.set_u8(this + 0x1fc, 0);
        e.mem.set_u32(this + 0x1f0, 0x1f0);
        e.call_log = Some(vec![]);
        e.call(
            0x00b9_7550,
            &args![this, 7u32, 0x22u32, 0x33u32, 0x44u32, 0x55u32, 0x66u32],
        );
        assert_eq!(
            calls(&e, 0x0f00_1004),
            vec![vec![object, 0x1f0, 0x22, 0x55, 0x66]]
        );
        let set = calls(&e, TEXTURE_SET_00BA37A0);
        assert_eq!((set[0][1], set[1][1]), (0x33, 0x44));
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
    }

    #[test]
    fn fn_00b97690_tears_the_manager_down() {
        for initialized in [1u8, 0] {
            let mut e = Engine::new();
            e.map(0x011f_9000, 0x1000);
            reference_doubles(&mut e);
            e.set_global(OBJECT_011F91A8, 0x4100u32);
            for addr in [
                ARRAY_SET_00BA3AA0,
                SHADER_PARAM_00B8A7E0,
                RETURN_RENDERED_TEXTURE_00B6DA10,
                IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00,
                NI_FREE_00AA10F0,
            ] {
                e.register(addr, |_, _| Ret::default());
            }
            let (this, block) = with_effects(&mut e, 0);
            e.mem.set_u8(this + INITIALIZED, initialized);
            e.mem.set_u32(this + 0x318, 0x318);
            let held = counted(&mut e, 1);
            for offset in [0x1f0u32, 0x1f4, 0x1f8, 0x200] {
                e.mem.set_u32(this + offset, 0);
            }
            e.mem.set_u32(this + 0x1f4, held);
            e.call_log = Some(vec![]);
            e.call(0x00b9_7690, &args![this]);
            if initialized != 0 {
                assert_eq!(
                    calls(&e, SHADER_PARAM_00B8A7E0),
                    vec![vec![CONSTANT_MAP_01202410]]
                );
                assert_eq!(
                    calls(&e, RETURN_RENDERED_TEXTURE_00B6DA10),
                    vec![vec![0x4100, 0x318]]
                );
                assert_eq!(e.mem.u8(this + INITIALIZED), 0);
                // 00b8c690 cleared the pointers before the destructor's own releases.
                assert_eq!(calls(&e, 0x0f00_2004), vec![vec![held]]);
            } else {
                assert!(calls(&e, SHADER_PARAM_00B8A7E0).is_empty());
                assert_eq!(calls(&e, 0x0f00_2004), vec![vec![held]]);
                assert_eq!(e.mem.u32(this + 0x1f4), held);
            }
            assert_eq!(
                calls(&e, IMAGE_SPACE_TEXTURE_DESTRUCT_00BA3A00),
                vec![vec![this + 0x204]]
            );
            assert_eq!(e.mem.u32(this + 4), 0x010b_9d24);
            assert_eq!(calls(&e, NI_FREE_00AA10F0), vec![vec![block]]);
        }
    }
}

#[cfg(test)]
mod n012_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0106_d000, 0x1000);
        e.map(0x0109_b000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// A camera at (1, 2, 3) and a point at (4, 6, 15): distance 13.
    fn camera_and_point(e: &mut Engine) -> u32 {
        let camera = e.mem.alloc(0x100);
        for (k, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(camera + 0x8c + 4 * k as u32, *v);
        }
        e.mem.set_u32(CAMERA_011F917C, camera);
        e.register(CRT_SQRT, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            v.sqrt().into_ret()
        });
        let point = e.mem.alloc(0xc);
        for (k, v) in [4.0f32, 6.0, 15.0].iter().enumerate() {
            e.mem.set_f32(point + 4 * k as u32, *v);
        }
        point
    }

    #[test]
    fn endian_swaps_the_listed_words() {
        let mut e = rig();
        let this = e.mem.alloc(0x98);
        for k in 0..0x26u32 {
            e.mem.set_u32(this + 4 * k, 0x0102_0300 + k);
        }
        e.call(0x00b8_ad00, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0003_0201);
        assert_eq!(e.mem.u32(this + 0x38), 0x0e03_0201);
        assert_eq!(e.mem.u32(this + 0x3c), 0x0f03_0201);
        assert_eq!(e.mem.u32(this + 0x50), 0x1403_0201);
        assert_eq!(e.mem.u32(this + 0x54), 0x0102_0315);
        assert_eq!(e.mem.u32(this + 0x60), 0x0102_0318);
        assert_eq!(e.mem.u32(this + 0x64), 0x1903_0201);
        assert_eq!(e.mem.u32(this + 0x90), 0x2403_0201);
        assert_eq!(e.mem.u32(this + 0x94), 0x2503_0201);
    }

    #[test]
    fn fn_00b8b060_takes_only_a_stronger_request() {
        let mut e = rig();
        let this = e.mem.alloc(0x2f0);
        e.mem.set_f32(this + 0x2d0, 0.5);
        let point = e.mem.alloc(8);
        e.mem.set_u32(point, 0x11);
        e.mem.set_u32(point + 4, 0x22);
        let f = |v: f32| v.to_bits();
        e.call(
            0x00b8_b060,
            &args![this, f(0.25), 7u32, f(1.0), f(2.0), f(3.0), f(4.0), point],
        );
        assert_eq!(e.mem.f32(this + 0x2d0), 0.5);
        assert_eq!(e.mem.u32(this + 0x2ec), 0);
        e.call(
            0x00b8_b060,
            &args![this, f(0.75), 7u32, f(1.0), f(2.0), f(3.0), f(4.0), point],
        );
        assert_eq!(e.mem.f32(this + 0x2d0), 0.75);
        assert_eq!(
            (e.mem.f32(this + 0x2d4), e.mem.f32(this + 0x2d8)),
            (3.0, 4.0)
        );
        assert_eq!(
            (e.mem.f32(this + 0x2dc), e.mem.f32(this + 0x2e0)),
            (1.0, 2.0)
        );
        assert_eq!(
            (
                e.mem.u32(this + 0x2e4),
                e.mem.u32(this + 0x2e8),
                e.mem.u32(this + 0x2ec)
            ),
            (0x11, 0x22, 7)
        );
        e.call(
            0x00b8_b060,
            &args![
                this,
                0x7fc0_0000u32,
                9u32,
                f(1.0),
                f(2.0),
                f(3.0),
                f(4.0),
                point
            ],
        );
        assert_eq!(e.mem.u32(this + 0x2ec), 7);
    }

    #[test]
    fn small_globals_and_projection_wrappers() {
        let mut e = rig();
        e.register(PROJECT_00B59D30, |e, a| {
            assert_eq!(e.mem.u32(a[2]), a[1]);
            Ret::default()
        });
        e.call(0x00b8_b1c0, &args![0u32, 0x100u32, 0x200u32]);
        assert_eq!(calls(&e, PROJECT_00B59D30)[0][..2], [0x100, 0x200]);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 5);
        e.mem.set_u32(pair + 4, 6);
        e.call(0x00b8_b1e0, &args![0u32, pair]);
        assert_eq!(
            (e.mem.u32(PAIR_012023F4), e.mem.u32(PAIR_012023F4 + 4)),
            (5, 6)
        );
        assert_eq!(e.call(0x00b8_bd90, &args![]).u32(), RTTI_0126F554);
    }

    #[test]
    fn fn_00b8b3b0_calls_each_effect_shader() {
        let mut e = rig();
        let mut slots = [0u32; 0x49];
        slots[0x120 / 4] = 0x0f00_0120;
        e.put_vtable(0x0f00_1000, &slots);
        e.register(0x0f00_0120, |_, _| Ret::default());
        let shader = e.mem.alloc(0x80);
        e.mem.set_u32(shader, 0x0f00_1000);
        let table = e.mem.alloc(0x1d0);
        e.mem.set_u32(table + 0x88, shader + 0x6c);
        e.mem.set_u32(table + 0x1cc, shader + 0x6c);
        e.mem.set_u32(table + 0x84, shader + 0x6c);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this + 8, table);
        e.call(0x00b8_b3b0, &args![this]);
        assert_eq!(calls(&e, 0x0f00_0120), vec![vec![shader], vec![shader]]);
    }

    #[test]
    fn distance_requests_measure_from_the_camera() {
        let mut e = rig();
        let point = camera_and_point(&mut e);
        e.register(REQUEST_00B8B0D0, |_, _| Ret::default());
        let this = e.mem.alloc(0x2f0);
        e.call(0x00b8_b8d0, &args![this, 0.5f32, 3u32, 2.0f32, point]);
        assert_eq!(
            calls(&e, REQUEST_00B8B0D0),
            vec![vec![
                this,
                0.5f32.to_bits(),
                3,
                2.0f32.to_bits(),
                13.0f32.to_bits(),
                13.0f32.to_bits()
            ]]
        );
        let out = e.mem.alloc(8);
        e.call(0x00b8_bb70, &args![0u32, point, out]);
        assert_eq!((e.mem.f32(out), e.mem.f32(out + 4)), (13.0, 13.0));
        // 00b8b7d0: the projection's depth plus the fourth word, into 00b8b060.
        e.register(PROJECT_00B59D30, |e, a| {
            e.mem.set_f32(a[1], 0.25);
            e.mem.set_f32(a[1] + 4, 0.75);
            e.mem.set_f32(a[2], 100.0);
            Ret::default()
        });
        e.call(
            0x00b8_b7d0,
            &args![this, 1.0f32, 9u32, 5.0f32, 20.0f32, point],
        );
        assert_eq!(e.mem.f32(this + 0x2d0), 1.0);
        assert_eq!(e.mem.u32(this + 0x2ec), 9);
        assert_eq!(
            (e.mem.f32(this + 0x2dc), e.mem.f32(this + 0x2e0)),
            (13.0, 13.0)
        );
        assert_eq!(
            (e.mem.f32(this + 0x2d4), e.mem.f32(this + 0x2d8)),
            (5.0, 120.0)
        );
        assert_eq!(e.mem.f32(this + 0x2e4), 0.25);
        // 00b8b760: the sums with the projection's depth.
        e.register(REQUEST_00B8B000, |_, _| Ret::default());
        e.call(
            0x00b8_b760,
            &args![this, 1.0f32, 2.0f32, 3.0f32, 4.0f32, 5.0f32, point],
        );
        let request = calls(&e, REQUEST_00B8B000).pop().unwrap();
        assert_eq!(
            request[..6],
            [
                this,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                103.0f32.to_bits(),
                4.0f32.to_bits(),
                105.0f32.to_bits()
            ]
        );
    }

    #[test]
    fn fn_00b8bc80_sets_the_four_flags() {
        let mut e = rig();
        e.register_double(SET_FLAG_00B8B270, |e, a| {
            let v = e.mem.u8(a[2]);
            Ret {
                eax: u32::from(v),
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x200);
        e.call(0x00b8_bc80, &args![this]);
        let set = calls(&e, SET_FLAG_00B8B270);
        let indices: Vec<u32> = set.iter().map(|w| w[1]).collect();
        assert_eq!(indices, vec![0, 1, 2, 3]);
        assert!(set.iter().all(|w| w[0] == this + 0x58));
    }

    fn render_rig(e: &mut Engine) -> (u32, u32) {
        for addr in [
            IMAGE_SPACE_TEXTURE_SET_00BA37F0,
            IMAGE_SPACE_TEXTURE_SET_00BA37A0,
            IMAGE_SPACE_TEXTURE_DESTRUCTOR,
            EFFECT_SET_TEXTURE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(0x0f00_0004, |e, _| {
            let tls = e.tls();
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), CONTEXT_IMAGE_SPACE);
            Ret::default()
        });
        e.put_vtable(0x0f00_2000, &[0, 0x0f00_0004]);
        let effect = e.mem.alloc(0x60);
        e.mem.set_u32(effect, 0x0f00_2000);
        let this = e.mem.alloc(0x200);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table + 4, effect);
        e.mem.set_u32(this + 8, table);
        e.mem.set_u32(this + 0x1f0, 0x1f0);
        e.mem.set_u32(this + 0x1f8, 0x1f8);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 3);
        (this, effect)
    }

    #[test]
    fn render_effect_renders_with_one_texture() {
        let mut e = rig();
        let (this, effect) = render_rig(&mut e);
        e.call(
            0x00b8_c730,
            &args![this, 1u32, 0xaau32, 0x7e7eu32, 0xbbu32, 0xccu32],
        );
        assert_eq!(
            calls(&e, 0x0f00_0004),
            vec![vec![effect, 0x1f0, 0xaa, 0xbb, 0xcc]]
        );
        let set = calls(&e, EFFECT_SET_TEXTURE);
        assert_eq!(set.len(), 2);
        assert_eq!(set[1], vec![effect, 0, 0, 0]);
        assert_eq!(calls(&e, IMAGE_SPACE_TEXTURE_SET_00BA37A0)[0][1], 0x7e7e);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 3);
        // No effect at that index: only the local texture.
        e.call(
            0x00b8_c730,
            &args![this, 0u32, 0xaau32, 0x7e7eu32, 0xbbu32, 0xccu32],
        );
        assert_eq!(calls(&e, 0x0f00_0004).len(), 1);
        assert_eq!(calls(&e, IMAGE_SPACE_TEXTURE_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn two_texture_render_uses_the_partial_geometry_when_set() {
        let mut e = rig();
        let (this, effect) = render_rig(&mut e);
        e.mem.set_u8(this + 0x1fc, 1);
        e.call(
            0x00b9_75f0,
            &args![this, 1u32, 0xaau32, 0x11u32, 0x22u32, 0xbbu32, 0xccu32],
        );
        assert_eq!(
            calls(&e, 0x0f00_0004),
            vec![vec![effect, 0x1f8, 0xaa, 0xbb, 0xcc]]
        );
        assert_eq!(calls(&e, IMAGE_SPACE_TEXTURE_SET_00BA37F0)[0][1], 0x11);
        assert_eq!(calls(&e, IMAGE_SPACE_TEXTURE_SET_00BA37A0)[0][1], 0x22);
        let set = calls(&e, EFFECT_SET_TEXTURE);
        let one = calls(&e, IMAGE_SPACE_TEXTURE_SET_00BA37F0)[0][0];
        let two = calls(&e, IMAGE_SPACE_TEXTURE_SET_00BA37A0)[0][0];
        assert_eq!(set[0], vec![effect, 0, two, 0]);
        assert_eq!(set[1], vec![effect, 1, one, 0]);
        assert_eq!(set.len(), 4);
        let destroyed: Vec<u32> = calls(&e, IMAGE_SPACE_TEXTURE_DESTRUCTOR)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(destroyed, vec![two, one]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 3);
    }

    #[test]
    fn fn_00b8cb00_and_00b8d440() {
        let mut e = rig();
        let this = e.mem.alloc(0x200);
        e.call(0x00b8_cb00, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(this + 0xb0), 0x55);
        e.register(IMAGE_SPACE_TEXTURE_GET_TEXTURE, |_, a| {
            u32::from(a[0] == 0x7000).into_ret()
        });
        let textures = e.mem.alloc(0xc);
        e.mem.set_u32(this + 0x2c, textures);
        assert_eq!(e.call(0x00b8_d440, &args![this]).u32(), 0);
        e.mem.set_u32(textures + 8, 0x7001);
        assert_eq!(e.call(0x00b8_d440, &args![this]).u32(), 0);
        e.mem.set_u32(textures + 8, 0x7000);
        assert_eq!(e.call(0x00b8_d440, &args![this]).u32(), 1);
    }

    #[test]
    fn create_partial_screen_geometry_builds_and_hands_out_the_elements() {
        let mut e = rig();
        e.mem.set_f32(RECTANGLE_X_01012054, -1.0);
        e.mem.set_f64(MINUS_TWO_0106D4C8, -2.0);
        e.mem.set_u32(NULL_NAME_0109B220, 0x0e0e);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(SCREEN_ELEMENTS_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, 1);
            a[0].into_ret()
        });
        e.register(SCREEN_ELEMENTS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xb8, a[1]);
            a[0].into_ret()
        });
        for addr in [
            SCREEN_ELEMENTS_DATA_INSERT,
            SCREEN_ELEMENTS_DATA_SET_RECTANGLE,
            SCREEN_ELEMENTS_DATA_SET_TEXTURES,
            SCREEN_ELEMENTS_DATA_UPDATE_BOUND,
            SET_NAME_00A5B950,
            UPDATE_PROPERTIES,
            UPDATE_00A59C60,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(MAKE_FIXED_STRING_00A5B690, |_, _| 0x0e0eu32.into_ret());
        let this = e.mem.alloc(0x200);
        let result = e.mem.alloc(4);
        e.call(0x00b8_c3b0, &args![this, 0.5f32, 0.25f32, result]);
        let elements = e.mem.u32(this + 0x1f8);
        let data = calls(&e, SCREEN_ELEMENTS_DATA_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(result), elements);
        assert_eq!(e.mem.u32(elements + 4), 2);
        // The local data pointer took a reference and gave it back.
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_SET_RECTANGLE),
            vec![vec![
                data,
                0,
                (-1.0f32).to_bits(),
                1.0f32.to_bits(),
                1.0f32.to_bits(),
                (-0.5f32).to_bits()
            ]]
        );
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_SET_TEXTURES),
            vec![vec![data, 0, 0, 0, 0, 0.5f32.to_bits(), 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, SCREEN_ELEMENTS_DATA_INSERT),
            vec![vec![data, 4, 0, 0]]
        );
        assert_eq!(
            calls(&e, MAKE_FIXED_STRING_00A5B690),
            vec![vec![PARTIAL_GEOMETRY_NAME_010B04C8]]
        );
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 0);
    }
}
