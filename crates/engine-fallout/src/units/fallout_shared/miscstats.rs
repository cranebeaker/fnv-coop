//! `fallout shared/miscstats.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004d5dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004d5c20` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004d5dd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004d_5c20, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The table of 0x2b `MiscStat*` pointers (`.data`); its first slot is the "built" test.
const MISC_STAT_TABLE: u32 = 0x011c_6d50;
/// The table of the 0x2b name pointers handed to the constructor (`.data`).
const MISC_STAT_NAMES: u32 = 0x0118_9280;
/// Number of entries in both tables.
const MISC_STAT_COUNT: u32 = 0x2b;
/// `operator new` (`00401000`, one word).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `MiscStat::MiscStat` (Xbox PDB, `004d5b60`): `this` plus name, index and a third word
/// (`RET 0xc`).
const MISC_STAT_CONSTRUCTOR: u32 = 0x004d_5b60;

// Translated from 004d5c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d5c80 - the compiler's exception frame (FS:[0] chain, state
// bookkeeping around the allocation) is not translated.
/// Unnamed in the engine map (unit `miscstats.cpp`): builds the table of `MiscStat` objects
/// once. If the first table slot is empty, for each of the 0x2b indices it allocates 8 bytes
/// and, when the allocation succeeded, constructs a `MiscStat` in it with the name pointer
/// at `MISC_STAT_NAMES[index]`, the index and 0 (a failed allocation stores null), and stores
/// the result in the table. Returns `this`, which it does not otherwise read.
pub fn fn_004d5c80(e: &mut Engine, this: Ptr) -> Ptr {
    if e.global::<u32>(MISC_STAT_TABLE) != 0 {
        return this;
    }
    for index in 0..MISC_STAT_COUNT {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let stat = if block != 0 {
            let name = e.global::<u32>(MISC_STAT_NAMES + index * 4);
            e.call(MISC_STAT_CONSTRUCTOR, &args![block, name, index, 0u32])
                .u32()
        } else {
            0
        };
        e.set_global(MISC_STAT_TABLE + index * 4, stat);
    }
    this
}

layout! {
    /// `MiscStat` (Xbox PDB), 8 bytes; the PC code uses the same offsets.
    pub struct MiscStat: 0x08 {
        /// `pDisplayName` (Xbox PDB): `SettingT<GameSettingCollection>*`.
        0x00 pDisplayName: u32,
        /// `iVal` (Xbox PDB).
        0x04 iVal: i32,
    }
}

/// `"sMiscStat%03d"`: the name of a stat's display-name setting.
const SETTING_NAME_FORMAT: u32 = 0x0102_0d58;
/// The formatted print `00406d00(buffer, size, format, ...)` (cdecl).
const PRINT: u32 = 0x0040_6d00;
/// The 0xc-byte setting's constructor (`0040c150(name, value)`,
/// `thiscall`).
const SETTING_CONSTRUCT: u32 = 0x0040_c150;
/// Fills bytes (`00403d30(dest, value, count)`).
const FILL_BYTES: u32 = 0x0040_3d30;
/// The event `ModVal` sends (`005f5950`, cdecl, seven words: 0xb, the
/// amount, 0, 0, the stat id as a `u16`, 0, 0).
const SEND_EVENT: u32 = 0x005f_5950;
/// `Tile::GetMenuByClass` (engine map, cdecl) and `Tile::GetMenu` (engine
/// map, `thiscall`).
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// The stats menu's update for a stat (`007ddad0`, `statsmenu.cpp` by the
/// map; `thiscall(menu, id)`).
const STATS_MENU_UPDATE: u32 = 0x007d_dad0;
/// The log call (`005b5e40`, cdecl with a format; does nothing in this
/// build) and `ModVal`'s message ("MENUS: Unable to modify miscellaneous
/// st...").
const LOG: u32 = 0x005b_5e40;
const MOD_VAL_FAILED: u32 = 0x0102_0d68;
/// The save buffer's write (`00865e50(data, size, 0)`, `thiscall` on the
/// buffer; `bgssavegamebuffer.cpp` by the map).
const SAVE_WRITE: u32 = 0x0086_5e50;

