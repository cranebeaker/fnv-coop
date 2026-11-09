//! The classes of the render family, once each, at their PC offsets. Names marked (Xbox PDB) are the
//! prototype's; the others say what the code does with the field. A class read only as far as these
//! functions read it says so.

use crate::image;
use crate::shader::layout::StageStateCache;
use engine_core::ptr::Ptr;

image! {
    /// `NiD3DTextureStage` (Xbox PDB name), as far as read (the shader family's layout is its first
    /// 0x14 bytes).
    pub struct TextureStage / TextureStageImage: 0x68 {
        /// The filter mode (6, anisotropic, also sets the maximum anisotropy).
        0x00 m_eFilterMode: u32,
        /// `m_uiStage` (Xbox PDB).
        0x04 m_uiStage: u32,
        /// `m_spTexture` (Xbox PDB).
        0x08 m_pkTexture: u32,
        0x0c unknown_0c: u32,
        /// The stage's state cache, which `00e89060` and `00e89410` are called on.
        0x10 pStateCache: Ptr<StageStateCache>,
        /// The transform flags: the top nibble picks the matrix (0 the stage's own, 1 the source's,
        /// 2 the stage's own through `00e7df90`), the low 28 bits the mode for `00e7df90`.
        0x14 uiTransformFlags: u32,
        /// For the top nibble 1: an object whose word +0x14 is the matrix.
        0x18 pTransformSource: u32,
        /// The stage's own texture transform (a 4 x 4 `float` matrix, handed over by address).
        0x1c afTransform: [u32; 16],
        0x5c unknown_5c: [u8; 2],
        /// Whether the stage has a transform.
        0x5e bHasTransform: u8,
        0x5f unknown_5f: [u8; 5],
        /// Whether the pass binds the stage (`00be2170`).
        0x64 bEnabled: u8,
        0x65 padding_65: [u8; 3],
    }
}
