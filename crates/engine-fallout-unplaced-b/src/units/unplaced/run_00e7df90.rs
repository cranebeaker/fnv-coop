//! `(unplaced)/run_00e7df90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions sit inside `NiXenonRenderer/nid3dtexturestage.cpp` (the
//! renderer's texture stage class, `NiD3DTextureStage`, 0x14 bytes on the
//! Xbox) between `NiD3DTextureStage::SetFilterMode` `00e7def0` and
//! `00e7eb50`, but the Xbox PDB names no unit for them. On PC the class is
//! larger and its layout differs from the PDB's: only `m_uiStage` (`+4`) and
//! `m_spTexture` (`+8`) keep their Xbox offsets; the other fields are named
//! after what these functions do with them.
//!
//! The renderer globals (all in `.data`):
//! - `0126f6c4` the renderer object (`NiXenonRenderer` in the PDB): `+0x288`
//!   holds the Direct3D device interface, `+0x8b8` the render state object,
//!   `+0x8c4` the texture manager, `+0xa40` a 3 x 4 matrix (rows of 16 bytes,
//!   the last row the translation).
//! - `0126f6c0` the Direct3D device (COM: `AddRef` slot 4, `Release` slot 8,
//!   `SetTransform` slot `0xb0`, `SetSamplerState` slot `0x114`), `0126f6c8`
//!   the render state object (thiscall virtuals), `0126f680` a table of the
//!   textures shadowed per stage, `0126f6d0` a 4 x 4 `float` matrix the
//!   functions build, `011bf558` the number of texture stages.
//!
//! Notes for the next session:
//! - This unit is complete after `00e7eb00`.
//! - `00e7df90` is generated from the disassembly: each case is a straight
//!   x87 expression tree (`FLD`/`FMUL`/`FADDP`/`FADD`), kept in the exe's
//!   evaluation order and computed in `f64`, with one rounding to `float` at
//!   each store (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiD3DTextureStage` (Xbox PDB), as the PC build lays it out (0x60 or
    /// more bytes). `m_uiStage` and `m_spTexture` are the Xbox PDB names at
    /// the same offsets; the other names describe the use.
    pub struct NiD3DTextureStage: 0x64 {
        /// The kind of stage (the sampler type `00e7ea00` tests for `6`).
        0x00 kind: u32,
        /// `m_uiStage` (Xbox PDB).
        0x04 m_uiStage: u32,
        /// `m_spTexture` (Xbox PDB).
        0x08 m_spTexture: Ptr,
        /// The object `00e89060` and `00e89410` are called on.
        0x10 owner: Ptr,
        /// Transform flags: the top nibble selects where the matrix comes
        /// from (`0` the embedded one, `1` the texture's, `2` the embedded
        /// one through `00e7df90`), the low 28 bits the mode for `00e7df90`.
        0x14 transform_flags: u32,
        /// For the top nibble `1`: an object whose `+0x14` is the matrix.
        0x18 transform_source: Ptr,
        /// The embedded texture transform matrix (`0x40` bytes).
        0x1C transform: Inline<()>,
        /// Whether the stage has a transform.
        0x5E has_transform: bool,
    }
}

/// The renderer object (`0126f6c4`).
const RENDERER: u32 = 0x0126_f6c4;
/// The Direct3D device (`0126f6c0`).
const DEVICE: u32 = 0x0126_f6c0;
/// The render state object (`0126f6c8`).
const RENDER_STATE: u32 = 0x0126_f6c8;
/// The textures shadowed per stage (`0126f680`, one word per stage).
const SHADOWED_TEXTURES: u32 = 0x0126_f680;
/// The 4 x 4 matrix `00e7df90` fills (`0126f6d0`).
const MATRIX_RESULT: u32 = 0x0126_f6d0;
/// The number of texture stages (`011bf558`).
const STAGE_COUNT: u32 = 0x011b_f558;
/// A sampler state value (`011a9608`).
const SAMPLER_VALUE: u32 = 0x011a_9608;
/// The empty string handle of `NiFixedString` (`0109b220`).
const EMPTY_STRING: u32 = 0x0109_b220;
/// `float` `0.5` (`01016248`).
const HALF: u32 = 0x0101_6248;

/// `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Allocation (`00aa1420`), delete (`00aa1460(ptr, size)`), array delete
/// (`00aa1490(ptr, size)`) and pool delete (`00aa4060`, thiscall on the pool
/// at `011f6238`).
const ALLOCATE: u32 = 0x00aa_1420;
const FREE: u32 = 0x00aa_1460;
const FREE_ARRAY: u32 = 0x00aa_1490;
const POOL_FREE: u32 = 0x00aa_4060;
const POOL: u32 = 0x011f_6238;
/// The element constructor/iterator wrapper (`00401050(array, size, count,
/// constructor)`, stdcall) and the constructor (`00e7dc60`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
const ELEMENT_CONSTRUCTOR: u32 = 0x00e7_dc60;
/// `NiShaderFactory::ReleaseGlobalShaderConstant(NiFixedString *)`.
const RELEASE_GLOBAL_SHADER_CONSTANT: u32 = 0x00aa_04e0;
/// The stage's destructor body (`00e7ec60`), the element destructor body
/// (`00e7de30`), the release of a stage entry (`00e89670`), the callbacks on
/// the owner (`00e89060`, `00e89410`) and `00e7dc90`.
const STAGE_DESTRUCT: u32 = 0x00e7_ec60;
const ELEMENT_DESTRUCT: u32 = 0x00e7_de30;
const ELEMENT_RELEASE: u32 = 0x00e8_9670;
const OWNER_QUERY: u32 = 0x00e8_9060;
const OWNER_SET: u32 = 0x00e8_9410;
const APPLY_SAMPLER_FLAGS: u32 = 0x00e7_dc90;
const MEMSET: u32 = 0x00ec_61c0;
/// Virtual slots: COM (`AddRef`, `Release`, `SetTransform`,
/// `SetSamplerState`) and the render state object's.
const VSLOT_ADD_REF: u32 = 4;
const VSLOT_RELEASE: u32 = 8;
const VSLOT_SET_TRANSFORM: u32 = 0xb0;
const VSLOT_SET_SAMPLER_STATE: u32 = 0x114;
const RENDER_STATE_SLOT_SET_STAGE_VALUE: u32 = 0xc0;
const RENDER_STATE_SLOT_SET_TEXTURE: u32 = 0xdc;
const RENDER_STATE_SLOT_SET_TRANSFORM_FLAGS: u32 = 0xcc;
/// The texture manager's slot `0xc` (`+0x8c4` of the renderer).
const TEXTURE_MANAGER_SLOT_LOOKUP: u32 = 0xc;

/// The renderer's `float` at `offset` (`ECX` in the exe).
fn d(e: &Engine, offset: u32) -> f64 {
    let renderer = e.global::<u32>(RENDERER);
    e.mem.f32(renderer + offset) as f64
}

/// The texture matrix's `float` at `offset`.
fn m(e: &Engine, matrix: Ptr, offset: u32) -> f64 {
    e.mem.f32(matrix.addr() + offset) as f64
}

/// A `float` global.
fn g(e: &Engine, address: u32) -> f64 {
    e.global::<f32>(address) as f64
}

/// Stores a value into a `float` global (the x87 `FSTP float`).
fn s(e: &mut Engine, address: u32, value: f64) {
    e.set_global(address, value as f32);
}

