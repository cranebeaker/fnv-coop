//! `fallout/audio/run_0082f140` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `LeaveCriticalSection` (import slot).
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

// Translated from 0082f1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: leaves the critical section at `this`
/// (`LeaveCriticalSection(this)`).
pub fn fn_0082f1f0(e: &mut Engine, this: Ptr) {
    e.call(LEAVE_CRITICAL_SECTION, &args![this]);
}

/// The function `fn_0082f190` calls on the member (`0082f1b0`).
const CALLEE_0082F1B0: u32 = 0x0082_f1b0;

/// The function `fn_0082f1d0` calls on the member (`0082f1f0`).
const CALLEE_0082F1F0: u32 = 0x0082_f1f0;

// Translated from 0082f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0082f1b0` with `ECX` = `this + 0x10c` and passes on nothing (the callee returns no value).
pub fn fn_0082f190(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0082F1B0, &args![this.byte_add(0x10c)]);
}

// Translated from 0082f1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0082f1f0` with `ECX` = `this + 0x10c` and passes on nothing (the callee returns no value).
pub fn fn_0082f1d0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0082F1F0, &args![this.byte_add(0x10c)]);
}

/// `BSThread::BSThread` (engine map; `thiscall`, no stack words).
const BS_THREAD_CONSTRUCT: u32 = 0x00aa_6360;
/// The vtable `0107cbe8` (RTTI `.?AVFAMThread@?A0x94d3b31f@@`, a class in an anonymous
/// namespace).
const FAM_THREAD_VTABLE: u32 = 0x0107_cbe8;
/// `0096a2d0`, run on the member at +0x34 (`thiscall`, no stack words; unnamed in the engine
/// map).
const MEMBER_34_CALL: u32 = 0x0096_a2d0;
/// `CreateEventA` (import slot).
const CREATE_EVENT_A: u32 = 0x00fd_f0cc;
/// `0059bb30` (`thiscall`, no stack words): answers the word at +0x24 of its object. The engine
/// map's name for it (`D3DTexture_LockRect`) does not fit that body.
const READ_WORD_24: u32 = 0x0059_bb30;

// Translated from 0082f610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of `FAMThread` (by the vtable it stores; the
/// element constructor `falloutaudiomedia.rs` hands to the array constructor `00ec782f`,
/// 0x4c-byte elements). `thiscall`, no stack words (plain `RET`). Runs `BSThread`'s
/// constructor, stores the vtable, runs `0096a2d0` on +0x34, sets +0x3c to 1, stores
/// `CreateEventA(NULL, FALSE, FALSE, NULL)` at +0x30, clears +0x40 and +0x44, and stores what
/// `0059bb30` answers for `this` (its word at +0x24) at +0x48. Answers `this`.
// Translation incomplete: 0082f610 - C++ exception unwinding (SEH frame state: the BSThread destructor on a throw after its constructor) is not translated.
pub fn fn_0082f610(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.call(BS_THREAD_CONSTRUCT, &args![this]);
    e.mem.set_u32(at, FAM_THREAD_VTABLE);
    e.call(MEMBER_34_CALL, &args![at + 0x34]);
    e.mem.set_u32(at + 0x3c, 1);
    let event = e.call(CREATE_EVENT_A, &args![0u32, 0u32, 0u32, 0u32]).u32();
    e.mem.set_u32(at + 0x30, event);
    e.mem.set_u32(at + 0x40, 0);
    e.mem.set_u32(at + 0x44, 0);
    let word = e.call(READ_WORD_24, &args![this]).u32();
    e.mem.set_u32(at + 0x48, word);
    this
}

// Translated from 0082f160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): the scalar deleting destructor of the class
/// [`fn_0082f140`] destroys, `thiscall` with the flags (`RET 4`): the destructor, then with bit 0
/// set the object freed (`00401030`). Answers `this`.
pub fn fn_0082f160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0082f140(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082f1f0, fn_0082f1f0(Ptr)),
        entry!(0x0082_f190, fn_0082f190(Ptr)),
        entry!(0x0082_f1d0, fn_0082f1d0(Ptr)),
        entry!(0x0082_f610, fn_0082f610(Ptr) -> Ptr),
        entry!(0x0082_f140, fn_0082f140(Ptr)),
        entry!(0x0082_f1b0, fn_0082f1b0(Ptr)),
        entry!(0x0082_f210, fn_0082f210(Ptr, u32) -> Ptr),
        entry!(0x0082_f240, fn_0082f240(Ptr) -> Ptr),
        entry!(0x0082_f260, fn_0082f260(Ptr)),
        entry!(0x0082_f310, fn_0082f310(Ptr, u32, i32) -> u32),
        entry!(0x0082_f3c0, fn_0082f3c0(Ptr, u32, u32, u32, Ptr) -> u32),
        entry!(0x0082_f520, fn_0082f520(Ptr, u32) -> Ptr),
        entry!(0x0082_f550, fn_0082f550(Ptr)),
        entry!(0x0082_f570, fn_0082f570(Ptr, Ptr) -> u64),
        entry!(0x0082_f160, fn_0082f160(Ptr, u32) -> Ptr),
    ]
}

