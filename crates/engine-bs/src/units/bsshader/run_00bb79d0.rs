//! `BSShader/run_00bb79d0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Ghidra has no function at `00bb79e0`; its translation follows the instruction bytes read
//! from the executable (`8b442404 56 50 8bf1 e8.. 8b8694000000 85c0 7413 6a00 6a00 83c660
//! 56 6a01 50 e8.. 83c414 5e c20400`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObjectNET::PostLinkObject` (`00a5c6a0`, `thiscall`, one word: the stream).
const NI_OBJECT_NET_POST_LINK_OBJECT: u32 = 0x00a5_c6a0;
/// `BSShaderManager::GetTexture` (engine map, `00b55840`, `cdecl`, five words).
const BS_SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;

// Translated from 00bb79d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, one word; stores its low byte at `+0xac` (on a
/// `TileShaderProperty`, whose constructor `00bb7c30` clears it, the Xbox PDB's
/// `bUseScissor` sits there once the PC's 8 bytes smaller base is counted).
pub fn fn_00bb79d0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0xac), value);
}

// Translated from 00bb79e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot +0x6c (`PostLinkObject` in the Xbox PDB's slot list) of the vtable at
/// `010b9d28` that `TileShaderProperty::TileShaderProperty` (`00bb7c30`) installs:
/// `thiscall`, one word (the stream). `NiObjectNET::PostLinkObject` with the stream, then,
/// when the word at `+0x94` (the texture file name's string) is non-null,
/// `BSShaderManager::GetTexture(name, 1, this + 0x60, 0, 0)` (the texture pointer at
/// `+0x60`); its answer is not used.
pub fn fn_00bb79e0(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(NI_OBJECT_NET_POST_LINK_OBJECT, &args![this, stream]);
    let name = e.mem.u32(this.addr().wrapping_add(0x94));
    if name != 0 {
        let texture = this.addr().wrapping_add(0x60);
        e.call(
            BS_SHADER_MANAGER_GET_TEXTURE,
            &args![name, 1u32, texture, 0u32, 0u32],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_79d0, fn_00bb79d0(Ptr, u8)),
        entry!(0x00bb_79e0, fn_00bb79e0(Ptr, u32)),
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
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00bb79d0_stores_the_low_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.call(0x00bb_79d0, &args![this, 0x1201u32]);
        assert_eq!(e.mem.u8(this + 0xac), 1);
        assert_eq!(e.mem.u8(this + 0xad), 0);
    }

    #[test]
    fn fn_00bb79e0_loads_the_texture_only_with_a_name() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_NET_POST_LINK_OBJECT, |_, _| Ret::default());
        e.register(BS_SHADER_MANAGER_GET_TEXTURE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xb0);
        e.call(0x00bb_79e0, &args![this, 0x7000u32]);
        assert!(calls(&e, BS_SHADER_MANAGER_GET_TEXTURE).is_empty());
        e.mem.set_u32(this + 0x94, 0x8000);
        e.call(0x00bb_79e0, &args![this, 0x7000u32]);
        assert_eq!(
            calls(&e, NI_OBJECT_NET_POST_LINK_OBJECT),
            vec![vec![this, 0x7000], vec![this, 0x7000]]
        );
        assert_eq!(
            calls(&e, BS_SHADER_MANAGER_GET_TEXTURE),
            vec![vec![0x8000, 1, this + 0x60, 0, 0]]
        );
    }
}
