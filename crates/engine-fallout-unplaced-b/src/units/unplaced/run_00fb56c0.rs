//! `(unplaced)/run_00fb56c0` (no source unit: four of the compiler's dynamic initializers, entries
//! of the `_initterm` table `00fdf4a4..010113c0` that Ghidra never made functions of,
//! research/engine-map/ghidra_gaps.tsv; research/engine-map/unit_overrides.tsv), subsystem
//! `compiler: static init/exit`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md).
//!
//! Each constructs a global object in place with a tail jump: `MOV ECX, global` then
//! `JMP constructor`; the constructor's answer (`EAX`, the object) is left as theirs.
//! `NiInit` (`00aa20b0`) needs two of them: `00fb56c0` sets its start-up hook (`NiMain`'s
//! initializer, which makes `NiStream`'s loader map), `00fc72d0` registers one of its start-up
//! functions. The other two make a default `NiPixelFormat` (`00fb5860`) and a default
//! `NiPlane` (`00fb97d0`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiMainSDM`'s constructor (`00a8a5f0`, `thiscall`, no stack words): on its first run it
/// stores `00a89cc0` (and `00a8a280`) as `NiInit`'s start-up hook (`00aa2060`).
const NI_MAIN_SDM_CONSTRUCT: u32 = 0x00a8_a5f0;
/// The object it constructs.
const NI_MAIN_SDM: u32 = 0x011f_4414;
/// The renderer library's registrar constructor (`00e91e30`, `thiscall`, no stack words): on its
/// first run it registers `00e91dc0` (and `00e91c80`) with `NiInit`'s list (`00aa2080`).
const RENDERER_SDM_CONSTRUCT: u32 = 0x00e9_1e30;
/// The object it constructs.
const RENDERER_SDM: u32 = 0x0126_f104;

// Translated from 00fb56c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `_initterm` entry at `0100f820`: `MOV ECX,0x011f4414`, `JMP 0x00a8a5f0`: `NiMainSDM`'s
/// constructor on the global at `011f4414`. Answers what the constructor answers (`EAX`).
pub fn fn_00fb56c0(e: &mut Engine) -> u32 {
    e.call(NI_MAIN_SDM_CONSTRUCT, &args![NI_MAIN_SDM]).u32()
}

// Translated from 00fc72d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `_initterm` entry at `0101130c`: `MOV ECX,0x0126f104`, `JMP 0x00e91e30`: the renderer
/// library's registrar constructor on the global at `0126f104`. Answers what the constructor
/// answers (`EAX`).
pub fn fn_00fc72d0(e: &mut Engine) -> u32 {
    e.call(RENDERER_SDM_CONSTRUCT, &args![RENDERER_SDM]).u32()
}

/// `NiPixelFormat::NiPixelFormat` (`00a7cca0`, `thiscall`, no stack words) and its object.
const NI_PIXEL_FORMAT_CONSTRUCT: u32 = 0x00a7_cca0;
const PIXEL_FORMAT: u32 = 0x011f_4600;
/// `NiPlane::NiPlane` (`00a69940`, `thiscall`, no stack words) and its object.
const NI_PLANE_CONSTRUCT: u32 = 0x00a6_9940;
const PLANE: u32 = 0x011f_9604;

// Translated from 00fb5860 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `_initterm` entry at `0100f858`: `MOV ECX,0x011f4600`, `JMP 0x00a7cca0`:
/// `NiPixelFormat::NiPixelFormat` on the global at `011f4600`. Answers what the constructor
/// answers (`EAX`).
pub fn fn_00fb5860(e: &mut Engine) -> u32 {
    e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![PIXEL_FORMAT])
        .u32()
}

// Translated from 00fb97d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `_initterm` entry at `010100c4`: `MOV ECX,0x011f9604`, `JMP 0x00a69940`:
/// `NiPlane::NiPlane` on the global at `011f9604`. Answers what the constructor answers
/// (`EAX`).
pub fn fn_00fb97d0(e: &mut Engine) -> u32 {
    e.call(NI_PLANE_CONSTRUCT, &args![PLANE]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00fb_56c0, fn_00fb56c0() -> u32),
        entry!(0x00fb_5860, fn_00fb5860() -> u32),
        entry!(0x00fb_97d0, fn_00fb97d0() -> u32),
        entry!(0x00fc_72d0, fn_00fc72d0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each entry runs its constructor once with the global as `this` and answers its `EAX`.
    #[test]
    fn the_entries_construct_their_globals() {
        for (entry, constructor, object) in [
            (0x00fb_56c0u32, NI_MAIN_SDM_CONSTRUCT, NI_MAIN_SDM),
            (0x00fc_72d0, RENDERER_SDM_CONSTRUCT, RENDERER_SDM),
            (0x00fb_5860, NI_PIXEL_FORMAT_CONSTRUCT, PIXEL_FORMAT),
            (0x00fb_97d0, NI_PLANE_CONSTRUCT, PLANE),
        ] {
            let mut e = Engine::new();
            e.register(constructor, |_, a| (a[0] ^ 0x5a).into_ret());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(entry, &args![]).u32(), object ^ 0x5a);
            let log = e.call_log.take().unwrap();
            let calls: Vec<_> = log.iter().filter(|(a, _)| *a == constructor).collect();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].1[0], object);
        }
    }
}
