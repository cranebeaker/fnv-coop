//! A handle for adapting the world's state to the game's ExtraData lists.
//!
//! Operations dispatch to the existing PC translations, including the
//! bitmap, insertion order, recursive lock and TLS lookup cache. This is
//! the list boundary only: payload conversion, world ownership, persistence
//! and save writing remain open. No viewer system uses this adapter yet.
//!
//! Memory belongs to the supplied [`Engine`]. A handle must only be used
//! with that engine while its list is alive. Copying a handle does not copy
//! the list, and dropping it does not destroy the list or its nodes.
//! Use an initialized engine, normally [`Engine::with_exe`], so the native
//! globals and platform imports are present. [`Engine::new`] alone does
//! not initialize the global list lock.

use crate::units::fallout_shared::extradatalist::{BSExtraData, BaseExtraList};
use crate::{Engine, Ptr};

/// A `BaseExtraList` / `ExtraDataList` (Xbox PDB) in one engine's memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraList(Ptr<BaseExtraList>);

impl ExtraList {
    /// Allocates a list in the engine arena and runs its constructor
    /// (`0040f740`). Node payloads are constructed separately by their
    /// translated class constructors, not manufactured by this adapter.
    pub fn new(engine: &mut Engine) -> Self {
        let list: Ptr<BaseExtraList> = engine.new_object();
        engine.call(0x0040_f740, &[list.addr()]);
        Self(list)
    }

    /// Adapts an existing, initialized list (for example an embedded
    /// reference field). Null is not a list. Other address validity remains
    /// the caller's responsibility, as for other engine pointers.
    pub fn from_ptr(list: Ptr<BaseExtraList>) -> Option<Self> {
        (!list.is_null()).then_some(Self(list))
    }

    pub fn as_ptr(self) -> Ptr<BaseExtraList> {
        self.0
    }

    /// Counts the linked nodes under the native lock (`0040fe20`).
    pub fn len(self, engine: &mut Engine) -> u32 {
        engine.call(0x0040_fe20, &[self.0.addr()]).u32()
    }

    pub fn is_empty(self, engine: &mut Engine) -> bool {
        self.len(engine) == 0
    }

    /// Reads the signed item count (`00418770`), defaulting to 1 when
    /// there is no `ExtraCount` (Xbox PDB). This differs from `len`, which
    /// counts extra-data nodes rather than items in a stack.
    pub fn item_count(self, engine: &mut Engine) -> i16 {
        engine.call(0x0041_8770, &[self.0.addr()]).u16() as i16
    }

    /// Runs `ExtraDataList::SetCount` (Xbox PDB, `00419ad0`). Counts 0 and
    /// 1 remove and destroy an existing count node; a subsequent read is 1.
    /// Other signed values create or update the native 16-bit payload.
    pub fn set_item_count(self, engine: &mut Engine, count: i16) {
        engine.call(0x0041_9ad0, &[self.0.addr(), count as u16 as u32]);
    }

    /// Reads `ExtraHealth::fHealth` (Xbox PDB, `004186f0`), or the native
    /// -1.0 default when absent. Use `has(engine, 0x25)` to distinguish an
    /// absent extra from an explicitly stored -1.0 value.
    pub fn health(self, engine: &mut Engine) -> f32 {
        engine.call(0x0041_86f0, &[self.0.addr()]).f32()
    }

    /// Creates or updates the health extra (`00419970`). The native setter
    /// keeps a node for every value, including zero and -1.0, and quiets
    /// signaling NaNs through its x87 load/store. It does not clamp values.
    pub fn set_health(self, engine: &mut Engine, health: f32) {
        engine.call(0x0041_9970, &[self.0.addr(), health.to_bits()]);
    }

    /// Tests the native 21-byte presence bitmap (`0040fe80`).
    pub fn has(self, engine: &mut Engine, kind: u8) -> bool {
        engine
            .call(0x0040_fe80, &[self.0.addr(), kind.into()])
            .bool()
    }

