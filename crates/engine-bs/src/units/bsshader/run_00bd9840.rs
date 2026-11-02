//! `BSShader/run_00bd9840` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::bsshaderproperty::BSShaderProperty;

/// `BSShaderProperty::RenderPassArray::Add` (`thiscall`, eight words: geometry, pass enum,
/// first-pass flag, light count, four light words).
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;

// Translated from 00bd9840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x24`: geometry, a light, pass counter, pass
/// count, first-pass byte, flags `a` and `b`, an unread word, flag `c`). The pass is `0x12f`
/// (`a` and `b`), `0x12e` (`a` only), `0x131` (`b` and `c`), `0x12d` (`b` only), `0x130`
/// (`c` only) or `0x12c` (none). When the pass count's byte is 1 the pass is added to the
/// property's render-pass list (+0x3c) with the first-pass byte and one light (the second
/// word); otherwise the 16-bit counter goes up by one. The first-pass byte is cleared
/// either way; returns 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00bd9840(
    e: &mut Engine,
    this: Ptr<BSShaderProperty>,
    geometry: u32,
    light: u32,
    counter: Ptr<u16>,
    pass_count: u8,
    first_pass: Ptr<u8>,
    a: bool,
    b: bool,
    _unused_8: u32,
    c: bool,
) -> u32 {
    let pass: u32 = if a {
        if b {
            0x12f
        } else {
            0x12e
        }
    } else if b {
        if c {
            0x131
        } else {
            0x12d
        }
    } else if c {
        0x130
    } else {
        0x12c
    };
    if pass_count == 1 {
        let first = u32::from(e.mem.u8(first_pass.addr()));
        let list = e.get(this, BSShaderProperty::pRenderPassList);
        e.call(
            RENDER_PASS_ARRAY_ADD,
            &args![list, geometry, pass, first, 1u32, light, 0u32, 0u32, 0u32],
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
        0x00bd_9840,
        fn_00bd9840(
            Ptr<BSShaderProperty>,
            u32,
            u32,
            Ptr<u16>,
            u8,
            Ptr<u8>,
            bool,
            bool,
            u32,
            bool,
        ) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bd9840_adds_one_light_pass_or_counts() {
        let cases = [
            ([1u32, 1, 0], 0x12fu32),
            ([1, 1, 1], 0x12f),
            ([1, 0, 1], 0x12e),
            ([0, 1, 1], 0x131),
            ([0, 1, 0], 0x12d),
            ([0, 0, 1], 0x130),
            ([0, 0, 0], 0x12c),
        ];
        for (count, flags, pass) in cases
            .iter()
            .flat_map(|&(f, p)| [(1u32, f, Some(p)), (0, f, None)])
        {
            let mut e = Engine::new();
            e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
            let this = e.mem.alloc(0x60);
            e.mem.set_u32(this + 0x3c, 0x0070_0000);
            let counter = e.mem.alloc(2);
            e.mem.set_u16(counter, 7);
            let first = e.mem.alloc(1);
            e.mem.set_u8(first, 1);
            e.call_log = Some(vec![]);
            let words = [
                this, 0x50, 0x99, counter, count, first, flags[0], flags[1], 0xdead, flags[2],
            ];
            assert_eq!(e.call(0x00bd_9840, &words).u32(), 0);
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
                    assert_eq!(
                        adds,
                        vec![vec![0x0070_0000, 0x50, pass, 1, 1, 0x99, 0, 0, 0]]
                    );
                    assert_eq!(e.mem.u16(counter), 7);
                }
                None => {
                    assert!(adds.is_empty());
                    assert_eq!(e.mem.u16(counter), 8);
                }
            }
            assert_eq!(e.mem.u8(first), 0);
        }
    }
}
