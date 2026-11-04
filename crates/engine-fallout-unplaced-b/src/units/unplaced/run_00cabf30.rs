//! `(unplaced)/run_00cabf30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cabf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00cac120`. `00cac120` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00cabf30(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00ca_c120, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00cabf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xc (`SUB ECX,0xc`) and jumps to `00cac120`. `00cac120` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00cabf40(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00ca_c120, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

/// `bhkCharacterProxy`'s vtable (`010c45cc`; RTTI `.?AVbhkCharacterProxy@@`).
const VTABLE_BHK_CHARACTER_PROXY: u32 = 0x010c_45cc;
/// The word the destructor calls through (`CALL dword ptr [010c4660]`, a
/// slot of that vtable read at run time), `thiscall` on `this`.
const CALL_SLOT_010C4660: u32 = 0x010c_4660;
/// `bhkSerializable::Kill` (engine map), `thiscall`.
const BHK_SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
/// The instance count the destructor decrements (a word).
const INSTANCE_COUNT_01268340: u32 = 0x0126_8340;
/// `bhkCharacterPointCollector::~bhkCharacterPointCollector` (engine map),
/// on `this + 0x10`.
const POINT_COLLECTOR_DESTRUCTOR: u32 = 0x00cd_3490;
/// `bhkSerializable::~bhkSerializable` (engine map), `thiscall`.
const BHK_SERIALIZABLE_DESTRUCTOR: u32 = 0x00c8_77b0;

// Translated from 00cabf50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cabf50 - the C++ exception frame (FS:[0]) is not translated
/// The `bhkCharacterProxy` destructor body (named by the vtable it sets):
/// calls the function in the word at `010c4660` on `this`, then
/// `bhkSerializable::Kill`, decrements the count at `01268340`, destroys the
/// point collector at +0x10 and runs `~bhkSerializable`.
pub fn fn_00cabf50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_BHK_CHARACTER_PROXY);
    let target = e.global::<u32>(CALL_SLOT_010C4660);
    e.call(target, &args![base]);
    e.call(BHK_SERIALIZABLE_KILL, &args![base]);
    let count = e.global::<u32>(INSTANCE_COUNT_01268340);
    e.set_global(INSTANCE_COUNT_01268340, count.wrapping_sub(1));
    e.call(POINT_COLLECTOR_DESTRUCTOR, &args![base + 0x10]);
    e.call(BHK_SERIALIZABLE_DESTRUCTOR, &args![base]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca_bf30, fn_00cabf30(Ptr, u32) -> u32),
        entry!(0x00ca_bf40, fn_00cabf40(Ptr, u32) -> u32),
        entry!(0x00ca_bf50, fn_00cabf50(Ptr)),
        entry!(0x00ca_bfd0, fn_00cabfd0(Ptr, Ptr)),
    ]
}

/// `00cac9a0` (unit `hkpcharacterproxy.cpp`, void; thiscall: the destination block).
const PROXY_FILL_INFO: u32 = 0x00ca_c9a0;

// Translated from 00cabfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: when `this` is not null and its word at +8 (the Havok
/// character proxy) is not null, jumps to `00cac9a0` on that object with `destination`;
/// otherwise returns (`RET 4`).
pub fn fn_00cabfd0(e: &mut Engine, this: Ptr, destination: Ptr) {
    if this.addr() == 0 {
        return;
    }
    let proxy = e.mem.u32(this.addr() + 8);
    if proxy != 0 {
        e.call(PROXY_FILL_INFO, &args![proxy, destination]);
    }
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn fn_00cabfd0_forwards_to_the_proxy_only_when_both_are_set() {
        let mut e = Engine::new();
        e.register(PROXY_FILL_INFO, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        for (object, proxy, calls) in [(0u32, 0u32, 0usize), (this, 0, 0), (this, 0x6000, 1)] {
            e.mem.set_u32(this + 8, proxy);
            e.call_log = Some(vec![]);
            e.call(0x00ca_bfd0, &args![object, 0x7000u32]);
            let log: Vec<_> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 == PROXY_FILL_INFO)
                .collect();
            assert_eq!(log.len(), calls);
            if calls == 1 {
                assert_eq!(log[0].1, vec![0x6000, 0x7000]);
            }
        }
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00cabf30() {
        assert_eq!(
            check(0x00ca_bf30, 0x00ca_c120, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00cabf40() {
        assert_eq!(
            check(0x00ca_bf40, 0x00ca_c120, 0xc, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00cabf50_tears_down_in_order() {
        let mut e = Engine::new();
        e.map(CALL_SLOT_010C4660, 4);
        e.set_global(CALL_SLOT_010C4660, 0x7000_0094u32);
        e.map(INSTANCE_COUNT_01268340, 4);
        e.set_global(INSTANCE_COUNT_01268340, 3u32);
        for addr in [
            0x7000_0094,
            BHK_SERIALIZABLE_KILL,
            POINT_COLLECTOR_DESTRUCTOR,
            BHK_SERIALIZABLE_DESTRUCTOR,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00ca_bf50, &args![Ptr::<()>::new(this)]);
        assert_eq!(e.mem.u32(this), VTABLE_BHK_CHARACTER_PROXY);
        assert_eq!(e.global::<u32>(INSTANCE_COUNT_01268340), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x7000_0094, vec![this]),
                (BHK_SERIALIZABLE_KILL, vec![this]),
                (POINT_COLLECTOR_DESTRUCTOR, vec![this + 0x10]),
                (BHK_SERIALIZABLE_DESTRUCTOR, vec![this]),
            ]
        );
    }
}