/// The stat at `index` of the table (any `i32`: the code indexes without a
/// lower bound).
fn stat_slot(index: i32) -> u32 {
    MISC_STAT_TABLE.wrapping_add((index as u32).wrapping_mul(4))
}

// Translated from 004d5b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiscStat::MiscStat(display_name, index, ival)` (Xbox PDB): keeps `ival` in
/// `iVal`, prints the setting name `"sMiscStat%03d"` of `index` into a new
/// 0xd-byte buffer and makes the display-name setting (a new 0xc-byte
/// setting, `0040c150(name, display_name)`; null when the allocation
/// fails).
/// Returns `this`.
// Translation incomplete: 004d5b60 - the FS:[0] exception frame (frees the setting's block if its constructor throws) is not translated
pub fn misc_stat_construct(
    e: &mut Engine,
    this: Ptr<MiscStat>,
    display_name: u32,
    index: u32,
    ival: i32,
) -> Ptr<MiscStat> {
    e.set(this, MiscStat::iVal, ival);
    let name = e.call(OPERATOR_NEW, &args![0xdu32]).u32();
    e.call(PRINT, &args![name, 0xdu32, SETTING_NAME_FORMAT, index]);
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let setting = if block == 0 {
        0
    } else {
        e.call(SETTING_CONSTRUCT, &args![block, name, display_name])
            .u32()
    };
    e.set(this, MiscStat::pDisplayName, setting);
    this
}

// Translated from 004d5c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiscStat`'s destructor body: deletes the display-name setting through
/// its vtable slot `+0` (scalar deleting destructor, flags 1) when there
/// is one.
pub fn fn_004d5c20(e: &mut Engine, this: Ptr<MiscStat>) {
    let setting = e.get(this, MiscStat::pDisplayName);
    if setting != 0 {
        e.vcall(setting, 0, &args![1u32]);
    }
}

// Translated from 004d5c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiscStatManager::Increment` (Xbox PDB, cdecl): `ModVal(id, 1)`.
pub fn misc_stat_manager_increment(e: &mut Engine, id: i32) {
    misc_stat_manager_mod_val(e, id, 1);
}

// Translated from 004d5d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `thiscall` member that runs `004d5d50` (`this` is not read).
pub fn fn_004d5d40(e: &mut Engine, _this: Ptr) {
    fn_004d5d50(e);
}

// Translated from 004d5d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every `MiscStat` of the table (`004d5dd0` with flags 1) and
/// clears the table's 0xac bytes.
pub fn fn_004d5d50(e: &mut Engine) {
    for index in 0..MISC_STAT_COUNT as i32 {
        let stat = e.mem.u32(stat_slot(index));
        if stat != 0 {
            fn_004d5dd0(e, Ptr::new(stat), 1);
        }
    }
    e.call(FILL_BYTES, &args![MISC_STAT_TABLE, 0u32, 0xacu32]);
}

// Translated from 004d5e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu class `ModVal` updates: 0x3eb.
pub fn fn_004d5e00(_e: &mut Engine) -> u32 {
    0x3eb
}

// Translated from 004d5e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiscStatManager::ModVal(id, amount)` (Xbox PDB, cdecl): nothing for a
/// zero amount; for an id below 0x2b (signed) with a stat, adds the amount
/// to its `iVal`, sends event 0xb (`005f5950`) and, when the menu of class
/// 0x3eb exists, tells it the stat (`007ddad0`); otherwise logs
/// "MENUS: Unable to modify ..." (a no-op in this build).
pub fn misc_stat_manager_mod_val(e: &mut Engine, id: i32, amount: i32) {
    if amount == 0 {
        return;
    }
    if id < MISC_STAT_COUNT as i32 && e.mem.u32(stat_slot(id)) != 0 {
        let stat: Ptr<MiscStat> = Ptr::new(e.mem.u32(stat_slot(id)));
        let value = e.get(stat, MiscStat::iVal).wrapping_add(amount);
        let stat: Ptr<MiscStat> = Ptr::new(e.mem.u32(stat_slot(id)));
        e.set(stat, MiscStat::iVal, value);
        e.call(
            SEND_EVENT,
            &args![0xbu32, amount, 0u32, 0u32, u32::from(id as u16), 0u32, 0u32],
        );
        let class = fn_004d5e00(e);
        let tile = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
        let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
        if menu != 0 {
            e.call(STATS_MENU_UPDATE, &args![menu, id]);
        }
    } else {
        e.call(LOG, &args![MOD_VAL_FAILED, id, amount]);
    }
}

