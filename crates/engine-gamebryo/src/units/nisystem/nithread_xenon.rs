//! `NiSystem/nithread_xenon.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `RaiseException` import slot (stdcall: code, flags, argument count, arguments).
const RAISE_EXCEPTION: u32 = 0x00fd_f154;
/// The exception code a debugger takes as "name this thread" (`MS_VC_EXCEPTION`).
const MS_VC_EXCEPTION: u32 = 0x406d_1388;

// Translated from 00aa2740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aa2740 - its FS:[0] SEH frame and the __except handler that swallows the raised exception are not translated
/// `SetThreadName` (Xbox PDB; cdecl, two words: thread id, name): fills a `THREADNAME_INFO`
/// on the stack (type 0x1000, the name, the thread id, flags 0) and raises the exception
/// `0x406d1388` with its four words (`RaiseException(code, 0, 4, &info)`), which a debugger
/// takes as the thread's name.
pub fn set_thread_name(e: &mut Engine, thread_id: u32, name: Ptr) {
    e.with_stack(0x10, |e, info| {
        let info = info.addr();
        e.mem.set_u32(info, 0x1000);
        e.mem.set_u32(info + 4, name.addr());
        e.mem.set_u32(info + 8, thread_id);
        e.mem.set_u32(info + 0xc, 0);
        e.call(RAISE_EXCEPTION, &args![MS_VC_EXCEPTION, 0u32, 4u32, info]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa2740, set_thread_name(u32, Ptr))]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    #[test]
    fn set_thread_name_raises_the_naming_exception() {
        let mut e = Engine::new();
        e.register(RAISE_EXCEPTION, |e, a| {
            assert_eq!(&a[..3], &[MS_VC_EXCEPTION, 0, 4]);
            let info = a[3];
            assert_eq!(
                (
                    e.mem.u32(info),
                    e.mem.u32(info + 4),
                    e.mem.u32(info + 8),
                    e.mem.u32(info + 0xc)
                ),
                (0x1000, 0x5000, 0x77, 0)
            );
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00aa2740, &args![0x77u32, 0x5000u32]);
        let raised = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == RAISE_EXCEPTION)
            .count();
        assert_eq!(raised, 1);
    }
}
