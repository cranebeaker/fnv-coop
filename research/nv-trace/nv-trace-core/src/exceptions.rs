//! What the recorder says about C++ exceptions in the real game (ADR-0013): which exception a
//! `_CxxThrowException` throws, and which catch block will take it. The recorder hooks
//! `_CxxThrowException` (`00ec7894`), `___CxxFrameHandler3` (`00ec3d33`) and
//! `___InternalCxxFrameHandler` (`00eceeb8`) in every run ([`LIST`]) and writes a line in the
//! run's `rec.log` for each throw and each catch found. Everything here reads the exe's own tables
//! through a reader, so it runs on any host.

/// The lines the recorder adds to any function list. `_CxxThrowException`: the object and the
/// `ThrowInfo` (both 0 for `throw;`). The two handlers: the exception record, the registration
/// node, the context and the dispatcher context; the second also the `FuncInfo`, the catch depth,
/// the marker node and the recursive flag.
pub const LIST: &str = "\
00ec7894 stdcall  pp        void _CxxThrowException
00ec3d33 cdecl    pppp      u32  ___CxxFrameHandler3
00eceeb8 cdecl    pppppwpw  u32  ___InternalCxxFrameHandler
";

/// The three hooked addresses.
pub const THROW: u32 = 0x00ec_7894;
pub const FRAME_HANDLER: u32 = 0x00ec_3d33;
pub const INTERNAL_HANDLER: u32 = 0x00ec_eeb8;

/// Reads what an exception needs from the running process: a word, or a C string.
pub trait Reader {
    fn word(&mut self, addr: u32) -> Option<u32>;
    fn c_string(&mut self, addr: u32) -> Option<String>;
}

/// The `TypeDescriptor` names of a `ThrowInfo`'s catchable types, most derived first.
pub fn type_chain(throw_info: u32, read: &mut impl Reader) -> Vec<String> {
    let Some(array) = read.word(throw_info.wrapping_add(0xc)) else {
        return Vec::new();
    };
    let count = read.word(array).unwrap_or(0).min(64);
    (0..count)
        .filter_map(|index| {
            let catchable = read.word(array.wrapping_add(4 + 4 * index))?;
            let descriptor = read.word(catchable.wrapping_add(4))?;
            read.c_string(descriptor.wrapping_add(8))
        })
        .collect()
}

/// A decorated class name as C++ writes it: `.?AVfailure@ios_base@std@@` is
/// `std::ios_base::failure`.
pub fn readable_type(name: &str) -> String {
    match name
        .strip_prefix(".?AV")
        .or_else(|| name.strip_prefix(".?AU"))
    {
        Some(rest) => rest
            .trim_end_matches('@')
            .split('@')
            .rev()
            .collect::<Vec<_>>()
            .join("::"),
        None => name.to_string(),
    }
}

/// The object's message, for the classes the exe throws: `runtime_error` and what derives from it
/// keep a `std::string` at `+0xc` (its text inline at `+0x10`, or from a capacity of 0x10 on, the
/// pointer there; the capacity at `+0x24`), `std::exception` a text pointer at `+4`.
pub fn message(object: u32, chain: &[String], read: &mut impl Reader) -> Option<String> {
    if chain.iter().any(|n| n == ".?AVruntime_error@std@@") {
        let text = if read.word(object.wrapping_add(0x24))? >= 0x10 {
            read.word(object.wrapping_add(0x10))?
        } else {
            object.wrapping_add(0x10)
        };
        return read.c_string(text);
    }
    if chain.iter().any(|n| n == ".?AVexception@std@@") {
        let text = read.word(object.wrapping_add(4))?;
        return if text == 0 {
            None
        } else {
            read.c_string(text)
        };
    }
    None
}

/// A catch block a frame has for an exception.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Catch {
    /// The catch funclet's address.
    pub block: u32,
    /// The type it names (`None`: `catch(...)`).
    pub type_name: Option<String>,
    /// The function that has it, when it is one of the exe's 153 with a try block.
    pub function: Option<u32>,
}

