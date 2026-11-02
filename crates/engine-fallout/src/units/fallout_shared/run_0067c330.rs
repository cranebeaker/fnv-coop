//! `fallout shared/run_0067c330` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0067c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `010689e4` (RTTI `.?AVTESPatrolPackageData@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `0067ad70`. One word, plain `RET`, returns nothing.
pub fn fn_0067c490(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_89e4);
    e.call(0x0067_ad70, &args![this]);
}

// Translated from 0067c3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPatrolPackageData` default constructor (the class of the vtable
/// `010689e4` it stores, by its RTTI): the `TESPackageData` constructor
/// (`0067ad20`), the vtable, the byte +0x4 = 1 and +0x5 = 0. Returns `this`.
pub fn fn_0067c3f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0067_ad20, &args![this]);
    e.mem.set_u32(this.addr(), 0x0106_89e4);
    e.mem.set_u8(this.addr() + 4, 1);
    e.mem.set_u8(this.addr() + 5, 0);
    this
}

// Translated from 0067c420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPatrolPackageData` scalar deleting destructor: the destructor body
/// [`fn_0067c490`], then the release `00401030` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_0067c420(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0067c490(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0067c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPatrolPackageData` constructor from a 2-byte value passed in one
/// stack word: the `TESPackageData` constructor (`0067ad20`), the vtable
/// `010689e4`, then the word's low byte into +0x4 and its second byte into
/// +0x5. Returns `this`.
pub fn fn_0067c450(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(0x0067_ad20, &args![this]);
    e.mem.set_u32(this.addr(), 0x0106_89e4);
    e.mem.set_u8(this.addr() + 4, value as u8);
    e.mem.set_u8(this.addr() + 5, (value >> 8) as u8);
    this
}

// Translated from 0067c330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067c330 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated
/// Loads from the load game buffer (`RET 4`; the engine map gives it no
/// name): the word at +8 (`00864980`, 4 bytes), then a byte; when set, a
/// new 0xC-byte object (`00401000`, constructed with `0067f030`, or 0 when
/// the allocation fails) is stored at +4 and loads itself (`0067fbe0` with
/// the buffer).
pub fn fn_0067c330(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0086_4980, &args![buffer, this.addr() + 8, 4u32]);
    let present = e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), 0);
        e.call(0x0086_4980, &args![buffer, flag, 1u32]);
        e.mem.u8(flag.addr())
    });
    if present == 0 {
        return;
    }
    let block = e.call(0x0040_1000, &args![0xcu32]).u32();
    let object = if block != 0 {
        e.call(0x0067_f030, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(this.addr() + 4, object);
    let object = e.mem.u32(this.addr() + 4);
    e.call(0x0067_fbe0, &args![object, buffer]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_c490, fn_0067c490(Ptr)),
        entry!(0x0067_c3f0, fn_0067c3f0(Ptr) -> Ptr),
        entry!(0x0067_c420, fn_0067c420(Ptr, u32) -> Ptr),
        entry!(0x0067_c450, fn_0067c450(Ptr, u32) -> Ptr),
        entry!(0x0067_c330, fn_0067c330(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for callee in [0x0067_ad20, 0x0067_ad70, 0x0040_1030] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn fn_0067c3f0_sets_the_defaults() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcc; 8]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_c3f0, &args![this]).u32(), this);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x0067_ad20, vec![this])));
        assert_eq!(e.mem.u32(this), 0x0106_89e4);
        assert_eq!(e.mem.bytes(this + 4, 4), vec![1, 0, 0xcc, 0xcc]);
    }

    #[test]
    fn fn_0067c450_takes_the_two_bytes_of_the_word() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcc; 8]);
        assert_eq!(
            e.call(0x0067_c450, &args![this, 0x7766_0201u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), 0x0106_89e4);
        assert_eq!(e.mem.bytes(this + 4, 4), vec![1, 2, 0xcc, 0xcc]);
    }

    #[test]
    fn fn_0067c420_destroys_and_releases_with_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        for flags in [0u32, 1, 2, 3] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0067_c420, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert!(log.contains(&(0x0067_ad70, vec![this])));
            let releases = log.iter().filter(|c| c.0 == 0x0040_1030).count();
            assert_eq!(releases, (flags & 1) as usize);
            assert_eq!(e.mem.u32(this), 0x0106_89e4);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_0067c490_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0067_ad70, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0067_c490, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0067_ad70)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_89e4);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_89e4,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn load_reads_the_word_then_the_object() {
        let mut e = Engine::new();
        e.register(0x0086_4980, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], 0x1234);
            } else {
                e.mem.set_u8(a[1], 1);
            }
            Ret::default()
        });
        e.register(0x0040_1000, |_, _| 0x7000u32.into_ret());
        e.register(0x0067_f030, |_, a| (a[0] + 1).into_ret());
        e.register(0x0067_fbe0, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x0067_c330, &args![this, 0x8800u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0086_4980, vec![0x8800, this + 8, 4]));
        assert_eq!(e.mem.u32(this + 8), 0x1234);
        assert_eq!(e.mem.u32(this + 4), 0x7001);
        assert_eq!(log.last().unwrap(), &(0x0067_fbe0, vec![0x7001, 0x8800]));
        e.register(0x0086_4980, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_c330, &args![this, 0x8800u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }
}