// Translated from 004d5fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the stats (cdecl): writes the count 0x2b, then each stat's `iVal`
/// (0 for a missing stat), each as 4 bytes through the save buffer's write
/// (`00865e50`).
pub fn fn_004d5fa0(e: &mut Engine, buffer: Ptr) {
    e.with_stack(8, |e, locals| {
        let count = locals.addr();
        let value = locals.addr() + 4;
        e.mem.set_u32(count, MISC_STAT_COUNT);
        e.call(SAVE_WRITE, &args![buffer, count, 4u32, 0u32]);
        let mut index: u32 = 0;
        while index < e.mem.u32(count) {
            let stat = e.mem.u32(stat_slot(index as i32));
            let v = if stat != 0 {
                e.get(Ptr::<MiscStat>::new(stat), MiscStat::iVal) as u32
            } else {
                0
            };
            e.mem.set_u32(value, v);
            e.call(SAVE_WRITE, &args![buffer, value, 4u32, 0u32]);
            index += 1;
        }
    });
}

/// `00404dc0` (`cdecl`, two strings): 0 when they match.
const STRING_COMPARE: u32 = 0x0040_4dc0;
/// `007dc0a0` (`cdecl`, the address of a stat index; unit `statsmenu.cpp`
/// by the engine map): steps the index.
const NEXT_STAT: u32 = 0x007d_c0a0;
/// `"MENUS: Miscellaneous stat %s could not be found.  Returning MS_COUNT."`.
const STAT_NOT_FOUND: u32 = 0x0102_0da8;
/// The load buffer's read (`00864980`, `thiscall`: destination, size;
/// `bgsloadgamebuffer.cpp` by the engine map).
const LOAD_READ: u32 = 0x0086_4980;

// Translated from 004d5eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiscStatManager::GetIDByName` (Xbox PDB name in the engine map; `cdecl`,
/// one word): the index of the stat whose name (`MISC_STAT_NAMES`, null
/// entries skipped) matches `name` (`00404dc0`), the index stepped by
/// `007dc0a0` through its stack address; past the last (a signed compare
/// with 0x2b) the not-found message is logged (`005b5e40`) and 0x2b
/// (`MS_COUNT`) answered.
pub fn misc_stat_manager_get_id_by_name(e: &mut Engine, name: u32) -> i32 {
    e.with_stack(4, |e, index_at| loop {
        let index = e.mem.i32(index_at.addr());
        if index >= MISC_STAT_COUNT as i32 {
            e.call(LOG, &args![STAT_NOT_FOUND, name]);
            return MISC_STAT_COUNT as i32;
        }
        let entry = MISC_STAT_NAMES.wrapping_add((index as u32).wrapping_mul(4));
        let stat_name = e.mem.u32(entry);
        if stat_name != 0 && e.call(STRING_COMPARE, &args![stat_name, name]).u32() == 0 {
            return index;
        }
        e.call(NEXT_STAT, &args![index_at]);
    })
}

// Translated from 004d5f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets every stat's `iVal` from the 0x2b words at `values` (`cdecl`, one
/// word; the engine map gives it no name). The table's entries are not
/// checked for null.
pub fn fn_004d5f20(e: &mut Engine, values: u32) {
    for index in 0..MISC_STAT_COUNT as i32 {
        let stat = e.mem.u32(stat_slot(index));
        let value = e.mem.u32(values.wrapping_add(index as u32 * 4));
        e.mem.set_u32(stat.wrapping_add(4), value);
    }
}

// Translated from 004d5f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies every stat's `iVal` into the 0x2b words at `values` (`cdecl`,
/// one word; the engine map gives it no name). The table's entries are not
/// checked for null.
pub fn fn_004d5f60(e: &mut Engine, values: u32) {
    for index in 0..MISC_STAT_COUNT as i32 {
        let stat = e.mem.u32(stat_slot(index));
        let value = e.mem.u32(stat.wrapping_add(4));
        e.mem.set_u32(values.wrapping_add(index as u32 * 4), value);
    }
}