/// The catch block of a frame for an exception of `chain`, as the exe's handler search goes: the
/// try blocks whose states hold `state`, in the table's order, and in each the first handler
/// naming a type in the chain or none. `FuncInfo`: `+0xc` the try block count, `+0x10` the map
/// (0x14 bytes each: low state, high state, catch high, handler count, handler array). Handlers:
/// 0x10 bytes (`+4` the `TypeDescriptor` or 0, `+0xc` the funclet). The type's const and reference
/// flags are not looked at: every type the exe catches by name is `std::exception&`.
pub fn find_catch(
    func_info: u32,
    state: i32,
    chain: &[String],
    read: &mut impl Reader,
) -> Option<Catch> {
    let tries = read.word(func_info.wrapping_add(0xc))?.min(256);
    let map = read.word(func_info.wrapping_add(0x10))?;
    for t in 0..tries {
        let entry = map.wrapping_add(0x14 * t);
        let low = read.word(entry)? as i32;
        let high = read.word(entry.wrapping_add(4))? as i32;
        if state < low || state > high {
            continue;
        }
        let handlers = read.word(entry.wrapping_add(0xc))?.min(64);
        let array = read.word(entry.wrapping_add(0x10))?;
        for h in 0..handlers {
            let handler = array.wrapping_add(0x10 * h);
            let descriptor = read.word(handler.wrapping_add(4))?;
            let name = if descriptor == 0 {
                None
            } else {
                read.c_string(descriptor.wrapping_add(8))
                    .filter(|n| !n.is_empty())
            };
            if name.as_ref().is_none_or(|n| chain.contains(n)) {
                return Some(Catch {
                    block: read.word(handler.wrapping_add(0xc))?,
                    type_name: name,
                    function: catching_function(func_info),
                });
            }
        }
    }
    None
}

/// The function a `FuncInfo` with a try block belongs to (the exe's tables, read 2026-11-05).
pub fn catching_function(func_info: u32) -> Option<u32> {
    CATCHERS
        .binary_search_by_key(&func_info, |&(f, _)| f)
        .ok()
        .map(|i| CATCHERS[i].1)
}

