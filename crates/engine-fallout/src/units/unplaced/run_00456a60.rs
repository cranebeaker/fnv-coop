//! `(unplaced)/run_00456a60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte global `00456c70` returns.
const FLAG_011C3C0D: u32 = 0x011c_3c0d;
/// `GridCellArray::Get` (Xbox PDB): thiscall(x, y).
const GRID_CELL_ARRAY_GET: u32 = 0x004b_a490;
/// `TESObjectCELL::Get3D` (Xbox PDB): thiscall, no stack words.
const CELL_GET_3D: u32 = 0x0054_5cb0;
/// `0043b4a0` (`modelloader.cpp`): thiscall(word) on the cell's 3D.
const NODE_LOOKUP_43B4A0: u32 = 0x0043_b4a0;

// Translated from 00456c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `011c3c0d` (in `AL`).
pub fn fn_00456c70(e: &mut Engine) -> u8 {
    e.global::<u8>(FLAG_011C3C0D)
}

// Translated from 00456fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For the cell at `this`: when `TESObjectCELL::Get3D` gives a node, the
/// result of `0043b4a0(node, word)`; null otherwise.
pub fn fn_00456fc0(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    let node = e.call(CELL_GET_3D, &args![this]).u32();
    if node == 0 {
        0
    } else {
        e.call(NODE_LOOKUP_43B4A0, &args![node, word]).u32()
    }
}

// Translated from 00457050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridCellArray::Get(x, y)` on the grid at `this + 8`, its result passed
/// through (`EAX` is left as the callee set it).
pub fn fn_00457050(e: &mut Engine, this: Ptr, x: u32, y: u32) -> u32 {
    let grid = e.mem.u32(this.addr().wrapping_add(8));
    e.call(GRID_CELL_ARRAY_GET, &args![grid, x, y]).u32()
}

/// The `TES` singleton pointer.
const TES: u32 = 0x011d_ea10;
/// `005f36f0` (`ECX` = the `TES`): `TES::pInteriorCell` (docs/ENGINE_CRATE.md: the engine
/// map's `ActorMover::GetPreferredMoveMode` name is folded code).
const INTERIOR_CELL: u32 = 0x005f_36f0;
/// `00408d60` (`ECX`): the address of the byte at +4 of the object.
const TEXT_ADDRESS: u32 = 0x0040_8d60;
/// The objects whose +4 bytes [`fn_00456aa0`] and [`fn_00456c90`] toggle and write.
const TOGGLE_BYTE_OWNER: u32 = 0x011c_a2dc;
const RAGDOLL_BYTE_OWNER: u32 = 0x0126_7c24;
/// The flag words [`fn_00457000`] sets bits in.
const NODE_FLAGS: u32 = 0x011c_a08c;
/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `004543c0` (`ECX` = the interior cell) and `00451010` (no arguments): the physics world
/// of an interior and of the exterior.
const CELL_WORLD: u32 = 0x0045_43c0;
const EXTERIOR_WORLD: u32 = 0x0045_1010;
/// `00406d30` and `00406d50` (`cdecl`, three words: destination, size, source): the CRT's
/// string copy and append.
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_APPEND: u32 = 0x0040_6d50;
/// `".hkx"`.
const HKX_EXTENSION: u32 = 0x0101_7cec;
/// `00c67430` (`bhkworld.obj`; `ECX` = the world, one stack word: the file name).
const WORLD_SAVE_00C67430: u32 = 0x00c6_7430;
/// `00454380` (`ECX` = the world, one stack word).
const WORLD_TOGGLE_00454380: u32 = 0x0045_4380;
/// `0043d4d0` (`ECX` = a setting): the address of its value.
const SETTING_VALUE: u32 = 0x0043_d4d0;
/// `00450f90` (`ECX` = a node, one stack word: the flag).
const NODE_SET_HIDDEN_00450F90: u32 = 0x0045_0f90;
/// `004e7ff0` (no arguments): the water node.
const WATER_NODE_004E7FF0: u32 = 0x004e_7ff0;
/// `004518e0` (`ECX` = the interior cell), `AL`.
const CELL_TEST_004518E0: u32 = 0x0045_18e0;
/// `00557990` (`ECX` = a cell, two stack words: the flag and the node kind).
const CELL_TOGGLE_00557990: u32 = 0x0055_7990;

