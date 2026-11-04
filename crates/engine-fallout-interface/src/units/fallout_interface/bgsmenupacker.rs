//! `fallout/interface/bgsmenupacker.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;

// Translated from 007037c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at +0x24. The PC body is shared by otherwise unrelated
/// callers, including BGSLoadFormBuffer::SetForm (Xbox PDB); the map's
/// BGSMenuPacker::RecomputePacking (Xbox PDB) name does not describe this store.
pub fn fn_007037c0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x24), value);
}

/// Cdecl (format, ...): the interface's debug print (`00703c00`,
/// `interface.cpp`).
const PRINT_00703C00: u32 = 0x0070_3c00;
/// Thiscall on the packer: its number of entries (`00658930`) and the
/// address of entry `index` (`00877a30`, `RET 4`).
const ENTRY_COUNT_00658930: u32 = 0x0065_8930;
const ENTRY_ADDRESS_00877A30: u32 = 0x0087_7a30;
/// Thiscall on a tile: a related tile (`0045cd60`), its name (`00700300`),
/// and its node (`0056c7f0`), whose position `0043c490` gives the address
/// of.
const TILE_RELATED_0045CD60: u32 = 0x0045_cd60;
const TILE_NAME_00700300: u32 = 0x0070_0300;
const TILE_NODE_0056C7F0: u32 = 0x0056_c7f0;
const NODE_POSITION_0043C490: u32 = 0x0043_c490;
/// `"==== Begin PACK AREA Output ===="`, `"Final packing for glow branch %s
/// of tile %s: "`, `"  Pos: ( %f, %f )    [Original Pos: ( %f, %f )]"`,
/// `"  Dim: ( %f, %f )    [Node Pos: ( %f, %f )]"` and `"==== End PACK
/// AREA Output ===="`.
const BEGIN_TEXT: u32 = 0x0106_e8ac;
const BRANCH_TEXT: u32 = 0x0106_e87c;
const POSITION_TEXT: u32 = 0x0106_e84c;
const DIMENSION_TEXT: u32 = 0x0106_e820;
const END_TEXT: u32 = 0x0106_e800;

