//! `Havok SDK/hksystemmemoryblockserver.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkSystemMemoryBlockServer` hands out blocks from a heap allocator (the object at `+0x10`;
//! its vtable slot `+4` allocates `(size)` and slot `+8` frees `(block, size)`) while counting
//! the bytes in use (`+8`) against an optional limit (`+0xc`, 0 for none). The class is not
//! in the Xbox PDB: the names of its fields are descriptive and unconfirmed.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkSystemMemoryBlockServer` (engine map name; not in the Xbox PDB), 0x14 bytes: the
    /// vtable, then these words. Their names are descriptive, not confirmed.
    pub struct HkSystemMemoryBlockServer: 0x14 {
        /// The size `recommendSize` falls back to when there is no limit and the request is
        /// below it.
        0x04 m_recommendedSize: u32,
        /// The bytes handed out and not yet returned.
        0x08 m_memoryUsed: u32,
        /// The limit on `m_memoryUsed`; 0 means no limit.
        0x0C m_memoryLimit: u32,
        /// The allocator the blocks come from (an object with a vtable).
        0x10 m_allocator: u32,
    }
}

// Translated from 00d603f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSystemMemoryBlockServer::recommendSize` (engine map name), thiscall, one stack word
/// `size`. With a limit it is the room left (`limit - used`) when that is at least `size`
/// (unsigned), otherwise `size`. Without one it is the word at `+4` when `size` is below it
/// (unsigned), otherwise `size` rounded up to a multiple of `0x8000`
/// (`(size + 0x7fff) & 0xffff8000`).
pub fn hk_system_memory_block_server_recommend_size(
    e: &mut Engine,
    this: Ptr<HkSystemMemoryBlockServer>,
    size: u32,
) -> u32 {
    let limit = e.get(this, HkSystemMemoryBlockServer::m_memoryLimit);
    if limit != 0 {
        let room = limit.wrapping_sub(e.get(this, HkSystemMemoryBlockServer::m_memoryUsed));
        if room >= size {
            return room;
        }
        return size;
    }
    let base = e.get(this, HkSystemMemoryBlockServer::m_recommendedSize);
    if size < base {
        return base;
    }
    size.wrapping_add(0x7fff) & 0xffff_8000
}

// Translated from 00d60420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSystemMemoryBlockServer::allocate` (engine map name), thiscall, two stack words `(size,
/// granted)`: with a limit, answers 0 when `used + size` is above it (unsigned). Otherwise
/// stores `size` through `granted`, adds it to the bytes in use and returns what the
/// allocator's slot `+4` gives for `size`.
pub fn hk_system_memory_block_server_allocate(
    e: &mut Engine,
    this: Ptr<HkSystemMemoryBlockServer>,
    size: u32,
    granted: Ptr,
) -> u32 {
    let limit = e.get(this, HkSystemMemoryBlockServer::m_memoryLimit);
    let used = e.get(this, HkSystemMemoryBlockServer::m_memoryUsed);
    if limit != 0 && used.wrapping_add(size) > limit {
        return 0;
    }
    e.mem.set_u32(granted.addr(), size);
    e.set(
        this,
        HkSystemMemoryBlockServer::m_memoryUsed,
        used.wrapping_add(size),
    );
    let allocator = e.get(this, HkSystemMemoryBlockServer::m_allocator);
    e.vcall(allocator, 4, &args![size]).u32()
}

// Translated from 00d60460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSystemMemoryBlockServer::free` (engine map name), thiscall, two stack words `(block,
/// size)`: takes `size` off the bytes in use and jumps to the allocator's slot `+8` with the
/// same two words; returns what that returns.
pub fn hk_system_memory_block_server_free(
    e: &mut Engine,
    this: Ptr<HkSystemMemoryBlockServer>,
    block: Ptr,
    size: u32,
) -> u32 {
    let used = e.get(this, HkSystemMemoryBlockServer::m_memoryUsed);
    e.set(
        this,
        HkSystemMemoryBlockServer::m_memoryUsed,
        used.wrapping_sub(size),
    );
    let allocator = e.get(this, HkSystemMemoryBlockServer::m_allocator);
    e.vcall(allocator, 8, &args![block, size]).u32()
}

