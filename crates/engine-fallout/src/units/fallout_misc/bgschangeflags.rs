//! `fallout/misc/saveload/bgschangeflags.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008c71b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores the word it is given at `this` and returns `this` (`thiscall`,
/// `RET 4`). The map names it `BGSChangeFlags::IsForcedChange`, but the
/// linker folded identical code here: the body is a one-word constructor
/// (Ghidra matches it to `std::locale::id::id`). `TESForm::ForceChange`
/// (`00484ba2`) builds its change-flags word with it, so every form
/// constructor that flags a change (the quest's enable setter `0060c9c0`)
/// runs it.
pub fn fn_008c71b0(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), value);
    this
}

/// `"Bad Flag Name"` (`0107de00`), the answer for a flag the form type does not name.
const BAD_FLAG_NAME: u32 = 0x0107_de00;

/// The readable name when `pretty` is set, otherwise the `CHANGE_..` one.
fn pick(pretty: u8, readable: u32, raw: u32) -> Option<u32> {
    Some(if pretty != 0 { readable } else { raw })
}

// Translated from 0083fef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSChangeFlags::GetFlagName` (Xbox PDB; cdecl: flag, form type, byte): the name of a
/// change flag as a string literal's address, readable (`"Moved"`) with the byte set and the
/// constant's name (`"CHANGE_REFR_MOVE"`) without. Flag 1 is the form flags for every type;
/// the other flags are named per form type (the compiler's jump table on `type - 7`, 0 to
/// 0x62, translated as the match arms; other types take the default arm: the base object's
/// value and name flags, the talking activator's speaker (type 0x16) and the book's skill
/// (type 0x19)). The exe tests the flag against signed ranges before each comparison; only
/// equal values answer, so the arms list the values. An unnamed flag answers
/// `"Bad Flag Name"`.
pub fn bgs_change_flags_get_flag_name(
    _e: &mut Engine,
    flag: u32,
    form_type: u32,
    pretty: u8,
) -> u32 {
    if flag == 1 {
        return pick(pretty, 0x0104_4aec, 0x0107_e8d0).unwrap_or(BAD_FLAG_NAME);
    }
    let name = match form_type {
        7 => match flag {
            2 => pick(pretty, 0x0107_dfb0, 0x0107_df98),
            _ => None,
        },
        8 => match flag {
            0x8000_0000 => pick(pretty, 0x0107_e174, 0x0107_e158),
            2 => pick(pretty, 0x0107_e1d4, 0x0107_e1bc),
            4 => pick(pretty, 0x0107_e1a8, 0x0107_e18c),
            _ => None,
        },
        0x2a | 0x2b => match flag {
            2 => pick(pretty, 0x0107_e0f8, 0x0107_e0e0),
            4 => pick(pretty, 0x0102_0e5c, 0x0107_e0c0),
            8 => pick(pretty, 0x0107_e0b8, 0x0107_e09c),
            0x10 => pick(pretty, 0x0107_e090, 0x0107_e074),
            0x20 => pick(pretty, 0x0107_e068, 0x0107_e04c),
            _ if form_type == 0x2a => match flag {
                0x800 => pick(pretty, 0x0105_c2c8, 0x0107_e008),
                0x200 => pick(pretty, 0x0107_e040, 0x0107_e02c),
                0x400 => pick(pretty, 0x0101_feec, 0x0107_e018),
                0x0100_0000 => pick(pretty, 0x0105_c30c, 0x0107_dfe4),
                0x0200_0000 => pick(pretty, 0x0101_fef4, 0x0107_dff8),
                _ => None,
            },
            0x200 if form_type == 0x2b => pick(pretty, 0x0107_dfd4, 0x0107_dfbc),
            _ => None,
        },
        0x2c | 0x2d | 0x34 => match flag {
            0x8000_0000 => pick(pretty, 0x0107_df0c, 0x0107_dee8),
            _ => None,
        },
        0x31 => match flag {
            0x8000_0000 => pick(pretty, 0x0107_e2f4, 0x0107_e2e0),
            _ => None,
        },
        0x39 => match flag {
            8 => pick(pretty, 0x0107_e1fc, 0x0107_e1e4),
            0x8000_0000 => pick(pretty, 0x0107_e2d4, 0x0107_e2bc),
            2 => pick(pretty, 0x0104_4aec, 0x0107_e22c),
            4 => pick(pretty, 0x0107_e220, 0x0107_e208),
            0x1000_0000 => pick(pretty, 0x0107_e25c, 0x0107_e240),
            0x2000_0000 => pick(pretty, 0x0107_e288, 0x0107_e26c),
            0x4000_0000 => pick(pretty, 0x0107_e2b0, 0x0107_e298),
            _ => None,
        },
        0x3a..=0x40 | 0x69 => {
            let reference = match flag {
                0x40 => pick(pretty, 0x0107_e71c, 0x0107_e700),
                8 => pick(pretty, 0x0107_e880, 0x0107_e864),
                0x8000_0000 => pick(pretty, 0x0107_e7b0, 0x0107_e794),
                2 => pick(pretty, 0x0107_e8c8, 0x0107_e8b4),
                4 => pick(pretty, 0x0107_e8a8, 0x0107_e890),
                0x10 => pick(pretty, 0x0107_e85c, 0x0107_e848),
                0x20 => pick(pretty, 0x0105_ac24, 0x0107_e830),
                0x1000_0000 => pick(pretty, 0x0107_e6f4, 0x0107_e6dc),
                0x0400_0000 => pick(pretty, 0x0107_e7e8, 0x0107_e7c0),
                0x0800_0000 => pick(pretty, 0x0107_e81c, 0x0107_e7fc),
                0x2000_0000 => pick(pretty, 0x0107_e750, 0x0107_e72c),
                0x4000_0000 => pick(pretty, 0x0107_e780, 0x0107_e760),
                _ => None,
            };
            if reference.is_some() {
                reference
            } else if !(0x3b..=0x3c).contains(&form_type) {
                let object = match flag {
                    0x1000 => pick(pretty, 0x0107_e490, 0x0107_e474),
                    0x400 => pick(pretty, 0x0107_e4e4, 0x0107_e4c4),
                    0x800 => pick(pretty, 0x0107_e4b8, 0x0107_e49c),
                    0x0040_0000 => pick(pretty, 0x0107_e438, 0x0107_e414),
                    0x0080_0000 => pick(pretty, 0x0107_e468, 0x0107_e44c),
                    _ => None,
                };
                // The exe then tests for a door (type 0x1c) with flag 0x20000, which no type
                // of this arm is: the branch is kept but can never answer.
                if object.is_none() && form_type == 0x1c && flag == 0x0002_0000 {
                    pick(pretty, 0x0107_e404, 0x0107_e3e8)
                } else {
                    object
                }
            } else {
                match flag {
                    0x0008_0000 => pick(pretty, 0x0107_e56c, 0x0107_e548),
                    0x1000 => pick(pretty, 0x0107_e670, 0x0107_e648),
                    0x400 => pick(pretty, 0x0107_e6d0, 0x0107_e6b8),
                    0x800 => pick(pretty, 0x0107_e6a4, 0x0107_e684),
                    0x0002_0000 => pick(pretty, 0x0102_0aac, 0x0107_e4f4),
                    0x0004_0000 => pick(pretty, 0x0107_e538, 0x0107_e51c),
                    0x0040_0000 => pick(pretty, 0x0107_e5fc, 0x0107_e5dc),
                    0x0010_0000 => pick(pretty, 0x0107_e598, 0x0107_e57c),
                    0x0020_0000 => pick(pretty, 0x0107_e5c8, 0x0107_e5a8),
                    0x0080_0000 => pick(pretty, 0x0107_e634, 0x0107_e610),
                    _ => None,
                }
            }
        }
        0x46 => match flag {
            0x8000_0000 => pick(pretty, 0x0107_e318, 0x0107_e300),
            _ => None,
        },
        0x47 => match flag {
            4 => pick(pretty, 0x0107_e3b4, 0x0107_e398),
            0x8000_0000 => pick(pretty, 0x0107_e388, 0x0107_e374),
            2 => pick(pretty, 0x0107_e3dc, 0x0107_e3c8),
            0x2000_0000 => pick(pretty, 0x0107_e33c, 0x0107_e324),
            0x4000_0000 => pick(pretty, 0x0107_e364, 0x0107_e350),
            _ => None,
        },
        0x49 => match flag {
            0x8000_0000 => pick(pretty, 0x0107_e148, 0x0107_e12c),
            0x4000_0000 => pick(pretty, 0x0107_e11c, 0x0107_e104),
            _ => None,
        },
        0x4e => match flag {
            0x8000_0000 => pick(pretty, 0x0107_dedc, 0x0107_dec4),
            _ => None,
        },
        0x55 => match flag {
            0x8000_0000 => pick(pretty, 0x0107_df38, 0x0107_df1c),
            _ => None,
        },
        0x61 => match flag {
            0x8000_0000 => pick(pretty, 0x0107_df64, 0x0107_df44),
            2 => pick(pretty, 0x0107_df8c, 0x0107_df70),
            _ => None,
        },
        _ => match (flag, form_type) {
            (2, _) => pick(pretty, 0x0107_deb4, 0x0107_de98),
            (4, _) => pick(pretty, 0x0107_de84, 0x0107_de68),
            (0x0080_0000, 0x16) => pick(pretty, 0x0107_de34, 0x0107_de10),
            (0x20, 0x19) => pick(pretty, 0x0107_de58, 0x0107_de3c),
            _ => None,
        },
    };
    name.unwrap_or(BAD_FLAG_NAME)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008c71b0, fn_008c71b0(Ptr, u32) -> Ptr),
        entry!(
            0x0083_fef0,
            bgs_change_flags_get_flag_name(u32, u32, u8) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_word_and_returns_this() {
        let mut e = Engine::new();
        let p = e.mem.alloc(8);
        e.mem.set_u32(p + 4, 0x5555_5555);
        assert_eq!(e.call(0x008c71b0, &[p, 0x4000_0002]).u32(), p);
        assert_eq!(e.mem.u32(p), 0x4000_0002);
        assert_eq!(e.mem.u32(p + 4), 0x5555_5555);
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    fn name(flag: u32, form_type: u32, pretty: u32) -> u32 {
        let mut e = Engine::new();
        e.call(0x0083_fef0, &args![flag, form_type, pretty]).u32()
    }

    #[test]
    fn get_flag_name_names_the_form_flags_for_every_type() {
        assert_eq!(name(1, 0x3a, 1), 0x0104_4aec);
        assert_eq!(name(1, 0x99, 0), 0x0107_e8d0);
        // Only the low byte of the third word counts.
        assert_eq!(name(1, 7, 0x100), 0x0107_e8d0);
    }

    #[test]
    fn get_flag_name_names_reference_object_and_actor_flags() {
        for form_type in [0x3a, 0x3b, 0x40, 0x69] {
            assert_eq!(name(2, form_type, 1), 0x0107_e8c8);
            assert_eq!(name(2, form_type, 0), 0x0107_e8b4);
        }
        // The object extras for a non-actor reference, the actor ones for 0x3b and 0x3c.
        assert_eq!(name(0x1000, 0x3a, 1), 0x0107_e490);
        assert_eq!(name(0x0080_0000, 0x3c, 0), 0x0107_e610);
        assert_eq!(name(0x0080_0000, 0x3a, 1), name(0x0080_0000, 0x3d, 1));
        assert_ne!(name(0x0080_0000, 0x3a, 1), name(0x0080_0000, 0x3b, 1));
        // The door teleport flag is never named for a reference type.
        assert_eq!(name(0x0002_0000, 0x3a, 1), BAD_FLAG_NAME);
    }

    #[test]
    fn get_flag_name_names_npc_and_creature_flags() {
        assert_eq!(name(0x200, 0x2a, 1), 0x0107_e040);
        assert_eq!(name(0x200, 0x2b, 0), 0x0107_dfbc);
        assert_eq!(name(0x0100_0000, 0x2b, 1), BAD_FLAG_NAME);
    }

    #[test]
    fn get_flag_name_default_arm_and_bad_names() {
        // Types outside the table and types the table sends to the default.
        for form_type in [0, 6, 0x10, 0x16, 0x6a] {
            assert_eq!(name(2, form_type, 1), 0x0107_deb4);
            assert_eq!(name(2, form_type, 0), 0x0107_de98);
        }
        assert_eq!(name(0x0080_0000, 0x16, 1), 0x0107_de34);
        assert_eq!(name(0x0080_0000, 0x17, 1), BAD_FLAG_NAME);
        assert_eq!(name(0x20, 0x19, 0), 0x0107_de3c);
        // A table type with an unnamed flag does not fall back to the default names.
        assert_eq!(name(2, 0x31, 1), BAD_FLAG_NAME);
        assert_eq!(name(0x8000_0000, 0x31, 1), 0x0107_e2f4);
        assert_eq!(name(4, 0x2b, 1), 0x0102_0e5c);
    }
}
