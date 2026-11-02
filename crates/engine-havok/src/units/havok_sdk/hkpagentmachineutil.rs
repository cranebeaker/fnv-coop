//! `Havok SDK/hkpagentmachineutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The statistics functions walk the agent sectors of a machine. A sector is a block of `0x200`
//! bytes of variable-sized entries: byte `+0` is the entry type, byte `+1` the agent type, byte
//! `+3` the entry's size in bytes, and the word at `+8` the contact manager. The statistics
//! collector is a vtable object: slot `0xc` is "add a chunk" (`name, pointer, 0`, thiscall),
//! slot `0x14` is "add an array" (`3, name, data, used bytes, allocated bytes`, thiscall).
//! The dispatcher argument of the `Nn` and `1n` functions is a pointer to a word that holds
//! the dispatcher; its table of per-agent-type statistics functions is read at
//! `agent type * 0x50 + 0x16cc`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkStatisticsCollector::addNormalChunk` (`00d06fb0`, `this` = the collector, `(name, pointer,
/// used bytes, allocated bytes)`; the callee cleans the stack).
const ADD_NORMAL_CHUNK: u32 = 0x00d0_6fb0;
/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`, cdecl `(pointer, bytes)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// The names handed to the collector (pointers to exe strings): "Sector", "ContactMgr",
/// "SectorPtrs" and "Sectors".
const NAME_SECTOR: u32 = 0x010d_3ffc;
const NAME_CONTACT_MGR: u32 = 0x010d_3ff0;
const NAME_SECTOR_PTRS: u32 = 0x010c_e828;
const NAME_SECTORS: u32 = 0x010d_4004;
/// Offset of the per-agent-type statistics functions in the dispatcher, and their stride.
const AGENT_FUNCTION_TABLE: u32 = 0x16cc;
const AGENT_STRIDE: u32 = 0x50;
/// Bytes of one sector.
const SECTOR_SIZE: u32 = 0x200;

/// The statistics function the dispatcher keeps for the agent type in byte `+1` of `entry`.
fn agent_function(e: &Engine, dispatcher_holder: u32, entry: u32) -> u32 {
    let dispatcher = e.mem.u32(dispatcher_holder);
    let record = u32::from(e.mem.u8(entry + 1)).wrapping_mul(AGENT_STRIDE);
    e.mem.u32(
        dispatcher
            .wrapping_add(record)
            .wrapping_add(AGENT_FUNCTION_TABLE),
    )
}

/// Calls the agent's statistics function `(entry, entry + offset, dispatcher holder,
/// collector)` (cdecl) when the dispatcher has one.
fn call_agent_function(e: &mut Engine, holder: u32, entry: u32, offset: u32, collector: u32) {
    let function = agent_function(e, holder, entry);
    if function != 0 {
        e.call(
            function,
            &args![entry, entry.wrapping_add(offset), holder, collector],
        );
    }
}

/// Hands the entry's contact manager to the collector (slot `0xc`).
fn add_contact_manager(e: &mut Engine, collector: u32, entry: u32) {
    let manager = e.mem.u32(entry + 8);
    e.vcall(collector, 0xc, &args![NAME_CONTACT_MGR, manager, 0u32]);
}