// Translated from 004d6020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MiscStatManager::LoadGame` (Xbox PDB name in the engine map; `cdecl`,
/// the load buffer): the count is 0x15 unless the buffer's version (its
/// virtual slot 0, a byte) is 11 or more, when it is read (`00864980`, 4
/// bytes). With a count up to 0x1000000, that many words are read (each
/// into a word cleared first); the first 0x2b go into the stats' `iVal`s
/// whose table entry is set.
pub fn misc_stat_manager_load_game(e: &mut Engine, buffer: u32) {
    e.with_stack(8, |e, locals| {
        let count_at = locals.addr();
        let value_at = locals.addr() + 4;
        e.mem.set_u32(count_at, 0x15);
        let version = e.vcall(buffer, 0, &args![]).u8();
        if version >= 0xb {
            e.call(LOAD_READ, &args![buffer, count_at, 4u32]);
        }
        if e.mem.u32(count_at) > 0x0100_0000 {
            return;
        }
        let mut index = 0u32;
        while index < e.mem.u32(count_at) {
            e.mem.set_u32(value_at, 0);
            e.call(LOAD_READ, &args![buffer, value_at, 4u32]);
            if index < MISC_STAT_COUNT {
                let stat = e.mem.u32(stat_slot(index as i32));
                if stat != 0 {
                    let value = e.mem.u32(value_at);
                    e.mem.set_u32(stat + 4, value);
                }
            }
            index += 1;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004d_5eb0, misc_stat_manager_get_id_by_name(u32) -> i32),
        entry!(0x004d_5f20, fn_004d5f20(u32)),
        entry!(0x004d_5f60, fn_004d5f60(u32)),
        entry!(0x004d_6020, misc_stat_manager_load_game(u32)),
        entry!(0x004d_5dd0, fn_004d5dd0(Ptr, u32) -> Ptr),
        entry!(0x004d_5c80, fn_004d5c80(Ptr) -> Ptr),
        entry!(
            0x004d5b60,
            misc_stat_construct(Ptr<MiscStat>, u32, u32, i32) -> Ptr<MiscStat>
        ),
        entry!(0x004d5c20, fn_004d5c20(Ptr<MiscStat>)),
        entry!(0x004d5c60, misc_stat_manager_increment(i32)),
        entry!(0x004d5d40, fn_004d5d40(Ptr)),
        entry!(0x004d5d50, fn_004d5d50()),
        entry!(0x004d5e00, fn_004d5e00() -> u32),
        entry!(0x004d5e10, misc_stat_manager_mod_val(i32, i32)),
        entry!(0x004d5fa0, fn_004d5fa0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004d5dd0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004d_5c20, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004d_5dd0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004d_5dd0)
                .collect();
            let mut expected = vec![(0x004d_5c20, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    fn tables(e: &mut Engine) {
        e.map(MISC_STAT_TABLE & !0xfff, 0x1000);
        e.map(MISC_STAT_NAMES & !0xfff, 0x1000);
        for i in 0..MISC_STAT_COUNT {
            e.set_global(MISC_STAT_NAMES + i * 4, 0x5000 + i);
        }
    }

    #[test]
    fn builds_every_entry_when_the_first_slot_is_empty() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OPERATOR_NEW, |e, _| Ret {
            eax: e.mem.alloc(8),
            ..Ret::default()
        });
        e.register(MISC_STAT_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_5c80, &args![0x1234u32]).u32(), 0x1234);
        let built = calls_to(&mut e, MISC_STAT_CONSTRUCTOR);
        assert_eq!(built.len(), 0x2b);
        for (index, words) in built.iter().enumerate() {
            assert_eq!(words[1], 0x5000 + index as u32);
            assert_eq!(words[2], index as u32);
            assert_eq!(words[3], 0);
            assert_eq!(
                e.global::<u32>(MISC_STAT_TABLE + index as u32 * 4),
                words[0]
            );
        }
        assert_eq!(calls_to(&mut e, OPERATOR_NEW).len(), 0x2b);
        assert_eq!(calls_to(&mut e, OPERATOR_NEW)[0], vec![8]);
    }

    #[test]
    fn stores_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(MISC_STAT_CONSTRUCTOR, |_, _| panic!("not constructed"));
        e.set_global(MISC_STAT_TABLE + 4, 0x99u32);
        e.call(0x004d_5c80, &args![0u32]);
        for i in 0..MISC_STAT_COUNT {
            assert_eq!(e.global::<u32>(MISC_STAT_TABLE + i * 4), 0);
        }
    }

    #[test]
    fn does_nothing_when_the_first_slot_is_filled() {
        let mut e = Engine::new();
        tables(&mut e);
        e.set_global(MISC_STAT_TABLE, 0x77u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_5c80, &args![0x42u32]).u32(), 0x42);
        assert_eq!(
            e.call_log.take().unwrap().len(),
            1,
            "only the top-level call"
        );
        assert_eq!(e.global::<u32>(MISC_STAT_TABLE), 0x77);
        assert_eq!(e.global::<u32>(MISC_STAT_TABLE + 4), 0);
    }
}

