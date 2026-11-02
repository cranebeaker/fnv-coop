//! `Havok SDK/hkmemoryrouter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread-local-storage index (a word in `.data`).
const TLS_INDEX_01268108: u32 = 0x0126_8108;
/// The import slot of `TlsGetValue` (`kernel32`).
const TLS_GET_VALUE: u32 = 0x00fd_f044;
/// The import slot of `TlsSetValue` (`kernel32`).
const TLS_SET_VALUE: u32 = 0x00fd_f04c;

// Translated from 00c85750 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `hkmemoryrouter.cpp`): `TlsGetValue`
/// of the index stored at `01268108`; the other Havok code uses the result
/// as the calling thread's memory router. Returns that value.
pub fn fn_00c85750(e: &mut Engine) -> u32 {
    let index = e.global::<u32>(TLS_INDEX_01268108);
    e.call(TLS_GET_VALUE, &args![index]).u32()
}

// Translated from 00c85770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryRouter::hkMemoryRouter` (Xbox PDB), `thiscall`, no stack arguments:
/// clears the first eight words (0x20 bytes) of the router. Returns `this`.
pub fn hk_memory_router_hk_memory_router(e: &mut Engine, this: Ptr) -> Ptr {
    for word in 0..8 {
        e.mem.set_u32(this.addr() + word * 4, 0);
    }
    this
}

// Translated from 00c856c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryRouter::easyFree` (Xbox PDB): does nothing for a null `block`.
/// Otherwise the allocation starts 0x10 bytes before `block`: zeroes its
/// first word and frees it through slot +8 of `router`'s vtable, with the
/// start and the size (the word at `block - 0xC`, plus 0x10).
pub fn hk_memory_router_easy_free(e: &mut Engine, router: u32, block: u32) {
    if block == 0 {
        return;
    }
    let start = block.wrapping_sub(0x10);
    let size = e.mem.u32(start + 4).wrapping_add(0x10);
    e.mem.set_u32(start, 0);
    e.vcall(router, 8, &args![start, size]);
}

// Translated from 00c85620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryRouter::alignedAlloc` (Xbox PDB), cdecl `(router, size, alignment)`: allocates
/// `size + alignment + 0x10` bytes through slot +4 of `router`'s vtable, takes the address
/// `(block + alignment + 0xf) & !(alignment - 1)` and records, below it, the allocation's
/// bookkeeping: `size + alignment` at `-0xc`, the router at `-0x10` and the distance back to
/// the block at `-8`. Returns the aligned address.
pub fn hk_memory_router_aligned_alloc(
    e: &mut Engine,
    router: u32,
    size: u32,
    alignment: u32,
) -> u32 {
    let total = size.wrapping_add(alignment);
    let block = e.vcall(router, 4, &args![total.wrapping_add(0x10)]).u32();
    let aligned = block.wrapping_add(alignment).wrapping_add(0xf) & !alignment.wrapping_sub(1);
    e.mem.set_u32(aligned.wrapping_sub(0xc), total);
    e.mem.set_u32(aligned.wrapping_sub(0x10), router);
    e.mem
        .set_u32(aligned.wrapping_sub(8), aligned.wrapping_sub(block));
    aligned
}

// Translated from 00c85660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryRouter::alignedFree` (Xbox PDB), cdecl `(router, block)`: does nothing for a null
/// `block`. Otherwise the allocation started `block - (word at block - 8)`; it zeroes the word
/// at `block - 0x10` and frees through slot +8 of `router`'s vtable with that start and the
/// size `(word at block - 0xc) + 0x10`.
pub fn hk_memory_router_aligned_free(e: &mut Engine, router: u32, block: u32) {
    if block == 0 {
        return;
    }
    let size = e.mem.u32(block.wrapping_sub(0xc)).wrapping_add(0x10);
    let start = block.wrapping_sub(e.mem.u32(block.wrapping_sub(8)));
    e.mem.set_u32(block.wrapping_sub(0x10), 0);
    e.vcall(router, 8, &args![start, size]);
}

// Translated from 00c85690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryRouter::easyAlloc` (Xbox PDB), cdecl `(router, size)`: allocates `size + 0x10`
/// bytes through slot +4 of `router`'s vtable, stores `size` at `+4` and `router` at `+0` of
/// the block and returns the block plus 0x10.
pub fn hk_memory_router_easy_alloc(e: &mut Engine, router: u32, size: u32) -> u32 {
    let block = e.vcall(router, 4, &args![size.wrapping_add(0x10)]).u32();
    e.mem.set_u32(block.wrapping_add(4), size);
    e.mem.set_u32(block, router);
    block.wrapping_add(0x10)
}