// Translated from 00d60480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSystemMemoryBlockServer::getTotalAvailableMemory` (engine map name), `__fastcall`: -1
/// without a limit, otherwise `limit - used`.
pub fn hk_system_memory_block_server_get_total_available_memory(
    e: &mut Engine,
    this: Ptr<HkSystemMemoryBlockServer>,
) -> i32 {
    let limit = e.get(this, HkSystemMemoryBlockServer::m_memoryLimit);
    if limit == 0 {
        return -1;
    }
    limit.wrapping_sub(e.get(this, HkSystemMemoryBlockServer::m_memoryUsed)) as i32
}

// Translated from 00d60490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSystemMemoryBlockServer::getMemoryLimit` (engine map name), `__fastcall`: the limit.
pub fn hk_system_memory_block_server_get_memory_limit(
    e: &mut Engine,
    this: Ptr<HkSystemMemoryBlockServer>,
) -> u32 {
    e.get(this, HkSystemMemoryBlockServer::m_memoryLimit)
}

// Translated from 00d604a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSystemMemoryBlockServer::setMemoryLimit` (engine map name), thiscall, two stack words
/// `(hkBool result, limit)`: a limit of 0 removes the limit (result true); a limit below
/// the bytes in use (unsigned) is refused (result false); otherwise it is stored (true).
/// Returns the `hkBool` pointer.
pub fn hk_system_memory_block_server_set_memory_limit(
    e: &mut Engine,
    this: Ptr<HkSystemMemoryBlockServer>,
    result: Ptr,
    limit: u32,
) -> Ptr {
    if limit == 0 {
        e.set(this, HkSystemMemoryBlockServer::m_memoryLimit, 0);
        e.mem.set_u8(result.addr(), 1);
        return result;
    }
    if e.get(this, HkSystemMemoryBlockServer::m_memoryUsed) > limit {
        e.mem.set_u8(result.addr(), 0);
        return result;
    }
    e.set(this, HkSystemMemoryBlockServer::m_memoryLimit, limit);
    e.mem.set_u8(result.addr(), 1);
    result
}

// Translated from 00d604e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSystemMemoryBlockServer::criticalAlloc` (engine map name), `__fastcall`: a jump to the
/// allocator's slot `+4`, so the one stack word `size` goes straight to it; returns its
/// result.
pub fn hk_system_memory_block_server_critical_alloc(
    e: &mut Engine,
    this: Ptr<HkSystemMemoryBlockServer>,
    size: u32,
) -> u32 {
    let allocator = e.get(this, HkSystemMemoryBlockServer::m_allocator);
    e.vcall(allocator, 4, &args![size]).u32()
}

