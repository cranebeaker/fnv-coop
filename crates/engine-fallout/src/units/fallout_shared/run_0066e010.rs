//! `fallout shared/run_0066e010` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`thiscall`, no stack word): called on +8 first (its result is not used).
const PREPARE_POINT: u32 = 0x0068_15c0;
/// `00436aa0` (`thiscall`, no stack word; unit `fallout shared/extradataobjects.cpp`): the
/// address of the reference's three-word position.
const REFERENCE_POSITION: u32 = 0x0043_6aa0;
/// The three zero words (`011f426c`) used when there is no reference.
const ZERO_POINT: u32 = 0x011f_426c;
/// `008d8520` (`thiscall`, no stack word; engine map
/// `MiddleHighProcess::GetSavedAcquireObject`, a folded body).
const REFERENCE_PROCESS: u32 = 0x008d_8520;
/// The running counter `0066e010` numbers its records with (`011d60cc`).
const RECORD_COUNTER: u32 = 0x011d_60cc;
/// The list the new record is added to (`011d6124`) and its add (`005ae3d0`, `thiscall`,
/// `RET 4`: the address of a word holding the record).
const RECORD_LIST: u32 = 0x011d_6124;
const LIST_ADD: u32 = 0x005a_e3d0;

// Translated from 0066e010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a constructor of a 0x1c-byte record for
/// `reference`. Calls `006815c0` on +8, stores the reference at +0, copies the reference's
/// position (`00436aa0`; the zero point `011f426c` for a null reference) to +8, clears +4 and
/// sets it, when the reference has a process (`008d8520`), to what the process's vtable slot
/// +0xf4 answers for the reference; clears +0x18, numbers the record at +0x14 from the
/// counter `011d60cc` (then incremented), and adds the record to the list at `011d6124`
/// (`005ae3d0`, given the address of a local holding `this`). Returns `this`.
pub fn fn_0066e010(e: &mut Engine, this: Ptr, reference: u32) -> Ptr {
    let base = this.addr();
    e.call(PREPARE_POINT, &args![base + 8]);
    e.mem.set_u32(base, reference);
    let source = if e.mem.u32(base) != 0 {
        let stored = e.mem.u32(base);
        e.call(REFERENCE_POSITION, &args![stored]).u32()
    } else {
        ZERO_POINT
    };
    for word in 0..3u32 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(base + 8 + 4 * word, value);
    }
    e.mem.set_u32(base + 4, 0);
    let process = if e.mem.u32(base) != 0 {
        e.call(REFERENCE_PROCESS, &args![reference]).u32()
    } else {
        0
    };
    if process != 0 {
        let stored = e.mem.u32(base);
        let answer = e.vcall(process, 0xf4, &args![stored]).u32();
        e.mem.set_u32(base + 4, answer);
    }
    e.mem.set_u32(base + 0x18, 0);
    let number = e.mem.u32(RECORD_COUNTER);
    e.mem.set_u32(base + 0x14, number);
    let next = e.mem.u32(RECORD_COUNTER).wrapping_add(1);
    e.mem.set_u32(RECORD_COUNTER, next);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), base);
        e.call(LIST_ADD, &args![RECORD_LIST, local]);
    });
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0066_e010, fn_0066e010(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Engine {
        let mut e = Engine::new();
        e.map(ZERO_POINT & !0xfff, 0x1000);
        e.map(RECORD_COUNTER & !0xfff, 0x1000);
        e.mem.set_u32(RECORD_COUNTER, 41);
        e.register(PREPARE_POINT, |_, _| Ret::default());
        e.register(LIST_ADD, |e, a| {
            assert_ne!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        e
    }

    #[test]
    fn a_reference_with_a_process() {
        let mut e = fixture();
        let position = e.mem.alloc(0xc);
        for word in 0..3u32 {
            e.mem.set_u32(position + 4 * word, 0x100 + word);
        }
        e.register_double(REFERENCE_POSITION, move |_, _| position.into_ret());
        e.put_vtable(0x0150_0000, &[0; 0x3e]);
        e.mem.set_u32(0x0150_00f4, 0x0150_1000);
        e.register(0x0150_1000, |_, _| 0x99u32.into_ret());
        let process = e.mem.alloc(4);
        e.mem.set_u32(process, 0x0150_0000);
        e.register_double(REFERENCE_PROCESS, move |_, _| process.into_ret());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0066_e010, &args![this, 0x6000u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (PREPARE_POINT, vec![this + 8]));
        assert!(log.contains(&(REFERENCE_PROCESS, vec![0x6000])));
        assert!(log.contains(&(0x0150_1000, vec![process, 0x6000])));
        assert_eq!(log.last().unwrap().0, LIST_ADD);
        assert_eq!(log.last().unwrap().1[0], RECORD_LIST);
        let words: Vec<u32> = (0..7).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, [0x6000, 0x99, 0x100, 0x101, 0x102, 41, 0]);
        assert_eq!(e.mem.u32(RECORD_COUNTER), 42);
    }

    #[test]
    fn a_null_reference_takes_the_zero_point() {
        let mut e = fixture();
        e.mem.set_u32(ZERO_POINT, 0x55);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 4, 7);
        e.call_log = Some(vec![]);
        e.call(0x0066_e010, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .all(|c| c.0 != REFERENCE_POSITION && c.0 != REFERENCE_PROCESS));
        assert_eq!(e.mem.u32(this + 8), 0x55);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0x14), 41);
    }
}