    /// Uses the native bitmap, TLS cache and chain lookup (`00410220`).
    pub fn get(self, engine: &mut Engine, kind: u8) -> Option<Ptr<BSExtraData>> {
        let node: Ptr<BSExtraData> = engine
            .call(0x0041_0220, &[self.0.addr(), kind.into()])
            .ptr();
        (!node.is_null()).then_some(node)
    }

    /// Adds an initialized, unlinked node (`0040ff60`). The native code
    /// prepends types 0x0c/0x0d/0x0e/0x15/0x2b and appends the others.
    /// It does not enforce unique types or detach a node from another list.
    pub fn add(self, engine: &mut Engine, node: Ptr<BSExtraData>) {
        engine.call(0x0040_ff60, &[self.0.addr(), node.addr()]);
    }

    /// Detaches the first node of its type in this list, without destroying it
    /// (`00410020`, destroy=false). The native code clears its next pointer,
    /// presence bit and cache entry and advances the dirty counters.
    /// The caller must supply that first node: predecessor lookup uses its
    /// type, not pointer identity. Duplicate types retain the native caveat
    /// that removing one clears the shared presence bit.
    pub fn detach(self, engine: &mut Engine, node: Ptr<BSExtraData>) {
        engine.call(0x0041_0020, &[self.0.addr(), node.addr(), 0]);
    }

