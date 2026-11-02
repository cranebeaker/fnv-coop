//! `BSShader/run_00bd9bc0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::bsshaderproperty::BSShaderProperty;

/// `BSShaderProperty::RenderPassArray::Add` (`thiscall`, eight words: geometry, pass enum,
/// first-pass flag, light count, four light words).
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;

// Translated from 00bd9bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x1c`: geometry, pass counter, pass count,
/// first-pass byte, then three flags `a`, `b`, `c`). The pass is `0x256` when `b` is set,
/// otherwise `0x255` (`a` and `c`), `0x254` (`a` only), `0x253` (`c` only) or `0x252`
/// (neither). When the pass count's byte is 1 it is added to the property's render-pass list
/// (+0x3c) with the first-pass byte and no lights; otherwise the 16-bit counter goes up by
/// one. The first-pass byte is cleared either way; returns 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bd9bc0(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    a: bool,
    b: bool,
    c: bool,
) -> u32 {
    let pass: u32 = if b {
        0x256
    } else if a {
        if c {
            0x255
        } else {
            0x254
        }
    } else if c {
        0x253
    } else {
        0x252
    };
    if pass_count == 1 {
        let first = u32::from(e.mem.u8(first_pass.addr()));
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        e.call(
            RENDER_PASS_ARRAY_ADD,
            &args![list, geometry, pass, first, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
    } else {
        let count = e.mem.u16(counter.addr());
        e.mem.set_u16(counter.addr(), count.wrapping_add(1));
    }
    e.mem.set_u8(first_pass.addr(), 0);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00bd_9bc0,
        fn_00bd9bc0(Ptr<BSShaderProperty>, u32, Ptr<u16>, u8, Ptr<u8>, bool, bool, bool) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bd9bc0_picks_the_pass_from_three_flags_or_counts() {
        let cases = [
            ([0u32, 1, 0], 0x256u32),
            ([1, 1, 1], 0x256),
            ([1, 0, 1], 0x255),
            ([1, 0, 0], 0x254),
            ([0, 0, 1], 0x253),
            ([0, 0, 0], 0x252),
        ];
        for (count, flags, pass) in cases
            .iter()
            .flat_map(|&(f, p)| [(1u32, f, Some(p)), (0x103, f, None)])
        {
            let mut e = Engine::new();
            e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
            let this = e.mem.alloc(0x60);
            e.mem.set_u32(this + 0x3c, 0x0070_0000);
            let counter = e.mem.alloc(2);
            e.mem.set_u16(counter, 0xffff);
            let first = e.mem.alloc(1);
            e.mem.set_u8(first, 1);
            e.call_log = Some(vec![]);
            let words = [
                this, 0x50, counter, count, first, flags[0], flags[1], flags[2],
            ];
            assert_eq!(e.call(0x00bd_9bc0, &words).u32(), 0);
            let adds: Vec<_> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a == RENDER_PASS_ARRAY_ADD)
                .map(|(_, w)| w)
                .collect();
            match pass {
                Some(pass) => {
                    assert_eq!(adds, vec![vec![0x0070_0000, 0x50, pass, 1, 0, 0, 0, 0, 0]]);
                    assert_eq!(e.mem.u16(counter), 0xffff);
                }
                None => {
                    assert!(adds.is_empty());
                    assert_eq!(e.mem.u16(counter), 0);
                }
            }
            assert_eq!(e.mem.u8(first), 0);
        }
    }
}
