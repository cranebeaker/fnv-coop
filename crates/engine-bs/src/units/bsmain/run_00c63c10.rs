//! `BSMain/run_00c63c10` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global holding the address of the extra-data name handle (`012043d0`), made on
/// first use.
const NAME_HOLDER: u32 = 0x0120_43d0;
/// `"DVPG"` (`01020504`), the extra data's name, and `"DecalPlacementVectorGroup"`
/// (`010c3940`), the node-name prefix searched for.
const EXTRA_DATA_NAME_TEXT: u32 = 0x0102_0504;
const GROUP_PREFIX_TEXT: u32 = 0x010c_3940;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, `RET 4`);
/// `NiNew` (`00aa13e0`, `cdecl`: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const NI_NEW: u32 = 0x00aa_13e0;
/// The pooled `NiFixedString` maker (`00a5b690`, `cdecl`: text) and the empty handle
/// (`0109b220`).
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// `NiObjectNET::GetExtraData` (`00a5bdd0`, `thiscall`, `RET 4`: address of the name) and
/// `NiObjectNET::AddExtraData` (`00a5bc40`, `thiscall`, `RET 8`: address of the name, the
/// data).
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const ADD_EXTRA_DATA: u32 = 0x00a5_bc40;
/// `BSstristr` (`004b75a0`, engine map; `cdecl`: text, text searched for).
const BS_STRISTR: u32 = 0x004b_75a0;
/// The extra data's constructor (`00c63ab0`, `thiscall`, `RET 4`) and the vector group's
/// (`00c64640`, `thiscall`).
const GROUPS_CONSTRUCTOR: u32 = 0x00c6_3ab0;
const GROUP_CONSTRUCTOR: u32 = 0x00c6_4640;
/// The `BSString` stack local's set from a text (`0040c0e0`, `thiscall`, `RET 4`) and its
/// release (`004037f0`, `thiscall`, `RET 8`).
const BS_STRING_SET: u32 = 0x0040_c0e0;
const BS_STRING_RELEASE: u32 = 0x0040_37f0;
/// `strlen` (`00ec6130`) and `atol` (`00eca6d3`), both `cdecl`.
const STRLEN: u32 = 0x00ec_6130;
const ATOL: u32 = 0x00ec_a6d3;
/// The vector normalisation (`004a0c10`, `thiscall` on the vector).
const UNITIZE: u32 = 0x004a_0c10;
/// The object virtual slot `+0x0C` (`IsNode`).
const SLOT_IS_NODE: u32 = 0x0c;
/// `NiAVObject`'s local rotation (`m_kLocal`, Xbox PDB; nine words at +0x34) and
/// translation (+0x58); `NiNode`'s children: elements at +0xA0, size at +0xA6 and the count
/// of set slots at +0xA8.
const LOCAL_ROTATION: u32 = 0x34;
const LOCAL_TRANSLATION: u32 = 0x58;
const CHILDREN: u32 = 0xa0;
const CHILDREN_SIZE: u32 = 0xa6;
const CHILDREN_SET: u32 = 0xa8;

// Translated from 00c63c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): [`fn_00c63c30`] with the node as both the
/// node and the root; answers its byte.
pub fn fn_00c63c10(e: &mut Engine, node: Ptr) -> bool {
    fn_00c63c30(e, node, node)
}