#[cfg(test)]
mod misc_stat_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> Log {
        e.call_log = Some(vec![]);
        e.call(addr, words);
        e.call_log.take().unwrap()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(MISC_STAT_TABLE & !0xfff, 0x1000);
        e
    }

    fn stat(e: &mut Engine, index: u32, value: i32) -> Ptr<MiscStat> {
        let s: Ptr<MiscStat> = e.new_object();
        e.set(s, MiscStat::iVal, value);
        e.set_global(MISC_STAT_TABLE + index * 4, s.addr());
        s
    }

    #[test]
    fn constructor_makes_the_display_name_setting() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PRINT, no_op);
        e.register(SETTING_CONSTRUCT, |_, a| a[0].into_ret());
        let this: Ptr<MiscStat> = e.new_object();
        let log = run(&mut e, 0x004d_5b60, &args![this, 0x5000u32, 7u32, -3i32]);
        assert_eq!(e.get(this, MiscStat::iVal), -3);
        let news = calls(&log, OPERATOR_NEW);
        assert_eq!(news, vec![vec![0xd], vec![0xc]]);
        let print = &calls(&log, PRINT)[0];
        assert_eq!(print[1..], [0xd, SETTING_NAME_FORMAT, 7]);
        let setting = &calls(&log, SETTING_CONSTRUCT)[0];
        assert_eq!(setting[1..], [print[0], 0x5000]);
        assert_eq!(e.get(this, MiscStat::pDisplayName), setting[0]);
        // A failed setting allocation leaves null.
        e.register(OPERATOR_NEW, |e, a| {
            (if a[0] == 0xc { 0 } else { e.mem.alloc(a[0]) }).into_ret()
        });
        run(&mut e, 0x004d_5b60, &args![this, 0x5000u32, 7u32, 0i32]);
        assert_eq!(e.get(this, MiscStat::pDisplayName), 0);
    }

    #[test]
    fn destructor_body_deletes_the_setting() {
        let mut e = engine();
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, 0x0f50_0000);
        e.register(0x0f50_0000, no_op);
        let setting = e.mem.alloc(0xc);
        e.mem.set_u32(setting, vtable);
        let this: Ptr<MiscStat> = e.new_object();
        e.set(this, MiscStat::pDisplayName, setting);
        let log = run(&mut e, 0x004d_5c20, &args![this]);
        assert_eq!(calls(&log, 0x0f50_0000), vec![vec![setting, 1]]);
        e.set(this, MiscStat::pDisplayName, 0);
        let log = run(&mut e, 0x004d_5c20, &args![this]);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn mod_val_adds_sends_and_updates_the_menu() {
        let mut e = engine();
        for addr in [SEND_EVENT, STATS_MENU_UPDATE, LOG] {
            e.register(addr, no_op);
        }
        e.register(GET_MENU_BY_CLASS, |_, a| (a[0] + 0x1_0000).into_ret());
        e.register(TILE_GET_MENU, |_, a| a[0].into_ret());
        let s = stat(&mut e, 5, 10);
        let log = run(&mut e, 0x004d_5e10, &args![5i32, -4i32]);
        assert_eq!(e.get(s, MiscStat::iVal), 6);
        assert_eq!(
            calls(&log, SEND_EVENT),
            vec![vec![0xb, (-4i32) as u32, 0, 0, 5, 0, 0]]
        );
        assert_eq!(calls(&log, GET_MENU_BY_CLASS), vec![vec![0x3eb]]);
        assert_eq!(calls(&log, STATS_MENU_UPDATE), vec![vec![0x1_03eb, 5]]);
        // No menu: no update.
        e.register(TILE_GET_MENU, |_, _| 0u32.into_ret());
        let log = run(&mut e, 0x004d_5c60, &args![5i32]);
        assert_eq!(e.get(s, MiscStat::iVal), 7);
        assert!(calls(&log, STATS_MENU_UPDATE).is_empty());
        // Zero amount: nothing.
        let log = run(&mut e, 0x004d_5e10, &args![5i32, 0i32]);
        assert_eq!(log.len(), 1);
        // Out of range or missing stat: logged.
        let log = run(&mut e, 0x004d_5e10, &args![0x2bi32, 1i32]);
        assert_eq!(calls(&log, LOG), vec![vec![MOD_VAL_FAILED, 0x2b, 1]]);
        let log = run(&mut e, 0x004d_5e10, &args![6i32, 1i32]);
        assert_eq!(calls(&log, LOG).len(), 1);
    }

    #[test]
    fn mod_val_reads_below_the_table_for_a_negative_id() {
        let mut e = engine();
        e.register(LOG, no_op);
        // The word before the table is 0 here: logged, as the game would.
        let log = run(&mut e, 0x004d_5e10, &args![-1i32, 1i32]);
        assert_eq!(calls(&log, LOG).len(), 1);
    }

    #[test]
    fn increment_and_menu_class() {
        let mut e = engine();
        e.register(LOG, no_op);
        let log = run(&mut e, 0x004d_5c60, &args![3i32]);
        assert_eq!(calls(&log, LOG), vec![vec![MOD_VAL_FAILED, 3, 1]]);
        assert_eq!(e.call(0x004d_5e00, &[]).u32(), 0x3eb);
    }

    #[test]
    fn clear_deletes_every_stat_and_zeroes_the_table() {
        let mut e = engine();
        e.register(0x004d_5c20, no_op);
        e.register(0x0040_1030, no_op);
        e.register(FILL_BYTES, no_op);
        let first = stat(&mut e, 0, 1);
        let last = stat(&mut e, 0x2a, 1);
        for addr in [0x004d_5d40u32, 0x004d_5d50] {
            let log = run(&mut e, addr, &args![0u32]);
            assert_eq!(
                calls(&log, 0x0040_1030),
                vec![vec![first.addr()], vec![last.addr()]]
            );
            assert_eq!(
                calls(&log, FILL_BYTES),
                vec![vec![MISC_STAT_TABLE, 0, 0xac]]
            );
        }
    }

    #[test]
    fn save_writes_the_count_and_every_value() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let mut e = engine();
        let written = Rc::new(RefCell::new(Vec::new()));
        let sink = written.clone();
        e.register_double(SAVE_WRITE, move |e, a| {
            assert_eq!((a[0], a[2], a[3]), (0x600, 4, 0));
            sink.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        stat(&mut e, 0, 9);
        stat(&mut e, 2, -1);
        e.call(0x004d_5fa0, &args![0x600u32]);
        let written = written.borrow();
        assert_eq!(written.len(), 1 + 0x2b);
        assert_eq!(written[..4], [0x2b, 9, 0, 0xffff_ffff]);
        assert!(written[4..].iter().all(|v| *v == 0));
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The two tables mapped; stats 0..0x2b allocated with `iVal` = index.
    fn with_stats() -> (Engine, Vec<u32>) {
        let mut e = Engine::new();
        e.mem.map(MISC_STAT_NAMES, 0x100);
        e.mem.map(MISC_STAT_TABLE, 0x100);
        let mut stats = vec![];
        for index in 0..MISC_STAT_COUNT {
            let stat = e.mem.alloc(8);
            e.mem.set_u32(stat + 4, index);
            e.mem.set_u32(MISC_STAT_TABLE + 4 * index, stat);
            stats.push(stat);
        }
        (e, stats)
    }

    #[test]
    fn get_id_by_name_finds_or_logs() {
        let mut e = Engine::new();
        e.mem.map(MISC_STAT_NAMES, 0x100);
        // Names: entry 3 is "x" (0x5003), entry 1 is null, the rest 0x5000+i.
        for index in 0..MISC_STAT_COUNT {
            let name = if index == 1 { 0 } else { 0x5000 + index };
            e.mem.set_u32(MISC_STAT_NAMES + 4 * index, name);
        }
        e.register(STRING_COMPARE, |_, a| word(u32::from(a[0] != a[1])));
        e.register(NEXT_STAT, |e, a| {
            let i = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], i + 1);
            Ret::default()
        });
        e.register(LOG, |_, _| Ret::default());
        assert_eq!(e.call(0x004d_5eb0, &args![0x5003u32]).i32(), 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_5eb0, &args![0x9999u32]).i32(), 0x2b);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(LOG, vec![STAT_NOT_FOUND, 0x9999]));
        // The null name is skipped without a compare.
        assert_eq!(log.iter().filter(|c| c.0 == STRING_COMPARE).count(), 0x2a);
    }

    #[test]
    fn values_go_in_and_out() {
        let (mut e, stats) = with_stats();
        let values = e.mem.alloc(4 * MISC_STAT_COUNT);
        e.call(0x004d_5f60, &args![values]);
        assert_eq!(e.mem.u32(values + 4 * 0x2a), 0x2a);
        for index in 0..MISC_STAT_COUNT {
            e.mem.set_u32(values + 4 * index, 100 + index);
        }
        e.call(0x004d_5f20, &args![values]);
        assert_eq!(e.mem.u32(stats[0] + 4), 100);
        assert_eq!(e.mem.u32(stats[0x2a] + 4), 100 + 0x2a);
    }

    #[test]
    fn load_game_reads_the_count_by_version() {
        let (mut e, stats) = with_stats();
        e.mem.set_u32(MISC_STAT_TABLE + 4 * 2, 0);
        // The buffer: vtable slot 0 answers the version at +4; reads hand out
        // 0x1000 + n for the n-th read, the first being the count at +8.
        e.put_vtable(0x0f00_1000, &[0x0f00_2000]);
        e.register(0x0f00_2000, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(LOAD_READ, |e, a| {
            let n = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[0] + 0xc, n + 1);
            let value = if n == 0 {
                e.mem.u32(a[0] + 8)
            } else {
                0x1000 + n
            };
            e.mem.set_u32(a[1], value);
            Ret::default()
        });
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer, 0x0f00_1000);
        // Version 10: 0x15 values, no count read.
        e.mem.set_u32(buffer + 4, 10);
        e.mem.set_u32(buffer + 0xc, 1);
        e.call(0x004d_6020, &args![buffer]);
        assert_eq!(e.mem.u32(stats[0] + 4), 0x1001);
        assert_eq!(e.mem.u32(stats[0x14] + 4), 0x1015);
        assert_eq!(e.mem.u32(stats[0x15] + 4), 0x15);
        assert_eq!(e.mem.u32(stats[2] + 4), 2);
        // Version 11: count 0x30 read first; reads past 0x2b are dropped.
        e.mem.set_u32(buffer + 4, 11);
        e.mem.set_u32(buffer + 8, 0x30);
        e.mem.set_u32(buffer + 0xc, 0);
        e.call(0x004d_6020, &args![buffer]);
        assert_eq!(e.mem.u32(stats[0x2a] + 4), 0x1000 + 0x2b);
        assert_eq!(e.mem.u32(buffer + 0xc), 0x31);
        // A count above 0x1000000 reads nothing more.
        e.mem.set_u32(buffer + 8, 0x0100_0001);
        e.mem.set_u32(buffer + 0xc, 0);
        e.call(0x004d_6020, &args![buffer]);
        assert_eq!(e.mem.u32(buffer + 0xc), 1);
    }
}