// Translated from 00d97130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAgentMachineUtil::calcNnStatistics` (named in the engine map, cdecl): `machine` holds
/// the used bytes of the last sector (`+0`), the sector pointer array (`+4`) and the sector
/// count (`+8`, signed). Every sector is added as a chunk ("Sector", the last one with its
/// used bytes, the others `0x200`, allocated `0x200`). Then every entry of every sector (the
/// last up to its used bytes, the others `0x200`) of type 2, 6, 10 or 14 (extra data at
/// `+0x20`) or 4 or 12 (at `+0x30`) calls the agent's statistics function
/// `(entry, extra data, dispatcher holder, collector)` when the dispatcher has one, and adds
/// its contact manager ("ContactMgr", slot `0xc`); other types are skipped.
pub fn hkp_agent_machine_util_calc_nn_statistics(
    e: &mut Engine,
    machine: u32,
    dispatcher_holder: u32,
    collector: u32,
) {
    let mut index = 0i32;
    if e.mem.i32(machine + 8) > 0 {
        loop {
            let sectors = e.mem.u32(machine + 4);
            let sector = e
                .mem
                .u32(sectors.wrapping_add((index as u32).wrapping_mul(4)));
            index += 1;
            let used = if index == e.mem.i32(machine + 8) {
                e.mem.u32(machine)
            } else {
                SECTOR_SIZE
            };
            e.call(
                ADD_NORMAL_CHUNK,
                &args![collector, NAME_SECTOR, sector, used, SECTOR_SIZE],
            );
            if index >= e.mem.i32(machine + 8) {
                break;
            }
        }
    }
    index = 0;
    if e.mem.i32(machine + 8) > 0 {
        loop {
            let sectors = e.mem.u32(machine + 4);
            let sector = e
                .mem
                .u32(sectors.wrapping_add((index as u32).wrapping_mul(4)));
            index += 1;
            let end = if index == e.mem.i32(machine + 8) {
                e.mem.u32(machine).wrapping_add(sector)
            } else {
                sector.wrapping_add(SECTOR_SIZE)
            };
            let mut entry = sector;
            while entry < end {
                let extra = match e.mem.u8(entry) {
                    2 | 6 | 10 | 14 => Some(0x20),
                    4 | 12 => Some(0x30),
                    _ => None,
                };
                if let Some(offset) = extra {
                    call_agent_function(e, dispatcher_holder, entry, offset, collector);
                    add_contact_manager(e, collector, entry);
                }
                entry = entry.wrapping_add(u32::from(e.mem.u8(entry + 3)));
            }
            if index >= e.mem.i32(machine + 8) {
                break;
            }
        }
    }
}

// Translated from 00d97250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAgentMachineUtil::calc1nStatistics` (named in the engine map, cdecl): `machine` holds
/// the sector pointer array (`+0`), its size (`+4`) and its capacity word (`+8`, low 30 bits
/// the capacity, bit 31 set when the array is not owned). When the array is owned (or the
/// collector's flag byte at `+0x10` is set) the array is added through slot `0x14`
/// (`3, "SectorPtrs", data, size * 4, getAllocatedSize(data, capacity * 4)`). Then the
/// sectors are walked from the first on, with no bound other than the terminator: each sector
/// is added as a chunk ("Sector", used bytes + `0x10` of `0x200`) and its entries (from
/// `+0x10`, up to the used bytes in the word at `+0`) are visited: type 1 ends the whole walk;
/// type 0 is skipped; types 2, 3, 6, 10, 11 and 14 call the agent's statistics function with
/// extra data at `+0x10`, types 4, 5, 12 and 13 with extra data at `+0x20`. The exe spins
/// forever on an entry of any other type (7 to 9, or above 14); this translation panics
/// instead.
pub fn hkp_agent_machine_util_calc_1n_statistics(
    e: &mut Engine,
    machine: u32,
    dispatcher_holder: u32,
    collector: u32,
) {
    let capacity_word = e.mem.u32(machine + 8);
    if (capacity_word as i32) >= 0 || e.mem.u8(collector + 0x10) != 0 {
        let data = e.mem.u32(machine);
        let bytes = (capacity_word & 0x3fff_ffff).wrapping_mul(4);
        let allocated = e.call(GET_ALLOCATED_SIZE, &args![data, bytes]).u32();
        let data = e.mem.u32(machine);
        let listed = e.mem.u32(machine + 4).wrapping_mul(4);
        e.vcall(
            collector,
            0x14,
            &args![3u32, NAME_SECTOR_PTRS, data, listed, allocated],
        );
    }
    let mut offset = 0u32;
    loop {
        let sectors = e.mem.u32(machine);
        let sector = e.mem.u32(sectors.wrapping_add(offset));
        let used = e.mem.u32(sector);
        let end = used.wrapping_add(sector).wrapping_add(0x10);
        let mut entry = sector + 0x10;
        e.call(
            ADD_NORMAL_CHUNK,
            &args![
                collector,
                NAME_SECTOR,
                sector,
                used.wrapping_add(0x10),
                SECTOR_SIZE
            ],
        );
        offset += 4;
        loop {
            match e.mem.u8(entry) {
                0 => {}
                1 => return,
                kind @ (2 | 3 | 4 | 5 | 6 | 10 | 11 | 12 | 13 | 14) => {
                    let extra = if matches!(kind, 4 | 5 | 12 | 13) {
                        0x20
                    } else {
                        0x10
                    };
                    call_agent_function(e, dispatcher_holder, entry, extra, collector);
                }
                kind => panic!(
                    "{entry:#010x}: entry type {kind} never advances, the exe loops forever (00d97250)"
                ),
            }
            entry = entry.wrapping_add(u32::from(e.mem.u8(entry + 3)));
            if entry >= end {
                break;
            }
        }
    }
}

