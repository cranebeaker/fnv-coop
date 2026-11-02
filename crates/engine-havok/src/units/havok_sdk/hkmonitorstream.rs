//! `Havok SDK/hkmonitorstream.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The calling thread's memory router (`00c85750`).
const GET_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The thread-local-storage index of the monitor stream (a word in `.data`).
const MONITOR_STREAM_TLS_INDEX: u32 = 0x0126_835c;
/// The import slot of `TlsSetValue` (`kernel32`).
const TLS_SET_VALUE: u32 = 0x00fd_f04c;

// Translated from 00cb0db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStream::init` (Xbox PDB): takes the calling thread's router
/// (`00c85750`), allocates 0x14 bytes through virtual slot +4 of the object
/// at router `+0x14`, stores the block as the thread's value for the TLS
/// index at `0126835c` (`TlsSetValue`), and clears its words at +0, +4, +8,
/// +0xC and its byte at +0x10.
pub fn hk_monitor_stream_init(e: &mut Engine) {
    let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
    // hkMemoryRouter +0x14: the object whose slot +4 allocates.
    let allocator = e.mem.u32(router + 0x14);
    let block = e.vcall(allocator, 4, &args![0x14u32]).u32();
    let index = e.global::<u32>(MONITOR_STREAM_TLS_INDEX);
    e.call(TLS_SET_VALUE, &args![index, block]);
    e.mem.set_u8(block + 0x10, 0);
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 0xc, 0);
}

/// The import slot of `TlsGetValue` (`kernel32`).
const TLS_GET_VALUE: u32 = 0x00fd_f044;

// Translated from 00cb0da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStream::reset` (Xbox PDB), cdecl `(stream)`: makes the stream's current
/// position (`+4`) its start (`+0`).
pub fn hk_monitor_stream_reset(e: &mut Engine, stream: Ptr) {
    let start = e.mem.u32(stream.addr());
    e.mem.set_u32(stream.addr() + 4, start);
}

// Translated from 00cb0df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStream::quit` (Xbox PDB), cdecl `(stream)`: takes the calling thread's block
/// (`TlsGetValue` of the index at `0126835c`). When the stream's buffer (`+0`) is not null
/// and its byte `+0x10` is set, frees it through slot `0x10` of the object at router `+0x14`
/// (`00c85750`) with the buffer and its size (`+8` minus `+0`). Then frees the thread's block
/// (0x14 bytes) through slot 8 of that object and clears the thread's value (`TlsSetValue`
/// with 0).
pub fn hk_monitor_stream_quit(e: &mut Engine, stream: Ptr) {
    let index = e.global::<u32>(MONITOR_STREAM_TLS_INDEX);
    let block = e.call(TLS_GET_VALUE, &args![index]).u32();
    let start = e.mem.u32(stream.addr());
    if start != 0 && e.mem.u8(stream.addr() + 0x10) != 0 {
        let end = e.mem.u32(stream.addr() + 8);
        let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x14);
        e.vcall(allocator, 0x10, &args![start, end.wrapping_sub(start)]);
    }
    let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x14);
    e.vcall(allocator, 8, &args![block, 0x14u32]);
    let index = e.global::<u32>(MONITOR_STREAM_TLS_INDEX);
    e.call(TLS_SET_VALUE, &args![index, 0u32]);
}

