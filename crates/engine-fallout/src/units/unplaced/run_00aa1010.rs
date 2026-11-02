//! `(unplaced)/run_00aa1010` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Most of the run are thin cdecl wrappers around the Gamebryo allocator
//! object's virtual methods (vtable `+4`, `+8`, `+0x10`, `+0x14`, `+0x18`),
//! each passing its own constant code and fixed trailing words. The engine
//! map names none of them; what the codes and words mean is not confirmed,
//! so the translations pass them as the exe does and name only what the
//! bodies show.

#[allow(unused_imports)]
use crate::prelude::*;

/// The Gamebryo allocator reference: the word at `011f6080` points to a word
/// that holds the allocator object (set up by `00aa2020`).
const ALLOCATOR_REFERENCE: u32 = 0x011f_6080;
/// The allocator's allocation method, byte offset in its vtable.
const ALLOCATE_SLOT: u32 = 0x4;
/// The allocator's release method (block, code, -1).
const RELEASE_SLOT: u32 = 0x8;
/// The allocator's second allocation method (same arguments as
/// [`ALLOCATE_SLOT`]).
const ALLOCATE_SECOND_SLOT: u32 = 0x10;
/// The allocator's second release method (block, code, -1).
const RELEASE_SECOND_SLOT: u32 = 0x14;
/// The allocator's reallocation method (block, size address, second word
/// address, code, 0, -1, 0, -1, 0).
const REALLOCATE_SLOT: u32 = 0x18;

/// CRT `memcpy` (cdecl).
const MEMCPY: u32 = 0x00ec_44d0;

/// Import slots (KERNEL32): `QueryPerformanceCounter` and
/// `QueryPerformanceFrequency`.
const QUERY_PERFORMANCE_COUNTER: u32 = 0x00fd_f0a0;
const QUERY_PERFORMANCE_FREQUENCY: u32 = 0x00fd_f0a4;
/// Byte that is non-zero until the timer's first call (initially set in
/// the exe's data).
const TIMER_FIRST_CALL: u32 = 0x011a_c222;
/// The counter frequency and the counter at the first call (64-bit).
const TIMER_FREQUENCY: u32 = 0x011f_6070;
const TIMER_START: u32 = 0x011f_6068;

/// The allocator object (`**011f6080`).
fn allocator(e: &Engine) -> u32 {
    let reference = e.mem.u32(ALLOCATOR_REFERENCE);
    e.mem.u32(reference)
}

/// The shape shared by `00aa10b0` and `00aa1130`: `size` (0 made 1) and
/// `second` are passed by address, followed by `code`, 0, 0, -1, 0, to the
/// allocator method at `slot`; returns its `EAX`.
fn allocate_by_address(e: &mut Engine, slot: u32, size: u32, second: u32, code: u32) -> Ptr {
    let size = if size == 0 { 1 } else { size };
    let object = allocator(e);
    e.with_stack(8, |e, locals| {
        e.mem.set_u32(locals.addr(), size);
        e.mem.set_u32(locals.addr() + 4, second);
        e.vcall(
            object,
            slot,
            &args![
                locals.addr(),
                locals.addr() + 4,
                code,
                0u32,
                0u32,
                0xffff_ffffu32,
                0u32
            ],
        )
        .ptr()
    })
}

/// The shape shared by the release wrappers: nothing for null, otherwise
/// the allocator method at `slot` with the block, `code` and -1.
fn release(e: &mut Engine, slot: u32, block: Ptr, code: u32) {
    if block.is_null() {
        return;
    }
    let object = allocator(e);
    e.vcall(object, slot, &args![block, code, 0xffff_ffffu32]);
}