/// The base table both `0082f140` and `0082f240` store (`0107cb6c`).
const BASE_VTABLE_0107CB6C: u32 = 0x0107_cb6c;

// Translated from 0082f140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): a base destructor, `thiscall`, no stack words:
/// stores the base table `0107cb6c`.
pub fn fn_0082f140(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BASE_VTABLE_0107CB6C);
}

/// `EnterCriticalSection` (import slot).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;

// Translated from 0082f1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): enters the critical section at `this`
/// (`EnterCriticalSection(this)`).
pub fn fn_0082f1b0(e: &mut Engine, this: Ptr) {
    e.call(ENTER_CRITICAL_SECTION, &args![this]);
}

/// `operator delete` (`00401030`, `cdecl`, the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 0082f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): the scalar deleting destructor of the class
/// `0082f260` destroys, `thiscall` with the flags: the destructor, then with bit 0 set the
/// object freed (`00401030`). Answers `this`.
pub fn fn_0082f210(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0082f260(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0082f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): a base constructor, `thiscall`, no stack
/// words: stores the base table `0107cb6c` and answers `this`.
pub fn fn_0082f240(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), BASE_VTABLE_0107CB6C);
    this
}

/// The class's table (`0107cb4c`) and the member destructor `0044fb00` (unnamed; `thiscall`,
/// no stack words) run on +0x10c.
const VTABLE_0107CB4C: u32 = 0x0107_cb4c;
const MEMBER_DESTRUCT_0044FB00: u32 = 0x0044_fb00;

// Translated from 0082f260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): a destructor, `thiscall`, no stack words.
/// Stores the class's table `0107cb4c`; the object at +0x108, when set, is deleted (its slot 0
/// with 1); the member at +0x10c is destroyed (`0044fb00`) and the base destructor `0082f140`
/// runs.
// Translation incomplete: 0082f260 - C++ exception unwinding (the FS:[0] frame whose states run the member and base destructors on a throw) is not translated.
pub fn fn_0082f260(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_0107CB4C);
    let object = e.mem.u32(base.wrapping_add(0x108));
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.call(MEMBER_DESTRUCT_0044FB00, &args![base.wrapping_add(0x10c)]);
    fn_0082f140(e, this);
}

/// The stream's ready check `008905f0` (unit `actor.cpp` in the engine map; `thiscall`, no
/// stack words; answers a byte), the read `00462d80` (unit `tesdatahandler.cpp`; `thiscall`:
/// buffer and size) and the word `0082f310` passes to the stream's seek (`010a2480`).
const STREAM_READY_008905F0: u32 = 0x0089_05f0;
const STREAM_READ_00462D80: u32 = 0x0046_2d80;
const SEEK_ARGUMENT_010A2480: u32 = 0x010a_2480;

/// Whether the stream at +0x108 is there and ready (`008905f0`).
fn stream_ready(e: &mut Engine, this: u32) -> bool {
    let stream = e.mem.u32(this.wrapping_add(0x108));
    stream != 0 && e.call(STREAM_READY_008905F0, &args![stream]).bool()
}

// Translated from 0082f310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): a seek, `thiscall` with a 64-bit position (low
/// word, then the signed high word). Answers 1 for a negative position, one past the length
/// (+0x128, high +0x12c, compared as signed 64 bits on the high word) or without a ready
/// stream (+0x108, `008905f0`); otherwise keeps the position at +0x130/+0x134, seeks the stream
/// (its slot 0x14, `thiscall`: the low word and the word at `010a2480`) and answers 0.
pub fn fn_0082f310(e: &mut Engine, this: Ptr, low: u32, high: i32) -> u32 {
    let base = this.addr();
    if high < 0 {
        return 1;
    }
    let length_high = e.mem.i32(base.wrapping_add(0x12c));
    if high > length_high || (high == length_high && low > e.mem.u32(base.wrapping_add(0x128))) {
        return 1;
    }
    if !stream_ready(e, base) {
        return 1;
    }
    e.mem.set_u32(base.wrapping_add(0x130), low);
    e.mem.set_u32(base.wrapping_add(0x134), high as u32);
    let word = e.mem.u32(SEEK_ARGUMENT_010A2480);
    let stream = e.mem.u32(base.wrapping_add(0x108));
    e.vcall(stream, 0x14, &args![low, word]);
    0
}

