//! `BSMenu/tilerect.cpp` (Xbox PDB source unit), subsystem `BSMenu`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TileRect` is the plain rectangle tile: a bare `NiNode` that holds the
//! children's models. The unit also holds `Tile::Extra`, the extra data
//! (named `"Tileptr"`) that links a tile's node back to its tile.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsmenu::font::{
    release_after_use, NI_FIXED_STRING_CREATE, NI_FIXED_STRING_EMPTY,
};
use crate::units::bsmenu::tile::{Tile, TRAIT_VISIBLE, TRAIT_X, TRAIT_Y};

layout! {
    /// `TileRect` (Xbox PDB), 0x3C bytes: a `Tile` and a scissor flag.
    pub struct TileRect: 0x3c {
        /// `bIsScissor` (Xbox PDB).
        0x38 bIsScissor: bool,
    }

    /// `Tile::Extra` (Xbox PDB), 0x14 bytes: an `NiExtraData` (vtable,
    /// reference count, `m_kName`) and the tile it belongs to.
    pub struct TileExtra: 0x14 {
        /// `m_kName` (Xbox PDB, `NiExtraData`): `NiFixedString`.
        0x08 m_kName: u32,
        /// `pTile` (Xbox PDB).
        0x0C pTile: Ptr<Tile>,
        /// `pNode` (Xbox PDB): `NiNode*`.
        0x10 pNode: Ptr,
    }
}

/// `Tile::Init` (`009ff600`, Xbox PDB name).
const TILE_INIT: u32 = 0x009f_f600;
/// Sets a float trait (`00a012d0`, thiscall: trait, value, flag).
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
/// `Tile::SetNeedsUpdate` (`00a074d0`, Xbox PDB name).
const TILE_SET_NEEDS_UPDATE: u32 = 0x00a0_74d0;
/// `Tile::GetParentModel` (`00a089c0`, Xbox PDB name).
const TILE_GET_PARENT_MODEL: u32 = 0x00a0_89c0;
/// `Tile::IsTrue` (`00a01230`, Xbox PDB name).
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// The float value of a trait (`00a011b0`, thiscall: trait; `ST0`).
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// `Tile::FinalPostParse` (`00a037e0`, Xbox PDB name).
const TILE_FINAL_POST_PARSE: u32 = 0x00a0_37e0;
/// Gamebryo's object allocation (`00aa13e0`, cdecl: size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The `NiNode` constructor (`00a5ecb0`, NiMain/ninode.cpp; thiscall with a
/// child count).
const NI_NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
/// `NiPointer` assignment (`0066b0d0`, thiscall on the pointer slot).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `Interface::GetInterfaceRoot` (`00706d30`, Xbox PDB name, cdecl).
const INTERFACE_GET_INTERFACE_ROOT: u32 = 0x0070_6d30;
/// `Interface::SetNeedToUpdate` (`00707930`, Xbox PDB name, cdecl).
const INTERFACE_SET_NEED_TO_UPDATE: u32 = 0x0070_7930;
/// Sets an object's name from an `NiFixedString` (`00a5b950`, thiscall).
const NI_OBJECT_NET_SET_NAME: u32 = 0x00a5_b950;
/// `NiAVObject::UpdateProperties` (`00a5a040`, Xbox PDB name).
const NI_AV_OBJECT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `NiObjectNET::AddExtraData` (`00a5bca0`, Xbox PDB name `_ov2`).
const NI_OBJECT_NET_ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
/// The `NiExtraData` constructor (`00a7b2e0`, NiMain/niextradata.cpp).
const NI_EXTRA_DATA_CONSTRUCTOR: u32 = 0x00a7_b2e0;
/// The `NiNode` virtual slot the model is attached to its parent through
/// (`+0xDC`, with a second word 1).
const NODE_ATTACH_CHILD_SLOT: u32 = 0xdc;
/// `Tile::Extra`'s vtable (`01094cfc`).
const TILE_EXTRA_VTABLE: u32 = 0x0109_4cfc;
/// `"Tileptr"` (`010954cc`): the extra data's name.
const TILE_EXTRA_NAME: u32 = 0x0109_54cc;
/// `1.0f`: what `Init` sets the visible trait to (`FLD1`).
const ONE: f32 = 1.0;

// Translated from 00a1f370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileRect::Init` (Xbox PDB): `Tile::Init` with the same three words,
/// then the visible trait set to 1 (flag word 1) and the update bits 3
/// queued.
pub fn tile_rect_init(
    e: &mut Engine,
    this: Ptr<Tile>,
    parent: Ptr<Tile>,
    name: Ptr,
    set_parent_arg: u32,
) {
    e.call(TILE_INIT, &args![this, parent, name, set_parent_arg]);
    e.call(TILE_SET_FLOAT, &args![this, TRAIT_VISIBLE, ONE, 1u32]);
    e.call(TILE_SET_NEEDS_UPDATE, &args![this, 3u32]);
}

