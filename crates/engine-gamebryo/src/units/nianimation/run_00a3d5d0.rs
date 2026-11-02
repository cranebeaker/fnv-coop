//! `NiAnimation/run_00a3d5d0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a3d5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (time, keys, count, &index, size byte)): where a key at
/// `time` goes among `count` keys of `size` bytes sorted by time: `*index` runs from 0 past
/// every key whose time is before `time`; answers false when it stops at a key whose time
/// equals `time` (or is NaN against it), true otherwise (also with no keys, or when every key
/// is before `time`). The answer is `AL`.
pub fn fn_00a3d5d0(
    e: &mut Engine,
    time: f32,
    keys: Ptr,
    count: u32,
    index_at: Ptr,
    size: u32,
) -> bool {
    e.mem.set_u32(index_at.addr(), 0);
    if count == 0 {
        return true;
    }
    let t = x87_load32(time.to_bits());
    let size = size & 0xff;
    loop {
        let index = e.mem.u32(index_at.addr());
        let key = size.wrapping_mul(index).wrapping_add(keys.addr());
        let key_time = x87_load32(e.mem.u32(key));
        // FCOMP: the key's time against `time`; TEST AH,5 / JP stops unless it is below.
        if !(key_time < t) {
            return key_time > t;
        }
        let index = index.wrapping_add(1);
        e.mem.set_u32(index_at.addr(), index);
        if index >= count {
            return true;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a3_d5d0,
        fn_00a3d5d0(f32, Ptr, u32, Ptr, u32) -> bool
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_position_skips_earlier_keys() {
        let mut e = Engine::new();
        let keys = e.mem.alloc(0x18);
        for (i, t) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(keys + 8 * i as u32, *t);
        }
        let index = e.mem.alloc(4);
        let call = |e: &mut Engine, time: f32, count: u32| {
            e.call(0x00a3_d5d0, &args![time, keys, count, index, 0x108u32])
                .bool()
        };
        assert!(call(&mut e, 2.5, 3));
        assert_eq!(e.mem.u32(index), 2);
        // A key at that time: false at its index.
        assert!(!call(&mut e, 2.0, 3));
        assert_eq!(e.mem.u32(index), 1);
        // After every key, or no keys: true.
        assert!(call(&mut e, 9.0, 3));
        assert_eq!(e.mem.u32(index), 3);
        assert!(call(&mut e, 9.0, 0));
        assert_eq!(e.mem.u32(index), 0);
        // NaN time: stops at the first key with false.
        assert!(!call(&mut e, f32::NAN, 3));
        assert_eq!(e.mem.u32(index), 0);
    }
}
