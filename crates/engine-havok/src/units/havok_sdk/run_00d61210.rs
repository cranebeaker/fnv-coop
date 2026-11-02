//! `Havok SDK/run_00d61210` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d61260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_10e0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d61260(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_10e0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d61220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xc (`SUB ECX,0xc`) and jumps to `00d61260`. `00d61260` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d61220(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_1260, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00d61230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d61260`. `00d61260` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d61230(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_1260, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// The process factory object (`01269cd0`, a pointer) and the two factory methods the
/// virtual thunks `00d61240` and `00d61250` jump to: `00dd9270` (thiscall, `RET 4`: under the
/// factory's lock, the index of the entry whose name equals the argument, or -1) and
/// `00dd9240` (thiscall, `RET 4`: the name word of the entry at the index, bit 0 cleared).
const PROCESS_FACTORY: u32 = 0x0126_9cd0;
const FACTORY_FIND_PROCESS: u32 = 0x00dd_9270;
const FACTORY_PROCESS_NAME: u32 = 0x00dd_9240;
/// The process id the class of the vtables `010d1558` and `010d157c` answers (`01268fb8`).
const VIEWER_PROCESS_ID: u32 = 0x0126_8fb8;

// Translated from 00d61210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id kept at `01268fb8`: slot 1 of the vtable `010d1558` (RTTI:
/// `hkServerProcessHandler`); `ECX` unused.
pub fn fn_00d61210(e: &mut Engine) -> u32 {
    e.global(VIEWER_PROCESS_ID)
}

// Translated from 00d61240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual thunk, slot 1 of the vtable `010d157c` (RTTI: `hkServerProcessHandler`; `ECX` is the
/// object, ignored): loads `ECX` with the process factory (`01269cd0`) and jumps to `00dd9270`
/// with the one stack word `name`; answers its `EAX`.
pub fn fn_00d61240(e: &mut Engine, _this: Ptr, name: u32) -> u32 {
    let factory: u32 = e.global(PROCESS_FACTORY);
    e.call(FACTORY_FIND_PROCESS, &args![factory, name]).u32()
}

// Translated from 00d61250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual thunk, slot 2 of the vtable `010d157c` (RTTI: `hkServerProcessHandler`; `ECX` is the
/// object, ignored): loads `ECX` with the process factory (`01269cd0`) and jumps to `00dd9240`
/// with the one stack word `index`; answers its `EAX`.
pub fn fn_00d61250(e: &mut Engine, _this: Ptr, index: u32) -> u32 {
    let factory: u32 = e.global(PROCESS_FACTORY);
    e.call(FACTORY_PROCESS_NAME, &args![factory, index]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_1210, fn_00d61210() -> u32),
        entry!(0x00d6_1240, fn_00d61240(Ptr, u32) -> u32),
        entry!(0x00d6_1250, fn_00d61250(Ptr, u32) -> u32),
        entry!(0x00d6_1260, deleting_destructor_00d61260(Ptr, u8) -> Ptr),
        entry!(0x00d6_1220, fn_00d61220(Ptr, u32) -> u32),
        entry!(0x00d6_1230, fn_00d61230(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d61260_frees_only_when_bit_zero_is_set() {
        check(0x00d6_1260, 0x00d6_10e0);
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
    fn le81_thunk_00d61220() {
        assert_eq!(
            check(0x00d6_1220, 0x00d6_1260, 0xc, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d61230() {
        assert_eq!(
            check(0x00d6_1230, 0x00d6_1260, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod process_tests {
    use crate::prelude::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x2000);
        e.set_global(0x0126_9cd0, 0x1111u32);
        e
    }

    #[test]
    fn fn_00d61210_answers_the_registered_process_id() {
        let mut e = engine();
        e.set_global(0x0126_8fb8, 0xabcdu32);
        assert_eq!(e.call(0x00d6_1210, &args![0x99u32]).u32(), 0xabcd);
    }

    #[test]
    fn fn_00d61240_passes_the_name_to_the_factory() {
        let mut e = engine();
        e.register(0x00dd_9270, |_, a| Ret {
            eax: a[0] ^ a[1],
            ..Ret::default()
        });
        let got = e.call(0x00d6_1240, &args![0x99u32, 0x2222u32]).u32();
        assert_eq!(got, 0x1111 ^ 0x2222);
    }

    #[test]
    fn fn_00d61250_passes_the_index_to_the_factory() {
        let mut e = engine();
        e.register(0x00dd_9240, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        let got = e.call(0x00d6_1250, &args![0x99u32, 0x2222u32]).u32();
        assert_eq!(got, 0x1111 + 0x2222);
    }
}