// Translated from 00a1f3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileRect::MakeNode` (Xbox PDB): makes the tile's model a new `NiNode`
/// (0xAC bytes; null when the allocation fails), attaches it (virtual slot
/// `+0xDC`, second word 1) to the nearest ancestor's model or else the
/// interface root, names it after the tile, sets bit 0 of its flags word
/// (+0x30) when the visible trait is false and clears it when true, updates
/// its properties, tells the interface it needs an update, adds a new
/// `Tile::Extra` (0x14 bytes; null when the allocation fails) for the tile
/// and the node, runs `FinalPostParse` on the x and y traits with their
/// current values, and returns the model.
// Translation incomplete: 00a1f3b0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn tile_rect_make_node(e: &mut Engine, this: Ptr<Tile>) -> Ptr {
    let mut parent = e.call(TILE_GET_PARENT_MODEL, &args![this]).u32();
    let block = e.call(NI_NEW, &args![0xacu32]).u32();
    let node = if block == 0 {
        0
    } else {
        e.call(NI_NODE_CONSTRUCTOR, &args![block, 0u32]).u32()
    };
    e.call(NI_POINTER_ASSIGN, &args![this.byte_add(0x2c), node]);
    if parent == 0 {
        parent = e.call(INTERFACE_GET_INTERFACE_ROOT, &args![]).u32();
    }
    let model = e.get(this, Tile::spModel);
    e.vcall(parent, NODE_ATTACH_CHILD_SLOT, &args![model, 1u32]);

    // `Tile::xName`'s text (`BSStringT` +0): an `NiFixedString` of it, or
    // the empty handle when the tile has no name.
    let name = e.mem.u32(this.addr() + 0x20);
    let handle = if name == 0 {
        e.global::<u32>(NI_FIXED_STRING_EMPTY)
    } else {
        e.call(NI_FIXED_STRING_CREATE, &args![name]).u32()
    };
    release_after_use(e, handle, |e, local| {
        let model = e.get(this, Tile::spModel);
        e.call(NI_OBJECT_NET_SET_NAME, &args![model, local]);
    });

    // `NiAVObject` flags word (+0x30): bit 0 set while the tile is hidden.
    let model = e.get(this, Tile::spModel);
    let visible = e.call(TILE_IS_TRUE, &args![this, TRAIT_VISIBLE]).bool();
    let flags = e.mem.u32(model.addr() + 0x30);
    if !visible {
        e.mem.set_u32(model.addr() + 0x30, flags | 1);
    } else {
        e.mem.set_u32(model.addr() + 0x30, flags & !1);
    }
    let model = e.get(this, Tile::spModel);
    e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![model]);
    e.call(INTERFACE_SET_NEED_TO_UPDATE, &args![1u32]);

    let block = e.call(NI_NEW, &args![0x14u32]).u32();
    let extra = if block == 0 {
        Ptr::NULL
    } else {
        let model = e.get(this, Tile::spModel);
        tile_extra_constructor(e, Ptr::new(block), this, model)
    };
    let model = e.get(this, Tile::spModel);
    e.call(NI_OBJECT_NET_ADD_EXTRA_DATA, &args![model, extra]);

    for trait_id in [TRAIT_X, TRAIT_Y] {
        let value = e.call(TILE_GET_FLOAT, &args![this, trait_id]).f32();
        e.call(TILE_FINAL_POST_PARSE, &args![this, trait_id, value, 0u32]);
    }
    e.get(this, Tile::spModel)
}

// Translated from 00a1f600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileRect::PostParse` (Xbox PDB): does nothing and returns 0; the three
/// stack words (`RET 0xC`) are not read.
pub fn tile_rect_post_parse(
    _e: &mut Engine,
    _this: Ptr<Tile>,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    0
}