// Translated from 0082f3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): a read, `thiscall` with four words: the
/// buffer, the size, a word never read and the address of the result. Holds the lock at
/// +0x10c for its duration (the guard `0082f520`/`0082f550` on a stack word). Without a ready
/// stream answers 1. Otherwise the stream reads (`00462d80` with the buffer and the size) into
/// the result word, and the position (+0x130/+0x134) moves on by the size, or by what is left
/// to the length (+0x128) when the size would pass it (low words only); answers 0.
// Translation incomplete: 0082f3c0 - C++ exception unwinding (the FS:[0] frame whose state releases the lock guard on a throw) is not translated.
pub fn fn_0082f3c0(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
    _unused_3: u32,
    result: Ptr,
) -> u32 {
    let base = this.addr();
    e.with_stack(4, |e, guard| {
        fn_0082f520(e, guard, base.wrapping_add(0x10c));
        if !stream_ready(e, base) {
            fn_0082f550(e, guard);
            return 1;
        }
        let position = e.mem.u32(base.wrapping_add(0x130));
        let (end_low, carry) = size.overflowing_add(position);
        let end_high = e
            .mem
            .u32(base.wrapping_add(0x134))
            .wrapping_add(u32::from(carry)) as i32;
        let length_high = e.mem.i32(base.wrapping_add(0x12c));
        let fits = end_high < length_high
            || (end_high == length_high && end_low <= e.mem.u32(base.wrapping_add(0x128)));
        let step = if fits {
            size
        } else {
            e.mem
                .u32(base.wrapping_add(0x128))
                .wrapping_sub(e.mem.u32(base.wrapping_add(0x130)))
        };
        let stream = e.mem.u32(base.wrapping_add(0x108));
        let read = e
            .call(STREAM_READ_00462D80, &args![stream, buffer, size])
            .u32();
        e.mem.set_u32(result.addr(), read);
        let position = e.mem.u32(base.wrapping_add(0x130));
        let (low, carry) = step.overflowing_add(position);
        let high = e
            .mem
            .u32(base.wrapping_add(0x134))
            .wrapping_add(u32::from(carry));
        e.mem.set_u32(base.wrapping_add(0x130), low);
        e.mem.set_u32(base.wrapping_add(0x134), high);
        fn_0082f550(e, guard);
        0
    })
}

// Translated from 0082f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): a lock guard's constructor, `thiscall` with the
/// critical section: keeps it at +0 and enters it (`0082f1b0`). Answers `this`.
pub fn fn_0082f520(e: &mut Engine, this: Ptr, section: u32) -> Ptr {
    e.mem.set_u32(this.addr(), section);
    let section = e.mem.u32(this.addr());
    fn_0082f1b0(e, Ptr::new(section));
    this
}

// Translated from 0082f550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): the lock guard's destructor, `thiscall`, no
/// stack words: leaves the critical section it keeps at +0 (`0082f1f0`).
pub fn fn_0082f550(e: &mut Engine, this: Ptr) {
    let section = e.mem.u32(this.addr());
    fn_0082f1f0(e, Ptr::new(section));
}

