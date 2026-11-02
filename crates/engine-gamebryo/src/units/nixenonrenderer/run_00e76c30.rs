//! `NiXenonRenderer/run_00e76c30` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_memcpy` (`00ec44d0`) and `_memset` (`00ec61c0`), `cdecl`, three words each.
const MEMCPY: u32 = 0x00ec_44d0;
const MEMSET: u32 = 0x00ec_61c0;

// Translated from 00e783b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word: a stream-copy descriptor; plain `RET`). Copies `count`
/// (`u16` at +8) elements into the destination (+0x2c), stepping it by the destination
/// stride (+0x20), and returns the sum of the element size (+0x1c) over the elements.
///
/// - No source (+0x10 null): each destination element is cleared with `memset(dst, 0,
///   size)`, the size re-read after each one.
/// - With a source: each element copies `n` bytes, `n` being the `u16` at +4 minus 0x18
///   (as a `u16`) when that is at most 4, else 4, then clears the rest of the first four
///   bytes (`memset(dst + n, 0, 4 - n)`) when `n < 4`. Without an index array (+0xc null)
///   the source steps by its stride (+0x18); with one, element `i` is read at
///   `source + index[i] * stride` (`u16` indices).
pub fn fn_00e783b0(e: &mut Engine, descriptor: Ptr) -> u32 {
    let d = descriptor.addr();
    let source = e.mem.u32(d.wrapping_add(0x10));
    let index = e.mem.u32(d.wrapping_add(0xc));
    let mut destination = e.mem.u32(d.wrapping_add(0x2c));
    let mut total = 0u32;
    let count = |e: &Engine| e.mem.u16(d.wrapping_add(8));
    if source == 0 {
        let mut i = 0u16;
        while i < count(e) {
            let size = e.mem.u32(d.wrapping_add(0x1c));
            e.call(MEMSET, &args![destination, 0u32, size]);
            let size = e.mem.u32(d.wrapping_add(0x1c));
            destination = destination.wrapping_add(e.mem.u32(d.wrapping_add(0x20)));
            total = total.wrapping_add(size);
            i = i.wrapping_add(1);
        }
        return total;
    }
    let kind = e.mem.u16(d.wrapping_add(4)).wrapping_sub(0x18);
    let n: u16 = if kind > 4 { 4 } else { kind };
    let pad = |e: &mut Engine, destination: u32| {
        if n < 4 {
            e.call(
                MEMSET,
                &args![
                    destination.wrapping_add(u32::from(n)),
                    0u32,
                    u32::from(4 - n)
                ],
            );
        }
    };
    let mut i = 0u16;
    if index != 0 {
        while i < count(e) {
            let entry = u32::from(e.mem.u16(index.wrapping_add(u32::from(i) * 2)));
            let offset = entry.wrapping_mul(e.mem.u32(d.wrapping_add(0x18)));
            e.call(
                MEMCPY,
                &args![destination, source.wrapping_add(offset), u32::from(n)],
            );
            pad(e, destination);
            total = total.wrapping_add(e.mem.u32(d.wrapping_add(0x1c)));
            destination = destination.wrapping_add(e.mem.u32(d.wrapping_add(0x20)));
            i = i.wrapping_add(1);
        }
    } else {
        let mut source = source;
        while i < count(e) {
            e.call(MEMCPY, &args![destination, source, u32::from(n)]);
            pad(e, destination);
            let size = e.mem.u32(d.wrapping_add(0x1c));
            source = source.wrapping_add(e.mem.u32(d.wrapping_add(0x18)));
            destination = destination.wrapping_add(e.mem.u32(d.wrapping_add(0x20)));
            total = total.wrapping_add(size);
            i = i.wrapping_add(1);
        }
    }
    total
}

/// `_ftol2` (`00ec62c0`): the value in ST0 (a leading `f64` argument) truncated to an
/// integer in EAX.
const FTOL: u32 = 0x00ec_62c0;

