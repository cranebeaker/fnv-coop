//! `Havok SDK/run_00df8910` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00df8930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0xc` (`SUB ECX,0xc`) and jumps to `00df9470`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00df8930(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_9470, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00df8940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x4` (`SUB ECX,0x4`) and jumps to `00df9470`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00df8940(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_9470, &args![this.addr().wrapping_sub(0x4), flags])
        .u32()
}

// Translated from 00df8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: copies the byte at `this + 0x14` to the byte at `out` and returns
/// `out` (`EAX`). The class is unconfirmed.
pub fn fn_00df8910(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let value = e.mem.u8(this.addr() + 0x14);
    e.mem.set_u8(out.addr(), value);
    out
}

// Translated from 00df8920 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: returns (`FLD`) the float at `this + 0x10`. The class is
/// unconfirmed.
pub fn fn_00df8920(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x10))
}

// Translated from 00df8950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `8` (`SUB ECX,0x8`) and jumps to `00df9470`, which (`RET 4`) is
/// the scalar deleting destructor the other thunks of this unit share (the decompiler shows its
/// body: four vtable words and a `0x80`-byte free when bit 0 of the flags is set). The caller
/// gets its `EAX`; the class is unconfirmed.
pub fn fn_00df8950(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_9470, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00df8800 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a job function: `imm 00da34db` in the gap list): cdecl, two stack words
/// (nothing popped): `owner` and `job`. Appends the timer record `010dbab0`. On a stack frame it
/// builds an array record A (data = the word at `job + 0x50`, size = the word at `job + 0x5c`,
/// capacity word = that size with the top bit set: storage that is not owned) and an object B that
/// `00d9df50` initialises from A (with 0), calls `00d9dab0` on B with the words at `job + 0x58`,
/// `+0x5c` and `+0x54`, then runs `00d04e80` on `owner` with (`job`, `job`, 0). B's first word is
/// set to `0102e388`; A is destroyed (size set to 0; when its capacity word is not negative its
/// storage, `(capacity & 0x3fffffff) * 4` bytes, is freed through the memory router's allocator,
/// virtual function at +0x10 of the object at router + 0x10), its data word cleared and its
/// capacity word set to `0x80000000`. Appends the timer record `010c41f8` and returns the answer
/// of `00d04e80`. Ghidra made no function here; the address is an immediate operand at 00da34db.
pub fn fn_00df8800(e: &mut Engine, owner: u32, job: u32) -> u32 {
    crate::units::havok_sdk::hkpworld::profile_record(e, 0x010d_bab0);
    // The exe's frame: A at +4 (data, size, capacity), B at +0x10.
    e.with_stack(0x1c, |e, frame| {
        let array = frame.addr() + 4;
        let object = frame.addr() + 0x10;
        let data = e.mem.u32(job + 0x50);
        e.mem.set_u32(array, data);
        let size = e.mem.u32(job + 0x5c);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, size | 0x8000_0000);
        e.call(0x00d9_df50, &args![object, array, 0u32]);
        let first = e.mem.u32(job + 0x58);
        let second = e.mem.u32(job + 0x5c);
        let third = e.mem.u32(job + 0x54);
        e.call(0x00d9_dab0, &args![object, first, second, third]);
        let result = e.call(0x00d0_4e80, &args![owner, job, job, 0u32]).u32();
        e.mem.set_u32(object, 0x0102_e388);
        let router = e.call(0x00c8_5750, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let storage = e.mem.u32(array);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![storage, bytes]);
        }
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        crate::units::havok_sdk::hkpworld::profile_record(e, 0x010c_41f8);
        result
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_8930, fn_00df8930(Ptr, u32) -> u32),
        entry!(0x00df_8940, fn_00df8940(Ptr, u32) -> u32),
        entry!(0x00df_8910, fn_00df8910(Ptr, Ptr) -> Ptr),
        entry!(0x00df_8920, fn_00df8920(Ptr) -> f32),
        entry!(0x00df_8950, fn_00df8950(Ptr, u32) -> u32),
        entry!(0x00df_8800, fn_00df8800(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00df8930() {
        assert_eq!(check(0x00df_8930, 0x00df_9470, 0xc), 0xdead_beef);
    }

    #[test]
    fn thunk_00df8940() {
        assert_eq!(check(0x00df_8940, 0x00df_9470, 0x4), 0xdead_beef);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// The byte at +0x14 is copied to `out`, which is returned.
    #[test]
    fn fn_00df8910_copies_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let out = e.mem.alloc(4);
        e.mem.set_u8(this + 0x14, 0x5a);
        e.mem.set_u8(out, 0xaa);
        assert_eq!(e.call(0x00df_8910, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 0x5a);
    }

    /// The float at +0x10 comes back; a signalling NaN comes back quiet.
    #[test]
    fn fn_00df8920_returns_the_float() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this + 0x10, -4.25);
        assert_eq!(e.call(0x00df_8920, &args![this]).f32(), -4.25);
        e.mem.set_u32(this + 0x10, 0xff80_0007);
        let bits = e.call(0x00df_8920, &args![this]).f32().to_bits();
        assert_eq!(bits & 0x7fff_ffff, 0x7fc0_0007);
    }

    /// The thunk moves `this` back by 8 and passes the flags on; the callee's `EAX` comes back.
    #[test]
    fn fn_00df8950_moves_this_and_jumps() {
        let mut e = Engine::new();
        e.register(0x00df_9470, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let got = e.call(0x00df_8950, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00df_9470).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - 8, 0x1]);
        assert_eq!(got.u32(), 0xdead_beef);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::units::havok_sdk::hkpworld::profiling_support::{labels, profiling_engine};
    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Doubles for the callees; `capacity_after_init` is what the constructor double leaves in A's
    /// capacity word (it receives A as its second word). A free function double logs the free call.
    fn array_engine(capacity_after_init: Option<u32>) -> (Engine, u32, u32, Words, Words) {
        let (mut e, stream, area) = profiling_engine(2);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0009);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&frees);
        e.register_double(0x0900_0009, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let calls: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&calls);
        e.register_double(0x00d9_df50, move |e, a| {
            sink.borrow_mut().push(vec![
                0x00d9_df50,
                a[1],
                a[2],
                e.mem.u32(a[1]),
                e.mem.u32(a[1] + 4),
                e.mem.u32(a[1] + 8),
            ]);
            if let Some(capacity) = capacity_after_init {
                e.mem.set_u32(a[1] + 8, capacity);
            }
            Ret::default()
        });
        let sink = Rc::clone(&calls);
        e.register_double(0x00d9_dab0, move |_, a| {
            sink.borrow_mut().push(vec![0x00d9_dab0, a[1], a[2], a[3]]);
            Ret::default()
        });
        let sink = Rc::clone(&calls);
        e.register_double(0x00d0_4e80, move |_, a| {
            sink.borrow_mut()
                .push(vec![0x00d0_4e80, a[0], a[1], a[2], a[3]]);
            Ret {
                eax: 0x1234,
                ..Ret::default()
            }
        });
        (e, stream, area, calls, frees)
    }

    /// The job's array fields reach the helper calls and the owned storage is not freed while the top
    /// bit of the capacity word is set.
    #[test]
    fn fn_00df8800_builds_the_array_and_wraps_the_calls() {
        let (mut e, stream, area, calls, frees) = array_engine(None);
        let job = e.mem.alloc(0x80);
        e.mem.set_u32(job + 0x50, 0x9000);
        e.mem.set_u32(job + 0x54, 0xaa);
        e.mem.set_u32(job + 0x58, 0xbb);
        e.mem.set_u32(job + 0x5c, 6);
        assert_eq!(e.call(0x00df_8800, &args![0x4000u32, job]).u32(), 0x1234);
        let calls = calls.borrow();
        assert_eq!(calls[0][2..], [0, 0x9000, 6, 0x8000_0006]);
        assert_eq!(calls[1], vec![0x00d9_dab0, 0xbb, 6, 0xaa]);
        assert_eq!(calls[2], vec![0x00d0_4e80, 0x4000, job, job, 0]);
        assert!(frees.borrow().is_empty());
        assert_eq!(labels(&e, stream, area), vec![0x010d_bab0, 0x010c_41f8]);
    }

    /// When the helper leaves a non-negative capacity word, the storage is freed with its size in bytes.
    #[test]
    fn fn_00df8800_frees_owned_storage() {
        let (mut e, _, _, _, frees) = array_engine(Some(0x4000_0003));
        let job = e.mem.alloc(0x80);
        e.mem.set_u32(job + 0x50, 0x9000);
        e.mem.set_u32(job + 0x5c, 6);
        // 0x4000_0003 has the top bit clear and the flag bit 0x40000000 set: 3 elements of 4 bytes.
        e.call(0x00df_8800, &args![0x4000u32, job]);
        let frees = frees.borrow();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][1..], [0x9000, 12]);
    }
}
