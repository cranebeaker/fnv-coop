//! `fallout shared/run_005e3630` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e3c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0051b950` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e3c30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_b950, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `00406d30`: copies the string `source` into `destination` of `size`
/// bytes (cdecl: destination, size, source).
const STRING_COPY: u32 = 0x0040_6d30;
/// `00406d50`: appends `source` to `destination` of `size` bytes (cdecl:
/// destination, size, source).
const STRING_APPEND: u32 = 0x0040_6d50;
/// `"*.wav"`.
const WAV_PATTERN: u32 = 0x0104_51c4;
/// `"*.ogg"`.
const OGG_PATTERN: u32 = 0x0104_51bc;
/// `FileFinder::BuildFileList` (`00afe420`, engine map): cdecl, four words.
const FILE_FINDER_BUILD_FILE_LIST: u32 = 0x00af_e420;
/// `00af7400` (`BSSystem/archive.cpp`): cdecl, three words.
const ARCHIVE_FIND_00AF7400: u32 = 0x00af_7400;
/// `005ae380`: the number of entries of a file list (`ECX` = the list; the
/// map's name `VATS::GetCount` is folded code).
const LIST_COUNT: u32 = 0x005a_e380;
/// `006815c0`: returns its `ECX` unchanged (used as the address of a list
/// node's item, the node's first word).
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070`: the next list node (the node's word at +4; `ECX` = the
/// node).
const NODE_NEXT: u32 = 0x0072_6070;
/// The flag `fn_005e39a0` returns.
const USE_FILE_LISTS_FLAG: u32 = 0x011a_cdf9;
/// `fn_005e3630`'s ten remembered name hashes (`int[10]`).
const RECENT_HASHES: u32 = 0x011c_af04;
/// The choice remembered with each hash (`char[10]`).
const RECENT_CHOICES: u32 = 0x011c_af2c;
/// The slot the next hash is written to (`char`).
const RECENT_SLOT: u32 = 0x011c_af36;
/// The three words `fn_005e3a00` copies twice (to +0x2C and +0x38).
const DEFAULT_TRIPLE: u32 = 0x011f_426c;
/// `BGSBodyPart`'s vtable (RTTI `.?AVBGSBodyPart@@`).
const BGS_BODY_PART_VTABLE: u32 = 0x0104_5404;
/// `TESModelPSA`'s vtable (RTTI `.?AVTESModelPSA@@`).
const TES_MODEL_PSA_VTABLE: u32 = 0x0104_540c;

// Translated from 005e3630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e3630 - the /GS security-cookie check (00ec408c) is not translated.
/// Picks a sound file for the path in `path` (`char[0x104]`, cdecl, one
/// word) and writes the chosen file name back into it; returns whether one
/// was found.
///
/// Without the flag `011acdf9` it asks `00af7400` for `path` + `"*.wav"`
/// and copies the found name back. With it, it builds the file list of
/// `"*.ogg"` (then of `"*.wav"` when that is empty), takes a random index
/// (`00487f50` modulo the count) that differs from the one remembered for
/// the path's name hash (`BSGameSound::GetNameHash`, `00ae9d80`; redrawn
/// through `005a00a0` while equal), remembers the pair in the slot
/// `011caf36` (which the code sets to itself modulo 10, so it never moves),
/// copies that entry's name into `path`, frees every entry's name
/// (`00401030`), empties the list (`00470470`) and deletes it (`004702f0`).
pub fn fn_005e3630(e: &mut Engine, path: Ptr) -> bool {
    e.with_stack(0x318, |e, locals| {
        let wav = locals.addr();
        let base = locals.addr() + 0x108;
        let ogg = locals.addr() + 0x210;
        e.call(STRING_COPY, &args![wav, 0x104u32, path]);
        e.call(STRING_COPY, &args![base, 0x104u32, path]);
        e.call(STRING_APPEND, &args![wav, 0x104u32, WAV_PATTERN]);
        e.call(STRING_COPY, &args![ogg, 0x104u32, path]);
        e.call(STRING_APPEND, &args![ogg, 0x104u32, OGG_PATTERN]);
        if fn_005e39a0(e) == 0 {
            let found = e.call(ARCHIVE_FIND_00AF7400, &args![wav, base, 8u32]).u8();
            if found != 0 {
                e.call(STRING_COPY, &args![path, 0x104u32, base]);
            }
            return found != 0;
        }
        let mut list = e
            .call(FILE_FINDER_BUILD_FILE_LIST, &args![ogg, base, 8u32, 0u32])
            .u32();
        if list == 0 {
            list = e
                .call(FILE_FINDER_BUILD_FILE_LIST, &args![wav, base, 8u32, 0u32])
                .u32();
        }
        if list == 0 {
            return false;
        }
        let count = e.call(LIST_COUNT, &args![list]).u32();
        if count == 0 {
            e.call(0x0047_02f0, &args![list, 1u32]);
            return false;
        }
        let mut choice = e.call(0x0048_7f50, &args![]).u32() % count;
        if count > 1 {
            let hash = e.call(0x00ae_9d80, &args![path]).u32();
            let mut index = 0u32;
            while index < 10 {
                if e.mem.u32(RECENT_HASHES + 4 * index) == hash {
                    let remembered = e.mem.i8(RECENT_CHOICES + index) as i32 as u32;
                    while choice == remembered {
                        choice = e.call(0x005a_00a0, &args![count]).u32();
                    }
                    break;
                }
                index += 1;
            }
            let slot = e.mem.i8(RECENT_SLOT) as i32;
            e.mem
                .set_u32(RECENT_HASHES.wrapping_add((slot * 4) as u32), hash);
            e.mem
                .set_u8(RECENT_CHOICES.wrapping_add(slot as u32), choice as u8);
            let slot = e.mem.i8(RECENT_SLOT) as i32;
            e.mem.set_u8(RECENT_SLOT, (slot % 10) as u8);
        }
        let mut node = list;
        while node != 0 {
            let item = e.call(NODE_ITEM, &args![node]).u32();
            if e.mem.u32(item) != 0 {
                if choice == 0 {
                    let item = e.call(NODE_ITEM, &args![node]).u32();
                    let name = e.mem.u32(item);
                    e.call(STRING_COPY, &args![path, 0x104u32, name]);
                }
                let item = e.call(NODE_ITEM, &args![node]).u32();
                let name = e.mem.u32(item);
                e.call(0x0040_1030, &args![name]);
            }
            choice = choice.wrapping_sub(1);
            node = e.call(NODE_NEXT, &args![node]).u32();
        }
        e.call(0x0047_0470, &args![list]);
        e.call(0x0047_02f0, &args![list, 1u32]);
        true
    })
}

// Translated from 005e39a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte flag at `011acdf9` (in `AL`).
pub fn fn_005e39a0(e: &mut Engine) -> u8 {
    e.mem.u8(USE_FILE_LISTS_FLAG)
}

// Translated from 005e39b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Maps flags (cdecl, one word) to one value: 0x10 when bit 0x10 is set,
/// else 0x2000000 for bit 0x200, else 0x4000000 for bit 0x400, else 0.
pub fn fn_005e39b0(_e: &mut Engine, flags: u32) -> u32 {
    if flags & 0x10 != 0 {
        0x10
    } else if flags & 0x200 != 0 {
        0x0200_0000
    } else if flags & 0x400 != 0 {
        0x0400_0000
    } else {
        0
    }
}

// Translated from 005e3a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 0x54-byte block at `this` (`ECX`) to its start values: bytes
/// +6 = 5, +7 = 0xFF, +8 = 100, +9 and +0xA = 0; floats +0, +0x18, +0x28
/// and +0x50 = 1.0 (`FLD1`), +0x14 = 0.0 (`FLDZ`); words +0xC, +0x10,
/// +0x20, +0x24, +0x44, +0x48 = 0; bytes +0x1C, +0x4C, +0x4D = 0; and the
/// three words at `011f426c` copied to +0x2C and to +0x38.
pub fn fn_005e3a00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 0x6, 5);
    e.mem.set_f32(base, 1.0);
    e.mem.set_u8(base + 0x7, 0xff);
    e.mem.set_u8(base + 0x8, 100);
    e.mem.set_u8(base + 0x9, 0);
    e.mem.set_u8(base + 0xa, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_f32(base + 0x14, 0.0);
    e.mem.set_f32(base + 0x18, 1.0);
    e.mem.set_f32(base + 0x50, 1.0);
    e.mem.set_u8(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_f32(base + 0x28, 1.0);
    for target in [base + 0x2c, base + 0x38] {
        for word in 0..3 {
            let value = e.mem.u32(DEFAULT_TRIPLE + 4 * word);
            e.mem.set_u32(target + 4 * word, value);
        }
    }
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u8(base + 0x4c, 0);
    e.mem.set_u8(base + 0x4d, 0);
}

// Translated from 005e3af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e3af0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSBodyPart` constructor (the class of the vtable `01045404` it stores,
/// by its RTTI): the five strings at +0x4, +0xC, +0x14, +0x1C, +0x24
/// (`004037b0`), the component at +0x2C (`00488f50`), the `TESModelPSA` at
/// +0x44 ([`fn_005e3be0`]), the block at +0x5C ([`fn_005e3c00`], cleared
/// with `00403d30`(+0x5C, 0, 0x54), then [`fn_005e3a00`]), and finally
/// `005e3d20`. Returns `this`.
pub fn fn_005e3af0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, BGS_BODY_PART_VTABLE);
    for offset in [0x4, 0xc, 0x14, 0x1c, 0x24] {
        e.call(0x0040_37b0, &args![base + offset]);
    }
    e.call(0x0048_8f50, &args![base + 0x2c]);
    fn_005e3be0(e, Ptr::new(base + 0x44));
    fn_005e3c00(e, Ptr::new(base + 0x5c));
    e.call(0x0040_3d30, &args![base + 0x5c, 0u32, 0x54u32]);
    fn_005e3a00(e, Ptr::new(base + 0x5c));
    e.call(0x005e_3d20, &args![this]);
    this
}