/// The element loop the packers below share (the exe repeats it in each). A descriptor holds
/// the format kind (+0), the component count (`u16` +4), the element count (`u16` +8, re-read
/// each time), an optional `u16` index array (+0xc), the source (+0x10) and its stride
/// (+0x18), the element size (+0x1c) and the destination (+0x2c) and its stride (+0x20).
/// Without a source each destination element is cleared (`memset(destination, 0, size)`, the
/// size read before and after each clear). With one, `element` packs each element from its
/// source (`source + index[i] * stride` with an index array, else the source stepping by its
/// stride) into the destination. Returns the sum of the element size (read after each
/// element).
fn pack_elements(e: &mut Engine, d: u32, element: &mut dyn FnMut(&mut Engine, u32, u32)) -> u32 {
    let source = e.mem.u32(d.wrapping_add(0x10));
    let index = e.mem.u32(d.wrapping_add(0xc));
    let mut destination = e.mem.u32(d.wrapping_add(0x2c));
    let mut total = 0u32;
    let count = |e: &Engine| e.mem.u16(d.wrapping_add(8));
    let mut i = 0u16;
    if source == 0 {
        if i >= count(e) {
            return total;
        }
        let mut size = e.mem.u32(d.wrapping_add(0x1c));
        loop {
            e.call(MEMSET, &args![destination, 0u32, size]);
            size = e.mem.u32(d.wrapping_add(0x1c));
            destination = destination.wrapping_add(e.mem.u32(d.wrapping_add(0x20)));
            i = i.wrapping_add(1);
            total = total.wrapping_add(size);
            if i >= count(e) {
                return total;
            }
        }
    }
    if index != 0 {
        while i < count(e) {
            let entry = u32::from(e.mem.u16(index.wrapping_add(u32::from(i) * 2)));
            let from = entry
                .wrapping_mul(e.mem.u32(d.wrapping_add(0x18)))
                .wrapping_add(source);
            element(e, from, destination);
            total = total.wrapping_add(e.mem.u32(d.wrapping_add(0x1c)));
            destination = destination.wrapping_add(e.mem.u32(d.wrapping_add(0x20)));
            i = i.wrapping_add(1);
        }
    } else {
        let mut from = source;
        while i < count(e) {
            element(e, from, destination);
            total = total.wrapping_add(e.mem.u32(d.wrapping_add(0x1c)));
            from = from.wrapping_add(e.mem.u32(d.wrapping_add(0x18)));
            destination = destination.wrapping_add(e.mem.u32(d.wrapping_add(0x20)));
            i = i.wrapping_add(1);
        }
    }
    total
}

/// The 16-bit packers' component count: the descriptor's `u16` +4 less `skip` (as a `u16`),
/// at most the width (2 for format kind 6, else 4); and the width.
fn short_counts(e: &Engine, d: u32, skip: u16) -> (u16, u16) {
    let components = e.mem.u16(d.wrapping_add(4)).wrapping_sub(skip);
    let width: u16 = if e.mem.u32(d) != 6 { 4 } else { 2 };
    (components.min(width), width)
}

/// The byte packers' component count: the descriptor's `u16` +4 less `skip` (as a `u16`), at
/// most 4.
fn byte_count(e: &Engine, d: u32, skip: u16) -> u16 {
    e.mem.u16(d.wrapping_add(4)).wrapping_sub(skip).min(4)
}

/// The rest of a 16-bit element up to the width, zeroed in place (`REP STOSD`/`STOSW`).
fn pad_shorts(e: &mut Engine, at: u32, written: u16, width: u16) {
    if written < width {
        for k in 0..u32::from(width - written) {
            e.mem.set_u16(at.wrapping_add(k * 2), 0);
        }
    }
}

/// The rest of a 4-byte element, cleared with `memset` when fewer than 4 bytes were written.
fn pad_bytes(e: &mut Engine, at: u32, written: u16) {
    if written < 4 {
        e.call(MEMSET, &args![at, 0u32, u32::from(4 - written)]);
    }
}

