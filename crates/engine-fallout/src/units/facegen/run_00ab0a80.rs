//! `FaceGen/run_00ab0a80` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The container proxy's allocation (`0044d670`, `cdecl`: count and hint).
const ALLOCATE_PROXY: u32 = 0x0044_d670;
/// The float vector's buy (`0065f380`, `thiscall`: the count).
const FLOATS_BUY: u32 = 0x0065_f380;

// Translated from 00ab0a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a float vector's default constructor (`thiscall`, no words): its container proxy
/// (one, `0044d670(1, 0)`, pointing back at the vector; null when the allocation fails) at +0,
/// then the buy of 0 elements (`0065f380`), which only clears the pointers. Returns `this`.
// Translation incomplete: 00ab0a80 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f1f8d6, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00ab0a80(e: &mut Engine, this: Ptr) -> Ptr {
    let proxy = e.call(ALLOCATE_PROXY, &args![1u32, 0u32]).u32();
    let proxy = if proxy != 0 {
        e.mem.set_u32(proxy, this.addr());
        proxy
    } else {
        0
    };
    e.mem.set_u32(this.addr(), proxy);
    e.call(FLOATS_BUY, &args![this, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ab_0a80, fn_00ab0a80(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_float_vector_links_its_proxy_and_buys_nothing() {
        let mut e = Engine::new();
        e.register(ALLOCATE_PROXY, |e, _| e.mem.alloc(8).into_ret());
        e.register(FLOATS_BUY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this: Ptr = Ptr::new(e.mem.alloc(0x18));
        assert_eq!(fn_00ab0a80(&mut e, this), this);
        let proxy = e.mem.u32(this.addr());
        assert_eq!(e.mem.u32(proxy), this.addr());
        let log = e.call_log.take().unwrap();
        let buys: Vec<_> = log.iter().filter(|(a, _)| *a == FLOATS_BUY).collect();
        assert_eq!(buys.len(), 1);
        assert_eq!(buys[0].1, vec![this.addr(), 0]);
        // A failed proxy allocation leaves a null proxy.
        e.register(ALLOCATE_PROXY, |_, _| Ret::default());
        fn_00ab0a80(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), 0);
    }
}
