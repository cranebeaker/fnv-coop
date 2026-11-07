//! The per-thread interfaces of a map (`ThreadSpecificInterfaceManager<..>`, Xbox PDB): each game
//! thread that touches a map gets its own `LockFreeMapInterface`, found again through a TLS slot.

use super::layout::{InterfaceEntry, InterfaceManager, LockFreeMap, MapInterface};
use super::{
    field_ptr, FAMILY, OPERATOR_NEW, SCOPE_BEGIN, SCOPE_CONTEXT, SCOPE_END, SCOPE_MODE, SOURCE_FILE,
};
use engine_core::ptr::{Layout, Ptr};
use engine_core::{args, typed_call, Engine};

/// `TlsGetValue` and `TlsSetValue`, by their import slots.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const TLS_SET_VALUE: u32 = 0x00fd_f04c;
/// The interlocked increment (`cdecl`, the word's address; answers the new count).
const INTERLOCKED_INCREMENT: u32 = 0x0040_b460;
/// `GetCurrentThreadId`'s wrapper (no arguments).
const CURRENT_THREAD_ID: u32 = 0x0040_fc90;
/// The map's virtual `AllocateInterface(index)` (Xbox PDB name of the slot in the queues).
const ALLOCATE_INTERFACE_SLOT: u32 = 4;

// Typed from 0044d5c0
/// The current thread's interface to `map`: the value of the TLS slot at +4 (`TlsGetValue`, as
/// `0044d5b0` asks it), or a new one (`0044e3a0`) when the slot holds none.
pub fn interface_for_thread(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    map: Ptr<LockFreeMap>,
) -> Ptr<MapInterface> {
    let index = e.get(this, InterfaceManager::tls_index);
    let found: Ptr<MapInterface> = Ptr::new(e.call(TLS_GET_VALUE, &args![index]).u32());
    if !found.is_null() {
        return found;
    }
    typed_call!(FAMILY, 0x0044_e3a0, make_interface(e, this, map))
}

// Typed from 0044e3a0
/// Hands the current thread the next interface: counts it (interlocked, at +0xc); while the count
/// is within `max_threads`, has `map` make the interface for that index (its virtual slot +4), puts
/// it and the thread's id in that index's entry, and in the TLS slot (`TlsSetValue`, as `0044e340`
/// does). Answers the interface, or null past the limit, which is not counted back.
pub fn make_interface(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    map: Ptr<LockFreeMap>,
) -> Ptr<MapInterface> {
    let mut made: Ptr<MapInterface> = Ptr::NULL;
    let count = e
        .call(
            INTERLOCKED_INCREMENT,
            &args![field_ptr(this, InterfaceManager::handed_out)],
        )
        .u32();
    let index = count.wrapping_sub(1);
    if index < e.get(this, InterfaceManager::max_threads) {
        made = Ptr::new(
            e.vcall(map.addr(), ALLOCATE_INTERFACE_SLOT, &args![index])
                .u32(),
        );
        let entries = e.get(this, InterfaceManager::entries);
        e.set(
            InterfaceManager::entry(entries, index),
            InterfaceEntry::interface,
            made,
        );
        let thread = e.call(CURRENT_THREAD_ID, &[]).u32();
        let entries = e.get(this, InterfaceManager::entries);
        e.set(
            InterfaceManager::entry(entries, index),
            InterfaceEntry::thread_id,
            thread,
        );
        let tls = e.get(this, InterfaceManager::tls_index);
        e.call(TLS_SET_VALUE, &args![tls, made]);
    }
    made
}

/// The line `006c73b0` names in its scope.
const MANAGER_LINE: u32 = 0x14;
/// The vector constructor iterator (`stdcall(block, element size, count, constructor)`), and the
/// constructor of an entry (both words cleared; `thiscall`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const ENTRY_CONSTRUCT: u32 = 0x0096_a2d0;
/// `TlsAlloc`'s wrapper (no arguments).
const TLS_ALLOC: u32 = 0x0044_e330;

// Typed from 006c73b0
/// The interface manager's constructor (`thiscall(max threads)`, answers `this`), in a
/// memory-context scope: none handed out yet, room for `max_threads` entries (`operator new` of
/// eight bytes each, all ones when that overflows; the entries cleared by the vector constructor
/// iterator, or null when the allocation failed), and a TLS slot of its own (`TlsAlloc`).
pub fn construct(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    max_threads: u32,
) -> Ptr<InterfaceManager> {
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_BEGIN,
            &args![scope, SCOPE_CONTEXT, SCOPE_MODE, SOURCE_FILE, MANAGER_LINE],
        );
        e.set(this, InterfaceManager::handed_out, 0);
        e.set(this, InterfaceManager::max_threads, max_threads);
        let count = e.get(this, InterfaceManager::max_threads);
        let bytes = count.saturating_mul(InterfaceEntry::SIZE);
        let block: Ptr<InterfaceEntry> = Ptr::new(e.call(OPERATOR_NEW, &args![bytes]).u32());
        if !block.is_null() {
            e.call(
                VECTOR_CONSTRUCT,
                &args![block, InterfaceEntry::SIZE, count, ENTRY_CONSTRUCT],
            );
        }
        e.set(this, InterfaceManager::entries, block);
        let tls = e.call(TLS_ALLOC, &[]).u32();
        e.set(this, InterfaceManager::tls_index, tls);
        e.call(SCOPE_END, &args![scope]);
    });
    this
}