/// A 16-bit packer whose components are read with `read` from a source stepping by `step`.
fn pack_shorts(
    e: &mut Engine,
    d: u32,
    skip: u16,
    step: u32,
    read: fn(&mut Engine, u32) -> u16,
) -> u32 {
    let source = e.mem.u32(d.wrapping_add(0x10));
    if source == 0 {
        return pack_elements(e, d, &mut |_, _, _| {});
    }
    let (n, width) = short_counts(e, d, skip);
    pack_elements(e, d, &mut |e, from, to| {
        for k in 0..u32::from(n) {
            let value = read(e, from.wrapping_add(k * step));
            e.mem.set_u16(to.wrapping_add(k * 2), value);
        }
        pad_shorts(e, to.wrapping_add(u32::from(n) * 2), n, width);
    })
}

/// A byte packer whose components are read with `read` from a source stepping by `step`.
fn pack_bytes(
    e: &mut Engine,
    d: u32,
    skip: u16,
    step: u32,
    read: fn(&mut Engine, u32) -> u8,
) -> u32 {
    let source = e.mem.u32(d.wrapping_add(0x10));
    if source == 0 {
        return pack_elements(e, d, &mut |_, _, _| {});
    }
    let n = byte_count(e, d, skip);
    pack_elements(e, d, &mut |e, from, to| {
        for k in 0..u32::from(n) {
            let value = read(e, from.wrapping_add(k * step));
            e.mem.set_u8(to.wrapping_add(k), value);
        }
        pad_bytes(e, to.wrapping_add(u32::from(n)), n);
    })
}

fn float_to_short(e: &mut Engine, at: u32) -> u16 {
    let value = x87_load32(e.mem.u32(at));
    e.call(FTOL, &args![value]).u16()
}

fn low_short(e: &mut Engine, at: u32) -> u16 {
    e.mem.u16(at)
}

fn signed_byte_short(e: &mut Engine, at: u32) -> u16 {
    e.mem.u8(at) as i8 as i16 as u16
}

fn unsigned_byte_short(e: &mut Engine, at: u32) -> u16 {
    u16::from(e.mem.u8(at))
}

fn float_to_byte(e: &mut Engine, at: u32) -> u8 {
    crate::x87::fistp_chop(x87_load32(e.mem.u32(at))) as u8
}

fn low_byte(e: &mut Engine, at: u32) -> u8 {
    e.mem.u8(at)
}

// Translated from 00e76c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word: a stream-copy descriptor, see `pack_elements`): packs floats
/// into 16-bit integers. Without a source the elements are cleared. Otherwise each element
/// takes `n` components (the `u16` +4, at most the width: 2 for format kind 6, else 4), each a
/// float (4 bytes) truncated by `_ftol2`, and the rest of the width is zeroed. Returns the
/// summed element size.
pub fn fn_00e76c30(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_shorts(e, descriptor.addr(), 0, 4, float_to_short)
}

// Translated from 00e76e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word: a stream-copy descriptor): as `fn_00e76c30`, with `n` from the
/// `u16` +4 less 4 and each component the low 16 bits of a 4-byte source value.
pub fn fn_00e76e30(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_shorts(e, descriptor.addr(), 4, 4, low_short)
}

// Translated from 00e77010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): as `fn_00e76e30` (the same code), with `n` from the `u16` +4
/// less 8.
pub fn fn_00e77010(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_shorts(e, descriptor.addr(), 8, 4, low_short)
}

// Translated from 00e771f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): as `fn_00e76c30`, with `n` from the `u16` +4 less 0xc; each
/// element's `n` 16-bit components are copied with `memcpy` (`2 n` bytes, also when `n` is 0)
/// and the rest of the width zeroed.
pub fn fn_00e771f0(e: &mut Engine, descriptor: Ptr) -> u32 {
    let d = descriptor.addr();
    let source = e.mem.u32(d.wrapping_add(0x10));
    if source == 0 {
        return pack_elements(e, d, &mut |_, _, _| {});
    }
    let (n, width) = short_counts(e, d, 0xc);
    let bytes = u32::from(n) * 2;
    pack_elements(e, d, &mut |e, from, to| {
        e.call(MEMCPY, &args![to, from, bytes]);
        pad_shorts(e, to.wrapping_add(bytes), n, width);
    })
}

