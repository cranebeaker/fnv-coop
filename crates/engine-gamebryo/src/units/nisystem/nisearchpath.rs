//! `NiSystem/nisearchpath.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSearchPath::m_acFilePath` (Xbox PDB): the 0x104-byte file path at +8 (the same on PC).
const FILE_PATH: u32 = 0x8;
/// `_strncpy_s` (`00ec8d5f`, `cdecl`: destination, its size, source, count) and
/// `NiPath::Standardize` (`00aa2a00`, `cdecl`: the path).
const STRNCPY_S: u32 = 0x00ec_8d5f;
const PATH_STANDARDIZE: u32 = 0x00aa_2a00;

// Translated from 00aa2540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSearchPath::SetFilePath` (Xbox PDB), `thiscall`, one stack word (the path). A path that
/// is not null and not empty is copied into the file path at +8 (`_strncpy_s` with size 0x104
/// and count 0x103) and standardized there (`NiPath::Standardize`); otherwise the file path
/// is made empty.
pub fn ni_search_path_set_file_path(e: &mut Engine, this: Ptr, path: Ptr) {
    let path = path.addr();
    let file_path = this.addr().wrapping_add(FILE_PATH);
    if path != 0 && e.mem.u8(path) != 0 {
        e.call(STRNCPY_S, &args![file_path, 0x104u32, path, 0x103u32]);
        e.call(PATH_STANDARDIZE, &args![file_path]);
        return;
    }
    e.mem.set_u8(file_path, 0);
}

// Translated from 00aa2500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSearchPath::Reset` (Xbox PDB), `thiscall`, no stack words: sets
/// `m_uiNextPath` (+4) to 0.
pub fn ni_search_path_reset(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
}

/// `NiFilename::NiFilename` (`00aa27d0`, `thiscall`, the path).
const NI_FILENAME_CONSTRUCT: u32 = 0x00aa_27d0;
/// `00aa29d0` (`thiscall` on an `NiFilename`, the buffer and its size): joins
/// the parts into the buffer.
const NI_FILENAME_JOIN: u32 = 0x00aa_29d0;
/// CRT `_strcpy_s` (`00ec65a6`, `cdecl`: destination, its size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `""` in `.rdata` (`01011584`).
const EMPTY_TEXT: u32 = 0x0101_1584;
/// `NiSearchPath::ms_acDefPath` (Xbox PDB), `011f6120`.
const DEFAULT_PATH: u32 = 0x011f_6120;

// Translated from 00aa2580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aa2580 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// `NiSearchPath::GetNextSearchPath` (Xbox PDB), `thiscall`, `RET 8` (the
/// output buffer and its size). Splits `m_acFilePath` (+8) into an
/// `NiFilename` on the stack, then by `m_uiNextPath` (+4): 0 keeps it; 1
/// empties the drive and the directory; 2 empties the drive, takes
/// `m_acReferencePath` (+0x10c) as the directory and the file path's own
/// directory (a second split) as the subdirectory; 3 is 2 with an empty
/// subdirectory; 4 (only when `ms_acDefPath` is not empty) empties the drive and
/// takes `ms_acDefPath` as the directory. Any other value, or 4 with an empty
/// default path, answers false. Otherwise the parts are joined into the buffer
/// (`00aa29d0`), standardized (`NiPath::Standardize`), `m_uiNextPath` grows by
/// one and the answer is true. All copies go through `_strcpy_s`.
pub fn ni_search_path_get_next_search_path(e: &mut Engine, this: Ptr, out: Ptr, size: u32) -> bool {
    let t = this.addr();
    e.with_stack(0x808, |e, frame| {
        let first = frame.addr();
        let second = first + 0x404;
        e.call(NI_FILENAME_CONSTRUCT, &args![first, t + 8]);
        match e.mem.u32(t + 4) {
            0 => {}
            1 => {
                e.call(STRCPY_S, &args![first + 0x100, 3u32, EMPTY_TEXT]);
                e.call(STRCPY_S, &args![first, 0x100u32, EMPTY_TEXT]);
            }
            2 => {
                e.call(STRCPY_S, &args![first + 0x100, 3u32, EMPTY_TEXT]);
                e.call(STRCPY_S, &args![first, 0x100u32, t + 0x10c]);
                e.call(NI_FILENAME_CONSTRUCT, &args![second, t + 8]);
                e.call(STRCPY_S, &args![first + 0x303, 0x100u32, second]);
            }
            3 => {
                e.call(STRCPY_S, &args![first + 0x100, 3u32, EMPTY_TEXT]);
                e.call(STRCPY_S, &args![first, 0x100u32, t + 0x10c]);
                e.call(STRCPY_S, &args![first + 0x303, 0x100u32, EMPTY_TEXT]);
            }
            4 => {
                if e.mem.u8(DEFAULT_PATH) == 0 {
                    return false;
                }
                e.call(STRCPY_S, &args![first + 0x100, 3u32, EMPTY_TEXT]);
                e.call(STRCPY_S, &args![first, 0x100u32, DEFAULT_PATH]);
            }
            _ => return false,
        }
        e.call(NI_FILENAME_JOIN, &args![first, out, size]);
        e.call(PATH_STANDARDIZE, &args![out]);
        let next = e.mem.u32(t + 4).wrapping_add(1);
        e.mem.set_u32(t + 4, next);
        true
    })
}