/// The physics world [`fn_00456b10`] and [`fn_00456be0`] use: the interior cell's when
/// the `TES` has one, else the exterior's.
fn current_world(e: &mut Engine) -> u32 {
    let tes: u32 = e.global(TES);
    let cell = e.call(INTERIOR_CELL, &args![tes]).u32();
    if cell != 0 {
        e.call(CELL_WORLD, &args![cell]).u32()
    } else {
        e.call(EXTERIOR_WORLD, &args![]).u32()
    }
}

// Translated from 00456a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` = the `TES`, plain `RET`; called by `005b9220` and
/// `005b9240`): `00554910` on the interior cell when there is one, else `004bad40` on the
/// grid at `+8`. Answers the callee's `EAX`.
pub fn fn_00456a60(e: &mut Engine, this: Ptr) -> u32 {
    let cell = e.call(INTERIOR_CELL, &args![this]).u32();
    if cell != 0 {
        e.call(0x0055_4910, &args![cell]).u32()
    } else {
        let grid = e.mem.u32(this.addr() + 8);
        e.call(0x004b_ad40, &args![grid]).u32()
    }
}

// Translated from 00456aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` = the `TES`, plain `RET`; called by `005b9500`):
/// toggles the byte at `011ca2e0` (through `00408d60` on `011ca2dc`: set to whether it was
/// 0), then hands the new byte to `005549d0` on the interior cell when there is one, else to
/// `004bae20` on the grid at `+8`. Answers the callee's `EAX`.
pub fn fn_00456aa0(e: &mut Engine, this: Ptr) -> u32 {
    let cell = e.call(INTERIOR_CELL, &args![this]).u32();
    let flag = e.call(TEXT_ADDRESS, &args![TOGGLE_BYTE_OWNER]).u32();
    let current = e.call(TEXT_ADDRESS, &args![TOGGLE_BYTE_OWNER]).u32();
    let toggled = u8::from(e.mem.u8(current) == 0);
    e.mem.set_u8(flag, toggled);
    let value = u32::from(e.mem.u8(flag));
    if cell != 0 {
        e.call(0x0055_49d0, &args![cell, value]).u32()
    } else {
        let grid = e.mem.u32(this.addr() + 8);
        e.call(0x004b_ae20, &args![grid, value]).u32()
    }
}

// Translated from 00456b10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00456b10 - the /GS cookie check (00ec408c) on the frame's file-name buffer is not translated.
/// Unnamed in the engine map (`ECX` not read, one stack word, `RET 4`; called by
/// `005b9300`): with a non-null `name`, saves the current physics world (the interior
/// cell's, `004543c0`, or the exterior's, `00451010`) through `00c67430` under `name` with
/// `".hkx"` appended (a 0x104-byte frame buffer: `00406d30` copies, `00406d50` appends).
pub fn fn_00456b10(e: &mut Engine, _this: Ptr, name: u32) {
    if name == 0 {
        return;
    }
    let world = current_world(e);
    if world == 0 {
        return;
    }
    e.with_stack(0x104, |e, buffer| {
        e.call(STRING_COPY, &args![buffer, 0x104u32, name]);
        e.call(STRING_APPEND, &args![buffer, 0x104u32, HKX_EXTENSION]);
        e.call(WORLD_SAVE_00C67430, &args![world, buffer]);
    });
}

