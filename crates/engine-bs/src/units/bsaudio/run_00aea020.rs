//! `BSAudio/run_00aea020` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aea210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `010a39d8` (RTTI `.?AVBSWin32Audio@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00ad7000`. One word, plain `RET`, returns nothing.
pub fn fn_00aea210(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010a_39d8);
    e.call(0x00ad_7000, &args![this]);
}

/// `GetTickCount` (import slot).
const GET_TICK_COUNT: u32 = 0x00fd_f060;
/// The audio singleton's storage (`011f7080`), its construction guard (bit 0 of
/// `011f7124`), the pointer to it (`011f6d98`), `_atexit` (`00ec658f`) and the destructor it
/// registers (`00fdbfe0`).
const AUDIO_STORAGE: u32 = 0x011f_7080;
const AUDIO_GUARD: u32 = 0x011f_7124;
const AUDIO_POINTER: u32 = 0x011f_6d98;
const ATEXIT: u32 = 0x00ec_658f;
const AUDIO_STORAGE_DESTRUCTOR: u32 = 0x00fd_bfe0;
/// The base constructor `00ad6f40` (`thiscall`, no stack words) and `memset` (`00ec61c0`).
const AUDIO_BASE_CONSTRUCTOR: u32 = 0x00ad_6f40;
const MEMSET: u32 = 0x00ec_61c0;
/// The timer's starting count, 600000 (ten minutes in milliseconds).
const TIMER_START: u32 = 600_000;

// Translated from 00aea020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aea020 - the FS:[0] exception frame (C++ unwinding of the static's construction) is not translated
/// Unnamed in the engine map (`run_00aea020`): the audio singleton's getter set-up (no
/// arguments). On the first run (bit 0 of `011f7124` clear) it sets the bit, constructs the
/// `BSWin32Audio` object at `011f7080` (`00aea1b0`) and registers its destructor `00fdbfe0`
/// with `_atexit`; then `011f6d98` points at the object.
pub fn fn_00aea020(e: &mut Engine) {
    let guard = e.mem.u32(AUDIO_GUARD);
    if guard & 1 == 0 {
        let guard = e.mem.u32(AUDIO_GUARD);
        e.mem.set_u32(AUDIO_GUARD, guard | 1);
        fn_00aea1b0(e, Ptr::new(AUDIO_STORAGE));
        e.call(ATEXIT, &args![AUDIO_STORAGE_DESTRUCTOR]);
    }
    e.mem.set_u32(AUDIO_POINTER, AUDIO_STORAGE);
}

/// The three-word timer reset shared by `00aea0a0`, `00aea0d0` and `00aea150`: +8 cleared,
/// +0 set to 600000 and +4 to the tick count.
fn reset_timer(e: &mut Engine, this: u32) {
    e.mem.set_u32(this.wrapping_add(8), 0);
    e.mem.set_u32(this, TIMER_START);
    let now = e.call(GET_TICK_COUNT, &args![]).u32();
    e.mem.set_u32(this.wrapping_add(4), now);
}

// Translated from 00aea0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea020`): a timer's constructor (`thiscall`, no stack
/// words): flags (+8) cleared, the count (+0) set to 600000 and the last tick (+4) to
/// `GetTickCount`. Answers `this`.
pub fn fn_00aea0a0(e: &mut Engine, this: Ptr) -> Ptr {
    reset_timer(e, this.addr());
    this
}

// Translated from 00aea0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea020`): the same timer reset as `00aea0a0`, without a
/// result (`thiscall`, no stack words).
pub fn fn_00aea0d0(e: &mut Engine, this: Ptr) {
    reset_timer(e, this.addr());
}

// Translated from 00aea100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea020`): advances the timer (`thiscall`, no stack
/// words): unless flag bit 0 (+8) is set, the count (+0) grows by the ticks since the last
/// tick (+4); the last tick becomes `GetTickCount` either way.
pub fn fn_00aea100(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let now = e.call(GET_TICK_COUNT, &args![]).u32();
    let elapsed = now.wrapping_sub(e.mem.u32(base.wrapping_add(4)));
    if e.mem.u32(base.wrapping_add(8)) & 1 == 0 {
        let count = e.mem.u32(base);
        e.mem.set_u32(base, count.wrapping_add(elapsed));
    }
    e.mem.set_u32(base.wrapping_add(4), now);
}

// Translated from 00aea150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea020`): the same timer reset as `00aea0d0`
/// (`thiscall`, no stack words).
pub fn fn_00aea150(e: &mut Engine, this: Ptr) {
    reset_timer(e, this.addr());
}

// Translated from 00aea1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aea020`): the `BSWin32Audio` constructor (`thiscall`, no
/// stack words; its virtual table `010a39d8`): the base constructor `00ad6f40`, the virtual
/// table, +0x38, +0x3c and +0xa0 cleared and the 0x60 bytes from +0x40 cleared (`memset`).
/// Answers `this`.
pub fn fn_00aea1b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(AUDIO_BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, 0x010a_39d8);
    e.mem.set_u32(base.wrapping_add(0x38), 0);
    e.mem.set_u32(base.wrapping_add(0x3c), 0);
    e.mem.set_u32(base.wrapping_add(0xa0), 0);
    e.call(MEMSET, &args![base.wrapping_add(0x40), 0u32, 0x60u32]);
    this
}

