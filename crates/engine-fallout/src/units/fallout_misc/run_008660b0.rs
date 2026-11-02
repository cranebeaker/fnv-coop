//! `fallout/misc/run_008660b0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008660b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Grows a save buffer by 0x10000 bytes. The mode byte at global
/// 011de134's receiver +0xc suppresses allocation/copying when set.
/// Captures old pointer/size before the callback, reloads new size after it,
/// copies the old capacity and deletes the old block.
pub fn grow_save_buffer(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let old_buffer = e.mem.u32(base.wrapping_add(4));
    let old_size = e.mem.u32(base.wrapping_add(8));
    e.mem
        .set_u32(base.wrapping_add(8), old_size.wrapping_add(0x10000));
    let state = e.mem.u32(0x011de134);
    if e.call(0x00424940, &args![state]).u8() == 0 {
        let size = e.mem.u32(base.wrapping_add(8));
        let buffer = e.call(0x00401000, &args![size]).u32();
        e.mem.set_u32(base.wrapping_add(4), buffer);
        if old_buffer != 0 {
            let destination = e.mem.u32(base.wrapping_add(4));
            e.call(0x00401460, &args![destination, old_buffer, old_size]);
        }
        e.call(0x00401030, &args![old_buffer]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008660b0, grow_save_buffer(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn growth_captures_old_allocation_but_reloads_capacity_after_mode_callback() {
        let mut e = Engine::new();
        e.map(0x011de134, 4);
        let this = e.mem.alloc(20);
        let old = e.mem.alloc(8);
        let replacement = e.mem.alloc(32);
        e.mem.write(old, b"old-data");
        e.mem.set_u32(this + 4, old);
        e.mem.set_u32(this + 8, 8);
        e.set_global(0x011de134, 0x03000000u32);
        e.register_double(0x00424940, move |e, a| {
            assert_eq!(a, &[0x03000000]);
            assert_eq!(e.mem.u32(this + 8), 0x10008);
            e.mem.set_u32(this + 4, 0xdeadbeef);
            e.mem.set_u32(this + 8, 32);
            0u32.into_ret()
        });
        e.register_double(0x00401000, move |_, a| {
            assert_eq!(a, &[32]);
            replacement.into_ret()
        });
        e.register_double(0x00401460, move |e, a| {
            assert_eq!(a, &[replacement, old, 8]);
            let data = e.mem.bytes(old, 8).to_vec();
            e.mem.write(replacement, &data);
            Ret::default()
        });
        e.register_double(0x00401030, move |_, a| {
            assert_eq!(a, &[old]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x008660b0, &[this]);
        assert_eq!(e.mem.u32(this + 4), replacement);
        assert_eq!(e.mem.u32(this + 8), 32);
        assert_eq!(e.mem.bytes(replacement, 8), b"old-data");
        let addresses: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .map(|(address, _)| address)
            .collect();
        assert_eq!(
            addresses,
            [0x008660b0, 0x00424940, 0x00401000, 0x00401460, 0x00401030]
        );
    }

    #[test]
    fn first_growth_skips_copy_but_still_deletes_null() {
        let mut e = Engine::new();
        e.map(0x011de134, 4);
        let this = e.mem.alloc(20);
        e.mem.set_u32(this + 4, 0);
        e.mem.set_u32(this + 8, 0);
        e.set_global(0x011de134, 0u32);
        e.register_double(0x00424940, |_, _| 0u32.into_ret());
        e.register_double(0x00401000, |_, a| {
            assert_eq!(a, &[0x10000]);
            0x03000000u32.into_ret()
        });
        e.register_double(0x00401460, |_, _| panic!("no old buffer to copy"));
        e.register_double(0x00401030, |_, a| {
            assert_eq!(a, &[0]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x008660b0, &[this]);
        assert_eq!(e.mem.u32(this + 4), 0x03000000);
        assert_eq!(e.mem.u32(this + 8), 0x10000);
        assert!(e
            .call_log
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0x00401030));
    }

    #[test]
    fn nonzero_low_mode_byte_suppresses_allocation_and_capacity_wraps() {
        let mut e = Engine::new();
        e.map(0x011de134, 4);
        let this = e.mem.alloc(20);
        e.mem.set_u32(this + 4, 0xdeadbeef);
        e.mem.set_u32(this + 8, 0xffff0008);
        e.set_global(0x011de134, 0u32);
        e.register_double(0x00424940, |_, _| 0x101u32.into_ret());
        e.register_double(0x00401000, |_, _| panic!("sizing mode must not allocate"));
        e.register_double(0x00401030, |_, _| panic!("sizing mode must not delete"));
        e.call(0x008660b0, &[this]);
        assert_eq!(e.mem.u32(this + 8), 8);
        assert_eq!(e.mem.u32(this + 4), 0xdeadbeef);
    }
}
