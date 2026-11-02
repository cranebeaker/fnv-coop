//! `(unplaced)/run_006bf2d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// An array's count (`0044ddc0`, `thiscall`, no arguments), the point test `006c3570`
/// (`cdecl`, `(&point, area, low, high)`) and its two float constants, 200 (`0102226c`) and
/// 32 (`0101e340`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const POINT_TEST_006C3570: u32 = 0x006c_3570;
const FLOAT_200: u32 = 0x0102_226c;
const FLOAT_32: u32 = 0x0101_e340;

// Translated from 006bf2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(&point)`): once per element of the array at
/// `[this + 8] + 0x20` (the index is not used), `006c3570(point, this + 4, 200, 32)`; true at
/// the first that answers true, else false.
pub fn fn_006bf2d0(e: &mut Engine, this: Ptr, point: Ptr) -> bool {
    let mut index = 0u32;
    loop {
        let array = e.mem.u32(this.addr() + 8).wrapping_add(0x20);
        if index >= e.call(ARRAY_COUNT, &args![array]).u32() {
            return false;
        }
        let low = x87_quiet_bits(e.global::<u32>(FLOAT_200));
        let high = x87_quiet_bits(e.global::<u32>(FLOAT_32));
        if e.call(
            POINT_TEST_006C3570,
            &args![point, this.addr() + 4, low, high],
        )
        .bool()
        {
            return true;
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006bf2d0, fn_006bf2d0(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006bf2d0_tests_once_per_element() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(FLOAT_200, 4);
        e.map(FLOAT_32, 4);
        e.set_global::<f32>(FLOAT_200, 200.0);
        e.set_global::<f32>(FLOAT_32, 32.0);
        e.register(ARRAY_COUNT, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register(POINT_TEST_006C3570, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let owner = e.mem.alloc(0x24);
        e.mem.set_u32(this + 8, owner);
        assert!(!e.call(0x006bf2d0, &args![this, 0x7000u32]).bool());
        let log = e.call_log.take().unwrap();
        let tests: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == POINT_TEST_006C3570)
            .map(|c| &c.1)
            .collect();
        assert_eq!(tests.len(), 3);
        assert_eq!(
            *tests[0],
            vec![0x7000, this + 4, 200.0f32.to_bits(), 32.0f32.to_bits()]
        );
        assert!(log
            .iter()
            .any(|c| c.0 == ARRAY_COUNT && c.1 == vec![owner + 0x20]));
        e.call_log = Some(vec![]);
        e.register(POINT_TEST_006C3570, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(e.call(0x006bf2d0, &args![this, 0x7000u32]).bool());
    }
}
