//! `Havok SDK/run_00cf5f80` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkQuaternion::set` (`00cb26e0`, `this` = the quaternion, stack `(matrix)`, callee-cleaned;
/// named by Ghidra from the Xbox PDB).
const QUATERNION_SET: u32 = 0x00cb_26e0;

/// Copies one 16-byte vector (`MOVAPS` load and store), bit for bit.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

// Translated from 00cf5f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a block of a source object into `this` (both classes are unconfirmed): the float at
/// `source + 4` goes to `+0x90` through the x87 stack (a signalling NaN is quieted) and the
/// 16-byte vectors at `source + 0x20`, `+0x30`, `+0x40` and `+0x10` to `+0x50`, `+0x60`, `+0x70`
/// and `+0x80`.
pub fn fn_00cf5f80(e: &mut Engine, this: Ptr, source: Ptr) {
    let (base, from) = (this.addr(), source.addr());
    let bits = x87_quiet_bits(e.mem.u32(from + 4));
    e.mem.set_u32(base + 0x90, bits);
    copy_vector(e, from + 0x20, base + 0x50);
    copy_vector(e, from + 0x30, base + 0x60);
    copy_vector(e, from + 0x40, base + 0x70);
    copy_vector(e, from + 0x10, base + 0x80);
}

// Translated from 00cf5fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 16-byte vector at `source + 0x30` to `this + 0x10` and sets the quaternion at `this
/// + 0x20` from the matrix at `source` (`hkQuaternion::set`, `00cb26e0`).
pub fn fn_00cf5fc0(e: &mut Engine, this: Ptr, source: Ptr) {
    copy_vector(e, source.addr() + 0x30, this.addr() + 0x10);
    e.call(QUATERNION_SET, &args![this.addr() + 0x20, source]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_5f80, fn_00cf5f80(Ptr, Ptr)),
        entry!(0x00cf_5fc0, fn_00cf5fc0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A source whose words are numbered from `0x100`.
    fn source(e: &mut Engine) -> u32 {
        let at = e.mem.alloc(0x50);
        for i in 0..20 {
            e.mem.set_u32(at + 4 * i, 0x100 + i);
        }
        at
    }

    fn words(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    /// The four vectors land in their places, the float goes through the x87 quieting, and the
    /// other words of `this` stay.
    #[test]
    fn block_copy_places_four_vectors_and_a_float() {
        let mut e = Engine::new();
        let from = source(&mut e);
        e.mem.set_u32(from + 4, 0x7f80_0001);
        let this = e.mem.alloc(0xa0);
        e.mem.write(this, &[0xaa; 0xa0]);
        e.call(0x00cf_5f80, &args![this, from]);
        assert_eq!(e.mem.u32(this + 0x90), 0x7fc0_0001);
        assert_eq!(words(&e, this + 0x50, 4), vec![0x108, 0x109, 0x10a, 0x10b]);
        assert_eq!(words(&e, this + 0x60, 4), vec![0x10c, 0x10d, 0x10e, 0x10f]);
        assert_eq!(words(&e, this + 0x70, 4), vec![0x110, 0x111, 0x112, 0x113]);
        assert_eq!(words(&e, this + 0x80, 4), vec![0x104, 0x105, 0x106, 0x107]);
        assert_eq!(e.mem.u32(this + 0x94), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(this + 0x4c), 0xaaaa_aaaa);
    }

    /// A vector is copied and the quaternion is set from the whole source block.
    #[test]
    fn vector_copy_and_quaternion_set() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(QUATERNION_SET, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let from = source(&mut e);
        let this = e.mem.alloc(0x40);
        e.call(0x00cf_5fc0, &args![this, from]);
        assert_eq!(words(&e, this + 0x10, 4), vec![0x10c, 0x10d, 0x10e, 0x10f]);
        assert_eq!(*seen.borrow(), vec![vec![this + 0x20, from]]);
    }
}