// Translated from 00d604f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSystemMemoryBlockServer::criticalFree` (engine map name), `__fastcall`: a jump to the
/// allocator's slot `+8`, so the two stack words `(block, size)` go straight to it; returns
/// its result.
pub fn hk_system_memory_block_server_critical_free(
    e: &mut Engine,
    this: Ptr<HkSystemMemoryBlockServer>,
    block: Ptr,
    size: u32,
) -> u32 {
    let allocator = e.get(this, HkSystemMemoryBlockServer::m_allocator);
    e.vcall(allocator, 8, &args![block, size]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d6_03f0,
            hk_system_memory_block_server_recommend_size(
                Ptr<HkSystemMemoryBlockServer>,
                u32,
            ) -> u32
        ),
        entry!(
            0x00d6_0420,
            hk_system_memory_block_server_allocate(Ptr<HkSystemMemoryBlockServer>, u32, Ptr) -> u32
        ),
        entry!(
            0x00d6_0460,
            hk_system_memory_block_server_free(Ptr<HkSystemMemoryBlockServer>, Ptr, u32) -> u32
        ),
        entry!(
            0x00d6_0480,
            hk_system_memory_block_server_get_total_available_memory(
                Ptr<HkSystemMemoryBlockServer>,
            ) -> i32
        ),
        entry!(
            0x00d6_0490,
            hk_system_memory_block_server_get_memory_limit(Ptr<HkSystemMemoryBlockServer>) -> u32
        ),
        entry!(
            0x00d6_04a0,
            hk_system_memory_block_server_set_memory_limit(
                Ptr<HkSystemMemoryBlockServer>,
                Ptr,
                u32,
            ) -> Ptr
        ),
        entry!(
            0x00d6_04e0,
            hk_system_memory_block_server_critical_alloc(
                Ptr<HkSystemMemoryBlockServer>,
                u32,
            ) -> u32
        ),
        entry!(
            0x00d6_04f0,
            hk_system_memory_block_server_critical_free(
                Ptr<HkSystemMemoryBlockServer>,
                Ptr,
                u32,
            ) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATE_SLOT: u32 = 0x0900_0080;
    const FREE_SLOT: u32 = 0x0900_0081;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A block server over an allocator whose slots log `(slot, words)` and answer
    /// `0x7000` (allocate) and `0x7100` (free). Returns the server and the log.
    fn server(e: &mut Engine, recommended: u32, used: u32, limit: u32) -> (u32, Calls) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 4, recommended);
        e.mem.set_u32(this + 8, used);
        e.mem.set_u32(this + 0xc, limit);
        e.mem.set_u32(this + 0x10, allocator);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for (slot, answer) in [(ALLOCATE_SLOT, 0x7000u32), (FREE_SLOT, 0x7100)] {
            let log = Rc::clone(&calls);
            e.register_double(slot, move |_, a| {
                log.borrow_mut().push((slot, a.to_vec()));
                Ret {
                    eax: answer,
                    ..Ret::default()
                }
            });
        }
        (this, calls)
    }

    #[test]
    fn recommend_size_without_a_limit_rounds_up_to_32k() {
        let mut e = Engine::new();
        let (this, _) = server(&mut e, 0x10000, 0, 0);
        for (size, want) in [
            (0u32, 0x10000u32),
            (0xffff, 0x10000),
            // At the base size or above: rounded up to a multiple of 0x8000.
            (0x10000, 0x10000),
            (0x10001, 0x18000),
            (0x20000, 0x20000),
            (0x2ffff, 0x30000),
        ] {
            assert_eq!(
                e.call(0x00d6_03f0, &args![this, size]).u32(),
                want,
                "{size:x}"
            );
        }
    }

    #[test]
    fn recommend_size_with_a_limit_is_the_room_left_or_the_request() {
        let mut e = Engine::new();
        let (this, _) = server(&mut e, 0x10000, 0x1000, 0x5000);
        // Room left is 0x4000: kept when at least the request, else the request itself.
        assert_eq!(e.call(0x00d6_03f0, &args![this, 0x100u32]).u32(), 0x4000);
        assert_eq!(e.call(0x00d6_03f0, &args![this, 0x4000u32]).u32(), 0x4000);
        assert_eq!(e.call(0x00d6_03f0, &args![this, 0x4001u32]).u32(), 0x4001);
    }

    #[test]
    fn allocate_counts_the_bytes_and_asks_the_allocator() {
        let mut e = Engine::new();
        let (this, calls) = server(&mut e, 0x10000, 0x100, 0);
        let granted = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00d6_0420, &args![this, 0x40u32, granted]).u32(),
            0x7000
        );
        assert_eq!(e.mem.u32(granted), 0x40);
        assert_eq!(e.mem.u32(this + 8), 0x140);
        let allocator = e.mem.u32(this + 0x10);
        assert_eq!(
            *calls.borrow(),
            vec![(ALLOCATE_SLOT, vec![allocator, 0x40])]
        );
    }

    #[test]
    fn allocate_refuses_what_would_pass_the_limit() {
        let mut e = Engine::new();
        let (this, calls) = server(&mut e, 0x10000, 0x100, 0x200);
        let granted = e.mem.alloc(4);
        e.mem.set_u32(granted, 0xaaaa);
        // 0x100 + 0x101 is above 0x200.
        assert_eq!(
            e.call(0x00d6_0420, &args![this, 0x101u32, granted]).u32(),
            0
        );
        assert_eq!(e.mem.u32(granted), 0xaaaa);
        assert_eq!(e.mem.u32(this + 8), 0x100);
        assert!(calls.borrow().is_empty());
        // Exactly up to the limit is fine.
        assert_eq!(
            e.call(0x00d6_0420, &args![this, 0x100u32, granted]).u32(),
            0x7000
        );
        assert_eq!(e.mem.u32(this + 8), 0x200);
    }

    #[test]
    fn free_returns_the_bytes_and_forwards_to_the_allocator() {
        let mut e = Engine::new();
        let (this, calls) = server(&mut e, 0x10000, 0x300, 0);
        assert_eq!(
            e.call(0x00d6_0460, &args![this, 0x5000u32, 0x100u32]).u32(),
            0x7100
        );
        assert_eq!(e.mem.u32(this + 8), 0x200);
        let allocator = e.mem.u32(this + 0x10);
        assert_eq!(
            *calls.borrow(),
            vec![(FREE_SLOT, vec![allocator, 0x5000, 0x100])]
        );
    }

    #[test]
    fn the_critical_functions_go_straight_to_the_allocator() {
        let mut e = Engine::new();
        let (this, calls) = server(&mut e, 0x10000, 0x300, 0x400);
        assert_eq!(e.call(0x00d6_04e0, &args![this, 0x40u32]).u32(), 0x7000);
        assert_eq!(
            e.call(0x00d6_04f0, &args![this, 0x5000u32, 0x40u32]).u32(),
            0x7100
        );
        let allocator = e.mem.u32(this + 0x10);
        assert_eq!(
            *calls.borrow(),
            vec![
                (ALLOCATE_SLOT, vec![allocator, 0x40]),
                (FREE_SLOT, vec![allocator, 0x5000, 0x40])
            ]
        );
        // Neither touches the byte count.
        assert_eq!(e.mem.u32(this + 8), 0x300);
    }

    #[test]
    fn available_memory_and_limit_getters() {
        let mut e = Engine::new();
        let (unlimited, _) = server(&mut e, 0, 0x300, 0);
        assert_eq!(e.call(0x00d6_0480, &args![unlimited]).i32(), -1);
        assert_eq!(e.call(0x00d6_0490, &args![unlimited]).u32(), 0);
        let (limited, _) = server(&mut e, 0, 0x300, 0x1000);
        assert_eq!(e.call(0x00d6_0480, &args![limited]).i32(), 0xd00);
        assert_eq!(e.call(0x00d6_0490, &args![limited]).u32(), 0x1000);
    }

    #[test]
    fn set_memory_limit_refuses_a_limit_below_the_bytes_in_use() {
        let mut e = Engine::new();
        let (this, _) = server(&mut e, 0, 0x300, 0);
        let result = e.mem.alloc(4);
        // Below the bytes in use: refused, unchanged.
        assert_eq!(
            e.call(0x00d6_04a0, &args![this, result, 0x2ffu32]).u32(),
            result
        );
        assert_eq!(e.mem.u8(result), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // Exactly the bytes in use: accepted.
        assert_eq!(
            e.call(0x00d6_04a0, &args![this, result, 0x300u32]).u32(),
            result
        );
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0x300);
        // Zero removes the limit.
        e.mem.set_u8(result, 0);
        assert_eq!(
            e.call(0x00d6_04a0, &args![this, result, 0u32]).u32(),
            result
        );
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }
}
