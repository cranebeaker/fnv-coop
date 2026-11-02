//! `NiMain/nifrustum.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFrustum` (Xbox PDB): `m_fLeft` (+0x00), `m_fRight` (+0x04), `m_fTop` (+0x08),
/// `m_fBottom` (+0x0c), `m_fNear` (+0x10), `m_fFar` (+0x14), `m_bOrtho` (+0x18).
const FRUSTUM_ORTHO: u32 = 0x18;

// Translated from 00a71b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFrustum::NiFrustum` (Xbox PDB; `__thiscall`, one word: `bOrtho`): stores only the
/// orthographic flag (the low byte of the word) and answers `this`; the planes are left as
/// they are.
pub fn ni_frustum_ni_frustum(e: &mut Engine, this: Ptr, ortho: u32) -> Ptr {
    e.mem.set_u8(this.addr() + FRUSTUM_ORTHO, ortho as u8);
    this
}

// Translated from 00a71b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiFrustum` constructor from its planes (unnamed in the map; `__thiscall`, seven words:
/// left, right, top, bottom, near, far as floats, then `bOrtho`): each float is copied through
/// the x87 stack (`FLD`/`FSTP`, so a signalling NaN arrives quiet) and the flag's low byte is
/// stored. Answers `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a71b80(
    e: &mut Engine,
    this: Ptr,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    near: f32,
    far: f32,
    ortho: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, x87_quiet(left).to_bits());
    e.mem.set_u8(base + FRUSTUM_ORTHO, ortho as u8);
    e.mem.set_u32(base + 0x04, x87_quiet(right).to_bits());
    e.mem.set_u32(base + 0x08, x87_quiet(top).to_bits());
    e.mem.set_u32(base + 0x0c, x87_quiet(bottom).to_bits());
    e.mem.set_u32(base + 0x10, x87_quiet(near).to_bits());
    e.mem.set_u32(base + 0x14, x87_quiet(far).to_bits());
    this
}

/// The array allocation (`00aa1070`, `cdecl`, one word; unnamed in the engine map) and
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, arguments).
const ALLOCATE_ARRAY: u32 = 0x00aa_1070;
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// The format `"%s = (N=%g,F=%g,L=%g,R=%g,T=%g,B=%g,O=%u)"`.
const FORMAT_FRUSTUM: u32 = 0x0109_cd10;

// Translated from 00a71bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFrustum::GetViewerString` (Xbox PDB; `thiscall`, one word: the name): a new buffer of
/// the name's length plus 0x50 (`00aa1070`), into which `NiSprintf` writes the name, near,
/// far, left, right, top and bottom (each float widened to a double through the x87 stack,
/// `FLD dword`/`FSTP qword`) and the orthographic byte. Answers the buffer.
pub fn ni_frustum_get_viewer_string(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    let mut length = 0u32;
    while e.mem.u8(name.addr().wrapping_add(length)) != 0 {
        length += 1;
    }
    let size = length.wrapping_add(0x50);
    let buffer = e.call(ALLOCATE_ARRAY, &args![size]).u32();
    let base = this.addr();
    let plane = |e: &Engine, offset: u32| x87_load32(e.mem.u32(base + offset));
    let (near, far) = (plane(e, 0x10), plane(e, 0x14));
    let (left, right) = (plane(e, 0x00), plane(e, 0x04));
    let (top, bottom) = (plane(e, 0x08), plane(e, 0x0c));
    let ortho = u32::from(e.mem.u8(base + FRUSTUM_ORTHO));
    e.call(
        NI_SPRINTF,
        &args![
            buffer,
            size,
            FORMAT_FRUSTUM,
            name,
            near,
            far,
            left,
            right,
            top,
            bottom,
            ortho
        ],
    );
    Ptr::new(buffer)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a71b70, ni_frustum_ni_frustum(Ptr, u32) -> Ptr),
        entry!(0x00a71b80, fn_00a71b80(Ptr, f32, f32, f32, f32, f32, f32, u32) -> Ptr),
        entry!(0x00a7_1bc0, ni_frustum_get_viewer_string(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    #[test]
    fn ni_frustum_ni_frustum_sets_only_the_flag() {
        let mut e = Engine::new();
        let f = e.mem.alloc(0x1c);
        e.mem.set_u32(f, 0x1234);
        assert_eq!(e.call(0x00a71b70, &args![f, 0x0101u32]).u32(), f);
        assert_eq!(e.mem.u8(f + 0x18), 1);
        assert_eq!(e.mem.u32(f), 0x1234);
    }

    #[test]
    fn fn_00a71b80_copies_the_planes_through_the_x87_stack() {
        let mut e = Engine::new();
        let f = e.mem.alloc(0x1c);
        e.mem.set_u8(f + 0x19, 0x77);
        let signalling = 0x7f80_0001u32;
        let back = e.call(
            0x00a71b80,
            &args![
                f,
                (-1.0f32).to_bits(),
                1.0f32.to_bits(),
                0.5f32.to_bits(),
                (-0.5f32).to_bits(),
                signalling,
                1000.0f32.to_bits(),
                0x0100u32
            ],
        );
        assert_eq!(back.u32(), f);
        assert_eq!(e.mem.u32(f), (-1.0f32).to_bits());
        assert_eq!(e.mem.u32(f + 0x0c), (-0.5f32).to_bits());
        assert_eq!(e.mem.u32(f + 0x10), 0x7fc0_0001);
        assert_eq!(e.mem.u32(f + 0x14), 1000.0f32.to_bits());
        assert_eq!(e.mem.u8(f + 0x18), 0);
        assert_eq!(e.mem.u8(f + 0x19), 0x77);
    }
}

#[cfg(test)]
mod n016_tests {
    use super::*;

    #[test]
    fn get_viewer_string_prints_the_planes() {
        let mut e = Engine::new();
        e.register(ALLOCATE_ARRAY, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let name = e.mem.alloc(8);
        e.mem.write(name, b"View\0");
        let f = e.mem.alloc(0x1c);
        for (i, v) in [-1.0f32, 1.0, 0.5, -0.5, 1.0, 1000.0].iter().enumerate() {
            e.mem.set_f32(f + 4 * i as u32, *v);
        }
        e.mem.set_u8(f + 0x18, 1);
        let buffer = e.call(0x00a7_1bc0, &args![f, name]).u32();
        let printed: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == NI_SPRINTF)
            .map(|c| c.1.clone())
            .collect();
        let doubles = |d: &[f64]| -> Vec<u32> {
            d.iter()
                .flat_map(|v| {
                    let bits = v.to_bits();
                    [bits as u32, (bits >> 32) as u32]
                })
                .collect()
        };
        let mut expected = vec![buffer, 4 + 0x50, FORMAT_FRUSTUM, name];
        expected.extend(doubles(&[1.0, 1000.0, -1.0, 1.0, 0.5, -0.5]));
        expected.push(1);
        assert_eq!(printed, vec![expected]);
    }
}
