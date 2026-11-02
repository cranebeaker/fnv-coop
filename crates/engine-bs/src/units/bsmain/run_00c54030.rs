//! `BSMain/run_00c54030` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c54030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual function (virtual table word `010c2adc`) that
/// answers `this`; `thiscall`, `RET 4` (the stack word is not read).
pub fn fn_00c54030(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

/// `NiExtraData`'s member copy (`00a7b330`; `thiscall` on the source, `RET 8`: clone,
/// cloning process) and `NiTArray<FurnitureMark,NiTNewInterface<FurnitureMark>>::Add`
/// (`00c54b70`, engine map; `thiscall`, `RET 4`: address of the mark).
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
const FURNITURE_MARKS_ADD: u32 = 0x00c5_4b70;

// Translated from 00c54040 (decompiled, FalloutNV.exe 1.4.0.525)
/// A furniture marker's member copy (unnamed in the engine map; `thiscall` on the source,
/// `RET 8`: clone, cloning process): `NiExtraData`'s copy (`00a7b330`), then each of the
/// 16-byte marks (the array at +0x10, its 16-bit count at +0x18, read once) added to the
/// clone's array at +0xC (`00c54b70`).
pub fn fn_00c54040(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(EXTRA_DATA_COPY_MEMBERS, &args![this, clone, cloning]);
    let count = u32::from(e.mem.u16(this.addr().wrapping_add(0x18)));
    let mut index = 0u32;
    while index < count {
        let marks = e.mem.u32(this.addr().wrapping_add(0x10));
        let mark = marks.wrapping_add(index.wrapping_mul(0x10));
        e.call(
            FURNITURE_MARKS_ADD,
            &args![clone.addr().wrapping_add(0xc), mark],
        );
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_4030, fn_00c54030(Ptr, u32) -> Ptr),
        entry!(0x00c5_4040, fn_00c54040(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_4030, &args![0x1234u32, 9u32]).u32(), 0x1234);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn every_mark_is_added_to_the_clone() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_COPY_MEMBERS, |_, _| Ret::default());
        e.register(FURNITURE_MARKS_ADD, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x10, 0x5000);
        e.mem.set_u16(this + 0x18, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c5_4040, &args![this, 0x600u32, 0x700u32]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x00c5_4040)
            .collect();
        assert_eq!(
            log,
            vec![
                (EXTRA_DATA_COPY_MEMBERS, vec![this, 0x600, 0x700]),
                (FURNITURE_MARKS_ADD, vec![0x60c, 0x5000]),
                (FURNITURE_MARKS_ADD, vec![0x60c, 0x5010]),
            ]
        );
    }
}