// Translated from 00456be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` not read, plain `RET`; called by
/// `ToggleCollisionFunction`): flips the flag byte at `011c3c0d` ([`fn_00456c70`], then
/// [`fn_00456c80`] with 1 when it was 0, else 0), hands the old state (whether the new flag
/// is 0) to `00454380` on the current physics world when there is one, and to
/// [`fn_00456c90`].
pub fn fn_00456be0(e: &mut Engine, _this: Ptr) {
    let flag = u8::from(fn_00456c70(e) == 0);
    fn_00456c80(e, flag);
    let world = current_world(e);
    if world != 0 {
        e.call(WORLD_TOGGLE_00454380, &args![world, u32::from(flag == 0)]);
    }
    fn_00456c90(e, u8::from(flag == 0));
}

// Translated from 00456c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one byte word): stores the byte at `011c3c0d`.
pub fn fn_00456c80(e: &mut Engine, value: u8) {
    e.set_global(FLAG_011C3C0D, value);
}

// Translated from 00456c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one byte word; also called by `ToggleRagdollAnim`):
/// stores the byte at `00408d60`'s address for `01267c24`.
pub fn fn_00456c90(e: &mut Engine, value: u8) {
    let address = e.call(TEXT_ADDRESS, &args![RAGDOLL_BYTE_OWNER]).u32();
    e.mem.set_u8(address, value);
}

// Translated from 00456cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` = the `TES`, one byte word, `RET 4`; called by
/// `ToggleBordersFunction`): when the byte at `+0x60` differs from `value`, stores it and,
/// with a data handler and no interior cell, hands it to `004baca0` on the grid at `+8`.
pub fn fn_00456cb0(e: &mut Engine, this: Ptr, value: u8) {
    let base = this.addr();
    if e.mem.u8(base + 0x60) == value {
        return;
    }
    e.mem.set_u8(base + 0x60, value);
    if e.global::<u32>(DATA_HANDLER) == 0 {
        return;
    }
    if e.call(INTERIOR_CELL, &args![this]).u32() != 0 {
        return;
    }
    let grid = e.mem.u32(base + 8);
    e.call(0x004b_aca0, &args![grid, u32::from(value)]);
}

/// The node kinds whose node the loops over loaded cells hide or show (the compiler's
/// byte map at `00456f9c`/`00456fb0` sends 0 to 5, 7 and 8 to the body and 6 past it).
fn toggled_kind(kind: i32) -> bool {
    matches!(kind as u32, 0..=5 | 7 | 8)
}

/// [`fn_00456fc0`] for `cell` and `kind`; a node found gets `00450f90` with `hide`.
fn toggle_cell_node(e: &mut Engine, cell: u32, kind: i32, hide: u32) {
    let node = fn_00456fc0(e, Ptr::new(cell), kind as u32);
    if node != 0 {
        e.call(NODE_SET_HIDDEN_00450F90, &args![node, hide]);
    }
}