// Translated from 00e7df90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the 4 x 4 texture matrix at `0126f6d0` for a texture transform of
/// kind `mode` (`1` to `6`; any other value returns false without touching
/// it). The inputs are the renderer's 3 x 4 matrix (rows at `+0xa40`,
/// `+0xa50`, `+0xa60`, translation row `+0xa70`) and the 4 x 4 matrix at
/// `matrix`; the result's rotation part is their product (the operand order
/// of the products differs between the modes), its translation row
/// (`+0x30..+0x38`) is the translated product for modes 1 and 2 and a copy
/// of the matrix's `+0x30..+0x38` for 3, 5 and 6. The fourth column
/// (`+0xc`, `+0x1c`, `+0x2c`, `+0x3c`) is zero for the modes other than 4.
/// Mode 4 clears the whole matrix and stores `0.5` (`01016248`) at `+0x14`,
/// `+0x20`, `+0x30` and `+0x34`. Returns true on success.
pub fn fn_00e7df90(e: &mut Engine, mode: u32, matrix: Ptr) -> bool {
    match mode {
        1 => {
            s(
                e,
                0x0126f6d0,
                ((d(e, 0xa44) * m(e, matrix, 0x10)) + (d(e, 0xa40) * m(e, matrix, 0)))
                    + (d(e, 0xa48) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6e0,
                ((d(e, 0xa54) * m(e, matrix, 0x10)) + (m(e, matrix, 0) * d(e, 0xa50)))
                    + (d(e, 0xa58) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6f0,
                ((m(e, matrix, 0x10) * d(e, 0xa64)) + (d(e, 0xa60) * m(e, matrix, 0)))
                    + (d(e, 0xa68) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6d4,
                ((d(e, 0xa40) * m(e, matrix, 0x4)) + (d(e, 0xa44) * m(e, matrix, 0x14)))
                    + (m(e, matrix, 0x24) * d(e, 0xa48)),
            );
            s(
                e,
                0x0126f6e4,
                ((d(e, 0xa54) * m(e, matrix, 0x14)) + (d(e, 0xa50) * m(e, matrix, 0x4)))
                    + (d(e, 0xa58) * m(e, matrix, 0x24)),
            );
            s(
                e,
                0x0126f6f4,
                ((d(e, 0xa60) * m(e, matrix, 0x4)) + (m(e, matrix, 0x14) * d(e, 0xa64)))
                    + (m(e, matrix, 0x24) * d(e, 0xa68)),
            );
            s(
                e,
                0x0126f6d8,
                ((d(e, 0xa40) * m(e, matrix, 0x8)) + (d(e, 0xa44) * m(e, matrix, 0x18)))
                    + (d(e, 0xa48) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f6e8,
                ((m(e, matrix, 0x8) * d(e, 0xa50)) + (d(e, 0xa54) * m(e, matrix, 0x18)))
                    + (d(e, 0xa58) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f6f8,
                ((d(e, 0xa64) * m(e, matrix, 0x18)) + (d(e, 0xa60) * m(e, matrix, 0x8)))
                    + (d(e, 0xa68) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f700,
                (((d(e, 0xa74) * m(e, matrix, 0x10)) + (m(e, matrix, 0) * d(e, 0xa70)))
                    + (d(e, 0xa78) * m(e, matrix, 0x20)))
                    + m(e, matrix, 0x30),
            );
            s(
                e,
                0x0126f704,
                (((d(e, 0xa74) * m(e, matrix, 0x14)) + (d(e, 0xa70) * m(e, matrix, 0x4)))
                    + (d(e, 0xa78) * m(e, matrix, 0x24)))
                    + m(e, matrix, 0x34),
            );
            s(
                e,
                0x0126f708,
                (((m(e, matrix, 0x8) * d(e, 0xa70)) + (d(e, 0xa74) * m(e, matrix, 0x18)))
                    + (d(e, 0xa78) * m(e, matrix, 0x28)))
                    + m(e, matrix, 0x38),
            );
            s(e, 0x0126f6dc, 0.0);
            s(e, 0x0126f6ec, 0.0);
            s(e, 0x0126f6fc, 0.0);
            s(e, 0x0126f70c, 0.0);
        }
        2 => {
            s(
                e,
                0x0126f6d0,
                ((m(e, matrix, 0x10) * d(e, 0xa44)) + (d(e, 0xa40) * m(e, matrix, 0)))
                    + (d(e, 0xa48) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6e0,
                ((d(e, 0xa54) * m(e, matrix, 0x10)) + (m(e, matrix, 0) * d(e, 0xa50)))
                    + (d(e, 0xa58) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6f0,
                ((m(e, matrix, 0x10) * d(e, 0xa64)) + (d(e, 0xa60) * m(e, matrix, 0)))
                    + (d(e, 0xa68) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6d4,
                ((m(e, matrix, 0x14) * d(e, 0xa44)) + (d(e, 0xa40) * m(e, matrix, 0x4)))
                    + (m(e, matrix, 0x24) * d(e, 0xa48)),
            );
            s(
                e,
                0x0126f6e4,
                ((d(e, 0xa54) * m(e, matrix, 0x14)) + (d(e, 0xa50) * m(e, matrix, 0x4)))
                    + (d(e, 0xa58) * m(e, matrix, 0x24)),
            );
            s(
                e,
                0x0126f6f4,
                ((d(e, 0xa60) * m(e, matrix, 0x4)) + (m(e, matrix, 0x14) * d(e, 0xa64)))
                    + (m(e, matrix, 0x24) * d(e, 0xa68)),
            );
            s(
                e,
                0x0126f6d8,
                ((d(e, 0xa40) * m(e, matrix, 0x8)) + (m(e, matrix, 0x18) * d(e, 0xa44)))
                    + (d(e, 0xa48) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f6e8,
                ((m(e, matrix, 0x8) * d(e, 0xa50)) + (d(e, 0xa54) * m(e, matrix, 0x18)))
                    + (d(e, 0xa58) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f6f8,
                ((d(e, 0xa64) * m(e, matrix, 0x18)) + (d(e, 0xa60) * m(e, matrix, 0x8)))
                    + (d(e, 0xa68) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f700,
                (((m(e, matrix, 0x10) * d(e, 0xa74)) + (d(e, 0xa70) * m(e, matrix, 0)))
                    + (d(e, 0xa78) * m(e, matrix, 0x20)))
                    + m(e, matrix, 0x30),
            );
            s(
                e,
                0x0126f704,
                (((d(e, 0xa70) * m(e, matrix, 0x4)) + (m(e, matrix, 0x14) * d(e, 0xa74)))
                    + (m(e, matrix, 0x24) * d(e, 0xa78)))
                    + m(e, matrix, 0x34),
            );
            s(
                e,
                0x0126f708,
                (((d(e, 0xa74) * m(e, matrix, 0x18)) + (d(e, 0xa70) * m(e, matrix, 0x8)))
                    + (d(e, 0xa78) * m(e, matrix, 0x28)))
                    + m(e, matrix, 0x38),
            );
            s(e, 0x0126f6dc, 0.0);
            s(e, 0x0126f6ec, 0.0);
            s(e, 0x0126f6fc, 0.0);
            s(e, 0x0126f70c, 0.0);
        }
        3 => {
            s(
                e,
                0x0126f6d0,
                ((m(e, matrix, 0x10) * d(e, 0xa44)) + (d(e, 0xa40) * m(e, matrix, 0)))
                    + (d(e, 0xa48) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6e0,
                ((m(e, matrix, 0x10) * d(e, 0xa54)) + (d(e, 0xa50) * m(e, matrix, 0)))
                    + (d(e, 0xa58) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6f0,
                ((d(e, 0xa64) * m(e, matrix, 0x10)) + (m(e, matrix, 0) * d(e, 0xa60)))
                    + (d(e, 0xa68) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6d4,
                ((d(e, 0xa40) * m(e, matrix, 0x4)) + (m(e, matrix, 0x14) * d(e, 0xa44)))
                    + (d(e, 0xa48) * m(e, matrix, 0x24)),
            );
            s(
                e,
                0x0126f6e4,
                ((d(e, 0xa50) * m(e, matrix, 0x4)) + (m(e, matrix, 0x14) * d(e, 0xa54)))
                    + (m(e, matrix, 0x24) * d(e, 0xa58)),
            );
            s(
                e,
                0x0126f6f4,
                ((d(e, 0xa64) * m(e, matrix, 0x14)) + (d(e, 0xa60) * m(e, matrix, 0x4)))
                    + (d(e, 0xa68) * m(e, matrix, 0x24)),
            );
            s(
                e,
                0x0126f6d8,
                ((d(e, 0xa40) * m(e, matrix, 0x8)) + (d(e, 0xa44) * m(e, matrix, 0x18)))
                    + (d(e, 0xa48) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f6e8,
                ((d(e, 0xa54) * m(e, matrix, 0x18)) + (d(e, 0xa50) * m(e, matrix, 0x8)))
                    + (d(e, 0xa58) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f6f8,
                ((m(e, matrix, 0x8) * d(e, 0xa60)) + (d(e, 0xa64) * m(e, matrix, 0x18)))
                    + (d(e, 0xa68) * m(e, matrix, 0x28)),
            );
            s(e, 0x0126f700, m(e, matrix, 0x30));
            s(e, 0x0126f704, m(e, matrix, 0x34));
            s(e, 0x0126f708, m(e, matrix, 0x38));
            s(e, 0x0126f6dc, 0.0);
            s(e, 0x0126f6ec, 0.0);
            s(e, 0x0126f6fc, 0.0);
            s(e, 0x0126f70c, 0.0);
        }
        4 => {
            e.call(MEMSET, &args![MATRIX_RESULT, 0u32, 0x40u32]);
            s(e, 0x0126f6e4, g(e, HALF));
            s(e, 0x0126f6f0, g(e, HALF));
            s(e, 0x0126f700, g(e, HALF));
            s(e, 0x0126f704, g(e, HALF));
        }
        5 => {
            s(
                e,
                0x0126f6d0,
                ((d(e, 0xa44) * m(e, matrix, 0x10)) + (m(e, matrix, 0) * d(e, 0xa40)))
                    + (d(e, 0xa48) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6e0,
                ((m(e, matrix, 0x10) * d(e, 0xa54)) + (d(e, 0xa50) * m(e, matrix, 0)))
                    + (d(e, 0xa58) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6f0,
                ((m(e, matrix, 0x10) * d(e, 0xa64)) + (d(e, 0xa60) * m(e, matrix, 0)))
                    + (d(e, 0xa68) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6d4,
                ((d(e, 0xa44) * m(e, matrix, 0x14)) + (m(e, matrix, 0x4) * d(e, 0xa40)))
                    + (m(e, matrix, 0x24) * d(e, 0xa48)),
            );
            s(
                e,
                0x0126f6e4,
                ((m(e, matrix, 0x14) * d(e, 0xa54)) + (d(e, 0xa50) * m(e, matrix, 0x4)))
                    + (m(e, matrix, 0x24) * d(e, 0xa58)),
            );
            s(
                e,
                0x0126f6f4,
                ((m(e, matrix, 0x14) * d(e, 0xa64)) + (d(e, 0xa60) * m(e, matrix, 0x4)))
                    + (d(e, 0xa68) * m(e, matrix, 0x24)),
            );
            s(
                e,
                0x0126f6d8,
                ((m(e, matrix, 0x8) * d(e, 0xa40)) + (d(e, 0xa44) * m(e, matrix, 0x18)))
                    + (d(e, 0xa48) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f6e8,
                ((d(e, 0xa50) * m(e, matrix, 0x8)) + (d(e, 0xa54) * m(e, matrix, 0x18)))
                    + (d(e, 0xa58) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f6f8,
                ((m(e, matrix, 0x18) * d(e, 0xa64)) + (m(e, matrix, 0x8) * d(e, 0xa60)))
                    + (d(e, 0xa68) * m(e, matrix, 0x28)),
            );
            s(e, 0x0126f700, m(e, matrix, 0x30));
            s(e, 0x0126f704, m(e, matrix, 0x34));
            s(e, 0x0126f708, m(e, matrix, 0x38));
            s(e, 0x0126f6dc, 0.0);
            s(e, 0x0126f6ec, 0.0);
            s(e, 0x0126f6fc, 0.0);
            s(e, 0x0126f70c, 0.0);
        }
        6 => {
            s(
                e,
                0x0126f6d0,
                ((m(e, matrix, 0x10) * d(e, 0xa44)) + (m(e, matrix, 0) * d(e, 0xa40)))
                    + (d(e, 0xa48) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6e0,
                ((d(e, 0xa54) * m(e, matrix, 0x10)) + (m(e, matrix, 0) * d(e, 0xa50)))
                    + (d(e, 0xa58) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6f0,
                ((m(e, matrix, 0x10) * d(e, 0xa64)) + (d(e, 0xa60) * m(e, matrix, 0)))
                    + (d(e, 0xa68) * m(e, matrix, 0x20)),
            );
            s(
                e,
                0x0126f6d4,
                ((m(e, matrix, 0x14) * d(e, 0xa44)) + (d(e, 0xa40) * m(e, matrix, 0x4)))
                    + (m(e, matrix, 0x24) * d(e, 0xa48)),
            );
            s(
                e,
                0x0126f6e4,
                ((d(e, 0xa54) * m(e, matrix, 0x14)) + (d(e, 0xa50) * m(e, matrix, 0x4)))
                    + (d(e, 0xa58) * m(e, matrix, 0x24)),
            );
            s(
                e,
                0x0126f6f4,
                ((d(e, 0xa60) * m(e, matrix, 0x4)) + (m(e, matrix, 0x14) * d(e, 0xa64)))
                    + (m(e, matrix, 0x24) * d(e, 0xa68)),
            );
            s(
                e,
                0x0126f6d8,
                ((m(e, matrix, 0x8) * d(e, 0xa40)) + (m(e, matrix, 0x18) * d(e, 0xa44)))
                    + (d(e, 0xa48) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f6e8,
                ((m(e, matrix, 0x8) * d(e, 0xa50)) + (d(e, 0xa54) * m(e, matrix, 0x18)))
                    + (d(e, 0xa58) * m(e, matrix, 0x28)),
            );
            s(
                e,
                0x0126f6f8,
                ((d(e, 0xa64) * m(e, matrix, 0x18)) + (d(e, 0xa60) * m(e, matrix, 0x8)))
                    + (d(e, 0xa68) * m(e, matrix, 0x28)),
            );
            s(e, 0x0126f700, m(e, matrix, 0x30));
            s(e, 0x0126f704, m(e, matrix, 0x34));
            s(e, 0x0126f708, m(e, matrix, 0x38));
            s(e, 0x0126f6dc, 0.0);
            s(e, 0x0126f6ec, 0.0);
            s(e, 0x0126f6fc, 0.0);
            s(e, 0x0126f70c, 0.0);
        }
        _ => return false,
    }
    true
}

// Translated from 00e7e7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of a texture stage: runs the destructor body
/// (`00e7ec60`) and, when bit 0 of `flags` is set, gives the object back to
/// the pool at `011f6238` (`00aa4060`). Returns `this`.
pub fn fn_00e7e7a0(
    e: &mut Engine,
    this: Ptr<NiD3DTextureStage>,
    flags: u8,
) -> Ptr<NiD3DTextureStage> {
    e.call(STAGE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(POOL_FREE, &args![POOL, this]);
    }
    this
}

// Translated from 00e7e7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vector deleting destructor of an array of `0x68`-byte elements (the
/// array's count is the word before `this`). With bit 1 of `flags`: from the
/// last element to the first, the string handle (`NiFixedString`) of the
/// object at element `+0x18` (when set) is copied (incrementing its
/// reference count unless it is the empty handle), given to
/// `NiShaderFactory::ReleaseGlobalShaderConstant`, and the copy destroyed
/// (decrementing it); then the stage entry at element `+0x10` is released
/// (`00e89670`). Bit 0 then frees the block (`00aa1490`, which starts one
/// word before the elements) and `this - 4` is returned. Without bit 1 the
/// single element's destructor body (`00e7de30`) runs, bit 0 frees it
/// (`00aa1460`) and `this` is returned.
pub fn fn_00e7e7c0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    if flags & 2 == 0 {
        e.call(ELEMENT_DESTRUCT, &args![this]);
        if flags & 1 != 0 {
            e.call(FREE, &args![this, 0x68u32]);
        }
        return this;
    }
    let block = this.addr().wrapping_sub(4);
    let count = e.mem.u32(block) as i32;
    let empty = e.global::<u32>(EMPTY_STRING);
    let mut index = count.wrapping_sub(1);
    while index >= 0 {
        let element = this.addr().wrapping_add((index as u32).wrapping_mul(0x68));
        let object = e.mem.u32(element + 0x18);
        if object != 0 {
            let handle = e.mem.u32(object + 8);
            if handle != empty {
                e.call(INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
            }
            let handle = e.mem.u32(object + 8);
            e.with_stack(4, |e, copy| {
                e.mem.set_u32(copy.addr(), handle);
                e.call(RELEASE_GLOBAL_SHADER_CONSTANT, &args![copy]);
                let copy = e.mem.u32(copy.addr());
                if copy != empty {
                    e.call(INTERLOCKED_DECREMENT, &args![copy.wrapping_sub(8)]);
                }
            });
        }
        let entry = e.mem.u32(element + 0x10);
        e.call(ELEMENT_RELEASE, &args![entry]);
        index -= 1;
    }
    if flags & 1 != 0 {
        e.call(FREE_ARRAY, &args![block, 0x68u32]);
    }
    Ptr::new(block)
}

// Translated from 00e7e880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates an array of `count` `0x68`-byte elements with the count in the
/// word before them (the size saturates at `0xffffffff` when it overflows),
/// constructs the elements (`00401050` with the constructor `00e7dc60`) and
/// returns the first element, or null when the allocation fails.
pub fn fn_00e7e880(e: &mut Engine, count: u32) -> Ptr {
    let product = count as u64 * 0x68;
    let low = if product >> 32 != 0 {
        u32::MAX
    } else {
        product as u32
    };
    let bytes = low.saturating_add(4);
    let block = e.call(ALLOCATE, &args![bytes]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.mem.set_u32(block, count);
    let first = block.wrapping_add(4);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![first, 0x68u32, count, ELEMENT_CONSTRUCTOR],
    );
    Ptr::new(first)
}

// Translated from 00e7e8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the renderer (`0126f6c4`) and takes its device (`+0x288`) as the
/// current one (`0126f6c0`), releasing the old device (COM `Release`, slot
/// 8) and adding a reference to the new (`AddRef`, slot 4); the render state
/// object (`0126f6c8`) is the renderer's `+0x8b8`. A null renderer clears
/// the device and the render state object after releasing the device.
pub fn fn_00e7e8d0(e: &mut Engine, renderer: Ptr) {
    e.set_global(RENDERER, renderer.addr());
    if !renderer.is_null() {
        let device = e.mem.u32(renderer.addr() + 0x288);
        let old = e.global::<u32>(DEVICE);
        if old != 0 {
            e.vcall(old, VSLOT_RELEASE, &args![]);
        }
        e.set_global(DEVICE, device);
        if device != 0 {
            e.vcall(device, VSLOT_ADD_REF, &args![]);
        }
        let renderer = e.global::<u32>(RENDERER);
        let render_state = e.mem.u32(renderer + 0x8b8);
        e.set_global(RENDER_STATE, render_state);
        return;
    }
    let old = e.global::<u32>(DEVICE);
    if old != 0 {
        e.vcall(old, VSLOT_RELEASE, &args![]);
    }
    e.set_global(DEVICE, 0u32);
    e.set_global(RENDER_STATE, 0u32);
}

// Translated from 00e7e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sends the stage's texture transform to the device. Does nothing for a
/// stage number at or past `011bf558`. Without a transform (`+0x5e` clear)
/// the render state object's slot `0xc0` is called with `(stage, 0x18, 0,
/// 0)`. Otherwise the top nibble of `+0x14` decides: `0` the embedded
/// matrix at `+0x1c`; `1` the matrix at `+0x14` of the object at `+0x18`;
/// `2` the embedded matrix; any other nibble does nothing. A non-zero mode
/// (the low 28 bits) of kind `1` or `2` builds a matrix through `00e7df90`
/// and, if that succeeds, sets it (`SetTransform(stage + 0x10, 0126f6d0)`);
/// otherwise the chosen matrix is set directly.
pub fn fn_00e7e940(e: &mut Engine, this: Ptr<NiD3DTextureStage>) {
    let stage = e.get(this, NiD3DTextureStage::m_uiStage);
    if stage >= e.global::<u32>(STAGE_COUNT) {
        return;
    }
    if !e.get(this, NiD3DTextureStage::has_transform) {
        let state = e.global::<u32>(RENDER_STATE);
        e.vcall(
            state,
            RENDER_STATE_SLOT_SET_STAGE_VALUE,
            &args![stage, 0x18u32, 0u32, 0u32],
        );
        return;
    }
    let flags = e.get(this, NiD3DTextureStage::transform_flags);
    let mode = flags & 0x0fff_ffff;
    let embedded = this.addr() + NiD3DTextureStage::transform.off;
    let matrix = match flags & 0xf000_0000 {
        0 => {
            set_transform(e, stage, embedded);
            return;
        }
        0x1000_0000 => {
            let source = e.get(this, NiD3DTextureStage::transform_source).addr();
            let source_matrix = e.mem.u32(source + 0x14);
            if mode == 0 {
                set_transform(e, stage, source_matrix);
                return;
            }
            source_matrix
        }
        0x2000_0000 => {
            if mode == 0 {
                set_transform(e, stage, embedded);
                return;
            }
            embedded
        }
        _ => return,
    };
    if fn_00e7df90(e, mode, Ptr::new(matrix)) {
        set_transform(e, stage, MATRIX_RESULT);
    }
}

/// `SetTransform(stage + 0x10, matrix)` on the device.
fn set_transform(e: &mut Engine, stage: u32, matrix: u32) {
    let device = e.global::<u32>(DEVICE);
    e.vcall(
        device,
        VSLOT_SET_TRANSFORM,
        &args![stage.wrapping_add(0x10), matrix],
    );
}

// Translated from 00e7ea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the stage's texture to the device. For a stage number below
/// `011bf558` the render state object is told the stage's value (slot
/// `0xc0`, `(stage, 0xb, value, 0)`, the value from `00e89060(owner, 0xb)`
/// when that succeeds, else the stage number); then the texture's slot in
/// the renderer's texture manager (`+0x8c4`, slot `0xc`, when the stage has
/// a texture) is looked up and set on the render state object (slot `0xdc`);
/// when it exists, a missing flag byte sets the stage's transform flags
/// (slot `0xcc`, `(stage, 7, 0, 0)`), `00e7dc90` applies the sampler data
/// and the transform is sent (`00e7e940`). Stages of kind `6` also set a
/// sampler state (`SetSamplerState(stage, 10, 011a9608)`).
pub fn fn_00e7ea00(e: &mut Engine, this: Ptr<NiD3DTextureStage>) {
    let stage = e.get(this, NiD3DTextureStage::m_uiStage);
    // The scratch words the callbacks fill: a dword, two flag bytes and a
    // dword (the layout of the exe's frame).
    e.with_stack(0x10, |e, frame| {
        let f = frame.addr();
        let value_slot = f; // dword, zero before `00e89060`
        let flag_a = f + 8; // byte
        let flag_b = f + 9; // byte
        let table_slot = f + 0xc; // dword
        if stage < e.global::<u32>(STAGE_COUNT) {
            e.mem.set_u32(value_slot, 0);
            e.mem.set_u8(flag_a, 0);
            let owner = e.get(this, NiD3DTextureStage::owner);
            let found = e
                .call(OWNER_QUERY, &args![owner, 0xbu32, value_slot, flag_a])
                .bool();
            let value = if found {
                e.mem.u32(value_slot)
            } else {
                e.get(this, NiD3DTextureStage::m_uiStage)
            };
            let state = e.global::<u32>(RENDER_STATE);
            let stage_now = e.get(this, NiD3DTextureStage::m_uiStage);
            e.vcall(
                state,
                RENDER_STATE_SLOT_SET_STAGE_VALUE,
                &args![stage_now, 0xbu32, value, 0u32],
            );
        }
        let mut slot = 0u32;
        let texture = e.get(this, NiD3DTextureStage::m_spTexture).addr();
        if texture != 0 {
            let renderer = e.global::<u32>(RENDERER);
            let manager = e.mem.u32(renderer + 0x8c4);
            slot = e
                .vcall(
                    manager,
                    TEXTURE_MANAGER_SLOT_LOOKUP,
                    &args![texture, flag_a, flag_b, table_slot],
                )
                .u32();
        }
        let state = e.global::<u32>(RENDER_STATE);
        let stage_now = e.get(this, NiD3DTextureStage::m_uiStage);
        e.vcall(
            state,
            RENDER_STATE_SLOT_SET_TEXTURE,
            &args![stage_now, slot],
        );
        if slot != 0 {
            if e.mem.u8(flag_b) == 0 {
                let state = e.global::<u32>(RENDER_STATE);
                let stage_now = e.get(this, NiD3DTextureStage::m_uiStage);
                e.vcall(
                    state,
                    RENDER_STATE_SLOT_SET_TRANSFORM_FLAGS,
                    &args![stage_now, 7u32, 0u32, 0u32],
                );
            }
            let table_value = e.mem.u32(table_slot);
            e.call(APPLY_SAMPLER_FLAGS, &args![this, table_value]);
            fn_00e7e940(e, this);
        }
        if e.get(this, NiD3DTextureStage::kind) == 6 {
            let device = e.global::<u32>(DEVICE);
            let stage_now = e.get(this, NiD3DTextureStage::m_uiStage);
            let value = e.global::<u32>(SAMPLER_VALUE);
            e.vcall(
                device,
                VSLOT_SET_SAMPLER_STATE,
                &args![stage_now, 10u32, value],
            );
        }
    })
}

// Translated from 00e7eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Binds the stage's texture: unless `force` is set and the texture is
/// already the one shadowed for this stage (`0126f680[stage]`) nothing
/// happens and true is returned; otherwise the shadow is updated and the
/// owner is told the stage (`00e89410`); a non-zero answer returns false,
/// else `00e7ea00` applies the stage and true is returned.
pub fn fn_00e7eb00(e: &mut Engine, this: Ptr<NiD3DTextureStage>, force: u8) -> bool {
    let stage = e.get(this, NiD3DTextureStage::m_uiStage);
    let texture = e.get(this, NiD3DTextureStage::m_spTexture).addr();
    if force == 0 && texture == e.mem.u32(SHADOWED_TEXTURES + stage.wrapping_mul(4)) {
        return true;
    }
    e.mem
        .set_u32(SHADOWED_TEXTURES + stage.wrapping_mul(4), texture);
    let owner = e.get(this, NiD3DTextureStage::owner);
    if e.call(OWNER_SET, &args![owner, stage]).u32() != 0 {
        return false;
    }
    fn_00e7ea00(e, this);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7df90, fn_00e7df90(u32, Ptr) -> bool),
        entry!(
            0x00e7e7a0,
            fn_00e7e7a0(Ptr<NiD3DTextureStage>, u8) -> Ptr<NiD3DTextureStage>
        ),
        entry!(0x00e7e7c0, fn_00e7e7c0(Ptr, u8) -> Ptr),
        entry!(0x00e7e880, fn_00e7e880(u32) -> Ptr),
        entry!(0x00e7e8d0, fn_00e7e8d0(Ptr)),
        entry!(0x00e7e940, fn_00e7e940(Ptr<NiD3DTextureStage>)),
        entry!(0x00e7ea00, fn_00e7ea00(Ptr<NiD3DTextureStage>)),
        entry!(0x00e7eb00, fn_00e7eb00(Ptr<NiD3DTextureStage>, u8) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn mock(
        e: &mut Engine,
        log: &Log,
        addr: u32,
        f: impl Fn(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        let log = log.clone();
        e.register_double(addr, move |e, w| {
            log.borrow_mut().push((addr, w.to_vec()));
            f(e, w)
        });
    }

    fn quiet(e: &mut Engine, log: &Log, addrs: &[u32]) {
        for a in addrs {
            mock(e, log, *a, |_, _| Ret::default());
        }
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn new_log() -> Log {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// A renderer with the 3 x 4 matrix rows `R0 = (1, 2, 3)`, `R1 =
    /// (4, 5, 6)`, `R2 = (7, 8, 9)`, `T = (10, 11, 12)` at `+0xa40`.
    fn engine_with_renderer() -> (Engine, u32) {
        let mut e = Engine::new();
        let renderer = e.mem.alloc(0xb00);
        e.map(RENDERER, 4);
        e.set_global(RENDERER, renderer);
        for (row, base) in [0xa40u32, 0xa50, 0xa60, 0xa70].into_iter().enumerate() {
            for column in 0..3u32 {
                e.mem.set_f32(
                    renderer + base + 4 * column,
                    (3 * row as u32 + column + 1) as f32,
                );
            }
        }
        e.map(MATRIX_RESULT, 0x40);
        e.map(HALF, 4);
        e.set_global(HALF, 0.5f32);
        (e, renderer)
    }

    /// A 4 x 4 matrix at `matrix` with `diagonal` on its diagonal and the
    /// translation `t`.
    fn matrix(e: &mut Engine, diagonal: [f32; 3], t: [f32; 3]) -> Ptr {
        let at = e.mem.alloc(0x40);
        for i in 0..3 {
            e.mem.set_f32(at + 0x14 * i as u32, diagonal[i]);
            e.mem.set_f32(at + 0x30 + 4 * i as u32, t[i]);
        }
        Ptr::new(at)
    }

    fn result(e: &Engine, offset: u32) -> f32 {
        e.global::<f32>(MATRIX_RESULT + offset)
    }

    #[test]
    fn unknown_modes_do_nothing() {
        let (mut e, _) = engine_with_renderer();
        let m = matrix(&mut e, [1.0; 3], [0.0; 3]);
        for mode in [0u32, 7, 0x80] {
            assert!(!e.call(0x00e7_df90, &args![mode, m]).bool());
        }
        assert_eq!(result(&e, 0), 0.0);
    }

    #[test]
    fn modes_one_and_two_combine_rotation_and_translation() {
        for mode in [1u32, 2] {
            let (mut e, _) = engine_with_renderer();
            let m = matrix(&mut e, [2.0, 3.0, 4.0], [0.5, 0.25, 0.125]);
            assert!(e.call(0x00e7_df90, &args![mode, m]).bool());
            // Diagonal matrix: column j of the result is column j of the
            // renderer matrix times m[j][j], so out[3*j + i]-style cells:
            assert_eq!(result(&e, 0x00), 2.0 * 1.0);
            assert_eq!(result(&e, 0x04), 3.0 * 2.0);
            assert_eq!(result(&e, 0x08), 4.0 * 3.0);
            assert_eq!(result(&e, 0x10), 2.0 * 4.0);
            assert_eq!(result(&e, 0x14), 3.0 * 5.0);
            assert_eq!(result(&e, 0x18), 4.0 * 6.0);
            assert_eq!(result(&e, 0x20), 2.0 * 7.0);
            assert_eq!(result(&e, 0x24), 3.0 * 8.0);
            assert_eq!(result(&e, 0x28), 4.0 * 9.0);
            // Translation row: renderer translation scaled, plus the matrix's.
            assert_eq!(result(&e, 0x30), 2.0 * 10.0 + 0.5);
            assert_eq!(result(&e, 0x34), 3.0 * 11.0 + 0.25);
            assert_eq!(result(&e, 0x38), 4.0 * 12.0 + 0.125);
            for fourth in [0x0c, 0x1c, 0x2c, 0x3c] {
                assert_eq!(result(&e, fourth), 0.0);
            }
        }
    }

    #[test]
    fn the_other_modes_copy_the_translation() {
        for mode in [3u32, 5, 6] {
            let (mut e, _) = engine_with_renderer();
            let m = matrix(&mut e, [2.0, 3.0, 4.0], [0.5, 0.25, 0.125]);
            assert!(e.call(0x00e7_df90, &args![mode, m]).bool(), "mode {mode}");
            assert_eq!(result(&e, 0x00), 2.0);
            assert_eq!(result(&e, 0x14), 15.0);
            assert_eq!(result(&e, 0x28), 36.0);
            assert_eq!(result(&e, 0x30), 0.5);
            assert_eq!(result(&e, 0x34), 0.25);
            assert_eq!(result(&e, 0x38), 0.125);
            for fourth in [0x0c, 0x1c, 0x2c, 0x3c] {
                assert_eq!(result(&e, fourth), 0.0);
            }
        }
    }

    #[test]
    fn mode_four_is_a_cleared_matrix_with_halves() {
        let (mut e, _) = engine_with_renderer();
        for i in 0..16 {
            e.set_global(MATRIX_RESULT + 4 * i, 9.0f32);
        }
        let log = new_log();
        mock(&mut e, &log, MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        let m = matrix(&mut e, [1.0; 3], [0.0; 3]);
        assert!(e.call(0x00e7_df90, &args![4u32, m]).bool());
        assert_eq!(calls_to(&log, MEMSET), vec![vec![MATRIX_RESULT, 0, 0x40]]);
        for i in 0..16u32 {
            let want = if [0x14, 0x20, 0x30, 0x34].contains(&(4 * i)) {
                0.5
            } else {
                0.0
            };
            assert_eq!(result(&e, 4 * i), want, "cell {i}");
        }
    }

    #[test]
    fn results_are_rounded_to_float_once_per_store() {
        // A sum whose exact value is not a float: the store rounds it.
        let (mut e, renderer) = engine_with_renderer();
        e.mem.set_f32(renderer + 0xa44, 1.0e-8);
        e.mem.set_f32(renderer + 0xa40, 1.0);
        e.mem.set_f32(renderer + 0xa48, 0.0);
        let m = matrix(&mut e, [1.0; 3], [0.0; 3]);
        assert!(e.call(0x00e7_df90, &args![1u32, m]).bool());
        // 1.0 * 1.0 + 1e-8 * 0 + 0 = 1.0
        assert_eq!(result(&e, 0), 1.0);
    }

    #[test]
    fn stage_destructor_frees_to_the_pool_only_with_bit_zero() {
        let mut e = Engine::new();
        let log = new_log();
        quiet(&mut e, &log, &[STAGE_DESTRUCT, POOL_FREE]);
        assert_eq!(e.call(0x00e7_e7a0, &args![0x1000u32, 0u32]).u32(), 0x1000);
        assert!(calls_to(&log, POOL_FREE).is_empty());
        assert_eq!(e.call(0x00e7_e7a0, &args![0x1000u32, 3u32]).u32(), 0x1000);
        assert_eq!(calls_to(&log, POOL_FREE), vec![vec![POOL, 0x1000]]);
        assert_eq!(calls_to(&log, STAGE_DESTRUCT).len(), 2);
    }

    #[test]
    fn single_element_delete_runs_the_destructor_body() {
        let mut e = Engine::new();
        let log = new_log();
        quiet(&mut e, &log, &[ELEMENT_DESTRUCT, FREE, FREE_ARRAY]);
        assert_eq!(e.call(0x00e7_e7c0, &args![0x2000u32, 0u32]).u32(), 0x2000);
        assert!(calls_to(&log, FREE).is_empty());
        assert_eq!(e.call(0x00e7_e7c0, &args![0x2000u32, 1u32]).u32(), 0x2000);
        assert_eq!(calls_to(&log, FREE), vec![vec![0x2000, 0x68]]);
        assert!(calls_to(&log, FREE_ARRAY).is_empty());
    }

    /// An array of `count` elements: returns the address of the first.
    fn element_array(e: &mut Engine, count: u32) -> u32 {
        let block = e.mem.alloc(4 + 0x68 * count);
        e.mem.set_u32(block, count);
        block + 4
    }

    #[test]
    fn array_delete_walks_from_the_last_element() {
        let mut e = Engine::new();
        let log = new_log();
        e.map(EMPTY_STRING, 4);
        e.set_global(EMPTY_STRING, 0x7777u32);
        let array = element_array(&mut e, 3);
        // Element 2 has an object with a counted string, element 1 an object
        // whose string is the empty handle, element 0 no object.
        let counted = e.mem.alloc(16);
        let object_a = e.mem.alloc(16);
        e.mem.set_u32(object_a + 8, counted + 8);
        let object_b = e.mem.alloc(16);
        e.mem.set_u32(object_b + 8, 0x7777);
        e.mem.set_u32(array + 2 * 0x68 + 0x18, object_a);
        e.mem.set_u32(array + 0x68 + 0x18, object_b);
        for i in 0..3u32 {
            e.mem.set_u32(array + i * 0x68 + 0x10, 0x100 + i);
        }
        quiet(
            &mut e,
            &log,
            &[
                INTERLOCKED_INCREMENT,
                INTERLOCKED_DECREMENT,
                RELEASE_GLOBAL_SHADER_CONSTANT,
                ELEMENT_RELEASE,
                FREE_ARRAY,
            ],
        );
        let result = e.call(0x00e7_e7c0, &args![array, 3u32]).u32();
        assert_eq!(result, array - 4);
        // Entries released last to first.
        let released: Vec<u32> = calls_to(&log, ELEMENT_RELEASE)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(released, vec![0x102, 0x101, 0x100]);
        // The counted string is incremented and decremented (at handle - 8);
        // the empty one neither.
        assert_eq!(calls_to(&log, INTERLOCKED_INCREMENT), vec![vec![counted]]);
        assert_eq!(calls_to(&log, INTERLOCKED_DECREMENT), vec![vec![counted]]);
        let releases = calls_to(&log, RELEASE_GLOBAL_SHADER_CONSTANT);
        assert_eq!(releases.len(), 2);
        // Bit 0 was set: the block is freed.
        assert_eq!(calls_to(&log, FREE_ARRAY), vec![vec![array - 4, 0x68]]);
    }

    #[test]
    fn array_delete_with_bit_zero_frees_the_block_and_empty_arrays_do_nothing() {
        let mut e = Engine::new();
        let log = new_log();
        e.map(EMPTY_STRING, 4);
        quiet(&mut e, &log, &[ELEMENT_RELEASE, FREE_ARRAY]);
        let array = element_array(&mut e, 0);
        assert_eq!(e.call(0x00e7_e7c0, &args![array, 3u32]).u32(), array - 4);
        assert!(calls_to(&log, ELEMENT_RELEASE).is_empty());
        assert_eq!(calls_to(&log, FREE_ARRAY), vec![vec![array - 4, 0x68]]);
    }

    #[test]
    fn the_decremented_handle_is_the_one_the_release_left_in_its_copy() {
        let mut e = Engine::new();
        let log = new_log();
        e.map(EMPTY_STRING, 4);
        e.set_global(EMPTY_STRING, 0x7777u32);
        let array = element_array(&mut e, 1);
        let object = e.mem.alloc(16);
        e.mem.set_u32(object + 8, 0x5008);
        e.mem.set_u32(array + 0x18, object);
        quiet(
            &mut e,
            &log,
            &[
                INTERLOCKED_INCREMENT,
                INTERLOCKED_DECREMENT,
                ELEMENT_RELEASE,
            ],
        );
        // The release replaces the copy with the empty handle: no decrement.
        mock(&mut e, &log, RELEASE_GLOBAL_SHADER_CONSTANT, |e, a| {
            e.mem.set_u32(a[0], 0x7777);
            Ret::default()
        });
        e.call(0x00e7_e7c0, &args![array, 2u32]);
        assert_eq!(calls_to(&log, INTERLOCKED_INCREMENT), vec![vec![0x5000]]);
        assert!(calls_to(&log, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn array_allocation_stores_the_count_and_constructs_the_elements() {
        let mut e = Engine::new();
        let log = new_log();
        mock(&mut e, &log, ALLOCATE, |e, a| {
            ret(e.mem.alloc(a[0].min(0x1000)))
        });
        quiet(&mut e, &log, &[VECTOR_CONSTRUCTOR_ITERATOR]);
        let first = e.call(0x00e7_e880, &args![3u32]).u32();
        assert_eq!(e.mem.u32(first - 4), 3);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![3 * 0x68 + 4]]);
        assert_eq!(
            calls_to(&log, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![first, 0x68, 3, ELEMENT_CONSTRUCTOR]]
        );
    }

    #[test]
    fn array_allocation_saturates_its_size_and_survives_failure() {
        let mut e = Engine::new();
        let log = new_log();
        mock(&mut e, &log, ALLOCATE, |_, _| ret(0));
        quiet(&mut e, &log, &[VECTOR_CONSTRUCTOR_ITERATOR]);
        assert_eq!(e.call(0x00e7_e880, &args![0x1000_0000u32]).u32(), 0);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0xffff_ffff]]);
        assert!(calls_to(&log, VECTOR_CONSTRUCTOR_ITERATOR).is_empty());
        // The product fits but the extra word does not: 0x68 * 0x2762762 =
        // 0xffff_fff0 + ... use the largest count that still fits.
        let count = u32::MAX / 0x68;
        e.call(0x00e7_e880, &args![count]);
        let size = calls_to(&log, ALLOCATE)[1][0];
        assert_eq!(size, count * 0x68 + 4);
    }

    /// A COM-like object whose `AddRef` (slot 4) and `Release` (slot 8) are
    /// recorded.
    fn com_object(e: &mut Engine, log: &Log, vtable: u32) -> u32 {
        let slots = [0, 0x0070_0004u32, 0x0070_0008];
        e.put_vtable(vtable, &slots);
        mock(e, log, 0x0070_0004, |_, _| Ret::default());
        mock(e, log, 0x0070_0008, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn setting_a_renderer_swaps_the_device_references() {
        let mut e = Engine::new();
        let log = new_log();
        for a in [RENDERER, DEVICE, RENDER_STATE] {
            e.map(a, 4);
        }
        let old = com_object(&mut e, &log, 0x0a00_6000);
        e.set_global(DEVICE, old);
        let renderer = e.mem.alloc(0x900);
        let device = e.mem.alloc(8);
        e.mem.set_u32(device, 0x0a00_6000);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x6666);
        e.call(0x00e7_e8d0, &args![renderer]);
        assert_eq!(e.global::<u32>(RENDERER), renderer);
        assert_eq!(e.global::<u32>(DEVICE), device);
        assert_eq!(e.global::<u32>(RENDER_STATE), 0x6666);
        // The old device was released, the new one gained a reference.
        assert_eq!(calls_to(&log, 0x0070_0008), vec![vec![old]]);
        assert_eq!(calls_to(&log, 0x0070_0004), vec![vec![device]]);
    }

    #[test]
    fn setting_a_renderer_without_device_or_old_device() {
        let mut e = Engine::new();
        let log = new_log();
        for a in [RENDERER, DEVICE, RENDER_STATE] {
            e.map(a, 4);
        }
        let renderer = e.mem.alloc(0x900);
        e.call(0x00e7_e8d0, &args![renderer]);
        assert_eq!(e.global::<u32>(DEVICE), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn clearing_the_renderer_releases_the_device() {
        let mut e = Engine::new();
        let log = new_log();
        for a in [RENDERER, DEVICE, RENDER_STATE] {
            e.map(a, 4);
        }
        let device = com_object(&mut e, &log, 0x0a00_6000);
        e.set_global(DEVICE, device);
        e.set_global(RENDER_STATE, 0x1111u32);
        e.set_global(RENDERER, 0x2222u32);
        e.call(0x00e7_e8d0, &args![0u32]);
        assert_eq!(e.global::<u32>(RENDERER), 0);
        assert_eq!(e.global::<u32>(DEVICE), 0);
        assert_eq!(e.global::<u32>(RENDER_STATE), 0);
        assert_eq!(calls_to(&log, 0x0070_0008), vec![vec![device]]);
        assert!(calls_to(&log, 0x0070_0004).is_empty());
    }

    /// A stage object and a device/render-state pair that record their
    /// calls: device slots 0xb0 (`SetTransform`) and 0x114
    /// (`SetSamplerState`), render state slots 0xc0, 0xcc, 0xdc.
    struct Stage {
        e: Engine,
        log: Log,
        this: Ptr<NiD3DTextureStage>,
    }

    fn stage_scene() -> Stage {
        let mut e = Engine::new();
        let log = new_log();
        for a in [RENDERER, DEVICE, RENDER_STATE, STAGE_COUNT, SAMPLER_VALUE] {
            e.map(a, 4);
        }
        e.set_global(STAGE_COUNT, 2u32);
        e.set_global(SAMPLER_VALUE, 0x33u32);
        let mut device_slots = vec![0u32; 0x80];
        device_slots[0xb0 / 4] = 0x0070_00b0;
        device_slots[0x114 / 4] = 0x0070_0114;
        e.put_vtable(0x0a00_7000, &device_slots);
        let device = e.mem.alloc(8);
        e.mem.set_u32(device, 0x0a00_7000);
        e.set_global(DEVICE, device);
        let mut state_slots = vec![0u32; 0x80];
        state_slots[0xc0 / 4] = 0x0070_00c0;
        state_slots[0xcc / 4] = 0x0070_00cc;
        state_slots[0xdc / 4] = 0x0070_00dc;
        e.put_vtable(0x0a00_7400, &state_slots);
        let state = e.mem.alloc(8);
        e.mem.set_u32(state, 0x0a00_7400);
        e.set_global(RENDER_STATE, state);
        for a in [0x0070_00b0u32, 0x0070_0114, 0x0070_00c0, 0x0070_00cc] {
            quiet(&mut e, &log, &[a]);
        }
        let renderer = e.mem.alloc(0xb00);
        e.set_global(RENDERER, renderer);
        let mut manager_slots = vec![0u32; 8];
        manager_slots[3] = 0x0070_0d0c;
        e.put_vtable(0x0a00_7c00, &manager_slots);
        let manager = e.mem.alloc(8);
        e.mem.set_u32(manager, 0x0a00_7c00);
        e.mem.set_u32(renderer + 0x8c4, manager);
        quiet(&mut e, &log, &[0x0070_0d0c]);
        e.map(MATRIX_RESULT, 0x40);
        e.map(HALF, 4);
        let this = e.new_object::<NiD3DTextureStage>();
        e.set(this, NiD3DTextureStage::m_uiStage, 1);
        e.map(SHADOWED_TEXTURES, 0x40);
        Stage { e, log, this }
    }

    #[test]
    fn a_stage_past_the_stage_count_does_nothing() {
        let mut s = stage_scene();
        s.e.set(s.this, NiD3DTextureStage::m_uiStage, 2);
        fn_00e7e940(&mut s.e, s.this);
        assert!(s.log.borrow().is_empty());
    }

    #[test]
    fn a_stage_without_transform_resets_the_stage_value() {
        let mut s = stage_scene();
        fn_00e7e940(&mut s.e, s.this);
        let state = s.e.global::<u32>(RENDER_STATE);
        assert_eq!(
            calls_to(&s.log, 0x0070_00c0),
            vec![vec![state, 1, 0x18, 0, 0]]
        );
    }

    #[test]
    fn the_embedded_matrix_is_set_directly() {
        let mut s = stage_scene();
        s.e.set(s.this, NiD3DTextureStage::has_transform, true);
        s.e.set(s.this, NiD3DTextureStage::transform_flags, 0x0000_0005);
        fn_00e7e940(&mut s.e, s.this);
        let device = s.e.global::<u32>(DEVICE);
        assert_eq!(
            calls_to(&s.log, 0x0070_00b0),
            vec![vec![device, 0x11, s.this.addr() + 0x1c]]
        );
    }

    #[test]
    fn nibble_one_uses_the_sources_matrix() {
        let mut s = stage_scene();
        s.e.set(s.this, NiD3DTextureStage::has_transform, true);
        let source = s.e.mem.alloc(0x20);
        let source_matrix = s.e.mem.alloc(0x40);
        for i in 0..3 {
            s.e.mem.set_f32(source_matrix + 0x14 * i, 1.0);
        }
        s.e.mem.set_u32(source + 0x14, source_matrix);
        s.e.set(
            s.this,
            NiD3DTextureStage::transform_source,
            Ptr::new(source),
        );
        // Mode zero: the source's matrix is set as is.
        s.e.set(s.this, NiD3DTextureStage::transform_flags, 0x1000_0000);
        fn_00e7e940(&mut s.e, s.this);
        let device = s.e.global::<u32>(DEVICE);
        assert_eq!(
            calls_to(&s.log, 0x0070_00b0),
            vec![vec![device, 0x11, source_matrix]]
        );
        // Non-zero mode: the result of `00e7df90` is set when it succeeds.
        s.e.set(s.this, NiD3DTextureStage::transform_flags, 0x1000_0002);
        fn_00e7e940(&mut s.e, s.this);
        let calls = calls_to(&s.log, 0x0070_00b0);
        assert_eq!(calls[1], vec![device, 0x11, MATRIX_RESULT]);
    }

    #[test]
    fn nibble_two_builds_from_the_embedded_matrix_unless_the_mode_is_zero() {
        let mut s = stage_scene();
        s.e.set(s.this, NiD3DTextureStage::has_transform, true);
        s.e.set(s.this, NiD3DTextureStage::transform_flags, 0x2000_0000);
        fn_00e7e940(&mut s.e, s.this);
        let device = s.e.global::<u32>(DEVICE);
        assert_eq!(
            calls_to(&s.log, 0x0070_00b0),
            vec![vec![device, 0x11, s.this.addr() + 0x1c]]
        );
        // Mode 4 always builds (and succeeds): the shared matrix is set.
        s.e.map(MATRIX_RESULT, 0x40);
        s.e.map(HALF, 4);
        s.e.set(s.this, NiD3DTextureStage::transform_flags, 0x2000_0004);
        mock(&mut s.e, &s.log, MEMSET, |_, _| Ret::default());
        fn_00e7e940(&mut s.e, s.this);
        let calls = calls_to(&s.log, 0x0070_00b0);
        assert_eq!(calls[1], vec![device, 0x11, MATRIX_RESULT]);
    }

    #[test]
    fn a_failed_build_sets_nothing_and_unknown_nibbles_are_ignored() {
        let mut s = stage_scene();
        s.e.set(s.this, NiD3DTextureStage::has_transform, true);
        s.e.map(MATRIX_RESULT, 0x40);
        // Mode 0x70 is unknown to `00e7df90`: it fails.
        s.e.set(s.this, NiD3DTextureStage::transform_flags, 0x2000_0070);
        fn_00e7e940(&mut s.e, s.this);
        assert!(calls_to(&s.log, 0x0070_00b0).is_empty());
        s.e.set(s.this, NiD3DTextureStage::transform_flags, 0x3000_0000);
        fn_00e7e940(&mut s.e, s.this);
        assert!(calls_to(&s.log, 0x0070_00b0).is_empty());
    }

    #[test]
    fn applying_a_stage_without_texture_clears_the_slot() {
        let mut s = stage_scene();
        mock(&mut s.e, &s.log, OWNER_QUERY, |e, a| {
            e.mem.set_u32(a[2], 0x42);
            ret(1)
        });
        mock(&mut s.e, &s.log, 0x0070_00dc, |_, _| Ret::default());
        s.e.set(s.this, NiD3DTextureStage::kind, 3);
        fn_00e7ea00(&mut s.e, s.this);
        let state = s.e.global::<u32>(RENDER_STATE);
        // The owner supplied the value 0x42.
        assert_eq!(
            calls_to(&s.log, 0x0070_00c0),
            vec![vec![state, 1, 0xb, 0x42, 0]]
        );
        // No texture: slot 0 is set and nothing else happens.
        assert_eq!(calls_to(&s.log, 0x0070_00dc), vec![vec![state, 1, 0]]);
        assert!(calls_to(&s.log, 0x0070_00cc).is_empty());
        assert!(calls_to(&s.log, 0x0070_0114).is_empty());
    }

    #[test]
    fn without_an_owner_answer_the_stage_number_is_the_value() {
        let mut s = stage_scene();
        mock(&mut s.e, &s.log, OWNER_QUERY, |_, _| ret(0));
        mock(&mut s.e, &s.log, 0x0070_00dc, |_, _| Ret::default());
        fn_00e7ea00(&mut s.e, s.this);
        let state = s.e.global::<u32>(RENDER_STATE);
        assert_eq!(
            calls_to(&s.log, 0x0070_00c0),
            vec![vec![state, 1, 0xb, 1, 0]]
        );
    }

    #[test]
    fn a_stage_past_the_count_skips_the_value_but_still_binds() {
        let mut s = stage_scene();
        s.e.set(s.this, NiD3DTextureStage::m_uiStage, 5);
        mock(&mut s.e, &s.log, 0x0070_00dc, |_, _| Ret::default());
        fn_00e7ea00(&mut s.e, s.this);
        assert!(calls_to(&s.log, 0x0070_00c0).is_empty());
        assert_eq!(calls_to(&s.log, 0x0070_00dc).len(), 1);
    }

    #[test]
    fn a_textured_stage_looks_up_its_slot_and_applies_it() {
        let mut s = stage_scene();
        // The renderer's texture manager answers slot 0x77 and writes the
        // flag byte and the table word.
        let mut manager_slots = vec![0u32; 8];
        manager_slots[3] = 0x0070_0c0c;
        s.e.put_vtable(0x0a00_7800, &manager_slots);
        let manager = s.e.mem.alloc(8);
        s.e.mem.set_u32(manager, 0x0a00_7800);
        let renderer = s.e.global::<u32>(RENDERER);
        s.e.mem.set_u32(renderer + 0x8c4, manager);
        mock(&mut s.e, &s.log, 0x0070_0c0c, |e, a| {
            e.mem.set_u8(a[3], 0); // flag byte B cleared
            e.mem.set_u32(a[4], 0x1234);
            ret(0x77)
        });
        mock(&mut s.e, &s.log, OWNER_QUERY, |_, _| ret(0));
        mock(&mut s.e, &s.log, 0x0070_00dc, |_, _| Ret::default());
        mock(&mut s.e, &s.log, APPLY_SAMPLER_FLAGS, |_, _| Ret::default());
        s.e.set(s.this, NiD3DTextureStage::m_spTexture, Ptr::new(0x5550));
        // No transform: the final `00e7e940` resets the stage value.
        fn_00e7ea00(&mut s.e, s.this);
        let state = s.e.global::<u32>(RENDER_STATE);
        let lookup = &calls_to(&s.log, 0x0070_0c0c)[0];
        assert_eq!(lookup[0], manager);
        assert_eq!(lookup[1], 0x5550);
        assert_eq!(calls_to(&s.log, 0x0070_00dc), vec![vec![state, 1, 0x77]]);
        // The flag byte B was clear: the transform flags are reset.
        assert_eq!(calls_to(&s.log, 0x0070_00cc), vec![vec![state, 1, 7, 0, 0]]);
        assert_eq!(
            calls_to(&s.log, APPLY_SAMPLER_FLAGS),
            vec![vec![s.this.addr(), 0x1234]]
        );
        // `00e7e940` ran: the stage has no transform, so slot 0xc0 was called
        // a second time with `(stage, 0x18, 0, 0)`.
        assert_eq!(calls_to(&s.log, 0x0070_00c0).len(), 2);
    }

    #[test]
    fn a_set_flag_byte_skips_the_flag_reset_and_kind_six_sets_the_sampler() {
        let mut s = stage_scene();
        let mut manager_slots = vec![0u32; 8];
        manager_slots[3] = 0x0070_0c0c;
        s.e.put_vtable(0x0a00_7800, &manager_slots);
        let manager = s.e.mem.alloc(8);
        s.e.mem.set_u32(manager, 0x0a00_7800);
        let renderer = s.e.global::<u32>(RENDERER);
        s.e.mem.set_u32(renderer + 0x8c4, manager);
        mock(&mut s.e, &s.log, 0x0070_0c0c, |e, a| {
            e.mem.set_u8(a[3], 1);
            ret(0x77)
        });
        mock(&mut s.e, &s.log, OWNER_QUERY, |_, _| ret(0));
        mock(&mut s.e, &s.log, 0x0070_00dc, |_, _| Ret::default());
        mock(&mut s.e, &s.log, APPLY_SAMPLER_FLAGS, |_, _| Ret::default());
        s.e.set(s.this, NiD3DTextureStage::m_spTexture, Ptr::new(0x5550));
        s.e.set(s.this, NiD3DTextureStage::kind, 6);
        fn_00e7ea00(&mut s.e, s.this);
        assert!(calls_to(&s.log, 0x0070_00cc).is_empty());
        let device = s.e.global::<u32>(DEVICE);
        assert_eq!(
            calls_to(&s.log, 0x0070_0114),
            vec![vec![device, 1, 10, 0x33]]
        );
    }

    #[test]
    fn binding_is_skipped_when_the_texture_is_already_shadowed() {
        let mut s = stage_scene();
        mock(&mut s.e, &s.log, OWNER_SET, |_, _| ret(0));
        mock(&mut s.e, &s.log, OWNER_QUERY, |_, _| ret(0));
        mock(&mut s.e, &s.log, 0x0070_00dc, |_, _| Ret::default());
        s.e.set(s.this, NiD3DTextureStage::m_spTexture, Ptr::new(0x5550));
        s.e.mem.set_u32(SHADOWED_TEXTURES + 4, 0x5550);
        assert!(fn_00e7eb00(&mut s.e, s.this, 0));
        assert!(calls_to(&s.log, OWNER_SET).is_empty());
        // Forced: the shadow is rewritten and the stage applied.
        assert!(fn_00e7eb00(&mut s.e, s.this, 1));
        assert_eq!(calls_to(&s.log, OWNER_SET).len(), 1);
        assert_eq!(calls_to(&s.log, 0x0070_00dc).len(), 1);
    }

    #[test]
    fn binding_a_new_texture_updates_the_shadow_and_applies_the_stage() {
        let mut s = stage_scene();
        mock(&mut s.e, &s.log, OWNER_SET, |_, _| ret(0));
        mock(&mut s.e, &s.log, OWNER_QUERY, |_, _| ret(0));
        mock(&mut s.e, &s.log, 0x0070_00dc, |_, _| Ret::default());
        s.e.set(s.this, NiD3DTextureStage::m_spTexture, Ptr::new(0x6660));
        assert!(e_bool(fn_00e7eb00(&mut s.e, s.this, 0)));
        assert_eq!(s.e.mem.u32(SHADOWED_TEXTURES + 4), 0x6660);
        assert_eq!(calls_to(&s.log, OWNER_SET).len(), 1);
        assert_eq!(calls_to(&s.log, OWNER_SET)[0][1], 1);
        assert_eq!(calls_to(&s.log, 0x0070_00dc).len(), 1);
    }

    fn e_bool(value: bool) -> bool {
        value
    }

    #[test]
    fn a_refused_binding_returns_false_without_applying() {
        let mut s = stage_scene();
        mock(&mut s.e, &s.log, OWNER_SET, |_, _| ret(1));
        s.e.set(s.this, NiD3DTextureStage::m_spTexture, Ptr::new(0x6660));
        assert!(!s.e.call(0x00e7_eb00, &args![s.this, 0u32]).bool());
        // The shadow was updated before the refusal.
        assert_eq!(s.e.mem.u32(SHADOWED_TEXTURES + 4), 0x6660);
        assert!(calls_to(&s.log, 0x0070_00c0).is_empty());
    }
}