/// The shape shared by `00aa11b0` and `00aa1250`: a null-sized request on a
/// block releases it (slot `+0x14`, `release_code`) and returns 0; a
/// sized request on a block reallocates it (slot `+0x18`: block, size
/// address, second-word address, `reallocate_code`, 0, -1, 0, -1, 0); with
/// no block it allocates (slot `+0x10`: size, 0 made 1, and second word by
/// address, `allocate_code`, 0, 0, -1, 0). Returns the method's `EAX`.
fn resize(e: &mut Engine, block: Ptr, size: u32, second: u32, codes: [u32; 3]) -> Ptr {
    let [release_code, allocate_code, reallocate_code] = codes;
    if size == 0 && !block.is_null() {
        let object = allocator(e);
        e.vcall(
            object,
            RELEASE_SECOND_SLOT,
            &args![block, release_code, 0xffff_ffffu32],
        );
        return Ptr::new(0);
    }
    if size != 0 && !block.is_null() {
        let object = allocator(e);
        return e.with_stack(8, |e, locals| {
            e.mem.set_u32(locals.addr(), size);
            e.mem.set_u32(locals.addr() + 4, second);
            e.vcall(
                object,
                REALLOCATE_SLOT,
                &args![
                    block,
                    locals.addr(),
                    locals.addr() + 4,
                    reallocate_code,
                    0u32,
                    0xffff_ffffu32,
                    0u32,
                    0xffff_ffffu32,
                    0u32
                ],
            )
            .ptr()
        });
    }
    allocate_by_address(e, ALLOCATE_SECOND_SLOT, size, second, allocate_code)
}

// Translated from 00aa1010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets a `NiGlobalConstantEntry`'s data (`this` in ECX, `RET 8`; the only
/// caller is the constructor `00aa00d0`, whose vtable `010a2148` has that
/// RTTI name; the method name is not confirmed). When `size` is larger
/// than `m_uiDataSize` (+0x10, Xbox PDB), the buffer `m_pvDataSource`
/// (+0x14) is freed (`00aa10f0`), cleared and the size taken; a null buffer
/// is then allocated with `m_uiDataSize` bytes (`00aa1070`); a non-null
/// `source` is copied in (`memcpy`, `size` bytes).
pub fn fn_00aa1010(e: &mut Engine, this: Ptr, size: u32, source: Ptr) {
    // NiGlobalConstantEntry::m_uiDataSize (Xbox PDB) +0x10, m_pvDataSource +0x14.
    let data_size = this.addr() + 0x10;
    let data = this.addr() + 0x14;
    if e.mem.u32(data_size) < size {
        let old = Ptr::new(e.mem.u32(data));
        fn_00aa10f0(e, old);
        e.mem.set_u32(data, 0);
        e.mem.set_u32(data_size, size);
    }
    if e.mem.u32(data) == 0 {
        let wanted = e.mem.u32(data_size);
        let buffer = fn_00aa1070(e, wanted);
        e.mem.set_u32(data, buffer.addr());
    }
    if !source.is_null() {
        let buffer = e.mem.u32(data);
        e.call(MEMCPY, &args![buffer, source, size]);
    }
}

// Translated from 00aa1070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates `size` bytes (a request for 0 is made for 1) through the
/// Gamebryo allocator's virtual allocation method (vtable `+4`), the sibling
/// of `00aa13e0`; it passes 5, 0 where that passes 1, 1. The size and an
/// word holding 4 are passed by
/// address, as locals of this function. Returns the allocator's result
/// (`EAX`), which the decompiler does not show.
///
/// Arguments of the call, in order: size address, address of the 4, 5, 0,
/// 0, -1, 0 (their names are not known).
pub fn fn_00aa1070(e: &mut Engine, size: u32) -> Ptr {
    let size = if size == 0 { 1 } else { size };
    let reference = e.mem.u32(ALLOCATOR_REFERENCE);
    let allocator = e.mem.u32(reference);
    e.with_stack(8, |e, locals| {
        e.mem.set_u32(locals.addr(), size);
        e.mem.set_u32(locals.addr() + 4, 4);
        e.vcall(
            allocator,
            ALLOCATE_SLOT,
            &args![
                locals.addr(),
                locals.addr() + 4,
                5u32,
                0u32,
                0u32,
                0xffff_ffffu32,
                0u32
            ],
        )
        .ptr()
    })
}