// Translated from 00e773b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): as `fn_00e76e30`, with `n` from the `u16` +4 less 0x10 and the
/// 16-bit components read 2 bytes apart.
pub fn fn_00e773b0(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_shorts(e, descriptor.addr(), 0x10, 2, low_short)
}

// Translated from 00e77590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): as `fn_00e76e30`, with `n` from the `u16` +4 less 0x14 and each
/// component a signed byte (`MOVSX`) widened to 16 bits.
pub fn fn_00e77590(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_shorts(e, descriptor.addr(), 0x14, 1, signed_byte_short)
}

// Translated from 00e77770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): as `fn_00e76e30`, with `n` from the `u16` +4 less 0x18 and each
/// component an unsigned byte (`MOVZX`) widened to 16 bits.
pub fn fn_00e77770(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_shorts(e, descriptor.addr(), 0x18, 1, unsigned_byte_short)
}

// Translated from 00e77950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word: a stream-copy descriptor): packs floats into bytes. Without a
/// source the elements are cleared. Otherwise each element takes `n` components (the `u16`
/// +4, at most 4), each a float (4 bytes) stored as an integer under rounding toward zero
/// (`FISTP` with the control word's rounding bits set) of which the low byte is kept; when
/// fewer than 4 bytes were written the rest is cleared with `memset`. Returns the summed
/// element size.
pub fn fn_00e77950(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_bytes(e, descriptor.addr(), 0, 4, float_to_byte)
}

// Translated from 00e77b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): as `fn_00e77950`, with `n` from the `u16` +4 less 4 and each
/// component the low byte of a 4-byte source value.
pub fn fn_00e77b50(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_bytes(e, descriptor.addr(), 4, 4, low_byte)
}

// Translated from 00e77d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): as `fn_00e77b50` (the same code), with `n` from the `u16` +4
/// less 8.
pub fn fn_00e77d00(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_bytes(e, descriptor.addr(), 8, 4, low_byte)
}

// Translated from 00e77eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): as `fn_00e77b50`, with `n` from the `u16` +4 less 0xc and the
/// components read 2 bytes apart.
pub fn fn_00e77eb0(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_bytes(e, descriptor.addr(), 0xc, 2, low_byte)
}

// Translated from 00e78060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): as `fn_00e77eb0`, with `n` from the `u16` +4 less 0x10.
pub fn fn_00e78060(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_bytes(e, descriptor.addr(), 0x10, 2, low_byte)
}

