//! `NiMain/run_00a8b4b0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nitexturecodec::NiTextureCodec;

/// `m_aaucExplicitAlphaLookup` (Xbox PDB `u8[2][512]`), `+0x10`: the code reads it as two
/// bytes per compressed byte (`+0x10 + 2 * byte` and `+0x11 + 2 * byte`).
const EXPLICIT_ALPHA_LOOKUP: u32 = 0x10;

// Translated from 00a8b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The explicit-alpha block decoder `NiTextureCodec::ExpandCompressedData` (`00a8be70`)
/// installs as `m_pfnAlphaBlockDecoder` for format 5 (Ghidra made no function of it;
/// `thiscall`, one stack word: the output, `RET 4`). The exe unrolls eight steps: each reads
/// the byte at `m_pucSrcPtr`, stores the lookup's two bytes for it
/// (`m_aaucExplicitAlphaLookup` at `+0x10 + 2 * byte`, then `+0x11 + 2 * byte`) as the low
/// bytes of the output's words `2 * step` and `2 * step + 1` (the other bytes stay), and
/// advances `m_pucSrcPtr` by one.
pub fn fn_00a8b4b0(e: &mut Engine, this: Ptr<NiTextureCodec>, out: u32) {
    let base = this.addr();
    for step in 0..8u32 {
        let source = e.get(this, NiTextureCodec::m_pucSrcPtr);
        let byte = u32::from(e.mem.u8(source));
        let first = e.mem.u8(base + EXPLICIT_ALPHA_LOOKUP + 2 * byte);
        e.mem.set_u8(out + 8 * step, first);
        let source = e.get(this, NiTextureCodec::m_pucSrcPtr);
        let byte = u32::from(e.mem.u8(source));
        let second = e.mem.u8(base + EXPLICIT_ALPHA_LOOKUP + 1 + 2 * byte);
        e.mem.set_u8(out + 8 * step + 4, second);
        e.set(this, NiTextureCodec::m_pucSrcPtr, source.wrapping_add(1));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_b4b0, fn_00a8b4b0(Ptr<NiTextureCodec>, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a8b4b0_expands_eight_bytes_through_the_lookup() {
        let mut e = Engine::new();
        let codec: Ptr<NiTextureCodec> = e.new_object();
        let base = codec.addr();
        // Lookup: byte b -> (b + 1, 0x80 | b).
        for b in 0..256u32 {
            e.mem.set_u8(base + 0x10 + 2 * b, (b as u8).wrapping_add(1));
            e.mem.set_u8(base + 0x11 + 2 * b, 0x80 | b as u8);
        }
        let source = e.mem.alloc(8);
        let bytes = [0x00u8, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xfe];
        for (i, b) in bytes.iter().enumerate() {
            e.mem.set_u8(source + i as u32, *b);
        }
        e.set(codec, NiTextureCodec::m_pucSrcPtr, source);
        let out = e.mem.alloc(0x40);
        for word in 0..16u32 {
            e.mem.set_u32(out + 4 * word, 0xaabb_cc00);
        }
        e.call(0x00a8_b4b0, &args![codec, out]);
        for (i, b) in bytes.iter().enumerate() {
            let i = i as u32;
            assert_eq!(
                e.mem.u32(out + 8 * i),
                0xaabb_cc00 | u32::from(b.wrapping_add(1))
            );
            assert_eq!(
                e.mem.u32(out + 8 * i + 4),
                0xaabb_cc00 | u32::from(0x80 | *b)
            );
        }
        assert_eq!(e.get(codec, NiTextureCodec::m_pucSrcPtr), source + 8);
    }
}