// Translated from 00c63c30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c63c30 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30478, state variable) is not translated
/// Collects decal placement vector groups (unnamed in the engine map; `cdecl`: node, root).
/// False for an object that is not a node (`IsNode`). The `"DVPG"` name handle is made once
/// (4 bytes from the memory manager holding `00a5b690` of the text; the compiler kept a test
/// of the constant text's address, so the branch storing the empty handle never runs,
/// translated anyway) and the root's extra data of that name looked up. For a node whose
/// name contains `"DecalPlacementVectorGroup"` (`BSstristr`, any case): the extra data is
/// made when missing (`NiNew` 0x40, `00c63ab0` with 0) and added to the root; the group
/// number after the prefix (`atol`, past the length of a `BSString` of the prefix) selects a
/// slot (+0x14, 8 of them); when fewer than 8 groups exist, the number is below 8 and the
/// slot is free, a new group (0xC4 bytes from the memory manager, `00c64640`) gets, for each
/// of the first 8 children (by the count of set slots), the child's translation and the
/// third column of its rotation, normalised (`004a0c10`); the group goes to the slot and the
/// group count (16 bits, +0x10) goes up. Such a node answers true. Other nodes visit their
/// non-null children and answer false.
pub fn fn_00c63c30(e: &mut Engine, node: Ptr, root: Ptr) -> bool {
    let node = node.addr();
    if e.vcall(node, SLOT_IS_NODE, &args![]).u32() == 0 {
        return false;
    }
    if e.mem.u32(NAME_HOLDER) == 0 {
        let block = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 4u32])
            .u32();
        let holder = if block == 0 {
            0
        } else {
            let text = std::hint::black_box(EXTRA_DATA_NAME_TEXT);
            let handle = if text == 0 {
                e.mem.u32(FIXED_STRING_EMPTY)
            } else {
                e.call(FIXED_STRING_FROM_TEXT, &args![text]).u32()
            };
            e.mem.set_u32(block, handle);
            block
        };
        e.mem.set_u32(NAME_HOLDER, holder);
    }
    let holder = e.mem.u32(NAME_HOLDER);
    let mut groups = e.call(GET_EXTRA_DATA, &args![root, holder]).u32();
    let name = e.mem.u32(node + 8);
    if name == 0 || e.call(BS_STRISTR, &args![name, GROUP_PREFIX_TEXT]).u32() == 0 {
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(node + CHILDREN_SIZE)) {
            let child = e
                .mem
                .u32(e.mem.u32(node + CHILDREN).wrapping_add(index * 4));
            if child != 0 {
                // The answer only clears a local the exe never reads again.
                fn_00c63c30(e, Ptr::new(child), root);
            }
            index = index.wrapping_add(1);
        }
        return false;
    }
    if groups == 0 {
        let block = e.call(NI_NEW, &args![0x40u32]).u32();
        groups = if block == 0 {
            0
        } else {
            e.call(GROUPS_CONSTRUCTOR, &args![block, 0u32]).u32()
        };
        let holder = e.mem.u32(NAME_HOLDER);
        e.call(ADD_EXTRA_DATA, &args![root, holder, groups]);
    }
    e.with_stack(0x18, |e, locals| {
        let text = locals.addr();
        let vector = text + 8;
        e.call(BS_STRING_SET, &args![text, GROUP_PREFIX_TEXT]);
        let name = e.mem.u32(node + 8);
        let length = if e.mem.u16(text + 4) == 0xffff {
            let chars = e.mem.u32(text);
            e.call(STRLEN, &args![chars]).u32()
        } else {
            u32::from(e.mem.u16(text + 4))
        };
        let number = e.call(ATOL, &args![name.wrapping_add(length)]).i32();
        let slot = groups
            .wrapping_add((number as u32).wrapping_mul(4))
            .wrapping_add(0x14);
        if i32::from(e.mem.u16(groups + 0x10)) < 8 && number < 8 && e.mem.u32(slot) == 0 {
            let block = e
                .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0xc4u32])
                .u32();
            let group = if block == 0 {
                0
            } else {
                e.call(GROUP_CONSTRUCTOR, &args![block]).u32()
            };
            let mut index = 0i32;
            while index < i32::from(e.mem.u16(node + CHILDREN_SET)) {
                if index < 8 {
                    let children = e.mem.u32(node + CHILDREN);
                    let child = e.mem.u32(children.wrapping_add(index as u32 * 4));
                    let rotation = child.wrapping_add(LOCAL_ROTATION);
                    for (k, element) in [2u32, 5, 8].iter().enumerate() {
                        let word = x87_quiet_bits(e.mem.u32(rotation + 4 * element));
                        e.mem.set_u32(vector + 4 * k as u32, word);
                    }
                    e.call(UNITIZE, &args![vector]);
                    let count = u32::from(e.mem.u16(group));
                    for k in 0..3u32 {
                        let word = e.mem.u32(child.wrapping_add(LOCAL_TRANSLATION) + 4 * k);
                        e.mem.set_u32(group + 4 + count * 0xc + 4 * k, word);
                    }
                    let count = u32::from(e.mem.u16(group));
                    for k in 0..3u32 {
                        let word = e.mem.u32(vector + 4 * k);
                        e.mem.set_u32(group + 0x64 + count * 0xc + 4 * k, word);
                    }
                    let count = e.mem.u16(group).wrapping_add(1);
                    e.mem.set_u16(group, count);
                }
                index = index.wrapping_add(1);
            }
            let slot = groups
                .wrapping_add((number as u32).wrapping_mul(4))
                .wrapping_add(0x14);
            e.mem.set_u32(slot, group);
            let count = e.mem.u16(groups + 0x10).wrapping_add(1);
            e.mem.set_u16(groups + 0x10, count);
        }
        e.call(BS_STRING_RELEASE, &args![text, 0u32, 0u32]);
    });
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_3c10, fn_00c63c10(Ptr) -> bool),
        entry!(0x00c6_3c30, fn_00c63c30(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    const NODE_VTABLE: u32 = 0x0300_c000;
    const IS_NODE: u32 = 0x00ee_f000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.put_vtable(NODE_VTABLE, &[0, 0, 0, IS_NODE]);
        e.register(IS_NODE, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.map(NAME_HOLDER, 4);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(FIXED_STRING_FROM_TEXT, |_, _| 0x0600_0008u32.into_ret());
        e.register(BS_STRISTR, |e, a| {
            let name = String::from_utf8(e.mem.cstr(a[0])).unwrap().to_lowercase();
            let found = name.find("decalplacementvectorgroup");
            found.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.register(BS_STRING_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u16(a[0] + 4, 0xffff);
            a[0].into_ret()
        });
        e.register(STRLEN, |_, _| 25u32.into_ret());
        e.register(ATOL, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            text.parse::<i32>().unwrap_or(0).into_ret()
        });
        e.register(UNITIZE, |e, a| {
            let v = e.mem.f32(a[0]) * 0.5;
            e.mem.set_f32(a[0], v);
            Ret::default()
        });
        e.register(BS_STRING_RELEASE, |_, _| Ret::default());
        e.register(ADD_EXTRA_DATA, |_, _| Ret::default());
        e
    }

    /// An object (`is_node` decides `IsNode`) with a name and children.
    fn object(e: &mut Engine, is_node: bool, name: &[u8], children: &[u32]) -> u32 {
        let o = e.mem.alloc(0xb0);
        e.mem.set_u32(o, NODE_VTABLE);
        e.mem.set_u32(o + 4, u32::from(!is_node));
        if !name.is_empty() {
            let text = e.mem.alloc(name.len() as u32 + 1);
            e.mem.set_cstr(text, name);
            e.mem.set_u32(o + 8, text);
        }
        let list = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *child);
        }
        e.mem.set_u32(o + CHILDREN, list);
        e.mem.set_u16(o + CHILDREN_SIZE, children.len() as u16);
        e.mem.set_u16(o + CHILDREN_SET, children.len() as u16);
        o
    }

    #[test]
    fn a_group_node_fills_a_slot_from_its_children() {
        let mut e = engine();
        let groups = e.mem.alloc(0x40);
        let group = e.mem.alloc(0xc4);
        e.register_double(GET_EXTRA_DATA, move |_, _| groups.into_ret());
        e.register_double(GROUP_CONSTRUCTOR, move |_, _| group.into_ret());
        let child = object(&mut e, false, b"", &[]);
        e.mem.set_f32(child + LOCAL_ROTATION + 8, 4.0);
        e.mem.set_f32(child + LOCAL_TRANSLATION + 4, 7.0);
        let marker = object(&mut e, true, b"DecalPlacementVectorGroup3", &[child]);
        let root = object(&mut e, true, b"root", &[0, marker]);
        assert!(!e.call(0x00c6_3c30, &args![root, root]).bool());
        assert_ne!(e.mem.u32(NAME_HOLDER), 0);
        assert_eq!(e.mem.u32(e.mem.u32(NAME_HOLDER)), 0x0600_0008);
        assert_eq!(e.mem.u32(groups + 0x14 + 3 * 4), group);
        assert_eq!(e.mem.u16(groups + 0x10), 1);
        assert_eq!(e.mem.u16(group), 1);
        assert_eq!(e.mem.f32(group + 4 + 4), 7.0);
        assert_eq!(e.mem.f32(group + 0x64), 2.0);
        // The slot is taken now: a second visit adds nothing.
        assert!(e.call(0x00c6_3c10, &args![marker]).bool());
        assert_eq!(e.mem.u16(groups + 0x10), 1);
    }

    #[test]
    fn missing_extra_data_is_made_and_non_nodes_answer_false() {
        let mut e = engine();
        e.register(GET_EXTRA_DATA, |_, _| 0u32.into_ret());
        let groups = e.mem.alloc(0x40);
        e.register_double(NI_NEW, move |_, _| groups.into_ret());
        e.register(GROUPS_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(GROUP_CONSTRUCTOR, |_, a| a[0].into_ret());
        let marker = object(&mut e, true, b"decalplacementvectorgroup9", &[]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c6_3c10, &args![marker]).bool());
        let log = e.call_log.take().unwrap();
        let holder = e.mem.u32(NAME_HOLDER);
        assert!(log.contains(&(ADD_EXTRA_DATA, vec![marker, holder, groups])));
        // Group 9 is out of range: no group made.
        assert!(!log.iter().any(|c| c.0 == GROUP_CONSTRUCTOR));
        let leaf = object(&mut e, false, b"DecalPlacementVectorGroup1", &[]);
        assert!(!e.call(0x00c6_3c10, &args![leaf]).bool());
    }
}
