//! C runtime pieces left after the earlier files (MSVC 2008 LIBCMT/libcpmt, statically linked
//! into FalloutNV.exe 1.4.0.525), translated from the exe: the plain character classes
//! `isalpha` (`00ec86e1`), `_isupper` (`00ec8762`), `_islower` (`00ec87e1`) and `isalnum`
//! (`00ec89e9`); `std::bad_cast`'s copy constructor (`00ec79e2`) and destructor (`00ec79ff`),
//! `std::bad_typeid`'s destructor (`00ec7a7f`), `std::__non_rtti_object`'s constructors
//! (`00ec7a45`, `00ec7a62`); `type_info::~type_info` (`00ec409b`), its scalar deleting
//! destructor (`00ec40ab`) and `type_info::_Type_info_dtor` (`00ed16a2`); and
//! `std::locale::_Locimp`'s destructor (`00ec3648`) and scalar deleting destructor
//! (`00ec3685`).
//!
//! Every call is made by address, as the exe makes it, so a test can double each one.

use crate::prelude::*;

/// `__locale_changed` (`0126fdf4`) and the C ctype table pointer `_pctype` (`011c23e8`).
const LOCALE_CHANGED: u32 = 0x0126_fdf4;
const PCTYPE: u32 = 0x011c_23e8;
/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `std::exception`'s copy constructor (`00ec7944`, thiscall, `RET 4`) and destructor
/// (`00ec79a1`, thiscall).
const EXCEPTION_COPY: u32 = 0x00ec_7944;
const EXCEPTION_DTOR: u32 = 0x00ec_79a1;
/// The vtables of `std::bad_cast` (`010f1fc4`) and `std::bad_typeid` (`010f1fd0`).
const BAD_CAST_VTABLE: u32 = 0x010f_1fc4;
const BAD_TYPEID_VTABLE: u32 = 0x010f_1fd0;

// ---- Character classes -----------------------------------------------------------------------

/// The plain forms `isalpha`, `_isupper`, `_islower` and `isalnum` share: while
/// `__locale_changed` is 0 the C table `_pctype` masked (`MOVZX EAX,word ptr [ECX+EAX*2]; AND
/// EAX,mask`), else the `_l` form at `with_locale` called by address with a null locale
/// (`PUSH 0; PUSH c; CALL`), whose `EAX` is answered.
fn class_by_address(e: &mut Engine, c: u32, mask: u32, with_locale: u32) -> u32 {
    if e.mem.u32(LOCALE_CHANGED) == 0 {
        let table = e.mem.u32(PCTYPE);
        return u32::from(e.mem.u16(table.wrapping_add(c.wrapping_mul(2)))) & mask;
    }
    e.call(with_locale, &args![c, 0u32]).u32()
}

// Translated from 00ec86e1 (disassembled, FalloutNV.exe 1.4.0.525)
/// `isalpha` (LIBCMT/_ctype.c; cdecl): `_pctype[c] & _ALPHA` (0x103), or `_isalpha_l(c, NULL)`
/// (`00ec868b`) once the locale has changed.
pub fn isalpha(e: &mut Engine, c: u32) -> u32 {
    class_by_address(e, c, 0x103, 0x00ec_868b)
}

// Translated from 00ec8762 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_isupper` (LIBCMT/_ctype.c; cdecl): `_pctype[c] & _UPPER` (1), or `_isupper_l(c, NULL)`
/// (`00ec8711`) once the locale has changed.
pub fn isupper(e: &mut Engine, c: u32) -> u32 {
    class_by_address(e, c, 1, 0x00ec_8711)
}

// Translated from 00ec87e1 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_islower` (LIBCMT/_ctype.c; cdecl): `_pctype[c] & _LOWER` (2), or `_islower_l(c, NULL)`
/// (`00ec8790`) once the locale has changed.
pub fn islower(e: &mut Engine, c: u32) -> u32 {
    class_by_address(e, c, 2, 0x00ec_8790)
}

// Translated from 00ec89e9 (disassembled, FalloutNV.exe 1.4.0.525)
/// `isalnum` (LIBCMT/_ctype.c; cdecl): `_pctype[c] & (_ALPHA | _DIGIT)` (0x107), or
/// `_isalnum_l(c, NULL)` (`00ec8993`) once the locale has changed.
pub fn isalnum(e: &mut Engine, c: u32) -> u32 {
    class_by_address(e, c, 0x107, 0x00ec_8993)
}

// ---- Exceptions, type_info, _Locimp ------------------------------------------------------

/// The vtables of `std::__non_rtti_object` (`010f1fdc`), `type_info` (`010f1ee0`),
/// `std::locale::_Locimp` (`010f1ed8`) and `std::locale::facet` (`010a31fc`).
const NON_RTTI_OBJECT_VTABLE: u32 = 0x010f_1fdc;
const TYPE_INFO_VTABLE: u32 = 0x010f_1ee0;
const LOCIMP_VTABLE: u32 = 0x010f_1ed8;
const FACET_VTABLE: u32 = 0x010a_31fc;
/// `std::bad_typeid::bad_typeid(const char*)` (`00ec7a0a`) and its copy constructor
/// (`00ec7a28`), thiscall, `RET 4`.
const BAD_TYPEID_FROM_TEXT: u32 = 0x00ec_7a0a;
const BAD_TYPEID_COPY: u32 = 0x00ec_7a28;
/// `type_info::~type_info` (`00ec409b`, thiscall) and `type_info::_Type_info_dtor`
/// (`00ed16a2`, cdecl, one word).
const TYPE_INFO_DESTRUCTOR: u32 = 0x00ec_409b;
const TYPE_INFO_DTOR_HELPER: u32 = 0x00ed_16a2;
/// `std::locale::_Locimp::~_Locimp` (`00ec3648`, thiscall), the cdecl `00ec3504` it calls with
/// `this` (registered as `locimp_dtor`) and the thiscall `0044a730` (`RET 8`) it calls on
/// `this + 0x18`.
const LOCIMP_DESTRUCTOR: u32 = 0x00ec_3648;
const FN_00EC3504: u32 = 0x00ec_3504;
const FN_0044A730: u32 = 0x0044_a730;
/// `_lock` (`00eddf39`) and `_free` (`00ecd291`), cdecl, one word; the `__finally` block of
/// `_Type_info_dtor` (`00ed1709`, no arguments).
const LOCK: u32 = 0x00ed_df39;
const FREE: u32 = 0x00ec_d291;
const TYPE_INFO_DTOR_FINALLY: u32 = 0x00ed_1709;
/// `__type_info_root_node` (`0127013c`): its `_Next` word is at `01270140`.
const TYPE_INFO_ROOT: u32 = 0x0127_013c;

