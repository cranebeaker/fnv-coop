//! The lock-free map family (T4 of the migration plan, stage 1): the game's `LockFreeMap` (Xbox PDB),
//! its per-thread interfaces, its iterators and its counted-pointer holder. The model, texture, lip
//! sync, pathing and cell loaders keep their tasks in these maps, and every frame walks them.
//!
//! Stage 1: the objects stay in guest memory where the game made them, and are read through their
//! [`layout`]. Each function stays registered at its exe address, so the translations, the vtables
//! and the callback words reach the typed version unchanged; calls between functions of the family
//! are `typed_call!`s, Rust calls while the family is on.
//!
//! What a call by address in the translation did that the typed version does in its own body (the
//! iterator's flag tests, the marked-pointer helpers, the interface's release of its node slots)
//! is in each function's comment. The lock at +0x20 stays the game's own (`0040fbf0`, which yields
//! to the other game threads).
//!
//! Integers only, so the family's row is [`Threads::Both`]: nothing here depends on the control word.

use engine_core::ptr::{Field, Ptr};
use engine_core::typed::{Family, FamilyId, Threads};
use engine_core::{entry, AbiFn};
use layout::{
    HashListNode, Holder, InterfaceManager, LockFreeMap, LockFreeMapIterator, LockFreeQueue,
    MapInterface,
};

pub mod holder;
pub mod iface;
pub mod layout;
pub mod manager;
pub mod map;
pub mod queue;

/// The memory-context scope the family's allocating functions run in (`thiscall(scope, context,
/// mode, source file, line)` .. `thiscall(scope)`), a 4-byte local: the context for the allocations,
/// named by the source of the thread-safe structures and a line.
pub(crate) const SCOPE_BEGIN: u32 = 0x0040_4eb0;
pub(crate) const SCOPE_END: u32 = 0x0040_4ee0;
pub(crate) const SCOPE_CONTEXT: u32 = 6;
pub(crate) const SCOPE_MODE: u32 = 1;
pub(crate) const SOURCE_FILE: u32 = 0x0101_73f8;
/// `operator new` and `operator delete` (`cdecl`).
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;

/// This family's bit: what each `typed_call!` of it names.
pub const FAMILY: FamilyId = FamilyId::new(1);

/// `NV_TYPED=lockfree`. Not past its gate.
pub static LOCKFREE: Family = Family {
    name: "lockfree",
    id: FAMILY,
    past_gate: false,
    threads: Threads::Both,
    funcs,
};

/// The address of a field: what the game passes on where it hands a field to a function by
/// pointer (a count to an interlocked increment, a lock to its entry).
#[inline(always)]
pub(crate) fn field_ptr<S, T>(p: Ptr<S>, f: Field<S, T>) -> Ptr<T> {
    p.byte_add(f.off).cast()
}