// Translated from 00a1f610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Tile::Extra::Extra` (Xbox PDB): constructs the `NiExtraData` base,
/// stores the vtable, the tile and the node, and names it `"Tileptr"`
/// through a temporary `NiFixedString`. Returns `this`.
// Translation incomplete: 00a1f610 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
pub fn tile_extra_constructor(
    e: &mut Engine,
    this: Ptr<TileExtra>,
    tile: Ptr<Tile>,
    node: Ptr,
) -> Ptr<TileExtra> {
    e.call(NI_EXTRA_DATA_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), TILE_EXTRA_VTABLE);
    e.set(this, TileExtra::pTile, tile);
    e.set(this, TileExtra::pNode, node);
    // The code tests the name's address for null (a constant, so the empty
    // handle branch never runs; it is kept as the exe has it).
    let name = std::hint::black_box(TILE_EXTRA_NAME);
    let handle = if name == 0 {
        e.global::<u32>(NI_FIXED_STRING_EMPTY)
    } else {
        e.call(NI_FIXED_STRING_CREATE, &args![name]).u32()
    };
    release_after_use(e, handle, |e, local| {
        e.call(NI_OBJECT_NET_SET_NAME, &args![this, local]);
    });
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a1_f370, tile_rect_init(Ptr<Tile>, Ptr<Tile>, Ptr, u32)),
        entry!(0x00a1_f3b0, tile_rect_make_node(Ptr<Tile>) -> Ptr),
        entry!(
            0x00a1_f600,
            tile_rect_post_parse(Ptr<Tile>, u32, u32, u32) -> u32
        ),
        entry!(
            0x00a1_f610,
            tile_extra_constructor(Ptr<TileExtra>, Ptr<Tile>, Ptr) -> Ptr<TileExtra>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsmenu::font::INTERLOCKED_DECREMENT;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The attach slot's target in the tests' parent vtable.
    const ATTACH: u32 = 0x0000_1dc0;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn tile_rect_init_runs_the_base_and_shows_the_tile() {
        let mut e = Engine::new();
        quiet(&mut e, &[TILE_INIT, TILE_SET_FLOAT, TILE_SET_NEEDS_UPDATE]);
        e.call_log = Some(vec![]);
        e.call(0x00a1_f370, &args![0x5000u32, 0x6000u32, 0x7000u32, 9u32]);
        assert_eq!(
            calls_to(&e, TILE_INIT),
            vec![vec![0x5000, 0x6000, 0x7000, 9]]
        );
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x5000, 0xfa3, 1.0f32.to_bits(), 1]]
        );
        assert_eq!(calls_to(&e, TILE_SET_NEEDS_UPDATE), vec![vec![0x5000, 3]]);
    }

    /// Doubles for `MakeNode`: a heap allocator, a node constructor that
    /// returns its block, the pointer assignment storing into the slot, the
    /// fixed string handle `0x9008` (count 8 bytes before).
    fn make_node_engine(visible: bool, parent_model: u32, named: bool) -> (Engine, Ptr<Tile>) {
        let mut e = Engine::new();
        e.map(NI_FIXED_STRING_EMPTY, 4);
        e.set_global(NI_FIXED_STRING_EMPTY, 0x0000_7770u32);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NI_NODE_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register_double(TILE_GET_PARENT_MODEL, move |_, _| Ret {
            eax: parent_model,
            ..Ret::default()
        });
        e.register(INTERFACE_GET_INTERFACE_ROOT, |_, _| Ret {
            eax: 0x0004_0000,
            ..Ret::default()
        });
        e.register(NI_FIXED_STRING_CREATE, |_, _| Ret {
            eax: 0x9008,
            ..Ret::default()
        });
        e.register_double(TILE_IS_TRUE, move |_, _| Ret {
            eax: visible as u32,
            ..Ret::default()
        });
        e.register(TILE_GET_FLOAT, |_, a| Ret {
            st0: if a[1] == 0xfa1 { 12.5 } else { -3.0 },
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                ATTACH,
                NI_OBJECT_NET_SET_NAME,
                INTERLOCKED_DECREMENT,
                NI_AV_OBJECT_UPDATE_PROPERTIES,
                INTERFACE_SET_NEED_TO_UPDATE,
                NI_EXTRA_DATA_CONSTRUCTOR,
                NI_OBJECT_NET_ADD_EXTRA_DATA,
                TILE_FINAL_POST_PARSE,
            ],
        );
        // Both possible parents share one vtable with the attach slot.
        e.put_vtable(0x0003_0000, &[0; 0x38]);
        e.mem.set_u32(0x0003_0000 + NODE_ATTACH_CHILD_SLOT, ATTACH);
        for parent in [0x0004_0000u32, 0x0004_1000] {
            e.map(parent, 4);
            e.mem.set_u32(parent, 0x0003_0000);
        }
        let this = e.new_object::<Tile>();
        if named {
            let name = e.mem.alloc(8);
            e.mem.set_cstr(name, b"rect");
            e.mem.set_u32(this.addr() + 0x20, name);
        }
        (e, this)
    }

    #[test]
    fn tile_rect_make_node_builds_names_and_links_the_node() {
        let (mut e, this) = make_node_engine(false, 0, true);
        e.call_log = Some(vec![]);
        let model = e.call(0x00a1_f3b0, &args![this]).u32();
        assert_ne!(model, 0);
        assert_eq!(e.get(this, Tile::spModel).addr(), model);
        assert_eq!(calls_to(&e, NI_NEW)[0], vec![0xac]);
        assert_eq!(calls_to(&e, NI_NODE_CONSTRUCTOR), vec![vec![model, 0]]);
        // No parent model: attached to the interface root.
        assert_eq!(calls_to(&e, ATTACH), vec![vec![0x0004_0000, model, 1]]);
        let name = e.mem.u32(this.addr() + 0x20);
        assert_eq!(calls_to(&e, NI_FIXED_STRING_CREATE)[0], vec![name]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![0x9000], vec![0x9000]]
        );
        // Hidden: bit 0 set.
        assert_eq!(e.mem.u32(model + 0x30) & 1, 1);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_UPDATE_PROPERTIES),
            vec![vec![model]]
        );
        assert_eq!(calls_to(&e, INTERFACE_SET_NEED_TO_UPDATE), vec![vec![1]]);
        let extra = calls_to(&e, NI_OBJECT_NET_ADD_EXTRA_DATA)[0].clone();
        assert_eq!(extra[0], model);
        let extra = Ptr::<TileExtra>::new(extra[1]);
        assert_eq!(e.mem.u32(extra.addr()), TILE_EXTRA_VTABLE);
        assert_eq!(e.get(extra, TileExtra::pTile), this);
        assert_eq!(e.get(extra, TileExtra::pNode).addr(), model);
        assert_eq!(
            calls_to(&e, TILE_FINAL_POST_PARSE),
            vec![
                vec![this.addr(), 0xfa1, 12.5f32.to_bits(), 0],
                vec![this.addr(), 0xfa2, (-3.0f32).to_bits(), 0]
            ]
        );
    }

    #[test]
    fn tile_rect_make_node_uses_the_parent_model_and_the_empty_name() {
        let (mut e, this) = make_node_engine(true, 0x0004_1000, false);
        e.call_log = Some(vec![]);
        let model = e.call(0x00a1_f3b0, &args![this]).u32();
        assert!(calls_to(&e, INTERFACE_GET_INTERFACE_ROOT).is_empty());
        assert_eq!(calls_to(&e, ATTACH), vec![vec![0x0004_1000, model, 1]]);
        // The node's name is the empty handle, whose release is skipped;
        // only the extra data's "Tileptr" is created and released.
        assert_eq!(calls_to(&e, NI_OBJECT_NET_SET_NAME).len(), 2);
        assert_eq!(
            calls_to(&e, NI_FIXED_STRING_CREATE),
            vec![vec![TILE_EXTRA_NAME]]
        );
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![0x9000]]);
        assert_eq!(e.mem.u32(model + 0x30) & 1, 0);
    }

    #[test]
    fn tile_rect_make_node_survives_failed_allocations() {
        let (mut e, this) = make_node_engine(true, 0x0004_1000, false);
        e.register(NI_NEW, |_, _| Ret::default());
        // A null model: the flags word is read and written at +0x30.
        e.map(0, 0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a1_f3b0, &args![this]).u32(), 0);
        assert!(calls_to(&e, NI_NODE_CONSTRUCTOR).is_empty());
        assert!(calls_to(&e, NI_EXTRA_DATA_CONSTRUCTOR).is_empty());
        assert_eq!(calls_to(&e, NI_OBJECT_NET_ADD_EXTRA_DATA), vec![vec![0, 0]]);
    }

    #[test]
    fn tile_rect_post_parse_answers_zero() {
        let mut e = Engine::new();
        let result = e.call(0x00a1_f600, &args![0x5000u32, 1u32, 2u32, 3u32]);
        assert_eq!(result.u32(), 0);
    }

    #[test]
    fn tile_extra_constructor_stores_the_tile_and_names_itself() {
        let mut e = Engine::new();
        let names = Rc::new(RefCell::new(vec![]));
        let seen = names.clone();
        e.register(NI_FIXED_STRING_CREATE, |_, _| Ret {
            eax: 0x9008,
            ..Ret::default()
        });
        e.register_double(NI_OBJECT_NET_SET_NAME, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.map(NI_FIXED_STRING_EMPTY, 4);
        quiet(&mut e, &[NI_EXTRA_DATA_CONSTRUCTOR, INTERLOCKED_DECREMENT]);
        let this = e.new_object::<TileExtra>();
        e.call_log = Some(vec![]);
        let result = e.call(0x00a1_f610, &args![this, 0x5000u32, 0x6000u32]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(
            calls_to(&e, NI_EXTRA_DATA_CONSTRUCTOR),
            vec![vec![this.addr()]]
        );
        assert_eq!(e.mem.u32(this.addr()), TILE_EXTRA_VTABLE);
        assert_eq!(e.get(this, TileExtra::pTile).addr(), 0x5000);
        assert_eq!(e.get(this, TileExtra::pNode).addr(), 0x6000);
        assert_eq!(
            calls_to(&e, NI_FIXED_STRING_CREATE),
            vec![vec![TILE_EXTRA_NAME]]
        );
        assert_eq!(*names.borrow(), vec![(this.addr(), 0x9008)]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![0x9000]]);
    }
}