    /// Forgets the chain without destroying nodes (`0040fae0`, false).
    /// Unlike repeated detach calls, this preserves links between nodes.
    pub fn forget_all(self, engine: &mut Engine) {
        engine.call(0x0040_fae0, &[self.0.addr(), 0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exe::Import;

    /// Synthetic initial globals plus the real platform import functions.
    /// No game-function doubles: these tests exercise the dependency chain.
    fn engine() -> Engine {
        let mut engine = Engine::new();
        engine.map(0x011c_3000, 0x1000); // dirty counter and list lock
        assert_eq!(
            engine.register_imports(&[
                Import {
                    slot: 0x00fd_f1d0,
                    dll: "KERNEL32.dll".into(),
                    name: "GetCurrentThreadId".into()
                },
                Import {
                    slot: 0x00fd_f1bc,
                    dll: "KERNEL32.dll".into(),
                    name: "InterlockedCompareExchange".into()
                },
            ]),
            2
        );
        engine
    }

    // Generated header-only nodes suffice for generic list operations;
    // these fixtures do not represent or test a subclass's payload.
    fn node(engine: &mut Engine, kind: u8) -> Ptr<BSExtraData> {
        let node: Ptr<BSExtraData> = engine.new_object();
        engine.call(0x0040_ec80, &[node.addr(), kind.into()]);
        node
    }

    fn next(engine: &Engine, node: Ptr<BSExtraData>) -> Ptr<BSExtraData> {
        Ptr::new(engine.mem.u32(node.addr() + 8))
    }

    #[test]
    fn real_count_lifecycle_preserves_signed_payload_and_other_nodes() {
        let mut engine = engine();
        // PC ExtraCount vtable +0: scalar deleting destructor 0041b680.
        // Generated fixture contains only the slot this lifecycle needs.
        engine.map(0x0101_5000, 0x1000);
        engine.mem.set_u32(0x0101_58d8, 0x0041_b680);
        let list = ExtraList::new(&mut engine);
        assert_eq!(list.item_count(&mut engine), 1);
        for default in [0, 1] {
            list.set_item_count(&mut engine, default);
            assert!(list.is_empty(&mut engine));
            assert_eq!(list.item_count(&mut engine), 1);
        }
        let other = node(&mut engine, 0x43);
        list.add(&mut engine, other);
        for count in [2, 5, -2, i16::MIN, i16::MAX] {
            let before = list.get(&mut engine, 0x24);
            list.set_item_count(&mut engine, count);
            assert_eq!(list.item_count(&mut engine), count);
            assert_eq!(list.len(&mut engine), 2);
            if let Some(before) = before {
                assert_eq!(list.get(&mut engine, 0x24), Some(before));
            }
        }
        for default in [0, 1] {
            list.set_item_count(&mut engine, 7);
            assert!(list.has(&mut engine, 0x24));
            let removed = list.get(&mut engine, 0x24).unwrap();
            assert_eq!(engine.mem.block_size(removed.addr()), Some(16));
            list.set_item_count(&mut engine, default);
            assert_eq!(engine.mem.block_size(removed.addr()), None);
            assert_eq!(list.item_count(&mut engine), 1);
            assert!(!list.has(&mut engine, 0x24));
            assert!(list.get(&mut engine, 0x24).is_none());
            assert_eq!(list.len(&mut engine), 1);
            assert_eq!(list.get(&mut engine, 0x43), Some(other));
            assert!(next(&engine, other).is_null());
        }
        assert_eq!(engine.mem.u32(0x011c_3920), 0);
        assert_eq!(engine.mem.u32(0x011c_3924), 0);
    }

    #[test]
    fn real_health_registry_keeps_explicit_defaults_and_native_float_bits() {
        let mut engine = engine();
        engine.map(0x0101_2000, 0x1000);
        engine.mem.set_f32(0x0101_2054, -1.0);
        let list = ExtraList::new(&mut engine);
        assert_eq!(list.health(&mut engine), -1.0);
        assert!(!list.has(&mut engine, 0x25));
        list.set_item_count(&mut engine, 4);
        // First value exercises allocation; the rest update that same node.
        let mut health_node = None;
        for (input, expected) in [
            (0x7f80_0001, 0x7fc0_0001),
            (0x0000_0000, 0x0000_0000),
            (0x8000_0000, 0x8000_0000),
            (0x42c8_8000, 0x42c8_8000),
            (0xbf80_0000, 0xbf80_0000),
            (0xff80_0001, 0xffc0_0001),
        ] {
            list.set_health(&mut engine, f32::from_bits(input));
            assert_eq!(list.health(&mut engine).to_bits(), expected);
            let node = list.get(&mut engine, 0x25).unwrap();
            assert_eq!(engine.mem.u32(node.addr() + 0x0c), expected);
            if let Some(previous) = health_node {
                assert_eq!(node, previous);
            }
            health_node = Some(node);
            assert!(list.has(&mut engine, 0x25));
            assert_eq!(list.len(&mut engine), 2);
            assert_eq!(list.item_count(&mut engine), 4);
        }
        let node = health_node.unwrap();
        engine.mem.set_u32(node.addr() + 0x0c, 0x7f80_0001);
        assert_eq!(list.health(&mut engine).to_bits(), 0x7fc0_0001);
        assert_eq!(engine.mem.u32(node.addr() + 0x0c), 0x7f80_0001);
        assert_eq!(engine.mem.u32(0x011c_3920), 0);
        assert_eq!(engine.mem.u32(0x011c_3924), 0);
    }

    #[test]
    fn real_subclass_constructor_reaches_base_constructor_and_list() {
        use crate::units::fallout_shared::extradatalist::ExtraNorthRotation;
        let mut engine = engine();
        let list = ExtraList::new(&mut engine);
        let rotation: Ptr<ExtraNorthRotation> = engine.new_object();
        assert_eq!(
            engine.call(0x0040_f680, &[rotation.addr()]).u32(),
            rotation.addr()
        );
        assert_eq!(engine.get(rotation, ExtraNorthRotation::fNorthRot), 0.0);
        list.add(&mut engine, rotation.cast());
        assert_eq!(list.get(&mut engine, 0x43), Some(rotation.cast()));
        list.detach(&mut engine, rotation.cast());
        assert!(list.is_empty(&mut engine));
        assert!(!list.has(&mut engine, 0x43));
    }

    #[test]
    fn real_registry_preserves_native_head_and_tail_insertion_and_bitmap() {
        let mut engine = engine();
        let list = ExtraList::new(&mut engine);
        assert!(list.is_empty(&mut engine));
        assert!(ExtraList::from_ptr(Ptr::NULL).is_none());
        assert_eq!(ExtraList::from_ptr(list.as_ptr()), Some(list));
        let tail_a = node(&mut engine, 0x43);
        let tail_b = node(&mut engine, 0x44);
        let head = node(&mut engine, 0x0d);
        for item in [tail_a, tail_b, head] {
            list.add(&mut engine, item);
        }
        assert_eq!(list.len(&mut engine), 3);
        assert_eq!(engine.mem.u32(list.as_ptr().addr() + 4), head.addr());
        assert_eq!(next(&engine, head), tail_a);
        assert_eq!(next(&engine, tail_a), tail_b);
        assert!(next(&engine, tail_b).is_null());
        for (kind, item) in [(0x43, tail_a), (0x44, tail_b), (0x0d, head)] {
            assert!(list.has(&mut engine, kind));
            assert_eq!(list.get(&mut engine, kind), Some(item));
        }
        assert!(!list.has(&mut engine, 0xff));
        assert_eq!(engine.mem.u32(0x011c_3920), 0, "lock released");
        assert_eq!(engine.mem.u32(0x011c_3924), 0, "recursion balanced");
    }

    #[test]
    fn real_registry_switches_cached_lists_and_invalidates_detached_nodes() {
        let mut engine = engine();
        let first = ExtraList::new(&mut engine);
        let second = ExtraList::new(&mut engine);
        let a = node(&mut engine, 0x43);
        let b = node(&mut engine, 0x43);
        let remaining = node(&mut engine, 0x44);
        first.add(&mut engine, a);
        first.add(&mut engine, remaining);
        second.add(&mut engine, b);
        for _ in 0..2 {
            assert_eq!(first.get(&mut engine, 0x43), Some(a));
            assert_eq!(second.get(&mut engine, 0x43), Some(b));
        }
        assert_eq!(first.get(&mut engine, 0x43), Some(a));
        let dirty_before = engine.mem.u32(0x011c_38e4);
        first.detach(&mut engine, a);
        assert!(first.get(&mut engine, 0x43).is_none());
        assert!(next(&engine, a).is_null());
        assert_eq!(first.len(&mut engine), 1);
        assert_eq!(first.get(&mut engine, 0x44), Some(remaining));
        assert_eq!(second.get(&mut engine, 0x43), Some(b));
        assert_ne!(engine.mem.u32(0x011c_38e4), dirty_before);
        let last = node(&mut engine, 0x45);
        first.add(&mut engine, last);
        assert_eq!(first.get(&mut engine, 0x45), Some(last));
        first.detach(&mut engine, last);
        assert_eq!(first.len(&mut engine), 1);
        assert!(next(&engine, remaining).is_null());
        assert!(next(&engine, last).is_null());
        assert!(!first.has(&mut engine, 0x45));
        assert!(first.get(&mut engine, 0x45).is_none());
    }

    #[test]
    fn real_registry_forgets_membership_but_preserves_the_old_chain() {
        let mut engine = engine();
        let list = ExtraList::new(&mut engine);
        let first = node(&mut engine, 0x43);
        let last = node(&mut engine, 0x44);
        list.add(&mut engine, first);
        list.add(&mut engine, last);
        assert_eq!(list.get(&mut engine, 0x44), Some(last));
        list.forget_all(&mut engine);
        assert!(list.is_empty(&mut engine));
        assert!(list.get(&mut engine, 0x43).is_none());
        assert!(list.get(&mut engine, 0x44).is_none());
        assert_eq!(next(&engine, first), last);
        assert!(next(&engine, last).is_null());
    }
}
