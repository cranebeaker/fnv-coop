//! `NiSystem/run_00aa2ff0` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nisystem::nimemstream::{
    NiMemStream, ALLOCATE, BASE_CONSTRUCTOR, MEM_STREAM_READ, MEM_STREAM_VTABLE, MEM_STREAM_WRITE,
};

/// `00aa2710`: the jump target of `00aa3040`.
const JUMP_TARGET_00AA3040: u32 = 0x00aa_2710;

/// The size of the buffer the default constructor allocates.
const DEFAULT_BUFFER_SIZE: u32 = 0x400;

// Translated from 00aa2ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the default constructor of the memory stream (it stores the
/// virtual table of `NiMemStream` constructed by `00aa2fb0`, and has the same fields):
/// a buffer of 0x400 bytes of its own (`m_bUserMemory` clear, so the
/// destructor frees it), position and end 0. Returns `this`.
pub fn fn_00aa2ff0(e: &mut Engine, this: Ptr<NiMemStream>) -> Ptr<NiMemStream> {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), MEM_STREAM_VTABLE);
    e.set(this, NiMemStream::m_uiAllocSize, DEFAULT_BUFFER_SIZE);
    let buffer = e.call(ALLOCATE, &args![DEFAULT_BUFFER_SIZE]).u32();
    e.set(this, NiMemStream::m_pBuffer, buffer);
    e.set(this, NiMemStream::m_uiPos, 0);
    e.set(this, NiMemStream::m_uiEnd, 0);
    e.set(this, NiMemStream::m_bUserMemory, false);
    e.set(this, NiMemStream::m_bFreeze, false);
    e.set(this, NiMemStream::m_pfnRead, MEM_STREAM_READ);
    e.set(this, NiMemStream::m_pfnWrite, MEM_STREAM_WRITE);
    this
}

// Translated from 00aa3040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00aa2710`, passing on `this`; returns the target result word. Return type (u32): the target `00aa2710` returns EAX at every return (-1, or the result of an imported call); its meaning is unconfirmed.
pub fn fn_00aa3040(e: &mut Engine, this: Ptr) -> u32 {
    e.call(JUMP_TARGET_00AA3040, &args![this]).u32()
}

/// Byte `011f6229`: `00aa3070` runs only while it is set and clears it.
const START_UP_DONE_FLAG: u32 = 0x011f_6229;
/// The three calls (no arguments) `00aa3070` makes, in order; the last one is a `JMP`.
/// `00aa81d0` is in unit `BSCore/oei_io_thread.cpp`.
const SHUTDOWN_CALLS: [u32; 3] = [0x00aa_81d0, 0x00aa_1980, 0x00a2_9680];

// Translated from 00aa3070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; called by `00aa2170`): when the byte `011f6229`
/// is set, clears it and calls `00aa81d0`, `00aa1980` and `00a29680` (the last through a
/// `JMP`).
pub fn fn_00aa3070(e: &mut Engine) {
    if e.mem.u8(START_UP_DONE_FLAG) == 0 {
        return;
    }
    e.mem.set_u8(START_UP_DONE_FLAG, 0);
    for step in SHUTDOWN_CALLS {
        e.call(step, &args![]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa_2ff0,
            fn_00aa2ff0(Ptr<NiMemStream>) -> Ptr<NiMemStream>
        ),
        entry!(0x00aa_3040, fn_00aa3040(Ptr) -> u32),
        entry!(0x00aa_3070, fn_00aa3070()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00aa3040_forwards_this_and_returns_the_target_result() {
        const THUNK: u32 = 0x00aa_3040;
        const TARGET: u32 = 0x00aa_2710;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 0xffff_ffff,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32]).u32(), 0xffff_ffff);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32]);
    }

    #[test]
    fn default_constructor_allocates_its_own_buffer() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        let stream = e.new_object::<NiMemStream>();
        for off in (0u32..0x24).step_by(4) {
            e.mem.set_u32(stream.addr() + off, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_2ff0, &args![stream]).u32(), stream.addr());
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|call| call.0 == ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, args![0x400u32]);
        let at = stream.addr();
        assert_eq!(e.mem.u32(at), MEM_STREAM_VTABLE);
        assert_eq!(e.mem.u32(at + 0x08), MEM_STREAM_READ);
        assert_eq!(e.mem.u32(at + 0x0c), MEM_STREAM_WRITE);
        assert_eq!(e.mem.u32(at + 0x10), 0x6000);
        assert_eq!(e.mem.u32(at + 0x14), 0);
        assert_eq!(e.mem.u32(at + 0x18), 0);
        assert_eq!(e.mem.u32(at + 0x1c), 0x400);
        assert_eq!(e.mem.u8(at + 0x20), 0);
        assert_eq!(e.mem.u8(at + 0x21), 0);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00aa3070_shuts_down_once() {
        let mut e = Engine::new();
        e.map(START_UP_DONE_FLAG & !0xfff, 0x1000);
        for step in SHUTDOWN_CALLS {
            e.register(step, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00aa_3070, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u8(START_UP_DONE_FLAG, 1);
        e.call_log = Some(vec![]);
        e.call(0x00aa_3070, &args![]);
        let log = e.call_log.take().unwrap();
        let expected: Vec<_> = SHUTDOWN_CALLS.iter().map(|&a| (a, vec![])).collect();
        assert_eq!(log[1..], expected[..]);
        assert_eq!(e.mem.u8(START_UP_DONE_FLAG), 0);
    }
}