// Translated from 00c856f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `hkmemoryrouter.cpp`), `thiscall`, no stack arguments:
/// calls slot `0x30` of the objects at `this + 0x14`, `+0x1c`, `+0x10`, `+0x18` and `+0xc`, in
/// that order, with no arguments. The last call is a tail jump, so the answer is that call's.
pub fn fn_00c856f0(e: &mut Engine, this: Ptr) -> u32 {
    let mut answer = 0;
    for offset in [0x14, 0x1c, 0x10, 0x18, 0xc] {
        let object = e.mem.u32(this.addr() + offset);
        answer = e.vcall(object, 0x30, &args![]).u32();
    }
    answer
}

// Translated from 00c85730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryRouter::replaceInstance` (Xbox PDB), cdecl `(router)`: `TlsSetValue` of the index
/// stored at `01268108` with `router`. The exe returns what `TlsSetValue` returned.
pub fn hk_memory_router_replace_instance(e: &mut Engine, router: u32) -> u32 {
    let index = e.global::<u32>(TLS_INDEX_01268108);
    e.call(TLS_SET_VALUE, &args![index, router]).u32()
}

// Translated from 00c85760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryRouter::getInstancePtr` (Xbox PDB), no arguments: `TlsGetValue` of the index
/// stored at `01268108`; returns that value (the calling thread's router).
pub fn hk_memory_router_get_instance_ptr(e: &mut Engine) -> u32 {
    let index = e.global::<u32>(TLS_INDEX_01268108);
    e.call(TLS_GET_VALUE, &args![index]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c85620, hk_memory_router_aligned_alloc(u32, u32, u32) -> u32),
        entry!(0x00c85660, hk_memory_router_aligned_free(u32, u32)),
        entry!(0x00c85690, hk_memory_router_easy_alloc(u32, u32) -> u32),
        entry!(0x00c856f0, fn_00c856f0(Ptr) -> u32),
        entry!(0x00c85730, hk_memory_router_replace_instance(u32) -> u32),
        entry!(0x00c85760, hk_memory_router_get_instance_ptr() -> u32),
        entry!(0x00c85770, hk_memory_router_hk_memory_router(Ptr) -> Ptr),
        entry!(0x00c856c0, hk_memory_router_easy_free(u32, u32)),
        entry!(0x00c85750, fn_00c85750() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c85750_returns_the_tls_value_of_the_stored_index() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(TLS_INDEX_01268108, 7u32);
        e.register(TLS_GET_VALUE, |_, a| Ret {
            eax: 0x5000 + a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x00c8_5750, &args![]).u32(), 0x5007);
    }

    #[test]
    fn easy_free_returns_a_block_through_the_routers_slot() {
        let mut e = Engine::new();
        let router = e.mem.alloc(4);
        let block = e.mem.alloc(0x30);
        e.put_vtable(0x4400_0000, &[0, 0, 0x4500_0000]);
        e.mem.set_u32(router, 0x4400_0000);
        e.mem.set_u32(block, 0x1234);
        e.mem.set_u32(block + 4, 0x20);
        e.register(0x4500_0000, |e, a| {
            // The first word of the allocation is already cleared.
            assert_eq!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c8_56c0, &args![router, block + 0x10]);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x4500_0000, vec![router, block, 0x30])));
    }

    #[test]
    fn easy_free_ignores_a_null_block() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00c8_56c0, &args![0x1000u32, 0u32]);
        assert_eq!(e.call_log.clone().unwrap().len(), 1);
    }

    #[test]
    fn the_constructor_clears_eight_words_and_nothing_more() {
        let mut e = Engine::new();
        let router = e.mem.alloc(0x30);
        e.mem.write(router, &[0xcd; 0x30]);
        assert_eq!(e.call(0x00c8_5770, &args![router]).u32(), router);
        assert_eq!(e.mem.bytes(router, 0x20), vec![0; 0x20]);
        assert_eq!(e.mem.u32(router + 0x20), 0xcdcd_cdcd);
    }

    /// A router object whose vtable allocates (slot +4, heap blocks of the size asked) and
    /// frees (slot +8, nothing).
    fn heap_router(e: &mut Engine) -> u32 {
        e.put_vtable(0x4400_0000, &[0, 0x4500_0000, 0x4500_0001]);
        let router = e.mem.alloc(8);
        e.mem.set_u32(router, 0x4400_0000);
        e.register(0x4500_0000, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(0x4500_0001, |_, _| Ret::default());
        router
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.clone().unwrap();
        log.into_iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1)
            .collect()
    }

    #[test]
    fn aligned_alloc_aligns_and_records_how_to_free() {
        let mut e = Engine::new();
        let router = heap_router(&mut e);
        e.call_log = Some(vec![]);
        let aligned = e.call(0x00c8_5620, &args![router, 0x30u32, 0x40u32]).u32();
        assert_eq!(aligned % 0x40, 0);
        // size + alignment + 0x10 bytes were asked for
        assert_eq!(calls_to(&e, 0x4500_0000), vec![vec![router, 0x80]]);
        assert_eq!(e.mem.u32(aligned - 0xc), 0x70);
        assert_eq!(e.mem.u32(aligned - 0x10), router);
        let back = e.mem.u32(aligned - 8);
        assert!((0x10..=0x4f).contains(&back));
        // the block the router handed out is `back` bytes below, and the 0x30 bytes fit in it
        let block = aligned - back;
        assert_eq!(e.mem.block_size(block), Some(0x80));
        assert!(aligned + 0x30 <= block + 0x80);
    }

    #[test]
    fn aligned_free_returns_the_whole_block_and_ignores_null() {
        let mut e = Engine::new();
        let router = heap_router(&mut e);
        let aligned = e.call(0x00c8_5620, &args![router, 0x30u32, 0x40u32]).u32();
        let block = aligned - e.mem.u32(aligned - 8);
        e.call_log = Some(vec![]);
        e.call(0x00c8_5660, &args![router, aligned]);
        // the router word is zeroed first; the free call gets the start and the full size
        assert_eq!(e.mem.u32(aligned - 0x10), 0);
        assert_eq!(calls_to(&e, 0x4500_0001), vec![vec![router, block, 0x80]]);
        e.call_log = Some(vec![]);
        e.call(0x00c8_5660, &args![router, 0u32]);
        assert_eq!(e.call_log.clone().unwrap().len(), 1);
    }

    #[test]
    fn easy_alloc_records_the_router_and_the_size_below_the_block() {
        let mut e = Engine::new();
        let router = heap_router(&mut e);
        e.call_log = Some(vec![]);
        let pointer = e.call(0x00c8_5690, &args![router, 0x20u32]).u32();
        assert_eq!(calls_to(&e, 0x4500_0000), vec![vec![router, 0x30]]);
        assert_eq!(e.mem.block_size(pointer - 0x10), Some(0x30));
        assert_eq!(e.mem.u32(pointer - 0x10), router);
        assert_eq!(e.mem.u32(pointer - 0xc), 0x20);
    }

    #[test]
    fn fn_00c856f0_calls_slot_0x30_of_five_objects_in_order() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 12];
        slots.push(0x4500_0000);
        e.put_vtable(0x4400_0000, &slots);
        e.register(0x4500_0000, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let objects: Vec<u32> = (0..8).map(|_| e.mem.alloc(8)).collect();
        for (offset, object) in [0x14, 0x1c, 0x10, 0x18, 0xc].into_iter().zip(&objects) {
            e.mem.set_u32(this + offset, *object);
            e.mem.set_u32(*object, 0x4400_0000);
        }
        e.call_log = Some(vec![]);
        let answer = e.call(0x00c8_56f0, &args![this]).u32();
        let order: Vec<u32> = calls_to(&e, 0x4500_0000)
            .iter()
            .map(|call| call[0])
            .collect();
        assert_eq!(order, objects[..5].to_vec());
        // the last call is a tail jump: its answer is returned
        assert_eq!(answer, objects[4]);
    }

    #[test]
    fn replace_and_get_instance_use_the_tls_index() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(TLS_INDEX_01268108, 7u32);
        e.register(TLS_SET_VALUE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(TLS_GET_VALUE, |_, a| Ret {
            eax: 0x5000 + a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8_5730, &args![0x1234u32]).u32(), 1);
        assert_eq!(calls_to(&e, TLS_SET_VALUE), vec![vec![7, 0x1234]]);
        assert_eq!(e.call(0x00c8_5760, &args![]).u32(), 0x5007);
    }
}