// Translated from 00456d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` = the `TES`, three stack words, `RET 0xc`; called by
/// `ToggleCellNode`): shows (`on`) or hides node `kind` of the loaded cells. First records
/// the state in the flag word at `011ca08c` ([`fn_00457000`] with `on == 0`). Kind 0xb is
/// the water: unless the interior cell answers true to `004518e0`, `00450f90` on
/// `004e7ff0`'s node with `on == 0`. Other kinds: for the interior cell and then (unless
/// `everything`) each of the setting at `011c3e38`'s count of cells in the array at `+0x38`,
/// a non-null cell with `0 <= kind <= 8` gets [`fn_00456fc0`]'s node hidden or shown and
/// `00557990(on == 0, kind)`. Then, without `everything`, each of the setting at
/// `011c3c90`'s count of cells in the array at `+0x3c`; with it, every cell of the grid
/// ([`fn_00457050`] on the `TES` singleton, both indices up to the setting at `011c63cc`):
/// a non-null cell with a kind of 0 to 5, 7 or 8 gets its node hidden or shown.
pub fn fn_00456d00(e: &mut Engine, this: Ptr, kind: i32, on: u8, everything: u8) {
    let hide = u32::from(on == 0);
    fn_00457000(e, kind as u8, u8::from(on == 0));
    let base = this.addr();
    if kind == 0xb {
        let tes: u32 = e.global(TES);
        if e.call(INTERIOR_CELL, &args![tes]).u32() != 0 {
            let tes: u32 = e.global(TES);
            let cell = e.call(INTERIOR_CELL, &args![tes]).u32();
            if e.call(CELL_TEST_004518E0, &args![cell]).u8() != 0 {
                return;
            }
        }
        let water = e.call(WATER_NODE_004E7FF0, &args![]).u32();
        e.call(NODE_SET_HIDDEN_00450F90, &args![water, hide]);
        return;
    }
    let count = if everything != 0 {
        0
    } else {
        let value = e.call(SETTING_VALUE, &args![0x011c_3e38u32]).u32();
        e.mem.i32(value)
    };
    let mut index: i32 = -1;
    while index < count {
        let cell = if index == -1 {
            e.call(INTERIOR_CELL, &args![this]).u32()
        } else {
            let array = e.mem.u32(base + 0x38);
            e.mem
                .u32(array.wrapping_add((index as u32).wrapping_mul(4)))
        };
        if cell != 0 && (0..=8).contains(&kind) {
            toggle_cell_node(e, cell, kind, hide);
            e.call(CELL_TOGGLE_00557990, &args![cell, hide, kind]);
        }
        index += 1;
    }
    if everything == 0 {
        let mut index: i32 = 0;
        loop {
            let value = e.call(SETTING_VALUE, &args![0x011c_3c90u32]).u32();
            if index >= e.mem.i32(value) {
                break;
            }
            let array = e.mem.u32(base + 0x3c);
            let cell = e
                .mem
                .u32(array.wrapping_add((index as u32).wrapping_mul(4)));
            if cell != 0 && toggled_kind(kind) {
                toggle_cell_node(e, cell, kind, hide);
            }
            index += 1;
        }
    } else {
        let mut x: u32 = 0;
        loop {
            let value = e.call(SETTING_VALUE, &args![0x011c_63ccu32]).u32();
            if x >= e.mem.u32(value) {
                break;
            }
            let mut y: u32 = 0;
            loop {
                let value = e.call(SETTING_VALUE, &args![0x011c_63ccu32]).u32();
                if y >= e.mem.u32(value) {
                    break;
                }
                let tes: u32 = e.global(TES);
                let slot = fn_00457050(e, Ptr::new(tes), x, y);
                let cell = e.mem.u32(slot);
                if cell != 0 && toggled_kind(kind) {
                    toggle_cell_node(e, cell, kind, hide);
                }
                y += 1;
            }
            x += 1;
        }
    }
}

