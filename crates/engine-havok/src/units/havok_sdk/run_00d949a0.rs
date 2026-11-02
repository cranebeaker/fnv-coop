//! `Havok SDK/run_00d949a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkSimpleContactConstraintDataBuildJacobian` (`00d72190`, engine map name): cdecl `(atoms,
/// info, flag, &cursor)`; the third word is tested as a byte.
const BUILD_JACOBIAN_CONTACT: u32 = 0x00d7_2190;
/// `hkSolverBuildJacobianFromAtomsNotContact` (`00d6f460`, engine map name): cdecl `(atoms,
/// word, info, &cursor)`.
const BUILD_JACOBIAN_NOT_CONTACT: u32 = 0x00d6_f460;
/// The `double` `1.1920929e-07` (`010c9300`) the closing header adds to its float.
const HEADER_EPSILON: u32 = 0x010c_9300;

// Translated from 00d949a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the Jacobians of a job's constraints. Register inputs first: `job` (`EAX`) and `info`
/// (`ESI`, the solver info record); no stack words (the exe's `RET`). The job holds the entries
/// pointer at `+0`, their count at `+4` (28-byte entries), a base address at `+8`, the output
/// cursor at `+0xc`, the output limit at `+0x10` and a flag byte at `+0x18`. For each entry the
/// 0x30..0x3c, 0x60..0x74 words of `info` are loaded from it (`+0x30`/`+0x34`: the 16-bit
/// indices at `+0x18`/`+0x1a` times 128 plus the base; `+0x6c`/`+0x70` the indices themselves;
/// `+0x38`, `+0x3c`, `+0x60`, `+0x64`, `+0x74` entry words) and the atoms at the entry's first
/// word are built with `00d72190` (when their 16-bit type is `0x16`, with the flag 1; the other
/// three bytes of that flag word are stale stack in the exe and unread) or `00d6f460` (with the
/// 16-bit word at `+0x14`), both given the address of the running cursor. Afterwards, with the
/// flag byte at `+0x18` clear the cursor's dword is zeroed; otherwise, when room is left up to
/// the limit, a closing header is written at the cursor: its float plus `1.1920929e-07` with
/// the low byte forced to 2, and the remaining byte count at `+4`. Returns the cursor (`EAX`).
pub fn fn_00d949a0(e: &mut Engine, job: u32, info: u32) -> u32 {
    let count = e.mem.i32(job.wrapping_add(4));
    let cursor_start = e.mem.u32(job.wrapping_add(0xc));
    let mut entry = e.mem.u32(job);
    // The local the callees update: the output cursor.
    let cursor = e.with_stack(8, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, cursor_start);
        let mut remaining = count;
        if count > 0 {
            loop {
                let atoms_entry = entry;
                let word = e.mem.u32(atoms_entry.wrapping_add(0xc));
                e.mem.set_u32(info.wrapping_add(0x38), word);
                let word = e.mem.u32(atoms_entry.wrapping_add(0x10));
                e.mem.set_u32(info.wrapping_add(0x3c), word);
                let first_index = u32::from(e.mem.u16(atoms_entry.wrapping_add(0x18)));
                let base = e.mem.u32(job.wrapping_add(8));
                entry = entry.wrapping_add(0x1c);
                e.mem.set_u32(
                    info.wrapping_add(0x30),
                    (first_index << 7).wrapping_add(base),
                );
                let second_index = u32::from(e.mem.u16(atoms_entry.wrapping_add(0x1a)));
                let base = e.mem.u32(job.wrapping_add(8));
                e.mem.set_u32(
                    info.wrapping_add(0x34),
                    (second_index << 7).wrapping_add(base),
                );
                let first_index = u32::from(e.mem.u16(atoms_entry.wrapping_add(0x18)));
                e.mem.set_u32(info.wrapping_add(0x6c), first_index);
                let second_index = u32::from(e.mem.u16(atoms_entry.wrapping_add(0x1a)));
                e.mem.set_u32(info.wrapping_add(0x70), second_index);
                let word = e.mem.u32(atoms_entry.wrapping_add(4));
                e.mem.set_u32(info.wrapping_add(0x60), word);
                let word = e.mem.u32(atoms_entry.wrapping_add(8));
                e.mem.set_u32(info.wrapping_add(0x64), word);
                let word = e.mem.u32(atoms_entry.wrapping_add(8));
                e.mem.set_u32(info.wrapping_add(0x74), word);
                let kind_word = u32::from(e.mem.u16(atoms_entry.wrapping_add(0x14)));
                let atoms = e.mem.u32(atoms_entry);
                if e.mem.u16(atoms) == 0x16 {
                    e.call(BUILD_JACOBIAN_CONTACT, &args![atoms, info, 1u32, local]);
                } else {
                    e.call(
                        BUILD_JACOBIAN_NOT_CONTACT,
                        &args![atoms, kind_word, info, local],
                    );
                }
                remaining = remaining.wrapping_sub(1);
                if remaining == 0 {
                    break;
                }
            }
        }
        e.mem.u32(local)
    });
    if e.mem.u8(job.wrapping_add(0x18)) == 0 {
        e.mem.set_u32(cursor, 0);
        return cursor;
    }
    let left = e.mem.u32(job.wrapping_add(0x10)).wrapping_sub(cursor) as i32;
    if left > 0 {
        let float = x87_load32(e.mem.u32(cursor));
        e.mem.set_u32(cursor.wrapping_add(4), left as u32);
        let epsilon = e.mem.f64(HEADER_EPSILON);
        let bits = x87_store32(x87_add(float, epsilon));
        e.mem.set_u32(cursor, bits);
        e.mem.set_u8(cursor, 2);
    }
    cursor
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d9_49a0, fn_00d949a0(u32, u32) -> u32)]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Two doubles over the builders; each advances the cursor by 0x10 and logs its arguments.
    fn builders(e: &mut Engine) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        for address in [BUILD_JACOBIAN_CONTACT, BUILD_JACOBIAN_NOT_CONTACT] {
            let log = Rc::clone(&seen);
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let cursor = a[3];
                let value = e.mem.u32(cursor);
                e.mem.set_u32(cursor, value + 0x10);
                Ret::default()
            });
        }
        seen
    }

    /// A job with the given entries `(atoms type, kind word)`; the output starts at 0x2000 of a
    /// mapped region with room up to `limit`, flag byte `flag`.
    fn make_job(e: &mut Engine, entries: &[(u16, u16)], limit: u32, flag: u8) -> (u32, u32) {
        e.map(0x2000, 0x1000);
        let job = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x1c * entries.len() as u32 + 8);
        for (i, (atoms_type, kind)) in entries.iter().enumerate() {
            let entry = data + 0x1c * i as u32;
            let atoms = e.mem.alloc(8);
            e.mem.set_u16(atoms, *atoms_type);
            e.mem.set_u32(entry, atoms);
            e.mem.set_u32(entry + 4, 0x100 + i as u32);
            e.mem.set_u32(entry + 8, 0x200 + i as u32);
            e.mem.set_u32(entry + 0xc, 0x300 + i as u32);
            e.mem.set_u32(entry + 0x10, 0x400 + i as u32);
            e.mem.set_u16(entry + 0x14, *kind);
            e.mem.set_u16(entry + 0x18, 2 + i as u16);
            e.mem.set_u16(entry + 0x1a, 7);
        }
        e.mem.set_u32(job, data);
        e.mem.set_u32(job + 4, entries.len() as u32);
        e.mem.set_u32(job + 8, 0x10000);
        e.mem.set_u32(job + 0xc, 0x2000);
        e.mem.set_u32(job + 0x10, limit);
        e.mem.set_u8(job + 0x18, flag);
        let info = e.mem.alloc(0x80);
        (job, info)
    }

    #[test]
    fn each_entry_loads_the_info_words_and_picks_the_builder_by_atoms_type() {
        let mut e = Engine::new();
        let seen = builders(&mut e);
        let (job, info) = make_job(&mut e, &[(0x16, 5), (0x20, 6)], 0x2100, 0);
        let atoms_first = e.mem.u32(e.mem.u32(job));
        let atoms_second = e.mem.u32(e.mem.u32(job) + 0x1c);
        let cursor = e.call(0x00d9_49a0, &args![job, info]).u32();
        assert_eq!(cursor, 0x2020);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].0, BUILD_JACOBIAN_CONTACT);
        assert_eq!(seen[0].1[..3], [atoms_first, info, 1]);
        assert_eq!(seen[1].0, BUILD_JACOBIAN_NOT_CONTACT);
        assert_eq!(seen[1].1[..3], [atoms_second, 6, info]);
        // The second entry's words are what is left in the info record.
        assert_eq!(e.mem.u32(info + 0x38), 0x301);
        assert_eq!(e.mem.u32(info + 0x3c), 0x401);
        assert_eq!(e.mem.u32(info + 0x30), (3 << 7) + 0x10000);
        assert_eq!(e.mem.u32(info + 0x34), (7 << 7) + 0x10000);
        assert_eq!(e.mem.u32(info + 0x6c), 3);
        assert_eq!(e.mem.u32(info + 0x70), 7);
        assert_eq!(e.mem.u32(info + 0x60), 0x101);
        assert_eq!(e.mem.u32(info + 0x64), 0x201);
        assert_eq!(e.mem.u32(info + 0x74), 0x201);
        // The flag byte is clear: the cursor's dword is zeroed.
        assert_eq!(e.mem.u32(cursor), 0);
    }

    #[test]
    fn a_set_flag_writes_the_closing_header_when_room_is_left() {
        let mut e = Engine::new();
        builders(&mut e);
        e.map(0x010c_9000, 0x1000);
        e.mem.set_f64(HEADER_EPSILON, 1.0);
        let (job, info) = make_job(&mut e, &[(0x16, 0)], 0x2100, 1);
        e.mem.set_f32(0x2010, 1.5);
        let cursor = e.call(0x00d9_49a0, &args![job, info]).u32();
        assert_eq!(cursor, 0x2010);
        // 1.5 + 1.0 = 2.5 with the low byte replaced by 2; the room is 0xf0 bytes.
        assert_eq!(e.mem.u32(0x2010), (2.5f32.to_bits() & !0xff) | 2);
        assert_eq!(e.mem.u32(0x2014), 0xf0);
    }

    #[test]
    fn a_set_flag_without_room_writes_nothing_and_an_empty_job_keeps_the_cursor() {
        let mut e = Engine::new();
        let seen = builders(&mut e);
        e.map(0x010c_9000, 0x1000);
        let (job, info) = make_job(&mut e, &[(0x16, 0)], 0x2010, 1);
        e.mem.set_u32(0x2010, 0x1234_5678);
        let cursor = e.call(0x00d9_49a0, &args![job, info]).u32();
        assert_eq!(cursor, 0x2010);
        assert_eq!(e.mem.u32(0x2010), 0x1234_5678);
        assert_eq!(seen.borrow().len(), 1);
        let (empty, info) = make_job(&mut e, &[], 0x2100, 0);
        e.mem.set_u32(0x2000, 0xabcd);
        assert_eq!(e.call(0x00d9_49a0, &args![empty, info]).u32(), 0x2000);
        assert_eq!(e.mem.u32(0x2000), 0);
        assert_eq!(seen.borrow().len(), 1);
    }
}
