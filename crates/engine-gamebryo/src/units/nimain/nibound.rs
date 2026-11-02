//! `NiMain/nibound.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a7fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBound::GetViewerString` (Xbox PDB): `"<name> = (x, y, z) , radius"` (format `0109df6c`,
/// each `%g`): a text of `strlen(name) + 0x41` bytes (`00aa1070`) written by `NiSprintf`
/// (`00a305b0`) with the center's three floats and the radius (+0xc) widened to doubles
/// (`FLD dword`, `FSTP qword`).
pub fn ni_bound_get_viewer_string(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    let size = (e.mem.cstr(name.addr()).len() as u32).wrapping_add(0x41);
    let text = e.call(0x00aa_1070, &args![size]).u32();
    let mut words = args![text, size, 0x0109_df6cu32, name];
    for index in 0..4 {
        let value = x87_load32(e.mem.u32(this.addr() + 4 * index));
        words.extend(args![value]);
    }
    e.call(0x00a3_05b0, &words);
    Ptr::new(text)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_fb20, ni_bound_get_viewer_string(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod n015_tests {
    use super::*;

    #[test]
    fn viewer_string_passes_center_and_radius() {
        let mut e = Engine::new();
        e.register(0x00aa_1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00a3_05b0, |_, _| Ret::default());
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"bound");
        let bound = e.mem.alloc(0x10);
        for (index, value) in [1.5f32, -2.0, 0.25, 8.0].into_iter().enumerate() {
            e.mem.set_f32(bound + 4 * index as u32, value);
        }
        e.call_log = Some(vec![]);
        let text = e.call(0x00a7_fb20, &args![bound, name]).u32();
        let log = e.call_log.take().unwrap();
        let sprintf: Vec<_> = log.iter().filter(|c| c.0 == 0x00a3_05b0).collect();
        let mut expected = args![text, 0x46u32, 0x0109_df6cu32, name];
        expected.extend(args![1.5f64, -2.0f64, 0.25f64, 8.0f64]);
        assert_eq!(sprintf.len(), 1);
        assert_eq!(sprintf[0].1, expected);
        assert!(log.contains(&(0x00aa_1070, vec![0x46])));
    }
}
