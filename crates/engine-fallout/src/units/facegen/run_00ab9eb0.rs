//! `FaceGen/run_00ab9eb0` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Small constructors of FaceGen's library containers: a resize that passes a default value
//! of 0x20 bytes, the byte vector's sized constructor, and three element constructors (each
//! starting with an empty `std::string`, `0044a730(0, 0)`).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{
    INVALID_PARAMETER, MEMORY_MANAGER_FREE, MEMORY_MANAGER_OBJECT, STRING_TIDY, VECTOR_LAST,
};

/// The resize with a value (`00abb060`, `thiscall`: the count and the 0x20-byte value by
/// value, nine words), unit `run_00aba3c0`.
const RESIZE_WITH_VALUE: u32 = 0x00ab_b060;
/// The container proxy's allocation (`0044d670`, `cdecl`: count and a hint; answers the
/// block) and the byte vector's construct-n (`00abbfc0`, `thiscall`: the count and the
/// address of the byte value), unit `run_00aba3c0`.
const ALLOCATE_PROXY: u32 = 0x0044_d670;
const CONSTRUCT_N: u32 = 0x00ab_bfc0;
/// The members `00aba270` and `00aba2f0` construct after their string: `0065c630` on +0x1C and
/// `00aba4f0` on +0x20 (both `thiscall`, no stack words).
const MEMBER_CONSTRUCT_0065C630: u32 = 0x0065_c630;
const MEMBER_CONSTRUCT_00ABA4F0: u32 = 0x00ab_a4f0;
/// The byte vector's element free (`00abc0a0`, `thiscall`), unit `run_00aba3c0`.
const BYTE_VECTOR_TIDY: u32 = 0x00ab_c0a0;

// Translated from 00ab9eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `resize(count)` through the resize with a value (`00abb060`).
/// The value is a 0x20-byte temporary built in the argument area: a word the code leaves
/// uninitialized (the stack's old contents; here 0) and an empty string at +4.
pub fn fn_00ab9eb0(e: &mut Engine, this: Ptr, count: u32) {
    e.with_stack(0x20, |e, value| {
        let value = value.addr();
        e.call(STRING_TIDY, &args![value + 4, 0u32, 0u32]);
        let mut words = vec![this.addr(), count];
        for i in 0..8 {
            words.push(e.mem.u32(value + 4 * i));
        }
        e.call(RESIZE_WITH_VALUE, &words);
    });
}

// Translated from 00aba0d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aba0d0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the byte vector's constructor from a count (`thiscall`). Its
/// container proxy (one, `0044d670(1, 0)`) points back at the vector when allocated and is
/// stored at +0 (null when the allocation fails); then `count` zero bytes are constructed
/// (`00abbfc0`, given the address of a zero byte). Returns `this`.
pub fn fn_00aba0d0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    let proxy = e.call(ALLOCATE_PROXY, &args![1u32, 0u32]).u32();
    let proxy = if proxy != 0 {
        e.mem.set_u32(proxy, this.addr());
        proxy
    } else {
        0
    };
    e.mem.set_u32(this.addr(), proxy);
    e.with_stack(4, |e, zero| {
        e.mem.set_u8(zero.addr(), 0);
        e.call(CONSTRUCT_N, &args![this, count, zero]);
    });
    this
}

// Translated from 00aba220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a constructor: the three floats at +4, +8, +0xC become 0.0 and
/// the string at +0x10 is made empty. Returns `this`.
pub fn fn_00aba220(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in [4u32, 8, 0xc] {
        e.mem.set_u32(base + offset, x87_store32(0.0));
    }
    e.call(STRING_TIDY, &args![base + 0x10, 0u32, 0u32]);
    this
}

// Translated from 00aba270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aba270 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a constructor: the string at +0 is made empty, then `0065c630`
/// constructs the member at +0x1C. Returns `this`.
pub fn fn_00aba270(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(STRING_TIDY, &args![this, 0u32, 0u32]);
    e.call(MEMBER_CONSTRUCT_0065C630, &args![this.addr() + 0x1c]);
    this
}