/// A float field as the print gets it: `FLD float`, `FSTP double`.
fn float_as_double(e: &mut Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// The address of the position of `tile`'s node (`0056c7f0`, `0043c490`).
fn node_position(e: &mut Engine, tile: u32) -> u32 {
    let node = e.call(TILE_NODE_0056C7F0, &args![tile]).u32();
    e.call(NODE_POSITION_0043C490, &args![node]).u32()
}

// Translated from 00700350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMenuPacker::DumpInfo` (engine map; `thiscall`, plain `RET`): prints
/// (`00703c00`) the begin line, then for each entry (the count read again
/// each time) the names of its tile (+0x18) and of the tile `0045cd60`
/// relates to it, its position (+0, +4) and original position (+0x10,
/// +0x14), its dimensions (+8, +0xc) and its tile's node position (+0 and
/// +8 of what `0043c490` gives), the floats passed as doubles; then the end
/// line.
pub fn bgs_menu_packer_dump_info(e: &mut Engine, this: Ptr) {
    e.call(PRINT_00703C00, &args![BEGIN_TEXT]);
    let mut index = 0i32;
    while index < e.call(ENTRY_COUNT_00658930, &args![this]).i32() {
        let slot = e.call(ENTRY_ADDRESS_00877A30, &args![this, index]).u32();
        let entry = e.mem.u32(slot);
        let tile = e.mem.u32(entry.wrapping_add(0x18));
        let related = e.call(TILE_RELATED_0045CD60, &args![tile]).u32();
        let related_name = e.call(TILE_NAME_00700300, &args![related]).u32();
        let tile = e.mem.u32(entry.wrapping_add(0x18));
        let tile_name = e.call(TILE_NAME_00700300, &args![tile]).u32();
        e.call(PRINT_00703C00, &args![BRANCH_TEXT, tile_name, related_name]);
        let original_y = float_as_double(e, entry.wrapping_add(0x14));
        let original_x = float_as_double(e, entry.wrapping_add(0x10));
        let y = float_as_double(e, entry.wrapping_add(4));
        let x = float_as_double(e, entry);
        e.call(
            PRINT_00703C00,
            &args![POSITION_TEXT, x, y, original_x, original_y],
        );
        let tile = e.mem.u32(entry.wrapping_add(0x18));
        let position = node_position(e, tile);
        let node_z = float_as_double(e, position.wrapping_add(8));
        let tile = e.mem.u32(entry.wrapping_add(0x18));
        let position = node_position(e, tile);
        let node_x = float_as_double(e, position);
        let height = float_as_double(e, entry.wrapping_add(0xc));
        let width = float_as_double(e, entry.wrapping_add(8));
        e.call(
            PRINT_00703C00,
            &args![DIMENSION_TEXT, width, height, node_x, node_z],
        );
        index += 1;
    }
    e.call(PRINT_00703C00, &args![END_TEXT]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007037c0, fn_007037c0(Ptr, u32)),
        entry!(0x0070_0350, bgs_menu_packer_dump_info(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_setter_keeps_neighbors_and_accepts_all_word_bits() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x2c);
        e.mem.write(object, &[0xa5; 0x2c]);
        for value in [0, 0x2000_0040, u32::MAX] {
            e.call(0x007037c0, &[object, value]);
            assert_eq!(e.mem.u32(object + 0x24), value);
            assert_eq!(e.mem.bytes(object, 0x24), vec![0xa5; 0x24]);
            assert_eq!(e.mem.u32(object + 0x28), 0xa5a5_a5a5);
        }
    }
}

#[cfg(test)]
mod dump_tests {
    use super::*;

    fn double_words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    #[test]
    fn dump_prints_each_entry() {
        let mut e = Engine::new();
        // The packer: +0 the entry count, +4 the address of the slots.
        let packer = e.mem.alloc(8);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(packer, 1);
        e.mem.set_u32(packer + 4, slots);
        let entry = e.mem.alloc(0x1c);
        for (i, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].iter().enumerate() {
            e.mem.set_u32(entry + 4 * i as u32, value.to_bits());
        }
        let tile = 0x0000_7000;
        e.mem.set_u32(entry + 0x18, tile);
        e.mem.set_u32(slots, entry);
        let position = e.mem.alloc(12);
        e.mem.set_u32(position, 7.0f32.to_bits());
        e.mem.set_u32(position + 8, 9.0f32.to_bits());
        e.register(ENTRY_COUNT_00658930, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ENTRY_ADDRESS_00877A30, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(TILE_RELATED_0045CD60, |_, a| (a[0] + 1).into_ret());
        e.register(TILE_NAME_00700300, |_, a| (a[0] + 0x100).into_ret());
        e.register(TILE_NODE_0056C7F0, |_, a| (a[0] + 2).into_ret());
        e.register_double(NODE_POSITION_0043C490, move |_, _| position.into_ret());
        e.register(PRINT_00703C00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0070_0350, &args![packer]);
        let prints: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == PRINT_00703C00)
            .map(|(_, w)| w)
            .collect();
        let mut position_words = vec![POSITION_TEXT];
        let mut dimension_words = vec![DIMENSION_TEXT];
        for value in [1.0, 2.0, 5.0, 6.0] {
            position_words.extend(double_words(value));
        }
        for value in [3.0, 4.0, 7.0, 9.0] {
            dimension_words.extend(double_words(value));
        }
        assert_eq!(
            prints,
            vec![
                vec![BEGIN_TEXT],
                vec![BRANCH_TEXT, tile + 0x100, tile + 1 + 0x100],
                position_words,
                dimension_words,
                vec![END_TEXT],
            ]
        );
        // No entries: only the two lines.
        e.mem.set_u32(packer, 0);
        e.call_log = Some(vec![]);
        bgs_menu_packer_dump_info(&mut e, Ptr::new(packer));
        let count = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == PRINT_00703C00)
            .count();
        assert_eq!(count, 2);
    }
}
