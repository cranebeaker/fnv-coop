//! `Havok SDK/run_00cf6b50` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Methods of an 8-byte ownership marker (it looks like Havok's multi-thread lock check; the
//! class is unconfirmed): a word at `+0` holds the owning thread (`0xfffffff1`, `-0xf`, means
//! none), a 16-bit count at `+4` whose bit 15 means the checks are off, and a 16-bit stack of
//! mark bits at `+6`. The methods that change the marker do it inside the critical section
//! whose pointer is the global at `01268794`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The global pointer to the critical section (`01268794`).
const CRITICAL_SECTION: u32 = 0x0126_8794;
/// `EnterCriticalSection` and `LeaveCriticalSection` import slots.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `00d87a40` (no arguments): the token of the calling thread (what the marker stores at `+0`).
const THREAD_TOKEN: u32 = 0x00d8_7a40;
/// The word at `+0` that means "no owner" (`-0xf`).
const NO_OWNER: u32 = 0xffff_fff1;

/// Whether the checks are on: bit 15 of the 16-bit word at `+4` is clear.
fn checks_on(e: &Engine, base: u32) -> bool {
    e.mem.u16(base + 4) >> 15 == 0
}

// Translated from 00cf6b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the checks are on (bit 15 of `+4` clear): inside the critical section, if the owner
/// word is `-0x3f`, `-0x1f` or `-0xf` it becomes `-0x1f` when `mode` is 1 and `-0x3f`
/// otherwise; then the 16-bit word at `+6` is shifted left by one and the one at `+4` raised by
/// one. The critical section pointer is read again to leave it (the exe tail-jumps to the
/// import).
pub fn fn_00cf6b50(e: &mut Engine, this: Ptr, mode: u32) {
    let base = this.addr();
    if !checks_on(e, base) {
        return;
    }
    let owner = e.mem.u32(base);
    let section: u32 = e.global(CRITICAL_SECTION);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    if owner == 0xffff_ffc1 || owner == 0xffff_ffe1 || owner == NO_OWNER {
        e.mem
            .set_u32(base, if mode != 1 { 0xffff_ffc1 } else { 0xffff_ffe1 });
    }
    let marks = e.mem.u16(base + 6);
    e.mem.set_u16(base + 6, marks << 1);
    let count = e.mem.u16(base + 4);
    e.mem.set_u16(base + 4, count.wrapping_add(1));
    let section: u32 = e.global(CRITICAL_SECTION);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00cf6bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the checks are on: inside the critical section, the thread token (`00d87a40`) is taken,
/// the count at `+4` is raised by one, the mark word at `+6` becomes `(marks << 1) | 1`, and an
/// owner word of `-0xf` (none) becomes the token.
pub fn fn_00cf6bc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if !checks_on(e, base) {
        return;
    }
    let section: u32 = e.global(CRITICAL_SECTION);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let token = e.call(THREAD_TOKEN, &args![]).u32();
    let marks = e.mem.u16(base + 6);
    let count = e.mem.u16(base + 4);
    e.mem.set_u16(base + 4, count.wrapping_add(1));
    e.mem.set_u16(base + 6, (marks << 1) | 1);
    if e.mem.u32(base) == NO_OWNER {
        e.mem.set_u32(base, token);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00cf6c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the checks are off; otherwise, inside the critical section, whether the owner word
/// equals the calling thread token (`00d87a40`).
pub fn fn_00cf6c10(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    if !checks_on(e, base) {
        return true;
    }
    let section: u32 = e.global(CRITICAL_SECTION);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let token = e.call(THREAD_TOKEN, &args![]).u32();
    let same = e.mem.u32(base) == token;
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    same
}

/// The body shared by `00cf6c60` and `00cf6cb0` (identical code): when the checks are on,
/// inside the critical section the mark word at `+6` is shifted right by one, the count at `+4`
/// is lowered by one (16 bits) and, when it reaches zero, the owner word becomes `-0xf`.
fn release_mark(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if !checks_on(e, base) {
        return;
    }
    let section: u32 = e.global(CRITICAL_SECTION);
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let marks = e.mem.u16(base + 6);
    e.mem.set_u16(base + 6, marks >> 1);
    let count = e.mem.u16(base + 4).wrapping_add(0xffff);
    e.mem.set_u16(base + 4, count);
    if count == 0 {
        e.mem.set_u32(base, NO_OWNER);
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00cf6c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases one mark (see `release_mark`).
pub fn fn_00cf6c60(e: &mut Engine, this: Ptr) {
    release_mark(e, this);
}

// Translated from 00cf6cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The identical copy of `00cf6c60`: releases one mark (see `release_mark`).
pub fn fn_00cf6cb0(e: &mut Engine, this: Ptr) {
    release_mark(e, this);
}

// Translated from 00cf6d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns the checks off: sets bit 15 of the 16-bit word at `+4`.
pub fn fn_00cf6d20(e: &mut Engine, this: Ptr) {
    let count = e.mem.u16(this.addr() + 4);
    e.mem.set_u16(this.addr() + 4, count | 0x8000);
}

// Translated from 00cf6d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns the checks on when they are off (bit 15 of `+4` set): the owner word becomes `-0xf`
/// (none) and the 16-bit word at `+4` becomes 0. Nothing happens when they are already on.
pub fn fn_00cf6d30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if !checks_on(e, base) {
        e.mem.set_u32(base, NO_OWNER);
        e.mem.set_u16(base + 4, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_6b50, fn_00cf6b50(Ptr, u32)),
        entry!(0x00cf_6bc0, fn_00cf6bc0(Ptr)),
        entry!(0x00cf_6c10, fn_00cf6c10(Ptr) -> bool),
        entry!(0x00cf_6c60, fn_00cf6c60(Ptr)),
        entry!(0x00cf_6cb0, fn_00cf6cb0(Ptr)),
        entry!(0x00cf_6d20, fn_00cf6d20(Ptr)),
        entry!(0x00cf_6d30, fn_00cf6d30(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const SECTION: u32 = 0x0222_2000;

    /// The section global, doubles for the two imports and the thread token (`0x77`); the log
    /// records `enter`/`leave` and the section they got.
    fn rig() -> (Engine, Rc<RefCell<Vec<(&'static str, u32)>>>, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.set_global(CRITICAL_SECTION, SECTION);
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(ENTER_CRITICAL_SECTION, move |_, a| {
            sink.borrow_mut().push(("enter", a[0]));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(LEAVE_CRITICAL_SECTION, move |_, a| {
            sink.borrow_mut().push(("leave", a[0]));
            Ret::default()
        });
        e.register_double(THREAD_TOKEN, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        let marker = e.mem.alloc(0x10);
        (e, log, marker)
    }

    fn set(e: &mut Engine, marker: u32, owner: u32, count: u16, marks: u16) {
        e.mem.set_u32(marker, owner);
        e.mem.set_u16(marker + 4, count);
        e.mem.set_u16(marker + 6, marks);
    }

    fn state(e: &Engine, marker: u32) -> (u32, u16, u16) {
        (
            e.mem.u32(marker),
            e.mem.u16(marker + 4),
            e.mem.u16(marker + 6),
        )
    }

    const SECTION_CALLS: [(&str, u32); 2] = [("enter", SECTION), ("leave", SECTION)];

    /// The first mark: owner words `-0x3f`, `-0x1f` and `-0xf` are replaced by `-0x3f` or `-0x1f`
    /// by the mode, any other owner is kept; the mark word shifts and the count rises; with the
    /// checks off nothing happens, not even the critical section.
    #[test]
    fn first_marker_sets_the_owner_by_mode() {
        let (mut e, log, marker) = rig();
        set(&mut e, marker, 0xffff_fff1, 2, 0b101);
        e.call(0x00cf_6b50, &args![marker, 1u32]);
        assert_eq!(state(&e, marker), (0xffff_ffe1, 3, 0b1010));
        assert_eq!(*log.borrow(), SECTION_CALLS.to_vec());
        set(&mut e, marker, 0xffff_ffe1, 0, 0);
        e.call(0x00cf_6b50, &args![marker, 0u32]);
        assert_eq!(state(&e, marker).0, 0xffff_ffc1);
        set(&mut e, marker, 0xffff_ffc1, 0, 0);
        e.call(0x00cf_6b50, &args![marker, 7u32]);
        assert_eq!(state(&e, marker).0, 0xffff_ffc1);
        set(&mut e, marker, 0x1234, 0x7fff, 0x8001);
        e.call(0x00cf_6b50, &args![marker, 1u32]);
        assert_eq!(state(&e, marker), (0x1234, 0x8000, 0x0002));
        log.borrow_mut().clear();
        set(&mut e, marker, 5, 0x8000, 3);
        e.call(0x00cf_6b50, &args![marker, 1u32]);
        assert_eq!(state(&e, marker), (5, 0x8000, 3));
        assert!(log.borrow().is_empty());
    }

    /// The marker takes the thread token when it has no owner and records the mark.
    #[test]
    fn take_marker_records_the_thread() {
        let (mut e, log, marker) = rig();
        set(&mut e, marker, 0xffff_fff1, 0, 0b10);
        e.call(0x00cf_6bc0, &args![marker]);
        assert_eq!(state(&e, marker), (0x77, 1, 0b101));
        assert_eq!(*log.borrow(), SECTION_CALLS.to_vec());
        set(&mut e, marker, 0x55, 4, 0);
        e.call(0x00cf_6bc0, &args![marker]);
        assert_eq!(state(&e, marker), (0x55, 5, 1));
        set(&mut e, marker, 0xffff_fff1, 0x7fff, 0);
        e.call(0x00cf_6bc0, &args![marker]);
        assert_eq!(e.mem.u16(marker + 4), 0x8000);
    }

    /// With the checks off the answer is true without the section; with them on it compares
    /// the owner with the token.
    #[test]
    fn ownership_check_compares_with_the_thread_token() {
        let (mut e, log, marker) = rig();
        set(&mut e, marker, 1, 0x8000, 0);
        assert!(e.call(0x00cf_6c10, &args![marker]).bool());
        assert!(log.borrow().is_empty());
        set(&mut e, marker, 0x77, 1, 0);
        assert!(e.call(0x00cf_6c10, &args![marker]).bool());
        assert_eq!(*log.borrow(), SECTION_CALLS.to_vec());
        set(&mut e, marker, 0x78, 1, 0);
        assert!(!e.call(0x00cf_6c10, &args![marker]).bool());
    }

    /// Both copies release a mark: shift, lower the count, and give up the owner at zero.
    #[test]
    fn releasing_marks_clears_the_owner_at_zero() {
        for address in [0x00cf_6c60, 0x00cf_6cb0] {
            let (mut e, log, marker) = rig();
            set(&mut e, marker, 0x77, 2, 0b101);
            e.call(address, &args![marker]);
            assert_eq!(state(&e, marker), (0x77, 1, 0b10));
            assert_eq!(*log.borrow(), SECTION_CALLS.to_vec());
            e.call(address, &args![marker]);
            assert_eq!(state(&e, marker), (0xffff_fff1, 0, 0b1));
            // From zero the count wraps (and the owner is not touched again).
            set(&mut e, marker, 0x66, 0, 0);
            e.call(address, &args![marker]);
            assert_eq!(state(&e, marker), (0x66, 0xffff, 0));
            log.borrow_mut().clear();
            set(&mut e, marker, 0x66, 0x8001, 4);
            e.call(address, &args![marker]);
            assert_eq!(state(&e, marker), (0x66, 0x8001, 4));
            assert!(log.borrow().is_empty());
        }
    }

    /// The checks are switched off by bit 15 and back on by the reset, which also clears the
    /// count; the reset does nothing while they are on.
    #[test]
    fn checks_are_toggled_by_bit_fifteen() {
        let (mut e, _, marker) = rig();
        set(&mut e, marker, 9, 3, 1);
        e.call(0x00cf_6d30, &args![marker]);
        assert_eq!(state(&e, marker), (9, 3, 1));
        e.call(0x00cf_6d20, &args![marker]);
        assert_eq!(e.mem.u16(marker + 4), 0x8003);
        e.call(0x00cf_6d30, &args![marker]);
        assert_eq!(state(&e, marker), (0xffff_fff1, 0, 1));
    }
}