// Translated from 00457000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words; called by [`fn_00456d00`]): sets
/// (`set` nonzero) or clears bit `kind` (`SHL` by its low five bits) of the word at
/// `011ca08c`.
pub fn fn_00457000(e: &mut Engine, kind: u8, set: u8) {
    let bit = 1u32 << (u32::from(kind) & 0x1f);
    let flags: u32 = e.global(NODE_FLAGS);
    let flags = if set != 0 { bit | flags } else { !bit & flags };
    e.set_global(NODE_FLAGS, flags);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0045_6a60, fn_00456a60(Ptr) -> u32),
        entry!(0x0045_6aa0, fn_00456aa0(Ptr) -> u32),
        entry!(0x0045_6b10, fn_00456b10(Ptr, u32)),
        entry!(0x0045_6be0, fn_00456be0(Ptr)),
        entry!(0x0045_6c80, fn_00456c80(u8)),
        entry!(0x0045_6c90, fn_00456c90(u8)),
        entry!(0x0045_6cb0, fn_00456cb0(Ptr, u8)),
        entry!(0x0045_6d00, fn_00456d00(Ptr, i32, u8, u8)),
        entry!(0x0045_7000, fn_00457000(u8, u8)),
        entry!(0x0045_6c70, fn_00456c70() -> u8),
        entry!(0x0045_6fc0, fn_00456fc0(Ptr, u32) -> u32),
        entry!(0x0045_7050, fn_00457050(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_reads_the_byte_global() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(0x011c_3c0c, 0xffff_33ffu32);
        assert_eq!(e.call(0x0045_6c70, &args![]).u32() & 0xff, 0x33);
    }

    #[test]
    fn cell_lookup_needs_the_cells_3d() {
        let mut e = Engine::new();
        e.register(CELL_GET_3D, |_, _| Ret::default());
        e.register(NODE_LOOKUP_43B4A0, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x0045_6fc0, &args![0x1000u32, 7u32]).u32(), 0);
        e.register(CELL_GET_3D, |_, _| 0x5000u32.into_ret());
        assert_eq!(e.call(0x0045_6fc0, &args![0x1000u32, 7u32]).u32(), 0x5007);
    }

    #[test]
    fn grid_get_goes_through_the_grid_at_8() {
        let mut e = Engine::new();
        e.register(GRID_CELL_ARRAY_GET, |_, a| (a[0] ^ a[1] ^ a[2]).into_ret());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x7000);
        e.call_log = Some(vec![]);
        let back = e.call(0x0045_7050, &args![this, 3u32, 5u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0x7000 ^ 3 ^ 5);
        let call = log.iter().find(|(a, _)| *a == GRID_CELL_ARRAY_GET).unwrap();
        assert_eq!(call.1, vec![0x7000, 3, 5]);
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Globals mapped; the `TES` at a heap block with its grid word at +8.
    fn engine(interior: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x8000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0126_7000, 0x1000);
        let tes = e.mem.alloc(0x80);
        e.mem.set_u32(tes + 8, 0x0055_0000);
        e.set_global(TES, tes);
        e.register_double(INTERIOR_CELL, move |_, _| interior.into_ret());
        e.register(TEXT_ADDRESS, |_, a| (a[0] + 4).into_ret());
        e.call_log = Some(vec![]);
        (e, tes)
    }

    #[test]
    fn fn_00456a60_asks_the_interior_or_the_grid() {
        let (mut e, tes) = engine(0x4000);
        e.register(0x0055_4910, |_, _| 7u32.into_ret());
        e.register(0x004b_ad40, |_, _| 8u32.into_ret());
        assert_eq!(e.call(0x0045_6a60, &args![tes]).u32(), 7);
        let (mut e, tes) = engine(0);
        e.register(0x004b_ad40, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x0045_6a60, &args![tes]).u32(), 0x0055_0000);
    }

    #[test]
    fn fn_00456aa0_toggles_the_byte_and_hands_it_on() {
        let (mut e, tes) = engine(0);
        e.register(0x004b_ae20, |_, _| Ret::default());
        e.call(0x0045_6aa0, &args![tes]);
        assert_eq!(e.mem.u8(TOGGLE_BYTE_OWNER + 4), 1);
        let (mut e2, tes2) = engine(0x4000);
        e2.register(0x0055_49d0, |_, _| Ret::default());
        e2.mem.set_u8(TOGGLE_BYTE_OWNER + 4, 1);
        e2.call(0x0045_6aa0, &args![tes2]);
        assert_eq!(e2.mem.u8(TOGGLE_BYTE_OWNER + 4), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x004b_ae20), vec![vec![0x0055_0000, 1]]);
        let log = e2.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0055_49d0), vec![vec![0x4000, 0]]);
    }

    #[test]
    fn fn_00456b10_saves_the_world_under_the_name() {
        let (mut e, tes) = engine(0x4000);
        e.register(CELL_WORLD, |_, _| 0x6000u32.into_ret());
        e.register(STRING_COPY, |_, _| Ret::default());
        e.register(STRING_APPEND, |_, _| Ret::default());
        e.register(WORLD_SAVE_00C67430, |_, _| Ret::default());
        e.call(0x0045_6b10, &args![tes, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        let copy = calls(&log, STRING_COPY);
        assert_eq!(copy.len(), 1);
        let buffer = copy[0][0];
        assert_eq!(copy[0][1..], [0x104, 0x9000]);
        assert_eq!(
            calls(&log, STRING_APPEND),
            vec![vec![buffer, 0x104, HKX_EXTENSION]]
        );
        assert_eq!(calls(&log, WORLD_SAVE_00C67430), vec![vec![0x6000, buffer]]);
        // No name, or no world: nothing.
        let (mut e, tes) = engine(0);
        e.register(EXTERIOR_WORLD, |_, _| 0u32.into_ret());
        e.call(0x0045_6b10, &args![tes, 0u32]);
        e.call(0x0045_6b10, &args![tes, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, EXTERIOR_WORLD).len(), 1);
        assert!(calls(&log, STRING_COPY).is_empty());
    }

    #[test]
    fn fn_00456be0_flips_the_collision_flag() {
        let (mut e, tes) = engine(0);
        e.register(EXTERIOR_WORLD, |_, _| 0x6100u32.into_ret());
        e.register(WORLD_TOGGLE_00454380, |_, _| Ret::default());
        e.call(0x0045_6be0, &args![tes]);
        assert_eq!(e.mem.u8(FLAG_011C3C0D), 1);
        assert_eq!(e.mem.u8(RAGDOLL_BYTE_OWNER + 4), 0);
        e.call(0x0045_6be0, &args![tes]);
        assert_eq!(e.mem.u8(FLAG_011C3C0D), 0);
        assert_eq!(e.mem.u8(RAGDOLL_BYTE_OWNER + 4), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, WORLD_TOGGLE_00454380),
            vec![vec![0x6100, 0], vec![0x6100, 1]]
        );
    }

    #[test]
    fn fn_00456c80_and_00456c90_store_their_bytes() {
        let (mut e, _) = engine(0);
        e.call(0x0045_6c80, &args![0x1ffu32]);
        assert_eq!(e.mem.u8(FLAG_011C3C0D), 0xff);
        e.call(0x0045_6c90, &args![0x102u32]);
        assert_eq!(e.mem.u8(RAGDOLL_BYTE_OWNER + 4), 2);
    }

    #[test]
    fn fn_00456cb0_stores_a_changed_border_flag() {
        let (mut e, tes) = engine(0);
        e.register(0x004b_aca0, |_, _| Ret::default());
        // No data handler: stored only.
        e.call(0x0045_6cb0, &args![tes, 1u32]);
        assert_eq!(e.mem.u8(tes + 0x60), 1);
        e.set_global(DATA_HANDLER, 0x1234u32);
        e.call(0x0045_6cb0, &args![tes, 1u32]);
        e.call(0x0045_6cb0, &args![tes, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x004b_aca0), vec![vec![0x0055_0000, 0]]);
        let (mut e, tes) = engine(0x4000);
        e.register(0x004b_aca0, |_, _| Ret::default());
        e.set_global(DATA_HANDLER, 0x1234u32);
        e.call(0x0045_6cb0, &args![tes, 1u32]);
        assert!(calls(&e.call_log.take().unwrap(), 0x004b_aca0).is_empty());
    }

    /// For 00456d00: settings at 011c3e38, 011c3c90 and 011c63cc hold their counts at +4.
    fn toggle_engine(interior: u32, counts: [i32; 3]) -> (Engine, u32) {
        let (mut e, tes) = engine(interior);
        for (setting, count) in [0x011c_3e38u32, 0x011c_3c90, 0x011c_63cc]
            .iter()
            .zip(counts)
        {
            e.mem.set_u32(setting + 4, count as u32);
        }
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(CELL_GET_3D, |_, a| (a[0] + 0x100).into_ret());
        e.register(NODE_LOOKUP_43B4A0, |_, a| (a[0] + 0x10).into_ret());
        e.register(NODE_SET_HIDDEN_00450F90, |_, _| Ret::default());
        e.register(CELL_TOGGLE_00557990, |_, _| Ret::default());
        e.register(WATER_NODE_004E7FF0, |_, _| 0x7700u32.into_ret());
        e.register(CELL_TEST_004518E0, |_, _| 0u32.into_ret());
        let cells = e.mem.alloc(8);
        e.mem.set_u32(cells, 0x1000);
        e.mem.set_u32(cells + 4, 0);
        e.mem.set_u32(tes + 0x38, cells);
        let more = e.mem.alloc(4);
        e.mem.set_u32(more, 0x2000);
        e.mem.set_u32(tes + 0x3c, more);
        (e, tes)
    }

    #[test]
    fn fn_00456d00_hides_the_water_node() {
        let (mut e, tes) = toggle_engine(0, [0; 3]);
        e.call(0x0045_6d00, &args![tes, 0xbu32, 0u32, 0u32]);
        assert_eq!(e.global::<u32>(NODE_FLAGS), 1 << 0xb);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, NODE_SET_HIDDEN_00450F90), vec![vec![0x7700, 1]]);
        // An interior that answers true keeps it.
        let (mut e, tes) = toggle_engine(0x3000, [0; 3]);
        e.register(CELL_TEST_004518E0, |_, _| 1u32.into_ret());
        e.call(0x0045_6d00, &args![tes, 0xbu32, 1u32, 0u32]);
        assert!(calls(&e.call_log.take().unwrap(), NODE_SET_HIDDEN_00450F90).is_empty());
    }

    #[test]
    fn fn_00456d00_toggles_the_loaded_cells() {
        let (mut e, tes) = toggle_engine(0x3000, [2, 1, 0]);
        e.set_global(NODE_FLAGS, 0xffu32);
        e.call(0x0045_6d00, &args![tes, 3u32, 1u32, 0u32]);
        assert_eq!(e.global::<u32>(NODE_FLAGS), 0xf7);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, CELL_TOGGLE_00557990),
            vec![vec![0x3000, 0, 3], vec![0x1000, 0, 3]]
        );
        assert_eq!(
            calls(&log, NODE_SET_HIDDEN_00450F90),
            vec![vec![0x3110, 0], vec![0x1110, 0], vec![0x2110, 0]]
        );
        // Kind 6 skips the body of the second loop, kind 9 that of the first.
        let (mut e, tes) = toggle_engine(0, [0, 1, 0]);
        e.call(0x0045_6d00, &args![tes, 6u32, 1u32, 0u32]);
        e.call(0x0045_6d00, &args![tes, 9u32, 1u32, 0u32]);
        assert!(calls(&e.call_log.take().unwrap(), NODE_SET_HIDDEN_00450F90).is_empty());
    }

    #[test]
    fn fn_00456d00_walks_the_whole_grid_for_everything() {
        let (mut e, tes) = toggle_engine(0, [5, 5, 2]);
        e.register(GRID_CELL_ARRAY_GET, |e, a| {
            let slot = e.mem.alloc(4);
            let cell = if a[1] == 1 && a[2] == 0 { 0x5000u32 } else { 0 };
            e.mem.set_u32(slot, cell);
            slot.into_ret()
        });
        e.call(0x0045_6d00, &args![tes, 8u32, 0u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GRID_CELL_ARRAY_GET).len(), 4);
        assert_eq!(calls(&log, NODE_SET_HIDDEN_00450F90), vec![vec![0x5110, 1]]);
        assert!(calls(&log, CELL_TOGGLE_00557990).is_empty());
    }

    #[test]
    fn fn_00457000_sets_and_clears_bits() {
        let (mut e, _) = engine(0);
        e.call(0x0045_7000, &args![0x23u32, 1u32]);
        assert_eq!(e.global::<u32>(NODE_FLAGS), 8);
        e.call(0x0045_7000, &args![3u32, 0x100u32]);
        assert_eq!(e.global::<u32>(NODE_FLAGS), 0);
    }
}
