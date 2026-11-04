//! `fallout shared/tempeffects/run_00682510` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006825e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00682600`, a destructor with an exception frame
/// that clears `+8` through `00528820`, frees the object at `+0xc` through `006826a0` and
/// finishes through `00682680`, and returns nothing. Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. `00682600`'s
/// disassembly ends with its exception-frame unlink and returns no value (Ghidra types it
/// void).
pub fn fn_006825e0(e: &mut Engine, this: Ptr) {
    e.call(0x0068_2600, &args![this]);
}

// Translated from 00682680 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `006826d0`, which frees the block held at `+0`
/// through the cdecl `004ede70`, and returns nothing. Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. `006826d0`'s
/// disassembly ends with the stack cleanup of that call (`ADD ESP,4`) and computes no result
/// (Ghidra types it void).
pub fn fn_00682680(e: &mut Engine, this: Ptr) {
    e.call(0x0068_26d0, &args![this]);
}

/// The constructor body `00682530` (`this` plus one stack word, `RET 4`).
const CONSTRUCTOR_BODY_00682530: u32 = 0x0068_2530;

// Translated from 00682510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: forwards `this` and its one stack word to `00682530` and
/// returns `this`.
pub fn fn_00682510(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    e.call(CONSTRUCTOR_BODY_00682530, &args![this, argument]);
    this
}

// Translated from 00682530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a constructor body (thiscall, one word, `RET 4`): runs [`fn_00682570`] with 0,
/// stores 0 at `+0xc` and the argument at `+0x10` and `+0x14`. Returns `this`.
pub fn fn_00682530(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    fn_00682570(e, this, 0);
    let base = this.addr();
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, value);
    e.mem.set_u32(base + 0x14, value);
    this
}

// Translated from 00682570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a forwarder (thiscall, one word, `RET 4`) to [`fn_00682590`]. Returns `this`.
pub fn fn_00682570(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    fn_00682590(e, this, count);
    this
}

// Translated from 00682590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a block constructor (thiscall, one word, `RET 4`): with a count other than 0 the
/// block at `+0` is `0096afc0(count)` (cdecl), otherwise 0; the count goes to `+4` and 0 to
/// `+8`. Returns `this`.
pub fn fn_00682590(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    let base = this.addr();
    if count > 0 {
        let block = e.call(0x0096_afc0, &args![count]).u32();
        e.mem.set_u32(base, block);
    } else {
        e.mem.set_u32(base, 0);
    }
    e.mem.set_u32(base + 4, count);
    e.mem.set_u32(base + 8, 0);
    this
}

// Translated from 00682600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00682600 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: a destructor (thiscall): `00528820` (thiscall) on `this`; when the object at `+0xc`
/// is set, its `006826a0` (thiscall) with 1; then [`fn_00682680`].
pub fn fn_00682600(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0052_8820, &args![base]);
    let owned = e.mem.u32(base + 0xc);
    if owned != 0 {
        e.call(0x0068_26a0, &args![owned, 1u32]);
    }
    fn_00682680(e, this);
}

// Translated from 006826d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: frees the block at `+0` through `004ede70` (cdecl, one word).
pub fn fn_006826d0(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr());
    e.call(0x004e_de70, &args![block]);
}

/// The vtable `006827b0` stores (RTTI `BSTempEffect`).
const VTABLE_BSTEMPEFFECT_01069754: u32 = 0x0106_9754;

// Translated from 006827b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `BSTempEffect` constructor (by its vtable `01069754`; thiscall, two words, `RET
/// 8`): the base constructor `00a5d3a0` (thiscall), then the vtable, the first word at `+0xc`,
/// the float at `+8` (`FLD`/`FSTP`), 0.0 at `+0x10` and the byte 0 at `+0x14`. Returns `this`.
pub fn fn_006827b0(e: &mut Engine, this: Ptr, cell: u32, lifetime: f32) -> Ptr {
    let base = this.addr();
    e.call(0x00a5_d3a0, &args![base]);
    e.mem.set_u32(base, VTABLE_BSTEMPEFFECT_01069754);
    e.mem.set_u32(base + 0xc, cell);
    e.mem.set_u32(base + 8, x87_quiet_bits(lifetime.to_bits()));
    e.mem.set_u32(base + 0x10, 0.0f32.to_bits());
    e.mem.set_u8(base + 0x14, 0);
    this
}

/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;

// Translated from 006826a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a scalar deleting destructor (thiscall, `RET 4`): the body
/// [`fn_006826f0`], then frees `this` (`00401030`) when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_006826a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006826f0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 006826f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a destructor body: the array at `+0` is deleted
/// ([`fn_00682740`]), then the object at `+8`, when set, is deleted the
/// same way as this one ([`fn_006826a0`] with 1).
pub fn fn_006826f0(e: &mut Engine, this: Ptr) {
    let array = e.mem.u32(this.addr());
    fn_00682740(e, array);
    let next = e.mem.u32(this.addr() + 8);
    if next != 0 {
        fn_006826a0(e, Ptr::new(next), 1);
    }
}