/// The `NiSearchPath` vtable (`010a2554`, RTTI `.?AVNiSearchPath@@`).
const NI_SEARCH_PATH_VTABLE: u32 = 0x010a_2554;
/// The memory manager object (`011f6238`), the `this` of its deallocate.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa4060` (`thiscall` on the memory manager, one word): its deallocate.
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00aa2510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSearchPath::scalar deleting destructor` (Xbox PDB), `thiscall`,
/// `RET 4` (the flags): stores the `NiSearchPath` vtable (the destructor body
/// has nothing else) and, when bit 0 of the flags is set, gives `this` back
/// to the memory manager (`00aa4060`). Answers `this`.
pub fn ni_search_path_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_SEARCH_PATH_VTABLE);
    if flags & 1 != 0 {
        e.call(
            MEMORY_MANAGER_DEALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, this],
        );
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_2540, ni_search_path_set_file_path(Ptr, Ptr)),
        entry!(0x00aa_2500, ni_search_path_reset(Ptr)),
        entry!(0x00aa_2580, ni_search_path_get_next_search_path(Ptr, Ptr, u32) -> bool),
        entry!(
            0x00aa_2510,
            ni_search_path_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(STRNCPY_S, |_, _| Ret::default());
        e.register(PATH_STANDARDIZE, |_, _| Ret::default());
        let this = e.mem.alloc(0x210);
        e.mem.set_u8(this + FILE_PATH, b'x');
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn ni_search_path_set_file_path_copies_and_standardizes_a_path() {
        let (mut e, this) = engine();
        let path = e.mem.alloc(8);
        e.mem.write(path, b"a\\b\0");
        e.call(0x00aa_2540, &args![this, path]);
        assert_eq!(
            calls(&e, STRNCPY_S),
            vec![vec![this + 8, 0x104, path, 0x103]]
        );
        assert_eq!(calls(&e, PATH_STANDARDIZE), vec![vec![this + 8]]);
        assert_eq!(e.mem.u8(this + FILE_PATH), b'x');
    }

    #[test]
    fn ni_search_path_set_file_path_empties_for_a_null_or_empty_path() {
        let (mut e, this) = engine();
        e.call(0x00aa_2540, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + FILE_PATH), 0);
        e.mem.set_u8(this + FILE_PATH, b'x');
        let empty = e.mem.alloc(4);
        e.mem.set_u8(empty, 0);
        e.call(0x00aa_2540, &args![this, empty]);
        assert_eq!(e.mem.u8(this + FILE_PATH), 0);
        assert!(calls(&e, STRNCPY_S).is_empty());
        assert!(calls(&e, PATH_STANDARDIZE).is_empty());
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn ni_search_path_reset_clears_the_index() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this + 4, 3);
        e.call(0x00aa_2500, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    type Calls = Vec<(u32, Vec<u32>)>;

    /// Runs `GetNextSearchPath` with `next` in `m_uiNextPath`; answers the
    /// result, the calls (without the top-level one, frame addresses made
    /// relative to the first `NiFilename`) and the index afterwards.
    fn run(next: u32, default_path: u8) -> (bool, Calls, u32) {
        let mut e = Engine::new();
        e.map(DEFAULT_PATH, 4);
        e.mem.set_u8(DEFAULT_PATH, default_path);
        for f in [
            NI_FILENAME_CONSTRUCT,
            NI_FILENAME_JOIN,
            STRCPY_S,
            PATH_STANDARDIZE,
        ] {
            e.register(f, |_, a| a[0].into_ret());
        }
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this + 4, next);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00aa_2580, &args![this, 0x9000u32, 0x104u32])
            .bool();
        let log = e.call_log.take().unwrap();
        let frame = log[1].1[0];
        let rel = |w: u32| {
            if w == this + 8 {
                0xf008
            } else if w == this + 0x10c {
                0xf10c
            } else if (frame..frame + 0x808).contains(&w) {
                w - frame
            } else {
                w
            }
        };
        let calls = log[1..]
            .iter()
            .map(|(a, w)| (*a, w.iter().map(|&x| rel(x)).collect()))
            .collect();
        (result, calls, e.mem.u32(this + 4))
    }

    #[test]
    fn ni_search_path_get_next_search_path_case_0_keeps_the_split() {
        let (result, calls, next) = run(0, 0);
        assert!(result);
        assert_eq!(
            calls,
            vec![
                (NI_FILENAME_CONSTRUCT, vec![0, 0xf008]),
                (NI_FILENAME_JOIN, vec![0, 0x9000, 0x104]),
                (PATH_STANDARDIZE, vec![0x9000]),
            ]
        );
        assert_eq!(next, 1);
    }

    #[test]
    fn ni_search_path_get_next_search_path_cases_1_to_3() {
        let (result, calls, next) = run(1, 0);
        assert!(result);
        assert_eq!(next, 2);
        assert_eq!(calls[1], (STRCPY_S, vec![0x100, 3, EMPTY_TEXT]));
        assert_eq!(calls[2], (STRCPY_S, vec![0, 0x100, EMPTY_TEXT]));
        assert_eq!(calls[3].0, NI_FILENAME_JOIN);

        let (result, calls, next) = run(2, 0);
        assert!(result);
        assert_eq!(next, 3);
        assert_eq!(calls[2], (STRCPY_S, vec![0, 0x100, 0xf10c]));
        assert_eq!(calls[3], (NI_FILENAME_CONSTRUCT, vec![0x404, 0xf008]));
        assert_eq!(calls[4], (STRCPY_S, vec![0x303, 0x100, 0x404]));
        assert_eq!(calls[5].0, NI_FILENAME_JOIN);

        let (result, calls, _) = run(3, 0);
        assert!(result);
        assert_eq!(calls[3], (STRCPY_S, vec![0x303, 0x100, EMPTY_TEXT]));
    }

    #[test]
    fn ni_search_path_get_next_search_path_case_4_and_the_end() {
        let (result, calls, next) = run(4, 0);
        assert!(!result);
        assert_eq!(calls.len(), 1);
        assert_eq!(next, 4);

        let (result, calls, next) = run(4, b'd');
        assert!(result);
        assert_eq!(calls[2], (STRCPY_S, vec![0, 0x100, DEFAULT_PATH]));
        assert_eq!(next, 5);

        let (result, calls, next) = run(5, b'd');
        assert!(!result);
        assert_eq!(calls.len(), 1);
        assert_eq!(next, 5);
    }
}

#[cfg(test)]
mod tests_n084 {
    use super::*;

    fn frees(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == MEMORY_MANAGER_DEALLOCATE)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn ni_search_path_scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x210);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_2510, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_SEARCH_PATH_VTABLE);
        assert!(frees(&e).is_empty());
        assert_eq!(e.call(0x00aa_2510, &args![this, 1u32]).u32(), this);
        assert_eq!(frees(&e), vec![vec![MEMORY_MANAGER_OBJECT, this]]);
    }
}