// Translated from 00e78210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): as `fn_00e77b50`, with `n` from the `u16` +4 less 0x14 and the
/// components read 1 byte apart.
pub fn fn_00e78210(e: &mut Engine, descriptor: Ptr) -> u32 {
    pack_bytes(e, descriptor.addr(), 0x14, 1, low_byte)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_83b0, fn_00e783b0(Ptr) -> u32),
        entry!(0x00e7_6c30, fn_00e76c30(Ptr) -> u32),
        entry!(0x00e7_6e30, fn_00e76e30(Ptr) -> u32),
        entry!(0x00e7_7010, fn_00e77010(Ptr) -> u32),
        entry!(0x00e7_71f0, fn_00e771f0(Ptr) -> u32),
        entry!(0x00e7_73b0, fn_00e773b0(Ptr) -> u32),
        entry!(0x00e7_7590, fn_00e77590(Ptr) -> u32),
        entry!(0x00e7_7770, fn_00e77770(Ptr) -> u32),
        entry!(0x00e7_7950, fn_00e77950(Ptr) -> u32),
        entry!(0x00e7_7b50, fn_00e77b50(Ptr) -> u32),
        entry!(0x00e7_7d00, fn_00e77d00(Ptr) -> u32),
        entry!(0x00e7_7eb0, fn_00e77eb0(Ptr) -> u32),
        entry!(0x00e7_8060, fn_00e78060(Ptr) -> u32),
        entry!(0x00e7_8210, fn_00e78210(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, a| a[0].into_ret());
        e.register(MEMSET, |_, a| a[0].into_ret());
        let d = e.mem.alloc(0x30);
        e.mem.set_u16(d + 8, 2);
        e.mem.set_u32(d + 0x1c, 6);
        e.mem.set_u32(d + 0x20, 0x10);
        e.mem.set_u32(d + 0x18, 0x20);
        e.mem.set_u32(d + 0x2c, 0x5000);
        e.call_log = Some(vec![]);
        (e, d)
    }

    #[test]
    fn without_a_source_clears_each_element() {
        let (mut e, d) = rig();
        assert_eq!(e.call(0x00e7_83b0, &args![d]).u32(), 12);
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![vec![0x5000, 0, 6], vec![0x5010, 0, 6]]
        );
        e.mem.set_u16(d + 8, 0);
        assert_eq!(e.call(0x00e7_83b0, &args![d]).u32(), 0);
    }

    #[test]
    fn with_a_source_copies_and_pads_to_four_bytes() {
        let (mut e, d) = rig();
        e.mem.set_u32(d + 0x10, 0x8000);
        e.mem.set_u16(d + 4, 0x1a);
        assert_eq!(e.call(0x00e7_83b0, &args![d]).u32(), 12);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x5000, 0x8000, 2], vec![0x5010, 0x8020, 2]]
        );
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![vec![0x5002, 0, 2], vec![0x5012, 0, 2]]
        );
    }

    #[test]
    fn large_kinds_copy_four_bytes_without_padding() {
        let (mut e, d) = rig();
        e.mem.set_u32(d + 0x10, 0x8000);
        // 0x10 - 0x18 wraps to a large `u16`: four bytes.
        e.mem.set_u16(d + 4, 0x10);
        e.call(0x00e7_83b0, &args![d]);
        assert_eq!(calls_to(&e, MEMCPY)[0], vec![0x5000, 0x8000, 4]);
        assert!(calls_to(&e, MEMSET).is_empty());
    }

    #[test]
    fn with_an_index_array_reads_the_indexed_elements() {
        let (mut e, d) = rig();
        e.mem.set_u32(d + 0x10, 0x8000);
        e.mem.set_u16(d + 4, 0x1c);
        let index = e.mem.alloc(4);
        e.mem.set_u16(index, 3);
        e.mem.set_u16(index + 2, 1);
        e.mem.set_u32(d + 0xc, index);
        assert_eq!(e.call(0x00e7_83b0, &args![d]).u32(), 12);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![0x5000, 0x8060, 4], vec![0x5010, 0x8020, 4]]
        );
    }
}

#[cfg(test)]
mod n010_tests {
    use super::*;

