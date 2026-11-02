//! `fallout shared/pathfinding/run_006dbcf0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0` (`cdecl`: size, place): placement `new`, answers the place.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// The element constructor (`006d5320`, `ECX` = element).
const ELEMENT_CONSTRUCT: u32 = 0x006d_5320;
/// Bytes per element.
const ELEMENT_SIZE: u32 = 0x10;

// Translated from 006dbcf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006dbcf0 - the exception-unwinding frame (FS:[0]) and its security-cookie word are not translated.
/// Constructs `count` 16-byte elements from `first` on (unnamed in the map;
/// `ECX` not read, two stack words, `RET 8`; `PathFind`'s door array calls
/// it): for each element, placement `new` (`006e6da0`, size 16) and, when
/// that answers a place, the element constructor `006d5320` on it. The
/// exception frame around the constructor call is left out.
pub fn fn_006dbcf0(e: &mut Engine, _this: Ptr, first: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = (index << 4).wrapping_add(first);
        let element = e.call(PLACEMENT_NEW, &args![ELEMENT_SIZE, place]).u32();
        if element != 0 {
            e.call(ELEMENT_CONSTRUCT, &args![element]);
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006dbcf0, fn_006dbcf0(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn constructed(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == PLACEMENT_NEW || c.0 == ELEMENT_CONSTRUCT)
            .collect()
    }

    #[test]
    fn places_and_constructs_each_element() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(ELEMENT_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = Ptr::<()>::new(0x0800_0000);
        e.call(0x006d_bcf0, &args![this, 0x0900_0000u32, 2u32]);
        assert_eq!(
            constructed(&mut e),
            vec![
                (PLACEMENT_NEW, vec![16, 0x0900_0000]),
                (ELEMENT_CONSTRUCT, vec![0x0900_0000]),
                (PLACEMENT_NEW, vec![16, 0x0900_0010]),
                (ELEMENT_CONSTRUCT, vec![0x0900_0010]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x006d_bcf0, &args![this, 0x0900_0000u32, 0u32]);
        assert!(constructed(&mut e).is_empty());
    }

    #[test]
    fn skips_the_constructor_when_no_place_comes_back() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, _| Ret::default());
        e.register(ELEMENT_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006d_bcf0, &args![Ptr::<()>::new(0), 0x0900_0000u32, 2u32]);
        assert_eq!(
            constructed(&mut e),
            vec![
                (PLACEMENT_NEW, vec![16, 0x0900_0000]),
                (PLACEMENT_NEW, vec![16, 0x0900_0010]),
            ]
        );
    }
}