// Translated from 00ec79e2 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_cast`'s copy constructor (thiscall, one word; `RET 4`): the exception copy
/// constructor (`00ec7944`), then the `bad_cast` vtable. Answers `this`.
pub fn bad_cast_copy(e: &mut Engine, this: u32, other: u32) -> u32 {
    e.call(EXCEPTION_COPY, &args![this, other]);
    e.mem.set_u32(this, BAD_CAST_VTABLE);
    this
}

// Translated from 00ec79ff (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_cast::~bad_cast` (thiscall, no stack arguments): the `bad_cast` vtable, then a
/// tail jump to the exception destructor (`00ec79a1`), whose `EAX` is this function's.
pub fn bad_cast_destructor(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, BAD_CAST_VTABLE);
    e.call(EXCEPTION_DTOR, &args![this]).u32()
}

// Translated from 00ec7a45 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::__non_rtti_object::__non_rtti_object(const char*)` (thiscall, one word; `RET 4`):
/// `bad_typeid(const char*)` (`00ec7a0a`) with the same word, then the `__non_rtti_object`
/// vtable. Answers `this`.
pub fn non_rtti_object_from_text(e: &mut Engine, this: u32, text: u32) -> u32 {
    e.call(BAD_TYPEID_FROM_TEXT, &args![this, text]);
    e.mem.set_u32(this, NON_RTTI_OBJECT_VTABLE);
    this
}

// Translated from 00ec7a62 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::__non_rtti_object`'s copy constructor (thiscall, one word; `RET 4`): `bad_typeid`'s
/// copy constructor (`00ec7a28`), then the `__non_rtti_object` vtable. Answers `this`.
pub fn non_rtti_object_copy(e: &mut Engine, this: u32, other: u32) -> u32 {
    e.call(BAD_TYPEID_COPY, &args![this, other]);
    e.mem.set_u32(this, NON_RTTI_OBJECT_VTABLE);
    this
}

// Translated from 00ec7a7f (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_typeid::~bad_typeid` (thiscall, no stack arguments): the `bad_typeid` vtable,
/// then a tail jump to the exception destructor (`00ec79a1`), whose `EAX` is this function's.
pub fn bad_typeid_destructor(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, BAD_TYPEID_VTABLE);
    e.call(EXCEPTION_DTOR, &args![this]).u32()
}

// Translated from 00ec409b (disassembled, FalloutNV.exe 1.4.0.525)
/// `type_info::~type_info` (thiscall, no stack arguments): the `type_info` vtable, then
/// `type_info::_Type_info_dtor(this)` (`00ed16a2`, cdecl).
pub fn type_info_destructor(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, TYPE_INFO_VTABLE);
    e.call(TYPE_INFO_DTOR_HELPER, &args![this]);
}

// Translated from 00ec40ab (disassembled, FalloutNV.exe 1.4.0.525)
/// `type_info`'s scalar deleting destructor (thiscall, one word: the flags; `RET 4`):
/// `~type_info` (`00ec409b`), then `operator delete` when bit 0 of the flags is set. Answers
/// `this`.
pub fn type_info_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.call(TYPE_INFO_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00ed16a2 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed16a2 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180288) is not translated; its __finally block (00ed1709) is called as the exe calls it on the way out.
/// `type_info::_Type_info_dtor` (cdecl, one word: the `type_info`): under lock 0xE, when its
/// name word (+4) is set, the node list from `__type_info_root_node` (`0127013c`, next at +4)
/// is searched for a node holding that name; a match is unlinked (`[prev+4] = [node+4]`) and
/// freed, then the name is freed and cleared.
///
/// The exe's search never advances: a node that does not match is made the previous one
/// (`00ed1705: MOV EDX,EAX; JMP 00ed16ce`) and the same node (still the list's first) is
/// tested again, so a first node that does not match spins forever, as here. Only a null
/// first node or a matching one leaves the loop.
pub fn type_info_dtor_helper(e: &mut Engine, info: u32) {
    e.call(LOCK, &args![0xeu32]);
    let name = e.mem.u32(info.wrapping_add(4));
    if name != 0 {
        let node = e.mem.u32(TYPE_INFO_ROOT.wrapping_add(4));
        let mut prev = TYPE_INFO_ROOT;
        if node != 0 {
            loop {
                if e.mem.u32(node) == name {
                    let next = e.mem.u32(node.wrapping_add(4));
                    e.mem.set_u32(prev.wrapping_add(4), next);
                    e.call(FREE, &args![node]);
                    break;
                }
                prev = node;
            }
        }
        let name = e.mem.u32(info.wrapping_add(4));
        e.call(FREE, &args![name]);
        e.mem.set_u32(info.wrapping_add(4), 0);
    }
    e.call(TYPE_INFO_DTOR_FINALLY, &args![]);
}

// Translated from 00ec3648 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec3648 - the compiler's exception frame (`__EH_prolog3` at 00ece255 with unwind table 00f34e11, `__EH_epilog3` at 00ece2be: FS:[0] chain) is not translated; its state word (1 after the vtable store) only matters to an unwind.
/// `std::locale::_Locimp::~_Locimp` (thiscall, no stack arguments): the `_Locimp` vtable,
/// the cdecl `00ec3504(this)`, the thiscall `0044a730` on `this + 0x18` with (1, 0)
/// (`PUSH 0; PUSH 1`), then the `std::locale::facet` vtable.
pub fn locimp_destructor(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, LOCIMP_VTABLE);
    e.call(FN_00EC3504, &args![this]);
    e.call(FN_0044A730, &args![this.wrapping_add(0x18), 1u32, 0u32]);
    e.mem.set_u32(this, FACET_VTABLE);
}