// Translated from 00aba2f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aba2f0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a constructor: the string at +0 is made empty, then `00aba4f0`
/// constructs the member at +0x20. Returns `this`.
pub fn fn_00aba2f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(STRING_TIDY, &args![this, 0u32, 0u32]);
    e.call(MEMBER_CONSTRUCT_00ABA4F0, &args![this.addr() + 0x20]);
    this
}

/// The iterator `++` of the vector instances below (`thiscall`): a null proxy fails a check
/// (`_invalid_parameter_noinfo`), and so does a pointer (+4) not below the container's last
/// pointer (the container is the proxy's first word, read at address 0 when the proxy is null);
/// then the pointer moves by one element.
fn increment_checked(e: &mut Engine, iterator: u32, size: u32) {
    if e.mem.u32(iterator) == 0 {
        e.call(INVALID_PARAMETER, &[]);
    }
    let proxy = e.mem.u32(iterator);
    let container = if proxy == 0 { 0 } else { e.mem.u32(proxy) };
    let pointer = e.mem.u32(iterator + 4);
    if pointer >= e.mem.u32(container.wrapping_add(VECTOR_LAST)) {
        e.call(INVALID_PARAMETER, &[]);
    }
    let pointer = e.mem.u32(iterator + 4).wrapping_add(size);
    e.mem.set_u32(iterator + 4, pointer);
}

// Translated from 00ab9f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x20-byte element iterator's `++` (`thiscall`; see
/// [`increment_checked`]). Returns `this`.
pub fn fn_00ab9f10(e: &mut Engine, this: Ptr) -> Ptr {
    increment_checked(e, this.addr(), 0x20);
    this
}

// Translated from 00ab9f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x2C-byte element iterator's `++` (`thiscall`; see
/// [`increment_checked`]). Returns `this`.
pub fn fn_00ab9f80(e: &mut Engine, this: Ptr) -> Ptr {
    increment_checked(e, this.addr(), 0x2c);
    this
}

// Translated from 00ab9ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x34-byte element iterator's `++` (`thiscall`; see
/// [`increment_checked`]). Returns `this`.
pub fn fn_00ab9ff0(e: &mut Engine, this: Ptr) -> Ptr {
    increment_checked(e, this.addr(), 0x34);
    this
}

// Translated from 00aba060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x38-byte element iterator's `++` (`thiscall`; see
/// [`increment_checked`]). Returns `this`.
pub fn fn_00aba060(e: &mut Engine, this: Ptr) -> Ptr {
    increment_checked(e, this.addr(), 0x38);
    this
}

