//! `fallout shared/run_0067da20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A 0x10-byte block (`PackageEventAction` keeps one; `PackageDataPatrol`
//! holds one at +4): words at +0, +8 and +0xc, and at +4 an object deleted
//! through its virtual slot `0x10`.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0067da40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes the four words (+0, +8, +0xc, +4, in that order). Returns `this`.
/// The engine map gives it no name.
pub fn fn_0067da40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0x8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x4, 0);
    this
}

// Translated from 0067da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes +0 and +8 and deletes the object at +4 (its virtual slot `0x10`
/// with 1) when there is one; +4 itself is left as it is. The engine map
/// gives it no name.
pub fn fn_0067da80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0x8, 0);
    let object = e.mem.u32(base + 0x4);
    if object != 0 {
        e.vcall(object, 0x10, &args![1u32]);
    }
}

// Translated from 0067da20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at +4 (`RET 4`; the stack word is not read). The engine
/// map gives it no name.
pub fn fn_0067da20(e: &mut Engine, this: Ptr, _unused_1: u32) {
    e.mem.set_u32(this.addr() + 4, 0);
}

// Translated from 0067dad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067dad0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated
/// Copies another block (`RET 4`; the engine map gives it no name): the
/// words at +0 (`00559450`, `00537e90`), +8 (`0044ddc0`, `00403550`) and
/// +0xC (`0084e3a0`, `0041fd00`); then, when the other has an object at
/// +4, this one makes its own when it has none (a new 0x54-byte object,
/// `00401000` and `005aa0f0`, or 0 when the allocation fails, then
/// `00484490` on it) and copies the other's into it (`005aa420`); when the
/// other has none, this one's object is deleted (virtual slot 0x10 with 1)
/// and +4 cleared.
pub fn fn_0067dad0(e: &mut Engine, this: Ptr, other: Ptr) {
    let base = this.addr();
    let word = e.call(0x0055_9450, &args![other]).u32();
    e.call(0x0053_7e90, &args![this, word]);
    let word = e.call(0x0044_ddc0, &args![other]).u32();
    e.call(0x0040_3550, &args![this, word]);
    let word = e.call(0x0084_e3a0, &args![other]).u32();
    e.call(0x0041_fd00, &args![this, word]);
    if e.mem.u32(other.addr() + 4) != 0 {
        if e.mem.u32(base + 4) == 0 {
            let block = e.call(0x0040_1000, &args![0x54u32]).u32();
            let object = if block != 0 {
                e.call(0x005a_a0f0, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(base + 4, object);
            let object = e.mem.u32(base + 4);
            e.call(0x0048_4490, &args![object]);
        }
        let theirs = e.mem.u32(other.addr() + 4);
        let mine = e.mem.u32(base + 4);
        e.call(0x005a_a420, &args![mine, theirs]);
    } else {
        let object = e.mem.u32(base + 4);
        if object != 0 {
            e.vcall(object, 0x10, &args![1u32]);
        }
        e.mem.set_u32(base + 4, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_da40, fn_0067da40(Ptr) -> Ptr),
        entry!(0x0067_da80, fn_0067da80(Ptr)),
        entry!(0x0067_da20, fn_0067da20(Ptr, u32)),
        entry!(0x0067_dad0, fn_0067dad0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0067da40_zeroes_the_block() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0067_da40, &args![this]).u32(), this);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0, 0]);
    }

    fn deletes(e: &mut Engine, this: u32) -> Vec<Vec<u32>> {
        e.call_log = Some(vec![]);
        e.call(0x0067_da80, &args![this]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0f00_1000)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn at_0067da80_deletes_the_object() {
        let mut e = Engine::new();
        e.register(0x0f00_1000, |_, _| Ret::default());
        e.put_vtable(0x0f00_0000, &[0, 0, 0, 0, 0x0f00_1000]);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0f00_0000);
        let this = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(this + 4 * i, 0x77);
        }
        e.mem.set_u32(this + 4, object);
        assert_eq!(deletes(&mut e, this), vec![vec![object, 1]]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, object, 0, 0x77]);
        e.mem.set_u32(this + 4, 0);
        assert!(deletes(&mut e, this).is_empty());
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for getter in [0x0055_9450u32, 0x0044_ddc0, 0x0084_e3a0] {
            e.register(getter, |_, a| (a[0] + 0x10).into_ret());
        }
        for addr in [
            0x0053_7e90,
            0x0040_3550,
            0x0041_fd00,
            0x0048_4490,
            0x005a_a420,
            0x0f00_1010,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(0x0040_1000, |_, _| 0x7000u32.into_ret());
        e.register(0x005a_a0f0, |_, a| (a[0] + 1).into_ret());
        e.put_vtable(0x0f00_0000, &[0, 0, 0, 0, 0x0f00_1010]);
        e
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn clear_word_4() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 9);
        e.call(0x0067_da20, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    #[test]
    fn copy_makes_and_copies_the_object() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other + 4, 0x6600);
        e.call_log = Some(vec![]);
        e.call(0x0067_dad0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0053_7e90), vec![vec![this, other + 0x10]]);
        assert_eq!(calls(&log, 0x0040_3550), vec![vec![this, other + 0x10]]);
        assert_eq!(calls(&log, 0x0041_fd00), vec![vec![this, other + 0x10]]);
        assert_eq!(e.mem.u32(this + 4), 0x7001);
        assert_eq!(calls(&log, 0x0048_4490), vec![vec![0x7001]]);
        assert_eq!(calls(&log, 0x005a_a420), vec![vec![0x7001, 0x6600]]);
        // With an object already: no new one.
        e.call_log = Some(vec![]);
        e.call(0x0067_dad0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x0040_1000).is_empty());
        assert_eq!(calls(&log, 0x005a_a420), vec![vec![0x7001, 0x6600]]);
    }

    #[test]
    fn copy_without_the_others_object_deletes_ours() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0f00_0000);
        e.mem.set_u32(this + 4, object);
        e.call_log = Some(vec![]);
        e.call(0x0067_dad0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0f00_1010), vec![vec![object, 1]]);
        assert_eq!(e.mem.u32(this + 4), 0);
        e.call_log = Some(vec![]);
        e.call(0x0067_dad0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, 0x0f00_1010).is_empty());
    }
}