// Translated from 00cb0e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStream::resize` (Xbox PDB), cdecl `(stream, size)`: nothing when `size` equals the
/// current capacity (`+8` minus `+0`); `quit` (`00cb0df0`) when `size` is not above 0 (signed).
/// Otherwise it frees an existing buffer (when `+0` is not null and the byte `+0x10` is set)
/// through slot `0x10` of the object at router `+0x14` (`00c85750`) with the buffer and its
/// capacity, sets the byte `+0x10` to 1 and allocates through slot `0xc` of that object, given
/// the address of a word holding `size` that the allocator may change: the stream's start
/// (`+0`) and current position (`+4`) become the block, its end (`+8`) the block plus the
/// word, and `+0xc` the end minus 0x20.
pub fn hk_monitor_stream_resize(e: &mut Engine, stream: Ptr, size: i32) {
    let start = e.mem.u32(stream.addr());
    let end = e.mem.u32(stream.addr() + 8);
    if size == end.wrapping_sub(start) as i32 {
        return;
    }
    if size <= 0 {
        hk_monitor_stream_quit(e, stream);
        return;
    }
    if start != 0 && e.mem.u8(stream.addr() + 0x10) != 0 {
        let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x14);
        e.vcall(allocator, 0x10, &args![start, end.wrapping_sub(start)]);
    }
    e.mem.set_u8(stream.addr() + 0x10, 1);
    let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x14);
    let (block, granted) = e.with_stack(4, |e, requested| {
        e.mem.set_u32(requested.addr(), size as u32);
        let block = e.vcall(allocator, 0xc, &args![requested]).u32();
        (block, e.mem.u32(requested.addr()))
    });
    e.mem.set_u32(stream.addr(), block);
    e.mem.set_u32(stream.addr() + 4, block);
    let new_end = block.wrapping_add(granted);
    e.mem.set_u32(stream.addr() + 8, new_end);
    e.mem
        .set_u32(stream.addr() + 0xc, new_end.wrapping_sub(0x20));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb0da0, hk_monitor_stream_reset(Ptr)),
        entry!(0x00cb0df0, hk_monitor_stream_quit(Ptr)),
        entry!(0x00cb0e50, hk_monitor_stream_resize(Ptr, i32)),
        entry!(0x00cb0db0, hk_monitor_stream_init()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_allocates_clears_and_stores_the_block_for_the_thread() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(MONITOR_STREAM_TLS_INDEX, 9u32);
        let router = e.mem.alloc(0x24);
        let allocator = e.mem.alloc(8);
        let block = e.mem.alloc(0x14);
        for offset in 0..5 {
            e.mem.set_u32(block + 4 * offset, 0xdead_beef);
        }
        e.put_vtable(0x4400_0000, &[0, 0x4500_0000]);
        e.mem.set_u32(allocator, 0x4400_0000);
        e.mem.set_u32(router + 0x14, allocator);
        e.register_double(GET_MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x4500_0000, move |_, a| {
            assert_eq!(a, [allocator, 0x14]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(TLS_SET_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00cb_0db0, &args![]);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(TLS_SET_VALUE, vec![9, block])));
        for offset in [0, 4, 8, 0xc] {
            assert_eq!(e.mem.u32(block + offset), 0);
        }
        assert_eq!(e.mem.u8(block + 0x10), 0);
        // Only the byte at +0x10 is cleared, not the rest of that word.
        assert_eq!(e.mem.u8(block + 0x11), 0xbe);
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.clone().unwrap();
        log.into_iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1)
            .collect()
    }

    /// A router whose object at `+0x14` frees a block (slot 8), allocates (slot `0xc`: the word
    /// it is given grows by 0x10 and the block is `0x60000000`) and frees a buffer (slot
    /// `0x10`); the thread's value is `0x70000000` plus the TLS index 9. Returns the allocator.
    fn install(e: &mut Engine) -> u32 {
        e.map(0x0126_8000, 0x1000);
        e.set_global(MONITOR_STREAM_TLS_INDEX, 9u32);
        e.put_vtable(0x4400_0000, &[0, 0, 0x4500_0008, 0x4500_000c, 0x4500_0010]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_0000);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x14, allocator);
        e.register_double(GET_MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x4500_0008, |_, _| Ret::default());
        e.register(0x4500_000c, |e, a| {
            let requested = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], requested + 0x10);
            Ret {
                eax: 0x6000_0000,
                ..Ret::default()
            }
        });
        e.register(0x4500_0010, |_, _| Ret::default());
        e.register(TLS_SET_VALUE, |_, _| Ret::default());
        e.register(TLS_GET_VALUE, |_, a| Ret {
            eax: 0x7000_0000 + a[0],
            ..Ret::default()
        });
        allocator
    }

    fn stream(e: &mut Engine, start: u32, end: u32, owned: u8) -> u32 {
        let stream = e.mem.alloc(0x14);
        e.mem.set_u32(stream, start);
        e.mem.set_u32(stream + 4, start + 0x20);
        e.mem.set_u32(stream + 8, end);
        e.mem.set_u32(stream + 0xc, end.wrapping_sub(0x20));
        e.mem.set_u8(stream + 0x10, owned);
        stream
    }

    #[test]
    fn reset_rewinds_the_current_position_to_the_start() {
        let mut e = Engine::new();
        let s = stream(&mut e, 0x6000, 0x6400, 1);
        e.call(0x00cb_0da0, &args![s]);
        assert_eq!(e.mem.u32(s + 4), 0x6000);
        assert_eq!(e.mem.u32(s), 0x6000);
    }

    #[test]
    fn quit_frees_an_owned_buffer_then_the_thread_block() {
        let mut e = Engine::new();
        let allocator = install(&mut e);
        let s = stream(&mut e, 0x6000, 0x6400, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cb_0df0, &args![s]);
        assert_eq!(
            calls_to(&e, 0x4500_0010),
            vec![vec![allocator, 0x6000, 0x400]]
        );
        assert_eq!(
            calls_to(&e, 0x4500_0008),
            vec![vec![allocator, 0x7000_0009, 0x14]]
        );
        assert_eq!(calls_to(&e, TLS_SET_VALUE), vec![vec![9, 0]]);
        // a null buffer, or one the stream does not own, is not freed
        for (start, owned) in [(0u32, 1u8), (0x6000, 0)] {
            let s = stream(&mut e, start, 0x6400, owned);
            e.call_log = Some(vec![]);
            e.call(0x00cb_0df0, &args![s]);
            assert!(calls_to(&e, 0x4500_0010).is_empty());
            assert_eq!(calls_to(&e, 0x4500_0008).len(), 1);
        }
    }

    #[test]
    fn resize_to_the_same_capacity_does_nothing() {
        let mut e = Engine::new();
        install(&mut e);
        let s = stream(&mut e, 0x6000, 0x6100, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cb_0e50, &args![s, 0x100i32]);
        assert_eq!(e.call_log.clone().unwrap().len(), 1);
    }

    #[test]
    fn resize_to_nothing_quits() {
        let mut e = Engine::new();
        install(&mut e);
        for size in [0i32, -5] {
            let s = stream(&mut e, 0x6000, 0x6100, 1);
            e.call_log = Some(vec![]);
            e.call(0x00cb_0e50, &args![s, size]);
            assert_eq!(calls_to(&e, 0x4500_0010).len(), 1);
            assert_eq!(calls_to(&e, TLS_SET_VALUE).len(), 1);
            assert!(calls_to(&e, 0x4500_000c).is_empty());
        }
    }

    #[test]
    fn resize_replaces_the_buffer_with_the_size_the_allocator_grants() {
        let mut e = Engine::new();
        let allocator = install(&mut e);
        let s = stream(&mut e, 0x6000, 0x6100, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cb_0e50, &args![s, 0x400i32]);
        assert_eq!(
            calls_to(&e, 0x4500_0010),
            vec![vec![allocator, 0x6000, 0x100]]
        );
        assert_eq!(calls_to(&e, 0x4500_000c).len(), 1);
        assert_eq!(e.mem.u32(s), 0x6000_0000);
        assert_eq!(e.mem.u32(s + 4), 0x6000_0000);
        // the allocator granted 0x10 more than the 0x400 asked for
        assert_eq!(e.mem.u32(s + 8), 0x6000_0410);
        assert_eq!(e.mem.u32(s + 0xc), 0x6000_03f0);
        assert_eq!(e.mem.u8(s + 0x10), 1);
        // a stream without a buffer frees nothing but still becomes owned
        let fresh = stream(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cb_0e50, &args![fresh, 0x40i32]);
        assert!(calls_to(&e, 0x4500_0010).is_empty());
        assert_eq!(e.mem.u32(fresh + 8), 0x6000_0050);
        assert_eq!(e.mem.u8(fresh + 0x10), 1);
    }
}
