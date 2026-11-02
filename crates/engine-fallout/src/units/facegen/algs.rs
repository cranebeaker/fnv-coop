//! `FaceGen/algs.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::facegen::binaryfile::{string_copy_arg, StringArg};

// Translated from 00acbb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `algs.obj`). Thiscall that ignores
/// `this` and returns 0.
pub fn fn_00acbb70(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// The functions below are the C++ runtime's stream and locale code the map places in this
// unit (Ghidra's library matches name some of them; the RTTI of the vtables they store names
// the classes: `basic_streambuf<char>` `010a30fc`, `basic_filebuf<char>` `010a3138`,
// `basic_ostream<char>` `010a318c`, `basic_ios<char>` `010a2fa4`, `ios_base` `010a2f9c`,
// `ctype<char>` `010a31c4`, `ctype_base` `010a31f4`, `locale::facet` `010a31fc`,
// `runtime_error` `010a307c`, `ios_base::failure` `010a3070`).

/// `operator new` (`00401000`) and `operator delete` (`00401030`), cdecl, one word.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `std::_Lockit::_Lockit` (`00ec32e3`, thiscall, one word: the lock kind) and its destructor
/// (`00ec330b`, thiscall).
const LOCKIT_CTOR: u32 = 0x00ec_32e3;
const LOCKIT_DTOR: u32 = 0x00ec_330b;
/// `_CxxThrowException` (`00ec7894`, two words: the object and the throw information); it
/// does not return in the game.
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `0044a4c0` (thiscall, one word: a C string): constructs a string from it.
const STRING_FROM_C_0044A4C0: u32 = 0x0044_a4c0;
/// `0044a500` (thiscall, one word: the string to copy): copy-constructs a string.
const STRING_COPY_0044A500: u32 = 0x0044_a500;
/// `0044a730` (thiscall, two words): the string tidy the stream code calls with `(0, 0)` on a
/// new string and `(1, 0)` to destroy one.
const STRING_TIDY_0044A730: u32 = 0x0044_a730;
/// `0044a580` (thiscall, three words: source string, offset, count): assigns part of a string.
const STRING_ASSIGN_0044A580: u32 = 0x0044_a580;
/// `npos` of the string class (`.rdata`).
const STRING_NPOS_010173F0: u32 = 0x0101_73f0;
/// `00ec78e0` (thiscall, no stack words): the `std::exception` base constructor of the
/// stream failure object.
const EXCEPTION_CTOR_00EC78E0: u32 = 0x00ec_78e0;
/// The vtables `ios_base::clear` stores in its failure object (RTTI `runtime_error`,
/// `ios_base::failure`) and the failure's throw information.
const VTABLE_RUNTIME_ERROR_010A307C: u32 = 0x010a_307c;
const VTABLE_IOS_FAILURE_010A3070: u32 = 0x010a_3070;
const IOS_FAILURE_THROW_INFO_0116C3A4: u32 = 0x0116_c3a4;
/// The messages: `"ios_base::badbit set"`, `"ios_base::failbit set"`, `"ios_base::eofbit set"`.
const MESSAGE_BADBIT_010A3084: u32 = 0x010a_3084;
const MESSAGE_FAILBIT_010A3054: u32 = 0x010a_3054;
const MESSAGE_EOFBIT_010A303C: u32 = 0x010a_303c;
/// `std::locale::_Init` (`00ec36a6`, cdecl, no arguments) and the global locale getter
/// `00ec34d4` (cdecl, no arguments).
const LOCALE_INIT: u32 = 0x00ec_36a6;
const GLOBAL_LOCALE_00EC34D4: u32 = 0x00ec_34d4;
/// Vtables stored by the stream buffer and file buffer destructors and constructors (RTTI
/// `basic_streambuf<char>`, `basic_filebuf<char>`).
const VTABLE_STREAMBUF_010A30FC: u32 = 0x010a_30fc;
const VTABLE_FILEBUF_010A3138: u32 = 0x010a_3138;
/// Vtables of `ctype<char>`, `ctype_base` and `locale::facet`.
const VTABLE_CTYPE_010A31C4: u32 = 0x010a_31c4;
const VTABLE_CTYPE_BASE_010A31F4: u32 = 0x010a_31f4;
const VTABLE_FACET_010A31FC: u32 = 0x010a_31fc;

// Translated from 00ac9da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map names this body `std::getline<...>` (`cg`), but it is a forwarder (thiscall, one
/// word: a string; `RET 4`): the string's characters are inline at `+4` when its capacity
/// (`+0x18`) is below 0x10, else at the pointer at `+4`; it calls `00ac6100` (thiscall, four
/// words) on `this` with `(0, this[+0x14], characters, length)`, the length being the
/// string's `+0x14`, and returns that call's `EAX`.
pub fn fn_00ac9da0(e: &mut Engine, this: Ptr, string: Ptr) -> u32 {
    let string = string.addr();
    let length = e.mem.u32(string + 0x14);
    let characters = if e.mem.u32(string + 0x18) < 0x10 {
        string + 4
    } else {
        e.mem.u32(string + 4)
    };
    let own = e.mem.u32(this.addr() + 0x14);
    e.call(0x00ac_6100, &args![this, 0u32, own, characters, length])
        .u32()
}

/// The throw at the end of [`ios_base_clear`]: builds the message string (0x1c bytes) and the
/// `ios_base::failure` object (0x28 bytes, its string at `+0xc`) on the stack and throws the
/// object. The game does not return from the throw; the code after it destroys the message
/// string (`0044a730(1, 0)`) and leaves.
fn throw_ios_failure(e: &mut Engine, message: u32) {
    e.with_stack(0x44, |e, frame| {
        let string = frame.addr();
        let object = frame.addr() + 0x1c;
        e.call(STRING_FROM_C_0044A4C0, &args![string, message]);
        e.call(EXCEPTION_CTOR_00EC78E0, &args![object]);
        e.mem.set_u32(object, VTABLE_RUNTIME_ERROR_010A307C);
        e.call(STRING_COPY_0044A500, &args![object + 0xc, string]);
        e.mem.set_u32(object, VTABLE_IOS_FAILURE_010A3070);
        e.call(
            CXX_THROW_EXCEPTION,
            &args![object, IOS_FAILURE_THROW_INFO_0116C3A4],
        );
        e.call(STRING_TIDY_0044A730, &args![string, 1u32, 0u32]);
    });
}

// Translated from 00ac9ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac9ff0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `std::ios_base::clear(state, reraise)` (the map's name; thiscall, two words, `RET 8`):
/// stores `state & 0x17` at `+8`; returns when it has no bit in common with the exception mask
/// at `+0xc`. Otherwise, with the `reraise` byte set it rethrows (`_CxxThrowException(0, 0)`);
/// else it throws an `ios_base::failure` saying `badbit` (state bit 4), `failbit` (bit 2) or
/// `eofbit`. The throws do not return in the game; the code after each returns.
pub fn ios_base_clear(e: &mut Engine, this: Ptr, state: u32, reraise: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 8, state & 0x17);
    if e.mem.u32(base + 8) & e.mem.u32(base + 0xc) == 0 {
        return;
    }
    if reraise as u8 != 0 {
        e.call(CXX_THROW_EXCEPTION, &args![0u32, 0u32]);
        return;
    }
    if e.mem.u32(base + 8) & e.mem.u32(base + 0xc) & 4 != 0 {
        throw_ios_failure(e, MESSAGE_BADBIT_010A3084);
        return;
    }
    if e.mem.u32(base + 8) & e.mem.u32(base + 0xc) & 2 != 0 {
        throw_ios_failure(e, MESSAGE_FAILBIT_010A3054);
        return;
    }
    throw_ios_failure(e, MESSAGE_EOFBIT_010A303C);
}

// Translated from 00acb6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf<char>` destructor body (thiscall). Stores the vtable
/// `010a30fc`; when the locale holder at `+0x38` is set it releases its facet
/// ([`fn_00acb720`]) and frees the holder (`operator delete`; the compiler's always-true
/// delete flag is kept as such); then destroys the mutex at `+4` (`00ec3260`, thiscall).
pub fn fn_00acb6b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_STREAMBUF_010A30FC);
    let holder = e.mem.u32(base + 0x38);
    if holder != 0 {
        fn_00acb720(e, Ptr::new(holder));
        // `MOV ECX,1; AND ECX,1; JZ`: the delete flag is the constant 1.
        let delete_flag = std::hint::black_box(1u32);
        if delete_flag & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![holder]);
        }
    }
    e.call(0x00ec_3260, &args![base + 4]);
}

// Translated from 00acb720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the locale destructor shape (thiscall on a one-word holder). When the facet
/// pointer at `+0` is set it drops a reference ([`facet_decref`]); when that returns the facet
/// (no references left) it calls the facet's slot 0 (its deleting destructor) with 1.
pub fn fn_00acb720(e: &mut Engine, this: Ptr) {
    let facet = e.mem.u32(this.addr());
    if facet != 0 {
        let released = facet_decref(e, Ptr::new(facet));
        if released.addr() != 0 {
            e.vcall(released.addr(), 0, &args![1u32]);
        }
    }
}

// Translated from 00acb770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::locale::facet::_Decref` (Ghidra library match; thiscall). Under a `_Lockit(0)` (4
/// bytes on the stack) it decrements the reference count at `+4` unless it is 0 or
/// `0xffffffff`; returns `this` when the count is then 0, else 0.
pub fn facet_decref(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.with_stack(4, |e, lock| {
        e.call(LOCKIT_CTOR, &args![lock, 0u32]);
        let count = e.mem.u32(base + 4);
        if count > 0 && count < 0xffff_ffff {
            e.mem.set_u32(base + 4, count - 1);
        }
        let result = if e.mem.u32(base + 4) == 0 { base } else { 0 };
        e.call(LOCKIT_DTOR, &args![lock]);
        Ptr::new(result)
    })
}

// Translated from 00acb7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: thiscall that ignores `this` and returns `0xffffffff` (`OR EAX,-1`).
pub fn fn_00acb7e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0xffff_ffff
}

// Translated from 00acc970 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acc970 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_filebuf<char>` destructor body (thiscall). Stores the vtable `010a3138`;
/// when the byte at `+0x48` is set calls `00acfb90` (thiscall) on `this`; then runs the
/// stream-buffer destructor body [`fn_00acb6b0`].
pub fn fn_00acc970(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_FILEBUF_010A3138);
    if e.mem.u8(base + 0x48) != 0 {
        e.call(0x00ac_fb90, &args![base]);
    }
    fn_00acb6b0(e, this);
}

// Translated from 00ace2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a `basic_ostream<char>` destructor with its virtual base (thiscall). Reads the
/// virtual-base offset from `[[this] + 4]` and stores the vtable `010a318c` at `this + offset`;
/// then stores `010a2fa4` and `010a2f9c` (RTTI `basic_ios`, `ios_base`) at `+4` and destroys
/// the `ios_base` at `+4` (`00ec3205`, cdecl, one word).
pub fn fn_00ace2d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let offset = e.mem.u32(e.mem.u32(base).wrapping_add(4));
    e.mem.set_u32(base.wrapping_add(offset), 0x010a_318c);
    e.mem.set_u32(base + 4, 0x010a_2fa4);
    e.mem.set_u32(base + 4, 0x010a_2f9c);
    e.call(0x00ec_3205, &args![base + 4]);
}

// Translated from 00ace750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ios_base::getloc` shape (thiscall, one word: the locale it fills; `RET 4`).
/// Copies the word of the locale held at `+0x24` into `[out]`, adds a reference to the object
/// it points at ([`facet_incref`]) and returns `out`.
pub fn fn_00ace750(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let holder = e.mem.u32(this.addr() + 0x24);
    let facet = e.mem.u32(holder);
    e.mem.set_u32(out.addr(), facet);
    let facet = e.mem.u32(out.addr());
    facet_incref(e, Ptr::new(facet));
    out
}

// Translated from 00ace790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::locale::facet::_Incref` (Ghidra library match; thiscall). Under a `_Lockit(0)` it
/// increments the reference count at `+4` unless it is `0xffffffff`.
pub fn facet_incref(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, lock| {
        e.call(LOCKIT_CTOR, &args![lock, 0u32]);
        let count = e.mem.u32(base + 4);
        if count < 0xffff_ffff {
            e.mem.set_u32(base + 4, count + 1);
        }
        e.call(LOCKIT_DTOR, &args![lock]);
    });
}

/// The word a file buffer copies to `+0x44` when it opens (`.data`).
const INITIAL_STATE_011F6D8C: u32 = 0x011f_6d8c;

// Translated from 00acfc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_filebuf` `_Init(file, mode)` shape (thiscall, two words, `RET 8`).
/// Sets the byte at `+0x48` to `mode == 1` and the byte at `+0x41` to 0 and resets the buffer
/// pointers ([`fn_00ad0720`]); with a file it points the pointer slots at the file's own
/// (`+0x10`/`+0x14` at `file + 8`, `+0x20`/`+0x24` at `file`, `+0x30`/`+0x34` at `file + 4`;
/// the compiler's always-true test is kept). Stores the file at `+0x4c`, the word at
/// `011f6d8c` at `+0x44` and 0 at `+0x3c`.
pub fn fn_00acfc10(e: &mut Engine, this: Ptr, file: u32, mode: u32) {
    let base = this.addr();
    e.mem.set_u8(base + 0x48, (mode == 1) as u8);
    e.mem.set_u8(base + 0x41, 0);
    fn_00ad0720(e, this);
    // `MOV EAX,1; TEST EAX,EAX; JZ`: the second condition is the constant 1.
    let always = std::hint::black_box(1u32);
    if file != 0 && always != 0 {
        let pointers = file.wrapping_add(8);
        let middle = file.wrapping_add(4);
        e.mem.set_u32(base + 0x10, pointers);
        e.mem.set_u32(base + 0x14, pointers);
        e.mem.set_u32(base + 0x20, file);
        e.mem.set_u32(base + 0x24, file);
        e.mem.set_u32(base + 0x30, middle);
        e.mem.set_u32(base + 0x34, middle);
    }
    e.mem.set_u32(base + 0x4c, file);
    let state = e.global::<u32>(INITIAL_STATE_011F6D8C);
    e.mem.set_u32(base + 0x44, state);
    e.mem.set_u32(base + 0x3c, 0);
}

// Translated from 00ad0470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_ios::init(buffer, add_standard)` shape (thiscall, two words, `RET 8`).
/// Initialises the `ios_base` part ([`fn_00ad0550`]), stores the stream buffer at `+0x28` and 0
/// at `+0x2c`, and the fill character `widen(' ')` ([`fn_00ad30c0`] with 0x20) as the byte at
/// `+0x30`. Without a buffer it sets `badbit` (`clear(state | 4, 0)`; the compiler's
/// always-true test is kept). With the `add_standard` byte set it registers the stream
/// (`00ec31b3`, cdecl, one word); otherwise it clears `+4`.
pub fn fn_00ad0470(e: &mut Engine, this: Ptr, buffer: u32, add_standard: u32) {
    let base = this.addr();
    fn_00ad0550(e, this);
    e.mem.set_u32(base + 0x28, buffer);
    e.mem.set_u32(base + 0x2c, 0);
    let fill = fn_00ad30c0(e, this, 0x20);
    e.mem.set_u8(base + 0x30, fill as u8);
    // `MOV EAX,4; TEST EAX,EAX; JZ`: the second condition is the constant 4.
    let always = std::hint::black_box(4u32);
    if e.mem.u32(base + 0x28) == 0 && always != 0 {
        let state = e.mem.u32(base + 8) | 4;
        let state = if e.mem.u32(base + 0x28) == 0 {
            state | 4
        } else {
            state
        };
        ios_base_clear(e, this, state, 0);
    }
    if add_standard as u8 != 0 {
        e.call(0x00ec_31b3, &args![base]);
    } else {
        e.mem.set_u32(base + 4, 0);
    }
}

/// The 4-byte locale both stream constructors make: `operator new(4)`; when that succeeds its
/// word is `locale::_Init()` and the global locale (`00ec34d4`) gets a reference
/// ([`facet_incref`]). Returns the block, or 0.
fn new_locale(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![4u32]).u32();
    if block == 0 {
        return 0;
    }
    let implementation = e.call(LOCALE_INIT, &args![]).u32();
    e.mem.set_u32(block, implementation);
    let global = e.call(GLOBAL_LOCALE_00EC34D4, &args![]).u32();
    facet_incref(e, Ptr::new(global));
    block
}

// Translated from 00ad0550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ios_base::_Init` shape (thiscall). Clears `+0x24`, `+4` and `+0xc`, stores
/// the format flags 0x201 at `+0x10` and the precision 6 at `+0x14`, clears `+0x18` to `+0x20`,
/// runs `clear(0, 0)` ([`ios_base_clear`]) and stores a new locale (see [`new_locale`]) at
/// `+0x24`.
pub fn fn_00ad0550(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0x201);
    e.mem.set_u32(base + 0x14, 6);
    e.mem.set_u32(base + 0x18, 0);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0);
    ios_base_clear(e, this, 0, 0);
    let locale = new_locale(e);
    e.mem.set_u32(base + 0x24, locale);
}

// Translated from 00ad0630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad0630 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_streambuf<char>` constructor (thiscall). Stores the vtable `010a30fc`,
/// constructs the mutex at `+4` (`00ec3246`, thiscall), stores a new locale (see
/// [`new_locale`]) at `+0x38` and resets the buffer pointers ([`fn_00ad0720`]). Returns
/// `this`.
pub fn fn_00ad0630(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_STREAMBUF_010A30FC);
    e.call(0x00ec_3246, &args![base + 4]);
    let locale = new_locale(e);
    e.mem.set_u32(base + 0x38, locale);
    fn_00ad0720(e, this);
    this
}

// Translated from 00ad0720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the stream buffer's `_Init()` (thiscall). Points the six pointer slots at its own
/// storage (`+0x10` at `+8`, `+0x14` at `+0xc`, `+0x20` at `+0x18`, `+0x24` at `+0x1c`, `+0x30`
/// at `+0x28`, `+0x34` at `+0x2c`), then clears the words they point at, in the exe's order
/// (through `+0x14`, `+0x24`, `+0x34`, `+0x10`, `+0x20`, `+0x30`).
pub fn fn_00ad0720(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x10, base + 8);
    e.mem.set_u32(base + 0x14, base + 0xc);
    e.mem.set_u32(base + 0x20, base + 0x18);
    e.mem.set_u32(base + 0x24, base + 0x1c);
    e.mem.set_u32(base + 0x30, base + 0x28);
    e.mem.set_u32(base + 0x34, base + 0x2c);
    for slot in [0x14u32, 0x24, 0x34, 0x10, 0x20, 0x30] {
        let target = e.mem.u32(base + slot);
        e.mem.set_u32(target, 0);
    }
}

// Translated from 00ad30c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad30c0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// The map names this body `std::ios_base::eof` (`cg`); it is the `basic_ios::widen(ch)` shape
/// (thiscall, one word, `RET 4`): gets the stream's locale into a 4-byte local
/// ([`fn_00ace750`]), its `ctype<char>` facet ([`fn_00ad38d0`]), releases the local
/// ([`fn_00acb720`]) and returns the facet's slot `0x18` called with the character byte
/// (zero-extended).
pub fn fn_00ad30c0(e: &mut Engine, this: Ptr, character: u32) -> u32 {
    e.with_stack(4, |e, local| {
        let locale = fn_00ace750(e, this, local);
        let facet = fn_00ad38d0(e, locale);
        fn_00acb720(e, local);
        e.vcall(facet.addr(), 0x18, &args![character & 0xff]).u32()
    })
}

// Translated from 00ad33c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad33c0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed (thiscall, one stack word it does not read, `RET 4`): allocates one word through
/// `0044d670` (cdecl; called with the two words 1 and 0), stores `this` in it when the
/// allocation succeeded, and stores the block (or 0) at `+0`. Returns `this`.
pub fn fn_00ad33c0(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    let block = e.call(0x0044_d670, &args![1u32, 0u32]).u32();
    let stored = if block != 0 {
        e.mem.set_u32(block, this.addr());
        block
    } else {
        0
    };
    e.mem.set_u32(this.addr(), stored);
    this
}

/// The cached `ctype<char>` facet, its facet id and the facet id counter (`.data`).
const CACHED_CTYPE_011F6D90: u32 = 0x011f_6d90;
const CTYPE_ID_0126FBA4: u32 = 0x0126_fba4;
const FACET_ID_COUNTER_0126FBA0: u32 = 0x0126_fba0;

// Translated from 00ad38d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad38d0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `std::use_facet<ctype<char>>(locale)` (cdecl, one word). Under a `_Lockit(0)` it
/// reads the cached facet `011f6d90`; when the facet id `0126fba4` is 0 it gives it the next
/// id from `0126fba0` (checked again under a second `_Lockit(0)`). It looks the id up in the
/// locale ([`locale_getfacet`]); when that finds none it takes the cached facet, or when there
/// is none makes one ([`fn_00ad3aa0`] on the cache local): a result of -1 throws `bad_cast`
/// (`00ec79c4` with `"bad cast"` `010a31b4`, throw information `0116cdb8`; the game does not
/// return, the code after it returns 0), otherwise the new facet becomes the cached one, gets
/// a reference ([`facet_incref`]) and is registered (`00ec3599`, cdecl). Returns the facet.
pub fn fn_00ad38d0(e: &mut Engine, locale: Ptr) -> Ptr {
    e.with_stack(0x20, |e, frame| {
        // Frame: the inner lock at +0, the `bad_cast` object at +8 (0xc bytes), the cache local
        // at +0x18 and the outer lock at +0x1c.
        let inner_lock = frame.addr();
        let bad_cast = frame.addr() + 8;
        let cached_at = frame.addr() + 0x18;
        let outer_lock = frame.addr() + 0x1c;
        e.call(LOCKIT_CTOR, &args![outer_lock, 0u32]);
        let cached = e.global::<u32>(CACHED_CTYPE_011F6D90);
        e.mem.set_u32(cached_at, cached);
        if e.global::<u32>(CTYPE_ID_0126FBA4) == 0 {
            e.call(LOCKIT_CTOR, &args![inner_lock, 0u32]);
            if e.global::<u32>(CTYPE_ID_0126FBA4) == 0 {
                let next = e.global::<u32>(FACET_ID_COUNTER_0126FBA0).wrapping_add(1);
                e.set_global(FACET_ID_COUNTER_0126FBA0, next);
                let id = e.global::<u32>(FACET_ID_COUNTER_0126FBA0);
                e.set_global(CTYPE_ID_0126FBA4, id);
            }
            e.call(LOCKIT_DTOR, &args![inner_lock]);
        }
        let id = e.global::<u32>(CTYPE_ID_0126FBA4);
        let mut facet = locale_getfacet(e, locale, id);
        if facet == 0 {
            let cached = e.mem.u32(cached_at);
            if cached != 0 {
                facet = cached;
            } else if fn_00ad3aa0(e, Ptr::new(cached_at), locale) == 0xffff_ffff {
                e.call(0x00ec_79c4, &args![bad_cast, 0x010a_31b4u32]);
                e.call(CXX_THROW_EXCEPTION, &args![bad_cast, 0x0116_cdb8u32]);
            } else {
                facet = e.mem.u32(cached_at);
                let made = e.mem.u32(cached_at);
                e.set_global(CACHED_CTYPE_011F6D90, made);
                let made = e.mem.u32(cached_at);
                facet_incref(e, Ptr::new(made));
                e.call(0x00ec_3599, &args![made]);
            }
        }
        e.call(LOCKIT_DTOR, &args![outer_lock]);
        Ptr::new(facet)
    })
}

// Translated from 00ad3a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::locale::_Getfacet(id)` (Ghidra library match; thiscall, one word, `RET 4`): the
/// entry `id` of the implementation's facet table (`+8`, count `+0xc`) when `id` is below the
/// count, else 0; when that is 0 and the implementation's byte at `+0x14` is set, the same
/// lookup in the global locale implementation (`00ec34d4`).
pub fn locale_getfacet(e: &mut Engine, this: Ptr, id: u32) -> u32 {
    let implementation = e.mem.u32(this.addr());
    let found = if id < e.mem.u32(implementation + 0xc) {
        let table = e.mem.u32(implementation + 8);
        e.mem.u32(table.wrapping_add(id.wrapping_mul(4)))
    } else {
        0
    };
    if found != 0 || e.mem.u8(e.mem.u32(this.addr()) + 0x14) == 0 {
        return found;
    }
    let global = e.call(GLOBAL_LOCALE_00EC34D4, &args![]).u32();
    if id < e.mem.u32(global + 0xc) {
        let table = e.mem.u32(global + 8);
        e.mem.u32(table.wrapping_add(id.wrapping_mul(4)))
    } else {
        0
    }
}

// Translated from 00ad3aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad3aa0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `ctype<char>::_Getcat(slot, locale)` (cdecl, two words). When the slot is set and
/// holds 0 it allocates 0x18 bytes; when that succeeds it takes the locale's name
/// ([`fn_00ad3e60`] into a stack string), builds a `_Locinfo` from it ([`fn_00ad3ca0`]), stores
/// the vtables `010a31fc`, `010a31f4`, `010a31c4` (0 at `+4` after the first) and copies the
/// four words of `__Getctype` (`00ec38a9` on a stack `_Ctypevec`) to `+8`. The block (or 0)
/// goes into the slot; the `_Locinfo` ([`fn_00ad3db0`]) and the string (`0044a730(1, 0)`) are
/// destroyed when they were built. Returns 2.
pub fn fn_00ad3aa0(e: &mut Engine, slot: Ptr, locale: Ptr) -> u32 {
    if slot.addr() == 0 || e.mem.u32(slot.addr()) != 0 {
        return 2;
    }
    e.with_stack(0xf4, |e, frame| {
        // Frame: the `_Ctypevec` at +0 (0x10 bytes), the name string at +0x64, the `_Locinfo`
        // at +0x80.
        let ctype_vector = frame.addr();
        let string = frame.addr() + 0x64;
        let locinfo = frame.addr() + 0x80;
        let mut built = 0u32;
        let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
        let made = if block != 0 {
            let name = fn_00ad3e60(e, locale, Ptr::new(string));
            built |= 1;
            fn_00ad3ca0(e, Ptr::new(locinfo), name.addr());
            built |= 2;
            e.mem.set_u32(block, VTABLE_FACET_010A31FC);
            e.mem.set_u32(block + 4, 0);
            e.mem.set_u32(block, VTABLE_CTYPE_BASE_010A31F4);
            e.mem.set_u32(block, VTABLE_CTYPE_010A31C4);
            let vector = e.call(0x00ec_38a9, &args![ctype_vector]).u32();
            let words = [
                e.mem.u32(vector),
                e.mem.u32(vector + 4),
                e.mem.u32(vector + 8),
                e.mem.u32(vector + 0xc),
            ];
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(block + 8 + 4 * i as u32, *word);
            }
            block
        } else {
            0
        };
        e.mem.set_u32(slot.addr(), made);
        if built & 2 != 0 {
            built &= !2;
            fn_00ad3db0(e, Ptr::new(locinfo));
        }
        if built & 1 != 0 {
            e.call(STRING_TIDY_0044A730, &args![string, 1u32, 0u32]);
        }
    });
    2
}

// Translated from 00ad3ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad3ca0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `_Locinfo(name)` constructor (thiscall, one word: the name string; `RET 4`).
/// Constructs its `_Lockit(0)` at `+0`, initialises the strings at `+4`, `+0x20`, `+0x3c` and
/// `+0x58` (`0044a730(0, 0)`), then runs `_Locinfo_ctor(this, name)` (`00ec3740`, cdecl).
/// Returns `this`.
pub fn fn_00ad3ca0(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    let base = this.addr();
    e.call(LOCKIT_CTOR, &args![base, 0u32]);
    for offset in [4u32, 0x20, 0x3c, 0x58] {
        e.call(STRING_TIDY_0044A730, &args![base + offset, 0u32, 0u32]);
    }
    e.call(0x00ec_3740, &args![base, name]);
    this
}

// Translated from 00ad3db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad3db0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `_Locinfo` destructor (thiscall): `_Locinfo_dtor(this)` (`00ec35d7`, cdecl),
/// destroys the strings at `+0x58`, `+0x3c`, `+0x20` and `+4` (`0044a730(1, 0)`) and the
/// `_Lockit` at `+0`.
pub fn fn_00ad3db0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x00ec_35d7, &args![base]);
    for offset in [0x58u32, 0x3c, 0x20, 4] {
        e.call(STRING_TIDY_0044A730, &args![base + offset, 1u32, 0u32]);
    }
    e.call(LOCKIT_DTOR, &args![base]);
}

// Translated from 00ad3e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `locale::name()` (thiscall, one word: the string it fills; `RET 4`). Initialises
/// the string (`0044a730(0, 0)`), assigns it the implementation's name string at `+0x18`
/// (`0044a580(source, 0, npos)`, `npos` read from `010173f0`) and returns it.
pub fn fn_00ad3e60(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let source = e.mem.u32(this.addr()).wrapping_add(0x18);
    e.call(STRING_TIDY_0044A730, &args![out, 0u32, 0u32]);
    let npos = e.global::<u32>(STRING_NPOS_010173F0);
    e.call(STRING_ASSIGN_0044A580, &args![out, source, 0u32, npos]);
    out
}

// Translated from 00ad4000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall, one word, `RET 4`): returns the low byte of its argument (`MOV AL`), the
/// shape of `ctype<char>::do_widen`.
pub fn fn_00ad4000(_e: &mut Engine, _this: Ptr, value: u32) -> u8 {
    value as u8
}

// Translated from 00ad4110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad4110 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `ctype<char>` scalar deleting destructor (thiscall, one word, `RET 4`). Stores
/// the vtable `010a31c4`, frees the table ([`ctype_tidy`]), stores `010a31f4` and `010a31fc`
/// and with flag bit 0 frees `this` (`operator delete`). Returns `this`.
pub fn fn_00ad4110(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_CTYPE_010A31C4);
    ctype_tidy(e, this);
    e.mem.set_u32(base, VTABLE_CTYPE_BASE_010A31F4);
    e.mem.set_u32(base, VTABLE_FACET_010A31FC);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![base]);
    }
    this
}

// Translated from 00ad41c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ctype<char>::_Tidy` (Ghidra library match; thiscall): with a positive delete flag at
/// `+0x14` frees the table at `+0x10` with `_free` (`00ecd291`), with a negative one with
/// `operator delete`.
pub fn ctype_tidy(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let flag = e.mem.u32(base + 0x14) as i32;
    if flag > 0 {
        let table = e.mem.u32(base + 0x10);
        e.call(0x00ec_d291, &args![table]);
    } else if flag < 0 {
        let table = e.mem.u32(base + 0x10);
        e.call(OPERATOR_DELETE, &args![table]);
    }
}

// Functions of task n060 (`00ac79e0` to `00ad6b60`). Most are the C++ runtime's stream,
// string-container and tree code the map places in this unit; the helpers and constants just
// below are shared by them.

/// `fr2Assert` (`00aafb90`, cdecl: file name, line) and the file name it is given here
/// (`"D:\_Fallout3\...\FaceGen\algs.cpp"`, `.rdata`).
const FR2_ASSERT: u32 = 0x00aa_fb90;
const ALGS_CPP_010A2F50: u32 = 0x010a_2f50;
/// The CRT's `_isnan` (`00ec75b1`), `_finite` (`00ec7595`) and `fabs` (`00ec6cde`): cdecl,
/// one `double` (two words); `fabs` returns in `ST0`.
const CRT_ISNAN: u32 = 0x00ec_75b1;
const CRT_FINITE: u32 = 0x00ec_7595;
const CRT_FABS: u32 = 0x00ec_6cde;
/// The two floats [`fr2_add_saturated_float`] returns for an infinite sum (`.rdata`; the
/// decompiler shows them as `3.4028235e+38` and `-3.4028235e+38`).
const SATURATE_HIGH_01016970: u32 = 0x0101_6970;
const SATURATE_LOW_01015F5C: u32 = 0x0101_5f5c;
/// `_invalid_parameter_noinfo` (`00ec7c56`, no arguments): the checked containers call it
/// when an iterator or index is out of range.
const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// `ios_base::_Ios_base_dtor` (`00ec3205`, cdecl, one word).
const IOS_BASE_DTOR: u32 = 0x00ec_3205;
/// The `ios_base` and `basic_ios<char>` vtables (`.rdata`).
const VTABLE_IOS_BASE_010A2F9C: u32 = 0x010a_2f9c;
const VTABLE_BASIC_IOS_010A2FA4: u32 = 0x010a_2fa4;
/// `std::exception::~exception` (`00ec79a1`, thiscall) and its copy constructor (`00ec7944`,
/// thiscall, one word: the source).
const EXCEPTION_DTOR: u32 = 0x00ec_79a1;
const EXCEPTION_COPY: u32 = 0x00ec_7944;
/// The stream buffer's mutex unlock (`00ec3280`, thiscall).
const MUTEX_UNLOCK: u32 = 0x00ec_3280;
/// `std::uncaught_exception` (`00ec3241`, no arguments; the result is `AL`).
const UNCAUGHT_EXCEPTION: u32 = 0x00ec_3241;
/// `0044d670` (cdecl; the containers call it with the two words 1 and 0): allocates the
/// one-word container proxy.
const ALLOCATE_PROXY_0044D670: u32 = 0x0044_d670;

/// The `basic_ios` part of a stream with a virtual base: `this + [[this] + 4]`.
fn stream_base(e: &Engine, this: u32) -> u32 {
    let vbtable = e.mem.u32(this);
    this.wrapping_add(e.mem.u32(vbtable.wrapping_add(4)))
}

/// A checked vector iterator: calls `_invalid_parameter_noinfo` when the vector is null or
/// `at` lies outside `[first, last]` (`+0xc`, `+0x10`), then returns `(proxy, at)`, the proxy
/// being the vector's word at `+0`.
fn vector_iterator(e: &mut Engine, this: u32, at: u32) -> (u32, u32) {
    if this == 0 || e.mem.u32(this + 0xc) > at || at > e.mem.u32(this + 0x10) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    (e.mem.u32(this), at)
}

// Translated from 00ac79e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fr2AddSaturated<float>` (cdecl, two floats): asserts (`fr2Assert`, lines 0x1b2 and 0x1b3)
/// when either is a NaN (`_isnan`), adds them (rounded to a float) and returns the sum when
/// it is finite (`_finite`); otherwise the float at `01016970` when the sum equals its
/// `fabs` rounded to a float, else the float at `01015f5c` (`FUCOMPP`, unordered counts as
/// not equal). The result is `ST0`.
pub fn fr2_add_saturated_float(e: &mut Engine, a: f32, b: f32) -> f64 {
    if e.call(CRT_ISNAN, &args![x87_load32(a.to_bits())]).u32() != 0 {
        e.call(FR2_ASSERT, &args![ALGS_CPP_010A2F50, 0x1b2u32]);
    }
    if e.call(CRT_ISNAN, &args![x87_load32(b.to_bits())]).u32() != 0 {
        e.call(FR2_ASSERT, &args![ALGS_CPP_010A2F50, 0x1b3u32]);
    }
    let sum = x87_store32(x87_add(x87_load32(a.to_bits()), x87_m32(b.to_bits())));
    if e.call(CRT_FINITE, &args![x87_load32(sum)]).u32() != 0 {
        return x87_load32(sum);
    }
    let value = x87_load32(sum);
    let magnitude = x87_store32(e.call(CRT_FABS, &args![value]).f64());
    if value == x87_load32(magnitude) {
        x87_load32(e.global::<u32>(SATURATE_HIGH_01016970))
    } else {
        x87_load32(e.global::<u32>(SATURATE_LOW_01015F5C))
    }
}

// Translated from 00ac9bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac9bd0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the destructor of a pair of strings (thiscall): destroys the string at `+0x1c`,
/// then the one at `+0` (`0044a730(1, 0)`).
pub fn fn_00ac9bd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(STRING_TIDY_0044A730, &args![base + 0x1c, 1u32, 0u32]);
    e.call(STRING_TIDY_0044A730, &args![base, 1u32, 0u32]);
}

// Translated from 00ac9c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ios_base` scalar deleting destructor (the map's vtable name; thiscall, one word,
/// `RET 4`): stores the vtable `010a2f9c`, runs `_Ios_base_dtor` (`00ec3205`) and with flag
/// bit 0 frees `this`. Returns `this`.
pub fn ios_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_IOS_BASE_010A2F9C);
    e.call(IOS_BASE_DTOR, &args![base]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![base]);
    }
    this
}

/// The tail the three stream destructors below share: stores the `basic_ios` then the
/// `ios_base` vtable at `ios` and runs `_Ios_base_dtor` on it.
fn destroy_ios(e: &mut Engine, ios: u32) {
    e.mem.set_u32(ios, VTABLE_BASIC_IOS_010A2FA4);
    e.mem.set_u32(ios, VTABLE_IOS_BASE_010A2F9C);
    e.call(IOS_BASE_DTOR, &args![ios]);
}

// Translated from 00ac9c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a stream's virtual-base destructor (thiscall; `this` is the stream with its
/// virtual base `0x50` above): runs the stream's destructor [`fn_00acb610`] on `this + 0x50`,
/// then destroys the `basic_ios` at the same address (vtables `010a2fa4`, `010a2f9c`,
/// `_Ios_base_dtor`).
pub fn fn_00ac9c70(e: &mut Engine, this: Ptr) {
    let stream = this.addr() + 0x50;
    fn_00acb610(e, Ptr::new(stream));
    destroy_ios(e, stream);
}

// Translated from 00ac9cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: as [`fn_00ac9c70`] with the offset `0x58` and the destructor [`fn_00acc8c0`].
pub fn fn_00ac9cb0(e: &mut Engine, this: Ptr) {
    let stream = this.addr() + 0x58;
    fn_00acc8c0(e, Ptr::new(stream));
    destroy_ios(e, stream);
}

// Translated from 00ac9cf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac9cf0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: a container destructor (thiscall): empties it ([`fn_00acede0`]) and frees its
/// proxy, the word at `+0` (`operator delete`).
pub fn fn_00ac9cf0(e: &mut Engine, this: Ptr) {
    fn_00acede0(e, this);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00ac9d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: as [`fn_00ac9c70`] with the offset `0x54` and the destructor [`fn_00acdeb0`].
pub fn fn_00ac9d60(e: &mut Engine, this: Ptr) {
    let stream = this.addr() + 0x54;
    fn_00acdeb0(e, Ptr::new(stream));
    destroy_ios(e, stream);
}

// Translated from 00ac9df0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac9df0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_ostream<char>::put(ch)` shape (thiscall, one word, `RET 4`). Builds an
/// output sentry on the stack ([`fn_00ad0210`]); when it is good it writes the character byte
/// to the stream buffer (`[base + 0x28]`, [`fn_00ace8c0`]) and an end of file (-1) means
/// `badbit`; a bad sentry means `badbit`. A nonzero state goes to [`fn_00aca4e0`] on the
/// `basic_ios` part. Then, unless `uncaught_exception()` (`00ec3241`), the sentry's stream is
/// flushed when it asks for it ([`fn_00ad03e0`]), and the sentry is destroyed
/// ([`fn_00aca9a0`]). Returns `this`.
pub fn fn_00ac9df0(e: &mut Engine, this: Ptr, character: u32) -> Ptr {
    let stream = this.addr();
    e.with_stack(8, |e, sentry| {
        let mut state = 0u32;
        fn_00ad0210(e, sentry, this);
        if e.mem.u8(sentry.addr() + 4) == 0 {
            state |= 4;
        } else {
            let base = stream_base(e, stream);
            let buffer = e.mem.u32(base + 0x28);
            if fn_00ace8c0(e, Ptr::new(buffer), character & 0xff) == 0xffff_ffff {
                state |= 4;
            }
        }
        let base = stream_base(e, stream);
        fn_00aca4e0(e, Ptr::new(base), state, 0);
        if !e.call(UNCAUGHT_EXCEPTION, &args![]).bool() {
            let owner = e.mem.u32(sentry.addr());
            fn_00ad03e0(e, Ptr::new(owner));
        }
        fn_00aca9a0(e, sentry);
    });
    this
}

// Translated from 00aca1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `runtime_error::what()` shape (thiscall): the characters of the message string
/// at `+0xc`, inline at `+0x10` when its capacity (`+0x24`) is below 0x10, else the pointer
/// at `+0x10`.
pub fn fn_00aca1e0(e: &mut Engine, this: Ptr) -> u32 {
    let string = this.addr() + 0xc;
    if e.mem.u32(string + 0x18) < 0x10 {
        string + 4
    } else {
        e.mem.u32(string + 4)
    }
}

// Translated from 00aca220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `runtime_error` scalar deleting destructor (thiscall, one word, `RET 4`):
/// stores the vtable `010a307c`, destroys the message string at `+0xc` (`0044a730(1, 0)`),
/// runs `exception::~exception` (`00ec79a1`) and with flag bit 0 frees `this`. Returns
/// `this`.
pub fn fn_00aca220(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_RUNTIME_ERROR_010A307C);
    e.call(STRING_TIDY_0044A730, &args![base + 0xc, 1u32, 0u32]);
    e.call(EXCEPTION_DTOR, &args![base]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![base]);
    }
    this
}

// Translated from 00aca270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aca270 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `runtime_error` copy constructor (thiscall, one word: the source, `RET 4`):
/// copies the `exception` part (`00ec7944`), stores the vtable `010a307c`, initialises the
/// message string at `+0xc` (`0044a730(0, 0)`) and assigns it the source's
/// (`0044a580(source + 0xc, 0, npos)`). Returns `this`.
pub fn fn_00aca270(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EXCEPTION_COPY, &args![base, source]);
    e.mem.set_u32(base, VTABLE_RUNTIME_ERROR_010A307C);
    e.call(STRING_TIDY_0044A730, &args![base + 0xc, 0u32, 0u32]);
    let npos = e.global::<u32>(STRING_NPOS_010173F0);
    e.call(
        STRING_ASSIGN_0044A580,
        &args![base + 0xc, source.addr() + 0xc, 0u32, npos],
    );
    this
}

/// The `ios_base::failure` destructor body [`fn_00aca300`] and [`fn_00aca340`] share.
fn destroy_failure(e: &mut Engine, base: u32) {
    e.mem.set_u32(base, VTABLE_IOS_FAILURE_010A3070);
    e.mem.set_u32(base, VTABLE_RUNTIME_ERROR_010A307C);
    e.call(STRING_TIDY_0044A730, &args![base + 0xc, 1u32, 0u32]);
    e.call(EXCEPTION_DTOR, &args![base]);
}

// Translated from 00aca300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ios_base::failure` destructor (thiscall): stores the vtables `010a3070` then
/// `010a307c`, destroys the message string at `+0xc` and runs `exception::~exception`.
pub fn fn_00aca300(e: &mut Engine, this: Ptr) {
    destroy_failure(e, this.addr());
}

// Translated from 00aca340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ios_base::failure` scalar deleting destructor (thiscall, one word,
/// `RET 4`): the destructor of [`fn_00aca300`], then with flag bit 0 frees `this`. Returns
/// `this`.
pub fn fn_00aca340(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    destroy_failure(e, this.addr());
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00aca390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ios_base::failure` copy constructor (thiscall, one word, `RET 4`): the
/// `runtime_error` copy ([`fn_00aca270`]), then the vtable `010a3070`. Returns `this`.
pub fn fn_00aca390(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    fn_00aca270(e, this, source);
    e.mem.set_u32(this.addr(), VTABLE_IOS_FAILURE_010A3070);
    this
}

// Translated from 00aca3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_ostream<char>::flush()` shape (thiscall). When the state at
/// `base + 8` has neither `failbit` nor `badbit` (`& 6`), calls the stream buffer's
/// (`base + 0x28`, not checked for null) slot `0x30` (`sync`); -1 means `badbit`, given to
/// the state as [`fn_00aca4e0`] does. Returns `this`.
pub fn fn_00aca3c0(e: &mut Engine, this: Ptr) -> Ptr {
    let stream = this.addr();
    let mut state = 0u32;
    let base = stream_base(e, stream);
    if e.mem.u32(base + 8) & 6 == 0 {
        let buffer = e.mem.u32(base + 0x28);
        if e.vcall(buffer, 0x30, &args![]).u32() == 0xffff_ffff {
            state |= 4;
        }
    }
    let base = stream_base(e, stream);
    fn_00aca4e0(e, Ptr::new(base), state, 0);
    this
}

// Translated from 00aca4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `basic_ios<char>` destructor (the map calls it `std::ios_base::operator_`; thiscall):
/// stores the vtables `010a2fa4` then `010a2f9c` and runs `_Ios_base_dtor` (`00ec3205`).
pub fn basic_ios_destructor(e: &mut Engine, this: Ptr) {
    destroy_ios(e, this.addr());
}

// Translated from 00aca4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `basic_ios::setstate(state, reraise)` (thiscall, two words, `RET 8`). With a
/// nonzero `state` calls `clear` ([`ios_base_clear`]) with the state at `+8` or `state`,
/// plus `badbit` (4) when the stream buffer at `+0x28` is null, and the `reraise` byte
/// (zero-extended).
pub fn fn_00aca4e0(e: &mut Engine, this: Ptr, state: u32, reraise: u32) {
    let base = this.addr();
    if state == 0 {
        return;
    }
    let mut new_state = e.mem.u32(base + 8) | state;
    if e.mem.u32(base + 0x28) == 0 {
        new_state |= 4;
    }
    ios_base_clear(e, this, new_state, reraise & 0xff);
}

/// The single-character input both `get` shapes share: zeroes the count at `+4`, builds an
/// input sentry on the stack ([`fn_00ad2fa0`], its "good" byte from [`fn_00ace390`] with
/// `noskip` 1); a bad sentry gives -1. Otherwise `read` takes the character from the stream
/// buffer (`[base + 0x28]`) and returns it with the state bits for an end of file. The state
/// goes to [`fn_00aca4e0`]; the sentry is destroyed ([`fn_00aca9a0`]).
fn get_character(e: &mut Engine, this: Ptr, read: fn(&mut Engine, u32, u32) -> (u32, u32)) -> u32 {
    let stream = this.addr();
    e.with_stack(8, |e, sentry| {
        let mut state = 0u32;
        e.mem.set_u32(stream + 4, 0);
        fn_00ad2fa0(e, sentry, this);
        let owner = e.mem.u32(sentry.addr());
        let good = fn_00ace390(e, Ptr::new(owner), 1);
        e.mem.set_u8(sentry.addr() + 4, good);
        let result = if good == 0 {
            0xffff_ffff
        } else {
            let base = stream_base(e, stream);
            let buffer = e.mem.u32(base + 0x28);
            let (character, bits) = read(e, stream, buffer);
            state |= bits;
            character
        };
        let base = stream_base(e, stream);
        fn_00aca4e0(e, Ptr::new(base), state, 0);
        fn_00aca9a0(e, sentry);
        result
    })
}

// Translated from 00aca560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aca560 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_istream<char>::get()` shape (thiscall). Zeroes the count at `+4`; with
/// a good sentry (see the helper) takes a character from the stream buffer
/// ([`fn_00ace860`], `sbumpc`): -1 sets `eofbit | failbit` (3), anything else counts one
/// (`+4`). Returns the character, or -1.
pub fn fn_00aca560(e: &mut Engine, this: Ptr) -> u32 {
    get_character(e, this, |e, stream, buffer| {
        let character = fn_00ace860(e, Ptr::new(buffer));
        if character == 0xffff_ffff {
            (character, 3)
        } else {
            let count = e.mem.u32(stream + 4).wrapping_add(1);
            e.mem.set_u32(stream + 4, count);
            (character, 0)
        }
    })
}

// Translated from 00aca780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall): forwards to the sentry destructor [`fn_00aca9a0`].
pub fn fn_00aca780(e: &mut Engine, this: Ptr) {
    fn_00aca9a0(e, this);
}

// Translated from 00aca7a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aca7a0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_istream<char>::peek()` shape (thiscall). Zeroes the count at `+4`;
/// with a good sentry looks at the stream buffer's next character ([`fn_00aca9f0`], `sgetc`):
/// -1 sets `eofbit` (1). Returns the character, or -1.
pub fn fn_00aca7a0(e: &mut Engine, this: Ptr) -> u32 {
    get_character(e, this, |e, _, buffer| {
        let character = fn_00aca9f0(e, Ptr::new(buffer));
        (character, u32::from(character == 0xffff_ffff))
    })
}

// Translated from 00aca9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the stream sentry destructor (thiscall; the sentry's word `+0` is its stream):
/// when the stream's buffer (`base + 0x28`) is set, unlocks its mutex at `+4` (`00ec3280`).
pub fn fn_00aca9a0(e: &mut Engine, this: Ptr) {
    let stream = e.mem.u32(this.addr());
    let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
    if buffer != 0 {
        let stream = e.mem.u32(this.addr());
        let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
        e.call(MUTEX_UNLOCK, &args![buffer + 4]);
    }
}

// Translated from 00aca9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf::sgetc()` shape (thiscall). The characters available are
/// `[[+0x30]]` when the read pointer `[[+0x20]]` is set, else 0; with more than 0 (signed) it
/// returns the byte at the read pointer, otherwise the buffer's slot `0x10` (`underflow`).
pub fn fn_00aca9f0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let available = if e.mem.u32(e.mem.u32(base + 0x20)) == 0 {
        0
    } else {
        e.mem.u32(e.mem.u32(base + 0x30)) as i32
    };
    if available <= 0 {
        e.vcall(base, 0x10, &args![]).u32()
    } else {
        let next = e.mem.u32(e.mem.u32(base + 0x20));
        u32::from(e.mem.u8(next))
    }
}

// Translated from 00acaa50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acaa50 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: a container constructor (thiscall). Allocates its proxy (`0044d670(1, 0)`),
/// stores `this` in it when that succeeded and the proxy (or 0) at `+0`, then initialises
/// the rest ([`fn_00aced20`] with 0). Returns `this`.
pub fn fn_00acaa50(e: &mut Engine, this: Ptr) -> Ptr {
    let proxy = e.call(ALLOCATE_PROXY_0044D670, &args![1u32, 0u32]).u32();
    let stored = if proxy != 0 {
        e.mem.set_u32(proxy, this.addr());
        proxy
    } else {
        0
    };
    e.mem.set_u32(this.addr(), stored);
    fn_00aced20(e, this, 0);
    this
}

// Translated from 00acab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<string>::push_back(value)` (thiscall, one word: the string, `RET 4`;
/// elements of 0x1c bytes from `+0xc`, last `+0x10`, end of storage `+0x14`, allocator at
/// `+8`). With room (size below capacity, sizes by signed division) it copy-constructs the
/// string at the end ([`fn_00ad5b00`] with count 1; its two tag words are the zero byte and a
/// byte the exe leaves uninitialised, which the callee does not read) and moves the end on;
/// otherwise it inserts at a checked end iterator ([`fn_00acea50`]).
pub fn fn_00acab30(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    let first = e.mem.u32(base + 0xc);
    let size = ((e.mem.u32(base + 0x10).wrapping_sub(first)) as i32 / 0x1c) as u32;
    let capacity = if first == 0 {
        0
    } else {
        ((e.mem.u32(base + 0x14).wrapping_sub(first)) as i32 / 0x1c) as u32
    };
    if size < capacity {
        let last = e.mem.u32(base + 0x10);
        fn_00ad5b00(e, last, 1, value, base + 8, 0, 0);
        e.mem.set_u32(base + 0x10, last.wrapping_add(0x1c));
    } else {
        let last = e.mem.u32(base + 0x10);
        let (proxy, at) = vector_iterator(e, base, last);
        e.with_stack(8, |e, out| {
            fn_00acea50(e, this, out, proxy, at, value);
        });
    }
}

// Translated from 00acac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<string>::clear()` (thiscall): builds checked iterators at the end
/// (`+0x10`) and then at the start (`+0xc`) and erases the range ([`fn_00aceb80`], the result
/// iterator in a stack local).
pub fn fn_00acac40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let last = e.mem.u32(base + 0x10);
    let end = vector_iterator(e, base, last);
    let first = e.mem.u32(base + 0xc);
    let begin = vector_iterator(e, base, first);
    e.with_stack(8, |e, out| {
        fn_00aceb80(e, this, out, begin.0, begin.1, end.0, end.1);
    });
}

// Translated from 00acad30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<char>::operator[](index)` (thiscall, one word, `RET 4`): calls
/// `_invalid_parameter_noinfo` when the index is not below the size (`+0x10 - +0xc`,
/// unsigned) and returns `[+0xc] + index`.
pub fn fn_00acad30(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let base = this.addr();
    if index >= e.mem.u32(base + 0x10).wrapping_sub(e.mem.u32(base + 0xc)) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u32(base + 0xc).wrapping_add(index)
}

// Translated from 00acad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<char>::push_back(value)` (thiscall, one word: a pointer to the
/// character, `RET 4`). With room (size below capacity, unsigned) it fills one element at the
/// end (`00ac3300`, cdecl: destination, count 1, value) and moves the end on by one;
/// otherwise it inserts at a checked end iterator ([`fn_00aceeb0`]).
pub fn fn_00acad60(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    let size = e.mem.u32(base + 0x10).wrapping_sub(e.mem.u32(base + 0xc));
    let capacity = if e.mem.u32(base + 0xc) == 0 {
        0
    } else {
        e.mem.u32(base + 0x14).wrapping_sub(e.mem.u32(base + 0xc))
    };
    if size < capacity {
        let last = e.mem.u32(base + 0x10);
        e.call(0x00ac_3300, &args![last, 1u32, value]);
        e.mem.set_u32(base + 0x10, last.wrapping_add(1));
    } else {
        let last = e.mem.u32(base + 0x10);
        let (proxy, at) = vector_iterator(e, base, last);
        e.with_stack(8, |e, out| {
            fn_00aceeb0(e, this, out, proxy, at, value);
        });
    }
}

// Translated from 00acae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<char>::clear()` (thiscall): builds checked iterators at the end and then
/// at the start and erases the range ([`fn_00acefd0`]).
pub fn fn_00acae90(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let last = e.mem.u32(base + 0x10);
    let end = vector_iterator(e, base, last);
    let first = e.mem.u32(base + 0xc);
    let begin = vector_iterator(e, base, first);
    e.with_stack(8, |e, out| {
        fn_00acefd0(e, this, out, begin.0, begin.1, end.0, end.1);
    });
}

/// Whether a tree node is the head (its "is nil" byte at `+0x45` set). Tree nodes: left
/// `+0`, parent `+4`, right `+8`, the value from `+0xc` (its key string first), colour
/// `+0x44`, "is nil" `+0x45`; the tree's head node is at `+0x18`, its size at `+0x1c`.
fn tree_is_nil(e: &Engine, node: u32) -> bool {
    e.mem.u8(node + 0x45) != 0
}

// Translated from 00acaf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the string-keyed tree's `insert(value)` (thiscall, two words: the `pair<iterator,
/// bool>` it fills and the value, `RET 8`). Walks down from the root comparing the value's
/// key with each node's ([`fn_00ac9da0`], "less" when negative) to find the parent. The
/// multi-key branch is compiled out (`XOR EDX,EDX; JZ`) and kept as such. With an iterator
/// at the parent ([`fn_00ad1a50`]): when the walk ended on a left turn and the parent is the
/// leftmost node (iterators checked against each other, `_invalid_parameter_noinfo`), it
/// inserts there ([`fn_00acf120`], "add left"); otherwise it steps the iterator back
/// ([`fn_00ad3450`]) and inserts only when that node's key is less than the value's. The
/// result's byte `+8` says whether it inserted; otherwise the iterator is the existing
/// node's. Returns `out`.
pub fn fn_00acaf50(e: &mut Engine, this: Ptr, out: Ptr, value: u32) -> Ptr {
    let base = this.addr();
    let head = e.mem.u32(base + 0x18);
    let mut node = e.mem.u32(head + 4);
    let mut parent = e.mem.u32(base + 0x18);
    let mut less = true;
    while !tree_is_nil(e, node) {
        parent = node;
        less = (fn_00ac9da0(e, Ptr::new(value), Ptr::new(node + 0xc)) as i32) < 0;
        node = if less {
            e.mem.u32(node)
        } else {
            e.mem.u32(node + 8)
        };
    }
    let write = |e: &mut Engine, iterator: u32, inserted: bool| {
        let first = e.mem.u32(iterator);
        let second = e.mem.u32(iterator + 4);
        e.mem.set_u32(out.addr(), first);
        e.mem.set_u32(out.addr() + 4, second);
        e.mem.set_u8(out.addr() + 8, inserted as u8);
        out
    };
    e.with_stack(0x28, |e, frame| {
        // Frame: the iterator at +0, the leftmost iterator at +8, the insert results at +0x10,
        // +0x18 and +0x20.
        let iterator = frame.addr();
        let leftmost = frame.addr() + 8;
        // `XOR EDX,EDX; JZ`: the multi-key branch is compiled out (the constant 0).
        if std::hint::black_box(0u32) != 0 {
            let made = fn_00acf120(
                e,
                this,
                Ptr::new(frame.addr() + 0x10),
                less as u32,
                parent,
                value,
            );
            return write(e, made.addr(), true);
        }
        fn_00ad1a50(e, Ptr::new(iterator), parent, this);
        if less {
            let head = e.mem.u32(base + 0x18);
            let first = e.mem.u32(head);
            fn_00ad1a50(e, Ptr::new(leftmost), first, this);
            let proxy = e.mem.u32(iterator);
            if proxy == 0 || proxy != e.mem.u32(leftmost) {
                e.call(INVALID_PARAMETER, &args![]);
            }
            if e.mem.u32(iterator + 4) == e.mem.u32(leftmost + 4) {
                let made = fn_00acf120(e, this, Ptr::new(frame.addr() + 0x18), 1, parent, value);
                return write(e, made.addr(), true);
            }
            fn_00ad3450(e, Ptr::new(iterator));
        }
        let found = e.mem.u32(iterator + 4);
        if (fn_00ac9da0(e, Ptr::new(found + 0xc), Ptr::new(value)) as i32) < 0 {
            let made = fn_00acf120(
                e,
                this,
                Ptr::new(frame.addr() + 0x20),
                less as u32,
                parent,
                value,
            );
            return write(e, made.addr(), true);
        }
        write(e, iterator, false)
    })
}

// Translated from 00acb1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the string-keyed tree's `clear()` (thiscall): erases every node under the root
/// ([`fn_00acf0b0`]), then points the head's parent, left and right back at the head and
/// zeroes the size (`+0x1c`).
pub fn fn_00acb1a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let root = e.mem.u32(e.mem.u32(base + 0x18) + 4);
    fn_00acf0b0(e, this, root);
    let head = e.mem.u32(base + 0x18);
    e.mem.set_u32(head + 4, head);
    e.mem.set_u32(base + 0x1c, 0);
    let head = e.mem.u32(base + 0x18);
    e.mem.set_u32(head, head);
    let head = e.mem.u32(base + 0x18);
    e.mem.set_u32(head + 8, head);
}

// Translated from 00acb200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the string-keyed tree's `find(key)` (thiscall, two words: the iterator it fills
/// and the key, `RET 8`). Takes the lower bound ([`fn_00ad1790`]) as a checked iterator
/// ([`fn_00ad1a50`]) and the end iterator (the head; `_invalid_parameter_noinfo` for a null
/// tree or iterators of different trees); returns the lower bound unless it is the end or the
/// key is less than its key ([`fn_00ac9da0`]), else a fresh end iterator. Returns `out`.
pub fn fn_00acb200(e: &mut Engine, this: Ptr, out: Ptr, key: u32) -> Ptr {
    let base = this.addr();
    let bound = fn_00ad1790(e, this, key);
    e.with_stack(0x18, |e, frame| {
        // Frame: the bound iterator at +0, the end iterator at +8, the second end at +0x10.
        let iterator = frame.addr();
        let end = frame.addr() + 8;
        let second_end = frame.addr() + 0x10;
        fn_00ad1a50(e, Ptr::new(iterator), bound, this);
        let head = e.mem.u32(base + 0x18);
        e.mem.set_u32(end + 4, head);
        if base == 0 {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let proxy = e.mem.u32(base);
        e.mem.set_u32(end, proxy);
        let own = e.mem.u32(iterator);
        if own == 0 || own != e.mem.u32(end) {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let mut result = second_end;
        let node = e.mem.u32(iterator + 4);
        if node != e.mem.u32(end + 4)
            && (fn_00ac9da0(e, Ptr::new(key), Ptr::new(node + 0xc)) as i32) >= 0
        {
            result = iterator;
        } else {
            let head = e.mem.u32(base + 0x18);
            e.mem.set_u32(second_end + 4, head);
            if base == 0 {
                e.call(INVALID_PARAMETER, &args![]);
            }
            let proxy = e.mem.u32(base);
            e.mem.set_u32(second_end, proxy);
        }
        let first = e.mem.u32(result);
        let second = e.mem.u32(result + 4);
        e.mem.set_u32(out.addr(), first);
        e.mem.set_u32(out.addr() + 4, second);
        out
    })
}

// Translated from 00acf0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the string-keyed tree's `_Erase(node)` (thiscall, one word, `RET 4`): for each
/// node down the left spine until the head, erases its right subtree (recursively), destroys
/// its value ([`fn_00ad5490`]) and frees it (`operator delete`; a second, compiled-out free
/// behind `XOR EDX,EDX; AND EDX,1` is kept as such).
pub fn fn_00acf0b0(e: &mut Engine, this: Ptr, node: u32) {
    let mut node = node;
    while !tree_is_nil(e, node) {
        let right = e.mem.u32(node + 8);
        fn_00acf0b0(e, this, right);
        let next = e.mem.u32(node);
        fn_00ad5490(e, Ptr::new(node));
        // `XOR EDX,EDX; AND EDX,1; JZ`: the first delete is compiled out (the constant 0).
        if std::hint::black_box(0u32) & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![node]);
        }
        e.call(OPERATOR_DELETE, &args![node]);
        node = next;
    }
}

// Translated from 00ad1790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the string-keyed tree's `_Lbound(key)` (thiscall, one word, `RET 4`): from the
/// root, goes right while the node's key is less than `key` ([`fn_00ac9da0`] negative), else
/// remembers the node and goes left; returns the last node remembered (the head when none).
pub fn fn_00ad1790(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let base = this.addr();
    let mut node = e.mem.u32(e.mem.u32(base + 0x18) + 4);
    let mut bound = e.mem.u32(base + 0x18);
    while !tree_is_nil(e, node) {
        if (fn_00ac9da0(e, Ptr::new(node + 0xc), Ptr::new(key)) as i32) < 0 {
            node = e.mem.u32(node + 8);
        } else {
            bound = node;
            node = e.mem.u32(node);
        }
    }
    bound
}

// Translated from 00ad1a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked tree iterator's constructor (thiscall, two words: the node and the
/// tree, `RET 8`): zeroes `+0`, stores the node at `+4`, calls `_invalid_parameter_noinfo`
/// for a null tree and stores the tree's proxy (its word `+0`) at `+0`. Returns `this`.
pub fn fn_00ad1a50(e: &mut Engine, this: Ptr, node: u32, tree: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, node);
    if tree.addr() == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(tree.addr());
    e.mem.set_u32(base, proxy);
    this
}

// Translated from 00ad3450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked tree iterator's `operator--` (thiscall; proxy `+0`, node `+4`). A null
/// proxy calls `_invalid_parameter_noinfo` first. From the head it goes to the rightmost
/// node (the head's `+8`; still the head means `_invalid_parameter_noinfo`); with a left
/// child to that child's rightmost node; otherwise up while it is its parent's left child,
/// then to the parent (`_invalid_parameter_noinfo`, and no step, when it stopped on the
/// head).
pub fn fn_00ad3450(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let node = e.mem.u32(base + 4);
    if tree_is_nil(e, node) {
        let rightmost = e.mem.u32(node + 8);
        e.mem.set_u32(base + 4, rightmost);
        if tree_is_nil(e, e.mem.u32(base + 4)) {
            e.call(INVALID_PARAMETER, &args![]);
        }
        return;
    }
    if !tree_is_nil(e, e.mem.u32(node)) {
        let mut child = e.mem.u32(node);
        while !tree_is_nil(e, e.mem.u32(child + 8)) {
            child = e.mem.u32(child + 8);
        }
        e.mem.set_u32(base + 4, child);
        return;
    }
    let mut parent;
    loop {
        parent = e.mem.u32(e.mem.u32(base + 4) + 4);
        if !tree_is_nil(e, parent) && e.mem.u32(base + 4) == e.mem.u32(parent) {
            e.mem.set_u32(base + 4, parent);
        } else {
            break;
        }
    }
    if tree_is_nil(e, e.mem.u32(base + 4)) {
        e.call(INVALID_PARAMETER, &args![]);
    } else {
        e.mem.set_u32(base + 4, parent);
    }
}

/// `memcpy_s` (`00ec7c66`, cdecl: destination, its size, source, count).
const MEMCPY_S: u32 = 0x00ec_7c66;
/// The `basic_istringstream<char>` tables (`.rdata`, RTTI checked): `010a30b4` (its
/// virtual-base table), `010a30ac` (the one of the compiled-out branch), `010a30a8` and
/// `010a30a0` (the `basic_istream` and `basic_istringstream` vtables stored at the virtual
/// base), and `010a30c0` (the `basic_stringbuf` vtable its buffer gets).
const VBTABLE_ISTRINGSTREAM_010A30B4: u32 = 0x010a_30b4;
const VBTABLE_ISTREAM_010A30AC: u32 = 0x010a_30ac;
const VTABLE_ISTREAM_010A30A8: u32 = 0x010a_30a8;
const VTABLE_ISTRINGSTREAM_010A30A0: u32 = 0x010a_30a0;
const VTABLE_STRINGBUF_010A30C0: u32 = 0x010a_30c0;

// Translated from 00acb310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acb310 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: a pair of strings' constructor (thiscall, two words: the two source strings,
/// `RET 8`): initialises the string at `+0` (`0044a730(0, 0)`) and assigns it the first
/// (`0044a580(first, 0, npos)`), then the same for the string at `+0x1c` and the second.
/// Returns `this`.
pub fn fn_00acb310(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    let base = this.addr();
    e.call(STRING_TIDY_0044A730, &args![base, 0u32, 0u32]);
    let npos = e.global::<u32>(STRING_NPOS_010173F0);
    e.call(STRING_ASSIGN_0044A580, &args![base, first, 0u32, npos]);
    e.call(STRING_TIDY_0044A730, &args![base + 0x1c, 0u32, 0u32]);
    let npos = e.global::<u32>(STRING_NPOS_010173F0);
    e.call(
        STRING_ASSIGN_0044A580,
        &args![base + 0x1c, second, 0u32, npos],
    );
    this
}

// Translated from 00acb3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `push_back(value)` of a vector of string pairs (thiscall, one word, `RET 4`;
/// elements of 0x38 bytes): as [`fn_00acab30`], constructing the element with
/// [`fn_00ad5bd0`] and inserting with [`fn_00acf7d0`].
pub fn fn_00acb3c0(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    let first = e.mem.u32(base + 0xc);
    let size = ((e.mem.u32(base + 0x10).wrapping_sub(first)) as i32 / 0x38) as u32;
    let capacity = if first == 0 {
        0
    } else {
        ((e.mem.u32(base + 0x14).wrapping_sub(first)) as i32 / 0x38) as u32
    };
    if size < capacity {
        let last = e.mem.u32(base + 0x10);
        fn_00ad5bd0(e, last, 1, value, base + 8, 0, 0);
        e.mem.set_u32(base + 0x10, last.wrapping_add(0x38));
    } else {
        let last = e.mem.u32(base + 0x10);
        let (proxy, at) = vector_iterator(e, base, last);
        e.with_stack(8, |e, out| {
            fn_00acf7d0(e, this, out, proxy, at, value);
        });
    }
}

// Translated from 00acb4d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acb4d0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_istringstream<char>(string, mode)` constructor (RTTI of the vtables it
/// stores; thiscall, three words: the string, the open mode and the "construct the virtual base" flag, `RET 0xc`). With the flag it
/// stores the virtual-base table `010a30b4` and the `ios_base` and `basic_ios` vtables at
/// `+0x50`; a compiled-out branch (`XOR EDX,EDX; JZ`, kept as such) would store `010a30ac` and
/// the same vtables at `+8`. Stores `010a30a8` at the virtual base, zeroes the count at `+4`,
/// initialises the `basic_ios` with the buffer at `+8` ([`fn_00ad0470`], no standard stream),
/// stores `010a30a0` at the virtual base and constructs the string buffer at `+8` from the
/// string with the mode plus `in` (1) ([`fn_00acf900`]). Returns `this`.
pub fn fn_00acb4d0(e: &mut Engine, this: Ptr, string: u32, mode: u32, construct_base: u32) -> Ptr {
    let base = this.addr();
    if construct_base != 0 {
        e.mem.set_u32(base, VBTABLE_ISTRINGSTREAM_010A30B4);
        e.mem.set_u32(base + 0x50, VTABLE_IOS_BASE_010A2F9C);
        e.mem.set_u32(base + 0x50, VTABLE_BASIC_IOS_010A2FA4);
    }
    // `XOR EDX,EDX; JZ`: this branch is compiled out (the constant 0).
    if std::hint::black_box(0u32) != 0 {
        e.mem.set_u32(base, VBTABLE_ISTREAM_010A30AC);
        e.mem.set_u32(base + 8, VTABLE_IOS_BASE_010A2F9C);
        e.mem.set_u32(base + 8, VTABLE_BASIC_IOS_010A2FA4);
    }
    let ios = stream_base(e, base);
    e.mem.set_u32(ios, VTABLE_ISTREAM_010A30A8);
    e.mem.set_u32(base + 4, 0);
    let ios = stream_base(e, base);
    fn_00ad0470(e, Ptr::new(ios), base + 8, 0);
    let ios = stream_base(e, base);
    e.mem.set_u32(ios, VTABLE_ISTRINGSTREAM_010A30A0);
    fn_00acf900(e, Ptr::new(base + 8), string, mode | 1);
    this
}

// Translated from 00acb5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_istream<char>` destructor at its virtual base (thiscall; the stream
/// starts 8 bytes below `this`): stores the vtable `010a30a8` at the stream's virtual base.
pub fn fn_00acb5f0(e: &mut Engine, this: Ptr) {
    let stream = this.addr().wrapping_sub(8);
    let ios = stream_base(e, stream);
    e.mem.set_u32(ios, VTABLE_ISTREAM_010A30A8);
}

// Translated from 00acb610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acb610 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_istringstream<char>` destructor at its virtual base (thiscall; the
/// stream starts 0x50 bytes below `this`): stores `010a30a0` at the virtual base, the
/// `basic_stringbuf` vtable `010a30c0` at the buffer (`this - 0x48`), frees its buffers ([`fn_00acf9e0`]) and
/// runs the stream-buffer destructor ([`fn_00acb6b0`]), then stores `010a30a8` at the virtual
/// base.
pub fn fn_00acb610(e: &mut Engine, this: Ptr) {
    let stream = this.addr().wrapping_sub(0x50);
    let ios = stream_base(e, stream);
    e.mem.set_u32(ios, VTABLE_ISTRINGSTREAM_010A30A0);
    let buffer = this.addr().wrapping_sub(0x48);
    e.mem.set_u32(buffer, VTABLE_STRINGBUF_010A30C0);
    fn_00acf9e0(e, Ptr::new(buffer));
    fn_00acb6b0(e, Ptr::new(buffer));
    let stream = buffer.wrapping_sub(8);
    let ios = stream_base(e, stream);
    e.mem.set_u32(ios, VTABLE_ISTREAM_010A30A8);
}

// Translated from 00acb7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (thiscall, one word it does not read, `RET 4`): returns -1 (`OR EAX,-1`).
pub fn fn_00acb7d0(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0xffff_ffff
}

// Translated from 00acb7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf::uflow()` shape (thiscall): calls the buffer's slot `0x10`
/// (`underflow`); -1 gives -1, otherwise the byte at the read pointer, which
/// [`fn_00ace970`] returns and moves on.
pub fn fn_00acb7f0(e: &mut Engine, this: Ptr) -> u32 {
    if e.vcall(this.addr(), 0x10, &args![]).u32() == 0xffff_ffff {
        return 0xffff_ffff;
    }
    let next = fn_00ace970(e, this);
    u32::from(e.mem.u8(next))
}

// Translated from 00acb850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra's library match lists `basic_streambuf::xsgetn`; thiscall, two words,
/// `RET 8`): calls the buffer's slot `0x1c` with `(first, -1, second)` and returns its `EAX`.
pub fn fn_00acb850(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    e.vcall(this.addr(), 0x1c, &args![first, 0xffff_ffffu32, second])
        .u32()
}

// Translated from 00acb880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf::_Xsgetn_s(destination, size, count)` shape (thiscall,
/// three words, `RET 0xc`). While the count (signed) is positive: with characters in the get
/// area (`[[+0x30]]` when `[[+0x20]]` is set) it copies as many as fit the count
/// (`memcpy_s`, always given the original `size`) and moves the destination, the total, the
/// count, the available count and the read pointer; otherwise it takes one character
/// through the buffer's slot `0x14` (`uflow`), stopping at -1. Returns the total.
pub fn fn_00acb880(e: &mut Engine, this: Ptr, destination: u32, size: u32, count: u32) -> u32 {
    let base = this.addr();
    let mut destination = destination;
    let mut count = count as i32;
    let mut total = 0u32;
    while count > 0 {
        let available = if e.mem.u32(e.mem.u32(base + 0x20)) == 0 {
            0
        } else {
            e.mem.u32(e.mem.u32(base + 0x30)) as i32
        };
        if available > 0 {
            let mut take = available;
            if count < take {
                take = count;
            }
            let source = e.mem.u32(e.mem.u32(base + 0x20));
            e.call(MEMCPY_S, &args![destination, size, source, take as u32]);
            destination = destination.wrapping_add(take as u32);
            total = total.wrapping_add(take as u32);
            count = count.wrapping_sub(take);
            let left_at = e.mem.u32(base + 0x30);
            let left = e.mem.u32(left_at).wrapping_sub(take as u32);
            e.mem.set_u32(left_at, left);
            let next_at = e.mem.u32(base + 0x20);
            let next = e.mem.u32(next_at).wrapping_add(take as u32);
            e.mem.set_u32(next_at, next);
        } else {
            let character = e.vcall(base, 0x14, &args![]).u32();
            if character == 0xffff_ffff {
                break;
            }
            e.mem.set_u8(destination, character as u8);
            destination = destination.wrapping_add(1);
            total = total.wrapping_add(1);
            count = count.wrapping_sub(1);
        }
    }
    total
}

// Translated from 00acb9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf::xsputn(source, count)` shape (thiscall, two words,
/// `RET 8`). While the count (signed) is positive: with room in the put area (`[[+0x34]]`
/// when `[[+0x24]]` is set) it copies as many as fit (`memcpy_s` into the write pointer,
/// sized by the room) and moves the source, the total, the count, the room and the write
/// pointer; otherwise it writes one character through the buffer's slot 4 (`overflow`,
/// the byte zero-extended), stopping at -1. Returns the total.
pub fn fn_00acb9b0(e: &mut Engine, this: Ptr, source: u32, count: u32) -> u32 {
    let base = this.addr();
    let mut source = source;
    let mut count = count as i32;
    let mut total = 0u32;
    while count > 0 {
        let room = if e.mem.u32(e.mem.u32(base + 0x24)) == 0 {
            0
        } else {
            e.mem.u32(e.mem.u32(base + 0x34)) as i32
        };
        if room > 0 {
            let mut take = room;
            if count < take {
                take = count;
            }
            let destination = e.mem.u32(e.mem.u32(base + 0x24));
            e.call(
                MEMCPY_S,
                &args![destination, room as u32, source, take as u32],
            );
            source = source.wrapping_add(take as u32);
            total = total.wrapping_add(take as u32);
            count = count.wrapping_sub(take);
            let room_at = e.mem.u32(base + 0x34);
            let left = e.mem.u32(room_at).wrapping_sub(take as u32);
            e.mem.set_u32(room_at, left);
            let next_at = e.mem.u32(base + 0x24);
            let next = e.mem.u32(next_at).wrapping_add(take as u32);
            e.mem.set_u32(next_at, next);
        } else {
            let character = u32::from(e.mem.u8(source));
            if e.vcall(base, 4, &args![character]).u32() == 0xffff_ffff {
                break;
            }
            source = source.wrapping_add(1);
            total = total.wrapping_add(1);
            count = count.wrapping_sub(1);
        }
    }
    total
}

// Translated from 00acf900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acf900 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_stringbuf<char>(string, mode)` constructor (thiscall, two words: the
/// string and the stream open mode, `RET 8`). Constructs the stream buffer
/// ([`fn_00ad0630`]), stores the vtable `010a30c0`, turns the mode into the buffer's state
/// (4 without `in`, 2 without `out`, 8 with bit 3, 0x10 with bit 2) and initialises the
/// buffer from the string's characters (inline below capacity 0x10, length `+0x14`) with it
/// ([`fn_00ad2500`]). Returns `this`.
pub fn fn_00acf900(e: &mut Engine, this: Ptr, name: u32, mode: u32) -> Ptr {
    let base = this.addr();
    fn_00ad0630(e, this);
    e.mem.set_u32(base, VTABLE_STRINGBUF_010A30C0);
    let mut flags = 0u32;
    if mode & 1 == 0 {
        flags |= 4;
    }
    if mode & 2 == 0 {
        flags |= 2;
    }
    if mode & 8 != 0 {
        flags |= 8;
    }
    if mode & 4 != 0 {
        flags |= 0x10;
    }
    let length = e.mem.u32(name + 0x14);
    let characters = if e.mem.u32(name + 0x18) < 0x10 {
        name + 4
    } else {
        e.mem.u32(name + 4)
    };
    fn_00ad2500(e, this, characters, length, flags);
    this
}

// Translated from 00acf9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_stringbuf<char>::_Tidy()` shape (thiscall). When it owns its buffer (bit 0 of
/// `+0x40`) it reads the end of the put or get area (the sum is not used) and frees the buffer
/// (`[[+0x10]]`, `operator delete`); then it clears the six area pointers (through `+0x10`,
/// `+0x20`, `+0x30`, `+0x14`, `+0x24`, `+0x34`), `+0x3c` and the ownership bit.
pub fn fn_00acf9e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x40) & 1 != 0 {
        let put = e.mem.u32(e.mem.u32(base + 0x24));
        // The end of the area the exe computes here is never used.
        let _end = if put != 0 {
            put.wrapping_add(e.mem.u32(e.mem.u32(base + 0x34)))
        } else {
            e.mem
                .u32(e.mem.u32(base + 0x20))
                .wrapping_add(e.mem.u32(e.mem.u32(base + 0x30)))
        };
        let _start = e.mem.u32(e.mem.u32(base + 0x10));
        let buffer = e.mem.u32(e.mem.u32(base + 0x10));
        e.call(OPERATOR_DELETE, &args![buffer]);
    }
    for slot in [0x10u32, 0x20, 0x30, 0x14, 0x24, 0x34] {
        let target = e.mem.u32(base + slot);
        e.mem.set_u32(target, 0);
    }
    e.mem.set_u32(base + 0x3c, 0);
    let flags = e.mem.u32(base + 0x40) & 0xffff_fffe;
    e.mem.set_u32(base + 0x40, flags);
}

/// The bad stream offset (-1 in the exe) and the zero file-position state the seek functions
/// return (`.rdata`, words).
const BAD_OFFSET_010F1E40: u32 = 0x010f_1e40;
const ZERO_STATE_01011D78: u32 = 0x0101_1d78;
/// `allocator<char>::allocate` (`0065f7a0`, thiscall, one word: the count).
const ALLOCATE_CHARACTERS: u32 = 0x0065_f7a0;

/// The file position the default seeks return: the bad offset (`010f1e40`) at `+0`, zeros at
/// `+8` and `+0xc` and the zero state (`01011d78`) at `+0x10`; `+4` is left as it is.
fn bad_position(e: &mut Engine, out: Ptr) -> Ptr {
    let offset = e.global::<u32>(BAD_OFFSET_010F1E40);
    e.mem.set_u32(out.addr(), offset);
    e.mem.set_u32(out.addr() + 8, 0);
    e.mem.set_u32(out.addr() + 0xc, 0);
    let state = e.global::<u32>(ZERO_STATE_01011D78);
    e.mem.set_u32(out.addr() + 0x10, state);
    out
}

// Translated from 00acbae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the default `basic_streambuf::seekoff` shape (thiscall, four words: the file
/// position it fills and three it does not read, `RET 0x10`): fills the bad position (see
/// [`bad_position`]) and returns it.
pub fn fn_00acbae0(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> Ptr {
    bad_position(e, out)
}

// Translated from 00acbb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the default `basic_streambuf::seekpos` shape (thiscall, eight words: the file
/// position it fills and seven it does not read, `RET 0x20`): fills the bad position and
/// returns it.
#[allow(clippy::too_many_arguments)]
pub fn fn_00acbb20(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
) -> Ptr {
    bad_position(e, out)
}

// Translated from 00acbb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the default `basic_streambuf::setbuf` shape (thiscall, two words it does not
/// read, `RET 8`): returns `this`.
pub fn fn_00acbb60(_e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) -> Ptr {
    this
}

// Translated from 00acbb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_stringbuf<char>::overflow(ch)` shape (thiscall, one word, `RET 4`; the
/// mode at `+0x40`: bit 0 owns the buffer, bit 1 constant, bit 2 no reading, bit 3 append;
/// `+0x3c` the high-water mark; `+0x44` the allocator). In append mode it first moves the
/// write pointer up to the mark. An end of file (-1) returns 0 (`not_eof`; the exe's other
/// arm, the character itself, cannot be taken there and is kept). With room left it stores
/// the character at the write pointer ([`basic_streambuf_pninc`]); a constant buffer returns -1.
/// Otherwise it grows the buffer: by half its size, at least 0x20, halved while the sum
/// would pass `0x7fffffff`; allocates (`0065f7a0`), copies the old contents (`memcpy_s`) and
/// moves every pointer (the read pointers too, or with no reading a null read area), frees
/// an owned old buffer, takes ownership and stores the character. Returns the character.
pub fn fn_00acbb80(e: &mut Engine, this: Ptr, character: u32) -> u32 {
    let base = this.addr();
    let at = |e: &Engine, slot: u32| e.mem.u32(e.mem.u32(base + slot));
    let set = |e: &mut Engine, slot: u32, value: u32| {
        let target = e.mem.u32(base + slot);
        e.mem.set_u32(target, value);
    };
    if e.mem.u32(base + 0x40) & 8 != 0 && at(e, 0x24) != 0 && at(e, 0x24) < e.mem.u32(base + 0x3c) {
        let end = at(e, 0x24).wrapping_add(at(e, 0x34));
        let start = at(e, 0x14);
        let mark = e.mem.u32(base + 0x3c);
        set(e, 0x14, start);
        set(e, 0x24, mark);
        set(e, 0x34, end.wrapping_sub(mark));
    }
    if character == 0xffff_ffff {
        // `not_eof`: the exe's `NEG`/`SBB`/`ADD 1` of -1 gives 0.
        return if character != 0xffff_ffff {
            character
        } else {
            0
        };
    }
    let next = at(e, 0x24);
    if next != 0 && at(e, 0x24) < at(e, 0x24).wrapping_add(at(e, 0x34)) {
        let slot = basic_streambuf_pninc(e, this);
        e.mem.set_u8(slot, character as u8);
        return character;
    }
    if e.mem.u32(base + 0x40) & 2 != 0 {
        return 0xffff_ffff;
    }
    let old_size = if at(e, 0x24) == 0 {
        0
    } else {
        at(e, 0x24)
            .wrapping_add(at(e, 0x34))
            .wrapping_sub(at(e, 0x10))
    };
    let mut total = old_size;
    let mut growth = if total >> 1 < 0x20 { 0x20 } else { total >> 1 };
    let mut fresh = 0u32;
    let old = at(e, 0x10);
    while growth > 0 && 0x7fff_ffffu32.wrapping_sub(growth) < total {
        growth >>= 1;
    }
    if growth > 0 {
        total = total.wrapping_add(growth);
        fresh = e
            .call(ALLOCATE_CHARACTERS, &args![base + 0x44, total])
            .u32();
    }
    if old_size > 0 {
        e.call(MEMCPY_S, &args![fresh, total, old, old_size]);
    }
    if old_size == 0 {
        e.mem.set_u32(base + 0x3c, fresh);
        set(e, 0x14, fresh);
        set(e, 0x24, fresh);
        set(e, 0x34, fresh.wrapping_add(total).wrapping_sub(fresh));
        if e.mem.u32(base + 0x40) & 4 != 0 {
            set(e, 0x10, fresh);
            set(e, 0x20, 0);
            set(e, 0x30, fresh);
        } else {
            set(e, 0x10, fresh);
            set(e, 0x20, fresh);
            set(e, 0x30, fresh.wrapping_add(1).wrapping_sub(fresh));
        }
    } else {
        let mark = e.mem.u32(base + 0x3c).wrapping_sub(old).wrapping_add(fresh);
        e.mem.set_u32(base + 0x3c, mark);
        let next = at(e, 0x24);
        let start = at(e, 0x14);
        let moved = next.wrapping_sub(old).wrapping_add(fresh);
        set(e, 0x14, start.wrapping_sub(old).wrapping_add(fresh));
        set(e, 0x24, moved);
        set(e, 0x34, fresh.wrapping_add(total).wrapping_sub(moved));
        if e.mem.u32(base + 0x40) & 4 != 0 {
            set(e, 0x10, fresh);
            set(e, 0x20, 0);
            set(e, 0x30, fresh);
        } else {
            let write_next = at(e, 0x24);
            let read_next = at(e, 0x20);
            let moved_read = read_next.wrapping_sub(old).wrapping_add(fresh);
            set(e, 0x10, fresh);
            set(e, 0x20, moved_read);
            set(e, 0x30, write_next.wrapping_add(1).wrapping_sub(moved_read));
        }
    }
    if e.mem.u32(base + 0x40) & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![old]);
    }
    let mode = e.mem.u32(base + 0x40) | 1;
    e.mem.set_u32(base + 0x40, mode);
    let slot = basic_streambuf_pninc(e, this);
    e.mem.set_u8(slot, character as u8);
    character
}

// Translated from 00acbf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_stringbuf<char>::pbackfail(ch)` shape (thiscall, one word, `RET 4`).
/// Fails (-1) without a read pointer, at the start of the area, or when the character
/// differs from the one before the read pointer in a constant buffer (mode bit 1). Otherwise
/// it gives back one character (count `[[+0x30]]` up, read pointer `[[+0x20]]` down), stores
/// the character there unless it is -1, and returns it (-1 gives 0, `not_eof`).
pub fn fn_00acbf60(e: &mut Engine, this: Ptr, character: u32) -> u32 {
    let base = this.addr();
    let next = e.mem.u32(e.mem.u32(base + 0x20));
    if next == 0 || next <= e.mem.u32(e.mem.u32(base + 0x10)) {
        return 0xffff_ffff;
    }
    if character != 0xffff_ffff {
        let previous = e.mem.u8(e.mem.u32(e.mem.u32(base + 0x20)).wrapping_sub(1)) as i8;
        if character as u8 as i8 != previous && e.mem.u32(base + 0x40) & 2 != 0 {
            return 0xffff_ffff;
        }
    }
    let count_at = e.mem.u32(base + 0x30);
    let count = e.mem.u32(count_at).wrapping_add(1);
    e.mem.set_u32(count_at, count);
    let next_at = e.mem.u32(base + 0x20);
    let next = e.mem.u32(next_at).wrapping_sub(1);
    e.mem.set_u32(next_at, next);
    if character != 0xffff_ffff {
        let slot = e.mem.u32(e.mem.u32(base + 0x20));
        e.mem.set_u8(slot, character as u8);
    }
    if character != 0xffff_ffff {
        character
    } else {
        0
    }
}

// Translated from 00acc070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_stringbuf<char>::underflow()` shape (thiscall). -1 without a read
/// pointer; the byte at the read pointer while characters are left (`[[+0x30]]`). Otherwise,
/// unless reading is off (mode bit 2) or there is no write pointer, when the write pointer
/// or the high-water mark lies past the read pointer it raises the mark to the write pointer,
/// extends the read area to the mark and returns the byte at the read pointer; else -1.
pub fn fn_00acc070(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let at = |e: &Engine, slot: u32| e.mem.u32(e.mem.u32(base + slot));
    if at(e, 0x20) == 0 {
        return 0xffff_ffff;
    }
    if at(e, 0x20) < at(e, 0x20).wrapping_add(at(e, 0x30)) {
        return u32::from(e.mem.u8(at(e, 0x20)));
    }
    if e.mem.u32(base + 0x40) & 4 != 0
        || at(e, 0x24) == 0
        || (at(e, 0x24) <= at(e, 0x20) && e.mem.u32(base + 0x3c) <= at(e, 0x20))
    {
        return 0xffff_ffff;
    }
    if e.mem.u32(base + 0x3c) < at(e, 0x24) {
        let write_next = at(e, 0x24);
        e.mem.set_u32(base + 0x3c, write_next);
    }
    let next = at(e, 0x20);
    let start = at(e, 0x10);
    let mark = e.mem.u32(base + 0x3c);
    let target = e.mem.u32(base + 0x10);
    e.mem.set_u32(target, start);
    let target = e.mem.u32(base + 0x20);
    e.mem.set_u32(target, next);
    let target = e.mem.u32(base + 0x30);
    e.mem.set_u32(target, mark.wrapping_sub(next));
    u32::from(e.mem.u8(at(e, 0x20)))
}

/// The `basic_ifstream<char>` virtual-base table and vtable (`.rdata`, RTTI checked).
const VBTABLE_IFSTREAM_010A3178: u32 = 0x010a_3178;
const VTABLE_IFSTREAM_010A3174: u32 = 0x010a_3174;

/// The checked move of one area to a new offset that both seek shapes share: with the offset
/// negative or past the high-water mark (`+0x3c`) from the area start (`[[+0x10]]`, signed)
/// the result is the bad offset (`010f1e40`); otherwise the area's next pointer (`next_slot`)
/// moves to the start plus the offset and its count (`count_slot`) shrinks by the same
/// amount.
fn string_buffer_seek_area(
    e: &mut Engine,
    base: u32,
    offset: u32,
    next_slot: u32,
    count_slot: u32,
) -> Result<(), u32> {
    let start = e.mem.u32(e.mem.u32(base + 0x10));
    let limit = e.mem.u32(base + 0x3c).wrapping_sub(start);
    if (offset as i32) < 0 || (offset as i32) > limit as i32 {
        return Err(e.global::<u32>(BAD_OFFSET_010F1E40));
    }
    let start = e.mem.u32(e.mem.u32(base + 0x10));
    let next = e.mem.u32(e.mem.u32(base + next_slot));
    let delta = start.wrapping_sub(next).wrapping_add(offset);
    let count_at = e.mem.u32(base + count_slot);
    let count = e.mem.u32(count_at).wrapping_sub(delta);
    e.mem.set_u32(count_at, count);
    let next_at = e.mem.u32(base + next_slot);
    let next = e.mem.u32(next_at).wrapping_add(delta);
    e.mem.set_u32(next_at, next);
    Ok(())
}

/// After a read-area seek in a buffer that also writes: the write pointer goes to the read
/// pointer and the write count to the end of the put area from there.
fn string_buffer_sync_put(e: &mut Engine, base: u32) {
    let at = |e: &Engine, slot: u32| e.mem.u32(e.mem.u32(base + slot));
    let end = at(e, 0x24).wrapping_add(at(e, 0x34));
    let next = at(e, 0x20);
    let start = at(e, 0x14);
    let target = e.mem.u32(base + 0x14);
    e.mem.set_u32(target, start);
    let target = e.mem.u32(base + 0x24);
    e.mem.set_u32(target, next);
    let target = e.mem.u32(base + 0x34);
    e.mem.set_u32(target, end.wrapping_sub(next));
}

/// Raises the high-water mark (`+0x3c`) to the write pointer when that is set and above it.
fn string_buffer_raise_mark(e: &mut Engine, base: u32) {
    let write_next = e.mem.u32(e.mem.u32(base + 0x24));
    if write_next != 0 && e.mem.u32(base + 0x3c) < e.mem.u32(e.mem.u32(base + 0x24)) {
        let write_next = e.mem.u32(e.mem.u32(base + 0x24));
        e.mem.set_u32(base + 0x3c, write_next);
    }
}

/// Fills a file position with `offset` at `+0`, zeros at `+8` and `+0xc` and the zero state
/// (`01011d78`) at `+0x10`.
fn write_position(e: &mut Engine, out: Ptr, offset: u32) -> Ptr {
    e.mem.set_u32(out.addr(), offset);
    e.mem.set_u32(out.addr() + 8, 0);
    e.mem.set_u32(out.addr() + 0xc, 0);
    let state = e.global::<u32>(ZERO_STATE_01011D78);
    e.mem.set_u32(out.addr() + 0x10, state);
    out
}

// Translated from 00acc1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_stringbuf<char>::seekoff(offset, way, mode)` shape (thiscall, four
/// words: the file position it fills, the offset, the direction and the mode, `RET 0x10`).
/// Raises the high-water mark to the write pointer first. With `in` (mode bit 0) and a read
/// area it makes the offset absolute (end: from the mark; current: from the read pointer,
/// unless `out` is also asked; beginning as is; anything else the bad offset), moves the read
/// pointer there when it is in range (else the bad offset) and with `out` and a write area
/// moves the write pointer along. Otherwise with `out` and a write area the same on the
/// write pointer; else the bad offset. Returns the position (the offset, zeros and the zero
/// state).
pub fn fn_00acc1b0(e: &mut Engine, this: Ptr, out: Ptr, offset: u32, way: u32, mode: u32) -> Ptr {
    let base = this.addr();
    let at = |e: &Engine, slot: u32| e.mem.u32(e.mem.u32(base + slot));
    let mut offset = offset;
    string_buffer_raise_mark(e, base);
    if mode & 1 != 0 && at(e, 0x20) != 0 {
        if way == 2 {
            offset = e
                .mem
                .u32(base + 0x3c)
                .wrapping_sub(at(e, 0x10))
                .wrapping_add(offset);
        } else if way == 1 && mode & 2 == 0 {
            offset = at(e, 0x20).wrapping_sub(at(e, 0x10)).wrapping_add(offset);
        } else if way != 0 {
            offset = e.global::<u32>(BAD_OFFSET_010F1E40);
        }
        match string_buffer_seek_area(e, base, offset, 0x20, 0x30) {
            Ok(()) => {
                if mode & 2 != 0 && at(e, 0x24) != 0 {
                    string_buffer_sync_put(e, base);
                }
            }
            Err(bad) => offset = bad,
        }
    } else if mode & 2 != 0 && at(e, 0x24) != 0 {
        if way == 2 {
            offset = e
                .mem
                .u32(base + 0x3c)
                .wrapping_sub(at(e, 0x10))
                .wrapping_add(offset);
        } else if way == 1 {
            offset = at(e, 0x24).wrapping_sub(at(e, 0x10)).wrapping_add(offset);
        } else if way != 0 {
            offset = e.global::<u32>(BAD_OFFSET_010F1E40);
        }
        if let Err(bad) = string_buffer_seek_area(e, base, offset, 0x24, 0x34) {
            offset = bad;
        }
    } else {
        offset = e.global::<u32>(BAD_OFFSET_010F1E40);
    }
    write_position(e, out, offset)
}

// Translated from 00acc490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_stringbuf<char>::seekpos(position, mode)` shape (thiscall, eight words:
/// the file position it fills, the six words of the position passed by value and the mode,
/// `RET 0x20`). The offset is the position's `+0` plus its file position's low word (`+8`).
/// Raises the high-water mark first. Unless the offset is the bad offset: with `in` and a
/// read area it moves the read pointer there (in range, else the bad offset) and with `out`
/// and a write area the write pointer along; otherwise with `out` and a write area it moves
/// the write pointer; else the bad offset. Returns the position.
#[allow(clippy::too_many_arguments)]
pub fn fn_00acc490(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    position_offset: u32,
    _unused_2: u32,
    position_low: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    mode: u32,
) -> Ptr {
    let base = this.addr();
    let at = |e: &Engine, slot: u32| e.mem.u32(e.mem.u32(base + slot));
    let mut offset = position_low.wrapping_add(position_offset);
    string_buffer_raise_mark(e, base);
    if offset != e.global::<u32>(BAD_OFFSET_010F1E40) {
        if mode & 1 != 0 && at(e, 0x20) != 0 {
            match string_buffer_seek_area(e, base, offset, 0x20, 0x30) {
                Ok(()) => {
                    if mode & 2 != 0 && at(e, 0x24) != 0 {
                        string_buffer_sync_put(e, base);
                    }
                }
                Err(bad) => offset = bad,
            }
        } else if mode & 2 != 0 && at(e, 0x24) != 0 {
            if let Err(bad) = string_buffer_seek_area(e, base, offset, 0x24, 0x34) {
                offset = bad;
            }
        } else {
            offset = e.global::<u32>(BAD_OFFSET_010F1E40);
        }
    }
    write_position(e, out, offset)
}

// Translated from 00acc6c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acc6c0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_ifstream<char>(name, mode, protection)` constructor (RTTI of its vtables) (thiscall, four
/// words: the file name, the open mode, the protection and the "construct the virtual base"
/// flag, `RET 0x10`). With the flag it stores the virtual-base table `010a3178` and the
/// `ios_base`/`basic_ios` vtables at `+0x58`; a compiled-out branch (`XOR EDX,EDX; JZ`, kept)
/// would store `010a30ac` and the same at `+8`. Stores `010a30a8` at the virtual base, zeroes
/// the count at `+4`, initialises the `basic_ios` with the buffer at `+8` ([`fn_00ad0470`]),
/// stores `010a3174` at the virtual base, constructs the file buffer at `+8`
/// ([`fn_00ad0630`], vtable `010a3138`, [`fn_00acfc10`] with no file) and opens the name with
/// the mode plus `in` ([`fn_00acfab0`]); a failed open sets `failbit` (the compiler's
/// always-true test is kept), plus `badbit` without a buffer. Returns `this`.
pub fn fn_00acc6c0(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    mode: u32,
    protection: u32,
    construct_base: u32,
) -> Ptr {
    let base = this.addr();
    if construct_base != 0 {
        e.mem.set_u32(base, VBTABLE_IFSTREAM_010A3178);
        e.mem.set_u32(base + 0x58, VTABLE_IOS_BASE_010A2F9C);
        e.mem.set_u32(base + 0x58, VTABLE_BASIC_IOS_010A2FA4);
    }
    // `XOR EDX,EDX; JZ`: this branch is compiled out (the constant 0).
    if std::hint::black_box(0u32) != 0 {
        e.mem.set_u32(base, VBTABLE_ISTREAM_010A30AC);
        e.mem.set_u32(base + 8, VTABLE_IOS_BASE_010A2F9C);
        e.mem.set_u32(base + 8, VTABLE_BASIC_IOS_010A2FA4);
    }
    let ios = stream_base(e, base);
    e.mem.set_u32(ios, VTABLE_ISTREAM_010A30A8);
    e.mem.set_u32(base + 4, 0);
    let ios = stream_base(e, base);
    fn_00ad0470(e, Ptr::new(ios), base + 8, 0);
    let ios = stream_base(e, base);
    e.mem.set_u32(ios, VTABLE_IFSTREAM_010A3174);
    let buffer = Ptr::new(base + 8);
    fn_00ad0630(e, buffer);
    e.mem.set_u32(buffer.addr(), VTABLE_FILEBUF_010A3138);
    fn_00acfc10(e, buffer, 0, 0);
    if fn_00acfab0(e, buffer, name, mode | 1, protection).addr() == 0 {
        let ios = stream_base(e, base);
        // `MOV ECX,2; TEST ECX,ECX; JZ`: the state is the constant 2.
        let state = std::hint::black_box(2u32);
        if state != 0 {
            let mut new_state = e.mem.u32(ios + 8) | state;
            if e.mem.u32(ios + 0x28) == 0 {
                new_state |= 4;
            }
            ios_base_clear(e, Ptr::new(ios), new_state, 0);
        }
    }
    this
}

// Translated from 00acc8c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acc8c0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the destructor at the virtual base of the stream [`fn_00acc6c0`] builds
/// (thiscall; the stream starts 0x58 bytes below `this`): stores `010a3174` at the virtual
/// base, the file buffer vtable `010a3138` at the buffer (`this - 0x50`), closes an open file
/// (byte `+0x48` of the buffer, [`fn_00acfb90`]), runs the stream-buffer destructor
/// ([`fn_00acb6b0`]) and stores `010a30a8` at the virtual base.
pub fn fn_00acc8c0(e: &mut Engine, this: Ptr) {
    let stream = this.addr().wrapping_sub(0x58);
    let ios = stream_base(e, stream);
    e.mem.set_u32(ios, VTABLE_IFSTREAM_010A3174);
    let buffer = this.addr().wrapping_sub(0x50);
    e.mem.set_u32(buffer, VTABLE_FILEBUF_010A3138);
    if e.mem.u8(buffer + 0x48) != 0 {
        fn_00acfb90(e, Ptr::new(buffer));
    }
    fn_00acb6b0(e, Ptr::new(buffer));
    let stream = buffer.wrapping_sub(8);
    let ios = stream_base(e, stream);
    e.mem.set_u32(ios, VTABLE_ISTREAM_010A30A8);
}

/// The characters of a string (`basic_string<char>`: inline at `+4` while its capacity at
/// `+0x18` is below 0x10, else the pointer at `+4`; length `+0x14`).
fn string_chars(e: &Engine, string: u32) -> u32 {
    if e.mem.u32(string + 0x18) < 0x10 {
        string + 4
    } else {
        e.mem.u32(string + 4)
    }
}

/// The CRT's `fputc` (`00ecd3c2`, cdecl: the character, sign-extended, and the file) and
/// `fwrite` (`00ecb086`, cdecl: buffer, size, count, file).
const CRT_FPUTC: u32 = 0x00ec_d3c2;
const CRT_FWRITE: u32 = 0x00ec_b086;
/// `0044a8d0` (thiscall, two words: the new size and a flag): the string's `_Grow`; `AL` says
/// whether there is room. `00aae180` (thiscall, three words: offset, count, character): the
/// string's `_Chassign`. `00aadeb0` (thiscall, two words: count, character): the string's
/// `append(count, character)`. `00ec30b0` (no arguments): the string's `_Xlen`.
const STRING_GROW_0044A8D0: u32 = 0x0044_a8d0;
const STRING_FILL_00AAE180: u32 = 0x00aa_e180;
const STRING_APPEND_FILL_00AADEB0: u32 = 0x00aa_deb0;
const STRING_TOO_LONG_00EC30B0: u32 = 0x00ec_30b0;

// Translated from 00acc9f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acc9f0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and the /GS cookie check (00ec408c) are not translated
/// Unnamed: the `basic_filebuf<char>::overflow(ch)` shape (thiscall, one word, `RET 4`; the
/// file at `+0x4c`, the converter at `+0x3c`, its state at `+0x44`). -1 returns 0
/// (`not_eof`; the exe's other arm is kept). With room in the put area it stores the
/// character there. Without a file it fails (-1); without a converter it writes the
/// character with `fputc` (sign-extended) and returns it, or -1. Otherwise it converts the
/// one character through the converter's slot `0x14` (`out`) into an 8-character string
/// ([`fn_00ad0360`]; its pointers through the checked iterators [`fn_00ad3550`] and
/// [`fn_00ad2f30`]): a negative result or one above 1 other than 3 fails; 3 (no conversion)
/// writes the character with `fputc`. With 0 or 1 it writes what was produced (`fwrite`,
/// failing when short), marks the buffer as written (`+0x41`), and returns the character once
/// it was consumed; when nothing was produced it grows the string by 8 (`00aadeb0`) while it
/// is below 0x20 characters and tries again, else fails. The string is destroyed
/// (`0044a730(1, 0)`) on every way out of the conversion.
pub fn fn_00acc9f0(e: &mut Engine, this: Ptr, character: u32) -> u32 {
    let base = this.addr();
    if character == 0xffff_ffff {
        // `not_eof`: the exe's `NEG`/`SBB`/`ADD 1` of -1 gives 0.
        return if character != 0xffff_ffff {
            character
        } else {
            0
        };
    }
    let next = e.mem.u32(e.mem.u32(base + 0x24));
    if next != 0
        && e.mem.u32(e.mem.u32(base + 0x24))
            < e.mem
                .u32(e.mem.u32(base + 0x24))
                .wrapping_add(e.mem.u32(e.mem.u32(base + 0x34)))
    {
        let count_at = e.mem.u32(base + 0x34);
        let count = e.mem.u32(count_at).wrapping_sub(1);
        e.mem.set_u32(count_at, count);
        let next_at = e.mem.u32(base + 0x24);
        let slot = e.mem.u32(next_at);
        e.mem.set_u32(next_at, slot.wrapping_add(1));
        e.mem.set_u8(slot, character as u8);
        return character;
    }
    if e.mem.u32(base + 0x4c) == 0 {
        return 0xffff_ffff;
    }
    if e.mem.u32(base + 0x3c) == 0 {
        let file = e.mem.u32(base + 0x4c);
        let written = e
            .call(CRT_FPUTC, &args![character as u8 as i8 as i32, file])
            .u32();
        return if written != 0xffff_ffff {
            character
        } else {
            0xffff_ffff
        };
    }
    e.with_stack(0x58, |e, frame| {
        // Frame (the exe's `EBP - 0x64` at +0): four checked iterators at +0, +8, +0x10 and
        // +0x18, the string at +0x28, the converter's "to next" at +0x48, the character byte
        // at +0x4f directly followed by the "from next" word at +0x50 (the exe passes that
        // word's address as the end of the one-character source), the constant 8 at +0x54.
        let frame = frame.addr();
        let string = frame + 0x28;
        let to_next = frame + 0x48;
        let source = frame + 0x4f;
        let from_next = frame + 0x50;
        e.mem.set_u32(frame + 0x54, 8);
        e.mem.set_u8(source, character as u8);
        e.call(STRING_TIDY_0044A730, &args![string, 0u32, 0u32]);
        fn_00ad0360(e, Ptr::new(string), 8, 0);
        let result = loop {
            let iterator = Ptr::new(frame + 0x18);
            let data = string_chars(e, string);
            fn_00ad3550(e, iterator, data, string);
            let first = fn_00ad2f30(e, iterator);
            let size = e.mem.u32(string + 0x14);
            let iterator = Ptr::new(frame + 0x10);
            let data = string_chars(e, string);
            fn_00ad3550(e, iterator, data, string);
            let to = fn_00ad2f30(e, iterator);
            let converter = e.mem.u32(base + 0x3c);
            let converted = e
                .vcall(
                    converter,
                    0x14,
                    &args![
                        base + 0x44,
                        source,
                        from_next,
                        from_next,
                        to,
                        first.wrapping_add(size),
                        to_next
                    ],
                )
                .i32();
            if converted < 0 {
                break 0xffff_ffff;
            }
            if converted > 1 {
                if converted != 3 {
                    break 0xffff_ffff;
                }
                let file = e.mem.u32(base + 0x4c);
                let byte = e.mem.u8(source) as i8 as i32;
                let written = e.call(CRT_FPUTC, &args![byte, file]).u32();
                break if written != 0xffff_ffff {
                    character
                } else {
                    0xffff_ffff
                };
            }
            let iterator = Ptr::new(frame + 8);
            let data = string_chars(e, string);
            fn_00ad3550(e, iterator, data, string);
            let start = fn_00ad2f30(e, iterator);
            let produced = e.mem.u32(to_next).wrapping_sub(start);
            if produced != 0 {
                let iterator = Ptr::new(frame);
                let data = string_chars(e, string);
                fn_00ad3550(e, iterator, data, string);
                let start = fn_00ad2f30(e, iterator);
                let file = e.mem.u32(base + 0x4c);
                let written = e
                    .call(CRT_FWRITE, &args![start, 1u32, produced, file])
                    .u32();
                if produced != written {
                    break 0xffff_ffff;
                }
            }
            e.mem.set_u8(base + 0x41, 1);
            if e.mem.u32(from_next) != source {
                break character;
            }
            if produced == 0 {
                if e.mem.u32(string + 0x14) < 0x20 {
                    e.call(STRING_APPEND_FILL_00AADEB0, &args![string, 8u32, 0u32]);
                } else {
                    break 0xffff_ffff;
                }
            }
        };
        e.call(STRING_TIDY_0044A730, &args![string, 1u32, 0u32]);
        result
    })
}

// Translated from 00ad0360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `basic_string<char>::assign(count, character)` (thiscall, two words, `RET 8`). A
/// count equal to `npos` (`010173f0`) calls `_Xlen` (`00ec30b0`); when `_Grow(count, 0)`
/// (`0044a8d0`, its `AL`) has room it fills the string (`00aae180(0, count, character)`, the
/// character byte zero-extended), stores the size at `+0x14` and a terminating zero after
/// it. Returns `this`.
pub fn fn_00ad0360(e: &mut Engine, this: Ptr, count: u32, character: u32) -> Ptr {
    let base = this.addr();
    if count == e.global::<u32>(STRING_NPOS_010173F0) {
        e.call(STRING_TOO_LONG_00EC30B0, &args![]);
    }
    if e.call(STRING_GROW_0044A8D0, &args![base, count, 0u32])
        .bool()
    {
        e.call(
            STRING_FILL_00AAE180,
            &args![base, 0u32, count, character & 0xff],
        );
        e.mem.set_u32(base + 0x14, count);
        let data = string_chars(e, base);
        e.mem.set_u8(data.wrapping_add(count), 0);
    }
    this
}

// Translated from 00ad2f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked string iterator's `operator*` address (thiscall; the string `+0`, the
/// pointer `+4`). Unless the string word is -4 it calls `_invalid_parameter_noinfo` for a null
/// string and for a pointer at or past the string's end; returns the pointer.
pub fn fn_00ad2f30(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u32(base) != 0xffff_fffc {
        if e.mem.u32(base) == 0 {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let string = e.mem.u32(base);
        let end = string_chars(e, string).wrapping_add(e.mem.u32(e.mem.u32(base) + 0x14));
        if e.mem.u32(base + 4) >= end {
            e.call(INVALID_PARAMETER, &args![]);
        }
    }
    e.mem.u32(base + 4)
}

// Translated from 00ad3550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked string iterator's constructor (thiscall, two words: the pointer and
/// the string, `RET 8`). Zeroes `+0`; calls `_invalid_parameter_noinfo` for a null string or
/// pointer, or a pointer outside the string's characters (end included); stores the string
/// at `+0` and the pointer at `+4`. Returns `this`.
pub fn fn_00ad3550(e: &mut Engine, this: Ptr, pointer: u32, string: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    if string == 0
        || pointer == 0
        || string_chars(e, string) > pointer
        || pointer > string_chars(e, string).wrapping_add(e.mem.u32(string + 0x14))
    {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.set_u32(base, string);
    e.mem.set_u32(base + 4, pointer);
    this
}

/// The CRT's `ungetc` (`00ecd5bf`, cdecl: the character and the file) and `getc`
/// (`00ecd631`, cdecl: the file).
const CRT_UNGETC: u32 = 0x00ec_d5bf;
const CRT_GETC: u32 = 0x00ec_d631;
/// `0044a690` (thiscall, two words: offset, count): the string's `erase`.
const STRING_ERASE_0044A690: u32 = 0x0044_a690;

// Translated from 00accef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_filebuf<char>::pbackfail(ch)` shape (thiscall, one word, `RET 4`).
/// With a read pointer past the area start and the character -1 or equal to the byte before
/// it, steps the read pointer back ([`basic_streambuf_gndec`]) and returns the character (-1 gives 0).
/// Otherwise -1 without a file or for -1; without a converter it pushes the character back
/// with `ungetc` (the byte zero-extended) and returns it when that worked. Everything else
/// returns -1: the exe's branch that would keep the character in the buffer's own byte at
/// `+0x40` is compiled out (`XOR EAX,EAX; JZ`) and kept as such.
pub fn fn_00accef0(e: &mut Engine, this: Ptr, character: u32) -> u32 {
    let base = this.addr();
    let next = e.mem.u32(e.mem.u32(base + 0x20));
    if next != 0 && e.mem.u32(e.mem.u32(base + 0x10)) < e.mem.u32(e.mem.u32(base + 0x20)) {
        let matches = character == 0xffff_ffff || {
            let previous = e.mem.u32(e.mem.u32(base + 0x20)).wrapping_sub(1);
            u32::from(e.mem.u8(previous)) == character
        };
        if matches {
            basic_streambuf_gndec(e, this);
            return if character != 0xffff_ffff {
                character
            } else {
                0
            };
        }
    }
    if e.mem.u32(base + 0x4c) == 0 || character == 0xffff_ffff {
        return 0xffff_ffff;
    }
    if e.mem.u32(base + 0x3c) == 0 {
        let file = e.mem.u32(base + 0x4c);
        if e.call(CRT_UNGETC, &args![character & 0xff, file]).u32() != 0xffff_ffff {
            return character;
        }
    }
    // `XOR EAX,EAX; JZ`: the branch below is compiled out (the constant 0).
    if std::hint::black_box(0u32) != 0 {
        e.mem.set_u8(base + 0x40, character as u8);
        let target = e.mem.u32(base + 0x10);
        e.mem.set_u32(target, base + 0x40);
        let target = e.mem.u32(base + 0x20);
        e.mem.set_u32(target, base + 0x40);
        let target = e.mem.u32(base + 0x30);
        e.mem
            .set_u32(target, (base + 0x41).wrapping_sub(base + 0x40));
        return character;
    }
    0xffff_ffff
}

// Translated from 00acd060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_filebuf<char>::underflow()` shape (thiscall): the byte at the read
/// pointer while characters are left; otherwise takes one through the buffer's slot `0x14`
/// (`uflow`) and, unless it is -1, gives it back through slot 8 (`pbackfail`). Returns the
/// character.
pub fn fn_00acd060(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let next = e.mem.u32(e.mem.u32(base + 0x20));
    if next != 0
        && e.mem.u32(e.mem.u32(base + 0x20))
            < e.mem
                .u32(e.mem.u32(base + 0x20))
                .wrapping_add(e.mem.u32(e.mem.u32(base + 0x30)))
    {
        return u32::from(e.mem.u8(e.mem.u32(e.mem.u32(base + 0x20))));
    }
    let character = e.vcall(base, 0x14, &args![]).u32();
    if character == 0xffff_ffff {
        return character;
    }
    e.vcall(base, 8, &args![character]);
    character
}

// Translated from 00acd100 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acd100 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and the /GS cookie check (00ec408c) are not translated
/// Unnamed: the `basic_filebuf<char>::uflow()` shape (thiscall). Takes the byte at the read
/// pointer while characters are left. Without a file -1; without a converter one `getc`
/// (its byte zero-extended, or -1). Otherwise it reads bytes with `getc` into a string
/// (`00aadeb0(1, byte)`) and converts them through the converter's slot `0x10` (`in`) into
/// one character: a negative result, or one above 1 other than 3, fails; with 0 or 1 and a
/// character produced it pushes the unconsumed bytes back (`ungetc`, last first) and returns
/// the character, else it drops the consumed bytes (`0044a690(0, n)`) and reads on; with 3
/// (no conversion) and a byte read it copies the first byte (`memcpy_s`) and returns it. An
/// end of file returns -1. The string is destroyed on every way out.
pub fn fn_00acd100(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let next = e.mem.u32(e.mem.u32(base + 0x20));
    if next != 0
        && e.mem.u32(e.mem.u32(base + 0x20))
            < e.mem
                .u32(e.mem.u32(base + 0x20))
                .wrapping_add(e.mem.u32(e.mem.u32(base + 0x30)))
    {
        let count_at = e.mem.u32(base + 0x30);
        let count = e.mem.u32(count_at).wrapping_sub(1);
        e.mem.set_u32(count_at, count);
        let next_at = e.mem.u32(base + 0x20);
        let slot = e.mem.u32(next_at);
        e.mem.set_u32(next_at, slot.wrapping_add(1));
        return u32::from(e.mem.u8(slot));
    }
    if e.mem.u32(base + 0x4c) == 0 {
        return 0xffff_ffff;
    }
    if e.mem.u32(base + 0x3c) == 0 {
        let file = e.mem.u32(base + 0x4c);
        let read = e.call(CRT_GETC, &args![file]).u32();
        return if read == 0xffff_ffff {
            0xffff_ffff
        } else {
            read & 0xff
        };
    }
    e.with_stack(0x60, |e, frame| {
        // Frame (the exe's `EBP - 0x74` at +0): checked iterators at +0, +8, +0x14, +0x1c and
        // +0x24, the converter's "to next" at +0x34, the character byte at +0x3b directly
        // followed by the "from next" word at +0x3c (the exe passes that word's address as
        // the end of the one-character destination), the string at +0x44.
        let frame = frame.addr();
        let to_next = frame + 0x34;
        let target = frame + 0x3b;
        let from_next = frame + 0x3c;
        let string = frame + 0x44;
        e.call(STRING_TIDY_0044A730, &args![string, 0u32, 0u32]);
        let result = loop {
            let file = e.mem.u32(base + 0x4c);
            let read = e.call(CRT_GETC, &args![file]).u32();
            if read == 0xffff_ffff {
                break 0xffff_ffff;
            }
            e.call(
                STRING_APPEND_FILL_00AADEB0,
                &args![string, 1u32, read & 0xff],
            );
            let iterator = Ptr::new(frame + 0x24);
            let data = string_chars(e, string);
            fn_00ad3550(e, iterator, data, string);
            let first = fn_00ad2f30(e, iterator);
            let size = e.mem.u32(string + 0x14);
            let iterator = Ptr::new(frame + 0x1c);
            let data = string_chars(e, string);
            fn_00ad3550(e, iterator, data, string);
            let from = fn_00ad2f30(e, iterator);
            let converter = e.mem.u32(base + 0x3c);
            let converted = e
                .vcall(
                    converter,
                    0x10,
                    &args![
                        base + 0x44,
                        from,
                        first.wrapping_add(size),
                        from_next,
                        target,
                        from_next,
                        to_next
                    ],
                )
                .i32();
            if converted < 0 {
                break 0xffff_ffff;
            }
            if converted > 1 {
                if converted != 3 {
                    break 0xffff_ffff;
                }
                if e.mem.u32(string + 0x14) >= 1 {
                    let iterator = Ptr::new(frame);
                    let data = string_chars(e, string);
                    fn_00ad3550(e, iterator, data, string);
                    let first = fn_00ad2f30(e, iterator);
                    e.call(MEMCPY_S, &args![target, 1u32, first, 1u32]);
                    break u32::from(e.mem.u8(target));
                }
                continue;
            }
            if e.mem.u32(to_next) != target {
                let iterator = Ptr::new(frame + 0x14);
                let data = string_chars(e, string);
                fn_00ad3550(e, iterator, data, string);
                let first = fn_00ad2f30(e, iterator);
                let size = e.mem.u32(string + 0x14);
                let mut left = first.wrapping_add(size).wrapping_sub(e.mem.u32(from_next)) as i32;
                while left > 0 {
                    left -= 1;
                    let file = e.mem.u32(base + 0x4c);
                    let byte = e.mem.u8(e.mem.u32(from_next).wrapping_add(left as u32));
                    e.call(CRT_UNGETC, &args![byte as i8 as i32, file]);
                }
                break u32::from(e.mem.u8(target));
            }
            let iterator = Ptr::new(frame + 8);
            let data = string_chars(e, string);
            fn_00ad3550(e, iterator, data, string);
            let first = fn_00ad2f30(e, iterator);
            let consumed = e.mem.u32(from_next).wrapping_sub(first);
            e.call(STRING_ERASE_0044A690, &args![string, 0u32, consumed]);
        };
        e.call(STRING_TIDY_0044A730, &args![string, 1u32, 0u32]);
        result
    })
}

/// The CRT's `fseek` (`00ecb65a`, cdecl: file, offset, origin), `fgetpos` (`00ecd735`, cdecl:
/// file, position) and `fsetpos` (`00ecd7a3`, cdecl: file, position).
const CRT_FSEEK: u32 = 0x00ec_b65a;
const CRT_FGETPOS: u32 = 0x00ec_d735;
const CRT_FSETPOS: u32 = 0x00ec_d7a3;

/// After a successful file seek: a read pointer on the buffer's own byte (`+0x40`) is reset
/// to an empty area there (start `+0x40`, read pointer `+0x41`, count 0). Then the position
/// is filled: 0 at `+0`, the file position (`fpos`, two words) at `+8`, the converter state
/// (`+0x44`) at `+0x10`.
fn file_buffer_seek_done(e: &mut Engine, base: u32, out: Ptr, position: (u32, u32)) -> Ptr {
    if e.mem.u32(e.mem.u32(base + 0x20)) == base + 0x40 {
        let end = base + 0x41;
        let target = e.mem.u32(base + 0x10);
        e.mem.set_u32(target, base + 0x40);
        let target = e.mem.u32(base + 0x20);
        e.mem.set_u32(target, end);
        let target = e.mem.u32(base + 0x30);
        e.mem.set_u32(target, (base + 0x41).wrapping_sub(end));
    }
    let state = e.mem.u32(base + 0x44);
    e.mem.set_u32(out.addr(), 0);
    e.mem.set_u32(out.addr() + 8, position.0);
    e.mem.set_u32(out.addr() + 0xc, position.1);
    e.mem.set_u32(out.addr() + 0x10, state);
    out
}

/// A failed seek's position: the bad offset (`010f1e40`), zeros and the zero state.
fn file_buffer_seek_failed(e: &mut Engine, out: Ptr) -> Ptr {
    let offset = e.global::<u32>(BAD_OFFSET_010F1E40);
    write_position(e, out, offset)
}

// Translated from 00acd5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_filebuf<char>::seekoff(offset, way, mode)` shape (thiscall, four
/// words: the file position it fills, the offset, the direction and the mode it does not
/// read, `RET 0x10`). A relative seek while a put-back character sits in the buffer's own
/// byte (and no converter) counts it (offset - 1). With a file, after [`fn_00acfcd0`]
/// succeeded, it seeks (`fseek`; a relative seek by 0 is skipped) and reads the position
/// (`fgetpos`): on success the put-back area is reset and the position returned (see the
/// helper); otherwise the bad position.
pub fn fn_00acd5b0(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    offset: u32,
    way: u32,
    _unused_4: u32,
) -> Ptr {
    let base = this.addr();
    let mut offset = offset;
    if e.mem.u32(e.mem.u32(base + 0x20)) == base + 0x40 && way == 1 && e.mem.u32(base + 0x3c) == 0 {
        offset = offset.wrapping_sub(1);
    }
    e.with_stack(8, |e, position| {
        let position = position.addr();
        let ok = e.mem.u32(base + 0x4c) != 0
            && fn_00acfcd0(e, this) != 0
            && ((offset == 0 && way == 1) || {
                let file = e.mem.u32(base + 0x4c);
                e.call(CRT_FSEEK, &args![file, offset, way]).u32() == 0
            })
            && {
                let file = e.mem.u32(base + 0x4c);
                e.call(CRT_FGETPOS, &args![file, position]).u32() == 0
            };
        if !ok {
            return file_buffer_seek_failed(e, out);
        }
        let low = e.mem.u32(position);
        let high = e.mem.u32(position + 4);
        file_buffer_seek_done(e, base, out, (low, high))
    })
}

// Translated from 00acd760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_filebuf<char>::seekpos(position, mode)` shape (thiscall, eight words:
/// the file position it fills, the six words of the position by value — offset, an unread
/// word, the file position's two words, the converter state, an unread word — and the mode it
/// does not read, `RET 0x20`). With a file, after [`fn_00acfcd0`] succeeded, it sets the file
/// position (`fsetpos`), seeks on by the offset (`fseek` relative, skipped for 0; the exe
/// computes it as `low + offset - low`) and reads the position back (`fgetpos`): on success
/// it stores the state at `+0x44`, resets the put-back area and returns the position;
/// otherwise the bad position.
#[allow(clippy::too_many_arguments)]
pub fn fn_00acd760(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    offset: u32,
    _unused_2: u32,
    position_low: u32,
    position_high: u32,
    state: u32,
    _unused_6: u32,
    _unused_7: u32,
) -> Ptr {
    let base = this.addr();
    e.with_stack(8, |e, position| {
        let position = position.addr();
        e.mem.set_u32(position, position_low);
        e.mem.set_u32(position + 4, position_high);
        let delta = position_low
            .wrapping_add(offset)
            .wrapping_sub(e.mem.u32(position));
        let ok = e.mem.u32(base + 0x4c) != 0
            && fn_00acfcd0(e, this) != 0
            && {
                let file = e.mem.u32(base + 0x4c);
                e.call(CRT_FSETPOS, &args![file, position]).u32() == 0
            }
            && (delta == 0 || {
                let file = e.mem.u32(base + 0x4c);
                e.call(CRT_FSEEK, &args![file, delta, 1u32]).u32() == 0
            })
            && {
                let file = e.mem.u32(base + 0x4c);
                e.call(CRT_FGETPOS, &args![file, position]).u32() == 0
            };
        if !ok {
            return file_buffer_seek_failed(e, out);
        }
        e.mem.set_u32(base + 0x44, state);
        let low = e.mem.u32(position);
        let high = e.mem.u32(position + 4);
        file_buffer_seek_done(e, base, out, (low, high))
    })
}

/// The CRT's `setvbuf` (`00ecd7e8`, cdecl: file, buffer, mode, size) and `fflush`
/// (`00ecda70`, cdecl: file).
const CRT_SETVBUF: u32 = 0x00ec_d7e8;
const CRT_FFLUSH: u32 = 0x00ec_da70;

// Translated from 00acd930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::basic_filebuf<char>::setbuf(buffer, size)` (Ghidra library match; thiscall, two
/// words, `RET 8`): without a file returns 0; otherwise gives the file the buffer with
/// `setvbuf` (mode 4, unbuffered, for a null buffer of size 0, else 0) and returns 0 when
/// that fails; else re-initialises the buffer on the file ([`fn_00acfc10`] with mode 1) and
/// returns `this`.
pub fn basic_filebuf_setbuf(e: &mut Engine, this: Ptr, buffer: u32, size: u32) -> Ptr {
    let base = this.addr();
    if e.mem.u32(base + 0x4c) == 0 {
        return Ptr::new(0);
    }
    let mode = if buffer == 0 && size == 0 { 4u32 } else { 0 };
    let file = e.mem.u32(base + 0x4c);
    if e.call(CRT_SETVBUF, &args![file, buffer, mode, size]).u32() != 0 {
        return Ptr::new(0);
    }
    let file = e.mem.u32(base + 0x4c);
    fn_00acfc10(e, this, file, 1);
    this
}

// Translated from 00acd9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_filebuf<char>::sync()` shape (thiscall): 0 without a file; otherwise
/// it flushes the put area through the buffer's slot 4 (`overflow(-1)`); unless that returned
/// -1 it flushes the file (`fflush`) and returns -1 when that is negative. Else 0.
pub fn fn_00acd9a0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + 0x4c) != 0 && e.vcall(base, 4, &args![0xffff_ffffu32]).u32() != 0xffff_ffff
    {
        let file = e.mem.u32(base + 0x4c);
        if (e.call(CRT_FFLUSH, &args![file]).u32() as i32) < 0 {
            return 0xffff_ffff;
        }
    }
    0
}

// Translated from 00acda10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_filebuf<char>::imbue(locale)` shape (thiscall, one word, `RET 4`):
/// takes the locale's converter facet ([`fn_00ad4fd0`]); when its slot 4 (`always_noconv`,
/// `AL`) says so the buffer keeps no converter (`+0x3c` = 0), otherwise it keeps the facet
/// and re-initialises the buffer pointers ([`fn_00ad0720`]).
pub fn fn_00acda10(e: &mut Engine, this: Ptr, locale: u32) {
    let base = this.addr();
    let converter = fn_00ad4fd0(e, Ptr::new(locale)).addr();
    if e.vcall(converter, 4, &args![]).bool() {
        e.mem.set_u32(base + 0x3c, 0);
    } else {
        e.mem.set_u32(base + 0x3c, converter);
        fn_00ad0720(e, this);
    }
}

// Translated from 00acda90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `push_back(value)` of a vector of 0x18-byte elements (thiscall, one word,
/// `RET 4`): as [`fn_00acab30`], constructing the element with [`fn_00ad5cc0`] and inserting
/// with [`fn_00acffe0`].
pub fn fn_00acda90(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    let first = e.mem.u32(base + 0xc);
    let size = ((e.mem.u32(base + 0x10).wrapping_sub(first)) as i32 / 0x18) as u32;
    let capacity = if first == 0 {
        0
    } else {
        ((e.mem.u32(base + 0x14).wrapping_sub(first)) as i32 / 0x18) as u32
    };
    if size < capacity {
        let last = e.mem.u32(base + 0x10);
        fn_00ad5cc0(e, last, 1, value, base + 8, 0, 0);
        e.mem.set_u32(base + 0x10, last.wrapping_add(0x18));
    } else {
        let last = e.mem.u32(base + 0x10);
        let (proxy, at) = vector_iterator(e, base, last);
        e.with_stack(8, |e, out| {
            fn_00acffe0(e, this, out, proxy, at, value);
        });
    }
}

// Translated from 00acdba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `clear()` of the vector of 0x18-byte elements (thiscall): checked iterators at
/// the end and then the start, erased with [`fn_00ad0110`].
pub fn fn_00acdba0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let last = e.mem.u32(base + 0x10);
    let end = vector_iterator(e, base, last);
    let first = e.mem.u32(base + 0xc);
    let begin = vector_iterator(e, base, first);
    e.with_stack(8, |e, out| {
        fn_00ad0110(e, this, out, begin.0, begin.1, end.0, end.1);
    });
}

/// The `basic_ofstream<char>` tables (`.rdata`; `010a3184` RTTI checked): its virtual-base
/// table `010a3198`, the one of the compiled-out branch `010a3190`, its vtable `010a3184`;
/// and the `basic_ostream<char>` vtable `010a318c`.
const VBTABLE_OFSTREAM_010A3198: u32 = 0x010a_3198;
const VBTABLE_OSTREAM_010A3190: u32 = 0x010a_3190;
const VTABLE_OFSTREAM_010A3184: u32 = 0x010a_3184;
const VTABLE_OSTREAM_010A318C: u32 = 0x010a_318c;

// Translated from 00acdc90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acdc90 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_ofstream<char>(name, mode, protection)` constructor (RTTI of its
/// vtable; thiscall, four words: the file name, the open mode, the protection and the
/// "construct the virtual base" flag, `RET 0x10`). With the flag it stores the virtual-base
/// table `010a3198` and the `ios_base`/`basic_ios` vtables at `+0x54`; a compiled-out branch
/// (`XOR EDX,EDX; JZ`, kept) would store `010a3190` and the same at `+4`. Stores `010a318c`
/// at the virtual base, initialises the `basic_ios` with the buffer at `+4`
/// ([`fn_00ad0470`]), stores `010a3184` at the virtual base, constructs the file buffer at
/// `+4` ([`fn_00ad0630`], vtable `010a3138`, [`fn_00acfc10`] with no file) and opens the name
/// with the mode plus `out` (2) ([`fn_00acfab0`]); a failed open sets `failbit` (the
/// compiler's always-true test is kept), plus `badbit` without a buffer. Returns `this`.
pub fn fn_00acdc90(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    mode: u32,
    protection: u32,
    construct_base: u32,
) -> Ptr {
    let base = this.addr();
    if construct_base != 0 {
        e.mem.set_u32(base, VBTABLE_OFSTREAM_010A3198);
        e.mem.set_u32(base + 0x54, VTABLE_IOS_BASE_010A2F9C);
        e.mem.set_u32(base + 0x54, VTABLE_BASIC_IOS_010A2FA4);
    }
    // `XOR EDX,EDX; JZ`: this branch is compiled out (the constant 0).
    if std::hint::black_box(0u32) != 0 {
        e.mem.set_u32(base, VBTABLE_OSTREAM_010A3190);
        e.mem.set_u32(base + 4, VTABLE_IOS_BASE_010A2F9C);
        e.mem.set_u32(base + 4, VTABLE_BASIC_IOS_010A2FA4);
    }
    let ios = stream_base(e, base);
    e.mem.set_u32(ios, VTABLE_OSTREAM_010A318C);
    let ios = stream_base(e, base);
    fn_00ad0470(e, Ptr::new(ios), base + 4, 0);
    let ios = stream_base(e, base);
    e.mem.set_u32(ios, VTABLE_OFSTREAM_010A3184);
    let buffer = Ptr::new(base + 4);
    fn_00ad0630(e, buffer);
    e.mem.set_u32(buffer.addr(), VTABLE_FILEBUF_010A3138);
    fn_00acfc10(e, buffer, 0, 0);
    if fn_00acfab0(e, buffer, name, mode | 2, protection).addr() == 0 {
        let ios = stream_base(e, base);
        // `MOV EAX,2; TEST EAX,EAX; JZ`: the state is the constant 2.
        let state = std::hint::black_box(2u32);
        if state != 0 {
            let mut new_state = e.mem.u32(ios + 8) | state;
            if e.mem.u32(ios + 0x28) == 0 {
                new_state |= 4;
            }
            ios_base_clear(e, Ptr::new(ios), new_state, 0);
        }
    }
    this
}

// Translated from 00acde90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::basic_ostream<char>::~basic_ostream` (Ghidra library match; thiscall at the virtual
/// base, the stream 4 bytes below `this`): stores `010a318c` at the stream's virtual base.
pub fn basic_ostream_destructor(e: &mut Engine, this: Ptr) {
    let stream = this.addr().wrapping_sub(4);
    let ios = stream_base(e, stream);
    e.mem.set_u32(ios, VTABLE_OSTREAM_010A318C);
}

// Translated from 00acdeb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acdeb0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_ofstream<char>` destructor at its virtual base (thiscall; the stream
/// starts 0x54 bytes below `this`): stores `010a3184` at the virtual base, the file buffer
/// vtable `010a3138` at the buffer (`this - 0x50`), closes an open file (byte `+0x48`,
/// [`fn_00acfb90`]), runs the stream-buffer destructor ([`fn_00acb6b0`]) and stores
/// `010a318c` at the virtual base.
pub fn fn_00acdeb0(e: &mut Engine, this: Ptr) {
    let stream = this.addr().wrapping_sub(0x54);
    let ios = stream_base(e, stream);
    e.mem.set_u32(ios, VTABLE_OFSTREAM_010A3184);
    let buffer = this.addr().wrapping_sub(0x50);
    e.mem.set_u32(buffer, VTABLE_FILEBUF_010A3138);
    if e.mem.u8(buffer + 0x48) != 0 {
        fn_00acfb90(e, Ptr::new(buffer));
    }
    fn_00acb6b0(e, Ptr::new(buffer));
    let stream = buffer.wrapping_sub(4);
    let ios = stream_base(e, stream);
    e.mem.set_u32(ios, VTABLE_OSTREAM_010A318C);
}

// Translated from 00acdf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a deleting destructor through a base 4 bytes into the object (thiscall, one word,
/// `RET 4`): destroys the object at `this - 4` ([`fn_00ace2d0`]), frees it with flag bit 0
/// and returns it.
pub fn fn_00acdf60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let object = this.addr().wrapping_sub(4);
    fn_00ace2d0(e, Ptr::new(object));
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![object]);
    }
    Ptr::new(object)
}

// Translated from 00acdfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::basic_ios<char>` scalar deleting destructor (the map's vtable name; thiscall, one
/// word, `RET 4`): stores the vtables `010a2fa4` then `010a2f9c`, runs `_Ios_base_dtor` and
/// with flag bit 0 frees `this`. Returns `this`.
pub fn basic_ios_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    destroy_ios(e, this.addr());
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00acdff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a deleting destructor through a base 8 bytes into the object (thiscall, one word,
/// `RET 4`): destroys the object at `this - 8` ([`fn_00ace330`]), frees it with flag bit 0
/// and returns it.
pub fn fn_00acdff0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let object = this.addr().wrapping_sub(8);
    fn_00ace330(e, Ptr::new(object));
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![object]);
    }
    Ptr::new(object)
}

// Translated from 00ace030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf<char>` scalar deleting destructor (thiscall, one word,
/// `RET 4`): the destructor body ([`fn_00acb6b0`]), then with flag bit 0 frees `this`.
/// Returns `this`.
pub fn fn_00ace030(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00acb6b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// The deleting destructor shape of a stream at its virtual base: `this` points `offset`
/// bytes into the object; runs `destroy` there, destroys the `basic_ios` at the same
/// address, frees the object with flag bit 0 and returns it.
fn delete_stream_at_base(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
    offset: u32,
    destroy: fn(&mut Engine, Ptr),
) -> Ptr {
    let object = this.addr().wrapping_sub(offset);
    destroy(e, Ptr::new(object.wrapping_add(offset)));
    destroy_ios(e, object.wrapping_add(offset));
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this.addr().wrapping_sub(offset)]);
    }
    Ptr::new(this.addr().wrapping_sub(offset))
}

// Translated from 00ace060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_istringstream<char>` deleting destructor at its virtual base
/// (thiscall, one word, `RET 4`; the stream starts 0x50 below `this`): [`fn_00acb610`], the
/// `basic_ios` destructor, and with flag bit 0 frees the stream. Returns the stream.
pub fn fn_00ace060(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_stream_at_base(e, this, flags, 0x50, fn_00acb610)
}

// Translated from 00ace0d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ace0d0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_stringbuf<char>` scalar deleting destructor (thiscall, one word,
/// `RET 4`): stores the vtable `010a30c0`, frees its buffer ([`fn_00acf9e0`]), runs the
/// stream-buffer destructor ([`fn_00acb6b0`]) and with flag bit 0 frees `this`. Returns
/// `this`.
pub fn fn_00ace0d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_STRINGBUF_010A30C0);
    fn_00acf9e0(e, this);
    fn_00acb6b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00ace150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_ifstream<char>` deleting destructor at its virtual base (thiscall,
/// one word, `RET 4`; the stream starts 0x58 below `this`): [`fn_00acc8c0`], the `basic_ios`
/// destructor, and with flag bit 0 frees the stream. Returns the stream.
pub fn fn_00ace150(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_stream_at_base(e, this, flags, 0x58, fn_00acc8c0)
}

// Translated from 00ace1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ace1c0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_filebuf<char>` scalar deleting destructor (thiscall, one word,
/// `RET 4`): stores the vtable `010a3138`, closes an open file (byte `+0x48`,
/// [`fn_00acfb90`]), runs the stream-buffer destructor ([`fn_00acb6b0`]) and with flag bit 0
/// frees `this`. Returns `this`.
pub fn fn_00ace1c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_FILEBUF_010A3138);
    if e.mem.u8(base + 0x48) != 0 {
        fn_00acfb90(e, this);
    }
    fn_00acb6b0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![base]);
    }
    this
}

// Translated from 00ace260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_ofstream<char>` deleting destructor at its virtual base (thiscall,
/// one word, `RET 4`; the stream starts 0x54 below `this`): [`fn_00acdeb0`], the `basic_ios`
/// destructor, and with flag bit 0 frees the stream. Returns the stream.
pub fn fn_00ace260(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    delete_stream_at_base(e, this, flags, 0x54, fn_00acdeb0)
}

// Translated from 00ace330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a `basic_istream<char>` destructor with its `basic_ios` at `+8` (thiscall):
/// stores `010a30a8` at the stream's virtual base, then destroys the `basic_ios` at `+8`
/// (vtables `010a2fa4`, `010a2f9c`, `_Ios_base_dtor`).
pub fn fn_00ace330(e: &mut Engine, this: Ptr) {
    let stream = this.addr();
    let ios = stream_base(e, stream);
    e.mem.set_u32(ios, VTABLE_ISTREAM_010A30A8);
    destroy_ios(e, stream + 8);
}

/// Reads the word a stream-buffer slot points at (`[[this + slot]]`).
fn buffer_at(e: &Engine, base: u32, slot: u32) -> u32 {
    e.mem.u32(e.mem.u32(base + slot))
}

/// Adds `delta` to the word a stream-buffer slot points at.
fn buffer_add(e: &mut Engine, base: u32, slot: u32, delta: u32) {
    let target = e.mem.u32(base + slot);
    let value = e.mem.u32(target).wrapping_add(delta);
    e.mem.set_u32(target, value);
}

// Translated from 00ace390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ace390 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_istream<char>::_Ipfx(noskip)` shape (thiscall, one word, `RET 4`;
/// returns `AL`). With a good state it flushes the tied stream (`base + 0x2c`,
/// [`fn_00aca3c0`]); unless the `noskip` byte is set, with `skipws` (bit 0 of `base + 0x10`)
/// it takes the locale's `ctype` facet ([`fn_00ace750`], [`fn_00ad38d0`], the local released
/// with [`fn_00acb720`]) and skips characters whose class bits (the facet's table at `+0x10`,
/// 16-bit entries) have 0x48 set ([`fn_00aca9f0`], [`fn_00ace7d0`]); an end of file sets
/// `eofbit` (the compiler's always-true test is kept). A state still good returns 1;
/// otherwise it sets `failbit` (with `badbit` when there is no buffer; again an always-true
/// test kept) and returns 0.
pub fn fn_00ace390(e: &mut Engine, this: Ptr, noskip: u32) -> u8 {
    let stream = this.addr();
    if e.mem.u32(stream_base(e, stream) + 8) == 0 {
        let tie = e.mem.u32(stream_base(e, stream) + 0x2c);
        if tie != 0 {
            fn_00aca3c0(e, Ptr::new(tie));
        }
        if noskip as u8 == 0 && e.mem.u32(stream_base(e, stream) + 0x10) & 1 != 0 {
            let facet = e.with_stack(4, |e, local| {
                let ios = stream_base(e, stream);
                let locale = fn_00ace750(e, Ptr::new(ios), local);
                let facet = fn_00ad38d0(e, locale);
                fn_00acb720(e, local);
                facet
            });
            let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
            let mut character = fn_00aca9f0(e, Ptr::new(buffer));
            loop {
                if character == 0xffff_ffff {
                    let ios = stream_base(e, stream);
                    // `MOV EDX,1; TEST EDX,EDX; JZ`: the state is the constant 1.
                    let state = std::hint::black_box(1u32);
                    if state != 0 {
                        let mut new_state = e.mem.u32(ios + 8) | state;
                        if e.mem.u32(ios + 0x28) == 0 {
                            new_state |= 4;
                        }
                        ios_base_clear(e, Ptr::new(ios), new_state, 0);
                    }
                    break;
                }
                let table = e.mem.u32(facet.addr() + 0x10);
                let class = e.mem.u16(table.wrapping_add((character & 0xff) * 2));
                if class & 0x48 == 0 {
                    break;
                }
                let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
                character = fn_00ace7d0(e, Ptr::new(buffer));
            }
        }
        if e.mem.u32(stream_base(e, stream) + 8) == 0 {
            return 1;
        }
    }
    let ios = stream_base(e, stream);
    // `MOV EAX,2; TEST EAX,EAX; JZ`: the state is the constant 2.
    let state = std::hint::black_box(2u32);
    if state != 0 {
        let mut new_state = e.mem.u32(ios + 8) | state;
        if e.mem.u32(ios + 0x28) == 0 {
            new_state |= 4;
        }
        ios_base_clear(e, Ptr::new(ios), new_state, 0);
    }
    0
}

// Translated from 00ace7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf::snextc()` shape (thiscall). With more than one character
/// left (signed) it moves the read pointer on ([`fn_00ace9c0`]) and returns the byte there;
/// otherwise it takes one ([`fn_00ace860`]) and returns -1 at an end of file, else the next
/// one ([`fn_00aca9f0`]).
pub fn fn_00ace7d0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let available = if buffer_at(e, base, 0x20) == 0 {
        0
    } else {
        buffer_at(e, base, 0x30) as i32
    };
    if available > 1 {
        let next = fn_00ace9c0(e, this);
        return u32::from(e.mem.u8(next));
    }
    if fn_00ace860(e, this) == 0xffff_ffff {
        0xffff_ffff
    } else {
        fn_00aca9f0(e, this)
    }
}

// Translated from 00ace860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf::sbumpc()` shape (thiscall): with characters left (signed)
/// the byte at the read pointer, which moves on ([`fn_00ace970`]); otherwise the buffer's
/// slot `0x14` (`uflow`).
pub fn fn_00ace860(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let available = if buffer_at(e, base, 0x20) == 0 {
        0
    } else {
        buffer_at(e, base, 0x30) as i32
    };
    if available > 0 {
        let next = fn_00ace970(e, this);
        u32::from(e.mem.u8(next))
    } else {
        e.vcall(base, 0x14, &args![]).u32()
    }
}

// Translated from 00ace8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf::sputc(ch)` shape (thiscall, one word, `RET 4`): with room in
/// the put area (signed) it stores the character byte at the write pointer, which moves on
/// ([`basic_streambuf_pninc`]), and returns the stored byte; otherwise the buffer's slot 4
/// (`overflow`) with the byte zero-extended.
pub fn fn_00ace8c0(e: &mut Engine, this: Ptr, character: u32) -> u32 {
    let base = this.addr();
    let room = if buffer_at(e, base, 0x24) == 0 {
        0
    } else {
        buffer_at(e, base, 0x34) as i32
    };
    if room > 0 {
        let slot = basic_streambuf_pninc(e, this);
        e.mem.set_u8(slot, character as u8);
        u32::from(e.mem.u8(slot))
    } else {
        e.vcall(base, 4, &args![character & 0xff]).u32()
    }
}

// Translated from 00ace930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::basic_streambuf<char>::_Gndec` (Ghidra library match; thiscall): one more character
/// in the get area (`[[+0x30]]`), the read pointer (`[[+0x20]]`) one back; returns it.
pub fn basic_streambuf_gndec(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    buffer_add(e, base, 0x30, 1);
    buffer_add(e, base, 0x20, 0xffff_ffff);
    buffer_at(e, base, 0x20)
}

// Translated from 00ace970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `basic_streambuf<char>::_Gninc` shape (one of Ghidra's library matches; thiscall):
/// one character less in the get area, the read pointer one on; returns its old value.
pub fn fn_00ace970(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    buffer_add(e, base, 0x30, 0xffff_ffff);
    let old = buffer_at(e, base, 0x20);
    let target = e.mem.u32(base + 0x20);
    e.mem.set_u32(target, old.wrapping_add(1));
    old
}

// Translated from 00ace9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf::_Gnpreinc` shape (thiscall): one character less in the get
/// area, the read pointer one on; returns its new value.
pub fn fn_00ace9c0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    buffer_add(e, base, 0x30, 0xffff_ffff);
    buffer_add(e, base, 0x20, 1);
    buffer_at(e, base, 0x20)
}

// Translated from 00acea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::basic_streambuf<char>::_Pninc` (Ghidra library match; thiscall): one place less in
/// the put area (`[[+0x34]]`), the write pointer (`[[+0x24]]`) one on; returns its old value.
pub fn basic_streambuf_pninc(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    buffer_add(e, base, 0x34, 0xffff_ffff);
    let old = buffer_at(e, base, 0x24);
    let target = e.mem.u32(base + 0x24);
    e.mem.set_u32(target, old.wrapping_add(1));
    old
}

/// `00ac0460` (thiscall, one word: the other iterator): a checked vector iterator's
/// `operator!=`, `AL`. `00ac0b00` (no arguments): the vector's `_Xlen`.
const ITERATOR_NOT_EQUAL_00AC0460: u32 = 0x00ac_0460;
const VECTOR_TOO_LONG_00AC0B00: u32 = 0x00ac_0b00;
/// `00ad5a80` (cdecl, six words: source first, source last, destination and three tag words
/// it does not read): copies a range of strings down; returns the destination's end.
const COPY_STRINGS_00AD5A80: u32 = 0x00ad_5a80;

/// The insert shape shared by [`fn_00acea50`] and [`fn_00aceeb0`]: the element index of the
/// position (0 for an empty vector; otherwise a checked start iterator, the position's proxy
/// checked against it, and the signed distance divided by `element`), the insertion of one
/// copy of the value (`insert_n`), then a checked start iterator moved on by that index
/// (`advance`) written to `out`.
#[allow(clippy::too_many_arguments)]
fn vector_insert_one(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    proxy: u32,
    at: u32,
    value: u32,
    element: i32,
    insert_n: fn(&mut Engine, Ptr, u32, u32, u32, u32),
    advance: fn(&mut Engine, Ptr, u32) -> Ptr,
) -> Ptr {
    let base = this.addr();
    let used = e.mem.u32(base + 0x10).wrapping_sub(e.mem.u32(base + 0xc)) as i32 / element;
    let index = if used == 0 {
        0
    } else {
        let first = e.mem.u32(base + 0xc);
        let begin = vector_iterator(e, base, first);
        if proxy == 0 || proxy != begin.0 {
            e.call(INVALID_PARAMETER, &args![]);
        }
        (at.wrapping_sub(begin.1) as i32 / element) as u32
    };
    insert_n(e, this, proxy, at, 1, value);
    let first = e.mem.u32(base + 0xc);
    let begin = vector_iterator(e, base, first);
    e.with_stack(8, |e, iterator| {
        e.mem.set_u32(iterator.addr(), begin.0);
        e.mem.set_u32(iterator.addr() + 4, begin.1);
        advance(e, iterator, index);
        let first = e.mem.u32(iterator.addr());
        let second = e.mem.u32(iterator.addr() + 4);
        e.mem.set_u32(out.addr(), first);
        e.mem.set_u32(out.addr() + 4, second);
    });
    out
}

// Translated from 00acea50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<string>::insert(position, value)` (thiscall, four words: the iterator it
/// fills, the position's two words and the value, `RET 0x10`; elements of 0x1c bytes). Takes
/// the position's index (see the helper), inserts one copy ([`fn_00ad07c0`]) and returns a
/// checked iterator at that index ([`fn_00ad35e0`]).
pub fn fn_00acea50(e: &mut Engine, this: Ptr, out: Ptr, proxy: u32, at: u32, value: u32) -> Ptr {
    vector_insert_one(
        e,
        this,
        out,
        proxy,
        at,
        value,
        0x1c,
        fn_00ad07c0,
        fn_00ad35e0,
    )
}

// Translated from 00aceb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<string>::erase(first, last)` (thiscall, five words: the iterator it fills
/// and the two iterators' words, `RET 0x14`). Checks both iterators against the vector; when
/// they differ (`00ac0460`) copies the tail down over the range (`00ad5a80`; its three tag
/// words are a zero and two bytes the exe leaves uninitialised, which the callee does not
/// read), destroys the strings left at the end (`0044a730(1, 0)`; a compiled-out free behind
/// `XOR EAX,EAX; AND EAX,1` is kept) and moves the end. Returns the first iterator.
#[allow(clippy::too_many_arguments)]
pub fn fn_00aceb80(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    let base = this.addr();
    let start = vector_iterator(e, base, first);
    let end = vector_iterator(e, base, last);
    e.with_stack(0x10, |e, frame| {
        let start_at = frame.addr();
        let end_at = frame.addr() + 8;
        e.mem.set_u32(start_at, start.0);
        e.mem.set_u32(start_at + 4, start.1);
        e.mem.set_u32(end_at, end.0);
        e.mem.set_u32(end_at + 4, end.1);
        if e.call(ITERATOR_NOT_EQUAL_00AC0460, &args![start_at, end_at])
            .bool()
        {
            let old_last = e.mem.u32(base + 0x10);
            let new_last = e
                .call(
                    COPY_STRINGS_00AD5A80,
                    &args![end.1, old_last, start.1, 0u32, 0u32, 0u32],
                )
                .u32();
            let old_last = e.mem.u32(base + 0x10);
            let mut element = new_last;
            while element != old_last {
                e.call(STRING_TIDY_0044A730, &args![element, 1u32, 0u32]);
                // `XOR EAX,EAX; AND EAX,1; JZ`: this free is compiled out (the constant 0).
                if std::hint::black_box(0u32) & 1 != 0 {
                    e.call(OPERATOR_DELETE, &args![element]);
                }
                element = element.wrapping_add(0x1c);
            }
            e.mem.set_u32(base + 0x10, new_last);
        }
    });
    e.mem.set_u32(out.addr(), start.0);
    e.mem.set_u32(out.addr() + 4, start.1);
    out
}

// Translated from 00aced20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<string>::_Buy(count)` (thiscall, one word, `RET 4`; returns `AL`). Clears
/// the three pointers; 0 for no elements. A count above the maximum 0x9249249 (the exe's
/// "maximum or 1" test on that constant is kept) calls `_Xlen` (`00ac0b00`) and returns 1
/// without allocating; otherwise it allocates the elements ([`fn_00ad5310`]) and sets first,
/// last and the end of storage (`count * 0x1c` on). Returns 1.
pub fn fn_00aced20(e: &mut Engine, this: Ptr, count: u32) -> u8 {
    let base = this.addr();
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0);
    if count == 0 {
        return 0;
    }
    let maximum = std::hint::black_box(0x0924_9249u32);
    let limit = if maximum > 0 { maximum } else { 1 };
    if limit < count {
        e.call(VECTOR_TOO_LONG_00AC0B00, &args![]);
        return 1;
    }
    let block = fn_00ad5310(e, count, 0);
    e.mem.set_u32(base + 0xc, block);
    e.mem.set_u32(base + 0x10, block);
    e.mem
        .set_u32(base + 0x14, count.wrapping_mul(0x1c).wrapping_add(block));
    1
}

// Translated from 00acede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<string>::_Tidy()` (thiscall): with storage it destroys each string from
/// first to last (`0044a730(1, 0)`; a compiled-out free is kept), computes the capacity (not
/// used) and frees the storage (`operator delete`); then clears the three pointers.
pub fn fn_00acede0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0xc) != 0 {
        let last = e.mem.u32(base + 0x10);
        let mut element = e.mem.u32(base + 0xc);
        while element != last {
            e.call(STRING_TIDY_0044A730, &args![element, 1u32, 0u32]);
            // `XOR EAX,EAX; AND EAX,1; JZ`: this free is compiled out (the constant 0).
            if std::hint::black_box(0u32) & 1 != 0 {
                e.call(OPERATOR_DELETE, &args![element]);
            }
            element = element.wrapping_add(0x1c);
        }
        // The capacity the exe computes here is never used.
        let _capacity = (e.mem.u32(base + 0x14).wrapping_sub(e.mem.u32(base + 0xc))) as i32 / 0x1c;
        let storage = e.mem.u32(base + 0xc);
        e.call(OPERATOR_DELETE, &args![storage]);
    }
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0);
}

// Translated from 00aceeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<char>::insert(position, value)` (thiscall, four words: the iterator it
/// fills, the position's two words and a pointer to the character, `RET 0x10`): as
/// [`fn_00acea50`] with one-byte elements, [`fn_00ad10e0`] and [`fn_00ad36a0`].
pub fn fn_00aceeb0(e: &mut Engine, this: Ptr, out: Ptr, proxy: u32, at: u32, value: u32) -> Ptr {
    vector_insert_one(e, this, out, proxy, at, value, 1, fn_00ad10e0, fn_00ad36a0)
}

// Translated from 00acefd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<char>::erase(first, last)` (thiscall, five words: the iterator it fills
/// and the two iterators' words, `RET 0x14`): checks both iterators; when they differ
/// (`00ac0460`) moves the tail down ([`fn_00ad5110`]) and sets the new end. Returns the first
/// iterator.
#[allow(clippy::too_many_arguments)]
pub fn fn_00acefd0(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    let base = this.addr();
    let start = vector_iterator(e, base, first);
    let end = vector_iterator(e, base, last);
    e.with_stack(0x10, |e, frame| {
        let start_at = frame.addr();
        let end_at = frame.addr() + 8;
        e.mem.set_u32(start_at, start.0);
        e.mem.set_u32(start_at + 4, start.1);
        e.mem.set_u32(end_at, end.0);
        e.mem.set_u32(end_at + 4, end.1);
        if e.call(ITERATOR_NOT_EQUAL_00AC0460, &args![start_at, end_at])
            .bool()
        {
            let old_last = e.mem.u32(base + 0x10);
            let new_last = fn_00ad5110(e, end.1, old_last, start.1);
            e.mem.set_u32(base + 0x10, new_last);
        }
    });
    e.mem.set_u32(out.addr(), start.0);
    e.mem.set_u32(out.addr() + 4, start.1);
    out
}

/// The `length_error` [`fn_00acf120`] throws: its message `"map/set<T> too long"`, the
/// `0044a310` constructor (thiscall, one word: the message string), its vtable and its throw
/// information.
const MAP_TOO_LONG_010A31A0: u32 = 0x010a_31a0;
const LENGTH_ERROR_CTOR_0044A310: u32 = 0x0044_a310;
const VTABLE_LENGTH_ERROR_010173DC: u32 = 0x0101_73dc;
const LENGTH_ERROR_THROW_INFO_01140CB0: u32 = 0x0114_0cb0;

// Translated from 00acf120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acf120 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the string-keyed tree's `_Insert(add_left, parent, value)` (thiscall, four
/// words: the iterator it fills, the "add left" byte, the parent node and the value,
/// `RET 0x10`). At the maximum size (`0x4924924 - 1`; the exe's "maximum or 1" test is kept)
/// it throws a `length_error` ("map/set<T> too long"; the game does not return, the model
/// returns `out` after the throw). Otherwise it makes the node ([`fn_00ad1960`]: head, parent,
/// head, value, colour 0), counts it, links it (as the root and both extremes into an empty
/// tree, else as the parent's left or right child, updating the leftmost or rightmost node),
/// then restores the red-black rules (colour byte `+0x44`: 0 red, 1 black) with recolouring
/// and the rotations [`fn_00ad1800`] (left) and [`fn_00ad18b0`] (right), and blackens the
/// root. Returns a checked iterator at the new node.
pub fn fn_00acf120(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    add_left: u32,
    parent: u32,
    value: u32,
) -> Ptr {
    let base = this.addr();
    let maximum = std::hint::black_box(0x0492_4924u32);
    let limit = if maximum > 0 { maximum } else { 1 };
    if limit.wrapping_sub(1) <= e.mem.u32(base + 0x1c) {
        e.with_stack(0x58, |e, frame| {
            // Frame: the message string at +0 (0x1c bytes), the exception object at +0x1c.
            let string = frame.addr();
            let object = frame.addr() + 0x1c;
            e.call(
                STRING_FROM_C_0044A4C0,
                &args![string, MAP_TOO_LONG_010A31A0],
            );
            e.call(LENGTH_ERROR_CTOR_0044A310, &args![object, string]);
            e.mem.set_u32(object, VTABLE_LENGTH_ERROR_010173DC);
            e.call(
                CXX_THROW_EXCEPTION,
                &args![object, LENGTH_ERROR_THROW_INFO_01140CB0],
            );
        });
        return out;
    }
    let head = e.mem.u32(base + 0x18);
    let node = fn_00ad1960(e, this, head, parent, head, value, 0);
    let size = e.mem.u32(base + 0x1c).wrapping_add(1);
    e.mem.set_u32(base + 0x1c, size);
    let head = e.mem.u32(base + 0x18);
    if parent == head {
        e.mem.set_u32(head + 4, node);
        e.mem.set_u32(head, node);
        e.mem.set_u32(head + 8, node);
    } else if add_left as u8 != 0 {
        e.mem.set_u32(parent, node);
        if parent == e.mem.u32(head) {
            e.mem.set_u32(head, node);
        }
    } else {
        e.mem.set_u32(parent + 8, node);
        if parent == e.mem.u32(head + 8) {
            e.mem.set_u32(head + 8, node);
        }
    }
    let colour = |e: &Engine, node: u32| e.mem.u8(node + 0x44);
    let mut current = node;
    while colour(e, e.mem.u32(current + 4)) == 0 {
        let up = e.mem.u32(current + 4);
        let grand = e.mem.u32(up + 4);
        if up == e.mem.u32(grand) {
            let uncle = e.mem.u32(e.mem.u32(e.mem.u32(current + 4) + 4) + 8);
            if colour(e, uncle) == 0 {
                e.mem.set_u8(e.mem.u32(current + 4) + 0x44, 1);
                e.mem.set_u8(uncle + 0x44, 1);
                e.mem
                    .set_u8(e.mem.u32(e.mem.u32(current + 4) + 4) + 0x44, 0);
                current = e.mem.u32(e.mem.u32(current + 4) + 4);
            } else {
                if current == e.mem.u32(e.mem.u32(current + 4) + 8) {
                    current = e.mem.u32(current + 4);
                    fn_00ad1800(e, this, current);
                }
                e.mem.set_u8(e.mem.u32(current + 4) + 0x44, 1);
                e.mem
                    .set_u8(e.mem.u32(e.mem.u32(current + 4) + 4) + 0x44, 0);
                let pivot = e.mem.u32(e.mem.u32(current + 4) + 4);
                fn_00ad18b0(e, this, pivot);
            }
        } else {
            let uncle = e.mem.u32(e.mem.u32(e.mem.u32(current + 4) + 4));
            if colour(e, uncle) == 0 {
                e.mem.set_u8(e.mem.u32(current + 4) + 0x44, 1);
                e.mem.set_u8(uncle + 0x44, 1);
                e.mem
                    .set_u8(e.mem.u32(e.mem.u32(current + 4) + 4) + 0x44, 0);
                current = e.mem.u32(e.mem.u32(current + 4) + 4);
            } else {
                if current == e.mem.u32(e.mem.u32(current + 4)) {
                    current = e.mem.u32(current + 4);
                    fn_00ad18b0(e, this, current);
                }
                e.mem.set_u8(e.mem.u32(current + 4) + 0x44, 1);
                e.mem
                    .set_u8(e.mem.u32(e.mem.u32(current + 4) + 4) + 0x44, 0);
                let pivot = e.mem.u32(e.mem.u32(current + 4) + 4);
                fn_00ad1800(e, this, pivot);
            }
        }
    }
    let root = e.mem.u32(e.mem.u32(base + 0x18) + 4);
    e.mem.set_u8(root + 0x44, 1);
    e.mem.set_u32(out.addr(), 0);
    e.mem.set_u32(out.addr() + 4, node);
    if base == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(base);
    e.mem.set_u32(out.addr(), proxy);
    out
}

// Translated from 00acf500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acf500 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: a pair of strings' default constructor (thiscall): initialises the strings at
/// `+0` and `+0x1c` (`0044a730(0, 0)`). Returns `this`.
pub fn fn_00acf500(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(STRING_TIDY_0044A730, &args![base, 0u32, 0u32]);
    e.call(STRING_TIDY_0044A730, &args![base + 0x1c, 0u32, 0u32]);
    this
}

// Translated from 00acf580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acf580 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `resize(count, value)` of the vector of string pairs (thiscall, fifteen words: the
/// count and the 0x38-byte pair by value, `RET 0x3c`; registered through
/// [`fn_00acf580_entry`], which puts the pair's words in a stack block as the exe's argument
/// area). Below the count it inserts copies at a checked end iterator ([`fn_00ad1c10`] with
/// the address of the pair); above it it erases from a checked start iterator moved on by
/// the count ([`fn_00ad3750`]) to a checked end iterator ([`fn_00ad1a90`]; the end is checked
/// first). Then it destroys the pair's two strings (`0044a730(1, 0)`, the second first).
pub fn fn_00acf580(e: &mut Engine, this: Ptr, count: u32, value: u32) {
    let base = this.addr();
    let size = ((e.mem.u32(base + 0x10).wrapping_sub(e.mem.u32(base + 0xc))) as i32 / 0x38) as u32;
    if size < count {
        let last = e.mem.u32(base + 0x10);
        let at = vector_iterator(e, base, last);
        let size =
            ((e.mem.u32(base + 0x10).wrapping_sub(e.mem.u32(base + 0xc))) as i32 / 0x38) as u32;
        fn_00ad1c10(e, this, at.0, at.1, count.wrapping_sub(size), value);
    } else {
        let size =
            ((e.mem.u32(base + 0x10).wrapping_sub(e.mem.u32(base + 0xc))) as i32 / 0x38) as u32;
        if count < size {
            let last = e.mem.u32(base + 0x10);
            let end = vector_iterator(e, base, last);
            let first = e.mem.u32(base + 0xc);
            let begin = vector_iterator(e, base, first);
            e.with_stack(0x10, |e, frame| {
                let iterator = frame.addr();
                let out = frame.addr() + 8;
                e.mem.set_u32(iterator, begin.0);
                e.mem.set_u32(iterator + 4, begin.1);
                fn_00ad3750(e, Ptr::new(iterator), count);
                let from = (e.mem.u32(iterator), e.mem.u32(iterator + 4));
                fn_00ad1a90(e, this, Ptr::new(out), from.0, from.1, end.0, end.1);
            });
        }
    }
    e.call(STRING_TIDY_0044A730, &args![value + 0x1c, 1u32, 0u32]);
    e.call(STRING_TIDY_0044A730, &args![value, 1u32, 0u32]);
}

/// The uniform-form entry of [`fn_00acf580`]: `this`, the count and the pair's fourteen
/// words, copied into a stack block (the exe's argument area) whose address the translation
/// uses.
pub fn fn_00acf580_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let this = Ptr::new(words[0]);
    let count = words[1];
    let pair = &words[2..16];
    e.with_stack(0x38, |e, area| {
        for (i, word) in pair.iter().enumerate() {
            e.mem.set_u32(area.addr() + 4 * i as u32, *word);
        }
        fn_00acf580(e, this, count, area.addr());
    });
    Ret::default()
}

// Translated from 00acf7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `insert(position, value)` of the vector of string pairs (thiscall, four words:
/// the iterator it fills, the position's two words and the pair, `RET 0x10`): as
/// [`fn_00acea50`] with 0x38-byte elements, [`fn_00ad1c10`] and [`fn_00ad3750`].
pub fn fn_00acf7d0(e: &mut Engine, this: Ptr, out: Ptr, proxy: u32, at: u32, value: u32) -> Ptr {
    vector_insert_one(
        e,
        this,
        out,
        proxy,
        at,
        value,
        0x38,
        fn_00ad1c10,
        fn_00ad3750,
    )
}

/// `std::_Fiopen` (`00ec33da`, cdecl: name, mode, protection) and the CRT's `fclose`
/// (`00ec9907`, cdecl: file).
const FIOPEN: u32 = 0x00ec_33da;
const CRT_FCLOSE: u32 = 0x00ec_9907;

// Translated from 00acfab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acfab0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_filebuf<char>::open(name, mode, protection)` shape (thiscall, three
/// words, `RET 0xc`). Returns 0 when a file is already open or `_Fiopen` fails; otherwise
/// initialises the buffer on the file ([`fn_00acfc10`] with mode 1), takes the locale
/// ([`fn_00ad06e0`] into a stack local), keeps its converter facet ([`fn_00ad4fd0`]) unless
/// the facet's slot 4 (`always_noconv`, `AL`) says no conversion is needed (then `+0x3c` is
/// 0), re-initialising the pointers ([`fn_00ad0720`]) when it keeps it, releases the local
/// ([`fn_00acb720`]) and returns `this`.
pub fn fn_00acfab0(e: &mut Engine, this: Ptr, name: u32, mode: u32, protection: u32) -> Ptr {
    let base = this.addr();
    if e.mem.u32(base + 0x4c) != 0 {
        return Ptr::new(0);
    }
    let file = e.call(FIOPEN, &args![name, mode, protection]).u32();
    if file == 0 {
        return Ptr::new(0);
    }
    fn_00acfc10(e, this, file, 1);
    e.with_stack(4, |e, local| {
        let locale = fn_00ad06e0(e, this, local);
        let converter = fn_00ad4fd0(e, locale).addr();
        if e.vcall(converter, 4, &args![]).bool() {
            e.mem.set_u32(base + 0x3c, 0);
        } else {
            e.mem.set_u32(base + 0x3c, converter);
            fn_00ad0720(e, this);
        }
        fn_00acb720(e, local);
    });
    this
}

// Translated from 00acfb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_filebuf<char>::close()` shape (thiscall). Without a file the result is
/// 0; otherwise it finishes writing ([`fn_00acfcd0`]) and closes the file (`fclose`), the
/// result being `this` unless either failed. Then it resets the buffer without a file
/// ([`fn_00acfc10`] with mode 2). Returns the result.
pub fn fn_00acfb90(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let mut result = base;
    if e.mem.u32(base + 0x4c) == 0 {
        result = 0;
    } else {
        if fn_00acfcd0(e, this) == 0 {
            result = 0;
        }
        let file = e.mem.u32(base + 0x4c);
        if e.call(CRT_FCLOSE, &args![file]).u32() != 0 {
            result = 0;
        }
    }
    fn_00acfc10(e, this, 0, 2);
    Ptr::new(result)
}

// Translated from 00acfcd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00acfcd0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and the /GS cookie check (00ec408c) are not translated
/// Unnamed: the `basic_filebuf<char>::_Endwrite()` shape (thiscall; returns `AL`). Without a
/// converter or without pending output (byte `+0x41`) it returns 1. It flushes the put area
/// through slot 4 (`overflow(-1)`; -1 returns 0), then lets the converter's slot `0x18`
/// (`unshift`) write its closing sequence into an 8-character string ([`fn_00ad0360`],
/// pointers through [`fn_00ad3550`]/[`fn_00ad2f30`]): 0 clears the pending byte and goes on
/// as 1; with 1 it writes what was produced (`fwrite`, 0 when short), returns 1 once nothing
/// is pending, and grows the string by 8 (`00aadeb0`) when nothing was produced before trying
/// again; 3 returns 1; anything else 0. The string is destroyed on every way out.
pub fn fn_00acfcd0(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    if e.mem.u32(base + 0x3c) == 0 || e.mem.u8(base + 0x41) == 0 {
        return 1;
    }
    if e.vcall(base, 4, &args![0xffff_ffffu32]).u32() == 0xffff_ffff {
        return 0;
    }
    e.with_stack(0x50, |e, frame| {
        // Frame (the exe's `EBP - 0x60` at +0): checked iterators at +0, +8, +0x10 and +0x18,
        // the string at +0x2c, the converter's "to next" at +0x4c.
        let frame = frame.addr();
        let string = frame + 0x2c;
        let to_next = frame + 0x4c;
        e.call(STRING_TIDY_0044A730, &args![string, 0u32, 0u32]);
        fn_00ad0360(e, Ptr::new(string), 8, 0);
        let result = loop {
            let iterator = Ptr::new(frame + 0x18);
            let data = string_chars(e, string);
            fn_00ad3550(e, iterator, data, string);
            let first = fn_00ad2f30(e, iterator);
            let size = e.mem.u32(string + 0x14);
            let iterator = Ptr::new(frame + 0x10);
            let data = string_chars(e, string);
            fn_00ad3550(e, iterator, data, string);
            let to = fn_00ad2f30(e, iterator);
            let converter = e.mem.u32(base + 0x3c);
            let converted = e
                .vcall(
                    converter,
                    0x18,
                    &args![base + 0x44, to, first.wrapping_add(size), to_next],
                )
                .u32();
            match converted {
                0 | 1 => {
                    if converted == 0 {
                        e.mem.set_u8(base + 0x41, 0);
                    }
                    let iterator = Ptr::new(frame + 8);
                    let data = string_chars(e, string);
                    fn_00ad3550(e, iterator, data, string);
                    let start = fn_00ad2f30(e, iterator);
                    let produced = e.mem.u32(to_next).wrapping_sub(start);
                    if produced != 0 {
                        let iterator = Ptr::new(frame);
                        let data = string_chars(e, string);
                        fn_00ad3550(e, iterator, data, string);
                        let start = fn_00ad2f30(e, iterator);
                        let file = e.mem.u32(base + 0x4c);
                        let written = e
                            .call(CRT_FWRITE, &args![start, 1u32, produced, file])
                            .u32();
                        if produced != written {
                            break 0;
                        }
                    }
                    if e.mem.u8(base + 0x41) == 0 {
                        break 1;
                    }
                    if produced == 0 {
                        e.call(STRING_APPEND_FILL_00AADEB0, &args![string, 8u32, 0u32]);
                    }
                }
                3 => break 1,
                _ => break 0,
            }
        };
        e.call(STRING_TIDY_0044A730, &args![string, 1u32, 0u32]);
        result
    })
}

// Translated from 00acffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `insert(position, value)` of the vector of 0x18-byte elements (thiscall, four
/// words: the iterator it fills, the position's two words and the value, `RET 0x10`): as
/// [`fn_00acea50`] with 0x18-byte elements, [`fn_00ad26f0`] and [`fn_00ad3810`].
pub fn fn_00acffe0(e: &mut Engine, this: Ptr, out: Ptr, proxy: u32, at: u32, value: u32) -> Ptr {
    vector_insert_one(
        e,
        this,
        out,
        proxy,
        at,
        value,
        0x18,
        fn_00ad26f0,
        fn_00ad3810,
    )
}

// Translated from 00ad0110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `erase(first, last)` of the vector of 0x18-byte elements (thiscall, five words:
/// the iterator it fills and the two iterators' words, `RET 0x14`): checks both iterators;
/// when they differ (`00ac0460`) copies the tail down ([`fn_00ad5180`]), destroys what is
/// left at the end ([`fn_00ad2640`]) and sets the new end. Returns the first iterator.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ad0110(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    let base = this.addr();
    let start = vector_iterator(e, base, first);
    let end = vector_iterator(e, base, last);
    e.with_stack(0x10, |e, frame| {
        let start_at = frame.addr();
        let end_at = frame.addr() + 8;
        e.mem.set_u32(start_at, start.0);
        e.mem.set_u32(start_at + 4, start.1);
        e.mem.set_u32(end_at, end.0);
        e.mem.set_u32(end_at + 4, end.1);
        if e.call(ITERATOR_NOT_EQUAL_00AC0460, &args![start_at, end_at])
            .bool()
        {
            let old_last = e.mem.u32(base + 0x10);
            let new_last = fn_00ad5180(e, end.1, old_last, start.1);
            let old_last = e.mem.u32(base + 0x10);
            fn_00ad2640(e, this, new_last, old_last);
            e.mem.set_u32(base + 0x10, new_last);
        }
    });
    e.mem.set_u32(out.addr(), start.0);
    e.mem.set_u32(out.addr() + 4, start.1);
    out
}

// Translated from 00ad0210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad0210 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_ostream<char>::sentry` constructor (thiscall, one word: the stream,
/// `RET 4`): the shared sentry part ([`fn_00ad2fa0`]); with a good state it flushes the tied
/// stream (`base + 0x2c`, [`fn_00aca3c0`]); the byte at `+4` says whether the state is good.
/// Returns `this`.
pub fn fn_00ad0210(e: &mut Engine, this: Ptr, stream: Ptr) -> Ptr {
    fn_00ad2fa0(e, this, stream);
    let stream = stream.addr();
    if e.mem.u32(stream_base(e, stream) + 8) == 0 {
        let tie = e.mem.u32(stream_base(e, stream) + 0x2c);
        if tie != 0 {
            fn_00aca3c0(e, Ptr::new(tie));
        }
    }
    let good = e.mem.u32(stream_base(e, stream) + 8) == 0;
    e.mem.set_u8(this.addr() + 4, good as u8);
    this
}

// Translated from 00ad02f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad02f0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_ostream<char>::sentry` destructor (thiscall): unless
/// `uncaught_exception()` (`00ec3241`, `AL`) it lets the stream flush when asked
/// ([`fn_00ad03e0`]); then the shared sentry part is destroyed ([`fn_00aca9a0`]).
pub fn fn_00ad02f0(e: &mut Engine, this: Ptr) {
    if !e.call(UNCAUGHT_EXCEPTION, &args![]).bool() {
        let stream = e.mem.u32(this.addr());
        fn_00ad03e0(e, Ptr::new(stream));
    }
    fn_00aca9a0(e, this);
}

// Translated from 00ad03e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad03e0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the `basic_ostream<char>::_Osfx()` shape (thiscall): with `unitbuf` (bit 1 of
/// `base + 0x10`) it flushes the stream ([`fn_00aca3c0`]).
pub fn fn_00ad03e0(e: &mut Engine, this: Ptr) {
    if e.mem.u32(stream_base(e, this.addr()) + 0x10) & 2 != 0 {
        fn_00aca3c0(e, this);
    }
}

// Translated from 00ad06e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_streambuf::getloc()` shape (thiscall, one word: the locale it fills,
/// `RET 4`): copies the word of the buffer's locale (`[+0x38]`) into `[out]`, adds a reference
/// to the object it points at ([`facet_incref`]) and returns `out`.
pub fn fn_00ad06e0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let holder = e.mem.u32(this.addr() + 0x38);
    let facet = e.mem.u32(holder);
    e.mem.set_u32(out.addr(), facet);
    let facet = e.mem.u32(out.addr());
    facet_incref(e, Ptr::new(facet));
    out
}

/// `00ad66e0` (cdecl, six words: source first, source last, destination, allocator and two tag
/// words it does not read): copy-constructs a range of strings; returns the destination's
/// end.
const COPY_CONSTRUCT_STRINGS_00AD66E0: u32 = 0x00ad_66e0;

/// The element count of a vector's `[+0xc, end_slot)` range, by signed division as the exe
/// computes it.
fn vector_count(e: &Engine, base: u32, end_slot: u32, element: i32) -> u32 {
    (e.mem
        .u32(base + end_slot)
        .wrapping_sub(e.mem.u32(base + 0xc)) as i32
        / element) as u32
}

// Translated from 00ad07c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad07c0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and the /GS cookie check (00ec408c) are not translated
/// Unnamed: `vector<string>::_Insert_n(position, count, value)` (thiscall, four words: the
/// position's proxy (not read) and pointer, the count and the value string, `RET 0x10`;
/// elements of 0x1c bytes). Nothing for a count of 0; past the maximum (0x9249249; the exe's
/// "maximum or 1" tests are kept) `_Xlen` (`00ac0b00`). Without room it allocates half again
/// the capacity (0 when that would pass the maximum) or the needed size, whichever is larger
/// ([`fn_00ad5310`]), copies the front (`00ad66e0`), fills the copies ([`fn_00ad5b00`]),
/// copies the back ([`fn_00ad5dd0`]), destroys and frees the old elements and points the
/// vector at the new storage. With room it works on a copy of the value (`0044a730(0, 0)`,
/// `0044a580`): when the insertion passes the end it copies the tail out (`00ad66e0`), fills
/// the rest past the old end ([`fn_00ad5b00`]) and assigns the copy over the moved part;
/// otherwise it copies the last `count` elements out to the end (`00ad66e0`), moves the
/// middle back ([`fn_00ad5250`]) and assigns the copy over the gap. The copy is destroyed.
/// The tag words of the range helpers are a zero and a byte the exe leaves uninitialised,
/// which none of them reads.
pub fn fn_00ad07c0(e: &mut Engine, this: Ptr, _proxy: u32, at: u32, count: u32, value: u32) {
    let base = this.addr();
    let allocator = base + 8;
    let capacity = if e.mem.u32(base + 0xc) == 0 {
        0
    } else {
        vector_count(e, base, 0x14, 0x1c)
    };
    if count == 0 {
        return;
    }
    let maximum = std::hint::black_box(0x0924_9249u32);
    let limit = if maximum > 0 { maximum } else { 1 };
    if limit.wrapping_sub(vector_count(e, base, 0x10, 0x1c)) < count {
        e.call(VECTOR_TOO_LONG_00AC0B00, &args![]);
        return;
    }
    if capacity < vector_count(e, base, 0x10, 0x1c).wrapping_add(count) {
        let maximum = std::hint::black_box(0x0924_9249u32);
        let limit = if maximum > 0 { maximum } else { 1 };
        let mut new_capacity = if limit.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        if new_capacity < vector_count(e, base, 0x10, 0x1c).wrapping_add(count) {
            new_capacity = vector_count(e, base, 0x10, 0x1c).wrapping_add(count);
        }
        let block = fn_00ad5310(e, new_capacity, 0);
        let first = e.mem.u32(base + 0xc);
        let mut next = e
            .call(
                COPY_CONSTRUCT_STRINGS_00AD66E0,
                &args![first, at, block, allocator, 0u32, 0u32],
            )
            .u32();
        fn_00ad5b00(e, next, count, value, allocator, 0, 0);
        next = count.wrapping_mul(0x1c).wrapping_add(next);
        let last = e.mem.u32(base + 0x10);
        fn_00ad5dd0(e, at, last, next, allocator);
        let new_size = vector_count(e, base, 0x10, 0x1c).wrapping_add(count);
        if e.mem.u32(base + 0xc) != 0 {
            let last = e.mem.u32(base + 0x10);
            let mut element = e.mem.u32(base + 0xc);
            while element != last {
                e.call(STRING_TIDY_0044A730, &args![element, 1u32, 0u32]);
                // `XOR EDX,EDX; AND EDX,1; JZ`: this free is compiled out (the constant 0).
                if std::hint::black_box(0u32) & 1 != 0 {
                    e.call(OPERATOR_DELETE, &args![element]);
                }
                element = element.wrapping_add(0x1c);
            }
            // The capacity the exe computes here is never used.
            let _old_capacity = vector_count(e, base, 0x14, 0x1c);
            let storage = e.mem.u32(base + 0xc);
            e.call(OPERATOR_DELETE, &args![storage]);
        }
        e.mem.set_u32(
            base + 0x14,
            new_capacity.wrapping_mul(0x1c).wrapping_add(block),
        );
        e.mem
            .set_u32(base + 0x10, new_size.wrapping_mul(0x1c).wrapping_add(block));
        e.mem.set_u32(base + 0xc, block);
        return;
    }
    e.with_stack(0x1c, |e, copy| {
        let copy = copy.addr();
        e.call(STRING_TIDY_0044A730, &args![copy, 0u32, 0u32]);
        let npos = e.global::<u32>(STRING_NPOS_010173F0);
        e.call(STRING_ASSIGN_0044A580, &args![copy, value, 0u32, npos]);
        let after = (e.mem.u32(base + 0x10).wrapping_sub(at) as i32 / 0x1c) as u32;
        if after < count {
            let last = e.mem.u32(base + 0x10);
            e.call(
                COPY_CONSTRUCT_STRINGS_00AD66E0,
                &args![
                    at,
                    last,
                    count.wrapping_mul(0x1c).wrapping_add(at),
                    allocator,
                    0u32,
                    0u32
                ],
            );
            let last = e.mem.u32(base + 0x10);
            let after = (last.wrapping_sub(at) as i32 / 0x1c) as u32;
            fn_00ad5b00(e, last, count.wrapping_sub(after), copy, allocator, 0, 0);
            let last = count
                .wrapping_mul(0x1c)
                .wrapping_add(e.mem.u32(base + 0x10));
            e.mem.set_u32(base + 0x10, last);
            let stop = e
                .mem
                .u32(base + 0x10)
                .wrapping_sub(count.wrapping_mul(0x1c));
            let mut element = at;
            while element != stop {
                let npos = e.global::<u32>(STRING_NPOS_010173F0);
                e.call(STRING_ASSIGN_0044A580, &args![element, copy, 0u32, npos]);
                element = element.wrapping_add(0x1c);
            }
        } else {
            let old_last = e.mem.u32(base + 0x10);
            let last = e.mem.u32(base + 0x10);
            let moved = old_last.wrapping_sub(count.wrapping_mul(0x1c));
            let new_last = e
                .call(
                    COPY_CONSTRUCT_STRINGS_00AD66E0,
                    &args![moved, old_last, last, allocator, 0u32, 0u32],
                )
                .u32();
            e.mem.set_u32(base + 0x10, new_last);
            fn_00ad5250(
                e,
                at,
                old_last.wrapping_sub(count.wrapping_mul(0x1c)),
                old_last,
            );
            let stop = count.wrapping_mul(0x1c).wrapping_add(at);
            let mut element = at;
            while element != stop {
                let npos = e.global::<u32>(STRING_NPOS_010173F0);
                e.call(STRING_ASSIGN_0044A580, &args![element, copy, 0u32, npos]);
                element = element.wrapping_add(0x1c);
            }
        }
        e.call(STRING_TIDY_0044A730, &args![copy, 1u32, 0u32]);
    });
}

/// `00ac3300` (cdecl: destination, count, a pointer to the byte): fills bytes.
/// `00ad5e20` (cdecl: first, last, destination, allocator): copies a byte range into
/// uninitialised storage, returning the destination's end. `00ab4490` (cdecl: first, last,
/// destination end): copies a byte range backwards.
const FILL_BYTES_00AC3300: u32 = 0x00ac_3300;
const COPY_BYTES_00AD5E20: u32 = 0x00ad_5e20;
const COPY_BYTES_BACKWARD_00AB4490: u32 = 0x00ab_4490;

// Translated from 00ad10e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad10e0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `vector<char>::_Insert_n(position, count, value)` (thiscall, four words: the
/// position's proxy (not read) and pointer, the count and a pointer to the character,
/// `RET 0x10`). Nothing for a count of 0; past the maximum (-1; the exe's "maximum or 1"
/// tests are kept) `_Xlen` (`00ac0b00`). Without room it allocates half again the capacity
/// (0 when that would pass the maximum) or the needed size (`0065f7a0`), copies the front
/// ([`fn_00ad6ab0`]), fills the new characters (`00ac3300`), copies the back, frees the old
/// storage and points the vector at the new one. With room it copies the character first:
/// when the insertion passes the end it copies the tail out (`00ad5e20`), fills the rest past
/// the old end ([`fn_00ad1740`]) and sets the moved part; otherwise it copies the last
/// `count` characters out to the end (`00ad5e20`), moves the middle back (`00ab4490`) and
/// sets the gap.
pub fn fn_00ad10e0(e: &mut Engine, this: Ptr, _proxy: u32, at: u32, count: u32, value: u32) {
    let base = this.addr();
    let allocator = base + 8;
    let size = |e: &Engine| e.mem.u32(base + 0x10).wrapping_sub(e.mem.u32(base + 0xc));
    let capacity = if e.mem.u32(base + 0xc) == 0 {
        0
    } else {
        e.mem.u32(base + 0x14).wrapping_sub(e.mem.u32(base + 0xc))
    };
    if count == 0 {
        return;
    }
    let maximum = std::hint::black_box(0xffff_ffffu32);
    let limit = if maximum > 0 { maximum } else { 1 };
    if limit.wrapping_sub(size(e)) < count {
        e.call(VECTOR_TOO_LONG_00AC0B00, &args![]);
        return;
    }
    if capacity < size(e).wrapping_add(count) {
        let maximum = std::hint::black_box(0xffff_ffffu32);
        let limit = if maximum > 0 { maximum } else { 1 };
        let mut new_capacity = if limit.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        if new_capacity < size(e).wrapping_add(count) {
            new_capacity = size(e).wrapping_add(count);
        }
        let block = e
            .call(ALLOCATE_CHARACTERS, &args![allocator, new_capacity])
            .u32();
        let first = e.mem.u32(base + 0xc);
        let mut next = fn_00ad6ab0(e, first, at, block, allocator);
        e.call(FILL_BYTES_00AC3300, &args![next, count, value]);
        next = next.wrapping_add(count);
        let last = e.mem.u32(base + 0x10);
        fn_00ad6ab0(e, at, last, next, allocator);
        let new_size = size(e).wrapping_add(count);
        if e.mem.u32(base + 0xc) != 0 {
            // The capacity the exe computes here is never used.
            let _old_capacity = e.mem.u32(base + 0x14).wrapping_sub(e.mem.u32(base + 0xc));
            let storage = e.mem.u32(base + 0xc);
            e.call(OPERATOR_DELETE, &args![storage]);
        }
        e.mem.set_u32(base + 0x14, block.wrapping_add(new_capacity));
        e.mem.set_u32(base + 0x10, block.wrapping_add(new_size));
        e.mem.set_u32(base + 0xc, block);
        return;
    }
    e.with_stack(4, |e, local| {
        let local = local.addr();
        let character = e.mem.u8(value);
        e.mem.set_u8(local, character);
        if e.mem.u32(base + 0x10).wrapping_sub(at) < count {
            let last = e.mem.u32(base + 0x10);
            e.call(
                COPY_BYTES_00AD5E20,
                &args![at, last, at.wrapping_add(count), allocator],
            );
            let last = e.mem.u32(base + 0x10);
            let rest = count.wrapping_sub(e.mem.u32(base + 0x10).wrapping_sub(at));
            fn_00ad1740(e, this, last, rest, local);
            let last = e.mem.u32(base + 0x10).wrapping_add(count);
            e.mem.set_u32(base + 0x10, last);
            let stop = e.mem.u32(base + 0x10).wrapping_sub(count);
            let mut slot = at;
            while slot != stop {
                let character = e.mem.u8(local);
                e.mem.set_u8(slot, character);
                slot = slot.wrapping_add(1);
            }
        } else {
            let old_last = e.mem.u32(base + 0x10);
            let last = e.mem.u32(base + 0x10);
            let new_last = e
                .call(
                    COPY_BYTES_00AD5E20,
                    &args![old_last.wrapping_sub(count), old_last, last, allocator],
                )
                .u32();
            e.mem.set_u32(base + 0x10, new_last);
            e.call(
                COPY_BYTES_BACKWARD_00AB4490,
                &args![at, old_last.wrapping_sub(count), old_last],
            );
            let stop = at.wrapping_add(count);
            let mut slot = at;
            while slot != stop {
                let character = e.mem.u8(local);
                e.mem.set_u8(slot, character);
                slot = slot.wrapping_add(1);
            }
        }
    });
}

// Translated from 00ad1740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<char>::_Ufill(destination, count, value)` (thiscall on the vector, which
/// it does not read; three words, `RET 0xc`): fills the bytes (`00ac3300`) and returns
/// `destination + count`.
pub fn fn_00ad1740(e: &mut Engine, _this: Ptr, destination: u32, count: u32, value: u32) -> u32 {
    e.call(FILL_BYTES_00AC3300, &args![destination, count, value]);
    destination.wrapping_add(count)
}

// Translated from 00ad1800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the string-keyed tree's `_Lrotate(node)` (thiscall, one word, `RET 4`): the
/// node's right child takes its place (the child's left subtree becomes the node's right,
/// re-parented unless it is the head; the root, or the parent's matching link, points at the
/// child) and the node becomes the child's left child.
pub fn fn_00ad1800(e: &mut Engine, this: Ptr, node: u32) {
    let base = this.addr();
    let child = e.mem.u32(node + 8);
    let inner = e.mem.u32(child);
    e.mem.set_u32(node + 8, inner);
    if !tree_is_nil(e, e.mem.u32(child)) {
        let inner = e.mem.u32(child);
        e.mem.set_u32(inner + 4, node);
    }
    let parent = e.mem.u32(node + 4);
    e.mem.set_u32(child + 4, parent);
    let head = e.mem.u32(base + 0x18);
    if node == e.mem.u32(head + 4) {
        e.mem.set_u32(head + 4, child);
    } else {
        let parent = e.mem.u32(node + 4);
        if node == e.mem.u32(parent) {
            e.mem.set_u32(parent, child);
        } else {
            e.mem.set_u32(parent + 8, child);
        }
    }
    e.mem.set_u32(child, node);
    e.mem.set_u32(node + 4, child);
}

// Translated from 00ad18b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the string-keyed tree's `_Rrotate(node)` (thiscall, one word, `RET 4`): the
/// mirror of [`fn_00ad1800`] with the left child.
pub fn fn_00ad18b0(e: &mut Engine, this: Ptr, node: u32) {
    let base = this.addr();
    let child = e.mem.u32(node);
    let inner = e.mem.u32(child + 8);
    e.mem.set_u32(node, inner);
    if !tree_is_nil(e, e.mem.u32(child + 8)) {
        let inner = e.mem.u32(child + 8);
        e.mem.set_u32(inner + 4, node);
    }
    let parent = e.mem.u32(node + 4);
    e.mem.set_u32(child + 4, parent);
    let head = e.mem.u32(base + 0x18);
    if node == e.mem.u32(head + 4) {
        e.mem.set_u32(head + 4, child);
    } else {
        let parent = e.mem.u32(node + 4);
        if node == e.mem.u32(parent + 8) {
            e.mem.set_u32(parent + 8, child);
        } else {
            e.mem.set_u32(parent, child);
        }
    }
    e.mem.set_u32(child + 8, node);
    e.mem.set_u32(node + 4, child);
}

// Translated from 00ad1960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad1960 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the string-keyed tree's `_Buynode(left, parent, right, value, colour)` (thiscall,
/// five words, `RET 0x14`): allocates one node ([`fn_00ad5370`] with 1); when that succeeded
/// it stores the three links, copy-constructs the value at `+0xc` ([`fn_00ad3000`]), stores
/// the colour byte at `+0x44` and clears "is nil" (`+0x45`). Returns the allocation.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ad1960(
    e: &mut Engine,
    _this: Ptr,
    left: u32,
    parent: u32,
    right: u32,
    value: u32,
    colour: u32,
) -> u32 {
    let node = fn_00ad5370(e, 1, 0);
    if node != 0 {
        e.mem.set_u32(node, left);
        e.mem.set_u32(node + 4, parent);
        e.mem.set_u32(node + 8, right);
        fn_00ad3000(e, Ptr::new(node + 0xc), value);
        e.mem.set_u8(node + 0x44, colour as u8);
        e.mem.set_u8(node + 0x45, 0);
    }
    node
}

/// `std::_Allocate<char>` (`0065fa30`, cdecl: count, a null pointer of the element type).
const ALLOCATE_CHARACTER_BLOCK_0065FA30: u32 = 0x0065_fa30;

// Translated from 00ad1a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `erase(first, last)` of the vector of string pairs (thiscall, five words: the
/// iterator it fills and the two iterators' words, `RET 0x14`): checks both iterators; when
/// they differ (`00ac0460`) copies the tail down ([`fn_00ad5ec0`]; its tag words are a zero
/// and two bytes the exe leaves uninitialised, which it does not read), destroys the pairs
/// left at the end ([`fn_00ad6a30`] with 0) and moves the end. Returns the first iterator.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ad1a90(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    let base = this.addr();
    let start = vector_iterator(e, base, first);
    let end = vector_iterator(e, base, last);
    e.with_stack(0x10, |e, frame| {
        let start_at = frame.addr();
        let end_at = frame.addr() + 8;
        e.mem.set_u32(start_at, start.0);
        e.mem.set_u32(start_at + 4, start.1);
        e.mem.set_u32(end_at, end.0);
        e.mem.set_u32(end_at + 4, end.1);
        if e.call(ITERATOR_NOT_EQUAL_00AC0460, &args![start_at, end_at])
            .bool()
        {
            let old_last = e.mem.u32(base + 0x10);
            let new_last = fn_00ad5ec0(e, end.1, old_last, start.1, 0, 0, 0);
            let old_last = e.mem.u32(base + 0x10);
            let mut element = new_last;
            while element != old_last {
                fn_00ad6a30(e, Ptr::new(element), 0);
                element = element.wrapping_add(0x38);
            }
            e.mem.set_u32(base + 0x10, new_last);
        }
    });
    e.mem.set_u32(out.addr(), start.0);
    e.mem.set_u32(out.addr() + 4, start.1);
    out
}

// Translated from 00ad2500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `basic_stringbuf<char>::_Init(characters, count, state)` shape (thiscall,
/// three words, `RET 0xc`). Clears the high-water mark (`+0x3c`) and stores the state at
/// `+0x40`. With characters and not both "constant" and "no reading" (state bits 1 and 2) it
/// allocates a copy (`0065fa30`, `memcpy_s`), puts the mark at its end, sets the read area
/// over it unless reading is off, and unless the buffer is constant the write area (its
/// pointer at the end with bit 4, "at end"), a missing read area then becoming an empty one
/// at the copy; and marks the copy as owned (bit 0).
pub fn fn_00ad2500(e: &mut Engine, this: Ptr, characters: u32, count: u32, state: u32) {
    let base = this.addr();
    let set = |e: &mut Engine, slot: u32, value: u32| {
        let target = e.mem.u32(base + slot);
        e.mem.set_u32(target, value);
    };
    e.mem.set_u32(base + 0x3c, 0);
    e.mem.set_u32(base + 0x40, state);
    if count == 0 || e.mem.u32(base + 0x40) & 6 == 6 {
        return;
    }
    let copy = e
        .call(ALLOCATE_CHARACTER_BLOCK_0065FA30, &args![count, 0u32])
        .u32();
    e.call(MEMCPY_S, &args![copy, count, characters, count]);
    e.mem.set_u32(base + 0x3c, copy.wrapping_add(count));
    if e.mem.u32(base + 0x40) & 4 == 0 {
        set(e, 0x10, copy);
        set(e, 0x20, copy);
        set(e, 0x30, copy.wrapping_add(count).wrapping_sub(copy));
    }
    if e.mem.u32(base + 0x40) & 2 == 0 {
        let next = if e.mem.u32(base + 0x40) & 0x10 != 0 {
            copy.wrapping_add(count)
        } else {
            copy
        };
        set(e, 0x14, copy);
        set(e, 0x24, next);
        set(e, 0x34, copy.wrapping_add(count).wrapping_sub(next));
        if e.mem.u32(e.mem.u32(base + 0x20)) == 0 {
            set(e, 0x10, copy);
            set(e, 0x20, 0);
            set(e, 0x30, copy);
        }
    }
    let state = e.mem.u32(base + 0x40) | 1;
    e.mem.set_u32(base + 0x40, state);
}

// Translated from 00ad2640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad2640 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `_Destroy(first, last)` of the vector of string vectors (thiscall on the vector,
/// which it does not read; two words, `RET 8`; elements of 0x18 bytes): for each element it
/// empties the string vector ([`fn_00acede0`]) and frees its proxy (the word at `+0`,
/// `operator delete`); a compiled-out free of the element (`XOR ECX,ECX; AND ECX,1`) is kept.
pub fn fn_00ad2640(e: &mut Engine, _this: Ptr, first: u32, last: u32) {
    let mut element = first;
    while element != last {
        fn_00acede0(e, Ptr::new(element));
        let proxy = e.mem.u32(element);
        e.call(OPERATOR_DELETE, &args![proxy]);
        // `XOR ECX,ECX; AND ECX,1; JZ`: this free is compiled out (the constant 0).
        if std::hint::black_box(0u32) & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![element]);
        }
        element = element.wrapping_add(0x18);
    }
}

/// `00ad61e0` (thiscall, one word: the source): a string vector's assignment.
/// `00702220` (thiscall): frees a container's proxy (the word at `+0`).
const ASSIGN_STRING_VECTOR_00AD61E0: u32 = 0x00ad_61e0;
const FREE_PROXY_00702220: u32 = 0x0070_2220;

// Translated from 00ad1c10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad1c10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and the /GS cookie check (00ec408c) are not translated
/// Unnamed: `_Insert_n(position, count, value)` of the vector of string pairs (thiscall, four
/// words: the position's proxy (not read) and pointer, the count and the pair, `RET 0x10`;
/// elements of 0x38 bytes, maximum 0x4924924): as [`fn_00ad07c0`] with the pair helpers —
/// allocation [`fn_00ad53d0`], copies [`fn_00ad6b10`] and [`fn_00ad5f80`], fill
/// [`fn_00ad5bd0`], destruction [`fn_00ad6a30`] (with 0), the value's copy made with
/// [`fn_00ad3000`] and assigned with [`fn_00ad6190`], the backward move [`fn_00ad6000`]; the
/// copy's two strings are destroyed (`0044a730(1, 0)`, the second first). The tag words of
/// the range helpers are zeros and bytes the exe leaves uninitialised; only
/// [`fn_00ad6000`] reads one, and only to pass it on to a helper that does not read it.
pub fn fn_00ad1c10(e: &mut Engine, this: Ptr, _proxy: u32, at: u32, count: u32, value: u32) {
    let base = this.addr();
    let allocator = base + 8;
    let capacity = if e.mem.u32(base + 0xc) == 0 {
        0
    } else {
        vector_count(e, base, 0x14, 0x38)
    };
    if count == 0 {
        return;
    }
    let maximum = std::hint::black_box(0x0492_4924u32);
    let limit = if maximum > 0 { maximum } else { 1 };
    if limit.wrapping_sub(vector_count(e, base, 0x10, 0x38)) < count {
        e.call(VECTOR_TOO_LONG_00AC0B00, &args![]);
        return;
    }
    if capacity < vector_count(e, base, 0x10, 0x38).wrapping_add(count) {
        let maximum = std::hint::black_box(0x0492_4924u32);
        let limit = if maximum > 0 { maximum } else { 1 };
        let mut new_capacity = if limit.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        if new_capacity < vector_count(e, base, 0x10, 0x38).wrapping_add(count) {
            new_capacity = vector_count(e, base, 0x10, 0x38).wrapping_add(count);
        }
        let block = fn_00ad53d0(e, new_capacity, 0);
        let first = e.mem.u32(base + 0xc);
        let mut next = fn_00ad6b10(e, first, at, block, allocator);
        fn_00ad5bd0(e, next, count, value, allocator, 0, 0);
        next = count.wrapping_mul(0x38).wrapping_add(next);
        let last = e.mem.u32(base + 0x10);
        fn_00ad5f80(e, at, last, next, allocator);
        let new_size = vector_count(e, base, 0x10, 0x38).wrapping_add(count);
        if e.mem.u32(base + 0xc) != 0 {
            let last = e.mem.u32(base + 0x10);
            let mut element = e.mem.u32(base + 0xc);
            while element != last {
                fn_00ad6a30(e, Ptr::new(element), 0);
                element = element.wrapping_add(0x38);
            }
            // The capacity the exe computes here is never used.
            let _old_capacity = vector_count(e, base, 0x14, 0x38);
            let storage = e.mem.u32(base + 0xc);
            e.call(OPERATOR_DELETE, &args![storage]);
        }
        e.mem.set_u32(
            base + 0x14,
            new_capacity.wrapping_mul(0x38).wrapping_add(block),
        );
        e.mem
            .set_u32(base + 0x10, new_size.wrapping_mul(0x38).wrapping_add(block));
        e.mem.set_u32(base + 0xc, block);
        return;
    }
    e.with_stack(0x38, |e, copy| {
        let copy = copy.addr();
        fn_00ad3000(e, Ptr::new(copy), value);
        let after = (e.mem.u32(base + 0x10).wrapping_sub(at) as i32 / 0x38) as u32;
        if after < count {
            let last = e.mem.u32(base + 0x10);
            fn_00ad6b10(
                e,
                at,
                last,
                count.wrapping_mul(0x38).wrapping_add(at),
                allocator,
            );
            let last = e.mem.u32(base + 0x10);
            let after = (last.wrapping_sub(at) as i32 / 0x38) as u32;
            fn_00ad5bd0(e, last, count.wrapping_sub(after), copy, allocator, 0, 0);
            let last = count
                .wrapping_mul(0x38)
                .wrapping_add(e.mem.u32(base + 0x10));
            e.mem.set_u32(base + 0x10, last);
            let stop = e
                .mem
                .u32(base + 0x10)
                .wrapping_sub(count.wrapping_mul(0x38));
            let mut element = at;
            while element != stop {
                fn_00ad6190(e, Ptr::new(element), copy);
                element = element.wrapping_add(0x38);
            }
        } else {
            let old_last = e.mem.u32(base + 0x10);
            let last = e.mem.u32(base + 0x10);
            let moved = old_last.wrapping_sub(count.wrapping_mul(0x38));
            let new_last = fn_00ad5f80(e, moved, old_last, last, allocator);
            e.mem.set_u32(base + 0x10, new_last);
            fn_00ad6000(
                e,
                at,
                old_last.wrapping_sub(count.wrapping_mul(0x38)),
                old_last,
                0,
                0,
                0,
            );
            let stop = count.wrapping_mul(0x38).wrapping_add(at);
            let mut element = at;
            while element != stop {
                fn_00ad6190(e, Ptr::new(element), copy);
                element = element.wrapping_add(0x38);
            }
        }
        e.call(STRING_TIDY_0044A730, &args![copy + 0x1c, 1u32, 0u32]);
        e.call(STRING_TIDY_0044A730, &args![copy, 1u32, 0u32]);
    });
}

// Translated from 00ad26f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad26f0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `_Insert_n(position, count, value)` of the vector of string vectors (thiscall,
/// four words: the position's proxy (not read) and pointer, the count and the value,
/// `RET 0x10`; elements of 0x18 bytes, maximum 0xaaaaaaa): as [`fn_00ad07c0`] with the
/// string-vector helpers — allocation [`fn_00ad5430`], copies [`fn_00ad6b60`] and
/// [`fn_00ad6040`], fill [`fn_00ad5cc0`], the old elements destroyed with [`fn_00ad2640`],
/// the value's copy made with [`fn_00ad3150`] and assigned with `00ad61e0`, the backward
/// move [`fn_00ad60c0`]. The copy is emptied ([`fn_00acede0`]) and its proxy freed
/// (`00702220` in the first in-place case, `operator delete` in the second).
pub fn fn_00ad26f0(e: &mut Engine, this: Ptr, _proxy: u32, at: u32, count: u32, value: u32) {
    let base = this.addr();
    let allocator = base + 8;
    let capacity = if e.mem.u32(base + 0xc) == 0 {
        0
    } else {
        vector_count(e, base, 0x14, 0x18)
    };
    if count == 0 {
        return;
    }
    let maximum = std::hint::black_box(0x0aaa_aaaau32);
    let limit = if maximum > 0 { maximum } else { 1 };
    if limit.wrapping_sub(vector_count(e, base, 0x10, 0x18)) < count {
        e.call(VECTOR_TOO_LONG_00AC0B00, &args![]);
        return;
    }
    if capacity < vector_count(e, base, 0x10, 0x18).wrapping_add(count) {
        let maximum = std::hint::black_box(0x0aaa_aaaau32);
        let limit = if maximum > 0 { maximum } else { 1 };
        let mut new_capacity = if limit.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        if new_capacity < vector_count(e, base, 0x10, 0x18).wrapping_add(count) {
            new_capacity = vector_count(e, base, 0x10, 0x18).wrapping_add(count);
        }
        let block = fn_00ad5430(e, new_capacity, 0);
        let first = e.mem.u32(base + 0xc);
        let mut next = fn_00ad6b60(e, first, at, block, allocator);
        fn_00ad5cc0(e, next, count, value, allocator, 0, 0);
        next = count.wrapping_mul(0x18).wrapping_add(next);
        let last = e.mem.u32(base + 0x10);
        fn_00ad6040(e, at, last, next, allocator);
        let new_size = vector_count(e, base, 0x10, 0x18).wrapping_add(count);
        if e.mem.u32(base + 0xc) != 0 {
            let first = e.mem.u32(base + 0xc);
            let last = e.mem.u32(base + 0x10);
            fn_00ad2640(e, this, first, last);
            // The capacity the exe computes here is never used.
            let _old_capacity = vector_count(e, base, 0x14, 0x18);
            let storage = e.mem.u32(base + 0xc);
            e.call(OPERATOR_DELETE, &args![storage]);
        }
        e.mem.set_u32(
            base + 0x14,
            new_capacity.wrapping_mul(0x18).wrapping_add(block),
        );
        e.mem
            .set_u32(base + 0x10, new_size.wrapping_mul(0x18).wrapping_add(block));
        e.mem.set_u32(base + 0xc, block);
        return;
    }
    e.with_stack(0x18, |e, copy| {
        let copy = copy.addr();
        fn_00ad3150(e, Ptr::new(copy), value);
        let after = (e.mem.u32(base + 0x10).wrapping_sub(at) as i32 / 0x18) as u32;
        if after < count {
            let last = e.mem.u32(base + 0x10);
            fn_00ad6b60(
                e,
                at,
                last,
                count.wrapping_mul(0x18).wrapping_add(at),
                allocator,
            );
            let last = e.mem.u32(base + 0x10);
            let after = (last.wrapping_sub(at) as i32 / 0x18) as u32;
            fn_00ad5cc0(e, last, count.wrapping_sub(after), copy, allocator, 0, 0);
            let last = count
                .wrapping_mul(0x18)
                .wrapping_add(e.mem.u32(base + 0x10));
            e.mem.set_u32(base + 0x10, last);
            let stop = e
                .mem
                .u32(base + 0x10)
                .wrapping_sub(count.wrapping_mul(0x18));
            let mut element = at;
            while element != stop {
                e.call(ASSIGN_STRING_VECTOR_00AD61E0, &args![element, copy]);
                element = element.wrapping_add(0x18);
            }
            fn_00acede0(e, Ptr::new(copy));
            e.call(FREE_PROXY_00702220, &args![copy]);
        } else {
            let old_last = e.mem.u32(base + 0x10);
            let last = e.mem.u32(base + 0x10);
            let moved = old_last.wrapping_sub(count.wrapping_mul(0x18));
            let new_last = fn_00ad6040(e, moved, old_last, last, allocator);
            e.mem.set_u32(base + 0x10, new_last);
            fn_00ad60c0(
                e,
                at,
                old_last.wrapping_sub(count.wrapping_mul(0x18)),
                old_last,
                0,
                0,
                0,
            );
            let stop = count.wrapping_mul(0x18).wrapping_add(at);
            let mut element = at;
            while element != stop {
                e.call(ASSIGN_STRING_VECTOR_00AD61E0, &args![element, copy]);
                element = element.wrapping_add(0x18);
            }
            fn_00acede0(e, Ptr::new(copy));
            let proxy = e.mem.u32(copy);
            e.call(OPERATOR_DELETE, &args![proxy]);
        }
    });
}

/// The stream buffer's mutex lock (`00ec3277`, thiscall).
const MUTEX_LOCK: u32 = 0x00ec_3277;
/// `00ad68c0` (cdecl, six words: source first, source last, destination, allocator and two tag
/// words it does not read): copy-constructs a range of strings; returns the destination's
/// end.
const COPY_CONSTRUCT_STRINGS_00AD68C0: u32 = 0x00ad_68c0;

// Translated from 00ad2fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the stream sentry's shared constructor (thiscall, one word: the stream, `RET 4`):
/// stores the stream at `+0` and, when its buffer (`base + 0x28`) is set, locks the buffer's
/// mutex at `+4` (`00ec3277`). Returns `this`.
pub fn fn_00ad2fa0(e: &mut Engine, this: Ptr, stream: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), stream.addr());
    let stream = e.mem.u32(this.addr());
    let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
    if buffer != 0 {
        let stream = e.mem.u32(this.addr());
        let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
        e.call(MUTEX_LOCK, &args![buffer + 4]);
    }
    this
}

// Translated from 00ad3000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad3000 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: a pair of strings' copy constructor (thiscall, one word: the source pair,
/// `RET 4`): each of the strings at `+0` and `+0x1c` is initialised (`0044a730(0, 0)`) and
/// assigned the source's (`0044a580(source, 0, npos)`). Returns `this`.
pub fn fn_00ad3000(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    fn_00acb310(e, this, source, source.wrapping_add(0x1c))
}

// Translated from 00ad3150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad3150 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `vector<string>` copy constructor (thiscall, one word: the source vector,
/// `RET 4`). Makes the proxy ([`fn_00ad33c0`]; the exe passes `this` as its unread word),
/// reserves the source's size ([`fn_00aced20`]) and, when that returned nonzero, copies the
/// source's strings between checked iterators (the end checked first) into the storage
/// (`00ad68c0`; its tag words are a zero and a byte the exe leaves uninitialised, which it
/// does not read), storing the end at `+0x10`. Returns `this`.
pub fn fn_00ad3150(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let base = this.addr();
    fn_00ad33c0(e, this, base);
    let count = vector_count(e, source, 0x10, 0x1c);
    if fn_00aced20(e, this, count) != 0 {
        let last = e.mem.u32(source + 0x10);
        let end = vector_iterator(e, source, last);
        let first = e.mem.u32(source + 0xc);
        let begin = vector_iterator(e, source, first);
        let destination = e.mem.u32(base + 0xc);
        let new_last = e
            .call(
                COPY_CONSTRUCT_STRINGS_00AD68C0,
                &args![begin.1, end.1, destination, base + 8, 0u32, 0u32],
            )
            .u32();
        e.mem.set_u32(base + 0x10, new_last);
    }
    this
}

/// A checked vector iterator's `operator+=(count)` for elements of `element` bytes: a null
/// proxy calls `_invalid_parameter_noinfo`; the vector is the proxy's word (0 for no proxy);
/// a target past the vector's last element or before its first calls it too. The pointer
/// (`+4`) moves to the target. Returns `this`.
fn vector_iterator_advance(e: &mut Engine, this: Ptr, count: u32, element: u32) -> Ptr {
    let base = this.addr();
    if e.mem.u32(base) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let vector = |e: &Engine| {
        let proxy = e.mem.u32(base);
        if proxy != 0 {
            e.mem.u32(proxy)
        } else {
            0
        }
    };
    let target = |e: &Engine| {
        count
            .wrapping_mul(element)
            .wrapping_add(e.mem.u32(base + 4))
    };
    let owner = vector(e);
    if target(e) > e.mem.u32(owner + 0x10) {
        e.call(INVALID_PARAMETER, &args![]);
    } else {
        let owner = vector(e);
        if target(e) < e.mem.u32(owner + 0xc) {
            e.call(INVALID_PARAMETER, &args![]);
        }
    }
    let moved = target(e);
    e.mem.set_u32(base + 4, moved);
    this
}

// Translated from 00ad35e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked `vector<string>` iterator's `operator+=(count)` (thiscall, one word,
/// `RET 4`; elements of 0x1c bytes; see [`vector_iterator_advance`]).
pub fn fn_00ad35e0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    vector_iterator_advance(e, this, count, 0x1c)
}

// Translated from 00ad36a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked `vector<char>` iterator's `operator+=(count)` (thiscall, one word,
/// `RET 4`; one-byte elements).
pub fn fn_00ad36a0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    vector_iterator_advance(e, this, count, 1)
}

// Translated from 00ad3750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked iterator's `operator+=(count)` over the vector of string pairs
/// (thiscall, one word, `RET 4`; elements of 0x38 bytes).
pub fn fn_00ad3750(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    vector_iterator_advance(e, this, count, 0x38)
}

// Translated from 00ad3810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked iterator's `operator+=(count)` over the vector of string vectors
/// (thiscall, one word, `RET 4`; elements of 0x18 bytes).
pub fn fn_00ad3810(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    vector_iterator_advance(e, this, count, 0x18)
}

/// The CRT's `_Tolower` (`00ec379c`) and `_Toupper` (`00ec3909`): cdecl, the character
/// (zero-extended) and the `_Ctypevec`; the result is `AL`.
const CRT_TOLOWER: u32 = 0x00ec_379c;
const CRT_TOUPPER: u32 = 0x00ec_3909;

// Translated from 00ad3eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::locale::facet` scalar deleting destructor (the map's vtable name; thiscall, one
/// word, `RET 4`): stores the vtable `010a31fc` and with flag bit 0 frees `this`. Returns
/// `this`.
pub fn locale_facet_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_FACET_010A31FC);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00ad3ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ctype_base` scalar deleting destructor (the map's vtable name; thiscall, one word,
/// `RET 4`): stores the vtables `010a31f4` then `010a31fc` and with flag bit 0 frees `this`.
/// Returns `this`.
pub fn ctype_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_CTYPE_BASE_010A31F4);
    e.mem.set_u32(this.addr(), VTABLE_FACET_010A31FC);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// Converts each byte of `[first, last)` in place through `convert` (the CRT's `_Tolower` or
/// `_Toupper`, with the facet's `_Ctypevec` at `+8`) and returns `last`.
fn ctype_convert_range(e: &mut Engine, this: Ptr, first: u32, last: u32, convert: u32) -> u32 {
    let mut at = first;
    while at != last {
        let character = u32::from(e.mem.u8(at));
        let converted = e.call(convert, &args![character, this.addr() + 8]).u8();
        e.mem.set_u8(at, converted);
        at = at.wrapping_add(1);
    }
    at
}

// Translated from 00ad3f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ctype<char>::do_tolower(char)` (Ghidra library match; thiscall, one word, `RET 4`):
/// `_Tolower(ch, this + 8)`, its `EAX` returned.
pub fn ctype_do_tolower(e: &mut Engine, this: Ptr, character: u32) -> u32 {
    e.call(CRT_TOLOWER, &args![character & 0xff, this.addr() + 8])
        .u32()
}

// Translated from 00ad3f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ctype<char>::do_tolower(char *, const char *)` (Ghidra library match; thiscall, two
/// words, `RET 8`): lowers each character in place; returns the end.
pub fn ctype_do_tolower_range(e: &mut Engine, this: Ptr, first: u32, last: u32) -> u32 {
    ctype_convert_range(e, this, first, last, CRT_TOLOWER)
}

// Translated from 00ad3f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ctype<char>::do_toupper(char)` (Ghidra library match; thiscall, one word, `RET 4`):
/// `_Toupper(ch, this + 8)`, its `EAX` returned.
pub fn ctype_do_toupper(e: &mut Engine, this: Ptr, character: u32) -> u32 {
    e.call(CRT_TOUPPER, &args![character & 0xff, this.addr() + 8])
        .u32()
}

// Translated from 00ad3fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ctype<char>::do_toupper(char *, const char *)` (Ghidra library match; thiscall, two
/// words, `RET 8`): raises each character in place; returns the end.
pub fn ctype_do_toupper_range(e: &mut Engine, this: Ptr, first: u32, last: u32) -> u32 {
    ctype_convert_range(e, this, first, last, CRT_TOUPPER)
}

// Translated from 00ad4010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ctype<char>::do_widen(first, last, destination)` (Ghidra library match; thiscall,
/// three words, `RET 0xc`): the facet's slot `0x1c` (`_Do_widen_s`) with the size
/// `last - first`; returns its `EAX`.
pub fn ctype_do_widen_range(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    last: u32,
    destination: u32,
) -> u32 {
    let size = last.wrapping_sub(first);
    e.vcall(this.addr(), 0x1c, &args![first, last, destination, size])
        .u32()
}

// Translated from 00ad4040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ctype<char>::_Do_widen_s(first, last, destination, size)` shape (thiscall,
/// four words, `RET 0x10`): `_invalid_parameter_noinfo` when the size is below the length;
/// copies the characters (`memcpy_s`) and returns `last`.
pub fn fn_00ad4040(
    e: &mut Engine,
    _this: Ptr,
    first: u32,
    last: u32,
    destination: u32,
    size: u32,
) -> u32 {
    if size < last.wrapping_sub(first) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let length = last.wrapping_sub(first);
    e.call(MEMCPY_S, &args![destination, size, first, length]);
    last
}

// Translated from 00ad4080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ctype<char>::do_narrow(ch, default)` shape (thiscall, two words, the second
/// not read, `RET 8`): returns the character byte (`MOV AL`).
pub fn fn_00ad4080(_e: &mut Engine, _this: Ptr, character: u32, _unused_2: u32) -> u8 {
    character as u8
}

// Translated from 00ad4090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::ctype<char>::do_narrow(first, last, default, destination)` (Ghidra library match;
/// thiscall, four words, `RET 0x10`): the facet's slot `0x28` (`_Do_narrow_s`) with the
/// default byte zero-extended and the size `last - first`; returns its `EAX`.
pub fn ctype_do_narrow_range(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    last: u32,
    default: u32,
    destination: u32,
) -> u32 {
    let size = last.wrapping_sub(first);
    e.vcall(
        this.addr(),
        0x28,
        &args![first, last, default & 0xff, destination, size],
    )
    .u32()
}

// Translated from 00ad40d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ctype<char>::_Do_narrow_s(first, last, default, destination, size)` shape
/// (thiscall, five words, the default not read, `RET 0x14`): `_invalid_parameter_noinfo`
/// when the size is below the length; copies the characters (`memcpy_s`) and returns `last`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ad40d0(
    e: &mut Engine,
    _this: Ptr,
    first: u32,
    last: u32,
    _unused_3: u32,
    destination: u32,
    size: u32,
) -> u32 {
    if size < last.wrapping_sub(first) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let length = last.wrapping_sub(first);
    e.call(MEMCPY_S, &args![destination, size, first, length]);
    last
}

// Translated from 00ad41a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `ctype_base` destructor (thiscall): stores the vtables `010a31f4` then
/// `010a31fc`.
pub fn fn_00ad41a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CTYPE_BASE_010A31F4);
    e.mem.set_u32(this.addr(), VTABLE_FACET_010A31FC);
}

/// The CRT's `strlen` (`00ec6130`, cdecl: the text).
const CRT_STRLEN: u32 = 0x00ec_6130;
/// `00a0c430` (thiscall, two words: text, length) and `00a0c570` (thiscall, three words:
/// string, offset, count): the string's `append`; both return the string.
const STRING_APPEND_CHARS_00A0C430: u32 = 0x00a0_c430;
const STRING_APPEND_STRING_00A0C570: u32 = 0x00a0_c570;
/// `memmove_s` (`00ec7ce3`, cdecl: destination, its size, source, count).
const MEMMOVE_S: u32 = 0x00ec_7ce3;
/// `00ad6bb0` (thiscall, one word: the source): the string's assignment.
const STRING_ASSIGN_COPY_00AD6BB0: u32 = 0x00ad_6bb0;
/// The cached converter facet, its facet id (`.data`), the `bad_cast` constructor
/// (`00ec79c4`, thiscall, one word: the message), its message `"bad cast"` and its throw
/// information.
const CACHED_CODECVT_011F6D94: u32 = 0x011f_6d94;
const CODECVT_ID_011C33E0: u32 = 0x011c_33e0;
const BAD_CAST_CTOR_00EC79C4: u32 = 0x00ec_79c4;
const MESSAGE_BAD_CAST_010A31B4: u32 = 0x010a_31b4;
const BAD_CAST_THROW_INFO_0116CDB8: u32 = 0x0116_cdb8;
/// The facet registration (`00ec3599`, cdecl, one word).
const FACET_REGISTER_00EC3599: u32 = 0x00ec_3599;
/// The `bad_alloc` the allocation helpers throw: the `exception` constructor from a message
/// pointer (`00ec78f1`, thiscall, one word: the address of the pointer), its vtable and its
/// throw information.
const EXCEPTION_FROM_MESSAGE_00EC78F1: u32 = 0x00ec_78f1;
const VTABLE_BAD_ALLOC_010176F8: u32 = 0x0101_76f8;
const BAD_ALLOC_THROW_INFO_01140EF8: u32 = 0x0114_0ef8;

/// The concatenation both `operator+` shapes share: a copy of `left` (`0044a730(0, 0)`,
/// `0044a580`), `append` applied to it, the result assigned to `out` (initialised first) and
/// the copy destroyed. Returns `out`.
fn concatenate(
    e: &mut Engine,
    out: u32,
    left: u32,
    append: impl FnOnce(&mut Engine, u32) -> u32,
) -> u32 {
    e.with_stack(0x1c, |e, copy| {
        let copy = copy.addr();
        e.call(STRING_TIDY_0044A730, &args![copy, 0u32, 0u32]);
        let npos = e.global::<u32>(STRING_NPOS_010173F0);
        e.call(STRING_ASSIGN_0044A580, &args![copy, left, 0u32, npos]);
        let joined = append(e, copy);
        e.call(STRING_TIDY_0044A730, &args![out, 0u32, 0u32]);
        let npos = e.global::<u32>(STRING_NPOS_010173F0);
        e.call(STRING_ASSIGN_0044A580, &args![out, joined, 0u32, npos]);
        e.call(STRING_TIDY_0044A730, &args![copy, 1u32, 0u32]);
    });
    out
}

// Translated from 00ad4210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad4210 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `operator+(string, const char *)` (cdecl: the result string, the left string, the
/// text): appends the text (`strlen`, `00a0c430`) to a copy of the left string and assigns
/// that to the result. Returns the result.
pub fn fn_00ad4210(e: &mut Engine, out: u32, left: u32, text: u32) -> u32 {
    concatenate(e, out, left, |e, copy| {
        let length = e.call(CRT_STRLEN, &args![text]).u32();
        e.call(STRING_APPEND_CHARS_00A0C430, &args![copy, text, length])
            .u32()
    })
}

// Translated from 00ad42e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad42e0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `operator+(string, string)` (cdecl: the result, the left string, the right
/// string): appends the right string (`00a0c570(right, 0, -1)`) to a copy of the left one and
/// assigns that to the result. Returns the result.
pub fn fn_00ad42e0(e: &mut Engine, out: u32, left: u32, right: u32) -> u32 {
    concatenate(e, out, left, |e, copy| {
        e.call(
            STRING_APPEND_STRING_00A0C570,
            &args![copy, right, 0u32, 0xffff_ffffu32],
        )
        .u32()
    })
}

/// Sets `state` on a stream's `basic_ios` (see [`fn_00aca4e0`]).
fn stream_setstate(e: &mut Engine, stream: u32, state: u32) {
    let base = stream_base(e, stream);
    fn_00aca4e0(e, Ptr::new(base), state, 0);
}

// Translated from 00ad43a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad43a0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `operator>>(istream &, string &)` (cdecl: the stream and the string). With a good
/// input sentry ([`fn_00ad2fa0`], [`fn_00ace390`] skipping white space) it takes the
/// locale's `ctype` facet, empties the string (`0044a690(0, npos)`) and reads characters
/// ([`fn_00aca9f0`], [`fn_00ace7d0`]) up to the width (`base + 0x18` when positive and below
/// `0xfffffffe`, else `0xfffffffe`; the exe's "maximum or 1" tests are kept) until an end of
/// file (`eofbit`) or a white-space class character (bits 0x48), appending each
/// (`00aadeb0(1, ch)`). Then it clears the width, sets `failbit` when nothing was read and
/// gives the state to the stream; the sentry is destroyed ([`fn_00aca9a0`]). Returns the
/// stream.
pub fn fn_00ad43a0(e: &mut Engine, stream: u32, string: u32) -> u32 {
    e.with_stack(0xc, |e, frame| {
        // Frame: the sentry at +0 (8 bytes), the locale local at +8.
        let sentry = frame.addr();
        let local = frame.addr() + 8;
        let mut state = 0u32;
        let mut changed = false;
        fn_00ad2fa0(e, Ptr::new(sentry), Ptr::new(stream));
        let owner = e.mem.u32(sentry);
        let good = fn_00ace390(e, Ptr::new(owner), 0);
        e.mem.set_u8(sentry + 4, good);
        if good != 0 {
            let ios = stream_base(e, stream);
            let locale = fn_00ace750(e, Ptr::new(ios), Ptr::new(local));
            let facet = fn_00ad38d0(e, locale);
            fn_00acb720(e, Ptr::new(local));
            e.call(STRING_ERASE_0044A690, &args![string, 0u32, 0xffff_ffffu32]);
            let width = e.mem.u32(stream_base(e, stream) + 0x18);
            let largest = || {
                let maximum = std::hint::black_box(0xffff_ffffu32);
                let limit = if maximum > 0 { maximum } else { 1 };
                if limit > 1 {
                    limit - 1
                } else {
                    1
                }
            };
            let mut left = if (width as i32) > 0 && width < largest() {
                e.mem.u32(stream_base(e, stream) + 0x18)
            } else {
                largest()
            };
            let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
            let mut character = fn_00aca9f0(e, Ptr::new(buffer));
            while left > 0 {
                if character == 0xffff_ffff {
                    state |= 1;
                    break;
                }
                let table = e.mem.u32(facet.addr() + 0x10);
                if e.mem.u16(table.wrapping_add((character & 0xff) * 2)) & 0x48 != 0 {
                    break;
                }
                e.call(
                    STRING_APPEND_FILL_00AADEB0,
                    &args![string, 1u32, character & 0xff],
                );
                changed = true;
                left -= 1;
                let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
                character = fn_00ace7d0(e, Ptr::new(buffer));
            }
        }
        let ios = stream_base(e, stream);
        e.mem.set_u32(ios + 0x18, 0);
        if !changed {
            state |= 2;
        }
        stream_setstate(e, stream, state);
        fn_00aca9a0(e, Ptr::new(sentry));
    });
    stream
}

/// Writes `count` copies of the stream's fill character (`base + 0x30`) through
/// [`fn_00ace8c0`]; returns `badbit` (4) when one fails. `count` is decremented as the exe's
/// loop does (`signed` says whether it tests `> 0` signed or `!= 0`).
fn write_fill(e: &mut Engine, stream: u32, count: &mut u32, signed: bool) -> u32 {
    while if signed {
        (*count as i32) > 0
    } else {
        *count != 0
    } {
        let fill = u32::from(e.mem.u8(stream_base(e, stream) + 0x30));
        let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
        if fn_00ace8c0(e, Ptr::new(buffer), fill) == 0xffff_ffff {
            return 4;
        }
        *count = count.wrapping_sub(1);
    }
    0
}

/// The tail both `operator<<` shapes share: the state to the stream, the flush unless an
/// exception is in flight ([`fn_00ad03e0`]) and the sentry's destruction.
fn finish_output(e: &mut Engine, stream: u32, sentry: u32, state: u32) {
    stream_setstate(e, stream, state);
    if !e.call(UNCAUGHT_EXCEPTION, &args![]).bool() {
        let owner = e.mem.u32(sentry);
        fn_00ad03e0(e, Ptr::new(owner));
    }
    fn_00aca9a0(e, Ptr::new(sentry));
}

// Translated from 00ad47f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad47f0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `operator<<(ostream &, const char *)` (cdecl: the stream and the text). The pad
/// is the width (`base + 0x18`) less the length (`strlen`) when the width is larger (signed),
/// else 0. With a good output sentry ([`fn_00ad0210`]) it pads before the text unless
/// left-adjusted (`adjustfield` 0x1c0 is 0x40), writes the text through the buffer's slot
/// `0x20` (`sputn`; a short count is `badbit`), pads after it and clears the width; a bad
/// sentry is `badbit`. Then the shared tail (state, flush, sentry). Returns the stream.
pub fn fn_00ad47f0(e: &mut Engine, stream: u32, text: u32) -> u32 {
    let mut state = 0u32;
    let length = e.call(CRT_STRLEN, &args![text]).u32();
    let width = e.mem.u32(stream_base(e, stream) + 0x18) as i32;
    let mut pad = if width > 0 && width > length as i32 {
        (e.mem.u32(stream_base(e, stream) + 0x18)).wrapping_sub(length)
    } else {
        0
    };
    e.with_stack(8, |e, sentry| {
        let sentry = sentry.addr();
        fn_00ad0210(e, Ptr::new(sentry), Ptr::new(stream));
        if e.mem.u8(sentry + 4) == 0 {
            state |= 4;
        } else {
            if e.mem.u32(stream_base(e, stream) + 0x10) & 0x1c0 != 0x40 {
                state |= write_fill(e, stream, &mut pad, true);
            }
            if state == 0 {
                let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
                if e.vcall(buffer, 0x20, &args![text, length]).u32() != length {
                    state |= 4;
                }
            }
            if state == 0 {
                state |= write_fill(e, stream, &mut pad, true);
            }
            let ios = stream_base(e, stream);
            e.mem.set_u32(ios + 0x18, 0);
        }
        finish_output(e, stream, sentry, state);
    });
    stream
}

// Translated from 00ad4ba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad4ba0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `operator<<(ostream &, const string &)` (cdecl: the stream and the string). The
/// pad is the width less the string's length (`+0x14`) when the width is positive and larger
/// (unsigned), else 0. With a good output sentry it pads unless left-adjusted, writes the
/// string's characters one by one ([`fn_00ace8c0`]; an index past the length calls
/// `_invalid_parameter_noinfo`), pads after it and clears the width; failures are `badbit`.
/// Then the shared tail. Returns the stream.
pub fn fn_00ad4ba0(e: &mut Engine, stream: u32, string: u32) -> u32 {
    let mut state = 0u32;
    let length = e.mem.u32(string + 0x14);
    let width = e.mem.u32(stream_base(e, stream) + 0x18);
    let mut pad = if (width as i32) > 0 && e.mem.u32(stream_base(e, stream) + 0x18) > length {
        e.mem
            .u32(stream_base(e, stream) + 0x18)
            .wrapping_sub(length)
    } else {
        0
    };
    e.with_stack(8, |e, sentry| {
        let sentry = sentry.addr();
        fn_00ad0210(e, Ptr::new(sentry), Ptr::new(stream));
        if e.mem.u8(sentry + 4) == 0 {
            state |= 4;
        } else {
            if e.mem.u32(stream_base(e, stream) + 0x10) & 0x1c0 != 0x40 {
                state |= write_fill(e, stream, &mut pad, false);
            }
            if state == 0 {
                let mut index = 0u32;
                while index < length {
                    if e.mem.u32(string + 0x14) < index {
                        e.call(INVALID_PARAMETER, &args![]);
                    }
                    let character =
                        u32::from(e.mem.u8(string_chars(e, string).wrapping_add(index)));
                    let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
                    if fn_00ace8c0(e, Ptr::new(buffer), character) == 0xffff_ffff {
                        state |= 4;
                        break;
                    }
                    index += 1;
                }
            }
            if state == 0 {
                state |= write_fill(e, stream, &mut pad, false);
            }
            let ios = stream_base(e, stream);
            e.mem.set_u32(ios + 0x18, 0);
        }
        finish_output(e, stream, sentry, state);
    });
    stream
}

// Translated from 00ad4fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad4fd0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `std::use_facet<codecvt<char, char, int>>(locale)` (cdecl, one word): as
/// [`fn_00ad38d0`] with the cached facet `011f6d94`, the id `011c33e0` (from the counter
/// `0126fba0`) and [`fn_00ad5500`] to make one. A failed make throws `bad_cast` (the game
/// does not return; the model goes on as the code after the throw does).
pub fn fn_00ad4fd0(e: &mut Engine, locale: Ptr) -> Ptr {
    e.with_stack(0x20, |e, frame| {
        // Frame: the inner lock at +0, the `bad_cast` object at +8 (0xc bytes), the cache local
        // at +0x18 and the outer lock at +0x1c.
        let inner_lock = frame.addr();
        let bad_cast = frame.addr() + 8;
        let cached_at = frame.addr() + 0x18;
        let outer_lock = frame.addr() + 0x1c;
        e.call(LOCKIT_CTOR, &args![outer_lock, 0u32]);
        let cached = e.global::<u32>(CACHED_CODECVT_011F6D94);
        e.mem.set_u32(cached_at, cached);
        if e.global::<u32>(CODECVT_ID_011C33E0) == 0 {
            e.call(LOCKIT_CTOR, &args![inner_lock, 0u32]);
            if e.global::<u32>(CODECVT_ID_011C33E0) == 0 {
                let next = e.global::<u32>(FACET_ID_COUNTER_0126FBA0).wrapping_add(1);
                e.set_global(FACET_ID_COUNTER_0126FBA0, next);
                let id = e.global::<u32>(FACET_ID_COUNTER_0126FBA0);
                e.set_global(CODECVT_ID_011C33E0, id);
            }
            e.call(LOCKIT_DTOR, &args![inner_lock]);
        }
        let id = e.global::<u32>(CODECVT_ID_011C33E0);
        let mut facet = locale_getfacet(e, locale, id);
        if facet == 0 {
            let cached = e.mem.u32(cached_at);
            if cached != 0 {
                facet = cached;
            } else {
                if fn_00ad5500(e, cached_at, locale.addr()) == 0xffff_ffff {
                    e.call(
                        BAD_CAST_CTOR_00EC79C4,
                        &args![bad_cast, MESSAGE_BAD_CAST_010A31B4],
                    );
                    e.call(
                        CXX_THROW_EXCEPTION,
                        &args![bad_cast, BAD_CAST_THROW_INFO_0116CDB8],
                    );
                }
                facet = e.mem.u32(cached_at);
                let made = e.mem.u32(cached_at);
                e.set_global(CACHED_CODECVT_011F6D94, made);
                let made = e.mem.u32(cached_at);
                facet_incref(e, Ptr::new(made));
                e.call(FACET_REGISTER_00EC3599, &args![made]);
            }
        }
        e.call(LOCKIT_DTOR, &args![outer_lock]);
        Ptr::new(facet)
    })
}

// Translated from 00ad5110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector<char>`'s copy of `[first, last)` to `destination` (cdecl, three words):
/// `memmove_s` of the length when it is positive (signed); returns `destination + length`.
pub fn fn_00ad5110(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let length = last.wrapping_sub(first);
    let end = destination.wrapping_add(length);
    if (length as i32) > 0 {
        e.call(MEMMOVE_S, &args![destination, length, first, length]);
    }
    end
}

// Translated from 00ad5180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the copy of a range of string vectors (cdecl: first, last, destination; elements
/// of 0x18 bytes): assigns each (`00ad61e0`) in order; returns the destination's end.
pub fn fn_00ad5180(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let end = ((last.wrapping_sub(first) as i32 / 0x18) as u32)
        .wrapping_mul(0x18)
        .wrapping_add(destination);
    let mut to = destination;
    let mut from = first;
    while from != last {
        e.call(ASSIGN_STRING_VECTOR_00AD61E0, &args![to, from]);
        to = to.wrapping_add(0x18);
        from = from.wrapping_add(0x18);
    }
    end
}

// Translated from 00ad5250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the backward copy of a range of strings (cdecl: first, last, destination end;
/// elements of 0x1c bytes): assigns each (`00ad6bb0`) from the back; returns the
/// destination's start.
pub fn fn_00ad5250(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let start = destination
        .wrapping_sub(((last.wrapping_sub(first) as i32 / 0x1c) as u32).wrapping_mul(0x1c));
    let mut to = destination;
    let mut from = last;
    while first != from {
        from = from.wrapping_sub(0x1c);
        to = to.wrapping_sub(0x1c);
        e.call(STRING_ASSIGN_COPY_00AD6BB0, &args![to, from]);
    }
    start
}

/// The `std::_Allocate<T>(count)` shape (cdecl, two words: the count and a null pointer it
/// does not read): a count of 0 allocates 0 bytes; when `0xffffffff / count` is below the
/// element size it throws `bad_alloc` (the game does not return; the model allocates after
/// the throw as the code after it does); allocates `count * size` (`operator new`).
fn allocate_elements(e: &mut Engine, count: u32, element: u32) -> u32 {
    // A count of 0 is stored back as 0 (`JA`; no change) and skips the size test.
    if count != 0 && 0xffff_ffff / count < element {
        e.with_stack(0x10, |e, frame| {
            // Frame: the null message pointer at +0, the exception object at +4.
            let message = frame.addr();
            let object = frame.addr() + 4;
            e.call(EXCEPTION_FROM_MESSAGE_00EC78F1, &args![object, message]);
            e.mem.set_u32(object, VTABLE_BAD_ALLOC_010176F8);
            e.call(
                CXX_THROW_EXCEPTION,
                &args![object, BAD_ALLOC_THROW_INFO_01140EF8],
            );
        });
    }
    e.call(OPERATOR_NEW, &args![count.wrapping_mul(element)])
        .u32()
}

// Translated from 00ad5310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `std::_Allocate<string>(count)` (cdecl, two words; elements of 0x1c bytes; see
/// [`allocate_elements`]).
pub fn fn_00ad5310(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 0x1c)
}

// Translated from 00ad5370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `std::_Allocate` of the string-keyed tree's nodes (cdecl, two words; 0x48 bytes
/// each).
pub fn fn_00ad5370(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 0x48)
}

// Translated from 00ad53d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `std::_Allocate` of string pairs (cdecl, two words; 0x38 bytes each).
pub fn fn_00ad53d0(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 0x38)
}

// Translated from 00ad5430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `std::_Allocate` of string vectors (cdecl, two words; 0x18 bytes each).
pub fn fn_00ad5430(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 0x18)
}

// Translated from 00ad5490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad5490 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: the string-keyed tree node's value destructor (thiscall on the node): destroys
/// the pair of strings at `+0xc` (`0044a730(1, 0)` on `+0x28`, then on `+0xc`).
pub fn fn_00ad5490(e: &mut Engine, this: Ptr) {
    let value = this.addr() + 0xc;
    e.call(STRING_TIDY_0044A730, &args![value + 0x1c, 1u32, 0u32]);
    e.call(STRING_TIDY_0044A730, &args![value, 1u32, 0u32]);
}

/// The `codecvt<char, char, int>` and `codecvt_base` vtables (`.rdata`).
const VTABLE_CODECVT_010A3204: u32 = 0x010a_3204;
const VTABLE_CODECVT_BASE_010A3228: u32 = 0x010a_3228;
/// `00ad6990` (cdecl: destination, source): copy-constructs one string. `00ad6d30` and
/// `00ad6e20` (cdecl, six words: source first, source last, destination, allocator and two
/// tag words they do not read): copy-construct a range of string pairs and of string
/// vectors. `00ad6800` (cdecl, six words, the same shape): copies a range of string pairs
/// backwards.
const CONSTRUCT_STRING_00AD6990: u32 = 0x00ad_6990;
const COPY_CONSTRUCT_PAIRS_00AD6D30: u32 = 0x00ad_6d30;
const COPY_CONSTRUCT_VECTORS_00AD6E20: u32 = 0x00ad_6e20;
const COPY_PAIRS_BACKWARD_00AD6800: u32 = 0x00ad_6800;

// Translated from 00ad5500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad5500 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `codecvt<char, char, int>::_Getcat(slot, locale)` (cdecl, two words). When the
/// slot is set and holds 0 it allocates 8 bytes; when that succeeds it takes the locale's
/// name ([`fn_00ad3e60`] into a stack string), builds a `_Locinfo` from it ([`fn_00ad3ca0`])
/// and stores the vtables `010a31fc` (0 at `+4` after it), `010a3228` and `010a3204`. The block
/// (or 0) goes into the slot; the `_Locinfo` ([`fn_00ad3db0`]) and the string
/// (`0044a730(1, 0)`) are destroyed when they were built. Returns 2.
pub fn fn_00ad5500(e: &mut Engine, slot: u32, locale: u32) -> u32 {
    if slot == 0 || e.mem.u32(slot) != 0 {
        return 2;
    }
    e.with_stack(0x90, |e, frame| {
        // Frame: the name string at +0 (0x1c bytes), the `_Locinfo` at +0x1c.
        let string = frame.addr();
        let locinfo = frame.addr() + 0x1c;
        let mut built = 0u32;
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let made = if block != 0 {
            let name = fn_00ad3e60(e, Ptr::new(locale), Ptr::new(string));
            built |= 1;
            fn_00ad3ca0(e, Ptr::new(locinfo), name.addr());
            built |= 2;
            e.mem.set_u32(block, VTABLE_FACET_010A31FC);
            e.mem.set_u32(block + 4, 0);
            e.mem.set_u32(block, VTABLE_CODECVT_BASE_010A3228);
            e.mem.set_u32(block, VTABLE_CODECVT_010A3204);
            block
        } else {
            0
        };
        e.mem.set_u32(slot, made);
        if built & 2 != 0 {
            built &= !2;
            fn_00ad3db0(e, Ptr::new(locinfo));
        }
        if built & 1 != 0 {
            e.call(STRING_TIDY_0044A730, &args![string, 1u32, 0u32]);
        }
    });
    2
}

// Translated from 00ad5670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::codecvt_base` scalar deleting destructor (the map's vtable name; thiscall, one word,
/// `RET 4`): stores the vtables `010a3228` then `010a31fc` and with flag bit 0 frees `this`.
/// Returns `this`.
pub fn codecvt_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_CODECVT_BASE_010A3228);
    e.mem.set_u32(this.addr(), VTABLE_FACET_010A31FC);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00ad56b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::codecvt<char, char, int>::do_in` (the map's vtable name; thiscall, seven words:
/// state, from, from end, from next, to, to end, to next; `RET 0x1c`): "no conversion":
/// stores `from` in `from next` and `to` in `to next`; returns 3.
#[allow(clippy::too_many_arguments)]
pub fn codecvt_do_in(
    e: &mut Engine,
    _this: Ptr,
    _state: u32,
    from: u32,
    _from_end: u32,
    from_next: u32,
    to: u32,
    _to_end: u32,
    to_next: u32,
) -> u32 {
    e.mem.set_u32(from_next, from);
    e.mem.set_u32(to_next, to);
    3
}

// Translated from 00ad56e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::codecvt<char, char, int>::do_unshift` (the map's vtable name; thiscall, four words:
/// state, to, to end, to next; `RET 0x10`): stores `to` in `to next`; returns 3.
pub fn codecvt_do_unshift(
    e: &mut Engine,
    _this: Ptr,
    _state: u32,
    to: u32,
    _to_end: u32,
    to_next: u32,
) -> u32 {
    e.mem.set_u32(to_next, to);
    3
}

// Translated from 00ad5700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::codecvt<char, char, int>::do_length` (Ghidra library match; thiscall, four words:
/// state, first, last, maximum; `RET 0x10`): the smaller of the maximum and `last - first`
/// (unsigned).
pub fn codecvt_do_length(
    _e: &mut Engine,
    _this: Ptr,
    _state: u32,
    first: u32,
    last: u32,
    maximum: u32,
) -> u32 {
    if maximum < last.wrapping_sub(first) {
        maximum
    } else {
        last.wrapping_sub(first)
    }
}

// Translated from 00ad5730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::codecvt<char, char, int>` scalar deleting destructor (the map's vtable name;
/// thiscall, one word, `RET 4`): stores the vtables `010a3204`, `010a3228` and `010a31fc` and
/// with flag bit 0 frees `this`. Returns `this`.
pub fn codecvt_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_CODECVT_010A3204);
    e.mem.set_u32(this.addr(), VTABLE_CODECVT_BASE_010A3228);
    e.mem.set_u32(this.addr(), VTABLE_FACET_010A31FC);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00ad5770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad5770 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `std::getline(istream &, string &, char delimiter)` (cdecl, three words). With a
/// good input sentry ([`fn_00ad2fa0`], [`fn_00ace390`] without skipping) it empties the
/// string and reads characters ([`fn_00aca9f0`], [`fn_00ace7d0`]): an end of file sets
/// `eofbit`; the delimiter is taken ([`fn_00ace860`]) and ends the line; a string at the
/// maximum length (`0xfffffffe`; the exe's "maximum or 1" tests are kept) sets `failbit`;
/// anything else is appended (`00aadeb0(1, ch)`). Nothing taken sets `failbit`. The state
/// goes to the stream and the sentry is destroyed. Returns the stream.
pub fn fn_00ad5770(e: &mut Engine, stream: u32, string: u32, delimiter: u32) -> u32 {
    e.with_stack(8, |e, sentry| {
        let sentry = sentry.addr();
        let mut state = 0u32;
        let mut changed = false;
        fn_00ad2fa0(e, Ptr::new(sentry), Ptr::new(stream));
        let owner = e.mem.u32(sentry);
        let good = fn_00ace390(e, Ptr::new(owner), 1);
        e.mem.set_u8(sentry + 4, good);
        if good != 0 {
            e.call(STRING_ERASE_0044A690, &args![string, 0u32, 0xffff_ffffu32]);
            let delimiter = delimiter & 0xff;
            let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
            let mut character = fn_00aca9f0(e, Ptr::new(buffer));
            loop {
                if character == 0xffff_ffff {
                    state |= 1;
                    break;
                }
                if character == delimiter {
                    changed = true;
                    let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
                    fn_00ace860(e, Ptr::new(buffer));
                    break;
                }
                let maximum = std::hint::black_box(0xffff_ffffu32);
                let limit = if maximum > 0 { maximum } else { 1 };
                let largest = if limit > 1 { limit - 1 } else { 1 };
                if largest <= e.mem.u32(string + 0x14) {
                    state |= 2;
                    break;
                }
                e.call(
                    STRING_APPEND_FILL_00AADEB0,
                    &args![string, 1u32, character & 0xff],
                );
                changed = true;
                let buffer = e.mem.u32(stream_base(e, stream) + 0x28);
                character = fn_00ace7d0(e, Ptr::new(buffer));
            }
        }
        if !changed {
            state |= 2;
        }
        stream_setstate(e, stream, state);
        fn_00aca9a0(e, Ptr::new(sentry));
    });
    stream
}

// Translated from 00ad5b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad5b00 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `_Uninit_fill_n` of strings (cdecl, six words: destination, count, value and three
/// it does not read): copy-constructs the value into each of `count` 0x1c-byte slots
/// (`00ad6990`).
pub fn fn_00ad5b00(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        e.call(CONSTRUCT_STRING_00AD6990, &args![destination, value]);
        count -= 1;
        destination = destination.wrapping_add(0x1c);
    }
}

// Translated from 00ad5bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad5bd0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `_Uninit_fill_n` of string pairs (cdecl, six words: destination, count, value and
/// three it does not read): copy-constructs the value ([`fn_00ad3000`]) into each non-null
/// 0x38-byte slot.
pub fn fn_00ad5bd0(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        if destination != 0 {
            fn_00ad3000(e, Ptr::new(destination), value);
        }
        count -= 1;
        destination = destination.wrapping_add(0x38);
    }
}

// Translated from 00ad5cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad5cc0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: `_Uninit_fill_n` of string vectors (cdecl, six words: destination, count, value
/// and three it does not read): copy-constructs the value ([`fn_00ad3150`]) into each
/// non-null 0x18-byte slot.
pub fn fn_00ad5cc0(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        if destination != 0 {
            fn_00ad3150(e, Ptr::new(destination), value);
        }
        count -= 1;
        destination = destination.wrapping_add(0x18);
    }
}

/// The forwarders below pass their four words on with two tag words: a zero and a byte the
/// exe copies from uninitialised stack, which the callees do not read.
fn forward_range(
    e: &mut Engine,
    target: u32,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
) -> u32 {
    e.call(
        target,
        &args![first, last, destination, allocator, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00ad5dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` of strings (cdecl, four words: first, last, destination,
/// allocator): forwards to `00ad66e0` and returns its `EAX`.
pub fn fn_00ad5dd0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    forward_range(
        e,
        COPY_CONSTRUCT_STRINGS_00AD66E0,
        first,
        last,
        destination,
        allocator,
    )
}

// Translated from 00ad5ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the copy of a range of string pairs (cdecl, six words: first, last, destination
/// and three it does not read): assigns both strings of each pair (`0044a580(source, 0,
/// npos)`) in order; returns the destination's end.
pub fn fn_00ad5ec0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let end = ((last.wrapping_sub(first) as i32 / 0x38) as u32)
        .wrapping_mul(0x38)
        .wrapping_add(destination);
    let mut to = destination;
    let mut from = first;
    while from != last {
        let npos = e.global::<u32>(STRING_NPOS_010173F0);
        e.call(STRING_ASSIGN_0044A580, &args![to, from, 0u32, npos]);
        let npos = e.global::<u32>(STRING_NPOS_010173F0);
        e.call(
            STRING_ASSIGN_0044A580,
            &args![to + 0x1c, from + 0x1c, 0u32, npos],
        );
        to = to.wrapping_add(0x38);
        from = from.wrapping_add(0x38);
    }
    end
}

// Translated from 00ad5f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` of string pairs (cdecl, four words): forwards to
/// `00ad6d30` and returns its `EAX`.
pub fn fn_00ad5f80(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    forward_range(
        e,
        COPY_CONSTRUCT_PAIRS_00AD6D30,
        first,
        last,
        destination,
        allocator,
    )
}

// Translated from 00ad6000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Copy_backward` of string pairs (cdecl, six words: first, last, destination end,
/// a tag byte and two words it does not read): forwards to `00ad6800` with the tag byte
/// (zero-extended), a byte of its own uninitialised stack (0 here; `00ad6800` reads neither)
/// and 0; returns its `EAX`.
pub fn fn_00ad6000(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    tag: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    e.call(
        COPY_PAIRS_BACKWARD_00AD6800,
        &args![first, last, destination, tag & 0xff, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00ad6040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` of string vectors (cdecl, four words): forwards to
/// `00ad6e20` and returns its `EAX`.
pub fn fn_00ad6040(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    forward_range(
        e,
        COPY_CONSTRUCT_VECTORS_00AD6E20,
        first,
        last,
        destination,
        allocator,
    )
}

// Translated from 00ad60c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Copy_backward` of string vectors (cdecl, six words: first, last, destination
/// end and three it does not read): assigns each (`00ad61e0`) from the back; returns the
/// destination's start.
pub fn fn_00ad60c0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start = destination
        .wrapping_sub(((last.wrapping_sub(first) as i32 / 0x18) as u32).wrapping_mul(0x18));
    let mut to = destination;
    let mut from = last;
    while first != from {
        from = from.wrapping_sub(0x18);
        to = to.wrapping_sub(0x18);
        e.call(ASSIGN_STRING_VECTOR_00AD61E0, &args![to, from]);
    }
    start
}

// Translated from 00ad6190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a pair of strings' assignment (thiscall, one word: the source pair, `RET 4`):
/// assigns both strings (`0044a580(source, 0, npos)`). Returns `this`.
pub fn fn_00ad6190(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let base = this.addr();
    let npos = e.global::<u32>(STRING_NPOS_010173F0);
    e.call(STRING_ASSIGN_0044A580, &args![base, source, 0u32, npos]);
    let npos = e.global::<u32>(STRING_NPOS_010173F0);
    e.call(
        STRING_ASSIGN_0044A580,
        &args![base + 0x1c, source.wrapping_add(0x1c), 0u32, npos],
    );
    this
}

// Translated from 00ad6a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ad6a30 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed: a pair of strings' scalar deleting destructor (thiscall, one word, `RET 4`):
/// destroys the strings at `+0x1c` and `+0` (`0044a730(1, 0)`) and with flag bit 0 frees
/// `this`. Returns `this`.
pub fn fn_00ad6a30(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    e.call(STRING_TIDY_0044A730, &args![base + 0x1c, 1u32, 0u32]);
    e.call(STRING_TIDY_0044A730, &args![base, 1u32, 0u32]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![base]);
    }
    this
}

// Translated from 00ad6ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` of bytes (cdecl, four words: first, last, destination and
/// the allocator it does not read): `memmove_s` of the length when it is not 0; returns
/// `destination + length`.
pub fn fn_00ad6ab0(e: &mut Engine, first: u32, last: u32, destination: u32, _unused_3: u32) -> u32 {
    let length = last.wrapping_sub(first);
    let end = destination.wrapping_add(length);
    if length > 0 {
        e.call(MEMMOVE_S, &args![destination, length, first, length]);
    }
    end
}

// Translated from 00ad6b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` of string pairs (cdecl, four words): forwards to
/// `00ad6d30` and returns its `EAX`.
pub fn fn_00ad6b10(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    forward_range(
        e,
        COPY_CONSTRUCT_PAIRS_00AD6D30,
        first,
        last,
        destination,
        allocator,
    )
}

// Translated from 00ad6b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` of string vectors (cdecl, four words): forwards to
/// `00ad6e20` and returns its `EAX`.
pub fn fn_00ad6b60(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    forward_range(
        e,
        COPY_CONSTRUCT_VECTORS_00AD6E20,
        first,
        last,
        destination,
        allocator,
    )
}

/// `futSetLastError` (`00ac6430`, cdecl: the error code and a string by value).
const FUT_SET_LAST_ERROR: u32 = 0x00ac_6430;
/// `00ac6100` (thiscall, four words: offset, count, text, text length): the string's
/// `compare`. `00ac6e10` (cdecl: the path string): whether the file opens for reading.
const STRING_COMPARE_00AC6100: u32 = 0x00ac_6100;
const FILE_EXISTS_00AC6E10: u32 = 0x00ac_6e10;
/// The texts the parsers use (`.rdata`): `""`, `"//"`, `"\""`, `","` and the messages
/// `"Input buffer is empty."`, `"Unable to create string stream for buffer"`, `"Unable to
/// parse string stream."`, `"string stream buffer"` and `"CSV input is empty."`.
const EMPTY_CHARS_01011584: u32 = 0x0101_1584;
const COMMENT_CHARS_010A2FE0: u32 = 0x010a_2fe0;
const QUOTE_CHARS_01021184: u32 = 0x0102_1184;
const COMMA_CHARS_01063CD8: u32 = 0x0106_3cd8;
const MESSAGE_BUFFER_EMPTY_010A3010: u32 = 0x010a_3010;
const MESSAGE_NO_STRING_STREAM_010A2FE4: u32 = 0x010a_2fe4;
const MESSAGE_PARSE_FAILED_010A2FC0: u32 = 0x010a_2fc0;
const MESSAGE_STRING_STREAM_BUFFER_010A2FA8: u32 = 0x010a_2fa8;
const MESSAGE_CSV_EMPTY_010A3028: u32 = 0x010a_3028;

/// `futSetLastError(code, <the text as a string>)`: the string built from the text
/// (`0044a4c0`) in the argument area.
fn set_last_error_text(e: &mut Engine, code: u32, text: u32) {
    let message = e.with_stack(0x1c, |e, block| {
        e.call(STRING_FROM_C_0044A4C0, &args![block, text]);
        StringArg::load(e, block.addr())
    });
    e.call(FUT_SET_LAST_ERROR, &args![code, message]);
}

/// `futSetLastError(code, <copy of the string at source>)`, the copy made empty and assigned
/// (`0044a730(0, 0)`, `0044a580`).
fn set_last_error_copy(e: &mut Engine, code: u32, source: u32) {
    let message = string_copy_arg(e, source);
    e.call(FUT_SET_LAST_ERROR, &args![code, message]);
}

/// `futSetLastError(code, <copy of the string at source>)`, the copy made with the copy
/// constructor (`0044a500`).
fn set_last_error_copy_constructed(e: &mut Engine, code: u32, source: u32) {
    let message = e.with_stack(0x1c, |e, block| {
        e.call(STRING_COPY_0044A500, &args![block, source]);
        StringArg::load(e, block.addr())
    });
    e.call(FUT_SET_LAST_ERROR, &args![code, message]);
}

/// The stream state word (`base + 8`).
fn stream_state(e: &Engine, stream: u32) -> u32 {
    e.mem.u32(stream_base(e, stream) + 8)
}

/// The key/value parser both `FutPairParser` overloads share: reads keys ([`fn_00ad43a0`])
/// while the stream is good and fewer than `maximum` pairs were taken (no limit for 0), skips
/// blanks and tabs after the key ([`fn_00aca7a0`] peeks, [`fn_00aca560`] takes; a newline
/// stops the skipping), reads the rest of the line as the value ([`fn_00ad5770`] with the
/// stream's widened newline, [`fn_00ad30c0`]). A key equal to the terminator ([`fn_00ac9da0`])
/// is skipped unless the terminator is empty (`00ac6100`); every other pair goes to `insert`
/// and counts. Returns 0 when the stream ended with `failbit`/`badbit` but not `eofbit`,
/// else 1.
fn fut_pair_parse(
    e: &mut Engine,
    stream: u32,
    container: u32,
    terminator: u32,
    maximum: u32,
    insert: fn(&mut Engine, u32, u32, u32),
) -> u8 {
    e.with_stack(0x38, |e, frame| {
        let key = frame.addr();
        let value = frame.addr() + 0x1c;
        e.call(STRING_TIDY_0044A730, &args![key, 0u32, 0u32]);
        let mut count = 0i32;
        loop {
            if stream_state(e, stream) & 1 != 0
                || stream_state(e, stream) & 6 != 0
                || (maximum != 0 && count >= maximum as i32)
            {
                break;
            }
            fn_00ad43a0(e, stream, key);
            if stream_state(e, stream) & 1 != 0 || stream_state(e, stream) & 6 != 0 {
                break;
            }
            loop {
                if stream_state(e, stream) & 1 != 0 || stream_state(e, stream) & 6 != 0 {
                    break;
                }
                let next = fn_00aca7a0(e, Ptr::new(stream)) as u8 as i8;
                if next == 0xa
                    || stream_state(e, stream) & 1 != 0
                    || stream_state(e, stream) & 6 != 0
                {
                    break;
                }
                if next == 0x20 || next == 9 {
                    fn_00aca560(e, Ptr::new(stream));
                } else {
                    break;
                }
            }
            e.call(STRING_FROM_C_0044A4C0, &args![value, EMPTY_CHARS_01011584]);
            if stream_state(e, stream) & 1 == 0 && stream_state(e, stream) & 6 == 0 {
                let ios = stream_base(e, stream);
                let newline = fn_00ad30c0(e, Ptr::new(ios), 0xa) & 0xff;
                fn_00ad5770(e, stream, value, newline);
            }
            if fn_00ac9da0(e, Ptr::new(key), Ptr::new(terminator)) == 0 {
                let length = e.call(CRT_STRLEN, &args![EMPTY_CHARS_01011584]).u32();
                let own = e.mem.u32(terminator + 0x14);
                let differs = e
                    .call(
                        STRING_COMPARE_00AC6100,
                        &args![terminator, 0u32, own, EMPTY_CHARS_01011584, length],
                    )
                    .u32();
                if differs != 0 {
                    e.call(STRING_TIDY_0044A730, &args![value, 1u32, 0u32]);
                    continue;
                }
            }
            insert(e, container, key, value);
            count = count.wrapping_add(1);
            e.call(STRING_TIDY_0044A730, &args![value, 1u32, 0u32]);
        }
        let state = stream_state(e, stream);
        let result = if state & 6 != 0 && state & 1 == 0 {
            0
        } else {
            1
        };
        e.call(STRING_TIDY_0044A730, &args![key, 1u32, 0u32]);
        result
    })
}

/// [`fut_pair_parser`]'s insertion: a pair of copies of the key and value (`0044a500`), a
/// copy of that pair, inserted into the map ([`fn_00acaf50`]); the four strings destroyed,
/// the second pair's first.
fn insert_into_map(e: &mut Engine, map: u32, key: u32, value: u32) {
    e.with_stack(0x7c, |e, frame| {
        // Frame: the insert result at +0 (9 bytes), the first pair at +0xc, the second at
        // +0x44.
        let result = frame.addr();
        let first = frame.addr() + 0xc;
        let second = frame.addr() + 0x44;
        e.call(STRING_COPY_0044A500, &args![first, key]);
        e.call(STRING_COPY_0044A500, &args![first + 0x1c, value]);
        e.call(STRING_COPY_0044A500, &args![second, first]);
        e.call(STRING_COPY_0044A500, &args![second + 0x1c, first + 0x1c]);
        fn_00acaf50(e, Ptr::new(map), Ptr::new(result), second);
        for string in [second + 0x1c, second, first + 0x1c, first] {
            e.call(STRING_TIDY_0044A730, &args![string, 1u32, 0u32]);
        }
    });
}

/// [`fut_pair_parser_ov2`]'s insertion: a pair built from the key and value
/// ([`fn_00acb310`]) appended to the vector ([`fn_00acb3c0`]); its two strings destroyed.
fn insert_into_vector(e: &mut Engine, vector: u32, key: u32, value: u32) {
    e.with_stack(0x38, |e, pair| {
        let pair = fn_00acb310(e, pair, key, value);
        fn_00acb3c0(e, Ptr::new(vector), pair.addr());
        e.call(STRING_TIDY_0044A730, &args![pair.addr() + 0x1c, 1u32, 0u32]);
        e.call(STRING_TIDY_0044A730, &args![pair, 1u32, 0u32]);
    });
}

// Translated from 00ac7aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac7aa0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and the /GS cookie check (00ec408c) are not translated
/// `FutPairParser` (Xbox PDB name, by the map's call-graph match; cdecl: the input stream,
/// the string-keyed map, the terminator string and the maximum count; returns `AL`): reads
/// key/value lines into the map (see [`fut_pair_parse`] and [`insert_into_map`]).
pub fn fut_pair_parser(e: &mut Engine, stream: u32, map: u32, terminator: u32, maximum: u32) -> u8 {
    fut_pair_parse(e, stream, map, terminator, maximum, insert_into_map)
}

// Translated from 00ac7f50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac7f50 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and the /GS cookie check (00ec408c) are not translated
/// `FutPairParser_ov2` (Xbox PDB name, by the map's call-graph match; cdecl: the input
/// stream, the vector of string pairs, the terminator string and the maximum count; returns
/// `AL`): as [`fut_pair_parser`], appending each pair to the vector
/// ([`insert_into_vector`]).
pub fn fut_pair_parser_ov2(
    e: &mut Engine,
    stream: u32,
    vector: u32,
    terminator: u32,
    maximum: u32,
) -> u8 {
    fut_pair_parse(e, stream, vector, terminator, maximum, insert_into_vector)
}

/// Runs `parse` (one of the pair parsers) on `stream` with a `"//"` terminator string built
/// for the call (`0044a4c0`, destroyed after it) and the maximum count; answers whether it
/// failed.
fn parse_with_comment_terminator(
    e: &mut Engine,
    stream: u32,
    container: u32,
    maximum: u32,
    parse: fn(&mut Engine, u32, u32, u32, u32) -> u8,
) -> bool {
    e.with_stack(0x1c, |e, terminator| {
        e.call(
            STRING_FROM_C_0044A4C0,
            &args![terminator, COMMENT_CHARS_010A2FE0],
        );
        let failed = parse(e, stream, container, terminator.addr(), maximum) == 0;
        e.call(STRING_TIDY_0044A730, &args![terminator, 1u32, 0u32]);
        failed
    })
}

/// Whether the string-keyed map lacks `key`: a checked end iterator ([`fn_00ad1a50`] on the
/// head) and `find` ([`fn_00acb200`]); iterators of different trees call
/// `_invalid_parameter_noinfo`.
fn map_lacks(e: &mut Engine, map: u32, key: u32) -> bool {
    e.with_stack(0x10, |e, frame| {
        let end = frame.addr();
        let found = frame.addr() + 8;
        let head = e.mem.u32(map + 0x18);
        fn_00ad1a50(e, Ptr::new(end), head, Ptr::new(map));
        let found = fn_00acb200(e, Ptr::new(map), Ptr::new(found), key).addr();
        let proxy = e.mem.u32(found);
        if proxy == 0 || proxy != e.mem.u32(end) {
            e.call(INVALID_PARAMETER, &args![]);
        }
        e.mem.u32(found + 4) == e.mem.u32(end + 4)
    })
}

/// Destroys the input string stream whose `basic_ios` is at `ios` ([`fn_00acb610`] and the
/// `basic_ios` destructor).
fn destroy_istringstream(e: &mut Engine, ios: u32) {
    fn_00acb610(e, Ptr::new(ios));
    basic_ios_destructor(e, Ptr::new(ios));
}

/// Destroys the input file stream whose `basic_ios` is at `ios` ([`fn_00acc8c0`] and the
/// `basic_ios` destructor).
fn destroy_ifstream(e: &mut Engine, ios: u32) {
    fn_00acc8c0(e, Ptr::new(ios));
    basic_ios_destructor(e, Ptr::new(ios));
}

// Translated from 00ac8390 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac8390 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `FutBufferPairParser` (Xbox PDB name, by the map's string match; cdecl: the text buffer,
/// the header key string and the string-keyed map; returns `AL`). A null buffer is error 0xb
/// ("Input buffer is empty."). Otherwise it reads the buffer through an input string stream
/// ([`fn_00acb4d0`] on a string made from it; the string destroyed after): a stream that
/// failed is error 0xb. It clears the map; with a non-empty header key (`00ac6100`) it first
/// parses one pair ([`fut_pair_parser`], terminator `"//"`, maximum 1; failure: error 0xb,
/// "Unable to parse string stream.") and requires the header key in the map (error 2,
/// "string stream buffer"). Then it clears the map again and parses everything (no maximum;
/// failure: error 4). The stream is destroyed on every way out. Returns 1 on success.
pub fn fut_buffer_pair_parser(e: &mut Engine, buffer: u32, header: u32, map: u32) -> u8 {
    if buffer == 0 {
        set_last_error_text(e, 0xb, MESSAGE_BUFFER_EMPTY_010A3010);
        return 0;
    }
    e.with_stack(0xa8, |e, frame| {
        // Frame: the buffer string at +0, the input string stream at +0x1c (its `basic_ios`
        // at +0x50 within it).
        let text = frame.addr();
        let stream = frame.addr() + 0x1c;
        let ios = stream + 0x50;
        e.call(STRING_FROM_C_0044A4C0, &args![text, buffer]);
        fn_00acb4d0(e, Ptr::new(stream), text, 1, 1);
        e.call(STRING_TIDY_0044A730, &args![text, 1u32, 0u32]);
        if stream_state(e, stream) & 6 != 0 {
            set_last_error_text(e, 0xb, MESSAGE_NO_STRING_STREAM_010A2FE4);
            destroy_istringstream(e, ios);
            return 0;
        }
        fn_00acb1a0(e, Ptr::new(map));
        let length = e.call(CRT_STRLEN, &args![EMPTY_CHARS_01011584]).u32();
        let own = e.mem.u32(header + 0x14);
        let compared = e
            .call(
                STRING_COMPARE_00AC6100,
                &args![header, 0u32, own, EMPTY_CHARS_01011584, length],
            )
            .u32();
        if compared != 0 {
            if parse_with_comment_terminator(e, stream, map, 1, fut_pair_parser) {
                set_last_error_text(e, 0xb, MESSAGE_PARSE_FAILED_010A2FC0);
                destroy_istringstream(e, ios);
                return 0;
            }
            if map_lacks(e, map, header) {
                set_last_error_text(e, 2, MESSAGE_STRING_STREAM_BUFFER_010A2FA8);
                destroy_istringstream(e, ios);
                return 0;
            }
        }
        fn_00acb1a0(e, Ptr::new(map));
        if parse_with_comment_terminator(e, stream, map, 0, fut_pair_parser) {
            set_last_error_text(e, 4, MESSAGE_STRING_STREAM_BUFFER_010A2FA8);
            destroy_istringstream(e, ios);
            return 0;
        }
        destroy_istringstream(e, ios);
        1
    })
}

/// Opens `path` (a string) as an input file stream at `stream` ([`fn_00acc6c0`]: mode 1,
/// protection 0x40).
fn open_input_file(e: &mut Engine, stream: u32, path: u32) {
    let name = string_chars(e, path);
    fn_00acc6c0(e, Ptr::new(stream), name, 1, 0x40, 1);
}

// Translated from 00ac87b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac87b0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and the /GS cookie check (00ec408c) are not translated
/// Unnamed (cdecl: the file path string, the header key string and the string-keyed map;
/// returns `AL`): the file counterpart of [`fut_buffer_pair_parser`]. An unused string is
/// made and destroyed around it. A file that does not open is error 4; it clears the map,
/// parses one pair ([`fut_pair_parser`], `"//"`, maximum 1; failure: error 4), requires the
/// header key (error 2), clears the map and parses everything (failure: error 4, its message
/// copy-constructed). Every error message is a copy of the path. The stream is destroyed on
/// every way out. Returns 1 on success.
pub fn fn_00ac87b0(e: &mut Engine, path: u32, header: u32, map: u32) -> u8 {
    e.with_stack(0xb0, |e, frame| {
        // Frame: the unused string at +0, the input file stream at +0x1c (its `basic_ios` at
        // +0x58 within it).
        let unused = frame.addr();
        let stream = frame.addr() + 0x1c;
        let ios = stream + 0x58;
        e.call(STRING_TIDY_0044A730, &args![unused, 0u32, 0u32]);
        open_input_file(e, stream, path);
        let result = 'parse: {
            if stream_state(e, stream) & 6 != 0 {
                set_last_error_copy(e, 4, path);
                break 'parse 0;
            }
            fn_00acb1a0(e, Ptr::new(map));
            if parse_with_comment_terminator(e, stream, map, 1, fut_pair_parser) {
                set_last_error_copy(e, 4, path);
                break 'parse 0;
            }
            if map_lacks(e, map, header) {
                set_last_error_copy(e, 2, path);
                break 'parse 0;
            }
            fn_00acb1a0(e, Ptr::new(map));
            if parse_with_comment_terminator(e, stream, map, 0, fut_pair_parser) {
                set_last_error_copy_constructed(e, 4, path);
                break 'parse 0;
            }
            1
        };
        destroy_ifstream(e, ios);
        e.call(STRING_TIDY_0044A730, &args![unused, 1u32, 0u32]);
        result
    })
}

/// Empties the vector of string pairs: `resize(0, <a default pair>)` ([`fn_00acf500`] builds
/// the pair in the argument area, [`fn_00acf580`] destroys it).
fn clear_pair_vector(e: &mut Engine, vector: u32) {
    e.with_stack(0x38, |e, pair| {
        fn_00acf500(e, pair);
        fn_00acf580(e, Ptr::new(vector), 0, pair.addr());
    });
}

// Translated from 00ac8c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac8c50 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and the /GS cookie check (00ec408c) are not translated
/// Unnamed (cdecl: the file path string, the header key string and the vector of string
/// pairs; returns `AL`): as [`fn_00ac87b0`] with the vector parser
/// ([`fut_pair_parser_ov2`]); the vector is emptied with [`clear_pair_vector`], and the
/// header must be the first pair's key ([`fn_00ac9da0`]; an empty vector calls
/// `_invalid_parameter_noinfo` first), else error 2. Every error message is a copy of the
/// path. Returns 1 on success.
pub fn fn_00ac8c50(e: &mut Engine, path: u32, header: u32, vector: u32) -> u8 {
    e.with_stack(0xb0, |e, frame| {
        let unused = frame.addr();
        let stream = frame.addr() + 0x1c;
        let ios = stream + 0x58;
        e.call(STRING_TIDY_0044A730, &args![unused, 0u32, 0u32]);
        open_input_file(e, stream, path);
        let result = 'parse: {
            if stream_state(e, stream) & 6 != 0 {
                set_last_error_copy(e, 4, path);
                break 'parse 0;
            }
            clear_pair_vector(e, vector);
            if parse_with_comment_terminator(e, stream, vector, 1, fut_pair_parser_ov2) {
                set_last_error_copy(e, 4, path);
                break 'parse 0;
            }
            if vector_count(e, vector, 0x10, 0x38) == 0 {
                e.call(INVALID_PARAMETER, &args![]);
            }
            let first = e.mem.u32(vector + 0xc);
            if fn_00ac9da0(e, Ptr::new(first), Ptr::new(header)) != 0 {
                set_last_error_copy(e, 2, path);
                break 'parse 0;
            }
            clear_pair_vector(e, vector);
            if parse_with_comment_terminator(e, stream, vector, 0, fut_pair_parser_ov2) {
                set_last_error_copy(e, 4, path);
                break 'parse 0;
            }
            1
        };
        destroy_ifstream(e, ios);
        e.call(STRING_TIDY_0044A730, &args![unused, 1u32, 0u32]);
        result
    })
}

/// Splits one CSV line (characters at `cursor`) into the string vector `row` (emptied first,
/// [`fn_00acac40`]): fields end at a comma outside quotes, a NUL or a newline; quotes toggle
/// the quoting and are dropped (a leading one starts quoted); every other character is
/// appended (`00a0c430` with a one-character text at `character`, `strlen`). Each field is
/// appended to the row ([`fn_00acab30`]). Answers whether every character was a blank or a
/// tab.
fn split_csv_line(e: &mut Engine, row: u32, line: u32, character: u32) -> bool {
    e.mem.set_u8(character, 0);
    e.mem.set_u8(character + 1, 0);
    let mut cursor = string_chars(e, line);
    let mut blank = true;
    fn_00acac40(e, Ptr::new(row));
    e.with_stack(0x1c, |e, field| {
        let field = field.addr();
        while e.mem.u8(cursor) as i8 != 0xa && e.mem.u8(cursor) != 0 {
            e.call(STRING_FROM_C_0044A4C0, &args![field, EMPTY_CHARS_01011584]);
            let mut quoted = false;
            if e.mem.u8(cursor) == b'"' {
                quoted = true;
                cursor = cursor.wrapping_add(1);
            }
            loop {
                let next = e.mem.u8(cursor);
                if (next == b',' && !quoted) || next == 0 || next == 0xa {
                    break;
                }
                if next == b'"' {
                    quoted = !quoted;
                } else {
                    if next != b' ' && next != 9 {
                        blank = false;
                    }
                    e.mem.set_u8(character, next);
                    let length = e.call(CRT_STRLEN, &args![character]).u32();
                    e.call(
                        STRING_APPEND_CHARS_00A0C430,
                        &args![field, character, length],
                    );
                }
                cursor = cursor.wrapping_add(1);
            }
            if e.mem.u8(cursor) != 0 {
                cursor = cursor.wrapping_add(1);
            }
            fn_00acab30(e, Ptr::new(row), field);
            e.call(STRING_TIDY_0044A730, &args![field, 1u32, 0u32]);
        }
    });
    blank
}

// Translated from 00ac9120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac9120 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and the /GS cookie check (00ec408c) are not translated
/// Unnamed: the CSV reader (cdecl: the file path string and the table, a vector of string
/// vectors; returns `AL`). Empties the table ([`fn_00acdba0`]) and makes a row vector
/// ([`fn_00acaa50`]). A missing file (`00ac6e10`) is error 3, one that does not open error
/// 4. While the stream is good it reads lines ([`fn_00ad5770`] with the widened newline);
/// each line read without failure is split ([`split_csv_line`]) and the row appended to the
/// table ([`fn_00acda90`]) unless it was all blanks. A stream that ended with
/// `failbit`/`badbit` but not `eofbit` is error 1. The stream and the row (emptied with
/// [`fn_00acede0`], its proxy freed with `00702220`) are destroyed on every way out. Every
/// error message is a copy of the path. Returns 1 on success.
pub fn fn_00ac9120(e: &mut Engine, path: u32, table: u32) -> u8 {
    fn_00acdba0(e, Ptr::new(table));
    e.with_stack(0xd0, |e, frame| {
        // Frame: the row at +0 (0x18 bytes), the line at +0x18, the one-character text at
        // +0x34, the input file stream at +0x38 (its `basic_ios` at +0x58 within it).
        let row = frame.addr();
        let line = frame.addr() + 0x18;
        let character = frame.addr() + 0x34;
        let stream = frame.addr() + 0x38;
        let ios = stream + 0x58;
        fn_00acaa50(e, Ptr::new(row));
        let destroy_row = |e: &mut Engine| {
            fn_00acede0(e, Ptr::new(row));
            e.call(FREE_PROXY_00702220, &args![row]);
        };
        if !e.call(FILE_EXISTS_00AC6E10, &args![path]).bool() {
            set_last_error_copy(e, 3, path);
            destroy_row(e);
            return 0;
        }
        open_input_file(e, stream, path);
        if stream_state(e, stream) & 6 != 0 {
            set_last_error_copy(e, 4, path);
            destroy_ifstream(e, ios);
            destroy_row(e);
            return 0;
        }
        while stream_state(e, stream) & 1 == 0 && stream_state(e, stream) & 6 == 0 {
            e.call(STRING_TIDY_0044A730, &args![line, 0u32, 0u32]);
            let base = stream_base(e, stream);
            let newline = fn_00ad30c0(e, Ptr::new(base), 0xa) & 0xff;
            fn_00ad5770(e, stream, line, newline);
            if stream_state(e, stream) & 6 == 0 && !split_csv_line(e, row, line, character) {
                fn_00acda90(e, Ptr::new(table), row);
            }
            e.call(STRING_TIDY_0044A730, &args![line, 1u32, 0u32]);
        }
        if stream_state(e, stream) & 1 == 0 && stream_state(e, stream) & 6 != 0 {
            set_last_error_copy(e, 1, path);
            destroy_ifstream(e, ios);
            destroy_row(e);
            return 0;
        }
        fn_00acc8c0(e, Ptr::new(ios));
        destroy_ios(e, ios);
        destroy_row(e);
        1
    })
}

/// Destroys the output file stream whose `basic_ios` is at `ios` ([`fn_00acdeb0`] and the
/// `basic_ios` destructor).
fn destroy_ofstream(e: &mut Engine, ios: u32) {
    fn_00acdeb0(e, Ptr::new(ios));
    basic_ios_destructor(e, Ptr::new(ios));
}

/// Closes the output file stream's buffer (`+4`, [`fn_00acfb90`]); a failed close sets
/// `failbit` ([`fn_00aca4e0`]).
fn close_output_file(e: &mut Engine, stream: u32) {
    if fn_00acfb90(e, Ptr::new(stream + 4)).addr() == 0 {
        let base = stream_base(e, stream);
        fn_00aca4e0(e, Ptr::new(base), 2, 0);
    }
}

// Translated from 00ac9790 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac9790 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `futSaveCsvFile` (Xbox PDB name, by the map's string match; cdecl: the file path string
/// and the table, a vector of string vectors; returns `AL`). An empty table is error 0xb
/// ("CSV input is empty."); a file that does not open for writing ([`fn_00acdc90`], mode 2,
/// protection 0x40) error 5 with a copy of the path. Each cell is written quoted
/// ([`fn_00ad47f0`] `"\""`, [`fn_00ad4ba0`], `"\""`), followed by `","`
/// ([`fn_00ad47f0`]) or, after the row's last cell, a newline ([`fn_00ac9df0`]) and two
/// flushes ([`fn_00aca3c0`]); the indices are checked against the sizes
/// (`_invalid_parameter_noinfo`). A stream that failed after a row is error 6 with a copy of
/// the path, closed ([`close_output_file`]) and destroyed; otherwise the file is closed and
/// the stream destroyed and the result is 1.
pub fn fut_save_csv_file(e: &mut Engine, path: u32, table: u32) -> u8 {
    if vector_count(e, table, 0x10, 0x18) == 0 {
        set_last_error_text(e, 0xb, MESSAGE_CSV_EMPTY_010A3028);
        return 0;
    }
    e.with_stack(0x90, |e, stream| {
        // The output file stream (its `basic_ios` at +0x54).
        let stream = stream.addr();
        let ios = stream + 0x54;
        let name = string_chars(e, path);
        fn_00acdc90(e, Ptr::new(stream), name, 2, 0x40, 1);
        if stream_state(e, stream) & 6 != 0 {
            set_last_error_copy(e, 5, path);
            destroy_ofstream(e, ios);
            return 0;
        }
        let mut row = 0u32;
        while row < vector_count(e, table, 0x10, 0x18) {
            if row >= vector_count(e, table, 0x10, 0x18) {
                e.call(INVALID_PARAMETER, &args![]);
            }
            let cells = row.wrapping_mul(0x18).wrapping_add(e.mem.u32(table + 0xc));
            let count = vector_count(e, cells, 0x10, 0x1c);
            let mut column = 0u32;
            while column < count {
                if row >= vector_count(e, table, 0x10, 0x18) {
                    e.call(INVALID_PARAMETER, &args![]);
                }
                let cells = row.wrapping_mul(0x18).wrapping_add(e.mem.u32(table + 0xc));
                if column >= vector_count(e, cells, 0x10, 0x1c) {
                    e.call(INVALID_PARAMETER, &args![]);
                }
                let cell = column
                    .wrapping_mul(0x1c)
                    .wrapping_add(e.mem.u32(cells + 0xc));
                let out = fn_00ad47f0(e, stream, QUOTE_CHARS_01021184);
                let out = fn_00ad4ba0(e, out, cell);
                fn_00ad47f0(e, out, QUOTE_CHARS_01021184);
                if column == count.wrapping_sub(1) {
                    fn_00ac9df0(e, Ptr::new(stream), 0xa);
                    fn_00aca3c0(e, Ptr::new(stream));
                    fn_00aca3c0(e, Ptr::new(stream));
                } else {
                    fn_00ad47f0(e, stream, COMMA_CHARS_01063CD8);
                }
                column += 1;
            }
            if stream_state(e, stream) & 6 != 0 {
                set_last_error_copy(e, 6, path);
                close_output_file(e, stream);
                destroy_ofstream(e, ios);
                return 0;
            }
            row += 1;
        }
        close_output_file(e, stream);
        destroy_ofstream(e, ios);
        1
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00acbb70, fn_00acbb70(Ptr) -> u32),
        entry!(0x00ac_9da0, fn_00ac9da0(Ptr, Ptr) -> u32),
        entry!(0x00ac_9ff0, ios_base_clear(Ptr, u32, u32)),
        entry!(0x00ac_b6b0, fn_00acb6b0(Ptr)),
        entry!(0x00ac_b720, fn_00acb720(Ptr)),
        entry!(0x00ac_b770, facet_decref(Ptr) -> Ptr),
        entry!(0x00ac_b7e0, fn_00acb7e0(Ptr) -> u32),
        entry!(0x00ac_c970, fn_00acc970(Ptr)),
        entry!(0x00ac_e2d0, fn_00ace2d0(Ptr)),
        entry!(0x00ac_e750, fn_00ace750(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_e790, facet_incref(Ptr)),
        entry!(0x00ac_fc10, fn_00acfc10(Ptr, u32, u32)),
        entry!(0x00ad_0470, fn_00ad0470(Ptr, u32, u32)),
        entry!(0x00ad_0550, fn_00ad0550(Ptr)),
        entry!(0x00ad_0630, fn_00ad0630(Ptr) -> Ptr),
        entry!(0x00ad_0720, fn_00ad0720(Ptr)),
        entry!(0x00ad_30c0, fn_00ad30c0(Ptr, u32) -> u32),
        entry!(0x00ad_33c0, fn_00ad33c0(Ptr, u32) -> Ptr),
        entry!(0x00ad_38d0, fn_00ad38d0(Ptr) -> Ptr),
        entry!(0x00ad_3a10, locale_getfacet(Ptr, u32) -> u32),
        entry!(0x00ad_3aa0, fn_00ad3aa0(Ptr, Ptr) -> u32),
        entry!(0x00ad_3ca0, fn_00ad3ca0(Ptr, u32) -> Ptr),
        entry!(0x00ad_3db0, fn_00ad3db0(Ptr)),
        entry!(0x00ad_3e60, fn_00ad3e60(Ptr, Ptr) -> Ptr),
        entry!(0x00ad_4000, fn_00ad4000(Ptr, u32) -> u8),
        entry!(0x00ad_4110, fn_00ad4110(Ptr, u32) -> Ptr),
        entry!(0x00ad_41c0, ctype_tidy(Ptr)),
        entry!(0x00ac_79e0, fr2_add_saturated_float(f32, f32) -> f64),
        entry!(0x00ac_7aa0, fut_pair_parser(u32, u32, u32, u32) -> u8),
        entry!(0x00ac_7f50, fut_pair_parser_ov2(u32, u32, u32, u32) -> u8),
        entry!(0x00ac_8390, fut_buffer_pair_parser(u32, u32, u32) -> u8),
        entry!(0x00ac_87b0, fn_00ac87b0(u32, u32, u32) -> u8),
        entry!(0x00ac_8c50, fn_00ac8c50(u32, u32, u32) -> u8),
        entry!(0x00ac_9120, fn_00ac9120(u32, u32) -> u8),
        entry!(0x00ac_9790, fut_save_csv_file(u32, u32) -> u8),
        entry!(0x00ac_9bd0, fn_00ac9bd0(Ptr)),
        entry!(0x00ac_9c30, ios_base_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ac_9c70, fn_00ac9c70(Ptr)),
        entry!(0x00ac_9cb0, fn_00ac9cb0(Ptr)),
        entry!(0x00ac_9cf0, fn_00ac9cf0(Ptr)),
        entry!(0x00ac_9d60, fn_00ac9d60(Ptr)),
        entry!(0x00ac_9df0, fn_00ac9df0(Ptr, u32) -> Ptr),
        entry!(0x00ac_a1e0, fn_00aca1e0(Ptr) -> u32),
        entry!(0x00ac_a220, fn_00aca220(Ptr, u32) -> Ptr),
        entry!(0x00ac_a270, fn_00aca270(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_a300, fn_00aca300(Ptr)),
        entry!(0x00ac_a340, fn_00aca340(Ptr, u32) -> Ptr),
        entry!(0x00ac_a390, fn_00aca390(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_a3c0, fn_00aca3c0(Ptr) -> Ptr),
        entry!(0x00ac_a4b0, basic_ios_destructor(Ptr)),
        entry!(0x00ac_a4e0, fn_00aca4e0(Ptr, u32, u32)),
        entry!(0x00ac_a560, fn_00aca560(Ptr) -> u32),
        entry!(0x00ac_a780, fn_00aca780(Ptr)),
        entry!(0x00ac_a7a0, fn_00aca7a0(Ptr) -> u32),
        entry!(0x00ac_a9a0, fn_00aca9a0(Ptr)),
        entry!(0x00ac_a9f0, fn_00aca9f0(Ptr) -> u32),
        entry!(0x00ac_aa50, fn_00acaa50(Ptr) -> Ptr),
        entry!(0x00ac_ab30, fn_00acab30(Ptr, u32)),
        entry!(0x00ac_ac40, fn_00acac40(Ptr)),
        entry!(0x00ac_ad30, fn_00acad30(Ptr, u32) -> u32),
        entry!(0x00ac_ad60, fn_00acad60(Ptr, u32)),
        entry!(0x00ac_ae90, fn_00acae90(Ptr)),
        entry!(0x00ac_af50, fn_00acaf50(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ac_b1a0, fn_00acb1a0(Ptr)),
        entry!(0x00ac_b200, fn_00acb200(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ac_b310, fn_00acb310(Ptr, u32, u32) -> Ptr),
        entry!(0x00ac_b3c0, fn_00acb3c0(Ptr, u32)),
        entry!(0x00ac_b4d0, fn_00acb4d0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ac_b5f0, fn_00acb5f0(Ptr)),
        entry!(0x00ac_b610, fn_00acb610(Ptr)),
        entry!(0x00ac_b7d0, fn_00acb7d0(Ptr, u32) -> u32),
        entry!(0x00ac_b7f0, fn_00acb7f0(Ptr) -> u32),
        entry!(0x00ac_b850, fn_00acb850(Ptr, u32, u32) -> u32),
        entry!(0x00ac_b880, fn_00acb880(Ptr, u32, u32, u32) -> u32),
        entry!(0x00ac_b9b0, fn_00acb9b0(Ptr, u32, u32) -> u32),
        entry!(0x00ac_bae0, fn_00acbae0(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ac_bb20, fn_00acbb20(Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ac_bb60, fn_00acbb60(Ptr, u32, u32) -> Ptr),
        entry!(0x00ac_bb80, fn_00acbb80(Ptr, u32) -> u32),
        entry!(0x00ac_bf60, fn_00acbf60(Ptr, u32) -> u32),
        entry!(0x00ac_c070, fn_00acc070(Ptr) -> u32),
        entry!(0x00ac_c1b0, fn_00acc1b0(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ac_c490, fn_00acc490(Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ac_c6c0, fn_00acc6c0(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ac_c8c0, fn_00acc8c0(Ptr)),
        entry!(0x00ac_c9f0, fn_00acc9f0(Ptr, u32) -> u32),
        entry!(0x00ac_cef0, fn_00accef0(Ptr, u32) -> u32),
        entry!(0x00ac_d060, fn_00acd060(Ptr) -> u32),
        entry!(0x00ac_d100, fn_00acd100(Ptr) -> u32),
        entry!(0x00ac_d5b0, fn_00acd5b0(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ac_d760, fn_00acd760(Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ac_d930, basic_filebuf_setbuf(Ptr, u32, u32) -> Ptr),
        entry!(0x00ac_d9a0, fn_00acd9a0(Ptr) -> u32),
        entry!(0x00ac_da10, fn_00acda10(Ptr, u32)),
        entry!(0x00ac_da90, fn_00acda90(Ptr, u32)),
        entry!(0x00ac_dba0, fn_00acdba0(Ptr)),
        entry!(0x00ac_dc90, fn_00acdc90(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ac_de90, basic_ostream_destructor(Ptr)),
        entry!(0x00ac_deb0, fn_00acdeb0(Ptr)),
        entry!(0x00ac_df60, fn_00acdf60(Ptr, u32) -> Ptr),
        entry!(0x00ac_dfa0, basic_ios_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ac_dff0, fn_00acdff0(Ptr, u32) -> Ptr),
        entry!(0x00ac_e030, fn_00ace030(Ptr, u32) -> Ptr),
        entry!(0x00ac_e060, fn_00ace060(Ptr, u32) -> Ptr),
        entry!(0x00ac_e0d0, fn_00ace0d0(Ptr, u32) -> Ptr),
        entry!(0x00ac_e150, fn_00ace150(Ptr, u32) -> Ptr),
        entry!(0x00ac_e1c0, fn_00ace1c0(Ptr, u32) -> Ptr),
        entry!(0x00ac_e260, fn_00ace260(Ptr, u32) -> Ptr),
        entry!(0x00ac_e330, fn_00ace330(Ptr)),
        entry!(0x00ac_e390, fn_00ace390(Ptr, u32) -> u8),
        entry!(0x00ac_e7d0, fn_00ace7d0(Ptr) -> u32),
        entry!(0x00ac_e860, fn_00ace860(Ptr) -> u32),
        entry!(0x00ac_e8c0, fn_00ace8c0(Ptr, u32) -> u32),
        entry!(0x00ac_e930, basic_streambuf_gndec(Ptr) -> u32),
        entry!(0x00ac_e970, fn_00ace970(Ptr) -> u32),
        entry!(0x00ac_e9c0, fn_00ace9c0(Ptr) -> u32),
        entry!(0x00ac_ea00, basic_streambuf_pninc(Ptr) -> u32),
        entry!(0x00ac_ea50, fn_00acea50(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ac_eb80, fn_00aceb80(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ac_ed20, fn_00aced20(Ptr, u32) -> u8),
        entry!(0x00ac_ede0, fn_00acede0(Ptr)),
        entry!(0x00ac_eeb0, fn_00aceeb0(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ac_efd0, fn_00acefd0(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ac_f0b0, fn_00acf0b0(Ptr, u32)),
        entry!(0x00ac_f120, fn_00acf120(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ac_f500, fn_00acf500(Ptr) -> Ptr),
        (0x00ac_f580, fn_00acf580_entry as AbiFn),
        entry!(0x00ac_f7d0, fn_00acf7d0(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ac_f900, fn_00acf900(Ptr, u32, u32) -> Ptr),
        entry!(0x00ac_f9e0, fn_00acf9e0(Ptr)),
        entry!(0x00ac_fab0, fn_00acfab0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ac_fb90, fn_00acfb90(Ptr) -> Ptr),
        entry!(0x00ac_fcd0, fn_00acfcd0(Ptr) -> u8),
        entry!(0x00ac_ffe0, fn_00acffe0(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ad_0110, fn_00ad0110(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ad_0210, fn_00ad0210(Ptr, Ptr) -> Ptr),
        entry!(0x00ad_02f0, fn_00ad02f0(Ptr)),
        entry!(0x00ad_0360, fn_00ad0360(Ptr, u32, u32) -> Ptr),
        entry!(0x00ad_03e0, fn_00ad03e0(Ptr)),
        entry!(0x00ad_06e0, fn_00ad06e0(Ptr, Ptr) -> Ptr),
        entry!(0x00ad_07c0, fn_00ad07c0(Ptr, u32, u32, u32, u32)),
        entry!(0x00ad_10e0, fn_00ad10e0(Ptr, u32, u32, u32, u32)),
        entry!(0x00ad_1740, fn_00ad1740(Ptr, u32, u32, u32) -> u32),
        entry!(0x00ad_1790, fn_00ad1790(Ptr, u32) -> u32),
        entry!(0x00ad_1800, fn_00ad1800(Ptr, u32)),
        entry!(0x00ad_18b0, fn_00ad18b0(Ptr, u32)),
        entry!(0x00ad_1960, fn_00ad1960(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_1a50, fn_00ad1a50(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00ad_1a90, fn_00ad1a90(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ad_1c10, fn_00ad1c10(Ptr, u32, u32, u32, u32)),
        entry!(0x00ad_2500, fn_00ad2500(Ptr, u32, u32, u32)),
        entry!(0x00ad_2640, fn_00ad2640(Ptr, u32, u32)),
        entry!(0x00ad_26f0, fn_00ad26f0(Ptr, u32, u32, u32, u32)),
        entry!(0x00ad_2f30, fn_00ad2f30(Ptr) -> u32),
        entry!(0x00ad_2fa0, fn_00ad2fa0(Ptr, Ptr) -> Ptr),
        entry!(0x00ad_3000, fn_00ad3000(Ptr, u32) -> Ptr),
        entry!(0x00ad_3150, fn_00ad3150(Ptr, u32) -> Ptr),
        entry!(0x00ad_3450, fn_00ad3450(Ptr)),
        entry!(0x00ad_3550, fn_00ad3550(Ptr, u32, u32) -> Ptr),
        entry!(0x00ad_35e0, fn_00ad35e0(Ptr, u32) -> Ptr),
        entry!(0x00ad_36a0, fn_00ad36a0(Ptr, u32) -> Ptr),
        entry!(0x00ad_3750, fn_00ad3750(Ptr, u32) -> Ptr),
        entry!(0x00ad_3810, fn_00ad3810(Ptr, u32) -> Ptr),
        entry!(0x00ad_3eb0, locale_facet_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ad_3ee0, ctype_base_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ad_3f20, ctype_do_tolower(Ptr, u32) -> u32),
        entry!(0x00ad_3f50, ctype_do_tolower_range(Ptr, u32, u32) -> u32),
        entry!(0x00ad_3f90, ctype_do_toupper(Ptr, u32) -> u32),
        entry!(0x00ad_3fc0, ctype_do_toupper_range(Ptr, u32, u32) -> u32),
        entry!(0x00ad_4010, ctype_do_widen_range(Ptr, u32, u32, u32) -> u32),
        entry!(0x00ad_4040, fn_00ad4040(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_4080, fn_00ad4080(Ptr, u32, u32) -> u8),
        entry!(0x00ad_4090, ctype_do_narrow_range(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_40d0, fn_00ad40d0(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_41a0, fn_00ad41a0(Ptr)),
        entry!(0x00ad_4210, fn_00ad4210(u32, u32, u32) -> u32),
        entry!(0x00ad_42e0, fn_00ad42e0(u32, u32, u32) -> u32),
        entry!(0x00ad_43a0, fn_00ad43a0(u32, u32) -> u32),
        entry!(0x00ad_47f0, fn_00ad47f0(u32, u32) -> u32),
        entry!(0x00ad_4ba0, fn_00ad4ba0(u32, u32) -> u32),
        entry!(0x00ad_4fd0, fn_00ad4fd0(Ptr) -> Ptr),
        entry!(0x00ad_5110, fn_00ad5110(u32, u32, u32) -> u32),
        entry!(0x00ad_5180, fn_00ad5180(u32, u32, u32) -> u32),
        entry!(0x00ad_5250, fn_00ad5250(u32, u32, u32) -> u32),
        entry!(0x00ad_5310, fn_00ad5310(u32, u32) -> u32),
        entry!(0x00ad_5370, fn_00ad5370(u32, u32) -> u32),
        entry!(0x00ad_53d0, fn_00ad53d0(u32, u32) -> u32),
        entry!(0x00ad_5430, fn_00ad5430(u32, u32) -> u32),
        entry!(0x00ad_5490, fn_00ad5490(Ptr)),
        entry!(0x00ad_5500, fn_00ad5500(u32, u32) -> u32),
        entry!(0x00ad_5670, codecvt_base_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ad_56b0, codecvt_do_in(Ptr, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_56e0, codecvt_do_unshift(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_5700, codecvt_do_length(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_5730, codecvt_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ad_5770, fn_00ad5770(u32, u32, u32) -> u32),
        entry!(0x00ad_5b00, fn_00ad5b00(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ad_5bd0, fn_00ad5bd0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ad_5cc0, fn_00ad5cc0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ad_5dd0, fn_00ad5dd0(u32, u32, u32, u32) -> u32),
        entry!(0x00ad_5ec0, fn_00ad5ec0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_5f80, fn_00ad5f80(u32, u32, u32, u32) -> u32),
        entry!(0x00ad_6000, fn_00ad6000(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_6040, fn_00ad6040(u32, u32, u32, u32) -> u32),
        entry!(0x00ad_60c0, fn_00ad60c0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ad_6190, fn_00ad6190(Ptr, u32) -> Ptr),
        entry!(0x00ad_6a30, fn_00ad6a30(Ptr, u32) -> Ptr),
        entry!(0x00ad_6ab0, fn_00ad6ab0(u32, u32, u32, u32) -> u32),
        entry!(0x00ad_6b10, fn_00ad6b10(u32, u32, u32, u32) -> u32),
        entry!(0x00ad_6b60, fn_00ad6b60(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ac_bb70, &args![Ptr::<()>::new(0x10)]).u32(), 0);
    }
}

#[cfg(test)]
mod b046_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, quiet);
        }
    }

    #[test]
    fn b046_00ac9da0_passes_the_inline_or_heap_characters() {
        let mut e = Engine::new();
        e.register(0x00ac_6100, |_, _| 0x77u32.into_ret());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 0x5555);
        let string = e.mem.alloc(0x1c);
        e.mem.set_u32(string + 0x14, 3);
        e.mem.set_u32(string + 0x18, 0xf);
        e.call_log = Some(vec![]);
        let result = e.call(0x00ac_9da0, &args![this, string]).u32();
        assert_eq!(result, 0x77);
        assert_eq!(
            calls_to(&e, 0x00ac_6100),
            vec![vec![this, 0, 0x5555, string + 4, 3]]
        );
        e.mem.set_u32(string + 0x18, 0x10);
        e.mem.set_u32(string + 4, 0x9000);
        e.call_log = Some(vec![]);
        e.call(0x00ac_9da0, &args![this, string]);
        assert_eq!(
            calls_to(&e, 0x00ac_6100),
            vec![vec![this, 0, 0x5555, 0x9000, 3]]
        );
    }

    #[test]
    fn b046_00ac9ff0_clear_stores_the_state_and_throws_by_bit() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                STRING_FROM_C_0044A4C0,
                EXCEPTION_CTOR_00EC78E0,
                STRING_COPY_0044A500,
                CXX_THROW_EXCEPTION,
                STRING_TIDY_0044A730,
            ],
        );
        let this = e.mem.alloc(0x10);
        // No bit in common with the mask: nothing thrown.
        e.mem.set_u32(this + 0xc, 0x2);
        e.call_log = Some(vec![]);
        e.call(0x00ac_9ff0, &args![this, 0xffff_ff0du32, 0u32]);
        assert_eq!(e.mem.u32(this + 8), 0x05);
        assert!(calls_to(&e, CXX_THROW_EXCEPTION).is_empty());
        // Reraise: `_CxxThrowException(0, 0)` only.
        e.call_log = Some(vec![]);
        e.call(0x00ac_9ff0, &args![this, 2u32, 0x101u32]);
        assert_eq!(calls_to(&e, CXX_THROW_EXCEPTION), vec![vec![0, 0]]);
        assert!(calls_to(&e, STRING_FROM_C_0044A4C0).is_empty());
        // Only the low byte of the reraise word counts.
        for (state, mask, message) in [
            (4u32, 6u32, MESSAGE_BADBIT_010A3084),
            (6, 2, MESSAGE_FAILBIT_010A3054),
            (1, 1, MESSAGE_EOFBIT_010A303C),
        ] {
            e.mem.set_u32(this + 0xc, mask);
            e.call_log = Some(vec![]);
            e.call(0x00ac_9ff0, &args![this, state, 0x100u32]);
            let made = calls_to(&e, STRING_FROM_C_0044A4C0);
            assert_eq!(made.len(), 1);
            assert_eq!(made[0][1], message);
            let string = made[0][0];
            let object = string + 0x1c;
            assert_eq!(calls_to(&e, EXCEPTION_CTOR_00EC78E0), vec![vec![object]]);
            assert_eq!(
                calls_to(&e, STRING_COPY_0044A500),
                vec![vec![object + 0xc, string]]
            );
            assert_eq!(
                calls_to(&e, CXX_THROW_EXCEPTION),
                vec![vec![object, IOS_FAILURE_THROW_INFO_0116C3A4]]
            );
            assert_eq!(calls_to(&e, STRING_TIDY_0044A730), vec![vec![string, 1, 0]]);
        }
    }

    fn facet(e: &mut Engine, count: u32) -> u32 {
        let facet = e.mem.alloc(8);
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        e.mem.set_u32(facet, 0x0300_0000);
        e.mem.set_u32(facet + 4, count);
        facet
    }

    #[test]
    fn b046_00acb6b0_releases_the_locale_and_the_mutex() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[OPERATOR_DELETE, 0x00ec_3260, LOCKIT_CTOR, LOCKIT_DTOR],
        );
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00ac_b6b0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_STREAMBUF_010A30FC);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![this + 4]]);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(this + 0x38, holder);
        e.call_log = Some(vec![]);
        e.call(0x00ac_b6b0, &args![this]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![holder]]);
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![this + 4]]);
    }

    #[test]
    fn b046_00acb720_deletes_the_facet_when_its_last_reference_goes() {
        let mut e = Engine::new();
        stub(&mut e, &[LOCKIT_CTOR, LOCKIT_DTOR, 0x0300_1000]);
        let holder = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00ac_b720, &args![holder]);
        assert!(calls_to(&e, LOCKIT_CTOR).is_empty());
        let shared = facet(&mut e, 2);
        e.mem.set_u32(holder, shared);
        e.call_log = Some(vec![]);
        e.call(0x00ac_b720, &args![holder]);
        assert!(calls_to(&e, 0x0300_1000).is_empty());
        assert_eq!(e.mem.u32(shared + 4), 1);
        e.call(0x00ac_b720, &args![holder]);
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![shared, 1]]);
    }

    #[test]
    fn b046_00acb770_decref_keeps_zero_and_the_immortal_count() {
        let mut e = Engine::new();
        stub(&mut e, &[LOCKIT_CTOR, LOCKIT_DTOR]);
        for (count, after, released) in [
            (2u32, 1u32, false),
            (1, 0, true),
            (0, 0, true),
            (0xffff_ffff, 0xffff_ffff, false),
        ] {
            let shared = facet(&mut e, count);
            e.call_log = Some(vec![]);
            let result = e.call(0x00ac_b770, &args![shared]).u32();
            assert_eq!(e.mem.u32(shared + 4), after);
            assert_eq!(result, if released { shared } else { 0 });
            let lock = calls_to(&e, LOCKIT_CTOR);
            assert_eq!(lock.len(), 1);
            assert_eq!(lock[0][1], 0);
            assert_eq!(calls_to(&e, LOCKIT_DTOR), vec![vec![lock[0][0]]]);
        }
    }

    #[test]
    fn b046_00acb7e0_returns_all_ones() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ac_b7e0, &args![0x10u32]).u32(), 0xffff_ffff);
    }

    #[test]
    fn b046_00acc970_closes_an_open_file_then_runs_the_base() {
        let mut e = Engine::new();
        stub(&mut e, &[0x00ac_fb90, 0x00ec_3260]);
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        e.call(0x00ac_c970, &args![this]);
        assert!(calls_to(&e, 0x00ac_fb90).is_empty());
        assert_eq!(e.mem.u32(this), VTABLE_STREAMBUF_010A30FC);
        e.mem.set_u8(this + 0x48, 1);
        e.register(0x00ac_fb90, |e, a| {
            assert_eq!(e.mem.u32(a[0]), VTABLE_FILEBUF_010A3138);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ac_c970, &args![this]);
        assert_eq!(calls_to(&e, 0x00ac_fb90), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![this + 4]]);
    }

    #[test]
    fn b046_00ace2d0_stores_the_vtables_and_destroys_the_base() {
        let mut e = Engine::new();
        stub(&mut e, &[0x00ec_3205]);
        let this = e.mem.alloc(0x40);
        let vbtable = e.mem.alloc(8);
        e.mem.set_u32(vbtable + 4, 0x18);
        e.mem.set_u32(this, vbtable);
        e.call_log = Some(vec![]);
        e.call(0x00ac_e2d0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x18), 0x010a_318c);
        assert_eq!(e.mem.u32(this + 4), 0x010a_2f9c);
        assert_eq!(calls_to(&e, 0x00ec_3205), vec![vec![this + 4]]);
    }

    #[test]
    fn b046_00ace750_and_00ace790_copy_and_reference_the_locale() {
        let mut e = Engine::new();
        stub(&mut e, &[LOCKIT_CTOR, LOCKIT_DTOR]);
        let shared = facet(&mut e, 5);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, shared);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x24, holder);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00ac_e750, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u32(out), shared);
        assert_eq!(e.mem.u32(shared + 4), 6);
        let immortal = facet(&mut e, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x00ac_e790, &args![immortal]);
        assert_eq!(e.mem.u32(immortal + 4), 0xffff_ffff);
        assert_eq!(calls_to(&e, LOCKIT_CTOR).len(), 1);
        assert_eq!(calls_to(&e, LOCKIT_DTOR).len(), 1);
    }

    #[test]
    fn b046_00acfc10_points_the_buffer_at_the_file() {
        let mut e = Engine::new();
        e.map(INITIAL_STATE_011F6D8C, 4);
        e.set_global(INITIAL_STATE_011F6D8C, 0x1234u32);
        let this = e.mem.alloc(0x50);
        e.mem.write(this, &[0xcc; 0x50]);
        e.call(0x00ac_fc10, &args![this, 0u32, 1u32]);
        assert_eq!(e.mem.u8(this + 0x48), 1);
        assert_eq!(e.mem.u8(this + 0x41), 0);
        assert_eq!(e.mem.u32(this + 0x10), this + 8);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0x1234);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        e.call(0x00ac_fc10, &args![this, 0x7000u32, 2u32]);
        assert_eq!(e.mem.u8(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x7008);
        assert_eq!(e.mem.u32(this + 0x14), 0x7008);
        assert_eq!(e.mem.u32(this + 0x20), 0x7000);
        assert_eq!(e.mem.u32(this + 0x24), 0x7000);
        assert_eq!(e.mem.u32(this + 0x30), 0x7004);
        assert_eq!(e.mem.u32(this + 0x34), 0x7004);
        assert_eq!(e.mem.u32(this + 0x4c), 0x7000);
    }

    #[test]
    fn b046_00ad0720_resets_the_six_pointers() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        e.mem.write(this, &[0xcc; 0x38]);
        e.call(0x00ad_0720, &args![this]);
        for (slot, target) in [
            (0x10, 8),
            (0x14, 0xc),
            (0x20, 0x18),
            (0x24, 0x1c),
            (0x30, 0x28),
            (0x34, 0x2c),
        ] {
            assert_eq!(e.mem.u32(this + slot), this + target);
            assert_eq!(e.mem.u32(this + target), 0);
        }
    }

    /// A stream whose locale finds no `ctype` facet but the cached one: the cached facet's slot
    /// `0x18` (widen) returns the character plus 1.
    fn stream_world(e: &mut Engine) -> (u32, u32, u32) {
        stub(
            e,
            &[
                LOCKIT_CTOR,
                LOCKIT_DTOR,
                0x00ec_31b3,
                CXX_THROW_EXCEPTION,
                0x0300_1000,
            ],
        );
        e.map(CACHED_CTYPE_011F6D90, 4);
        e.map(FACET_ID_COUNTER_0126FBA0, 8);
        // The locale implementation (a facet itself: vtable, reference count, an empty table).
        let implementation = e.mem.alloc(0x18);
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        e.mem.set_u32(implementation, 0x0300_0000);
        e.mem.set_u32(implementation + 4, 5);
        let global = facet(e, 1);
        e.register_double(LOCALE_INIT, move |_, _| implementation.into_ret());
        e.register_double(GLOBAL_LOCALE_00EC34D4, move |_, _| global.into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let ctype = e.mem.alloc(8);
        e.put_vtable(0x0310_0000, &[0, 0, 0, 0, 0, 0, 0x0310_1000]);
        e.mem.set_u32(ctype, 0x0310_0000);
        e.mem.set_u32(ctype + 4, 0xffff_ffff);
        e.register(0x0310_1000, |_, a| (a[1] + 1).into_ret());
        e.set_global(CACHED_CTYPE_011F6D90, ctype);
        (implementation, global, ctype)
    }

    #[test]
    fn b046_00ad0550_initialises_the_base_and_its_locale() {
        let mut e = Engine::new();
        let (implementation, global, _) = stream_world(&mut e);
        let this = e.mem.alloc(0x34);
        e.mem.write(this, &[0xcc; 0x34]);
        e.call(0x00ad_0550, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x201);
        assert_eq!(e.mem.u32(this + 0x14), 6);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        let locale = e.mem.u32(this + 0x24);
        assert_eq!(e.mem.u32(locale), implementation);
        assert_eq!(e.mem.u32(global + 4), 2);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call(0x00ad_0550, &args![this]);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(global + 4), 2);
    }

    #[test]
    fn b046_00ad0470_and_00ad30c0_widen_the_fill_and_mark_a_missing_buffer() {
        let mut e = Engine::new();
        let (_, global, _) = stream_world(&mut e);
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00ad_0470, &args![this, 0u32, 0x100u32]);
        assert_eq!(e.mem.u8(this + 0x30), 0x21);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 8), 4);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert!(calls_to(&e, 0x00ec_31b3).is_empty());
        // The widen's locale reference was taken and dropped again.
        assert_eq!(e.mem.u32(global + 4), 2);
        e.call_log = Some(vec![]);
        e.call(0x00ad_0470, &args![this, 0x7000u32, 1u32]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0x7000);
        assert_eq!(calls_to(&e, 0x00ec_31b3), vec![vec![this]]);
        assert_eq!(e.call(0x00ad_30c0, &args![this, 0x141u32]).u32(), 0x42);
    }

    #[test]
    fn b046_00ad0630_constructs_the_stream_buffer() {
        let mut e = Engine::new();
        let (implementation, _, _) = stream_world(&mut e);
        stub(&mut e, &[0x00ec_3246]);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_0630, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_STREAMBUF_010A30FC);
        assert_eq!(calls_to(&e, 0x00ec_3246), vec![vec![this + 4]]);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0x38)), implementation);
        assert_eq!(e.mem.u32(this + 0x34), this + 0x2c);
    }

    #[test]
    fn b046_00ad33c0_stores_the_back_pointer_block() {
        let mut e = Engine::new();
        let block = e.mem.alloc(4);
        e.register_double(0x0044_d670, move |_, _| block.into_ret());
        let this = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_33c0, &args![this, 9u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x0044_d670), vec![vec![1, 0]]);
        assert_eq!(e.mem.u32(this), block);
        assert_eq!(e.mem.u32(block), this);
        e.register(0x0044_d670, |_, _| 0u32.into_ret());
        e.call(0x00ad_33c0, &args![this, 9u32]);
        assert_eq!(e.mem.u32(this), 0);
    }

    fn locale_with(e: &mut Engine, table: &[u32], fallback: bool) -> u32 {
        let implementation = e.mem.alloc(0x18);
        let entries = e.mem.alloc(4 * table.len() as u32 + 4);
        for (i, entry) in table.iter().enumerate() {
            e.mem.set_u32(entries + 4 * i as u32, *entry);
        }
        e.mem.set_u32(implementation + 8, entries);
        e.mem.set_u32(implementation + 0xc, table.len() as u32);
        e.mem.set_u8(implementation + 0x14, fallback as u8);
        let locale = e.mem.alloc(4);
        e.mem.set_u32(locale, implementation);
        locale
    }

    #[test]
    fn b046_00ad3a10_looks_in_the_locale_then_the_global_one() {
        let mut e = Engine::new();
        let global = e.mem.alloc(0x18);
        let entries = e.mem.alloc(8);
        e.mem.set_u32(entries + 4, 0x4444);
        e.mem.set_u32(global + 8, entries);
        e.mem.set_u32(global + 0xc, 2);
        e.register_double(GLOBAL_LOCALE_00EC34D4, move |_, _| global.into_ret());
        let own = locale_with(&mut e, &[0, 0x1111], false);
        assert_eq!(e.call(0x00ad_3a10, &args![own, 1u32]).u32(), 0x1111);
        assert_eq!(e.call(0x00ad_3a10, &args![own, 0u32]).u32(), 0);
        let falls_back = locale_with(&mut e, &[0], true);
        assert_eq!(e.call(0x00ad_3a10, &args![falls_back, 1u32]).u32(), 0x4444);
        assert_eq!(e.call(0x00ad_3a10, &args![falls_back, 2u32]).u32(), 0);
    }

    fn ctype_world(e: &mut Engine) -> u32 {
        stub(
            e,
            &[
                LOCKIT_CTOR,
                LOCKIT_DTOR,
                STRING_TIDY_0044A730,
                STRING_ASSIGN_0044A580,
                0x00ec_3740,
                0x00ec_35d7,
                0x00ec_3599,
            ],
        );
        e.map(STRING_NPOS_010173F0, 4);
        e.set_global(STRING_NPOS_010173F0, 0xffff_ffffu32);
        e.map(CACHED_CTYPE_011F6D90, 4);
        e.map(FACET_ID_COUNTER_0126FBA0, 8);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let vector = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(vector + 4 * i, 0xa0 + i);
        }
        e.register_double(0x00ec_38a9, move |_, _| vector.into_ret());
        vector
    }

    #[test]
    fn b046_00ad3aa0_makes_a_ctype_facet_once() {
        let mut e = Engine::new();
        ctype_world(&mut e);
        let locale = locale_with(&mut e, &[], false);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x55);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_3aa0, &args![slot, locale]).u32(), 2);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        e.mem.set_u32(slot, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_3aa0, &args![slot, locale]).u32(), 2);
        let made = e.mem.u32(slot);
        assert_eq!(e.mem.u32(made), VTABLE_CTYPE_010A31C4);
        assert_eq!(e.mem.u32(made + 4), 0);
        for i in 0..4 {
            assert_eq!(e.mem.u32(made + 8 + 4 * i), 0xa0 + i);
        }
        assert_eq!(calls_to(&e, 0x00ec_35d7).len(), 1);
        let tidy = calls_to(&e, STRING_TIDY_0044A730);
        assert_eq!(tidy.last().unwrap()[1..], [1, 0]);
        assert!(e.call(0x00ad_3aa0, &args![0u32, locale]).u32() == 2);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.mem.set_u32(slot, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ad_3aa0, &args![slot, locale]);
        assert_eq!(e.mem.u32(slot), 0);
        assert!(calls_to(&e, 0x00ec_35d7).is_empty());
        assert!(calls_to(&e, STRING_TIDY_0044A730).is_empty());
    }

    #[test]
    fn b046_00ad38d0_finds_caches_or_makes_the_facet() {
        let mut e = Engine::new();
        ctype_world(&mut e);
        e.set_global(FACET_ID_COUNTER_0126FBA0, 4u32);
        // Found in the locale: the id is assigned (5) and the entry returned.
        let mut table = vec![0u32; 6];
        table[5] = 0x5151;
        let locale = locale_with(&mut e, &table, false);
        assert_eq!(e.call(0x00ad_38d0, &args![locale]).u32(), 0x5151);
        assert_eq!(e.global::<u32>(CTYPE_ID_0126FBA4), 5);
        assert_eq!(e.global::<u32>(FACET_ID_COUNTER_0126FBA0), 5);
        // Not found, cached: the cached one.
        let empty = locale_with(&mut e, &[], false);
        e.set_global(CACHED_CTYPE_011F6D90, 0x6262u32);
        assert_eq!(e.call(0x00ad_38d0, &args![empty]).u32(), 0x6262);
        // Not found, none cached: made, cached, referenced and registered.
        e.set_global(CACHED_CTYPE_011F6D90, 0u32);
        e.call_log = Some(vec![]);
        let made = e.call(0x00ad_38d0, &args![empty]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.global::<u32>(CACHED_CTYPE_011F6D90), made);
        assert_eq!(e.mem.u32(made + 4), 1);
        assert_eq!(calls_to(&e, 0x00ec_3599), vec![vec![made]]);
        // The outer lock, the `_Locinfo`'s and `_Incref`'s.
        assert_eq!(calls_to(&e, LOCKIT_CTOR).len(), 3);
    }

    #[test]
    fn b046_00ad3ca0_and_00ad3db0_build_and_destroy_the_locinfo() {
        let mut e = Engine::new();
        ctype_world(&mut e);
        let this = e.mem.alloc(0x74);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_3ca0, &args![this, 0x8000u32]).u32(), this);
        assert_eq!(calls_to(&e, LOCKIT_CTOR), vec![vec![this, 0]]);
        assert_eq!(
            calls_to(&e, STRING_TIDY_0044A730),
            vec![
                vec![this + 4, 0, 0],
                vec![this + 0x20, 0, 0],
                vec![this + 0x3c, 0, 0],
                vec![this + 0x58, 0, 0]
            ]
        );
        assert_eq!(calls_to(&e, 0x00ec_3740), vec![vec![this, 0x8000]]);
        e.call_log = Some(vec![]);
        e.call(0x00ad_3db0, &args![this]);
        assert_eq!(calls_to(&e, 0x00ec_35d7), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, STRING_TIDY_0044A730),
            vec![
                vec![this + 0x58, 1, 0],
                vec![this + 0x3c, 1, 0],
                vec![this + 0x20, 1, 0],
                vec![this + 4, 1, 0]
            ]
        );
        assert_eq!(calls_to(&e, LOCKIT_DTOR), vec![vec![this]]);
    }

    #[test]
    fn b046_00ad3e60_assigns_the_locale_name() {
        let mut e = Engine::new();
        ctype_world(&mut e);
        let implementation = e.mem.alloc(0x40);
        let locale = e.mem.alloc(4);
        e.mem.set_u32(locale, implementation);
        let out = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_3e60, &args![locale, out]).u32(), out);
        assert_eq!(calls_to(&e, STRING_TIDY_0044A730), vec![vec![out, 0, 0]]);
        assert_eq!(
            calls_to(&e, STRING_ASSIGN_0044A580),
            vec![vec![out, implementation + 0x18, 0, 0xffff_ffff]]
        );
    }

    #[test]
    fn b046_00ad4000_returns_the_low_byte() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00ad_4000, &args![0u32, 0x1234u32]).u32() & 0xff,
            0x34
        );
    }

    #[test]
    fn b046_00ad4110_and_00ad41c0_free_the_table_by_its_flag() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE, 0x00ec_d291]);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x10, 0x9000);
        for (flag, freed, deleted) in [
            (1u32, true, false),
            (0xffff_ffff, false, true),
            (0, false, false),
        ] {
            e.mem.set_u32(this + 0x14, flag);
            e.call_log = Some(vec![]);
            e.call(0x00ad_41c0, &args![this]);
            assert_eq!(!calls_to(&e, 0x00ec_d291).is_empty(), freed);
            assert_eq!(!calls_to(&e, OPERATOR_DELETE).is_empty(), deleted);
        }
        e.mem.set_u32(this + 0x14, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ad_4110, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_FACET_010A31FC);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x00ad_4110, &args![this, 3u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
    }
}

#[cfg(test)]
mod n060_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, quiet);
        }
    }

    fn returns(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    fn logging(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    // A string model for the string doubles: the characters always on the heap.

    fn set_string(e: &mut Engine, string: u32, text: &[u8]) {
        let capacity = (text.len() as u32 + 1).max(0x10);
        let data = e.mem.alloc(capacity + 1);
        e.mem.write(data, text);
        e.mem.set_u8(data + text.len() as u32, 0);
        e.mem.set_u32(string + 4, data);
        e.mem.set_u32(string + 0x14, text.len() as u32);
        e.mem.set_u32(string + 0x18, capacity);
    }

    fn new_string(e: &mut Engine, text: &[u8]) -> u32 {
        let string = e.mem.alloc(0x1c);
        set_string(e, string, text);
        string
    }

    fn read_string(e: &Engine, string: u32) -> Vec<u8> {
        let data = string_chars(e, string);
        let length = e.mem.u32(string + 0x14);
        (0..length).map(|i| e.mem.u8(data + i)).collect()
    }

    fn read_c(e: &Engine, text: u32) -> Vec<u8> {
        let mut out = vec![];
        let mut at = text;
        while e.mem.u8(at) != 0 {
            out.push(e.mem.u8(at));
            at += 1;
        }
        out
    }

    fn new_c(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(at, text);
        at
    }

    fn string_doubles(e: &mut Engine) {
        e.map(STRING_NPOS_010173F0, 4);
        e.set_global(STRING_NPOS_010173F0, 0xffff_ffffu32);
        e.register(STRING_TIDY_0044A730, |e, a| {
            set_string(e, a[0], b"");
            Ret::default()
        });
        e.register(STRING_FROM_C_0044A4C0, |e, a| {
            let text = read_c(e, a[1]);
            set_string(e, a[0], &text);
            a[0].into_ret()
        });
        e.register(STRING_COPY_0044A500, |e, a| {
            let text = read_string(e, a[1]);
            set_string(e, a[0], &text);
            a[0].into_ret()
        });
        e.register(STRING_ASSIGN_0044A580, |e, a| {
            let text = read_string(e, a[1]);
            let start = (a[2] as usize).min(text.len());
            let count = (a[3] as usize).min(text.len() - start);
            set_string(e, a[0], &text[start..start + count]);
            a[0].into_ret()
        });
        e.register(STRING_APPEND_FILL_00AADEB0, |e, a| {
            let mut text = read_string(e, a[0]);
            text.extend(std::iter::repeat(a[2] as u8).take(a[1] as usize));
            set_string(e, a[0], &text);
            a[0].into_ret()
        });
        e.register(STRING_APPEND_CHARS_00A0C430, |e, a| {
            let mut text = read_string(e, a[0]);
            for i in 0..a[2] {
                text.push(e.mem.u8(a[1] + i));
            }
            set_string(e, a[0], &text);
            a[0].into_ret()
        });
        e.register(STRING_ERASE_0044A690, |e, a| {
            let mut text = read_string(e, a[0]);
            let start = (a[1] as usize).min(text.len());
            let count = (a[2] as usize).min(text.len() - start);
            text.drain(start..start + count);
            set_string(e, a[0], &text);
            a[0].into_ret()
        });
        e.register(CRT_STRLEN, |e, a| (read_c(e, a[0]).len() as u32).into_ret());
    }

    fn allocator_doubles(e: &mut Engine) {
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        e.register(OPERATOR_DELETE, quiet);
        e.register(INVALID_PARAMETER, |_, _| {
            panic!("_invalid_parameter_noinfo")
        });
    }

    /// A stream buffer with its six pointer slots on its own storage ([`fn_00ad0720`]) and a
    /// vtable of 16 slots at 0x0400_0000, slot `k` calling 0x0410_0000 + k.
    const BUFFER_VTABLE: u32 = 0x0400_0000;
    fn slot_target(slot: u32) -> u32 {
        0x0410_0000 + slot
    }

    fn new_buffer(e: &mut Engine, size: u32) -> u32 {
        let slots: Vec<u32> = (0..32).map(|i| slot_target(4 * i)).collect();
        e.put_vtable(BUFFER_VTABLE, &slots);
        let buffer = e.mem.alloc(size.max(0x50));
        e.mem.set_u32(buffer, BUFFER_VTABLE);
        fn_00ad0720(e, Ptr::new(buffer));
        buffer
    }

    /// Sets the get area to `text` (start, next and count) and returns the text's address.
    fn set_get_area(e: &mut Engine, buffer: u32, text: &[u8]) -> u32 {
        let data = new_c(e, text);
        for (slot, value) in [(0x10, data), (0x20, data), (0x30, text.len() as u32)] {
            let target = e.mem.u32(buffer + slot);
            e.mem.set_u32(target, value);
        }
        data
    }

    /// Sets the put area to `room` free bytes and returns its start.
    fn set_put_area(e: &mut Engine, buffer: u32, room: u32) -> u32 {
        let data = e.mem.alloc(room.max(1));
        for (slot, value) in [(0x14, data), (0x24, data), (0x34, room)] {
            let target = e.mem.u32(buffer + slot);
            e.mem.set_u32(target, value);
        }
        data
    }

    fn slot_word(e: &Engine, buffer: u32, slot: u32) -> u32 {
        e.mem.u32(e.mem.u32(buffer + slot))
    }

    /// A stream with its `basic_ios` `ios_offset` bytes in (virtual-base table at `+0`), the
    /// buffer at `base + 0x28`, `skipws` set and the fill character `' '`.
    struct Stream {
        stream: u32,
        ios: u32,
        buffer: u32,
    }

    fn new_stream(e: &mut Engine, ios_offset: u32) -> Stream {
        let stream = e.mem.alloc(ios_offset + 0x40);
        let vbtable = e.mem.alloc(8);
        e.mem.set_u32(vbtable + 4, ios_offset);
        e.mem.set_u32(stream, vbtable);
        let ios = stream + ios_offset;
        let buffer = new_buffer(e, 0x50);
        e.mem.set_u32(ios + 0x28, buffer);
        e.mem.set_u32(ios + 0x10, 1);
        e.mem.set_u8(ios + 0x30, b' ');
        Stream {
            stream,
            ios,
            buffer,
        }
    }

    /// The sentry's mutex, the uncaught-exception query (none), the `_Lockit` pair.
    fn stream_doubles(e: &mut Engine) {
        stub(e, &[MUTEX_LOCK, MUTEX_UNLOCK, LOCKIT_CTOR, LOCKIT_DTOR]);
        returns(e, UNCAUGHT_EXCEPTION, 0);
        e.register(slot_target(0x10), |_, _| 0xffff_ffffu32.into_ret());
        e.register(slot_target(0x14), |_, _| 0xffff_ffffu32.into_ret());
        e.register(slot_target(4), |_, _| 0xffff_ffffu32.into_ret());
        e.register(slot_target(0x30), |_, _| 0u32.into_ret());
    }

    /// Makes the cached `ctype<char>` facet (blanks, tabs and newlines classed as white space,
    /// 0x48; `widen` answering the character) and answers a locale implementation that has no
    /// facets of its own.
    fn ctype_world(e: &mut Engine) -> u32 {
        e.map(CACHED_CTYPE_011F6D90, 4);
        e.map(FACET_ID_COUNTER_0126FBA0, 8);
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        e.register(0x0300_1000, quiet);
        let implementation = e.mem.alloc(0x18);
        e.mem.set_u32(implementation, 0x0300_0000);
        e.mem.set_u32(implementation + 4, 5);
        let table = e.mem.alloc(0x200);
        for character in *b" \t\n" {
            e.mem.set_u16(table + 2 * character as u32, 0x48);
        }
        e.put_vtable(0x0310_0000, &[0, 0, 0, 0, 0, 0, 0x0310_1000]);
        e.register(0x0310_1000, |_, a| a[1].into_ret());
        let ctype = e.mem.alloc(0x18);
        e.mem.set_u32(ctype, 0x0310_0000);
        e.mem.set_u32(ctype + 4, 0xffff_ffff);
        e.mem.set_u32(ctype + 0x10, table);
        e.set_global(CACHED_CTYPE_011F6D90, ctype);
        implementation
    }

    /// Gives the stream a locale (see [`ctype_world`]).
    fn give_locale(e: &mut Engine, ios: u32) {
        let implementation = ctype_world(e);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, implementation);
        e.mem.set_u32(ios + 0x24, holder);
    }

    // The tests come below, one or more per translated function.

    fn saturate_doubles(e: &mut Engine) {
        e.register(CRT_ISNAN, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            u32::from(value.is_nan()).into_ret()
        });
        e.register(CRT_FINITE, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            u32::from(value.is_finite()).into_ret()
        });
        e.register(CRT_FABS, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            value.abs().into_ret()
        });
        stub(e, &[FR2_ASSERT]);
        e.map(SATURATE_HIGH_01016970, 4);
        e.map(SATURATE_LOW_01015F5C, 4);
        e.set_global(SATURATE_HIGH_01016970, 7.0f32.to_bits());
        e.set_global(SATURATE_LOW_01015F5C, (-7.0f32).to_bits());
    }

    #[test]
    fn n060_00ac79e0_adds_and_saturates() {
        let mut e = Engine::new();
        saturate_doubles(&mut e);
        logging(&mut e);
        let sum = e.call(0x00ac_79e0, &args![1.5f32, 2.25f32]).f64();
        assert_eq!(sum, 3.75);
        assert!(calls_to(&e, FR2_ASSERT).is_empty());
        let high = e.call(0x00ac_79e0, &args![3.0e38f32, 3.0e38f32]).f64();
        assert_eq!(high, 7.0);
        let low = e.call(0x00ac_79e0, &args![-3.0e38f32, -3.0e38f32]).f64();
        assert_eq!(low, -7.0);
        logging(&mut e);
        let nan = e.call(0x00ac_79e0, &args![f32::NAN, 1.0f32]).f64();
        assert_eq!(nan, -7.0);
        assert_eq!(
            calls_to(&e, FR2_ASSERT),
            vec![vec![ALGS_CPP_010A2F50, 0x1b2]]
        );
        logging(&mut e);
        e.call(0x00ac_79e0, &args![1.0f32, f32::NAN]);
        assert_eq!(
            calls_to(&e, FR2_ASSERT),
            vec![vec![ALGS_CPP_010A2F50, 0x1b3]]
        );
    }

    #[test]
    fn n060_00ac9bd0_destroys_both_strings() {
        let mut e = Engine::new();
        stub(&mut e, &[STRING_TIDY_0044A730]);
        let pair = e.mem.alloc(0x38);
        logging(&mut e);
        e.call(0x00ac_9bd0, &args![pair]);
        assert_eq!(
            calls_to(&e, STRING_TIDY_0044A730),
            vec![vec![pair + 0x1c, 1, 0], vec![pair, 1, 0]]
        );
    }

    #[test]
    fn n060_00ac9c30_destroys_and_frees_with_the_flag() {
        let mut e = Engine::new();
        stub(&mut e, &[IOS_BASE_DTOR, OPERATOR_DELETE]);
        let ios = e.mem.alloc(0x34);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_9c30, &args![ios, 0u32]).u32(), ios);
        assert_eq!(e.mem.u32(ios), VTABLE_IOS_BASE_010A2F9C);
        assert_eq!(calls_to(&e, IOS_BASE_DTOR), vec![vec![ios]]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x00ac_9c30, &args![ios, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![ios]]);
    }

    /// An object whose stream sits `offset` below `this` (virtual base at `offset`), with a
    /// stream buffer of initialised pointer slots at `stream + 8` (or `+4` for an output
    /// stream).
    fn stream_object(e: &mut Engine, offset: u32, buffer_at: u32) -> (u32, u32) {
        let object = e.mem.alloc(offset + 0x40);
        let vbtable = e.mem.alloc(8);
        e.mem.set_u32(vbtable + 4, offset);
        e.mem.set_u32(object, vbtable);
        fn_00ad0720(&mut *e, Ptr::new(object + buffer_at));
        (object, object + offset)
    }

    #[test]
    fn n060_00ac9c70_and_00acb610_destroy_the_string_stream() {
        let mut e = Engine::new();
        stub(&mut e, &[IOS_BASE_DTOR, 0x00ec_3260]);
        let (object, ios) = stream_object(&mut e, 0x50, 8);
        logging(&mut e);
        e.call(0x00ac_9c70, &args![object]);
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![object + 8 + 4]]);
        assert_eq!(calls_to(&e, IOS_BASE_DTOR), vec![vec![ios]]);
        assert_eq!(e.mem.u32(ios), VTABLE_IOS_BASE_010A2F9C);
        assert_eq!(e.mem.u32(object + 8), VTABLE_STREAMBUF_010A30FC);
        let (object, ios) = stream_object(&mut e, 0x50, 8);
        logging(&mut e);
        e.call(0x00ac_b610, &args![ios]);
        assert_eq!(e.mem.u32(ios), VTABLE_ISTREAM_010A30A8);
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![object + 8 + 4]]);
        assert!(calls_to(&e, IOS_BASE_DTOR).is_empty());
    }

    #[test]
    fn n060_00ac9cb0_and_00acc8c0_destroy_the_file_stream() {
        let mut e = Engine::new();
        stub(&mut e, &[IOS_BASE_DTOR, 0x00ec_3260]);
        let (object, ios) = stream_object(&mut e, 0x58, 8);
        logging(&mut e);
        e.call(0x00ac_9cb0, &args![object]);
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![object + 8 + 4]]);
        assert_eq!(calls_to(&e, IOS_BASE_DTOR), vec![vec![ios]]);
        // An open file is closed first.
        let (object, ios) = stream_object(&mut e, 0x58, 8);
        e.mem.set_u8(object + 8 + 0x48, 1);
        e.register(0x00ec_9907, |_, _| 0u32.into_ret());
        e.register(CRT_FCLOSE, |_, _| 0u32.into_ret());
        e.map(INITIAL_STATE_011F6D8C, 4);
        e.mem.set_u32(object + 8 + 0x4c, 0x7777);
        logging(&mut e);
        e.call(0x00ac_c8c0, &args![ios]);
        assert_eq!(calls_to(&e, CRT_FCLOSE), vec![vec![0x7777]]);
        assert_eq!(e.mem.u32(ios), VTABLE_ISTREAM_010A30A8);
        assert_eq!(e.mem.u32(object + 8), VTABLE_STREAMBUF_010A30FC);
    }

    #[test]
    fn n060_00ac9cf0_empties_the_vector_and_frees_the_proxy() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE, STRING_TIDY_0044A730]);
        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector, 0x4444);
        let storage = e.mem.alloc(0x38);
        e.mem.set_u32(vector + 0xc, storage);
        e.mem.set_u32(vector + 0x10, storage + 0x38);
        e.mem.set_u32(vector + 0x14, storage + 0x38);
        logging(&mut e);
        e.call(0x00ac_9cf0, &args![vector]);
        assert_eq!(
            calls_to(&e, STRING_TIDY_0044A730),
            vec![vec![storage, 1, 0], vec![storage + 0x1c, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, OPERATOR_DELETE),
            vec![vec![storage], vec![0x4444]]
        );
        assert_eq!(e.mem.u32(vector + 0xc), 0);
    }

    #[test]
    fn n060_00ac9d60_and_00acdeb0_destroy_the_output_file_stream() {
        let mut e = Engine::new();
        stub(&mut e, &[IOS_BASE_DTOR, 0x00ec_3260]);
        let (object, ios) = stream_object(&mut e, 0x54, 4);
        logging(&mut e);
        e.call(0x00ac_9d60, &args![object]);
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![object + 4 + 4]]);
        assert_eq!(calls_to(&e, IOS_BASE_DTOR), vec![vec![ios]]);
        let (object, ios) = stream_object(&mut e, 0x54, 4);
        e.call(0x00ac_deb0, &args![ios]);
        assert_eq!(e.mem.u32(ios), VTABLE_OSTREAM_010A318C);
        assert_eq!(e.mem.u32(object + 4), VTABLE_STREAMBUF_010A30FC);
    }

    #[test]
    fn n060_00ac9df0_puts_a_character_or_marks_badbit() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let s = new_stream(&mut e, 0x10);
        let data = set_put_area(&mut e, s.buffer, 4);
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_9df0, &args![s.stream, 0x141u32]).u32(),
            s.stream
        );
        assert_eq!(e.mem.u8(data), 0x41);
        assert_eq!(slot_word(&e, s.buffer, 0x34), 3);
        assert_eq!(e.mem.u32(s.ios + 8), 0);
        assert_eq!(calls_to(&e, MUTEX_LOCK), vec![vec![s.buffer + 4]]);
        assert_eq!(calls_to(&e, MUTEX_UNLOCK), vec![vec![s.buffer + 4]]);
        // A full buffer whose overflow fails: badbit.
        set_put_area(&mut e, s.buffer, 0);
        e.call(0x00ac_9df0, &args![s.stream, 0x42u32]);
        assert_eq!(e.mem.u32(s.ios + 8), 4);
        // A bad stream writes nothing.
        let data = set_put_area(&mut e, s.buffer, 4);
        logging(&mut e);
        e.call(0x00ac_9df0, &args![s.stream, 0x43u32]);
        assert_eq!(e.mem.u8(data), 0);
        assert_eq!(e.mem.u32(s.ios + 8), 4);
    }

    #[test]
    fn n060_00aca1e0_returns_the_message_characters() {
        let mut e = Engine::new();
        let error = e.mem.alloc(0x28);
        e.mem.set_u32(error + 0xc + 0x18, 0xf);
        assert_eq!(e.call(0x00ac_a1e0, &args![error]).u32(), error + 0x10);
        e.mem.set_u32(error + 0xc + 0x18, 0x10);
        e.mem.set_u32(error + 0x10, 0x9000);
        assert_eq!(e.call(0x00ac_a1e0, &args![error]).u32(), 0x9000);
    }

    #[test]
    fn n060_00aca220_and_00aca340_and_00aca300_destroy_the_errors() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[STRING_TIDY_0044A730, EXCEPTION_DTOR, OPERATOR_DELETE],
        );
        let error = e.mem.alloc(0x28);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_a220, &args![error, 1u32]).u32(), error);
        assert_eq!(e.mem.u32(error), VTABLE_RUNTIME_ERROR_010A307C);
        assert_eq!(
            calls_to(&e, STRING_TIDY_0044A730),
            vec![vec![error + 0xc, 1, 0]]
        );
        assert_eq!(calls_to(&e, EXCEPTION_DTOR), vec![vec![error]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![error]]);
        logging(&mut e);
        e.call(0x00ac_a340, &args![error, 0u32]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.mem.u32(error), VTABLE_RUNTIME_ERROR_010A307C);
        e.call(0x00ac_a340, &args![error, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![error]]);
        logging(&mut e);
        e.call(0x00ac_a300, &args![error]);
        assert_eq!(calls_to(&e, EXCEPTION_DTOR), vec![vec![error]]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn n060_00aca270_and_00aca390_copy_the_errors() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        stub(&mut e, &[EXCEPTION_COPY]);
        let source = e.mem.alloc(0x28);
        set_string(&mut e, source + 0xc, b"failure");
        let copy = e.mem.alloc(0x28);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_a270, &args![copy, source]).u32(), copy);
        assert_eq!(calls_to(&e, EXCEPTION_COPY), vec![vec![copy, source]]);
        assert_eq!(e.mem.u32(copy), VTABLE_RUNTIME_ERROR_010A307C);
        assert_eq!(read_string(&e, copy + 0xc), b"failure");
        let other = e.mem.alloc(0x28);
        assert_eq!(e.call(0x00ac_a390, &args![other, source]).u32(), other);
        assert_eq!(e.mem.u32(other), VTABLE_IOS_FAILURE_010A3070);
        assert_eq!(read_string(&e, other + 0xc), b"failure");
    }

    #[test]
    fn n060_00aca3c0_flushes_unless_failed() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let s = new_stream(&mut e, 0x10);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_a3c0, &args![s.stream]).u32(), s.stream);
        assert_eq!(calls_to(&e, slot_target(0x30)), vec![vec![s.buffer]]);
        assert_eq!(e.mem.u32(s.ios + 8), 0);
        e.register(slot_target(0x30), |_, _| 0xffff_ffffu32.into_ret());
        e.call(0x00ac_a3c0, &args![s.stream]);
        assert_eq!(e.mem.u32(s.ios + 8), 4);
        logging(&mut e);
        e.call(0x00ac_a3c0, &args![s.stream]);
        assert!(calls_to(&e, slot_target(0x30)).is_empty());
    }

    #[test]
    fn n060_00aca4b0_destroys_the_basic_ios() {
        let mut e = Engine::new();
        stub(&mut e, &[IOS_BASE_DTOR]);
        let ios = e.mem.alloc(0x34);
        logging(&mut e);
        e.call(0x00ac_a4b0, &args![ios]);
        assert_eq!(e.mem.u32(ios), VTABLE_IOS_BASE_010A2F9C);
        assert_eq!(calls_to(&e, IOS_BASE_DTOR), vec![vec![ios]]);
    }

    #[test]
    fn n060_00aca4e0_sets_the_state_with_badbit_without_a_buffer() {
        let mut e = Engine::new();
        let ios = e.mem.alloc(0x34);
        e.call(0x00ac_a4e0, &args![ios, 0u32, 0u32]);
        assert_eq!(e.mem.u32(ios + 8), 0);
        e.mem.set_u32(ios + 0x28, 0x5000);
        e.call(0x00ac_a4e0, &args![ios, 2u32, 0u32]);
        assert_eq!(e.mem.u32(ios + 8), 2);
        e.mem.set_u32(ios + 0x28, 0);
        e.call(0x00ac_a4e0, &args![ios, 1u32, 0u32]);
        assert_eq!(e.mem.u32(ios + 8), 7);
    }

    #[test]
    fn n060_00aca560_gets_and_counts() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let s = new_stream(&mut e, 0x10);
        e.mem.set_u32(s.ios + 0x10, 0);
        set_get_area(&mut e, s.buffer, b"xy");
        e.mem.set_u32(s.stream + 4, 9);
        assert_eq!(e.call(0x00ac_a560, &args![s.stream]).u32(), u32::from(b'x'));
        assert_eq!(e.mem.u32(s.stream + 4), 1);
        assert_eq!(e.call(0x00ac_a560, &args![s.stream]).u32(), u32::from(b'y'));
        assert_eq!(e.call(0x00ac_a560, &args![s.stream]).u32(), 0xffff_ffff);
        assert_eq!(e.mem.u32(s.stream + 4), 0);
        assert_eq!(e.mem.u32(s.ios + 8), 3);
        // A failed stream answers -1 at once.
        assert_eq!(e.call(0x00ac_a560, &args![s.stream]).u32(), 0xffff_ffff);
        assert_eq!(e.mem.u32(s.ios + 8), 3);
    }

    #[test]
    fn n060_00aca7a0_peeks_without_taking() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let s = new_stream(&mut e, 0x10);
        e.mem.set_u32(s.ios + 0x10, 0);
        set_get_area(&mut e, s.buffer, b"q");
        assert_eq!(e.call(0x00ac_a7a0, &args![s.stream]).u32(), u32::from(b'q'));
        assert_eq!(e.call(0x00ac_a7a0, &args![s.stream]).u32(), u32::from(b'q'));
        assert_eq!(slot_word(&e, s.buffer, 0x30), 1);
        set_get_area(&mut e, s.buffer, b"");
        assert_eq!(e.call(0x00ac_a7a0, &args![s.stream]).u32(), 0xffff_ffff);
        assert_eq!(e.mem.u32(s.ios + 8), 1);
    }

    #[test]
    fn n060_00aca780_and_00aca9a0_unlock_through_the_sentry() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let s = new_stream(&mut e, 0x10);
        let sentry = e.mem.alloc(8);
        e.mem.set_u32(sentry, s.stream);
        logging(&mut e);
        e.call(0x00ac_a780, &args![sentry]);
        e.call(0x00ac_a9a0, &args![sentry]);
        assert_eq!(
            calls_to(&e, MUTEX_UNLOCK),
            vec![vec![s.buffer + 4], vec![s.buffer + 4]]
        );
        e.mem.set_u32(s.ios + 0x28, 0);
        logging(&mut e);
        e.call(0x00ac_a9a0, &args![sentry]);
        assert!(calls_to(&e, MUTEX_UNLOCK).is_empty());
    }

    #[test]
    fn n060_00aca9f0_reads_or_underflows() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let buffer = new_buffer(&mut e, 0x50);
        set_get_area(&mut e, buffer, b"k");
        assert_eq!(e.call(0x00ac_a9f0, &args![buffer]).u32(), u32::from(b'k'));
        e.register(slot_target(0x10), |_, _| 0x55u32.into_ret());
        set_get_area(&mut e, buffer, b"");
        assert_eq!(e.call(0x00ac_a9f0, &args![buffer]).u32(), 0x55);
    }

    #[test]
    fn n060_00acaa50_makes_the_proxy_and_reserves_nothing() {
        let mut e = Engine::new();
        let proxy = e.mem.alloc(4);
        returns(&mut e, ALLOCATE_PROXY_0044D670, proxy);
        let vector = e.mem.alloc(0x18);
        e.mem.write(vector, &[0xcc; 0x18]);
        assert_eq!(e.call(0x00ac_aa50, &args![vector]).u32(), vector);
        assert_eq!(e.mem.u32(vector), proxy);
        assert_eq!(e.mem.u32(proxy), vector);
        assert_eq!(e.mem.u32(vector + 0xc), 0);
        assert_eq!(e.mem.u32(vector + 0x14), 0);
        returns(&mut e, ALLOCATE_PROXY_0044D670, 0);
        e.call(0x00ac_aa50, &args![vector]);
        assert_eq!(e.mem.u32(vector), 0);
    }

    /// A vector with storage for `capacity` elements of `element` bytes, `used` of them in use,
    /// its proxy at `+0` pointing back at it.
    fn new_vector(e: &mut Engine, element: u32, used: u32, capacity: u32) -> u32 {
        let vector = e.mem.alloc(0x18);
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, vector);
        e.mem.set_u32(vector, proxy);
        let storage = e.mem.alloc((element * capacity).max(4));
        e.mem.set_u32(vector + 0xc, storage);
        e.mem.set_u32(vector + 0x10, storage + element * used);
        e.mem.set_u32(vector + 0x14, storage + element * capacity);
        vector
    }

    #[test]
    fn n060_00acab30_appends_in_place_or_inserts() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        e.register(CONSTRUCT_STRING_00AD6990, |e, a| {
            let text = read_string(e, a[1]);
            set_string(e, a[0], &text);
            Ret::default()
        });
        let vector = new_vector(&mut e, 0x1c, 1, 2);
        let value = new_string(&mut e, b"cell");
        let first = e.mem.u32(vector + 0xc);
        e.call(0x00ac_ab30, &args![vector, value]);
        assert_eq!(e.mem.u32(vector + 0x10), first + 0x38);
        assert_eq!(read_string(&e, first + 0x1c), b"cell");
        // Full: the insert path grows the storage.
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_DELETE, quiet);
        e.register(COPY_CONSTRUCT_STRINGS_00AD66E0, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = read_string(e, from);
                set_string(e, to, &text);
                from += 0x1c;
                to += 0x1c;
            }
            to.into_ret()
        });
        e.call(0x00ac_ab30, &args![vector, value]);
        let first = e.mem.u32(vector + 0xc);
        assert_eq!(e.mem.u32(vector + 0x10), first + 3 * 0x1c);
        assert_eq!(read_string(&e, first + 0x38), b"cell");
        assert_eq!(read_string(&e, first + 0x1c), b"cell");
    }

    #[test]
    fn n060_00acac40_clears_the_string_vector() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        e.register(ITERATOR_NOT_EQUAL_00AC0460, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        e.register(COPY_STRINGS_00AD5A80, |_, a| a[2].into_ret());
        let vector = new_vector(&mut e, 0x1c, 2, 2);
        let first = e.mem.u32(vector + 0xc);
        logging(&mut e);
        e.call(0x00ac_ac40, &args![vector]);
        assert_eq!(e.mem.u32(vector + 0x10), first);
        assert_eq!(
            calls_to(&e, COPY_STRINGS_00AD5A80),
            vec![vec![first + 0x38, first + 0x38, first, 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, STRING_TIDY_0044A730).len(), 2);
    }

    #[test]
    fn n060_00acad30_indexes_with_a_check() {
        let mut e = Engine::new();
        let vector = new_vector(&mut e, 1, 4, 8);
        let first = e.mem.u32(vector + 0xc);
        assert_eq!(e.call(0x00ac_ad30, &args![vector, 3u32]).u32(), first + 3);
        stub(&mut e, &[INVALID_PARAMETER]);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_ad30, &args![vector, 4u32]).u32(), first + 4);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn n060_00acad60_appends_a_byte() {
        let mut e = Engine::new();
        e.register(FILL_BYTES_00AC3300, |e, a| {
            let value = e.mem.u8(a[2]);
            for i in 0..a[1] {
                e.mem.set_u8(a[0] + i, value);
            }
            Ret::default()
        });
        let vector = new_vector(&mut e, 1, 1, 2);
        let first = e.mem.u32(vector + 0xc);
        let value = new_c(&mut e, b"z");
        e.call(0x00ac_ad60, &args![vector, value]);
        assert_eq!(e.mem.u8(first + 1), b'z');
        assert_eq!(e.mem.u32(vector + 0x10), first + 2);
        // Full: the insert path.
        e.register(ALLOCATE_CHARACTERS, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(OPERATOR_DELETE, quiet);
        e.register(MEMMOVE_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.call(0x00ac_ad60, &args![vector, value]);
        let first = e.mem.u32(vector + 0xc);
        assert_eq!(e.mem.bytes(first, 3), [0, b'z', b'z']);
        assert_eq!(e.mem.u32(vector + 0x10), first + 3);
    }

    #[test]
    fn n060_00acae90_clears_the_byte_vector() {
        let mut e = Engine::new();
        e.register(ITERATOR_NOT_EQUAL_00AC0460, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        stub(&mut e, &[MEMMOVE_S]);
        let vector = new_vector(&mut e, 1, 5, 8);
        let first = e.mem.u32(vector + 0xc);
        e.call(0x00ac_ae90, &args![vector]);
        assert_eq!(e.mem.u32(vector + 0x10), first);
    }

    /// `00ac6100`'s double: compares the string `this` with the `count` characters at `text`
    /// (-1, 0 or 1).
    fn compare_double(e: &mut Engine) {
        e.register(STRING_COMPARE_00AC6100, |e, a| {
            let own = read_string(e, a[0]);
            let other = e.mem.bytes(a[3], a[4]);
            let order = match own.cmp(&other) {
                std::cmp::Ordering::Less => -1i32,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            order.into_ret()
        });
    }

    fn new_pair(e: &mut Engine, key: &[u8], value: &[u8]) -> u32 {
        let pair = e.mem.alloc(0x38);
        set_string(e, pair, key);
        set_string(e, pair + 0x1c, value);
        pair
    }

    /// An empty string-keyed tree (head node black and "nil", its proxy pointing back).
    fn new_tree(e: &mut Engine) -> u32 {
        let tree = e.mem.alloc(0x20);
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, tree);
        e.mem.set_u32(tree, proxy);
        let head = e.mem.alloc(0x48);
        for offset in [0, 4, 8] {
            e.mem.set_u32(head + offset, head);
        }
        e.mem.set_u8(head + 0x44, 1);
        e.mem.set_u8(head + 0x45, 1);
        e.mem.set_u32(tree + 0x18, head);
        tree
    }

    fn tree_doubles(e: &mut Engine) {
        string_doubles(e);
        compare_double(e);
        allocator_doubles(e);
    }

    /// Inserts `key = value`; answers the node and whether it was inserted.
    fn insert(e: &mut Engine, tree: u32, key: &[u8], value: &[u8]) -> (u32, bool) {
        let pair = new_pair(e, key, value);
        let out = e.mem.alloc(0xc);
        e.call(0x00ac_af50, &args![tree, out, pair]);
        (e.mem.u32(out + 4), e.mem.u8(out + 8) != 0)
    }

    /// The keys in order, walking from the rightmost node back with `operator--`.
    fn keys_backwards(e: &mut Engine, tree: u32) -> Vec<Vec<u8>> {
        let head = e.mem.u32(tree + 0x18);
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, e.mem.u32(tree));
        e.mem.set_u32(iterator + 4, head);
        let mut keys = vec![];
        for _ in 0..e.mem.u32(tree + 0x1c) {
            e.call(0x00ad_3450, &args![iterator]);
            let node = e.mem.u32(iterator + 4);
            keys.push(read_string(e, node + 0xc));
        }
        keys
    }

    #[test]
    fn n060_00acaf50_00acf120_00ad1960_insert_and_balance() {
        let mut e = Engine::new();
        tree_doubles(&mut e);
        let tree = new_tree(&mut e);
        for key in [b"m", b"c", b"t", b"a", b"e", b"r", b"z", b"b"] {
            assert!(insert(&mut e, tree, key, b"v").1);
        }
        let (node, inserted) = insert(&mut e, tree, b"e", b"other");
        assert!(!inserted);
        assert_eq!(read_string(&e, node + 0x28), b"v");
        assert_eq!(e.mem.u32(tree + 0x1c), 8);
        let head = e.mem.u32(tree + 0x18);
        let root = e.mem.u32(head + 4);
        assert_eq!(e.mem.u8(root + 0x44), 1);
        assert_eq!(read_string(&e, e.mem.u32(head) + 0xc), b"a");
        assert_eq!(read_string(&e, e.mem.u32(head + 8) + 0xc), b"z");
        let keys = keys_backwards(&mut e, tree);
        let expected: Vec<Vec<u8>> = [b"z", b"t", b"r", b"m", b"e", b"c", b"b", b"a"]
            .iter()
            .map(|k| k.to_vec())
            .collect();
        assert_eq!(keys, expected);
    }

    #[test]
    fn n060_00acf120_throws_at_the_maximum() {
        let mut e = Engine::new();
        tree_doubles(&mut e);
        stub(&mut e, &[LENGTH_ERROR_CTOR_0044A310, CXX_THROW_EXCEPTION]);
        e.map(MAP_TOO_LONG_010A31A0, 0x20);
        e.mem.write(MAP_TOO_LONG_010A31A0, b"map/set<T> too long");
        let tree = new_tree(&mut e);
        e.mem.set_u32(tree + 0x1c, 0x0492_4923);
        let out = e.mem.alloc(8);
        let pair = new_pair(&mut e, b"k", b"v");
        logging(&mut e);
        let head = e.mem.u32(tree + 0x18);
        e.call(0x00ac_f120, &args![tree, out, 1u32, head, pair]);
        assert_eq!(calls_to(&e, CXX_THROW_EXCEPTION).len(), 1);
        assert_eq!(
            calls_to(&e, CXX_THROW_EXCEPTION)[0][1],
            LENGTH_ERROR_THROW_INFO_01140CB0
        );
        assert_eq!(e.mem.u32(tree + 0x1c), 0x0492_4923);
    }

    #[test]
    fn n060_00acb200_and_00ad1790_find_by_key() {
        let mut e = Engine::new();
        tree_doubles(&mut e);
        let tree = new_tree(&mut e);
        let (b, _) = insert(&mut e, tree, b"b", b"1");
        let (d, _) = insert(&mut e, tree, b"d", b"2");
        let head = e.mem.u32(tree + 0x18);
        let key = new_string(&mut e, b"c");
        assert_eq!(e.call(0x00ad_1790, &args![tree, key]).u32(), d);
        let key = new_string(&mut e, b"b");
        assert_eq!(e.call(0x00ad_1790, &args![tree, key]).u32(), b);
        let key = new_string(&mut e, b"e");
        assert_eq!(e.call(0x00ad_1790, &args![tree, key]).u32(), head);
        let out = e.mem.alloc(8);
        let key = new_string(&mut e, b"d");
        e.call(0x00ac_b200, &args![tree, out, key]);
        assert_eq!(e.mem.u32(out), e.mem.u32(tree));
        assert_eq!(e.mem.u32(out + 4), d);
        let key = new_string(&mut e, b"c");
        e.call(0x00ac_b200, &args![tree, out, key]);
        assert_eq!(e.mem.u32(out + 4), head);
    }

    #[test]
    fn n060_00acb1a0_00acf0b0_00ad5490_clear_the_tree() {
        let mut e = Engine::new();
        tree_doubles(&mut e);
        let tree = new_tree(&mut e);
        for key in [b"x", b"y", b"w"] {
            insert(&mut e, tree, key, b"v");
        }
        logging(&mut e);
        e.call(0x00ac_b1a0, &args![tree]);
        let head = e.mem.u32(tree + 0x18);
        assert_eq!(e.mem.u32(tree + 0x1c), 0);
        for offset in [0, 4, 8] {
            assert_eq!(e.mem.u32(head + offset), head);
        }
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 3);
        assert_eq!(calls_to(&e, STRING_TIDY_0044A730).len(), 6);
        let node = e.mem.alloc(0x48);
        logging(&mut e);
        e.call(0x00ad_5490, &args![node]);
        assert_eq!(
            calls_to(&e, STRING_TIDY_0044A730),
            vec![vec![node + 0x28, 1, 0], vec![node + 0xc, 1, 0]]
        );
    }

    #[test]
    fn n060_00ad1a50_builds_a_checked_iterator() {
        let mut e = Engine::new();
        let tree = new_tree(&mut e);
        let iterator = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00ad_1a50, &args![iterator, 0x5555u32, tree]).u32(),
            iterator
        );
        assert_eq!(e.mem.u32(iterator), e.mem.u32(tree));
        assert_eq!(e.mem.u32(iterator + 4), 0x5555);
        stub(&mut e, &[INVALID_PARAMETER]);
        e.map(0, 4);
        logging(&mut e);
        e.call(0x00ad_1a50, &args![iterator, 0x5555u32, 0u32]);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn n060_00ad3450_steps_back_and_checks_the_head() {
        let mut e = Engine::new();
        tree_doubles(&mut e);
        let tree = new_tree(&mut e);
        let (a, _) = insert(&mut e, tree, b"a", b"v");
        let (b, _) = insert(&mut e, tree, b"b", b"v");
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, e.mem.u32(tree));
        e.mem.set_u32(iterator + 4, b);
        e.call(0x00ad_3450, &args![iterator]);
        assert_eq!(e.mem.u32(iterator + 4), a);
        // Before the first node the parent walk ends on the head, without a report.
        stub(&mut e, &[INVALID_PARAMETER]);
        logging(&mut e);
        e.call(0x00ad_3450, &args![iterator]);
        assert_eq!(e.mem.u32(iterator + 4), e.mem.u32(tree + 0x18));
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
        // From the head of an empty tree: reported.
        let empty = new_tree(&mut e);
        let head = e.mem.u32(empty + 0x18);
        e.mem.set_u32(iterator + 4, head);
        e.call(0x00ad_3450, &args![iterator]);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
        logging(&mut e);
        // A null proxy is reported too.
        e.mem.set_u32(iterator, 0);
        e.mem.set_u32(iterator + 4, b);
        logging(&mut e);
        e.call(0x00ad_3450, &args![iterator]);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
        assert_eq!(e.mem.u32(iterator + 4), a);
    }

    #[test]
    fn n060_00ad1800_and_00ad18b0_rotate() {
        let mut e = Engine::new();
        let tree = new_tree(&mut e);
        let head = e.mem.u32(tree + 0x18);
        let node = |e: &mut Engine| {
            let n = e.mem.alloc(0x48);
            for offset in [0, 8] {
                e.mem.set_u32(n + offset, head);
            }
            n
        };
        let x = node(&mut e);
        let y = node(&mut e);
        let inner = node(&mut e);
        e.mem.set_u32(head + 4, x);
        e.mem.set_u32(x + 4, head);
        e.mem.set_u32(x + 8, y);
        e.mem.set_u32(y + 4, x);
        e.mem.set_u32(y, inner);
        e.mem.set_u32(inner + 4, y);
        e.call(0x00ad_1800, &args![tree, x]);
        assert_eq!(e.mem.u32(head + 4), y);
        assert_eq!(e.mem.u32(y), x);
        assert_eq!(e.mem.u32(x + 4), y);
        assert_eq!(e.mem.u32(x + 8), inner);
        assert_eq!(e.mem.u32(inner + 4), x);
        assert_eq!(e.mem.u32(y + 4), head);
        e.call(0x00ad_18b0, &args![tree, y]);
        assert_eq!(e.mem.u32(head + 4), x);
        assert_eq!(e.mem.u32(x + 8), y);
        assert_eq!(e.mem.u32(y + 4), x);
        assert_eq!(e.mem.u32(y), inner);
        assert_eq!(e.mem.u32(inner + 4), y);
    }

    #[test]
    fn n060_00ad5310_00ad5370_00ad53d0_00ad5430_allocate_or_throw() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (a[0] | 0x1000_0000).into_ret());
        stub(
            &mut e,
            &[EXCEPTION_FROM_MESSAGE_00EC78F1, CXX_THROW_EXCEPTION],
        );
        for (address, element) in [
            (0x00ad_5310u32, 0x1cu32),
            (0x00ad_5370, 0x48),
            (0x00ad_53d0, 0x38),
            (0x00ad_5430, 0x18),
        ] {
            logging(&mut e);
            assert_eq!(
                e.call(address, &args![3u32, 0u32]).u32(),
                (3 * element) | 0x1000_0000
            );
            assert!(calls_to(&e, CXX_THROW_EXCEPTION).is_empty());
            assert_eq!(e.call(address, &args![0u32, 0u32]).u32(), 0x1000_0000);
            e.call(address, &args![0x1000_0000u32, 0u32]);
            let thrown = calls_to(&e, CXX_THROW_EXCEPTION);
            assert_eq!(thrown.len(), 1);
            assert_eq!(thrown[0][1], BAD_ALLOC_THROW_INFO_01140EF8);
        }
    }

    #[test]
    fn n060_00acb310_and_00ad3000_build_pairs() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        let first = new_string(&mut e, b"key");
        let second = new_string(&mut e, b"value");
        let pair = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00ac_b310, &args![pair, first, second]).u32(), pair);
        assert_eq!(read_string(&e, pair), b"key");
        assert_eq!(read_string(&e, pair + 0x1c), b"value");
        let copy = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00ad_3000, &args![copy, pair]).u32(), copy);
        assert_eq!(read_string(&e, copy), b"key");
        assert_eq!(read_string(&e, copy + 0x1c), b"value");
    }

    fn pair_range_doubles(e: &mut Engine) {
        e.register(COPY_CONSTRUCT_PAIRS_00AD6D30, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                for offset in [0, 0x1c] {
                    let text = read_string(e, from + offset);
                    set_string(e, to + offset, &text);
                }
                from += 0x38;
                to += 0x38;
            }
            to.into_ret()
        });
    }

    #[test]
    fn n060_00acb3c0_appends_pairs() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        allocator_doubles(&mut e);
        pair_range_doubles(&mut e);
        let vector = new_vector(&mut e, 0x38, 0, 1);
        let pair = new_pair(&mut e, b"k", b"v");
        e.call(0x00ac_b3c0, &args![vector, pair]);
        let first = e.mem.u32(vector + 0xc);
        assert_eq!(e.mem.u32(vector + 0x10), first + 0x38);
        assert_eq!(read_string(&e, first + 0x1c), b"v");
        let pair = new_pair(&mut e, b"k2", b"v2");
        e.call(0x00ac_b3c0, &args![vector, pair]);
        let first = e.mem.u32(vector + 0xc);
        assert_eq!(e.mem.u32(vector + 0x10), first + 0x70);
        assert_eq!(read_string(&e, first), b"k");
        assert_eq!(read_string(&e, first + 0x38), b"k2");
    }

    #[test]
    fn n060_00acb4d0_00acf900_00ad2500_open_a_string_stream() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        allocator_doubles(&mut e);
        stream_doubles(&mut e);
        stub(&mut e, &[0x00ec_3246, 0x00ec_31b3]);
        let implementation = ctype_world(&mut e);
        returns(&mut e, LOCALE_INIT, implementation);
        let global = e.mem.alloc(8);
        returns(&mut e, GLOBAL_LOCALE_00EC34D4, global);
        e.register(ALLOCATE_CHARACTER_BLOCK_0065FA30, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(MEMCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.map(VBTABLE_ISTRINGSTREAM_010A30B4, 8);
        e.mem.set_u32(VBTABLE_ISTRINGSTREAM_010A30B4 + 4, 0x50);
        let object = e.mem.alloc(0x90);
        let text = new_string(&mut e, b"ab");
        assert_eq!(
            e.call(0x00ac_b4d0, &args![object, text, 0u32, 1u32]).u32(),
            object
        );
        assert_eq!(e.mem.u32(object), VBTABLE_ISTRINGSTREAM_010A30B4);
        assert_eq!(e.mem.u32(object + 0x50), VTABLE_ISTRINGSTREAM_010A30A0);
        let buffer = object + 8;
        assert_eq!(e.mem.u32(object + 0x50 + 0x28), buffer);
        assert_eq!(e.mem.u32(buffer), VTABLE_STRINGBUF_010A30C0);
        // `in` only: the state is "no writing" (2) plus owning (1); the read area is the copy.
        assert_eq!(e.mem.u32(buffer + 0x40), 3);
        let next = slot_word(&e, buffer, 0x20);
        assert_eq!(e.mem.bytes(next, 2), b"ab");
        assert_eq!(slot_word(&e, buffer, 0x30), 2);
        assert_eq!(slot_word(&e, buffer, 0x24), 0);
        assert_eq!(e.mem.u32(buffer + 0x3c), next + 2);
    }

    #[test]
    fn n060_00ad2500_sets_up_the_write_area() {
        let mut e = Engine::new();
        e.register(ALLOCATE_CHARACTER_BLOCK_0065FA30, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        stub(&mut e, &[MEMCPY_S]);
        let buffer = new_buffer(&mut e, 0x50);
        let text = new_c(&mut e, b"abc");
        // "No reading" and "at end": only the write area, its pointer at the end.
        e.call(0x00ad_2500, &args![buffer, text, 3u32, 0x14u32]);
        let start = slot_word(&e, buffer, 0x14);
        assert_eq!(slot_word(&e, buffer, 0x24), start + 3);
        assert_eq!(slot_word(&e, buffer, 0x34), 0);
        assert_eq!(slot_word(&e, buffer, 0x10), start);
        assert_eq!(slot_word(&e, buffer, 0x20), 0);
        assert_eq!(slot_word(&e, buffer, 0x30), start);
        assert_eq!(e.mem.u32(buffer + 0x40), 0x15);
        // Constant and no reading: nothing allocated.
        logging(&mut e);
        e.call(0x00ad_2500, &args![buffer, text, 3u32, 6u32]);
        assert!(calls_to(&e, ALLOCATE_CHARACTER_BLOCK_0065FA30).is_empty());
        assert_eq!(e.mem.u32(buffer + 0x40), 6);
        assert_eq!(e.mem.u32(buffer + 0x3c), 0);
    }

    #[test]
    fn n060_00acb5f0_stores_the_istream_vtable() {
        let mut e = Engine::new();
        let (object, ios) = stream_object(&mut e, 0x10, 8);
        e.call(0x00ac_b5f0, &args![object + 8]);
        assert_eq!(e.mem.u32(ios), VTABLE_ISTREAM_010A30A8);
    }

    #[test]
    fn n060_00acb7d0_and_00acbb60_answer_constants() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00ac_b7d0, &args![0x10u32, 5u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(e.call(0x00ac_bb60, &args![0x10u32, 5u32, 6u32]).u32(), 0x10);
    }

    #[test]
    fn n060_00acb7f0_takes_after_underflow() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let buffer = new_buffer(&mut e, 0x50);
        let data = set_get_area(&mut e, buffer, b"pq");
        e.register(slot_target(0x10), |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ac_b7f0, &args![buffer]).u32(), u32::from(b'p'));
        assert_eq!(slot_word(&e, buffer, 0x20), data + 1);
        e.register(slot_target(0x10), |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(e.call(0x00ac_b7f0, &args![buffer]).u32(), 0xffff_ffff);
        assert_eq!(slot_word(&e, buffer, 0x20), data + 1);
    }

    #[test]
    fn n060_00acb850_forwards_to_slot_0x1c() {
        let mut e = Engine::new();
        let buffer = new_buffer(&mut e, 0x50);
        e.register(slot_target(0x1c), |_, a| {
            a[1].wrapping_add(a[2]).wrapping_add(a[3]).into_ret()
        });
        logging(&mut e);
        assert_eq!(e.call(0x00ac_b850, &args![buffer, 5u32, 7u32]).u32(), 11);
        assert_eq!(
            calls_to(&e, slot_target(0x1c)),
            vec![vec![buffer, 5, 0xffff_ffff, 7]]
        );
    }

    #[test]
    fn n060_00acb880_copies_then_takes_single_characters() {
        let mut e = Engine::new();
        e.register(MEMCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let buffer = new_buffer(&mut e, 0x50);
        set_get_area(&mut e, buffer, b"ab");
        e.register_double(slot_target(0x14), {
            let mut left = 1;
            move |_, _| {
                if left > 0 {
                    left -= 1;
                    u32::from(b'c').into_ret()
                } else {
                    0xffff_ffffu32.into_ret()
                }
            }
        });
        let out = e.mem.alloc(8);
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_b880, &args![buffer, out, 8u32, 5u32]).u32(),
            3
        );
        assert_eq!(e.mem.bytes(out, 3), b"abc");
        assert_eq!(calls_to(&e, MEMCPY_S)[0][1], 8);
        assert_eq!(slot_word(&e, buffer, 0x30), 0);
        assert_eq!(
            e.call(0x00ac_b880, &args![buffer, out, 8u32, 0u32]).u32(),
            0
        );
    }

    #[test]
    fn n060_00acb9b0_copies_then_overflows() {
        let mut e = Engine::new();
        e.register(MEMCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let buffer = new_buffer(&mut e, 0x50);
        let area = set_put_area(&mut e, buffer, 2);
        e.register(slot_target(4), |_, a| {
            if a[1] == u32::from(b'z') {
                0xffff_ffffu32.into_ret()
            } else {
                a[1].into_ret()
            }
        });
        let text = new_c(&mut e, b"xyqz");
        logging(&mut e);
        assert_eq!(e.call(0x00ac_b9b0, &args![buffer, text, 4u32]).u32(), 3);
        assert_eq!(e.mem.bytes(area, 2), b"xy");
        assert_eq!(calls_to(&e, MEMCPY_S)[0][1], 2);
        assert_eq!(
            calls_to(&e, slot_target(4)),
            vec![vec![buffer, u32::from(b'q')], vec![buffer, u32::from(b'z')]]
        );
    }

    #[test]
    fn n060_00acf9e0_frees_an_owned_buffer() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE]);
        let buffer = new_buffer(&mut e, 0x50);
        let area = set_put_area(&mut e, buffer, 4);
        let target = e.mem.u32(buffer + 0x10);
        e.mem.set_u32(target, area);
        e.mem.set_u32(buffer + 0x40, 5);
        e.mem.set_u32(buffer + 0x3c, 0x77);
        logging(&mut e);
        e.call(0x00ac_f9e0, &args![buffer]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![area]]);
        for slot in [0x10, 0x14, 0x20, 0x24, 0x30, 0x34] {
            assert_eq!(slot_word(&e, buffer, slot), 0);
        }
        assert_eq!(e.mem.u32(buffer + 0x3c), 0);
        assert_eq!(e.mem.u32(buffer + 0x40), 4);
        logging(&mut e);
        e.call(0x00ac_f9e0, &args![buffer]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn n060_00acbae0_and_00acbb20_answer_the_bad_position() {
        let mut e = Engine::new();
        e.map(BAD_OFFSET_010F1E40, 4);
        e.map(ZERO_STATE_01011D78, 4);
        e.set_global(BAD_OFFSET_010F1E40, 0xffff_ffffu32);
        e.set_global(ZERO_STATE_01011D78, 0x1234u32);
        let out = e.mem.alloc(0x18);
        e.mem.write(out, &[0xcc; 0x18]);
        assert_eq!(
            e.call(0x00ac_bae0, &args![0u32, out, 1u32, 2u32, 3u32])
                .u32(),
            out
        );
        assert_eq!(e.mem.u32(out), 0xffff_ffff);
        assert_eq!(e.mem.u32(out + 4), 0xcccc_cccc);
        assert_eq!(e.mem.u32(out + 8), 0);
        assert_eq!(e.mem.u32(out + 0xc), 0);
        assert_eq!(e.mem.u32(out + 0x10), 0x1234);
        let other = e.mem.alloc(0x18);
        let words = args![0u32, other, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32];
        assert_eq!(e.call(0x00ac_bb20, &words).u32(), other);
        assert_eq!(e.mem.u32(other + 0x10), 0x1234);
    }

    /// A string buffer with `mode` at `+0x40`, a write area over `text` with `room` more bytes
    /// and the read area over the same text.
    fn string_buffer(e: &mut Engine, text: &[u8], room: u32, mode: u32) -> (u32, u32) {
        let buffer = new_buffer(e, 0x50);
        let data = e.mem.alloc(text.len() as u32 + room + 1);
        e.mem.write(data, text);
        let length = text.len() as u32;
        for (slot, value) in [
            (0x10, data),
            (0x20, data),
            (0x30, length),
            (0x14, data),
            (0x24, data + length),
            (0x34, room),
        ] {
            let target = e.mem.u32(buffer + slot);
            e.mem.set_u32(target, value);
        }
        e.mem.set_u32(buffer + 0x40, mode);
        e.mem.set_u32(buffer + 0x3c, data + length);
        (buffer, data)
    }

    #[test]
    fn n060_00acbb80_stores_or_grows() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE]);
        e.register(ALLOCATE_CHARACTERS, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let (buffer, data) = string_buffer(&mut e, b"ab", 1, 1);
        assert_eq!(e.call(0x00ac_bb80, &args![buffer, 0xffff_ffffu32]).u32(), 0);
        assert_eq!(
            e.call(0x00ac_bb80, &args![buffer, u32::from(b'c')]).u32(),
            u32::from(b'c')
        );
        assert_eq!(e.mem.u8(data + 2), b'c');
        // Full: grows by 0x20, copies and moves every pointer, frees the owned old one.
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_bb80, &args![buffer, u32::from(b'd')]).u32(),
            u32::from(b'd')
        );
        assert_eq!(calls_to(&e, ALLOCATE_CHARACTERS)[0][1], 3 + 0x20);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![data]]);
        let fresh = slot_word(&e, buffer, 0x10);
        assert_eq!(e.mem.bytes(fresh, 4), b"abcd");
        assert_eq!(slot_word(&e, buffer, 0x24), fresh + 4);
        assert_eq!(slot_word(&e, buffer, 0x34), 0x20 - 1);
        assert_eq!(slot_word(&e, buffer, 0x30), fresh + 3 + 1 - fresh);
        // A constant buffer does not grow.
        let (buffer, _) = string_buffer(&mut e, b"ab", 0, 2);
        assert_eq!(
            e.call(0x00ac_bb80, &args![buffer, 0x41u32]).u32(),
            0xffff_ffff
        );
        // An empty one gets a first block (with no reading: an empty read area).
        let buffer = new_buffer(&mut e, 0x50);
        e.mem.set_u32(buffer + 0x40, 4);
        assert_eq!(e.call(0x00ac_bb80, &args![buffer, 0x41u32]).u32(), 0x41);
        let fresh = slot_word(&e, buffer, 0x14);
        assert_eq!(e.mem.u8(fresh), 0x41);
        assert_eq!(slot_word(&e, buffer, 0x20), 0);
        assert_eq!(slot_word(&e, buffer, 0x30), fresh);
        assert_eq!(e.mem.u32(buffer + 0x40), 5);
    }

    #[test]
    fn n060_00acbf60_puts_back() {
        let mut e = Engine::new();
        let (buffer, data) = string_buffer(&mut e, b"ab", 0, 0);
        assert_eq!(
            e.call(0x00ac_bf60, &args![buffer, 0x41u32]).u32(),
            0xffff_ffff
        );
        let target = e.mem.u32(buffer + 0x20);
        e.mem.set_u32(target, data + 2);
        e.mem.set_u32(e.mem.u32(buffer + 0x30), 0);
        assert_eq!(
            e.call(0x00ac_bf60, &args![buffer, u32::from(b'x')]).u32(),
            u32::from(b'x')
        );
        assert_eq!(slot_word(&e, buffer, 0x20), data + 1);
        assert_eq!(slot_word(&e, buffer, 0x30), 1);
        assert_eq!(e.mem.u8(data + 1), b'x');
        assert_eq!(e.call(0x00ac_bf60, &args![buffer, 0xffff_ffffu32]).u32(), 0);
        assert_eq!(slot_word(&e, buffer, 0x20), data);
        // A constant buffer only takes back the same character.
        let (buffer, data) = string_buffer(&mut e, b"ab", 0, 2);
        let target = e.mem.u32(buffer + 0x20);
        e.mem.set_u32(target, data + 1);
        assert_eq!(
            e.call(0x00ac_bf60, &args![buffer, u32::from(b'x')]).u32(),
            0xffff_ffff
        );
        assert_eq!(
            e.call(0x00ac_bf60, &args![buffer, u32::from(b'a')]).u32(),
            u32::from(b'a')
        );
    }

    #[test]
    fn n060_00acc070_reads_or_extends_to_the_mark() {
        let mut e = Engine::new();
        let (buffer, data) = string_buffer(&mut e, b"ab", 4, 0);
        assert_eq!(e.call(0x00ac_c070, &args![buffer]).u32(), u32::from(b'a'));
        // Read area used up while writes went further: extended to the write pointer.
        e.mem.set_u32(e.mem.u32(buffer + 0x30), 0);
        let target = e.mem.u32(buffer + 0x24);
        e.mem.set_u32(target, data + 2);
        e.mem.write(data, b"ab");
        e.mem.set_u32(buffer + 0x3c, data);
        assert_eq!(e.call(0x00ac_c070, &args![buffer]).u32(), u32::from(b'a'));
        assert_eq!(e.mem.u32(buffer + 0x3c), data + 2);
        assert_eq!(slot_word(&e, buffer, 0x30), 2);
        // No reading: -1.
        e.mem.set_u32(e.mem.u32(buffer + 0x30), 0);
        e.mem.set_u32(buffer + 0x40, 4);
        assert_eq!(e.call(0x00ac_c070, &args![buffer]).u32(), 0xffff_ffff);
        let empty = new_buffer(&mut e, 0x50);
        assert_eq!(e.call(0x00ac_c070, &args![empty]).u32(), 0xffff_ffff);
    }

    fn seek_globals(e: &mut Engine) {
        e.map(BAD_OFFSET_010F1E40, 4);
        e.map(ZERO_STATE_01011D78, 4);
        e.set_global(BAD_OFFSET_010F1E40, 0xffff_ffffu32);
    }

    #[test]
    fn n060_00acc1b0_seeks_by_offset() {
        let mut e = Engine::new();
        seek_globals(&mut e);
        let (buffer, data) = string_buffer(&mut e, b"abcd", 0, 0);
        let out = e.mem.alloc(0x18);
        // Read area from the beginning; the write pointer follows with `out`.
        e.call(0x00ac_c1b0, &args![buffer, out, 1u32, 0u32, 3u32]);
        assert_eq!(e.mem.u32(out), 1);
        assert_eq!(slot_word(&e, buffer, 0x20), data + 1);
        assert_eq!(slot_word(&e, buffer, 0x30), 3);
        assert_eq!(slot_word(&e, buffer, 0x24), data + 1);
        assert_eq!(slot_word(&e, buffer, 0x34), 3);
        // From the end, read only.
        e.call(0x00ac_c1b0, &args![buffer, out, 0xffff_ffffu32, 2u32, 1u32]);
        assert_eq!(e.mem.u32(out), 3);
        assert_eq!(slot_word(&e, buffer, 0x20), data + 3);
        // Current with both asked: the bad offset.
        e.call(0x00ac_c1b0, &args![buffer, out, 0u32, 1u32, 3u32]);
        assert_eq!(e.mem.u32(out), 0xffff_ffff);
        // Write area only, relative to the write pointer.
        e.call(0x00ac_c1b0, &args![buffer, out, 1u32, 1u32, 2u32]);
        assert_eq!(e.mem.u32(out), 2);
        assert_eq!(slot_word(&e, buffer, 0x24), data + 2);
        // Out of range.
        e.call(0x00ac_c1b0, &args![buffer, out, 9u32, 0u32, 1u32]);
        assert_eq!(e.mem.u32(out), 0xffff_ffff);
        // Neither area.
        e.call(0x00ac_c1b0, &args![buffer, out, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(out), 0xffff_ffff);
    }

    #[test]
    fn n060_00acc490_seeks_to_a_position() {
        let mut e = Engine::new();
        seek_globals(&mut e);
        let (buffer, data) = string_buffer(&mut e, b"abcd", 0, 0);
        let out = e.mem.alloc(0x18);
        let call = |e: &mut Engine, offset: u32, low: u32, mode: u32| {
            let words = args![buffer, out, offset, 0u32, low, 0u32, 0u32, 0u32, mode];
            e.call(0x00ac_c490, &words);
            e.mem.u32(out)
        };
        assert_eq!(call(&mut e, 1, 1, 3), 2);
        assert_eq!(slot_word(&e, buffer, 0x20), data + 2);
        assert_eq!(slot_word(&e, buffer, 0x24), data + 2);
        assert_eq!(call(&mut e, 3, 0, 2), 3);
        assert_eq!(slot_word(&e, buffer, 0x24), data + 3);
        assert_eq!(call(&mut e, 0xffff_ffff, 0, 3), 0xffff_ffff);
        assert_eq!(call(&mut e, 5, 0, 1), 0xffff_ffff);
        assert_eq!(call(&mut e, 1, 0, 0), 0xffff_ffff);
    }

    /// The doubles a file stream's construction needs: the locale (with the cached `ctype`
    /// facet), the mutex, the global vtables' virtual-base tables.
    fn file_stream_doubles(e: &mut Engine) {
        allocator_doubles(e);
        stream_doubles(e);
        stub(e, &[0x00ec_3246, 0x00ec_31b3]);
        let implementation = ctype_world(e);
        returns(e, LOCALE_INIT, implementation);
        let global = e.mem.alloc(8);
        returns(e, GLOBAL_LOCALE_00EC34D4, global);
        e.map(INITIAL_STATE_011F6D8C, 4);
        e.map(VBTABLE_IFSTREAM_010A3178, 8);
        e.mem.set_u32(VBTABLE_IFSTREAM_010A3178 + 4, 0x58);
        e.map(VBTABLE_OFSTREAM_010A3198, 8);
        e.mem.set_u32(VBTABLE_OFSTREAM_010A3198 + 4, 0x54);
    }

    #[test]
    fn n060_00acc6c0_00acfab0_open_an_input_file() {
        let mut e = Engine::new();
        file_stream_doubles(&mut e);
        returns(&mut e, FIOPEN, 0x7000);
        let converter = e.mem.alloc(8);
        e.put_vtable(0x0320_0000, &[0, 0x0320_1000]);
        e.mem.set_u32(converter, 0x0320_0000);
        e.register(0x0320_1000, |_, _| 1u32.into_ret());
        e.map(CACHED_CODECVT_011F6D94, 4);
        e.set_global(CACHED_CODECVT_011F6D94, converter);
        e.map(CODECVT_ID_011C33E0, 4);
        let object = e.mem.alloc(0x90);
        let name = new_c(&mut e, b"faces.txt");
        logging(&mut e);
        e.call(0x00ac_c6c0, &args![object, name, 0u32, 0x40u32, 1u32]);
        assert_eq!(calls_to(&e, FIOPEN), vec![vec![name, 1, 0x40]]);
        assert_eq!(e.mem.u32(object + 0x58), VTABLE_IFSTREAM_010A3174);
        assert_eq!(e.mem.u32(object + 8), VTABLE_FILEBUF_010A3138);
        assert_eq!(e.mem.u32(object + 8 + 0x4c), 0x7000);
        assert_eq!(e.mem.u32(object + 8 + 0x3c), 0);
        assert_eq!(e.mem.u32(object + 0x58 + 8), 0);
        // A failed open sets failbit.
        returns(&mut e, FIOPEN, 0);
        let object = e.mem.alloc(0x90);
        e.call(0x00ac_c6c0, &args![object, name, 0u32, 0x40u32, 1u32]);
        assert_eq!(e.mem.u32(object + 0x58 + 8), 2);
    }

    /// A file buffer: the stream-buffer slots initialised, the file at `+0x4c`, the converter
    /// at `+0x3c`.
    fn file_buffer(e: &mut Engine, file: u32, converter: u32) -> u32 {
        let buffer = new_buffer(e, 0x50);
        e.mem.set_u32(buffer + 0x4c, file);
        e.mem.set_u32(buffer + 0x3c, converter);
        buffer
    }

    /// A converter whose slots `0x10` (`in`), `0x14` (`out`) and `0x18` (`unshift`) are the
    /// doubles at 0x0330_0010, 0x0330_0014 and 0x0330_0018, slot 4 `always_noconv`.
    fn new_converter(e: &mut Engine) -> u32 {
        let slots: Vec<u32> = (0..8).map(|i| 0x0330_0000 + 4 * i).collect();
        e.put_vtable(0x0331_0000, &slots);
        let converter = e.mem.alloc(8);
        e.mem.set_u32(converter, 0x0331_0000);
        converter
    }

    fn file_doubles(e: &mut Engine) {
        string_doubles(e);
        e.register(STRING_GROW_0044A8D0, |_, _| 1u32.into_ret());
        e.register(STRING_FILL_00AAE180, |e, a| {
            set_string(e, a[0], &vec![a[3] as u8; a[2] as usize]);
            Ret::default()
        });
        stub(e, &[INVALID_PARAMETER]);
        e.register(CRT_FPUTC, |_, a| a[0].into_ret());
        e.register(CRT_FWRITE, |_, a| a[2].into_ret());
        e.register(CRT_UNGETC, |_, a| a[0].into_ret());
    }

    #[test]
    fn n060_00acc9f0_overflows_through_the_file() {
        let mut e = Engine::new();
        file_doubles(&mut e);
        // Room in the put area.
        let buffer = file_buffer(&mut e, 0x6000, 0);
        let area = set_put_area(&mut e, buffer, 2);
        assert_eq!(e.call(0x00ac_c9f0, &args![buffer, 0x61u32]).u32(), 0x61);
        assert_eq!(e.mem.u8(area), 0x61);
        assert_eq!(e.call(0x00ac_c9f0, &args![buffer, 0xffff_ffffu32]).u32(), 0);
        // No room, no converter: fputc (sign-extended).
        set_put_area(&mut e, buffer, 0);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_c9f0, &args![buffer, 0xe9u32]).u32(), 0xe9);
        assert_eq!(calls_to(&e, CRT_FPUTC), vec![vec![0xffff_ffe9, 0x6000]]);
        e.register(CRT_FPUTC, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(
            e.call(0x00ac_c9f0, &args![buffer, 0x62u32]).u32(),
            0xffff_ffff
        );
        // No file: -1.
        let closed = file_buffer(&mut e, 0, 0);
        assert_eq!(
            e.call(0x00ac_c9f0, &args![closed, 0x62u32]).u32(),
            0xffff_ffff
        );
        // A converter that produces two bytes: written, pending output marked.
        e.register(CRT_FPUTC, |_, a| a[0].into_ret());
        let converter = new_converter(&mut e);
        e.register(0x0330_0014, |e, a| {
            // out(state, from, from_end, from_next, to, to_end, to_next)
            e.mem.set_u32(a[4], a[3]);
            e.mem.set_u8(a[5], b'<');
            e.mem.set_u8(a[5] + 1, b'>');
            e.mem.set_u32(a[7], a[5] + 2);
            1u32.into_ret()
        });
        let buffer = file_buffer(&mut e, 0x6000, converter);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_c9f0, &args![buffer, 0x63u32]).u32(), 0x63);
        let written = calls_to(&e, CRT_FWRITE);
        assert_eq!(written.len(), 1);
        assert_eq!(written[0][1..], [1, 2, 0x6000]);
        assert_eq!(e.mem.u8(buffer + 0x41), 1);
        assert_eq!(
            calls_to(&e, STRING_TIDY_0044A730).last().unwrap()[1..],
            [1, 0]
        );
        // `noconv` (3): fputc of the character.
        e.register(0x0330_0014, |_, _| 3u32.into_ret());
        logging(&mut e);
        assert_eq!(e.call(0x00ac_c9f0, &args![buffer, 0x64u32]).u32(), 0x64);
        assert_eq!(calls_to(&e, CRT_FPUTC), vec![vec![0x64, 0x6000]]);
        // An error result: -1.
        e.register(0x0330_0014, |_, _| 2u32.into_ret());
        assert_eq!(
            e.call(0x00ac_c9f0, &args![buffer, 0x64u32]).u32(),
            0xffff_ffff
        );
    }

    #[test]
    fn n060_00ad0360_fills_the_string() {
        let mut e = Engine::new();
        e.map(STRING_NPOS_010173F0, 4);
        e.set_global(STRING_NPOS_010173F0, 0xffff_ffffu32);
        returns(&mut e, STRING_GROW_0044A8D0, 1);
        stub(&mut e, &[STRING_FILL_00AAE180, STRING_TOO_LONG_00EC30B0]);
        let string = new_string(&mut e, b"zzzzzzzzzz");
        logging(&mut e);
        assert_eq!(
            e.call(0x00ad_0360, &args![string, 3u32, 0x178u32]).u32(),
            string
        );
        assert_eq!(calls_to(&e, STRING_GROW_0044A8D0), vec![vec![string, 3, 0]]);
        assert_eq!(
            calls_to(&e, STRING_FILL_00AAE180),
            vec![vec![string, 0, 3, 0x78]]
        );
        assert_eq!(e.mem.u32(string + 0x14), 3);
        assert_eq!(e.mem.u8(string_chars(&e, string) + 3), 0);
        returns(&mut e, STRING_GROW_0044A8D0, 0x100);
        logging(&mut e);
        e.call(0x00ad_0360, &args![string, 0xffff_ffffu32, 0u32]);
        assert_eq!(calls_to(&e, STRING_TOO_LONG_00EC30B0).len(), 1);
        assert!(calls_to(&e, STRING_FILL_00AAE180).is_empty());
    }

    #[test]
    fn n060_00ad3550_and_00ad2f30_check_string_iterators() {
        let mut e = Engine::new();
        stub(&mut e, &[INVALID_PARAMETER]);
        let string = new_string(&mut e, b"abc");
        let data = string_chars(&e, string);
        let iterator = e.mem.alloc(8);
        logging(&mut e);
        e.call(0x00ad_3550, &args![iterator, data + 1, string]);
        assert_eq!(e.mem.u32(iterator), string);
        assert_eq!(e.call(0x00ad_2f30, &args![iterator]).u32(), data + 1);
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
        // The end is a valid position but not a valid element.
        e.call(0x00ad_3550, &args![iterator, data + 3, string]);
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
        e.call(0x00ad_2f30, &args![iterator]);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
        logging(&mut e);
        e.call(0x00ad_3550, &args![iterator, data + 4, string]);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
        // The -4 marker skips the checks.
        e.mem.set_u32(iterator, 0xffff_fffc);
        e.mem.set_u32(iterator + 4, 0x1234);
        logging(&mut e);
        assert_eq!(e.call(0x00ad_2f30, &args![iterator]).u32(), 0x1234);
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
    }

    #[test]
    fn n060_00accef0_00ace930_put_back_into_the_file_buffer() {
        let mut e = Engine::new();
        file_doubles(&mut e);
        let buffer = file_buffer(&mut e, 0x6000, 0);
        let data = set_get_area(&mut e, buffer, b"ab");
        let target = e.mem.u32(buffer + 0x20);
        e.mem.set_u32(target, data + 1);
        // The same character before the read pointer: stepped back.
        assert_eq!(
            e.call(0x00ac_cef0, &args![buffer, u32::from(b'a')]).u32(),
            u32::from(b'a')
        );
        assert_eq!(slot_word(&e, buffer, 0x20), data);
        assert_eq!(slot_word(&e, buffer, 0x30), 3);
        // At the start: ungetc to the file.
        logging(&mut e);
        assert_eq!(e.call(0x00ac_cef0, &args![buffer, 0x1ffu32]).u32(), 0x1ff);
        assert_eq!(calls_to(&e, CRT_UNGETC), vec![vec![0xff, 0x6000]]);
        e.register(CRT_UNGETC, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(
            e.call(0x00ac_cef0, &args![buffer, 0x41u32]).u32(),
            0xffff_ffff
        );
        // -1 without a step back, or a converter: -1.
        assert_eq!(
            e.call(0x00ac_cef0, &args![buffer, 0xffff_ffffu32]).u32(),
            0xffff_ffff
        );
        e.mem.set_u32(buffer + 0x3c, 0x9000);
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_cef0, &args![buffer, 0x41u32]).u32(),
            0xffff_ffff
        );
        assert!(calls_to(&e, CRT_UNGETC).is_empty());
    }

    #[test]
    fn n060_00acd060_peeks_through_uflow_and_pbackfail() {
        let mut e = Engine::new();
        let buffer = new_buffer(&mut e, 0x50);
        set_get_area(&mut e, buffer, b"r");
        assert_eq!(e.call(0x00ac_d060, &args![buffer]).u32(), u32::from(b'r'));
        set_get_area(&mut e, buffer, b"");
        e.register(slot_target(0x14), |_, _| 0x73u32.into_ret());
        e.register(slot_target(8), quiet);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_d060, &args![buffer]).u32(), 0x73);
        assert_eq!(calls_to(&e, slot_target(8)), vec![vec![buffer, 0x73]]);
        e.register(slot_target(0x14), |_, _| 0xffff_ffffu32.into_ret());
        logging(&mut e);
        assert_eq!(e.call(0x00ac_d060, &args![buffer]).u32(), 0xffff_ffff);
        assert!(calls_to(&e, slot_target(8)).is_empty());
    }

    #[test]
    fn n060_00acd100_takes_from_the_file() {
        let mut e = Engine::new();
        file_doubles(&mut e);
        e.register(MEMCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        // From the get area.
        let buffer = file_buffer(&mut e, 0x6000, 0);
        let data = set_get_area(&mut e, buffer, b"x");
        assert_eq!(e.call(0x00ac_d100, &args![buffer]).u32(), u32::from(b'x'));
        assert_eq!(slot_word(&e, buffer, 0x20), data + 1);
        // Without a converter: getc.
        e.register(CRT_GETC, |_, _| 0x1e5u32.into_ret());
        assert_eq!(e.call(0x00ac_d100, &args![buffer]).u32(), 0xe5);
        e.register(CRT_GETC, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(e.call(0x00ac_d100, &args![buffer]).u32(), 0xffff_ffff);
        // With a converter: two bytes make one character, the third goes back.
        let converter = new_converter(&mut e);
        let buffer = file_buffer(&mut e, 0x6000, converter);
        e.register_double(CRT_GETC, {
            let mut bytes = vec![b'1', b'2', b'3'].into_iter();
            move |_, _| bytes.next().map_or(0xffff_ffff, u32::from).into_ret()
        });
        e.register(0x0330_0010, |e, a| {
            // in(state, from, from_end, from_next, to, to_end, to_next)
            let available = a[3] - a[2];
            if available < 2 {
                e.mem.set_u32(a[4], a[2]);
                e.mem.set_u32(a[7], a[5]);
                return 1u32.into_ret();
            }
            e.mem.set_u8(a[5], b'#');
            e.mem.set_u32(a[4], a[2] + 1);
            e.mem.set_u32(a[7], a[5] + 1);
            0u32.into_ret()
        });
        logging(&mut e);
        assert_eq!(e.call(0x00ac_d100, &args![buffer]).u32(), u32::from(b'#'));
        assert_eq!(
            calls_to(&e, CRT_UNGETC),
            vec![vec![u32::from(b'2'), 0x6000]]
        );
        // `noconv`: the first byte read.
        e.register(CRT_GETC, |_, _| u32::from(b'q').into_ret());
        e.register(0x0330_0010, |_, _| 3u32.into_ret());
        assert_eq!(e.call(0x00ac_d100, &args![buffer]).u32(), u32::from(b'q'));
        // An error: -1.
        e.register(0x0330_0010, |_, _| 0xffff_fffeu32.into_ret());
        assert_eq!(e.call(0x00ac_d100, &args![buffer]).u32(), 0xffff_ffff);
    }

    #[test]
    fn n060_00acd5b0_seeks_the_file() {
        let mut e = Engine::new();
        seek_globals(&mut e);
        e.set_global(ZERO_STATE_01011D78, 0x99u32);
        e.register(CRT_FSEEK, |_, _| 0u32.into_ret());
        e.register(CRT_FGETPOS, |e, a| {
            e.mem.set_u32(a[1], 0x40);
            e.mem.set_u32(a[1] + 4, 1);
            0u32.into_ret()
        });
        let buffer = file_buffer(&mut e, 0x6000, 0);
        e.mem.set_u32(buffer + 0x44, 0x55);
        // A put-back character in the buffer's own byte counts for a relative seek.
        let target = e.mem.u32(buffer + 0x20);
        e.mem.set_u32(target, buffer + 0x40);
        let out = e.mem.alloc(0x18);
        logging(&mut e);
        e.call(0x00ac_d5b0, &args![buffer, out, 5u32, 1u32, 3u32]);
        assert_eq!(calls_to(&e, CRT_FSEEK), vec![vec![0x6000, 4, 1]]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u32(out + 8), 0x40);
        assert_eq!(e.mem.u32(out + 0xc), 1);
        assert_eq!(e.mem.u32(out + 0x10), 0x55);
        assert_eq!(slot_word(&e, buffer, 0x20), buffer + 0x41);
        assert_eq!(slot_word(&e, buffer, 0x30), 0);
        // A relative seek by 0 skips fseek.
        logging(&mut e);
        e.call(0x00ac_d5b0, &args![buffer, out, 0u32, 1u32, 3u32]);
        assert!(calls_to(&e, CRT_FSEEK).is_empty());
        // Failures: the bad position.
        e.register(CRT_FSEEK, |_, _| 1u32.into_ret());
        e.call(0x00ac_d5b0, &args![buffer, out, 5u32, 0u32, 3u32]);
        assert_eq!(e.mem.u32(out), 0xffff_ffff);
        assert_eq!(e.mem.u32(out + 0x10), 0x99);
        let closed = file_buffer(&mut e, 0, 0);
        e.call(0x00ac_d5b0, &args![closed, out, 0u32, 1u32, 3u32]);
        assert_eq!(e.mem.u32(out), 0xffff_ffff);
    }

    #[test]
    fn n060_00acd760_sets_the_file_position() {
        let mut e = Engine::new();
        seek_globals(&mut e);
        e.register(CRT_FSETPOS, |_, _| 0u32.into_ret());
        e.register(CRT_FSEEK, |_, _| 0u32.into_ret());
        e.register(CRT_FGETPOS, |e, a| {
            e.mem.set_u32(a[1], 0x80);
            0u32.into_ret()
        });
        let buffer = file_buffer(&mut e, 0x6000, 0);
        let out = e.mem.alloc(0x18);
        logging(&mut e);
        let words = args![buffer, out, 3u32, 0u32, 0x70u32, 0u32, 0x22u32, 0u32, 0u32];
        e.call(0x00ac_d760, &words);
        assert_eq!(calls_to(&e, CRT_FSEEK), vec![vec![0x6000, 3, 1]]);
        assert_eq!(calls_to(&e, CRT_FSETPOS).len(), 1);
        assert_eq!(e.mem.u32(buffer + 0x44), 0x22);
        assert_eq!(e.mem.u32(out + 8), 0x80);
        assert_eq!(e.mem.u32(out + 0x10), 0x22);
        e.register(CRT_FSETPOS, |_, _| 1u32.into_ret());
        e.call(0x00ac_d760, &words);
        assert_eq!(e.mem.u32(out), 0xffff_ffff);
    }

    #[test]
    fn n060_00acd930_sets_the_file_buffer() {
        let mut e = Engine::new();
        e.map(INITIAL_STATE_011F6D8C, 4);
        returns(&mut e, CRT_SETVBUF, 0);
        let buffer = file_buffer(&mut e, 0x6000, 0);
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_d930, &args![buffer, 0u32, 0u32]).u32(),
            buffer
        );
        assert_eq!(calls_to(&e, CRT_SETVBUF), vec![vec![0x6000, 0, 4, 0]]);
        assert_eq!(e.mem.u32(buffer + 0x10), 0x6008);
        logging(&mut e);
        e.call(0x00ac_d930, &args![buffer, 0x5000u32, 0x100u32]);
        assert_eq!(
            calls_to(&e, CRT_SETVBUF),
            vec![vec![0x6000, 0x5000, 0, 0x100]]
        );
        returns(&mut e, CRT_SETVBUF, 1);
        assert_eq!(e.call(0x00ac_d930, &args![buffer, 0u32, 0u32]).u32(), 0);
        let closed = file_buffer(&mut e, 0, 0);
        assert_eq!(e.call(0x00ac_d930, &args![closed, 0u32, 0u32]).u32(), 0);
    }

    #[test]
    fn n060_00acd9a0_syncs() {
        let mut e = Engine::new();
        let buffer = file_buffer(&mut e, 0x6000, 0);
        e.register(slot_target(4), |_, _| 0u32.into_ret());
        returns(&mut e, CRT_FFLUSH, 0);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_d9a0, &args![buffer]).u32(), 0);
        assert_eq!(
            calls_to(&e, slot_target(4)),
            vec![vec![buffer, 0xffff_ffff]]
        );
        returns(&mut e, CRT_FFLUSH, 0xffff_ffff);
        assert_eq!(e.call(0x00ac_d9a0, &args![buffer]).u32(), 0xffff_ffff);
        e.register(slot_target(4), |_, _| 0xffff_ffffu32.into_ret());
        logging(&mut e);
        assert_eq!(e.call(0x00ac_d9a0, &args![buffer]).u32(), 0);
        assert!(calls_to(&e, CRT_FFLUSH).is_empty());
        let closed = file_buffer(&mut e, 0, 0);
        assert_eq!(e.call(0x00ac_d9a0, &args![closed]).u32(), 0);
    }

    /// The cached converter facet `011f6d94` (its `always_noconv` answering `noconv`) and a
    /// locale with no facets of its own.
    fn codecvt_world(e: &mut Engine, noconv: bool) -> (u32, u32) {
        stub(e, &[LOCKIT_CTOR, LOCKIT_DTOR]);
        e.map(CACHED_CODECVT_011F6D94, 4);
        e.map(CODECVT_ID_011C33E0, 4);
        e.map(FACET_ID_COUNTER_0126FBA0, 8);
        let converter = new_converter(e);
        e.register_double(0x0330_0004, move |_, _| u32::from(noconv).into_ret());
        e.set_global(CACHED_CODECVT_011F6D94, converter);
        let implementation = e.mem.alloc(0x18);
        let locale = e.mem.alloc(4);
        e.mem.set_u32(locale, implementation);
        (converter, locale)
    }

    #[test]
    fn n060_00acda10_00ad4fd0_imbue_the_converter() {
        let mut e = Engine::new();
        let (converter, locale) = codecvt_world(&mut e, false);
        let buffer = file_buffer(&mut e, 0x6000, 0);
        e.call(0x00ac_da10, &args![buffer, locale]);
        assert_eq!(e.mem.u32(buffer + 0x3c), converter);
        assert_eq!(e.global::<u32>(CODECVT_ID_011C33E0), 1);
        let (_, locale) = codecvt_world(&mut e, true);
        e.call(0x00ac_da10, &args![buffer, locale]);
        assert_eq!(e.mem.u32(buffer + 0x3c), 0);
    }

    #[test]
    fn n060_00ad4fd0_makes_the_facet_when_none_is_cached() {
        let mut e = Engine::new();
        let (_, locale) = codecvt_world(&mut e, false);
        e.set_global(CACHED_CODECVT_011F6D94, 0u32);
        string_doubles(&mut e);
        allocator_doubles(&mut e);
        stub(&mut e, &[0x00ec_3740, 0x00ec_35d7, FACET_REGISTER_00EC3599]);
        let implementation = e.mem.u32(locale);
        set_string(&mut e, implementation + 0x18, b"C");
        logging(&mut e);
        let made = e.call(0x00ad_4fd0, &args![locale]).u32();
        assert_eq!(e.mem.u32(made), VTABLE_CODECVT_010A3204);
        assert_eq!(e.global::<u32>(CACHED_CODECVT_011F6D94), made);
        assert_eq!(e.mem.u32(made + 4), 1);
        assert_eq!(calls_to(&e, FACET_REGISTER_00EC3599), vec![vec![made]]);
        // `_Getcat` with a filled slot answers 2 and makes nothing.
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 1);
        logging(&mut e);
        assert_eq!(e.call(0x00ad_5500, &args![slot, locale]).u32(), 2);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn n060_00acda90_00acffe0_append_string_vectors() {
        let mut e = Engine::new();
        allocator_doubles(&mut e);
        string_doubles(&mut e);
        e.register(ALLOCATE_PROXY_0044D670, |e, _| e.mem.alloc(4).into_ret());
        e.register(COPY_CONSTRUCT_STRINGS_00AD68C0, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = read_string(e, from);
                set_string(e, to, &text);
                from += 0x1c;
                to += 0x1c;
            }
            to.into_ret()
        });
        e.register(COPY_CONSTRUCT_VECTORS_00AD6E20, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                for i in 0..6 {
                    let word = e.mem.u32(from + 4 * i);
                    e.mem.set_u32(to + 4 * i, word);
                }
                from += 0x18;
                to += 0x18;
            }
            to.into_ret()
        });
        let row = new_vector(&mut e, 0x1c, 2, 2);
        let first = e.mem.u32(row + 0xc);
        set_string(&mut e, first, b"a");
        set_string(&mut e, first + 0x1c, b"b");
        let table = new_vector(&mut e, 0x18, 0, 1);
        e.call(0x00ac_da90, &args![table, row]);
        let element = e.mem.u32(table + 0xc);
        assert_eq!(e.mem.u32(table + 0x10), element + 0x18);
        let copied = e.mem.u32(element + 0xc);
        assert_eq!(read_string(&e, copied + 0x1c), b"b");
        // Full: through the insert, the copy now second.
        e.call(0x00ac_da90, &args![table, row]);
        let element = e.mem.u32(table + 0xc);
        assert_eq!(e.mem.u32(table + 0x10), element + 0x30);
        let copied = e.mem.u32(element + 0x18 + 0xc);
        assert_eq!(read_string(&e, copied), b"a");
    }

    #[test]
    fn n060_00acdba0_00ad0110_00ad2640_clear_the_table() {
        let mut e = Engine::new();
        allocator_doubles(&mut e);
        string_doubles(&mut e);
        e.register(ITERATOR_NOT_EQUAL_00AC0460, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        stub(&mut e, &[ASSIGN_STRING_VECTOR_00AD61E0]);
        let table = new_vector(&mut e, 0x18, 2, 2);
        let element = e.mem.u32(table + 0xc);
        for row in [element, element + 0x18] {
            e.mem.set_u32(row, 0x4400 + row);
        }
        logging(&mut e);
        e.call(0x00ac_dba0, &args![table]);
        assert_eq!(e.mem.u32(table + 0x10), element);
        assert_eq!(
            calls_to(&e, OPERATOR_DELETE),
            vec![vec![0x4400 + element], vec![0x4400 + element + 0x18]]
        );
        assert!(calls_to(&e, ASSIGN_STRING_VECTOR_00AD61E0).is_empty());
    }

    #[test]
    fn n060_00acdc90_opens_an_output_file() {
        let mut e = Engine::new();
        file_stream_doubles(&mut e);
        returns(&mut e, FIOPEN, 0x7100);
        let (_, _) = codecvt_world(&mut e, true);
        let object = e.mem.alloc(0x90);
        let name = new_c(&mut e, b"out.csv");
        logging(&mut e);
        e.call(0x00ac_dc90, &args![object, name, 2u32, 0x40u32, 1u32]);
        assert_eq!(calls_to(&e, FIOPEN), vec![vec![name, 2, 0x40]]);
        assert_eq!(e.mem.u32(object), VBTABLE_OFSTREAM_010A3198);
        assert_eq!(e.mem.u32(object + 0x54), VTABLE_OFSTREAM_010A3184);
        assert_eq!(e.mem.u32(object + 4), VTABLE_FILEBUF_010A3138);
        assert_eq!(e.mem.u32(object + 0x54 + 0x28), object + 4);
        assert_eq!(e.mem.u32(object + 0x54 + 8), 0);
        returns(&mut e, FIOPEN, 0);
        let object = e.mem.alloc(0x90);
        e.call(0x00ac_dc90, &args![object, name, 2u32, 0x40u32, 1u32]);
        assert_eq!(e.mem.u32(object + 0x54 + 8), 2);
    }

    #[test]
    fn n060_00acde90_00ace330_store_the_stream_vtables() {
        let mut e = Engine::new();
        stub(&mut e, &[IOS_BASE_DTOR]);
        let (object, ios) = stream_object(&mut e, 0x10, 4);
        e.call(0x00ac_de90, &args![object + 4]);
        assert_eq!(e.mem.u32(ios), VTABLE_OSTREAM_010A318C);
        let (object, ios) = stream_object(&mut e, 0x10, 4);
        logging(&mut e);
        e.call(0x00ac_e330, &args![object]);
        assert_eq!(e.mem.u32(ios), VTABLE_ISTREAM_010A30A8);
        assert_eq!(e.mem.u32(object + 8), VTABLE_IOS_BASE_010A2F9C);
        assert_eq!(calls_to(&e, IOS_BASE_DTOR), vec![vec![object + 8]]);
    }

    #[test]
    fn n060_00acdf60_00acdff0_delete_through_a_base() {
        let mut e = Engine::new();
        stub(&mut e, &[IOS_BASE_DTOR, OPERATOR_DELETE]);
        e.map(0x010a_318c, 4);
        let (object, _) = stream_object(&mut e, 0x10, 4);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_df60, &args![object + 4, 1u32]).u32(), object);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![object]]);
        let (object, _) = stream_object(&mut e, 0x10, 4);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_dff0, &args![object + 8, 0u32]).u32(), object);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, IOS_BASE_DTOR), vec![vec![object + 8]]);
    }

    #[test]
    fn n060_00acdfa0_00ace030_scalar_deleting_destructors() {
        let mut e = Engine::new();
        stub(&mut e, &[IOS_BASE_DTOR, OPERATOR_DELETE, 0x00ec_3260]);
        let ios = e.mem.alloc(0x34);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_dfa0, &args![ios, 1u32]).u32(), ios);
        assert_eq!(e.mem.u32(ios), VTABLE_IOS_BASE_010A2F9C);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![ios]]);
        let buffer = new_buffer(&mut e, 0x50);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_e030, &args![buffer, 1u32]).u32(), buffer);
        assert_eq!(e.mem.u32(buffer), VTABLE_STREAMBUF_010A30FC);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![buffer]]);
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![buffer + 4]]);
    }

    #[test]
    fn n060_00ace060_00ace150_00ace260_delete_the_streams() {
        let mut e = Engine::new();
        stub(&mut e, &[IOS_BASE_DTOR, OPERATOR_DELETE, 0x00ec_3260]);
        for (address, offset, buffer_at) in [
            (0x00ac_e060u32, 0x50u32, 8u32),
            (0x00ac_e150, 0x58, 8),
            (0x00ac_e260, 0x54, 4),
        ] {
            let (object, ios) = stream_object(&mut e, offset, buffer_at);
            logging(&mut e);
            assert_eq!(e.call(address, &args![ios, 1u32]).u32(), object);
            assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![object]]);
            assert_eq!(calls_to(&e, IOS_BASE_DTOR), vec![vec![ios]]);
            assert_eq!(
                calls_to(&e, 0x00ec_3260),
                vec![vec![object + buffer_at + 4]]
            );
        }
    }

    #[test]
    fn n060_00ace0d0_00ace1c0_delete_the_buffers() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE, 0x00ec_3260]);
        let buffer = new_buffer(&mut e, 0x50);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_e0d0, &args![buffer, 1u32]).u32(), buffer);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![buffer]]);
        assert_eq!(e.mem.u32(buffer), VTABLE_STREAMBUF_010A30FC);
        let buffer = file_buffer(&mut e, 0, 0);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_e1c0, &args![buffer, 0u32]).u32(), buffer);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(calls_to(&e, 0x00ec_3260), vec![vec![buffer + 4]]);
    }

    #[test]
    fn n060_00ace390_skips_white_space_and_flushes_the_tie() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let s = new_stream(&mut e, 0x10);
        give_locale(&mut e, s.ios);
        let data = set_get_area(&mut e, s.buffer, b" \t x");
        let tie = new_stream(&mut e, 0x10);
        e.mem.set_u32(s.ios + 0x2c, tie.stream);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_e390, &args![s.stream, 0u32]).u32() & 0xff, 1);
        assert_eq!(slot_word(&e, s.buffer, 0x20), data + 3);
        assert_eq!(calls_to(&e, slot_target(0x30)), vec![vec![tie.buffer]]);
        // `noskip` leaves the blanks.
        let data = set_get_area(&mut e, s.buffer, b"  y");
        assert_eq!(e.call(0x00ac_e390, &args![s.stream, 1u32]).u32() & 0xff, 1);
        assert_eq!(slot_word(&e, s.buffer, 0x20), data);
        // Only blanks: eofbit, then failbit.
        set_get_area(&mut e, s.buffer, b"  ");
        assert_eq!(e.call(0x00ac_e390, &args![s.stream, 0u32]).u32() & 0xff, 0);
        assert_eq!(e.mem.u32(s.ios + 8), 3);
        // A bad state answers 0 at once.
        assert_eq!(e.call(0x00ac_e390, &args![s.stream, 0u32]).u32() & 0xff, 0);
    }

    #[test]
    fn n060_00ace7d0_00ace9c0_move_to_the_next_character() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let buffer = new_buffer(&mut e, 0x50);
        let data = set_get_area(&mut e, buffer, b"abc");
        assert_eq!(e.call(0x00ac_e7d0, &args![buffer]).u32(), u32::from(b'b'));
        assert_eq!(slot_word(&e, buffer, 0x20), data + 1);
        assert_eq!(e.call(0x00ac_e9c0, &args![buffer]).u32(), data + 2);
        assert_eq!(slot_word(&e, buffer, 0x30), 1);
        // One character left: taken, then the next through underflow.
        assert_eq!(e.call(0x00ac_e7d0, &args![buffer]).u32(), 0xffff_ffff);
        assert_eq!(slot_word(&e, buffer, 0x20), data + 3);
        set_get_area(&mut e, buffer, b"");
        assert_eq!(e.call(0x00ac_e7d0, &args![buffer]).u32(), 0xffff_ffff);
    }

    #[test]
    fn n060_00ace860_00ace970_take_a_character() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let buffer = new_buffer(&mut e, 0x50);
        let data = set_get_area(&mut e, buffer, b"m");
        assert_eq!(e.call(0x00ac_e970, &args![buffer]).u32(), data);
        set_get_area(&mut e, buffer, b"m");
        assert_eq!(e.call(0x00ac_e860, &args![buffer]).u32(), u32::from(b'm'));
        e.register(slot_target(0x14), |_, _| 0x6eu32.into_ret());
        assert_eq!(e.call(0x00ac_e860, &args![buffer]).u32(), 0x6e);
    }

    #[test]
    fn n060_00ace8c0_00acea00_put_a_character() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let buffer = new_buffer(&mut e, 0x50);
        let data = set_put_area(&mut e, buffer, 2);
        assert_eq!(e.call(0x00ac_ea00, &args![buffer]).u32(), data);
        assert_eq!(slot_word(&e, buffer, 0x34), 1);
        assert_eq!(e.call(0x00ac_e8c0, &args![buffer, 0x1f1u32]).u32(), 0xf1);
        assert_eq!(e.mem.u8(data + 1), 0xf1);
        e.register(slot_target(4), |_, a| a[1].into_ret());
        logging(&mut e);
        assert_eq!(e.call(0x00ac_e8c0, &args![buffer, 0x1f2u32]).u32(), 0xf2);
        assert_eq!(calls_to(&e, slot_target(4)), vec![vec![buffer, 0xf2]]);
    }

    #[test]
    fn n060_00acea50_00ad07c0_insert_strings() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        allocator_doubles(&mut e);
        e.register(CONSTRUCT_STRING_00AD6990, |e, a| {
            let text = read_string(e, a[1]);
            set_string(e, a[0], &text);
            Ret::default()
        });
        e.register(COPY_CONSTRUCT_STRINGS_00AD66E0, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = read_string(e, from);
                set_string(e, to, &text);
                from += 0x1c;
                to += 0x1c;
            }
            to.into_ret()
        });
        e.register(STRING_ASSIGN_COPY_00AD6BB0, |e, a| {
            let text = read_string(e, a[1]);
            set_string(e, a[0], &text);
            a[0].into_ret()
        });
        let vector = new_vector(&mut e, 0x1c, 3, 7);
        let first = e.mem.u32(vector + 0xc);
        for (i, text) in [b"a", b"b", b"c"].iter().enumerate() {
            set_string(&mut e, first + 0x1c * i as u32, *text);
        }
        let value = new_string(&mut e, b"N");
        let read_all = |e: &Engine, vector: u32| -> Vec<Vec<u8>> {
            let mut at = e.mem.u32(vector + 0xc);
            let mut out = vec![];
            while at != e.mem.u32(vector + 0x10) {
                out.push(read_string(e, at));
                at += 0x1c;
            }
            out
        };
        // In place, within the elements: the tail moves back.
        let proxy = e.mem.u32(vector);
        e.call(
            0x00ad_07c0,
            &args![vector, proxy, first + 0x1c, 2u32, value],
        );
        assert_eq!(
            read_all(&e, vector),
            [
                b"a".to_vec(),
                b"N".to_vec(),
                b"N".to_vec(),
                b"b".to_vec(),
                b"c".to_vec()
            ]
        );
        // In place, reaching past the old end: the tail copied out, the rest filled.
        e.call(
            0x00ad_07c0,
            &args![vector, proxy, first + 4 * 0x1c, 2u32, value],
        );
        let all = read_all(&e, vector);
        assert_eq!(all.len(), 7);
        assert_eq!(
            all[3..],
            [b"b".to_vec(), b"N".to_vec(), b"N".to_vec(), b"c".to_vec()]
        );
        // Grows, through insert (iterator at the inserted element).
        let out = e.mem.alloc(8);
        let last = e.mem.u32(vector + 0x10);
        e.call(0x00ac_ea50, &args![vector, out, proxy, last, value]);
        let first = e.mem.u32(vector + 0xc);
        assert_eq!(e.mem.u32(out + 4), first + 7 * 0x1c);
        assert_eq!(read_all(&e, vector).len(), 8);
        assert_eq!(e.mem.u32(vector + 0x14), first + 10 * 0x1c);
        // A count of 0 does nothing; past the maximum `_Xlen`.
        stub(&mut e, &[VECTOR_TOO_LONG_00AC0B00]);
        logging(&mut e);
        e.call(0x00ad_07c0, &args![vector, proxy, first, 0u32, value]);
        e.call(
            0x00ad_07c0,
            &args![vector, proxy, first, 0x0924_9249u32, value],
        );
        assert_eq!(calls_to(&e, VECTOR_TOO_LONG_00AC0B00).len(), 1);
        assert_eq!(read_all(&e, vector).len(), 8);
    }

    #[test]
    fn n060_00aced20_buys_storage() {
        let mut e = Engine::new();
        allocator_doubles(&mut e);
        stub(&mut e, &[VECTOR_TOO_LONG_00AC0B00]);
        let vector = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00ac_ed20, &args![vector, 0u32]).u32() & 0xff, 0);
        assert_eq!(e.call(0x00ac_ed20, &args![vector, 2u32]).u32() & 0xff, 1);
        let first = e.mem.u32(vector + 0xc);
        assert_eq!(e.mem.u32(vector + 0x10), first);
        assert_eq!(e.mem.u32(vector + 0x14), first + 0x38);
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_ed20, &args![vector, 0x0924_924au32]).u32() & 0xff,
            1
        );
        assert_eq!(calls_to(&e, VECTOR_TOO_LONG_00AC0B00).len(), 1);
        assert_eq!(e.mem.u32(vector + 0xc), 0);
    }

    #[test]
    fn n060_00acf500_00acf580_00ad1c10_00ad1a90_resize_pairs() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        allocator_doubles(&mut e);
        pair_range_doubles(&mut e);
        e.register(ITERATOR_NOT_EQUAL_00AC0460, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        let vector = new_vector(&mut e, 0x38, 0, 0);
        let pair = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00ac_f500, &args![pair]).u32(), pair);
        set_string(&mut e, pair, b"k");
        set_string(&mut e, pair + 0x1c, b"v");
        let mut words = vec![vector, 3];
        for i in 0..14 {
            words.push(e.mem.u32(pair + 4 * i));
        }
        logging(&mut e);
        e.call(0x00ac_f580, &words);
        let first = e.mem.u32(vector + 0xc);
        assert_eq!(e.mem.u32(vector + 0x10), first + 3 * 0x38);
        assert_eq!(read_string(&e, first + 2 * 0x38 + 0x1c), b"v");
        // The by-value pair is destroyed (its two strings).
        assert_eq!(
            calls_to(&e, STRING_TIDY_0044A730)
                .iter()
                .filter(|w| w[1] == 1)
                .count(),
            2
        );
        // Shrinking erases from the count.
        words[1] = 1;
        e.call(0x00ac_f580, &words);
        assert_eq!(e.mem.u32(vector + 0x10), first + 0x38);
        // Equal size: nothing but the destruction of the argument.
        e.call(0x00ac_f580, &words);
        assert_eq!(e.mem.u32(vector + 0x10), first + 0x38);
    }

    #[test]
    fn n060_00acf7d0_inserts_a_pair() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        allocator_doubles(&mut e);
        pair_range_doubles(&mut e);
        e.register(COPY_PAIRS_BACKWARD_00AD6800, |e, a| {
            let (first, mut from, mut to) = (a[0], a[1], a[2]);
            while from != first {
                from -= 0x38;
                to -= 0x38;
                for offset in [0, 0x1c] {
                    let text = read_string(e, from + offset);
                    set_string(e, to + offset, &text);
                }
            }
            to.into_ret()
        });
        let vector = new_vector(&mut e, 0x38, 2, 4);
        let first = e.mem.u32(vector + 0xc);
        set_string(&mut e, first, b"a");
        set_string(&mut e, first + 0x38, b"b");
        let pair = new_pair(&mut e, b"z", b"1");
        let out = e.mem.alloc(8);
        let proxy = e.mem.u32(vector);
        e.call(0x00ac_f7d0, &args![vector, out, proxy, first, pair]);
        assert_eq!(e.mem.u32(out + 4), first);
        assert_eq!(read_string(&e, first), b"z");
        assert_eq!(read_string(&e, first + 0x38), b"a");
        assert_eq!(read_string(&e, first + 0x70), b"b");
        assert_eq!(e.mem.u32(vector + 0x10), first + 3 * 0x38);
    }

    #[test]
    fn n060_00acfb90_00acfcd0_close_the_file() {
        let mut e = Engine::new();
        file_doubles(&mut e);
        e.map(INITIAL_STATE_011F6D8C, 4);
        returns(&mut e, CRT_FCLOSE, 0);
        let buffer = file_buffer(&mut e, 0x6000, 0);
        assert_eq!(e.call(0x00ac_fb90, &args![buffer]).u32(), buffer);
        assert_eq!(e.mem.u32(buffer + 0x4c), 0);
        assert_eq!(e.call(0x00ac_fb90, &args![buffer]).u32(), 0);
        returns(&mut e, CRT_FCLOSE, 0xffff_ffff);
        let buffer = file_buffer(&mut e, 0x6000, 0);
        assert_eq!(e.call(0x00ac_fb90, &args![buffer]).u32(), 0);
        // `_Endwrite` with a converter and pending output: the closing sequence is written.
        returns(&mut e, CRT_FCLOSE, 0);
        let converter = new_converter(&mut e);
        e.register(0x0330_0018, |e, a| {
            // unshift(state, to, to_end, to_next)
            e.mem.set_u8(a[2], b'~');
            e.mem.set_u32(a[4], a[2] + 1);
            0u32.into_ret()
        });
        e.register(slot_target(4), |_, _| 0u32.into_ret());
        let buffer = file_buffer(&mut e, 0x6000, converter);
        e.mem.set_u8(buffer + 0x41, 1);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_fcd0, &args![buffer]).u32() & 0xff, 1);
        assert_eq!(calls_to(&e, CRT_FWRITE)[0][1..], [1, 1, 0x6000]);
        assert_eq!(e.mem.u8(buffer + 0x41), 0);
        // `noconv` (3) answers 1, an error 0, a failed flush 0.
        e.mem.set_u8(buffer + 0x41, 1);
        e.register(0x0330_0018, |_, _| 3u32.into_ret());
        assert_eq!(e.call(0x00ac_fcd0, &args![buffer]).u32() & 0xff, 1);
        e.register(0x0330_0018, |_, _| 2u32.into_ret());
        assert_eq!(e.call(0x00ac_fcd0, &args![buffer]).u32() & 0xff, 0);
        e.register(slot_target(4), |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(e.call(0x00ac_fcd0, &args![buffer]).u32() & 0xff, 0);
    }

    #[test]
    fn n060_00ad0210_00ad02f0_00ad03e0_output_sentries() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let s = new_stream(&mut e, 0x10);
        let tie = new_stream(&mut e, 0x10);
        e.mem.set_u32(s.ios + 0x2c, tie.stream);
        let sentry = e.mem.alloc(8);
        logging(&mut e);
        assert_eq!(e.call(0x00ad_0210, &args![sentry, s.stream]).u32(), sentry);
        assert_eq!(e.mem.u32(sentry), s.stream);
        assert_eq!(e.mem.u8(sentry + 4), 1);
        assert_eq!(calls_to(&e, slot_target(0x30)), vec![vec![tie.buffer]]);
        // `unitbuf` flushes when the sentry goes.
        e.mem.set_u32(s.ios + 0x10, 2);
        logging(&mut e);
        e.call(0x00ad_02f0, &args![sentry]);
        assert_eq!(calls_to(&e, slot_target(0x30)), vec![vec![s.buffer]]);
        assert_eq!(calls_to(&e, MUTEX_UNLOCK), vec![vec![s.buffer + 4]]);
        // Not while an exception is in flight.
        returns(&mut e, UNCAUGHT_EXCEPTION, 1);
        logging(&mut e);
        e.call(0x00ad_02f0, &args![sentry]);
        assert!(calls_to(&e, slot_target(0x30)).is_empty());
        // A bad stream: no tie flush, not good.
        e.mem.set_u32(s.ios + 8, 4);
        logging(&mut e);
        e.call(0x00ad_0210, &args![sentry, s.stream]);
        assert_eq!(e.mem.u8(sentry + 4), 0);
        assert!(calls_to(&e, slot_target(0x30)).is_empty());
        e.mem.set_u32(s.ios + 0x10, 0);
        logging(&mut e);
        e.call(0x00ad_03e0, &args![s.stream]);
        assert!(calls_to(&e, slot_target(0x30)).is_empty());
    }

    #[test]
    fn n060_00ad06e0_copies_the_buffer_locale() {
        let mut e = Engine::new();
        stub(&mut e, &[LOCKIT_CTOR, LOCKIT_DTOR]);
        let facet = e.mem.alloc(8);
        e.mem.set_u32(facet + 4, 2);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, facet);
        let buffer = e.mem.alloc(0x50);
        e.mem.set_u32(buffer + 0x38, holder);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00ad_06e0, &args![buffer, out]).u32(), out);
        assert_eq!(e.mem.u32(out), facet);
        assert_eq!(e.mem.u32(facet + 4), 3);
    }

    #[test]
    fn n060_00ad10e0_00ad1740_insert_bytes_in_place() {
        let mut e = Engine::new();
        e.register(FILL_BYTES_00AC3300, |e, a| {
            let value = e.mem.u8(a[2]);
            for i in 0..a[1] {
                e.mem.set_u8(a[0] + i, value);
            }
            Ret::default()
        });
        e.register(COPY_BYTES_00AD5E20, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            e.mem.write(a[2], &bytes);
            (a[2] + bytes.len() as u32).into_ret()
        });
        e.register(COPY_BYTES_BACKWARD_00AB4490, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            e.mem.write(a[2] - bytes.len() as u32, &bytes);
            Ret::default()
        });
        let vector = new_vector(&mut e, 1, 3, 8);
        let first = e.mem.u32(vector + 0xc);
        e.mem.write(first, b"abc");
        let value = new_c(&mut e, b"-");
        let proxy = e.mem.u32(vector);
        // Inside: the tail moves back.
        e.call(0x00ad_10e0, &args![vector, proxy, first + 1, 2u32, value]);
        assert_eq!(e.mem.bytes(first, 5), b"a--bc");
        // Reaching past the end: the tail copied out, the rest filled (00ad1740).
        e.call(0x00ad_10e0, &args![vector, proxy, first + 4, 2u32, value]);
        assert_eq!(e.mem.bytes(first, 7), b"a--b--c");
        assert_eq!(e.mem.u32(vector + 0x10), first + 7);
        assert_eq!(
            e.call(0x00ad_1740, &args![vector, first, 2u32, value])
                .u32(),
            first + 2
        );
        assert_eq!(e.mem.bytes(first, 2), b"--");
        // A count of 0 does nothing.
        e.call(0x00ad_10e0, &args![vector, proxy, first, 0u32, value]);
        assert_eq!(e.mem.u32(vector + 0x10), first + 7);
    }

    /// A string vector element (0x18 bytes) holding one string `text`.
    fn string_vector_element(e: &mut Engine, at: u32, text: &[u8]) {
        let storage = e.mem.alloc(0x1c);
        set_string(e, storage, text);
        e.mem.set_u32(at, 0x5000 + at);
        e.mem.set_u32(at + 0xc, storage);
        e.mem.set_u32(at + 0x10, storage + 0x1c);
        e.mem.set_u32(at + 0x14, storage + 0x1c);
    }

    fn first_string_of(e: &Engine, element: u32) -> Vec<u8> {
        read_string(e, e.mem.u32(element + 0xc))
    }

    #[test]
    fn n060_00ad26f0_00ad3150_insert_string_vectors_in_place() {
        let mut e = Engine::new();
        allocator_doubles(&mut e);
        string_doubles(&mut e);
        e.register(ALLOCATE_PROXY_0044D670, |e, _| e.mem.alloc(4).into_ret());
        stub(&mut e, &[FREE_PROXY_00702220]);
        e.register(COPY_CONSTRUCT_STRINGS_00AD68C0, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = read_string(e, from);
                set_string(e, to, &text);
                from += 0x1c;
                to += 0x1c;
            }
            to.into_ret()
        });
        // Element copies and assignments copy the one string deeply.
        e.register(COPY_CONSTRUCT_VECTORS_00AD6E20, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = first_string_of(e, from);
                string_vector_element(e, to, &text);
                from += 0x18;
                to += 0x18;
            }
            to.into_ret()
        });
        e.register(ASSIGN_STRING_VECTOR_00AD61E0, |e, a| {
            let text = first_string_of(e, a[1]);
            string_vector_element(e, a[0], &text);
            a[0].into_ret()
        });
        let table = new_vector(&mut e, 0x18, 2, 6);
        let first = e.mem.u32(table + 0xc);
        string_vector_element(&mut e, first, b"a");
        string_vector_element(&mut e, first + 0x18, b"b");
        let value = e.mem.alloc(0x18);
        string_vector_element(&mut e, value, b"N");
        let proxy = e.mem.u32(table);
        // Inside: the last element copied out, the middle moved back (00ad60c0).
        logging(&mut e);
        e.call(0x00ad_26f0, &args![table, proxy, first, 1u32, value]);
        assert_eq!(e.mem.u32(table + 0x10), first + 0x30 + 0x18);
        assert_eq!(first_string_of(&e, first), b"N");
        assert_eq!(first_string_of(&e, first + 0x18), b"a");
        assert_eq!(first_string_of(&e, first + 0x30), b"b");
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 2);
        // Past the end: the tail copied out, the rest constructed.
        logging(&mut e);
        e.call(0x00ad_26f0, &args![table, proxy, first + 0x30, 2u32, value]);
        assert_eq!(e.mem.u32(table + 0x10), first + 5 * 0x18);
        assert_eq!(first_string_of(&e, first + 0x30), b"N");
        assert_eq!(first_string_of(&e, first + 0x48), b"N");
        assert_eq!(first_string_of(&e, first + 0x60), b"b");
        assert_eq!(calls_to(&e, FREE_PROXY_00702220).len(), 1);
    }

    #[test]
    fn n060_00ad5250_00ad60c0_00ad5180_copy_ranges() {
        let mut e = Engine::new();
        e.register(STRING_ASSIGN_COPY_00AD6BB0, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            a[0].into_ret()
        });
        e.register(ASSIGN_STRING_VECTOR_00AD61E0, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            a[0].into_ret()
        });
        let block = e.mem.alloc(0x100);
        for i in 0..4u32 {
            e.mem.set_u32(block + 0x1c * i, 10 + i);
        }
        // Backward copy of strings 0..2 to end at 3: 1 then 0 assigned.
        logging(&mut e);
        let start = e
            .call(0x00ad_5250, &args![block, block + 0x38, block + 0x54])
            .u32();
        assert_eq!(start, block + 0x1c);
        assert_eq!(e.mem.u32(block + 0x38), 11);
        assert_eq!(e.mem.u32(block + 0x1c), 10);
        let order: Vec<u32> = calls_to(&e, STRING_ASSIGN_COPY_00AD6BB0)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(order, vec![block + 0x38, block + 0x1c]);
        let block = e.mem.alloc(0x100);
        for i in 0..4u32 {
            e.mem.set_u32(block + 0x18 * i, 20 + i);
        }
        let words = args![block, block + 0x30, block + 0x48, 0u32, 0u32, 0u32];
        assert_eq!(e.call(0x00ad_60c0, &words).u32(), block + 0x18);
        assert_eq!(e.mem.u32(block + 0x30), 21);
        let words = args![block + 0x18, block + 0x48, block];
        assert_eq!(e.call(0x00ad_5180, &words).u32(), block + 0x30);
        assert_eq!(e.mem.u32(block), 20);
        assert_eq!(e.mem.u32(block + 0x18), 21);
    }

    #[test]
    fn n060_00ad6190_00ad6a30_00ad5ec0_pair_assignment() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        stub(&mut e, &[OPERATOR_DELETE]);
        let source = new_pair(&mut e, b"k", b"v");
        let target = new_pair(&mut e, b"", b"");
        assert_eq!(e.call(0x00ad_6190, &args![target, source]).u32(), target);
        assert_eq!(read_string(&e, target + 0x1c), b"v");
        let pairs = e.mem.alloc(0x38 * 3);
        for (i, key) in [b"x", b"y", b"z"].iter().enumerate() {
            set_string(&mut e, pairs + 0x38 * i as u32, *key);
            set_string(&mut e, pairs + 0x38 * i as u32 + 0x1c, b"w");
        }
        let words = args![pairs + 0x38, pairs + 0xa8, pairs, 0u32, 0u32, 0u32];
        assert_eq!(e.call(0x00ad_5ec0, &words).u32(), pairs + 0x70);
        assert_eq!(read_string(&e, pairs), b"y");
        assert_eq!(read_string(&e, pairs + 0x38), b"z");
        logging(&mut e);
        assert_eq!(e.call(0x00ad_6a30, &args![target, 1u32]).u32(), target);
        assert_eq!(read_string(&e, target + 0x1c), b"");
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![target]]);
    }

    #[test]
    fn n060_range_forwarders_pass_two_tag_words() {
        let mut e = Engine::new();
        for target in [
            COPY_CONSTRUCT_STRINGS_00AD66E0,
            COPY_CONSTRUCT_PAIRS_00AD6D30,
            COPY_CONSTRUCT_VECTORS_00AD6E20,
            COPY_PAIRS_BACKWARD_00AD6800,
        ] {
            e.register(target, |_, a| {
                (a[0] + a[1] + a[2] + a[3] + a[4] + a[5]).into_ret()
            });
        }
        for (address, target) in [
            (0x00ad_5dd0u32, COPY_CONSTRUCT_STRINGS_00AD66E0),
            (0x00ad_5f80, COPY_CONSTRUCT_PAIRS_00AD6D30),
            (0x00ad_6040, COPY_CONSTRUCT_VECTORS_00AD6E20),
            (0x00ad_6b10, COPY_CONSTRUCT_PAIRS_00AD6D30),
            (0x00ad_6b60, COPY_CONSTRUCT_VECTORS_00AD6E20),
        ] {
            logging(&mut e);
            assert_eq!(e.call(address, &args![1u32, 2u32, 3u32, 4u32]).u32(), 10);
            assert_eq!(calls_to(&e, target), vec![vec![1, 2, 3, 4, 0, 0]]);
        }
        logging(&mut e);
        let words = args![1u32, 2u32, 3u32, 0x1234u32, 9u32, 9u32];
        assert_eq!(e.call(0x00ad_6000, &words).u32(), 1 + 2 + 3 + 0x34);
        assert_eq!(
            calls_to(&e, COPY_PAIRS_BACKWARD_00AD6800),
            vec![vec![1, 2, 3, 0x34, 0, 0]]
        );
    }

    #[test]
    fn n060_00ad6ab0_00ad5110_move_bytes() {
        let mut e = Engine::new();
        e.register(MEMMOVE_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let block = e.mem.alloc(0x10);
        e.mem.write(block, b"abcd");
        assert_eq!(
            e.call(0x00ad_6ab0, &args![block, block + 2, block + 8, 0u32])
                .u32(),
            block + 10
        );
        assert_eq!(e.mem.bytes(block + 8, 2), b"ab");
        assert_eq!(
            e.call(0x00ad_5110, &args![block + 2, block + 4, block])
                .u32(),
            block + 2
        );
        assert_eq!(e.mem.bytes(block, 2), b"cd");
        logging(&mut e);
        assert_eq!(
            e.call(0x00ad_5110, &args![block, block, block + 4]).u32(),
            block + 4
        );
        assert_eq!(
            e.call(0x00ad_6ab0, &args![block, block, block + 4, 0u32])
                .u32(),
            block + 4
        );
        assert!(calls_to(&e, MEMMOVE_S).is_empty());
    }

    #[test]
    fn n060_00ad35e0_00ad36a0_00ad3750_00ad3810_advance_checked_iterators() {
        let mut e = Engine::new();
        stub(&mut e, &[INVALID_PARAMETER]);
        for (address, element) in [
            (0x00ad_35e0u32, 0x1cu32),
            (0x00ad_36a0, 1),
            (0x00ad_3750, 0x38),
            (0x00ad_3810, 0x18),
        ] {
            let vector = new_vector(&mut e, element, 3, 3);
            let first = e.mem.u32(vector + 0xc);
            let iterator = e.mem.alloc(8);
            e.mem.set_u32(iterator, e.mem.u32(vector));
            e.mem.set_u32(iterator + 4, first);
            logging(&mut e);
            assert_eq!(e.call(address, &args![iterator, 3u32]).u32(), iterator);
            assert_eq!(e.mem.u32(iterator + 4), first + 3 * element);
            assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
            e.call(address, &args![iterator, 1u32]);
            assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
            logging(&mut e);
            e.call(address, &args![iterator, 0u32.wrapping_sub(5)]);
            assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
        }
    }

    #[test]
    fn n060_00ad1a90_erases_pairs() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        stub(&mut e, &[OPERATOR_DELETE]);
        e.register(ITERATOR_NOT_EQUAL_00AC0460, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        let vector = new_vector(&mut e, 0x38, 3, 3);
        let first = e.mem.u32(vector + 0xc);
        for (i, key) in [b"a", b"b", b"c"].iter().enumerate() {
            set_string(&mut e, first + 0x38 * i as u32, *key);
            set_string(&mut e, first + 0x38 * i as u32 + 0x1c, b"v");
        }
        let proxy = e.mem.u32(vector);
        let out = e.mem.alloc(8);
        let words = args![vector, out, proxy, first, proxy, first + 0x38];
        e.call(0x00ad_1a90, &words);
        assert_eq!(e.mem.u32(vector + 0x10), first + 0x70);
        assert_eq!(read_string(&e, first), b"b");
        assert_eq!(read_string(&e, first + 0x38), b"c");
        assert_eq!(e.mem.u32(out + 4), first);
        // An empty range changes nothing.
        e.call(0x00ad_1a90, &args![vector, out, proxy, first, proxy, first]);
        assert_eq!(e.mem.u32(vector + 0x10), first + 0x70);
    }

    #[test]
    fn n060_facet_destructors_store_their_vtables() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE]);
        let facet = e.mem.alloc(0x18);
        for (address, vtable, deletes) in [
            (0x00ad_3eb0u32, VTABLE_FACET_010A31FC, true),
            (0x00ad_3ee0, VTABLE_FACET_010A31FC, false),
            (0x00ad_5670, VTABLE_FACET_010A31FC, true),
            (0x00ad_5730, VTABLE_FACET_010A31FC, false),
        ] {
            logging(&mut e);
            let flags = u32::from(deletes);
            assert_eq!(e.call(address, &args![facet, flags]).u32(), facet);
            assert_eq!(e.mem.u32(facet), vtable);
            assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), usize::from(deletes));
        }
        e.mem.set_u32(facet, 0);
        e.call(0x00ad_41a0, &args![facet]);
        assert_eq!(e.mem.u32(facet), VTABLE_FACET_010A31FC);
    }

    #[test]
    fn n060_00ad3f20_00ad3f50_00ad3f90_00ad3fc0_convert_case() {
        let mut e = Engine::new();
        e.register(CRT_TOLOWER, |_, a| {
            (u32::from((a[0] as u8).to_ascii_lowercase()) | 0x100).into_ret()
        });
        e.register(CRT_TOUPPER, |_, a| {
            (u32::from((a[0] as u8).to_ascii_uppercase()) | 0x100).into_ret()
        });
        let facet = e.mem.alloc(0x18);
        logging(&mut e);
        assert_eq!(e.call(0x00ad_3f20, &args![facet, 0x341u32]).u32(), 0x161);
        assert_eq!(calls_to(&e, CRT_TOLOWER), vec![vec![0x41, facet + 8]]);
        assert_eq!(e.call(0x00ad_3f90, &args![facet, 0x61u32]).u32(), 0x141);
        let text = new_c(&mut e, b"MiXed");
        assert_eq!(
            e.call(0x00ad_3f50, &args![facet, text, text + 5]).u32(),
            text + 5
        );
        assert_eq!(e.mem.bytes(text, 5), b"mixed");
        assert_eq!(
            e.call(0x00ad_3fc0, &args![facet, text, text + 3]).u32(),
            text + 3
        );
        assert_eq!(e.mem.bytes(text, 5), b"MIXed");
    }

    #[test]
    fn n060_00ad4010_00ad4040_00ad4090_00ad40d0_00ad4080_widen_and_narrow() {
        let mut e = Engine::new();
        e.register(MEMCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.put_vtable(
            0x0340_0000,
            &[0, 0, 0, 0, 0, 0, 0, 0x00ad_4040, 0, 0, 0x00ad_40d0],
        );
        let facet = e.mem.alloc(0x18);
        e.mem.set_u32(facet, 0x0340_0000);
        let text = new_c(&mut e, b"abc");
        let out = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00ad_4010, &args![facet, text, text + 3, out])
                .u32(),
            text + 3
        );
        assert_eq!(e.mem.bytes(out, 3), b"abc");
        let other = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00ad_4090, &args![facet, text, text + 2, 0x13fu32, other])
                .u32(),
            text + 2
        );
        assert_eq!(e.mem.bytes(other, 2), b"ab");
        stub(&mut e, &[INVALID_PARAMETER]);
        logging(&mut e);
        e.call(0x00ad_4040, &args![facet, text, text + 3, out, 2u32]);
        e.call(0x00ad_40d0, &args![facet, text, text + 3, 0u32, out, 2u32]);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 2);
        assert_eq!(
            e.call(0x00ad_4080, &args![facet, 0x1e9u32, 0x3fu32]).u32() & 0xff,
            0xe9
        );
    }

    #[test]
    fn n060_00ad56b0_00ad56e0_00ad5700_codecvt_no_conversion() {
        let mut e = Engine::new();
        let next = e.mem.alloc(8);
        let words = args![
            0u32,
            0u32,
            0x100u32,
            0x110u32,
            next,
            0x200u32,
            0x210u32,
            next + 4
        ];
        assert_eq!(e.call(0x00ad_56b0, &words).u32(), 3);
        assert_eq!(e.mem.u32(next), 0x100);
        assert_eq!(e.mem.u32(next + 4), 0x200);
        assert_eq!(
            e.call(0x00ad_56e0, &args![0u32, 0u32, 0x300u32, 0x310u32, next])
                .u32(),
            3
        );
        assert_eq!(e.mem.u32(next), 0x300);
        let words = args![0u32, 0u32, 0x100u32, 0x110u32, 4u32];
        assert_eq!(e.call(0x00ad_5700, &words).u32(), 4);
        let words = args![0u32, 0u32, 0x100u32, 0x110u32, 0x40u32];
        assert_eq!(e.call(0x00ad_5700, &words).u32(), 0x10);
    }

    #[test]
    fn n060_00ad4210_00ad42e0_concatenate() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        e.register(STRING_APPEND_STRING_00A0C570, |e, a| {
            let mut text = read_string(e, a[0]);
            text.extend(read_string(e, a[1]));
            set_string(e, a[0], &text);
            a[0].into_ret()
        });
        let left = new_string(&mut e, b"face");
        let out = e.mem.alloc(0x1c);
        let suffix = new_c(&mut e, b".tri");
        assert_eq!(e.call(0x00ad_4210, &args![out, left, suffix]).u32(), out);
        assert_eq!(read_string(&e, out), b"face.tri");
        assert_eq!(read_string(&e, left), b"face");
        let right = new_string(&mut e, b"gen");
        logging(&mut e);
        assert_eq!(e.call(0x00ad_42e0, &args![out, left, right]).u32(), out);
        assert_eq!(read_string(&e, out), b"facegen");
        assert_eq!(
            calls_to(&e, STRING_APPEND_STRING_00A0C570)[0][2..],
            [0, 0xffff_ffff]
        );
    }

    fn input_stream(e: &mut Engine, text: &[u8]) -> Stream {
        string_doubles(e);
        stream_doubles(e);
        let s = new_stream(e, 0x10);
        give_locale(e, s.ios);
        set_get_area(e, s.buffer, text);
        s
    }

    #[test]
    fn n060_00ad43a0_reads_a_word() {
        let mut e = Engine::new();
        let s = input_stream(&mut e, b"  key value");
        let word = new_string(&mut e, b"old");
        assert_eq!(e.call(0x00ad_43a0, &args![s.stream, word]).u32(), s.stream);
        assert_eq!(read_string(&e, word), b"key");
        assert_eq!(e.mem.u32(s.ios + 8), 0);
        // The width limits the word and is cleared.
        e.mem.set_u32(s.ios + 0x18, 2);
        e.call(0x00ad_43a0, &args![s.stream, word]);
        assert_eq!(read_string(&e, word), b"va");
        assert_eq!(e.mem.u32(s.ios + 0x18), 0);
        // At the end of the text: eofbit; then nothing read: failbit.
        e.call(0x00ad_43a0, &args![s.stream, word]);
        assert_eq!(read_string(&e, word), b"lue");
        assert_eq!(e.mem.u32(s.ios + 8), 1);
        e.call(0x00ad_43a0, &args![s.stream, word]);
        assert_eq!(e.mem.u32(s.ios + 8), 3);
    }

    #[test]
    fn n060_00ad5770_reads_lines() {
        let mut e = Engine::new();
        let s = input_stream(&mut e, b"one two\nthree");
        let line = new_string(&mut e, b"");
        assert_eq!(
            e.call(0x00ad_5770, &args![s.stream, line, 0x10au32]).u32(),
            s.stream
        );
        assert_eq!(read_string(&e, line), b"one two");
        assert_eq!(e.mem.u32(s.ios + 8), 0);
        e.call(0x00ad_5770, &args![s.stream, line, 0xau32]);
        assert_eq!(read_string(&e, line), b"three");
        assert_eq!(e.mem.u32(s.ios + 8), 1);
        e.call(0x00ad_5770, &args![s.stream, line, 0xau32]);
        assert_eq!(e.mem.u32(s.ios + 8), 3);
        // An empty line still counts as read.
        let s = input_stream(&mut e, b"\nx");
        e.call(0x00ad_5770, &args![s.stream, line, 0xau32]);
        assert_eq!(read_string(&e, line), b"");
        assert_eq!(e.mem.u32(s.ios + 8), 0);
    }

    fn output_stream(e: &mut Engine) -> (Stream, u32) {
        stream_doubles(e);
        let s = new_stream(e, 0x10);
        let data = set_put_area(e, s.buffer, 0x40);
        e.register(slot_target(0x20), |e, a| {
            // xsputn(text, count): copies into the put area.
            let target = e.mem.u32(a[0] + 0x24);
            let next = e.mem.u32(target);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(next, &bytes);
            e.mem.set_u32(target, next + a[2]);
            a[2].into_ret()
        });
        (s, data)
    }

    #[test]
    fn n060_00ad47f0_writes_text_with_padding() {
        let mut e = Engine::new();
        e.register(CRT_STRLEN, |e, a| (read_c(e, a[0]).len() as u32).into_ret());
        let (s, data) = output_stream(&mut e);
        let text = new_c(&mut e, b"ab");
        assert_eq!(e.call(0x00ad_47f0, &args![s.stream, text]).u32(), s.stream);
        assert_eq!(e.mem.bytes(data, 2), b"ab");
        // Right-adjusted in a width of 4, then left-adjusted.
        e.mem.set_u32(s.ios + 0x18, 4);
        e.call(0x00ad_47f0, &args![s.stream, text]);
        assert_eq!(e.mem.bytes(data, 6), b"ab  ab");
        assert_eq!(e.mem.u32(s.ios + 0x18), 0);
        e.mem.set_u32(s.ios + 0x18, 3);
        e.mem.set_u32(s.ios + 0x10, 0x40);
        e.call(0x00ad_47f0, &args![s.stream, text]);
        assert_eq!(e.mem.bytes(data, 9), b"ab  abab ");
        // A short write is badbit.
        e.register(slot_target(0x20), |_, _| 0u32.into_ret());
        e.call(0x00ad_47f0, &args![s.stream, text]);
        assert_eq!(e.mem.u32(s.ios + 8), 4);
    }

    #[test]
    fn n060_00ad4ba0_writes_a_string() {
        let mut e = Engine::new();
        stub(&mut e, &[INVALID_PARAMETER]);
        let (s, data) = output_stream(&mut e);
        let string = new_string(&mut e, b"xyz");
        e.mem.set_u32(s.ios + 0x18, 5);
        assert_eq!(
            e.call(0x00ad_4ba0, &args![s.stream, string]).u32(),
            s.stream
        );
        assert_eq!(e.mem.bytes(data, 5), b"  xyz");
        e.mem.set_u32(s.ios + 0x10, 0x40);
        e.mem.set_u32(s.ios + 0x18, 4);
        e.call(0x00ad_4ba0, &args![s.stream, string]);
        assert_eq!(e.mem.bytes(data, 9), b"  xyzxyz ");
        // A failing character write is badbit.
        set_put_area(&mut e, s.buffer, 1);
        e.call(0x00ad_4ba0, &args![s.stream, string]);
        assert_eq!(e.mem.u32(s.ios + 8), 4);
    }

    #[test]
    fn n060_00ad5b00_00ad5bd0_00ad5cc0_fill_uninitialised_slots() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        allocator_doubles(&mut e);
        e.register(CONSTRUCT_STRING_00AD6990, |e, a| {
            let text = read_string(e, a[1]);
            set_string(e, a[0], &text);
            Ret::default()
        });
        e.register(ALLOCATE_PROXY_0044D670, |e, _| e.mem.alloc(4).into_ret());
        e.register(COPY_CONSTRUCT_STRINGS_00AD68C0, |_, a| a[2].into_ret());
        let value = new_string(&mut e, b"s");
        let slots = e.mem.alloc(0x1c * 2);
        e.call(0x00ad_5b00, &args![slots, 2u32, value, 0u32, 0u32, 0u32]);
        assert_eq!(read_string(&e, slots + 0x1c), b"s");
        let pair = new_pair(&mut e, b"k", b"v");
        let pairs = e.mem.alloc(0x38 * 2);
        e.call(0x00ad_5bd0, &args![pairs, 2u32, pair, 0u32, 0u32, 0u32]);
        assert_eq!(read_string(&e, pairs + 0x38 + 0x1c), b"v");
        // A null slot is skipped (and the pointer still moves on).
        logging(&mut e);
        e.call(0x00ad_5bd0, &args![0u32, 1u32, pair, 0u32, 0u32, 0u32]);
        assert!(calls_to(&e, STRING_TIDY_0044A730).is_empty());
        let vector = new_vector(&mut e, 0x1c, 0, 0);
        let vectors = e.mem.alloc(0x18 * 2);
        e.call(0x00ad_5cc0, &args![vectors, 2u32, vector, 0u32, 0u32, 0u32]);
        assert_ne!(e.mem.u32(vectors + 0x18), 0);
        assert_eq!(e.mem.u32(e.mem.u32(vectors + 0x18)), vectors + 0x18);
    }

    #[test]
    fn n060_00ad2fa0_locks_the_buffer() {
        let mut e = Engine::new();
        stream_doubles(&mut e);
        let s = new_stream(&mut e, 0x10);
        let sentry = e.mem.alloc(8);
        logging(&mut e);
        assert_eq!(e.call(0x00ad_2fa0, &args![sentry, s.stream]).u32(), sentry);
        assert_eq!(e.mem.u32(sentry), s.stream);
        assert_eq!(calls_to(&e, MUTEX_LOCK), vec![vec![s.buffer + 4]]);
        e.mem.set_u32(s.ios + 0x28, 0);
        logging(&mut e);
        e.call(0x00ad_2fa0, &args![sentry, s.stream]);
        assert!(calls_to(&e, MUTEX_LOCK).is_empty());
    }

    fn tree_keys_and_values(e: &mut Engine, tree: u32) -> Vec<(Vec<u8>, Vec<u8>)> {
        let head = e.mem.u32(tree + 0x18);
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, e.mem.u32(tree));
        e.mem.set_u32(iterator + 4, head);
        let mut out = vec![];
        for _ in 0..e.mem.u32(tree + 0x1c) {
            e.call(0x00ad_3450, &args![iterator]);
            let node = e.mem.u32(iterator + 4);
            out.push((read_string(e, node + 0xc), read_string(e, node + 0x28)));
        }
        out.reverse();
        out
    }

    fn pairs_of(list: &[(&[u8], &[u8])]) -> Vec<(Vec<u8>, Vec<u8>)> {
        list.iter().map(|(k, v)| (k.to_vec(), v.to_vec())).collect()
    }

    fn parser_world(e: &mut Engine) {
        allocator_doubles(e);
        compare_double(e);
        e.map(EMPTY_CHARS_01011584, 4);
        e.map(COMMENT_CHARS_010A2FE0, 4);
        e.mem.write(COMMENT_CHARS_010A2FE0, b"//");
    }

    #[test]
    fn n060_00ac7aa0_parses_pairs_into_the_map() {
        let mut e = Engine::new();
        let s = input_stream(&mut e, b"a  1\nb\t2\n// skipped\nc 3 4");
        parser_world(&mut e);
        let tree = new_tree(&mut e);
        let terminator = new_string(&mut e, b"//");
        let ok = e
            .call(0x00ac_7aa0, &args![s.stream, tree, terminator, 0u32])
            .u32();
        assert_eq!(ok & 0xff, 1);
        assert_eq!(
            tree_keys_and_values(&mut e, tree),
            pairs_of(&[(b"a", b"1"), (b"b", b"2"), (b"c", b"3 4")])
        );
        // A maximum of one pair.
        let s = input_stream(&mut e, b"x 1\ny 2\n");
        parser_world(&mut e);
        let tree = new_tree(&mut e);
        e.call(0x00ac_7aa0, &args![s.stream, tree, terminator, 1u32]);
        assert_eq!(
            tree_keys_and_values(&mut e, tree),
            pairs_of(&[(b"x", b"1")])
        );
        // A stream that failed answers 0.
        e.mem.set_u32(s.ios + 8, 2);
        let ok = e
            .call(0x00ac_7aa0, &args![s.stream, tree, terminator, 0u32])
            .u32();
        assert_eq!(ok & 0xff, 0);
        // An empty terminator: a key "" never matches, an empty-named key would be kept.
        let s = input_stream(&mut e, b"k v");
        parser_world(&mut e);
        let tree = new_tree(&mut e);
        let empty = new_string(&mut e, b"");
        e.call(0x00ac_7aa0, &args![s.stream, tree, empty, 0u32]);
        assert_eq!(
            tree_keys_and_values(&mut e, tree),
            pairs_of(&[(b"k", b"v")])
        );
    }

    fn pair_vector_contents(e: &Engine, vector: u32) -> Vec<(Vec<u8>, Vec<u8>)> {
        let mut at = e.mem.u32(vector + 0xc);
        let mut out = vec![];
        while at != e.mem.u32(vector + 0x10) {
            out.push((read_string(e, at), read_string(e, at + 0x1c)));
            at += 0x38;
        }
        out
    }

    #[test]
    fn n060_00ac7f50_parses_pairs_into_the_vector() {
        let mut e = Engine::new();
        let s = input_stream(&mut e, b"z 1\na 2\n");
        parser_world(&mut e);
        pair_range_doubles(&mut e);
        let vector = new_vector(&mut e, 0x38, 0, 0);
        let terminator = new_string(&mut e, b"//");
        let ok = e
            .call(0x00ac_7f50, &args![s.stream, vector, terminator, 0u32])
            .u32();
        assert_eq!(ok & 0xff, 1);
        assert_eq!(
            pair_vector_contents(&e, vector),
            pairs_of(&[(b"z", b"1"), (b"a", b"2")])
        );
    }

    /// The `basic_stringbuf` vtable as the exe has it, its slots on the translations
    /// (`underflow` `00acc070`, `uflow` `00acb7f0`), and what a string stream's life needs.
    fn string_stream_world(e: &mut Engine) {
        parser_world(e);
        string_doubles(e);
        stream_doubles(e);
        stub(
            e,
            &[
                0x00ec_3246,
                0x00ec_31b3,
                0x00ec_3260,
                IOS_BASE_DTOR,
                FUT_SET_LAST_ERROR,
            ],
        );
        let implementation = ctype_world(e);
        returns(e, LOCALE_INIT, implementation);
        let global = e.mem.alloc(8);
        returns(e, GLOBAL_LOCALE_00EC34D4, global);
        e.register(ALLOCATE_CHARACTER_BLOCK_0065FA30, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        e.register(MEMCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let mut slots = vec![0u32; 16];
        slots[4] = 0x00ac_c070;
        slots[5] = 0x00ac_b7f0;
        e.put_vtable(VTABLE_STRINGBUF_010A30C0, &slots);
        e.map(VBTABLE_ISTRINGSTREAM_010A30B4, 8);
        e.mem.set_u32(VBTABLE_ISTRINGSTREAM_010A30B4 + 4, 0x50);
        for (address, text) in [
            (
                MESSAGE_BUFFER_EMPTY_010A3010,
                &b"Input buffer is empty."[..],
            ),
            (MESSAGE_NO_STRING_STREAM_010A2FE4, b"Unable"),
            (
                MESSAGE_PARSE_FAILED_010A2FC0,
                b"Unable to parse string stream.",
            ),
            (
                MESSAGE_STRING_STREAM_BUFFER_010A2FA8,
                b"string stream buffer",
            ),
        ] {
            e.map(address, 0x20);
            e.mem.write(address, text);
        }
    }

    /// The code and message of the last `futSetLastError` (its string argument's characters
    /// through the data pointer word, the length word at +0x14).
    fn last_error(e: &Engine) -> (u32, Vec<u8>) {
        let calls = calls_to(e, FUT_SET_LAST_ERROR);
        let words = calls.last().unwrap();
        (words[0], e.mem.bytes(words[2], words[6]))
    }

    #[test]
    fn n060_00ac8390_parses_a_buffer() {
        let mut e = Engine::new();
        string_stream_world(&mut e);
        let tree = new_tree(&mut e);
        let header = new_string(&mut e, b"face");
        let text = new_c(&mut e, b"face 1\nb 2\n");
        logging(&mut e);
        let ok = e.call(0x00ac_8390, &args![text, header, tree]).u32();
        assert_eq!(ok & 0xff, 1);
        assert!(calls_to(&e, FUT_SET_LAST_ERROR).is_empty());
        // The header pair is read first and dropped when the map is cleared for the rest.
        assert_eq!(
            tree_keys_and_values(&mut e, tree),
            pairs_of(&[(b"b", b"2")])
        );
        // The header is not the first key: error 2.
        let text = new_c(&mut e, b"other 1\n");
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_8390, &args![text, header, tree]).u32() & 0xff,
            0
        );
        assert_eq!(last_error(&e), (2, b"string stream buffer".to_vec()));
        // No buffer: error 0xb.
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_8390, &args![0u32, header, tree]).u32() & 0xff,
            0
        );
        assert_eq!(last_error(&e), (0xb, b"Input buffer is empty.".to_vec()));
        // An empty header skips the check.
        let empty = new_string(&mut e, b"");
        let text = new_c(&mut e, b"q 9");
        assert_eq!(
            e.call(0x00ac_8390, &args![text, empty, tree]).u32() & 0xff,
            1
        );
        assert_eq!(
            tree_keys_and_values(&mut e, tree),
            pairs_of(&[(b"q", b"9")])
        );
    }

    /// A `FILE` as the CRT lays it out (`_ptr` +0, `_cnt` +4, `_base` +8): the file buffer
    /// holds `text` (reading) or 0x400 free bytes (writing, `text` empty). The file buffer
    /// shares these as its get and put areas.
    fn new_file(e: &mut Engine, text: &[u8]) -> u32 {
        let file = e.mem.alloc(0x10);
        let (data, count) = if text.is_empty() {
            (e.mem.alloc(0x400), 0x400)
        } else {
            (new_c(e, text), text.len() as u32)
        };
        e.mem.set_u32(file, data);
        e.mem.set_u32(file + 4, count);
        e.mem.set_u32(file + 8, data);
        file
    }

    fn written(e: &Engine, file: u32) -> Vec<u8> {
        let base = e.mem.u32(file + 8);
        e.mem.bytes(base, e.mem.u32(file) - base)
    }

    /// The `basic_filebuf` vtable on the translations (overflow, pbackfail, underflow, uflow,
    /// xsputn, sync) and the CRT file doubles over the model.
    fn file_stream_world(e: &mut Engine) {
        parser_world(e);
        string_doubles(e);
        file_stream_doubles(e);
        stub(
            e,
            &[
                0x00ec_3260,
                IOS_BASE_DTOR,
                FUT_SET_LAST_ERROR,
                FREE_PROXY_00702220,
            ],
        );
        let (_, _) = codecvt_world(e, true);
        let mut slots = vec![0u32; 16];
        slots[1] = 0x00ac_c9f0;
        slots[2] = 0x00ac_cef0;
        slots[4] = 0x00ac_d060;
        slots[5] = 0x00ac_d100;
        slots[8] = 0x00ac_b9b0;
        slots[12] = 0x00ac_d9a0;
        e.put_vtable(VTABLE_FILEBUF_010A3138, &slots);
        e.register(CRT_GETC, |e, a| {
            let file = a[0];
            let count = e.mem.u32(file + 4) as i32;
            if count > 0 {
                let next = e.mem.u32(file);
                e.mem.set_u32(file, next + 1);
                e.mem.set_u32(file + 4, (count - 1) as u32);
                u32::from(e.mem.u8(next)).into_ret()
            } else {
                0xffff_ffffu32.into_ret()
            }
        });
        e.register(CRT_UNGETC, |e, a| {
            let next = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], next - 1);
            let count = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[1] + 4, count + 1);
            a[0].into_ret()
        });
        e.register(CRT_FPUTC, |e, a| {
            let file = a[1];
            let next = e.mem.u32(file);
            e.mem.set_u8(next, a[0] as u8);
            e.mem.set_u32(file, next + 1);
            let count = e.mem.u32(file + 4);
            e.mem.set_u32(file + 4, count.wrapping_sub(1));
            (a[0] & 0xff).into_ret()
        });
        returns(e, CRT_FFLUSH, 0);
        returns(e, CRT_FCLOSE, 0);
        returns(e, FILE_EXISTS_00AC6E10, 1);
        e.register(MEMCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
    }

    #[test]
    fn n060_00ac87b0_parses_a_pair_file() {
        let mut e = Engine::new();
        file_stream_world(&mut e);
        let file = new_file(&mut e, b"head x\nk v\n");
        returns(&mut e, FIOPEN, file);
        let tree = new_tree(&mut e);
        let header = new_string(&mut e, b"head");
        let path = new_string(&mut e, b"data.txt");
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_87b0, &args![path, header, tree]).u32() & 0xff,
            1
        );
        assert_eq!(
            tree_keys_and_values(&mut e, tree),
            pairs_of(&[(b"k", b"v")])
        );
        assert!(calls_to(&e, FUT_SET_LAST_ERROR).is_empty());
        // Missing header: error 2 with the path.
        let file = new_file(&mut e, b"k v\n");
        returns(&mut e, FIOPEN, file);
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_87b0, &args![path, header, tree]).u32() & 0xff,
            0
        );
        assert_eq!(calls_to(&e, FUT_SET_LAST_ERROR)[0][0], 2);
        // A file that does not open: error 4.
        returns(&mut e, FIOPEN, 0);
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_87b0, &args![path, header, tree]).u32() & 0xff,
            0
        );
        assert_eq!(calls_to(&e, FUT_SET_LAST_ERROR)[0][0], 4);
    }

    #[test]
    fn n060_00ac8c50_parses_a_pair_file_into_a_vector() {
        let mut e = Engine::new();
        file_stream_world(&mut e);
        pair_range_doubles(&mut e);
        e.register(ITERATOR_NOT_EQUAL_00AC0460, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        let file = new_file(&mut e, b"head x\nk v\n");
        returns(&mut e, FIOPEN, file);
        let vector = new_vector(&mut e, 0x38, 0, 0);
        let header = new_string(&mut e, b"head");
        let path = new_string(&mut e, b"data.txt");
        assert_eq!(
            e.call(0x00ac_8c50, &args![path, header, vector]).u32() & 0xff,
            1
        );
        assert_eq!(pair_vector_contents(&e, vector), pairs_of(&[(b"k", b"v")]));
        let file = new_file(&mut e, b"k v\n");
        returns(&mut e, FIOPEN, file);
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_8c50, &args![path, header, vector]).u32() & 0xff,
            0
        );
        assert_eq!(calls_to(&e, FUT_SET_LAST_ERROR)[0][0], 2);
    }

    fn table_contents(e: &Engine, table: u32) -> Vec<Vec<Vec<u8>>> {
        let mut rows = vec![];
        let mut row = e.mem.u32(table + 0xc);
        while row != e.mem.u32(table + 0x10) {
            let mut cells = vec![];
            let mut cell = e.mem.u32(row + 0xc);
            while cell != e.mem.u32(row + 0x10) {
                cells.push(read_string(e, cell));
                cell += 0x1c;
            }
            rows.push(cells);
            row += 0x18;
        }
        rows
    }

    fn csv_doubles(e: &mut Engine) {
        e.register(ALLOCATE_PROXY_0044D670, |e, _| e.mem.alloc(4).into_ret());
        e.register(ITERATOR_NOT_EQUAL_00AC0460, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        e.register(COPY_STRINGS_00AD5A80, |_, a| a[2].into_ret());
        e.register(CONSTRUCT_STRING_00AD6990, |e, a| {
            let text = read_string(e, a[1]);
            set_string(e, a[0], &text);
            Ret::default()
        });
        e.register(COPY_CONSTRUCT_STRINGS_00AD66E0, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = read_string(e, from);
                set_string(e, to, &text);
                from += 0x1c;
                to += 0x1c;
            }
            to.into_ret()
        });
        e.register(COPY_CONSTRUCT_STRINGS_00AD68C0, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = read_string(e, from);
                set_string(e, to, &text);
                from += 0x1c;
                to += 0x1c;
            }
            to.into_ret()
        });
    }

    #[test]
    fn n060_00ac9120_reads_a_csv_file() {
        let mut e = Engine::new();
        file_stream_world(&mut e);
        csv_doubles(&mut e);
        let file = new_file(&mut e, b"a,\"b,c\" ,d\n  \t\nx\n");
        returns(&mut e, FIOPEN, file);
        let table = new_vector(&mut e, 0x18, 0, 4);
        let path = new_string(&mut e, b"t.csv");
        assert_eq!(e.call(0x00ac_9120, &args![path, table]).u32() & 0xff, 1);
        let rows = table_contents(&e, table);
        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows[0],
            vec![b"a".to_vec(), b"b,c ".to_vec(), b"d".to_vec()]
        );
        assert_eq!(rows[1], vec![b"x".to_vec()]);
        // A missing file: error 3.
        returns(&mut e, FILE_EXISTS_00AC6E10, 0);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_9120, &args![path, table]).u32() & 0xff, 0);
        assert_eq!(calls_to(&e, FUT_SET_LAST_ERROR)[0][0], 3);
        // One that does not open: error 4.
        returns(&mut e, FILE_EXISTS_00AC6E10, 1);
        returns(&mut e, FIOPEN, 0);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_9120, &args![path, table]).u32() & 0xff, 0);
        assert_eq!(calls_to(&e, FUT_SET_LAST_ERROR)[0][0], 4);
    }

    #[test]
    fn n060_00ac9790_writes_a_csv_file() {
        let mut e = Engine::new();
        file_stream_world(&mut e);
        csv_doubles(&mut e);
        e.map(QUOTE_CHARS_01021184, 4);
        e.mem.write(QUOTE_CHARS_01021184, b"\"");
        e.map(COMMA_CHARS_01063CD8, 4);
        e.mem.write(COMMA_CHARS_01063CD8, b",");
        e.map(MESSAGE_CSV_EMPTY_010A3028, 0x20);
        let file = new_file(&mut e, b"");
        returns(&mut e, FIOPEN, file);
        // Two rows: "a","b" and "c".
        let table = new_vector(&mut e, 0x18, 2, 2);
        let rows = e.mem.u32(table + 0xc);
        for (row, cells) in [
            (rows, vec![&b"a"[..], b"b"]),
            (rows + 0x18, vec![&b"c"[..]]),
        ] {
            let storage = e.mem.alloc(0x1c * cells.len() as u32);
            for (i, cell) in cells.iter().enumerate() {
                set_string(&mut e, storage + 0x1c * i as u32, cell);
            }
            e.mem.set_u32(row + 0xc, storage);
            e.mem
                .set_u32(row + 0x10, storage + 0x1c * cells.len() as u32);
        }
        let path = new_string(&mut e, b"out.csv");
        logging(&mut e);
        assert_eq!(e.call(0x00ac_9790, &args![path, table]).u32() & 0xff, 1);
        assert_eq!(written(&e, file), b"\"a\",\"b\"\n\"c\"\n");
        assert_eq!(calls_to(&e, FIOPEN)[0][1..], [2, 0x40]);
        // An empty table: error 0xb; a file that does not open: error 5.
        let empty = new_vector(&mut e, 0x18, 0, 0);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_9790, &args![path, empty]).u32() & 0xff, 0);
        assert_eq!(calls_to(&e, FUT_SET_LAST_ERROR)[0][0], 0xb);
        returns(&mut e, FIOPEN, 0);
        logging(&mut e);
        assert_eq!(e.call(0x00ac_9790, &args![path, table]).u32() & 0xff, 0);
        assert_eq!(calls_to(&e, FUT_SET_LAST_ERROR)[0][0], 5);
    }

    #[test]
    fn n060_00ace930_steps_the_read_pointer_back() {
        let mut e = Engine::new();
        let buffer = new_buffer(&mut e, 0x50);
        let data = set_get_area(&mut e, buffer, b"ab");
        let target = e.mem.u32(buffer + 0x20);
        e.mem.set_u32(target, data + 2);
        e.mem.set_u32(e.mem.u32(buffer + 0x30), 0);
        assert_eq!(e.call(0x00ac_e930, &args![buffer]).u32(), data + 1);
        assert_eq!(slot_word(&e, buffer, 0x30), 1);
    }

    #[test]
    fn n060_00aceb80_erases_strings() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        e.register(ITERATOR_NOT_EQUAL_00AC0460, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        e.register(COPY_STRINGS_00AD5A80, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = read_string(e, from);
                set_string(e, to, &text);
                from += 0x1c;
                to += 0x1c;
            }
            to.into_ret()
        });
        let vector = new_vector(&mut e, 0x1c, 3, 3);
        let first = e.mem.u32(vector + 0xc);
        for (i, text) in [b"a", b"b", b"c"].iter().enumerate() {
            set_string(&mut e, first + 0x1c * i as u32, *text);
        }
        let proxy = e.mem.u32(vector);
        let out = e.mem.alloc(8);
        let words = args![vector, out, proxy, first, proxy, first + 0x1c];
        assert_eq!(e.call(0x00ac_eb80, &words).u32(), out);
        assert_eq!(e.mem.u32(out + 4), first);
        assert_eq!(read_string(&e, first), b"b");
        assert_eq!(read_string(&e, first + 0x1c), b"c");
        assert_eq!(e.mem.u32(vector + 0x10), first + 0x38);
    }

    #[test]
    fn n060_00acede0_frees_the_strings() {
        let mut e = Engine::new();
        stub(&mut e, &[OPERATOR_DELETE, STRING_TIDY_0044A730]);
        let vector = new_vector(&mut e, 0x1c, 2, 4);
        let first = e.mem.u32(vector + 0xc);
        logging(&mut e);
        e.call(0x00ac_ede0, &args![vector]);
        assert_eq!(calls_to(&e, STRING_TIDY_0044A730).len(), 2);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![first]]);
        assert_eq!(e.mem.u32(vector + 0x14), 0);
        logging(&mut e);
        e.call(0x00ac_ede0, &args![vector]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn n060_00aceeb0_inserts_a_byte() {
        let mut e = Engine::new();
        e.register(FILL_BYTES_00AC3300, |e, a| {
            let value = e.mem.u8(a[2]);
            for i in 0..a[1] {
                e.mem.set_u8(a[0] + i, value);
            }
            Ret::default()
        });
        e.register(COPY_BYTES_00AD5E20, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            e.mem.write(a[2], &bytes);
            (a[2] + bytes.len() as u32).into_ret()
        });
        e.register(COPY_BYTES_BACKWARD_00AB4490, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            e.mem.write(a[2] - bytes.len() as u32, &bytes);
            Ret::default()
        });
        let vector = new_vector(&mut e, 1, 2, 4);
        let first = e.mem.u32(vector + 0xc);
        e.mem.write(first, b"ac");
        let value = new_c(&mut e, b"b");
        let out = e.mem.alloc(8);
        let proxy = e.mem.u32(vector);
        e.call(0x00ac_eeb0, &args![vector, out, proxy, first + 1, value]);
        assert_eq!(e.mem.bytes(first, 3), b"abc");
        assert_eq!(e.mem.u32(out + 4), first + 1);
    }

    #[test]
    fn n060_00acefd0_erases_bytes() {
        let mut e = Engine::new();
        e.register(ITERATOR_NOT_EQUAL_00AC0460, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        e.register(MEMMOVE_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let vector = new_vector(&mut e, 1, 4, 4);
        let first = e.mem.u32(vector + 0xc);
        e.mem.write(first, b"abcd");
        let proxy = e.mem.u32(vector);
        let out = e.mem.alloc(8);
        e.call(
            0x00ac_efd0,
            &args![vector, out, proxy, first + 1, proxy, first + 3],
        );
        assert_eq!(e.mem.bytes(first, 2), b"ad");
        assert_eq!(e.mem.u32(vector + 0x10), first + 2);
        assert_eq!(e.mem.u32(out + 4), first + 1);
    }

    #[test]
    fn n060_00acf0b0_and_00ad1960_make_and_erase_nodes() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        allocator_doubles(&mut e);
        let tree = new_tree(&mut e);
        let head = e.mem.u32(tree + 0x18);
        let pair = new_pair(&mut e, b"k", b"v");
        let node = e
            .call(0x00ad_1960, &args![tree, head, head, head, pair, 1u32])
            .u32();
        assert_eq!(e.mem.u32(node + 4), head);
        assert_eq!(read_string(&e, node + 0xc), b"k");
        assert_eq!(e.mem.u8(node + 0x44), 1);
        assert_eq!(e.mem.u8(node + 0x45), 0);
        logging(&mut e);
        e.call(0x00ac_f0b0, &args![tree, node]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![node]]);
        logging(&mut e);
        e.call(0x00ac_f0b0, &args![tree, head]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        // A failed allocation leaves the node null.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        logging(&mut e);
        assert_eq!(
            e.call(0x00ad_1960, &args![tree, head, head, head, pair, 0u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, STRING_TIDY_0044A730).is_empty());
    }

    #[test]
    fn n060_00acf900_constructs_a_string_buffer() {
        let mut e = Engine::new();
        string_doubles(&mut e);
        allocator_doubles(&mut e);
        stub(&mut e, &[0x00ec_3246, LOCKIT_CTOR, LOCKIT_DTOR, MEMCPY_S]);
        let implementation = ctype_world(&mut e);
        returns(&mut e, LOCALE_INIT, implementation);
        let global = e.mem.alloc(8);
        returns(&mut e, GLOBAL_LOCALE_00EC34D4, global);
        e.register(ALLOCATE_CHARACTER_BLOCK_0065FA30, |e, a| {
            e.mem.alloc(a[0]).into_ret()
        });
        let buffer = e.mem.alloc(0x50);
        let text = new_string(&mut e, b"xyz");
        // `in | out | ate`: no flags but "at end" (0x10) and ownership.
        assert_eq!(
            e.call(0x00ac_f900, &args![buffer, text, 7u32]).u32(),
            buffer
        );
        assert_eq!(e.mem.u32(buffer), VTABLE_STRINGBUF_010A30C0);
        assert_eq!(e.mem.u32(buffer + 0x40), 0x11);
        let start = slot_word(&e, buffer, 0x14);
        assert_eq!(slot_word(&e, buffer, 0x24), start + 3);
        // `out` only and "append": no reading (4) and 8.
        let other = e.mem.alloc(0x50);
        e.call(0x00ac_f900, &args![other, text, 0xau32]);
        assert_eq!(e.mem.u32(other + 0x40), 0xd);
    }

    #[test]
    fn n060_00acfab0_refuses_a_second_open() {
        let mut e = Engine::new();
        stub(&mut e, &[FIOPEN]);
        let buffer = file_buffer(&mut e, 0x6000, 0);
        logging(&mut e);
        assert_eq!(
            e.call(0x00ac_fab0, &args![buffer, 0x10u32, 1u32, 0x40u32])
                .u32(),
            0
        );
        assert!(calls_to(&e, FIOPEN).is_empty());
        let closed = file_buffer(&mut e, 0, 0);
        assert_eq!(
            e.call(0x00ac_fab0, &args![closed, 0x10u32, 1u32, 0x40u32])
                .u32(),
            0
        );
        assert_eq!(calls_to(&e, FIOPEN), vec![vec![0x10, 1, 0x40]]);
    }

    #[test]
    fn n060_00acffe0_00ad0110_00ad2640_insert_and_erase_string_vectors() {
        let mut e = Engine::new();
        allocator_doubles(&mut e);
        string_doubles(&mut e);
        e.register(ALLOCATE_PROXY_0044D670, |e, _| e.mem.alloc(4).into_ret());
        stub(&mut e, &[FREE_PROXY_00702220]);
        e.register(COPY_CONSTRUCT_STRINGS_00AD68C0, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = read_string(e, from);
                set_string(e, to, &text);
                from += 0x1c;
                to += 0x1c;
            }
            to.into_ret()
        });
        e.register(ITERATOR_NOT_EQUAL_00AC0460, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        e.register(COPY_CONSTRUCT_VECTORS_00AD6E20, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = first_string_of(e, from);
                string_vector_element(e, to, &text);
                from += 0x18;
                to += 0x18;
            }
            to.into_ret()
        });
        e.register(ASSIGN_STRING_VECTOR_00AD61E0, |e, a| {
            let text = first_string_of(e, a[1]);
            string_vector_element(e, a[0], &text);
            a[0].into_ret()
        });
        let table = new_vector(&mut e, 0x18, 2, 4);
        let first = e.mem.u32(table + 0xc);
        string_vector_element(&mut e, first, b"a");
        string_vector_element(&mut e, first + 0x18, b"b");
        let value = e.mem.alloc(0x18);
        string_vector_element(&mut e, value, b"z");
        let proxy = e.mem.u32(table);
        let out = e.mem.alloc(8);
        e.call(0x00ac_ffe0, &args![table, out, proxy, first + 0x18, value]);
        assert_eq!(e.mem.u32(out + 4), first + 0x18);
        assert_eq!(e.mem.u32(table + 0x10), first + 0x48);
        assert_eq!(first_string_of(&e, first + 0x30), b"b");
        logging(&mut e);
        e.call(
            0x00ad_0110,
            &args![table, out, proxy, first, proxy, first + 0x18],
        );
        assert_eq!(e.mem.u32(table + 0x10), first + 0x30);
        assert_eq!(first_string_of(&e, first + 0x18), b"b");
        // One element destroyed: its strings' storage and proxy.
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 2);
        logging(&mut e);
        e.call(0x00ad_2640, &args![table, first, first + 0x18]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 2);
    }

    #[test]
    fn n060_00ad1c10_count_zero_and_maximum() {
        let mut e = Engine::new();
        stub(&mut e, &[VECTOR_TOO_LONG_00AC0B00]);
        let vector = new_vector(&mut e, 0x38, 1, 1);
        let first = e.mem.u32(vector + 0xc);
        let proxy = e.mem.u32(vector);
        logging(&mut e);
        e.call(0x00ad_1c10, &args![vector, proxy, first, 0u32, 0u32]);
        assert!(calls_to(&e, VECTOR_TOO_LONG_00AC0B00).is_empty());
        e.call(
            0x00ad_1c10,
            &args![vector, proxy, first, 0x0492_4924u32, 0u32],
        );
        assert_eq!(calls_to(&e, VECTOR_TOO_LONG_00AC0B00).len(), 1);
        assert_eq!(e.mem.u32(vector + 0x10), first + 0x38);
    }

    #[test]
    fn n060_00ad3150_copies_a_string_vector() {
        let mut e = Engine::new();
        allocator_doubles(&mut e);
        e.register(ALLOCATE_PROXY_0044D670, |e, _| e.mem.alloc(4).into_ret());
        e.register(COPY_CONSTRUCT_STRINGS_00AD68C0, |e, a| {
            let (mut from, last, mut to) = (a[0], a[1], a[2]);
            while from != last {
                let text = read_string(e, from);
                set_string(e, to, &text);
                from += 0x1c;
                to += 0x1c;
            }
            to.into_ret()
        });
        let source = new_vector(&mut e, 0x1c, 2, 2);
        let first = e.mem.u32(source + 0xc);
        set_string(&mut e, first, b"p");
        set_string(&mut e, first + 0x1c, b"q");
        let copy = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00ad_3150, &args![copy, source]).u32(), copy);
        let copied = e.mem.u32(copy + 0xc);
        assert_ne!(copied, first);
        assert_eq!(e.mem.u32(copy + 0x10), copied + 0x38);
        assert_eq!(read_string(&e, copied + 0x1c), b"q");
        assert_eq!(e.mem.u32(e.mem.u32(copy)), copy);
    }
}