// Translated from 00aba1a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aba1a0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the byte vector's destructor (`thiscall`): its elements freed
/// (`00abc0a0`), then its container proxy (+0) given back to the memory manager.
pub fn fn_00aba1a0(e: &mut Engine, this: Ptr) {
    e.call(BYTE_VECTOR_TIDY, &args![this]);
    let proxy = e.mem.u32(this.addr());
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ab_9eb0, fn_00ab9eb0(Ptr, u32)),
        entry!(0x00ab_a0d0, fn_00aba0d0(Ptr, u32) -> Ptr),
        entry!(0x00ab_a220, fn_00aba220(Ptr) -> Ptr),
        entry!(0x00ab_a270, fn_00aba270(Ptr) -> Ptr),
        entry!(0x00ab_a2f0, fn_00aba2f0(Ptr) -> Ptr),
        entry!(0x00ab_9f10, fn_00ab9f10(Ptr) -> Ptr),
        entry!(0x00ab_9f80, fn_00ab9f80(Ptr) -> Ptr),
        entry!(0x00ab_9ff0, fn_00ab9ff0(Ptr) -> Ptr),
        entry!(0x00ab_a060, fn_00aba060(Ptr) -> Ptr),
        entry!(0x00ab_a1a0, fn_00aba1a0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        // The empty string: capacity 15, length 0.
        e.register(STRING_TIDY, |e, a| {
            e.mem.set_u32(a[0] + 0x14, 0);
            e.mem.set_u32(a[0] + 0x18, 15);
            e.mem.set_u8(a[0] + 4, 0);
            Ret::default()
        });
        for addr in [
            RESIZE_WITH_VALUE,
            CONSTRUCT_N,
            MEMBER_CONSTRUCT_0065C630,
            MEMBER_CONSTRUCT_00ABA4F0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn resize_passes_an_empty_default_value() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00ab_9eb0, &args![this, 5u32]);
        let log = e.call_log.take().unwrap();
        let resize = calls_to(&log, RESIZE_WITH_VALUE);
        assert_eq!(resize.len(), 1);
        assert_eq!(resize[0].len(), 10);
        assert_eq!(resize[0][..2], [this, 5]);
        // The string's length and capacity words.
        assert_eq!(resize[0][2 + 6], 0);
        assert_eq!(resize[0][2 + 7], 15);
    }

    #[test]
    fn byte_vector_links_its_proxy_and_constructs_zeros() {
        let mut e = engine();
        e.register(ALLOCATE_PROXY, |e, _| e.mem.alloc(8).into_ret());
        e.register_double(CONSTRUCT_N, |e, a| {
            assert_eq!(e.mem.u8(a[2]), 0);
            Ret::default()
        });
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ab_a0d0, &args![this, 9u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE_PROXY), vec![vec![1, 0]]);
        let proxy = e.mem.u32(this);
        assert_eq!(e.mem.u32(proxy), this);
        assert_eq!(calls_to(&log, CONSTRUCT_N)[0][..2], [this, 9]);
        // A failed proxy allocation leaves a null proxy.
        e.register(ALLOCATE_PROXY, |_, _| Ret::default());
        e.call(0x00ab_a0d0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn element_constructors_build_their_members() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        for i in 0..0x10 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ab_a220, &args![this]).u32(), this);
        assert_eq!(e.call(0x00ab_a270, &args![this]).u32(), this);
        assert_eq!(e.call(0x00ab_a2f0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(this + 4), 0.0);
        assert_eq!(e.mem.f32(this + 0xc), 0.0);
        assert_eq!(
            calls_to(&log, STRING_TIDY),
            vec![vec![this + 0x10, 0, 0], vec![this, 0, 0], vec![this, 0, 0]]
        );
        assert_eq!(
            calls_to(&log, MEMBER_CONSTRUCT_0065C630),
            vec![vec![this + 0x1c]]
        );
        assert_eq!(
            calls_to(&log, MEMBER_CONSTRUCT_00ABA4F0),
            vec![vec![this + 0x20]]
        );
    }

    #[test]
    fn iterator_increments_check_the_end() {
        let mut e = engine();
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        let vector = e.mem.alloc(0x18);
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, vector);
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, proxy);
        for (addr, size) in [
            (0x00ab_9f10u32, 0x20u32),
            (0x00ab_9f80, 0x2c),
            (0x00ab_9ff0, 0x34),
            (0x00ab_a060, 0x38),
        ] {
            e.mem.set_u32(vector + VECTOR_LAST, 0x1000 + size);
            e.mem.set_u32(iterator + 4, 0x1000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![iterator]).u32(), iterator);
            assert_eq!(e.mem.u32(iterator + 4), 0x1000 + size);
            // At the end: the check fails and the pointer still moves.
            e.call(addr, &args![iterator]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.u32(iterator + 4), 0x1000 + 2 * size);
            assert_eq!(calls_to(&log, INVALID_PARAMETER).len(), 1);
        }
    }

    #[test]
    fn byte_vector_destructor_frees_elements_then_proxy() {
        let mut e = engine();
        for addr in [BYTE_VECTOR_TIDY, MEMORY_MANAGER_FREE] {
            e.register(addr, |_, _| Ret::default());
        }
        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00ab_a1a0, &args![vector]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (BYTE_VECTOR_TIDY, vec![vector]),
                (MEMORY_MANAGER_FREE, vec![MEMORY_MANAGER_OBJECT, 0x4444])
            ]
        );
    }
}