// Translated from 00aa10b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates through the allocator's method at vtable `+4` (cdecl, two
/// words): `size` (0 made 1) and `second` are passed by address (the
/// function's own argument slots), then 7, 0, 0, -1, 0. Where `00aa1070`
/// passes the constant 4 by address, this passes its second argument.
/// Returns the method's `EAX`.
pub fn fn_00aa10b0(e: &mut Engine, size: u32, second: u32) -> Ptr {
    allocate_by_address(e, ALLOCATE_SLOT, size, second, 7)
}

// Translated from 00aa10f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a block through the allocator's method at vtable `+8` (block,
/// 9, -1); null does nothing. The release counterpart of `00aa1070`.
pub fn fn_00aa10f0(e: &mut Engine, block: Ptr) {
    release(e, RELEASE_SLOT, block, 9);
}

// Translated from 00aa1110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a block through the allocator's method at vtable `+8` (block,
/// 10, -1); null does nothing.
pub fn fn_00aa1110(e: &mut Engine, block: Ptr) {
    release(e, RELEASE_SLOT, block, 10);
}

// Translated from 00aa1130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates through the allocator's method at vtable `+0x10` (cdecl, two
/// words): `size` (0 made 1) and `second` by address, then 0xd, 0, 0, -1,
/// 0. Returns the method's `EAX`.
pub fn fn_00aa1130(e: &mut Engine, size: u32, second: u32) -> Ptr {
    allocate_by_address(e, ALLOCATE_SECOND_SLOT, size, second, 0xd)
}

// Translated from 00aa1170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a block through the allocator's method at vtable `+0x14`
/// (block, 0xf, -1); null does nothing.
pub fn fn_00aa1170(e: &mut Engine, block: Ptr) {
    release(e, RELEASE_SECOND_SLOT, block, 0xf);
}

// Translated from 00aa1190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a block through the allocator's method at vtable `+0x14`
/// (block, 0x10, -1); null does nothing.
pub fn fn_00aa1190(e: &mut Engine, block: Ptr) {
    release(e, RELEASE_SECOND_SLOT, block, 0x10);
}

// Translated from 00aa11b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resizes a block (cdecl, block and size), the shape of `realloc`: size 0
/// with a block releases it (vtable `+0x14`, code 0xf) and returns 0; a
/// size with a block reallocates (vtable `+0x18`, size and the constant 4
/// by address, code 0xc); no block allocates (vtable `+0x10`, size, 0 made
/// 1, and the 4 by address, code 0xb). Returns the method's `EAX`.
pub fn fn_00aa11b0(e: &mut Engine, block: Ptr, size: u32) -> Ptr {
    resize(e, block, size, 4, [0xf, 0xb, 0xc])
}

// Translated from 00aa1250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00aa11b0` with its own codes and the second word given by the caller
/// (cdecl, block, size, `second`): release code 0x10, reallocation code
/// 0xe, allocation code 0xd.
pub fn fn_00aa1250(e: &mut Engine, block: Ptr, size: u32, second: u32) -> Ptr {
    resize(e, block, size, second, [0x10, 0xd, 0xe])
}