// Translated from 0082f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/audio`): the length, `thiscall` with the address of a
/// 64-bit result. With a ready stream (+0x108, `008905f0`) the length becomes what the stream's
/// slot 0x1c answers (low word +0x128, high +0x12c cleared) and is written to the result and
/// answered (EDX:EAX); otherwise the result and the answer are 0.
pub fn fn_0082f570(e: &mut Engine, this: Ptr, result: Ptr) -> u64 {
    let base = this.addr();
    if !stream_ready(e, base) {
        e.mem.set_u32(result.addr(), 0);
        e.mem.set_u32(result.addr().wrapping_add(4), 0);
        return 0;
    }
    let stream = e.mem.u32(base.wrapping_add(0x108));
    let length = e.vcall(stream, 0x1c, &args![]).u32();
    e.mem.set_u32(base.wrapping_add(0x128), length);
    e.mem.set_u32(base.wrapping_add(0x12c), 0);
    let low = e.mem.u32(base.wrapping_add(0x128));
    e.mem.set_u32(result.addr(), low);
    let high = e.mem.u32(base.wrapping_add(0x12c));
    e.mem.set_u32(result.addr().wrapping_add(4), high);
    let low = e.mem.u32(base.wrapping_add(0x128));
    let high = e.mem.u32(base.wrapping_add(0x12c));
    u64::from(low) | (u64::from(high) << 32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaves_the_section_at_this() {
        let mut e = Engine::new();
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082f1f0, &args![0x1234u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (LEAVE_CRITICAL_SECTION, vec![0x1234])
        );
    }

    #[test]
    fn l68d_0082f190_passes_this_plus_10c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0082F1B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_f190, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0082F1B0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x10c]);
    }

    #[test]
    fn l68d_0082f1d0_passes_this_plus_10c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0082F1F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_f1d0, &args![0x4200u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0082F1F0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x10c]);
    }
}

#[cfg(test)]
mod b000o_tests {
    use super::*;