// Translated from 00682740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): deletes an array made with a count word in
/// front of it. Nothing for null; when the count at `array - 4` is non-zero
/// the first element's vtable slot 0 (its vector deleting destructor) runs
/// with 3; otherwise the block at `array - 4` is freed (`00401030`). The
/// exe leaves no meaningful value in `EAX`.
pub fn fn_00682740(e: &mut Engine, array: u32) {
    if array == 0 {
        return;
    }
    if e.mem.u32(array.wrapping_sub(4)) != 0 {
        e.vcall(array, 0, &args![3u32]);
    } else {
        e.call(DEALLOCATE, &args![array.wrapping_sub(4)]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_26a0, fn_006826a0(Ptr, u32) -> Ptr),
        entry!(0x0068_26f0, fn_006826f0(Ptr)),
        entry!(0x0068_2740, fn_00682740(u32)),
        entry!(0x0068_25e0, fn_006825e0(Ptr)),
        entry!(0x0068_2680, fn_00682680(Ptr)),
        entry!(0x0068_2510, fn_00682510(Ptr, u32) -> Ptr),
        entry!(0x0068_2530, fn_00682530(Ptr, u32) -> Ptr),
        entry!(0x0068_2570, fn_00682570(Ptr, u32) -> Ptr),
        entry!(0x0068_2590, fn_00682590(Ptr, u32) -> Ptr),
        entry!(0x0068_2600, fn_00682600(Ptr)),
        entry!(0x0068_26d0, fn_006826d0(Ptr)),
        entry!(0x0068_27b0, fn_006827b0(Ptr, u32, f32) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_006825e0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0068_2600, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0068_25e0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0068_2600), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00682680_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0068_26d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0068_2680, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0068_26d0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn forwards_this_and_the_word_and_returns_this() {
        let mut e = Engine::new();
        e.register(CONSTRUCTOR_BODY_00682530, |_, _| Ret {
            eax: 0x999,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0068_2510, &args![0x2000u32, 0x33u32]).u32(),
            0x2000
        );
        assert_eq!(
            calls_to(&mut e, CONSTRUCTOR_BODY_00682530),
            vec![vec![0x2000, 0x33]]
        );
    }
}

#[cfg(test)]
mod b046_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn b046_00682530_00682570_and_00682590_build_the_block() {
        let mut e = Engine::new();
        e.register(0x0096_afc0, |_, a| (a[0] + 0x1000).into_ret());
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xcc; 0x18]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_2530, &args![this, 0x33u32]).u32(), this);
        assert!(calls_to(&e, 0x0096_afc0).is_empty());
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x33);
        assert_eq!(e.mem.u32(this + 0x14), 0x33);
        assert_eq!(e.call(0x0068_2570, &args![this, 4u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x0096_afc0), vec![vec![4]]);
        assert_eq!(e.mem.u32(this), 0x1004);
        assert_eq!(e.mem.u32(this + 4), 4);
        assert_eq!(e.call(0x0068_2590, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn b046_00682600_and_006826d0_release_the_parts() {
        let mut e = Engine::new();
        for address in [0x0052_8820, 0x0068_26a0, 0x004e_de70] {
            e.register(address, quiet);
        }
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0068_2600, &args![this]);
        assert_eq!(calls_to(&e, 0x0052_8820), vec![vec![this]]);
        assert!(calls_to(&e, 0x0068_26a0).is_empty());
        assert_eq!(calls_to(&e, 0x004e_de70), vec![vec![0x7000]]);
        e.mem.set_u32(this + 0xc, 0x8000);
        e.call_log = Some(vec![]);
        e.call(0x0068_2600, &args![this]);
        assert_eq!(calls_to(&e, 0x0068_26a0), vec![vec![0x8000, 1]]);
        e.call_log = Some(vec![]);
        e.call(0x0068_26d0, &args![this]);
        assert_eq!(calls_to(&e, 0x004e_de70), vec![vec![0x7000]]);
    }

    #[test]
    fn b046_006827b0_constructs_the_temp_effect() {
        let mut e = Engine::new();
        e.register(0x00a5_d3a0, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xcc; 0x18]);
        let lifetime = f32::from_bits(0x7f80_0001);
        assert_eq!(
            e.call(0x0068_27b0, &args![this, 0x44u32, lifetime]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), VTABLE_BSTEMPEFFECT_01069754);
        assert_eq!(e.mem.u32(this + 0xc), 0x44);
        assert_eq!(e.mem.u32(this + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u8(this + 0x14), 0);
        e.call(0x0068_27b0, &args![this, 0x44u32, 2.5f32]);
        assert_eq!(e.mem.u32(this + 8), 2.5f32.to_bits());
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00682740_deletes_counted_arrays() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let block = e.mem.alloc(0x10);
        let array = block + 4;
        e.mem.set_u32(array, 0x0150_0000);
        e.call_log = Some(vec![]);
        e.call(0x0068_2740, &args![0u32]);
        e.call(0x0068_2740, &args![array]);
        e.mem.set_u32(block, 2);
        e.call(0x0068_2740, &args![array]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, DEALLOCATE), vec![vec![block]]);
        assert_eq!(calls_to(&log, 0x0150_1000), vec![vec![array, 3]]);
    }

    #[test]
    fn fn_006826a0_deletes_the_chain_and_frees_on_flag_one() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, |_, _| Ret::default());
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first + 8, second);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0068_26a0, &args![first, 0u32]).u32(), first);
        let log = e.call_log.take().unwrap();
        // Only the chained object is freed (with 1); the arrays are null.
        assert_eq!(calls_to(&log, DEALLOCATE), vec![vec![second]]);
        e.call_log = Some(vec![]);
        e.call(0x0068_26a0, &args![first, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, DEALLOCATE), vec![vec![second], vec![first]]);
    }

    #[test]
    fn fn_006826f0_deletes_the_array_then_the_next_object() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        let block = e.mem.alloc(8);
        e.mem.set_u32(this, block + 4);
        e.call_log = Some(vec![]);
        e.call(0x0068_26f0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, DEALLOCATE), vec![vec![block]]);
    }
}