// Translated from 00ec3685 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::locale::_Locimp`'s scalar deleting destructor (thiscall, one word: the flags;
/// `RET 4`): `~_Locimp` (`00ec3648`), then `operator delete` when bit 0 of the flags is set.
/// Answers `this`.
pub fn locimp_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.call(LOCIMP_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// ---- Streams ---------------------------------------------------------------------------------

/// `_errno` (`00ec85e3`, cdecl, no arguments: the thread's `errno` word) and
/// `_invalid_parameter` (`00ec7c30`, five words, all 0 here).
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
/// `_lock_file` (`00ece01b`), `_fileno` (`00ed806c`), `_filbuf` (`00edc81f`), `_flush`
/// (`00ecd8e6`), `_freebuf` (`00ed8207`) and `_malloc_crt` (`00ed0cdf`), cdecl, one word;
/// `_flsbuf` (`00ed2d64`) and `_ungetc_nolock` (`00ecd4ce`), cdecl: character, stream.
const LOCK_FILE: u32 = 0x00ec_e01b;
const FILENO: u32 = 0x00ed_806c;
const FILBUF: u32 = 0x00ed_c81f;
const FLUSH: u32 = 0x00ec_d8e6;
const FREEBUF: u32 = 0x00ed_8207;
const MALLOC_CRT: u32 = 0x00ed_0cdf;
const FLSBUF: u32 = 0x00ed_2d64;
const UNGETC_NOLOCK: u32 = 0x00ec_d4ce;
/// `_ftelli64` (`00edf08c`, cdecl, one word; `EDX:EAX`) and `_fseeki64` (`00edf172`, cdecl:
/// stream, offset low, offset high, origin).
const FTELLI64: u32 = 0x00ed_f08c;
const FSEEKI64: u32 = 0x00ed_f172;
/// The `__finally` blocks (cdecl here, one word: the stream) of `fputc` (`00ecd4c6`), `ungetc`
/// (`00ecd627`), `fgetc` (`00ecd72d`) and `setvbuf` (`00ecd8dc`).
const FPUTC_FINALLY: u32 = 0x00ec_d4c6;
const UNGETC_FINALLY: u32 = 0x00ec_d627;
const FGETC_FINALLY: u32 = 0x00ec_d72d;
const SETVBUF_FINALLY: u32 = 0x00ec_d8dc;
/// `__pioinfo` (`01270960`, 32 blocks of 0x40-byte entries), `__badioinfo` (`011c26f0`) and
/// `_cflush` (`0126fdf0`).
const PIOINFO: u32 = 0x0127_0960;
const BAD_IOINFO: u32 = 0x011c_26f0;
const CFLUSH: u32 = 0x0126_fdf0;
/// `vprintf_helper` (`00ecbdd0`, cdecl: output function, format, locale, arguments) and
/// `_output_l` (`00ed2f21`), the output function `vprintf` hands it.
const VPRINTF_HELPER: u32 = 0x00ec_bdd0;
const OUTPUT_L: u32 = 0x00ed_2f21;
/// `_mbstowcs_s_l` (`00ecf205`, cdecl, six words).
const MBSTOWCS_S_L: u32 = 0x00ec_f205;

/// `errno = EINVAL` (0x16) and `_invalid_parameter(0, 0, 0, 0, 0)`, as each check below does.
fn invalid(e: &mut Engine) {
    let errno = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(errno, 0x16);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

/// `_pioinfo(fh)` as the stream functions inline it: `__badioinfo` for -1 and -2, else the
/// entry `(fh & 0x1F) * 0x40` into block `fh >> 5` (arithmetic shift) of `__pioinfo`.
fn ioinfo(e: &Engine, fh: u32) -> u32 {
    if fh == u32::MAX || fh == 0xffff_fffe {
        return BAD_IOINFO;
    }
    let block = (((fh as i32) >> 5) as u32).wrapping_mul(4);
    ((fh & 0x1f) << 6).wrapping_add(e.mem.u32(block.wrapping_add(PIOINFO)))
}

/// `_VALIDATE_STREAM_ANSI`: for a stream that is not a string (`_IOSTRG`, 0x40 of +0xC), its
/// handle's entry must have no text mode bits (0x7F of +0x24) and not be UTF-8 (0x80 of +0x24);
/// otherwise `EINVAL` is reported and -1 answered. 0 when it passes.
fn validate_stream_ansi(e: &mut Engine, stream: u32) -> u32 {
    if e.mem.u8(stream.wrapping_add(0xc)) & 0x40 != 0 {
        return 0;
    }
    let fh = e.call(FILENO, &args![stream]).u32();
    let failed = if e.mem.u8(ioinfo(e, fh).wrapping_add(0x24)) & 0x7f != 0 {
        true
    } else {
        e.mem.u8(ioinfo(e, fh).wrapping_add(0x24)) & 0x80 != 0
    };
    if failed {
        invalid(e);
        return u32::MAX;
    }
    0
}

// Translated from 00ecbe60 (disassembled, FalloutNV.exe 1.4.0.525)
/// `vprintf` (LIBCMT/vprintf.c; cdecl: format, argument list): `vprintf_helper(_output_l,
/// format, NULL, arguments)`, whose `EAX` is answered.
pub fn vprintf(e: &mut Engine, format: u32, list: u32) -> u32 {
    e.call(VPRINTF_HELPER, &args![OUTPUT_L, format, 0u32, list])
        .u32()
}

// Translated from 00ecd3c2 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecd3c2 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117ff50) is not translated; its __finally block (00ecd4c6) is called as the exe calls it on the way out.
/// `fputc` (LIBCMT/fputc.c; cdecl: character, stream). A null stream reports `EINVAL` and
/// answers -1. Otherwise, with the stream locked, a stream that fails
/// [`validate_stream_ansi`] answers -1; else the count (+4) is decremented and, while it
/// stays non-negative, the character's low byte is stored at the pointer (+0), which moves on,
/// and answered as a byte; a negative count hands the character to `_flsbuf`.
pub fn fputc(e: &mut Engine, c: u32, stream: u32) -> i32 {
    if stream == 0 {
        invalid(e);
        return -1;
    }
    e.call(LOCK_FILE, &args![stream]);
    let mut result = validate_stream_ansi(e, stream);
    if result == 0 {
        let count = e.mem.u32(stream.wrapping_add(4)).wrapping_sub(1);
        e.mem.set_u32(stream.wrapping_add(4), count);
        result = if (count as i32) < 0 {
            e.call(FLSBUF, &args![c, stream]).u32()
        } else {
            let at = e.mem.u32(stream);
            e.mem.set_u8(at, c as u8);
            let at = e.mem.u32(stream).wrapping_add(1);
            e.mem.set_u32(stream, at);
            u32::from(c as u8)
        };
    }
    e.call(FPUTC_FINALLY, &args![stream]);
    result as i32
}

// Translated from 00ecd5bf (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecd5bf - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117ff70) is not translated; its __finally block (00ecd627) is called as the exe calls it on the way out.
/// `ungetc` (LIBCMT/ungetc.c; cdecl: character, stream): a null stream reports `EINVAL` and
/// answers -1; otherwise `_ungetc_nolock` with the stream locked, whose `EAX` is answered.
pub fn ungetc(e: &mut Engine, c: u32, stream: u32) -> i32 {
    if stream == 0 {
        invalid(e);
        return -1;
    }
    e.call(LOCK_FILE, &args![stream]);
    let result = e.call(UNGETC_NOLOCK, &args![c, stream]).u32();
    e.call(UNGETC_FINALLY, &args![stream]);
    result as i32
}

// Translated from 00ecd631 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecd631 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117ff90) is not translated; its __finally block (00ecd72d) is called as the exe calls it on the way out.
/// `fgetc` (LIBCMT/fgetc.c; cdecl: the stream). A null stream reports `EINVAL` and answers -1.
/// Otherwise, with the stream locked, a stream that fails [`validate_stream_ansi`] answers -1;
/// else the count (+4) is decremented and, while it stays non-negative, the byte at the
/// pointer (+0) is answered and the pointer moves on; a negative count answers `_filbuf`.
pub fn fgetc(e: &mut Engine, stream: u32) -> i32 {
    if stream == 0 {
        invalid(e);
        return -1;
    }
    e.call(LOCK_FILE, &args![stream]);
    let mut result = validate_stream_ansi(e, stream);
    if result == 0 {
        let count = e.mem.u32(stream.wrapping_add(4)).wrapping_sub(1);
        e.mem.set_u32(stream.wrapping_add(4), count);
        result = if (count as i32) < 0 {
            e.call(FILBUF, &args![stream]).u32()
        } else {
            let at = e.mem.u32(stream);
            let byte = u32::from(e.mem.u8(at));
            e.mem.set_u32(stream, at.wrapping_add(1));
            byte
        };
    }
    e.call(FGETC_FINALLY, &args![stream]);
    result as i32
}

// Translated from 00ecd735 (disassembled, FalloutNV.exe 1.4.0.525)
/// `fgetpos` (LIBCMT/fgetpos.c; cdecl: stream, position): a null stream or position reports
/// `EINVAL` and answers -1. Otherwise `_ftelli64(stream)` is stored at the position (low word,
/// high word) and -1 answered when both words are -1 (`AND ECX,EDX; CMP ECX,-1`), else 0.
pub fn fgetpos(e: &mut Engine, stream: u32, position: u32) -> i32 {
    if stream == 0 || position == 0 {
        invalid(e);
        return -1;
    }
    let at = e.call(FTELLI64, &args![stream]).u64();
    let (low, high) = (at as u32, (at >> 32) as u32);
    e.mem.set_u32(position, low);
    e.mem.set_u32(position.wrapping_add(4), high);
    if low & high == u32::MAX {
        -1
    } else {
        0
    }
}

// Translated from 00ecd7a3 (disassembled, FalloutNV.exe 1.4.0.525)
/// `fsetpos` (LIBCMT/fsetpos.c; cdecl: stream, position): a null stream or position reports
/// `EINVAL` and answers -1; otherwise `_fseeki64(stream, *position, SEEK_SET)`, whose `EAX` is
/// answered.
pub fn fsetpos(e: &mut Engine, stream: u32, position: u32) -> i32 {
    if stream == 0 || position == 0 {
        invalid(e);
        return -1;
    }
    let low = e.mem.u32(position);
    let high = e.mem.u32(position.wrapping_add(4));
    e.call(FSEEKI64, &args![stream, low, high, 0u32]).u32() as i32
}

// Translated from 00ecd7e8 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecd7e8 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117ffb0) is not translated; its __finally block (00ecd8dc) is called as the exe calls it on the way out.
/// `setvbuf` (LIBCMT/setvbuf.c; cdecl: stream, buffer, mode, size). A null stream, a mode
/// other than `_IONBF` (4), `_IOFBF` (0) and `_IOLBF` (0x40), or for the last two a size
/// outside 2..=0x7FFFFFFF (`size - 2` above 0x7FFFFFFD unsigned) reports `EINVAL` and answers
/// -1. The size is made even. With the stream locked it is flushed (`_flush`) and its buffer
/// freed (`_freebuf`), and its flags (+0xC) lose 0x3D0C. `_IONBF` uses the stream's own
/// two-byte buffer (+0x14) with flag 4; a given buffer is used with flag 0x500; with none,
/// `_malloc_crt(size)` makes one (flags 0x408), and when that fails `_cflush` is counted up and
/// -1 answered with the stream left unbuffered as it is. A buffer in use is stored as base
/// (+8) and pointer (+0), its size at +0x18 and the count (+4) cleared; 0 is answered.
pub fn setvbuf(e: &mut Engine, stream: u32, buffer: u32, mode: u32, size: u32) -> i32 {
    if stream == 0 {
        invalid(e);
        return -1;
    }
    let size = if mode == 4 {
        size
    } else if mode == 0 || mode == 0x40 {
        if size.wrapping_sub(2) > 0x7fff_fffd {
            invalid(e);
            return -1;
        }
        size
    } else {
        invalid(e);
        return -1;
    };
    let mut size = size & 0xffff_fffe;
    let mut result = 0u32;
    e.call(LOCK_FILE, &args![stream]);
    e.call(FLUSH, &args![stream]);
    e.call(FREEBUF, &args![stream]);
    let flags_at = stream.wrapping_add(0xc);
    let flags = e.mem.u32(flags_at) & 0xffff_c2f3;
    e.mem.set_u32(flags_at, flags);
    let flags = e.mem.u32(flags_at);
    let base = if mode & 4 != 0 {
        e.mem.set_u32(flags_at, flags | 4);
        size = 2;
        Some(stream.wrapping_add(0x14))
    } else if buffer != 0 {
        e.mem.set_u32(flags_at, flags | 0x500);
        Some(buffer)
    } else {
        let block = e.call(MALLOC_CRT, &args![size]).u32();
        if block == 0 {
            let flushes = e.mem.u32(CFLUSH).wrapping_add(1);
            e.mem.set_u32(CFLUSH, flushes);
            result = u32::MAX;
            None
        } else {
            let flags = e.mem.u32(flags_at) | 0x408;
            e.mem.set_u32(flags_at, flags);
            Some(block)
        }
    };
    if let Some(base) = base {
        e.mem.set_u32(stream.wrapping_add(0x18), size);
        e.mem.set_u32(stream.wrapping_add(8), base);
        e.mem.set_u32(stream, base);
        e.mem.set_u32(stream.wrapping_add(4), 0);
    }
    e.call(SETVBUF_FINALLY, &args![stream]);
    result as i32
}

// Translated from 00ecf315 (disassembled, FalloutNV.exe 1.4.0.525)
/// `mbstowcs_s` (LIBCMT/mbstowcs.c; cdecl: count out, destination, size, source, count):
/// `_mbstowcs_s_l` with the same five words and a null locale, whose `EAX` is answered.
pub fn mbstowcs_s(e: &mut Engine, out: u32, dest: u32, size: u32, source: u32, count: u32) -> u32 {
    e.call(MBSTOWCS_S_L, &args![out, dest, size, source, count, 0u32])
        .u32()
}

// ---- Small blocks: exit, lock releases, filters ------------------------------------------------

/// `doexit` (`00eccd99`, cdecl: code, quick, retcaller), `_amsg_exit` (`00eccc55`, cdecl, one
/// word) and `_unlock` (`00edde5f`, cdecl, one word).
const DOEXIT: u32 = 0x00ec_cd99;
const AMSG_EXIT: u32 = 0x00ec_cc55;
const UNLOCK: u32 = 0x00ed_de5f;
/// The vtable of `std::bad_exception` (`010f20a8`).
const BAD_EXCEPTION_VTABLE: u32 = 0x010f_20a8;

// Translated from 00eccf00 (disassembled, FalloutNV.exe 1.4.0.525)
/// No Ghidra function (the shape of LIBCMT's `_c_exit`, unconfirmed; cdecl, no arguments):
/// `doexit(0, 1, 1)` (`PUSH 1; PUSH 1; PUSH 0`), the C termination without the `atexit`
/// table and without leaving the process.
pub fn c_exit(e: &mut Engine) {
    e.call(DOEXIT, &args![0u32, 1u32, 1u32]);
}

// Translated from 00ee1a8f (disassembled, FalloutNV.exe 1.4.0.525)
/// `_fptrap` (LIBCMT/crt0fp.c; cdecl, no arguments): `_amsg_exit(_RT_FLOAT)` (2), the
/// "floating point not loaded" exit.
pub fn fptrap(e: &mut Engine) {
    e.call(AMSG_EXIT, &args![2u32]);
}

// Translated from 00ecd0a1 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_realloc` (`00eccf5d`): `_unlock(4)` (the heap lock).
pub fn realloc_finally(e: &mut Engine) {
    e.call(UNLOCK, &args![4u32]);
}

// Translated from 00ecd2e7 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_free` (`00ecd291`): `_unlock(4)` (the heap lock).
pub fn free_finally(e: &mut Engine) {
    e.call(UNLOCK, &args![4u32]);
}

// Translated from 00ecd3b9 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_msize` (`00ecd31f`): `_unlock(4)` (the heap lock).
pub fn msize_finally(e: &mut Engine) {
    e.call(UNLOCK, &args![4u32]);
}

// Translated from 00ee16de (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `__calloc_impl` (`00ee15e2`): `_unlock(4)` (the heap lock).
pub fn calloc_impl_finally(e: &mut Engine) {
    e.call(UNLOCK, &args![4u32]);
}

// Translated from 00ee3114 (disassembled, FalloutNV.exe 1.4.0.525)
/// The outer `__finally` block of `_alloc_osfhnd` (`00ee2f83`): `_unlock(0xB)`.
pub fn alloc_osfhnd_finally(e: &mut Engine) {
    e.call(UNLOCK, &args![0xbu32]);
}

// Translated from 00ec44a1 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` filter of `__RTDynamicCast` (`00ec43fb`, scope table `0117fc68`): its frame
/// word `[EBP-0x14]` (the `EXCEPTION_POINTERS`) is the argument. Answers 1
/// (`EXCEPTION_EXECUTE_HANDLER`) when the exception code is an access violation (0xC0000005),
/// else 0 (`EXCEPTION_CONTINUE_SEARCH`).
pub fn rt_dynamic_cast_filter(e: &mut Engine, pointers: u32) -> u32 {
    let record = e.mem.u32(pointers);
    u32::from(e.mem.u32(record) == 0xc000_0005)
}

// Translated from 00ece2f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_exception::~bad_exception` (thiscall, no stack arguments): the `bad_exception`
/// vtable, then a tail jump to the exception destructor (`00ec79a1`), whose `EAX` is this
/// function's.
pub fn bad_exception_destructor(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, BAD_EXCEPTION_VTABLE);
    e.call(EXCEPTION_DTOR, &args![this]).u32()
}

/// This file's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_86e1, isalpha(u32) -> u32),
        entry!(0x00ec_8762, isupper(u32) -> u32),
        entry!(0x00ec_87e1, islower(u32) -> u32),
        entry!(0x00ec_89e9, isalnum(u32) -> u32),
        entry!(0x00ec_79e2, bad_cast_copy(u32, u32) -> u32),
        entry!(0x00ec_79ff, bad_cast_destructor(u32) -> u32),
        entry!(0x00ec_7a45, non_rtti_object_from_text(u32, u32) -> u32),
        entry!(0x00ec_7a62, non_rtti_object_copy(u32, u32) -> u32),
        entry!(0x00ec_7a7f, bad_typeid_destructor(u32) -> u32),
        entry!(0x00ec_409b, type_info_destructor(u32)),
        entry!(0x00ec_40ab, type_info_deleting_destructor(u32, u32) -> u32),
        entry!(0x00ed_16a2, type_info_dtor_helper(u32)),
        entry!(0x00ec_3648, locimp_destructor(u32)),
        entry!(0x00ec_3685, locimp_deleting_destructor(u32, u32) -> u32),
        entry!(0x00ec_be60, vprintf(u32, u32) -> u32),
        entry!(0x00ec_d3c2, fputc(u32, u32) -> i32),
        entry!(0x00ec_d5bf, ungetc(u32, u32) -> i32),
        entry!(0x00ec_d631, fgetc(u32) -> i32),
        entry!(0x00ec_d735, fgetpos(u32, u32) -> i32),
        entry!(0x00ec_d7a3, fsetpos(u32, u32) -> i32),
        entry!(0x00ec_d7e8, setvbuf(u32, u32, u32, u32) -> i32),
        entry!(0x00ec_f315, mbstowcs_s(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ec_cf00, c_exit()),
        entry!(0x00ee_1a8f, fptrap()),
        entry!(0x00ec_d0a1, realloc_finally()),
        entry!(0x00ec_d2e7, free_finally()),
        entry!(0x00ec_d3b9, msize_finally()),
        entry!(0x00ee_16de, calloc_impl_finally()),
        entry!(0x00ee_3114, alloc_osfhnd_finally()),
        entry!(0x00ec_44a1, rt_dynamic_cast_filter(u32) -> u32),
        entry!(0x00ec_e2f0, bad_exception_destructor(u32) -> u32),
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

    fn nothing(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    #[test]
    fn the_plain_classes_read_pctype_until_the_locale_changes() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011c_2000, 0x1000);
        let table = e.mem.alloc(0x200);
        e.mem.set_u32(PCTYPE, table);
        e.mem.set_u32(LOCALE_CHANGED, 0);
        e.mem.set_u16(table + u32::from(b'q') * 2, 0x102);
        e.mem.set_u16(table + u32::from(b'Q') * 2, 0x101);
        e.mem.set_u16(table + u32::from(b'7') * 2, 0x84);
        e.mem.set_u16(table + u32::from(b'-') * 2, 0x10);
        assert_eq!(e.call(0x00ec_86e1, &[u32::from(b'q')]).u32(), 0x102);
        assert_eq!(e.call(0x00ec_86e1, &[u32::from(b'7')]).u32(), 0);
        assert_eq!(e.call(0x00ec_8762, &[u32::from(b'Q')]).u32(), 1);
        assert_eq!(e.call(0x00ec_8762, &[u32::from(b'q')]).u32(), 0);
        assert_eq!(e.call(0x00ec_87e1, &[u32::from(b'q')]).u32(), 2);
        assert_eq!(e.call(0x00ec_87e1, &[u32::from(b'Q')]).u32(), 0);
        assert_eq!(e.call(0x00ec_89e9, &[u32::from(b'7')]).u32(), 4);
        assert_eq!(e.call(0x00ec_89e9, &[u32::from(b'q')]).u32(), 0x102);
        assert_eq!(e.call(0x00ec_89e9, &[u32::from(b'-')]).u32(), 0);
        // Once the locale has changed each asks its _l form with a null locale.
        e.mem.set_u32(LOCALE_CHANGED, 1);
        for (plain, with_locale) in [
            (0x00ec_86e1_u32, 0x00ec_868b_u32),
            (0x00ec_8762, 0x00ec_8711),
            (0x00ec_87e1, 0x00ec_8790),
            (0x00ec_89e9, 0x00ec_8993),
        ] {
            e.register(with_locale, |_, a| (0x7700 | a[0] | a[1] << 16).into_ret());
            assert_eq!(e.call(plain, &[0x41]).u32(), 0x7741, "{plain:08x}");
        }
    }

    #[test]
    fn bad_cast_and_non_rtti_object_constructors_and_destructors() {
        let mut e = Engine::new();
        nothing(
            &mut e,
            &[EXCEPTION_COPY, BAD_TYPEID_FROM_TEXT, BAD_TYPEID_COPY],
        );
        e.register(EXCEPTION_DTOR, |_, a| (a[0] ^ 0x55).into_ret());
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_79e2, &[a, b]).u32(), a);
        assert_eq!(e.mem.u32(a), BAD_CAST_VTABLE);
        assert_eq!(calls(&e, EXCEPTION_COPY), vec![vec![a, b]]);
        assert_eq!(e.call(0x00ec_7a45, &[a, 0x0600_0000]).u32(), a);
        assert_eq!(e.mem.u32(a), NON_RTTI_OBJECT_VTABLE);
        assert_eq!(calls(&e, BAD_TYPEID_FROM_TEXT), vec![vec![a, 0x0600_0000]]);
        assert_eq!(e.call(0x00ec_7a62, &[b, a]).u32(), b);
        assert_eq!(e.mem.u32(b), NON_RTTI_OBJECT_VTABLE);
        assert_eq!(calls(&e, BAD_TYPEID_COPY), vec![vec![b, a]]);
        // The destructors store their vtable and answer what the exception destructor does.
        assert_eq!(e.call(0x00ec_79ff, &[a]).u32(), a ^ 0x55);
        assert_eq!(e.mem.u32(a), BAD_CAST_VTABLE);
        assert_eq!(e.call(0x00ec_7a7f, &[b]).u32(), b ^ 0x55);
        assert_eq!(e.mem.u32(b), BAD_TYPEID_VTABLE);
        assert_eq!(calls(&e, EXCEPTION_DTOR), vec![vec![a], vec![b]]);
    }

    #[test]
    fn type_info_destructors_reach_type_info_dtor() {
        let mut e = Engine::new();
        nothing(&mut e, &[TYPE_INFO_DTOR_HELPER, OPERATOR_DELETE]);
        let info = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        e.call(0x00ec_409b, &[info]);
        assert_eq!(e.mem.u32(info), TYPE_INFO_VTABLE);
        assert_eq!(calls(&e, TYPE_INFO_DTOR_HELPER), vec![vec![info]]);
        e.mem.set_u32(info, 0);
        assert_eq!(e.call(0x00ec_40ab, &[info, 0]).u32(), info);
        assert_eq!(e.mem.u32(info), TYPE_INFO_VTABLE);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x00ec_40ab, &[info, 3]).u32(), info);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![info]]);
        assert_eq!(calls(&e, TYPE_INFO_DESTRUCTOR).len(), 3);
    }

    #[test]
    fn type_info_dtor_unlinks_a_matching_first_node_and_frees_the_name() {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x1000);
        nothing(&mut e, &[LOCK, FREE, UNLOCK]);
        let info = e.mem.alloc(12);
        let node = e.mem.alloc(8);
        let name = 0x0600_0000;
        e.mem.set_u32(info + 4, name);
        e.mem.set_u32(node, name);
        e.mem.set_u32(node + 4, 0x0600_1000);
        e.mem.set_u32(TYPE_INFO_ROOT + 4, node);
        e.call_log = Some(vec![]);
        e.call(0x00ed_16a2, &[info]);
        assert_eq!(e.mem.u32(TYPE_INFO_ROOT + 4), 0x0600_1000);
        assert_eq!(calls(&e, FREE), vec![vec![node], vec![name]]);
        assert_eq!(e.mem.u32(info + 4), 0);
        assert_eq!(calls(&e, LOCK), vec![vec![0xe]]);
        assert_eq!(calls(&e, TYPE_INFO_DTOR_FINALLY), vec![vec![]]);
        // An empty list: only the name is freed. A null name: nothing but the lock pair.
        e.mem.set_u32(TYPE_INFO_ROOT + 4, 0);
        e.mem.set_u32(info + 4, name);
        e.call_log = Some(vec![]);
        e.call(0x00ed_16a2, &[info]);
        assert_eq!(calls(&e, FREE), vec![vec![name]]);
        e.call_log = Some(vec![]);
        e.call(0x00ed_16a2, &[info]);
        assert!(calls(&e, FREE).is_empty());
        assert_eq!(calls(&e, LOCK), vec![vec![0xe]]);
        assert_eq!(calls(&e, TYPE_INFO_DTOR_FINALLY).len(), 1);
    }

    #[test]
    fn locimp_destructors_store_both_vtables_around_their_calls() {
        let mut e = Engine::new();
        nothing(&mut e, &[FN_0044A730, OPERATOR_DELETE]);
        e.register(FN_00EC3504, |e, a| {
            assert_eq!(e.mem.u32(a[0]), LOCIMP_VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3648, &[this]);
        assert_eq!(e.mem.u32(this), FACET_VTABLE);
        assert_eq!(calls(&e, FN_00EC3504), vec![vec![this]]);
        assert_eq!(calls(&e, FN_0044A730), vec![vec![this + 0x18, 1, 0]]);
        assert_eq!(e.call(0x00ec_3685, &[this, 0]).u32(), this);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x00ec_3685, &[this, 1]).u32(), this);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
        assert_eq!(calls(&e, LOCIMP_DESTRUCTOR).len(), 3);
    }
    /// An engine with `_errno`, `_invalid_parameter`, the stream lock and the finally blocks'
    /// `_unlock_file` doubled; answers it and the `errno` word.
    fn stream_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let errno = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        nothing(&mut e, &[INVALID_PARAMETER, LOCK_FILE, 0x00ec_e08e]);
        e.map(0x0127_0000, 0x1000);
        e.map(0x011c_2000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        (e, errno)
    }

    #[test]
    fn vprintf_and_mbstowcs_s_forward_their_words() {
        let mut e = Engine::new();
        e.register(VPRINTF_HELPER, |_, a| {
            (a[0] ^ a[1] ^ a[2] ^ a[3]).into_ret()
        });
        e.register(MBSTOWCS_S_L, |_, a| {
            assert_eq!(a[5], 0);
            (a[0] + a[1] + a[2] + a[3] + a[4]).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_be60, &[0x100, 0x2000]).u32(),
            OUTPUT_L ^ 0x2100
        );
        assert_eq!(
            calls(&e, VPRINTF_HELPER),
            vec![vec![OUTPUT_L, 0x100, 0, 0x2000]]
        );
        assert_eq!(e.call(0x00ec_f315, &[1, 2, 3, 4, 5]).u32(), 15);
        assert_eq!(calls(&e, MBSTOWCS_S_L), vec![vec![1, 2, 3, 4, 5, 0]]);
    }

    #[test]
    fn fputc_and_fgetc_use_the_buffer_then_flsbuf_and_filbuf() {
        let (mut e, errno) = stream_engine();
        e.register(FLSBUF, |_, a| (a[0] | 0x1_0000).into_ret());
        e.register(FILBUF, |_, _| 0x77u32.into_ret());
        e.register(FILENO, |_, _| u32::MAX.into_ret());
        let stream = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(8);
        e.mem.set_u32(stream, buffer);
        e.mem.set_u32(stream + 4, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d3c2, &[0x1ab, stream]).i32(), 0xab);
        assert_eq!(e.mem.u8(buffer), 0xab);
        assert_eq!(e.mem.u32(stream), buffer + 1);
        assert_eq!(e.mem.u32(stream + 4), 0);
        // The count goes negative: _flsbuf gets the whole character word.
        assert_eq!(e.call(0x00ec_d3c2, &[0x1ab, stream]).i32(), 0x1_01ab);
        assert_eq!(calls(&e, FLSBUF), vec![vec![0x1ab, stream]]);
        assert_eq!(e.mem.u32(stream + 4), u32::MAX);
        assert_eq!(calls(&e, FPUTC_FINALLY), vec![vec![stream]; 2]);
        // fgetc reads the byte back, then _filbuf.
        e.mem.set_u32(stream, buffer);
        e.mem.set_u32(stream + 4, 1);
        assert_eq!(e.call(0x00ec_d631, &[stream]).i32(), 0xab);
        assert_eq!(e.mem.u32(stream), buffer + 1);
        assert_eq!(e.call(0x00ec_d631, &[stream]).i32(), 0x77);
        assert_eq!(calls(&e, FGETC_FINALLY), vec![vec![stream]; 2]);
        // A null stream: EINVAL, -1, no lock.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d3c2, &[0x41, 0]).i32(), -1);
        assert_eq!(e.call(0x00ec_d631, &[0]).i32(), -1);
        assert_eq!(e.mem.u32(errno), 0x16);
        assert!(calls(&e, LOCK_FILE).is_empty());
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 2);
    }

    #[test]
    fn fputc_and_fgetc_refuse_a_text_mode_or_utf8_handle() {
        let (mut e, errno) = stream_engine();
        nothing(&mut e, &[FLSBUF, FILBUF]);
        e.register(FILENO, |_, _| 0x21u32.into_ret());
        let block = e.mem.alloc(0x800);
        e.mem.set_u32(PIOINFO + 4, block);
        let entry = block + 0x40;
        let stream = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(8);
        for (mode, refused) in [(0u8, false), (1, true), (0x80, true), (0x7f, true)] {
            e.mem.set_u8(entry + 0x24, mode);
            e.mem.set_u32(stream, buffer);
            e.mem.set_u32(stream + 4, 4);
            e.mem.set_u32(errno, 0);
            let want = if refused { -1 } else { 0x41 };
            assert_eq!(
                e.call(0x00ec_d3c2, &[0x41, stream]).i32(),
                want,
                "{mode:02x}"
            );
            e.mem.set_u32(stream, buffer);
            assert_eq!(e.call(0x00ec_d631, &[stream]).i32(), want, "{mode:02x}");
            assert_eq!(e.mem.u32(errno), if refused { 0x16 } else { 0 });
        }
        // A string stream (_IOSTRG) is not asked about its handle.
        e.mem.set_u8(entry + 0x24, 0x80);
        e.mem.set_u32(stream + 0xc, 0x40);
        e.mem.set_u32(stream, buffer);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d3c2, &[0x42, stream]).i32(), 0x42);
        assert!(calls(&e, FILENO).is_empty());
    }

    #[test]
    fn ungetc_locks_around_ungetc_nolock() {
        let (mut e, errno) = stream_engine();
        e.register(UNGETC_NOLOCK, |_, a| (a[0] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d5bf, &[0x41, 0x0600_0000]).i32(), 0x42);
        assert_eq!(calls(&e, UNGETC_NOLOCK), vec![vec![0x41, 0x0600_0000]]);
        assert_eq!(calls(&e, LOCK_FILE), vec![vec![0x0600_0000]]);
        assert_eq!(calls(&e, UNGETC_FINALLY), vec![vec![0x0600_0000]]);
        assert_eq!(e.call(0x00ec_d5bf, &[0x41, 0]).i32(), -1);
        assert_eq!(e.mem.u32(errno), 0x16);
    }

    #[test]
    fn fgetpos_and_fsetpos_go_through_the_64_bit_calls() {
        let (mut e, errno) = stream_engine();
        let answer = e.mem.alloc(8);
        e.register_double(FTELLI64, move |e, _| Ret {
            eax: e.mem.u32(answer),
            edx: e.mem.u32(answer + 4),
            ..Ret::default()
        });
        e.register(FSEEKI64, |_, a| (a[1] ^ a[2]).into_ret());
        let position = e.mem.alloc(8);
        e.mem.set_u32(answer, 0x1234);
        e.mem.set_u32(answer + 4, 5);
        assert_eq!(e.call(0x00ec_d735, &[0x0600_0000, position]).i32(), 0);
        assert_eq!((e.mem.u32(position), e.mem.u32(position + 4)), (0x1234, 5));
        e.mem.set_u32(answer, u32::MAX);
        assert_eq!(e.call(0x00ec_d735, &[0x0600_0000, position]).i32(), 0);
        e.mem.set_u32(answer + 4, u32::MAX);
        assert_eq!(e.call(0x00ec_d735, &[0x0600_0000, position]).i32(), -1);
        e.mem.set_u32(position, 0x30);
        e.mem.set_u32(position + 4, 0x3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d7a3, &[0x0600_0000, position]).i32(), 0x33);
        assert_eq!(calls(&e, FSEEKI64), vec![vec![0x0600_0000, 0x30, 3, 0]]);
        for (stream, at) in [(0, position), (0x0600_0000, 0)] {
            e.mem.set_u32(errno, 0);
            assert_eq!(e.call(0x00ec_d735, &[stream, at]).i32(), -1);
            assert_eq!(e.mem.u32(errno), 0x16);
            e.mem.set_u32(errno, 0);
            assert_eq!(e.call(0x00ec_d7a3, &[stream, at]).i32(), -1);
            assert_eq!(e.mem.u32(errno), 0x16);
        }
    }

    #[test]
    fn setvbuf_checks_mode_and_size_then_sets_the_buffer() {
        let (mut e, errno) = stream_engine();
        nothing(&mut e, &[FLUSH, FREEBUF]);
        let block = e.mem.alloc(0x100);
        e.register_double(MALLOC_CRT, move |e, a| {
            if a[0] == 0x40 {
                0u32.into_ret()
            } else {
                assert_eq!(e.mem.u32(CFLUSH), 0);
                block.into_ret()
            }
        });
        let stream = e.mem.alloc(0x20);
        // Refused: a bad mode, a size below 2 or above 0x7FFFFFFF for the buffered modes.
        for (mode, size) in [(1u32, 0x100u32), (0, 1), (0x40, 0x8000_0000), (0x40, 0)] {
            e.mem.set_u32(errno, 0);
            assert_eq!(e.call(0x00ec_d7e8, &[stream, 0, mode, size]).i32(), -1);
            assert_eq!(e.mem.u32(errno), 0x16);
        }
        assert_eq!(e.call(0x00ec_d7e8, &[0, 0, 4, 0]).i32(), -1);
        // _IONBF takes any size and uses the stream's own buffer.
        e.mem.set_u32(stream + 0xc, 0xffff_ffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d7e8, &[stream, 0x0700_0000, 4, 0]).i32(), 0);
        assert_eq!(e.mem.u32(stream + 0xc), 0xffff_c2f7);
        assert_eq!(e.mem.u32(stream), stream + 0x14);
        assert_eq!(e.mem.u32(stream + 8), stream + 0x14);
        assert_eq!(e.mem.u32(stream + 0x18), 2);
        assert_eq!(e.mem.u32(stream + 4), 0);
        assert_eq!(calls(&e, FLUSH), vec![vec![stream]]);
        assert_eq!(calls(&e, FREEBUF), vec![vec![stream]]);
        assert_eq!(calls(&e, SETVBUF_FINALLY), vec![vec![stream]]);
        // A given buffer, the size made even.
        e.mem.set_u32(stream + 0xc, 0x3d0c);
        e.mem.set_u32(stream + 4, 9);
        assert_eq!(
            e.call(0x00ec_d7e8, &[stream, 0x0700_0000, 0, 0x101]).i32(),
            0
        );
        assert_eq!(e.mem.u32(stream + 0xc), 0x500);
        assert_eq!(e.mem.u32(stream), 0x0700_0000);
        assert_eq!(e.mem.u32(stream + 0x18), 0x100);
        assert_eq!(e.mem.u32(stream + 4), 0);
        // No buffer: _malloc_crt makes one.
        e.mem.set_u32(stream + 0xc, 0);
        assert_eq!(e.call(0x00ec_d7e8, &[stream, 0, 0x40, 0x81]).i32(), 0);
        assert_eq!(e.mem.u32(stream + 0xc), 0x408);
        assert_eq!(e.mem.u32(stream), block);
        assert_eq!(e.mem.u32(stream + 0x18), 0x80);
        // _malloc_crt fails: _cflush counted, -1, the buffer words left alone.
        e.mem.set_u32(stream + 0xc, 0);
        e.mem.set_u32(stream + 4, 9);
        assert_eq!(e.call(0x00ec_d7e8, &[stream, 0, 0, 0x40]).i32(), -1);
        assert_eq!(e.mem.u32(CFLUSH), 1);
        assert_eq!(e.mem.u32(stream + 4), 9);
        assert_eq!(e.mem.u32(stream + 0xc), 0);
        assert_eq!(calls(&e, SETVBUF_FINALLY).len(), 4);
    }
    #[test]
    fn the_small_blocks_make_their_one_call() {
        let mut e = Engine::new();
        nothing(&mut e, &[DOEXIT, AMSG_EXIT, UNLOCK]);
        e.call_log = Some(vec![]);
        e.call(0x00ec_cf00, &[]);
        e.call(0x00ee_1a8f, &[]);
        for a in [
            0x00ec_d0a1,
            0x00ec_d2e7,
            0x00ec_d3b9,
            0x00ee_16de,
            0x00ee_3114,
        ] {
            e.call(a, &[]);
        }
        assert_eq!(calls(&e, DOEXIT), vec![vec![0, 1, 1]]);
        assert_eq!(calls(&e, AMSG_EXIT), vec![vec![2]]);
        assert_eq!(
            calls(&e, UNLOCK),
            vec![vec![4], vec![4], vec![4], vec![4], vec![0xb]]
        );
    }

    #[test]
    fn the_dynamic_cast_filter_takes_only_access_violations() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x50);
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, record);
        e.mem.set_u32(record, 0xc000_0005);
        assert_eq!(e.call(0x00ec_44a1, &[pointers]).u32(), 1);
        e.mem.set_u32(record, 0xc000_0094);
        assert_eq!(e.call(0x00ec_44a1, &[pointers]).u32(), 0);
    }

    #[test]
    fn bad_exception_destructor_stores_its_vtable() {
        let mut e = Engine::new();
        e.register(EXCEPTION_DTOR, |_, a| (a[0] + 1).into_ret());
        let x = e.mem.alloc(12);
        assert_eq!(e.call(0x00ec_e2f0, &[x]).u32(), x + 1);
        assert_eq!(e.mem.u32(x), BAD_EXCEPTION_VTABLE);
    }
}