    #[test]
    fn fn_0082f610_constructs_the_thread_object() {
        let mut e = Engine::new();
        e.register(BS_THREAD_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x24, 0x9999);
            a[0].into_ret()
        });
        e.register(MEMBER_34_CALL, |e, a| {
            // Runs after the vtable is stored.
            assert_eq!(e.mem.u32(a[0] - 0x34), FAM_THREAD_VTABLE);
            a[0].into_ret()
        });
        e.register(CREATE_EVENT_A, |_, _| 0x0abcu32.into_ret());
        e.register(READ_WORD_24, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        let this = e.mem.alloc(0x4c);
        e.mem.write(this, &[0xcd; 0x4c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_f610, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                BS_THREAD_CONSTRUCT,
                MEMBER_34_CALL,
                CREATE_EVENT_A,
                READ_WORD_24
            ]
        );
        let args_of = |addr: u32| log.iter().find(|c| c.0 == addr).unwrap().1.clone();
        assert_eq!(args_of(BS_THREAD_CONSTRUCT), vec![this]);
        assert_eq!(args_of(MEMBER_34_CALL), vec![this + 0x34]);
        assert_eq!(args_of(CREATE_EVENT_A), vec![0, 0, 0, 0]);
        assert_eq!(args_of(READ_WORD_24), vec![this]);
        assert_eq!(e.mem.u32(this), FAM_THREAD_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0x0abc);
        assert_eq!(e.mem.u32(this + 0x3c), 1);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0x9999);
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    const DELETE: u32 = 0x0ed0_0000;
    const SEEK: u32 = 0x0ed0_0014;
    const LENGTH: u32 = 0x0ed0_001c;
    const TABLE: u32 = 0x0ed1_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object with a stream at +0x108 (ready as `ready` says), length 0x100 and position
    /// 0xf0.
    fn rig(ready: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x010a_2000, 0x1000);
        e.mem.set_u32(SEEK_ARGUMENT_010A2480, 0x2480);
        let mut slots = vec![0u32; 8];
        slots[0] = DELETE;
        slots[5] = SEEK;
        slots[7] = LENGTH;
        e.put_vtable(TABLE, &slots);
        for no_op in [
            DELETE,
            SEEK,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            MEMBER_DESTRUCT_0044FB00,
            OPERATOR_DELETE,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(LENGTH, |_, _| 0x300u32.into_ret());
        e.register(STREAM_READ_00462D80, |_, a| (a[2] + 1).into_ret());
        if ready {
            e.register(STREAM_READY_008905F0, |_, _| 1u32.into_ret());
        } else {
            e.register(STREAM_READY_008905F0, |_, _| 0u32.into_ret());
        }
        let stream = e.mem.alloc(4);
        e.mem.set_u32(stream, TABLE);
        let this = e.mem.alloc(0x138);
        e.mem.set_u32(this + 0x108, stream);
        e.mem.set_u32(this + 0x128, 0x100);
        e.mem.set_u32(this + 0x130, 0xf0);
        (e, this, stream)
    }

    #[test]
    fn base_tables_and_the_lock() {
        let (mut e, this, _) = rig(true);
        e.call(0x0082_f140, &args![this]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE_0107CB6C);
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x0082_f240, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE_0107CB6C);
        let guard = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_f520, &args![guard, 0x5ec0u32]).u32(), guard);
        e.call(0x0082_f550, &args![guard]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![0x5ec0]]);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![0x5ec0]]);
        assert_eq!(e.mem.u32(guard), 0x5ec0);
        e.call_log = Some(vec![]);
        e.call(0x0082_f1b0, &args![0x77u32]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![0x77]]);
    }

    #[test]
    fn destructors_delete_the_stream() {
        let (mut e, this, stream) = rig(true);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_f210, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, DELETE), vec![vec![stream, 1]]);
        assert_eq!(
            calls(&e, MEMBER_DESTRUCT_0044FB00),
            vec![vec![this + 0x10c]]
        );
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE_0107CB6C);
        e.mem.set_u32(this + 0x108, 0);
        e.call_log = Some(vec![]);
        e.call(0x0082_f210, &args![this, 0u32]);
        assert!(calls(&e, DELETE).is_empty());
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn seek_checks_the_range() {
        let (mut e, this, stream) = rig(true);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_f310, &args![this, 0x80u32, 0i32]).u32(), 0);
        assert_eq!(calls(&e, SEEK), vec![vec![stream, 0x80, 0x2480]]);
        assert_eq!(e.mem.u32(this + 0x130), 0x80);
        assert_eq!(e.mem.u32(this + 0x134), 0);
        assert_eq!(e.call(0x0082_f310, &args![this, 0x100u32, 0i32]).u32(), 0);
        assert_eq!(e.call(0x0082_f310, &args![this, 0x101u32, 0i32]).u32(), 1);
        assert_eq!(e.call(0x0082_f310, &args![this, 0u32, -1i32]).u32(), 1);
        assert_eq!(e.call(0x0082_f310, &args![this, 0u32, 1i32]).u32(), 1);
        let (mut e, this, _) = rig(false);
        assert_eq!(e.call(0x0082_f310, &args![this, 0u32, 0i32]).u32(), 1);
    }

    #[test]
    fn read_moves_the_position_up_to_the_length() {
        let (mut e, this, stream) = rig(true);
        let result = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0082_f3c0, &args![this, 0xb000u32, 8u32, 0u32, result])
                .u32(),
            0
        );
        assert_eq!(
            calls(&e, STREAM_READ_00462D80),
            vec![vec![stream, 0xb000, 8]]
        );
        assert_eq!(e.mem.u32(result), 9);
        assert_eq!(e.mem.u32(this + 0x130), 0xf8);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION), vec![vec![this + 0x10c]]);
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION), vec![vec![this + 0x10c]]);
        // Past the length: only what is left.
        e.call(0x0082_f3c0, &args![this, 0xb000u32, 0x20u32, 0u32, result]);
        assert_eq!(e.mem.u32(result), 0x21);
        assert_eq!(e.mem.u32(this + 0x130), 0x100);
        // Not ready: 1, the lock still left.
        let (mut e, this, _) = rig(false);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0082_f3c0, &args![this, 0xb000u32, 8u32, 0u32, result])
                .u32(),
            1
        );
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 1);
        assert!(calls(&e, STREAM_READ_00462D80).is_empty());
    }

    #[test]
    fn length_asks_the_stream() {
        let (mut e, this, _) = rig(true);
        let result = e.mem.alloc(8);
        e.mem.set_u32(this + 0x12c, 5);
        assert_eq!(e.call(0x0082_f570, &args![this, result]).u64(), 0x300);
        assert_eq!(e.mem.u64(result), 0x300);
        assert_eq!(e.mem.u32(this + 0x128), 0x300);
        assert_eq!(e.mem.u32(this + 0x12c), 0);
        let (mut e, this, _) = rig(false);
        let result = e.mem.alloc(8);
        e.mem.set_u64(result, 7);
        assert_eq!(e.call(0x0082_f570, &args![this, result]).u64(), 0);
        assert_eq!(e.mem.u64(result), 0);
    }
}

#[cfg(test)]
mod n089_tests {
    use super::*;

    #[test]
    fn n089_0082f160_destroys_and_frees_on_bit_0() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_f160, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE_0107CB6C);
        let deletes = |e: &Engine| {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == OPERATOR_DELETE)
                .count()
        };
        assert_eq!(deletes(&e), 0);
        e.call_log = Some(vec![]);
        e.call(0x0082_f160, &args![this, 1u32]);
        assert_eq!(deletes(&e), 1);
    }
}
