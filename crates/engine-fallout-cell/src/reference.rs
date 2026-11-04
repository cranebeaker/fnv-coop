//! A handle for an initialized PC reference and its embedded extra-data list.
//!
//! This boundary adapts an existing `TESObjectREFR` (Xbox PDB); it does not
//! allocate, load, destroy or save a reference. Its memory belongs to the
//! supplied [`Engine`]. Use a handle only with that engine while the object
//! is alive. Copying the handle aliases the same object.
//!
//! Reference construction additionally requires allocator/global startup;
//! [`Engine::with_exe`] maps the executable but does not perform that startup.
//! World ownership and save conversion remain separate work.

use crate::extra_data::ExtraList;
use crate::units::fallout_shared::tesobjectrefr::{TESObjectREFR, GET_EXTRA_LIST};
use crate::{Engine, Ptr};

/// A `TESObjectREFR` (Xbox PDB) in one engine's memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reference(Ptr<TESObjectREFR>);

impl Reference {
    /// Adapts an existing initialized reference. Null is not a reference;
    /// object type, address validity and lifetime remain the caller's responsibility.
    pub fn from_ptr(reference: Ptr<TESObjectREFR>) -> Option<Self> {
        (!reference.is_null()).then_some(Self(reference))
    }

    pub fn as_ptr(self) -> Ptr<TESObjectREFR> {
        self.0
    }

    /// Reaches the embedded list through the PC getter (`005d43c0`).
    /// Returned handles alias the reference's list; no data is copied.
    pub fn extra_data(self, engine: &mut Engine) -> ExtraList {
        let list = engine.call(GET_EXTRA_LIST, &[self.0.addr()]).ptr();
        ExtraList::from_ptr(list).expect("initialized reference has an embedded extra-data list")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exe::Import;

    #[test]
    fn embedded_lists_alias_their_reference_and_keep_other_references_isolated() {
        let mut engine = Engine::new();
        engine.map(0x011c3000, 0x1000); // Generated lock/counter globals.
        engine.map(0x01012000, 0x1000);
        engine.mem.set_f32(0x01012054, -1.0);
        engine.map(0x01015000, 0x1000);
        engine.mem.set_u32(0x010158d8, 0x0041b680); // ExtraCount deleting destructor.
        let imports = [
            (0x00fdf1d0, "GetCurrentThreadId"),
            (0x00fdf1bc, "InterlockedCompareExchange"),
        ]
        .map(|(slot, name)| Import {
            slot,
            dll: "KERNEL32.dll".into(),
            name: name.into(),
        });
        assert_eq!(engine.register_imports(&imports), 2);
        assert!(Reference::from_ptr(Ptr::NULL).is_none());

        // Generated reference-sized storage with guarded surrounding fields.
        // Only embedded-list construction is exercised here; full reference
        // construction is covered separately by the installed-executable probe.
        let mut references = Vec::new();
        for _ in 0..2 {
            let object = engine.new_object::<TESObjectREFR>();
            engine.mem.write(object.addr(), &[0xcc; 0x68]);
            engine.call(0x00410360, &[object.addr() + 0x44]);
            references.push(Reference::from_ptr(object).unwrap());
        }
        let first = references[0];
        let second = references[1];
        let first_list = first.extra_data(&mut engine);
        let alias = Reference::from_ptr(first.as_ptr())
            .unwrap()
            .extra_data(&mut engine);
        let second_list = second.extra_data(&mut engine);
        assert_eq!(first_list, alias);
        assert_ne!(first_list, second_list);
        first_list.set_item_count(&mut engine, 5);
        first_list.set_health(&mut engine, 42.5);
        second_list.set_item_count(&mut engine, 9);
        assert_eq!(alias.item_count(&mut engine), 5);
        assert_eq!(alias.health(&mut engine), 42.5);
        assert_eq!(second_list.item_count(&mut engine), 9);
        assert_eq!(second_list.health(&mut engine), -1.0);
        alias.set_item_count(&mut engine, 1);
        assert_eq!(first_list.item_count(&mut engine), 1);
        assert_eq!(first_list.health(&mut engine), 42.5);
        assert_eq!(first_list.len(&mut engine), 1);
        assert_eq!(second_list.item_count(&mut engine), 9);
        for reference in references {
            let address = reference.as_ptr().addr();
            assert_eq!(engine.mem.bytes(address, 0x44), [0xcc; 0x44]);
            assert_eq!(engine.mem.u32(address + 0x64), 0xcccccccc);
            assert_eq!(engine.mem.block_size(address), Some(0x68));
        }
        assert_eq!(engine.mem.u32(0x011c3920), 0);
        assert_eq!(engine.mem.u32(0x011c3924), 0);
    }
}