/// The family's functions, by exe address.
fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0092_c820, holder::assign(Ptr<Holder>, Ptr<Holder>) -> Ptr),
        entry!(
            0x0044_d5c0,
            manager::interface_for_thread(Ptr<InterfaceManager>, Ptr<LockFreeMap>) -> Ptr
        ),
        entry!(
            0x0044_e3a0,
            manager::make_interface(Ptr<InterfaceManager>, Ptr<LockFreeMap>) -> Ptr
        ),
        entry!(
            0x0044_e550,
            iface::first_in_bucket(Ptr<MapInterface>, u32, Ptr<u32>, Ptr<Holder>) -> bool
        ),
        entry!(
            0x0052_9030,
            iface::search(Ptr<MapInterface>, u32, u32) -> u32
        ),
        entry!(
            0x0052_92a0,
            iface::retire(Ptr<MapInterface>, Ptr<HashListNode>)
        ),
        entry!(
            0x0044_e680,
            iface::next_in_bucket(Ptr<MapInterface>, u32, Ptr<u32>, Ptr<Holder>) -> bool
        ),
        entry!(
            0x0052_8ce0,
            iface::remove_from_bucket(Ptr<MapInterface>, u32, u32) -> bool
        ),
        entry!(0x0052_8bb0, iface::remove(Ptr<MapInterface>, u32) -> bool),
        entry!(
            0x0044_ea60,
            iface::first_word_in_bucket(Ptr<MapInterface>, u32, Ptr<u32>, Ptr<u32>) -> bool
        ),
        entry!(
            0x0044_eb80,
            iface::next_word_in_bucket(Ptr<MapInterface>, u32, Ptr<u32>, Ptr<u32>) -> bool
        ),
        entry!(
            0x0044_e820,
            iface::find_value(Ptr<MapInterface>, u32, Ptr<u32>) -> bool
        ),
        entry!(
            0x0044_c8e0,
            iface::find(Ptr<MapInterface>, u32, Ptr<Holder>) -> u8
        ),
        entry!(
            0x0044_c000,
            iface::insert(Ptr<MapInterface>, u32, u32, u8) -> u32
        ),
        entry!(
            0x0044_bd60,
            map::find(Ptr<LockFreeMap>, u32, Ptr<Holder>) -> u8
        ),
        entry!(
            0x0044_d070,
            iface::insert_in_bucket(Ptr<MapInterface>, u32, u32, Ptr<Holder>, u8) -> bool
        ),
        entry!(
            0x0044_d240,
            iface::insert_word_in_bucket(Ptr<MapInterface>, u32, u32, Ptr<u32>, u8) -> bool
        ),
        entry!(
            0x0044_b8d0,
            map::iterate(
                Ptr<LockFreeMap>,
                Ptr<LockFreeMapIterator>,
                Ptr<u32>,
                Ptr<Holder>,
                u8,
            ) -> bool
        ),
        entry!(
            0x0044_c640,
            map::iterate_words(
                Ptr<LockFreeMap>,
                Ptr<LockFreeMapIterator>,
                Ptr<u32>,
                Ptr<u32>,
                u8,
            ) -> bool
        ),
        entry!(0x0044_c480, map::clear(Ptr<LockFreeMap>, u8)),
        entry!(
            0x0044_b750,
            map::construct(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0044_ba10,
            map::construct_anim_idle_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0044_bbe0,
            map::construct_replacement_kf_list_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0044_be80,
            map::construct_helmet_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0044_c040,
            map::construct_model_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0044_c270,
            map::construct_kf_model_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0044_c760,
            map::construct_texture_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0044_c920,
            map::construct_loaded_file_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0052_89e0,
            map::construct_cell_task_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0066_5cb0,
            map::construct_tree_node_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x006e_c660,
            map::construct_pathing_task_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0090_6160,
            map::construct_lip_task_map(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0044_b660,
            queue::allocate_interface(Ptr<LockFreeQueue>, u32) -> u32
        ),
        entry!(0x0044_f220, iface::scan_deleted(Ptr<MapInterface>)),
        entry!(0x0044_c3f0, map::destroy(Ptr<LockFreeMap>)),
        entry!(
            0x0066_62a0,
            iface::search_string_map(Ptr<MapInterface>, u32, u32) -> u32
        ),
        entry!(
            0x0066_6080,
            iface::remove_from_bucket_string_map(Ptr<MapInterface>, u32, u32) -> bool
        ),
        entry!(
            0x0044_9530,
            map::find_word(Ptr<LockFreeMap>, u32, Ptr<u32>) -> u8
        ),
        entry!(
            0x0090_6570,
            map::iterate_lip_task_map(
                Ptr<LockFreeMap>,
                Ptr<LockFreeMapIterator>,
                Ptr<u32>,
                Ptr<u32>,
                u8,
            ) -> bool
        ),
        entry!(
            0x0044_f070,
            iface::scan_deleted_releasing(Ptr<MapInterface>)
        ),
        entry!(0x0044_c1e0, map::destroy_model_map(Ptr<LockFreeMap>)),
        entry!(0x0044_cae0, map::destroy_loaded_file_map(Ptr<LockFreeMap>)),
        entry!(0x0066_61e0, map::destroy_tree_node_map(Ptr<LockFreeMap>)),
        entry!(0x006e_c870, map::destroy_pathing_task_map(Ptr<LockFreeMap>)),
        entry!(
            0x0044_d8f0,
            map::destroy_queued_reference_map(Ptr<LockFreeMap>)
        ),
        entry!(0x0044_d9b0, map::destroy_anim_idle_map(Ptr<LockFreeMap>)),
        entry!(
            0x0044_da70,
            map::destroy_replacement_kf_list_map(Ptr<LockFreeMap>)
        ),
        entry!(0x0044_db30, map::destroy_helmet_map(Ptr<LockFreeMap>)),
        entry!(0x0044_dc50, map::destroy_texture_map(Ptr<LockFreeMap>)),
        entry!(0x0052_8be0, map::destroy_cell_task_map(Ptr<LockFreeMap>)),
        entry!(0x0090_64e0, map::destroy_lip_task_map(Ptr<LockFreeMap>)),
        entry!(0x0044_9f80, map::interface_of(Ptr<LockFreeMap>) -> Ptr),
        entry!(
            0x006c_73b0,
            manager::construct(Ptr<InterfaceManager>, u32) -> Ptr
        ),
    ]
}