// Translated from 005e3be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelPSA` constructor (the class of the vtable `0104540c`, by its
/// RTTI): the base constructor `00488f50`, then the vtable. Returns `this`.
pub fn fn_005e3be0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0048_8f50, &args![this]);
    e.mem.set_u32(this.addr(), TES_MODEL_PSA_VTABLE);
    this
}

// Translated from 005e3c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `006815c0` on the members at `this + 0x2C` and `this + 0x38`.
/// Returns `this`.
pub fn fn_005e3c00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NODE_ITEM, &args![this.byte_add(0x2c)]);
    e.call(NODE_ITEM, &args![this.byte_add(0x38)]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_3c30, fn_005e3c30(Ptr, u32) -> Ptr),
        entry!(0x005e_3630, fn_005e3630(Ptr) -> bool),
        entry!(0x005e_39a0, fn_005e39a0() -> u8),
        entry!(0x005e_39b0, fn_005e39b0(u32) -> u32),
        entry!(0x005e_3a00, fn_005e3a00(Ptr)),
        entry!(0x005e_3af0, fn_005e3af0(Ptr) -> Ptr),
        entry!(0x005e_3be0, fn_005e3be0(Ptr) -> Ptr),
        entry!(0x005e_3c00, fn_005e3c00(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_005e3c30_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_b950, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_3c30, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_3c30)
                .collect();
            let mut expected = vec![(0x0051_b950, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    /// Doubles for the string helpers (real copies on the C strings), the
    /// list walkers (node = {item, next}) and the globals' pages.
    fn sound_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.map(0x011c_a000, 0x1000);
        e.map(0x0104_5000, 0x1000);
        e.mem.set_cstr(WAV_PATTERN, b"*.wav");
        e.mem.set_cstr(OGG_PATTERN, b"*.ogg");
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRING_APPEND, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_COUNT, |e, a| {
            let mut node = a[0];
            let mut count = 0u32;
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    count += 1;
                }
                node = e.mem.u32(node + 4);
            }
            count.into_ret()
        });
        for callee in [0x0040_1030, 0x0047_0470, 0x0047_02f0] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    fn string(e: &mut Engine, text: &[u8]) -> u32 {
        let p = e.mem.alloc(0x104);
        e.mem.set_cstr(p, text);
        p
    }

    /// A list of `names` (one node each).
    fn list_of(e: &mut Engine, names: &[&[u8]]) -> u32 {
        let mut next = 0;
        for name in names.iter().rev() {
            let node = e.mem.alloc(8);
            let text = string(e, name);
            e.mem.set_u32(node, text);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    #[test]
    fn fn_005e3630_without_the_flag_asks_the_archive_for_wav() {
        let mut e = sound_engine();
        e.register(ARCHIVE_FIND_00AF7400, |e, a| {
            assert_eq!(e.mem.cstr(a[0]), b"sound/x*.wav".to_vec());
            e.mem.set_cstr(a[1], b"found.wav");
            Ret {
                eax: 0x1234_5601,
                ..Ret::default()
            }
        });
        let path = string(&mut e, b"sound/x");
        assert!(e.call(0x005e_3630, &args![path]).bool());
        assert_eq!(e.mem.cstr(path), b"found.wav".to_vec());

        // Not found: the path stays.
        e.register(ARCHIVE_FIND_00AF7400, |_, _| Ret::default());
        let path = string(&mut e, b"sound/y");
        assert!(!e.call(0x005e_3630, &args![path]).bool());
        assert_eq!(e.mem.cstr(path), b"sound/y".to_vec());
    }

    #[test]
    fn fn_005e3630_with_lists_picks_the_random_entry_and_frees_the_list() {
        let mut e = sound_engine();
        e.mem.set_u8(USE_FILE_LISTS_FLAG, 1);
        let list = list_of(&mut e, &[b"a.ogg", b"b.ogg", b"c.ogg"]);
        e.register_double(FILE_FINDER_BUILD_FILE_LIST, move |e, a| {
            assert_eq!(e.mem.cstr(a[0]), b"p*.ogg".to_vec());
            assert_eq!(a[2..], [8, 0]);
            list.into_ret()
        });
        // Random 7: 7 % 3 = 1.
        e.register(0x0048_7f50, |_, _| 7u32.into_ret());
        e.register(0x00ae_9d80, |_, _| 0xabcdu32.into_ret());
        let path = string(&mut e, b"p");
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_3630, &args![path]).bool());
        assert_eq!(e.mem.cstr(path), b"b.ogg".to_vec());
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == 0x0040_1030).count(), 3);
        assert!(log.contains(&(0x0047_02f0, vec![list, 1])));
        // The hash and choice went into slot 0.
        assert_eq!(e.mem.u32(RECENT_HASHES), 0xabcd);
        assert_eq!(e.mem.u8(RECENT_CHOICES), 1);
        assert_eq!(e.mem.u8(RECENT_SLOT), 0);
    }

    #[test]
    fn fn_005e3630_redraws_a_remembered_choice_and_falls_back_to_wav() {
        let mut e = sound_engine();
        e.mem.set_u8(USE_FILE_LISTS_FLAG, 1);
        // Slot 3 remembers choice 1 for hash 0x77; the slot to write is 2.
        e.mem.set_u32(RECENT_HASHES + 12, 0x77);
        e.mem.set_u8(RECENT_CHOICES + 3, 1);
        e.mem.set_u8(RECENT_SLOT, 2);
        let list = list_of(&mut e, &[b"a.wav", b"b.wav"]);
        e.register_double(FILE_FINDER_BUILD_FILE_LIST, move |e, a| {
            if e.mem.cstr(a[0]).ends_with(b".ogg") {
                0u32.into_ret()
            } else {
                list.into_ret()
            }
        });
        e.register(0x0048_7f50, |_, _| 5u32.into_ret());
        e.register(0x00ae_9d80, |_, _| 0x77u32.into_ret());
        e.register(0x005a_00a0, |_, a| {
            assert_eq!(a[0], 2);
            0u32.into_ret()
        });
        let path = string(&mut e, b"q");
        assert!(e.call(0x005e_3630, &args![path]).bool());
        assert_eq!(e.mem.cstr(path), b"a.wav".to_vec());
        assert_eq!(e.mem.u32(RECENT_HASHES + 8), 0x77);
        assert_eq!(e.mem.u8(RECENT_CHOICES + 2), 0);
        assert_eq!(e.mem.u8(RECENT_SLOT), 2);
    }

    #[test]
    fn fn_005e3630_fails_without_a_list_or_entries() {
        let mut e = sound_engine();
        e.mem.set_u8(USE_FILE_LISTS_FLAG, 1);
        e.register(FILE_FINDER_BUILD_FILE_LIST, |_, _| Ret::default());
        let path = string(&mut e, b"r");
        assert!(!e.call(0x005e_3630, &args![path]).bool());

        // A list whose only node has no item counts 0 entries.
        let node = e.mem.alloc(8);
        e.register_double(FILE_FINDER_BUILD_FILE_LIST, move |_, _| node.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005e_3630, &args![path]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0047_02f0, vec![node, 1])));
        assert!(!log.iter().any(|c| c.0 == 0x0048_7f50));
    }

    #[test]
    fn fn_005e39b0_maps_the_flags_in_priority_order() {
        let mut e = Engine::new();
        let cases = [
            (0u32, 0u32),
            (0x10, 0x10),
            (0x610, 0x10),
            (0x600, 0x0200_0000),
            (0x400, 0x0400_0000),
            (0x1, 0),
        ];
        for (flags, want) in cases {
            assert_eq!(e.call(0x005e_39b0, &args![flags]).u32(), want);
        }
    }

    #[test]
    fn fn_005e39a0_returns_the_flag() {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.mem.set_u8(USE_FILE_LISTS_FLAG, 1);
        assert_eq!(e.call(0x005e_39a0, &args![]).u32() & 0xff, 1);
    }

    #[test]
    fn fn_005e3a00_sets_the_start_values() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(DEFAULT_TRIPLE, 11);
        e.mem.set_u32(DEFAULT_TRIPLE + 4, 22);
        e.mem.set_u32(DEFAULT_TRIPLE + 8, 33);
        let block = e.mem.alloc(0x58);
        for i in 0..0x58 {
            e.mem.set_u8(block + i, 0xcc);
        }
        e.call(0x005e_3a00, &args![block]);
        assert_eq!(e.mem.f32(block), 1.0);
        assert_eq!(
            e.mem.bytes(block + 4, 7),
            vec![0xcc, 0xcc, 5, 0xff, 100, 0, 0]
        );
        assert_eq!(e.mem.u8(block + 0xb), 0xcc);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.f32(block + 0x14), 0.0);
        assert_eq!(e.mem.f32(block + 0x18), 1.0);
        assert_eq!(e.mem.u8(block + 0x1c), 0);
        assert_eq!(e.mem.u8(block + 0x1d), 0xcc);
        assert_eq!(e.mem.f32(block + 0x28), 1.0);
        for start in [0x2c, 0x38] {
            assert_eq!(e.mem.u32(block + start), 11);
            assert_eq!(e.mem.u32(block + start + 4), 22);
            assert_eq!(e.mem.u32(block + start + 8), 33);
        }
        assert_eq!(e.mem.u32(block + 0x44), 0);
        assert_eq!(e.mem.u32(block + 0x48), 0);
        assert_eq!(e.mem.bytes(block + 0x4c, 4), vec![0, 0, 0xcc, 0xcc]);
        assert_eq!(e.mem.f32(block + 0x50), 1.0);
    }

    #[test]
    fn fn_005e3af0_builds_the_body_part() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        for callee in [
            0x0040_37b0,
            0x0048_8f50,
            0x0040_3d30,
            0x005e_3d20,
            NODE_ITEM,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xb0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_3af0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BGS_BODY_PART_VTABLE);
        assert_eq!(e.mem.u32(this + 0x44), TES_MODEL_PSA_VTABLE);
        assert_eq!(e.mem.f32(this + 0x5c), 1.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0040_37b0, vec![this + 0x4]),
                (0x0040_37b0, vec![this + 0xc]),
                (0x0040_37b0, vec![this + 0x14]),
                (0x0040_37b0, vec![this + 0x1c]),
                (0x0040_37b0, vec![this + 0x24]),
                (0x0048_8f50, vec![this + 0x2c]),
                (0x0048_8f50, vec![this + 0x44]),
                (NODE_ITEM, vec![this + 0x5c + 0x2c]),
                (NODE_ITEM, vec![this + 0x5c + 0x38]),
                (0x0040_3d30, vec![this + 0x5c, 0, 0x54]),
                (0x005e_3d20, vec![this]),
            ]
        );
    }

    #[test]
    fn fn_005e3be0_and_fn_005e3c00_return_this() {
        let mut e = Engine::new();
        e.register(0x0048_8f50, |_, _| Ret::default());
        e.register(NODE_ITEM, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        assert_eq!(e.call(0x005e_3be0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TES_MODEL_PSA_VTABLE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005e_3c00, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (NODE_ITEM, vec![this + 0x2c]),
                (NODE_ITEM, vec![this + 0x38])
            ]
        );
    }
}