// Translated from 00aa12f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Seconds since the first call, as a `float` (returned in ST0). The first
/// call (byte `011ac222` non-zero) stores the counter frequency
/// (`011f6070`) and the counter (`011f6068`) and clears the byte; every
/// call reads the counter and divides the 64-bit difference from the first
/// counter by the frequency (both loaded as signed 64-bit integers), stored
/// as a `float`. Computed in `f64`: for differences beyond 2^53 the
/// integer load can round where `FILD` is exact.
pub fn fn_00aa12f0(e: &mut Engine) -> f32 {
    if e.mem.u8(TIMER_FIRST_CALL) != 0 {
        e.call(QUERY_PERFORMANCE_FREQUENCY, &args![TIMER_FREQUENCY]);
        e.call(QUERY_PERFORMANCE_COUNTER, &args![TIMER_START]);
        e.mem.set_u8(TIMER_FIRST_CALL, 0);
    }
    let now = e.with_stack(8, |e, counter| {
        e.call(QUERY_PERFORMANCE_COUNTER, &args![counter]);
        e.mem.u64(counter.addr())
    });
    let elapsed = now.wrapping_sub(e.mem.u64(TIMER_START)) as i64;
    let frequency = e.mem.u64(TIMER_FREQUENCY) as i64;
    (elapsed as f64 / frequency as f64) as f32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa1010, fn_00aa1010(Ptr, u32, Ptr)),
        entry!(0x00aa1070, fn_00aa1070(u32) -> Ptr),
        entry!(0x00aa10b0, fn_00aa10b0(u32, u32) -> Ptr),
        entry!(0x00aa10f0, fn_00aa10f0(Ptr)),
        entry!(0x00aa1110, fn_00aa1110(Ptr)),
        entry!(0x00aa1130, fn_00aa1130(u32, u32) -> Ptr),
        entry!(0x00aa1170, fn_00aa1170(Ptr)),
        entry!(0x00aa1190, fn_00aa1190(Ptr)),
        entry!(0x00aa11b0, fn_00aa11b0(Ptr, u32) -> Ptr),
        entry!(0x00aa1250, fn_00aa1250(Ptr, u32, u32) -> Ptr),
        entry!(0x00aa12f0, fn_00aa12f0() -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATE: u32 = 0x0070_0000;
    const VTABLE: u32 = 0x0300_0000;

    type Seen = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    /// An allocator whose allocation method records the values behind the two
    /// address arguments and the remaining arguments, and returns 0x4242.
    fn engine_with_allocator() -> (Engine, Seen) {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        let allocator = e.mem.alloc(8);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, allocator);
        e.mem.set_u32(ALLOCATOR_REFERENCE, holder);
        e.put_vtable(VTABLE, &[0, ALLOCATE]);
        e.mem.set_u32(allocator, VTABLE);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(ALLOCATE, move |e, a| {
            let mut words = vec![a[0], e.mem.u32(a[1]), e.mem.u32(a[2])];
            words.extend_from_slice(&a[3..]);
            log.borrow_mut().push(words);
            Ret {
                eax: 0x4242,
                ..Ret::default()
            }
        });
        (e, seen)
    }

    #[test]
    fn allocates_through_the_virtual_method() {
        let (mut e, seen) = engine_with_allocator();
        let allocator = e.mem.u32(e.mem.u32(ALLOCATOR_REFERENCE));
        let result = e.call(0x00aa_1070, &args![100u32]).u32();
        assert_eq!(result, 0x4242);
        assert_eq!(
            seen.borrow()[0],
            vec![allocator, 100, 4, 5, 0, 0, 0xffff_ffff, 0]
        );
    }

    #[test]
    fn a_zero_size_is_requested_as_one() {
        let (mut e, seen) = engine_with_allocator();
        e.call(0x00aa_1070, &args![0u32]);
        assert_eq!(seen.borrow()[0][1], 1);
    }

    /// Method addresses of the full allocator vtable used below.
    const METHODS: [u32; 7] = [
        0,
        0x0070_1004,
        0x0070_1008,
        0,
        0x0070_1010,
        0x0070_1014,
        0x0070_1018,
    ];

    /// An allocator with every method of the run: each records its slot and
    /// its words with the address arguments read (`+4`/`+0x10`: words 1 and
    /// 2; `+0x18`: words 2 and 3), and returns `0x5000 + slot`.
    fn engine_with_methods() -> (Engine, Seen, u32) {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        let object = e.mem.alloc(8);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, object);
        e.mem.set_u32(ALLOCATOR_REFERENCE, holder);
        e.put_vtable(VTABLE, &METHODS);
        e.mem.set_u32(object, VTABLE);
        let seen: Seen = Default::default();
        for (index, method) in METHODS.iter().enumerate() {
            if *method == 0 {
                continue;
            }
            let slot = 4 * index as u32;
            let log = seen.clone();
            e.register_double(*method, move |e, a| {
                let mut words = vec![slot];
                words.extend_from_slice(a);
                match slot {
                    0x4 | 0x10 => {
                        words[2] = e.mem.u32(a[1]);
                        words[3] = e.mem.u32(a[2]);
                    }
                    0x18 => {
                        words[3] = e.mem.u32(a[2]);
                        words[4] = e.mem.u32(a[3]);
                    }
                    _ => {}
                }
                log.borrow_mut().push(words);
                Ret {
                    eax: 0x5000 + slot,
                    ..Ret::default()
                }
            });
        }
        (e, seen, object)
    }

    const MINUS_ONE: u32 = 0xffff_ffff;

    #[test]
    fn set_data_grows_allocates_and_copies() {
        let (mut e, seen, object) = engine_with_methods();
        e.register(MEMCPY, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let entry = e.mem.alloc(0x18);
        e.mem.set_u32(entry + 0x10, 8);
        e.mem.set_u32(entry + 0x14, 0x9000);
        e.call_log = Some(vec![]);
        // Larger: frees the old buffer, allocates the new size, copies.
        e.call(0x00aa_1010, &args![entry, 16u32, 0x7777u32]);
        assert_eq!(seen.borrow()[0], vec![0x8, object, 0x9000, 9, MINUS_ONE]);
        assert_eq!(
            seen.borrow()[1],
            vec![0x4, object, 16, 4, 5, 0, 0, MINUS_ONE, 0]
        );
        assert_eq!(e.mem.u32(entry + 0x10), 16);
        assert_eq!(e.mem.u32(entry + 0x14), 0x5004);
        let copies: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMCPY)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(copies, vec![vec![0x5004, 0x7777, 16]]);
        // Smaller with a buffer and no source: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x00aa_1010, &args![entry, 4u32, 0u32]);
        assert_eq!(seen.borrow().len(), 2);
        assert_eq!(e.mem.u32(entry + 0x10), 16);
    }

    #[test]
    fn allocation_wrappers_pass_their_codes() {
        let (mut e, seen, object) = engine_with_methods();
        assert_eq!(e.call(0x00aa_10b0, &args![0u32, 16u32]).u32(), 0x5004);
        assert_eq!(e.call(0x00aa_1130, &args![24u32, 8u32]).u32(), 0x5010);
        assert_eq!(
            *seen.borrow(),
            vec![
                vec![0x4, object, 1, 16, 7, 0, 0, MINUS_ONE, 0],
                vec![0x10, object, 24, 8, 0xd, 0, 0, MINUS_ONE, 0],
            ]
        );
    }

    #[test]
    fn release_wrappers_skip_null_and_pass_their_codes() {
        let (mut e, seen, object) = engine_with_methods();
        for address in [0x00aa_10f0, 0x00aa_1110, 0x00aa_1170, 0x00aa_1190] {
            e.call(address, &args![0u32]);
        }
        assert!(seen.borrow().is_empty());
        for address in [0x00aa_10f0, 0x00aa_1110, 0x00aa_1170, 0x00aa_1190] {
            e.call(address, &args![0x1234u32]);
        }
        assert_eq!(
            *seen.borrow(),
            vec![
                vec![0x8, object, 0x1234, 9, MINUS_ONE],
                vec![0x8, object, 0x1234, 10, MINUS_ONE],
                vec![0x14, object, 0x1234, 0xf, MINUS_ONE],
                vec![0x14, object, 0x1234, 0x10, MINUS_ONE],
            ]
        );
    }

    #[test]
    fn resize_releases_reallocates_or_allocates() {
        let (mut e, seen, object) = engine_with_methods();
        assert_eq!(e.call(0x00aa_11b0, &args![0x1234u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00aa_11b0, &args![0x1234u32, 32u32]).u32(), 0x5018);
        assert_eq!(e.call(0x00aa_11b0, &args![0u32, 0u32]).u32(), 0x5010);
        assert_eq!(e.call(0x00aa_11b0, &args![0u32, 40u32]).u32(), 0x5010);
        assert_eq!(
            *seen.borrow(),
            vec![
                vec![0x14, object, 0x1234, 0xf, MINUS_ONE],
                vec![0x18, object, 0x1234, 32, 4, 0xc, 0, MINUS_ONE, 0, MINUS_ONE, 0],
                vec![0x10, object, 1, 4, 0xb, 0, 0, MINUS_ONE, 0],
                vec![0x10, object, 40, 4, 0xb, 0, 0, MINUS_ONE, 0],
            ]
        );
    }

    #[test]
    fn resize_with_second_word_uses_its_codes() {
        let (mut e, seen, object) = engine_with_methods();
        assert_eq!(e.call(0x00aa_1250, &args![0x1234u32, 0u32, 16u32]).u32(), 0);
        e.call(0x00aa_1250, &args![0x1234u32, 32u32, 16u32]);
        e.call(0x00aa_1250, &args![0u32, 0u32, 16u32]);
        assert_eq!(
            *seen.borrow(),
            vec![
                vec![0x14, object, 0x1234, 0x10, MINUS_ONE],
                vec![0x18, object, 0x1234, 32, 16, 0xe, 0, MINUS_ONE, 0, MINUS_ONE, 0],
                vec![0x10, object, 1, 16, 0xd, 0, 0, MINUS_ONE, 0],
            ]
        );
    }

    #[test]
    fn timer_starts_on_first_call_and_divides_by_the_frequency() {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.map(0x011f_6000, 0x1000);
        e.mem.set_u8(TIMER_FIRST_CALL, 1);
        e.register(QUERY_PERFORMANCE_FREQUENCY, |e, a| {
            e.mem.set_u64(a[0], 1000);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let mut counter = 5_000u64;
        e.register_double(QUERY_PERFORMANCE_COUNTER, move |e, a| {
            e.mem.set_u64(a[0], counter);
            counter += 2_500;
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        // First call: start 5000, now 7500 -> 2.5 s.
        assert_eq!(e.call(0x00aa_12f0, &[]).f32(), 2.5);
        assert_eq!(e.mem.u8(TIMER_FIRST_CALL), 0);
        assert_eq!(e.mem.u64(TIMER_START), 5_000);
        // Second call: now 10000 -> 5 s, no new start.
        assert_eq!(e.call(0x00aa_12f0, &[]).f32(), 5.0);
        assert_eq!(e.mem.u64(TIMER_START), 5_000);
    }

    // Moved from the tests of `units/nisystem/allocator_support.rs` with the engine
    // split: it runs the real NiSystem, BSCore and unplaced functions together, and this
    // crate is the one whose tests link all three.
    #[test]
    fn initialization_uses_real_constructors_and_global_holder() {
        let mut e = Engine::new();
        e.map(0x011f6000, 0x1000);
        e.put_vtable(0x010a2528, &[0, 0x00aa2240, 0, 0, 0, 0, 0, 0x00a29680]);
        let allocator = e.mem.alloc(4);
        let source = e.mem.alloc(8);
        e.call(0x00aa2230, &[allocator]);
        e.call(0x00aa21d0, &[source, allocator]);
        e.mem.set_u32(0x011f6118, source);
        e.call_log = Some(Vec::new());
        e.call(0x00aa2020, &[]);
        let holder = e.mem.u32(0x011f6080);
        // The platform heap rounds requests to eight-byte blocks. Check
        // the actual native request separately from the backing block size.
        assert_eq!(e.mem.block_size(holder), Some(8));
        assert_eq!(e.mem.u32(holder), allocator);
        // Actual translated allocation wrapper, virtual method, memory
        // manager and memset; no game-function doubles in this test.
        let bytes = e.call(0x00aa1070, &[13]).u32();
        assert_eq!(e.mem.block_size(bytes), Some(16));
        assert_eq!(e.mem.bytes(bytes, 13), [0; 13]);
        let requests: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00aa3e40)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(requests, [vec![0x011f6238, 4], vec![0x011f6238, 13]]);
    }
}