// Translated from 00d97360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAgentMachineUtil::calcNnStatisticsContactMgrsOnly` (named in the engine map, cdecl):
/// `machine` holds the used bytes of the last sector (`+0`), the sector pointer array (`+4`),
/// the sector count (`+8`, signed) and the array's capacity word (`+0xc`, low 30 bits the
/// capacity, bit 31 set when not owned). When the array is owned (or the collector's flag byte
/// at `+0x10` is set) it is added through slot `0x14` (`3, "Sectors", data, count * 4,
/// getAllocatedSize(data, capacity * 4)`). Then every entry of every sector (the last up to its
/// used bytes) of type 2, 4, 6, 10, 12 or 14 has its contact manager added (slot `0xc`).
pub fn hkp_agent_machine_util_calc_nn_statistics_contact_mgrs_only(
    e: &mut Engine,
    machine: u32,
    collector: u32,
) {
    let capacity_word = e.mem.u32(machine + 0xc);
    if (capacity_word as i32) >= 0 || e.mem.u8(collector + 0x10) != 0 {
        let data = e.mem.u32(machine + 4);
        let bytes = (capacity_word & 0x3fff_ffff).wrapping_mul(4);
        let allocated = e.call(GET_ALLOCATED_SIZE, &args![data, bytes]).u32();
        let data = e.mem.u32(machine + 4);
        let listed = e.mem.u32(machine + 8).wrapping_mul(4);
        e.vcall(
            collector,
            0x14,
            &args![3u32, NAME_SECTORS, data, listed, allocated],
        );
    }
    let mut index = 0i32;
    if e.mem.i32(machine + 8) > 0 {
        loop {
            let sectors = e.mem.u32(machine + 4);
            let sector = e
                .mem
                .u32(sectors.wrapping_add((index as u32).wrapping_mul(4)));
            index += 1;
            let end = if index == e.mem.i32(machine + 8) {
                e.mem.u32(machine).wrapping_add(sector)
            } else {
                sector.wrapping_add(SECTOR_SIZE)
            };
            let mut entry = sector;
            while entry < end {
                if matches!(e.mem.u8(entry), 2 | 4 | 6 | 10 | 12 | 14) {
                    add_contact_manager(e, collector, entry);
                }
                entry = entry.wrapping_add(u32::from(e.mem.u8(entry + 3)));
            }
            if index >= e.mem.i32(machine + 8) {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d9_7130,
            hkp_agent_machine_util_calc_nn_statistics(u32, u32, u32)
        ),
        entry!(
            0x00d9_7250,
            hkp_agent_machine_util_calc_1n_statistics(u32, u32, u32)
        ),
        entry!(
            0x00d9_7360,
            hkp_agent_machine_util_calc_nn_statistics_contact_mgrs_only(u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const AGENT_DOUBLE: u32 = 0x0900_0010;
    const MANAGER_DOUBLE: u32 = 0x0900_0020;
    const ARRAY_DOUBLE: u32 = 0x0900_0030;
    /// The agent type that has a statistics function in the test dispatcher.
    const AGENT_WITH_FUNCTION: u8 = 5;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The calls to `addr` as argument lists.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A collector (slot `0xc` and `0x14` doubles), a dispatcher holder whose table has a
    /// function for agent type 5 only, and the other callee doubles. Returns
    /// `(collector, holder)`.
    fn fixture(e: &mut Engine) -> (u32, u32) {
        for address in [AGENT_DOUBLE, MANAGER_DOUBLE, ARRAY_DOUBLE, ADD_NORMAL_CHUNK] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.register_double(GET_ALLOCATED_SIZE, |_, a| Ret {
            eax: a[1] + 1000,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0xc, MANAGER_DOUBLE);
        e.mem.set_u32(table + 0x14, ARRAY_DOUBLE);
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, table);
        let dispatcher = e.mem.alloc(0x2000);
        e.mem.set_u32(
            dispatcher + AGENT_FUNCTION_TABLE + u32::from(AGENT_WITH_FUNCTION) * AGENT_STRIDE,
            AGENT_DOUBLE,
        );
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, dispatcher);
        (collector, holder)
    }

    /// Writes an entry of `size` bytes at `entry`.
    fn entry(e: &mut Engine, entry: u32, kind: u8, agent: u8, size: u8, manager: u32) {
        e.mem.set_u8(entry, kind);
        e.mem.set_u8(entry + 1, agent);
        e.mem.set_u8(entry + 3, size);
        e.mem.set_u32(entry + 8, manager);
    }

    /// Two sectors of the `Nn` layout (`0x80`-byte entries starting at the sector). The first
    /// is full: types 2, 3, 4, 12 with agent types 5, 5, 6, 5; the second holds `0x100` used
    /// bytes: types 14 and 5. Returns `(machine, sector 0, sector 1)`.
    fn nn_machine(e: &mut Engine) -> (u32, u32, u32) {
        let first = e.mem.alloc(SECTOR_SIZE);
        let second = e.mem.alloc(SECTOR_SIZE);
        let layout = [
            (first, [(2u8, 5u8), (3, 5), (4, 6), (12, 5)]),
            (second, [(14, 5), (5, 5), (2, 5), (2, 5)]),
        ];
        for (sector, entries) in layout {
            for (i, (kind, agent)) in entries.into_iter().enumerate() {
                let at = sector + 0x80 * i as u32;
                entry(e, at, kind, agent, 0x80, 0xa000 + at);
            }
        }
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, first);
        e.mem.set_u32(array + 4, second);
        let machine = e.mem.alloc(0x20);
        e.mem.set_u32(machine, 0x100);
        e.mem.set_u32(machine + 4, array);
        e.mem.set_u32(machine + 8, 2);
        e.mem.set_u32(machine + 0xc, 8);
        (machine, first, second)
    }

    #[test]
    fn calc_nn_statistics_adds_the_sectors_then_the_agent_entries() {
        let mut e = engine();
        let (collector, holder) = fixture(&mut e);
        let (machine, first, second) = nn_machine(&mut e);
        e.call(0x00d9_7130, &args![machine, holder, collector]);
        assert_eq!(
            calls(&e, ADD_NORMAL_CHUNK),
            vec![
                vec![collector, NAME_SECTOR, first, SECTOR_SIZE, SECTOR_SIZE],
                vec![collector, NAME_SECTOR, second, 0x100, SECTOR_SIZE],
            ]
        );
        // Entries 0 (type 2, +0x20) and 3 (type 12, +0x30) of the first sector, entry 0
        // (type 14, +0x20) of the second; the type 4 entry has agent type 6 (no function).
        assert_eq!(
            calls(&e, AGENT_DOUBLE),
            vec![
                vec![first, first + 0x20, holder, collector],
                vec![first + 0x180, first + 0x180 + 0x30, holder, collector],
                vec![second, second + 0x20, holder, collector],
            ]
        );
        let managers: Vec<u32> = calls(&e, MANAGER_DOUBLE)
            .into_iter()
            .map(|words| words[2])
            .collect();
        assert_eq!(
            managers,
            vec![
                0xa000 + first,
                0xa000 + first + 0x100,
                0xa000 + first + 0x180,
                0xa000 + second,
            ]
        );
        assert!(calls(&e, MANAGER_DOUBLE)
            .iter()
            .all(|words| words[0] == collector && words[1] == NAME_CONTACT_MGR && words[3] == 0));
    }

    #[test]
    fn calc_nn_statistics_does_nothing_without_sectors() {
        let mut e = engine();
        let (collector, holder) = fixture(&mut e);
        let (machine, _, _) = nn_machine(&mut e);
        e.mem.set_u32(machine + 8, 0);
        e.call(0x00d9_7130, &args![machine, holder, collector]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
    }

    /// One sector of the `1n` layout: entries from `+0x10` of types 2, 0, 4, 3 (agent type 5,
    /// `0x20` bytes each) and then a type 1 terminator; the used-bytes word is `0xa0`.
    fn one_n_machine(e: &mut Engine, capacity_word: u32) -> (u32, u32) {
        let sector = e.mem.alloc(SECTOR_SIZE);
        e.mem.set_u32(sector, 0xa0);
        let kinds = [2u8, 0, 4, 3, 1];
        for (i, kind) in kinds.into_iter().enumerate() {
            entry(e, sector + 0x10 + 0x20 * i as u32, kind, 5, 0x20, 0);
        }
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, sector);
        let machine = e.mem.alloc(0x10);
        e.mem.set_u32(machine, array);
        e.mem.set_u32(machine + 4, 3);
        e.mem.set_u32(machine + 8, capacity_word);
        (machine, sector)
    }

    #[test]
    fn calc_1n_statistics_adds_the_array_and_walks_to_the_terminator() {
        let mut e = engine();
        let (collector, holder) = fixture(&mut e);
        let (machine, sector) = one_n_machine(&mut e, 5);
        let array = e.mem.u32(machine);
        e.call(0x00d9_7250, &args![machine, holder, collector]);
        assert_eq!(calls(&e, GET_ALLOCATED_SIZE), vec![vec![array, 20]]);
        assert_eq!(
            calls(&e, ARRAY_DOUBLE),
            vec![vec![collector, 3, NAME_SECTOR_PTRS, array, 12, 1020]]
        );
        assert_eq!(
            calls(&e, ADD_NORMAL_CHUNK),
            vec![vec![collector, NAME_SECTOR, sector, 0xb0, SECTOR_SIZE]]
        );
        // Type 2 at +0x10 (extra data +0x10), type 4 (+0x20), type 3 (+0x10); type 0 skipped.
        let first = sector + 0x10;
        assert_eq!(
            calls(&e, AGENT_DOUBLE),
            vec![
                vec![first, first + 0x10, holder, collector],
                vec![first + 0x40, first + 0x40 + 0x20, holder, collector],
                vec![first + 0x60, first + 0x60 + 0x10, holder, collector],
            ]
        );
    }

    #[test]
    fn calc_1n_statistics_skips_the_array_when_it_is_not_owned_unless_the_flag_is_set() {
        let mut e = engine();
        let (collector, holder) = fixture(&mut e);
        let (machine, _) = one_n_machine(&mut e, 0x8000_0005);
        e.call(0x00d9_7250, &args![machine, holder, collector]);
        assert!(calls(&e, ARRAY_DOUBLE).is_empty());
        assert!(calls(&e, GET_ALLOCATED_SIZE).is_empty());
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00d9_7250, &args![machine, holder, collector]);
        assert_eq!(calls(&e, GET_ALLOCATED_SIZE).len(), 1);
        assert_eq!(calls(&e, GET_ALLOCATED_SIZE)[0][1], 20);
    }

    #[test]
    fn calc_1n_statistics_continues_with_the_next_sector() {
        let mut e = engine();
        let (collector, holder) = fixture(&mut e);
        let (machine, first) = one_n_machine(&mut e, 5);
        // The first sector now holds only a type 0 entry that fills it (used word 0x10); a
        // second sector with a terminator follows.
        e.mem.set_u32(first, 0x10);
        entry(&mut e, first + 0x10, 0, 5, 0x10, 0);
        let second = e.mem.alloc(SECTOR_SIZE);
        e.mem.set_u32(second, 0x10);
        entry(&mut e, second + 0x10, 1, 5, 0x10, 0);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, first);
        e.mem.set_u32(array + 4, second);
        e.mem.set_u32(machine, array);
        e.call(0x00d9_7250, &args![machine, holder, collector]);
        assert_eq!(
            calls(&e, ADD_NORMAL_CHUNK),
            vec![
                vec![collector, NAME_SECTOR, first, 0x20, SECTOR_SIZE],
                vec![collector, NAME_SECTOR, second, 0x20, SECTOR_SIZE],
            ]
        );
        assert!(calls(&e, AGENT_DOUBLE).is_empty());
    }

    #[test]
    #[should_panic(expected = "never advances")]
    fn calc_1n_statistics_panics_where_the_exe_would_loop_forever() {
        let mut e = engine();
        let (collector, holder) = fixture(&mut e);
        let (machine, sector) = one_n_machine(&mut e, 5);
        e.mem.set_u8(sector + 0x10, 8);
        e.call(0x00d9_7250, &args![machine, holder, collector]);
    }

    #[test]
    fn contact_mgrs_only_adds_the_array_and_the_managers_of_the_listed_types() {
        let mut e = engine();
        let (collector, _) = fixture(&mut e);
        let (machine, first, second) = nn_machine(&mut e);
        let array = e.mem.u32(machine + 4);
        e.call(0x00d9_7360, &args![machine, collector]);
        assert_eq!(calls(&e, GET_ALLOCATED_SIZE), vec![vec![array, 32]]);
        assert_eq!(
            calls(&e, ARRAY_DOUBLE),
            vec![vec![collector, 3, NAME_SECTORS, array, 8, 1032]]
        );
        // Types 2, 4 and 12 of the first sector, 14 and 2 of the second; the type 3 and 5
        // entries are skipped, and the second sector stops after two entries.
        let managers: Vec<u32> = calls(&e, MANAGER_DOUBLE)
            .into_iter()
            .map(|words| words[2])
            .collect();
        assert_eq!(
            managers,
            vec![
                0xa000 + first,
                0xa000 + first + 0x100,
                0xa000 + first + 0x180,
                0xa000 + second,
            ]
        );
        assert!(calls(&e, AGENT_DOUBLE).is_empty());
        assert!(calls(&e, ADD_NORMAL_CHUNK).is_empty());
    }

    #[test]
    fn contact_mgrs_only_skips_an_unowned_array_unless_the_flag_is_set() {
        let mut e = engine();
        let (collector, _) = fixture(&mut e);
        let (machine, _, _) = nn_machine(&mut e);
        e.mem.set_u32(machine + 0xc, 0x8000_0008);
        e.call(0x00d9_7360, &args![machine, collector]);
        assert!(calls(&e, ARRAY_DOUBLE).is_empty());
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00d9_7360, &args![machine, collector]);
        assert_eq!(calls(&e, ARRAY_DOUBLE).len(), 1);
    }
}
