//! `NiMain/nipoint3.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa1070` (unnamed, `cdecl`, one word): allocates the text of a viewer string.
const ALLOCATE_TEXT: u32 = 0x00aa_1070;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `"%s = (%g,%g,%g)"`.
const FORMAT_POINT: u32 = 0x0109_aff4;

// Translated from 00a59af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3::GetViewerString` (Xbox PDB): `"<name> = (x,y,z)"` (format `0109aff4`, each
/// `%g`) in a text of `strlen(name) + 0x40` bytes from `00aa1070`, written by `NiSprintf`
/// with each float widened to a `double` (`FLD dword`, `FSTP qword`). Answers the text.
pub fn ni_point3_get_viewer_string(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    let size = (e.mem.cstr(name.addr()).len() as u32).wrapping_add(0x40);
    let text = e.call(ALLOCATE_TEXT, &args![size]).u32();
    let z = x87_load32(e.mem.u32(this.addr() + 8));
    let y = x87_load32(e.mem.u32(this.addr() + 4));
    let x = x87_load32(e.mem.u32(this.addr()));
    e.call(NI_SPRINTF, &args![text, size, FORMAT_POINT, name, x, y, z]);
    Ptr::new(text)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a5_9af0,
        ni_point3_get_viewer_string(Ptr, Ptr) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewer_string_sizes_the_text_and_widens_the_components() {
        let mut e = Engine::new();
        e.register(ALLOCATE_TEXT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"m_kPos\0");
        let point = e.mem.alloc(12);
        e.mem.set_f32(point, 1.5);
        e.mem.set_f32(point + 4, -2.0);
        e.mem.set_f32(point + 8, 0.25);
        let text = e.call(0x00a5_9af0, &args![point, name]).u32();
        let log = e.call_log.take().unwrap();
        let alloc: Vec<_> = log.iter().filter(|c| c.0 == ALLOCATE_TEXT).collect();
        assert_eq!(alloc[0].1, vec![6 + 0x40]);
        let sprintf: Vec<_> = log.iter().filter(|c| c.0 == NI_SPRINTF).collect();
        assert_eq!(
            sprintf[0].1,
            args![text, 0x46u32, FORMAT_POINT, name, 1.5f64, -2.0f64, 0.25f64]
        );
    }
}