    /// An engine whose `memcpy`, `memset` and `_ftol2` doubles do the work, and a descriptor
    /// of two elements (format kind `kind`, `components` in +4) over `source` bytes with
    /// source stride 0x10, element size 6 and destination stride 0x10.
    fn rig(kind: u32, components: u16, source: &[u8]) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(MEMCPY, |e, a| {
            for k in 0..a[2] {
                let b = e.mem.u8(a[1] + k);
                e.mem.set_u8(a[0] + k, b);
            }
            a[0].into_ret()
        });
        e.register(MEMSET, |e, a| {
            for k in 0..a[2] {
                e.mem.set_u8(a[0] + k, a[1] as u8);
            }
            a[0].into_ret()
        });
        e.register(FTOL, |_, a| {
            let value = f64::take(a, &mut 0);
            (value as i32 as u32).into_ret()
        });
        let d = e.mem.alloc(0x30);
        e.mem.set_u32(d, kind);
        e.mem.set_u16(d + 4, components);
        e.mem.set_u16(d + 8, 2);
        e.mem.set_u32(d + 0x18, 0x10);
        e.mem.set_u32(d + 0x1c, 6);
        e.mem.set_u32(d + 0x20, 0x10);
        let from = e.mem.alloc(0x40);
        e.mem.write(from, source);
        e.mem.set_u32(d + 0x10, from);
        let to = e.mem.alloc(0x20);
        e.mem.write(to, &[0xee; 0x20]);
        e.mem.set_u32(d + 0x2c, to);
        e.call_log = Some(vec![]);
        (e, d, to)
    }

    fn bytes(e: &Engine, at: u32, n: u32) -> Vec<u8> {
        (0..n).map(|k| e.mem.u8(at + k)).collect()
    }

    fn floats(values: &[f32]) -> Vec<u8> {
        let mut out = vec![];
        for v in values {
            out.extend(v.to_le_bytes());
        }
        out
    }

    /// Source words 0x8811 to 0x8814 (one per 4 bytes), then 0x8821 to 0x8824 in the second
    /// element.
    fn words() -> Vec<u8> {
        let mut out = vec![];
        for v in [
            0x8811u32, 0x8812, 0x8813, 0x8814, 0x8821, 0x8822, 0x8823, 0x8824,
        ] {
            out.extend(v.to_le_bytes());
        }
        out
    }

    #[test]
    fn fn_00e76c30_truncates_floats_to_shorts_and_zero_pads() {
        let source = floats(&[1.9, -2.5, 3.0, 0.0, 7.5, 8.25, 9.0, 0.0]);
        let (mut e, d, to) = rig(6, 3, &source);
        // Kind 6: width 2, so two components.
        assert_eq!(e.call(0x00e7_6c30, &args![d]).u32(), 12);
        assert_eq!(bytes(&e, to, 6), vec![1, 0, 0xfe, 0xff, 0xee, 0xee]);
        assert_eq!(bytes(&e, to + 0x10, 4), vec![7, 0, 8, 0]);
        // Another kind: width 4, one component, three zeroed.
        let (mut e, d, to) = rig(1, 1, &source);
        e.call(0x00e7_6c30, &args![d]);
        assert_eq!(bytes(&e, to, 10), vec![1, 0, 0, 0, 0, 0, 0, 0, 0xee, 0xee]);
    }

    #[test]
    fn fn_00e76c30_without_a_source_clears_the_elements() {
        let (mut e, d, to) = rig(1, 1, &[]);
        e.mem.set_u32(d + 0x10, 0);
        assert_eq!(e.call(0x00e7_6c30, &args![d]).u32(), 12);
        assert_eq!(bytes(&e, to, 7), vec![0, 0, 0, 0, 0, 0, 0xee]);
        e.mem.set_u16(d + 8, 0);
        assert_eq!(e.call(0x00e7_6c30, &args![d]).u32(), 0);
    }

    #[test]
    fn fn_00e76e30_copies_low_halves_with_an_index_array() {
        let (mut e, d, to) = rig(1, 4 + 2, &words());
        let index = e.mem.alloc(4);
        e.mem.set_u16(index, 1);
        e.mem.set_u16(index + 2, 0);
        e.mem.set_u32(d + 0xc, index);
        assert_eq!(e.call(0x00e7_6e30, &args![d]).u32(), 12);
        assert_eq!(bytes(&e, to, 8), vec![0x21, 0x88, 0x22, 0x88, 0, 0, 0, 0]);
        assert_eq!(bytes(&e, to + 0x10, 4), vec![0x11, 0x88, 0x12, 0x88]);
    }

    #[test]
    fn fn_00e77010_skips_eight() {
        let (mut e, d, to) = rig(1, 8 + 1, &words());
        e.call(0x00e7_7010, &args![d]);
        assert_eq!(bytes(&e, to, 8), vec![0x11, 0x88, 0, 0, 0, 0, 0, 0]);
        // Fewer than 8 components wrap to a large count: the full width.
        let (mut e, d, to) = rig(1, 2, &words());
        e.call(0x00e7_7010, &args![d]);
        assert_eq!(
            bytes(&e, to, 8),
            vec![0x11, 0x88, 0x12, 0x88, 0x13, 0x88, 0x14, 0x88]
        );
    }

    #[test]
    fn fn_00e771f0_copies_shorts_with_memcpy() {
        let source = [
            1u8, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 9, 10, 11, 12,
        ];
        let (mut e, d, to) = rig(1, 0xc + 2, &source);
        assert_eq!(e.call(0x00e7_71f0, &args![d]).u32(), 12);
        assert_eq!(bytes(&e, to, 9), vec![1, 2, 3, 4, 0, 0, 0, 0, 0xee]);
        assert_eq!(bytes(&e, to + 0x10, 4), vec![9, 10, 11, 12]);
        let log = e.call_log.take().unwrap();
        let copies: Vec<_> = log.iter().filter(|c| c.0 == MEMCPY).collect();
        assert_eq!(copies.len(), 2);
        assert_eq!(copies[0].1[2], 4);
    }

    #[test]
    fn fn_00e773b0_reads_shorts_two_bytes_apart() {
        let source = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let (mut e, d, to) = rig(6, 0x10 + 2, &source);
        e.call(0x00e7_73b0, &args![d]);
        assert_eq!(bytes(&e, to, 5), vec![1, 2, 3, 4, 0xee]);
    }

    #[test]
    fn fn_00e77590_widens_signed_bytes() {
        let source = [0x80u8, 0x7f, 3, 4];
        let (mut e, d, to) = rig(1, 0x14 + 2, &source);
        e.call(0x00e7_7590, &args![d]);
        assert_eq!(bytes(&e, to, 8), vec![0x80, 0xff, 0x7f, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn fn_00e77770_widens_unsigned_bytes() {
        let source = [0x80u8, 0x7f, 3, 4];
        let (mut e, d, to) = rig(1, 0x18 + 2, &source);
        e.call(0x00e7_7770, &args![d]);
        assert_eq!(bytes(&e, to, 8), vec![0x80, 0, 0x7f, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn fn_00e77950_chops_floats_to_bytes() {
        let source = floats(&[1.9, -2.5, 255.9, 300.0]);
        let (mut e, d, to) = rig(1, 3, &source);
        assert_eq!(e.call(0x00e7_7950, &args![d]).u32(), 12);
        // 1, -2 (0xfe), 255, then the fourth byte cleared.
        assert_eq!(bytes(&e, to, 5), vec![1, 0xfe, 0xff, 0, 0xee]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MEMSET, vec![to + 3, 0, 1])));
        // Four components: no clearing; 300 keeps its low byte.
        let (mut e, d, to) = rig(1, 4, &source);
        e.call(0x00e7_7950, &args![d]);
        assert_eq!(bytes(&e, to, 4), vec![1, 0xfe, 0xff, 0x2c]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == MEMSET));
    }

    #[test]
    fn fn_00e77b50_copies_low_bytes() {
        let (mut e, d, to) = rig(1, 4 + 2, &words());
        e.call(0x00e7_7b50, &args![d]);
        assert_eq!(bytes(&e, to, 5), vec![0x11, 0x12, 0, 0, 0xee]);
        assert_eq!(bytes(&e, to + 0x10, 2), vec![0x21, 0x22]);
    }

    #[test]
    fn fn_00e77d00_skips_eight() {
        let (mut e, d, to) = rig(1, 8, &words());
        e.call(0x00e7_7d00, &args![d]);
        // No components: the whole element is cleared.
        assert_eq!(bytes(&e, to, 5), vec![0, 0, 0, 0, 0xee]);
    }

    #[test]
    fn fn_00e77eb0_reads_bytes_two_apart() {
        let source = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let (mut e, d, to) = rig(1, 0xc + 4, &source);
        e.call(0x00e7_7eb0, &args![d]);
        assert_eq!(bytes(&e, to, 4), vec![1, 3, 5, 7]);
    }

    #[test]
    fn fn_00e78060_reads_bytes_two_apart() {
        let source = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let (mut e, d, to) = rig(1, 0x10 + 3, &source);
        e.call(0x00e7_8060, &args![d]);
        assert_eq!(bytes(&e, to, 4), vec![1, 3, 5, 0]);
    }

    #[test]
    fn fn_00e78210_reads_consecutive_bytes() {
        let source = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let (mut e, d, to) = rig(1, 0x14 + 2, &source);
        assert_eq!(e.call(0x00e7_8210, &args![d]).u32(), 12);
        assert_eq!(bytes(&e, to, 4), vec![1, 2, 0, 0]);
    }
}
