//! `(unplaced)/run_00619a70` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00619a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at `this + 0x20` (returned in `AX`). The engine map
/// gives it no name.
pub fn fn_00619a70(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x20)
}

// Translated from 00619b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the byte at `this + 0x25` is set. The engine map gives
/// it no name.
pub fn fn_00619b20(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x25) & 4 != 0
}

/// `00619f70` (`ECX` = topic, one stack word: the quest): the topic's info array for the
/// quest, or null.
const QUEST_INFOS: u32 = 0x0061_9f70;
/// `0084e3a0` (`ECX` = array): its count; `00877a30` (`ECX` = array, one stack word): the
/// element's address.
const ARRAY_COUNT: u32 = 0x0084_e3a0;
const ELEMENT_ADDRESS: u32 = 0x0087_7a30;
/// `00619b40` (`ECX` = info), `AL`; `0061f280` (`ECX` = info).
const INFO_TEST_00619B40: u32 = 0x0061_9b40;
const INFO_0061F280: u32 = 0x0061_f280;

// Translated from 00619a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` = a topic, one stack word, `RET 4`; called by
/// `00619b60`): for each info of the topic's array for `quest` (`00619f70`; nothing when
/// null), a non-null one with flag 4 of its byte at +0x25 ([`fn_00619b20`]) that
/// `00619b40` also answers true for gets `0061f280`.
pub fn fn_00619a90(e: &mut Engine, this: Ptr, quest: u32) {
    let array = e.call(QUEST_INFOS, &args![this, quest]).u32();
    if array == 0 {
        return;
    }
    let count = e.call(ARRAY_COUNT, &args![array]).u32();
    let mut index = 0u32;
    while index < count {
        let element = e.call(ELEMENT_ADDRESS, &args![array, index]).u32();
        let info = e.mem.u32(element);
        if info != 0
            && fn_00619b20(e, Ptr::new(info))
            && e.call(INFO_TEST_00619B40, &args![info]).u8() != 0
        {
            e.call(INFO_0061F280, &args![info]);
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_9a90, fn_00619a90(Ptr, u32)),
        entry!(0x0061_9a70, fn_00619a70(Ptr) -> u16),
        entry!(0x0061_9b20, fn_00619b20(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_readers() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x20, 0x00fb_fffe);
        assert_eq!(e.call(0x0061_9a70, &args![this]).u16(), 0xfffe);
        assert!(!e.call(0x0061_9b20, &args![this]).bool());
        e.mem.set_u8(this + 0x25, 0x04);
        assert!(e.call(0x0061_9b20, &args![this]).bool());
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_00619a90_handles_the_flagged_infos_of_the_quest() {
        let mut e = Engine::new();
        let infos: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x28)).collect();
        e.mem.set_u8(infos[0] + 0x25, 4);
        e.mem.set_u8(infos[2] + 0x25, 4);
        let elements = e.mem.alloc(16);
        for (i, info) in infos.iter().enumerate() {
            e.mem.set_u32(elements + 4 * i as u32, *info);
        }
        e.mem.set_u32(elements + 12, 0);
        e.register_double(QUEST_INFOS, move |_, a| {
            (if a[1] == 0x77 { 0x5000u32 } else { 0 }).into_ret()
        });
        e.register(ARRAY_COUNT, |_, _| 4u32.into_ret());
        e.register_double(ELEMENT_ADDRESS, move |_, a| {
            (elements + 4 * a[1]).into_ret()
        });
        let rejected = infos[2];
        e.register_double(INFO_TEST_00619B40, move |_, a| {
            u32::from(a[0] != rejected).into_ret()
        });
        e.register(INFO_0061F280, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_9a90, &args![0x1000u32, 0x77u32]);
        let log = e.call_log.take().unwrap();
        let handled: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == INFO_0061F280)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(handled, vec![infos[0]]);
        e.call_log = Some(vec![]);
        e.call(0x0061_9a90, &args![0x1000u32, 0x78u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