/// `fseek` (`00ecb65a`, cdecl: stream, offset, origin).
const FSEEK: u32 = 0x00ec_b65a;

// Translated from 00aea180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl; only data references: the second entry of two
/// four-function tables at `011acb78` and `011acb88`, beside `fread`, `fclose` (or null) and
/// `ftell`). It reads four words: a null `stream` answers -1; otherwise
/// `fseek(stream, offset_low, origin)`. The third word (the high half of a 64-bit offset, by
/// the gap) is not read.
pub fn fn_00aea180(
    e: &mut Engine,
    stream: Ptr,
    offset_low: u32,
    _unused_2: u32,
    origin: u32,
) -> i32 {
    if stream.is_null() {
        return -1;
    }
    e.call(FSEEK, &args![stream, offset_low, origin]).i32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ae_a180, fn_00aea180(Ptr, u32, u32, u32) -> i32),
        entry!(0x00ae_a210, fn_00aea210(Ptr)),
        entry!(0x00ae_a020, fn_00aea020()),
        entry!(0x00ae_a0a0, fn_00aea0a0(Ptr) -> Ptr),
        entry!(0x00ae_a0d0, fn_00aea0d0(Ptr)),
        entry!(0x00ae_a100, fn_00aea100(Ptr)),
        entry!(0x00ae_a150, fn_00aea150(Ptr)),
        entry!(0x00ae_a1b0, fn_00aea1b0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_00aea210_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00ad_7000, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ae_a210, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00ad_7000)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x010a_39d8);
        assert_eq!(
            e.mem.u32(this + 4),
            0x010a_39d8,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod b049_tests {
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
    fn fn_00aea020_constructs_the_singleton_once() {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x2000);
        e.register(AUDIO_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ae_a020, &args![]);
        assert_eq!(e.mem.u32(AUDIO_GUARD) & 1, 1);
        assert_eq!(e.mem.u32(AUDIO_STORAGE), 0x010a_39d8);
        assert_eq!(e.mem.u32(AUDIO_POINTER), AUDIO_STORAGE);
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![AUDIO_STORAGE_DESTRUCTOR]]);
        // The second run only sets the pointer.
        e.mem.set_u32(AUDIO_POINTER, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ae_a020, &args![]);
        assert!(calls_to(&e, AUDIO_BASE_CONSTRUCTOR).is_empty());
        assert!(calls_to(&e, ATEXIT).is_empty());
        assert_eq!(e.mem.u32(AUDIO_POINTER), AUDIO_STORAGE);
    }

    /// Runs the timer reset at `address` on a timer whose flags are set; answers the timer
    /// and the result.
    fn reset_timer_at(address: u32) -> (u32, u32) {
        let mut e = Engine::new();
        e.register(GET_TICK_COUNT, |_, _| 1234u32.into_ret());
        let timer = e.mem.alloc(12);
        e.mem.set_u32(timer + 8, 7);
        let back = e.call(address, &args![timer]).u32();
        assert_eq!(e.mem.u32(timer), 600_000);
        assert_eq!(e.mem.u32(timer + 4), 1234);
        assert_eq!(e.mem.u32(timer + 8), 0);
        (timer, back)
    }

    #[test]
    fn fn_00aea0a0_constructs_a_timer() {
        let (timer, back) = reset_timer_at(0x00ae_a0a0);
        assert_eq!(back, timer);
    }

    #[test]
    fn fn_00aea0d0_resets_a_timer() {
        reset_timer_at(0x00ae_a0d0);
    }

    #[test]
    fn fn_00aea150_resets_a_timer() {
        reset_timer_at(0x00ae_a150);
    }

    #[test]
    fn fn_00aea100_counts_unless_paused() {
        let mut e = Engine::new();
        e.register(GET_TICK_COUNT, |_, _| 1500u32.into_ret());
        let timer = e.mem.alloc(12);
        e.mem.set_u32(timer, 100);
        e.mem.set_u32(timer + 4, 1000);
        e.call(0x00ae_a100, &args![timer]);
        assert_eq!(e.mem.u32(timer), 600);
        assert_eq!(e.mem.u32(timer + 4), 1500);
        // Paused (bit 0 of +8): only the last tick moves.
        e.mem.set_u32(timer + 4, 1000);
        e.mem.set_u32(timer + 8, 1);
        e.call(0x00ae_a100, &args![timer]);
        assert_eq!(e.mem.u32(timer), 600);
        assert_eq!(e.mem.u32(timer + 4), 1500);
    }

    #[test]
    fn fn_00aea1b0_constructs_the_win32_audio_object() {
        let mut e = Engine::new();
        e.register(AUDIO_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        let this = e.mem.alloc(0xa4);
        e.mem.write(this, &[0xcd; 0xa4]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_a1b0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, AUDIO_BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x010a_39d8);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u32(this + 0xa0), 0);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![this + 0x40, 0, 0x60]]);
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    #[test]
    fn fn_00aea180_seeks_with_the_low_offset_or_refuses_null() {
        let mut e = Engine::new();
        e.register(FSEEK, |_, _| 5u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ae_a180, &args![0u32, 1u32, 2u32, 0u32]).i32(),
            -1
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(
            e.call(0x00ae_a180, &args![0x5000u32, 0x40u32, 0x99u32, 1u32])
                .i32(),
            5
        );
        let seeks: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == FSEEK)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(seeks, vec![vec![0x5000, 0x40, 1]]);
    }
}