/// The `FuncInfo`s with try blocks and their functions, sorted.
const CATCHERS: [(u32, u32); 153] = [
    (0x0114_0d90, 0x0044_aa10),
    (0x0114_c3c8, 0x0060_b4d0),
    (0x0114_efc4, 0x0064_df50),
    (0x0114_f044, 0x0064_e710),
    (0x0114_f09c, 0x0064_e9c0),
    (0x0114_fe6c, 0x0065_fb10),
    (0x0114_fecc, 0x0065_fc10),
    (0x0114_ff24, 0x0066_0070),
    (0x0114_ffa8, 0x0066_0510),
    (0x0115_0000, 0x0066_05d0),
    (0x0116_a630, 0x00aa_e3a0),
    (0x0116_a7e0, 0x00aa_f8a0),
    (0x0116_a840, 0x00aa_fa80),
    (0x0116_a970, 0x00ab_0fd0),
    (0x0116_a9dc, 0x00ab_1800),
    (0x0116_aaa4, 0x00ab_23c0),
    (0x0116_ab04, 0x00ab_2490),
    (0x0116_ac1c, 0x00ab_3370),
    (0x0116_ac88, 0x00ab_3a40),
    (0x0116_ad40, 0x00ab_45f0),
    (0x0116_b16c, 0x00ab_bfc0),
    (0x0116_b1fc, 0x00ab_c320),
    (0x0116_b288, 0x00ab_ca90),
    (0x0116_b314, 0x00ab_d2e0),
    (0x0116_b3a0, 0x00ab_db10),
    (0x0116_b40c, 0x00ab_e310),
    (0x0116_b4a8, 0x00ab_eb00),
    (0x0116_b544, 0x00ab_f210),
    (0x0116_b5e0, 0x00ab_fa90),
    (0x0116_b6c4, 0x00ac_0640),
    (0x0116_b724, 0x00ac_08b0),
    (0x0116_b7a8, 0x00ac_20d0),
    (0x0116_b800, 0x00ac_21b0),
    (0x0116_b858, 0x00ac_2290),
    (0x0116_b8b0, 0x00ac_2370),
    (0x0116_b910, 0x00ac_2440),
    (0x0116_b970, 0x00ac_2540),
    (0x0116_b9d0, 0x00ac_2650),
    (0x0116_ba28, 0x00ac_3370),
    (0x0116_baac, 0x00ac_3820),
    (0x0116_bb04, 0x00ac_3900),
    (0x0116_bb5c, 0x00ac_39e0),
    (0x0116_bbbc, 0x00ac_3ac0),
    (0x0116_bc1c, 0x00ac_3bd0),
    (0x0116_bc7c, 0x00ac_3ce0),
    (0x0116_bfac, 0x00ac_5f00),
    (0x0116_c380, 0x00ac_9df0),
    (0x0116_c494, 0x00ac_a560),
    (0x0116_c4fc, 0x00ac_a7a0),
    (0x0116_c808, 0x00ac_e390),
    (0x0116_c99c, 0x00ad_03e0),
    (0x0116_ca0c, 0x00ad_07c0),
    (0x0116_cac8, 0x00ad_10e0),
    (0x0116_cb50, 0x00ad_1960),
    (0x0116_cb94, 0x00ad_1c10),
    (0x0116_cc6c, 0x00ad_26f0),
    (0x0116_cd68, 0x00ad_3150),
    (0x0116_cfa8, 0x00ad_43a0),
    (0x0116_d038, 0x00ad_47f0),
    (0x0116_d0a0, 0x00ad_4ba0),
    (0x0116_d19c, 0x00ad_5770),
    (0x0116_d1f4, 0x00ad_5b00),
    (0x0116_d254, 0x00ad_5bd0),
    (0x0116_d2bc, 0x00ad_5cc0),
    (0x0116_d31c, 0x00ad_66e0),
    (0x0116_d374, 0x00ad_68c0),
    (0x0116_d458, 0x00ad_6d30),
    (0x0116_d4c0, 0x00ad_6e20),
    (0x0116_f688, 0x00b0_2ef0),
    (0x0116_f768, 0x00b0_36d0),
    (0x0116_f804, 0x00b0_44a0),
    (0x0116_f8b8, 0x00b0_5210),
    (0x0116_f948, 0x00b0_5420),
    (0x0116_f9e0, 0x00b0_59e0),
    (0x0116_faf0, 0x00b0_6280),
    (0x0116_fb68, 0x00b0_6440),
    (0x0116_fbe0, 0x00b0_6660),
    (0x0116_fc58, 0x00b0_6810),
    (0x0116_fcd0, 0x00b0_69c0),
    (0x0116_fd48, 0x00b0_6c10),
    (0x0116_fdc0, 0x00b0_7060),
    (0x0116_fe38, 0x00b0_71f0),
    (0x0116_feb0, 0x00b0_7510),
    (0x0116_ff28, 0x00b0_76e0),
    (0x0116_ffa0, 0x00b0_7900),
    (0x0117_0018, 0x00b0_7b00),
    (0x0117_0090, 0x00b0_7db0),
    (0x0117_0108, 0x00b0_8050),
    (0x0117_0180, 0x00b0_8200),
    (0x0117_01f8, 0x00b0_88d0),
    (0x0117_0270, 0x00b0_8af0),
    (0x0117_02e8, 0x00b0_8cc0),
    (0x0117_0360, 0x00b0_8ec0),
    (0x0117_0638, 0x00b0_a7a0),
    (0x0117_06b0, 0x00b0_ab20),
    (0x0117_0728, 0x00b0_cb10),
    (0x0117_07a0, 0x00b0_d040),
    (0x0117_08b0, 0x00b0_ecf0),
    (0x0117_093c, 0x00b0_f410),
    (0x0117_09c8, 0x00b0_fc00),
    (0x0117_0a48, 0x00b1_0990),
    (0x0117_0aa0, 0x00b1_0c20),
    (0x0117_0c70, 0x00b1_57e0),
    (0x0117_0cfc, 0x00b1_6070),
    (0x0117_0d7c, 0x00b1_6cd0),
    (0x0117_0dd4, 0x00b1_7b40),
    (0x0117_0e2c, 0x00b1_7c20),
    (0x0117_12a0, 0x00b1_ecf0),
    (0x0117_1368, 0x00b2_00f0),
    (0x0117_13cc, 0x00b2_0360),
    (0x0117_1464, 0x00b2_0ce0),
    (0x0117_1518, 0x00b2_1560),
    (0x0117_15d4, 0x00b2_1e00),
    (0x0117_166c, 0x00b2_2700),
    (0x0117_173c, 0x00b2_2ff0),
    (0x0117_179c, 0x00b2_31f0),
    (0x0117_17f4, 0x00b2_3f60),
    (0x0117_185c, 0x00b2_4180),
    (0x0117_18c4, 0x00b2_4290),
    (0x0117_1924, 0x00b2_43a0),
    (0x0117_197c, 0x00b2_44a0),
    (0x0117_19e4, 0x00b2_4580),
    (0x0117_1a68, 0x00b2_5a90),
    (0x0117_1ac8, 0x00b2_5ba0),
    (0x0117_1b10, 0x00b2_5ca0),
    (0x0117_1b88, 0x00b2_5e20),
    (0x0117_1c20, 0x00b2_5fa0),
    (0x0117_1d7c, 0x00b2_6db0),
    (0x0117_22ac, 0x00b2_e570),
    (0x0117_260c, 0x00b3_2600),
    (0x0117_2664, 0x00b3_2780),
    (0x0117_26f0, 0x00b3_2ce0),
    (0x0117_27c8, 0x00b3_3bd0),
    (0x0117_287c, 0x00b3_4450),
    (0x0117_2950, 0x00b3_4e10),
    (0x0117_29d8, 0x00b3_55d0),
    (0x0117_2a38, 0x00b3_57d0),
    (0x0117_2aa0, 0x00b3_60a0),
    (0x0117_2b08, 0x00b3_61b0),
    (0x0117_2b70, 0x00b3_68d0),
    (0x0117_2bd8, 0x00b3_69f0),
    (0x0117_2c20, 0x00b3_6bf0),
    (0x0117_2c98, 0x00b3_6d70),
    (0x0117_306c, 0x00b3_a0d0),
    (0x0117_3124, 0x00b3_b330),
    (0x0117_317c, 0x00b3_b5f0),
    (0x0117_3200, 0x00b3_b880),
    (0x0117_34d0, 0x00b3_dbd0),
    (0x0117_3588, 0x00b3_e5b0),
    (0x0117_38c4, 0x00b4_2180),
    (0x0117_3970, 0x00b4_2f40),
    (0x0117_39c8, 0x00b4_3230),
    (0x0118_0090, 0x00ec_e5fc),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    struct Memory {
        words: HashMap<u32, u32>,
        strings: HashMap<u32, String>,
    }

    impl Reader for Memory {
        fn word(&mut self, addr: u32) -> Option<u32> {
            self.words.get(&addr).copied()
        }
        fn c_string(&mut self, addr: u32) -> Option<String> {
            self.strings.get(&addr).cloned()
        }
    }

    /// A `ThrowInfo` at 0x1000 for `chain`.
    fn throw_info(m: &mut Memory, chain: &[&str]) {
        m.words.insert(0x100c, 0x2000);
        m.words.insert(0x2000, chain.len() as u32);
        for (i, name) in chain.iter().enumerate() {
            let i = i as u32;
            m.words.insert(0x2004 + 4 * i, 0x3000 + 0x100 * i);
            m.words.insert(0x3004 + 0x100 * i, 0x4000 + 0x100 * i);
            m.strings.insert(0x4008 + 0x100 * i, name.to_string());
        }
    }

    /// A `FuncInfo` at 0x5000 with one try block (states 1 to 2) with a `std::exception&` handler
    /// and a `catch(...)`.
    fn func_info(m: &mut Memory) {
        for (at, word) in [
            (0x500c, 1),
            (0x5010, 0x6000),
            (0x6000, 1),
            (0x6004, 2),
            (0x600c, 2),
            (0x6010, 0x7000),
            (0x7004, 0x8000),
            (0x700c, 0xaaaa),
            (0x7014, 0),
            (0x701c, 0xbbbb),
        ] {
            m.words.insert(at, word);
        }
        m.strings.insert(0x8008, ".?AVexception@std@@".into());
    }

    #[test]
    fn the_chain_comes_most_derived_first_and_reads_as_cpp() {
        let mut m = Memory::default();
        throw_info(
            &mut m,
            &[".?AVIdvFileError@@", ".?AVruntime_error@std@@", ".?AVexception@std@@"],
        );
        let chain = type_chain(0x1000, &mut m);
        assert_eq!(chain[0], ".?AVIdvFileError@@");
        assert_eq!(readable_type(&chain[0]), "IdvFileError");
        assert_eq!(readable_type(&chain[1]), "std::runtime_error");
        assert_eq!(
            readable_type(".?AVfailure@ios_base@std@@"),
            "std::ios_base::failure"
        );
        assert_eq!(readable_type("something else"), "something else");
        // An unreadable ThrowInfo is an empty chain, not a fault.
        assert!(type_chain(0x9000, &mut m).is_empty());
    }

    #[test]
    fn the_message_follows_the_class() {
        let mut m = Memory::default();
        let plain = vec![".?AVexception@std@@".to_string()];
        m.words.insert(0x104, 0x900);
        m.strings.insert(0x900, "reads short".into());
        assert_eq!(message(0x100, &plain, &mut m).as_deref(), Some("reads short"));
        // A runtime_error: inline below a capacity of 0x10, through the pointer from there on.
        let derived = vec![".?AVruntime_error@std@@".to_string(), plain[0].clone()];
        m.words.insert(0x224, 0xf);
        m.strings.insert(0x210, "short".into());
        assert_eq!(message(0x200, &derived, &mut m).as_deref(), Some("short"));
        m.words.insert(0x224, 0x10);
        m.words.insert(0x210, 0x950);
        m.strings.insert(0x950, "a longer message".into());
        assert_eq!(
            message(0x200, &derived, &mut m).as_deref(),
            Some("a longer message")
        );
        assert_eq!(message(0x300, &[".?AVOther@@".into()], &mut m), None);
    }

    #[test]
    fn the_catch_is_found_by_state_and_type() {
        let mut m = Memory::default();
        func_info(&mut m);
        let std_chain = vec![".?AVbad_alloc@std@@".to_string(), ".?AVexception@std@@".to_string()];
        let found = find_catch(0x5000, 1, &std_chain, &mut m).unwrap();
        assert_eq!(found.block, 0xaaaa);
        assert_eq!(found.type_name.as_deref(), Some(".?AVexception@std@@"));
        // Another type goes to catch(...).
        let found = find_catch(0x5000, 2, &[".?AVOther@@".to_string()], &mut m).unwrap();
        assert_eq!(found.block, 0xbbbb);
        assert_eq!(found.type_name, None);
        // Outside the try states: no catch here.
        assert_eq!(find_catch(0x5000, 0, &std_chain, &mut m), None);
        assert_eq!(find_catch(0x5000, 3, &std_chain, &mut m), None);
    }

    #[test]
    fn the_table_names_the_catching_functions() {
        assert!(CATCHERS.windows(2).all(|w| w[0].0 < w[1].0), "sorted");
        // `LoadTree`'s and the constructor's.
        assert_eq!(catching_function(0x0116_f948), Some(0x00b0_5420));
        assert_eq!(catching_function(0x0116_f688), Some(0x00b0_2ef0));
        assert_eq!(catching_function(0x1234), None);
    }

    #[test]
    fn the_list_parses_and_the_addresses_are_its_own() {
        let specs = crate::list::parse(LIST).unwrap();
        assert_eq!(
            specs.iter().map(|s| s.addr).collect::<Vec<_>>(),
            [THROW, FRAME_HANDLER, INTERNAL_HANDLER]
        );
        assert_eq!(specs[0].stack_words(), 2);
        assert_eq!(specs[2].stack_words(), 8);
    }
}
